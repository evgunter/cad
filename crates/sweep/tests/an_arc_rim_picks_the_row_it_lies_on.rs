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
//!
//! A second fixture closes the chart in `v`: a quarter tube whose wall
//! is ONE face on the exact rational torus net, its outer equator a
//! seam the face meets from both sides. Both rows hold the seam's
//! circle, and the seam has one image.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2};
use geom::{NurbsSurface, Surface};
use geom_brep::{CertifyError, EdgeDescriptionSpec, PlaneNurbsRefusal, SsiLimb, SurfacePair};
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use sweep::test_support::{stacked_at, tube_frame};
use sweep::{TubeWindow, tube_along_arc};
use topo::construction::Construct;
use topo::construction::FaceSurface;
use topo::pcurves::PcurveMintError;
use topo::{Body, EulerOpError, FaceKey, HalfEdgeKey, Pcurve};

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

/// `(p0.x, pd.x, p0.y, pd.y)` of an IsoArc image: where it starts, how
/// far `u` moves, its row, and how far `v` moves.
fn arc(image: &Pcurve<f64>) -> (f64, f64, f64, f64) {
    match image {
        Pcurve::IsoArc { p0, pd, .. } => (p0.x, pd.x, p0.y, pd.y),
        other => panic!("an arc rim's image is an IsoArc: {other:?}"),
    }
}

/// **A rim against the chart's `u` takes its reversed row.** Each rim
/// derives the IsoArc that starts at `u = 1` and runs to `u = 0` on its
/// own row, and the mint certifies and stores exactly that row. The native wall is
/// the control: the same rims, forward. (Tier 3 is not read: the lifted
/// re-chart leaves the rims' descriptions naming the old chart.)
#[test]
fn a_rim_against_the_charts_u_takes_its_reversed_row() {
    let body = semicircle_prism();
    let (wall, rims) = arc_wall(&body);
    for (h, row) in rims.into_iter().zip([0.0, 1.0]) {
        assert_eq!(
            arc(&derived(&body, h).unwrap()),
            (0.0, 1.0, row, 0.0),
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
            (1.0, -1.0, row, 0.0),
            "{h:?}: the rim runs from u = 1 to u = 0 on its own row"
        );
    }
    assert_eq!(
        topo::mint_pcurves(&mut reversed, tol()),
        Ok(()),
        "the mint stores the reversed wall's rows"
    );
    for (h, row) in rims.into_iter().zip([0.0, 1.0]) {
        let stored = reversed.pcurve(h).expect("the rim's row is stored");
        assert_eq!(
            arc(stored.pcurve()),
            (1.0, -1.0, row, 0.0),
            "{h:?}: the stored row runs from u = 1 to u = 0 on its own row"
        );
    }
}

/// **A half-rim takes its row over the row's own arc.** Splitting a rim
/// leaves two arcs that lie on the chart's row `v = 0` but each span half
/// its `u` domain. The second half starts inside the domain, so a row
/// picked off the start would refuse it before any candidate was judged;
/// both halves are judged at the probe, and each derives the row the
/// parent had, `u` from 0 to 1 over the parent's arc, in which the
/// half's span lies.
#[test]
fn a_half_rim_takes_its_rows_arc() {
    let mut body = semicircle_prism();
    let (wall, [bottom, _]) = arc_wall(&body);
    let parent = body.pcurve(bottom).unwrap().pcurve().clone();
    let edge = body.get_half_edge(bottom).unwrap().edge;
    let (t0, t1) = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .params();
    let made = body.split_edge(edge, 0.5 * (t0 + t1), tol()).unwrap();
    let halves: Vec<_> = [made.he_plus, made.he_minus, bottom]
        .into_iter()
        .filter(|h| body.face_of_half_edge(*h) == Some(wall))
        .collect();
    assert_eq!(
        halves.len(),
        2,
        "both halves of the split rim are on the wall"
    );
    let Pcurve::IsoArc {
        t0: arc_t0,
        angle: arc_angle,
        ..
    } = parent
    else {
        panic!("the rim's row is an IsoArc: {parent:?}")
    };
    for h in halves {
        let image = derived(&body, h).unwrap();
        assert_eq!(arc(&image), (0.0, 1.0, 0.0, 0.0), "{h:?}: the parent's row");
        let Pcurve::IsoArc { t0, angle, .. } = image else {
            unreachable!()
        };
        assert!(
            (t0 - arc_t0).abs() < 1e-12 && (angle - arc_angle).abs() < 1e-12,
            "{h:?}: the parent's arc ({arc_t0}, {arc_angle}), measured: ({t0}, {angle})"
        );
    }
}

