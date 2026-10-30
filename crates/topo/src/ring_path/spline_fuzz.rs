//! **The ring lane's parity across spline arcs, searched for a wrong
//! answer** (shape 1: every sampled pair, the parity the membership
//! oracle gives or none). Two families:
//!
//! - the carried loop itself: a cylinder × sphere section's fitted loop
//!   (`geom_brep::PairSection::carry`), cut into one to three arcs with
//!   their evidence of lying on the surfaces ([`SplineArc::of_section`]),
//!   read on the cylinder against membership in the ball and on the
//!   sphere against membership in the cylinder, between points clustered
//!   from 1e-5 to 5e-2 off the loop; and a sphere × torus section's oval
//!   (the sphere cutting one tube circle of its meridian), read on the
//!   torus against membership in the ball and on the sphere against
//!   membership in the torus, and random sections of the torus by a ball
//!   or a rod parallel to its axis, every component read at once;
//! - islands on a cylinder bounded by two rulings and two tilted plane
//!   sections, every edge an exact spline (a degree-one ruling, rational
//!   quadratic arcs of the ellipses), whose extrema in height fall inside
//!   the island's span, read between random points and, arriving on a
//!   section arc's midpoint, against the side the path arrives from.

test_utils::gated_to![
    "crates/topo/src/ring_path.rs",
    "crates/geom-brep/src/curve_pair.rs",
    "crates/geom-brep/src/curve_pair/",
    "crates/geom-brep/src/pair_section.rs",
    "crates/geom/src/curves/nurbs.rs",
    "crates/geom-core/src/spline/",
    "crates/geom-core/src/predicate.rs",
];

use super::cone_islands::band;
use super::*;
use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::spline::KnotVector;
use test_utils::fuzz::{self, Rng};

/// The arcs of `net` cut at `cuts` (interior parameters, ascending), each
/// a spline arc on `component`'s evidence.
fn cut_into(component: &geom_brep::CarriedComponent<f64>, cuts: &[f64]) -> Vec<LoopArc<f64>> {
    let geom::Curve3::Nurbs(net) = &component.carrier else {
        panic!("a fitted component's carrier is a spline");
    };
    let mut rest = (**net).clone();
    let mut pieces = Vec::new();
    for &u in cuts {
        let (head, tail) = rest.split_at(u).unwrap();
        pieces.push(head);
        rest = tail;
    }
    pieces.push(rest);
    pieces
        .into_iter()
        .map(|piece| {
            let span = piece.domain();
            let curve = geom::Curve3::Nurbs(std::sync::Arc::new(piece));
            LoopArc::Spline(SplineArc::of_section(component, &curve, span).unwrap())
        })
        .collect()
}

/// A unit vector square to `n`, drawn from `rng`.
fn tangent(n: Vec3<f64>, rng: &mut Rng) -> Vec3<f64> {
    let (a, b) = n.orthonormal_basis();
    let t = rng.range(-PI, PI);
    a * t.cos() + b * t.sin()
}

