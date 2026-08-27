//! SCRATCH 5 — is the opened rim's defect the POLE?
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::authoring::{p2, validated};
use pncad::geom::Surface;
use pncad::geom_core::{Band, Point2, Tol, Vec2};
use pncad::prelude::{Open, Start};
use pncad::profile::{ProfileLoop, SketchPlane};
use pncad::sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use pncad::topo::Body;

const T: f64 = 1.0 / 128.0;

fn revolved(lp: ProfileLoop<f64>, tol: Tol) -> Body<f64> {
    revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("validates"),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("revolves")
    .body
}

fn report(name: &str, body: &Body<f64>, y: f64, t: f64, tol: Tol) {
    let band = Band::linear(tol).expect("band");
    let chart: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    print!("--- {name}: chart {} faces -> ", chart.len());
    match pncad::topo::shell_open(body, t, &chart, 1e-6, band, tol) {
        Ok(cup) => {
            let (v, e, f) = (
                cup.vertices().count(),
                cup.edges().count(),
                cup.faces().count(),
            );
            let r: usize = cup.faces().map(|(_, x)| x.rings.len()).sum();
            let s = cup.shells().count();
            let g = s as i64 - (v as i64 - e as i64 + f as i64 - r as i64) / 2;
            println!(
                "OK v={v} e={e} f={f} rings={r} shells={s} genus={g} tier3={:?} mesh={:?}",
                pncad::topo::validate_geometric(&cup, tol),
                pncad::mesh::tessellate(&cup, 1e-3, tol).map(|m| m.patches.len()),
            );
        }
        Err(e) => println!("REFUSED {e:?}"),
    }
}

/// The stepped pot, mouth at the axis (a POLE disc).
#[test]
fn pole_mouth() {
    let tol = Tol::witness();
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
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
        .line_to(Point2::new(3.0 / 64.0, 8.0 / 64.0), tol)
        .expect("neck")
        .line_to(Point2::new(0.0, 8.0 / 64.0), tol)
        .expect("mouth")
        .line_to(Start, tol)
        .expect("axis")
        .into();
    report(
        "stepped pot, POLE mouth",
        &revolved(lp, tol),
        8.0 / 64.0,
        T,
        tol,
    );
}

/// The same pot BORED: the mouth is a true annulus, no axis anywhere.
#[test]
fn annular_mouth() {
    let tol = Tol::witness();
    let b = 1.0 / 64.0;
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(b, 0.0))
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
        .line_to(Point2::new(3.0 / 64.0, 8.0 / 64.0), tol)
        .expect("neck")
        .line_to(Point2::new(b, 8.0 / 64.0), tol)
        .expect("mouth annulus")
        .line_to(Start, tol)
        .expect("bore")
        .into();
    report(
        "stepped pot, ANNULAR mouth",
        &revolved(lp, tol),
        8.0 / 64.0,
        T,
        tol,
    );
}

/// The box the acceptance corpus uses, for contrast.
#[test]
fn box_top() {
    let tol = Tol::witness();
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(0.2, 0.0), tol)
        .expect("a")
        .line_to(Point2::new(0.2, 0.3), tol)
        .expect("b")
        .line_to(Point2::new(0.0, 0.3), tol)
        .expect("c")
        .line_to(Start, tol)
        .expect("d")
        .into();
    let profile = validated(SketchPlane::xy(), vec![lp], tol).expect("validates");
    let body = extrude(&profile, Extrusion::Distance(0.25), tol)
        .expect("extrudes")
        .body;
    // the extrusion's top cap sits at z = 0.25; find it by normal
    let band = Band::linear(tol).expect("band");
    let top: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9 && (origin.z - 0.25).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    let cup = pncad::topo::shell_open(&body, 0.02, &top, 1e-6, band, tol).expect("opens");
    let (v, e, f) = (
        cup.vertices().count(),
        cup.edges().count(),
        cup.faces().count(),
    );
    let r: usize = cup.faces().map(|(_, x)| x.rings.len()).sum();
    let s = cup.shells().count();
    println!(
        "--- box cup: v={v} e={e} f={f} rings={r} shells={s} genus={} mesh={:?}",
        s as i64 - (v as i64 - e as i64 + f as i64 - r as i64) / 2,
        pncad::mesh::tessellate(&cup, 1e-3, tol).map(|m| m.patches.len()),
    );
}

/// The SEALED stepped pot: what the vessel would ship as.
#[test]
fn sealed_pot() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
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
        .line_to(Point2::new(3.0 / 64.0, 8.0 / 64.0), tol)
        .expect("neck")
        .line_to(Point2::new(0.0, 8.0 / 64.0), tol)
        .expect("mouth")
        .line_to(Start, tol)
        .expect("axis")
        .into();
    let body = revolved(lp, tol);
    let sealed = pncad::topo::shell(&body, T, 1e-6, band, tol).expect("seals");
    let (v, e, f) = (
        sealed.vertices().count(),
        sealed.edges().count(),
        sealed.faces().count(),
    );
    let r: usize = sealed.faces().map(|(_, x)| x.rings.len()).sum();
    let s = sealed.shells().count();
    let props = pncad::topo::mass_properties(&sealed, tol).expect("props");
    println!(
        "--- sealed pot: v={v} e={e} f={f} rings={r} shells={s} genus={} V={} pad={} A={} tier3={:?} mesh={:?}",
        s as i64 - (v as i64 - e as i64 + f as i64 - r as i64) / 2,
        props.volume,
        props.volume_pad,
        props.surface_area,
        pncad::topo::validate_geometric(&sealed, tol),
        pncad::mesh::tessellate(&sealed, 2e-4, tol)
            .map(|m| pncad::mesh::validate::triangle_count(&m)),
    );
    let classes = pncad::topo::classify_shells(&sealed, tol).expect("classify");
    for c in &classes {
        println!("    shell {:?} role {:?} V={}", c.shell, c.role, c.volume);
    }
    println!(
        "    STEP {:?}",
        pncad::step_export::step_string(
            &sealed,
            &pncad::step_export::StepOptions {
                product_name: "sealed".into(),
                ..Default::default()
            },
            tol,
        )
        .map(|s| s.len())
    );
}
