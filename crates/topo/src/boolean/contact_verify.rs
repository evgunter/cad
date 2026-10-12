//! **Contact verification** — the C4 per-class tables, executable.
//!
//! A declaration asserts intent about the nominal geometry, and the
//! kernel VERIFIES it, never trusts it. Each class states exactly
//! three lists, and this module is where each list becomes code:
//!
//! - *must-verify-DEFINITE* — the geometry must definitely satisfy
//!   these, or the declaration is contradicted;
//! - *contradiction triggers* — definite counter-evidence, typed;
//! - *bridged residue* — the indeterminate margins intent covers, and
//!   NOTHING else ([`crate::contact::ContactVerdict::Bridged`]).
//!
//! [`contact_pair_verdict`] is the class-dispatching door: `Rest`
//! goes down the carrier ladder ([`mod@super::carrier_eq`]), `Tangent`
//! down the jet schedule ([`tangent_pair_relation`]).
//!
//! **The Tangent lane reuses the jet machinery, it does not restate
//! it.** `tangent_jet`, `tangent_certificate_lane`,
//! `tangent_span_bounds` and the certification schedule's own
//! `sample_param` all come from `geom_brep` — the demanded set IS the
//! certifiable set, which is C3's order-k boundary as a fact about
//! the code rather than a promise about it. Outside the lane, this
//! refuses [`ContactRefusal::NotCertifiable`]: an order-k > 1 contact
//! has no record type, so it gets no verdict either.
//!
//! **The bridged residue of `Tangent`, stated to be attacked** (C4,
//! the #175 clause). A declared tangency bridges the in-band `κ_rel`
//! — *including exact zeros at isolated points*. That is deliberately
//! WEAKER than a jet certificate, which refuses those points
//! (`NotSecondOrderSeparated`), and it must be: a G1 tube chain's
//! neutral meridian points have both spines' curvature projections
//! vanishing, so demanding jet-determinacy everywhere would make
//! every such chain undeclarable at exactly two points — #175
//! finding 1, reproduced with extra steps.
//!
//! **The residue is that margin and NOTHING else**, which is a claim
//! about code and is enforced by its shape: the pair's separation
//! along the shared normal (the touch), the witness's residual on each
//! surface (each in band, or decided on the surface at a sample whose
//! span bound is wider than the band) and the opposed-senses test
//! return `Escalated` whether or not a declaration is in hand, and so
//! does the first-order parallelism test — those are C4's
//! must-verify-DEFINITE list, and a list you may bridge is not a
//! must-verify list. Only the second-order arm consults `declared`.
//! In-band there is therefore a THIRD outcome, not a widened second
//! one: verified / bridged-by-declaration / escalated, with
//! contradiction sitting across all five arms.
//!
//! **What this lane does NOT check, named** (C4's "definite crossing
//! (II_rel definitely indefinite)"): a crossing at first order, the
//! surfaces running into each other's material at a sample, is the
//! touch's negative side (`touching`); a crossing at second order is
//! decided only through the transverse `κ_rel` the jet computes along
//! the locus tangent's perpendicular. A full second-fundamental-form
//! indefiniteness test over all tangent directions needs machinery
//! that does not exist yet. The honest consequence, stated in the
//! direction the code actually takes: a declared tangency whose
//! relative shape operator is indefinite only in a direction the jet
//! does not sample passes every arm this lane runs, so it comes back
//! `Definite` — not `Bridged`, which would at least record that
//! intent was doing work. Nothing here is weakened *by* the
//! declaration; the contradiction list is simply narrower than C4's,
//! and a configuration outside the sampled direction is missed rather
//! than bridged.

use geom::Curve3;
use geom_brep::{
    CERT_SAMPLES, implicit_outward_normal, sample_param, tangent_certificate_lane, tangent_jet,
    tangent_span_bounds,
};
use geom_core::k_stats::Magnitude;
use geom_core::{Band, Decide, Indeterminate, Margin, Sign, Vec3};

use crate::body::Body;
use crate::contact::{ContactClass, ContactRefusal, ContactVerdict, FIT_DEFERRAL};
use crate::entity::FaceKey;

use super::carrier_eq::{CarrierEqError, CarrierRelation};
use super::refusal_routes::Contradiction;

/// **The class-dispatching contact door**: does this face pair hold
/// the declared contact, and on whose evidence?
///
/// `witness` is the locus a `Tangent` declaration is verified along,
/// with its parameter range; `Rest` ignores it (the conformal class's
/// evidence is the carrier, not a locus).
///
/// The verdict is the AQ6 trilean: [`ContactVerdict::Definite`] (the
/// geometry says so on its own — the declaration added nothing),
/// [`ContactVerdict::Bridged`] (the definite conditions hold and the
/// declaration covers the in-band residue), or a typed
/// [`ContactRefusal`]. Every definite verdict wins over every
/// declaration.
///
/// # Errors
///
/// [`ContactRefusal`] — contradiction, escalation, or a configuration
/// outside the class's certifiable set.
pub fn contact_pair_verdict<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    class: ContactClass,
    witness: Option<(&Curve3<T>, T, T)>,
    band: Band,
) -> Result<ContactVerdict, ContactRefusal> {
    match class {
        ContactClass::Rest => rest_pair_reading(a, fa, b, fb, band).map(|(verdict, _)| verdict),
        ContactClass::Tangent => {
            let Some((carrier, t0, t1)) = witness else {
                return Err(ContactRefusal::NotCertifiable {
                    what: "a Tangent declaration is verified ALONG a locus; none was supplied",
                });
            };
            tangent_pair_relation(a, fa, b, fb, carrier, t0, t1, true, band)
        }
    }
}

