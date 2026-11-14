//! **The shell checks its cavity embedded at certifying scalars**
//! (`topo::shell`'s module docs): at `Interval` and `Sym` over it, the
//! moved clone is asked the verdict door's question
//! (`topo::clearance::embedded`) before it is inserted. At `f64` the
//! shell does not ask yet, and the rows that will pin it there wait on
//! `work/embed/the-f64-cavity-gate-refuses-thirty-two-valid-shells.md`.
//!
//! - a vessel holding a coaxial void whose dilation reaches past the
//!   eroded wall refuses `CavityCrosses` at `Interval` and at
//!   `Sym<Interval>`, the witness on the vessel's wall and the void's
//!   cap, on the same pair of operand faces at each;
//! - the tube and the vessel shell Ok at `Interval`, and the vessel at
//!   `Sym<Interval>`, with nothing refused, and the build logs none of
//!   the cavity sweep's cell readings: the engine keeps them out of the
//!   caller's log itself;
//! - the tube at `Sym<Interval>`, and the corpus pot, the cone frustum
//!   and the klein elbow at both certifying scalars, shell Ok;
//! - the planar dumbbell still refuses at the planar gate, at both
//!   scalars;
//! - the planar dumbbell's curved twin refuses in the general offset
//!   door at `Interval`, before the cavity is built;
//! - the box whose cylindrical void dilates past the cavity's planar
//!   wall refuses before the gate is reached, at the planar
//!   `moved_walls_cross`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::time::Instant;
use topo::LeafBody as _;

