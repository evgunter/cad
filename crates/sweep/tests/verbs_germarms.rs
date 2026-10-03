//! **The curved pierce RING lane**: what happens when a straight edge
//! definitely crosses a cylinder WALL rather than a cap.
//!
//! Until this unit the crossing layer had one plane-only assumption
//! left — the pierced face's oriented datum — and the whole family
//! stopped at one typed door, `CurvedPierceUnsupported`, whatever the
//! configuration behind it. The rows here are what the ring lane
//! actually buys, measured rather than asserted:
//!
//! - a box driven through a wall has its crossings FOUND: both operands
//!   split, the pierce ring inserts, and every boolean builds to the
//!   closed form, whether the section runs seam to seam or closes inside
//!   one wall face;
//! - a box definitely clear of the wall still answers, bit for bit;
//! - a box that GRAZES the wall keeps the pierce door, because a
//!   tangency is not a crossing at any order this lane sees;
//! - a cone wall keeps its own door, which is a different one.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::brick;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, BooleanError};

/// A cylinder about the z axis, `r = 1`, `z ∈ [−2, 2]` — the wall every
/// row below pierces.
fn pipe() -> Body<f64> {
    pipe_at((0.0, 0.0))
}

/// [`pipe`] with its axis through `(cx, cy)`.
fn pipe_at((cx, cy): (f64, f64)) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(cx, cy), 1.0, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -2.0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(4.0), tol)
        .unwrap()
        .body
}

fn union_err(a: &Body<f64>, b: &Body<f64>) -> BooleanError {
    topo::union(a, b, Tol::witness()).expect_err("this pair has no join arm yet")
}

/// `∫ √(1 − y²) dy`.
fn half_chord_integral(y: f64) -> f64 {
    0.5 * (y * (1.0 - y * y).sqrt() + y.asin())
}

/// The area the rectangle `x ∈ [x0, x1]`, `y ∈ [y0, y1]` shares with the
/// unit disc, for the two shapes these rows drive: a rectangle spanning
/// every chord it crosses (`x0 ≤ −1`, `x1 ≥ 1`), or one starting at
/// `x0 > 0` inside the disc and leaving it (`x1 ≥ 1`), so every chord
/// it meets runs from `x0` to the circle.
fn rect_disc_area((x0, x1): (f64, f64), (y0, y1): (f64, f64)) -> f64 {
    assert!(
        x1 >= 1.0,
        "the rows' rectangles all leave the disc on the right"
    );
    let half = half_chord_integral(y1) - half_chord_integral(y0);
    if x0 <= -1.0 {
        2.0 * half
    } else {
        half - x0 * (y1 - y0)
    }
}

/// `a` and `b` under ∪, ∩ and both ∖, each held to every validation
/// tier and to its closed form, from the operands' volumes and the
/// volume they share.
fn assert_every_op(
    label: &str,
    (a, v_a): (&Body<f64>, f64),
    (b, v_b): (&Body<f64>, f64),
    shared: f64,
) {
    let tol = Tol::witness();
    let run = |op: &str, out: Result<topo::BooleanResult<f64>, BooleanError>, truth: f64| {
        let label = format!("{label}, {op}");
        let out = out.unwrap_or_else(|e| panic!("{label}: refused {e:?}"));
        let topo::BooleanResult::Body(out) = out else {
            panic!("{label}: came back empty");
        };
        assert_eq!(topo::validate(&out.body), Ok(()), "{label}: validate");
        assert_eq!(topo::validate_closed(&out.body), Ok(()), "{label}: closed");
        assert_eq!(
            topo::validate_geometric(&out.body, tol),
            Ok(()),
            "{label}: tier 3"
        );
        let m = topo::mass_properties(&out.body, tol)
            .unwrap_or_else(|e| panic!("{label}: mass properties {e:?}"));
        assert_eq!(m.volume_pad, 0.0, "{label}: closed-form faces only");
        assert!(
            (m.volume - truth).abs() <= 1e-12 * truth.max(1.0),
            "{label}: volume {} against the closed form {truth}",
            m.volume
        );
    };
    run("∪", topo::union(a, b, tol), v_a + v_b - shared);
    run("∩", topo::intersect(a, b, tol), shared);
    run("a ∖ b", topo::subtract(a, b, tol), v_a - shared);
    run("b ∖ a", topo::subtract(b, a, tol), v_b - shared);
}

