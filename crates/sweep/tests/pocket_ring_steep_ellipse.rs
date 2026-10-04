//! **Conic germ lines on a steep ellipse.** The join pairs each germ with
//! its nearest partner along the section conic. On an ellipse of aspect
//! √2 or more the chord from a site near a minor end peaks inside the
//! half-turn the germ runs into and falls again, so the nearest site by
//! chord can lie past the true partner. Two shapes reach that regime:
//!
//! - a cylinder (r = 1) against a convex quad prism whose cap lies in the
//!   plane `z = k·x`, turned `ψ` about the axis; each long edge of the
//!   quad cuts the cap's ellipse `(cos φ, sin φ, k cos φ)` twice. Oracle:
//!   a polar quadrature of cylinder ∩ prism, good to about 1e-6.
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
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{Body, BooleanError, BooleanResult};

use crate::common::differential::{area, disc_clip_area, outcome};

fn tol() -> Tol {
    Tol::witness()
}

fn cylinder(r: f64, z0: f64, h: f64) -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(r, 0.0), 1.0),
        (Point2::new(-r, 0.0), 1.0),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
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
    let [e1a, e1b, e2a, e2b] = sites.map(|d| uv(k, d));
    let quad = [
        (at_v(e1a, e1b, -cap), -cap),
        (at_v(e2a, e2b, -cap), -cap),
        (at_v(e2a, e2b, cap), cap),
        (at_v(e1a, e1b, cap), cap),
    ];
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
    let profile = Profile::new(SketchPlane::new(turn * rot), vec![lp])
        .validate(tol())
        .unwrap();
    extrude(&profile, Extrusion::Distance { depth: h, side }, tol())
        .unwrap()
        .body
}

/// Volume of cylinder (r = 1, z ∈ [zlo, zhi]) ∩ prism by a polar
/// midpoint quadrature over the disc of the vertical extent inside the
/// prism (the prism is convex: an interval per vertical line).
fn oracle_intersection(prism_planes: &[(Vec3<f64>, f64)], zlo: f64, zhi: f64, n: usize) -> f64 {
    let mut v = 0.0;
    let (nr, nt) = (n, 4 * n);
    for i in 0..nr {
        let r = (i as f64 + 0.5) / nr as f64;
        for j in 0..nt {
            let t = (j as f64 + 0.5) / nt as f64 * core::f64::consts::TAU;
            let (x, y) = (r * t.cos(), r * t.sin());
            let (mut lo, mut hi) = (zlo, zhi);
            for &(nrm, d) in prism_planes {
                // nrm · p <= d, p = (x, y, z)
                let c = nrm.x * x + nrm.y * y;
                if nrm.z.abs() < 1e-15 {
                    if c > d {
                        hi = lo - 1.0;
                    }
                } else if nrm.z > 0.0 {
                    hi = hi.min((d - c) / nrm.z);
                } else {
                    lo = lo.max((d - c) / nrm.z);
                }
            }
            if hi > lo {
                v += (hi - lo) * r;
            }
        }
    }
    v * (1.0 / nr as f64) * (core::f64::consts::TAU / nt as f64)
}

