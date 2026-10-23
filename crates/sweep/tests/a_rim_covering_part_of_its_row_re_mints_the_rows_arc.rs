//! **An arc rim covering part of its row re-mints the row over the row's
//! own arc.**
//!
//! A loft wall over a profile arc is a rational-quadratic spline chart,
//! and its rims are `Pcurve::IsoArc` rows: the boundary row over the
//! arc `[t0, t0 + angle]`. A split leaves children that cover part of
//! that row. `mint_pcurves` clears every row and derives again, so a
//! child's row is what the arc-rim arm measures for it: the closed form
//! (the edge's interval onto the whole row) misses it at the probe, and
//! the row over the arc read off the row and the circle holds. Each
//! child's re-minted row is the parent's, its arc to rounding, and tier
//! 3 re-certifies the body.
//!
//! The bulged fixture's arc is THREE sub-arcs (joints at 1/3 and 2/3 of
//! the row), so a split at 0.3 leaves a child holding a joint and one
//! starting short of it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::stacked_at;
use topo::{Body, EdgeKey, FaceKey, FaceSurface, HalfEdgeKey, Pcurve};

fn tol() -> Tol {
    Tol::witness()
}

/// A prism whose profile's second edge is an arc of bulge `bulge`.
fn arc_prism(bulge: f64) -> Body<f64> {
    let v = |x: f64, y: f64, b: f64| (Point2::new(x, y), b);
    let profile = || {
        vec![bulge_loop(vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, bulge),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ])]
    };
    sweep::loft_body::<f64>(&[profile(), profile()], &stacked_at(&[0.0, 1.0]), 1, tol())
        .expect("the prism builds")
        .body
}

/// The three-sub-arc fixture.
fn bulged_prism() -> Body<f64> {
    arc_prism(1.6)
}

/// The `v = 0` rim of the arc wall: its half-edge on the wall, its
/// edge, the wall, and the row stored for it.
fn bottom_rim(body: &Body<f64>) -> (HalfEdgeKey, EdgeKey, FaceKey, Pcurve<f64>) {
    let (he, row) = body
        .pcurves()
        .find_map(|(he, c)| match c.pcurve() {
            row @ Pcurve::IsoArc { p0, .. } if p0.y == 0.0 => Some((he, row.clone())),
            _ => None,
        })
        .expect("the arc wall's bottom rim is an IsoArc row");
    let h = body.get_half_edge(he).unwrap();
    (he, h.edge, body.face_of_half_edge(he).unwrap(), row)
}

fn params(body: &Body<f64>, edge: EdgeKey) -> (f64, f64) {
    body.get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .params()
}

/// The half-edge of `edge` on `wall`.
fn on_wall(body: &Body<f64>, edge: EdgeKey, wall: FaceKey) -> HalfEdgeKey {
    let e = body.get_edge(edge).unwrap();
    [e.he_plus, e.he_minus]
        .into_iter()
        .find(|h| body.face_of_half_edge(*h) == Some(wall))
        .expect("the piece bounds the wall")
}

/// `(p0, pd, breaks)` bit for bit and `(t0, angle)` to rounding: `got`
/// is the row `want` is, over `want`'s arc.
fn assert_same_row(got: &Pcurve<f64>, want: &Pcurve<f64>, what: &str) {
    let (
        Pcurve::IsoArc {
            p0,
            pd,
            t0,
            angle,
            breaks,
        },
        Pcurve::IsoArc {
            p0: w_p0,
            pd: w_pd,
            t0: w_t0,
            angle: w_angle,
            breaks: w_breaks,
        },
    ) = (got, want)
    else {
        panic!("{what}: two IsoArc rows: {got:?} / {want:?}")
    };
    assert_eq!(
        (p0.x, p0.y, pd.x, pd.y, format!("{breaks:?}")),
        (w_p0.x, w_p0.y, w_pd.x, w_pd.y, format!("{w_breaks:?}")),
        "{what}: the row, its direction and its sub-arcs"
    );
    assert!(
        (t0 - w_t0).abs() < 1e-12 && (angle - w_angle).abs() < 1e-12,
        "{what}: the arc ({t0}, {angle}) is the parent's ({w_t0}, {w_angle})"
    );
}

