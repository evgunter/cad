//! Pre-GUI visual demo tour: builds the highlight bodies through the
//! kernel's public profile/sweep/boolean/split APIs plus the M4 recipe
//! layer, narrates each stop (operations, topology census, genus,
//! validation tiers, exact vs meshed mass properties), and exports
//! binary STL + (where the analytic subset allows) AP214 STEP per
//! body, plus a scene manifest (`scenes.json`) for the render step
//! (`demos/render.sh` — headless FreeCAD importing OUR STEP files,
//! matplotlib as fallback).
//!
//! Usage: `cargo run --release -- <outdir>` (from `demos/tour/`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod az;
mod bodies;
mod bool_bodies;
mod booleans;
mod crosslap;
mod cutaway;
mod heatsink;
mod letterforms;
mod projectbox;

use mesh::validate::{check_mesh, signed_volume, triangle_count};
use topo::{Body, ContactRecords};

/// One body of a tour scene: its own STL/STEP exports, its own
/// validation posture. `contacts` is `Some` exactly when the body is a
/// boolean result — tier 3′ then runs `validate_pseudomanifold` with
/// the op's OWN declared contacts (the M3 PR 6a contract; the old
/// `upgrade_edges_to_intersections` clone hack is retired).
struct SceneBody {
    name: String,
    body: Body<f64>,
    contacts: Option<ContactRecords>,
    /// Base RGB for the render manifest.
    color: [f64; 3],
    /// Whether STEP export MUST succeed for this body (#91 review M2:
    /// every all-planar body is inside the writer's analytic subset,
    /// so a refusal there is a regression that fails the tour, never
    /// a silently hollowed F6 dogfood). Boolean results are always
    /// planar today (`gate_planar`); curved sweeps are the honest
    /// refusals until the M5 arms.
    step_expected: bool,
}

impl SceneBody {
    /// A (possibly curved) non-boolean body: STEP is attempted and a
    /// typed analytic-subset refusal is narration, not failure.
    fn plain(name: impl Into<String>, color: [f64; 3], body: Body<f64>) -> Self {
        Self {
            name: name.into(),
            body,
            contacts: None,
            color,
            step_expected: false,
        }
    }

    /// An all-planar non-boolean body (split halves, transformed
    /// planar bodies): STEP export is REQUIRED to succeed.
    fn plain_planar(name: impl Into<String>, color: [f64; 3], body: Body<f64>) -> Self {
        Self {
            step_expected: true,
            ..Self::plain(name, color, body)
        }
    }

    fn seamed(
        name: impl Into<String>,
        color: [f64; 3],
        body: Body<f64>,
        contacts: ContactRecords,
    ) -> Self {
        Self {
            name: name.into(),
            body,
            contacts: Some(contacts),
            color,
            step_expected: true,
        }
    }
}

/// Scene presentation: the classic matplotlib view spec (elevation and
/// azimuth in degrees, plus which axis is display-up); the renderers
/// derive their cameras from it.
struct View {
    elev: f64,
    azim: f64,
    up: char,
}

/// One tour stop = one rendered scene (possibly several bodies).
struct Stop {
    name: &'static str,
    /// Montage caption (defaults to `name` when empty).
    caption: String,
    /// Whether the scene is a montage panel (aux proof renders — the
    /// silhouette shadow views — render standalone only).
    montage: bool,
    story: &'static str,
    ops: &'static str,
    delta: f64,
    note: Option<String>,
    view: View,
    bodies: Vec<SceneBody>,
}

/// Topology census + genus via the Euler–Poincaré identity
/// `v − e + f − r = 2(s − g)` (s = shells; g summed over shells; the
/// identity is narration, tier 2 is the checker).
fn census(body: &Body<f64>) -> (usize, usize, usize, usize, usize, i64) {
    let v = body.vertices().count();
    let e = body.edges().count();
    let f = body.faces().count();
    let r: usize = body.faces().map(|(_, face)| face.rings.len()).sum();
    let s = body.shells().count();
    let genus = s as i64 - (v as i64 - e as i64 + f as i64 - r as i64) / 2;
    (v, e, f, r, s, genus)
}

/// A body entry for the scene manifest: file stems + render color.
/// Every body exports STL; `step` is `None` where the writer's
/// analytic subset legitimately refuses (curved surfaces until M5).
struct ManifestBody {
    stl: String,
    step: Option<String>,
    color: [f64; 3],
}

