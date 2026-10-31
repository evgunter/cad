//! **A loft computes its walls in its sections' scalar.** At `Sym` a
//! section arc whose bulge is a variable gives a skinned wall whose
//! weight `cos(Δ/2)` is that function of the variable, not an unknown
//! the `f64` skin was lifted through: it re-values with the variable to
//! the `f64` build's own bits, and the arc rim's weight margin, which
//! reads the wall's boundary column against the rim's own sweep
//! (`geom_brep`'s arc-rim class), is a symbolic Zero. At `Interval` a
//! loft over a section or placement wider than a point encloses the
//! family it spans, and what a carrier cannot hold refuses typed.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::enclosure::{assert_encloses, box_about};

use geom::Surface;
use geom_core::Point3;
use geom_core::sym::revalue::{record, revalue};
use geom_core::sym::with_session;
use geom_core::{ParamSymbol, Point2, Real, Sym, SymBudget, Tol};
use profile::ProfileLoop;
use profile::SketchPlane;
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
        SketchPlane::xy(),
        SketchPlane::xy_at(Point3::new(T::zero(), T::zero(), T::one())),
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
        .get_face(lofted.side_faces[0][1][0])
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

/// The quadrilateral `(−1, −1), (1, −1), (h, 1), (−1, 1)`, run
/// counterclockwise: every side a line, and only one corner moving.
fn quad<T: Real>(h: T) -> Vec<ProfileLoop<T>> {
    let corner = |x: T, y: f64| (Point2::new(x, T::from_f64(y)), T::zero());
    vec![bulge_loop(vec![
        corner(T::from_f64(-1.0), -1.0),
        corner(T::one(), -1.0),
        corner(h, 1.0),
        corner(T::from_f64(-1.0), 1.0),
    ])]
}

/// **At `Interval` a loft over a box encloses the family the box
/// spans.** A [`quad`] whose moving corner is at `x` in [`box_about`]`(1.2)`, at
/// heights `0`, `1` and `2` (its middle section the wide one), lofts
/// at v-degree 2 in `T`: its middle section's v-parameter is wider
/// than a point and enters only the collocation matrix, over the one
/// v-span a three-section Bezier skin has. Every wall value holds the
/// `f64` loft's at each end of the box and its middle.
#[test]
fn a_loft_over_a_wide_section_encloses_its_family_at_interval() {
    use geom_core::Interval;
    let ends = Interval::from_f64(1.2);
    let (lo, hi) = box_about(1.2);
    let wide = Interval::from_bounds(lo, hi);
    let places = [0.0, 1.0, 2.0].map(|z| at_height(Interval::from_f64(z)));
    let enclosing = loft_body::<Interval>(
        &[quad(ends), quad(wide), quad(ends)],
        &places,
        2,
        Tol::witness(),
    )
    .expect("the boxed square lofts at Interval");
    use geom_core::Bounds;
    use geom_core::spline::SpanLocate;
    let middle = enclosing.section_params[1];
    assert!(
        middle.point_value().is_none(),
        "the middle section's parameter is no single point: {middle:?}"
    );
    for h in [lo, 1.2, hi] {
        let sample = loft_body::<f64>(
            &[quad(1.2), quad(h), quad(1.2)],
            &[0.0, 1.0, 2.0].map(at_height),
            2,
            Tol::witness(),
        )
        .expect("each member lofts at f64");
        assert_encloses(&format!("h = {h}"), &enclosing, &sample);
        let p = sample.section_params[1];
        assert!(
            middle.lo() <= p && p <= middle.hi(),
            "h = {h}: the member's middle parameter {p} lies outside {middle:?}"
        );
    }
}

/// **A placement over a box encloses the family too**: the top quad
/// at a height in [`box_about`]`(1)` lofts at `Interval`, and its walls
/// hold the `f64` loft's at each end of the box and its middle.
#[test]
fn a_loft_over_a_wide_placement_encloses_its_family_at_interval() {
    use geom_core::Interval;
    let corner = Interval::from_f64(1.2);
    let (lo, hi) = box_about(1.0);
    let places = [
        at_height(Interval::zero()),
        at_height(Interval::from_bounds(lo, hi)),
    ];
    let enclosing =
        loft_body::<Interval>(&[quad(corner), quad(corner)], &places, 1, Tol::witness())
            .expect("the square lofts over a wide placement at Interval");
    for z in [lo, 1.0, hi] {
        let sample = loft_body::<f64>(
            &[quad(1.2), quad(1.2)],
            &[at_height(0.0), at_height(z)],
            1,
            Tol::witness(),
        )
        .expect("each member lofts at f64");
        assert_encloses(&format!("z = {z}"), &enclosing, &sample);
    }
}

