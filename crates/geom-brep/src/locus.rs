//! **The closed-form tangent locus** — the DEV-1 witness lane: the
//! contact line of a tangent carrier pair, for exactly the
//! configurations whose locus is closed-form. Every numeric decision is
//! a named row through `dihedral::decide`.

use geom::Surface;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use crate::dihedral::decide;
use crate::extent::ExtentBall;

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
/// decision is a named three-outcome row:
///
/// - `tangent_locus_axis_parallel` — the axis/plane (or axis/axis)
///   angular deviation `|d × n̂|` (a sine of unit vectors) levered by
///   `reach`'s extent from the point the gap row is read at — the foot
///   of the extent's centre on the cylinder's axis (the second
///   cylinder's, for a pair). A tilt moves the ruling by that angle
///   times the distance from there, so the row reads the displacement
///   the tilt induces across the faces the locus is consumed on; with
///   the gap row it mints only where each reads zero, so the ruling
///   stands within two zero bands of the carriers across the faces,
///   and the `Tangent` table then verifies it sample by sample. `reach` is the declared pair's consumed
///   extent, the one `topo`'s carrier-pair doors lever their ladder at.
/// - `tangent_locus_gap` — the metre gap at the tangency: for
///   plane×cylinder the axis-to-plane distance minus the radius; for
///   parallel cylinders the axis-to-axis distance minus `r1 + r2`
///   (external) falling back to `|r1 − r2|` (internal). Zero ⇒
///   tangent (the locus mints); Positive ⇒ definitely apart;
///   Negative ⇒ definitely crossing.
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
            Surface::Plane { origin, normal, .. },
            Surface::Cylinder {
                origin: co,
                axis,
                radius,
                ..
            },
        )
        | (
            Surface::Cylinder {
                origin: co,
                axis,
                radius,
                ..
            },
            Surface::Plane { origin, normal, .. },
        ) => {
            // Ruling tangency needs the axis IN the plane's direction
            // space: |axis · n̂| is the sine of the axis' elevation.
            let co = &reach.foot_on(*co, *axis);
            match decide(
                "tangent_locus_axis_parallel",
                Margin::levered(axis.dot(*normal).abs(), reach.lever_from(*co)),
                band,
            )
            .map_err(escalate)?
            {
                Sign::Zero => {}
                _ => {
                    return Err(TangentLocusError::Unsupported {
                        what: "plane×cylinder tangency is closed-form only along a ruling — \
                               the axis must lie in the plane's direction space",
                    });
                }
            }
            // Signed axis-to-plane height; its SIGN picks the tangent
            // generator, its magnitude minus r is the tangency gap.
            let h = (*co - *origin).dot(*normal);
            let side = match decide("tangent_locus_side", Margin::of(h), band).map_err(escalate)? {
                Sign::Positive => T::one(),
                Sign::Negative => T::zero() - T::one(),
                Sign::Zero => {
                    // Axis ON the plane: the cylinder definitely
                    // crosses (both sides pierce).
                    return Err(TangentLocusError::NotTangent { apart: false });
                }
            };
            match decide("tangent_locus_gap", Margin::of(h.abs() - *radius), band)
                .map_err(escalate)?
            {
                Sign::Zero => Ok(TangentLocus::Line {
                    origin: *co - *normal * (side * *radius),
                    dir: *axis,
                }),
                Sign::Positive => Err(TangentLocusError::NotTangent { apart: true }),
                Sign::Negative => Err(TangentLocusError::NotTangent { apart: false }),
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
            match decide(
                "tangent_locus_axis_parallel",
                Margin::levered(a1.cross(*a2).norm(), reach.lever_from(*o2)),
                band,
            )
            .map_err(escalate)?
            {
                Sign::Zero => {}
                _ => {
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
            match decide("tangent_locus_gap", Margin::of(dist - (*r1 + *r2)), band)
                .map_err(escalate)?
            {
                Sign::Zero => {
                    let w_hat = w.normalize();
                    return Ok(TangentLocus::Line {
                        origin: *o1 + w_hat * *r1,
                        dir: *a1,
                    });
                }
                Sign::Positive => return Err(TangentLocusError::NotTangent { apart: true }),
                Sign::Negative => {}
            }
            match decide(
                "tangent_locus_gap",
                Margin::of((*r1 - *r2).abs() - dist),
                band,
            )
            .map_err(escalate)?
            {
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
                // dist < |r1 − r2|: one cylinder NESTED strictly
                // inside the other — the surfaces definitely do NOT
                // meet (their minimum distance is |r1 − r2| − dist,
                // definitely positive here): APART, not crossing
                // (union fix F3 — the pre-fix arm labeled this
                // definite clearance a crossing).
                Sign::Positive => Err(TangentLocusError::NotTangent { apart: true }),
                // |r1 − r2| < dist < r1 + r2 (the external row already
                // refused the ≥ side): the surfaces definitely cross.
                Sign::Negative => Err(TangentLocusError::NotTangent { apart: false }),
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
                Some("tangent_locus_axis_parallel"),
                "the axis row escalates"
            ),
            other => panic!("over 10 m the tilt reads in band and escalates: {other:?}"),
        }
    }
}
