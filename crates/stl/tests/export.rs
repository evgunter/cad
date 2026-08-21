//! M2 PR 7 STL acceptance: byte-identity (D9), ASCII↔binary triangle
//! equivalence under parse-back, and format sanity.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom_core::Tol;
use mesh::tessellate;
use stl::{AsciiOptions, BinaryOptions, SolidName, write_ascii, write_binary};

fn fnv(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// The acceptance meshes — the donut pays the documented quadratic CDT
/// cost, see `common::acceptance_bodies` for the per-body δ. The
/// writers under test are pure functions of the mesh, so one build
/// serves every claim made about them.
///
/// **No memo, and that is the point.** nextest is process-per-test, so
/// a memo would share nothing across tests and every row that touched
/// it would pay the whole eleven-body tessellation. The rows that
/// share it are ONE test, which binds this list once; the only other
/// caller is the `#[ignore]`d hash printer, which runs in its own
/// process anyway.
fn meshes() -> Vec<(&'static str, mesh::Mesh)> {
    common::acceptance_bodies()
        .into_iter()
        .map(|(name, body, delta)| (name, tessellate(&body, delta, Tol::witness()).unwrap()))
        .collect()
}

fn binary_of(mesh: &mesh::Mesh) -> Vec<u8> {
    let mut out = Vec::new();
    write_binary(mesh, &BinaryOptions::default(), &mut out).unwrap();
    out
}

fn ascii_of(mesh: &mesh::Mesh) -> Vec<u8> {
    let mut out = Vec::new();
    write_ascii(mesh, &AsciiOptions::default(), &mut out).unwrap();
    out
}

// ---- the ε-row / cross-profile byte oracle ----------------------------

/// Shell-driven cross-profile / cross-ε oracle: prints one FNV line
/// per body. Run under debug and release (and any ε row) and diff —
/// the same mechanism as the mesh determinism suite.
#[test]
#[ignore]
fn print_stl_hashes() {
    for (name, mesh) in meshes() {
        println!(
            "STLHASH {name} bin={:016x} ascii={:016x}",
            fnv(&binary_of(&mesh)),
            fnv(&ascii_of(&mesh))
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

// ---- the shared-mesh laws: ascii↔binary equivalence, normal honesty,
//      header discipline, byte identity ---------------------------------

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

/// **The writer laws that share one tessellation of the acceptance
/// set**: the `AGREE` block; the `NORMAL` / `FLUX` blocks; the
/// `HEADER` block, with the `SOLID NAME` / `NAME/OPTIONS` blocks that
/// pin the ASCII name's shipped default and its caller-settable path;
/// and the `BYTE-IDENTITY` block.
///
/// # One tessellation of eleven bodies, every law on it
///
/// They all read the same acceptance meshes, and nextest runs one
/// process per test — so as separate rows the eleven-body
/// tessellation (the donut's quadratic CDT included) would be paid
/// once per row per ε row. It runs once here.
///
/// The `BYTE-IDENTITY` block still rebuilds every body from its recipe
/// and retessellates — that second build is the CONTENT of the claim,
/// not duplicated setup, and merging must never remove it.
///
/// What a merged row cannot buy is failure ISOLATION, so every
/// assertion NAMES its law — `AGREE`, `NORMAL`, `FLUX`, `HEADER`,
/// `NAME`, `NAME/OPTIONS`, `BYTE-IDENTITY` — and the message alone
/// says which one broke.
#[test]
fn the_acceptance_exports_agree_are_honest_and_are_byte_identical() {
    // THE one tessellation. INVARIANT: nothing below may call
    // `meshes()` or `acceptance_bodies()` again except the
    // BYTE-IDENTITY block, whose whole point is the independent
    // rebuild.
    let meshes = meshes();

    for (name, mesh) in &meshes {
        let bin_bytes = binary_of(mesh);
        let bin = parse_binary(&bin_bytes);

        // ---- AGREE: the two formats carry the same triangle set, bit
        // for bit.
        let asc = parse_ascii(std::str::from_utf8(&ascii_of(mesh)).unwrap());
        assert_eq!(bin.len(), asc.len(), "AGREE: {name}: facet counts differ");
        for (i, (b, a)) in bin.iter().zip(&asc).enumerate() {
            for k in 0..12 {
                assert_eq!(
                    b[k].to_bits(),
                    a[k].to_bits(),
                    "AGREE: {name}: facet {i} field {k}: binary {} vs ascii {}",
                    b[k],
                    a[k]
                );
            }
        }

        // ---- NORMAL / FLUX: winding-derived normals must be unit (f32
        // tolerance) and agree with the mesh's positive signed volume
        // via 13.7's flux form: Σ (centroid · n̂) · area > 0.
        let mut flux = 0.0f64;
        for f in &bin {
            let n = [f64::from(f[0]), f64::from(f[1]), f64::from(f[2])];
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            assert!((len - 1.0).abs() < 1e-5, "NORMAL: {name}: non-unit normal");
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
        assert!(flux > 0.0, "FLUX: {name}: outward flux must be positive");
    }

    // ---- HEADER: two DIFFERENT bodies must produce the same 80-byte
    // DEFAULT header, it must not sniff as ascii, and the record size
    // must be exactly 84 + 50·n. The two are the "l_prism" and "ball" rows of
    // `common::acceptance_bodies` (rows 0 and 2, both at δ = 1e-2) —
    // taken out of the list above rather than rebuilt.
    let by_name = |want: &str| -> &mesh::Mesh {
        meshes
            .iter()
            .find(|(n, _)| *n == want)
            .map(|(_, m)| m)
            .unwrap_or_else(|| panic!("the acceptance set carries {want}"))
    };
    let a = binary_of(by_name("l_prism"));
    let b = binary_of(by_name("ball"));
    assert_eq!(
        &a[..80],
        &b[..80],
        "HEADER: header must be input-independent"
    );
    assert!(
        !a.starts_with(b"solid"),
        "HEADER: binary header must not sniff as ascii"
    );
    let count = u32::from_le_bytes(a[80..84].try_into().unwrap()) as usize;
    assert_eq!(a.len(), 84 + count * 50, "HEADER: binary facet record size");

    // ---- SOLID NAME: the ASCII writer's `solid <name>` opener is a
    // SHIPPED byte string with no other coverage — the byte oracles
    // above compare exports to each other, so a changed name is
    // invisible to them. Pin the exact DEFAULT text, its
    // input-independence, and that `endsolid` closes on the same name.
    // The expected strings are literals on purpose: sourcing them from
    // `AsciiOptions::default()` would pin the writer's agreement with
    // itself, not the bytes the library ships.
    let ascii_a = String::from_utf8(ascii_of(by_name("l_prism"))).expect("NAME: ascii is utf-8");
    let ascii_b = String::from_utf8(ascii_of(by_name("ball"))).expect("NAME: ascii is utf-8");
    for (which, text) in [("l_prism", &ascii_a), ("ball", &ascii_b)] {
        let first = text.lines().next().expect("NAME: ascii export is empty");
        assert_eq!(
            first, "solid part",
            "NAME: {which}: the shipped default solid name"
        );
        let last = text
            .lines()
            .next_back()
            .expect("NAME: ascii export is empty");
        assert_eq!(
            last, "endsolid part",
            "NAME: {which}: endsolid must close on the same name"
        );
    }

    // ---- NAME/OPTIONS: a caller-supplied name reaches BOTH the opener
    // and the closer, and nothing else in the file moves. The tail
    // comparison is what makes this a claim about the name rather than
    // about the whole file: everything after the first line is
    // byte-identical to the default export.
    let named = {
        let mut out = Vec::new();
        write_ascii(
            by_name("l_prism"),
            &AsciiOptions {
                solid_name: SolidName::new("widget-7").unwrap(),
            },
            &mut out,
        )
        .unwrap();
        String::from_utf8(out).expect("NAME/OPTIONS: ascii is utf-8")
    };
    assert_eq!(
        named.lines().next().unwrap(),
        "solid widget-7",
        "NAME/OPTIONS: the caller's name opens the solid"
    );
    assert_eq!(
        named.lines().next_back().unwrap(),
        "endsolid widget-7",
        "NAME/OPTIONS: endsolid closes on the caller's name"
    );
    let body_of = |text: &str| -> String {
        text.lines()
            .skip(1)
            .take_while(|l| !l.starts_with("endsolid"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(
        body_of(&named),
        body_of(&ascii_a),
        "NAME/OPTIONS: only the solid name moves"
    );
    // The admissible set is `0x20..=0x7E`, and BOTH bounds are pinned
    // plus the two characters just outside them. A row that only
    // covered `'\n'` could not see the range widened — dropping the
    // upper bound would make DEL and every non-ASCII character
    // writable with nothing going red.
    //
    // The verdict is `SolidName::new`'s, not the writer's: the name is
    // validated where it is built, so a name that exists is one the
    // writer can emit. The row below still proves the ADMISSIBLE names
    // reach a file — each `Ok` case is written through `write_ascii`
    // as well — so the two halves cannot drift apart.
    let name_verdict = |name: &str| -> Result<(), char> {
        match SolidName::new(name) {
            Ok(validated) => {
                write_ascii(
                    by_name("l_prism"),
                    &AsciiOptions {
                        solid_name: validated,
                    },
                    &mut Vec::new(),
                )
                .expect("NAME/OPTIONS: an admissible name must reach a file");
                Ok(())
            }
            Err(stl::SolidNameError::Unrepresentable { character }) => Err(character),
        }
    };
    for (name, want) in [
        ("two\nlines", Err('\n')),
        ("bell\u{7}", Err('\u{7}')),
        ("del\u{7f}", Err('\u{7f}')),
        ("caf\u{e9}", Err('\u{e9}')),
        ("line\u{2028}sep", Err('\u{2028}')),
        ("emoji\u{1f600}", Err('\u{1f600}')),
        // The bounds themselves, and the empty name, are ADMISSIBLE —
        // so a mutation that narrows the range reddens here rather
        // than silently refusing legal names.
        (" ", Ok(())),
        ("~", Ok(())),
        ("", Ok(())),
        ("part 7 (rev C) — no: ASCII only", Err('\u{2014}')),
    ] {
        assert_eq!(
            name_verdict(name),
            want,
            "NAME/OPTIONS: {name:?} must be {}",
            if want.is_ok() { "written" } else { "refused" }
        );
    }

    // ---- BYTE-IDENTITY: repeat-call identity through the FULL
    // pipeline — rebuild the body, retessellate, rewrite; bytes must
    // match the first pipeline's output exactly.
    for (name, body, delta) in common::acceptance_bodies() {
        // INVARIANT: both arms compare the FIRST pipeline's mesh
        // against this freshly rebuilt one — the single retessellation
        // here serves both writers, and comparing a writer's output to
        // itself would assert nothing.
        let rebuilt_mesh = tessellate(&body, delta, Tol::witness()).unwrap();
        let rebuilt = binary_of(&rebuilt_mesh);
        let first = by_name(name);
        let a = binary_of(first);
        assert_eq!(fnv(&a), fnv(&rebuilt), "BYTE-IDENTITY: {name}: fnv");
        assert_eq!(
            a, rebuilt,
            "BYTE-IDENTITY: {name}: binary export not byte-identical"
        );
        assert_eq!(
            ascii_of(first),
            ascii_of(&rebuilt_mesh),
            "BYTE-IDENTITY: {name}: ascii export not byte-identical"
        );
    }
}

// ---- typed refusals ---------------------------------------------------

/// Finding (M2 PR 7): at coarse δ the cone's apex fan emits triangles
/// whose three points are exactly collinear on a generator (apex +
/// two stacked meridian chord points — distinct position indices, so
/// the tessellator's id-degenerate drop does not catch them, and
/// `check_mesh` is combinatorial-only). The writer's non-degeneracy
/// contract makes this a typed refusal, never a bad file. Pinned here
/// so a future mesh-side fix flips this test loudly.
#[test]
fn coarse_cone_apex_fan_is_refused_typed() {
    let mesh = tessellate(&common::cone(), 0.05, Tol::witness()).unwrap();
    let mut out = Vec::new();
    match write_binary(&mesh, &BinaryOptions::default(), &mut out) {
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
    match write_binary(&mesh, &BinaryOptions::default(), &mut out) {
        Err(stl::StlError::DegenerateTriangle { triangle }) => {
            assert_eq!(triangle, [0, 1, 2]);
        }
        other => panic!("expected DegenerateTriangle, got {other:?}"),
    }
}
