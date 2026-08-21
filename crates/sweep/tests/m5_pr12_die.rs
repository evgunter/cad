//! **The die** (M5 PR 12 §4, acceptance shape (v)): a filleted cube
//! that then takes its pips.
//!
//! Stage 1 is `fillet_edges` — open chains terminating in
//! sphere-octant corners (pinned in `m5_pr12_die_body.rs`). Stage 2 is
//! S13's live `slab ∖ ball`, now run against a body whose faces are
//! planes AND cylinders AND spheres: each pip is a ball dipped into a
//! SHRUNK planar face, and the removed volume is exactly a spherical
//! cap, so the die's certified volume stays a closed form all the way
//! through.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::PI;
use profile::RawLoop;

use geom_core::{Affine3, Band, Point2, Tolerance, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::fillet::build::fillet_edges;
use sweep::test_support::cube;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations};
use geom_core::Tol;

/// The die's side, meters.
const DIE_L: f64 = 1.0;
/// The blend radius, meters.
const DIE_R: f64 = 0.12;
/// The pip balls' radius, meters.
const PIP_R: f64 = 0.09;
/// How deep each pip ball dips into its face, meters.
const PIP_H: f64 = 0.05;
/// Pip spacing from the face centre, meters.
const PIP_D: f64 = 0.22;

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A radius-`r` ball centred at `c` with its POLAR AXIS along `pole`.
///
/// The axis matters: `revolve` puts the ball's poles on the sketch
/// axis, and a plane×sphere section taken against a chart whose polar
/// axis is TILTED to the plane is a typed frontier of the split-join
/// (`the azimuth-anchored arc-side rule needs a polar section`). A pip
/// is cut by a face plane, so its ball is charted with the pole along
/// that face's normal and the section stays polar by construction.
fn ball_poled(r: f64, c: Vec3<f64>, pole: Vec3<f64>) -> Body<f64> {
    let ball = ball_at(r, Vec3::new(0.0, 0.0, 0.0));
    let y = Vec3::new(0.0, 1.0, 0.0);
    let axis = y.cross(pole);
    let placed = if axis.norm() < 1e-12 {
        if y.dot(pole) > 0.0 {
            ball
        } else {
            topo::transform_rigid(
                &ball,
                &Affine3::rotation_about_axis(
                    geom_core::Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                    PI,
                ),
                Tol::witness(),
            )
            .unwrap()
        }
    } else {
        topo::transform_rigid(
            &ball,
            &Affine3::rotation_about_axis(
                geom_core::Point3::new(0.0, 0.0, 0.0),
                axis.normalize(),
                y.dot(pole).clamp(-1.0, 1.0).acos(),
            ),
            Tol::witness(),
        )
        .unwrap()
    };
    topo::transform_rigid(&placed, &Affine3::translation(c), Tol::witness()).unwrap()
}

fn ball_at(r: f64, c: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -r), 1.0),
        ProfileVertex::new(p2(0.0, r), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full, Tol::witness()).unwrap().body;
    topo::transform_rigid(&ball, &Affine3::translation(c), Tol::witness()).unwrap()
}

/// The blank: the cube with every edge filleted.
fn blank() -> Body<f64> {
    let body = cube(DIE_L, Tol::witness());
    let edges: Vec<_> = body.edges().map(|(k, _)| k).collect();
    fillet_edges(&body, &edges, DIE_R, band(), Tol::witness())
        .expect("the die blank")
        .body
}

/// The 2-D pip layout of face value `n`, in units of `PIP_D` about the
/// face centre — the classical die arrangement.
fn layout(n: u32) -> Vec<(f64, f64)> {
    let c = vec![(0.0, 0.0)];
    let diag = vec![(-1.0, -1.0), (1.0, 1.0)];
    let anti = vec![(-1.0, 1.0), (1.0, -1.0)];
    let sides = vec![(-1.0, 0.0), (1.0, 0.0)];
    match n {
        1 => c,
        2 => diag,
        3 => [diag.clone(), c].concat(),
        4 => [diag.clone(), anti.clone()].concat(),
        5 => [diag.clone(), anti.clone(), c].concat(),
        _ => [diag, anti, sides].concat(),
    }
}

/// Every pip ball of the die, as centres in world space. Face `n`
/// carries `n` pips; opposite faces sum to seven.
fn pip_centres() -> Vec<Vec3<f64>> {
    pip_placements().into_iter().map(|(c, _)| c).collect()
}

