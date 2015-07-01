/**
Contains everything relating to the feature set abstraction.

Feature sets are used to work out under what conditions something should exist.  Currently, this is based on three things:

1. API Partitions (Desktop, Metro, Phone).
2. Windows Versions.
3. Architectures.
*/
use std::fmt;
use clang;

pub mod archs;
pub mod cc;
pub mod parts;
pub mod scan;
pub mod winvers;

pub use self::archs::Architectures;
pub use self::parts::Partitions;
pub use self::scan::scan_features;
pub use self::winvers::WinVersions;

/**
Combines the various parts of a feature set into one glorious whole.

The major reason for this is how `None` is handled.  `None` is *not* the same as "everything" or "nothing".  The problem is that it's often necessary to compute the complement of a feature set.  For example, `!X86_32` should be `(X86_64|Arm)`; without ignoring `None`s, we'd *also* end up excluding all versions of windows and all partitions!

**Note**: There are `From` implementations for each of the component feature set types.
*/
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Features {
    pub parts: Option<Partitions>,
    pub winver: Option<WinVersions>,
    pub arch: Option<Architectures>,
}

impl Features {
    /**
    Determines whether a feature set is "valid", and returns it in a `Result`.

    This is primarily to check for cases where one or more parts of the feature set exclude *everything*.  This shouldn't *ever* happen, and is an excellent sign that something has gone horribly wrong somewhere.
    */
    pub fn check_valid(self) -> Result<Features, &'static str> {
        if let Some(ref parts) = self.parts {
            if !parts.is_any() { return Err("cannot have empty partition set"); }
        }
        if let Some(ref winver) = self.winver {
            if !winver.is_any() { return Err("cannot have empty winver set"); }
        }
        if let Some(ref arch) = self.arch {
            if !arch.is_any() { return Err("cannot have empty architecture set"); }
        }
        Ok(self)
    }

    /// Compute the complement of this feature set.
    pub fn complement(self) -> Features {
        Features {
            parts: { debug!(".. parts..."); self.parts.map(|p| !p) },
            winver: { debug!(".. winver..."); self.winver.map(WinVersions::complement) },
            arch: { debug!(".. arch..."); self.arch.map(|a| !a) },
        }
    }

    /// Compute the intersection of two feature sets.
    pub fn and(self, other: Features) -> Features {
        Features {
            parts: match (self.parts, other.parts) {
                (None, None) => None,
                (Some(a), None) | (None, Some(a)) => Some(a),
                (Some(a), Some(b)) => Some(a & b)
            },
            winver: match (self.winver, other.winver) {
                (None, None) => None,
                (Some(a), None) | (None, Some(a)) => Some(a),
                (Some(a), Some(b)) => Some(a.intersect(b))
            },
            arch: match (self.arch, other.arch) {
                (None, None) => None,
                (Some(a), None) | (None, Some(a)) => Some(a),
                (Some(a), Some(b)) => Some(a & b)
            },
        }
    }

    /// Compute the union of two feature sets.
    pub fn or(self, other: Features) -> Features {
        Features {
            parts: match (self.parts, other.parts) {
                (None, None) => None,
                (Some(a), None) | (None, Some(a)) => Some(a),
                (Some(a), Some(b)) => Some(a | b)
            },
            winver: match (self.winver, other.winver) {
                (None, None) => None,
                (Some(a), None) | (None, Some(a)) => Some(a),
                (Some(a), Some(b)) => Some(a.union(b))
            },
            arch: match (self.arch, other.arch) {
                (None, None) => None,
                (Some(a), None) | (None, Some(a)) => Some(a),
                (Some(a), Some(b)) => Some(a | b)
            },
        }
    }
}

impl Default for Features {
    fn default() -> Self {
        Features {
            parts: None,
            winver: None,
            arch: None,
        }
    }
}

