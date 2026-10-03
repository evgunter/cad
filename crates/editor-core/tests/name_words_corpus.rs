//! **A name's words tell it apart, within a readable sentence**, over
//! every name the corpus mints (`work/recipe/names-render-a-faces-leaf-
//! role-in-words.md`, "A name's words tell it apart", ruled on #3906).
//!
//! Every node's name table of every corpus document, evaluated once:
//!
//! - the full form — what a speaker holding no table says — never says
//!   two names of one table alike, from the document or by tag;
//! - the words a speaker holding the evaluation says
//!   ([`Speaker::within`]) never say two names a node holds alike in
//!   its whole table, tied names and every body included;
//! - every refusal that forwards a name, said through the door a frame
//!   holding the evaluation says it through (`spoken(doc, evaluation)`)
//!   with the corpus's longest names that door says, meets the refusal
//!   standard (`test_utils::refusal::problems`), but for the word budget
//!   of the rows [`OVER_BUDGET`] admits and the recourse marker of the
//!   rows [`UNMARKED_RECOURSE`] admits;
//! - the words of every name the corpus holds, in full and scoped, sum
//!   to [`SAID_WORDS`].
//!
//! Documents built outside the corpus to attack the joins are held to
//! the same uniqueness
//! (`documents_outside_the_corpus_read_apart_too`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::corpus::{self, Recorder};
use crate::fixture::{self, len, scl};
use editor_core::{
    BooleanOp, Diagnosis, EntityKind, EvalOptions, Evaluation, Expr, ExtrudeSide, FaceName,
    HitTestError, InterrogateError, NameTable, NameTables, Node, NodeError, NodeErrorKind,
    PatternKind, PickHit, ProfileDoc, RecipeEditRef, RecipeNodeId, ResolveError, SelectRefusal,
    Speaker, StableName,
};

/// **The rows admitted over the word budget, and the most words each
/// may render**: a ratchet, so a row that grows fails and a row that
/// shrinks lowers its number. Each is said as production says it: the
/// resolve rows forward names their evaluation no longer holds, so in
/// full; the others, names it holds, within its tables. Filed as
/// `work/recipe/refusals-with-the-longest-scoped-names-overrun-the-budget.md`.
const OVER_BUDGET: &[(&str, usize)] = &[
    ("ResolveError::Vanished", 263),
    ("ResolveError::NodeGone", 136),
    ("SelectRefusal::InBand", 106),
    ("SelectRefusal::TiedDisagrees", 81),
    ("SelectRefusal::Unreadable", 76),
    ("SelectRefusal::PairInBand", 145),
    ("NodeErrorKind::CrossingUnverified", 127),
    ("HitTestError::Ambiguous", 133),
];

/// **The rows whose own prose states its recourse in words the standard
/// does not read as one** ("the repair is an explicit rebind", "aim away
/// from the shared edge"): the prose's, not the names'. Filed as
/// `work/wire/refusals-forwarding-a-name-state-no-marked-recourse.md`.
const UNMARKED_RECOURSE: &[&str] = &[
    "ResolveError::Vanished",
    "ResolveError::NodeGone",
    "SelectRefusal::TiedDisagrees",
    "SelectRefusal::Unreadable",
    "NodeErrorKind::CrossingUnverified",
    "HitTestError::Ambiguous",
];

/// **The words of every name a corpus node holds itself, summed: in
/// full, then within its evaluation**. A ratchet over every name, not
/// only the longest: a row that grows fails, one that shrinks lowers
/// its number.
const SAID_WORDS: (usize, usize) = (311_608, 291_146);

/// The tables an evaluation answers for a name it does not hold: a
/// vanished name is in no table of the run that refuses it, and a
/// deleted node has none.
struct Gone;

impl NameTables for Gone {
    fn table(&self, _: RecipeNodeId) -> Option<&NameTable> {
        None
    }
}

/// A name said, its words counted, with the document it was said from.
struct Counted {
    words: usize,
    doc: usize,
    name: StableName,
}

