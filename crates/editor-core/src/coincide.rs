//! **The coincidence door** (D10): the one place where it is decided
//! whether a coincidence an operation decided from values holds
//! structurally — across the family, not only at the current values.
//!
//! The kernel records each value decision as a [`topo::Coincidence`]
//! keyed in the deciding operation's inputs. The evaluation names each
//! row's cells by their [`StableName`]s in those inputs' tables
//! ([`NamedCoincidence`], carried on [`crate::eval::NodeValue`]), so a
//! row survives whatever the operation, or a later one, does to its
//! cells. [`prove`] tries the door's rungs in order, and a rung may
//! only prove more than the rungs before it. The
//! `unproven-coincidence` check ([`crate::CheckId::UnprovenCoincidence`])
//! reports each row the door leaves [`Proof::Unproven`].
//!
//! **One rung today, [`Rung::SameConstruction`]**: the same
//! construction read twice. Each cell is walked from the read it
//! entered the deciding operation through, down its name's
//! carry-through segments and the placements its read passes, to the
//! node that minted it ([`construction`]); two cells are one
//! construction when they reach one minting role through one chain of
//! placements. Provenance is the document's: the walk reads the recipe
//! and the names, never a stamp the kernel carries.

use core::fmt;

use geom_core::MarginDiag;

use crate::names::{EntityKey, EntityRef, NameTable, NamingError, RoleSeg, StableName};
use crate::node::{Node, RecipeNodeId};

/// **One coincidence an operation decided from values**, its cells
/// named in the tables of the inputs the decision read.
#[derive(Clone, Debug, PartialEq)]
pub struct NamedCoincidence {
    /// The two cells decided one, in the order the decision read them.
    pub cells: [NamedCell; 2],
    /// What was decided between them.
    pub relation: topo::Relation,
    /// Where it was decided.
    pub site: topo::DecisionSite,
    /// The margin that decision read: Zero, or in band where a
    /// declaration bridged it. For reporting only.
    pub margin: MarginDiag,
}

/// One cell of a [`NamedCoincidence`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NamedCell {
    /// An entity of an input body of the deciding node, by its name in
    /// that input's table (its output body 0).
    Entity {
        /// The input node whose table names it.
        input: RecipeNodeId,
        /// Its name there.
        name: StableName,
    },
    /// The plane the deciding node cuts with: its tool input, a datum
    /// rather than a cell of any body.
    Tool {
        /// The tool node.
        input: RecipeNodeId,
    },
}

/// **What the door decided about a row.**
#[derive(Clone, Debug, PartialEq)]
pub enum Proof {
    /// The coincidence holds structurally, proved by this rung.
    Structural(Rung),
    /// No rung proves it: it holds at the current values only.
    Unproven {
        /// What separates the two cells' constructions.
        residual: Residual,
        /// What would make the coincidence structural.
        recourse: Recourse,
    },
}

/// The rung of the door that proved a row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rung {
    /// The two cells are one construction read twice: one minting role,
    /// reached through one chain of placements.
    SameConstruction,
}

/// **A cell's construction**, as the door reads it off the document:
/// the name the minting node gave the entity (its role there, piece
/// qualifiers dropped, since every piece of an entity lies on its one
/// carrier), and the placements met on the way from the read to it,
/// outermost first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Construction {
    /// The entity as its minting node named it.
    pub minted: StableName,
    /// The placements between the minting node and the read.
    pub placed: Vec<Placed>,
}

/// One placement a cell's read passes on the way to its minting node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Placed {
    /// A `Transform`'s one map.
    Transform(RecipeNodeId),
    /// A pattern's (or a placed union's) instance `i`.
    Instance(RecipeNodeId, u32),
}

/// **What separates an unproven row's two constructions**: each cell's
/// construction, `None` where the walk reaches none (a tool plane, a
/// read through an operation that is not a placement, a name the walk
/// cannot classify).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Residual {
    /// Each cell's construction, in the row's cell order.
    pub constructions: [Option<Construction>; 2],
}

impl fmt::Display for Residual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.constructions {
            [Some(_), Some(_)] => f.write_str("the two cells are two constructions"),
            [None, None] => f.write_str("neither cell's carrier is read to a construction"),
            [None, Some(_)] | [Some(_), None] => {
                f.write_str("one cell's carrier is not read to a construction")
            }
        }
    }
}

/// **What would make an unproven row structural.**
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Recourse {
    /// Build the two cells as one construction of the same variables,
    /// or assert the coincidence at its site.
    OneConstruction,
}

impl Recourse {
    /// The recourse as a sentence's ending.
    #[must_use]
    pub const fn words(self) -> &'static str {
        match self {
            Self::OneConstruction => {
                "Recourse: build the two cells as one construction of the same variables, so \
                 the coincidence holds whatever their values, or assert it at its site"
            }
        }
    }
}

/// A relation as the clause between a row's two cells.
pub(crate) const fn relation_words(relation: topo::Relation) -> &'static str {
    match relation {
        topo::Relation::SameOriented => "continues on one carrier with",
        topo::Relation::SameOpposite => "rests on one carrier against",
        topo::Relation::OnCarrier => "lies on",
        topo::Relation::EqualAngles => "makes an equal angle at its turn with",
    }
}

/// A decision site in words.
pub(crate) const fn site_words(site: topo::DecisionSite) -> &'static str {
    match site {
        topo::DecisionSite::PlaneLadder => "a declared pair of planes read as one",
        topo::DecisionSite::CarrierLadder => "a declared pair of carriers read as one",
        topo::DecisionSite::SplitOn => "a split's on-plane verdict where its pieces touch",
        topo::DecisionSite::BatteryTurn => "a blend's isosceles turn",
    }
}

