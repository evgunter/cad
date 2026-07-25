//! **Oriented-plane-equality** — the typed replacement for ch. 15's
//! `vecequal` on raw plane 4-vectors (Program 15.10's ⁺/⁻ gate, notes'
//! predicate inventory: "fragile on unnormalized Newell vectors").
//!
//! Verdicts: *same plane, same orientation* / *same plane, opposite
//! orientation* / *different plane* — plus the typed refusals the
//! coincidence ladder demands. The rungs, in order:
//!
//! 1. **Same-source rung (syntactic, N6 — the M4 retirement)**: both
//!    descriptions carry a [`GeomSource`] with the same base
//!    `(node, expr)` ⇒ same plane, by the N6 theorem (same source ⇒
//!    bit-identical descriptions, D9 determinism); equal `orient` ⇒
//!    [`PlaneRelation::SameOriented`], opposite ⇒
//!    [`PlaneRelation::SameOpposite`]. A provenance lookup, zero
//!    numerics; the retired bit comparison survives only as the
//!    debug assertion that the records agree with the bits.
//! 2. **Declared-pair rung (recipe intent, F5)**: the consuming op's
//!    recipe data declares THIS face pair coincident
//!    ([`PlaneIdentity::declared`]). The declaration is verified, not
//!    trusted: a definitely-distinct pair refuses
//!    ([`PlaneEqError::Contradicted`]) — a wrong declaration must
//!    never force wrong geometry; within band, the decided
//!    orientation sign picks Same±.
//! 3. **Geometric trilean (definite-different only)**: parallelism
//!    margin `‖n₁ × n₂‖·arm` (`bool_plane_parallel`) definitely
//!    positive ⇒ [`PlaneRelation::Distinct`]; else offset margin
//!    `(d₁ − σ·d₂)` (`bool_plane_offset`, σ the orientation sign)
//!    definitely nonzero ⇒ parallel-but-offset ⇒ `Distinct`.
//! 4. Geometrically coincident-or-near **without** shared source or
//!    declared intent ⇒ [`PlaneEqError::Undeclared`] —
//!    near-coincidence NEVER silently becomes contact (F6), and
//!    value-equality never certifies coincidence (the ladder's
//!    ratified rung (b); equal bits without shared source stay
//!    unglued — the M4 PR 5 narrowing of the M3-era bit rung).
//!
//! In-band margins escalate typed ([`PlaneEqError::Escalated`]).
//!
//! **Retirement DONE (M4 PR 5; DESIGN.md roadmap; Evan, #53)**: the
//! M3-era rung 1 (canonical `(n̂, d)` bit comparison through
//! `geom_core::bit_identity`) left production. The bit channel
//! survives here only through `crate::source`'s
//! `cfg(debug_assertions)` helpers, inside `debug_assert!` — the
//! "records agree with bits" assertion N6 promises.

use geom_core::{Band, Decide, Indeterminate, Point3, Sign, Vec3};

use crate::source::GeomSource;
use crate::validate::decide;

/// The relation between two oriented planes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaneRelation {
    /// Same plane, same orientation (the ⁺ case of Eq. 15.3).
    SameOriented,
    /// Same plane, opposite orientation (the ⁻ case).
    SameOpposite,
    /// Definitely different planes.
    Distinct,
}

/// Typed refusal of [`oriented_plane_eq`].
#[derive(Debug)]
pub enum PlaneEqError {
    /// A margin landed in the sliver band.
    Escalated(Indeterminate),
    /// Geometrically coincident-or-near without shared source or
    /// declared intent: an undeclared coincidence (F6).
    Undeclared(Indeterminate),
    /// A declared-coincident pair whose planes are DEFINITELY distinct
    /// — the recipe's declaration contradicts the geometry; refused
    /// loudly, never glued (rung 2's verification direction).
    Contradicted(Indeterminate),
}

/// The identity evidence for one oriented-plane comparison (M4 PR 5):
/// the two descriptions' recipe sources (N6) and whether the consuming
/// op's recipe data declares this face pair coincident (F5). The
/// no-evidence value ([`PlaneIdentity::NONE`]) runs the geometric
/// rungs only.
#[derive(Clone, Copy, Debug, Default)]
pub struct PlaneIdentity<'a> {
    /// The first description's recipe source, if stamped.
    pub s1: Option<&'a GeomSource>,
    /// The second description's recipe source, if stamped.
    pub s2: Option<&'a GeomSource>,
    /// The face pair is declared coincident by recipe data.
    pub declared: bool,
}

