//! **The row registry of sweep's funnel bundles**: every bundle of
//! predicate rows a reader in this crate decides under, held here alone,
//! as `geom_brep::rows` and `topo::rows` hold theirs. The bundle type's
//! fields are private to this module; every reader's bundle is a `const`
//! here; [`ROSTER`] lists their names, and a `const` assertion fails this
//! crate's build on a name used twice ([`geom_core::rows::first_shared_in`]).
//! editor-core's tree-wide test walks this roster with every other crate's.

/// The predicate names one verb's cosurface decision reports under —
/// the K recorder meters each sweep's line and arc margins separately,
/// so the names are per-verb data passed into the shared body rather
/// than a property of the body.
///
/// **A table of row names, so a new value here is a roster change**
/// (`docs/K-REPORT.md`, "The inventory method, restated"): a name
/// reaching the funnel as a field is invisible to any grep for a
/// literal at the decide site.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CosurfaceNames {
    /// The line/line predicate (chord collinearity).
    lines: &'static str,
    /// The arc/arc predicate (carrier identity).
    arcs: &'static str,
}

impl CosurfaceNames {
    /// The line/line row.
    #[must_use]
    pub(crate) const fn lines(&self) -> &'static str {
        self.lines
    }

    /// The arc/arc row.
    #[must_use]
    pub(crate) const fn arcs(&self) -> &'static str {
        self.arcs
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub(crate) const fn names(&self) -> [&'static str; 2] {
        [self.lines, self.arcs]
    }
}

/// The extrude's side walls (`extrude`).
pub(crate) const SIDE_COSURFACE: CosurfaceNames = CosurfaceNames {
    lines: "side_planes_cosurface",
    arcs: "side_cylinders_cosurface",
};

/// The revolve's walls (`revolve`).
pub(crate) const WALL_COSURFACE: CosurfaceNames = CosurfaceNames {
    lines: "wall_lines_cosurface",
    arcs: "wall_arcs_cosurface",
};

/// **Every registered bundle's rows**, one entry per const above, kept
/// beside them by hand.
pub const ROSTER: &[&[&str]] = &[&SIDE_COSURFACE.names(), &WALL_COSURFACE.names()];

const _: () = assert!(
    geom_core::rows::first_shared_in(ROSTER).is_none(),
    "two of sweep's registered row bundles share a row name"
);
