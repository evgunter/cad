//! M2 PR 7 STL acceptance: byte-identity (D9), ASCII↔binary triangle
//! equivalence under parse-back, and format sanity.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use mesh::tessellate;
use stl::{write_ascii, write_binary};

fn fnv(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// The acceptance meshes, tessellated once per process (the donut pays
/// the documented quadratic CDT cost — see `common::acceptance_bodies`
/// for the per-body δ) — the writers under test are pure functions of
/// the mesh, so sharing it is sound.
fn meshes() -> &'static [(&'static str, mesh::Mesh)] {
    static MESHES: std::sync::OnceLock<Vec<(&'static str, mesh::Mesh)>> =
        std::sync::OnceLock::new();
    MESHES.get_or_init(|| {
        common::acceptance_bodies()
            .into_iter()
            .map(|(name, body, delta)| (name, tessellate(&body, delta).unwrap()))
            .collect()
    })
}

fn binary_of(mesh: &mesh::Mesh) -> Vec<u8> {
    let mut out = Vec::new();
    write_binary(mesh, &mut out).unwrap();
    out
}

fn ascii_of(mesh: &mesh::Mesh) -> Vec<u8> {
    let mut out = Vec::new();
    write_ascii(mesh, &mut out).unwrap();
    out
}

// ---- byte identity ----------------------------------------------------

#[test]
fn repeated_export_is_byte_identical() {
    // Repeat-call identity through the FULL pipeline: rebuild the
    // body, retessellate, rewrite — bytes must match the cached
    // pipeline's output exactly.
    for (name, body, delta) in common::acceptance_bodies() {
        let rebuilt = binary_of(&tessellate(&body, delta).unwrap());
        let cached = meshes().iter().find(|(n, _)| *n == name).unwrap();
        let a = binary_of(&cached.1);
        assert_eq!(fnv(&a), fnv(&rebuilt));
        assert_eq!(a, rebuilt, "{name}: binary export not byte-identical");
        assert_eq!(
            ascii_of(&cached.1),
            ascii_of(&cached.1),
            "{name}: ascii export not byte-identical"
        );
    }
}

/// Shell-driven cross-profile / cross-ε oracle: prints one FNV line
/// per body. Run under debug and release (and any ε row) and diff —
/// the same mechanism as the mesh determinism suite.
#[test]
#[ignore]
fn print_stl_hashes() {
    for (name, mesh) in meshes() {
        println!(
            "STLHASH {name} bin={:016x} ascii={:016x}",
            fnv(&binary_of(mesh)),
            fnv(&ascii_of(mesh))
        );
    }
}

#[test]
fn eps_rows_export_identical_bytes() {
    // Re-exec the printer under each ε row: the STL bytes must be a
    // function of (body, δ) alone (the mesh is ε-row bitwise
    // independent — PR 6; the writer adds nothing).
    let exe = std::env::current_exe().unwrap();
    let mut rows = Vec::new();
    for row in ["1e-6", "1e-9", "1e-12"] {
        // Self re-exec: name the probe by MODULE PATH, not by bare fn name.
        // `tests/all.rs` aggregates every suite into one binary, so libtest
        // sees this probe as `<this_module>::print_stl_hashes`. Stripping the leading
        // crate name off `module_path!()` gives the right filter in the
        // aggregated layout AND in a standalone one (no `::` at all).
        let probe = match module_path!().split_once("::") {
            Some((_, m)) => format!("{m}::print_stl_hashes"),
            None => "print_stl_hashes".to_string(),
        };
        let out = std::process::Command::new(&exe)
            .args([probe.as_str(), "--ignored", "--exact", "--nocapture"])
            .env("CAD_TOLERANCE_EPS", row)
            .output()
            .unwrap();
        assert!(out.status.success(), "row {row} run failed");
        let text = String::from_utf8_lossy(&out.stdout);
        let lines: Vec<&str> = text.lines().filter(|l| l.starts_with("STLHASH")).collect();
        assert!(!lines.is_empty(), "row {row}: printer lines missing");
        rows.push(lines.join(";"));
    }
    assert_eq!(rows[0], rows[1], "stl bytes differ between eps rows");
    assert_eq!(rows[1], rows[2], "stl bytes differ between eps rows");
}

// ---- ascii ↔ binary equivalence --------------------------------------

/// Parse the binary payload into (normal, vertices) f32 tuples.
fn parse_binary(bytes: &[u8]) -> Vec<[f32; 12]> {
    let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    let mut out = Vec::with_capacity(count);
    let mut off = 84;
    for _ in 0..count {
        let mut vals = [0f32; 12];
        for v in &mut vals {
            *v = f32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
            off += 4;
        }
        let attr = u16::from_le_bytes(bytes[off..off + 2].try_into().unwrap());
        assert_eq!(attr, 0, "attribute byte count must be zero");
        off += 2;
        out.push(vals);
    }
    assert_eq!(off, bytes.len(), "trailing bytes after last facet");
    out
}

/// Parse the ASCII grammar into the same tuples (bit-exact f32 parse).
fn parse_ascii(text: &str) -> Vec<[f32; 12]> {
    let mut out = Vec::new();
    let mut current: Vec<f32> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("facet normal ") {
            current = rest
                .split_whitespace()
                .map(|t| t.parse::<f32>().unwrap())
                .collect();
        } else if let Some(rest) = line.strip_prefix("vertex ") {
            current.extend(rest.split_whitespace().map(|t| t.parse::<f32>().unwrap()));
        } else if line == "endfacet" {
            let vals: [f32; 12] = current.clone().try_into().unwrap();
            out.push(vals);
        }
    }
    out
}

