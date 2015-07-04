use WinVersion;

use super::Cache;

pub fn sanity_check_features(cache: &mut Cache) {
    use std::collections::BTreeSet;

    let mut weird_vers = BTreeSet::new();

    cache.iter_features(|path, line, &ref feat| {
        use features::Partitions;

        /*
        What we're looking for are any features that might mess up the expansion.  This currently means:

        - Features with upper limits on versions.
        - Features that *do not* target the desktop.
        */

        let mut suspect = vec![];

        if let Some(ref parts) = feat.parts {
            if (parts.clone() & Partitions::DesktopApp).is_empty() {
                suspect.push("non-desktop-app");
            }
        }

        if let Some(ref winver) = feat.winver {
            if !winver.is_simple() {
                for &ref range in winver.ranges() {
                    weird_vers.insert(range.end);
                }
                suspect.push("complex-winver");
            }
        }

        if suspect.len() != 0 {
            warn!("suspect feature set: {}:{}: {} {:?}",
                path, line, suspect.connect(", "), feat);
        }
    });

    if weird_vers.len() > 0 {
        warn!("suspect versions:");
        for ver in weird_vers {
            warn!(".. 0x{:08x} - {:?}",
                ver, WinVersion::from_u32_round_up(ver));
        }
    }
}