/// **A sweep from a start frame over a box encloses its family**: the
/// start frame's offset along `x` in [`box_about`]`(0)`, three stations
/// along the elbow at v-degree 2, and every member's `f64` sweep held.
#[test]
fn a_sweep_from_a_wide_start_frame_encloses_its_family_at_interval() {
    use geom_core::Interval;
    use sweep::sweep_body;
    use sweep::test_support::elbow_path;
    let shift =
        |x: Interval| SketchPlane::xy_at(Point3::new(x, Interval::zero(), Interval::zero()));
    let (lo, hi) = box_about(0.0);
    let swept = sweep_body::<Interval>(
        &quad(Interval::from_f64(1.2)),
        shift(Interval::from_bounds(lo, hi)),
        &elbow_path(),
        3,
        2,
        Tol::witness(),
    );
    let enclosing = swept.expect("the quad sweeps from a wide start frame at Interval");
    for x in [lo, 0.0, hi] {
        let sample = sweep_body::<f64>(
            &quad(1.2),
            SketchPlane::xy_at(Point3::new(x, 0.0, 0.0)),
            &elbow_path(),
            3,
            2,
            Tol::witness(),
        )
        .expect("each member sweeps at f64");
        assert_encloses(&format!("x = {x}"), &enclosing, &sample);
    }
}

/// **A control row whose motion is not decided over the box refuses.**
/// Three lines whose start is pinned at the origin in the end sections
/// and boxed about it in the middle one: whether that row moves is not
/// the same for every member, and a member whose row moves counts it in
/// the chord-length average where one whose row is pinned does not. The
/// skin refuses rather than parameterize by the members that abstain.
#[test]
fn a_row_whose_motion_is_undecided_over_the_box_refuses() {
    use core::num::NonZeroUsize;
    use geom::NurbsCurve3;
    use geom_core::spline::{Knots, SpanLocate};
    use geom_core::{Interval, Point3};
    use sweep::{SkinError, skin};
    fn line<T: SpanLocate>(a: Point3<T>, b: Point3<T>) -> NurbsCurve3<T> {
        let two = NonZeroUsize::new(2).expect("two");
        NurbsCurve3::new(
            Knots::new(vec![(T::zero(), two), (T::one(), two)], 1).expect("clamped line knots"),
            vec![a, b],
            vec![T::one(); 2],
        )
        .expect("a line")
    }
    fn sections<T: SpanLocate>(x: T) -> Vec<NurbsCurve3<T>> {
        let p = |x: f64, z: f64| Point3::new(T::from_f64(x), T::zero(), T::from_f64(z));
        vec![
            line(p(0.0, 0.0), p(1.0, 0.0)),
            line(Point3::new(x, T::zero(), T::zero()), p(1.0, 1.0)),
            line(p(0.0, 0.0), p(1.0, 3.0)),
        ]
    }
    skin::<f64>(&sections(1e-6), 2).expect("a member whose row moves skins");
    match skin::<Interval>(&sections(Interval::from_bounds(-1e-6, 1e-6)), 2) {
        Err(SkinError::RowMotionUndecided { row: 0 }) => {}
        other => panic!("expected row 0's refusal, got {:?}", other.map(|_| ())),
    }
}

/// **An enclosure reaching an infinity is not finite**: a corner in
/// `[1.2, ∞]` refuses as the section, and a height in `[1, ∞]` as the
/// placement, before any skin.
#[test]
fn an_enclosure_reaching_an_infinity_refuses_as_not_finite() {
    use geom_core::Interval;
    use sweep::LoftError;
    let corner = Interval::from_f64(1.2);
    let points = [0.0, 1.0].map(|z| at_height(Interval::from_f64(z)));
    match loft_body::<Interval>(
        &[
            quad(corner),
            quad(Interval::from_bounds(1.2, f64::INFINITY)),
        ],
        &points,
        1,
        Tol::witness(),
    ) {
        Err(LoftError::SectionNotFinite { section: 1 }) => {}
        other => panic!("expected section 1's refusal, got {:?}", other.map(|_| ())),
    }
    match loft_body::<Interval>(
        &[quad(corner), quad(corner)],
        &[
            at_height(Interval::zero()),
            at_height(Interval::from_bounds(1.0, f64::INFINITY)),
        ],
        1,
        Tol::witness(),
    ) {
        Err(LoftError::PlacementNotFinite { section: 1 }) => {}
        other => panic!(
            "expected section 1's placement refusal, got {:?}",
            other.map(|_| ())
        ),
    }
}

