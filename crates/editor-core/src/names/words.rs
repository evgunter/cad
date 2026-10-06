//! **A name in words** — what a person reads to tell one entity from
//! another (`work/recipe/names-render-a-faces-leaf-role-in-words.md`,
//! ruled on #3571 and #3906).
//!
//! The path as a structure is the machine channel; a person reads it
//! as one sentence, `<role> of <feature>[, <join>…]`:
//!
//! - **The role** is the leaf's: the entity the author made, reached by
//!   looking through every segment that carries an operand's entity on
//!   ([`origin`]'s `Carried`). A carry that changed the entity — a
//!   fragment qualifier, a split's half, a pattern's copy, a band's cut
//!   — is said around it: "the part of the end cap of Extrude e548
//!   bordering …", "instance 2's copy of …".
//! - **The feature** is the node that made the leaf, said right after
//!   the leaf's own words, so a name a role cites keeps its feature:
//!   "the blend face over the end rim edge over loop 0 step 2 of
//!   Extrude e548 of Fillet 92b0".
//! - **A join** is a carry through a secondary operand — a Boolean's B,
//!   a union's member — said with its node, outermost first. A Boolean
//!   is said by its operation: "…, cut in at Subtract 1669", "…, joined
//!   at Union 2b40", "…, intersected at Intersect 77c1"; a union by its
//!   member: "…, joined at Union d1aa from Transform 3218". By tag, or
//!   where the document does not hold the Boolean, a B join says what
//!   the name holds: "…, through operand B of node 1669". A carry
//!   through a primary operand (a Boolean's A, a fillet's target) is the
//!   body's own continuation and is silent. Two names of one table first
//!   differ at a node where one went through a secondary operand, which
//!   a join says.
//! - **Wraps and joins are said in the order the path takes them.** A
//!   join beneath a wrap is said inside it, in brackets: "instance 1's
//!   copy of (the start cap of Extrude e548, cut in at Subtract 1669)"
//!   is a copy of the cut-in cap, where "instance 1's copy of the start
//!   cap of Extrude e548, cut in at Subtract 1669" is a cut-in copy.
//! - **A cited name whose words run on past its node** — a qualifier's
//!   list, a join — is said in brackets: "the part of the start cap of
//!   Extrude e548 bordering (the part of the side wall over loop 0 step
//!   2 of Extrude e548 bordering 2 faces) and the side wall over loop 0
//!   step 3 of Extrude e548". Every other cited name ends at its node,
//!   so no "and" or join the citing sentence says after a citation reads
//!   as more of it.
//!
//! **The full form says two names alike only where they differ in a
//! node it never says**: the node of a primary carry (silent above), or
//! of a split's, a copy's, a band cut's or a part's carry, which the
//! wrap's words leave unsaid. Every other difference between two names
//! is in their words.
//!
//! **Which node's output holds the entity is not the name's to say**:
//! two copies of one body hold names alike, and the sentence that says
//! a name says the node it is held at where that is not already fixed
//! (a pick hit's node, a flush query's two nodes).
//!
//! **How much of a cited name is said is the speaker's [`Detail`].**
//! The full form says every cited name and every list member in full,
//! however deep. A speaker holding the name table the name was read
//! from ([`Speaker::within`]) says the detail that table gives it, so
//! no two of its names read alike ([`table_details`]); one holding none
//! says the full form. Past the detail's depth a cited name is said by
//! its kind ("a face"), and past its width a list says how many more.
//!
//! **Every number is counted from zero**, as the profile pane numbers
//! a loop and its steps: `loop 0 step 2`, `instance 0's copy`, `part 1
//! of 3`. A profile piece is a "piece" (`piece 1 of loop 0 step 0`); a
//! cut of a face, edge or body is a "part". A leg is its step's only
//! piece, so the step alone says it (`the side wall over loop 0 step
//! 2`).
//!
//! A profile step is said as the pane numbers it wherever the speaker's
//! document holds it, and with its profile unless the feature reads
//! that profile alone; by its tag (`the profile step <tag>`) where no
//! document is at hand.
//!
//! The sentence is built from an explicit stack, never the call stack,
//! so a name nested past every thread's stack renders.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::names::NameTable;
use crate::names::attribute::{CarriedAs, SegOrigin, origin};
use crate::names::role::{
    CapEnd, EntityKind, MeridianEnd, NameRef, PieceRun, ProfileEdgeRef, ProfileVertexRef,
    Qualifier, RimSupport, RoleSeg, SectionCircle, SplitHalf, StableName, fragment_tail_start,
};
use crate::node::{BooleanOp, RecipeNodeId};
use crate::spoken::{Said, Say, Speaker};
use profile::PieceRole;

/// **A name in words** ([module docs](self)): `the end cap of Extrude
/// e548`, `the part above the split of the side wall over loop 0 step 2
/// of Extrude e548`. Article-led, so a
/// sentence takes it as a noun phrase.
///
/// Its `Display` says each node and profile step by its tag, in full;
/// said by a [`Speaker`] ([`Said`]), as that speaker's document holds
/// them, at that speaker's detail. The name's own `Display` is this.
#[derive(Clone, Copy)]
pub struct LeafRole<'a>(pub &'a StableName);

/// **`name` in words** ([`LeafRole`]).
#[must_use]
pub fn leaf_role(name: &StableName) -> LeafRole<'_> {
    LeafRole(name)
}

impl Say for LeafRole<'_> {
    fn say(&self, f: &mut fmt::Formatter<'_>, by: Speaker<'_>) -> fmt::Result {
        f.write_str(&words(self.0, by, &by.detail_of(self.0)))
    }
}

impl fmt::Display for LeafRole<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Said(self, Speaker::TAG).fmt(f)
    }
}

/// **The name whose role a [`LeafRole`] says**: `name` with every
/// carrying segment looked through ([`origin`]) and every fragment
/// qualifier dropped. Its node is the feature the sentence says, the
/// one that minted the entity ([`super::attribute`]'s answer, where
/// that walk finds one).
#[must_use]
pub fn role_leaf(name: &StableName) -> &StableName {
    walk(name).leaf
}

/// Where a cited name sits in a sentence: the index of each citation on
/// the way down from the name said, the name itself at `[]`. Numbered by
/// the name's structure, so one position means the same citation at
/// every detail.
pub(crate) type Pos = Vec<u16>;

