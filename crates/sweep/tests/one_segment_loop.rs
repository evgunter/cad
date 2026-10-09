//! **A one-segment closed loop through the solid builders** (D1: a full
//! turn is one segment at one vertex, so a closed carrier is one edge).
//!
//! Every fixture is a raw table — a one-segment circle written through
//! the fixture door — and every body is checked at all three tiers and
//! against its closed-form volume:
//!
//! - **extrude**: one vertex per cap, one self-loop rim per cap, one
//!   periodic wall, one strut with both halves in that wall, described
//!   as the wall's wrap edge; outer and hole, both directions, `f64`
//!   and `Interval`;
//! - **revolve**: one torus wall whose strut wraps its tube angle `v`,
//!   and on a full turn one face closed both ways; holes, outers and
//!   annuli, every vertex phase and winding, `f64` and `Interval`; a
//!   circle reaching the axis keeps its own axis refusal;
//! - **loft**: one spline wall whose strut wraps its `u`, its tier-3
//!   verdict and its volume held to the two-arc wall's;
//! - **tier 3 both ways**: an edge whose halves bound one face is that
//!   face's wrap edge, and one without the flag is refused;
//! - **a boolean** on an extruded periodic wall with a wrap strut, and
//!   **a split** through its strut's vertices and along it, held
//!   against the two-arc form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_brep::EdgeDescription;
use geom_core::{Arc2, Bounds, Interval, Point2, Real, Tol};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane, ValidatedProfile};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{Body, EdgeKey, validate, validate_closed, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

/// A one-segment circle about `(cx, cy)` of radius `r`, its vertex at
/// carrier angle 0, turning `sweep` (±2π).
fn circle<T: Real>(cx: f64, cy: f64, r: f64, sweep: f64) -> ProfileLoop<T> {
    circle_at(cx, cy, r, 0.0, sweep)
}

/// [`circle`] with its vertex at carrier angle `phase`.
fn circle_at<T: Real>(cx: f64, cy: f64, r: f64, phase: f64, sweep: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    RawLoop::new([(
        Point2::new(f(cx + r * phase.cos()), f(cy + r * phase.sin())),
        Segment::Arc(Arc2 {
            centre: Point2::new(f(cx), f(cy)),
            radius: f(r),
            sweep: f(sweep),
        }),
    )])
}

fn rect<T: Real>(x0: f64, y0: f64, x1: f64, y1: f64) -> ProfileLoop<T> {
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    RawLoop::polygon([p(x0, y0), p(x1, y0), p(x1, y1), p(x0, y1)])
}

fn validated<T: geom_core::Decide>(loops: Vec<ProfileLoop<T>>) -> ValidatedProfile<T> {
    Profile::new(SketchPlane::<T>::xy(), loops)
        .validate(tol())
        .unwrap_or_else(|e| panic!("the fixture validates: {e}"))
}

/// All three tiers, by name.
fn tiers<T: geom_core::Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>(
    body: &Body<T>,
    what: &str,
) {
    assert_eq!(validate(body), Ok(()), "{what}: tier 1");
    assert_eq!(validate_closed(body), Ok(()), "{what}: tier 2");
    assert_eq!(validate_geometric(body, tol()), Ok(()), "{what}: tier 3");
}

/// A body's volume and its certified half-width: a curved-cut face
/// contributes a quadrature enclosure converged to an ε-scaled target,
/// so the closed form is only certified within `volume ± pad`.
fn volume(body: &Body<f64>) -> (f64, f64) {
    let m = topo::mass_properties(body, tol()).unwrap();
    (m.volume, m.volume_pad)
}

fn close((got, pad): (f64, f64), want: f64, what: &str) {
    assert!(
        (got - want).abs() <= pad + 1e-9 * want.abs().max(1.0),
        "{what}: volume {got} ± {pad}, closed form {want}"
    );
}

/// `(V, E, F)` of a body.
fn census<T: Real>(body: &Body<T>) -> (usize, usize, usize) {
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
    )
}

/// Whether `edge` is described as its chart's wrap edge.
fn is_wrap<T: Real>(body: &Body<T>, edge: EdgeKey) -> bool {
    let e = body.get_edge(edge).unwrap();
    let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    matches!(c.description(), EdgeDescription::Chart(ch) if ch.wrap)
}

fn extruded(loops: Vec<ProfileLoop<f64>>, depth: f64, side: ExtrudeSide) -> sweep::Extruded<f64> {
    extrude(
        &validated(loops),
        Extrusion::Distance { depth, side },
        tol(),
    )
    .unwrap_or_else(|e| panic!("the extrusion builds: {e}"))
}

