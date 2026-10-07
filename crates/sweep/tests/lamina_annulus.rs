//! **A lamina full revolve's plane walls are one face each, with no
//! meridian slit**, and every door that reads them answers at a closed
//! form derived here: the topology itself, rim fillets whose host is a
//! plane annulus (outer cycle or ring — the latter a one-edge ring that
//! routes to the annulus band and its lone host trim), the ring
//! carry-through meter on rings that are not one circle, ruled bands
//! whose supports or caps carry the bore as a ring, and the shell and
//! offset doors.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Surface;
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::blend::BlendError;
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::test_support::{finished, prism_at, prism_on, realized, revolved_about_y, rim_arcs_at};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::BooleanOp;
use topo::readback::euler_counts;
use topo::{Body, EdgeKey, mass_properties, validate, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn v(x: f64, y: f64) -> (Point2<f64>, f64) {
    (Point2::new(x, y), 0.0)
}

/// A definite refusal's margin at the `f64` scalar.
fn reading(m: &sweep::blend::ClassifiedMargin) -> f64 {
    assert_eq!(m.sign, geom_core::Sign::Negative, "a definite refusal");
    m.reading
        .diagnostic_f64_for_error_text()
        .value()
        .expect("a definite reading")
}

fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, tol())
        .expect("mass properties")
        .volume
}

/// Every plane face: no edge with both halves in it (no slit).
fn plane_slits(body: &Body<f64>) -> usize {
    body.edges()
        .filter(|(_, e)| {
            let fp = body.face_of_half_edge(e.he_plus);
            let fm = body.face_of_half_edge(e.he_minus);
            fp.is_some()
                && fp == fm
                && matches!(
                    body.get_surface(body.get_face(fp.unwrap()).unwrap().surface),
                    Some(Surface::Plane { .. })
                )
        })
        .count()
}

/// Rings carried by plane faces.
fn plane_rings(body: &Body<f64>) -> usize {
    body.faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Plane { .. })))
        .map(|(_, f)| f.rings.len())
        .sum()
}

/// Spandrel area (square corner minus quarter disc) and its centroid's
/// distance from the corner along either leg.
fn spandrel(r: f64) -> (f64, f64) {
    (
        r * r * (1.0 - PI / 4.0),
        r * (10.0 - 3.0 * PI) / (3.0 * (4.0 - PI)),
    )
}

/// Volume swept by a rim spandrel whose corner is at radius `rc`;
/// `outward` when the spandrel lies at radii above `rc`.
fn rim_spandrel(rc: f64, r: f64, outward: bool) -> f64 {
    let (a, d) = spandrel(r);
    2.0 * PI * if outward { rc + d } else { rc - d } * a
}

// ---------------------------------------------------------------------
// The topology across the lamina matrix.
// ---------------------------------------------------------------------

/// A profile loop: its vertices and their bulges.
type Loop = Vec<(Point2<f64>, f64)>;

fn lamina_matrix() -> Vec<(&'static str, Vec<Loop>, i64)> {
    vec![
        (
            "washer",
            vec![vec![v(1., 0.), v(2., 0.), v(2., 1.), v(1., 1.)]],
            1,
        ),
        (
            "flange",
            vec![vec![
                v(1., 0.),
                v(3., 0.),
                v(3., 0.5),
                v(2., 0.5),
                v(2., 2.),
                v(1., 2.),
            ]],
            1,
        ),
        (
            "trapezoid",
            vec![vec![v(1., 0.), v(3., 0.), v(2.5, 1.), v(1.5, 1.)]],
            1,
        ),
        (
            "run on the bottom",
            vec![vec![v(1., 0.), v(1.5, 0.), v(2., 0.), v(2., 1.), v(1., 1.)]],
            1,
        ),
        (
            "stepped both ways",
            vec![vec![
                v(1., 0.),
                v(4., 0.),
                v(4., 1.),
                v(3., 1.),
                v(3., 2.),
                v(2., 2.),
                v(2., 1.),
                v(1., 1.),
            ]],
            1,
        ),
        (
            "holed ring",
            vec![
                vec![v(1., 0.), v(4., 0.), v(4., 3.), v(1., 3.)],
                vec![v(2., 1.), v(3., 1.), v(3., 2.), v(2., 2.)],
            ],
            2,
        ),
        (
            "axis-touching L (wire, control)",
            vec![vec![
                v(0., 0.),
                v(2., 0.),
                v(2., 0.5),
                v(1., 0.5),
                v(1., 1.),
                v(0., 1.),
            ]],
            0,
        ),
    ]
}

fn revolve_loops(loops: &[Loop]) -> Body<f64> {
    let lps: Vec<ProfileLoop<f64>> = loops
        .iter()
        .map(|l| ProfileLoop::polygon(l.iter().map(|p| p.0)))
        .collect();
    let vp = Profile::new(SketchPlane::<f64>::xy(), lps)
        .validate(tol())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, tol()).unwrap().body
}

