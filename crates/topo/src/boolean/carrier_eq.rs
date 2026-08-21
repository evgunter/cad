//! **`carrier_eq`** — the oriented-carrier-equality ladder,
//! kind-generalized (CONTACT-DESIGN C4's `Rest` table).
//!
//! [`super::oriented_plane_eq`] answers "same plane?" through four
//! rungs: same recipe source, declared intent, definite geometric
//! difference, else a typed undeclared-coincidence refusal. `Rest`
//! generalizes S1's planar vocabulary to every carrier kind, so the
//! ladder generalizes with it — SAME four rungs, SAME three verdicts
//! ([`CarrierRelation`]), SAME typed refusals ([`CarrierEqError`]),
//! with only the rung-3 margin list varying by kind:
//!
//! | kind | defining data | margins, each at its named lever arm |
//! |------|---------------|--------------------------------------|
//! | plane | origin, outward normal | `bool_plane_parallel` (·arm), `bool_plane_offset` |
//! | sphere | centre, radius | `carrier_sphere_center`, `carrier_sphere_radius` |
//! | cylinder | axis line, radius | `carrier_cyl_axis_parallel` (·arm), `carrier_cyl_axis_offset`, `carrier_cyl_radius` |
//!
//! The lever arms are the honest ones: an ANGULAR margin (two
//! normalized directions crossed) is dimensionless, so it is metered
//! at the extent over which the verdict is consumed (`arm`, metres),
//! turning it into the displacement the misalignment induces there.
//! A LENGTH margin (a centre separation, a radius difference, a
//! point-to-axis distance) is already in metres and is metered at
//! unit arm — multiplying it by the extent would price the same
//! error twice.
//!
//! **The plane arm is byte-for-byte the old one**: the `(Plane,
//! Plane)` case delegates to [`super::oriented_plane_eq`] rather than
//! re-deriving it, so no planar verdict, margin, or blessed fixture
//! moves in the generalization.
//!
//! **Orientation across kinds.** The plane arm's Same± verdict is a
//! statement about MATERIAL SIDES, and so is every other arm's: a
//! sphere's or cylinder's description carries the `outward` bit
//! saying whether the face's outward normal agrees with the surface's
//! outward radial direction (S10's sense fold, one dimension curved).
//! `Rest` contact is precisely [`CarrierRelation::SameOpposite`] — a
//! peg's convex wall against a bore's concave wall is the curved
//! spelling of two boxes' opposed faces. Aligned coincidence is
//! CONTAINMENT, not contact (the C1 lemma); the carrier ladder
//! reports it honestly as `SameOriented` and the CONTACT doors
//! ([`super::contact_verify::contact_pair_verdict`]) are what refuse it.
//!
//! **Value-equality still never glues** (AQ6). Two independently
//! authored spheres with bit-equal radii reach rung 4 and refuse
//! `Undeclared`, exactly as two bit-equal planes do — the declaration
//! is what makes them one carrier, and nothing else is.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::contact::ContactVerdict;
use crate::validate::decide;

use super::plane_eq::{PlaneDesc, PlaneIdentity, oriented_plane_eq_verdict};

/// The relation between two oriented carriers: the three outcomes
/// every kind's ladder produces.
///
/// This is ONE type across kinds — `plane_eq` re-exports it as
/// `PlaneRelation`, the spelling its planar callers use. A parallel
/// per-kind verdict enum would let a caller handle "same carrier" for
/// planes and forget it for cylinders.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarrierRelation {
    /// Same carrier, same material side (the ⁺ case of Eq. 15.3 and
    /// its curved analogues) — flush walls, and the merge stage's
    /// pair.
    SameOriented,
    /// Same carrier, opposite material sides (the ⁻ case) — this, and
    /// only this, is `Rest` contact.
    SameOpposite,
    /// Definitely different carriers.
    Distinct,
}

