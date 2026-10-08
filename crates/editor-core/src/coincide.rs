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
//! **One rung today, [`Rung::SameSource`]**: the two cells' carriers
//! hold one recipe source ([`topo::GeomSource`]) once their placements
//! are composed, which the sources carry ([`topo::source`]'s
//! `Placed`). That is what the kernel's own structural rung proves, so
//! the door proves exactly what the kernel already calls structural.
//! Where a cell's carrier is read lives in [`carrier_source`] alone.

use core::fmt;

use geom_core::{Decide, MarginDiag};

use crate::eval::Evaluation;
use crate::names::{EntityKey, EntityRef, Entry, NameTable, NamingError, StableName};
use crate::node::RecipeNodeId;

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
    /// The two cells' carriers are one recipe source once their
    /// placements are composed.
    SameSource,
}

/// **What separates an unproven row's two constructions**: each cell's
/// carrier source, `None` where the cell has none (a tool plane, or a
/// carrier no operation stamped).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Residual {
    /// Each cell's carrier source, in the row's cell order.
    pub sources: [Option<topo::GeomSource>; 2],
}

impl fmt::Display for Residual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.sources {
            [Some(_), Some(_)] => f.write_str("the two cells are two constructions"),
            [None, None] => f.write_str("neither cell's carrier is a recorded construction"),
            [None, Some(_)] | [Some(_), None] => {
                f.write_str("one cell's carrier is not a recorded construction")
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

/// **The door**: whether `row`, a coincidence a node of `eval` decided,
/// holds structurally. The rungs are tried in order (module docs).
#[must_use]
pub fn prove<T: Decide>(eval: &Evaluation<T>, row: &NamedCoincidence) -> Proof {
    let sources = row.cells.each_ref().map(|cell| carrier_source(eval, cell));
    match &sources {
        [Some(a), Some(b)] if a.same_base(b) => Proof::Structural(Rung::SameSource),
        _ => Proof::Unproven {
            residual: Residual { sources },
            recourse: Recourse::OneConstruction,
        },
    }
}

/// The recipe source of `cell`'s carrier in `eval`: a face's surface,
/// an edge's curve, a vertex's point. `None` for a tool, a name that no
/// longer resolves to one entity, or a carrier with no source.
fn carrier_source<T: Decide>(eval: &Evaluation<T>, cell: &NamedCell) -> Option<topo::GeomSource> {
    let NamedCell::Entity { input, name } = cell else {
        return None;
    };
    let value = eval.value(*input)?;
    let Some(Entry::Unique(EntityRef { body, key })) = value.name_table.lookup(name) else {
        return None;
    };
    let (_, held, ..) = crate::product::sources_of(value)?
        .into_iter()
        .find(|(ix, ..)| ix == body)?;
    match *key {
        EntityKey::Face(f) => held.surface_source(held.get_face(f)?.surface),
        EntityKey::Edge(e) => held.curve_source(held.get_edge(e)?.curve),
        EntityKey::Vertex(v) => held.point_source(held.get_vertex(v)?.point),
        EntityKey::Body => None,
    }
    .cloned()
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
