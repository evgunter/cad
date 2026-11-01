//! **The tour's rational walls against themselves**
//! (`work/clear/rect-jet-forms-the-rational-derivative-in-the-absolute-frame.md`):
//! `lily_leaf_a`'s lens blade and the Klein loop are lofted and swept
//! over rational section arcs, so the injectivity question's monotone
//! arm reads each wall through a rational net's derivative over a cell
//! (`NurbsSurface::rect_jet`). Each wall against itself holds within the
//! default budget at both certifying scalars, and the same wall moved
//! off the origin reads the same derivative widths and certifies the
//! same cells.

use std::time::Instant;

use demo_tour::klein::bottle;
use demo_tour::lily::{leaf_a, leaf_a_plan};
use demo_tour::scalar::Scalar;
use pncad::geom_core::{Band, Bounds, Interval, Real, RigidMotion, Tol, Vec3};
use pncad::topo::clearance::{
    BoundReport, BoundVerdict, Budgets, ClearanceBound, DEFAULT_MAX_CELL_DEPTH, Side, Sweep, Which,
};
use pncad::topo::{Body, FaceKey, Surface, transform_rigid};

/// The engine's default pair budget.
const BUDGET: usize = 65_536;

fn tol() -> Tol {
    Tol::witness()
}

/// Each spline face of `body`.
fn walls<T: Real>(body: &Body<T>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface).expect("a live surface"),
                Surface::Nurbs(_)
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// The injectivity question on `faces` of `body` alone, at the default
/// depth and `budget` pairs; no candidate is witnessed.
fn injective<T: Scalar>(body: &Body<T>, faces: Vec<FaceKey>, budget: usize) -> BoundReport<()> {
    let side = Side { body, faces };
    let windows = side.windows(Which::First).expect("every face has a window");
    let sweep = Sweep {
        bound: ClearanceBound::StrictlyPositive,
        band: Band::linear(tol()).expect("the run's tolerance forms a band"),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: budget,
        },
    };
    sweep.run(&windows, &windows, true, |_| {
        Err::<(), _>("no witness asked".to_owned())
    })
}

/// Each wall of `body` against itself holds within the default budget;
/// the receipts, for the comparison rows.
fn each_wall_holds<T: Scalar>(what: &str, body: &Body<T>) -> Vec<(FaceKey, BoundReport<()>)> {
    let walls = walls(body);
    assert!(!walls.is_empty(), "{what} has spline walls");
    walls
        .into_iter()
        .map(|wall| {
            let start = Instant::now();
            let report = injective(body, vec![wall], BUDGET);
            println!(
                "[jet frame] {what}: {wall:?} against itself {:?} in {:.2} s, deepest {}, {:?}",
                class(&report.verdict),
                start.elapsed().as_secs_f64(),
                report.widths.deepest,
                report.receipt
            );
            assert!(
                matches!(report.verdict, BoundVerdict::Holds),
                "{what}: {wall:?} against itself holds within {BUDGET} pairs, got {:?} {:?}",
                report.verdict,
                report.receipt
            );
            (wall, report)
        })
        .collect()
}

fn class<W>(verdict: &BoundVerdict<W>) -> &'static str {
    match verdict {
        BoundVerdict::Holds => "Holds",
        BoundVerdict::Violated(_) => "Violated",
        BoundVerdict::Refused(_) => "Refused",
    }
}

#[test]
fn the_lily_blades_walls_hold_against_themselves_at_both_scalars() {
    each_wall_holds("lily_leaf_a", &leaf_a::<f64>(leaf_a_plan(), tol()));
    each_wall_holds(
        "lily_leaf_a at Interval",
        &leaf_a::<Interval>(leaf_a_plan(), tol()),
    );
}

#[test]
fn the_klein_loops_walls_hold_against_themselves_at_both_scalars() {
    each_wall_holds("the Klein loop", bottle::<f64>(tol()).top.body());
    each_wall_holds(
        "the Klein loop at Interval",
        bottle::<Interval>(tol()).top.body(),
    );
}

