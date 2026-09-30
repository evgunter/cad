//! **An Euler operator that adds a half-edge to a minted curved face
//! mints that half-edge's pcurve row at the mint site.** The forcing
//! fixture is `split_edge_pcurve_rows`' minted cylinder-wall sheet: a
//! curved chart, so the face stores rows, and a ruling to grow struts
//! along — a line ON the cylinder, whose chart image has a closed form.
//!
//! The claim under every row is one sentence: `mev`, `mef` and `mekr`
//! leave no complete face half-minted. A face whose rows were complete
//! is complete after the op, with the rows the minting pass would
//! derive, or — where the closed-form lane cannot mint it as the
//! surgery leaves it — stores nothing; a face that stored no row still
//! stores none; a face that was already half-minted is left as found,
//! unless its only gaps are on loops a null edge holds open or the op
//! takes its last null edge off; and on a spline chart the op refuses
//! before it mutates. Doors that are not
//! these three can still leave a face half-minted, and the `kef` row
//! below is one. `mev_null`, whose edge has no carrier, holds its loop
//! open, and the null-edge rows at the end pin one door that releases
//! it: the edge's first description, which mints every loop no other
//! null edge holds open, and the whole face once none is left
//! (`topo::null`'s rows pin the other, an operator that cuts the edge
//! off the loop).

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

/// **Absence is never a claim.** A wall whose rows are detached stores
/// none, and the op mints none onto it: an unminted face is the minting
/// pass's, and a row minted onto it would half-mint it the other way.
/// The seed face on the same cylinder keeps its rows, so the op's plan
/// runs; it reads the face it touches, not the body, and the seed
/// face's rows are exactly as they were.
#[test]
fn an_unminted_face_beside_a_minted_one_stays_rowless() {
    let (mut body, face, m) = wall();
    for he in halves_of(&body, face) {
        body.detach_pcurve(he);
    }
    let elsewhere = rows_deep(&body);
    assert!(!elsewhere.is_empty(), "the seed face keeps its rows");
    strut(&mut body, face, m);
    assert_eq!(rows_of(&body, face), (0, 7));
    assert_eq!(rows_deep(&body), elsewhere);
}

/// **A face with no half-edge at all is not a complete one.** A second
/// shell's seed face, put on the cylinder, is a lone vertex: every one
/// of its (zero) half-edges carries a row, and it stores none. A `mev`
/// growing a segment on it — a ruling, whose image the lane derives —
/// leaves it rowless: it was never minted, so it is the minting pass's.
#[test]
fn a_lone_vertex_face_on_a_minting_chart_stays_unminted() {
    let (mut body, _, _) = wall();
    let seed = body.mvfs(at(0.6, 0.2), true).unwrap();
    body.set_face_surface(
        seed.face,
        topo::FaceSurface::New {
            surface: CylFrame::canonical(1.0).surface(),
            sense: true,
        },
    )
    .unwrap();
    let made = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            at(0.6, 0.7),
            tol(),
        )
        .unwrap();
    assert!(body.pcurve(made.he_plus).is_none());
    assert_eq!(rows_of(&body, seed.face), (0, 2));
}

/// **A strut spliced before the loop's `first` is minted too.** Halves
/// inserted before `first` are the last ones the walk from `first`
/// reaches; the face is complete afterwards, with the pass's rows.
#[test]
fn a_strut_at_the_loops_first_half_edge_is_minted() {
    let (mut body, face, _) = wall();
    let outer = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the wall is bounded by a cycle")
    };
    let v = body.get_half_edge(first).unwrap().start;
    let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    // Along the ruling through `first`'s start, into the sheet.
    let rise = if p.z > 0.5 { -0.3 } else { 0.3 };
    let made = body
        .mev_line(
            MevSite::Fan {
                he1: first,
                he2: first,
            },
            p + geom_core::Vec3::new(0.0, 0.0, rise),
            tol(),
        )
        .unwrap();
    assert!(body.pcurve(made.he_plus).is_some());
    assert!(body.pcurve(made.he_minus).is_some());
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    let minted = rows_deep(&body);
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(rows_deep(&body), minted);
}

