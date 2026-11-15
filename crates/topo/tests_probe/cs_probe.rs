#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]
use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Decide, Interval, ParamSymbol, Point3, Real, Sym, SymBudget, Tol, Vec3};
use topo::construction::{Construct, FaceSurface};
use topo::euler::{MefSite, MevSite};
use topo::{Body, FaceKey};

type Boxed = Sym<Interval>;

/// The cylinder of `radius` about `z` over `[0, top]`: caps and one wall, one seam.
pub fn cylinder<T: Decide + topo::AtRestPolicy>(radius: T, top: T, flip: bool, tol: Tol) -> (Body<T>, [FaceKey; 3]) {
    let (zero, one) = (T::zero(), T::one());
    let (x, z) = (Vec3::new(one, zero, zero), Vec3::new(zero, zero, one));
    let axis = if flip { z * -one } else { z };
    let mut body = Body::<T>::new();
    let p0 = Point3::new(radius, zero, zero);
    let p1 = Point3::new(radius, zero, top);
    let seed = body.mvfs(p0, true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: Surface::Cylinder { origin: Point3::origin(), axis: z, radius, u_ref: x },
            sense: true,
        },
    )
    .unwrap();
    let circle = |at: T, up: bool| {
        let c = Curve3::circle(Point3::new(zero, zero, at), if up { axis } else { axis * -one }, radius, x);
        EdgeCurveSpec::arc_of_circle(c, T::zero(), T::tau()).unwrap()
    };
    let bottom = body
        .mef(
            MefSite::Lone { r#loop: seed.r#loop },
            circle(zero, true),
            FaceSurface::New {
                surface: Surface::Plane { origin: Point3::origin(), normal: z * -one, u_ref: x },
                sense: true,
            },
            tol,
        )
        .unwrap();
    let strut = body
        .mev_line(MevSite::Fan { he1: bottom.he_plus, he2: bottom.he_plus }, p1, tol)
        .unwrap();
    let cap = body
        .mef(
            MefSite::Chords { he1: strut.he_minus, he2: strut.he_minus },
            circle(top, false),
            FaceSurface::New {
                surface: Surface::Plane { origin: Point3::new(zero, zero, top), normal: z, u_ref: x },
                sense: true,
            },
            tol,
        )
        .unwrap();
    (body, [seed.face, bottom.face, cap.face])
}

fn over<R>(top: (f64, f64), radius: (f64, f64), read: impl FnOnce(Boxed, Boxed, Band, Tol) -> R) -> R {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let budget = SymBudget { max_terms: 64, max_degree: 8 };
    let axis = |name: &str, (lo, hi): (f64, f64)| {
        if lo == hi {
            Boxed::from_f64(lo)
        } else {
            Sym::param_over(ParamSymbol::new(test_utils::symbol_id(name)), Interval::from_bounds(lo, hi), lo, hi)
        }
    };
    geom_core::sym::with_session(budget, || {
        read(axis("cs_top", top), axis("cs_radius", radius), band, tol)
    })
    .0
}

#[test]
fn sanity() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for flip in [false, true] {
        let (body, faces) = cylinder::<f64>(1.0, 1.0, flip, tol);
        println!("flip {flip}: faces {faces:?}");
        for r in topo::test_support::each_aim(&body, band) {
            println!("  f64 {r}");
        }
        println!("  rays {:?}", topo::test_support::aimed_rays(&body, band));
        for (lo, hi) in [(1.0, 1.0), (0.9, 1.1)] {
            let rs = over((lo, hi), (1.0, 1.0), |top, radius, band, tol| {
                let (body, _) = cylinder::<Boxed>(radius, top, flip, tol);
                topo::test_support::each_aim(&body, band)
            });
            for r in rs {
                println!("  boxed top [{lo},{hi}] {}", r.chars().take(300).collect::<String>());
            }
        }
    }
}

fn kind(r: &str) -> &'static str {
    if r.contains("Side(In)") { "IN" } else if r.contains("Side(Out)") { "out" }
    else if r.contains("SetAside(Crossing") { "aside:crossing" }
    else if r.contains("SetAside(Graze") { "aside:graze" }
    else if r.contains("SetAside(Corner") || r.contains("SetAside(Wedge") { "aside:corner" }
    else if r.contains("SetAside(Limit") { "aside:limit" }
    else if r.contains("Pair(") { "pair" } else { "other" }
}

