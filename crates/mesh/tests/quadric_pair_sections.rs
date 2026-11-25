//! **Booleans of quadric pairs whose section is a closed fitted loop
//! build, and mesh.** A closed fitted section loop is a periodic cubic,
//! seated by its span; an edge on it is the loop's carrier and a span,
//! read as the clamped view over that span, whose interior knots are
//! simple (`crates/geom-brep/README.md` C11). The four pairs below each
//! built and refused in the chord pass while a seated loop carried its
//! own old start, a full-multiplicity knot, inside an edge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::CurveKind;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::test_support::{finished, lens, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude, revolve};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

use crate::common::{axis_y, validated};

fn tol() -> Tol {
    Tol::witness()
}

/// A disc of radius `r` about `(0, 0, z0)` extruded `depth` along `z`.
fn rod(r: f64, z0: f64, depth: f64) -> Body<f64> {
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol()).unwrap();
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let profile = Profile::new(plane, vec![lp.into()])
        .validate(tol())
        .unwrap();
    let side = ExtrudeSide::Along;
    extrude(&profile, Extrusion::Distance { depth, side }, tol())
        .unwrap()
        .body
        .into_body()
}

/// The ball of radius `r` about the origin, poled along `y`.
fn ball(r: f64) -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        tol(),
    )
}

/// The donut of major radius `big` and tube radius `r` about `y`.
fn donut(big: f64, r: f64) -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(big, -r), 1.0),
        (Point2::new(big, r), 1.0),
    ]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full, tol())
        .unwrap()
        .body
        .into_body()
}

fn posed(what: &str, raw: Body<f64>, pose: Affine3<f64>) -> AtRestBody<f64> {
    finished(
        what,
        topo::transform_rigid(&raw, &geom_core::test_support::rigid(&pose, tol())).unwrap(),
        tol(),
    )
}

