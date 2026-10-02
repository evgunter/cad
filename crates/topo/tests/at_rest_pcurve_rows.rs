//! **What tier 3's pcurve pass measures on a stored row** (C4): every
//! stored row is re-certified against the window the face's own
//! derivation hulls out to, on a complete face and a half-minted one
//! alike — so a row stated over a part of the chart the face does not
//! reach (another branch of the azimuth) escapes it either way, and a
//! stale row on a half-minted face is measured beside the gap.
//!
//! The fixture is a minted cylinder wall sheet (`cyl_wall_sheet`), tier-3
//! clean as built.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{ChartWindow, Pcurve, PcurveCache, PcurveCertifyError};
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

fn escapes(findings: &[PcurveMintError], he: HalfEdgeKey) -> bool {
    findings.iter().any(|e| {
        matches!(
            e,
            PcurveMintError::Certify {
                half_edge,
                error: PcurveCertifyError::TrimEscape,
            } if *half_edge == he
        )
    })
}

/// **A wrong-branch row escapes the face's window on a half-minted
/// face**, where no continuity joint can see it (both its neighbours
/// miss their rows): only the window derived from the FACE can. Kills
/// the mutant that hulls a half-minted face's window from its own
/// stored rows. (Adopted from PCERT reviewer R1's probe p5 on PR 3759.)
#[test]
fn a_wrong_branch_row_on_a_half_minted_face_escapes_the_faces_window() {
    let (mut body, face) = sheet();
    let hs = halves(&body, face);
    let moved = wrong_branch_row(&body, face, hs[0]);
    body.attach_pcurve(hs[0], moved);
    body.detach_pcurve(hs[1]).unwrap();
    body.detach_pcurve(hs[3]).unwrap();
    let f = validate_pcurves(&body, band());
    assert!(
        f.contains(&PcurveMintError::MissingCache { half_edge: hs[1] })
            && f.contains(&PcurveMintError::MissingCache { half_edge: hs[3] }),
        "the gaps are reported: {f:?}"
    );
    assert!(escapes(&f, hs[0]), "the wrong-branch row escapes: {f:?}");
}

/// **And on a complete face too.** Every row moved one period over,
/// consistently, so every joint still meets and the loop still closes:
/// a window hulled from the stored rows holds them by construction,
/// and only the face's derived window refuses them — one escape per
/// row. (R1's probe p6 on PR 3759, which read clean.)
#[test]
fn wrong_branch_rows_on_a_complete_face_escape_the_faces_window() {
    let (mut body, face) = sheet();
    let hs = halves(&body, face);
    let moved: Vec<_> = hs
        .iter()
        .map(|&he| wrong_branch_row(&body, face, he))
        .collect();
    for (&he, row) in hs.iter().zip(moved) {
        body.attach_pcurve(he, row);
    }
    let f = validate_pcurves(&body, band());
    for &he in &hs {
        assert!(escapes(&f, he), "{he:?} escapes the face's window: {f:?}");
    }
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
        f.iter().any(
            |e| matches!(e, PcurveMintError::Certify { half_edge, .. } if *half_edge == hs[1])
        ),
        "the stale row is re-certified and refused: {f:?}"
    );
}
