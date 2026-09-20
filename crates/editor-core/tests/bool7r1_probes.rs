//! BOOL-7 review probes (issue 134, the shadow-exec rung): a sample of
//! the PR body's 54-cell sweep, the ladder-order rows re-spelled on the
//! REAL document so they can go red, the prior/current-context attacks
//! the rung's docs promise typed answers or fall-throughs for, and the
//! trigger's granularity (node, not pair).
//!
//! Rows that print rather than assert are measurements; the report
//! carries their output.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::sync::Arc;

use editor_core::{
    Axis3, BooleanOp, CancelToken, Diagnosis, DocEdit, Entry, EvalOptions, Evaluation, FlipSource,
    NameTable, Node, NodeResult, ProfileDoc, Qualifier, RecipeNodeId, Resolution, ResolveError,
    RoleSeg, RunCtx, SHADOW_EXEC_MAX_PAIRS, SlotId, StableName, diff_verdicts, evaluate,
    resolve_with_prior,
};
use fixture::{ang, insert, len, on_frame, scl, step};
use geom_core::k_stats::Verdict;
use geom_core::{Sign, Tol};

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

/// The unit's own scenario (its `slot`), with the bar's extrude id
/// kept so a "landing short" edit can shorten it.
struct Slot {
    doc: ProfileDoc,
    bar: RecipeNodeId,
    tr: RecipeNodeId,
    cut: RecipeNodeId,
}

fn slot() -> Slot {
    let doc = ProfileDoc::empty_derived("bool7r1", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, bar) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, tr) = insert(
        doc,
        Node::Transform {
            input: bar,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: tr,
            declare: None,
        },
    );
    Slot { doc, bar, tr, cut }
}

fn side_of_fragments(ev: &Evaluation<f64>, cut: RecipeNodeId) -> Vec<StableName> {
    let Some(value) = ev.value(cut) else {
        return Vec::new();
    };
    value
        .name_table
        .iter()
        .filter_map(|(n, e)| {
            let discriminated =
                matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::SideOf(_))));
            (discriminated && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .collect()
}

/// Sign histogram of the node's `name_frag_side_of` population.
fn pop(ev: &Evaluation<f64>, node: RecipeNodeId) -> [usize; 3] {
    let mut h = [0usize; 3];
    if let Some(v) = ev.value(node) {
        for w in v
            .verdicts
            .iter()
            .filter(|w| w.predicate == "name_frag_side_of")
        {
            h[match w.sign {
                Sign::Negative => 0,
                Sign::Zero => 1,
                Sign::Positive => 2,
            }] += 1;
        }
    }
    h
}

fn vanished(res: &Resolution) -> Option<&Diagnosis> {
    let Resolution::Failed(f) = res else {
        return None;
    };
    let ResolveError::Vanished { diagnosis, .. } = &f.error else {
        return None;
    };
    Some(diagnosis)
}

fn resolve<'a>(
    new: (&'a ProfileDoc, &'a Evaluation<f64>),
    prior: (&'a ProfileDoc, &'a Evaluation<f64>),
    name: &StableName,
) -> Resolution {
    resolve_with_prior(
        RunCtx {
            doc: new.0,
            eval: new.1,
        },
        RunCtx {
            doc: prior.0,
            eval: prior.1,
        },
        name,
        Tol::witness(),
    )
}

fn describe(res: &Resolution) -> String {
    match res {
        Resolution::Resolved(r) => format!("RESOLVED {r:?}"),
        Resolution::Indeterminate(i) => format!("INDETERMINATE {i:?}"),
        Resolution::Failed(f) => match &f.error {
            ResolveError::Vanished { diagnosis, .. } => match diagnosis {
                Diagnosis::PredicateFlip {
                    predicate,
                    from,
                    to,
                    source,
                } => format!("PredicateFlip {predicate} {from:?}->{to:?} [{source:?}]"),
                other => format!("{other:?}"),
            },
            other => format!("NOT VANISHED: {other}"),
        },
    }
}

