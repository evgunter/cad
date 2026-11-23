//! SCRATCH (oneround/conic-suspects-probe, never merges). Task 3: the
//! public boolean of a frustum against a box whose one face is a plane
//! tilted toward the cone's parabola. Dropped into `crates/sweep/examples/`
//! of a clean checkout of main and run with
//! `cargo run -p sweep --example suspects_t3_boolean -- <alpha> <rot 0|1>`.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom::{Conic, Curve3, CurveDatum, DatumValue, Surface};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::{AtRestBody, Body, BooleanOp, BooleanResult};

fn p3(p: Point3<f64>) -> String { format!("[{:?},{:?},{:?}]", p.x, p.y, p.z) }
fn v3(p: Vec3<f64>) -> String { format!("[{:?},{:?},{:?}]", p.x, p.y, p.z) }

fn trim(e: &impl core::fmt::Debug, n: usize) -> String {
    format!("{e:?}").chars().filter(|c| !c.is_whitespace()).take(n).collect()
}

pub fn dump(body: &Body<f64>) -> String {
    let mut curves = Vec::new();
    for (_, g) in body.curves() {
        let Some(curve) = g.certified() else { continue };
        let Curve3::Ellipse(e) = curve.carrier() else { continue };
        let c = curve.carrier();
        let (t0, t1) = curve.params();
        let (a, b) = e.semi_axes();
        let pt = |d| match c.data().named(d) { Some(DatumValue::Point(p)) => Some(p), _ => None };
        let phase = match c.data().named(CurveDatum::Phase) { Some(DatumValue::Direction(d)) => format!("[{:?},{:?}]", d.x, d.y), _ => "null".into() };
        let (held, s) = match (pt(CurveDatum::Anchor), pt(CurveDatum::Center)) {
            (Some(p), _) => ("arc", p),
            (_, Some(p)) => ("centre", p),
            _ => ("?", Point3::new(f64::NAN, f64::NAN, f64::NAN)),
        };
        curves.push(format!(
            "{{\"held\":\"{held}\",\"S\":{},\"axis\":{},\"u\":{},\"a\":{a:?},\"b\":{b:?},\"phase\":{phase},\"span\":[{t0:?},{t1:?}],\"p0\":{},\"p1\":{}}}",
            p3(s), v3(e.frame().axis()), v3(e.frame().u_ref()), p3(e.point_at(t0)), p3(e.point_at(t1))
        ));
    }
    let verts: Vec<String> = body.vertex_points().map(|(_, p)| p3(p)).collect();
    let mut surfs = Vec::new();
    for (_, s) in body.surfaces() {
        match s {
            Surface::Plane { frame } => surfs.push(format!("{{\"k\":\"plane\",\"o\":{},\"n\":{}}}", p3(frame.origin()), v3(frame.axis()))),
            Surface::Cone { frame, half_angle } => surfs.push(format!("{{\"k\":\"cone\",\"o\":{},\"n\":{},\"alpha\":{half_angle:?}}}", p3(frame.origin()), v3(frame.axis()))),
            _ => surfs.push("{\"k\":\"other\"}".into()),
        }
    }
    format!("{{\"faces\":{},\"ellipses\":[{}],\"verts\":[{}],\"surfs\":[{}]}}", body.faces().count(), curves.join(","), verts.join(","), surfs.join(","))
}

