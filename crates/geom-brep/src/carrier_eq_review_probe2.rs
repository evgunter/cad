//! Second review probe (PR 7710 at `4baebf756e`): past the first
//! probe's rows. Not permanent rows: each prints what it measured.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr,
    clippy::type_complexity
)]

use super::review_probe::{main_ladder_pub as main_ladder, main_upper_pub as main_upper};
use super::*;
use crate::plane_eq::{PlaneDesc, plane_ladder};
use geom_core::{Band, Decide, Interval, Margin, Point3, Real, Sign, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn sym(&mut self) -> f64 {
        2.0 * self.next() - 1.0
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.sym(), self.sym(), self.sym());
            let n = v.norm();
            if n > 0.1 && n < 1.0 {
                return v / n;
            }
        }
    }
    fn pick<const N: usize>(&mut self, of: [f64; N]) -> f64 {
        of[(self.next() * N as f64) as usize % N]
    }
}

fn perp(n: Vec3<f64>) -> Vec3<f64> {
    let a = if n.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let u = n.cross(a);
    u / u.norm()
}

fn word(r: &Result<CarrierVerdict, CarrierEqError>) -> String {
    match r {
        Ok(CarrierVerdict::One { relation, .. }) => format!("One {relation:?}"),
        Ok(CarrierVerdict::Apart(_)) => "Apart".into(),
        Ok(other) => format!("{other:?}"),
        Err(CarrierEqError::Undecided { .. }) => "Undecided".into(),
        Err(CarrierEqError::Escalated { rung, .. }) => format!("Esc {rung:?}"),
        Err(e) => format!("{e:?}"),
    }
}

fn head_door<T: Decide>(
    a: &PlaneDesc<T>,
    b: &PlaneDesc<T>,
    e: &ConsumedExtent<T>,
    band: Band,
) -> String {
    match pair_door_reading(&CarrierDesc::Plane(*a), &CarrierDesc::Plane(*b), e, band) {
        Ok(PairDoorReading::Verdict(v)) => word(&Ok(v)),
        Ok(other) => format!("{other:?}"),
        Err(e) => word(&Err(e)),
    }
}

fn main_door<T: Decide>(
    a: &PlaneDesc<T>,
    b: &PlaneDesc<T>,
    e: &ConsumedExtent<T>,
    band: Band,
) -> String {
    let l = main_ladder(a, b, e, band);
    match &l {
        Ok(CarrierVerdict::One { relation, .. }) => {
            let sigma = match relation {
                CarrierRelation::SameOpposite => -T::one(),
                _ => T::one(),
            };
            let up = main_upper(a, b, e.reach, sigma);
            match geom_core::k_stats::decide_reported("carrier_plane_reach", Margin::of(up), band) {
                Ok(geom_core::Decided {
                    sign: Sign::Zero, ..
                }) => word(&l),
                _ => "Undecided".into(),
            }
        }
        _ => word(&l),
    }
}

/// The exact largest distance from a point of either plane's disc in
/// the ball to the other plane (unit normals), `None` where neither
/// plane meets the ball.
fn truth(a: &PlaneDesc<f64>, b: &PlaneDesc<f64>, c: Point3<f64>, r: f64) -> Option<f64> {
    let mut best: Option<f64> = None;
    for (p, q) in [(a, b), (b, a)] {
        let d = p.normal.dot(p.origin - c);
        if d.abs() > r {
            continue;
        }
        let foot = c + p.normal * d;
        let rho = (r * r - d * d).sqrt();
        let sin = p.normal.cross(q.normal).norm();
        let off = q.normal.dot(foot - q.origin).abs() + rho * sin;
        best = Some(best.map_or(off, |b: f64| b.max(off)));
    }
    best
}