/// A one-segment circle extruded either way: two vertices, three edges
/// (two self-loop rims and the strut), three faces; the strut's two
/// halves both bound the wall, which is its seam; πr²h.
#[test]
fn an_extruded_one_segment_circle_is_one_wall_with_a_seam_strut() {
    for side in [ExtrudeSide::Along, ExtrudeSide::Against] {
        for sweep in [TAU, -TAU] {
            let what = format!("{side:?}, sweep {sweep}");
            let t = extruded(vec![circle(0.5, -0.25, 1.5, sweep)], 2.0, side);
            tiers(&t.body, &what);
            assert_eq!(census(&t.body), (2, 3, 3), "{what}");
            assert_eq!(t.walls.len(), 1);
            let [wall] = &t.walls[0][..] else {
                panic!("{what}: one wall, got {:?}", t.walls[0]);
            };
            assert_eq!(wall.segments, vec![0], "{what}");
            assert!(
                is_wrap(&t.body, wall.strut),
                "{what}: the strut is the wall's seam"
            );
            for rim in [&wall.top_rim, &wall.bottom_rim] {
                let e = t.body.get_edge(*rim).unwrap();
                let (a, b) = (
                    t.body.get_half_edge(e.he_plus).unwrap().start,
                    t.body.get_half_edge(e.he_minus).unwrap().start,
                );
                assert_eq!(a, b, "{what}: a rim is a self-loop");
            }
            close(volume(&t.body), PI * 1.5 * 1.5 * 2.0, &what);
        }
    }
}

/// One-segment circles as holes, and as the outer around a polygonal
/// hole: an annulus of two circles, a square with a circular hole, a
/// circle with a square hole.
#[test]
fn one_segment_circles_extrude_as_holes_and_around_them() {
    let h = 1.25;
    type Case = (
        &'static str,
        Vec<ProfileLoop<f64>>,
        f64,
        (usize, usize, usize),
    );
    let cases: Vec<Case> = vec![
        (
            "annulus",
            vec![circle(0.0, 0.0, 2.0, TAU), circle(0.0, 0.0, 1.0, -TAU)],
            PI * (4.0 - 1.0) * h,
            (4, 6, 4),
        ),
        (
            "square with a round hole",
            vec![rect(-2.0, -2.0, 2.0, 2.0), circle(0.25, 0.0, 1.0, TAU)],
            (16.0 - PI) * h,
            (10, 15, 7),
        ),
        (
            "round plate with a square hole",
            vec![circle(0.0, 0.0, 2.0, TAU), rect(-0.5, -0.5, 0.5, 0.5)],
            (4.0 * PI - 1.0) * h,
            (10, 15, 7),
        ),
        (
            "two round holes",
            vec![
                rect(-3.0, -2.0, 3.0, 2.0),
                circle(-1.5, 0.0, 1.0, TAU),
                circle(1.5, 0.0, 0.5, -TAU),
            ],
            (24.0 - PI * 1.25) * h,
            (12, 18, 8),
        ),
    ];
    for side in [ExtrudeSide::Along, ExtrudeSide::Against] {
        for (what, loops, want, counts) in &cases {
            let what = format!("{what}, {side:?}");
            let t = extruded(loops.clone(), h, side);
            tiers(&t.body, &what);
            assert_eq!(census(&t.body), *counts, "{what}");
            close(volume(&t.body), *want, &what);
            for (l, walls) in t.walls.iter().enumerate() {
                if walls.iter().map(|w| w.segments.len()).sum::<usize>() == 1 {
                    assert!(is_wrap(&t.body, walls[0].strut), "{what}: loop {l}'s strut");
                }
            }
        }
    }
}

/// **The same extrusions at `Interval`**, over vertex phase × winding
/// × extrude side × outer or hole: all three tiers, the census, the
/// strut described as the wall's seam, and a volume enclosure that
/// holds the closed form.
#[test]
fn one_segment_circles_extrude_at_interval() {
    let (r, h) = (1.0, 1.5);
    for phase in [0.0, 1.0] {
        for sweep in [TAU, -TAU] {
            for side in [ExtrudeSide::Along, ExtrudeSide::Against] {
                for hole in [false, true] {
                    let what = format!("phase {phase}, sweep {sweep}, {side:?}, hole {hole}");
                    let circle = circle_at::<Interval>(0.25, 0.0, r, phase, sweep);
                    let (loops, want, counts) = if hole {
                        (
                            vec![rect(-2.0, -2.0, 2.0, 2.0), circle],
                            (16.0 - PI * r * r) * h,
                            (10, 15, 7),
                        )
                    } else {
                        (vec![circle], PI * r * r * h, (2, 3, 3))
                    };
                    let t = extrude(
                        &validated(loops),
                        Extrusion::Distance {
                            depth: Interval::from_f64(h),
                            side,
                        },
                        tol(),
                    )
                    .unwrap_or_else(|e| panic!("{what}: the extrusion builds at Interval: {e}"));
                    tiers(&t.body, &what);
                    assert_eq!(census(&t.body), counts, "{what}");
                    let walls = t.walls.last().unwrap();
                    assert_eq!(walls.len(), 1, "{what}: the circle's one wall");
                    assert!(
                        is_wrap(&t.body, walls[0].strut),
                        "{what}: the strut is the wall's seam"
                    );
                    let v = topo::mass_properties(&t.body, tol()).unwrap().volume;
                    assert!(
                        v.lo() <= want && want <= v.hi(),
                        "{what}: volume {v:?} does not hold the closed form {want}"
                    );
                }
            }
        }
    }
}

