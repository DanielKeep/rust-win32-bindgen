use std::fmt;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use itertools::Itertools;
use WinVersion;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WinVersions(Vec<Range<u32>>);

impl WinVersions {
    pub fn is_any(&self) -> bool {
        &*self.0 != [0..0]
    }

    pub fn is_simple(&self) -> bool {
        !self.is_any() || (self.0.len() == 1 && self.0[0].end == !0)
    }

    pub fn ranges(&self) -> &[Range<u32>] {
        &self.0
    }

    pub fn complement(self) -> WinVersions {
        debug!("WinVersions::complement({:?})", self);
        if &*self.0 == &[0..!0] {
            return WinVersions(vec![0..0]);
        }

        if &*self.0 == &[0..0] {
            return WinVersions(vec![0..!0]);
        }

        let mut pts: Vec<_> = self.0.into_iter().flat_map(|ab| vec![ab.start, ab.end].into_iter()).collect();
        pts.dedup();
        debug!(".. pts: {:?}", pts);

        let pts: Vec<_> = match (pts[0] == 0, pts[pts.len()-1] == !0) {
            (true, true) => pts[1..pts.len()-1].into(),
            (true, false) => pts[1..].iter().cloned().chain(Some(!0)).collect(),
            (false, true) => Some(0).into_iter().chain(pts[..pts.len()-1].iter().cloned()).collect(),
            (false, false) => Some(0).into_iter()
                .chain(pts.iter().cloned())
                .chain(Some(!0))
                .collect()
        };
        debug!(".. pts: {:?}", pts);

        let ranges = pts.iter().cloned()
            .batching(|mut it| it.next().and_then(
                |a| it.next().map(
                    |b| a..b)))
            .collect();
        debug!(".. ranges: {:?}", ranges);

        WinVersions(ranges)
    }

    pub fn intersect(self, other: WinVersions) -> WinVersions {
        let mut abs = &self.0[..];
        let mut ijs = &other.0[..];

        let mut acc = vec![];
        while abs.len() > 0 && ijs.len() > 0 {
            let Range { start: a, end: b } = abs[0].clone();
            let Range { start: i, end: j } = ijs[0].clone();
            assert!(a <= b);
            assert!(i <= j);

            if b < i {
                /*
                Drop ab.

                    a .. b
                            i .. j
                */
                abs = &abs[1..];
            } else if a <= i && i <= b && b <= j {
                /*
                Emit i..b, drop ab.

                    a .. b
                       i .. j
                */
                acc.push(i..b);
                abs = &abs[1..];
            } else if i <= a && a <= j && j <= b {
                /*
                Emit a..j, drop ij.

                       a .. b
                    i .. j
                */
                acc.push(a..j);
                ijs = &ijs[1..];
            } else if j < b {
                /*
                Drop ij.

                            a .. b
                    i .. j
                */
                ijs = &ijs[1..];
            } else {
                panic!("unreachable: 0x{:08x}..0x{:08x}, 0x{:08x}..0x{:08x}", a, b, i, j);
            }
        }

        WinVersions(acc).simplify()
    }

    pub fn union(mut self, mut other: WinVersions) -> WinVersions {
        fn inner(mut acc: Vec<Range<u32>>, abs: &mut [Range<u32>], ijs: &mut [Range<u32>]) -> Vec<Range<u32>> {
            if abs.len() == 0 || ijs.len() == 0 {
                acc.extend(abs.iter().cloned());
                acc.extend(ijs.iter().cloned());
                acc
            } else {
                let Range { start: a, end: b } = abs[0].clone();
                let Range { start: i, end: j } = ijs[0].clone();
                assert!(a <= b);
                assert!(i <= j);

                if a == b {
                    /*
                    Drop ab.
                    */
                    inner(acc, &mut abs[1..], ijs)
                } else if i == j {
                    /*
                    Drop ij.
                    */
                    inner(acc, abs, &mut ijs[1..])
                }
                else if b < i {
                    /*
                    Emit a..b, drop ab.

                        a .. b
                                i .. j
                    */
                    acc.push(a..b);
                    inner(acc, &mut abs[1..], ijs)
                } else if a <= i && i <= b && b <= j {
                    /*
                    ij = a..j, drop ab.

                        a .. b
                           i .. j
                    */
                    ijs[0] = a..j;
                    inner(acc, &mut abs[1..], ijs)
                } else if i <= a && a <= j && j <= b {
                    /*
                    ab = i..b, drop ij.

                           a .. b
                        i .. j
                    */
                    abs[0] = i..b;
                    inner(acc, abs, &mut ijs[1..])
                } else if j < a {
                    /*
                    Emit i..j, drop ij.

                                a .. b
                        i .. j
                    */
                    acc.push(i..j);
                    inner(acc, abs, &mut ijs[1..])
                } else {
                    panic!("unreachable: 0x{:08x}..0x{:08x}, 0x{:08x}..0x{:08x}", a, b, i, j);
                }
            }
        }

        let mut ranges = inner(vec![], &mut self.0, &mut other.0);

        if ranges.len() == 0 {
            ranges.push(0..0);
        }

        WinVersions(ranges).simplify()
    }

