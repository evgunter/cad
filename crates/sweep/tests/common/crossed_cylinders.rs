//! **The volume of a boolean of two finite cylinders whose axes are not
//! parallel**, sliced: every plane parallel to both axes (normal
//! `n = â₁ × â₂`) cuts each solid in a rectangle — the strip of
//! half-width `√(r² − (s − s₀)²)` about the axis' trace, `s₀` the axis'
//! offset along `n`, clipped by the caps, whose traces are square to the
//! axis — so each slice's area is a convex polygon clip, exact, and the
//! volume is one integral in `s` by adaptive 7–15 Gauss–Kronrod, split
//! where a strip opens or closes.
//!
//! The oracle is absolute: each op's slice area is its own (`A ∩ B` the
//! clip, `A ∪ B` and `A ∖ B` the strips' areas less it), so no op's
//! volume is read off another op's measurement.

use crate::common::differential::{area, ccw, clip_convex};
use crate::common::quadrature::gk15;
use geom_core::{Affine3, Point3};

/// A solid cylinder: its axis from `base` along the unit `axis` for
/// `length`, radius `radius`.
#[derive(Clone, Copy, Debug)]
pub struct Cylinder {
    /// The centre of the first cap.
    pub base: [f64; 3],
    /// The unit axis.
    pub axis: [f64; 3],
    /// The radius.
    pub radius: f64,
    /// The axis' length.
    pub length: f64,
}

/// A boolean operation.
#[derive(Clone, Copy, Debug)]
pub enum Op {
    /// `A ∪ B`.
    Union,
    /// `A ∖ B`.
    Subtract,
    /// `A ∩ B`.
    Intersect,
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn unit(a: [f64; 3]) -> [f64; 3] {
    let n = dot(a, a).sqrt();
    [a[0] / n, a[1] / n, a[2] / n]
}

impl Cylinder {
    /// The cylinder about `z` of radius `r` over `z ∈ [z0, z1]`, its axis
    /// through `(x, y)`.
    pub fn upright(x: f64, y: f64, r: f64, (z0, z1): (f64, f64)) -> Self {
        Self {
            base: [x, y, z0],
            axis: [0.0, 0.0, 1.0],
            radius: r,
            length: z1 - z0,
        }
    }

    /// The cylinder carried by the rigid map `m`.
    pub fn under(self, m: Affine3<f64>) -> Self {
        let at = |p: [f64; 3]| {
            let q = m.transform_point(Point3::new(p[0], p[1], p[2]));
            [q.x, q.y, q.z]
        };
        let base = at(self.base);
        let tip = at([
            self.base[0] + self.axis[0],
            self.base[1] + self.axis[1],
            self.base[2] + self.axis[2],
        ]);
        Self {
            base,
            axis: unit([tip[0] - base[0], tip[1] - base[1], tip[2] - base[2]]),
            ..self
        }
    }

    /// Its slice by the plane `p·n = s`, in the plane's basis
    /// `(e1, e2)`: the rectangle, or nothing.
    fn slice(&self, n: [f64; 3], e: [[f64; 3]; 2], s: f64) -> Option<Vec<(f64, f64)>> {
        let off = s - dot(self.base, n);
        let w2 = self.radius * self.radius - off * off;
        if w2 <= 0.0 {
            return None;
        }
        let w = w2.sqrt();
        let q = (dot(self.base, e[0]), dot(self.base, e[1]));
        let d = (dot(self.axis, e[0]), dot(self.axis, e[1]));
        let perp = (-d.1 * w, d.0 * w);
        let end = (q.0 + d.0 * self.length, q.1 + d.1 * self.length);
        Some(ccw(vec![
            (q.0 + perp.0, q.1 + perp.1),
            (q.0 - perp.0, q.1 - perp.1),
            (end.0 - perp.0, end.1 - perp.1),
            (end.0 + perp.0, end.1 + perp.1),
        ]))
    }
}

/// `∫ f` over `[a, b]` to `tol`, by adaptive 7–15 Gauss–Kronrod.
fn gk(f: &dyn Fn(f64) -> f64, a: f64, b: f64, tol: f64, depth: u32) -> f64 {
    let (k, err) = gk15(f, a, b);
    if err <= tol || depth == 0 {
        k
    } else {
        let c = (a + b) / 2.0;
        gk(f, a, c, tol / 2.0, depth - 1) + gk(f, c, b, tol / 2.0, depth - 1)
    }
}

/// **The oracle**: the volume of `a op b`, two cylinders whose axes are
/// not parallel (module docs).
///
/// # Panics
///
/// Where the axes are parallel.
pub fn volume(a: Cylinder, b: Cylinder, op: Op) -> f64 {
    let n = cross(a.axis, b.axis);
    assert!(dot(n, n) > 1e-12, "the axes are parallel");
    let n = unit(n);
    let e = [a.axis, cross(n, a.axis)];
    let slice = |s: f64| {
        let (pa, pb) = (a.slice(n, e, s), b.slice(n, e, s));
        let size = |p: &Option<Vec<(f64, f64)>>| p.as_ref().map_or(0.0, |p| area(p));
        let shared = match (&pa, &pb) {
            (Some(pa), Some(pb)) => {
                let c = clip_convex(pa, pb);
                if c.len() < 3 { 0.0 } else { area(&c).abs() }
            }
            _ => 0.0,
        };
        match op {
            Op::Intersect => shared,
            Op::Union => size(&pa) + size(&pb) - shared,
            Op::Subtract => size(&pa) - shared,
        }
    };
    let mut cuts: Vec<f64> = [a, b]
        .iter()
        .flat_map(|c| {
            let s0 = dot(c.base, n);
            [s0 - c.radius, s0 + c.radius]
        })
        .collect();
    cuts.sort_by(f64::total_cmp);
    cuts.windows(2)
        .map(|w| gk(&slice, w[0], w[1], 1e-13, 40))
        .sum()
}

/// **The oracle holds the closed forms it can be checked against**: the
/// Steinmetz solid `16r³/3` of two equal perpendicular cylinders whose
/// axes meet, and `16r³/(3·sin θ)` at the angle `θ` between them, each
/// long enough to pass through the other; and the operands' own
/// volumes, through `A ∪ B − A ∩ B = |A| + |B| − 2|A ∩ B|` read from
/// the ops' independent slices.
#[test]
fn the_oracle_holds_its_closed_forms() {
    for (theta, r) in [(core::f64::consts::FRAC_PI_2, 1.0), (0.7, 0.6)] {
        let a = Cylinder::upright(0.0, 0.0, r, (-10.0, 10.0));
        let b = a.under(Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            geom_core::Vec3::new(1.0, 0.0, 0.0),
            theta,
        ));
        let want = 16.0 * r * r * r / (3.0 * theta.sin());
        let got = volume(a, b, Op::Intersect);
        assert!(
            (got - want).abs() < 1e-10,
            "θ = {theta}: {got} against {want}"
        );
        let each = core::f64::consts::PI * r * r * 20.0;
        let union = volume(a, b, Op::Union);
        let minus = volume(a, b, Op::Subtract);
        assert!(
            (union - (2.0 * each - got)).abs() < 1e-9 && (minus - (each - got)).abs() < 1e-9,
            "θ = {theta}: ∪ {union}, ∖ {minus}, ∩ {got}"
        );
    }
}