/// **AQ6's steer**: when a declared `Rest` is contradicted by a
/// SEPARATION-shaped margin — a radius difference, a centre offset, a
/// parallel-plane offset — the geometry is usually a designed
/// clearance wearing the wrong declaration. The steer names the class
/// that fits AND names its deferral, so the message is a live pointer
/// rather than a dead one.
///
/// An ANGULAR contradiction (axes not parallel, planes not parallel)
/// gets no steer: no gap makes those two carriers one, so pointing at
/// `Fit` there would be advice that cannot work
/// ([`Contradiction::fits_a_clearance`]).
pub(super) fn fit_steer(fact: Contradiction) -> Option<&'static str> {
    fact.fits_a_clearance().then_some(FIT_DEFERRAL)
}

/// The `Rest` table (C4): carrier non-contradiction through the
/// kind-generalized ladder, plus opposed senses.
///
/// **Aligned senses contradict.** Two faces on one carrier whose
/// material sides agree are not in contact — they are the same
/// material, or one contains the other (the C1 lemma). The carrier
/// ladder reports that honestly as `SameOriented`; refusing it is
/// THIS door's job, because `SameOriented` is a legitimate and
/// exercised answer at the classification/merge site (flush walls),
/// and only a claim of CONTACT makes it a lie.
pub(crate) fn rest_pair_reading<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<(ContactVerdict, geom_core::MarginDiag), ContactRefusal> {
    let outcome =
        super::carrier_pair::carrier_pair_reading(a, fa, b, fb, true, band).map_err(|unread| {
            ContactRefusal::NotCertifiable {
                what: match unread {
                    super::carrier_pair::PairUnread::OutsideInventory => {
                        "a declared face's surface kind is outside the Rest ladder's inventory \
                     (plane, sphere, cylinder, torus)"
                    }
                    super::carrier_pair::PairUnread::Extent(_) => {
                        "a declared face's consumed extent cannot be read (its box has no claim to \
                     make, or its boundary cannot be walked)"
                    }
                },
            }
        })?;
    match outcome {
        Ok((CarrierRelation::SameOpposite, verdict, Some(margin))) => Ok((verdict, margin)),
        // A one-carrier reading carries the margin that decided it
        // (`CarrierReading`): only `Distinct` reads none.
        Ok((CarrierRelation::SameOpposite, _, None)) => {
            unreachable!("a carrier reading decided one carrier, opposed, and carries no margin")
        }
        Ok((CarrierRelation::SameOriented, ..)) => Err(ContactRefusal::Contradicted {
            diag: Indeterminate {
                margin: geom_core::MarginDiag::INVALID,
                band,
                predicate: Some("contact_rest_senses_opposed"),
                terminal_sliver: false,
            },
            steer: None,
        }),
        // The ladder contradicts a declared pair before it can call
        // it `Distinct`, and a declared pair never reaches the
        // undeclared posture's undecided coincidence; either here would
        // be the ladder breaking its own contract.
        Ok((CarrierRelation::Distinct, ..)) | Err(CarrierEqError::Undecided { .. }) => {
            Err(ContactRefusal::Escalated {
                diag: Indeterminate {
                    margin: geom_core::MarginDiag::INVALID,
                    band,
                    predicate: Some("contact_rest_ladder_invariant"),
                    terminal_sliver: false,
                },
            })
        }
        Err(CarrierEqError::Contradicted { fact, diag }) => Err(ContactRefusal::Contradicted {
            steer: fit_steer(fact),
            diag,
        }),
        Err(CarrierEqError::Escalated { diag, .. } | CarrierEqError::Unsettled { diag }) => {
            Err(ContactRefusal::Escalated { diag })
        }
    }
}

