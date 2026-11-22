//! **A section circle certified inside both faces, built as a ring of
//! its sphere face** before the crossings path re-enters.
//!
//! The interior-loop guard ([`super::interior_loop_verdict`]) refuses a
//! pair whose section has a component inside both faces with no event on
//! it (the section certificate's R-loop): the join cuts only along what
//! the crossing layer saw, so the loop would go unbuilt. Where the pair
//! is a sphere face against a plane or another sphere, that component is
//! the carriers' one circle, known in closed form, and it is built where
//! it lies: the sphere face takes it as a ring, one closed edge through
//! one conventional vertex, which divides off the cap it bounds as a
//! face of its own. Re-entered, the circle is a closed edge lying in the
//! partner's face, which the join takes at its one site
//! (`boolean::join`'s one-site segments), and the cap and the rest are
//! sided like any other fragments.
//!
//! No seam is added. The ring is the section the result bounds its
//! faces along; it is not a cut of the face's chart, and it ends in the
//! result as the edge between the two operands' pieces.
//!
//! **Which side is the cap.** No event lies on the pair, so no edge of
//! the face crosses the circle, and every boundary vertex of the face is
//! read on one side of the circle's plane; the other side's cap holds no
//! boundary, lies inside the face, and is the new face. A face whose
//! boundary lies on both sides (the circle separating two of its loops),
//! or one a vertex of which the plane does not decide, keeps the guard's
//! refusal: so does any refusing pair that is not an R-loop of a sphere
//! against a plane or a sphere, and a face asked for two rings.

use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Tol, Vec3};

use super::super::section_cert::Refusal as SectionRefusal;
use super::{ChartCuts, CutsPerOperand, PairVerdict, SphereCutIn};
use crate::body::Body;
use crate::entity::FaceKey;
use crate::live::BoundaryMember;
use crate::validate::decide;
use super::super::{BooleanError, Operand};

/// One ring to build: the sphere face it divides, its circle, and the
/// partner surface the circle is the section with.
pub(super) struct InteriorRing<T: Real> {
    /// The operand holding the sphere face.
    operand: Operand,
    /// The sphere face the circle lies inside.
    face: FaceKey,
    /// The circle's centre.
    foot: Point3<T>,
    /// Its radius.
    rho: T,
    /// Its axis, unit, turned so the circle runs clockwise about the
    /// cap's outward normal: the face's ring runs along it, the cap's
    /// outer loop against it.
    axis: Vec3<T>,
    /// A unit direction across the circle from `foot` to its vertex.
    u_ref: Vec3<T>,
    /// The surface the circle is the section with, as the face's body
    /// will record it beside the face's sphere.
    partner: geom::Surface<T>,
}

/// **The rings the refusing pairs ask for**, or `None` where any of them
/// is not one this module builds (module docs): the guard's refusal then
/// stands.
///
/// # Errors
///
/// None of its own: a reading that does not decide answers `None`.
pub(super) fn plan<T: Decide + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    pairs: &[PairVerdict],
    band: Band,
) -> Result<Option<Vec<InteriorRing<T>>>, BooleanError> {
    let mut rings: Vec<InteriorRing<T>> = Vec::new();
    for p in pairs {
        match p.verdict {
            Ok(_) => continue,
            Err(SectionRefusal::Loop) => {}
            Err(_) => return Ok(None),
        }
        let sa = a.proven_face(p.a_face).surface().clone();
        let sb = b.proven_face(p.b_face).surface().clone();
        use geom::Surface as S;
        // The holder takes the ring: the plane face of a sphere-plane
        // pair, A's face of two spheres. The circle is read off the
        // sphere `x` against `y`; the partner is the other face's
        // surface, which the holder's body records beside its own.
        let (holder, x_surface, y_surface) = match (&sa, &sb) {
            (S::Plane { .. }, S::Sphere { .. }) => (Operand::B, &sb, &sa),
            (S::Sphere { .. }, S::Plane { .. }) => (Operand::A, &sa, &sb),
            (S::Sphere { .. }, S::Sphere { .. }) => (Operand::A, &sa, &sb),
            _ => return Ok(None),
        };
        let (x, face, own, partner) = match holder {
            Operand::A => (a, p.a_face, &sa, &sb),
            Operand::B => (b, p.b_face, &sb, &sa),
        };
        if rings.iter().any(|r| r.operand == holder && r.face == face) {
            return Ok(None);
        }
        let Some(circle) = circle_of(holder, face, x_surface, y_surface, band)? else {
            return Ok(None);
        };
        let Some(axis) = cap_axis(x, face, own, &circle, band) else {
            return Ok(None);
        };
        rings.push(InteriorRing {
            operand: holder,
            face,
            foot: circle.foot,
            rho: circle.rho,
            axis,
            u_ref: circle.u_ref,
            partner: partner.clone(),
        });
    }
    Ok((!rings.is_empty()).then_some(rings))
}

