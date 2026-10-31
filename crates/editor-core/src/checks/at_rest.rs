//! **The at-rest check** (D10, Assertions; ASSEMBLY A5): what the
//! census at each `⊔` found between its arrivals, as the registry's
//! `AtRest` resident reports it, and the quieting rule's one home.
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

use std::collections::{BTreeMap, BTreeSet};

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

mod carried;

pub use carried::{
    AtRestRow, CarriedAtRestFinding, CarriedCoincidence, CarriedCoincidenceRows, CarriedFindings,
    CarriedInterference, PinnedDoc, Route,
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
    /// The evaluation's scalar runs no census at rest
    /// ([`topo::AtRestOutcome::NotRunAtThisScalar`]): nothing was
    /// looked at, which is one finding rather than a clean report.
    NotRun,
    /// A part's could-not-look finding, crossed with its route.
    Carried(Box<CarriedAtRestFinding>),
}

// The kernel's own verdict, forwarded verbatim through its `Display`:
// its tier-3′ messages end in their own recourse.
impl crate::spoken::Say for AtRestEvidence {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::NoLane(error) | Self::Malformed(error) | Self::InsideArrival(error) => {
                write!(f, "{error}")
            }
            Self::NotGathered(refusal) => write!(
                f,
                "its operands do not gather: {}",
                crate::spoken::Said(refusal, by)
            ),
            Self::NotRun => f.write_str(
                "this evaluation's scalar runs no census at rest, so nothing was looked at",
            ),
            Self::Carried(row) => write!(f, "{}", crate::spoken::Said(&**row, by)),
        }
    }
}

/// The evidence where no document is at hand.
impl core::fmt::Display for AtRestEvidence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
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

/// [`WholeReading`] over `product` and the gathers beside it.
pub(crate) fn whole_reading<T: LocalizeLane>(product: &Product<T>, tol: Tol) -> WholeReading {
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
        } = reading(product, tol);
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
/// which is a could-not-look finding. A scalar that runs no census
/// reads as one [`AtRestEvidence::NotRun`]. A contact between arrivals
/// is not among the verdicts: the gather recorded it as a row.
pub(crate) fn reading<T: LocalizeLane>(product: &Product<T>, tol: Tol) -> Reading {
    if product.body.outcome() == topo::AtRestOutcome::NotRunAtThisScalar {
        return Reading {
            interference: Vec::new(),
            could_not_look: vec![AtRestEvidence::NotRun],
        };
    }
    let errors = product.census.clone();
    if errors.is_empty() {
        return Reading {
            interference: Vec::new(),
            could_not_look: Vec::new(),
        };
    }
    let (interference, rest) = partition(product, errors, tol);
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
    if finding_solids(body, &error).is_some()
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
        let read = said.read(doc, evaluation, tol)?;
        let kinds = [pose_kind(&read.cells[0])?, pose_kind(&read.cells[1])?];
        if said.coaxial && kinds == [crate::VarKind::Axis; 2] && !axes_parallel(&read, band)? {
            return None;
        }
        kinds.map(Some)
    } else {
        [None, None]
    };
    holding_measures(doc, evaluation).find_map(|held| {
        if !said.verbs.contains(&held.verb)
            || held.relation != AssertionRelation::Equal
            || decide(AT_REST_QUIET_CONTACT_BOUND, Margin::of(*held.bound), band) != Ok(Sign::Zero)
        {
            return None;
        }
        let (Some(x), Some(y)) = (said.reference(doc, held.a), said.reference(doc, held.b)) else {
            return None;
        };
        let reads = |r: &(RowCell, Option<crate::VarKind>), i: usize| {
            r.0.reads(doc, &said.cells[i]) && r.1 == kinds[i]
        };
        ((reads(&x, 0) && reads(&y, 1)) || (reads(&x, 1) && reads(&y, 0))).then_some(held.id)
    })
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
/// one computing frame (`Framed::computing_among`), the other read into
/// it through `relative`.
struct ReadCells<'e, T: Decide> {
    cells: Vec<(std::sync::Arc<Body<T>>, EntityRef)>,
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
        tol: Tol,
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
            .map(|(f, ent)| Some((computing.read_in(f, tol).ok()?, *ent)))
            .collect::<Option<Vec<_>>>()?;
        Some(ReadCells { cells, computing })
    }

    /// **An assertion's reference as the rule matches it**: the row's
    /// tool where it is the tool's read, whatever pose that is; else a
    /// selection as its cell, a point, an axis or a plane read directly
    /// off a selection as that cell with its kind
    /// (`eval::read_directly_off`), and any other read as
    /// a tool it is not.
    fn reference<P>(&self, doc: &Doc<P>, var: VarId) -> Option<(RowCell, Option<crate::VarKind>)> {
        if self.cells.contains(&RowCell::Tool(var)) {
            return Some((RowCell::Tool(var), None));
        }
        let selected = |var: VarId| {
            let select = doc.selection(var)?;
            let [name] = select.names.as_slice() else {
                return None;
            };
            Some(RowCell::Entity(select.body, name.clone()))
        };
        if let Some(cell) = selected(var) {
            return Some((cell, None));
        }
        match crate::eval::read_directly_off(doc, var) {
            Some((cell, kind)) => Some((selected(cell)?, Some(kind))),
            None => Some((RowCell::Tool(var), None)),
        }
    }
}

