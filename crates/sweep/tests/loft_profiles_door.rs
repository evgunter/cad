//! **[`sweep::loft_profiles`], the loft over sections already
//! validated at `T`.** Each section is a [`profile::ValidatedProfile`]
//! carrying its own plane, which is its placement; the structure is
//! selected on the sections' value channel
//! ([`profile::ValidatedProfile::value_channel`]), and no section is
//! validated again. At `f64` it is [`sweep::loft_body`] bit for bit; at
//! `Dual64` its values are the caller's, derivatives included; at
//! `Interval` a section or placement wider than a point encloses its
//! family. A value that is not finite refuses by name at both doors.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::enclosure::{assert_encloses, box_about};

use geom::Surface;
use geom_core::Point3;
use geom_core::{Decide, Dual64, Interval, Point2, Real, Tol};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{LoftError, Lofted, loft_body, loft_profiles};

/// The square `[−1, 1]²`, its right corner at `(r, ±1)` and its right
/// side an arc of bulge `b`, run counterclockwise.
fn section<T: Real>(r: T, b: T) -> Vec<ProfileLoop<T>> {
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    vec![bulge_loop(vec![
        (p(-1.0, -1.0), T::zero()),
        (Point2::new(r, T::from_f64(-1.0)), b),
        (Point2::new(r, T::one()), T::zero()),
        (p(-1.0, 1.0), T::zero()),
    ])]
}

/// The plane at height `z`.
fn at<T: Real>(z: T) -> SketchPlane<T> {
    SketchPlane::xy_at(Point3::new(T::zero(), T::zero(), z))
}

/// `section(r, b)` on the plane at `z`, validated at `T`.
fn validated<T: Decide>(r: T, b: T, z: T) -> ValidatedProfile<T> {
    Profile::new(at(z), section(r, b))
        .validate(Tol::witness())
        .expect("the section validates")
}

/// Every wall pole and weight of a loft, in face order, as bits of
/// their value channel.
fn wall_bits<T: geom_core::spline::SpanLocate>(lofted: &Lofted<T>) -> Vec<u64> {
    wall_values(lofted, |x| x.value_channel().to_bits())
}

/// Every wall pole and weight of a loft, in face order, through `f`.
fn wall_values<T: geom_core::spline::SpanLocate, R>(
    lofted: &Lofted<T>,
    f: impl Fn(T) -> R,
) -> Vec<R> {
    lofted
        .side_faces
        .iter()
        .flatten()
        .flatten()
        .flat_map(|face| {
            let face = lofted.body.get_face(*face).expect("a built face");
            match lofted
                .body
                .get_surface(face.surface)
                .expect("a described face")
            {
                Surface::Nurbs(n) => n
                    .control()
                    .iter()
                    .flat_map(|p| [p.x, p.y, p.z])
                    .chain(n.weights().iter().copied())
                    .map(&f)
                    .collect::<Vec<_>>(),
                other => panic!("a lofted wall is a NURBS surface, got {other:?}"),
            }
        })
        .collect()
}

/// At `f64` the door is `loft_body` over the same sections and
/// placements, bit for bit — a polygon and an arc-sided section alike.
#[test]
fn at_f64_loft_profiles_is_loft_body() {
    for b in [0.0, 0.2] {
        let profiles = [validated(1.0, b, 0.0), validated(1.2, b, 1.0)];
        let via_profiles = loft_profiles::<f64>(&profiles, 1, Tol::witness())
            .expect("the validated sections loft");
        let via_loops = loft_body::<f64>(
            &[section(1.0, b), section(1.2, b)],
            &[at(0.0), at(1.0)],
            1,
            Tol::witness(),
        )
        .expect("the loops loft");
        assert_eq!(via_profiles.section_params, via_loops.section_params);
        assert_eq!(wall_bits(&via_profiles), wall_bits(&via_loops), "b = {b}");
    }
}

/// At `Dual64` the walls are the `f64` walls on the value channel, and
/// a seeded section vertex moves the poles it sits under: the upper
/// section's right corner is `r`, so the wall poles at `x = r` carry
/// `∂x/∂r = 1`.
#[test]
fn at_dual_a_seeded_section_value_rides_into_the_walls() {
    let c = Dual64::constant;
    let profiles = [
        validated(c(1.0), c(0.0), c(0.0)),
        validated(Dual64::variable(1.2), c(0.0), c(1.0)),
    ];
    let lofted =
        loft_profiles::<Dual64>(&profiles, 1, Tol::witness()).expect("the Dual sections loft");
    let nominal = loft_profiles::<f64>(
        &[validated(1.0, 0.0, 0.0), validated(1.2, 0.0, 1.0)],
        1,
        Tol::witness(),
    )
    .expect("the f64 sections loft");
    assert_eq!(wall_bits(&lofted), wall_bits(&nominal));
    let seeded: Vec<f64> = wall_values(&lofted, |x| x)
        .into_iter()
        .filter(|x| x.value == 1.2)
        .map(|x| x.deriv)
        .collect();
    assert!(!seeded.is_empty(), "the upper right corner is a wall pole");
    assert!(
        seeded.iter().all(|d| *d == 1.0),
        "every pole at the seeded corner moves with it: {seeded:?}"
    );
}

