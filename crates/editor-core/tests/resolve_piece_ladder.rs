//! The `Vanished` ladder's rungs for a split face's pieces: the border
//! delta (N5's qualifier-delta rung for a face piece), the group-size
//! rung, and the recorded flips above them.
//!
//! A piece is named by the divider walls it borders (`Borders`, N2),
//! so a piece whose walls changed is read off the names: the vanished
//! name's walls against those of the nearest piece its parent is held
//! as now. A group that stopped being divided has no such piece, and
//! the group-size rung states the count; a recorded flip on the path
//! outranks both, because it names a cause.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::sync::Arc;

use editor_core::eval::WitnessSlot;
use editor_core::{
    Axis3, BooleanOp, CancelToken, ContentKey, Diagnosis, DocEdit, EntityKind, Entry, EvalOptions,
    EvalOutcome, Evaluation, FragmentGroups, GroupCutters, NameTable, NamingKey, Node, ProfileDoc,
    Qualifier, RecipeNodeId, Resolution, ResolveError, RoleSeg, RunCtx, SlotId, StableName,
    evaluate, resolve_with_prior,
};
use fixture::{ang, insert, len, minted, on_frame, scl, step};
use geom_core::Tol;
use geom_core::k_stats::Verdict;

// ---------------------------------------------------------------
// The document: a bar crossing a plate's end cap, so the cap
// descends as a multi-fragment group discriminated against the
// bar's walls.
// ---------------------------------------------------------------

fn run(doc: &editor_core::ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
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

/// The scenario: plate `a` (3×3×1), bar `b` crossing its end cap
/// fully in y behind a `Transform`, subtracted at `cut`.
struct Slot {
    doc: ProfileDoc,
    /// The bar's own extrude, whose faces the cut's seams name.
    bar: RecipeNodeId,
    tr: RecipeNodeId,
    cut: RecipeNodeId,
}

/// The bar's wall over profile segment `segment` (`block`'s profile
/// runs counter-clockwise from `(x0, y0)`: wall 0 is y = y0, 1 is
/// x = x1, 3 is x = x0).
fn wall(doc: &ProfileDoc, bar: RecipeNodeId, segment: u32) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: bar,
        path: vec![RoleSeg::Lateral(crate::fixture::piece(
            doc,
            bar,
            0,
            segment as usize,
        ))],
    }
}

