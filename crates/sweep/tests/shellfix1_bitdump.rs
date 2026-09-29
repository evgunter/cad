//! SHELLFIX PR-1's bit-identity corpus: the sealed box, the box cup,
//! the box tube, the sealed vessel and the sealed tube (#1048's
//! acceptance shapes plus the sealed revolves), each written through
//! [`crate::common::bitdump`] to `$BITDUMP_DIR/<name>.txt`. Run at a
//! merge base and at a head, then `diff`. Unarmed when the variable is
//! unset (an explicit clean skip).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom_core::Tol;
use sweep::test_support::block;
use topo::{Body, FaceKey};

use crate::common::bitdump::{dump, dump_dir, save};
use crate::common::shell_operands::{tube, vessel};

fn plane_face_at_z(body: &Body<f64>, z: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.z - z).abs() < 1e-9 && normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .unwrap()
}

#[test]
fn shellfix1_bitdump_corpus() {
    let Some(dir) = dump_dir() else {
        println!("[shellfix1_bitdump] BITDUMP_DIR unset; clean skip");
        return;
    };
    let write_dump = |name: &str, body: &Body<f64>| save(&dir, name, &dump(body));
    let (w, d, h, t) = (2.0, 3.0, 4.0, 0.25);
    let body = block(w, d, h, Tol::witness());
    write_dump(
        "sealed_box",
        &topo::shell(&body, t, Tol::witness()).unwrap().body,
    );
    let top = plane_face_at_z(&body, h);
    let bottom = plane_face_at_z(&body, 0.0);
    write_dump(
        "box_cup",
        &topo::shell_open(&body, t, &[top], Tol::witness())
            .unwrap()
            .body,
    );
    write_dump(
        "box_tube",
        &topo::shell_open(&body, t, &[top, bottom], Tol::witness())
            .unwrap()
            .body,
    );
    write_dump(
        "sealed_vessel",
        &topo::shell(&vessel(1.0, 2.0), 0.2, Tol::witness())
            .unwrap()
            .body,
    );
    write_dump(
        "sealed_tube",
        &topo::shell(&tube(0.6, 1.0, 2.0), 0.1, Tol::witness())
            .unwrap()
            .body,
    );
}