/// A pose with every knob wider than the first probe's: the centre's
/// distance from the first plane up to `reach_d` radii (past 1: the
/// plane misses the ball), the tilt up to `tilt_k` bands at the arm, the
/// offset up to `off_k` bands, origins up to `1e6` radii along.
fn wide_pose(
    rng: &mut Rng,
    r: f64,
    z: f64,
    reach_d: f64,
    tilt_k: f64,
    off_k: f64,
) -> (PlaneDesc<f64>, PlaneDesc<f64>, Point3<f64>) {
    let cs = rng.pick([0.0, 1.0, 10.0, 1e3]);
    let c = Point3::new(rng.sym() * cs, rng.sym() * cs, rng.sym() * cs);
    let n1 = rng.unit();
    let d1 = rng.sym() * r * reach_d;
    let foot1 = c + n1 * d1;
    let u = perp(n1);
    let w = n1.cross(u);
    let ang = rng.next() * std::f64::consts::TAU;
    let axis = u * ang.cos() + w * ang.sin();
    let tilt = rng.sym() * tilt_k * z / r;
    let side = axis.cross(n1);
    let n2 = n1 * tilt.cos() + side * tilt.sin();
    let n2 = n2 / n2.norm();
    let pivot = foot1 + axis * (rng.sym() * r) + side * (rng.sym() * r);
    let off = rng.sym() * off_k * z;
    let flip = if rng.next() < 0.5 { -1.0 } else { 1.0 };
    let far = rng.pick([0.0, 1.0, 30.0, 1e3, 1e6]) * r;
    (
        PlaneDesc {
            origin: foot1 + perp(n1) * (rng.sym() * far),
            normal: n1,
        },
        PlaneDesc {
            origin: pivot + n2 * off + perp(n2) * far,
            normal: n2 * flip,
        },
        c,
    )
}

/// **Claims 1, 3, 6 at `f64`, wider.** Planes that meet the ball
/// (`reach_d = 1`) and planes that miss it (`reach_d` up to `1e6`);
/// classes of disagreement between the head and main's form, with how
/// far main's own offset margin stood from the band's edges where they
/// disagree, and the exact distance under every head `One`.
#[test]
fn review2_fuzz_wide() {
    let band = band();
    let z = band.zero();
    let esc = band.escalate();
    eprintln!("band zero {z:e} escalate {esc:e}");
    for (label, reach_d, tilt_k, off_k) in [
        ("in the ball", 1.0, 2.0, 2.5),
        ("in the ball, offsets to 30 bands", 1.0, 2.0, 30.0),
        ("centre to 1e2 radii off", 1e2, 2.0, 2.5),
        ("centre to 1e4 radii off", 1e4, 2.0, 2.5),
        ("centre to 1e6 radii off", 1e6, 2.0, 2.5),
    ] {
        let mut rng = Rng(0x1234_5678_9abc_def1);
        let mut counts = std::collections::BTreeMap::<String, (usize, String)>::new();
        let mut worst_door = 0.0_f64;
        let mut worst_ladder = 0.0_f64;
        let mut worst_ladder_main = 0.0_f64;
        let mut flips_away = 0usize;
        for k in 0..300_000 {
            let r = [2e-8, 1e-7, 1e-6, 1e-3, 1.0, 1e3][k % 6];
            let (a, b, c) = wide_pose(&mut rng, r, z, reach_d, tilt_k, off_k);
            let e = ConsumedExtent::new(crate::ExtentBall::unsourced(c, r));
            let hl = word(&plane_ladder(&a, &b, &e, band));
            let hlx = word(&plane_ladder(&b, &a, &e, band));
            let ml = word(&main_ladder(&a, &b, &e, band));
            let hd = head_door(&a, &b, &e, band);
            let hdx = head_door(&b, &a, &e, band);
            let md = main_door(&a, &b, &e, band);
            let t = truth(&a, &b, c, r);
            let scale = [a.origin, b.origin, c]
                .iter()
                .map(|p| p.x.abs().max(p.y.abs()).max(p.z.abs()))
                .fold(0.0, f64::max)
                * 1e-14;
            // Main's own offset, to say how far from an edge a flip is.
            let d1 = a.normal.dot(a.origin - c);
            let d2 = b.normal.dot(b.origin - c);
            let cos = a.normal.dot(b.normal);
            let m_off = (d1 - cos.signum() * d2).abs();
            let h_off = crate::plane_eq::pair_offset(d1, d2, cos);
            let ex = format!(
                "R {r:e} main offset {m_off:e} head offset {h_off:e} d1 {d1:e} d2 {d2:e} \
                 sin {:e} truth {t:?} a {a:?} b {b:?} c {c:?}",
                a.normal.cross(b.normal).norm()
            );
            let mut note = |key: String| {
                let slot = counts.entry(key).or_insert((0, ex.clone()));
                slot.0 += 1;
            };
            if hl != hlx {
                note(format!("ORDER ladder {hl} / {hlx}"));
            }
            if hd != hdx {
                note(format!("ORDER door {hd} / {hdx}"));
            }
            if hl != ml {
                // Away from an edge: main's margin neither within 10% of
                // the zero band's edge nor of the escalate edge, beyond
                // rounding at the data's scale.
                let near = |edge: f64| (m_off - edge).abs() <= 0.1 * edge + scale;
                let away = !near(z) && !near(esc);
                if away {
                    flips_away += 1;
                }
                note(format!(
                    "ladder head {hl} / main {ml}{}",
                    if away { " AWAY FROM AN EDGE" } else { "" }
                ));
            }
            if hd != md {
                note(format!("door head {hd} / main {md}"));
            }
            if let Some(t) = t {
                if hd.starts_with("One") {
                    worst_door = worst_door.max(t / z);
                    if t > z + scale {
                        note("door One past the band beyond rounding".into());
                    }
                }
                if hl.starts_with("One") {
                    worst_ladder = worst_ladder.max(t / z);
                }
                if ml.starts_with("One") {
                    worst_ladder_main = worst_ladder_main.max(t / z);
                }
                let (_, upper) =
                    reading(&CarrierDesc::Plane(a), &CarrierDesc::Plane(b), e.reach).unwrap();
                if t > upper * (1.0 + 1e-12) + scale {
                    note("BOUND VIOLATED".into());
                }
            }
        }
        eprintln!(
            "== {label}: worst exact distance under a head door One {worst_door:.4} z; under \
             a head ladder One {worst_ladder:.4} z; under a main ladder One \
             {worst_ladder_main:.4} z; head/main ladder flips away from an edge {flips_away}"
        );
        for (k, (n, ex)) in &counts {
            eprintln!("{n:>8}  {k}\n          e.g. {ex}");
        }
    }
}

