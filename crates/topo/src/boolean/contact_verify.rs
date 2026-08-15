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
//! goes down the carrier ladder ([`super::carrier_eq`]), `Tangent`
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
//! finding 1, reproduced with extra steps. The declaration carries
//! the non-crossing intent across exactly those samples and nothing
//! further: normal opposition and on-surface residual are still
//! must-verify-DEFINITE at every sample, so the weakening is confined
//! to the second-order margin.
//!
//! **What this lane does NOT check, named** (C4's "definite crossing
//! (II_rel definitely indefinite)"): crossing is decided here only
//! through the transverse `κ_rel` the jet computes along the locus
//! tangent's perpendicular. A full second-fundamental-form
//! indefiniteness test over all tangent directions needs machinery
//! that does not exist yet; the honest consequence is that a
//! declared tangency whose relative shape operator is indefinite in a
//! direction the jet does not sample is BRIDGED rather than
//! contradicted. This is a known weakening of the contradiction list,
//! not of the must-verify list.

use geom_brep::{
    CERT_SAMPLES, sample_param, tangent_certificate_lane, tangent_jet, tangent_span_bounds,
};
use geom_core::{Band, Decide, Indeterminate, Margin, Sign, Vec3};
use geom_curves::Curve3;

use crate::body::Body;
use crate::contact::{ContactClass, ContactRefusal, ContactVerdict, FIT_DEFERRAL};
use crate::entity::FaceKey;

use super::carrier_eq::{CarrierEqError, CarrierRelation};

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
        ContactClass::Rest => rest_pair_verdict(a, fa, b, fb, band),
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
/// `Fit` there would be advice that cannot work.
pub(super) fn fit_steer(diag: &Indeterminate) -> Option<&'static str> {
    matches!(
        diag.predicate,
        Some(
            "carrier_sphere_radius"
                | "carrier_cyl_radius"
                | "carrier_sphere_center"
                | "carrier_cyl_axis_offset"
                | "bool_plane_offset"
        )
    )
    .then_some(FIT_DEFERRAL)
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
fn rest_pair_verdict<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<ContactVerdict, ContactRefusal> {
    let outcome = super::rest::carrier_pair_verdict(a, fa, b, fb, true, band).ok_or(
        ContactRefusal::NotCertifiable {
            what: "a declared face's surface kind is outside the Rest ladder's inventory \
                   (plane, sphere, cylinder)",
        },
    )?;
    match outcome {
        Ok((CarrierRelation::SameOpposite, verdict)) => Ok(verdict),
        Ok((CarrierRelation::SameOriented, _)) => Err(ContactRefusal::Contradicted {
            diag: Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("contact_rest_senses_opposed"),
            },
            steer: None,
        }),
        // The ladder contradicts a declared pair before it can call
        // it `Distinct`; a `Distinct` here would be the ladder
        // breaking its own contract.
        Ok((CarrierRelation::Distinct, _)) => Err(ContactRefusal::Escalated {
            diag: Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("contact_rest_ladder_invariant"),
            },
        }),
        Err(CarrierEqError::Contradicted(diag)) => Err(ContactRefusal::Contradicted {
            steer: fit_steer(&diag),
            diag,
        }),
        Err(CarrierEqError::Escalated(diag)) => Err(ContactRefusal::Escalated { diag }),
        Err(CarrierEqError::Undeclared(diag)) => Err(ContactRefusal::Undeclared { diag }),
    }
}

