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
//! about code and is enforced by its shape: the on-surface residual
//! and the opposed-senses test return `Escalated` from an in-band
//! margin whether or not a declaration is in hand, and so does the
//! first-order parallelism test — those three are C4's
//! must-verify-DEFINITE list, and a list you may bridge is not a
//! must-verify list. Only the second-order arm consults `declared`.
//! In-band there is therefore a THIRD outcome, not a widened second
//! one: verified / bridged-by-declaration / escalated, with
//! contradiction sitting across all four arms.
//!
//! **What this lane does NOT check, named** (C4's "definite crossing
//! (II_rel definitely indefinite)"): crossing is decided here only
//! through the transverse `κ_rel` the jet computes along the locus
//! tangent's perpendicular. A full second-fundamental-form
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
    CERT_SAMPLES, sample_param, tangent_certificate_lane, tangent_jet, tangent_span_bounds,
};
use geom_core::{Band, Decide, Indeterminate, Margin, Sign, Vec3};

use crate::body::Body;
use crate::contact::{ContactClass, ContactRefusal, ContactVerdict, FIT_DEFERRAL};
use crate::entity::FaceKey;

use super::carrier_eq::{CarrierEqError, CarrierRelation};
use geom_core::Tol;

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
        Err(CarrierEqError::Undeclared { diag, .. }) => Err(ContactRefusal::Undeclared { diag }),
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
/// - **bridges** (declared only): the second-order margin and ONLY it
///   — in-band `κ_rel`, and exact zeros at isolated neutral points
///   (module docs, the #175 clause);
/// - **escalates**: an in-band margin on ANY must-verify arm
///   (residual, opposition, first-order parallelism), declared or not,
///   and the second-order residue when undeclared.
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
    let sa = if sense1 { T::one() } else { -T::one() };
    let sb = if sense2 { T::one() } else { -T::one() };
    // The locus's honest spatial extent, capping every lever arm the
    // schedule uses — the same `edge_extent` cap the edge certifier
    // applies (`certify.rs`'s tangent arm), shared rather than
    // mirrored. Without it a planar carrier's curvature lever arm is
    // `f64::MAX` and every angular margin reads definite at a lever no
    // configuration is consumed over.
    let chord = carrier.eval(t0).distance(carrier.eval(t1));
    let extent = geom_brep::edge_extent(carrier, t0, t1, chord);
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
                // MUST-VERIFY-DEFINITE (C4): "the locus on both
                // surfaces within ε". An in-band residual is neither
                // verified nor contradicted, and it is NOT the bridged
                // residue — that list holds in-band κ_rel and nothing
                // else. So it escalates, declared or not.
                Err(diag) => return Err(ContactRefusal::Escalated { diag }),
            }
        }
        let jet = tangent_jet(s1, s2, p, tau);
        // (2) Senses opposed: the two faces' OUTWARD normals must
        // point at each other. Aligned normals on a shared tangency
        // are containment, and a contact claiming it is a lie
        // (the C1 lemma, one dimension down from C3's patch clause).
        let n1: Vec3<T> = geom_brep::implicit_gradient(s1, p).normalize() * sa;
        let n2: Vec3<T> = geom_brep::implicit_gradient(s2, p).normalize() * sb;
        let arm = geom_brep::curvature_lever_arm(s1, p)
            .min(geom_brep::curvature_lever_arm(s2, p))
            .min(extent);
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
            // MUST-VERIFY-DEFINITE, same reasoning as the residual
            // arm: opposed senses are on C4's first list, not its
            // third.
            Err(diag) => return Err(ContactRefusal::Escalated { diag }),
        }
        // (3) and (4) are decided TOGETHER because C7's first-order
        // threshold is derived FROM the second-order margin: the lever
        // arm of the normal-parallelism test is 1/κ_rel, so whether
        // that lever exists is exactly the second-order question. The
        // second-order margin is therefore computed first and ACTED ON
        // last — computing it early must not let it refuse before the
        // first-order test (which can contradict) has spoken.
        let so = Margin::sagitta(jet.kappa_rel.abs() - bounds.kappa_drift, arm);
        let second_order_definite =
            match crate::validate::decide("contact_tangent_second_order", so, band) {
                Ok(Sign::Positive) => Some(true),
                // An exact zero is the #175 neutral point: definite,
                // and definitely NOT a usable lever.
                Ok(Sign::Zero | Sign::Negative) => Some(false),
                Err(_) => None,
            };
        // (3) First-order tangency. With a definite κ_rel the lever is
        // C7's own 1/κ_rel (`levered_inv`, verbatim what the edge
        // certifier uses). Without one the derived angle is unbounded,
        // which would make the test vacuous, so the margin falls back
        // to the ordinary angular door: the defect metered at the
        // extent over which the verdict is consumed. Both spellings
        // answer the same question and carry the same predicate name.
        let parallel = if second_order_definite == Some(true) {
            Margin::levered_inv(jet.sin_theta, jet.kappa_rel.abs())
        } else {
            Margin::levered(jet.sin_theta, arm)
        };
        match crate::validate::decide("contact_tangent_parallel", parallel, band) {
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
        match second_order_definite {
            Some(true) => {}
            residue => {
                if !declared {
                    return Err(ContactRefusal::Escalated {
                        diag: Indeterminate {
                            margin: geom_core::MarginDiag::Invalid,
                            band,
                            predicate: Some("contact_tangent_second_order"),
                        },
                    });
                }
                let _ = residue;
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use super::*;
    use geom::Surface;
    use geom_core::{Point3, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn plane(o: [f64; 3], n: [f64; 3]) -> Surface<f64> {
        Surface::Plane {
            origin: Point3::new(o[0], o[1], o[2]),
            normal: Vec3::new(n[0], n[1], n[2]),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn line(o: [f64; 3], d: [f64; 3]) -> geom::Curve3<f64> {
        geom::Curve3::Line {
            origin: Point3::new(o[0], o[1], o[2]),
            dir: Vec3::new(d[0], d[1], d[2]),
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
        let diag = |p| Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band: band(),
            predicate: Some(p),
        };
        assert_eq!(
            fit_steer(&diag("carrier_sphere_radius")),
            Some(FIT_DEFERRAL)
        );
        assert_eq!(
            fit_steer(&diag("carrier_cyl_axis_offset")),
            Some(FIT_DEFERRAL)
        );
        assert_eq!(fit_steer(&diag("carrier_cyl_axis_parallel")), None);
        assert_eq!(fit_steer(&diag("bool_plane_parallel")), None);
    }
}
