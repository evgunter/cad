//! Wild-corpus montage generator: imports the license-cleared wild
//! STEP fixtures (files nobody on this project authored) through
//! `step-import`, tessellates each body with the kernel's own
//! tessellator, and exports STL + a `scenes.json` manifest for
//! `demos/render-wild.sh` — the KERNEL-TESSELLATION lane only, by
//! Evan-approved scope (no FreeCAD import, no OCC comparison lane).
//!
//! **The cell set is LAW, not discovery** — `docs/WILD-CORPUS-LICENSES.md`
//! is the license audit that governs it:
//!
//!   * only files the audit marks render-OK may appear on the sheet.
//!     The four `stepcode/` fixtures are EXCLUDED there (D2: unclear
//!     upstream rights for redistributed CAx-IF models) and this tool
//!     does not read them AT ALL — `sg1-c5-214.stp` imports fine and
//!     is still excluded, by license rather than by capability;
//!   * every render-OK file is attempted. The ones that import AND
//!     tessellate become cells ([`WILD_CELLS`], pinned); the one that
//!     refuses import ([`WILD_REFUSALS`]: `b123d_nema17_bracket`,
//!     `SURFACE_CURVE` edge geometry) is pinned as a typed failure.
//!     **Drift in any direction fails this tool loudly**: a montage
//!     whose cell set moved silently would detach the sheet from the
//!     audit's attribution block (`demos/README.md` carries it). If a
//!     pinned failure starts working, add the cell AND extend the
//!     attribution block per the audit's own instructions, in the
//!     same change.
//!
//! The audit's import-status snapshot ("only 6 import today") predates
//! the M7-5 band-seam re-mint (#252), which flipped
//! `nist_ftc_11_asme1_rb` and `cq_red_cube_blue_cylinder` to
//! imports-class — so eight of the nine render-OK files import today,
//! all eight render-OK in the audit's own table (its "YES (moot)" rows
//! are moot no longer). Both arrivals stay inside the attribution
//! block's law: nist_ftc_11 rides the NIST terms, and the CadQuery
//! file's Apache-2.0 entry follows the audit's explicit extension
//! instructions.
//!
//! Usage: `cargo run --release -- <outdir>` (from `demos/wild/`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::mesh::validate::{check_mesh, signed_volume, triangle_count};
use pncad::step_import::{ImportOptions, StepImport, import_step};
use pncad::topo::Body;

/// One montage cell: a render-OK wild fixture that imports first-class.
struct Cell {
    /// Fixture path under `crates/step-import/tests/fixtures/wild/`.
    fixture: &'static str,
    /// Scene name (= STL/PNG stem).
    name: &'static str,
    /// Montage caption — the honest label: vein/file + license.
    caption: &'static str,
    /// Base RGB for the renderer.
    color: [f64; 3],
    /// View (elev, azim, up) — the classic matplotlib spec.
    view: (f64, f64, char),
    /// Chordal tolerance as a fraction of the body's bbox diagonal.
    /// 1.2e-3 is the lane default; `nist_ftc_11` is coarsened to
    /// 6e-3 because its four torus/cylinder band faces at the default
    /// cost ~143k triangles (~13 min of matplotlib zsort per render
    /// pass) for a montage cell — at 6e-3 the silhouette is the same
    /// and the volume error stays in the other cells' 0.2% class.
    delta_frac: f64,
}

