//! **The keel trees in the plane** (`a_keel_tree_in_the_plane.rs`): the
//! block `0..4 × 0..4 × 0..2` less V-grooves whose floors lie on the
//! line `y = 1`, `z = 1`, at any scalar.

use geom_core::{Decide, Point2, Point3, RigidMotion, Tol};
use profile::SketchPlane;
use sweep::test_support::{block, finished, prism_on};
use topo::{AtRestBody, AtRestPolicy, Body, BooleanResult, subtract};

/// A V of half-width `w` at `z = 2.5`, its tip on `z = 1` at `tip` in the
/// sketch's first coordinate, run `len` along the sketch normal of
/// `frame`.
pub fn groove<T: Decide + geom_core::Bounds + AtRestPolicy<Finished = AtRestBody<T>>>(
    frame: RigidMotion<T>,
    tip: f64,
    w: f64,
    len: f64,
    tol: Tol,
) -> Body<T> {
    let p = |u: f64, v: f64| (Point2::new(T::from_f64(u), T::from_f64(v)), T::zero());
    prism_on(
        SketchPlane::from_frame(frame),
        vec![p(tip - w, 2.5), p(tip, 1.0), p(tip + w, 2.5)],
        T::from_f64(len),
        tol,
    )
}

/// The block less each groove in turn.
pub fn grooved<T: Decide + geom_core::Bounds + AtRestPolicy<Finished = AtRestBody<T>>>(
    grooves: Vec<Body<T>>,
    tol: Tol,
) -> Body<T> {
    let mut body = block::<T>(4.0, 4.0, 2.0, tol);
    for (i, g) in grooves.into_iter().enumerate() {
        body = match subtract(
            &finished("the block", body, tol),
            &finished("a groove", g, tol),
            tol,
        ) {
            Ok(BooleanResult::Body(b)) => b.body.into_body(),
            other => panic!("the block less groove {i}: {other:?}"),
        };
    }
    body
}

/// Along `x`: the first groove from outside the block to `x = 2`, and a
/// wider one on to `x = 3`, both centred on `y = 1`.
pub fn deep<T: Decide + geom_core::Bounds + AtRestPolicy<Finished = AtRestBody<T>>>(
    tol: Tol,
) -> Vec<Body<T>> {
    let at = |x: f64| RigidMotion::axes_yz(Point3::new(T::from_f64(x), T::zero(), T::zero()));
    vec![
        groove(at(-1.0), 1.0, 0.3, 3.0, tol),
        groove(at(2.0), 1.0, 0.5, 1.0, tol),
    ]
}

/// [`deep`], and a groove along `y` from `y = 1` to `y = 2.5` at `x = 2`.
pub fn branching<T: Decide + geom_core::Bounds + AtRestPolicy<Finished = AtRestBody<T>>>(
    tol: Tol,
) -> Vec<Body<T>> {
    let mut grooves = deep(tol);
    // The zx frame's first coordinate is z; sketch in it with x as the
    // second, run along y.
    let p = |u: f64, v: f64| (Point2::new(T::from_f64(u), T::from_f64(v)), T::zero());
    grooves.push(prism_on(
        SketchPlane::from_frame(RigidMotion::axes_zx(Point3::new(
            T::zero(),
            T::one(),
            T::zero(),
        ))),
        vec![p(2.5, 1.6), p(1.0, 2.0), p(2.5, 2.4)],
        T::from_f64(1.5),
        tol,
    ));
    grooves
}
