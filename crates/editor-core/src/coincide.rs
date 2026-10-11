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
//! **Three rungs.** The first, [`Rung::SameConstruction`], is the same
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
//! A row the first rung leaves is read off a replay of the document at
//! `Sym` with every variable a symbol ([`replay`]): the kernel's own
//! carrier-pair verdict on the two cells' symbolic carriers
//! ([`Rung::CarrierVerdict`]), then the deciding margin's identity
//! ([`Rung::MarginIdentity`]). A row no rung proves is
//! [`Proof::Unproven`]: it holds at the current values, and the door
//! has not shown that it holds across the family.

use core::fmt;

use geom_core::MarginDiag;

use crate::names::{
    EntityKey, EntityRef, NameTable, NamingError, ProfileEdgeRef, RoleSeg, StableName,
};
use crate::node::{Node, RecipeNodeId};

mod replay;

/// **What a contact record's positional input is**: the input a
/// [`topo::Backing::Carried`] names by its position, as the value that
/// holds the record publishes it ([`crate::eval::NodeValue::cited_inputs`]).
/// A carried citation names that input's record, which cites its own
/// backing, so a chain of them ends at a row a node decided.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CitedInput {
    /// An operand the node reads, by its read.
    Read(crate::VarId),
    /// The part an instance places: its product's records.
    Part(crate::ident::DocRef),
}

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
    /// An entity of an input body of the deciding node, by the read it
    /// entered through and its name in that read's table.
    Entity {
        /// The read the deciding node took the entity in through.
        input: crate::VarId,
        /// Its name there.
        name: StableName,
    },
    /// The plane the deciding node cuts with: its tool read, a datum
    /// rather than a cell of any body.
    Tool {
        /// The tool read.
        input: crate::VarId,
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
    /// The kernel's carrier-pair verdict, run on the two cells'
    /// carriers in the replay at `Sym`, decided them one carrier, every
    /// Zero it stands on a theorem of this kind.
    CarrierVerdict(geom_core::Theorem),
    /// The deciding margin, read in the replay at `Sym`, is zero as a
    /// polynomial in the variables: a theorem of this kind.
    MarginIdentity(geom_core::Theorem),
}

/// **The door was canceled** before it decided every row: the token
/// [`prove`] was handed was canceled, in the replay or between the
/// rows it discharges. It decided nothing it can report.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Canceled;

impl fmt::Display for Canceled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("the coincidence door was canceled")
    }
}

impl core::error::Error for Canceled {}

/// **Why the replay at `Sym` proved nothing about a row.**
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Unreplayed {
    /// The replay produced no value at the deciding node: it refused or
    /// escalated there, or somewhere upstream.
    Refused,
    /// The replay produced a value, and took other decisions there: no
    /// row of its own at that site and ordinal names these cells.
    OtherDecisions,
    /// The replay took the decision, and neither the carrier-pair
    /// verdict nor the margin is identically zero within the door's
    /// budget: it is a coincidence of values, as far as the door can
    /// tell.
    Numeric,
    /// The row stands on a part instance, and the replay resolves no
    /// part: a symbolic run crosses no part seam
    /// ([`crate::analysis::ParamBoxError::SymbolsAcrossAPartSeam`]).
    ThroughAPart,
    /// The row is not replayed: the at-rest census's rows ride the
    /// gathered product, which the door does not replay, and a
    /// document of a payload the evaluator does not read is not
    /// replayed either.
    NotReplayed,
}

impl fmt::Display for Unreplayed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Refused => "the replay with every variable a symbol produced no value there",
            Self::OtherDecisions => {
                "the replay with every variable a symbol took other decisions there"
            }
            Self::Numeric => {
                "with every variable a symbol, neither the carriers' verdict nor the deciding \
                 margin is shown identically zero within the door's budget"
            }
            Self::ThroughAPart => {
                "the row stands on a part, and the replay with every variable a symbol resolves \
                 no part"
            }
            Self::NotReplayed => "the row is not replayed with every variable a symbol",
        })
    }
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
    /// (an `Instance` at a node that is not a pattern, say).
    Misplaced(RecipeNodeId),
    /// The document holds no node by this id.
    Absent(RecipeNodeId),
    /// This node's operand reads a variable no live operation defines
    /// (D10: a delete leaves its readers unresolved).
    Unresolved(RecipeNodeId),
    /// The read the cell entered through is no live operation's output.
    Unread(crate::VarId),
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
            Self::Unresolved(node) => write!(
                f,
                "its read passes {node}, whose operand reads what no live operation defines"
            ),
            Self::Unread(read) => write!(
                f,
                "the read it entered through, {read}, is no live operation's output"
            ),
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
    /// What the replay at `Sym` made of the row.
    pub replay: Unreplayed,
}

impl fmt::Display for Residual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.constructions_words(f)?;
        write!(f, "; {}", self.replay)
    }
}

