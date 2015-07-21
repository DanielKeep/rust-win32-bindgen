use std::collections::HashMap;
use std::fs;
use std::path;
use itertools::Itertools;
use OutConfig;
use features::Features;

/**
An "abstract" calling convention.

This is to answer the question: "if a function uses the C calling convention, is that the same thing as `"system"`, or do I have to *actually* say `"C"`?"

Without this, almost every Windows API call would need two decls: one with `extern "C"`, and one with `extern "stdcall"`.  Yuck.
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AbsCallConv {
    ExplicitlyC,
    System,
}

impl AbsCallConv {
    /// Gets the calling convention as a string, suitable for use with Rust's `extern`.
    pub fn as_str(self) -> &'static str {
        use self::AbsCallConv::*;
        match self {
            ExplicitlyC => "C",
            System => "system",
        }
    }
}

/**
Used to centralise how output of translated items is done.

One of the major reasons for this is to consolidate disparate bindings.  That is, if the output for both x86 and x86-64 are the same, then they should use a *single* declaration with an appropriate `#[cfg]` attribute.

Note that `annot` is used for "annotations", which are free-form strings that may be emitted as comments in the output.  These are handy for identifying, for example, *where* a declaration originally came from, for debugging purposes.
*/
pub struct OutputItems {
    next_seq_id: u64,

    /// `[name => [(alias, feat, decl, annot)]]`
    pub fn_aliases: HashMap<String, Vec<(u64, String, Features, String, String)>>,

    /// `[name => [(feat, cconv, decl, annot)]]`
    pub fn_items: HashMap<String, Vec<(u64, Features, AbsCallConv, String, String)>>,

    /// `[name => [(header, feat, decl, annot)]]`
    pub header_items: HashMap<String, Vec<(u64, String, Features, String, String)>>,

    /// `[name => [(feat, decl, annot)]]`
    pub var_items: HashMap<String, Vec<(u64, Features, String, String)>>,
}

impl OutputItems {
    pub fn new() -> Self {
        OutputItems {
            next_seq_id: 0,
            fn_aliases: HashMap::new(),
            fn_items: HashMap::new(),
            header_items: HashMap::new(),
            var_items: HashMap::new(),
        }
    }

    /**
    Adds a function alias.

    This is different from a regular function item in that the library is should be emitted to is based on some *other* function which it is aliasing.

    If the given `decl` matches an already existing `decl` with the same `name`, the existing entry will have its feature set unioned with `feat`, and `annot` appended to its annotation.
    */
    pub fn add_func_alias(&mut self, name: String, alias: String, feat: Features, decl: String, annot: String) {
        use std::mem::replace;
        debug!("add_func_alias({:?}, {:?}, {:?}, {:?}, {:?})", name, alias, feat, decl, annot);

        let decls = self.fn_aliases.entry(name).or_insert(vec![]);

        // Is there already a decl which is compatible with this one?
        for &mut (_, ref mut dal, ref mut df, ref dd, ref mut da) in decls.iter_mut() {
            if *dd == decl && *dal == alias {
                debug!(".. merging");
                // The decls are the same.  Just combine the feature sets together.
                let new_df = replace(df, Features::default()).or(feat);
                *df = new_df;
                if *da != annot {
                    da.push_str(", ");
                    da.push_str(&annot);
                }
                return;
            }
        }

        // Add it to the set of decls.
        debug!(".. adding");
        decls.push((self.next_seq_id, alias, feat, decl, annot));
        self.next_seq_id += 1;
    }