/// **No path across a carried loop's arcs reads a parity the membership
/// oracle contradicts**, on the cylinder and on the sphere, over poses
/// of one loop and two.
#[test]
fn a_path_across_a_carried_loop_reads_the_membership_parity() {
    let mut rng = fuzz::start("ring_path::spline_fuzz::carried_loop");
    let r = 0.5;
    let cylinder = geom::Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    // One loop (centre on the wall), one wider than the wall, two loops.
    let poses = [(0.5, 0.3), (0.2, 0.699), (0.1, 0.7)];
    let (mut asked, mut decided) = (0usize, 0usize);
    for (d, big) in poses {
        let centre = Point3::new(d, 0.0, 0.0);
        let sphere = geom::Surface::Sphere {
            center: centre,
            radius: big,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let geom_brep::SectionRead::Built(section) =
            geom_brep::pair_section(&cylinder, &sphere, band()).unwrap()
        else {
            panic!("the value builds cylinder × sphere");
        };
        let carried = section
            .carry(Some(geom_brep::SsiLane::marched()), band())
            .unwrap();
        for _ in 0..fuzz::scaled(2) {
            // The loop's arcs, each component cut at up to two places.
            let mut arcs = Vec::new();
            for c in carried.components() {
                let (lo, hi) = c.span;
                let mut cuts: Vec<f64> = (0..rng.below(3))
                    .map(|_| rng.range(lo + 0.05 * (hi - lo), hi - 0.05 * (hi - lo)))
                    .collect();
                cuts.sort_by(f64::total_cmp);
                cuts.dedup_by(|a, b| (*a - *b).abs() < 0.01 * (hi - lo));
                arcs.extend(cut_into(c, &cuts));
            }
            // Points on each surface a clustered distance off the loop.
            let near = |rng: &mut Rng, on_sphere: bool| loop {
                let c = &carried.components()[rng.below(carried.components().len())];
                let p = c.carrier.eval(rng.range(c.span.0, c.span.1));
                let off = rng.log_uniform(1e-5, 5e-2);
                let q = if on_sphere {
                    let n = (p - centre) / big;
                    let moved = p + tangent(n, rng) * off - centre;
                    centre + moved / moved.norm() * big
                } else {
                    let turn = rng.range(-PI, PI);
                    let theta = p.y.atan2(p.x) + turn.cos() * off / r;
                    Point3::new(r * theta.cos(), r * theta.sin(), p.z + turn.sin() * off)
                };
                // Membership decided well past the loop's rounding.
                let margin = if on_sphere {
                    q.x.hypot(q.y) - r
                } else {
                    (q - centre).norm() - big
                };
                if margin.abs() > 1e-8 {
                    return (q, margin < 0.0);
                }
            };
            for (on_sphere, surface) in [(false, &cylinder), (true, &sphere)] {
                let quadric = Quadric::of(surface).unwrap();
                let (a, ia) = near(&mut rng, on_sphere);
                let (b, ib) = near(&mut rng, on_sphere);
                let Ok(paths) = quadric.paths((a, b), band()) else {
                    continue;
                };
                for path in paths {
                    asked += 1;
                    let got = path_parity(&path, &arcs, None, band());
                    if let Ok(Some(got)) = got {
                        decided += 1;
                        assert_eq!(
                            got,
                            ia != ib,
                            "d {d}, R {big}, on the {}: {a:?} → {b:?}; {}",
                            if on_sphere { "sphere" } else { "cylinder" },
                            fuzz::replay()
                        );
                    }
                }
            }
        }
    }
    println!("[fuzz] carried loop: {decided} of {asked} paths decided");
}

/// **No path across a sphere × torus oval's arcs reads a parity the
/// membership oracle contradicts**, on the torus and on the sphere, over
/// ovals on the tube's outer side, on its crest and on its inner side.
#[test]
fn a_path_across_a_carried_torus_oval_reads_the_membership_parity() {
    let mut rng = fuzz::start("ring_path::spline_fuzz::torus_oval");
    let (big_r, r) = (0.6, 0.2);
    let torus = geom::Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: big_r,
        minor_radius: r,
        u_ref: Vec3::unit_x(),
    };
    let on_torus = |q: Point3<f64>| {
        let core = Point3::new(q.x, q.y, 0.0);
        let k = Point3::origin() + (core - Point3::origin()) / core.x.hypot(core.y) * big_r;
        k + (q - k) / (q - k).norm() * r
    };
    let torus_margin = |q: Point3<f64>| (q.x.hypot(q.y) - big_r).hypot(q.z) - r;
    let poses = [
        (Point3::new(0.85, 0.0, 0.1), 0.15),
        (Point3::new(0.6, 0.1, 0.25), 0.1),
        (Point3::new(0.3, -0.1, -0.05), 0.2),
    ];
    let (mut asked, mut decided) = (0usize, 0usize);
    for (centre, big) in poses {
        let sphere = geom::Surface::Sphere {
            center: centre,
            radius: big,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let geom_brep::SectionRead::Built(section) =
            geom_brep::pair_section(&torus, &sphere, band()).unwrap()
        else {
            panic!("the value builds sphere × torus");
        };
        assert_eq!(
            section.counted().map(<[_]>::len),
            Some(1),
            "centre {centre:?}, radius {big}: the pose is one oval"
        );
        let carried = section
            .carry(Some(geom_brep::SsiLane::marched()), band())
            .unwrap();
        for _ in 0..fuzz::scaled(2) {
            let mut arcs = Vec::new();
            for c in carried.components() {
                let (lo, hi) = c.span;
                let mut cuts: Vec<f64> = (0..rng.below(3))
                    .map(|_| rng.range(lo + 0.05 * (hi - lo), hi - 0.05 * (hi - lo)))
                    .collect();
                cuts.sort_by(f64::total_cmp);
                cuts.dedup_by(|a, b| (*a - *b).abs() < 0.01 * (hi - lo));
                arcs.extend(cut_into(c, &cuts));
            }
            let near = |rng: &mut Rng, on_sphere: bool| loop {
                let c = &carried.components()[0];
                let p = c.carrier.eval(rng.range(c.span.0, c.span.1));
                let off = rng.log_uniform(1e-5, 5e-2);
                let q = if on_sphere {
                    let n = (p - centre) / big;
                    let moved = p + tangent(n, rng) * off - centre;
                    centre + moved / moved.norm() * big
                } else {
                    let foot = on_torus(p);
                    let core = Point3::new(foot.x, foot.y, 0.0);
                    let k =
                        Point3::origin() + (core - Point3::origin()) / core.x.hypot(core.y) * big_r;
                    on_torus(foot + tangent((foot - k) / r, rng) * off)
                };
                let margin = if on_sphere {
                    torus_margin(q)
                } else {
                    (q - centre).norm() - big
                };
                if margin.abs() > 1e-8 {
                    return (q, margin < 0.0);
                }
            };
            for (on_sphere, surface) in [(false, &torus), (true, &sphere)] {
                let quadric = Quadric::of(surface).unwrap();
                let (a, ia) = near(&mut rng, on_sphere);
                let (b, ib) = near(&mut rng, on_sphere);
                let Ok(paths) = quadric.paths((a, b), band()) else {
                    continue;
                };
                for path in paths {
                    asked += 1;
                    if let Ok(Some(got)) = path_parity(&path, &arcs, None, band()) {
                        decided += 1;
                        assert_eq!(
                            got,
                            ia != ib,
                            "centre {centre:?}, radius {big}, on the {}: {a:?} → {b:?}; {}",
                            if on_sphere { "sphere" } else { "torus" },
                            fuzz::replay()
                        );
                    }
                }
            }
        }
    }
    println!("[fuzz] torus oval: {decided} of {asked} paths decided");
}

/// A partner of the torus in [`random_torus_sections`]: a ball, or a
/// rod parallel to the torus axis, with its signed distance (negative
/// inside).
#[derive(Clone, Copy, Debug)]
enum Partner {
    Ball { centre: Point3<f64>, radius: f64 },
    Rod { x: f64, y: f64, radius: f64 },
}

impl Partner {
    fn surface(self) -> geom::Surface<f64> {
        match self {
            Self::Ball { centre, radius } => geom::Surface::Sphere {
                center: centre,
                radius,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Self::Rod { x, y, radius } => geom::Surface::Cylinder {
                origin: Point3::new(x, y, -2.0),
                axis: Vec3::unit_z(),
                radius,
                u_ref: Vec3::unit_x(),
            },
        }
    }

    fn margin(self, q: Point3<f64>) -> f64 {
        match self {
            Self::Ball { centre, radius } => (q - centre).norm() - radius,
            Self::Rod { x, y, radius } => (q.x - x).hypot(q.y - y) - radius,
        }
    }

    /// The unit outward normal at `q`, a point of the partner.
    fn normal(self, q: Point3<f64>) -> Vec3<f64> {
        match self {
            Self::Ball { centre, radius } => (q - centre) / radius,
            Self::Rod { x, y, radius } => Vec3::new(q.x - x, q.y - y, 0.0) / radius,
        }
    }

    /// The point of the partner nearest `q`.
    fn project(self, q: Point3<f64>) -> Point3<f64> {
        match self {
            Self::Ball { centre, radius } => centre + (q - centre) / (q - centre).norm() * radius,
            Self::Rod { x, y, radius } => {
                let (dx, dy) = (q.x - x, q.y - y);
                let h = dx.hypot(dy);
                Point3::new(x + dx / h * radius, y + dy / h * radius, q.z)
            }
        }
    }
}

/// **No path across a random torus section's arcs reads a parity the
/// membership oracle contradicts** (shape 1). A ball about the torus
/// (`R = 0.6`, `r = 0.2`), or a rod parallel to its axis, a third of
/// them within 1e-4..3e-2 of a tangency; every carried component cut
/// into one to three spline arcs and all of them read at once (the
/// section bounds the part of either surface inside the other solid, so
/// together the arcs separate even where one component winds round the
/// torus); each end near a random component or anywhere on the surface,
/// half of them pushed ±0.9 of the band's zero along the normal, read against
/// membership in the other solid past 1e-8. A pose the value refuses or
/// carries no fitted component on is passed over.
#[test]
fn random_torus_sections() {
    let mut rng = fuzz::start("ring_path::spline_fuzz::random_torus_sections");
    let (big_r, r) = (0.6, 0.2);
    let torus = geom::Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: big_r,
        minor_radius: r,
        u_ref: Vec3::unit_x(),
    };
    let core = |q: Point3<f64>| {
        let h = q.x.hypot(q.y);
        Point3::new(q.x / h * big_r, q.y / h * big_r, 0.0)
    };
    let torus_project = |q: Point3<f64>| {
        let k = core(q);
        k + (q - k) / (q - k).norm() * r
    };
    let torus_margin = |q: Point3<f64>| (q.x.hypot(q.y) - big_r).hypot(q.z) - r;
    let (mut poses, mut carried_poses, mut asked, mut decided) = (0usize, 0usize, 0usize, 0usize);
    for _ in 0..fuzz::scaled(10) {
        poses += 1;
        let near_tangent = rng.below(3) == 0;
        let partner = if rng.below(2) == 0 {
            let centre = Point3::new(
                rng.range(-0.9, 0.9),
                rng.range(-0.9, 0.9),
                rng.range(-0.4, 0.4),
            );
            let radius = if near_tangent {
                // A critical distance of the centre from the core, off by
                // a small amount: `|d ± r|` or `d + r` for a meridian `d`.
                let s = centre.x.hypot(centre.y);
                let d = [(s - big_r).hypot(centre.z), (s + big_r).hypot(centre.z)][rng.below(2)];
                let level = [d + r, (d - r).abs()][rng.below(2)];
                let off = rng.log_uniform(1e-4, 3e-2) * if rng.below(2) == 0 { 1.0 } else { -1.0 };
                level + off
            } else {
                rng.range(0.05, 0.9)
            };
            if radius <= 0.02 {
                continue;
            }
            Partner::Ball { centre, radius }
        } else {
            let (x, y) = (rng.range(-0.9, 0.9), rng.range(-0.9, 0.9));
            let radius = if near_tangent {
                let s = x.hypot(y);
                let level = [s - big_r - r, s - big_r + r, s + big_r - r, s + big_r + r]
                    [rng.below(4)]
                .abs();
                level + rng.log_uniform(1e-4, 3e-2) * if rng.below(2) == 0 { 1.0 } else { -1.0 }
            } else {
                rng.range(0.03, 0.5)
            };
            if radius <= 0.02 {
                continue;
            }
            Partner::Rod { x, y, radius }
        };
        let other = partner.surface();
        let Ok(geom_brep::SectionRead::Built(section)) =
            geom_brep::pair_section(&torus, &other, band())
        else {
            continue;
        };
        let Ok(carried) = section.carry(Some(geom_brep::SsiLane::marched()), band()) else {
            continue;
        };
        let components = carried.components();
        if components.is_empty()
            || components
                .iter()
                .any(|c| !matches!(c.carrier, geom::Curve3::Nurbs(_)) || !c.closed)
        {
            continue;
        }
        carried_poses += 1;
        let mut arcs = Vec::new();
        for c in components {
            let (lo, hi) = c.span;
            let mut cuts: Vec<f64> = (0..rng.below(3))
                .map(|_| rng.range(lo + 0.05 * (hi - lo), hi - 0.05 * (hi - lo)))
                .collect();
            cuts.sort_by(f64::total_cmp);
            cuts.dedup_by(|a, b| (*a - *b).abs() < 0.01 * (hi - lo));
            arcs.extend(cut_into(c, &cuts));
        }
        // An end on the torus (or the partner), with its membership in the
        // other solid.
        let end = |rng: &mut Rng, on_partner: bool| loop {
            let q = if rng.below(2) == 0 {
                let c = &components[rng.below(components.len())];
                let p = c.carrier.eval(rng.range(c.span.0, c.span.1));
                let off = rng.log_uniform(1e-5, 5e-2);
                if on_partner {
                    let n = partner.normal(partner.project(p));
                    partner.project(p + tangent(n, rng) * off)
                } else {
                    let foot = torus_project(p);
                    torus_project(foot + tangent((foot - core(foot)) / r, rng) * off)
                }
            } else if on_partner {
                match partner {
                    Partner::Ball { centre, radius } => {
                        let n = Vec3::new(
                            rng.range(-1.0, 1.0),
                            rng.range(-1.0, 1.0),
                            rng.range(-1.0, 1.0),
                        );
                        if n.norm() < 1e-3 {
                            continue;
                        }
                        centre + n / n.norm() * radius
                    }
                    Partner::Rod { x, y, radius } => {
                        let t = rng.range(-PI, PI);
                        Point3::new(
                            x + radius * t.cos(),
                            y + radius * t.sin(),
                            rng.range(-0.5, 0.5),
                        )
                    }
                }
            } else {
                let (u, v) = (rng.range(-PI, PI), rng.range(-PI, PI));
                let rho = big_r + r * v.cos();
                Point3::new(rho * u.cos(), rho * u.sin(), r * v.sin())
            };
            let margin = if on_partner {
                torus_margin(q)
            } else {
                partner.margin(q)
            };
            if margin.abs() <= 1e-8 {
                continue;
            }
            let q = if rng.below(2) == 0 {
                let n = if on_partner {
                    partner.normal(q)
                } else {
                    (q - core(q)) / r
                };
                q + n * (0.9 * band().zero() * if rng.below(2) == 0 { 1.0 } else { -1.0 })
            } else {
                q
            };
            return (q, margin < 0.0);
        };
        for on_partner in [false, true] {
            let surface = if on_partner { &other } else { &torus };
            let quadric = Quadric::of(surface).unwrap();
            let (a, ia) = end(&mut rng, on_partner);
            let (b, ib) = end(&mut rng, on_partner);
            let Ok(paths) = quadric.paths((a, b), band()) else {
                continue;
            };
            for path in paths {
                asked += 1;
                if let Ok(Some(got)) = path_parity(&path, &arcs, None, band()) {
                    decided += 1;
                    assert_eq!(
                        got,
                        ia != ib,
                        "{partner:?}, on the {}: {a:?} → {b:?}; {}",
                        if on_partner { "partner" } else { "torus" },
                        fuzz::replay()
                    );
                }
            }
        }
    }
    println!(
        "[fuzz] random torus sections: {carried_poses} of {poses} poses carried, \
         {decided} of {asked} paths decided"
    );
}

/// The rational quadratic arcs of `e(θ) = c + U·cos θ + V·sin θ` over
/// `[t0, t1]`, each spanning at most a quarter turn: exact.
fn conic_arcs(
    (c, u, v): (Point3<f64>, Vec3<f64>, Vec3<f64>),
    (t0, t1): (f64, f64),
) -> Vec<LoopArc<f64>> {
    let n = ((t1 - t0) / FRAC_PI_2).ceil().max(1.0);
    let step = (t1 - t0) / n;
    let e = |t: f64| c + u * t.cos() + v * t.sin();
    (0..n as usize)
        .map(|i| {
            let (a, b) = (t0 + step * i as f64, t0 + step * (i + 1) as f64);
            let (m, w) = (0.5 * (a + b), (0.5 * step).cos());
            let mid = c + (u * m.cos() + v * m.sin()) / w;
            let net = geom::NurbsCurve3::new(
                KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
                vec![e(a), mid, e(b)],
                vec![1.0, w, 1.0],
            )
            .unwrap();
            LoopArc::Spline(
                SplineArc::on_by_construction(
                    &geom::Curve3::Nurbs(std::sync::Arc::new(net)),
                    (0.0, 1.0),
                )
                .unwrap(),
            )
        })
        .collect()
}

/// A degree-one spline from `p` to `q`.
fn ruling(p: Point3<f64>, q: Point3<f64>) -> LoopArc<f64> {
    let net = geom::NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![p, q],
        vec![1.0, 1.0],
    )
    .unwrap();
    LoopArc::Spline(
        SplineArc::on_by_construction(&geom::Curve3::Nurbs(std::sync::Arc::new(net)), (0.0, 1.0))
            .unwrap(),
    )
}

