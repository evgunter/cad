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
//! carry-through segments and the maps its read passes (a placement's
//! copy, a pattern's instance, a mirror's image), to the node that
//! minted it ([`construction`]); two cells are one construction when
//! they reach one origin through one chain of maps. A reflection fixes
//! its plane, so a cell on the plane's face read through the mirror is
//! that cell read again. A plane read off a face is that face's
//! construction ([`plane`]), so a split's section face and the face its
//! tool reads are one origin. A split's halves each hold a copy of one
//! crossing, its sense against that half, and the walk reads both
//! against the `Above` half ([`crossing`]), so the two copies are one
//! construction. A row laying an edge or a vertex on a face is that
//! rung's where the one lies on the face as its minter built it
//! ([`lies_on`]): a rim on its cap, a split's crossing vertex or
//! section edge on its tool's plane. Provenance is the document's: the walk
//! reads the recipe and the names, never a stamp the kernel carries.
//!
//! **A mate is that rung's statement** (`[coincidence.structural]`):
//! the two poses a mate of a pinning bundle equates, each read in a
//! copy that holds it where that mate's side reads it
//! ([`crate::mate::reads_in`]), are one construction read twice, with
//! the sense the mate's flips state, and being one construction is the
//! equivalence the mates generate, closed over every pose kind
//! ([`stated::Stated`]). A band a revolve mints is about the revolve's
//! own `axis` output ([`stated::axis_of`]), so coaxiality is "one
//! `Axis` read twice, directly or as projections of one construction":
//! two faces whose planes are one class, or two cells whose axes are,
//! are that rung's ([`stated_by_the_mates`]). A placed copy's frame is
//! the construction its bundle states (`[placement.placement]`), so no
//! replay reads the fold: the door reads what the mates state, and
//! replays where they state nothing; the replay reads two cylinders'
//! axes as the mates state them, and their radii (each wall's rims its
//! own radius, and the two radii one) and senses off its carriers.
//! Only a bundle the doors admit states anything
//! ([`crate::mate::solve::admits`]). A standoff equates no face's plane with another's, and a
//! copy another bundle moves holds the face where the mate does not
//! read it, so neither is stated here. An unproven row between two
//! cells with an axis names the first of one axis, one radius and one
//! sense the door did not establish ([`Axes`]).
//!
//! **A traced profile is that rung's statement too** ([`traced`]): a
//! tangency row between a face and an extrude's wall over a piece that
//! leaves the face's trace along its tangent holds where the extrude's
//! heading, its frame's `z`, is one `Axis` with the face's, and the face
//! is itself an extrude's wall, which its own construction slides along
//! that axis. A wall that would keep the tangency but for its heading
//! names the axis to read off the face ([`Recourse::ReadTheAxis`]).
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
//!
//! **A row at rest the first rung leaves** is read off the replay's
//! copies: the replay folds each bundle at `Sym` like any other
//! operation, resolving the parts the document instances, and runs the
//! carrier verdict on the two copies' faces. The census itself is not
//! re-run, so a row at rest between anything but two faces has no form
//! the door reads, and one whose replay decides the pair otherwise is
//! [`Unreplayed::OtherDecisions`]. Parts are resolved only where such a
//! row is left; a node's row on a part is not replayed
//! ([`Unreplayed::ThroughAPart`]).

use core::fmt;

use geom_core::MarginDiag;

use crate::names::{
    EntityKey, EntityRef, NameTable, NamingError, ProfileEdgeRef, RoleSeg, StableName,
};
use crate::node::{Node, RecipeNodeId};

mod replay;
mod stated;

pub use replay::with_replay_session;

/// **What a contact record's positional input is**: the input a
/// [`topo::Backing::Carried`] names by its position, as the value that
/// holds the record publishes it ([`crate::Carry::cited_inputs`]).
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
    /// The cells decided: for a [`topo::Relation::Meet`], the two or
    /// more cells that meet, sorted, each once; for every other
    /// relation, two, in the order the decision read them.
    pub cells: Vec<NamedCell>,
    /// What was decided between them.
    pub relation: topo::Relation,
    /// Where it was decided.
    pub site: topo::DecisionSite,
    /// The margin that decision read: Zero, or in band where a
    /// declaration bridged it ([`topo::Coincidence::margin`]).
    pub margin: MarginDiag,
    /// How that margin's Zero was discharged.
    pub discharge: topo::Discharge,
    /// Which of the node's decisions it is.
    pub origin: RowOrigin,
}

/// **Which decision of a node's evaluation a row is** (DM4: a union's
/// rows are its evaluation's decisions).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RowOrigin {
    /// The node's own operation decided it.
    Node,
    /// A union's or an intersect's judgement of the members at these
    /// places in its list, the lesser first: one of the node's rows
    /// that are order-free modulo the members' places.
    Judged {
        /// The two members' places.
        pair: [u32; 2],
    },
    /// The fold step of a union or an intersect of three members or
    /// more that folds in the member at this place in its list.
    Fold {
        /// The member's place.
        step: u32,
    },
}

impl NamedCoincidence {
    /// **A meeting of `cells` at one point** ([`topo::Relation::Meet`]):
    /// the cells sorted, each once.
    ///
    /// # Errors
    ///
    /// [`NamingError::Emission`] where fewer than two cells remain: a
    /// point decided on the cell it is, which states nothing.
    pub(crate) fn meet(
        mut cells: Vec<NamedCell>,
        site: topo::DecisionSite,
        (margin, discharge): (MarginDiag, topo::Discharge),
        origin: RowOrigin,
    ) -> Result<Self, NamingError> {
        cells.sort();
        cells.dedup();
        if cells.len() < 2 {
            return Err(NamingError::Emission { what: MEET_OF_ONE });
        }
        Ok(Self {
            cells,
            relation: topo::Relation::Meet,
            site,
            margin,
            discharge,
            origin,
        })
    }

    /// **Whether `self` and `other` are decided at one point**: both
    /// name one member vertex. A row naming a vertex decides at that
    /// vertex, whatever its relation; a row naming none fixes no point
    /// by its cells, so it is at one point with no other row.
    #[must_use]
    pub(crate) fn at_one_vertex(&self, other: &Self) -> bool {
        self.cells.iter().any(|c| {
            matches!(c, NamedCell::Entity { name, .. } if name.kind == crate::names::EntityKind::Vertex)
                && other.cells.contains(c)
        })
    }
}

/// A coincidence row's cell is of an input the node read, and that
/// input's table names no entity at its key ([`name_rows`]).
pub(crate) const ROW_CELL_NOT_IN_TABLE: &str =
    "a coincidence row names a cell its input's table does not name";

/// A meeting that resolves to fewer than two cells.
const MEET_OF_ONE: &str = "a coincidence meeting resolves to fewer than two cells";