/// **An `Intersection` circle on a spline chart is refused at the
/// door, on the certificate's own image.** The spline Intersection certificate
/// takes a circle carrier and reads it as it reads a spline, so the
/// rim's circle restated as the cap × wall `Intersection` reaches limb
/// 2: its chart image, degree 1 over the schedule's feet, misses the
/// semicircle by its chord error, and the hull measures that past ε at
/// a break where the circle's own foot is on the wall. The refusal says
/// the image misses, not that the exact circle is off its surfaces.
/// `nurbs_iso_derive` offers an `Intersection` circle the door does
/// certify to the arc rim, and keeps the rim only where it certifies.
#[test]
fn an_intersection_circle_on_a_spline_wall_is_refused_at_the_door() {
    let mut body = semicircle_prism();
    let (_, [bottom, _]) = arc_wall(&body);
    let edge = body.get_half_edge(bottom).unwrap().edge;
    let geom = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .clone();
    let (s1, s2) = topo::readback::edge_sides(&body, edge).unwrap().surfaces();
    let (t0, t1) = geom.params();
    let mut spec = geom.restated_spec();
    spec.description = EdgeDescriptionSpec::Intersection {
        pair: SurfacePair::new(s1, s2),
        selector: geom_brep::CurveSelector::Witness(geom.carrier().eval(0.5 * (t0 + t1))),
    };
    let before = format!("{body:?}");
    let refused = body.set_edge_curve(edge, spec, tol());
    let Err(EulerOpError::Certification {
        error: ref error @ CertifyError::PlaneNurbs(PlaneNurbsRefusal::Limb { limb, margin }),
    }) = refused
    else {
        panic!(
            "the rim's circle is refused at a limb of the spline Intersection certificate: {refused:?}"
        )
    };
    assert_eq!(
        limb,
        SsiLimb::ImageMiss,
        "limb 2 refuses it at a break where the circle is on the wall"
    );
    let miss = margin.diagnostic_f64_for_error_text().value();
    assert!(
        miss.is_some_and(|m| (band().zero()..1e-2).contains(&m)),
        "the chord error, measured past ε: {miss:?}"
    );
    let words = error.to_string();
    assert!(
        words.contains("own image of the carrier misses the carrier's foot path")
            && words.contains("not shown off its surfaces"),
        "the refusal names the image, not the carrier: {words}"
    );
    assert_eq!(format!("{body:?}"), before, "the body is untouched");
}

/// The quarter tube's major and minor radii.
const MAJOR: f64 = 1.0;
const MINOR: f64 = 0.25;

/// The exact torus of the quarter tube as one rational biquadratic
/// net: `u` the azimuth over the window (one span from `+x` to `+z`
/// about `−y`), `v` the whole tube circle (four quarter spans) from the
/// outer equator back to it. Its first and last control rows are the
/// same points, so the chart is closed in `v`.
fn torus_net() -> NurbsSurface<f64> {
    let radial = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let wu = [1.0, FRAC_1_SQRT_2, 1.0];
    let (outer, inner) = (MAJOR + MINOR, MAJOR - MINOR);
    let section = [
        (outer, 0.0),
        (outer, MINOR),
        (MAJOR, MINOR),
        (inner, MINOR),
        (inner, 0.0),
        (inner, -MINOR),
        (MAJOR, -MINOR),
        (outer, -MINOR),
        (outer, 0.0),
    ];
    let wv = [1.0, FRAC_1_SQRT_2].repeat(4).into_iter().chain([1.0]);
    let wv: Vec<f64> = wv.collect();
    let mut points = Vec::new();
    let mut weights = Vec::new();
    for ((x, z), a) in radial.into_iter().zip(wu) {
        for (&(rho, h), &b) in section.iter().zip(&wv) {
            points.push(Point3::new(rho * x, -h, rho * z));
            weights.push(a * b);
        }
    }
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    NurbsSurface::new(Knots::from(ku), Knots::from(kv), points, weights).unwrap()
}