/// **No path across a random cylinder island bounded by splines reads a
/// parity the membership oracle contradicts**, between points, and
/// arriving on a section arc against the side it arrives from.
#[test]
fn a_path_across_a_spline_bounded_cylinder_island_reads_the_membership_parity() {
    let mut rng = fuzz::start("ring_path::spline_fuzz::cylinder_island");
    let r = 0.5;
    let cylinder = geom::Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    let quadric = Quadric::of(&cylinder).unwrap();
    let (mut asked, mut decided) = (0usize, 0usize);
    for _ in 0..fuzz::scaled(12) {
        let ta = rng.range(-2.5, 0.0);
        let tb = ta + rng.range(0.4, 2.5);
        // Two plane sections `z = h + a·cos θ + b·sin θ`, the upper above
        // the lower over the span by at least a gap.
        let (al, bl, hl) = (
            rng.range(-0.3, 0.3),
            rng.range(-0.3, 0.3),
            rng.range(-0.5, 0.0),
        );
        let (au, bu) = (rng.range(-0.3, 0.3), rng.range(-0.3, 0.3));
        let z = |h: f64, a: f64, b: f64, t: f64| h + a * t.cos() + b * t.sin();
        let lowest_gap = (0..=200)
            .map(|i| {
                let t = ta + (tb - ta) * f64::from(i) / 200.0;
                z(0.0, au, bu, t) - z(hl, al, bl, t)
            })
            .fold(f64::INFINITY, f64::min);
        let hu = 0.05 - lowest_gap + rng.range(0.0, 0.6);
        let section = |h: f64, a: f64, b: f64| {
            (
                Point3::new(0.0, 0.0, h),
                Vec3::new(r, 0.0, a),
                Vec3::new(0.0, r, b),
            )
        };
        let (lower, upper) = (section(hl, al, bl), section(hu, au, bu));
        let at =
            |(c, u, v): (Point3<f64>, Vec3<f64>, Vec3<f64>), t: f64| c + u * t.cos() + v * t.sin();
        let mut arcs = conic_arcs(lower, (ta, tb));
        arcs.extend(conic_arcs(upper, (ta, tb)));
        arcs.push(ruling(at(lower, ta), at(upper, ta)));
        arcs.push(ruling(at(lower, tb), at(upper, tb)));
        let inside = |p: Point3<f64>| {
            let t = p.y.atan2(p.x);
            let (lo, hi) = (z(hl, al, bl, t), z(hu, au, bu, t));
            let m = [t - ta, tb - t, p.z - lo, hi - p.z]
                .into_iter()
                .fold(f64::INFINITY, f64::min);
            (m.abs() > 1e-7).then_some(m > 0.0)
        };
        let point = |rng: &mut Rng| {
            let t = rng.range(ta - 0.5, tb + 0.5);
            let zz = rng.range(hl - 0.8, hu + 0.8);
            Point3::new(r * t.cos(), r * t.sin(), zz)
        };
        for _ in 0..12 {
            let (a, b) = (point(&mut rng), point(&mut rng));
            let (Some(ia), Some(ib)) = (inside(a), inside(b)) else {
                continue;
            };
            for path in quadric.paths((a, b), band()).unwrap_or_default() {
                asked += 1;
                if let Ok(Some(got)) = path_parity(&path, &arcs, None, band()) {
                    decided += 1;
                    assert_eq!(got, ia != ib, "{a:?} → {b:?}; {}", fuzz::replay());
                }
            }
        }
        // Arriving on the lower section's first arc at its midpoint: the
        // parity before it says whether `w` lies on the side the path
        // arrives from.
        let first = (tb - ta) / ((tb - ta) / FRAC_PI_2).ceil().max(1.0);
        let q = at(lower, ta + first / 2.0);
        for _ in 0..4 {
            let w = point(&mut rng);
            let Some(iw) = inside(w) else {
                continue;
            };
            for path in quadric.paths((w, q), band()).unwrap_or_default() {
                let before = q - path.arrival * 1e-5;
                let Some(ib) = inside(Point3::new(
                    before.x / before.x.hypot(before.y) * r,
                    before.y / before.x.hypot(before.y) * r,
                    before.z,
                )) else {
                    continue;
                };
                asked += 1;
                if let Ok(Some(got)) = path_parity(&path, &arcs, Some(0), band()) {
                    decided += 1;
                    assert_eq!(got, iw != ib, "{w:?} → {q:?} arriving; {}", fuzz::replay());
                }
            }
        }
    }
    println!("[fuzz] cylinder islands: {decided} of {asked} paths decided");
}

