//! **The at-rest check** (D10, Assertions; ASSEMBLY A5): what the
//! census at each `⊔` found between its arrivals, as the registry's
//! `AtRest` resident reports it, and the quieting rule's one home: the
//! rule that matches a holding assertion, and the mint of the assertion
//! a loud finding carries, which keeps only what that rule takes
//! ([`quieting`], `could_quiet`).
//!
//! The census decides each pair of arrivals apart, in contact,
//! overlapping or undecided, and nothing it finds refuses. A contact is
//! a row the `unproven-coincidence` lint reads ([`AtRestRow`]), quiet
//! under [`quiet_row`]. An overlap of two arrivals' material is an
//! [`InterferenceFinding`]: the census decides that two copies interfere
//! (a vertex of one strictly inside the other's material, an edge
//! piercing a face, or an in-plane crossing whose side test reads
//! `SameSide`), the check intersects the two copies, and each connected
//! solid of the result is one finding, named by the faces of the two
//! copies that bound it. An overlap the kernel cannot bound is one
//! [`Overlap::Unlocalized`] finding for the pair, loud, which nothing
//! quiets. Every other verdict is a pair the census could not look at
//! ([`AtRestEvidence`]): loud, unquietable, and never refusing (CHECKS
//! DS6).
//!
//! **What quiets an interference** (`quiet_verdicts`): a holding
//! assertion that reads a `Gap`'s output directly, over an opposed pair
//! of faces of the two copies, admitting only negative values (`≤ b` or
//! `= b`, `b` decided negative), whose two faces both bound the overlap,
//! and between whose carriers the overlap lies, which the kernel's
//! boolean decides (`beyond`). **What quiets an unproven row**
//! ([`quiet_row`]): a holding `= 0` over the measure that says what it
//! decided, an at-rest row's or an operation's alike: a `Gap` between
//! its two cells for a contact, a flush pair or a tangency, read on
//! their sides at whatever dimension they are, and a `Distance` between
//! the poses read off them, or the split's tool, for a coincidence of
//! poses. Both read the assertion's verdict,
//! never its measure's value, and an assertion speaks for nothing but
//! its own site: every other finding is loud.
//!
//! A part is one copy of the document instantiating it, and its own
//! census decided the meetings inside that copy: those findings are the
//! part's, carried up and reported as the part's ([`CarriedInterference`],
//! [`CarriedAtRestFinding`]). A census reading a body some door already
//! censused as one arrival (a `⊔`'s, or an instance's copy) discards what
//! it finds inside it (`product::gather_sources`, [`finding_group`]).

use crate::say;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Sign, Tol};
use topo::{AtRestPolicy, Body, FaceKey, SolidKey, ValidationError};

use crate::VarId;
use crate::doc::Doc;
use crate::eval::{Evaluation, ValuePayload};
use crate::measure::{AssertionRelation, AssertionVerdict, MeasurePrimitive};
use crate::names::{EntityKey, EntityRef, Entry, NameTable, StableName};
use crate::node::{Node, RecipeNodeId};
use crate::product::{GatheredCopy, Product};

use super::quieting::{OpposedPair, Quieting, QuietingRef, Reference};

mod carried;

pub use carried::{
    AtRestRow, CarriedAtRestFinding, CarriedCoincidence, CarriedCoincidenceRows, CarriedFindings,
    CarriedInterference, PinnedDoc, Route, RouteHop,
};
pub(crate) use carried::{DocRows, PartRow, Pinned, once_per_part_row};

/// **Why the at-rest check could not look** (CHECKS DS6): one finding,
/// loud, unquietable, and refusing at no severity, since what the check
/// could not judge is not something the document said.
#[derive(Debug, Clone, PartialEq)]
pub enum AtRestEvidence {
    /// A pair between two arrivals the census had no lane for or left
    /// undecided, or a touch between them it could not clear (kept as a
    /// finding, not a row: [`topo::record_at_rest`]). The census's own
    /// verdict.
    NoLane(ValidationError),
    /// The gathered body fails a check of its own structure or geometry.
    Malformed(ValidationError),
    /// A touch or an overlap inside one arrival that no carried record
    /// backs and no carried finding matches, or a stale record: the
    /// arrival's door's defect, found again by the census here.
    InsideArrival(ValidationError),
    /// A `⊔` the product does not read whose operands did not gather.
    NotGathered(crate::product::ProductRefusal),
    /// A part's could-not-look finding, crossed with its route.
    Carried(Box<CarriedAtRestFinding>),
}

// The kernel's own verdict, forwarded verbatim and ended at rest, the
// door the check reads it at: its tier-3′ messages end in their own
// recourse.
impl crate::spoken::Say for AtRestEvidence {
    fn say(&self, f: &mut crate::Sentence, by: crate::spoken::Speaker<'_>) -> core::fmt::Result {
        match self {
            Self::NoLane(error) | Self::Malformed(error) | Self::InsideArrival(error) => {
                write!(f, "{}", error.render(geom_brep::recourse::Reading::AtRest))
            }
            Self::NotGathered(refusal) => say!(
                f,
                "its operands do not gather: ",
                crate::spoken::Said(refusal, by)
            ),
            Self::Carried(row) => say!(f, crate::spoken::Said(&**row, by)),
        }
    }
}

/// The evidence where no document is at hand.
impl core::fmt::Display for AtRestEvidence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::display_said(self, crate::spoken::Speaker::TAG, f)
    }
}

/// **One census's verdicts, read** ([`reading`]): the interference
/// findings between its arrivals, and what it could not look at.
pub(crate) struct Reading {
    pub(crate) interference: Vec<InterferenceFinding>,
    pub(crate) could_not_look: Vec<AtRestEvidence>,
}

/// **What a document's at-rest check reads, whole**: each gather's
/// [`Reading`] together (the product's and each door's the product does
/// not read, [`Product::assembles`]), a door whose operands did not
/// gather one could-not-look finding, and the rows its parts carried
/// up, each part row once. What a part's instance carries across the
/// seam as the part's own report.
pub(crate) struct WholeReading {
    pub(crate) reading: Reading,
    pub(crate) carried_interference: Vec<CarriedInterference>,
    pub(crate) carried_at_rest: Vec<CarriedAtRestFinding>,
}

/// [`WholeReading`] over `product`, a gather of `doc`, and the gathers
/// beside it.
pub(crate) fn whole_reading<P: crate::ProfilePayload, T: LocalizeLane>(
    doc: &Doc<P>,
    product: &Product<T>,
    tol: Tol,
) -> WholeReading {
    let mut whole = WholeReading {
        reading: Reading {
            interference: Vec::new(),
            could_not_look: Vec::new(),
        },
        carried_interference: Vec::new(),
        carried_at_rest: Vec::new(),
    };
    let gathers = std::iter::once(Ok(product))
        .chain(product.assembles.iter().map(|door| door.gather.as_deref()));
    for gather in gathers {
        let product = match gather {
            Ok(product) => product,
            Err(refusal) if refusal.kind().means_no_body() => continue,
            Err(refusal) => {
                whole
                    .reading
                    .could_not_look
                    .push(AtRestEvidence::NotGathered(refusal.clone()));
                continue;
            }
        };
        let Reading {
            interference,
            could_not_look,
        } = reading(doc, product, tol);
        whole.reading.interference.extend(interference);
        whole.reading.could_not_look.extend(could_not_look);
        once_per_part_row(
            &mut whole.carried_interference,
            product.carried_interference.iter().cloned(),
        );
        once_per_part_row(
            &mut whole.carried_at_rest,
            product.carried_at_rest.iter().cloned(),
        );
    }
    whole
}

/// **The at-rest check's reading of one gather**: the census's verdicts
/// as the gather kept them ([`Product::census`]), partitioned into the
/// interference between arrivals ([`partition`]) and the rest, each of
/// which is a could-not-look finding. A contact between arrivals
/// is not among the verdicts: the gather recorded it as a row.
///
/// An interference finding carries the pair an author writes a quieting
/// `Gap` over ([`OpposedPair`]) where the rule takes what they write
/// ([`written_pair`]), a family's copy by the name its member wraps.
pub(crate) fn reading<P: crate::ProfilePayload, T: LocalizeLane>(
    doc: &Doc<P>,
    product: &Product<T>,
    tol: Tol,
) -> Reading {
    let errors = product.census.clone();
    if errors.is_empty() {
        return Reading {
            interference: Vec::new(),
            could_not_look: Vec::new(),
        };
    }
    let band = Band::linear(tol).ok();
    let write = |outer, inner| written_pair(doc, product, outer, inner, band?);
    let (interference, rest) = partition(product, errors, tol, &write);
    let groups = copy_groups(&product.copies);
    let could_not_look = rest
        .into_iter()
        .map(|error| could_not_look(&product.body, &groups, error))
        .collect();
    Reading {
        interference,
        could_not_look,
    }
}

/// **Which could-not-look finding a census verdict is**: inside one
/// arrival, or a stale record of one ([`AtRestEvidence::InsideArrival`]);
/// a census verdict between arrivals or naming none
/// ([`AtRestEvidence::NoLane`]); or a finding about the gathered body's
/// own structure ([`AtRestEvidence::Malformed`]), an arc end off its
/// corner among them (two of the kernel's readings of one body
/// disagree).
///
/// A contradicted contact declaration between two arrivals is
/// `NoLane`: inside one arrival it is that arrival's door's record,
/// contradicted, but between two no door at rest declares a contact
/// (mates declare none), so it is a record the census could not square
/// with what it read, which nothing the document says can answer. An
/// escalation names no entity, so even inside a nested `⊔`'s arrival
/// the discard cannot place it there, and it is `NoLane`
/// (`work/contacthold/an-escalated-census-finding-names-no-pair.md`).
fn could_not_look<T: Decide>(
    body: &Body<T>,
    groups: &BTreeMap<SolidKey, u32>,
    error: ValidationError,
) -> AtRestEvidence {
    if matches!(error, ValidationError::StaleContactDeclaration { .. })
        || finding_group(body, groups, &error).is_some()
    {
        return AtRestEvidence::InsideArrival(error);
    }
    if topo::census_finding_solids(body, &error).is_some()
        || matches!(
            error,
            ValidationError::CensusEscalated { .. }
                | ValidationError::CensusUnsupported { .. }
                | ValidationError::CensusLaneUnsupported { .. }
                | ValidationError::CensusUndecidable { .. }
                | ValidationError::UnrecordedContact { .. }
        )
    {
        return AtRestEvidence::NoLane(error);
    }
    AtRestEvidence::Malformed(error)
}

/// The funnel site name of the contact quieting rule's decision that an
/// assertion's bound is zero.
pub(crate) const AT_REST_QUIET_CONTACT_BOUND: &str = "at_rest_quiet_contact_bound";

