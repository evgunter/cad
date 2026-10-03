//! **A name in words** — what a person reads to tell one entity from
//! another (`work/recipe/names-render-a-faces-leaf-role-in-words.md`,
//! ruled on #3571 and #3906).
//!
//! The path as a structure is the machine channel; a person reads it
//! as one sentence, `<role> of <feature>[, <join>…][, on <node>]`:
//!
//! - **The role** is the leaf's: the entity the author made, reached by
//!   looking through every segment that carries an operand's entity on
//!   ([`origin`]'s `Carried`). A carry that changed the entity — a
//!   fragment qualifier, a split's half, a pattern's copy, a band's cut
//!   — is said around it: "the part of the end cap of Extrude e548
//!   bordering …", "instance 2's copy of …".
//! - **The feature** is the node that made the leaf, said right after
//!   the leaf's own words, so a name a role cites keeps its feature:
//!   "the blend face over the end rim edge over the leg of loop 0
//!   step 2 of Extrude e548 of Fillet 92b0".
//! - **A join** is a carry through a secondary operand — a Boolean's B,
//!   a union's member — said with its node, outermost first: "…, joined
//!   at Boolean 1669", "…, joined at Union d1aa from Transform 3218". A
//!   carry through a primary operand (a Boolean's A, a fillet's target)
//!   is the body's own continuation and is silent. Two names of one
//!   table first differ at a node where one went through a secondary
//!   operand, which a join says.
//! - **"on <node>"** names the node whose output holds the name, said
//!   only where the sentence does not already say it: not when it made
//!   the leaf, not when it is the outermost join, not when the
//!   enclosing sentence is about it ([`Speaker::about`]), and never for
//!   a cited name, whose holder its citing role fixes.
//!
//! **How much of a cited name is said is the speaker's [`Detail`].**
//! The full form says every cited name and every list member in full,
//! however deep. A speaker holding the name table the name was read
//! from ([`Speaker::within`]) says the least detail that no other name
//! of that table reads alike at; one holding none says the full form.
//! Past the detail's depth a cited name is said by its kind ("a face"),
//! and past its width a list says how many more.
//!
//! **Every number is counted from zero**, as the profile pane numbers
//! a loop and its steps: `loop 0 step 2`, `instance 0's copy`, `part 1
//! of 3`. A profile piece is a "piece" (`piece 1 of loop 0 step 0`); a
//! cut of a face, edge or body is a "part".
//!
//! A profile step is said as the pane numbers it wherever the speaker's
//! document holds it, and with its profile unless the feature reads
//! that profile alone; by its tag (`the profile step <tag>`) where no
//! document is at hand.
//!
//! The sentence is built from an explicit stack, never the call stack,
//! so a name nested past every thread's stack renders.

use core::fmt;
use std::collections::BTreeSet;

use crate::names::NameTable;
use crate::names::attribute::{CarriedAs, SegOrigin, origin};
use crate::names::role::{
    CapEnd, EntityKind, MeridianEnd, PieceRun, ProfileEdgeRef, ProfileVertexRef, Qualifier,
    RimSupport, RoleSeg, SectionCircle, SplitHalf, StableName, fragment_tail_start,
};
use crate::node::RecipeNodeId;
use crate::spoken::{Said, Say, Speaker};
use profile::PieceRole;

/// **A name in words** ([module docs](self)): `the end cap of Extrude
/// e548`, `the part above the split of the side wall over the leg of
/// loop 0 step 2 of Extrude e548, on Split 2fec`. Article-led, so a
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
    /// Every citation opened: the form two distinct names never share.
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

