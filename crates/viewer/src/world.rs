//! **The world, as the viewer reads it**: which bodies the picture
//! draws, in which spaces, and what a pick on a drawn body is a pick on.
//!
//! The picture draws bodies, not placements (`crates/viewer/README.md`,
//! *The picture draws bodies*). **A body is drawn unless an operation
//! reads it as a body** ([`drawn_bodies`]): as an operator's operand
//! (`∪ − ∩ ⊔`), as a one-input operation's input (through the selection
//! a fillet, a chamfer or a shell reads, or the body a split or a mirror
//! reads), or as a placement's source, wherever the copy lands. A read of
//! one member of a family, or of one half of a split, takes over that
//! body alone. A mate, a selection, a measure and an assertion read no
//! body as a body, so none of them hides one. A `⊔` is drawn as its
//! pieces: each operand it reads is drawn as itself, where it stands.
//! What this derives is the picture only: the product stays the slot the
//! document writes, and nothing here re-enters the model.
//!
//! Each drawn body is drawn in its own space, and the **shown spaces**
//! are the world and every space that holds a drawn body
//! ([`shown_spaces`], IDENTITY DI5). A copy carries its body's names as
//! they are, so a pick on a copy names the copied body's entity; a
//! gesture that authors against a body reads the copy's source under
//! that name ([`on_body`]). A pick on any other drawn body — a
//! construction, an operator's result — is on that body's own operation.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::collections::{BTreeMap, BTreeSet};

use pncad::document::{
    Doc, Node, Operand, OperandSlot, PortKind, ProfileProgram, RecipeNodeId, VarDef, VarId,
    VarKind, eval_var_count,
};
use pncad::prelude::StableName;

/// **Which of a node's bodies a read takes, or a drawing covers**, by
/// the body index the node's value holds them at (a family's members in
/// index order, a split's halves by `SplitHalf::output_body`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Covers {
    /// Every body of the value but these.
    AllBut(BTreeSet<u32>),
    /// These bodies alone.
    Only(BTreeSet<u32>),
}

impl Covers {
    /// Every body of the value.
    pub fn all() -> Self {
        Self::AllBut(BTreeSet::new())
    }

    /// Whether the body at `body` is covered.
    pub fn covers(&self, body: u32) -> bool {
        match self {
            Self::AllBut(taken) => !taken.contains(&body),
            Self::Only(bodies) => bodies.contains(&body),
        }
    }
}

/// **One drawing of a body**: the bodies of `node`'s value it covers,
/// and the `⊔`s it is drawn as a piece of, outermost first (empty for a
/// body drawn on its own). A body two drawings cover — a piece two `⊔`s
/// draw, the product's piece an operator also reads — is drawn once,
/// and is out of the picture only once every drawing of it is
/// ([`crate::display::DisplayView::hides`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Drawn {
    /// The node whose value is drawn.
    pub node: RecipeNodeId,
    /// Which of its bodies.
    pub covers: Covers,
    /// The `⊔`s this is a piece of, outermost first.
    pub within: Vec<RecipeNodeId>,
}

impl Drawn {
    /// Every body of `node`, drawn on its own.
    pub fn whole(node: RecipeNodeId) -> Self {
        Self {
            node,
            covers: Covers::all(),
            within: Vec::new(),
        }
    }

    /// Whether this draws body `body` of `node`.
    pub fn draws(&self, node: RecipeNodeId, body: u32) -> bool {
        self.node == node && self.covers.covers(body)
    }

    /// Whether this is a drawing of `node`'s body: `node`'s own, or a
    /// piece of `node` when it is a `⊔`.
    pub fn of_node(&self, node: RecipeNodeId) -> bool {
        self.node == node || self.within.contains(&node)
    }
}