/// **A boolean on an extruded periodic wall with a seam strut** — no
/// evidence that one had ever run before this row. The cylinder is
/// r = 1 about the z axis from z = 0 to 2, its seam strut on the
/// meridian x = 1. Every operand pair is plane × cylinder.
///
/// Every cut builds, at all three tiers and its closed form:
///
/// - ALONG the wall: the half x ≥ 0 cut away (the seam inside the
///   tool), the half y ≥ 0 cut away (the tool's face through the seam),
///   and a bar united through the wall around the seam;
/// - ACROSS it: a slab kept and cut away, each cap-parallel plane
///   meeting the wall in a circle that crosses the one seam once, a
///   section loop with one site (every op in both orders:
///   `a_plane_across_a_one_face_wall.rs`).
#[test]
fn a_boolean_on_an_extruded_seam_wall_builds_along_and_across_it() {
    use sweep::test_support::brick;
    use topo::BooleanDeclarations;
    use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};

    let cyl = extruded(vec![circle(0.0, 0.0, 1.0, TAU)], 2.0, ExtrudeSide::Along).body;
    // ∫_{-1/2}^{1/2} √(1 − y²) dy: the disc's share of a bar of width 1
    // reaching past x = 1 from x = 0.
    let bar_in_disc = 0.75f64.sqrt() * 0.5 + (0.5f64).asin();
    let cases: Vec<(&str, BooleanOp, Body<f64>, f64)> = vec![
        (
            "x ≥ 0 cut away",
            BooleanOp::Subtract,
            brick((0.0, 3.0), (-3.0, 3.0), (-1.0, 3.0), tol()),
            PI,
        ),
        (
            "y ≥ 0 cut away",
            BooleanOp::Subtract,
            brick((-3.0, 3.0), (0.0, 3.0), (-1.0, 3.0), tol()),
            PI,
        ),
        (
            "a bar through the seam",
            BooleanOp::Union,
            brick((0.0, 3.0), (-0.5, 0.5), (0.5, 1.5), tol()),
            2.0 * PI + 3.0 - bar_in_disc,
        ),
        (
            "a slab kept",
            BooleanOp::Intersect,
            brick((-3.0, 3.0), (-3.0, 3.0), (0.5, 1.0), tol()),
            0.5 * PI,
        ),
        (
            "a slab cut away",
            BooleanOp::Subtract,
            brick((-3.0, 3.0), (-3.0, 3.0), (0.5, 1.0), tol()),
            1.5 * PI,
        ),
    ];
    for (what, op, tool, want) in cases {
        let out = boolean_op_with(
            op,
            &topo::test_support::finished("the seam cylinder", cyl.clone(), tol()),
            &topo::test_support::finished("the tool", tool, tol()),
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            tol(),
        );
        let out = out.unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let body = &out
            .body()
            .unwrap_or_else(|| panic!("{what}: the boolean left no body"))
            .body;
        tiers(body, what);
        close(volume(body), want, what);
    }
}

/// The sketch y axis: `r(p) = p.x`.
fn y_axis<T: Real>() -> sweep::RevolveAxis<T> {
    sweep::RevolveAxis {
        origin: Point2::new(T::zero(), T::zero()),
        dir: geom_core::Vec2::new(T::zero(), T::one()),
    }
}

/// The circle about `(cx, cy)` of radius `r` as two arcs, its vertices
/// at carrier angles `phase` and `phase + π`: the form a one-segment
/// circle is held against.
fn two_arcs_at(cx: f64, cy: f64, r: f64, phase: f64, sweep: f64) -> ProfileLoop<f64> {
    let half = Segment::Arc(Arc2 {
        centre: Point2::new(cx, cy),
        radius: r,
        sweep: sweep / 2.0,
    });
    let (c, s) = (r * phase.cos(), r * phase.sin());
    RawLoop::new([
        (Point2::new(cx + c, cy + s), half),
        (Point2::new(cx - c, cy - s), half),
    ])
}

fn revolved<T: geom_core::Decide + topo::AtRestPolicy>(
    loops: Vec<ProfileLoop<T>>,
    turn: sweep::Revolution<T>,
    what: &str,
) -> sweep::Revolved<T> {
    sweep::revolve(&validated(loops), y_axis(), turn, tol())
        .unwrap_or_else(|e| panic!("{what}: the revolve builds: {e}"))
}

/// The turns every revolve row sweeps: part of a turn either way, and
/// the full turn, with the angle each sweeps.
fn turns() -> [(sweep::Revolution<f64>, f64); 3] {
    [
        (sweep::Revolution::Partial(1.25), 1.25),
        (sweep::Revolution::Partial(-1.25), 1.25),
        (sweep::Revolution::Full, TAU),
    ]
}