/// **A carrier whose ends meet the chart and whose middle leaves it.**
/// An arc from the split vertex up the ruling, bowing radially outward
/// in the plane of the axis: its chart image is the vertical segment
/// between its ends, so the branch walk accepts it, and the
/// certification — the image mapped back through the cylinder against
/// the arc — refuses it. The face is left storing nothing, as for a
/// carrier the walk refuses, and never half-minted.
#[test]
fn a_strut_that_bows_off_the_chart_between_its_ends_leaves_the_face_unminted() {
    let (mut body, face, m) = wall();
    let he = leaving(&body, face, m);
    let pm = *body.get_point(body.get_vertex(m).unwrap().point).unwrap();
    let outward = geom_core::Vec3::new(UM.cos(), UM.sin(), 0.0);
    let up = geom_core::Vec3::unit_z();
    let (half, inset) = (0.25, 0.2);
    let centre = pm + up * half - outward * inset;
    let (from_c, to_c) = (pm - centre, (pm + up * (2.0 * half)) - centre);
    let radius = from_c.norm();
    let axis = from_c.cross(to_c).normalize();
    let theta = 2.0 * (half / inset).atan();
    let carrier = geom::Curve3::Circle {
        center: centre,
        axis,
        radius,
        u_ref: from_c / radius,
    };
    let end = carrier.eval(theta);
    let spec = geom_brep::EdgeCurveSpec::arc_of_circle(carrier, 0.0, theta).unwrap();
    body.mev(MevSite::Fan { he1: he, he2: he }, end, spec, tol())
        .unwrap();
    assert_eq!(rows_of(&body, face), (0, 7));
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    assert!(topo::mint_pcurves(&mut body, tol()).is_err());
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

/// A lone vertex inside the minted wall: a strut up the ruling cut free
/// of the boundary with `kemr` leaves its far end `p` as an EMPTY ring.
/// The face is re-minted so the map holds no row of a killed half, and
/// is complete: a ring with no half-edge holds no row to miss.
fn lone_ring(body: &mut Body<f64>, face: FaceKey, m: VertexKey) -> (topo::LoopKey, VertexKey) {
    let s = strut(body, face, m);
    let ring = body.kemr(s.he_plus, s.he_minus).unwrap().ring;
    assert!(matches!(
        body.get_loop(ring).unwrap().boundary,
        topo::LoopBoundary::Empty { .. }
    ));
    topo::mint_pcurves(body, tol()).unwrap();
    assert_eq!(validate_pcurves(body, band()), vec![]);
    assert_eq!(rows_of(body, face), (5, 0));
    (ring, s.vertex)
}

/// **`mev` at a `Lone` site mints its halves' rows.** A strut grown from
/// the wall's lone-vertex ring up the ruling turns the ring into the
/// two-half cycle `p → q → p`; the face stays complete, and its rows are
/// the pass's.
#[test]
fn a_mev_at_a_lone_vertex_ring_of_a_minted_wall_leaves_the_face_complete() {
    let (mut body, face, m) = wall();
    let (ring, _) = lone_ring(&mut body, face, m);
    let made = body
        .mev_line(MevSite::Lone { r#loop: ring }, at(UM, 0.8), tol())
        .unwrap();
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    assert!(body.pcurve(made.he_plus).is_some());
    assert!(body.pcurve(made.he_minus).is_some());
    assert_eq!(rows_of(&body, face), (7, 0));
    let minted = rows_deep(&body);
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(rows_deep(&body), minted);
}

/// **`mef` at a `Lone` site mints both pieces' rows.** The rim circle
/// through the wall's lone vertex closes on itself there, and `mef`
/// splits the ring along it: the wall keeps the plus half as a one-half
/// ring, the new face — on the wall's own chart — gets the minus half as
/// its outer loop. Each one-half loop closes by one whole period of the
/// chart, and both faces are complete with the pass's rows. (The circle
/// runs off the sheet's `u` span; the rows read only the chart, so the
/// fixture is a pcurve fixture and asks nothing else of the body.)
#[test]
fn a_mef_closing_a_rim_circle_at_a_lone_vertex_mints_both_pieces() {
    let (mut body, face, m) = wall();
    let (ring, _) = lone_ring(&mut body, face, m);
    let frame = CylFrame::canonical(1.0);
    let carrier = geom::Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.5),
        axis: frame.axis,
        radius: 1.0,
        u_ref: frame.radial(UM),
    };
    let spec =
        geom_brep::EdgeCurveSpec::arc_of_circle(carrier, 0.0, core::f64::consts::TAU).unwrap();
    let made = body
        .mef(
            topo::MefSite::Lone { r#loop: ring },
            spec,
            topo::FaceSurface::Inherit,
            tol(),
        )
        .unwrap();
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    assert!(body.pcurve(made.he_plus).is_some());
    assert!(body.pcurve(made.he_minus).is_some());
    assert_eq!(rows_of(&body, face), (6, 0));
    assert_eq!(rows_of(&body, made.face), (1, 0));
    let minted = rows_deep(&body);
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(rows_deep(&body), minted);
}

/// A ring on the minted wall with two half-edges: a strut up the ruling
/// with a second strut from its tip, the first cut away with `kemr`. The
/// face is re-minted, so it is complete with a two-half ring.
fn two_half_ring(body: &mut Body<f64>, face: FaceKey, m: VertexKey) -> topo::LoopKey {
    let first = strut(body, face, m);
    body.mev_line(
        MevSite::Fan {
            he1: first.he_minus,
            he2: first.he_minus,
        },
        at(UM, 0.8),
        tol(),
    )
    .unwrap();
    let ring = body.kemr(first.he_plus, first.he_minus).unwrap().ring;
    topo::mint_pcurves(body, tol()).unwrap();
    assert_eq!(validate_pcurves(body, band()), vec![]);
    assert_eq!(rows_of(body, face), (7, 0));
    ring
}

/// The first half-edge of `face`'s outer loop.
fn outer_first(body: &Body<f64>, face: FaceKey) -> HalfEdgeKey {
    let outer = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the wall is bounded by a cycle")
    };
    first
}

/// **A loop the op does not touch keeps its rows.** A strut on the outer
/// loop of a wall carrying a two-half ring re-mints the outer loop and
/// leaves the ring's rows as they were, and the face's rows are still
/// the pass's, byte for byte.
#[test]
fn a_strut_beside_a_ring_keeps_the_rings_rows() {
    let (mut body, face, m) = wall();
    let ring = two_half_ring(&mut body, face, m);
    let topo::LoopBoundary::Cycle { first } = body.get_loop(ring).unwrap().boundary else {
        panic!("the ring is a cycle")
    };
    let ring_rows = |body: &Body<f64>| -> Vec<String> {
        body.loop_cycle(first)
            .unwrap()
            .into_iter()
            .map(|he| {
                let row = body.pcurve(he).unwrap();
                format!(
                    "{:?} {:?} {:?}",
                    row.params(),
                    row.pcurve(),
                    row.certificate()
                )
            })
            .collect()
    };
    let before = ring_rows(&body);
    let o = outer_first(&body, face);
    let v = body.get_half_edge(o).unwrap().start;
    let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let rise = if p.z > 0.5 { -0.3 } else { 0.3 };
    body.mev_line(
        MevSite::Fan { he1: o, he2: o },
        p + geom_core::Vec3::new(0.0, 0.0, rise),
        tol(),
    )
    .unwrap();
    assert_eq!(ring_rows(&body), before);
    assert_eq!(rows_of(&body, face), (9, 0));
    let minted = rows_deep(&body);
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(rows_deep(&body), minted);
}

/// **A face the op clears is cleared whole, rings included.** A secant
/// strut on the outer loop of a wall carrying a two-half ring leaves the
/// wall with no closed-form row set: every row goes, the ring's with the
/// outer loop's, and the face is unminted rather than half-minted.
#[test]
fn a_secant_strut_beside_a_ring_clears_the_ring_too() {
    let (mut body, face, m) = wall();
    two_half_ring(&mut body, face, m);
    let first = outer_first(&body, face);
    body.mev_line(
        MevSite::Fan {
            he1: first,
            he2: first,
        },
        at(1.2, 0.5),
        tol(),
    )
    .unwrap();
    assert_eq!(rows_of(&body, face), (0, 9));
    assert_eq!(validate_pcurves(&body, band()), vec![]);
}

/// **`mekr` across the chart's principal azimuth cut.** The wall sheet
/// spans azimuth `[4.2, 5.2]`, across the cut at `3π/2` where the
/// closed-form image's principal branch wraps. A ring whose halves
/// straddle it — a strut up the ruling at `4.6`, an arc along the
/// parallel to `4.8`, the strut cut away with `kemr` — is joined to the
/// outer loop at `4.8`. The merged loop, walked from its new `first`,
/// pins the ring's halves on the branch the outer loop reaches them on,
/// and every row is the pass's, byte for byte.
#[test]
fn a_mekr_across_the_principal_azimuth_cut_mints_the_passs_rows() {
    let (ua, ub) = (4.6_f64, 4.8_f64);
    let frame = CylFrame::canonical(1.0);
    let mut body = Body::<f64>::new();
    let face = cyl_wall_sheet(&mut body, frame, None, (4.2, 5.2), (0.0, 1.0), tol());
    let rim = body
        .edges()
        .map(|(e, _)| e)
        .find(|&e| {
            let c = body
                .get_curve_geom(body.get_edge(e).unwrap().curve)
                .and_then(topo::CurveGeom::certified)
                .unwrap();
            matches!(c.carrier(), geom::Curve3::Circle { .. }) && c.params() == (4.2, 5.2)
        })
        .unwrap();
    let ma = body.split_edge(rim, ua, tol()).unwrap();
    let mb = body.split_edge(ma.new_edge, ub, tol()).unwrap();
    let he = leaving(&body, face, ma.vertex);
    let s = body
        .mev_line(MevSite::Fan { he1: he, he2: he }, frame.at(ua, 0.5), tol())
        .unwrap();
    let circle = geom::Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.5),
        axis: frame.axis,
        radius: 1.0,
        u_ref: frame.radial(ua),
    };
    let q = circle.eval(ub - ua);
    let arc = body
        .mev(
            MevSite::Fan {
                he1: s.he_minus,
                he2: s.he_minus,
            },
            q,
            geom_brep::EdgeCurveSpec::arc_of_circle(circle, 0.0, ub - ua).unwrap(),
            tol(),
        )
        .unwrap();
    body.kemr(s.he_plus, s.he_minus).unwrap();
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(validate_pcurves(&body, band()), vec![]);

    let target = leaving(&body, face, mb.vertex);
    let from = *body
        .get_point(body.get_vertex(mb.vertex).unwrap().point)
        .unwrap();
    let made = body
        .mekr(
            MekrSite::Cycles {
                target,
                ring: arc.he_minus,
            },
            geom_brep::EdgeCurveSpec::line_between(from, q),
            tol(),
        )
        .unwrap();
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    assert!(body.pcurve(made.he_plus).is_some());
    assert_eq!(rows_of(&body, face), (10, 0));
    let minted = rows_deep(&body);
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(rows_deep(&body), minted);
}