/// **What `var`, read as a body by `reader` with the indices `at`,
/// takes**: the operation defining it and which of its bodies. A read
/// of a split's port takes that half; an indexed read (`xs[i]`) the
/// members its indices name, over every member of `reader`
/// ([`members_read`]); a plain read the whole value. A selection takes
/// the body it is read in; a pick of a piece of a copy takes the copy,
/// which is drawn as one body. `None` for a variable that is no body's.
fn taken_by(
    doc: &Doc<ProfileProgram>,
    reader: RecipeNodeId,
    var: VarId,
    at: &[VarId],
) -> Option<(RecipeNodeId, Covers)> {
    match doc.var(var)?.def() {
        VarDef::Output { node, port } => {
            let covers = if matches!(doc.node(*node)?, Node::Split { .. }) {
                Covers::Only(BTreeSet::from([u32::from(*port)]))
            } else if at.is_empty() {
                Covers::all()
            } else {
                members_read(doc, reader, *node, at).map_or_else(Covers::all, |members| {
                    Covers::Only(members.into_iter().collect())
                })
            };
            Some((*node, covers))
        }
        VarDef::Select(select) => taken_by(doc, reader, select.body, &[]),
        VarDef::Pick(pick) => taken_by(doc, reader, pick.placement, &[]),
        _ => None,
    }
}

/// **The members of the family `family` an indexed read by `reader`
/// names**, one per member of `reader` in its member order (one for a
/// reader evaluated once): each member's index values, the read's
/// indices evaluated in that member's environment, found among the
/// family's own ([`Doc::member_envs`], the evaluator's enumeration).
/// `None` where a count, an index or a member does not evaluate: that
/// read refuses at evaluation, and is taken to read the family whole.
fn members_read(
    doc: &Doc<ProfileProgram>,
    reader: RecipeNodeId,
    family: RecipeNodeId,
    at: &[VarId],
) -> Option<Vec<u32>> {
    let family_reach = doc.node_reach(family);
    let members: Vec<Vec<i64>> = doc
        .member_envs::<f64>(family_reach.chain()?)
        .map(|member| member.map(|member| member.at))
        .collect::<Result<_, _>>()
        .ok()?;
    let reader_reach = doc.node_reach(reader);
    let chain = reader_reach.chain()?;
    let envs = if chain.is_empty() {
        vec![doc.var_env::<f64>()]
    } else {
        doc.member_envs::<f64>(chain)
            .map(|member| member.map(|member| member.env))
            .collect::<Result<Vec<_>, _>>()
            .ok()?
    };
    envs.iter()
        .map(|env| {
            let values = at
                .iter()
                .map(|&index| eval_var_count(index, env).ok())
                .collect::<Option<Vec<i64>>>()?;
            let position = members.iter().position(|member| *member == values)?;
            u32::try_from(position).ok()
        })
        .collect()
}