impl PlaneIdentity<'_> {
    /// No identity evidence: raw geometric comparison.
    pub const NONE: PlaneIdentity<'static> = PlaneIdentity {
        s1: None,
        s2: None,
        declared: false,
    };
}

/// One plane's conventional description (a `Surface::Plane`'s origin
/// and unit outward normal).
#[derive(Clone, Copy, Debug)]
pub struct PlaneDesc<T: geom_core::Real> {
    /// A point on the plane.
    pub origin: Point3<T>,
    /// The unit outward normal.
    pub normal: Vec3<T>,
}

/// **`oriented_plane_eq`** — module docs for the ladder. `id` is the
/// comparison's identity evidence (sources + declared intent, M4
/// PR 5); `arm` is the lever arm in meters metering the angular/offset
/// margins (the extent over which the verdict is consumed); `band` the
/// run's linear band.
///
/// # Errors
///
/// [`PlaneEqError`] — sliver escalation, undeclared coincidence, or a
/// contradicted declaration.
pub fn oriented_plane_eq<T: Decide>(
    p1: &PlaneDesc<T>,
    p2: &PlaneDesc<T>,
    id: PlaneIdentity<'_>,
    arm: T,
    band: Band,
) -> Result<PlaneRelation, PlaneEqError> {
    // Canonical offsets (d = n̂·origin) for the geometric rungs.
    let d1 = p1.normal.dot(p1.origin - Point3::origin());
    let d2 = p2.normal.dot(p2.origin - Point3::origin());

    // Rung 1: same source (N6) — syntactic identity, zero numerics.
    if let (Some(s1), Some(s2)) = (id.s1, id.s2)
        && s1.same_base(s2)
    {
        let opposite = s1.orient != s2.orient;
        #[cfg(debug_assertions)]
        debug_assert!(
            crate::source::plane_bits_agree(p1.origin, p1.normal, p2.origin, p2.normal, opposite),
            "N6 theorem violated: same-source descriptions disagree bitwise (kernel bug: \
             a source survived a geometric rewrite)"
        );
        return Ok(if opposite {
            PlaneRelation::SameOpposite
        } else {
            PlaneRelation::SameOriented
        });
    }

    // Rung 2: declared pair (F5) — verified intent, never trusted
    // blindly. Definitely-distinct contradicts the declaration.
    if id.declared {
        return declared_rung(p1, p2, d1, d2, arm, band);
    }

    // Rung 3: definite-different by geometry. Parallelism first.
    let parallel_margin = p1.normal.cross(p2.normal).norm() * arm;
    match decide("bool_plane_parallel", parallel_margin, band) {
        Ok(Sign::Positive) => return Ok(PlaneRelation::Distinct),
        Ok(Sign::Zero) => {}
        Ok(Sign::Negative) => {
            // A norm cannot be definitely negative — poisoned input.
            return Err(PlaneEqError::Escalated(Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("bool_plane_parallel"),
            }));
        }
        Err(diag) => return Err(PlaneEqError::Escalated(diag)),
    }
    // Parallel (within band): orientation sign from the normal dot
    // (definite by construction when the cross is ~0 and both unit).
    let sign_margin = p1.normal.dot(p2.normal);
    let sigma = match decide("bool_plane_orient", sign_margin, band) {
        Ok(Sign::Positive) => T::one(),
        Ok(Sign::Negative) => -T::one(),
        Ok(Sign::Zero) => {
            return Err(PlaneEqError::Escalated(Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("bool_plane_orient"),
            }));
        }
        Err(diag) => return Err(PlaneEqError::Escalated(diag)),
    };
    let offset_margin = d1 - sigma * d2;
    match decide("bool_plane_offset", offset_margin, band) {
        Ok(Sign::Positive | Sign::Negative) => Ok(PlaneRelation::Distinct),
        // Rung 4: geometrically the same plane, but neither identity
        // rung fired — undeclared coincidence, typed (rung (b): value
        // equality never glues).
        Ok(Sign::Zero) => Err(PlaneEqError::Undeclared(Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some("bool_plane_offset"),
        })),
        Err(diag) => Err(PlaneEqError::Undeclared(diag)),
    }
}