/// The prism's half-spaces `(n, d)`, `n · p ≤ d`, for the oracle.
fn prism_planes(k: f64, sites: [f64; 4], cap: f64, h: f64, along: bool) -> Vec<(Vec3<f64>, f64)> {
    let s = 1.0 / (1.0 + k * k).sqrt();
    let n = Vec3::new(-k, 0.0, 1.0) * s;
    let ux = Vec3::new(1.0, 0.0, k) * s;
    let vy = Vec3::new(0.0, 1.0, 0.0);
    let [e1a, e1b, e2a, e2b] = sites.map(|d| uv(k, d));
    let quad = [
        (at_v(e1a, e1b, -cap), -cap),
        (at_v(e2a, e2b, -cap), -cap),
        (at_v(e2a, e2b, cap), cap),
        (at_v(e1a, e1b, cap), cap),
    ];
    let sg = if along { 1.0 } else { -1.0 };
    let mut planes = vec![(n * -sg, 0.0), (n * sg, h)];
    for i in 0..4 {
        let (a, b) = (quad[i], quad[(i + 1) % 4]);
        let (du, dv) = (b.0 - a.0, b.1 - a.1);
        let nrm = ux * dv + vy * (-du);
        let d = nrm.x * (ux.x * a.0) + nrm.y * a.1 + nrm.z * (ux.z * a.0);
        planes.push((nrm, d));
    }
    planes
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
    let vc = topo::mass_properties(&base, tol()).unwrap().volume;
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
                    let turn = Affine3::rotation_about_axis(
                        Point3::origin(),
                        Vec3::new(0.0, 0.0, 1.0),
                        psi,
                    );
                    let pr = prism_turned(k, sites, 2.0, h, side, psi);
                    let vp = topo::mass_properties(&pr, tol()).unwrap().volume;
                    let planes: Vec<_> = prism_planes(k, sites, 2.0, h, along)
                        .into_iter()
                        .map(|(n, d)| (turn.transform_vec(n), d))
                        .collect();
                    let vi = oracle_intersection(&planes, cyl_z.0, cyl_z.0 + cyl_z.1, 1200);
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
                            let got = match op {
                                "U" => topo::union(l, r, tol()),
                                "S" => topo::subtract(l, r, tol()),
                                _ => topo::intersect(l, r, tol()),
                            };
                            let line = match got {
                                Err(e) => {
                                    let s = format!("{e:?}");
                                    format!("ERR {}", s.chars().take(110).collect::<String>())
                                }
                                Ok(res) => match res.body() {
                                    None => format!("EMPTY want={want:.6}"),
                                    Some(bb) => {
                                        let t2 = topo::validate_closed(&bb.body).is_ok();
                                        let t3 = topo::validate_pseudomanifold(
                                            &bb.body,
                                            &bb.contacts,
                                            tol(),
                                        )
                                        .is_ok();
                                        let cert =
                                            topo::validate_geometric_certificate(&bb.body, tol())
                                                .is_ok();
                                        match topo::mass_properties(&bb.body, tol()) {
                                            Ok(m) => {
                                                let good = (m.volume - want).abs() < 2e-5;
                                                format!(
                                                    "OK {} t2={t2} t3p={t3} cert={cert} v={:.9} \
                                                     want~{want:.6}",
                                                    if good && t2 && t3 && cert {
                                                        "SOUND"
                                                    } else {
                                                        "BAD"
                                                    },
                                                    m.volume
                                                )
                                            }
                                            Err(e) => format!("OK-UNMEASURED {e:?}"),
                                        }
                                    }
                                },
                            };
                            println!(
                                "R1BAT k={k} sites={sites:?} psi={psi} {sname} {op} {order} => \
                                 {line}"
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
    let p = Profile::new(SketchPlane::new(place), vec![lp])
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
                            let rd = rod(theta, phi, t, len);
                            let va = (xs.1 - xs.0) * 4.0 * t;
                            let vb = topo::mass_properties(&rd, tol()).unwrap().volume;
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
                                        (&pl, &rd)
                                    } else {
                                        (&rd, &pl)
                                    };
                                    let got = match op {
                                        "U" => topo::union(l, r, tol()),
                                        "S" => topo::subtract(l, r, tol()),
                                        _ => topo::intersect(l, r, tol()),
                                    };
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
                let rd = rod(0.0, phi, t, 3.0);
                let va = area(&poly) * t;
                let vb = topo::mass_properties(&rd, tol()).unwrap().volume;
                let vi = disc_clip_area(R, &poly) * t;
                let tag = format!("R2U c={c} w={w} phi={phi:.2}");
                for (op, w_ab, w_ba) in [
                    ("U", va + vb - vi, va + vb - vi),
                    ("S", va - vi, vb - vi),
                    ("I", vi, vi),
                ] {
                    for (order, want) in [("AB", w_ab), ("BA", w_ba)] {
                        let (l, r) = if order == "AB" {
                            (&pl, &rd)
                        } else {
                            (&rd, &pl)
                        };
                        let got = match op {
                            "U" => topo::union(l, r, tol()),
                            "S" => topo::subtract(l, r, tol()),
                            _ => topo::intersect(l, r, tol()),
                        };
                        println!("{tag} {op} {order} => {}", outcome(got, want, tol()));
                    }
                }
            }
        }
    }
}

/// What a gating run must do.
#[derive(Clone, Copy, Debug)]
enum Want {
    /// Build a sound body.
    Sound,
    /// Build a sound body, or refuse with anything but a join desync —
    /// for runs whose travel-correct pairing reaches a frontier past the
    /// join (`VolumeUnmeasured { RingOnCurvedFace }`, the volume lane's).
    SoundOrRefused,
}

