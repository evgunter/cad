//! Review probe (PR 7710): the one-plane reading's claims, run against
//! the tree. Not a permanent row: each test prints what it measured and
//! asserts only where the claim is stated as a bound.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stderr
)]

use super::*;
use crate::plane_eq::{PlaneDesc, plane_ladder};
use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A small deterministic generator.
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

/// Main's ladder (merge base), reproduced: `d₁ − σ·d₂`.
fn main_ladder<T: Decide>(
    p1: &PlaneDesc<T>,
    p2: &PlaneDesc<T>,
    extent: &ConsumedExtent<T>,
    band: Band,
) -> Result<CarrierVerdict, CarrierEqError> {
    use geom_core::k_stats::{decide_nonzero, decide_reported};
    use geom_core::{Decided, NonzeroSign};
    let centre = extent.reach.center();
    let arm = extent.reach.lever();
    let d1 = p1.normal.dot(p1.origin - centre);
    let d2 = p2.normal.dot(p2.origin - centre);
    let parallel_margin = Margin::levered(p1.normal.cross(p2.normal).norm(), arm);
    let parallel = match decide_reported("plane_eq_parallel", parallel_margin, band) {
        Ok(
            t @ Decided {
                sign: Sign::Positive,
                ..
            },
        ) => return Ok(CarrierVerdict::Apart(t)),
        Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) => margin,
        Ok(n) => return Err(crate::plane_eq::unreadable_norm(n, band)),
        Err(diag) => {
            return Err(CarrierEqError::Escalated {
                rung: crate::plane_eq::PlaneRung::Parallel,
                diag,
            });
        }
    };
    let sign_margin = Margin::levered(p1.normal.dot(p2.normal), arm);
    let side = decide_nonzero("plane_eq_orient", sign_margin, band).map_err(|diag| {
        CarrierEqError::Escalated {
            rung: crate::plane_eq::PlaneRung::Orientation,
            diag,
        }
    })?;
    let sigma = side.unit::<T>();
    let relation = match side {
        NonzeroSign::Positive => CarrierRelation::SameOriented,
        NonzeroSign::Negative => CarrierRelation::SameOpposite,
    };
    match CoincidenceMeasure::decide("plane_eq_offset", Margin::of(d1 - sigma * d2), band) {
        Ok(
            o @ Decided {
                sign: Sign::Positive | Sign::Negative,
                ..
            },
        ) => Ok(CarrierVerdict::Apart(o)),
        Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) => Ok(CarrierVerdict::One {
            relation,
            margin: margin.jointly(parallel),
        }),
        Err(coincidence) => Err(CarrierEqError::Undecided {
            coincidence,
            relation,
        }),
    }
}

