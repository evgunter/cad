//! **Contact verification** — the C4 per-class tables, executable.
//!
//! Each door reads a face pair's geometry as the glue door reads it
//! and returns a verdict: the class holds, on its margin; a definite
//! "no", on the row that decided it and its reading; or in band. Each
//! class states two lists, and this module is where each becomes code:
//!
//! - *must-verify-DEFINITE* — the geometry must definitely satisfy
//!   these;
//! - *rulings* — definite counter-evidence, a typed row.
//!
//! An in-band margin on either list escalates: nothing is bridged. A
//! ruling is a verdict, not a contradiction: only a record at rest is
//! contradicted by one (`ValidationError::ContactContradicted`).
//!
//! `Rest` goes down the carrier ladder ([`rest_pair_verdict`],
//! [`mod@super::carrier_eq`]), `Tangent` down the jet schedule
//! ([`tangent_pair_relation`]).
//!
//! **One `Tangent` table, two readings** (`tangent_table`): a contact
//! claims opposed senses and a void crescent ([`tangent_locus_relation`]);
//! the glue door's tangency ([`tangency_reading`]) claims neither. Both
//! read the second-order margin `κ_rel` as the glue door does: it must
//! be definite (jet-determinate), and in band, or exactly zero at an
//! isolated point, it escalates. The #175 clause, which bridged an exact
//! zero so that a G1 tube chain's two neutral meridian points stayed
//! declarable, binds nothing now: no contact is declared, and the glue
//! door refuses such a chain at those points, as D10 has it
//! (`mate7a_torus_rest::the_g1_tube_chain_refuses_at_the_glue_door`).
//!
//! **The Tangent lane reuses the jet machinery, it does not restate
//! it.** `tangent_jet`, `tangent_certificate_lane`,
//! `tangent_span_bounds` and the certification schedule's own
//! stations (`schedule_param`) all come from `geom_brep` — the
//! demanded set IS the certifiable set, which is C3's order-k boundary
//! as a fact about the code rather than a promise about it. Outside the lane, this
//! reads [`TangentVerdict::NotCertifiable`]: an order-k > 1 contact
//! has no record type, so it gets no verdict either.
//!
//! **What this lane does NOT check, named** (C4's "definite crossing
//! (II_rel definitely indefinite)"): a crossing at first order, the
//! surfaces running into each other's material at a sample, is the
//! touch's negative side (`touching`), and a crossing between two
//! tangent samples whose normals agree at one and oppose at the other
//! is [`TangentRuling::SidesMixed`]; a crossing at second order is
//! decided only through the transverse `κ_rel` the jet computes along
//! the locus tangent's perpendicular. A full second-fundamental-form
//! indefiniteness test over all tangent directions needs machinery
//! that does not exist yet. The honest consequence: a tangency whose
//! relative shape operator is indefinite only in a direction the jet
//! does not sample passes every arm this lane runs. The list of rulings
//! is narrower than C4's, and a configuration outside the sampled
//! direction is missed.

use geom::Curve3;
use geom_brep::{
    CERT_SAMPLES, PairSide, implicit_outward_normal, schedule_param, tangent_certificate_lane,
    tangent_jet, tangent_span_bounds,
};
use geom_core::k_stats::Magnitude;
use geom_core::{Band, Decide, Decided, Indeterminate, Margin, Sign, Vec3};

use crate::body::Body;
use crate::contact::{
    ContactRefusal, RestRow, RestRuling, RestVerdict, TangentRow, TangentRuling, TangentVerdict,
    Uncertified,
};
use crate::entity::FaceKey;

use super::carrier_eq::{CarrierEqError, CarrierRelation, CarrierVerdict};

/// **The `Rest` contact door**: do these two faces rest on one
/// carrier, opposed? The table (C4): one carrier through the face-pair
/// door's reading ([`super::carrier_pair_decided`]), plus opposed
/// senses. [`RestVerdict::Rest`] carries the margin the carrier was
/// decided one on.
///
/// **Aligned senses rule it out** ([`RestRow::SensesAligned`]). Two
/// faces on one carrier whose material sides agree are not in contact:
/// they are the same material, or one contains the other (the C1 lemma).
/// The carrier ladder reports that honestly as `SameOriented`, a
/// legitimate and exercised answer at the classification/merge site
/// (flush walls); only a claim of contact rules it out. Two carriers
/// ([`RestRow::TwoCarriers`]) carry the datum decided apart where one
/// was; a sense is a bit, and two kinds decide no datum.
///
/// # Errors
///
/// [`ContactRefusal`] — escalation, or a configuration outside the
/// class's certifiable set.
pub fn rest_pair_verdict<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<RestVerdict, ContactRefusal> {
    let outcome =
        super::carrier_pair::carrier_pair_decided(a, fa, b, fb, band).map_err(|unread| {
            ContactRefusal::NotCertifiable {
                what: match unread {
                    super::carrier_pair::PairUnread::OutsideInventory => {
                        super::carrier_eq::OUTSIDE_REST_INVENTORY
                    }
                    super::carrier_pair::PairUnread::Extent(_) => {
                        "a recorded face's consumed extent cannot be read (its box has no claim to \
                     make, or its boundary cannot be walked)"
                    }
                },
            }
        })?;
    let ruled = |row, by| Ok(RestVerdict::Ruled(RestRuling { row, by }));
    match outcome {
        Ok(CarrierVerdict::One {
            relation: CarrierRelation::SameOpposite,
            margin,
        }) => Ok(RestVerdict::Rest(margin)),
        Ok(CarrierVerdict::One {
            relation: CarrierRelation::SameOriented,
            ..
        }) => ruled(RestRow::SensesAligned, None),
        Ok(CarrierVerdict::Apart(decided)) => ruled(RestRow::TwoCarriers, Some(decided)),
        Ok(
            CarrierVerdict::TwoKinds
            | CarrierVerdict::One {
                relation: CarrierRelation::Distinct,
                ..
            },
        ) => ruled(RestRow::TwoCarriers, None),
        Err(CarrierEqError::Escalated { diag, .. }) => Err(ContactRefusal::Escalated { diag }),
        Err(CarrierEqError::Undecided { coincidence, .. }) => Err(ContactRefusal::Escalated {
            diag: coincidence.reported(),
        }),
        // Every caller reads a validated body, and tier 3 refuses such a
        // torus first (`ValidationError::DegenerateTorus`), so no row
        // reaches this arm through one.
        Err(CarrierEqError::OffRing { refusal, .. }) => Err(match refusal.verdict {
            geom_brep::TorusVerdict::Undecided(diag) => ContactRefusal::Escalated { diag },
            geom_brep::TorusVerdict::Refused(_) => ContactRefusal::NotCertifiable {
                what: "a recorded face's torus is outside the ring convention (a horn or \
                       spindle torus, or one with no tube), which no carrier is compared on",
            },
        }),
    }
}