/// The section circle of sphere `x` with `y`, a plane or a sphere, as
/// the extent scan's cut-ins read it; `None` for any other partner, or
/// where the carriers' crossing does not decide.
fn circle_of<T: Decide>(
    holder: Operand,
    face: FaceKey,
    x: &geom::Surface<T>,
    y: &geom::Surface<T>,
    band: Band,
) -> Result<Option<SphereCutIn<T>>, BooleanError> {
    let &geom::Surface::Sphere { frame, radius } = x else {
        return Ok(None);
    };
    let center = frame.origin();
    Ok(match y {
        &geom::Surface::Plane { frame: plane } => {
            let normal = plane.axis().normalize();
            let Ok((side, s)) =
                super::ball_against_plane(center, radius, plane.origin(), normal, band)
            else {
                return Ok(None);
            };
            if !matches!(side, geom_core::NonzeroSign::Positive) {
                return Ok(None);
            }
            Some(SphereCutIn {
                operand: holder,
                face,
                center,
                radius,
                foot: center - normal * s,
                rho: ((radius - s.abs()) * (radius + s.abs())).sqrt(),
                normal,
                u_ref: plane.u_ref().normalize(),
            })
        }
        geom::Surface::Sphere { .. } => super::sphere_pair_cut((holder, face), x, y, band).ok(),
        _ => None,
    })
}

/// The circle's axis turned against the cap's outward normal: the cap
/// is the region of `face` the circle bounds that holds none of the
/// face's boundary, the disc inside it on a plane, and on a sphere the
/// side of the circle's plane no boundary vertex is read on. `None`
/// where the boundary is not read wholly off the cap, or the face's
/// normal over the cap does not decide.
fn cap_axis<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    surface: &geom::Surface<T>,
    circle: &SphereCutIn<T>,
    band: Band,
) -> Option<Vec3<T>> {
    let planar = matches!(surface, geom::Surface::Plane { .. });
    // Each boundary vertex's side: off the disc on a plane, of the
    // circle's plane on a sphere.
    let side_of = |p: Point3<T>| {
        let m = if planar {
            Margin::of((p - circle.foot).norm() - circle.rho)
        } else {
            Margin::of(geom::plane_offset(circle.foot, circle.normal, p))
        };
        decide("bool_interior_ring_side", m, band).ok()
    };
    let mut side: Option<Sign> = None;
    for (_, members, _) in body.proven_face(face).boundary_by_loop() {
        for member in members {
            let p = match member {
                BoundaryMember::Edge { half, .. } => {
                    body.resolve_vertex_point(half.start, crate::live::Proven)
                }
                BoundaryMember::Isolated { point, .. } => point,
            };
            let s = side_of(p)?;
            if s == Sign::Zero || side.is_some_and(|t| t != s) || (planar && s != Sign::Positive) {
                return None;
            }
            side = Some(s);
        }
    }
    // A point of the cap off its rim, and the way into the cap there.
    let (inside, into) = if planar {
        (circle.foot, circle.normal)
    } else {
        let into = match side? {
            Sign::Positive => -circle.normal,
            Sign::Negative | Sign::Zero => circle.normal,
        };
        (circle.center + into * circle.radius, into)
    };
    let outward = crate::face_normal::face_outward_normal_at(body, face, inside, band)
        .ok()??
        .vec();
    match decide(
        "bool_interior_ring_facing",
        Margin::of(outward.dot(into)),
        band,
    )
    .ok()?
    {
        Sign::Positive => Some(-into),
        Sign::Negative => Some(into),
        Sign::Zero => None,
    }
}

