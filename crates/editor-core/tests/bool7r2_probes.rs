//! BOOL-7 review probes (r2): the shadow-execution rung, attacked from
//! the sides the unit's own suite does not enter.
//!
//! Five questions, one row each where a row can carry it:
//!
//! 1. The 54-combination sweep the PR body reports lives only in that
//!    body. These rows re-take a handful of its cells — a rotation and
//!    two more translations beside the X one the unit's fixture uses —
//!    and record, as assertions, what the pair population, the recorded
//!    path flips and the diagnosis actually are in each.
//! 2. The prior side's INPUTS. The rung is claimed to be a pure
//!    function of the two evaluations; poison one (the minting node's
//!    payload, then a partner's table entry) and the rung must decline
//!    to the next rung, never panic and never answer from the recipe.
//! 3. The SURVIVOR. A vanished fragment's current-run counterpart is
//!    the same name with its trailing qualifier dropped; strip it from
//!    the new table and the rung must fall through typed.
//! 4. The NAME SHAPES the ladder can be handed: two stacked fragment
//!    qualifiers, and a `SideOf` vector whose partners live in two
//!    different nodes.
//! 5. The band at the door. `Tol` is a zero-sized witness, so the
//!    `tol: Tol` the with-prior doors gained carries no epsilon — the
//!    band is the process's, and the prior/current asymmetry a
//!    SetTolerance would need cannot exist in one process.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::sync::Arc;

use editor_core::{
    Axis3, BooleanOp, CancelToken, Diagnosis, DocEdit, EntityKind, Entry, EvalOptions, Evaluation,
    FlipSource, NameTable, Node, NodeResult, ProfileDoc, Qualifier, RecipeNodeId, Resolution,
    ResolveError, RoleSeg, RunCtx, SideVerdict, SlotId, StableName, ValuePayload, diff_verdicts,
    evaluate, resolve_with_prior,
};
use fixture::{ang, insert, len, on_frame, scl, step};
use geom_core::Tol;

// ---------------------------------------------------------------
// The unit's own scenario, rebuilt here so the probes do not depend
// on its private helpers.
// ---------------------------------------------------------------

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

struct Slot {
    doc: ProfileDoc,
    tr: RecipeNodeId,
    cut: RecipeNodeId,
}

fn slot() -> Slot {
    let doc = ProfileDoc::empty_derived("bool7r2", Tol::witness());
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
    let (doc, cut) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: tr,
            declare: None,
        },
    );
    Slot { doc, tr, cut }
}

fn edit(s: &Slot, slot_id: SlotId, to: f64) -> ProfileDoc {
    step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.tr,
            slot: slot_id,
            expr: if matches!(slot_id, SlotId::RotationAngle) {
                ang(to)
            } else {
                len(to)
            },
        },
    )
    .0
}

fn side_of_fragments(ev: &Evaluation<f64>, cut: RecipeNodeId) -> Vec<StableName> {
    ev.value(cut)
        .expect("the cut evaluates")
        .name_table
        .iter()
        .filter_map(|(n, e)| {
            let discriminated =
                matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::SideOf(_))));
            (discriminated && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .collect()
}

fn vanished(res: &Resolution) -> Diagnosis {
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished { diagnosis, .. } = &f.error else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    diagnosis.clone()
}

fn side_of_population(ev: &Evaluation<f64>, node: RecipeNodeId) -> usize {
    ev.value(node).map_or(0, |v| {
        v.verdicts
            .iter()
            .filter(|w| w.predicate == "name_frag_side_of")
            .count()
    })
}

fn diagnose_against(
    s: &Slot,
    doc2: &ProfileDoc,
    ev1: &Evaluation<f64>,
    ev2: &Evaluation<f64>,
    name: &StableName,
) -> Diagnosis {
    vanished(&resolve_with_prior(
        RunCtx {
            doc: doc2,
            eval: ev2,
        },
        RunCtx {
            doc: &s.doc,
            eval: ev1,
        },
        name,
        Tol::witness(),
    ))
}