/// **A secant strut, then killed, leaves the wall unminted until the
/// pass runs.** The strut clears the complete wall — it has no
/// closed-form row set with the strut in it — and killing the strut,
/// by `kemr` or by `kev`, does not bring the rows back: no operator
/// re-mints a face that stores nothing. While the strut stands, the
/// pcurve pass says nothing about the rowless wall, and the at-rest
/// validation names the strut by its tip. The producer's closing mint
/// is what restores the rows once the strut is gone.
#[test]
fn a_secant_strut_then_killed_leaves_the_wall_unminted_until_the_pass() {
    for kill in ["kemr", "kev"] {
        let (mut body, face, m) = wall();
        let he = leaving(&body, face, m);
        let s = body
            .mev_line(MevSite::Fan { he1: he, he2: he }, at(1.2, 0.5), tol())
            .unwrap();
        assert_eq!(rows_of(&body, face), (0, 7), "{kill}");
        assert_eq!(
            topo::validate_geometric(&body, tol()),
            Err(vec![topo::ValidationError::ScaffoldingStrutVertex {
                vertex: s.vertex
            }]),
            "{kill}: the at-rest validation names the standing strut"
        );
        if kill == "kemr" {
            body.kemr(s.he_plus, s.he_minus).unwrap();
        } else {
            // The strut kill, from its base: the tip dies with no fan.
            body.kev(s.he_plus).unwrap();
        }
        assert_eq!(rows_of(&body, face), (0, 5), "{kill}");
        assert_eq!(validate_pcurves(&body, band()), vec![], "{kill}");
        topo::mint_pcurves(&mut body, tol()).unwrap();
        assert_eq!(rows_of(&body, face), (5, 0), "{kill}");
    }
}

