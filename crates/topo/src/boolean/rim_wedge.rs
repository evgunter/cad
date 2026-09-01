//! **The shared-rim routing** — the ruling's material-wedge table read
//! at a cross-operand contact, before a body exists to validate.
//!
//! Two faces of two operands meeting along one circle are two
//! physically different situations wearing one description, and which
//! one a rim is decides what the boolean owes it:
//!
//! - **wedge π** — the outward normals agree across the rim, the
//!   composed material is smooth through it, and the rim is a SEAM of
//!   one composite wall. Nothing is declared and nothing is verified:
//!   the join's job there is structural.
//! - **wedge 0 or 2π** — the normals oppose, the material pinches to a
//!   knife edge or opens to a circular slit. That is the declared-cusp
//!   family, whose verification needs a certified witness along the
//!   rim; the witness lane has no torus arm, so the family is DEFINED
//!   AND UNBUILT and its refusal says exactly that.
//! - **anything the samples cannot settle** — escalates, naming the
//!   predicate that failed to decide. Never a silent verdict.
//!
//! **This is the tier-3 verdict table's own machinery, consumed one
//! stage earlier.** `validate`'s check-4 material arm asks the same
//! question of an EDGE of a finished body; the answer there is a fold
//! over per-sample `classify_material_pairing` and `tangent_jet`
//! readings, and both are functions of two SURFACES, two face senses,
//! a point and a direction — no topology at all. So the classification
//! ports to a cross-operand rim by supplying the rim curve in place of
//! the edge, and the fold is imported rather than restated: one table,
//! two callers, and a new wedge row cannot reach one and miss the
//! other.
//!
//! What does NOT port is the rim's own identification. A body's edge
//! IS its two faces' shared locus; two operands share nothing, so the
//! rim is found geometrically — a boundary edge of each face riding
//! the same circle carrier, decided on the carrier's own data.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Vec3};

use crate::validate::{MaterialArmOutcome, material_arm_outcome};
use crate::{Body, FaceKey};
use geom_brep::MaterialWedge;

/// A rim circle, in the data the curve carries: centre, unit axis,
/// radius and the seam reference the sampling phase starts from.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Rim<T: Real> {
    /// The circle's centre.
    pub center: Point3<T>,
    /// The unit axis.
    pub axis: Vec3<T>,
    /// The radius.
    pub radius: T,
    /// The unit reference direction where the parameter is zero.
    pub u_ref: Vec3<T>,
}

/// What the ruling's table routes a shared rim to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RimRouting {
    /// Wedge π: the smooth seam — the built arm.
    Seam,
    /// Wedge 0 or 2π: the declared-`Tangent` cusp family, defined and
    /// unbuilt.
    Cusp(MaterialWedge),
    /// Opposed sides whose jets osculate — conformal contact along the
    /// locus, which is a `Rest` claim wearing a rim's clothes.
    Lamina,
    /// Tangent planes definitely DISTINCT at every station: a genuine
    /// corner, which is the ordinary crossing lanes' business and not a
    /// rim contact at all. A first-order verdict, decided by the screen
    /// and never by the material arm.
    Transverse,
}

impl RimRouting {
    /// The wedge verdict behind the routing, where there is one.
    ///
    /// `None` for [`RimRouting::Lamina`]: an osculating rim has no
    /// wedge end at all — that is precisely what makes it a lamina —
    /// so naming one would be inventing a verdict the samples refused
    /// to give.
    pub(crate) fn wedge(self) -> Option<MaterialWedge> {
        match self {
            Self::Seam => Some(MaterialWedge::Seam),
            Self::Cusp(w) => Some(w),
            Self::Transverse => Some(MaterialWedge::Transverse),
            Self::Lamina => None,
        }
    }
}

/// The circle two faces' boundaries share — the rim the routing is
/// taken along, carried with the seam reference the curve itself
/// stores so no perpendicular has to be invented for it.
///
/// Decided on the carrier's own data at the run's band, which is the
/// same three data `carrier_eq` compares for a cylinder and for the
/// same reason: a circle is its centre, its axis LINE and its radius,
/// and the axis's direction sign carries no information about the
/// point set. Every candidate pair is examined and the FIRST match
/// wins; a face pair riding two common circles at once is not a rim
/// contact and is outside what this door claims to see.
pub(crate) fn shared_rim<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Option<Rim<T>> {
    for ra in face_boundary_circles(a, fa) {
        for rb in face_boundary_circles(b, fb) {
            let same = [
                ("rim_circle_center", Margin::norm3(ra.center - rb.center)),
                (
                    "rim_circle_axis_parallel",
                    Margin::levered(ra.axis.cross(rb.axis).norm(), T::one()),
                ),
                ("rim_circle_radius", Margin::of(ra.radius - rb.radius)),
            ]
            .into_iter()
            .all(|(name, margin)| {
                matches!(
                    crate::validate::decide(name, margin, band),
                    Ok(geom_core::Sign::Zero)
                )
            });
            if same {
                return Some(ra);
            }
        }
    }
    None
}

