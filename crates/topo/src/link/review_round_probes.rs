//! Review probes (PR #5827): `Round::offset`, `Round::lowest_on`, the
//! Interval and Dual lanes, and `rest::by_round`. Each probe prints its
//! measurement; the asserting ones are named `probe_red_*`.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

test_utils::gated_to![
    "crates/topo/src/link.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
];

use geom_core::{Bounds, Dual, Interval};

use super::*;

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

fn band() -> Band {
    Band::linear(geom_core::Tol::witness()).expect("band")
}

/// An independent offset: |rel x a| for a cylinder, |rel| for a sphere.
fn oracle_offset(
    cyl: bool,
    convex: bool,
    c: Point3<f64>,
    a: Vec3<f64>,
    r: f64,
    q: Point3<f64>,
) -> f64 {
    let rel = q - c;
    let d = if cyl { rel.cross(a).norm() } else { rel.norm() };
    if convex { d - r } else { r - d }
}

/// Dense walk then golden-section refinement about the best sample.
fn oracle_lowest(
    cyl: bool,
    convex: bool,
    c: Point3<f64>,
    a: Vec3<f64>,
    r: f64,
    x0: Point3<f64>,
    x1: Point3<f64>,
) -> f64 {
    let f = |t: f64| oracle_offset(cyl, convex, c, a, r, x0 + (x1 - x0) * t);
    let n = 20000;
    let (mut best, mut bt) = (f64::INFINITY, 0.0);
    for i in 0..=n {
        let t = f64::from(i) / f64::from(n);
        let y = f(t);
        if y < best {
            best = y;
            bt = t;
        }
    }
    let (mut lo, mut hi) = (
        (bt - 1.0 / f64::from(n)).max(0.0),
        (bt + 1.0 / f64::from(n)).min(1.0),
    );
    for _ in 0..200 {
        let m1 = lo + (hi - lo) / 3.0;
        let m2 = hi - (hi - lo) / 3.0;
        if f(m1) < f(m2) { hi = m2 } else { lo = m1 }
    }
    best.min(f(0.5 * (lo + hi))).min(f(0.0)).min(f(1.0))
}

fn tilted() -> Vec3<f64> {
    v(0.31, -0.57, 0.76).normalize()
}

fn perp(a: Vec3<f64>) -> Vec3<f64> {
    a.cross(v(0.9, 0.2, -0.1)).normalize()
}

/// Claim 2: worst excess of `lowest_on` over the oracle (positive: the
/// closed form missed a lower value) and worst deficit.
#[test]
fn probe_lowest_on_against_oracle() {
    let a = tilted();
    let w = perp(a);
    let w2 = a.cross(w);
    let mut worst_excess: (f64, String) = (0.0, String::new());
    let mut worst_deficit: (f64, String) = (0.0, String::new());
    for &far in &[0.0, 1.0e3] {
        let c = Point3::new(0.37, -1.21, 0.83) + a * far;
        for &r in &[1.0, 1e-3, 37.0] {
            for cyl in [true, false] {
                for convex in [true, false] {
                    let round = Round::<f64>::of(
                        &geom::Surface::Cylinder {
                            origin: c,
                            axis: a,
                            radius: r,
                            u_ref: w,
                        },
                        convex,
                    )
                    .unwrap();
                    let round = if cyl {
                        round
                    } else {
                        Round::sphere(c, r, convex)
                    };
                    let mut cases: Vec<(String, Point3<f64>, Point3<f64>)> = Vec::new();
                    let base = c + w * (0.3 * r);
                    cases.push((
                        "crossing axis".into(),
                        c - w * (2.0 * r) + a * 0.1,
                        c + w * (2.0 * r) + a * 0.3,
                    ));
                    cases.push((
                        "through centre".into(),
                        c - w * (2.0 * r),
                        c + w * (2.0 * r),
                    ));
                    cases.push((
                        "tangent".into(),
                        c + w * r - w2 * (3.0 * r),
                        c + w * r + w2 * (3.0 * r),
                    ));
                    cases.push(("parallel".into(), base, base + a * (5.0 * r)));
                    for &eps in &[1e-6, 1e-9, 1e-12, 1e-15] {
                        for &len in &[1.0, 1e3] {
                            let d = (a + w2 * eps).normalize();
                            cases.push((
                                format!("nearly parallel {eps:e} len {len}"),
                                base - d * (0.5 * len * r),
                                base + d * (0.5 * len * r),
                            ));
                            let d = (a + w * eps).normalize();
                            cases.push((
                                format!("nearly parallel radial {eps:e} len {len}"),
                                base - d * (0.5 * len * r),
                                base + d * (0.5 * len * r),
                            ));
                        }
                    }
                    cases.push(("short".into(), base, base + w2 * 1e-12));
                    cases.push(("point".into(), base, base));
                    for (name, x0, x1) in cases {
                        let got = round.lowest_on((x0, x1));
                        let want = oracle_lowest(cyl, convex, c, a, r, x0, x1);
                        let tag = format!(
                            "{} {} far {far} r {r} {name}: got {got:e} oracle {want:e}",
                            if cyl { "cyl" } else { "sph" },
                            if convex { "convex" } else { "concave" }
                        );
                        if !got.is_finite() {
                            println!("NONFINITE {tag}");
                        }
                        let rel = r.max(1.0);
                        if (got - want) / rel > worst_excess.0 {
                            worst_excess = ((got - want) / rel, tag.clone());
                        }
                        if (want - got) / rel > worst_deficit.0 {
                            worst_deficit = ((want - got) / rel, tag);
                        }
                    }
                }
            }
        }
    }
    println!(
        "WORST EXCESS (rel) {:e}: {}",
        worst_excess.0, worst_excess.1
    );
    println!(
        "WORST DEFICIT (rel) {:e}: {}",
        worst_deficit.0, worst_deficit.1
    );
}

