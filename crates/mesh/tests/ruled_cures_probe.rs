//! THROWAWAY: the stage-1 measurement of unit 4c's three cures over the
//! spec's corpus (`mesh::ruled_probe`). Ignored; run with
//! `PROBE_OUT=<file> cargo nextest run -p mesh --release --run-ignored only ruled_cures_probe`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;
use std::io::Write as _;
use std::time::Instant;

use geom_core::{Affine3, Point2, Point3, Vec3};
use profile::ProfileLoop;
use profile::RawLoop as _;
use sweep::{Revolution, revolve};
use topo::Body;
use topo::construction::Construct;

use crate::common::{axis_y, validated};
use crate::reach_probes::{geometry_enclosing, tol};

fn turned(pts: &[(f64, f64)], turn: Revolution<f64>) -> Body<f64> {
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    revolve(&validated(vec![lp]), axis_y(), turn, tol())
        .unwrap()
        .body
        .into_body()
}

fn is_line(body: &Body<f64>, ek: topo::EdgeKey) -> bool {
    matches!(
        body.get_curve_geom(body.get_edge(ek).unwrap().curve)
            .and_then(|g| g.certified())
            .map(|c| c.carrier()),
        Some(geom::Curve3::Line(..))
    )
}

fn rulings(body: &Body<f64>) -> Vec<topo::EdgeKey> {
    body.edges()
        .filter(|(ek, _)| is_line(body, *ek))
        .map(|(ek, _)| ek)
        .collect()
}

fn place(b: &Body<f64>) -> Body<f64> {
    let axis = Vec3::new(0.317_8_f64, 0.941_2, -0.110_9);
    let placement = Affine3::from_parts(
        Affine3::rotation_about_axis(Point3::origin(), axis * (1.0 / axis.norm()), 1.0 / 3.0)
            .linear,
        Vec3::new(0.117, -0.339_1, 5.001_7),
    );
    topo::transform_rigid(b, &geom_core::test_support::rigid(&placement, tol())).unwrap()
}