/// **`kef` merging a complete face into an unminted one on its chart
/// leaves the survivor half-minted.** `kef` is not a minting operator:
/// it moves the dying face's rows onto the survivor, whose own
/// half-edges had none. The wall split along a ruling by `mef` is two
/// complete faces; the new one's rows are detached, and `kef` on the old
/// face's side of the chord kills the old face into it. The pass names
/// every half-edge of the survivor that arrived without a row — the
/// state `PcurveMintError::MissingCache` lists `kef` among the doors of.
#[test]
fn kef_merging_a_complete_face_into_an_unminted_one_leaves_it_half_minted() {
    let (mut body, face, m) = wall();
    // The top rim, split where the ruling through `m` meets it.
    let want_at = at(UM, 1.0);
    let (rim, t) = body
        .edges()
        .map(|(e, _)| e)
        .find_map(|e| {
            let c = body
                .get_curve_geom(body.get_edge(e).unwrap().curve)
                .and_then(topo::CurveGeom::certified)
                .unwrap();
            let (t0, t1) = c.params();
            let geom::Curve3::Circle {
                center,
                axis,
                u_ref,
                ..
            } = *c.carrier()
            else {
                return None;
            };
            let d = want_at - center;
            let angle = d.dot(axis.cross(u_ref)).atan2(d.dot(u_ref));
            let tau = core::f64::consts::TAU;
            let (lo, hi) = (t0.min(t1), t0.max(t1));
            let t = angle + tau * ((lo - angle) / tau).ceil();
            (center.z == 1.0 && t < hi).then_some((e, t))
        })
        .unwrap();
    let top = body.split_edge(rim, t, tol()).unwrap().vertex;
    let (he1, he2) = (leaving(&body, face, m), leaving(&body, face, top));
    let point = |v: VertexKey| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let chord = geom_brep::EdgeCurveSpec::line_between(point(m), point(top));
    let made = body
        .mef(
            topo::MefSite::Chords { he1, he2 },
            chord,
            topo::FaceSurface::Inherit,
            tol(),
        )
        .unwrap();
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    assert_eq!(
        rows_of(&body, made.face),
        (4, 0),
        "the ruling mef mints both pieces"
    );
    let unminted: Vec<HalfEdgeKey> = halves_of(&body, made.face);
    for &he in &unminted {
        body.detach_pcurve(he).unwrap();
    }
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    body.kef(made.he_plus, tol()).unwrap();
    let mut want: Vec<HalfEdgeKey> = unminted
        .into_iter()
        .filter(|&he| he != made.he_minus)
        .collect();
    want.sort();
    let mut missing: Vec<HalfEdgeKey> = validate_pcurves(&body, band())
        .into_iter()
        .map(|f| match f {
            PcurveMintError::MissingCache { half_edge } => half_edge,
            other => panic!("only missing rows are reported, got {other:?}"),
        })
        .collect();
    missing.sort();
    assert!(!want.is_empty());
    assert_eq!(missing, want);
}