/// **The row half of the quieting rule** (D10 `[assertions.quiet]`):
/// the first assertion, in document order, that quiets `row`, or `None`
/// when the row stays loud.
///
/// One rule for every row the lint reads, an at-rest row's or an
/// operation's: an at-rest row's cells are its arrivals', an
/// operation's row's the deciding node's input reads' (`[operations]`,
/// *Selections*), so a cell the result merged away is still selected
/// on its operand.
///
/// An assertion quiets a row when it relates a measure that says what
/// the row decided ([`saying`]) by `=` to a bound decided zero, read
/// directly, its verdict in `evaluation` is `Holds`, and its two
/// references are the row's two cells, each paired with the cell it
/// reads, in either order. A reference reads a cell where it is the
/// cell's read (or a pick of that read's piece) and the cell's name
/// there, or a pose read directly off such a cell of the kind the cell's
/// carrier has ([`pose_kind`]: an axis off a cylinder, a cone or a
/// torus, a centre off a sphere); it reads a split's tool where it is
/// the tool's read, whatever pose that is.
///
/// **Two axes one.** `Distance(axis, axis)` is the distance between two
/// lines, zero where they coincide or meet. A coaxial row is said by it
/// only where the reading takes the parallel arm: the rule decides the
/// two cells' axes parallel at the measure's own site
/// (`carrier_cyl_axis_parallel`, levered by the faces' reach), so a
/// `Distance = 0` over two axes that only meet quiets no coaxial row.
/// The kernel records a coaxial row only over axes it decided one, so
/// this costs a document nothing; after an edit that tilts the axes
/// until they only meet, the kernel records the meeting instead, which
/// the same assertion says.
///
/// **Families.** An assertion whose reads reach an index is one
/// assertion per member (D10 `[assertions]`, "Families"), each by its
/// own verdict, and each member's reference names its cell as that
/// member reads it ([`member_name`]): the member at `k` quiets the row
/// over member `k`'s cell, and no other member's.
///
/// The rule matches cells, so another contact between the same two
/// reads stays loud, and it reads verdicts only, so an assertion that
/// failed or did not evaluate quiets nothing.
pub(crate) fn quiet_row<P, T>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    row: &crate::coincide::NamedCoincidence,
    tol: Tol,
) -> Option<RecipeNodeId>
where
    P: crate::ProfilePayload,
    T: crate::mate::SolveScalar + topo::AtRestPolicy,
{
    let said = saying(row)?;
    let band = Band::linear(tol).ok()?;
    let kinds = if said.poses {
        said.pose_kinds(&said.read(doc, evaluation)?, band)?
    } else {
        [None, None]
    };
    holding_measures(doc, evaluation).find_map(|held| {
        if held.relation != Quieting::RELATION
            || decide(AT_REST_QUIET_CONTACT_BOUND, Margin::of(*held.bound), band) != Ok(Sign::Zero)
        {
            return None;
        }
        let reads = |var: VarId, i: usize| {
            reference(doc, var)
                .iter()
                .any(|r| said.reads_cell(doc, r, &held.at, i, kinds[i]))
        };
        said.orders()
            .any(|(verb, [i, j])| verb == held.verb && reads(held.a, i) && reads(held.b, j))
            .then_some(held.id)
    })
}

/// **The assertion that would quiet `row`** (D10 `[assertions.quiet]`),
/// in `evaluation`, minted so that [`quiet_row`] takes it: of the
/// measures that say what the row decided ([`saying`]), over the row's
/// two cells in each order ([`Saying::orders`]), the first decided zero
/// at the site the rule decides a bound at ([`Saying::zeros`]), written
/// as an author writes it ([`Saying::minted`]): each cell a selection of
/// its name in its read, a family member's by the name its member wraps.
/// The mint keeps a writing only where the rule's own test
/// ([`Saying::reads_cell`]) reads it as the row's two cells, and, for a
/// writing whose reads reach an index, only where every member of the
/// family it is reads zero: an assertion one of whose members is
/// violated is no recourse.
///
/// `None` where no such assertion exists: a cell the measure has no form
/// for, a name that does not resolve, a measure read non-zero, two
/// members of one family (an assertion reading both reads them by
/// indexed reads, which the rule matches no cell by), a family some
/// member of which does not read zero. Such a row's finding never
/// refuses (CHECKS DS6), and its recourse names the construction alone.
pub(crate) fn quieting<P, T>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    row: &crate::coincide::NamedCoincidence,
    tol: Tol,
) -> Option<Quieting>
where
    P: crate::ProfilePayload,
    T: crate::mate::SolveScalar + topo::AtRestPolicy,
{
    let band = Band::linear(tol).ok()?;
    let said = saying(row)?;
    // A tool that repeats has no plane at the document's own values:
    // each order that says the row is put to every member instead.
    let tool = said.cells.iter().any(|c| matches!(c, RowCell::Tool(_)));
    let (kinds, zeros) = match said.zeros(doc, evaluation, &evaluation.env, band, tol) {
        Some(read) => read,
        None if tool => ([None, None], said.orders().collect()),
        None => return None,
    };
    zeros
        .into_iter()
        .find_map(|(verb, order)| said.minted(doc, evaluation, verb, order, kinds, band, tol))
}

/// **An assertion's reference, as the rule can read it**: the variable
/// itself, which reads a split's tool where it is the tool's read,
/// whatever pose that is; and, where it is a selection of one name in a
/// body read plainly or a pose read directly off one
/// (`eval::read_directly_off`), that selection and pose. An indexed read
/// is no selection the rule reads.
fn reference<P: crate::ProfilePayload>(doc: &Doc<P>, var: VarId) -> Vec<QuietingRef> {
    let selected = |var: VarId| {
        let select = doc
            .selection(var)
            .filter(|select| !select.body.is_indexed())?;
        let [name] = select.names.as_slice() else {
            return None;
        };
        Some((select.body.read, name.clone()))
    };
    let entity = match selected(var) {
        Some((read, name)) => Some(QuietingRef::entity(read, name, None)),
        None => crate::eval::read_directly_off(doc, var).and_then(|(cell, kind)| {
            let (read, name) = selected(cell)?;
            Some(QuietingRef::entity(read, name, Some(kind)))
        }),
    };
    std::iter::once(QuietingRef::variable(var))
        .chain(entity)
        .collect()
}

/// **What says a row** ([`saying`]): the measures that say it and the
/// row's two cells; whether each cell is read as the pose its carrier
/// has, and whether the row decided two axes one.
struct Saying {
    verbs: &'static [crate::measure::MeasureVerb],
    cells: [RowCell; 2],
    poses: bool,
    coaxial: bool,
    /// The role a split's tool is read through.
    tool_role: Option<crate::pose::PoseRole>,
}

/// **A cell of a row, or a reference of an assertion**: an entity's
/// read and its name there, or the read of the tool a split cuts with.
#[derive(Clone, Debug, PartialEq)]
enum RowCell {
    Entity(VarId, StableName),
    Tool(VarId),
}

impl RowCell {
    /// Whether this reference reads the row's `cell`: a tool by its read,
    /// an entity by its read, or by a pick of that read's piece, whose
    /// name for the cell is keyed by each `⊔` the pick's path walks
    /// through, by the read that piece entered it by.
    fn reads<P>(&self, doc: &Doc<P>, cell: &Self) -> bool {
        match (self, cell) {
            (Self::Tool(a), Self::Tool(b)) => a == b,
            (Self::Entity(body, name), Self::Entity(read, own)) => {
                if body == read {
                    return name == own;
                }
                doc.pick(*body).is_some_and(|pick| {
                    pick.placement == *read
                        && pick
                            .of
                            .iter()
                            .try_fold(own, |held, &step| keyed_by(held, step))
                            == Some(name)
                })
            }
            _ => false,
        }
    }
}

/// **A row's entity cells, read as a measure reads its references**: in
/// one computing frame (`Framed::computing_among`), each off its own
/// body, the other's data taken through their relative motion
/// (`crate::eval::Seat`).
struct ReadCells<'e, T: topo::AtRestPolicy + Decide> {
    cells: Vec<(crate::eval::Seat<'e, T>, EntityRef)>,
    computing: &'e crate::eval::Framed<T>,
}

