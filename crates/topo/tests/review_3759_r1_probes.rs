//! PCERT reviewer R1's probes on PR 3759 (C4: rows mandatory at rest).



use geom_core::{Band, Tol};
use topo::pcurves::validate_pcurves;
use topo::test_support::{CylFrame, cyl_wall_sheet};
use topo::{Body, FaceKey, HalfEdgeKey, MevSite, NewVertexSide, PcurveMintError};

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
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    (body, face)
}

fn halves(body: &Body<f64>, face: FaceKey) -> Vec<HalfEdgeKey> {
    let f = body.get_face(face).unwrap();
    let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
        panic!()
    };
    body.loop_cycle(first).unwrap()
}

/// Rowless curved face: tier 3 names it `Unminted`.
#[test]
fn p1_rowless_face_reads_unminted() {
    let (mut body, face) = sheet();
    for he in halves(&body, face) {
        body.detach_pcurve(he).unwrap();
    }
    assert_eq!(
        validate_pcurves(&body, band()),
        vec![PcurveMintError::Unminted { face }]
    );
}

/// Half-minted face with a STALE row (another half-edge's row
/// transplanted): the gap AND the stale row are both reported.
#[test]
fn p2_stale_row_on_a_half_minted_face_is_measured() {
    let (mut body, face) = sheet();
    let hs = halves(&body, face);
    let donor = body.pcurve(hs[0]).unwrap().clone();
    body.detach_pcurve(hs[2]).unwrap();
    body.attach_pcurve(hs[1], donor);
    let f = validate_pcurves(&body, band());
    eprintln!("p2 findings: {f:#?}");
    assert!(f.contains(&PcurveMintError::MissingCache { half_edge: hs[2] }));
    assert!(
        f.iter().any(|e| matches!(e, PcurveMintError::Certify { half_edge, .. } if *half_edge == hs[1])),
        "the stale row is re-certified: {f:?}"
    );
}

/// Row from a WIDER sheet on the same chart, transplanted onto a
/// half-minted face: stated over more of the chart than the face
/// reaches.
#[test]
fn p3_wide_row_on_a_half_minted_face() {
    let (mut body, face) = sheet();
    let mut wide = Body::<f64>::new();
    let wface = cyl_wall_sheet(
        &mut wide,
        CylFrame::canonical(1.0),
        None,
        (0.2, 3.0),
        (0.0, 1.0),
        tol(),
    );
    let hs = halves(&body, face);
    let ws = halves(&wide, wface);
    for (i, he) in ws.iter().enumerate() {
        eprintln!("wide {i}: {:?}", wide.pcurve(*he).map(|c| (c.params(), c.pcurve().clone())));
    }
    for (i, he) in hs.iter().enumerate() {
        eprintln!("ours {i}: {:?}", body.pcurve(*he).map(|c| (c.params(), c.pcurve().clone())));
    }
}

/// A null edge holding the loop open on a rowless curved sheet face:
/// tier 3 excuses the face; what does the rest of validation say?
#[test]
fn p4_null_edge_rowless_face() {
    let (mut body, face) = sheet();
    let hs = halves(&body, face);
    body.mev_null(MevSite::Fan { he1: hs[0], he2: hs[0] }, NewVertexSide::Above)
        .unwrap();
    for he in halves(&body, face) {
        body.detach_pcurve(he);
    }
    let t3 = validate_pcurves(&body, band());
    let t1 = topo::validate(&body);
    let t2 = topo::validate_closed(&body);
    let geo = topo::validate_geometric(&body, tol());
    eprintln!("p4 t3={t3:?}\n t1={t1:?}\n t2={t2:?}\n geo={geo:?}");
    // And on the at-rest door a sheet is judged by?
}

/// A row restated on the WRONG BRANCH (u + 2π) of the cylinder, on a
/// half-minted face whose rows beside it are both missing, so no
/// continuity joint can see it: only the window derived from the FACE
/// can (a window hulled from the stored rows contains it by
/// construction).
fn shifted_row(body: &Body<f64>, face: FaceKey, he: HalfEdgeKey) -> geom_brep::PcurveCache<f64> {
    let cache = body.pcurve(he).unwrap();
    let geom_brep::Pcurve::Harmonic { p0, pa, pb, pl } = cache.pcurve().clone() else {
        panic!()
    };
    let tau = core::f64::consts::TAU;
    let shifted = geom_brep::Pcurve::Harmonic {
        p0: geom_core::Point2::new(p0.x + tau, p0.y),
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
    let window = geom_brep::ChartWindow {
        u_min: 0.0,
        u_max: 2.0 * tau,
        v_min: -1.0,
        v_max: 2.0,
    };
    geom_brep::PcurveCache::certify(shifted, t0, t1, curve.carrier(), surface, window, band())
        .unwrap()
}

#[test]
fn p5_wrong_branch_row_on_a_half_minted_face_escapes_the_faces_window() {
    let (mut body, face) = sheet();
    let hs = halves(&body, face);
    let moved = shifted_row(&body, face, hs[0]);
    body.attach_pcurve(hs[0], moved);
    body.detach_pcurve(hs[1]).unwrap();
    body.detach_pcurve(hs[3]).unwrap();
    let f = validate_pcurves(&body, band());
    eprintln!("p5 findings: {f:?}");
    assert!(
        f.iter().any(|e| matches!(e, PcurveMintError::Certify { half_edge, error: geom_brep::PcurveCertifyError::TrimEscape { .. } } if *half_edge == hs[0])),
        "the wrong-branch row escapes the face's window: {f:?}"
    );
}

/// The same row on a COMPLETE face whose every row is shifted
/// consistently: the documented blind spot (certifies in another branch).
#[test]
fn p6_wrong_branch_complete_face() {
    let (mut body, face) = sheet();
    let hs = halves(&body, face);
    let moved: Vec<_> = hs.iter().map(|&he| shifted_row(&body, face, he)).collect();
    for (he, c) in hs.iter().zip(moved) {
        body.attach_pcurve(*he, c);
    }
    eprintln!("p6 findings (complete face, all rows on u+2π): {:?}", validate_pcurves(&body, band()));
}
