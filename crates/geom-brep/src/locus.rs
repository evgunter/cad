//! **The closed-form tangent locus** — the DEV-1 witness lane: the
//! contact line of a tangent carrier pair, for exactly the
//! configurations whose locus is closed-form. Every numeric decision is
//! a named row through `dihedral::decide`.

use geom::Surface;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use crate::dihedral::decide;
use crate::extent::ExtentBall;
use crate::intersect::{
    PlaneCylinder, RuledSection, cylinder_axes_parallel, parallel_cylinder_gap,
    plane_cylinder_ruled,
};

/// **The certified-lane tangent LOCUS** (M9-2, the M9-1 PR-2 DEV-1
/// ruling): the closed-form contact line of a tangent carrier pair,
/// for exactly the configurations whose locus IS closed-form — a
/// plane and a cylinder tangent along a ruling, and two PARALLEL
/// cylinders tangent along the line between closest generators.
///
/// Its consumers are `topo`'s: the flush detector's `Tangent` arm (a
/// tangency finding without a locus is one the verifier cannot check)
/// and every at-rest verification that mints the witness a `Tangent`
/// declaration is verified along.
#[derive(Clone, Copy, Debug)]
pub enum TangentLocus<T: Real> {
    /// The tangent line: `origin + t·dir`, `dir` unit (both certified
    /// carriers are ruled along it).
    Line {
        /// A point on the locus.
        origin: Point3<T>,
        /// The locus direction (the shared ruling / axis direction).
        dir: Vec3<T>,
    },
}

/// Typed refusal of [`tangent_locus`] (closed enum, D3 style).
#[derive(Debug)]
pub enum TangentLocusError {
    /// A margin landed in the sliver band.
    Escalated(Indeterminate),
    /// The pair is definitely NOT tangent: `apart` distinguishes the
    /// definite-clearance side from the definite-crossing side.
    NotTangent {
        /// `true`: definite clearance; `false`: definite crossing.
        apart: bool,
        /// The row whose definite verdict refused the tangency.
        predicate: &'static str,
    },
    /// The configuration is outside the closed-form lane (kinds other
    /// than plane×cylinder / parallel cylinders, or a non-parallel
    /// axis relation): the demanded set IS the certifiable set — no
    /// sampled locus, ever.
    Unsupported {
        /// Why the configuration has no closed-form locus.
        what: &'static str,
    },
}

