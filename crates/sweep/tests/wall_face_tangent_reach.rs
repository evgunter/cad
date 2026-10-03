//! A curved wall resting on a plane face's interior along a ruling,
//! declared `Tangent` (`work/cleave/tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line.md`).
//!
//! A rod (r = 0.5, z ∈ [0.25, 0.75]) rests on the side face y = 2 of a
//! 4 × 4 × 1 plate, and the wall × face pairs are declared `Tangent`. The
//! union has material on both sides of the ruling x = 0, y = 2: #131's
//! doubled cusp, two coincident distinct slit edges on the locus, whose
//! arm is unbuilt. Two vertex-on-face records would certify two points of
//! that line, and a later union bridging the rod to the plate fused such
//! an answer into one shell with an edgeless contact every at-rest gate
//! passed. So the union refuses `TangentSlitArmUnbuilt`, at `f64` and at
//! `Interval`; subtract and intersect, which leave material on at most
//! one side of the ruling, still answer.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::test_support::finished;
use sweep::{Extrusion, extrude};
use topo::boolean::{BooleanDeclarations, BooleanResult, FacePairDeclaration};
use topo::{
    AtRestPolicy, Body, BooleanError, ContactClass, FaceKey, Operand, intersect_with,
    subtract_with, union_with,
};

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
    topo::transform_rigid(&b, &up, tol).unwrap()
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

/// The plate's face on y = 2 and the rod's cylinder faces, read off the
/// `f64` build: every scalar builds the same arenas in the same order.
fn declared_pairs() -> Vec<FacePairDeclaration> {
    let p = plate::<f64>();
    let side = p
        .faces()
        .find(|(_, f)| {
            matches!(p.get_surface(f.surface), Some(geom::Surface::Plane { origin, normal, .. })
                if normal.y == 1.0 && origin.y == 2.0)
        })
        .unwrap()
        .0;
    let r = rod::<f64>();
    let walls: Vec<FaceKey> = r
        .faces()
        .filter(|(_, f)| {
            matches!(
                r.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert!(!walls.is_empty(), "the rod has a wall");
    walls
        .into_iter()
        .map(|w| FacePairDeclaration::new(side, w, ContactClass::Tangent))
        .collect()
}

/// The declared union refuses, naming a declared pair and the plate as
/// the operand whose face the locus runs through; subtract answers the
/// plate (the rod only touches it) and intersect answers nothing.
fn the_declared_rest<T: geom_core::Decide + geom_core::Bounds + AtRestPolicy>(
    volume: impl Fn(&Body<T>) -> (f64, f64),
) {
    let tol = Tol::witness();
    let pairs = declared_pairs();
    let decls = BooleanDeclarations {
        coincident_faces: pairs.clone(),
        ..Default::default()
    };
    let (a, b) = (
        finished("the plate", plate::<T>(), tol),
        finished("the rod", rod::<T>(), tol),
    );
    match union_with(&a, &b, &decls, tol) {
        Err(BooleanError::TangentSlitArmUnbuilt {
            declaration,
            interior,
        }) => {
            assert!(
                pairs
                    .iter()
                    .any(|p| (p.a, p.b) == (declaration.a, declaration.b))
                    && declaration.class == ContactClass::Tangent,
                "union: names a declared pair: {declaration:?}"
            );
            assert_eq!(interior, Operand::A, "union: the plate's face");
        }
        other => panic!("union: TangentSlitArmUnbuilt, got {:?}", other.map(|_| ())),
    }
    match subtract_with(&a, &b, &decls, tol) {
        Ok(BooleanResult::Body(r)) => {
            let (lo, hi) = volume(&r.body);
            assert!(
                lo - 1e-9 <= 16.0 && 16.0 <= hi + 1e-9,
                "subtract: the plate's volume, got [{lo}, {hi}]"
            );
        }
        other => panic!("subtract: the plate, got {:?}", other.map(|_| ())),
    }
    assert!(
        matches!(
            intersect_with(&a, &b, &decls, tol),
            Ok(BooleanResult::Empty)
        ),
        "intersect: empty"
    );
}

#[test]
fn a_declared_tangent_rest_on_a_face_interior_refuses_its_union() {
    the_declared_rest::<f64>(|b| {
        let v = topo::mass_properties(b, Tol::witness()).unwrap().volume;
        (v, v)
    });
}

#[test]
fn a_declared_tangent_rest_on_a_face_interior_refuses_its_union_at_interval() {
    use geom_core::Bounds;
    the_declared_rest::<geom_core::Interval>(|b| {
        let v = topo::mass_properties(b, Tol::witness()).unwrap().volume;
        (v.lo(), v.hi())
    });
}
