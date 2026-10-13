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
use geom_core::{Affine3, ParamSymbol, Point2, Real, Sym, SymBudget, Tol, Vec3};
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
        let rim_theorems: Vec<bool> = decisions
            .iter()
            .filter(|d| d.predicate == "pcurve_iso_boundary")
            .filter(|d| d.sign == Some(geom_core::Sign::Zero))
            .filter(|d| revalue(d.node, &none).is_none())
            .map(|d| d.proven)
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
        vec![true; 10],
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

/// At `Interval` the loft encloses the `f64` skin of its sections'
/// value channel, so a section wider than a point has no skin to
/// enclose: it refuses at the structure pass, rather than standing for
/// the nominal surface over the family it spans.
#[test]
fn a_loft_over_a_section_wider_than_a_point_refuses_at_interval() {
    use geom_core::Interval;
    let b = Interval::from_bounds(0.19, 0.21);
    let places = [
        Affine3::identity(),
        Affine3::translation(Vec3::new(
            Interval::zero(),
            Interval::zero(),
            Interval::one(),
        )),
    ];
    match loft_body::<Interval>(&[section(b), section(b)], &places, 1, Tol::witness()) {
        Err(sweep::LoftError::Skin(sweep::SkinError::SectionProfile { section: 0, .. })) => {}
        other => panic!(
            "expected section 0's profile refusal, got {:?}",
            other.map(|_| ())
        ),
    }
}

/// The wall poles of every face a loft built, in face order, as bits of
/// their value channel.
fn wall_bits<T: geom_core::spline::SpanLocate>(lofted: &Lofted<T>) -> Vec<u64> {
    lofted
        .side_faces
        .iter()
        .flatten()
        .flat_map(|f| {
            let face = lofted.body.get_face(*f).expect("a built face");
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
                    .map(|x| x.value_channel().to_bits())
                    .collect::<Vec<_>>(),
                other => panic!("a lofted wall is a NURBS surface, got {other:?}"),
            }
        })
        .collect()
}

/// At `Interval` a sweep along a turning path is the `f64` sweep its
/// value channel describes, enclosed: the stations are composed at
/// `f64` and lifted, so the turn's sines and cosines, which are no
/// points at `Interval`, never reach the skin.
#[test]
fn a_turning_sweep_at_interval_encloses_its_f64_sweep() {
    use geom_core::Interval;
    use sweep::sweep_body;
    use sweep::test_support::{
        ELBOW_STATIONS, ELBOW_V_DEGREE, elbow_path, elbow_section, lift_sections,
    };
    let tol = Tol::witness();
    let profile = elbow_section();
    let nominal = sweep_body::<f64>(
        &profile,
        Affine3::identity(),
        &elbow_path(),
        ELBOW_STATIONS,
        ELBOW_V_DEGREE,
        tol,
    )
    .expect("the elbow sweeps at f64");
    let lane = lift_sections::<Interval>(core::slice::from_ref(&profile)).remove(0);
    let enclosed = sweep_body::<Interval>(
        &lane,
        Affine3::identity(),
        &elbow_path(),
        ELBOW_STATIONS,
        ELBOW_V_DEGREE,
        tol,
    )
    .expect("the elbow sweeps at Interval");
    assert_eq!(enclosed.section_params, nominal.section_params);
    assert_eq!(wall_bits(&enclosed), wall_bits(&nominal));
}

/// A placement wider than a point refuses by name, before any skin.
#[test]
fn a_placement_wider_than_a_point_refuses_by_name() {
    use geom_core::Interval;
    let places = [
        Affine3::identity(),
        Affine3::translation(Vec3::new(
            Interval::zero(),
            Interval::zero(),
            Interval::from_bounds(0.99, 1.01),
        )),
    ];
    let b = Interval::from_f64(0.2);
    match loft_body::<Interval>(&[section(b), section(b)], &places, 1, Tol::witness()) {
        Err(sweep::LoftError::PlacementNotAPoint { section: 1 }) => {}
        other => panic!(
            "expected section 1's placement refusal, got {:?}",
            other.map(|_| ())
        ),
    }
}