/// Each wall's wrap edges: the edges with both halves in it,
/// described as its wrap edges (D1). Every edge with both halves in
/// one face is one.
fn wrap_edges<T: Real>(body: &Body<T>) -> Vec<(topo::FaceKey, EdgeKey)> {
    let mut out = Vec::new();
    for (edge, _) in body.edges() {
        let sides = topo::readback::edge_sides(body, edge).unwrap();
        let (a, b) = sides.faces();
        assert_eq!(
            a == b,
            is_wrap(body, edge),
            "{edge:?}: wrap flag against its faces"
        );
        if a == b {
            out.push((a, edge));
        }
    }
    out
}

/// **A revolved one-segment circle is one torus wall** (D1's wrap
/// edge, #4175). Part of a turn: two planar caps, one torus wall, three
/// edges (the two meridian circles and the strut, the wall's wrap edge
/// in `v`), two vertices. The full turn: ONE face closed on itself both
/// ways — the meridian wraps `u`, the latitude circle wraps `v` — two
/// edges and one vertex. Every case at all three tiers, at the vertex
/// phases 0, 1 and π (the inner equator), both windings of the turn;
/// its volume is Pappus' `θ·πr²·R` and the two-arc circle's, and the
/// body meshes closed.
#[test]
fn a_revolved_one_segment_circle_is_one_torus_wall_with_a_wrap_strut() {
    let (cx, cy, r) = (3.0, 0.25, 0.5);
    for phase in [0.0, 1.0, PI] {
        for (turn, theta) in turns() {
            let what = format!("phase {phase}, {turn:?}");
            let one = revolved(vec![circle_at(cx, cy, r, phase, TAU)], turn, &what);
            let full = matches!(turn, sweep::Revolution::Full);
            tiers(&one.body, &what);
            let want = theta * PI * r * r * cx;
            close(volume(&one.body), want, &what);
            let two = revolved(vec![two_arcs_at(cx, cy, r, phase, TAU)], turn, &what);
            close(
                volume(&two.body),
                want,
                &format!("{what}, the two-arc circle"),
            );
            let walls = one.walls();
            let wall = walls[0][0].expect("the circle sweeps a wall");
            assert!(
                matches!(
                    one.body
                        .get_surface(one.body.get_face(wall).unwrap().surface),
                    Some(geom::Surface::Torus { .. })
                ),
                "{what}: the wall is a torus"
            );
            let wraps = wrap_edges(&one.body);
            if full {
                assert_eq!(census(&one.body), (1, 2, 1), "{what}");
                assert_eq!(wraps.len(), 2, "{what}: the meridian and the parallel");
            } else {
                assert_eq!(census(&one.body), (2, 3, 3), "{what}");
                assert_eq!(wraps.len(), 1, "{what}: the strut");
            }
            assert!(wraps.iter().all(|&(f, _)| f == wall), "{what}: {wraps:?}");
            let rim = one.rims[0][0].expect("the strut is the vertex's rim");
            assert!(
                wraps.iter().any(|&(_, e)| e == rim),
                "{what}: the strut wraps"
            );
            let mesh = mesh::tessellate(&one.body, 1e-2, tol())
                .unwrap_or_else(|e| panic!("{what}: meshes: {e}"));
            assert_eq!(mesh::validate::check_mesh(&mesh), Ok(()), "{what}: closed");
            let got = mesh::validate::signed_volume(&mesh);
            assert!(
                (got - want).abs() < 0.02 * want,
                "{what}: mesh volume {got} vs {want}"
            );
        }
    }
}

/// One-segment circles as holes, and as the outer around a polygonal
/// hole, revolved: a rectangle with a round hole, a circle with a
/// square hole, and an annulus of two circles. A full revolve's hole is
/// a cavity, its own one-face torus.
#[test]
fn one_segment_circles_revolve_as_holes_and_around_them() {
    let sq = |c: f64, h: f64| rect(c - h, -h, c + h, h);
    type Case = (
        &'static str,
        Vec<ProfileLoop<f64>>,
        f64,
        (usize, usize, usize),
        (usize, usize, usize),
    );
    // (what, loops, area × centroid radius, partial census, full census)
    let cases: Vec<Case> = vec![
        (
            "a rectangle with a round hole",
            vec![rect(2.0, -1.0, 4.0, 1.0), circle(3.0, 0.0, 0.5, -TAU)],
            (4.0 - PI * 0.25) * 3.0,
            (10, 15, 7),
            (5, 8, 5),
        ),
        (
            "a circle with a square hole",
            vec![circle_at(3.0, 0.0, 1.0, 0.5, TAU), sq(3.0, 0.25)],
            (PI - 0.25) * 3.0,
            (10, 15, 7),
            (5, 8, 5),
        ),
        (
            "an annulus of two circles",
            vec![
                circle(3.0, 0.0, 1.0, TAU),
                circle_at(3.0, 0.0, 0.5, 2.0, -TAU),
            ],
            (PI - PI * 0.25) * 3.0,
            (4, 6, 4),
            (2, 4, 2),
        ),
    ];
    for (what, loops, moment, partial, full) in cases {
        for (turn, theta) in turns() {
            let what = format!("{what}, {turn:?}");
            let t = revolved(loops.clone(), turn, &what);
            tiers(&t.body, &what);
            let counts = if matches!(turn, sweep::Revolution::Full) {
                full
            } else {
                partial
            };
            assert_eq!(census(&t.body), counts, "{what}");
            close(volume(&t.body), theta * moment, &what);
            let wraps = wrap_edges(&t.body);
            assert!(!wraps.is_empty(), "{what}: a wrap edge");
        }
    }
}

