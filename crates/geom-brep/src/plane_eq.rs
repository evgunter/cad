//! **Oriented-plane-equality** — the typed replacement for ch. 15's
//! `vecequal` on raw plane 4-vectors (Program 15.10's ⁺/⁻ gate, notes'
//! predicate inventory: "fragile on unnormalized Newell vectors").
//!
//! Verdicts: *same plane, same orientation* / *same plane, opposite
//! orientation* / *different plane* — plus the typed refusals. The
//! rungs, in order:
//!
//! 1. **Geometric trilean (definite-different)**: parallelism margin
//!    `‖n₁ × n₂‖·arm` (`plane_eq_parallel`, the arm the extent's
//!    radius, the offsets read at its centre) definitely positive ⇒
//!    [`CarrierRelation::Distinct`]; else offset margin `(d₁ − σ·d₂)`
//!    (`plane_eq_offset`, σ the orientation sign) definitely nonzero
//!    ⇒ parallel-but-offset ⇒ `Distinct`.
//! 2. **Zero glues** (D10, Booleans): an offset margin decided Zero is
//!    one plane, `SameOriented` or `SameOpposite` by the decided
//!    orientation, and the decided margin rides out with the verdict
//!    for the caller to record (`topo`'s `coincidence`). An offset in
//!    band is a sliver and refuses ([`CarrierEqError::Undecided`]):
//!    near-coincidence never silently becomes contact (F6).
//!
//! In-band parallelism and orientation margins escalate typed
//! ([`CarrierEqError::Escalated`]), naming the rung that could not decide
//! ([`PlaneRung`]). Whether two planes are one construction is not the
//! kernel's question: the document's coincidence door decides it
//! (D10, Coincidence). A pair an operation glued is read through its
//! record, never through this ladder (`[coincidence.booleans]`).

use crate::carrier_eq::{
    CarrierEqError, CarrierRelation, CarrierVerdict, CoincidenceMeasure, ConsumedExtent,
};
use geom_core::{Band, Decide, Decided, Margin, Point3, Sign, Vec3};

use geom_core::NonzeroSign;
use geom_core::k_stats::{decide_nonzero, decide_reported};

