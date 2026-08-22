//! Pre-GUI visual demo tour: builds the highlight bodies through the
//! kernel's public profile/sweep/boolean/split APIs plus the M4 recipe
//! layer, narrates each stop (operations, topology census, genus,
//! validation tiers, exact vs meshed mass properties), and exports
//! binary STL + (where the analytic subset allows) AP214 STEP per
//! body, plus a scene manifest (`scenes.json`) for the render step
//! (`demos/render.sh` — headless FreeCAD importing OUR STEP files,
//! matplotlib as fallback) and, per face, its `(u, v)` chart with its
//! trim loops drawn on it (`uv/*.svg` + `uv.json`, the renderer-free
//! third lane — see [`uvdump`] and `demos/render-uv.sh`).
//!
//! Usage: `cargo run --release -- <outdir>` (from `demos/tour/`).
//!
//! # The demos' purpose (Evan, 2026-08-09 — binding for every edit here)
//!
//! These scenes exist to demonstrate REAL, NATURAL library usage —
//! the way a user would actually write the model. Consequences:
//!
//! - It is always acceptable to update a demo in a way that is NOT
//!   byte-identical when the point is better authoring; mechanical
//!   migrations (imports, plumbing) should still prove byte-identity
//!   because there the diff proves nothing changed.
//! - If some aspect of a demo is AWKWARD to write through the public
//!   surface, that awkwardness is a LIBRARY FINDING: record it (gap
//!   comment here + the orchestrator's log) as something to fix in
//!   the library — never quietly work around it, and never contort
//!   the demo to hide it.
//! - Standing goal: every demo authorable through the Python
//!   bindings; what a demo cannot do through the curated document
//!   surface is a named gap, not a private exception.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod az;
mod bodies;
mod bool_bodies;
mod booleans;
mod bossplate;
mod crosslap;
mod curvedcut;
mod cutaway;
mod diechamfer;
mod diefillet;
mod heatsink;
mod klein;
mod letterforms;
mod lily;
mod paths;
#[cfg(feature = "probe")]
mod probe;
mod projectbox;
mod rocker;
mod scalar;
mod skinned;
#[cfg(feature = "budget")]
mod tessbudget;
mod tube;
mod uvdump;
mod walls;

use pncad::geom_core::Tol;
use pncad::mesh::validate::{check_mesh, signed_volume, triangle_count};
use pncad::topo::{Body, ContactRecords};

/// One body of a tour scene: its own STL/STEP exports, its own
/// validation posture. `contacts` is `Some` exactly when the body is a
/// boolean result — tier 3′ then runs `validate_pseudomanifold` with
/// the op's OWN declared contacts (the M3 PR 6a contract).
struct SceneBody {
    name: String,
    body: Body<f64>,
    contacts: Option<ContactRecords>,
    /// Base RGB for the render manifest.
    color: [f64; 3],
    /// Render transparency, 0–100 (0 = opaque, the default). Carried
    /// in the manifest so
    /// it is a property of the SCENE rather than of a renderer: a
    /// shape whose point is what happens INSIDE it (a neck entering a
    /// body wall) cannot be read from an opaque render at any camera.
    transparency: u8,
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
            transparency: 0,
        }
    }

    /// The same body, rendered see-through (`t` = 0–100). Only for
    /// shapes whose subject is interior — the Klein bottle's neck
    /// inside its own body wall is the founding case.
    fn transparent(mut self, t: u8) -> Self {
        self.transparency = t;
        self
    }

    /// A boolean RESULT: validated at tier 3′ against the op's own
    /// declared contacts rather than through the plain geometric gate.
    /// Curved results (M5 PR 11's boss ∪ plate, whose cylinder walls
    /// and circle seam arcs are what the curved arms were written for)
    /// take this door too — the contacts, not the surface kind, are
    /// what it is about.
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
            transparency: 0,
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
///
/// Both stems are unconditional here. Every tour body exports STL and
/// STEP — [`run_body`] fails the tour on any refusal rather than
/// emitting a body without one — so neither is optional on this side.
/// A null `step` is still a legitimate manifest value: the wild-corpus
/// generator writes one for every cell, because its STEP is an input
/// fixture rather than something it exported. `demos/manifest.py` is
/// where that fact is stated for both readers.
struct ManifestBody {
    stl: String,
    step: String,
    color: [f64; 3],
    transparency: u8,
}