/// **What the enclosure skin cannot carry refuses typed.** A carrier's
/// weights and knots are points at every scalar
/// (`work/inlane/an-enclosure-carrier-holds-point-weights-and-knots.md`):
/// - an arc's weight `cos(Δ/2)` is an enclosure at `Interval`, from a
///   wide bulge or from rounding alone, so an arc section refuses at the
///   carrier's door (`CarrierWeightNotAPoint`, until C6's S4/S5);
/// - three sections at v-degree 1 put their middle parameter in the
///   skin's knot vector: the fit encloses over a wide one (C6 S5c), and
///   the skinned wall's carrier refuses it at its door, before any slab
///   is cut (`CarrierKnotNotAPoint`,
///   until C6's S5z).
#[test]
fn what_a_carrier_cannot_hold_refuses_typed_at_interval() {
    use geom_core::Interval;
    use geom_core::spline::SplineError;
    use sweep::{LoftError, SkinError};
    let places = [0.0, 1.0].map(|z| at_height(Interval::from_f64(z)));
    for (what, b) in [
        ("a wide bulge", {
            let (lo, hi) = box_about(0.2);
            Interval::from_bounds(lo, hi)
        }),
        (
            "a point bulge, its sweep widened by `atan`",
            Interval::from_f64(0.2),
        ),
    ] {
        match loft_body::<Interval>(&[section(b), section(b)], &places, 1, Tol::witness()) {
            Err(LoftError::Skin(SkinError::Structure(SplineError::CarrierWeightNotAPoint {
                ..
            }))) => {}
            other => panic!(
                "{what}: expected section 0's arc refusal, got {:?}",
                other.map(|_| ())
            ),
        }
    }
    let ends = Interval::from_f64(1.2);
    let (lo, hi) = box_about(1.2);
    match loft_body::<Interval>(
        &[quad(ends), quad(Interval::from_bounds(lo, hi)), quad(ends)],
        &[0.0, 1.0, 2.0].map(|z| at_height(Interval::from_f64(z))),
        1,
        Tol::witness(),
    ) {
        Err(LoftError::Skin(SkinError::Structure(SplineError::CarrierKnotNotAPoint {
            index: 1,
        }))) => {}
        other => panic!(
            "expected the wide interior knot's refusal, got {:?}",
            other.map(|_| ())
        ),
    }
}

/// **Point inputs keep the lifted `f64` skin.** Where every input to
/// the skin is a point, the loft at `Interval` is the `f64` loft lifted,
/// bit for bit: an arc section (lifted from `f64`, so its sweep is a
/// point), and four sections at v-degree 2, whose walls' interior v knot
/// is the mean of two computed parameters. That is the status quo, not
/// an enclosure claim over a box: the family is one member. Widening an
/// interior section's parameter refuses at the skin's carrier door
/// (`what_a_carrier_cannot_hold_refuses_typed_at_interval`). At v-degree
/// 1 the body's walls are one slab each and carry no interior knot, so
/// that case lofts at degree 2 and its walls are checked to carry one.
#[test]
fn point_inputs_loft_as_the_lifted_f64_skin_at_interval() {
    use geom_core::Interval;
    let lift = |s: Vec<ProfileLoop<f64>>| -> Vec<ProfileLoop<Interval>> {
        s.iter()
            .map(|lp| lp.map_scalar(Interval::from_f64))
            .collect()
    };
    let cases = [
        (
            "an arc section",
            vec![section(0.2), section(0.2)],
            vec![0.0, 1.0],
            1,
        ),
        (
            "four sections at v-degree 2",
            vec![quad(1.2), quad(1.3), quad(1.25), quad(1.2)],
            vec![0.0, 1.0, 2.0, 3.0],
            2,
        ),
    ];
    for (what, sections, zs, degree) in cases {
        let places: Vec<SketchPlane<f64>> = zs.iter().map(|z| at_height(*z)).collect();
        let nominal = loft_body::<f64>(&sections, &places, degree, Tol::witness())
            .expect("the f64 loft builds");
        if zs.len() > degree + 1 {
            // The case's hypothesis: the body's walls carry the computed
            // interior knot.
            let face = nominal
                .body
                .get_face(nominal.side_faces[0][0][0])
                .expect("a built face");
            let Some(Surface::Nurbs(net)) = nominal.body.get_surface(face.surface) else {
                panic!("{what}: a lofted wall is a NURBS surface")
            };
            let knots = net.knot_values_v();
            let (lo, hi) = knots.domain();
            assert!(
                knots.values().iter().any(|k| *k > lo && *k < hi),
                "{what}: the wall carries an interior v knot"
            );
        }
        let lifted: Vec<Vec<ProfileLoop<Interval>>> = sections.into_iter().map(lift).collect();
        let lifted_places: Vec<SketchPlane<Interval>> = zs
            .iter()
            .map(|z| at_height(Interval::from_f64(*z)))
            .collect();
        let enclosed = loft_body::<Interval>(&lifted, &lifted_places, degree, Tol::witness())
            .unwrap_or_else(|e| panic!("{what}: the point loft builds at Interval: {e:?}"));
        assert_eq!(
            points(&enclosed.section_params),
            nominal
                .section_params
                .iter()
                .map(|u| Some(*u))
                .collect::<Vec<_>>(),
            "{what}"
        );
        assert_eq!(wall_bits(&enclosed), wall_bits(&nominal), "{what}");
    }
}