/// `body` with `ek` split at `fracs`, placed; `None` where `split_edge`
/// refuses.
fn split(body: &Body<f64>, ek: topo::EdgeKey, fracs: &[f64]) -> Option<Body<f64>> {
    let mut b = body.clone();
    let (t0, t1) = b
        .get_curve_geom(b.get_edge(ek).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .params();
    for &f in fracs {
        b.split_edge(ek, t0 + (t1 - t0) * f, tol()).ok()?;
    }
    Some(place(&b))
}

struct Cell {
    family: &'static str,
    name: String,
    solid: Body<f64>,
    split: Body<f64>,
    deltas: Vec<f64>,
}

fn splits() -> Vec<(&'static str, Vec<f64>)> {
    vec![
        ("1@0.5", vec![0.5]),
        ("2", vec![0.312_9, 0.156_45]),
        ("5", vec![0.9, 0.7, 0.5, 0.3, 0.1]),
        ("end1e-3", vec![1e-3]),
        ("end1e-6", vec![1e-6]),
        ("end1-1e-8", vec![1.0 - 1e-8]),
        ("end1e-8", vec![1e-8]),
    ]
}

fn cells_of(family: &'static str, name: &str, body: &Body<f64>, deltas: &[f64]) -> Vec<Cell> {
    let solid = place(body);
    let mut out = Vec::new();
    for (k, ek) in rulings(body).into_iter().enumerate() {
        for (what, fracs) in splits() {
            if let Some(s) = split(body, ek, &fracs) {
                out.push(Cell {
                    family,
                    name: format!("{name} e{k} {what}"),
                    solid: solid.clone(),
                    split: s,
                    deltas: deltas.to_vec(),
                });
            }
        }
    }
    out
}

fn corpus(only: Option<&str>) -> Vec<Cell> {
    let tau = core::f64::consts::TAU;
    let ds = [0.1, 0.03, 0.01, 0.003, 0.001];
    let want = |f: &str| only.is_none_or(|o| o == f);
    let mut cells = Vec::new();
    let cone = |h: f64| vec![(0.0, 0.0), (f64::tan(h.to_radians()), 0.0), (0.0, 1.0)];
    let wall = [(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)];
    if want("walls") {
        let bodies: Vec<(&str, Body<f64>)> = vec![
            ("cylinder wall", turned(&wall, Revolution::Full)),
            (
                "frustum wall",
                turned(
                    &[(1.0, 0.0), (3.0, 0.0), (2.0, 1.0), (1.5, 1.0)],
                    Revolution::Full,
                ),
            ),
            (
                "slender wall",
                turned(
                    &[(1.0, 0.0), (1.2, 0.0), (1.1, 1.0), (1.05, 1.0)],
                    Revolution::Full,
                ),
            ),
            (
                "needle wall",
                turned(
                    &[(0.1, 0.0), (0.15, 0.0), (0.12, 3.0), (0.11, 3.0)],
                    Revolution::Full,
                ),
            ),
            (
                "half cylinder wall",
                turned(&wall, Revolution::Partial(tau / 2.0)),
            ),
            (
                "draft wall 0.1°",
                turned(
                    &[
                        (1.0, 0.0),
                        (2.0 + 0.1f64.to_radians().tan(), 0.0),
                        (2.0, 1.0),
                        (1.0, 1.0),
                    ],
                    Revolution::Full,
                ),
            ),
        ];
        for (n, b) in &bodies {
            cells.extend(cells_of("walls", n, b, &ds));
        }
    }
    if want("cones") {
        for h in [0.01, 0.1, 1.0, 5.0, 20.0, 45.0, 80.0] {
            let b = turned(&cone(h), Revolution::Full);
            cells.extend(cells_of("cones", &format!("cone {h}°"), &b, &ds));
        }
        let b = turned(&cone(20.0), Revolution::Partial(tau - 0.3));
        cells.extend(cells_of("cones", "cone 20° 2π−0.3", &b, &ds));
    }
    if want("frustums") {
        let f = [(0.0, 0.0), (2.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        cells.extend(cells_of(
            "frustums",
            "frustum full",
            &turned(&f, Revolution::Full),
            &ds,
        ));
        cells.extend(cells_of(
            "frustums",
            "frustum half",
            &turned(&f, Revolution::Partial(tau / 2.0)),
            &ds,
        ));
    }
    if want("slender") {
        // The orchestrator's two: a 45° cone of 1000 a full turn split
        // 1e-8 from its rim end at δ 1, and a 2° cone of height 100 split
        // 3e-8 from its rim end at δ 0.01.
        let big = turned(
            &[(0.0, 0.0), (1000.0, 0.0), (0.0, 1000.0)],
            Revolution::Full,
        );
        let thin = turned(
            &[(0.0, 0.0), (100.0 * 2f64.to_radians().tan(), 0.0), (0.0, 100.0)],
            Revolution::Full,
        );
        for (name, body, frac, deltas) in [
            ("cone 1000", &big, 1e-8, vec![1.0, 0.1]),
            ("cone 2° h100", &thin, 3e-8, vec![0.1, 0.01]),
        ] {
            let solid = place(body);
            for (k, ek) in rulings(body).into_iter().enumerate() {
                // The rim end: the ruling's end farther from the apex.
                let c = body
                    .get_curve_geom(body.get_edge(ek).unwrap().curve)
                    .unwrap()
                    .certified()
                    .unwrap();
                let (t0, t1) = c.params();
                let r = |p: Point3<f64>| (p.x * p.x + p.z * p.z).sqrt();
                let len = (c.carrier().eval(t1) - c.carrier().eval(t0)).norm();
                for (what, rel) in [("rel", frac), ("abs", frac / len)] {
                    let f = if r(c.carrier().eval(t1)) > r(c.carrier().eval(t0)) {
                        1.0 - rel
                    } else {
                        rel
                    };
                    match split(body, ek, &[f]) {
                        Some(s) => cells.push(Cell {
                            family: "slender",
                            name: format!("{name} e{k} rim-end {frac:e} {what}"),
                            solid: solid.clone(),
                            split: s,
                            deltas: deltas.clone(),
                        }),
                        None => eprintln!("PROBE: {name} e{k} {what}: split_edge refuses"),
                    }
                }
            }
        }
    }
    if want("t1") {
        let (a, b) = crate::reach_probes::t1_toward_the_apex(0.6);
        for op in ["A ∪ B", "A ∩ B", "A ∖ B", "B ∖ A"] {
            let body = crate::reach_probes::op_of(&a, &b, op);
            let solid = body.clone();
            for (k, ek) in rulings(&body).into_iter().enumerate() {
                let mut s = body.clone();
                let (t0, t1) = s
                    .get_curve_geom(s.get_edge(ek).unwrap().curve)
                    .unwrap()
                    .certified()
                    .unwrap()
                    .params();
                if s.split_edge(ek, 0.5 * (t0 + t1), tol()).is_ok() {
                    cells.push(Cell {
                        family: "t1",
                        name: format!("T1 {op} e{k}"),
                        solid: solid.clone(),
                        split: s,
                        deltas: vec![2e-2, 5e-3, 1e-3],
                    });
                }
            }
        }
    }
    cells
}

fn outcome(cell: &Cell, r: &Result<mesh::Mesh, mesh::TessellateError>, delta: f64) -> String {
    match r {
        Ok(m) => match mesh::validate::check_mesh(m) {
            Ok(()) => match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                geometry_enclosing(&cell.split, &cell.solid, m, delta)
            })) {
                Ok(Ok(())) => "ok".into(),
                Ok(Err(e)) => format!("BAD {e}"),
                Err(_) => match mesh::validate::check_winding(m) {
                    Ok(()) => "ok(winding; no volume: body outside session box)".into(),
                    Err(e) => format!("BAD winding {e:?}"),
                },
            },
            Err(e) => format!("check_mesh {e:?}"),
        },
        Err(e) => {
            let s = format!("{e:?}");
            s.split(['{', '(']).next().unwrap().trim().to_string()
                + &if s.contains("DevelopedMerge") {
                    "/DevelopedMerge".to_string()
                } else if let Some(i) = s.find("kind:") {
                    format!("/{}", &s[i..(i + 40).min(s.len())])
                } else {
                    String::new()
                }
        }
    }
}

