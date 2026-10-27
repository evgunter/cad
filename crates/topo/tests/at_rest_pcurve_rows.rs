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

use geom_brep::{Pcurve, PcurveCache};
use geom_core::{Band, Tol};
use topo::construction::Construct;
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

/// `he`'s row restated one period over (`u + 2π`), certified: a lifted
/// copy of the row, the same curve in a part of the chart the face does
/// not reach.
fn wrong_branch_row(
    body: &Body<f64>,
    face: FaceKey,
    he: HalfEdgeKey,
) -> Result<PcurveCache<f64>, geom_brep::PcurveCertifyError> {
    let cache = body.pcurve(he).unwrap();
    assert!(
        matches!(cache.pcurve(), Pcurve::Harmonic { .. }),
        "a cylinder sheet's rows are harmonic"
    );
    let period = geom_brep::Period::new(core::f64::consts::TAU);
    let shifted = cache.pcurve().shift_branch(1, period);
    let edge = body.get_half_edge(he).unwrap().edge;
    let curve = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap();
    let surface = body
        .get_surface(body.get_face(face).unwrap().surface)
        .unwrap();
    let (t0, t1) = cache.params();
    PcurveCache::certify(shifted, t0, t1, curve.carrier(), surface, band())
}

/// **A loop's rows a whole period over are no rows.** Each row of the
/// sheet's loop restated one period over is a lifted copy of its row,
/// the row with a deck element, which C4 never certifies again: a row is
/// its placed image, so certification refuses each one at check 4, its
/// whole period read as the azimuth's offset, and no such face can be
/// stored for tier 3 to read. Adopted from R1's probes p5/p6 on PR 3759.
#[test]
fn a_loop_moved_a_whole_period_is_no_rows() {
    let (body, face) = sheet();
    for he in halves(&body, face) {
        let row = wrong_branch_row(&body, face, he);
        assert!(
            matches!(
                row,
                Err(geom_brep::PcurveCertifyError::ResidualExceeded {
                    check: geom_brep::PcurveCheck::EnvelopeTerm(geom_brep::EnvelopeTerm::FidelityU),
                    ..
                })
            ),
            "{he:?}: a lifted copy is no row: {row:?}"
        );
    }
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
    let row = PcurveCache::certify(wide, lo, hi, curve.carrier(), &surface, band())
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
