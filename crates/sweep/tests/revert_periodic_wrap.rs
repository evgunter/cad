//! **`Body::revert` re-parks a periodic chart's loop wrap at the
//! reversed closure.** The one-branch loop walk starts at a loop's
//! `first` and pins every joint it meters to its predecessor's exit,
//! so the one joint a periodic chart's one-period wrap can be
//! REPORTED at is the closure, between the cycle's last half-edge and
//! `first`. Reversed with `first` fixed, that joint sits right after
//! `first`, mid-chain, and tier 3's stored-row continuity pass reports
//! it as a `LoopDiscontinuity` on the second half-edge of the reversed
//! cycle. The reversal moves every loop's anchor to its source
//! predecessor — the same joint is the closure again — and touches no
//! row (`topo::revert` module docs, the anchor bullet;
//! `topo::LoopBoundary::Cycle`'s `first` states the invariant).
//!
//! Which loops wrap at closure, on this tree: the ones with an
//! AZIMUTH-FREE joint. At a sphere's pole or a cone's apex the lever is
//! zero, so the walk meters nothing there in either direction and
//! leaves the meridians on their own base branches; a lune whose
//! meridians meet at the pole then comes back a whole period off at
//! closure. That is what the fixtures measure, not a rule the code
//! states: the walk guarantees only that every joint it METERS is
//! pinned, that an azimuth-free joint is unmetered both ways, and that
//! the closure is the only joint a wrap can be reported at. A torus has
//! no azimuth-free joint (its lever `|R + r·cos v|` never vanishes for
//! `R > r`, which the revolve and tube doors require), and a drum's
//! wall absorbs its azimuth in a full rim, so those loops close exactly
//! and are the controls: re-anchored all the same, rows untouched,
//! tier 3 reporting nothing but the complement.
//!
//! The fixtures are `common::latitude_seam`'s, `shell7_common`'s and
//! `revolve_common`'s, shared with the SHELL-9 suites and the
//! mass-properties rows so every row here measures a body those rows
//! measure.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use core::mem::{Discriminant, discriminant};

use geom_core::Band;
use sweep::Revolution;
use topo::{Body, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, PcurveMintError, ValidationError};

use super::common::approx::twisted_loft;
use super::common::latitude_seam::{collinear_cap_drum, door_cavity, two_arc_sphere};
use super::revolve_common::donut_profile;
use super::shell7_common::*;

/// A cone of radius 1 and height 2 through its apex: the wall's loop
/// runs the base rim and the two seam generators, which meet at the
/// apex — the cone's azimuth-free joint.
fn apex_cone() -> Body<f64> {
    polyline(&[(0.0, 0.0), (1.0, 0.0), (0.0, 2.0)], Revolution::Full)
}

/// A torus of radii `2, 0.5` authored as two cocircular arcs, the same
/// way `two_arc_sphere` is authored — `mass_props`'s donut: one torus
/// in four faces, no azimuth-free joint anywhere on it.
fn two_arc_torus() -> Body<f64> {
    revolved(donut_profile(), Revolution::Full)
}

/// Half a drum: a cylinder wall whose loop never wraps the chart.
fn half_drum() -> Body<f64> {
    wedge(1.0, 2.0, PI)
}

/// Every stored row of `body`, keyed by its half-edge, as `Debug`
/// text: image, interval and certificate, bit for bit.
fn rows(body: &Body<f64>) -> Vec<(HalfEdgeKey, String)> {
    body.pcurves()
        .map(|(he, cache)| (he, format!("{cache:?}")))
        .collect()
}

/// Every cycle's anchor.
fn anchors(body: &Body<f64>) -> Vec<(LoopKey, HalfEdgeKey)> {
    body.loops()
        .filter_map(|(lk, lp)| match lp.boundary {
            LoopBoundary::Cycle { first } => Some((lk, first)),
            LoopBoundary::Empty { .. } => None,
        })
        .collect()
}

/// The azimuth gap the loop's stored rows leave at its CLOSURE — the
/// last half-edge's exit against `first`'s entry, read in the cycle's
/// own order from its anchor, as the tier-3 continuity pass reads it.
/// `None` where a half-edge of the loop carries no row. The entry/exit
/// convention (a half-edge enters at its row's `t0` iff it is its
/// edge's `he_plus`) is a copy of `topo::pcurves`' crate-private
/// `is_plus`, which no test-visible door exposes; it is the only
/// arithmetic here.
fn closure_gap(body: &Body<f64>, lk: LoopKey) -> Option<f64> {
    let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
        return None;
    };
    let cycle = body.loop_cycle(first).unwrap();
    let chart_ends = |he: HalfEdgeKey| {
        let cache = body.pcurve(he)?;
        let (t0, t1) = cache.params();
        let edge = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
        let (entry_t, exit_t) = if edge.he_plus == he {
            (t0, t1)
        } else {
            (t1, t0)
        };
        Some((cache.pcurve().eval(entry_t), cache.pcurve().eval(exit_t)))
    };
    let (start, _) = chart_ends(*cycle.first()?)?;
    let (_, end) = chart_ends(*cycle.last()?)?;
    Some(end.x - start.x)
}

/// Whether `gap` is a whole period, EXACTLY: the walk's own azimuths on
/// these fixtures are `0`, `π` and `τ`, so a wrapped closure reads `±τ`
/// bit for bit, and a tolerance here would only hide a gap that is
/// not one (the tier-3 pass meters the joint through the band; this
/// reader only names which loops wrap).
fn is_whole_period(gap: f64) -> bool {
    gap.abs() == TAU
}

/// The loops whose rows wrap the chart by a whole period at closure.
fn wrapping_loops(body: &Body<f64>) -> Vec<LoopKey> {
    anchors(body)
        .into_iter()
        .filter(|(lk, _)| closure_gap(body, *lk).is_some_and(is_whole_period))
        .map(|(lk, _)| lk)
        .collect()
}

