//! Review 5225 r2 probes: a sphere wedge's south cap, whose meridian
//! edges' midpoints sit below latitude −1.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{axis_y, check_mesh_acceptance, validated};
use geom_core::{Point2, Tol};
use mesh::validate::signed_volume;
use profile::{ProfileLoop, test_support::bulge_loop};
use sweep::{Revolution, revolve};
use profile::RawLoop;

const PI: f64 = core::f64::consts::PI;

fn cap(lat: f64, theta: f64, south: bool) -> topo::Body<f64> {
    let s = if south { -1.0 } else { 1.0 };
    let (c, sn) = (lat.cos(), lat.sin());
    let b = ((PI / 2.0 - lat) / 4.0).tan();
    let lp: ProfileLoop<f64> = if south {
        bulge_loop(vec![
            (Point2::new(0.0, -1.0), b),
            (Point2::new(c, -sn), 0.0),
            (Point2::new(0.0, -sn), 0.0),
        ])
    } else {
        bulge_loop(vec![
            (Point2::new(0.0, sn), 0.0),
            (Point2::new(c, sn), b),
            (Point2::new(0.0, 1.0), 0.0),
        ])
    };
    let _ = s;
    revolve(&validated(vec![lp]), axis_y(), Revolution::Partial(theta), Tol::witness())
        .unwrap()
        .body
}

#[test]
fn r2_sphere_caps_mesh() {
    let mut out = vec![];
    for south in [true, false] {
        for lat in [0.3, 0.6] {
            for theta in [1.0, PI, 2.0 * PI - 0.5] {
                let body = cap(lat, theta, south);
                topo::validate(&body).unwrap();
                for (he, pc) in body.pcurves() {
                    let (a, b) = pc.params();
                    let m = pc.pcurve().eval(0.5 * (a + b));
                    eprintln!("south={south} lat={lat} th={theta:.3} {he:?} [{a:.4},{b:.4}] mid {m:?}");
                }
                let h = 1.0 - lat.sin();
                let v_exact = PI * h * h * (3.0 - h) / 3.0 * theta / (2.0 * PI);
                for delta in [0.1, 0.02] {
                    let r = std::panic::catch_unwind(|| {
                        let mesh = check_mesh_acceptance(&body, delta, None);
                        signed_volume(&mesh)
                    });
                    let line = format!("south={south} lat={lat} theta={theta:.3} delta={delta}: {:?} exact {v_exact:.6}", r.as_ref().map(|v| format!("{v:.6}")));
                    eprintln!("{line}");
                    out.push(line);
                }
            }
        }
    }
    for l in &out { eprintln!("SUMMARY {l}"); }
}

#[test]
fn r2_dump_tilted() {
    use std::io::Write;
    let dir = std::env::var("R2_DUMP").unwrap();
    let (a, b) = crate::d9_mesh_goldens::tilted_halves();
    for (name, body) in [("above", a), ("below", b)] {
        for (he, pc) in body.pcurves() {
            let (t0, t1) = pc.params();
            eprintln!("{name} {he:?} [{t0},{t1}] {:?} at t0 {:?} t1 {:?}", pc.pcurve(), pc.pcurve().eval(t0), pc.pcurve().eval(t1));
        }
        for delta in [0.1, 0.01] {
            let m = mesh::tessellate(&body, delta, Tol::witness()).unwrap();
            let mut f = std::fs::File::create(format!("{dir}/{name}_{delta}.txt")).unwrap();
            writeln!(f, "vol {:.17e}", signed_volume(&m)).unwrap();
            for p in &m.positions { writeln!(f, "p {:.17e} {:.17e} {:.17e}", p.x, p.y, p.z).unwrap(); }
            for q in &m.patches { writeln!(f, "patch {:?} {}", q.face, q.triangles.len()).unwrap(); for t in &q.triangles { writeln!(f, "t {} {} {}", t[0], t[1], t[2]).unwrap(); } }
        }
    }
}

fn dome() -> topo::Body<f64> {
    let b = (PI / 8.0).tan();
    let lp: ProfileLoop<f64> = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), b),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