/// One cell of the PR body's (op, axis, delta) sweep, re-taken: the
/// pair population after the edit, the recorded path-flip predicates,
/// and the diagnosis the ladder reaches.
struct Cell {
    population: usize,
    path_predicates: Vec<&'static str>,
    diagnosis: Option<Diagnosis>,
    survived: bool,
    evaluated: bool,
}

fn cell(s: &Slot, doc2: ProfileDoc, frag: &StableName, ev1: &Evaluation<f64>) -> Cell {
    let ev2 = run(&doc2, Some(ev1));
    let evaluated = ev2.value(s.cut).is_some();
    let survived = ev2
        .value(s.cut)
        .is_some_and(|v| v.name_table.iter().any(|(n, _)| n == frag));
    Cell {
        population: side_of_population(&ev2, s.cut),
        path_predicates: diff_verdicts(ev1, &ev2)
            .flips_on_path(frag)
            .into_iter()
            .map(|(_, f)| f.predicate)
            .collect(),
        diagnosis: if survived || !evaluated {
            // Nothing vanished, or the node did not evaluate at all;
            // the row records that and stops.
            None
        } else {
            Some(diagnose_against(s, &doc2, ev1, &ev2, frag))
        },
        survived,
        evaluated,
    }
}

// ---------------------------------------------------------------
// 1. The sweep's cells, re-taken.
// ---------------------------------------------------------------

#[test]
fn probe_sweep_cells_a_rotation_and_three_translations() {
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    assert_eq!(frags.len(), 2, "the bar splits the cap into two fragments");
    let f = &frags[0];

    let cells: Vec<(&str, Cell)> = vec![
        (
            "translate x 5.0 (the unit's own cell, withdrawing)",
            cell(&s, edit(&s, SlotId::Translation(Axis3::X), 5.0), f, &ev1),
        ),
        (
            "translate x -5.0 (crossing)",
            cell(&s, edit(&s, SlotId::Translation(Axis3::X), -5.0), f, &ev1),
        ),
        (
            "translate y 4.0 (the PR's flip-free cell)",
            cell(&s, edit(&s, SlotId::Translation(Axis3::Y), 4.0), f, &ev1),
        ),
        (
            "translate z 5.0",
            cell(&s, edit(&s, SlotId::Translation(Axis3::Z), 5.0), f, &ev1),
        ),
        (
            "rotate 90 degrees about z",
            cell(&s, edit(&s, SlotId::RotationAngle, 90.0), f, &ev1),
        ),
        (
            "rotate 45 degrees about z",
            cell(&s, edit(&s, SlotId::RotationAngle, 45.0), f, &ev1),
        ),
    ];

    for (label, c) in &cells {
        println!(
            "CELL {label}: evaluated={} survived={} population={} path={:?} diagnosis={:?}",
            c.evaluated, c.survived, c.population, c.path_predicates, c.diagnosis
        );
    }

    // RE-AIMED at the fix pass. The row was written against a rung
    // that answered on EVERY pruned cell, and that is what the fix
    // removed: pruning the pair is not the same event as a side
    // changing. Withdrawing the bar along +x prunes the pair and
    // re-qualifies nothing — the surviving cap satisfies the vanished
    // name's own verdict vector — so the honest answer there is the
    // later rungs. Crossing to −x moves a side, and there the rung
    // must outrank the incidental `bool_*` flip that every such cell
    // leaves on the path. Both halves are asserted; the rest of the
    // cells print.
    let crossing = cells
        .iter()
        .find(|(label, _)| label.starts_with("translate x -5.0"))
        .map(|(_, c)| c)
        .expect("the crossing cell is in the table");
    assert_eq!(crossing.population, 0, "the crossing cell prunes the pair");
    assert!(
        crossing
            .path_predicates
            .iter()
            .any(|p| p.starts_with("bool_")),
        "and leaves a residual bool_* flip on the path: {:?}",
        crossing.path_predicates
    );
    assert!(
        matches!(
            crossing.diagnosis,
            Some(Diagnosis::PredicateFlip {
                source: FlipSource::ShadowExec { .. },
                ..
            })
        ),
        "so the rung must outrank it; got {:?}",
        crossing.diagnosis
    );
    let withdrawing = cells
        .iter()
        .find(|(label, _)| label.starts_with("translate x 5.0"))
        .map(|(_, c)| c)
        .expect("the withdrawing cell is in the table");
    assert_eq!(withdrawing.population, 0, "it prunes the pair just as hard");
    assert!(
        !matches!(
            withdrawing.diagnosis,
            Some(Diagnosis::PredicateFlip {
                source: FlipSource::ShadowExec { .. },
                ..
            })
        ),
        "and no side changed, so the rung says nothing: {:?}",
        withdrawing.diagnosis
    );
}