/// What one evaluated document says of its names.
#[derive(Default)]
struct Census {
    /// Each group of names said alike, one line each.
    alike: Vec<String>,
    /// Each name a node holds itself, in full.
    full: Vec<(usize, StableName)>,
    /// The same names, within the evaluation.
    scoped: Vec<(usize, StableName)>,
    /// Every full form said, for what a join says.
    said: Vec<String>,
}

/// Every name of every node's table of `doc`, said in full (from the
/// document and by tag) and within `ev`, each form checked for names
/// said alike.
fn census(label: &str, doc: &ProfileDoc, ev: &Evaluation<f64>) -> Census {
    let full = Speaker::of(doc);
    let scoped = full.within(ev);
    let mut out = Census::default();
    for &id in doc.order() {
        let Some(value) = ev.value(id) else { continue };
        let mut groups: [BTreeMap<String, usize>; 3] = Default::default();
        for (name, _) in value.name_table.iter() {
            let said = full.name(name).to_string();
            *groups[0].entry(said.clone()).or_default() += 1;
            *groups[1].entry(name.to_string()).or_default() += 1;
            // A name another node holds is said within that node's
            // table, which a pass-through repeats row for row.
            if name.node != id {
                out.said.push(said);
                continue;
            }
            let within = scoped.name(name).to_string();
            out.full
                .push((said.split_whitespace().count(), name.clone()));
            out.scoped
                .push((within.split_whitespace().count(), name.clone()));
            *groups[2].entry(within).or_default() += 1;
            out.said.push(said);
        }
        for (form, group) in ["from the document", "by tag", "within the evaluation"]
            .iter()
            .zip(&groups)
        {
            out.alike.extend(
                group
                    .iter()
                    .filter(|(_, n)| **n > 1)
                    .map(|(said, n)| format!("[{label}] {form}, x{n}: {said}")),
            );
        }
    }
    out
}

