//! Reviewer e2e probe for PR 4573: public doors producing sphere faces,
//! held to tier 3 and the mass properties against closed forms.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::panic)]

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

fn report(label: &str, body: &topo::Body<f64>, want: f64, out: &mut Vec<String>) -> bool {
    let tol = Tol::witness();
    let g = topo::validate_geometric(body, tol);
    let m = topo::mass_properties(body, tol);
    match (&g, &m) {
        (Ok(()), Ok(p)) if (p.volume - want).abs() <= 1e-9 * want.abs().max(1e-3) => true,
        _ => {
            out.push(format!(
                "{label}: geo {g:?} mass {:?} want {want}",
                m.map(|p| p.volume)
            ));
            false
        }
    }
}

#[test]
fn review_4573_ball_pairs() {
    let tol = Tol::witness();
    let mut g = Rng(0xDEADBEEF12345);
    let mut bad = Vec::new();
    let mut ok = 0;
    let mut built = 0;
    for i in 0..40 {
        let ra = 1.0;
        let rb = g.range(0.5, 1.2);
        let d = g.range(0.4, ra + rb - 0.2);
        let off = g.dir() * d;
        let (pa, pb) = if i % 2 == 0 {
            (g.dir(), g.dir())
        } else {
            (Vec3::new(0.0, 1.0, 0.0), g.dir())
        };
        let a = finished("a", ball_poled(ra, Vec3::new(0.0, 0.0, 0.0), pa, tol), tol);
        let b = finished("b", ball_poled(rb, off, pb, tol), tol);
        let va = 4.0 / 3.0 * PI * ra.powi(3);
        let vb = 4.0 / 3.0 * PI * rb.powi(3);
        let sh = lens(ra, rb, d);
        for (name, r, want) in [
            ("union", topo::boolean::union(&a, &b, tol), va + vb - sh),
            ("inter", topo::boolean::intersect(&a, &b, tol), sh),
            ("a-b", topo::boolean::subtract(&a, &b, tol), va - sh),
        ] {
            let label = format!("#{i} {name} rb={rb:.3} d={d:.3}");
            match r {
                Ok(res) => match res.body() {
                    Some(bb) => {
                        built += 1;
                        if report(&label, &bb.body, want, &mut bad) {
                            ok += 1;
                        }
                    }
                    None => bad.push(format!("{label}: no body")),
                },
                Err(e) => bad.push(format!("{label}: boolean refused {e:?}")),
            }
        }
    }
    eprintln!("ball pairs: built {built} ok {ok} bad {}", bad.len());
    for b in &bad {
        eprintln!("  {b}");
    }
}

#[test]
fn review_4573_revolves() {
    let tol = Tol::witness();
    let mut bad = Vec::new();
    // Quarter disc → hemisphere solid (pole a vertex on the axis).
    let b45 = (PI / 8.0).tan();
    let r = 0.7;
    let hemi = vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(r, 0.0), (PI / 8.0).tan()),
        (Point2::new(0.0, r), 0.0),
    ];
    let body = revolved_about_y(hemi.clone(), Revolution::Full, tol);
    report(
        "hemisphere full",
        &body,
        2.0 / 3.0 * PI * r.powi(3),
        &mut bad,
    );
    for th in [0.3, 1.0, PI / 2.0, 2.0, PI, 4.0, 5.5, 6.2] {
        let body = revolved_about_y(hemi.clone(), Revolution::Partial(th), tol);
        report(
            &format!("hemisphere wedge {th}"),
            &body,
            th / 3.0 * r.powi(3),
            &mut bad,
        );
        let half = vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)];
        let body = revolved_about_y(half, Revolution::Partial(th), tol);
        report(
            &format!("ball wedge {th}"),
            &body,
            th * 2.0 / 3.0 * r.powi(3),
            &mut bad,
        );
        let body = revolved_about_y(hemi.clone(), Revolution::Partial(-th), tol);
        report(
            &format!("hemisphere wedge -{th}"),
            &body,
            th / 3.0 * r.powi(3),
            &mut bad,
        );
    }
    let _ = b45;
    eprintln!("revolves bad {}", bad.len());
    for b in &bad {
        eprintln!("  {b}");
    }
}

