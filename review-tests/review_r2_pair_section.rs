//! Reviewer R2 probes for PR 5872: the cylinder × sphere pose against
//! the formulas it retired, and the component / foot reading near the
//! node of the section.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::{ComponentShape, PairSection, SectionError, SectionRead, SsiLane};
use geom_core::{Band, Point3, Sign, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A tiny deterministic generator.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(
                2.0 * self.next() - 1.0,
                2.0 * self.next() - 1.0,
                2.0 * self.next() - 1.0,
            );
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v * (1.0 / n);
            }
        }
    }
}

fn square_to(a: Vec3<f64>) -> Vec3<f64> {
    let t = if a.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let u = t - a * t.dot(a);
    u * (1.0 / u.norm())
}

fn cylinder(origin: Point3<f64>, axis: Vec3<f64>, r: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin,
        axis,
        radius: r,
        u_ref: square_to(axis),
    }
}

fn sphere(center: Point3<f64>, big: f64) -> Surface<f64> {
    Surface::Sphere {
        center,
        radius: big,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn sign(name: &'static str, m: f64) -> Option<Sign> {
    geom_core::k_stats::decide(name, geom_core::Margin::of(m), band()).ok()
}

/// The retired `glue::frame_rows` row: `cylinder_sphere_section` as it
/// stood on main (coaxial first, then the radius guards, then the wall
/// reach), answering whether a coincidence row was written.
fn main_row(origin: Point3<f64>, axis: Vec3<f64>, r: f64, c: Point3<f64>, big: f64) -> bool {
    let q = c - origin;
    let d = (q - axis * q.dot(axis)).norm();
    if sign("x", d) != Some(Sign::Zero) {
        return false;
    }
    if sign("x", r) != Some(Sign::Positive) || sign("x", big) != Some(Sign::Positive) {
        return false;
    }
    sign("x", big - r).is_some()
}

/// The retired `section_cert::sphere_cylinder` arm's verdict class: the
/// number of components, whether a tangency/touch, as main read it.
#[derive(Debug, PartialEq)]
enum Old {
    None,
    Two,
    One,
    Touch,
    Tangent,
}

fn main_guard(origin: Point3<f64>, axis: Vec3<f64>, r: f64, c: Point3<f64>, big: f64) -> Old {
    let w = c - origin;
    let perp = axis.cross(w.cross(axis));
    let e = perp.norm();
    let reach = sign("x", big - (e - r).abs());
    let girdle = sign("x", e + r - big);
    // `first_pinch` reads the reach first: a Zero or undecided row is
    // the pinch; a Zero reach beside a Positive girdle is the touch.
    match (reach, girdle) {
        (None, _) => Old::Tangent,
        (Some(Sign::Zero), Some(Sign::Positive)) => Old::Touch,
        (Some(Sign::Zero), _) => Old::Tangent,
        (Some(_), None | Some(Sign::Zero)) => Old::Tangent,
        (Some(Sign::Negative), Some(_)) => Old::None,
        (Some(Sign::Positive), Some(Sign::Positive)) => Old::One,
        (Some(Sign::Positive), Some(Sign::Negative)) => Old::Two,
    }
}

fn new_guard(section: &Result<SectionRead<f64>, SectionError>) -> Option<Old> {
    let s = match section {
        Ok(SectionRead::Built(s)) => s,
        Ok(SectionRead::Elsewhere(_)) => panic!("built"),
        Err(SectionError::Escalated(_) | SectionError::RadiusEscalated { .. }) => {
            return Some(Old::Tangent);
        }
        Err(_) => return None,
    };
    if s.tangency().is_some() {
        return Some(match s.components() {
            [c] if c.shape == ComponentShape::Point => Old::Touch,
            _ => Old::Tangent,
        });
    }
    Some(match s.components().len() {
        0 => Old::None,
        1 => Old::One,
        2 => Old::Two,
        _ => panic!("three components"),
    })
}

/// **Claims 5 and 6 at the value**: over random poses in both operand
/// orders, the coincidence row is written exactly where main wrote it,
/// and the guard's verdict class is main's, wherever the offset is not
/// in the band (the in-band ruling is the one intended move).
#[test]
fn r2_the_pose_agrees_with_the_retired_formulas() {
    let mut g = Lcg(0x5872);
    let b = band();
    let (mut rows, mut moved_in_band, mut checked) = (0, 0, 0);
    for k in 0..20_000 {
        let origin = Point3::new(g.next() - 0.5, g.next() - 0.5, g.next() - 0.5);
        let axis = g.unit();
        let r = 0.05 + g.next();
        let big = 0.05 + 2.0 * g.next();
        // A third coaxial, a third near the band, a third anywhere; and
        // radii and reaches pushed onto their tangencies now and then.
        let off = match k % 3 {
            0 => 0.0,
            1 => b.escalate() * 3.0 * g.next(),
            _ => 1.5 * g.next(),
        };
        let big = match k % 7 {
            0 => r,
            1 => (off - r).abs(),
            2 => off + r,
            3 => (off - r).abs() + b.zero() * 0.5,
            _ => big,
        };
        if big <= 0.0 {
            continue;
        }
        let side = square_to(axis);
        let c = origin + axis * (2.0 * g.next() - 1.0) + side * off;
        let (cyl, sph) = (cylinder(origin, axis, r), sphere(c, big));
        for (x, y) in [(&cyl, &sph), (&sph, &cyl)] {
            let read = geom_brep::pair_section(x, y, b);
            let new_row = matches!(&read, Ok(SectionRead::Built(s)) if s.coincidence().is_some());
            assert_eq!(
                new_row,
                main_row(origin, axis, r, c, big),
                "pose {k}: the coincidence row moved (off {off:e}, r {r}, R {big})"
            );
            rows += usize::from(new_row);
            let offset_in_band = off > b.zero() * 0.999 && off < b.escalate() * 1.001;
            let old = main_guard(origin, axis, r, c, big);
            let new = new_guard(&read).expect("positive radii");
            if offset_in_band {
                moved_in_band += usize::from(old != new);
                continue;
            }
            checked += 1;
            // Main read a coaxial ball by the same reach and girdle,
            // which are `±(R − r)` there.
            assert_eq!(new, old, "pose {k}: the guard moved (off {off:e}, r {r}, R {big})");
        }
    }
    println!("rows {rows}, guard checked {checked}, moved in band {moved_in_band}");
    assert!(rows > 1000 && checked > 20_000);
}

/// The analytic section of a radius-`r` cylinder about `z` and a sphere
/// of radius `big` about `(d, 0, 0.1)`: the wall point at azimuth `θ`,
/// on the upper (`+1`) or lower (`−1`) sheet; `None` where there is none.
fn wall_point(r: f64, d: f64, big: f64, theta: f64, sheet: f64) -> Option<Point3<f64>> {
    let h2 = big * big - r * r - d * d + 2.0 * r * d * theta.cos();
    (h2 >= 0.0).then(|| Point3::new(r * theta.cos(), r * theta.sin(), 0.1 + sheet * h2.sqrt()))
}

fn carried(r: f64, d: f64, big: f64) -> (PairSection<f64>, geom_brep::CarriedSection<f64>) {
    let cyl = cylinder(Point3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0), r);
    let sph = sphere(Point3::new(d, 0.0, 0.1), big);
    let Ok(SectionRead::Built(s)) = geom_brep::pair_section(&cyl, &sph, band()) else {
        panic!("built");
    };
    let c = s.carry(Some(SsiLane::marched()), band()).expect("carried");
    (s, c)
}

/// Whether the cyclic sequence `ts` on a closed span of length `period`
/// runs one way round exactly once.
fn turns_once(ts: &[f64], period: f64) -> bool {
    let steps: Vec<f64> = ts
        .iter()
        .zip(ts.iter().cycle().skip(1))
        .map(|(a, b)| (b - a).rem_euclid(period))
        .collect();
    let fwd: f64 = steps.iter().sum();
    let back: f64 = steps.iter().map(|s| period - s).sum();
    (fwd - period).abs() < 1e-6 * period || (back - period).abs() < 1e-6 * period
}

/// **Claim 2 near the node, two loops**: `R` a hair past `d + r`, so the
/// loops nearly meet at the far ruling. Every analytic site lies on the
/// component `component_of` names and `foot` agrees, and the feet run
/// round each loop in the azimuth's order, once.
#[test]
fn r2_two_loops_near_the_node_keep_their_sites_and_order() {
    for gap in [1e-2, 1e-3] {
        let (r, d) = (0.5, 0.2);
        let big = r + d + gap;
        let (s, c) = carried(r, d, big);
        assert_eq!(c.components().len(), 2, "gap {gap}");
        for (sheet, want) in [(1.0, 0), (-1.0, 1)] {
            let mut ts = Vec::new();
            for i in 0..97 {
                // Densest near the far ruling, θ = π.
                let u = f64::from(i) / 97.0;
                let theta = std::f64::consts::PI * (1.0 + (2.0 * u - 1.0).powi(3));
                let p = wall_point(r, d, big, theta, sheet).unwrap();
                assert_eq!(s.component_of(p, band()), Ok(want), "gap {gap} θ {theta}");
                let f = c.foot(p, band()).unwrap_or_else(|e| panic!("gap {gap} θ {theta}: {e:?}"));
                assert_eq!(f.component, want, "gap {gap} θ {theta}");
                ts.push(f.t);
            }
            let (lo, hi) = c.components()[want].span;
            assert!(turns_once(&ts, hi - lo), "gap {gap} sheet {sheet}: {ts:?}");
        }
    }
}

/// **Claim 2 near the node, one loop**: `R` a hair short of `d + r`. The
/// loop's two lobes pass close at the far side; every site's foot lies
/// on the one component and the feet run round it once in the loop's own
/// order.
#[test]
fn r2_one_loop_near_the_node_keeps_its_order() {
    for gap in [1e-2, 1e-3] {
        let (r, d) = (0.5, 0.2);
        let big = r + d - gap;
        let (_, c) = carried(r, d, big);
        assert_eq!(c.components().len(), 1);
        let t0 = ((r * r + d * d - big * big) / (2.0 * r * d)).acos();
        let mut ts = Vec::new();
        for i in 0..200 {
            let tau = 4.0 * t0 * f64::from(i) / 200.0;
            let (theta, sheet) = if tau < 2.0 * t0 {
                (tau - t0, 1.0)
            } else {
                (3.0 * t0 - tau, -1.0)
            };
            let Some(p) = wall_point(r, d, big, theta, sheet) else {
                continue;
            };
            let f = c.foot(p, band()).unwrap_or_else(|e| panic!("gap {gap} τ {tau}: {e:?}"));
            ts.push(f.t);
        }
        let (lo, hi) = c.components()[0].span;
        assert!(turns_once(&ts, hi - lo), "gap {gap}: {ts:?}");
    }
}