/// The findings `validate_pcurves` reports on `body`, each a missing
/// row, as their half-edges sorted.
fn missing_rows(body: &Body<f64>) -> Vec<HalfEdgeKey> {
    let mut missing: Vec<HalfEdgeKey> = validate_pcurves(body, band())
        .into_iter()
        .map(|f| match f {
            PcurveMintError::MissingCache { half_edge } => half_edge,
            other => panic!("only missing rows are reported, got {other:?}"),
        })
        .collect();
    missing.sort();
    missing
}

/// A strut up the ruling from `m`, then a null strut at its tip: the
/// wall missing exactly the null edge's two rows, the null edge, and
/// the tip's point.
fn null_strut_at_tip(
    body: &mut Body<f64>,
    face: FaceKey,
    m: VertexKey,
) -> (topo::MevCreated, Point3<f64>) {
    let tip = strut(body, face, m).vertex;
    let he = leaving(body, face, tip);
    let null = body
        .mev_null(
            MevSite::Fan { he1: he, he2: he },
            topo::NewVertexSide::Above,
        )
        .unwrap();
    let mut want = vec![null.he_plus, null.he_minus];
    want.sort();
    assert_eq!(
        missing_rows(body),
        want,
        "mev_null leaves the wall missing its two rows"
    );
    let p = *body.get_point(body.get_vertex(tip).unwrap().point).unwrap();
    (null, p)
}

