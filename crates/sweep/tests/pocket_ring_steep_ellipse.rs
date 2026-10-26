//! **Conic germ lines on a steep ellipse.** The join pairs each germ with
//! its nearest partner along the section conic. On an ellipse of aspect
//! √2 or more the chord from a site near a minor end peaks inside the
//! half-turn the germ runs into and falls again, so the nearest site by
//! chord can lie past the true partner. Two shapes reach that regime:
//!
//! - a cylinder (r = 1) against a convex quad prism whose cap lies in the
//!   plane `z = k·x`, turned `ψ` about the axis; each long edge of the
//!   quad cuts the cap's ellipse `(cos φ, sin φ, k cos φ)` twice. Oracle:
//!   cylinder ∩ prism in closed form, each prism face's height
//!   integrated over the disc against its footprint
//!   ([`prism_in_cylinder`]).
//! - a thin plate pierced by a round rod tilted `θ` from `z`, whose caps
//!   cut the rod's wall in ellipses of aspect `1/cos θ`. Oracle: each
//!   slice, scaled by `cos θ` along `x`, is a disc against a rectangle,
//!   integrated in `z` by Simpson.
//!
//! The batteries are `#[ignore]`d and print one line per run, to diff two
//! trees; the gating row holds the poses the chord order paired wrongly.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{Body, BooleanError, BooleanResult};

use crate::common::differential::{area, ccw, disc_clip_area, disc_clip_moments, outcome};

fn tol() -> Tol {
    Tol::witness()
}

/// `l op r` through the door (`U`, `S`, or `I` for anything else), each
/// operand finished first.
fn door(op: &str, l: &Body<f64>, r: &Body<f64>) -> Result<BooleanResult<f64>, BooleanError> {
    let (l, r) = (
        finished("the left operand", l.clone(), tol()),
        finished("the right operand", r.clone(), tol()),
    );
    match op {
        "U" => topo::union(&l, &r, tol()),
        "S" => topo::subtract(&l, &r, tol()),
        _ => topo::intersect(&l, &r, tol()),
    }
}

fn cylinder(r: f64, z0: f64, h: f64) -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(r, 0.0), 1.0),
        (Point2::new(-r, 0.0), 1.0),
    ]);
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let profile = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

/// The ellipse point at azimuth `deg` in the sketch plane's own (u, v).
fn uv(k: f64, deg: f64) -> (f64, f64) {
    let p = deg.to_radians();
    ((1.0 + k * k).sqrt() * p.cos(), p.sin())
}

/// u on the line through a and b at height v.
fn at_v(a: (f64, f64), b: (f64, f64), v: f64) -> f64 {
    a.0 + (b.0 - a.0) * (v - a.1) / (b.1 - a.1)
}

/// The prism's convex quad in its cap plane's own `(u, v)`: each long
/// edge through its two ellipse sites ([`uv`]), cut at `v = ±cap`.
fn quad(k: f64, sites: [f64; 4], cap: f64) -> [(f64, f64); 4] {
    let [e1a, e1b, e2a, e2b] = sites.map(|d| uv(k, d));
    [
        (at_v(e1a, e1b, -cap), -cap),
        (at_v(e2a, e2b, -cap), -cap),
        (at_v(e2a, e2b, cap), cap),
        (at_v(e1a, e1b, cap), cap),
    ]
}

/// The prism: a convex quad in the plane `z = k x` (local u along
/// `(1, 0, k)/√(1+k²)`, v along y), extruded `h` along the plane's
/// normal (`side`) and turned `psi` about `z`. `sites` = (edge-1 φ
/// pair, edge-2 φ pair), degrees.
fn prism_turned(
    k: f64,
    sites: [f64; 4],
    cap: f64,
    h: f64,
    side: ExtrudeSide,
    psi: f64,
) -> Body<f64> {
    let quad = quad(k, sites, cap);
    let lp = bulge_loop(
        quad.iter()
            .map(|&(u, v)| (Point2::new(u, v), 0.0))
            .collect(),
    );
    let beta = -(k.atan());
    let rot = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), beta);
    let x = rot.transform_vec(Vec3::new(1.0, 0.0, 0.0));
    assert!(
        (x.z / x.x - k).abs() < 1e-12,
        "local u runs along (1, 0, k)"
    );
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), psi);
    let profile = Profile::new(
        SketchPlane::from_frame(geom_core::test_support::rigid(
            &(turn * rot),
            geom_core::Tol::witness(),
        )),
        vec![lp],
    )
    .validate(tol())
    .unwrap();
    extrude(&profile, Extrusion::Distance { depth: h, side }, tol())
        .unwrap()
        .body
}