/// The pipe and a bar through its wall under ∪, ∩ and both ∖ (
/// [`assert_every_op`]): the shared volume is the bar's height times
/// [`rect_disc_area`].
fn assert_bar_through_the_pipe(x: (f64, f64), y: (f64, f64), z: (f64, f64)) {
    assert_bar_through_the_pipe_at((0.0, 0.0), x, y, z);
}

/// [`assert_bar_through_the_pipe`] with the pipe's axis through `c`
/// and the bar moved with it, so the volumes are the same.
fn assert_bar_through_the_pipe_at(c: (f64, f64), x: (f64, f64), y: (f64, f64), z: (f64, f64)) {
    let bar = brick(
        (x.0 + c.0, x.1 + c.0),
        (y.0 + c.1, y.1 + c.1),
        z,
        Tol::witness(),
    );
    let shared = (z.1 - z.0) * rect_disc_area(x, y);
    let v_bar = (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0);
    assert_every_op(
        &format!("pipe at {c:?}, bar {x:?} × {y:?} × {z:?}"),
        (&pipe_at(c), PI * 4.0),
        (&bar, v_bar),
        shared,
    );
}

/// A block `[cx ± 2] × [cy ± 2] × [−1, 1]` bored through by [`pipe_at`]
/// `c`, built by the boolean itself: its bore is a cylinder wall whose
/// material lies OUTSIDE the cylinder, a face of reversed sense.
fn bored_block_at(c: (f64, f64)) -> (Body<f64>, f64) {
    let tol = Tol::witness();
    let block = brick(
        (c.0 - 2.0, c.0 + 2.0),
        (c.1 - 2.0, c.1 + 2.0),
        (-1.0, 1.0),
        tol,
    );
    let topo::BooleanResult::Body(out) =
        topo::subtract(&block, &pipe_at(c), tol).expect("the block bores")
    else {
        panic!("the bored block came back empty");
    };
    (out.body, 32.0 - 2.0 * PI)
}

/// The bored block and a bar crossing its bore under every op: the bar
/// shares all of itself with the block's material but the part in the
/// bore, its height times [`rect_disc_area`].
fn assert_bar_across_the_bore_at(c: (f64, f64), x: (f64, f64), y: (f64, f64), z: (f64, f64)) {
    let (bored, v_bored) = bored_block_at(c);
    let bar = brick(
        (x.0 + c.0, x.1 + c.0),
        (y.0 + c.1, y.1 + c.1),
        z,
        Tol::witness(),
    );
    let v_bar = (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0);
    let shared = v_bar - (z.1 - z.0) * rect_disc_area(x, y);
    assert_every_op(
        &format!("bore at {c:?}, bar {x:?} × {y:?} × {z:?}"),
        (&bored, v_bored),
        (&bar, v_bar),
        shared,
    );
}

/// **The row the ring lane exists for.** A bar driven straight through
/// the pipe crosses the wall in eight places — four box edges, twice
/// each — and every one of them is strictly inside a wall face, on no
/// boundary of either operand. Each half-wall's section runs seam to
/// seam, through the pierce rings, and every op builds.
#[test]
fn a_bar_driven_through_a_wall_builds() {
    assert_bar_through_the_pipe((-1.1, 1.1), (-0.3, 0.3), (-0.3, 0.3));
}

/// The one-sided pose: the bar starts INSIDE the pipe and leaves
/// through the wall once. Its endpoint sides are `(Negative, Positive)`
/// rather than `(Positive, Positive)`, so it enters the lane through
/// the straddle arm instead of the belly arm — a different route to the
/// same roots, and worth its own row because the two arms are argued
/// differently.
#[test]
fn a_bar_leaving_through_one_side_of_a_wall_builds() {
    assert_bar_through_the_pipe((0.5, 1.1), (-0.3, 0.3), (-0.3, 0.3));
}

/// **The asymmetric pose.** The pipe's wall is two faces split at the
/// seam rulings `(±1, 0, z)`, and the bars above straddle both, so each
/// half-wall's section runs seam to seam. Lifted to `y ∈ [0.15, 0.7]`
/// the bar's section on each side closes INSIDE one wall face —
/// azimuths 8.6° to 44.4°, clear of every seam — so the wall is left
/// with a hole: the section's chords join ring to ring, the island's
/// role order is wound on the wall's chart, and the ringed wall
/// measures. Rings whose arcs are wider than the gap between them
/// (`asin y1 − asin y0 > π − 2·asin y1`) refuse instead,
/// `RingHomingAmbiguous`: the loose ends pair across the gap
/// (`work/tang/in-face-pierce-rings-pair-across-the-gap.md`).
#[test]
fn a_bar_whose_section_closes_inside_one_wall_face_builds() {
    assert_bar_through_the_pipe((-1.1, 1.1), (0.15, 0.7), (-0.4, 0.1));
}