/// The cylinder's own circle at the tip's height, once round from the
/// tip: a closed carrier ON the chart, whose image is the `u` line
/// `v = 0.5`, so it describes an edge whose two ends are one point.
fn circle_through_tip() -> geom_brep::EdgeCurveSpec<f64> {
    let frame = CylFrame::canonical(1.0);
    let carrier = geom::Curve3::Circle {
        center: frame.origin + frame.axis * 0.5,
        axis: frame.axis,
        radius: frame.radius,
        u_ref: frame.u_ref,
    };
    geom_brep::EdgeCurveSpec::arc_of_circle(carrier, UM, UM + core::f64::consts::TAU).unwrap()
}

/// **A null edge's first description completes the face `mev_null`
/// left it on.** The null strut leaves the minted wall missing its two
/// rows; `set_edge_curve` gives it a carrier on the chart, and the wall
/// leaves complete, with the minting pass's rows. At this unit's merge
/// base the two `MissingCache` findings survived the description.
#[test]
fn a_null_edges_first_description_completes_the_wall() {
    let (mut body, face, m) = wall();
    let (null, _) = null_strut_at_tip(&mut body, face, m);
    body.set_edge_curve(null.edge, circle_through_tip(), tol())
        .unwrap();
    assert_eq!(
        missing_rows(&body),
        vec![],
        "the description mints both rows"
    );
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    assert_eq!(rows_of(&body, face), (9, 0));
    let minted = rows_deep(&body);
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(rows_deep(&body), minted, "the rows are the pass's");
}

/// **A carrier off the chart leaves the wall rowless, never
/// half-minted.** Described by the scaffolding circle through the tip,
/// which leaves the cylinder, the wall has no closed-form row set, and
/// the description gives it the answer an Euler operator gives it: it
/// stores nothing.
#[test]
fn a_null_edge_described_off_the_chart_leaves_the_wall_rowless() {
    let (mut body, face, m) = wall();
    let (null, p) = null_strut_at_tip(&mut body, face, m);
    body.set_edge_curve(
        null.edge,
        geom_brep::EdgeCurveSpec::self_loop_circle_at(p),
        tol(),
    )
    .unwrap();
    assert_eq!(rows_of(&body, face), (0, 9));
    assert_eq!(validate_pcurves(&body, band()), vec![]);
}