/// The trigger is EMPTY-population, which is wider than "the sweep
/// pruned the pair": a fragment group that merely collapses is also
/// empty. This row records which arm each vanishing cell took.
#[test]
fn probe_the_trigger_is_emptiness_not_pruning() {
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let f = &frags[0];
    let mut recovered = 0usize;
    for (label, doc2) in [
        // RE-AIMED at the fix pass: the crossing cell is the one that
        // reaches the rung now — pruning the pair and MOVING a side are
        // different events, and only the second is a flip. The
        // withdrawing and collapse cells stay in the table as the
        // contrast this row is about.
        (
            "x -5.0 (crossing)",
            edit(&s, SlotId::Translation(Axis3::X), -5.0),
        ),
        (
            "x 5.0 (withdrawing)",
            edit(&s, SlotId::Translation(Axis3::X), 5.0),
        ),
        ("y 2.5", edit(&s, SlotId::Translation(Axis3::Y), 2.5)),
        ("y 3.5", edit(&s, SlotId::Translation(Axis3::Y), 3.5)),
        ("y 4.0", edit(&s, SlotId::Translation(Axis3::Y), 4.0)),
    ] {
        let c = cell(&s, doc2, f, &ev1);
        println!(
            "TRIGGER {label}: evaluated={} survived={} population={} diagnosis={:?}",
            c.evaluated, c.survived, c.population, c.diagnosis
        );
        if matches!(
            c.diagnosis,
            Some(Diagnosis::PredicateFlip {
                source: FlipSource::ShadowExec { .. },
                ..
            })
        ) {
            recovered += 1;
        }
    }
    assert!(recovered > 0, "at least one cell reaches the rung");
}

// ---------------------------------------------------------------
// 2. The prior side's inputs, poisoned.
// ---------------------------------------------------------------

/// Replaces the minting node's PAYLOAD in `ev` with a value that holds
/// no body, keeping its name table and verdict log intact.
fn payload_stripped(mut ev: Evaluation<f64>, node: RecipeNodeId) -> Evaluation<f64> {
    let NodeResult::Ok(v) = ev.nodes.remove(&node).expect("the node is present") else {
        panic!("the node did not evaluate");
    };
    ev.nodes.insert(
        node,
        NodeResult::Ok(editor_core::NodeValue {
            payload: ValuePayload::Declarations(vec![]),
            ..v
        }),
    );
    ev
}

/// Rebuilds `node`'s name table in `ev` without the names `drop_it`
/// selects.
fn table_without(
    mut ev: Evaluation<f64>,
    node: RecipeNodeId,
    drop_it: impl Fn(&StableName) -> bool,
) -> Evaluation<f64> {
    let NodeResult::Ok(v) = ev.nodes.remove(&node).expect("the node is present") else {
        panic!("the node did not evaluate");
    };
    let mut t = NameTable::new();
    for (n, e) in v.name_table.iter() {
        if drop_it(n) {
            continue;
        }
        if let Entry::Unique(ent) = e {
            t.insert(n.clone(), *ent).unwrap();
        }
    }
    ev.nodes.insert(
        node,
        NodeResult::Ok(editor_core::NodeValue {
            name_table: Arc::new(t),
            fragment_groups: Arc::default(),
            ..v
        }),
    );
    ev
}