/// One cell of a [`NamedCoincidence`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NamedCell {
    /// An entity of an input body of the deciding node, by the read it
    /// entered through and its name in that read's table.
    Entity {
        /// The read the deciding node took the entity in through.
        input: crate::VarId,
        /// Its name there.
        name: StableName,
    },
    /// The plane the deciding node cuts with: its tool read, a pose
    /// rather than a cell of any body.
    Tool {
        /// The tool read.
        input: crate::VarId,
        /// The role the tool is read through, `None` for a plane read
        /// plainly.
        role: Option<crate::pose::PoseRole>,
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
        /// What separates the cells' constructions.
        residual: Residual,
        /// What would make the coincidence structural.
        recourse: Recourse,
    },
}

/// The rung of the door that proved a row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rung {
    /// The cells are one construction read as many times: one origin,
    /// reached through one chain of maps, or two poses the mates make
    /// one ([`stated_by_the_mates`]).
    SameConstruction,
    /// The kernel's carrier-pair verdict, run on the two cells'
    /// carriers in the replay at `Sym`, decided them one carrier, every
    /// Zero it stands on proven by the symbolic tier.
    CarrierVerdict,
    /// The deciding margin, read in the replay at `Sym`, is proven zero
    /// by the symbolic tier.
    MarginIdentity,
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
    /// row of its own at that site and ordinal names these cells, or,
    /// for an at-rest row, its carrier verdict on the two copies' faces
    /// decides them other than one carrier of the row's relation.
    OtherDecisions,
    /// The replay took the decision, and neither the carrier-pair
    /// verdict nor the margin is identically zero within the door's
    /// budget: it is a coincidence of values, as far as the door can
    /// tell.
    Numeric,
    /// The row stands on a part instance the replay resolves no part
    /// for: an at-rest row the door was handed no resolver for, or a
    /// node's row, which the replay does not read through a part.
    ThroughAPart,
    /// The row is not replayed: an at-rest census row between cells
    /// other than two faces one carrier (the census itself is not
    /// re-run), and every row of a document of a payload the evaluator
    /// does not read.
    NotReplayed,
    /// The replay could not read one cell's body into the frame the
    /// carrier verdict is decided in (D10 `[placement.frame]`): the
    /// relative map overflowed, or the kernel refused the mapped body.
    Frame,
    /// The replay ran out of member budget
    /// ([`crate::EvalOutcome::Exhausted`]) at or before the node the
    /// row was decided at.
    Exhausted,
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
            Self::Frame => {
                "the replay with every variable a symbol could not read one cell's body into \
                 the frame its carriers are decided in"
            }
            Self::Exhausted => {
                "the replay with every variable a symbol ran out of member budget before the \
                 row's node"
            }
        })
    }
}

/// **A cell's construction**, as the door reads it off the document:
/// where its carrier comes from ([`Origin`]) and the maps met on the
/// way from the read to it, outermost first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Construction {
    /// Where the cell's carrier comes from.
    pub origin: Origin,
    /// The maps between the origin and the read.
    pub maps: Vec<Map>,
}

/// **Where a cell's carrier comes from.**
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Origin {
    /// An entity, as the node that minted it named it: its role there,
    /// piece qualifiers dropped, since every piece of an entity lies on
    /// its one carrier.
    Minted(StableName),
    /// A plane read off no face ([`plane`]): the pose, through the
    /// role it is read through (`None` for the pose itself). A split's
    /// tool, every section face the split mints on it, and a mirror's
    /// plane.
    Pose(crate::VarId, Option<crate::pose::PoseRole>),
    /// A profile's piece: the step of that profile that drew it, and
    /// its role there.
    Piece(RecipeNodeId, ProfileEdgeRef),
}

/// **Why the walk reached no construction.** Each arm names the node
/// it stopped at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unwalked {
    /// The read passes this node, which neither carries the name
    /// verbatim (N1) nor minted it.
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
                "its read passes {node}, which neither carries its name through nor minted it"
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

/// One map a cell's read passes on the way to its minting node.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Map {
    /// A family's member, the operation at those index values.
    Member(RecipeNodeId, crate::names::MemberAt),
    /// A placement's copy (a [`crate::Node::Place`]'s).
    World(RecipeNodeId),
    /// A mirror's image (a [`crate::Node::Mirror`]'s), of a cell off
    /// its plane.
    Mirror(RecipeNodeId),
}

/// **What separates an unproven row's constructions**: each cell's
/// construction, or why the walk reached none.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Residual {
    /// Each cell's construction, in the row's cell order.
    pub constructions: Vec<Result<Construction, Unwalked>>,
    /// What the replay at `Sym` made of the row.
    pub replay: Unreplayed,
    /// For a row between two cells with an axis, what of one axis, one
    /// radius and one sense the door established.
    pub axes: Option<Axes>,
}

/// **An unproven row's axes** (`[coincidence.structural]`: "coaxiality
/// is one `Axis` read twice"): a row between two cells with an axis is
/// one axis, one radius and one sense, and the door names the first it
/// did not establish.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Axes {
    /// Each cell's axis as the pose its mates reach, in the row's cell
    /// order: a pose variable, read where it stands where one is in its
    /// class; `None` where the axis is the cell's own.
    pub reached: [Option<crate::VarId>; 2],
    /// The copy each cell is read in (its placement's output), `None`
    /// for a cell read where it was built.
    pub copies: [Option<crate::VarId>; 2],
    /// The first component not established.
    pub left: Component,
}

/// **A component of a coaxial reading** ([`Axes`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Component {
    /// The two axes are not shown one.
    Axis,
    /// The axes are one; the radii are equal only at the current values,
    /// or a wall's two ends are.
    Radius,
    /// The axes are one; the replay did not read the radii (it did not
    /// run on the row, or took other decisions there).
    RadiusUnread,
    /// The axes and the radii are one; the senses are not those the row
    /// decided.
    Sense,
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
        let n = self.constructions.len();
        let unwalked: Vec<(usize, &Unwalked)> = self
            .constructions
            .iter()
            .enumerate()
            .filter_map(|(i, c)| c.as_ref().err().map(|why| (i, why)))
            .collect();
        match (self.constructions.as_slice(), unwalked.as_slice()) {
            ([Ok(a), Ok(b)], _) => match (a.origin == b.origin, a.maps == b.maps) {
                (false, false) => {
                    f.write_str("the two cells are two constructions, placed or reflected apart")
                }
                (false, true) => f.write_str("the two cells are two constructions"),
                (true, _) => {
                    f.write_str("the two cells are one construction placed or reflected apart")
                }
            },
            ([Err(a), Err(b)], _) => write!(
                f,
                "neither cell is read to a construction: the first because {a}, the second \
                 because {b}"
            ),
            ([Err(why), Ok(_)], _) => {
                write!(f, "the first cell is not read to a construction: {why}")
            }
            ([Ok(_), Err(why)], _) => {
                write!(f, "the second cell is not read to a construction: {why}")
            }
            (all, []) => {
                let walked: Vec<&Construction> =
                    all.iter().filter_map(|c| c.as_ref().ok()).collect();
                let mut origins: Vec<&Origin> = Vec::new();
                for c in &walked {
                    if !origins.contains(&&c.origin) {
                        origins.push(&c.origin);
                    }
                }
                let distinct = origins.len();
                let apart = walked.windows(2).any(|w| w[0].maps != w[1].maps);
                match (distinct, apart) {
                    (1, _) => write!(
                        f,
                        "the {n} cells are one construction placed or reflected apart"
                    ),
                    (d, true) => write!(
                        f,
                        "the {n} cells are {d} constructions, placed or reflected apart"
                    ),
                    (d, false) => write!(f, "the {n} cells are {d} constructions"),
                }
            }
            (_, unwalked) => {
                for (k, (i, why)) in unwalked.iter().enumerate() {
                    if k > 0 {
                        f.write_str("; ")?;
                    }
                    write!(
                        f,
                        "cell {} of {n} is not read to a construction: {why}",
                        i + 1
                    )?;
                }
                Ok(())
            }
        }
    }
}