/// **The rows above, with the pipe's axis off the world origin.** A
/// cylinder face's flux is `R²·A + origin·A⃗` summed over every loop,
/// rings included; with the axis through the origin a ring's
/// `origin·A⃗` is `(0, 0, −2)·A⃗`, and a ring on a wall bounds no
/// axial vector area, so dropping that term moved no row. At
/// `(3, −2)` it carries the ring's whole offset: drop it and `rod ∖ bar`
/// measured 11.8951 against 12.0782 with every tier green.
#[test]
fn the_ringed_wall_rows_hold_off_the_origin() {
    let c = (3.0, -2.0);
    assert_bar_through_the_pipe_at(c, (-1.1, 1.1), (-0.3, 0.3), (-0.3, 0.3));
    assert_bar_through_the_pipe_at(c, (0.5, 1.1), (-0.3, 0.3), (-0.3, 0.3));
    assert_bar_through_the_pipe_at(c, (-1.1, 1.1), (0.15, 0.7), (-0.4, 0.1));
}

/// **A thin bar turned about two axes gets through the join — and needs
/// the exact closure to.** The bar `x ∈ [0.3, 3]`, `y ∈ [0.2, 0.8]`,
/// 1 mm thick, turned 0.3 rad about `x` and then 0.5 rad about `y`: its
/// section on the wall is a sinusoid on the chart, and the island the
/// ring lane winds hugs it. Closing that island with a straight chart
/// segment instead of the exact section reverses its sign (the delta
/// review's MI3) and the union refuses `SeamOrientation` at the zip.
/// With the exact closure the ∩ and bar ∖ pipe build, pass every tier
/// and balance against the bar; the ∪ and pipe ∖ bar build too, and
/// stop one layer later, at the volume backstop: their wall carries a
/// ring trimmed by ellipse arcs, which no volume lane reads yet
/// (`work/props/an-ellipse-trimmed-ring-on-a-cylinder-wall-has-no-volume-lane.md`).
#[test]
fn a_thin_bar_turned_about_two_axes_gets_through_the_join() {
    let tol = Tol::witness();
    let t = 0.001;
    let bar = brick((0.3, 3.0), (0.2, 0.8), (-t / 2.0, t / 2.0), tol);
    let o = geom_core::Point3::origin();
    let turned = topo::transform_rigid(
        &bar,
        &Affine3::rotation_about_axis(o, Vec3::new(1.0, 0.0, 0.0), 0.3),
        tol,
    )
    .expect("a rigid pose");
    let bar = topo::transform_rigid(
        &turned,
        &Affine3::rotation_about_axis(o, Vec3::new(0.0, 1.0, 0.0), 0.5),
        tol,
    )
    .expect("a rigid pose");
    let pipe = pipe();
    let measure = |op: &str, out: Result<topo::BooleanResult<f64>, BooleanError>| {
        let out = out.unwrap_or_else(|e| panic!("turned thin bar, {op}: refused {e:?}"));
        let topo::BooleanResult::Body(out) = out else {
            panic!("turned thin bar, {op}: came back empty");
        };
        assert_eq!(
            topo::validate_geometric(&out.body, tol),
            Ok(()),
            "turned thin bar, {op}: tier 3"
        );
        let m = topo::mass_properties(&out.body, tol)
            .unwrap_or_else(|e| panic!("turned thin bar, {op}: mass properties {e:?}"));
        (m.volume, m.volume_pad)
    };
    let (vi, pi) = measure("∩", topo::intersect(&pipe, &bar, tol));
    let (vd, pd) = measure("bar ∖ pipe", topo::subtract(&bar, &pipe, tol));
    let v_bar = 2.7 * 0.6 * t;
    assert!(
        vi > 0.0 && vd > 0.0 && (vi + vd - v_bar).abs() <= pi + pd + 1e-12 * v_bar,
        "∩ {vi} + bar ∖ pipe {vd} against the bar {v_bar}"
    );
    for (op, out) in [
        ("∪", topo::union(&pipe, &bar, tol)),
        ("pipe ∖ bar", topo::subtract(&pipe, &bar, tol)),
    ] {
        assert!(
            matches!(
                out,
                Err(BooleanError::VolumeUnmeasured {
                    source: topo::MassPropsError::RingOnCurvedFace { .. },
                    ..
                })
            ),
            "turned thin bar, {op}: expected the ellipse-ringed wall's volume backstop, got {:?}",
            out.map(|_| "a body")
        );
    }
}