/// Splits `edge` at each fraction of the piece left last, in order, and
/// returns every piece: `[t0, t]` stays on `edge`, `[t, t1]` is the new
/// edge, which the next fraction splits.
fn split_at(body: &mut Body<f64>, edge: EdgeKey, fracs: &[f64]) -> Vec<EdgeKey> {
    let mut pieces = vec![edge];
    for frac in fracs {
        let last = *pieces.last().unwrap();
        let (t0, t1) = params(body, last);
        let made = body.split_edge(last, t0 + (t1 - t0) * frac, tol()).unwrap();
        pieces.push(made.new_edge);
    }
    pieces
}

/// The whole re-mint: every row cleared and derived again, then each
/// piece's wall row checked against the parent's, over the piece's own
/// interval, and tier 3 read where `tier3`.
fn re_mint_pieces(
    body: &mut Body<f64>,
    wall: FaceKey,
    pieces: &[EdgeKey],
    parent: &Pcurve<f64>,
    tier3: bool,
) {
    assert_eq!(topo::mint_pcurves(body, tol()), Ok(()), "the mint pass");
    let mut ends = Vec::new();
    for (i, &piece) in pieces.iter().enumerate() {
        let he = on_wall(body, piece, wall);
        let row = body.pcurve(he).expect("the piece's wall row is stored");
        assert_same_row(row.pcurve(), parent, &format!("piece {i}"));
        assert_eq!(
            row.params(),
            params(body, piece),
            "piece {i}: its own interval"
        );
        ends.push(row.params());
    }
    let mut sorted = ends.clone();
    sorted.sort_by(|a, b| a.0.total_cmp(&b.0));
    assert!(
        sorted.windows(2).all(|w| w[0].1 == w[1].0),
        "the pieces tile the parent's interval: {sorted:?}"
    );
    if tier3 {
        assert_eq!(topo::validate_geometric(body, tol()), Ok(()), "tier 3");
    }
}