/// Replaces a node's verdict log with `f(log)`.
fn splice_log(ev: &mut Evaluation<f64>, node: RecipeNodeId, f: impl FnOnce(&mut Vec<Verdict>)) {
    let Some(NodeResult::Ok(v)) = ev.nodes.get_mut(&node) else {
        panic!("the node has a value");
    };
    let mut log = (*v.verdicts).clone();
    f(&mut log);
    v.verdicts = Arc::new(log);
}

fn v(predicate: &'static str, sign: Sign) -> Verdict {
    Verdict { predicate, sign }
}

fn is_shadow(d: Option<&Diagnosis>) -> bool {
    matches!(
        d,
        Some(Diagnosis::PredicateFlip {
            source: FlipSource::ShadowExec { .. },
            ..
        })
    )
}

// ---------------------------------------------------------------
// 1. A sample of the 54-cell sweep the PR body reports in prose.
// ---------------------------------------------------------------

#[test]
fn sweep_sample_population_and_diagnosis_per_cell() {
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    assert_eq!(frags.len(), 2);
    eprintln!("prior population (N,Z,P) at cut: {:?}", pop(&ev1, s.cut));
    eprintln!("vanishing name: {}", frags[0]);
    let set = |node, slot, expr| DocEdit::SetParam { node, slot, expr };
    let cells: Vec<(&str, DocEdit<editor_core::ProfileProgram>)> = vec![
        (
            "X +5.0 (disjoint, the PR's row)",
            set(s.tr, SlotId::Translation(Axis3::X), len(5.0)),
        ),
        (
            "X -5.0 (disjoint)",
            set(s.tr, SlotId::Translation(Axis3::X), len(-5.0)),
        ),
        (
            "X +2.0 (bar wall flush with plate side x=3)",
            set(s.tr, SlotId::Translation(Axis3::X), len(2.0)),
        ),
        (
            "X +1.5 (bar straddles plate side: one-fragment collapse)",
            set(s.tr, SlotId::Translation(Axis3::X), len(1.5)),
        ),
        (
            "Y +2.5 (PR: collapse, no pruning)",
            set(s.tr, SlotId::Translation(Axis3::Y), len(2.5)),
        ),
        ("Y +3.5", set(s.tr, SlotId::Translation(Axis3::Y), len(3.5))),
        (
            "Y +4.0 (bar edge flush with plate side y=3)",
            set(s.tr, SlotId::Translation(Axis3::Y), len(4.0)),
        ),
        (
            "Y +10.0 (disjoint)",
            set(s.tr, SlotId::Translation(Axis3::Y), len(10.0)),
        ),
        (
            "Z +1.0 (bar lifted clear: disjoint)",
            set(s.tr, SlotId::Translation(Axis3::Z), len(1.0)),
        ),
        (
            "Z -1.0 (bar below the cap: bottom cap fragments instead)",
            set(s.tr, SlotId::Translation(Axis3::Z), len(-1.0)),
        ),
        (
            "Z +0.5 (bar bottom coplanar with the cap)",
            set(s.tr, SlotId::Translation(Axis3::Z), len(0.5)),
        ),
        ("rot z 180", set(s.tr, SlotId::RotationAngle, ang(180.0))),
        (
            "rot z pi",
            set(s.tr, SlotId::RotationAngle, ang(std::f64::consts::PI)),
        ),
        ("rot z 90", set(s.tr, SlotId::RotationAngle, ang(90.0))),
        (
            "rot z pi/2",
            set(
                s.tr,
                SlotId::RotationAngle,
                ang(std::f64::consts::FRAC_PI_2),
            ),
        ),
        (
            "bar distance 0.3 (lands short of the cap)",
            set(s.bar, SlotId::Distance, len(0.3)),
        ),
        (
            "bar distance 0.5 (bar top coplanar with the cap)",
            set(s.bar, SlotId::Distance, len(0.5)),
        ),
        (
            "delete the transform (the bar's carrier)",
            DocEdit::DeleteNode { id: s.tr },
        ),
        ("delete the bar block", DocEdit::DeleteNode { id: s.bar }),
    ];
    for (label, edit) in cells {
        let applied = match s
            .doc
            .apply(&edit, Tol::witness(), &editor_core::RefusingReach)
        {
            Ok(a) => a,
            Err(e) => {
                eprintln!("CELL {label}: edit refused: {e:?}");
                continue;
            }
        };
        let doc2 = applied.doc;
        let ev2 = run(&doc2, Some(&ev1));
        if ev2.value(s.cut).is_none() {
            eprintln!(
                "CELL {label}: the cut has no value ({:?})\n    -> {}",
                ev2.nodes.get(&s.cut).map(|r| match r {
                    NodeResult::Ok(_) => "ok",
                    NodeResult::Failed(_) => "failed",
                    NodeResult::Poisoned { .. } => "poisoned",
                }),
                describe(&resolve((&doc2, &ev2), (&s.doc, &ev1), &frags[0]))
            );
            continue;
        }
        let p2 = pop(&ev2, s.cut);
        let n2 = side_of_fragments(&ev2, s.cut).len();
        let on_path: Vec<String> = diff_verdicts(&ev1, &ev2)
            .flips_on_path(&frags[0])
            .iter()
            .map(|(n, f)| {
                format!(
                    "{}@{} {:?}->{:?} x{}",
                    f.predicate, n.0, f.from, f.to, f.count
                )
            })
            .collect();
        let res = resolve((&doc2, &ev2), (&s.doc, &ev1), &frags[0]);
        eprintln!(
            "CELL {label}\n    pop2(N,Z,P)={p2:?} sideof-frags2={n2} on-path flips={on_path:?}\n    -> {}",
            describe(&res)
        );
    }
}