/// **No spline arc is made without its evidence of lying on the
/// surface**: [`spline_crossings`] reads meetings in space, which are
/// crossings on the surface only for an arc on it, and an arc fifty bands
/// off a cylinder reads hundreds of wrong parities in a few thousand
/// paths (the dual review of PR 5894). So a closed-form arc's door makes
/// none, and a section's arc is refused where its component holds no
/// certificate.
#[test]
fn a_spline_arc_is_made_only_on_its_evidence() {
    let net = geom::NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![Point3::new(0.5, 0.0, 0.0), Point3::new(0.0, 0.5, 0.0)],
        vec![1.0, 1.0],
    )
    .unwrap();
    let curve = geom::Curve3::Nurbs(std::sync::Arc::new(net));
    assert!(
        LoopArc::of(&curve, (0.0, 1.0)).is_none(),
        "the closed-form door makes no spline arc"
    );
    let uncertified = geom_brep::CarriedComponent {
        carrier: curve.clone(),
        span: (0.0, 1.0),
        closed: false,
        certificate: None,
        winds: [false, false],
        pad: 0.0,
        witness: curve.eval(0.5),
    };
    assert!(
        SplineArc::of_section(&uncertified, &curve, (0.0, 1.0)).is_none(),
        "a component with no certificate gives no spline arc"
    );
}