impl Saying {
    /// The row's entity cells read ([`ReadCells`]); `None` where a name
    /// does not resolve or a frame refuses to read, which is no
    /// assertion either.
    fn read<'e, P: crate::ProfilePayload, T: Decide + topo::AtRestPolicy>(
        &self,
        doc: &Doc<P>,
        evaluation: &'e Evaluation<T>,
    ) -> Option<ReadCells<'e, T>> {
        let mut framed = Vec::with_capacity(2);
        for (i, cell) in self.cells.iter().enumerate() {
            let RowCell::Entity(read, name) = cell else {
                continue;
            };
            let slot = crate::OperandSlot::Measured(self.verbs[0], u8::try_from(i).ok()?);
            let at =
                crate::eval::resolve_name(doc, &evaluation.nodes, slot, *read, name, |error| {
                    crate::eval::NodeErrorKind::SelectResolve {
                        slot,
                        var: *read,
                        reference: 0,
                        error,
                    }
                })
                .ok()?;
            let body = crate::eval::reference_body(&evaluation.nodes, slot, *read, at).ok()?;
            framed.push((body, at.1));
        }
        let computing = crate::eval::Framed::computing_among(framed.iter().map(|(f, _)| *f))?;
        let cells = framed
            .iter()
            .map(|(f, ent)| Some((computing.seat(f).ok()?, *ent)))
            .collect::<Option<Vec<_>>>()?;
        Some(ReadCells { cells, computing })
    }

    /// **The measures that say the row, each over the row's two cells in
    /// each order**: the measure, and which cell each of its two
    /// references reads. What [`quiet_row`] matches a holding assertion
    /// against and [`quieting`] mints from.
    fn orders(&self) -> impl Iterator<Item = (crate::measure::MeasureVerb, [usize; 2])> + '_ {
        self.verbs
            .iter()
            .flat_map(|&verb| [[0, 1], [1, 0]].map(|order| (verb, order)))
    }

    /// **The pose each cell of a row of poses is read as**
    /// ([`pose_kind`]); `None` where a cell's carrier has none, or the
    /// row decided two axes one and the reading does not take them
    /// parallel ([`axes_parallel`]).
    fn pose_kinds<T: crate::mate::SolveScalar + topo::AtRestPolicy>(
        &self,
        read: &ReadCells<'_, T>,
        band: Band,
    ) -> Option<[Option<crate::VarKind>; 2]> {
        let kinds = [
            pose_kind::<T>(read.cells.first()?)?,
            pose_kind::<T>(read.cells.get(1)?)?,
        ];
        if self.coaxial && kinds == [crate::VarKind::Axis; 2] && !axes_parallel(read, band)? {
            return None;
        }
        Some(kinds.map(Some))
    }

    /// **The one test both halves of the rule read**: whether the
    /// reference `r`, as the member at `at` of its assertion's family
    /// reads it, reads the row's cell `i`, read as a pose of `kind`
    /// where the row is one of poses. A variable reads the row's tool
    /// where it is the tool's read, whatever pose that is; an entity
    /// reads a cell where its pose is `kind` and the selection, named as
    /// that member names it ([`member_name`]), reads the cell
    /// ([`RowCell::reads`]). [`quiet_row`] asks it of a holding
    /// assertion's references, [`Self::minted`] of the references it
    /// writes.
    fn reads_cell<P: crate::ProfilePayload>(
        &self,
        doc: &Doc<P>,
        r: &QuietingRef,
        at: &crate::names::MemberAt,
        i: usize,
        kind: Option<crate::VarKind>,
    ) -> bool {
        let posed = match &r.0 {
            Reference::Read(_) => true,
            Reference::Entity { pose, .. } => *pose == kind,
        };
        posed && cell_of(doc, r, at).reads(doc, &self.cells[i])
    }

    /// **The poses the row's cells are read as, and the measures over
    /// them, in each order, that read zero** at the site the rule
    /// decides a bound at, the cells resolved and read as the measure
    /// resolves and reads them ([`crate::eval::resolve_name`]): each as
    /// the pose its carrier has where the row is one of poses, a split's
    /// tool as its plane against the vertex, read in `env` (the member's,
    /// for a tool that repeats). `None` where a cell does not read.
    #[allow(clippy::type_complexity)] // the two halves of one reading
    fn zeros<P, T>(
        &self,
        doc: &Doc<P>,
        evaluation: &Evaluation<T>,
        env: &crate::expr::VarEnv<T>,
        band: Band,
        tol: Tol,
    ) -> Option<(
        [Option<crate::VarKind>; 2],
        Vec<(crate::measure::MeasureVerb, [usize; 2])>,
    )>
    where
        P: crate::ProfilePayload,
        T: crate::mate::SolveScalar + topo::AtRestPolicy,
    {
        use crate::eval::measure::{Carrier, Pose};
        let read = self.read(doc, evaluation)?;
        let tool = self.cells.iter().enumerate().find_map(|(i, c)| match c {
            RowCell::Tool(tool) => Some((i, *tool)),
            RowCell::Entity(..) => None,
        });
        let (kinds, carriers): (_, [Carrier<'_, T>; 2]) = if let Some((t, tool)) = tool {
            let [(seat, ent)] = read.cells.as_slice() else {
                return None;
            };
            let Carrier::Point { at, .. } = crate::eval::measure::carrier_of(*seat, *ent, band)
            else {
                return None;
            };
            let roled = crate::pose::RoleRead {
                read: tool,
                role: self.tool_role,
            };
            let plane =
                crate::eval::read_pose(P::evaluable(doc)?, &evaluation.nodes, roled, env, tol)
                    .ok()?;
            let plane = Carrier::Pose {
                pose: Pose::of(&plane)?,
                reach: None,
            };
            let vertex = Carrier::Pose {
                pose: Pose::Point(read.computing.point_in_space(at)),
                reach: None,
            };
            (
                [None, None],
                if t == 0 {
                    [plane, vertex]
                } else {
                    [vertex, plane]
                },
            )
        } else if self.poses {
            let kinds = self.pose_kinds(&read, band)?;
            let carrier = |i: usize| {
                let (seat, ent) = &read.cells[i];
                crate::eval::read_off_cell(*seat, *ent, kinds[i]?, band).ok()
            };
            (kinds, [carrier(0)?, carrier(1)?])
        } else {
            let carrier = |i: usize| {
                let (seat, ent) = &read.cells[i];
                crate::eval::measure::carrier_of(*seat, *ent, band)
            };
            ([None, None], [carrier(0), carrier(1)])
        };
        let zeros = self
            .orders()
            .filter(|&(verb, order)| {
                let [a, b] = order.map(|i| &carriers[i]);
                measure_sign_as(verb, a, b, AT_REST_QUIET_CONTACT_BOUND, band) == Some(Sign::Zero)
            })
            .collect();
        Some((kinds, zeros))
    }

    /// **The assertion `verb` over the cells in `order`, as an author
    /// writes it**, where the rule takes it ([`quieting`]): each cell a
    /// selection of its name in its read, or the tool's read; a cell
    /// whose name is a member's of its read's family, by the name the
    /// member wraps, which a selection of the family read plainly holds
    /// and each member of the assertion reads as its own (D10
    /// `[assertions]`, "Families"). Kept only where some member of the
    /// assertion reads the row's two cells by [`Self::reads_cell`], and
    /// every other member's two cells read zero by the same measure, as
    /// the same poses.
    #[allow(clippy::too_many_arguments)] // one reading, carried through
    fn minted<P, T>(
        &self,
        doc: &Doc<P>,
        evaluation: &Evaluation<T>,
        verb: crate::measure::MeasureVerb,
        order: [usize; 2],
        kinds: [Option<crate::VarKind>; 2],
        band: Band,
        tol: Tol,
    ) -> Option<Quieting>
    where
        P: crate::ProfilePayload,
        T: crate::mate::SolveScalar + topo::AtRestPolicy,
    {
        let written = |i: usize| match &self.cells[i] {
            RowCell::Tool(var) => QuietingRef::variable(*var),
            RowCell::Entity(read, name) => {
                QuietingRef::entity(*read, written_name(doc, *read, name), kinds[i])
            }
        };
        let references = order.map(written);
        let members = members_of::<P, T>(doc, &references)?;
        let reads_row = |at: &crate::names::MemberAt| {
            (0..2).all(|k| self.reads_cell(doc, &references[k], at, order[k], kinds[order[k]]))
        };
        if !members.iter().any(|(at, _)| reads_row(at)) {
            return None;
        }
        // Each member reads its own cells, and a repeated tool its own
        // plane: the row's reading stands for none of them.
        for (at, env) in members.iter().filter(|(_, env)| env.is_some()) {
            let mut cells = self.cells.clone();
            for k in 0..2 {
                cells[order[k]] = cell_of(doc, &references[k], at);
            }
            let member = Saying { cells, ..*self };
            let env = env.as_ref().unwrap_or(&evaluation.env);
            let (held, zeros) = member.zeros(doc, evaluation, env, band, tol)?;
            if held != kinds || !zeros.contains(&(verb, order)) {
                return None;
            }
        }
        Some(Quieting::minted(verb, references))
    }
}

/// **The name an author selects a cell named `name` in `read` by**: the
/// name a family member's `Member` segment wraps, where `name` is one of
/// `read`'s family's members, which a selection of the family read
/// plainly holds and each member reads as its own; else `name`. A guess
/// each mint keeps only where the rule reads it back.
fn written_name<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    read: VarId,
    name: &StableName,
) -> StableName {
    match name.path.as_slice() {
        [crate::names::RoleSeg::Member { of, .. }] if doc.operation_of(read) == Some(name.node) => {
            let plain: &StableName = of;
            plain.clone()
        }
        _ => name.clone(),
    }
}

/// **The cell a reference reads, as the member at `at` of its
/// assertion's family reads it**: a variable as the tool it would be, an
/// entity as its read's cell named as that member names it
/// ([`member_name`]).
fn cell_of<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    r: &QuietingRef,
    at: &crate::names::MemberAt,
) -> RowCell {
    match &r.0 {
        Reference::Read(var) => RowCell::Tool(*var),
        Reference::Entity { read, name, .. } => {
            RowCell::Entity(*read, member_name(doc, *read, name, at))
        }
    }
}

/// **The members of the assertion two references make**: one per
/// member of the indices their reads reach together (D10
/// `[repetition]`, a selection by its body read, a variable read as it
/// is by itself), each as its values for those indices and the
/// environment it evaluates in; one, empty and with no environment of
/// its own, where they reach none. `None` where two of the indices are
/// unrelated, which no assertion reads, or a count does not evaluate.
#[allow(clippy::type_complexity)] // a member and its environment
fn members_of<P: crate::ProfilePayload, T: Decide>(
    doc: &Doc<P>,
    references: &[QuietingRef; 2],
) -> Option<Vec<(crate::names::MemberAt, Option<crate::expr::VarEnv<T>>)>> {
    let mut reached = BTreeSet::new();
    for r in references {
        let read = r.read();
        reached.extend(doc.var_reach(read).chain()?.iter().copied());
    }
    let chain = crate::repetition::Reach::new(doc)
        .order(&reached)
        .chain()?
        .to_vec();
    if chain.is_empty() {
        return Some(vec![(Vec::new(), None)]);
    }
    doc.member_envs::<T>(&chain)
        .map(|member| {
            let member = member.ok()?;
            let at = chain
                .iter()
                .zip(&member.at)
                .map(|(&index, &value)| Some((index, u32::try_from(value).ok()?)))
                .collect::<Option<_>>()?;
            Some((at, Some(member.env)))
        })
        .collect()
}