/// **What would make an unproven row structural.**
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Recourse {
    /// Build the cells as one construction of the same variables, or
    /// assert the coincidence where an assertion says it: the residual
    /// the replay read whole is a function of the variables alone that
    /// the door did not show zero.
    OneConstruction,
    /// Assert the coincidence where an assertion says it, else build it
    /// as a construction the door reads: the door has no form of the
    /// row's residual to read ([`Unreplayed`] other than
    /// [`Unreplayed::Numeric`]), or the one it read keeps a value
    /// computed apart from the variables
    /// ([`geom_core::sym::deferred::Undischarged::KeepsAnOpaqueAtom`]).
    Assert,
    /// Assert the coincidence where an assertion says it, else build it
    /// as one construction: the form the replay read outgrew the door's
    /// budget
    /// ([`geom_core::sym::deferred::Undischarged::OverTheBudget`]), so
    /// whether it is zero is not decided there.
    Budget,
    /// Read the sweep's heading off the face the profile traces: the
    /// wall leaves that face's trace along its tangent and a slide along
    /// that face's axis would keep the tangency, but no statement the
    /// door reads makes the extrude's heading that axis (a frame read off
    /// a rim may be coaxial with its wall for every value, and the door
    /// does not read it so) ([`traced`]).
    ReadTheAxis,
    /// Raise the evaluation's member budget, or lower a count: the
    /// replay ran out of it before the row ([`Unreplayed::Exhausted`]).
    MemberBudget,
}

impl Recourse {
    /// **The recourse as a sentence's ending**, `sayable` whether an
    /// assertion could quiet the row (`checks::at_rest::row_quietable`):
    /// the assertion is offered only where one can say the row, and a
    /// row no assertion says (a blend turn's two feet, two radii written
    /// apart, a cell no measure reads) names the construction alone
    /// (`[intent.checked]`).
    #[must_use]
    pub const fn words(self, sayable: bool) -> &'static str {
        // Each reason once, behind the assertion where one can say the
        // row and behind the construction where none can.
        macro_rules! either {
            ($why:literal) => {
                if sayable {
                    concat!("Recourse: assert the coincidence at its site; ", $why)
                } else {
                    concat!(
                        "Recourse: build the cells as one construction the door reads; ",
                        $why
                    )
                }
            };
        }
        macro_rules! one_construction {
            ($($or:literal)?) => {
                concat!(
                    "Recourse: build the cells as one construction of the same variables, so \
                     the coincidence holds whatever their values"
                    $(, $or)?
                )
            };
        }
        match self {
            Self::OneConstruction if sayable => one_construction!(", or assert it at its site"),
            Self::OneConstruction => one_construction!(),
            Self::Assert => {
                either!("the door cannot read what decided it as a function of the variables")
            }
            Self::Budget => either!(
                "what decided it, read as a function of the variables, outgrew the door's budget"
            ),
            Self::ReadTheAxis => {
                "Recourse: read the sweep's axis off the face the profile traces, so the sweep \
                 keeps the tangency the profile draws whatever the values"
            }
            Self::MemberBudget => {
                "Recourse: raise the evaluation's member budget, or lower a count, so the replay \
                 reaches the row"
            }
        }
    }
}

/// A relation as the clause between the cells of a two-cell row.
pub(crate) const fn relation_words(relation: topo::Relation) -> &'static str {
    match relation {
        topo::Relation::SameOriented => "continues on one carrier with",
        topo::Relation::SameOpposite => "rests on one carrier against",
        topo::Relation::OnCarrier => "lies on",
        topo::Relation::CommonFoot => "ends on its turn's third edge at one point with",
        topo::Relation::Tangent { aligned: true } => "continues tangent into",
        topo::Relation::Tangent { aligned: false } => "turns back tangent into",
        topo::Relation::SurfaceTangent => "is tangent along a locus to",
        topo::Relation::Coaxial => "shares an axis with",
        topo::Relation::SameRadius => "shares a radius with",
        topo::Relation::AxesMeet => "has an axis meeting the axis of",
        topo::Relation::CoRuled => "is ruled along one direction with",
        topo::Relation::Meet => "shares one point with",
    }
}

/// **A row's statement in words**, each cell as `cell` spells it: two
/// cells and the clause between them, or every cell of a meeting of
/// three or more.
///
/// # Panics
///
/// On a row of fewer than two cells, which no constructor makes
/// ([`NamedCoincidence::meet`]).
pub(crate) fn statement_words(
    row: &NamedCoincidence,
    cell: impl Fn(&NamedCell) -> String,
) -> String {
    match row.cells.as_slice() {
        [a, b] => format!("{} {} {}", cell(a), relation_words(row.relation), cell(b)),
        [init @ .., last] if !init.is_empty() => format!(
            "{} and {} share one point",
            init.iter().map(&cell).collect::<Vec<_>>().join(", "),
            cell(last)
        ),
        _ => unreachable!("a coincidence row holds two cells or more: {row:?}"),
    }
}

/// A decision site in words.
pub(crate) const fn site_words(site: topo::DecisionSite) -> &'static str {
    match site {
        topo::DecisionSite::PlaneLadder => "a pair of planes its margins read as one",
        topo::DecisionSite::CarrierLadder => "a pair of carriers its margins read as one",
        topo::DecisionSite::TangentWitness => "a tangency verified along its locus",
        topo::DecisionSite::CoaxialSphere => "a sphere's centre read on a cylinder's axis",
        topo::DecisionSite::SphereOnTorusAxis => "a sphere's centre read on a torus's axis",
        topo::DecisionSite::CoaxialCylinderTorus => "a cylinder read coaxial with a torus",
        topo::DecisionSite::CoaxialTori => "two tori read on one axis",
        topo::DecisionSite::CrossingCylinders => {
            "two cylinders read as one radius with meeting axes"
        }
        topo::DecisionSite::SplitOn => "a split's on-plane verdict where its pieces touch",
        topo::DecisionSite::BatteryTurn => "a blend turn's two feet read as one point",
        topo::DecisionSite::BatteryJoint => "a blend chain's joint read as tangent",
        topo::DecisionSite::BatterySupportAxis => {
            "a blend's two supports read as sharing the axis its band is minted on"
        }
        topo::DecisionSite::VertexFusion => "a vertex its margin read on another operand's cell",
        topo::DecisionSite::CensusAtRest => "two cells at rest the census read as one",
        topo::DecisionSite::ProfileJunction => "a profile junction no constructor made",
    }
}