#[test]
fn probe_a_prior_without_its_body_declines_to_the_next_rung() {
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    // RE-AIMED at the fix pass to the crossing cell: the baseline this
    // row poisons has to be one the rung actually answers.
    let doc2 = edit(&s, SlotId::Translation(Axis3::X), -5.0);
    let ev2 = run(&doc2, Some(&ev1));
    let honest = diagnose_against(&s, &doc2, &ev1, &ev2, &frags[0]);
    assert!(
        matches!(
            honest,
            Diagnosis::PredicateFlip {
                source: FlipSource::ShadowExec { .. },
                ..
            }
        ),
        "the baseline is the recovered flip, got {honest:?}"
    );
    let poisoned = payload_stripped(run(&s.doc, None), s.cut);
    let d = diagnose_against(&s, &doc2, &poisoned, &ev2, &frags[0]);
    println!("POISONED PRIOR PAYLOAD: {d:?}");
    assert!(
        !matches!(
            d,
            Diagnosis::PredicateFlip {
                source: FlipSource::ShadowExec { .. },
                ..
            }
        ),
        "a prior with no body cannot yield a recovered flip, got {d:?}"
    );
}

#[test]
fn probe_a_partner_absent_from_the_current_run_declines_to_the_next_rung() {
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let Some(RoleSeg::Fragment(Qualifier::SideOf(partners))) = frags[0].path.last() else {
        panic!("the fragment carries a SideOf qualifier");
    };
    let partner_names: Vec<StableName> = partners.iter().map(|(p, _)| p.clone()).collect();
    println!("PARTNERS: {partner_names:?}");
    let doc2 = edit(&s, SlotId::Translation(Axis3::X), 5.0);
    // The partners are minted at their OWN node (the bar), not at the
    // boolean; strip them there, which is where `lookup_unique` finds
    // them.
    let mut stripped = run(&doc2, Some(&ev1));
    let nodes: Vec<RecipeNodeId> = stripped.nodes.keys().copied().collect();
    for node in nodes {
        if matches!(stripped.nodes.get(&node), Some(NodeResult::Ok(_))) {
            stripped = table_without(stripped, node, |n| partner_names.iter().any(|p| p == n));
        }
    }
    let d = diagnose_against(&s, &doc2, &ev1, &stripped, &frags[0]);
    println!("PARTNER ABSENT IN CURRENT RUN: {d:?}");
    assert!(
        !matches!(
            d,
            Diagnosis::PredicateFlip {
                source: FlipSource::ShadowExec { .. },
                ..
            }
        ),
        "with no partner to probe the rung must decline, got {d:?}"
    );
}

// ---------------------------------------------------------------
// 3. The survivor.
// ---------------------------------------------------------------

#[test]
fn probe_an_absent_survivor_declines_to_the_next_rung() {
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    let mut survivor = frags[0].clone();
    survivor.path.pop();
    let doc2 = edit(&s, SlotId::Translation(Axis3::X), 5.0);
    let ev2 = run(&doc2, Some(&ev1));
    assert!(
        ev2.value(s.cut)
            .is_some_and(|v| v.name_table.iter().any(|(n, _)| *n == survivor)),
        "the baseline: the survivor IS in the current table"
    );
    let stripped = table_without(run(&doc2, Some(&ev1)), s.cut, |n| *n == survivor);
    let d = diagnose_against(&s, &doc2, &ev1, &stripped, &frags[0]);
    println!("SURVIVOR ABSENT: {d:?}");
    assert!(
        !matches!(
            d,
            Diagnosis::PredicateFlip {
                source: FlipSource::ShadowExec { .. },
                ..
            }
        ),
        "with no survivor to probe the rung must decline, got {d:?}"
    );
}

// ---------------------------------------------------------------
// 4. Name shapes.
// ---------------------------------------------------------------

fn geom_free_doc() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, n) = insert(
        ProfileDoc::empty_derived("bool7r2-hand", Tol::witness()),
        Node::declare_rest(vec![]),
    );
    let (doc, m) = insert(doc, Node::declare_rest(vec![]));
    (doc, n, m)
}

