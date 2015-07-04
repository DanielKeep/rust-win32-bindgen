use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use GenConfig;
use clang::{
    self,
    TranslationUnit,
    Cursor,
};

use super::next_from;

pub struct Renames {
    renames: HashMap<Cursor, Cursor>,
    invalidations: HashSet<Cursor>,
}

impl Renames {
    pub fn add_rename(&mut self, from: Cursor, to: Cursor) {
        assert!(!self.is_renamed(&from), "definition is already renamed");
        assert!(!self.invalidations.contains(&to), "already have invalidation");
        self.renames.insert(from, to.clone());
        self.invalidations.insert(to);
    }

    pub fn is_invalidated(&self, decl_cur: &Cursor) -> bool {
        self.invalidations.contains(decl_cur)
    }

    pub fn is_renamed(&self, defn_cur: &Cursor) -> bool {
        self.renames.contains_key(defn_cur)
    }

    pub fn rename_decl<'a, 'b>(&'a self, decl_cur: &'b Cursor) -> Result<&'a Cursor, &'b Cursor> {
        if let Some(&ref cur) = self.renames.get(decl_cur) {
            Ok(cur)
        } else {
            Err(decl_cur)
        }
    }

    pub fn rename_ty(&self, ty: clang::Type) -> Result<clang::Cursor, clang::Type> {
        use clang::TypeKind as TK;

        // We're only concerned about structs, enums and unions.
        match ty.kind() {
            TK::Record | TK::Enum => {
                let ty_decl = match ty.declaration().definition() {
                    Some(cur) => cur,
                    None => return Err(ty)
                };
                if let Some(&ref cur) = self.renames.get(&ty_decl) {
                    return Ok(cur.clone());
                }
                Err(ty)
            },
            _ => Err(ty)
        }
    }
}

impl Default for Renames {
    fn default() -> Self {
        Renames {
            renames: HashMap::new(),
            invalidations: HashSet::new(),
        }
    }
}

pub fn scan_for_renames(tu: Rc<TranslationUnit>, gen_config: &GenConfig) -> Renames {
    /*
    The goal here is to find two kinds of things:

    1. Types which have *no* name in tag space, but are given one via typedef.
    2. Types which *have* a name in tag space, but the *canonical* name is given via typedef.

    We handle both of these by scanning through all the typedefs.  If we find one whose subject is one of the above types, we record the subject's cursor and the *new* name, as well as an "invalidation" of the typedef's cursor.
    */
    info!("scanning for renames...");
    let mut renames = Renames::default();
    let mut decl_curs = tu.cursor().children().into_iter();
    let mut deferred: Vec<Cursor> = vec![];
    let mut deferred_iter = None;

    while let Some(decl_cur) = next_from(&mut decl_curs, &mut deferred, &mut deferred_iter) {
        if !gen_config.should_ignore(&decl_cur) {
            let rename = scan_decl_for_rename(decl_cur, gen_config, &renames, &mut |cur| deferred.push(cur));
            if let Some((from, to)) = rename {
                renames.add_rename(from, to);
            }
        }
    }

    renames
}

fn scan_decl_for_rename<Defer>(
    decl_cur: Cursor,
    gen_config: &GenConfig,
    renames: &Renames,
    mut defer: Defer,
) -> Option<(Cursor, Cursor)>
where Defer: FnMut(Cursor) {
    use clang::CursorKind as CK;
    use clang::TypeKind as TK;

    match decl_cur.kind() {
        CK::TypedefDecl => (),
        _ => return None
    }

    debug!("scan_decl_for_rename({}, ..)", decl_cur);

    let ty = decl_cur.typedef_decl_underlying_type();

    // Resolve unexposed types if possible.
    let ty = match ty.kind() {
        TK::Unexposed => ty.canonical(),
        _ => ty
    };

    // We don't want to even look at things that aren't just direct ADT typedefs.
    match ty.kind() {
        TK::Record | TK::Enum => (),
        other => {
            debug!(".. ignoring indirect typedef: {:?}", other);
            return None;
        }
    }

    // Get the original type definition.
    let ty_defn = ty.declaration().definition();
    let ty_defn = match ty_defn {
        Some(c) => c,
        None => {
            debug!(".. ignoring; has no definition");
            return None;
        }
    };

    // Double-check that this is really what we think it is.
    match ty_defn.kind() {
        CK::StructDecl | CK::EnumDecl | CK::UnionDecl => (),
        other => {
            debug!(".. ignoring non-adt target: {:?}", other);
            return None;
        }
    };

    // Defer all the child cursors.
    for child in ty_defn.children() { defer(child); }

    // Check if it's already canonical.
    let ty_name = ty_defn.spelling();
    if !gen_config.is_tag_name_non_canonical(&ty_name) {
        debug!(".. ignoring canonical name {:?}", ty_name);
        return None;
    }

    // Finally, make sure we aren't renaming it more than once.
    if renames.is_renamed(&ty_defn) {
        debug!(".. ignoring; already renamed");
        return None;
    }

    // Do the rename.
    debug!(".. renaming {} to {}", ty_defn, decl_cur);
    Some((ty_defn, decl_cur))
}
