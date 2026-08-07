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
mod bossplate;
mod crosslap;
mod curvedcut;
mod cutaway;
mod diefillet;
mod heatsink;
mod letterforms;
mod lily;
mod probe;
mod projectbox;
mod rocker;
mod scalar;
mod skinned;
mod tube;

use pncad::mesh::validate::{check_mesh, signed_volume, triangle_count};
use pncad::topo::{Body, ContactRecords};

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
    /// a refusal on a body inside the writer's subset is a regression
    /// that fails the tour, never a silently hollowed F6 dogfood).
    ///
    /// **Since M5 PR 13 this is true for every tour body.** The
    /// writer's subset grew to the whole elementary-surface vocabulary
    /// plus conic and NURBS carriers, and since M6-3 to described
    /// NURBS faces (the loft walls) — every shape the tour builds is
    /// inside it. The field stays because the one live refusal (a
    /// multi-shell CURVED solid, which the outward/void classifier
    /// cannot sign) would produce a body the tour must not silently
    /// drop.
    step_expected: bool,
}

impl SceneBody {
    /// A non-boolean body, planar or curved. STEP export is REQUIRED
    /// to succeed: since M5 PR 13 the writer covers every surface and
    /// carrier kind these bodies carry.
    fn plain(name: impl Into<String>, color: [f64; 3], body: Body<f64>) -> Self {
        Self {
            name: name.into(),
            body,
            contacts: None,
            color,
            step_expected: true,
        }
    }

