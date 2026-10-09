//! **Oriented-plane-equality** — the typed replacement for ch. 15's
//! `vecequal` on raw plane 4-vectors (Program 15.10's ⁺/⁻ gate, notes'
//! predicate inventory: "fragile on unnormalized Newell vectors").
//!
//! Verdicts: *same plane, same orientation* / *same plane, opposite
//! orientation* / *different plane* — plus the typed refusals. The
//! rungs, in order:
//!
//! 1. **Declared-pair rung (recipe intent, F5)**: the consuming op's
//!    recipe data declares THIS face pair coincident
//!    ([`PlaneIdentity::declared`]). The declaration is verified, not
//!    trusted, as one displacement over the consumed extent
//!    (`carrier_eq::declared_reading`): a point of a face definitely
//!    off the other plane refuses ([`PlaneEqError::Contradicted`]) — a
//!    wrong declaration must never force wrong geometry; a displacement
//!    in band at every point of the extent is bridged, with the decided
//!    orientation sign picking Same±; anything between is
//!    [`PlaneEqError::Unsettled`].
//! 2. **Geometric trilean (definite-different)**: parallelism margin
//!    `‖n₁ × n₂‖·arm` (`bool_plane_parallel`, the arm the extent's
//!    radius, the offsets read at its centre) definitely positive ⇒
//!    [`PlaneRelation::Distinct`]; else offset margin `(d₁ − σ·d₂)`
//!    (`bool_plane_offset`, σ the orientation sign) definitely nonzero
//!    ⇒ parallel-but-offset ⇒ `Distinct`.
//! 3. **Zero glues** (D10, Booleans): an offset margin decided Zero is
//!    one plane, `SameOriented` or `SameOpposite` by the decided
//!    orientation, and the decided margin rides out with the verdict
//!    for the caller to record ([`crate::coincidence`]). An offset in
//!    band is a sliver and refuses ([`PlaneEqError::Undecided`]):
//!    near-coincidence never silently becomes contact (F6).
//!
//! In-band parallelism and orientation margins escalate typed
//! ([`PlaneEqError::Escalated`]), naming the rung that could not decide
//! ([`PlaneRung`]). Whether two planes are one construction is not the
//! kernel's question: the document's coincidence door decides it
//! (D10, Coincidence).


use super::carrier_eq::{CarrierDesc, CoincidenceMeasure, ConsumedExtent};
use geom_core::{Band, Decide, Decided, Indeterminate, Margin, MarginDiag, Point3, Sign, Vec3};

use crate::contact::ContactVerdict;
use crate::validate::{decide, decide_reported};

/// The relation between two oriented planes — the PLANE SPELLING of
/// the one carrier verdict.
///
/// One type, not two: `Rest` generalizes to every carrier kind
/// ([`mod@super::carrier_eq`]), so a caller that handles "same carrier"
/// for planes handles it for spheres and cylinders by construction
/// rather than by remembering to.
pub use super::carrier_eq::CarrierRelation as PlaneRelation;

/// Typed refusal of [`oriented_plane_eq`] — the plane spelling of the
/// one carrier refusal (see [`PlaneRelation`] for why it is one type).
/// The `Undecided` arm carries the RELATION the ladder decided before
/// refusing — see the enum's own docs.
pub use super::carrier_eq::CarrierEqError as PlaneEqError;

/// Which rung of the plane ladder could not decide
/// ([`PlaneEqError::Escalated`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum PlaneRung {
    /// Whether the two planes are parallel (`bool_plane_parallel`). On
    /// an undeclared pair an in-band margin is a sliver a declaration
    /// would bridge; on a declared pair the rung bridges it.
    Parallel,
    /// Whether the two planes face the same way or opposite ways
    /// (`bool_plane_orient`). Its margin is the normals' cosine levered
    /// at the caller's arm, asked only once parallelism has read within
    /// the zero band at that arm, so `|cos| ≈ 1` and the rung refuses
    /// only where the arm itself is within the band. Each door ends it
    /// from what it passes on and what its arm is: the Boolean's
    /// cross-operand doors from the corner's sense (`refusal_routes::CORNER_SENSE`), the merge and the
    /// maximal-faces gate from their own decisions.
    Orientation,
    /// Whether the two planes' normals can be read at all: the
    /// parallelism rung's norm read definitely negative, poisoned input
    /// (`unreadable_norm`). No declaration and no move of the parts
    /// reads it.
    Norm,
}