/// Claim 3: the Interval lane encloses the f64 value over wide boxes
/// straddling the axis, the centre and the surface.
#[test]
fn probe_interval_encloses_over_wide_boxes() {
    let a = tilted();
    let w = perp(a);
    let c = Point3::new(0.37, -1.21, 0.83);
    let r = 0.7;
    let mut misses = Vec::new();
    let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut rnd = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    let centres = [
        ("axis/centre", c),
        ("surface", c + w * r),
        ("off", c + w * 2.0 * r),
    ];
    for cyl in [true, false] {
        for convex in [true, false] {
            let f = if cyl {
                Round::<f64>::cylinder(c, a, r, convex)
            } else {
                Round::sphere(c, r, convex)
            };
            let iv = |x: f64| Interval::from_bounds(x, x);
            let ic = Point3::new(iv(c.x), iv(c.y), iv(c.z));
            let ia = Vec3::new(iv(a.x), iv(a.y), iv(a.z));
            let ir = if cyl {
                Round::<Interval>::cylinder(ic, ia, iv(r), convex)
            } else {
                Round::sphere(ic, iv(r), convex)
            };
            for (name, m) in centres {
                for &h in &[0.05, 0.5] {
                    let bx = |q: Point3<f64>| {
                        Point3::new(
                            Interval::from_bounds(q.x - h, q.x + h),
                            Interval::from_bounds(q.y - h, q.y + h),
                            Interval::from_bounds(q.z - h, q.z + h),
                        )
                    };
                    let off = ir.offset(bx(m));
                    let m1 = m + perp(w) * 0.9;
                    let low = ir.lowest_on((bx(m), bx(m1)));
                    for _ in 0..400 {
                        let j = |q: Point3<f64>, rnd: &mut dyn FnMut() -> f64| {
                            q + v(rnd() * 2.0 - 1.0, rnd() * 2.0 - 1.0, rnd() * 2.0 - 1.0) * h
                        };
                        let q = j(m, &mut rnd);
                        let y = f.offset(q);
                        if !(off.lo() <= y && y <= off.hi()) {
                            misses.push(format!(
                                "offset {cyl} {convex} {name} h {h}: [{}, {}] misses {y}",
                                off.lo(),
                                off.hi()
                            ));
                        }
                        let q1 = j(m1, &mut rnd);
                        let y = f.lowest_on((q, q1));
                        if !(low.lo() <= y && y <= low.hi()) {
                            misses.push(format!(
                                "lowest {cyl} {convex} {name} h {h}: [{}, {}] misses {y}",
                                low.lo(),
                                low.hi()
                            ));
                        }
                    }
                }
            }
        }
    }
    println!("interval misses: {}", misses.len());
    for m in misses.iter().take(20) {
        println!("{m}");
    }
    assert!(misses.is_empty());
}