/// **Claim 2 at `Interval`, and how loose.** Random planes at every
/// tilt, each datum widened to a box; the bound's upper end against the
/// exact distance at sampled poses of the box, and at `f64` the excess
/// of the bound over the exact distance less the stated slack
/// (`sin·(R − ρ) + sin²·(|d₁| + |d₂|)`).
#[test]
fn review2_bound_interval_and_tightness() {
    let mut rng = Rng(777);
    let mut worst_ratio = 0.0_f64;
    let mut worst_excess = f64::NEG_INFINITY;
    let mut worst_loose = 0.0_f64;
    let mut violations = 0usize;
    for _ in 0..60_000 {
        let r = 10f64.powf(rng.sym() * 4.0);
        let c = Point3::new(rng.sym() * 100.0, rng.sym() * 100.0, rng.sym() * 100.0);
        let small = rng.next() < 0.5;
        let n1 = rng.unit();
        let n2 = if small {
            let t = 10f64.powf(-12.0 * rng.next());
            let v = n1 + perp(n1) * t;
            v / v.norm() * if rng.next() < 0.5 { -1.0 } else { 1.0 }
        } else {
            rng.unit()
        };
        let mk = |rng: &mut Rng, n: Vec3<f64>| {
            let d = rng.sym() * r;
            let far = 10f64.powf(rng.sym() * 3.0) * r;
            PlaneDesc {
                origin: c + n * d + perp(n) * far,
                normal: n,
            }
        };
        let (a, b) = (mk(&mut rng, n1), mk(&mut rng, n2));
        let Some(t) = truth(&a, &b, c, r) else {
            continue;
        };
        let reach = crate::ExtentBall::unsourced(c, r);
        let (_, upper) = reading(&CarrierDesc::Plane(a), &CarrierDesc::Plane(b), reach).unwrap();
        worst_ratio = worst_ratio.max(t / upper);
        let d1 = a.normal.dot(a.origin - c);
        let d2 = b.normal.dot(b.origin - c);
        let sin = a.normal.cross(b.normal).norm();
        let rho = (r * r - d1.abs().max(d2.abs()).powi(2)).max(0.0).sqrt();
        let slack = sin * (r - rho) + sin * sin * (d1.abs() + d2.abs());
        let scale = (c.x.abs().max(c.y.abs()).max(c.z.abs()) + a.origin.x.abs() + b.origin.x.abs())
            * 1e-13;
        worst_excess = worst_excess.max((upper - t - slack - scale) / r);
        worst_loose = worst_loose.max((upper - t) / upper.max(1e-300));
        // Interval: widen every datum by a relative 1e-9, sample poses.
        let w = 1e-9 * r;
        let iv = |x: f64, w: f64| Interval::from_bounds(x - w, x + w);
        let lift = |p: &PlaneDesc<f64>| PlaneDesc {
            origin: Point3::new(iv(p.origin.x, w), iv(p.origin.y, w), iv(p.origin.z, w)),
            normal: Vec3::new(
                iv(p.normal.x, 1e-12),
                iv(p.normal.y, 1e-12),
                iv(p.normal.z, 1e-12),
            ),
        };
        let reach_i = crate::ExtentBall::unsourced(
            Point3::new(iv(c.x, 0.0), iv(c.y, 0.0), iv(c.z, 0.0)),
            iv(r, 0.0),
        );
        let (_, upper_i) = reading(
            &CarrierDesc::Plane(lift(&a)),
            &CarrierDesc::Plane(lift(&b)),
            reach_i,
        )
        .unwrap();
        let hi = geom_core::Bounds::hi(upper_i);
        for _ in 0..4 {
            let jig = |p: &PlaneDesc<f64>, rng: &mut Rng| PlaneDesc {
                origin: p.origin + Vec3::new(rng.sym() * w, rng.sym() * w, rng.sym() * w),
                normal: p.normal,
            };
            let (aj, bj) = (jig(&a, &mut rng), jig(&b, &mut rng));
            if let Some(tj) = truth(&aj, &bj, c, r) {
                if tj > hi * (1.0 + 1e-9) + scale {
                    violations += 1;
                    eprintln!("INTERVAL BOUND VIOLATED {tj:e} > {hi:e}: {aj:?} {bj:?} {c:?} {r}");
                }
            }
        }
        assert!(
            t <= upper * (1.0 + 1e-9) + scale,
            "{a:?} {b:?} {c:?} {r}: {t} > {upper}"
        );
    }
    eprintln!(
        "worst exact/bound {worst_ratio}; worst (bound − exact − stated slack)/R \
         {worst_excess:e}; worst relative looseness {worst_loose}; Interval violations \
         {violations}"
    );
    assert_eq!(violations, 0);
}

