//! S28 guard adversarial probe (report-only lane, never committed).
//!
//! Hunts a LEGITIMATE body that `curved::require_swept_rectangle`
//! now refuses through the public `mesh::tessellate` door.
#![allow(clippy::all, clippy::pedantic)]

use std::f64::consts::{FRAC_PI_2, FRAC_PI_8, PI, TAU};

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
    match Profile::new(SketchPlane::xy(), loops).validate(Tolerance::get()) {
        Ok(v) => Some(v),
        Err(e) => {
            println!("  [profile refused] {e:?}");
            None
        }
    }
}

fn axis_y() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

fn rev(lp: ProfileLoop<f64>, r: Revolution<f64>) -> Option<Body<f64>> {
    let v = validated(vec![lp])?;
    match revolve(&v, axis_y(), r) {
        Ok(o) => Some(o.body),
        Err(e) => {
            println!("  [revolve refused] {e:?}");
            None
        }
    }
}

/// Bulge for an arc subtending `sweep` radians (CCW positive).
fn bulge(sweep: f64) -> f64 {
    (sweep / 4.0).tan()
}

fn ball(r: f64) -> Option<Body<f64>> {
    rev(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -r), 1.0),
            ProfileVertex::new(p2(0.0, r), 0.0),
        ]),
        Revolution::Full,
    )
}

fn sphere_band(theta: f64) -> Option<Body<f64>> {
    rev(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -1.0), 1.0),
            ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ]),
        Revolution::Partial(theta),
    )
}

/// A spherical CAP: sphere of radius 1 truncated at height `h` — the
/// face carries a rim AND a pole, `nv > 1`.
fn sphere_cap(h: f64, r: Revolution<f64>) -> Option<Body<f64>> {
    let x = (1.0f64 - h * h).sqrt();
    let sweep = h.asin() + FRAC_PI_2;
    rev(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -1.0), bulge(sweep)),
            ProfileVertex::new(p2(x, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        r,
    )
}

/// A sphere ZONE: two rims, no pole (the barrel of a sphere).
fn sphere_zone(h0: f64, h1: f64, r: Revolution<f64>) -> Option<Body<f64>> {
    let x0 = (1.0f64 - h0 * h0).sqrt();
    let x1 = (1.0f64 - h1 * h1).sqrt();
    let sweep = h1.asin() - h0.asin();
    rev(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, h0), 0.0),
            ProfileVertex::new(p2(x0, h0), bulge(sweep)),
            ProfileVertex::new(p2(x1, h1), 0.0),
            ProfileVertex::new(p2(0.0, h1), 0.0),
        ]),
        r,
    )
}

fn cone_body(r: Revolution<f64>) -> Option<Body<f64>> {
    rev(
        <ProfileLoop<f64> as RawLoop<f64>>::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]),
        r,
    )
}

/// Truncated cone (frustum): two rims, NO apex.
fn frustum(r: Revolution<f64>) -> Option<Body<f64>> {
    rev(
        <ProfileLoop<f64> as RawLoop<f64>>::polygon([
            p2(0.5, 0.0),
            p2(2.0, 0.0),
            p2(1.0, 2.0),
            p2(0.5, 2.0),
        ]),
        r,
    )
}

fn washer(r: Revolution<f64>) -> Option<Body<f64>> {
    rev(
        <ProfileLoop<f64> as RawLoop<f64>>::polygon([
            p2(1.0, 0.0),
            p2(2.0, 0.0),
            p2(2.0, 1.0),
            p2(1.0, 1.0),
        ]),
        r,
    )
}

fn donut(major: f64, minor: f64, r: Revolution<f64>) -> Option<Body<f64>> {
    rev(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(major, -minor), 1.0),
            ProfileVertex::new(p2(major, minor), 1.0),
        ]),
        r,
    )
}

/// A torus band: the tube cross-section is a pie slice, so the face's
/// two rims sit at NON-extreme minor angles (v neither 0 nor pi).
fn torus_band(major: f64, minor: f64, a0: f64, a1: f64, r: Revolution<f64>) -> Option<Body<f64>> {
    let pt = |a: f64| p2(major + minor * a.cos(), minor * a.sin());
    rev(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(major, 0.0), 0.0),
            ProfileVertex::new(pt(a0), bulge(a1 - a0)),
            ProfileVertex::new(pt(a1), 0.0),
        ]),
        r,
    )
}