/// **How much of the names a name cites is said.** The name's own role,
/// feature and joins are said at every detail; a cited name is said in
/// full where the detail opens its position, and by its kind ("a face")
/// where it does not. A list of cited names says those it opens and how
/// many more, or how many of what kind where it opens none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Detail {
    /// Every citation opened, its run-on citations bracketed: the form
    /// two distinct names share only where they differ in a carry's
    /// node it never says ([module docs](self)).
    Full,
    /// These citations opened, and no others.
    Open(BTreeSet<Pos>),
}

impl Detail {
    fn opens(&self, pos: &[u16]) -> bool {
        pos.is_empty()
            || match self {
                Self::Full => true,
                Self::Open(open) => open.contains(pos),
            }
    }
}

/// `name` in words at `detail`, each node and step said by `by`.
pub(crate) fn words(name: &StableName, by: Speaker<'_>, detail: &Detail) -> String {
    let mut out = String::new();
    say(name, by, detail, |text| out.push_str(text), |_| ());
    out
}

/// **The detail each name of `table` is said at within it**: what
/// [`Speaker::within`] says, so that no two names of the table read
/// alike as each is said.
///
/// A name no other reads alike with at no citation opened is said so.
/// Each other name is searched greedily against the names it reads
/// alike with there: one citation at a time is opened — the one that
/// leaves the fewest of them reading alike, the nearest among equals —
/// until none does, then each opening it can is shut again while that
/// still holds. Where opening every citation still leaves one alike,
/// the full form.
///
/// Then every name is said at its own detail, and each group of names
/// whose words are alike — each at the detail it chose, which no search
/// above compared — is said in full, until no two read alike or every
/// name still alike is said in full. A detail is unique in the table,
/// not the fewest openings.
///
/// **Where the loop ends with names alike**: two names of the table
/// that differ only in a carry's node the full form never says
/// ([`Detail::Full`]) — two merged faces, say, each citing the end cap
/// carried through the A of a different Subtract. A table of one
/// node's output first differs at a node a join says (module docs), so
/// no evaluation is known to mint such a pair; a debug build asserts
/// none does.
///
/// **Cost.** Each name of a group alike at no citation is said once per
/// detail some search of the group asks of it ([`Alike`]), and the
/// searches share those sayings: a detail one search tries is mostly
/// one every search of the group tries, since the group's names cite
/// alike. What grows with the group is comparing each name's words with
/// its rivals', quadratic in the group's size by string comparison.
///
/// Worked out by tag: a speaker holding a document says each node and
/// step it names in a spelling of its own for each, so names that read
/// apart by tag read apart said by it.
pub(crate) fn table_details(table: &NameTable) -> BTreeMap<NameRef, Detail> {
    let by = Speaker::TAG;
    let names: Vec<&NameRef> = table.iter_refs().map(|(name, _)| name).collect();
    let bare = Detail::Open(BTreeSet::new());
    let mut alike: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, name) in names.iter().enumerate() {
        alike.entry(words(name, by, &bare)).or_default().push(i);
    }
    let mut details = vec![bare; names.len()];
    for group in alike.values().filter(|group| group.len() > 1) {
        let mut said = Alike::new(group.iter().map(|&i| &**names[i]).collect());
        for (k, &i) in group.iter().enumerate() {
            details[i] = said.apart(k);
        }
    }
    let mut said: Vec<String> = names
        .iter()
        .zip(&details)
        .map(|(name, detail)| words(name, by, detail))
        .collect();
    loop {
        let mut groups: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
        for (i, words) in said.iter().enumerate() {
            groups.entry(words).or_default().push(i);
        }
        let opened: Vec<usize> = groups
            .values()
            .filter(|group| group.len() > 1)
            .flatten()
            .copied()
            .filter(|&i| details[i] != Detail::Full)
            .collect();
        if opened.is_empty() {
            debug_assert!(
                groups.values().all(|group| group.len() < 2),
                "names of one table alike in full, differing only in a carry's node the full \
                 form never says: {:?}",
                groups.iter().find(|(_, group)| group.len() > 1)
            );
            break;
        }
        for i in opened {
            details[i] = Detail::Full;
            said[i] = words(names[i], by, &Detail::Full);
        }
    }
    names.into_iter().cloned().zip(details).collect()
}

/// **One group of names alike at no citation, and each said at every
/// detail a search asked of it**, so that the group's searches
/// ([`Alike::apart`]) say each name at each detail once between them.
struct Alike<'n> {
    names: Vec<&'n StableName>,
    /// Each name's words, by the openings said and the name's place in
    /// the group.
    said: BTreeMap<BTreeSet<Pos>, Vec<Option<String>>>,
}

impl<'n> Alike<'n> {
    fn new(names: Vec<&'n StableName>) -> Self {
        Self {
            names,
            said: BTreeMap::new(),
        }
    }

    /// The words of the group's `i`th name with `open` opened.
    fn words(&mut self, open: &BTreeSet<Pos>, i: usize) -> &str {
        let len = self.names.len();
        let row = self
            .said
            .entry(open.clone())
            .or_insert_with(|| vec![None; len]);
        let name = self.names[i];
        row[i].get_or_insert_with(|| words(name, Speaker::TAG, &Detail::Open(open.clone())))
    }

    /// How many of `among` read alike with the `i`th name at `open`.
    fn count(&mut self, open: &BTreeSet<Pos>, i: usize, among: &[usize]) -> usize {
        let mine = self.words(open, i).to_owned();
        among
            .iter()
            .filter(|&&j| self.words(open, j) == mine)
            .count()
    }

    /// A detail at which the group's `i`th name reads apart from each
    /// other name of the group, each said at that same detail, found
    /// greedily ([`table_details`]).
    fn apart(&mut self, i: usize) -> Detail {
        let rivals: Vec<usize> = (0..self.names.len()).filter(|&j| j != i).collect();
        let mut open = BTreeSet::new();
        let mut left = rivals.clone();
        let mut shut = cited_positions(self.names[i], Speaker::TAG);
        loop {
            let mine = self.words(&open, i).to_owned();
            left.retain(|&j| self.words(&open, j) == mine);
            if left.is_empty() {
                // An opening can make alike a rival an earlier detail
                // told apart ("2 faces" and "2 entities" both open to
                // "the side wall … and 1 more"), so every rival is asked
                // again.
                left = rivals
                    .iter()
                    .copied()
                    .filter(|&j| self.words(&open, j) == mine)
                    .collect();
                if left.is_empty() {
                    break;
                }
            }
            // The citations said now: those whose citing name is open.
            let detail = Detail::Open(open.clone());
            let sayable: Vec<usize> = (0..shut.len())
                .filter(|&k| detail.opens(&shut[k][..shut[k].len() - 1]))
                .collect();
            let Some(pick) = sayable.iter().copied().min_by_key(|&k| {
                let mut tried = open.clone();
                tried.insert(shut[k].clone());
                self.count(&tried, i, &left)
            }) else {
                return Detail::Full;
            };
            open.insert(shut.remove(pick));
        }
        // An opening a later one made needless is shut again, with every
        // opening beneath it, while the name still reads apart; in
        // reverse order, so an opening is tried before the one it sits
        // beneath.
        for pos in open.clone().iter().rev() {
            let mut fewer = open.clone();
            fewer.retain(|kept| !kept.starts_with(pos));
            if self.count(&fewer, i, &rivals) == 0 {
                open = fewer;
            }
        }
        Detail::Open(open)
    }
}

