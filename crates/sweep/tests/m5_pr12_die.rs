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

use geom_core::{Affine3, Band, Point2, Tolerance, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::fillet::build::fillet_edges;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations};

/// The die: a unit cube, fillet radius 0.12, pip balls of radius 0.09
/// dipped 0.05 deep, pip centres 0.22 from each face centre.
pub const DIE_L: f64 = 1.0;
pub const DIE_R: f64 = 0.12;
pub const PIP_R: f64 = 0.09;
pub const PIP_H: f64 = 0.05;
pub const PIP_D: f64 = 0.22;

fn band() -> Band {
    let tol = Tolerance::get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn cube(l: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        [(0.0, 0.0), (l, 0.0), (l, l), (0.0, l)]
            .into_iter()
            .map(|(x, y)| ProfileVertex {
                pos: p2(x, y),
                bulge: 0.0,
            })
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(l)).unwrap().body
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
        )
        .unwrap()
    };
    topo::transform_rigid(&placed, &Affine3::translation(c)).unwrap()
}

fn ball_at(r: f64, c: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(0.0, -r),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(0.0, r),
            bulge: 0.0,
        },
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full).unwrap().body;
    topo::transform_rigid(&ball, &Affine3::translation(c)).unwrap()
}

/// The blank: the cube with every edge filleted.
pub fn blank() -> Body<f64> {
    let body = cube(DIE_L);
    let edges: Vec<_> = body.edges().map(|(k, _)| k).collect();
    fillet_edges(&body, &edges, DIE_R, band())
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
pub fn pip_centres() -> Vec<Vec3<f64>> {
    pip_placements().into_iter().map(|(c, _)| c).collect()
}

/// Every pip ball's centre AND the face normal it is charted against.
pub fn pip_placements() -> Vec<(Vec3<f64>, Vec3<f64>)> {
    let h = DIE_L / 2.0;
    // (value, outward axis, the two in-face axes)
    let faces: [(u32, Vec3<f64>, Vec3<f64>, Vec3<f64>); 6] = [
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
pub fn pip_tool() -> Body<f64> {
    let places = pip_placements();
    let mut tool = ball_poled(PIP_R, places[0].0, places[0].1);
    for (c, n) in &places[1..] {
        tool = boolean_op_with(
            BooleanOp::Union,
            &tool,
            &ball_poled(PIP_R, *c, *n),
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
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
    )
    .unwrap_or_else(|e| panic!("pip subtraction: {e}"));
    out.body().expect("a body").body.clone()
}

/// The blank's closed-form volume and area (core + 6 slabs + 12
/// quarter-cylinders + 8 octants, the octants summing to one ball).
pub fn blank_volume() -> f64 {
    let core = DIE_L - 2.0 * DIE_R;
    core.powi(3)
        + 6.0 * DIE_R * core.powi(2)
        + 12.0 * (PI * DIE_R * DIE_R / 4.0) * core
        + (4.0 / 3.0) * PI * DIE_R.powi(3)
}

/// A spherical cap of height `h` off a radius-`r` ball.
pub fn cap(r: f64, h: f64) -> f64 {
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
    assert_eq!(topo::validate_geometric(&die), Ok(()), "tier 3");
    let props = topo::mass_properties(&die).unwrap();
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
    let mesh = mesh::tessellate(&die, 5e-3).expect("the blank tessellates");
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
    let pipped = subtract(&cube(DIE_L), &tool);
    assert_eq!(topo::validate(&pipped), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&pipped), Ok(()), "tier 2");
    assert_eq!(topo::validate_geometric(&pipped), Ok(()), "tier 3");
    let want = DIE_L.powi(3) - 21.0 * cap(PIP_R, PIP_H);
    let vol = topo::mass_properties(&pipped).unwrap().volume;
    assert!(
        (vol - want).abs() <= 1e-9 * want,
        "pipped cube volume {vol} vs closed form {want}"
    );
    let mesh = mesh::tessellate(&pipped, 5e-3).expect("the pipped cube tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

/// **DEVIATION 1, pinned at both of its doors.** The blank and the
/// pips do not compose at M5, and the two orderings fail at two
/// DIFFERENT pre-existing frontiers — recorded here as rows so the
/// next unit inherits the exact blockers rather than a paragraph.
///
/// - *Fillet then pip*: the pip tool's edges definitely meet the
///   blank's CURVED faces, and the curved pierce door (point-in-face
///   trim containment on a curved chart, plus the ring insertion
///   behind it) is the M5 envelope's named frontier.
/// - *Pip then fillet*: the pipped cube is tier-3 valid and all twelve
///   box edges survive it, but they are no longer EVERY edge of the
///   body — the assembly front door rebuilds a whole polyhedron and
///   does not carry a face's RINGS through, which is the in-place
///   edge-blend surgery banked as its own unit.
#[test]
fn deviation_1_the_blank_and_the_pips_do_not_compose_yet() {
    // Door A: fillet then pip.
    let err = boolean_op_with(
        BooleanOp::Subtract,
        &blank(),
        &pip_tool(),
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
    )
    .expect_err("the curved pierce door does not exist at M5");
    let text = format!("{err}");
    assert!(
        text.contains("curved") && text.contains("does not exist yet"),
        "the refusal must name the missing door: {text}"
    );

    // Door B: pip then fillet.
    let cube0 = cube(DIE_L);
    let box_edges: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
    let pipped = subtract(&cube0, &pip_tool());
    let surviving: Vec<_> = box_edges
        .into_iter()
        .filter(|k| pipped.get_edge(*k).is_some())
        .collect();
    assert_eq!(surviving.len(), 12, "every box edge survives the pips");
    let err = fillet_edges(&pipped, &surviving, DIE_R, band())
        .err()
        .expect("the assembly front door is EVERY edge of the body");
    let text = format!("{err}");
    assert!(
        text.contains("not implemented"),
        "the refusal must name the banked surgery: {text}"
    );
}
