//! **The at-rest rows**: a rim arc the whole reading carried, certified
//! again as an `Intersection` edge of its spline wall and its curved
//! partner. The edge certificate holds the carrier the reading fitted,
//! and refuses it once either surface stands off it by more than the
//! band. Each row names the mutant that turns it red, and runs at the
//! run's ε.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::test_support::fixture_lever;
use geom_core::{Point3, Vec3};

use super::carry_rows::{ball, band, listed, pt, wall};
use crate::certify::{CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec, NurbsLane};
use crate::description::{CurveSelector, EdgeDescriptionSpec, SurfacePair};
use crate::edge_nurbs::{PlaneNurbsRefusal, curved_nurbs_limbs};
use crate::keys::SurfaceKey;
use crate::pair_section::{CarriedComponent, SsiLane};
use crate::ssi::{PointLever, SsiLimb, SsiTube};

/// The cylinder along `x` about `(y, z) = (1.15·s, 1.6·s)` of radius
/// `0.5·s`: it cuts [`wall`] in one arc from side to side, its upper
/// crossing past the wall's top.
fn rod(s: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: pt(0.0, 1.15 * s, 1.6 * s),
        axis: Vec3::unit_x(),
        radius: 0.5 * s,
        u_ref: Vec3::unit_z(),
    }
}

/// The ring torus about the vertical axis through `(0, 4.15·s, 1.6·s)`,
/// of radii `3·s` and `0.5·s`: beside [`wall`] its tube runs along `x` as
/// [`rod`] does, bending away from the wall toward its sides.
fn ring(s: f64) -> Surface<f64> {
    Surface::Torus {
        center: pt(0.0, 4.15 * s, 1.6 * s),
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
        major_radius: 3.0 * s,
        minor_radius: 0.5 * s,
    }
}

fn chart(wall: &Surface<f64>) -> &NurbsSurface<f64> {
    let Surface::Nurbs(net) = wall else {
        unreachable!("the fixture's wall is a spline")
    };
    net
}

/// The edge certificate of `carrier` over `span` as the `Intersection` of
/// `partner` and `wall`, its witness the carrier's own middle.
fn at_rest(
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
    partner: &Surface<f64>,
    wall: &Surface<f64>,
    extent: f64,
) -> Result<EdgeCurve<f64>, CertifyError> {
    let witness = carrier.mid_point(t0, t1);
    witnessed(carrier, (t0, t1), witness, partner, wall, extent).0
}

/// [`at_rest`] under the witness `witness`, with the partner's key.
fn witnessed(
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
    witness: Point3<f64>,
    partner: &Surface<f64>,
    wall: &Surface<f64>,
    extent: f64,
) -> (Result<EdgeCurve<f64>, CertifyError>, SurfaceKey) {
    let mut arena = slotmap::SlotMap::<SurfaceKey, Surface<f64>>::with_key();
    let (k1, k2) = (arena.insert(partner.clone()), arena.insert(wall.clone()));
    let edge = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(k1, k2),
                selector: CurveSelector::Witness(witness),
            },
            carrier: carrier.clone(),
            param_start: t0,
            param_end: t1,
        },
        carrier.eval(t0),
        carrier.eval(t1),
        move |k| arena.get(k).cloned(),
        fixture_lever(extent),
        band(),
        NurbsLane::certified(),
    );
    (edge, k1)
}

/// Whether `refused` is the lane's own image missing the carrier's foot
/// path: limb 2 against the wall, definite at a break of its composite or
/// undecided there.
fn image_missed(refused: &CertifyError) -> bool {
    matches!(
        refused,
        CertifyError::PlaneNurbs(PlaneNurbsRefusal::Limb {
            limb: SsiLimb::ImageMiss,
            ..
        }) | CertifyError::Escalated {
            check: CertCheck::PlaneNurbsHullValue,
            ..
        }
    )
}

