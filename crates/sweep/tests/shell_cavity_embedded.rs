//! **The shell checks its cavity embedded at certifying scalars**
//! (`topo::shell`'s module docs): where the scalar holds the clearance
//! engine's door, the moved clone is asked E7's self-intersection
//! question before it is inserted.
//!
//! - a vessel holding a coaxial void whose dilation reaches past the
//!   eroded wall refuses `CavityCrosses` at `Interval` and at
//!   `Sym<Interval>`, the witness on the vessel's wall and the void's
//!   cap, and shells Ok at `f64`, where no lane answers and the curved
//!   window stays open;
//! - the tube and the vessel shell Ok at `Interval`, and the vessel at
//!   `Sym<Interval>`, with the self-intersection site decided during
//!   the build and nothing refused, so a gate that stopped running goes
//!   red here too;
//! - the tube at `Sym<Interval>`, and the corpus pot, the cone frustum
//!   and the klein elbow at both certifying scalars, shell Ok with the
//!   gate run;
//! - the planar dumbbell still refuses at the planar gate, at both
//!   scalars;
//! - two rows refuse before the gate is reached, each typed: the
//!   planar dumbbell's curved twin, which the general offset door
//!   cannot build at `Interval`, and the box whose cylindrical void
//!   dilates past the cavity's planar wall, which the planar
//!   `moved_walls_cross` reads first.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::time::Instant;

use geom_core::k_stats::Bracket;
use geom_core::{Decide, Interval, Point2, Point3, Sign, Sym, Tol, Vec2};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::test_support::{brick, finished, prism};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::voids::{VoidContainment, VoidEvidence, insert_void};
use topo::clearance::{
    BoundVerdict, Budgets, ClearanceBound, DEFAULT_MAX_CELL_DEPTH, SELF_INTERSECTION_GAP, Side,
    Sweep, Which,
};
use topo::{Body, FaceKey, ShellError, Shelled};

use crate::common::shell_operands::{tube_at, vessel_at};
use sweep::test_support::revolved_about_y_at;

fn tol() -> Tol {
    Tol::witness()
}

fn shell<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>(
    body: Body<T>,
    t: f64,
) -> Result<Shelled<T>, ShellError<T>> {
    topo::shell(&finished("the operand", body, tol()), T::from_f64(t), tol())
}

/// **The planar dumbbell** (`offd2_r1_probes`): two 2×2 blobs joined
/// along `x ∈ [2, 3]` by a neck between `y = 0.8` and `y = 1.2`,
/// extruded 2. With a nonzero `bulge` the two neck walls are arcs that
/// bow into the neck, each by `bulge / 2` at its middle — the curved
/// twin, whose neck is narrowest at `x = 2.5`.
fn dumbbell<T: Decide + topo::AtRestPolicy>(bulge: f64) -> Body<T> {
    let v = |x: f64, y: f64, b: f64| (Point2::new(x, y).map(T::from_f64), T::from_f64(b));
    prism(
        vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 0.0),
            v(2.0, 0.8, 0.0),
            v(3.0, 0.8, bulge),
            v(3.0, 0.0, 0.0),
            v(5.0, 0.0, 0.0),
            v(5.0, 2.0, 0.0),
            v(3.0, 2.0, 0.0),
            v(3.0, 1.2, 0.0),
            v(2.0, 1.2, bulge),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ],
        T::from_f64(2.0),
        tol(),
    )
}

/// **The box with a cylindrical void** (the curved-residue row's
/// measured instance): a 4×4×4 box holding a void of radius 0.65 whose
/// axis is `(x, z) = (3.2, 2)` along `y`, for `y ∈ [1, 3]`. At
/// `t = 0.1` the dilated void reaches `x = 3.95`, past the cavity's
/// planar wall at `x = 3.9`.
fn box_with_void<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    let dst: Body<T> = brick((0.0, 4.0), (0.0, 4.0), (0.0, 4.0), tol());
    let lp = bulge_loop(
        [(3.2, 1.0), (3.85, 1.0), (3.85, 3.0), (3.2, 3.0)]
            .iter()
            .map(|&(x, y)| (Point2::new(x, y).map(T::from_f64), T::zero()))
            .collect(),
    );
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, 2.0).map(T::from_f64));
    let profile = Profile::new(plane, vec![lp])
        .validate(tol())
        .expect("the meridian is a valid profile");
    let axis = RevolveAxis {
        origin: Point2::new(3.2, 0.0).map(T::from_f64),
        dir: Vec2::new(0.0, 1.0).map(T::from_f64),
    };
    let cavity = revolve(&profile, axis, Revolution::Full, tol())
        .expect("the meridian revolves")
        .body;
    with_void(dst, cavity)
}

