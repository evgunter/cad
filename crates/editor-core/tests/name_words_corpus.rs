//! **A name's words tell it apart, within a readable sentence**, over
//! every name the corpus mints (`work/recipe/names-render-a-faces-leaf-
//! role-in-words.md`, "A name's words tell it apart", ruled on #3906;
//! the gate's shape ruled on #4069, `work/recipe/refusals-with-the-
//! longest-scoped-names-overrun-the-budget.md`).
//!
//! Every node's name table of every corpus document, evaluated once:
//!
//! - the full form — what a speaker holding no table says — never says
//!   two names of one table alike, from the document or by tag;
//! - the words a speaker holding the evaluation says
//!   ([`Speaker::within`]) never say two names a node holds alike in
//!   its whole table, tied names and every body included;
//! - the names' own lengths hold their ratchet ([`NAME_WORDS`]), and
//!   every word said holds its digest ([`SAID_DIGEST`]), so a wrong word
//!   of the same length shows too;
//! - every refusal that forwards a name, said through the door
//!   production says it through with the corpus's 90th-percentile name,
//!   meets the refusal standard (`test_utils::refusal::problems`), but
//!   for the word budget of the rows [`OVER_BUDGET`] admits and the
//!   recourse marker of the rows [`UNMARKED_RECOURSE`] admits. The
//!   longest names' rows are printed, not gated.
//!
//! Documents built outside the corpus to attack the joins are held to
//! the same uniqueness
//! (`documents_outside_the_corpus_read_apart_too`), each Boolean's join
//! says its operation (`each_boolean_join_says_its_operation`), two
//! copies of one body read apart where a sentence names a face of each
//! (`two_copies_of_one_body_read_apart_where_a_sentence_names_both`),
//! and saying a failed row within the largest table costs no search
//! after its first saying (`a_failed_row_said_within_the_largest_table_is_cheap_again`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::time::Instant;

use crate::corpus::{self, Recorder};
use crate::fixture::{self, len, scl};
use editor_core::{
    Diagnosis, EntityKind, EvalOptions, Evaluation, ExtrudeSide, FaceName, Formula, HitTestError,
    InterrogateError, NameTable, NameTables, Node, NodeError, NodeErrorKind, PatternKind, PickHit,
    ProfileDoc, RecipeEditRef, RecipeNodeId, ResolveError, RoleSeg, SelectRefusal, Speaker,
    StableName,
};
use topo::BooleanOp;

/// **The rows admitted over the word budget at the 90th-percentile
/// name, and the most words each may render**: a ratchet, so a row
/// that grows fails and a row that shrinks lowers its number.
///
/// Two rows, one word over, since INTENT stage 2 PR C: every placed
/// body's names are held twice, by the body and by its world copy, so
/// the 90th-percentile name is a longer one of the same corpus (37
/// words in full). No name a document held before grew.
///
/// Both rows forward two names, and the corpus's 90th-percentile name
/// is a die pip's band face joined into the cutting tool and cut into
/// the die: every member of a union says its join (FORK-DM4), so that
/// name says two joins where a pair boolean's `a` operand said none.
/// The crossing row says its claimed name by tag, which says each read
/// a carry came through.
const OVER_BUDGET: &[(&str, usize)] = &[
    ("SelectRefusal::PairInBand", 82),
    ("NodeErrorKind::CrossingUnverified", 80),
];

/// **The rows whose own prose states its recourse in words the standard
/// does not read as one** ("aim away from the shared edge" is marked;
/// these are not): the prose's, not the names'. Filed as
/// `work/wire/refusals-forwarding-a-name-state-no-marked-recourse.md`.
const UNMARKED_RECOURSE: &[&str] = &[
    "ResolveError::Vanished",
    "SelectRefusal::TiedDisagrees",
    "SelectRefusal::Unreadable",
    "NodeErrorKind::CrossingUnverified",
];

