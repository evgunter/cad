//! **VERBS-1031B — the arc-bounded winding arm**, measured at the
//! teapot cup.
//!
//! The cup is the live consumer of `merge_coplanar_faces`' coplanar
//! pair: `shell_open` on the teapot's own stepped meridian leaves three
//! full-valence coplanar pairs per side (the shoulders and their cavity
//! twins, each seam two disjoint collinear Line segments with all four
//! endpoints at valence 4) plus two pole-split base caps. The merge's
//! surgery completes on all of them; what refused was the ROLE pass,
//! because the merged annulus is bounded by circles and the winding
//! functional was line-bounded only.
//!
//! Both rows here are measurements of doors, not of a shape: one names
//! what the merge does, one names what the boolean gate says about the
//! unmerged operand. They move independently and are meant to.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Open, Profile, ProfileLoop, SketchPlane, Start};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey, Surface};

const FIT_TOL: f64 = 1e-6;
const TOP: f64 = 8.0 / 64.0;

fn revolved(lp: ProfileLoop<f64>, tol: Tol) -> Body<f64> {
    revolve(
        &Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .expect("the meridian validates"),
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the meridian fully revolves")
    .body
}

/// The teapot's own vessel meridian, transcribed from
/// `demos/tour/tests/verbs_teapot.rs::teapot_pot`.
fn teapot_pot(tol: Tol) -> Body<f64> {
    revolved(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(3.0 / 64.0, 0.0), tol)
            .expect("base")
            .line_to(Point2::new(3.0 / 64.0, 1.0 / 64.0), tol)
            .expect("foot")
            .line_to(Point2::new(5.0 / 64.0, 1.0 / 64.0), tol)
            .expect("lower shoulder")
            .line_to(Point2::new(5.0 / 64.0, 6.0 / 64.0), tol)
            .expect("belly")
            .line_to(Point2::new(3.0 / 64.0, 6.0 / 64.0), tol)
            .expect("upper shoulder")
            .line_to(Point2::new(3.0 / 64.0, TOP), tol)
            .expect("neck")
            .line_to(Point2::new(0.0, TOP), tol)
            .expect("mouth")
            .line_to(Start, tol)
            .expect("axis")
            .into(),
        tol,
    )
}

fn plane_chart_at(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

/// The cup: the teapot pot, opened at its mouth chart.
fn teapot_cup(tol: Tol) -> Body<f64> {
    let body = teapot_pot(tol);
    let chart = plane_chart_at(&body, TOP);
    assert_eq!(chart.len(), 2, "a full revolve's cap is two half-discs");
    topo::shell_open(&body, 1.0 / 128.0, &chart, FIT_TOL, tol).expect("the cup opens")
}

/// A cutter box: `x in [0.02, 0.2]`, `y in [-0.01, 0.1]`, `z in [0, 0.3]`.
fn cutter(tol: Tol) -> Body<f64> {
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.02, -0.01))
        .line_to(Point2::new(0.2, -0.01), tol)
        .expect("south")
        .line_to(Point2::new(0.2, 0.1), tol)
        .expect("east")
        .line_to(Point2::new(0.02, 0.1), tol)
        .expect("north")
        .line_to(Start, tol)
        .expect("west")
        .into();
    sweep::extrude(
        &Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .expect("a rectangle is a valid profile"),
        sweep::Extrusion::Distance(0.3),
        tol,
    )
    .expect("a rectangle extrudes")
    .body
}

/// **The differential**: the UNMERGED cup is a non-maximal operand and
/// the boolean gate says so, at `gate_maximal_faces`' same-surface-key
/// planar branch. This row does not move with the winding arm — it is
/// the statement that the cup's coplanar pairs are real, read by a
/// second door that never consults `loop_winding`.
#[test]
fn the_unmerged_cup_is_a_non_maximal_operand() {
    let tol = Tol::witness();
    let cup = teapot_cup(tol);
    assert!(
        matches!(
            topo::boolean::subtract(&cup, &cutter(tol), tol),
            Err(topo::BooleanError::NonMaximalFaces {
                operand: topo::Operand::A,
                ..
            })
        ),
        "the unmerged cup's own coplanar pairs are what F7 refuses"
    );
}

/// The radii of every circular carrier one loop rides, sorted — the
/// role decision made observable from outside: on a merged latitude
/// annulus the OUTER loop must ride the larger circle and the ring the
/// smaller, and that assignment is exactly what a positively-wound
/// outline buys.
fn loop_radii(body: &Body<f64>, l: topo::LoopKey) -> Vec<f64> {
    let topo::LoopBoundary::Cycle { first } = body.get_loop(l).expect("live loop").boundary else {
        return vec![];
    };
    let mut out: Vec<f64> = body
        .loop_cycle(first)
        .expect("a cycle walks")
        .iter()
        .filter_map(|&he| {
            let hd = body.get_half_edge(he)?;
            let e = body.get_edge(hd.edge)?;
            match body.get_curve_geom(e.curve)?.certified()?.carrier() {
                geom::Curve3::Circle { radius, .. } => Some(*radius),
                _ => None,
            }
        })
        .collect();
    out.sort_by(|a, b| a.partial_cmp(b).expect("finite radii"));
    out
}

/// The merge, run on a cup and reported as the facts the acceptance
/// pins: the census delta, the group shapes, and the annulus roles.
struct Merged {
    census: ((usize, usize, usize), (usize, usize, usize)),
    /// Groups that minted a RING — the full-valence pairs, whose seam
    /// is two disjoint collinear segments and whose merged survivor is
    /// a genuine annulus.
    annuli: usize,
    /// Groups that killed a VERTEX — the pole-split caps, half-A's
    /// valence-2 machinery, reachable only once the role pass stops
    /// refusing the whole call.
    pole_caps: usize,
    /// Pairs left to the period-closure refusal: a curved run that
    /// would close its chart's full period is not a coplanar merge.
    period_closures: usize,
    /// `(outer radii, ring radii)` for every survivor carrying a ring.
    annulus_roles: Vec<(Vec<f64>, Vec<f64>)>,
}