/// `cavity` inserted into `dst`'s solid as a void, with the carried
/// evidence the shell verb itself hands the door: the fixture puts it
/// strictly inside.
fn with_void<T: Decide>(mut dst: Body<T>, cavity: Body<T>) -> Body<T> {
    let (solid, _) = dst.solids().next().expect("one solid");
    let evidence = VoidEvidence {
        shells: cavity
            .shells()
            .map(|(k, _)| {
                (
                    k,
                    VoidContainment::Carried {
                        sign: Sign::Positive,
                    },
                )
            })
            .collect(),
    };
    insert_void(&mut dst, solid, cavity, &evidence).expect("the void is inside the box");
    dst
}

fn surface<T: Decide>(body: &Body<T>, face: FaceKey) -> &geom::Surface<T> {
    body.get_surface(body.get_face(face).expect("a face of the operand").surface)
        .expect("its surface")
}

fn is_cylinder<T: Decide>(body: &Body<T>, face: FaceKey) -> bool {
    matches!(surface(body, face), geom::Surface::Cylinder { .. })
}

/// The witness of a `CavityCrosses` refusal, or a panic naming what
/// `what` returned instead.
fn crossing<T: Decide>(
    what: &str,
    r: Result<Shelled<T>, ShellError<T>>,
) -> (FaceKey, FaceKey, [(f64, f64); 2], f64) {
    match r {
        Err(ShellError::CavityCrosses {
            face,
            other,
            stations,
            distance,
        }) => (face, other, stations, distance),
        Err(e) => panic!("{what}: expected CavityCrosses, got {e:?}"),
        Ok(_) => panic!("{what}: expected CavityCrosses, the cavity was inserted"),
    }
}

/// **A vessel holding a coaxial void**: the vessel of radius 1 and
/// height 2, and inside it a void of radius 0.95 for `y ∈ [0.5, 1.5]`.
/// At `t = 0.1` the dilated void reaches radius 1.05 and the eroded
/// wall stands at 0.9, so the void's dilated caps run through the
/// cavity's wall. Every planar pair is parallel and far apart, so
/// neither planar gate reads it.
fn vessel_with_void<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    let v = |x: f64, y: f64| (Point2::new(x, y).map(T::from_f64), T::zero());
    with_void(
        vessel_at(1.0, 2.0),
        revolved_about_y_at(
            vec![v(0.0, 0.5), v(0.95, 0.5), v(0.95, 1.5), v(0.0, 1.5)],
            Revolution::Full,
            tol(),
        ),
    )
}

/// The coaxial void's crossing: the witness on the vessel's cylinder
/// wall and one of the void's planar caps.
fn coaxial_crossing<T: Decide>(
    what: &str,
    operand: &Body<T>,
    r: Result<Shelled<T>, ShellError<T>>,
) -> (FaceKey, FaceKey) {
    let (face, other, _, distance) = crossing(what, r);
    let plane = |f: FaceKey| matches!(surface(operand, f), geom::Surface::Plane { .. });
    assert!(
        (is_cylinder(operand, face) && plane(other))
            || (is_cylinder(operand, other) && plane(face)),
        "{what}: the witness is on a wall and a cap: {face:?} {other:?}"
    );
    assert!(
        distance <= 1e-6,
        "{what}: the witness pair coincides: {distance}"
    );
    (face, other)
}

/// **A void dilated through a curved cavity wall refuses with a
/// witness** at `Interval`, and the certifying `Sym` rebuild refuses
/// on the same pair of operand faces. At `f64` no lane answers and the
/// same call shells Ok: the curved window the module docs state.
#[test]
fn a_void_dilated_through_the_curved_wall_refuses_with_a_witness() {
    let operand: Body<Interval> = vessel_with_void();
    let at_interval = coaxial_crossing("at Interval", &operand, shell(operand.clone(), 0.1));
    let sym: Body<Sym<Interval>> = vessel_with_void();
    let at_sym = coaxial_crossing("at Sym<Interval>", &sym, shell(sym.clone(), 0.1));
    assert_eq!(
        at_sym, at_interval,
        "the Sym rebuild's witness is on the Interval rebuild's faces"
    );
    let at_f64 = shell(vessel_with_void::<f64>(), 0.1);
    assert!(
        at_f64.is_ok(),
        "at f64 no lane answers, so the curved window is open: {:?}",
        at_f64.err()
    );
}