/// **The names' own lengths in words, p50, p99, max and the total**: a
/// face said within its table, then every name a node holds said in
/// full. A ratchet: a number that grows fails, one that shrinks lowers
/// it. The total moves with a word said once more by every name of a
/// kind, which the quantiles of a long tail need not.
///
/// Raised by INTENT stage 2 PR C (the product is the world): each
/// corpus document places its bodies, and a copy's names are its
/// body's under the placement, said "the world copy of …", four words
/// over the body's own. The p99s rise by those four words and the
/// totals by the copies' names; the p50s and the maxima held.
///
/// Merged with main's blend change (`d5a518b1b2`, the die's blend ends
/// with the join), which on main moved the full p99 97 → 98 and the
/// total down: the die's names are said once more each as their copy's,
/// so its longer names weigh twice in the tail and the full p99 reads
/// 106. The total fell by twice main's drop; the p50s, the scoped row
/// and the maxima held.
///
/// Every number rose with the three boolean nodes (FORK-DM4): a union's
/// or an intersect's every member says its join and the read it came in
/// through ("…, joined at Union d1aa from Extrude e548"), where a pair
/// boolean's `a` operand was silent and its `b` said the join alone. A
/// list has no primary member, so the longer names are the reading, not
/// a regression. Merged over INTENT stage 2 C and D, both effects add:
/// a world copy says its body's name, joins and all, under "the world
/// copy of".
const NAME_WORDS: [(&str, [usize; 4]); 2] = [
    ("scoped faces", [16, 43, 57, 55_693]),
    ("full", [19, 123, 196, 411_350]),
];

/// **A digest of every word the corpus's names say** — each name a
/// node holds, in full from the document, by tag, and within the
/// evaluation, in the corpus's order: a wrong word of the same length
/// moves it where [`NAME_WORDS`] cannot see. Re-pinned with the words
/// that moved, said in the PR that moves them.
///
/// INTENT-LITERALS PR C: the words that moved are the node tags (`Extrude
/// e548`): every slot holds a variable's id, so every node is minted
/// from other bytes. [`NAME_WORDS`] held, so no name says a word more or
/// fewer.
///
/// Ids as their mint ordinal and digest: a `Borders` refusal lists its
/// walls in mint order now (it listed them by digest), and nothing else
/// moved — the node tags are still the digest's. Re-taken on PR 4228's
/// tree (a cited line, and `Ends` on every piece), whose words moved
/// it. On that tree the ids reorder an `Ends` list the same way they
/// reorder a `Borders` one (mint order, not digest order), and move no
/// other word.
///
/// INTENT stage 2 PR C: the words that moved are the copies' names, new
/// with the placements ("the world copy of …"), and the node tags of
/// the placements; no name a document held before says another word.
/// Re-taken merged with main's blend change, whose die names it says.
///
/// INTENT stage 2 PR D, merged over C: three words moved, all in
/// `measured_web` — its placement's tag (`PlaceInWorld 57cd328e4061` is
/// now `… 1d7dbb564bd2`), said three times. The placement is minted
/// after the measure, whose preimage D changed. No other word moved.
///
/// INTENT stage 5 PR A: the same tag again (`… 1d7dbb564bd2` is now
/// `… cd9c076ade6a`). The placement is minted after the assertion,
/// whose stored field `dir` became `relation`.
///
/// The three boolean nodes moved it with [`NAME_WORDS`]: every member
/// says its join, and a carry said by tag says the read it came through.
const SAID_DIGEST: u64 = 0x3a99_2833_ddc3_fb0b;

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
#[derive(Clone)]
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
    /// Every form of every name a node holds itself, in order.
    every: Vec<String>,
}