// ---------------------------------------------------------------
// 2. Ladder-order rows on the REAL document: these go red if the
//    trigger or the order changes, where the unit's hand-built rows
//    (whose partners are body rows, so `face_at` never yields a face)
//    are green whatever the rung does.
// ---------------------------------------------------------------

#[test]
fn a_recorded_population_in_both_runs_keeps_the_rung_out_on_the_real_document() {
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let doc2 = step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.tr,
            // RE-AIMED at the fix pass: the cell moved from +5.0 to
            // −5.0 (the bar crosses to the far side instead of
            // withdrawing on the side it was already on). The row's
            // subject is unchanged; what changed under the fix is that
            // withdrawing re-qualifies NOTHING — the survivor still
            // satisfies the vanished name's own verdict vector — so
            // the rung honestly answers nothing there and this row
            // needs a cell where a side actually moves.
            slot: SlotId::Translation(Axis3::X),
            expr: len(-5.0),
        },
    )
    .0;
    let mut ev2 = run(&doc2, Some(&ev1));
    assert_eq!(
        pop(&ev2, s.cut),
        [0, 0, 0],
        "premise: the pruned run's pair population is empty"
    );
    let honest = resolve((&doc2, &ev2), (&s.doc, &ev1), &frags[0]);
    assert!(is_shadow(vanished(&honest)));
    // One recorded pair verdict in the current run: the population is
    // no longer empty on either side, so the trigger is false.
    splice_log(&mut ev2, s.cut, |log| {
        log.push(v("name_frag_side_of", Sign::Positive))
    });
    let res = resolve((&doc2, &ev2), (&s.doc, &ev1), &frags[0]);
    eprintln!(
        "with a recorded population on both sides -> {}",
        describe(&res)
    );
    assert!(
        !is_shadow(vanished(&res))
            && !matches!(vanished(&res), Some(Diagnosis::ShadowExecDeclined { .. })),
        "the rung must not fire on a recorded population: {res:?}"
    );
}