/// **The pose a cell's carrier has**, as a coincidence of poses reads
/// it: an axis off a cylinder, a cone or a torus, a centre off a sphere;
/// `None` for any other cell.
fn pose_kind<T: Decide + topo::AtRestPolicy>(
    (seat, ent): &(crate::eval::Seat<'_, T>, EntityRef),
) -> Option<crate::VarKind> {
    let body = seat.body();
    use geom::SurfaceKind as S;
    let EntityKey::Face(face) = ent.key else {
        return None;
    };
    match topo::readback::face_carrier_kind(body, face).ok()? {
        S::Cylinder | S::Cone | S::Torus => Some(crate::VarKind::Axis),
        S::Sphere => Some(crate::VarKind::Point),
        _ => None,
    }
}

/// **Whether two cells' axes read parallel**, at the site and lever the
/// measure's own two-axes arm decides them (`carrier_cyl_axis_parallel`):
/// the reading a coaxial row is said by. `None` where an axis does not
/// read or the decision escalates.
fn axes_parallel<T: topo::AtRestPolicy + crate::mate::SolveScalar>(
    read: &ReadCells<'_, T>,
    band: Band,
) -> Option<bool> {
    let axis = |i: usize| {
        let (seat, ent) = &read.cells[i];
        crate::eval::read_off_cell(*seat, *ent, crate::VarKind::Axis, band).ok()
    };
    let (a, b) = (axis(0)?, axis(1)?);
    crate::eval::measure::axes_parallel(&a, &b, band)
}

/// **The sign a `Gap` assertion's measure reads over two carriers**,
/// decided at `site`: the value the measure reports
/// ([`crate::eval::measure::gap_value`]), `None` where it refuses or
/// the decision does. Both halves of the quieting rule ask it of a pair
/// an assertion could be written over.
fn gap_sign_as<T: Decide>(
    outer: &crate::eval::measure::Carrier<'_, T>,
    inner: &crate::eval::measure::Carrier<'_, T>,
    site: &'static str,
    band: Band,
) -> Option<Sign> {
    measure_sign_as(crate::measure::MeasureVerb::Gap, outer, inner, site, band)
}

/// [`gap_sign_as`] for a `Gap` or a `Distance`
/// ([`crate::eval::measure::value_of`]).
fn measure_sign_as<T: Decide>(
    verb: crate::measure::MeasureVerb,
    a: &crate::eval::measure::Carrier<'_, T>,
    b: &crate::eval::measure::Carrier<'_, T>,
    site: &'static str,
    band: Band,
) -> Option<Sign> {
    let value = crate::eval::measure::value_of(verb, a, b, band).ok()?;
    decide(site, Margin::of(value), band).ok()
}

/// **What says what a row decided** (D10 `[assertions.quiet]`): the
/// measure an assertion relates to zero, and how it reads the row's two
/// cells; `None` for a row no assertion says.
///
/// A `Gap = 0` over two faces read on their sides says they touch,
/// opposed, or lie one on the other, aligned, so it says what a row
/// decided where that is that its two cells meet, rest flush or touch
/// tangent. A `Distance = 0` between the poses read off the two cells
/// (an axis off a cylinder, a cone or a torus, a centre off a sphere)
/// says a coincidence of poses, and a `Gap = 0` or a `Distance = 0`
/// between a split's tool and a vertex says the vertex lies on it. By
/// the kinds of row the kernel decides:
///
/// | site | relation | what says it |
/// |---|---|---|
/// | `PlaneLadder`, `CarrierLadder` | `SameOpposite` (glue), `SameOriented` (flush) | `Gap` over the two faces |
/// | `CensusAtRest` | `SameOpposite`, `SameOriented`, `OnCarrier` | `Gap` over the two cells |
/// | `VertexFusion` | `OnCarrier`, `Meet` of two cells | `Gap` over the two cells |
/// | `TangentWitness` | `SurfaceTangent` | `Gap` over the two faces: an opposed tangency is a contact, an aligned one a continuation |
/// | `RimJunction` | `RimJunction` | `Gap` over the two faces: they meet on the rim both end on, and nowhere near it |
/// | `CoaxialSphere`, `SphereOnTorusAxis` | `OnCarrier` | `Distance` between the poses read off the two cells |
/// | `CoaxialCylinderTorus`, `CoaxialTori`, `BatterySupportAxis` | `Coaxial` | `Distance` between the poses, two axes read parallel (a sphere's support its centre) |
/// | `CrossingCylinders` | `AxesMeet` | `Distance` between the two axes |
/// | `CrossingCylinders` | `SameRadius` | nothing: two radii written apart |
/// | `SplitOn` | `OnCarrier`, over the tool | `Gap` or `Distance` between the tool and the vertex |
/// | `BatteryTurn` | `CommonFoot` | nothing: two feet only the blend computes |
/// | `BatteryJoint` | `Tangent` | nothing |
/// | `ProfileJunction` | `SameOriented`, `Tangent`, over pieces | nothing |
///
/// A `Meet` of more than two cells, a profile piece, and a tool read
/// through a role other than a frame's xy plane are no pair an
/// assertion is written over.
fn saying(row: &crate::coincide::NamedCoincidence) -> Option<Saying> {
    use crate::measure::MeasureVerb as M;
    use topo::{DecisionSite as Site, Relation as R};
    let (verbs, poses, coaxial): (&'static [M], _, _) = match (row.site, row.relation) {
        (Site::PlaneLadder | Site::CarrierLadder, R::SameOpposite | R::SameOriented)
        | (Site::CensusAtRest, R::SameOpposite | R::SameOriented | R::OnCarrier)
        | (Site::VertexFusion, R::OnCarrier | R::Meet)
        | (Site::TangentWitness, _)
        | (Site::RimJunction, R::RimJunction) => (&[M::Gap], false, false),
        (Site::SplitOn, _) => (&[M::Gap, M::Distance], false, false),
        (Site::CoaxialSphere | Site::SphereOnTorusAxis, _)
        | (Site::CrossingCylinders, R::AxesMeet) => (&[M::Distance], true, false),
        (Site::CoaxialCylinderTorus | Site::CoaxialTori | Site::BatterySupportAxis, _) => {
            (&[M::Distance], true, true)
        }
        _ => return None,
    };
    let mut tool_role = None;
    let mut cell = |c: &crate::coincide::NamedCell| match c {
        crate::coincide::NamedCell::Entity { input, name } => Some(RowCell::Entity(
            *input,
            keyed_by(name, *input).unwrap_or(name).clone(),
        )),
        // A tool read plainly, or a frame read as its xy plane, which a
        // measure reads a frame as.
        crate::coincide::NamedCell::Tool {
            input,
            role: role @ (None | Some(crate::pose::PoseRole::Xy)),
        } => {
            tool_role = *role;
            Some(RowCell::Tool(*input))
        }
        crate::coincide::NamedCell::Tool { role: Some(_), .. }
        | crate::coincide::NamedCell::Piece { .. } => None,
    };
    let [a, b] = row.cells.as_slice() else {
        return None;
    };
    let cells = [cell(a)?, cell(b)?];
    // A tool is a split's alone, and a split's row has one.
    let tools = cells
        .iter()
        .filter(|c| matches!(c, RowCell::Tool(_)))
        .count();
    (tools == usize::from(row.site == Site::SplitOn)).then_some(Saying {
        verbs,
        cells,
        poses,
        coaxial,
        tool_role,
    })
}

/// [`saying`]'s measures and whether it reads poses, for a test that
/// writes assertions in the rule's language.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn saying_of(
    row: &crate::coincide::NamedCoincidence,
) -> Option<(&'static [crate::measure::MeasureVerb], bool)> {
    saying(row).map(|said| (said.verbs, said.poses))
}

/// **A holding assertion that reads a `Gap`'s or a `Distance`'s output
/// directly**, the preamble both halves of the quieting rule share: the
/// assertion, its relation, the measure's verb and two references, and
/// the bound it holds at.
struct HoldingMeasure<'a, T> {
    id: RecipeNodeId,
    /// The member of the assertion's family it is, for an assertion
    /// whose reads reach an index (D10 `[assertions]`, "Families");
    /// empty for any other.
    at: crate::names::MemberAt,
    relation: AssertionRelation,
    verb: crate::measure::MeasureVerb,
    /// A `Gap`'s outer, or a `Distance`'s first.
    a: VarId,
    /// A `Gap`'s inner, or a `Distance`'s second.
    b: VarId,
    bound: &'a T,
}

/// Every [`HoldingMeasure`] of `doc` in `evaluation`, in document
/// order, an assertion whose reads reach an index once per member, each
/// member by its own verdict. It reads verdicts only: an assertion that
/// did not evaluate, or does not hold, is none of them.
fn holding_measures<'a, P, T: Decide + topo::AtRestPolicy>(
    doc: &'a Doc<P>,
    evaluation: &'a Evaluation<T>,
) -> impl Iterator<Item = HoldingMeasure<'a, T>> + 'a {
    use crate::measure::MeasureVerb as M;
    doc.ids()
        .into_iter()
        .flat_map(move |id| {
            let Some(Node::Assertion(crate::measure::Claim::Value {
                value, relation, ..
            })) = doc.node(id)
            else {
                return None;
            };
            let (measure, 0) = doc.var(*value).and_then(|v| v.def().output())? else {
                return None;
            };
            let (verb, a, b) = match doc.node(measure) {
                Some(Node::Measure {
                    primitive: MeasurePrimitive::Gap { outer, inner },
                }) => (M::Gap, *outer, *inner),
                Some(Node::Measure {
                    primitive: MeasurePrimitive::Distance { a, b },
                }) => (M::Distance, *a, *b),
                _ => return None,
            };
            let value = evaluation.value(id)?;
            let members: Vec<(crate::names::MemberAt, &'a ValuePayload<T>)> = match &value.members {
                Some(family) => family
                    .members
                    .iter()
                    .map(|(values, member)| (family.member_at(values), &member.payload))
                    .collect(),
                None => vec![(Vec::new(), &value.payload)],
            };
            Some(members.into_iter().filter_map(move |(at, payload)| {
                let ValuePayload::Assertion(AssertionVerdict::Holds { bound, .. }) = payload else {
                    return None;
                };
                Some(HoldingMeasure {
                    id,
                    at,
                    relation: *relation,
                    verb,
                    a,
                    b,
                    bound,
                })
            }))
        })
        .flatten()
}

/// **The assertion in `doc` that already says `claim`**, verdict
/// aside: a `Claim::Value` relating a measure of `claim`'s verb to zero
/// by its relation, whose two references read, in order, as
/// `claim`'s ([`reference()`]). `None` where none does.
pub(crate) fn asserted<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    claim: &Quieting,
) -> Option<RecipeNodeId> {
    use crate::measure::MeasureVerb as M;
    doc.ids().into_iter().find(|&id| {
        let Some(Node::Assertion(crate::measure::Claim::Value {
            value,
            relation,
            bound,
        })) = doc.node(id)
        else {
            return false;
        };
        let Some((measure, 0)) = doc.var(*value).and_then(|v| v.def().output()) else {
            return false;
        };
        let (verb, a, b) = match doc.node(measure) {
            Some(Node::Measure {
                primitive: MeasurePrimitive::Gap { outer, inner },
            }) => (M::Gap, *outer, *inner),
            Some(Node::Measure {
                primitive: MeasurePrimitive::Distance { a, b },
            }) => (M::Distance, *a, *b),
            Some(Node::Measure {
                primitive: MeasurePrimitive::Angle { a, b },
            }) => (M::Angle, *a, *b),
            _ => return false,
        };
        let [ra, rb] = claim.references();
        verb == claim.measure()
            && *relation == Quieting::RELATION
            && doc.var(*bound).is_some_and(
                |var| matches!(var.def(), crate::var::VarDef::Defined(zero) if zero.is_zero()),
            )
            && reference(doc, a).contains(ra)
            && reference(doc, b).contains(rb)
    })
}

/// **The name a selection holds, as the member at `at` reads it**: the
/// selection's own name where its body reaches no index of `at`, else
/// that name wrapped in the `Member` segment its family's table puts
/// round it, at the values of `at` for the indices the body reaches.
fn member_name<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    body: VarId,
    name: &StableName,
    at: &crate::names::MemberAt,
) -> StableName {
    let Some(chain) = doc.var_reach(body).chain().map(<[VarId]>::to_vec) else {
        return name.clone();
    };
    let values: Option<crate::names::MemberAt> = chain
        .iter()
        .map(|index| at.iter().find(|(held, _)| held == index).copied())
        .collect();
    match (values, doc.operation_of(body)) {
        (Some(values), Some(family)) if !values.is_empty() => StableName {
            kind: name.kind,
            node: family,
            path: vec![crate::names::RoleSeg::Member {
                at: values,
                of: crate::names::NameRef::new(name.clone()),
            }],
        },
        _ => name.clone(),
    }
}

