//! **What a row says about a name it cannot say by its words alone**,
//! and **the full form's and a table's words, over names built to
//! collide** (`work/recipe/names-render-a-faces-leaf-role-in-words.md`).
//!
//! A name's words say the node that made its leaf, not the node that
//! minted it, and not which slot of a payload holds it: a strand row
//! says the node the edit deleted, and a resolve row the slot that
//! failed, a reference being (site, name). The full form brackets every
//! cited name that runs on past its node and says wraps and joins in the
//! order the path takes them, so two names read alike in full only where
//! they differ in a carry's node it never says; a table's details read
//! its names apart however they cite each other.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/names/words.rs",
    "crates/editor-core/src/names/role.rs",
    "crates/editor-core/src/names/table.rs",
    "crates/editor-core/src/names/attribute.rs",
    "crates/editor-core/src/spoken.rs",
    "crates/editor-core/src/eval/wire.rs",
    "crates/editor-core/src/eval/mod.rs",
    "crates/editor-core/src/resolve/",
    "crates/editor-core/src/node.rs",
    "crates/editor-core/src/edit.rs",
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
    PieceRole, ProfileEdgeRef, Qualifier, RecipeNodeId, RoleSeg, SitedRef, Speaker, SplitHalf,
    StableName, StepId,
};
use test_utils::fuzz;

const EXTRUDE: RecipeNodeId = RecipeNodeId::new(0, 1 << 16);
const OTHER: RecipeNodeId = RecipeNodeId::new(0, 2 << 16);
const OP: RecipeNodeId = RecipeNodeId::new(0, 3 << 16);