#[test]
fn a_recorded_discriminator_flip_beats_the_rung_on_the_real_document() {
    let s = slot();
    let mut ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let doc2 = step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.tr,
            // RE-AIMED at the fix pass: the cell moved from +5.0 to
            // −5.0 (the bar crosses to the far side instead of
            // withdrawing on the side it was already on). The row's
            // subject is unchanged; what changed under the fix is that
            // withdrawing re-qualifies NOTHING — the survivor still
            // satisfies the vanished name's own verdict vector — so
            // the rung honestly answers nothing there and this row
            // needs a cell where a side actually moves.
            slot: SlotId::Translation(Axis3::X),
            expr: len(-5.0),
        },
    )
    .0;
    let mut ev2 = run(&doc2, Some(&ev1));
    // A recorded `name_frag_order_along` flip at the minting node, with
    // the `name_frag_side_of` population still empty in the pruned run,
    // so the rung WOULD recover a flip if it were reached first. The
    // pruned run records no `order_along` either, so the two logs are
    // first stripped of it: one Positive against one Negative is then
    // a FLIP, where one more on each side of unequal populations would
    // only be divergence.
    splice_log(&mut ev1, s.cut, |log| {
        log.retain(|w| w.predicate != "name_frag_order_along");
        log.push(v("name_frag_order_along", Sign::Positive))
    });
    splice_log(&mut ev2, s.cut, |log| {
        log.retain(|w| w.predicate != "name_frag_order_along");
        log.push(v("name_frag_order_along", Sign::Negative))
    });
    assert_eq!(pop(&ev2, s.cut), [0, 0, 0]);
    let res = resolve((&doc2, &ev2), (&s.doc, &ev1), &frags[0]);
    eprintln!(
        "recorded order_along flip vs shadow side_of -> {}",
        describe(&res)
    );
    assert_eq!(
        vanished(&res),
        Some(&Diagnosis::PredicateFlip {
            predicate: "name_frag_order_along",
            from: Sign::Positive,
            to: Sign::Negative,
            source: FlipSource::VerdictLog,
        })
    );
}

#[test]
fn the_prior_only_empty_direction_also_triggers() {
    // The trigger is "empty in ONE of the two runs". A prior run whose
    // pair population is stripped (the current one keeps its own,
    // here the honest empty one) must still trigger; and a prior run
    // stripped while the current run RECORDS the pair is the reverse
    // direction, which real runs never produce (a SideOf-bearing name
    // is minted by the probes that record the population).
    let s = slot();
    let mut ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let doc2 = step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.tr,
            // RE-AIMED at the fix pass: the cell moved from +5.0 to
            // −5.0 (the bar crosses to the far side instead of
            // withdrawing on the side it was already on). The row's
            // subject is unchanged; what changed under the fix is that
            // withdrawing re-qualifies NOTHING — the survivor still
            // satisfies the vanished name's own verdict vector — so
            // the rung honestly answers nothing there and this row
            // needs a cell where a side actually moves.
            slot: SlotId::Translation(Axis3::X),
            expr: len(-5.0),
        },
    )
    .0;
    let mut ev2 = run(&doc2, Some(&ev1));
    splice_log(&mut ev1, s.cut, |log| {
        log.retain(|w| w.predicate != "name_frag_side_of")
    });
    assert_eq!(pop(&ev1, s.cut), [0, 0, 0]);
    let both_empty = resolve((&doc2, &ev2), (&s.doc, &ev1), &frags[0]);
    eprintln!("both populations empty -> {}", describe(&both_empty));
    assert!(is_shadow(vanished(&both_empty)));
    splice_log(&mut ev2, s.cut, |log| {
        log.push(v("name_frag_side_of", Sign::Negative))
    });
    let prior_only = resolve((&doc2, &ev2), (&s.doc, &ev1), &frags[0]);
    eprintln!("prior empty, current recorded -> {}", describe(&prior_only));
    assert!(is_shadow(vanished(&prior_only)), "{prior_only:?}");
}

