//! **An Euler operator that adds a half-edge to a minted curved face
//! mints that half-edge's pcurve row at the mint site.** The forcing
//! fixture is `split_edge_pcurve_rows`' minted cylinder-wall sheet: a
//! curved chart, so the face stores rows, and a ruling to grow struts
//! along — a line ON the cylinder, whose chart image has a closed form.
//!
//! The claim under every row is one sentence: no Euler operator returns
//! a face half-minted. A face whose rows were complete is complete after
//! the op, with the rows the minting pass would derive; a face that
//! stored no row still stores none; a face that was already half-minted
//! is left as found; and where the row cannot be minted the op refuses
//! before it mutates.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol};
use topo::pcurves::validate_pcurves;
use topo::test_support::{CylFrame, cyl_wall_sheet};
use topo::{Body, FaceKey, HalfEdgeKey, MekrSite, MevSite, PcurveMintError, VertexKey};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// The ruling the struts grow along.
const UM: f64 = 0.8;

/// The minted wall sheet over `[0.2, 1.4] x [0, 1]` of the canonical
/// unit cylinder, with its bottom rim split at the ruling `UM` —
/// `split_edge` carries the rows, so the face is still complete — and
/// the vertex that split made.
fn wall() -> (Body<f64>, FaceKey, VertexKey) {
    let mut body = Body::<f64>::new();
    let face = cyl_wall_sheet(
        &mut body,
        CylFrame::canonical(1.0),
        None,
        (0.2, 1.4),
        (0.0, 1.0),
        tol(),
    );
    // The bottom rim is the one circle edge whose interval is `[0.2,
    // 1.4]`, the ascending one, whose parameter IS the azimuth.
    let rim = body
        .edges()
        .map(|(e, _)| e)
        .find(|&e| {
            let c = body
                .get_curve_geom(body.get_edge(e).unwrap().curve)
                .and_then(topo::CurveGeom::certified)
                .unwrap();
            matches!(c.carrier(), geom::Curve3::Circle { .. }) && c.params() == (0.2, 1.4)
        })
        .unwrap();
    let split = body.split_edge(rim, UM, tol()).unwrap();
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    (body, face, split.vertex)
}

/// The cylinder's point at chart coordinates `(u, v)`.
fn at(u: f64, v: f64) -> Point3<f64> {
    CylFrame::canonical(1.0).at(u, v)
}

/// Every half-edge of every loop of `face`.
fn halves_of(body: &Body<f64>, face: FaceKey) -> Vec<HalfEdgeKey> {
    let f = body.get_face(face).unwrap();
    core::iter::once(f.outer)
        .chain(f.rings.iter().copied())
        .filter_map(|lk| match body.get_loop(lk).unwrap().boundary {
            topo::LoopBoundary::Cycle { first } => Some(body.loop_cycle(first).unwrap()),
            topo::LoopBoundary::Empty { .. } => None,
        })
        .flatten()
        .collect()
}

/// The half-edge of `face` leaving `v` — one per vertex on these
/// fixtures' loops where it is asked.
fn leaving(body: &Body<f64>, face: FaceKey, v: VertexKey) -> HalfEdgeKey {
    let hit: Vec<_> = halves_of(body, face)
        .into_iter()
        .filter(|&he| body.get_half_edge(he).unwrap().start == v)
        .collect();
    assert_eq!(hit.len(), 1, "one half-edge of the face leaves {v:?}");
    hit[0]
}

/// `(rows stored, half-edges with no row)` over every loop of `face`.
fn rows_of(body: &Body<f64>, face: FaceKey) -> (usize, usize) {
    let halves = halves_of(body, face);
    let stored = halves
        .iter()
        .filter(|&&he| body.pcurve(he).is_some())
        .count();
    (stored, halves.len() - stored)
}

/// Every stored row of the body with its interval, image and
/// certificate, sorted — what "the pass rewrites the same bits" is
/// measured against.
fn rows_deep(body: &Body<f64>) -> Vec<String> {
    let mut out: Vec<String> = body
        .pcurves()
        .map(|(he, c)| {
            format!(
                "{he:?} {:?} {:?} {:?}",
                c.params(),
                c.pcurve(),
                c.certificate()
            )
        })
        .collect();
    out.sort();
    out
}

/// A strut up the ruling from the split vertex `m` to `(UM, 0.5)`.
fn strut(body: &mut Body<f64>, face: FaceKey, m: VertexKey) -> topo::MevCreated {
    let he = leaving(body, face, m);
    body.mev_line(MevSite::Fan { he1: he, he2: he }, at(UM, 0.5), tol())
        .unwrap()
}