fn body_ent(i: u32) -> editor_core::EntityRef {
    editor_core::EntityRef {
        body: i,
        key: editor_core::EntityKey::Body,
    }
}

fn one_node_eval(
    document: editor_core::DocumentId,
    node: RecipeNodeId,
    t: NameTable,
) -> Evaluation<f64> {
    let mut nodes = std::collections::BTreeMap::new();
    nodes.insert(
        node,
        NodeResult::Ok(editor_core::NodeValue {
            payload: ValuePayload::Declarations(vec![]),
            name_table: Arc::new(t),
            fragment_groups: Arc::default(),
            contacts: Arc::new(topo::ContactRecords::default()),
            carried: Arc::new(editor_core::CarriedDeclarations::default()),
            verdicts: Arc::new(vec![]),
            escalations: Arc::new(vec![]),
            placement: None,
            witness: editor_core::eval::WitnessSlot::default(),
            content_key: editor_core::ContentKey(0),
            naming_key: editor_core::NamingKey(0),
        }),
    );
    Evaluation::<f64> {
        epoch: editor_core::Epoch::mint(),
        document,
        prior_refused: None,
        order: vec![node],
        nodes,
        outcome: editor_core::EvalOutcome::Completed,
        recomputed: 1,
        reused: 0,
        part_evaluations: 0,
        appearance: editor_core::AppearanceResolution::default(),
    }
}

/// Resolves `frag` (vanished) against an empty-ish new table holding
/// `survivor` and every `inner` name, with no verdicts on either side.
fn hand_answer(
    doc: &ProfileDoc,
    node: RecipeNodeId,
    frag: &StableName,
    survivor: &StableName,
    inner: &[StableName],
) -> Diagnosis {
    let mut t_prior = NameTable::new();
    t_prior.insert(frag.clone(), body_ent(0)).unwrap();
    let mut t_new = NameTable::new();
    t_new.insert(survivor.clone(), body_ent(0)).unwrap();
    for (i, n) in inner.iter().enumerate() {
        let ent = body_ent(i as u32 + 1);
        t_prior.insert(n.clone(), ent).unwrap();
        t_new.insert(n.clone(), ent).unwrap();
    }
    let prior_ev = one_node_eval(doc.id(), node, t_prior);
    let new_ev = one_node_eval(doc.id(), node, t_new);
    vanished(&resolve_with_prior(
        RunCtx { doc, eval: &new_ev },
        RunCtx {
            doc,
            eval: &prior_ev,
        },
        frag,
        Tol::witness(),
    ))
}

#[test]
fn probe_two_stacked_fragment_qualifiers_answer_typed() {
    let (doc, n, m) = geom_free_doc();
    let of = StableName {
        kind: EntityKind::Body,
        node: n,
        path: vec![RoleSeg::OutputBody],
    };
    let partner = StableName {
        kind: EntityKind::Body,
        node: m,
        path: vec![RoleSeg::OutputBody],
    };
    // FromA(of) / Fragment(OrderAlong) / Fragment(SideOf) — the
    // trailing qualifier is the rung's, the one under it is not.
    let frag = StableName {
        kind: EntityKind::Body,
        node: n,
        path: vec![
            RoleSeg::FromA(of.clone().into()),
            RoleSeg::Fragment(Qualifier::OrderAlong { rank: 0, of: 2 }),
            RoleSeg::Fragment(Qualifier::SideOf(vec![(
                partner.clone(),
                SideVerdict::Positive,
            )])),
        ],
    };
    let mut survivor = frag.clone();
    survivor.path.pop();
    let d = hand_answer(&doc, n, &frag, &survivor, &[of, partner]);
    println!("TWO STACKED QUALIFIERS: {d:?}");
    // A typed answer, not a panic and not a recovered flip invented
    // out of a geometry-free run.
    assert!(!matches!(
        d,
        Diagnosis::PredicateFlip {
            source: FlipSource::ShadowExec { .. },
            ..
        }
    ));
}