/// **The Tangent verify door** — C4's `Tangent` table run over the
/// jet schedule.
///
/// Per sample on the certification schedule (`geom_brep::schedule_param`:
/// the nine stations, and the finer schedules a span past ε is read
/// again on, [`tangent_table`]):
///
/// - **must-verify-DEFINITE**: the two surfaces touch, their
///   separation along the shared normal within ε (`touching`, and the
///   span line after the loop); the witness lies within ε of each
///   surface (`on_surface`); and the two outward normals are parallel
///   within the derived angle at lever arm `1/κ_rel`, the C7 threshold
///   verbatim, and definitely OPPOSED;
/// - **rules out** ([`TangentVerdict::Ruled`]): definite separation or
///   definite crossing (the separation definitely positive or negative),
///   a bad witness (a residual definitely nonzero where the pair
///   touches), definite normal independence (`sin θ` definitely nonzero
///   at that arm, or the normals perpendicular), or aligned normals at
///   a tangent sample (containment, not tangency);
/// - **is in band**: an in-band margin on ANY arm (the touch at a
///   sample, a residual, the senses, first-order parallelism, the
///   second-order `κ_rel`, in band or exactly zero), and a span bound
///   still in band on the finest schedule;
/// - **is not certifiable**: a span bound still beyond the band on the
///   finest schedule, every other reading decided.
///
/// The span bounds between samples come from the same
/// `tangent_span_bounds` the edge certifier uses, so the hull
/// discipline is shared rather than mirrored. [`TangentVerdict::Touch`]
/// carries the touch's margin ([`tangent_locus_relation`]).
#[allow(clippy::too_many_arguments)]
pub fn tangent_pair_relation<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
    band: Band,
) -> Result<TangentVerdict, ContactRefusal> {
    // This is a public door, so a stale face key is the caller's input
    // and refuses typed.
    let (face_a, face_b) = (
        a.face(fa).ok_or(ContactRefusal::NotCertifiable {
            what: "the A-side face does not resolve",
        })?,
        b.face(fb).ok_or(ContactRefusal::NotCertifiable {
            what: "the B-side face does not resolve",
        })?,
    );
    Ok(tangent_locus_relation(
        face_a.surface(),
        face_a.sense(),
        face_b.surface(),
        face_b.sense(),
        carrier,
        t0,
        t1,
        band,
    ))
}

/// [`tangent_pair_relation`]'s body, at the SURFACE level: the two
/// oriented surfaces (each with its face's material-side bit), the
/// locus and its parameter range.
///
/// The face-level door resolves keys and calls this; the geometry is
/// all here, so a fixture can drive the C4 table on raw surfaces
/// without building a body around a configuration whose whole point is
/// the surface pair. One ladder, two entry points.
///
/// A CONTACT claims two materials touching along the locus, so where
/// `κ_rel` is definitely nonzero its sign is read against the material
/// sides too ([`geom_brep::material_kappa_rel`]): the two sheets must
/// leave the locus apart, the gap between them void, never crossing so
/// that their materials overlap (an internal tangency whose senses put
/// both materials in the crescent between the walls). A definite
/// overlap rules it out ([`TangentRow::Overlap`]).
/// [`TangentVerdict::Touch`] carries the margin the touch was decided
/// on: the span line's reading of the pair's separation
/// (`contact_tangent_touch`), the margin a verified pair's row records.
#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn tangent_locus_relation<T: Decide>(
    s1: &geom::Surface<T>,
    sense1: bool,
    s2: &geom::Surface<T>,
    sense2: bool,
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
    band: Band,
) -> TangentVerdict {
    tangent_table(
        s1,
        sense1,
        s2,
        sense2,
        carrier,
        t0,
        t1,
        Claim::Contact,
        band,
    )
}

/// **The boolean glue door's tangency**: the C4 `Tangent` table along
/// `carrier` with no claim about the materials. It keeps the touch, the
/// witness on both surfaces, first-order parallelism and the
/// second-order margin, which must be DEFINITE (jet-determinate). The
/// outward normals may agree or oppose, so long as they do one along
/// the whole locus, and the crescent between the sheets may be void or
/// shared.
///
/// The touch is read at every sample of the held locus, the residual
/// every carrier owes however its pair decided its tangency: a pair's
/// pose decides whether the surfaces are tangent and where
/// (`geom_brep::pair_section::gap`), not that the held rim stands within
/// ε of both all the way round, so a rim of a pair its pose decides
/// tangent can still escalate here.
#[allow(clippy::too_many_arguments)]
pub(crate) fn tangency_reading<T: Decide>(
    s1: &geom::Surface<T>,
    sense1: bool,
    s2: &geom::Surface<T>,
    sense2: bool,
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
    band: Band,
) -> TangentVerdict {
    tangent_table(
        s1,
        sense1,
        s2,
        sense2,
        carrier,
        t0,
        t1,
        Claim::Tangency,
        band,
    )
}

/// What the table is asked to verify.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Claim {
    /// A contact: senses opposed and the crescent void (C4's `Tangent`).
    Contact,
    /// A tangency alone, claiming nothing of the materials.
    Tangency,
}

/// The C4 `Tangent` table along `carrier` under `claim`.
///
/// A pass whose only unsettled reading is a span bound ([`Unsettled::Sag`])
/// is read again on a schedule twice as fine, its sag a quarter as
/// large, up to [`REFINE_HALVINGS`] times; the passes it supersedes
/// keep their verdicts and drop their escalations
/// (`k_stats::splice_superseded`). Past that the bound refuses: an
/// in-band one is that reading, and one beyond the band is the
/// certificate not reaching the span ([`Uncertified::SagUnreached`]).
#[allow(clippy::too_many_arguments)]
fn tangent_table<T: Decide>(
    s1: &geom::Surface<T>,
    sense1: bool,
    s2: &geom::Surface<T>,
    sense2: bool,
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
    claim: Claim,
    band: Band,
) -> TangentVerdict {
    // The lane gate FIRST: the demanded set is the certifiable set,
    // so a configuration whose span bounds do not exist gets no
    // per-sample verdict either — refusing here is the order-k
    // boundary, not a missing feature.
    if !tangent_certificate_lane(carrier, s1, s2) {
        return TangentVerdict::NotCertifiable(Uncertified::OutsideLane);
    }
    // The locus's extent from above, capping every lever arm the
    // schedule uses. Without it a planar carrier's curvature lever arm
    // is `f64::MAX` and every angular margin reads definite at a lever
    // no configuration is consumed over; from above, because the
    // parallel reading's Zero is what verifies the contact. (The edge
    // certifier levers the same pair at its faces' extent; this table
    // has no faces.)
    let extent = geom_brep::edge_extent_sup(carrier, t0, t1, (carrier.eval(t0), carrier.eval(t1)));
    let mut halvings = 0;
    loop {
        let samples = (CERT_SAMPLES - 1) * (1 << halvings) + 1;
        // Both arms of the span bounds read the carrier and the step and
        // never where the span stands on it, so the schedule's first
        // span bounds every one.
        let span_end = schedule_param(t0, t1, CERT_SAMPLES - 1, samples);
        let Some(bounds) = tangent_span_bounds(s1, s2, carrier, t0, span_end) else {
            return TangentVerdict::NotCertifiable(Uncertified::NoSpanBounds);
        };
        let mut sag_logs = Vec::new();
        let (pass, log) = geom_core::k_stats::detached(|| {
            tangent_pass(
                (s1, sense1),
                (s2, sense2),
                (carrier, t0, t1, samples),
                &bounds,
                extent,
                claim,
                band,
                &mut sag_logs,
            )
        });
        // The span bounds' own runs: what a sag-only answer rests on,
        // and nothing a refusal or a finer pass does.
        let settles = matches!(pass, Err(Unsettled::Sag(_))) && halvings == REFINE_HALVINGS;
        for sag_log in sag_logs {
            if settles {
                geom_core::k_stats::splice(sag_log);
            } else {
                geom_core::k_stats::splice_superseded(sag_log);
            }
        }
        match pass {
            Err(Unsettled::Sag(_)) if halvings < REFINE_HALVINGS => {
                geom_core::k_stats::splice_superseded(log);
                halvings += 1;
            }
            Err(Unsettled::Sag(reading)) => {
                geom_core::k_stats::splice(log);
                return reading.map_or(
                    TangentVerdict::NotCertifiable(Uncertified::SagUnreached),
                    TangentVerdict::InBand,
                );
            }
            Err(Unsettled::Ruled(ruling)) => {
                geom_core::k_stats::splice(log);
                return TangentVerdict::Ruled(ruling);
            }
            Err(Unsettled::InBand(diag)) => {
                geom_core::k_stats::splice(log);
                return TangentVerdict::InBand(diag);
            }
            Ok(read) => {
                geom_core::k_stats::splice(log);
                return TangentVerdict::Touch(read);
            }
        }
    }
}

