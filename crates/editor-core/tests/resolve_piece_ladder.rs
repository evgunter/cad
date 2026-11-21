//! The `Vanished` ladder's rungs for a split face's pieces: the border
//! delta (N5's qualifier-delta rung for a face piece), the group-size
//! rung, and the recorded flips above them.
//!
//! A piece is named by the divider walls it borders (`Borders`, N2),
//! so a piece whose walls changed is read off the names: the vanished
//! name's walls against those of the nearest piece its parent is held
//! as now. A group that stopped being divided has no such piece, and
//! the group-size rung states the count; a recorded flip on the path
//! outranks both, because it names a cause. Below every rung that names
//! a cause and above the group size, a node whose two runs part where a
//! query handed on different candidates names the query and the step
//! (`Diagnosis::Parted`): no cause, but where.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, Diagnosis, EntityKind, Entry, EvalOptions, Evaluation, FragmentGroups,
    GroupCutters, NameTable, Node, ProfileDoc, Qualifier, RecipeNodeId, Resolution, ResolveError,
    RoleSeg, RunCtx, StableName, evaluate,
};
use fixture::{insert, len, minted};
use geom_core::k_stats::{Mark, MarkItem, Role, Verdict};
use geom_core::{Sign, Tol};

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

/// A block on its own xy: a construction, the root of its own space.
fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(fixture::desc(vec![vec![
            (x0, y0),
            (x1, y0),
            (x1, y1),
            (x0, y1),
        ]])),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
        },
    )
}

/// The scenario: plate `a` (3×3×1), bar `b` crossing its end cap
/// fully in y, placed against the plate, subtracted at `cut`.
struct Slot {
    doc: ProfileDoc,
    /// The bar's placed copy, which the cut reads.
    bar: crate::resolve_group_membership::Placed,
    cut: RecipeNodeId,
}

impl Slot {
    /// The bar's wall over profile segment `segment` as its placed copy
    /// names it (`block`'s profile runs counter-clockwise from
    /// `(x0, y0)`: wall 0 is y = y0, 1 is x = x1, 3 is x = x0).
    fn wall(&self, segment: u32) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: self.bar.body,
            path: vec![RoleSeg::Lateral(
                crate::fixture::piece(&self.doc, self.bar.body, 0, segment as usize).into(),
            )],
        }
    }

    /// The bar's wall `segment` as the cut's table carries it in,
    /// through the read the cut's tool seat holds: a seam's side (N3).
    fn cutter(&self, segment: u32) -> StableName {
        let Some(Node::Subtract { tool, .. }) = self.doc.node(self.cut) else {
            panic!("the cut is a subtract");
        };
        StableName {
            kind: EntityKind::Face,
            node: self.cut,
            path: vec![RoleSeg::From {
                read: tool.read,
                of: self.wall(segment).into(),
            }],
        }
    }
}

fn slot() -> Slot {
    let doc = ProfileDoc::empty_derived("bool7", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, b0) = block(doc, (1.0, 2.0), (-1.0, 4.0), 1.0);
    let (doc, bar) = crate::resolve_group_membership::placed(doc, a, b0, 0.5);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: a.into(),
            tool: bar.placement.into(),
        },
    );
    Slot { doc, bar, cut }
}

/// Slides the bar along `axis` — the interaction-boundary edit.
fn slide(s: &Slot, axis: usize, to: f64) -> ProfileDoc {
    let mut by = [0.0; 3];
    by[axis] = to;
    crate::resolve_group_membership::moved(s.doc.clone(), &s.bar, by)
}