#[test]
fn every_lamina_full_revolve_has_one_unslit_face_per_plane_wall() {
    for (what, loops, genus_sum) in lamina_matrix() {
        let body = revolve_loops(&loops);
        assert_eq!(validate(&body), Ok(()), "{what}: tier 1");
        assert_eq!(validate_geometric(&body, tol()), Ok(()), "{what}: tier 3");
        let c = euler_counts(&body);
        let g = c.genus().unwrap();
        assert_eq!(plane_slits(&body), 0, "{what}: no slit on a plane face");
        let annuli = match what {
            "washer" | "trapezoid" | "run on the bottom" => 2,
            "flange" => 3,
            "stepped both ways" | "holed ring" => 4,
            "axis-touching L (wire, control)" => 1,
            other => unreachable!("{other}: a matrix row without an annulus count"),
        };
        assert_eq!(
            plane_rings(&body),
            annuli,
            "{what}: each plane annulus carries its hole as a ring"
        );
        assert_eq!(g, genus_sum, "{what}: genus");
        // Every plane wall here is an annulus (or the wire-case disc);
        // each annulus carries exactly one ring.
        for (_, f) in body.faces() {
            if let Some(Surface::Plane { .. }) = body.get_surface(f.surface) {
                assert!(
                    f.rings.len() <= 1,
                    "{what}: a plane wall carries one ring at most"
                );
            }
        }
        let want = {
            // Pappus: 2π Σ x̄·A over the profile loops (outer minus holes).
            let area_moment = |l: &Loop| {
                let n = l.len();
                let (mut a, mut mx) = (0.0, 0.0);
                for i in 0..n {
                    let (p, q) = (l[i].0, l[(i + 1) % n].0);
                    let cr = p.x * q.y - q.x * p.y;
                    a += cr / 2.0;
                    mx += (p.x + q.x) * cr / 6.0;
                }
                (a, mx)
            };
            let (_, m0) = area_moment(&loops[0]);
            let holes: f64 = loops[1..].iter().map(|l| area_moment(l).1).sum();
            2.0 * PI * (m0.abs() - holes.abs())
        };
        let got = volume(&body);
        assert!(
            (got - want).abs() < 1e-9 * want,
            "{what}: volume {got} vs Pappus {want}"
        );
    }
}

// ---------------------------------------------------------------------
// Rim blends on unslit annuli, against closed forms.
// ---------------------------------------------------------------------

fn washer() -> Body<f64> {
    revolved_about_y(
        vec![v(1., 0.), v(2., 0.), v(2., 1.), v(1., 1.)],
        Revolution::Full,
        tol(),
    )
}

fn flange() -> Body<f64> {
    revolved_about_y(
        vec![
            v(1., 0.),
            v(3., 0.),
            v(3., 0.5),
            v(2., 0.5),
            v(2., 2.),
            v(1., 2.),
        ],
        Revolution::Full,
        tol(),
    )
}

/// Requests rims (radius, station) and checks ΔV against the sum of
/// signed spandrel volumes (`+1` adds, `-1` removes; `outward` as in
/// [`rim_spandrel`]).
fn rims_carve(
    what: &str,
    body: &Body<f64>,
    rims: &[((f64, f64), bool, f64)],
    r: f64,
) -> Result<(), BlendError> {
    let mut edges: Vec<EdgeKey> = Vec::new();
    for &((rr, y), _, _) in rims {
        let e = rim_arcs_at(body, rr, y);
        assert_eq!(e.len(), 1, "{what}: the rim at ({rr},{y}) is one edge");
        edges.extend(e);
    }
    let v0 = volume(body);
    let out =
        fillet_edges(&sweep::test_support::at_rest(body), &edges, r, tol()).map_err(|e| e.error)?;
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3 after the carve, {e:?}"));
    assert_eq!(validate(&out.body), Ok(()), "{what}: tier 1");
    let want: f64 = rims
        .iter()
        .map(|&((rc, _), outward, sign)| sign * rim_spandrel(rc, r, outward))
        .sum();
    let got = volume(&out.body) - v0;
    assert!(
        (got - want).abs() < 1e-9,
        "{what}: ΔV {got} vs closed form {want}"
    );
    assert_eq!(out.band_faces.len(), rims.len(), "{what}: one band per rim");
    Ok(())
}

/// **Each washer rim, and each set of them, removes exactly its
/// spandrel ring.** Every rim here is one edge whose plane host carries
/// it as a whole cycle — the outer rims as the annulus's outer cycle,
/// the bores as its ring — so each carve takes the lone host trim. A
/// trim that kept the wrong side of itself would carve the annulus's
/// remainder instead of the strip, which no spandrel ring's ΔV
/// survives: the rows pin the volume, not only counts and tier 3.
#[test]
fn each_washer_rim_and_rim_set_carves_at_the_pappus_closed_form() {
    let b = washer();
    let r = 0.2;
    // (rim, material at larger radii?, sign)
    let bore_b = ((1.0, 0.0), true, -1.0);
    let out_b = ((2.0, 0.0), false, -1.0);
    let bore_t = ((1.0, 1.0), true, -1.0);
    let out_t = ((2.0, 1.0), false, -1.0);
    for (what, rims) in [
        ("washer bore bottom", vec![bore_b]),
        ("washer outer bottom", vec![out_b]),
        ("washer bore top", vec![bore_t]),
        ("washer outer top", vec![out_t]),
        (
            "washer both rims of the bottom annulus",
            vec![bore_b, out_b],
        ),
        ("washer both bores", vec![bore_b, bore_t]),
        ("washer all four rims", vec![bore_b, out_b, bore_t, out_t]),
    ] {
        rims_carve(what, &b, &rims, r).unwrap_or_else(|e| panic!("{what}: refused {e:?}"));
    }
    // Both rims of one annulus at a radius whose trims overlap: the
    // annulus is 1 wide, so two 0.55 trims cross. Must refuse typed.
    let e = rims_carve("washer bottom, crossing trims", &b, &[bore_b, out_b], 0.55)
        .expect_err("crossing trims on one annulus");
    let BlendError::FaceClearanceUncertified {
        margin,
        cross_chain: true,
        ..
    } = e
    else {
        panic!("crossing trims refuse at the cross-chain screen: {e:?}")
    };
    let read = reading(&margin);
    assert!(
        (read - (1.0 - 2.0 * 0.55)).abs() < 1e-12,
        "the trims overlap by the annulus width less both setbacks, read {read}"
    );
}

