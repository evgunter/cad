//! STAGE-1 MEASUREMENT HARNESS — does not ship. The class of
//! `work/tesslane/a-boundary-run-on-a-chart-iso-line-gets-a-lattice-line.md`:
//! notched pipe walls and notched sphere faces, meshed over δ with the
//! lane's probe log armed (`ISO_PROBE_LOG`); one CELL line per body and δ.
//! Run: `ISO_PROBE_LOG=<file> cargo nextest run -p mesh --run-ignored only iso_density_probe`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::io::Write as _;

use geom::SurfaceKind;
use geom_core::{Affine3, Point3, Vec3};
use sweep::test_support::{ball_poled_z, brick, finished};
use topo::{AtRestBody, Body};

use crate::reach_probes::{op_of, posed, rod, tol};

fn deltas() -> Vec<f64> {
    std::env::var("ISO_DELTAS")
        .map(|s| s.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or_else(|_| vec![0.1, 0.03, 0.01, 0.003, 0.001])
}

fn log(line: &str) {
    let Ok(p) = std::env::var("ISO_PROBE_LOG") else {
        println!("{line}");
        return;
    };
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(p)
        .unwrap();
    writeln!(f, "-\t-\tCELL\t{line}").unwrap();
}

fn oblique(body: &Body<f64>) -> Body<f64> {
    let axis = Vec3::new(0.317_8_f64, 0.941_2, -0.110_9);
    let placement = Affine3::from_parts(
        Affine3::rotation_about_axis(Point3::origin(), axis * (1.0 / axis.norm()), 1.0 / 3.0)
            .linear,
        Vec3::new(0.117, -0.339_1, 5.001_7),
    );
    topo::transform_rigid(body, &geom_core::test_support::rigid(&placement, tol())).unwrap()
}

/// A curved face with no ring the shape door decides is no rectangle.
fn class_faces(body: &Body<f64>) -> Vec<topo::FaceKey> {
    let band = geom_core::Band::linear(tol()).unwrap();
    body.faces()
        .filter(|(fk, f)| {
            let surface = body.face_surface(*fk).unwrap();
            !matches!(
                surface.kind(),
                SurfaceKind::Plane | SurfaceKind::Nurbs | SurfaceKind::Approx
            ) && !f.rings.is_empty()
                || f.rings.is_empty()
                && !matches!(
                    surface.kind(),
                    SurfaceKind::Plane | SurfaceKind::Nurbs | SurfaceKind::Approx
                )
                && matches!(
                    geom_brep::props::require_iso_rectangle(
                        surface,
                        &topo::props::loop_edges(body, f.outer).unwrap().0,
                        band,
                    ),
                    Err(geom_brep::props::PropsError::NotIsoRectangle { .. })
                )
        })
        .map(|(fk, _)| fk)
        .collect()
}

/// Worst sampled distance from a face's triangles to its surface.
fn worst_dev(body: &Body<f64>, mesh: &mesh::Mesh, fk: topo::FaceKey) -> f64 {
    let surface = body.face_surface(fk).unwrap();
    let patch = mesh.patches.iter().find(|p| p.face == fk).unwrap();
    let mut worst = 0.0f64;
    const N: usize = 8;
    for t in &patch.triangles {
        let [a, b, c] = t.map(|i| mesh.positions[i as usize]);
        for i in 0..=N {
            for j in 0..=N - i {
                #[allow(clippy::cast_precision_loss)]
                let (x, y) = (i as f64 / N as f64, j as f64 / N as f64);
                let z = 1.0 - x - y;
                let p = Point3::new(
                    x * a.x + y * b.x + z * c.x,
                    x * a.y + y * b.y + z * c.y,
                    x * a.z + y * b.z + z * c.z,
                );
                let d = crate::reach_probes::distance(surface, p).unwrap();
                worst = if d.is_nan() { d } else { worst.max(d) };
            }
        }
    }
    worst
}

fn cell(name: &str, body: &Body<f64>) {
    let class = class_faces(body);
    for delta in deltas() {
        let t0 = std::time::Instant::now();
        let got = mesh::tessellate(body, delta, tol());
        let secs = t0.elapsed().as_secs_f64();
        match got {
            Err(e) => log(&format!(
                "{name}\tdelta={delta}\tclass={}\tREFUSED\t{}\tsecs={secs:.3}",
                class.len(),
                format!("{e:?}").chars().take(120).collect::<String>()
            )),
            Ok(m) => {
                let total: usize = m.patches.iter().map(|p| p.triangles.len()).sum();
                let check = mesh::validate::check_mesh(&m).map_or_else(|e| format!("{e:?}"), |()| "ok".into());
                let wind = mesh::validate::check_winding(&m).map_or_else(|e| format!("{e:?}"), |()| "ok".into());
                let per: Vec<String> = class
                    .iter()
                    .map(|&fk| {
                        let n = m.patches.iter().find(|p| p.face == fk).unwrap().triangles.len();
                        let d = worst_dev(body, &m, fk);
                        format!("{fk:?}:{:?}:tris={n}:dev/delta={:.3}", body.face_surface(fk).unwrap().kind(), d / delta)
                    })
                    .collect();
                log(&format!(
                    "{name}\tdelta={delta}\tclass={}\tOK\ttotal={total}\tcheck_mesh={check}\twinding={wind}\tsecs={secs:.3}\t{}",
                    class.len(),
                    per.join(" ")
                ));
            }
        }
    }
}

fn both(name: &str, body: &Body<f64>) {
    cell(&format!("{name} [axis]"), body);
    cell(&format!("{name} [oblique]"), &oblique(body));
}

fn pipe() -> AtRestBody<f64> {
    finished("the pipe", rod(1.0, -2.0, 4.0), tol())
}

fn bar(x: (f64, f64), w: f64, z: (f64, f64), turn: f64) -> AtRestBody<f64> {
    posed(
        "the bar",
        brick(x, (-w, w), z, tol()),
        Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), turn),
    )
}