/// **The planar dumbbell's curved twin** — bulge 0.2 narrows the neck
/// to 0.2 at `x = 2.5`, and two walls of 0.15 need 0.3 — shells Ok at
/// `f64`, the curved window, and at `Interval` refuses in the general
/// offset door before the cavity is built, an escalation at
/// `offset_corner_on_surface`. It waits on
/// `work/offset/offset-corner-on-surface-escalates-at-interval-on-an-arc-necked-extrude.md`,
/// whose fix turns this row's second half into a `CavityCrosses` row.
#[test]
fn the_curved_dumbbell_is_the_window_at_f64_and_waits_on_the_offset_door_at_interval() {
    let at_f64 = shell(dumbbell::<f64>(0.2), 0.15);
    assert!(
        at_f64.is_ok(),
        "at f64 no lane answers, so the curved window is open: {:?}",
        at_f64.err()
    );
    match shell(dumbbell::<Interval>(0.2), 0.15) {
        Err(ShellError::Face { error, .. })
            if matches!(
                &*error,
                topo::ReplaceFaceError::Escalated { source }
                    if source.predicate == Some("offset_corner_on_surface")
            ) => {}
        other => panic!(
            "the general offset door refuses the curved twin at Interval, typed; got {:?}",
            other.map(|_| ())
        ),
    }
}

/// **The box with a cylindrical void refuses at both scalars before
/// the cavity gate**: its dilated void crosses the cavity's planar
/// `x = 3.9` wall at an angle the planar `moved_walls_cross` reads, so
/// `f64` refuses `OffsetsCross`, and at `Interval` that read's overlap
/// encloses zero and escalates at `shell_moved_walls_overlap`.
#[test]
fn the_box_with_a_cylindrical_void_refuses_at_the_moved_walls_read() {
    let at_f64 = shell(box_with_void::<f64>(), 0.1);
    assert!(
        matches!(at_f64, Err(ShellError::OffsetsCross { .. })),
        "at f64: {:?}",
        at_f64.err()
    );
    match shell(box_with_void::<Interval>(), 0.1) {
        Err(ShellError::Escalated { source })
            if source.predicate == Some("shell_moved_walls_overlap") => {}
        other => panic!("at Interval: {:?}", other.map(|_| ())),
    }
}

/// **The planar dumbbell still refuses at the planar gate**, which
/// runs first at every scalar: its neck walls face each other across
/// 0.4 and two walls of 0.3 need 0.6.
#[test]
fn the_planar_dumbbell_refuses_at_the_planar_gate() {
    let interval = shell(dumbbell::<Interval>(0.0), 0.3);
    assert!(
        matches!(interval, Err(ShellError::WallClearance { .. })),
        "at Interval: {:?}",
        interval.err()
    );
    let float = shell(dumbbell::<f64>(0.0), 0.3);
    assert!(
        matches!(float, Err(ShellError::WallClearance { .. })),
        "at f64: {:?}",
        float.err()
    );
}

/// `operand` shells Ok at `Interval` with the self-intersection site
/// decided during the build, and its cavity — the result's inner wall,
/// the moved clone grafted verbatim — certifies embedded, its receipt
/// and time printed.
fn holds(what: &str, operand: Body<Interval>, t: f64) {
    let bracket = Bracket::open();
    let start = Instant::now();
    let shelled = shell(operand, t).unwrap_or_else(|e| panic!("{what} shells: {e:?}"));
    let built = start.elapsed();
    let verdicts = bracket.finish().verdicts;
    let gap = verdicts
        .iter()
        .filter(|v| v.predicate == SELF_INTERSECTION_GAP)
        .count();
    assert!(gap > 0, "{what}: the cavity gate decided no separation");

    let faces: Vec<FaceKey> = shelled.naming.inner.iter().map(|&(twin, _)| twin).collect();
    let side = Side {
        body: &shelled.body,
        faces,
    };
    let windows = side
        .windows(Which::First)
        .expect("every cavity face has a window");
    let sweep = Sweep {
        bound: ClearanceBound::StrictlyPositive,
        band: geom_core::Band::linear(tol()).unwrap(),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: 65_536,
        },
    };
    let start = Instant::now();
    let report = sweep.run(&windows, &windows, true, |_| {
        Err::<(), _>("no witness is sought".to_owned())
    });
    let swept = start.elapsed();
    assert!(
        matches!(report.verdict, BoundVerdict::Holds),
        "{what}: the cavity certifies embedded: {:?}",
        report.verdict
    );
    assert!(report.receipt.holds(), "{what}: the receipt adds up");
    assert_eq!(report.receipt.refused, 0, "{what}: nothing refused");
    println!(
        "[shell4] {what}: shell {built:?}, cavity sweep {swept:?}, {gap} gap decides, {:?}",
        report.receipt
    );
}

#[test]
fn the_tube_holds() {
    holds("the tube", tube_at(0.4, 0.8, 0.6), 0.1);
}