/// **A wall's sense bit is checked against its boundary even when the
/// wall is notched or ringed.** Tier 3's check 6 reads a cylinder
/// face's side off every loop's chart area; flipping the bit of a
/// notched wall (the bar driven seam to seam) or of a ringed one (the
/// bar whose section closes inside one wall face) must refuse
/// `CurvedSenseInverted` on that face.
#[test]
fn a_notched_or_ringed_walls_flipped_sense_refuses() {
    let tol = Tol::witness();
    for (what, y, z, ringed) in [
        ("notched", (-0.3, 0.3), (-0.3, 0.3), false),
        ("ringed", (0.15, 0.7), (-0.4, 0.1), true),
    ] {
        let bar = brick((-1.1, 1.1), y, z, tol);
        let topo::BooleanResult::Body(out) = topo::union(&pipe(), &bar, tol).expect("builds")
        else {
            panic!("{what}: empty");
        };
        let body = out.body;
        let wall = body
            .faces()
            .find(|(_, f)| {
                let cylinder = matches!(
                    body.get_surface(f.surface),
                    Some(topo::Surface::Cylinder { .. })
                );
                let outer_edges = match body.get_loop(f.outer).map(|l| l.boundary) {
                    Some(topo::LoopBoundary::Cycle { first }) => {
                        body.loop_cycle(first).map_or(0, |c| c.len())
                    }
                    _ => 0,
                };
                cylinder
                    && if ringed {
                        !f.rings.is_empty()
                    } else {
                        f.rings.is_empty() && outer_edges > 4
                    }
            })
            .map(|(k, _)| k)
            .unwrap_or_else(|| panic!("{what}: the union has such a wall"));
        assert_eq!(
            topo::validate_geometric(&body, tol),
            Ok(()),
            "{what}: as built"
        );
        let flipped = body
            .flipped_face_sense_for_tests(wall)
            .expect("live face key");
        let errs = topo::validate_geometric(&flipped, tol)
            .expect_err(&format!("{what}: a flipped wall must refuse"));
        assert!(
            errs.iter().any(
                |e| matches!(e, topo::ValidationError::CurvedSenseInverted { face } if *face == wall)
            ),
            "{what}: expected CurvedSenseInverted on the flipped wall, got {errs:?}"
        );
    }
}

/// **A ring on a wall of reversed sense.** The bore of a bored block is
/// a cylinder face whose outward normal is `−r̂`: the island the ring
/// lane winds on its chart turns the other way about the outward
/// normal, and the role order reads the face's sense bit to say so.
/// Every pose, at the origin and off it.
#[test]
fn a_bar_across_a_bore_builds() {
    for c in [(0.0, 0.0), (3.0, -2.0)] {
        assert_bar_across_the_bore_at(c, (-1.1, 1.1), (-0.3, 0.3), (-0.3, 0.3));
        assert_bar_across_the_bore_at(c, (0.5, 1.1), (-0.3, 0.3), (-0.3, 0.3));
        assert_bar_across_the_bore_at(c, (-1.1, 1.1), (0.15, 0.7), (-0.4, 0.1));
    }
}

/// The OUT direction of the same reach, metered: a bar definitely clear
/// of the wall still answers, and answers with a volume. The operands'
/// padded extents overlap, so the pair is examined by the sweep rather
/// than pruned — the clearance is decided, not avoided.
#[test]
fn a_bar_clear_of_the_wall_still_answers() {
    let tol = Tol::witness();
    let topo::BooleanResult::Body(out) = topo::union(
        &pipe(),
        &brick((1.5, 2.5), (-0.3, 0.3), (-0.3, 0.3), tol),
        tol,
    )
    .expect("no crossing to route") else {
        panic!("two clear solids union into a two-shell body");
    };
    assert_eq!(out.body.shells().count(), 2);
    assert_eq!(topo::validate_geometric(&out.body, tol), Ok(()), "tier 3");
    let v = topo::mass_properties(&out.body, tol).unwrap().volume;
    let truth = PI * 4.0 + 1.0 * 0.6 * 0.6;
    assert!((v - truth).abs() < 1e-12, "{v} vs {truth}");
}