/// **How many bodies `node` builds**, read off the recipe: a split's
/// two halves, a family's members ([`Doc::member_envs`]), one body
/// otherwise. `None` where a count does not evaluate, which refuses the
/// node at evaluation.
fn body_count(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Option<u32> {
    if matches!(doc.node(node)?, Node::Split { .. }) {
        return Some(2);
    }
    let reach = doc.node_reach(node);
    let chain = reach.chain()?;
    if chain.is_empty() {
        return Some(1);
    }
    u32::try_from(doc.member_envs::<f64>(chain).count()).ok()
}

/// **Whether a read at `seat` reads a body as a body**: an operand of
/// an operator, a one-input operation's input (a split's target, a
/// mirror's body, the body a blend's or a shell's selection is read
/// in), a placement's source. A mate's side, a measure's reference, a
/// profile's face or frame, a split's or a mirror's plane and a
/// placement's target read no body as one.
fn reads_as_body(seat: OperandSlot) -> bool {
    match seat {
        OperandSlot::Source
        | OperandSlot::Target
        | OperandSlot::Body
        | OperandSlot::From
        | OperandSlot::Cut
        | OperandSlot::Member(_)
        | OperandSlot::Members
        | OperandSlot::Selection
        | OperandSlot::Open => true,
        OperandSlot::Profile
        | OperandSlot::Section(_)
        | OperandSlot::Path
        | OperandSlot::Axis
        | OperandSlot::Frame
        | OperandSlot::Face
        | OperandSlot::Traced(_)
        | OperandSlot::Side { .. }
        | OperandSlot::Measured(..)
        | OperandSlot::Tool
        | OperandSlot::Input
        | OperandSlot::Of
        | OperandSlot::Plane
        | OperandSlot::Space => false,
    }
}

/// **The reads the node `id` takes bodies by**, each resolved to what
/// it takes ([`taken_by`]), in field order.
fn takes(doc: &Doc<ProfileProgram>, id: RecipeNodeId) -> Vec<(RecipeNodeId, Covers)> {
    let Some(node) = doc.node(id) else {
        return Vec::new();
    };
    node.operand_rows()
        .into_iter()
        .filter(|&(seat, _)| reads_as_body(seat))
        .filter_map(|(seat, var)| {
            let at = node
                .body_read(seat)
                .map(|read| read.at.clone())
                .unwrap_or_default();
            taken_by(doc, id, var, &at)
        })
        .collect()
}

/// **Whether `node` builds a body the picture can draw**: one of its
/// ports is a body, a family of bodies, or a placement's copy.
pub fn builds_a_body(node: &Node<ProfileProgram>) -> bool {
    node.outputs().iter().any(|port| match port.kind {
        PortKind::Of(kind) => matches!(kind, VarKind::Body | VarKind::Bodies),
        PortKind::Placed => true,
    })
}

/// **The bodies the author's toggles show** beyond what the default
/// draws ([`Picture::showing`]): display state, and the third part of
/// the picture's key ([`crate::pickindex::PictureKey`]), compared by
/// the set it holds. Shared, so a key and a request carry it cheaply.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Shown(std::sync::Arc<BTreeSet<RecipeNodeId>>);

impl Shown {
    /// No show: the default picture.
    pub fn none() -> Self {
        Self::default()
    }

    /// The show of `nodes`.
    pub fn of(nodes: BTreeSet<RecipeNodeId>) -> Self {
        Self(std::sync::Arc::new(nodes))
    }

    /// The nodes shown, in id order.
    pub fn nodes(&self) -> &BTreeSet<RecipeNodeId> {
        &self.0
    }
}

/// **The picture a document draws**, computed once (module docs): every
/// drawing of a body, and the product's pieces among them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Picture {
    /// Every drawing: in document order, each body no operation reads
    /// as a body (a family less the members read alone), a `⊔` as its
    /// pieces; then the product's pieces no such drawing is, since the
    /// product slot's body is always drawn.
    pub drawn: Vec<Drawn>,
    /// What the product slot reaches, a `⊔` as its pieces: a drawn
    /// body none of these covers is outside the product, and the
    /// picture tints it so.
    pub product: Vec<Drawn>,
}

impl Picture {
    /// The picture `doc` draws.
    pub fn of(doc: &Doc<ProfileProgram>) -> Self {
        // Per node: `None` where some read takes it whole, else the
        // bodies read alone.
        let mut taken: BTreeMap<RecipeNodeId, Option<BTreeSet<u32>>> = BTreeMap::new();
        for id in doc.ids() {
            for (at, covers) in takes(doc, id) {
                let entry = taken.entry(at).or_insert_with(|| Some(BTreeSet::new()));
                match (covers, entry.as_mut()) {
                    (Covers::Only(bodies), Some(held)) => held.extend(bodies),
                    _ => *entry = None,
                }
            }
        }
        let mut drawn = Vec::new();
        for id in doc.ids() {
            if !doc.node(id).is_some_and(builds_a_body) {
                continue;
            }
            let covers = match taken.get(&id) {
                None => Covers::all(),
                Some(None) => continue,
                // Every body read alone is the node read whole.
                Some(Some(bodies))
                    if body_count(doc, id)
                        .is_some_and(|count| (0..count).all(|body| bodies.contains(&body))) =>
                {
                    continue;
                }
                Some(Some(bodies)) => Covers::AllBut(bodies.clone()),
            };
            pieces(doc, id, covers, &mut Vec::new(), &mut drawn);
        }
        let mut product = Vec::new();
        if let Some(var) = doc.product()
            && let Some((node, covers)) = taken_by(doc, RecipeNodeId::WORLD, var, &[])
        {
            pieces(doc, node, covers, &mut Vec::new(), &mut product);
        }
        let mut picture = Self {
            drawn,
            product: Vec::new(),
        };
        // The product slot's body is in the picture whatever reads it.
        picture.draw(product.iter().cloned());
        picture.product = product;
        picture
    }

