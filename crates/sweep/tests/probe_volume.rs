//! PROBE (boxfinish/probe-volume-reading, never merged): check 7's
//! volume reading over a box, term by term. Every test prints and
//! asserts nothing; run one with `--nocapture`, the trace on with
//! `BOXFINISH_VOL_TRACE=1`.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;
use std::time::Instant;

use geom_core::{
    Bounds, CertifiedBounds, Decide, Interval, ParamSymbol, Point2, Point3, Real, Sym, SymBudget,
    Tol, Vec3,
};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude, loft_body};
use topo::Body;

type SI = Sym<Interval>;
type P = Point3<Interval>;
type V = Vec3<Interval>;

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 64,
        max_degree: 8,
    }
}

fn sym_box(name: &str, lo: f64, hi: f64) -> SI {
    if lo == hi {
        return SI::from_f64(lo);
    }
    Sym::param_over(
        ParamSymbol::new(test_utils::symbol_id(name)),
        Interval::from_bounds(lo, hi),
        lo,
        hi,
    )
}

/// A polygon `pts` (shifted by `shift` on every axis, its plane at
/// `z = shift`) extruded along `+z` by `depth`.
fn prism<T: Decide + topo::AtRestPolicy>(
    pts: &[(T, T)],
    depth: T,
    shift: f64,
    tol: Tol,
) -> Result<T::Leaf, String> {
    let s = T::from_f64(shift);
    let chain = pts
        .iter()
        .map(|&(x, y)| (Point2::new(x + s, y + s), T::zero()))
        .collect();
    let plane = SketchPlane::xy_at(Point3::new(T::zero(), T::zero(), s));
    let profile = Profile::new(plane, vec![bulge_loop(chain)])
        .validate(tol)
        .map_err(|e| format!("profile: {e:?}"))?;
    extrude(
        &profile,
        Extrusion::Distance {
            depth,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .map(|e| e.body)
    .map_err(|e| format!("extrude: {e:?}"))
}

fn square<T: Real>() -> Vec<(T, T)> {
    let c = T::from_f64;
    vec![(c(0.0), c(0.0)), (c(1.0), c(0.0)), (c(1.0), c(1.0)), (c(0.0), c(1.0))]
}

fn slab<T: Decide + topo::AtRestPolicy>(depth: T, shift: f64, tol: Tol) -> Result<T::Leaf, String> {
    prism(&square::<T>(), depth, shift, tol)
}

fn lift<T: CertifiedBounds>(p: Point3<T>) -> P {
    Point3::new(
        Interval::from_certified(p.x),
        Interval::from_certified(p.y),
        Interval::from_certified(p.z),
    )
}
fn liftv<T: CertifiedBounds>(p: Vec3<T>) -> V {
    Vec3::new(
        Interval::from_certified(p.x),
        Interval::from_certified(p.y),
        Interval::from_certified(p.z),
    )
}
fn iv(x: Interval) -> String {
    format!("[{:.6}, {:.6}] w={:.3e}", x.lo(), x.hi(), x.hi() - x.lo())
}
fn third(x: Interval) -> Interval {
    x / Interval::from_f64(3.0)
}

/// `polygon_face_about`'s arithmetic, by hand: (plane flux, tilt slack).
fn code_term(rings: &[Vec<P>], n: V, c: P) -> (Interval, Interval) {
    let pts: Vec<P> = rings.iter().flatten().copied().collect();
    let l = pts[1..].iter().fold(pts[0], |lo, p| {
        Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z))
    });
    let (sum, k) = pts
        .iter()
        .fold((V::zero(), Interval::zero()), |(s, k), &p| (s + (p - l), k + Interval::one()));
    let offset = sum * (Interval::one() / k);
    let mut va = V::zero();
    for ring in rings {
        for (i, &p) in ring.iter().enumerate() {
            let q = ring[(i + 1) % ring.len()];
            va = va + (p - l).cross(q - l) * Interval::from_f64(0.5);
        }
    }
    let lever = (l - c) + offset;
    let flux = lever.dot(n) * va.dot(n) / n.dot(n);
    let across = va - n * (va.dot(n) / n.dot(n));
    let b = (lever.norm() * across.norm()).hi();
    (flux, Interval::from_bounds(-b, b))
}

