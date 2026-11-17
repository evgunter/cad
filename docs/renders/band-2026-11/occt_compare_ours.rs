//! Our side of `occt-compare.md`: every fixture of `band_mixed_corner` (flush, inset, flush at 120°, the bore-less
//! bracket) under each request, both verbs, into `ours.tsv` (status, ΔV, face count), a soup per build and, per
//! chamfer build, membership samples (`ours-*.pts`) that `occt_compare.py` classifies in OCCT. It reads the
//! suite's own fixtures, so it runs as a module of `crates/sweep/tests/all.rs` (add `#[path = "../../../docs/renders/band-2026-11/occt_compare_ours.rs"] mod zz_occt_compare;`
//! locally, do not commit that line), as an ignored test, `OCCT_CMP_OUT` naming the output directory.
#![allow(
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    dead_code
)]
use core::f64::consts::PI;
use geom::Surface;
use std::io::Write;
use sweep::blend::build::fillet_edges;
use sweep::chamfer::chamfer_edges;
use sweep::test_support::at_rest;
use topo::boolean::solid_contain::{SolidContainment, point_in_solid};
use topo::{Body, EdgeKey};

use crate::band_mixed_corner::{P, boreless, edge, flush, flush_at, inset, tol, volume};

fn dump(body: &Body<f64>, path: &str) {
    let m = mesh::tessellate(body, 2e-3, tol()).expect("tessellate");
    let mut f = std::fs::File::create(path).unwrap();
    for p in &m.positions {
        writeln!(f, "v {} {} {}", p.x, p.y, p.z).unwrap();
    }
    for patch in &m.patches {
        let kind = match body.face_surface(patch.face) {
            Some(Surface::Plane { .. }) => 0,
            Some(Surface::Cylinder { .. }) => 1,
            Some(Surface::Torus { .. }) => 2,
            Some(Surface::Sphere { .. }) => 3,
            _ => 4,
        };
        for t in &patch.triangles {
            writeln!(f, "f {} {} {} {kind}", t[0], t[1], t[2]).unwrap();
        }
    }
}

fn inside(body: &Body<f64>, p: P) -> Option<bool> {
    let band = geom_core::Band::linear(tol()).unwrap();
    match point_in_solid(body, geom_core::Point3::new(p.0, p.1, p.2), band, tol()) {
        Ok(SolidContainment::In) => Some(true),
        Ok(SolidContainment::Out) => Some(false),
        _ => None,
    }
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

fn case(
    dir: &str,
    log: &mut std::fs::File,
    fixture: &str,
    body: &Body<f64>,
    req: &str,
    edges: &[EdgeKey],
    s: f64,
    boxes: &[(P, P)],
) {
    let v0 = volume(body).0;
    for verb in ["chamfer", "fillet"] {
        let operand = at_rest(body, tol());
        let got = if verb == "chamfer" {
            chamfer_edges(&operand, edges, s, tol())
        } else {
            fillet_edges(&operand, edges, s, tol())
        }
        .map_err(|r| r.error);
        match got {
            Ok(out) => {
                let b = out.body.into_body();
                let dv = volume(&b).0 - v0;
                let faces = topo::query::all_faces(&b).len();
                writeln!(log, "{fixture}\t{req}\t{verb}\tbuilds\t{dv:.15e}\t{faces}").unwrap();
                let name = format!("{fixture}-{req}-{verb}");
                dump(&b, &format!("{dir}/ours-{name}.soup"));
                if verb == "chamfer" {
                    let mut f = std::fs::File::create(format!("{dir}/ours-{name}.pts")).unwrap();
                    let mut g = Lcg(0x0cc7_7038);
                    for &(lo, hi) in boxes {
                        for _ in 0..3000 {
                            let p = (
                                lo.0 + (hi.0 - lo.0) * g.next(),
                                lo.1 + (hi.1 - lo.1) * g.next(),
                                lo.2 + (hi.2 - lo.2) * g.next(),
                            );
                            let m = match inside(&b, p) {
                                Some(true) => 1,
                                Some(false) => 0,
                                None => -1,
                            };
                            writeln!(f, "{} {} {} {m}", p.0, p.1, p.2).unwrap();
                        }
                    }
                }
            }
            Err(e) => {
                let t: String = format!("{e:?}").chars().take(200).collect();
                writeln!(
                    log,
                    "{fixture}\t{req}\t{verb}\trefuses\t{}\t",
                    t.replace(['\n', '\t'], " ")
                )
                .unwrap();
            }
        }
    }
}

#[test]
#[ignore = "scratch"]
fn occt_compare_dump() {
    let dir = std::env::var("OCCT_CMP_OUT").unwrap();
    let mut log = std::fs::File::create(format!("{dir}/ours.tsv")).unwrap();
    let flush_box = [((-0.2, -0.2, 0.0005), (0.25, 0.25, 0.2))];
    let inset_box = [((-0.25, -0.25, -0.2), (0.2, 0.2, 0.2))];
    let corner3 = |name: &str,
                   log: &mut std::fs::File,
                   body: &Body<f64>,
                   [odd, p1, p2]: [EdgeKey; 3],
                   boxes: &[(P, P)]| {
        for (req, edges) in [
            ("a", vec![odd, p1]),
            ("b", vec![odd, p1, p2]),
            ("c", vec![odd]),
            ("d", vec![p1]),
            ("e", vec![p1, p2]),
        ] {
            case(&dir, log, name, body, req, &edges, 0.1, boxes);
        }
    };
    let (body, e) = flush::<f64>();
    corner3("flush", &mut log, &body, e, &flush_box);
    let (body, e) = inset::<f64>();
    corner3("inset", &mut log, &body, e, &inset_box);
    let (body, e) = flush_at::<f64>(2.0 * PI / 3.0);
    corner3("flush120", &mut log, &body, e, &flush_box);
    let body = boreless::<f64>();
    let r1 = edge(&body, (0.0, 0.5, 0.5), (0.5, 0.5, 0.5));
    let r2 = edge(&body, (0.0, 3.5, 0.5), (0.5, 3.5, 0.5));
    let ry = edge(&body, (0.5, 0.5, 0.5), (0.5, 3.5, 0.5));
    let v1 = edge(&body, (0.0, 0.5, 0.5), (0.0, 0.5, 2.0));
    let v2 = edge(&body, (0.0, 3.5, 0.5), (0.0, 3.5, 2.0));
    let s1 = edge(&body, (0.0, 0.5, 0.5), (0.0, 0.0, 0.5));
    let s2 = edge(&body, (0.0, 3.5, 0.5), (0.0, 4.0, 0.5));
    let w1 = edge(&body, (0.5, 0.5, 0.5), (0.5, 0.5, 2.0));
    let w2 = edge(&body, (0.5, 3.5, 0.5), (0.5, 3.5, 2.0));
    let h = 0.1;
    let boxes: Vec<(P, P)> = [(0.0, 0.5), (0.5, 0.5), (0.0, 3.5), (0.5, 3.5)]
        .iter()
        .map(|&(x, y)| ((x - h, y - h, 0.5 - h), (x + h, y + h, 0.5 + h)))
        .collect();
    case(
        &dir,
        &mut log,
        "bracket",
        &body,
        "a",
        &[r1, r2, v1, s2, w1, w2],
        0.05,
        &boxes,
    );
    case(
        &dir,
        &mut log,
        "bracket",
        &body,
        "b",
        &[r1, r2, ry, v1, v2, s1, s2, w1, w2],
        0.05,
        &boxes,
    );
}