/// **The door**: whether each row a node of `doc` decided holds
/// structurally. `nodes` holds each deciding node with its rows in
/// decision order, as its value carries them (a placement stands for
/// the at-rest census's rows on its copies); the answer is one [`Proof`] per
/// row, in the same shape. The rungs are tried in order (module docs)
/// at the door's one strength ([`replay`]'s budget): the document is
/// replayed once, at `tol`, only where a row the first rung leaves asks
/// for it, its parts resolved through `resolver` (the evaluation's own,
/// [`crate::Evaluation::resolver`]).
///
/// # Errors
///
/// [`Canceled`] once `stop`'s token is canceled, which the replay checks
/// between nodes and the door between the margins it discharges.
pub fn prove<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    nodes: &[(RecipeNodeId, &[NamedCoincidence])],
    resolver: Option<&std::sync::Arc<dyn crate::part::PartResolver>>,
    tol: geom_core::Tol,
    stop: &crate::Stop,
) -> Result<Vec<Vec<Proof>>, Canceled> {
    let walked: Vec<Vec<Walked>> = nodes
        .iter()
        .map(|(_, rows)| {
            rows.iter()
                .map(|row| constructions(doc, row, tol))
                .collect()
        })
        .collect();
    let stated = stated::Stated::of(doc, tol);
    let traced: Vec<Vec<Traced>> = nodes
        .iter()
        .zip(&walked)
        .map(|((_, rows), walked)| {
            rows.iter()
                .zip(walked)
                .map(|(row, c)| traced(doc, &stated, row, c, tol))
                .collect()
        })
        .collect();
    let left: Vec<Vec<bool>> = nodes
        .iter()
        .zip(&walked)
        .zip(&traced)
        .map(|(((_, rows), walked), traced)| {
            rows.iter()
                .zip(walked)
                .zip(traced)
                .map(|((row, constructions), &traced)| {
                    !first_rung(doc, &stated, row, constructions, traced, tol)
                })
                .collect()
        })
        .collect();
    // Whether each row's two cells' axes are one by the mates: the
    // replay reads a stated axis as stated, not as the fold builds it.
    let axes_one: Vec<Vec<bool>> = walked
        .iter()
        .map(|walked| {
            walked
                .iter()
                .map(|c| matches!(c.as_slice(), [Ok(a), Ok(b)] if one_axis(doc, &stated, a, b)))
                .collect()
        })
        .collect();
    let replayed = match P::evaluable(doc) {
        Some(doc) if left.iter().flatten().any(|&l| l) => {
            replay::rungs(doc, nodes, (&left, &axes_one), resolver, tol, stop)?
        }
        _ => left
            .iter()
            .map(|left| {
                left.iter()
                    .map(|&left| {
                        left.then_some(Err(((Unreplayed::NotReplayed, Recourse::Assert), None)))
                    })
                    .collect()
            })
            .collect(),
    };
    Ok(nodes
        .iter()
        .zip(walked)
        .zip(replayed)
        .zip(axes_one)
        .zip(traced)
        .map(|(((((_, rows), walked), replayed), axes_one), traced)| {
            rows.iter()
                .zip(walked)
                .zip(replayed)
                .zip(axes_one)
                .zip(traced)
                .map(
                    |((((row, constructions), replayed), one), traced)| match replayed {
                        None => Proof::Structural(Rung::SameConstruction),
                        Some(Ok(rung)) => Proof::Structural(rung),
                        Some(Err(((replay, recourse), left))) => {
                            // An axis row the first rung left has two axes;
                            // a curved wall pair whose axes the mates make
                            // one, and whose radii the replay never read,
                            // is left at its radius unread.
                            let left = left.or_else(|| {
                                if axis_row(row) {
                                    Some(Component::Axis)
                                } else {
                                    (one && row.site == topo::DecisionSite::CarrierLadder)
                                        .then_some(Component::RadiusUnread)
                                }
                            });
                            // The edit a finding names makes two axes one
                            // construction; no construction changes a sense
                            // the row decided.
                            let recourse = match (left, recourse) {
                                _ if traced == Traced::AxisUnread => Recourse::ReadTheAxis,
                                (Some(Component::Axis), Recourse::Assert) => {
                                    Recourse::OneConstruction
                                }
                                (Some(Component::Sense), _) => Recourse::Assert,
                                (_, recourse) => recourse,
                            };
                            let axes = left.map(|left| axes(doc, &stated, &constructions, left));
                            Proof::Unproven {
                                residual: Residual {
                                    constructions,
                                    replay,
                                    axes,
                                },
                                recourse,
                            }
                        }
                    },
                )
                .collect()
        })
        .collect())
}

/// **What of one axis, one radius and one sense a row between two
/// cells with an axis is left**, `left` the first component not
/// established: each cell's axis as the pose its mates reach
/// ([`stated::Stated::reached`]), and the copy each cell is read in.
fn axes<P>(
    doc: &crate::doc::Doc<P>,
    stated: &stated::Stated,
    constructions: &[Result<Construction, Unwalked>],
    left: Component,
) -> Axes {
    let cell = |i: usize| constructions.get(i).and_then(|c| c.as_ref().ok());
    let reached = |i| cell(i).and_then(|c| stated.reached(&stated::Key::axis(doc, c)));
    // The outermost copy the cell is read through.
    let copy = |i| {
        cell(i)
            .and_then(|c| {
                c.maps.iter().find_map(|m| match m {
                    Map::World(p) => Some(*p),
                    Map::Member(..) | Map::Mirror(_) => None,
                })
            })
            .and_then(|p| doc.output(p, 0))
    };
    Axes {
        reached: [reached(0), reached(1)],
        copies: [copy(0), copy(1)],
        left,
    }
}

/// **The door's first rung**, over `row`'s cells walked to their
/// constructions: one construction read twice, a cell its face's minter
/// built on it ([`on_by_construction`]), two poses the mates make one
/// ([`stated_by_the_mates`]), or a wall a traced profile swept tangent
/// (`traced`, [`traced`]).
fn first_rung<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    stated: &stated::Stated,
    row: &NamedCoincidence,
    constructions: &[Result<Construction, Unwalked>],
    traced: Traced,
    tol: geom_core::Tol,
) -> bool {
    one_construction(constructions)
        || on_by_construction(doc, row, constructions, tol)
        || stated_by_the_mates(doc, stated, row, constructions, tol)
        || traced == Traced::Stated
}

/// **How far a row is the trace statement's**, the least to the most
/// ([`traced`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Traced {
    /// The row is no wall leaving a traced face's trace along its
    /// tangent, by a sweep along that face's generators.
    No,
    /// It is, but the extrude's heading is not one `Axis` with the
    /// face's by any statement the door reads.
    AxisUnread,
    /// It is, and the heading is that `Axis`: the row holds.
    Stated,
}

