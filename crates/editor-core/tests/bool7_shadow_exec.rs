//! BOOL-7 measurement probe (issue 134).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Axis3, BooleanOp, CancelToken, CapEnd, DocEdit, EntityKind, Entry, EvalOptions, Evaluation,
    Node, ProfileDoc, Qualifier, RecipeNodeId, Resolution, RoleSeg, RunCtx, SlotId, StableName,
    diff_verdicts, evaluate, resolve_with_prior,
};
use fixture::{ang, insert, len, on_frame, scl, step};
use geom_core::Tol;

fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

#[test]
fn m1_probe() {
    let doc = ProfileDoc::empty_derived("bool7", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, b0) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, tr) = insert(
        doc,
        Node::Transform {
            input: b0,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: tr,
            declare: None,
        },
    );
    let ev1 = run(&doc, None);
    let t = &ev1.value(sub).expect("sub evaluates").name_table;
    let end = StableName {
        kind: EntityKind::Face,
        node: a,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let frags: Vec<StableName> = t
        .iter()
        .filter_map(|(n, e)| {
            let hit = n.kind == EntityKind::Face
                && matches!(n.path.first(), Some(RoleSeg::FromA(inner)) if **inner == end)
                && matches!(n.path.get(1), Some(RoleSeg::Fragment(Qualifier::SideOf(_))));
            (hit && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .collect();
    eprintln!("FRAGS ({}):", frags.len());
    for f in &frags {
        eprintln!("  {f:?}");
    }
    // Move B far out in x: operands disjoint, the C10 sweep prunes.
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetParam {
            node: tr,
            slot: SlotId::Translation(Axis3::X),
            expr: len(5.0),
        },
    );
    let ev2 = run(&doc2, Some(&ev1));
    let new = RunCtx {
        doc: &doc2,
        eval: &ev2,
    };
    let prior = RunCtx {
        doc: &doc,
        eval: &ev1,
    };
    // populations at sub in both runs
    for (label, ev) in [("prior", &ev1), ("new", &ev2)] {
        let mut m: std::collections::BTreeMap<&str, usize> = Default::default();
        if let Some(v) = ev.value(sub) {
            for w in v.verdicts.iter() {
                *m.entry(w.predicate).or_default() += 1;
            }
        }
        eprintln!("{label} verdicts at sub: {m:?}");
    }
    let fs = diff_verdicts(&ev1, &ev2);
    eprintln!("FLIPSET nodes: {:?}", fs.nodes.keys().collect::<Vec<_>>());
    for f in &frags {
        eprintln!("--- name {f:?}");
        eprintln!("  flips_on_path: {:?}", fs.flips_on_path(f));
        let res = resolve_with_prior(new, prior, f);
        match &res {
            Resolution::Failed(fail) => eprintln!("  FAILED: {:?}", fail.error),
            other => eprintln!("  {other:?}"),
        }
    }
    // Does the base (unfragmented) name live in the new run?
    let base = StableName {
        kind: EntityKind::Face,
        node: sub,
        path: vec![RoleSeg::FromA(end.clone().into())],
    };
    eprintln!(
        "BASE in new table: {:?}",
        ev2.value(sub).map(|v| v.name_table.lookup(&base).is_some())
    );
    eprintln!("new sub table rows:");
    if let Some(v) = ev2.value(sub) {
        for (n, e) in v.name_table.iter().take(40) {
            eprintln!("   {n:?} -> {e:?}");
        }
    }
    panic!("probe");
}

#[test]
fn m2_probe_union_like_corpus() {
    // Scenario A of the diagnosis corpus, verbatim in shape.
    let doc = ProfileDoc::empty_derived("bool7", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b0) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, tr) = insert(
        doc,
        Node::Transform {
            input: b0,
            translation: [len(0.5), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let (doc, decl) = fixture::declare_x_offset_flush(doc, a, b0);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b: tr,
            declare: Some(decl),
        },
    );
    let ev1 = run(&doc, None);
    let t = &ev1.value(u).expect("union evaluates").name_table;
    let sideof: Vec<StableName> = t
        .iter()
        .filter_map(|(n, e)| {
            let hit = n
                .path
                .iter()
                .any(|s| matches!(s, RoleSeg::Fragment(Qualifier::SideOf(_))));
            (hit && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .collect();
    eprintln!("SIDEOF names at union ({}):", sideof.len());
    for n in &sideof {
        eprintln!("  {n:?}");
    }
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetParam {
            node: tr,
            slot: SlotId::Translation(Axis3::X),
            expr: len(2.5),
        },
    );
    let ev2 = run(&doc2, Some(&ev1));
    let new = RunCtx {
        doc: &doc2,
        eval: &ev2,
    };
    let prior = RunCtx {
        doc: &doc,
        eval: &ev1,
    };
    for (label, ev) in [("prior", &ev1), ("new", &ev2)] {
        let mut m: std::collections::BTreeMap<&str, usize> = Default::default();
        if let Some(v) = ev.value(u) {
            for w in v.verdicts.iter() {
                *m.entry(w.predicate).or_default() += 1;
            }
        }
        eprintln!("{label} verdicts at u: {m:?}");
    }
    let fs = diff_verdicts(&ev1, &ev2);
    eprintln!("FLIPSET nodes: {:?}", fs.nodes.keys().collect::<Vec<_>>());
    for (id, d) in &fs.nodes {
        eprintln!(
            "  node {id:?} flips={:?} diverged={:?}",
            d.flips, d.diverged
        );
    }
    for n in &sideof {
        eprintln!("--- {n:?}");
        eprintln!("  path={:?}", editor_core::derivation_nodes(n));
        eprintln!("  flips_on_path={:?}", fs.flips_on_path(n));
        match resolve_with_prior(new, prior, n) {
            Resolution::Failed(f) => eprintln!("  FAILED: {:?}", f.error),
            o => eprintln!("  {o:?}"),
        }
    }
    panic!("probe2");
}

fn slot_scenario(op: BooleanOp, axis: Axis3, delta: f64) {
    let doc = ProfileDoc::empty_derived("bool7", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, b0) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, tr) = insert(
        doc,
        Node::Transform {
            input: b0,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let (doc, sub) = insert(
        doc,
        Node::Boolean {
            op,
            a,
            b: tr,
            declare: None,
        },
    );
    let ev1 = run(&doc, None);
    let Some(v1) = ev1.value(sub) else {
        eprintln!(
            "[{op:?} {axis:?} {delta}] prior did not evaluate: {:?}",
            ev1.nodes.get(&sub)
        );
        return;
    };
    let sideof: Vec<StableName> = v1
        .name_table
        .iter()
        .filter_map(|(n, e)| {
            let hit = n
                .path
                .iter()
                .any(|s| matches!(s, RoleSeg::Fragment(Qualifier::SideOf(_))));
            (hit && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .collect();
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetParam {
            node: tr,
            slot: SlotId::Translation(axis),
            expr: len(delta),
        },
    );
    let ev2 = run(&doc2, Some(&ev1));
    let new = RunCtx {
        doc: &doc2,
        eval: &ev2,
    };
    let prior = RunCtx {
        doc: &doc,
        eval: &ev1,
    };
    let fs = diff_verdicts(&ev1, &ev2);
    let pop = |ev: &Evaluation<f64>| {
        ev.value(sub)
            .map(|v| {
                v.verdicts
                    .iter()
                    .filter(|w| w.predicate == "name_frag_side_of")
                    .count()
            })
            .unwrap_or(0)
    };
    eprintln!(
        "[{op:?} {axis:?} {delta}] sideof_names={} side_of_pop prior={} new={}",
        sideof.len(),
        pop(&ev1),
        pop(&ev2)
    );
    for (id, d) in &fs.nodes {
        eprintln!("   node {id:?} flips={:?}", d.flips);
    }
    for n in sideof.iter().take(1) {
        eprintln!("   flips_on_path={:?}", fs.flips_on_path(n));
        match resolve_with_prior(new, prior, n) {
            Resolution::Failed(f) => {
                if let editor_core::ResolveError::Vanished { diagnosis, .. } = &f.error {
                    eprintln!("   DIAGNOSIS: {diagnosis:?}");
                } else {
                    eprintln!("   other error: {:?}", f.error);
                }
            }
            o => eprintln!("   {o:?}"),
        }
    }
}

#[test]
fn m3_probe_sweep() {
    for op in [BooleanOp::Union, BooleanOp::Subtract] {
        for axis in [Axis3::X, Axis3::Y, Axis3::Z] {
            for delta in [2.5, 3.5, 4.0, 6.0, 7.0, 10.0, -2.5, -4.0, -7.0] {
                slot_scenario(op, axis, delta);
            }
        }
    }
    panic!("probe3");
}