/// Which rung of the plane ladder could not decide
/// ([`CarrierEqError::Escalated`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(any(test, feature = "test-support"), derive(strum::EnumIter))]
pub enum PlaneRung {
    /// Whether the two planes are parallel (`plane_eq_parallel`). An
    /// in-band margin is a sliver.
    Parallel,
    /// Whether the two planes face the same way or opposite ways
    /// (`plane_eq_orient`). Its margin is the normals' cosine levered
    /// at the caller's arm, asked only once parallelism has read within
    /// the zero band at that arm, so `|cos| ≈ 1` and the rung refuses
    /// only where the arm itself is within the band. Each door ends it
    /// from what it passes on and what its arm is: the Boolean's
    /// cross-operand doors from the corner's sense (`refusal_routes::CORNER_SENSE`), the merge and the
    /// maximal-faces gate from their own decisions.
    Orientation,
    /// Whether the two planes' normals can be read at all: the
    /// parallelism rung's norm read definitely negative, poisoned input
    /// (`unreadable_norm`). No move of the parts reads it.
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

/// One plane's conventional description: a point on it and its unit
/// **outward** normal — outward for the FACE the description came
/// from, which is the surface's chart normal with that face's
/// `sense` folded in, not the chart normal itself
/// (`face_normal::face_plane` is the door that folds the bit in).
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

/// **`oriented_plane_eq`** — module docs for the ladder. `extent` is
/// the region the verdict is consumed on — the offsets are read at its
/// centre and the angular margins levered at its radius
/// ([`ConsumedExtent`]); `band` the run's linear
/// band.
///
/// # Errors
///
/// [`CarrierEqError`] — sliver escalation, or an undecided coincidence.
pub fn oriented_plane_eq<T: Decide>(
    p1: &PlaneDesc<T>,
    p2: &PlaneDesc<T>,
    extent: &ConsumedExtent<T>,
    band: Band,
) -> Result<CarrierRelation, CarrierEqError> {
    plane_ladder(p1, p2, extent, band).map(CarrierVerdict::relation)
}

/// The ladder itself (module docs), with the margin that decided a
/// coincidence: the offset's decided Zero.
pub(crate) fn plane_ladder<T: Decide>(
    p1: &PlaneDesc<T>,
    p2: &PlaneDesc<T>,
    extent: &ConsumedExtent<T>,
    band: Band,
) -> Result<CarrierVerdict, CarrierEqError> {
    // Offsets read at the extent's centre (`d = n̂·(origin − c)`) for
    // the geometric rungs, which lever the angular data at its radius.
    let centre = extent.reach.center();
    let arm = extent.reach.lever();
    let d1 = p1.normal.dot(p1.origin - centre);
    let d2 = p2.normal.dot(p2.origin - centre);

    // Rung 1: definite-different by geometry. Parallelism first.
    let parallel_margin = Margin::levered(p1.normal.cross(p2.normal).norm(), arm);
    // The offset's Zero below decides one plane only with this one, so
    // the row's margin stands for both.
    let parallel = match decide_reported("plane_eq_parallel", parallel_margin, band) {
        Ok(
            tilt @ Decided {
                sign: Sign::Positive,
                ..
            },
        ) => return Ok(CarrierVerdict::Apart(tilt)),
        Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) => margin,
        Ok(
            norm @ Decided {
                sign: Sign::Negative,
                ..
            },
        ) => return Err(unreadable_norm(norm, band)),
        Err(diag) => {
            return Err(CarrierEqError::Escalated {
                rung: PlaneRung::Parallel,
                diag,
            });
        }
    };
    // Parallel (within band): orientation sign from the normal dot
    // (definite by construction when the cross is ~0 and both unit),
    // metered at the caller's lever arm — a unit·unit cosine is
    // dimensionless, and the length band wants the displacement the
    // orientation flip induces at the arm (rim-dimensional audit,
    // class (c); |cos| ≈ 1 here so the margin is ≈ ±arm, decisive).
    let sign_margin = Margin::levered(p1.normal.dot(p2.normal), arm);
    // `relation` rides beside σ: it is the orientation this decision
    // just made definite, and the relation the offset's Zero glues
    // under. A zero has no side to read, so it is the gate's rejection.
    let side = decide_nonzero("plane_eq_orient", sign_margin, band).map_err(|diag| {
        CarrierEqError::Escalated {
            rung: PlaneRung::Orientation,
            diag,
        }
    })?;
    let sigma = side.unit::<T>();
    let relation = match side {
        NonzeroSign::Positive => CarrierRelation::SameOriented,
        NonzeroSign::Negative => CarrierRelation::SameOpposite,
    };
    let offset_margin = Margin::of(d1 - sigma * d2);
    match CoincidenceMeasure::decide("plane_eq_offset", offset_margin, band) {
        Ok(
            offset @ Decided {
                sign: Sign::Positive | Sign::Negative,
                ..
            },
        ) => Ok(CarrierVerdict::Apart(offset)),
        // Rung 2: one plane, decided by its margin — Zero glues.
        Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) => Ok(CarrierVerdict::One {
            relation,
            margin: margin.jointly(parallel),
        }),
        Err(coincidence) => Err(CarrierEqError::Undecided {
            coincidence,
            relation,
        }),
    }
}