/// **The first rung's statement by a traced profile**
/// (`[coincidence.structural]`): a tangency row between a face `F` and an
/// extrude's wall over a piece that leaves a crossing of `F`'s trace
/// along its tangent ([`leaves_along_a_trace`]), where the extrude's
/// slide is a motion of `F`'s. Three things are read, none of them per
/// carrier:
///
/// 1. the step's own equality, the piece's heading the trace's tangent;
/// 2. a pose the door decides: the extrude's heading, its frame's `z`
///    ([`stated::frame_z`]), one `Axis` with `F`'s ([`stated::Stated`]);
/// 3. the symmetry `F`'s construction states: `F` is an extrude's wall,
///    which its own extrude slides along its generators, and an axis
///    read off it runs along them.
///
/// `F` is the face the profile traces, read where the wall is read,
/// or another face of its carrier ([`stated::carrier`]).
fn traced<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    stated: &stated::Stated,
    row: &NamedCoincidence,
    constructions: &[Result<Construction, Unwalked>],
    tol: geom_core::Tol,
) -> Traced {
    let ([Ok(a), Ok(b)], topo::Relation::SurfaceTangent, Some(doc)) =
        (constructions, row.relation, P::evaluable(doc))
    else {
        return Traced::No;
    };
    swept_tangent(doc, stated, a, b, tol).max(swept_tangent(doc, stated, b, a, tol))
}

/// **How far `wall` is an extrude's wall over a piece leaving `face`'s
/// trace along its tangent, slid along `face`'s generators** ([`traced`]).
fn swept_tangent(
    doc: &crate::ProfileDoc,
    stated: &stated::Stated,
    wall: &Construction,
    face: &Construction,
    tol: geom_core::Tol,
) -> Traced {
    let Origin::Minted(name) = &wall.origin else {
        return Traced::No;
    };
    let [RoleSeg::Lateral(run)] = name.path.as_slice() else {
        return Traced::No;
    };
    if stated::in_a_part(doc, wall) {
        return Traced::No;
    }
    let Some(Node::Extrude { profile, .. }) = doc.node(name.node) else {
        return Traced::No;
    };
    let Some(Node::Profile(program)) = doc.operation_of(*profile).and_then(|p| doc.node(p)) else {
        return Traced::No;
    };
    let Some(on) = &program.on else {
        return Traced::No;
    };
    run.pieces()
        .iter()
        .map(|piece| {
            let Some(traced) = leaves_along_a_trace(program, piece)
                .and_then(|at| on.faces.get(at))
                .and_then(|&read| {
                    let select = doc.selection(read)?;
                    let [name] = select.names.as_slice() else {
                        return None;
                    };
                    construction(doc, select.body, name, tol).ok()
                })
            else {
                return Traced::No;
            };
            let read_here = Construction {
                origin: traced.origin.clone(),
                maps: wall.maps.iter().chain(&traced.maps).cloned().collect(),
            };
            if stated::carrier(&read_here) != stated::carrier(face)
                || !slides_along_its_generators(&traced)
            {
                return Traced::No;
            }
            let heading = stated::frame_z(doc, stated, on.frame, tol);
            if heading.is_some_and(|z| stated.sense(&z, &stated::Key::axis(doc, &traced)).is_some())
            {
                Traced::Stated
            } else {
                Traced::AxisUnread
            }
        })
        .max()
        .unwrap_or(Traced::No)
}

/// **The face a profile piece leaves along the tangent of its trace**,
/// by its place in [`crate::DrawnOn::faces`]: the leg of the step after
/// an [`crate::ProgramStep::AlongTrace`], at the crossing the step
/// before that ran onto. The profile's lattice admits after a director
/// only a leg that departs along it (a line, or an arc tangent to it),
/// so the leg's heading at the crossing is the trace's tangent. `None`
/// for any other piece.
fn leaves_along_a_trace(
    program: &crate::ProfileProgram,
    piece: &crate::names::ProfileEdgeRef,
) -> Option<usize> {
    use crate::{LoopProgram, ProgramArcData, ProgramStep, ProgramTarget};
    let crate::names::ProfileEdgeRef::Piece {
        step,
        role: profile::PieceRole::Leg,
    } = piece
    else {
        return None;
    };
    let (loop_, at) = program
        .ids
        .iter()
        .enumerate()
        .find_map(|(l, ids)| ids.iter().position(|s| s == step).map(|i| (l, i)))?;
    let LoopProgram::Chain(steps) = program.loops.get(loop_)? else {
        return None;
    };
    let [reach, ProgramStep::AlongTrace { .. }] = steps.get(at.checked_sub(2)?..at)? else {
        return None;
    };
    let face = match reach {
        ProgramStep::LineTo(ProgramTarget::OnTrace { face, .. })
        | ProgramStep::ContinueTo(ProgramTarget::OnTrace { face, .. })
        | ProgramStep::ArcTo(ProgramArcData::Center {
            target: ProgramTarget::OnTrace { face, .. },
            ..
        }) => *face,
        _ => return None,
    };
    usize::try_from(face).ok()
}

/// **Whether `face`'s construction slides it along its generators**:
/// an extrude's wall (the one minter of a `Lateral` face), whose own
/// extrude is that slide, so an axis read off it runs along them.
fn slides_along_its_generators(face: &Construction) -> bool {
    matches!(&face.origin, Origin::Minted(name) if matches!(name.path.as_slice(), [RoleSeg::Lateral(_)]))
}

/// Each cell of a row walked to its construction, or why it was not.
type Walked = Vec<Result<Construction, Unwalked>>;

/// Each cell's construction.
fn constructions<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    row: &NamedCoincidence,
    tol: geom_core::Tol,
) -> Vec<Result<Construction, Unwalked>> {
    row.cells
        .iter()
        .map(|cell| match cell {
            NamedCell::Entity { input, name } => construction(doc, *input, name, tol),
            NamedCell::Tool { input, role } => Ok(plane(
                doc,
                &crate::pose::RoleRead {
                    read: *input,
                    role: *role,
                },
                tol,
            )),
            NamedCell::Piece { profile, piece } => Ok(Construction {
                origin: Origin::Piece(*profile, *piece),
                maps: Vec::new(),
            }),
        })
        .collect()
}

/// The first rung: every cell one construction, read as many times.
fn one_construction(constructions: &[Result<Construction, Unwalked>]) -> bool {
    match constructions {
        [Ok(first), rest @ ..] if !rest.is_empty() => {
            rest.iter().all(|c| c.as_ref().is_ok_and(|c| c == first))
        }
        _ => false,
    }
}

