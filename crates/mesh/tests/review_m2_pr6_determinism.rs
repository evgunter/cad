//! M2 PR 6 adversarial review — determinism (assignments 3 and 6):
//! byte-identical rebuilds, ε-row independence (mesh is a function of
//! (body, δ) alone), δ/ε separation, spade insertion-order stability,
//! and a cross-profile dump printer (debug↔release comparison is
//! driven from the shell: run `print_dump_hashes` in both profiles).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{
    axis_y, ball, cone, donut, dump, eps, l_prism, p2, rounded_prism, validated, washer, wedge,
};
use mesh::tessellate;
use profile::ProfileLoop;
use sweep::{Revolution, revolve};

/// FNV-1a over a string (independent tiny hash, no deps).
fn fnv(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

fn all_dumps(delta: f64) -> String {
    let mut out = String::new();
    for (name, body) in [
        ("l_prism", l_prism()),
        ("rounded", rounded_prism()),
        ("ball", ball()),
        ("cone", cone()),
        ("washer", washer()),
        ("donut", donut()),
        ("wedge", wedge()),
    ] {
        let mesh = tessellate(&body, delta).unwrap();
        out.push_str(name);
        out.push('\n');
        out.push_str(&dump(&mesh));
    }
    out
}

#[test]
fn survives_byte_identical_rebuild_across_bodies() {
    for delta in [0.2, 0.03] {
        let a = all_dumps(delta);
        let b = all_dumps(delta);
        assert_eq!(fnv(&a), fnv(&b));
        assert_eq!(a, b, "rebuild not byte-identical at delta {delta}");
    }
}

/// Shell-driven cross-profile oracle: prints one FNV line per δ.
/// Run with `--ignored --nocapture` in debug and release and diff.
#[test]
#[ignore]
fn print_dump_hashes() {
    for delta in [0.2, 0.05, 0.021] {
        println!("DUMPHASH delta={delta} fnv={:016x}", fnv(&all_dumps(delta)));
    }
}

#[test]
fn survives_eps_row_bitwise_independence() {
    // Re-exec this test binary under each ε row and compare full dump
    // hashes: the mesh must be bitwise a function of (body, δ) alone.
    // (ε is read once, for pole identification; across sane rows the
    // identification must not flip for these bodies.)
    let exe = std::env::current_exe().unwrap();
    let mut hashes = Vec::new();
    for row in ["1e-6", "1e-9", "1e-12"] {
        let out = std::process::Command::new(&exe)
            .args(["print_dump_hashes", "--ignored", "--exact", "--nocapture"])
            .env("CAD_TOLERANCE_EPS", row)
            .output()
            .unwrap();
        assert!(out.status.success(), "row {row} run failed");
        let text = String::from_utf8_lossy(&out.stdout);
        let lines: Vec<&str> = text.lines().filter(|l| l.starts_with("DUMPHASH")).collect();
        assert_eq!(lines.len(), 3, "row {row}: printer lines missing");
        hashes.push(lines.join(";"));
    }
    assert_eq!(hashes[0], hashes[1], "mesh differs between eps rows");
    assert_eq!(hashes[1], hashes[2], "mesh differs between eps rows");
}

#[test]
fn survives_delta_sweep_at_fixed_eps_monotone_sane() {
    // δ sweep at the ambient ε: finer δ must not produce fewer
    // triangles, and every level rebuilds byte-identically.
    let body = donut();
    let mut last = 0usize;
    for delta in [0.5, 0.1, 0.02] {
        let m1 = tessellate(&body, delta).unwrap();
        let m2 = tessellate(&body, delta).unwrap();
        assert_eq!(dump(&m1), dump(&m2));
        let n = mesh::validate::triangle_count(&m1);
        assert!(n >= last, "triangle count shrank as delta tightened");
        last = n;
    }
}

#[test]
fn survives_canonically_equal_profile_constructions() {
    // The same washer geometry entered with a rotated vertex list and
    // with reversed (CW) winding: profile canonicalization makes the
    // same solid. Arena orders may differ (mesh ids are body-lineage
    // values), so the D9 claim is NOT bitwise equality here — but both
    // meshes must pass the full battery and agree on volume to within
    // the certified band, and each must rebuild bitwise.
    let variants = [
        vec![p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)],
        vec![p2(2.0, 1.0), p2(1.0, 1.0), p2(1.0, 0.0), p2(2.0, 0.0)],
        vec![p2(1.0, 0.0), p2(1.0, 1.0), p2(2.0, 1.0), p2(2.0, 0.0)], // CW
    ];
    let delta = 0.05;
    let mut vols = Vec::new();
    for pts in variants {
        let body = revolve(
            &validated(vec![ProfileLoop::polygon(pts)]),
            axis_y(),
            Revolution::Full,
        )
        .unwrap()
        .body;
        let mesh = common::check_mesh_acceptance(&body, delta, None);
        vols.push(mesh::validate::signed_volume(&mesh));
    }
    let band = 2.0 * 12.0 * core::f64::consts::PI * (delta + eps());
    for v in &vols {
        assert!(
            (v - vols[0]).abs() <= 2.0 * band,
            "volumes diverge: {vols:?}"
        );
    }
}

#[test]
fn survives_near_axis_vertex_arc_endpoint() {
    // ε-row-sensitive construction: an arc endpoint a hair off the
    // axis (d = 1e-7 sits between the 1e-6 and 1e-9 rows). Whatever
    // the kernel's axis-contact classification decides at the ambient
    // row, the tessellation must either be refused typed upstream or
    // produce a watertight certified mesh — never a broken one.
    let d = 1e-7;
    let lp = ProfileLoop::new(vec![
        profile::ProfileVertex {
            pos: p2(d, -1.0),
            bulge: 1.0,
        },
        profile::ProfileVertex {
            pos: p2(d, 1.0),
            bulge: 0.0,
        },
    ]);
    let profile = profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
        .validate(geom_core::Tolerance::get());
    let Ok(vp) = profile else {
        return; // refused at profile validation on this row — typed, fine
    };
    match revolve(&vp, axis_y(), Revolution::Full) {
        Err(_) => {} // refused typed upstream — fine
        Ok(out) => {
            let body = out.body;
            match tessellate(&body, 0.05) {
                Err(e) => {
                    // Typed refusal is acceptable; a panic would not be.
                    let _ = e;
                }
                Ok(mesh) => {
                    assert_eq!(mesh::validate::check_mesh(&mesh), Ok(()));
                    assert!(mesh::validate::signed_volume(&mesh) > 0.0);
                }
            }
        }
    }
}