impl fmt::Display for Features {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if let Some(ref parts) = self.parts { try!(write!(fmt, "{}", parts)); }
        if let Some(ref winver) = self.winver { try!(write!(fmt, "{}", winver)); }
        if let Some(ref arch) = self.arch { try!(write!(fmt, "{}", arch)); }
        Ok(())
    }
}

impl From<Partitions> for Features {
    fn from(v: Partitions) -> Features {
        Features {
            parts: Some(v),
            ..Features::default()
        }
    }
}

impl From<WinVersions> for Features {
    fn from(v: WinVersions) -> Features {
        Features {
            winver: Some(v),
            ..Features::default()
        }
    }
}

impl From<Architectures> for Features {
    fn from(v: Architectures) -> Features {
        Features {
            arch: Some(v),
            ..Features::default()
        }
    }
}

/// Determines whether any of the given tokens are "important".
pub fn has_important_defines(toks: &[String]) -> bool {
    toks.iter().any(|tok| is_important_define(&**tok))
}

/**
Determines whether this token, interpreted as an identifier, is "important".

The practical definition of "important" is: any identifier whose presence *forces* us to correctly parse and evaluate a conditional compilation expression.

In other words, anything that could affect a feature set.
*/
pub fn is_important_define(tok: &str) -> bool {
    match tok {
        // Architecture defines
        "_X86_"
        | "__X86__"
        | "__i386__"
        | "_M_IX86"
        | "_AMD64_"
        | "__x86_64"
        | "__x86_64__"
        | "_M_AMD64"
        | "_M_X64"
        | "__arm__"
        | "_ARM_"
        | "_M_ARM"

        // Obsolete or uninteresting architectures
        | "_M_MRX000"
        | "_M_ALPHA"
        | "_M_PPC"
        | "__ia64__"
        | "_IA64_"
        | "_M_IA64"

        // Important winapi defines
        | "WIN64" | "_WIN64" | "__WIN64" | "__WIN64__"
        | "WINVER"
        | "_WIN32_WINNT"
        | "NTDDI_VERSION"
        | "WINAPI_FAMILY_PARTITION"
        | "WINAPI_FAMILY_ONE_PARTITION"
        | "WINAPI_PARTITION_DESKTOP"
        | "WINAPI_PARTITION_APP"
        | "WINAPI_PARTITION_PC_APP"
        | "WINAPI_PARTITION_PHONE_APP"

        => true,
        _ => false
    }
}

/**
Given an identifier, work out the feature set it represents.
*/
pub fn define_feature(name: &str) -> Features {
    debug!("define_feature({:?})", name);
    match cc::Node::eval_defined(name) {
        Ok(value) => {
            match value.to_features() {
                Ok(f) => f,
                Err(err) => panic!("error defining feature {:?}: {}", name, err)
            }
        },
        Err(err) => panic!("error defining feature {:?}: {}", name, err)
    }
}

/**
Given a conditional compilation expression, work out the feature set it represents.
*/
pub fn define_feature_expr(toks: &[String], loc: &clang::SourceLocation) -> Features {
    debug!("define_feature_expr({:?}, {})", toks, loc.display_short());
    if !has_important_defines(toks) {
        debug!(".. nothing important");
        return Features::default();
    }

    let node = match cc::parse_conditional_expr(toks) {
        Ok(Some((node, tail))) => {
            if tail.len() != 0 {
                panic!("could not fully parse cc expr at {} {:?}; leftover: {:?}", loc.display_short(), toks, tail)
            }
            node
        },
        Ok(None) => panic!("could not parse cc expr at {} {:?}", loc.display_short(), toks),
        Err(err) => panic!("could not parse cc expr at {} {:?}: {}", loc.display_short(), toks, err)
    };

    debug!(".. node: {:?}", node);

    match node.clone().eval().and_then(|v| v.to_features()) {
        Ok(f) => { debug!(".. f: {:?}", f); f },
        Err(err) => panic!("error evaluating expr at {} {:?}: {}", loc.display_short(), node, err)
    }
}