/// **The Tangent verify door** — C4's `Tangent` table run over the
/// jet schedule.
///
/// Per sample on the certification schedule (the same 9 parameters,
/// through `geom_brep::sample_param`):
///
/// - **must-verify-DEFINITE**: the locus lies on BOTH surfaces (each
///   implicit residual not definitely nonzero), and the two outward
///   normals are definitely OPPOSED and parallel within the derived
///   angle at lever arm `1/κ_rel` — the C7 threshold verbatim;
/// - **contradicts**: definite separation (a residual definitely
///   nonzero), definite normal independence (`sin θ` definitely
///   nonzero at that arm), or aligned normals (containment, not
///   tangency);
/// - **bridges** (declared only): in-band `sin θ`, and the entire
///   second-order margin including exact `κ_rel` zeros (module docs).
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
    // The lane gate FIRST: the demanded set is the certifiable set,
    // so a configuration whose span bounds do not exist gets no
    // per-sample verdict either — refusing here is the order-k
    // boundary, not a missing feature.
    if !tangent_certificate_lane(carrier, s1, s2) {
        return Err(ContactRefusal::NotCertifiable {
            what: "the (carrier kind, surface-kind pair) triple is outside the jet \
                   certificate's span-bound lane (C3's order-k boundary)",
        });
    }
    let bounds =
        tangent_span_bounds(s1, s2, carrier, t0, t1).ok_or(ContactRefusal::NotCertifiable {
            what: "the jet schedule's span bounds are unavailable for this configuration",
        })?;
    let (sa, sb) = (face_a.sense_sign::<T>(), face_b.sense_sign::<T>());
    let mut bridged = false;
    for i in 0..CERT_SAMPLES {
        let t = sample_param(t0, t1, i);
        let p = carrier.eval(t);
        let tau = carrier.deriv(t);
        // (1) On both surfaces. The sag bound rides the residual so
        // the between-sample interior is covered by the same
        // certificate the edge lane uses.
        for (name, s) in [("contact_tangent_on_1", s1), ("contact_tangent_on_2", s2)] {
            let r = geom_brep::implicit_residual(s, p);
            let padded = Margin::of(r.abs() - bounds.residual_sag);
            match crate::validate::decide(name, padded, band) {
                Ok(Sign::Positive) => {
                    return Err(ContactRefusal::Contradicted {
                        diag: Indeterminate {
                            margin: geom_core::MarginDiag::Invalid,
                            band,
                            predicate: Some(name),
                        },
                        steer: Some(FIT_DEFERRAL),
                    });
                }
                Ok(Sign::Zero | Sign::Negative) => {}
                Err(diag) => {
                    if !declared {
                        return Err(ContactRefusal::Escalated { diag });
                    }
                    bridged = true;
                }
            }
        }
        let jet = tangent_jet(s1, s2, p, tau);
        // (2) Senses opposed: the two faces' OUTWARD normals must
        // point at each other. Aligned normals on a shared tangency
        // are containment, and a contact claiming it is a lie
        // (the C1 lemma, one dimension down from C3's patch clause).
        let n1: Vec3<T> = geom_brep::implicit_gradient(s1, p).normalize() * sa;
        let n2: Vec3<T> = geom_brep::implicit_gradient(s2, p).normalize() * sb;
        let arm = geom_brep::curvature_lever_arm(s1, p).min(geom_brep::curvature_lever_arm(s2, p));
        match crate::validate::decide(
            "contact_tangent_opposed",
            Margin::levered(n1.dot(n2), arm),
            band,
        ) {
            Ok(Sign::Negative) => {}
            Ok(Sign::Positive) => {
                return Err(ContactRefusal::Contradicted {
                    diag: Indeterminate {
                        margin: geom_core::MarginDiag::Invalid,
                        band,
                        predicate: Some("contact_tangent_opposed"),
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
                        margin: geom_core::MarginDiag::Invalid,
                        band,
                        predicate: Some("contact_tangent_independent"),
                    },
                    steer: None,
                });
            }
            Err(diag) => {
                if !declared {
                    return Err(ContactRefusal::Escalated { diag });
                }
                bridged = true;
            }
        }
        // (3) First-order tangency: normal parallelism within the
        // derived angle at lever arm 1/κ_rel (C7 verbatim — the same
        // `levered_inv` the edge certifier uses).
        match crate::validate::decide(
            "contact_tangent_parallel",
            Margin::levered_inv(jet.sin_theta, jet.kappa_rel.abs()),
            band,
        ) {
            Ok(Sign::Positive | Sign::Negative) => {
                return Err(ContactRefusal::Contradicted {
                    diag: Indeterminate {
                        margin: geom_core::MarginDiag::Invalid,
                        band,
                        predicate: Some("contact_tangent_parallel"),
                    },
                    steer: None,
                });
            }
            Ok(Sign::Zero) => {}
            Err(diag) => {
                if !declared {
                    return Err(ContactRefusal::Escalated { diag });
                }
                bridged = true;
            }
        }
        // (4) The second-order margin — THE bridged residue (module
        // docs, the #175 clause). Definitely positive is the jet
        // certificate's own verdict; anything else (in-band, or an
        // exact zero at an isolated neutral point) is what the
        // declaration carries, and what a detector refuses.
        let so = Margin::sagitta(jet.kappa_rel.abs() - bounds.kappa_drift, arm);
        match crate::validate::decide("contact_tangent_second_order", so, band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) => {
                if !declared {
                    return Err(ContactRefusal::Escalated {
                        diag: Indeterminate {
                            margin: geom_core::MarginDiag::Invalid,
                            band,
                            predicate: Some("contact_tangent_second_order"),
                        },
                    });
                }
                bridged = true;
            }
            Err(diag) => {
                if !declared {
                    return Err(ContactRefusal::Escalated { diag });
                }
                bridged = true;
            }
        }
    }
    Ok(if bridged {
        ContactVerdict::Bridged
    } else {
        ContactVerdict::Definite
    })
}
