use std::collections::{BTreeMap, HashMap};
use itertools::Itertools;
use ExpConfig;
use clang::{
    self,
    RcIndexExt,
};
use features::{Features, scan_features};
use util;

use super::{Cache, TuCache};

/**
Calculate the feature set map for a given file.
*/
fn get_all_features<'a>(file: clang::File, cache: &'a mut Cache) -> &'a BTreeMap<u32, Features> {
    let path = file.file_name();
    let tu_cache = &mut cache.tu;
    let fmap = cache.features.entry(path.clone()).or_insert_with(||
        scan_features(get_token_lines(file, tu_cache)));

    fmap
}

/**
Calculate the feature set at a given line.
*/
pub fn get_features_at(file: clang::File, line: u32, cache: &mut Cache) -> Features {
    use std::collections::Bound;

    debug!("get_features_at({:?}, {}, _)", file.file_name(), line);

    let fmap = get_all_features(file, cache);
    fmap.range(Bound::Unbounded, Bound::Included(&line)).next_back()
        .map(|(i, v)| {
            debug!(".. found: {}: {:?}", i, v);
            v.clone()
        })
        .unwrap_or_else(|| Features::default())
}

/**
Returns a given file as a sequence of `(line_number, tokens)` pairs.
*/
fn get_token_lines(file: clang::File, tu_cache: &mut TuCache) -> Vec<(u32, Vec<clang::Token>)> {
    debug!("get_token_lines({:?}, _)", file.file_name());
    let path = file.file_name();

    // Architecture shouldn't matter since we just want the tokens.
    let tu = tu_cache.parse_translation_unit(&path, &ExpConfig::DUMMY_CFG).unwrap();

    // Get the set of line numbers which *contain* a line continuation.
    let cont_lines: Vec<_> = util::read_lines(&path)
        .map(|rs| rs.unwrap())
        .enumerate()
        .filter(|&(_, ref s)| s.trim_right().ends_with("\\"))
        // +1 because enumerate is 0-based, line numbers are 1-based
        .map(|(i, _)| (i + 1) as u32) // TODO: checked
        .collect();

    // Work out the starting line for continued lines.
    let mut line_starts = HashMap::new();
    for i in cont_lines {
        let start = *line_starts.get(&i).unwrap_or(&i);
        line_starts.insert(i+1, start);
    }

    // Change each line's line number to be the *first* line the (possibly continued) line starts on.
    let remap_line_number = |tok: &clang::Token| {
        let l = tok.location().line();
        *line_starts.get(&l).unwrap_or(&l)
    };

    // Grab all the tokens, then re-group them by logical line.
    tu.tokenize().into_iter().group_by(remap_line_number).collect()
}