#[test]
fn each_flange_rim_carves_at_the_pappus_closed_form() {
    let b = flange();
    let r = 0.2;
    let bottom_bore = ((1.0, 0.0), true, -1.0);
    let bottom_out = ((3.0, 0.0), false, -1.0);
    let step_out = ((3.0, 0.5), false, -1.0);
    // The concave rim: hub wall r=2 meets the step annulus y=0.5; the
    // fillet ADDS material at radii above 2.
    let step_in = ((2.0, 0.5), true, 1.0);
    let hub_top = ((2.0, 2.0), false, -1.0);
    let bore_top = ((1.0, 2.0), true, -1.0);
    for (what, rims) in [
        ("flange bottom bore", vec![bottom_bore]),
        ("flange bottom outer", vec![bottom_out]),
        ("flange step outer", vec![step_out]),
        (
            "flange step inner (concave, the annulus's ring)",
            vec![step_in],
        ),
        (
            "flange step annulus both rims (convex outer + concave ring)",
            vec![step_out, step_in],
        ),
        ("flange hub top", vec![hub_top]),
        ("flange top bore", vec![bore_top]),
        ("flange hub top + top bore", vec![hub_top, bore_top]),
        (
            "flange every rim",
            vec![
                bottom_bore,
                bottom_out,
                step_out,
                step_in,
                hub_top,
                bore_top,
            ],
        ),
    ] {
        rims_carve(what, &b, &rims, r).unwrap_or_else(|e| panic!("{what}: refused {e:?}"));
    }
}

#[test]
fn a_two_annuli_laminas_outer_and_ring_hosted_rims_carve() {
    // "Stepped both ways": three plane annuli at y=1 (two of them,
    // disjoint, inner r∈[1,2] and outer r∈[3,4]) and the bottom.
    let b = revolved_about_y(
        vec![
            v(1., 0.),
            v(4., 0.),
            v(4., 1.),
            v(3., 1.),
            v(3., 2.),
            v(2., 2.),
            v(2., 1.),
            v(1., 1.),
        ],
        Revolution::Full,
        tol(),
    );
    let r = 0.15;
    for (what, rims) in [
        (
            "inner annulus ring (concave at r=2)",
            vec![((2.0, 1.0), false, 1.0)],
        ),
        (
            "outer annulus ring (concave at r=3)",
            vec![((3.0, 1.0), true, 1.0)],
        ),
        (
            "inner annulus outer cycle (convex r=1)",
            vec![((1.0, 1.0), true, -1.0)],
        ),
        (
            "outer annulus outer cycle (convex r=4)",
            vec![((4.0, 1.0), false, -1.0)],
        ),
        (
            "both y=1 annuli, all four rims",
            vec![
                ((1.0, 1.0), true, -1.0),
                ((2.0, 1.0), false, 1.0),
                ((3.0, 1.0), true, 1.0),
                ((4.0, 1.0), false, -1.0),
            ],
        ),
    ] {
        rims_carve(what, &b, &rims, r).unwrap_or_else(|e| panic!("{what}: refused {e:?}"));
    }
}

#[test]
fn a_washer_rim_chamfer_refuses_typed() {
    let b = washer();
    let e = rim_arcs_at(&b, 1.0, 0.0);
    let err = chamfer_edges(&sweep::test_support::at_rest(&b), &e, 0.2, tol())
        .expect_err("plane × cylinder")
        .error;
    assert!(
        matches!(err, BlendError::ChamferArmUnsupported { .. }),
        "{err:?}"
    );
}

#[test]
fn an_interval_washers_rims_fillet_tier_3_valid() {
    use geom_core::{Interval, Real};
    let p = |x: f64, y: f64| {
        (
            Point2::new(Interval::from_f64(x), Interval::from_f64(y)),
            Interval::from_f64(0.0),
        )
    };
    let b = sweep::test_support::revolved_about_y_at(
        vec![p(1., 0.), p(2., 0.), p(2., 1.), p(1., 1.)],
        Revolution::Full,
        tol(),
    );
    for (rr, y) in [(1.0, 0.0), (2.0, 1.0)] {
        let e = rim_arcs_at(&b, rr, y);
        match fillet_edges(
            &sweep::test_support::at_rest(&b),
            &e,
            Interval::from_f64(0.2),
            tol(),
        ) {
            Ok(out) => {
                assert_eq!(
                    validate_geometric(&out.body, tol()),
                    Ok(()),
                    "interval ({rr},{y})"
                );
            }
            Err(e) => panic!("interval washer rim ({rr},{y}): refused {:?}", e.error),
        }
    }
}

// ---------------------------------------------------------------------
// Ruled bands on plane supports and caps carrying the bore.
// ---------------------------------------------------------------------

fn moved(b: &Body<f64>, m: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, m, tol()).expect("a rigid motion")
}

fn upright(pts: &[(f64, f64)]) -> Body<f64> {
    let loop_: Vec<_> = pts.iter().map(|&(x, z)| (Point2::new(z, x), 0.0)).collect();
    let block = prism_on(SketchPlane::zx(), loop_, 2.0, tol());
    moved(&block, &Affine3::translation(Vec3::new(0.0, -0.5, 0.0)))
}

fn ends(body: &Body<f64>, e: EdgeKey) -> [Vec3<f64>; 2] {
    let he = body.get_edge(e).unwrap().he_plus;
    let at = |v| {
        let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        Vec3::new(p.x, p.y, p.z)
    };
    [
        at(body.get_half_edge(he).unwrap().start),
        at(body.half_edge_end(he).unwrap()),
    ]
}