impl PlaneRung {
    /// What the rung decides, as a clause with no colon or dash of its
    /// own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::Parallel => "whether the two planes are parallel",
            Self::Orientation => "whether the two planes face the same way or opposite ways",
            Self::Norm => "whether the normals of two faces can be read",
        }
    }
}

/// The orientation rung's refusal of a zero verdict, its decided
/// margin riding the diagnostics.
pub(super) fn orientation_zero(margin: MarginDiag, band: Band) -> PlaneEqError {
    PlaneEqError::Escalated {
        rung: PlaneRung::Orientation,
        diag: Indeterminate {
            margin,
            band,
            predicate: Some("bool_plane_orient"),
            terminal_sliver: false,
        },
    }
}

/// Whether the consuming op's recipe data declares the face pair
/// coincident (F5). The no-declaration value
/// ([`PlaneIdentity::NONE`]) runs the geometric rungs only.
#[derive(Clone, Copy, Debug, Default)]
pub struct PlaneIdentity {
    /// The face pair is declared coincident by recipe data.
    pub declared: bool,
}

impl PlaneIdentity {
    /// No declaration: the geometric rungs decide.
    pub const NONE: PlaneIdentity = PlaneIdentity { declared: false };
    /// A declared pair, read as one displacement over its extent.
    pub const DECLARED: PlaneIdentity = PlaneIdentity { declared: true };
}

/// One plane's conventional description: a point on it and its unit
/// **outward** normal — outward for the FACE the description came
/// from, which is the surface's chart normal with that face's
/// `sense` folded in, not the chart normal itself
/// (`boolean::reduce::face_plane` is the door that folds the bit in).
/// The Same±-orientation verdict is a statement about material sides,
/// so a description built from a raw chart normal would make it a
/// statement about nothing.
#[derive(Clone, Copy, Debug)]
pub struct PlaneDesc<T: geom_core::Real> {
    /// A point on the plane.
    pub origin: Point3<T>,
    /// The unit outward normal (of the face, not of the chart).
    pub normal: Vec3<T>,
}

/// **`oriented_plane_eq`** — module docs for the ladder. `id` says
/// whether the pair is declared (F5); `extent` the region the verdict
/// is consumed on — the offsets are read at its centre and the angular
/// margins levered at its radius, and a declared pair reads as one
/// displacement over it ([`super::carrier_eq::ConsumedExtent`]); `band`
/// the run's linear band.
///
/// # Errors
///
/// [`PlaneEqError`] — sliver escalation, an undecided coincidence, or a
/// contradicted declaration.
pub fn oriented_plane_eq<T: Decide>(
    p1: &PlaneDesc<T>,
    p2: &PlaneDesc<T>,
    id: PlaneIdentity,
    extent: &ConsumedExtent<'_, T>,
    band: Band,
) -> Result<PlaneRelation, PlaneEqError> {
    oriented_plane_eq_verdict(p1, p2, id, extent, band).map(|(rel, _)| rel)
}

/// [`oriented_plane_eq`] plus the TRILEAN: whether the verdict stands
/// on the geometry's own definite evidence
/// ([`ContactVerdict::Definite`]) or on the declaration bridging an
/// in-band residue ([`ContactVerdict::Bridged`]).
///
/// One implementation, two projections — the plain door drops the
/// trilean for the callers that only need the relation, and no second
/// traversal of the margins exists to drift from this one. C4's
/// invariant ("a declaration is trusted exactly on its bridged
/// residue and nowhere else") is only checkable by a caller that can
/// SEE the residue, which is what this door is for.
///
/// # Errors
///
/// [`PlaneEqError`] — as [`oriented_plane_eq`].
pub fn oriented_plane_eq_verdict<T: Decide>(
    p1: &PlaneDesc<T>,
    p2: &PlaneDesc<T>,
    id: PlaneIdentity,
    extent: &ConsumedExtent<'_, T>,
    band: Band,
) -> Result<(PlaneRelation, ContactVerdict), PlaneEqError> {
    plane_ladder(p1, p2, id, extent, band).map(|(rel, verdict, _)| (rel, verdict))
}

