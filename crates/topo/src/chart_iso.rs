//! **The iso classification of a chart-bounded face's boundary edges**
//! — which edge is a rim and which a meridian, what constant
//! coordinate each one's own carrier states, and which consecutive
//! edges carry ONE iso side between them.
//!
//! [`crate::chart`] answers what a surface's coordinates are; this
//! module answers what a BOUNDARY EDGE is in them. Its consumer is
//! `mesh`'s boundary walk and chord rule, which read each edge of a
//! curved face's loop at the family the shape door certified
//! ([`traversal_kind`]) before emitting its UV polygon.
//!
//! **What is here is the rule; what is NOT here is the caller's
//! disposition of it.** `mesh` reads the separation band through its
//! own `Eps` newtype and this module takes the answer as a `bool`: a
//! band's spelling belongs to the crate that owns the band.
//!
//! `f64` alone: `mesh` reads `f64` bodies.

use geom::Curve3;

use crate::chart::Chart;

/// The iso classification of a boundary edge on a curved face.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TravKind {
    /// A circle around the surface axis; carries the raw row v.
    Rim {
        /// Raw v (unwrapped at emission for periodic v).
        v_raw: f64,
    },
    /// A u = const boundary; carries the raw column azimuth.
    Meridian {
        /// Raw u in the angle window, from the chart inversion at the edge's
        /// mid-point.
        u_raw: f64,
    },
}

impl TravKind {
    /// Whether two traversals are the same KIND — the first half of
    /// the iso-side run rule, and the half that is decided
    /// structurally rather than at a band.
    pub fn same_kind(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Rim { .. }, Self::Rim { .. }) | (Self::Meridian { .. }, Self::Meridian { .. })
        )
    }
}

/// The azimuth of the edge's mid-parameter carrier point — a
/// representative interior point, never an apex/pole endpoint.
pub fn mid_azimuth(chart: &Chart, curve: &geom_brep::EdgeCurve<f64>) -> f64 {
    chart.u_of(curve.mid_point())
}

/// **The traversal an edge is, given its iso family**: a meridian (`U`)
/// reads its column at the mid-point chart inversion
/// ([`mid_azimuth`]), a rim (`V`) its row from its circle
/// ([`Chart::rim_v`]). The family is the caller's — `mesh` reads the one
/// `geom_brep::props::iso_rectangle_sides` certified for the edge — so
/// this decides nothing; it reads the coordinate.
///
/// `None` for a rim whose carrier is no circle, which has no row to
/// read.
pub fn traversal_kind(
    chart: &Chart,
    curve: &geom_brep::EdgeCurve<f64>,
    family: geom_brep::IsoFamily,
) -> Option<TravKind> {
    match family {
        geom_brep::IsoFamily::U => Some(TravKind::Meridian {
            u_raw: mid_azimuth(chart, curve),
        }),
        geom_brep::IsoFamily::V => match *curve.carrier() {
            Curve3::Circle(circle) => Some(TravKind::Rim {
                v_raw: chart.rim_v(circle.frame().origin(), circle.radius()),
            }),
            _ => None,
        },
    }
}

/// Which traversals OPEN an iso side, cyclically (issue #653) — the
/// RULE, in one home.
///
/// An iso side of a curved face may be carried by SEVERAL edges: a
/// vertex dropped on it by [`crate::Body::split_edge`](crate::construction::Construct::split_edge), a boolean, or
/// an exporter emitting two collinear edges. Two consecutive
/// traversals continue ONE side when they are the same KIND and the
/// junction between them lies OFF the chart axis; anything else opens
/// a side.
///
/// `separated[k]` is the second half of that conjunction: "the
/// junction traversal `k` starts at is separated from the chart axis".
/// It arrives as an answer rather than as a length because the BAND
/// that decides it is the caller's — `mesh` spells it
/// `Eps::separates(chart.radial(junction))` through the newtype that
/// owns every one of that crate's terminal ε reads, and the seam is
/// visible in this signature rather than hidden in a copy of the rule.
///
/// Every chart singularity lies ON THE AXIS — a sphere's poles, a
/// cone's apex — so one separation answer covers them all, without a
/// match on [`crate::ChartKind`] and without consulting
/// [`Chart::poles`]. Within the band an azimuth carries no
/// distinguishable direction, so the answer there is to break the run
/// and keep the per-edge coordinate.
///
/// Cyclic: traversal `k`'s predecessor is `k - 1` modulo the length,
/// so a run that wraps past index 0 is seen as one run. Fewer than two
/// traversals: everything opens, since there is no predecessor to
/// continue from. **Nothing here forces an opening to exist** — a loop
/// whose every traversal reads as a continuation is a single cyclic
/// run with none, and each caller resolves that for itself.
pub fn iso_side_starts(kinds: &[TravKind], separated: &[bool]) -> Vec<bool> {
    let m = kinds.len();
    debug_assert_eq!(
        m,
        separated.len(),
        "one separation answer per traversal: the caller reads its own band at each \
         junction, so a length mismatch is a caller that lost one"
    );
    if m < 2 {
        // A single traversal has no predecessor to continue from, and
        // an empty loop has nothing to mark. Hoisted: it does not vary
        // with `k`.
        return vec![true; m];
    }
    (0..m)
        .map(|k| !(kinds[(k + m - 1) % m].same_kind(&kinds[k]) && separated[k]))
        .collect()
}