/// **The Tangent verify door** — C4's `Tangent` table run over the
/// jet schedule.
///
/// Per sample on the certification schedule (the same 9 parameters,
/// through `geom_brep::sample_param`):
///
/// - **must-verify-DEFINITE**: the two surfaces touch, their
///   separation along the shared normal within ε (`touching`, and the
///   span line after the loop); the witness lies within ε of each
///   surface (`on_surface`); and the two outward normals are definitely
///   OPPOSED and parallel within the derived angle at lever arm
///   `1/κ_rel`, the C7 threshold verbatim;
/// - **contradicts**: definite separation or definite crossing (the
///   separation definitely positive or negative, with the `Fit` steer),
///   a bad witness (a residual definitely nonzero where the pair
///   touches, no steer), definite normal independence (`sin θ`
///   definitely nonzero at that arm), or aligned normals (containment,
///   not tangency);
/// - **bridges** (declared only): the second-order margin and ONLY it
///   — in-band `κ_rel`, and exact zeros at isolated neutral points
///   (module docs, the #175 clause);
/// - **escalates**: an in-band margin on ANY must-verify arm (the
///   touch at a sample or across a span, a residual, opposition,
///   first-order parallelism), declared or not, and the second-order
///   residue when undeclared.
///
/// The span bounds between samples come from the same
/// `tangent_span_bounds` the edge certifier uses, so the hull
/// discipline is shared rather than mirrored.
///
/// `declared = false` runs the same ladder as a DETECTOR: in-band
/// residue then escalates instead of bridging, which is the honest
/// candidate-generation posture (a candidate is only ever definite).
///
/// # Errors
///
/// [`ContactRefusal`] — see [`contact_pair_verdict`].
#[allow(clippy::too_many_arguments)]
pub fn tangent_pair_relation<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
    declared: bool,
    band: Band,
) -> Result<ContactVerdict, ContactRefusal> {
    let (face_a, face_b) = (
        a.get_face(fa).ok_or(ContactRefusal::NotCertifiable {
            what: "the A-side declared face does not resolve",
        })?,
        b.get_face(fb).ok_or(ContactRefusal::NotCertifiable {
            what: "the B-side declared face does not resolve",
        })?,
    );
    let (s1, s2) = (
        a.get_surface(face_a.surface)
            .ok_or(ContactRefusal::NotCertifiable {
                what: "the A-side declared face has no surface",
            })?,
        b.get_surface(face_b.surface)
            .ok_or(ContactRefusal::NotCertifiable {
                what: "the B-side declared face has no surface",
            })?,
    );
    tangent_locus_relation(
        s1,
        face_a.sense,
        s2,
        face_b.sense,
        carrier,
        t0,
        t1,
        declared,
        band,
    )
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
/// overlap contradicts (`contact_tangent_overlap`).
///
/// # Errors
///
/// [`ContactRefusal`] — see [`tangent_pair_relation`].
#[allow(clippy::too_many_arguments)]
pub fn tangent_locus_relation<T: Decide>(
    s1: &geom::Surface<T>,
    sense1: bool,
    s2: &geom::Surface<T>,
    sense2: bool,
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
    declared: bool,
    band: Band,
) -> Result<ContactVerdict, ContactRefusal> {
    tangent_locus_reading(s1, sense1, s2, sense2, carrier, t0, t1, declared, band)
        .map(|(verdict, _)| verdict)
}

/// [`tangent_locus_relation`] with the margin the touch was decided on:
/// the span line's reading of the pair's separation
/// (`contact_tangent_touch`), the margin a verified pair's row records.
///
/// # Errors
///
/// [`ContactRefusal`] — see [`tangent_pair_relation`].
#[allow(clippy::too_many_arguments)]
pub(crate) fn tangent_locus_reading<T: Decide>(
    s1: &geom::Surface<T>,
    sense1: bool,
    s2: &geom::Surface<T>,
    sense2: bool,
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
    declared: bool,
    band: Band,
) -> Result<(ContactVerdict, geom_core::MarginDiag), ContactRefusal> {
    tangent_table(
        s1,
        sense1,
        s2,
        sense2,
        carrier,
        t0,
        t1,
        declared,
        Claim::Contact,
        band,
    )
    .map(|(verdict, touch, _)| (verdict, touch))
}

/// **The boolean glue door's tangency**: the C4 `Tangent` table along
/// `carrier` with no claim about the materials. It keeps the touch, the
/// witness on both surfaces, first-order parallelism and the
/// second-order margin, which must be DEFINITE (jet-determinate: no
/// declaration stands behind the pair to bridge an in-band or exactly
/// zero `κ_rel`). The outward normals may agree or oppose, and the
/// crescent between the sheets may be void or shared; both are
/// reported ([`Posture`]), and neither contradicts.
///
/// # Errors
///
/// [`ContactRefusal`] — see [`tangent_pair_relation`].
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
) -> Result<(geom_core::MarginDiag, Posture), ContactRefusal> {
    tangent_table(
        s1,
        sense1,
        s2,
        sense2,
        carrier,
        t0,
        t1,
        false,
        Claim::Tangency,
        band,
    )
    .map(|(_, touch, posture)| (touch, posture))
}

/// What a tangency reading saw of the two faces' senses along the locus,
/// read off every sample.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Posture {
    /// The outward normals agree along the locus.
    pub(crate) aligned: bool,
    /// The normals oppose and the crescent between the sheets is void at
    /// every sample: the posture of two materials touching from outside.
    pub(crate) void: bool,
}

/// What the table is asked to verify.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Claim {
    /// A contact: senses opposed and the crescent void (C4's `Tangent`).
    Contact,
    /// A tangency alone, claiming nothing of the materials.
    Tangency,
}