/// **A path on a ball across the torus section, one crossing a
/// millimetre from its end, reads the membership parity** (shape 2):
/// `random_torus_sections`' pose for seed `0xfcf36c24b2674116`, its one
/// carried component uncut, the path from an end pushed 0.9 of the
/// band's zero off the ball to an end on it. An end pushed past the band
/// carries its offset round the whole arc, off the section it crosses:
/// the push is the band's, not a fixed length.
#[test]
fn a_ball_path_reads_a_crossing_near_its_end() {
    let torus = geom::Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 0.6,
        minor_radius: 0.2,
        u_ref: Vec3::unit_x(),
    };
    let torus_margin = |q: Point3<f64>| (q.x.hypot(q.y) - 0.6).hypot(q.z) - 0.2;
    let partner = Partner::Ball {
        centre: Point3::new(
            -0.539_789_177_441_680_1,
            -0.113_374_250_213_059_02,
            0.241_444_151_539_482_18,
        ),
        radius: 0.443_792_980_007_016_17,
    };
    let on = partner.project(Point3::new(
        -0.431_682_161_061_048_3,
        -0.539_031_459_910_225_6,
        0.177_561_268_507_832_7,
    ));
    let a = on + partner.normal(on) * (0.9 * band().zero());
    let b = partner.project(Point3::new(
        -0.365_914_426_294_379_4,
        -0.519_264_341_531_642,
        0.197_025_531_723_414_1,
    ));
    let other = partner.surface();
    let geom_brep::SectionRead::Built(section) =
        geom_brep::pair_section(&torus, &other, band()).unwrap()
    else {
        panic!("the value builds sphere × torus");
    };
    let carried = match section.carry(Some(geom_brep::SsiLane::marched()), band()) {
        Ok(carried) => carried,
        Err(why) => {
            eprintln!(
                "STOOD DOWN at {:?}: the section does not carry ({why:?})",
                band()
            );
            return;
        }
    };
    let arcs: Vec<_> = carried
        .components()
        .iter()
        .flat_map(|c| cut_into(c, &[]))
        .collect();
    assert_eq!(arcs.len(), 1, "the pose carries one closed component");
    let paths = Quadric::of(&other).unwrap().paths((a, b), band()).unwrap();
    assert_eq!(paths.len(), 1, "a ball has one path between two ends");
    assert_eq!(
        path_parity(&paths[0], &arcs, None, band()).unwrap(),
        Some((torus_margin(a) < 0.0) != (torus_margin(b) < 0.0)),
        "the path reads the oracle's parity across the crossing near its end"
    );
}