#[test]
#[ignore = "stage-1 measurement"]
fn iso_density_references() {
    both("ref: the plain pipe", &rod(1.0, -2.0, 4.0));
    both("ref: the plain ball", &ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol()));
}

#[test]
#[ignore = "stage-1 measurement"]
fn iso_density_pipe_walls() {
    let pipe = pipe();
    // The reach probe's: the bar half in across the seam, A ∪ B and A ∖ B.
    let half_in = bar((0.5, 1.1), 0.3, (-0.3, 0.3), 0.0);
    for op in ["A ∪ B", "A ∖ B"] {
        both(&format!("pipe: bar half in, seam, {op}"), &op_of(&pipe, &half_in, op));
    }
    // Holed (a ring each side): the bar through, and a bar beside the seam.
    let through = bar((-1.1, 1.1), 0.3, (-0.3, 0.3), 0.0);
    both("pipe(ring): bar through", &op_of(&pipe, &through, "A ∖ B"));
    let beside = bar((0.5, 1.1), 0.3, (-0.3, 0.3), 1.0);
    both("pipe(ring): bar half in beside seam", &op_of(&pipe, &beside, "A ∖ B"));
    // A notch down from the top rim: width, depth, across or beside the seam.
    for (wn, w) in [("narrow", 0.1), ("mid", 0.3), ("wide", 0.6)] {
        for (dn, d) in [("shallow", 1.5), ("half", 0.0), ("deep", -1.5)] {
            for (sn, turn) in [("across seam", 0.0), ("beside seam", 1.0)] {
                if wn != "mid" && dn != "half" {
                    continue;
                }
                let b = bar((0.0, 1.1), w, (d, 2.5), turn);
                both(&format!("pipe: top notch {wn} {dn} {sn}"), &op_of(&pipe, &b, "A ∖ B"));
            }
        }
    }
    // Two notches on one wall.
    let two = topo::union(
        &bar((0.0, 1.1), 0.3, (0.0, 2.5), 1.0),
        &bar((0.0, 1.1), 0.3, (-2.5, 0.5), 2.5),
        tol(),
    )
    .unwrap();
    let two = finished("two bars", Body::clone(&two.body().unwrap().body), tol());
    both("pipe: two notches top and bottom", &op_of(&pipe, &two, "A ∖ B"));
}

#[test]
#[ignore = "stage-1 measurement"]
fn iso_density_sphere_faces() {
    let ball = finished(
        "the ball",
        ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
        tol(),
    );
    for (what, z, turn) in [
        ("below, through the south pole, across seam", (-2.0, 0.3), -0.785),
        ("below, through the south pole, beside seam", (-2.0, 0.3), 0.3),
        ("above, through the north pole", (0.3, 2.0), 2.0),
        ("a band, beside both poles", (-0.4, 0.5), 0.3),
        ("a thin band, beside both poles", (-0.1, 0.1), 0.3),
        ("below, to the equator less", (-2.0, -0.3), 1.0),
    ] {
        let wedge = posed(
            "the wedge",
            brick((0.0, 3.0), (0.0, 3.0), z, tol()),
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), turn),
        );
        both(&format!("ball: wedge {what}"), &op_of(&ball, &wedge, "A ∖ B"));
    }
}

#[test]
#[ignore = "stage-1 measurement"]
fn iso_density_scaled() {
    // Slender, squat and large walls, the notch scaled with the pipe.
    for (what, r, z0, h) in [("slender", 0.1, -2.0, 4.0), ("squat", 1.0, -0.2, 0.4), ("large", 10.0, -2.0, 4.0)] {
        let pipe = finished("the pipe", rod(r, z0, h), tol());
        let top = z0 + h;
        let b = bar((0.0, 1.1 * r), 0.3 * r, (z0 + 0.5 * h, top + 0.5), 1.0);
        both(&format!("pipe {what}: top notch beside seam"), &op_of(&pipe, &b, "A ∖ B"));
        let b = bar((0.0, 1.1 * r), 0.3 * r, (z0 + 0.5 * h, top + 0.5), 0.0);
        both(&format!("pipe {what}: top notch across seam"), &op_of(&pipe, &b, "A ∖ B"));
    }
    for (what, r) in [("small", 0.1), ("large", 10.0)] {
        let ball = finished("the ball", ball_poled_z(r, Vec3::new(0.0, 0.0, 0.0), tol()), tol());
        let wedge = posed(
            "the wedge",
            brick((0.0, 3.0 * r), (0.0, 3.0 * r), (-2.0 * r, 0.3 * r), tol()),
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), 0.3),
        );
        both(&format!("ball {what}: wedge through the south pole"), &op_of(&ball, &wedge, "A ∖ B"));
    }
}
