//! Reviewer probe (dual r2, PR 3982): the split gate's per-face reach in
//! the plane's frame, against an oracle that owes nothing to the kernel.
//! Fixtures beyond the PR's own: a lollipop (sphere zone larger than a
//! hemisphere), a shallow dome, a CONCAVE torus fillet, a whole donut, a
//! spindle torus (R < r: the rule's box must stand), a big complement
//! zone and a two-rim band. Random directions, random rigid poses, three
//! scales; meeting splits are counted (soundness), clear refusals are
//! counted per pose (invariance), and every split half is checked
//! against an adaptive-Simpson volume and sampled point_in_solid.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Band, Point2, Point3, Tol, UnitVec3, Vec3};
use profile::{ArcSweep, RawLoop, bulge_from_center, test_support::bulge_loop};
use sweep::{Revolution, revolve};
use topo::splitting::{SplitError, SplitPart, SplitPlane, SplitReduceError, split};
use topo::{Body, DATUM_UNIT_NORM, SolidContainment, point_in_solid, transform_rigid};

/// An unarmed face's meridian arc in `(ρ, y)`: centre, radius, angle
/// window (radians, `lo < hi`).
#[derive(Clone, Copy)]
struct Arc {
    c: (f64, f64),
    r: f64,
    a: (f64, f64),
}

struct Fix {
    name: &'static str,
    chain: Vec<(Point2<f64>, f64)>,
    joints: Vec<usize>,
    /// Profile material at height `y`, as ρ-intervals (scale 1).
    region: fn(f64) -> Vec<(f64, f64)>,
    ylim: (f64, f64),
    breaks: Vec<f64>,
    faces: Vec<Arc>,
}

fn sq(x: f64) -> f64 {
    x * x
}

fn arcpt(c: (f64, f64), r: f64, t: f64) -> (f64, f64) {
    (c.0 + r * t.cos(), c.1 + r * t.sin())
}

