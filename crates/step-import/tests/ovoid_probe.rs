//! Private designer probe (never committed): what a body with an exact
//! rational NURBS wall of revolution with poles (a quarter spheroid
//! wedge) does today at import/at-rest, measure, tessellation, export
//! and the boolean, against the same body with aspect 1 (promoted to a
//! sphere) as the control.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};
use std::fmt::Write as _;

use geom_core::{Point2, Tol, Vec2};
use profile::SketchPlane;
use profile::test_support::bulge_loop;
use quantity::Length;
use step_import::{ImportOptions, StepImport, import_step};

const C45: f64 = core::f64::consts::FRAC_1_SQRT_2;

/// The source: a half disc of radius 1 on the y axis revolved a quarter
/// turn about y, written as STEP.
fn sphere_wedge_step(tol: Tol) -> String {
    let lp = bulge_loop(vec![
        (Point2::new(0.0, -1.0), 1.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let vp = profile::Profile::new(SketchPlane::<f64>::xy(), vec![lp])
        .validate(tol)
        .expect("the half disc validates");
    let axis = sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let body = sweep::revolve(&vp, axis, sweep::Revolution::Partial(FRAC_PI_2), tol)
        .expect("the quarter wedge revolves")
        .body;
    step_export::step_string(
        &topo::test_support::finished("wedge", body, tol),
        &step_export::StepOptions::default(),
        tol,
    )
    .expect("the wedge writes")
}

#[test]
fn dump_sphere_wedge() {
    let text = sphere_wedge_step(Tol::witness());
    eprintln!("{text}");
}

fn arc_pts(a0: f64, a1: f64, spans: usize) -> Vec<((f64, f64), f64)> {
    let a = (a1 - a0) / spans as f64;
    let snap = |x: f64| if x.abs() < 1e-12 { 0.0 } else if (x - 1.0).abs() < 1e-12 { 1.0 } else if (x + 1.0).abs() < 1e-12 { -1.0 } else { x };
    let at = |t: f64, k: f64| (snap(t.cos() / k), snap(t.sin() / k));
    let mut v = vec![(at(a0, 1.0), 1.0)];
    for i in 0..spans {
        let (b0, b1) = (a0 + a * i as f64, a0 + a * (i + 1) as f64);
        let c = (0.5 * a).cos();
        v.push((at(0.5 * (b0 + b1), c), c));
        v.push((at(b1, 1.0), 1.0));
    }
    v
}

fn pts(xyz: &[(f64, f64, f64)], next: &mut usize, out: &mut String) -> Vec<String> {
    xyz.iter()
        .map(|&(x, y, z)| {
            writeln!(out, "#{next} = CARTESIAN_POINT('', ({x:?}, {y:?}, {z:?}));").unwrap();
            *next += 1;
            format!("#{}", *next - 1)
        })
        .collect()
}

fn on(m: (f64, f64), t: (f64, f64), a: f64) -> (f64, f64, f64) {
    (m.0 * t.0, a * m.1, -m.0 * t.1)
}

/// The quarter spheroid wedge: every point's y scaled by `a`, the sphere
/// swapped for its exact rational net, the meridians for its rows.
fn spheroid_wedge_step(tol: Tol, a: f64) -> String {
    let src = sphere_wedge_step(tol);
    let mut text = String::new();
    for l in src.lines() {
        if l.contains("CARTESIAN_POINT") {
            let (head, rest) = l.split_once("', (").unwrap();
            let inner = rest.split_once("))").unwrap().0;
            let v: Vec<f64> = inner.split(',').map(|s| s.trim().parse().unwrap()).collect();
            writeln!(text, "{head}', ({:?}, {:?}, {:?}));", v[0], v[1] * a, v[2]).unwrap();
        } else {
            writeln!(text, "{l}").unwrap();
        }
    }
    let (mut next, mut extra) = (1_000_000usize, String::new());
    let mer = arc_pts(-FRAC_PI_2, FRAC_PI_2, 2);
    let turn = arc_pts(0.0, FRAC_PI_2, 1);
    let (mut rows, mut ws) = (Vec::new(), Vec::new());
    for &(m, wm) in &mer {
        let xyz: Vec<_> = turn.iter().map(|&(t, _)| on(m, t, a)).collect();
        rows.push(format!("({})", pts(&xyz, &mut next, &mut extra).join(", ")));
        let w: Vec<String> = turn.iter().map(|&(_, wt)| format!("{:?}", wm * wt)).collect();
        ws.push(format!("({})", w.join(", ")));
    }
    let surf = format!(
        "#46 = ( BOUNDED_SURFACE() B_SPLINE_SURFACE(2, 2, ({}), .UNSPECIFIED., .F., .F., .F.) \
         B_SPLINE_SURFACE_WITH_KNOTS((3, 2, 3), (3, 3), (0.0, 0.5, 1.0), (0.0, 1.0), .UNSPECIFIED.) \
         GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_SURFACE(({})) \
         REPRESENTATION_ITEM('') SURFACE() );",
        rows.join(", "),
        ws.join(", ")
    );
    let curve = |t: (f64, f64), next: &mut usize, extra: &mut String| {
        let xyz: Vec<_> = mer.iter().map(|&(m, _)| on(m, t, a)).collect();
        let r = pts(&xyz, next, extra);
        let w: Vec<String> = mer.iter().map(|&(_, w)| format!("{w:?}")).collect();
        format!(
            "( BOUNDED_CURVE() B_SPLINE_CURVE(2, ({}), .UNSPECIFIED., .F., .F.) \
             B_SPLINE_CURVE_WITH_KNOTS((3, 2, 3), (0.0, 0.5, 1.0), .UNSPECIFIED.) CURVE() \
             GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_CURVE(({})) REPRESENTATION_ITEM('') );",
            r.join(", "),
            w.join(", ")
        )
    };
    let c14 = format!("#14 = {}", curve((0.0, 1.0), &mut next, &mut extra));
    let c35 = format!("#35 = {}", curve((1.0, 0.0), &mut next, &mut extra));
    let mut out = String::new();
    for l in text.lines() {
        let l = if l.starts_with("#46 = ") {
            surf.clone()
        } else if l.starts_with("#14 = ") {
            c14.clone()
        } else if l.starts_with("#35 = ") {
            c35.clone()
        } else if l.starts_with("#15 = ") || l.starts_with("#36 = ") {
            l.replace(".T.);", ".F.);")
        } else if l.starts_with("#51 = ") {
            l.replace(".T.);", ".F.);")
        } else {
            l.to_owned()
        };
        writeln!(out, "{l}").unwrap();
    }
    out.replacen("ENDSEC;\nEND-ISO", &format!("{extra}ENDSEC;\nEND-ISO"), 1)
}

fn kinds(body: &topo::AtRestBody<f64>) -> Vec<String> {
    body.faces()
        .map(|(fk, _)| format!("{:?}", body.face_surface(fk).map(|s| s.kind())))
        .collect()
}

fn vol(what: &str, body: &topo::AtRestBody<f64>) -> String {
    match body.measure(topo::Settle::Resolution(Length::from_meters(1.024e-6))) {
        Ok(m) => format!("{what}: V = {} ± {:e}, stop {:?}", m.enclosure.volume, m.enclosure.volume_pad, m.stop),
        Err(e) => format!("{what}: measure REFUSES {e:?}"),
    }
}

fn probe(a: f64) {
    let tol = Tol::witness();
    let text = spheroid_wedge_step(tol, a);
    eprintln!("==== aspect {a}: closed-form V = {}", PI * a / 3.0);
    let body = match import_step(&text, &ImportOptions::default(), tol) {
        Ok(StepImport::Solid { body, .. }) => body,
        Ok(_) => { eprintln!("IMPORT: not a solid"); return; }
        Err(e) => { eprintln!("IMPORT REFUSES: {}", e.render()); return; }
    };
    eprintln!("IMPORT ok (tier-3 at rest): faces {:?}", kinds(&body));
    eprintln!("{}", vol("MEASURE", &body));
    match mesh::tessellate(&body, 1e-3, tol) {
        Ok(m) => eprintln!("TESSELLATE ok: check {:?}, signed volume {}", mesh::validate::check_mesh(&m), mesh::validate::signed_volume(&m)),
        Err(e) => eprintln!("TESSELLATE REFUSES {e:?}"),
    }
    match step_export::step_string(&body, &step_export::StepOptions::default(), tol) {
        Ok(t) => eprintln!("EXPORT ok ({} bytes; B_SPLINE_SURFACE {})", t.len(), t.matches("B_SPLINE_SURFACE(").count()),
        Err(e) => eprintln!("EXPORT REFUSES {e:?}"),
    }
    let bricks: [(&str, (f64, f64), (f64, f64), (f64, f64)); 3] = [
        ("brick across the wall at the equator", (0.5, 2.0), (-0.2, 0.2), (-2.0, -0.3)),
        ("slab over the north pole", (-2.0, 2.0), (0.5 * a, 2.0 * a), (-2.0, 2.0)),
        ("brick inside", (0.1, 0.3), (-0.1, 0.1), (-0.3, -0.1)),
    ];
    for (name, x, y, z) in bricks {
        let b = topo::test_support::finished(
            "brick",
            topo::test_support::brick_with_fresh_keys::<f64>(x, y, z, tol),
            tol,
        );
        for (op, r) in [
            ("∪", topo::union(&body, &b, tol)),
            ("∖", topo::subtract(&body, &b, tol)),
            ("∩", topo::intersect(&body, &b, tol)),
        ] {
            match r {
                Ok(topo::BooleanResult::Body(bb)) => eprintln!("BOOL {name} {op}: builds; {}", vol("result", &bb.body)),
                Ok(topo::BooleanResult::Empty) => eprintln!("BOOL {name} {op}: empty"),
                Err(e) => eprintln!("BOOL {name} {op}: REFUSES {e:?}"),
            }
        }
    }
}

#[test]
fn spheroid_wedge_census() {
    probe(1.0);
    probe(1.6);
}
