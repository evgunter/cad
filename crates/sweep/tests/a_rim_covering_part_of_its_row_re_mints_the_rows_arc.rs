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
use topo::construction::Construct;
use topo::construction::FaceSurface;
use topo::{Body, EdgeKey, FaceKey, HalfEdgeKey, Pcurve};

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

/// The body's volume through the at-rest door.
fn volume(body: &Body<f64>) -> Result<f64, topo::Stop> {
    let at = topo::AtRestBody::validate(body.clone(), tol()).expect("the body is at rest");
    let measured = topo::mass_properties(&at, tol(), topo::test_support::pinned_resolution(tol()))
        .unwrap_or_else(|e| panic!("the body measures: {e:?}"));
    match measured.stop {
        topo::Stop::Met => Ok(measured.enclosure.volume),
        stop => Err(stop),
    }
}

/// The half-edge of `edge` on `wall`.
fn on_wall(body: &Body<f64>, edge: EdgeKey, wall: FaceKey) -> HalfEdgeKey {
    let e = body.get_edge(edge).unwrap();
    [e.he_plus, e.he_minus]
        .into_iter()
        .find(|h| body.face_of_half_edge(*h) == Some(wall))
        .expect("the piece bounds the wall")
}

/// `got` is the row `want` is, over `want`'s arc ([`test_utils::arc_row`]).
fn assert_same_row(got: &Pcurve<f64>, want: &Pcurve<f64>, what: &str) {
    test_utils::arc_row::assert_same_row(&arc_row(got), &arc_row(want), what);
}

fn arc_row(image: &Pcurve<f64>) -> test_utils::arc_row::ArcRow {
    let Pcurve::IsoArc {
        p0,
        pd,
        t0,
        angle,
        breaks,
    } = image
    else {
        panic!("an IsoArc row: {image:?}")
    };
    test_utils::arc_row::ArcRow {
        p0: [p0.x, p0.y],
        pd: [pd.x, pd.y],
        breaks: format!("{breaks:?}"),
        t0: *t0,
        angle: *angle,
    }
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
/// its other one. The re-minted body measures the unsplit volume, or
/// stops short of the resolution as the unsplit body does. The mesher refuses the halves,
/// pinned as its frontier.
#[test]
fn both_halves_of_a_split_rim_re_mint_the_parents_row() {
    let mut body = bulged_prism();
    let (_, edge, wall, parent) = bottom_rim(&body);
    let whole = volume(&body);
    let pieces = split_at(&mut body, edge, &[0.5]);
    re_mint_pieces(&mut body, wall, &pieces, &parent, true);
    match (whole, volume(&body)) {
        (Ok(whole), Ok(split)) => assert!(
            (split - whole).abs() <= 1e-12 * whole,
            "the re-minted rows measure the unsplit volume: {split} / {whole}"
        ),
        // At eps 1e-12 the unsplit rational wall already stops short of
        // the resolution; the split body must stop the same way.
        (Err(whole), Err(split)) => assert_eq!(split, whole, "both stop short alike"),
        (whole, split) => panic!("split {split:?} against unsplit {whole:?}"),
    }
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
/// would restate them, refuses (`RechartFalsifies { Unimplemented }`,
/// `work/restate/the-describing-door-refuses-a-wall-re-charted-reversed-in-u.md`).
/// The mint pass certifies every row it stores.
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

/// The carrier angle, from the row's start, at which the CLOSED-FORM
/// image of a piece of turn `s` (its interval onto the whole row of turn
/// `big`, `m` sub-arcs) puts the point a quarter of the piece in. A piece
/// starting at `a = that − s/4` is one the closed form crosses exactly at
/// its quarter point: the coincidence a one-point judge cannot see.
fn closed_form_quarter_angle(big: f64, m: f64, s: f64) -> f64 {
    let hs = s / m;
    let off = s / 4.0;
    let k = (off / hs).floor();
    let phi = off - k * hs - hs / 2.0;
    let sigma = 0.5 + (phi / 2.0).tan() / (2.0 * (hs / 4.0).tan());
    let g = (k + sigma) / m;
    let ha = big / m;
    let kp = (g * m).floor();
    let sp = g * m - kp;
    kp * ha + ha / 2.0 + 2.0 * ((2.0 * sp - 1.0) * (ha / 4.0).tan()).atan()
}

/// **A piece the closed form crosses at its quarter point still takes
/// the row's arc.** For each piece turn there is one start at which the
/// closed-form image (the piece's interval onto the whole row) passes
/// through the carrier exactly a quarter in; the judge also reads the
/// piece's ends, which that image puts on the row's corners. The family
/// at three bulges (one, two and three sub-arcs) and three piece turns:
/// each re-mints the parent's row and tier 3 passes.
#[test]
fn a_piece_the_closed_form_crosses_at_its_quarter_point_takes_the_rows_arc() {
    for (bulge, m) in [(0.3, 1.0), (1.0, 2.0), (1.6, 3.0)] {
        for frac in [0.3, 0.5, 0.7] {
            let mut body = arc_prism(bulge);
            let (_, edge, wall, parent) = bottom_rim(&body);
            let (t0, t1) = params(&body, edge);
            let big = t1 - t0;
            let s = frac * big;
            let a = closed_form_quarter_angle(big, m, s) - s / 4.0;
            assert!(
                a > 1e-6 && a + s < big - 1e-6,
                "bulge {bulge} / {frac}: the coincident piece [{a}, {}] lies inside the rim",
                a + s
            );
            let pieces = split_at(&mut body, edge, &[a / big, s / (big - a)]);
            re_mint_pieces(&mut body, wall, &pieces, &parent, true);
        }
    }
}

/// **The line rim's twin.** A square prism's wall edge split so its
/// middle piece is `[1/8, 5/8]` of it: the closed-form row crosses the
/// carrier exactly a quarter in. The pass re-mints every wall row an
/// exact `IsoLine` and tier 3 passes.
#[test]
fn a_line_piece_the_closed_form_crosses_at_its_quarter_point_takes_its_row() {
    let mut body = arc_prism(0.0);
    let he = body
        .pcurves()
        .find_map(|(he, c)| match c.pcurve() {
            Pcurve::IsoLine { p0, pl } if p0.y == 0.0 && pl.y == 0.0 && pl.x != 0.0 => Some(he),
            _ => None,
        })
        .expect("a line rim on a wall row");
    let edge = body.get_half_edge(he).unwrap().edge;
    let wall = body.face_of_half_edge(he).unwrap();
    let pieces = split_at(&mut body, edge, &[0.125, 4.0 / 7.0]);
    assert_eq!(
        topo::mint_pcurves(&mut body, tol()),
        Ok(()),
        "the mint pass"
    );
    for piece in pieces {
        let row = body.pcurve(on_wall(&body, piece, wall)).unwrap();
        assert!(
            matches!(row.pcurve(), Pcurve::IsoLine { .. }),
            "{piece:?}: an exact row: {:?}",
            row.pcurve()
        );
    }
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()), "tier 3");
}