fn wall(step: u64) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: EXTRUDE,
        path: vec![RoleSeg::Lateral(
            ProfileEdgeRef::Piece {
                step: StepId::new(0, step << 16),
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
        let key = slotmap::KeyData::from_ffi(i);
        let entity = EntityRef {
            body: 0,
            key: match name.kind {
                EntityKind::Edge => EntityKey::Edge(topo::EdgeKey::from(key)),
                _ => EntityKey::Face(topo::FaceKey::from(key)),
            },
        };
        table.insert(name.clone(), entity).expect("each name once");
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
            node: RecipeNodeId::new(0, (5 + rng.below(2) as u64) << 16),
            path: vec![RoleSeg::FromB(NameRef::new(wall_of(rng)))],
        },
        _ => StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, 7 << 16),
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

/// A random name nested to `depth` over every carry the words say
/// around a leaf or as a join, and every list-bearing role: walls, caps
/// and rim edges of the extrude; merged faces, seams, blend faces and
/// band faces; a part bordering, along or between a list, or ranked; a
/// split's half and a pattern's copy; a carry through a Boolean's B or
/// a union's member — each at one of three nodes.
fn random_rich(rng: &mut fuzz::Rng, depth: u32, pool: usize) -> StableName {
    fn leaf(rng: &mut fuzz::Rng, pool: usize) -> StableName {
        let step = 1 + rng.below(pool) as u64;
        match rng.below(3) {
            0 => wall(step),
            1 => StableName {
                kind: EntityKind::Face,
                node: EXTRUDE,
                path: vec![RoleSeg::Cap(if rng.below(2) == 0 {
                    CapEnd::Start
                } else {
                    CapEnd::End
                })],
            },
            _ => StableName {
                kind: EntityKind::Edge,
                node: EXTRUDE,
                path: vec![RoleSeg::RimEdge(
                    CapEnd::End,
                    ProfileEdgeRef::Piece {
                        step: StepId::new(0, step << 16),
                        role: PieceRole::Leg,
                    }
                    .into(),
                )],
            },
        }
    }
    if depth == 0 {
        return leaf(rng, pool);
    }
    let list = |rng: &mut fuzz::Rng| -> Vec<StableName> {
        let n = 1 + rng.below(3);
        (0..n).map(|_| random_rich(rng, depth - 1, pool)).collect()
    };
    let inner = |rng: &mut fuzz::Rng| NameRef::new(random_rich(rng, depth - 1, pool));
    let node = |rng: &mut fuzz::Rng| RecipeNodeId::new(0, (8 + rng.below(3) as u64) << 16);
    let half = |rng: &mut fuzz::Rng| {
        if rng.below(2) == 0 {
            SplitHalf::Above
        } else {
            SplitHalf::Below
        }
    };
    let face = |node: RecipeNodeId, seg: RoleSeg| StableName {
        kind: EntityKind::Face,
        node,
        path: vec![seg],
    };
    match rng.below(13) {
        0 | 1 => leaf(rng, pool),
        2 => merged_of(node(rng), list(rng)),
        3..=5 => {
            let base = random_rich(rng, depth - 1, pool);
            let q = match rng.below(4) {
                0 => Qualifier::Borders(list(rng)),
                1 => Qualifier::Keeps(list(rng)),
                2 => Qualifier::Ends(list(rng)),
                _ => Qualifier::OrderAlong {
                    rank: rng.below(3) as u32,
                    of: 3,
                },
            };
            // A qualifier on a carried name, as an operation mints it.
            StableName {
                kind: base.kind,
                node: node(rng),
                path: vec![RoleSeg::FromA(NameRef::new(base)), RoleSeg::Fragment(q)],
            }
        }
        6 => face(node(rng), RoleSeg::FromB(inner(rng))),
        7 => {
            let member = node(rng);
            face(
                node(rng),
                RoleSeg::FromMember {
                    member,
                    of: inner(rng),
                },
            )
        }
        8 => {
            let side = half(rng);
            face(
                node(rng),
                RoleSeg::SplitFragment {
                    parent: inner(rng),
                    side,
                },
            )
        }
        9 => {
            let i = rng.below(2) as u32;
            face(node(rng), RoleSeg::Instance { i, of: inner(rng) })
        }
        10 => face(node(rng), RoleSeg::BlendFace(inner(rng))),
        11 => StableName {
            kind: EntityKind::Edge,
            node: node(rng),
            path: vec![RoleSeg::Seam {
                a: inner(rng),
                b: inner(rng),
            }],
        },
        _ => face(node(rng), RoleSeg::BandFace(list(rng))),
    }
}

/// `name` with the node of every carry its words never say set to one
/// node: a primary carry's (`FromA`), and a split's or a copy's. Two
/// names this makes equal differ only where the full form is silent.
fn unsaid_nodes_erased(name: &StableName) -> StableName {
    let erase = |n: &NameRef| NameRef::new(unsaid_nodes_erased(n));
    let all = |v: &[StableName]| v.iter().map(unsaid_nodes_erased).collect();
    let path: Vec<RoleSeg> = name
        .path
        .iter()
        .map(|seg| match seg {
            RoleSeg::FromA(of) => RoleSeg::FromA(erase(of)),
            RoleSeg::FromB(of) => RoleSeg::FromB(erase(of)),
            RoleSeg::FromMember { member, of } => RoleSeg::FromMember {
                member: *member,
                of: erase(of),
            },
            RoleSeg::SplitFragment { parent, side } => RoleSeg::SplitFragment {
                parent: erase(parent),
                side: *side,
            },
            RoleSeg::Instance { i, of } => RoleSeg::Instance {
                i: *i,
                of: erase(of),
            },
            RoleSeg::BlendFace(of) => RoleSeg::BlendFace(erase(of)),
            RoleSeg::Seam { a, b } => RoleSeg::Seam {
                a: erase(a),
                b: erase(b),
            },
            RoleSeg::Merged(v) => RoleSeg::Merged(all(v)),
            RoleSeg::BandFace(v) => RoleSeg::BandFace(all(v)),
            RoleSeg::Fragment(Qualifier::Borders(v)) => {
                RoleSeg::Fragment(Qualifier::Borders(all(v)))
            }
            RoleSeg::Fragment(Qualifier::Keeps(v)) => RoleSeg::Fragment(Qualifier::Keeps(all(v))),
            RoleSeg::Fragment(Qualifier::Ends(v)) => RoleSeg::Fragment(Qualifier::Ends(all(v))),
            other => other.clone(),
        })
        .collect();
    let unsaid = matches!(
        path.first(),
        Some(RoleSeg::FromA(_) | RoleSeg::SplitFragment { .. } | RoleSeg::Instance { .. })
    );
    StableName {
        kind: name.kind,
        node: if unsaid {
            RecipeNodeId::new(0, 0)
        } else {
            name.node
        },
        path,
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

/// **A join beneath a wrap reads apart from one above it**: a copy of
/// a cut-in cap and a cut-in copy of the cap, and the part of a cut-in
/// cap and a cut-in part of the cap.
#[test]
fn a_join_beneath_a_wrap_reads_apart_from_one_above_it() {
    let cap = StableName {
        kind: EntityKind::Face,
        node: EXTRUDE,
        path: vec![RoleSeg::Cap(CapEnd::Start)],
    };
    let through_b = |name: StableName| StableName {
        kind: name.kind,
        node: OTHER,
        path: vec![RoleSeg::FromB(NameRef::new(name))],
    };
    let copy = |name: StableName| StableName {
        kind: name.kind,
        node: OP,
        path: vec![RoleSeg::Instance {
            i: 1,
            of: NameRef::new(name),
        }],
    };
    let part = |name: StableName| StableName {
        kind: name.kind,
        node: OP,
        path: vec![
            RoleSeg::FromA(NameRef::new(name)),
            RoleSeg::Fragment(Qualifier::Borders(vec![wall(3)])),
        ],
    };
    for (outer, inner, what) in [
        (
            copy(through_b(cap.clone())),
            through_b(copy(cap.clone())),
            "a copy",
        ),
        (
            part(through_b(cap.clone())),
            through_b(part(cap.clone())),
            "a part",
        ),
    ] {
        let (wrapped, joined) = (outer.to_string(), inner.to_string());
        assert_ne!(
            wrapped, joined,
            "{what} of a joined name and a joined {what}"
        );
        assert!(
            wrapped.contains("(the start cap of node ") && wrapped.contains(")"),
            "the join beneath the wrap is said inside it, bracketed: {wrapped}"
        );
        assert!(
            !joined.contains('('),
            "a join above every wrap is said after them, unbracketed: {joined}"
        );
    }
}

/// **Two names read alike in full only where they differ in a carry's
/// node the full form never says** ([`unsaid_nodes_erased`]): the full
/// form brackets every cited name whose words run on past its node, and
/// says wraps and joins in the order the path takes them. The minted
/// shapes a table holds, and names nested up to four deep over every
/// carry and list-bearing role.
#[test]
fn the_full_form_says_no_two_names_alike() {
    let mut rng = fuzz::start("the_full_form_says_no_two_names_alike");
    let mut seen: BTreeMap<String, StableName> = BTreeMap::new();
    let mut collided = Vec::new();
    let mut unsaid = 0usize;
    for k in 0..fuzz::scaled(8000) {
        let name = if k % 2 == 0 {
            random_minted(&mut rng, 3, 4)
        } else {
            let depth = 1 + rng.below(4) as u32;
            random_rich(&mut rng, depth, 3)
        };
        let said = name.to_string();
        match seen.get(&said) {
            Some(prev) if *prev != name => {
                if unsaid_nodes_erased(prev) == unsaid_nodes_erased(&name) {
                    unsaid += 1;
                } else {
                    collided.push((said, prev.clone(), name));
                }
            }
            Some(_) => {}
            None => {
                seen.insert(said, name);
            }
        }
    }
    println!("{unsaid} pairs alike in full differ only in a node it never says");
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
/// lists, nested lists, seams and joins, and of names nested over every
/// carry — no two of which differ only in a node the full form never
/// says, which no table of one node's output is known to hold
/// (`work/recipe/two-names-differing-only-in-an-unsaid-carry-node-read-alike.md`).
#[test]
fn a_tables_words_say_no_two_of_its_names_alike() {
    let mut rng = fuzz::start("a_tables_words_say_no_two_of_its_names_alike");
    let mut checked = 0usize;
    for _ in 0..fuzz::scaled(400) {
        let size = 2 + rng.below(9);
        let pool = 2 + rng.below(5);
        let depth = rng.below(3) as u32;
        let rich = rng.below(2) == 0;
        let mut names: Vec<StableName> = Vec::new();
        for _ in 0..size * 3 {
            let name = if rich {
                random_rich(&mut rng, depth + 1, pool)
            } else {
                random_minted(&mut rng, depth, pool)
            };
            let erased = unsaid_nodes_erased(&name);
            if !names.iter().any(|n| unsaid_nodes_erased(n) == erased) {
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

/// A block, and a Subtract of a pin through it: the block's caps are
/// carried through the Subtract's A, so their names hold at the block
/// and not at the Subtract.
fn block_and_cut(r: &mut Recorder) -> (RecipeNodeId, RecipeNodeId) {
    let (xy, uv) = ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let p = r.profile(
        [0.0; 3],
        xy,
        uv,
        vec![vec![(0.0, -1.0), (4.0, -1.0), (4.0, 3.0), (0.0, 3.0)]],
    );
    let block = r.insert(Node::Extrude {
        profile: p.into(),
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
        profile: q.into(),
        distance: len(1.0),
        side: ExtrudeSide::Along,
    });
    let cut = r.insert(Node::Boolean {
        op: BooleanOp::Subtract,
        a: block.into(),
        b: pin.into(),
        declare: Vec::new(),
    });
    (block, cut)
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
        profile: p.into(),
        distance: len(1.0),
        side: ExtrudeSide::Along,
    });
    let tool = r.insert(Node::Datum(Datum::Plane {
        origin: [len(2.0), len(0.0), len(0.0)],
        normal: [scl(1.0), scl(0.0), scl(0.0)],
    }));
    let split = r.insert(Node::Split {
        target: block.into(),
        tool: tool.into(),
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
        a: block.into(),
        b: split.into(),
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
    let (_, cut) = block_and_cut(&mut r);
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

/// A rim edge or side wall over a step the block's profile draws, by a
/// role that step never draws: a name of the block's own vocabulary its
/// table does not hold, so the insert door admits it and evaluation
/// fails to resolve it.
fn unheld(r: &Recorder, block: RecipeNodeId, kind: EntityKind) -> StableName {
    let piece = ProfileEdgeRef::Piece {
        step: fixture::step_of(&fixture::piece(&r.doc, block, 0, 1)),
        role: PieceRole::Arc,
    };
    StableName {
        kind,
        node: block,
        path: vec![match kind {
            EntityKind::Edge => RoleSeg::RimEdge(CapEnd::End, piece.into()),
            _ => RoleSeg::Lateral(piece.into()),
        }],
    }
}

/// **A blend's and a shell's resolve rows name the slot that failed,
/// above slot zero, as evaluation assigns it**: a fillet whose last
/// edge, and a shell whose second open face, is a name the block does
/// not hold.
#[test]
fn a_resolve_row_names_a_payload_slot_above_zero() {
    let mut r = Recorder::new();
    let (block, _) = block_and_split(&mut r);
    let ev = fixture::run(&r.doc, &EvalOptions::default());
    let unheld_edge = unheld(&r, block, EntityKind::Edge);
    let mut edges: Vec<StableName> = ev
        .value(block)
        .expect("the block evaluates")
        .name_table
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| n.kind == EntityKind::Edge)
        .take(3)
        .chain([unheld_edge.clone()])
        .collect();
    edges.sort();
    edges.dedup();
    let at = edges
        .iter()
        .position(|e| *e == unheld_edge)
        .expect("the selection holds it");
    assert!(at > 0, "the unheld edge sits above slot zero: {edges:#?}");
    let fillet = r.insert(Node::Fillet {
        target: block.into(),
        radius: len(0.1),
        selection: edges,
    });
    let said = failure_of(&r, fillet);
    assert!(
        said.contains(&format!("this fillet's edge {at} ")),
        "the fillet's row names edge {at}: {said}"
    );

    let mut r = Recorder::new();
    let (block, _) = block_and_split(&mut r);
    let shell = r.insert(Node::Shell {
        target: block.into(),
        thickness: len(0.1),
        open: vec![
            fixture::fname(block, RoleSeg::Cap(CapEnd::End)),
            unheld(&r, block, EntityKind::Face),
        ],
    });
    let said = failure_of(&r, shell);
    assert!(
        said.contains("this shell's open face 1 "),
        "the shell's row names open face 1: {said}"
    );
}

/// **A union's declared pair names its second side** where the side
/// that fails is the second: the block's start cap held at the block
/// and at a Subtract of it, which carries the cap under a name of its
/// own, read through the union's member space.
#[test]
fn a_union_resolve_row_names_its_declared_pairs_second_side() {
    let mut r = Recorder::new();
    let (block, cut) = block_and_cut(&mut r);
    let cap = fixture::fname(block, RoleSeg::Cap(CapEnd::Start));
    let union = r.insert(Node::Union {
        members: vec![block.into(), cut.into()],
        declare: vec![(
            (SitedRef::new(block, cap.clone()), SitedRef::new(cut, cap)),
            BooleanCoincidence::REST,
        )],
    });
    let said = failure_of(&r, union);
    assert!(
        said.contains("this union's declared pair 0's second face "),
        "the union's row names the pair's second side: {said}"
    );
}

/// **A strand row names a deleted union**: a face the union holds,
/// painted, and the union deleted.
#[test]
fn a_strand_row_names_a_deleted_union() {
    use editor_core::{Attr, DocEdit, RefusingReach, Rgba8, apply};
    let mut r = Recorder::new();
    let (block, _) = block_and_split(&mut r);
    let q = r.profile(
        [0.0, 0.0, 0.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(1.0, 0.0), (1.5, 0.0), (1.5, 0.5), (1.0, 0.5)]],
    );
    let boss = r.insert(Node::Extrude {
        profile: q.into(),
        distance: len(1.0),
        side: ExtrudeSide::Along,
    });
    let union = r.insert(Node::Union {
        members: vec![block.into(), boss.into()],
        declare: Vec::new(),
    });
    let doc = r.doc;
    let ev = fixture::run(&doc, &EvalOptions::default());
    let painted = ev
        .value(union)
        .expect("the union evaluates")
        .name_table
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| n.kind == EntityKind::Face && n.node == union)
        .expect("a face the union holds");
    let tol = fixture::tol();
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
            name: painted,
            attr: Attr::Color(Rgba8::opaque(1, 2, 3)),
        },
        tol,
        &RefusingReach,
    )
    .expect("paint")
    .doc;
    let applied = apply(
        &doc,
        &DocEdit::DeleteNode { id: union },
        tol,
        &RefusingReach,
    )
    .expect("the delete applies");
    let rows: Vec<String> = applied.maintenance.iter().map(|m| m.to_string()).collect();
    let deleted = format!(
        "this edit deleted {}, which minted the name",
        doc.spoken(union)
    );
    assert!(
        !rows.is_empty() && rows.iter().all(|row| row.contains(&deleted)),
        "every strand row names the deleted union ({deleted}): {rows:#?}"
    );
}