impl Residual {
    /// The constructions, in words.
    fn constructions_words(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    /// or assert the coincidence at its site: the residual the replay
    /// read is a function of the variables alone that the door did not
    /// show zero.
    OneConstruction,
    /// Assert the coincidence at its site: the door has no form of the
    /// row's residual to read ([`Unreplayed`] other than
    /// [`Unreplayed::Numeric`]), or the one it read keeps a value
    /// computed apart from the variables
    /// ([`geom_core::sym::deferred::Undischarged::KeepsAnOpaqueAtom`]).
    Assert,
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
            Self::Assert => {
                "Recourse: assert the coincidence at its site; the door cannot read what decided it \
                 as a function of the variables"
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
        topo::Relation::TangentContact { seam: false } => "touches tangentially against",
        topo::Relation::TangentContact { seam: true } => "continues tangentially into",
        topo::Relation::Coaxial => "shares an axis with",
        topo::Relation::CoRuled => "is ruled along one direction with",
    }
}

/// A decision site in words.
pub(crate) const fn site_words(site: topo::DecisionSite) -> &'static str {
    match site {
        topo::DecisionSite::PlaneLadder => "a pair of planes its margins read as one",
        topo::DecisionSite::CarrierLadder => "a pair of carriers its margins read as one",
        topo::DecisionSite::TangentWitness => "a tangency verified along its locus",
        topo::DecisionSite::CoaxialSphere => "a sphere's centre read on a cylinder's axis",
        topo::DecisionSite::SplitOn => "a split's on-plane verdict where its pieces touch",
        topo::DecisionSite::BatteryTurn => "a blend's isosceles turn",
        topo::DecisionSite::BatteryJoint => "a blend chain's joint read as tangent",
        topo::DecisionSite::BatterySupportAxis => {
            "a blend's two supports read as sharing the axis its band is minted on"
        }
        topo::DecisionSite::VertexFusion => "a vertex its margin read on another operand's cell",
        topo::DecisionSite::CensusAtRest => "two placed faces the at-rest census read as one",
        topo::DecisionSite::ImportAnchor => "two imported vertices an anchor read as one",
        topo::DecisionSite::ProfileJunction => "a profile junction no constructor made",
    }
}

/// **The door**: whether each row a node of `doc` decided holds
/// structurally. `nodes` holds each deciding node with its rows in
/// decision order, as its value carries them (a mate stands for the
/// at-rest census's rows on its pair); the answer is one [`Proof`] per
/// row, in the same shape. The rungs are tried in order (module docs)
/// at the door's one strength ([`replay`]'s budget): the document is
/// replayed once, at `tol`, only where a row the first rung leaves asks
/// for it.
///
/// # Errors
///
/// [`Canceled`] once `cancel` is canceled, which the replay checks
/// between nodes and the door between the margins it discharges.
pub fn prove<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    nodes: &[(RecipeNodeId, &[NamedCoincidence])],
    tol: geom_core::Tol,
    cancel: &crate::CancelToken,
) -> Result<Vec<Vec<Proof>>, Canceled> {
    let walked: Vec<Vec<[Result<Construction, Unwalked>; 2]>> = nodes
        .iter()
        .map(|(_, rows)| rows.iter().map(|row| constructions(doc, row)).collect())
        .collect();
    let asks = walked.iter().flatten().any(|c| !one_construction(c));
    let replayed = match P::evaluable(doc) {
        Some(doc) if asks => replay::rungs(doc, nodes, tol, cancel)?,
        _ => nodes
            .iter()
            .map(|(_, rows)| {
                rows.iter()
                    .map(|_| Err((Unreplayed::NotReplayed, Recourse::Assert)))
                    .collect()
            })
            .collect(),
    };
    Ok(walked
        .into_iter()
        .zip(replayed)
        .map(|(walked, replayed)| {
            walked
                .into_iter()
                .zip(replayed)
                .map(|(constructions, replayed)| {
                    if one_construction(&constructions) {
                        return Proof::Structural(Rung::SameConstruction);
                    }
                    match replayed {
                        Ok(rung) => Proof::Structural(rung),
                        Err((replay, recourse)) => Proof::Unproven {
                            residual: Residual {
                                constructions,
                                replay,
                            },
                            recourse,
                        },
                    }
                })
                .collect()
        })
        .collect())
}

/// Each cell's construction.
fn constructions<P>(
    doc: &crate::doc::Doc<P>,
    row: &NamedCoincidence,
) -> [Result<Construction, Unwalked>; 2] {
    row.cells.each_ref().map(|cell| match cell {
        NamedCell::Entity { input, name } => construction(doc, *input, name),
        NamedCell::Tool { input } => doc
            .operation_of(*input)
            .ok_or(Unwalked::Unread(*input))
            .and_then(|at| datum(doc, at, Vec::new())),
        NamedCell::Piece { profile, piece } => Ok(Construction {
            origin: Origin::Piece(*profile, *piece),
            placed: Vec::new(),
        }),
    })
}

