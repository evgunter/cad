//! S28 guard adversarial probe 2: `Body::split_edge` (a PUBLIC topo
//! door) on a curved face's boundary, then `mesh::tessellate`.
#![allow(clippy::all, clippy::pedantic)]

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use geom_core::{Affine3, Point2, Point3, Tolerance, Vec2, Vec3};
use mesh::tessellate;
use mesh::validate::check_mesh;
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::Body;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}
fn validated(loops: Vec<ProfileLoop<f64>>) -> Option<ValidatedProfile<f64>> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tolerance::get())
        .ok()
}
fn axis_y() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}
fn rev(lp: ProfileLoop<f64>, r: Revolution<f64>) -> Option<Body<f64>> {
    revolve(&validated(vec![lp])?, axis_y(), r).ok().map(|o| o.body)
}
fn bulge(sweep: f64) -> f64 {
    (sweep / 4.0).tan()
}

fn bodies() -> Vec<(String, Body<f64>)> {
    let mut v: Vec<(String, Option<Body<f64>>)> = Vec::new();
    let tri = || {
        <ProfileLoop<f64> as RawLoop<f64>>::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)])
    };
    let rect = || {
        <ProfileLoop<f64> as RawLoop<f64>>::polygon([
            p2(1.0, 0.0),
            p2(2.0, 0.0),
            p2(2.0, 1.0),
            p2(1.0, 1.0),
        ])
    };
    let frust = || {
        <ProfileLoop<f64> as RawLoop<f64>>::polygon([
            p2(0.5, 0.0),
            p2(2.0, 0.0),
            p2(1.0, 2.0),
            p2(0.5, 2.0),
        ])
    };
    let half = || {
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -1.0), 1.0),
            ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ])
    };
    let ring = || {
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(2.0, -0.5), 1.0),
            ProfileVertex::new(p2(2.0, 0.5), 1.0),
        ])
    };
    let cap = || {
        let h = 0.3f64;
        let x = (1.0 - h * h).sqrt();
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -1.0), bulge(h.asin() + FRAC_PI_2)),
            ProfileVertex::new(p2(x, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ])
    };
    v.push(("cone full".into(), rev(tri(), Revolution::Full)));
    v.push((
        "cone wedge(pi/2)".into(),
        rev(tri(), Revolution::Partial(FRAC_PI_2)),
    ));
    v.push((
        "cone wedge(2pi-0.05)".into(),
        rev(tri(), Revolution::Partial(TAU - 0.05)),
    ));
    v.push(("frustum full".into(), rev(frust(), Revolution::Full)));
    v.push((
        "frustum wedge(pi/2)".into(),
        rev(frust(), Revolution::Partial(FRAC_PI_2)),
    ));
    v.push(("washer full".into(), rev(rect(), Revolution::Full)));
    v.push((
        "washer wedge(pi/2)".into(),
        rev(rect(), Revolution::Partial(FRAC_PI_2)),
    ));
    v.push(("ball".into(), rev(half(), Revolution::Full)));
    v.push((
        "sphere band(pi/2)".into(),
        rev(half(), Revolution::Partial(FRAC_PI_2)),
    ));
    v.push(("sphere cap".into(), rev(cap(), Revolution::Full)));
    v.push((
        "sphere cap wedge(pi/2)".into(),
        rev(cap(), Revolution::Partial(FRAC_PI_2)),
    ));
    v.push(("donut".into(), rev(ring(), Revolution::Full)));
    v.push((
        "donut wedge(pi/2)".into(),
        rev(ring(), Revolution::Partial(FRAC_PI_2)),
    ));
    let sq = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
        p2(0.0, 0.0),
        p2(1.0, 0.0),
        p2(1.0, 1.0),
        p2(0.0, 1.0),
    ]);
    v.push((
        "axis wedge(pi/2)".into(),
        rev(sq, Revolution::Partial(FRAC_PI_2)),
    ));
    let dia =
        <ProfileLoop<f64> as RawLoop<f64>>::polygon([p2(1.0, 0.0), p2(2.0, 1.0), p2(1.0, 2.0)]);
    v.push((
        "mirror nappe(pi+0.05)".into(),
        rev(dia, Revolution::Partial(PI + 0.05)),
    ));
    // an extruded round: cylinder faces with LINE meridians
    let b = (PI / 8.0).tan();
    let mut lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.5, 0.0), 0.0),
        ProfileVertex::new(p2(1.5, 0.0), b),
        ProfileVertex::new(p2(2.0, 0.5), 0.0),
        ProfileVertex::new(p2(2.0, 1.5), b),
        ProfileVertex::new(p2(1.5, 2.0), 0.0),
        ProfileVertex::new(p2(0.5, 2.0), b),
        ProfileVertex::new(p2(0.0, 1.5), 0.0),
        ProfileVertex::new(p2(0.0, 0.5), b),
    ]);
    let n = lp.vertices().len();
    lp = lp.with_tangent_joints((0..n).collect());
    v.push((
        "rounded prism".into(),
        validated(vec![lp])
            .and_then(|p| extrude(&p, Extrusion::Distance(1.0)).ok())
            .map(|o| o.body),
    ));

    let mut out = Vec::new();
    for (n, b) in v {
        match b {
            Some(b) => out.push((n, b)),
            None => println!("  [skip] {n}"),
        }
    }
    out
}

