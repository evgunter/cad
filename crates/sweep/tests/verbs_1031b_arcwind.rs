//! **VERBS-1031B — the teapot cup's latitude annuli**, measured.
//!
//! The cup was the live consumer of `merge_coplanar_faces`' coplanar
//! pair: `shell_open` on the teapot's stepped meridian left four split
//! latitude annuli (the two shoulders and their cavity twins) and two
//! pole-split base caps, and the merge's ROLE pass had to learn
//! circle-bounded windings to put each annulus's outline in its outer
//! slot. The full revolve now builds every plane wall whole
//! (`crates/sweep/README.md`, "Walls: one per run"), so the cup is born
//! in the merged state: the rows pin that census, the annulus roles on
//! the cup as built, and where its boolean stands.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Open, Profile, ProfileLoop, SketchPlane, Start};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey, Surface};

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
    assert_eq!(chart.len(), 1, "a full revolve builds its cap whole");
    topo::shell_open(&body, 1.0 / 128.0, &chart, tol)
        .expect("the cup opens")
        .body
}

/// A cutter box: `x in [0.02, 0.2]`, `y in [-0.01, 0.1]`, `z in [0, 0.3]`.
fn cutter(tol: Tol) -> Body<f64> {
    sweep::test_support::brick((0.02, 0.2), (-0.01, 0.1), (0.0, 0.3), tol)
}

/// The radii of every circular carrier one loop rides, sorted — the
/// role decision made observable from outside: on a latitude annulus
/// the OUTER loop must ride the larger circle and the ring the smaller.
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

/// `(outer radii, ring radii)` for every face of `body` carrying a ring.
fn annulus_roles(body: &Body<f64>) -> Vec<(Vec<f64>, Vec<f64>)> {
    body.faces()
        .filter(|(_, f)| !f.rings.is_empty())
        .map(|(_, f)| {
            (
                loop_radii(body, f.outer),
                f.rings.iter().flat_map(|&r| loop_radii(body, r)).collect(),
            )
        })
        .collect()
}

/// The census, the four latitude annuli's roles, and tier 3 — what the
/// merge used to produce, which the cup now carries as built.
fn assert_the_cup_as_built(label: &str, cup: &Body<f64>, tol: Tol) {
    assert_eq!(
        (
            cup.faces().count(),
            cup.vertices().count(),
            cup.edges().count()
        ),
        (19, 24, 36),
        "{label}: the census the merge used to reach"
    );
    let roles = annulus_roles(cup);
    // Two shoulders and their cavity twins; the mouth's rim carries
    // the cavity's ring as well.
    assert!(roles.len() >= 4, "{label}: {roles:?}");
    for (outer, ring) in &roles {
        assert!(
            outer.last() > ring.last(),
            "{label}: the OUTER loop rides the larger circle: outer {outer:?}, ring {ring:?}"
        );
    }
    assert_eq!(
        topo::validate_geometric(cup, tol),
        Ok(()),
        "{label}: tier 3"
    );
    let mut merged = cup.clone();
    let out = merged.merge_coplanar_faces(tol).expect("the merge runs");
    assert!(
        out.groups.is_empty(),
        "{label}: no coplanar pair is left to merge: {:?}",
        out.groups
    );
    // The curved runs closing their chart's full period are still the
    // merge's to decline, by design.
    assert_eq!(
        out.skipped
            .iter()
            .filter(|s| matches!(s.reason, topo::MergeCoplanarError::PeriodClosure { .. }))
            .count(),
        6,
        "{label}: six period-closure skips"
    );
}

/// **The cup is maximal as built.** The full revolve builds every plane
/// wall whole — the shoulders as annuli whose inner circle is a ring,
/// the base as one disc — and the shell offsets them as such, so the
/// cup carries the census and the annulus roles `merge_coplanar_faces`
/// used to produce from four split annuli and two pole-split caps, and
/// the merge finds nothing to do.
#[test]
fn the_cup_is_maximal_as_built_and_its_annuli_take_their_roles() {
    let tol = Tol::witness();
    assert_the_cup_as_built("cup", &teapot_cup(tol), tol);
}

/// **The re-posed twin.** The same cup under a rigid transform off every
/// axis plane is the same cup.
#[test]
fn the_re_posed_cup_is_the_same_cup() {
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
    assert_the_cup_as_built("re-posed cup", &posed, tol);
}

/// **The boolean on the cup, MEASURED.** F7 does not answer and the
/// crossing layer passes: the subtract stops at the join,
/// `UnpairedLooseEnds { count: 4 }`. That is this row's whole content —
/// it records where the cup's boolean actually stands, and the boundary
/// it names belongs to the join.
///
/// The crossing layer's door it once stopped at was the cutter's edge
/// `x = 0.02, y = 0.1` (along `z`) against one of the cup's
/// half-cylinder faces: the line straddles the carrier and crosses it
/// once, OUTSIDE that half's window. The straddle arm read the
/// accounted-for crossing as a contradiction; it is now the certified
/// negative (`SpanVerdict::Elsewhere`), and the sibling half records
/// the crossing on its own visit.
#[test]
fn the_boolean_on_the_cup_reaches_the_join() {
    let tol = Tol::witness();
    let out = topo::boolean::subtract(&teapot_cup(tol), &cutter(tol), tol);
    assert!(
        matches!(
            out,
            Err(topo::BooleanError::Join(
                topo::SplitJoinError::UnpairedLooseEnds { count: 4 }
            ))
        ),
        "the cup clears F7 and the crossing layer and stops at the join, got {:?}",
        out.map(|_| "Ok")
    );
}