/// The circle carriers of a face's boundary edges.
fn face_boundary_circles<T: Real>(body: &Body<T>, face: FaceKey) -> Vec<Rim<T>> {
    let mut out = Vec::new();
    let Some(f) = body.get_face(face) else {
        return out;
    };
    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let Some(l) = body.get_loop(lk) else { continue };
        let crate::entity::LoopBoundary::Cycle { first } = l.boundary else {
            continue;
        };
        let Some(cycle) = body.loop_cycle(first) else {
            continue;
        };
        for he in cycle {
            let Some(h) = body.get_half_edge(he) else {
                continue;
            };
            let Some(e) = body.get_edge(h.edge) else {
                continue;
            };
            if let Some(crate::CurveGeom::Certified(c)) = body.get_curve_geom(e.curve)
                && let geom::Curve3::Circle {
                    center,
                    axis,
                    radius,
                    u_ref,
                } = *c.carrier()
            {
                out.push(Rim {
                    center,
                    axis,
                    radius,
                    u_ref,
                });
            }
        }
    }
    out
}

/// **The routing itself**: the wedge the two faces' material subtends
/// across the rim, sampled around the circle.
///
/// **The first-order screen runs FIRST, and it is the precondition the
/// imported fold is only valid behind.** Check 4's material arm reads
/// `sign(n̂₊ · n̂₋)`, which distinguishes one material side from two —
/// and NOTHING else. On a pair whose tangent planes are definitely
/// distinct that sign is still perfectly well defined and perfectly
/// meaningless: a sphere cut by a plane at 53° has aligned normals at
/// its rim and would read "seam", which is a false statement about the
/// geometry rather than a missing verdict. Tier 3 never asks the
/// question there because it reaches the material arm only after every
/// interior sample classified definitely `Smooth`; this door owes the
/// same screen, so `classify_dihedral` runs per sample before anything
/// second-order is computed. Importing a fold means importing what it
/// is defined over.
///
/// The five outcomes are the validator's own, one for one:
///
/// - every sample `Transverse` ⇒ [`RimRouting::Transverse`], set
///   DIRECTLY and never through the fold (which cannot produce it —
///   the material arm is not consulted on a genuine corner);
/// - every sample `Smooth` ⇒ the material arm, whose verdicts are the
///   seam, the two cusp ends and the lamina;
/// - samples that DISAGREE ⇒ escalated, naming `dihedral_wedge`: a rim
///   that is a corner in one place and a tangency in another is not
///   one contact, and no single routing is honest for it;
/// - a `classify_dihedral` escalation ⇒ passed through verbatim;
/// - a spline-chart face on either side ⇒ escalated: the implicit
///   gradient is poison on a fit, so neither the screen nor the arm can
///   run, and answering from a poisoned normal is worse than declining.
///
/// The samples are the certification schedule's
/// (`geom_brep::CERT_SAMPLES`), taken at uniform phase around the CLOSED
/// rim starting at its own `u_ref`. **That schedule differs from tier
/// 3's on purpose and the divergence is named at both ends**: an edge is
/// an open arc whose endpoints are vertices already classified by other
/// rules, so the validator samples its interior (`1..CERT_SAMPLES-1`); a
/// rim is a closed circle with no endpoint to exclude, so every sample
/// is interior to it and the phase-zero sample is an ordinary point of
/// the contact, not a boundary site. Sampling `1..n-1` here would drop
/// two of nine readings for a reason that does not apply. Every sample
/// must agree either way — a rim whose own samples disagree escalates
/// rather than being decided by majority, which is the fold's rule and
/// is imported, not restated.
///
/// `extent` is the lever arm the angular margins are metered at — the
/// contact's own reach, so a misalignment is priced as the
/// displacement it induces where the verdict is consumed.
///
/// # Errors
///
/// [`Indeterminate`] naming the predicate that could not decide.
pub(crate) fn classify_shared_rim<T: Decide>(
    s_plus: &geom::Surface<T>,
    sense_plus: T,
    s_minus: &geom::Surface<T>,
    sense_minus: T,
    rim: Rim<T>,
    extent: T,
    band: Band,
) -> Result<RimRouting, Indeterminate> {
    let Rim {
        center,
        axis,
        radius,
        u_ref: u,
    } = rim;
    let v = axis.cross(u);
    let n = geom_brep::CERT_SAMPLES;
    let phase = |i: u32| core::f64::consts::TAU * (f64::from(i) / f64::from(n));
    // The sample points and their rim tangents, derived once: the
    // first-order screen and the material arm must read the SAME
    // stations, or the screen certifies a rim the arm did not judge.
    let station = |i: u32| {
        let theta = T::from_f64(phase(i));
        let (s, c) = (theta.sin(), theta.cos());
        (
            center + (u * c + v * s) * radius,
            (v * c - u * s) * radius,
        )
    };

    // ---- The first-order screen (docs above): the precondition the
    // material arm is only defined behind. ----
    //
    // A spline chart on either side is exempt by KIND and says so: the
    // implicit gradient both stages read is poison on a fit, so this is
    // "the question cannot be posed here", not "the answer is no".
    if s_plus.spline_chart().is_some() || s_minus.spline_chart().is_some() {
        return Err(Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some("dihedral_wedge"),
        });
    }
    let mut all_transverse = true;
    let mut all_smooth = true;
    for i in 0..n {
        let (p, _) = station(i);
        match geom_brep::classify_dihedral(s_plus, s_minus, p, extent, band)? {
            geom_brep::DihedralClass::Transverse => all_smooth = false,
            geom_brep::DihedralClass::Smooth => all_transverse = false,
        }
    }
    if all_transverse {
        // Set DIRECTLY, exactly as the validator does: a genuine corner
        // is a first-order verdict and the material arm has no say in
        // it. This is also why the fold below can never return
        // `Transverse` — nothing routes to it there.
        return Ok(RimRouting::Transverse);
    }
    if !all_smooth {
        // Corner at one station, tangency at another: not one contact,
        // and no single routing is honest for it.
        return Err(Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some("dihedral_wedge"),
        });
    }

    // ---- The material arm, now validly posed. ----
    let mut aligned = true;
    let mut opposed = true;
    let mut jet_determinate = true;
    let mut side: Option<MaterialWedge> = None;
    let mut side_mixed = false;
    for i in 0..n {
        let (p, dir) = station(i);
        let arm = geom_brep::folded_lever_arm(s_plus, s_minus, p, extent);
        match geom_brep::classify_material_pairing(
            s_plus,
            sense_plus,
            s_minus,
            sense_minus,
            p,
            arm,
            band,
        )? {
            geom_brep::MaterialPairing::Aligned => opposed = false,
            geom_brep::MaterialPairing::Opposed => aligned = false,
        }
        let jet = geom_brep::tangent_jet(s_plus, s_minus, p, dir);
        match crate::validate::decide(
            "tangent_second_order",
            Margin::sagitta(jet.kappa_rel.abs(), arm),
            band,
        )? {
            geom_core::Sign::Positive => {}
            geom_core::Sign::Zero | geom_core::Sign::Negative => {
                jet_determinate = false;
                break;
            }
        }
        let signed = geom_brep::material_kappa_rel(jet.kappa_rel, sense_plus);
        let this = match crate::validate::decide(
            "material_cusp_side",
            Margin::sagitta(signed, arm),
            band,
        )? {
            geom_core::Sign::Positive => MaterialWedge::Cusp,
            geom_core::Sign::Negative => MaterialWedge::Slit,
            // The same quantity classified definitely nonzero one
            // decision above, so this cannot honestly land here.
            // Announced anyway — a state that cannot occur is reported,
            // never swallowed.
            geom_core::Sign::Zero => {
                return Err(Indeterminate {
                    margin: geom_core::MarginDiag::Invalid,
                    band,
                    predicate: Some("material_cusp_side"),
                });
            }
        };
        match side {
            Some(seen) if seen != this => side_mixed = true,
            _ => side = Some(this),
        }
    }
    match material_arm_outcome(aligned, opposed, jet_determinate, side, side_mixed) {
        MaterialArmOutcome::Wedge(MaterialWedge::Seam) => Ok(RimRouting::Seam),
        // `Transverse` is unreachable from the fold BY CONSTRUCTION —
        // the validator sets it directly off the first-order screen and
        // so does the screen above, so nothing routes to it here. Kept
        // total rather than `unreachable!`: a verdict this door cannot
        // account for is reported, never assumed away.
        MaterialArmOutcome::Wedge(MaterialWedge::Transverse) => Ok(RimRouting::Transverse),
        MaterialArmOutcome::Wedge(w) => Ok(RimRouting::Cusp(w)),
        MaterialArmOutcome::Lamina => Ok(RimRouting::Lamina),
        MaterialArmOutcome::Split { predicate } => Err(Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some(predicate),
        }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod redfirst {
    use super::*;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// RED-FIRST (MAJ-1): a unit sphere cut by z = 0.6 is a DEFINITE
    /// 53-degree crossing, not a tangency. The routing must not call
    /// it a seam.
    #[test]
    fn a_transverse_rim_is_not_a_seam() {
        let sphere = geom::Surface::Sphere {
            center: Point3::origin(),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let plane = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.6),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let rim = Rim {
            center: Point3::new(0.0, 0.0, 0.6),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.8,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let got = classify_shared_rim(&sphere, 1.0, &plane, 1.0, rim, 1.6, band());
        println!("transverse rim answers {got:?}");
        assert!(
            !matches!(got, Ok(RimRouting::Seam)),
            "a definite 53-degree crossing is not a smooth seam: {got:?}"
        );
        let flipped = classify_shared_rim(&sphere, 1.0, &plane, -1.0, rim, 1.6, band());
        println!("transverse rim, reversed sense, answers {flipped:?}");
        assert!(
            !matches!(flipped, Ok(RimRouting::Cusp(_))),
            "reversing a face sense cannot turn a crossing into a cusp: {flipped:?}"
        );
    }
}
