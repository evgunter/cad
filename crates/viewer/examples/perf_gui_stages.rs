//! PERF-GUI exploration harness (lane `perf-gui`, NOT for main).
//!
//! Times every stage between "an edit is committed in the viewer" and
//! "the viewport has a new scene mesh", through the same doors the
//! application uses, on the `editor-core` corpus documents.
//!
//! Run: `cargo run --release -p viewer --example perf_gui_stages -- [reps] [name...]`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]
#![allow(dead_code, unreachable_pub)]

#[path = "../../editor-core/tests/corpus/mod.rs"]
mod corpus;
#[path = "../../editor-core/tests/fixture/mod.rs"]
mod fixture;

use std::collections::BTreeSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use editor_core::{CancelToken, DocEdit, EvalOptions, Evaluation, ProfileDoc, evaluate, unparse};
use geom_core::Tol;

use viewer::display::DisplayView;
use viewer::evalseam::EvalDone;
use viewer::generation::Generation;
use viewer::pickindex::PickIndex;
use viewer::scene::{self, DisplayTolerance};
use viewer::session::{DocSession, SessionOp};

/// The δ the application starts on (`app::INITIAL_DELTA`).
const INITIAL_DELTA: f64 = 1.0e-4;

fn median(mut v: Vec<f64>) -> (f64, f64) {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = v[v.len() / 2];
    let spread = if m > 0.0 {
        (v[v.len() - 1] - v[0]) / 2.0 / m * 100.0
    } else {
        0.0
    };
    (m, spread)
}

fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

/// Time `f` `reps` times, returning (median ms, ± half-range %).
fn timed<T>(reps: usize, mut f: impl FnMut() -> T) -> (f64, f64) {
    let mut out = Vec::with_capacity(reps);
    for _ in 0..reps {
        let t = Instant::now();
        let value = f();
        out.push(ms(t.elapsed()));
        drop(value);
    }
    median(out)
}

fn evaluate_run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>, tol: Tol) -> Evaluation<f64> {
    evaluate::<f64>(doc, prior, &CancelToken::new(), &EvalOptions::default(), tol)
}

