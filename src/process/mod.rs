/**
This module contains the bulk of the header-processing code, and the core structures.

Note that it specifically *does not* contain conditional expression handling, or feature set abstractions.
*/
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;
use {ExpConfig, GenConfig, OutConfig};
use clang::{
    self,
    Index, RcIndexExt,
    TranslationUnit, TranslationUnitFlags,
    Cursor,
};
use features::Features;

mod features;
mod output;
mod renames;
mod sanity;
mod trans_decls;
mod trans_macros;

use self::output::OutputItems;
use self::renames::Renames;

const EMIT_STUBS: bool = true;

// TODO: Make NameMap a proper type.

pub type NameMap = HashMap<String, Cursor>;

fn add_to_name_map_checked(map: &mut NameMap, name: String, cur: Cursor) -> Result<(), String> {
    use std::collections::hash_map::Entry;
    match map.entry(name.clone()) {
        Entry::Occupied(e) => {
            Err(format!("cannot insert {:?} ({}) into name map; already exists as {}",
                name, cur, e.get()))
        },
        Entry::Vacant(e) => {
            info!("add_to_name_map_checked(_, {:?}, {})", name, cur);
            e.insert(cur);
            Ok(())
        }
    }
}

/**
Bundles together any caches we need for efficiency.
*/
pub struct Cache<'a> {
    /// Parsed Clang `TranslationUnit`s.
    tu: TuCache<'a>,

    /// Evaluated per-line feature sets.
    features: HashMap<String, BTreeMap<u32, Features>>,
}

impl<'a> Cache<'a> {
    fn new(index: Rc<Index>, gen_config: &'a GenConfig) -> Self {
        Cache {
            tu: TuCache::new(index, gen_config),
            features: HashMap::new(),
        }
    }

    /**
    Iterates over all feature sets.

    Note that this works by iterating over the underlying feature set maps.  What this gives you *in effect* is the evaluated feature set of *every* conditional compilation branch.
    */
    fn iter_features<F>(&mut self, mut f: F)
    where F: FnMut(&str, u32, &Features) {
        for (&ref name, &ref map) in self.features.iter() {
            for (&line, &ref feat) in map.iter() {
                f(name, line, feat);
            }
        }
    }
}

/**
A `TranslationUnit` cache.
*/
pub struct TuCache<'a> {
    index: Rc<Index>,
    cache: HashMap<TuCacheKey, Rc<TranslationUnit>>,
    gen_config: &'a GenConfig,
}