/// The C4 `Tangent` table along `carrier` under `claim`, with the
/// touch's margin and the senses' posture.
#[allow(clippy::too_many_arguments)]
fn tangent_table<T: Decide>(
    s1: &geom::Surface<T>,
    sense1: bool,
    s2: &geom::Surface<T>,
    sense2: bool,
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
    declared: bool,
    claim: Claim,
    band: Band,
) -> Result<(ContactVerdict, geom_core::MarginDiag, Posture), ContactRefusal> {
    // The lane gate FIRST: the demanded set is the certifiable set,
    // so a configuration whose span bounds do not exist gets no
    // per-sample verdict either — refusing here is the order-k
    // boundary, not a missing feature.
    if !tangent_certificate_lane(carrier, s1, s2) {
        return Err(ContactRefusal::NotCertifiable {
            what: "the (carrier kind, surface-kind pair) triple is outside the jet \
                   certificate's span-bound lane (the order-k boundary)",
        });
    }
    let bounds =
        tangent_span_bounds(s1, s2, carrier, t0, t1).ok_or(ContactRefusal::NotCertifiable {
            what: "the jet schedule's span bounds are unavailable for this configuration",
        })?;
    // The locus's honest spatial extent, capping every lever arm the
    // schedule uses — the same `edge_extent` cap the edge certifier
    // applies (`certify.rs`'s tangent arm), shared rather than
    // mirrored. Without it a planar carrier's curvature lever arm is
    // `f64::MAX` and every angular margin reads definite at a lever no
    // configuration is consumed over.
    let chord = carrier.eval(t0).distance(carrier.eval(t1));
    let extent = geom_brep::edge_extent(carrier, t0, t1, chord);
    let mut bridged = false;
    let mut gap_max = T::zero();
    let mut aligned = None;
    let mut void = true;
    for i in 0..CERT_SAMPLES {
        let t = sample_param(t0, t1, i);
        let (p, tau) = carrier.ders1(t);
        let (r1, r2) = (
            geom_brep::implicit_residual(s1, p),
            geom_brep::implicit_residual(s2, p),
        );
        // (1) Senses: a contact's two OUTWARD normals must point at each
        // other, since aligned normals on a shared tangency are
        // containment and a contact claiming it is a lie (the C1 lemma,
        // one dimension down from C3's patch clause). A tangency takes
        // either, reading the second face as reversed where they agree
        // so the touch below reads one convention.
        let n1: Vec3<T> = implicit_outward_normal(s1, sense1, p).vec();
        let n2: Vec3<T> = implicit_outward_normal(s2, sense2, p).vec();
        // Which way the shared normal points is metered over the
        // extent, as the edge certifier reads it (`tangent_gap_orientation`):
        // a surface curving on a scale near ε still has an orientation,
        // and its refusal is the second-order arm's, below.
        let arm = geom_brep::folded_lever_arm(s1, s2, p, extent);
        match crate::validate::decide(
            "contact_tangent_opposed",
            Margin::levered(n1.dot(n2), extent),
            band,
        ) {
            Ok(Sign::Negative) => {
                if aligned.replace(false) == Some(true) {
                    return Err(sides_mixed(band));
                }
            }
            Ok(Sign::Positive) if claim == Claim::Tangency => {
                if aligned.replace(true) == Some(false) {
                    return Err(sides_mixed(band));
                }
            }
            Ok(Sign::Positive) => {
                return Err(ContactRefusal::Contradicted {
                    diag: Indeterminate {
                        margin: geom_core::MarginDiag::INVALID,
                        band,
                        predicate: Some("contact_tangent_opposed"),
                        terminal_sliver: false,
                    },
                    steer: None,
                });
            }
            // A definite zero means the normals are perpendicular:
            // definite normal INDEPENDENCE, C4's first contradiction
            // trigger.
            Ok(Sign::Zero) => {
                return Err(ContactRefusal::Contradicted {
                    diag: Indeterminate {
                        margin: geom_core::MarginDiag::INVALID,
                        band,
                        predicate: Some("contact_tangent_independent"),
                        terminal_sliver: false,
                    },
                    steer: None,
                });
            }
            // MUST-VERIFY-DEFINITE, same reasoning as the residual
            // arm: opposed senses are on C4's first list, not its
            // third.
            Err(diag) => return Err(ContactRefusal::Escalated { diag }),
        }
        // (2) The touch, before the witness: the pair's separation
        // along the shared normal ([`touching`]), its orientation σ the
        // one (1) decided.
        let sense2 = sense2 != (aligned == Some(true));
        let separation = touching(r1, sense1, r2, sense2, band)?;
        gap_max = gap_max.max(separation.abs());
        // The witness on both surfaces, read across the spans each
        // sample bounds ([`on_surface`]).
        for (name, miss, r) in [
            ("contact_tangent_on_1", "contact_tangent_off_1", r1),
            ("contact_tangent_on_2", "contact_tangent_off_2", r2),
        ] {
            on_surface([name, miss], r, bounds.residual_sag, band)?;
        }
        let jet = tangent_jet(s1, s2, p, tau);
        // (3) and (4) are decided TOGETHER because C7's first-order
        // threshold is derived FROM the second-order margin: the lever
        // arm of the normal-parallelism test is 1/κ_rel, so whether
        // that lever exists is exactly the second-order question. The
        // second-order margin is therefore computed first and ACTED ON
        // last — computing it early must not let it refuse before the
        // first-order test (which can contradict) has spoken.
        let so = Margin::sagitta(jet.kappa_rel.abs() - bounds.kappa_drift, arm);
        // Both readings run detached so the second-order one's
        // escalation reaches the frame's log only where the outcome
        // rests on it (`k_stats::splice_superseded`, below).
        let (second_order, second_order_log) = geom_core::k_stats::detached(|| {
            crate::validate::decide("contact_tangent_second_order", so, band)
        });
        // (3) First-order tangency. With a definitely positive κ_rel
        // (an exact zero is the #175 neutral point: definite, and NOT a
        // usable lever) the lever is C7's own 1/κ_rel (`levered_inv`, verbatim what the edge
        // certifier uses). Without one the derived angle is unbounded,
        // which would make the test vacuous, so the margin falls back
        // to the folded arm (`geom_brep::folded_parallelism`, the one
        // spelling the edge certifier's namings read too). Here it is
        // the first-order verdict itself, so an in-band reading
        // escalates; there it only names a refusal already made.
        let parallel = if matches!(second_order, Ok(Sign::Positive)) {
            Margin::levered_inv(jet.sin_theta, jet.kappa_rel.abs())
        } else {
            geom_brep::folded_parallelism(jet.sin_theta, arm)
        };
        let (first_order, first_order_log) = geom_core::k_stats::detached(|| {
            crate::validate::decide("contact_tangent_parallel", parallel, band)
        });
        // An in-band second-order reading is overruled when the
        // first-order reading refuses on its own: a definite defect
        // contradicts, and an in-band one is the escalation returned.
        // Only a verified first order leaves the outcome resting on it.
        if second_order.is_err() && !matches!(first_order, Ok(Sign::Zero)) {
            geom_core::k_stats::splice_superseded(second_order_log);
        } else {
            geom_core::k_stats::splice(second_order_log);
        }
        geom_core::k_stats::splice(first_order_log);
        match first_order {
            Ok(Sign::Positive | Sign::Negative) => {
                return Err(ContactRefusal::Contradicted {
                    diag: Indeterminate {
                        margin: geom_core::MarginDiag::INVALID,
                        band,
                        predicate: Some("contact_tangent_parallel"),
                        terminal_sliver: false,
                    },
                    steer: None,
                });
            }
            Ok(Sign::Zero) => {}
            // First-order tangency is must-verify-DEFINITE (C4's first
            // list): an in-band angular defect escalates, declared or
            // not.
            Err(diag) => return Err(ContactRefusal::Escalated { diag }),
        }
        // (4) The second-order margin — THE bridged residue, and the
        // ONLY one (module docs, the #175 clause). Definitely positive
        // is the jet certificate's own verdict; an in-band κ_rel or an
        // exact zero at an isolated neutral point is what a
        // declaration carries and what a detector refuses.
        match second_order {
            // A definite κ_rel has a definite sign: the sheets leave the
            // locus one above the other, and a contact's materials must
            // be on the outer sides of that pair, not between it.
            Ok(Sign::Positive) if claim == Claim::Contact => {
                let crescent =
                    Margin::sagitta(geom_brep::material_kappa_rel(jet.kappa_rel, sense1), arm);
                match crate::validate::decide("contact_tangent_overlap", crescent, band) {
                    Ok(Sign::Negative) => {}
                    Ok(Sign::Positive | Sign::Zero) => {
                        return Err(ContactRefusal::Contradicted {
                            diag: Indeterminate {
                                margin: geom_core::MarginDiag::INVALID,
                                band,
                                predicate: Some("contact_tangent_overlap"),
                                terminal_sliver: false,
                            },
                            steer: None,
                        });
                    }
                    Err(diag) => return Err(ContactRefusal::Escalated { diag }),
                }
            }
            Ok(Sign::Positive) => {
                let crescent =
                    Margin::sagitta(geom_brep::material_kappa_rel(jet.kappa_rel, sense1), arm);
                void &= aligned == Some(false)
                    && matches!(
                        crate::validate::decide("contact_tangent_overlap", crescent, band),
                        Ok(Sign::Negative)
                    );
            }
            residue => {
                if !declared {
                    return Err(ContactRefusal::Escalated {
                        diag: residue.err().unwrap_or(Indeterminate {
                            margin: geom_core::MarginDiag::INVALID,
                            band,
                            predicate: Some("contact_tangent_second_order"),
                            terminal_sliver: false,
                        }),
                    });
                }
                bridged = true;
            }
        }
    }
    // The touch between samples: the worst sampled separation plus its
    // certified sag (`TangentSpanBounds::gap_sag`), the edge
    // certifier's `tangent_gap_hull_sup` statement. Every sample decided
    // on the surfaces, so a refusal here rests on the sag and carries
    // this reading.
    let touch = match geom_core::k_stats::decide_magnitude_reported(
        "contact_tangent_touch",
        Margin::of(gap_max + bounds.gap_sag),
        band,
    ) {
        Ok((Magnitude::Zero, margin)) => margin,
        Ok((Magnitude::Positive, margin)) => {
            return Err(ContactRefusal::Escalated {
                diag: Indeterminate {
                    margin,
                    band,
                    predicate: Some("contact_tangent_touch"),
                    terminal_sliver: false,
                },
            });
        }
        Err(diag) => return Err(ContactRefusal::Escalated { diag }),
    };
    let verdict = if bridged {
        ContactVerdict::Bridged
    } else {
        ContactVerdict::Definite
    };
    let posture = Posture {
        aligned: aligned == Some(true),
        void: void && aligned == Some(false),
    };
    Ok((verdict, touch, posture))
}

