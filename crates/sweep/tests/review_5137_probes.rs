//! review 5137 probes
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2};
use geom::{NurbsSurface, Surface};
use geom_brep::{
    CertifyError, EdgeDescriptionSpec, PcurveCertifyError, PlaneNurbsRefusal, SurfacePair,
};
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use sweep::test_support::{stacked_at, tube_frame};
use sweep::{TubeWindow, tube_along_arc};
use topo::pcurves::PcurveMintError;
use topo::{Body, EulerOpError, FaceKey, FaceSurface, HalfEdgeKey, Pcurve};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
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

/// Whether the face lies on `image`'s side of `h`: the chart's step
/// into the face from the image's row (`+v` off `v = 0`, `−v` off
/// `v = 1`) points left of `h` about the face's outward normal, at the
/// middle of the edge. Read in 3-D off the net and the carrier, not off
/// the pick's own algebra.
fn face_is_on_the_rows_side(body: &Body<f64>, h: HalfEdgeKey, image: &Pcurve<f64>) -> bool {
    let face = body.get_face(body.face_of_half_edge(h).unwrap()).unwrap();
    let surface = body.get_surface(face.surface).unwrap();
    let edge = body.get_edge(body.get_half_edge(h).unwrap().edge).unwrap();
    let curve = body
        .get_curve_geom(edge.curve)
        .unwrap()
        .certified()
        .unwrap();
    let (t0, t1) = curve.params();
    let (mid, step) = (0.5 * (t0 + t1), 1e-4);
    let along = if edge.he_plus == h { 1.0 } else { -1.0 };
    let tangent = (curve.carrier().eval(mid + step) - curve.carrier().eval(mid - step)) * along;
    let uv = image.eval(mid);
    let outward = surface.normal(uv.x, uv.y) * if face.sense { 1.0 } else { -1.0 };
    let into = if uv.y < 0.5 { step } else { -step };
    let inward = surface.eval(uv.x, uv.y + into) - surface.eval(uv.x, uv.y);
    outward.cross(tangent).dot(inward) > 0.0
}


fn surf_flip_variant(body: &mut Body<f64>, wall: FaceKey, which: u8, flip_sense: bool) -> bool {
    let face = body.get_face(wall).unwrap();
    let (key, sense) = (face.surface, face.sense);
    let Some(Surface::Nurbs(net)) = body.get_surface(key).cloned() else { panic!() };
    let (n, flips) = match which {
        0 => ((*net).clone(), false),
        1 => (net.reversed_v().unwrap(), true),
        2 => (net.reversed_u().unwrap(), true),
        _ => (net.reversed_u().unwrap().reversed_v().unwrap(), false),
    };
    let s = sense ^ flips ^ flip_sense;
    body.set_face_surface_unvouched_for_tests(
        wall,
        FaceSurface::New { surface: Surface::Nurbs(std::sync::Arc::new(n)), sense: s },
    )
    .unwrap();
    s
}