/// The widest coordinate of each of `rect_jet`'s three partials over each
/// cell of an `8 × 32` grid of `wall`'s chart.
fn grid_widths(body: &Body<f64>, wall: FaceKey) -> Vec<[f64; 3]> {
    let face = body.get_face(wall).expect("a live face");
    let Some(Surface::Nurbs(net)) = body.get_surface(face.surface) else {
        panic!("a blade wall is a spline")
    };
    let net = net.map_scalar(Interval::from_f64);
    let domain = |k: &[Interval]| (k[0].lo(), k[k.len() - 1].hi());
    let (u, v) = (
        domain(net.knot_values_u().values()),
        domain(net.knot_values_v().values()),
    );
    let at = |(a, b): (f64, f64), i: usize, n: usize| {
        #[allow(clippy::cast_precision_loss)]
        let s = |i: usize| a + (b - a) * (i as f64) / (n as f64);
        (s(i), if i + 1 == n { b } else { s(i + 1) })
    };
    (0..8)
        .flat_map(|i| (0..32).map(move |j| (i, j)))
        .map(|(i, j)| {
            let (su, sv, suv) = net
                .rect_jet(at(u, i, 8), at(v, j, 32))
                .expect("a cell inside the domain");
            [su, sv, suv].map(|e| {
                [e.x, e.y, e.z]
                    .iter()
                    .map(|c| c.hi() - c.lo())
                    .fold(0.0, f64::max)
            })
        })
        .collect()
}

/// **The lily blade moved by `(10, 10, 10)` reads its walls alike.** On
/// an `8 × 32` grid of each wall's chart every partial's width matches
/// the unmoved wall's within 1% plus `1e-9`, and each wall against
/// itself holds certifying a cell-pair count within 1% of its unmoved
/// twin's. Read in the absolute frame, the moved blade's walls certify
/// no cell at that grid and run out of budget.
#[test]
fn the_lily_blade_moved_off_the_origin_reads_its_walls_alike() {
    let home = leaf_a::<f64>(leaf_a_plan(), tol());
    let motion = RigidMotion::translation(Vec3::new(10.0, 10.0, 10.0)).expect("a translation");
    let away = transform_rigid(&home, &motion, tol()).expect("the blade moves");
    for wall in walls(&home) {
        let (a, b) = (grid_widths(&home, wall), grid_widths(&away, wall));
        for (cell, (a, b)) in a.iter().zip(&b).enumerate() {
            for k in 0..3 {
                assert!(
                    b[k] <= 1.01 * a[k] + 1e-9 && a[k] <= 1.01 * b[k] + 1e-9,
                    "{wall:?} cell {cell} partial {k}: {} wide moved against {} at home",
                    b[k],
                    a[k]
                );
            }
        }
    }
    let (home, away) = (
        each_wall_holds("lily_leaf_a", &home),
        each_wall_holds("lily_leaf_a moved by (10, 10, 10)", &away),
    );
    for ((wall, a), (_, b)) in home.iter().zip(&away) {
        let (a, b) = (a.receipt.certified, b.receipt.certified);
        #[allow(clippy::cast_precision_loss)]
        let close = (a as f64 - b as f64).abs() <= 0.01 * (a.max(b) as f64);
        assert!(
            close,
            "{wall:?}: certifies {b} cell pairs moved against {a} at home"
        );
    }
}

/// **The whole bodies**, reported: `lily_leaf_a` and the Klein loop, each
/// against itself at the default budget, at both scalars. Their cap
/// corners refuse Departure at depth (`cap_corner_departure` in `lily.rs`
/// and `klein.rs`), so this asserts nothing; it prints the verdict, the
/// pair count and the time.
///
/// `cd demos/tour && cargo nextest run --cargo-profile ci-tour --run-ignored only
/// wall_jets::whole_bodies --no-capture`
#[test]
#[ignore = "a report: the whole bodies' verdicts, pairs and times"]
fn whole_bodies() {
    fn report<T: Scalar>(what: &str, body: &Body<T>) {
        let start = Instant::now();
        let faces = body.faces().map(|(k, _)| k).collect();
        let r = injective(body, faces, BUDGET);
        println!(
            "[jet frame] {what}: {} in {:.1} s, deepest {}, {:?}, {:?}",
            class(&r.verdict),
            start.elapsed().as_secs_f64(),
            r.widths.deepest,
            r.verdict,
            r.receipt
        );
    }
    report("lily_leaf_a", &leaf_a::<f64>(leaf_a_plan(), tol()));
    report(
        "lily_leaf_a at Interval",
        &leaf_a::<Interval>(leaf_a_plan(), tol()),
    );
    report("the Klein loop", bottle::<f64>(tol()).top.body());
    report(
        "the Klein loop at Interval",
        bottle::<Interval>(tol()).top.body(),
    );
}
