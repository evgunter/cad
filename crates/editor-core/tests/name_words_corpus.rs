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
use crate::fixture::{self, len};
use editor_core::{
    Diagnosis, EntityKind, EvalOptions, Evaluation, FaceName, Formula, HitTestError,
    InterrogateError, NameTable, NameTables, Node, NodeError, NodeErrorKind, PickHit, ProfileDoc,
    RecipeEditRef, RecipeNodeId, ResolveError, RoleSeg, SelectRefusal, Speaker, StableName,
};
use topo::BooleanOp;

/// **The rows admitted over the word budget at the 90th-percentile
/// name, and the most words each may render**: a ratchet, so a row
/// that grows fails and a row that shrinks lowers its number.
///
/// One row: every placed body's names are held twice, by the body and
/// by its world copy, so the 90th-percentile name is a longer one of
/// the same corpus (37 words in full).
///
/// The row forwards two names, and the corpus's 90th-percentile name
/// is a die pip's band face joined into the cutting tool and cut into
/// the die: every member of a union says its join (FORK-DM4), so that
/// name says two joins where a pair boolean's `a` operand said none.
/// It says its claimed name by tag, which says each read a carry came
/// through.
///
/// Every construction is the root of its own space, so a later operand
/// of a union is placed against the first before it is joined, and the
/// name says that read too ("joined at Union … from Place …"), and its
/// claimed name says one more read by tag
/// (`name-words-budget-grew-under-placed-copy-wrapping`).
const OVER_BUDGET: &[(&str, usize)] = &[("NodeErrorKind::CrossingUnverified", 80)];

