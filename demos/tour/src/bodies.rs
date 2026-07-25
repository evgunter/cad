//! The tour's sweep bodies, each built through the public
//! profile/sweep API. Retirements: the donut (#91 refresh) and the
//! pulley (#91 revision pass) both fold into the rope-groove sheave —
//! it carries plane + cylinder + cone + torus on one real part; the
//! plain wedge became the quarter-turn chute (same partial-revolve
//! capability, a real profile).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tolerance, Vec2};
use profile::{LoopBuilder, Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};

use crate::{SceneBody, Stop, View};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tolerance::get())
        .expect("profile validation")
}

fn axis_y() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

/// A circle as a two-vertex closed arc carrier (bulge 1 = semicircle).
fn circle(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(cx + r, cy),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(cx - r, cy),
            bulge: 1.0,
        },
    ])
}

/// L-bracket: polyline + one fillet arc at the inner corner, extruded.
fn bracket() -> topo::Body<f64> {
    let lp = LoopBuilder::start(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .line_to(p2(3.0, 1.0))
        .line_to(p2(1.5, 1.0))
        .arc_to_via(p2(1.146, 1.146), p2(1.0, 1.5)) // r = 0.5 inner fillet
        .line_to(p2(1.0, 3.0))
        .line_to(p2(0.0, 3.0))
        .close();
    extrude(&validated(vec![lp]), Extrusion::Distance(0.75))
        .expect("extrude bracket")
        .body
}

/// Rectangular plate with two circular holes: a genus-2 extrusion.
fn plate() -> topo::Body<f64> {
    let outer = ProfileLoop::polygon([p2(-3.0, -1.5), p2(3.0, -1.5), p2(3.0, 1.5), p2(-3.0, 1.5)]);
    let holes = vec![circle(-1.5, 0.0, 0.7), circle(1.5, 0.0, 0.7)];
    let mut loops = vec![outer];
    loops.extend(holes);
    extrude(&validated(loops), Extrusion::Distance(0.6))
        .expect("extrude plate")
        .body
}

/// Solid vase: an axis-touching profile — conical base, spherical
/// belly (the arc's carrier center sits ON the revolve axis, so the
/// swept surface is a sphere zone; an OFF-axis arc carrier sweeps a
/// ring torus — see the sheave, which showcases exactly that), conical
/// flared lip — fully revolved about the y axis.
fn vase() -> topo::Body<f64> {
    // Belly arc: circle of radius 1.3 centered at (0, 0.8) — on the
    // axis — from (1.2, 0.3) through (1.3, 0.8) to (0.5, 2.0).
    let lp = LoopBuilder::start(p2(0.0, 0.0))
        .line_to(p2(1.2, 0.0))
        .line_to(p2(1.2, 0.3))
        .arc_to_via(p2(1.3, 0.8), p2(0.5, 2.0))
        .line_to(p2(0.9, 2.5))
        .line_to(p2(0.0, 2.5))
        .close();
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .expect("revolve vase")
        .body
}

/// Rope-groove sheave (the donut's AND the pulley's successor): full
/// revolve of a polyline + arc profile — stepped hub, recessed web,
/// rim with TAPERED (conical) shoulders flanking a SEMICIRCULAR rope
/// groove. The groove arc's carrier center sits OFF the revolve axis,
/// so its wall is a ring-torus zone (`WallKind::Torus` — only
/// horn/spindle tori refuse typed); the shoulders are cone zones. One
/// stop carries all four analytic surface kinds (plane, cylinder,
/// cone, torus), which is why the pulley (plane/cylinder/cone only)
/// retired into it at the #91 revision pass. Center bore → genus 1.
fn sheave() -> (topo::Body<f64>, String) {
    let lp = LoopBuilder::start(p2(0.4, 0.0))
        .line_to(p2(0.9, 0.0))
        .line_to(p2(0.9, 0.25))
        .line_to(p2(1.6, 0.25))
        .line_to(p2(1.6, 0.0))
        .line_to(p2(2.0, 0.0))
        .line_to(p2(2.1, 0.2)) // tapered shoulder: cone zone
        .arc_to_via(p2(1.8, 0.5), p2(2.1, 0.8)) // groove: r = 0.3 semicircle
        .line_to(p2(2.0, 1.0)) // tapered shoulder: cone zone
        .line_to(p2(1.6, 1.0))
        .line_to(p2(1.6, 0.75))
        .line_to(p2(0.9, 0.75))
        .line_to(p2(0.9, 1.0))
        .line_to(p2(0.4, 1.0))
        .close();
    let body = revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .expect("revolve sheave")
        .body;
    // Closed-form volume by Pappus (independent derivation, exact
    // rationals): hub + web + rim annuli + two cone shoulder wedges,
    // minus the revolved half-disc groove:
    //   V = 2*pi*(1997/1200) - (189/1000)*pi^2.
    let pi = core::f64::consts::PI;
    let oracle = 2.0 * (1997.0 / 1200.0) * pi - 0.189 * pi * pi;
    let v = topo::mass_properties(&body)
        .expect("sheave mass properties")
        .volume;
    let rel = ((v - oracle) / oracle).abs();
    assert!(
        rel < 1e-12,
        "sheave volume {v} vs closed-form {oracle} (rel {rel:.3e})"
    );
    let kind_count = |pred: fn(&topo::Surface<f64>) -> bool| {
        body.faces()
            .filter(|(_, f)| body.get_surface(f.surface).is_some_and(pred))
            .count()
    };
    assert_eq!(
        kind_count(|s| matches!(s, topo::Surface::Torus { .. })),
        1,
        "the groove must be a torus wall"
    );
    assert_eq!(
        kind_count(|s| matches!(s, topo::Surface::Cone { .. })),
        2,
        "the rim shoulders must be cone walls"
    );
    let note = format!(
        "surface census: 6 planes, 5 cylinders, 2 CONES (tapered rim shoulders), \
         1 TORUS (groove) — all four analytic wall kinds on one part (the pulley, \
         whose kinds were a subset, retired here); volume matches the closed-form \
         Pappus value 2pi*1997/1200 - 0.189pi^2 to {rel:.1e} relative"
    );
    (body, note)
}

/// Quarter-turn chute (the wedge's successor at the #91 revision
/// pass): a C-channel cross-section swept through a 270-degree
/// partial revolve — a curved trough with wedge caps at both ends,
/// annular rims, and four cylinder bands. A more interesting partial
/// revolve than the old plain rectangle, still boolean-free.
fn chute() -> (topo::Body<f64>, String) {
    let lp = ProfileLoop::polygon([
        p2(1.0, 0.0),
        p2(1.75, 0.0),
        p2(1.75, 0.625),
        p2(1.5625, 0.625),
        p2(1.5625, 0.1875),
        p2(1.1875, 0.1875),
        p2(1.1875, 0.625),
        p2(1.0, 0.625),
    ]);
    let body = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(3.0 * core::f64::consts::FRAC_PI_2),
    )
    .expect("revolve chute")
    .body;
    // Pappus for the partial sweep (independent derivation, exact):
    // profile first moment 429/1024, angle 3pi/2 => V = (1287/2048)pi.
    let oracle = (1287.0 / 2048.0) * core::f64::consts::PI;
    let v = topo::mass_properties(&body)
        .expect("chute mass properties")
        .volume;
    let rel = ((v - oracle) / oracle).abs();
    assert!(
        rel < 1e-12,
        "chute volume {v} vs closed-form {oracle} (rel {rel:.3e})"
    );
    let note = format!(
        "C-channel profile x 270 degrees about y; volume matches the closed-form \
         Pappus value (1287/2048)pi to {rel:.1e} relative"
    );
    (body, note)
}