impl<'a> TuCache<'a> {
    pub fn new(index: Rc<Index>, gen_config: &'a GenConfig) -> TuCache<'a> {
        TuCache {
            index: index,
            cache: HashMap::new(),
            gen_config: gen_config,
        }
    }

    /**
    Parse a translation unit with the given expansion config.

    Unsurprisingly, this will return a cached TU if one has already been parsed.
    */
    pub fn parse_translation_unit(
        &mut self,
        path: &str,
        exp_config: &ExpConfig,
    ) -> Result<Rc<TranslationUnit>, clang::ErrorCode> {
        let index_opts = TranslationUnitFlags::None
            | TranslationUnitFlags::DetailedPreprocessingRecord
            | TranslationUnitFlags::Incomplete
            ;

        let key = TuCacheKey::new(path, exp_config);
        info!("parsing tu {:?} with {:?} ({:?})", path, exp_config, key);

        if let Some(rc_tu) = self.cache.get(&key) {
            info!(".. already in cache");
            return Ok(rc_tu.clone())
        }

        let switches: Vec<String> = self.gen_config.switches().iter().map(|s| s.clone())
            .chain(exp_config.switches().into_iter())
            .collect();

        let tu = try!(self.index.parse_translation_unit(
            path,
            &switches,
            &[],
            index_opts,
        ));
        self.cache.insert(key, tu.clone());
        Ok(tu)
    }
}

/**
This is the unique key for each entry in the `TuCache`.
*/
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TuCacheKey(String, ExpConfig);

impl TuCacheKey {
    pub fn new(path: &str, exp_config: &ExpConfig) -> TuCacheKey {
        TuCacheKey(path.into(), exp_config.clone())
    }
}
/**
This is effectively the "entry point" for processing.  Given a header and a configuration, it attempts to generate a Rust binding.
*/
pub fn process_header(path: &str, gen_config: &GenConfig, out_config: &OutConfig) {
    info!("using clang version {}", clang::version());

    let index = Index::create(
        /*exclude_declarations_from_pch*/ false,
        /*display_diagnostics*/ false,
    );

    let mut out_items = OutputItems::new();
    let mut cache = Cache::new(index, gen_config);

    // Expand once for each expansion config.
    for exp_config in &gen_config.exp_configs {
        info!("expanding with config {:?}", exp_config);
        info!(".. switches: {:?}", exp_config.switches());
        let tu = cache.tu.parse_translation_unit(path, exp_config).ok().expect("parse TU");
        let renames = renames::scan_for_renames(tu.clone(), gen_config);
        trans_decls::process_decls(tu, gen_config, exp_config, &mut out_items, &mut cache, &renames);
    }

    info!("generating output...");
    let out_files = &mut output::OutputFiles::new(out_config);
    output::output_header_items(&out_items, out_files);
    output::output_func_items(&out_items, out_files, out_config);

    info!("sanity-checking features...");
    sanity::sanity_check_features(&mut cache);
}

/**
A helper method that yields the "next" cursor to process from both a primary sequence and a list of deferred items.
*/
fn next_from(
    dc: &mut ::std::vec::IntoIter<Cursor>,
    d: &mut Vec<Cursor>,
    di: &mut Option<::std::vec::IntoIter<Cursor>>
) -> Option<Cursor> {
    use std::mem::replace;

    if let Some(cur) = di.as_mut().and_then(|di| di.next()) {
        return Some(cur)
    }

    *di = None;

    if d.len() > 0 {
        *di = Some(replace(d, vec![]).into_iter());
        return next_from(dc, d, di);
    }

    dc.next()
}

fn escape_ident(ident: String) -> String {
    let is_bad = match &*ident {
          "abstract"    | "alignof" | "as"          | "become"      | "box"
        | "break"       | "const"   | "continue"    | "crate"       | "do"
        | "else"        | "enum"    | "extern"      | "false"       | "final"
        | "fn"          | "for"     | "if"          | "impl"        | "in"
        | "let"         | "loop"    | "macro"       | "match"       | "mod"
        | "move"        | "mut"     | "offsetof"    | "override"    | "priv"
        | "proc"        | "pub"     | "pure"        | "ref"         | "return"
        | "Self"        | "self"    | "sizeof"      | "static"      | "struct"
        | "super"       | "trait"   | "true"        | "type"        | "typeof"
        | "unsafe"      | "unsized" | "use"         | "virtual"     | "where"
        | "while"       | "yield"
        => true,
        _ => false
    };
    if is_bad {
        ident + "_"
    } else {
        ident
    }
}

fn file_stem(cur: &Cursor) -> String {
    cur.location().file().expect("valid file for file_stem").name()
}

/**
This works out the module qualifier for a given type.  This is intended to let you put types into files based on their source header.
*/
fn mod_qual(cur: &Cursor) -> String {
    let file = cur.location().file();
    match file.map(|f| f.name()) {
        Some(name) => format!("::{}::", name),
        None => String::new()
    }
}

/**
Works out a name for the given structure, even if it doesn't otherwise *have* one.
*/
fn name_for_maybe_anon(decl_cur: &Cursor, renames: &Renames) -> Result<(String, String), String> {
    // Check to see if this cursor has been renamed...
    let cur = match renames.rename_decl(&decl_cur) {
        Ok(cur) => cur,
        Err(cur) => cur
    };

    // TODO: Use clang_Cursor_isAnonymous once its released.
    let name = cur.spelling();

    if name == "" {
        /*
        This is *probably* an anonymous type.  We need to give it a name that will be both reasonable *and* stable across invocations.
        */
        return Err(format!("anonymous-struct {}", cur));
    }
    Ok((name, file_stem(&cur)))
}
