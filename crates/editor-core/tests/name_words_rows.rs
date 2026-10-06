//! **What a row says about a name it cannot say by its words alone**,
//! and **the full form's and a table's words, over names built to
//! collide** (`work/recipe/names-render-a-faces-leaf-role-in-words.md`).
//!
//! A name's words say the node that made its leaf, not the node that
//! minted it, and not which slot of a payload holds it: a strand row
//! says the node the edit deleted, and a resolve row the slot that
//! failed, a reference being (site, name). The full form brackets every
//! cited name that runs on past its node, so it reads one way only, and
//! a table's details read its names apart however they cite each other.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/names/words.rs",
    "crates/editor-core/src/names/role.rs",
    "crates/editor-core/src/names/table.rs",
    "crates/editor-core/src/spoken.rs",
    "crates/editor-core/tests/corpus/",
    "crates/editor-core/tests/fixture/",
];

use std::collections::BTreeMap;
use std::time::Instant;

use crate::corpus::Recorder;
use crate::fixture::{self, len, scl};
use editor_core::{
    BooleanCoincidence, BooleanOp, CapEnd, Datum, EntityKey, EntityKind, EntityRef, EvalOptions,
    ExtrudeSide, MeasureExpr, MeasurePrimitive, NameRef, NameTable, NameTables, Node, NodeResult,
    PieceRole, ProfileEdgeRef, Qualifier, RecipeNodeId, RoleSeg, SitedRef, Speaker, StableName,
    StepId,
};
use test_utils::fuzz;

const EXTRUDE: RecipeNodeId = RecipeNodeId(1 << 16);
const OTHER: RecipeNodeId = RecipeNodeId(2 << 16);
const OP: RecipeNodeId = RecipeNodeId(3 << 16);

fn wall(step: u64) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: EXTRUDE,
        path: vec![RoleSeg::Lateral(
            ProfileEdgeRef::Piece {
                step: StepId(step << 16),
                role: PieceRole::Leg,
            }
            .into(),
        )],
    }
}

fn merged_of(node: RecipeNodeId, members: Vec<StableName>) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Merged(members)],
    }
}

fn end_cap() -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: EXTRUDE,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    }
}

/// `name`'s part bordering `walls`, minted where `name` is.
fn bordering(name: StableName, walls: Vec<StableName>) -> StableName {
    StableName {
        kind: name.kind,
        node: name.node,
        path: [
            name.path.clone(),
            vec![RoleSeg::Fragment(Qualifier::Borders(walls))],
        ]
        .concat(),
    }
}

/// The one table [`OP`]'s output holds.
struct One(NameTable);

impl NameTables for One {
    fn table(&self, node: RecipeNodeId) -> Option<&NameTable> {
        (node == OP).then_some(&self.0)
    }
}

fn table(names: &[StableName]) -> One {
    let mut table = NameTable::new();
    for (i, name) in (1u64..).zip(names) {
        let face = EntityRef {
            body: 0,
            key: EntityKey::Face(topo::FaceKey::from(slotmap::KeyData::from_ffi(i))),
        };
        table.insert(name.clone(), face).expect("each name once");
    }
    One(table)
}

/// The words said alike by more than one name, and how many say each.
fn alike(said: &[String]) -> Vec<(&str, usize)> {
    let mut groups: BTreeMap<&str, usize> = BTreeMap::new();
    for s in said {
        *groups.entry(s).or_default() += 1;
    }
    groups.into_iter().filter(|(_, n)| *n > 1).collect()
}