#[test]
fn the_vessel_holds() {
    holds("the vessel", vessel_at(1.0, 2.0), 0.1);
    let at_sym = shell(vessel_at::<Sym<Interval>>(1.0, 2.0), 0.1);
    assert!(
        at_sym.is_ok(),
        "the Sym rebuild shells the vessel: {:?}",
        at_sym.err()
    );
}

/// **The corpus pot**: a revolved meridian of a foot, a spherical belly
/// and a mouth (`editor-core`'s `corpus/vessel.rs`, sealed here).
fn corpus_pot<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    let p = |x: f64, y: f64| Point2::new(x, y).map(T::from_f64);
    let (foot, mouth) = (p(4.0 / 64.0, 1.0 / 64.0), p(3.0 / 64.0, 8.0 / 64.0));
    let belly = profile::bulge_from_center(foot, mouth, p(0.0, 4.0 / 64.0), profile::ArcSweep::Ccw);
    revolved_about_y_at(
        vec![
            (p(0.0, 0.0), T::zero()),
            (p(4.0 / 64.0, 0.0), T::zero()),
            (foot, belly),
            (mouth, T::zero()),
            (p(0.0, 8.0 / 64.0), T::zero()),
        ],
        Revolution::Full,
        tol(),
    )
}

/// **The cone frustum** (`sf2b_interval_probe`'s): base radius `4/64`,
/// top `2/64`, height `8/64`.
fn cone_frustum<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    let v = |x: f64, y: f64| (Point2::new(x / 64.0, y / 64.0).map(T::from_f64), T::zero());
    revolved_about_y_at(
        vec![v(0.0, 0.0), v(4.0, 0.0), v(2.0, 8.0), v(0.0, 8.0)],
        Revolution::Full,
        tol(),
    )
}

/// **The klein elbow** (`torax_interval`'s): a disc of radius 0.275
/// revolved a quarter turn about an axis 1.2 off its centre.
fn klein_elbow<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    let p = |x: f64, y: f64| Point2::new(x, y).map(T::from_f64);
    let profile = Profile::new(
        SketchPlane::<T>::xy(),
        vec![bulge_loop(vec![
            (p(-0.275, 0.0), T::one()),
            (p(0.275, 0.0), T::one()),
        ])],
    )
    .validate(tol())
    .expect("the cross-section validates");
    let axis = RevolveAxis {
        origin: p(1.2, 0.0),
        dir: Vec2::new(0.0, -1.0).map(T::from_f64),
    };
    let quarter = Revolution::Partial(T::from_f64(-core::f64::consts::FRAC_PI_2));
    revolve(&profile, axis, quarter, tol())
        .expect("the elbow revolves")
        .body
}

/// `what` shells Ok at `T` with the self-intersection site decided
/// during the build, the time printed.
fn shells<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>(
    what: &str,
    body: Body<T>,
    t: f64,
) {
    let bracket = Bracket::open();
    let start = Instant::now();
    let r = shell(body, t);
    let elapsed = start.elapsed();
    let gap = bracket
        .finish()
        .verdicts
        .iter()
        .filter(|v| v.predicate == SELF_INTERSECTION_GAP)
        .count();
    let scalar = core::any::type_name::<T>();
    assert!(r.is_ok(), "{what} shells at {scalar}: {:?}", r.err());
    assert!(
        gap > 0,
        "{what} at {scalar}: the cavity gate decided no separation"
    );
    println!("[shell4] {what} at {scalar}: {elapsed:?}, {gap} gap decides");
}

/// **Valid bodies shell at both certifying scalars**: the tube, and the
/// three whose cavities the engine once could not certify — the corpus
/// pot's sphere meeting its cylinder at a vertex, the cone frustum's
/// window, the klein elbow's spiric caps — at `Interval` and at the
/// `Sym` rebuild.
#[test]
fn valid_bodies_shell_at_both_certifying_scalars() {
    shells("the tube", tube_at::<Sym<Interval>>(0.4, 0.8, 0.6), 0.1);
    shells("the corpus pot", corpus_pot::<Interval>(), 1.0 / 128.0);
    shells("the corpus pot", corpus_pot::<Sym<Interval>>(), 1.0 / 128.0);
    shells("the cone frustum", cone_frustum::<Interval>(), 1.0 / 128.0);
    shells(
        "the cone frustum",
        cone_frustum::<Sym<Interval>>(),
        1.0 / 128.0,
    );
    shells("the klein elbow", klein_elbow::<Interval>(), 0.05);
    shells("the klein elbow", klein_elbow::<Sym<Interval>>(), 0.05);
}