/// **Builds each ring** in its operand: a vertex inside the face at the
/// circle's `u_ref` point, the closed edge round the circle from it, the
/// face's ring along the circle and the cap's outer loop against it
/// (`MefSite::Lone`), the cap a new face on the sphere. The circle's
/// description is the section of the sphere with the partner surface,
/// which the body records beside it.
///
/// Returns what the rings made per operand, A's then B's, in the
/// cut-ins' record: each cap a division of its face, and each ring's
/// vertex and edge read as that face.
///
/// # Errors
///
/// The Euler operators' and the pcurve mint's refusals.
pub(super) fn apply<T: Decide + crate::props::AtRestPolicy>(
    a: &mut Body<T>,
    b: &mut Body<T>,
    rings: &[InteriorRing<T>],
    tol: Tol,
) -> Result<CutsPerOperand, BooleanError> {
    let mut cuts: CutsPerOperand = Default::default();
    for ring in rings {
        let body = match ring.operand {
            Operand::A => &mut *a,
            Operand::B => &mut *b,
        };
        let own = body.proven_face(ring.face).record().surface;
        let start = ring.foot + ring.u_ref * ring.rho;
        let carrier = geom::Curve3::circle(ring.foot, ring.axis, ring.rho, ring.u_ref);
        let turn = T::tau();
        let (vertex, lone) = super::super::vtxfac::ring_vertex_in(body, ring.face, start, tol)?;
        let partner = body.add_surface(ring.partner.clone());
        let created = body.mef(
            crate::euler::MefSite::Lone { r#loop: lone },
            geom_brep::EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    pair: geom_brep::SurfacePair::new(own, partner),
                    selector: geom_brep::CurveSelector::Witness(
                        carrier.mid_point(T::zero(), turn),
                    ),
                },
                carrier,
                param_start: T::zero(),
                param_end: turn,
            },
            crate::euler::FaceSurface::Inherit,
            tol,
        )?;
        let cut: &mut ChartCuts = &mut cuts[usize::from(ring.operand == Operand::B)];
        cut.divisions.push((created.face, ring.face));
        cut.rings.push((vertex, created.edge, ring.face));
    }
    for body in [a, b] {
        crate::pcurves::mint_pcurves(body, tol)
            .map_err(|source| BooleanError::Pcurves { source })?;
    }
    Ok(cuts)
}

/// Experiment: the refusing pairs as transient meridian cut-ins of their sphere faces.
pub(super) fn plan_cut_ins<T: Decide + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    pairs: &[PairVerdict],
    band: Band,
) -> Result<Option<Vec<SphereCutIn<T>>>, BooleanError> {
    let mut out = Vec::new();
    for p in pairs {
        match p.verdict {
            Ok(_) => continue,
            Err(SectionRefusal::Loop) => {}
            Err(_) => return Ok(None),
        }
        let sa = a.proven_face(p.a_face).surface().clone();
        let sb = b.proven_face(p.b_face).surface().clone();
        use geom::Surface as S;
        let (holder, face, x, y) = match (&sa, &sb) {
            (S::Sphere { .. }, _) => (Operand::A, p.a_face, &sa, &sb),
            (_, S::Sphere { .. }) => (Operand::B, p.b_face, &sb, &sa),
            _ => return Ok(None),
        };
        let Some(c) = circle_of(holder, face, x, y, band)? else { return Ok(None) };
        out.push(c);
    }
    Ok((!out.is_empty()).then_some(out))
}

/// Experiment: re-chart closed groups onto the circle's axis.
pub(super) fn plan_recharts<T: Decide + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    pairs: &[PairVerdict],
    band: Band,
) -> Result<Option<Vec<super::SphereRecut<T>>>, BooleanError> {
    let mut out: Vec<super::SphereRecut<T>> = Vec::new();
    for p in pairs {
        match p.verdict {
            Ok(_) => continue,
            Err(SectionRefusal::Loop) => {}
            Err(_) => return Ok(None),
        }
        let sa = a.proven_face(p.a_face).surface().clone();
        let sb = b.proven_face(p.b_face).surface().clone();
        use geom::Surface as S;
        let (holder, body, face, x, y) = match (&sa, &sb) {
            (S::Sphere { .. }, _) => (Operand::A, a, p.a_face, &sa, &sb),
            (_, S::Sphere { .. }) => (Operand::B, b, p.b_face, &sb, &sa),
            _ => return Ok(None),
        };
        let Some(c) = circle_of(holder, face, x, y, band)? else { return Ok(None) };
        let charts = crate::chart_groups::ChartGroups::of_body(body);
        let Some(representative) = crate::solid_contain::closed_sphere_group(body, face, &charts) else { return Ok(None) };
        let &S::Sphere { frame, .. } = x else { return Ok(None) };
        if out.iter().any(|r| r.operand == holder) { return Ok(None); }
        out.push(super::SphereRecut {
            operand: holder,
            representative,
            center: frame.origin(),
            arm: geom_brep::uniform_curvature_arm(x),
            axis: frame.axis(),
            align: c.normal,
            u_ref: frame.u_ref(),
            seam: None,
        });
    }
    Ok((!out.is_empty()).then_some(out))
}