    /// An all-planar non-boolean body (split halves, transformed
    /// planar bodies). Kept as a distinct spelling because the CALLER
    /// is asserting planarity, which is information about the body;
    /// the STEP posture is now the same as [`Self::plain`]'s.
    fn plain_planar(name: impl Into<String>, color: [f64; 3], body: Body<f64>) -> Self {
        Self::plain(name, color, body)
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

    /// A CURVED boolean result (M5 PR 11's boss∪plate): 3′ validation
    /// with the op's declared contacts. Its STEP export is REQUIRED
    /// since M5 PR 13 — this body's cylinder walls and circle seam
    /// arcs are exactly what the curved arms were written for, and it
    /// is the tour's end-to-end proof that they work on a boolean
    /// result and not only on a swept primitive.
    fn seamed_curved(
        name: impl Into<String>,
        color: [f64; 3],
        body: Body<f64>,
        contacts: ContactRecords,
    ) -> Self {
        Self::seamed(name, color, body, contacts)
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

fn run_body(sb: &SceneBody, delta: f64, outdir: &str) -> Option<ManifestBody> {
    let label = &sb.name;

    // Tiers 1 + 2 on every body.
    pncad::topo::validate(&sb.body)
        .unwrap_or_else(|e| panic!("{label}: tier-1 structural validation failed: {e:?}"));
    pncad::topo::validate_closed(&sb.body)
        .unwrap_or_else(|e| panic!("{label}: tier-2 closed-solid validation failed: {e:?}"));

    // Tier 3 / 3′: boolean results validate AS THEY ARE, with the
    // op's declared contacts (3′); everything else through the plain
    // geometric gate (on contact-free bodies the two gates agree).
    match &sb.contacts {
        Some(contacts) => {
            pncad::topo::validate_pseudomanifold(&sb.body, contacts).unwrap_or_else(|e| {
                panic!("{label}: tier-3' (declared-contact) validation failed: {e:?}")
            });
        }
        None => {
            pncad::topo::validate_geometric(&sb.body)
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
    // faces — not the mesh). Since M5 PR 11 curved-CUT faces
    // contribute certified quadrature enclosures: `volume` is then a
    // bracket midpoint with half-width `volume_pad` (0.0 on
    // closed-form bodies).
    let props = pncad::topo::mass_properties(&sb.body).expect("mass properties");

    // Tessellate, self-check the mesh, and compare its signed volume
    // against the exact one as an end-to-end sanity ribbon.
    let mesh = pncad::mesh::tessellate(&sb.body, delta).expect("tessellate");
    check_mesh(&mesh).unwrap_or_else(|e| panic!("{label}: check_mesh failed: {e:?}"));
    let v_mesh = signed_volume(&mesh);
    assert!(v_mesh > 0.0, "{label}: mesh signed volume must be positive");
    let rel = ((v_mesh - props.volume) / props.volume).abs();
    let certified = if props.volume_pad > 0.0 {
        format!(" (certified enclosure ± {:.1e})", props.volume_pad)
    } else {
        String::new()
    };
    println!(
        "   [{label}] exact: V = {:.6} m^3{certified}, A = {:.6} m^2; mesh (delta = {:.0e}): \
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
    pncad::stl::write_binary(&mesh, &mut stl_buf)
        .unwrap_or_else(|e| panic!("{label}: STL write failed: {e:?}"));
    std::fs::write(&stl_path, &stl_buf).expect("write stl");
    let stl = stl_name.clone();

    // The STEP lane (#88): AP214 export beside every STL. Since M5
    // PR 13 the writer's analytic subset is the whole elementary-
    // surface vocabulary (plane/cylinder/cone/sphere/torus) with
    // line/circle/ellipse/NURBS carriers, all as EXACT native AP214
    // entities — so every tour body exports, curved ones included, and
    // a refusal anywhere here is now a regression rather than a
    // narrated frontier.
    let step_name = format!("{label}.step");
    let step = match pncad::step_export::step_string(
        &sb.body,
        &pncad::step_export::StepOptions {
            product_name: label.clone(),
            ..Default::default()
        },
    ) {
        Ok(doc) => {
            std::fs::write(format!("{outdir}/{step_name}"), doc).expect("write step");
            println!("   [{label}] exported {stl} + {step_name}");
            Some(step_name)
        }
        // The subset-frontier refusal stays an acceptable CLASS (a
        // multi-shell curved solid awaits a curved outward/void
        // classifier; NURBS faces export natively since M6-3), but no
        // tour body is in it today — `step_expected` is true
        // everywhere, so reaching this arm fails the tour loud. The
        // arm is kept, not deleted: it is what keeps a future curved
        // frontier from being silently dropped from the manifest.
        Err(
            e @ (pncad::step_export::StepExportError::UnsupportedSurface { .. }
            | pncad::step_export::StepExportError::UnsupportedCurve { .. }
            | pncad::step_export::StepExportError::CurvedShellClassification { .. }),
        ) => {
            assert!(
                !sb.step_expected,
                "{label}: this body is inside the writer's analytic \
                 subset and MUST export STEP, but the writer refused: {e:?}"
            );
            println!(
                "   [{label}] exported {stl}; STEP refused typed ({e:?}) — \
                 a named subset frontier, not a silent drop"
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

    Some(ManifestBody {
        stl,
        step,
        color: sb.color,
    })
}

/// Runs one stop; returns whether it contributed a scene to the render
/// manifest. A fully STAGED stop (every body behind a frontier) has
/// nothing to draw yet, so it narrates and emits no scene entry —
/// `scenes.json` never carries a scene the renderers cannot render.
fn run_stop(stop: &Stop, outdir: &str, manifest: &mut String) -> bool {
    println!("\n== {} ==", stop.name);
    println!("   {}", stop.story);
    println!("   built by: {}", stop.ops);
    if let Some(note) = &stop.note {
        println!("   note: {note}");
    }
    let bodies: Vec<ManifestBody> = stop
        .bodies
        .iter()
        .filter_map(|sb| run_body(sb, stop.delta, outdir))
        .collect();
    if bodies.is_empty() {
        return false;
    }
    manifest.push_str(&scene_json(stop, &bodies));
    true
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
    let outdir = std::env::args()
        .nth(1)
        .expect("usage: demo-tour <outdir> | demo-tour k-probe [out.csv]");
    // The K-telemetry mode (M4 PR 8b): rebuild every scene at the
    // recording scalar and dump the margin CSV — see `probe`.
    if outdir == "k-probe" {
        probe::run(std::env::args().nth(2));
        return;
    }
    std::fs::create_dir_all(&outdir).expect("create outdir");
    let mut manifest = String::new();
    let mut scenes: Vec<String> = Vec::new();
    let mut run = |stop: &Stop| {
        manifest.clear();
        if run_stop(stop, &outdir, &mut manifest) {
            scenes.push(manifest.clone());
        }
    };

    println!("B-rep kernel demo tour — sweeps, booleans, split, and the M4 recipe layer");
    println!("==========================================================================");
    for stop in bodies::stops() {
        run(&stop);
    }

    println!("\n-- the rocker plate (M5 S2/S8: fillets on arc legs, the branch PICKED) --");
    for stop in rocker::stops() {
        run(&stop);
    }

    println!("\n-- the die (M5 PR 12: rolling-ball fillets, and the pips) --");
    for stop in diefillet::stops() {
        run(&stop);
    }

    println!("\n-- the globe lily (Calochortus albus): a plant, at the kernel's frontier --");
    for stop in lily::stops() {
        run(&stop);
    }
    lily::wall_probes::<f64>();

    println!("\n-- the tilted cut (M5 PR 5's exact ellipse; RENDERING since PR 11) --");
    for stop in curvedcut::stops() {
        run(&stop);
    }

    println!("\n-- boss ∪ plate (M5 PR 9's first transverse curved boolean, visible) --");
    for stop in bossplate::stops() {
        run(&stop);
    }

    skinned::narration();
    for stop in skinned::stops() {
        run(&stop);
    }

    println!("\n-- the tube door (M6-3 Leg F: a torus from its INTENT parameters) --");
    for stop in tube::stops() {
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

    bodies::finale_fail_loud::<f64>();

    let json = format!("[\n{}\n]\n", scenes.join(",\n"));
    std::fs::write(format!("{outdir}/scenes.json"), json).expect("write scenes.json");
    println!("\ntour complete: STL/STEP + scenes.json in {outdir}/ — render with demos/render.sh");
}