/// Every value of every wall a loft built, in face order: poles,
/// weights, then the v knots, each read through `f`.
fn wall_values<T: geom_core::spline::SpanLocate, R>(
    lofted: &Lofted<T>,
    f: impl Fn(T) -> R,
) -> Vec<R> {
    let mut out = Vec::new();
    for face in lofted.side_faces.iter().flatten() {
        let face = lofted.body.get_face(*face).expect("a built face");
        match lofted
            .body
            .get_surface(face.surface)
            .expect("a described face")
        {
            Surface::Nurbs(n) => {
                for p in n.control() {
                    out.extend([f(p.x), f(p.y), f(p.z)]);
                }
                out.extend(n.weights().iter().map(|w| f(*w)));
                out.extend(n.knot_values_v().values().iter().map(|k| f(*k)));
            }
            other => panic!("a lofted wall is a NURBS surface, got {other:?}"),
        }
    }
    out
}

/// A section placed at height `z` (its other entries built from `b`, so
/// they are `T`'s constants).
fn at_height<T: Real>(z: T) -> Affine3<T> {
    Affine3::translation(Vec3::new(T::zero(), T::zero(), z))
}

/// The skin of four section curves at `z = 0, 1, 2, 4`: the arc of
/// bulge `b` on the chord `(1, −1) → (1, 1)`, that chord straight, the
/// arc of bulge `2b`, and the arc of `b` again, at v-degree 2. The line
/// is elevated and every curve refined onto the union of their knots
/// (`make_compatible`), the parameters read the bulge through the
/// chord lengths, and the averaged v knots hold one interior knot.
fn strip_skin<T: geom_core::spline::ProjectiveScalar>(b: T) -> geom::NurbsSurface<T> {
    use geom_core::Arc2;
    use sweep::{SketchSegment, make_compatible, segment_curve, skin};
    let (a, e) = (
        Point2::new(T::one(), -T::one()),
        Point2::new(T::one(), T::one()),
    );
    let arc = |x: T| SketchSegment::Arc {
        a,
        b: e,
        arc: Arc2::from_chord(a, e, x),
    };
    let segs = [arc(b), SketchSegment::Line { a, b: e }, arc(b + b), arc(b)];
    let curves: Vec<geom::NurbsCurve3<T>> = segs
        .into_iter()
        .zip([0.0, 1.0, 2.0, 4.0])
        .enumerate()
        .map(|(i, (seg, z))| {
            segment_curve(i, seg, at_height(T::from_f64(z))).expect("a section curve")
        })
        .collect();
    let compatible = make_compatible(&curves).expect("the strip is made compatible");
    skin(&compatible, 2).expect("the strip skins")
}

/// The skin of a strip mixing a line with arcs: every pole, weight and
/// v knot re-values with the bulge to the `f64` build's bits, through
/// the degree raise, the knot merge, the chord-length parameters and
/// the averaged knots.
#[test]
fn a_mixed_strips_skin_re_values_its_parameters_and_merged_knots() {
    // Both builds keep one structure: the `b` arcs one sub-arc, the
    // `2b` arc two (sweeps 4·atan(2b) ≈ 2.2 and 2.4 rad), so the line
    // and the `b` arcs are refined onto its interior knot.
    let (b0, b1) = (0.3, 0.35);
    let symbol = ParamSymbol::new(0xb1f);
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let built = strip_skin(b1);
    let mut want: Vec<Option<u64>> = Vec::new();
    for p in built.control() {
        want.extend([p.x, p.y, p.z].map(|x| Some(x.to_bits())));
    }
    want.extend(built.weights().iter().map(|w| Some(w.to_bits())));
    want.extend(
        built
            .knot_values_v()
            .values()
            .iter()
            .map(|k| Some(k.to_bits())),
    );
    assert!(
        built.knot_values_v().values().len() > 6,
        "the strip's v knots hold an interior average"
    );
    let at_b1 = |s: ParamSymbol| (s == symbol).then_some(b1);
    let (got, _) = with_session(budget, || {
        let n = strip_skin(S::param(symbol, b0));
        let re = |x: S| revalue(x.node(), &at_b1).map(f64::to_bits);
        let mut got = Vec::new();
        for p in n.control() {
            got.extend([p.x, p.y, p.z].map(re));
        }
        got.extend(n.weights().iter().map(|w| re(*w)));
        got.extend(n.knot_values_v().values().iter().map(|k| re(*k)));
        got
    });
    assert_eq!(
        got, want,
        "every pole, weight and v knot re-values with the bulge to the f64 build's bits"
    );
}

