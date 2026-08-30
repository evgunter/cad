//! **The curved pierce RING lane**: what happens when a straight edge
//! definitely crosses a cylinder WALL rather than a cap.
//!
//! Until this unit the crossing layer had one plane-only assumption
//! left — the pierced face's oriented datum — and the whole family
//! stopped at one typed door, `CurvedPierceUnsupported`, whatever the
//! configuration behind it. The rows here are what the ring lane
//! actually buys, measured rather than asserted:
//!
//! - a box driven through a wall now has its crossings FOUND: both
//!   operands split, the pierce ring inserts, and the union refuses one
//!   layer down, at the JOIN, naming the arm that is missing there;
//! - a box definitely clear of the wall still answers, bit for bit;
//! - a box that GRAZES the wall keeps the pierce door, because a
//!   tangency is not a crossing at any order this lane sees;
//! - a cone wall keeps its own door, which is a different one.
//!
//! **The join refusal is the honest destination, not a shortfall.** A
//! pierce ring is an EMPTY loop carrying only null scaffolding, so the
//! run co-bounding a chord across it has no edge with a chart image and
//! the divided face has no azimuth window to select an arc against. The
//! planar sibling has the same shape and the same status — a box driven
//! through a cylinder CAP refuses at the join too (`verbs_pierce`) —
//! which is what says the missing arm is the RING's join, shared by
//! both, rather than anything this lane left undone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, BooleanError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A cylinder about the z axis, `r = 1`, `z ∈ [−2, 2]` — the wall every
/// row below pierces.
fn pipe() -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(p2(0.0, 0.0), 1.0, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -2.0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(4.0), tol)
        .unwrap()
        .body
}

fn boxx(x0: f64, x1: f64, y0: f64, y1: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp: ProfileLoop<f64> = RawLoop::polygon([p2(x0, y0), p2(x1, y0), p2(x1, y1), p2(x0, y1)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

fn union_err(a: &Body<f64>, b: &Body<f64>) -> BooleanError {
    topo::union(a, b, Tol::witness()).expect_err("this pair has no join arm yet")
}

/// **The row the ring lane exists for.** A bar driven straight through
/// the pipe crosses the wall in eight places — four box edges, twice
/// each — and every one of them is strictly inside a wall face, on no
/// boundary of either operand.
///
/// **The bar is short on purpose**, and the reason is the sector-side
/// curvature charge rather than the crossing lane: a pierce vertex's
/// sector arms are the split edge's two fragments, and a fragment
/// LONGER than the wall's radius makes the sagitta bound exceed any
/// first-order displacement, so no tangent-plane verdict there is
/// certifiable (`a_long_armed_bar_cannot_certify_its_sector_sides`
/// below is that pose, pinned). At `x = ±1.1` against `r = 1` the near
/// fragment is 0.146 m and the verdict stands. Before this unit the first of them
/// refused at the crossing layer with `CurvedPierceUnsupported`; the
/// door it refuses at now is the JOIN's, which is the measurement that
/// says the crossings were found, the edges split and the rings
/// inserted.
///
/// The site is asserted, not just the variant: `NoChartedRun` is the
/// pierce ring's own signature — the run carries only null scaffolding,
/// so there is no chart image to build an azimuth window from. A
/// different sub-case would mean a different story.
#[test]
fn a_bar_driven_through_a_wall_reaches_the_join() {
    let err = union_err(&pipe(), &boxx(-1.1, 1.1, -0.3, 0.3, -0.3, 0.3));
    assert!(
        matches!(
            err,
            BooleanError::Join(topo::SplitJoinError::SectionArcWindow {
                case: topo::ArcWindowCase::NoChartedRun,
                ..
            })
        ),
        "the crossing layer passes it; the ring's join arm is what is left: {err:?}"
    );
}

/// The one-sided pose: the bar starts INSIDE the pipe and leaves
/// through the wall once. Its endpoint sides are `(Negative, Positive)`
/// rather than `(Positive, Positive)`, so it enters the lane through
/// the straddle arm instead of the belly arm — a different route to the
/// same roots, and worth its own row because the two arms are argued
/// differently.
#[test]
fn a_bar_leaving_through_one_side_of_a_wall_reaches_the_join() {
    let err = union_err(&pipe(), &boxx(0.5, 1.1, -0.3, 0.3, -0.3, 0.3));
    assert!(
        matches!(
            err,
            BooleanError::Join(topo::SplitJoinError::SectionArcWindow {
                case: topo::ArcWindowCase::NoChartedRun,
                ..
            })
        ),
        "{err:?}"
    );
}

/// The OUT direction of the same reach, metered: a bar definitely clear
/// of the wall still answers, and answers with a volume. The operands'
/// padded extents overlap, so the pair is examined by the sweep rather
/// than pruned — the clearance is decided, not avoided.
#[test]
fn a_bar_clear_of_the_wall_still_answers() {
    let tol = Tol::witness();
    let topo::BooleanResult::Body(out) =
        topo::union(&pipe(), &boxx(1.5, 2.5, -0.3, 0.3, -0.3, 0.3), tol)
            .expect("no crossing to route")
    else {
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
    let (pipe, bar) = (pipe(), boxx(-3.0, 3.0, -1.0, 1.0, -0.3, 0.3));
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

/// **The curvature charge's planted red, on a real body.** The same
/// bar made LONG: at `x = ±3` against a wall of radius 1, the pierce
/// vertex's shorter edge fragment is 1.9 m, so the sagitta bound
/// `arm²/lever = 3.6 m` exceeds any first-order displacement the
/// sector can offer (which is at most `arm` itself). No tangent-plane
/// verdict about the material side is certifiable there, and the lane
/// refuses instead of answering one — the wrong answer it would
/// otherwise give is a wrong TOPOLOGY, not a conservative refusal
/// (`boolean::sectors::side_code` carries the witness).
///
/// This is the row that makes the short bar above a measurement rather
/// than a lucky pose: the two differ only in the bar's length, and they
/// land on different doors for a stated reason.
#[test]
fn a_long_armed_bar_cannot_certify_its_sector_sides() {
    let err = union_err(&pipe(), &boxx(-3.0, 3.0, -0.3, 0.3, -0.3, 0.3));
    assert!(
        matches!(err, BooleanError::CurvedSectorSideUnsupported { .. }),
        "a sector arm past the wall's radius is not first-order decidable: {err:?}"
    );
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
        let lp = ProfileLoop::new(
            [(0.2, 0.0), (0.6, 0.0), (0.4, 0.6), (0.2, 0.6)]
                .into_iter()
                .map(|(r, y)| ProfileVertex::new(p2(r, y), 0.0))
                .collect(),
        );
        let profile = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .unwrap();
        revolve(
            &profile,
            RevolveAxis {
                origin: p2(0.0, 0.0),
                dir: Vec2::new(0.0, 1.0),
            },
            Revolution::Full,
            tol,
        )
        .unwrap()
        .body
    };
    let err = union_err(&frustum, &boxx(-1.0, 1.0, -0.05, 0.05, 0.25, 0.35));
    assert!(
        matches!(
            err,
            BooleanError::CurvedPairUnsupported {
                kind: geom_brep::SurfaceKind::Cone,
                other_kind: geom_brep::SurfaceKind::Plane,
                ..
            }
        ),
        "the cone's own door, asserted rather than excluded: {err:?}"
    );
}