/// The prism's corners, as [`prism_turned`] builds it: the quad in the
/// cap plane, then the same quad moved `h` along the extrusion.
fn prism_corners(k: f64, sites: [f64; 4], cap: f64, h: f64, along: bool, psi: f64) -> [V3; 8] {
    let s = 1.0 / (1.0 + k * k).sqrt();
    let n = Vec3::new(-k, 0.0, 1.0) * (s * if along { h } else { -h });
    let ux = Vec3::new(1.0, 0.0, k) * s;
    let vy = Vec3::new(0.0, 1.0, 0.0);
    let quad = quad(k, sites, cap);
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), psi);
    core::array::from_fn(|i| {
        let (u, v) = quad[i % 4];
        let lift = if i < 4 { Vec3::zero() } else { n };
        turn.transform_vec(ux * u + vy * v + lift)
    })
}

type V3 = Vec3<f64>;

/// The half-spaces `n · p ≤ d` of the prism with `corners`, `n` outward.
fn prism_half_spaces(corners: &[V3; 8]) -> Vec<(V3, f64)> {
    let centroid = corners.iter().fold(Vec3::zero(), |acc, c| acc + *c) * (1.0 / 8.0);
    let mut faces: Vec<[usize; 3]> = vec![[0, 1, 2], [4, 5, 6]];
    faces.extend((0..4).map(|i| [i, (i + 1) % 4, 4 + (i + 1) % 4]));
    faces
        .iter()
        .map(|f| {
            let p = f.map(|i| corners[i]);
            let n = (p[1] - p[0]).cross(p[2] - p[0]);
            let n = if n.dot(centroid - p[0]) > 0.0 { -n } else { n };
            (n, n.dot(p[0]))
        })
        .collect()
}

/// **The volume of the unit-radius cylinder about `z`, between the
/// heights `zs`, against the convex prism with `corners`**
/// ([`prism_corners`]), closed form. The slab's two planes join the
/// prism's half-spaces, so the region is one convex polytope with the
/// disc's line through it: each vertical line through the disc meets it
/// in one segment, from a face whose outward normal points down to one
/// whose normal points up. The volume is the sum over its faces of their
/// height over the disc against their footprint, signed by the normal's
/// `z`: the height is linear, and its integral is
/// [`disc_clip_moments`]'. The faces come from the planes' triple
/// intersections that every half-space holds.
fn prism_in_cylinder(corners: &[V3; 8], zs: (f64, f64)) -> f64 {
    let mut planes = prism_half_spaces(corners);
    planes.push((Vec3::new(0.0, 0.0, 1.0), zs.1));
    planes.push((Vec3::new(0.0, 0.0, -1.0), -zs.0));
    let on = |(n, d): (V3, f64), p: V3| (n.dot(p) - d).abs() <= 1e-9 * n.norm();
    let mut vertices: Vec<V3> = Vec::new();
    for i in 0..planes.len() {
        for j in i + 1..planes.len() {
            for k in j + 1..planes.len() {
                let [(a, da), (b, db), (c, dc)] = [planes[i], planes[j], planes[k]];
                let det = a.dot(b.cross(c));
                if det.abs() < 1e-12 * a.norm() * b.norm() * c.norm() {
                    continue;
                }
                let p = (b.cross(c) * da + c.cross(a) * db + a.cross(b) * dc) * (1.0 / det);
                let inside = planes.iter().all(|&(n, d)| n.dot(p) - d <= 1e-9 * n.norm());
                if inside && !vertices.iter().any(|v| (*v - p).norm() < 1e-9) {
                    vertices.push(p);
                }
            }
        }
    }
    planes
        .iter()
        .map(|&(n, d)| {
            let face: Vec<V3> = vertices
                .iter()
                .copied()
                .filter(|&v| on((n, d), v))
                .collect();
            if face.len() < 3 || n.z.abs() < 1e-12 * n.norm() {
                return 0.0;
            }
            // The face's corners in turn about their mean, seen down `z`.
            let mid = face.iter().fold(Vec3::zero(), |acc, v| acc + *v) * (1.0 / face.len() as f64);
            let mut footprint: Vec<(f64, f64)> = face.iter().map(|v| (v.x, v.y)).collect();
            footprint.sort_by(|p, q| {
                let at = |r: &(f64, f64)| (r.1 - mid.y).atan2(r.0 - mid.x);
                at(p).total_cmp(&at(q))
            });
            let (area, mx, my) = disc_clip_moments(1.0, &ccw(footprint));
            (d * area - n.x * mx - n.y * my) / n.z.abs()
        })
        .sum()
}