/// How many times [`tangent_table`] halves its schedule's step before a
/// span bound refuses: sixty-four spans to each of the nine-sample
/// schedule's, the sag down by 4096.
pub const REFINE_HALVINGS: u32 = 6;

/// Why a pass of the table did not settle.
enum Unsettled {
    /// A ruling no finer schedule changes.
    Ruled(TangentRuling),
    /// An in-band reading no finer schedule changes.
    InBand(Indeterminate),
    /// Every sample decided, and a span bound between them not within
    /// ε: its reading where it was in band, `None` where beyond.
    Sag(Option<Indeterminate>),
}

/// [`TangentRuling::Row`] on `row`, as a pass ends.
fn ruled(row: TangentRow, by: Decided) -> Unsettled {
    Unsettled::Ruled(TangentRuling::Row { row, by })
}

/// The row the jet's second-order margin `κ_rel` is decided on.
const SECOND_ORDER: &str = "contact_tangent_second_order";

/// One pass of [`tangent_table`] on a uniform `samples`-point schedule
/// over `[t0, t1]`, `bounds` its span bounds.
#[allow(clippy::too_many_arguments)]
fn tangent_pass<T: Decide>(
    (s1, sense1): (&geom::Surface<T>, bool),
    (s2, sense2): (&geom::Surface<T>, bool),
    (carrier, t0, t1, samples): (&Curve3<T>, T, T, u32),
    bounds: &geom_brep::TangentSpanBounds<T>,
    extent: geom_core::Lever<T>,
    claim: Claim,
    band: Band,
    sag_logs: &mut Vec<geom_core::k_stats::Detached>,
) -> Result<geom_core::MarginDiag, Unsettled> {
    let mut gap_max = T::zero();
    // The first tangent sample's senses: whether its outward normals
    // agree, and the reading that decided it.
    let mut sides: Option<(bool, Decided)> = None;
    // A span bound left unsettled is the pass's answer only where every
    // other reading on every sample decides: a later verdict or
    // escalation answers instead. An in-band bound's reading stands
    // before one beyond the band.
    let mut sag: Option<Option<Indeterminate>> = None;
    let mut unsettled = |reading: Option<Indeterminate>| {
        if !matches!(sag, Some(Some(_))) {
            sag = Some(reading.or(sag.flatten()));
        }
    };
    for i in 0..samples {
        let t = schedule_param(t0, t1, i, samples);
        let (p, tau) = carrier.ders1(t);
        let (r1, r2) = (
            geom_brep::implicit_residual(s1, p),
            geom_brep::implicit_residual(s2, p),
        );
        // (1) Which way the two OUTWARD normals point, metered over the
        // extent as the edge certifier reads it
        // (`tangent_gap_orientation`): a surface curving on a scale near
        // ε still has an orientation, and its refusal is the second-order
        // arm's, below. Perpendicular normals are definite normal
        // INDEPENDENCE, C4's first trigger; whether they agree or oppose
        // is read against the claim once the sample reads tangent (3).
        let n1: Vec3<T> = implicit_outward_normal(s1, sense1, p).vec();
        let n2: Vec3<T> = implicit_outward_normal(s2, sense2, p).vec();
        let arm = geom_brep::folded_lever_arm(s1, s2, p, extent);
        let senses = match crate::validate::decide_reported(
            "contact_tangent_opposed",
            Margin::levered(n1.dot(n2), extent),
            band,
        ) {
            Ok(
                by @ Decided {
                    sign: Sign::Zero, ..
                },
            ) => return Err(ruled(TangentRow::Independent, by)),
            Ok(by) => by,
            Err(diag) => return Err(Unsettled::InBand(diag)),
        };
        let agree = senses.sign == Sign::Positive;
        // (2) The touch, before the witness: the pair's separation
        // along the shared normal ([`touching`]), its orientation σ the
        // one (1) decided, the second face read as reversed where the
        // normals agree so the touch reads one convention.
        let separation = touching(r1, sense1, r2, sense2 != agree, band)?;
        gap_max = gap_max.max(separation.abs());
        // The witness on both surfaces, read across the spans each
        // sample bounds ([`on_surface`]).
        for (side, r) in [(PairSide::First, r1), (PairSide::Second, r2)] {
            let on = match side {
                PairSide::First => CONTACT_TANGENT_ON_1,
                PairSide::Second => CONTACT_TANGENT_ON_2,
            };
            let (read, log) = geom_core::k_stats::detached(|| {
                on_surface((on, side), r, bounds.residual_sag, band)
            });
            match read {
                Err(Unsettled::Sag(reading)) => {
                    sag_logs.push(log);
                    unsettled(reading);
                }
                read => {
                    geom_core::k_stats::splice(log);
                    read?;
                }
            }
        }
        let jet = tangent_jet(s1, s2, p, tau);
        // (3) and (4) are decided TOGETHER because C7's first-order
        // threshold is derived FROM the second-order margin: the lever
        // arm of the normal-parallelism test is 1/κ_rel, so whether
        // that lever exists is exactly the second-order question. The
        // second-order margin is therefore computed first and ACTED ON
        // last — computing it early must not let it refuse before the
        // first-order test (which can rule the pair out) has spoken.
        let so = Margin::sagitta(jet.kappa_rel.abs() - bounds.kappa_drift, arm.length());
        // Both readings run detached so the second-order one's
        // escalation reaches the frame's log only where the outcome
        // rests on it (`k_stats::splice_superseded`, below).
        let (second_order, second_order_log) = geom_core::k_stats::detached(|| {
            crate::validate::decide_reported(SECOND_ORDER, so, band)
        });
        let positive = matches!(
            second_order,
            Ok(Decided {
                sign: Sign::Positive,
                ..
            })
        );
        // (3) First-order tangency. With a definitely positive κ_rel
        // (an exact zero is the #175 neutral point: definite, and NOT a
        // usable lever) the lever is C7's own 1/κ_rel (`levered_inv`, verbatim what the edge
        // certifier uses). Without one the derived angle is unbounded,
        // which would make the test vacuous, so the margin falls back
        // to the folded arm (`geom_brep::folded_parallelism`, the one
        // spelling the edge certifier's namings read too). Here it is
        // the first-order verdict itself, so an in-band reading
        // escalates; there it only names a refusal already made.
        let parallel = if positive {
            Margin::levered_inv(jet.sin_theta, jet.kappa_rel.abs())
        } else {
            geom_brep::folded_parallelism(jet.sin_theta, arm)
        };
        let (first_order, first_order_log) = geom_core::k_stats::detached(|| {
            crate::validate::decide_reported("contact_tangent_parallel", parallel, band)
        });
        let first_zero = matches!(
            first_order,
            Ok(Decided {
                sign: Sign::Zero,
                ..
            })
        );
        // An in-band second-order reading is overruled when the
        // first-order reading refuses on its own: a definite defect
        // rules the pair out, and an in-band one is the escalation
        // returned. Only a verified first order leaves the outcome
        // resting on it. With `κ_rel` decided at the sample and only its
        // drift bound between samples keeping the margin from deciding,
        // the second order is the schedule's to settle, as the witness's
        // sag is (its run kept with the sag's).
        let deferred = !positive && first_zero && {
            let (at, log) = geom_core::k_stats::detached(|| {
                crate::validate::decide(
                    "contact_tangent_second_order_at",
                    Margin::sagitta(jet.kappa_rel.abs(), arm.length()),
                    band,
                )
            });
            geom_core::k_stats::splice_superseded(log);
            matches!(at, Ok(Sign::Positive))
        };
        if deferred {
            sag_logs.push(second_order_log);
        } else if second_order.is_err() && !first_zero {
            geom_core::k_stats::splice_superseded(second_order_log);
        } else {
            geom_core::k_stats::splice(second_order_log);
        }
        geom_core::k_stats::splice(first_order_log);
        match first_order {
            Ok(
                by @ Decided {
                    sign: Sign::Positive | Sign::Negative,
                    ..
                },
            ) => return Err(ruled(TangentRow::Parallel, by)),
            Ok(_) => {}
            // First-order tangency is must-verify-DEFINITE (C4's first
            // list): an in-band angular defect escalates.
            Err(diag) => return Err(Unsettled::InBand(diag)),
        }
        // (3′) The senses, at a sample read tangent. A contact's outward
        // normals must point at each other, since aligned normals on a
        // shared tangency are containment (the C1 lemma, one dimension
        // down from C3's patch clause). A tangency takes either, but one
        // along the whole locus: two tangent samples whose normals agree
        // at one and oppose at the other are a definite crossing between
        // them.
        if agree && claim == Claim::Contact {
            return Err(ruled(TangentRow::SensesAligned, senses));
        }
        match sides {
            None => sides = Some((agree, senses)),
            Some((first, at)) if first != agree => {
                return Err(Unsettled::Ruled(TangentRuling::SidesMixed {
                    at: [at, senses],
                }));
            }
            Some(_) => {}
        }
        // (4) The second-order margin. Definitely positive is the jet
        // certificate's own verdict; an in-band κ_rel escalates, and an
        // exact zero at an isolated neutral point is the gate's
        // rejection, minted only where the refusal quotes it.
        match second_order {
            // A definite κ_rel has a definite sign: the sheets leave the
            // locus one above the other, and a contact's materials must
            // be on the outer sides of that pair, not between it.
            Ok(Decided {
                sign: Sign::Positive,
                ..
            }) if claim == Claim::Contact => {
                let crescent = Margin::sagitta(
                    geom_brep::material_kappa_rel(jet.kappa_rel, sense1),
                    arm.length(),
                );
                match crate::validate::decide_reported("contact_tangent_overlap", crescent, band) {
                    Ok(Decided {
                        sign: Sign::Negative,
                        ..
                    }) => {}
                    Ok(by) => return Err(ruled(TangentRow::Overlap, by)),
                    Err(diag) => return Err(Unsettled::InBand(diag)),
                }
            }
            Ok(Decided {
                sign: Sign::Positive,
                ..
            }) => {}
            // The schedule's to settle: an in-band reading stands as the
            // sag's, a decided one as none.
            Ok(_) if deferred => unsettled(None),
            Err(diag) if deferred => unsettled(Some(diag)),
            Ok(decided) => {
                return Err(Unsettled::InBand(geom_core::k_stats::gate_rejection(
                    SECOND_ORDER,
                    decided,
                    band,
                    geom_core::SizedPass::Positive,
                )));
            }
            Err(diag) => return Err(Unsettled::InBand(diag)),
        }
    }
    let (touch, log) = geom_core::k_stats::detached(|| span_touch(gap_max, bounds.gap_sag, band));
    let touch = match touch {
        Ok(touch) => {
            geom_core::k_stats::splice(log);
            Some(touch)
        }
        Err(Unsettled::Sag(reading)) => {
            sag_logs.push(log);
            unsettled(reading);
            None
        }
        Err(ended) => {
            geom_core::k_stats::splice(log);
            return Err(ended);
        }
    };
    let (Some(touch), None) = (touch, sag) else {
        return Err(Unsettled::Sag(sag.flatten()));
    };
    Ok(touch)
}

