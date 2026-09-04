//! M5 PR 11: the Band 4 corpus's CURVED documents gain
//! tessellation/props columns — cut_cylinder's split halves and
//! boss_union's body tessellate WATERTIGHT and carry their PR 11 mass
//! answers (certified enclosure for the conic-trimmed halves; closed
//! form for the boss). Wall-clock per column is PRINTED (the T6
//! tripwire's evidence — reported, never gated; PERF-PLAN stays
//! advisory).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

use std::time::Instant;

use editor_core::{SplitSide, ValuePayload};
use mesh::validate::{check_mesh, signed_volume, triangle_count};
use topo::Body;

use corpus::{body_of, documents, eval, failures};
use geom_core::Tol;

const DELTA: f64 = 1e-2;

/// Tessellate + watertight + volume sanity for one body; returns the
/// (tessellation wall-clock, triangle count) column.
fn tess_column(label: &str, body: &Body<f64>) -> (f64, usize) {
    let t0 = Instant::now();
    let mesh = mesh::tessellate(body, DELTA, Tol::witness())
        .unwrap_or_else(|e| panic!("{label}: tessellates at delta {DELTA}: {e:?}"));
    let dt = t0.elapsed().as_secs_f64() * 1e3;
    check_mesh(&mesh).unwrap_or_else(|e| panic!("{label}: watertight: {e:?}"));
    assert!(signed_volume(&mesh) > 0.0, "{label}: outward orientation");
    (dt, triangle_count(&mesh))
}

/// cut_cylinder: both split halves tessellate watertight and the
/// certified volume enclosure brackets πr²H/2 (r = 0.5, H = 1, tilt
/// through the mid-height axis point).
#[test]
fn cut_cylinder_gains_tessellation_and_props_columns() {
    let d = documents()
        .into_iter()
        .find(|d| d.name == "cut_cylinder")
        .unwrap();
    let ev = eval::<f64>(&d.doc);
    assert!(failures(&ev).is_empty());
    let split = d
        .doc
        .order()
        .iter()
        .copied()
        .find(|&id| {
            matches!(
                ev.value(id).map(|v| &v.payload),
                Some(ValuePayload::Split { .. })
            )
        })
        .expect("the document's head is a Split");
    let Some(ValuePayload::Split { above, below }) = ev.value(split).map(|v| &v.payload) else {
        panic!("split payload");
    };
    let (SplitSide::Body(above), SplitSide::Body(below)) = (above, below) else {
        panic!("both sides carry material");
    };
    // r = 0.5, H = 1: each half of the mid-axis tilted cut encloses
    // exactly πr²H/2.
    let half = core::f64::consts::PI * 0.5f64.powi(2) * 1.0 / 2.0;
    for (label, body) in [
        ("cut_cylinder/above", &**above),
        ("cut_cylinder/below", &**below),
    ] {
        let (ms, tris) = tess_column(label, body);
        let m = topo::mass_properties(body, Tol::witness())
            .expect("the PR 11 quadrature lane computes");
        assert!(
            m.volume - m.volume_pad <= half && half <= m.volume + m.volume_pad,
            "{label}: bracket [{}, {}] vs {half}",
            m.volume - m.volume_pad,
            m.volume + m.volume_pad
        );
        println!("column {label}: tessellate {ms:.1} ms, {tris} triangles (delta {DELTA})");
    }
}

/// boss_union: the result body tessellates watertight; volume stays
/// the closed form 7.2 + π·0.35²·0.5 (the corpus document's dims).
#[test]
fn boss_union_gains_tessellation_and_props_columns() {
    let d = documents()
        .into_iter()
        .find(|d| d.name == "boss_union")
        .unwrap();
    let ev = eval::<f64>(&d.doc);
    assert!(failures(&ev).is_empty());
    let body = body_of(&ev, d.result.expect("boss carries a result"));
    let (ms, tris) = tess_column("boss_union", body);
    let vol = topo::mass_properties(body, Tol::witness()).unwrap().volume;
    // 3×3×0.8 plate + r = 0.35 boss poking 0.5 out of the plate.
    let expect = 7.2 + core::f64::consts::PI * 0.35f64.powi(2) * 0.5;
    assert!((vol - expect).abs() < 1e-6, "vol {vol} vs {expect}");
    println!("column boss_union: tessellate {ms:.1} ms, {tris} triangles (delta {DELTA})");
}