fn stop(
    name: &'static str,
    story: &'static str,
    ops: &'static str,
    delta: f64,
    view: View,
    color: [f64; 3],
    body: topo::Body<f64>,
    note: Option<String>,
) -> Stop {
    Stop {
        name,
        caption: String::new(),
        montage: true,
        story,
        ops,
        delta,
        note,
        view,
        bodies: vec![SceneBody::plain(name, color, body)],
    }
}

/// The sweep stops, in tour order.
pub fn stops() -> Vec<Stop> {
    let (sheave_body, sheave_note) = sheave();
    let (chute_body, chute_note) = chute();
    vec![
        stop(
            "bracket",
            "L-bracket with a filleted inner corner (polyline + tangent arc profile)",
            "LoopBuilder (line_to/arc_to_via) -> Profile::validate -> extrude(Distance)",
            1e-2,
            View {
                elev: 32.0,
                azim: -55.0,
                up: 'z',
            },
            [0.36, 0.56, 0.86],
            bracket(),
            None,
        ),
        stop(
            "plate",
            "plate with two circular holes — genus 2 (each hole: 2 rings, wall band)",
            "polygon outer + two closed arc-carrier holes -> extrude(Distance)",
            1e-2,
            View {
                elev: 42.0,
                azim: -60.0,
                up: 'z',
            },
            [0.86, 0.51, 0.27],
            plate(),
            None,
        ),
        stop(
            "vase",
            "solid vase — axis-touching profile, spherical belly zone + conical lip",
            "LoopBuilder line/arc_to_via -> revolve(axis y, Full); sphere/cone/plane faces",
            2e-2,
            View {
                elev: 16.0,
                azim: -55.0,
                up: 'y',
            },
            [0.42, 0.72, 0.50],
            vase(),
            None,
        ),
        stop(
            "sheave",
            "rope-groove sheave — hub, web, TAPERED rim shoulders, semicircular groove: \
             plane + cylinder + cone + torus on one part",
            "LoopBuilder polyline + arc -> revolve(axis y, Full), genus 1",
            5e-2,
            View {
                elev: 26.0,
                azim: -55.0,
                up: 'y',
            },
            [0.78, 0.42, 0.72],
            sheave_body,
            Some(sheave_note),
        ),
        stop(
            "chute",
            "quarter-turn chute — C-channel profile swept 270 degrees; wedge caps, \
             curved trough",
            "8-gon C-channel -> revolve(axis y, Partial(3pi/2))",
            2e-2,
            View {
                elev: 35.0,
                azim: -40.0,
                up: 'y',
            },
            [0.44, 0.68, 0.78],
            chute_body,
            Some(chute_note),
        ),
    ]
}

/// The tour's coda: the kernel is fail-loud — a self-intersecting
/// (bowtie) profile is refused with a typed error, not a crash or a
/// silently broken solid.
pub fn finale_fail_loud() {
    println!("\n== finale: fail-loud ==");
    println!("   a bowtie (self-intersecting) profile is refused before any sweep runs:");
    let bowtie = ProfileLoop::polygon([p2(0.0, 0.0), p2(2.0, 2.0), p2(2.0, 0.0), p2(0.0, 2.0)]);
    match Profile::new(SketchPlane::xy(), vec![bowtie]).validate(Tolerance::get()) {
        Ok(_) => panic!("bowtie profile unexpectedly validated"),
        Err(e) => println!("   typed rejection: {e:?}"),
    }
}