fn rim_points(body: &Body<f64>, m: &mesh::Mesh) -> usize {
    m.boundaries
        .iter()
        .filter(|b| !is_line(body, b.edge))
        .map(|b| b.points.len())
        .sum()
}

#[test]
#[ignore = "stage-1 measurement; run by hand"]
fn ruled_cures_probe() {
    let only = std::env::var("PROBE_FAMILY").ok();
    let cures: Vec<u8> = std::env::var("PROBE_CURES")
        .map(|s| s.split(',').map(|c| c.parse().unwrap()).collect())
        .unwrap_or_else(|_| vec![0, 1, 4, 2, 3]);
    let path = std::env::var("PROBE_OUT").unwrap_or_else(|_| "/dev/stderr".into());
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    let eps = tol().eps();
    for cell in corpus(only.as_deref()) {
        for &delta in &cell.deltas {
            mesh::ruled_probe::set(0);
            let reference = mesh::tessellate(&cell.solid, delta, tol());
            mesh::ruled_probe::take();
            for &cure in &cures {
                mesh::ruled_probe::set(cure);
                let t = Instant::now();
                let r = mesh::tessellate(&cell.split, delta, tol());
                let ms = t.elapsed().as_secs_f64() * 1e3;
                let stats = mesh::ruled_probe::take();
                let got = outcome(&cell, &r, delta);
                let mut faces = String::new();
                for s in &stats {
                    let unsplit = reference.as_ref().ok().and_then(|m| {
                        m.patches
                            .iter()
                            .find(|p| p.face == s.face)
                            .map(|p| p.triangles.len())
                    });
                    let _ = write!(
                        faces,
                        "[{}/{} pass{} cert/δ {:.3} int {}{}] ",
                        s.triangles,
                        unsplit.map_or("-".into(), |u| u.to_string()),
                        s.passes,
                        s.worst / delta,
                        s.interior,
                        s.refused
                            .as_deref()
                            .map_or(String::new(), |e| format!(
                                " REF {}",
                                e.split(['{', '(']).next().unwrap().trim()
                            )),
                    );
                }
                let rims = match (&r, &reference) {
                    (Ok(m), Ok(rm)) => format!(
                        "{}/{}",
                        rim_points(&cell.split, m),
                        rim_points(&cell.solid, rm)
                    ),
                    _ => "-".into(),
                };
                let total = match (&r, &reference) {
                    (Ok(m), Ok(rm)) => format!(
                        "{}/{}",
                        m.patches.iter().map(|p| p.triangles.len()).sum::<usize>(),
                        rm.patches.iter().map(|p| p.triangles.len()).sum::<usize>()
                    ),
                    _ => "-".into(),
                };
                writeln!(
                    file,
                    "{eps:e}\t{}\t{}\t{delta}\tc{cure}\t{got}\t{faces}\trims {rims}\ttotal {total}\t{ms:.1}ms",
                    cell.family, cell.name
                )
                .unwrap();
            }
        }
    }
    mesh::ruled_probe::set(0);
}