/// At `Interval` a polygon whose values are points lofts to the
/// enclosure of the `f64` loft its value channel describes.
#[test]
fn at_interval_a_point_polygon_lofts_on_its_value_channel() {
    let i = Interval::from_f64;
    let lofted = loft_profiles::<Interval>(
        &[
            validated(i(1.0), i(0.0), i(0.0)),
            validated(i(1.2), i(0.0), i(1.0)),
        ],
        1,
        Tol::witness(),
    )
    .expect("the point polygons loft at Interval");
    let nominal = loft_profiles::<f64>(
        &[validated(1.0, 0.0, 0.0), validated(1.2, 0.0, 1.0)],
        1,
        Tol::witness(),
    )
    .expect("the f64 sections loft");
    assert_eq!(
        lofted
            .section_params
            .iter()
            .map(|u| geom_core::spline::SpanLocate::point_value(*u))
            .collect::<Vec<_>>(),
        nominal
            .section_params
            .iter()
            .map(|u| Some(*u))
            .collect::<Vec<_>>()
    );
    assert_eq!(wall_bits(&lofted), wall_bits(&nominal));
}

/// At `Interval` a section vertex wider than a point lofts in `T`, and
/// the walls hold the `f64` loft of every member sampled: the box's
/// ends and its middle.
#[test]
fn at_interval_a_wide_section_encloses_its_family() {
    let i = Interval::from_f64;
    let (lo, hi) = box_about(1.2);
    let enclosing = loft_profiles::<Interval>(
        &[
            validated(i(1.0), i(0.0), i(0.0)),
            validated(Interval::from_bounds(lo, hi), i(0.0), i(1.0)),
        ],
        1,
        Tol::witness(),
    )
    .expect("the wide polygon lofts at Interval");
    for r in [lo, 1.2, hi] {
        let sample = loft_profiles::<f64>(
            &[validated(1.0, 0.0, 0.0), validated(r, 0.0, 1.0)],
            1,
            Tol::witness(),
        )
        .expect("each member lofts at f64");
        assert_encloses(&format!("r = {r}"), &enclosing, &sample);
    }
}

/// At `Interval` a plane wider than a point is a placement over a box,
/// and the walls hold every sampled member's.
#[test]
fn at_interval_a_wide_plane_encloses_its_family() {
    let i = Interval::from_f64;
    let (lo, hi) = box_about(1.0);
    let enclosing = loft_profiles::<Interval>(
        &[
            validated(i(1.0), i(0.0), i(0.0)),
            validated(i(1.2), i(0.0), Interval::from_bounds(lo, hi)),
        ],
        1,
        Tol::witness(),
    )
    .expect("the polygon on a wide plane lofts at Interval");
    for z in [lo, 1.0, hi] {
        let sample = loft_profiles::<f64>(
            &[validated(1.0, 0.0, 0.0), validated(1.2, 0.0, z)],
            1,
            Tol::witness(),
        )
        .expect("each member lofts at f64");
        assert_encloses(&format!("z = {z}"), &enclosing, &sample);
    }
}

/// A section or placement holding a value that is not finite refuses
/// by name at `loft_body`; a form validated over a plane that is not
/// finite cannot exist, so none reaches `loft_profiles`.
#[test]
fn a_value_that_is_not_finite_refuses_by_name_at_both_doors() {
    match loft_body::<f64>(
        &[section(1.0, 0.0), section(f64::NAN, 0.0)],
        &[at(0.0), at(1.0)],
        1,
        Tol::witness(),
    ) {
        Err(LoftError::SectionNotFinite { section: 1 }) => {}
        other => panic!(
            "loft_body: expected section 1's refusal, got {:?}",
            other.map(|_| ())
        ),
    }
    match loft_body::<f64>(
        &[section(1.0, 0.0), section(1.2, 0.0)],
        &[at(0.0), at(f64::INFINITY)],
        1,
        Tol::witness(),
    ) {
        Err(LoftError::PlacementNotFinite { section: 1 }) => {}
        other => panic!(
            "loft_body: expected section 1's placement refusal, got {:?}",
            other.map(|_| ())
        ),
    }
    // `loft_profiles` takes forms already validated, and validation
    // refuses a plane that is not finite before it is a form.
    assert_eq!(
        Profile::new(at(f64::NAN), section(1.2, 0.0))
            .validate(Tol::witness())
            .map(|_| ()),
        Err(profile::ProfileError::PlaneNotFinite),
        "a NaN plane never reaches loft_profiles"
    );
}

/// The value channel of a form validated at `Dual64` is the form the
/// same profile validates to at `f64`: every stored value bit for bit,
/// every carried decision equal.
#[test]
fn a_dual_forms_value_channel_is_its_f64_form() {
    let c = Dual64::constant;
    let dual = validated(Dual64::variable(1.2), c(0.2), c(1.0))
        .value_channel()
        .expect("a Dual form's values are points");
    let nominal = validated(1.2, 0.2, 1.0);
    assert_eq!(format!("{dual:?}"), format!("{nominal:?}"));
}