fn axis_wedge(theta: f64) -> Option<Body<f64>> {
    rev(
        <ProfileLoop<f64> as RawLoop<f64>>::polygon([
            p2(0.0, 0.0),
            p2(1.0, 0.0),
            p2(1.0, 1.0),
            p2(0.0, 1.0),
        ]),
        Revolution::Partial(theta),
    )
}

fn mirror_nappe(theta: f64) -> Option<Body<f64>> {
    rev(
        <ProfileLoop<f64> as RawLoop<f64>>::polygon([p2(1.0, 0.0), p2(2.0, 1.0), p2(1.0, 2.0)]),
        Revolution::Partial(theta),
    )
}

fn rounded_prism(r: f64, height: f64) -> Option<Body<f64>> {
    let b = FRAC_PI_8.tan();
    let v = |pos: Point2<f64>, bulge: f64| ProfileVertex::new(pos, bulge);
    let mut lp = ProfileLoop::new(vec![
        v(p2(r, 0.0), 0.0),
        v(p2(2.0 - r, 0.0), b),
        v(p2(2.0, r), 0.0),
        v(p2(2.0, 2.0 - r), b),
        v(p2(2.0 - r, 2.0), 0.0),
        v(p2(r, 2.0), b),
        v(p2(0.0, 2.0 - r), 0.0),
        v(p2(0.0, r), b),
    ]);
    let n = lp.vertices().len();
    lp = lp.with_tangent_joints((0..n).collect());
    extrude(&validated(vec![lp])?, Extrusion::Distance(height))
        .ok()
        .map(|o| o.body)
}

fn die_pip() -> Option<Body<f64>> {
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
        p2(0.0, 0.0),
        p2(4.0, 0.0),
        p2(4.0, 4.0),
        p2(0.0, 4.0),
    ]);
    let slab = extrude(&validated(vec![lp])?, Extrusion::Distance(1.0))
        .ok()?
        .body;
    let ball = ball(0.5)?;
    let ball =
        topo::transform_rigid(&ball, &Affine3::translation(Vec3::new(2.0, 2.0, 1.2))).ok()?;
    Some(
        topo::boolean::subtract(&slab, &ball)
            .ok()?
            .body()?
            .body
            .clone(),
    )
}

/// The rigid maps every fixture is replayed under. The point of the
/// non-dyadic ones is the exactness claim in `entries_off_bbox`.
fn maps() -> Vec<(&'static str, Affine3<f64>)> {
    let irr = Vec3::new(0.3178_f64, 0.9412, -0.1109);
    let irr = irr * (1.0 / irr.norm());
    vec![
        ("identity", Affine3::identity()),
        (
            "translate(pi/7, e/11, sqrt2/3)",
            Affine3::translation(Vec3::new(
                PI / 7.0,
                std::f64::consts::E / 11.0,
                std::f64::consts::SQRT_2 / 3.0,
            )),
        ),
        (
            "rot z by 0.7391 (Dottie)",
            Affine3::rotation_about_axis(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                0.739_085_133_215_161,
            ),
        ),
        (
            "rot about irrational axis by 1/3 rad",
            Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), irr, 1.0 / 3.0),
        ),
        (
            "rot x by pi/3 (chart axis leaves +z)",
            Affine3::rotation_about_axis(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                PI / 3.0,
            ),
        ),
        (
            "rot x by pi/2 + translate",
            Affine3::from_parts(
                Affine3::rotation_about_axis(
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                    FRAC_PI_2,
                )
                .linear,
                Vec3::new(0.117, -0.339_1, 5.001_7),
            ),
        ),
    ]
}