fn merge_the_cup(mut cup: Body<f64>, tol: Tol) -> (Body<f64>, Merged) {
    let census = |b: &Body<f64>| (b.faces().count(), b.vertices().count(), b.edges().count());
    let before = census(&cup);
    let out = cup
        .merge_coplanar_faces(tol)
        .unwrap_or_else(|e| panic!("the cup's coplanar pairs must merge, got {e:?}"));
    let annulus_roles = out
        .groups
        .iter()
        .filter(|g| !g.rings_made.is_empty())
        .map(|g| {
            let f = cup.get_face(g.kept).expect("the survivor is live");
            (
                loop_radii(&cup, f.outer),
                f.rings.iter().map(|&r| loop_radii(&cup, r)).fold(
                    vec![],
                    |mut acc, mut r| {
                        acc.append(&mut r);
                        acc
                    },
                ),
            )
        })
        .collect();
    let m = Merged {
        census: (before, census(&cup)),
        annuli: out.groups.iter().filter(|g| !g.rings_made.is_empty()).count(),
        pole_caps: out
            .groups
            .iter()
            .filter(|g| !g.killed_vertices.is_empty())
            .count(),
        period_closures: out
            .skipped
            .iter()
            .filter(|s| matches!(s.reason, topo::MergeCoplanarError::PeriodClosure { .. }))
            .count(),
        annulus_roles,
    };
    (cup, m)
}

/// **The acceptance: the cup merges.** Every coplanar pair the cup owns
/// closes — four full-valence latitude annuli (two per side: the
/// shoulder and its cavity twin) and the two pole-split base caps —
/// and the body that comes out is tier-3 valid.
///
/// The four annuli are what this unit bought: their survivors' outline
/// and ring are both circles, so the role pass had nothing to read
/// until the winding functional learned arcs. The two base caps are
/// half-A's machinery, and they were never the defect — they were
/// unreachable because the first refusal aborted the whole call.
///
/// The six period-closure skips are not failures and never were: a
/// curved run closing its chart's full period is a seam the merge
/// declines by design.
#[test]
fn the_cup_merges_and_its_annuli_take_their_roles() {
    let tol = Tol::witness();
    let (cup, m) = merge_the_cup(teapot_cup(tol), tol);
    assert_eq!(
        m.census,
        ((25, 26, 48), (19, 24, 36)),
        "six faces absorbed, two poles killed, twelve edges eaten"
    );
    assert_eq!(
        (m.annuli, m.pole_caps, m.period_closures),
        (4, 2, 6),
        "four annuli, two pole caps, six period-closure skips"
    );
    assert_eq!(
        topo::validate_geometric(&cup, tol),
        Ok(()),
        "tier 3 on the merged cup"
    );
    for (outer, ring) in &m.annulus_roles {
        assert_eq!(
            (outer.len(), ring.len()),
            (2, 2),
            "each annulus is two circles outside and two inside"
        );
        assert!(
            outer[0] > ring[1],
            "the OUTER loop rides the larger circle: outer {outer:?}, ring {ring:?}"
        );
    }
}

/// **The re-posed twin.** The same cup under a rigid transform off every
/// axis plane merges identically — the winding arm reads the loop's own
/// geometry against the face's own normal, so no axis is special to it.
#[test]
fn the_re_posed_cup_merges_identically() {
    let tol = Tol::witness();
    let turned = topo::transform_rigid(
        &teapot_cup(tol),
        &Affine3::rotation_about_axis(
            Point3::new(0.013, -0.007, 0.021),
            Vec3::new(1.0, 2.0, 3.0),
            0.7,
        ),
        tol,
    )
    .expect("a rigid pose is a rigid pose");
    let posed = topo::transform_rigid(
        &turned,
        &Affine3::translation(Vec3::new(0.31, -0.17, 0.23)),
        tol,
    )
    .expect("and so is a translation");
    let (cup, m) = merge_the_cup(posed, tol);
    assert_eq!(m.census, ((25, 26, 48), (19, 24, 36)), "the same census");
    assert_eq!(
        (m.annuli, m.pole_caps, m.period_closures),
        (4, 2, 6),
        "the same groups"
    );
    assert_eq!(
        topo::validate_geometric(&cup, tol),
        Ok(()),
        "tier 3 on the re-posed merged cup"
    );
    for (outer, ring) in &m.annulus_roles {
        assert!(
            outer[0] > ring[1],
            "the same roles: outer {outer:?}, ring {ring:?}"
        );
    }
}

/// **The boolean after the merge, MEASURED.** The merge was the
/// precondition F7 was asking for, and with it satisfied the subtract
/// walks past that gate and stops at the next door on the road:
/// `CurvedPierceUnsupported`, the shared curved-pierce substrate. That
/// is this row's whole content — it records where the cup's boolean
/// actually stands, and the boundary it names belongs to the pierce
/// lane, not to the coplanar pair this unit repaired.
#[test]
fn the_boolean_after_the_merge_reaches_the_curved_pierce_door() {
    let tol = Tol::witness();
    let (cup, _) = merge_the_cup(teapot_cup(tol), tol);
    let out = topo::boolean::subtract(&cup, &cutter(tol), tol);
    assert!(
        matches!(out, Err(topo::BooleanError::CurvedPierceUnsupported { .. })),
        "the merged cup clears F7 and stops at the curved-pierce substrate, got {:?}",
        out.map(|_| "Ok")
    );
}