/// **The first rung's laying of an edge or a vertex on a face**: a row
/// that lays one cell on the other's face ([`topo::Relation::OnCarrier`])
/// where the one lies on that face as its minter built it ([`lies_on`]).
fn on_by_construction<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    row: &NamedCoincidence,
    constructions: &[Result<Construction, Unwalked>],
    tol: geom_core::Tol,
) -> bool {
    use crate::names::EntityKind;
    let face = |cell: &NamedCell| matches!(cell, NamedCell::Entity { name, .. } if name.kind == EntityKind::Face);
    let ([Ok(a), Ok(b)], [ka, kb]) = (constructions, row.cells.as_slice()) else {
        return false;
    };
    row.relation == topo::Relation::OnCarrier
        && ((face(ka) && lies_on(doc, b, a, tol)) || (face(kb) && lies_on(doc, a, b, tol)))
}

/// **The first rung's statement by the mates** (`[coincidence.structural]`):
/// the row's two cells are poses the mates' equivalence makes one
/// ([`stated::Stated`]). Two faces decided on one plane are one where
/// their planes are in one class, with the sense the class states
/// between them. A row laying an edge or a vertex on a face holds where
/// it lies, as its minter built it ([`lies_on`]), on a face of that
/// face's plane class. A coaxial row, or a sphere's centre on an axis,
/// holds where the two cells' axes ([`stated::axis_of`]) are in one
/// class: the same `Axis` read twice, directly or as projections of
/// one construction.
fn stated_by_the_mates<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    stated: &stated::Stated,
    row: &NamedCoincidence,
    constructions: &[Result<Construction, Unwalked>],
    tol: geom_core::Tol,
) -> bool {
    use crate::names::EntityKind;
    use stated::Key;
    let kind = |cell: &NamedCell| match cell {
        NamedCell::Entity { name, .. } => Some(name.kind),
        NamedCell::Tool { .. } | NamedCell::Piece { .. } => None,
    };
    let ([Ok(a), Ok(b)], [ka, kb]) = (constructions, row.cells.as_slice()) else {
        return false;
    };
    let face = Some(EntityKind::Face);
    let on = |face: &Construction, other: &Construction| {
        stated
            .class(&Key::plane(face))
            .into_iter()
            .any(|(g, _)| lies_on(doc, other, g, tol))
    };
    match (row.relation, kind(ka), kind(kb)) {
        _ if axis_row(row) => one_axis(doc, stated, a, b),
        (topo::Relation::SameOpposite | topo::Relation::SameOriented, k, l)
            if k == face && l == face =>
        {
            stated.sense(&Key::plane(a), &Key::plane(b))
                == Some(row.relation == topo::Relation::SameOpposite)
        }
        (topo::Relation::OnCarrier, k, l) => (k == face && on(a, b)) || (l == face && on(b, a)),
        _ => false,
    }
}

/// **The axis the first rung reads a cell whose construction is
/// `cell` about** ([`stated::axis_of`]): its revolve's `axis` output for
/// a band a revolve of `doc` mints, else the cell's own. Public so the
/// arm is checked against the symbolic evaluation as the door reads it.
#[must_use]
pub fn axis_of<P>(doc: &crate::doc::Doc<P>, cell: &Construction) -> Construction {
    stated::axis_of(doc, cell)
}

/// **Whether a row is about two cells' axes alone**: a coaxial row,
/// or a sphere's centre read on an axis (its sphere a band about the
/// axis it is on, [`stated::axis_of`]).
fn axis_row(row: &NamedCoincidence) -> bool {
    row.relation == topo::Relation::Coaxial
        || matches!(
            row.site,
            topo::DecisionSite::CoaxialSphere | topo::DecisionSite::SphereOnTorusAxis
        )
}

/// **Whether two cells' axes are one `Axis` read twice** by the mates'
/// equivalence ([`stated::Stated`]), whatever their senses.
fn one_axis<P>(
    doc: &crate::doc::Doc<P>,
    stated: &stated::Stated,
    a: &Construction,
    b: &Construction,
) -> bool {
    stated
        .sense(&stated::Key::axis(doc, a), &stated::Key::axis(doc, b))
        .is_some()
}

/// **Whether `cell` lies on the face `face`, as their minter built
/// them**: both read through one chain of maps to one minting node,
/// which minted the edge or vertex `cell` in `face`'s closure. A rim
/// edge or a cap vertex lies on its cap; a rim edge on the wall it
/// bounds; an extrude's strut or cap vertex on each wall whose run it
/// bounds, read off the profile's pieces ([`crate::ProfilePieces::bounds`]).
/// A part's entity is minted at its instance, where the walk stops, so
/// the two names are read on inside the part: there a carried entity
/// stands where the one it carries does, so a cell carried alike with
/// a face lies on it where the two carried names do, and the profile
/// pieces are the part's, which this document does not hold. Anything
/// else is `false`, whether or not it lies there.
fn lies_on<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    cell: &Construction,
    face: &Construction,
    tol: geom_core::Tol,
) -> bool {
    if on_its_tool(doc, cell, face, tol) {
        return true;
    }
    let (Origin::Minted(cell_name), Origin::Minted(face_name)) = (&cell.origin, &face.origin)
    else {
        return false;
    };
    if cell.maps != face.maps {
        return false;
    }
    // A walk that stopped at an instance holds the part's names, whose
    // profiles this document does not hold.
    if stated::in_a_part(doc, cell) {
        return on_face(cell_name, face_name, &|_, _, _| false);
    }
    // The profile an extrude at `node` of this document sweeps.
    let bounds =
        |node: RecipeNodeId, run: &crate::names::PieceRun, v: &crate::names::ProfileVertexRef| {
            let Some(doc) = P::evaluable(doc) else {
                return false;
            };
            let Some(Node::Extrude { profile, .. }) = doc.node(node) else {
                return false;
            };
            let Some(Node::Profile(program)) = doc.operation_of(*profile).and_then(|p| doc.node(p))
            else {
                return false;
            };
            program
                .pieces(&doc.var_env::<f64>(), tol)
                .is_ok_and(|pieces| pieces.bounds(run, v))
        };
    on_face(cell_name, face_name, &bounds)
}

/// **Whether `cell` is a split's crossing vertex or section edge and
/// `face` its tool's plane** ([`plane`]), or, for a section edge, the
/// operand face it runs across: each lies there as the split built it,
/// read through the maps above the split. A half's copy of an operand
/// vertex the tool happens to pass through (`OnToolVertex`) lies there
/// by its values alone, and is not one.
fn on_its_tool<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    cell: &Construction,
    face: &Construction,
    tol: geom_core::Tol,
) -> bool {
    let Origin::Minted(name) = &cell.origin else {
        return false;
    };
    let seg = name.path.first();
    if !matches!(
        seg,
        Some(RoleSeg::CrossingVertex { .. } | RoleSeg::SectionEdge { .. })
    ) {
        return false;
    }
    let Some(Node::Split { target, tool }) = doc.node(name.node) else {
        return false;
    };
    let below = |on: Construction| Construction {
        origin: on.origin,
        maps: cell.maps.iter().cloned().chain(on.maps).collect(),
    };
    if *face == below(plane(doc, tool, tol)) {
        return true;
    }
    match seg {
        Some(RoleSeg::SectionEdge { face: across }) => {
            construction(doc, target.read, across, tol).is_ok_and(|on| *face == below(on))
        }
        _ => false,
    }
}