/// The closed-form tangent locus of two carriers (see
/// [`TangentLocus`]). Kind dispatch is structural; every numeric
/// decision is a named three-outcome row, and the tangency itself is
/// the section classifiers' verdict, read through their own rows:
///
/// - **plane×cylinder** is [`crate::plane_cylinder_section`]'s
///   axis-in-plane lane, run on the cylinder re-based to the foot of
///   `reach`'s centre on its axis, its axis row levered by `reach`'s
///   extent from there: `pc_axis_plane_parallel` (a definite tilt is
///   `Unsupported`) then `pc_parallel_gap` (`TangentLine` mints its
///   ruling, `ParallelLines` crosses, `Empty` clears). A tilt moves
///   the ruling by its angle times the distance from the foot, so the
///   axis row reads the displacement it induces across the faces the
///   locus is consumed on; with the gap row it mints only where each
///   reads zero, so the ruling stands within two zero bands of the
///   carriers across the faces, and the `Tangent` table then verifies
///   it sample by sample.
/// - **parallel cylinders** read [`crate::cylinder_cylinder_section`]'s
///   rows at the same lever from the second cylinder's foot:
///   `cc_axes_parallel`, then `cc_parallel_gap`, the external margin
///   `r1 + r2 − d` (Zero mints the ruling, Negative clears). Where the
///   walls cross that margin's way, `tangent_locus_internal_gap` reads
///   `|r1 − r2| − d` — an internal tangency, which no section
///   classifier holds — and `tangent_locus_side` places its generator.
///
/// `reach` is the declared pair's consumed extent, the one `topo`'s
/// carrier-pair doors lever their ladder at.
///
/// **CONTRACT — the separation invariant** (consumed by the reduce
/// sweep's declared-cover rung): every configuration this lane mints
/// a locus for has each carrier wholly in ONE closed residual
/// half-space of the other — a plane tangent to a cylinder has the
/// whole cylinder on one side; each of two externally (or
/// internally) tangent parallel cylinders is one-signed against the
/// other — so an on-carrier edge under a verified declaration never
/// crosses the partner surface. A new arm may NOT land here without
/// restating its own residual-sign story.
///
/// **The coaxial cylinder×sphere pair satisfies this contract and still
/// has no arm.** At the only coaxial tangency, `R = r`, the sphere lies
/// wholly in the cylinder's non-positive residual half-space and the
/// cylinder wholly in the sphere's non-negative one — opposite
/// orientations per direction, which the contract allows (the
/// internally tangent parallel cylinder pair the `|r1 − r2|` fallback
/// admits has the same structure); both halves are pinned by
/// `crates/topo/tests/verbs_cylsph_tangent_residuals.rs`. Its circle
/// locus serves the coaxial KISS alone (material wedge 0 or 2π), and two
/// things keep it out. No consumer builds the kiss edge from a locus of
/// any shape, so a circle arm would complete no boolean. And answering
/// that pair here would take it past `Unsupported`, which is where
/// `topo` routes a declared pair meeting along one circle by material
/// wedge — the routing that tells a smooth seam (wedge π, never a
/// `Tangent` contact) from a kiss — so the arm lands only behind that
/// routing, run on the locus.
///
/// # Errors
///
/// [`TangentLocusError`] — escalation, definite non-tangency, or a
/// configuration outside the closed-form lane.
pub fn tangent_locus<T: Decide>(
    a: &geom::Surface<T>,
    b: &geom::Surface<T>,
    reach: ExtentBall<T>,
    band: Band,
) -> Result<TangentLocus<T>, TangentLocusError> {
    let escalate = TangentLocusError::Escalated;
    match (a, b) {
        (
            &Surface::Plane {
                origin: q,
                normal: n,
                ..
            },
            &Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            },
        )
        | (
            &Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            },
            &Surface::Plane {
                origin: q,
                normal: n,
                ..
            },
        ) => {
            // The cylinder's origin moved along its axis to the foot of
            // the extent's centre: the gap is read there, where the
            // tilt's lever is least.
            let foot = reach.foot_on(origin, axis);
            let pc = PlaneCylinder {
                q,
                n,
                o: foot,
                a: axis,
                r: radius,
            };
            let gap = "pc_parallel_gap";
            match plane_cylinder_ruled(&pc, reach.lever_from(foot), band).map_err(escalate)? {
                Some(RuledSection::TangentLine { origin, dir }) => {
                    Ok(TangentLocus::Line { origin, dir })
                }
                Some(RuledSection::ParallelLines { .. }) => Err(TangentLocusError::NotTangent {
                    apart: false,
                    predicate: gap,
                }),
                Some(RuledSection::Empty) => Err(TangentLocusError::NotTangent {
                    apart: true,
                    predicate: gap,
                }),
                None => Err(TangentLocusError::Unsupported {
                    what: "plane×cylinder tangency is closed-form only along a ruling — \
                           the axis must lie in the plane's direction space",
                }),
            }
        }
        (
            Surface::Cylinder {
                origin: o1,
                axis: a1,
                radius: r1,
                ..
            },
            Surface::Cylinder {
                origin: o2,
                axis: a2,
                radius: r2,
                ..
            },
        ) => {
            let o2 = &reach.foot_on(*o2, *a2);
            match cylinder_axes_parallel(a1.cross(*a2).norm(), reach.lever_from(*o2), band)
                .map_err(escalate)?
            {
                Sign::Zero => {}
                Sign::Positive | Sign::Negative => {
                    return Err(TangentLocusError::Unsupported {
                        what: "cylinder×cylinder tangency is closed-form only for PARALLEL \
                               axes (the generator line); skew/crossing axes are outside \
                               the lane",
                    });
                }
            }
            // Perpendicular axis-to-axis offset (the axis LINE datum,
            // the carrier ladder's own construction).
            let delta = *o2 - *o1;
            let w = delta - *a1 * delta.dot(*a1);
            let dist = w.norm();
            // External tangency first (|w| = r1 + r2): the common case
            // and the flush detector's; internal (|w| = |r1 − r2|)
            // second. Fixed probe order (D9).
            match parallel_cylinder_gap(*r1, *r2, dist, band).map_err(escalate)? {
                Sign::Zero => {
                    let w_hat = w.normalize();
                    return Ok(TangentLocus::Line {
                        origin: *o1 + w_hat * *r1,
                        dir: *a1,
                    });
                }
                Sign::Negative => {
                    return Err(TangentLocusError::NotTangent {
                        apart: true,
                        predicate: "cc_parallel_gap",
                    });
                }
                Sign::Positive => {}
            }
            let internal = "tangent_locus_internal_gap";
            match decide(internal, Margin::of((*r1 - *r2).abs() - dist), band).map_err(escalate)? {
                Sign::Zero => {
                    // Internal tangency: the smaller cylinder rests
                    // inside the larger; the generator sits on the
                    // offset direction at the LARGER radius from the
                    // larger axis. With coaxial axes (dist in the
                    // zero band AND radii in the zero band) the locus
                    // direction is ill-posed — refuse typed.
                    match decide("tangent_locus_side", Margin::of(dist), band).map_err(escalate)? {
                        Sign::Positive => {}
                        _ => {
                            return Err(TangentLocusError::Unsupported {
                                what: "coaxial equal-radius cylinders have no isolated \
                                       tangent generator (conformal contact is Rest, \
                                       not Tangent)",
                            });
                        }
                    }
                    // Which cylinder contains which decides the
                    // generator's side (derived: with ŵ = o1→o2 and
                    // |w| = |r1 − r2|, the touch point is
                    // P = o1 + ŵ·r1 when r1 > r2 — c2 inside c1 —
                    // and P = o1 − ŵ·r1 when r1 < r2, both from
                    // collinearity of P, o1, o2 with |P−o1| = r1,
                    // |P−o2| = r2). The side is DECIDED, never an
                    // evaluation-lane comparison.
                    let w_hat = w.normalize();
                    let sign = match decide("tangent_locus_side", Margin::of(*r1 - *r2), band)
                        .map_err(escalate)?
                    {
                        Sign::Positive => T::one(),
                        Sign::Negative => T::zero() - T::one(),
                        Sign::Zero => {
                            return Err(TangentLocusError::Unsupported {
                                what: "equal-radius internal tangency contradicts the \
                                       definite axis offset — no closed-form generator",
                            });
                        }
                    };
                    Ok(TangentLocus::Line {
                        origin: *o1 + w_hat * (sign * *r1),
                        dir: *a1,
                    })
                }
                // dist < |r1 − r2|: one cylinder nested strictly inside
                // the other, their minimum distance |r1 − r2| − dist
                // definitely positive: apart.
                Sign::Positive => Err(TangentLocusError::NotTangent {
                    apart: true,
                    predicate: internal,
                }),
                // |r1 − r2| < dist < r1 + r2: the surfaces cross.
                Sign::Negative => Err(TangentLocusError::NotTangent {
                    apart: false,
                    predicate: internal,
                }),
            }
        }
        _ => Err(TangentLocusError::Unsupported {
            what: "the closed-form tangent-locus lane holds plane×cylinder and parallel \
                   cylinder pairs only (the DEV-1 certified set)",
        }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;

    /// **The axis row is levered over the consumed extent.** A unit
    /// cylinder resting on `z = 0` at the origin, its axis rising
    /// `0.3·ε` per metre along `x`: over a 1 m patch about the origin
    /// (a 2 m lever from the axis) the tilt reads zero and the ruling
    /// is minted; over a 10 m patch along `x` the lever is 6 m, the tilt
    /// reads in band, and the row escalates rather than mint a ruling
    /// the far end stands `3·ε` off.
    #[test]
    fn the_axis_row_reads_the_tilt_across_the_extent() {
        let band = Band::linear(Tol::witness()).unwrap();
        let theta: f64 = 0.3 * band.zero();
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let cyl = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0),
            axis: Vec3::new(theta.cos(), 0.0, theta.sin()),
            radius: 1.0,
            u_ref: Vec3::unit_z(),
        };
        let metre = ExtentBall::new(Point3::origin(), 1.0);
        match tangent_locus(&plane, &cyl, metre, band) {
            Ok(TangentLocus::Line { .. }) => {}
            other => panic!("over a metre the tilt reads zero and the ruling is minted: {other:?}"),
        }
        let ten = ExtentBall::new(Point3::new(5.0, 0.0, 0.0), 5.0);
        match tangent_locus(&plane, &cyl, ten, band) {
            Err(TangentLocusError::Escalated(d)) => assert_eq!(
                d.predicate,
                Some("pc_axis_plane_parallel"),
                "the axis row escalates"
            ),
            other => panic!("over 10 m the tilt reads in band and escalates: {other:?}"),
        }
    }

    fn x_cylinder(y: f64, z: f64, radius: f64) -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::new(0.0, y, z),
            axis: Vec3::unit_x(),
            radius,
            u_ref: Vec3::unit_z(),
        }
    }

    /// **A plane through a cylinder's axis, the axis in band of it,
    /// crosses.** A unit cylinder along `x` whose axis stands
    /// `√(zero·escalate)` above `z = 0`: the plane cuts it along two
    /// rulings a metre from the axis, whichever side the axis lies on,
    /// so the section's `pc_parallel_gap` decides the crossing. The
    /// side of the axis is not a question the tangency asks.
    #[test]
    fn an_axis_in_band_of_the_plane_crosses_rather_than_escalates() {
        let band = Band::linear(Tol::witness()).unwrap();
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let cyl = x_cylinder(0.0, (band.zero() * band.escalate()).sqrt(), 1.0);
        let metre = ExtentBall::new(Point3::origin(), 1.0);
        match tangent_locus(&plane, &cyl, metre, band) {
            Err(TangentLocusError::NotTangent {
                apart: false,
                predicate: "pc_parallel_gap",
            }) => {}
            other => panic!("the plane definitely cuts the cylinder: {other:?}"),
        }
    }

    /// **On exactly parallel axes the external gap reads both radii, in
    /// either order.** Radii `1` and `1 − 0.9·zero` (equal within the
    /// band, so the declaration verifies), axes `r1 + r2 − 0.6·zero`
    /// apart: the walls overlap by `0.6·zero`, inside the zero band. The
    /// gap is `r1 + r2 − d` whichever cylinder comes first; one radius
    /// doubled would put `1.5·zero` on one order (in band, escalating)
    /// and `−0.3·zero` on the other (tangent). The axes are parallel
    /// exactly, so no tilt pivot enters the reading.
    #[test]
    fn on_exact_parallels_the_external_gap_reads_both_radii_in_either_order() {
        use crate::intersect::{EqualCylinderSection, RadiusEvidence, cylinder_cylinder_section};
        let band = Band::linear(Tol::witness()).unwrap();
        let zero = band.zero();
        let (r1, r2) = (1.0, 1.0 - 0.9 * zero);
        let c1 = x_cylinder(0.0, 0.0, r1);
        let c2 = x_cylinder(r1 + r2 - 0.6 * zero, 0.0, r2);
        let metre = ExtentBall::new(Point3::origin(), 1.0);
        for (label, a, b) in [("c1, c2", &c1, &c2), ("c2, c1", &c2, &c1)] {
            match tangent_locus(a, b, metre, band) {
                Ok(TangentLocus::Line { .. }) => {}
                other => panic!("{label}: the witness mints the ruling: {other:?}"),
            }
            match cylinder_cylinder_section(a, b, RadiusEvidence::Declared, 1.0, band) {
                Ok(EqualCylinderSection::TangentLine(_)) => {}
                other => panic!("{label}: the section classifies the same tangency: {other:?}"),
            }
        }
    }

    /// A unit cylinder whose axis stands at height 1 above `z = 0` at
    /// `x = 0`, rising `0.3·zero` per metre along `x`, its stored origin
    /// `back` metres behind that point along the axis.
    fn tilted_resting_cylinder(back: f64, zero: f64) -> Surface<f64> {
        let theta = 0.3 * zero;
        let axis = Vec3::new(theta.cos(), 0.0, theta.sin());
        Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0) - axis * back,
            axis,
            radius: 1.0,
            u_ref: Vec3::unit_y(),
        }
    }

    /// **The gap is read where the extent is, not at the stored
    /// origin.** The resting, barely tilted cylinder of
    /// [`tilted_resting_cylinder`] stored 1000 m back along its axis:
    /// over a metre about the origin the tilt reads zero and the axis
    /// stands one radius off the plane, so the ruling mints. Read at the
    /// stored origin the axis would stand `300·zero` lower, a definite
    /// crossing.
    #[test]
    fn a_far_stored_origin_does_not_move_the_gap() {
        let band = Band::linear(Tol::witness()).unwrap();
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let cyl = tilted_resting_cylinder(1000.0, band.zero());
        let metre = ExtentBall::new(Point3::origin(), 1.0);
        match tangent_locus(&plane, &cyl, metre, band) {
            Ok(TangentLocus::Line { origin, .. }) => assert!(
                origin.x.abs() < 1.0 && origin.z.abs() < band.zero(),
                "the ruling is read beside the extent, on the plane: {origin:?}"
            ),
            other => panic!("the resting cylinder is tangent across the extent: {other:?}"),
        }
    }

    /// **The tilt is levered from the foot to the far side of the
    /// extent, not over its radius alone.** The same cylinder stored at
    /// its foot, with a 1 m extent centred 5 m off its axis along `y`:
    /// the extent reaches `√26 + 1 ≈ 6.1` m from the foot, so the
    /// `0.3·zero` tilt reads `1.83·zero`, in band. Over the extent's
    /// radius alone it would read `0.3·zero` and mint a ruling that
    /// stands nearly twice the zero band off across the faces.
    #[test]
    fn the_tilt_is_levered_from_the_foot_across_the_extent() {
        let band = Band::linear(Tol::witness()).unwrap();
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let cyl = tilted_resting_cylinder(0.0, band.zero());
        let off_axis = ExtentBall::new(Point3::new(0.0, 5.0, 0.0), 1.0);
        match tangent_locus(&plane, &cyl, off_axis, band) {
            Err(TangentLocusError::Escalated(d)) => assert_eq!(
                d.predicate,
                Some("pc_axis_plane_parallel"),
                "the axis row escalates"
            ),
            other => panic!("the tilt levered across the extent reads in band: {other:?}"),
        }
    }
}