/// Typed refusal of [`carrier_eq`]; re-exported by `plane_eq` as
/// `PlaneEqError`.
#[derive(Debug)]
pub enum CarrierEqError {
    /// A margin landed in the sliver band.
    Escalated(Indeterminate),
    /// Geometrically coincident-or-near without shared source or
    /// declared intent: an undeclared coincidence (F6). Carries the
    /// orientation the data rungs had ALREADY decided before the
    /// refusal (alignment is settled before the coincidence margins
    /// are taken), so a refusal can name the relation a declaration
    /// would assert — the refusal-menu payload (SELECT-DESIGN §3d,
    /// LIB-PYG5 R3) — without re-running any decide on the error
    /// path.
    Undeclared {
        /// The coincidence predicate's diagnostics (a decided-zero
        /// margin encodes as `MarginDiag::Invalid`; an in-band margin
        /// rides as measured).
        diag: Indeterminate,
        /// The decided orientation: [`CarrierRelation::SameOriented`]
        /// or [`CarrierRelation::SameOpposite`], never `Distinct`.
        relation: CarrierRelation,
    },
    /// A declared pair whose carriers are DEFINITELY distinct — the
    /// recipe's declaration contradicts the geometry; refused loudly,
    /// never glued.
    Contradicted(Indeterminate),
}

/// One carrier's conventional oriented description.
///
/// Every variant carries the MATERIAL side, not the chart's: the
/// plane arm folds the face sense into `normal` (S10), and the curved
/// arms carry it as `outward`. A description built from a raw chart
/// normal would make the Same± verdict a statement about nothing.
#[derive(Clone, Copy, Debug)]
pub enum CarrierDesc<T: geom_core::Real> {
    /// A plane, by a point on it and its unit outward normal.
    Plane {
        /// A point on the plane.
        origin: Point3<T>,
        /// The unit outward normal (of the face, not of the chart).
        normal: Vec3<T>,
    },
    /// A sphere, by centre and radius.
    Sphere {
        /// The centre.
        center: Point3<T>,
        /// The radius (positive).
        radius: T,
        /// Whether the face's outward normal points AWAY from the
        /// centre (a convex wall) rather than toward it (a cavity).
        outward: bool,
    },
    /// A cylinder, by a point on its axis, the unit axis direction,
    /// and the radius. The axis is a LINE: its direction sign carries
    /// no material information, so it never enters the Same± verdict.
    Cylinder {
        /// A point on the axis.
        origin: Point3<T>,
        /// The unit axis direction.
        axis: Vec3<T>,
        /// The radius (positive).
        radius: T,
        /// Whether the face's outward normal points AWAY from the
        /// axis (a shaft) rather than toward it (a bore).
        outward: bool,
    },
}

impl<T: geom_core::Real> CarrierDesc<T> {
    /// The kind's name, for messages and the kind-mismatch rung.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Plane { .. } => "plane",
            Self::Sphere { .. } => "sphere",
            Self::Cylinder { .. } => "cylinder",
        }
    }
}

/// **`carrier_eq`** — module docs for the ladder. `id` is the
/// comparison's identity evidence (recipe sources + declared intent);
/// `arm` is the lever arm in metres metering the ANGULAR margins (the
/// extent over which the verdict is consumed); `band` the run's
/// linear band.
///
/// # Errors
///
/// [`CarrierEqError`] — sliver escalation, undeclared coincidence, or
/// a contradicted declaration.
pub fn carrier_eq<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    id: PlaneIdentity<'_>,
    arm: T,
    band: Band,
) -> Result<CarrierRelation, CarrierEqError> {
    carrier_eq_verdict(c1, c2, id, arm, band).map(|(rel, _)| rel)
}