/// The ladder itself (module docs), with the margin that decided a
/// coincidence: the declared reading's, or the offset's decided Zero.
pub(super) fn plane_ladder<T: Decide>(
    p1: &PlaneDesc<T>,
    p2: &PlaneDesc<T>,
    id: PlaneIdentity,
    extent: &ConsumedExtent<'_, T>,
    band: Band,
) -> Result<super::carrier_eq::CarrierReading, PlaneEqError> {
    // Rung 1: declared pair (F5) — verified intent, never trusted
    // blindly, through the one declared reading every carrier kind
    // shares.
    if id.declared {
        let desc = |p: &PlaneDesc<T>| CarrierDesc::Plane {
            origin: p.origin,
            normal: p.normal,
        };
        return super::carrier_eq::declared_reading(&desc(p1), &desc(p2), extent, band)
            .map(|(rel, verdict, margin)| (rel, verdict, Some(margin)));
    }
    // Offsets read at the extent's centre (`d = n̂·(origin − c)`) for
    // the geometric rungs, which lever the angular data at its radius.
    let centre = extent.reach.center();
    let arm = extent.reach.radius();
    let d1 = p1.normal.dot(p1.origin - centre);
    let d2 = p2.normal.dot(p2.origin - centre);

    // Rung 2: definite-different by geometry. Parallelism first.
    let parallel_margin = Margin::levered(p1.normal.cross(p2.normal).norm(), arm);
    match decide("bool_plane_parallel", parallel_margin, band) {
        Ok(Sign::Positive) => return Ok((PlaneRelation::Distinct, ContactVerdict::Definite, None)),
        Ok(Sign::Zero) => {}
        Ok(Sign::Negative) => {
            // A norm cannot be definitely negative — poisoned input.
            return Err(unreadable_norm(band));
        }
        Err(diag) => {
            return Err(PlaneEqError::Escalated {
                rung: PlaneRung::Parallel,
                diag,
            });
        }
    }
    // Parallel (within band): orientation sign from the normal dot
    // (definite by construction when the cross is ~0 and both unit),
    // metered at the caller's lever arm — a unit·unit cosine is
    // dimensionless, and the length band wants the displacement the
    // orientation flip induces at the arm (rim-dimensional audit,
    // class (c); |cos| ≈ 1 here so the margin is ≈ ±arm, decisive).
    let sign_margin = Margin::levered(p1.normal.dot(p2.normal), arm);
    // `relation` rides beside σ: it is the orientation this decision
    // just made definite, and the relation the offset's Zero glues
    // under.
    let (sigma, relation) = match decide_reported("bool_plane_orient", sign_margin, band) {
        Ok(Decided {
            sign: Sign::Positive,
            ..
        }) => (T::one(), PlaneRelation::SameOriented),
        Ok(Decided {
            sign: Sign::Negative,
            ..
        }) => (-T::one(), PlaneRelation::SameOpposite),
        Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) => return Err(orientation_zero(margin, band)),
        Err(diag) => {
            return Err(PlaneEqError::Escalated {
                rung: PlaneRung::Orientation,
                diag,
            });
        }
    };
    let offset_margin = Margin::of(d1 - sigma * d2);
    match CoincidenceMeasure::decide("bool_plane_offset", offset_margin, band) {
        Ok(Decided {
            sign: Sign::Positive | Sign::Negative,
            ..
        }) => Ok((PlaneRelation::Distinct, ContactVerdict::Definite, None)),
        // Rung 3: one plane, decided by its margin — Zero glues.
        Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) => Ok((relation, ContactVerdict::Definite, Some(margin))),
        Err(coincidence) => Err(PlaneEqError::Undecided {
            coincidence,
            relation,
        }),
    }
}