/// **The same revolves at `Interval`**, outer and hole, at two vertex
/// phases, part of a turn either way and the full turn: all three
/// tiers, the census, and a volume enclosure that holds the closed
/// form.
#[test]
fn one_segment_circles_revolve_at_interval() {
    let i = Interval::from_f64;
    for phase in [0.0, 1.0] {
        for (turn, theta) in [
            (sweep::Revolution::Partial(i(1.25)), 1.25),
            (sweep::Revolution::Partial(i(-1.25)), 1.25),
            (sweep::Revolution::Full, TAU),
        ] {
            let full = matches!(turn, sweep::Revolution::Full);
            for hole in [false, true] {
                let what = format!("phase {phase}, {turn:?}, hole {hole}");
                let (loops, moment, counts) = if hole {
                    (
                        vec![
                            rect::<Interval>(2.0, -1.0, 4.0, 1.0),
                            circle_at::<Interval>(3.0, 0.0, 0.5, phase, -TAU),
                        ],
                        (4.0 - PI * 0.25) * 3.0,
                        if full { (5, 8, 5) } else { (10, 15, 7) },
                    )
                } else {
                    (
                        vec![circle_at::<Interval>(3.0, 0.25, 0.5, phase, TAU)],
                        PI * 0.25 * 3.0,
                        if full { (1, 2, 1) } else { (2, 3, 3) },
                    )
                };
                let t = revolved(loops, turn, &what);
                tiers(&t.body, &what);
                assert_eq!(census(&t.body), counts, "{what}");
                let want = theta * moment;
                let v = topo::mass_properties(&t.body, tol()).unwrap().volume;
                assert!(
                    v.lo() <= want && want <= v.hi(),
                    "{what}: volume {v:?} does not hold the closed form {want}"
                );
            }
        }
    }
}

/// **A lofted one-segment section is one spline wall** (D1's wrap
/// edge, #4175): the strut through the vertex is both the `u = 0` and
/// the `u = 1` column of the one wall, its wrap edge in `u`, laid top
/// to bottom and carried by the column run back. A circle, and a square
/// with a round hole, each lofted between two stacked copies: one wall
/// per one-segment loop with its strut the body's one wrap edge, all
/// three tiers at `f64`, and at `Interval` tiers 1–2 and the two-arc
/// circle's tier-3 verdict. A rational wall is not meshed
/// (`work/tess/lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late.md`);
/// its volume is the next row's.
#[test]
fn a_lofted_one_segment_section_is_one_spline_wall_with_a_wrap_strut() {
    let places = sweep::test_support::stacked_at(&[0.0, 2.0]);
    type Case = (
        &'static str,
        Vec<ProfileLoop<f64>>,
        Vec<ProfileLoop<f64>>,
        usize,
    );
    let cases: Vec<Case> = vec![
        (
            "a circle",
            vec![circle(0.0, 0.0, 1.0, TAU)],
            vec![two_arcs_at(0.0, 0.0, 1.0, 0.0, TAU)],
            0,
        ),
        (
            "a square with a round hole",
            vec![rect(-2.0, -2.0, 2.0, 2.0), circle(0.25, 0.0, 1.0, TAU)],
            vec![
                rect(-2.0, -2.0, 2.0, 2.0),
                two_arcs_at(0.25, 0.0, 1.0, 0.0, TAU),
            ],
            1,
        ),
    ];
    for (what, section, two_arcs, li) in cases {
        let one = sweep::loft_body::<f64>(&[section.clone(), section.clone()], &places, 1, tol())
            .unwrap_or_else(|e| panic!("{what}: the loft builds: {e}"));
        tiers(&one.body, what);
        let [wall] = one.side_faces[li][..] else {
            panic!("{what}: one wall, got {:?}", one.side_faces[li]);
        };
        let [strut] = one.seam_edges[li][..] else {
            panic!("{what}: one strut, got {:?}", one.seam_edges[li]);
        };
        assert_eq!(
            wrap_edges(&one.body),
            vec![(wall, strut)],
            "{what}: the strut is the body's one wrap edge"
        );
        let at_i = |loops: Vec<ProfileLoop<f64>>| {
            let lofted = sweep::loft_body::<Interval>(&[loops.clone(), loops], &places, 1, tol())
                .unwrap_or_else(|e| panic!("{what}: the loft builds at Interval: {e}"));
            assert_eq!(validate(&lofted.body), Ok(()), "{what} at Interval: tier 1");
            assert_eq!(
                validate_closed(&lofted.body),
                Ok(()),
                "{what} at Interval: tier 2"
            );
            // The face a refusal names differs between the two bodies;
            // what it says about that face is compared.
            match validate_geometric(&lofted.body, tol()) {
                Ok(()) => None,
                Err(errors) => match &errors[..] {
                    [
                        topo::ValidationError::VolumeUncomputable {
                            source: topo::MassPropsError::Face { source, .. },
                            ..
                        },
                    ] => Some(source.clone()),
                    other => panic!("{what} at Interval: tier 3 refused otherwise: {other:?}"),
                },
            }
        };
        assert_eq!(
            at_i(section),
            at_i(two_arcs),
            "{what} at Interval: tier 3 reads the one-segment wall as the two-arc walls"
        );
    }
}