/// [`carrier_eq`] plus the TRILEAN: whether the verdict stands on the
/// geometry's own definite evidence or on the declaration bridging an
/// in-band residue. One implementation, two projections — see
/// [`super::plane_eq::oriented_plane_eq_verdict`], whose contract this
/// widens to every kind.
///
/// # Errors
///
/// [`CarrierEqError`] — as [`carrier_eq`].
pub fn carrier_eq_verdict<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    id: PlaneIdentity<'_>,
    arm: T,
    band: Band,
) -> Result<(CarrierRelation, ContactVerdict), CarrierEqError> {
    match (c1, c2) {
        (
            CarrierDesc::Plane {
                origin: o1,
                normal: n1,
            },
            CarrierDesc::Plane {
                origin: o2,
                normal: n2,
            },
        ) => oriented_plane_eq_verdict(
            &PlaneDesc {
                origin: *o1,
                normal: *n1,
            },
            &PlaneDesc {
                origin: *o2,
                normal: *n2,
            },
            id,
            arm,
            band,
        ),
        (
            CarrierDesc::Sphere {
                center: p1,
                radius: r1,
                outward: w1,
            },
            CarrierDesc::Sphere {
                center: p2,
                radius: r2,
                outward: w2,
            },
        ) => {
            if let Some(v) = source_rung(id, *w1 != *w2) {
                return Ok((v, ContactVerdict::Definite));
            }
            let margins = [
                ("carrier_sphere_center", Margin::norm3(*p1 - *p2)),
                ("carrier_sphere_radius", Margin::of(*r1 - *r2)),
            ];
            data_rungs(&margins, id.declared, *w1 == *w2, band)
        }
        (
            CarrierDesc::Cylinder {
                origin: p1,
                axis: a1,
                radius: r1,
                outward: w1,
            },
            CarrierDesc::Cylinder {
                origin: p2,
                axis: a2,
                radius: r2,
                outward: w2,
            },
        ) => {
            if let Some(v) = source_rung(id, *w1 != *w2) {
                return Ok((v, ContactVerdict::Definite));
            }
            // The axis LINE, not the axis ray: parallelism is metered
            // on the cross product (sign-free by construction), and
            // the offset is the perpendicular distance from c2's axis
            // point to c1's axis — the two data a cylinder's axis
            // actually has.
            let delta = *p2 - *p1;
            let perp = delta - *a1 * delta.dot(*a1);
            let margins = [
                (
                    "carrier_cyl_axis_parallel",
                    Margin::levered(a1.cross(*a2).norm(), arm),
                ),
                ("carrier_cyl_axis_offset", Margin::norm3(perp)),
                ("carrier_cyl_radius", Margin::of(*r1 - *r2)),
            ];
            data_rungs(&margins, id.declared, *w1 == *w2, band)
        }
        // Different kinds: definitely different carriers. A plane is
        // not a cylinder at any radius, so this needs no numerics —
        // and a declaration asserting otherwise is contradicted by
        // the same structural fact.
        _ => {
            if id.declared {
                Err(CarrierEqError::Contradicted(Indeterminate {
                    margin: geom_core::MarginDiag::Invalid,
                    band,
                    predicate: Some("carrier_kind"),
                }))
            } else {
                Ok((CarrierRelation::Distinct, ContactVerdict::Definite))
            }
        }
    }
}

/// Rung 1 for the curved arms: both descriptions carry the same
/// recipe source ⇒ same carrier by the N6 theorem, with the material
/// side read off the descriptions' own `outward` bits.
///
/// The plane arm's version additionally debug-asserts that the bits
/// agree; the curved arms have no canonicalized bit form to assert
/// against, and inventing one would be a second source of truth.
fn source_rung(id: PlaneIdentity<'_>, opposed: bool) -> Option<CarrierRelation> {
    let (s1, s2) = (id.s1?, id.s2?);
    s1.same_base(s2).then_some(if opposed {
        CarrierRelation::SameOpposite
    } else {
        CarrierRelation::SameOriented
    })
}