fn fixtures() -> Vec<(String, Body<f64>)> {
    let mut v: Vec<(String, Option<Body<f64>>)> = Vec::new();
    let full = Revolution::Full;
    v.push(("ball".into(), ball(1.0)));
    v.push(("ball r=1e-4".into(), ball(1.0e-4)));
    v.push(("ball r=1e4".into(), ball(1.0e4)));
    for t in [0.05, FRAC_PI_2, PI, PI + 1e-9, TAU - 0.08, TAU - 1e-9] {
        v.push((format!("sphere band({t})"), sphere_band(t)));
    }
    v.push(("cone".into(), cone_body(full)));
    for t in [FRAC_PI_2, PI, TAU - 0.05] {
        v.push((format!("cone wedge({t})"), cone_body(Revolution::Partial(t))));
    }
    v.push(("frustum".into(), frustum(full)));
    for t in [FRAC_PI_2, PI, TAU - 0.05] {
        v.push((format!("frustum wedge({t})"), frustum(Revolution::Partial(t))));
    }
    v.push(("sphere cap(h=0.3)".into(), sphere_cap(0.3, full)));
    v.push(("sphere cap(h=-0.6)".into(), sphere_cap(-0.6, full)));
    for t in [FRAC_PI_2, PI, TAU - 0.05] {
        v.push((
            format!("sphere cap wedge({t})"),
            sphere_cap(0.3, Revolution::Partial(t)),
        ));
    }
    v.push(("sphere zone(-0.4,0.7)".into(), sphere_zone(-0.4, 0.7, full)));
    v.push(("sphere zone(0.1,0.7)".into(), sphere_zone(0.1, 0.7, full)));
    for t in [FRAC_PI_2, PI, TAU - 0.05] {
        v.push((
            format!("sphere zone wedge({t})"),
            sphere_zone(-0.4, 0.7, Revolution::Partial(t)),
        ));
    }
    v.push(("washer".into(), washer(full)));
    for t in [0.05, FRAC_PI_2, PI, PI + 1e-9, TAU - 0.05] {
        v.push((format!("washer wedge({t})"), washer(Revolution::Partial(t))));
    }
    v.push(("donut(2,0.5)".into(), donut(2.0, 0.5, full)));
    v.push(("donut(1,0.999) near-horn".into(), donut(1.0, 0.999, full)));
    v.push(("donut(1,1) horn".into(), donut(1.0, 1.0, full)));
    v.push(("donut(1,1.5) spindle".into(), donut(1.0, 1.5, full)));
    for t in [FRAC_PI_2, PI, TAU - 0.05] {
        v.push((
            format!("donut wedge({t})"),
            donut(2.0, 0.5, Revolution::Partial(t)),
        ));
    }
    v.push(("torus band(0.4,1.9)".into(), torus_band(2.0, 0.5, 0.4, 1.9, full)));
    v.push(("torus band(-1.1,0.3)".into(), torus_band(2.0, 0.5, -1.1, 0.3, full)));
    v.push(("torus band(0.0,PI)".into(), torus_band(2.0, 0.5, 0.0, PI - 1e-6, full)));
    for t in [FRAC_PI_2, PI] {
        v.push((
            format!("torus band wedge({t})"),
            torus_band(2.0, 0.5, 0.4, 1.9, Revolution::Partial(t)),
        ));
    }
    for t in [FRAC_PI_2, PI, PI + 0.5, TAU - 0.05] {
        v.push((format!("axis wedge({t})"), axis_wedge(t)));
    }
    for t in [FRAC_PI_2, PI + 0.05, TAU - 0.05] {
        v.push((format!("mirror nappe({t})"), mirror_nappe(t)));
    }
    v.push(("rounded prism".into(), rounded_prism(0.5, 1.0)));
    v.push(("rounded prism tall".into(), rounded_prism(0.13, 7.3)));
    v.push(("die pip".into(), die_pip()));

    let mut out = Vec::new();
    for (name, b) in v {
        match b {
            Some(b) => out.push((name, b)),
            None => println!("  [skip] {name}: construction refused"),
        }
    }
    out
}

fn main() {
    let deltas = [0.25, 0.04, 0.005];
    let mut refusals = Vec::new();
    let mut n = 0usize;
    println!("building fixtures...");
    let fx = fixtures();
    println!("{} fixtures x {} maps x {} deltas", fx.len(), maps().len(), deltas.len());
    for (name, body) in &fx {
        for (mname, map) in maps() {
            let placed = match topo::transform_rigid(body, &map) {
                Ok(b) => b,
                Err(e) => {
                    println!("  [skip] {name} @ {mname}: transform refused {e:?}");
                    continue;
                }
            };
            for d in deltas {
                n += 1;
                match tessellate(&placed, d) {
                    Ok(m) => {
                        if let Err(e) = check_mesh(&m) {
                            println!("  DIRTY  {name} @ {mname} d={d}: {e:?}");
                        }
                    }
                    Err(e) => {
                        let s = format!("{e:?}");
                        if s.contains("UnsupportedCurvedDomain") {
                            println!("  *** REFUSED {name} @ {mname} d={d}: {s}");
                            refusals.push(format!("{name} @ {mname} d={d}: {s}"));
                        } else {
                            println!("  (other) {name} @ {mname} d={d}: {s}");
                        }
                    }
                }
            }
        }
    }
    println!("\n{n} tessellate calls; {} UnsupportedCurvedDomain refusals", refusals.len());
    for r in &refusals {
        println!("  {r}");
    }
}