#[test]
fn search() {
    let mut rng = test_utils::fuzz::start("cs_probe_search");
    let n_boxes: usize = std::env::var("CS_BOXES").ok().and_then(|s| s.parse().ok()).unwrap_or(40);
    let n_rays: usize = std::env::var("CS_RAYS").ok().and_then(|s| s.parse().ok()).unwrap_or(60);
    let mut tally = std::collections::BTreeMap::<&str, usize>::new();
    let mut wrong = 0;
    for k in 0..n_boxes {
        let mid_h = rng.log_uniform(0.05, 3.0);
        let mid_r = rng.log_uniform(0.05, 3.0);
        let half_h = match k % 3 { 1 => 0.0, _ => rng.log_uniform(1e-7, 0.9) * mid_h };
        let half_r = match k % 3 { 0 => 0.0, _ => rng.log_uniform(1e-7, 0.9) * mid_r };
        let (top, radius) = ((mid_h - half_h, mid_h + half_h), (mid_r - half_r, mid_r + half_r));
        let draws: Vec<([f64; 3], [f64; 3])> = (0..n_rays)
            .map(|_| {
                let pick = |rng: &mut test_utils::fuzz::Rng, (lo, hi): (f64, f64)| match rng.below(4) {
                    0 => lo, 1 => hi, 2 => 0.5 * (lo + hi), _ => rng.range(lo, hi),
                };
                let (h, r) = (pick(&mut rng, top), pick(&mut rng, radius));
                let a = rng.range(0.0, core::f64::consts::TAU);
                // off the rim by a signed amount at several scales
                let off = match rng.below(5) { 0 => 0.0, 1 => rng.range(-1e-2, 1e-2), 2 => rng.range(-0.2, 0.2) * r, 3 => rng.range(-1e-6, 1e-6), _ => rng.range(-0.05, 0.05) };
                let dz = match rng.below(4) { 0 => rng.range(-0.3, 0.3) * h, 1 => rng.range(-1e-2, 1e-2), _ => 0.0 };
                let rr = r + off;
                let aim = [rr * a.cos(), rr * a.sin(), h + dz];
                let normal = match rng.below(6) {
                    0 => [0.0, 0.0, 1.0],
                    1 => [a.cos(), a.sin(), 0.0],
                    2 => [0.0, 0.0, -1.0],
                    3 => [-a.cos(), -a.sin(), 0.0],
                    _ => {
                        let v = [rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0)];
                        let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-3);
                        [v[0] / n, v[1] / n, v[2] / n]
                    }
                };
                (aim, normal)
            })
            .collect();
        let readings = over(top, radius, |t, r, band, tol| {
            let (body, faces) = cylinder::<Boxed>(r, t, false, tol);
            let iv = Boxed::from_f64;
            draws.iter().map(|&(a, n)| {
                topo::test_support::ray_at(&body, faces[0], Point3::new(iv(a[0]), iv(a[1]), iv(a[2])), Vec3::new(iv(n[0]), iv(n[1]), iv(n[2])), band)
            }).collect::<Vec<_>>()
        });
        for (r, (aim, normal)) in readings.iter().zip(&draws) {
            *tally.entry(kind(r)).or_default() += 1;
            if kind(r) == "IN" {
                wrong += 1;
                println!("WRONG top {top:?} radius {radius:?} aim {aim:?} normal {normal:?}: {r}");
            }
        }
    }
    println!("tally {tally:?} wrong {wrong}");
}

/// A draw: the aim as `(a, f, dz, ride)`: angle, radial factor offset, z offset, and whether it rides the box.
#[derive(Clone, Copy, Debug)]
struct Draw { a: f64, off: f64, dz: f64, ride: bool, h: f64, r: f64, n: [f64; 3] }

fn aim_of<T: Real>(d: &Draw, top: T, radius: T) -> Point3<T> {
    let f = T::from_f64;
    if d.ride {
        let rr = radius + f(d.off);
        Point3::new(rr * f(d.a.cos()), rr * f(d.a.sin()), top + f(d.dz))
    } else {
        let rr = d.r + d.off;
        Point3::new(f(rr * d.a.cos()), f(rr * d.a.sin()), f(d.h + d.dz))
    }
}

