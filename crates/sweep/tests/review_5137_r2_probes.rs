//! Review probes for PR 5137 (r2).

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

/// The quarter tube's major and minor radii.
const MAJOR: f64 = 1.0;
const MINOR: f64 = 0.25;

/// The exact torus of the quarter tube as one rational biquadratic
/// net: `u` the azimuth over the window (one span from `+x` to `+z`
/// about `−y`), `v` the whole tube circle (four quarter spans) from the
/// outer equator back to it. Its first and last control rows are the
/// same points, so the chart is closed in `v`.
fn torus_net(rev_u: bool, rev_v: bool) -> NurbsSurface<f64> {
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
    let mut radial = radial.to_vec();
    let mut section = section.to_vec();
    if rev_u { radial.reverse(); }
    if rev_v { section.reverse(); }
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
/// face. The wall is then re-charted on `torus_net`.
fn seam_tube(rev_u: bool, rev_v: bool) -> (Body<f64>, FaceKey, [HalfEdgeKey; 2]) {
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
            surface: Surface::Nurbs(std::sync::Arc::new(torus_net(rev_u, rev_v))),
            sense: sense ^ rev_u ^ rev_v,
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


fn check(body: &Body<f64>, halves: [HalfEdgeKey; 2], what: &str) {
    let mut rows = Vec::new();
    for h in halves {
        let image = derived(body, h).unwrap();
        let (u0, du, row, dv) = arc(&image);
        eprintln!("{what}: {h:?} is_plus={} sense={} -> u0={u0} du={du} row={row} dv={dv}",
            body.get_edge(body.get_half_edge(h).unwrap().edge).unwrap().he_plus == h,
            body.get_face(body.face_of_half_edge(h).unwrap()).unwrap().sense);
        assert!(face_is_on_the_rows_side(body, h, &image), "{what}: {h:?} row {row} wrong side");
        rows.push(row);
    }
    rows.sort_by(f64::total_cmp);
    assert_eq!(rows, [0.0, 1.0], "{what}");
}

/// The net's outward normal agrees with the torus's at one wall point.
fn outward_agrees(body: &Body<f64>, wall: FaceKey) {
    let face = body.get_face(wall).unwrap();
    let s = body.get_surface(face.surface).unwrap();
    let n = s.normal(0.5, 0.1) * if face.sense { 1.0 } else { -1.0 };
    let p = s.eval(0.5, 0.1);
    // torus axis -y through origin: centre circle point nearest p
    let radial = Vec3::new(p.x, 0.0, p.z);
    let c = Point3::origin() + radial * (MAJOR / radial.norm());
    assert!(n.dot(p - c) > 0.0, "the re-charted wall's outward normal points out of the tube");
}

#[test]
fn p1_net_variants() {
    for (ru, rv) in [(false, false), (true, false), (false, true), (true, true)] {
        let (body, wall, halves) = seam_tube(ru, rv);
        outward_agrees(&body, wall);
        check(&body, halves, &format!("rev_u={ru} rev_v={rv}"));
    }
}

#[test]
fn p2_reverted_body() {
    for (ru, rv) in [(false, false), (true, true)] {
        let (body, _, _) = seam_tube(ru, rv);
        let r = body.revert();
        // find the seam on the reverted body: an edge both halves of which bound one face, circle carrier
        let seams: Vec<_> = r.edges().filter(|(_, e)| r.face_of_half_edge(e.he_plus) == r.face_of_half_edge(e.he_minus)
            && matches!(r.get_curve_geom(e.curve).unwrap().certified().unwrap().carrier(), geom::Curve3::Circle { .. }))
            .map(|(_, e)| [e.he_plus, e.he_minus]).collect();
        assert_eq!(seams.len(), 1);
        check(&r, seams[0], &format!("reverted rev_u={ru} rev_v={rv}"));
    }
}

/// A full cylinder (r = 1, z ∈ [0, 1]) as one wall face, its seam the
/// generator at +x, re-charted on a net with u AXIAL and v AROUND — so
/// the chart is closed in v and the seam is a LINE on both rows: the
/// iso_schedule's row pick, the arc rim's sibling.
fn seam_cylinder() -> (Body<f64>, FaceKey, [HalfEdgeKey; 2]) {
    let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
    let mut body = sweep::test_support::extruded::<f64>(
        sweep::test_support::sketch_at(0.0),
        vec![bulge_loop(vec![v(1.0, 0.0, 1.0), v(-1.0, 0.0, 1.0)])],
        1.0,
        tol(),
    );
    let on_cyl = |body: &Body<f64>, h: HalfEdgeKey| {
        let face = body.get_face(body.face_of_half_edge(h).unwrap()).unwrap();
        matches!(body.get_surface(face.surface), Some(Surface::Cylinder { .. }))
    };
    let lines: Vec<_> = body
        .edges()
        .filter(|(_, e)| on_cyl(&body, e.he_plus) && on_cyl(&body, e.he_minus))
        .map(|(k, e)| {
            let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
            let x = c.carrier().eval(c.params().0).x;
            (x, k)
        })
        .collect();
    assert_eq!(lines.len(), 2, "{lines:?}");
    let (kill, seam) = if lines[0].0 < lines[1].0 { (lines[0].1, lines[1].1) } else { (lines[1].1, lines[0].1) };
    body.kef(body.get_edge(kill).unwrap().he_plus).unwrap();
    let e = body.get_edge(seam).unwrap();
    let halves = [e.he_plus, e.he_minus];
    let wall = body.face_of_half_edge(halves[0]).unwrap();
    assert_eq!(body.face_of_half_edge(halves[1]), Some(wall));
    let s = FRAC_1_SQRT_2;
    let section = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0), (-1.0, 0.0), (-1.0, -1.0), (0.0, -1.0), (1.0, -1.0), (1.0, 0.0)];
    let wv = [1.0, s, 1.0, s, 1.0, s, 1.0, s, 1.0];
    let mut points = Vec::new();
    let mut weights = Vec::new();
    for z in [0.0, 1.0] {
        for (&(x, y), &w) in section.iter().zip(&wv) {
            points.push(Point3::new(x, y, z));
            weights.push(w);
        }
    }
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0], 2).unwrap();
    let net = NurbsSurface::new(Knots::from(ku), Knots::from(kv), points, weights).unwrap();
    // du × dv = z × (dθ) = −radial: outward needs sense false.
    let net = Surface::Nurbs(std::sync::Arc::new(net));
    let n = net.normal(0.5, 0.1);
    let p = net.eval(0.5, 0.1);
    let sense = n.dot(Vec3::new(p.x, p.y, 0.0)) > 0.0;
    body.set_face_surface_unvouched_for_tests(
        wall,
        FaceSurface::New { surface: net, sense },
    )
    .unwrap();
    (body, wall, halves)
}