/// **At an enclosure scalar the knot algebra runs homogeneous (C6 S4).**
/// A line and an integral quadratic at `Interval` are made compatible:
/// the line is elevated to degree 2 as `(w·P, w)`, so its unit weights
/// stay exactly 1 and its new middle pole is the exact midpoint, the
/// `f64` elevation lifted; the carrier's door admits it. A rational
/// section whose refinement writes a weight wider than a point (from
/// rounding alone) refuses at the door, typed, at that weight. Curves
/// already compatible come back as themselves. Red where the enclosure
/// branch refuses what the algebra can do, forms a weight quotient that
/// widens a unit weight, or admits a wide weight.
#[test]
fn make_compatible_at_interval_runs_the_knot_algebra_homogeneous() {
    use core::num::NonZeroUsize;
    use geom::NurbsCurve3;
    use geom_core::spline::{Knots, SplineError};
    use geom_core::{Bounds, Interval, Point3};
    use sweep::{SketchSegment, SkinError, make_compatible, segment_curve};
    let i = Interval::from_f64;
    let p = |x: f64, y: f64| Point2::new(i(x), i(y));
    let line = segment_curve::<Interval>(
        0,
        SketchSegment::Line {
            a: p(0.0, 0.0),
            b: p(1.0, 0.0),
        },
        geom_core::Affine3::identity(),
    )
    .expect("a line converts at Interval");
    let three = NonZeroUsize::new(3).expect("three");
    let quadratic = |w: f64| {
        NurbsCurve3::new(
            Knots::new(vec![(i(0.0), three), (i(1.0), three)], 2).expect("clamped quadratic knots"),
            vec![
                Point3::new(i(0.0), i(0.0), i(1.0)),
                Point3::new(i(0.5), i(0.5), i(1.0)),
                Point3::new(i(1.0), i(0.0), i(1.0)),
            ],
            vec![i(1.0), i(w), i(1.0)],
        )
        .expect("a quadratic")
    };
    let both = make_compatible(&[line.clone(), quadratic(1.0)]).expect("the line elevates");
    let elevated = &both[0];
    assert_eq!(elevated.degree(), 2);
    let want: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0], [0.5, 0.0, 0.0], [1.0, 0.0, 0.0]];
    let got: Vec<[f64; 3]> = elevated
        .control()
        .iter()
        .map(|q| {
            let pt = |v: Interval| {
                assert_eq!(v.lo(), v.hi(), "a pole of the elevated line is a point");
                v.lo()
            };
            [pt(q.x), pt(q.y), pt(q.z)]
        })
        .collect();
    assert_eq!(got, want, "the elevated line's poles");
    for w in elevated.weights() {
        assert_eq!(
            (w.lo(), w.hi()),
            (1.0, 1.0),
            "a unit weight stays exactly 1"
        );
    }
    // A rational section refined onto an interior knot writes a weight
    // that rounds: the door refuses it.
    let with_knot = NurbsCurve3::new(
        Knots::new(
            vec![
                (i(0.0), three),
                (i(0.3), NonZeroUsize::MIN),
                (i(1.0), three),
            ],
            2,
        )
        .expect("knots with one interior"),
        vec![
            Point3::new(i(0.0), i(0.0), i(1.0)),
            Point3::new(i(0.25), i(0.5), i(1.0)),
            Point3::new(i(0.75), i(0.5), i(1.0)),
            Point3::new(i(1.0), i(0.0), i(1.0)),
        ],
        vec![i(1.0); 4],
    )
    .expect("an integral quadratic with an interior knot");
    match make_compatible(&[quadratic(0.7), with_knot]) {
        Err(SkinError::KnotAlgebra(geom_core::spline::KnotAlgebraError::Structure(
            SplineError::CarrierWeightNotAPoint { .. },
        ))) => {}
        other => panic!("a rational refinement at Interval: {:?}", other.map(|_| ())),
    }
    let same = make_compatible(&[line.clone(), line.clone()]).expect("two lines are compatible");
    for c in &same {
        assert_eq!(
            (c.knots_f64_interim().knots(), c.control().len()),
            (line.knots_f64_interim().knots(), 2),
            "a compatible line comes back as itself"
        );
    }
}

