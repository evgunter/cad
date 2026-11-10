//! **The row registry of geom's funnel bundles**: the rows every reader
//! of the ring convention ([`crate::RingTorus::of_at`]) and of the spiric
//! regime ([`crate::spiric_regime`]) decides under, held here alone, as
//! `geom_brep::rows`, `topo::rows` and `sweep::rows` hold their crates'.
//!
//! - **The bundle types live here, their constructors private to this
//!   module**: [`RingSites`] and [`SpiricRegimeSites`] are minted only by
//!   their reader enums ([`RingReader::sites`], [`SpiricReader::sites`]),
//!   which are the registries of who reads each convention.
//! - **[`ROSTER`] is read off the enums' variants**, so it has no hand-kept
//!   list, and a `const` assertion fails this crate's build on a name used
//!   twice ([`geom_core::rows::first_shared_in`]). editor-core's tree-wide
//!   test walks this roster with every other crate's.
//!
//! The enums spell their callers' row names (`topo`'s, `geom-brep`'s) as
//! strings only: those names are global to the `k_stats` stream and the
//! ledger already, and this crate depends on nothing of its callers to
//! hold them.

use geom_core::rows::str_eq;
use strum::VariantArray as _;

/// **The site names one reader decides the ring convention under**, one
/// per half, so a refusal and its `k_stats` stream name the reading that
/// met them ([`RingTorus::of_at`](crate::surfaces::RingTorus::of_at)). Minted only by its reader
/// ([`RingReader::sites`]), so no two readers share a name: the roster
/// of every reader's rows is one const, checked distinct when this crate
/// compiles ([`ROSTER`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RingSites {
    tube: &'static str,
    ring: &'static str,
}

impl RingSites {
    /// The rows for the tube half (`r`) and the ring half (`R − r`).
    /// Private to this module: a reader names its rows at
    /// [`RingReader::sites`], and no code mints a bundle elsewhere, this
    /// crate's included.
    ///
    /// # Panics
    ///
    /// Where the two names are equal: each names its own margin. In a
    /// `const` item the panic is a compile error.
    #[must_use]
    const fn rows_as(tube: &'static str, ring: &'static str) -> Self {
        assert!(
            !str_eq(tube, ring),
            "the ring convention's two halves need two names"
        );
        Self { tube, ring }
    }

    /// The ring half's row.
    #[must_use]
    pub const fn ring(self) -> &'static str {
        self.ring
    }

    /// The tube half's row.
    #[must_use]
    pub const fn tube(self) -> &'static str {
        self.tube
    }
}

/// **Every reader of the ring convention**, each with its own rows
/// ([`Self::sites`]): a new reader is a variant here, which the roster
/// ([`ROSTER`]) reads off this enum, so its names are
/// checked against every other reader's when this crate compiles.
///
/// **This enum is the registry of who reads the convention.** It spells
/// its callers' row names, `topo`'s and `geom-brep`'s, as strings only:
/// those names are global to the `k_stats` stream and the ledger
/// (`docs/predicate-dimension-audit.md`) already, and this crate depends
/// on nothing of its callers to hold them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, strum::VariantArray)]
pub enum RingReader {
    /// The convention's own rows ([`torus_tube`](crate::surfaces::torus_tube), [`ring_torus`](crate::surfaces::ring_torus)), which
    /// tier 3, the pierce normal and the plane×torus section read.
    Convention,
    /// `topo`'s self-intersection engine, deciding a torus face
    /// injective by its kind.
    SelfIntersection,
    /// The Boolean's spline-sheet reduction, deciding a torus carrier a
    /// ring before reading a sheet off it.
    BooleanSheet,
    /// The Boolean's rider, deciding a torus face a ring before reading
    /// an edge's carrier on it.
    BooleanRider,
}

impl RingReader {
    /// The reader's rows, its tube half's and then its ring half's.
    #[must_use]
    pub const fn sites(self) -> RingSites {
        match self {
            Self::Convention => RingSites::rows_as("torus_tube_positive", "ring_torus_convention"),
            Self::SelfIntersection => RingSites::rows_as(
                "self_intersection_torus_tube",
                "self_intersection_ring_torus",
            ),
            Self::BooleanSheet => RingSites::rows_as(
                "bool_spline_sheet_torus_tube",
                "bool_spline_sheet_ring_torus",
            ),
            Self::BooleanRider => {
                RingSites::rows_as("bool_rider_torus_tube", "bool_rider_ring_torus")
            }
        }
    }
}

/// **The site names one reader of [`spiric_regime`](crate::curves::spiric_regime) decides under**, so a
/// refusal and its `k_stats` stream name the reading that met them:
/// three distinct names, one per margin.
/// Minted only by its reader ([`SpiricReader::sites`]), so no two readers
/// share a name ([`ROSTER`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpiricRegimeSites {
    off_axis: &'static str,
    two_ovals: [&'static str; 2],
}