#[test]
fn p3_line_seam_on_a_chart_closed_in_v() {
    let (body, _, halves) = seam_cylinder();
    let mut rows = Vec::new();
    for h in halves {
        let image = derived(&body, h);
        eprintln!("line seam {h:?}: is_plus={} -> {image:?}",
            body.get_edge(body.get_half_edge(h).unwrap().edge).unwrap().he_plus == h);
        let image = image.unwrap();
        let side = face_is_on_the_rows_side(&body, h, &image);
        let uv = image.eval(0.5);
        eprintln!("   row v at mid = {}, on own side = {side}", uv.y);
        rows.push((uv.y, side));
    }
    assert!(rows.iter().all(|r| r.1), "each line-seam half on its own side: {rows:?}");
}

#[test]
fn p4_mint_on_the_seam_tube() {
    let (mut body, wall, halves) = seam_tube(false, false);
    let r = topo::pcurves::mint_pcurves_of(&mut body, &[wall], tol());
    eprintln!("mint seam tube: {r:?}");
    let _ = halves;
    let (body, wall, _) = seam_tube(false, false);
    for (_, e) in body.edges() {
        for h in [e.he_plus, e.he_minus] {
            if body.face_of_half_edge(h) == Some(wall) {
                let img = derived(&body, h);
                let ends = img.as_ref().ok().map(|i| {
                    let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
                    let (a, b) = c.params();
                    (i.eval(a), i.eval(b))
                });
                eprintln!("  {h:?} plus={} ends(t0,t1)={ends:?} kind={}", e.he_plus == h,
                    match &img { Ok(Pcurve::General(_)) => "General", Ok(Pcurve::IsoArc{..}) => "IsoArc", Ok(_) => "other", Err(_) => "err" });
            }
        }
    }
}