/// **Only a null edge's description re-mints.** Re-describing the edge
/// once it is certified moves no key and leaves every row where it is,
/// as for any certified edge: a wall missing the edge's own row stays
/// missing it, though the face is one a null edge's description would
/// re-mint.
#[test]
fn a_second_description_leaves_the_rows_as_found() {
    let (mut body, face, m) = wall();
    let (null, _) = null_strut_at_tip(&mut body, face, m);
    body.set_edge_curve(null.edge, circle_through_tip(), tol())
        .unwrap();
    body.detach_pcurve(null.he_plus).unwrap();
    let before = rows_deep(&body);
    body.set_edge_curve(null.edge, circle_through_tip(), tol())
        .unwrap();
    assert_eq!(rows_deep(&body), before);
    assert_eq!(missing_rows(&body), vec![null.he_plus]);
}

/// The cylinder's own circle at height `v`, once round from the ruling
/// `UM`: a closed carrier on the chart, for a null edge at `(UM, v)`.
fn circle_at(v: f64) -> geom_brep::EdgeCurveSpec<f64> {
    let frame = CylFrame::canonical(1.0);
    let carrier = geom::Curve3::Circle {
        center: frame.origin + frame.axis * v,
        axis: frame.axis,
        radius: frame.radius,
        u_ref: frame.u_ref,
    };
    geom_brep::EdgeCurveSpec::arc_of_circle(carrier, UM, UM + core::f64::consts::TAU).unwrap()
}

/// A null strut at the vertex `he` leaves.
fn null_at(body: &mut Body<f64>, he: HalfEdgeKey) -> topo::MevCreated {
    body.mev_null(
        MevSite::Fan { he1: he, he2: he },
        topo::NewVertexSide::Above,
    )
    .unwrap()
}

/// **Two null edges on one wall: the second description re-mints it.**
/// Two struts up the ruling, to `(UM, 0.5)` and on to `(UM, 0.8)`, and a
/// null strut at each tip. Describing the first leaves the wall as
/// found — the other null edge's halves have no carrier yet — and
/// describing the second re-mints it whole, the first's halves
/// included, with the minting pass's rows. At this unit's review head
/// the wall kept four `MissingCache` findings. (Adopted from the
/// review's probe P1.)
#[test]
fn two_null_edges_on_one_wall_complete_it_at_the_second_description() {
    let (mut body, face, m) = wall();
    let first = strut(&mut body, face, m);
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
    assert_eq!(missing_rows(&body), vec![]);
    let upper = null_at(&mut body, second.he_minus);
    let lower = null_at(&mut body, first.he_minus);
    let mut four = vec![upper.he_plus, upper.he_minus, lower.he_plus, lower.he_minus];
    four.sort();
    assert_eq!(
        missing_rows(&body),
        four,
        "two null edges, four missing rows"
    );

    body.set_edge_curve(upper.edge, circle_at(0.8), tol())
        .unwrap();
    assert_eq!(
        missing_rows(&body),
        four,
        "a null edge left on the wall defers the re-mint"
    );
    body.set_edge_curve(lower.edge, circle_at(0.5), tol())
        .unwrap();
    assert_eq!(
        missing_rows(&body),
        vec![],
        "the last description completes it"
    );
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    let minted = rows_deep(&body);
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(rows_deep(&body), minted, "the rows are the pass's");
}