#[test]
fn differential() {
    let mut rng = test_utils::fuzz::start("cs_probe_diff");
    let n_boxes: usize = std::env::var("CS_BOXES").ok().and_then(|s| s.parse().ok()).unwrap_or(40);
    let n_rays: usize = std::env::var("CS_RAYS").ok().and_then(|s| s.parse().ok()).unwrap_or(60);
    let mut moves = std::collections::BTreeMap::<(&str, &str), usize>::new();
    let tol = Tol::witness();
    let fband = Band::linear(tol).unwrap();
    for k in 0..n_boxes {
        let mid_h = rng.log_uniform(0.05, 3.0);
        let mid_r = rng.log_uniform(0.05, 3.0);
        let half_h = match k % 3 { 1 => 0.0, _ => rng.log_uniform(1e-7, 0.9) * mid_h };
        let half_r = match k % 3 { 0 => 0.0, _ => rng.log_uniform(1e-7, 0.9) * mid_r };
        let (top, radius) = ((mid_h - half_h, mid_h + half_h), (mid_r - half_r, mid_r + half_r));
        let draws: Vec<Draw> = (0..n_rays)
            .map(|_| {
                let pick = |rng: &mut test_utils::fuzz::Rng, (lo, hi): (f64, f64)| match rng.below(4) {
                    0 => lo, 1 => hi, 2 => 0.5 * (lo + hi), _ => rng.range(lo, hi),
                };
                let (h, r) = (pick(&mut rng, top), pick(&mut rng, radius));
                let a = rng.range(0.0, core::f64::consts::TAU);
                let off = match rng.below(5) { 0 => 0.0, 1 => rng.range(-1e-2, 1e-2), 2 => rng.range(-0.2, 0.2) * r, 3 => rng.range(-1e-6, 1e-6), _ => rng.range(-0.05, 0.05) };
                let dz = match rng.below(4) { 0 => rng.range(-0.3, 0.3) * h, 1 => rng.range(-1e-2, 1e-2), _ => 0.0 };
                let n = match rng.below(6) {
                    0 => [0.0, 0.0, 1.0],
                    1 => [a.cos(), a.sin(), 0.0],
                    2 => [0.0, 0.0, -1.0],
                    3 => [-a.cos(), -a.sin(), 0.0],
                    _ => {
                        let v = [rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0)];
                        let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-3);
                        [v[0] / n, v[1] / n, v[2] / n]
                    }
                };
                Draw { a, off, dz, ride: rng.below(2) == 0, h, r, n }
            })
            .collect();
        let read = |fix: bool| {
            mode(if fix { "unread" } else { "met" });
            over(top, radius, |t, r, band, tol| {
                let (body, faces) = cylinder::<Boxed>(r, t, false, tol);
                let iv = Boxed::from_f64;
                draws.iter().map(|d| {
                    topo::test_support::ray_at(&body, faces[0], aim_of(d, t, r), Vec3::new(iv(d.n[0]), iv(d.n[1]), iv(d.n[2])), band)
                }).collect::<Vec<_>>()
            })
        };
        let (old, new) = (read(false), read(true));
        for ((o, n), d) in old.iter().zip(&new).zip(&draws) {
            *moves.entry((kind(o), kind(n))).or_default() += 1;
            if kind(o) != kind(n) && (kind(o) == "out" || kind(o) == "IN") {
                let members: Vec<String> = [0.0, 0.5, 1.0].iter().flat_map(|&u| [0.0, 0.5, 1.0].map(|w| (u, w))).map(|(u, w)| {
                    let (t, r) = (top.0 + u * (top.1 - top.0), radius.0 + w * (radius.1 - radius.0));
                    let (body, faces) = cylinder::<f64>(r, t, false, tol);
                    kind(&topo::test_support::ray_at(&body, faces[0], aim_of(d, t, r), Vec3::new(d.n[0], d.n[1], d.n[2]), fband)).to_owned()
                }).collect();
                println!("RESTED {} -> {} top {top:?} radius {radius:?} {d:?} members {members:?}", kind(o), kind(n));
            }
        }
    }
    println!("moves {moves:?}");
}