// ---------------------------------------------------------------
// 3. The prior/current CONTEXT attacks.
// ---------------------------------------------------------------

/// A second evaluation of `doc` with `nodes` removed from its result
/// map (the evaluation type is not `Clone`; its fields are public).
/// The partner names are minted at the bar's EXTRUDE node and carried
/// through the transform's table, so making them unresolvable means
/// removing both carrying nodes.
fn without_nodes(doc: &ProfileDoc, nodes: &[RecipeNodeId]) -> Evaluation<f64> {
    let mut ev = run(doc, None);
    for n in nodes {
        ev.nodes.remove(n);
    }
    ev
}

#[test]
fn partner_node_value_missing_in_the_prior_falls_through_without_panic() {
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let doc2 = step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.tr,
            // RE-AIMED at the fix pass: the cell moved from +5.0 to
            // −5.0 (the bar crosses to the far side instead of
            // withdrawing on the side it was already on). The row's
            // subject is unchanged; what changed under the fix is that
            // withdrawing re-qualifies NOTHING — the survivor still
            // satisfies the vanished name's own verdict vector — so
            // the rung honestly answers nothing there and this row
            // needs a cell where a side actually moves.
            slot: SlotId::Translation(Axis3::X),
            expr: len(-5.0),
        },
    )
    .0;
    let ev2 = run(&doc2, Some(&ev1));
    let prior_missing_partner = without_nodes(&s.doc, &[s.bar, s.tr]);
    let res = resolve((&doc2, &ev2), (&s.doc, &prior_missing_partner), &frags[0]);
    eprintln!("prior without the partner node -> {}", describe(&res));
    assert!(
        !is_shadow(vanished(&res)),
        "no partner context in the prior: the rung cannot have executed: {res:?}"
    );
}

#[test]
fn partner_node_value_missing_in_the_current_run_is_a_cascade() {
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let doc2 = step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.tr,
            // RE-AIMED at the fix pass: the cell moved from +5.0 to
            // −5.0 (the bar crosses to the far side instead of
            // withdrawing on the side it was already on). The row's
            // subject is unchanged; what changed under the fix is that
            // withdrawing re-qualifies NOTHING — the survivor still
            // satisfies the vanished name's own verdict vector — so
            // the rung honestly answers nothing there and this row
            // needs a cell where a side actually moves.
            slot: SlotId::Translation(Axis3::X),
            expr: len(-5.0),
        },
    )
    .0;
    let ev2 = without_nodes(&doc2, &[s.bar, s.tr]);
    let res = resolve((&doc2, &ev2), (&s.doc, &ev1), &frags[0]);
    eprintln!("current without the partner node -> {}", describe(&res));
    assert!(
        matches!(vanished(&res), Some(Diagnosis::Cascade { .. })),
        "an unresolvable partner is the Cascade rung's, above the flip stage: {res:?}"
    );
}

#[test]
fn the_rung_reads_the_prior_payload_not_only_the_prior_table() {
    // The prior's cut node keeps its NAME TABLE (so the fragment's
    // face key still resolves) but carries the CURRENT run's payload —
    // the whole, un-notched plate. A rung that reads the payload
    // answers differently from the honest run, or declines; a rung
    // that answered identically would not be reading the geometry.
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let doc2 = step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.tr,
            // RE-AIMED at the fix pass: the cell moved from +5.0 to
            // −5.0 (the bar crosses to the far side instead of
            // withdrawing on the side it was already on). The row's
            // subject is unchanged; what changed under the fix is that
            // withdrawing re-qualifies NOTHING — the survivor still
            // satisfies the vanished name's own verdict vector — so
            // the rung honestly answers nothing there and this row
            // needs a cell where a side actually moves.
            slot: SlotId::Translation(Axis3::X),
            expr: len(-5.0),
        },
    )
    .0;
    let ev2 = run(&doc2, Some(&ev1));
    let honest = resolve((&doc2, &ev2), (&s.doc, &ev1), &frags[0]);
    let mut poisoned = run(&s.doc, None);
    let current_payload = ev2.value(s.cut).unwrap().payload.clone();
    let Some(NodeResult::Ok(pv)) = poisoned.nodes.get_mut(&s.cut) else {
        panic!("the cut has a value");
    };
    pv.payload = current_payload;
    let over_poisoned_prior = resolve((&doc2, &ev2), (&s.doc, &poisoned), &frags[0]);
    eprintln!(
        "honest -> {}\npoisoned prior payload -> {}",
        describe(&honest),
        describe(&over_poisoned_prior)
    );
    assert!(is_shadow(vanished(&honest)));
    assert_ne!(
        vanished(&honest),
        vanished(&over_poisoned_prior),
        "the prior side is decided from the prior payload"
    );
}