#[test]
fn every_corpus_name_reads_apart_and_forwards_within_the_refusal_budget() {
    let mut docs = corpus::documents();
    docs.push(corpus::cup::document());
    docs.push(corpus::vessel::document());
    let evals: Vec<Evaluation<f64>> = docs
        .iter()
        .map(|d| fixture::run(&d.doc, &EvalOptions::default()))
        .collect();
    let mut alike = Vec::new();
    let mut said = Vec::new();
    let (mut full, mut scoped) = (Vec::new(), Vec::new());
    for (di, (d, ev)) in docs.iter().zip(&evals).enumerate() {
        let one = census(d.name, &d.doc, ev);
        alike.extend(one.alike);
        said.extend(one.said);
        let counted = |(words, name)| Counted {
            words,
            doc: di,
            name,
        };
        full.extend(one.full.into_iter().map(counted));
        scoped.extend(one.scoped.into_iter().map(counted));
    }
    assert!(
        alike.is_empty(),
        "distinct names of one table are said alike:\n{}",
        alike.join("\n")
    );

    // A boolean's B is said by its operation, never by the node's kind.
    assert!(
        said.iter().any(|s| s.contains(", cut in at Subtract ")),
        "the corpus cuts a pocket, and its walls say the Subtract that cut them in"
    );
    let by_kind: Vec<&String> = said.iter().filter(|s| s.contains(" at Boolean ")).collect();
    assert!(
        by_kind.is_empty(),
        "a join says the Boolean's kind, not its operation: {by_kind:?}"
    );

    let sum = |names: &[Counted]| names.iter().map(|n| n.words).sum::<usize>();
    let words = (sum(&full), sum(&scoped));
    println!("the words of every name, in full and scoped: {words:?}");
    assert!(
        words == SAID_WORDS,
        "the corpus's names say {words:?} words, in full and scoped; SAID_WORDS pins \
         {SAID_WORDS:?}: a count that grew is a regression, one that shrank lowers the pin"
    );

    full.sort_by_key(|said| core::cmp::Reverse(said.words));
    scoped.sort_by_key(|said| core::cmp::Reverse(said.words));
    let mut rows = Vec::new();
    // The resolve rows forward names their evaluation no longer holds,
    // so in full, over the two longest one document holds; every other
    // row a face its evaluation holds, so within it, over the two
    // longest face names one document holds.
    for (resolve, names) in [(true, &full), (false, &scoped)] {
        let mut kept = names
            .iter()
            .filter(|said| resolve || said.name.kind == EntityKind::Face);
        let one = kept.next().expect("the corpus names a face");
        let two = kept
            .find(|other| other.doc == one.doc)
            .expect("the longest-named document holds two names");
        let (doc, ev) = (&docs[one.doc].doc, &evals[one.doc]);
        println!(
            "the longest names {}: {} words, {} words",
            if resolve { "in full" } else { "scoped" },
            one.words,
            two.words,
        );
        let tables: &dyn NameTables = if resolve { &Gone } else { ev };
        rows.extend(
            refusals(&one.name, &two.name, doc, tables)
                .into_iter()
                .filter(|(row, _)| row.starts_with("ResolveError::") == resolve),
        );
    }
    let mut over = Vec::new();
    for (row, text) in &rows {
        let words = text.split_whitespace().count();
        println!("{row}: {words} words");
        let allowed: &[&str] = match row.split("::").next() {
            Some("SelectRefusal") => &["select"],
            Some("HitTestError") => &["hit test"],
            _ => &[],
        };
        // The budget is the ratchet's below; an unmarked recourse, the
        // admission's.
        let budget = format!("{row} renders {words} words, over");
        let unmarked = format!("{row} states no recourse");
        let problems = test_utils::refusal::problems(row, text, allowed, false);
        let admitted = UNMARKED_RECOURSE.contains(row);
        if admitted && !problems.iter().any(|p| p.starts_with(&unmarked)) {
            over.push(format!(
                "{row} now marks its recourse; take it out of UNMARKED_RECOURSE"
            ));
        }
        over.extend(problems.into_iter().filter(|problem| {
            !problem.starts_with(&budget) && !(admitted && problem.starts_with(&unmarked))
        }));
        let admitted = OVER_BUDGET
            .iter()
            .find_map(|(admitted, most)| (admitted == row).then_some(*most));
        match admitted {
            None if words > test_utils::refusal::BUDGET => {
                over.push(format!(
                    "{row} renders {words} words, over the budget: {text}"
                ));
            }
            Some(most) if words > most => {
                over.push(format!(
                    "{row} renders {words} words, over its admitted {most}: {text}"
                ));
            }
            Some(most) if words <= test_utils::refusal::BUDGET || words < most => {
                over.push(format!(
                    "{row} renders {words} words; lower its admission of {most}"
                ));
            }
            _ => {}
        }
    }
    assert!(
        over.is_empty(),
        "a refusal forwarding the corpus's longest names misses the standard:\n{}",
        over.join("\n")
    );
}

