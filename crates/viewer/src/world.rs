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
    Doc, FreeVar, Node, Operand, OperandSlot, PortKind, ProfileProgram, RecipeNodeId, VarDef,
    VarId, VarKind,
};
use pncad::prelude::StableName;

/// **Which of a node's bodies a read takes, or a drawing covers**, by
/// the body index the node's value holds them at (a family's members in
/// index order, a split's halves by `SplitHalf::output_body`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Covers {
    /// Every body of the value but these.
    AllBut(BTreeSet<u32>),
    /// The one body at this index.
    One(u32),
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
            Self::One(at) => *at == body,
        }
    }
}

/// **One body the picture draws**: the bodies of `node`'s value it
/// covers, and the `⊔`s it is drawn as a piece of, outermost first
/// (empty for a body drawn on its own).
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
}

/// **What `var`, read as a body, takes**: the operation defining it and
/// which of its bodies. `at` is the read's indices (`xs[i]`), empty for
/// a plain read. A selection takes the body it is read in; a pick of a
/// piece of a copy takes the copy, which is drawn as one body. `None`
/// for a variable that is no body's.
fn taken_by(doc: &Doc<ProfileProgram>, var: VarId, at: &[VarId]) -> Option<(RecipeNodeId, Covers)> {
    match doc.var(var)?.def() {
        VarDef::Output { node, port } => {
            let covers = if matches!(doc.node(*node)?, Node::Split { .. }) {
                Covers::One(u32::from(*port))
            } else {
                member_at(doc, at).map_or_else(Covers::all, Covers::One)
            };
            Some((*node, covers))
        }
        VarDef::Select(select) => taken_by(doc, select.body, &[]),
        VarDef::Pick(pick) => taken_by(doc, pick.placement, &[]),
        _ => None,
    }
}

/// **The member a read's indices name**, where the recipe writes it: one
/// index, a count written as a constant. `None` for a plain read, and
/// for a read whose member only an evaluation knows (an index read at
/// another index, a family of several indices): that read is taken to
/// read the family whole.
fn member_at(doc: &Doc<ProfileProgram>, at: &[VarId]) -> Option<u32> {
    let [index] = at else {
        return None;
    };
    let value = match doc.var(*index)?.def() {
        VarDef::Free(FreeVar::Count { value }) => *value,
        VarDef::Defined(expr) => expr.as_integer()?,
        _ => return None,
    };
    u32::try_from(value).ok()
}