/// **The least detail at which `name` reads apart from every other
/// name of `table`**, said by `by`. Starting from no citation opened,
/// it opens one citation at a time — the one that leaves the fewest
/// names reading alike, the nearest among equals — until none does,
/// then shuts again each opening the others made needless; where
/// opening every citation still leaves one, the full form.
pub(crate) fn least_detail(name: &StableName, by: Speaker<'_>, table: &NameTable) -> Detail {
    let others: Vec<&StableName> = table
        .iter()
        .map(|(n, _)| n)
        .filter(|n| *n != name)
        .collect();
    let alike = |open: &BTreeSet<Pos>, among: &[&StableName]| -> usize {
        let detail = Detail::Open(open.clone());
        let mine = words(name, by, &detail);
        among
            .iter()
            .filter(|other| words(other, by, &detail) == mine)
            .count()
    };
    let mut open = BTreeSet::new();
    let mut rivals = others.clone();
    let mut shut = cited_positions(name, by);
    loop {
        let detail = Detail::Open(open.clone());
        let mine = words(name, by, &detail);
        rivals.retain(|other| words(other, by, &detail) == mine);
        if rivals.is_empty() {
            break;
        }
        // The citations said now: those whose citing name is open.
        let sayable: Vec<usize> = (0..shut.len())
            .filter(|&i| detail.opens(&shut[i][..shut[i].len() - 1]))
            .collect();
        // The opening that leaves the fewest names reading alike, the
        // nearest among equals.
        let Some(pick) = sayable.iter().copied().min_by_key(|&i| {
            let mut tried = open.clone();
            tried.insert(shut[i].clone());
            alike(&tried, &rivals)
        }) else {
            return Detail::Full;
        };
        open.insert(shut.remove(pick));
    }
    // An opening a later one made needless is shut again, deepest
    // first, while the name still reads apart from every other.
    for pos in open.clone().iter().rev() {
        let mut fewer = open.clone();
        fewer.retain(|kept| !kept.starts_with(pos));
        if alike(&fewer, &others) == 0 {
            open = fewer;
        }
    }
    Detail::Open(open)
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

/// A name, its carrying segments and qualifiers looked through.
struct Walk<'n> {
    /// The carries said around the leaf, outermost first.
    wraps: Vec<Wrap<'n>>,
    /// The joins, outermost first.
    joins: Vec<Join>,
    /// The name the walk stopped at.
    leaf: &'n StableName,
}