/// **The pinned cell set.** Derivation (audit table + `wild.rs` + the
/// tessellation record in the module docs): 13 wild fixtures − 4
/// stepcode (license-EXCLUDED, D2) = 9 render-OK, − 1 typed import
/// refusal (`b123d_nema17_bracket`, [`WILD_REFUSALS`]) = **8**.
///
/// The derivation's answer is stated once, as this array's own length:
/// a cell added or dropped without re-deriving is a type error here,
/// and every other drift fails loudly below. No count is written in
/// prose a second time, because the second copy is the one that rots.
const WILD_CELLS: [Cell; 8] = [
    Cell {
        fixture: "adafruit/64_Halfsize_Breadboard.step",
        name: "64_Halfsize_Breadboard",
        caption: "adafruit/64_Halfsize_Breadboard.step (MIT)",
        color: [0.84, 0.84, 0.80],
        view: (38.0, -55.0, 'z'),
        delta_frac: 1.2e-3,
    },
    Cell {
        fixture: "adafruit/328_2500mAh_battery.step",
        name: "328_2500mAh_battery",
        caption: "adafruit/328_2500mAh_battery.step (MIT)",
        color: [0.66, 0.62, 0.55],
        view: (32.0, -55.0, 'z'),
        delta_frac: 1.2e-3,
    },
    Cell {
        fixture: "adafruit/805_slide_switch.step",
        name: "805_slide_switch",
        caption: "adafruit/805_slide_switch.step (MIT)",
        color: [0.82, 0.36, 0.30],
        view: (32.0, -55.0, 'z'),
        delta_frac: 1.2e-3,
    },
    Cell {
        fixture: "adafruit/931_OLED_128x32_I2C.step",
        name: "931_OLED_128x32_I2C",
        caption: "adafruit/931_OLED_128x32_I2C.step (MIT)",
        color: [0.36, 0.62, 0.82],
        view: (32.0, -55.0, 'z'),
        delta_frac: 1.2e-3,
    },
    Cell {
        fixture: "adafruit/1982_MPR121.step",
        name: "1982_MPR121",
        caption: "adafruit/1982_MPR121.step (MIT)",
        color: [0.30, 0.52, 0.46],
        view: (32.0, -55.0, 'z'),
        delta_frac: 1.2e-3,
    },
    Cell {
        fixture: "nist/nist_ftc_09_asme1_rd.stp",
        name: "nist_ftc_09_asme1_rd",
        caption: "nist/nist_ftc_09_asme1_rd.stp (NIST, PD)",
        color: [0.62, 0.66, 0.72],
        view: (30.0, -50.0, 'z'),
        delta_frac: 1.2e-3,
    },
    Cell {
        fixture: "nist/nist_ftc_11_asme1_rb.stp",
        name: "nist_ftc_11_asme1_rb",
        caption: "nist/nist_ftc_11_asme1_rb.stp (NIST, PD)",
        color: [0.55, 0.62, 0.74],
        view: (30.0, -50.0, 'z'),
        delta_frac: 6e-3,
    },
    Cell {
        fixture: "occ-oss/cq_red_cube_blue_cylinder.step",
        name: "cq_red_cube_blue_cylinder",
        caption: "occ-oss/cq_red_cube_blue_cylinder.step (Apache-2.0)",
        color: [0.62, 0.46, 0.68],
        view: (28.0, -55.0, 'z'),
        delta_frac: 1.2e-3,
    },
];

/// Render-OK fixtures pinned as typed IMPORT refusals (fixture,
/// message substring — the class, matching `wild.rs`'s own pin). If
/// one of these imports, the cell set has grown: extend
/// [`WILD_CELLS`] AND the attribution block in `demos/README.md`,
/// per the audit.
const WILD_REFUSALS: [(&str, &str); 1] = [("occ-oss/b123d_nema17_bracket.step", "SURFACE_CURVE")];

fn fixture_text(rel: &str) -> String {
    let path = format!(
        "{}/../../crates/step-import/tests/fixtures/wild/{rel}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {path}: {e}"))
}

/// Topology census + genus (the tour's own narration identity).
fn census(body: &Body<f64>) -> (usize, usize, usize, usize, usize, i64) {
    let v = body.vertices().count();
    let e = body.edges().count();
    let f = body.faces().count();
    let r: usize = body.faces().map(|(_, face)| face.rings.len()).sum();
    let s = body.shells().count();
    let genus = s as i64 - (v as i64 - e as i64 + f as i64 - r as i64) / 2;
    (v, e, f, r, s, genus)
}