/// **The name a read holds, of a name a `⊔` keyed by that read**: a
/// `⊔` keys each name it takes from one of its reads by the read
/// ([`crate::names::RoleSeg::From`]), so a selection reading the arrival
/// itself names it by `of`. `None` for a name `read` did not key.
pub(crate) fn keyed_by(name: &StableName, read: VarId) -> Option<&StableName> {
    match name.path.as_slice() {
        [crate::names::RoleSeg::From { read: by, of }] if *by == read => Some(of),
        _ => None,
    }
}

/// **One copy, as a finding names it**: the placement's output
/// variable, and which member of that placement's value it is (0 for a
/// placement of one body).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CopyRef {
    /// The placement's output variable.
    pub var: VarId,
    /// The output-body index within the placement's value.
    pub member: u32,
}

/// **A face of a copy**, as the copy's own name table spells it, which
/// is the name a selection of the copy reads.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FaceSite {
    /// The copy.
    pub copy: CopyRef,
    /// The face's name.
    pub face: StableName,
}

/// **Where two copies overlap.**
#[derive(Debug, Clone, PartialEq)]
pub enum Overlap {
    /// One connected overlap of the two copies' material, named by the
    /// faces that bound it, sorted and deduplicated: each on the copy
    /// whose face the intersection keeps, which for a face of one copy
    /// lying on a face of the other is the first copy's (Eq. 15.3).
    Bounded {
        /// The bounding faces.
        faces: Vec<FaceSite>,
    },
    /// The census decided that the copies overlap and the kernel could
    /// not bound the overlap. Loud, and nothing quiets it.
    Unlocalized(Unlocalized),
}

/// **Why an overlap has no site.**
#[derive(Debug, Clone, PartialEq)]
pub enum Unlocalized {
    /// The intersection of the two copies refused, in its own words.
    Refused {
        /// The refusal's `Display`.
        refusal: String,
    },
    /// The intersection is empty although the census decided an
    /// overlap: the overlap is thinner than the boolean's tolerance.
    Empty,
    /// A face of the intersection descends from no named face of
    /// either copy.
    Unnamed,
    /// The check ran at a scalar with no localizing lane
    /// ([`LocalizeLane`]): the overlap is decided and not bounded.
    Lane,
    /// An assertion's faces bound the overlap, and the kernel could not
    /// say whether the overlap lies between their carriers.
    Containment {
        /// Why, in the refusing door's words.
        refusal: String,
    },
}

/// **An overlap of two copies' material at rest** (D10): reported,
/// never refused, and quiet only under the assertion `quiet_verdicts`
/// finds.
#[derive(Debug, Clone, PartialEq)]
pub struct InterferenceFinding {
    /// The copy earlier in gather order.
    pub a: CopyRef,
    /// The other copy.
    pub b: CopyRef,
    /// The census's verdicts on the pair, in its own order: every
    /// finding of one pair carries the same list.
    pub evidence: Vec<ValidationError>,
    /// The overlap.
    pub overlap: Overlap,
    /// The assertion that quiets it, or `None` when it is loud.
    pub quiet: Option<RecipeNodeId>,
    /// The member of `quiet`'s family that quiets it, for an assertion
    /// whose reads reach an index (D10 `[assertions]`, "Families"):
    /// each member quiets its own finding. Empty for any other.
    pub quiet_at: crate::names::MemberAt,
    /// The two faces a `Gap` that would quiet it is written over
    /// ([`OpposedPair`]), asked of the rule itself ([`localize`]): the
    /// overlap is bounded; every verdict it carries lies on it
    /// (`covered`, which no assertion changes); and the pair is the
    /// quieting assertion's own, or the first two of its bounding
    /// faces, one of each copy, in some role order, that are faces a
    /// holding `Gap ≤ −δ` over which the rule would take (the faces
    /// opposed, their gap decided negative) and between whose carriers
    /// the overlap lies (neither face's crossing keeps any of it).
    /// `None` for an unlocalized overlap and for one with no such pair:
    /// a finding no assertion could quiet is outside what the document
    /// can yet say (CHECKS DS6), loud, and refused at no severity.
    pub quieting: Option<OpposedPair>,
}

impl InterferenceFinding {
    /// Whether no assertion quiets it.
    #[must_use]
    pub fn is_loud(&self) -> bool {
        self.quiet.is_none()
    }

    /// Whether [`crate::enforce_checks`] may refuse it: it is loud and
    /// an assertion could have quieted it ([`Self::quieting`]).
    #[must_use]
    pub fn refusable(&self) -> bool {
        self.is_loud() && self.quieting.is_some()
    }
}

/// **An assertion that can quiet an interference**: it holds, it reads
/// a `Gap`'s output directly over faces of two different copies, those
/// faces are opposed, and it admits only negative values. Whether it
/// quiets a given overlap is `quiet_verdicts`'s.
#[derive(Debug, Clone)]
pub struct GapAssertion {
    /// The assertion.
    pub assertion: RecipeNodeId,
    /// The member of its family it is, empty for an assertion reaching
    /// no index.
    pub at: crate::names::MemberAt,
    /// The `Gap`'s outer face: its copy's index in
    /// [`Product::copies`], and the face in the aggregate.
    outer: (usize, FaceKey),
    /// The `Gap`'s inner face, likewise.
    inner: (usize, FaceKey),
}

/// The funnel site name of the quieting rule's decision that a bound
/// is negative.
pub(crate) const AT_REST_QUIET_BOUND: &str = "at_rest_quiet_bound";

/// **Every assertion in `doc` that can quiet an interference between
/// two of `copies`** ([`GapAssertion`]), in document order.
///
/// It reads the assertion's verdict in `evaluation` and never its
/// measure's value, and evaluates nothing: an assertion that did not
/// evaluate, or does not hold, is simply not a candidate.
pub(crate) fn gap_assertions<P: crate::ProfilePayload, T: Decide + topo::AtRestPolicy>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    aggregate: &Body<T>,
    names: &NameTable,
    copies: &[GatheredCopy<T>],
    tol: Tol,
) -> Vec<GapAssertion> {
    let Ok(band) = Band::linear(tol) else {
        return Vec::new();
    };
    let face_of = |var: VarId, at: &crate::names::MemberAt| {
        reference(doc, var)
            .iter()
            .find_map(|r| face_read(doc, aggregate, names, copies, r, at))
    };
    let mut out = Vec::new();
    for gap in holding_measures(doc, evaluation) {
        if gap.verb != crate::measure::MeasureVerb::Gap
            || !matches!(
                gap.relation,
                AssertionRelation::AtMost | AssertionRelation::Equal
            )
            || decide(AT_REST_QUIET_BOUND, Margin::of(*gap.bound), band) != Ok(Sign::Negative)
        {
            continue;
        }
        let (Some(outer), Some(inner)) = (face_of(gap.a, &gap.at), face_of(gap.b, &gap.at)) else {
            continue;
        };
        if !pair_takes(aggregate, outer, inner, band) {
            continue;
        }
        out.push(GapAssertion {
            assertion: gap.id,
            at: gap.at,
            outer,
            inner,
        });
    }
    out
}

/// **The face of the product a `Gap`'s reference reads, as the member
/// at `at` of its assertion reads it** (D10 `[assertions]`,
/// "Families"): a selection of one name in a copy's read, named as that
/// member names it ([`member_name`]), looked up in the product's table
/// by its own name or by the name its `⊔` keyed it by, a unique face of
/// a copy of that read. The one reading [`gap_assertions`] matches a
/// holding assertion by and [`written_pair`] mints a pair by. `None` for
/// a pose, a variable read as it is, or a name the table does not hold
/// so.
fn face_read<P: crate::ProfilePayload, T: Decide + topo::AtRestPolicy>(
    doc: &Doc<P>,
    aggregate: &Body<T>,
    names: &NameTable,
    copies: &[GatheredCopy<T>],
    r: &QuietingRef,
    at: &crate::names::MemberAt,
) -> Option<CopyFace> {
    let Reference::Entity {
        read,
        name,
        pose: None,
    } = &r.0
    else {
        return None;
    };
    let name = &member_name(doc, *read, name, at);
    let entry = names.lookup(name).or_else(|| {
        names
            .iter()
            .find_map(|(held, entry)| (keyed_by(held, *read) == Some(name)).then_some(entry))
    });
    let Some(Entry::Unique(EntityRef {
        key: EntityKey::Face(face),
        ..
    })) = entry
    else {
        return None;
    };
    let solid = aggregate.solid_of_face(*face)?;
    let copy = copies
        .iter()
        .position(|c| c.var == *read && c.keys.solids().contains(&solid))?;
    Some((copy, *face))
}

/// **Whether the rule takes a `Gap` over two faces of the product**:
/// faces of two different copies, [`opposed`].
fn pair_takes<T: Decide>(
    aggregate: &Body<T>,
    outer: CopyFace,
    inner: CopyFace,
    band: Band,
) -> bool {
    outer.0 != inner.0 && opposed(aggregate, outer.1, inner.1, band)
}

/// **Whether a `Gap ≤ −δ` over two faces could hold**: the rule takes
/// the pair ([`pair_takes`]) and the gap between them reads negative.
fn pair_holds<T: Decide>(
    aggregate: &Body<T>,
    outer: CopyFace,
    inner: CopyFace,
    band: Band,
) -> bool {
    use crate::eval::measure::carrier_of;
    let carrier = |key| {
        carrier_of(
            crate::eval::Seat::own(aggregate),
            EntityRef {
                body: 0,
                key: EntityKey::Face(key),
            },
            band,
        )
    };
    pair_takes(aggregate, outer, inner, band)
        && gap_sign_as(
            &carrier(outer.1),
            &carrier(inner.1),
            AT_REST_QUIET_BOUND,
            band,
        ) == Some(Sign::Negative)
}

/// **How an author writes a `Gap` over two faces of a product**, where
/// the rule takes what they write: the two references (each face a
/// selection of its name in its copy's read, a family member's by the
/// name its member wraps, [`written_name`]), kept only where some member
/// of the assertion they make reads them back as those two faces
/// ([`face_read`]) and at every member the two faces it reads could hold
/// a negative gap ([`pair_holds`]). What a pair minted for an
/// interference ([`could_quiet`]) is written as.
pub type WritePair<'a> = dyn Fn(CopyFace, CopyFace) -> Option<[QuietingRef; 2]> + 'a;

/// [`WritePair`] over `product`, a gather of `doc`.
fn written_pair<P: crate::ProfilePayload, T: Decide + topo::AtRestPolicy>(
    doc: &Doc<P>,
    product: &Product<T>,
    outer: CopyFace,
    inner: CopyFace,
    band: Band,
) -> Option<[QuietingRef; 2]> {
    let (aggregate, names, copies) = (&*product.body, &product.names, &product.copies[..]);
    let written = |(copy, face): CopyFace| {
        let read = copies.get(copy)?.var;
        let name = names.name_of(&EntityRef {
            body: 0,
            key: EntityKey::Face(face),
        })?;
        let name = keyed_by(name, read).unwrap_or(name);
        Some(QuietingRef::entity(
            read,
            written_name(doc, read, name),
            None,
        ))
    };
    let references = [written(outer)?, written(inner)?];
    let members: Vec<_> = members_of::<P, f64>(doc, &references)?
        .into_iter()
        .map(|(at, _)| at)
        .collect();
    let read = |at| {
        Some((
            face_read(doc, aggregate, names, copies, &references[0], at)?,
            face_read(doc, aggregate, names, copies, &references[1], at)?,
        ))
    };
    let holds = members
        .iter()
        .all(|at| read(at).is_some_and(|(outer, inner)| pair_holds(aggregate, outer, inner, band)));
    (holds && members.iter().any(|at| read(at) == Some((outer, inner)))).then_some(references)
}