/// Main's pair-door bound: `|d₁ − σ·d₂| + |n₁ − σ·n₂|·R`.
fn main_upper<T: Decide>(
    p1: &PlaneDesc<T>,
    p2: &PlaneDesc<T>,
    reach: crate::ExtentBall<T>,
    sigma: T,
) -> T {
    let c = reach.center();
    let offset = p1.normal.dot(p1.origin - c) - sigma * p2.normal.dot(p2.origin - c);
    offset.abs() + Margin::levered((p1.normal - p2.normal * sigma).norm(), reach.lever()).value()
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

/// The true largest distance from a point of either plane in the ball to
/// the other plane (exact for unit normals): the disc of plane `i` in the
/// ball is centred at the centre's foot with radius `√(R² − dᵢ²)`.
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

/// A pair near the band: plane 1 through a point of the ball, plane 2
/// tilted by a multiple of `z/R` about a random in-plane axis through a
/// random point, offset by a multiple of `z`, its origin stated `far`
/// arms along itself, and its normal flipped half the time.
fn pose(rng: &mut Rng, r: f64, z: f64) -> (PlaneDesc<f64>, PlaneDesc<f64>, Point3<f64>) {
    let c = Point3::new(rng.sym() * 10.0, rng.sym() * 10.0, rng.sym() * 10.0);
    let n1 = rng.unit();
    let d1 = rng.sym() * r;
    let foot1 = c + n1 * d1;
    let u = perp(n1);
    let w = n1.cross(u);
    let ang = rng.next() * std::f64::consts::TAU;
    let axis = u * ang.cos() + w * ang.sin();
    let tilt = rng.sym() * 2.0 * z / r;
    let side = axis.cross(n1);
    let n2 = n1 * tilt.cos() + side * tilt.sin();
    let pivot = foot1 + axis * (rng.sym() * r) + side * (rng.sym() * r);
    let off = rng.sym() * 2.5 * z;
    let flip = if rng.next() < 0.5 { -1.0 } else { 1.0 };
    let n2 = n2 / n2.norm();
    let far = [0.0, 1.0, 30.0, 1e3][(rng.next() * 4.0) as usize % 4] * r;
    let along = perp(n2);
    let o2 = pivot + n2 * off + along * far;
    let o1 = foot1 + perp(n1) * (rng.sym() * far);
    (
        PlaneDesc {
            origin: o1,
            normal: n1,
        },
        PlaneDesc {
            origin: o2,
            normal: n2 * flip,
        },
        c,
    )
}

/// **Claims 1, 2, 3, 6 at `f64` and `Interval`:** over random poses near
/// the band, (a) a door `One` is within the band over the ball (exact
/// distance), (b) the head's bound bounds the exact distance (any tilt),
/// (c) where head and main disagree, (d) where the two operand orders
/// disagree.
#[test]
fn review_fuzz_head_against_main_and_order() {
    let band = band();
    let z = band.zero();
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    let mut examples = std::collections::BTreeMap::<String, String>::new();
    let mut worst_one = 0.0_f64;
    let mut worst_ladder_one = 0.0_f64;
    for k in 0..400_000 {
        let r = [2e-8, 1e-6, 1e-3, 1.0, 1e3][k % 5];
        let (a, b, c) = pose(&mut rng, r, z);
        let e = ConsumedExtent::new(crate::ExtentBall::unsourced(c, r));
        let hl = word(&plane_ladder(&a, &b, &e, band));
        let ml = word(&main_ladder(&a, &b, &e, band));
        let hlx = word(&plane_ladder(&b, &a, &e, band));
        let hd = head_door(&a, &b, &e, band);
        let md = main_door(&a, &b, &e, band);
        let hdx = head_door(&b, &a, &e, band);
        let mdx = main_door(&b, &a, &e, band);
        let t = truth(&a, &b, c, r);
        let mut note = |key: String, ex: String| {
            *counts.entry(key.clone()).or_default() += 1;
            examples.entry(key).or_insert(ex);
        };
        let ex = format!("R {r:e} a {a:?} b {b:?} c {c:?} truth {t:?}");
        if hl != ml {
            note(format!("f64 ladder head {hl} / main {ml}"), ex.clone());
        }
        if hd != md {
            note(format!("f64 door head {hd} / main {md}"), ex.clone());
        }
        if hl.split(' ').next() != hlx.split(' ').next() {
            note(format!("f64 ladder order {hl} / {hlx}"), ex.clone());
        }
        if hd.split(' ').next() != hdx.split(' ').next() {
            note(format!("f64 door order {hd} / {hdx}"), ex.clone());
        }
        if md.split(' ').next() != mdx.split(' ').next() {
            note(format!("f64 MAIN door order {md} / {mdx}"), ex.clone());
        }
        // The rounding scale of the data: the largest coordinate times a few ulps.
        let scale = [a.origin, b.origin, c]
            .iter()
            .map(|p| p.x.abs().max(p.y.abs()).max(p.z.abs()))
            .fold(0.0, f64::max)
            * 1e-14;
        if let Some(t) = t {
            if md.starts_with("One") && t > z + scale {
                note(
                    "f64 MAIN door One past the band beyond rounding".into(),
                    ex.clone(),
                );
            }
            if hd.starts_with("One") && t > z + scale {
                note(
                    "f64 door One past the band beyond rounding".into(),
                    ex.clone(),
                );
            }
            if hd.starts_with("One") {
                worst_one = worst_one.max(t / z);
            }
            if hl.starts_with("One") {
                worst_ladder_one = worst_ladder_one.max(t / z);
            }
        }
        // (b): the head's bound over any tilt.
        let (_, upper) = reading(&CarrierDesc::Plane(a), &CarrierDesc::Plane(b), e.reach).unwrap();
        if let Some(t) = t {
            if t > upper * (1.0 + 1e-12) + scale {
                note("f64 BOUND VIOLATED".into(), format!("{ex} upper {upper:e}"));
            }
        }
        if k % 20 == 0 {
            use geom_core::Interval;
            let iv = |p: &PlaneDesc<f64>| PlaneDesc {
                origin: Point3::new(
                    Interval::from_f64(p.origin.x),
                    Interval::from_f64(p.origin.y),
                    Interval::from_f64(p.origin.z),
                ),
                normal: Vec3::new(
                    Interval::from_f64(p.normal.x),
                    Interval::from_f64(p.normal.y),
                    Interval::from_f64(p.normal.z),
                ),
            };
            let (ai, bi) = (iv(&a), iv(&b));
            let ei = ConsumedExtent::new(crate::ExtentBall::unsourced(
                Point3::new(
                    Interval::from_f64(c.x),
                    Interval::from_f64(c.y),
                    Interval::from_f64(c.z),
                ),
                Interval::from_f64(r),
            ));
            let hdi = head_door(&ai, &bi, &ei, band);
            let mdi = main_door(&ai, &bi, &ei, band);
            let hdix = head_door(&bi, &ai, &ei, band);
            if hdi != mdi {
                note(format!("Interval door head {hdi} / main {mdi}"), ex.clone());
            }
            if hdi.split(' ').next() != hdix.split(' ').next() {
                note(format!("Interval door order {hdi} / {hdix}"), ex.clone());
            }
            if hdi.starts_with("One") && !hd.starts_with("One") {
                note(format!("Interval One where f64 {hd}"), ex.clone());
            }
        }
    }
    eprintln!(
        "worst exact distance under a door One: {worst_one} z; under a ladder One: {worst_ladder_one} z"
    );
    for (k, n) in &counts {
        eprintln!("{n:>8}  {k}\n          e.g. {}", examples[k]);
    }
    assert!(!counts.contains_key("f64 BOUND VIOLATED"));
    assert!(!counts.contains_key("f64 door One past the band beyond rounding"));
}

/// **Claim 2 over every tilt:** random planes, random balls, origins far
/// along each plane; the head's bound at `f64` holds against the exact
/// distance, and its `Interval` upper end against the same.
#[test]
fn review_bound_holds_at_every_tilt() {
    let mut rng = Rng(12345);
    let mut worst = 0.0_f64;
    for _ in 0..200_000 {
        let r = 10f64.powf(rng.sym() * 4.0);
        let c = Point3::new(rng.sym() * 100.0, rng.sym() * 100.0, rng.sym() * 100.0);
        let mk = |rng: &mut Rng| {
            let n = rng.unit();
            let d = rng.sym() * r;
            let far = 10f64.powf(rng.sym() * 3.0) * r;
            PlaneDesc {
                origin: c + n * d + perp(n) * far,
                normal: n,
            }
        };
        let (a, b) = (mk(&mut rng), mk(&mut rng));
        let Some(t) = truth(&a, &b, c, r) else {
            continue;
        };
        let reach = crate::ExtentBall::unsourced(c, r);
        let (_, upper) = reading(&CarrierDesc::Plane(a), &CarrierDesc::Plane(b), reach).unwrap();
        worst = worst.max(t / upper);
        assert!(
            t <= upper * (1.0 + 1e-9),
            "{a:?} {b:?} {c:?} {r}: {t} > {upper}"
        );
    }
    eprintln!("worst exact/bound ratio {worst}");
}

/// **Claim 6 at a symbolic scalar:** two walls on one plane, the first's
/// normal a literal axis, the second's the literal axis times a sign
/// factor `λ` spelled several ways. The offset `d₁ − (n₁·n₂)·d₂` needs
/// `λ² ≡ 1`; exchanged, `d₂ − (n₁·n₂)·d₁` is linear in `λ`. Prints which
/// orders discharge at the ladder and the pair door.
#[test]
fn review_symbolic_order() {
    use geom_core::sym::deferred::{discharge, with_deferred_session};
    use geom_core::{ParamSymbol, Sym, SymBudget, SymRetry, SymRules};
    type S = Sym<f64>;
    let symbol = |name: &str| ParamSymbol::new(test_utils::symbol_id(name));
    let mut rows = Vec::new();
    for (max_terms, spelling) in
        (0..10)
            .map(|s| (256, s))
            .chain([(4096, 4), (4096, 7), (4096, 8), (4096, 9)])
    {
        let budget = SymBudget {
            max_terms,
            max_degree: 64,
        };
        for exchanged in [false, true] {
            let (got, _) =
                with_deferred_session(budget, SymRules::shipped(), SymRetry::none(), || {
                    let at = |name: &str, value| S::param(symbol(name), value);
                    let k = S::from_f64;
                    let vars: Vec<S> = (0..24)
                        .map(|i| at(&format!("x{i}"), 1.0 + 0.1 * f64::from(i)))
                        .collect();
                    let sum = |range: std::ops::Range<usize>| {
                        vars[range].iter().copied().fold(k(0.0), |a, b| a + b)
                    };
                    let unitise = |x: S| x / (x * x).sqrt();
                    // λ, the second normal's sign factor, each spelling positive at its value.
                    let lambda = match spelling {
                        0 => unitise(vars[0]),
                        1 => unitise(vars[0] - vars[1] + k(1.0)),
                        2 => unitise(vars[0] * vars[1] - vars[2] * vars[3] + k(5.0)),
                        3 => unitise(sum(0..4) * sum(4..8) - sum(0..3) * sum(3..6)),
                        4 => unitise(sum(0..8) * sum(0..8) * sum(0..4) - sum(0..5)),
                        7 => unitise(sum(0..24) - k(20.0)),
                        8 => unitise(sum(0..4) * sum(0..4) - sum(0..3)),
                        9 => unitise(sum(0..4) * sum(4..8) * sum(0..2) - sum(0..5)),
                        5 => {
                            // A chord's unit in 3-space whose other components are zero as formulas.
                            let dz = sum(0..4) - sum(4..6);
                            let v = Vec3::new(vars[0] - vars[0], k(0.0), dz);
                            v.z / v.norm()
                        }
                        _ => {
                            // The cross product of two chords in the plane z = const.
                            let u = Vec3::new(sum(0..3), vars[3], k(0.0));
                            let w = Vec3::new(vars[4], sum(5..8), k(0.0));
                            let n = u.cross(w);
                            n.z / n.norm()
                        }
                    };
                    let h = at("h", 1.0);
                    let y = -(h / k(2.0));
                    let p1 = PlaneDesc {
                        origin: Point3::new(k(0.0), k(0.0), y),
                        normal: Vec3::new(k(0.0), k(0.0), k(-1.0)),
                    };
                    let p2 = PlaneDesc {
                        origin: Point3::new(vars[7], k(0.0), y),
                        normal: Vec3::new(k(0.0), k(0.0), lambda),
                    };
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
                    [ladder, door, mainl].map(|o| o.map(|(rel, m)| (rel, discharge(m))))
                });
            rows.push(format!("terms {max_terms} spelling {spelling} exchanged {exchanged}: ladder, door, main ladder {got:?}"));
        }
    }
    for r in &rows {
        eprintln!("{r}");
    }
}

/// **Claim 5 at `Sym<Interval>` over a box:** the author's two walls (a
/// literal axis beside `v/√v²`, and two quotients), main's ladder against
/// the head's, both operand orders.
#[test]
fn review_sym_interval_box_main_against_head() {
    use geom_core::{Interval, ParamSymbol, Sym, SymBudget};
    type S = Sym<Interval>;
    let budget = SymBudget {
        max_terms: 256,
        max_degree: 64,
    };
    let symbol = |name: &str| ParamSymbol::new(test_utils::symbol_id(name));
    for literal in [true, false] {
        for exchanged in [false, true] {
            let (got, _) = geom_core::sym::with_session(budget, || {
                let at = |name, lo, hi| S::param(symbol(name), Interval::from_bounds(lo, hi));
                let k = S::from_f64;
                let (w, v, h) = (at("w", 1.9, 2.1), at("v", 1.9, 2.1), at("h", 0.9, 1.1));
                let y = -(h / k(2.0));
                let down = if literal {
                    k(-1.0)
                } else {
                    -(w / (w * w).sqrt())
                };
                let p1 = PlaneDesc {
                    origin: Point3::new(k(0.0), y, k(0.0)),
                    normal: Vec3::new(k(0.0), down, k(0.0)),
                };
                let p2 = PlaneDesc {
                    origin: Point3::new(v, y, k(0.0)),
                    normal: Vec3::new(k(0.0), v / (v * v).sqrt(), k(0.0)),
                };
                let (a, b) = if exchanged { (p2, p1) } else { (p1, p2) };
                let e = ConsumedExtent::arm(S::from_f64(1.0));
                let head = plane_ladder(&a, &b, &e, band());
                let main = main_ladder(&a, &b, &e, band());
                (
                    format!("{head:?}").chars().take(300).collect::<String>(),
                    format!("{main:?}").chars().take(400).collect::<String>(),
                )
            });
            eprintln!(
                "literal {literal} exchanged {exchanged}\n  head {}\n  main {}",
                got.0, got.1
            );
        }
    }
}