type Run = (&'static str, Result<BooleanResult<f64>, BooleanError>);

fn six(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> [Run; 6] {
    [
        ("A ∪ B", topo::union(a, b, tol())),
        ("B ∪ A", topo::union(b, a, tol())),
        ("A ∩ B", topo::intersect(a, b, tol())),
        ("B ∩ A", topo::intersect(b, a, tol())),
        ("A ∖ B", topo::subtract(a, b, tol())),
        ("B ∖ A", topo::subtract(b, a, tol())),
    ]
}

/// A body's spline edges, each as a reader reads it.
fn spline_edges(body: &Body<f64>) -> Vec<(topo::EdgeKey, &geom_brep::EdgeCurve<f64>)> {
    body.edges()
        .filter_map(|(ek, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            (c.carrier().kind() == CurveKind::Nurbs).then_some((ek, c))
        })
        .collect()
}

/// What a built body's section edges are.
#[derive(Debug, PartialEq, Eq)]
struct Section {
    /// Its spline edges.
    edges: usize,
    /// Those stored as a span of a periodic cubic.
    on_loops: usize,
    /// The greatest interior knot multiplicity any of them reads.
    worst_knot: usize,
    /// Those that are one whole closed loop: both ends one vertex, the
    /// span one period.
    whole_loops: usize,
}

fn section_of(body: &Body<f64>) -> Section {
    let mut s = Section {
        edges: 0,
        on_loops: 0,
        worst_knot: 0,
        whole_loops: 0,
    };
    for (ek, c) in spline_edges(body) {
        let geom::Curve3::Nurbs(net) = c.carrier() else {
            unreachable!("filtered to splines");
        };
        s.edges += 1;
        let runs: Vec<(f64, usize)> = net.knot_values().runs().collect();
        for &(_, m) in &runs[1..runs.len() - 1] {
            s.worst_knot = s.worst_knot.max(m);
        }
        let stored = c.stored_carrier();
        s.on_loops += usize::from(stored.kind() == CurveKind::Periodic);
        let (start, end) = body.edge_vertices(ek).expect("an edge's ends");
        let (t0, t1) = c.params();
        // One period to rounding: a loop joined across its seat states
        // `(t, t + T)`, whose difference rounds.
        let turn = stored
            .period()
            .is_some_and(|p| ((t1 - t0) - p.length()).abs() <= 1e-12 * p.length());
        let whole = start == end && turn;
        // Check 2's turn is the stored loop's: a loop edge records where
        // its span stands against the loop's period, a whole turn for
        // the closed edge and short of one for an arc.
        if stored.kind() == CurveKind::Periodic {
            let want = if whole {
                geom_core::period::Within::Whole
            } else {
                geom_core::period::Within::Short
            };
            assert_eq!(
                c.turn().map(geom_brep::CertifiedTurn::turn),
                Some(want),
                "a loop edge over ({t0}, {t1}) records its turn"
            );
        }
        s.whole_loops += usize::from(whole);
    }
    s
}

/// One `tessellate` answer.
#[derive(Debug, PartialEq, Eq, Clone)]
enum Got {
    /// `Ok`, and `check_mesh` passes.
    Meshed,
    /// `UnsupportedCurve` from `chords::chart_image`: a sphere or a
    /// torus face bounded by a spline's projected image, which the
    /// trimmed lane has no speed bound for
    /// (`work/tesslane/cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm.md`,
    /// `work/tesslane/a-torus-face-bounded-by-a-projected-spline-image-has-no-chord-bound.md`).
    ProjectedSpline,
    /// `UnsupportedCurve` from `chords::nurbs_chord_count`: a carrier
    /// with an interior knot of multiplicity its degree.
    Kink,
    /// Anything else.
    Other(String),
}

fn got(body: &Body<f64>, delta: f64) -> Got {
    use mesh::TessellateError as E;
    match mesh::tessellate(body, delta, tol()) {
        Ok(m) => match mesh::validate::check_mesh(&m) {
            Ok(()) => Got::Meshed,
            Err(e) => Got::Other(format!("meshed, and check_mesh refuses: {e:?}")),
        },
        Err(E::UnsupportedCurve { note, .. }) if note.contains("C⁰ kink") => Got::Kink,
        Err(E::UnsupportedCurve { note, .. })
            if note.contains("the projected image of a spline or line carrier") =>
        {
            Got::ProjectedSpline
        }
        Err(e) => Got::Other(format!("{e:?}")),
    }
}

const DELTAS: [f64; 3] = [2e-2, 5e-3, 1e-3];

/// Every op of `a` and `b` builds; every spline edge of each result is
/// a span of a periodic cubic whose reading holds simple interior knots
/// only; and each result answers `want` at every δ.
fn assert_ops(
    what: &str,
    runs: impl IntoIterator<Item = Run>,
    deltas: &[f64],
    want: &Got,
) -> Vec<Section> {
    runs.into_iter()
        .map(|(op, run)| {
            let label = format!("{what}, {op}");
            let built = run.unwrap_or_else(|e| panic!("{label}: the op builds, got {e:?}"));
            let body = &built
                .body()
                .unwrap_or_else(|| panic!("{label}: a body"))
                .body;
            let section = section_of(body);
            assert!(
                section.edges > 0 && section.on_loops == section.edges,
                "{label}: every section edge is a span of a periodic loop: {section:?}"
            );
            assert_eq!(
                section.worst_knot, 1,
                "{label}: no interior knot of multiplicity past one: {section:?}"
            );
            for &delta in deltas {
                assert_eq!(&got(body, delta), want, "{label}, δ = {delta:e}");
            }
            section
        })
        .collect()
}

fn cylinder_and_ball(s: f64) -> (AtRestBody<f64>, AtRestBody<f64>) {
    (
        finished("the cylinder", rod(0.5 * s, -s, 2.0 * s), tol()),
        posed(
            "the ball",
            ball(0.3 * s),
            Affine3::translation(Vec3::new(0.5 * s, 0.0, 0.0)),
        ),
    )
}

/// **Two rods crossing off-axis build, and mesh**: a rod of radius 0.2
/// across one of radius 0.5, two section loops, every op in both member
/// orders, at three δ, each mesh passing `check_mesh`.
#[test]
fn two_rods_crossing_build_and_mesh() {
    let x = Vec3::new(1.0, 0.0, 0.0);
    let wide = finished("the wide rod", rod(0.5, -1.0, 2.0), tol());
    let thin = posed(
        "the thin rod",
        rod(0.2, -1.5, 3.0),
        Affine3::translation(Vec3::new(0.15, 0.0, 0.1))
            * Affine3::rotation_about_axis(Point3::origin(), x, core::f64::consts::FRAC_PI_2),
    );
    assert_ops("the rods", six(&wide, &thin), &DELTAS, &Got::Meshed);
}

/// **A ball across a cylinder's wall builds on one periodic loop, and
/// its arcs rejoin.** The first pose of `sweep`'s
/// `cylinder_sphere_frame`, the ball centred on the wall. Every op
/// builds; no section edge reads an interior knot past multiplicity
/// one, so the chord pass takes every carrier; and the mesh stops later,
/// at the sphere face's projected spline image, which the trimmed lane
/// does not bound.
///
/// **The rejoin**: the intersection is a lens of one cylinder face and
/// one sphere face. Its loop was cut in four at the operands' seams,
/// which the merge removes; the four vertices are then each between two
/// arcs of one loop and two faces, and the join takes them all, leaving
/// the loop one closed edge over one whole period.
#[test]
fn a_ball_across_a_cylinder_wall_builds_on_one_loop_and_its_arcs_rejoin() {
    let (cylinder, ball) = cylinder_and_ball(1.0);
    let sections = assert_ops(
        "the cylinder and the ball",
        six(&cylinder, &ball),
        &DELTAS,
        &Got::ProjectedSpline,
    );
    let one_loop = Section {
        edges: 1,
        on_loops: 1,
        worst_knot: 1,
        whole_loops: 1,
    };
    assert_eq!(sections[2], one_loop, "cylinder ∩ ball");
    assert_eq!(sections[3], one_loop, "ball ∩ cylinder");
}

/// **No interior knot of multiplicity past one at any scale**: the
/// cylinder and the ball at `s = 0.01`, `1` and `100`, intersected, each
/// lens one closed loop edge that tier 3 takes at rest. The ε rows are
/// the suite's (`CAD_TOLERANCE_EPS`).
#[test]
fn a_loops_edges_read_simple_knots_at_every_scale() {
    for s in [0.01, 1.0, 100.0] {
        let (cylinder, ball) = cylinder_and_ball(s);
        let built = topo::intersect(&cylinder, &ball, tol())
            .unwrap_or_else(|e| panic!("s = {s}: the lens builds, got {e:?}"));
        let body = &built.body().expect("a body").body;
        // Tier 3 re-certifies the loop's edge from its stored carrier
        // and span, at the edge door.
        let _ = finished(&format!("the lens at s = {s}"), body.clone(), tol());
        assert_eq!(
            section_of(body),
            Section {
                edges: 1,
                on_loops: 1,
                worst_knot: 1,
                whole_loops: 1,
            },
            "s = {s}"
        );
        // Moved, the lens keeps its loop: the stored carrier is the
        // loop moved, the reading its view, and tier 3 takes it.
        let motion = Affine3::translation(Vec3::new(0.3, -0.2, 0.7) * s)
            * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.6, 0.8), 0.9);
        let moved =
            topo::transform_rigid(body, &geom_core::test_support::rigid(&motion, tol())).unwrap();
        assert_eq!(section_of(&moved), section_of(body), "s = {s}: moved");
        let (was, now) = (spline_edges(body), spline_edges(&moved));
        let ((_, c0), (_, c1)) = (&was[0], &now[0]);
        let (t0, t1) = c0.params();
        for k in 0..=50 {
            let t = t0 + (t1 - t0) * f64::from(k) / 50.0;
            let want = motion.transform_point(c0.carrier().eval(t));
            for (what, got) in [
                ("reading", c1.carrier().eval(t)),
                ("stored", c1.stored_carrier().eval(t)),
            ] {
                let gap = (got - want).norm();
                assert!(
                    gap < 1e-12 * s.max(1.0),
                    "s = {s}: the moved {what} at {t}: {gap:e}"
                );
            }
        }
        let _ = finished(&format!("the moved lens at s = {s}"), moved, tol());
    }
}

