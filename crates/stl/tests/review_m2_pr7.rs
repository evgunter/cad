//! M2 PR 7 e2e review — STL honesty and determinism, checked with an
//! INDEPENDENT parser written against the STL specification (never the
//! writer's own helpers): raw byte layout, header discipline, f32
//! narrowing fidelity, ASCII↔binary bit agreement, normal honesty
//! (winding-derived, unit, outward by parsed signed volume), writer
//! determinism under repetition/interleaving, typed degenerate
//! refusals, and a fresh consumer e2e (vase + bracket) from profile
//! data to parsed-back mesh volume. The caller-settable header's own
//! row lives here too, for the independent parser: it is the only
//! reader in the suite that checks the 80 bytes without using the
//! writer's own view of them.
//!
//! **Two of the jobs in this file do not call an `stl` door at all**,
//! and the file name does not say so:
//! `check_mesh_catches_hand_broken_meshes` exercises `mesh::validate`
//! (the export pre-flight, not the export), and
//! `review_shapes_mesh_volume_within_3_delta_area` is here only
//! because the sweep suite cannot link `mesh` without a dependency
//! cycle. Both are lodgers. The file is also named after a
//! milestone-2 PR review while now carrying the newest public API's
//! pins — left as-is under S36's boundary (milestone naming inside
//! test files is a backlog marker kept until the suite is combed),
//! and recorded here so the accumulation is visible rather than
//! discovered.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "common/mod.rs"]
mod common;

use profile::RawLoop;
use profile::{ProfileLoop, ProfileVertex};
use sweep::{Extrusion, Revolution, extrude, revolve};
use geom_core::Tol;

// ---------------------------------------------------------------------
// Independent parsers (spec-derived).
// ---------------------------------------------------------------------

struct Facet {
    normal: [f32; 3],
    verts: [[f32; 3]; 3],
}

/// Binary STL per spec: 80-byte header, u32 LE count, then per facet
/// 12 LE f32 + u16 attribute count. Asserts the byte layout hard.
fn parse_binary_strict(bytes: &[u8]) -> (Vec<u8>, Vec<Facet>) {
    assert!(bytes.len() >= 84, "truncated header");
    let header = bytes[..80].to_vec();
    let n = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    assert_eq!(
        bytes.len(),
        84 + n * 50,
        "byte length must be exactly 84 + 50·n (no trailer, no padding)"
    );
    let mut facets = Vec::with_capacity(n);
    for i in 0..n {
        let at = 84 + i * 50;
        let f32_at = |o: usize| f32::from_le_bytes(bytes[at + o..at + o + 4].try_into().unwrap());
        let attr = u16::from_le_bytes(bytes[at + 48..at + 50].try_into().unwrap());
        assert_eq!(attr, 0, "attribute byte count must be zero");
        facets.push(Facet {
            normal: [f32_at(0), f32_at(4), f32_at(8)],
            verts: [
                [f32_at(12), f32_at(16), f32_at(20)],
                [f32_at(24), f32_at(28), f32_at(32)],
                [f32_at(36), f32_at(40), f32_at(44)],
            ],
        });
    }
    (header, facets)
}

/// ASCII STL per grammar; floats parsed with `str::parse::<f32>`
/// (bit-exact for shortest-round-trip output).
fn parse_ascii_strict(text: &str) -> Vec<Facet> {
    let mut lines = text.lines();
    let first = lines.next().unwrap();
    assert!(first.starts_with("solid "), "ascii must open with solid");
    let mut facets = Vec::new();
    let mut cur_n = None;
    let mut cur_v: Vec<[f32; 3]> = Vec::new();
    for line in lines {
        let t: Vec<&str> = line.split_whitespace().collect();
        match t.as_slice() {
            ["facet", "normal", x, y, z] => {
                cur_n = Some([x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()]);
            }
            ["vertex", x, y, z] => {
                cur_v.push([x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()]);
            }
            ["endfacet"] => {
                assert_eq!(cur_v.len(), 3, "exactly three vertices per facet");
                facets.push(Facet {
                    normal: cur_n.take().unwrap(),
                    verts: [cur_v[0], cur_v[1], cur_v[2]],
                });
                cur_v.clear();
            }
            ["outer", "loop"] | ["endloop"] => {}
            ["endsolid", ..] => {}
            other => panic!("unexpected ascii stl line: {other:?}"),
        }
    }
    facets
}