fn det<T: Real>(a: Vec3<T>, b: Vec3<T>, c: Vec3<T>) -> T {
    a.dot(b.cross(c))
}

/// The fan sum `(1/6) Σ det(r0 − c, ri − c, ri+1 − c)` at any scalar.
fn fan<T: Real>(faces: &[Vec<Vec<Point3<T>>>], c: Point3<T>) -> T {
    let mut six = T::zero();
    for rings in faces {
        for ring in rings {
            for i in 1..ring.len() - 1 {
                six = six + det(ring[0] - c, ring[i] - c, ring[i + 1] - c);
            }
        }
    }
    six / T::from_f64(6.0)
}

/// Every hand re-posing of the polygon route's volume on `body`.
fn reposings<T: Decide + CertifiedBounds>(body: &Body<T>, exact_point: [f64; 3]) {
    let Some((centre, faces)) = topo::props::probe_plane_shell(body) else {
        println!("  (no plane shell: no re-posing)");
        return;
    };
    let c_code = lift(centre);
    let pt = |a: [f64; 3]| Point3::new(Interval::from_f64(a[0]), Interval::from_f64(a[1]), Interval::from_f64(a[2]));
    let lifted: Vec<(Vec<Vec<P>>, V)> = faces
        .iter()
        .map(|(rings, n, _)| (rings.iter().map(|r| r.iter().map(|&p| lift(p)).collect()).collect(), liftv(*n)))
        .collect();
    let sum = |c: P, slack: bool| {
        lifted.iter().fold(Interval::zero(), |acc, (rings, n)| {
            let (f, s) = code_term(rings, *n, c);
            acc + if slack { f + s } else { f }
        })
    };
    println!("  centre the code takes: ({:?}, {:?}, {:?})", (c_code.x.lo(), c_code.x.hi()), (c_code.y.lo(), c_code.y.hi()), (c_code.z.lo(), c_code.z.hi()));
    println!("  R0 code's formula, code's centre, with tilt slack : {}", iv(third(sum(c_code, true))));
    println!("  R1 same, tilt slack dropped                       : {}", iv(third(sum(c_code, false))));
    println!("  R2 about the world origin, with slack             : {}", iv(third(sum(pt([0.0; 3]), true))));
    println!("  R2' about the world origin, slack dropped         : {}", iv(third(sum(pt([0.0; 3]), false))));
    println!("  R3 about an exact point of the body {exact_point:?}, with slack: {}", iv(third(sum(pt(exact_point), true))));
    println!("  R3' about that exact point, slack dropped         : {}", iv(third(sum(pt(exact_point), false))));
    let rings_only: Vec<Vec<Vec<P>>> = lifted.iter().map(|(r, _)| r.clone()).collect();
    println!("  R4 fan sum det(r0,ri,ri+1)/6 about the world origin: {}", iv(fan(&rings_only, pt([0.0; 3]))));
    println!("  R5 fan sum about the exact body point             : {}", iv(fan(&rings_only, pt(exact_point))));
    println!("  R5' fan sum about the code's (boxed) centre       : {}", iv(fan(&rings_only, c_code)));
    // R6: the caps paired. The two faces whose normal is along z: their
    // fluxes about any centre sum to (anchor_top − anchor_bottom)·n × A,
    // the centre cancelled by hand; the walls as R3'.
    let mut walls = Interval::zero();
    let mut caps: Vec<(P, V, Interval)> = Vec::new();
    for (rings, n) in &lifted {
        if n.z.lo().abs() > 0.5 {
            let pts: Vec<P> = rings.iter().flatten().copied().collect();
            let l = pts[1..].iter().fold(pts[0], |lo, p| Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z)));
            let mut va = V::zero();
            for ring in rings {
                for (i, &p) in ring.iter().enumerate() {
                    let q = ring[(i + 1) % ring.len()];
                    va = va + (p - l).cross(q - l) * Interval::from_f64(0.5);
                }
            }
            caps.push((l, *n, va.dot(*n) / n.dot(*n)));
        } else {
            walls = walls + code_term(rings, *n, pt(exact_point)).0;
        }
    }
    if caps.len() == 2 {
        let (top, bot) = if caps[0].1.z.lo() > 0.0 { (&caps[0], &caps[1]) } else { (&caps[1], &caps[0]) };
        let paired = (top.0 - bot.0).dot(top.1) * top.2;
        println!("  R6 caps paired ((top−bottom)·n × A), walls as R3'  : {}  [caps pair alone {}, walls {}]", iv(third(paired + walls)), iv(third(paired)), iv(third(walls)));
    }
}