/// The first rung: the two cells one construction read twice.
fn one_construction(constructions: &[Result<Construction, Unwalked>; 2]) -> bool {
    matches!(constructions, [Ok(a), Ok(b)] if a == b)
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
/// read that segment says — a `From`'s own read, an `Instance`'s
/// pattern input, a one-body operation's target — where the walk
/// continues.
///
/// # Errors
///
/// [`Unwalked`], naming the node the walk stopped at.
pub fn construction<P>(
    doc: &crate::doc::Doc<P>,
    read: crate::VarId,
    name: &StableName,
) -> Result<Construction, Unwalked> {
    use crate::names::attribute::{SegOrigin, origin};
    let node = |at: RecipeNodeId| doc.node(at).ok_or(Unwalked::Absent(at));
    let mut placed = Vec::new();
    let (mut read, mut name) = (read, name.clone());
    let mut at = doc.operation_of(read).ok_or(Unwalked::Unread(read))?;
    loop {
        while at != name.node {
            read = match node(at)? {
                Node::Transform { input, .. } => {
                    placed.push(Placed::Transform(at));
                    *input
                }
                Node::Part { of, .. } => *of,
                Node::Split { target, .. } => target.read,
                _ => return Err(Unwalked::Through(at)),
            };
            at = operation(doc, at, read)?;
        }
        let seg = name.path.first().ok_or(Unwalked::Unclassified(at))?;
        let of = match origin(seg) {
            SegOrigin::Minted => {
                return match (seg, node(at)?) {
                    (RoleSeg::SectionFace { .. }, Node::Split { tool, .. }) => {
                        datum(doc, operation(doc, at, *tool)?, placed)
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
        read = carried_read(seg, node(at)?, at, &mut placed)?;
        at = operation(doc, at, read)?;
        name = of;
    }
}

/// **The operation defining the read `read`** that `at` takes (D10).
///
/// # Errors
///
/// [`Unwalked::Unresolved`] where no live operation defines it.
fn operation<P>(
    doc: &crate::doc::Doc<P>,
    at: RecipeNodeId,
    read: crate::VarId,
) -> Result<RecipeNodeId, Unwalked> {
    doc.read_operation(read).ok_or(Unwalked::Unresolved(at))
}

/// **The read a carried segment at `at` names its entity in**, by the
/// segment and the node together: a `From`'s own read; a placing
/// segment (`Instance`) adds its placement.
///
/// # Errors
///
/// [`Unwalked::Misplaced`] for a segment `at`'s kind does not carry.
fn carried_read<P>(
    seg: &RoleSeg,
    node: &Node<P>,
    at: RecipeNodeId,
    placed: &mut Vec<Placed>,
) -> Result<crate::VarId, Unwalked> {
    let misplaced = Unwalked::Misplaced(at);
    match (seg, node) {
        (RoleSeg::From { read, .. }, _) => Ok(*read),
        (
            RoleSeg::Instance { i, .. },
            Node::Pattern { input, .. } | Node::PlacedUnion { input, .. },
        ) => {
            placed.push(Placed::Instance(at, *i));
            Ok(*input)
        }
        (RoleSeg::Instance { .. }, _) => Err(misplaced),
        // Every other carried segment is a split's, carried from its
        // target.
        (_, Node::Split { target, .. }) => Ok(target.read),
        _ => Err(misplaced),
    }
}

/// **The construction of the datum `at` defines**: the walk through
/// the transforms that place it, to the node that defines the plane.
fn datum<P>(
    doc: &crate::doc::Doc<P>,
    mut at: RecipeNodeId,
    mut placed: Vec<Placed>,
) -> Result<Construction, Unwalked> {
    loop {
        match doc.node(at).ok_or(Unwalked::Absent(at))? {
            Node::Transform { input, .. } => {
                placed.push(Placed::Transform(at));
                at = operation(doc, at, *input)?;
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

/// The deciding node's inputs a row's cells are keyed in: the read
/// and table each [`topo::Operand`] names, and the node's tool, if it
/// cuts with one.
pub(crate) struct RowInputs<'a> {
    /// Operand A's read and table (a one-body operation's input).
    pub a: (crate::VarId, &'a NameTable),
    /// Operand B's, for an operation that reads two bodies.
    pub b: Option<(crate::VarId, &'a NameTable)>,
    /// The tool read, for an operation that cuts with one.
    pub tool: Option<crate::VarId>,
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
            topo::RowCell::Result { .. } => Err(unnamed()),
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
                discharge: topo::Discharge::of(decided.margin),
            });
        }
    }
    Ok(rows)
}
