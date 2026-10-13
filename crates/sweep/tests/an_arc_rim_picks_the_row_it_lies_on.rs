//! **An arc rim on a spline chart picks the row it lies on**, in
//! whichever `u` direction its carrier runs.
//!
//! The fixture is a loft of a profile with one semicircular edge: that
//! wall is a rational-quadratic spline chart whose two boundary rows
//! are the edge's rims, each an arc carried by a circle and charted as
//! `Pcurve::IsoArc`. Re-charting the wall with its `u` reversed (the
//! same surface, `reversed_u`) leaves each rim running AGAINST the
//! chart's `u`, as an imported wall's rim may: its start is the chart's
//! `u₁` corner, not its `u₀` one.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::PcurveCertifyError;
use geom_core::{Band, Point2, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::stacked_at;
use topo::pcurves::PcurveMintError;
use topo::{Body, FaceKey, FaceSurface, HalfEdgeKey, Pcurve};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// A prism whose profile's second edge is a semicircle (bulge 1), so
/// its wall's rims are half-period arcs, `breaks` mirror-symmetric and
/// the wall reversible in `u` bitwise.
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

/// The arc wall and its two rim half-edges, `v = 0` first.
fn arc_wall(body: &Body<f64>) -> (FaceKey, [HalfEdgeKey; 2]) {
    let mut rims: Vec<(f64, HalfEdgeKey)> = body
        .pcurves()
        .filter_map(|(h, c)| match c.pcurve() {
            Pcurve::IsoArc { p0, .. } => Some((p0.y, h)),
            _ => None,
        })
        .collect();
    rims.sort_by(|a, b| a.0.total_cmp(&b.0));
    let [(_, bottom), (_, top)] = rims.as_slice() else {
        panic!("the arc wall's two rims are its IsoArc rows: {rims:?}")
    };
    let face = body.face_of_half_edge(*bottom).unwrap();
    assert_eq!(body.face_of_half_edge(*top), Some(face), "one wall");
    (face, [*bottom, *top])
}

/// What `nurbs_iso_derive` answers for `h`, the stored row detached:
/// `pcurve_of` answers a cached image first.
fn derived(body: &Body<f64>, h: HalfEdgeKey) -> Result<Pcurve<f64>, PcurveMintError> {
    let mut fresh = body.clone();
    fresh.detach_pcurve(h);
    topo::pcurve_of(&fresh, h, band())
}

/// `(p0.x, pd.x, p0.y)` of an IsoArc image.
fn arc(image: &Pcurve<f64>) -> (f64, f64, f64) {
    match image {
        Pcurve::IsoArc { p0, pd, .. } => (p0.x, pd.x, p0.y),
        other => panic!("an arc rim's image is an IsoArc: {other:?}"),
    }
}

/// **A rim against the chart's `u` takes its reversed row.** Each rim
/// derives the IsoArc that starts at `u = 1` and runs to `u = 0` on its
/// own row, and the mint certifies and stores it. The native wall is
/// the control: the same rims, forward. (Tier 3 is not read: the lifted
/// re-chart leaves the rims' descriptions naming the old chart.)
#[test]
fn a_rim_against_the_charts_u_takes_its_reversed_row() {
    let body = semicircle_prism();
    let (wall, rims) = arc_wall(&body);
    for (h, row) in rims.into_iter().zip([0.0, 1.0]) {
        assert_eq!(
            arc(&derived(&body, h).unwrap()),
            (0.0, 1.0, row),
            "the control: a native rim runs with the chart's u"
        );
    }

    let mut reversed = body.clone();
    let face = reversed.get_face(wall).unwrap();
    let (key, sense) = (face.surface, face.sense);
    let Some(Surface::Nurbs(net)) = reversed.get_surface(key).cloned() else {
        panic!("the arc wall is a spline chart")
    };
    // Lifts RechartUnvouched and RechartStrandsDescriptions: the wall
    // re-charted in place onto the same surface reversed in `u`, its
    // edges' descriptions left as they were, is the row's premise.
    reversed
        .set_face_surface_unvouched_for_tests(
            wall,
            FaceSurface::New {
                surface: Surface::Nurbs(std::sync::Arc::new(net.reversed_u().unwrap())),
                sense: !sense,
            },
        )
        .unwrap();
    for (h, row) in rims.into_iter().zip([0.0, 1.0]) {
        assert_eq!(
            arc(&derived(&reversed, h).unwrap()),
            (1.0, -1.0, row),
            "{h:?}: the rim runs from u = 1 to u = 0 on its own row"
        );
    }
    assert_eq!(
        topo::mint_pcurves(&mut reversed, tol()),
        Ok(()),
        "the mint stores the reversed wall's rows"
    );
    for h in rims {
        let stored = reversed.pcurve(h).expect("the rim's row is stored");
        assert_eq!(
            arc(stored.pcurve()).1,
            -1.0,
            "{h:?}: the stored row runs against u"
        );
    }
}

/// **A half-rim is refused for its map, not for its row.** Splitting a
/// rim leaves two arcs that lie on the chart's row `v = 0` but each
/// span half its `u` domain, which the IsoArc map (the edge's whole
/// carrier onto the whole domain) cannot state. The second half starts
/// inside the domain, so a row picked off the start would refuse it for
/// lying on no boundary; both halves are refused for the map instead.
#[test]
fn a_half_rim_is_refused_for_its_map_not_its_row() {
    let mut body = semicircle_prism();
    let (wall, [bottom, _]) = arc_wall(&body);
    let edge = body.get_half_edge(bottom).unwrap().edge;
    let (t0, t1) = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .params();
    let made = body.split_edge(edge, 0.5 * (t0 + t1), tol()).unwrap();
    let halves = [made.he_plus, made.he_minus, bottom]
        .into_iter()
        .filter(|h| body.face_of_half_edge(*h) == Some(wall));
    let mut refused = 0;
    for h in halves {
        match derived(&body, h) {
            Err(PcurveMintError::Certify {
                error: PcurveCertifyError::IsoUnsupported { what },
                ..
            }) => assert!(
                what.contains("does not span this chart's u domain"),
                "{h:?}: the half-rim is refused for its map: {what}"
            ),
            other => panic!("{h:?}: a half-rim has no IsoArc image: {other:?}"),
        }
        refused += 1;
    }
    assert_eq!(refused, 2, "both halves of the split rim are on the wall");
}