fn report<T: Decide + CertifiedBounds>(label: &str, body: &Body<T>, tol: Tol) {
    println!("== {label}");
    for line in topo::props::probe_volume_reading(body, tol) {
        println!("  READING {line}");
    }
}

const DEPTHS: [(f64, f64); 4] = [(0.75, 1.25), (0.95, 1.05), (0.99, 1.01), (1.0, 1.0)];

/// Item 1 and 2: the slab's reading at `Sym<Interval>`, `Interval` and
/// `f64` members, with the hand re-posings.
#[test]
fn item1_slab_terms() {
    let tol = Tol::witness();
    for (lo, hi) in DEPTHS {
        println!("\n######## slab, depth [{lo}, {hi}]");
        geom_core::sym::with_session(budget(), || {
            match slab::<SI>(sym_box("slab_depth", lo, hi), 0.0, tol) {
                Ok(leaf) => {
                    report("Sym<Interval>", &leaf, tol);
                    reposings(&*leaf, [0.0, 0.0, 0.0]);
                    // The symbolic channel: the fan sum formed at Sym<Interval>.
                    if let Some((_, faces)) = topo::props::probe_plane_shell(&*leaf) {
                        let rings: Vec<Vec<Vec<Point3<SI>>>> = faces.iter().map(|f| f.0.clone()).collect();
                        let v = fan(&rings, Point3::origin());
                        println!("  R8 fan sum formed at Sym<Interval>: value channel [{:.6}, {:.6}], form = {:?}", v.lo(), v.hi(), geom_core::sym::report::render_of(v.node()));
                        for (k, f) in faces.iter().enumerate() {
                            let z: Vec<String> = f.0[0].iter().map(|p| format!("{:?}", geom_core::sym::report::render_of(p.z.node()))).collect();
                            println!("    face {k}: vertex z forms {}", z.join(" "));
                        }
                    }
                }
                Err(e) => println!("== Sym<Interval>: {e}"),
            }
        });
        match slab::<Interval>(Interval::from_bounds(lo, hi), 0.0, tol) {
            Ok(leaf) => {
                report("Interval", &leaf, tol);
                reposings(&*leaf, [0.0, 0.0, 0.0]);
            }
            Err(e) => println!("== Interval: {e}"),
        }
        for d in [lo, 0.5 * (lo + hi), hi] {
            match slab::<f64>(d, 0.0, tol) {
                Ok(leaf) => report(&format!("f64 member depth {d}"), &leaf, tol),
                Err(e) => println!("== f64 {d}: {e}"),
            }
        }
    }
}