/// **The touch at a sample**: the two surfaces' separation along their
/// shared normal, read off the witness point's residuals `r1`, `r2`.
///
/// `s = r₁ − σ·r₂`, σ the sign of the two implicit gradients'
/// alignment, which the senses' reading decided: with the outward
/// normals opposed σ is the senses' `−ε₁ε₂` (εᵢ = ±1 the face's bit),
/// and `ε₁·s = ε₁r₁ + ε₂r₂` is the sum of the witness's heights above
/// each face (a residual read along the face's outward normal):
/// positive where the surfaces stand apart, negative where each runs
/// into the other's material. The witness's position along the normal
/// cancels, so a witness between the two surfaces reads their whole gap
/// rather than each half of it.
///
/// - Zero passes; the span between samples is the caller's line.
/// - A definite sign rules the pair out ([`TangentRow::Gap`], its
///   margin signed: where the senses oppose, positive is C4's definite
///   separation and negative its definite crossing; where they agree,
///   the sign says which sheet stands over the other).
/// - In band escalates: the touch is MUST-VERIFY-DEFINITE (C4).
///
/// Returns the signed separation.
fn touching<T: Decide>(
    r1: T,
    sense1: bool,
    r2: T,
    sense2: bool,
    band: Band,
) -> Result<T, Unsettled> {
    let height = |r: T, sense: bool| if sense { r } else { -r };
    let separation = height(r1, sense1) + height(r2, sense2);
    match crate::validate::decide_reported("contact_tangent_gap", Margin::of(separation), band) {
        Ok(Decided {
            sign: Sign::Zero, ..
        }) => Ok(separation),
        Ok(by) => Err(ruled(TangentRow::Gap, by)),
        Err(diag) => Err(Unsettled::InBand(diag)),
    }
}

/// **The touch between samples**: the worst sampled separation
/// `gap_max` plus its certified sag (`TangentSpanBounds::gap_sag`), the
/// edge certifier's `tangent_gap_hull_sup` statement
/// (`contact_tangent_touch`). Every sample decided touching, so a span
/// not within ε is the sag's to settle ([`Unsettled::Sag`]), carrying
/// the reading where it is in band.
fn span_touch<T: Decide>(
    gap_max: T,
    gap_sag: T,
    band: Band,
) -> Result<geom_core::MarginDiag, Unsettled> {
    match geom_core::k_stats::decide_magnitude_reported(
        "contact_tangent_touch",
        Margin::of(gap_max + gap_sag),
        band,
    ) {
        Ok((Magnitude::Zero, margin)) => Ok(margin),
        Ok((Magnitude::Positive, _)) => Err(Unsettled::Sag(None)),
        Err(diag) => Err(Unsettled::Sag(Some(diag))),
    }
}

/// The names the witness is decided on each surface under ([`on_surface`]).
const CONTACT_TANGENT_ON_1: &str = "contact_tangent_on_1";
const CONTACT_TANGENT_ON_2: &str = "contact_tangent_on_2";