/// **A ball and a rod against a donut build on periodic loops.** The
/// ball and the donut are each one face round their axis, and their
/// section, a null oval where the ball scrapes the tube, crosses neither
/// face's seam: the section certificate counts it inside both faces, the
/// ball's face is cut in along a meridian the oval crosses and glued
/// back, and the result carries no trace of the cut. Every op of the
/// pair builds in both operand orders; each volume answers the operands'
/// closed forms with the lens's, and the lens answers its integral over
/// slices across the donut's axis (`sweep::test_support::lens`). The union, the
/// intersection and the difference of the donut and the rod build too.
/// The mesh of each union and intersection stops at the sphere's and the
/// torus's faces' projected spline images.
#[test]
fn a_ball_and_a_rod_against_a_donut_build_on_periodic_loops() {
    let x = Vec3::new(1.0, 0.0, 0.0);
    let ball = finished("the ball", ball(0.5), tol());
    let pose = Affine3::translation(Vec3::new(0.1, 0.35, 0.2))
        * Affine3::rotation_about_axis(Point3::origin(), x, 0.7);
    let ring = posed("the donut", donut(0.6, 0.2), pose);
    let runs = six(&ball, &ring);
    cut_in_volumes(
        "the ball and the donut",
        (&runs, [&ball, &ring]),
        (ball_volume(0.5), DONUT_VOLUME),
        donut_lens(Vec3::new(0.1, 0.35, 0.2), 0.7),
    );
    assert_ops(
        "the ball and the donut",
        runs,
        &[5e-3],
        &Got::ProjectedSpline,
    );
    let donut = finished("the donut", donut(0.6, 0.2), tol());
    let rod = posed(
        "the rod",
        rod(0.1, -1.0, 2.0),
        Affine3::translation(Vec3::new(0.62, 0.03, 0.0)),
    );
    assert_ops(
        "the donut and the rod",
        [
            ("A ∪ B", topo::union(&donut, &rod, tol())),
            ("A ∩ B", topo::intersect(&donut, &rod, tol())),
            ("A ∖ B", topo::subtract(&donut, &rod, tol())),
        ],
        &[5e-3],
        &Got::ProjectedSpline,
    );
}