fn run_body(sb: &SceneBody, delta: f64, outdir: &str) -> ManifestBody {
    let label = &sb.name;

    // Tiers 1 + 2 on every body.
    topo::validate(&sb.body)
        .unwrap_or_else(|e| panic!("{label}: tier-1 structural validation failed: {e:?}"));
    topo::validate_closed(&sb.body)
        .unwrap_or_else(|e| panic!("{label}: tier-2 closed-solid validation failed: {e:?}"));

    // Tier 3 / 3′: boolean results validate AS THEY ARE, with the
    // op's declared contacts (3′); everything else through the plain
    // geometric gate (on contact-free bodies the two gates agree).
    match &sb.contacts {
        Some(contacts) => {
            topo::validate_pseudomanifold(&sb.body, contacts).unwrap_or_else(|e| {
                panic!("{label}: tier-3' (declared-contact) validation failed: {e:?}")
            });
        }
        None => {
            topo::validate_geometric(&sb.body)
                .unwrap_or_else(|e| panic!("{label}: tier-3 geometric validation failed: {e:?}"));
        }
    }

    let (v, e, f, r, s, genus) = census(&sb.body);
    println!(
        "   [{label}] topology: {v} vertices, {e} edges, {f} faces, {r} rings, \
         {s} shell(s) -> genus {genus}; validation: {}",
        if sb.contacts.is_some() {
            "tiers 1-2 + 3' on the RESULT body with its declared contacts"
        } else {
            "tiers 1-3 (structural, closed-solid census, geometric/+V)"
        }
    );

    // Exact B-rep mass properties (divergence theorem over the exact
    // faces — not the mesh).
    let props = topo::mass_properties(&sb.body).expect("mass properties");

    // Tessellate, self-check the mesh, and compare its signed volume
    // against the exact one as an end-to-end sanity ribbon.
    let mesh = mesh::tessellate(&sb.body, delta).expect("tessellate");
    check_mesh(&mesh).unwrap_or_else(|e| panic!("{label}: check_mesh failed: {e:?}"));
    let v_mesh = signed_volume(&mesh);
    assert!(v_mesh > 0.0, "{label}: mesh signed volume must be positive");
    let rel = ((v_mesh - props.volume) / props.volume).abs();
    println!(
        "   [{label}] exact: V = {:.6} m^3, A = {:.6} m^2; mesh (delta = {:.0e}): \
         {} triangles, V_mesh = {v_mesh:.6} ({:.3}% off exact — chordal, inscribed)",
        props.volume,
        props.surface_area,
        delta,
        triangle_count(&mesh),
        rel * 100.0
    );

    // STL export — fail-loud on any refusal.
    let stl_name = format!("{label}.stl");
    let stl_path = format!("{outdir}/{stl_name}");
    let mut stl_buf = Vec::new();
    stl::write_binary(&mesh, &mut stl_buf)
        .unwrap_or_else(|e| panic!("{label}: STL write failed: {e:?}"));
    std::fs::write(&stl_path, &stl_buf).expect("write stl");
    let stl = stl_name.clone();

    // The STEP lane (#88): AP214 export beside every STL. The writer's
    // analytic subset is planes/lines today (M5 adds the curved arms),
    // so curved bodies refuse TYPED — narrated, never patched around.
    let step_name = format!("{label}.step");
    let step = match step_export::step_string(
        &sb.body,
        &step_export::StepOptions {
            product_name: label.clone(),
            ..Default::default()
        },
    ) {
        Ok(doc) => {
            std::fs::write(format!("{outdir}/{step_name}"), doc).expect("write step");
            println!("   [{label}] exported {stl} + {step_name}");
            Some(step_name)
        }
        // Only the analytic-subset class is an acceptable refusal, and
        // only on bodies not known planar (#91 review M2); anything
        // else - or a refusal on a planar body - fails the tour loud.
        Err(
            e @ (step_export::StepExportError::UnsupportedSurface { .. }
            | step_export::StepExportError::UnsupportedCurve { .. }),
        ) => {
            assert!(
                !sb.step_expected,
                "{label}: this body is all-planar and MUST export STEP, \
                 but the writer refused: {e:?}"
            );
            println!(
                "   [{label}] exported {stl}; STEP refused typed ({e:?}) — \
                 the writer's analytic subset is planar until M5"
            );
            None
        }
        Err(other) => panic!(
            "{label}: STEP export failed OUTSIDE the analytic-subset \
             refusal class: {other:?}"
        ),
    };
    assert!(
        !(sb.step_expected && step.is_none()),
        "{label}: STEP expected but not produced"
    );

    ManifestBody {
        stl,
        step,
        color: sb.color,
    }
}