fn volume(body: &AtRestBody<f64>, tol: Tol) -> String {
    match topo::mass_properties(body, tol, topo::test_support::pinned_resolution(tol)) {
        Ok(m) => match m.met() { Ok(p) => format!("{{\"v\":{:?},\"pad\":{:?}}}", p.volume, p.volume_pad), Err(e) => format!("{{\"unmet\":{:?}}}", trim(&e, 100)) },
        Err(e) => format!("{{\"err\":{:?}}}", trim(&e, 160)),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let alpha: f64 = args.get(1).map_or(1.0, |s| s.parse().unwrap());
    let rot = args.get(2).is_some_and(|s| s == "1");
    let tol = Tol::witness();
    let m = if rot {
        Affine3::rotation_about_axis(Point3::new(0.37, -0.21, 0.13), Vec3::new(0.3, 0.5, 0.8).normalize(), 0.7)
    } else {
        Affine3::identity()
    };
    // arg 3: the frustum's scale S (height S, top radius S/2); arg 4: a fixed tilt gap (else the ladder)
    let scale: f64 = args.get(3).map_or(1.0, |s| s.parse().unwrap());
    let fixed: Option<f64> = args.get(4).map(|s| s.parse().unwrap());
    let (h, r1) = (scale, 0.5 * scale);
    let r0 = r1 + h * alpha.tan();
    let finish = |b: Body<f64>| -> Result<AtRestBody<f64>, String> {
        let moved = if rot { topo::transform_rigid(&b, &geom_core::test_support::rigid(&m, tol)).map_err(|e| trim(&e, 200))? } else { b };
        topo::finish(moved, tol).map_err(|e| trim(&e, 200))
    };
    let frustum = finish(sweep::test_support::revolved_about_y(
        vec![(Point2::new(0.0, 0.0), 0.0), (Point2::new(r0, 0.0), 0.0), (Point2::new(r1, h), 0.0), (Point2::new(0.0, h), 0.0)],
        sweep::Revolution::Full,
        tol,
    ))
    .unwrap();
    println!("{{\"row\":\"frustum\",\"alpha\":{alpha:?},\"rot\":{rot},\"eps\":{:?},\"body\":{},\"vol\":{}}}", tol.eps(), dump(&frustum), volume(&frustum, tol));
    let q0 = Point3::new(0.0, 0.5 * h, 0.0);
    let big = if scale > 1.0 { 1.5 * (r0 + h) } else { 4.0 * (r0 + h) };
    for k in 2..=11 {
        for lead in [1.0, 1.5, 2.0, 3.0, 5.0, 7.0] {
            let gap: f64 = match fixed { Some(g) if k == 2 && lead == 1.0 => g, Some(_) => continue, None => lead * 10f64.powi(-k) };
            let f = 1.0 - gap;
            let phi = f * (core::f64::consts::FRAC_PI_2 - alpha);
            // the box y in [q.y, q.y + big], turned about z through q so its
            // face y = q.y becomes the plane through q of normal (sin phi, cos phi, 0)
            let turn = Affine3::rotation_about_axis(q0, Vec3::new(0.0, 0.0, 1.0), -phi);
            let slab = topo::test_support::brick::<f64>((-big, big), (q0.y, q0.y + big), (-big, big), tol);
            let slab = match topo::transform_rigid(&slab, &geom_core::test_support::rigid(&turn, tol)).map_err(|e| trim(&e, 200)).and_then(|b| finish(b)) {
                Ok(b) => b,
                Err(e) => { println!("{{\"row\":\"slab-refused\",\"gap\":{gap:?},\"why\":{e:?}}}"); continue; }
            };
            let n = m.transform_vec(Vec3::new(phi.sin(), phi.cos(), 0.0));
            let q = m.transform_point(q0);
            // the split door on the same plane
            {
                let head = format!("\"row\":\"bool\",\"scale\":{scale:?},\"alpha\":{alpha:?},\"rot\":{rot},\"eps\":{:?},\"gap\":{gap:?},\"phi\":{phi:?},\"q\":{},\"n\":{},\"op\":\"split\"", tol.eps(), p3(q), v3(n));
                match topo::splitting::split(&frustum, &topo::test_support::split_plane(q, n / n.norm(), tol), tol) {
                    Ok(r) => {
                        let parts: Vec<String> = [&r.above, &r.below].iter().filter_map(|h| h.body()).map(|b| dump(b)).collect();
                        println!("{{{head},\"kind\":\"built\",\"how\":\"split\",\"body\":{},\"vol\":{{}},\"parts\":{}}}", parts.first().cloned().unwrap_or("{\"faces\":0,\"ellipses\":[],\"verts\":[],\"surfs\":[]}".into()), parts.len());
                    }
                    Err(e) => println!("{{{head},\"kind\":\"refused\",\"why\":{:?}}}", trim(&e, 400)),
                }
            }
            for (name, op) in [("intersect", BooleanOp::Intersect), ("subtract", BooleanOp::Subtract), ("union", BooleanOp::Union)] {
                let got = match op {
                    BooleanOp::Intersect => topo::intersect(&frustum, &slab, tol),
                    BooleanOp::Subtract => topo::subtract(&frustum, &slab, tol),
                    _ => topo::union(&frustum, &slab, tol),
                };
                let head = format!("\"row\":\"bool\",\"scale\":{scale:?},\"alpha\":{alpha:?},\"rot\":{rot},\"eps\":{:?},\"gap\":{gap:?},\"phi\":{phi:?},\"q\":{},\"n\":{},\"op\":\"{name}\"", tol.eps(), p3(q), v3(n));
                match got {
                    Ok(BooleanResult::Body(b)) => println!("{{{head},\"kind\":\"built\",\"how\":\"{:?}\",\"body\":{},\"vol\":{}}}", b.kind, dump(&b.body), volume(&b.body, tol)),
                    Ok(BooleanResult::Empty) => println!("{{{head},\"kind\":\"empty\"}}"),
                    Err(e) => println!("{{{head},\"kind\":\"refused\",\"why\":{:?}}}", trim(&e, 400)),
                }
            }
        }
    }
}