    /**
    Adds a function declaration.

    If the given `decl` matches an already existing `decl` with the same `name`, the existing entry will have its feature set unioned with `feat`, and `annot` appended to its annotation.
    */
    pub fn add_func_item(&mut self, name: String, feat: Features, cconv: AbsCallConv, decl: String, annot: String) {
        use std::mem::replace;
        debug!("add_func_item({:?}, {:?}, {:?}, {:?}, {:?})", name, feat, cconv, decl, annot);

        let decls = self.fn_items.entry(name).or_insert(vec![]);

        // Is there already a decl which is compatible with this one?
        for &mut (_, ref mut df, ref dcc, ref dd, ref mut da) in decls.iter_mut() {
            if *dd == decl && *dcc == cconv {
                debug!(".. merging");
                // The decls are the same.  Just combine the feature sets together.
                let new_df = replace(df, Features::default()).or(feat);
                *df = new_df;
                if *da != annot {
                    da.push_str(", ");
                    da.push_str(&annot);
                }
                return;
            }
        }

        // Add it to the set of decls.
        debug!(".. adding");
        decls.push((self.next_seq_id, feat, cconv, decl, annot));
        self.next_seq_id += 1;
    }

    /**
    Adds a header declaration.

    If the given `decl` matches an already existing `decl` with the same `name`, the existing entry will have its feature set unioned with `feat`, and `annot` appended to its annotation.
    */
    pub fn add_header_item(&mut self, name: String, header: String, feat: Features, decl: String, annot: String) {
        use std::mem::replace;
        debug!("add_header_item({:?}, {:?}, {:?}, {:?}, {:?})", header, name, feat, decl, annot);

        let decls = self.header_items.entry(name).or_insert(vec![]);

        // Is there already a decl which is compatible with this one?
        for &mut (_, ref dh, ref mut df, ref dd, ref mut da) in decls.iter_mut() {
            if *dh == header && *dd == decl {
                debug!(".. merging");
                // The decls are the same.  Just combine the feature sets together.
                let new_df = replace(df, Features::default()).or(feat);
                *df = new_df;
                da.push_str(", ");
                da.push_str(&annot);
                return;
            }
        }

        // Add it to the set of decls.
        debug!(".. adding");
        decls.push((self.next_seq_id, header, feat, decl, annot));
        self.next_seq_id += 1;
    }

    /**
    Adds a variable declaration.

    If the given `decl` matches an already existing `decl` with the same `name`, the existing entry will have its feature set unioned with `feat`, and `annot` appended to its annotation.
    */
    pub fn add_var_item(&mut self, name: String, feat: Features, decl: String, annot: String) {
        use std::mem::replace;
        debug!("add_var_item({:?}, {:?}, {:?}, {:?})", name, feat, decl, annot);

        let decls = self.var_items.entry(name).or_insert(vec![]);

        // Is there already a decl which is compatible with this one?
        for &mut (_, ref mut df, ref dd, ref mut da) in decls.iter_mut() {
            if *dd == decl {
                debug!(".. merging");
                // The decls are the same.  Just combine the feature sets together.
                let new_df = replace(df, Features::default()).or(feat);
                *df = new_df;
                if *da != annot {
                    da.push_str(", ");
                    da.push_str(&annot);
                }
                return;
            }
        }

        // Add it to the set of decls.
        debug!(".. adding");
        decls.push((self.next_seq_id, feat, decl, annot));
        self.next_seq_id += 1;
    }
}

/**
This cache owns the output files and saves us from constantly opening and closing them.
*/
pub struct OutputFiles<'a> {
    out_config: &'a OutConfig,
    files: HashMap<path::PathBuf, (fs::File, Option<(Features, AbsCallConv)>)>,
}

impl<'a> OutputFiles<'a> {
    pub fn new(out_config: &'a OutConfig) -> Self {
        OutputFiles {
            out_config: out_config,
            files: HashMap::new(),
        }
    }

    pub fn emit_to_header(&mut self, name: &str, feat: &Features, decl: &str, annot: &str) {
        use std::io::prelude::*;
        let (file, _) = self.get_file(name, &self.out_config.header_path);
        let decl = decl.replace("${feat}", &feat.to_string());
        if !decl.starts_with("//") {
            writeln!(file, "{}{} /* {} */", feat, decl, annot).unwrap();
        } else {
            writeln!(file, "/* {}{} */ /* {} */", feat, decl, annot).unwrap();
        }
    }

