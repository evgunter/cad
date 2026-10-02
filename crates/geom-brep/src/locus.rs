//! **The closed-form tangent locus** — the DEV-1 witness lane: the
//! contact line of a tangent carrier pair, for exactly the
//! configurations whose locus is closed-form. Every numeric decision is
//! a named row through `dihedral::decide`.

use geom::Surface;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use crate::dihedral::decide;

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
///   the **1 m verification arm**, a `T::one()` literal that `topo`'s
///   carrier-pair doors (`rest::flush_pair_relation`,
///   `rest::carrier_pair_verdict`) spell too and must agree with:
///   tangency along an unbounded ruling is a carrier-level claim,
///   metered at the same arm the carrier ladder meters its
///   parallelism rungs.
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
/// **The coaxial cylinder×sphere circle arm's story is MEASURED and it
/// PASSES — and that is not what still blocks the arm** (issue #974;
/// the blocker's stated cause is superseded here rather than left
/// standing). At the only coaxial tangency, `R = r`, the sphere lies
/// wholly in the cylinder's non-positive residual half-space and the
/// cylinder wholly in the sphere's non-negative one. The orientations
/// are OPPOSITE per direction, which this contract never forbade: the
/// internally tangent parallel cylinder pair the `|r1 − r2|` fallback
/// already admits has exactly that structure. Both halves are pinned
/// by `crates/topo/tests/verbs_cylsph_tangent_residuals.rs`. What
/// blocks the arm is downstream of the story: [`TangentLocus`] carries
/// a LINE and nothing else, its consumers all read a locus DIRECTION
/// and none has a circle story, and the arm would need a
/// declared-coaxiality channel this lane cannot reach. #974 stays open
/// for that work.
///
/// # Errors
///
/// [`TangentLocusError`] — escalation, definite non-tangency, or a
/// configuration outside the closed-form lane.
pub fn tangent_locus<T: Decide>(
    a: &geom::Surface<T>,
    b: &geom::Surface<T>,
    band: Band,
) -> Result<TangentLocus<T>, TangentLocusError> {
    let arm = T::one();
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
            match decide(
                "tangent_locus_axis_parallel",
                Margin::levered(axis.dot(*normal).abs(), arm),
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
            match decide(
                "tangent_locus_axis_parallel",
                Margin::levered(a1.cross(*a2).norm(), arm),
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