/// Every refusal production says that forwards a name, naming `a` (and
/// `b` where it names two), through the door a frame holding the
/// evaluation says it through: `spoken(doc, tables)`. The three kind
/// refusals of `NodeErrorKind` carry a kind only evaluation constructs,
/// so they are not here; each says its name in a sentence shorter than
/// `InBand`'s.
fn refusals(
    a: &StableName,
    b: &StableName,
    doc: &ProfileDoc,
    tables: &dyn NameTables,
) -> Vec<(&'static str, String)> {
    let face = |n: &StableName| {
        FaceName::new(StableName {
            kind: EntityKind::Face,
            node: n.node,
            path: n.path.clone(),
        })
        .expect("a face kind is a face name")
    };
    let band = || geom_core::Indeterminate {
        margin: geom_core::MarginDiag::value(3e-11),
        band: geom_core::Band::new(1e-12, 1e-9).expect("zero < escalate"),
        predicate: Some("bool_plane_offset"),
        terminal_sliver: false,
    };
    let hit = |name: &StableName| PickHit {
        name: name.clone(),
        node: name.node,
        body: 0,
        t: 1.0,
        t_lo: 1.0,
        t_hi: 1.0,
        point: geom_core::Point3::new(0.0, 0.0, 0.0),
    };
    let instance = RecipeNodeId(test_utils::refusal::tagged(1));
    vec![
        (
            "ResolveError::Vanished",
            ResolveError::Vanished {
                name: a.clone(),
                diagnosis: Diagnosis::Cascade { through: b.clone() },
                last_good: None,
            }
            .spoken(doc, tables),
        ),
        (
            "ResolveError::NodeGone",
            ResolveError::NodeGone {
                name: a.clone(),
                edit: RecipeEditRef::NodeDeleted { node: a.node },
            }
            .spoken(doc, tables),
        ),
        (
            "SelectRefusal::InBand",
            SelectRefusal::InBand {
                name: Box::new(a.clone()),
                predicate: "sel_datum_distance",
                source: band(),
            }
            .spoken(doc, tables),
        ),
        (
            "SelectRefusal::TiedDisagrees",
            SelectRefusal::TiedDisagrees {
                name: Box::new(a.clone()),
                matched: 1,
                candidates: 3,
            }
            .spoken(doc, tables),
        ),
        (
            "SelectRefusal::Unreadable",
            SelectRefusal::Unreadable {
                name: Box::new(a.clone()),
                error: InterrogateError::WholeBody,
            }
            .spoken(doc, tables),
        ),
        (
            "SelectRefusal::PairInBand",
            SelectRefusal::PairInBand {
                pair: Box::new((a.clone(), b.clone())),
                predicate: "bool_plane_offset",
                source: band(),
            }
            .spoken(doc, tables),
        ),
        (
            "NodeErrorKind::CrossingUnverified",
            NodeError {
                node: instance,
                kind: NodeErrorKind::CrossingUnverified {
                    instance,
                    outer: Box::new(face(a)),
                    name: Box::new(b.clone()),
                },
                escalations: Default::default(),
            }
            .spoken(doc, tables),
        ),
        (
            "HitTestError::Ambiguous",
            HitTestError::Ambiguous {
                hits: vec![hit(a), hit(b)],
            }
            .spoken(doc, tables),
        ),
    ]
}

fn extrude(r: &mut Recorder, profile: RecipeNodeId, distance: f64) -> RecipeNodeId {
    r.insert(Node::Extrude {
        profile,
        distance: len(distance),
        side: ExtrudeSide::Along,
    })
}

fn moved(r: &mut Recorder, input: RecipeNodeId, by: [f64; 3]) -> RecipeNodeId {
    r.insert(fixture::xform(input, by, [0.0, 0.0, 1.0], 0.0))
}

fn boolean(r: &mut Recorder, op: BooleanOp, a: RecipeNodeId, b: RecipeNodeId) -> RecipeNodeId {
    r.insert(Node::Boolean {
        op,
        a,
        b,
        declare: Vec::new(),
    })
}

fn square(x: f64, y: f64, h: f64) -> Vec<(f64, f64)> {
    vec![
        (x - h, y - h),
        (x + h, y - h),
        (x + h, y + h),
        (x - h, y + h),
    ]
}

const XY: ([f64; 3], [f64; 3]) = ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);

/// A block and a square pin on it, the pin's master extruded once.
fn block_and_pin(r: &mut Recorder) -> (RecipeNodeId, RecipeNodeId) {
    let block = r.profile([0.0; 3], XY.0, XY.1, vec![square(2.0, 1.0, 2.0)]);
    let block = extrude(r, block, 1.0);
    let pin = r.profile([0.0, 0.0, 0.75], XY.0, XY.1, vec![square(0.5, 0.5, 0.125)]);
    (block, extrude(r, pin, 0.5))
}