/// Rays shallow to the cap and near-tangent to the wall at the moving rim, over tiny to small boxes.
#[test]
fn skim() {
    let mut rng = test_utils::fuzz::start("cs_probe_skim");
    let n_boxes: usize = std::env::var("CS_BOXES").ok().and_then(|s| s.parse().ok()).unwrap_or(60);
    let n_rays: usize = std::env::var("CS_RAYS").ok().and_then(|s| s.parse().ok()).unwrap_or(80);
    let mut tally = std::collections::BTreeMap::<&str, usize>::new();
    let tol = Tol::witness();
    let fband = Band::linear(tol).unwrap();
    let eps = tol.eps();
    for k in 0..n_boxes {
        let g = rng.log_uniform(5.0 * eps, 1e-2);
        let (top, radius) = match k % 3 { 0 => ((1.0, 1.0 + g), (1.0, 1.0)), 1 => ((1.0, 1.0), (1.0, 1.0 + g)), _ => ((1.0, 1.0 + g), (1.0, 1.0 + g)) };
        // each draw: a point near the rim and a direction given whole
        let draws: Vec<([f64; 3], [f64; 3])> = (0..n_rays).map(|_| {
            let a = rng.range(0.0, core::f64::consts::TAU);
            let (c, s) = (a.cos(), a.sin());
            let sc = |rng: &mut test_utils::fuzz::Rng| match rng.below(3) { 0 => 0.0, 1 => rng.range(-2.0, 2.0) * g, _ => { let m = rng.log_uniform(eps, 1e-1); if rng.below(2) == 0 { m } else { -m } } };
            let (dr, dz) = (sc(&mut rng), sc(&mut rng));
            let h = 1.0 + rng.unit() * (top.1 - top.0);
            let r = 1.0 + rng.unit() * (radius.1 - radius.0);
            let p = [(r + dr) * c, (r + dr) * s, h + dz];
            // direction: tangential + small radial + small vertical, or generic
            let small = |rng: &mut test_utils::fuzz::Rng| { let m = rng.log_uniform(1e-9, 1.0); if rng.below(2) == 0 { m } else { -m } };
            let (tr, tz, tt) = match rng.below(4) { 0 => (small(&mut rng), small(&mut rng), 1.0), 1 => (small(&mut rng), 1.0, small(&mut rng)), 2 => (1.0, small(&mut rng), small(&mut rng)), _ => (rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0)) };
            let d = [tr * c - tt * s, tr * s + tt * c, tz];
            let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            (p, [d[0] / n, d[1] / n, d[2] / n])
        }).collect();
        // ray_at turns `normal` by the aimed incidence: give the normal that yields direction -d approximately = d itself
        let readings = over(top, radius, |t, r, band, tol| {
            let (body, faces) = cylinder::<Boxed>(r, t, false, tol);
            let iv = Boxed::from_f64;
            draws.iter().map(|&(a, n)| topo::test_support::ray_at(&body, faces[0], Point3::new(iv(a[0]), iv(a[1]), iv(a[2])), Vec3::new(iv(n[0]), iv(n[1]), iv(n[2])), band)).collect::<Vec<_>>()
        });
        for (rd, (p, n)) in readings.iter().zip(&draws) {
            *tally.entry(kind(rd)).or_default() += 1;
            if kind(rd) == "out" || kind(rd) == "IN" {
                let members: Vec<&str> = [0.0, 0.25, 0.5, 0.75, 1.0].iter().flat_map(|&u| [0.0, 0.5, 1.0].map(|w| (u, w))).map(|(u, w)| {
                    let (t, r) = (top.0 + u * (top.1 - top.0), radius.0 + w * (radius.1 - radius.0));
                    let (body, faces) = cylinder::<f64>(r, t, false, tol);
                    kind(&topo::test_support::ray_at(&body, faces[0], Point3::new(p[0], p[1], p[2]), Vec3::new(n[0], n[1], n[2]), fband))
                }).collect();
                if kind(rd) == "IN" || members.iter().any(|m| *m == "IN") {
                    println!("WRONG {} top {top:?} radius {radius:?} p {p:?} n {n:?} members {members:?}", kind(rd));
                }
            }
        }
    }
    println!("skim tally {tally:?}");
}