fn slot() -> Slot {
    let doc = ProfileDoc::empty_derived("bool7", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, b0) = block(doc, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
    let (doc, tr) = insert(
        doc,
        Node::transform(
            b0,
            editor_core::Step::Rigid {
                translation: [len(0.0), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
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
    Slot {
        doc,
        bar: b0,
        tr,
        cut,
    }
}

/// Slides the bar along `axis` — the interaction-boundary edit.
fn slide(s: &Slot, axis: Axis3, to: f64) -> ProfileDoc {
    step(
        s.doc.clone(),
        DocEdit::SetParam {
            node: s.tr,
            slot: SlotId::Translation(axis),
            expr: len(to),
        },
    )
    .0
}

/// The `Borders`-qualified cap pieces of `cut`'s table, left to right.
fn border_pieces(ev: &Evaluation<f64>, cut: RecipeNodeId) -> Vec<StableName> {
    let fragments = ev
        .value(cut)
        .expect("the cut evaluates")
        .name_table
        .iter()
        .filter_map(|(n, e)| {
            let discriminated = matches!(
                n.path.last(),
                Some(RoleSeg::Fragment(Qualifier::Borders(_)))
            );
            (discriminated && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .collect();
    fixture::left_to_right(ev, cut, fragments)
}

/// A fragment name's base: the name without its trailing `Fragment`
/// qualifier (the suite's one spelling of that pop).
fn base_of(name: &StableName) -> StableName {
    assert!(
        matches!(name.path.last(), Some(RoleSeg::Fragment(_))),
        "{name:?} has no fragment tail"
    );
    let mut base = name.clone();
    base.path.pop();
    base
}

fn failure(res: &Resolution) -> &editor_core::ResolutionFailure {
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    f
}

fn vanished(res: &Resolution) -> &Diagnosis {
    let Resolution::Failed(f) = res else {
        panic!("expected Failed, got {res:?}");
    };
    let ResolveError::Vanished { diagnosis, .. } = &f.error else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    diagnosis
}

// ---------------------------------------------------------------
// On the real document.
// ---------------------------------------------------------------

#[test]
fn a_vanished_rim_piece_is_diagnosed_as_its_group_resizing() {
    // The corpus's own pruned-pair row: the rim piece's group
    // goes from two to one (why no flip exists there:
    // `resolve::group_resized`'s docs). The undivided rim edge rides
    // in the offers.
    let rows = fixture::pr4::diagnosis_corpus::<f64>();
    let (_, res) = rows
        .iter()
        .find(|(label, _)| *label == "flip-vanish")
        .expect("the corpus carries the flip-vanish row");
    let f = failure(res);
    let ResolveError::Vanished { name, .. } = &f.error else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    assert!(
        matches!(
            name.path.last(),
            Some(RoleSeg::Fragment(Qualifier::Ends(_)))
        ),
        "the row is about a piece named by its ends: {name:?}"
    );
    // The cutter whose seam vertex with the rim edge is gone is the
    // other operand's cap VERTEX, a point on the edge, not a face.
    let Diagnosis::GroupResized {
        node,
        was: 2,
        now: 1,
        cutters: GroupCutters::Read { gone, new },
    } = vanished(res)
    else {
        panic!("expected a 2 -> 1 resize with its cutters read: {res:?}");
    };
    assert_eq!(*node, name.node);
    assert!(new.is_empty(), "no cutter starts cutting: {new:?}");
    let [cutter] = gone.as_slice() else {
        panic!("one cutter stops cutting: {gone:?}");
    };
    assert_eq!(cutter.kind, EntityKind::Vertex, "{cutter:?}");
    assert_ne!(cutter.node, name.node, "{cutter:?}");
    assert!(
        matches!(
            cutter.path.as_slice(),
            [RoleSeg::CapVertex(
                editor_core::CapEnd::End,
                editor_core::ProfileVertexRef::Piece {
                    role: editor_core::PieceRole::Leg,
                    ..
                },
            )]
        ),
        "an end cap vertex where a leg starts: {cutter:?}"
    );
    assert!(f.offers.contains(&base_of(name)), "{:?}", f.offers);
}

// ---------------------------------------------------------------
// Ladder order, on hand-built runs (the geometry is not the
// subject here — the rung's PLACE is).
// ---------------------------------------------------------------

fn body_ent(i: u32) -> editor_core::EntityRef {
    editor_core::EntityRef {
        body: i,
        key: editor_core::EntityKey::Body,
    }
}

/// The piece of `of` that borders `walls`.
fn frag(node: RecipeNodeId, of: &StableName, walls: Vec<StableName>) -> StableName {
    StableName {
        kind: EntityKind::Body,
        node,
        path: vec![
            RoleSeg::FromA(of.clone().into()),
            RoleSeg::Fragment(Qualifier::Borders(walls)),
        ],
    }
}

/// One-node evaluation carrying `t`, the verdict log `log` and the
/// fragment-group record `groups`.
fn one_node_eval(
    document: editor_core::DocumentId,
    node: RecipeNodeId,
    t: NameTable,
    log: Vec<Verdict>,
    groups: FragmentGroups,
) -> Evaluation<f64> {
    let mut nodes = std::collections::BTreeMap::new();
    nodes.insert(
        node,
        editor_core::NodeResult::Ok(editor_core::NodeValue {
            payload: editor_core::ValuePayload::Declarations(vec![]),
            name_table: Arc::new(t),
            fragment_groups: Arc::new(groups),
            contacts: Arc::new(topo::ContactRecords::default()),
            carried: Arc::new(editor_core::CarriedDeclarations::default()),
            verdicts: Arc::new(log),
            escalations: Arc::new(vec![]),
            placement: None,
            witness: WitnessSlot::default(),
            content_key: ContentKey(0),
            naming_key: NamingKey(0),
        }),
    );
    Evaluation::<f64> {
        epoch: editor_core::Epoch::mint(),
        unplaced: Default::default(),
        document,
        prior_refused: None,
        order: vec![node],
        nodes,
        outcome: EvalOutcome::Completed,
        recomputed: 1,
        reused: 0,
        part_evaluations: 0,
        appearance: editor_core::AppearanceResolution::default(),
    }
}

/// A two-`declare_rest` document and the vanished/base/wall names
/// over its first node. The document is deliberately geometry-free:
/// every row below decides a rung's PLACE, and none of them may depend
/// on a body existing.
struct Hand {
    doc: ProfileDoc,
    node: RecipeNodeId,
    /// The piece bordering wall 0.
    frag: StableName,
    base: StableName,
    /// Two walls, at the second node.
    walls: [StableName; 2],
    /// The names embedded in `frag` — its operand name — and the walls.
    inner: Vec<StableName>,
}

fn hand() -> Hand {
    let (doc, n) = insert(
        ProfileDoc::empty_derived("bool7-hand", Tol::witness()),
        Node::declare_rest(vec![]),
    );
    let (doc, m) = insert(doc, Node::declare_rest(vec![]));
    let of = minted(EntityKind::Body, n, RoleSeg::OutputBody);
    let wall = |rank| StableName {
        kind: EntityKind::Body,
        node: m,
        path: vec![
            RoleSeg::OutputBody,
            RoleSeg::Fragment(Qualifier::OrderAlong { rank, of: 2 }),
        ],
    };
    let walls = [wall(0), wall(1)];
    Hand {
        base: StableName {
            kind: EntityKind::Body,
            node: n,
            path: vec![RoleSeg::FromA(of.clone().into())],
        },
        frag: frag(n, &of, vec![walls[0].clone()]),
        inner: vec![of, walls[0].clone(), walls[1].clone()],
        walls,
        doc,
        node: n,
    }
}

#[test]
fn a_withdrawn_bar_is_answered_by_the_recorded_flip() {
    // The bar moves away along x and no longer meets the plate. Both
    // pieces vanish, and the cut records a containment flip: a flip
    // names a predicate, where the group-size rung (which this edit
    // also satisfies: two pieces became one) names only the effect, so
    // the recorded flip wins. This row goes red if either lower rung is
    // ever raised above the recorded flips.
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = border_pieces(&ev1, s.cut);
    let doc2 = slide(&s, Axis3::X, 5.0);
    let ev2 = run(&doc2, Some(&ev1));
    assert!(border_pieces(&ev2, s.cut).is_empty(), "the cap is whole");
    let res = resolve_with_prior(
        RunCtx {
            doc: &doc2,
            eval: &ev2,
        },
        RunCtx {
            doc: &s.doc,
            eval: &ev1,
        },
        &frags[0],
    );
    assert!(
        matches!(
            vanished(&res),
            Diagnosis::PredicateFlip {
                predicate: "bool_point_in_solid_plane",
                ..
            }
        ),
        "the recorded flip outranks the group-size rung: {:?}",
        vanished(&res)
    );
}

#[test]
fn a_collapsed_borders_group_is_diagnosed_group_resized_and_offers_the_survivor() {
    // The collapse: the bar stops CROSSING the cap (it lands short in
    // y) and no side moves (`resolve::group_resized`'s docs). The
    // cap's group goes from two to one, and the undivided cap is
    // offered for an explicit rebind.
    for to in [2.5_f64, 3.5] {
        let s = slot();
        let ev1 = run(&s.doc, None);
        let frags = border_pieces(&ev1, s.cut);
        let doc2 = slide(&s, Axis3::Y, to);
        let ev2 = run(&doc2, Some(&ev1));
        assert!(border_pieces(&ev2, s.cut).is_empty(), "y = {to}");
        let res = resolve_with_prior(
            RunCtx {
                doc: &doc2,
                eval: &ev2,
            },
            RunCtx {
                doc: &s.doc,
                eval: &ev1,
            },
            &frags[0],
        );
        assert_eq!(
            vanished(&res),
            &Diagnosis::GroupResized {
                node: s.cut,
                was: 2,
                now: 1,
                // The bar's two x walls still cut the cap, and its
                // y = y0 wall, now short of the plate's far edge,
                // crosses it too.
                cutters: GroupCutters::Read {
                    gone: vec![],
                    new: vec![wall(&s.doc, s.bar, 0)],
                },
            },
            "y = {to}"
        );
        let base = base_of(&frags[0]);
        assert!(
            failure(&res).offers.contains(&base),
            "y = {to}: the undivided cap is the offer, got {:?}",
            failure(&res).offers
        );
    }
}

#[test]
fn a_collapsed_edge_piece_group_at_the_cut_is_diagnosed_group_resized() {
    // The same collapse, read off the EDGE pieces the cut
    // mints along the cap's rim: the second, non-flush witness of the
    // `Ends` half, on a subtract rather than the corpus union.
    for to in [2.5_f64, 3.5] {
        let s = slot();
        let ev1 = run(&s.doc, None);
        let pieces: Vec<StableName> = ev1
            .value(s.cut)
            .expect("the cut evaluates")
            .name_table
            .iter()
            .filter_map(|(n, e)| {
                let hit = matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::Ends(_))));
                (hit && matches!(e, Entry::Unique(_))).then(|| n.clone())
            })
            .collect();
        assert!(
            !pieces.is_empty(),
            "the cut names some edge pieces by their ends"
        );
        let doc2 = slide(&s, Axis3::Y, to);
        let ev2 = run(&doc2, Some(&ev1));
        let gone: Vec<&StableName> = pieces
            .iter()
            .filter(|n| {
                ev2.value(s.cut)
                    .expect("the cut evaluates")
                    .name_table
                    .lookup(n)
                    .is_none()
            })
            .collect();
        assert!(!gone.is_empty(), "y = {to}: some edge piece vanishes");
        for name in gone {
            let res = resolve_with_prior(
                RunCtx {
                    doc: &doc2,
                    eval: &ev2,
                },
                RunCtx {
                    doc: &s.doc,
                    eval: &ev1,
                },
                name,
            );
            assert_eq!(
                vanished(&res),
                &Diagnosis::GroupResized {
                    node: s.cut,
                    was: 2,
                    now: 1,
                    // The near rim edge: the bar has left it, both x
                    // walls at once, read in name order.
                    cutters: GroupCutters::Read {
                        gone: {
                            let mut gone = vec![wall(&s.doc, s.bar, 1), wall(&s.doc, s.bar, 3)];
                            gone.sort();
                            gone
                        },
                        new: vec![],
                    },
                },
                "y = {to}: {name:?}"
            );
        }
    }
}

// ---------------------------------------------------------------
// The group-size rung's own boundary, on hand-built runs: when it
// answers, what it counts, and when it declines to the fallback.
// Geometry-free: every row decides a rung's place, and none of them
// may depend on a body existing.
// ---------------------------------------------------------------

/// The fixed cast: a hand document, the vanished piece `frag`
/// (bordering wall 0), and a sibling bordering `walls` of the two.
fn sibling(h: &Hand, walls: &[usize]) -> StableName {
    let mut name = h.base.clone();
    name.path.push(RoleSeg::Fragment(Qualifier::Borders(
        walls.iter().map(|&i| h.walls[i].clone()).collect(),
    )));
    name
}

/// Resolves `name` with the prior table holding `prior` rows and the
/// new table holding `now` rows, each `(name, entities)`, and each
/// run's fragment-group record holding the groups `(base, size)` its
/// run lists; every name embedded in `h.frag` resolves in both runs,
/// so no Cascade.
/// One run for [`group_diagnosis`]: its rows `(name, entities)` and
/// its recorded groups `(base, size)`.
type Run = (Vec<(StableName, usize)>, Vec<(StableName, usize)>);

fn group_diagnosis(
    h: &Hand,
    name: &StableName,
    (prior, prior_groups): Run,
    (now, now_groups): Run,
) -> editor_core::ResolutionFailure {
    let record = FragmentGroups::from_sizes;
    let table = |rows: Vec<(StableName, usize)>| {
        let mut t = NameTable::new();
        let mut next = 0u32;
        for (row, n) in rows {
            // A row's entity has the row's kind (the table refuses a
            // kind disagreement); only the kind-decoy row is a face.
            let kind = row.kind;
            let ents: Vec<_> = (0..n)
                .map(|_| {
                    next += 1;
                    match kind {
                        EntityKind::Face => editor_core::EntityRef {
                            body: next,
                            key: editor_core::EntityKey::Face(topo::FaceKey::default()),
                        },
                        _ => body_ent(next),
                    }
                })
                .collect();
            if n == 1 {
                t.insert(row, ents[0]).unwrap();
            } else {
                t.insert_tied(row, ents).unwrap();
            }
        }
        for (i, inner) in h.inner.iter().enumerate() {
            t.insert(inner.clone(), body_ent(1000 + i as u32)).unwrap();
        }
        t
    };
    let prior_ev = one_node_eval(
        h.doc.id(),
        h.node,
        table(prior),
        vec![],
        record(prior_groups),
    );
    let new_ev = one_node_eval(h.doc.id(), h.node, table(now), vec![], record(now_groups));
    let res = resolve_with_prior(
        RunCtx {
            doc: &h.doc,
            eval: &new_ev,
        },
        RunCtx {
            doc: &h.doc,
            eval: &prior_ev,
        },
        name,
    );
    failure(&res).clone()
}

fn diag(f: &editor_core::ResolutionFailure) -> &Diagnosis {
    let ResolveError::Vanished { diagnosis, .. } = &f.error else {
        panic!("expected Vanished, got {:?}", f.error);
    };
    diagnosis
}

fn fallback(h: &Hand) -> Diagnosis {
    Diagnosis::RecipeEdit {
        edit: editor_core::RecipeEditRef::NodeChanged { node: h.node },
    }
}

/// One group of `size` under `h`'s base: the record an emitter that
/// divided the one parent into those rows keeps.
fn one_group(h: &Hand, size: usize) -> Vec<(StableName, usize)> {
    vec![(h.base.clone(), size)]
}

#[test]
fn a_group_that_stops_being_divided_is_resized_to_one_and_offers_the_base() {
    let h = hand();
    let f = group_diagnosis(
        &h,
        &h.frag,
        (
            vec![(h.frag.clone(), 1), (sibling(&h, &[1]), 1)],
            one_group(&h, 2),
        ),
        (vec![(h.base.clone(), 1)], one_group(&h, 1)),
    );
    assert_eq!(
        diag(&f),
        &Diagnosis::GroupResized {
            node: h.node,
            was: 2,
            now: 1,
            cutters: GroupCutters::NotSeamBounded,
        }
    );
    assert_eq!(f.offers, vec![h.base.clone()]);
}

#[test]
fn a_group_whose_parent_no_longer_descends_is_resized_to_zero() {
    let h = hand();
    let f = group_diagnosis(
        &h,
        &h.frag,
        (
            vec![(h.frag.clone(), 1), (sibling(&h, &[1]), 1)],
            one_group(&h, 2),
        ),
        (vec![], vec![]),
    );
    assert_eq!(
        diag(&f),
        &Diagnosis::GroupResized {
            node: h.node,
            was: 2,
            now: 0,
            cutters: GroupCutters::NotSeamBounded,
        }
    );
    assert!(f.offers.is_empty(), "nothing survives to offer");
}

#[test]
fn the_count_is_the_record_not_the_rows_spelled_from_the_base() {
    // The parent passes through under a name that is not the base (a
    // split that stops dividing a face keeps its upstream name), so no
    // row is spelled from the base; its group still holds it.
    let h = hand();
    let f = group_diagnosis(
        &h,
        &h.frag,
        (
            vec![(h.frag.clone(), 1), (sibling(&h, &[1]), 1)],
            one_group(&h, 2),
        ),
        (vec![], one_group(&h, 1)),
    );
    assert_eq!(
        diag(&f),
        &Diagnosis::GroupResized {
            node: h.node,
            was: 2,
            now: 1,
            cutters: GroupCutters::NotSeamBounded,
        }
    );
}

#[test]
fn a_group_that_grows_is_resized_too_and_a_tie_inside_it_is_several_members() {
    // Prior: the vanished fragment beside a TIED sibling row of two,
    // all three dividing one parent — a group of three entities under
    // two names. Now: two distinct fragments, neither bordering the
    // vanished one's wall (so no border delta) — a group of two.
    let h = hand();
    let f = group_diagnosis(
        &h,
        &h.frag,
        (
            vec![(h.frag.clone(), 1), (sibling(&h, &[1]), 2)],
            one_group(&h, 3),
        ),
        (
            vec![(sibling(&h, &[1]), 1), (sibling(&h, &[]), 1)],
            one_group(&h, 2),
        ),
    );
    assert_eq!(
        diag(&f),
        &Diagnosis::GroupResized {
            node: h.node,
            was: 3,
            now: 2,
            cutters: GroupCutters::NotSeamBounded,
        }
    );
    // And growth: a group of two became three. The new members are
    // spelled with a different qualifier kind on purpose — the count
    // is of the group, whatever qualifies its members, and a `Borders`
    // sibling nearest the vanished piece would be the border delta's.
    let ranked = |rank| {
        let mut n = h.base.clone();
        n.path
            .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of: 3 }));
        (n, 1)
    };
    let f = group_diagnosis(
        &h,
        &h.frag,
        (
            vec![(h.frag.clone(), 1), (sibling(&h, &[1]), 1)],
            one_group(&h, 2),
        ),
        (vec![ranked(0), ranked(1), ranked(2)], one_group(&h, 3)),
    );
    assert_eq!(
        diag(&f),
        &Diagnosis::GroupResized {
            node: h.node,
            was: 2,
            now: 3,
            cutters: GroupCutters::NotSeamBounded,
        }
    );
}

#[test]
fn two_tied_parents_are_counted_one_parent_at_a_time() {
    // Two tied parents share the base, and the tie lane gives their
    // members' rows one set of names: each row is TIED across the two
    // groups. Each parent's group went from two to one; the rows'
    // candidates, four then two, are not a group.
    let h = hand();
    let two = |size| vec![(h.base.clone(), size), (h.base.clone(), size)];
    let f = group_diagnosis(
        &h,
        &h.frag,
        (vec![(h.frag.clone(), 2), (sibling(&h, &[1]), 2)], two(2)),
        (vec![(h.base.clone(), 2)], two(1)),
    );
    assert_eq!(
        diag(&f),
        &Diagnosis::GroupResized {
            node: h.node,
            was: 2,
            now: 1,
            cutters: GroupCutters::NotSeamBounded,
        }
    );
    // Tied parents whose groups no longer agree have no one count.
    let f = group_diagnosis(
        &h,
        &h.frag,
        (vec![(h.frag.clone(), 2), (sibling(&h, &[1]), 2)], two(2)),
        (
            vec![(h.base.clone(), 1)],
            vec![(h.base.clone(), 1), (h.base.clone(), 3)],
        ),
    );
    assert_eq!(diag(&f), &fallback(&h));
}

#[test]
fn a_group_that_requalified_at_the_same_size_is_not_a_resize() {
    // Two fragments before, two after, the vanished one not among
    // them: no resize. The piece still bordering the vanished one's
    // wall now borders another too, and the border delta says so.
    let h = hand();
    let f = group_diagnosis(
        &h,
        &h.frag,
        (
            vec![(h.frag.clone(), 1), (sibling(&h, &[1]), 1)],
            one_group(&h, 2),
        ),
        (
            vec![(sibling(&h, &[0, 1]), 1), (sibling(&h, &[]), 1)],
            one_group(&h, 2),
        ),
    );
    assert_eq!(
        diag(&f),
        &Diagnosis::BorderDelta {
            node: h.node,
            gone: vec![],
            new: vec![h.walls[1].clone()],
        }
    );
}

#[test]
fn a_name_the_prior_run_never_minted_did_not_vanish_by_resizing() {
    // The prior group had two members and the current one has one —
    // but neither was the referenced name, so its group changing size
    // is not why it does not resolve.
    let h = hand();
    let f = group_diagnosis(
        &h,
        &h.frag,
        (
            vec![(sibling(&h, &[1]), 1), (sibling(&h, &[0, 1]), 1)],
            one_group(&h, 2),
        ),
        (vec![(h.base.clone(), 1)], one_group(&h, 1)),
    );
    assert_eq!(diag(&f), &fallback(&h));
}

#[test]
fn a_name_without_a_fragment_tail_never_reaches_the_group_size_rung() {
    // The base itself vanishing: no qualifier, so no group to count,
    // even though the table it was in had company.
    let h = hand();
    let f = group_diagnosis(
        &h,
        &h.base,
        (
            vec![(h.base.clone(), 1), (h.frag.clone(), 1)],
            one_group(&h, 2),
        ),
        (vec![(sibling(&h, &[1]), 1)], one_group(&h, 1)),
    );
    assert_eq!(diag(&f), &fallback(&h));
}

#[test]
fn without_a_prior_run_there_is_no_size_to_change_from() {
    let h = hand();
    let mut t = NameTable::new();
    t.insert(h.base.clone(), body_ent(0)).unwrap();
    for (i, inner) in h.inner.iter().enumerate() {
        t.insert(inner.clone(), body_ent(1000 + i as u32)).unwrap();
    }
    let groups = FragmentGroups::from_sizes([(h.base.clone(), 1)]);
    let ev = one_node_eval(h.doc.id(), h.node, t, vec![], groups);
    let res = editor_core::resolve(
        RunCtx {
            doc: &h.doc,
            eval: &ev,
        },
        &h.frag,
    );
    assert_eq!(vanished(&res), &fallback(&h));
}

#[test]
fn the_group_is_read_by_kind_and_minting_node_not_by_path_alone() {
    // A group recorded under a base whose PATH is the vanished name's
    // base but whose kind or minting node differs is another group:
    // reading it would turn this 2 → 1 into 2 → 2 and silence the
    // rung. One decoy per field, each in its own run.
    let h = hand();
    let other_node = h.inner[1].node;
    assert_ne!(other_node, h.node, "the partner lives at a second node");
    let decoys = [
        StableName {
            kind: EntityKind::Face,
            node: h.base.node,
            path: h.base.path.clone(),
        },
        StableName {
            kind: h.base.kind,
            node: other_node,
            path: h.base.path.clone(),
        },
    ];
    for decoy in decoys {
        let f = group_diagnosis(
            &h,
            &h.frag,
            (
                vec![(h.frag.clone(), 1), (sibling(&h, &[1]), 1)],
                one_group(&h, 2),
            ),
            (
                vec![(h.base.clone(), 1), (decoy.clone(), 1)],
                vec![(h.base.clone(), 1), (decoy.clone(), 2)],
            ),
        );
        assert_eq!(
            diag(&f),
            &Diagnosis::GroupResized {
                node: h.node,
                was: 2,
                now: 1,
                cutters: GroupCutters::NotSeamBounded,
            },
            "decoy {decoy:?} was read"
        );
    }
}
