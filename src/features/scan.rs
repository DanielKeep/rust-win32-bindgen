use std::collections::BTreeMap;
use clang;
use super::{Features, define_feature, define_feature_expr};

pub fn scan_features(tls: Vec<(u32, Vec<clang::Token>)>) -> BTreeMap<u32, Features> {
    debug!("scan_features([..; {}])", tls.len());
    /*
    The way this works is that we have to walk through *all* the lines, looking for preprocessor conditional compilation directives.  When we find them, we interpret them and push the enabled feature tests on to the stack.  Then, when we find something that *isn't* a conditional directive *and* the features have changed since the last time we did so, we add an entry to the map.
    */
    let mut stack = vec![Features::default()];

    fn fd() -> Features { Features::default() }

    fn seq(lhs: &[String], lhs_len: usize, rhs: &[&'static str]) -> bool {
        use std::cmp::min;
        let lhs = &lhs[..min(lhs.len(), lhs_len)];
        lhs.len() == rhs.len()
            && lhs.iter().zip(rhs.iter()).all(|(l,r)| &**l == *r)
    }

    fn append(stack: &mut Vec<Features>, f: Features) {
        debug!("append([..; {}], {:?})", stack.len(), f);
        let f = stack.last().expect("non-empty stack").clone().and(f);
        stack.push(f);
    }

    fn pop(stack: &mut Vec<Features>) {
        debug!("pop([..; {}])", stack.len());
        stack.pop();
    }

    let mut map = BTreeMap::new();

    // Insert fallback.
    map.insert(0, fd());

    for (line_num, toks) in tls {
        let loc = toks[0].location();
        let ts: Vec<_> = toks.iter()
            .map(|t| t.spelling())
            .filter(|s| !(s.starts_with("/*") || s.starts_with("//")))
            .collect();
        let ts = &*ts;
        debug!("scan_features(..): {}: {:?}", loc.display_short(), ts);
        // debug!(".. stack: {:?}", stack);

        if seq(&ts, 2, &["#", "if"]) {
            debug!(".. #if {:?}", &ts[2..]);
            append(&mut stack, define_feature_expr(&ts[2..], &loc));
        } else if seq(&ts, 2, &["#", "ifdef"]) && ts.len() == 3 {
            debug!(".. #ifdef {:?}", &ts[2..]);
            append(&mut stack, define_feature(&ts[2]));
        } else if seq(&ts, 2, &["#", "ifndef"]) && ts.len() == 3 {
            debug!(".. #ifndef {:?}", &ts[2..]);
            append(&mut stack, define_feature(&ts[2]).complement());
            // debug!(".. #ifndef done");
        } else if seq(&ts, 2, &["#", "elif"]) {
            debug!(".. #elif {:?}", &ts[2..]);
            pop(&mut stack);
            append(&mut stack, define_feature_expr(&ts[2..], &loc));
        } else if seq(&ts, 2, &["#", "else"]) {
            debug!(".. #else");
            pop(&mut stack);
            append(&mut stack, fd());
        } else if seq(&ts, 2, &["#", "endif"]) {
            debug!(".. #endif");
            pop(&mut stack);
        } else {
            // debug!(".. boring");
            // Work out what the last set of features we've seen is.
            let do_insert = {
                let prev_feat = map.values().next_back().expect("previous feature");
                *prev_feat != *stack.last().expect("non-empty stack")
            };
            // debug!(".. do_insert: {:?}", do_insert);
            if do_insert {
                let feat = stack.last().expect("non-empty stack").clone();
                debug!(" .. insert {}: {:?}", line_num, feat);
                map.insert(line_num, feat);
            }
        }
    }

    debug!(".. done ({} entries)", map.len());
    map
}