/// **Claim 1 at `Interval`:** a door `One` over a box of poses holds at
/// every sampled pose of the box (exact distance within the band).
#[test]
fn review2_interval_one_holds_over_its_box() {
    let band = band();
    let z = band.zero();
    let mut rng = Rng(4242);
    let (mut ones, mut bad, mut worst) = (0usize, 0usize, 0.0_f64);
    let mut versus = std::collections::BTreeMap::<String, (usize, String)>::new();
    for k in 0..40_000 {
        let r = [1e-6, 1e-3, 1.0, 1e3][k % 4];
        let (a, b, c) = wide_pose(&mut rng, r, z, 1.0, 1.0, 1.0);
        let w = rng.pick([0.0, 1e-3, 1e-2, 1e-1]) * z;
        let wn = w / r;
        let iv = |x: f64, w: f64| Interval::from_bounds(x - w, x + w);
        let lift = |p: &PlaneDesc<f64>| PlaneDesc {
            origin: Point3::new(iv(p.origin.x, w), iv(p.origin.y, w), iv(p.origin.z, w)),
            normal: Vec3::new(iv(p.normal.x, wn), iv(p.normal.y, wn), iv(p.normal.z, wn)),
        };
        let e = ConsumedExtent::new(crate::ExtentBall::unsourced(
            Point3::new(iv(c.x, 0.0), iv(c.y, 0.0), iv(c.z, 0.0)),
            iv(r, 0.0),
        ));
        let (ai, bi) = (lift(&a), lift(&b));
        let hd = head_door(&ai, &bi, &e, band);
        let hdx = head_door(&bi, &ai, &e, band);
        assert_eq!(hd, hdx, "Interval order");
        let md = main_door(&ai, &bi, &e, band);
        if md != hd {
            let slot = versus
                .entry(format!("width {:.0e} z: head {hd} / main {md}", w / z))
                .or_insert((0, format!("R {r:e} a {a:?} b {b:?} c {c:?}")));
            slot.0 += 1;
        }
        if !hd.starts_with("One") {
            continue;
        }
        ones += 1;
        for _ in 0..6 {
            let jig = |p: &PlaneDesc<f64>, rng: &mut Rng| {
                let n = p.normal + Vec3::new(rng.sym() * wn, rng.sym() * wn, rng.sym() * wn);
                PlaneDesc {
                    origin: p.origin + Vec3::new(rng.sym() * w, rng.sym() * w, rng.sym() * w),
                    normal: n / n.norm(),
                }
            };
            let (aj, bj) = (jig(&a, &mut rng), jig(&b, &mut rng));
            if let Some(t) = truth(&aj, &bj, c, r) {
                worst = worst.max(t / z);
                let scale = (c.x.abs() + aj.origin.x.abs() + bj.origin.x.abs() + r) * 1e-13;
                if t > z + scale {
                    bad += 1;
                }
            }
        }
    }
    for (k, (n, ex)) in &versus {
        eprintln!("{n:>8}  {k}\n          e.g. {ex}");
    }
    eprintln!("Interval door Ones {ones}; sampled poses past the band {bad}; worst {worst} z");
    assert_eq!(bad, 0);
}