/// A random cited name, nested to `depth`: a wall of `pool`, a merged
/// face of cited names, a bordering part of a wall, a seam of two, or a
/// carry through a Boolean's B.
fn random_cited(rng: &mut fuzz::Rng, depth: u32, pool: usize) -> StableName {
    let wall_of = |rng: &mut fuzz::Rng| wall(1 + rng.below(pool) as u64);
    let pick = if depth == 0 {
        rng.below(2) * 4
    } else {
        rng.below(6)
    };
    match pick {
        0 => wall_of(rng),
        1 => {
            let n = 1 + rng.below(3);
            merged_of(
                OTHER,
                (0..n).map(|_| random_cited(rng, depth - 1, pool)).collect(),
            )
        }
        2 => {
            let n = 1 + rng.below(3);
            let base = wall_of(rng);
            bordering(
                base,
                (0..n).map(|_| random_cited(rng, depth - 1, pool)).collect(),
            )
        }
        3 => StableName {
            kind: EntityKind::Edge,
            node: OTHER,
            path: vec![RoleSeg::Seam {
                a: NameRef::new(random_cited(rng, depth - 1, pool)),
                b: NameRef::new(random_cited(rng, depth - 1, pool)),
            }],
        },
        4 => StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId((5 + rng.below(2) as u64) << 16),
            path: vec![RoleSeg::FromB(NameRef::new(wall_of(rng)))],
        },
        _ => StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(7 << 16),
            path: vec![RoleSeg::FromB(NameRef::new(random_cited(
                rng,
                depth - 1,
                pool,
            )))],
        },
    }
}

/// A random name [`OP`] mints: a merged face of cited names, or the
/// part of the extrude's end cap bordering them, carried through OP.
fn random_minted(rng: &mut fuzz::Rng, depth: u32, pool: usize) -> StableName {
    let n = 1 + rng.below(4);
    let members: Vec<StableName> = (0..n).map(|_| random_cited(rng, depth, pool)).collect();
    if rng.below(4) == 0 {
        StableName {
            kind: EntityKind::Face,
            node: OP,
            path: vec![
                RoleSeg::FromA(NameRef::new(end_cap())),
                RoleSeg::Fragment(Qualifier::Borders(members)),
            ],
        }
    } else {
        merged_of(OP, members)
    }
}

/// **The two shapes an unbracketed list leaves open read one way
/// each**: a list whose last member ends in a list of its own (its
/// "and" could attach to either), and a join said after a list (it
/// could belong to the last member or to the name).
#[test]
fn a_list_ending_in_a_list_or_a_join_reads_one_way() {
    let cap = StableName {
        kind: EntityKind::Face,
        node: EXTRUDE,
        path: vec![RoleSeg::Cap(CapEnd::Start)],
    };
    let nested = bordering(
        cap.clone(),
        vec![bordering(wall(2), vec![wall(3), wall(4)])],
    );
    let beside = bordering(
        cap.clone(),
        vec![bordering(wall(2), vec![wall(3)]), wall(4)],
    );
    assert_ne!(nested.to_string(), beside.to_string(), "{nested}");

    let through_b = |name: StableName| StableName {
        kind: name.kind,
        node: OTHER,
        path: vec![RoleSeg::FromB(NameRef::new(name))],
    };
    let member_joined = bordering(cap.clone(), vec![through_b(wall(3))]);
    let name_joined = through_b(bordering(cap, vec![wall(3)]));
    assert_ne!(
        member_joined.to_string(),
        name_joined.to_string(),
        "{member_joined}"
    );
}

/// **Two distinct names never read alike in full**: the full form
/// brackets every cited name whose words run on past its node, so a
/// list's "and" and a join said after a citation read one way only.
/// Names nested three deep, citing lists of lists and joins.
#[test]
fn the_full_form_says_no_two_names_alike() {
    let mut rng = fuzz::start("the_full_form_says_no_two_names_alike");
    let mut seen: BTreeMap<String, StableName> = BTreeMap::new();
    let mut collided = Vec::new();
    for _ in 0..fuzz::scaled(4000) {
        let name = random_minted(&mut rng, 3, 4);
        let said = name.to_string();
        match seen.get(&said) {
            Some(prev) if *prev != name => collided.push((said, prev.clone(), name)),
            Some(_) => {}
            None => {
                seen.insert(said, name);
            }
        }
    }
    assert!(
        collided.is_empty(),
        "{} pairs of distinct names read alike in full, the first: {:#?} — {}",
        collided.len(),
        collided.first(),
        fuzz::replay()
    );
}

