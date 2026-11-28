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
//!    [`CarrierRelation::Distinct`]; else the pair's offset
//!    ([`pair_offset`], `plane_eq_offset`) definitely positive ⇒
//!    parallel-but-offset ⇒ `Distinct`.
//! 2. **Zero glues** (D10, Booleans): an offset margin decided Zero is
//!    one plane, `SameOriented` or `SameOpposite` by the decided
//!    orientation, and the decided margin rides out with the verdict
//!    for the caller to record. An offset in
//!    band is a sliver and refuses ([`CarrierEqError::Undecided`]):
//!    near-coincidence never silently becomes contact (F6).
//!
//! **The offset holds no decided sign and no operand order**
//! ([`pair_offset`]). With `dᵢ = nᵢ·(oᵢ − c)` each plane's signed
//! distance from the extent's centre `c` and `fᵢ = c + dᵢ·nᵢ` the
//! centre's foot on plane `i`, the second plane's foot stands
//! `e₁₂ = d₁ − (n₁·n₂)·d₂ = n₁·(f₁ − f₂)` off the first plane, and the
//! first's stands `e₂₁ = d₂ − (n₁·n₂)·d₁` off the second. Each is read
//! at the feet of the region in use and never at a plane's stored
//! origin, which may lie any distance along its plane from that region.
//!
//! The two are one claim to second order: `e₂₁ = −(n₁·n₂)·e₁₂ +
//! ‖n₁ × n₂‖²·d₂`, and the same with the planes exchanged, so their
//! magnitudes differ by no more than `‖n₁ × n₂‖²·max(|d₁|, |d₂|)`,
//! second order in the tilt the parallelism rung has bounded. Either is
//! by itself a reading of the planes' offset. The ladder reads their
//! geometric mean, `√|e₁₂·e₂₁|`, which lies between the two
//! magnitudes: exchanging the planes exchanges the two factors and
//! nothing else, so the margin is the same number, and as an expression
//! it is zero exactly where either factor is. `e₁₂` holds the first
//! normal once and the second twice, `e₂₁` the other way round, so
//! where one normal carries a sign factor `λ` the other does not, one
//! factor is zero given `λ² ≡ 1` and the other with nothing asked of
//! `λ`: the pair is one plane as an expression whichever plane is named
//! first. The orientation rung only labels the verdict.
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
    /// Whether the two planes are parallel over the extent, where the
    /// parallelism margin decided a sign a norm cannot take: negative,
    /// which only a negative lever arm gives (`unreadable_norm`). No
    /// caller's extent reads negative, so it is the kernel's defect, and
    /// no move of the parts reads it.
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
            Self::Norm => {
                "whether two planes are parallel over an extent whose lever arm reads negative"
            }
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
///
/// The verdict and its margins are the same with `p1` and `p2`
/// exchanged, at every scalar. At `f64` the margins are the pose's
/// values to rounding. At `Interval` they enclose every pose the
/// descriptions admit, so a Zero holds for all of them. At a symbolic
/// scalar the offset is zero as an expression where either mirror
/// offset is ([`pair_offset`]), and the orientation is read off the
/// value alone.
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
    let cos = p1.normal.dot(p2.normal);
    let sign_margin = Margin::levered(cos, arm);
    // The orientation this decision makes definite is the relation the
    // offset's Zero glues under, and enters no margin. A zero has no
    // side to read, so it is the gate's rejection.
    let side = decide_nonzero("plane_eq_orient", sign_margin, band).map_err(|diag| {
        CarrierEqError::Escalated {
            rung: PlaneRung::Orientation,
            diag,
        }
    })?;
    let relation = match side {
        NonzeroSign::Positive => CarrierRelation::SameOriented,
        NonzeroSign::Negative => CarrierRelation::SameOpposite,
    };
    let offset_margin = Margin::of(pair_offset(d1, d2, cos));
    match CoincidenceMeasure::decide("plane_eq_offset", offset_margin, band) {
        // The offset is a magnitude: a definite sign is apart.
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

/// **The offset between two near-parallel planes, read from both**
/// (module docs): the geometric mean `√|e₁₂·e₂₁|` of the two mirror
/// offsets, from each plane's signed distance `d1`, `d2` from the
/// region's centre and the normals' cosine `cos`. Never negative, the
/// same number with the planes exchanged, and zero as an expression
/// where either mirror offset is.
pub(crate) fn pair_offset<T: geom_core::Real>(d1: T, d2: T, cos: T) -> T {
    ((d1 - cos * d2) * (d2 - cos * d1)).abs().sqrt()
}

/// The parallelism rung's refusal of a margin decided definitely
/// negative (`norm`, the rung's own reading). `‖n₁ × n₂‖` is never
/// negative, so the sign is the lever arm's: an extent whose radius
/// reads negative, which no caller's extent is. A norm passes on a zero
/// or positive sign, so the refusal is the gate's rejection of that
/// sign, minted and logged by the funnel's door
/// ([`geom_core::k_stats::gate_rejection`]): the decided margin, tagged
/// with the rejected sign. Unreadable input (a NaN) never reaches here:
/// it escalates on the parallelism rung itself.
///
/// # Panics
///
/// Where `norm` is no definite negative verdict on its margin against
/// `band` (the door's own precondition); the ladder hands it only that.
pub(crate) fn unreadable_norm(norm: Decided, band: Band) -> CarrierEqError {
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
mod order_fuzz;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::carrier_eq::fixtures::{OrientationPose, orientation_in_zero_band, plane};
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }
    use geom_core::MarginDiag;

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

    /// The plane `z = 0` against a plane crossing it at the origin,
    /// tilted by `tilt`, its origin stated `far` along itself.
    fn tilted_through_the_centre<T: Decide>(tilt: f64, far: f64) -> [PlaneDesc<T>; 2] {
        let (s, c) = tilt.sin_cos();
        let k = T::from_f64;
        [
            PlaneDesc {
                origin: Point3::new(k(0.0), k(0.0), k(0.0)),
                normal: Vec3::new(k(0.0), k(0.0), k(1.0)),
            },
            PlaneDesc {
                origin: Point3::new(k(far) * k(c), k(0.0), -(k(far) * k(s))),
                normal: Vec3::new(k(s), k(0.0), k(c)),
            },
        ]
    }

    /// The far-origin poses' verdicts at `T`, at the ladder and at the
    /// face-pair door, in both operand orders.
    fn far_origin_verdicts<T: Decide>(band: Band) -> Vec<(String, Option<CarrierRelation>)> {
        use crate::carrier_eq::{CarrierDesc, pair_door_reading};
        let z = band.zero();
        let extent = ConsumedExtent::arm(T::from_f64(1.0));
        let mut got = Vec::new();
        for (tilt, far) in [
            (0.2 * z, 0.0),
            (0.2 * z, 30.0),
            (0.2 * z, 1e3),
            (1e-4 * z, 1e5),
        ] {
            let [a, b] = tilted_through_the_centre::<T>(tilt, far);
            for (order, x, y) in [("as stated", a, b), ("exchanged", b, a)] {
                let what = format!("tilt {tilt:e}, origin {far} away, {order}");
                let ladder = plane_ladder(&x, &y, &extent, band);
                got.push((
                    format!("{what}, the ladder: {ladder:?}"),
                    ladder
                        .ok()
                        .and_then(CarrierVerdict::one)
                        .map(|(rel, _)| rel),
                ));
                let door = pair_door_reading(
                    &CarrierDesc::Plane(x),
                    &CarrierDesc::Plane(y),
                    &extent,
                    band,
                );
                got.push((
                    format!("{what}, the pair door: {door:?}"),
                    door.ok()
                        .and_then(|read| read.verdict().one())
                        .map(|(rel, _)| rel),
                ));
            }
        }
        got
    }

    /// **Two planes that are one plane over the region are one plane
    /// wherever an origin is stated.** The planes cross at the extent's
    /// centre, tilted a fifth of the zero band at its arm, and the
    /// second's origin is stated on it up to a thousand arms away (and
    /// `1e5` arms at a tilt of `1e-4` of the band): one plane at every
    /// distance, in either operand order, at `f64` and at `Interval`, at
    /// the ladder and at the face-pair door. Red where the offset is read
    /// at a stored origin (`n₁·(o₂ − o₁)`): the tilt is then levered at
    /// the origin's distance, undecided at 30 arms and apart at `1e3`.
    #[test]
    fn planes_tilted_in_band_are_one_plane_wherever_an_origin_is_stated() {
        let band = band();
        for (scalar, got) in [
            ("f64", far_origin_verdicts::<f64>(band)),
            ("Interval", far_origin_verdicts::<geom_core::Interval>(band)),
        ] {
            assert_eq!(got.len(), 16, "{scalar}");
            for (what, relation) in got {
                assert_eq!(
                    relation,
                    Some(CarrierRelation::SameOriented),
                    "{scalar}: {what}"
                );
            }
        }
    }

    /// Two walls on the plane `y = −h/2` facing opposite ways, each
    /// normal spelled as `spelling` gives it from the side it was built
    /// along: a literal axis, or the side's chord over its own length
    /// (`w/√w²`, `v/√v²`), so one plane's normal carries a sign factor
    /// the other's does not.
    fn walls_spelled_two_ways<T: Decide>([w, v, h]: [T; 3], literal: bool) -> [PlaneDesc<T>; 2] {
        let k = T::from_f64;
        let y = -(h / k(2.0));
        let down = if literal {
            k(-1.0)
        } else {
            -(w / (w * w).sqrt())
        };
        [
            PlaneDesc {
                origin: Point3::new(k(0.0), y, k(0.0)),
                normal: Vec3::new(k(0.0), down, k(0.0)),
            },
            PlaneDesc {
                origin: Point3::new(v, y, k(0.0)),
                normal: Vec3::new(k(0.0), v / (v * v).sqrt(), k(0.0)),
            },
        ]
    }

    /// **One plane is one plane as an expression whichever way each
    /// normal is spelled.** A wall whose normal is a literal axis beside
    /// one whose normal is its side's chord over its length, and two
    /// walls each spelled through its own side, in either operand order:
    /// at the ladder and at the face-pair door the coincidence's margin
    /// discharges as a theorem at `Sym<f64>`, and over a box at
    /// `Sym<Interval>`, where the enclosures are a fifth of a metre wide,
    /// the pair is one plane all the same. Red where a decided
    /// orientation sign multiplies a normal (`d₁ − σ·d₂`, or the swing
    /// `|n₁ − σ·n₂|`): the Zero then needs `v/√v² ≡ 1`.
    #[test]
    fn one_plane_spelled_with_two_signs_is_one_plane_as_an_expression() {
        use crate::carrier_eq::{CarrierDesc, pair_door_reading};
        use geom_core::sym::deferred::{discharge, with_deferred_session};
        use geom_core::{Interval, ParamSymbol, Real, Sym, SymBudget, SymRetry, SymRules};
        let band = band();
        let budget = SymBudget {
            max_terms: 256,
            max_degree: 64,
        };
        let symbol = |name: &str| ParamSymbol::new(test_utils::symbol_id(name));
        fn ordered<T: Decide>([a, b]: [PlaneDesc<T>; 2], exchanged: bool) -> [PlaneDesc<T>; 2] {
            if exchanged { [b, a] } else { [a, b] }
        }
        for literal in [true, false] {
            for exchanged in [false, true] {
                let what = format!("literal axis {literal}, exchanged {exchanged}");
                let (proved, _) =
                    with_deferred_session(budget, SymRules::shipped(), SymRetry::none(), || {
                        type S = Sym<f64>;
                        let at = |name, value| S::param(symbol(name), value);
                        let [a, b] = ordered(
                            walls_spelled_two_ways(
                                [at("w", 2.0), at("v", 2.0), at("h", 1.0)],
                                literal,
                            ),
                            exchanged,
                        );
                        let extent = ConsumedExtent::arm(S::from_f64(1.0));
                        let ladder = plane_ladder(&a, &b, &extent, band)
                            .ok()
                            .and_then(CarrierVerdict::one);
                        let door = pair_door_reading(
                            &CarrierDesc::Plane(a),
                            &CarrierDesc::Plane(b),
                            &extent,
                            band,
                        )
                        .ok()
                        .and_then(|read| read.verdict().one());
                        [ladder, door].map(|one| one.map(|(rel, margin)| (rel, discharge(margin))))
                    });
                assert_eq!(
                    proved,
                    [Some((CarrierRelation::SameOpposite, true)); 2],
                    "{what}: the ladder's and the pair door's margins discharge"
                );
                let (over_a_box, _) = geom_core::sym::with_session(budget, || {
                    type S = Sym<Interval>;
                    let at = |name, lo, hi| S::param(symbol(name), Interval::from_bounds(lo, hi));
                    let [a, b] = ordered(
                        walls_spelled_two_ways(
                            [at("w", 1.9, 2.1), at("v", 1.9, 2.1), at("h", 0.9, 1.1)],
                            literal,
                        ),
                        exchanged,
                    );
                    let extent = ConsumedExtent::arm(S::from_f64(1.0));
                    let ladder = plane_ladder(&a, &b, &extent, band).map(CarrierVerdict::relation);
                    let door = pair_door_reading(
                        &CarrierDesc::Plane(a),
                        &CarrierDesc::Plane(b),
                        &extent,
                        band,
                    )
                    .map(|read| read.verdict().relation());
                    [ladder.ok(), door.ok()]
                });
                assert_eq!(
                    over_a_box,
                    [Some(CarrierRelation::SameOpposite); 2],
                    "{what}: one plane over the box"
                );
            }
        }
    }

    /// The second wall's sign factor `λ`, positive at its value, spelled
    /// the `spelling`th way over the variables `x`: a side over its own
    /// length for sides of growing size (0 to 4, 7 to 9), a chord's unit
    /// in space (5), and the unit of two chords' cross product (6).
    fn sign_factor<T: Decide>(spelling: usize, x: &[T]) -> T {
        let k = T::from_f64;
        let sum =
            |range: std::ops::Range<usize>| x[range].iter().copied().fold(k(0.0), |a, b| a + b);
        let unitise = |v: T| v / (v * v).sqrt();
        match spelling {
            0 => unitise(x[0]),
            1 => unitise(x[0] - x[1] + k(1.0)),
            2 => unitise(x[0] * x[1] - x[2] * x[3] + k(5.0)),
            3 => unitise(sum(0..4) * sum(4..8) - sum(0..3) * sum(3..6)),
            4 => unitise(sum(0..8) * sum(0..8) * sum(0..4) - sum(0..5)),
            5 => {
                // A component zero as a formula, not as a literal.
                let (a, b) = (x[0], x[0]);
                let v = Vec3::new(a - b, k(0.0), sum(0..4) - sum(4..6));
                v.z / v.norm()
            }
            6 => {
                let u = Vec3::new(sum(0..3), x[3], k(0.0));
                let w = Vec3::new(x[4], sum(5..8), k(0.0));
                let n = u.cross(w);
                n.z / n.norm()
            }
            7 => unitise(sum(0..24) - k(20.0)),
            8 => unitise(sum(0..4) * sum(0..4) - sum(0..3)),
            _ => unitise(sum(0..4) * sum(4..8) * sum(0..2) - sum(0..5)),
        }
    }

    /// Whether the ladder's and the pair door's coincidences discharge
    /// for a wall whose normal is a literal axis against one whose
    /// normal is that axis times [`sign_factor`]'s `spelling`, on one
    /// plane as formulas, at `max_terms`, the walls `exchanged` or not.
    fn spelled_pair_proves(
        max_terms: usize,
        spelling: usize,
        exchanged: bool,
    ) -> [Option<bool>; 2] {
        use crate::carrier_eq::{CarrierDesc, pair_door_reading};
        use geom_core::sym::deferred::{discharge, with_deferred_session};
        use geom_core::{ParamSymbol, Real, Sym, SymBudget, SymRetry, SymRules};
        type S = Sym<f64>;
        let budget = SymBudget {
            max_terms,
            max_degree: 64,
        };
        let band = band();
        with_deferred_session(budget, SymRules::shipped(), SymRetry::none(), || {
            let k = S::from_f64;
            let at =
                |name: &str, value| S::param(ParamSymbol::new(test_utils::symbol_id(name)), value);
            let x: Vec<S> = (0..24)
                .map(|i| at(&format!("x{i}"), 1.0 + 0.1 * f64::from(i)))
                .collect();
            let y = -(at("h", 1.0) / k(2.0));
            let literal = PlaneDesc {
                origin: Point3::new(k(0.0), k(0.0), y),
                normal: Vec3::new(k(0.0), k(0.0), k(-1.0)),
            };
            let spelled = PlaneDesc {
                origin: Point3::new(x[7], k(0.0), y),
                normal: Vec3::new(k(0.0), k(0.0), sign_factor(spelling, &x)),
            };
            let (a, b) = if exchanged {
                (spelled, literal)
            } else {
                (literal, spelled)
            };
            let extent = ConsumedExtent::arm(k(1.0));
            let ladder = plane_ladder(&a, &b, &extent, band)
                .ok()
                .and_then(CarrierVerdict::one);
            let door = pair_door_reading(
                &CarrierDesc::Plane(a),
                &CarrierDesc::Plane(b),
                &extent,
                band,
            )
            .ok()
            .and_then(|read| read.verdict().one());
            [ladder, door].map(|one| one.map(|(_, margin)| discharge(margin)))
        })
        .0
    }

    /// **Whether a pair is proven one plane does not follow the order
    /// its planes are named in.** A literal-axis wall against a wall
    /// whose normal carries a sign factor spelled ten ways, on one plane
    /// as formulas: at the ladder and at the pair door the coincidence
    /// discharges alike in both orders, at 256 terms and at 4096, and
    /// every spelling is proven. Red where the offset is read in one
    /// order only: a spelling whose factor's square does not fold to one
    /// (the first is spelling 4) then proves in one order and not the
    /// other.
    #[test]
    fn a_sign_factor_spelled_ten_ways_proves_alike_in_either_order() {
        for (max_terms, spelling) in (0..10)
            .map(|s| (256, s))
            .chain([4, 7, 8, 9].map(|s| (4096, s)))
        {
            let stated = spelled_pair_proves(max_terms, spelling, false);
            let exchanged = spelled_pair_proves(max_terms, spelling, true);
            println!("{max_terms} terms, spelling {spelling}: {stated:?} / {exchanged:?}");
            assert_eq!(
                stated, exchanged,
                "{max_terms} terms, spelling {spelling}: the ladder's and the door's proofs, \
                 stated and exchanged"
            );
            assert_eq!(
                stated,
                [Some(true); 2],
                "{max_terms} terms, spelling {spelling}: one plane as an expression"
            );
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
                sign: Sign::Positive,
                margin: MarginDiag::value(4.0),
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
    /// band (`fixtures::orientation_in_zero_band`) have `|cos|·arm = arm`
    /// there, so the orientation reads Zero, which has no side: the
    /// escalation carries that decided margin, `cos · arm`, tagged Zero,
    /// and is the one escalation on the frame's log.
    #[test]
    fn an_orientation_decided_zero_is_the_gates_rejection_on_the_log() {
        use geom_core::k_stats::Bracket;
        let band = band();
        for OrientationPose {
            what,
            up,
            other,
            extent,
            cos_arm,
        } in orientation_in_zero_band(band)
        {
            let bracket = Bracket::open();
            let got = oriented_plane_eq(&up, &other, &extent, band);
            let log = bracket.finish();
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
                diag.margin.diagnostic_f64_for_error_text().value(),
                Some(cos_arm),
                "{what}"
            );
            assert_eq!(
                log.escalations.iter().map(|e| e.source).collect::<Vec<_>>(),
                vec![diag],
                "{what}: the rejection is the one escalation on the log"
            );
        }
    }

    /// **A parallelism read over a negative arm is the norm gate's
    /// rejection, on the log.** Two perpendicular planes over an extent
    /// whose radius is `-1`: the parallelism margin `‖n₁ × n₂‖ · arm` is
    /// `-1`, a sign a norm cannot take, so the ladder refuses on the
    /// `Norm` rung with that decided margin tagged Negative, under
    /// `plane_eq_parallel`, the one escalation on the frame's log.
    #[test]
    fn a_parallelism_over_a_negative_arm_is_the_norm_gates_rejection_on_the_log() {
        use geom_core::k_stats::Bracket;
        let band = band();
        let flat = plane([0.0; 3], [0.0, 0.0, 1.0]);
        let wall = plane([0.0; 3], [1.0, 0.0, 0.0]);
        for (x, y) in [(flat, wall), (wall, flat)] {
            let bracket = Bracket::open();
            let got = oriented_plane_eq(&x, &y, &ConsumedExtent::arm(-1.0), band);
            let log = bracket.finish();
            let Err(CarrierEqError::Escalated {
                rung: PlaneRung::Norm,
                diag,
            }) = got
            else {
                panic!("the norm's own rung: {got:?}");
            };
            assert_eq!(diag.predicate, Some("plane_eq_parallel"));
            assert_eq!(diag.margin.rejected_sign(), Some(Sign::Negative));
            assert_eq!(
                diag.margin.diagnostic_f64_for_error_text().value(),
                Some(-1.0)
            );
            assert!(!diag.terminal_sliver);
            assert_eq!(
                log.escalations.iter().map(|e| e.source).collect::<Vec<_>>(),
                vec![diag],
                "the rejection is the one escalation on the log"
            );
        }
    }
}