/// What `chart_boundary` answers for every face of `body`, by result
/// KIND (`Ok`, or which refusal variant), in face order. Its closure
/// lever is `chart_u_arm` at the FIRST edge's entry, so the anchor move
/// changes where a refusal would fire; this reader is what pins that
/// it does not change whether one fires.
fn boundary_kinds(body: &Body<f64>) -> Vec<(FaceKey, Result<(), Discriminant<PcurveMintError>>)> {
    let band = Band::linear(tol()).unwrap();
    body.faces()
        .map(|(fk, face)| {
            let chart = body.get_surface(face.surface).unwrap();
            let kind = topo::chart_boundary(body, fk, chart, band)
                .map(|_| ())
                .map_err(|e| discriminant(&e));
            (fk, kind)
        })
        .collect()
}

/// **The reversal's anchor move, on `body`**: every loop's anchor is
/// its source predecessor, every row is the source's bit for bit,
/// tier 3 of the reverted body reports exactly the complement, the
/// wrap of every wrapping loop sits at the reversed closure too,
/// `chart_boundary` answers the same kind for every face, and the
/// involution restores the bits.
fn assert_reparked(label: &str, body: &Body<f64>) {
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "{label}: the source is tier-3 valid"
    );
    let reverted = body.revert().expect("revert");
    assert_eq!(
        topo::validate_geometric(&reverted, tol()),
        Err(vec![ValidationError::NegativeVolume]),
        "{label}: a reverted body bounds the complement and nothing else fails"
    );
    assert_eq!(rows(&reverted), rows(body), "{label}: no row moves");
    for (lk, first) in anchors(body) {
        let LoopBoundary::Cycle { first: after } = reverted.get_loop(lk).unwrap().boundary else {
            panic!("{label}: a cycle stays a cycle")
        };
        assert_eq!(
            after,
            body.get_half_edge(first).unwrap().prev,
            "{label}: every loop's anchor is its source predecessor"
        );
    }
    for lk in wrapping_loops(body) {
        let gap = closure_gap(&reverted, lk).unwrap();
        assert!(
            is_whole_period(gap),
            "{label}: the wrap of {lk:?} sits at the reversed closure, got {gap}"
        );
    }
    assert_eq!(
        boundary_kinds(&reverted),
        boundary_kinds(body),
        "{label}: chart_boundary answers the same kind for every face of the reversed body"
    );
    assert_eq!(
        format!("{:?}", reverted.revert().unwrap()),
        format!("{body:?}"),
        "{label}: bitwise involution"
    );
}

/// **The red-first row: the two-arc sphere and its door-built cavity.**
/// Each sphere lune's loop wraps at closure (its meridians meet at the
/// pole on their own base branches). On the merge base
/// `validate_geometric` of the reverted body reports a pcurve
/// `LoopDiscontinuity` on the second half-edge of the reversed cycle
/// beside the `NegativeVolume`; at the head it reports the
/// `NegativeVolume` alone, the rows are the cavity's bit for bit, and
/// the wrap sits at the reversed closure.
#[test]
fn reverted_sphere_and_its_cavity_report_only_the_complement() {
    let sphere = two_arc_sphere();
    assert!(
        !wrapping_loops(&sphere).is_empty(),
        "the sphere's lunes wrap at closure"
    );
    assert_reparked("sphere", &sphere);
    assert_reparked("cavity", &door_cavity(&sphere, 0.05));
}

/// **A cone through its apex, the same way.** The apex is the cone's
/// azimuth-free joint, and the wall's loop wraps at closure exactly
/// as a sphere lune's does; on the merge base the reversal reports
/// the same `LoopDiscontinuity`.
#[test]
fn reverted_apex_cone_reports_only_the_complement() {
    let cone = apex_cone();
    assert!(
        !wrapping_loops(&cone).is_empty(),
        "the cone wall's loop wraps at closure"
    );
    assert_reparked("cone", &cone);
}

/// **Controls: periodic charts whose loops close exactly.** Two tori
/// (the tube door's and a revolved two-arc profile's), the collinear-cap
/// drum and its cavity — whose rows are the plane mirror's, unchanged
/// by this — and a half drum, whose cylinder loop never wraps. No loop
/// wraps at closure, the reversal re-anchors their curved loops all
/// the same, no row moves, and tier 3 reports exactly the complement
/// before and after.
#[test]
fn periodic_charts_without_a_closure_wrap_are_re_anchored_and_report_only_the_complement() {
    for (label, body) in [
        ("tube torus", tube_torus(2.0, 0.5)),
        ("two-arc torus", two_arc_torus()),
        ("drum", drum(1.0, 2.0)),
        ("collinear-cap drum", collinear_cap_drum()),
        ("drum cavity", door_cavity(&collinear_cap_drum(), 0.05)),
        ("half drum", half_drum()),
    ] {
        assert!(
            wrapping_loops(&body).is_empty(),
            "{label}: no loop wraps at closure"
        );
        assert!(!rows(&body).is_empty(), "{label}: the fixture carries rows");
        assert_reparked(label, &body);
    }
}

/// **Control: a non-periodic curved chart.** The twisted loft's walls
/// are bilinear NURBS saddles, closed in nothing; their iso-line rows
/// are untouched, their loops re-anchored, and tier 3 of the reversed
/// body reports exactly the complement.
#[test]
fn a_non_periodic_curved_charts_rows_are_untouched() {
    let loft = twisted_loft(0.05);
    assert!(!rows(&loft).is_empty(), "the loft carries rows");
    assert!(wrapping_loops(&loft).is_empty());
    assert_reparked("twisted loft", &loft);
}