fn fixtures() -> Vec<Fix> {
    let yc_l = 1.0 + 0.91f64.sqrt();
    let yc_d = 1.0 - 8f64.sqrt();
    let p = |x: f64, y: f64| Point2::new(x, y);
    let b = |a: Point2<f64>, e: Point2<f64>, c: Point2<f64>, s| bulge_from_center(a, e, c, s);
    vec![
        Fix {
            name: "lollipop",
            chain: vec![
                (p(0.0, 0.0), 0.0),
                (p(0.3, 0.0), 0.0),
                (p(0.3, 1.0), b(p(0.3, 1.0), p(0.0, yc_l + 1.0), p(0.0, yc_l), ArcSweep::Ccw)),
                (p(0.0, yc_l + 1.0), 0.0),
            ],
            joints: vec![],
            region: |y| {
                let yc = 1.0 + 0.91f64.sqrt();
                if y <= 1.0 { vec![(0.0, 0.3)] } else { vec![(0.0, (1.0 - sq(y - yc)).max(0.0).sqrt())] }
            },
            ylim: (0.0, yc_l + 1.0),
            breaks: vec![0.0, 1.0, yc_l, yc_l + 1.0],
            faces: vec![Arc { c: (0.0, yc_l), r: 1.0, a: ((1.0 - yc_l).atan2(0.3), PI / 2.0) }],
        },
        Fix {
            name: "shallow dome",
            chain: vec![
                (p(0.0, 0.0), 0.0),
                (p(1.0, 0.0), 0.0),
                (p(1.0, 1.0), b(p(1.0, 1.0), p(0.0, yc_d + 3.0), p(0.0, yc_d), ArcSweep::Ccw)),
                (p(0.0, yc_d + 3.0), 0.0),
            ],
            joints: vec![],
            region: |y| {
                let yc = 1.0 - 8f64.sqrt();
                if y <= 1.0 { vec![(0.0, 1.0)] } else { vec![(0.0, (9.0 - sq(y - yc)).max(0.0).sqrt())] }
            },
            ylim: (0.0, yc_d + 3.0),
            breaks: vec![0.0, 1.0, yc_d + 3.0],
            faces: vec![Arc { c: (0.0, yc_d), r: 3.0, a: ((1.0 - yc_d).atan2(1.0), PI / 2.0) }],
        },
        Fix {
            name: "concave fillet",
            chain: vec![
                (p(0.0, 0.0), 0.0),
                (p(1.0, 0.0), 0.0),
                (p(1.0, 1.0), 0.0),
                (p(0.7, 1.0), b(p(0.7, 1.0), p(0.5, 1.2), p(0.7, 1.2), ArcSweep::Cw)),
                (p(0.5, 1.2), 0.0),
                (p(0.5, 2.0), 0.0),
                (p(0.0, 2.0), 0.0),
            ],
            joints: vec![3, 4],
            region: |y| {
                if y <= 1.0 {
                    vec![(0.0, 1.0)]
                } else if y <= 1.2 {
                    vec![(0.0, 0.7 - (0.04 - sq(y - 1.2)).max(0.0).sqrt())]
                } else {
                    vec![(0.0, 0.5)]
                }
            },
            ylim: (0.0, 2.0),
            breaks: vec![0.0, 1.0, 1.2, 2.0],
            faces: vec![Arc { c: (0.7, 1.2), r: 0.2, a: (PI, 1.5 * PI) }],
        },
        Fix {
            name: "donut",
            chain: vec![(p(2.0, -0.5), 1.0), (p(2.0, 0.5), 1.0)],
            joints: vec![],
            region: |y| {
                let h = (0.25 - y * y).max(0.0).sqrt();
                vec![(2.0 - h, 2.0 + h)]
            },
            ylim: (-0.5, 0.5),
            breaks: vec![-0.5, 0.5],
            faces: vec![Arc { c: (2.0, 0.0), r: 0.5, a: (0.0, 2.0 * PI) }],
        },
        Fix {
            name: "near-spindle fillet",
            chain: vec![
                (p(0.0, 0.0), 0.0),
                (p(0.5, 0.0), 0.0),
                (p(0.5, 1.0), b(p(0.5, 1.0), p(0.26, 1.24), p(0.26, 1.0), ArcSweep::Ccw)),
                (p(0.26, 1.24), 0.0),
                (p(0.0, 1.24), 0.0),
            ],
            joints: vec![2, 3],
            region: |y| {
                if y <= 1.0 { vec![(0.0, 0.5)] } else { vec![(0.0, 0.26 + (0.0576 - sq(y - 1.0)).max(0.0).sqrt())] }
            },
            ylim: (0.0, 1.24),
            breaks: vec![0.0, 1.0, 1.24],
            faces: vec![Arc { c: (0.26, 1.0), r: 0.24, a: (0.0, PI / 2.0) }],
        },
        Fix {
            name: "big complement zone",
            chain: vec![
                (p(0.0, -1.0), b(p(0.0, -1.0), p(0.6, 0.8), p(0.0, 0.0), ArcSweep::Ccw)),
                (p(0.6, 0.8), 0.0),
                (p(0.0, 0.8), 0.0),
            ],
            joints: vec![],
            region: |y| vec![(0.0, (1.0 - y * y).max(0.0).sqrt())],
            ylim: (-1.0, 0.8),
            breaks: vec![-1.0, 0.8],
            faces: vec![Arc { c: (0.0, 0.0), r: 1.0, a: (-PI / 2.0, 0.8f64.atan2(0.6)) }],
        },
        Fix {
            name: "two-rim band",
            chain: vec![
                (p(0.0, -0.9), 0.0),
                (p(0.19f64.sqrt(), -0.9), b(p(0.19f64.sqrt(), -0.9), p(0.0975f64.sqrt(), 0.95), p(0.0, 0.0), ArcSweep::Ccw)),
                (p(0.0975f64.sqrt(), 0.95), 0.0),
                (p(0.0, 0.95), 0.0),
            ],
            joints: vec![],
            region: |y| vec![(0.0, (1.0 - y * y).max(0.0).sqrt())],
            ylim: (-0.9, 0.95),
            breaks: vec![-0.9, 0.95],
            faces: vec![Arc { c: (0.0, 0.0), r: 1.0, a: ((-0.9f64).atan2(0.19f64.sqrt()), 0.95f64.atan2(0.0975f64.sqrt())) }],
        },
    ]
}