/// The head's ladder, the head's door and main's ladder on a pair, each
/// `Some(discharged)` where it read one plane.
fn sym_row(
    max_terms: usize,
    build: &dyn Fn(&[geom_core::Sym<f64>]) -> [PlaneDesc<geom_core::Sym<f64>>; 2],
    exchanged: bool,
) -> [Option<bool>; 3] {
    use geom_core::sym::deferred::{discharge, with_deferred_session};
    use geom_core::{ParamSymbol, Sym, SymBudget, SymRetry, SymRules};
    type S = Sym<f64>;
    let budget = SymBudget {
        max_terms,
        max_degree: 64,
    };
    with_deferred_session(budget, SymRules::shipped(), SymRetry::none(), || {
        let vars: Vec<S> = (0..24)
            .map(|i| {
                S::param(
                    ParamSymbol::new(test_utils::symbol_id(&format!("x{i}"))),
                    1.0 + 0.1 * f64::from(i),
                )
            })
            .collect();
        let [p1, p2] = build(&vars);
        let (a, b) = if exchanged { (p2, p1) } else { (p1, p2) };
        let extent = ConsumedExtent::arm(S::from_f64(1.0));
        let ladder = plane_ladder(&a, &b, &extent, band())
            .ok()
            .and_then(CarrierVerdict::one);
        let door = pair_door_reading(
            &CarrierDesc::Plane(a),
            &CarrierDesc::Plane(b),
            &extent,
            band(),
        )
        .ok()
        .and_then(|r| r.verdict().one());
        let mainl = main_ladder(&a, &b, &extent, band())
            .ok()
            .and_then(CarrierVerdict::one);
        [ladder, door, mainl].map(|o| o.map(|(_, m)| discharge(m)))
    })
    .0
}