    pub fn emit_to_library(&mut self, name: &str, feat: &Features, cconv: Option<AbsCallConv>, decl: &str, annot: &str) {
        use std::io::prelude::*;
        let (file, group) = self.get_file(name, &self.out_config.library_path);

        if let Some(cconv) = cconv {
            // Change grouping if necessary.
            match *group {
                Some((ref gf, ref gcc)) if gf == feat && *gcc == cconv => (),
                Some(_) => {
                    writeln!(file, "}}\n{}\nextern {:?} {{", feat, cconv.as_str()).unwrap();
                },
                None => {
                    writeln!(file, "{}\nextern {:?} {{", feat, cconv.as_str()).unwrap();
                }
            }

            // Proceed with output.
            writeln!(file, "    {} /* {} */", decl, annot).unwrap();

            // Update the "last" group.
            *group = Some((feat.clone(), cconv));
        } else {
            if group.is_some() {
                writeln!(file, "}}").unwrap();
            }
            writeln!(file, "{}{} /* {} */", feat, decl, annot).unwrap();
            *group = None;
        }
    }

    /**
    Finishes output to all open files.

    This includes things like closing `extern` blocks.
    */
    pub fn finish_output(self) {
        use std::io::prelude::*;
        debug!("finish_output()");

        for (_, (mut file, group)) in self.files.into_iter() {
            match group {
                Some(_) => writeln!(file, "}}").unwrap(),
                None => ()
            }
        }
    }

    fn get_file<'b>(
        &'b mut self,
        name: &str,
        pattern: &str
    ) -> (&'b mut fs::File, &'b mut Option<(Features, AbsCallConv)>) {
        use std::path::PathBuf;
        let mut path = PathBuf::from(&self.out_config.output_dir);
        path.push(pattern.replace("{}", name));
        let fg = self.files.entry(path.clone())
            .or_insert_with(|| (fs::File::create(path).unwrap(), None));
        (&mut fg.0, &mut fg.1)
    }
}

pub fn output_header_items(items: &OutputItems, output: &mut OutputFiles) {
    let mut lines = vec![];
    for (_, decls) in &items.header_items {
        for &(idx, ref header, ref feat, ref decl, ref annot) in decls {
            lines.push((header, idx, feat, decl, annot));
        }
    }
    lines.sort();

    let lines = lines.into_iter()
        .group_by_lazy(|&(header, _, feat, _, _)| (header, feat));

    for ((header, feat), group) in &lines {
        for (_, _, _, decl, annot) in group {
            output.emit_to_header(header, feat, decl, annot);
        }
    }
}

pub fn output_func_items(items: &OutputItems, output: &mut OutputFiles, out_config: &OutConfig) {
    let mut lines = vec![];
    for (name, decls) in &items.fn_items {
        for &(_, ref feat, ref cconv, ref decl, ref annot) in decls {
            for &ref lib in out_config.get_fn_libs(name) {
                lines.push((lib, feat, name, Some(*cconv), decl, annot));
            }
        }
    }
    for (name, decls) in &items.fn_aliases {
        for &(_, ref alias, ref feat, ref decl, ref annot) in decls {
            for &ref lib in out_config.get_fn_libs(alias) {
                lines.push((lib, feat, name, None, decl, annot));
            }
        }
    }
    for (name, decls) in &items.var_items {
        for &(_, ref feat, ref decl, ref annot) in decls {
            for &ref lib in out_config.get_fn_libs(name) {
                lines.push((lib, feat, name, Some(AbsCallConv::System), decl, annot));
            }
        }
    }
    lines.sort();

    let lines = lines.into_iter()
        .group_by_lazy(|&(lib, feat, _, cconv, _, _)| (lib, feat, cconv));

    for ((lib, feat, cconv), group) in &lines {
        for (_, _, _, _, decl, annot) in group {
            output.emit_to_library(lib, feat, cconv, decl, annot);
        }
    }
}