/// Every pip ball's centre AND the face normal it is charted against.
fn pip_placements() -> Vec<(Vec3<f64>, Vec3<f64>)> {
    let h = DIE_L / 2.0;
    // (value, outward axis, the two in-face axes)
    // (face value, outward normal, the two in-face axes).
    type Face = (u32, Vec3<f64>, Vec3<f64>, Vec3<f64>);
    let faces: [Face; 6] = [
        (
            1,
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            6,
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            2,
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        (
            5,
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        (
            3,
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            4,
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
    ];
    let mut out = Vec::new();
    for (n, normal, ex, ey) in faces {
        // The ball centre sits `R − H` OUTSIDE the face plane, so the
        // cavity is a cap of height H.
        let base = Vec3::new(h, h, h) + normal * (h + (PIP_R - PIP_H));
        for (u, v) in layout(n) {
            out.push((base + ex * (u * PIP_D) + ey * (v * PIP_D), normal));
        }
    }
    out
}

/// All 21 pip balls as ONE operand: a multi-shell body of disjoint
/// closed spheres. This is the shape S13's group arm certifies —
/// cutting them one at a time instead would present a body that
/// already carries a TRIMMED sphere face as the next operand, which
/// S13 pins as a typed refusal
/// (`trimmed_sphere_group_operand_refuses_typed_at_the_scan`): the
/// extent certificate needs the closed-group discipline and a trimmed
/// patch has no per-face chart-trim extent. So the die's pips are one
/// operation, by construction and not by luck.
fn pip_tool() -> Body<f64> {
    let places = pip_placements();
    let mut tool = ball_poled(PIP_R, places[0].0, places[0].1);
    for (c, n) in &places[1..] {
        tool = boolean_op_with(
            BooleanOp::Union,
            &tool,
            &ball_poled(PIP_R, *c, *n),
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("assembling the pip tool: {e}"))
        .body()
        .expect("a body")
        .body
        .clone();
    }
    tool
}

fn subtract(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let out = boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("pip subtraction: {e}"));
    out.body().expect("a body").body.clone()
}

/// The blank's closed-form volume and area (core + 6 slabs + 12
/// quarter-cylinders + 8 octants, the octants summing to one ball).
fn blank_volume() -> f64 {
    let core = DIE_L - 2.0 * DIE_R;
    core.powi(3)
        + 6.0 * DIE_R * core.powi(2)
        + 12.0 * (PI * DIE_R * DIE_R / 4.0) * core
        + (4.0 / 3.0) * PI * DIE_R.powi(3)
}

/// A spherical cap of height `h` off a radius-`r` ball.
fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

/// **The acceptance row for the die BLANK**: the filleted cube builds,
/// tier-3 certifies, meters its certified volume and area against the
/// closed forms, and tessellates WATERTIGHT.
#[test]
fn the_die_blank_certifies_and_tessellates_watertight() {
    let die = blank();
    assert_eq!(topo::validate(&die), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&die), Ok(()), "tier 2");
    assert_eq!(topo::validate_geometric(&die, Tol::witness()), Ok(()), "tier 3");
    let props = topo::mass_properties(&die, Tol::witness()).unwrap();
    let want = blank_volume();
    assert!(
        (props.volume - want).abs() <= 1e-9 * want,
        "blank volume {} vs closed form {want}",
        props.volume
    );
    assert_eq!(
        props.volume_pad, 0.0,
        "a closed-form body needs no enclosure pad"
    );
    let mesh = mesh::tessellate(&die, 5e-3, Tol::witness()).expect("the blank tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
    let v_mesh = mesh::validate::signed_volume(&mesh);
    assert!(v_mesh > 0.0 && v_mesh < props.volume, "an inscribed mesh");
    assert!(((v_mesh - props.volume) / props.volume).abs() < 5e-3);
}

/// **The pips, cut in ONE certified group operation** — 21 balls on
/// all six faces, each charted with its pole along the face it is cut
/// by, subtracted from the sharp cube as a single 21-shell operand.
///
/// Both facts in that sentence are load-bearing and each was a typed
/// refusal until it was met:
/// - cutting the pips ONE AT A TIME presents a body that already
///   carries a TRIMMED sphere face as the next operand, which S13 pins
///   as a refusal (the extent certificate needs the closed-group
///   discipline; a trimmed patch has no per-face chart-trim extent);
/// - charting a pip ball with its pole NOT along the cutting face's
///   normal makes the plane×sphere section tilted against the chart's
///   polar axis, which the split-join's azimuth-anchored arc-side rule
///   refuses typed.
#[test]
fn the_pips_cut_in_one_group_operation_on_all_six_faces() {
    let tool = pip_tool();
    assert_eq!(
        tool.shells().count(),
        21,
        "21 disjoint closed sphere shells"
    );
    let pipped = subtract(&cube(DIE_L, Tol::witness()), &tool);
    assert_eq!(topo::validate(&pipped), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&pipped), Ok(()), "tier 2");
    assert_eq!(topo::validate_geometric(&pipped, Tol::witness()), Ok(()), "tier 3");
    let want = DIE_L.powi(3) - 21.0 * cap(PIP_R, PIP_H);
    let vol = topo::mass_properties(&pipped, Tol::witness()).unwrap().volume;
    assert!(
        (vol - want).abs() <= 1e-9 * want,
        "pipped cube volume {vol} vs closed form {want}"
    );
    let mesh = mesh::tessellate(&pipped, 5e-3, Tol::witness()).expect("the pipped cube tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

/// **DEVIATION 1, FLIPPED at both doors** (M6 unit 1 — kept, per the
/// S9 pattern, as the record of the two frontiers it used to pin;
/// its M5 name was `deviation_1_the_blank_and_the_pips_do_not_compose_yet`).
///
/// - *Fillet then pip* (door A) refused at the curved pierce door
///   because the conic-carrier arm was UNCONDITIONAL — the reviewer
///   measured the true clearance of the named pair at 1.6 cm and it
///   refused anyway (fix pass F4). The M6 rider gives CIRCLE carriers
///   a definite-miss verdict in closed form
///   (`bool_circle_curved_clearance`, the `circle_span_bounds`
///   harmonic algebra), so every far pair now CLEARS and the ordering
///   marches past the reduce stage entirely — to its REAL frontier:
///   the containment stage's `PartialSphereFace` door (the blank's
///   octants are trimmed sphere faces, and the whole-sphere
///   containment class has no chart-trim extent for them — the M5
///   PR 9c door, reached honestly instead of masked by an
///   unconditional arm). Door A is still typed, one stage deeper.
/// - *Pip then fillet* (door B) composes: the in-place composition
///   surgery blends the twelve box edges in place with every pip rim
///   carried through. The full
///   composed-die ladder (tier 3, closed forms, watertight, rim tori)
///   lives in `m6_surgery.rs`.
#[test]
fn deviation_1_flipped_door_b_composes_door_a_reaches_its_real_frontier() {
    // Door A: fillet then pip — past the pierce door (the rider),
    // refusing typed at the containment stage's named frontier.
    let err = boolean_op_with(
        BooleanOp::Subtract,
        &blank(),
        &pip_tool(),
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .expect_err("trimmed sphere faces have no containment extent yet");
    let text = format!("{err}");
    assert!(
        text.contains("trimmed") && text.contains("sphere"),
        "door A's refusal names the partial-sphere containment door, not the pierce          door the rider retired: {text}"
    );

    // Door B: pip then fillet — the surgery composes.
    let cube0 = cube(DIE_L, Tol::witness());
    let box_edges: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
    let pipped = subtract(&cube0, &pip_tool());
    let surviving: Vec<_> = box_edges
        .into_iter()
        .filter(|k| pipped.get_edge(*k).is_some())
        .collect();
    assert_eq!(surviving.len(), 12, "every box edge survives the pips");
    let via_surgery = fillet_edges(&pipped, &surviving, DIE_R, band(), Tol::witness())
        .expect("the in-place surgery takes the subset request (M6 unit 1)")
        .body;
    assert_eq!(topo::validate_geometric(&via_surgery, Tol::witness()), Ok(()), "tier 3");
    let want = blank_volume() - 21.0 * cap(PIP_R, PIP_H);
    let vb = topo::mass_properties(&via_surgery, Tol::witness()).unwrap().volume;
    assert!(
        (vb - want).abs() <= 1e-9 * want,
        "door B volume {vb} vs closed form {want}"
    );
}
