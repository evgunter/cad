#![allow(
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::panic,
    dead_code,
    unused_imports
)]
//! Reviewer e2e probe for PR 4573: public doors producing sphere faces,
//! held to tier 3 and the mass properties against closed forms.

use geom_core::{Affine3, Point2, Tol, Vec3};
use std::f64::consts::PI;
use sweep::Revolution;
use sweep::test_support::{ball_poled, brick, finished, revolved_about_y};

fn cap_volume(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}
fn lens(r1: f64, r2: f64, d: f64) -> f64 {
    if d >= r1 + r2 {
        return 0.0;
    }
    PI * (r1 + r2 - d).powi(2)
        * (d * d + 2.0 * d * r2 - 3.0 * r2 * r2 + 2.0 * d * r1 + 6.0 * r1 * r2 - 3.0 * r1 * r1)
        / (12.0 * d)
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn dir(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
            );
            let n = v.norm();
            if n > 0.1 && n < 1.0 {
                return v / n;
            }
        }
    }
}
/// Flip every sphere face (alone, and every pair on one surface): never a
/// silent wrong volume (tier 3 green AND mass properties Ok).
fn flips(label: &str, body: &topo::Body<f64>, stats: &mut [usize; 4], bad: &mut Vec<String>) {
    let tol = Tol::witness();
    if let Err(e) = topo::validate_geometric(body, tol) {
        eprintln!("{label}: UNFLIPPED REFUSED {e:?}");
    }
    let sph: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Sphere { .. })
            )
        })
        .map(|(k, f)| (k, f.surface))
        .collect();
    let mut sets: Vec<Vec<topo::FaceKey>> = sph.iter().map(|(k, _)| vec![*k]).collect();
    for i in 0..sph.len() {
        for j in i + 1..sph.len() {
            if sph[i].1 == sph[j].1 {
                sets.push(vec![sph[i].0, sph[j].0]);
            }
        }
    }
    for set in sets {
        let fb = set.iter().fold(body.clone(), |b, &k| {
            b.flipped_face_sense_for_tests(k).unwrap()
        });
        let g = topo::validate_geometric(&fb, tol);
        let m = topo::mass_properties(&fb, tol);
        match (&g, &m) {
            (Ok(()), Ok(p)) => {
                stats[3] += 1;
                let band = geom_core::Band::linear(tol).unwrap();
                let sides: Vec<String> = set
                    .iter()
                    .map(|&k| {
                        let f = fb.faces().find(|(kk, _)| *kk == k).unwrap().1;
                        let (outer, _) = topo::props::loop_edges(&fb, f.outer).unwrap();
                        let surf = fb.get_surface(f.surface).unwrap();
                        format!(
                            "{:?}/rings{}/edges{}",
                            geom_brep::props::boundary_material_sign(surf, &outer, band),
                            f.rings.len(),
                            outer.len()
                        )
                    })
                    .collect();
                let v0 = topo::mass_properties(body, tol).map(|p| p.volume);
                bad.push(format!(
                    "{label} flip {set:?}: SILENT volume {} (true {:?}) sides {sides:?}",
                    p.volume, v0
                ));
            }
            (Err(e), _)
                if e.iter()
                    .any(|x| matches!(x, topo::ValidationError::CurvedSenseInverted { .. })) =>
            {
                stats[0] += 1
            }
            (Err(e), _) => {
                eprintln!(
                    "{label} flip {set:?}: {:?}",
                    e.iter()
                        .map(|x| format!("{x:?}").chars().take(60).collect::<String>())
                        .collect::<Vec<_>>()
                );
                stats[1] += 1
            }
            (Ok(()), Err(_)) => stats[2] += 1,
        }
    }
}