/// Item 3: the reading's width against the box's, at three placements.
#[test]
fn item3_scaling() {
    let tol = Tol::witness();
    println!("scalar shift halfwidth box_width reading_lo reading_hi reading_width ratio role");
    for shift in [0.0, 10.0, 1000.0] {
        for w in [1e-6, 1e-4, 1e-2, 0.05, 0.25] {
            let (lo, hi) = (1.0 - w, 1.0 + w);
            let line = |scalar: &str, got: Result<Vec<String>, String>| match got {
                Ok(lines) => {
                    let r = topo::props::probe_last_reading();
                    let role = lines[0].split("role=").nth(1).unwrap_or("").chars().take(60).collect::<String>();
                    match r {
                        Some((a, b)) => println!("{scalar} {shift} {w:e} {:e} {a:.9} {b:.9} {:.4e} {:.4} {role}", hi - lo, b - a, (b - a) / (hi - lo)),
                        None => println!("{scalar} {shift} {w:e} no reading: {}", lines[0]),
                    }
                }
                Err(e) => println!("{scalar} {shift} {w:e} BUILD REFUSED {}", e.chars().take(200).collect::<String>()),
            };
            geom_core::sym::with_session(budget(), || {
                line("Sym<Interval>", slab::<SI>(sym_box("slab_depth", lo, hi), shift, tol).map(|leaf| topo::props::probe_volume_reading(&*leaf, tol)));
            });
            line("Interval", slab::<Interval>(Interval::from_bounds(lo, hi), shift, tol).map(|leaf| topo::props::probe_volume_reading(&*leaf, tol)));
        }
    }
}

/// Item 3, the re-posings at a far placement.
#[test]
fn item3_far_reposings() {
    let tol = Tol::witness();
    for shift in [10.0, 1000.0] {
        for w in [1e-4, 0.05] {
            println!("\n######## slab shifted {shift}, depth 1 ± {w}");
            geom_core::sym::with_session(budget(), || {
                match slab::<SI>(sym_box("slab_depth", 1.0 - w, 1.0 + w), shift, tol) {
                    Ok(leaf) => {
                        report("Sym<Interval>", &leaf, tol);
                        reposings(&*leaf, [shift, shift, shift]);
                    }
                    Err(e) => println!("== Sym<Interval>: {e}"),
                }
            });
        }
    }
}

/// A lens of two arcs of bulge `b` between `(−1, 0)` and `(1, 0)`.
fn lens<T: Real>(b: T) -> ProfileLoop<T> {
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    bulge_loop(vec![(p(-1.0, 0.0), b), (p(1.0, 0.0), b)])
}

fn lens_prism<T: Decide + topo::AtRestPolicy>(b: f64, depth: T, tol: Tol) -> Result<T::Leaf, String> {
    let profile = Profile::new(SketchPlane::xy(), vec![lens(T::from_f64(b))])
        .validate(tol)
        .map_err(|e| format!("profile: {e:?}"))?;
    extrude(&profile, Extrusion::Distance { depth, side: ExtrudeSide::Along }, tol)
        .map(|e| e.body)
        .map_err(|e| format!("extrude: {e:?}"))
}

/// Item 4a: a two-arc extrusion (bulge 1: a circle in two halves; bulge
/// 0.5: a lens) over a box on its depth.
#[test]
fn item4_arcs() {
    let tol = Tol::witness();
    for b in [1.0, 0.5] {
        for (lo, hi) in [(0.95, 1.05), (0.99, 1.01), (1.0, 1.0)] {
            println!("\n######## two-arc prism bulge {b}, depth [{lo}, {hi}]");
            geom_core::sym::with_session(budget(), || match lens_prism::<SI>(b, sym_box("lens_depth", lo, hi), tol) {
                Ok(leaf) => report("Sym<Interval>", &leaf, tol),
                Err(e) => println!("== Sym<Interval>: {}", e.chars().take(400).collect::<String>()),
            });
            match lens_prism::<Interval>(b, Interval::from_bounds(lo, hi), tol) {
                Ok(leaf) => report("Interval", &leaf, tol),
                Err(e) => println!("== Interval: {}", e.chars().take(400).collect::<String>()),
            }
            for d in [lo, hi] {
                match lens_prism::<f64>(b, d, tol) {
                    Ok(leaf) => report(&format!("f64 member depth {d}"), &leaf, tol),
                    Err(e) => println!("== f64: {}", e.chars().take(400).collect::<String>()),
                }
            }
        }
    }
}

