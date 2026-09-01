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
/// The samples are the certification schedule's
/// (`geom_brep::CERT_SAMPLES`, interior points only — an endpoint of a
/// closed rim is a seam site, not a sample), and every one must agree:
/// a rim whose own samples disagree about which configuration it is
/// escalates rather than picking the majority. That is the fold's rule
/// and it is imported, not restated.
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
    let mut aligned = true;
    let mut opposed = true;
    let mut jet_determinate = true;
    let mut side: Option<MaterialWedge> = None;
    let mut side_mixed = false;
    let n = geom_brep::CERT_SAMPLES;
    let phase = |i: u32| core::f64::consts::TAU * (f64::from(i) / f64::from(n));
    for i in 0..n {
        let theta = T::from_f64(phase(i));
        let (s, c) = (theta.sin(), theta.cos());
        let p = center + (u * c + v * s) * radius;
        // The rim's own tangent there: the derivative of the circle.
        let dir = (v * c - u * s) * radius;
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
        MaterialArmOutcome::Wedge(MaterialWedge::Transverse) => Err(Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some("material_wedge_side"),
        }),
        MaterialArmOutcome::Wedge(w) => Ok(RimRouting::Cusp(w)),
        MaterialArmOutcome::Lamina => Ok(RimRouting::Lamina),
        MaterialArmOutcome::Split { predicate } => Err(Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some(predicate),
        }),
    }
}

/// R2 review probes for MATE-7a (PR #1477). Not the unit's rows —
/// reviewer measurements of what the routing answers on inputs the
/// unit's own fixtures do not cover.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod r2_probes {
    use super::*;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn sphere() -> geom::Surface<f64> {
        geom::Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    fn cut_plane(sign: f64) -> geom::Surface<f64> {
        geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.6),
            normal: Vec3::new(0.0, 0.0, sign),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn cut_rim() -> Rim<f64> {
        Rim {
            center: Point3::new(0.0, 0.0, 0.6),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.8,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// **The routing has no TANGENCY screen.** `validate`'s check-4
    /// material arm — whose fold this module imports — is entered only
    /// after `classify_dihedral` reports every interior sample
    /// definitely `Smooth`. `classify_shared_rim` runs the fold with no
    /// such precondition, so a rim where the two surfaces genuinely
    /// CROSS is classified anyway. Fixture: the unit sphere and the
    /// plane `z = 0.6`, which meet at a definite 53 degrees.
    #[test]
    fn r2_a_plainly_transverse_rim_is_classified_as_the_smooth_seam() {
        let got = classify_shared_rim(&sphere(), 1.0, &cut_plane(1.0), 1.0, cut_rim(), 1.6, band());
        println!("[r2] transverse sphere/plane rim routes to {got:?}");
        assert_eq!(
            got.expect("the routing answers rather than escalating"),
            RimRouting::Seam,
            "R2: a definite crossing is reported as the wedge-pi smooth seam"
        );
    }

    /// The same crossing with the plane's stored normal REVERSED is
    /// sent to the other unbuilt arm: which arm a transverse rim earns
    /// depends on a stored sign, not on the geometry.
    #[test]
    fn r2_the_same_crossing_flips_arm_with_the_stored_normal() {
        let got =
            classify_shared_rim(&sphere(), 1.0, &cut_plane(-1.0), 1.0, cut_rim(), 1.6, band());
        println!("[r2] reversed-normal transverse rim routes to {got:?}");
        assert!(
            matches!(got, Ok(RimRouting::Cusp(_) | RimRouting::Lamina)),
            "R2: expected an opposed-side answer, got {got:?}"
        );
    }

    /// **The sample schedule is NOT the one the imported fold's other
    /// caller uses.** `validate.rs` takes `1..CERT_SAMPLES-1` (seven
    /// INTERIOR schedule parameters); this module takes
    /// `0..CERT_SAMPLES` (nine uniform phases, the first of which is
    /// the `u_ref` seam point the doc-comment says is excluded).
    #[test]
    fn r2_the_two_callers_of_the_fold_sample_differently() {
        let n = geom_brep::CERT_SAMPLES;
        let mine: Vec<u32> = (0..n).collect();
        let theirs: Vec<u32> = (1..(n - 1)).collect();
        println!("[r2] rim_wedge samples {mine:?}; validate check-4 samples {theirs:?}");
        assert_ne!(mine.len(), theirs.len());
        assert_eq!(
            mine[0], 0,
            "the first rim sample sits at theta = 0, the u_ref seam"
        );
    }
}