/// **A carried rim arc certifies at rest against its partner, and refuses
/// once either surface stands off it**: a sphere at `s = 0.01, 1, 10`, a
/// cylinder and a torus at `s = 1`, each against the lofted wall. The
/// carrier the whole reading fitted certifies as the pair's
/// `Intersection` edge, its tube proved on the wall's chart. With the
/// partner's radius a thousand ε larger the arc lies on the wall and off
/// the partner, and with the wall pushed out as far it lies on the
/// partner and off the wall: the certificate refuses each on the on-locus
/// residual, which the arc as fitted passes on both surfaces. With either
/// moved a thousandth of ε the cylinder's certificate reads as it does
/// unmoved, and its witness is held to the carrier's middle and to the
/// partner. A sphere's reports the least sine of its radius against the
/// wall's normal over the interior feet.
///
/// Where the arc bends in the chart the lane's image of it is the pcurve
/// mint's finest rung, which misses the foot path near the span's ends by
/// about `5e-10·s` metres, so the certificate holds where that clears ε
/// and otherwise refuses on its own image's miss, never on the carrier:
/// held here as a measured tripwire, per ε row
/// (`work/sectarms/a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung.md`).
/// The cylinder's arc runs along the wall's rulings, straight in the
/// chart, and holds at every row.
///
/// Red against: a resolver with no arm for the pair
/// (`CertifyError::Unimplemented`); the arm reading the wall's image at
/// degree 1, whose chords miss the arc's bend in the chart by a fraction
/// of a millimetre at every row; limbs read against one surface alone,
/// which an arc off the other passes.
#[test]
fn a_carried_rim_arc_certifies_at_rest_and_refuses_once_a_surface_stands_off_it() {
    let eps = band().zero();
    let cases = [
        ("the sphere", 1e-2, ball(1e-2, (0.0, 1.1, 0.0), 0.4)),
        ("the sphere", 1.0, ball(1.0, (0.0, 1.1, 0.0), 0.4)),
        ("the sphere", 10.0, ball(10.0, (0.0, 1.1, 0.0), 0.4)),
        ("the cylinder", 1.0, rod(1.0)),
        ("the torus", 1.0, ring(1.0)),
    ];
    for (what, s, partner) in &cases {
        let w = wall(*s);
        let comp = first(&listed(&w, partner));
        let extent = 2.0 * s;
        // The carrier as the reading fitted it, against a partner whose
        // radius is `dr` larger and a wall pushed out by `dy`.
        let against = |dr: f64, dy: f64| {
            let wall = Surface::Nurbs(std::sync::Arc::new(
                chart(&w).map_points(|p| p + Vec3::new(0.0, dy, 0.0)),
            ));
            at_rest(
                &comp.carrier,
                comp.span,
                &swollen(partner, dr),
                &wall,
                extent,
            )
        };
        let straight = matches!(partner, Surface::Cylinder { .. });
        let image_holds = straight || eps >= 1e-6 || (eps >= 1e-9 && *s <= 1.0);
        let near = [
            ("as fitted", (0.0, 0.0)),
            ("the partner a thousandth of ε off", (1e-3 * eps, 0.0)),
            ("the wall a thousandth of ε off", (0.0, 1e-3 * eps)),
        ];
        // The cylinder's certificate is the cheap one: it reads all three.
        for (moved, (dr, dy)) in near.into_iter().take(if straight { 3 } else { 1 }) {
            let read = against(dr, dy);
            assert!(
                match &read {
                    Ok(_) => image_holds,
                    Err(refused) => !image_holds && image_missed(refused),
                },
                "s = {s}, {what}, {moved}: the arc certifies at rest where its image clears ε \
                 ({image_holds}), and refuses on that image alone where it does not: {:?}",
                read.err()
            );
        }
        for (moved, (dr, dy)) in [
            ("the partner a thousand ε off", (1e3 * eps, 0.0)),
            ("the wall a thousand ε off", (0.0, 1e3 * eps)),
        ] {
            let read = against(dr, dy);
            assert!(
                matches!(
                    read,
                    Err(CertifyError::PlaneNurbs(PlaneNurbsRefusal::Limb {
                        limb: SsiLimb::OnLocus,
                        ..
                    }))
                ),
                "s = {s}, {what}, {moved}: the arc refuses on the on-locus residual: {read:?}"
            );
        }
        if let Surface::Cylinder { axis, .. } = *partner {
            // A witness is pinned to the carrier's middle and to the
            // partner: the middle slid along the cylinder's rulings stays
            // on the partner and leaves the middle; the wall's point
            // beside the middle, off the arc, leaves the partner.
            let (t0, t1) = comp.span;
            let mid = comp.carrier.mid_point(t0, t1);
            let (slid, _) = witnessed(
                &comp.carrier,
                comp.span,
                mid + axis * (1e3 * eps),
                partner,
                &w,
                extent,
            );
            assert!(
                matches!(
                    slid,
                    Err(CertifyError::ResidualExceeded {
                        check: CertCheck::WitnessMidpoint,
                        ..
                    })
                ),
                "s = {s}, {what}: a witness a thousand ε along the arc's partner is not the \
                 carrier's middle: {slid:?}"
            );
            let foot = chart(&w).project(mid).expect("the middle's foot settles");
            let beside = chart(&w).eval(foot.u, foot.v + 1e-3);
            let (off, key) = witnessed(&comp.carrier, comp.span, beside, partner, &w, extent);
            assert!(
                matches!(
                    off,
                    Err(CertifyError::ResidualExceeded {
                        check: CertCheck::WitnessSurfaceResidual { surface },
                        ..
                    }) if surface == key
                ),
                "s = {s}, {what}: a witness on the wall beside the arc is off the partner, \
                 named by its key: {off:?}"
            );
        }
        let (true, Surface::Sphere { center, .. }) = (image_holds, partner) else {
            continue;
        };
        let limbs = curved_nurbs_limbs(
            &comp.carrier,
            comp.span,
            partner,
            chart(&w),
            fixture_lever(extent),
            band(),
        )
        .unwrap_or_else(|e| panic!("s = {s}, {what}: the lane's limbs hold: {e:?}"));
        assert!(
            matches!(limbs.tube, SsiTube::Chart { .. }),
            "s = {s}, {what}: the tube is proved on the wall's chart: {:?}",
            limbs.tube
        );
        let sine = least_sine(&comp.carrier, comp.span, *center, chart(&w));
        assert!(
            (limbs.min_sin_theta - sine).abs() <= 1e-9,
            "s = {s}, {what}: the least sine the lane reports, {}, is the sphere's radius \
             against the wall's normal at the interior feet, {sine}",
            limbs.min_sin_theta
        );
    }
}