/// **A ball against a donut turned a quarter builds in both operand
/// orders.** The donut turned a quarter about `x` and moved by
/// `(0.05, 0, 0.3)` scrapes the ball in an oval inside both faces, which
/// the ball's face is cut in across. Taken ball first, the oval's carrier
/// is the loop run back, and the edge the output's join closes round it
/// is a whole period of that carrier, not the far end of the arc it
/// absorbs, which names the same point only up to rounding: every op
/// builds in both orders, with no trace of the cut, at the operands'
/// closed forms with the lens.
#[test]
fn a_ball_against_a_donut_turned_a_quarter_builds_in_both_orders() {
    let x = Vec3::new(1.0, 0.0, 0.0);
    // The turn the pose was found at, short of a quarter by a few
    // parts in 10⁸: the witness, not the constant.
    #[allow(clippy::approx_constant)]
    let (by, turn) = (Vec3::new(0.05, 0.0, 0.30), 1.570_796_3);
    let ball = finished("the ball", ball(0.5), tol());
    let ring = posed(
        "the donut",
        donut(0.6, 0.2),
        Affine3::translation(by) * Affine3::rotation_about_axis(Point3::origin(), x, turn),
    );
    cut_in_volumes(
        "the ball and the donut turned a quarter",
        (&six(&ball, &ring), [&ball, &ring]),
        (ball_volume(0.5), DONUT_VOLUME),
        donut_lens(by, turn),
    );
}

/// **A loop with no site takes one from a cut-in.** The ball turned so
/// its seam plane misses the cap the wall cuts from it, and the cylinder
/// turned so its seam lines miss it: the section loop is interior to a
/// face of each solid, so no event seats it. The ball's face is cut in
/// along a meridian its tube is read crossing, which seats it where
/// it crosses, and the output merge glues the cut away: every op builds
/// with no trace of the cut, and each volume answers the operands'
/// closed forms with the lens's, which answers its quadrature. The
/// unseated loop's reading is `geom-brep`'s row
/// (`pair_section::the_general_rung_carries_through_its_door`).
#[test]
fn a_loop_with_no_site_takes_one_from_a_cut_in() {
    let z = Vec3::new(0.0, 0.0, 1.0);
    let y = Vec3::new(0.0, 1.0, 0.0);
    let quarter = core::f64::consts::FRAC_PI_2;
    let cylinder = posed(
        "the cylinder",
        rod(0.5, -1.0, 2.0),
        Affine3::rotation_about_axis(Point3::origin(), z, quarter),
    );
    let ball = posed(
        "the ball",
        ball(0.3),
        Affine3::translation(Vec3::new(0.7, 0.0, 0.0))
            * Affine3::rotation_about_axis(Point3::origin(), y, quarter),
    );
    let rod = |z: f64| (z.abs() < 1.0).then_some((0.0, 0.5));
    cut_in_volumes(
        "the cylinder and the ball",
        (&six(&cylinder, &ball), [&cylinder, &ball]),
        (PI * 0.25 * 2.0, ball_volume(0.3)),
        lens::ball_in_revolved(0.3, (0.7, 0.0), rod, LENS_PANELS),
    );
}