/// [`lies_on`] by the two minted names. `bounds` answers whether a
/// vertex bounds a wall's run, off the profile of the extrude a name's
/// node is; inside a part it answers `false`.
fn on_face(
    cell: &StableName,
    face: &StableName,
    bounds: &dyn Fn(RecipeNodeId, &crate::names::PieceRun, &crate::names::ProfileVertexRef) -> bool,
) -> bool {
    use crate::names::attribute::{SegOrigin, origin};
    if cell.node != face.node || face.kind != crate::names::EntityKind::Face {
        return false;
    }
    let ([seg], [wall]) = (cell.path.as_slice(), face.path.as_slice()) else {
        return false;
    };
    match (seg, wall) {
        (RoleSeg::RimEdge(end, _) | RoleSeg::CapVertex(end, _), RoleSeg::Cap(cap)) => end == cap,
        (RoleSeg::RimEdge(_, run), RoleSeg::Lateral(wall)) => run == wall,
        (RoleSeg::LateralEdge(v) | RoleSeg::CapVertex(_, v), RoleSeg::Lateral(wall)) => {
            bounds(cell.node, wall, v)
        }
        (carried, carrying) => {
            let (SegOrigin::Carried(of, _), SegOrigin::Carried(on, _)) =
                (origin(carried), origin(carrying))
            else {
                return false;
            };
            let mut same = carried.clone();
            same.each_name_mut(&mut |held| {
                if let crate::names::HoldMut::Shared(held) = held {
                    *held = on.clone().into();
                }
            });
            same == *carrying && on_face(of, on, bounds)
        }
    }
}

/// **The construction of a plane**: a plane read off a face is that
/// face's ([`construction`]), its sense aside, so a flip of one is the
/// face's too. Any other plane — a frame's coordinate plane, a slid or
/// spun one — is its pose ([`Origin::Pose`]), and so is one read off a
/// face the walk does not reach: two reads of one pose are still one
/// plane.
fn plane<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    plane: &crate::pose::RoleRead<crate::VarId>,
    tol: geom_core::Tol,
) -> Construction {
    use crate::pose::PoseDef;
    let pose = || Construction {
        origin: Origin::Pose(plane.read, plane.role),
        maps: Vec::new(),
    };
    if plane.role.is_some() {
        return pose();
    }
    match doc.var(plane.read).and_then(|var| var.def().pose()) {
        Some(PoseDef::Flip { pose }) => self::plane(doc, pose, tol),
        Some(PoseDef::Plane { face }) => {
            match doc.selection(*face).map(|s| (s, s.names.as_slice())) {
                Some((select, [name])) => {
                    construction(doc, select.body, name, tol).unwrap_or_else(|_| pose())
                }
                _ => pose(),
            }
        }
        _ => pose(),
    }
}

/// **The construction of the entity `name` names in `read`'s value**:
/// the walk down from the read to the node that minted it.
///
/// At each node the walk either passes a name-carrying edge
/// ([`crate::names::verbatim_edge`], N1), which adds no name segment,
/// or reads the name's outermost segment there. A `Part` and a split
/// that leaves the entity intact pass it through whole. A mirror is a
/// map: the cell is its source's, read through the mirror, except that
/// a reflection fixes its plane, so a cell of the plane's own
/// construction ([`plane`]), or one lying on that face ([`lies_on`]),
/// is the source's cell read again. A minted role ends the walk,
/// except a split's section face, which lies on the split's tool
/// ([`plane`]), and a crossing vertex, read against the split's
/// `Above` half ([`crossing`]). A carried segment names the entity in the read that
/// segment says — a `From`'s own read, an `Instance`'s pattern input,
/// a one-body operation's target — where the walk continues.
///
/// # Errors
///
/// [`Unwalked`], naming the node the walk stopped at.
pub fn construction<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    read: crate::VarId,
    name: &StableName,
    tol: geom_core::Tol,
) -> Result<Construction, Unwalked> {
    use crate::names::VerbatimEdge;
    use crate::names::attribute::{SegOrigin, origin};
    let node = |at: RecipeNodeId| doc.node(at).ok_or(Unwalked::Absent(at));
    let mut maps = Vec::new();
    let (mut read, mut name) = (read, name.clone());
    let mut at = doc.operation_of(read).ok_or(Unwalked::Unread(read))?;
    loop {
        // A `⊔`'s name of a member's entity, read in that member, is
        // the member's own name.
        if matches!(doc.node(name.node), Some(Node::Assemble { .. }))
            && let [RoleSeg::From { read: member, of }] = name.path.as_slice()
            && *member == read
        {
            name = (**of).clone();
            continue;
        }
        // A pick is its piece of the copy, placed there.
        if let Some(pick) = doc.pick(read) {
            maps.push(Map::World(at));
            read = *pick.of.last().ok_or(Unwalked::Unread(read))?;
            at = operation(doc, at, read)?;
            continue;
        }
        while at != name.node {
            read = match crate::names::verbatim_edge(node(at)?) {
                // A copy carries its source's names verbatim.
                Some(VerbatimEdge::Copied {
                    source: Some(source),
                }) => {
                    maps.push(Map::World(at));
                    source
                }
                // A part's entity enters at its instance, where the
                // walk stops: its name is the part's, in the part's ids.
                Some(VerbatimEdge::Copied { source: None }) => {
                    maps.push(Map::World(at));
                    return Ok(Construction {
                        origin: Origin::Minted(name),
                        maps,
                    });
                }
                Some(VerbatimEdge::Intact { target }) => target,
                Some(VerbatimEdge::Reflected { body, plane }) => {
                    let source = construction(doc, body, &name, tol)?;
                    let fixed = self::plane(doc, plane, tol);
                    if source != fixed && !lies_on(doc, &source, &fixed, tol) {
                        maps.push(Map::Mirror(at));
                    }
                    maps.extend(source.maps);
                    return Ok(Construction {
                        origin: source.origin,
                        maps,
                    });
                }
                None => return Err(Unwalked::Through(at)),
            };
            at = operation(doc, at, read)?;
        }
        let seg = name.path.first().ok_or(Unwalked::Unclassified(at))?;
        let of = match origin(seg) {
            SegOrigin::Minted => {
                return match (seg, node(at)?) {
                    (RoleSeg::SectionFace, Node::Split { tool, .. }) => {
                        let tool = plane(doc, tool, tol);
                        maps.extend(tool.maps);
                        Ok(Construction {
                            origin: tool.origin,
                            maps,
                        })
                    }
                    (RoleSeg::SectionFace, _) => Err(Unwalked::Misplaced(at)),
                    (RoleSeg::CrossingVertex { .. }, Node::Split { .. }) => {
                        let half = split_half(doc, read).ok_or(Unwalked::Misplaced(at))?;
                        Ok(Construction {
                            origin: Origin::Minted(crossing(name, half, at)?),
                            maps,
                        })
                    }
                    _ => {
                        let mut minted = name;
                        minted
                            .path
                            .truncate(crate::names::role::fragment_tail_start(&minted.path));
                        Ok(Construction {
                            origin: Origin::Minted(minted),
                            maps,
                        })
                    }
                };
            }
            SegOrigin::Unclassified => return Err(Unwalked::Unclassified(at)),
            SegOrigin::Carried(of, _) => of.clone(),
        };
        // A member is its operation at one value of its indices: its
        // name is that operation's own, read on at the same node.
        if let RoleSeg::Member { at: values, .. } = seg {
            maps.push(Map::Member(at, values.clone()));
            name = of;
            continue;
        }
        read = carried_read(seg, node(at)?, at)?;
        at = operation(doc, at, read)?;
        name = of;
    }
}