struct Row {
    name: &'static str,
    nodes: usize,
    delta: f64,
    triangles: usize,
    stages: Vec<(&'static str, f64, f64)>,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let reps: usize = args.first().and_then(|a| a.parse().ok()).unwrap_or(3);
    let wanted: Vec<String> = args.iter().skip(1).cloned().collect();
    let tol = Tol::witness();

    println!("# perf-gui stage timings — release profile");
    println!("# reps = {reps}, tol = witness, budget δ chosen by scene::fit_delta");

    let mut docs = corpus::documents();
    docs.push(gallery_ring(tol));
    docs.push(plate(tol));
    for c in docs {
        if !wanted.is_empty() && !wanted.iter().any(|w| w == c.name) {
            continue;
        }
        if !matches!(c.bump, DocEdit::SetParam { .. }) {
            println!("\n## {} — SKIPPED (bump is not SetParam)", c.name);
            continue;
        }
        let row = measure(&c, reps, tol);
        print_row(&row);
    }
}

/// The tour's gallery ring — the committed viewer fixture, and the
/// realistic CURVED document: the one the display budget was written
/// for. Its bump edits the ring's own tube radius.
fn gallery_ring(tol: Tol) -> corpus::CorpusDoc {
    let text = include_str!("../tests/gallery_ring.pncad");
    let loaded = editor_core::load(text, tol)
        .expect("the committed gallery-ring fixture loads");
    let doc = loaded.snapshot;
    let (node, slot, expr) = first_length_slot(&doc);
    corpus::CorpusDoc {
        name: "gallery_ring",
        about: "the tour's gallery ring (viewer fixture)",
        edits: Vec::new(),
        doc,
        result: None,
        pin: None,
        bump: DocEdit::SetParam { node, slot, expr },
        bump_root: node,
    }
}

/// The document the application STARTS on (`app::ViewerApp::new` →
/// `scene::plate_with_hole`).
fn plate(tol: Tol) -> corpus::CorpusDoc {
    let (doc, _root) = viewer::scene::plate_with_hole(tol).expect("the startup document");
    let (node, slot, expr) = first_length_slot(&doc);
    corpus::CorpusDoc {
        name: "startup_plate",
        about: "the viewer's startup document",
        edits: Vec::new(),
        doc,
        result: None,
        pin: None,
        bump: DocEdit::SetParam { node, slot, expr },
        bump_root: node,
    }
}

/// The last extrude-distance slot in the document, nudged — a stand-in
/// for the Properties-panel edit the corpus documents carry as `bump`.
fn first_length_slot(
    doc: &ProfileDoc,
) -> (
    editor_core::RecipeNodeId,
    editor_core::SlotId,
    editor_core::Expr,
) {
    let env = doc.param_env::<f64>();
    for &node in doc.order().iter().rev() {
        match doc.node(node).expect("a node") {
            editor_core::Node::Extrude { distance, .. } => {
                let value = editor_core::eval(distance, &env).expect("a literal distance");
                let expr =
                    editor_core::Expr::literal(value * 1.03125, editor_core::Dimension::Length)
                        .expect("a length literal");
                return (node, editor_core::SlotId::Distance, expr);
            }
            editor_core::Node::Revolve { angle, .. } => {
                let value = editor_core::eval(angle, &env).expect("a literal angle");
                let expr = editor_core::Expr::literal(value * 0.96875, editor_core::Dimension::Angle)
                    .expect("an angle literal");
                return (node, editor_core::SlotId::RevolveAngle, expr);
            }
            _ => {}
        }
    }
    panic!("no extrude or revolve in the document")
}

fn print_row(row: &Row) {
    println!();
    println!(
        "## {} — {} nodes, δ = {:.3e} m, {} triangles",
        row.name, row.nodes, row.delta, row.triangles
    );
    println!("| stage | median ms | ± % |");
    println!("|---|---:|---:|");
    for (name, m, s) in &row.stages {
        println!("| {name} | {m:.2} | {s:.1} |");
    }
}

fn measure(c: &corpus::CorpusDoc, reps: usize, tol: Tol) -> Row {
    let doc = c.doc.clone();
    let bumped = c.bumped();
    let mut stages: Vec<(&'static str, f64, f64)> = Vec::new();

    // ---- evaluation ------------------------------------------------
    let (m, s) = timed(reps, || evaluate_run(&doc, None, tol));
    stages.push(("eval: full, no prior (open)", m, s));

    let base = evaluate_run(&doc, None, tol);
    let (m, s) = timed(reps, || evaluate_run(&bumped, Some(&base), tol));
    stages.push(("eval: edit WITH prior (memo)", m, s));
    let (m, s) = timed(reps, || evaluate_run(&bumped, None, tol));
    stages.push(("eval: edit WITHOUT prior", m, s));

    let after = Arc::new(evaluate_run(&bumped, Some(&base), tol));

    // ---- the edit door itself --------------------------------------
    let DocEdit::SetParam { node, slot, expr } = c.bump.clone() else {
        panic!("{}: bump is not SetParam", c.name)
    };
    let text = unparse(&expr);
    let mut door: Vec<f64> = Vec::with_capacity(reps);
    for _ in 0..reps {
        let mut session = DocSession::inline(doc.clone(), tol);
        session.pump();
        let t = Instant::now();
        let outcome = session.perform(SessionOp::SetSlotExpression {
            node,
            slot,
            text: text.clone(),
        });
        door.push(ms(t.elapsed()));
        assert!(outcome.refusal.is_none(), "{}: edit refused", c.name);
    }
    let (m, s) = median(door);
    stages.push((
        "edit door: SetSlotExpression (parse+apply+history+submit)",
        m,
        s,
    ));

    // ---- landing ---------------------------------------------------
    let mut land: Vec<f64> = Vec::with_capacity(reps);
    for _ in 0..reps {
        let mut session = DocSession::inline(doc.clone(), tol);
        session.pump();
        session.perform(SessionOp::SetSlotExpression {
            node,
            slot,
            text: text.clone(),
        });
        let done = EvalDone {
            generation: session.generation(),
            evaluation: Arc::clone(&after),
        };
        let t = Instant::now();
        let landing = session.land(done);
        land.push(ms(t.elapsed()));
        println!("# {}: landing = {landing:?}", c.name);
    }
    let (m, s) = median(land);
    stages.push(("land: gather + checks registry + A5 badge", m, s));

    // The landing's two halves, through the same doors `land` calls
    // (`session.rs:915`, `:929`).
    let (m, s) = timed(reps, || {
        editor_core::product_recorded(&bumped, &after, tol).expect("product")
    });
    stages.push(("  of which: product_recorded (the gather)", m, s));
    let product = editor_core::product_recorded(&bumped, &after, tol).expect("product");
    let cfg = editor_core::ChecksConfig::default();
    let (m, s) = timed(reps, || {
        editor_core::run_checks_on(
            &bumped,
            &after,
            editor_core::Subject::Product(&product),
            &cfg,
            tol,
        )
    });
    stages.push(("  of which: run_checks_on (advisory registry)", m, s));

    // ---- the δ the document opens at -------------------------------
    let requested = DisplayTolerance::new(INITIAL_DELTA).unwrap();
    let mut session = DocSession::inline(bumped.clone(), tol);
    session.pump();
    let body = session
        .landed_body()
        .cloned()
        .expect("a landed body for the corpus document");
    let (m, s) = timed(reps, || scene::fit_delta(&body, requested, tol));
    stages.push(("fit_delta: probe tessellation (open only)", m, s));
    let fitted = scene::fit_delta(&body, requested, tol).expect("the fit");
    let delta = fitted.delta;

    // ---- the index build, at the δ the document opens at ------------
    let genr = Generation::FIRST;
    let (m, s) = timed(reps, || {
        PickIndex::build(&bumped, &after, genr, delta, tol).expect("index")
    });
    stages.push(("INDEX build total (tessellate + BVH + id map)", m, s));

    // Tessellation alone, over the gathered product at the same δ.
    // `scene::fit_delta`'s own docs pin gathered and per-root triangle
    // counts as equal (0.000%), so this is the tessellate half of the
    // index build's total and the remainder is BVH + windows + id map.
    let (m, s) = timed(reps, || {
        pncad::mesh::tessellate(&body, delta.get(), tol).expect("tessellate")
    });
    stages.push(("  of which: mesh::tessellate (whole model)", m, s));

    let index = PickIndex::build(&bumped, &after, genr, delta, tol).expect("index");
    let triangles: usize = index
        .parts()
        .iter()
        .map(|p| {
            p.mesh()
                .patches
                .iter()
                .map(|q| q.triangles.len())
                .sum::<usize>()
        })
        .sum();

    // tessellation alone, over the same bodies the index walked
    let bodies: Vec<_> = index.parts().iter().map(|p| p.mesh().clone()).collect();
    drop(bodies);

    // ---- the scene gather (vertex assembly) -------------------------
    let display = DisplayView::none();
    let focus = BTreeSet::new();
    let (m, s) = timed(reps, || index.scene_focused(&display, &focus).expect("scene"));
    stages.push(("scene: scene_focused (GPU vertex assembly)", m, s));

    // The whole wait a committed edit produces, as the medians add up:
    // the edit door, the memo-primed evaluation, the landing, the index
    // build and the scene assembly. `fit_delta` is NOT in it — the
    // budget binds on arrival only (`app::sync_scene`'s
    // `fit_delta_on_scene`).
    let per_edit: f64 = stages
        .iter()
        .filter(|(n, _, _)| {
            n.starts_with("eval: edit WITH prior")
                || n.starts_with("edit door")
                || n.starts_with("land:")
                || n.starts_with("INDEX")
                || n.starts_with("scene:")
        })
        .map(|(_, m, _)| m)
        .sum();
    stages.push(("== EDIT→PICTURE TOTAL ==", per_edit, 0.0));

    Row {
        name: c.name,
        nodes: c.len(),
        delta: delta.get(),
        triangles,
        stages,
    }
}