/// **The witness on a surface, read across the spans a sample bounds**:
/// `r` the sample's implicit residual on `side`'s surface and `sag` the
/// span bound (`TangentSpanBounds::residual_sag`). A span's interior
/// stands within its larger end residual plus `sag` (the edge
/// certifier's `tangent_hull_sup`), so each verdict is decided on its
/// own side:
///
/// - verified on the far side, `|r| + sag` within ε (`on`), which at
///   every sample bounds every point of the locus;
/// - ruled out by the sample itself, `r` definitely nonzero
///   ([`TangentRow::Off`]): a measured point of the witness off the
///   surface is a miss whatever the spans do, so the sag does not shrink
///   it. The pair has already been read touching there ([`touching`]),
///   so the miss is the witness's, a bad cache;
/// - escalated on an in-band sample, which no finer schedule moves;
/// - the sag's to settle with the sample decided on the surface
///   ([`Unsettled::Sag`]), carrying the far side's reading where it is
///   in band.
///
/// MUST-VERIFY-DEFINITE (C4): "the witness within ε of each surface".
/// An in-band residual escalates.
fn on_surface<T: Decide>(
    (on, side): (&'static str, PairSide),
    r: T,
    sag: T,
    band: Band,
) -> Result<(), Unsettled> {
    let far =
        match geom_core::k_stats::decide_magnitude_reported(on, Margin::of(r.abs() + sag), band) {
            Ok((Magnitude::Zero, _)) => return Ok(()),
            Ok((Magnitude::Positive, _)) => None,
            Err(diag) => Some(diag),
        };
    let row = TangentRow::Off(side);
    match crate::validate::decide_reported(row.predicate(), Margin::of(r), band) {
        Ok(Decided {
            sign: Sign::Zero, ..
        }) => Err(Unsettled::Sag(far)),
        Ok(by) => Err(ruled(row, by)),
        Err(diag) => Err(Unsettled::InBand(diag)),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom::Surface;
    use geom_core::Tol;
    use geom_core::{Point3, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn plane(o: [f64; 3], n: [f64; 3]) -> Surface<f64> {
        Surface::Plane {
            origin: Point3::from_array(o),
            normal: Vec3::from_array(n),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn line(o: [f64; 3], d: [f64; 3]) -> geom::Curve3<f64> {
        geom::Curve3::line(Point3::from_array(o), Vec3::from_array(d))
    }

    /// The certified-lane row: a cylinder resting on a plane, tangent
    /// along the contact ruling. Every must-verify condition is
    /// DEFINITE, so the contact verifies.
    #[test]
    fn cylinder_on_plane_verifies_definite_through_the_jet_loop() {
        let cyl = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0),
            axis: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        let flat = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let ruling = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        // Cylinder outward points away from its axis, so at z = 0 it
        // is −ẑ; the plane's face normal is +ẑ. Opposed.
        let read = tangent_locus_relation(&cyl, true, &flat, true, &ruling, -1.0, 1.0, band());
        assert!(matches!(read, TangentVerdict::Touch(_)), "{read:?}");
    }

    /// The independence row: a sphere and a plane through its equator.
    /// The locus lies on BOTH surfaces, so nothing is separated — but the
    /// normals there are definitely INDEPENDENT (radial versus axial),
    /// which is a transverse intersection, not a tangency.
    #[test]
    fn definite_normal_independence_rules_the_contact_out() {
        let sphere = Surface::Sphere {
            center: Point3::origin(),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let flat = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let equator = geom::Curve3::circle(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let read = tangent_locus_relation(
            &sphere,
            true,
            &flat,
            false,
            &equator,
            0.0,
            std::f64::consts::TAU,
            band(),
        );
        let TangentVerdict::Ruled(TangentRuling::Row {
            row: TangentRow::Independent,
            by,
        }) = read
        else {
            panic!("expected the independence row, got {read:?}");
        };
        assert_eq!(by.sign, Sign::Zero, "perpendicular normals: {by:?}");
    }

    /// **An exactly zero relative curvature escalates.** Two coincident
    /// opposite-sense planes are tangent along every line in them, and
    /// their relative curvature is EXACTLY zero everywhere, where the jet
    /// certificate is honestly indeterminate: the contact refuses typed,
    /// never silently passes.
    #[test]
    fn exact_kappa_zero_escalates() {
        let up = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let down = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let along = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let read = tangent_locus_relation(&up, true, &down, false, &along, -1.0, 1.0, band());
        assert!(matches!(read, TangentVerdict::InBand(_)), "{read:?}");
    }

    /// Outside the certified lane there is no verdict at all — the
    /// demanded set IS the certifiable set (C3's order-k boundary),
    /// so a cone tangency refuses typed rather than being sampled.
    #[test]
    fn outside_the_lane_refuses_typed_rather_than_sampling() {
        let cone = Surface::Cone {
            apex: Point3::new(0.0, 0.0, 2.0),
            axis: Vec3::new(0.0, 0.0, -1.0),
            half_angle: 0.5,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let flat = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let along = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert!(matches!(
            tangent_locus_relation(&cone, true, &flat, true, &along, -1.0, 1.0, band()),
            TangentVerdict::NotCertifiable(Uncertified::OutsideLane)
        ));
    }

    /// The on-surface gate's verdicts, each on its own side of the
    /// `|r| ± sag` bracket: the verify side carries the sag, the
    /// contradiction and the escalation read the sample alone, and a
    /// sample decided on the surface under a sag past ε leaves the span
    /// to a finer schedule, carrying the far side's reading where it is
    /// in band. Each row a padding error could flip is named for the
    /// error that flips it.
    #[test]
    fn on_surface_pads_each_verdict_in_its_own_direction() {
        let b = band();
        let (eps, big) = (b.zero(), 2.0 * b.escalate());
        #[derive(Debug, PartialEq)]
        enum Want {
            On,
            Escalated,
            Off,
            SagInBand,
            SagBeyond,
        }
        for (r, sag, want, why) in [
            (0.0, 0.0, Want::On, "an exact sample, no sag"),
            (0.4 * eps, 0.4 * eps, Want::On, "the sum within ε"),
            (
                0.6 * eps,
                0.6 * eps,
                Want::SagInBand,
                "each within ε, the sum in band",
            ),
            (
                0.0,
                big,
                Want::SagBeyond,
                "on at the sample, a sag beyond K·ε between",
            ),
            (
                big,
                1.5 * big,
                Want::Off,
                "a definite miss under a larger sag",
            ),
            (-big, 0.0, Want::Off, "a definite miss on the other side"),
            (
                0.6 * eps,
                big,
                Want::SagBeyond,
                "a sample within ε under a large sag",
            ),
            (
                3.0 * eps,
                big,
                Want::Escalated,
                "an in-band sample under a large sag",
            ),
            (
                3.0 * eps,
                0.0,
                Want::Escalated,
                "an in-band sample under no sag",
            ),
        ] {
            let off = TangentRow::Off(PairSide::Second);
            let got = match on_surface(("probe_on", PairSide::Second), r, sag, b) {
                Ok(()) => Want::On,
                Err(Unsettled::InBand(diag)) => {
                    assert_eq!(
                        diag.predicate,
                        Some(off.predicate()),
                        "{why}: the sample's own"
                    );
                    assert!(!diag.margin.is_invalid(), "{why}: the in-band reading");
                    Want::Escalated
                }
                Err(Unsettled::Ruled(TangentRuling::Row { row, by })) => {
                    assert_eq!(row, off, "{why}: the surface's own row");
                    assert!(!by.margin.is_invalid(), "{why}: the measured margin");
                    assert_eq!(
                        by.sign,
                        if r > 0.0 {
                            Sign::Positive
                        } else {
                            Sign::Negative
                        }
                    );
                    Want::Off
                }
                Err(Unsettled::Sag(Some(diag))) => {
                    assert_eq!(diag.predicate, Some("probe_on"), "{why}: the far side's");
                    Want::SagInBand
                }
                Err(Unsettled::Sag(None)) => Want::SagBeyond,
                Err(Unsettled::Ruled(other)) => panic!("{why}: {other:?}"),
            };
            assert_eq!(got, want, "{why}: r = {r:e}, sag = {sag:e}");
        }
    }

    /// The span's touch: the worst sampled separation plus the gap sag
    /// is verified within ε, and past ε it is the sag's to settle,
    /// carrying the reading where it is in band and none beyond, which
    /// is never an escalation of the geometry's.
    #[test]
    fn the_span_touch_leaves_a_sag_past_eps_to_the_schedule() {
        let b = band();
        let eps = b.zero();
        let read = |gap_max: f64, gap_sag: f64| span_touch(gap_max, gap_sag, b);
        assert!(read(0.5 * eps, 0.4 * eps).is_ok(), "the sum within ε");
        match read(0.5 * eps, 0.6 * eps) {
            Err(Unsettled::Sag(Some(diag))) => {
                assert_eq!(diag.predicate, Some("contact_tangent_touch"));
            }
            other => panic!("the sum in band: {:?}", other.err().map(|_| ())),
        }
        assert!(
            matches!(
                read(0.5 * eps, 2.0 * b.escalate()),
                Err(Unsettled::Sag(None))
            ),
            "a sag beyond K·ε"
        );
    }

    /// **A finer schedule settles a sag just past ε, and only the
    /// certificate's reach refuses past the finest.** Two tubes of one
    /// minor radius whose spines kiss at `(1, 0, 0)` share the meridian
    /// circle there, the G1 tube chain's junction: every sample lies on
    /// both tori, and the circle arm's span bound is the whole story.
    /// Over a span whose nine-sample sag stands at four times K·ε, the
    /// table halves its step until the sag is within ε and verifies.
    /// Over the arc between the poles about the outer equator no schedule
    /// it reads gets there, every other reading decided, and the span
    /// is not certifiable. The whole circle passes through the
    /// meridian's poles, where the two tubes osculate (`κ_rel` exactly
    /// zero at a sample), and that second-order reading answers, as an
    /// escalation, whatever the span bound reads.
    #[test]
    fn a_finer_schedule_settles_a_sag_the_nine_samples_leave_past_eps() {
        // The rows are about the schedule against the fixture's own
        // bounds, so they read one band at every eps row: at a coarser
        // one the finest schedule's bounds end inside it and escalate.
        let b = Band::linear_at(Tol::witness(), 1e-9).unwrap();
        let torus = |x: f64| Surface::Torus {
            center: Point3::new(x, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: 1.0,
            minor_radius: 0.1,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let (near, far) = (torus(0.0), torus(2.0));
        let meridian = geom::Curve3::circle(
            Point3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.1,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let tau = core::f64::consts::TAU;
        let whole = geom_brep::tangent_span_bounds(&near, &far, &meridian, 0.0, tau)
            .unwrap()
            .residual_sag;
        // The sag goes as the step squared.
        let short = tau * (4.0 * b.escalate() / whole).sqrt();
        let nine = geom_brep::tangent_span_bounds(&near, &far, &meridian, 0.0, short)
            .unwrap()
            .residual_sag;
        assert!(
            nine > b.escalate(),
            "the nine-sample sag stands past the band: {nine:e}"
        );
        let read = |t1: f64| tangency_reading(&near, true, &far, true, &meridian, 0.0, t1, b);
        let short_read = read(short);
        assert!(
            matches!(short_read, TangentVerdict::Touch(_)),
            "the finer schedule verifies the short span: {short_read:?}"
        );
        // The arc about the outer equator, a quarter radian short of each
        // pole.
        let arc = core::f64::consts::PI - 0.5;
        let finest = geom_brep::tangent_span_bounds(
            &near,
            &far,
            &meridian,
            0.0,
            arc / f64::from(1u32 << REFINE_HALVINGS),
        )
        .unwrap()
        .residual_sag;
        assert!(
            finest > b.escalate(),
            "the finest schedule's sag stands past the band: {finest:e}"
        );
        let bracket = geom_core::k_stats::Bracket::open();
        let span = tangency_reading(&near, true, &far, true, &meridian, -0.5 * arc, 0.5 * arc, b);
        let log = bracket.finish();
        assert!(
            matches!(
                span,
                TangentVerdict::NotCertifiable(Uncertified::SagUnreached)
            ),
            "the arc about the equator is past the finest schedule: {span:?}"
        );
        // The schedule left the second order to the span bound: the
        // outcome rests on its decided reading, which is no escalation.
        let second_order: Vec<_> = log
            .escalations
            .iter()
            .filter(|e| e.source.predicate == Some(SECOND_ORDER))
            .collect();
        assert!(
            second_order.is_empty(),
            "a sag-only outcome logs no second-order rejection: {second_order:?}"
        );
        // The lane's other `NotCertifiable` is the contact's own refusal,
        // not the span bound's: a carrier outside the lane.
        let outside = geom::Curve3::line(Point3::new(1.1, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(
            matches!(
                tangency_reading(&near, true, &far, true, &outside, 0.0, 0.1, b),
                TangentVerdict::NotCertifiable(Uncertified::OutsideLane)
            ),
            "a line on two tori is outside the lane"
        );
        let bracket = geom_core::k_stats::Bracket::open();
        let whole = read(tau);
        let log = bracket.finish();
        match whole {
            TangentVerdict::InBand(diag) => {
                assert!(
                    log.escalations.iter().any(|e| e.source == diag),
                    "the rejection the refusal quotes is on the log: {diag:?}"
                );
                assert_eq!(diag.predicate, Some("contact_tangent_second_order"));
                assert_eq!(
                    diag.margin.rejected_sign(),
                    Some(geom_core::Sign::Negative),
                    "the gate's rejection of κ_rel decided under its drift bound, on the \
                     margin it was decided on: {diag:?}"
                );
            }
            other => panic!("the whole circle osculates at the poles: {other:?}"),
        }
    }

    /// **A rim the pair's pose reads tangent can still escalate along
    /// the rim**, and both answers are right. Two pipe bends joined G1
    /// along the tube circle at `(2, 0, 0)` (cores of `R = 2` and
    /// `1.5` in planes `0.7` rad apart, tubes of `0.5`), the second's
    /// tube `0.6ε` wider and moved `0.6ε` along `x`: the cores' least
    /// distance and the tube level stand within the zero band of each
    /// other, so the pose's critical gap decides the pair tangent; the
    /// first bend's rim circle stands `1.2ε` off the second torus on its
    /// outer side, which the oracle (the exact distance from the rim's
    /// points to the second torus) measures too, so no edge certified at
    /// ε runs along it and the touch escalates.
    #[test]
    fn a_rim_the_pose_reads_tangent_can_still_escalate_along_the_rim() {
        let b = band();
        let e = b.zero();
        let torus = |c: Point3<f64>, a: Vec3<f64>, big: f64, r: f64| {
            let a = a.normalize();
            Surface::Torus {
                center: c,
                axis: a,
                major_radius: big,
                minor_radius: r,
                u_ref: a.orthonormal_basis().0,
            }
        };
        let first = torus(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), 2.0, 0.5);
        let alpha = 0.7f64;
        let inward = Vec3::new(-alpha.cos(), 0.0, alpha.sin());
        let (bend_center, bend_axis) = (
            Point3::new(2.0 + 0.6 * e, 0.0, 0.0) + inward * 1.5,
            inward.cross(Vec3::new(0.0, 1.0, 0.0)).normalize(),
        );
        let bend = torus(bend_center, bend_axis, 1.5, 0.5 + 0.6 * e);
        let rim = geom::Curve3::circle(
            Point3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.5,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let pose = geom_brep::TorusTorusPose::decide(&first, &bend, b).expect("two tori");
        assert!(
            matches!(pose, Ok(ref p) if matches!(p.gap(), geom_brep::TorusGap::Tangent { .. })),
            "the pose decides the pair tangent: {pose:?}"
        );
        let off = (0..360)
            .map(|k| {
                let q = rim.eval(core::f64::consts::TAU * f64::from(k) / 360.0);
                let d = q - bend_center;
                let h = d.dot(bend_axis);
                ((d - bend_axis * h).norm() - 1.5).hypot(h) - (0.5 + 0.6 * e)
            })
            .fold(0.0f64, |far, d| far.max(d.abs()));
        assert!(off > e, "the rim stands {off:e} off the second torus");
        for sense in [true, false] {
            match tangency_reading(
                &first,
                true,
                &bend,
                sense,
                &rim,
                0.0,
                core::f64::consts::TAU,
                b,
            ) {
                TangentVerdict::InBand(diag) => {
                    assert_eq!(diag.predicate, Some("contact_tangent_gap"));
                }
                other => panic!("the touch along the rim: {other:?}"),
            }
        }
    }

    /// A unit cylinder resting on the plane `z = 0` with its axis lifted
    /// to `z = 1 + gap` (a negative gap sinks it in), read along the
    /// line at height `z = witness` under the ruling: the pair's touch
    /// and the witness's place are set apart.
    fn lifted_cylinder_on_plane(gap: f64, witness: f64) -> TangentVerdict {
        let cyl = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0 + gap),
            axis: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        let flat = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let along = line([0.0, 0.0, witness], [1.0, 0.0, 0.0]);
        tangent_locus_relation(&cyl, true, &flat, true, &along, -1.0, 1.0, band())
    }

    /// **The touch is read on the pair, not on each half of it.** A
    /// witness halfway between two surfaces stands within each by half
    /// their gap, so two residual rows each pass where the gap is under
    /// 2ε; the separation `s = r₁ − σ·r₂` reads the whole gap.
    ///
    /// - 1.8ε apart: each residual is 0.9ε, within ε; the gap is in
    ///   band and escalates under its own name.
    /// - 15ε apart: the gap is definite, a separation (positive), and
    ///   rules the pair out. Each residual (7.5ε) is in band, so a
    ///   decider reading only the residuals escalates here.
    /// - 15ε into each other: the gap is definite and negative, a
    ///   crossing.
    #[test]
    fn a_witness_between_the_surfaces_reads_their_whole_gap() {
        let eps = band().zero();
        match lifted_cylinder_on_plane(1.8 * eps, 0.9 * eps) {
            TangentVerdict::InBand(diag) => {
                assert_eq!(
                    diag.predicate,
                    Some("contact_tangent_gap"),
                    "1.8ε: {diag:?}"
                );
            }
            other => panic!("1.8ε apart escalates on the gap: {other:?}"),
        }
        for (gap, sign, what) in [
            (15.0, Sign::Positive, "separation"),
            (-15.0, Sign::Negative, "crossing"),
        ] {
            match lifted_cylinder_on_plane(gap * eps, 0.5 * gap * eps) {
                TangentVerdict::Ruled(TangentRuling::Row {
                    row: TangentRow::Gap,
                    by,
                }) => {
                    assert_eq!(by.sign, sign, "{what}: the reading's sign names it: {by:?}");
                    assert!(
                        !by.margin.is_invalid(),
                        "{what}: the measured margin: {by:?}"
                    );
                }
                other => panic!("{what} by 15ε is ruled out on the gap: {other:?}"),
            }
        }
    }

    /// **A witness off a touching pair is a bad witness, not a gap.**
    /// Displaced 11ε along the shared normal, the witness is 11ε out of
    /// the cylinder and 11ε above the plane, and the two heights cancel
    /// in the gap: the pair touches. The residual row rules it out,
    /// naming the first surface it reads.
    #[test]
    fn a_witness_off_a_touching_pair_is_a_bad_witness() {
        let eps = band().zero();
        match lifted_cylinder_on_plane(0.0, 11.0 * eps) {
            TangentVerdict::Ruled(TangentRuling::Row { row, by }) => {
                assert_eq!(row, TangentRow::Off(PairSide::First), "{by:?}");
                assert_eq!(row.predicate(), "contact_tangent_off_1");
            }
            other => panic!("a witness 11ε off is ruled out on its residual: {other:?}"),
        }
        assert!(
            matches!(lifted_cylinder_on_plane(0.0, 0.0), TangentVerdict::Touch(_)),
            "the same pair read along its ruling"
        );
    }

    /// A pin in a bore, radii `1` and `1 + δ`, axes along z, placed so
    /// the two walls cross at angle `φ` along a z-line (they touch along
    /// it at `φ = 0`). `δ` is set so the second-order
    /// sagitta reads `so_in_eps · ε`; the carrier is the crossing line.
    fn pin_in_bore(so_in_eps: f64, phi: f64) -> (Surface<f64>, Surface<f64>, geom::Curve3<f64>) {
        let b = band();
        let cyl = |c: [f64; 3], radius: f64| Surface::Cylinder {
            origin: Point3::from_array(c),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        // `κ_rel = 1 − 1/R₂` across the touch, levered at the pin's
        // unit radius: the sagitta is `κ_rel / 2`.
        let kappa = 2.0 * so_in_eps * b.zero();
        let r2 = 1.0 / (1.0 - kappa);
        // The walls meet at `(−1, 0)`: the pin's outward normal there is
        // `−x̂`, and the bore's radial is that turned by `φ`.
        let (sin, cos) = phi.sin_cos();
        (
            cyl([0.0, 0.0, 0.0], 1.0),
            cyl([-1.0 + r2 * cos, r2 * sin, 0.0], r2),
            line([-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        )
    }

    /// The second-order reading is decided first and acted on last, so
    /// its escalation reaches the frame's log only when the outcome
    /// rests on it: a definite first-order defect rules the pair out with
    /// an empty escalation log, and an in-band one is the log's only
    /// escalation. Where the outcome does rest on it (the first order
    /// verified), the refusal carries the reading itself.
    #[test]
    fn an_overruled_second_order_escalation_leaves_the_log() {
        let b = band();
        let read = |phi: f64| {
            let (pin, bore, along) = pin_in_bore(3.0, phi);
            // The fixture's premise, read at the first sample: the
            // second-order sagitta in band.
            let p = along.eval(-1.0);
            let jet = tangent_jet(&pin, &bore, p, Vec3::new(0.0, 0.0, 1.0));
            let drift = tangent_span_bounds(&pin, &bore, &along, -1.0, 1.0)
                .unwrap()
                .kappa_drift;
            let so = 0.5 * (jet.kappa_rel.abs() - drift);
            assert!(
                so > b.zero() && so < b.escalate(),
                "phi = {phi:e}: the second-order sagitta {so:e} is in band"
            );
            let bracket = geom_core::k_stats::Bracket::open();
            let out = tangent_locus_relation(&pin, true, &bore, false, &along, -1.0, 1.0, b);
            let log = bracket.finish();
            let escalations: Vec<_> = log.escalations.iter().map(|e| e.source).collect();
            (out, escalations)
        };
        // A definite crossing: the parallelism defect rules it out.
        let (out, escalations) = read(1e-3);
        assert!(
            matches!(
                out,
                TangentVerdict::Ruled(TangentRuling::Row {
                    row: TangentRow::Parallel,
                    ..
                })
            ),
            "a definite defect rules the pair out: {out:?}"
        );
        assert_eq!(escalations, vec![], "a ruling leaves no escalation");
        // An in-band crossing: the parallel reading is the refusal.
        let (out, escalations) = read((3.0 * b.zero()).asin());
        let TangentVerdict::InBand(diag) = out else {
            panic!("an in-band defect escalates: {out:?}");
        };
        assert_eq!(diag.predicate, Some("contact_tangent_parallel"));
        assert_eq!(
            escalations,
            vec![diag],
            "the refusal is the log's escalation"
        );
        // A touch: the second-order reading is the refusal.
        let (out, escalations) = read(0.0);
        let TangentVerdict::InBand(diag) = out else {
            panic!("an in-band κ_rel escalates: {out:?}");
        };
        assert_eq!(diag.predicate, Some("contact_tangent_second_order"));
        assert_eq!(
            escalations,
            vec![diag],
            "the refusal is the log's escalation"
        );
    }

    /// **The glue door's tangency claims neither sense nor crescent**:
    /// three tangencies along the z-line through `(2, 0)` (or `(1, 0)`),
    /// each verified by [`tangency_reading`], and the contact table's
    /// verdict beside it where the two differ. A disc of radius 1 inside one of
    /// radius 2 has its outward normals aligned; a bore of radius 1 in a
    /// disc of radius 2 has them opposed with the crescent shared by both
    /// materials; two unit discs kissing have them opposed with the
    /// crescent void.
    #[test]
    fn the_door_reads_any_sense_and_any_crescent() {
        let b = band();
        let cyl = |cx: f64, radius: f64| Surface::Cylinder {
            origin: Point3::new(cx, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let (fat, small) = (cyl(0.0, 2.0), cyl(1.0, 1.0));
        let at_two = line([2.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let read = |s1, sense1, s2, sense2, along| {
            matches!(
                tangency_reading(s1, sense1, s2, sense2, along, 0.0, 1.0, b),
                TangentVerdict::Touch(_)
            )
        };
        assert!(
            read(&fat, true, &small, true, &at_two),
            "the nested discs: aligned"
        );
        assert!(
            matches!(
                tangent_locus_relation(&fat, true, &small, true, &at_two, 0.0, 1.0, b),
                TangentVerdict::Ruled(TangentRuling::Row {
                    row: TangentRow::SensesAligned,
                    ..
                })
            ),
            "the nested discs as a contact: ruled out on the senses"
        );
        assert!(
            read(&fat, true, &small, false, &at_two),
            "the bore in the disc: opposed, the crescent shared"
        );
        assert!(
            matches!(
                tangent_locus_relation(&fat, true, &small, false, &at_two, 0.0, 1.0, b),
                TangentVerdict::Ruled(TangentRuling::Row {
                    row: TangentRow::Overlap,
                    ..
                })
            ),
            "the bore in the disc as a contact: ruled out on the crescent"
        );
        let at_one = line([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!(
            read(&cyl(0.0, 1.0), true, &cyl(2.0, 1.0), true, &at_one),
            "the kissing discs: opposed, the crescent void"
        );
        // The same reading refuses what is no tangency, so the three
        // above are its verdicts and not a door left open.
        assert!(
            matches!(
                tangency_reading(
                    &cyl(0.0, 1.0),
                    true,
                    &cyl(2.5, 1.0),
                    true,
                    &at_one,
                    0.0,
                    1.0,
                    b
                ),
                TangentVerdict::Ruled(_)
            ),
            "discs a gap apart: the line is off one of them"
        );
        assert!(
            matches!(
                tangency_reading(
                    &cyl(0.0, 1.0),
                    true,
                    &cyl(1.5, 1.0),
                    true,
                    &at_one,
                    0.0,
                    1.0,
                    b
                ),
                TangentVerdict::Ruled(_)
            ),
            "overlapping discs: the line is off one of them"
        );
    }

    /// **The Villarceau probe.** A torus (`R = 2`, `r = 0.5`, about z)
    /// and its mirror image in a bitangent plane share that plane's
    /// Villarceau circle, a rim the lane admits (two round surfaces on a
    /// circle) that is a line of curvature of neither. Along it the two
    /// outward normals are each other's mirror image, so the pair is
    /// tangent at four points only: aligned where the first torus's
    /// normal lies in the plane (`t = π/2, 3π/2`), where the mirror makes
    /// `κ_rel` exactly zero, and opposed where it is the plane's normal,
    /// returned here. Every other sample is definitely not parallel.
    fn villarceau() -> (Surface<f64>, Surface<f64>, geom::Curve3<f64>, f64) {
        let (big, small) = (2.0f64, 0.5f64);
        let tilt = (small / big).asin();
        let m = Vec3::new(-tilt.sin(), 0.0, tilt.cos());
        let mirror = |v: Vec3<f64>| v - m * (2.0 * v.dot(m));
        let torus = |axis: Vec3<f64>, u_ref: Vec3<f64>| Surface::Torus {
            center: Point3::origin(),
            axis,
            major_radius: big,
            minor_radius: small,
            u_ref,
        };
        let (z, x) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let circle = geom::Curve3::circle(
            Point3::new(0.0, small, 0.0),
            m,
            big,
            Vec3::new(tilt.cos(), 0.0, tilt.sin()),
        );
        let first = torus(z, x);
        // The opposed point: a root of the first torus's normal's component
        // across the plane's normal, in the circle's normal plane.
        let across = |t: f64| {
            let (p, tau) = circle.ders1(t);
            implicit_outward_normal(&first, true, p)
                .vec()
                .dot(tau.cross(m).normalize())
        };
        let (mut lo, mut hi) = (3.0, 3.6);
        assert!(across(lo) * across(hi) < 0.0, "a root in [3, 3.6]");
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if across(lo) * across(mid) <= 0.0 {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        (first, torus(mirror(z), mirror(x)), circle, 0.5 * (lo + hi))
    }

    /// The probe's answer, with the parallel check ahead of the sides:
    /// started anywhere on the circle, the nine-station schedule's first
    /// sample off the four tangent points ends the pass not parallel (or
    /// an aligned point's zero `κ_rel` escalates first), so no two
    /// samples read tangent with opposite senses. Started on the opposed
    /// point, the next station reads the normals aligned but not
    /// parallel: the parallel row's, which a sides comparison read ahead
    /// of it would call mixed.
    #[test]
    fn the_villarceau_rim_reads_no_two_tangent_samples_with_opposite_senses() {
        let b = band();
        let (first, second, circle, opposed) = villarceau();
        let tau = core::f64::consts::TAU;
        let parallel = |read: &TangentVerdict| {
            matches!(
                read,
                TangentVerdict::Ruled(TangentRuling::Row {
                    row: TangentRow::Parallel,
                    ..
                })
            )
        };
        for k in 0..64 {
            let t0 = tau * f64::from(k) / 64.0;
            let read = tangency_reading(&first, true, &second, true, &circle, t0, t0 + tau, b);
            let neutral = matches!(read, TangentVerdict::InBand(diag)
                if diag.predicate == Some(SECOND_ORDER));
            assert!(parallel(&read) || neutral, "started at {t0}: {read:?}");
        }
        let p = circle.eval(opposed + tau / 8.0);
        let dot = implicit_outward_normal(&first, true, p)
            .vec()
            .dot(implicit_outward_normal(&second, true, p).vec());
        assert!(
            dot > 0.1,
            "the next station reads the normals aligned: {dot}"
        );
        let read = tangency_reading(
            &first,
            true,
            &second,
            true,
            &circle,
            opposed,
            opposed + tau,
            b,
        );
        assert!(
            parallel(&read),
            "the aligned station is not tangent: {read:?}"
        );
    }

    /// **Mixed sides are a verdict a span can reach.** A schedule over the
    /// same rim whose step is the arc from the opposed tangent point to
    /// the aligned one at `3π/2` puts its first two stations on both:
    /// each reads tangent, and their senses disagree.
    #[test]
    fn two_tangent_samples_with_opposite_senses_are_mixed_sides() {
        let b = band();
        let (first, second, circle, opposed) = villarceau();
        let step = 1.5 * core::f64::consts::PI - opposed;
        let t1 = opposed + step * f64::from(CERT_SAMPLES - 1);
        let read = tangency_reading(&first, true, &second, true, &circle, opposed, t1, b);
        let TangentVerdict::Ruled(TangentRuling::SidesMixed { at }) = read else {
            panic!("two tangent samples, opposed then aligned: {read:?}");
        };
        assert_eq!(
            at.map(|d| d.sign),
            [Sign::Negative, Sign::Positive],
            "the earlier sample's reading first: {at:?}"
        );
    }
}