fn edges_where(body: &Body<f64>, on: impl Fn(Vec3<f64>) -> bool) -> Vec<EdgeKey> {
    body.edges()
        .map(|(k, _)| k)
        .filter(|&k| {
            let [a, b] = ends(body, k);
            on(a) && on(b) && (a - b).norm() > 1e-6
        })
        .collect()
}

/// A washer `r ∈ [ri, 3]`, `y ∈ [0, 1]`, squared at half-side 2.1 about
/// azimuth 90°, with a rod (radius ρ, axis on `y = 0` along `z`, near
/// ruling at `x = near`) unioned under its bottom face and cut off at
/// `z = ±2`. The bottom face is ONE plane face carrying the bore as a
/// ring and the rod's rulings in its outer cycle.
fn rodded(ri: f64, near: f64, rho: f64) -> Body<f64> {
    let washer = revolved_about_y(
        vec![v(ri, 0.), v(3., 0.), v(3., 1.), v(ri, 1.)],
        Revolution::Full,
        tol(),
    );
    let s = 2.1;
    let square = upright(&[(-s, -s), (s, -s), (s, s), (-s, s)]);
    let squared = realized(BooleanOp::Intersect, &washer, &square, tol());
    let cx = near + rho;
    let rod = prism_at(
        vec![
            (Point2::new(cx - rho, 0.0), 1.0),
            (Point2::new(cx + rho, 0.0), 1.0),
        ],
        -4.0,
        8.0,
        tol(),
    );
    // Sketch xy extruded along z; the rod's axis is (cx, 0, ·).
    let joined = realized(BooleanOp::Union, &squared, &rod, tol());
    let caps = upright(&[(-5.0, -2.0), (5.0, -2.0), (5.0, 2.0), (-5.0, 2.0)]);
    realized(BooleanOp::Intersect, &joined, &caps, tol())
}

/// The concave plane–cylinder fillet's added area: rod axis on the
/// plane, ball tangent to both.
fn rod_fillet_area(rho: f64, r: f64) -> f64 {
    let h = ((rho + r).powi(2) - r * r).sqrt();
    let alpha = (r / (rho + r)).acos();
    let beta = (r / (rho + r)).asin();
    h * r / 2.0 - alpha * r * r / 2.0 - beta * rho * rho / 2.0
}

#[test]
fn a_ruled_band_on_a_plane_support_carrying_the_bore_carves_at_the_closed_form() {
    let (ri, rho) = (1.0, 0.34);
    let mut carved = 0;
    for (near, r) in [(1.4, 0.15), (1.4, 0.2), (1.2, 0.1), (1.12, 0.1), (1.1, 0.2)] {
        let body = rodded(ri, near, rho);
        validate_geometric(&body, tol()).expect("fixture tier 3");
        let bottom_rings: usize = body
            .faces()
            .filter(|(_, f)| match body.get_surface(f.surface) {
                Some(Surface::Plane { origin, normal, .. }) => {
                    origin.y.abs() < 1e-12 && normal.y.abs() > 0.5
                }
                _ => false,
            })
            .map(|(_, f)| f.rings.len())
            .sum();
        assert_eq!(
            bottom_rings, 1,
            "the bottom face carries the bore as a ring"
        );
        let near_ruling = edges_where(&body, |p| p.y.abs() < 1e-9 && (p.x - near).abs() < 1e-9);
        assert_eq!(near_ruling.len(), 1, "one near ruling");
        let setback = ((rho + r).powi(2) - r * r).sqrt() - rho;
        let clearance = near - setback - ri;
        let v0 = volume(&body);
        match fillet_edges(&sweep::test_support::at_rest(&body), &near_ruling, r, tol()) {
            Ok(out) => {
                validate_geometric(&out.body, tol())
                    .unwrap_or_else(|e| panic!("near {near} r {r}: tier 3 {e:?}"));
                let got = volume(&out.body) - v0;
                let want = rod_fillet_area(rho, r) * 4.0;
                assert!(
                    clearance > 0.0,
                    "carved through a negative clearance {clearance}"
                );
                assert!((got - want).abs() < 1e-9, "ΔV {got} vs {want}");
                carved += 1;
            }
            Err(e) => {
                assert!(
                    clearance < 0.0,
                    "refused a clear trimline {clearance}: {e:?}"
                );
            }
        }
    }
    assert_eq!(carved, 4, "every pose with a clear trimline carves");
}

/// A washer `r ∈ [ri, 2]` with a flat at `x = c`: the flat meets the
/// outer cylinder along two vertical rulings, each a ruled link whose
/// transverse caps are the two unslit annuli (each an annulus with a
/// flat, carrying the bore as a ring).
fn flatted(ri: f64, c: f64) -> Body<f64> {
    let washer = revolved_about_y(
        vec![v(ri, 0.), v(2., 0.), v(2., 1.), v(ri, 1.)],
        Revolution::Full,
        tol(),
    );
    let half = upright(&[(-3.0, -3.0), (c, -3.0), (c, 3.0), (-3.0, 3.0)]);
    realized(BooleanOp::Intersect, &washer, &half, tol())
}

/// Removed area at a convex corner between the line `x = c` (material
/// `x < c`) and the circle `R` (material inside), fillet radius `r`.
fn flat_corner_area(big: f64, c: f64, r: f64) -> f64 {
    let qx = c - r;
    let qy = ((big - r).powi(2) - qx * qx).sqrt();
    let k = (c, (big * big - c * c).sqrt());
    let t = (c, qy);
    let th_p = qy.atan2(qx);
    let th_k = k.1.atan2(k.0);
    // Green: ½∮(x dy − y dx) over K→T (segment), T→P (fillet arc about
    // Q from 0 to θp), P→K (big arc from θp to θk).
    let seg = |a: (f64, f64), b: (f64, f64)| 0.5 * (a.0 * b.1 - b.0 * a.1);
    let arc = |cx: f64, cy: f64, rr: f64, t1: f64, t2: f64| {
        0.5 * (rr * rr * (t2 - t1) + cx * rr * (t2.sin() - t1.sin())
            - cy * rr * (t2.cos() - t1.cos()))
    };
    (seg(k, t) + arc(qx, qy, r, 0.0, th_p) + arc(0.0, 0.0, big, th_p, th_k)).abs()
}