/// The quad prism's volume: its cap's area times its height.
fn prism_volume(k: f64, sites: [f64; 4], cap: f64, h: f64) -> f64 {
    area(&quad(k, sites, cap)).abs() * h
}

/// The quadrature [`prism_in_cylinder`] is checked against: a polar
/// midpoint rule over the disc of each vertical line's extent inside
/// the half-spaces `planes` (`n · p ≤ d`) and the heights `zs`,
/// independent of the face sum.
fn prism_in_cylinder_by_quadrature(planes: &[(V3, f64)], zs: (f64, f64), n: usize) -> f64 {
    let (nr, nt) = (n, 4 * n);
    let mut v = 0.0;
    for i in 0..nr {
        let r = (i as f64 + 0.5) / nr as f64;
        for j in 0..nt {
            let t = (j as f64 + 0.5) / nt as f64 * core::f64::consts::TAU;
            let (x, y) = (r * t.cos(), r * t.sin());
            let (mut lo, mut hi) = zs;
            for &(nrm, d) in planes {
                let c = nrm.x * x + nrm.y * y;
                if nrm.z > 0.0 {
                    hi = hi.min((d - c) / nrm.z);
                } else if nrm.z < 0.0 {
                    lo = lo.max((d - c) / nrm.z);
                } else if c > d {
                    hi = f64::NEG_INFINITY;
                }
            }
            if hi > lo {
                v += (hi - lo) * r;
            }
        }
    }
    v * (1.0 / nr as f64) * (core::f64::consts::TAU / nt as f64)
}

/// The facet planes `n · p ≤ d` (`n` outward) of the convex hull of
/// `points`: every plane through three of them that holds the rest on
/// one side.
fn hull_planes(points: &[V3]) -> Vec<(V3, f64)> {
    let mut planes: Vec<(V3, f64)> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for k in j + 1..points.len() {
                let n = (points[j] - points[i]).cross(points[k] - points[i]);
                if n.norm() < 1e-9 {
                    continue;
                }
                let n = n * (1.0 / n.norm());
                let d = n.dot(points[i]);
                let side: Vec<f64> = points.iter().map(|p| n.dot(*p) - d).collect();
                let (n, d) = if side.iter().all(|&t| t <= 1e-9) {
                    (n, d)
                } else if side.iter().all(|&t| t >= -1e-9) {
                    (-n, -d)
                } else {
                    continue;
                };
                if !planes
                    .iter()
                    .any(|&(m, e)| (m - n).norm() < 1e-9 && (e - d).abs() < 1e-9)
                {
                    planes.push((n, d));
                }
            }
        }
    }
    planes
}

/// The face sum over [`prism_corners`] agrees with the quadrature over
/// the hull of the vertices the kernel built ([`prism_turned`]), steep
/// and shallow, along and against; and the quad's area times the height
/// is the prism's volume the kernel measures.
#[test]
fn the_prism_in_cylinder_oracle_agrees_with_a_quadrature() {
    for (k, sites, psi, along) in [
        (0.5, [190.0, 195.0, 260.0, 90.0], 0.0, true),
        (2.0, [200.0, 230.0, 300.0, 120.0], 1.1, false),
        (6.0, [225.0, 240.0, 320.0, 110.0], 1.658, false),
    ] {
        let side = if along {
            ExtrudeSide::Along
        } else {
            ExtrudeSide::Against
        };
        let body = topo::test_support::finished(
            "body",
            prism_turned(k, sites, 2.0, 1.5, side, psi),
            tol(),
        );
        let exact = prism_in_cylinder(&prism_corners(k, sites, 2.0, 1.5, along, psi), (-8.0, 8.0));
        let built: Vec<V3> = body
            .vertex_points()
            .map(|(_, p)| p - Point3::origin())
            .collect();
        let hull = hull_planes(&built);
        assert_eq!(
            hull.len(),
            6,
            "k = {k}: the built prism's hull has six faces"
        );
        let quadrature = prism_in_cylinder_by_quadrature(&hull, (-8.0, 8.0), 400);
        assert!(
            (exact - quadrature).abs() < 3e-5,
            "k = {k}: the face sum {exact} against the quadrature of the built prism {quadrature}"
        );
        let measured = topo::test_support::pinned_volume("the measured body", &body, tol()).volume;
        let closed = prism_volume(k, sites, 2.0, 1.5);
        assert!(
            (measured - closed).abs() < 1e-9,
            "k = {k}: the prism measures {measured} against {closed}"
        );
    }
}