fn run_body(
    sb: &SceneBody,
    delta: f64,
    outdir: &str,
    dumps: &mut Vec<uvdump::FaceDump>,
    tol: Tol,
) -> ManifestBody {
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
            pncad::topo::validate_pseudomanifold(&sb.body, contacts, tol).unwrap_or_else(|e| {
                panic!("{label}: tier-3' (declared-contact) validation failed: {e:?}")
            });
        }
        None => {
            pncad::topo::validate_geometric(&sb.body, tol)
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
    let props = pncad::topo::mass_properties(&sb.body, tol).expect("mass properties");

    // Tessellate, self-check the mesh, and compare its signed volume
    // against the exact one as an end-to-end sanity ribbon.
    let mesh = pncad::mesh::tessellate(&sb.body, delta, tol).expect("tessellate");
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

    // STL export — fail-loud on any refusal. The binary format's
    // 80-byte header is the one caller-visible identity it carries, so
    // it gets this body's name, exactly as the STEP export below sets
    // `product_name`.
    let stl_options = pncad::stl::BinaryOptions {
        header: pncad::stl::BinaryHeader::new(label.clone())
            .unwrap_or_else(|e| panic!("{label}: STL header refused: {e}")),
    };
    let stl_name = format!("{label}.stl");
    let stl_path = format!("{outdir}/{stl_name}");
    let mut stl_buf = Vec::new();
    pncad::stl::write_binary(&mesh, &stl_options, &mut stl_buf)
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
        tol,
    ) {
        Ok(doc) => {
            std::fs::write(format!("{outdir}/{step_name}"), doc).expect("write step");
            println!("   [{label}] exported {stl} + {step_name}");
            step_name
        }
        // Every tour body is inside the writer's analytic subset, so
        // any refusal here fails the tour. The named subset frontier
        // (a multi-shell curved solid, which the outward/void
        // classifier cannot sign) is still spelled out as its own arm
        // because it says something different: reaching it means a
        // tour SCENE grew past the writer, not that the writer broke.
        // Either way the body does not go silently into the manifest
        // without its STEP.
        Err(
            e @ (pncad::step_export::StepExportError::UnsupportedSurface { .. }
            | pncad::step_export::StepExportError::UnsupportedCurve { .. }
            | pncad::step_export::StepExportError::CurvedShellClassification { .. }),
        ) => panic!(
            "{label}: STEP refused typed at the writer's named subset \
             frontier ({e:?}). No tour body is in that class; a scene that \
             legitimately enters it needs the tour to say so at the scene, \
             not a body dropped from the manifest here"
        ),
        Err(other) => panic!(
            "{label}: STEP export failed OUTSIDE the analytic-subset \
             refusal class: {other:?}"
        ),
    };

    // The UV lane (`demos/render-uv.sh`): every face's chart domain as
    // its own SVG. Runs beside the exports rather than in a separate
    // pass because the pcurve caches are a property of THIS body — a
    // reader that re-imported the STEP would be looking at re-minted
    // ones, which is a different question.
    let faces = uvdump::emit(label, &sb.body, outdir, tol);
    let refused = faces.iter().filter(|f| f.note.is_some()).count();
    println!(
        "   [{label}] uv: {} face chart(s) dumped to uv/{}",
        faces.len(),
        if refused > 0 {
            format!(" ({refused} could not be walked — drawn as labeled failure cells)")
        } else {
            String::new()
        }
    );
    dumps.extend(faces);

    ManifestBody {
        stl,
        step,
        color: sb.color,
        transparency: sb.transparency,
    }
}