impl SpiricRegimeSites {
    /// The rows for the offset off zero (`off_axis`) and the two-oval
    /// margin's far and floor rows ([`geom_core::k_stats::decide_across`]).
    /// Private to this module: a reader names its rows at
    /// [`SpiricReader::sites`], and no code mints a bundle elsewhere, this
    /// crate's included.
    ///
    /// # Panics
    ///
    /// Where two of the names are equal: each names its own margin. In a
    /// `const` item the panic is a compile error.
    #[must_use]
    const fn rows_as(off_axis: &'static str, far: &'static str, floor: &'static str) -> Self {
        assert!(
            !str_eq(off_axis, far) && !str_eq(off_axis, floor) && !str_eq(far, floor),
            "a spiric regime's three rows need three names"
        );
        Self {
            off_axis,
            two_ovals: [far, floor],
        }
    }
}

impl SpiricRegimeSites {
    /// The reader's three rows: off the axis, then the two-oval margin's
    /// far and floor rows.
    #[must_use]
    pub(crate) const fn rows(self) -> [&'static str; 3] {
        [self.off_axis, self.two_ovals[0], self.two_ovals[1]]
    }
}

/// **Every reader of [`spiric_regime`](crate::curves::spiric_regime)**, each with its own rows
/// ([`Self::sites`]): a new reader is a variant here, which the roster
/// ([`ROSTER`]) reads off this enum, so its names are
/// checked against every other reader's when this crate compiles.
///
/// **This enum is the registry of who reads the spiric regime.** It spells
/// its callers' row names, `topo`'s and `geom-brep`'s, as strings only:
/// those names are global to the `k_stats` stream and the ledger
/// (`docs/predicate-dimension-audit.md`) already, and this crate depends
/// on nothing of its callers to hold them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, strum::VariantArray)]
pub enum SpiricReader {
    /// [`Curve3::spiric`](crate::Curve3::spiric)'s mint.
    Mint,
    /// The Boolean's rider, reading a spiric edge against a curved face.
    BooleanRider,
    /// The plane×torus section, choosing the spiric it mints.
    PlaneTorus,
}

impl SpiricReader {
    /// The reader's rows.
    #[must_use]
    pub const fn sites(self) -> SpiricRegimeSites {
        match self {
            Self::Mint => SpiricRegimeSites::rows_as(
                "spiric_off_axis",
                "spiric_two_ovals",
                "spiric_two_ovals_floor",
            ),
            Self::BooleanRider => SpiricRegimeSites::rows_as(
                "bool_rider_spiric_off_axis",
                "bool_rider_spiric_regime",
                "bool_rider_spiric_regime_floor",
            ),
            Self::PlaneTorus => SpiricRegimeSites::rows_as(
                "pt_spiric_off_axis",
                "pt_spiric_two_ovals",
                "pt_spiric_two_ovals_floor",
            ),
        }
    }
}

impl SpiricRegimeSites {
    /// The offset-off-zero row.
    #[must_use]
    pub(crate) const fn off_axis(self) -> &'static str {
        self.off_axis
    }

    /// The two-oval margin's far and floor rows.
    #[must_use]
    pub(crate) const fn two_ovals(self) -> [&'static str; 2] {
        self.two_ovals
    }
}

const RINGS: &[RingReader] = RingReader::VARIANTS;
const SPIRICS: &[SpiricReader] = SpiricReader::VARIANTS;

/// How many rows the readers name.
const COUNT: usize = 2 * RINGS.len() + 3 * SPIRICS.len();

/// Every reader's rows: each ring reader's tube and ring, then each
/// spiric reader's three.
const ROWS: [&str; COUNT] = {
    let mut out = [""; COUNT];
    let mut i = 0;
    while i < RINGS.len() {
        let sites = RINGS[i].sites();
        out[2 * i] = sites.tube();
        out[2 * i + 1] = sites.ring();
        i += 1;
    }
    let base = 2 * RINGS.len();
    let mut j = 0;
    while j < SPIRICS.len() {
        let rows = SPIRICS[j].sites().rows();
        let mut k = 0;
        while k < 3 {
            out[base + 3 * j + k] = rows[k];
            k += 1;
        }
        j += 1;
    }
    out
};

/// **Every reader's rows, as the roster every crate's registry gives**:
/// one entry, read off the reader enums.
pub const ROSTER: &[&[&str]] = &[&ROWS];

const _: () = assert!(
    geom_core::rows::first_shared_in(ROSTER).is_none(),
    "two readers of the ring convention or the spiric regime share a row name"
);

#[cfg(test)]
mod tests {
    use super::{COUNT, ROWS, SpiricRegimeSites};

    /// **The roster holds every reader's rows**, none empty.
    #[test]
    fn the_roster_holds_every_readers_rows() {
        assert_eq!(
            COUNT,
            2 * 4 + 3 * 3,
            "four ring readers, three spiric readers"
        );
        assert!(ROWS.iter().all(|r| !r.is_empty()), "{ROWS:?}");
        assert!(ROWS.contains(&"self_intersection_ring_torus"), "{ROWS:?}");
        assert!(ROWS.contains(&"pt_spiric_two_ovals_floor"), "{ROWS:?}");
    }

    /// **A reader's three regime rows need three names**: one name on
    /// two margins would merge them in the `k_stats` stream.
    #[test]
    #[should_panic(expected = "three rows need three names")]
    fn spiric_regime_sites_refuse_a_shared_name() {
        let shared = std::hint::black_box("shared");
        let _ = SpiricRegimeSites::rows_as(shared, shared, "floor");
    }
}