/// A tangency whose outward normals agree at one sample and oppose at
/// another: no single pair of sides runs along the locus.
fn sides_mixed(band: Band) -> ContactRefusal {
    ContactRefusal::Contradicted {
        diag: Indeterminate {
            margin: geom_core::MarginDiag::INVALID,
            band,
            predicate: Some("tangency_sides_mixed"),
            terminal_sliver: false,
        },
        steer: None,
    }
}

/// **The touch at a sample**: the two surfaces' separation along their
/// shared normal, read off the witness point's residuals `r1`, `r2`.
///
/// `s = r₁ − σ·r₂`, σ the sign of the two implicit gradients'
/// alignment. The opposed-senses reading has already decided the
/// outward normals opposed, so σ is the senses' `−ε₁ε₂` (εᵢ = ±1 the
/// face's bit), and `ε₁·s = ε₁r₁ + ε₂r₂` is the sum of the witness's
/// heights above each face (a residual read along the face's outward
/// normal): positive where the surfaces stand apart, negative where
/// each runs into the other's material. The witness's position along
/// the normal cancels, so a witness between the two surfaces reads
/// their whole gap rather than each half of it.
///
/// - Zero passes; the span between samples is the caller's line.
/// - A definite sign contradicts (`contact_tangent_gap`, its margin
///   signed: positive is C4's definite separation, negative its
///   definite crossing), with the `Fit` steer: a designed gap or
///   interference is what the geometry holds.
/// - In band escalates: the touch is MUST-VERIFY-DEFINITE (C4), never
///   the bridged residue.
///
/// Returns the signed separation.
fn touching<T: Decide>(
    r1: T,
    sense1: bool,
    r2: T,
    sense2: bool,
    band: Band,
) -> Result<T, ContactRefusal> {
    let height = |r: T, sense: bool| if sense { r } else { -r };
    let separation = height(r1, sense1) + height(r2, sense2);
    match crate::validate::decide_reported("contact_tangent_gap", Margin::of(separation), band) {
        Ok(geom_core::Decided {
            sign: Sign::Zero, ..
        }) => Ok(separation),
        Ok(geom_core::Decided { margin, .. }) => Err(ContactRefusal::Contradicted {
            diag: Indeterminate {
                margin,
                band,
                predicate: Some("contact_tangent_gap"),
                terminal_sliver: false,
            },
            steer: Some(FIT_DEFERRAL),
        }),
        Err(diag) => Err(ContactRefusal::Escalated { diag }),
    }
}

