//! **`OnPair` on a plane × spline pair**: incidence on both surfaces
//! through the plane × NURBS lane's limbs 1 and 2
//! ([`geom_brep::plane_nurbs_incidence`]), and nothing about how they
//! meet. The plane side is its closed-form residual at each sample; the
//! wall side the foot residual and the composite sup of
//! `|S(P(t)) − C(t)|` over the chart image.
//!
//! The fixture is M7-8's: the rational quarter-cylinder wall and a plane
//! through its `u = 0` ruling, and a flat patch lying in a plane, the
//! pair D2 leaves conventional (it under-determines their shared side).
//! The certificate states incidence alone, so the TANGENT plane `x = 1`
//! certifies too, though D2's honest form there is a
//! `TangentIntersection`: which form an edge must carry is tier 3's
//! question, not this certificate's. An `Approx` wall is metered against
//! its fit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::shared::fixture::{quarter_cylinder_wall, segment, transverse_plane};
use crate::shared::tol::band;
use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::keys::SurfaceKey;
use geom_brep::{
    CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec,
    PlaneNurbsRefusal, SurfacePair,
};
use geom_core::test_support::upper;
use geom_core::{Point3, Tol, Vec3};
use slotmap::SlotMap;

/// The `x = 1` plane, tangent to [`quarter_cylinder_wall`] along its
/// `u = 0` ruling.
fn tangent_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(1.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    }
}

/// The ruling, `(1, 0, z)` for `z ∈ [0, 1]`, displaced by `off` along
/// `x`: off the wall, and on the `y = 0` plane.
fn ruling(off: f64) -> NurbsCurve3<f64> {
    segment(
        Point3::new(1.0 + off, 0.0, 0.0),
        Point3::new(1.0 + off, 0.0, 1.0),
    )
}

/// Certifies `carrier` as an `OnPair` of `plane` and `wall` through the
/// door, with `lane`.
fn on_pair(
    plane: Surface<f64>,
    wall: Surface<f64>,
    carrier: NurbsCurve3<f64>,
    lane: Option<geom_brep::NurbsLane<f64>>,
) -> Result<EdgeCurve<f64>, CertifyError> {
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let (s1, s2) = (arena.insert(plane), arena.insert(wall));
    let ends = (carrier.eval(0.0), carrier.eval(1.0));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::on_pair(SurfacePair::new(s1, s2)),
        carrier: Curve3::Nurbs(Arc::new(carrier)),
        param_start: 0.0,
        param_end: 1.0,
    };
    EdgeCurve::certify_via(
        spec,
        ends.0,
        ends.1,
        move |k| arena.get(k).cloned(),
        band(),
        lane,
    )
}

fn wall() -> Surface<f64> {
    Surface::Nurbs(Arc::new(quarter_cylinder_wall()))
}

fn assert_on_pair(edge: &EdgeCurve<f64>, what: &str) {
    assert!(
        matches!(edge.description(), EdgeDescription::OnPair { .. }),
        "{what}: {:?}",
        edge.description()
    );
    assert!(
        edge.certificate().max_residual <= Tol::witness().get().eps,
        "{what}: the recorded residual is within ε: {:e}",
        edge.certificate().max_residual
    );
}

/// The ruling on the transverse plane certifies as an `OnPair`, its
/// recorded residual the lane's certified sup.
#[test]
fn a_ruling_on_a_transverse_plane_certifies_on_the_pair() {
    let edge = on_pair(
        transverse_plane(),
        wall(),
        ruling(0.0),
        Some(geom_brep::NurbsLane::certified()),
    )
    .expect("the ruling lies on both");
    assert_on_pair(&edge, "the transverse plane");
}

/// **Incidence alone.** The tangent plane meets the wall along the same
/// ruling with parallel normals, a tangency the wall's curvature
/// determines, so D2's honest form there is a `TangentIntersection`, and
/// the `Intersection` lane refuses it `NotTransverse`
/// (`m7_8_plane_nurbs_edge`). The pair's certificate still passes,
/// incidence being all it states: holding the edge to the form D2 asks
/// of it is tier 3's must-carry check, which no door here is asking.
#[test]
fn a_ruling_on_the_tangent_plane_certifies_incidence_alone() {
    let edge = on_pair(
        tangent_plane(),
        wall(),
        ruling(0.0),
        Some(geom_brep::NurbsLane::certified()),
    )
    .expect("the ruling lies on both");
    assert_on_pair(&edge, "the tangent plane");
}