/// **Claims 4 and 6 at `Sym<f64>`, past the first probe:** sign factors
/// in BOTH normals. (a) both walls carry the SAME factor (two walls
/// minted from one chord: main's offset is linear in it and cancels);
/// (b) the walls carry two different factors; (c) both normals are one
/// chord's quotient normal in the plane (`(−dy, dx)/‖·‖`), the origins a
/// step along the wall apart; (d) the same with the second wall's normal
/// negated. Each at growing sizes of the factor, at 256 and 4,096 terms,
/// both orders: prints head ladder, head door, main ladder.
#[test]
fn review2_symbolic_factors_in_both_normals() {
    use geom_core::Sym;
    type S = Sym<f64>;
    let k = S::from_f64;
    fn sum(x: &[S], r: std::ops::Range<usize>) -> S {
        x[r].iter().copied().fold(S::from_f64(0.0), |a, b| a + b)
    }
    let unitise = |v: S| v / (v * v).sqrt();
    // Sides of growing size, each positive at its value.
    let side = move |size: usize, x: &[S], shift: usize| -> S {
        let s = shift;
        match size {
            0 => x[s],
            1 => x[s] * x[s + 1] - x[s + 2] * x[s + 3] + k(5.0),
            2 => sum(x, s..s + 4) * sum(x, s + 4..s + 8) - sum(x, s..s + 3) * sum(x, s + 3..s + 6),
            3 => sum(x, s..s + 8) * sum(x, s..s + 8) * sum(x, s..s + 4) - sum(x, s..s + 5),
            4 => sum(x, 0..24) - k(20.0),
            _ => sum(x, s..s + 4) * sum(x, s + 4..s + 8) * sum(x, s..s + 2) - sum(x, s..s + 5),
        }
    };
    let mut lost = Vec::new();
    let mut order = Vec::new();
    for size in 0..6usize {
        let shapes: Vec<(&str, Box<dyn Fn(&[S]) -> [PlaneDesc<S>; 2]>)> = vec![
            (
                "same factor in both normals",
                Box::new(move |x: &[S]| {
                    let lam = unitise(side(size, x, 0));
                    let y = -(x[20] / k(2.0));
                    [
                        PlaneDesc {
                            origin: Point3::new(k(0.0), k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), lam),
                        },
                        PlaneDesc {
                            origin: Point3::new(x[7], k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), lam),
                        },
                    ]
                }),
            ),
            (
                "same factor, second negated",
                Box::new(move |x: &[S]| {
                    let lam = unitise(side(size, x, 0));
                    let y = -(x[20] / k(2.0));
                    [
                        PlaneDesc {
                            origin: Point3::new(k(0.0), k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), lam),
                        },
                        PlaneDesc {
                            origin: Point3::new(x[7], k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), -lam),
                        },
                    ]
                }),
            ),
            (
                "two different factors",
                Box::new(move |x: &[S]| {
                    let la = unitise(side(size, x, 0));
                    let lb = unitise(side(size, x, 8));
                    let y = -(x[20] / k(2.0));
                    [
                        PlaneDesc {
                            origin: Point3::new(k(0.0), k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), la),
                        },
                        PlaneDesc {
                            origin: Point3::new(x[7], k(0.0), y),
                            normal: Vec3::new(k(0.0), k(0.0), lb),
                        },
                    ]
                }),
            ),
            (
                "one chord's quotient normal in both walls",
                Box::new(move |x: &[S]| {
                    let (dx, dy) = (side(size, x, 0), side(size.min(2), x, 8));
                    let len = (dx * dx + dy * dy).sqrt();
                    let n = Vec3::new(-(dy / len), dx / len, k(0.0));
                    let p = Point3::new(x[20], x[21], k(0.0));
                    let t = x[22];
                    [
                        PlaneDesc {
                            origin: p,
                            normal: n,
                        },
                        PlaneDesc {
                            origin: Point3::new(p.x + t * dx, p.y + t * dy, x[23]),
                            normal: n,
                        },
                    ]
                }),
            ),
            (
                "one chord's quotient normal, second wall opposed",
                Box::new(move |x: &[S]| {
                    let (dx, dy) = (side(size, x, 0), side(size.min(2), x, 8));
                    let len = (dx * dx + dy * dy).sqrt();
                    let n = Vec3::new(-(dy / len), dx / len, k(0.0));
                    let p = Point3::new(x[20], x[21], k(0.0));
                    let t = x[22];
                    [
                        PlaneDesc {
                            origin: p,
                            normal: n,
                        },
                        PlaneDesc {
                            origin: Point3::new(p.x + t * dx, p.y + t * dy, x[23]),
                            normal: -n,
                        },
                    ]
                }),
            ),
            (
                "a unit normal spun from one chord, as a cross product",
                Box::new(move |x: &[S]| {
                    let u = Vec3::new(side(size, x, 0), x[9], k(0.0));
                    let w = Vec3::new(k(0.0), k(0.0), k(1.0));
                    let c = u.cross(w);
                    let n = c / c.norm();
                    let p = Point3::new(x[20], x[21], k(0.0));
                    [
                        PlaneDesc {
                            origin: p,
                            normal: n,
                        },
                        PlaneDesc {
                            origin: Point3::new(p.x + x[22] * u.x, p.y + x[22] * u.y, x[23]),
                            normal: n,
                        },
                    ]
                }),
            ),
        ];
        for (name, build) in &shapes {
            for max_terms in [256usize, 4096] {
                if max_terms == 4096 && size < 2 {
                    continue;
                }
                let stated = sym_row(max_terms, build.as_ref(), false);
                let exchanged = sym_row(max_terms, build.as_ref(), true);
                eprintln!(
                    "size {size} {name}, {max_terms} terms: head ladder/door/main ladder \
                     {stated:?} exchanged {exchanged:?}"
                );
                if stated[..2] != exchanged[..2] {
                    order.push(format!("size {size} {name} {max_terms}"));
                }
                for (which, row) in [("stated", stated), ("exchanged", exchanged)] {
                    if row[2] == Some(true) && row[0] != Some(true) {
                        lost.push(format!("size {size} {name} {max_terms} {which}: {row:?}"));
                    }
                }
            }
        }
    }
    eprintln!("ORDER-DEPENDENT: {order:#?}");
    eprintln!("MAIN PROVES, HEAD DOES NOT: {lost:#?}");
}