/// The square `[−1, 1]²` whose right side is an arc of bulge `b`.
fn arc_square<T: Real>(b: T) -> Vec<ProfileLoop<T>> {
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    vec![bulge_loop(vec![
        (p(-1.0, -1.0), T::zero()),
        (p(1.0, -1.0), b),
        (p(1.0, 1.0), T::zero()),
        (p(-1.0, 1.0), T::zero()),
    ])]
}

fn loft2<T: Decide + topo::AtRestPolicy>(b: f64, top: T, tol: Tol) -> Result<T::Leaf, String> {
    let places = [
        SketchPlane::xy(),
        SketchPlane::xy_at(Point3::new(T::zero(), T::zero(), top)),
    ];
    loft_body::<T>(&[arc_square(T::from_f64(b)), arc_square(T::from_f64(b))], &places, 1, tol)
        .map(|l| l.body)
        .map_err(|e| format!("loft: {e:?}"))
}

/// Item 4b: a loft (NURBS walls) whose top section's height is a box.
#[test]
fn item4_loft() {
    let tol = Tol::witness();
    for b in [0.0, 0.3] {
        for (lo, hi) in [(0.999, 1.001), (0.99, 1.01), (0.95, 1.05), (1.0, 1.0)] {
            println!("\n######## loft of two arc-squares (bulge {b}), top height [{lo}, {hi}]");
            geom_core::sym::with_session(budget(), || match loft2::<SI>(b, sym_box("loft_top", lo, hi), tol) {
                Ok(leaf) => report("Sym<Interval>", &leaf, tol),
                Err(e) => println!("== Sym<Interval>: {}", e.chars().take(400).collect::<String>()),
            });
            match loft2::<Interval>(b, Interval::from_bounds(lo, hi), tol) {
                Ok(leaf) => report("Interval", &leaf, tol),
                Err(e) => println!("== Interval: {}", e.chars().take(400).collect::<String>()),
            }
            for d in [lo, hi] {
                match loft2::<f64>(b, d, tol) {
                    Ok(leaf) => report(&format!("f64 member top {d}"), &leaf, tol),
                    Err(e) => println!("== f64: {}", e.chars().take(400).collect::<String>()),
                }
            }
        }
    }
}

/// Item 4c: a pentagon prism (seven faces) one of whose corners moves
/// over a box, so two walls turn; depth exact.
#[test]
fn item4_turning_wall() {
    let tol = Tol::witness();
    fn pent<T: Real>(x: T) -> Vec<(T, T)> {
        let c = T::from_f64;
        vec![(c(0.0), c(0.0)), (c(2.0), c(0.0)), (x, c(1.0)), (c(1.0), c(2.0)), (c(0.0), c(1.0))]
    }
    for w in [0.3, 0.05, 0.01] {
        let (lo, hi) = (2.5 - w, 2.5 + w);
        println!("\n######## pentagon prism, corner x in [{lo}, {hi}], depth 1");
        geom_core::sym::with_session(budget(), || match prism::<SI>(&pent(sym_box("corner_x", lo, hi)), SI::one(), 0.0, tol) {
            Ok(leaf) => {
                report("Sym<Interval>", &leaf, tol);
                reposings(&*leaf, [0.0, 0.0, 0.0]);
            }
            Err(e) => println!("== Sym<Interval>: {}", e.chars().take(400).collect::<String>()),
        });
        match prism::<Interval>(&pent(Interval::from_bounds(lo, hi)), Interval::one(), 0.0, tol) {
            Ok(leaf) => {
                report("Interval", &leaf, tol);
                reposings(&*leaf, [0.0, 0.0, 0.0]);
            }
            Err(e) => println!("== Interval: {}", e.chars().take(400).collect::<String>()),
        }
        for x in [lo, hi] {
            match prism::<f64>(&pent(x), 1.0, 0.0, tol) {
                Ok(leaf) => report(&format!("f64 member corner {x}"), &leaf, tol),
                Err(e) => println!("== f64: {e}"),
            }
        }
    }
}