#[test]
fn a_ruled_band_whose_caps_are_unslit_annuli_carves_at_the_closed_form() {
    for (ri, c, r) in [
        (1.0, 1.8, 0.1),
        (1.0, 1.8, 0.2),
        (1.5, 1.6, 0.1),
        (1.5, 1.6, 0.3),
        (1.5, 1.55, 0.3),
    ] {
        let body = flatted(ri, c);
        validate_geometric(&body, tol()).expect("fixture tier 3");
        let creases: Vec<EdgeKey> = edges_where(&body, |p| {
            (p.x - c).abs() < 1e-9 && (p.x.hypot(p.z) - 2.0).abs() < 1e-9
        })
        .into_iter()
        .filter(|&e| {
            let [a, b] = ends(&body, e);
            (a.y - b.y).abs() > 0.5
        })
        .collect();
        assert_eq!(creases.len(), 2, "two vertical rulings");
        let v0 = volume(&body);
        match fillet_edges(&sweep::test_support::at_rest(&body), &creases, r, tol()) {
            Ok(out) => {
                validate_geometric(&out.body, tol())
                    .unwrap_or_else(|e| panic!("ri {ri} c {c} r {r}: tier 3 {e:?}"));
                let got = v0 - volume(&out.body);
                let want = 2.0 * flat_corner_area(2.0, c, r);
                assert!((got - want).abs() < 1e-9, "removed {got} vs {want}");
            }
            Err(e) => panic!("ri {ri} c {c} r {r}: refused {:?}", e.error),
        }
    }
}

fn plane_chart_at_y(body: &Body<f64>, y: f64) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

fn anchor(body: &Body<f64>, lp: topo::LoopKey) -> topo::HalfEdgeKey {
    match body.get_loop(lp).unwrap().boundary {
        topo::LoopBoundary::Cycle { first } => first,
        ref other => panic!("a cycle, got {other:?}"),
    }
}

/// A block standing on the `zx` sketch from `y = −0.5` to `1.5` over a
/// bulged loop given as world `(x, z, bulge)`.
fn upright_bulged(pts: &[(f64, f64, f64)]) -> Body<f64> {
    let loop_: Vec<_> = pts
        .iter()
        .map(|&(x, z, b)| (Point2::new(z, x), b))
        .collect();
    let block = prism_on(SketchPlane::zx(), loop_, 2.0, tol());
    moved(&block, &Affine3::translation(Vec3::new(0.0, -0.5, 0.0)))
}

/// **A non-circle ring on the outer rim's plane host**, metered edge by
/// edge (`support_boundary_clearance`). The washer `[1, 2] × [0, 1]` is
/// notched at its bore, so each annulus's ring is the bore arc plus the
/// notch's edges. The outer rim's trim circle at `2 − r` must clear the
/// notch's farthest reach. Box notch: the reach is a CORNER (a vertex).
/// Round notch: the reach is the INTERIOR of an arc whose ends sit on
/// the bore at radius ~1 — a vertex-only reader would pass it.
#[test]
fn a_notched_bore_ring_is_metered_edge_by_edge_against_the_outer_rims_trim() {
    let washer = washer();
    let boxy = upright(&[(0.5, -0.2), (1.3, -0.2), (1.3, 0.2), (0.5, 0.2)]);
    let round_at = |az: f64| {
        let (c, s) = (az.to_radians().cos(), az.to_radians().sin());
        upright_bulged(&[(0.8 * c, 0.8 * s, 1.0), (1.4 * c, 1.4 * s, 1.0)])
    };
    let (mut carved, mut metered) = (0, 0);
    for (what, tool, reach) in [
        ("box notch", boxy, 1.3f64.hypot(0.2)),
        ("round notch", round_at(0.0), 1.4),
        ("round notch off-station 22.5", round_at(22.5), 1.4),
        ("round notch off-station 10", round_at(10.0), 1.4),
    ] {
        let body = realized(BooleanOp::Subtract, &washer, &tool, tol());
        validate_geometric(&body, tol()).expect("fixture tier 3");
        let bottom = plane_chart_at_y(&body, 0.0);
        let rings: Vec<usize> = bottom
            .iter()
            .map(|&f| body.get_face(f).unwrap().rings.len())
            .collect();
        assert_eq!(
            rings,
            [1],
            "{what}: the notched bore is the bottom's one ring"
        );
        let rim = rim_arcs_at(&body, 2.0, 0.0);
        assert_eq!(rim.len(), 1);
        let v0 = volume(&body);
        for r in [
            reach - 0.02,
            reach - 0.005,
            2.0 - reach - 0.01,
            2.0 - reach + 0.01,
            2.0 - reach - 0.002,
            2.0 - reach + 0.002,
        ] {
            if r <= 0.0 || r >= 0.95 {
                continue;
            }
            let clear = (2.0 - r) - reach;
            match fillet_edges(&sweep::test_support::at_rest(&body), &rim, r, tol()) {
                Ok(out) => {
                    carved += 1;
                    let ok = validate_geometric(&out.body, tol());
                    let got = volume(&out.body) - v0;
                    let want = -rim_spandrel(2.0, r, false);
                    assert!(
                        clear > 0.0,
                        "{what} r {r}: carved through the notch ({clear})"
                    );
                    assert_eq!(ok, Ok(()), "{what} r {r}");
                    assert!(
                        (got - want).abs() < 1e-9,
                        "{what} r {r}: ΔV {got} vs {want}"
                    );
                }
                Err(e) => {
                    assert!(clear < 0.0, "{what} r {r}: refused a clear trim ({clear})");
                    // The meter reads the notch's deepest point; the
                    // battery's screen may answer first instead.
                    if let BlendError::RingClearance { margin, .. } = e.error {
                        let read = reading(&margin);
                        assert!(
                            (read - clear).abs() < 1e-9,
                            "{what} r {r}: the refusal reads the notch's reach, {read} vs {clear}"
                        );
                        metered += 1;
                    }
                }
            }
        }
    }
    assert!(carved > 0 && metered > 0, "both sides of zero are read");
}