/// The quarter tube with its wall made one face closed in `v`, and the
/// two halves of its seam. The tube door builds the wall as two torus
/// faces split at both equators; killing the inner equator joins them,
/// and the outer one is left a seam whose two halves bound the one
/// face. The wall is then re-charted on [`torus_net`].
fn seam_tube() -> (Body<f64>, FaceKey, [HalfEdgeKey; 2]) {
    let mut body = tube_along_arc(
        tube_frame(
            Point3::origin(),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            tol(),
        ),
        MAJOR,
        TubeWindow::Arc {
            t0: 0.0,
            t1: FRAC_PI_2,
        },
        MINOR,
        tol(),
    )
    .expect("the quarter tube builds")
    .body;
    let on_torus = |body: &Body<f64>, h: HalfEdgeKey| {
        let face = body.get_face(body.face_of_half_edge(h).unwrap()).unwrap();
        matches!(body.get_surface(face.surface), Some(Surface::Torus { .. }))
    };
    let equators: Vec<_> = body
        .edges()
        .filter(|(_, e)| on_torus(&body, e.he_plus) && on_torus(&body, e.he_minus))
        .map(|(k, e)| {
            let curve = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
            let geom::Curve3::Circle { radius, .. } = *curve.carrier() else {
                panic!("an equator is a circle")
            };
            (radius, k)
        })
        .collect();
    let [(r0, e0), (r1, e1)] = equators.as_slice() else {
        panic!("the wall's two faces share its two equators: {equators:?}")
    };
    let ((_, inner), (_, outer)) = if r0 < r1 {
        ((r0, *e0), (r1, *e1))
    } else {
        ((r1, *e1), (r0, *e0))
    };
    body.kef(body.get_edge(inner).unwrap().he_plus)
        .expect("killing the inner equator joins the wall");
    let seam = body.get_edge(outer).unwrap();
    let halves = [seam.he_plus, seam.he_minus];
    let wall = body.face_of_half_edge(halves[0]).unwrap();
    assert_eq!(body.face_of_half_edge(halves[1]), Some(wall), "a seam");
    let sense = body.get_face(wall).unwrap().sense;
    // Lifts RechartStrandsDescriptions: the wall re-charted onto the
    // net with its edges' descriptions left naming the torus is the
    // row's premise, and it is what routes the seam to the arc rim.
    body.set_face_surface_unvouched_for_tests(
        wall,
        FaceSurface::New {
            surface: Surface::Nurbs(std::sync::Arc::new(torus_net())),
            sense,
        },
    )
    .unwrap();
    (body, wall, halves)
}

/// **A seam circle on a chart closed in `v` has one image** (C4: a row
/// is a function of the edge and the chart alone, and a seam has one
/// image and two joint elements). Both boundary rows hold the circle, and
/// each half takes the first row offered, `v = 0`, running with the
/// chart's `u`: the two halves' rows are the same image. Which lift of it
/// each half's walk reads is its joint's `v` element on the chart's `v`
/// period, the knot domain's length (`topo`'s
/// `a_net_closed_in_v_lifts_its_seam_crossing_by_a_v_period` carries that
/// on a net whose walk closes).
///
/// The loop walk is not read here: the wall's cap arcs, circles along
/// `v`, take `General` images whose foot at the seam vertex lands on the
/// `v = 0` branch, so their fit folds back across the chart
/// (`work/isotail/a-general-image-ending-on-a-closed-charts-seam-takes-its-foot-on-the-first-branch`).
///
/// The fixture is PR 5137's.
#[test]
fn a_seam_circle_on_a_chart_closed_in_v_has_one_image() {
    let (body, _, halves) = seam_tube();
    let images = halves.map(|h| derived(&body, h).unwrap());
    for (h, image) in halves.iter().zip(&images) {
        assert_eq!(
            arc(image),
            (0.0, 1.0, 0.0, 0.0),
            "{h:?}: the seam's image is the row v = 0, running with the chart's u"
        );
    }
    assert_eq!(
        format!("{:?}", images[0]),
        format!("{:?}", images[1]),
        "the seam's two halves derive one image"
    );
}