fn binary_of(mesh: &mesh::Mesh) -> Vec<u8> {
    let mut out = Vec::new();
    stl::write_binary(mesh, &stl::BinaryOptions::default(), &mut out).unwrap();
    out
}

fn ascii_of(mesh: &mesh::Mesh) -> String {
    let mut out = Vec::new();
    stl::write_ascii(mesh, &stl::AsciiOptions::default(), &mut out).unwrap();
    String::from_utf8(out).unwrap()
}

/// Signed volume of a parsed triangle soup (divergence fan about the
/// origin, in f64 from the f32 vertices).
fn soup_volume(facets: &[Facet]) -> f64 {
    let mut six_v = 0.0;
    for f in facets {
        let p = |v: [f32; 3]| [f64::from(v[0]), f64::from(v[1]), f64::from(v[2])];
        let (a, b, c) = (p(f.verts[0]), p(f.verts[1]), p(f.verts[2]));
        six_v += a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
            + a[2] * (b[0] * c[1] - b[1] * c[0]);
    }
    six_v / 6.0
}

// ---------------------------------------------------------------------
// Byte layout, header, narrowing fidelity, ASCII↔binary agreement, and
// determinism under repetition and interleaving — one build, one test.
// ---------------------------------------------------------------------

/// **The independent-parser laws, on ONE tessellation of the
/// acceptance set.** Three rows until the test-cost audit:
///
/// - `binary_layout_header_and_f32_fidelity` → the `HEADER`, `CAST`,
///   `NORMAL`, `WINDING` and `VOLUME` blocks;
/// - `ascii_and_binary_agree_bitwise_via_independent_parsers` → the
///   `AGREE` block;
/// - `interleaved_and_repeated_exports_are_byte_identical` → the
///   `INTERLEAVE` block.
///
/// # One eleven-body tessellation, every law on it
///
/// One tessellation of the acceptance set serves every block below.
/// nextest's process-per-test isolation means separate rows share
/// nothing, so each would re-pay the whole set — the donut's quadratic
/// CDT included — for parsing work measured in milliseconds. `ball` and
/// `washer` are acceptance rows (2 and 4, δ = 1e-2), taken from the set
/// in hand rather than rebuilt.
///
/// The `INTERLEAVE` block still retessellates `ball` twice on purpose —
/// once at δ and once at 2δ — because "the bytes do not drift across
/// interleaving" is a claim about repeating the pipeline, and the
/// repetition IS the content. Merging must never remove it. It also
/// gets MORE interleaving than before (every other body's exports now
/// run between the first `ball` export and the repeat), which only
/// sharpens the claim.
///
/// What the split bought and a merged row cannot is failure ISOLATION,
/// so every assertion NAMES its law and the message alone says which
/// one broke.
#[test]
fn the_acceptance_exports_parse_back_honestly_and_never_drift() {
    // THE one build. INVARIANT: nothing below calls
    // `acceptance_bodies()` again; the only re-tessellations are the
    // INTERLEAVE block's, which are the content of its claim.
    let acceptance: Vec<(&'static str, topo::Body<f64>, f64, mesh::Mesh)> =
        common::acceptance_bodies()
            .into_iter()
            .map(|(name, body, delta)| {
                let m = mesh::tessellate(&body, delta, Tol::witness()).unwrap();
                (name, body, delta, m)
            })
            .collect();

    for (name, body, delta, mesh) in &acceptance {
        let (name, delta) = (*name, *delta);
        let bytes = binary_of(mesh);
        let (header, facets) = parse_binary_strict(&bytes);
        // ---- HEADER: the shipped DEFAULT header's exact bytes,
        // zero-padded, never "solid". The literal is deliberate — read
        // out of `BinaryOptions::default()` it would pin the writer's
        // agreement with itself rather than the bytes that ship.
        let expected = b"binary STL; CAD kernel tessellation export";
        assert_eq!(
            &header[..expected.len()],
            expected,
            "HEADER: {name}: header text"
        );
        assert!(
            header[expected.len()..].iter().all(|&b| b == 0),
            "HEADER: {name}: header padding must be zeros"
        );
        assert_ne!(
            &header[..5],
            b"solid",
            "HEADER: {name}: header must not sniff as ascii"
        );
        // ---- CAST: triangle stream == mesh order through the
        // documented narrowing cast.
        let mut k = 0;
        for patch in &mesh.patches {
            for tri in &patch.triangles {
                for (j, &i) in tri.iter().enumerate() {
                    let p = mesh.positions[i as usize];
                    let want = [p.x as f32, p.y as f32, p.z as f32];
                    let got = facets[k].verts[j];
                    assert_eq!(
                        got.map(f32::to_bits),
                        want.map(f32::to_bits),
                        "CAST: {name}: facet {k} vertex {j} must be the f32 cast of the \
                         mesh position"
                    );
                }
                k += 1;
            }
        }
        assert_eq!(k, facets.len(), "CAST: {name}: exact facet count");
        // ---- NORMAL / WINDING: unit length (f32-rounded), aligned with
        // the f64 winding normal.
        for (k, f) in facets.iter().enumerate() {
            let n = f.normal.map(f64::from);
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            assert!(
                (len - 1.0).abs() < 1e-6,
                "NORMAL: {name}: facet {k} normal not unit: {len}"
            );
            let p = |v: [f32; 3]| [f64::from(v[0]), f64::from(v[1]), f64::from(v[2])];
            let (a, b, c) = (p(f.verts[0]), p(f.verts[1]), p(f.verts[2]));
            let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
            let w = [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ];
            let dot = n[0] * w[0] + n[1] * w[1] + n[2] * w[2];
            let wlen = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
            // f32-collinear slivers have wlen == 0 from the file's own
            // vertices — the documented narrowing artifact; the stored
            // normal is the f64 truth there and cannot be re-derived.
            if wlen > 0.0 {
                assert!(
                    dot > 0.0,
                    "WINDING: facet {k}: stored normal disagrees with winding ({name})"
                );
            }
        }
        // ---- VOLUME: globally outward, and within 3δA of the exact
        // body volume.
        let vol = soup_volume(&facets);
        let exact = topo::mass_properties(body, Tol::witness()).unwrap();
        assert!(
            (vol - exact.volume).abs() <= 3.0 * delta * exact.surface_area,
            "VOLUME: {name}: parsed-back volume {vol} vs exact {} beyond 3δA",
            exact.volume
        );
        assert!(vol > 0.0, "VOLUME: {name}: shell must be outward");

        // ---- AGREE: the two formats agree bit for bit, read by the two
        // INDEPENDENT parsers above.
        let bin = facets;
        let asc = parse_ascii_strict(&ascii_of(mesh));
        assert_eq!(bin.len(), asc.len(), "AGREE: {name}: facet counts");
        for (k, (b, a)) in bin.iter().zip(&asc).enumerate() {
            assert_eq!(
                b.normal.map(f32::to_bits),
                a.normal.map(f32::to_bits),
                "AGREE: {name}: facet {k} normal bits"
            );
            for j in 0..3 {
                assert_eq!(
                    b.verts[j].map(f32::to_bits),
                    a.verts[j].map(f32::to_bits),
                    "AGREE: {name}: facet {k} vertex {j} bits"
                );
            }
        }
    }

    // ---- INTERLEAVE: the writer's output does not drift across
    // repetition or interleaving. `ball` and `washer` are acceptance
    // rows 2 and 4 (both δ = 1e-2), taken out of the set above; the
    // RE-tessellations below are deliberate.
    let row = |want: &str| -> &(&'static str, topo::Body<f64>, f64, mesh::Mesh) {
        acceptance
            .iter()
            .find(|(n, _, _, _)| *n == want)
            .unwrap_or_else(|| panic!("the acceptance set carries {want}"))
    };
    let (_, ball, delta, m1) = row("ball");
    let (_, _, _, w1) = row("washer");
    let delta = *delta;
    let b_first = binary_of(m1);
    let a_first = ascii_of(m1);
    // Interleave other exports, different δ, then repeat.
    let m_coarse = mesh::tessellate(ball, delta * 2.0, Tol::witness()).unwrap();
    let _ = binary_of(&m_coarse);
    let _ = binary_of(w1);
    let _ = ascii_of(w1);
    let m2 = mesh::tessellate(ball, delta, Tol::witness()).unwrap();
    assert_eq!(
        binary_of(&m2),
        b_first,
        "INTERLEAVE: binary bytes drift across interleaving"
    );
    assert_eq!(
        ascii_of(&m2),
        a_first,
        "INTERLEAVE: ascii bytes drift across interleaving"
    );
    // δ change actually changed the output (the determinism assertion
    // above is not vacuous).
    assert_ne!(
        binary_of(&m_coarse),
        b_first,
        "INTERLEAVE: a δ change must change the bytes"
    );
}

/// Rust `Display` for f32 is shortest-round-trip: spot-check the
/// adversarial corners the ASCII writer relies on (subnormals, the
/// 2^24 integer boundary, near-overflow, negative zero).
#[test]
fn f32_display_round_trip_spot_checks() {
    let cases: [f32; 10] = [
        f32::MIN_POSITIVE, // smallest normal
        1e-45,             // smallest subnormal
        -4.2e-45,          // negative subnormal
        16_777_216.0,      // 2^24
        16_777_215.0,
        3.402_823_5e38, // f32::MAX
        -0.0,
        0.1,
        core::f32::consts::PI,
        -1.000_000_1,
    ];
    for v in cases {
        let s = format!("{v}");
        let back: f32 = s.parse().unwrap();
        assert_eq!(back.to_bits(), v.to_bits(), "{v} formatted as {s}");
    }
}

// ---------------------------------------------------------------------
// Typed refusals.
// ---------------------------------------------------------------------

/// The implementer's live finding, reproduced: the acceptance cone at
/// δ = 0.05 emits an exactly collinear apex-fan triangle; the writer
/// must refuse typed — never a zeroed/guessed normal.
#[test]
fn coarse_cone_apex_fan_refuses_typed() {
    let mesh = mesh::tessellate(&common::cone(), 0.05, Tol::witness()).unwrap();
    let mut out = Vec::new();
    match stl::write_binary(&mesh, &stl::BinaryOptions::default(), &mut out) {
        Err(stl::StlError::DegenerateTriangle { .. }) => {}
        other => panic!("expected DegenerateTriangle, got {other:?}"),
    }
    let mut out = Vec::new();
    match stl::write_ascii(&mesh, &stl::AsciiOptions::default(), &mut out) {
        Err(stl::StlError::DegenerateTriangle { .. }) => {}
        other => panic!("expected DegenerateTriangle (ascii), got {other:?}"),
    }
}

#[test]
fn corrupt_index_refuses_typed() {
    let mut mesh = mesh::tessellate(&common::l_prism(), 1e-2, Tol::witness()).unwrap();
    let n = mesh.positions.len() as u32;
    mesh.patches[0].triangles[0][1] = n + 7;
    let mut out = Vec::new();
    match stl::write_binary(&mesh, &stl::BinaryOptions::default(), &mut out) {
        Err(stl::StlError::IndexOutOfRange { index }) => assert_eq!(index, n + 7),
        other => panic!("expected IndexOutOfRange, got {other:?}"),
    }
}

/// **The caller-settable header, through the same independent parser.**
/// A supplied header lands in the 80 bytes zero-padded, nothing after
/// byte 80 moves, and the two constraints the format imposes on it —
/// 80 bytes, and never sniffing as ASCII STL — are typed refusals
/// rather than truncation or a silently misread file.
#[test]
fn a_caller_supplied_header_lands_padded_and_its_limits_refuse_typed() {
    let mesh = mesh::tessellate(&common::l_prism(), 1e-2, Tol::witness()).unwrap();
    let default_bytes = binary_of(&mesh);

    let mut out = Vec::new();
    stl::write_binary(
        &mesh,
        &stl::BinaryOptions {
            header: stl::BinaryHeader::new("widget-7 rev C").unwrap(),
        },
        &mut out,
    )
    .unwrap();
    let (header, _) = parse_binary_strict(&out);
    let expected = b"widget-7 rev C";
    assert_eq!(
        &header[..expected.len()],
        expected,
        "HEADER/OPTIONS: the caller's header text"
    );
    assert!(
        header[expected.len()..].iter().all(|&b| b == 0),
        "HEADER/OPTIONS: padding must still be zeros"
    );
    assert_eq!(
        &out[80..],
        &default_bytes[80..],
        "HEADER/OPTIONS: only the header moves"
    );

    // The limits are refused at CONSTRUCTION now, so these rows call
    // `BinaryHeader::new` rather than the writer: a header that exists
    // is one the writer can emit, and "refused before any byte is
    // written" stopped being a property to test and became one that
    // cannot be violated.
    //
    // 80 bytes exactly is the largest header that fits; 81 refuses.
    assert!(
        stl::BinaryHeader::new("x".repeat(80)).is_ok(),
        "HEADER/OPTIONS: 80 bytes is the boundary and must fit"
    );
    match stl::BinaryHeader::new("x".repeat(81)) {
        Err(stl::BinaryHeaderError::TooLong { len }) => assert_eq!(len, 81),
        other => panic!("expected TooLong, got {other:?}"),
    }

    // The sniff constraint the writer used to satisfy by construction
    // is now enforced on every header, over the WHOLE class a
    // whitespace-skipping, case-folding reader recognises. Each
    // refused row below is a spelling a byte-exact `starts_with`
    // check would let through, so narrowing the predicate reddens
    // here rather than only deleting it doing so.
    let header_sniffs = |header: &str| -> bool {
        match stl::BinaryHeader::new(header) {
            Ok(_) => false,
            Err(stl::BinaryHeaderError::SniffsAscii) => true,
            Err(other) => panic!("HEADER/OPTIONS: unexpected refusal for {header:?}: {other:?}"),
        }
    };
    for (header, want) in [
        ("solid widget", true),
        // No trailing space: a `starts_with(b"solid ")` narrowing
        // passes this and is caught here.
        ("solid-block", true),
        ("solid", true),
        ("Solid widget", true),
        ("SOLID widget", true),
        (" solid widget", true),
        ("\tsolid widget", true),
        ("SolidWorks export", true),
        // Not the keyword, and must stay writable — so a mutation that
        // over-widens the check (matching any header containing
        // "solid", say) reddens too.
        ("consolidated frame", false),
        ("soli", false),
        ("binary STL; widget-7", false),
        ("", false),
    ] {
        assert_eq!(
            header_sniffs(header),
            want,
            "HEADER/OPTIONS: {header:?} must {} as ascii",
            if want {
                "be refused as sniffing"
            } else {
                "NOT sniff"
            }
        );
    }
}

/// Hand-broken meshes: `check_mesh` (the export pre-flight the CI
/// example runs) must catch a deleted triangle (open boundary), a
/// duplicated triangle (over-used edge), and a flipped triangle
/// (mismatched adjacent orientation).
#[test]
fn check_mesh_catches_hand_broken_meshes() {
    let good = mesh::tessellate(&common::l_prism(), 1e-2, Tol::witness()).unwrap();
    assert!(mesh::validate::check_mesh(&good).is_ok());
    // (a) hole: drop one triangle.
    let mut holed = good.clone();
    holed.patches[0].triangles.pop().unwrap();
    assert!(
        mesh::validate::check_mesh(&holed).is_err(),
        "deleting a triangle must break watertightness"
    );
    // (b) duplicate one triangle.
    let mut dup = good.clone();
    let t = dup.patches[0].triangles[0];
    dup.patches[0].triangles.push(t);
    assert!(
        mesh::validate::check_mesh(&dup).is_err(),
        "duplicating a triangle must break 2-manifoldness"
    );
    // (c) flip one triangle's winding.
    let mut flipped = good.clone();
    flipped.patches[0].triangles[0].swap(0, 2);
    assert!(
        mesh::validate::check_mesh(&flipped).is_err(),
        "a flipped triangle must break orientation coherence"
    );
}

// ---------------------------------------------------------------------
// Fresh-consumer e2e (assignment 9): vase + bracket, public API only.
// ---------------------------------------------------------------------

#[test]
fn consumer_e2e_vase_and_bracket() {
    let delta = 1e-2;
    // Vase: revolved profile with an arc belly (cylinder foot, sphere
    // belly, cylinder neck... kept in the M2 inventory: lines + arc).
    let mut vase_profile = ProfileLoop::new(vec![
        ProfileVertex::new(geom_core::Point2::new(0.0, 0.0), 0.0),
        ProfileVertex::new(geom_core::Point2::new(0.8, 0.0), 0.0),
        // quarter-arc belly
        ProfileVertex::new(
            geom_core::Point2::new(0.8, 0.4),
            (core::f64::consts::PI / 8.0).tan(),
        ),
        ProfileVertex::new(geom_core::Point2::new(1.2, 0.8), 0.0),
        ProfileVertex::new(geom_core::Point2::new(1.2, 1.4), 0.0),
        ProfileVertex::new(geom_core::Point2::new(0.0, 1.4), 0.0),
    ]);
    // The sphere belly blends tangentially into the neck cylinder at
    // (1.2, 0.8) -- intended smooth blend, declared (#101).
    vase_profile = vase_profile.with_tangent_joints(vec![3]);
    let vase = revolve(
        &common::validated(vec![vase_profile]),
        common::axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    // Bracket: L-plate with two round bolt holes.
    let outer = ProfileLoop::polygon([
        common::p2(0.0, 0.0),
        common::p2(3.0, 0.0),
        common::p2(3.0, 1.0),
        common::p2(1.0, 1.0),
        common::p2(1.0, 3.0),
        common::p2(0.0, 3.0),
    ]);
    let hole = |cx: f64, cy: f64, r: f64| {
        ProfileLoop::new(vec![
            ProfileVertex::new(geom_core::Point2::new(cx + r, cy), 1.0),
            ProfileVertex::new(geom_core::Point2::new(cx - r, cy), 1.0),
        ])
    };
    let bracket = extrude(
        &common::validated(vec![outer, hole(2.2, 0.5, 0.25), hole(0.5, 2.2, 0.25)]),
        Extrusion::Distance(0.5),
        Tol::witness(),
    )
    .unwrap()
    .body;
    let outdir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("review_e2e");
    std::fs::create_dir_all(&outdir).unwrap();
    for (name, body) in [("vase", &vase), ("bracket", &bracket)] {
        topo::validate(body).unwrap();
        topo::validate_closed(body).unwrap();
        topo::validate_geometric(body, Tol::witness()).unwrap();
        let props = topo::mass_properties(body, Tol::witness()).unwrap();
        assert!(props.volume > 0.0 && props.surface_area > 0.0);
        let mesh = mesh::tessellate(body, delta, Tol::witness()).unwrap();
        mesh::validate::check_mesh(&mesh).unwrap();
        // Export both formats to disk, re-read, re-derive volume.
        let bin_path = outdir.join(format!("{name}.stl"));
        let asc_path = outdir.join(format!("{name}.ascii.stl"));
        stl::write_binary(
            &mesh,
            &stl::BinaryOptions::default(),
            &mut std::fs::File::create(&bin_path).unwrap(),
        )
        .unwrap();
        stl::write_ascii(
            &mesh,
            &stl::AsciiOptions::default(),
            &mut std::fs::File::create(&asc_path).unwrap(),
        )
        .unwrap();
        let bytes = std::fs::read(&bin_path).unwrap();
        let (_, facets) = parse_binary_strict(&bytes);
        let v = soup_volume(&facets);
        assert!(
            (v - props.volume).abs() <= 3.0 * delta * props.surface_area,
            "{name}: parsed volume {v} vs exact {} beyond 3δA",
            props.volume
        );
        let asc = parse_ascii_strict(&std::fs::read_to_string(&asc_path).unwrap());
        assert_eq!(asc.len(), facets.len());
    }
}

/// Mesh-volume 3δA cross-checks for the review shapes of the sweep
/// suite (which cannot link `mesh` without a dependency cycle).
#[test]
fn review_shapes_mesh_volume_within_3_delta_area() {
    let delta = 1e-2;
    let shapes: Vec<(&str, topo::Body<f64>)> = vec![
        (
            "frustum",
            revolve(
                &common::validated(vec![ProfileLoop::polygon([
                    common::p2(1.0, 0.0),
                    common::p2(2.0, 0.0),
                    common::p2(1.0, 1.0),
                ])]),
                common::axis_y(),
                Revolution::Full,
                Tol::witness(),
            )
            .unwrap()
            .body,
        ),
        (
            "cup",
            revolve(
                &common::validated(vec![ProfileLoop::polygon([
                    common::p2(0.0, 0.0),
                    common::p2(2.0, 0.0),
                    common::p2(2.0, 2.0),
                    common::p2(1.5, 2.0),
                    common::p2(1.5, 0.5),
                    common::p2(0.0, 0.5),
                ])]),
                common::axis_y(),
                Revolution::Full,
                Tol::witness(),
            )
            .unwrap()
            .body,
        ),
        (
            "quarter_donut",
            revolve(
                &common::validated(vec![ProfileLoop::new(vec![
                    ProfileVertex::new(geom_core::Point2::new(2.0, -0.5), 1.0),
                    ProfileVertex::new(geom_core::Point2::new(2.0, 0.5), 1.0),
                ])]),
                common::axis_y(),
                Revolution::Partial(core::f64::consts::FRAC_PI_2),
                Tol::witness(),
            )
            .unwrap()
            .body,
        ),
    ];
    for (name, body) in &shapes {
        let props = topo::mass_properties(body, Tol::witness()).unwrap();
        let mesh = mesh::tessellate(body, delta, Tol::witness()).unwrap();
        mesh::validate::check_mesh(&mesh).unwrap();
        let vm = mesh::validate::signed_volume(&mesh);
        assert!(
            (vm - props.volume).abs() <= 3.0 * delta * props.surface_area,
            "{name}: mesh volume {vm} vs exact {} beyond 3δA",
            props.volume
        );
    }
}