/// Every citation of `name`'s full form, nearest first.
fn cited_positions(name: &StableName, by: Speaker<'_>) -> Vec<Pos> {
    let mut all = Vec::new();
    say(
        name,
        by,
        &Detail::Full,
        |_| (),
        |pos| {
            if !pos.is_empty() {
                all.push(pos.to_vec());
            }
        },
    );
    all.sort_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)));
    all
}

/// The sentence, from an explicit stack: each piece of text to `text`,
/// each name it says to `at` by position.
fn say(
    name: &StableName,
    by: Speaker<'_>,
    detail: &Detail,
    mut text: impl FnMut(&str),
    mut at: impl FnMut(&[u16]),
) {
    let mut stack = vec![Item::Name(name, Pos::new(), by)];
    while let Some(item) = stack.pop() {
        match item {
            Item::Text(words) => text(&words),
            Item::Name(name, pos, by) => {
                at(&pos);
                stack.extend(expand(name, &pos, by, detail).into_iter().rev());
            }
        }
    }
}

/// One piece of a sentence under construction: words, or a name to say
/// at its position.
enum Item<'n, 's> {
    Text(String),
    Name(&'n StableName, Pos, Speaker<'s>),
}

fn text<'n, 's>(s: impl Into<String>) -> Item<'n, 's> {
    Item::Text(s.into())
}

/// A name said by its kind alone: `a face`.
fn kind_np(kind: EntityKind) -> String {
    format!("{} {}", kind.article(), kind.noun())
}

/// The names one level of a sentence cites, numbered in the order it
/// cites them.
struct Cites<'p, 'd, 's> {
    pos: &'p [u16],
    next: u16,
    detail: &'d Detail,
    by: Speaker<'s>,
}

impl<'s> Cites<'_, '_, 's> {
    fn at(&mut self) -> Pos {
        let mut pos = self.pos.to_vec();
        pos.push(self.next);
        self.next = self.next.saturating_add(1);
        pos
    }

    /// One cited name.
    fn one<'n>(&mut self, name: &'n StableName) -> Item<'n, 's> {
        Item::Name(name, self.at(), self.by)
    }

    /// One cited name, said by tag.
    fn by_tag<'n>(&mut self, name: &'n StableName) -> Item<'n, 's> {
        Item::Name(name, self.at(), Speaker::TAG)
    }

    /// A list of cited names: those opened in full, then how many more;
    /// where none is opened, how many of what kind.
    fn list<'n>(&mut self, names: &'n [StableName]) -> Vec<Item<'n, 's>> {
        let Some(first) = names.first() else {
            return vec![text("nothing")];
        };
        let at: Vec<Pos> = names.iter().map(|_| self.at()).collect();
        let opened: Vec<(&'n StableName, Pos)> = names
            .iter()
            .zip(at)
            .filter(|(_, pos)| self.detail.opens(pos))
            .collect();
        if opened.is_empty() {
            if let [one] = names {
                return vec![text(kind_np(one.kind))];
            }
            let plural = if names.iter().all(|n| n.kind == first.kind) {
                plural(first.kind)
            } else {
                "entities"
            };
            return vec![text(format!("{} {plural}", names.len()))];
        }
        let rest = names.len() - opened.len();
        let said = opened.len();
        let mut items = Vec::new();
        for (i, (name, pos)) in opened.into_iter().enumerate() {
            if i > 0 {
                items.push(text(if i + 1 == said && rest == 0 {
                    " and "
                } else {
                    ", "
                }));
            }
            items.push(Item::Name(name, pos, self.by));
        }
        if rest > 0 {
            items.push(text(format!(" and {rest} more")));
        }
        items
    }
}

/// A carry the walk looked through that changed the entity, said around
/// the leaf.
enum Wrap<'n> {
    Part(&'n Qualifier),
    Split(SplitHalf),
    ToolCopy(SplitHalf),
    Instance(u32),
    Cut,
}

/// A carry through a secondary operand, at the node that carried it.
enum Join {
    B(RecipeNodeId),
    Member {
        union: RecipeNodeId,
        member: RecipeNodeId,
    },
}

/// A carry the walk looked through and says: around the leaf, or as a
/// join.
enum Step<'n> {
    Wrap(Wrap<'n>),
    Join(Join),
}

/// A name, its carrying segments and qualifiers looked through.
struct Walk<'n> {
    /// The carries said, outermost first.
    steps: Vec<Step<'n>>,
    /// The name the walk stopped at.
    leaf: &'n StableName,
}

fn walk(name: &StableName) -> Walk<'_> {
    let mut steps = Vec::new();
    let mut at = name;
    loop {
        let tail = fragment_tail_start(&at.path);
        // `[parent, Fragment(q1), Fragment(q2)]` is the q2 part of the
        // q1 part: the last qualifier is the outermost.
        steps.extend(at.path[tail..].iter().rev().filter_map(|seg| match seg {
            RoleSeg::Fragment(q) => Some(Step::Wrap(Wrap::Part(q))),
            _ => None,
        }));
        let [seg] = &at.path[..tail] else {
            return Walk { steps, leaf: at };
        };
        let SegOrigin::Carried(of, carry) = origin(seg) else {
            return Walk { steps, leaf: at };
        };
        steps.extend(match carry {
            CarriedAs::Primary => None,
            CarriedAs::Secondary => Some(Step::Join(match seg {
                RoleSeg::FromMember { member, .. } => Join::Member {
                    union: at.node,
                    member: *member,
                },
                _ => Join::B(at.node),
            })),
            CarriedAs::Split(side) => Some(Step::Wrap(Wrap::Split(side))),
            CarriedAs::ToolCopy(side) => Some(Step::Wrap(Wrap::ToolCopy(side))),
            CarriedAs::Instance(i) => Some(Step::Wrap(Wrap::Instance(i))),
            CarriedAs::Cut => Some(Step::Wrap(Wrap::Cut)),
        });
        at = of;
    }
}