#[test]
fn probe_partners_in_two_different_nodes_answer_typed() {
    let (doc, n, m) = geom_free_doc();
    let of = StableName {
        kind: EntityKind::Body,
        node: n,
        path: vec![RoleSeg::OutputBody],
    };
    let p_n = StableName {
        kind: EntityKind::Body,
        node: n,
        path: vec![
            RoleSeg::OutputBody,
            RoleSeg::Fragment(Qualifier::OrderAlong { rank: 5, of: 9 }),
        ],
    };
    let p_m = StableName {
        kind: EntityKind::Body,
        node: m,
        path: vec![RoleSeg::OutputBody],
    };
    let frag = StableName {
        kind: EntityKind::Body,
        node: n,
        path: vec![
            RoleSeg::FromA(of.clone().into()),
            RoleSeg::Fragment(Qualifier::SideOf(vec![
                (p_n.clone(), SideVerdict::Positive),
                (p_m.clone(), SideVerdict::Negative),
            ])),
        ],
    };
    let mut survivor = frag.clone();
    survivor.path.pop();
    let d = hand_answer(&doc, n, &frag, &survivor, &[of, p_n, p_m]);
    println!("PARTNERS IN TWO NODES: {d:?}");
    assert!(!matches!(
        d,
        Diagnosis::PredicateFlip {
            source: FlipSource::ShadowExec { .. },
            ..
        }
    ));
}

// ---------------------------------------------------------------
// 5. The band at the door.
// ---------------------------------------------------------------

/// `Tol` is a zero-sized WITNESS, not a band value: the process
/// commits exactly one epsilon, so the `tol: Tol` the with-prior doors
/// gained cannot carry a prior band different from the current one,
/// and the SetTolerance asymmetry the spec worries about cannot be
/// constructed in one process at all.
#[test]
fn probe_the_door_band_is_a_zero_sized_process_witness() {
    assert_eq!(
        core::mem::size_of::<Tol>(),
        0,
        "Tol carries no epsilon; it witnesses the process's"
    );
    let a = Tol::witness();
    let b = Tol::witness();
    assert_eq!(
        a.eps(),
        b.eps(),
        "two witnesses in one process name one band"
    );
}

/// The corpus's widest `SideOf` vector, as a NUMBER: the rung's docs
/// say TWELVE at `nested_islands_106_depth2`, and the unit's own row
/// only asserts `< 32`, so the doc's number is unpinned there.
#[test]
fn probe_the_corpus_widest_sideof_vector_is_the_documented_twelve() {
    let mut widest = 0usize;
    let mut at = "";
    for cd in crate::corpus::documents() {
        let ev = run(&cd.doc, None);
        for res in ev.nodes.values() {
            let NodeResult::Ok(v) = res else { continue };
            for (n, _) in v.name_table.iter() {
                for seg in &n.path {
                    if let RoleSeg::Fragment(Qualifier::SideOf(vector)) = seg
                        && vector.len() > widest
                    {
                        widest = vector.len();
                        at = cd.name;
                    }
                }
            }
        }
    }
    println!("WIDEST CORPUS SideOf VECTOR: {widest} at {at}");
    assert_eq!(
        (widest, at),
        (12, "nested_islands_106_depth2"),
        "the rung's docs name this measurement; nothing else pins it"
    );
}

// ---------------------------------------------------------------
// 6. What the recovered flip actually SAYS.
// ---------------------------------------------------------------