/// A two-section prism whose top sits at height `h`.
fn lofted_at<T: geom_core::Decide + topo::AtRestPolicy>(h: T) -> Lofted<T> {
    let b = T::from_f64(0.2);
    loft_body::<T>(
        &[section(b), section(b)],
        &[Affine3::identity(), at_height(h)],
        1,
        Tol::witness(),
    )
    .expect("the prism lofts")
}

/// The elbow swept from a start frame lifted to height `h`.
fn swept_at<T: geom_core::Decide + topo::AtRestPolicy>(h: T) -> Lofted<T> {
    use sweep::test_support::{ELBOW_STATIONS, ELBOW_V_DEGREE, elbow_path, elbow_section};
    let profile: Vec<ProfileLoop<T>> = elbow_section()
        .iter()
        .map(|lp| lp.map_scalar(T::from_f64))
        .collect();
    sweep::sweep_body::<T>(
        &profile,
        at_height(h),
        &elbow_path(),
        ELBOW_STATIONS,
        ELBOW_V_DEGREE,
        Tol::witness(),
    )
    .expect("the elbow sweeps")
}

/// A caller's placement in `T` stays the function it was. At `Sym`, a
/// loft whose top section's height is a parameter gives walls that
/// re-value with it to the `f64` build's bits. At `Dual`, a sweep whose
/// start frame's height is the variable carries it into every station:
/// each wall pole's height moves one for one with it, and nothing else
/// moves. (At `Sym` the sweep's path frames are computed in `f64` and
/// enter as unknowns, so its walls re-value to nothing.)
#[test]
fn a_placement_in_t_stays_a_function_of_its_parameter() {
    use geom_core::Dual64;
    let (h0, h1) = (1.0, 1.25);
    let symbol = ParamSymbol::new(0x2e1);
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let want = wall_values(&lofted_at(h1), |x: f64| Some(x.to_bits()));
    let at_h1 = |s: ParamSymbol| (s == symbol).then_some(h1);
    let (got, _) = with_session(budget, || {
        wall_values(&lofted_at(S::param(symbol, h0)), |x: S| {
            revalue(x.node(), &at_h1).map(f64::to_bits)
        })
    });
    assert_eq!(
        got, want,
        "the loft's walls re-value with its top placement"
    );
    let swept = swept_at(Dual64::variable(h0));
    let moved: Vec<[f64; 4]> = swept
        .side_faces
        .iter()
        .flatten()
        .flat_map(|f| {
            let face = swept.body.get_face(*f).expect("a built face");
            match swept
                .body
                .get_surface(face.surface)
                .expect("a described face")
            {
                Surface::Nurbs(n) => n
                    .control()
                    .iter()
                    .zip(n.weights())
                    .map(|(p, w)| [p.x.deriv, p.y.deriv, p.z.deriv, w.deriv])
                    .collect::<Vec<_>>(),
                other => panic!("a swept wall is a NURBS surface, got {other:?}"),
            }
        })
        .collect();
    assert!(!moved.is_empty());
    for d in &moved {
        assert!(
            d[0].abs() <= 1e-12
                && d[1].abs() <= 1e-12
                && (d[2] - 1.0).abs() <= 1e-12
                && d[3] == 0.0,
            "every swept pole moves one for one with the start frame's height: {d:?}"
        );
    }
}