/// Item 5: what a slab's finish costs at `Sym<Interval>`, by phase.
#[test]
fn item5_cost() {
    let tol = Tol::witness();
    for (lo, hi) in [(1.0, 1.0), (0.975, 1.025), (0.8, 1.2), (0.75, 1.25)] {
        for rep in 0..3 {
            let ((built, door_ms, take_ms, vol_ms), counts) = geom_core::sym::with_session(budget(), || {
                let t = Instant::now();
                let leaf = slab::<SI>(sym_box("slab_depth", lo, hi), 0.0, tol).expect("slab");
                let door_ms = t.elapsed().as_secs_f64() * 1e3;
                let before = geom_core::sym::session_counts().unwrap();
                let t = Instant::now();
                let lines = topo::props::probe_volume_reading(&*leaf, tol);
                let vol_ms = t.elapsed().as_secs_f64() * 1e3;
                let mid = geom_core::sym::session_counts().unwrap();
                println!("  volume reading alone: {vol_ms:.3} ms symbolic=+{} numeric=+{} :: {}", mid.symbolic - before.symbolic, mid.numeric - before.numeric, lines[0].chars().take(120).collect::<String>());
                let t = Instant::now();
                let taken = topo::take::<SI>(&Arc::new(leaf), tol).map(|_| ());
                let take_ms = t.elapsed().as_secs_f64() * 1e3;
                let after = geom_core::sym::session_counts().unwrap();
                println!("  finish (take): {take_ms:.3} ms symbolic=+{} numeric=+{} frozen={} -> {}", after.symbolic - mid.symbolic, after.numeric - mid.numeric, after.frozen, match &taken { Ok(()) => "finished".to_owned(), Err(e) => format!("REFUSED {}", format!("{e:?}").chars().take(300).collect::<String>()) });
                (taken.is_ok(), door_ms, take_ms, vol_ms)
            });
            println!("COST depth [{lo}, {hi}] rep {rep}: door {door_ms:.1} ms, volume-reading-alone {vol_ms:.2} ms, finish {take_ms:.1} ms, finished={built}, session totals symbolic={} numeric={} frozen={}", counts.symbolic, counts.numeric, counts.frozen);
        }
    }
}

/// Item 3: the half-width at which the slab's reading stops reading.
#[test]
fn item3_threshold() {
    let tol = Tol::witness();
    for w in [0.10, 0.14, 0.15, 0.155, 0.16, 0.17, 0.20, 0.215, 0.22] {
        geom_core::sym::with_session(budget(), || {
            let leaf = slab::<SI>(sym_box("slab_depth", 1.0 - w, 1.0 + w), 0.0, tol).expect("slab");
            let lines = topo::props::probe_volume_reading(&*leaf, tol);
            let rays = topo::test_support::aimed_rays(&*leaf, geom_core::Band::linear(tol).unwrap());
            println!("THRESH half-width {w}: reading {:?} decided={} rays={:?}", topo::props::probe_last_reading(), lines[0].contains("role=Ok"), rays.iter().map(|r| r.chars().take(60).collect::<String>()).collect::<Vec<_>>());
        });
    }
}