fn run_stop(stop: &Stop, outdir: &str, manifest: &mut String) {
    println!("\n== {} ==", stop.name);
    println!("   {}", stop.story);
    println!("   built by: {}", stop.ops);
    if let Some(note) = &stop.note {
        println!("   note: {note}");
    }
    let bodies: Vec<ManifestBody> = stop
        .bodies
        .iter()
        .map(|sb| run_body(sb, stop.delta, outdir))
        .collect();
    manifest.push_str(&scene_json(stop, &bodies));
}

/// One scene's manifest entry (hand-rolled JSON — fixed schema, no
/// string content beyond file stems and captions we control).
fn scene_json(stop: &Stop, bodies: &[ManifestBody]) -> String {
    let caption = if stop.caption.is_empty() {
        stop.name.to_string()
    } else {
        stop.caption.clone()
    };
    let body_entries: Vec<String> = bodies
        .iter()
        .map(|b| {
            let opt = |o: &Option<String>| match o {
                Some(s) => format!("\"{s}\""),
                None => "null".to_string(),
            };
            format!(
                "{{\"stl\": \"{}\", \"step\": {}, \"color\": [{}, {}, {}]}}",
                b.stl,
                opt(&b.step),
                b.color[0],
                b.color[1],
                b.color[2]
            )
        })
        .collect();
    format!(
        "  {{\"name\": \"{}\", \"caption\": \"{}\", \"montage\": {}, \"view\": \
         {{\"elev\": {}, \"azim\": {}, \"up\": \"{}\"}}, \"bodies\": [{}]}}",
        stop.name,
        caption.replace('"', "'"),
        stop.montage,
        stop.view.elev,
        stop.view.azim,
        stop.view.up,
        body_entries.join(", ")
    )
}

fn main() {
    let outdir = std::env::args().nth(1).expect("usage: demo-tour <outdir>");
    std::fs::create_dir_all(&outdir).expect("create outdir");
    let mut manifest = String::new();
    let mut scenes: Vec<String> = Vec::new();
    let mut run = |stop: &Stop| {
        manifest.clear();
        run_stop(stop, &outdir, &mut manifest);
        scenes.push(manifest.clone());
    };

    println!("B-rep kernel demo tour — sweeps, booleans, split, and the M4 recipe layer");
    println!("==========================================================================");
    for stop in bodies::stops() {
        run(&stop);
    }

    println!("\n-- the boolean leg (M3): union / subtract / intersect, planar-only --");
    for stop in bool_bodies::stops() {
        run(&stop);
    }
    bool_bodies::voidbox_narration();

    println!("\n-- silhouettes (the first `intersect` in the tour) --");
    for stop in letterforms::stops() {
        run(&stop);
    }

    println!("\n-- A x Z (#93's acceptance case, building since #108) --");
    for stop in az::stops() {
        run(&stop);
    }

    println!("\n-- the cross-lap joint (#90's boolean-of-boolean, made visible) --");
    for stop in crosslap::stops() {
        run(&stop);
    }

    println!("\n-- the project box (the longest boolean-of-boolean chain) --");
    let (box_stop, box_body) = projectbox::stop();
    run(&box_stop);

    println!("\n-- the cutaway (the first `topo::split` in the tour) --");
    for stop in cutaway::stops(&box_body) {
        run(&stop);
    }

    println!("\n-- the heat sink (the M4 recipe layer: edit, recompute, stable names) --");
    for stop in heatsink::stops() {
        run(&stop);
    }

    bodies::finale_fail_loud();

    let json = format!("[\n{}\n]\n", scenes.join(",\n"));
    std::fs::write(format!("{outdir}/scenes.json"), json).expect("write scenes.json");
    println!("\ntour complete: STL/STEP + scenes.json in {outdir}/ — render with demos/render.sh");
}