#[test]
fn review_4573_slab_dimples() {
    let tol = Tol::witness();
    let mut g = Rng(0x1234567);
    let mut bad = Vec::new();
    let mut ok = 0;
    for i in 0..30 {
        let r = g.range(0.3, 0.9);
        let dip = g.range(0.05, 0.9 * r);
        let c = Vec3::new(g.range(1.2, 2.8), g.range(1.2, 2.8), 1.0 + r - dip);
        // pole: along z (the boolean's polar frontier), sometimes flipped
        let pole = if i % 2 == 0 {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(0.0, 0.0, -1.0)
        };
        let slab = finished("slab", brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol), tol);
        let ball = finished("ball", ball_poled(r, c, pole, tol), tol);
        let cap = cap_volume(r, dip);
        let vball = 4.0 / 3.0 * PI * r.powi(3);
        for (name, res, want) in [
            (
                "slab-ball",
                topo::boolean::subtract(&slab, &ball, tol),
                16.0 - cap,
            ),
            (
                "slab+ball",
                topo::boolean::union(&slab, &ball, tol),
                16.0 + vball - cap,
            ),
            (
                "slab^ball",
                topo::boolean::intersect(&slab, &ball, tol),
                cap,
            ),
            (
                "ball-slab",
                topo::boolean::subtract(&ball, &slab, tol),
                vball - cap,
            ),
        ] {
            let label = format!("#{i} {name} r={r:.3} dip={dip:.3} pole={:?}", pole.z);
            match res {
                Ok(rr) => match rr.body() {
                    Some(bb) => {
                        if report(&label, &bb.body, want, &mut bad) {
                            ok += 1;
                        }
                    }
                    None => bad.push(format!("{label}: no body")),
                },
                Err(e) => bad.push(format!("{label}: boolean refused {e:?}")),
            }
        }
    }
    eprintln!("dimples ok {ok} bad {}", bad.len());
    for b in &bad {
        eprintln!("  {b}");
    }
    let _ = Affine3::<f64>::translation(Vec3::new(0.0, 0.0, 0.0));
}

/// Flip every sphere face (alone, and every pair on one surface): never a
/// silent wrong volume (tier 3 green AND mass properties Ok).
fn flips(label: &str, body: &topo::Body<f64>, stats: &mut [usize; 4], bad: &mut Vec<String>) {
    let tol = Tol::witness();
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
            (Err(_), _) => stats[1] += 1,
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
        let a = finished(
            "a",
            ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), g.dir(), tol),
            tol,
        );
        let b = finished("b", ball_poled(rb, off, g.dir(), tol), tol);
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

mod dimple_copy {
    use super::cap_volume;
    use geom_core::{Affine3, Tol, Vec3};
    use sweep::test_support::{ball_poled_y, brick, finished};
    use topo::AtRestBody;
    const R: f64 = 0.6;
    const C: (f64, f64, f64) = (2.0, 2.0, 1.3);

    fn dslab() -> AtRestBody<f64> {
        let tol = Tol::witness();
        finished(
            "the slab",
            brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol),
            tol,
        )
    }

    fn dball() -> AtRestBody<f64> {
        let tol = Tol::witness();
        let at = ball_poled_y(R, Vec3::new(0.0, 0.0, 0.0), tol);
        let moved =
            topo::transform_rigid(&at, &Affine3::translation(Vec3::new(C.0, C.1, C.2)), tol)
                .unwrap();
        finished("the ball", moved, tol)
    }

    #[test]
    fn review_4573_dimple_copy() {
        let tol = Tol::witness();
        let none = topo::BooleanDeclarations::none();
        let body = topo::subtract_with(&dslab(), &dball(), &none, tol)
            .expect("the slab less the ball builds")
            .body()
            .expect("a body")
            .body
            .clone();
        topo::validate_geometric(&body, tol).expect("the dimpled slab is valid");
        let volume = 16.0 - cap_volume(R, 1.0 - (C.2 - R));
        let v = topo::mass_properties(&body, tol).unwrap().volume;
        assert!(
            (v - volume).abs() <= 1e-9 * volume,
            "volume {v} against the closed form {volume}"
        );
        let dimple: Vec<_> = body
            .faces()
            .filter(|(_, f)| {
                matches!(
                    body.get_surface(f.surface),
                    Some(geom::Surface::Sphere { .. })
                )
            })
            .map(|(k, _)| k)
            .collect();
        assert_eq!(dimple.len(), 2, "the dimple is two half-faces on one seam");
        let flipped = dimple.iter().fold(topo::Body::clone(&body), |b, &k| {
            b.flipped_face_sense_for_tests(k).expect("the face")
        });
        assert_eq!(
            topo::validate_geometric(&flipped, tol),
            Err(dimple
                .iter()
                .map(|&face| topo::ValidationError::CurvedSenseInverted { face })
                .collect()),
            "check 6 names both flipped faces, and nothing else does"
        );
        assert!(
            matches!(
                topo::mass_properties(&flipped, tol),
                Err(topo::MassPropsError::Face {
                    face,
                    source: geom_brep::props::PropsError::SenseContradicted,
                }) if dimple.contains(&face)
            ),
            "the flipped dimple is refused, not measured as the complement"
        );
    }
}