    /// **The picture `doc` draws with `shown` drawn too**: [`Picture::of`],
    /// and every body of a node `shown` names drawn whole where it
    /// stands, a `⊔` as its pieces — an operand a boolean took over, a
    /// fillet's input, a piece of a hidden `⊔` drawn on its own. What a
    /// show adds is the picture's only: the product is the document's.
    pub fn showing(doc: &Doc<ProfileProgram>, shown: &Shown) -> Self {
        let mut picture = Self::of(doc);
        for &node in shown.nodes() {
            if !doc.node(node).is_some_and(builds_a_body) {
                continue;
            }
            let mut added = Vec::new();
            pieces(doc, node, Covers::all(), &mut Vec::new(), &mut added);
            picture.draw(added);
        }
        picture
    }

    /// Add `drawings`, each one the picture does not already hold: a
    /// drawing is drawn once however many reasons draw it.
    fn draw(&mut self, drawings: impl IntoIterator<Item = Drawn>) {
        for drawn in drawings {
            if !self.drawn.contains(&drawn) {
                self.drawn.push(drawn);
            }
        }
    }

    /// **Whether a drawing is of `node`'s body**: the body itself, or
    /// for a `⊔` one of its pieces.
    pub fn draws_node(&self, node: RecipeNodeId) -> bool {
        self.drawn.iter().any(|drawn| drawn.of_node(node))
    }

    /// **The spaces the picture shows** (DI5; D10 `[placement.spaces]`):
    /// the world, and every space that holds a drawn body, read off the
    /// recipe ([`Doc::node_space`]).
    pub fn shown_spaces(&self, doc: &Doc<ProfileProgram>) -> BTreeSet<VarId> {
        self.drawn
            .iter()
            .filter_map(|drawn| doc.node_space(drawn.node))
            .chain([VarId::WORLD])
            .collect()
    }

    /// The nodes some body of which is drawn, in document order, each
    /// once.
    pub fn nodes(&self) -> Vec<RecipeNodeId> {
        let mut seen = BTreeSet::new();
        self.drawn
            .iter()
            .map(|drawn| drawn.node)
            .filter(|&node| seen.insert(node))
            .collect()
    }

    /// Whether some body of `node` is drawn.
    pub fn is_drawn(&self, node: RecipeNodeId) -> bool {
        self.drawn.iter().any(|drawn| drawn.node == node)
    }

    /// Whether body `body` of `node` is drawn and the product does not
    /// reach it.
    pub fn is_outside(&self, node: RecipeNodeId, body: u32) -> bool {
        !self.product.iter().any(|piece| piece.draws(node, body))
    }