/// **Which way two faces' outward normals point, as a door reads it
/// for the declaration it offers**: the orientation rung's own question
/// (`bool_plane_orient`, the cosine levered at `arm`), asked of a pair
/// whose parallelism a rung could not decide. `None` where the sign is
/// not definite: the senses are unread, and no class is offered on them.
pub(crate) fn senses<T: Decide>(
    n1: Vec3<T>,
    n2: Vec3<T>,
    arm: T,
    band: Band,
) -> Option<PlaneRelation> {
    match decide("bool_plane_orient", Margin::levered(n1.dot(n2), arm), band) {
        Ok(Sign::Positive) => Some(PlaneRelation::SameOriented),
        Ok(Sign::Negative) => Some(PlaneRelation::SameOpposite),
        Ok(Sign::Zero) | Err(_) => None,
    }
}

/// The parallelism rung's refusal of a norm that read definitely
/// negative: poisoned input, with no margin to report.
pub(crate) fn unreadable_norm(band: Band) -> PlaneEqError {
    PlaneEqError::Escalated {
        rung: PlaneRung::Norm,
        diag: Indeterminate {
            margin: MarginDiag::INVALID,
            band,
            predicate: Some("bool_plane_parallel"),
            terminal_sliver: false,
        },
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Band;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn plane(o: [f64; 3], n: [f64; 3]) -> PlaneDesc<f64> {
        PlaneDesc {
            origin: Point3::from_array(o),
            normal: Vec3::from_array(n),
        }
    }

    /// **Zero glues, whatever the planes' descriptions** (D10): two
    /// copies of one plane read one oriented plane, and exact negations
    /// read one plane opposed, each with the offset's decided margin.
    #[test]
    fn equal_planes_glue_on_their_decided_margin() {
        let p1 = plane([1.0, 2.0, 5.0], [0.0, 0.0, 1.0]);
        let p1_rev = plane([1.0, 2.0, 5.0], [-0.0, -0.0, -1.0]);
        for (other, want) in [
            (p1, PlaneRelation::SameOriented),
            (p1_rev, PlaneRelation::SameOpposite),
        ] {
            let (rel, verdict, margin) =
                plane_ladder(&p1, &other, PlaneIdentity::NONE, &ConsumedExtent::arm(1.0), band())
                    .unwrap();
            assert_eq!((rel, verdict), (want, ContactVerdict::Definite));
            let margin = margin.expect("a Zero verdict carries the margin it was decided on");
            assert_eq!(margin.kind(), geom_core::MarginKind::Value, "{margin:?}");
        }
    }

    /// The declared-pair rung (F5): intent + non-contradiction glues
    /// (both orientations); a definitely-distinct pair refuses
    /// `Contradicted` — declarations are verified, never trusted.
    #[test]
    fn declared_pair_rung() {
        let declared = PlaneIdentity::DECLARED;
        let p1 = plane([1.0, 2.0, 5.0], [0.0, 0.0, 1.0]);
        let p2 = plane([-3.0, 7.0, 5.0], [0.0, 0.0, 1.0]);
        assert_eq!(
            oriented_plane_eq(&p1, &p2, declared, &ConsumedExtent::arm(1.0), band()).unwrap(),
            PlaneRelation::SameOriented
        );
        let p3 = plane([0.0, 0.0, 5.0], [0.0, 0.0, -1.0]);
        assert_eq!(
            oriented_plane_eq(&p1, &p3, declared, &ConsumedExtent::arm(1.0), band()).unwrap(),
            PlaneRelation::SameOpposite
        );
        // Contradiction, offset flavor: declared but definitely apart.
        let apart = plane([0.0, 0.0, 9.0], [0.0, 0.0, 1.0]);
        let err = oriented_plane_eq(&p1, &apart, declared, &ConsumedExtent::arm(1.0), band())
            .unwrap_err();
        assert!(matches!(err, PlaneEqError::Contradicted { .. }), "{err:?}");
        // Contradiction, angle flavor: declared but not parallel.
        // Only a point known on a face can show it: unwitnessed, the
        // tilt is unsettled.
        let tilted = plane([1.0, 2.0, 5.0], [0.0, 1.0, 0.0]);
        let err = oriented_plane_eq(&p1, &tilted, declared, &ConsumedExtent::arm(1.0), band())
            .unwrap_err();
        assert!(matches!(err, PlaneEqError::Escalated { .. }), "{err:?}");
        let on_p1 = [Point3::new(1.0, 3.0, 5.0)];
        let witnessed = ConsumedExtent {
            on: [&on_p1, &[]],
            ..ConsumedExtent::arm(1.0)
        };
        let err = oriented_plane_eq(&p1, &tilted, declared, &witnessed, band()).unwrap_err();
        assert!(matches!(err, PlaneEqError::Contradicted { .. }), "{err:?}");
    }

    /// Definitely different planes: non-parallel, and parallel-offset.
    #[test]
    fn distinct_rungs() {
        let p1 = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let tilted = plane([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert_eq!(
            oriented_plane_eq(
                &p1,
                &tilted,
                PlaneIdentity::NONE,
                &ConsumedExtent::arm(1.0),
                band()
            )
            .unwrap(),
            PlaneRelation::Distinct
        );
        let offset = plane([0.0, 0.0, 4.0], [0.0, 0.0, 1.0]);
        assert_eq!(
            oriented_plane_eq(
                &p1,
                &offset,
                PlaneIdentity::NONE,
                &ConsumedExtent::arm(1.0),
                band()
            )
            .unwrap(),
            PlaneRelation::Distinct
        );
        // Opposite-oriented offset plane is also distinct.
        let offset_flip = plane([0.0, 0.0, 4.0], [0.0, 0.0, -1.0]);
        assert_eq!(
            oriented_plane_eq(
                &p1,
                &offset_flip,
                PlaneIdentity::NONE,
                &ConsumedExtent::arm(1.0),
                band()
            )
            .unwrap(),
            PlaneRelation::Distinct
        );
    }

    /// **A coincidence decided Zero is one plane, and a poisoned offset
    /// decides nothing.** Two planes a quarter of ε apart, described
    /// differently, read one plane on the offset's decided margin; an
    /// offset datum that is NaN refuses with its poisoned margin.
    #[test]
    fn a_decided_zero_offset_glues_and_a_poisoned_one_refuses() {
        let p1 = plane([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        let eps = geom_core::Tol::witness().get().eps;
        let read = |p2: PlaneDesc<f64>| {
            plane_ladder(&p1, &p2, PlaneIdentity::NONE, &ConsumedExtent::arm(1.0), band())
        };
        let (rel, _, margin) = read(plane([0.0, 0.0, 5.0 + 0.25 * eps], [0.0, 0.0, 1.0]))
            .expect("a quarter-ε offset decides zero");
        assert_eq!(rel, PlaneRelation::SameOriented);
        assert_eq!(
            margin.map(|m| m.kind()),
            Some(geom_core::MarginKind::Value),
            "the decided margin rides: {margin:?}"
        );
        let err = read(plane([f64::NAN, 0.0, 5.0], [0.0, 0.0, 1.0])).unwrap_err();
        let PlaneEqError::Undecided {
            coincidence: CoincidenceMeasure::Unreadable(diag),
            ..
        } = err
        else {
            panic!("a NaN offset decides nothing: {err:?}");
        };
        assert!(diag.margin.is_invalid(), "poisoned: {diag:?}");
        assert_eq!(diag.predicate, Some("bool_plane_offset"));
    }

    /// A near-miss in the sliver band escalates typed.
    #[test]
    fn sliver_offset_escalates() {
        let p1 = plane([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        let eps = geom_core::Tol::witness().get().eps;
        let k = geom_core::Tol::witness().get().k;
        let p2 = plane([0.0, 0.0, 5.0 + 0.5 * k * eps], [0.0, 0.0, 1.0]);
        let err = oriented_plane_eq(
            &p1,
            &p2,
            PlaneIdentity::NONE,
            &ConsumedExtent::arm(1.0),
            band(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            PlaneEqError::Undecided {
                coincidence: CoincidenceMeasure::Undecided(_),
                ..
            }
        ));
    }
}
