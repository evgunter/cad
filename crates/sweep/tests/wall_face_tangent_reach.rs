//! A curved wall resting on a plane face's interior along a ruling.
//!
//! A rod (r = 0.5, z ∈ [0.25, 0.75]) rests on the side face y = 2 of a
//! 4 × 4 × 1 plate, tangent along the ruling x = 0, y = 2, its vertices
//! off the ruling. The union is the two solids touching along that ruling
//! through both faces' interiors, recorded as a face–face record of the
//! plate's face and the rod's wall (`crates/topo/README.md` C7), at `f64`
//! and at `Interval`; subtract answers the plate and intersect nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind;
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::test_support::finished;
use sweep::{Extrusion, extrude};
use topo::boolean::BooleanResult;
use topo::{AtRestPolicy, Body, intersect, subtract, union};

type Loop = Vec<((f64, f64), f64)>;

fn ext<T: geom_core::Decide + AtRestPolicy>(loops: Vec<Loop>, z0: f64, h: f64) -> Body<T> {
    let tol = Tol::witness();
    let lps = loops
        .into_iter()
        .map(|l| {
            bulge_loop(
                l.into_iter()
                    .map(|((x, y), b)| (Point2::new(x, y).map(T::from_f64), T::from_f64(b)))
                    .collect(),
            )
        })
        .collect();
    let vp = Profile::new(SketchPlane::<T>::xy(), lps)
        .validate(tol)
        .unwrap();
    let b = extrude(
        &vp,
        Extrusion::Distance {
            depth: T::from_f64(h),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    let up = Affine3::translation(Vec3::new(0.0, 0.0, z0).map(T::from_f64));
    topo::transform_rigid(&b, &geom_core::test_support::rigid(&up, tol), tol).unwrap()
}

fn plate<T: geom_core::Decide + AtRestPolicy>() -> Body<T> {
    let square = vec![
        ((-2.0, -2.0), 0.0),
        ((2.0, -2.0), 0.0),
        ((2.0, 2.0), 0.0),
        ((-2.0, 2.0), 0.0),
    ];
    ext(vec![square], 0.0, 1.0)
}

fn rod<T: geom_core::Decide + AtRestPolicy>() -> Body<T> {
    ext(vec![vec![((-0.5, 2.5), 1.0), ((0.5, 2.5), 1.0)]], 0.25, 0.5)
}

/// The union answers two solids and records the touch as a face–face
/// record of the plate's face y = 2 and the rod's wall; subtract answers
/// the plate (the rod only touches it) and intersect answers nothing.
fn the_tangent_rest<T: geom_core::Decide + geom_core::Bounds + AtRestPolicy>(
    volume: impl Fn(&Body<T>) -> (f64, f64),
) {
    let tol = Tol::witness();
    let (a, b) = (
        finished("the plate", plate::<T>(), tol),
        finished("the rod", rod::<T>(), tol),
    );
    match union(&a, &b, tol) {
        Ok(BooleanResult::Body(r)) => {
            assert_eq!(r.body.solids().count(), 2, "union: two solids");
            let (lo, hi) = volume(&r.body);
            let want = 16.0 + 0.125 * core::f64::consts::PI;
            assert!(
                lo - 1e-9 <= want && want <= hi + 1e-9,
                "union: both volumes, got [{lo}, {hi}]"
            );
            let kind = |f: topo::FaceKey| {
                let face = r.body.get_face(f).unwrap();
                r.body.get_surface(face.surface).unwrap().kind()
            };
            let ff: Vec<_> = r
                .contacts
                .face_face()
                .map(|(x, y)| (kind(x), kind(y)))
                .collect();
            assert!(
                ff == [(SurfaceKind::Plane, SurfaceKind::Cylinder)]
                    || ff == [(SurfaceKind::Cylinder, SurfaceKind::Plane)],
                "union: one face–face record, the plate's face and the rod's wall, got {ff:?}"
            );
        }
        other => panic!("union: two solids, got {:?}", other.map(|_| ())),
    }
    match subtract(&a, &b, tol) {
        Ok(BooleanResult::Body(r)) => {
            let (lo, hi) = volume(&r.body);
            assert!(
                lo - 1e-9 <= 16.0 && 16.0 <= hi + 1e-9,
                "subtract: the plate's volume, got [{lo}, {hi}]"
            );
            assert_eq!(
                r.contacts.face_face().count(),
                0,
                "subtract: the rod's wall is gone, and its record with it"
            );
        }
        other => panic!("subtract: the plate, got {:?}", other.map(|_| ())),
    }
    assert!(
        matches!(intersect(&a, &b, tol), Ok(BooleanResult::Empty)),
        "intersect: empty"
    );
}

#[test]
fn a_tangent_rest_on_a_face_interior_records_its_union() {
    the_tangent_rest::<f64>(|b| {
        let v = topo::mass_properties(
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(b),
                Tol::witness(),
            ),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .expect("the measured body")
        .met()
        .expect("the measured body")
        .volume;
        (v, v)
    });
}

#[test]
fn a_tangent_rest_on_a_face_interior_records_its_union_at_interval() {
    use geom_core::Bounds;
    the_tangent_rest::<geom_core::Interval>(|b| {
        let v = topo::mass_properties(
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(b),
                Tol::witness(),
            ),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .expect("the measured body")
        .met()
        .expect("the measured body")
        .volume;
        (v.lo(), v.hi())
    });
}
