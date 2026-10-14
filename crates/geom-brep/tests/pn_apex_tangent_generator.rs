//! **Plane × cone's tangent generator is the generator the plane
//! grazes.** A plane through the apex whose conic-type discriminant
//! decides Zero touches the cone along the generator where `g·n` is
//! extremal over the azimuth; the in-plane generators the residue
//! `cos(u − φ) = −cos α·c/(sin α·s)` solves for lie `√(2(1 − ratio))`
//! either side of it, which a plane dipping in by a sub-ε angle puts
//! ~√ε radians off. The line served is the extremum on both sides of the
//! tangency, in every frame, for either sign of `c = axis·normal`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/intersect.rs",
    "crates/geom-brep/tests/shared/"
];

use crate::shared::surf::at_apex;
use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::intersect::{PlaneConeSection, plane_cone_section};
use geom_core::{Bounds, Decide, Interval, Point3, Real, Vec3};

/// A cone frame: apex, axis, the radial direction the tangency lies
/// along, and the seam's turn from it about the axis.
struct Frame {
    name: &'static str,
    apex: Point3<f64>,
    axis: Vec3<f64>,
    radial: Vec3<f64>,
    seam_turn: f64,
}

fn frames() -> [Frame; 3] {
    let tilted = Vec3::new(1.0, 2.0, 2.0).normalize();
    let across = tilted.cross(Vec3::unit_z()).normalize();
    [
        Frame {
            name: "upright",
            apex: Point3::new(0.0, 0.0, 1.0),
            axis: Vec3::unit_z(),
            radial: Vec3::unit_x(),
            seam_turn: 0.0,
        },
        Frame {
            name: "turned",
            apex: Point3::new(0.3, -0.2, 1.1),
            axis: tilted,
            radial: across,
            seam_turn: 1.1,
        },
        Frame {
            name: "far",
            apex: Point3::new(400.0, -650.0, 300.0),
            axis: tilted,
            radial: across,
            seam_turn: -2.3,
        },
    ]
}

/// The item's table, `τ` in units of ε: the plane's normal leans
/// `α + τ` off the radial toward `−axis`, so `τ < 0` dips into the cone.
const TAUS: [f64; 5] = [-0.5, -0.1, 0.0, 0.1, 0.5];

/// The tangent generator served at scalar `T` for the cone and plane
/// given at `f64`, as each component's bracket; `None` for any other
/// outcome, which the caller reports with `row`.
fn served<T: Decide + Bounds>(
    cone: &Surface<f64>,
    plane: &Surface<f64>,
) -> Option<[(f64, f64); 3]> {
    let lift = |s: &Surface<f64>| s.map_scalar(<T as Real>::from_f64);
    let cone = lift(cone);
    let got = plane_cone_section(&lift(plane), &cone, &at_apex(&cone, T::one()), band());
    let Ok(PlaneConeSection::ApexTangentLine(Curve3::Line { dir, .. })) = got else {
        return None;
    };
    Some([dir.x, dir.y, dir.z].map(|x| (x.lo(), x.hi())))
}

#[test]
fn a_grazing_plane_serves_the_generator_it_grazes() {
    let alpha = core::f64::consts::FRAC_PI_6;
    let e = eps();
    for f in frames() {
        let (a, r) = (f.axis, f.radial);
        let ortho = a.cross(r);
        let (st, ct) = f.seam_turn.sin_cos();
        let cone = Surface::Cone {
            apex: f.apex,
            axis: a,
            half_angle: alpha,
            u_ref: r * ct + ortho * st,
        };
        let grazed = a * alpha.cos() + r * alpha.sin();
        for tau in TAUS.map(|t| t * e) {
            for flip in [1.0, -1.0] {
                let n = (r * (alpha + tau).cos() - a * (alpha + tau).sin()) * flip;
                let plane = Surface::Plane {
                    origin: f.apex,
                    normal: n,
                    u_ref: ortho,
                };
                let row = format!("{} τ = {tau:e} c = {:+.3} ε = {e:e}", f.name, a.dot(n));
                for (scalar, got) in [
                    ("f64", served::<f64>(&cone, &plane)),
                    ("Interval", served::<Interval>(&cone, &plane)),
                ] {
                    let dir = got.unwrap_or_else(|| {
                        panic!("{row} at {scalar}: expected the tangent generator")
                    });
                    for ((lo, hi), want) in dir.into_iter().zip([grazed.x, grazed.y, grazed.z]) {
                        let off = (lo - want).abs().max((hi - want).abs());
                        assert!(
                            off < 1e-12,
                            "{row} at {scalar}: the line served stands {off:e} off the \
                             grazed generator"
                        );
                    }
                }
            }
        }
    }
}
