//! R2 e2e for SENSE-DOORS: a real user program — a revolved ball, its
//! inside-out twin and a one-band-reversed twin — through
//! `validate_geometric` (check 4 -> classify_material_pairing) and
//! `mass_properties` (curved_face/sphere), Debug printed for a
//! merge-base diff.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::Tol;
use topo::props::mass_properties;
use topo::validate::validate_geometric;

fn report(label: &str, body: &topo::Body<f64>) {
    let faces: Vec<_> = body.faces().map(|(k, f)| (format!("{k:?}"), f.sense)).collect();
    println!("[e2e] {label} faces: {faces:?}");
    let mp = mass_properties(body, Tol::witness());
    println!("[e2e] {label} mass: {mp:?}");
    let v = validate_geometric(body, Tol::witness());
    println!("[e2e] {label} validate: {v:?}");
}

#[test]
fn r2_sense_e2e_ball_and_twins() {
    let ball = common::ball();
    report("ball", &ball);
    let mut inside_out = ball.clone();
    let keys: Vec<_> = inside_out.faces().map(|(k, _)| k).collect();
    for k in &keys {
        let s = inside_out.get_face(*k).unwrap().sense;
        inside_out.set_face_sense(*k, !s).unwrap();
    }
    report("inside-out", &inside_out);
    let mut one_band = ball.clone();
    let s = one_band.get_face(keys[0]).unwrap().sense;
    one_band.set_face_sense(keys[0], !s).unwrap();
    report("one-band-reversed", &one_band);
    let cylinder = common::rounded_prism();
    report("rounded-prism", &cylinder);
    // Reverse ONE wall of the rounded prism: its plane/cylinder joints
    // are declared tangent, so check 4's material arm runs
    // classify_material_pairing on plane-cylinder pairs with mixed bits.
    let mut wall = cylinder.clone();
    let wk: Vec<_> = wall.faces().map(|(k, _)| k).collect();
    let s = wall.get_face(wk[2]).unwrap().sense;
    wall.set_face_sense(wk[2], !s).unwrap();
    report("prism-one-wall-reversed", &wall);
}