use geom_core::k_stats::Bracket;
use geom_core::{Decide, Interval, Point2, Point3, Sign, Sym, Tol, Vec2};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::test_support::{brick, finished, prism};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::voids::{VoidContainment, VoidEvidence, insert_void};
use topo::clearance::{
    BoundVerdict, Budgets, CLEARANCE_MARGIN, CLEARANCE_SUPPORT_GAP, ClearanceBound,
    DEFAULT_MAX_CELL_DEPTH, SELF_INTERSECTION_GAP, Side, Sweep, Which,
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
/// extruded 2. With a nonzero `bulge` the two neck walls are arcs, a
/// negative one bowing each into the neck by `|bulge| / 2` at its
/// middle — the curved twin, whose neck is narrowest at `x = 2.5`.
fn dumbbell<T: Decide + topo::AtRestPolicy>(bulge: f64) -> Body<T> {
    let v = |x: f64, y: f64, b: f64| (Point2::new(x, y).map(T::from_f64), T::from_f64(b));
    prism(
        vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 0.0),
            v(2.0, 0.8, bulge),
            v(3.0, 0.8, 0.0),
            v(3.0, 0.0, 0.0),
            v(5.0, 0.0, 0.0),
            v(5.0, 2.0, 0.0),
            v(3.0, 2.0, 0.0),
            v(3.0, 1.2, bulge),
            v(2.0, 1.2, 0.0),
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
    with_void(dst, cavity.into_body())
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
    body.face_surface(face).expect("its surface")
}

fn is_cylinder<T: Decide>(body: &Body<T>, face: FaceKey) -> bool {
    matches!(surface(body, face), geom::Surface::Cylinder { .. })
}

/// The witness of a `CavityCrosses` refusal, or a panic naming what
/// `what` returned instead.
fn crossing<T: topo::AtRestPolicy + Decide>(
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
fn coaxial_crossing<T: topo::AtRestPolicy + Decide>(
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
/// witness** at `Interval`, and the certifying `Sym` rebuild refuses on
/// the same pair of operand faces.
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
}

/// **The planar dumbbell's curved twin** — bulge −0.2 narrows the neck
/// to 0.2 at `x = 2.5`, and two walls of 0.15 need 0.3 — refuses at
/// `Interval` in the general offset door before the cavity is built, an
/// escalation at `offset_corner_on_surface`. It waits on
/// `work/shell/offset-corner-on-surface-escalates-at-interval-on-an-arc-necked-extrude.md`,
/// whose fix turns this row into a `CavityCrosses` row. At ε = 1e-6 it
/// is that row already, since the extrude's caps and walls are its
/// sketch frame: the door reaches the cavity gate, which refuses the
/// neck, typed.
#[test]
fn the_curved_dumbbell_waits_on_the_offset_door_at_interval() {
    let at_1e6 = Tol::witness().eps() == 1e-6;
    match shell(dumbbell::<Interval>(-0.2), 0.15) {
        Err(ShellError::CavityCrosses { .. }) if at_1e6 => {}
        Err(ShellError::Face { error, .. })
            if !at_1e6
                && matches!(
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

/// How many of `recorded`'s verdicts and escalations are the clearance
/// engine's cell readings: its three distance compares and its cells'
/// region readings.
fn cell_readings(recorded: &geom_core::k_stats::Recorded) -> usize {
    let engine = |p: &str| {
        [
            SELF_INTERSECTION_GAP,
            CLEARANCE_SUPPORT_GAP,
            CLEARANCE_MARGIN,
        ]
        .contains(&p)
            || p.starts_with("cell_region_")
    };
    let verdicts = recorded.verdicts.iter().filter(|v| engine(v.predicate));
    let escalations = recorded
        .escalations
        .iter()
        .filter(|e| engine(e.predicate()));
    verdicts.count() + escalations.count()
}

/// `operand` shells Ok at `Interval`, and its cavity — the result's
/// inner wall, the moved clone grafted verbatim — certifies embedded,
/// its receipt and time printed.
///
/// **A shell at `Interval` logs no cell reading**: the cavity sweep
/// decides cells at every level, and the engine takes them out of the
/// caller's log itself (audit row F41), so the build's log holds none.
/// Measured with that scope dropped, the tube's shell logs 1,027 of
/// them and the vessel's 588.
fn holds(what: &str, operand: Body<Interval>, t: f64) {
    let bracket = Bracket::open();
    let start = Instant::now();
    let shelled = shell(operand, t).unwrap_or_else(|e| panic!("{what} shells: {e:?}"));
    let built = start.elapsed();
    let logged = cell_readings(&bracket.finish());
    assert_eq!(
        logged, 0,
        "{what}: the shell's log holds the cavity sweep's cell readings"
    );

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
        "[shell4] {what}: shell {built:?}, cavity sweep {swept:?}, {:?}",
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
        .into_body()
}

/// `what` shells Ok at `T`, the time printed.
fn shells<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>(
    what: &str,
    body: Body<T>,
    t: f64,
) {
    let start = Instant::now();
    let r = shell(body, t);
    let elapsed = start.elapsed();
    let scalar = core::any::type_name::<T>();
    assert!(r.is_ok(), "{what} shells at {scalar}: {:?}", r.err());
    println!("[shell4] {what} at {scalar}: {elapsed:?}");
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

/// **The one-face torus cut** (`self_intersection_proves_injectivity`'s
/// `one_face_torus_cut`): the torus `(2, 0.5)` about `y`, its profile one
/// circle from angle 0.3, less the half-space `x < 0.3`.
fn one_face_torus_cut() -> topo::AtRestBody<Interval> {
    use core::f64::consts::{FRAC_PI_2, TAU};
    use geom_core::{Affine3, Vec3};
    let at = <Interval as geom_core::Real>::from_f64;
    let phase = 0.3f64;
    let lp: profile::ProfileLoop<Interval> = profile::RawLoop::new([(
        Point2::new(2.0 + 0.5 * phase.cos(), 0.5 * phase.sin()).map(at),
        profile::Segment::Arc(geom_core::Arc2 {
            centre: Point2::new(2.0, 0.0).map(at),
            radius: at(0.5),
            sweep: at(TAU),
        }),
    )]);
    let profile = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(tol())
        .expect("the circle validates");
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0).map(at),
        dir: Vec2::new(0.0, 1.0).map(at),
    };
    let ring = revolve(&profile, axis, Revolution::Full, tol())
        .expect("the circle revolves")
        .body;
    let ring = finished("the one-face torus", ring, tol());
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), FRAC_PI_2);
    let rigid = geom_core::test_support::rigid(&turn.map(at), tol());
    let slab = finished(
        "the slab",
        brick::<Interval>((-50.0, 50.0), (-50.0, 50.0), (-100.0, 0.3), tol()),
        tol(),
    );
    let slab = finished(
        "the turned slab",
        topo::transform_rigid(&slab, &rigid).expect("a rigid turn"),
        tol(),
    );
    match topo::subtract(&ring, &slab, tol()) {
        Ok(topo::BooleanResult::Body(b)) => b.body,
        other => panic!("the cut builds: {other:?}"),
    }
}

/// **A cavity the engine cannot certify refuses `CavityUncertified`**,
/// never Ok: the one-face torus cut's cavity keeps its two torus faces
/// meeting along the full meridian, and the departure certificate does
/// not discharge that pair, the refusal the operand itself has at both
/// scalars. The row waits on
/// `work/embed/one-face-torus-cut-refuses-across-its-meridian.md`; when
/// that row is fixed this cavity certifies and the row moves to a body
/// the engine still refuses.
#[test]
fn a_cavity_the_engine_cannot_certify_refuses_uncertified() {
    match topo::shell(
        &one_face_torus_cut(),
        Interval::from_bounds(0.1, 0.1),
        tol(),
    ) {
        Err(ShellError::CavityUncertified {
            refusal:
                topo::clearance::EngineRefusal::Uncertified {
                    arm: topo::clearance::CertificateArm::Departure,
                    ..
                },
        }) => {}
        other => panic!(
            "the one-face torus cut's cavity refuses uncertified at Interval: {:?}",
            other.map(|_| ())
        ),
    }
}
