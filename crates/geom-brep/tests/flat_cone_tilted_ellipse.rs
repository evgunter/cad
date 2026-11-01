//! **Plane × cone's tilted ellipse stands on the cone and the plane to
//! its own resolution**, however flat the cone and however near square
//! to the axis the plane. The ellipse's semi-axes and centre divide by
//! `K = c² − sin²α`, which on a flat cone cut near square to its axis is
//! of order `(π/2 − α)²` while `c` and `sin α` sit near 1; the served
//! carrier must still stand within a few `u` of its own size off both
//! surfaces (`u = f64::EPSILON/2`), the rounding its stored fields
//! carry and no more.
//!
//! The oracle never reads the arm's construction: it evaluates the
//! served ellipse's points from their stored fields in double-double
//! arithmetic and measures each against the stored cone and plane
//! ([`ConeCut::ellipse_miss`]).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/intersect.rs",
    "crates/geom-brep/tests/shared/",
    "crates/test-utils/src/cone_cut.rs"
];

use crate::shared::surf::at_apex;
use crate::shared::tol::band;
use core::f64::consts::FRAC_PI_2;
use geom::{Curve3, Surface};
use geom_brep::intersect::{PlaneConeSection, plane_cone_section};
use geom_core::{Point3, Vec3};
use test_utils::cone_cut::ConeCut;
use test_utils::fuzz;

/// How many `u` of the carrier's size a served ellipse may stand off
/// the cut: its centre, semi-axes and directions are each one rounding
/// of an exact value, and evaluating them composes a few more.
const RESOLUTION_ULPS: f64 = 16.0;

/// A cone of half-angle `alpha` about `axis` from `apex`, and the plane
/// `gap` off the apex whose normal leans `tilt` off the axis toward
/// `toward` (reversed where `flip`, so `axis·normal < 0`), its section
/// consumed `lever` from the apex.
struct Pose {
    apex: Point3<f64>,
    axis: Vec3<f64>,
    alpha: f64,
    tilt: f64,
    toward: Vec3<f64>,
    gap: f64,
    lever: f64,
    flip: bool,
}

impl Pose {
    fn normal(&self) -> Vec3<f64> {
        let a = self.axis;
        let perp = (self.toward - a * self.toward.dot(a)).normalize();
        let n = (a * self.tilt.cos() + perp * self.tilt.sin()).normalize();
        if self.flip { -n } else { n }
    }

    fn surfaces(&self) -> (Surface<f64>, Surface<f64>) {
        let (a, n) = (self.axis, self.normal());
        let seam = |d: Vec3<f64>| {
            let w = if d.x.abs() < 0.9 {
                Vec3::unit_x()
            } else {
                Vec3::unit_y()
            };
            (w - d * w.dot(d)).normalize()
        };
        let cone = Surface::Cone {
            apex: self.apex,
            axis: a,
            half_angle: self.alpha,
            u_ref: seam(a),
        };
        let plane = Surface::Plane {
            origin: self.apex - n * self.gap,
            normal: n,
            u_ref: seam(n),
        };
        (cone, plane)
    }

    /// The served ellipse's miss off the cut in units of its resolution,
    /// `u·max(larger semi-axis, |centre|, |apex|)`; `None` where the arm serves no
    /// tilted ellipse.
    fn miss_in_ulps(&self) -> Option<f64> {
        let (cone, plane) = self.surfaces();
        let Ok(PlaneConeSection::TiltedEllipse(Curve3::Ellipse {
            center,
            axis,
            semi_u,
            semi_v,
            u_ref,
        })) = plane_cone_section(&plane, &cone, &at_apex(&cone, self.lever), band())
        else {
            return None;
        };
        let Surface::Plane { origin, normal, .. } = plane else {
            unreachable!()
        };
        let p = |x: Point3<f64>| [x.x, x.y, x.z];
        let w = |x: Vec3<f64>| [x.x, x.y, x.z];
        let cut = ConeCut::new(
            p(self.apex),
            w(self.axis),
            self.alpha,
            (p(origin), w(normal)),
        );
        let miss = cut.ellipse_miss(p(center), w(axis), semi_u, semi_v, w(u_ref));
        let size = Curve3::semi_axes_by_size(semi_u, semi_v)
            .0
            .max(center.distance(Point3::origin()))
            .max(self.apex.distance(Point3::origin()));
        Some(miss / (0.5 * f64::EPSILON * size))
    }
}