#[test]
fn review_4573_flips() {
    let tol = Tol::witness();
    let mut g = Rng(0xFEEDFACE);
    let mut bad = Vec::new();
    let mut st = [0usize; 4];
    for i in 0..25 {
        let rb = g.range(0.5, 1.2);
        let d = g.range(0.4, 1.0 + rb - 0.2);
        let off = g.dir() * d;
        let pa = g.dir();
        let pb = g.dir();
        eprintln!(
            "PARAM #{i} rb={rb:?} off=({:?},{:?},{:?}) pa=({:?},{:?},{:?}) pb=({:?},{:?},{:?})",
            off.x, off.y, off.z, pa.x, pa.y, pa.z, pb.x, pb.y, pb.z
        );
        let a = finished("a", ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), pa, tol), tol);
        let b = finished("b", ball_poled(rb, off, pb, tol), tol);
        for (name, r) in [
            ("union", topo::boolean::union(&a, &b, tol)),
            ("inter", topo::boolean::intersect(&a, &b, tol)),
            ("a-b", topo::boolean::subtract(&a, &b, tol)),
        ] {
            if let Ok(res) = r {
                if let Some(bb) = res.body() {
                    flips(&format!("#{i} {name}"), &bb.body, &mut st, &mut bad);
                }
            }
        }
    }
    for i in 0..15 {
        let r = g.range(0.3, 0.9);
        let dip = g.range(0.05, 0.9 * r);
        let c = Vec3::new(g.range(1.2, 2.8), g.range(1.2, 2.8), 1.0 + r - dip);
        let slab = finished("slab", brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol), tol);
        let ball = finished(
            "ball",
            ball_poled(
                r,
                c,
                Vec3::new(0.0, 0.0, if i % 2 == 0 { 1.0 } else { -1.0 }),
                tol,
            ),
            tol,
        );
        for (name, res) in [
            ("slab-ball", topo::boolean::subtract(&slab, &ball, tol)),
            ("slab+ball", topo::boolean::union(&slab, &ball, tol)),
            ("slab^ball", topo::boolean::intersect(&slab, &ball, tol)),
            ("ball-slab", topo::boolean::subtract(&ball, &slab, tol)),
        ] {
            if let Ok(rr) = res {
                if let Some(bb) = rr.body() {
                    flips(&format!("d#{i} {name}"), &bb.body, &mut st, &mut bad);
                }
            }
        }
    }
    eprintln!(
        "flips: check6 {} other-tier3 {} massprops-refuse {} SILENT {}",
        st[0], st[1], st[2], st[3]
    );
    for b in bad.iter().take(20) {
        eprintln!("  {b}");
    }
}

#[test]
fn probe_half_ball_through_axis() {
    let tol = Tol::witness();
    let mut st = [0usize; 4];
    let mut bad = Vec::new();
    for th in [0.0, 0.3, 1.0, 2.0, 2.9] {
        let ball = topo::transform_rigid(
            &ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), tol),
            &Affine3::rotation_about_axis(
                geom_core::Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                th,
            ),
            tol,
        )
        .unwrap();
        let a = finished("a", ball, tol);
        let b = finished("b", brick((0.0, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol), tol);
        for (name, r) in [
            ("inter", topo::boolean::intersect(&a, &b, tol)),
            ("a-b", topo::boolean::subtract(&a, &b, tol)),
            (
                "a+brick",
                topo::boolean::union(
                    &a,
                    &finished("c", brick((-3.0, 0.0), (-2.0, 2.0), (-2.0, 2.0), tol), tol),
                    tol,
                ),
            ),
            (
                "a+thin",
                topo::boolean::union(
                    &a,
                    &finished("c", brick((-0.5, 0.0), (-0.9, 0.9), (-0.9, 0.9), tol), tol),
                    tol,
                ),
            ),
        ] {
            match r {
                Ok(res) => {
                    if let Some(bb) = res.body() {
                        eprintln!("th {th} {name}: faces {}", bb.body.faces().count());
                        flips(&format!("th{th} {name}"), &bb.body, &mut st, &mut bad);
                    }
                }
                Err(e) => eprintln!("th {th} {name}: {e:?}"),
            }
        }
    }
    eprintln!(
        "half-ball flips: check6 {} other-tier3 {} massprops-refuse {} SILENT {}",
        st[0], st[1], st[2], st[3]
    );
    for b in &bad {
        eprintln!("  {b}");
    }
}