/// The `Borders`-qualified cap pieces of `cut`'s table, left to right.
fn border_pieces(ev: &Evaluation<f64>, cut: RecipeNodeId) -> Vec<StableName> {
    let fragments = ev
        .table_at(cut)
        .expect("the cut evaluates")
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
fn a_vanished_rim_piece_whose_union_parts_at_its_candidates_names_the_query() {
    // The corpus's own flip-vanish row: the rim piece's group goes
    // from two to one as the slab that cut it slides clear. The
    // union's two runs part where its first box query hands on other
    // candidates, before any choice turns, so no flip is on the record
    // (the slab's wall is still classified against `a`, but after the
    // runs parted), and the parting rung names that query. The
    // undivided rim edge rides in the offers.
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
    assert_eq!(
        vanished(res),
        &Diagnosis::Parted {
            node: name.node,
            site: "bvh_overlapping",
            step: 0,
        },
        "expected the parting rung at the union"
    );
    assert!(f.offers.contains(&base_of(name)), "{:?}", f.offers);
}

/// **A union's slide sweep names where its runs part, at every
/// position.** The bar of [`slot`], joined to the plate rather than cut
/// from it, slides along x from crossing the plate's middle to eight
/// other places: across an edge, flush with one, or clear of the plate,
/// on either side. At each, every name the union minted and no longer holds
/// vanishes, and the union's two runs part where its face-pair query
/// hands on other candidates, before any choice turns, so each name is
/// diagnosed `Parted` at the union by that query.
#[test]
fn a_unions_slide_sweep_names_the_query_its_runs_part_at() {
    let doc = ProfileDoc::empty_derived("bool7-sweep", Tol::witness());
    let (doc, a) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, b0) = block(doc, (1.0, 2.0), (-1.0, 4.0), 1.0);
    let (doc, bar) = crate::resolve_group_membership::placed(doc, a, b0, 0.5);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), bar.placement.into()]),
        },
    );
    let ev1 = run(&doc, None);
    let minted: Vec<StableName> = ev1
        .table_at(union)
        .expect("the union evaluates")
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| n.node == union)
        .collect();
    for to in [-3.0_f64, -2.0, -1.5, -1.0, 1.0, 1.5, 2.0, 3.0] {
        let doc2 = crate::resolve_group_membership::moved(doc.clone(), &bar, [to, 0.0, 0.0]);
        let ev2 = run(&doc2, Some(&ev1));
        let now = ev2.table_at(union).expect("the union evaluates");
        let gone: Vec<&StableName> = minted.iter().filter(|n| now.lookup(n).is_none()).collect();
        assert!(
            !gone.is_empty(),
            "x {to:+}: some name the union minted vanishes"
        );
        for name in gone {
            let res = crate::fixture::resolve_with_prior_at_mint(
                RunCtx {
                    doc: &doc2,
                    eval: &ev2,
                },
                RunCtx {
                    doc: &doc,
                    eval: &ev1,
                },
                name,
            );
            let d = vanished(&res);
            let Diagnosis::Parted { node, site, .. } = d else {
                panic!("x {to:+}: {name:?} is diagnosed {d:?}");
            };
            assert_eq!(*node, union, "x {to:+}: the union's runs part");
            assert_eq!(*site, "bvh_overlapping", "x {to:+}: at its face-pair query");
        }
    }
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

/// The piece of `of`, read through `read`, that borders `walls`.
fn frag(
    node: RecipeNodeId,
    read: editor_core::VarId,
    of: &StableName,
    walls: Vec<StableName>,
) -> StableName {
    StableName {
        kind: EntityKind::Body,
        node,
        path: vec![
            RoleSeg::From {
                read,
                of: of.clone().into(),
            },
            RoleSeg::Fragment(Qualifier::Borders(walls)),
        ],
    }
}

/// One node's decision log in one run: its verdicts and its candidate
/// marks.
type Log = (Vec<Verdict>, Vec<Mark>);

/// Each node's [`Log`] in one run.
type Logs = Vec<(RecipeNodeId, Log)>;

/// An evaluation whose `node` carries `t` and the fragment-group record
/// `groups`, and whose every node in `logs` carries that log (a node
/// other than `node` with an empty table).
fn hand_eval(
    document: editor_core::DocumentId,
    node: RecipeNodeId,
    t: NameTable,
    logs: Logs,
    groups: FragmentGroups,
) -> Evaluation<f64> {
    let mut nodes = std::collections::BTreeMap::new();
    nodes.insert(
        node,
        editor_core::NodeResult::Ok(editor_core::NodeValue::hand_built(t, groups, Vec::new())),
    );
    for (at, (verdicts, marks)) in logs {
        let entry = nodes.entry(at).or_insert_with(|| {
            editor_core::NodeResult::Ok(editor_core::NodeValue::hand_built(
                NameTable::new(),
                FragmentGroups::new(),
                Vec::new(),
            ))
        });
        let editor_core::NodeResult::Ok(value) = entry else {
            unreachable!("every hand-built node is Ok")
        };
        value.verdicts = std::sync::Arc::new(verdicts);
        value.marks = std::sync::Arc::new(marks);
    }
    let order = nodes.keys().copied().collect();
    Evaluation::<f64>::hand_built(document, order, nodes, Tol::witness())
}