/// Every name of every node's table of `doc`, said in full (from the
/// document and by tag) and within `ev`, each form checked for names
/// said alike.
fn census(label: &str, doc: &ProfileDoc, ev: &Evaluation<f64>) -> Census {
    let full = Speaker::of(doc);
    let scoped = full.within(ev);
    let mut out = Census::default();
    for id in doc.ids() {
        let Some(value) = ev.value(id) else { continue };
        let mut groups: [BTreeMap<String, usize>; 3] = Default::default();
        for (name, _) in value.name_table.iter() {
            let said = full.name(name).to_string();
            let by_tag = name.to_string();
            *groups[0].entry(said.clone()).or_default() += 1;
            *groups[1].entry(by_tag.clone()).or_default() += 1;
            // A name another node minted is said within that node's
            // table, which a pass-through repeats row for row: it is
            // counted there, and checked here against the names this
            // table mixes it with.
            let within = scoped.name(name).to_string();
            *groups[2].entry(within.clone()).or_default() += 1;
            if name.node != id {
                out.said.push(said);
                continue;
            }
            out.full
                .push((said.split_whitespace().count(), name.clone()));
            out.scoped
                .push((within.split_whitespace().count(), name.clone()));
            out.every.extend([said.clone(), by_tag, within]);
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

/// FNV-1a over `said`, each string ended by a zero byte.
fn digest<'a>(said: impl IntoIterator<Item = &'a String>) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for text in said {
        for byte in text.bytes().chain([0]) {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
}

/// p50, p99, max and total of `names`' words.
fn spread(names: &[Counted]) -> [usize; 4] {
    let mut words: Vec<usize> = names.iter().map(|n| n.words).collect();
    words.sort_unstable();
    let at = |q: usize| words[(words.len() - 1) * q / 100];
    [at(50), at(99), at(100), words.iter().sum()]
}

/// The name at the `q`th percentile of `names` by words, and the
/// nearest other name of its document, for a row that names two.
fn at_percentile(names: &[Counted], q: usize) -> (Counted, Counted) {
    let mut sorted = names.to_vec();
    sorted.sort_by_key(|n| n.words);
    let at = (sorted.len() - 1) * q / 100;
    let one = sorted[at].clone();
    let two = (1..sorted.len())
        .flat_map(|d| [at.checked_add(d), at.checked_sub(d)])
        .flatten()
        .filter_map(|i| sorted.get(i))
        .find(|other| other.doc == one.doc)
        .expect("a document holds two names")
        .clone();
    (one, two)
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
    let mut every = Vec::new();
    let (mut full, mut scoped) = (Vec::new(), Vec::new());
    for (di, (d, ev)) in docs.iter().zip(&evals).enumerate() {
        let one = census(d.name, &d.doc, ev);
        alike.extend(one.alike);
        said.extend(one.said);
        every.extend(one.every);
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
    let by_kind: Vec<&String> = said.iter().filter(|s| s.contains(" at Boolean ")).collect();
    assert!(
        by_kind.is_empty(),
        "a join says the Boolean's kind, not its operation: {by_kind:?}"
    );

    let faces: Vec<Counted> = scoped
        .iter()
        .filter(|n| n.name.kind == EntityKind::Face)
        .cloned()
        .collect();
    let words = [("scoped faces", spread(&faces)), ("full", spread(&full))];
    let hash = digest(&every);
    println!("the names' words, p50 p99 max total: {words:?}; digest {hash:#018x}");
    let mut moved = Vec::new();
    if words != NAME_WORDS {
        moved.push(format!(
            "the names say {words:?} words (p50, p99, max, total); NAME_WORDS pins {NAME_WORDS:?}: \
             a number that grew is a regression, one that shrank lowers the pin"
        ));
    }
    if hash != SAID_DIGEST {
        moved.push(format!(
            "the names' words digest to {hash:#018x}; SAID_DIGEST pins {SAID_DIGEST:#018x}: \
             some name says another word — re-pin it with the words that moved"
        ));
    }
    assert!(moved.is_empty(), "{}", moved.join("\n"));

    // The tail: printed, not gated.
    for (form, names, tables) in [("full", &full, false), ("scoped faces", &faces, true)] {
        let (one, two) = at_percentile(names, 100);
        let (doc, ev) = (&docs[one.doc].doc, &evals[one.doc]);
        let tables: &dyn NameTables = if tables { ev } else { &Gone };
        for (row, text) in refusals(&one.name, &two.name, doc, tables) {
            println!(
                "the longest {form} names, {row}: {} words",
                text.split_whitespace().count()
            );
        }
    }

    // The gate: each row at the 90th-percentile name. The bare resolve
    // rows say a name no table holds, so in full; the rest a face its
    // evaluation holds, within it.
    let mut rows = Vec::new();
    for (resolve, names) in [(true, &full), (false, &faces)] {
        let (one, two) = at_percentile(names, 90);
        let (doc, ev) = (&docs[one.doc].doc, &evals[one.doc]);
        println!(
            "the p90 names {}: {} words, {} words",
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
    rows.extend(slot_rows(&docs, &evals));
    let mut over = Vec::new();
    for (row, text) in &rows {
        let words = text.split_whitespace().count();
        println!("{row}: {words} words: {text}");
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
        "a refusal forwarding the corpus's 90th-percentile names misses the standard:\n{}",
        over.join("\n")
    );
}

/// **A node's resolve failure says which slot**: a corpus fillet's
/// last selected edge of several, stranded, as the tree row says it.
fn slot_rows(docs: &[corpus::CorpusDoc], evals: &[Evaluation<f64>]) -> Vec<(&'static str, String)> {
    let (doc, ev, fillet, at, edge) = docs
        .iter()
        .zip(evals)
        .find_map(|(d, ev)| {
            d.doc.ids().iter().find_map(|&id| match d.doc.node(id) {
                Some(Node::Fillet { selection, .. }) if selection.len() > 1 => Some((
                    &d.doc,
                    ev,
                    id,
                    selection.len() - 1,
                    selection.last()?.clone(),
                )),
                _ => None,
            })
        })
        .expect("the corpus fillets more than one edge");
    let stranded = NodeError {
        node: fillet,
        kind: NodeErrorKind::BlendSelectionResolve {
            verb: sweep::blend::BlendKind::Fillet,
            error: Box::new(ResolveError::NodeGone {
                name: edge.clone(),
                edit: RecipeEditRef::NodeDeleted { node: edge.node },
            }),
            reference: at,
        },
        escalations: Default::default(),
    }
    .spoken(doc, ev);
    assert!(
        stranded.contains(&format!("this fillet's edge {at} is stranded: ")),
        "the row says the slot: {stranded}"
    );
    vec![("NodeErrorKind::BlendSelectionResolve", stranded)]
}

/// Every refusal production says that forwards a name, naming `a` (and
/// `b` where it names two), through the door a frame holding the
/// evaluation says it through: `spoken(doc, tables)`. The pick tie is
/// the status line's too (`frame::pick_refusal` says the refusal's own
/// words). The three kind refusals of `NodeErrorKind`
/// carry a kind only evaluation constructs, so they are not here; each
/// says its name in a sentence shorter than `InBand`'s.
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
    let instance = RecipeNodeId::new(0, test_utils::refusal::tagged(1));
    vec![
        (
            "ResolveError::Vanished",
            // The upstream name is one `a` cites, said by its kind.
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
                at: (a.node, b.node),
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
        profile: profile.into(),
        distance: len(distance),
        side: ExtrudeSide::Along,
    })
}

fn moved(r: &mut Recorder, input: RecipeNodeId, by: [f64; 3]) -> RecipeNodeId {
    r.insert(fixture::xform(input, by, [0.0, 0.0, 1.0], 0.0))
}

fn boolean(r: &mut Recorder, op: BooleanOp, a: RecipeNodeId, b: RecipeNodeId) -> RecipeNodeId {
    let declare = Vec::new();
    r.insert(match op {
        BooleanOp::Subtract => Node::Subtract {
            from: a.into(),
            tool: b.into(),
            declare,
        },
        BooleanOp::Union => Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
            declare,
        },
        BooleanOp::Intersect => Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
            declare,
        },
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
            Formula::count(3),
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
            Formula::count(2),
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
    r.insert(Node::Split {
        target: cut.into(),
        tool: tool.into(),
    });
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
            frame: plane.into(),
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
            frame: plane.into(),
            loops: vec![rod_loop(false)],
            ids: Vec::new(),
        }),
    );
    let (doc, rod) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
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
                fresh: Vec::new(),
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

/// **Each Boolean's B join says its operation**: a pin cut in, joined
/// and intersected at a Boolean of each operation, every name the
/// Boolean carries through its B saying the operation and the node.
#[test]
fn each_boolean_join_says_its_operation() {
    for (op, verb, noun) in [
        (BooleanOp::Subtract, "cut in", "Subtract"),
        (BooleanOp::Union, "joined", "Union"),
        (BooleanOp::Intersect, "intersected", "Intersect"),
    ] {
        let mut r = Recorder::new();
        let (block, pin) = block_and_pin(&mut r);
        let at = boolean(&mut r, op, block, pin);
        let ev = fixture::run(&r.doc, &EvalOptions::default());
        let table = &ev
            .value(at)
            .unwrap_or_else(|| panic!("{op:?} evaluates: {:?}", corpus::failures(&ev)))
            .name_table;
        let by = Speaker::of(&r.doc);
        let pin_read = fixture::out(&r.doc, pin);
        // A member's join also says the read it came in through; a
        // subtract's tool is its one cut-in read, said by the verb alone.
        let from = match op {
            BooleanOp::Subtract => String::new(),
            BooleanOp::Union | BooleanOp::Intersect => {
                format!(" from Extrude {}", test_utils::refusal::tag(pin.0.digest()))
            }
        };
        let join = format!(
            ", {verb} at {noun} {}{from}",
            test_utils::refusal::tag(at.0.digest())
        );
        let through_b: Vec<String> = table
            .iter()
            .map(|(name, _)| name)
            .filter(|name| {
                matches!(
                    name.path.as_slice(),
                    [RoleSeg::From { read, .. }] if *read == pin_read
                )
            })
            .map(|name| by.name(name).to_string())
            .collect();
        assert!(
            !through_b.is_empty(),
            "{op:?}: the pin's faces come through B"
        );
        for said in &through_b {
            assert!(said.ends_with(&join), "{op:?}: {said}");
        }
    }
}

/// **Two copies of one body read apart where a sentence names a face of
/// each**: the copies' faces carry names alike, since a name says the
/// node that made it and not the node holding it, so the flush query's
/// in-band pair and the pick's tie say each face's node.
#[test]
fn two_copies_of_one_body_read_apart_where_a_sentence_names_both() {
    let mut r = Recorder::new();
    let block = r.profile([0.0; 3], XY.0, XY.1, vec![square(0.5, 0.5, 0.5)]);
    let block = extrude(&mut r, block, 1.0);
    let one = moved(&mut r, block, [0.0, 0.0, 0.0]);
    let two = moved(&mut r, block, [0.5, 0.0, 0.0]);
    let doc = r.doc;
    let ev = fixture::run(&doc, &EvalOptions::default());
    let findings = editor_core::find_flush_candidates(&ev, &doc, one, two, fixture::tol())
        .expect("the copies' flush faces are decided");
    let alike = findings
        .iter()
        .find(|finding| finding.pair.0.name == finding.pair.1.name)
        .expect("the copies' end caps lie flush, and carry one name");
    let face = alike.pair.0.name.clone();
    let by = Speaker::of(&doc).within(&ev);
    let (at_one, at_two) = (doc.spoken(one).to_string(), doc.spoken(two).to_string());
    let band = geom_core::Indeterminate {
        margin: geom_core::MarginDiag::value(3e-11),
        band: geom_core::Band::new(1e-12, 1e-9).expect("zero < escalate"),
        predicate: Some("bool_plane_offset"),
        terminal_sliver: false,
    };
    let pair = SelectRefusal::PairInBand {
        pair: Box::new((face.clone(), face.clone())),
        at: (one, two),
        predicate: "bool_plane_offset",
        source: band,
    }
    .spoken(&doc, &ev);
    let hit = |node| PickHit {
        name: face.clone(),
        node,
        body: 0,
        t: 1.0,
        t_lo: 1.0,
        t_hi: 1.0,
        point: geom_core::Point3::new(0.0, 0.0, 0.0),
    };
    let tie = HitTestError::Ambiguous {
        hits: vec![hit(one), hit(two)],
    }
    .spoken(&doc, &ev);
    let said = by.name(&face).to_string();
    for text in [&pair, &tie] {
        assert!(
            text.contains(&format!("{said} on {at_one}"))
                && text.contains(&format!("{said} on {at_two}")),
            "each face is said with the copy holding it: {text}"
        );
    }
}

/// **A failed row said within the largest corpus table costs no search
/// again**: the tree says a failed node's row every frame, and the
/// detail each name is said at is worked out once per table. Measured
/// on the corpus's largest table: the first saying pays the table's
/// search, every later one says words alone.
#[test]
fn a_failed_row_said_within_the_largest_table_is_cheap_again() {
    let mut docs = corpus::documents();
    docs.push(corpus::cup::document());
    docs.push(corpus::vessel::document());
    let (doc, ev, node) = docs
        .iter()
        .map(|d| (&d.doc, fixture::run(&d.doc, &EvalOptions::default())))
        .filter_map(|(doc, ev)| {
            let node = doc
                .ids()
                .iter()
                .copied()
                .filter(|&id| ev.value(id).is_some())
                .max_by_key(|&id| ev.value(id).map_or(0, |v| v.name_table.iter().count()))?;
            Some((doc, ev, node))
        })
        .max_by_key(|(_, ev, node)| ev.value(*node).map_or(0, |v| v.name_table.iter().count()))
        .expect("the corpus evaluates");
    let faces: Vec<StableName> = ev
        .value(node)
        .expect("the largest table's node")
        .name_table
        .iter()
        .map(|(name, _)| name.clone())
        .filter(|name| name.node == node && name.kind == EntityKind::Face)
        .collect();
    let row = |a: &StableName, b: &StableName| {
        NodeError {
            node,
            kind: NodeErrorKind::CrossingUnverified {
                instance: node,
                outer: Box::new(FaceName::new(a.clone()).expect("a face")),
                name: Box::new(b.clone()),
            },
            escalations: Default::default(),
        }
        .spoken(doc, &ev)
    };
    let (a, b) = (&faces[0], &faces[faces.len() - 1]);
    let first = Instant::now();
    let said = row(a, b);
    let first = first.elapsed();
    const FRAMES: u32 = 60;
    let again = Instant::now();
    for _ in 0..FRAMES {
        assert_eq!(row(a, b), said, "the row says the same words each frame");
    }
    let again = again.elapsed() / FRAMES;
    println!(
        "{} names in the largest table: the first saying {first:?}, each later one {again:?}",
        ev.value(node).map_or(0, |v| v.name_table.iter().count())
    );
    assert!(
        again * 4 < first,
        "a later saying searches the table again: the first took {first:?}, each later one \
         {again:?}"
    );
}
