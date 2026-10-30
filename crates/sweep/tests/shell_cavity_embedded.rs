//! **The shell checks its cavity embedded at certifying scalars**
//! (`topo::shell`'s module docs): where the scalar holds the clearance
//! engine's door, the moved clone is asked E7's self-intersection
//! question before it is inserted.
//!
//! - the planar dumbbell's curved twin — its neck two arcs — refuses
//!   `CavityCrosses` at `Interval` and at `Sym<Interval>` with one
//!   witness on the two neck walls, and shells Ok at `f64`, where no
//!   lane answers and the curved window stays open;
//! - a box whose cylindrical void dilates past the cavity's planar wall
//!   refuses `CavityCrosses` at `Interval`, the pair curved × planar;
//! - the tube and the vessel shell Ok at `Interval`, the self-
//!   intersection site decided during the build, so a gate that stopped
//!   running goes red here too;
//! - the planar dumbbell still refuses at the planar gate, at both
//!   scalars.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::time::Instant;

use geom_core::k_stats::Bracket;
use geom_core::{Decide, Interval, Point2, Point3, Real, Sign, Sym, Tol, Vec2};
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
    let mut dst: Body<T> = brick((0.0, 4.0), (0.0, 4.0), (0.0, 4.0), tol());
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

/// **The curved dumbbell refuses with a witness on its two neck
/// walls**: bulge 0.2 narrows the neck to 0.2 at `x = 2.5`, two walls
/// of 0.15 need 0.3, and every planar pair stands at least 0.4 apart,
/// so the planar gate passes it and the cavity gate is what refuses.
/// At `f64`, where no lane answers, the same call shells Ok: the
/// curved window the module docs state.
#[test]
fn the_curved_dumbbell_refuses_with_a_witness() {
    let operand: Body<Interval> = dumbbell(0.2);
    let (face, other, stations, distance) =
        crossing("the curved dumbbell", shell(operand.clone(), 0.15));
    assert!(
        face != other && is_cylinder(&operand, face) && is_cylinder(&operand, other),
        "the witness is on the two neck arcs: {face:?} and {other:?}"
    );
    for (f, (u, v)) in [(face, stations[0]), (other, stations[1])] {
        let p = surface(&operand, f).eval(Interval::from_f64(u), Interval::from_f64(v));
        let x = geom_core::Bounds::hi(p.x);
        assert!(
            (2.0..=3.0).contains(&x),
            "a witness point stands over the neck, x in [2, 3]: {x} on {f:?}"
        );
    }
    assert!(distance <= 1e-6, "the witness pair coincides: {distance}");
    println!("[shell4] curved dumbbell at Interval: {face:?} {other:?} {stations:?} {distance}");

    let at_f64 = shell(dumbbell::<f64>(0.2), 0.15);
    assert!(
        at_f64.is_ok(),
        "at f64 no lane answers, so the curved window is open: {:?}",
        at_f64.err()
    );
}

/// **The certifying `Sym` rebuild runs the same gate**: the curved
/// dumbbell at `Sym<Interval>` refuses on the same pair of operand
/// faces as at `Interval`.
#[test]
fn the_sym_rebuild_gives_the_same_verdict() {
    let (face, other, ..) = crossing("at Interval", shell(dumbbell::<Interval>(0.2), 0.15));
    let (sym_face, sym_other, ..) = crossing(
        "at Sym<Interval>",
        shell(dumbbell::<Sym<Interval>>(0.2), 0.15),
    );
    assert_eq!(
        (sym_face, sym_other),
        (face, other),
        "the Sym rebuild's witness is on the Interval rebuild's faces"
    );
}

/// **A void dilated through the cavity's planar wall refuses**: the
/// pair is the void's cylinder and the box's `x = 4` wall, which no
/// planar gate reads.
#[test]
fn a_void_dilated_through_the_cavity_wall_refuses() {
    let operand: Body<Interval> = box_with_void();
    let (face, other, ..) = crossing("the box with a void", shell(operand.clone(), 0.1));
    let plane_at_x4 = |f: FaceKey| {
        matches!(
            surface(&operand, f),
            geom::Surface::Plane { origin, normal, .. }
                if (geom_core::Bounds::hi(origin.x) - 4.0).abs() < 1e-9
                    && geom_core::Bounds::hi(normal.x).abs() > 0.5
        )
    };
    assert!(
        (is_cylinder(&operand, face) && plane_at_x4(other))
            || (is_cylinder(&operand, other) && plane_at_x4(face)),
        "the witness is on the void's wall and the box's x = 4 wall: {face:?} {other:?}"
    );
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
}