fn carrier_kind(body: &Body<f64>, ek: topo::EdgeKey) -> (&'static str, f64, f64) {
    let e = body.get_edge(ek).unwrap();
    let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    let (t0, t1) = c.params();
    let k = match c.carrier() {
        geom_curves::Curve3::Line { .. } => "Line",
        geom_curves::Curve3::Circle { .. } => "Circle",
        geom_curves::Curve3::Ellipse { .. } => "Ellipse",
        geom_curves::Curve3::Nurbs(_) => "Nurbs",
        _ => "?",
    };
    (k, t0, t1)
}

fn maps() -> Vec<(&'static str, Affine3<f64>)> {
    let irr = Vec3::new(0.3178_f64, 0.9412, -0.1109);
    let irr = irr * (1.0 / irr.norm());
    vec![
        ("id", Affine3::identity()),
        (
            "rotz(0.7391)",
            Affine3::rotation_about_axis(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                0.739_085_133_215_161,
            ),
        ),
        (
            "rot(irr,1/3)+tr",
            Affine3::from_parts(
                Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), irr, 1.0 / 3.0).linear,
                Vec3::new(0.117, -0.339_1, 5.001_7),
            ),
        ),
    ]
}

fn main() {
    let delta = 0.1;
    let mut hits = Vec::new();
    let mut tried = 0usize;
    for (name, body) in bodies() {
        // baseline must pass
        match tessellate(&body, delta) {
            Ok(_) => {}
            Err(e) => {
                println!("[baseline REFUSED] {name}: {e:?}");
                continue;
            }
        }
        for (mn, map) in maps() {
            let Ok(pb) = topo::transform_rigid(&body, &map) else {
                println!("  [transform refused] {name} [{mn}]");
                continue;
            };
            for d in [0.25, 0.1, 0.04, 0.01] {
                match tessellate(&pb, d) {
                    Ok(m) => {
                        if let Err(e) = check_mesh(&m) {
                            println!("  BASELINE DIRTY {name} [{mn}] d={d}: {e:?}");
                        }
                    }
                    Err(e) => println!("  BASELINE REFUSED {name} [{mn}] d={d}: {e:?}"),
                }
            }
        }
        let eks: Vec<_> = body.edges().map(|(k, _)| k).collect();
        for ek in eks {
            let (kind, t0, t1) = carrier_kind(&body, ek);
            for frac in [0.5, 0.3129] {
                for double in [false, true] {
                    let t = t0 + (t1 - t0) * frac;
                    let mut b = body.clone();
                    let Ok(_) = b.split_edge(ek, t) else { continue };
                    if double {
                        let t2 = t0 + (t1 - t0) * (frac * 0.5);
                        if b.split_edge(ek, t2).is_err() {
                            continue;
                        }
                    }
                    for (mn, map) in maps() {
                        let Ok(pb) = topo::transform_rigid(&b, &map) else {
                            continue;
                        };
                        tried += 1;
                        let tag = format!(
                            "{name} split {kind} {ek:?} @{frac}{} [{mn}]",
                            if double { " x2" } else { "" }
                        );
                        let status = match tessellate(&pb, delta) {
                            Ok(m) => {
                                let n: usize =
                                    m.patches.iter().map(|p| p.triangles.len()).sum();
                                match check_mesh(&m) {
                                    Ok(()) => format!("CLEAN n={n}"),
                                    Err(e) => format!("DIRTY n={n} {e:?}"),
                                }
                            }
                            Err(e) => {
                                let s = format!("{e:?}");
                                if s.contains("UnsupportedCurvedDomain") {
                                    hits.push(format!("{tag}: {s}"));
                                    "REFUSED_GUARD".to_string()
                                } else {
                                    format!("REFUSED {}", s.split(' ').next().unwrap_or(""))
                                }
                            }
                        };
                        println!("ROW\t{tag}\t{status}");
                    }
                }
            }
        }
    }
    println!("\n{tried} splits tessellated; {} UnsupportedCurvedDomain", hits.len());
    for h in &hits {
        println!("  *** {h}");
    }
}