/// A walk's steps in levels, outermost first: each level's joins, said
/// after its words, and its wraps, said around the next level —
/// bracketed — or, at the last level, around the leaf.
type Level<'w, 'n> = (&'w [Step<'n>], &'w [Step<'n>]);

fn levels<'w, 'n>(steps: &'w [Step<'n>]) -> Vec<Level<'w, 'n>> {
    let mut levels = Vec::new();
    let mut rest = steps;
    loop {
        let joins = rest
            .iter()
            .take_while(|step| matches!(step, Step::Join(_)))
            .count();
        let (joins, after) = rest.split_at(joins);
        let wraps = after
            .iter()
            .take_while(|step| matches!(step, Step::Wrap(_)))
            .count();
        let (wraps, inner) = after.split_at(wraps);
        levels.push((joins, wraps));
        if inner.is_empty() {
            return levels;
        }
        rest = inner;
    }
}

fn wraps<'w, 'n>(steps: &'w [Step<'n>]) -> impl DoubleEndedIterator<Item = &'w Wrap<'n>> {
    steps.iter().filter_map(|step| match step {
        Step::Wrap(wrap) => Some(wrap),
        Step::Join(_) => None,
    })
}

/// The sentence of `name` at `pos`, one level: its words, and the names
/// it cites still to be said.
fn expand<'n, 's>(
    name: &'n StableName,
    pos: &[u16],
    by: Speaker<'s>,
    detail: &Detail,
) -> Vec<Item<'n, 's>> {
    if !detail.opens(pos) {
        return vec![text(kind_np(name.kind))];
    }
    let mut cites = Cites {
        pos,
        next: 0,
        detail,
        by,
    };
    let Walk { steps, leaf } = walk(name);
    let levels = levels(&steps);
    let last = levels.len() - 1;
    let mut items: Vec<Item<'n, 's>> = Vec::new();
    for (k, (_, level)) in levels.iter().enumerate() {
        items.extend(wraps(level).map(|w| text(prefix(w, name.kind))));
        if k < last {
            items.push(text("("));
        }
    }
    items.extend(head(leaf, by, &mut cites));
    items.push(text(format!(" of {}", by.node(leaf.node))));
    for (k, (joins, level)) in levels.iter().enumerate().rev() {
        if k < last {
            items.push(text(")"));
        }
        for wrap in wraps(level).rev() {
            if let Wrap::Part(q) = wrap {
                let (word, names) = match q {
                    Qualifier::OrderAlong { .. } => continue,
                    Qualifier::Borders(walls) => (" bordering ", walls),
                    Qualifier::Keeps(edges) => (" along ", edges),
                    Qualifier::Ends(ends) => (" between ", ends),
                };
                items.push(text(word));
                items.extend(cites.list(names));
            }
        }
        for step in *joins {
            if let Step::Join(join) = step {
                items.push(text(join_words(join, by)));
            }
        }
    }
    // A cited name whose words run on past its node — a qualifier's
    // list, a join — is bracketed, so nothing the citing sentence says
    // after it reads as more of it.
    let (joins, outer) = levels[0];
    let runs_on = !joins.is_empty()
        || wraps(outer).any(|wrap| {
            matches!(
                wrap,
                Wrap::Part(Qualifier::Borders(_) | Qualifier::Keeps(_) | Qualifier::Ends(_))
            )
        });
    if runs_on && !pos.is_empty() {
        items.insert(0, text("("));
        items.push(text(")"));
    }
    items
}

/// The words a join says after the name it carried.
fn join_words(join: &Join, by: Speaker<'_>) -> String {
    match *join {
        Join::B(at) => match by.boolean_op(at) {
            Some(op) => {
                let verb = match op {
                    BooleanOp::Subtract => "cut in",
                    BooleanOp::Union => "joined",
                    BooleanOp::Intersect => "intersected",
                };
                format!(
                    ", {verb} at {}",
                    by.node_as_kind(at, verbs::VerbKind::Boolean(op).noun())
                )
            }
            // Which operation is the document's to say: the name holds
            // only that it came in as operand B.
            None => format!(", through operand B of {}", by.node(at)),
        },
        Join::Member { union, member } => {
            format!(", joined at {} from {}", by.node(union), by.node(member))
        }
    }
}

/// The words a carry says before the leaf.
fn prefix(wrap: &Wrap<'_>, kind: EntityKind) -> String {
    match wrap {
        Wrap::Split(side) => format!("the part {} of ", half(*side)),
        Wrap::ToolCopy(side) => format!("the copy {} of ", half(*side)),
        Wrap::Instance(i) => format!("instance {i}'s copy of "),
        Wrap::Cut => "the surviving part of ".to_owned(),
        Wrap::Part(Qualifier::OrderAlong { rank, of }) => {
            let what = match kind {
                EntityKind::Vertex => "crossing",
                EntityKind::Face | EntityKind::Edge | EntityKind::Body => "part",
            };
            format!("{what} {rank} of {of} of ")
        }
        Wrap::Part(Qualifier::Borders(_) | Qualifier::Keeps(_) | Qualifier::Ends(_)) => {
            "the part of ".to_owned()
        }
    }
}

fn plural(kind: EntityKind) -> &'static str {
    match kind {
        EntityKind::Body => "bodies",
        EntityKind::Face => "faces",
        EntityKind::Edge => "edges",
        EntityKind::Vertex => "vertices",
    }
}

fn half(side: SplitHalf) -> &'static str {
    match side {
        SplitHalf::Above => "above the split",
        SplitHalf::Below => "below the split",
    }
}

fn cap(e: CapEnd) -> &'static str {
    match e {
        CapEnd::Start => "start",
        CapEnd::End => "end",
    }
}

fn meridian(e: MeridianEnd) -> &'static str {
    match e {
        MeridianEnd::Start => "start",
        MeridianEnd::End => "end",
        MeridianEnd::Seam => "seam",
        MeridianEnd::Pi => "half-turn",
    }
}

/// A profile piece's role as a noun phrase: an indexed piece by its
/// index (`piece 1`), any other by its article (`the leg`).
fn role_np(role: PieceRole) -> String {
    match role {
        PieceRole::Piece(_) => role.to_string(),
        PieceRole::Leg | PieceRole::RunIn | PieceRole::Arc | PieceRole::RunOut => {
            format!("the {role}")
        }
    }
}