/// **The ring case of the one-face host with a second ring.** The
/// flange's step annulus `r ∈ [2, 3]`, `y = 0.5` carries the hub rim as
/// its ring; a through-hole in the flange plate puts a second ring on
/// it (and on the bottom annulus). The concave ring rim's trim at
/// `2 + r` and the outer rim's at `3 − r` must each clear the hole.
#[test]
fn a_second_ring_beside_a_ring_hosted_rim_is_metered_at_its_deepest_edge() {
    let f = flange();
    // A square hole spanning radius 2.3..2.7 on the +x side, rotated
    // 20° off the stations; a round one of radius 0.2 about radius 2.5.
    let az = 20f64.to_radians();
    let (c, s) = (az.cos(), az.sin());
    let rot = |u: f64, w: f64| (u * c - w * s, u * s + w * c);
    let sq = upright(&[
        rot(2.3, -0.15),
        rot(2.7, -0.15),
        rot(2.7, 0.15),
        rot(2.3, 0.15),
    ]);
    let rd = upright_bulged(&[(2.3 * c, 2.3 * s, 1.0), (2.7 * c, 2.7 * s, 1.0)]);
    // The square's nearest point to the axis is its inner edge's
    // midpoint; its farthest, an outer corner.
    let (sq_in, sq_out) = (2.3, 2.7f64.hypot(0.15));
    let mut wholly_inside_read = false;
    for (what, tool, inner, outer) in [
        ("square hole", sq, sq_in, sq_out),
        ("round hole", rd, 2.3, 2.7),
    ] {
        let body = realized(BooleanOp::Subtract, &f, &tool, tol());
        validate_geometric(&body, tol()).expect("fixture tier 3");
        let v0 = volume(&body);
        for (rim, gap_of, sign, outward) in [
            (
                (2.0, 0.5),
                Box::new(move |r: f64| inner - (2.0 + r)) as Box<dyn Fn(f64) -> f64>,
                1.0,
                true,
            ),
            (
                (3.0, 0.5),
                Box::new(move |r: f64| (3.0 - r) - outer),
                -1.0,
                false,
            ),
        ] {
            let e = rim_arcs_at(&body, rim.0, rim.1);
            assert_eq!(e.len(), 1);
            for r in [0.2, 0.28, 0.295, 0.302, 0.304, 0.305, 0.32] {
                let gap = gap_of(r);
                match fillet_edges(&sweep::test_support::at_rest(&body), &e, r, tol()) {
                    Ok(out) => {
                        let ok = validate_geometric(&out.body, tol());
                        let got = volume(&out.body) - v0;
                        let want = sign * rim_spandrel(rim.0, r, outward);
                        assert!(gap > 0.0, "{what} {rim:?} r {r}: carved through {gap}");
                        assert_eq!(ok, Ok(()));
                        assert!((got - want).abs() < 1e-9);
                    }
                    Err(err) => {
                        assert!(gap < 0.0, "{what} {rim:?} r {r}: refused a clear gap {gap}");
                        // The refusal carries the hole's DEEPEST reach
                        // past the trim, not the first edge's: at r =
                        // 0.305 the square's inner edge lies wholly in
                        // the strip (2.3 against a trim at 2.305) while
                        // its side edges reach only 2.30489.
                        if let BlendError::RingClearance { margin, .. } = err.error {
                            let read = reading(&margin);
                            assert!(
                                (read - gap).abs() < 1e-12,
                                "{what} {rim:?} r {r}: margin {read} vs the deepest gap {gap}"
                            );
                            wholly_inside_read |=
                                what == "square hole" && rim.0 == 2.0 && r == 0.305;
                        }
                    }
                }
            }
        }
    }
    assert!(
        wholly_inside_read,
        "the square hole's inner edge, wholly inside the strip, is metered"
    );
}

// ---------------------------------------------------------------------
// Offset and shell on unslit annuli.
// ---------------------------------------------------------------------

fn pappus(l: &[(f64, f64)]) -> f64 {
    let n = l.len();
    let mut mx = 0.0;
    for i in 0..n {
        let (p, q) = (l[i], l[(i + 1) % n]);
        let cr = p.0 * q.1 - q.0 * p.1;
        mx += (p.0 + q.0) * cr / 6.0;
    }
    2.0 * PI * mx.abs()
}