/// **The door**: whether `row`, a coincidence a node of `doc` decided,
/// holds structurally. The rungs are tried in order (module docs).
#[must_use]
pub fn prove<P>(doc: &crate::doc::Doc<P>, row: &NamedCoincidence) -> Proof {
    let constructions = row.cells.each_ref().map(|cell| match cell {
        NamedCell::Entity { input, name } => construction(doc, *input, name),
        NamedCell::Tool { .. } => None,
    });
    match &constructions {
        [Some(a), Some(b)] if a == b => Proof::Structural(Rung::SameConstruction),
        _ => Proof::Unproven {
            residual: Residual { constructions },
            recourse: Recourse::OneConstruction,
        },
    }
}

/// **The construction of the entity `name` names in `read`'s value**:
/// the walk down from the read to the node that minted it.
///
/// At each node the walk either passes through whole (a `Transform`,
/// which places, or a `Part` or a split that leaves the entity intact,
/// neither of which adds a name segment, N1) or reads the name's
/// outermost segment there: a minted role ends the walk, and a carried
/// one (`FromA`, `FromMember`, `Instance`, …) names the entity in the
/// input the segment says, where the walk continues. `None` where the
/// read passes an operation that is neither, or a segment the
/// partition does not place (`names::attribute`).
#[must_use]
pub fn construction<P>(
    doc: &crate::doc::Doc<P>,
    read: RecipeNodeId,
    name: &StableName,
) -> Option<Construction> {
    use crate::names::attribute::{SegOrigin, origin};
    let mut placed = Vec::new();
    let (mut at, mut name) = (read, name.clone());
    loop {
        while at != name.node {
            at = match doc.node(at)? {
                Node::Transform { input, .. } => {
                    placed.push(Placed::Transform(at));
                    *input
                }
                Node::Part { of, .. } => *of,
                Node::Split { target, .. } => *target,
                _ => return None,
            };
        }
        let seg = name.path.first()?;
        let of = match origin(seg) {
            SegOrigin::Minted => {
                let mut minted = name;
                minted
                    .path
                    .truncate(crate::names::role::fragment_tail_start(&minted.path));
                return Some(Construction { minted, placed });
            }
            SegOrigin::Unclassified => return None,
            SegOrigin::Carried(of, _) => of.clone(),
        };
        at = match (seg, doc.node(at)?) {
            (RoleSeg::FromA(_), Node::Boolean { a, .. }) => *a,
            (RoleSeg::FromB(_), Node::Boolean { b, .. }) => *b,
            (RoleSeg::FromMember { member, .. }, _) => *member,
            (RoleSeg::Instance { i, .. }, node) => {
                placed.push(Placed::Instance(at, *i));
                body_input(node)?
            }
            (_, node) => body_input(node)?,
        };
        name = of;
    }
}

/// The one body a single-operand operation reads, or `None`.
fn body_input<P>(node: &Node<P>) -> Option<RecipeNodeId> {
    match node {
        Node::Fillet { target, .. }
        | Node::Chamfer { target, .. }
        | Node::Shell { target, .. }
        | Node::Split { target, .. } => Some(*target),
        Node::Pattern { input, .. }
        | Node::PlacedUnion { input, .. }
        | Node::Transform { input, .. } => Some(*input),
        Node::Part { of, .. } => Some(*of),
        _ => None,
    }
}

/// The deciding node's inputs a row's cells are keyed in: the node
/// and table each [`topo::Operand`] names, and the node's tool, if it
/// cuts with one.
pub(crate) struct RowInputs<'a> {
    /// Operand A's node and table (a one-body operation's input).
    pub a: (RecipeNodeId, &'a NameTable),
    /// Operand B's, for an operation that reads two bodies.
    pub b: Option<(RecipeNodeId, &'a NameTable)>,
    /// The tool node, for an operation that cuts with one.
    pub tool: Option<RecipeNodeId>,
}

/// **The kernel's rows, named**: each cell by its name in the input
/// table its key is in.
///
/// # Errors
///
/// [`NamingError::Emission`] for a cell no input of the node holds by
/// one name: the kernel recorded a cell of a body it did not read, or
/// one the input's table does not name, which is a kernel bug.
pub(crate) fn name_rows(
    rows: &[topo::Coincidence],
    inputs: &RowInputs<'_>,
) -> Result<Vec<NamedCoincidence>, NamingError> {
    const UNNAMED: &str = "a coincidence row names a cell no input of its node names";
    let name = |cell: &topo::RowCell| -> Result<NamedCell, NamingError> {
        let unnamed = || NamingError::Emission { what: UNNAMED };
        match *cell {
            topo::RowCell::Tool => inputs
                .tool
                .map(|input| NamedCell::Tool { input })
                .ok_or_else(unnamed),
            topo::RowCell::Input { input, cell } => {
                let (node, table) = match input {
                    topo::Operand::A => Some(inputs.a),
                    topo::Operand::B => inputs.b,
                }
                .ok_or_else(unnamed)?;
                let key = match cell {
                    topo::Cell::Vertex(v) => EntityKey::Vertex(v),
                    topo::Cell::Edge(e) => EntityKey::Edge(e),
                    topo::Cell::Face(f) => EntityKey::Face(f),
                };
                let name = table
                    .name_of(&EntityRef { body: 0, key })
                    .ok_or_else(unnamed)?;
                Ok(NamedCell::Entity {
                    input: node,
                    name: name.clone(),
                })
            }
        }
    };
    rows.iter()
        .map(|row| {
            Ok(NamedCoincidence {
                cells: [name(&row.cells[0])?, name(&row.cells[1])?],
                relation: row.relation,
                site: row.site,
                margin: row.margin,
            })
        })
        .collect()
}