/// **Whether a `Gap`'s two faces are opposed**: each faces the other's
/// material, which is the mating configuration C5's gap is written for
/// (two plates face to face, a bore about a pin, a socket about a ball,
/// a ball or a pin pressed into a plate, two shafts side by side:
/// CONTACT-DESIGN C5's opposed pair, whose `g < 0` is the clash depth).
/// It is the facing of the pose the `Gap` over the two faces is read in
/// ([`crate::eval::measure::pose`]),
/// so the quieting rule and the measure decide the pair once, alike. A
/// pair the measure reads in no pose (two planes not parallel, a cone)
/// or whose facing does not decide is not opposed, and its finding stays
/// loud.
fn opposed<T: Decide>(body: &Body<T>, outer: FaceKey, inner: FaceKey, band: Band) -> bool {
    let carrier = |key| {
        crate::eval::measure::carrier_of(
            crate::eval::Seat::own(body),
            EntityRef {
                body: 0,
                key: EntityKey::Face(key),
            },
            band,
        )
    };
    use geom_brep::{CarrierPose, MaterialPairing};
    let facing = match crate::eval::measure::pose(&carrier(outer), &carrier(inner), band) {
        Some(CarrierPose::Planes(planes)) => planes.facing(band).ok(),
        Some(CarrierPose::Rounds(rounds)) => Some(rounds.facing()),
        Some(CarrierPose::PlaneRound(plane_round)) => Some(plane_round.facing()),
        None => None,
    };
    facing == Some(MaterialPairing::Opposed)
}

/// **The scalars the at-rest check localizes an overlap at**: one seam per
/// scalar, because localizing builds bodies (the copies' intersection
/// and the region past each asserted carrier), which reads the
/// bracket a scalar carries, and the check reads no bracket anywhere
/// else. `f64` is the one lane; the analysis scalars the check is also
/// run at report a decided overlap as [`Unlocalized::Lane`], loud.
pub trait LocalizeLane: Decide + AtRestPolicy + crate::mate::SolveScalar {
    /// The findings of each pair of copies of `product` the census
    /// decided interfere ([`partition`]), in `pairs`' order: by default
    /// one [`Unlocalized::Lane`] finding a pair, the scalar having no
    /// lane.
    /// `write` is how an author writes a quieting `Gap` over two faces
    /// of the product, where the rule takes it ([`WritePair`]).
    fn localize(
        product: &Product<Self>,
        pairs: Vec<(Pair, Vec<ValidationError>)>,
        _: Tol,
        _write: &WritePair<'_>,
    ) -> Vec<InterferenceFinding> {
        pairs
            .into_iter()
            .flat_map(|((a, b), evidence)| unlocalized_at_lane(product, a, b, evidence))
            .collect()
    }
}

/// Two copies of a product, by index, the smaller first.
pub type Pair = (usize, usize);

/// A gathered copy, as a finding names it.
fn copy_ref<T: Decide + topo::AtRestPolicy>(copy: &GatheredCopy<T>) -> CopyRef {
    CopyRef {
        var: copy.var,
        member: copy.output,
    }
}

/// The finding a scalar with no localizing lane reports for a pair:
/// the overlap is decided, and its site is not bounded.
fn unlocalized_at_lane<T: topo::AtRestPolicy + Decide>(
    product: &Product<T>,
    a: usize,
    b: usize,
    evidence: Vec<ValidationError>,
) -> Vec<InterferenceFinding> {
    vec![InterferenceFinding {
        a: copy_ref(&product.copies[a]),
        b: copy_ref(&product.copies[b]),
        evidence,
        overlap: Overlap::Unlocalized(Unlocalized::Lane),
        quiet: None,
        quiet_at: Vec::new(),
        quieting: None,
    }]
}

#[cfg(feature = "probe")]
impl LocalizeLane for geom_core::Probe {}

impl LocalizeLane for geom_core::Interval {}

impl<T> LocalizeLane for geom_core::Dual<T> where
    geom_core::Dual<T>: Decide + AtRestPolicy + crate::mate::SolveScalar
{
}

impl<T> LocalizeLane for geom_core::Sym<T> where
    geom_core::Sym<T>: Decide + AtRestPolicy + crate::mate::SolveScalar
{
}

impl LocalizeLane for f64 {
    fn localize(
        product: &Product<f64>,
        pairs: Vec<(Pair, Vec<ValidationError>)>,
        tol: Tol,
        write: &WritePair<'_>,
    ) -> Vec<InterferenceFinding> {
        let operands = Operands::of(product);
        pairs
            .into_iter()
            .flat_map(|((a, b), evidence)| localize(product, &operands, a, b, evidence, tol, write))
            .collect()
    }
}

/// **The at-rest census's verdicts, partitioned**: the interference
/// findings between copies, in the census's order of first mention of
/// each pair, and every other verdict, each a could-not-look finding
/// ([`AtRestEvidence`]).
pub(crate) fn partition<T: LocalizeLane>(
    product: &Product<T>,
    errors: Vec<ValidationError>,
    tol: Tol,
    write: &WritePair<'_>,
) -> (Vec<InterferenceFinding>, Vec<ValidationError>) {
    let copy_of: BTreeMap<SolidKey, usize> = product
        .copies
        .iter()
        .enumerate()
        .flat_map(|(i, c)| c.keys.solids().iter().map(move |&s| (s, i)))
        .collect();
    // The two copies a pair of solids belongs to, smaller first; `None`
    // for one copy, or a solid of no copy.
    let pair_of = |x: SolidKey, y: SolidKey| {
        let (x, y) = (*copy_of.get(&x)?, *copy_of.get(&y)?);
        (x != y).then_some((x.min(y), x.max(y)))
    };
    let mut pairs: Vec<(Pair, Vec<ValidationError>)> = Vec::new();
    let mut rest = Vec::new();
    for error in errors {
        let pair = topo::census_interference_solids(&product.body, &error)
            .and_then(|(x, y)| pair_of(x, y));
        match pair {
            Some(pair) => match pairs.iter_mut().find(|(p, _)| *p == pair) {
                Some((_, evidence)) => evidence.push(error),
                None => pairs.push((pair, vec![error])),
            },
            None => rest.push(error),
        }
    }
    // What the census left undecided between two copies it decided
    // interfere — curved faces within reach of each other, a face pair
    // read in band; never the containment arm's own refusal, which a
    // decided pair does not get — rides the pair's findings as evidence: often
    // the same overlap, but not always (a pin pressed into one bore and
    // slipped into another), so a quiet finding requires each such
    // verdict to lie on its own overlap (`covered`). So does a touch
    // between them, which the census records for no interfering pair
    // (`topo::record_at_rest`), and which keeps the finding loud. A pair
    // the census decided nothing about is a could-not-look finding.
    // A pair read in band hides no overlap thicker than the band
    // (`topo::Undecidable::ThinInBand`): it is no evidence of one, and
    // stands as its own could-not-look finding.
    let undecided_pair = |error: &ValidationError| match error {
        ValidationError::CensusUndecidable {
            reason: topo::Undecidable::ThinInBand,
            ..
        } => None,
        ValidationError::CensusUndecidable { .. } | ValidationError::UnrecordedContact { .. } => {
            topo::census_finding_solids(&product.body, error).and_then(|(x, y)| pair_of(x, y))
        }
        _ => None,
    };
    let mut kept = Vec::new();
    for error in rest {
        match undecided_pair(&error).and_then(|pair| pairs.iter_mut().find(|(p, _)| *p == pair)) {
            Some((_, evidence)) => evidence.push(error),
            None => kept.push(error),
        }
    }
    (T::localize(product, pairs, tol, write), kept)
}

/// **The one arrival a census finding is inside**: the group (in
/// [`copy_groups`]'s numbering) holding both solids it is about, `None`
/// for a finding between two arrivals or about no pair of solids.
pub(crate) fn finding_group<T: Decide>(
    body: &Body<T>,
    groups: &BTreeMap<SolidKey, u32>,
    error: &ValidationError,
) -> Option<u32> {
    let (a, b) = topo::census_finding_solids(body, error)?;
    let group = *groups.get(&a)?;
    (groups.get(&b) == Some(&group)).then_some(group)
}

/// **Which copy each of the product's solids is**: the census's groups
/// at rest ([`topo::record_at_rest`]), each copy its own.
pub(crate) fn copy_groups<T: Decide + topo::AtRestPolicy>(
    copies: &[GatheredCopy<T>],
) -> BTreeMap<SolidKey, u32> {
    copies
        .iter()
        .enumerate()
        .flat_map(|(i, c)| {
            let group = crate::names::to_u32(i, crate::product::GRAFTED_WITHIN_U32);
            c.keys.solids().iter().map(move |&s| (s, group))
        })
        .collect()
}

/// **A cell of the gathered product, named as its copy reads it**:
/// which copy holds it (by its index in `copies`) and the cell as that
/// copy's placement output names it, `None` for a cell of no copy.
pub(crate) fn copy_cell<T: Decide + topo::AtRestPolicy>(
    body: &Body<T>,
    names: &NameTable,
    copies: &[GatheredCopy<T>],
    groups: &BTreeMap<SolidKey, u32>,
    cell: topo::Cell,
) -> Option<(usize, crate::coincide::NamedCell)> {
    let key = match cell {
        topo::Cell::Face(f) => EntityKey::Face(f),
        topo::Cell::Edge(e) => EntityKey::Edge(e),
        topo::Cell::Vertex(v) => EntityKey::Vertex(v),
    };
    let copy = *groups.get(&topo::census_cell_solid(body, cell)?)? as usize;
    let name = names.name_of(&EntityRef { body: 0, key })?.clone();
    Some((
        copy,
        crate::coincide::NamedCell::Entity {
            input: copies[copy].var,
            name,
        },
    ))
}