#[test]
fn ascii_and_binary_triangle_sets_identical() {
    for (name, mesh) in meshes() {
        let bin = parse_binary(&binary_of(mesh));
        let asc = parse_ascii(std::str::from_utf8(&ascii_of(mesh)).unwrap());
        assert_eq!(bin.len(), asc.len(), "{name}: facet counts differ");
        for (i, (b, a)) in bin.iter().zip(&asc).enumerate() {
            for k in 0..12 {
                assert_eq!(
                    b[k].to_bits(),
                    a[k].to_bits(),
                    "{name}: facet {i} field {k}: binary {} vs ascii {}",
                    b[k],
                    a[k]
                );
            }
        }
    }
}

// ---- format sanity ----------------------------------------------------

#[test]
fn binary_header_is_constant_and_not_solid() {
    let a = binary_of(&meshes()[0].1);
    let b = binary_of(&meshes()[2].1);
    assert_eq!(&a[..80], &b[..80], "header must be input-independent");
    assert!(
        !a.starts_with(b"solid"),
        "binary header must not sniff as ascii"
    );
    let count = u32::from_le_bytes(a[80..84].try_into().unwrap()) as usize;
    assert_eq!(a.len(), 84 + count * 50, "binary facet record size");
}

#[test]
fn normals_are_unit_and_outward_consistent() {
    // Winding-derived normals must be unit (f32 tolerance) and agree
    // with the mesh's positive signed volume via 13.7's flux form:
    // Σ (centroid · n̂) · area > 0.
    for (name, mesh) in meshes() {
        let facets = parse_binary(&binary_of(mesh));
        let mut flux = 0.0f64;
        for f in &facets {
            let n = [f64::from(f[0]), f64::from(f[1]), f64::from(f[2])];
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            assert!((len - 1.0).abs() < 1e-5, "{name}: non-unit normal");
            let c = |k: usize| f64::from(f[3 + k]) + f64::from(f[6 + k]) + f64::from(f[9 + k]);
            // Cross product magnitude for the (double) triangle area:
            let u = [
                f64::from(f[6]) - f64::from(f[3]),
                f64::from(f[7]) - f64::from(f[4]),
                f64::from(f[8]) - f64::from(f[5]),
            ];
            let v = [
                f64::from(f[9]) - f64::from(f[3]),
                f64::from(f[10]) - f64::from(f[4]),
                f64::from(f[11]) - f64::from(f[5]),
            ];
            let cx = [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ];
            let area2 = (cx[0] * cx[0] + cx[1] * cx[1] + cx[2] * cx[2]).sqrt();
            flux += (c(0) / 3.0 * n[0] + c(1) / 3.0 * n[1] + c(2) / 3.0 * n[2]) * area2 / 2.0;
        }
        assert!(flux > 0.0, "{name}: outward flux must be positive");
    }
}

/// Finding (M2 PR 7): at coarse δ the cone's apex fan emits triangles
/// whose three points are exactly collinear on a generator (apex +
/// two stacked meridian chord points — distinct position indices, so
/// the tessellator's id-degenerate drop does not catch them, and
/// `check_mesh` is combinatorial-only). The writer's non-degeneracy
/// contract makes this a typed refusal, never a bad file. Pinned here
/// so a future mesh-side fix flips this test loudly.
#[test]
fn coarse_cone_apex_fan_is_refused_typed() {
    let mesh = tessellate(&common::cone(), 0.05).unwrap();
    let mut out = Vec::new();
    match write_binary(&mesh, &mut out) {
        Err(stl::StlError::DegenerateTriangle { .. }) => {}
        other => panic!("expected DegenerateTriangle at coarse delta, got {other:?}"),
    }
}

#[test]
fn degenerate_mesh_is_refused_typed() {
    // Hand-build a mesh with a geometrically degenerate triangle
    // (distinct indices, collinear points): the writer must fail loud.
    let mesh = mesh::Mesh {
        positions: vec![
            geom_core::Point3::new(0.0, 0.0, 0.0),
            geom_core::Point3::new(1.0, 0.0, 0.0),
            geom_core::Point3::new(2.0, 0.0, 0.0),
        ],
        patches: vec![mesh::FacePatch {
            face: Default::default(),
            triangles: vec![[0, 1, 2]],
        }],
        boundaries: vec![],
    };
    let mut out = Vec::new();
    match write_binary(&mesh, &mut out) {
        Err(stl::StlError::DegenerateTriangle { triangle }) => {
            assert_eq!(triangle, [0, 1, 2]);
        }
        other => panic!("expected DegenerateTriangle, got {other:?}"),
    }
}