/// Claim 3: the Dual derivative at the axis or the centre.
#[test]
fn probe_dual_at_axis_and_centre() {
    let d = |x: f64| Dual::constant(x);
    let dp = |q: Point3<f64>| Point3::new(d(q.x), d(q.y), d(q.z));
    // Radius the variable, as Gap's toleranced-radius pass seeds it.
    let pin = Round::<Dual<f64>>::cylinder(
        dp(Point3::new(0.0, 0.0, 0.0)),
        Vec3::new(d(0.0), d(0.0), d(1.0)),
        Dual::variable(1.0),
        true,
    );
    let ball =
        Round::<Dual<f64>>::sphere(dp(Point3::new(0.0, 0.0, 0.0)), Dual::variable(1.0), true);
    let on_axis = pin.offset(dp(Point3::new(0.0, 0.0, 0.3)));
    let at_centre = ball.offset(dp(Point3::new(0.0, 0.0, 0.0)));
    let across = pin.lowest_on((
        dp(Point3::new(-2.0, 0.0, 0.3)),
        dp(Point3::new(2.0, 0.0, 0.3)),
    ));
    let through = ball.lowest_on((
        dp(Point3::new(-2.0, 0.0, 0.0)),
        dp(Point3::new(2.0, 0.0, 0.0)),
    ));
    // A point seeded in x, on the axis.
    let px = Point3::new(Dual::variable(0.0), d(0.0), d(0.3));
    let pin_x = Round::<Dual<f64>>::cylinder(
        dp(Point3::new(0.0, 0.0, 0.0)),
        Vec3::new(d(0.0), d(0.0), d(1.0)),
        d(1.0),
        true,
    );
    let seeded = pin_x.offset(px);
    println!(
        "vertex on axis: {on_axis:?}\nvertex at centre: {at_centre:?}\nedge across axis: {across:?}\nedge through centre: {through:?}\nx-seeded on axis: {seeded:?}"
    );
}

/// Claim 5: a corner resting on a bore's wall whose edges run into the
/// hollow at a shallow angle reaches the far wall within `near`; every
/// ray reads Positive against the tangent plane and `by_round` reads
/// Rest without looking past first order.
#[test]
fn probe_by_round_concave_positive_arm_is_first_order() {
    // Bore of radius 1 about the z axis; touch at p = (1, 0, 0); the
    // hollow's outward normal (n) points to -x.
    let r = 1.0;
    let p = Point3::new(r, 0.0, 0.0);
    let theta: f64 = 0.05; // rays dip into the hollow by this angle
    let len = 0.4; // edge length; chord in the hollow is 2 r sin(theta) ~ 0.1
    // A thin corner: three rays, each `theta` into the hollow (-x),
    // fanned about +y / ±z.
    let dir = |phi: f64| {
        // tangent-plane direction (cos phi along y, sin phi along z), tipped in by theta
        v(
            -theta.sin(),
            theta.cos() * phi.cos(),
            theta.cos() * phi.sin(),
        )
    };
    let (d1, d2, d3) = (dir(-0.15), dir(0.15), dir(0.0) + v(-0.03, 0.0, 0.0));
    let d3 = d3.normalize();
    let fan = |x: Vec3<f64>, y: Vec3<f64>| {
        let m = x.cross(y).normalize();
        (m, vec![p + x * len, p + y * len])
    };
    // Orient the three faces outward of the thin cone x/y/z.
    let mut faces = vec![fan(d1, d2), fan(d2, d3), fan(d3, d1)];
    let centroid = (d1 + d2 + d3) * (1.0 / 3.0);
    for f in &mut faces {
        if f.0.dot(centroid) > 0.0 {
            f.0 = -f.0;
            f.1.reverse();
        }
    }
    let sectors = faces
        .iter()
        .map(|(m, pts)| Sector {
            surface: geom::Surface::Plane {
                origin: p,
                normal: *m,
                u_ref: (pts[0] - p).normalize(),
            },
            sense: true,
            outward: *m,
            apex: p,
            wedges: vec![(pts[0], pts[1])],
            closed: false,
        })
        .collect();
    let link = Link::of(p, sectors).expect("link");
    let round = Round::cylinder(Point3::new(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), r, false);
    let near = link.ball.min(0.5);
    let deepest = [d1, d2, d3]
        .iter()
        .map(|d| round.lowest_on((p, p + *d * near)))
        .fold(f64::INFINITY, f64::min);
    let got = rest::by_round(&link, &round, 0.5, band());
    println!(
        "link.ball {} near {near}; deepest ray point within near reads {deepest:e} against the bore; by_round: {got:?}",
        link.ball
    );
}
