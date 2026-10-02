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
//! In-band margins escalate typed ([`PlaneEqError::Escalated`]), naming
//! the rung that could not decide ([`PlaneRung`]).
//!
//! **Retirement DONE (M4 PR 5; DESIGN.md roadmap; Ev, #53)**: the
//! M3-era rung 1 (canonical `(n̂, d)` bit comparison through
//! `geom_core::bit_identity`) left production. The bit channel
//! survives here only through `crate::source`'s
//! `cfg(debug_assertions)` helpers, inside `debug_assert!` — the
//! "records agree with bits" assertion N6 promises.

use geom_brep::recourse::Classified;
use geom_core::{Band, Decide, Decided, Indeterminate, Margin, MarginDiag, Point3, Sign, Vec3};

use super::refusal_routes::Contradiction;
use crate::contact::ContactVerdict;
use crate::source::GeomSource;
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
/// Since LIB-PYG5 (R3) the `Undeclared` arm carries the RELATION the
/// ladder decided before refusing — see the enum's own docs.
pub use super::carrier_eq::CarrierEqError as PlaneEqError;

/// Which rung of the plane ladder could not decide
/// ([`PlaneEqError::Escalated`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum PlaneRung {
    /// Whether the two planes are parallel (`bool_plane_parallel`). On
    /// an undeclared pair an in-band margin is a coincidence a
    /// declaration would bridge; on a declared pair the rung bridges it.
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
fn orientation_zero(margin: MarginDiag, band: Band) -> PlaneEqError {
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

/// The identity evidence for one oriented-plane comparison (M4 PR 5):
/// the two descriptions' recipe sources (N6) and whether the consuming
/// op's recipe data declares this face pair coincident (F5). The
/// no-evidence value ([`PlaneIdentity::NONE`]) runs the geometric
/// rungs only.
#[derive(Clone, Copy, Debug, Default)]
pub struct PlaneIdentity<'a> {
    /// The first description's recipe source, if stamped. This is the
    /// source of the DESCRIPTION being compared, not of the surface
    /// underneath it: a face's outward normal is the surface
    /// expression's reversal when the face's `sense` is `false`, and
    /// `orient` is the tag that says so, so callers holding faces
    /// pass `boolean::reduce::face_oriented_source`, never the raw
    /// surface source (S10).
    pub s1: Option<&'a GeomSource>,
    /// The second description's recipe source, same contract as
    /// [`PlaneIdentity::s1`].
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

/// A plane description's data, its normal negated when `flip` — rung
/// 1's walk for its bit assertion. Destructured without `..`, so a
/// field the description gains is a compile error here.
#[cfg(debug_assertions)]
fn plane_data<T: geom_core::Real>(
    &PlaneDesc { origin, normal }: &PlaneDesc<T>,
    flip: bool,
) -> [geom::DatumValue<T>; 2] {
    let normal = if flip { -normal } else { normal };
    [
        geom::DatumValue::Point(origin),
        geom::DatumValue::Direction(normal),
    ]
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
    oriented_plane_eq_verdict(p1, p2, id, arm, band).map(|(rel, _)| rel)
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
    id: PlaneIdentity<'_>,
    arm: T,
    band: Band,
) -> Result<(PlaneRelation, ContactVerdict), PlaneEqError> {
    plane_ladder(p1, p2, id, arm, band).map_err(LadderRefusal::untyped)
}

/// [`oriented_plane_eq`] with rung 4's decided zero typed
/// ([`LadderRefusal::Coplanar`]), for a door that ends it from its
/// decided margin: the maximal-faces gate.
pub(crate) fn plane_eq_typed<T: Decide>(
    p1: &PlaneDesc<T>,
    p2: &PlaneDesc<T>,
    id: PlaneIdentity<'_>,
    arm: T,
    band: Band,
) -> Result<PlaneRelation, LadderRefusal> {
    plane_ladder(p1, p2, id, arm, band).map(|(rel, _)| rel)
}

/// The ladder's refusal, with rung 4's decided zero carried typed.
#[derive(Debug)]
pub(crate) enum LadderRefusal {
    /// Rung 4: the offset decided zero between two planes no identity
    /// rung glued, with the margin its band decided and the orientation
    /// decided before it.
    Coplanar {
        /// The decided offset.
        offset: Classified,
        /// The decided orientation.
        relation: PlaneRelation,
    },
    /// Any other refusal.
    Refused(PlaneEqError),
}

impl LadderRefusal {
    /// The refusal as [`PlaneEqError`], whose `Undeclared` carries a
    /// decided zero as `MarginDiag::INVALID`: the encoding the readers
    /// of the untyped error still take
    /// (`work/topo/plane-offset-rung-decided-zero-shares-invalid-with-a-poisoned-margin.md`).
    fn untyped(self) -> PlaneEqError {
        match self {
            Self::Coplanar { offset, relation } => PlaneEqError::Undeclared {
                diag: Indeterminate {
                    margin: geom_core::MarginDiag::INVALID,
                    band: offset.band,
                    predicate: Some("bool_plane_offset"),
                    terminal_sliver: false,
                },
                relation,
            },
            Self::Refused(refusal) => refusal,
        }
    }
}

/// The ladder itself (module docs).
fn plane_ladder<T: Decide>(
    p1: &PlaneDesc<T>,
    p2: &PlaneDesc<T>,
    id: PlaneIdentity<'_>,
    arm: T,
    band: Band,
) -> Result<(PlaneRelation, ContactVerdict), LadderRefusal> {
    // Canonical offsets (d = n̂·origin) for the geometric rungs.
    let d1 = p1.normal.dot(p1.origin - Point3::origin());
    let d2 = p2.normal.dot(p2.origin - Point3::origin());

    // Rung 1: same source (N6) — the declared-identity predicate over
    // the two descriptions' sources, zero numerics. `orient` carries
    // the DESCRIPTION's reversal, face sense included (see
    // `PlaneIdentity::s1`), so a mirrored declaration is an opposite
    // outward normal: without that composition two faces of one
    // surface with opposite senses would read `SameSource`, then
    // `SameOriented`, and blow the assertion below — their outward
    // normals are exact negations.
    use crate::source::SurfaceDeclaration as D;
    let opposite = match crate::source::source_declaration(id.s1, id.s2) {
        D::SameSource => Some(false),
        D::Mirrored => Some(true),
        D::SameKey | D::DistinctSources | D::Unsourced => None,
    };
    if let Some(opposite) = opposite {
        // Asserted only where the scalar HAS a bit channel: the rung is
        // syntactic (the source decides), the bits are its evidence,
        // and a scalar with no channel (`Dual`, `Sym`) offers none —
        // `None` there is not disagreement.
        #[cfg(debug_assertions)]
        if let Some(agree) = crate::source::data_bits_witness(
            plane_data(p1, false)
                .into_iter()
                .zip(plane_data(p2, opposite)),
        ) {
            debug_assert!(
                agree,
                "same-source theorem violated: same-source descriptions disagree bitwise (kernel \
                 bug: a source survived a geometric rewrite)"
            );
        }
        // Rung 1 is syntactic: nothing was measured, so nothing is
        // bridged.
        return Ok((
            if opposite {
                PlaneRelation::SameOpposite
            } else {
                PlaneRelation::SameOriented
            },
            ContactVerdict::Definite,
        ));
    }

    // Rung 2: declared pair (F5) — verified intent, never trusted
    // blindly. Definitely-distinct contradicts the declaration.
    if id.declared {
        return declared_rung(p1, p2, d1, d2, arm, band).map_err(LadderRefusal::Refused);
    }

    // Rung 3: definite-different by geometry. Parallelism first.
    let parallel_margin = Margin::levered(p1.normal.cross(p2.normal).norm(), arm);
    match decide("bool_plane_parallel", parallel_margin, band) {
        Ok(Sign::Positive) => return Ok((PlaneRelation::Distinct, ContactVerdict::Definite)),
        Ok(Sign::Zero) => {}
        Ok(Sign::Negative) => {
            // A norm cannot be definitely negative — poisoned input.
            return Err(LadderRefusal::Refused(unreadable_norm(band)));
        }
        Err(diag) => {
            return Err(LadderRefusal::Refused(PlaneEqError::Escalated {
                rung: PlaneRung::Parallel,
                diag,
            }));
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
    // just made definite, and the relation an `Undeclared` refusal
    // names (only the offset's coincidence lacks intent there).
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
        }) => return Err(LadderRefusal::Refused(orientation_zero(margin, band))),
        Err(diag) => {
            return Err(LadderRefusal::Refused(PlaneEqError::Escalated {
                rung: PlaneRung::Orientation,
                diag,
            }));
        }
    };
    let offset_margin = Margin::of(d1 - sigma * d2);
    match decide_reported("bool_plane_offset", offset_margin, band) {
        Ok(Decided {
            sign: Sign::Positive | Sign::Negative,
            ..
        }) => Ok((PlaneRelation::Distinct, ContactVerdict::Definite)),
        // Rung 4: geometrically the same plane, but neither identity
        // rung fired — undeclared coincidence, typed (rung (b): value
        // equality never glues).
        Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) => Err(LadderRefusal::Coplanar {
            offset: Classified { margin, band },
            relation,
        }),
        Err(diag) => Err(LadderRefusal::Refused(PlaneEqError::Undeclared {
            diag,
            relation,
        })),
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
) -> Result<(PlaneRelation, ContactVerdict), PlaneEqError> {
    // The residue the declaration is trusted on, and only on: set by
    // whichever margins landed in band (C4's third list).
    let mut bridged = false;
    let parallel_margin = Margin::levered(p1.normal.cross(p2.normal).norm(), arm);
    match decide("bool_plane_parallel", parallel_margin, band) {
        Ok(Sign::Positive) => {
            return Err(PlaneEqError::Contradicted {
                fact: Contradiction::PlanesNotParallel,
                diag: Indeterminate {
                    margin: geom_core::MarginDiag::INVALID,
                    band,
                    predicate: Some("bool_plane_parallel"),
                    terminal_sliver: false,
                },
            });
        }
        Ok(Sign::Zero) => {}
        Ok(Sign::Negative) => return Err(unreadable_norm(band)),
        // In-band parallelism does not contradict the declaration —
        // it IS the bridged residue.
        Err(_) => bridged = true,
    }
    // Metered at the arm like the undeclared rung (class (c) above).
    let same_orient = match decide_reported(
        "bool_plane_orient",
        Margin::levered(p1.normal.dot(p2.normal), arm),
        band,
    ) {
        Ok(Decided {
            sign: Sign::Positive,
            ..
        }) => true,
        Ok(Decided {
            sign: Sign::Negative,
            ..
        }) => false,
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
    let sigma = if same_orient { T::one() } else { -T::one() };
    match decide("bool_plane_offset", Margin::of(d1 - sigma * d2), band) {
        Ok(Sign::Positive | Sign::Negative) => Err(PlaneEqError::Contradicted {
            fact: Contradiction::PlanesApart,
            diag: Indeterminate {
                margin: geom_core::MarginDiag::INVALID,
                band,
                predicate: Some("bool_plane_offset"),
                terminal_sliver: false,
            },
        }),
        // Coincident: the geometry stands on its own.
        Ok(Sign::Zero) => Ok((
            if same_orient {
                PlaneRelation::SameOriented
            } else {
                PlaneRelation::SameOpposite
            },
            if bridged {
                ContactVerdict::Bridged
            } else {
                ContactVerdict::Definite
            },
        )),
        // In-band: the declaration bridges exactly this, so the
        // verdict is Bridged regardless of what the parallelism arm
        // already found (`bridged` can only have been true).
        Err(_) => Ok((
            if same_orient {
                PlaneRelation::SameOriented
            } else {
                PlaneRelation::SameOpposite
            },
            ContactVerdict::Bridged,
        )),
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
        assert!(matches!(err, PlaneEqError::Undeclared { .. }), "{err:?}");
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
        assert!(matches!(err, PlaneEqError::Contradicted { .. }), "{err:?}");
        // Contradiction, angle flavor: declared but not parallel.
        let tilted = plane([1.0, 2.0, 5.0], [0.0, 1.0, 0.0]);
        let err = oriented_plane_eq(&p1, &tilted, declared, 1.0, band()).unwrap_err();
        assert!(matches!(err, PlaneEqError::Contradicted { .. }), "{err:?}");
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
        let eps = geom_core::Tol::witness().get().eps;
        // Same plane to within a fraction of ε, described differently.
        let p2 = plane([0.0, 0.0, 5.0 + 0.25 * eps], [0.0, 0.0, 1.0]);
        let err = oriented_plane_eq(&p1, &p2, PlaneIdentity::NONE, 1.0, band()).unwrap_err();
        assert!(matches!(err, PlaneEqError::Undeclared { .. }), "{err:?}");
    }

    /// A near-miss in the sliver band escalates typed.
    #[test]
    fn sliver_offset_escalates() {
        let p1 = plane([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        let eps = geom_core::Tol::witness().get().eps;
        let k = geom_core::Tol::witness().get().k;
        let p2 = plane([0.0, 0.0, 5.0 + 0.5 * k * eps], [0.0, 0.0, 1.0]);
        let err = oriented_plane_eq(&p1, &p2, PlaneIdentity::NONE, 1.0, band()).unwrap_err();
        assert!(matches!(
            err,
            PlaneEqError::Undeclared { .. } | PlaneEqError::Escalated { .. }
        ));
    }
}