#[test]
fn r2_split_a_single_edge_rim() {
    let base = dome();
    topo::validate(&base).unwrap();
    let rims: Vec<_> = base
        .edges()
        .filter_map(|(e, edge)| {
            let c = base.get_curve_geom(edge.curve)?.certified()?;
            matches!(c.carrier(), geom::Curve3::Circle { .. }).then(|| (e, c.params()))
        })
        .collect();
    eprintln!("rims {rims:?}");
    for (he, pc) in base.pcurves() {
        let (a, b) = pc.params();
        eprintln!("  base {he:?} [{a:.4},{b:.4}] at a {:?} b {:?} joint {:?}", pc.pcurve().eval(a), pc.pcurve().eval(b), base.joint(he));
    }
    let mut bad = vec![];
    for (rim, (t0, t1)) in rims {
        for s in [0.1, 0.3, 0.5, 0.65, 0.7, 0.8, 0.9] {
            let t = t0 + s * (t1 - t0);
            let mut body = base.clone();
            match body.split_edge(rim, t, Tol::witness()) {
                Ok(_) => {
                    let v = topo::validate(&body);
                    for (he, pc) in body.pcurves() {
                        let (a, b) = pc.params();
                        eprintln!("  s={s} {he:?} [{a:.4},{b:.4}] at a {:?} b {:?} joint {:?}", pc.pcurve().eval(a), pc.pcurve().eval(b), body.joint(he));
                    }
                    let m = std::panic::catch_unwind(|| check_mesh_acceptance(&body, 0.05, None)).map(|m| signed_volume(&m));
                    let line = format!("split {rim:?} at {s}: validate {:?}; mesh {:?}", v.as_ref().map_err(|e| format!("{e:?}")), m.as_ref().map(|v| format!("{v:.6}")));
                    eprintln!("{line}");
                    if v.is_err() || m.is_err() { bad.push(line); }
                }
                Err(e) => { let l = format!("split {rim:?} at {s}: refused {e:?}"); eprintln!("{l}"); bad.push(l); }
            }
        }
    }
    for b in &bad { eprintln!("BAD {b}"); }
    assert!(bad.is_empty());
}

#[test]
fn r2_south_cap_readers() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    for (south, theta) in [(true, 1.0), (true, 2.0 * PI - 0.5), (false, 1.0)] {
        let body = cap(0.6, theta, south);
        let fin = topo::test_support::finished("cap", body.clone(), tol);
        let mp = topo::mass_properties(&body, tol).map(|m| (m.volume, m.surface_area));
        eprintln!("READER south={south} theta={theta:.3} mass {mp:?}");
        let mut grid = String::new();
        for i in 0..7 { for j in 0..7 { for k in 0..7 {
            let s = if south { -1.0 } else { 1.0 };
            let q = geom_core::Point3::new(-0.9 + 0.3 * f64::from(i), s * (0.5 + 0.08 * f64::from(j)), -0.9 + 0.3 * f64::from(k) + 0.013);
            let r = topo::point_in_solid(&body, q, band, tol);
            grid.push_str(&format!("{:?};", r.map(|c| format!("{c:?}")).map_err(|e| format!("{e:?}").chars().take(60).collect::<String>())));
        }}}
        let mut h: u64 = 0xcbf29ce484222325;
        for b in grid.bytes() { h ^= u64::from(b); h = h.wrapping_mul(0x100000001b3); }
        eprintln!("READER south={south} theta={theta:.3} containment digest {h:x} len {}", grid.len());
        let s = if south { -1.0 } else { 1.0 };
        let bx = sweep::extrude(
            &validated(vec![ProfileLoop::<f64>::polygon([
                Point2::new(0.2, s * 0.9), Point2::new(0.6, s * 0.9), Point2::new(0.6, s * 0.4), Point2::new(0.2, s * 0.4),
            ])]),
            sweep::Extrusion::Distance { depth: 0.6, side: sweep::ExtrudeSide::Along },
            tol,
        ).unwrap().body;
        let bx = topo::test_support::finished("box", bx, tol);
        for (name, r) in [("union", topo::union(&fin, &bx, tol)), ("intersection", topo::intersect(&fin, &bx, tol)), ("subtract", topo::subtract(&fin, &bx, tol))] {
            let out = r.as_ref().map(|r| r.body().map(|b| topo::mass_properties(&b.body, tol).map(|m| (m.volume, m.surface_area))));
            eprintln!("READER south={south} theta={theta:.3} {name}: {:?}", out.map_err(|e| format!("{e:?}").chars().take(200).collect::<String>()));
        }
    }
}

#[test]
fn r2_validate_geometric_caps() {
    let tol = Tol::witness();
    for south in [true, false] {
        for lat in [0.3, 0.6] {
            for theta in [1.0, PI, 2.0 * PI - 0.5] {
                let r = topo::validate_geometric(&cap(lat, theta, south), tol);
                eprintln!("VG south={south} lat={lat} theta={theta:.3}: {:?}", r.map_err(|e| format!("{e:?}").chars().take(300).collect::<String>()));
            }
        }
    }
    let base = dome();
    let rims: Vec<_> = base.edges().filter_map(|(e, edge)| {
        let c = base.get_curve_geom(edge.curve)?.certified()?;
        matches!(c.carrier(), geom::Curve3::Circle { .. }).then(|| (e, c.params()))
    }).collect();
    for (rim, (t0, t1)) in rims {
        for s in [0.3, 0.8] {
            let mut body = base.clone();
            body.split_edge(rim, t0 + s * (t1 - t0), tol).unwrap();
            let r = topo::validate_geometric(&body, tol);
            eprintln!("VG dome split {rim:?} at {s}: {:?}", r.map_err(|e| format!("{e:?}").chars().take(300).collect::<String>()));
        }
    }
}