/// **A lofted one-segment wall measures as the two-arc wall does.**
/// The rational wall's volume comes from the quadrature lane, whose
/// enclosure must reach `1024·ε`: where both forms reach it their
/// enclosures overlap each other and contain the closed form, where
/// one is known (a cylinder `2π`, a frustum `πh(R² + Rr + r²)/3`); a
/// section turned by a radian between the stations has none, and the
/// two forms are held to each other. Where neither reaches it (at
/// ε = 1e-9 and 1e-12 today) both refuse alike (`QuadratureBudget`),
/// and a run where only one form measures is a finding.
#[test]
fn a_lofted_one_segment_wall_measures_as_the_two_arc_wall() {
    let places = sweep::test_support::stacked_at(&[0.0, 2.0]);
    type Case = (
        &'static str,
        [ProfileLoop<f64>; 2],
        [ProfileLoop<f64>; 2],
        Option<f64>,
    );
    let cases: Vec<Case> = vec![
        (
            "a straight cylinder",
            [circle(0.0, 0.0, 1.0, TAU), circle(0.0, 0.0, 1.0, TAU)],
            [
                two_arcs_at(0.0, 0.0, 1.0, 0.0, TAU),
                two_arcs_at(0.0, 0.0, 1.0, 0.0, TAU),
            ],
            Some(TAU),
        ),
        (
            "a section turned a radian",
            [
                circle(0.0, 0.0, 1.0, TAU),
                circle_at(0.0, 0.0, 1.0, 1.0, TAU),
            ],
            [
                two_arcs_at(0.0, 0.0, 1.0, 0.0, TAU),
                two_arcs_at(0.0, 0.0, 1.0, 1.0, TAU),
            ],
            None,
        ),
        (
            "a frustum",
            [circle(0.0, 0.0, 1.0, TAU), circle(0.0, 0.0, 0.5, TAU)],
            [
                two_arcs_at(0.0, 0.0, 1.0, 0.0, TAU),
                two_arcs_at(0.0, 0.0, 0.5, 0.0, TAU),
            ],
            Some(PI * 2.0 * (1.0 + 0.5 + 0.25) / 3.0),
        ),
    ];
    let measure = |what: &str, [a, b]: [ProfileLoop<f64>; 2]| {
        let lofted = sweep::loft_body::<f64>(&[vec![a], vec![b]], &places, 1, tol())
            .unwrap_or_else(|e| panic!("{what}: the loft builds: {e}"));
        match topo::mass_properties(&lofted.body, tol()) {
            Ok(m) => Some((m.volume, m.volume_pad)),
            Err(topo::MassPropsError::Face {
                source: geom_brep::PropsError::QuadratureBudget { .. },
                ..
            }) => None,
            Err(e) => panic!("{what}: the volume refused otherwise: {e:?}"),
        }
    };
    for (what, one, two, want) in cases {
        let got = (
            measure(&format!("{what}, one segment"), one),
            measure(&format!("{what}, two arcs"), two),
        );
        match got {
            (Some(one), Some(two)) => {
                assert!(
                    (one.0 - two.0).abs() <= one.1 + two.1,
                    "{what} at ε = {}: one segment {} ± {}, two arcs {} ± {}",
                    tol().eps(),
                    one.0,
                    one.1,
                    two.0,
                    two.1
                );
                if let Some(want) = want {
                    close(one, want, &format!("{what}, one segment"));
                    close(two, want, &format!("{what}, two arcs"));
                }
                println!(
                    "{what} at ε = {}: one segment {one:?}, two arcs {two:?}",
                    tol().eps()
                );
            }
            (None, None) => println!(
                "NOT COMPARED: {what} at ε = {}: both forms refuse the quadrature budget",
                tol().eps()
            ),
            (one, two) => panic!(
                "{what} at ε = {}: one form measured and the other did not: \
                 one segment {one:?}, two arcs {two:?}",
                tol().eps()
            ),
        }
    }
}