/// A two-frame document and the vanished/base/wall names
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
        Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 0.5)])),
    );
    let (doc, m) = insert(
        doc,
        Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 0.5)])),
    );
    let of = minted(EntityKind::Body, n, RoleSeg::OutputBody);
    let wall = |k: u64| StableName {
        kind: EntityKind::Body,
        node: m,
        path: vec![
            RoleSeg::OutputBody,
            RoleSeg::Fragment(Qualifier::Ends(vec![
                minted(
                    EntityKind::Body,
                    m,
                    RoleSeg::OutputBody
                );
                usize::try_from(k).expect("small") + 1
            ])),
        ],
    };
    let walls = [wall(0), wall(1)];
    // The names are hand-built over the first node, read through its
    // own output.
    let read = fixture::out(&doc, n);
    Hand {
        base: StableName {
            kind: EntityKind::Body,
            node: n,
            path: vec![RoleSeg::From {
                read,
                of: of.clone().into(),
            }],
        },
        frag: frag(n, read, &of, vec![walls[0].clone()]),
        inner: vec![of, walls[0].clone(), walls[1].clone()],
        walls,
        doc,
        node: n,
    }
}

#[test]
fn a_withdrawn_bar_names_where_its_cuts_runs_part() {
    // The bar moves away along x and no longer meets the plate. Both
    // pieces vanish. The cut's two runs part at the candidates its box
    // queries hand on (the withdrawn bar's faces meet none of the
    // plate's), before any choice turns: the containment readings that
    // follow are about other pairs, so no flip is recorded, and the
    // parting rung names the query, above the group-size rung (two
    // pieces became one). The hand-built rows below hold the order.
    let s = slot();
    let ev1 = run(&s.doc, None);
    let frags = border_pieces(&ev1, s.cut);
    let doc2 = slide(&s, 0, 5.0);
    let ev2 = run(&doc2, Some(&ev1));
    assert!(border_pieces(&ev2, s.cut).is_empty(), "the cap is whole");
    let res = crate::fixture::resolve_with_prior_at_mint(
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
            Diagnosis::Parted { node, site: "bvh_overlapping", .. } if *node == s.cut
        ),
        "no turn on the record, so the parting rung at the cut: {:?}",
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
        let doc2 = slide(&s, 1, to);
        // Read silent: the cut's runs part at a query's candidates,
        // and the parting rung would answer above this one.
        let ev2 = fixture::silent(run(&doc2, Some(&ev1)));
        let ev1 = fixture::silent(ev1);
        assert!(border_pieces(&ev2, s.cut).is_empty(), "y = {to}");
        let res = crate::fixture::resolve_with_prior_at_mint(
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
                    new: vec![s.cutter(0)],
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
        let Some(Node::Subtract { from: plate, .. }) = s.doc.node(s.cut) else {
            panic!("the cut is a subtract")
        };
        let plate = plate.read;
        let pieces: Vec<StableName> = ev1
            .table_at(s.cut)
            .expect("the cut evaluates")
            .iter()
            .filter_map(|(n, e)| {
                // The plate's rim pieces: the bar's own edges are lone
                // pieces whose ends the slide moves.
                let hit = matches!(
                    n.path.first(),
                    Some(RoleSeg::From { read, .. }) if *read == plate
                ) && matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::Ends(_))));
                (hit && matches!(e, Entry::Unique(_))).then(|| n.clone())
            })
            .collect();
        assert!(
            !pieces.is_empty(),
            "the cut names some edge pieces by their ends"
        );
        let doc2 = slide(&s, 1, to);
        // Read silent, as the row above.
        let ev2 = fixture::silent(run(&doc2, Some(&ev1)));
        let ev1 = fixture::silent(ev1);
        let gone: Vec<&StableName> = pieces
            .iter()
            .filter(|n| {
                ev2.table_at(s.cut)
                    .expect("the cut evaluates")
                    .lookup(n)
                    .is_none()
            })
            .collect();
        assert!(!gone.is_empty(), "y = {to}: some edge piece vanishes");
        for name in gone {
            let res = crate::fixture::resolve_with_prior_at_mint(
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
                            let mut gone = vec![s.cutter(1), s.cutter(3)];
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
    prior: Run,
    now: Run,
) -> editor_core::ResolutionFailure {
    logged_group_diagnosis(h, name, (prior, vec![]), (now, vec![]))
}

/// [`group_diagnosis`] with each run's decision logs, by node.
fn logged_group_diagnosis(
    h: &Hand,
    name: &StableName,
    ((prior, prior_groups), prior_log): (Run, Logs),
    ((now, now_groups), now_log): (Run, Logs),
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
    let prior_ev = hand_eval(
        h.doc.id(),
        h.node,
        table(prior),
        prior_log,
        record(prior_groups),
    );
    let new_ev = hand_eval(h.doc.id(), h.node, table(now), now_log, record(now_groups));
    let res = crate::fixture::resolve_with_prior_at_mint(
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

/// **A recorded turn outranks the group-size rung**: the node's group
/// stops being divided AND its choices turn on one predicate, so the
/// flip, which names a decision, answers ahead of the group size, which
/// names only the effect. This row goes red if either lower rung is
/// ever raised above the recorded flips.
#[test]
fn a_recorded_turn_outranks_the_group_size_rung() {
    let h = hand();
    let turn = |sign| Verdict {
        predicate: "bool_point_in_solid_plane",
        sign,
        role: geom_core::k_stats::Role::Choice,
    };
    let f = logged_group_diagnosis(
        &h,
        &h.frag,
        (
            (
                vec![(h.frag.clone(), 1), (sibling(&h, &[1]), 1)],
                one_group(&h, 2),
            ),
            vec![(h.node, (vec![turn(Sign::Negative)], vec![]))],
        ),
        (
            (vec![(h.base.clone(), 1)], one_group(&h, 1)),
            vec![(h.node, (vec![turn(Sign::Positive)], vec![]))],
        ),
    );
    assert_eq!(
        diag(&f),
        &Diagnosis::PredicateFlip {
            predicate: "bool_point_in_solid_plane",
            from: Sign::Negative,
            to: Sign::Positive,
        }
    );
}

/// A choice of `predicate` decided `sign`.
fn choice(predicate: &'static str, sign: Sign) -> Verdict {
    Verdict {
        predicate,
        sign,
        role: Role::Choice,
    }
}

/// A mark of `site` placed before verdict `before`, feeding `role`.
fn mark(site: &'static str, before: usize, item: MarkItem, role: Role) -> Mark {
    Mark {
        before,
        site,
        item,
        role,
    }
}

/// [`logged_group_diagnosis`] over the group that stops being divided
/// (2 → 1) at `h.node`, with each run's decision logs.
fn parting_diagnosis(h: &Hand, prior: Logs, now: Logs) -> Diagnosis {
    let f = logged_group_diagnosis(
        h,
        &h.frag,
        (
            (
                vec![(h.frag.clone(), 1), (sibling(h, &[1]), 1)],
                one_group(h, 2),
            ),
            prior,
        ),
        ((vec![(h.base.clone(), 1)], one_group(h, 1)), now),
    );
    diag(&f).clone()
}

/// **A parting at a mark outranks the group-size rung**: the node's
/// group stops being divided AND its runs part where one query handed
/// on different candidates, before any choice turns. The parting names
/// the query and the step, the index along the choices and candidates
/// both runs walked alike; the group size names only the effect.
#[test]
fn a_parting_at_a_mark_outranks_the_group_size_rung() {
    let h = hand();
    let c = |sign| choice("bool_point_in_solid_plane", sign);
    let cand = |i| MarkItem::Candidate(i);
    // A query that handed on candidate 0 then 1, against one that
    // handed on 0 alone: the runs part at its second step.
    let d = parting_diagnosis(
        &h,
        vec![(
            h.node,
            (
                vec![c(Sign::Positive), c(Sign::Negative)],
                vec![
                    mark("bvh_overlapping", 0, cand(0), Role::Choice),
                    mark("bvh_overlapping", 0, cand(1), Role::Choice),
                    mark("bvh_overlapping", 0, MarkItem::End, Role::Choice),
                ],
            ),
        )],
        vec![(
            h.node,
            (
                vec![c(Sign::Positive), c(Sign::Positive)],
                vec![
                    mark("bvh_overlapping", 0, cand(0), Role::Choice),
                    mark("bvh_overlapping", 0, MarkItem::End, Role::Choice),
                ],
            ),
        )],
    );
    assert_eq!(
        d,
        Diagnosis::Parted {
            node: h.node,
            site: "bvh_overlapping",
            step: 1,
        },
        "a query's candidates part the runs"
    );
    // A marked test that answered otherwise, after one identical
    // choice: the runs part at the test, and the turn after it, about
    // what the test let through, is not read.
    let d = parting_diagnosis(
        &h,
        vec![(
            h.node,
            (
                vec![c(Sign::Positive), c(Sign::Negative)],
                vec![mark(
                    "blend_reach_sure",
                    1,
                    MarkItem::Outcome(true),
                    Role::Choice,
                )],
            ),
        )],
        vec![(
            h.node,
            (
                vec![c(Sign::Positive), c(Sign::Positive)],
                vec![mark(
                    "blend_reach_sure",
                    1,
                    MarkItem::Outcome(false),
                    Role::Choice,
                )],
            ),
        )],
    );
    assert_eq!(
        d,
        Diagnosis::Parted {
            node: h.node,
            site: "blend_reach_sure",
            step: 1,
        },
        "a marked test's answer parts the runs"
    );
}

/// **A recorded turn outranks a parting**: one node on the path parts
/// at a mark, and another turns. The turn names a decision, so it
/// answers; the parting would name only where one node's runs stopped
/// being comparable.
#[test]
fn a_recorded_turn_outranks_a_parting() {
    let h = hand();
    // The walls' node, on the path the vanished piece's name spells.
    let other = h.walls[0].node;
    assert_ne!(other, h.node);
    let c = |sign| choice("bool_point_in_solid_plane", sign);
    let parts = |item| {
        (
            h.node,
            (
                vec![c(Sign::Positive)],
                vec![mark("bvh_overlapping", 0, item, Role::Choice)],
            ),
        )
    };
    let turns = |sign| (other, (vec![c(sign)], vec![]));
    let d = parting_diagnosis(
        &h,
        vec![parts(MarkItem::Candidate(0)), turns(Sign::Negative)],
        vec![parts(MarkItem::End), turns(Sign::Positive)],
    );
    assert_eq!(
        d,
        Diagnosis::PredicateFlip {
            predicate: "bool_point_in_solid_plane",
            from: Sign::Negative,
            to: Sign::Positive,
        }
    );
}

/// **A parting at no mark names nothing**, and the group-size rung
/// answers (N5: a node whose choices part any other way names no
/// flip, and only a query's candidates name where). Each case parts
/// the node's two runs, at its first step, some other way.
#[test]
fn a_parting_at_no_mark_falls_to_the_group_size_rung() {
    let h = hand();
    let resized = Diagnosis::GroupResized {
        node: h.node,
        was: 2,
        now: 1,
        cutters: GroupCutters::NotSeamBounded,
    };
    let c = |predicate| choice(predicate, Sign::Positive);
    let at = |verdicts: Vec<Verdict>, marks: Vec<Mark>| vec![(h.node, (verdicts, marks))];
    let one = |site, item| vec![mark(site, 0, item, Role::Choice)];
    let cases: [(&str, Logs, Logs); 5] = [
        (
            "another predicate",
            at(vec![c("bool_point_in_solid_plane")], vec![]),
            at(vec![c("plane_eq_parallel")], vec![]),
        ),
        (
            "one log ending",
            at(vec![c("bool_point_in_solid_plane")], vec![]),
            at(vec![], vec![]),
        ),
        (
            "a choice against a mark",
            at(vec![c("bool_point_in_solid_plane")], vec![]),
            at(
                vec![c("bool_point_in_solid_plane")],
                one("bvh_overlapping", MarkItem::End),
            ),
        ),
        (
            "two queries' marks",
            at(vec![], one("bvh_overlapping", MarkItem::End)),
            at(vec![], one("bvh_within", MarkItem::End)),
        ),
        (
            "a query's candidates feeding only checks",
            at(
                vec![],
                vec![mark(
                    "bvh_overlapping",
                    0,
                    MarkItem::Candidate(0),
                    Role::Check,
                )],
            ),
            at(
                vec![],
                vec![mark("bvh_overlapping", 0, MarkItem::End, Role::Check)],
            ),
        ),
    ];
    for (case, prior, now) in cases {
        assert_eq!(parting_diagnosis(&h, prior, now), resized, "{case}");
    }
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
    let ranked = |k: u64| {
        let mut n = h.base.clone();
        n.path.push(RoleSeg::Fragment(Qualifier::Ends(vec![
            minted(
                EntityKind::Body,
                h.node,
                RoleSeg::OutputBody
            );
            usize::try_from(k)
                .expect("small")
                + 1
        ])));
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
    let ev = hand_eval(h.doc.id(), h.node, t, vec![], groups);
    let res = crate::fixture::resolve_at_mint(
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