/// **Documents built to attack the joins read apart too**: copies of
/// one master carried through chained Subtracts and an n-ary Union,
/// then also unioned on as bosses; two levels of pattern subtracted and
/// split through the pockets; a profile of four loops, three of them
/// alike, and two profiles of one shape joined by a Boolean and by an
/// n-ary Union; round pins cut, and unioned, by three successive
/// Booleans, a fillet over one pocket's whole rim, and the filleted
/// body unioned twice.
#[test]
fn documents_outside_the_corpus_read_apart_too() {
    let mut docs: Vec<(String, ProfileDoc)> = Vec::new();

    let mut r = Recorder::new();
    let (block, pin) = block_and_pin(&mut r);
    let copies = [0.0, 0.75, 1.5, 2.25].map(|x| moved(&mut r, pin, [x, 0.0, 0.0]));
    let first = boolean(&mut r, BooleanOp::Subtract, block, copies[0]);
    let second = boolean(&mut r, BooleanOp::Subtract, first, copies[1]);
    let (doc, union) = fixture::union_over(r.doc.clone(), &copies[2..], Vec::new());
    r.doc = doc;
    let cut = boolean(&mut r, BooleanOp::Subtract, second, union);
    docs.push(("copies".to_owned(), r.doc.clone()));
    let bosses = [[0.0, 1.0, 0.5], [0.75, 1.0, 0.5]].map(|at| moved(&mut r, pin, at));
    let once = boolean(&mut r, BooleanOp::Union, cut, bosses[0]);
    boolean(&mut r, BooleanOp::Union, once, bosses[1]);
    docs.push(("copies and bosses".to_owned(), r.doc.clone()));

    let mut r = Recorder::new();
    let (block, pin) = block_and_pin(&mut r);
    let row = r.insert(
        Node::placed_union(
            pin,
            Expr::count(3),
            PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(0.75),
            },
        )
        .expect("a linear pattern"),
    );
    let grid = r.insert(
        Node::placed_union(
            row,
            Expr::count(2),
            PatternKind::Linear {
                direction: [scl(0.0), scl(1.0), scl(0.0)],
                spacing: len(1.0),
            },
        )
        .expect("a linear pattern"),
    );
    let cut = boolean(&mut r, BooleanOp::Subtract, block, grid);
    let tool = r.insert(Node::Datum(editor_core::Datum::Plane {
        origin: [len(0.0), len(0.5), len(0.0)],
        normal: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    r.insert(Node::Split { target: cut, tool });
    docs.push(("pattern and split".to_owned(), r.doc.clone()));

    let mut r = Recorder::new();
    let loops = |o: f64| {
        vec![
            square(2.0 + o, 2.0 + o, 2.0),
            square(1.0 + o, 1.0 + o, 0.25),
            square(3.0 + o, 1.0 + o, 0.25),
            square(1.0 + o, 3.0 + o, 0.25),
        ]
    };
    let lower = r.profile([0.0; 3], XY.0, XY.1, loops(0.0));
    let lower = extrude(&mut r, lower, 0.5);
    let upper = r.profile([0.0, 0.0, 0.25], XY.0, XY.1, loops(0.1));
    let upper = extrude(&mut r, upper, 0.5);
    boolean(&mut r, BooleanOp::Union, lower, upper);
    let apart = [lower, upper].map(|at| moved(&mut r, at, [6.0, 0.0, 0.0]));
    let (doc, _) = fixture::union_over(r.doc.clone(), &[apart[0], apart[1], lower], Vec::new());
    docs.push(("loops and two profiles".to_owned(), doc));

    for op in [BooleanOp::Subtract, BooleanOp::Union] {
        let mut r = Recorder::new();
        let block = r.profile([0.0; 3], XY.0, XY.1, vec![square(2.0, 1.0, 2.0)]);
        let block = extrude(&mut r, block, 1.0);
        let plane = r.insert(fixture::frame([0.0, 0.0, 0.75], XY.0, XY.1));
        let pin = r.insert(Node::Profile(editor_core::ProfileProgram {
            plane,
            loops: vec![editor_core::LoopProgram::circle(0.5, 0.5, 0.2).expect("a circle")],
            ids: Vec::new(),
        }));
        let pin = extrude(&mut r, pin, 0.5);
        let copies = [0.0, 1.0, 2.0].map(|x| moved(&mut r, pin, [x, 0.0, 0.0]));
        let first = boolean(&mut r, op, block, copies[0]);
        let second = boolean(&mut r, op, first, copies[1]);
        let third = boolean(&mut r, op, second, copies[2]);
        let ev = fixture::run(&r.doc, &EvalOptions::default());
        let rim: Vec<StableName> = ev
            .value(third)
            .unwrap_or_else(|| panic!("the round pins evaluate: {:?}", corpus::failures(&ev)))
            .name_table
            .iter()
            .map(|(name, _)| name)
            .filter(|name| {
                name.kind == EntityKind::Edge
                    && name.node == third
                    && editor_core::role_leaf(name).node == second
            })
            .cloned()
            .collect();
        assert!(
            !rim.is_empty(),
            "the second pin's rim is named at the third Boolean"
        );
        let fillet = r.insert(Node::fillet(third, len(0.05), rim));
        let twice = [[0.0; 3], [0.0, 5.5, 0.0]].map(|at| moved(&mut r, fillet, at));
        let (doc, _) = fixture::union_over(r.doc.clone(), &twice, Vec::new());
        docs.push((format!("round pins, {op:?}"), doc));
    }

    let mut alike = Vec::new();
    for (label, doc) in &docs {
        let ev = fixture::run(doc, &EvalOptions::default());
        let failures = corpus::failures(&ev);
        assert!(failures.is_empty(), "[{label}] evaluates: {failures:?}");
        alike.extend(census(label, doc, &ev).alike);
    }
    assert!(
        alike.is_empty(),
        "distinct names of one table are said alike:\n{}",
        alike.join("\n")
    );
}

/// **A kept name spoken again after its step was dropped says the step
/// by its tag**, never the row it sat at, which is now another step's;
/// a kept step that only moved is said at its new row. Either way the
/// name reads as it would spoken fresh from the new document.
#[test]
fn respoken_after_a_dropped_step() {
    use crate::corpus::reshaped_rod::{CREASE, bump_ids, lateral_edge, rod_ids, rod_loop};
    use editor_core::{
        DocEdit, Node, ProfileDoc, ProfileProgram, ProfileVertexRef, RefusingReach, RoleSeg, apply,
    };

    let tol = fixture::tol();
    let doc = ProfileDoc::empty_derived("respoken-after-a-dropped-step", tol);
    let (doc, plane) = fixture::insert(doc, fixture::xy_frame());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![rod_loop(false)],
            ids: Vec::new(),
        }),
    );
    let (doc, rod) = fixture::insert(
        doc,
        Node::Extrude {
            profile,
            distance: fixture::len(2.0),
            side: editor_core::ExtrudeSide::Along,
        },
    );
    let crease = lateral_edge(&doc, rod, CREASE);
    let [RoleSeg::LateralEdge(ProfileVertexRef::Piece { step, .. })] = crease.path.as_slice()
    else {
        panic!("the crease is a lateral edge over a step's piece: {crease:?}");
    };
    let row = rod_ids(&doc, profile)
        .iter()
        .position(|id| id == step)
        .expect("the plain rod draws the crease's step");
    let kept = doc.spoken_name(&crease);
    assert!(
        kept.to_string().contains(&format!("loop 0 step {row} ")),
        "spoken from the plain rod, the step is its row: {kept}"
    );
    let reshape = |drop: bool| {
        let mut ids = bump_ids(&doc, profile);
        if drop {
            let at = ids[0]
                .iter()
                .position(|id| *id == Some(*step))
                .expect("the bump keeps the crease's step");
            ids[0][at] = None;
        }
        apply(
            &doc,
            &DocEdit::SetProgram {
                node: profile,
                loops: vec![rod_loop(true)],
                ids,
            },
            tol,
            &RefusingReach,
        )
        .expect("the reshaping applies")
        .doc
    };

    let dropped = reshape(true);
    let respoken = kept.respoken(&dropped).to_string();
    assert!(
        !respoken.contains(&format!("loop 0 step {row} ")),
        "a dropped step keeps no row another step now holds: {respoken}"
    );
    assert!(
        respoken.contains(&format!("the profile step {step}")),
        "a dropped step is said by its tag: {respoken}"
    );
    assert_eq!(respoken, dropped.spoken_name(&crease).to_string());

    let moved = reshape(false);
    let respoken = kept.respoken(&moved).to_string();
    assert!(
        respoken.contains(&format!("loop 0 step {} ", row + 2)),
        "a kept step that moved is said at its new row: {respoken}"
    );
    assert_eq!(respoken, moved.spoken_name(&crease).to_string());
}