/// **The census's rows between two copies, named** (D1 (iii), D10):
/// each row [`topo::record_at_rest`] recorded between two copies, its
/// cells named as each copy's placement reads them, and the two copies
/// it is about. The `unproven-coincidence` check reports each the door
/// does not prove.
pub(crate) fn copy_rows<T: Decide + topo::AtRestPolicy>(
    body: &Body<T>,
    names: &NameTable,
    copies: &[GatheredCopy<T>],
    rows: Vec<topo::Coincidence>,
) -> Vec<AtRestRow> {
    let groups = copy_groups(copies);
    let named = |cell: topo::RowCell| {
        let topo::RowCell::Result { cell } = cell else {
            return None;
        };
        copy_cell(body, names, copies, &groups, cell)
    };
    let by = |x: usize| copy_ref(&copies[x]);
    rows.into_iter()
        .map(|row| {
            let [Some((ca, na)), Some((cb, nb))] = row.cells.map(named) else {
                unreachable!("a row the census recorded between copies names a cell of each")
            };
            AtRestRow {
                copies: (by(ca.min(cb)), by(ca.max(cb))),
                row: crate::coincide::NamedCoincidence {
                    cells: vec![na, nb],
                    relation: row.relation,
                    site: row.site,
                    margin: row.margin,
                    discharge: row.discharge,
                    origin: crate::coincide::RowOrigin::Node,
                },
            }
        })
        .collect()
}

/// A copy as the intersection takes it: finished in its own frame, and
/// the motion into the product's frame, `None` where it is in it.
type Taken = (
    Arc<topo::AtRestBody<f64>>,
    Option<geom_core::RigidMotion<f64>>,
);

/// **Each copy of a product as the intersection takes it**, taken once
/// per localization however many interfering pairs it is in: its body
/// finished in its own frame (a finished copy as it is, a leaf finished
/// here, once), and its motion into the product's frame. Each pair hands
/// the two to the placed door, which moves them
/// ([`topo::boolean_placed`]).
struct Operands<'p> {
    product: &'p Product<f64>,
    read: Vec<std::cell::OnceCell<Result<Taken, String>>>,
}

impl<'p> Operands<'p> {
    fn of(product: &'p Product<f64>) -> Self {
        Self {
            product,
            read: product
                .copies
                .iter()
                .map(|_| std::cell::OnceCell::new())
                .collect(),
        }
    }

    /// Copy `i` finished in its own frame with its motion into the
    /// product's, or why it is not.
    fn get(&self, i: usize, tol: Tol) -> &Result<Taken, String> {
        self.read[i].get_or_init(|| {
            let copy = &self.product.copies[i].body;
            let motion = copy
                .moving_into(&self.product.frame)
                .map_err(|motion| crate::eval::NodeErrorKind::Motion(motion).to_string())?;
            let body = copy
                .holding()
                .finished(tol)
                .map_err(|refused| refused.to_string())?;
            Ok((body, motion))
        })
    }
}

/// `taken` placed in the product's frame.
fn placed((body, motion): &Taken) -> topo::Placed<'_, topo::AtRestBody<f64>> {
    topo::Placed::new(&**body, *motion)
}

/// **Two copies intersected**: the overlap of their material, `None`
/// when it is empty.
///
/// # Errors
///
/// Why it could not be formed.
fn intersection(
    operands: &Operands<'_>,
    a: usize,
    b: usize,
    tol: Tol,
) -> Result<Option<topo::BooleanBody<f64>>, Unlocalized> {
    let (body_a, body_b) = match (operands.get(a, tol), operands.get(b, tol)) {
        (Ok(x), Ok(y)) => (x, y),
        (Err(refusal), _) | (_, Err(refusal)) => {
            return Err(Unlocalized::Refused {
                refusal: refusal.clone(),
            });
        }
    };
    match topo::boolean_placed(
        topo::BooleanOp::Intersect,
        (placed(body_a), placed(body_b)),
        &topo::BooleanInputs {
            records_a: topo::FinishedBody::records(&*body_a.0).carried(),
            records_b: topo::FinishedBody::records(&*body_b.0).carried(),
        },
        topo::SweepStrategy::Realized,
        tol,
    ) {
        Ok((topo::BooleanResult::Body(result), _)) => Ok(Some(result)),
        Ok((topo::BooleanResult::Empty, _)) => Ok(None),
        // A copy is placed in the product's frame, where the gather
        // already grafted it and asked the box of its image
        // (`product::gather_sources`), so its move here asks the box of the
        // same placement: a product that built holds no copy this refuses.
        Err(topo::PlacedBooleanError::Placed { operand, findings }) => {
            let copy = match operand {
                topo::Operand::A => a,
                topo::Operand::B => b,
            };
            Err(Unlocalized::Refused {
                refusal: format!(
                    "copy {copy}'s image in the product's frame: {}",
                    crate::eval::NodeErrorKind::NotFinished { findings }
                ),
            })
        }
        Err(topo::PlacedBooleanError::Boolean(refusal)) => Err(Unlocalized::Refused {
            refusal: refusal.render(geom_brep::recourse::Reading::Build),
        }),
    }
}

/// One connected overlap: the faces bounding it, as aggregate faces of
/// the two copies and in the intersection's own arena.
struct Component {
    /// Its solid in the intersection's arena.
    solid: SolidKey,
    /// The aggregate faces it descends from, each with its copy's index.
    sources: BTreeSet<(usize, FaceKey)>,
}

/// **The findings of one interfering pair**: one per connected solid of
/// the two copies' intersection, each quieted by [`quiet_verdicts`].
fn localize(
    product: &Product<f64>,
    operands: &Operands<'_>,
    a: usize,
    b: usize,
    evidence: Vec<ValidationError>,
    tol: Tol,
    write: &WritePair<'_>,
) -> Vec<InterferenceFinding> {
    let copy_ref = |i: usize| copy_ref(&product.copies[i]);
    let finding = |overlap, quiet: Option<(RecipeNodeId, crate::names::MemberAt)>, quieting| {
        let (quiet, quiet_at) = quiet.map_or((None, Vec::new()), |(id, at)| (Some(id), at));
        InterferenceFinding {
            a: copy_ref(a),
            b: copy_ref(b),
            evidence: evidence.clone(),
            overlap,
            quiet,
            quiet_at,
            quieting,
        }
    };
    let unlocalized = |why| vec![finding(Overlap::Unlocalized(why), None, None)];
    let result = match intersection(operands, a, b, tol) {
        Ok(Some(result)) => result,
        Ok(None) => return unlocalized(Unlocalized::Empty),
        Err(why) => return unlocalized(why),
    };
    let Some(components) = components(product, a, b, &result) else {
        return unlocalized(Unlocalized::Unnamed);
    };
    let verdicts = quiet_verdicts(product, a, b, &components, &result, tol);
    // A quiet finding must not carry a verdict nothing checked: every
    // undecided verdict on the pair names faces of the component it
    // rides, which its containment check covered.
    let covered = |component: &Component| {
        evidence.iter().all(|error| match error {
            ValidationError::CensusUndecidable {
                a: topo::EntityId::Face(x),
                b: topo::EntityId::Face(y),
                ..
            } => [x, y]
                .into_iter()
                .all(|f| component.sources.iter().any(|(_, s)| s == f)),
            ValidationError::CensusUndecidable { .. } => false,
            // A touch between the two copies is no part of the overlap
            // an assertion bounds, and no assertion speaks for it here.
            ValidationError::UnrecordedContact { .. } => topo::census_contact_row(error).is_none(),
            _ => true,
        })
    };
    // The faces' crossings, each built once however many candidate pairs
    // a face is in (`could_quiet`).
    let mut crossings = BTreeMap::new();
    let mut out = Vec::with_capacity(components.len());
    // A face of a component, as its copy's own table names it.
    let site = |(copy, face): (usize, FaceKey)| {
        let name = product.names.name_of(&EntityRef {
            body: 0,
            key: EntityKey::Face(face),
        })?;
        let copy = copy_ref(copy);
        let face = keyed_by(name, copy.var).unwrap_or(name).clone();
        Some(FaceSite { copy, face })
    };
    for (i, (component, verdict)) in components.iter().zip(verdicts).enumerate() {
        let Some(mut faces) = component
            .sources
            .iter()
            .map(|&source| site(source))
            .collect::<Option<Vec<_>>>()
        else {
            return unlocalized(Unlocalized::Unnamed);
        };
        faces.sort();
        faces.dedup();
        let pair = match &verdict {
            Quiet::By(_, _, (outer, inner)) => {
                write(*outer, *inner).map(|written| (*outer, *inner, written))
            }
            Quiet::Loud => could_quiet(
                product,
                (a, b),
                &components,
                i,
                &result,
                tol,
                &mut crossings,
                write,
            ),
            Quiet::Refused(_) => None,
        };
        // Each face of the pair bounds the component, which names it.
        let quieting =
            pair.filter(|_| covered(component))
                .and_then(|(outer, inner, references)| {
                    Some(OpposedPair {
                        outer: site(outer)?,
                        inner: site(inner)?,
                        references,
                    })
                });
        out.push(match verdict {
            Quiet::Loud => finding(Overlap::Bounded { faces }, None, quieting),
            Quiet::By(assertion, at, _) if covered(component) => {
                finding(Overlap::Bounded { faces }, Some((assertion, at)), quieting)
            }
            Quiet::By(..) => finding(Overlap::Bounded { faces }, None, quieting),
            Quiet::Refused(refusal) => finding(
                Overlap::Unlocalized(Unlocalized::Containment { refusal }),
                None,
                None,
            ),
        });
    }
    out
}

/// A face of the product's aggregate, with its copy's index in
/// [`Product::copies`].
type CopyFace = (usize, FaceKey);

/// **The faces an assertion the quieting rule accepts could quiet the
/// component `i` over**, outer then inner, asked of the rule itself: the
/// first pair of the faces bounding it, one of copy `a` and one of copy
/// `b`, in either role order, that a holding `Gap ≤ −δ` over would be
/// one the rule takes ([`gap_assertions`]: the faces [`opposed`], the
/// gap they measure decided negative) and between whose carriers the
/// component lies as [`quiet_verdicts`] decides it ([`crossing`]). The
/// caller asks [`localize`]'s `covered` too, which no assertion changes.
/// Of those, the first an author can write so that the rule takes it
/// at every member of the assertion it makes (`write`, [`WritePair`]),
/// with what they write. `crossings` keeps each face's crossing, built
/// once.
#[allow(clippy::too_many_arguments)] // one component's reading, carried through
fn could_quiet(
    product: &Product<f64>,
    (a, b): (usize, usize),
    components: &[Component],
    i: usize,
    result: &topo::BooleanBody<f64>,
    tol: Tol,
    crossings: &mut BTreeMap<FaceKey, Result<BTreeSet<usize>, String>>,
    write: &WritePair<'_>,
) -> Option<(CopyFace, CopyFace, [QuietingRef; 2])> {
    let band = Band::linear(tol).ok()?;
    let body = &product.body;
    let of = |copy| {
        components[i]
            .sources
            .iter()
            .filter(move |(c, _)| *c == copy)
            .copied()
            .collect::<Vec<CopyFace>>()
    };
    let (from_a, from_b) = (of(a), of(b));
    let mut candidates = Vec::new();
    for &x in &from_a {
        for &y in &from_b {
            for (outer, inner) in [(x, y), (y, x)] {
                if pair_holds(body, outer, inner, band) {
                    candidates.push((outer, inner));
                }
            }
        }
    }
    candidates.into_iter().find_map(|(outer, inner)| {
        let between = [outer.1, inner.1].into_iter().all(|face| {
            let crossed = crossings
                .entry(face)
                .or_insert_with(|| crossing(body, face, components, result, tol));
            matches!(crossed, Ok(set) if !set.contains(&i))
        });
        between
            .then(|| write(outer, inner))
            .flatten()
            .map(|written| (outer, inner, written))
    })
}