/// **The pose a cell's carrier has**, as a coincidence of poses reads
/// it: an axis off a cylinder, a cone or a torus, a centre off a sphere;
/// `None` for any other cell.
fn pose_kind<T: Decide>(
    (body, ent): &(std::sync::Arc<Body<T>>, EntityRef),
) -> Option<crate::VarKind> {
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
fn axes_parallel<T: crate::mate::SolveScalar>(read: &ReadCells<'_, T>, band: Band) -> Option<bool> {
    let axis = |i: usize| {
        let (body, ent) = &read.cells[i];
        crate::eval::read_off_cell(body, *ent, crate::VarKind::Axis, band).ok()
    };
    let (a, b) = (axis(0)?, axis(1)?);
    crate::eval::measure::axes_parallel(&a, &b, band)
}

/// **Whether an assertion could quiet `row`** under the row half
/// ([`quiet_row`]), in `evaluation`, asked of the rule itself: some
/// measure says what the row decided ([`saying`]), and that measure,
/// over the row's two cells resolved and read as the measure resolves
/// and reads them ([`crate::eval::resolve_name`]), each read as the
/// pose its carrier has where the row is one of poses, a split's tool
/// read as its plane, is decided zero in some order at the site the
/// rule decides its bound at. A cell the measure has no form for, a
/// name that does not resolve, and a measure read non-zero are none of
/// them: no assertion over the row holds, so the finding never refuses
/// (CHECKS DS6), and its recourse names the construction alone.
pub(crate) fn row_quietable<P, T>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    row: &crate::coincide::NamedCoincidence,
    tol: Tol,
) -> bool
where
    P: crate::ProfilePayload,
    T: crate::mate::SolveScalar + topo::AtRestPolicy,
{
    let quietable = || -> Option<bool> {
        let band = Band::linear(tol).ok()?;
        let said = saying(row)?;
        let read = said.read(doc, evaluation, tol)?;
        let zero = |a: &crate::eval::measure::Carrier<'_, T>,
                    b: &crate::eval::measure::Carrier<'_, T>| {
            said.verbs.iter().any(|&verb| {
                [(a, b), (b, a)].into_iter().any(|(x, y)| {
                    measure_sign(verb, x, y, AT_REST_QUIET_CONTACT_BOUND, band) == Some(Sign::Zero)
                })
            })
        };
        let tool = said.cells.iter().find_map(|c| match c {
            RowCell::Tool(var) => Some(*var),
            RowCell::Entity(..) => None,
        });
        if let Some(tool) = tool {
            // A split's ON row: its tool read as its plane, in its
            // space's coordinates, against the vertex.
            let [(body, ent)] = read.cells.as_slice() else {
                return None;
            };
            let crate::eval::measure::Carrier::Point { at, .. } =
                crate::eval::measure::carrier_of(body, *ent)
            else {
                return None;
            };
            let roled = crate::pose::RoleRead {
                read: tool,
                role: said.tool_role,
            };
            let plane = crate::eval::read_pose(
                P::evaluable(doc)?,
                &evaluation.nodes,
                roled,
                &evaluation.env,
                tol,
            )
            .ok()?;
            let plane = crate::eval::measure::Carrier::Pose {
                pose: crate::eval::measure::Pose::of(&plane)?,
                reach: None,
            };
            let vertex = crate::eval::measure::Carrier::Pose {
                pose: crate::eval::measure::Pose::Point(read.computing.point_in_space(at)),
                reach: None,
            };
            return Some(zero(&plane, &vertex));
        }
        let carrier = |i: usize| {
            let (body, ent) = &read.cells[i];
            if !said.poses {
                return Some(crate::eval::measure::carrier_of(body, *ent));
            }
            let kind = pose_kind(&read.cells[i])?;
            crate::eval::read_off_cell(body, *ent, kind, band).ok()
        };
        let (a, b) = (carrier(0)?, carrier(1)?);
        let axes = said.poses
            && [0, 1].map(|i| pose_kind(&read.cells[i])) == [Some(crate::VarKind::Axis); 2];
        if said.coaxial && axes && !axes_parallel(&read, band)? {
            return Some(false);
        }
        Some(zero(&a, &b))
    };
    quietable() == Some(true)
}