/// **The wall-side falsifier.** The ruling moved to `x = 1 + off` stays
/// exactly on the `y = 0` plane and leaves only the wall, so the lane's
/// foot residual alone refuses it, with the measured miss.
#[test]
fn a_carrier_off_the_wall_refuses_with_the_measured_miss() {
    let off = 1e3 * Tol::witness().get().eps;
    match on_pair(
        transverse_plane(),
        wall(),
        ruling(off),
        Some(geom_brep::NurbsLane::certified()),
    ) {
        Err(CertifyError::PlaneNurbs(PlaneNurbsRefusal::Limb { margin, .. })) => {
            let value = upper(margin);
            assert!(value >= 0.5 * off, "the measured miss: {value:e}");
        }
        other => panic!("a carrier {off:e} m off the wall must refuse typed: {other:?}"),
    }
}

/// **The pair D2 leaves conventional.** A flat NURBS patch in the plane
/// `x = 0` and that plane itself: the two surfaces are one, so they
/// under-determine any locus on both, and the patch's `y = 0` side
/// rests on the pair.
#[test]
fn a_side_a_flat_patch_shares_with_its_plane_certifies_on_the_pair() {
    let edge = on_pair(
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        },
        Surface::Nurbs(Arc::new(flat_patch())),
        segment(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0)),
        Some(geom_brep::NurbsLane::certified()),
    )
    .expect("the side lies on both");
    assert_on_pair(&edge, "the coincident flat patch");
}

/// The flat patch `x = 0` over `y, z ∈ [0, 1]`.
fn flat_patch() -> NurbsSurface<f64> {
    let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsSurface::new(
        k.clone(),
        k,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 1.0),
        ],
        vec![1.0; 4],
    )
    .unwrap()
}

/// With no lane in hand the pair refuses typed, before any check of the
/// edge, as the plane × NURBS `Intersection` does.
#[test]
fn the_pair_with_no_lane_refuses_typed() {
    assert!(matches!(
        on_pair(transverse_plane(), wall(), ruling(0.0), None),
        Err(CertifyError::NurbsLaneNotSupplied)
    ));
}

/// **An `Approx` wall is metered against its fit.** The flat patch
/// `x = 0` over `y, z ∈ [0, 1]` offset by `d`: its fit is the plane
/// `x = ±d` itself, and its `y = 0` side lies on the `y = 0` plane.
#[test]
fn a_side_of_an_approx_wall_certifies_on_the_pair() {
    let approx =
        geom_brep::offset_fit::approx_offset_surface_at(Arc::new(flat_patch()), 0.05, 1e-6, band())
            .expect("the flat patch's offset fits");
    let side = segment(approx.eval(0.0, 0.0), approx.eval(0.0, 1.0));
    assert_eq!(
        (side.eval(0.0).y, side.eval(1.0).y),
        (0.0, 0.0),
        "the side lies on the y = 0 plane"
    );
    let edge = on_pair(
        transverse_plane(),
        approx,
        side,
        Some(geom_brep::NurbsLane::certified()),
    )
    .expect("the side lies on both");
    assert_on_pair(&edge, "the Approx wall");
}

/// Two spline charts have no incidence lane: the pair refuses as before.
#[test]
fn two_spline_charts_still_refuse_the_pair() {
    assert!(matches!(
        on_pair(
            wall(),
            Surface::Nurbs(Arc::new(quarter_cylinder_wall())),
            ruling(0.0),
            Some(geom_brep::NurbsLane::certified()),
        ),
        Err(CertifyError::Unimplemented)
    ));
}