/// The declared-pair rung (module docs, rung 2): verify the declared
/// coincidence against the geometry — definitely-distinct planes
/// CONTRADICT the declaration (typed refusal); otherwise the decided
/// orientation sign picks Same±. In-band offset/parallel margins are
/// accepted: the declaration supplies the intent, and the geometry
/// does not contradict it (the inverse of rung 4's refusal — intent
/// plus non-contradiction, never value-inferred coincidence).
fn declared_rung<T: Decide>(
    p1: &PlaneDesc<T>,
    p2: &PlaneDesc<T>,
    d1: T,
    d2: T,
    arm: T,
    band: Band,
) -> Result<PlaneRelation, PlaneEqError> {
    let parallel_margin = p1.normal.cross(p2.normal).norm() * arm;
    match decide("bool_plane_parallel", parallel_margin, band) {
        Ok(Sign::Positive) => {
            return Err(PlaneEqError::Contradicted(Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("bool_plane_parallel"),
            }));
        }
        Ok(Sign::Zero) => {}
        Ok(Sign::Negative) => {
            return Err(PlaneEqError::Escalated(Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("bool_plane_parallel"),
            }));
        }
        // In-band parallelism does not contradict the declaration.
        Err(_) => {}
    }
    let same_orient = match decide("bool_plane_orient", p1.normal.dot(p2.normal), band) {
        Ok(Sign::Positive) => true,
        Ok(Sign::Negative) => false,
        Ok(Sign::Zero) => {
            return Err(PlaneEqError::Escalated(Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("bool_plane_orient"),
            }));
        }
        Err(diag) => return Err(PlaneEqError::Escalated(diag)),
    };
    let sigma = if same_orient { T::one() } else { -T::one() };
    match decide("bool_plane_offset", d1 - sigma * d2, band) {
        Ok(Sign::Positive | Sign::Negative) => Err(PlaneEqError::Contradicted(Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some("bool_plane_offset"),
        })),
        // Coincident or in-band: the declaration stands.
        Ok(Sign::Zero) | Err(_) => Ok(if same_orient {
            PlaneRelation::SameOriented
        } else {
            PlaneRelation::SameOpposite
        }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Band;

    fn band() -> Band {
        Band::linear().unwrap()
    }

    fn plane(o: [f64; 3], n: [f64; 3]) -> PlaneDesc<f64> {
        PlaneDesc {
            origin: Point3::new(o[0], o[1], o[2]),
            normal: Vec3::new(n[0], n[1], n[2]),
        }
    }

    /// The same-source rung (N6): syntactic identity decides exactly;
    /// orient split picks Same±. Descriptions here are bit-identical
    /// copies (same orient) / exact negations (opposite orient) — the
    /// only states same-source can be in (the debug assertion pins
    /// it).
    #[test]
    fn same_source_rungs() {
        use crate::source::GeomSource;
        let p1 = plane([1.0, 2.0, 5.0], [0.0, 0.0, 1.0]);
        let s = GeomSource::minted(7, 3);
        fn id<'a>(s1: &'a GeomSource, s2: &'a GeomSource) -> PlaneIdentity<'a> {
            PlaneIdentity {
                s1: Some(s1),
                s2: Some(s2),
                declared: false,
            }
        }
        assert_eq!(
            oriented_plane_eq(&p1, &p1.clone(), id(&s, &s.clone()), 1.0, band()).unwrap(),
            PlaneRelation::SameOriented
        );
        let p1_rev = plane([1.0, 2.0, 5.0], [-0.0, -0.0, -1.0]);
        let s_rev = s.reverted();
        assert_eq!(
            oriented_plane_eq(&p1, &p1_rev, id(&s, &s_rev), 1.0, band()).unwrap(),
            PlaneRelation::SameOpposite
        );
        // A DIFFERENT base never fires the rung — bit-equal values
        // with independent sources fall through to rung 4 (Undeclared;
        // the ratified rung (b), the M4 PR 5 narrowing).
        let other = GeomSource::minted(9, 3);
        let err = oriented_plane_eq(&p1, &p1.clone(), id(&s, &other), 1.0, band()).unwrap_err();
        assert!(matches!(err, PlaneEqError::Undeclared(_)), "{err:?}");
    }

    /// The declared-pair rung (F5): intent + non-contradiction glues
    /// (both orientations); a definitely-distinct pair refuses
    /// `Contradicted` — declarations are verified, never trusted.
    #[test]
    fn declared_pair_rung() {
        let declared = PlaneIdentity {
            s1: None,
            s2: None,
            declared: true,
        };
        let p1 = plane([1.0, 2.0, 5.0], [0.0, 0.0, 1.0]);
        let p2 = plane([-3.0, 7.0, 5.0], [0.0, 0.0, 1.0]);
        assert_eq!(
            oriented_plane_eq(&p1, &p2, declared, 1.0, band()).unwrap(),
            PlaneRelation::SameOriented
        );
        let p3 = plane([0.0, 0.0, 5.0], [0.0, 0.0, -1.0]);
        assert_eq!(
            oriented_plane_eq(&p1, &p3, declared, 1.0, band()).unwrap(),
            PlaneRelation::SameOpposite
        );
        // Contradiction, offset flavor: declared but definitely apart.
        let apart = plane([0.0, 0.0, 9.0], [0.0, 0.0, 1.0]);
        let err = oriented_plane_eq(&p1, &apart, declared, 1.0, band()).unwrap_err();
        assert!(matches!(err, PlaneEqError::Contradicted(_)), "{err:?}");
        // Contradiction, angle flavor: declared but not parallel.
        let tilted = plane([1.0, 2.0, 5.0], [0.0, 1.0, 0.0]);
        let err = oriented_plane_eq(&p1, &tilted, declared, 1.0, band()).unwrap_err();
        assert!(matches!(err, PlaneEqError::Contradicted(_)), "{err:?}");
    }

    /// Definitely different planes: non-parallel, and parallel-offset.
    #[test]
    fn distinct_rungs() {
        let p1 = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let tilted = plane([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert_eq!(
            oriented_plane_eq(&p1, &tilted, PlaneIdentity::NONE, 1.0, band()).unwrap(),
            PlaneRelation::Distinct
        );
        let offset = plane([0.0, 0.0, 4.0], [0.0, 0.0, 1.0]);
        assert_eq!(
            oriented_plane_eq(&p1, &offset, PlaneIdentity::NONE, 1.0, band()).unwrap(),
            PlaneRelation::Distinct
        );
        // Opposite-oriented offset plane is also distinct.
        let offset_flip = plane([0.0, 0.0, 4.0], [0.0, 0.0, -1.0]);
        assert_eq!(
            oriented_plane_eq(&p1, &offset_flip, PlaneIdentity::NONE, 1.0, band()).unwrap(),
            PlaneRelation::Distinct
        );
    }

    /// Geometrically coincident but bit-different (an independently
    /// renormalized normal): undeclared coincidence, typed — never
    /// silently "same", never silently "different".
    #[test]
    fn near_coincidence_is_undeclared() {
        let p1 = plane([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        let eps = geom_core::Tolerance::get().eps;
        // Same plane to within a fraction of ε, described differently.
        let p2 = plane([0.0, 0.0, 5.0 + 0.25 * eps], [0.0, 0.0, 1.0]);
        let err = oriented_plane_eq(&p1, &p2, PlaneIdentity::NONE, 1.0, band()).unwrap_err();
        assert!(matches!(err, PlaneEqError::Undeclared(_)), "{err:?}");
    }

    /// A near-miss in the sliver band escalates typed.
    #[test]
    fn sliver_offset_escalates() {
        let p1 = plane([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        let eps = geom_core::Tolerance::get().eps;
        let k = geom_core::Tolerance::get().k;
        let p2 = plane([0.0, 0.0, 5.0 + 0.5 * k * eps], [0.0, 0.0, 1.0]);
        let err = oriented_plane_eq(&p1, &p2, PlaneIdentity::NONE, 1.0, band()).unwrap_err();
        assert!(matches!(
            err,
            PlaneEqError::Undeclared(_) | PlaneEqError::Escalated(_)
        ));
    }
}