/// **The sign a `Gap` assertion's measure reads over two carriers**,
/// decided at `site`: the value the measure reports
/// ([`crate::eval::measure::gap_value`]), `None` where it refuses or
/// the decision does. Both halves of the quieting rule ask it of a pair
/// an assertion could be written over.
fn gap_sign<T: Decide>(
    outer: &crate::eval::measure::Carrier<'_, T>,
    inner: &crate::eval::measure::Carrier<'_, T>,
    site: &'static str,
    band: Band,
) -> Option<Sign> {
    measure_sign(crate::measure::MeasureVerb::Gap, outer, inner, site, band)
}

/// [`gap_sign`] for a `Gap` or a `Distance`
/// ([`crate::eval::measure::value_of`]).
fn measure_sign<T: Decide>(
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
        | (Site::TangentWitness, _) => (&[M::Gap], false, false),
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
fn holding_measures<'a, P, T: Decide>(
    doc: &'a Doc<P>,
    evaluation: &'a Evaluation<T>,
) -> impl Iterator<Item = HoldingMeasure<'a, T>> + 'a {
    use crate::measure::MeasureVerb as M;
    doc.ids()
        .into_iter()
        .flat_map(move |id| {
            let Some(Node::Assertion {
                value, relation, ..
            }) = doc.node(id)
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
    /// A copy did not validate as a body of its own, so it could not
    /// be intersected.
    Invalid {
        /// That copy's findings.
        errors: Vec<ValidationError>,
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
    /// Whether an assertion the quieting rule accepts could quiet it,
    /// asked of the rule itself ([`localize`]): the overlap is bounded;
    /// every verdict it carries lies on it (`covered`, which no
    /// assertion changes); and either an assertion quiets it, or two of
    /// its bounding faces, one of each copy, in some role order, are
    /// faces a holding `Gap ≤ −δ` over which the rule would take (the
    /// faces opposed, their gap decided negative) and between whose
    /// carriers the overlap lies (neither face's crossing keeps any of
    /// it). `false` for an unlocalized overlap. A finding no assertion
    /// could quiet is outside what the document can yet say (CHECKS
    /// DS6): loud, and refused at no severity.
    pub quietable: bool,
}

impl InterferenceFinding {
    /// Whether no assertion quiets it.
    #[must_use]
    pub fn is_loud(&self) -> bool {
        self.quiet.is_none()
    }

    /// Whether [`crate::enforce_checks`] may refuse it: it is loud and
    /// an assertion could have quieted it ([`Self::quietable`]).
    #[must_use]
    pub fn refusable(&self) -> bool {
        self.is_loud() && self.quietable
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

/// The funnel site name of the quieting rule's decision that two faces'
/// materials face each other.
pub(crate) const AT_REST_QUIET_OPPOSED: &str = "at_rest_quiet_opposed";

/// The funnel site name of the quieting rule's decision that two
/// cylinders' axes are parallel.
pub(crate) const AT_REST_QUIET_AXES_PARALLEL: &str = "at_rest_quiet_axes_parallel";

/// The funnel site name of the quieting rule's decision that two
/// cylinders share an axis, or two spheres a centre.
pub(crate) const AT_REST_QUIET_CENTRED: &str = "at_rest_quiet_centred";

/// **Every assertion in `doc` that can quiet an interference between
/// two of `copies`** ([`GapAssertion`]), in document order.
///
/// It reads the assertion's verdict in `evaluation` and never its
/// measure's value, and evaluates nothing: an assertion that did not
/// evaluate, or does not hold, is simply not a candidate.
pub(crate) fn gap_assertions<P: crate::ProfilePayload, T: Decide>(
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
    let face_of = |var: VarId, at: &crate::names::MemberAt| -> Option<(usize, FaceKey)> {
        let select = doc.selection(var)?;
        let [name] = select.names.as_slice() else {
            return None;
        };
        let name = &member_name(doc, select.body, name, at);
        // The selection reads the copy, by the copy's own name; the
        // product's `⊔` keys that name by the copy it read.
        let entry = names.lookup(name).or_else(|| {
            names.iter().find_map(|(held, entry)| {
                (keyed_by(held, select.body) == Some(name)).then_some(entry)
            })
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
            .position(|c| c.var == select.body && c.keys.solids().contains(&solid))?;
        Some((copy, *face))
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
        if outer.0 == inner.0 || !opposed(aggregate, outer.1, inner.1, band) {
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

/// **Whether a `Gap`'s two faces are opposed**: each faces the other's
/// material, which is the mating configuration C5's gap is written for
/// (two planes whose outward normals oppose, a bore about a pin, a
/// socket about a ball). The material pairing door decides it, read at
/// a point of the outer face and levered by the two faces' reach from
/// it, as the census reads two faces' sides; a pairing it cannot decide
/// is not opposed.
///
/// One point answers for the whole pair only where the pairing is the
/// same at every point of the two faces ([`constant_pairing`]): two
/// planes, two cylinders on one axis, two spheres about one centre.
/// Any other pair is not opposed, so its finding stays loud: read at
/// one point, two parallel shafts side by side would pair one way at
/// one bearing and the other way at the next.
fn opposed<T: Decide>(body: &Body<T>, outer: FaceKey, inner: FaceKey, band: Band) -> bool {
    let Some(read) = pairing_read(body, outer, inner, band) else {
        return false;
    };
    let arm = geom_brep::folded_lever_arm(read.outer.0, read.inner.0, read.at, read.reach);
    geom_brep::classify_material_pairing_as(
        AT_REST_QUIET_OPPOSED,
        read.outer.0,
        read.outer.1,
        read.inner.0,
        read.inner.1,
        read.at,
        arm,
        band,
    ) == Ok(geom_brep::MaterialPairing::Opposed)
}

/// **Where one reading answers for two faces' pairing**
/// ([`pairing_read`]): each face's carrier and sense, the point of the
/// outer face it is read at, and the farther of the two faces' reach
/// from it.
struct PairingRead<'b, T: Decide> {
    outer: (&'b geom::Surface<T>, bool),
    inner: (&'b geom::Surface<T>, bool),
    at: geom_core::Point3<T>,
    reach: T,
}

/// **The pairing of two faces, where one reading answers for it**: the
/// reading at a point of the outer face, or `None` where the faces do
/// not pair their materials alike at every point ([`constant_pairing`])
/// or do not read. [`opposed`] decides the pairing on it, which both the
/// quieting rule and [`could_quiet`] ask.
fn pairing_read<T: Decide>(
    body: &Body<T>,
    outer: FaceKey,
    inner: FaceKey,
    band: Band,
) -> Option<PairingRead<'_, T>> {
    use crate::eval::measure::reach_of;
    let carrier = |key: FaceKey| {
        let face = body.get_face(key)?;
        Some((body.get_surface(face.surface)?, face.sense))
    };
    let (outer_carrier, inner_carrier) = (carrier(outer)?, carrier(inner)?);
    let at = face_point(body, outer)?;
    let (r_outer, r_inner) = (reach_of(body, outer, at)?, reach_of(body, inner, at)?);
    constant_pairing(outer_carrier.0, inner_carrier.0, r_outer + r_inner, band).then(|| {
        PairingRead {
            outer: outer_carrier,
            inner: inner_carrier,
            at,
            reach: r_outer.max(r_inner),
        }
    })
}

/// **Whether two carriers pair their materials alike at every point**:
/// two planes; two cylinders whose axes are parallel, levered by
/// `extent`, and one line; two spheres about one centre. Every other
/// pair, and a pair whose decision lands in band, is not.
fn constant_pairing<T: Decide>(
    outer: &geom::Surface<T>,
    inner: &geom::Surface<T>,
    extent: T,
    band: Band,
) -> bool {
    use geom::Surface as S;
    let zero = |name, margin| decide(name, margin, band) == Ok(Sign::Zero);
    match (outer, inner) {
        (S::Plane { .. }, S::Plane { .. }) => true,
        (
            S::Cylinder {
                origin: o1,
                axis: a1,
                ..
            },
            S::Cylinder {
                origin: o2,
                axis: a2,
                ..
            },
        ) => {
            let rel = *o2 - *o1;
            zero(
                AT_REST_QUIET_AXES_PARALLEL,
                Margin::levered(a1.cross(*a2).norm(), extent),
            ) && zero(
                AT_REST_QUIET_CENTRED,
                Margin::of((rel - *a1 * rel.dot(*a1)).norm()),
            )
        }
        (S::Sphere { center: c1, .. }, S::Sphere { center: c2, .. }) => {
            zero(AT_REST_QUIET_CENTRED, Margin::of((*c2 - *c1).norm()))
        }
        _ => false,
    }
}

/// A point of face `key`: the start of its outer loop.
fn face_point<T: Decide>(body: &Body<T>, key: FaceKey) -> Option<geom_core::Point3<T>> {
    let face = body.get_face(key)?;
    let vertex = match body.get_loop(face.outer)?.boundary {
        topo::LoopBoundary::Empty { vertex } => vertex,
        topo::LoopBoundary::Cycle { first, .. } => body.get_half_edge(first)?.start,
    };
    body.get_point(body.get_vertex(vertex)?.point).copied()
}

/// **The scalars the at-rest check localizes an overlap at**: one seam per
/// scalar, because localizing builds bodies (the copies' intersection
/// and the region past each asserted carrier), which reads the
/// bracket a scalar carries, and the check reads no bracket anywhere
/// else. `f64` is the one lane; the analysis scalars the check is also
/// run at report a decided overlap as [`Unlocalized::Lane`], loud.
pub trait LocalizeLane: Decide + AtRestPolicy + crate::mate::SolveScalar {
    /// The findings of the copies `a` and `b` of `product`, which the
    /// census decided interfere ([`partition`]): by default one
    /// [`Unlocalized::Lane`] finding, the scalar having no lane.
    fn localize(
        product: &Product<Self>,
        a: usize,
        b: usize,
        evidence: Vec<ValidationError>,
        _: Tol,
    ) -> Vec<InterferenceFinding> {
        unlocalized_at_lane(product, a, b, evidence)
    }
}

/// A gathered copy, as a finding names it.
fn copy_ref<T: Decide>(copy: &GatheredCopy<T>) -> CopyRef {
    CopyRef {
        var: copy.var,
        member: copy.output,
    }
}

/// The finding a scalar with no localizing lane reports for a pair:
/// the overlap is decided, and its site is not bounded.
fn unlocalized_at_lane<T: Decide>(
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
        quietable: false,
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
        a: usize,
        b: usize,
        evidence: Vec<ValidationError>,
        tol: Tol,
    ) -> Vec<InterferenceFinding> {
        localize(product, a, b, evidence, tol)
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
    let mut pairs: Vec<((usize, usize), Vec<ValidationError>)> = Vec::new();
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
            finding_solids(&product.body, error).and_then(|(x, y)| pair_of(x, y))
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
    let findings = pairs
        .into_iter()
        .flat_map(|((a, b), evidence)| T::localize(product, a, b, evidence, tol))
        .collect();
    (findings, kept)
}

/// **The two solids a census finding is about**, smaller key first:
/// the pair an interference verdict decided, the two entities an
/// undecidable verdict names, the two cells of an unrecorded touch, the
/// two edges of a coincident-wedges verdict, the edge and face of a
/// face-through-wedge verdict, the one solid (twice) of a solid
/// crossing itself, the two faces of a contradicted
/// declaration or of a pair the census had no lane for.
///
/// `None` for a finding that names no pair of solids: an escalation,
/// which names no entity at all
/// (`work/contacthold/an-escalated-census-finding-names-no-pair.md`), a
/// stale record, a lane subject that is one entity, a touch with a cell
/// of an input or a tool, and every finding about one body's own
/// topology. The match is exhaustive, so the in-arrival discard
/// ([`finding_group`]) never keeps a finding it could not place by
/// accident: a finding `topo` adds fails to compile here until it is
/// placed.
pub(crate) fn finding_solids<T: Decide>(
    body: &Body<T>,
    error: &ValidationError,
) -> Option<(SolidKey, SolidKey)> {
    use ValidationError as E;
    let edge = |e| topo::census_cell_solid(body, topo::Cell::Edge(e));
    let face = |f| body.solid_of_face(f);
    let (x, y) = match error {
        E::InstanceInterference { outer, inner, .. } => (*outer, *inner),
        E::SameSideInterference { a, b } => (face(*a)?, face(*b)?),
        E::TouchInterference { a, b, .. } => (
            topo::census_cell_solid(body, *a)?,
            topo::census_cell_solid(body, *b)?,
        ),
        E::CensusUndecidable { a, b, .. } => (
            topo::census_entity_solid(body, *a)?,
            topo::census_entity_solid(body, *b)?,
        ),
        // A pierce, or a crossing whose side test read `SameSide`, by
        // its own cells: two solids', or one solid's crossing itself.
        E::UnrecordedContact {
            contact: topo::CensusContact::EdgeFacePierce { edge: e, face: f },
            ..
        } => (edge(*e)?, face(*f)?),
        E::UnrecordedContact {
            contact:
                topo::CensusContact::EdgeEdgeCross {
                    a,
                    b,
                    side: Some(topo::CrossingSideVerdict::SameSide),
                },
            ..
        } => (edge(*a)?, edge(*b)?),
        touch @ E::UnrecordedContact { .. } => {
            let [x, y] = topo::census_contact_row(touch)?
                .cells
                .map(|cell| match cell {
                    topo::RowCell::Result { cell } => topo::census_cell_solid(body, cell),
                    topo::RowCell::Input { .. } | topo::RowCell::Tool => None,
                });
            (x?, y?)
        }
        E::CoincidentWedgesOverlap { edges: [a, b], .. }
        | E::CoincidentWedgesOverlapUndecided { edges: [a, b], .. } => (edge(*a)?, edge(*b)?),
        E::FaceThroughEdgeWedge {
            edge: e, face: f, ..
        }
        | E::FaceThroughEdgeWedgeUndecided {
            edge: e, face: f, ..
        } => (edge(*e)?, face(*f)?),
        // A solid crossing itself is about that one solid, placed as
        // (s, s) as census `Named::of` reads it: inside an arrival a
        // door already censused, it is that door's.
        E::BoundaryCrossesItself { solid, .. }
        | E::BoundaryCrossesItselfUndecided { solid, .. } => (*solid, *solid),
        E::ContactContradicted { declaration, .. } => (face(declaration.a)?, face(declaration.b)?),
        E::CensusUnsupported { subject, .. } | E::CensusLaneUnsupported { subject } => {
            match subject {
                topo::CensusSubject::FacePair(a, b) => (face(*a)?, face(*b)?),
                topo::CensusSubject::Entity(_) => return None,
            }
        }
        // Census findings that name no two solids: a stale record is
        // about the record a door carried, not a meeting; an escalation
        // names no entity at all
        // (`work/contacthold/an-escalated-census-finding-names-no-pair.md`);
        // an arc end off its corner names one face.
        E::StaleContactDeclaration { .. }
        | E::CensusEscalated { .. }
        | E::ArcEndOffCorner { .. } => {
            return None;
        }
        // Every other finding is about one body's own topology or
        // geometry, never a pair of solids. Listed rather than a
        // wildcard, so a finding topo adds is placed here by hand.
        E::Band { .. }
        | E::DanglingDescription { .. }
        | E::UncertifiableSurface { .. }
        | E::PoisonedSurfaceDescription { .. }
        | E::ApproxCertification { .. }
        | E::ApproxLaneUnsupported { .. }
        | E::DegenerateTorus { .. }
        | E::DegenerateTorusEscalated { .. }
        | E::PoisonedSurfaceDatum { .. }
        | E::UnrepresentableSurfaceDatum { .. }
        | E::PoisonedCurveDatum { .. }
        | E::UnrepresentableCurveDatum { .. }
        | E::EdgeCertification { .. }
        | E::DescriptionNotAdjacent { .. }
        | E::PlanarFaceResidual { .. }
        | E::PlanarFaceEscalated { .. }
        | E::PlanarBoundaryResidual { .. }
        | E::PlanarBoundaryEscalated { .. }
        | E::SliverDihedral { .. }
        | E::TransverseNotIntrinsic { .. }
        | E::MixedDihedral { .. }
        | E::ScaffoldAtRest { .. }
        | E::JoinableVertexAtRest { .. }
        | E::JoinUndecidedAtRest { .. }
        | E::TangentNotIntrinsic { .. }
        | E::LaminaWedge { .. }
        | E::NoDihedralArm { .. }
        | E::LoopRoleInverted { .. }
        | E::CurvedSenseInverted { .. }
        | E::SeamRegionOvercovered { .. }
        | E::SeamRegionCoverUnresolved { .. }
        | E::ChartSingularityNotAVertex { .. }
        | E::ChartSingularityUndecided { .. }
        | E::WrapTipOffChartSingularity { .. }
        | E::WrapTipUndecided { .. }
        | E::WrapTipWithoutChartSingularity { .. }
        | E::NegativeVolume { .. }
        | E::SliverInsideOut { .. }
        | E::VolumeUncomputable { .. }
        | E::VolumeSignUnresolved { .. }
        | E::OrientationUndecided { .. }
        | E::Pcurve { .. }
        | E::RingMeetsOuter { .. }
        | E::RingContactEscalated { .. }
        | E::RingOutsideOuter { .. }
        | E::RingNestingUndecided { .. }
        | E::RingMeetsRing { .. }
        | E::RingPairContactEscalated { .. }
        | E::PinchCornerCrossed { .. }
        | E::PinchCornerEscalated { .. }
        | E::ShellWinding { .. }
        | E::SolidOuterShells { .. }
        | E::ShellRoleUndecided { .. }
        | E::CuspEdgesMissing { .. }
        | E::DanglingTopology { .. }
        | E::DanglingGeometry { .. }
        | E::NextPrevMismatch { .. }
        | E::LoopCycleOverrun { .. }
        | E::ParentLoopMismatch { .. }
        | E::UnreachableHalfEdge { .. }
        | E::EdgeHalvesIdentical { .. }
        | E::EdgeSlotBackpointerMismatch { .. }
        | E::HalfEdgeUnclaimed { .. }
        | E::HalfEdgeMultiplyClaimed { .. }
        | E::EdgeNotAntiparallel { .. }
        | E::EmanatingStartMismatch { .. }
        | E::EmptyLoopVertexWithEmanating { .. }
        | E::LoneVertexWithIncidence { .. }
        | E::VertexOrbitOverrun { .. }
        | E::OrbitForeignMember { .. }
        | E::SplitVertexOrbit { .. }
        | E::OuterListedAsRing { .. }
        | E::BackPointerMismatch { .. }
        | E::OrphanEntity { .. }
        | E::MultiplyOwned { .. }
        | E::OrphanGeometry { .. }
        | E::SolidWithoutShells { .. }
        | E::ShellWithoutFaces { .. }
        | E::EdgeAcrossShells { .. }
        | E::ComponentEulerViolation { .. }
        | E::MissingProvenance { .. }
        | E::LeakedProvenance { .. }
        | E::ScaffoldingEmptyLoop { .. }
        | E::ScaffoldingStrutVertex { .. }
        | E::ShellDisconnected { .. }
        | E::NullScaffoldShared { .. }
        | E::LeakedNullFaceRecord { .. }
        | E::StaleNullFaceLoop { .. }
        | E::StaleNullFaceOwnership { .. }
        | E::NullEdgeAtRest { .. }
        | E::NullFaceAtRest { .. } => return None,
    };
    Some((x.min(y), x.max(y)))
}

/// **The one arrival a census finding is inside**: the group (in
/// [`copy_groups`]'s numbering) holding both solids it is about, `None`
/// for a finding between two arrivals or about no pair of solids.
pub(crate) fn finding_group<T: Decide>(
    body: &Body<T>,
    groups: &BTreeMap<SolidKey, u32>,
    error: &ValidationError,
) -> Option<u32> {
    let (a, b) = finding_solids(body, error)?;
    let group = *groups.get(&a)?;
    (groups.get(&b) == Some(&group)).then_some(group)
}

/// **Which copy each of the product's solids is**: the census's groups
/// at rest ([`topo::record_at_rest`]), each copy its own.
pub(crate) fn copy_groups<T: Decide>(copies: &[GatheredCopy<T>]) -> BTreeMap<SolidKey, u32> {
    copies
        .iter()
        .zip(0u32..)
        .flat_map(|(c, i)| c.keys.solids().iter().map(move |&s| (s, i)))
        .collect()
}

/// **A cell of the gathered product, named as its copy reads it**:
/// which copy holds it (by its index in `copies`) and the cell as that
/// copy's placement output names it, `None` for a cell of no copy.
pub(crate) fn copy_cell<T: Decide>(
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
pub(crate) fn copy_rows<T: Decide>(
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

/// **Two copies intersected**: the overlap of their material, `None`
/// when it is empty.
///
/// # Errors
///
/// Why it could not be formed.
fn intersection(
    product: &Product<f64>,
    a: usize,
    b: usize,
    tol: Tol,
) -> Result<Option<topo::BooleanBody<f64>>, Unlocalized> {
    let operand = |i: usize| f64::gate_at_rest_kept((*product.copies[i].body).clone(), tol);
    let (body_a, body_b) = match (operand(a), operand(b)) {
        (Ok(x), Ok(y)) => (x, y),
        (Err(errors), _) | (_, Err(errors)) => return Err(Unlocalized::Invalid { errors }),
    };
    match topo::intersect(&body_a, &body_b, tol) {
        Ok(topo::BooleanResult::Body(result)) => Ok(Some(result)),
        Ok(topo::BooleanResult::Empty) => Ok(None),
        Err(refusal) => Err(Unlocalized::Refused {
            refusal: refusal.to_string(),
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
    a: usize,
    b: usize,
    evidence: Vec<ValidationError>,
    tol: Tol,
) -> Vec<InterferenceFinding> {
    let copy_ref = |i: usize| copy_ref(&product.copies[i]);
    let finding = |overlap, quiet: Option<(RecipeNodeId, crate::names::MemberAt)>, quietable| {
        let (quiet, quiet_at) = quiet.map_or((None, Vec::new()), |(id, at)| (Some(id), at));
        InterferenceFinding {
            a: copy_ref(a),
            b: copy_ref(b),
            evidence: evidence.clone(),
            overlap,
            quiet,
            quiet_at,
            quietable,
        }
    };
    let unlocalized = |why| vec![finding(Overlap::Unlocalized(why), None, false)];
    let result = match intersection(product, a, b, tol) {
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
    for (i, (component, verdict)) in components.iter().zip(verdicts).enumerate() {
        let mut faces = Vec::with_capacity(component.sources.len());
        for &(copy, face) in &component.sources {
            let Some(name) = product.names.name_of(&EntityRef {
                body: 0,
                key: EntityKey::Face(face),
            }) else {
                return unlocalized(Unlocalized::Unnamed);
            };
            let copy = copy_ref(copy);
            // The site's name is the copy's own.
            let face = keyed_by(name, copy.var).unwrap_or(name).clone();
            faces.push(FaceSite { copy, face });
        }
        faces.sort();
        faces.dedup();
        let quietable = covered(component)
            && (matches!(verdict, Quiet::By(..))
                || (matches!(verdict, Quiet::Loud)
                    && could_quiet(
                        product,
                        (a, b),
                        &components,
                        i,
                        &result,
                        tol,
                        &mut crossings,
                    )));
        out.push(match verdict {
            Quiet::Loud => finding(Overlap::Bounded { faces }, None, quietable),
            Quiet::By(assertion, at) if covered(component) => {
                finding(Overlap::Bounded { faces }, Some((assertion, at)), quietable)
            }
            Quiet::By(..) => finding(Overlap::Bounded { faces }, None, quietable),
            Quiet::Refused(refusal) => finding(
                Overlap::Unlocalized(Unlocalized::Containment { refusal }),
                None,
                false,
            ),
        });
    }
    out
}

/// **Whether an assertion the quieting rule accepts could quiet the
/// component `i`**, asked of the rule itself: for each pair of the faces
/// bounding it, one of copy `a` and one of copy `b`, in both role orders,
/// whether a holding `Gap ≤ −δ` over the pair would be one the rule takes
/// ([`gap_assertions`]: the faces [`opposed`], the gap they measure
/// decided negative) and whether the component lies between their
/// carriers as [`quiet_verdicts`] decides it ([`crossing`]). The caller
/// asks [`localize`]'s `covered` first, which no assertion changes.
/// `crossings` keeps each face's crossing, built once.
fn could_quiet(
    product: &Product<f64>,
    (a, b): (usize, usize),
    components: &[Component],
    i: usize,
    result: &topo::BooleanBody<f64>,
    tol: Tol,
    crossings: &mut BTreeMap<FaceKey, Result<BTreeSet<usize>, String>>,
) -> bool {
    use crate::eval::measure::carrier_of;
    let Ok(band) = Band::linear(tol) else {
        return false;
    };
    let body = &product.body;
    let of = |copy| {
        components[i]
            .sources
            .iter()
            .filter(move |(c, _)| *c == copy)
            .map(|&(_, f)| f)
            .collect::<Vec<_>>()
    };
    let carrier = |key| {
        carrier_of(
            body,
            EntityRef {
                body: 0,
                key: EntityKey::Face(key),
            },
        )
    };
    let (from_a, from_b) = (of(a), of(b));
    let mut candidates = Vec::new();
    for &x in &from_a {
        for &y in &from_b {
            for (outer, inner) in [(x, y), (y, x)] {
                let negative =
                    gap_sign(&carrier(outer), &carrier(inner), AT_REST_QUIET_BOUND, band)
                        == Some(Sign::Negative);
                if negative && opposed(body, outer, inner, band) {
                    candidates.push((outer, inner));
                }
            }
        }
    }
    candidates.into_iter().any(|(outer, inner)| {
        [outer, inner].into_iter().all(|face| {
            let crossed = crossings
                .entry(face)
                .or_insert_with(|| crossing(body, face, components, result, tol));
            matches!(crossed, Ok(set) if !set.contains(&i))
        })
    })
}

/// The connected solids of `result`, each with the aggregate faces its
/// faces descend from; `None` when a face's lineage does not reach a
/// face of either copy.
fn components<T: Decide>(
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
                let (operand, root) = descent.result_face(piece);
                let copy = match operand {
                    topo::Operand::A => a,
                    topo::Operand::B => b,
                };
                sources.insert((copy, product.copies[copy].keys.face(root)?));
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
    /// This assertion, as its member at these values, quiets it.
    By(RecipeNodeId, crate::names::MemberAt),
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
                    verdicts[i] = Quiet::By(g.assertion, g.at.clone());
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
        Err(refusal) => return Err(refusal.to_string()),
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
            if let (topo::Operand::A, root) = descent.result_face(f)
                && let Some(&i) = result
                    .body
                    .solid_of_face(root)
                    .and_then(|s| of_solid.get(&s))
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
        aggregate,
        EntityRef {
            body: 0,
            key: EntityKey::Face(face),
        },
    );
    let origin = match &carrier {
        Carrier::Plane { origin, .. } | Carrier::Cylinder { origin, .. } => *origin,
        _ => return Err("the containment check has no lane for this carrier".to_owned()),
    };
    // Every face of the intersection a plane or a cylinder patch, whose
    // reach its own boundary bounds.
    let mut reach = 0.0_f64;
    for (f, held) in result.faces() {
        match result.get_surface(held.surface) {
            Some(topo::Surface::Plane { .. } | topo::Surface::Cylinder { .. }) => {}
            _ => return Err("a face of the overlap the containment check cannot bound".to_owned()),
        }
        let r = reach_of(result, f, origin).ok_or("a face of the overlap with no reach")?;
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
        } => (origin, outward, vec![rectangle(span)], span),
        Carrier::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let base = origin - axis * span;
            // A concave face (a bore) has its material outside the
            // carrier, so the region past it is the solid cylinder; a
            // convex one (a pin) the tube around it.
            let loops = if sense {
                vec![circle(radius + 2.0 * span), circle(radius)]
            } else {
                vec![circle(radius)]
            };
            (base, axis, loops, 2.0 * span)
        }
        _ => unreachable!("the carrier was matched above"),
    };
    // The region is turned with the carrier's own chart, so it moves
    // with the inputs and not with the computing axes.
    let reference = match aggregate
        .get_face(face)
        .and_then(|f| aggregate.get_surface(f.surface))
    {
        Some(topo::Surface::Plane { u_ref, .. } | topo::Surface::Cylinder { u_ref, .. }) => *u_ref,
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
    .map_err(|e| e.to_string())?;
    f64::gate_at_rest_kept(swept.body, tol).map_err(|errors| format!("{errors:?}"))
}