/// **The quad prism battery**: steep and shallow tilted quad prisms
/// against a cylinder, the cap's section an ellipse cut in two arcs,
/// turned about the cylinder's axis so the seam moves. One line per
/// run; main and head are diffed on these lines.
#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn steep_quad_prism_battery() {
    let cyl_z = (-8.0, 16.0);
    let base = cylinder(1.0, cyl_z.0, cyl_z.1);
    let vc = core::f64::consts::PI * cyl_z.1;
    let site_sets: [[f64; 4]; 4] = [
        [190.0, 195.0, 260.0, 90.0],
        [200.0, 230.0, 300.0, 120.0],
        [160.0, 175.0, 250.0, 60.0],
        [225.0, 240.0, 320.0, 110.0],
    ];
    let list = |var: &str, dflt: &[f64]| -> Vec<f64> {
        std::env::var(var).map_or(dflt.to_vec(), |s| {
            s.split(',').map(|x| x.parse().unwrap()).collect()
        })
    };
    for k in list("R1_K", &[0.5, 1.2, 2.0, 3.0, 5.0]) {
        for sites in site_sets {
            for psi in list("R1_PSI", &[0.0, 1.1, 2.5, 4.0]) {
                for (sname, side, along) in [
                    ("along", ExtrudeSide::Along, true),
                    ("against", ExtrudeSide::Against, false),
                ] {
                    let h = 1.5;
                    let pr = prism_turned(k, sites, 2.0, h, side, psi);
                    let vp = prism_volume(k, sites, 2.0, h);
                    let vi = prism_in_cylinder(
                        &prism_corners(k, sites, 2.0, h, along, psi),
                        (cyl_z.0, cyl_z.0 + cyl_z.1),
                    );
                    for (op, want_ab, want_ba) in [
                        ("U", vc + vp - vi, vc + vp - vi),
                        ("S", vc - vi, vp - vi),
                        ("I", vi, vi),
                    ] {
                        for (order, want) in [("AB", want_ab), ("BA", want_ba)] {
                            let (l, r) = if order == "AB" {
                                (&base, &pr)
                            } else {
                                (&pr, &base)
                            };
                            println!(
                                "R1BAT k={k} sites={sites:?} psi={psi} {sname} {op} {order} => {}",
                                outcome(door(op, l, r), want, tol())
                            );
                        }
                    }
                }
            }
        }
    }
}

const R: f64 = 0.5;