/// The section's first carried component.
fn first(section: &crate::pair_section::PairSection<f64>) -> CarriedComponent<f64> {
    section
        .carry(Some(SsiLane::marched()), band())
        .expect("the section carries")
        .components()
        .first()
        .expect("the section holds an arc")
        .clone()
}

/// `partner` with its radius, a torus's minor one, `dr` larger.
fn swollen(partner: &Surface<f64>, dr: f64) -> Surface<f64> {
    match partner.clone() {
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => Surface::Sphere {
            center,
            radius: radius + dr,
            axis,
            u_ref,
        },
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => Surface::Cylinder {
            origin,
            axis,
            radius: radius + dr,
            u_ref,
        },
        Surface::Torus {
            center,
            axis,
            u_ref,
            major_radius,
            minor_radius,
        } => Surface::Torus {
            center,
            axis,
            u_ref,
            major_radius,
            minor_radius: minor_radius + dr,
        },
        other => panic!("the rows' partners are curved: {:?}", other.kind()),
    }
}

/// The least sine, over the lane's 31 interior feet, of the angle between
/// the sphere's radius through a foot and the wall's normal there, each
/// foot the wall's nearest point to the carrier's.
fn least_sine(
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
    center: Point3<f64>,
    wall: &NurbsSurface<f64>,
) -> f64 {
    (1..32)
        .map(|i| {
            let p = carrier.eval(t0 + (t1 - t0) * f64::from(i) / 32.0);
            let foot = wall.project(p).expect("the foot settles");
            let jet = wall.ders(foot.u, foot.v);
            let (n, r) = (jet.du.cross(jet.dv), jet.point - center);
            r.cross(n).norm() / (r.norm() * n.norm())
        })
        .fold(f64::INFINITY, f64::min)
}