/// Claim 1: every net orientation, both senses.
#[test]
fn r_p1_side_rule_over_net_orientations_and_senses() {
    let mut bad = Vec::new();
    for which in 0..4u8 {
        for flip_sense in [false, true] {
            let (mut body, wall, halves) = seam_tube();
            let s = surf_flip_variant(&mut body, wall, which, flip_sense);
            let mut rows = Vec::new();
            for h in halves {
                let image = derived(&body, h).unwrap();
                let side = face_is_on_the_rows_side(&body, h, &image);
                eprintln!("variant {which} flip_sense {flip_sense} sense {s} {h:?}: {image:?} side_ok {side}");
                if !side { bad.push((which, flip_sense, h)); }
                if let Pcurve::IsoArc { p0, .. } = image { rows.push(p0.y); }
            }
            rows.sort_by(f64::total_cmp);
            if rows != [0.0, 1.0] { bad.push((which, flip_sense, halves[0])); }
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}

/// Claim 5: a LINE seam along u on a spline chart closed in v — the
/// boundary-iso schedule's rows `[cv0, cv1]` in fixed order.
#[test]
fn r_p2_line_seam_on_a_chart_closed_in_v() {
    use profile::test_support::bulge_loop;
    use sweep::test_support::stacked_at;
    let v = |x: f64, y: f64, b: f64| (Point2::new(x, y), b);
    let profile = || vec![bulge_loop(vec![v(0.0, 0.0, 1.0), v(2.0, 0.0, 1.0)])];
    let mut body = sweep::loft_body::<f64>(&[profile(), profile()], &stacked_at(&[0.0, 1.0]), 1, tol())
        .expect("the cylinder builds")
        .body;
    // the two vertical edges between the two walls
    let verticals: Vec<_> = body
        .edges()
        .filter(|(_, e)| {
            let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
            let (t0, t1) = c.params();
            (c.carrier().eval(t1).z - c.carrier().eval(t0).z).abs() > 0.5
        })
        .map(|(k, e)| {
            let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
            let (t0, _) = c.params();
            eprintln!("vertical carrier {:?}", c.carrier());
            (c.carrier().eval(t0), k)
        })
        .collect();
    eprintln!("verticals {verticals:?}");
    assert_eq!(verticals.len(), 2);
    let (kill, keep) = if verticals[0].0.x > verticals[1].0.x { (verticals[0].1, verticals[1].1) } else { (verticals[1].1, verticals[0].1) };
    let e = body.get_edge(kill).unwrap().clone();
    let r = body.kef(e.he_plus);
    eprintln!("kef plus: {r:?}");
    if r.is_err() { body.lifting_rechart_refusals_for_tests(|b| b.kef(e.he_plus)).expect("kef joins the walls"); }
    let seam = body.get_edge(keep).unwrap();
    let halves = [seam.he_plus, seam.he_minus];
    let wall = body.face_of_half_edge(halves[0]).unwrap();
    assert_eq!(body.face_of_half_edge(halves[1]), Some(wall), "a seam");
    let face = body.get_face(wall).unwrap();
    eprintln!("seam description: {:?}", body.get_curve_geom(seam.curve).unwrap().certified().unwrap().description());
    // outward at the seam point (0,0,·) is -x
    let old = body.get_surface(face.surface).unwrap().clone();
    eprintln!("old surface kind nurbs? {}", matches!(old, Surface::Nurbs(_)));
    // net: u axial (z 0..1, degree 1), v around circle c=(1,0) r=1 from angle pi, ccw
    let c = (1.0, 0.0);
    let ring = [(-1.0, 0.0), (-1.0, -1.0), (0.0, -1.0), (1.0, -1.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0), (-1.0, 0.0)];
    let wv: Vec<f64> = [1.0, FRAC_1_SQRT_2].repeat(4).into_iter().chain([1.0]).collect();
    let mut points = Vec::new();
    let mut weights = Vec::new();
    for z in [0.0, 1.0] {
        for (&(dx, dy), &w) in ring.iter().zip(&wv) {
            points.push(Point3::new(c.0 + dx, c.1 + dy, z));
            weights.push(w);
        }
    }
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0], 2).unwrap();
    let net = NurbsSurface::new(Knots::from(ku), Knots::from(kv), points, weights).unwrap();
    for which in [0u8, 1] {
        let mut b = body.clone();
        let n = if which == 0 { net.clone() } else { net.reversed_v().unwrap() };
        let s0 = Surface::Nurbs(std::sync::Arc::new(n));
        // sense: net normal at seam point vs outward -x
        let nn = s0.normal(0.5, 0.0);
        let sense = nn.x < 0.0;
        b.set_face_surface_unvouched_for_tests(wall, FaceSurface::New { surface: s0, sense }).unwrap();
        let mut rows = Vec::new();
        for h in halves {
            let image = derived(&b, h);
            eprintln!("v-dir {which} sense {sense} {h:?}: {image:?}");
            if let Ok(image) = image {
                let side = face_is_on_the_rows_side(&b, h, &image);
                eprintln!("   side_ok {side}");
                if let Pcurve::IsoLine { p0, .. } = image { rows.push(p0.y); }
            }
        }
        eprintln!("rows {rows:?}");
    }
}

/// Claim 4: the cap arcs' General images, their end feet.
#[test]
fn r_p3_cap_arc_feet() {
    let (body, wall, halves) = seam_tube();
    let lp: Vec<HalfEdgeKey> = body
        .pcurves()
        .map(|(h, _)| h)
        .chain(body.edges().flat_map(|(_, e)| [e.he_plus, e.he_minus]))
        .filter(|h| body.face_of_half_edge(*h) == Some(wall) && !halves.contains(h))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    for h in lp {
        let edge = body.get_edge(body.get_half_edge(h).unwrap().edge).unwrap();
        let c = body.get_curve_geom(edge.curve).unwrap().certified().unwrap();
        let (t0, t1) = c.params();
        match derived(&body, h) {
            Ok(img) => {
                let k: Vec<_> = (0..=8).map(|i| { let p = img.eval(t0 + (t1 - t0) * f64::from(i) / 8.0); (format!("{:.3}", p.x), format!("{:.3}", p.y)) }).collect();
                eprintln!("{h:?} plus={} {:?}", edge.he_plus == h, k);
            }
            Err(e) => eprintln!("{h:?} err {e:?}"),
        }
    }
    let mut b = body.clone();
    eprintln!("mint: {:?}", topo::mint_pcurves(&mut b, tol()));
}