fn plate(x: (f64, f64), t: f64) -> Body<f64> {
    let lp = bulge_loop(
        [(x.0, -2.0), (x.1, -2.0), (x.1, 2.0), (x.0, 2.0)]
            .into_iter()
            .map(|(a, b)| (Point2::new(a, b), 0.0))
            .collect(),
    );
    let p = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    extrude(
        &p,
        Extrusion::Distance {
            depth: t,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

/// The rod: a disc of radius `R` (its two vertices at sketch angle
/// `phi`), on a plane tilted `theta` about `y`, centred so its axis
/// meets `(0, 0, t/2)`, of length `len`.
fn rod(theta: f64, phi: f64, t: f64, len: f64) -> Body<f64> {
    let (s, c) = phi.sin_cos();
    let lp = bulge_loop(vec![
        (Point2::new(R * c, R * s), 1.0),
        (Point2::new(-R * c, -R * s), 1.0),
    ]);
    let rot = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), theta);
    let n = rot.transform_vec(Vec3::new(0.0, 0.0, 1.0));
    let p0 = Point3::new(0.0, 0.0, t / 2.0) - n * (len / 2.0);
    let place = Affine3::translation(p0 - Point3::origin()) * rot;
    let p = Profile::new(
        SketchPlane::from_frame(geom_core::test_support::rigid(
            &place,
            geom_core::Tol::witness(),
        )),
        vec![lp],
    )
    .validate(tol())
    .unwrap();
    extrude(
        &p,
        Extrusion::Distance {
            depth: len,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

/// The rod ∩ plate volume: slices are ellipses (semi-axes `R/cos θ`
/// along `x`, `R` along `y`) centred at `x = (z − t/2)·tan θ`.
fn want_i(theta: f64, x: (f64, f64), t: f64) -> f64 {
    let (c, tn) = (theta.cos(), theta.tan());
    let slice = |z: f64| {
        let xc = (z - t / 2.0) * tn;
        let (a, b) = ((x.0 - xc) * c, (x.1 - xc) * c);
        disc_clip_area(R, &[(a, -2.0), (b, -2.0), (b, 2.0), (a, 2.0)]) / c
    };
    let n = 4000;
    let h = t / n as f64;
    let mut s = slice(0.0) + slice(t);
    for k in 1..n {
        s += slice(k as f64 * h) * if k % 2 == 1 { 4.0 } else { 2.0 };
    }
    s * h / 3.0
}

#[test]
fn the_plate_rod_oracle_holds_at_a_plate_wider_than_the_rod() {
    // A plate the rod crosses whole: the slab's volume is the oblique
    // prism's, `π R² t / cos θ`.
    let (theta, t) = (1.2, 0.05);
    let w = want_i(theta, (-3.0, 3.0), t);
    let closed = core::f64::consts::PI * R * R * t / theta.cos();
    assert!((w - closed).abs() < 1e-10, "{w} vs {closed}");
}

#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn steep_plate_rod_battery() {
    for theta_deg in [30.0f64, 50.0, 60.0, 70.0, 78.0] {
        let theta = theta_deg.to_radians();
        let a_major = R / theta.cos();
        for t in [0.02, 0.1] {
            for x2f in [-0.3, -0.1, 0.1] {
                for x1f in [-0.6, -0.8, -0.95] {
                    let x = (x1f * a_major, x2f * R);
                    for phi in [0.0, core::f64::consts::FRAC_PI_2, 0.4] {
                        for mirror in [false, true] {
                            let xs = if mirror { (-x.1, -x.0) } else { x };
                            let pl = plate(xs, t);
                            let len = 2.0 * ((R + t) / theta.cos() + 1.0);
                            let rd =
                                topo::test_support::finished("rd", rod(theta, phi, t, len), tol());
                            let va = (xs.1 - xs.0) * 4.0 * t;
                            let vb =
                                topo::test_support::pinned_volume("the measured body", &rd, tol())
                                    .volume;
                            let vi = want_i(theta, xs, t);
                            let tag = format!(
                                "R2E th={theta_deg} t={t} x=({:.4},{:.4}) phi={phi:.2} m={mirror}",
                                xs.0, xs.1
                            );
                            for (op, w_ab, w_ba) in [
                                ("U", va + vb - vi, va + vb - vi),
                                ("S", va - vi, vb - vi),
                                ("I", vi, vi),
                            ] {
                                for (order, want) in [("AB", w_ab), ("BA", w_ba)] {
                                    let (l, r) = if order == "AB" {
                                        (&pl, &*rd)
                                    } else {
                                        (&*rd, &pl)
                                    };
                                    let got = door(op, l, r);
                                    println!("{tag} {op} {order} => {}", outcome(got, want, tol()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A U plate (`[−1, 1]² × [0, t]` less the slot `|x − c| < w`, `y > −0.2`,
/// open at `y = 1`) and an upright rod about the origin: the open row
/// `join-ranks-conic-facing-germs-by-chord`'s fixture shape, a circle cut
/// into two arcs on ONE face pair by a slot narrower than the prongs.
fn u_plate(c: f64, w: f64, t: f64) -> (Body<f64>, Vec<(f64, f64)>) {
    let poly = vec![
        (-1.0, -1.0),
        (1.0, -1.0),
        (1.0, 1.0),
        (c + w, 1.0),
        (c + w, -0.2),
        (c - w, -0.2),
        (c - w, 1.0),
        (-1.0, 1.0),
    ];
    let lp = bulge_loop(
        poly.iter()
            .map(|&(a, b)| (Point2::new(a, b), 0.0))
            .collect(),
    );
    let p = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let body = extrude(
        &p,
        Extrusion::Distance {
            depth: t,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    (body, poly)
}

#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn u_plate_battery() {
    let t = 0.1;
    for c in [0.0, 0.1, 0.25, -0.3] {
        for w in [0.03, 0.12] {
            for phi in [0.0, core::f64::consts::FRAC_PI_2, 0.4, 2.0] {
                let (pl, poly) = u_plate(c, w, t);
                let rd = topo::test_support::finished("rd", rod(0.0, phi, t, 3.0), tol());
                let va = area(&poly) * t;
                let vb = topo::test_support::pinned_volume("the measured body", &rd, tol()).volume;
                let vi = disc_clip_area(R, &poly) * t;
                let tag = format!("R2U c={c} w={w} phi={phi:.2}");
                for (op, w_ab, w_ba) in [
                    ("U", va + vb - vi, va + vb - vi),
                    ("S", va - vi, vb - vi),
                    ("I", vi, vi),
                ] {
                    for (order, want) in [("AB", w_ab), ("BA", w_ba)] {
                        let (l, r) = if order == "AB" {
                            (&pl, &*rd)
                        } else {
                            (&*rd, &pl)
                        };
                        let got = door(op, l, r);
                        println!("{tag} {op} {order} => {}", outcome(got, want, tol()));
                    }
                }
            }
        }
    }
}

/// One gating run: the op and the operand order.
type Run = (&'static str, &'static str);

/// The run's failure, if any: a refusal, or a body that fails tier 2, tier 3′, the
/// certificate or the volume (`vol_tol`, the oracle's accuracy).
fn miss(
    got: Result<BooleanResult<f64>, BooleanError>,
    volume: f64,
    vol_tol: f64,
) -> Option<String> {
    let r = match got {
        Err(e) => return Some(format!("refused {e:?}")),
        Ok(r) => r,
    };
    let Some(bb) = r.body() else {
        return Some("empty".into());
    };
    let t2 = topo::validate_closed(&bb.body).is_ok();
    let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()).is_ok();
    let cert = topo::validate_geometric_certificate(&bb.body, tol()).is_ok();
    let m = topo::mass_properties(
        &bb.body,
        tol(),
        topo::test_support::pinned_resolution(tol()),
    );
    let v = m
        .as_ref()
        .map(|m| m.enclosure.volume)
        .map_err(|e| e.clone());
    let pad = m.as_ref().map_or(0.0, |m| m.enclosure.volume_pad);
    match v {
        Ok(v) if t2 && t3 && cert && (v - volume).abs() < vol_tol + pad => None,
        _ => Some(format!(
            "body t2={t2} t3p={t3} cert={cert} v={v:?} pad={pad} want={volume}"
        )),
    }
}

/// At ε 1e-6 the θ = 70° mirrored pose reads its pierce-sector margin
/// (≈ 3.5e-6) inside the band: every run escalates on this predicate, typed,
/// rather than building. The same shape as `pinch_faces_tessellate`'s
/// `REFUSES_AT_1E6`.
const REFUSES_AT_1E6: (f64, &str) = (70.0, "bool_pierce_sector_side_curved");

/// **The poses the chord order paired wrongly build sound**, save the
/// θ = 70° mirrored pose at ε = 1e-6, which escalates typed
/// ([`REFUSES_AT_1E6`]). The quad prism poses are review r1's: at `k = 6`,
/// `[200, 230, 300, 120]`, `ψ = 1.75`, ∩ AB, the chord paired a germ
/// across its true partner and shipped a body failing tier 3′ and the
/// certificate; at `[225, 240, 320, 110]` every op refused
/// `RingHomingAmbiguous`; at `k = 0.5` the travel-correct pairing leaves
/// a ring on the cylinder wall. The plate poses are review r2's, where
/// the antipode of a germ near a minor end beat a nearer site by chord.
///
/// The plate's bodies are not asked to unite with a far brick (the
/// differential `outcome`'s operand check), which every plate body of
/// this shape refuses on main too.
#[test]
fn steep_ellipse_poses_build_sound() {
    let ops = door;
    let mut misses = Vec::new();

    let cyl_z = (-8.0, 16.0);
    let base = cylinder(1.0, cyl_z.0, cyl_z.1);
    let vc = core::f64::consts::PI * cyl_z.1;
    let all: [Run; 6] = [
        ("U", "AB"),
        ("U", "BA"),
        ("S", "AB"),
        ("S", "BA"),
        ("I", "AB"),
        ("I", "BA"),
    ];
    let quads: Vec<(f64, [f64; 4], f64, Vec<Run>)> = vec![
        (6.0, [200.0, 230.0, 300.0, 120.0], 1.75, vec![("I", "AB")]),
        (6.0, [225.0, 240.0, 320.0, 110.0], 1.658, all.to_vec()),
        (6.0, [225.0, 240.0, 320.0, 110.0], 4.8, all.to_vec()),
        (
            0.5,
            [200.0, 230.0, 300.0, 120.0],
            0.0,
            vec![("U", "AB"), ("U", "BA")],
        ),
    ];
    for (k, sites, psi, runs) in quads {
        let (h, side) = (1.5, ExtrudeSide::Against);
        let pr = prism_turned(k, sites, 2.0, h, side, psi);
        let vp = prism_volume(k, sites, 2.0, h);
        let vi = prism_in_cylinder(
            &prism_corners(k, sites, 2.0, h, false, psi),
            (cyl_z.0, cyl_z.0 + cyl_z.1),
        );
        for (op, order) in runs {
            let volume = match (op, order) {
                ("U", _) => vc + vp - vi,
                ("S", "AB") => vc - vi,
                ("S", _) => vp - vi,
                _ => vi,
            };
            let (l, r) = if order == "AB" {
                (&base, &pr)
            } else {
                (&pr, &base)
            };
            if let Some(m) = miss(ops(op, l, r), volume, 1e-7) {
                misses.push(format!("k={k} sites={sites:?} psi={psi} {op} {order}: {m}"));
            }
        }
    }

    for (theta_deg, t, x1f, x2f, mirror, runs) in [
        (60.0f64, 0.02, -0.95, 0.1, false, all),
        (70.0, 0.02, -0.8, -0.3, false, all),
        (70.0, 0.02, -0.8, 0.1, true, all),
    ] {
        let theta = theta_deg.to_radians();
        let x = (x1f * R / theta.cos(), x2f * R);
        let xs = if mirror { (-x.1, -x.0) } else { x };
        let pl = plate(xs, t);
        let rd = topo::test_support::finished(
            "rd",
            rod(
                theta,
                core::f64::consts::FRAC_PI_2,
                t,
                2.0 * ((R + t) / theta.cos() + 1.0),
            ),
            tol(),
        );
        let va = (xs.1 - xs.0) * 4.0 * t;
        let mb = topo::mass_properties(&rd, tol(), topo::test_support::pinned_resolution(tol()))
            .unwrap();
        let (vb, pad_b) = (mb.enclosure.volume, mb.enclosure.volume_pad);
        let vi = want_i(theta, xs, t);
        for (op, order) in runs {
            let volume = match (op, order) {
                ("U", _) => va + vb - vi,
                ("S", "AB") => va - vi,
                ("S", _) => vb - vi,
                _ => vi,
            };
            let (l, r) = if order == "AB" {
                (&pl, &*rd)
            } else {
                (&*rd, &pl)
            };
            let got = ops(op, l, r);
            if tol().eps() == 1e-6 && theta_deg == REFUSES_AT_1E6.0 && mirror {
                let escalated = match &got {
                    Err(BooleanError::Escalated { diag, .. }) => diag.predicate,
                    _ => None,
                };
                if escalated != Some(REFUSES_AT_1E6.1) {
                    misses.push(format!("theta={theta_deg} mirror {op} {order}: at 1e-6 want the in-band escalation, got {got:?}"));
                }
                continue;
            }
            if let Some(m) = miss(got, volume, 1e-7 + pad_b) {
                misses.push(format!(
                    "theta={theta_deg} x={xs:?} mirror={mirror} {op} {order}: {m}"
                ));
            }
        }
    }
    assert!(
        misses.is_empty(),
        "{} runs miss:\n{}",
        misses.len(),
        misses.join("\n")
    );
}