/// Import + tessellate + export one cell; returns its manifest entry.
fn run_cell(cell: &Cell, outdir: &str) -> String {
    let name = cell.name;
    let text = fixture_text(cell.fixture);
    let import = import_step(&text, &ImportOptions::default()).unwrap_or_else(|e| {
        panic!(
            "{name}: {} is a PINNED montage cell and must import; it refused: {e}. \
             The cell set drifted — reconcile WILD_CELLS with the license audit \
             (docs/WILD-CORPUS-LICENSES.md) and the attribution block together.",
            cell.fixture
        )
    });
    let StepImport::Solid {
        body,
        eps_in,
        normalizations,
        ..
    } = import
    else {
        panic!("{name}: expected a solid import, got a wireframe");
    };

    let (v, e, f, r, s, genus) = census(&body);
    println!(
        "   [{name}] imported first-class: {v} vertices, {e} edges, {f} faces, \
         {r} rings, {s} shell(s) -> genus {genus}; eps_in = {eps_in:e} m"
    );
    if normalizations.is_empty() {
        println!("   [{name}] structure normalizations: none (boundary graph adopted as stated)");
    } else {
        for n in &normalizations {
            println!("   [{name}] structure normalization: {n:?}");
        }
    }

    // Exact B-rep mass properties, then the kernel's own tessellation
    // with a chordal tolerance sized off the body's real extent (bbox
    // of the body's stored points — vertex points and control points,
    // whose hull bounds every NURBS patch). The wild has no closed
    // forms; the exact-vs-mesh volume row is the same end-to-end
    // sanity ribbon the tour prints.
    let props = pncad::topo::mass_properties(&body)
        .unwrap_or_else(|e| panic!("{name}: imported but has no volume: {e:?}"));
    let (lo, hi) = body.points().fold(
        ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]),
        |(lo, hi), (_, p)| {
            (
                [lo[0].min(p.x), lo[1].min(p.y), lo[2].min(p.z)],
                [hi[0].max(p.x), hi[1].max(p.y), hi[2].max(p.z)],
            )
        },
    );
    let diag = ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2) + (hi[2] - lo[2]).powi(2)).sqrt();
    assert!(diag.is_finite() && diag > 0.0, "{name}: degenerate bbox");
    let delta = diag * cell.delta_frac;
    let mesh = pncad::mesh::tessellate(&body, delta)
        .unwrap_or_else(|e| panic!("{name}: tessellation at delta {delta:e} failed: {e:?}"));
    check_mesh(&mesh).unwrap_or_else(|e| panic!("{name}: check_mesh failed: {e:?}"));
    let v_mesh = signed_volume(&mesh);
    assert!(v_mesh > 0.0, "{name}: mesh signed volume must be positive");
    let rel = ((v_mesh - props.volume) / props.volume).abs();
    println!(
        "   [{name}] exact: V = {:.3} mm^3, A = {:.3} mm^2; mesh (delta = {:.2e} m): \
         {} triangles, V_mesh = {:.3} mm^3 ({:.3}% off exact — chordal, inscribed)",
        props.volume * 1e9,
        props.surface_area * 1e6,
        delta,
        triangle_count(&mesh),
        v_mesh * 1e9,
        rel * 100.0
    );

    // The binary header is the one caller-visible identity binary STL
    // carries; this corpus names each body in it.
    let stl_options = pncad::stl::BinaryOptions {
        header: pncad::stl::BinaryHeader::new(name)
            .unwrap_or_else(|e| panic!("{name}: STL header refused: {e}")),
    };
    let mut stl_buf = Vec::new();
    pncad::stl::write_binary(&mesh, &stl_options, &mut stl_buf)
        .unwrap_or_else(|e| panic!("{name}: STL write failed: {e:?}"));
    std::fs::write(format!("{outdir}/{name}.stl"), &stl_buf).expect("write stl");
    println!("   [{name}] exported {name}.stl");

    // The manifest entry, in the same field set the tour writes --
    // `step` and `transparency` included, and written independently.
    // The agreement is DELIBERATE AND UNENFORCED: no shared type, no
    // crate depending on the tour, and nothing compares the two
    // emitters. Two fields do not pay for that. What holds it
    // together is that one reader (`demos/manifest.py`) walks both
    // manifests and READS both keys rather than defaulting them, so a
    // drift on either side fails the first render loudly instead of
    // drawing something plausible.
    //
    // Both values are constant here, and neither is a placeholder. A
    // wild cell has no STEP of its own: the fixture it imports is an
    // INPUT, never written into `outdir`, so there is no file for a
    // renderer to open -- `null` is the fact. And nothing in this
    // corpus is about an interior, so every cell is opaque.
    format!(
        "  {{\"name\": \"{name}\", \"caption\": \"{}\", \"montage\": true, \"view\": \
         {{\"elev\": {}, \"azim\": {}, \"up\": \"{}\"}}, \"bodies\": \
         [{{\"stl\": \"{name}.stl\", \"step\": null, \"color\": [{}, {}, {}], \
         \"transparency\": 0}}]}}",
        cell.caption,
        cell.view.0,
        cell.view.1,
        cell.view.2,
        cell.color[0],
        cell.color[1],
        cell.color[2]
    )
}