fn arc_prism<T: Decide + topo::AtRestPolicy>(halves: bool, depth: T, tol: Tol) -> Result<T::Leaf, String> {
    use geom_core::Arc2;
    use profile::{RawLoop, Segment};
    let c = T::from_f64;
    let arc = |sweep: f64| Segment::Arc(Arc2 { centre: Point2::new(c(0.0), c(0.0)), radius: c(1.0), sweep: c(sweep) });
    let rim = if halves {
        RawLoop::new(vec![
            (Point2::new(c(1.0), c(0.0)), arc(std::f64::consts::PI)),
            (Point2::new(c(-1.0), c(0.0)), arc(std::f64::consts::PI)),
        ])
    } else {
        RawLoop::new(vec![(Point2::new(c(1.0), c(0.0)), arc(std::f64::consts::TAU))])
    };
    let profile = Profile::new(SketchPlane::<T>::xy(), vec![rim])
        .validate(tol)
        .map_err(|e| format!("profile: {e:?}"))?;
    extrude(&profile, Extrusion::Distance { depth, side: ExtrudeSide::Along }, tol)
        .map(|e| e.body)
        .map_err(|e| format!("extrude: {e:?}"))
}

/// Item 4a': a circle (one full arc, or two half arcs with recorded
/// carriers) extruded over a box on its depth.
#[test]
fn item4_circle() {
    let tol = Tol::witness();
    for halves in [false, true] {
        for (lo, hi) in [(0.75, 1.25), (0.95, 1.05), (0.99, 1.01), (0.9999, 1.0001), (1.0, 1.0)] {
            println!("\n######## circle prism (two half arcs: {halves}), depth [{lo}, {hi}]");
            geom_core::sym::with_session(budget(), || match arc_prism::<SI>(halves, sym_box("circle_depth", lo, hi), tol) {
                Ok(leaf) => {
                    report("Sym<Interval>", &leaf, tol);
                    let rays = topo::test_support::aimed_rays(&*leaf, geom_core::Band::linear(tol).unwrap());
                    println!("  RAYS {:?}", rays.iter().map(|r| r.chars().take(80).collect::<String>()).collect::<Vec<_>>());
                }
                Err(e) => println!("== Sym<Interval>: {}", e.chars().take(400).collect::<String>()),
            });
            for d in [lo, hi] {
                match arc_prism::<f64>(halves, d, tol) {
                    Ok(leaf) => report(&format!("f64 member depth {d}"), &leaf, tol),
                    Err(e) => println!("== f64: {}", e.chars().take(400).collect::<String>()),
                }
            }
        }
    }
}

/// Item 5, the loft: its finish at `Sym<Interval>`, by phase.
#[test]
fn item5_loft_cost() {
    let tol = Tol::witness();
    for (lo, hi) in [(0.999, 1.001), (0.95, 1.05), (0.9, 1.1)] {
        let (_, counts) = geom_core::sym::with_session(budget(), || {
            let t = Instant::now();
            let leaf = match loft2::<SI>(0.0, sym_box("loft_top", lo, hi), tol) {
                Ok(leaf) => leaf,
                Err(e) => return println!("LOFT [{lo}, {hi}] door refused {}", e.chars().take(300).collect::<String>()),
            };
            let door_ms = t.elapsed().as_secs_f64() * 1e3;
            let rays = topo::test_support::aimed_rays(&*leaf, geom_core::Band::linear(tol).unwrap());
            let mid = geom_core::sym::session_counts().unwrap();
            let t = Instant::now();
            let taken = topo::take::<SI>(&Arc::new(leaf), tol).map(|_| ());
            let take_ms = t.elapsed().as_secs_f64() * 1e3;
            let after = geom_core::sym::session_counts().unwrap();
            println!("LOFT top [{lo}, {hi}]: door {door_ms:.1} ms, finish {take_ms:.1} ms symbolic=+{} numeric=+{} frozen={} rays={:?} -> {}", after.symbolic - mid.symbolic, after.numeric - mid.numeric, after.frozen, rays.iter().map(|r| r.chars().take(70).collect::<String>()).collect::<Vec<_>>(), match &taken { Ok(()) => "finished".to_owned(), Err(e) => format!("REFUSED {}", format!("{e:?}").chars().take(500).collect::<String>()) });
        });
        let _ = counts;
    }
}