/// A profile piece of `feature`'s profile: its role, and the step that
/// drew it as the profile pane numbers it — with the profile, unless
/// `feature` reads that profile alone — or, on a kernel-built section,
/// which circle. A leg is its step's only piece, so the step alone says
/// it (`loop 0 step 2`); a fillet's pieces say which (`the arc of loop
/// 0 step 2`).
fn piece(e: &ProfileEdgeRef, feature: RecipeNodeId, by: Speaker<'_>) -> String {
    match e {
        ProfileEdgeRef::Piece { step, role } => {
            let step = match by.step(*step) {
                Some(at) if by.sole_profile(feature) == Some(at.profile()) => at.to_string(),
                Some(at) => format!("{at} in {}", by.node(at.profile())),
                None => format!("the profile step {step}"),
            };
            match role {
                PieceRole::Leg => step,
                _ => format!("{} of {step}", role_np(*role)),
            }
        }
        ProfileEdgeRef::Section { circle, role } => format!(
            "{} of the {} circle",
            role_np(*role),
            match circle {
                SectionCircle::Outer => "outer",
                SectionCircle::Bore => "bore",
            }
        ),
    }
}

fn run(r: &PieceRun, feature: RecipeNodeId, by: Speaker<'_>) -> String {
    pieces(r.pieces().iter().map(|e| piece(e, feature, by)))
}

fn pieces(said: impl Iterator<Item = String>) -> String {
    said.collect::<Vec<_>>().join(", then ")
}

fn vertex(v: &ProfileVertexRef, feature: RecipeNodeId, by: Speaker<'_>) -> String {
    let edge = match *v {
        ProfileVertexRef::Piece { step, role } => ProfileEdgeRef::Piece { step, role },
        ProfileVertexRef::Section { circle, role } => ProfileEdgeRef::Section { circle, role },
    };
    format!("the start of {}", piece(&edge, feature, by))
}

/// The role the walk stopped at, in words, before its feature: one
/// segment's role ([`role`]); at a seam junction, each seam that meets
/// there; for a path no operation mints, each segment's role in turn,
/// so that name too reads apart from every other.
fn head<'n, 's>(
    leaf: &'n StableName,
    by: Speaker<'s>,
    cites: &mut Cites<'_, '_, 's>,
) -> Vec<Item<'n, 's>> {
    let path = &leaf.path[..fragment_tail_start(&leaf.path)];
    let (lead, segs) = match path {
        [] => return vec![text(format!("the {}", leaf.kind.noun()))],
        [seg] => return role(seg, leaf, by, cites),
        many if many.iter().all(|s| matches!(s, RoleSeg::Seam { .. })) => {
            ("the junction of ", many)
        }
        many => ("the composite role of ", many),
    };
    let mut items = vec![text(lead)];
    for (i, seg) in segs.iter().enumerate() {
        if i > 0 {
            items.push(text("; "));
        }
        items.extend(role(seg, leaf, by, cites));
    }
    items
}