#[test]
fn shell_and_offset_of_unslit_annuli_match_closed_forms() {
    let t = 0.1;
    let w = washer();
    let fl = flange();
    let wp = [(1., 0.), (2., 0.), (2., 1.), (1., 1.)];
    let fp = [(1., 0.), (3., 0.), (3., 0.5), (2., 0.5), (2., 2.), (1., 2.)];
    let inset_w = [(1. + t, t), (2. - t, t), (2. - t, 1. - t), (1. + t, 1. - t)];
    let inset_f = [
        (1. + t, t),
        (3. - t, t),
        (3. - t, 0.5 - t),
        (2. - t, 0.5 - t),
        (2. - t, 2. - t),
        (1. + t, 2. - t),
    ];
    for (what, body, outer, inner) in [
        ("washer", &w, &wp[..], &inset_w[..]),
        ("flange", &fl, &fp[..], &inset_f[..]),
    ] {
        let s = topo::shell(&finished(what, body.clone(), tol()), t, tol())
            .expect("sealed shell")
            .body;
        assert_eq!(validate_geometric(&s, tol()), Ok(()), "{what} sealed");
        let got = volume(&s);
        let want = pappus(outer) - pappus(inner);
        assert!((got - want).abs() < 1e-9);
    }
    // Open the washer's top annulus: a cup whose cavity is the inset
    // washer extended through the top.
    let top = plane_chart_at_y(&w, 1.0);
    let cup = topo::shell_open(&finished("the washer", w.clone(), tol()), t, &top, tol())
        .expect("open")
        .body;
    assert_eq!(validate_geometric(&cup, tol()), Ok(()), "washer cup");
    let want = pappus(&wp) - pappus(&[(1. + t, t), (2. - t, t), (2. - t, 1.), (1. + t, 1.)]);
    let got = volume(&cup);
    assert!((got - want).abs() < 1e-9);
    // Open the flange's step annulus (its hole is the hub wall).
    let step = plane_chart_at_y(&fl, 0.5);
    // Its counterpart's bore meets the hub wall's own: not one region.
    let refused = topo::shell_open(&finished("the flange", fl.clone(), tol()), t, &step, tol())
        .map(|_| ())
        .expect_err("the step annulus's rim is not expressible");
    assert!(
        matches!(refused, topo::ShellError::OpenFaceRimNotExpressible { .. }),
        "flange step: {refused:?}"
    );
    // Open the flange's bottom annulus.
    let bottom = plane_chart_at_y(&fl, 0.0);
    match topo::shell_open(
        &finished("the flange", fl.clone(), tol()),
        t,
        &bottom,
        tol(),
    ) {
        Ok(cup) => {
            let cup = cup.body;
            assert_eq!(validate_geometric(&cup, tol()), Ok(()), "flange bottom cup");
            let inner = [
                (1. + t, 0.),
                (3. - t, 0.),
                (3. - t, 0.5 - t),
                (2. - t, 0.5 - t),
                (2. - t, 2. - t),
                (1. + t, 2. - t),
            ];
            let want = pappus(&fp) - pappus(&inner);
            let got = volume(&cup);
            assert!((got - want).abs() < 1e-9);
        }
        Err(e) => panic!("flange cup (bottom open): refused {e:?}"),
    }
    // Offsets of each annulus.
    for (what, body, y, d, dv) in [
        ("washer top +0.1", &w, 1.0, 0.1, PI * 3.0 * 0.1),
        ("washer bottom +0.1", &w, 0.0, 0.1, PI * 3.0 * 0.1),
        ("flange step +0.2", &fl, 0.5, 0.2, PI * 5.0 * 0.2),
        ("flange step -0.2", &fl, 0.5, -0.2, -PI * 5.0 * 0.2),
        ("flange top +0.1", &fl, 2.0, 0.1, PI * 3.0 * 0.1),
    ] {
        let mut b = body.clone();
        let f = plane_chart_at_y(&b, y);
        assert_eq!(f.len(), 1);
        let v0 = volume(&b);
        topo::replace_face_offset(&mut b, f[0], d, tol())
            .unwrap_or_else(|e| panic!("{what}: {e:?}"));
        assert_eq!(validate_geometric(&b, tol()), Ok(()), "{what}");
        let got = volume(&b) - v0;
        assert!(
            (got.abs() - dv.abs()).abs() < 1e-9,
            "{what}: |ΔV| (sign is the stored normal's)"
        );
    }
}

/// The lone host trim keeps the host's key, its outer/ring designation
/// and its other rings, and lands the trim on the material side: a
/// ring rim's trim is the new ring at `1 + r`, an outer rim's the new
/// outer cycle at `2 − r`.
#[test]
fn the_lone_host_trim_keeps_the_hosts_key_and_designations() {
    let w = washer();
    let bottom = plane_chart_at_y(&w, 0.0)[0];
    let fd0 = w.get_face(bottom).unwrap().clone();
    for (rr, which) in [(1.0, "ring"), (2.0, "outer")] {
        let e = rim_arcs_at(&w, rr, 0.0);
        let out = fillet_edges(&sweep::test_support::at_rest(&w), &e, 0.2, tol())
            .unwrap()
            .body;
        let fd = out
            .get_face(bottom)
            .expect("the host keeps its key")
            .clone();
        let radii = |lp: topo::LoopKey| -> Vec<f64> {
            let first = anchor(&out, lp);
            out.loop_cycle(first)
                .unwrap()
                .iter()
                .map(|&h| {
                    let ek = out.get_half_edge(h).unwrap().edge;
                    let c = out
                        .get_curve_geom(out.get_edge(ek).unwrap().curve)
                        .unwrap()
                        .certified()
                        .unwrap();
                    match *c.carrier() {
                        geom::Curve3::Circle { radius, .. } => radius,
                        _ => f64::NAN,
                    }
                })
                .collect()
        };
        assert_eq!(fd.rings.len(), 1);
        if which == "ring" {
            assert_eq!(fd.outer, fd0.outer, "outer designation kept");
            assert_eq!(radii(fd.outer), vec![2.0]);
            assert_eq!(radii(fd.rings[0]), vec![1.2]);
        } else {
            assert_eq!(fd.rings, fd0.rings, "the bore ring kept");
            assert_eq!(radii(fd.rings[0]), vec![1.0]);
            assert!((radii(fd.outer)[0] - 1.8).abs() < 1e-15);
        }
    }
}

