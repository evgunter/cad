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
//! construction when they reach one origin through one chain of
//! placements. A split's section face and its tool are one origin, the
//! tool plane's. Provenance is the document's: the walk reads the
//! recipe and the names, never a stamp the kernel carries.
//!
//! A profile's junction no constructor made is a row of the profile
//! node itself, its cells the two pieces that meet there
//! ([`NamedCell::Piece`]); each piece is its own construction, so no
//! rung of today's proves one.
//!
//! A row this rung leaves [`Proof::Unproven`] is not proven
//! structural; it may still hold structurally by an argument a later
//! rung makes (the carrier-pair verdict, the margin identity at `Sym`).

use core::fmt;

use geom_core::MarginDiag;

use crate::names::{
    EntityKey, EntityRef, NameTable, NamingError, ProfileEdgeRef, RoleSeg, StableName,
};
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
    /// declaration bridged it ([`topo::Coincidence::margin`]).
    pub margin: MarginDiag,
    /// How that margin's Zero was discharged.
    pub discharge: topo::Discharge,
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
    /// A piece of the deciding profile itself: a junction's two
    /// segments, each named by the piece it is (`names/README.md`, "N1,
    /// the profile pieces").
    Piece {
        /// The profile node.
        profile: RecipeNodeId,
        /// The piece.
        piece: ProfileEdgeRef,
    },
}

/// **What the door decided about a row.**
#[derive(Clone, Debug, PartialEq)]
pub enum Proof {
    /// The coincidence holds structurally, proved by this rung.
    Structural(Rung),
    /// No rung proves it structural. It holds at the current values;
    /// whether it holds across the family the door has not shown.
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
    /// The two cells are one construction read twice: one origin,
    /// reached through one chain of placements.
    SameConstruction,
}

/// **A cell's construction**, as the door reads it off the document:
/// where its carrier comes from ([`Origin`]) and the placements met on
/// the way from the read to it, outermost first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Construction {
    /// Where the cell's carrier comes from.
    pub origin: Origin,
    /// The placements between the origin and the read.
    pub placed: Vec<Placed>,
}

/// **Where a cell's carrier comes from.**
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Origin {
    /// An entity, as the node that minted it named it: its role there,
    /// piece qualifiers dropped, since every piece of an entity lies on
    /// its one carrier.
    Minted(StableName),
    /// A datum node's own plane: a split's tool, and every section face
    /// the split mints on it.
    Datum(RecipeNodeId),
    /// A profile's piece: the step of that profile that drew it, and
    /// its role there.
    Piece(RecipeNodeId, ProfileEdgeRef),
}

/// **Why the walk reached no construction.** Each arm names the node
/// it stopped at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unwalked {
    /// The read passes this node, which neither places, nor passes the
    /// entity through intact, nor minted the name.
    Through(RecipeNodeId),
    /// The name's outermost segment at this node is one the partition
    /// does not place (`names::attribute`), or the name has none.
    Unclassified(RecipeNodeId),
    /// A carried segment at this node, whose kind does not carry it
    /// (a `FromA` at a node that is not a boolean, say).
    Misplaced(RecipeNodeId),
    /// The document holds no node by this id.
    Absent(RecipeNodeId),
}

impl fmt::Display for Unwalked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Through(node) => write!(
                f,
                "its read passes {node}, which neither places it nor carries it through"
            ),
            Self::Unclassified(node) => {
                write!(f, "its name at {node} has no segment the walk places")
            }
            Self::Misplaced(node) => {
                write!(
                    f,
                    "its name at {node} carries a segment that node does not carry"
                )
            }
            Self::Absent(node) => write!(f, "the document holds no {node}"),
        }
    }
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
/// construction, or why the walk reached none.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Residual {
    /// Each cell's construction, in the row's cell order.
    pub constructions: [Result<Construction, Unwalked>; 2],
}