/// **A planted red for the lane's fence.** The same bar widened until
/// its long edges are TANGENT to the wall: `y = ±1` puts each of them
/// at distance exactly `r` from the axis, so the certified discriminant
/// is exactly zero and there is no crossing to find. A tangency ties
/// every first-order datum the pierce machinery reads, so the lane must
/// refuse it rather than pick a side — the pierce door, unchanged.
#[test]
fn a_bar_grazing_the_wall_keeps_the_pierce_door() {
    let (pipe, bar) = (
        pipe(),
        brick((-3.0, 3.0), (-1.0, 1.0), (-0.3, 0.3), Tol::witness()),
    );
    let err = union_err(&pipe, &bar);
    let BooleanError::CurvedPierceUnsupported { operand, edge, .. } = err else {
        panic!("a tangency is not a crossing: {err:?}");
    };
    // **The variant alone would not pin this row.** Any other frontier
    // of the same kind would satisfy it, so the refusing edge's CARRIER
    // is asserted too: the tangency this row plants is the bar's long
    // straight edge at `y = ±1`, and a Circle or a sphere face reaching
    // the same variant would be a different finding wearing this row's
    // name.
    let owner = match operand {
        topo::Operand::A => &pipe,
        topo::Operand::B => &bar,
    };
    let Some(topo::CurveGeom::Certified(c)) = owner
        .get_edge(edge)
        .and_then(|e| owner.get_curve_geom(e.curve))
    else {
        panic!("the named edge has no certified curve");
    };
    assert!(
        matches!(c.carrier(), topo::Curve3::Line { .. }),
        "the grazing red must refuse on the tangent LINE: {:?}",
        c.carrier()
    );
}

/// **The bar's length does not move the outcome.** The same bar made
/// LONG: at `x = ±3` against a wall of radius 1 the pierce vertex's
/// shorter edge fragment is 1.9 m, nearly twice the radius, so the
/// edge re-crosses nothing but runs far past where the wall's sagitta
/// outgrows its first-order departure. The sector side is a statement
/// about the bound near the vertex, and the curvature charge certifies
/// it at the distance where it is largest (`slope·r/2`), so the long
/// bar builds as the short one does.
#[test]
fn a_long_armed_bar_builds() {
    assert_bar_through_the_pipe((-3.0, 3.0), (-0.3, 0.3), (-0.3, 0.3));
}

/// **The kind fence, differential — and what it does and does not
/// witness.**
///
/// MEASURED, the frustum's bar meets `CurvedPairUnsupported { kind:
/// Cone, other_kind: Plane }`: the kind-PAIR operand gate, on the cone
/// face against the BAR's own plane face. It never reaches the crossing
/// layer's `f2` fold or `face_geo` at all, so this row does NOT witness
/// "the ring lane gave a cone no roots" — that pair has had no arm
/// since long before this lane, and the row reads identically with the
/// ring lane reverted.
///
/// It is kept for what it does witness, asserted positively rather than
/// as a not-the-other-door: a cone operand is stopped at the OUTERMOST
/// gate, so no cone geometry is ever handed to the wall lane in the
/// first place. That is a dead-belt fence — the inner fences
/// (`face_geo`'s `KindUnsupported`, the `f2` fold's Cylinder/Sphere-only
/// arms, and `wall_crossing`'s own non-cylinder `Unsettled`) are the
/// live ones and are unreachable from any authorable cone body while
/// this gate stands. If a later unit opens the pair gate for cones,
/// this row flips and the inner fences become the ones under test.
#[test]
fn a_cone_wall_is_stopped_at_the_outermost_gate() {
    let tol = Tol::witness();
    let frustum = {
        let lp = bulge_loop(
            [(0.2, 0.0), (0.6, 0.0), (0.4, 0.6), (0.2, 0.6)]
                .into_iter()
                .map(|(r, y)| (Point2::new(r, y), 0.0))
                .collect(),
        );
        let profile = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .unwrap();
        revolve(
            &profile,
            RevolveAxis {
                origin: Point2::new(0.0, 0.0),
                dir: Vec2::new(0.0, 1.0),
            },
            Revolution::Full,
            tol,
        )
        .unwrap()
        .body
    };
    let err = union_err(
        &frustum,
        &brick((-1.0, 1.0), (-0.05, 0.05), (0.25, 0.35), tol),
    );
    assert!(
        matches!(
            err,
            BooleanError::CurvedPairUnsupported {
                kind: geom::SurfaceKind::Cone,
                other_kind: geom::SurfaceKind::Plane,
                ..
            }
        ),
        "the cone's own door, asserted rather than excluded: {err:?}"
    );
}
