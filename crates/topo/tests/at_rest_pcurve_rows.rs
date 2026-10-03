//! **What tier 3's pcurve pass measures on a stored row** (C4), on a
//! complete face and a half-minted one alike: every stored row must
//! state its edge's interval, and is re-certified against the window
//! its face's stored rows hull out to. A loop's rows standing a whole
//! period over from a fresh walk are the same face (the re-statement
//! `Body::revert` makes), so they read clean either way; a row stated
//! over more of its carrier than its edge spans is refused either way;
//! and a stale row on a half-minted face is measured beside the gap.
//!
//! The fixture is a minted cylinder wall sheet (`cyl_wall_sheet`), tier-3
//! clean as built.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{ChartWindow, Pcurve, PcurveCache};
use geom_core::{Band, Point2, Tol};
use topo::pcurves::validate_pcurves;
use topo::test_support::{CylFrame, cyl_wall_sheet};
use topo::{Body, FaceKey, HalfEdgeKey, PcurveMintError};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn sheet() -> (Body<f64>, FaceKey) {
    let mut body = Body::<f64>::new();
    let face = cyl_wall_sheet(
        &mut body,
        CylFrame::canonical(1.0),
        None,
        (0.2, 1.4),
        (0.0, 1.0),
        tol(),
    );
    assert_eq!(
        validate_pcurves(&body, band()),
        vec![],
        "the sheet is clean"
    );
    (body, face)
}

fn halves(body: &Body<f64>, face: FaceKey) -> Vec<HalfEdgeKey> {
    let f = body.get_face(face).unwrap();
    let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
        panic!("the sheet's outer loop is a cycle")
    };
    body.loop_cycle(first).unwrap()
}

/// `he`'s row restated one period over (`u + 2π`), certified against a
/// window wide enough to hold it: a row about the same curve, stated in
/// a part of the chart the face does not reach.
fn wrong_branch_row(body: &Body<f64>, face: FaceKey, he: HalfEdgeKey) -> PcurveCache<f64> {
    let cache = body.pcurve(he).unwrap();
    let Pcurve::Harmonic { p0, pa, pb, pl } = cache.pcurve().clone() else {
        panic!("a cylinder sheet's rows are harmonic")
    };
    let tau = core::f64::consts::TAU;
    let shifted = Pcurve::Harmonic {
        p0: Point2::new(p0.x + tau, p0.y),
        pa,
        pb,
        pl,
    };
    let edge = body.get_half_edge(he).unwrap().edge;
    let curve = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap();
    let surface = body
        .get_surface(body.get_face(face).unwrap().surface)
        .unwrap();
    let (t0, t1) = cache.params();
    let window = ChartWindow {
        u_min: 0.0,
        u_max: 2.0 * tau,
        v_min: -1.0,
        v_max: 2.0,
    };
    PcurveCache::certify(shifted, t0, t1, curve.carrier(), surface, window, band()).unwrap()
}

/// **A loop's rows a whole period over are the same face**, complete
/// or half-minted. Every row moved one period over, consistently, so
/// every joint still meets and the loop still closes: tier 3 reads the
/// complete face clean, and the half-minted one (a row detached) reads
/// only its gap — the two alike. This is the re-statement
/// `Body::revert` makes, so a fresh walk's branch is no reference for a
/// stored row. (R1's probes p5/p6 on PR 3759 found the two faces read
/// differently against a derived window.)
#[test]
fn a_loop_moved_a_whole_period_reads_the_same_complete_or_half_minted() {
    let (mut body, face) = sheet();
    let hs = halves(&body, face);
    let moved: Vec<_> = hs
        .iter()
        .map(|&he| wrong_branch_row(&body, face, he))
        .collect();
    for (&he, row) in hs.iter().zip(moved) {
        body.attach_pcurve(he, row);
    }
    assert_eq!(validate_pcurves(&body, band()), vec![], "complete");
    body.detach_pcurve(hs[1]).unwrap();
    assert_eq!(
        validate_pcurves(&body, band()),
        vec![PcurveMintError::MissingCache { half_edge: hs[1] }],
        "half-minted: the gap and nothing else"
    );
}

/// **A row stated over more of its carrier than its edge spans is
/// refused**, complete or half-minted — a row from before a split, say,
/// certified over its own wider window. It certifies against the
/// stored rows' hull (it widens that hull), so what refuses it is its
/// interval against its edge's: `RowInterval`, on both faces.
#[test]
fn a_row_wider_than_its_edge_is_refused_complete_or_half_minted() {
    let (mut body, face) = sheet();
    let hs = halves(&body, face);
    let cache = body.pcurve(hs[0]).unwrap().clone();
    let (t0, t1) = cache.params();
    let edge = body.get_half_edge(hs[0]).unwrap().edge;
    let curve = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap();
    let surface = body
        .get_surface(body.get_face(face).unwrap().surface)
        .unwrap()
        .clone();
    let (lo, hi) = (t0, t1 + 0.4 * (t1 - t0));
    let wide = cache.pcurve().clone();
    let window = wide.chart_box(lo, hi);
    let row = PcurveCache::certify(wide, lo, hi, curve.carrier(), &surface, window, band())
        .expect("the carrier's own image certifies over a longer span");
    body.attach_pcurve(hs[0], row);
    let refused = PcurveMintError::RowInterval { half_edge: hs[0] };
    let f = validate_pcurves(&body, band());
    assert!(f.contains(&refused), "complete: {f:?}");
    body.detach_pcurve(hs[2]).unwrap();
    let f = validate_pcurves(&body, band());
    assert!(
        f.contains(&refused) && f.contains(&PcurveMintError::MissingCache { half_edge: hs[2] }),
        "half-minted: the wide row and the gap: {f:?}"
    );
}

/// **A stale row on a half-minted face is measured beside the gap.** A
/// neighbour's row transplanted onto `hs[1]`, and `hs[2]`'s row
/// detached: the gap is reported, and so is the stale row's refusal.
/// (R1's probe p2 on PR 3759.)
#[test]
fn a_stale_row_on_a_half_minted_face_is_measured() {
    let (mut body, face) = sheet();
    let hs = halves(&body, face);
    let donor = body.pcurve(hs[0]).unwrap().clone();
    body.detach_pcurve(hs[2]).unwrap();
    body.attach_pcurve(hs[1], donor);
    let f = validate_pcurves(&body, band());
    assert!(
        f.contains(&PcurveMintError::MissingCache { half_edge: hs[2] }),
        "{f:?}"
    );
    assert!(
        f.iter().any(|e| matches!(
            e,
            PcurveMintError::Certify { half_edge, .. } | PcurveMintError::RowInterval { half_edge }
                if *half_edge == hs[1]
        )),
        "the stale row is measured and refused: {f:?}"
    );
}