/// **The unit's row.** A `mev_line` strut at a `Fan` site on the minted
/// wall's own loop leaves the face complete, and the tier-3 pcurve pass
/// with nothing to say. At this unit's merge base the op returned `Ok`
/// and the pass read two `MissingCache` findings, one per half it
/// minted.
#[test]
fn a_mev_on_a_minted_wall_leaves_the_face_complete() {
    let (mut body, face, m) = wall();
    let made = strut(&mut body, face, m);
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    assert!(body.pcurve(made.he_plus).is_some());
    assert!(body.pcurve(made.he_minus).is_some());
    assert_eq!(rows_of(&body, face), (7, 0));
}

/// **The rows the op mints are the minting pass's rows.** Running
/// `mint_pcurves` over the body a strut left rewrites every row with the
/// same bits — image, interval and certificate — so a caller that still
/// re-mints after the op keeps working, and its re-mint is a no-op.
#[test]
fn the_minted_rows_are_the_mint_passs_rows_byte_for_byte() {
    let (mut body, face, m) = wall();
    strut(&mut body, face, m);
    let minted = rows_deep(&body);
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(rows_deep(&body), minted);
}

/// **`mekr` mints its halves' rows too.** A ring on the wall — a strut
/// cut free of the boundary with `kemr`, and the face re-minted so it is
/// complete — is joined back to the outer loop along the ruling. The
/// merged loop is re-anchored at the new `he_plus`, and every row of it
/// is the pass's. At this unit's merge base the pass read two
/// `MissingCache` findings after the `mekr`.
#[test]
fn a_mekr_on_a_minted_wall_leaves_the_face_complete() {
    let (mut body, face, m) = wall();
    let first = strut(&mut body, face, m);
    let p = first.vertex;
    let second = body
        .mev_line(
            MevSite::Fan {
                he1: first.he_minus,
                he2: first.he_minus,
            },
            at(UM, 0.8),
            tol(),
        )
        .unwrap();
    let ring = body.kemr(first.he_plus, first.he_minus).unwrap().ring;
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    assert_eq!(body.get_face(face).unwrap().rings, vec![ring]);

    let target = leaving(&body, face, m);
    let point = |v: VertexKey| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let made = body
        .mekr(
            MekrSite::Cycles {
                target,
                ring: second.he_plus,
            },
            geom_brep::EdgeCurveSpec::line_between(point(m), point(p)),
            tol(),
        )
        .unwrap();
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    assert!(body.pcurve(made.he_plus).is_some());
    assert!(body.pcurve(made.he_minus).is_some());
    assert_eq!(rows_of(&body, face), (9, 0));
    let minted = rows_deep(&body);
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(rows_deep(&body), minted);
}

/// **Absence is never a claim.** A wall that never ran the minting pass
/// stores no rows, and the op mints none: an unminted face is the
/// minting pass's, and a row minted onto it would half-mint it the
/// other way.
#[test]
fn an_unminted_face_stays_rowless() {
    let (mut body, face, m) = wall();
    let keys: Vec<_> = body.pcurves().map(|(he, _)| he).collect();
    for he in keys {
        body.detach_pcurve(he);
    }
    strut(&mut body, face, m);
    assert_eq!(body.pcurves().count(), 0);
    assert_eq!(validate_pcurves(&body, band()), vec![]);
}

/// **A half-minted face is left as found.** It is already the defect
/// the pass reports; the op cannot pin its new rows against a
/// neighbour with none, so it mints nothing on it and moves no row it
/// holds — the pass then names the new halves beside the one that was
/// already missing.
#[test]
fn a_half_minted_face_is_left_as_found() {
    let (mut body, face, m) = wall();
    let dropped = halves_of(&body, face)[0];
    body.detach_pcurve(dropped);
    let before = rows_deep(&body);
    let made = strut(&mut body, face, m);
    assert_eq!(rows_deep(&body), before);
    let mut missing: Vec<HalfEdgeKey> = validate_pcurves(&body, band())
        .into_iter()
        .map(|f| match f {
            PcurveMintError::MissingCache { half_edge } => half_edge,
            other => panic!("only missing rows are reported, got {other:?}"),
        })
        .collect();
    missing.sort();
    let mut want = vec![dropped, made.he_plus, made.he_minus];
    want.sort();
    assert_eq!(missing, want);
}

/// **A carrier off the chart leaves the face unminted, never
/// half-minted.** A strut whose straight line leaves the cylinder has
/// no chart image that certifies on the wall, so the wall as the op
/// leaves it has no closed-form row set: it stores nothing, and the
/// pass, re-run over the result, refuses it loudly.
#[test]
fn a_mev_whose_carrier_leaves_the_chart_leaves_the_face_unminted() {
    let (mut body, face, m) = wall();
    let he = leaving(&body, face, m);
    body.mev_line(MevSite::Fan { he1: he, he2: he }, at(1.2, 0.5), tol())
        .unwrap();
    assert_eq!(rows_of(&body, face), (0, 7));
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    assert!(topo::mint_pcurves(&mut body, tol()).is_err());
}