/// **An operator on the half-minted wall, then the description.** A
/// `mev_line` strut from a corner of the sheet while the null strut
/// waits: the operator leaves the half-minted wall as found, so its own
/// halves arrive rowless, and the description re-mints the wall whole —
/// those halves included — with the minting pass's rows. At this unit's
/// review head the strut's two rows stayed missing. (Adopted from the
/// review's probe P2.)
#[test]
fn a_description_after_an_operator_on_the_half_minted_wall_completes_it() {
    let (mut body, face, m) = wall();
    let (null, _) = null_strut_at_tip(&mut body, face, m);
    let corner = outer_first(&body, face);
    let v = body.get_half_edge(corner).unwrap().start;
    let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let rise = if p.z > 0.5 { -0.3 } else { 0.3 };
    let made = body
        .mev_line(
            MevSite::Fan {
                he1: corner,
                he2: corner,
            },
            p + geom_core::Vec3::new(0.0, 0.0, rise),
            tol(),
        )
        .unwrap();
    let mut four = vec![null.he_plus, null.he_minus, made.he_plus, made.he_minus];
    four.sort();
    assert_eq!(
        missing_rows(&body),
        four,
        "the operator leaves the half-minted wall as found"
    );

    body.set_edge_curve(null.edge, circle_through_tip(), tol())
        .unwrap();
    assert_eq!(missing_rows(&body), vec![], "the description completes it");
    assert_eq!(validate_pcurves(&body, band()), vec![]);
    let minted = rows_deep(&body);
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(rows_deep(&body), minted, "the rows are the pass's");
}

/// **The description that leaves no null edge on a face re-mints it
/// whatever else it misses.** A wall carrying a two-half ring, one
/// ring half's row detached — a gap no null edge holds — and a null
/// strut at a corner of the outer loop. Describing the strut leaves no
/// null edge on the wall, so the description re-walks and mints every
/// loop: the wall leaves complete, the ring's gap filled, with the
/// minting pass's rows. At this unit's first review head the wall kept
/// three missing rows, the described edge's own two among them.
/// (Adopted from the review's probe C2.)
#[test]
fn a_description_that_leaves_no_null_edge_completes_a_wall_with_another_gap() {
    let (mut body, face, m) = wall();
    let ring = two_half_ring(&mut body, face, m);
    let topo::LoopBoundary::Cycle { first: on_ring } = body.get_loop(ring).unwrap().boundary else {
        panic!("the ring is a cycle")
    };
    assert!(body.detach_pcurve(on_ring).is_some());
    let corner = outer_first(&body, face);
    let v = body.get_half_edge(corner).unwrap().start;
    let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let null = null_at(&mut body, corner);
    let mut three = vec![on_ring, null.he_plus, null.he_minus];
    three.sort();
    assert_eq!(
        missing_rows(&body),
        three,
        "the ring's detached row and the null strut's two"
    );

    let frame = CylFrame::canonical(1.0);
    let carrier = geom::Curve3::Circle {
        center: frame.origin + frame.axis * p.z,
        axis: frame.axis,
        radius: frame.radius,
        u_ref: frame.u_ref,
    };
    let azimuth = p.y.atan2(p.x);
    let round =
        geom_brep::EdgeCurveSpec::arc_of_circle(carrier, azimuth, azimuth + core::f64::consts::TAU)
            .unwrap();
    body.set_edge_curve(null.edge, round, tol()).unwrap();
    assert_eq!(
        missing_rows(&body),
        vec![],
        "the description completes the wall, the ring's gap included"
    );
    assert_eq!(rows_of(&body, face), (9, 0));
    let minted = rows_deep(&body);
    topo::mint_pcurves(&mut body, tol()).unwrap();
    assert_eq!(rows_deep(&body), minted, "the rows are the pass's");
}

/// **A never-minted wall stays rowless.** A wall storing no row is the
/// minting pass's: a null edge described on it mints nothing onto it,
/// and the seed face beside it keeps its rows.
#[test]
fn a_null_edge_described_on_an_unminted_wall_leaves_it_rowless() {
    let (mut body, face, m) = wall();
    for he in halves_of(&body, face) {
        body.detach_pcurve(he);
    }
    let elsewhere = rows_deep(&body);
    assert!(!elsewhere.is_empty(), "the seed face keeps its rows");
    let tip = strut(&mut body, face, m).vertex;
    let he = leaving(&body, face, tip);
    let null = null_at(&mut body, he);
    body.set_edge_curve(null.edge, circle_through_tip(), tol())
        .unwrap();
    assert_eq!(rows_of(&body, face), (0, 9));
    assert_eq!(rows_deep(&body), elsewhere);
}