/// One segment of `leaf`'s head, in words. Exhaustive over [`RoleSeg`],
/// so a new segment is given words here or the compile breaks, and each
/// segment's words differ from every other's.
fn role<'n, 's>(
    seg: &'n RoleSeg,
    leaf: &'n StableName,
    by: Speaker<'s>,
    cites: &mut Cites<'_, '_, 's>,
) -> Vec<Item<'n, 's>> {
    let f = leaf.node;
    let kind = leaf.kind.noun();
    let one = |s: String| vec![text(s)];
    match seg {
        RoleSeg::OutputBody => one("the output body".to_owned()),
        RoleSeg::Cap(e) => one(format!("the {} cap", cap(*e))),
        RoleSeg::Lateral(r) => one(format!("the side wall over {}", run(r, f, by))),
        RoleSeg::RimEdge(c, e) => one(format!("the {} rim edge over {}", cap(*c), piece(e, f, by))),
        RoleSeg::LateralEdge(v) => one(format!("the lateral edge over {}", vertex(v, f, by))),
        RoleSeg::CapVertex(c, v) => one(format!(
            "the {} cap vertex over {}",
            cap(*c),
            vertex(v, f, by)
        )),
        RoleSeg::LoftWall(edges) => one(format!(
            "the loft wall over {}",
            pieces(edges.iter().map(|e| piece(e, f, by)))
        )),
        RoleSeg::LoftSeam(vertices) => one(format!(
            "the loft seam over {}",
            pieces(vertices.iter().map(|v| vertex(v, f, by)))
        )),
        RoleSeg::Band(r) => one(format!("the band face over {}", run(r, f, by))),
        RoleSeg::BandRim(v) => one(format!("the band rim over {}", vertex(v, f, by))),
        RoleSeg::BandRimPi(v) => one(format!("the second band rim over {}", vertex(v, f, by))),
        RoleSeg::BandPi(r) => one(format!("the second band face over {}", run(r, f, by))),
        RoleSeg::Meridian(m, r) => one(format!(
            "the {} meridian edge over {}",
            meridian(*m),
            run(r, f, by)
        )),
        RoleSeg::MeridianVertex(m, v) => one(format!(
            "the {} meridian vertex over {}",
            meridian(*m),
            vertex(v, f, by)
        )),
        RoleSeg::RevolveCap(m) => one(format!("the {} wedge cap", meridian(*m))),
        RoleSeg::Pole(v) => one(format!("the pole over {}", vertex(v, f, by))),
        RoleSeg::AxisEdge(e) => one(format!("the axis edge over {}", piece(e, f, by))),
        RoleSeg::Seam { a, b } => vec![
            text(format!("the seam {kind} of ")),
            cites.one(a),
            text(" and "),
            cites.one(b),
        ],
        RoleSeg::Merged(set) => {
            // A set on an edge is the edge a join made of its members.
            let what = if leaf.kind == EntityKind::Edge {
                "the joined edge of "
            } else {
                "the merged face of "
            };
            let mut items = vec![text(what)];
            items.extend(cites.list(set));
            items
        }
        RoleSeg::SplitBody(side) => one(format!("the body {}", half(*side))),
        RoleSeg::SectionFace { side, section } => {
            one(format!("section face {section} {}", half(*side)))
        }
        RoleSeg::SectionEdge { side, face } => vec![
            text("the section edge across "),
            cites.one(face),
            text(format!(" {}", half(*side))),
        ],
        RoleSeg::CrossingVertex { side, edge } => vec![
            text("the split's crossing of "),
            cites.one(edge),
            text(format!(" {}", half(*side))),
        ],
        RoleSeg::BlendFace(edge) => vec![text("the blend face over "), cites.one(edge)],
        RoleSeg::CornerFace(v) => vec![text("the corner face at "), cites.one(v)],
        RoleSeg::TrimEdge { edge, support } => vec![
            text("the trim edge on "),
            cites.one(support),
            text(" of the blend over "),
            cites.one(edge),
        ],
        RoleSeg::FootVertex { vertex, support } => vec![
            text("the blend foot on "),
            cites.one(support),
            text(" at "),
            cites.one(vertex),
        ],
        RoleSeg::EndArc { vertex, edge } => vec![
            text("the end arc at "),
            cites.one(vertex),
            text(" of the blend over "),
            cites.one(edge),
        ],
        RoleSeg::Mitre { vertex } => vec![text("the mitre at "), cites.one(vertex)],
        RoleSeg::TurnFoot { vertex } => vec![text("the turn foot at "), cites.one(vertex)],
        RoleSeg::BandFace(edges) => {
            let mut items = vec![text("the blend band over ")];
            items.extend(cites.list(edges));
            items
        }
        RoleSeg::BandTrim { edge, support } => vec![
            text(format!(
                "the {}-side trim edge of the blend band over ",
                match support {
                    RimSupport::Host => "host",
                    RimSupport::Mate => "mate",
                }
            )),
            cites.one(edge),
        ],
        RoleSeg::BandFoot(v) => vec![text("the blend band's foot at "), cites.one(v)],
        // A band's crossing and its slit each say their band: two
        // bands can cross, or slit, one edge.
        RoleSeg::BandCross { edge, band } => {
            let mut items = vec![text("the crossing of "), cites.one(edge)];
            items.push(text(" by the blend band over "));
            items.extend(cites.list(band));
            items
        }
        RoleSeg::BandSlit { edge, band } => {
            let mut items = vec![text("the slit along "), cites.one(edge)];
            items.push(text(" of the blend band over "));
            items.extend(cites.list(band));
            items
        }
        RoleSeg::Inner(of) => vec![text("the cavity twin of "), cites.one(of)],
        RoleSeg::Rim(of) => vec![text("the shell rim of "), cites.one(of)],
        RoleSeg::HoleRim { of, hole } => {
            vec![text(format!("the rim of hole {hole} in ")), cites.one(of)]
        }
        // The part's own steps and nodes are another document's ids,
        // so the part-local name is said by tag.
        RoleSeg::InPart { of } => vec![cites.by_tag(of), text(" in the part")],
        // A carry or a qualifier is a role only inside a path no
        // operation mints: the walk looks through a lone carry, and a
        // qualifier never ends the head. Each still has words of its
        // own, so such a path reads apart from every other.
        RoleSeg::FromA(of) => vec![cites.one(of), text(" from operand A")],
        RoleSeg::FromB(of) => vec![cites.one(of), text(" from operand B")],
        RoleSeg::FromMember { member, of } => vec![
            cites.one(of),
            text(format!(" from member {}", by.node(*member))),
        ],
        RoleSeg::FromTarget(of) => vec![cites.one(of), text(" from the target")],
        RoleSeg::SplitFragment { parent, side } => vec![
            text(format!("the part {} of ", half(*side))),
            cites.one(parent),
        ],
        RoleSeg::OnToolVertex { of, side } => {
            vec![text(format!("the copy {} of ", half(*side))), cites.one(of)]
        }
        RoleSeg::Instance { of, i } => {
            vec![text(format!("instance {i}'s copy of ")), cites.one(of)]
        }
        RoleSeg::BandCut(of) => vec![text("the surviving part of "), cites.one(of)],
        RoleSeg::Fragment(q) => {
            let (word, names) = match q {
                Qualifier::OrderAlong { .. } => {
                    return one(prefix(&Wrap::Part(q), leaf.kind)
                        .trim_end_matches(" of ")
                        .to_owned());
                }
                Qualifier::Borders(walls) => ("the part bordering ", walls),
                Qualifier::Keeps(edges) => ("the part along ", edges),
                Qualifier::Ends(ends) => ("the part between ", ends),
            };
            let mut items = vec![text(word)];
            items.extend(cites.list(names));
            items
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::role::NameRef;
    use crate::node::StepId;

    const EXTRUDE: RecipeNodeId = RecipeNodeId(1 << 16);
    const OTHER: RecipeNodeId = RecipeNodeId(2 << 16);
    const OP: RecipeNodeId = RecipeNodeId(3 << 16);
    const MOVED: RecipeNodeId = RecipeNodeId(4 << 16);

    fn name(kind: EntityKind, node: RecipeNodeId, path: Vec<RoleSeg>) -> StableName {
        StableName { kind, node, path }
    }

    fn leg(step: u64) -> ProfileEdgeRef {
        ProfileEdgeRef::Piece {
            step: StepId(step << 16),
            role: PieceRole::Leg,
        }
    }

    fn cap(end: CapEnd) -> StableName {
        name(EntityKind::Face, EXTRUDE, vec![RoleSeg::Cap(end)])
    }

    fn wall(step: u64) -> StableName {
        name(
            EntityKind::Face,
            EXTRUDE,
            vec![RoleSeg::Lateral(leg(step).into())],
        )
    }

    fn rim(step: u64) -> StableName {
        name(
            EntityKind::Edge,
            EXTRUDE,
            vec![RoleSeg::RimEdge(CapEnd::End, leg(step))],
        )
    }

    fn said(n: &StableName) -> String {
        leaf_role(n).to_string()
    }

    fn open(at: &[&[u16]]) -> Detail {
        Detail::Open(at.iter().map(|pos| pos.to_vec()).collect())
    }

    /// A carry through a primary operand is silent, and the sentence
    /// says the feature that made the leaf; which node holds the name is
    /// the enclosing sentence's to say.
    #[test]
    fn a_primary_carry_is_silent_and_the_feature_is_said() {
        let carried = name(
            EntityKind::Face,
            OP,
            vec![RoleSeg::FromA(NameRef::new(name(
                EntityKind::Face,
                OTHER,
                vec![RoleSeg::FromTarget(NameRef::new(cap(CapEnd::End)))],
            )))],
        );
        assert_eq!(said(&carried), "the end cap of node 000000000001");
        assert_eq!(role_leaf(&carried).node, EXTRUDE);
        assert_eq!(said(&cap(CapEnd::End)), "the end cap of node 000000000001");
        assert_eq!(
            carried.to_string(),
            said(&carried),
            "the name's own Display"
        );
    }

    /// Two copies of one master, brought into one body through two
    /// secondary operands, differ in their joins: each is said, outermost
    /// first. By tag a Boolean's B join says only what the name holds,
    /// that it came in as operand B.
    #[test]
    fn a_secondary_carry_is_a_join_said_with_its_node() {
        let through_b = |at: RecipeNodeId, inner: StableName| {
            name(
                EntityKind::Face,
                at,
                vec![RoleSeg::FromB(NameRef::new(inner))],
            )
        };
        let member = name(
            EntityKind::Face,
            OTHER,
            vec![RoleSeg::FromMember {
                member: MOVED,
                of: NameRef::new(cap(CapEnd::End)),
            }],
        );
        assert_eq!(
            said(&through_b(OP, member.clone())),
            "the end cap of node 000000000001, through operand B of node 000000000003, joined \
             at node 000000000002 from node 000000000004"
        );
        assert_ne!(
            said(&through_b(OP, cap(CapEnd::End))),
            said(&name(
                EntityKind::Face,
                OP,
                vec![RoleSeg::FromA(NameRef::new(cap(CapEnd::End)))]
            )),
            "B and A of one boolean read apart"
        );
    }

    /// The parts of one cut face differ in what they border, or in
    /// their rank, and a split's two parts in their side: each is said.
    #[test]
    fn the_parts_of_one_face_are_told_apart() {
        let part = |q| {
            name(
                EntityKind::Face,
                OP,
                vec![
                    RoleSeg::FromA(NameRef::new(cap(CapEnd::End))),
                    RoleSeg::Fragment(q),
                ],
            )
        };
        assert_eq!(
            said(&part(Qualifier::Borders(vec![wall(1)]))),
            "the part of the end cap of node 000000000001 bordering the side wall over the \
             profile step 000000000001 of node 000000000001"
        );
        assert_eq!(
            said(&part(Qualifier::OrderAlong { rank: 1, of: 3 })),
            "part 1 of 3 of the end cap of node 000000000001"
        );
        let half = |side| {
            name(
                EntityKind::Face,
                OP,
                vec![RoleSeg::SplitFragment {
                    side,
                    parent: NameRef::new(wall(1)),
                }],
            )
        };
        assert_eq!(
            said(&half(SplitHalf::Above)),
            "the part above the split of the side wall over the profile step 000000000001 of \
             node 000000000001"
        );
        assert_ne!(said(&half(SplitHalf::Above)), said(&half(SplitHalf::Below)));
    }

    /// A blend face, a shell's cavity twin and a split's section edge
    /// are said over what they were made against, which keeps its own
    /// feature.
    #[test]
    fn blend_shell_and_split_faces_are_said_over_their_source() {
        let blend = |step| {
            name(
                EntityKind::Face,
                OP,
                vec![RoleSeg::BlendFace(NameRef::new(rim(step)))],
            )
        };
        assert_eq!(
            said(&blend(1)),
            "the blend face over the end rim edge over the profile step 000000000001 of node \
             000000000001 of node 000000000003"
        );
        assert_ne!(said(&blend(1)), said(&blend(2)));
        let twin = name(
            EntityKind::Face,
            OP,
            vec![RoleSeg::Inner(NameRef::new(cap(CapEnd::Start)))],
        );
        assert_eq!(
            said(&twin),
            "the cavity twin of the start cap of node 000000000001 of node 000000000003"
        );
    }

    /// The full form says every cited name in full; a lesser detail
    /// says a citation it does not open by its kind, and a list by the
    /// members it opens and how many more.
    #[test]
    fn the_detail_bounds_how_much_of_a_cited_name_is_said() {
        let seam = name(
            EntityKind::Edge,
            OTHER,
            vec![RoleSeg::Seam {
                a: NameRef::new(cap(CapEnd::End)),
                b: NameRef::new(wall(1)),
            }],
        );
        let blend = name(
            EntityKind::Face,
            OP,
            vec![RoleSeg::BlendFace(NameRef::new(seam))],
        );
        assert_eq!(
            said(&blend),
            "the blend face over the seam edge of the end cap of node 000000000001 and the side \
             wall over the profile step 000000000001 of node 000000000001 of node 000000000002 \
             of node 000000000003"
        );
        assert_eq!(
            words(&blend, Speaker::TAG, &open(&[&[0]])),
            "the blend face over the seam edge of a face and a face of node 000000000002 of node \
             000000000003"
        );
        assert_eq!(
            words(&blend, Speaker::TAG, &open(&[])),
            "the blend face over an edge of node 000000000003"
        );
        let merged = name(
            EntityKind::Face,
            OP,
            vec![RoleSeg::Merged(vec![wall(1), wall(2), wall(3)])],
        );
        assert_eq!(
            words(&merged, Speaker::TAG, &open(&[])),
            "the merged face of 3 faces of node 000000000003"
        );
        assert!(
            words(&merged, Speaker::TAG, &open(&[&[0], &[1]])).contains(" and 1 more of node"),
            "two in full, then how many more"
        );
        // A set on an edge is the edge a join made of its members.
        let joined = name(
            EntityKind::Edge,
            OP,
            vec![RoleSeg::Merged(vec![rim(1), rim(2)])],
        );
        assert_eq!(
            words(&joined, Speaker::TAG, &open(&[])),
            "the joined edge of 2 edges of node 000000000003"
        );
    }

    /// A derivation of any depth is said whole from an explicit stack.
    #[test]
    fn a_deep_derivation_is_said_whole() {
        let deep = (0..3).fold(cap(CapEnd::End), |n, i| {
            name(
                EntityKind::Face,
                OP,
                vec![RoleSeg::Instance {
                    i,
                    of: NameRef::new(n),
                }],
            )
        });
        assert_eq!(
            said(&deep),
            "instance 2's copy of instance 1's copy of instance 0's copy of the end cap of node \
             000000000001"
        );
    }

    /// A name whose citations nest past every thread's stack is said in
    /// full on the smallest stack.
    #[test]
    fn a_citation_nested_past_every_stack_is_said_on_the_smallest_stack() {
        const DEEP: usize = 20_000;
        let said = test_utils::own_thread::on_the_smallest_stack(|| {
            let deep = (0..DEEP).fold(cap(CapEnd::End), |n, _| {
                name(EntityKind::Face, OP, vec![RoleSeg::Inner(NameRef::new(n))])
            });
            said(&deep)
        });
        assert_eq!(said.matches("the cavity twin of ").count(), DEEP);
    }

    /// A table naming `names`, each a face of its own.
    fn table(names: &[&StableName]) -> NameTable {
        let mut table = NameTable::new();
        for (i, name) in (1u64..).zip(names) {
            let face = crate::names::EntityRef {
                body: 0,
                key: crate::names::EntityKey::Face(topo::FaceKey::from(
                    slotmap::KeyData::from_ffi(i),
                )),
            };
            table.insert((*name).clone(), face).expect("each name once");
        }
        table
    }

    fn merged(members: Vec<StableName>) -> StableName {
        name(EntityKind::Face, OP, vec![RoleSeg::Merged(members)])
    }

    /// Each name of `names`, said at the detail a table of them all
    /// gives it.
    fn within(names: &[&StableName]) -> Vec<String> {
        let table = table(names);
        let details = table_details(&table);
        names
            .iter()
            .map(|n| words(n, Speaker::TAG, &details[*n]))
            .collect()
    }

    fn detail_in(names: &[&StableName], name: &StableName) -> Detail {
        table_details(&table(names))[name].clone()
    }

    /// The scoped detail opens a citation only where a rival needs it:
    /// a blend face beside a cap opens nothing, and beside a blend over
    /// another rim opens the one rim that tells them apart.
    #[test]
    fn the_unique_detail_opens_only_what_a_rival_needs() {
        let blend = |step| {
            name(
                EntityKind::Face,
                OP,
                vec![RoleSeg::BlendFace(NameRef::new(rim(step)))],
            )
        };
        let (one, two, top) = (blend(1), blend(2), cap(CapEnd::End));
        assert_eq!(
            detail_in(&[&one, &top], &one),
            open(&[]),
            "nothing alike, nothing opened"
        );
        assert_eq!(
            detail_in(&[&one, &two, &top], &one),
            open(&[&[0]]),
            "the rim the two blends differ by, and no more"
        );
    }

    /// **No two names of a table read alike as each is said**, including
    /// a name an earlier detail had told apart: `mixed` reads "2
    /// entities" beside `faces`' "2 faces", but opens to the same "the
    /// side wall … and 1 more" at the member the other rival needed
    /// opened.
    #[test]
    fn the_unique_detail_reads_apart_from_every_name_of_the_table() {
        let faces = merged(vec![wall(1), wall(2)]);
        let rival = merged(vec![wall(2), wall(3)]);
        let mixed = merged(vec![wall(1), rim(4)]);
        let names = [&faces, &rival, &mixed];
        assert_ne!(
            detail_in(&names, &faces),
            Detail::Full,
            "an opening tells all three apart"
        );
        let said = within(&names);
        assert!(
            said[0] != said[1] && said[0] != said[2] && said[1] != said[2],
            "said within the table, two names read alike: {said:#?}"
        );
        assert_eq!(
            words(&mixed, Speaker::TAG, &open(&[&[0]])),
            words(&faces, Speaker::TAG, &open(&[&[0]])),
            "the premise: the opening the first rival asks for makes the second alike"
        );
    }

    /// **Each name is told apart as every other is said, at its own
    /// detail**: `x` and `w` each read apart from every rival at one
    /// shared detail, but `x` opens its second member and `w` its first,
    /// and both then say "the side wall over step 2 … and 1 more". Lists
    /// in canonical order, then out of it.
    #[test]
    fn two_names_at_their_own_details_never_read_alike() {
        for steps in [[[1, 2], [2, 5], [1, 6]], [[2, 1], [2, 3], [3, 4]]] {
            let [x, w, r] = steps.map(|s| merged(s.iter().map(|&k| wall(k)).collect()));
            let top = name(
                EntityKind::Face,
                OP,
                vec![RoleSeg::FromA(NameRef::new(cap(CapEnd::End)))],
            );
            let said = within(&[&x, &w, &r, &top]);
            for (i, one) in said.iter().enumerate() {
                for other in &said[i + 1..] {
                    assert_ne!(one, other, "{steps:?}: two names read alike: {said:#?}");
                }
            }
        }
    }

    /// Two bands crossing, or slitting, one edge differ only in their
    /// band, and each says it.
    #[test]
    fn a_band_crossing_and_slit_say_their_band() {
        let edge = || NameRef::new(rim(1));
        let crossing = |band: Vec<StableName>| {
            name(
                EntityKind::Vertex,
                OP,
                vec![RoleSeg::BandCross { edge: edge(), band }],
            )
        };
        let slit = |band: Vec<StableName>| {
            name(
                EntityKind::Edge,
                OP,
                vec![RoleSeg::BandSlit { edge: edge(), band }],
            )
        };
        let (near, far) = (vec![rim(1), rim(2)], vec![rim(1), rim(3)]);
        assert_ne!(said(&crossing(near.clone())), said(&crossing(far.clone())));
        assert_ne!(said(&slit(near)), said(&slit(far)));
    }

    /// A path no operation mints is said segment by segment, so two of
    /// them read apart wherever a segment differs, and a seam junction
    /// says each seam that meets there.
    #[test]
    fn a_composite_path_and_a_junction_say_every_segment() {
        let composite = |q| {
            name(
                EntityKind::Face,
                OP,
                vec![RoleSeg::Fragment(q), RoleSeg::Cap(CapEnd::End)],
            )
        };
        let (first, second) = (
            composite(Qualifier::OrderAlong { rank: 0, of: 2 }),
            composite(Qualifier::OrderAlong { rank: 1, of: 2 }),
        );
        assert_eq!(
            said(&first),
            "the composite role of part 0 of 2; the end cap of node 000000000003"
        );
        assert_ne!(said(&first), said(&second));
        let seam = |a: StableName, b: StableName| RoleSeg::Seam {
            a: NameRef::new(a),
            b: NameRef::new(b),
        };
        let junction = |step| {
            name(
                EntityKind::Vertex,
                OP,
                vec![
                    seam(cap(CapEnd::End), wall(1)),
                    seam(cap(CapEnd::End), wall(step)),
                ],
            )
        };
        assert!(
            said(&junction(2)).starts_with("the junction of the seam vertex of the end cap"),
            "{}",
            said(&junction(2))
        );
        assert_ne!(said(&junction(2)), said(&junction(3)));
    }
}