/// **The fit, not the description.** The parabolic cylinder
/// `z = 3v²`, extruded along `x`, offset by `d = 0.05` and fitted
/// coarsely (to `1e-3`): every `v`-iso of the true offset is a straight
/// line in a plane `y = const`, and at `v = 0.75` the fit lies a
/// measured distance off it. The true offset line, on its plane, refuses
/// with that miss wherever the band reads it, and certifies where the
/// band is coarser: the lane meters the fit. A lane that metered the
/// description would certify it at every band.
#[test]
fn an_approx_wall_is_metered_against_its_fit() {
    let kv2 =
        geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = (0..9)
        .map(|k| {
            let (i, j) = (k / 3, k % 3);
            Point3::new(
                f64::from(i) * 0.5,
                f64::from(j) * 0.5,
                [0.0, 0.0, 3.0][j as usize],
            )
        })
        .collect();
    let base = NurbsSurface::new(kv2.clone(), kv2, control, vec![1.0; 9]).unwrap();
    let d = 0.05;
    let approx = geom_brep::offset_fit::approx_offset_surface_at(Arc::new(base), d, 1e-3, band())
        .expect("the parabolic cylinder's offset fits");
    // The true offset at `v`: `(x, v − 6v·d/n, 3v² + d/n)`, `n = √(1 + 36v²)`.
    let v = 0.75_f64;
    let n = (1.0 + 36.0 * v * v).sqrt();
    let (y, z) = (v - 6.0 * v * d / n, 3.0 * v * v + d / n);
    let line = segment(Point3::new(0.0, y, z), Point3::new(1.0, y, z));
    // The fit's distance from the line within the plane `y`, through
    // the fit's iso that meets the line's `y` at mid-span: an upper
    // bound on its distance from the fit.
    let mut w = v;
    for _ in 0..60 {
        w -= 0.9 * (approx.eval(0.5, w).y - y);
    }
    let miss = (0..=40)
        .map(|i| {
            let p = approx.eval(f64::from(i) / 40.0, w);
            (p.y - y).hypot(p.z - z)
        })
        .fold(0.0_f64, f64::max);
    let plane = Surface::Plane {
        origin: Point3::new(0.0, y, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let got = on_pair(plane, approx, line, Some(geom_brep::NurbsLane::certified()));
    println!("the fit lies {miss:e} off the description's line");
    if miss > 2.0 * band().zero() {
        match got {
            Err(CertifyError::PlaneNurbs(PlaneNurbsRefusal::Limb { margin, .. })) => {
                // The limb's margin is a foot distance at the sample it
                // refused at, at most the slice's miss.
                let value = upper(margin);
                assert!(
                    value > band().zero() && value <= miss,
                    "the refusal carries a miss off the fit within {miss:e}: {value:e}"
                );
            }
            other => panic!("the description's line lies {miss:e} off the fit: {other:?}"),
        }
    } else if miss < 0.5 * band().zero() {
        assert_on_pair(&got.expect("within the band of the fit"), "the coarse fit");
    }
}

/// The quarter-circle rim at `z = 0` of [`quarter_cylinder_wall`],
/// radius `r` about `z`, from `x̂` through `angle`.
fn rim(r: f64, angle: f64) -> (Curve3<f64>, f64) {
    (
        Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: r,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        angle,
    )
}

/// The `z = 0` plane, the wall's bottom cap.
fn cap() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, -1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// Certifies `carrier` over `[0, angle]` as an `OnPair` of the cap and
/// the wall, the wall's image `stated` where given.
fn arc_on_pair(
    (carrier, angle): (Curve3<f64>, f64),
    stated: Option<geom_brep::Pcurve<f64>>,
) -> Result<EdgeCurve<f64>, CertifyError> {
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let (s1, s2) = (arena.insert(cap()), arena.insert(wall()));
    let ends = (carrier.eval(0.0), carrier.eval(angle));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::on_pair(SurfacePair::new(s1, s2)),
        carrier,
        param_start: 0.0,
        param_end: angle,
    };
    let stated = stated.map(|pcurve| geom_brep::StatedImage {
        surface: s2,
        pcurve,
    });
    EdgeCurve::certify_via_stated(
        spec,
        ends.0,
        ends.1,
        move |k| arena.get(k).cloned(),
        band(),
        Some(geom_brep::NurbsLane::certified()),
        stated.as_ref(),
    )
}

/// **An arc rim certifies on its pair through its exact image.** The
/// fitted image's hull refuses a circle's foot path; the arc-rim
/// certificate compares the wall's boundary row with the circle in the
/// row's own spline space. The builder's candidate certifies, and so does
/// the same image stated, to the same bound.
#[test]
fn an_arc_rim_certifies_on_its_pair_through_its_exact_image() {
    let quarter = rim(1.0, core::f64::consts::FRAC_PI_2);
    let derived = arc_on_pair(quarter.clone(), None).expect("the rim lies on both");
    assert_on_pair(&derived, "the arc rim");
    let image = geom_brep::arc_rim_candidates(&quarter_cylinder_wall(), 0.0, quarter.1)
        .expect("the wall names its sub-arcs")
        .into_iter()
        .find(|p| {
            geom_brep::PcurveCache::certify(p.clone(), 0.0, quarter.1, &quarter.0, &wall(), band())
                .is_ok()
        })
        .expect("one candidate is the rim's row");
    let stated = arc_on_pair(quarter, Some(image)).expect("the stated image certifies");
    assert_eq!(
        stated.certificate().max_residual,
        derived.certificate().max_residual,
        "the stated image and the builder's are one image"
    );
}

/// **Off the wall, both causes.** The rim at radius `1 + off` lies on
/// the cap and `off` off the wall: its exact image refuses, and so does
/// the fitted path, with its measured miss.
#[test]
fn an_arc_rim_off_its_wall_refuses_with_both_causes() {
    let off = 1e3 * Tol::witness().get().eps;
    match arc_on_pair(rim(1.0 + off, core::f64::consts::FRAC_PI_2), None) {
        Err(CertifyError::OnPairArcRim { fitted, .. }) => {
            assert!(
                matches!(fitted, PlaneNurbsRefusal::Limb { margin, .. } if upper(margin) >= 0.5 * off),
                "the fitted path's measured miss: {fitted:?}"
            );
        }
        other => panic!("an arc rim {off:e} off its wall must refuse with both causes: {other:?}"),
    }
}

/// **A partial rim.** Half the quarter arc lies on the wall, but the
/// arc-rim certificate reads the whole boundary row, so its exact image
/// refuses typed; the fitted path decides the rest.
#[test]
fn a_partial_arc_rim_does_not_take_the_exact_image() {
    let half = rim(1.0, core::f64::consts::FRAC_PI_4);
    let image = geom_brep::arc_rim_candidates(&quarter_cylinder_wall(), 0.0, half.1)
        .expect("the wall names its sub-arcs");
    assert!(
        image.into_iter().all(|p| {
            geom_brep::PcurveCache::certify(p, 0.0, half.1, &half.0, &wall(), band()).is_err()
        }),
        "no full-row image holds a partial rim"
    );
    match arc_on_pair(half, None) {
        Ok(edge) => assert_on_pair(&edge, "the partial rim, by its fitted image"),
        Err(CertifyError::OnPairArcRim { .. }) => {}
        other => {
            panic!("a partial rim certifies by its fitted image or refuses both ways: {other:?}")
        }
    }
}

/// **A stated image is certified as it stands.** The wall's other row
/// stated as the rim's image refuses with both causes, though the
/// builder's own candidate would hold: a stated image is not a hint.
#[test]
fn a_wrong_stated_image_refuses_rather_than_falling_back() {
    let quarter = rim(1.0, core::f64::consts::FRAC_PI_2);
    let wrong = geom_brep::arc_rim_candidates(&quarter_cylinder_wall(), 0.0, quarter.1)
        .expect("the wall names its sub-arcs")
        .into_iter()
        .find(|p| {
            geom_brep::PcurveCache::certify(p.clone(), 0.0, quarter.1, &quarter.0, &wall(), band())
                .is_err()
        })
        .expect("a candidate that is not the rim's row");
    assert!(
        matches!(
            arc_on_pair(quarter, Some(wrong)),
            Err(CertifyError::OnPairArcRim { .. })
        ),
        "a wrong stated image refuses"
    );
}