/// **The words in a table never say two of its names alike**, each at
/// the detail the table gives it, over random tables of names citing
/// lists, nested lists, seams and joins.
#[test]
fn a_tables_words_say_no_two_of_its_names_alike() {
    let mut rng = fuzz::start("a_tables_words_say_no_two_of_its_names_alike");
    let mut checked = 0usize;
    for _ in 0..fuzz::scaled(400) {
        let size = 2 + rng.below(9);
        let pool = 2 + rng.below(5);
        let depth = rng.below(3) as u32;
        let mut names: Vec<StableName> = Vec::new();
        for _ in 0..size * 3 {
            let name = random_minted(&mut rng, depth, pool);
            if !names.contains(&name) {
                names.push(name);
            }
            if names.len() == size {
                break;
            }
        }
        let tables = table(&names);
        let by = Speaker::TAG.within(&tables);
        let said: Vec<String> = names.iter().map(|n| by.name(n).to_string()).collect();
        assert!(
            alike(&said).is_empty(),
            "a table says two of its names alike: {:#?} — {}",
            alike(&said),
            fuzz::replay()
        );
        checked += 1;
    }
    println!("{checked} tables read apart");
}

/// **The first saying within a table whose names all read alike at no
/// citation is not quadratic in renderings**: a thousand merged faces
/// of three walls each, said within their table, each read apart.
#[test]
fn a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time() {
    const NAMES: usize = 1000;
    let mut rng = fuzz::start("a_large_table_of_names_alike_at_no_citation");
    let mut names = Vec::new();
    while names.len() < NAMES {
        let mut steps: Vec<u64> = (0..3).map(|_| 1 + rng.below(40) as u64).collect();
        steps.sort_unstable();
        steps.dedup();
        let name = merged_of(OP, steps.into_iter().map(wall).collect());
        if !names.contains(&name) {
            names.push(name);
        }
    }
    let tables = table(&names);
    let by = Speaker::TAG.within(&tables);
    let start = Instant::now();
    let first = by.name(&names[0]).to_string();
    let first_took = start.elapsed();
    let said: Vec<String> = names.iter().map(|n| by.name(n).to_string()).collect();
    println!("{NAMES} names alike at no citation: the first saying {first_took:?} ({first})");
    assert!(
        alike(&said).is_empty(),
        "names said alike: {:#?} — {}",
        alike(&said),
        fuzz::replay()
    );
    // The search before this row's fix took 15 s at this size in a
    // release build; each name said once per detail its group's
    // searches try is well under one.
    assert!(
        first_took.as_secs() < 10,
        "the first saying of {NAMES} names took {first_took:?} — {}",
        fuzz::replay()
    );
}

/// A block, and a split of it whose plane cuts its end cap: the cap's
/// name holds at the block and not at the split.
fn block_and_split(r: &mut Recorder) -> (RecipeNodeId, RecipeNodeId) {
    let p = r.profile(
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, -1.0), (4.0, -1.0), (4.0, 3.0), (0.0, 3.0)]],
    );
    let block = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
        side: ExtrudeSide::Along,
    });
    let tool = r.insert(Node::Datum(Datum::Plane {
        origin: [len(2.0), len(0.0), len(0.0)],
        normal: [scl(1.0), scl(0.0), scl(0.0)],
    }));
    let split = r.insert(Node::Split {
        target: block,
        tool,
    });
    (block, split)
}

/// The failure `node` reports, said from `doc`.
fn failure_of(r: &Recorder, node: RecipeNodeId) -> String {
    let ev = fixture::run(&r.doc, &EvalOptions::default());
    match ev.nodes.iter().find(|(id, _)| **id == node).map(|(_, r)| r) {
        Some(NodeResult::Failed(e)) => e.spoken(&r.doc, &ev),
        other => panic!("{node:?} fails: {other:?}"),
    }
}