/// The connected solids of `result`, each with the aggregate faces its
/// faces descend from; `None` when a face's lineage does not reach one
/// face of either copy (a face a re-chart turned reaches every face of
/// its shell).
fn components<T: topo::AtRestPolicy + Decide>(
    product: &Product<T>,
    a: usize,
    b: usize,
    result: &topo::BooleanBody<T>,
) -> Option<Vec<Component>> {
    let descent = topo::FaceDescent::of(&result.naming);
    let merged = descent.merged();
    let mut out = Vec::new();
    for (solid, _) in result.body.solids() {
        let faces = result.body.faces_of_solid(solid)?;
        let mut sources = BTreeSet::new();
        for &face in &faces {
            let alone = [face];
            let held = merged.get(&face).map_or(&alone[..], Vec::as_slice);
            for &piece in held {
                let source = descent.source(piece);
                let copy = match source.operand {
                    topo::Operand::A => a,
                    topo::Operand::B => b,
                };
                // A face a re-chart turned stands for every face of its
                // input shell, of which it may hold no area: as a source it
                // would let `covered` and the quieting rule read faces the
                // component does not hold, so the component is unnamed.
                // Not pinned: no at-rest fixture turns a shell.
                let [input] = source.inputs[..] else {
                    return None;
                };
                sources.insert((copy, product.copies[copy].keys.face(input)?));
            }
        }
        out.push(Component { solid, sources });
    }
    Some(out)
}

/// What the quieting rule says of one component.
enum Quiet {
    /// No assertion quiets it.
    Loud,
    /// This assertion, as its member at these values, over these two
    /// faces, outer then inner, quiets it.
    By(RecipeNodeId, crate::names::MemberAt, (CopyFace, CopyFace)),
    /// An assertion whose faces bound it could not be checked against
    /// it, and none quiets it: the kernel could not bound the overlap
    /// against the carriers.
    Refused(String),
}

/// **The quieting rule** (D10), per component of one pair's
/// intersection: the first assertion, in document order, among the
/// product's [`GapAssertion`]s over the pair whose two faces both bound
/// the component and between whose carriers the component lies.
///
/// "Between the carriers" is decided by the kernel's own boolean: for
/// each of the assertion's two faces, the region on the far side of
/// its carrier from its copy's material is built as a solid
/// ([`beyond`]) and intersected with the pair's components, and a
/// component lies between when neither intersection keeps any of it.
/// A component touching a carrier is between it; one that crosses it is
/// not (a sunk flange past the face it is asserted against; a pin
/// bottoming in a blind bore, against the bore, whose slug lies past
/// the bore's carrier, though not against the plate's top and the pin's
/// bottom, between which it all lies).
fn quiet_verdicts(
    product: &Product<f64>,
    a: usize,
    b: usize,
    components: &[Component],
    result: &topo::BooleanBody<f64>,
    tol: Tol,
) -> Vec<Quiet> {
    let mut verdicts: Vec<Quiet> = components.iter().map(|_| Quiet::Loud).collect();
    for g in &product.gap_assertions {
        if (g.outer.0.min(g.inner.0), g.outer.0.max(g.inner.0)) != (a, b) {
            continue;
        }
        let bounded: Vec<usize> = components
            .iter()
            .enumerate()
            .filter(|(_, c)| c.sources.contains(&g.outer) && c.sources.contains(&g.inner))
            .filter(|(i, _)| !matches!(verdicts[*i], Quiet::By(..)))
            .map(|(i, _)| i)
            .collect();
        if bounded.is_empty() {
            continue;
        }
        let outside = [g.outer.1, g.inner.1]
            .into_iter()
            .map(|face| crossing(&product.body, face, components, result, tol))
            .collect::<Result<Vec<_>, String>>()
            .map(|sets| sets.into_iter().flatten().collect::<BTreeSet<usize>>());
        for i in bounded {
            match &outside {
                Ok(crossed) if !crossed.contains(&i) => {
                    verdicts[i] = Quiet::By(g.assertion, g.at.clone(), (g.outer, g.inner));
                }
                Ok(_) => {}
                Err(refusal) => {
                    if matches!(verdicts[i], Quiet::Loud) {
                        verdicts[i] = Quiet::Refused(refusal.clone());
                    }
                }
            }
        }
    }
    verdicts
}

/// The components of `result` that reach past the carrier of the
/// aggregate face `face`, away from its copy's material.
///
/// # Errors
///
/// Why the kernel could not answer: a carrier or a component face with
/// no lane here, or the boolean's own refusal.
fn crossing(
    aggregate: &Body<f64>,
    face: FaceKey,
    components: &[Component],
    result: &topo::BooleanBody<f64>,
    tol: Tol,
) -> Result<BTreeSet<usize>, String> {
    let beyond = beyond(aggregate, face, &result.body, tol)?;
    let kept = match topo::intersect(&result.body, &beyond, tol) {
        Ok(topo::BooleanResult::Empty) => return Ok(BTreeSet::new()),
        Ok(topo::BooleanResult::Body(kept)) => kept,
        Err(refusal) => return Err(refusal.render(geom_brep::recourse::Reading::Build)),
    };
    let descent = topo::FaceDescent::of(&kept.naming);
    let of_solid: BTreeMap<SolidKey, usize> = components
        .iter()
        .enumerate()
        .map(|(i, c)| (c.solid, i))
        .collect();
    let mut out = BTreeSet::new();
    for (solid, _) in kept.body.solids() {
        let mut named = false;
        for f in kept.body.faces_of_solid(solid).unwrap_or_default() {
            let source = descent.source(f);
            if source.operand == topo::Operand::A
                && let Some(&i) = source.inputs.iter().find_map(|&input| {
                    result
                        .body
                        .solid_of_face(input)
                        .and_then(|s| of_solid.get(&s))
                })
            {
                out.insert(i);
                named = true;
            }
        }
        // A kept piece bounded by the beyond solid's faces alone lies
        // inside some component and names none: every one is crossed.
        if !named {
            out.extend(0..components.len());
        }
    }
    Ok(out)
}

/// The funnel site name of the frame the beyond region is built on.
pub(crate) const AT_REST_BEYOND_FRAME: &str = "at_rest_beyond_frame";

/// **The region past the carrier of aggregate face `face`**, away from
/// its copy's material, as a solid that holds every point of `result`
/// in that region: a box on the far side of a plane, the solid
/// cylinder inside a bore, the tube outside a pin. Its sizes come from
/// the reach of `result`'s faces.
///
/// # Errors
///
/// A carrier with no arm here (a sphere, a cone), a face of `result`
/// whose reach this cannot bound, or a construction refusal.
fn beyond(
    aggregate: &Body<f64>,
    face: FaceKey,
    result: &Body<f64>,
    tol: Tol,
) -> Result<topo::AtRestBody<f64>, String> {
    use crate::eval::measure::{Carrier, carrier_of, reach_of};
    use geom_core::{Point2, RigidMotion};
    use profile::{Step, Target};
    let band = Band::linear(tol).map_err(|e| e.to_string())?;
    let sense = aggregate
        .get_face(face)
        .map(|f| f.sense)
        .ok_or("an unreadable face")?;
    let carrier = carrier_of(
        crate::eval::Seat::own(aggregate),
        EntityRef {
            body: 0,
            key: EntityKey::Face(face),
        },
        band,
    );
    let origin = match &carrier {
        Carrier::Plane { origin, .. } => *origin,
        Carrier::Cylinder { axis, .. } => axis.origin,
        _ => return Err("the containment check has no lane for this carrier".to_owned()),
    };
    // The region is sized to hold every face of the overlap: each face's
    // cover's farthest from the carrier's point.
    let mut reach = 0.0_f64;
    for (f, _) in result.faces() {
        let r = reach_of(result, f, origin, band).ok_or("a face of the overlap with no cover")?;
        reach = reach.max(r);
    }
    let pad = reach.mul_add(0.5, 1e3 * tol.eps());
    let span = reach + pad;
    let rectangle = |h: f64| {
        vec![
            Step::At(Point2::new(-h, -h)),
            Step::LineTo(Target::Point(Point2::new(h, -h))),
            Step::LineTo(Target::Point(Point2::new(h, h))),
            Step::LineTo(Target::Point(Point2::new(-h, h))),
            Step::LineTo(Target::Start),
        ]
    };
    let circle = |radius: f64| {
        vec![Step::Circle {
            centre: Point2::new(0.0, 0.0),
            radius,
        }]
    };
    // The frame's base point and normal, the loops in its plane, and the
    // depth the region is swept along the normal.
    let (base, normal, loops, depth) = match carrier {
        Carrier::Plane {
            origin, outward, ..
        } => (origin, outward.vec(), vec![rectangle(span)], span),
        Carrier::Cylinder { axis, radius, .. } => {
            let base = axis.origin - axis.dir.get() * span;
            // A concave face (a bore) has its material outside the
            // carrier, so the region past it is the solid cylinder; a
            // convex one (a pin) the tube around it.
            let loops = if sense {
                vec![circle(radius + 2.0 * span), circle(radius)]
            } else {
                vec![circle(radius)]
            };
            (base, axis.dir.get(), loops, 2.0 * span)
        }
        _ => unreachable!("the carrier was matched above"),
    };
    // The region is turned with the carrier's own chart, so it moves
    // with the inputs and not with the computing axes.
    let reference = match aggregate.face_surface(face) {
        Some(topo::Surface::Plane { frame, .. } | topo::Surface::Cylinder { frame, .. }) => {
            frame.u_ref()
        }
        _ => return Err("the carrier's surface is unreadable".to_owned()),
    };
    let frame =
        RigidMotion::from_axis_and_reference(base, normal, reference, AT_REST_BEYOND_FRAME, band)
            .map_err(|_| "the carrier's chart spans no frame")?;
    let loops = loops
        .iter()
        .map(|steps| profile::replay(steps, tol).map_err(|e| format!("{e:?}")))
        .collect::<Result<Vec<_>, String>>()?;
    let plane = profile::SketchPlane::from_frame(frame);
    let validated = profile::ConstructedProfile::new(plane, loops)
        .validate(tol)
        .map_err(|e| e.to_string())?;
    let swept = sweep::extrude(
        &validated,
        sweep::Extrusion::Distance {
            depth,
            side: sweep::ExtrudeSide::Along,
        },
        tol,
    )
    .map_err(|e| e.render(geom_brep::recourse::Reading::Build))?;
    Ok(swept.body)
}
