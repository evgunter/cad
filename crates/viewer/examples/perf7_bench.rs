//! PERF-7 fix-pass measurement scratch — NOT for commit.
//! `cargo run --release -p viewer --example perf7_bench`
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]
#![allow(dead_code, unreachable_pub)]

#[path = "../../editor-core/tests/corpus/mod.rs"]
mod corpus;
#[path = "../../editor-core/tests/fixture/mod.rs"]
mod fixture;

use std::sync::Arc;
use std::time::Instant;

use editor_core::{DocEdit, ProfileDoc, ProfileProgram, RecipeNodeId, SlotId, unparse};
use geom_core::Tol;
use topo::Body;

use viewer::evalseam::{IndexDone, IndexRequest, IndexService, InlineIndexer};
use viewer::scene::DisplayTolerance;
use viewer::session::{DocSession, SessionOp};

fn stats(mut t: Vec<f64>) -> String {
    t.sort_by(f64::total_cmp);
    let n = t.len();
    format!(
        "median {:.4} ms (min {:.4}, max {:.4}, n={n})",
        t[n / 2],
        t[0],
        t[n - 1]
    )
}

fn timed<R>(reps: usize, mut f: impl FnMut() -> R) -> Vec<f64> {
    (0..reps)
        .map(|_| {
            let s = Instant::now();
            let _ = f();
            s.elapsed().as_secs_f64() * 1e3
        })
        .collect()
}

fn ngon_prism(n: usize) -> Body<f64> {
    use pncad::authoring::{p2, validated};
    use pncad::profile::{ProfileLoop, SketchPlane};
    use profile::RawLoop;
    use pncad::sweep::{Extrusion, extrude};
    let pts: Vec<_> = (0..n)
        .map(|i| {
            let a = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
            p2(a.cos(), a.sin())
        })
        .collect();
    let profile = validated(
        SketchPlane::<f64>::xy(),
        vec![ProfileLoop::polygon(pts)],
        Tol::witness(),
    )
    .expect("the prism profile validates");
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .expect("the prism extrudes")
        .body
}

fn prisms() {
    let tol = Tol::witness();
    for n in [4usize, 16, 64, 256] {
        let body = ngon_prism(n);
        let faces = body.faces().count();
        let tris = pncad::mesh::tessellate(&body, 0.5, tol)
            .unwrap()
            .patches
            .iter()
            .map(|p| p.triangles.len())
            .sum::<usize>();
        let t = timed(41, || pncad::mesh::tessellate(&body, 0.5, tol).unwrap());
        println!("| prism n={n} | faces {faces} | tris {tris} | {} |", stats(t));
    }
}

fn request_at(session: &DocSession, at: DisplayTolerance) -> IndexRequest {
    let (doc, _) = session.landed_pair().expect("a landed pair");
    IndexRequest {
        generation: session.landed_generation().expect("a generation"),
        delta: at,
        doc: doc.clone(),
        evaluation: Arc::clone(session.evaluation_arc().expect("a landed run")),
        tol: session.tol(),
    }
}

fn answer(seam: &mut impl IndexService, request: IndexRequest) -> IndexDone {
    seam.submit(request);
    for _ in 0..100_000 {
        if let Some(done) = seam.poll() {
            return done;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    panic!("the seam never answered")
}

struct Edit {
    node: RecipeNodeId,
    slot: SlotId,
    text: String,
}

impl Edit {
    fn op(&self) -> SessionOp {
        SessionOp::SetSlotExpression {
            node: self.node,
            slot: self.slot,
            text: self.text.clone(),
        }
    }
}

fn memo_primed(name: &str, doc: ProfileDoc, bump: DocEdit<ProfileProgram>, reps: usize) {
    let tol = Tol::witness();
    let DocEdit::SetParam { node, slot, expr } = bump else {
        panic!("{name}: the bump is not SetParam")
    };
    let original = doc.node(node).unwrap().expr(slot).unwrap().clone();
    let bumped = Edit {
        node,
        slot,
        text: unparse(&expr),
    };
    let revert = Edit {
        node,
        slot,
        text: unparse(&original),
    };
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let at = DisplayTolerance::new(1e-4).unwrap();

    let first = timed(3, || {
        let (doc, ev) = session.landed_pair().expect("a landed pair");
        viewer::pickindex::PickIndex::build(
            doc,
            ev,
            session.landed_generation().unwrap(),
            at,
            tol,
        )
        .expect("index")
    });
    println!("| {name} PickIndex::build (first open) | {} |", stats(first));

    let mut seam = InlineIndexer::new();
    let done = answer(&mut seam, request_at(&session, at));
    assert!(done.index.is_ok());
    let mut t = Vec::new();
    let mut hits = Vec::new();
    for k in 0..reps {
        let edit = if k % 2 == 0 { &bumped } else { &revert };
        let out = session.perform(edit.op());
        assert!(out.refusal.is_none(), "{name}: edit refused");
        session.pump();
        let s = Instant::now();
        let done = answer(&mut seam, request_at(&session, at));
        t.push(s.elapsed().as_secs_f64() * 1e3);
        assert!(done.index.is_ok());
        let m = seam.memo();
        hits.push((m.patches().hits(), m.patches().misses()));
    }
    println!(
        "| {name} memo-primed re-index | face (hits,misses) {:?} | {} |",
        &hits[..hits.len().min(3)],
        stats(t)
    );

    // The HIT PATH ALONE, which is what the fix changes: a memo primed
    // on this body, then the same tessellation again — every face a
    // hit, nothing else in the timing.
    let body = session.landed_body().cloned().expect("a landed body");
    let fitted = viewer::scene::fit_delta(&body, at, tol).expect("a fit");
    let delta = fitted.delta.get();
    let mut memo = pncad::mesh::PatchMemo::new();
    let _ = pncad::mesh::tessellate_with(&body, delta, tol, &mut memo).expect("primes");
    memo.end_picture();
    let t = timed(201, || {
        let out = pncad::mesh::tessellate_with(&body, delta, tol, &mut memo).expect("hits");
        memo.end_picture();
        out
    });
    println!(
        "| {name} tessellate_with, memo primed | faces {} | hits {} misses {} | {} |",
        body.faces().count(),
        memo.hits(),
        memo.misses(),
        stats(t)
    );
}

fn main() {
    println!(
        "# RAYON_NUM_THREADS={:?}, available_parallelism = {}",
        std::env::var("RAYON_NUM_THREADS").unwrap_or_default(),
        std::thread::available_parallelism().map_or(0, std::num::NonZero::get)
    );
    prisms();
    for c in corpus::documents() {
        if c.name == "die" {
            memo_primed("die", c.doc.clone(), c.bump.clone(), 15);
        }
    }
}