#[test]
fn designed() {
    let ftol = Tol::witness();
    let fband = Band::linear(ftol).unwrap();
    for (top, radius) in [((0.9, 1.1), (1.0, 1.0)), ((1.0, 1.0 + 1e-6), (1.0, 1.0)), ((1.0, 1.0), (0.9, 1.1)), ((1.0, 1.0), (1.0, 1.0 + 1e-6))] {
        for (what, ride, off, dz, n) in [
            ("A ride on rim, n=z", true, 0.0, 0.0, [0.0, 0.0, 1.0]),
            ("A' ride on rim, n=radial", true, 0.0, 0.0, [1.0, 0.0, 0.0]),
            ("A'' ride on rim, n=diag", true, 0.0, 0.0, [0.6, 0.0, 0.8]),
            ("B ride 0.05 inside, n=z", true, -0.05, 0.0, [0.0, 0.0, 1.0]),
            ("C fixed 0.05 inside at mid, n=z", false, -0.05, 0.0, [0.0, 0.0, 1.0]),
            ("D fixed on rim at mid, n=diag", false, 0.0, 0.0, [0.6, 0.0, 0.8]),
            ("E fixed 0.02 inside at hi, n=z", false, -0.02, 0.5 * (top.1 - top.0), [0.0, 0.0, 1.0]),
        ] {
            let d = Draw { a: 0.0, off, dz, ride, h: 0.5 * (top.0 + top.1), r: 0.5 * (radius.0 + radius.1), n };
            let r = over(top, radius, |t, r, band, tol| {
                let (body, faces) = cylinder::<Boxed>(r, t, false, tol);
                let iv = Boxed::from_f64;
                topo::test_support::ray_at(&body, faces[0], aim_of(&d, t, r), Vec3::new(iv(n[0]), iv(n[1]), iv(n[2])), band)
            });
            let members: Vec<&str> = [0.0, 0.5, 1.0].iter().map(|&u| {
                let (t, r) = (top.0 + u * (top.1 - top.0), radius.0 + u * (radius.1 - radius.0));
                let (body, faces) = cylinder::<f64>(r, t, false, ftol);
                kind(&topo::test_support::ray_at(&body, faces[0], aim_of(&d, t, r), Vec3::new(n[0], n[1], n[2]), fband))
            }).collect();
            let pred = r.split("predicate: ").nth(1).unwrap_or("").chars().take(44).collect::<String>();
            println!("top {top:?} r {radius:?} {what}: {} {} members {members:?}", kind(&r), pred);
        }
    }
}

fn mode(m: &str) {
    std::fs::write(format!("/home/user/cs-logs/MODE.{}", std::process::id()), m).unwrap();
}


#[test]
fn family() {
    let ftol = Tol::witness();
    let fband = Band::linear(ftol).unwrap();
    for (top, radius) in [((0.9, 1.1), (1.0, 1.0)), ((1.0, 1.0), (0.9, 1.1)), ((0.9, 1.1), (0.9, 1.1))] {
        let mut rows = std::collections::BTreeMap::<String, usize>::new();
        for a in [0.7, 1.9, 3.1, 4.3, 5.5] {
            for off in [-0.01, -0.03, -0.06, -0.12] {
                for (ride, dz) in [(false, 0.0), (false, 0.5 * (top.1 - top.0)), (true, 0.0)] {
                    let n = [0.0, 0.0, 1.0];
                    let d = Draw { a, off, dz, ride, h: 0.5 * (top.0 + top.1), r: 0.5 * (radius.0 + radius.1), n };
                    let r = over(top, radius, |t, r, band, tol| {
                        let (body, faces) = cylinder::<Boxed>(r, t, false, tol);
                        let iv = Boxed::from_f64;
                        topo::test_support::ray_at(&body, faces[0], aim_of(&d, t, r), Vec3::new(iv(n[0]), iv(n[1]), iv(n[2])), band)
                    });
                    let members: Vec<&str> = [0.0, 0.25, 0.5, 0.75, 1.0].iter().map(|&u| {
                        let (t, r) = (top.0 + u * (top.1 - top.0), radius.0 + u * (radius.1 - radius.0));
                        let (body, faces) = cylinder::<f64>(r, t, false, ftol);
                        kind(&topo::test_support::ray_at(&body, faces[0], aim_of(&d, t, r), Vec3::new(n[0], n[1], n[2]), fband))
                    }).collect();
                    let pred = r.split("predicate: Some(\"").nth(1).unwrap_or("").split('"').next().unwrap_or("").to_owned();
                    let all_out = members.iter().all(|m| *m == "out");
                    *rows.entry(format!("{} {pred} members_all_out={all_out}", kind(&r))).or_default() += 1;
                    if all_out && pred == "curve_carrier_gap" { println!("  PICK top {top:?} r {radius:?} a {a} off {off} ride {ride} dz {dz}"); }
                }
            }
        }
        println!("top {top:?} r {radius:?}: {rows:?}");
    }
}