    /// **Which bodies of `node` the drawings cover**, by body index, or
    /// `None` for every body of its value: a node drawn whole, or one
    /// whose bodies the recipe does not count ([`body_count`]), which
    /// is then built whole and narrowed by [`Drawn::draws`].
    pub fn indices(&self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Option<Vec<u32>> {
        let mut indices = BTreeSet::new();
        for covers in self
            .drawn
            .iter()
            .filter(|d| d.node == node)
            .map(|d| &d.covers)
        {
            match covers {
                Covers::Only(bodies) => indices.extend(bodies),
                Covers::AllBut(taken) if taken.is_empty() => return None,
                Covers::AllBut(taken) => {
                    indices.extend((0..body_count(doc, node)?).filter(|k| !taken.contains(k)));
                }
            }
        }
        Some(indices.into_iter().collect())
    }
}

/// **The bodies the picture draws** ([`Picture::of`]'s drawings).
pub fn drawn_bodies(doc: &Doc<ProfileProgram>) -> Vec<Drawn> {
    Picture::of(doc).drawn
}

/// **`node`'s `covers`, drawn**: as itself, or for a `⊔` as each operand
/// it reads, taken as the read takes it, `within` naming the `⊔`s
/// around it.
fn pieces(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    covers: Covers,
    within: &mut Vec<RecipeNodeId>,
    out: &mut Vec<Drawn>,
) {
    if matches!(doc.node(node), Some(Node::Assemble { .. })) && !within.contains(&node) {
        within.push(node);
        for (operand, taken) in takes(doc, node) {
            pieces(doc, operand, taken, within, out);
        }
        within.pop();
    } else {
        out.push(Drawn {
            node,
            covers,
            within: within.clone(),
        });
    }
}

/// **The spaces the picture shows** ([`Picture::shown_spaces`]).
pub fn shown_spaces(doc: &Doc<ProfileProgram>) -> BTreeSet<VarId> {
    Picture::of(doc).shown_spaces(doc)
}

/// **The construction whose own space `node`'s value lives in**, read
/// off its recipe (D10 `[placement.spaces]`): `None` for a value in the
/// world's space, in a `new_space()`, or in no space. A placement whose
/// target is a construction's space is a relation to that
/// construction's body; a construction answers itself.
pub fn construction_of_space(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
) -> Option<RecipeNodeId> {
    let space = doc.node_space(node)?;
    let (construction, _) = doc.defined_by(space)?;
    (!matches!(doc.node(construction)?, Node::World | Node::Space)).then_some(construction)
}

/// **The nodes the picture draws some body of** ([`Picture::nodes`]).
pub fn drawn_nodes(doc: &Doc<ProfileProgram>) -> Vec<RecipeNodeId> {
    Picture::of(doc).nodes()
}

/// **Whether the picture draws some body of `node`** ([`Picture::is_drawn`]).
pub fn is_drawn(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> bool {
    Picture::of(doc).is_drawn(node)
}

/// **The spaces a gesture's copies stand in**: the world, and every
/// `new_space()` no placement reads a source body of. A placement into
/// one of them is a copy a gesture authors through and takes over
/// ([`copy_source`], [`placements_of_var`]); one into a space whose
/// bodies a placement copies out — a body moved against its own
/// geometry, a pattern in its body's own space — is a relation inside
/// that space, and a gesture reads it as the node it is.
///
/// This is the gesture vocabulary's own rule, not the picture's: what a
/// takeover re-points is the takeover's to decide
/// (`work/doors/`), and the picture draws by [`Picture::of`].
fn copy_spaces(doc: &Doc<ProfileProgram>) -> BTreeSet<VarId> {
    let placements = doc.placements();
    let read: BTreeSet<VarId> = placements
        .iter()
        .filter_map(|&placement| copied_var(doc, placement))
        .filter_map(|var| doc.space_of(var))
        .collect();
    placements
        .iter()
        .filter_map(|&placement| doc.node_space(placement))
        .chain([VarId::WORLD])
        .filter(|&space| {
            space == VarId::WORLD
                || (!read.contains(&space)
                    && doc
                        .defined_by(space)
                        .is_some_and(|(node, _)| matches!(doc.node(node), Some(Node::Space))))
        })
        .collect()
}

/// **The body a copy copies**: a `Place`'s source's read; `None` for
/// any other node, an instance included — it copies its part's product,
/// no body of this document.
pub fn copied_var(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Option<VarId> {
    match doc.node(node)? {
        Node::Place { source, .. } => Some(source.read),
        _ => None,
    }
}

/// **What a copy's body copies**: the variable the placement reads, the
/// operation defining it and the port, and the body of that
/// operation's value its body `body` is a copy of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Source {
    /// The variable read: a body, or a family for an indexed read.
    pub var: VarId,
    /// The operation defining it.
    pub operation: RecipeNodeId,
    /// Its port.
    pub port: u8,
    /// The body of the operation's value copied: the member an indexed
    /// read names, the member of a family copied whole, the port's
    /// body otherwise.
    pub body: u32,
    /// Whether the read names members of a family (`xs[i]`).
    pub member: bool,
}

/// **The one copy-to-body door**: what body `body` of `node` copies,
/// where `node` is a `Place` into a space a gesture's copies stand in
/// ([`copy_spaces`]); `None` for any other node, which a pick and a
/// seat take as itself.
pub fn copy_source(doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: u32) -> Option<Source> {
    let Some(Node::Place { source, .. }) = doc.node(node) else {
        return None;
    };
    if !copy_spaces(doc).contains(&doc.node_space(node)?) {
        return None;
    }
    let (operation, port) = doc.defined_by(source.read)?;
    let member = source.is_indexed();
    let body = if member {
        let members = members_read(doc, node, operation, &source.at)?;
        *members
            .get(usize::try_from(body).ok()?)
            .or(members.first())?
    } else if doc.node_reach(operation).is_none() {
        u32::from(port)
    } else {
        body
    };
    Some(Source {
        var: source.read,
        operation,
        port,
        body,
        member,
    })
}

/// **The body the copy `node` places**, a gesture's copy of one body
/// ([`copy_source`]): `None` for any other node, and for a copy of
/// members read by index, which a seat takes as the copy itself — the
/// variable it reads is the family, and a seat over it would read every
/// member.
pub fn placed_var(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Option<VarId> {
    copy_source(doc, node, 0)
        .filter(|source| !source.member)
        .map(|source| source.var)
}

/// **The copies of `var`**, in document order: the gesture's copies
/// ([`copy_source`]) whose placement reads it.
pub fn placements_of_var(doc: &Doc<ProfileProgram>, var: VarId) -> Vec<RecipeNodeId> {
    copies_where(doc, |copied| copied == var)
}

/// **The copies of any output of `node`**, in document order.
pub fn placements_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Vec<RecipeNodeId> {
    let outputs = doc.outputs(node);
    copies_where(doc, |copied| outputs.contains(&copied))
}

/// The placements into the spaces a gesture's copies stand in whose
/// source's read `copies` accepts, in document order.
fn copies_where(doc: &Doc<ProfileProgram>, copies: impl Fn(VarId) -> bool) -> Vec<RecipeNodeId> {
    let spaces = copy_spaces(doc);
    doc.placements()
        .into_iter()
        .filter(|&placement| {
            doc.node_space(placement)
                .is_some_and(|s| spaces.contains(&s))
        })
        .filter(|&placement| copied_var(doc, placement).is_some_and(&copies))
        .collect()
}

/// **Whether a copy copies an output of `node`** — what the feature
/// tree's placed badge marks.
pub fn is_placed(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> bool {
    !placements_of(doc, node).is_empty()
}

/// **A pick on a copy, read on its body**: the operation defining the
/// body `node` places, that body's index within the operation's value —
/// the member an indexed copy names — and `name`, which a copy carries
/// as its body does.
///
/// A pick on anything else is already on a body — a construction, an
/// operator's result, a relation inside a space — and comes back as it
/// was. A placement whose body is gone keeps its own node: there is no
/// body to read the pick on.
pub fn on_body(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    body: u32,
    name: &StableName,
) -> (RecipeNodeId, u32, StableName) {
    let (operation, body) = body_of(doc, node, body);
    (operation, body, name.clone())
}

/// **The body a picked body is**: through [`copy_source`] for a copy,
/// as it is otherwise.
pub fn body_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: u32) -> (RecipeNodeId, u32) {
    copy_source(doc, node, body).map_or((node, body), |source| (source.operation, source.body))
}

/// **The node a seat takes for a pick on `node`**: the operation whose
/// body a copy places ([`placed_var`]), so a tool authors against the
/// body and not against its copy; any other node as it is.
pub fn seat_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> RecipeNodeId {
    seat_read(doc, node, None).0
}

/// **The read a seat takes for a pick on `node`**: the operation and
/// port whose body a copy places ([`placed_var`]), or `node` itself
/// with the output a viewport pick hit (`body`). The port is kept only
/// for an operation that defines several outputs (a split's halves);
/// one with a single output is read by naming it alone.
pub fn seat_read(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    body: Option<u32>,
) -> (RecipeNodeId, Option<u8>) {
    let (node, port) = match copy_source(doc, node, 0).filter(|source| !source.member) {
        Some(source) => (source.operation, Some(source.port)),
        None => (node, body.and_then(|b| u8::try_from(b).ok())),
    };
    // A port is named only where the node holds several outputs: the
    // document's own rule for a read naming a node alone
    // (`Doc::read_of_node`), under which a construction's space is no
    // second output.
    let several = doc.read_of_node(node).is_none() && !doc.outputs(node).is_empty();
    (node, port.filter(|_| several))
}

/// **What a feature gesture's result takes over**: the read `target`
/// denotes in a body seat, and the placements reading it. Empty
/// for a target nothing places, which places nothing.
pub fn placements_of_target(doc: &Doc<ProfileProgram>, target: RecipeNodeId) -> Vec<RecipeNodeId> {
    doc.read_of_node(target)
        .map(|var| placements_of_var(doc, var))
        .unwrap_or_default()
}

/// **The output `read` names**: its operation, and the port where the
/// read names one. `None` for a selection, which is no output, and for
/// a variable the document does not define.
pub fn read_output(
    doc: &Doc<ProfileProgram>,
    read: &Operand,
) -> Option<(RecipeNodeId, Option<u8>)> {
    let defined = |var: VarId| {
        doc.var(var)?
            .def()
            .output()
            .map(|(node, port)| (node, Some(port)))
    };
    match read {
        Operand::Node(node) => Some((*node, None)),
        Operand::Output { node, port } => Some((*node, Some(*port))),
        Operand::Var(var) => defined(*var),
        Operand::Name(name) => defined(doc.var_named(name.as_str())?),
        Operand::Select { .. } | Operand::Pose(_) | Operand::Pick { .. } => None,
    }
}

/// **The placements reading the body `read` names**: a node
/// named alone reads its one output, a port the output it names.
pub fn placements_of_read(doc: &Doc<ProfileProgram>, read: &Operand) -> Vec<RecipeNodeId> {
    read_var(doc, read)
        .map(|var| placements_of_var(doc, var))
        .unwrap_or_default()
}

/// **The body variable `read` names**: a node named alone reads its
/// one output, a port the output it names. `None` for a selection, a
/// pose or a pick, and for a read the document does not define.
pub fn read_var(doc: &Doc<ProfileProgram>, read: &Operand) -> Option<VarId> {
    match read {
        Operand::Node(node) => doc.read_of_node(*node),
        Operand::Output { node, port } => doc.output(*node, *port),
        Operand::Var(var) => Some(*var),
        Operand::Name(name) => doc.var_named(name.as_str()),
        Operand::Select { .. } | Operand::Pose(_) | Operand::Pick { .. } => None,
    }
}

/// **The bodies the product reads** (D10 `[operations]`): the
/// operands of the `⊔` the product slot reads, in order, or the one
/// body it reads; empty where it reads nothing or a body that is gone.
/// The pieces of the same reading, one `⊔` deep — [`Picture::product`]
/// is the whole of it, every `⊔` as its pieces.
pub fn product_reads(doc: &Doc<ProfileProgram>) -> Vec<VarId> {
    let Some(var) = doc.product() else {
        return Vec::new();
    };
    match doc.operation_of(var).and_then(|at| doc.node(at)) {
        None => Vec::new(),
        Some(Node::Assemble { members }) => members
            .rows()
            .into_iter()
            .map(|(_, read)| read.read)
            .collect(),
        Some(_) => vec![var],
    }
}

/// **What the product slot reaches**, as the picture draws it
/// ([`Picture::product`]).
pub fn product_pieces(doc: &Doc<ProfileProgram>) -> Vec<Drawn> {
    Picture::of(doc).product
}

/// **The copy a world placement defines**, the body a product can read:
/// its output where `node` is a `Place` or an instance standing in the
/// world; `None` for any other node.
pub fn world_copy(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Option<VarId> {
    if !matches!(
        doc.node(node),
        Some(Node::Place { .. } | Node::InstantiatePart { .. })
    ) {
        return None;
    }
    doc.output(node, 0)
        .filter(|&copy| doc.space_of(copy) == Some(VarId::WORLD))
}

/// **The product with `copy` in it**: what it reads, then `copy`, as
/// the bodies a [`crate::session::SessionOp::SetProduct`] says.
pub fn product_with(doc: &Doc<ProfileProgram>, copy: VarId) -> Vec<Operand> {
    product_reads(doc)
        .into_iter()
        .filter(|&read| read != copy)
        .chain([copy])
        .map(Operand::Var)
        .collect()
}

/// **The product with `copy` taken out**: what it reads, less `copy`.
pub fn product_without(doc: &Doc<ProfileProgram>, copy: VarId) -> Vec<Operand> {
    product_reads(doc)
        .into_iter()
        .filter(|&read| read != copy)
        .map(Operand::Var)
        .collect()
}