/// **A resolve row names the reference that failed, not the first that
/// holds its name**: a reference is (site, name), and two references
/// can hold one name at two sites — here the block's end cap at the
/// block, which resolves, and at a split that cuts it, which does not.
/// A measure's references, and a Boolean's declared pair.
#[test]
fn a_resolve_row_names_the_slot_that_failed() {
    let mut r = Recorder::new();
    let (block, split) = block_and_split(&mut r);
    let cap = fixture::fname(block, RoleSeg::Cap(CapEnd::End));
    let measure = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![
                SitedRef::new(block, cap.clone()),
                SitedRef::new(split, cap.clone()),
            ],
        )
        .expect("both indices in range"),
    );
    let said = failure_of(&r, measure);
    assert!(
        said.contains("this measure's reference 1 "),
        "the measure's row names reference 1, the one at the split: {said}"
    );

    let mut r = Recorder::new();
    let (block, split) = block_and_split(&mut r);
    let cap = fixture::fname(block, RoleSeg::Cap(CapEnd::End));
    let union = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: block,
        b: split,
        declare: vec![(
            (
                SitedRef::new(block, cap.clone()),
                SitedRef::new(split, cap.clone()),
            ),
            BooleanCoincidence::REST,
        )],
    });
    let said = failure_of(&r, union);
    assert!(
        said.contains("this boolean's declared pair 0's second face "),
        "the Boolean's row names the pair's second side, the one at the split: {said}"
    );
}

/// **A strand row names the node the edit deleted**: a face painted at
/// a Subtract's output, carried through its A — so its words say the
/// block's end cap — and then the Subtract deleted. The row says the
/// Subtract, which minted the name, not only the block its words say.
#[test]
fn a_strand_row_names_the_node_the_delete_took() {
    use editor_core::{Attr, DocEdit, RefusingReach, Rgba8, apply};
    let mut r = Recorder::new();
    let (xy, uv) = ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let p = r.profile(
        [0.0; 3],
        xy,
        uv,
        vec![vec![(0.0, -1.0), (4.0, -1.0), (4.0, 3.0), (0.0, 3.0)]],
    );
    let block = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
        side: ExtrudeSide::Along,
    });
    let q = r.profile(
        [0.0, 0.0, -0.5],
        xy,
        uv,
        vec![vec![(1.0, 0.0), (1.5, 0.0), (1.5, 0.5), (1.0, 0.5)]],
    );
    let pin = r.insert(Node::Extrude {
        profile: q,
        distance: len(1.0),
        side: ExtrudeSide::Along,
    });
    let cut = r.insert(Node::Boolean {
        op: BooleanOp::Subtract,
        a: block,
        b: pin,
        declare: Vec::new(),
    });
    let doc = r.doc;
    let ev = fixture::run(&doc, &EvalOptions::default());
    let painted = ev
        .value(cut)
        .expect("the cut evaluates")
        .name_table
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| n.kind == EntityKind::Face && matches!(n.path.as_slice(), [RoleSeg::FromA(_)]))
        .expect("a face carried through A");
    let tol = fixture::tol();
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
            name: painted,
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        tol,
        &RefusingReach,
    )
    .expect("paint")
    .doc;
    let applied = apply(&doc, &DocEdit::DeleteNode { id: cut }, tol, &RefusingReach)
        .expect("the delete applies");
    let rows: Vec<String> = applied.maintenance.iter().map(|m| m.to_string()).collect();
    let deleted = format!(
        "this edit deleted {}, which minted the name",
        doc.spoken(cut)
    );
    assert!(
        !rows.is_empty() && rows.iter().all(|row| row.contains(&deleted)),
        "every strand row names the deleted Subtract ({deleted}): {rows:#?}"
    );
}
