//! **The iso classification of a chart-bounded face's boundary edges**
//! — which edge is a rim and which a meridian, what constant
//! coordinate each one's own carrier states, and which consecutive
//! edges carry ONE iso side between them.
//!
//! [`crate::chart`] answers what a surface's coordinates are; this
//! module answers what a BOUNDARY EDGE is in them. Two consumers run
//! these expressions and there is one copy of each:
//!
//! - `mesh`'s boundary walk, which classifies every edge of a curved
//!   face's loop before emitting its UV polygon;
//! - [`crate::coherence`], which measures how far the body's own two
//!   accounts of one of those constant coordinates disagree.
//!
//! **What is here is the rule; what is NOT here is each caller's
//! disposition of it.** `mesh` maps a non-iso carrier onto its own
//! typed refusal and this module answers `None`; `mesh` reads the
//! separation band through its own `Eps` newtype and this module takes
//! the answer as a `bool`. Those two seams are deliberate and are
//! named at the items below: a disposition belongs to the crate that
//! disposes, and a band's spelling belongs to the crate that owns the
//! band.
//!
//! `f64` alone, for the reason [`crate::chart`] gives: these are
//! float-path facts about `f64` coordinates.

use geom::Curve3;
use geom_brep::EdgeDescription;

use crate::chart::Chart;

/// Full turn — the period of the azimuth these functions unwrap on.
pub const TAU: f64 = core::f64::consts::TAU;

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
        /// Raw u ∈ (−π, π] (exactly 0.0 for `Seam` edges).
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
    let (t0, t1) = curve.params();
    chart.u_of(curve.carrier().eval(t0 + (t1 - t0) * 0.5))
}

/// `raw + 2πk` nearest `prev`.
pub fn unwrap_near(raw: f64, prev: f64) -> f64 {
    raw + TAU * ((prev - raw) / TAU).round()
}

/// Rim-vs-meridian classification: `Seam` descriptions and line
/// carriers are meridians; circle carriers split on axis alignment
/// (structurally either parallel — a rim — or perpendicular — a
/// meridian; 0.5 splits the two classes with maximal margin).
///
/// A meridian's column u always comes from the mid-point chart
/// inversion — **never** from the edge kind: a `Seam` edge is the
/// surface's `u_ref`-half-plane meridian, whose chart u is 0 on
/// ordinary kinds but π on a cone's mirror nappe (the kernel defines
/// the seam spatially via `u_ref`; [`Chart::u_of`] carries the nappe
/// correction).
///
/// **`None` is not a refusal, it is the absence of a classification**:
/// a conic or spline carrier is no chart iso curve, so this function
/// has no answer about it. What that MEANS is the caller's, and the
/// two callers mean different things by it — `mesh` reaches such a
/// carrier only as a dispatch defect and surfaces it typed, while
/// [`crate::coherence`] records the loop as unexamined and carries on.
/// Neither disposition belongs here.
pub fn classify_kind(chart: &Chart, curve: &geom_brep::EdgeCurve<f64>) -> Option<TravKind> {
    if matches!(curve.description(), EdgeDescription::Chart(c) if c.seam) {
        return Some(TravKind::Meridian {
            u_raw: mid_azimuth(chart, curve),
        });
    }
    match *curve.carrier() {
        Curve3::Line { .. } => Some(TravKind::Meridian {
            u_raw: mid_azimuth(chart, curve),
        }),
        Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } => {
            if axis.dot(chart.axis).abs() > 0.5 {
                Some(TravKind::Rim {
                    v_raw: chart.rim_v(center, radius),
                })
            } else {
                Some(TravKind::Meridian {
                    u_raw: mid_azimuth(chart, curve),
                })
            }
        }
        Curve3::Ellipse { .. } | Curve3::Nurbs(_) => None,
    }
}

/// Which traversals OPEN an iso side, cyclically (issue #653) — the
/// RULE, in one home.
///
/// An iso side of a curved face may be carried by SEVERAL edges: a
/// vertex dropped on it by [`crate::Body::split_edge`], a boolean, or
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
/// owns every one of that crate's terminal ε reads, and
/// [`crate::coherence`] spells the same comparison against the run's
/// ε. One rule, two bands, and the seam is visible in this signature
/// rather than hidden in a second copy of the rule.
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