/// **A whole rim is the closed form, unchanged.** The unsplit wall's
/// re-minted rows are the loft's, bit for bit: each rim's arc is its
/// edge's interval exactly. The body meshes watertight.
#[test]
fn a_whole_rim_re_mints_the_closed_form() {
    let mut body = bulged_prism();
    let stored: Vec<(HalfEdgeKey, String)> = body
        .pcurves()
        .map(|(he, c)| (he, format!("{:?}", c.pcurve())))
        .collect();
    let (he, edge, _, _) = bottom_rim(&body);
    assert_eq!(
        topo::mint_pcurves(&mut body, tol()),
        Ok(()),
        "the mint pass"
    );
    for (he, row) in &stored {
        assert_eq!(
            body.pcurve(*he)
                .map(|c| format!("{:?}", c.pcurve()))
                .as_ref(),
            Some(row),
            "{he:?}: the re-mint is the loft's row"
        );
    }
    let (t0, t1) = params(&body, edge);
    let Pcurve::IsoArc {
        t0: arc_t0, angle, ..
    } = body.pcurve(he).unwrap().pcurve()
    else {
        panic!("the rim's row is an IsoArc")
    };
    assert_eq!((*arc_t0, *angle), (t0, t1 - t0), "the arc is the interval");
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()), "tier 3");
    let mesh = mesh::tessellate(&body, 5e-2, tol()).expect("the prism tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

/// **Both halves of a split rim** re-mint the parent's row, over the
/// parent's arc: the first starting at its corner, the second ending at
/// its other one. The re-minted body measures the unsplit volume. The
/// mesher refuses the halves, pinned as its frontier.
#[test]
fn both_halves_of_a_split_rim_re_mint_the_parents_row() {
    let mut body = bulged_prism();
    let (_, edge, wall, parent) = bottom_rim(&body);
    let volume = |body: &Body<f64>| {
        let at = topo::AtRestBody::validate(body.clone(), tol()).expect("at rest");
        topo::mass_properties(&at, tol())
            .expect("the body measures")
            .volume
    };
    let whole = volume(&body);
    let pieces = split_at(&mut body, edge, &[0.5]);
    re_mint_pieces(&mut body, wall, &pieces, &parent, true);
    let split = volume(&body);
    assert!(
        (split - whole).abs() <= 1e-12 * whole,
        "the re-minted rows measure the volume the whole rim did: {split} / {whole}"
    );
    // The mesher's chord schedule lands on the sub-arc joints only for
    // a rim whose arc is its edge's interval, and refuses the children
    // (`work/tess/a-split-arc-rim-meshes-no-chord-point-on-its-sub-arc-joints.md`).
    match mesh::tessellate(&body, 5e-2, tol()) {
        Err(mesh::TessellateError::UnsupportedCurve { note, .. }) => assert!(
            note.starts_with("an arc-rim pcurve whose turn is not its edge's parameter interval"),
            "the split rim refuses at its chord schedule: {note}"
        ),
        other => panic!("the split rim's chord schedule: {:?}", other.map(|_| ())),
    }
}

/// **An off-centre split**: at 0.3 of the rim, the first child stops
/// short of the joint at 1/3 and the second holds both joints.
#[test]
fn an_off_centre_split_re_mints_the_parents_row() {
    let mut body = bulged_prism();
    let (_, edge, wall, parent) = bottom_rim(&body);
    let pieces = split_at(&mut body, edge, &[0.3]);
    re_mint_pieces(&mut body, wall, &pieces, &parent, true);
}

/// **A double split**: the middle piece touches neither corner of the
/// row, so neither end of its interval is an end of the arc.
#[test]
fn a_double_split_re_mints_every_piece() {
    let mut body = bulged_prism();
    let (_, edge, wall, parent) = bottom_rim(&body);
    let pieces = split_at(&mut body, edge, &[0.3, 0.6]);
    assert_eq!(pieces.len(), 3, "three pieces");
    re_mint_pieces(&mut body, wall, &pieces, &parent, true);
}

/// **A rim against the chart's `u`.** The wall re-charted in place on
/// the same surface reversed in `u` runs each rim from the `u = 1`
/// corner to the `u = 0` one. Split, the rim's pieces re-mint the
/// reversed row over the parent's arc, read off the row's LAST control
/// point. The fixture is a semicircle (two sub-arcs), whose knots
/// mirror bit for bit, so the wall reverses.
///
/// Tier 3 is not read: the in-place re-chart leaves the edges'
/// descriptions naming the old chart, and the describing door, which
/// would restate them, refuses a wall seam's restatement
/// (`Unimplemented`). The mint pass certifies every row it stores.
#[test]
fn a_rim_against_the_charts_u_re_mints_its_pieces() {
    let mut body = arc_prism(1.0);
    let (_, edge, wall, _) = bottom_rim(&body);
    let face = body.get_face(wall).unwrap();
    let Some(Surface::Nurbs(net)) = body.get_surface(face.surface).cloned() else {
        panic!("the arc wall is a spline chart")
    };
    // Lifts RechartUnvouched and RechartStrandsDescriptions: the wall
    // re-charted in place onto the same surface reversed in `u`, its
    // edges' descriptions left as they were, is the row's premise.
    body.set_face_surface_unvouched_for_tests(
        wall,
        FaceSurface::New {
            surface: Surface::Nurbs(std::sync::Arc::new(net.reversed_u().unwrap())),
            sense: !face.sense,
        },
    )
    .unwrap();
    assert_eq!(
        topo::mint_pcurves(&mut body, tol()),
        Ok(()),
        "the reversed wall mints"
    );
    let parent = body
        .pcurve(on_wall(&body, edge, wall))
        .unwrap()
        .pcurve()
        .clone();
    let Pcurve::IsoArc { p0, pd, .. } = &parent else {
        panic!("the reversed rim's row is an IsoArc: {parent:?}")
    };
    assert_eq!(
        (p0.x, pd.x),
        (1.0, -1.0),
        "the rim runs against the chart's u"
    );
    let pieces = split_at(&mut body, edge, &[0.3, 0.6]);
    re_mint_pieces(&mut body, wall, &pieces, &parent, false);
}