impl fmt::Display for Residual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.constructions {
            [Ok(a), Ok(b)] => match (a.origin == b.origin, a.placed == b.placed) {
                (false, false) => f.write_str("the two cells are two constructions, placed apart"),
                (false, true) => f.write_str("the two cells are two constructions"),
                (true, _) => f.write_str("the two cells are one construction placed apart"),
            },
            [Err(a), Err(b)] => write!(
                f,
                "neither cell is read to a construction: the first because {a}, the second \
                 because {b}"
            ),
            [Err(why), Ok(_)] => {
                write!(f, "the first cell is not read to a construction: {why}")
            }
            [Ok(_), Err(why)] => {
                write!(f, "the second cell is not read to a construction: {why}")
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
        topo::Relation::Tangent { aligned: true } => "continues tangent into",
        topo::Relation::Tangent { aligned: false } => "turns back tangent into",
    }
}

/// A decision site in words.
pub(crate) const fn site_words(site: topo::DecisionSite) -> &'static str {
    match site {
        topo::DecisionSite::PlaneLadder => "a declared pair of planes read as one",
        topo::DecisionSite::CarrierLadder => "a declared pair of carriers read as one",
        topo::DecisionSite::SplitOn => "a split's on-plane verdict where its pieces touch",
        topo::DecisionSite::BatteryTurn => "a blend's isosceles turn",
        topo::DecisionSite::ProfileJunction => "a profile junction no constructor made",
    }
}

/// **The door**: whether `row`, a coincidence a node of `doc` decided,
/// holds structurally. The rungs are tried in order (module docs).
#[must_use]
pub fn prove<P>(doc: &crate::doc::Doc<P>, row: &NamedCoincidence) -> Proof {
    let constructions = row.cells.each_ref().map(|cell| match cell {
        NamedCell::Entity { input, name } => construction(doc, *input, name),
        NamedCell::Tool { input } => datum(doc, *input, Vec::new()),
        NamedCell::Piece { profile, piece } => Ok(Construction {
            origin: Origin::Piece(*profile, *piece),
            placed: Vec::new(),
        }),
    });
    match &constructions {
        [Ok(a), Ok(b)] if a == b => Proof::Structural(Rung::SameConstruction),
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
/// outermost segment there. A minted role ends the walk, except a
/// split's section face, which lies on the split's tool and continues
/// there ([`Origin::Datum`]). A carried segment names the entity in the
/// input that segment says, where the walk continues.
///
/// # Errors
///
/// [`Unwalked`], naming the node the walk stopped at.
pub fn construction<P>(
    doc: &crate::doc::Doc<P>,
    read: RecipeNodeId,
    name: &StableName,
) -> Result<Construction, Unwalked> {
    use crate::names::attribute::{SegOrigin, origin};
    let node = |at: RecipeNodeId| doc.node(at).ok_or(Unwalked::Absent(at));
    let mut placed = Vec::new();
    let (mut at, mut name) = (read, name.clone());
    loop {
        while at != name.node {
            at = match node(at)? {
                Node::Transform { input, .. } => {
                    placed.push(Placed::Transform(at));
                    *input
                }
                Node::Part { of, .. } => *of,
                Node::Split { target, .. } => *target,
                _ => return Err(Unwalked::Through(at)),
            };
        }
        let seg = name.path.first().ok_or(Unwalked::Unclassified(at))?;
        let of = match origin(seg) {
            SegOrigin::Minted => {
                return match (seg, node(at)?) {
                    (RoleSeg::SectionFace { .. }, Node::Split { tool, .. }) => {
                        datum(doc, *tool, placed)
                    }
                    (RoleSeg::SectionFace { .. }, _) => Err(Unwalked::Misplaced(at)),
                    _ => {
                        let mut minted = name;
                        minted
                            .path
                            .truncate(crate::names::role::fragment_tail_start(&minted.path));
                        Ok(Construction {
                            origin: Origin::Minted(minted),
                            placed,
                        })
                    }
                };
            }
            SegOrigin::Unclassified => return Err(Unwalked::Unclassified(at)),
            SegOrigin::Carried(of, _) => of.clone(),
        };
        at = carried_input(seg, node(at)?, at, &mut placed)?;
        name = of;
    }
}

/// **The input a carried segment at `at` names its entity in**, by the
/// segment and the node together. A placing segment (`Instance`) adds
/// its placement.
///
/// # Errors
///
/// [`Unwalked::Misplaced`] for a segment `at`'s kind does not carry.
fn carried_input<P>(
    seg: &RoleSeg,
    node: &Node<P>,
    at: RecipeNodeId,
    placed: &mut Vec<Placed>,
) -> Result<RecipeNodeId, Unwalked> {
    let misplaced = Unwalked::Misplaced(at);
    match (seg, node) {
        (RoleSeg::FromA(_), Node::Boolean { a, .. }) => Ok(*a),
        (RoleSeg::FromB(_), Node::Boolean { b, .. }) => Ok(*b),
        (RoleSeg::FromA(_) | RoleSeg::FromB(_), _) => Err(misplaced),
        (RoleSeg::FromMember { member, .. }, Node::Union { .. }) => Ok(*member),
        (RoleSeg::FromMember { .. }, _) => Err(misplaced),
        (
            RoleSeg::Instance { i, .. },
            Node::Pattern { input, .. } | Node::PlacedUnion { input, .. },
        ) => {
            placed.push(Placed::Instance(at, *i));
            Ok(*input)
        }
        (RoleSeg::Instance { .. }, _) => Err(misplaced),
        // Every other carried segment is a one-body operation's, carried
        // from its one target.
        (
            _,
            Node::Fillet { target, .. }
            | Node::Chamfer { target, .. }
            | Node::Shell { target, .. }
            | Node::Split { target, .. },
        ) => Ok(*target),
        _ => Err(misplaced),
    }
}

/// **The construction of the datum `read` reaches**: the walk through
/// the transforms that place it, to the node that defines the plane.
fn datum<P>(
    doc: &crate::doc::Doc<P>,
    read: RecipeNodeId,
    mut placed: Vec<Placed>,
) -> Result<Construction, Unwalked> {
    let mut at = read;
    loop {
        match doc.node(at).ok_or(Unwalked::Absent(at))? {
            Node::Transform { input, .. } => {
                placed.push(Placed::Transform(at));
                at = *input;
            }
            _ => {
                return Ok(Construction {
                    origin: Origin::Datum(at),
                    placed,
                });
            }
        }
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
                discharge: row.discharge,
            })
        })
        .collect()
}