/// **Tier 3 holds the wrap flag both ways.** An edge whose two halves
/// bound one face is that face's wrap edge (D1); with the flag taken
/// off, the edge is described as an ordinary image though nothing lies
/// on its far side, and tier 3 refuses it by name
/// (`DescriptionNotAdjacent`) on the extruded cylinder's strut, the
/// part-turn torus's strut, and each of the one-face torus's two.
#[test]
fn an_edge_bounding_one_face_without_the_wrap_flag_is_refused() {
    let unflag = |body: &mut Body<f64>, edge: EdgeKey| {
        let e = body.get_edge(edge).unwrap();
        let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
        let mut spec = c.restated_spec();
        let geom_brep::EdgeDescriptionSpec::Chart { ref mut wrap, .. } = spec.description else {
            panic!("{edge:?}: a wrap edge is a chart image");
        };
        assert!(*wrap, "{edge:?}: the edge was a wrap edge");
        *wrap = false;
        body.set_edge_curve(edge, spec, tol())
            .expect("an ordinary image of one adjacent surface attaches");
    };
    let extruded = extruded(vec![circle(0.0, 0.0, 1.0, TAU)], 2.0, ExtrudeSide::Along).body;
    let part = revolved(
        vec![circle(3.0, 0.0, 0.5, TAU)],
        sweep::Revolution::Partial(1.25),
        "the part turn",
    )
    .body;
    let full = revolved(
        vec![circle(3.0, 0.0, 0.5, TAU)],
        sweep::Revolution::Full,
        "the full turn",
    )
    .body;
    for (what, body, wraps) in [
        ("the extruded cylinder", extruded, 1),
        ("the part-turn torus", part, 1),
        ("the one-face torus", full, 2),
    ] {
        let edges = wrap_edges(&body);
        assert_eq!(edges.len(), wraps, "{what}: its wrap edges");
        for (_, edge) in edges {
            let mut forged = body.clone();
            unflag(&mut forged, edge);
            assert_eq!(validate(&forged), Ok(()), "{what}: tier 1");
            assert_eq!(validate_closed(&forged), Ok(()), "{what}: tier 2");
            assert_eq!(
                validate_geometric(&forged, tol()),
                Err(vec![topo::ValidationError::DescriptionNotAdjacent { edge }]),
                "{what}: {edge:?} without its wrap flag"
            );
        }
    }
}

/// **A one-segment circle that reaches the axis is refused by what is
/// wrong with it**, before the one-segment refusal: each geometry pins
/// its own axis class.
#[test]
fn a_one_segment_circle_on_or_against_the_axis_refuses() {
    use sweep::RevolveError as E;
    let arc = |what: &str, got: &E| {
        assert!(
            matches!(
                got,
                E::ArcCrossesAxis {
                    loop_index: 0,
                    segment_index: 0
                }
            ),
            "{what}: {got:?}"
        );
    };
    let toroid = |what: &str, got: &E| {
        assert!(
            matches!(
                got,
                E::UnsupportedToroid {
                    loop_index: 0,
                    segment_index: 0
                }
            ),
            "{what}: {got:?}"
        );
    };
    let vertex = |what: &str, got: &E| {
        assert!(
            matches!(
                got,
                E::VertexCrossesAxis {
                    loop_index: 0,
                    vertex_index: 0
                }
            ),
            "{what}: {got:?}"
        );
    };
    type Want = fn(&str, &E);
    let cases: [(&str, ProfileLoop<f64>, Want); 5] = [
        // The sphere class: a full turn about an on-axis centre spans
        // past π, so it dips below the axis.
        ("centred on the axis", circle(0.0, 0.0, 1.0, TAU), arc),
        // The toroid classes: the carrier reaches the axis off-centre.
        ("crossing the axis", circle(0.5, 0.0, 1.0, TAU), toroid),
        (
            "tangent to the axis at its vertex",
            circle_at(1.0, 0.0, 1.0, PI, TAU),
            toroid,
        ),
        (
            "tangent to the axis away from its vertex",
            circle(1.0, 0.0, 1.0, -TAU),
            toroid,
        ),
        // The vertex itself lies across the axis.
        (
            "its vertex across the axis",
            circle_at(1.0, 0.0, 1.5, PI, TAU),
            vertex,
        ),
    ];
    for (what, lp, want) in cases {
        let got = sweep::revolve(
            &validated(vec![lp]),
            y_axis(),
            sweep::Revolution::Partial(1.0),
            tol(),
        );
        want(what, &got.expect_err(what));
    }
}

/// The circle about the origin of radius `r` as two arcs, its vertices
/// at carrier angles `phase` and `phase + π`: the form a one-segment
/// circle is held against.
fn two_arc_circle(r: f64, phase: f64) -> ProfileLoop<f64> {
    let half = Segment::Arc(Arc2 {
        centre: Point2::new(0.0, 0.0),
        radius: r,
        sweep: PI,
    });
    let (c, s) = (r * phase.cos(), r * phase.sin());
    RawLoop::new([(Point2::new(c, s), half), (Point2::new(-c, -s), half)])
}

/// What a split leaves on each side, each side checked at all three
/// tiers: its [`volume`], or `None` for no material.
fn split_volumes(
    body: &Body<f64>,
    origin: [f64; 3],
    normal: [f64; 3],
    what: &str,
) -> Result<[Option<(f64, f64)>; 2], topo::SplitError> {
    let plane = topo::test_support::split_plane(
        geom_core::Point3::new(origin[0], origin[1], origin[2]),
        geom_core::Vec3::new(normal[0], normal[1], normal[2]),
        tol(),
    );
    let out = topo::split(
        &topo::test_support::finished("the cylinder", body.clone(), tol()),
        &plane,
        tol(),
    )?;
    Ok([&out.above, &out.below].map(|part| {
        part.body().map(|b| {
            tiers(b, what);
            volume(b)
        })
    }))
}