#[test]
fn survivor_absent_in_the_current_run_falls_through_without_panic() {
    // The current run's cut table is rebuilt WITHOUT the survivor (the
    // fragment name minus its qualifier). Tied rows are dropped too;
    // none of them is the survivor.
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let mut survivor = frags[0].clone();
    survivor.path.pop();
    let doc2 = step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.tr,
            // RE-AIMED at the fix pass: the cell moved from +5.0 to
            // −5.0 (the bar crosses to the far side instead of
            // withdrawing on the side it was already on). The row's
            // subject is unchanged; what changed under the fix is that
            // withdrawing re-qualifies NOTHING — the survivor still
            // satisfies the vanished name's own verdict vector — so
            // the rung honestly answers nothing there and this row
            // needs a cell where a side actually moves.
            slot: SlotId::Translation(Axis3::X),
            expr: len(-5.0),
        },
    )
    .0;
    let mut ev2 = run(&doc2, Some(&ev1));
    let old = ev2.value(s.cut).unwrap().name_table.clone();
    assert!(
        old.lookup(&survivor).is_some(),
        "premise: the survivor exists in the honest run"
    );
    let mut t = NameTable::new();
    for (n, e) in old.iter() {
        if let Entry::Unique(ent) = e
            && *n != survivor
        {
            t.insert(n.clone(), *ent).unwrap();
        }
    }
    let Some(NodeResult::Ok(nv)) = ev2.nodes.get_mut(&s.cut) else {
        panic!("the cut has a value");
    };
    nv.name_table = Arc::new(t);
    let res = resolve((&doc2, &ev2), (&s.doc, &ev1), &frags[0]);
    eprintln!("survivor absent -> {}", describe(&res));
    assert!(
        !is_shadow(vanished(&res)),
        "no survivor to probe: the rung cannot have executed: {res:?}"
    );
}

#[test]
fn a_stacked_qualifier_name_declines_or_answers_typed() {
    // A name whose SideOf qualifier is NOT trailing: the same fragment
    // with an OrderAlong rank stacked on top. The rung reads only the
    // trailing segment, so it must decline (documented fall-through)
    // rather than probe or panic. Also the reverse stack: a bare
    // SideOf under a second SideOf whose survivor keeps the first.
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let doc2 = step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.tr,
            // RE-AIMED at the fix pass: the cell moved from +5.0 to
            // −5.0 (the bar crosses to the far side instead of
            // withdrawing on the side it was already on). The row's
            // subject is unchanged; what changed under the fix is that
            // withdrawing re-qualifies NOTHING — the survivor still
            // satisfies the vanished name's own verdict vector — so
            // the rung honestly answers nothing there and this row
            // needs a cell where a side actually moves.
            slot: SlotId::Translation(Axis3::X),
            expr: len(-5.0),
        },
    )
    .0;
    let ev2 = run(&doc2, Some(&ev1));
    let mut ranked = frags[0].clone();
    ranked
        .path
        .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank: 0, of: 2 }));
    let res = resolve((&doc2, &ev2), (&s.doc, &ev1), &ranked);
    eprintln!("SideOf under OrderAlong -> {}", describe(&res));
    assert!(!is_shadow(vanished(&res)), "{res:?}");
    let Some(RoleSeg::Fragment(Qualifier::SideOf(vector))) = frags[0].path.last().cloned() else {
        panic!("a SideOf fragment");
    };
    let mut doubled = frags[0].clone();
    doubled
        .path
        .push(RoleSeg::Fragment(Qualifier::SideOf(vector)));
    let res = resolve((&doc2, &ev2), (&s.doc, &ev1), &doubled);
    eprintln!(
        "SideOf under SideOf (survivor = the prior fragment) -> {}",
        describe(&res)
    );
}