/// **The witness on a surface, read across the spans a sample bounds**:
/// `r` the sample's implicit residual and `sag` the span bound
/// (`TangentSpanBounds::residual_sag`). A span's interior stands within
/// its larger end residual plus `sag` (the edge certifier's
/// `tangent_hull_sup`), so each verdict is decided on its own side:
///
/// - verified on the far side, `|r| + sag` within ε (`on`), which at
///   every sample bounds every point of the locus;
/// - contradicted by the sample itself, `r` definitely nonzero (`off`):
///   a measured point of the witness off the surface is a miss whatever
///   the spans do, so the sag does not shrink it. The pair has already
///   been read touching there ([`touching`]), so the miss is the
///   witness's, a bad cache, and no gap could help: no `Fit` steer. The
///   refusal carries the `off` reading;
/// - escalated between. An in-band far side is its own escalation. A
///   far side decided beyond the band beside a sample decided on the
///   surface rests on the sag, so the refusal carries the `on` reading
///   and its margin; an in-band sample carries its own.
///
/// MUST-VERIFY-DEFINITE (C4): "the witness within ε of each surface".
/// The in-band residual is not the bridged residue — that list holds
/// in-band κ_rel and nothing else — so it escalates, declared or not.
fn on_surface<T: Decide>(
    [on, off]: [&'static str; 2],
    r: T,
    sag: T,
    band: Band,
) -> Result<(), ContactRefusal> {
    let reading = |predicate, margin| Indeterminate {
        margin,
        band,
        predicate: Some(predicate),
        terminal_sliver: false,
    };
    let far = geom_core::k_stats::decide_magnitude_reported(on, Margin::of(r.abs() + sag), band);
    let far_margin = match far {
        Ok((Magnitude::Zero, _)) => return Ok(()),
        Ok((Magnitude::Positive, margin)) => margin,
        Err(diag) => return Err(ContactRefusal::Escalated { diag }),
    };
    match crate::validate::decide_reported(off, Margin::of(r), band) {
        Ok(geom_core::Decided {
            sign: Sign::Positive | Sign::Negative,
            margin,
        }) => Err(ContactRefusal::Contradicted {
            diag: reading(off, margin),
            steer: None,
        }),
        Ok(geom_core::Decided {
            sign: Sign::Zero, ..
        }) => Err(ContactRefusal::Escalated {
            diag: reading(on, far_margin),
        }),
        Err(diag) => Err(ContactRefusal::Escalated { diag }),
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
        geom::Curve3::Line {
            origin: Point3::from_array(o),
            dir: Vec3::from_array(d),
        }
    }

    /// The certified-lane row: a cylinder resting on a plane, tangent
    /// along the contact ruling. Every must-verify condition is
    /// DEFINITE, so the verdict stands without the declaration doing
    /// any work — and the detector arm (`declared: false`) agrees,
    /// which is what "the declaration added nothing" means.
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
        for declared in [true, false] {
            assert_eq!(
                tangent_locus_relation(
                    &cyl,
                    true,
                    &flat,
                    true,
                    &ruling,
                    -1.0,
                    1.0,
                    declared,
                    band()
                )
                .unwrap(),
                ContactVerdict::Definite,
                "declared = {declared}"
            );
        }
    }

    /// The contradiction row: a sphere and a plane through its
    /// equator. The locus lies on BOTH surfaces, so nothing is
    /// separated — but the normals there are definitely INDEPENDENT
    /// (radial versus axial), which is a transverse intersection, not
    /// a tangency. The declaration loses: every definite verdict wins.
    #[test]
    fn definite_normal_independence_contradicts_the_declaration() {
        let sphere = Surface::Sphere {
            center: Point3::origin(),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let flat = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let equator = geom::Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let err = tangent_locus_relation(
            &sphere,
            true,
            &flat,
            false,
            &equator,
            0.0,
            std::f64::consts::TAU,
            true,
            band(),
        )
        .unwrap_err();
        match err {
            ContactRefusal::Contradicted { diag, .. } => assert_eq!(
                diag.predicate,
                Some("contact_tangent_independent"),
                "the refusal must name the margin that decided"
            ),
            other => panic!("expected Contradicted, got {other:?}"),
        }
    }

    /// **The #175 clause, pinned as its own row.** Two coincident
    /// opposite-sense planes are tangent along every line in them, and
    /// their relative curvature is EXACTLY zero everywhere — the
    /// degenerate limit of the G1 tube chain's neutral meridian
    /// points. The jet certificate is honestly indeterminate there.
    ///
    /// The fixture is the DEGENERATE LIMIT of that class, not an
    /// instance of it: two coincident planes have κ_rel ≡ 0 at every
    /// sample, where a G1 tube chain has it at two isolated points.
    /// The clause is therefore pinned by proxy — the arm under test
    /// (a definite-zero second-order margin, bridged only under a
    /// declaration) is the same arm either way, and this fixture needs
    /// no tube-chain constructor to reach it. A real chain row belongs
    /// with the first constructor that mints one.
    ///
    /// Three outcomes at one geometry, which is the whole point:
    /// DECLARED it BRIDGES (intent carries the non-crossing claim
    /// across exactly those samples); UNDECLARED it refuses typed (a
    /// detector never mints a candidate it cannot certify); and the
    /// certifiable configuration one row up is DEFINITE. Demanding
    /// jet-determinacy here would make every G1 chain undeclarable at
    /// its neutral points — #175 finding 1 with extra steps.
    #[test]
    fn exact_kappa_zero_bridges_when_declared_and_refuses_when_not() {
        let up = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let down = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let along = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert_eq!(
            tangent_locus_relation(&up, true, &down, false, &along, -1.0, 1.0, true, band())
                .unwrap(),
            ContactVerdict::Bridged,
            "declared: the in-band/zero second-order margin is the bridged residue"
        );
        let refusal =
            tangent_locus_relation(&up, true, &down, false, &along, -1.0, 1.0, false, band())
                .unwrap_err();
        assert!(
            matches!(refusal, ContactRefusal::Escalated { .. }),
            "undeclared: the same geometry refuses typed, never silently passes: {refusal:?}"
        );
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
            tangent_locus_relation(&cone, true, &flat, true, &along, -1.0, 1.0, true, band()),
            Err(ContactRefusal::NotCertifiable { .. })
        ));
    }

    /// AQ6's steer: a SEPARATION-shaped contradiction points at the
    /// class that would fit and names its deferral; an ANGULAR one
    /// does not, because no gap makes two non-parallel carriers one.
    #[test]
    fn fit_steer_fires_only_where_a_gap_could_help() {
        assert_eq!(
            fit_steer(Contradiction::SphereRadiiDiffer),
            Some(FIT_DEFERRAL)
        );
        assert_eq!(
            fit_steer(Contradiction::CylinderAxesApart),
            Some(FIT_DEFERRAL)
        );
        assert_eq!(fit_steer(Contradiction::CylinderAxesNotParallel), None);
        assert_eq!(fit_steer(Contradiction::PlanesNotParallel), None);
    }

    /// The on-surface gate's three verdicts, each on its own side of
    /// the `|r| ± sag` bracket: the verify side carries the sag, the
    /// contradiction side reads the sample alone. Each row a padding
    /// error could flip is named for the error that flips it.
    #[test]
    fn on_surface_pads_each_verdict_in_its_own_direction() {
        let b = band();
        let (eps, big) = (b.zero(), 2.0 * b.escalate());
        #[derive(Debug, PartialEq)]
        enum Want {
            On,
            Escalated,
            Contradicted,
        }
        for (r, sag, want, why) in [
            (0.0, 0.0, Want::On, "an exact sample, no sag"),
            (0.4 * eps, 0.4 * eps, Want::On, "the sum within ε"),
            (
                0.6 * eps,
                0.6 * eps,
                Want::Escalated,
                "each within ε, the sum in band",
            ),
            (
                0.0,
                big,
                Want::Escalated,
                "on at the sample, a sag beyond K·ε between",
            ),
            (
                big,
                1.5 * big,
                Want::Contradicted,
                "a definite miss under a larger sag",
            ),
            (
                -big,
                0.0,
                Want::Contradicted,
                "a definite miss on the other side",
            ),
            (
                0.6 * eps,
                big,
                Want::Escalated,
                "an in-band sample under a large sag",
            ),
        ] {
            let got = match on_surface(["probe_on", "probe_off"], r, sag, b) {
                Ok(()) => Want::On,
                Err(ContactRefusal::Escalated { diag }) => {
                    // The sample decided on the surface: the sag is the cause.
                    if r == 0.0 {
                        assert_eq!(diag.predicate, Some("probe_on"), "{why}");
                    }
                    Want::Escalated
                }
                Err(ContactRefusal::Contradicted { diag, .. }) => {
                    assert_eq!(diag.predicate, Some("probe_off"), "{why}");
                    assert!(!diag.margin.is_invalid(), "{why}: the measured margin");
                    Want::Contradicted
                }
                Err(other) => panic!("{why}: {other:?}"),
            };
            assert_eq!(got, want, "{why}: r = {r:e}, sag = {sag:e}");
        }
    }

    /// A unit cylinder resting on the plane `z = 0` with its axis lifted
    /// to `z = 1 + gap` (a negative gap sinks it in), read along the
    /// line at height `z = witness` under the ruling: the pair's touch
    /// and the witness's place are set apart.
    fn lifted_cylinder_on_plane(gap: f64, witness: f64) -> Result<ContactVerdict, ContactRefusal> {
        let cyl = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0 + gap),
            axis: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        let flat = plane([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let along = line([0.0, 0.0, witness], [1.0, 0.0, 0.0]);
        tangent_locus_relation(&cyl, true, &flat, true, &along, -1.0, 1.0, true, band())
    }

    /// The signed margin a refusal's reading carries.
    fn margin_of(diag: &Indeterminate) -> f64 {
        diag.margin
            .diagnostic_f64_for_error_text()
            .value()
            .expect("a decided point margin")
    }

    /// **The touch is read on the pair, not on each half of it.** A
    /// witness halfway between two surfaces stands within each by half
    /// their gap, so two residual rows each pass where the gap is under
    /// 2ε; the separation `s = r₁ − σ·r₂` reads the whole gap.
    ///
    /// - 1.8ε apart: each residual is 0.9ε, within ε; the gap is in
    ///   band and escalates under its own name.
    /// - 15ε apart: the gap is definite, a separation (positive), and
    ///   carries the `Fit` steer. Each residual (7.5ε) is in band, so a
    ///   decider reading only the residuals escalates here.
    /// - 15ε into each other: the gap is definite and negative, a
    ///   crossing, with the same steer.
    #[test]
    fn a_witness_between_the_surfaces_reads_their_whole_gap() {
        let eps = band().zero();
        match lifted_cylinder_on_plane(1.8 * eps, 0.9 * eps) {
            Err(ContactRefusal::Escalated { diag }) => {
                assert_eq!(
                    diag.predicate,
                    Some("contact_tangent_gap"),
                    "1.8ε: {diag:?}"
                );
            }
            other => panic!("1.8ε apart escalates on the gap: {other:?}"),
        }
        for (gap, sign, what) in [(15.0, 1.0, "separation"), (-15.0, -1.0, "crossing")] {
            match lifted_cylinder_on_plane(gap * eps, 0.5 * gap * eps) {
                Err(ContactRefusal::Contradicted { diag, steer }) => {
                    assert_eq!(
                        diag.predicate,
                        Some("contact_tangent_gap"),
                        "{what}: {diag:?}"
                    );
                    assert!(
                        margin_of(&diag) * sign > 0.0,
                        "{what}: the margin's sign names it: {diag:?}"
                    );
                    assert_eq!(steer, Some(FIT_DEFERRAL), "{what}: the Fit steer");
                }
                other => panic!("{what} by 15ε contradicts on the gap: {other:?}"),
            }
        }
    }

    /// **A witness off a touching pair is a bad witness, not a gap.**
    /// Displaced 11ε along the shared normal, the witness is 11ε out of
    /// the cylinder and 11ε above the plane, and the two heights cancel
    /// in the gap: the pair touches. The residual row contradicts,
    /// naming the first surface it reads, with no `Fit` steer, since no
    /// designed clearance is what the geometry holds.
    #[test]
    fn a_witness_off_a_touching_pair_contradicts_as_a_bad_witness() {
        let eps = band().zero();
        match lifted_cylinder_on_plane(0.0, 11.0 * eps) {
            Err(ContactRefusal::Contradicted { diag, steer }) => {
                assert_eq!(diag.predicate, Some("contact_tangent_off_1"), "{diag:?}");
                assert_eq!(steer, None, "a bad witness carries no Fit steer");
            }
            other => panic!("a witness 11ε off contradicts on its residual: {other:?}"),
        }
        assert_eq!(
            lifted_cylinder_on_plane(0.0, 0.0).unwrap(),
            ContactVerdict::Definite,
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
    /// rests on it: a definite first-order defect contradicts with an
    /// empty escalation log, and an in-band one is the log's only
    /// escalation. Where the outcome does rest on it (undeclared, the
    /// first order verified), the refusal carries the reading itself.
    #[test]
    fn an_overruled_second_order_escalation_leaves_the_log() {
        let b = band();
        let read = |phi: f64, declared: bool| {
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
            let out =
                tangent_locus_relation(&pin, true, &bore, false, &along, -1.0, 1.0, declared, b);
            let log = bracket.finish();
            let escalations: Vec<_> = log.escalations.iter().map(|e| e.source).collect();
            (out, escalations)
        };
        // A definite crossing: the parallelism defect contradicts.
        let (out, escalations) = read(1e-3, true);
        assert!(
            matches!(
                out,
                Err(ContactRefusal::Contradicted { diag, .. })
                    if diag.predicate == Some("contact_tangent_parallel")
            ),
            "a definite defect contradicts: {out:?}"
        );
        assert_eq!(escalations, vec![], "a contradiction leaves no escalation");
        // An in-band crossing: the parallel reading is the refusal.
        let (out, escalations) = read((3.0 * b.zero()).asin(), true);
        let Err(ContactRefusal::Escalated { diag }) = out else {
            panic!("an in-band defect escalates: {out:?}");
        };
        assert_eq!(diag.predicate, Some("contact_tangent_parallel"));
        assert_eq!(
            escalations,
            vec![diag],
            "the refusal is the log's escalation"
        );
        // A touch, undeclared: the second-order reading is the refusal.
        let (out, escalations) = read(0.0, false);
        let Err(ContactRefusal::Escalated { diag }) = out else {
            panic!("an undeclared in-band κ_rel escalates: {out:?}");
        };
        assert_eq!(diag.predicate, Some("contact_tangent_second_order"));
        assert_eq!(
            escalations,
            vec![diag],
            "the refusal is the log's escalation"
        );
    }
}