    fn simplify(self) -> Self {
        let mut pts: Vec<_> = self.0.into_iter().flat_map(|ab| vec![ab.start, ab.end].into_iter()).collect();
        pts.dedup();
        let ranges = pts.iter().cloned()
            .batching(|mut it| it.next().and_then(
                |a| it.next().map(
                    |b| a..b)))
            .collect();
        WinVersions(ranges)
    }
}

pub const CFG_FEATURE_VERSION_PREFIX: &'static str = "winapi_ver_";

impl fmt::Display for WinVersions {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        assert!(&*self.0 != &[0..0], "can't have no versions enabled");
        const PREFIX: &'static str = CFG_FEATURE_VERSION_PREFIX;
        const END: u32 = !0;

        // Skip if there are no restrictions.
        if &*self.0 == &[0..!0] { return Ok(()); }

        // Here we go!
        try!(write!(fmt, "#[cfg(any("));
        for (i, Range { start: a, end: b }) in self.0.iter().cloned().enumerate() {
            try!(write!(fmt, "{}", if i == 0 { "" } else { ", " }));
            match (a, b) {
                (0, b) => try!(write!(fmt, "not(feature=\"{}{:08x}\")", PREFIX, b)),
                (a, END) => try!(write!(fmt, "feature=\"{}{:08x}\"", PREFIX, a)),
                (a, b) => try!(write!(fmt,
                    "all(feature=\"{0}{1:08x}\", not(feature=\"{0}{2:08x}\"))",
                    PREFIX, a, b))
            }
        }
        try!(write!(fmt, "))] "));
        Ok(())
    }
}

impl From<WinVersion> for WinVersions {
    fn from(v: WinVersion) -> WinVersions {
        match v.next_version() {
            Some(n) => WinVersions(vec![(v as u32)..(n as u32)]),
            None => WinVersions(vec![(v as u32)..!0])
        }
    }
}

impl From<Range<Option<WinVersion>>> for WinVersions {
    fn from(v: Range<Option<WinVersion>>) -> WinVersions {
        match (v.start, v.end) {
            (None, None) => (..).into(),
            (Some(a), None) => (a..).into(),
            (None, Some(b)) => (..b).into(),
            (Some(a), Some(b)) => (a..b).into()
        }
    }
}

impl From<Range<WinVersion>> for WinVersions {
    fn from(v: Range<WinVersion>) -> WinVersions {
        assert!(v.start < v.end);
        WinVersions(vec![(v.start as u32)..(v.end as u32)])
    }
}

impl From<RangeFrom<WinVersion>> for WinVersions {
    fn from(v: RangeFrom<WinVersion>) -> WinVersions {
        WinVersions(vec![(v.start as u32)..!0])
    }
}

impl From<RangeFull> for WinVersions {
    fn from(_: RangeFull) -> WinVersions {
        WinVersions(vec![0..!0])
    }
}

impl From<RangeTo<WinVersion>> for WinVersions {
    fn from(v: RangeTo<WinVersion>) -> WinVersions {
        WinVersions(vec![0..(v.end as u32)])
    }
}

impl From<(RangeTo<WinVersion>, RangeFrom<WinVersion>)> for WinVersions {
    fn from((i, j): (RangeTo<WinVersion>, RangeFrom<WinVersion>)) -> WinVersions {
        assert!(i.end < j.start);
        WinVersions(vec![0..(i.end as u32), (j.start as u32)..!0])
    }
}