/// **The squared washer's twelve square edges plus its bottom bore rim
/// carve in one call.** Closed form: the rounded box
/// (Steiner: `xyz + 2(xy+yz+zx)r + π(x+y+z)r² + 4πr³/3` on the inner
/// box) less the bore cylinder less the bore rim's spandrel ring.
#[test]
fn the_squared_washers_thirteen_edges_carve_at_the_closed_form() {
    let washer = revolved_about_y(
        vec![v(1., 0.), v(3., 0.), v(3., 1.), v(1., 1.)],
        Revolution::Full,
        tol(),
    );
    let half = 2.1 * 2f64.sqrt();
    let polar = |az: f64| {
        let a = az.to_radians();
        (half * a.cos(), half * a.sin())
    };
    let square = upright(&[polar(67.5), polar(157.5), polar(247.5), polar(337.5)]);
    let body = realized(BooleanOp::Intersect, &washer, &square, tol());
    let on_square = |p: Vec3<f64>| p.x.hypot(p.z) > 2.05;
    let mut edges = rim_arcs_at(&body, 1.0, 0.0);
    edges.extend(edges_where(&body, on_square));
    assert_eq!(edges.len(), 13);
    let r = 0.2;
    let v0 = volume(&body);
    let want0 = 4.2 * 4.2 * 1.0 - PI;
    assert!((v0 - want0).abs() < 1e-9, "fixture {v0} vs {want0}");
    let out = fillet_edges(&sweep::test_support::at_rest(&body), &edges, r, tol())
        .expect("carves")
        .body;
    assert_eq!(validate_geometric(&out, tol()), Ok(()));
    let (x, y, z) = (4.2 - 2.0 * r, 4.2 - 2.0 * r, 1.0 - 2.0 * r);
    let rounded = x * y * z
        + 2.0 * (x * y + y * z + z * x) * r
        + PI * (x + y + z) * r * r
        + 4.0 * PI * r.powi(3) / 3.0;
    let want = rounded - PI - rim_spandrel(1.0, r, true);
    let got = volume(&out);
    assert!((got - want).abs() < 1e-9);
}

/// Volume a convex fillet of radius `r` removes at a profile corner `k`
/// whose two edges leave along unit `d1`, `d2` (both into the
/// material's boundary), revolved about `y`: Pappus over the kite
/// (corner, two tangent points, centre) less the circular sector.
fn corner_ring_volume(k: (f64, f64), d1: (f64, f64), d2: (f64, f64), r: f64) -> f64 {
    let n = |v: (f64, f64)| {
        let l = v.0.hypot(v.1);
        (v.0 / l, v.1 / l)
    };
    let (d1, d2) = (n(d1), n(d2));
    let theta = (d1.0 * d2.0 + d1.1 * d2.1).acos();
    let t = r / (theta / 2.0).tan();
    let bis = n((d1.0 + d2.0, d1.1 + d2.1));
    let cdist = r / (theta / 2.0).sin();
    let t1 = (k.0 + d1.0 * t, k.1 + d1.1 * t);
    let t2 = (k.0 + d2.0 * t, k.1 + d2.1 * t);
    let c = (k.0 + bis.0 * cdist, k.1 + bis.1 * cdist);
    // Triangles' (area, x-moment).
    let tri = |a: (f64, f64), b: (f64, f64), cc: (f64, f64)| {
        let ar = 0.5 * ((b.0 - a.0) * (cc.1 - a.1) - (cc.0 - a.0) * (b.1 - a.1)).abs();
        (ar, ar * (a.0 + b.0 + cc.0) / 3.0)
    };
    let (_, m1) = tri(k, t1, c);
    let (_, m2) = tri(k, c, t2);
    let alpha = PI - theta;
    let sa = alpha / 2.0 * r * r;
    let sd = 4.0 * r * (alpha / 2.0).sin() / (3.0 * alpha);
    let sx = c.0 - bis.0 * sd;
    let m = m1 + m2 - sa * sx;
    2.0 * PI * m
}

#[test]
fn cone_and_plane_annulus_rims_carve_at_the_closed_form() {
    let b = revolved_about_y(
        vec![v(1., 0.), v(3., 0.), v(2.5, 1.), v(1.5, 1.)],
        Revolution::Full,
        tol(),
    );
    // sanity: the general form reproduces the right-angle spandrel
    let chk = corner_ring_volume((2.0, 0.0), (-1.0, 0.0), (0.0, 1.0), 0.2);
    assert!((chk - rim_spandrel(2.0, 0.2, false)).abs() < 1e-12, "{chk}");
    let v0 = volume(&b);
    let r = 0.15;
    for (k, d1, d2) in [
        ((1.0, 0.0), (1.0, 0.0), (0.5, 1.0)),
        ((3.0, 0.0), (-1.0, 0.0), (-0.5, 1.0)),
        ((2.5, 1.0), (-1.0, 0.0), (0.5, -1.0)),
        ((1.5, 1.0), (1.0, 0.0), (-0.5, -1.0)),
    ] {
        let e = rim_arcs_at(&b, k.0, k.1);
        let out = fillet_edges(&sweep::test_support::at_rest(&b), &e, r, tol())
            .unwrap_or_else(|e| panic!("{k:?}: {e:?}"))
            .body;
        assert_eq!(validate_geometric(&out, tol()), Ok(()), "{k:?}");
        let got = v0 - volume(&out);
        let want = corner_ring_volume(k, d1, d2, r);
        assert!((got - want).abs() < 1e-9);
    }
}