fn main() {
    let outdir = std::env::args()
        .nth(1)
        .expect("usage: wild-montage <outdir>");
    std::fs::create_dir_all(&outdir).expect("create outdir");

    println!(
        "wild-corpus montage — third-party STEP through the kernel's own import + tessellation"
    );
    println!(
        "====================================================================================="
    );
    println!(
        "cell law: docs/WILD-CORPUS-LICENSES.md (render-OK only; stepcode/ EXCLUDED, untouched)"
    );

    // Cell NAMES are the sheet's identity: each is an STL stem, a
    // scene name and a PNG stem at once, so two cells sharing one
    // would write one file twice and put the same body on the sheet
    // under two captions — silently, and after the first STL had
    // already been clobbered. Checked here, before any cell runs.
    //
    // There is no
    // separate "every scene got a file" check, because with distinct
    // names there is nothing left for one to catch: `run_cell` writes
    // `{name}.stl` from the same field the manifest then names, and
    // panics on a failed write. A second assertion there would be the
    // equal-by-construction shape again, one line down.
    for (i, cell) in WILD_CELLS.iter().enumerate() {
        if let Some(other) = WILD_CELLS[..i].iter().find(|c| c.name == cell.name) {
            panic!(
                "two pinned cells share the scene name {:?} ({} and {}): the name is \
                 the STL/PNG stem, so one would overwrite the other's export",
                cell.name, other.fixture, cell.fixture
            );
        }
    }

    let scenes: Vec<String> = WILD_CELLS
        .iter()
        .map(|cell| {
            println!("\n== {} ==", cell.fixture);
            run_cell(cell, &outdir)
        })
        .collect();

    // The pinned refusal row: still refusing, still the same class.
    println!();
    for (fixture, class) in WILD_REFUSALS {
        match import_step(&fixture_text(fixture), &ImportOptions::default()) {
            Err(e) => {
                let msg = e.to_string();
                assert!(
                    msg.contains(class),
                    "{fixture}: refusal drifted off its pinned class {class:?}: {msg}"
                );
                println!("== {fixture} == refuses typed, as pinned ({class}): {msg}");
            }
            Ok(_) => panic!(
                "{fixture}: PINNED as a refusal but now imports — the montage grew a \
                 cell. Add it to WILD_CELLS and extend the attribution block in \
                 demos/README.md per docs/WILD-CORPUS-LICENSES.md, in the same change."
            ),
        }
    }

    let json = format!("[\n{}\n]\n", scenes.join(",\n"));
    std::fs::write(format!("{outdir}/scenes.json"), json).expect("write scenes.json");
    println!(
        "\nwild montage manifest complete: {} cells -> {outdir}/scenes.json — render \
         with demos/render-wild.sh",
        WILD_CELLS.len()
    );
    // The ε this corpus was adopted and tessellated at, and where it
    // came from (S22, 2026-08-19) — the non-committing door, reported
    // at the end. Worth more here than in the tour: the wild lane's
    // refusal pins are ε-sensitive, so "which ε" is the first question
    // a surprising row raises.
    match pncad::tolerance::committed_report() {
        Some(report) => println!("{report}"),
        None => println!("tolerance: never committed (no predicate ran)"),
    }
}