#[test]
fn review_4573_zones() {
    let tol = Tol::witness();
    let mut bad = Vec::new();
    let mut st = [0usize; 4];
    for bore in [0.3, 1.0, 1.6] {
        let full = PI * (5.625 - 1.5 * bore * bore);
        for (rev, frac) in [
            (Revolution::Full, 1.0),
            (Revolution::Partial(1.0), 1.0 / (2.0 * PI)),
            (Revolution::Partial(PI), 0.5),
            (Revolution::Partial(5.0), 5.0 / (2.0 * PI)),
            (Revolution::Partial(-4.0), 4.0 / (2.0 * PI)),
        ] {
            let b = sweep::test_support::sphere_zone(bore, rev, tol);
            let label = format!("zone bore {bore} frac {frac:.3}");
            report(&label, &b, full * frac, &mut bad);
            flips(&label, &b, &mut st, &mut bad);
        }
    }
    let l = sweep::test_support::lantern(tol);
    let vl = topo::mass_properties(&l, tol).map(|p| p.volume);
    let lp = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), (0.6f64.asin() / 4.0).tan()),
            (Point2::new(0.8, 0.6), 0.0),
            (Point2::new(0.2, 1.2), 0.0),
            (Point2::new(0.0, 1.2), 0.0),
        ],
        Revolution::Partial(2.5),
        tol,
    );
    let vp = topo::mass_properties(&lp, tol).map(|p| p.volume * 2.0 * PI / 2.5);
    eprintln!(
        "lantern full {vl:?} partial-scaled {vp:?} geo {:?}",
        topo::validate_geometric(&l, tol)
    );
    flips("lantern", &l, &mut st, &mut bad);
    eprintln!(
        "zones: check6 {} other {} mp {} SILENT {}",
        st[0], st[1], st[2], st[3]
    );
    for b in &bad {
        eprintln!("  {b}");
    }
}

#[test]
fn review_4573_chained() {
    let tol = Tol::witness();
    let mut g = Rng(0xC0FFEE);
    let (mut ok, mut refused, mut wrong) = (0, 0, 0);
    for i in 0..30 {
        let ca = Vec3::new(0.0, 0.0, 0.0);
        let cb = g.dir() * g.range(0.6, 1.4);
        let cc = g.dir() * g.range(0.3, 1.5);
        let (rb, rc) = (g.range(0.6, 1.1), g.range(0.4, 0.9));
        let a = finished("a", ball_poled(1.0, ca, g.dir(), tol), tol);
        let b = finished("b", ball_poled(rb, cb, g.dir(), tol), tol);
        let c = finished("c", ball_poled(rc, cc, g.dir(), tol), tol);
        let Ok(u) = topo::boolean::union(&a, &b, tol) else {
            refused += 1;
            continue;
        };
        let Some(u) = u.body() else { continue };
        let u = u.body.clone();
        for (name, inside) in [("sub", 0), ("int", 1)] {
            let r = if inside == 0 {
                topo::boolean::subtract(&u, &c, tol)
            } else {
                topo::boolean::intersect(&u, &c, tol)
            };
            let body = match r {
                Ok(r) => match r.body() {
                    Some(b) => b.body.clone(),
                    None => {
                        eprintln!("#{i} {name} empty");
                        continue;
                    }
                },
                Err(e) => {
                    eprintln!("#{i} {name} refused {e:?}");
                    refused += 1;
                    continue;
                }
            };
            let geo = topo::validate_geometric(&body, tol);
            let v = topo::mass_properties(&body, tol).map(|p| p.volume);
            // Monte Carlo reference
            let mut rng = Rng(i as u64 * 7919 + 13);
            let n = 400_000;
            let (lo, hi) = (-2.6, 2.6);
            let mut hit = 0;
            for _ in 0..n {
                let p = Vec3::new(rng.range(lo, hi), rng.range(lo, hi), rng.range(lo, hi));
                let ina = (p - ca).norm() < 1.0;
                let inb = (p - cb).norm() < rb;
                let inc = (p - cc).norm() < rc;
                let inu = ina || inb;
                if (inside == 0 && inu && !inc) || (inside == 1 && inu && inc) {
                    hit += 1;
                }
            }
            let mc = (hi - lo as f64).powi(3) * hit as f64 / n as f64;
            let sd = (hi - lo as f64).powi(3) * ((hit as f64).sqrt() + 1.0) / n as f64;
            match (&geo, &v) {
                (Ok(()), Ok(v)) if (v - mc).abs() < 5.0 * sd + 1e-3 => ok += 1,
                (Ok(()), Ok(v)) => {
                    wrong += 1;
                    eprintln!("#{i} {name}: WRONG volume {v} mc {mc} ± {sd}");
                }
                _ => {
                    refused += 1;
                    eprintln!("#{i} {name}: geo {geo:?} v {v:?}");
                }
            }
        }
    }
    eprintln!("chained ok {ok} refused {refused} wrong {wrong}");
}