fn walk(name: &StableName) -> Walk<'_> {
    let mut wraps = Vec::new();
    let mut joins = Vec::new();
    let mut at = name;
    loop {
        let tail = fragment_tail_start(&at.path);
        // `[parent, Fragment(q1), Fragment(q2)]` is the q2 part of the
        // q1 part: the last qualifier is the outermost.
        wraps.extend(at.path[tail..].iter().rev().filter_map(|seg| match seg {
            RoleSeg::Fragment(q) => Some(Wrap::Part(q)),
            _ => None,
        }));
        let [seg] = &at.path[..tail] else {
            return Walk {
                wraps,
                joins,
                leaf: at,
            };
        };
        let SegOrigin::Carried(of, carry) = origin(seg) else {
            return Walk {
                wraps,
                joins,
                leaf: at,
            };
        };
        match carry {
            CarriedAs::Primary => {}
            CarriedAs::Secondary => joins.push(match seg {
                RoleSeg::FromMember { member, .. } => Join::Member {
                    union: at.node,
                    member: *member,
                },
                _ => Join::B(at.node),
            }),
            CarriedAs::Split(side) => wraps.push(Wrap::Split(side)),
            CarriedAs::ToolCopy(side) => wraps.push(Wrap::ToolCopy(side)),
            CarriedAs::Instance(i) => wraps.push(Wrap::Instance(i)),
            CarriedAs::Cut => wraps.push(Wrap::Cut),
        }
        at = of;
    }
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
    let Walk { wraps, joins, leaf } = walk(name);
    let mut items: Vec<Item<'n, 's>> = wraps.iter().map(|w| text(prefix(w, name.kind))).collect();
    items.extend(head(leaf, by, &mut cites));
    items.push(text(format!(" of {}", by.node(leaf.node))));
    for wrap in wraps.iter().rev() {
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
    for join in &joins {
        items.push(text(match *join {
            Join::B(at) => format!(", joined at {}", by.node(at)),
            Join::Member { union, member } => {
                format!(", joined at {} from {}", by.node(union), by.node(member))
            }
        }));
    }
    let holder_said = name.node == leaf.node
        || by.is_about(name.node)
        || matches!(
            joins.first(),
            Some(Join::B(at) | Join::Member { union: at, .. }) if *at == name.node
        );
    if pos.is_empty() && !holder_said {
        items.push(text(format!(", on {}", by.node(name.node))));
    }
    items
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
/// which circle.
fn piece(e: &ProfileEdgeRef, feature: RecipeNodeId, by: Speaker<'_>) -> String {
    match e {
        ProfileEdgeRef::Piece { step, role } => match by.step(*step) {
            Some(at) if by.sole_profile(feature) == Some(at.profile()) => {
                format!("{} of {at}", role_np(*role))
            }
            Some(at) => format!("{} of {at} in {}", role_np(*role), by.node(at.profile())),
            None => format!("{} of the profile step {step}", role_np(*role)),
        },
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

/// The role the walk stopped at, in words, before its feature.
/// Exhaustive over [`RoleSeg`], so a new segment is given words here or
/// the compile breaks.
fn head<'n, 's>(
    leaf: &'n StableName,
    by: Speaker<'s>,
    cites: &mut Cites<'_, '_, 's>,
) -> Vec<Item<'n, 's>> {
    let path = &leaf.path[..fragment_tail_start(&leaf.path)];
    let seg = match path {
        [] => return vec![text(format!("the {}", leaf.kind.noun()))],
        [seg] => seg,
        // A seam junction: the run of the seams that meet there.
        many if many.iter().all(|s| matches!(s, RoleSeg::Seam { .. })) => {
            return vec![text(format!("the junction of {} seams", many.len()))];
        }
        _ => return vec![text("the entity of an unread role")],
    };
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
            let mut items = vec![text("the merged face of ")];
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
        RoleSeg::BandCross { edge, .. } => {
            vec![text("the blend band's crossing of "), cites.one(edge)]
        }
        RoleSeg::BandSlit { edge, .. } => {
            vec![text("the blend band's slit along "), cites.one(edge)]
        }
        RoleSeg::Inner(of) => vec![text("the cavity twin of "), cites.one(of)],
        RoleSeg::Rim(of) => vec![text("the shell rim of "), cites.one(of)],
        RoleSeg::HoleRim { of, hole } => {
            vec![text(format!("the rim of hole {hole} in ")), cites.one(of)]
        }
        // The part's own steps and nodes are another document's ids,
        // so the part-local name is said by tag.
        RoleSeg::InPart { of } => vec![cites.by_tag(of), text(" in the part")],
        // `origin` carries these on, so the walk never stops at one;
        // said as the name they carry, the match stays total.
        RoleSeg::FromA(of)
        | RoleSeg::FromB(of)
        | RoleSeg::FromMember { of, .. }
        | RoleSeg::FromTarget(of)
        | RoleSeg::SplitFragment { parent: of, .. }
        | RoleSeg::OnToolVertex { of, .. }
        | RoleSeg::Instance { of, .. }
        | RoleSeg::BandCut(of) => vec![cites.one(of)],
        RoleSeg::Fragment(_) => one(format!("a part of {}", kind_np(leaf.kind))),
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
    /// says the feature that made the leaf and the node that holds the
    /// name.
    #[test]
    fn a_primary_carry_is_silent_and_the_feature_and_holder_are_said() {
        let carried = name(
            EntityKind::Face,
            OP,
            vec![RoleSeg::FromA(NameRef::new(name(
                EntityKind::Face,
                OTHER,
                vec![RoleSeg::FromTarget(NameRef::new(cap(CapEnd::End)))],
            )))],
        );
        assert_eq!(
            said(&carried),
            "the end cap of node 000000000001, on node 000000000003"
        );
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
    /// first, and the holder is not said twice.
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
            "the end cap of node 000000000001, joined at node 000000000003, joined at node \
             000000000002 from node 000000000004"
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
            "the part of the end cap of node 000000000001 bordering the side wall over the leg \
             of the profile step 000000000001 of node 000000000001, on node 000000000003"
        );
        assert_eq!(
            said(&part(Qualifier::OrderAlong { rank: 1, of: 3 })),
            "part 1 of 3 of the end cap of node 000000000001, on node 000000000003"
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
            "the part above the split of the side wall over the leg of the profile step \
             000000000001 of node 000000000001, on node 000000000003"
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
            "the blend face over the end rim edge over the leg of the profile step 000000000001 \
             of node 000000000001 of node 000000000003"
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
             wall over the leg of the profile step 000000000001 of node 000000000001 of node \
             000000000002 of node 000000000003"
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
             000000000001, on node 000000000003"
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
}