/// Flat cones (`π/2 − α` from 1e-5 to 1e-3) cut off the apex by planes
/// leaning a fraction of that off square to the axis, in a generic
/// frame: ellipses of 1e3 to 1e6 m whose `K` is 1e-11 to 1e-6, each
/// consumed over the axis-normal circle's radius `|gap|/(π/2 − α)`, so
/// the conic-type margin clears the band at every ε row. Half the rows,
/// a checkerboard over the three axes, reverse the normal.
#[test]
fn a_flat_cone_cut_near_square_serves_an_ellipse_on_the_cut() {
    let axis = Vec3::new(0.3, -0.5, 0.81).normalize();
    let toward = Vec3::new(-0.7, 0.2, 0.4);
    let apex = Point3::new(3.1, -2.7, 5.3);
    let mut rows = Vec::new();
    for (i, flat) in [1e-5, 1e-4, 1e-3].into_iter().enumerate() {
        for (j, lean) in [0.3, 0.6, 0.85].into_iter().enumerate() {
            for (k, gap) in [0.7, -2.3].into_iter().enumerate() {
                let flip = (i + j + k) % 2 == 1;
                let pose = Pose {
                    apex,
                    axis,
                    alpha: FRAC_PI_2 - flat,
                    tilt: lean * flat,
                    toward,
                    gap,
                    lever: gap.abs() / flat,
                    flip,
                };
                let miss = pose.miss_in_ulps().unwrap_or_else(|| {
                    panic!("flat {flat:e}, lean {lean}, gap {gap}, flip {flip}: no ellipse")
                });
                rows.push((flat, lean, gap, flip, miss));
            }
        }
    }
    let off: Vec<_> = rows
        .iter()
        .filter(|&&(.., miss)| miss > RESOLUTION_ULPS)
        .collect();
    assert!(
        off.is_empty(),
        "served ellipses stand past {RESOLUTION_ULPS} u of their size off the cut \
         (flat, lean, gap, flip, miss in u·size): {off:?}"
    );
}

/// Flat, needle and ordinary cones in random frames, cut at a random
/// fraction of the tilt that keeps the section an ellipse, the normal
/// reversed at random.
#[test]
fn every_served_tilted_ellipse_stands_on_the_cut() {
    let mut g = fuzz::start("flat_cone_tilted_ellipse::every_served");
    let unit = |g: &mut fuzz::Rng| loop {
        let v = Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0));
        if (0.3..1.0).contains(&v.norm()) {
            return v.normalize();
        }
    };
    let mut served = 0;
    let mut worst = (0.0_f64, 0.0, 0.0);
    for i in 0..fuzz::scaled(600) {
        let alpha = match i % 3 {
            0 => FRAC_PI_2 - g.log_uniform(1e-6, 1e-1),
            1 => g.log_uniform(1e-5, 1e-1),
            _ => g.range(0.1, 1.47),
        };
        let pose = Pose {
            apex: Point3::new(
                g.range(-10.0, 10.0),
                g.range(-10.0, 10.0),
                g.range(-10.0, 10.0),
            ),
            axis: unit(&mut g),
            alpha,
            tilt: g.range(0.0, 0.97) * (FRAC_PI_2 - alpha),
            toward: unit(&mut g),
            gap: g.range(0.3, 3.0) * if g.unit() < 0.5 { 1.0 } else { -1.0 },
            lever: 1.0,
            flip: g.unit() < 0.5,
        };
        if let Some(miss) = pose.miss_in_ulps() {
            served += 1;
            if miss > worst.0 {
                worst = (miss, pose.alpha, pose.tilt);
            }
        }
    }
    eprintln!(
        "served {served}; worst miss {:.3} u·size at α {:e}, tilt {:e} ({})",
        worst.0,
        worst.1,
        worst.2,
        fuzz::replay()
    );
    assert!(
        worst.0 <= RESOLUTION_ULPS,
        "a served ellipse stands {:.3e} u of its size off the cut at α {:e}, tilt {:e} ({})",
        worst.0,
        worst.1,
        worst.2,
        fuzz::replay()
    );
}