/// The run's failure, if it fails `want`: a desync, a refusal where a
/// body is wanted, or a body that fails tier 2, tier 3′, the
/// certificate or the volume (`vol_tol`, the oracle's accuracy).
fn miss(
    got: Result<BooleanResult<f64>, BooleanError>,
    want: Want,
    volume: f64,
    vol_tol: f64,
) -> Option<String> {
    let r = match got {
        Err(e @ BooleanError::JoinDesync { .. }) => return Some(format!("desync {e:?}")),
        Err(e) => {
            return match want {
                Want::Sound => Some(format!("refused {e:?}")),
                Want::SoundOrRefused => None,
            };
        }
        Ok(r) => r,
    };
    let Some(bb) = r.body() else {
        return Some("empty".into());
    };
    let t2 = topo::validate_closed(&bb.body).is_ok();
    let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()).is_ok();
    let cert = topo::validate_geometric_certificate(&bb.body, tol()).is_ok();
    let v = topo::mass_properties(&bb.body, tol()).map(|m| m.volume);
    match v {
        Ok(v) if t2 && t3 && cert && (v - volume).abs() < vol_tol => None,
        _ => Some(format!(
            "body t2={t2} t3p={t3} cert={cert} v={v:?} want={volume}"
        )),
    }
}

/// **The poses the chord order paired wrongly build sound or refuse
/// typed.** The quad prism poses are review r1's: at `k = 6`,
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
fn steep_ellipse_poses_build_sound_or_refuse_typed() {
    use Want::{Sound, SoundOrRefused};
    let ops = |op: &str, l: &Body<f64>, r: &Body<f64>| match op {
        "U" => topo::union(l, r, tol()),
        "S" => topo::subtract(l, r, tol()),
        _ => topo::intersect(l, r, tol()),
    };
    let mut misses = Vec::new();

    let cyl_z = (-8.0, 16.0);
    let base = cylinder(1.0, cyl_z.0, cyl_z.1);
    let vc = topo::mass_properties(&base, tol()).unwrap().volume;
    let all = |w| {
        [
            ("U", "AB", w),
            ("U", "BA", w),
            ("S", "AB", w),
            ("S", "BA", w),
            ("I", "AB", w),
            ("I", "BA", w),
        ]
    };
    let quads: Vec<(f64, [f64; 4], f64, Vec<(&str, &str, Want)>)> = vec![
        (
            6.0,
            [200.0, 230.0, 300.0, 120.0],
            1.75,
            vec![("I", "AB", Sound)],
        ),
        (
            6.0,
            [225.0, 240.0, 320.0, 110.0],
            1.658,
            all(Sound).to_vec(),
        ),
        (6.0, [225.0, 240.0, 320.0, 110.0], 4.8, all(Sound).to_vec()),
        (
            0.5,
            [200.0, 230.0, 300.0, 120.0],
            0.0,
            vec![("U", "AB", SoundOrRefused)],
        ),
    ];
    for (k, sites, psi, runs) in quads {
        let (h, side) = (1.5, ExtrudeSide::Against);
        let pr = prism_turned(k, sites, 2.0, h, side, psi);
        let vp = topo::mass_properties(&pr, tol()).unwrap().volume;
        let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), psi);
        let planes: Vec<_> = prism_planes(k, sites, 2.0, h, false)
            .into_iter()
            .map(|(n, d)| (turn.transform_vec(n), d))
            .collect();
        let vi = oracle_intersection(&planes, cyl_z.0, cyl_z.0 + cyl_z.1, 1200);
        for (op, order, want) in runs {
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
            if let Some(m) = miss(ops(op, l, r), want, volume, 2e-5) {
                misses.push(format!(
                    "k={k} sites={sites:?} psi={psi} {op} {order} ({want:?}): {m}"
                ));
            }
        }
    }

    for (theta_deg, t, x1f, x2f, mirror, runs) in [
        (60.0f64, 0.02, -0.95, 0.1, false, all(Sound)),
        (70.0, 0.02, -0.8, -0.3, false, all(SoundOrRefused)),
        (70.0, 0.02, -0.8, 0.1, true, all(Sound)),
    ] {
        let theta = theta_deg.to_radians();
        let x = (x1f * R / theta.cos(), x2f * R);
        let xs = if mirror { (-x.1, -x.0) } else { x };
        let pl = plate(xs, t);
        let rd = rod(
            theta,
            core::f64::consts::FRAC_PI_2,
            t,
            2.0 * ((R + t) / theta.cos() + 1.0),
        );
        let va = (xs.1 - xs.0) * 4.0 * t;
        let vb = topo::mass_properties(&rd, tol()).unwrap().volume;
        let vi = want_i(theta, xs, t);
        for (op, order, want) in runs {
            let volume = match (op, order) {
                ("U", _) => va + vb - vi,
                ("S", "AB") => va - vi,
                ("S", _) => vb - vi,
                _ => vi,
            };
            let (l, r) = if order == "AB" {
                (&pl, &rd)
            } else {
                (&rd, &pl)
            };
            if let Some(m) = miss(ops(op, l, r), want, volume, 1e-7) {
                misses.push(format!(
                    "theta={theta_deg} x={xs:?} mirror={mirror} {op} {order} ({want:?}): {m}"
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