/// RED on the frozen head by design, and GREEN after the fix: the row
/// pins that the partner is read at the boolean's OPERAND.
#[test]
fn a_partner_behind_a_transform_is_probed_against_which_body() {
    // The partner names are minted at the bar's extrude node and
    // carried (same rows) through the transform's table, so a table
    // scan finds them at the FIRST carrying node and reads their
    // carrier off the UNTRANSFORMED bar.
    //
    // RE-AIMED at the fix pass, and the discriminator is sharper than
    // it was: the current cell moves the bar ACROSS (to −5.5) instead
    // of withdrawing it. Read at the operand, the current carriers are
    // the walls at x = −4.5 and −3.5 and the surviving cap is
    // definitely on one side of each — a side that changed, which is
    // the recovered flip. Read at the extrude, the current carriers do
    // not move with the transform AT ALL: they stay at x = 1 and 2,
    // the 0..3 cap straddles both, every probe aggregates to `Mixed`,
    // and the rung finds nothing. So this row reds outright on the
    // unplaced reading.
    let s = slot();
    let doc1 = step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.tr,
            slot: SlotId::Translation(Axis3::X),
            expr: len(0.5),
        },
    )
    .0;
    let ev1 = run(&doc1, None);
    let frags = side_of_fragments(&ev1, s.cut);
    assert_eq!(frags.len(), 2);
    let Some(RoleSeg::Fragment(Qualifier::SideOf(vector))) = frags[0].path.last() else {
        panic!("a SideOf fragment");
    };
    for (partner, verdict) in vector {
        let at = editor_core::resolve(
            RunCtx {
                doc: &doc1,
                eval: &ev1,
            },
            partner,
        );
        eprintln!(
            "partner {partner:?} ({verdict:?}) resolves at {at:?}; extrude = {:?}, transform = {:?}",
            s.bar, s.tr
        );
    }
    let doc2 = step(
        doc1.clone(),
        DocEdit::SetParam {
            node: s.tr,
            slot: SlotId::Translation(Axis3::X),
            expr: len(-5.5),
        },
    )
    .0;
    let ev2 = run(&doc2, Some(&ev1));
    assert_eq!(pop(&ev2, s.cut), [0, 0, 0]);
    let res = resolve((&doc2, &ev2), (&doc1, &ev1), &frags[0]);
    eprintln!("prior bar at +0.5, current at −5.5 -> {}", describe(&res));
    // RE-AIMED at the fix pass (the row is kept, its expectation is
    // not): the redesign reports the first PARTNER whose side VERDICT
    // changed, through the emission's own aggregation rule, so the
    // honest answer here is a definite side flip naming a partner —
    // not the pooled residual this row was written against. What the
    // row still pins is the MAJOR it was written for: the partner is
    // read at the boolean's operand, so a bar behind a `Transform` is
    // probed against the PLACED wall. Against the untransformed
    // extrude the prior side would read the same as the current one
    // and the rung would find no flip at all.
    let Some(Diagnosis::PredicateFlip {
        predicate,
        from,
        to,
        source: FlipSource::ShadowExec { partner },
    }) = vanished(&res)
    else {
        panic!("expected the recovered flip, got {}", describe(&res));
    };
    assert_eq!(*predicate, "name_frag_side_of");
    assert_ne!(from, to, "a flip names two different sides");
    assert_eq!(
        partner.node, s.bar,
        "the partner is the bar's own operand-node name"
    );
}