/// **The half of a split the read `read` reads**: the split's output
/// port it is, through a selection of it. `None` for any other read.
fn split_half<P>(doc: &crate::doc::Doc<P>, read: crate::VarId) -> Option<crate::SplitHalf> {
    let read = doc.selection(read).map_or(read, |select| select.body);
    match doc.var(read)?.def() {
        crate::var::VarDef::Output { port, .. } => crate::SplitHalf::ALL
            .into_iter()
            .find(|half| half.port() == *port),
        _ => None,
    }
}

/// **A split's crossing vertex as one construction across its halves**:
/// the split, the crossed edge and the sense read against the
/// [`crate::SplitHalf::Above`] half. Each half holds its own copy of a
/// crossing, its sense against that half, so the `Below` copy's sense
/// is the `Above` copy's read the other way (`Sense::other_half`). A
/// crossing holds one piece of the edge on each side, so its sense
/// always crosses.
///
/// # Errors
///
/// [`Unwalked::Misplaced`] at `at` for a `Below` copy whose sense no
/// split reads on its line, which has no other half's read.
fn crossing(
    mut name: StableName,
    half: crate::SplitHalf,
    at: RecipeNodeId,
) -> Result<StableName, Unwalked> {
    if half == crate::SplitHalf::Below
        && let Some(RoleSeg::CrossingVertex { edge, sense }) = name.path.first_mut()
    {
        *sense = sense.other_half(edge).ok_or(Unwalked::Misplaced(at))?;
    }
    Ok(name)
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
/// segment and the node together: a `From`'s own read, a split's
/// target.
///
/// # Errors
///
/// [`Unwalked::Misplaced`] for a segment `at`'s kind does not carry.
fn carried_read<P>(
    seg: &RoleSeg,
    node: &Node<P>,
    at: RecipeNodeId,
) -> Result<crate::VarId, Unwalked> {
    let misplaced = Unwalked::Misplaced(at);
    match (seg, node) {
        (RoleSeg::From { read, .. }, _) => Ok(*read),
        // Every other carried segment is a split's, carried from its
        // target.
        (_, Node::Split { target, .. }) => Ok(target.read),
        _ => Err(misplaced),
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
    pub tool: Option<crate::pose::RoleRead<crate::VarId>>,
}

/// **The kernel's rows, named**: each cell by its name in the input
/// table its key is in.
///
/// # Errors
///
/// [`NamingError::Emission`] for a tool cell of a node that reads no
/// tool (`work/intent/a-pose-defined-split-tool-names-no-tool-cell.md`),
/// or ([`ROW_CELL_NOT_IN_TABLE`]) a cell the input's table does not
/// name.
///
/// # Panics
///
/// For a cell of a body the node did not read: a result cell, which
/// only the at-rest census records and whose rows never come here, or
/// operand B's of a node that reads one body. The kernel records only the operands it was handed.
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
                .map(|read| NamedCell::Tool {
                    input: read.read,
                    role: read.role,
                })
                .ok_or_else(unnamed),
            topo::RowCell::Result { cell } => unreachable!(
                "a node's coincidence row names result cell {cell:?}, though the at-rest census is \
                 the only producer of a result cell (no import mints one) and its rows reach the \
                 at-rest checks, never `name_rows`"
            ),
            topo::RowCell::Input { input, cell } => {
                let (node, table) = match input {
                    topo::Operand::A => inputs.a,
                    topo::Operand::B => inputs.b.unwrap_or_else(|| {
                        unreachable!(
                            "a coincidence row names operand B's {cell:?} for a node that reads \
                             one body, though the kernel records only the operands it is handed"
                        )
                    }),
                };
                let key = match cell {
                    topo::Cell::Vertex(v) => EntityKey::Vertex(v),
                    topo::Cell::Edge(e) => EntityKey::Edge(e),
                    topo::Cell::Face(f) => EntityKey::Face(f),
                };
                let name =
                    table
                        .name_of(&EntityRef { body: 0, key })
                        .ok_or(NamingError::Emission {
                            what: ROW_CELL_NOT_IN_TABLE,
                        })?;
                Ok(NamedCell::Entity {
                    input: node,
                    name: name.clone(),
                })
            }
        }
    };
    rows.iter()
        .map(|row| {
            let cells = row
                .every_cell()
                .map(|cell| name(&cell))
                .collect::<Result<Vec<_>, _>>()?;
            if row.relation == topo::Relation::Meet {
                return NamedCoincidence::meet(
                    cells,
                    row.site,
                    (row.margin, row.discharge),
                    RowOrigin::Node,
                );
            }
            Ok(NamedCoincidence {
                cells,
                relation: row.relation,
                site: row.site,
                margin: row.margin,
                discharge: row.discharge,
                origin: RowOrigin::Node,
            })
        })
        .collect()
}

/// **A profile's decided junctions, as rows** (D1's profile tangency):
/// each tangent joint validation decided Zero that no constructor made,
/// its two cells the pieces the arriving and leaving segments are. The
/// rows come in canonical order: loops outer first, joints ascending.
///
/// # Panics
///
/// For a canonical segment `pieces` does not name: the pieces and the
/// validated profile were built from one program by one pass, so this
/// is a kernel bug.
pub(crate) fn name_junctions<T: geom_core::Real>(
    profile: RecipeNodeId,
    validated: &profile::ValidatedProfile<T>,
    pieces: &crate::eval::ProfilePieces,
) -> Vec<NamedCoincidence> {
    let mut rows = Vec::new();
    for (loop_index, lp) in validated.loops().iter().enumerate() {
        let n = lp.segments().len();
        let piece = |segment: usize| -> NamedCell {
            let Some(&piece) = pieces
                .edges
                .get(loop_index)
                .and_then(|edges| edges.get(segment))
            else {
                unreachable!(
                    "profile {profile:?}'s junction names segment {segment} of loop {loop_index}, \
                     which its pieces do not hold, though the pieces and the validated profile \
                     are built from one program by one pass"
                )
            };
            NamedCell::Piece { profile, piece }
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
                cells: vec![piece((joint + n - 1) % n), piece(joint)],
                relation,
                site: topo::DecisionSite::ProfileJunction,
                margin: decided.margin,
                discharge: topo::Discharge::of(decided.margin),
                origin: RowOrigin::Node,
            });
        }
    }
    rows
}
