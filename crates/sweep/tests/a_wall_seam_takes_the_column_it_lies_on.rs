//! **A wall–wall seam on a spline chart takes the column it lies on**,
//! whichever way the chart's `v` runs against its neighbour's.
//!
//! The fixture is a loft of a profile with one semicircular edge: four
//! spline walls whose vertical seams are each described as an `IsoLine`
//! image on the NEIGHBOURING wall's chart. Re-charting the arc wall onto
//! the same surface reversed in `v` (and, for the second row, in `u`
//! too) leaves every seam on it running against the neighbour's `v`
//! map: the column that carries the stated map verbatim does not hold
//! the carrier, and the one that runs the other way does.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::stacked_at;
use topo::{Body, FaceKey, FaceSurface, HalfEdgeKey, Pcurve};

fn tol() -> Tol {
    Tol::witness()
}

/// A prism whose profile's second edge is a semicircle (bulge 1): its
/// wall's knots mirror bitwise, so `reversed_u` and `reversed_v` both
/// accept it.
fn semicircle_prism() -> Body<f64> {
    let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
    let profile = || {
        vec![bulge_loop(vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 1.0),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ])]
    };
    sweep::loft_body::<f64>(&[profile(), profile()], &stacked_at(&[0.0, 1.0]), 1, tol())
        .expect("the prism builds")
        .body
}

/// The arc wall: the face whose rows include the `IsoArc` rims.
fn arc_wall(body: &Body<f64>) -> FaceKey {
    let rim = body
        .pcurves()
        .find(|(_, c)| matches!(c.pcurve(), Pcurve::IsoArc { .. }))
        .map(|(h, _)| h)
        .expect("the prism has an arc rim");
    body.face_of_half_edge(rim).unwrap()
}

/// The wall's half-edges.
fn wall_half_edges(body: &Body<f64>, wall: FaceKey) -> Vec<HalfEdgeKey> {
    body.half_edges()
        .map(|(k, _)| k)
        .filter(|&k| body.face_of_half_edge(k) == Some(wall))
        .collect()
}

/// `body` with `wall` re-charted in place onto `remap(its surface)`,
/// stating `sense`.
fn recharted(
    body: &Body<f64>,
    wall: FaceKey,
    remap: impl Fn(&NurbsSurface<f64>) -> NurbsSurface<f64>,
    flip_sense: bool,
) -> Body<f64> {
    let mut out = body.clone();
    let face = out.get_face(wall).unwrap();
    let (key, sense) = (face.surface, face.sense);
    let Some(Surface::Nurbs(net)) = out.get_surface(key).cloned() else {
        panic!("the arc wall is a spline chart")
    };
    // Lifts RechartUnvouched and RechartStrandsDescriptions: the wall
    // re-charted in place onto the same surface reparameterized, its
    // edges' descriptions left as they were, is the row's premise.
    out.set_face_surface_unvouched_for_tests(
        wall,
        FaceSurface::New {
            surface: Surface::Nurbs(std::sync::Arc::new(remap(&net))),
            sense: sense != flip_sense,
        },
    )
    .unwrap();
    out
}

/// One stored wall row: its kind, its `p0`, and its moving channel.
type Row = (&'static str, (f64, f64), (f64, f64));

/// The minted wall's rows, sorted: the two seams are columns and the
/// two rims are rows.
fn stored_rows(body: &Body<f64>, wall: FaceKey) -> Vec<Row> {
    let mut rows: Vec<_> = wall_half_edges(body, wall)
        .into_iter()
        .map(|h| {
            match body
                .pcurve(h)
                .unwrap_or_else(|| panic!("{h:?}: the mint stores the wall's row"))
                .pcurve()
            {
                Pcurve::IsoLine { p0, pl } => ("column", (p0.x, p0.y), (pl.x, pl.y)),
                Pcurve::IsoArc { p0, pd, .. } => ("rim", (p0.x, p0.y), (pd.x, pd.y)),
                other => panic!("{h:?}: the wall's rows are exact iso images: {other:?}"),
            }
        })
        .collect();
    rows.sort_by(|a, b| a.partial_cmp(b).unwrap());
    rows
}

/// **A wall reversed in `v`: each seam takes the column that runs
/// against the neighbour's `v`.** The mint stores every wall row: the
/// two seams on columns `u = 0` and `u = 1` running from `v = 1` down
/// to `v = 0`, and the two rims each on the other row, running with
/// `u`. The native wall is the control, with every row forward.
#[test]
fn a_wall_reversed_in_v_takes_its_seams_reversed() {
    let body = semicircle_prism();
    let wall = arc_wall(&body);
    let mut native = body.clone();
    assert_eq!(
        topo::mint_pcurves(&mut native, tol()),
        Ok(()),
        "the control mints"
    );
    let forward = stored_rows(&native, wall);

    let mut reversed = recharted(&body, wall, |n| n.reversed_v().unwrap(), true);
    assert_eq!(
        topo::mint_pcurves(&mut reversed, tol()),
        Ok(()),
        "the mint stores the reversed wall's rows"
    );
    let got = stored_rows(&reversed, wall);
    let columns: Vec<_> = got.iter().filter(|r| r.0 == "column").collect();
    assert_eq!(
        columns,
        [
            &("column", (0.0, 1.0), (0.0, -1.0)),
            &("column", (1.0, 1.0), (0.0, -1.0))
        ],
        "each seam runs down its column, against the neighbour's v"
    );
    let rims: Vec<_> = got
        .iter()
        .filter(|r| r.0 == "rim")
        .map(|r| (r.1, r.2))
        .collect();
    let mut rims_forward: Vec<_> = forward
        .iter()
        .filter(|r| r.0 == "rim")
        .map(|r| ((r.1.0, 1.0 - r.1.1), r.2))
        .collect();
    rims_forward.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(
        rims, rims_forward,
        "each rim moves to the other row, still with u"
    );
}

/// **A wall reversed in `u` and `v`: the seams swap columns and run
/// reversed.** Every row of the wall is the native one mirrored in both
/// channels, and the mint stores them all.
#[test]
fn a_wall_reversed_in_u_and_v_takes_its_seams_reversed() {
    let body = semicircle_prism();
    let wall = arc_wall(&body);
    let mut reversed = recharted(
        &body,
        wall,
        |n| n.reversed_u().unwrap().reversed_v().unwrap(),
        false,
    );
    assert_eq!(
        topo::mint_pcurves(&mut reversed, tol()),
        Ok(()),
        "the mint stores the twice-reversed wall's rows"
    );
    let got = stored_rows(&reversed, wall);
    assert_eq!(
        got,
        [
            ("column", (0.0, 1.0), (0.0, -1.0)),
            ("column", (1.0, 1.0), (0.0, -1.0)),
            ("rim", (1.0, 0.0), (-1.0, 0.0)),
            ("rim", (1.0, 1.0), (-1.0, 0.0)),
        ],
        "every row is the native one mirrored in u and v"
    );
}