fn build(f: &Fix, s: f64) -> Result<Body<f64>, String> {
    let chain = f.chain.iter().map(|(q, bl)| (Point2::new(q.x * s, q.y * s), *bl)).collect();
    revolve(
        &validated(vec![bulge_loop(chain).with_tangent_joints(f.joints.clone())]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .map(|r| r.body)
    .map_err(|e| format!("{e:?}"))
}

/// Max of `w·q` over the arc, exactly.
fn arc_max(a: &Arc, w: (f64, f64)) -> f64 {
    let crest = w.1.atan2(w.0);
    let k = ((a.a.0 - crest) / (2.0 * PI)).ceil();
    let inside = crest + 2.0 * PI * k <= a.a.1;
    let at = |t: f64| {
        let q = arcpt(a.c, a.r, t);
        w.0 * q.0 + w.1 * q.1
    };
    if inside { w.0 * a.c.0 + w.1 * a.c.1 + a.r * (w.0 * w.0 + w.1 * w.1).sqrt() } else { at(a.a.0).max(at(a.a.1)) }
}

/// The revolved faces' support `(lo, hi)` along body-frame unit `n` (scale 1).
fn support(f: &Fix, n: Vec3<f64>) -> (f64, f64) {
    let m = (sq(n.x) + sq(n.z)).sqrt();
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for a in &f.faces {
        // over azimuth, n·p ∈ ny·y ± m·ρ, and ρ ≥ 0 on these faces
        hi = hi.max(arc_max(a, (m, n.y)));
        lo = lo.min(-arc_max(a, (m, -n.y)));
    }
    (lo, hi)
}

/// Area of the part of the disk of radius `rho` with `x > t`.
fn seg(rho: f64, t: f64) -> f64 {
    if t >= rho {
        0.0
    } else if t <= -rho {
        PI * rho * rho
    } else {
        rho * rho * (t / rho).acos() - t * (rho * rho - t * t).sqrt()
    }
}

/// Volume (scale 1) of the body on `n·p > d`, adaptive Simpson in y.
fn half_volume(f: &Fix, n: Vec3<f64>, d: f64) -> f64 {
    let m = (sq(n.x) + sq(n.z)).sqrt();
    let area = |y: f64| -> f64 {
        (f.region)(y)
            .iter()
            .map(|&(a, b)| {
                if m < 1e-14 {
                    if n.y * y > d { PI * (b * b - a * a) } else { 0.0 }
                } else {
                    let t = (d - n.y * y) / m;
                    seg(b, t) - seg(a, t)
                }
            })
            .sum()
    };
    let mut br = f.breaks.clone();
    if m < 1e-14 {
        let c = d / n.y;
        if c > f.ylim.0 && c < f.ylim.1 {
            br.push(c);
        }
    }
    br.sort_by(f64::total_cmp);
    fn simpson(f: &dyn Fn(f64) -> f64, a: f64, b: f64, fa: f64, fm: f64, fb: f64, whole: f64, tol: f64, depth: u32) -> f64 {
        let m = 0.5 * (a + b);
        let (lm, rm) = (0.5 * (a + m), 0.5 * (m + b));
        let (flm, frm) = (f(lm), f(rm));
        let left = (m - a) / 6.0 * (fa + 4.0 * flm + fm);
        let right = (b - m) / 6.0 * (fm + 4.0 * frm + fb);
        if depth == 0 || (left + right - whole).abs() <= 15.0 * tol {
            return left + right + (left + right - whole) / 15.0;
        }
        simpson(f, a, m, fa, flm, fm, left, tol / 2.0, depth - 1) + simpson(f, m, b, fm, frm, fb, right, tol / 2.0, depth - 1)
    }
    let mut v = 0.0;
    for w in br.windows(2) {
        let (a, b) = (w[0], w[1]);
        if b <= a {
            continue;
        }
        let (fa, fm, fb) = (area(a), area(0.5 * (a + b)), area(b));
        v += simpson(&area, a, b, fa, fm, fb, (b - a) / 6.0 * (fa + 4.0 * fm + fb), 1e-12, 40);
    }
    v
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn dir(&mut self) -> Vec3<f64> {
        let z = 2.0 * self.next() - 1.0;
        let t = 2.0 * PI * self.next();
        let r = (1.0 - z * z).sqrt();
        Vec3::new(r * t.cos(), z, r * t.sin())
    }
}

fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    let band = Band::linear(Tol::witness()).unwrap();
    UnitVec3::new(v * (1.0 / v.norm()), DATUM_UNIT_NORM, band).unwrap()
}

/// Oracle membership at body-frame point (scale 1): Some(in) when the
/// point is at least `m` from the profile boundary, else None.
fn member(f: &Fix, q: Point3<f64>, mg: f64) -> Option<bool> {
    let rho = (sq(q.x) + sq(q.z)).sqrt();
    let inside = |r: f64, y: f64| y >= f.ylim.0 && y <= f.ylim.1 && (f.region)(y).iter().any(|&(a, b)| r >= a && r <= b);
    let c = inside(rho, q.y);
    for (dr, dy) in [(mg, 0.0), (-mg, 0.0), (0.0, mg), (0.0, -mg), (mg, mg), (-mg, -mg), (mg, -mg), (-mg, mg)] {
        if inside((rho + dr).max(0.0), q.y + dy) != c {
            return None;
        }
    }
    Some(c)
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Tally {
    clear_split: usize,
    clear_gate: usize,
    clear_other: usize,
    meet_split: usize,
    meet_gate: usize,
    meet_other: usize,
    wrong_vol: usize,
    props_refused: usize,
    pis_wrong: usize,
    pis_checked: usize,
    resplit_ok: usize,
    resplit_wrong: usize,
}

fn poses(s: f64, rng: &mut Rng) -> Vec<(&'static str, Affine3<f64>)> {
    let o = Point3::new(0.0, 0.0, 0.0);
    let (a1, t1) = (rng.dir(), 2.0 * PI * rng.next());
    let (a2, t2) = (rng.dir(), 2.0 * PI * rng.next());
    vec![
        ("upright", Affine3::identity()),
        ("rand1", Affine3::rotation_about_axis(o, a1, t1)),
        (
            "rand2+t",
            Affine3::translation(Vec3::new(-1.7 * s, 0.4 * s, 2.3 * s)) * Affine3::rotation_about_axis(o, a2, t2),
        ),
    ]
}

/// Check one split half against volume + PIS oracles. Returns (wrong_vol, props_refused, pis_checked, pis_wrong).
fn check_half(f: &Fix, s: f64, part: &SplitPart<f64>, map: &Affine3<f64>, n: Vec3<f64>, d: f64, rng: &mut Rng, pis: usize, tag: &str) -> (usize, usize, usize, usize) {
    let oracle = half_volume(f, n, d / s) * s * s * s;
    let SplitPart::Body(b) = part else {
        let w = (oracle.abs() > 1e-9 * s * s * s) as usize;
        if w > 0 {
            println!("WRONG-EMPTY {tag}: oracle {oracle}");
        }
        return (w, 0, 0, 0);
    };
    let (mut wv, mut pr) = (0, 0);
    match topo::props::mass_properties(b, Tol::witness()) {
        Ok(p) => {
            if (p.volume - oracle).abs() > p.volume_pad + 1e-7 * s * s * s {
                wv = 1;
                println!("WRONG-VOL {tag}: {} ± {} vs {oracle}", p.volume, p.volume_pad);
            }
        }
        Err(e) => {
            pr = 1;
            println!("PROPS {tag}: {e}");
        }
    }
    let band = Band::linear(Tol::witness()).unwrap();
    let (mut ck, mut pw) = (0, 0);
    let mut tries = 0;
    let mut refused = 0usize;
    while ck < pis && refused < 3 && tries < 40 * pis.max(1) {
        tries += 1;
        let q = Point3::new((rng.next() * 6.0 - 3.0) * s, (rng.next() * 4.5 - 1.5) * s, (rng.next() * 6.0 - 3.0) * s);
        let off = n.x * q.x + n.y * q.y + n.z * q.z - d;
        if off.abs() < 1e-3 * s {
            continue;
        }
        let Some(inb) = member(f, Point3::new(q.x / s, q.y / s, q.z / s), 1e-3) else { continue };
        let want = inb && off > 0.0;
        let got = point_in_solid(b, map.transform_point(q), band, Tol::witness());
        ck += 1;
        match got {
            Ok(SolidContainment::In) if want => {}
            Ok(SolidContainment::Out) if !want => {}
            Err(_) => {
                ck -= 1;
                refused += 1;
            }
            other => {
                pw += 1;
                println!("PIS {tag}: q={q:?} want_in={want} got {other:?}");
            }
        }
    }
    (wv, pr, ck, pw)
}

fn run(scales: &[f64], ndir: usize, pis: usize) {
    run_with(scales, ndir, pis, &[3e-2, 1e-3, 1e-5], &[1e-5, 1e-3]);
}

/// `clear`: gaps (×s) outside the face's least support, and 1e-3 above its greatest;
/// `into`: depths (×s) into the face from both ends.
fn run_with(scales: &[f64], ndir: usize, pis: usize, clear: &[f64], into: &[f64]) {
    let tol = Tol::witness();
    println!("eps = {:e}", tol.eps());
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for &s in scales {
        for f in fixtures() {
            let body = match build(&f, s) {
                Ok(b) => b,
                Err(e) => {
                    println!("BUILD-REFUSED {} s={s}: {e}", f.name);
                    continue;
                }
            };
            let mut dirs: Vec<Vec3<f64>> = (0..ndir).map(|_| rng.dir()).collect();
            dirs.push(Vec3::new(0.0, 1.0, 0.0));
            dirs.push(Vec3::new(0.0, -1.0, 0.0));
            let mut per_pose = Vec::new();
            for (pose, map) in poses(s, &mut rng) {
                let posed = match transform_rigid(&body, &map, tol) {
                    Ok(p) => p,
                    Err(e) => {
                        println!("POSE-REFUSED {} s={s} {pose}: {e}", f.name);
                        continue;
                    }
                };
                let mut t = Tally::default();
                for n in &dirs {
                    let (lo, hi) = support(&f, *n);
                    let (lo, hi) = (lo * s, hi * s);
                    let mut cuts: Vec<(f64, f64)> = Vec::new();
                    for &g in clear {
                        cuts.push((lo - g * s, g));
                    }
                    cuts.push((hi + 1e-3 * s, 1e-3));
                    for &g in into {
                        cuts.push((lo + g * s, -g));
                        cuts.push((hi - g * s, -g));
                    }
                    for (d, gap) in cuts {
                        let clear = gap > 0.0;
                        let q = Point3::new(n.x * d, n.y * d, n.z * d);
                        let plane = SplitPlane { origin: map.transform_point(q), normal: unit(map.transform_vec(*n)) };
                        let tag = format!("{} s={s} {pose} n={n:?} gap={gap} d={d} lo={lo} hi={hi}", f.name);
                        match split(&posed, &plane, tol) {
                            Ok(res) => {
                                if clear {
                                    t.clear_split += 1;
                                } else {
                                    t.meet_split += 1;
                                    println!("MEET-SPLIT {tag}");
                                }
                                for (part, sg) in [(&res.above, 1.0), (&res.below, -1.0)] {
                                    let (a, b, c, e) = check_half(&f, s, part, &map, *n * sg, d * sg, &mut rng, pis, &tag);
                                    t.wrong_vol += a;
                                    t.props_refused += b;
                                    t.pis_checked += c;
                                    t.pis_wrong += e;
                                }
                                // reuse: re-split the half holding the face by a
                                // second cut between the first and the face, still clear
                                if clear && gap >= 1e-3 {
                                    let lo_side = d < lo;
                                    let (half, d2, sg) = if lo_side {
                                        (&res.above, d + 0.5 * (lo - d), 1.0)
                                    } else {
                                        (&res.below, d - 0.5 * (d - hi), -1.0)
                                    };
                                    if let SplitPart::Body(h) = half {
                                        let q2 = Point3::new(n.x * d2, n.y * d2, n.z * d2);
                                        let p2 = SplitPlane { origin: map.transform_point(q2), normal: unit(map.transform_vec(*n)) };
                                        match split(h, &p2, tol) {
                                            Ok(r2) => {
                                                let part = if lo_side { &r2.above } else { &r2.below };
                                                let (a, ..) = check_half(&f, s, part, &map, *n * sg, d2 * sg, &mut rng, 0, &format!("{tag} resplit"));
                                                if a == 0 { t.resplit_ok += 1 } else { t.resplit_wrong += 1 }
                                            }
                                            Err(e) => println!("RESPLIT-REFUSED {tag}: {e}"),
                                        }
                                    }
                                }
                            }
                            Err(SplitError::Reduce(SplitReduceError::CurvedBooleanUnsupported { .. })) => {
                                if clear { t.clear_gate += 1 } else { t.meet_gate += 1 }
                            }
                            Err(e) => {
                                if clear {
                                    t.clear_other += 1;
                                    println!("CLEAR-OTHER {tag}: {e}");
                                } else {
                                    t.meet_other += 1;
                                }
                            }
                        }
                    }
                }
                println!("ROW | {} | {s:e} | {pose} | {t:?}", f.name);
                per_pose.push((pose, t));
            }
            let gate: Vec<usize> = per_pose.iter().map(|(_, t)| t.clear_gate).collect();
            println!("INVARIANCE | {} | {s:e} | clear-refused per pose {gate:?}", f.name);
        }
    }
}

#[test]
#[ignore = "reviewer probe"]
fn r2_probe_full() {
    run(&[1e-3, 1.0, 1e3], 10, 4);
}

#[test]
#[ignore = "reviewer probe"]
fn r2_probe_fine() {
    run_with(&[1.0], 10, 0, &[1e-6, 1e-7], &[1e-6, 1e-7]);
}

#[test]
#[ignore = "reviewer probe"]
fn r2_probe_unit() {
    run(&[1.0], 10, 4);
}

/// The lollipop cut 1e-6 below its pole, square to its axis: the plane
/// meets the sphere face in a circle of radius ≈ 1.4e-3 about the pole.
#[test]
#[ignore = "reviewer probe"]
fn r2_probe_pole() {
    let f = &fixtures()[0];
    let body = build(f, 1.0).unwrap();
    let pole = 1.0 + 0.91f64.sqrt() + 1.0;
    for (n, y) in [(Vec3::new(0.0, 1.0, 0.0), pole - 1e-6), (Vec3::new(0.0, 1.0, 0.0), 1.0 + 1e-5), (Vec3::new(0.0, 1.0, 0.0), 1.0 + 1e-6), (Vec3::new(0.0, 1.0, 0.0), 1.0 + 1e-7), (Vec3::new(0.0, 1.0, 0.0), 1.0 + 1e-8), (Vec3::new(0.0, -1.0, 0.0), 1.0 + 1e-6), (Vec3::new(0.0, 1.0, 0.0), 1.0 + 3e-6), (Vec3::new(0.0, 1.0, 0.0), 1.0 + 6e-6)] {
        let dy = y;
        let q = Point3::new(0.0, y, 0.0);
        let plane = SplitPlane { origin: q, normal: unit(n) };
        match split(&body, &plane, Tol::witness()) {
            Ok(r) => {
                for (nm, p) in [("above", &r.above), ("below", &r.below)] {
                    match p {
                        SplitPart::Empty => println!("POLE n={n:?} dy={dy}: {nm} EMPTY"),
                        SplitPart::Body(b) => {
                            let v = topo::props::mass_properties(b, Tol::witness()).map(|p| p.volume);
                            println!("POLE n={n:?} dy={dy}: {nm} faces={} vol={v:?}", b.faces().count());
                        }
                    }
                }
            }
            Err(e) => println!("POLE n={n:?} dy={dy}: refused {e}"),
        }
    }
    println!("whole faces={} vol={:?}", body.faces().count(), topo::props::mass_properties(&body, Tol::witness()).map(|p| p.volume));
}
