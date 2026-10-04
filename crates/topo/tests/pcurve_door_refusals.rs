//! **What the pcurve doors refuse on a sound body** (D2 row 4): a key
//! the caller passed that does not resolve is
//! [`PcurveMintError::Stale`], stating the role it was passed as and
//! checked before any work; a half of a null edge — scaffolding a public
//! door mints — is [`PcurveMintError::NoCarrier`]; and a face whose
//! outer loop is a lone vertex describes no chart boundary,
//! [`PcurveMintError::EmptyOuter`]. Each refusal leaves the body as
//! found.
//!
//! The fixture is a minted cylinder wall sheet (`cyl_wall_sheet`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol};
use topo::test_support::{CylFrame, cyl_wall_sheet};
use topo::{
    Body, EntityId, FaceKey, HalfEdgeKey, MevSite, NewVertexSide, PcurveMintError, chart_boundary,
    mint_pcurves, mint_pcurves_of, pcurve_of,
};

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
    (body, face)
}

/// Every stored row, in a comparable form.
fn rows(body: &Body<f64>) -> Vec<String> {
    let mut out: Vec<String> = body
        .pcurves()
        .map(|(he, c)| format!("{he:?} {:?} {:?}", c.params(), c.pcurve()))
        .collect();
    out.sort();
    out
}

fn first_half(body: &Body<f64>, face: FaceKey) -> HalfEdgeKey {
    let outer = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the wall is bounded by a cycle")
    };
    first
}

/// A key no body holds, at each door that takes one: refused `Stale`
/// under the role it was passed as, and `mint_pcurves_of` refuses it
/// before clearing a row, even with a live face beside it.
#[test]
fn a_callers_key_that_does_not_resolve_is_refused_stale_under_its_role() {
    let (mut body, wall) = sheet();
    let he = HalfEdgeKey::default();
    let face = FaceKey::default();
    assert_eq!(
        pcurve_of(&body, he, band()).err(),
        Some(PcurveMintError::Stale {
            role: "half_edge",
            key: EntityId::HalfEdge(he),
        })
    );
    let chart = body
        .get_surface(body.get_face(wall).unwrap().surface)
        .unwrap()
        .clone();
    assert_eq!(
        chart_boundary(&body, face, &chart, band()).err(),
        Some(PcurveMintError::Stale {
            role: "face",
            key: EntityId::Face(face),
        })
    );
    let before = rows(&body);
    assert!(!before.is_empty(), "the sheet is minted");
    assert_eq!(
        mint_pcurves_of(&mut body, &[wall, face], tol()),
        Err(PcurveMintError::Stale {
            role: "faces",
            key: EntityId::Face(face),
        })
    );
    assert_eq!(rows(&body), before, "the wall's rows are left as found");
    let text = PcurveMintError::Stale {
        role: "faces",
        key: EntityId::Face(face),
    }
    .to_string();
    assert_eq!(
        test_utils::refusal::recourse_markers(&text),
        0,
        "a caller's key states the fact: {text}"
    );
}

/// A null edge on the minted wall: its half has no chart image
/// (`pcurve_of`), and the whole-body mint refuses the wall typed,
/// naming a half of that edge, with every row left as found.
#[test]
fn a_null_edges_half_has_no_carrier_and_the_mint_leaves_the_body_as_found() {
    let (mut body, wall) = sheet();
    let first = first_half(&body, wall);
    let null = body
        .mev_null(
            MevSite::Fan {
                he1: first,
                he2: first,
            },
            NewVertexSide::Above,
        )
        .unwrap();
    assert_eq!(
        pcurve_of(&body, null.he_plus, band()).err(),
        Some(PcurveMintError::NoCarrier {
            half_edge: null.he_plus
        })
    );
    let before = rows(&body);
    match mint_pcurves(&mut body, tol()) {
        Err(PcurveMintError::NoCarrier { half_edge }) => assert!(
            half_edge == null.he_plus || half_edge == null.he_minus,
            "the refusal names a half of the null edge: {half_edge:?}"
        ),
        other => panic!("expected NoCarrier, got {other:?}"),
    }
    assert_eq!(rows(&body), before, "a refused mint writes nothing");
}

/// A face whose outer loop is a lone vertex (`mvfs`'s) bounds no region
/// of any chart: refused `EmptyOuter`, not described as empty.
#[test]
fn a_lone_vertex_outer_loop_describes_no_chart_boundary() {
    let (mut body, wall) = sheet();
    let chart = body
        .get_surface(body.get_face(wall).unwrap().surface)
        .unwrap()
        .clone();
    let lone = body.mvfs(Point3::new(5.0, 0.0, 0.0), true).unwrap();
    assert_eq!(
        chart_boundary(&body, lone.face, &chart, band()).err(),
        Some(PcurveMintError::EmptyOuter { face: lone.face })
    );
}