/// `4πr³/3`.
fn ball_volume(r: f64) -> f64 {
    4.0 / 3.0 * PI * r * r * r
}

/// `2π²Rr²` of [`donut`]`(0.6, 0.2)`.
const DONUT_VOLUME: f64 = 2.0 * PI * PI * 0.6 * 0.04;

/// The midpoint rule's coarser panel count for a lens
/// ([`lens::ball_in_revolved`]); the finer is twice it.
const LENS_PANELS: u32 = 2000;

/// How many times the lens integral's stated error a volume may stand
/// from it, beside the measured pads.
const LENS_ERRORS: f64 = 10.0;

/// **The lens of [`ball`]`(0.5)` at the origin and [`donut`]`(0.6, 0.2)`
/// turned `turn` about `x` and moved by `by`**: the ball's centre read in
/// the donut's frame, where the donut is about `y`, and the lens
/// integrated over slices across that axis, with its error.
fn donut_lens(by: Vec3<f64>, turn: f64) -> (f64, f64) {
    let (c, s) = (turn.cos(), turn.sin());
    let q = Vec3::new(-by.x, -by.y, -by.z);
    let (x, y, z) = (q.x, c * q.y + s * q.z, -s * q.y + c * q.z);
    lens::ball_in_revolved(
        0.5,
        ((x * x + z * z).sqrt(), y),
        lens::torus_slice(0.6, 0.2),
        LENS_PANELS,
    )
}

/// **What a pair whose section loop is cut in builds**: every run, with
/// no trace of the cut (`sweep::test_support::cut_trace`), each volume
/// against the operands' closed forms `va` and `vb` with the lens, the
/// intersection's volume, and the lens against its integral `want`, within
/// its pad and [`LENS_ERRORS`] times the integral's stated `error`. A
/// run's label names its op as [`six`] does.
fn cut_in_volumes(
    what: &str,
    (runs, operands): (&[Run], [&Body<f64>; 2]),
    (va, vb): (f64, f64),
    (want, error): (f64, f64),
) {
    let volume = |op: &str, r: &Result<BooleanResult<f64>, BooleanError>| {
        let built = r
            .as_ref()
            .unwrap_or_else(|e| panic!("{what}, {op}: the op builds, got {e:?}"));
        sweep::test_support::cut_trace::assert_no_cut_trace(
            &format!("{what}, {op}"),
            built,
            operands,
        );
        let body = &built.body().expect("a body").body;
        let m = topo::mass_properties(body, tol(), topo::test_support::pinned_resolution(tol()))
            .unwrap_or_else(|e| panic!("{what}, {op}: a volume, got {e:?}"));
        (m.enclosure.volume, m.enclosure.volume_pad)
    };
    let read: Vec<(&str, (f64, f64))> = runs.iter().map(|(op, r)| (*op, volume(op, r))).collect();
    let &(_, (lens, lens_pad)) = read
        .iter()
        .find(|(op, _)| op.contains('∩'))
        .expect("an intersection among the runs");
    assert!(
        lens > 0.0 && (lens - want).abs() <= lens_pad + LENS_ERRORS * error + 1e-12,
        "{what}: the lens {lens} ± {lens_pad} against its integral {want} ± {error}"
    );
    for (op, (got, pad)) in read {
        let want = match op {
            "A ∪ B" | "B ∪ A" => va + vb - lens,
            "A ∩ B" | "B ∩ A" => lens,
            "A ∖ B" => va - lens,
            "B ∖ A" => vb - lens,
            other => panic!("{what}: no closed form for {other}"),
        };
        assert!(
            (got - want).abs() <= pad + lens_pad + 1e-9,
            "{what}, {op}: {got} ± {pad}, the closed forms with the lens {want}"
        );
    }
}
