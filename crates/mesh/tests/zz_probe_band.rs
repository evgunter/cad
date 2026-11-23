#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Revolution, RevolveAxis, revolve};

fn band(hole: f64, r: f64, bulge: f64) -> topo::Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(hole, r), bulge),
        (Point2::new(hole + r, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(hole, 1.0), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()).unwrap();
    revolve(&vp, RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) }, Revolution::Full, Tol::witness()).unwrap().body.into_body()
}

#[test]
#[ignore]
fn zz_probe() {
    let b: f64 = std::env::var("B").map(|s| s.parse().unwrap()).unwrap_or(-(std::f64::consts::PI / 8.0).tan());
    let hole: f64 = std::env::var("DBG_H").map(|s| s.parse().unwrap()).unwrap_or(0.3);
    for r in [0.0005, 0.001, 0.002, 0.005, 0.01, 0.02, 0.05] {
        let body = band(hole, r, b);
        if r == 0.005 {
            for (fk, f) in body.faces() { eprintln!("FACE {fk:?} {:?}", body.get_surface(f.surface).map(|s| format!("{s:?}").chars().take(160).collect::<String>())); }
        }
        for d in [3e-2, 1e-2, 3e-3, 1e-3, 3e-4, 1e-4, 3e-5] {
            match mesh::tessellate(&body, d, Tol::witness()) {
                Ok(m) => println!("ROW r={r} d={d:e} ok {}", m.positions.len()),
                Err(e) => println!("ROW r={r} d={d:e} ERR {e:?}"),
            }
        }
    }
}

fn fnv(h: &mut u64, x: u64) {
    for b in x.to_le_bytes() {
        *h ^= u64::from(b);
        *h = h.wrapping_mul(0x0100_0000_01b3);
    }
}

#[test]
#[ignore]
fn zz_patch_digests() {
    use crate::common::*;
    let mut bodies: Vec<(String, topo::Body<f64>)> = vec![
        ("l_prism".into(), l_prism()), ("holed_prism".into(), holed_prism()), ("rounded_prism".into(), rounded_prism()),
        ("ball".into(), ball()), ("cone".into(), cone()), ("cone_wedge".into(), cone_wedge(1.0, 1.2)),
        ("sphere_wedge12".into(), sphere_wedge(1.2)), ("sphere_wedge2".into(), sphere_wedge(2.0)),
        ("sphere_wedge_q".into(), sphere_wedge(std::f64::consts::FRAC_PI_4)),
        ("washer".into(), washer()), ("donut".into(), donut()), ("wedge".into(), wedge()), ("axis_wedge".into(), axis_wedge()),
        ("keyway".into(), witness_bodies::keyway().0), ("slit".into(), witness_bodies::slit().0),
        ("oblique_lens".into(), witness_bodies::oblique_lens().0),
        ("pole_crossing_half_cap".into(), witness_bodies::pole_crossing_half_cap().0),
        ("apex_crossing_bowtie".into(), witness_bodies::apex_crossing_bowtie().0),
    ];
    for r in [0.0005, 0.002, 0.02, 0.05] {
        bodies.push((format!("band{r}"), band(0.3, r, -(std::f64::consts::PI / 8.0).tan())));
    }
    for (name, body) in &bodies {
        for d in [0.15, 0.05, 1e-2, 3e-3, 1e-3, 3e-4] {
            match mesh::tessellate(body, d, Tol::witness()) {
                Ok(m) => {
                    for q in &m.patches {
                        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
                        for t in &q.triangles {
                            for &i in t {
                                let p = m.positions[i as usize];
                                fnv(&mut h, p.x.to_bits()); fnv(&mut h, p.y.to_bits()); fnv(&mut h, p.z.to_bits());
                            }
                        }
                        println!("PD {name} d={d:e} {:?} n={} h={h:016x}", q.face, q.triangles.len());
                    }
                }
                Err(e) => println!("PD {name} d={d:e} ERR {e:?}"),
            }
        }
    }
}