/// The flat wall `y = 1` over `x ∈ [−1, 1]`, `z ∈ [0, 2]`, bilinear.
fn flat() -> NurbsSurface<f64> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [-1.0, 1.0]
        .into_iter()
        .flat_map(|x| [0.0, 2.0].map(|z| pt(x, 1.0, z)))
        .collect();
    NurbsSurface::new(k(), k(), control, vec![1.0; 4]).unwrap()
}

/// **A sphere touching the wall has no `Intersection` edge there, read
/// at its own radius**: the sphere of radius `0.1` resting on the flat
/// wall at `(0, 1, 1)`, and a line of length ε on the wall through the
/// touch. At the first interior sample the sphere's radius and the wall's
/// normal are parallel to within the band over the sphere's curvature
/// radius, the shorter of it and the unit extent.
///
/// Red against: an arm read on the wall's curvature alone, which is flat
/// here and names the extent.
#[test]
fn a_sphere_touching_the_wall_is_not_transverse_at_its_curvature_radius() {
    let half = 0.5 * band().zero();
    let touch = Surface::Sphere {
        center: pt(0.0, 1.1, 1.0),
        radius: 0.1,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let line = Curve3::line(pt(0.0, 1.0, 1.0), Vec3::unit_x());
    let refused = curved_nurbs_limbs(
        &line,
        (-half, half),
        &touch,
        &flat(),
        fixture_lever(1.0),
        band(),
    )
    .expect_err("a tangent contact is no crossing");
    assert!(
        matches!(
            refused,
            PlaneNurbsRefusal::NotTransverse {
                sample: 1,
                lever: PointLever::CurvatureRadius,
                ..
            }
        ),
        "the first interior sample refuses on the sphere's curvature radius: {refused:?}"
    );
}

/// **The lane's door takes a sphere, a cylinder or a torus, and the
/// resolver sends a cone beside a spline nowhere**: a plane has its own
/// lane, and a cone's residual has no metres conversion at the rung-3
/// door, so each is refused by the lane as the wrong one, and a cone ×
/// spline `Intersection` stays unimplemented.
///
/// Red against: a door that admits any analytic partner.
#[test]
fn the_curved_lane_takes_no_plane_and_no_cone() {
    let cone = Surface::Cone {
        apex: pt(0.0, 3.0, 1.0),
        axis: -Vec3::unit_y(),
        half_angle: 0.3,
        u_ref: Vec3::unit_x(),
    };
    let plane = Surface::Plane {
        origin: pt(0.0, 0.0, 1.0),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let line = Curve3::line(pt(0.0, 1.0, 1.0), Vec3::unit_x());
    for (what, partner) in [("a plane", &plane), ("a cone", &cone)] {
        let refused = curved_nurbs_limbs(
            &line,
            (-0.5, 0.5),
            partner,
            &flat(),
            fixture_lever(1.0),
            band(),
        );
        assert!(
            matches!(refused, Err(PlaneNurbsRefusal::WrongLane { .. })),
            "{what} is not the curved lane's partner: {refused:?}"
        );
    }
    let flat = Surface::Nurbs(std::sync::Arc::new(flat()));
    let refused = at_rest(&line, (-0.5, 0.5), &cone, &flat, 1.0);
    assert!(
        matches!(refused, Err(CertifyError::Unimplemented)),
        "a cone × spline edge has no at-rest arm: {refused:?}"
    );
}