/// **How many bodies `node` builds**, where the recipe says: a split's
/// two halves, or a family's members over one index whose count is
/// written as a constant. `None` otherwise, and then a node whose
/// bodies are each read alone is still drawn, with no body, in its
/// space.
fn body_count(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Option<u32> {
    if matches!(doc.node(node)?, Node::Split { .. }) {
        return Some(2);
    }
    let reach = doc.node_reach(node);
    let [index] = reach.chain()? else {
        return None;
    };
    let count = &doc.var(*index)?.def().index()?.count;
    u32::try_from(doc.written(count).as_integer()?).ok()
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

/// **The reads `node` takes bodies by**, each resolved to what it takes
/// ([`taken_by`]), in field order.
fn takes(doc: &Doc<ProfileProgram>, node: &Node<ProfileProgram>) -> Vec<(RecipeNodeId, Covers)> {
    node.operand_rows()
        .into_iter()
        .filter(|&(seat, _)| reads_as_body(seat))
        .filter_map(|(seat, var)| {
            let at = node
                .body_read(seat)
                .map(|read| read.at.clone())
                .unwrap_or_default();
            taken_by(doc, var, &at)
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

/// **The bodies the picture draws**, in document order (module docs):
/// every body no operation reads as a body, a family less the members
/// read alone, and a `⊔` as its pieces.
pub fn drawn_bodies(doc: &Doc<ProfileProgram>) -> Vec<Drawn> {
    // Per node: `None` where some read takes it whole, else the bodies
    // read alone.
    let mut taken: BTreeMap<RecipeNodeId, Option<BTreeSet<u32>>> = BTreeMap::new();
    for id in doc.ids() {
        let Some(node) = doc.node(id) else { continue };
        for (at, covers) in takes(doc, node) {
            let entry = taken.entry(at).or_insert_with(|| Some(BTreeSet::new()));
            match (covers, entry.as_mut()) {
                (Covers::One(body), Some(bodies)) => {
                    bodies.insert(body);
                }
                _ => *entry = None,
            }
        }
    }
    let mut drawn = Vec::new();
    for id in doc.ids() {
        let Some(node) = doc.node(id) else { continue };
        if !builds_a_body(node) {
            continue;
        }
        let covers = match taken.get(&id) {
            None => Covers::all(),
            // Every body read alone is the node read whole.
            Some(Some(bodies))
                if body_count(doc, id)
                    .is_some_and(|count| (0..count).all(|k| bodies.contains(&k))) =>
            {
                continue;
            }
            Some(Some(bodies)) => Covers::AllBut(bodies.clone()),
            Some(None) => continue,
        };
        pieces(doc, id, covers, &mut Vec::new(), &mut drawn);
    }
    drawn
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
    match doc.node(node) {
        Some(assemble @ Node::Assemble { .. }) if !within.contains(&node) => {
            within.push(node);
            for (operand, taken) in takes(doc, assemble) {
                pieces(doc, operand, taken, within, out);
            }
            within.pop();
        }
        _ => out.push(Drawn {
            node,
            covers,
            within: within.clone(),
        }),
    }
}

/// **Which bodies of `node` the drawings in `drawn` cover**, by body
/// index, or `None` for every body of its value: a node drawn whole, or
/// a family less members read alone whose count the recipe does not
/// write, which is then built whole and narrowed by
/// [`Drawn::draws`].
pub fn drawn_indices(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    drawn: &[Drawn],
) -> Option<Vec<u32>> {
    let mut indices = BTreeSet::new();
    for covers in drawn.iter().filter(|d| d.node == node).map(|d| &d.covers) {
        match covers {
            Covers::One(body) => {
                indices.insert(*body);
            }
            Covers::AllBut(taken) if taken.is_empty() => return None,
            Covers::AllBut(taken) => {
                indices.extend((0..body_count(doc, node)?).filter(|k| !taken.contains(k)));
            }
        }
    }
    Some(indices.into_iter().collect())
}

/// **The spaces the picture shows** (DI5; D10 `[placement.spaces]`):
/// the world, and every space that holds a drawn body
/// ([`drawn_bodies`]), read off the recipe ([`Doc::node_space`]).
pub fn shown_spaces(doc: &Doc<ProfileProgram>) -> BTreeSet<VarId> {
    drawn_bodies(doc)
        .iter()
        .filter_map(|drawn| doc.node_space(drawn.node))
        .chain([VarId::WORLD])
        .collect()
}

/// **What the product slot reaches**, as the picture draws it: the body
/// it reads, a `⊔` as its pieces ([`drawn_bodies`]' rule). Empty where
/// it reads nothing. A drawn body none of these covers is outside the
/// product, and the picture tints it so.
pub fn product_pieces(doc: &Doc<ProfileProgram>) -> Vec<Drawn> {
    let mut out = Vec::new();
    if let Some((node, covers)) = doc.product().and_then(|var| taken_by(doc, var, &[])) {
        pieces(doc, node, covers, &mut Vec::new(), &mut out);
    }
    out
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

/// **The nodes the picture draws some body of** ([`drawn_bodies`]), in
/// document order, each once.
pub fn drawn_nodes(doc: &Doc<ProfileProgram>) -> Vec<RecipeNodeId> {
    let mut seen = BTreeSet::new();
    drawn_bodies(doc)
        .into_iter()
        .map(|drawn| drawn.node)
        .filter(|&node| seen.insert(node))
        .collect()
}

/// **Whether the picture draws some body of `node`** ([`drawn_bodies`]).
pub fn is_drawn(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> bool {
    drawn_bodies(doc).iter().any(|drawn| drawn.node == node)
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

/// **The body the drawn copy `node` places** ([`copied_var`] of a
/// `Place` the picture draws), or `None` for any other node: a pick
/// lands on a drawn copy only, and a copy the picture does not draw is
/// a body some operation reads, which a seat takes as itself. A family
/// of copies (a placement evaluated once per member) answers `None`
/// too: a pick on one of its members is on the family, which is what
/// a seat over it takes.
pub fn placed_var(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Option<VarId> {
    copied_var(doc, node).filter(|_| is_copy(doc, node) && is_drawn(doc, node))
}

/// **Whether `node` is one copy**: a `Place` evaluated once, not a
/// family of copies (a placement whose reads reach an index).
fn is_copy(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> bool {
    matches!(doc.node(node), Some(Node::Place { .. })) && doc.node_reach(node).is_none()
}

/// **The drawn copies of `var`**, in document order: the placements
/// whose source reads it and whose copy the picture draws, each one
/// copy ([`placed_var`]'s reading).
pub fn placements_of_var(doc: &Doc<ProfileProgram>, var: VarId) -> Vec<RecipeNodeId> {
    drawn_copies_where(doc, |copied| copied == var)
}

/// **The drawn copies of any output of `node`**, in document order.
pub fn placements_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Vec<RecipeNodeId> {
    let outputs = doc.outputs(node);
    drawn_copies_where(doc, |copied| outputs.contains(&copied))
}

/// The drawn copies ([`placed_var`]) whose source's read `copies`
/// accepts, in document order, each once.
fn drawn_copies_where(
    doc: &Doc<ProfileProgram>,
    copies: impl Fn(VarId) -> bool,
) -> Vec<RecipeNodeId> {
    drawn_nodes(doc)
        .into_iter()
        .filter(|&node| is_copy(doc, node))
        .filter(|&node| copied_var(doc, node).is_some_and(&copies))
        .collect()
}

/// **Whether a drawn copy copies an output of `node`** — what the
/// feature tree's placed badge marks.
pub fn is_placed(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> bool {
    !placements_of(doc, node).is_empty()
}

/// **A pick on a copy, read on its body**: the operation defining the
/// body `node` places, that body's index within the operation's value,
/// and `name`, which a copy carries as its body does.
///
/// A pick whose node is not a drawn copy is already on a body — a
/// construction, an operator's result — and comes back as it was. A placement whose body is gone keeps its own
/// node: there is no body to read the pick on.
pub fn on_body(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    body: u32,
    name: &StableName,
) -> (RecipeNodeId, u32, StableName) {
    let (operation, body) = body_of(doc, node, body);
    (operation, body, name.clone())
}

/// **The body a drawn copy is**: for a placement's copy, the
/// operation defining the body it places and that body's index in the
/// operation's value; any other drawn body as it is.
pub fn body_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: u32) -> (RecipeNodeId, u32) {
    placed_var(doc, node)
        .and_then(|var| doc.defined_by(var))
        .map_or((node, body), |(operation, port)| {
            (operation, u32::from(port))
        })
}

/// **The node a seat takes for a pick on `node`**: the operation whose
/// body a placement places, so a tool authors against the body
/// and not against its copy; any other node as it is.
pub fn seat_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> RecipeNodeId {
    placed_var(doc, node)
        .and_then(|var| doc.operation_of(var))
        .unwrap_or(node)
}

/// **The read a seat takes for a pick on `node`**: the operation and
/// port whose body a placement places, or `node` itself with the
/// output a viewport pick hit (`body`). The port is kept only for an
/// operation that defines several outputs (a split's halves); one with
/// a single output is read by naming it alone.
pub fn seat_read(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    body: Option<u32>,
) -> (RecipeNodeId, Option<u8>) {
    let (node, port) = match placed_var(doc, node).and_then(|var| doc.defined_by(var)) {
        Some((operation, port)) => (operation, Some(port)),
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

/// **The node a creation made**, out of the ids its action minted: the
/// last one that is not a placement — a creation gesture places
/// what it made after making it, and the placement is the world's, not
/// the thing a label or a form is about — or, for a gesture whose
/// creation IS a placement (the transform's), the first it minted.
pub fn made(doc: &Doc<ProfileProgram>, minted: &[RecipeNodeId]) -> Option<RecipeNodeId> {
    minted
        .iter()
        .rev()
        .copied()
        .find(|&node| !matches!(doc.node(node), Some(Node::Place { .. })))
        .or_else(|| minted.first().copied())
}

/// **The bodies the product reads** (D10 `[operations]`): the operands
/// of the `⊔` the product slot reads, in order, or the one body it
/// reads; empty where it reads nothing or a body that is gone.
pub fn product_reads(doc: &Doc<ProfileProgram>) -> Vec<VarId> {
    let Some(var) = doc.product().filter(|&var| doc.operation_of(var).is_some()) else {
        return Vec::new();
    };
    match doc.operation_of(var).and_then(|at| doc.node(at)) {
        Some(Node::Assemble { members }) => members
            .rows()
            .into_iter()
            .map(|(_, read)| read.read)
            .collect(),
        _ => vec![var],
    }
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