/// The wall poles, weights and knots of every face a loft built, in face
/// order, as bits of their value channel.
fn wall_bits<T: geom_core::spline::SpanLocate>(lofted: &Lofted<T>) -> Vec<u64> {
    lofted
        .side_faces
        .iter()
        .flatten()
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
                    .chain(n.knot_values_u().values().iter().copied())
                    .chain(n.knot_values_v().values().iter().copied())
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
        SketchPlane::xy(),
        &elbow_path(),
        ELBOW_STATIONS,
        ELBOW_V_DEGREE,
        tol,
    )
    .expect("the elbow sweeps at f64");
    let lane = lift_sections::<Interval>(core::slice::from_ref(&profile)).remove(0);
    let enclosed = sweep_body::<Interval>(
        &lane,
        SketchPlane::xy(),
        &elbow_path(),
        ELBOW_STATIONS,
        ELBOW_V_DEGREE,
        tol,
    )
    .expect("the elbow sweeps at Interval");
    assert_eq!(
        points(&enclosed.section_params),
        nominal
            .section_params
            .iter()
            .map(|u| Some(*u))
            .collect::<Vec<_>>()
    );
    assert_eq!(wall_bits(&enclosed), wall_bits(&nominal));
}

/// Every value of every wall a loft built, in face order: poles,
/// weights, then the v knots, each read through `f`.
fn wall_values<T: geom_core::spline::SpanLocate, R>(
    lofted: &Lofted<T>,
    f: impl Fn(T) -> R,
) -> Vec<R> {
    let mut out = Vec::new();
    for face in lofted.side_faces.iter().flatten().flatten() {
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
fn at_height<T: Real>(z: T) -> SketchPlane<T> {
    SketchPlane::xy_at(Point3::new(T::zero(), T::zero(), z))
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
            segment_curve(i, seg, at_height(T::from_f64(z)).placement()).expect("a section curve")
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
        &[SketchPlane::xy(), at_height(h)],
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
/// each wall value's derivative is what the `f64` builds a unit apart
/// differ by. (At `Sym` the sweep's path frames are computed in `f64` and
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
    // The sweep's stations turn about the path's start point, so a
    // start frame raised by `dh` raises station `i` by `Cᵢ·ẑ·dh` for its
    // carried rotation `Cᵢ`: every wall value is affine in the height,
    // and its derivative is the f64 builds' unit difference.
    let got = wall_values(&swept_at(Dual64::variable(h0)), |x| x.deriv);
    let step: Vec<f64> = wall_values(&swept_at(h0 + 1.0), |x: f64| x)
        .iter()
        .zip(wall_values(&swept_at(h0), |x: f64| x))
        .map(|(hi, lo)| hi - lo)
        .collect();
    assert_eq!(got.len(), step.len(), "the same walls at every height");
    assert!(
        step.iter().any(|d| d.abs() > 0.5),
        "the start frame's height reaches the swept walls"
    );
    for (k, (d, want)) in got.iter().zip(&step).enumerate() {
        assert!(
            (d - want).abs() <= 1e-12,
            "wall value {k} moves with the start frame's height as the f64 builds do: \
             {d} against {want}"
        );
    }
}

/// Each parameter's one `f64`, `None` where it has none.
fn points<T: geom_core::spline::SpanLocate>(params: &[T]) -> Vec<Option<f64>> {
    params.iter().map(|u| u.point_value()).collect()
}