/// **A profile's decided junctions, as rows** (D1's profile tangency):
/// each tangent joint validation decided Zero that no constructor made,
/// its two cells the pieces the arriving and leaving segments are. The
/// rows come in canonical order: loops outer first, joints ascending.
///
/// # Errors
///
/// [`NamingError::Emission`] for a canonical segment `pieces` does not
/// name: the pieces and the validated profile were built from one
/// program by one pass, so this is a kernel bug.
pub(crate) fn name_junctions<T: geom_core::Real>(
    profile: RecipeNodeId,
    validated: &profile::ValidatedProfile<T>,
    pieces: &crate::eval::ProfilePieces,
) -> Result<Vec<NamedCoincidence>, NamingError> {
    const UNNAMED: &str = "a profile junction row names a segment the profile's pieces do not";
    let mut rows = Vec::new();
    for (loop_index, lp) in validated.loops().iter().enumerate() {
        let n = lp.segments().len();
        let piece = |segment: usize| -> Result<NamedCell, NamingError> {
            pieces
                .edges
                .get(loop_index)
                .and_then(|edges| edges.get(segment))
                .map(|&piece| NamedCell::Piece { profile, piece })
                .ok_or(NamingError::Emission { what: UNNAMED })
        };
        for decided in lp.decided_joints() {
            let joint = decided.joint;
            let relation = match decided.carriers {
                profile::JointCarriers::Same => topo::Relation::SameOriented,
                profile::JointCarriers::Tangent => topo::Relation::Tangent {
                    aligned: !decided.reverses,
                },
            };
            rows.push(NamedCoincidence {
                cells: [piece((joint + n - 1) % n)?, piece(joint)?],
                relation,
                site: topo::DecisionSite::ProfileJunction,
                margin: decided.margin,
                discharge: topo::Discharge::Numeric,
            });
        }
    }
    Ok(rows)
}