// ---------------------------------------------------------------
// 4. The trigger's granularity: NODE, not pair. Two bars cross the
//    same cap; only one moves away. The vanished fragment's own pair
//    is pruned, but the node still records `name_frag_side_of` for
//    the other bar's fragments, so the rung never fires.
// ---------------------------------------------------------------

#[test]
fn a_second_pair_at_the_node_keeps_the_rung_out_of_a_pruned_one() {
    let doc = ProfileDoc::empty_derived("bool7r1-two", Tol::witness());
    let (doc, a) = block(doc, (0.0, 6.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, b1) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, b2) = block(doc, (4.0, 5.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, tr) = insert(
        doc,
        Node::Transform {
            input: b1,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let (doc, tool) = insert(
        doc,
        Node::Union {
            members: vec![tr, b2],
            declare: None,
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: tool,
            declare: None,
        },
    );
    let ev1 = run(&doc, None);
    let frags = side_of_fragments(&ev1, cut);
    eprintln!(
        "two-bar prior: {} SideOf fragments, pop {:?}",
        frags.len(),
        pop(&ev1, cut)
    );
    for f in &frags {
        eprintln!("   {f}");
    }
    assert!(
        frags.len() >= 3,
        "two bars split the cap into at least three fragments"
    );
    let doc2 = step(
        doc.clone(),
        DocEdit::SetParam {
            node: tr,
            slot: SlotId::Translation(Axis3::X),
            expr: len(20.0),
        },
    )
    .0;
    let ev2 = run(&doc2, Some(&ev1));
    let frags2 = side_of_fragments(&ev2, cut);
    eprintln!(
        "two-bar current: {} SideOf fragments, pop {:?}",
        frags2.len(),
        pop(&ev2, cut)
    );
    for f in &frags2 {
        eprintln!("   {f}");
    }
    assert_ne!(
        pop(&ev2, cut),
        [0, 0, 0],
        "the node still records the other bar's pair"
    );
    for f in &frags {
        let res = resolve((&doc2, &ev2), (&doc, &ev1), f);
        let on_path: Vec<String> = diff_verdicts(&ev1, &ev2)
            .flips_on_path(f)
            .iter()
            .map(|(n, fl)| {
                format!(
                    "{}@{} {:?}->{:?} x{}",
                    fl.predicate, n.0, fl.from, fl.to, fl.count
                )
            })
            .collect();
        eprintln!(
            "  {f}\n     on-path {on_path:?}\n     -> {}",
            describe(&res)
        );
        assert!(
            !is_shadow(vanished(&res)),
            "the trigger is node-granular: a recorded population at the node keeps the rung out"
        );
    }
}

// ---------------------------------------------------------------
// 5. The ceiling's measurement, printed.
// ---------------------------------------------------------------

#[test]
fn the_corpus_widest_side_of_vector_measured() {
    let mut widest = 0usize;
    let mut at = "";
    for cd in crate::corpus::documents() {
        let ev = run(&cd.doc, None);
        let mut doc_widest = 0usize;
        for res in ev.nodes.values() {
            let NodeResult::Ok(v) = res else {
                continue;
            };
            for (n, _) in v.name_table.iter() {
                for seg in &n.path {
                    if let RoleSeg::Fragment(Qualifier::SideOf(vector)) = seg {
                        doc_widest = doc_widest.max(vector.len());
                    }
                }
            }
        }
        eprintln!("corpus {:<28} widest SideOf vector {doc_widest}", cd.name);
        if doc_widest > widest {
            widest = doc_widest;
            at = cd.name;
        }
    }
    eprintln!("widest = {widest} at {at}; ceiling = {SHADOW_EXEC_MAX_PAIRS}");
    assert_eq!(
        (widest, at),
        (12, "nested_islands_106_depth2"),
        "the PR's measurement"
    );
}