/// A split row: its name, the plane's origin and normal, and the volume
/// each side keeps (`None` for no material).
type SplitCase = (&'static str, [f64; 3], [f64; 3], [Option<f64>; 2]);

/// **A split of an extruded one-segment cylinder builds where the
/// two-arc cylinder does, at the same volumes.** The cylinder is r = 1
/// about the z axis, its seam strut on the vertex's meridian. The
/// planes, at two vertex phases (one per winding) and both extrude
/// sides: through the bottom seam vertex and the axis's midpoint, and
/// through the top one (each halves the body, so the face-extent lever
/// of a cap whose one vertex is on the plane is read); tangent to the
/// wall along the strut (both vertices ON, all material on one side);
/// and containing the strut and the axis (π | π).
#[test]
fn a_split_through_the_seam_builds_as_the_two_arc_form_does() {
    for side in [ExtrudeSide::Along, ExtrudeSide::Against] {
        for (phase, sweep) in [(0.0, TAU), (PI / 2.0, -TAU)] {
            let one = extruded(vec![circle_at(0.0, 0.0, 1.0, phase, sweep)], 2.0, side).body;
            let two = extruded(vec![two_arc_circle(1.0, phase)], 2.0, side).body;
            let (c, s) = (phase.cos(), phase.sin());
            let (bottom, top) = match side {
                ExtrudeSide::Along => (0.0, 2.0),
                ExtrudeSide::Against => (-2.0, 0.0),
            };
            let mid = (bottom + top) / 2.0;
            let cases: [SplitCase; 4] = [
                (
                    "through the bottom seam vertex",
                    [c, s, bottom],
                    [c, s, 1.0],
                    [Some(PI), Some(PI)],
                ),
                (
                    "through the top seam vertex",
                    [c, s, top],
                    [-c, -s, 1.0],
                    [Some(PI), Some(PI)],
                ),
                (
                    "tangent along the strut",
                    [c, s, mid],
                    [c, s, 0.0],
                    [None, Some(TAU)],
                ),
                (
                    "containing the strut and the axis",
                    [0.0, 0.0, mid],
                    [-s, c, 0.0],
                    [Some(PI), Some(PI)],
                ),
            ];
            for (plane, origin, normal, want) in cases {
                let what = format!("{side:?}, phase {phase}, sweep {sweep}: {plane}");
                for (form, body) in [("one segment", &one), ("two arcs", &two)] {
                    let got = split_volumes(body, origin, normal, &what)
                        .unwrap_or_else(|e| panic!("{what}, {form}: the split builds: {e:?}"));
                    for (got, want) in got.iter().zip(&want) {
                        match (got, want) {
                            (Some(got), Some(want)) => {
                                close(*got, *want, &format!("{what}, {form}"));
                            }
                            (None, None) => {}
                            _ => panic!("{what}, {form}: sides {got:?}, want {want:?}"),
                        }
                    }
                }
            }
        }
    }
}

/// **`segment_curve` converts an arc back to its own start only as one
/// full turn.** A zero chord with a half turn or two turns is refused,
/// typed, and so is a full turn between two distinct points; the full
/// turn converts and closes on its start.
#[test]
fn segment_curve_converts_a_closed_arc_only_as_one_full_turn() {
    use geom_brep::SketchSegment;
    let place = geom_core::Affine3::translation(geom_core::Vec3::new(0.0, 0.0, 0.0));
    let arc = |sweep: f64| Arc2 {
        centre: Point2::new(0.0, 0.0),
        radius: 1.0,
        sweep,
    };
    let (a, b) = (Point2::new(1.0, 0.0), Point2::new(-1.0, 0.0));
    for (what, seg) in [
        (
            "a half turn back to its start",
            SketchSegment::Arc {
                a,
                b: a,
                arc: arc(PI),
            },
        ),
        (
            "two turns back to its start",
            SketchSegment::Arc {
                a,
                b: a,
                arc: arc(2.0 * TAU),
            },
        ),
        (
            "a full turn between two points",
            SketchSegment::Arc {
                a,
                b,
                arc: arc(TAU),
            },
        ),
    ] {
        assert!(
            matches!(
                sweep::segment_curve(0, seg, place),
                Err(sweep::SkinError::DegenerateSection { section: 0, .. })
            ),
            "{what}"
        );
    }
    for sweep in [TAU, -TAU] {
        let curve = sweep::segment_curve(
            0,
            SketchSegment::Arc {
                a,
                b: a,
                arc: arc(sweep),
            },
            place,
        )
        .unwrap_or_else(|e| panic!("a full turn of {sweep} converts: {e}"));
        let (t0, t1) = curve.domain();
        let (start, end) = (curve.eval(t0), curve.eval(t1));
        assert!(
            start.distance(end) < 1e-12 && (start.x - 1.0).abs() < 1e-12,
            "a full turn of {sweep} closes on its start: {start:?} → {end:?}"
        );
    }
}
