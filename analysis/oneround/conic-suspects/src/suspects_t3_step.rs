//! SCRATCH (oneround/conic-suspects-probe, never merges). Task 3, the STEP
//! door: `write <gap> <file>` builds frustum ∩ tilted box (public revolve +
//! boolean) at this process's ε and exports it; `read <file>` imports a file
//! at this process's ε. Dropped into `crates/step-import/examples/`.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom::{Conic, Curve3, CurveDatum, DatumValue};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use topo::{AtRestBody, Body, BooleanResult};

fn trim(e: &impl core::fmt::Debug, n: usize) -> String {
    format!("{e:?}").chars().filter(|c| !c.is_whitespace()).take(n).collect()
}

fn ellipses(body: &Body<f64>) -> String {
    let mut out = Vec::new();
    for (_, g) in body.curves() {
        let Some(curve) = g.certified() else { continue };
        let Curve3::Ellipse(e) = curve.carrier() else { continue };
        let held = match curve.carrier().data().named(CurveDatum::Anchor) { Some(DatumValue::Point(_)) => "arc", _ => "centre" };
        let (a, b) = e.semi_axes();
        out.push(format!("{{\"held\":\"{held}\",\"a\":{a:?},\"b\":{b:?}}}"));
    }
    format!("[{}]", out.join(","))
}

fn volume(body: &AtRestBody<f64>, tol: Tol) -> String {
    match topo::mass_properties(body, tol, topo::test_support::pinned_resolution(tol)) {
        Ok(m) => match m.met() { Ok(p) => format!("{:?}", p.volume), Err(e) => format!("\"unmet {}\"", trim(&e, 80)) },
        Err(e) => format!("\"err {}\"", trim(&e, 160)),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let tol = Tol::witness();
    match args[1].as_str() {
        "write" => {
            let gap: f64 = args[2].parse().unwrap();
            let alpha: f64 = 1.0;
            let (h, r1) = (1.0, 0.5);
            let r0 = r1 + h * alpha.tan();
            let loop_ = ProfileLoop::polygon([(0.0, 0.0), (r0, 0.0), (r1, h), (0.0, h)].map(|(x, y)| Point2::new(x, y)));
            let profile = Profile::new(SketchPlane::xy(), vec![loop_]).validate(tol).unwrap();
            let axis = sweep::RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) };
            let mut frustum = sweep::revolve(&profile, axis, sweep::Revolution::Full, tol).unwrap().body.into_body();
            frustum.merge_coplanar_faces(tol).unwrap();
            let frustum = topo::finish(frustum, tol).unwrap();
            let phi = (1.0 - gap) * (core::f64::consts::FRAC_PI_2 - alpha);
            let q0 = Point3::new(0.0, 0.5 * h, 0.0);
            let big = 4.0 * (r0 + h);
            let turn = Affine3::rotation_about_axis(q0, Vec3::new(0.0, 0.0, 1.0), -phi);
            let slab = topo::test_support::brick::<f64>((-big, big), (q0.y, q0.y + big), (-big, big), tol);
            let slab = topo::finish(topo::transform_rigid(&slab, &geom_core::test_support::rigid(&turn, tol)).unwrap(), tol).unwrap();
            match topo::intersect(&frustum, &slab, tol) {
                Ok(BooleanResult::Body(b)) => {
                    let text = step_export::step_string(&b.body, &step_export::StepOptions::default(), tol);
                    match text {
                        Ok(t) => { std::fs::write(&args[3], t).unwrap(); println!("{{\"row\":\"write\",\"eps\":{:?},\"gap\":{gap:?},\"kind\":\"written\",\"ellipses\":{},\"vol\":{}}}", tol.eps(), ellipses(&b.body), volume(&b.body, tol)); }
                        Err(e) => println!("{{\"row\":\"write\",\"eps\":{:?},\"gap\":{gap:?},\"kind\":\"export-refused\",\"why\":{:?}}}", tol.eps(), trim(&e, 300)),
                    }
                }
                Ok(BooleanResult::Empty) => println!("{{\"row\":\"write\",\"gap\":{gap:?},\"kind\":\"empty\"}}"),
                Err(e) => println!("{{\"row\":\"write\",\"eps\":{:?},\"gap\":{gap:?},\"kind\":\"boolean-refused\",\"why\":{:?}}}", tol.eps(), trim(&e, 300)),
            }
        }
        _ => {
            let text = std::fs::read_to_string(&args[2]).unwrap();
            match step_import::import_step(&text, &step_import::ImportOptions::default(), tol) {
                Ok(step_import::StepImport::Solid { body, eps_in, .. }) => println!("{{\"row\":\"read\",\"eps\":{:?},\"file\":{:?},\"kind\":\"imported\",\"eps_in\":{eps_in:?},\"ellipses\":{},\"vol\":{}}}", tol.eps(), args[2], ellipses(&body), volume(&body, tol)),
                Ok(_) => println!("{{\"row\":\"read\",\"file\":{:?},\"kind\":\"not-a-solid\"}}", args[2]),
                Err(e) => println!("{{\"row\":\"read\",\"eps\":{:?},\"file\":{:?},\"kind\":\"refused\",\"why\":{:?}}}", tol.eps(), args[2], trim(&e, 500)),
            }
        }
    }
}