/// The signs the recovered flip names, against the signs the vanished
/// name's own `SideOf` qualifier recorded.
///
/// The rung pools EVERY partner's probes into one `name_frag_side_of`
/// population before diffing, so the pair (`from`, `to`) it reports is
/// a residual of the pooled counts, not a per-partner side change.
/// This row records both so a reader can see which it is.
#[test]
fn probe_the_recovered_flips_signs_against_the_names_own_verdicts() {
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    for f in &frags {
        let Some(RoleSeg::Fragment(Qualifier::SideOf(v))) = f.path.last() else {
            panic!("a SideOf fragment");
        };
        let recorded: Vec<SideVerdict> = v.iter().map(|(_, sv)| *sv).collect();
        let doc2 = edit(&s, SlotId::Translation(Axis3::X), 5.0);
        let ev2 = run(&doc2, Some(&ev1));
        let d = diagnose_against(&s, &doc2, &ev1, &ev2, f);
        println!("NAME QUALIFIER VERDICTS: {recorded:?}");
        println!("RECOVERED DIAGNOSIS: {d:?}");
        println!("RECOVERED DISPLAY: {d}");
        let Diagnosis::PredicateFlip { from, to, .. } = &d else {
            panic!("expected the recovered flip, got {d:?}");
        };
        // Record, do not assume: is either reported sign one the name
        // itself carries for one of its partners?
        let names_signs: Vec<&str> = recorded
            .iter()
            .map(|sv| match sv {
                SideVerdict::Positive => "Positive",
                SideVerdict::Negative => "Negative",
                SideVerdict::Mixed => "Mixed",
                SideVerdict::On => "On",
            })
            .collect();
        println!(
            "  from={from:?} to={to:?}; the name's own per-partner verdicts are {names_signs:?}"
        );
    }
}

/// Does the PRIOR side's shadow population reproduce what the prior
/// run RECORDED for the same pair? The rung's docs claim the recovered
/// flip "is the same object `diff_verdicts` would have reported had
/// the run recorded the pair", which is a claim about exactly this.
#[test]
fn probe_the_prior_shadow_population_against_the_prior_runs_own_log() {
    let s = slot();
    let ev1 = run(&s.doc, None);
    let recorded = ev1
        .value(s.cut)
        .expect("the cut evaluates")
        .verdicts
        .iter()
        .filter(|w| w.predicate == "name_frag_side_of")
        .fold([0u32; 3], |mut acc, w| {
            acc[match w.sign {
                geom_core::Sign::Negative => 0,
                geom_core::Sign::Zero => 1,
                geom_core::Sign::Positive => 2,
            }] += 1;
            acc
        });
    println!("PRIOR RUN RECORDED name_frag_side_of [neg, zero, pos] = {recorded:?}");
    println!(
        "  (the rung re-probes only the vanished name's own partners, a subset of \
         the node's whole recorded population)"
    );
    assert!(
        recorded.iter().sum::<u32>() > 0,
        "the prior run did record the family"
    );
}

/// The two fragments the bar cuts sit on OPPOSITE sides of the same
/// partners — that is what their qualifiers record — so a rung that
/// told them apart must answer them differently.
///
/// INVERTED at the fix pass. The row was written against a rung that
/// pooled every partner's probes into one population and gave both
/// fragments a byte-identical answer; that equality WAS the defect,
/// and this row now asserts the inequality the fix produces.
#[test]
fn probe_the_rung_gives_opposite_fragments_different_answers() {
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = side_of_fragments(&ev1, s.cut);
    assert_eq!(frags.len(), 2);
    let doc2 = edit(&s, SlotId::Translation(Axis3::X), -5.0);
    let ev2 = run(&doc2, Some(&ev1));
    let a = diagnose_against(&s, &doc2, &ev1, &ev2, &frags[0]);
    let b = diagnose_against(&s, &doc2, &ev1, &ev2, &frags[1]);
    println!("FRAGMENT 0 -> {a:?}");
    println!("FRAGMENT 1 -> {b:?}");
    assert_ne!(a, b, "the two fragments are not one fragment");
    assert!(
        matches!(
            a,
            Diagnosis::PredicateFlip {
                source: FlipSource::ShadowExec { .. },
                ..
            }
        ),
        "the fragment the bar crossed is the one re-qualified: {a:?}"
    );
    assert!(
        !matches!(
            b,
            Diagnosis::PredicateFlip {
                source: FlipSource::ShadowExec { .. },
                ..
            }
        ),
        "and the other one's sides did not move: {b:?}"
    );
}