/// **The rows whose own prose states its recourse in words the standard
/// does not read as one** ("aim away from the shared edge" is marked;
/// these are not): the prose's, not the names'. Filed as
/// `work/wire/refusals-forwarding-a-name-state-no-marked-recourse.md`.
const UNMARKED_RECOURSE: &[&str] = &[
    "SelectRefusal::TiedDisagrees",
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
///
/// The totals rose again (12 and 68 words) when `part_select`'s union
/// came to read the split's ports where it read a `Part` of each half:
/// a member said as a port says the split and the half ("from Split
/// …'s above"), where a `Part` was said by its one tag. The percentiles
/// and maxima held.
///
/// The full total rose by 22 with the killed-lineage chase (`emit_topo`'s
/// `KilledSplits`). In `crossing_slots` and its world copy, an edge read
/// as a seam of two faces is now said as the piece of a rim it is,
/// between its two ends. Its end vertices are said by the edges they lie
/// on, not as a junction of seam lines. The longer sentences are the
/// right names, not a regression. The percentiles, the maxima and the
/// scoped row held.
///
/// The full total fell by 102 and its p99 by one with the vertex run
/// (N2: an `EdgeCrossing` is a run of lines): each edge of a run is said
/// against "the rest", where a pair was said "the other body" and "the
/// first", in `corner_table`'s and `crossing_slots`' edge × edge vertices
/// and the pieces whose ends cite them. No name moved: read as a set of
/// (line, sense), every run is the pair it replaced. The maxima and the
/// scoped row held.
///
/// The full total rose by 154 with the output stage's killed lineage
/// (the records of the edges the merge kills, copied into
/// `BooleanNaming::killed_lineage`). In `crossing_slots` and its world
/// copy, a stretch of the first cavity's rim said as a seam of two faces
/// is said as the piece of that rim it is, and a floor corner said as a
/// touch of a wall is said as a crossing of two rims, as are the pieces
/// whose ends cite it. The longer sentences are the right names, not a
/// regression. The percentiles, the maxima and the scoped row held.
///
/// The full total rose by 96 with a seam citing the parent face (N3).
/// In `crossing_slots` and its world copy, the two seams beside the
/// merged floor say its `Merged` parent, over both floors, where they
/// said one of them. The longer sentences are the right names, not a
/// regression. The percentiles, the maxima and the scoped row held.
///
/// Every number but the p50s and the scoped maximum moved with INTENT
/// stage 3 D (Transform retires into a placement), and the full row
/// grew: p99 122 → 128, max 196 → 366, total 411230 → 444393; the
/// scoped row's p99 fell 43 → 41 and its total rose 55615 → 59553.
/// Every construction is the root of its own space, so each operand
/// after a combine's first is placed against the first's root, and its
/// names come in wrapped `Placed { of }`, said "joined at Union … from
/// Place …". The longest full name, in `nested_islands_106_depth2`, is
/// a piece of a lateral edge between two crossings, whose ends say the
/// edge again: each extra placement read is said three times over in
/// it. Filed as `name-words-budget-grew-under-placed-copy-wrapping`,
/// which D′3 closed (below).
///
/// The full total rose by 11 970 and its p99 by seven, and the scoped
/// total by 200, with a seam's sides and a crossing's edge and face named
/// as the boolean's own table names them (N3): at every subtract each
/// says the read it came in through, where it said the operand's entity
/// alone, so two operands holding the same names are told apart. The
/// longer sentences are the right names, not a regression. The maxima
/// and the scoped percentiles held.
///
/// Merging main at `be82e5ad6b` into D, the two moves met: full p99
/// 128 → 135, max 366 → 396, total 444393 → 458233; scoped p99 41 → 43,
/// total 59553 → 59743. Each side a subtract carries in now says its
/// read as well as the `Placed` copy it came through, and the longest
/// name says it at each of its ends; the growth is the same placed-copy
/// wrapping `name-words-budget-grew-under-placed-copy-wrapping` files.
///
/// D′3 retired that wrapping, so no name spells a placement: a copy
/// carries its source's names as they are, so full p99 135 → 122, max
/// 396 → 196 (main's before D's wrapping), total 461129 → 342994, and
/// scoped total 60241 → 46933. Scoped p99 grew 43 → 45: a product that
/// reads a `⊔` keys each copy's faces by the read they came through
/// (`From { read }`), so a face scoped to the product says that read,
/// where the gather's table said the copy's wrapped name.
///
/// A pair boolean's `Borders` wall came to cite its parent as the
/// boolean's own table names it, the `From` it came in through: each
/// wall of `crossing_slots`' pieces now says its read ("… through
/// <read>"). Scoped p99 43 → 45, total 60241 → 60371; full total
/// 461129 → 461339. No outcome moved.
///
/// An orientation is read, never chosen: a union's seam in a run says
/// its sense where it said "runs through", and a line of several things
/// says the things it enters along ("…, entering along …, crosses …").
/// Full p99 135 → 137, total 461339 → 461547; scoped faces held. No
/// outcome moved.
///
/// 461129 → 461339. No outcome moved. Main's merge into D′3 at
/// `1235f4c050` (these walls and stage 4 I) moves D′3's scoped total
/// 46933 → 47003 and full total 342994 → 343104; the percentiles held.
///
/// Measured again for D′3 over main's merge at `4c341b9225`, which
/// brings the run line's bare sense (above) beneath D′3's names: full
/// total 343104 → 343208, the percentiles and scoped faces held. No
/// outcome moved.
const NAME_WORDS: [(&str, [usize; 4]); 2] = [
    ("scoped faces", [16, 43, 57, 46_335]),
    ("full", [19, 121, 196, 332_976]),
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
/// INTENT stage 2 PR E, merged over stage 5 A: 6558 of 46614 words
/// moved, every one a node tag — with each tag masked the two word
/// lists are equal. A blend, shell, face frame or measure now reads a
/// selection its insert mints, so its id and every id minted after it
/// in the seven documents that hold one moved.
///
/// The three boolean nodes moved it with [`NAME_WORDS`]: every member
/// says its join, and a carry said by tag says the read it came through.
///
/// `part_select`'s union reading the split's ports moved it with
/// [`NAME_WORDS`]: its members are said as the ports, and the nodes
/// minted after the two `Part`s it no longer holds took new tags.
///
/// INTENT stage 3 PR A: 5760 of 46614 words moved, every one a node
/// tag — with each tag masked the two word lists are equal. A revolve
/// writes its axis on itself rather than reading an in-plane axis
/// datum, so every id minted after that datum in the revolving
/// documents moved. (Counted on its own tree; re-taken merged over
/// FORK-DM4 unit 1, PR 4527, and the split-half retirement, PR 4618.)
///
/// INTENT stage 4 PR F: a boolean node lost its `declare` field, so the
/// nodes minted after the first boolean took new tags.
///
/// The killed-lineage chase moved it with [`NAME_WORDS`]: in
/// `crossing_slots`, a rim piece named as a seam, two floor corners and
/// the pieces ending at them say other words. No other document's words
/// moved (`lib_g16_corpus_name_digests` moved that row alone).
///
/// Re-pinned for the vertex run: the words that moved are an
/// `EdgeCrossing`'s, each line now "… the rest" where the pair said "…
/// the other body and … the first", in `corner_table` and
/// `crossing_slots`, the two documents whose pair booleans meet edge to
/// edge (`lib_g16_corpus_name_digests` moved those two rows alone).
///
/// The output stage's killed lineage moved it with [`NAME_WORDS`]: in
/// `crossing_slots`, a stretch of the first cavity's rim named as a
/// seam is a piece of that rim, the floor corner it ends at is a
/// crossing of two rims rather than a touch, and the pieces ending at
/// that corner say other words (`lib_g16_corpus_name_digests` moved
/// that row alone).
///
/// FORK-DM4 unit 2 (a name is scoped by the variable holding its body)
/// moved it with [`NAME_WORDS`], in `cut_cylinder`, `kitchen_sink` and
/// `part_select` alone, each holding a split: a split's names say no
/// half, and `part_select`'s union, which declares the rest between the
/// halves by the half-free section face, took a new tag. Both totals
/// shrank. Re-taken merged over main at `8612ce48e8`, and again over
/// INTENT stage 3 B (#5232), whose placement kind word moved: the merged
/// digest and the full total are neither tree's.
///
/// INTENT stage 3 A′ (a pose is moved by lengths and angles) moved it in
/// `kitchen_sink` alone, by ids: a revolve holds one slot fewer and a
/// lone `0` is the constant, so the mint chain re-draws. No total moved.
///
/// INTENT stage 3 C (a mate relates two poses) moved it in every
/// document, by tags: each places its copy where it stands, a loose
/// `Place` and then the named interim's world pin, where one edit minted
/// the placement holding its world pin, so the placement's tag moved
/// and every tag minted after it. [`NAME_WORDS`] held: no name says a
/// word more or fewer.
///
/// A seam citing the parent face (N3) moved it with [`NAME_WORDS`], in
/// `crossing_slots` alone.
///
/// INTENT stage 3 D (Transform retires into a placement) moved it in
/// every document, with [`NAME_WORDS`]: the documents were re-spelled
/// without `Datum` or `Transform`, so every tag moved, and every
/// operand placed against a first operand's root says that placement.
///
/// The kitchen sink's lone block hung below the world's xy, as its
/// retired `SetExtrudeSide` edit had it, moved it in `kitchen_sink`
/// alone, by tags: its copies are placed at a pose, which mints one id
/// fewer than standing did, so every tag minted after them moved.
/// [`NAME_WORDS`] held.
///
/// The identity bundle (FORK-REST) moved it by tags, in the documents
/// that mint a node after a stand (`heat_sink`, `declared_tangency`,
/// `kitchen_sink`, `cut_cylinder`, `part_select`, the five rows
/// `lib_g16_corpus_name_digests` moved): a copy standing where its body
/// stands is placed by no mate and the constant `0` on every freedom,
/// written as one `AddValues`, which mints a different number of ids from the plane
/// mate and three values it replaces, so every tag minted after a stand
/// moved. [`NAME_WORDS`] held.
///
/// Exact turns (INTENT stage 3 D) moved it by tags in the five documents
/// `lib_g16_corpus_name_digests` moved with them: `die_tool` writes its
/// ball's spin as the constant `0` (one variable fewer); the pips of
/// `die_pips`, `die_composed` and the tour die stand on lines stated
/// from the cube's corner; `kitchen_sink`'s revolve stands upright by a
/// bundle with no mate and its split's plane and pattern's axis are read
/// off its faces. [`NAME_WORDS`] held.
///
/// It moved again by tags in `die_tool` alone, when its master ball came
/// to stand on the cube's corner frame by an axis mate, which mints 42
/// fewer ids than the retired place-against helper it replaces. [`NAME_WORDS`] held.
///
/// A seam's sides and a crossing's lines named in the boolean's own
/// table (N3) moved it with [`NAME_WORDS`], in every document holding a
/// subtract. Measured with [`NAME_WORDS`] merging main at `be82e5ad6b`
/// into D.
///
/// Re-pinned, `die` alone, when its pips came to stand on frames stated
/// from the cube's corner by whole quarter turns and dyadic slides (a
/// frame mate each, `fixture::square_frame`), where the retired place-against helper had
/// written `atan2`'s rounded turns: the die's world vertices are the
/// merge base's (`f79829b57d`) bit for bit, 176 of 176, and its
/// placements, their ids and the names under them are new.
///
/// D′1 (a placement names its target space): the words that moved are
/// the node tags, every construction minting its space beside its body
/// and every bundle written whole. With the tags masked, every name of
/// every document says the words it said on the merge base
/// (`e605bc0e`), and [`NAME_WORDS`] held: a read of a construction's
/// body is said by the node alone, its space being no part of what it
/// holds.
///
/// **Re-pinned for PR 5616**, taken merged over D′2 (PR 5613), which
/// left this row red on main: D′2 moved every corpus document's ids,
/// and this PR's `test_support::plane_on` reads `symmetric_plate`'s
/// seam face plainly, so its node tags moved too; the words that moved
/// are the node tags.
///
/// Re-pinned for D′3: no name says a placement layer, a
/// product's names say the `⊔` read they came through, and node tags
/// moved with the ids.
///
/// Re-pinned with [`NAME_WORDS`] when a pair boolean's `Borders` walls
/// came to say the read each came in through: the words that moved are
/// those walls'.
///
/// Re-pinned with [`NAME_WORDS`] when an orientation came to be read,
/// never chosen: the words that moved are the senses of a union's seams
/// in a run and of lines of several things, which say what they enter
/// along.
///
/// Measured again for D′3 over main's merge at `1235f4c050`, with the
/// `Borders` walls' reads under D′3's names.
///
/// Measured again with [`NAME_WORDS`] over main's merge at `4c341b9225`.
const SAID_DIGEST: u64 = 0x725d_81bf_b29f_a0a9;

/// The tables an evaluation answers for a name it does not hold: a
/// vanished name is in no table of the run that refuses it, and a
/// deleted node has none.
struct Gone;

impl NameTables for Gone {
    fn tables(&self, _: RecipeNodeId) -> Vec<&NameTable> {
        Vec::new()
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
    // Each output's table on its own (NAMES N1: a split's halves are two
    // tables, a name in both said once in each).
    for (id, table) in doc.ids().into_iter().flat_map(|id| {
        crate::fixture::tables(ev, id)
            .into_iter()
            .map(move |t| (id, t))
    }) {
        let mut groups: [BTreeMap<String, usize>; 3] = Default::default();
        for (name, _) in table.iter() {
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
    let (doc, ev, fillet, var, at, edge) = docs
        .iter()
        .zip(evals)
        .find_map(|(d, ev)| {
            d.doc.ids().iter().find_map(|&id| match d.doc.node(id) {
                Some(Node::Fillet { selection, .. }) => {
                    let names = &d.doc.selection(*selection)?.names;
                    (names.len() > 1).then(|| {
                        (
                            &d.doc,
                            ev,
                            id,
                            *selection,
                            names.len() - 1,
                            names.last().cloned(),
                        )
                    })
                }
                _ => None,
            })
        })
        .expect("the corpus fillets more than one edge");
    let edge = edge.expect("a last edge");
    let stranded = NodeError {
        node: fillet,
        kind: NodeErrorKind::SelectResolve {
            slot: editor_core::OperandSlot::Selection,
            var,
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
    vec![("NodeErrorKind::SelectResolve", stranded)]
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
                predicate: "sel_plane_distance",
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
    })
}

/// A copy of `input` slid `by` in `target`'s own coordinates.
fn moved(
    r: &mut Recorder,
    target: RecipeNodeId,
    input: RecipeNodeId,
    by: [f64; 3],
) -> RecipeNodeId {
    r.against(target, input, &editor_core::Frame::translation(by))
}

fn boolean(r: &mut Recorder, op: BooleanOp, a: RecipeNodeId, b: RecipeNodeId) -> RecipeNodeId {
    r.insert(match op {
        BooleanOp::Subtract => Node::Subtract {
            from: a.into(),
            tool: b.into(),
        },
        BooleanOp::Union => Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
        BooleanOp::Intersect => Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
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

/// The height a pin's base stands at on its block.
const PIN_Z: f64 = 0.75;

/// A block and a square pin's master, extruded once on its own xy; its
/// copies stand `PIN_Z` up the block.
fn block_and_pin(r: &mut Recorder) -> (RecipeNodeId, RecipeNodeId) {
    let block = r.profile([0.0; 3], XY.0, XY.1, vec![square(2.0, 1.0, 2.0)]);
    let block = extrude(r, block, 1.0);
    let pin = r.profile([0.0; 3], XY.0, XY.1, vec![square(0.5, 0.5, 0.125)]);
    (block, extrude(r, pin, 0.5))
}

/// **Documents built to attack the joins read apart too**: copies of
/// one master carried through chained Subtracts and an n-ary Union,
/// then also unioned on as bosses; a grid of pins (one family over two indices) subtracted and
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
    let copies = [0.0, 0.75, 1.5, 2.25].map(|x| moved(&mut r, block, pin, [x, 0.0, PIN_Z]));
    let first = boolean(&mut r, BooleanOp::Subtract, block, copies[0]);
    let second = boolean(&mut r, BooleanOp::Subtract, first, copies[1]);
    let (doc, union) = fixture::union_over(r.doc.clone(), &copies[2..]);
    r.doc = doc;
    let cut = boolean(&mut r, BooleanOp::Subtract, second, union);
    docs.push(("copies".to_owned(), r.doc.clone()));
    let bosses =
        [[0.0, 1.0, PIN_Z + 0.5], [0.75, 1.0, PIN_Z + 0.5]].map(|at| moved(&mut r, block, pin, at));
    let once = boolean(&mut r, BooleanOp::Union, cut, bosses[0]);
    boolean(&mut r, BooleanOp::Union, once, bosses[1]);
    docs.push(("copies and bosses".to_owned(), r.doc.clone()));

    let mut r = Recorder::new();
    let (block, pin) = block_and_pin(&mut r);
    let pin = moved(&mut r, block, pin, [0.0, 0.0, PIN_Z]);
    // A grid of pins: `j` within `i`, one placement over both, and the
    // union of its family.
    let i = r.index("i", Formula::count(3));
    r.push(editor_core::DocEdit::DeclareVar {
        name: editor_core::VarName::from_static("j"),
        def: editor_core::VarDecl::Index {
            count: Formula::count(2),
            within: Some(Formula::var(i, editor_core::Dimension::Count)),
        },
    });
    let j = r.doc.var_named("j").expect("the inner index");
    let slides = [
        fixture::slides_along(i, [1.0, 0.0, 0.0], 0.75),
        fixture::slides_along(j, [0.0, 1.0, 0.0], 1.0),
    ]
    .concat();
    let pins = r.copies(pin, slides);
    let grid = r.insert(Node::Union {
        members: editor_core::Bodies::Family(pins.into()),
    });
    let cut = boolean(&mut r, BooleanOp::Subtract, block, grid);
    let tool = fixture::plane_on(&r.doc, block, [0.0, 0.5, 0.0], [0.0, 1.0, 0.0]);
    r.insert(Node::Split {
        target: cut.into(),
        tool,
    });
    docs.push(("family and split".to_owned(), r.doc.clone()));

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
    let upper = r.profile([0.0; 3], XY.0, XY.1, loops(0.1));
    let upper = extrude(&mut r, upper, 0.5);
    let raised = moved(&mut r, lower, upper, [0.0, 0.0, 0.25]);
    boolean(&mut r, BooleanOp::Union, lower, raised);
    let apart =
        [(lower, 0.0), (upper, 0.25)].map(|(at, z)| moved(&mut r, lower, at, [6.0, 0.0, z]));
    let (doc, _) = fixture::union_over(r.doc.clone(), &[apart[0], apart[1], lower]);
    docs.push(("loops and two profiles".to_owned(), doc));

    for op in [BooleanOp::Subtract, BooleanOp::Union] {
        let mut r = Recorder::new();
        let block = r.profile([0.0; 3], XY.0, XY.1, vec![square(2.0, 1.0, 2.0)]);
        let block = extrude(&mut r, block, 1.0);
        let pin = r.insert(Node::Profile(editor_core::ProfileProgram {
            loops: vec![editor_core::LoopProgram::circle(0.5, 0.5, 0.2).expect("a circle")],
            ids: Vec::new(),
            on: None,
        }));
        let pin = extrude(&mut r, pin, 0.5);
        let copies = [0.0, 1.0, 2.0].map(|x| moved(&mut r, block, pin, [x, 0.0, PIN_Z]));
        let first = boolean(&mut r, op, block, copies[0]);
        let second = boolean(&mut r, op, first, copies[1]);
        let third = boolean(&mut r, op, second, copies[2]);
        let ev = fixture::run(&r.doc, &EvalOptions::default());
        let rim: Vec<StableName> = ev
            .table_at(third)
            .unwrap_or_else(|| panic!("the round pins evaluate: {:?}", corpus::failures(&ev)))
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
        let twice = [[0.0; 3], [0.0, 5.5, 0.0]].map(|at| moved(&mut r, fillet, fillet, at));
        let (doc, _) = fixture::union_over(r.doc.clone(), &twice);
        docs.push((format!("round pins, {op:?}"), doc));
    }

    docs.push(("glued rim cut by a third block".to_owned(), glued_rim().0));

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

/// The three-block glued rim (`fixture::glued_rim`, glued flush) as
/// one document, and its union.
fn glued_rim() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("name_words_glued_rim", geom_core::Tol::witness());
    let (doc, members) = fixture::glued_rim(0.0)(doc);
    fixture::union_over(doc, &members)
}

/// **Each seam in a run of seams is said by its two faces**, in full
/// and within the evaluation alike: the three-block glued rim, where
/// `c`'s wall cuts the rim of `b` glued on `a`, holds two vertices that
/// are each the run of three seams and share the seam of `a`'s top and
/// `c`'s wall. Said within the evaluation, a seam said as "an edge"
/// says nothing of where the vertex is, and two such runs differ only
/// in the seams the detail opened.
#[test]
fn a_run_of_seams_says_each_seam_by_its_faces() {
    let (doc, u) = glued_rim();
    let ev = fixture::run(&doc, &EvalOptions::default());
    let full = Speaker::of(&doc);
    let within = full.within(&ev);
    let runs: Vec<StableName> = fixture::table(&ev, u)
        .iter()
        .map(|(name, _)| name)
        .filter(|name| {
            matches!(name.path.as_slice(), [RoleSeg::EdgeCrossing(run)]
                if run.len() == 3
                    && run.iter().all(|l| matches!(l.line.path.as_slice(), [RoleSeg::Seam { .. }])))
        })
        .cloned()
        .collect();
    assert_eq!(
        runs.len(),
        2,
        "two vertices are runs of three seams: {runs:?}"
    );
    for (form, by) in [("in full", full), ("within the evaluation", within)] {
        let said: Vec<String> = runs.iter().map(|n| by.name(n).to_string()).collect();
        for s in &said {
            assert_eq!(
                s.matches("the seam of ").count(),
                3,
                "{form}: each seam is said by its faces: {s}"
            );
            assert!(
                !s.contains("an edge"),
                "{form}: no seam is said as an edge: {s}"
            );
        }
        assert_ne!(said[0], said[1], "{form}: the two runs read apart");
    }
}

/// **A kept name spoken again after its step was dropped says the step
/// by its tag**, never the row it sat at, which is now another step's;
/// a kept step that only moved is said at its new row. Either way the
/// name reads as it would spoken fresh from the new document.
#[test]
fn respoken_after_a_dropped_step() {
    use crate::corpus::reshaped_rod::{CREASE, bump_ids, lateral_edge, rod_ids, rod_loop};
    use editor_core::{
        DocEdit, Node, ProfileDoc, ProfileProgram, ProfileVertexRef, RoleSeg, apply,
    };

    let tol = fixture::tol();
    let doc = ProfileDoc::empty_derived("respoken-after-a-dropped-step", tol);
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![rod_loop(false)],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, rod) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: fixture::len(2.0),
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
        let pin = moved(&mut r, block, pin, [0.0, 0.0, PIN_Z]);
        let at = boolean(&mut r, op, block, pin);
        let ev = fixture::run(&r.doc, &EvalOptions::default());
        let table = ev
            .table_at(at)
            .unwrap_or_else(|| panic!("{op:?} evaluates: {:?}", corpus::failures(&ev)));
        let by = Speaker::of(&r.doc);
        let pin_read = fixture::out(&r.doc, pin);
        // A member's join also says the read it came in through; a
        // subtract's tool is its one cut-in read, said by the verb alone.
        let from = match op {
            BooleanOp::Subtract => String::new(),
            BooleanOp::Union | BooleanOp::Intersect => {
                format!(" from Place {}", test_utils::refusal::tag(pin.0.digest()))
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
/// node that made it and not the node holding it, so the pick's tie
/// says each face's node.
#[test]
fn two_copies_of_one_body_read_apart_where_a_sentence_names_both() {
    let mut r = Recorder::new();
    let block = r.profile([0.0; 3], XY.0, XY.1, vec![square(0.5, 0.5, 0.5)]);
    let block = extrude(&mut r, block, 1.0);
    let one = moved(&mut r, block, block, [0.0, 0.0, 0.0]);
    let two = moved(&mut r, block, block, [0.5, 0.0, 0.0]);
    let doc = r.doc;
    let ev = fixture::run(&doc, &EvalOptions::default());
    // The copies' end caps carry one name: the extrude's.
    let face = fixture::fname(block, editor_core::RoleSeg::Cap(editor_core::CapEnd::End));
    let by = Speaker::of(&doc).within(&ev);
    let (at_one, at_two) = (doc.spoken(one).to_string(), doc.spoken(two).to_string());
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
    assert!(
        tie.contains(&format!("{said} on {at_one}"))
            && tie.contains(&format!("{said} on {at_two}")),
        "each face is said with the copy holding it: {tie}"
    );
}

/// How many faces `node`'s table names that `node` itself heads.
fn own_faces(ev: &editor_core::Evaluation<f64>, node: editor_core::RecipeNodeId) -> usize {
    ev.table_at(node).map_or(0, |t| {
        t.iter()
            .filter(|(name, _)| name.node == node && name.kind == EntityKind::Face)
            .count()
    })
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
                .max_by_key(|&id| own_faces(&ev, id))?;
            Some((doc, ev, node))
        })
        // The largest table of faces a node names itself: a copy's
        // table is its source's names (D′3), which it does not head.
        .max_by_key(|(_, ev, node)| own_faces(ev, *node))
        .expect("the corpus evaluates");
    let faces: Vec<StableName> = ev
        .table_at(node)
        .expect("the largest table's node")
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
        ev.table_at(node).map_or(0, |t| t.iter().count())
    );
    assert!(
        again * 4 < first,
        "a later saying searches the table again: the first took {first:?}, each later one \
         {again:?}"
    );
}
