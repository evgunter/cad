//! **A loft computes its walls in its sections' scalar.** At `Sym` a
//! section arc whose bulge is a variable gives a skinned wall whose
//! weight `cos(Δ/2)` is that function of the variable, not an unknown
//! the `f64` skin was lifted through: it re-values with the variable to
//! the `f64` build's own bits, and the arc rim's weight margin, which
//! reads the wall's boundary column against the rim's own sweep
//! (`geom_brep`'s arc-rim class), is a symbolic Zero.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::sym::revalue::{record, revalue};
use geom_core::sym::with_session;
use geom_core::{Affine3, ParamSymbol, Point2, Real, Sym, SymBudget, Theorem, Tol, Vec3};
use profile::ProfileLoop;
use profile::test_support::bulge_loop;
use sweep::{Lofted, loft_body};

type S = Sym<f64>;

/// The square `[−1, 1]²` whose right side is an arc of bulge `b`, run
/// counterclockwise, so a positive `b` turns it inward.
fn section<T: Real>(b: T) -> Vec<ProfileLoop<T>> {
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    vec![bulge_loop(vec![
        (p(-1.0, -1.0), T::zero()),
        (p(1.0, -1.0), b),
        (p(1.0, 1.0), T::zero()),
        (p(-1.0, 1.0), T::zero()),
    ])]
}

/// Two sections at `z = 0` and `z = 1`.
fn loft<T: geom_core::Decide + topo::AtRestPolicy>(b: T) -> Lofted<T> {
    let places = [
        Affine3::identity(),
        Affine3::translation(Vec3::new(T::zero(), T::zero(), T::one())),
    ];
    loft_body::<T>(&[section(b), section(b)], &places, 1, Tol::witness())
        .expect("the arc-sided prism lofts")
}

/// Where the wall's net holds pole `iu` of section `iv` (u-major).
fn pole<T: Real>(wall: &geom::NurbsSurface<T>, iu: usize, iv: usize) -> usize {
    iu * wall.control_counts().1 + iv
}

/// The arc's wall: canonical segment 1, from `(1, −1)` to `(1, 1)`.
fn arc_wall<T: geom_core::spline::SpanLocate>(lofted: &Lofted<T>) -> geom::NurbsSurface<T> {
    let face = lofted
        .body
        .get_face(lofted.side_faces[0][1])
        .expect("a built face");
    match lofted
        .body
        .get_surface(face.surface)
        .expect("a described face")
    {
        Surface::Nurbs(n) => n.as_ref().clone(),
        other => panic!("a lofted wall is a NURBS surface, got {other:?}"),
    }
}

#[test]
fn a_lofted_arc_wall_is_a_function_of_the_sections_bulge() {
    let (b0, b1) = (0.2, 0.35);
    let symbol = ParamSymbol::new(0xb1e);
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let built_at_b1 = arc_wall(&loft::<f64>(b1));
    let at_b1 = |s: ParamSymbol| (s == symbol).then_some(b1);
    let ((weights, corner, rim_theorems), _) = with_session(budget, || {
        let (lofted, decisions) = record(|| loft::<S>(S::param(symbol, b0)));
        let wall = arc_wall(&lofted);
        let weights: Vec<Option<u64>> = wall
            .weights()
            .iter()
            .map(|w| revalue(w.node(), &at_b1).map(f64::to_bits))
            .collect();
        // The arc's tangent-intersection pole in the wall's bottom row.
        let corner = revalue(wall.control()[pole(&wall, 1, 0)].x.node(), &at_b1).map(f64::to_bits);
        // The iso-boundary checks' `Zero`s whose margin reads the bulge
        // (a constant re-values with no parameter bound): the arc rims'
        // weight margins and the class's other checks over the arc.
        let none = |_: ParamSymbol| None::<f64>;
        let rim_theorems: Vec<Option<Theorem>> = decisions
            .iter()
            .filter(|d| d.predicate == "pcurve_iso_boundary")
            .filter(|d| d.sign == Some(geom_core::Sign::Zero))
            .filter(|d| revalue(d.node, &none).is_none())
            .map(|d| d.theorem)
            .collect();
        (weights, corner, rim_theorems)
    });
    let want: Vec<Option<u64>> = built_at_b1
        .weights()
        .iter()
        .map(|w| Some(w.to_bits()))
        .collect();
    assert_eq!(
        weights, want,
        "the wall's weights re-value with the bulge to the f64 build's bits"
    );
    assert_eq!(
        corner,
        Some(built_at_b1.control()[pole(&built_at_b1, 1, 0)].x.to_bits()),
        "the wall's tangent-intersection pole re-values with the bulge"
    );
    assert_eq!(
        rim_theorems,
        vec![Some(Theorem::SymbolicZero); 10],
        "every Zero the arc rims' checks answer over the bulge, the weight margin `w − cos(h/2)` \
         among them, is a symbolic Zero"
    );
}

/// At `Dual` the same wall carries the bulge's derivative: its interior
/// weight is `cos(Δ/2)` with `Δ = 4·atan(b)` on one sub-arc, whose
/// derivative is `−sin(2·atan b)·2/(1 + b²)`.
#[test]
fn a_lofted_arc_walls_weight_carries_the_bulges_derivative() {
    use geom_core::Dual64;
    let b = 0.2;
    let wall = arc_wall(&loft(Dual64::variable(b)));
    let w = wall.weights()[pole(&wall, 1, 0)];
    let want = -(2.0 * b.atan()).sin() * 2.0 / (1.0 + b * b);
    assert!(
        (w.deriv - want).abs() <= 1e-12,
        "the wall's interior weight moves with the bulge: d/db = {}, want {want}",
        w.deriv
    );
}