/// Runs one stop and appends its scene entry to the manifest.
///
/// Every stop contributes a scene: [`run_body`] fails the tour rather
/// than dropping a body, so there is no bodiless scene to suppress and
/// no "this stop is entirely behind a frontier" state to report. A
/// stop that genuinely could not be drawn would have to say so where
/// it is built — see the STEP arm in `run_body`.
fn run_stop(
    stop: &Stop,
    outdir: &str,
    manifest: &mut String,
    dumps: &mut Vec<uvdump::FaceDump>,
    tol: Tol,
) {
    println!("\n== {} ==", stop.name);
    println!("   {}", stop.story);
    println!("   built by: {}", stop.ops);
    if let Some(note) = &stop.note {
        println!("   note: {note}");
    }
    let bodies: Vec<ManifestBody> = stop
        .bodies
        .iter()
        .map(|sb| run_body(sb, stop.delta, outdir, dumps, tol))
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
    // The wild-corpus generator writes this same field set, scene
    // keys and body keys alike, INDEPENDENTLY. The agreement is
    // deliberate and unenforced: no shared type, no crate edge, and
    // nothing compares the two emitters — two fields do not pay for
    // that. What holds it together is that one reader
    // (`demos/manifest.py`) walks both manifests and reads every key
    // rather than defaulting any, so a drift on either side fails the
    // first render loudly instead of drawing something plausible.
    let body_entries: Vec<String> = bodies
        .iter()
        .map(|b| {
            format!(
                "{{\"stl\": \"{}\", \"step\": \"{}\", \"color\": [{}, {}, {}], \
                 \"transparency\": {}}}",
                b.stl, b.step, b.color[0], b.color[1], b.color[2], b.transparency
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

/// Walks the tour in order, handing every scene to `visit`.
///
/// This is the **one** enumeration of what the tour contains: the
/// render pass walks it, and so does the `tess-budget` sweep
/// (`tessbudget`). A scene cannot appear in one and be missing from
/// the other, which is the drift a second hand-maintained list would
/// guarantee.
///
/// A visitor rather than a `Vec<Stop>` because the tour is LAZY on
/// purpose. Several scene constructors narrate as they build (the
/// coincidence ladder, the mated-union doors, the stable-name count),
/// and returning a fully built list would print all of that up front,
/// detached from the stops it belongs to. Building each group as it is
/// reached also keeps one group's bodies alive at a time, and lets the
/// project box hand its body to the cutaway exactly as it always has.
fn walk_tour(visit: &mut dyn FnMut(&Stop), tol: Tol) {
    for stop in bodies::stops(tol) {
        visit(&stop);
    }

    println!("\n-- the rocker plate (M5 S2/S8: fillets on arc legs, the branch PICKED) --");
    for stop in rocker::stops(tol) {
        visit(&stop);
    }

    println!("\n-- the die (M5 PR 12: rolling-ball fillets, and the pips) --");
    for stop in diefillet::stops(tol) {
        visit(&stop);
    }

    println!("\n-- the same die, one verb over (VERBS: chamfer_edges at d == r) --");
    for stop in diechamfer::stops(tol) {
        visit(&stop);
    }

    println!(
        "\n-- the fairy lantern (Calochortus pulchellus): a plant, at the kernel's frontier --"
    );
    for stop in lily::stops(tol) {
        visit(&stop);
    }
    lily::wall_probes::<f64>(tol);

    println!("\n-- the Klein bottle: a non-orientable surface, three bodies deep --");
    for stop in klein::stops(tol) {
        visit(&stop);
    }
    klein::wall_probes::<f64>(tol);

    println!("\n-- the tilted cut (M5 PR 5's exact ellipse; RENDERING since PR 11) --");
    for stop in curvedcut::stops(tol) {
        visit(&stop);
    }

    println!("\n-- boss ∪ plate (M5 PR 9's first transverse curved boolean, visible) --");
    for stop in bossplate::stops(tol) {
        visit(&stop);
    }

    skinned::narration(tol);
    for stop in skinned::stops(tol) {
        visit(&stop);
    }

    println!("\n-- the tube door (M6-3 Leg F: a torus from its INTENT parameters) --");
    for stop in tube::stops(tol) {
        visit(&stop);
    }

    println!("\n-- the boolean leg (M3): union / subtract / intersect, planar-only --");
    for stop in bool_bodies::stops(tol) {
        visit(&stop);
    }
    bool_bodies::voidbox_narration(tol);

    println!("\n-- silhouettes (the first `intersect` in the tour) --");
    for stop in letterforms::stops(tol) {
        visit(&stop);
    }

    println!("\n-- A x Z (#93's acceptance case, building since #108) --");
    for stop in az::stops(tol) {
        visit(&stop);
    }

    println!("\n-- the cross-lap joint (#90's boolean-of-boolean, made visible) --");
    for stop in crosslap::stops(tol) {
        visit(&stop);
    }

    println!("\n-- the project box (the longest boolean-of-boolean chain) --");
    let (box_stop, box_body) = projectbox::stop(tol);
    visit(&box_stop);

    println!("\n-- the cutaway (the first `topo::split` in the tour) --");
    for stop in cutaway::stops(&box_body, tol) {
        visit(&stop);
    }

    println!("\n-- the heat sink (the M4 recipe layer: edit, recompute, stable names) --");
    for stop in heatsink::stops(tol) {
        visit(&stop);
    }
}

fn main() {
    // The tour is an entry point: it mints the run's tolerance witness
    // once, here, and hands it to every scene it walks.
    let tol = Tol::witness();
    let outdir = std::env::args().nth(1).expect(
        "usage: demo-tour <outdir> | demo-tour k-probe [out.csv] | \
                 demo-tour tess-budget [out.csv] [--deviation]",
    );
    // The K-telemetry mode (M4 PR 8b): rebuild every scene at the
    // recording scalar and dump the margin CSV — see `probe`.
    //
    // Behind the `probe` feature since the Probe gate: the recording
    // scalar is a `Real` instantiation, so carrying it here made every
    // release render of this tour monomorphize the whole geometry stack a
    // second time for a mode the render lanes never invoke.
    // `scripts/k_probe_sweep.sh` passes `--features probe`; without it,
    // this mode says so instead of silently rendering to a directory
    // literally named "k-probe".
    #[cfg(feature = "probe")]
    if outdir == "k-probe" {
        probe::run(std::env::args().nth(2), tol);
        return;
    }
    #[cfg(not(feature = "probe"))]
    if outdir == "k-probe" {
        eprintln!(
            "demo-tour: `k-probe` needs the `probe` feature \
             (cargo run --features probe -- k-probe [out.csv]); \
             scripts/k_probe_sweep.sh passes it."
        );
        std::process::exit(2);
    }
    // The tessellation-budget sweep (issue #320) — see `tessbudget`.
    // Behind the `budget` feature for the same reason `k-probe` is
    // behind `probe`: the recording half of `mesh::budget` is gated at
    // its module boundary, so without the feature there is no meter to
    // arm — and this mode says that instead of writing an empty CSV.
    #[cfg(feature = "budget")]
    if outdir == "tess-budget" {
        let rest: Vec<String> = std::env::args().skip(2).collect();
        let deviation = rest.iter().any(|a| a == "--deviation");
        tessbudget::run(
            rest.into_iter().find(|a| !a.starts_with("--")),
            deviation,
            tol,
        );
        return;
    }
    #[cfg(not(feature = "budget"))]
    if outdir == "tess-budget" {
        eprintln!(
            "demo-tour: `tess-budget` needs the `budget` feature \
             (cargo run --release --features budget -- tess-budget [out.csv] \
             [--deviation]); scripts/tess_budget_sweep.sh passes it."
        );
        std::process::exit(2);
    }
    std::fs::create_dir_all(&outdir).expect("create outdir");
    let mut manifest = String::new();
    let mut scenes: Vec<String> = Vec::new();
    let mut dumps: Vec<uvdump::FaceDump> = Vec::new();
    let mut cells = 0usize;
    let mut run = |stop: &Stop| {
        manifest.clear();
        run_stop(stop, &outdir, &mut manifest, &mut dumps, tol);
        scenes.push(manifest.clone());
        cells += usize::from(stop.montage);
    };

    println!("B-rep kernel demo tour — sweeps, booleans, split, and the M4 recipe layer");
    println!("==========================================================================");
    walk_tour(&mut run, tol);

    let json = format!("[\n{}\n]\n", scenes.join(",\n"));
    std::fs::write(format!("{outdir}/scenes.json"), json).expect("write scenes.json");
    std::fs::write(format!("{outdir}/uv.json"), uvdump::manifest_json(&dumps))
        .expect("write uv.json");
    let curved = dumps.iter().filter(|d| d.curved).count();
    let refused = dumps.iter().filter(|d| d.note.is_some()).count();
    // The scene and cell counts, MEASURED. `demos/README.md` explains
    // WHICH scenes stay off the montage and why; the arithmetic is
    // here, where the scenes are, so the README never has to restate a
    // number that a new stop changes.
    println!(
        "\ntour complete: {} scenes ({cells} montage cells, {} standalone) — \
         STL/STEP + scenes.json in {outdir}/, render with demos/render.sh",
        scenes.len(),
        scenes.len() - cells
    );
    println!(
        "uv lane: {} face charts ({curved} curved, {refused} unwalkable) in {outdir}/uv/ \
         + uv.json — sheet with demos/render-uv.sh",
        dumps.len()
    );
    // The uv lane's own claims, MEASURED on this run rather than
    // pinned in prose beside the code that computes them. Every number
    // the module documents about the corpus is here: how many charts
    // the winding check can speak about, how many agree with
    // `Face::sense`, and the two worst junction gaps. A count written
    // down beside its own computation is the one that drifts; this
    // line is the record.
    //
    // A face whose loops could not be WALKED carries
    // `FaceStats::default()` — all zeros, `winding_ok = false` — which
    // is the absence of a measurement, not a failed one. It is
    // excluded from every count here and reported as `unwalkable`
    // above, once.
    let walked: Vec<&uvdump::FaceDump> = dumps.iter().filter(|d| d.note.is_none()).collect();
    let jumped = walked.iter().filter(|d| d.stats.chart_jump > 1e-9).count();
    let disagree: Vec<&&uvdump::FaceDump> = walked.iter().filter(|d| !d.stats.winding_ok).collect();
    let worst_gap = walked.iter().map(|d| d.stats.gap).fold(0.0f64, f64::max);
    let worst_jump = walked
        .iter()
        .map(|d| d.stats.chart_jump)
        .fold(0.0f64, f64::max);
    println!(
        "   winding vs Face::sense: {} chart(s) checkable, {} carry a branch jump \
         (shoelace meaningless there); {} disagree",
        walked.len() - jumped,
        jumped,
        disagree.len()
    );
    println!(
        "   closure: worst 3-D loop gap {worst_gap:.2e} m; worst chart jump \
         {worst_jump:.6} (seam/pole structure, not a defect)"
    );
    // AND IT IS FATAL, not a printed number. Every face here is the
    // kernel's own output, so a chart winding that contradicts the
    // face's `sense` bit is a kernel regression: the even-odd interior
    // of that face is the complement of the intended one, and the
    // tessellator's trim walk composes the same rings. The tour
    // already fails on every other broken kernel invariant it meets
    // (the three validation tiers, `check_mesh`, a non-positive mesh
    // volume, any STEP refusal), and `uvdump`'s "a diagnostic must not
    // refuse broken input" governs which FACES get drawn — it is about
    // not hiding a bad chart from the reader, not about the tour
    // shrugging at one. If this ever fires, the fix is a kernel issue
    // and the witness is in the message.
    assert!(
        disagree.is_empty(),
        "WINDING CONTRADICTION on {} chart(s): {} — the measured chart winding \
         disagrees with the face's own `sense` bit, so the even-odd interior is \
         the complement of the intended one. These charts are kernel output; \
         this is a kernel regression, not a demo one.",
        disagree.len(),
        disagree
            .iter()
            .map(|d| format!("{} face {} ({})", d.body, d.face, d.chart))
            .collect::<Vec<_>>()
            .join(", ")
    );
    // The ε this whole tour was decided at, and WHERE IT CAME FROM
    // (S22, 2026-08-19). ε is a declared run parameter, so a run says
    // which one it used — a stale `CAD_TOLERANCE_EPS` in a shell
    // changes what "coincident" means, and without this line nothing
    // in the output would mention it (issues #415, #497).
    //
    // Reported at the END through the NON-committing door: asking is
    // not deciding, so this cannot pre-empt a document that states its
    // own ε. By now the first predicate has long since committed one.
    match pncad::tolerance::committed_report() {
        Some(report) => println!("{report}"),
        None => println!("tolerance: never committed (no predicate ran)"),
    }
}