/// The parallelism rung's refusal of a norm decided definitely
/// negative (`norm`, the rung's own reading): poisoned input. A norm
/// passes on a zero or positive sign, so the refusal is the gate's
/// rejection of that sign, minted and logged by the funnel's door
/// ([`geom_core::k_stats::gate_rejection`]): the decided margin, tagged
/// with the rejected sign.
///
/// # Panics
///
/// Where `norm` is no definite negative verdict on its margin against
/// `band` (the door's own precondition).
pub fn unreadable_norm(norm: Decided, band: Band) -> CarrierEqError {
    CarrierEqError::Escalated {
        rung: PlaneRung::Norm,
        diag: geom_core::k_stats::gate_rejection(
            "plane_eq_parallel",
            norm,
            band,
            geom_core::SizedPass::NonNegative,
        ),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Band;
    use geom_core::MarginDiag;
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

    /// **A coincidence of planes is a theorem only when parallelism is
    /// one too.** Both planes pass through the extent's centre, so the
    /// offset rung's margin is the zero polynomial; the second normal
    /// leans by a symbol `t` that only its value, zero, sets upright.
    /// The verdict's margin stands for both Zeros
    /// (`MarginDiag::jointly`), so it discharges as no theorem.
    #[test]
    fn an_offset_identity_on_planes_parallel_by_value_is_no_theorem() {
        use geom_core::sym::deferred::{discharge, with_deferred_session};
        use geom_core::{ParamSymbol, Real, Sym, SymBudget, SymRetry, SymRules};
        type S = Sym<f64>;
        let budget = SymBudget {
            max_terms: 256,
            max_degree: 64,
        };
        let (got, _) = with_deferred_session(budget, SymRules::shipped(), SymRetry::none(), || {
            let k = |v: f64| S::from_f64(v);
            let t = S::param(ParamSymbol::new(test_utils::symbol_id("lean")), 0.0);
            let flat = PlaneDesc {
                origin: Point3::new(k(0.0), k(0.0), k(0.0)),
                normal: Vec3::new(k(0.0), k(0.0), k(1.0)),
            };
            let leaning = PlaneDesc {
                origin: Point3::new(k(0.0), k(0.0), k(0.0)),
                normal: Vec3::new(k(0.0), t, k(1.0)),
            };
            let (rel, margin) = plane_ladder(&flat, &leaning, &ConsumedExtent::arm(k(1.0)), band())
                .unwrap()
                .one()
                .expect("one plane");
            (rel, discharge(margin))
        });
        assert_eq!(got, (CarrierRelation::SameOriented, false));
    }

    /// **Zero glues, whatever the planes' descriptions** (D10): two
    /// copies of one plane read one oriented plane, and exact negations
    /// read one plane opposed, each with the offset's decided margin.
    #[test]
    fn equal_planes_glue_on_their_decided_margin() {
        let p1 = plane([1.0, 2.0, 5.0], [0.0, 0.0, 1.0]);
        let p1_rev = plane([1.0, 2.0, 5.0], [-0.0, -0.0, -1.0]);
        for (other, want) in [
            (p1, CarrierRelation::SameOriented),
            (p1_rev, CarrierRelation::SameOpposite),
        ] {
            let got = plane_ladder(&p1, &other, &ConsumedExtent::arm(1.0), band()).unwrap();
            let Some((rel, margin)) = got.one() else {
                panic!("one plane: {got:?}");
            };
            assert_eq!(rel, want);
            assert_eq!(margin.kind(), geom_core::MarginKind::Value, "{margin:?}");
        }
    }

    /// **Definitely different planes keep the reading that parted
    /// them**: non-parallel on the tilt, parallel-offset on the offset,
    /// each with its sign and the margin it was decided on.
    #[test]
    fn distinct_rungs() {
        let p1 = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let read = |p2| plane_ladder(&p1, &p2, &ConsumedExtent::arm(1.0), band());
        let tilted = plane([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert_eq!(
            read(tilted),
            Ok(CarrierVerdict::Apart(Decided {
                sign: Sign::Positive,
                margin: MarginDiag::value(1.0),
            })),
            "the tilt parts them"
        );
        let offset = plane([0.0, 0.0, 4.0], [0.0, 0.0, 1.0]);
        assert_eq!(
            read(offset),
            Ok(CarrierVerdict::Apart(Decided {
                sign: Sign::Negative,
                margin: MarginDiag::value(-4.0),
            })),
            "the offset parts them"
        );
        // Opposite-oriented offset plane is also distinct.
        let offset_flip = plane([0.0, 0.0, 4.0], [0.0, 0.0, -1.0]);
        assert_eq!(
            oriented_plane_eq(&p1, &offset_flip, &ConsumedExtent::arm(1.0), band()).unwrap(),
            CarrierRelation::Distinct
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
        let read = |p2: PlaneDesc<f64>| plane_ladder(&p1, &p2, &ConsumedExtent::arm(1.0), band());
        let (rel, margin) = read(plane([0.0, 0.0, 5.0 + 0.25 * eps], [0.0, 0.0, 1.0]))
            .expect("a quarter-ε offset decides zero")
            .one()
            .expect("one plane");
        assert_eq!(rel, CarrierRelation::SameOriented);
        assert_eq!(
            margin.kind(),
            geom_core::MarginKind::Value,
            "the decided margin rides: {margin:?}"
        );
        let err = read(plane([f64::NAN, 0.0, 5.0], [0.0, 0.0, 1.0])).unwrap_err();
        let CarrierEqError::Undecided {
            coincidence: CoincidenceMeasure::Unreadable(diag),
            ..
        } = err
        else {
            panic!("a NaN offset decides nothing: {err:?}");
        };
        assert!(diag.margin.is_invalid(), "poisoned: {diag:?}");
        assert_eq!(diag.predicate, Some("plane_eq_offset"));
    }

    /// A near-miss in the sliver band escalates typed.
    #[test]
    fn sliver_offset_escalates() {
        let p1 = plane([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        let eps = geom_core::Tol::witness().get().eps;
        let k = geom_core::Tol::witness().get().k;
        let p2 = plane([0.0, 0.0, 5.0 + 0.5 * k * eps], [0.0, 0.0, 1.0]);
        let err = oriented_plane_eq(&p1, &p2, &ConsumedExtent::arm(1.0), band()).unwrap_err();
        assert!(matches!(
            err,
            CarrierEqError::Undecided {
                coincidence: CoincidenceMeasure::Undecided(_),
                ..
            }
        ));
    }

    /// **An orientation decided zero is the nonzero gate's rejection, on
    /// the log.** Two parallel planes read over an arm inside the zero
    /// band have `|cos|·arm = arm` there, so the orientation reads Zero,
    /// which has no side: the escalation carries that decided margin,
    /// tagged Zero, and is on the frame's log.
    #[test]
    fn an_orientation_decided_zero_is_the_gates_rejection_on_the_log() {
        use geom_core::k_stats::Bracket;
        let band = band();
        let up = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let down = plane([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]);
        for frac in [0.1, 0.5, 1.0] {
            let arm = frac * band.zero();
            let extent = ConsumedExtent::new(crate::ExtentBall::of_sphere(Point3::origin(), arm));
            for (other, _cos) in [(up, 1.0), (down, -1.0)] {
                let bracket = Bracket::open();
                let got = oriented_plane_eq(&up, &other, &extent, band);
                let log = bracket.finish();
                let what = format!("arm {arm:e}, cos {_cos}");
                let Err(CarrierEqError::Escalated {
                    rung: PlaneRung::Orientation,
                    diag,
                }) = got
                else {
                    panic!("{what}: the orientation reads zero: {got:?}");
                };
                assert_eq!(diag.predicate, Some("plane_eq_orient"), "{what}");
                assert_eq!(diag.margin.rejected_sign(), Some(Sign::Zero), "{what}");
                assert_eq!(
                    log.escalations.iter().map(|e| e.source).collect::<Vec<_>>(),
                    vec![diag],
                    "{what}: the rejection is the one escalation on the log"
                );
            }
        }
    }

    /// **A norm decided negative is the gate's rejection, on the log.**
    /// A norm passes on a zero or positive sign; the parallelism rung's
    /// refusal of a definitely negative one carries the decided margin
    /// tagged Negative, under the rung's own name, and is the one
    /// escalation on the frame's log.
    #[test]
    fn a_norm_decided_negative_is_the_gates_rejection_on_the_log() {
        use geom_core::k_stats::Bracket;
        let band = band();
        for value in [-1.0, -2.0 * band.escalate()] {
            let bracket = Bracket::open();
            let got = unreadable_norm(
                Decided {
                    sign: Sign::Negative,
                    margin: MarginDiag::value(value),
                },
                band,
            );
            let log = bracket.finish();
            let CarrierEqError::Escalated {
                rung: PlaneRung::Norm,
                diag,
            } = got
            else {
                panic!("{value:e}: the norm's own rung: {got:?}");
            };
            assert_eq!(diag.predicate, Some("plane_eq_parallel"), "{value:e}");
            assert_eq!(
                diag.margin.rejected_sign(),
                Some(Sign::Negative),
                "{value:e}"
            );
            assert_eq!(
                log.escalations.iter().map(|e| e.source).collect::<Vec<_>>(),
                vec![diag],
                "{value:e}: the rejection is the one escalation on the log"
            );
        }
    }
}