/// Rungs 2–4 for the curved arms, driven by the kind's margin list.
///
/// One traversal serves both directions of C4's ratified semantics:
/// *undeclared*, a definitely-nonzero margin means `Distinct` and an
/// all-zero list means the typed `Undeclared` refusal (value equality
/// never glues); *declared*, a definitely-nonzero margin CONTRADICTS
/// and anything else — zero or in-band — stands, which is exactly the
/// bridged residue and nothing more. Escalations refuse typed when
/// undeclared, and are the residue the declaration bridges when
/// declared.
fn data_rungs<T: Decide>(
    margins: &[(&'static str, Margin<T>)],
    declared: bool,
    aligned: bool,
    band: Band,
) -> Result<(CarrierRelation, ContactVerdict), CarrierEqError> {
    let same = if aligned {
        CarrierRelation::SameOriented
    } else {
        CarrierRelation::SameOpposite
    };
    let mut any_in_band: Option<Indeterminate> = None;
    for &(name, margin) in margins {
        match decide(name, margin, band) {
            Ok(Sign::Positive | Sign::Negative) => {
                let diag = Indeterminate {
                    margin: geom_core::MarginDiag::Invalid,
                    band,
                    predicate: Some(name),
                };
                return if declared {
                    Err(CarrierEqError::Contradicted(diag))
                } else {
                    Ok((CarrierRelation::Distinct, ContactVerdict::Definite))
                };
            }
            Ok(Sign::Zero) => {}
            Err(diag) => any_in_band = any_in_band.or(Some(diag)),
        }
    }
    if declared {
        return Ok((
            same,
            if any_in_band.is_some() {
                ContactVerdict::Bridged
            } else {
                ContactVerdict::Definite
            },
        ));
    }
    // Rung 4: coincident-or-near with no identity rung — near
    // coincidence NEVER silently becomes contact, and bit-equal data
    // without a shared source stays unglued.
    //
    // The predicate named is the first IN-BAND margin when there is
    // one (that is the margin the reader wants). When every datum
    // decided definitely zero there is no such margin, and the
    // fallback names the kind's FIRST datum rather than inventing a
    // predicate name no `decide` call ever used — an invented name
    // would read as a measurement that never happened.
    Err(CarrierEqError::Undeclared {
        diag: any_in_band.unwrap_or(Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some(margins[0].0),
        }),
        // The alignment this traversal was run under: the relation a
        // declaration of this pair would verify with (R3).
        relation: same,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use super::*;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn sphere(c: [f64; 3], r: f64, outward: bool) -> CarrierDesc<f64> {
        CarrierDesc::Sphere {
            center: Point3::new(c[0], c[1], c[2]),
            radius: r,
            outward,
        }
    }

    fn cyl(o: [f64; 3], a: [f64; 3], r: f64, outward: bool) -> CarrierDesc<f64> {
        CarrierDesc::Cylinder {
            origin: Point3::new(o[0], o[1], o[2]),
            axis: Vec3::new(a[0], a[1], a[2]),
            radius: r,
            outward,
        }
    }

    fn declared() -> PlaneIdentity<'static> {
        PlaneIdentity {
            s1: None,
            s2: None,
            declared: true,
        }
    }

    /// The peg-in-bore row: value-equal radii, opposed material
    /// sides. UNDECLARED it refuses (value equality never glues);
    /// DECLARED it is the `Rest` verdict.
    #[test]
    fn sphere_value_equal_needs_the_declaration() {
        let a = sphere([0.0, 0.0, 0.0], 2.0, true);
        let b = sphere([0.0, 0.0, 0.0], 2.0, false);
        assert!(matches!(
            carrier_eq(&a, &b, PlaneIdentity::NONE, 1.0, band()),
            Err(CarrierEqError::Undeclared { .. })
        ));
        assert_eq!(
            carrier_eq(&a, &b, declared(), 1.0, band()).unwrap(),
            CarrierRelation::SameOpposite
        );
    }

    /// Aligned coincidence is reported honestly as `SameOriented` —
    /// the carrier ladder's job is the carrier; refusing containment
    /// is the contact door's job (module docs, C1 lemma).
    #[test]
    fn sphere_aligned_is_same_oriented_not_a_carrier_error() {
        let a = sphere([1.0, 0.0, 0.0], 2.0, true);
        let b = sphere([1.0, 0.0, 0.0], 2.0, true);
        assert_eq!(
            carrier_eq(&a, &b, declared(), 1.0, band()).unwrap(),
            CarrierRelation::SameOriented
        );
    }

    /// Every definite verdict wins over every declaration: a
    /// definitely different radius contradicts, naming the margin
    /// that decided.
    #[test]
    fn definite_radius_difference_contradicts_the_declaration() {
        let a = sphere([0.0, 0.0, 0.0], 2.0, true);
        let b = sphere([0.0, 0.0, 0.0], 2.5, false);
        let err = carrier_eq(&a, &b, declared(), 1.0, band()).unwrap_err();
        match err {
            CarrierEqError::Contradicted(d) => {
                assert_eq!(d.predicate, Some("carrier_sphere_radius"));
            }
            other => panic!("expected Contradicted, got {other:?}"),
        }
        // Undeclared, the same pair is simply two different carriers.
        assert_eq!(
            carrier_eq(&a, &b, PlaneIdentity::NONE, 1.0, band()).unwrap(),
            CarrierRelation::Distinct
        );
    }

    /// ε-row, three outcomes at one geometry: a sub-band radius
    /// difference. UNDECLARED it refuses typed (in-band is never a
    /// silent pass); DECLARED it is the bridged residue and stands;
    /// a definite difference at the same site contradicts.
    #[test]
    fn sphere_radius_epsilon_row_three_outcomes() {
        let a = sphere([0.0, 0.0, 0.0], 2.0, true);
        let in_band = sphere([0.0, 0.0, 0.0], 2.0 + 1e-12, false);
        assert!(
            matches!(
                carrier_eq(&a, &in_band, PlaneIdentity::NONE, 1.0, band()),
                Err(CarrierEqError::Undeclared { .. })
            ),
            "in-band, undeclared: refuses"
        );
        assert_eq!(
            carrier_eq(&a, &in_band, declared(), 1.0, band()).unwrap(),
            CarrierRelation::SameOpposite,
            "in-band, declared: the bridged residue"
        );
        let definite = sphere([0.0, 0.0, 0.0], 2.001, false);
        assert!(
            matches!(
                carrier_eq(&a, &definite, declared(), 1.0, band()),
                Err(CarrierEqError::Contradicted(_))
            ),
            "definite, declared: contradicted"
        );
    }

    /// A cylinder's axis is a LINE: reversing the stored direction is
    /// the same carrier, and the material side comes from `outward`
    /// alone.
    #[test]
    fn cylinder_axis_direction_sign_is_not_material() {
        let a = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        let b = cyl([0.0, 0.0, 5.0], [0.0, 0.0, -1.0], 3.0, false);
        assert_eq!(
            carrier_eq(&a, &b, declared(), 1.0, band()).unwrap(),
            CarrierRelation::SameOpposite
        );
    }

    /// A parallel-but-offset axis is a definitely different cylinder;
    /// declaring it contradicts at the axis-offset margin.
    #[test]
    fn cylinder_offset_axis_is_distinct() {
        let a = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        let b = cyl([0.5, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, false);
        assert_eq!(
            carrier_eq(&a, &b, PlaneIdentity::NONE, 1.0, band()).unwrap(),
            CarrierRelation::Distinct
        );
        match carrier_eq(&a, &b, declared(), 1.0, band()).unwrap_err() {
            CarrierEqError::Contradicted(d) => {
                assert_eq!(d.predicate, Some("carrier_cyl_axis_offset"));
            }
            other => panic!("expected Contradicted, got {other:?}"),
        }
    }

    /// ε-row on the cylinder's own margins, three outcomes at one
    /// geometry — the row the sphere already had, owed to every new
    /// margin. The radius datum carries it: sub-band, the declaration
    /// bridges and the undeclared pair refuses; definite, it
    /// contradicts.
    #[test]
    fn cylinder_radius_epsilon_row_three_outcomes() {
        // Derived from the RUN's band, never a literal: the same
        // number is a gap at one ε and nothing at another, and a row
        // that only means what it says at one ε is not an ε row.
        let b = band();
        let a = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        let in_band = cyl(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            3.0 + (b.zero() + b.escalate()) * 0.5,
            false,
        );
        assert!(
            matches!(
                carrier_eq(&a, &in_band, PlaneIdentity::NONE, 1.0, band()),
                Err(CarrierEqError::Undeclared { .. })
            ),
            "in-band, undeclared: refuses"
        );
        assert_eq!(
            carrier_eq(&a, &in_band, declared(), 1.0, band()).unwrap(),
            CarrierRelation::SameOpposite,
            "in-band, declared: the bridged residue"
        );
        let definite = cyl(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            3.0 + b.escalate() * 1000.0,
            false,
        );
        match carrier_eq(&a, &definite, declared(), 1.0, band()).unwrap_err() {
            CarrierEqError::Contradicted(d) => {
                assert_eq!(d.predicate, Some("carrier_cyl_radius"));
            }
            other => panic!("expected Contradicted, got {other:?}"),
        }
    }

    /// The cylinder's ANGULAR margin is metered at its named lever
    /// arm, which is the arm the caller passes: a tilt that is
    /// indecisive over a 1 m consumption extent is definite over a
    /// 1000 km one. Same geometry, two arms, two honest answers.
    #[test]
    fn cylinder_axis_tilt_is_decided_at_the_lever_arm() {
        let b = band();
        let a = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        // A tilt whose displacement is sub-band over 1 m and decisive
        // over 1000 km — stated in band units so the row survives
        // every ε the matrix runs.
        let tilt = b.zero() * 0.1;
        let tilted = cyl([0.0, 0.0, 0.0], [tilt, 0.0, 1.0], 3.0, false);
        assert_eq!(
            carrier_eq(&a, &tilted, declared(), 1.0, band()).unwrap(),
            CarrierRelation::SameOpposite,
            "at a 1 m arm the tilt is below the band: the declaration stands"
        );
        match carrier_eq(&a, &tilted, declared(), 1e6, band()).unwrap_err() {
            CarrierEqError::Contradicted(d) => {
                assert_eq!(d.predicate, Some("carrier_cyl_axis_parallel"));
            }
            other => panic!("expected Contradicted at the long arm, got {other:?}"),
        }
    }

    /// Kinds do not compare: a plane is not a cylinder at any radius,
    /// and a declaration saying so is contradicted structurally.
    #[test]
    fn kind_mismatch_is_distinct_and_contradicts_when_declared() {
        let p = CarrierDesc::Plane {
            origin: Point3::origin(),
            normal: Vec3::new(0.0, 0.0, 1.0),
        };
        let c = cyl([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, true);
        assert_eq!(
            carrier_eq(&p, &c, PlaneIdentity::NONE, 1.0, band()).unwrap(),
            CarrierRelation::Distinct
        );
        assert!(matches!(
            carrier_eq(&p, &c, declared(), 1.0, band()),
            Err(CarrierEqError::Contradicted(_))
        ));
    }

    /// The plane arm is the OLD arm: `carrier_eq` on two plane
    /// descriptions agrees with `oriented_plane_eq` called directly,
    /// verdict for verdict — the generalization moved no planar
    /// number.
    #[test]
    fn plane_arm_delegates_unchanged() {
        let p1 = PlaneDesc {
            origin: Point3::origin(),
            normal: Vec3::new(0.0, 0.0, 1.0),
        };
        let p2 = PlaneDesc {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, -1.0),
        };
        let via_carrier = carrier_eq(
            &CarrierDesc::Plane {
                origin: p1.origin,
                normal: p1.normal,
            },
            &CarrierDesc::Plane {
                origin: p2.origin,
                normal: p2.normal,
            },
            declared(),
            1.0,
            band(),
        )
        .unwrap();
        let direct =
            crate::boolean::plane_eq::oriented_plane_eq(&p1, &p2, declared(), 1.0, band()).unwrap();
        assert_eq!(via_carrier, direct);
        assert_eq!(direct, CarrierRelation::SameOpposite);
    }
}
