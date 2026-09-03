//! **The E7 clearance engine**: the trichotomy over one certified leaf
//! of the E6 driver, and the same engine run at `c = 0⁺` as the global
//! parametric self-intersection check.
//!
//! Gated on `interval` for the driver's own reason: the inner
//! subdivision excludes by interval enclosure, and without that scalar
//! there is nothing to exclude WITH.
//!
//! Two nested subdivisions. The OUTER one is [`mod@crate::drive`]'s: it
//! hands this module a leaf whose topology is provably the witness
//! build's. This module never re-litigates that certification and never
//! touches the mass accounting except to consume it. The INNER one is
//! here — a subdivision of the two faces' GEOMETRY domains with
//! interval exclusion, run with the leaf's parameters bound as
//! intervals.
//!
//! # What each verdict claims, exactly
//!
//! [`ClearanceVerdict::Holds`] is a certificate over the leaf's whole
//! parameter box AND the whole of both selections: at every parameter
//! point in the box and every pair of points on the two carrier
//! windows, the separation satisfies the bound. Nothing samples,
//! nothing averages, and no probability enters — mass accounting
//! applies to LEAVES (E6), never inside one.
//!
//! [`ClearanceVerdict::Violated`] carries a concrete parameter point
//! and a concrete pair of surface points, and their distance is
//! **re-verified at `f64`** by an independent rebuild before the
//! verdict is minted ([`verify_witness`]). A witness the `f64`
//! recomputation does not confirm is never reported as one: the engine
//! refuses [`ClearanceRefusal::WitnessUnverified`] instead.
//!
//! [`ClearanceVerdict::Refused`] is typed: a terminal sliver (the
//! driver's rule shape — the deciding enclosure sits wholly inside the
//! funnel's band, so refinement provably cannot move it), a named
//! budget, a carrier with no admitted window, or a selection that does
//! not resolve.
//!
//! # The one place a verdict is loose, stated at the door
//!
//! The inner subdivision runs over each face's **carrier window** — the
//! rectangle of surface parameters derived in [`window_of`]. A window
//! is a conservative SUPERSET of the face's trimmed region: an L-shaped
//! planar face's window is its bounding rectangle, and a cylindrical
//! face's window is the whole turn at the face's axial span.
//!
//! So the looseness runs one way, and it is the safe way for a defect
//! gate. `Holds` covers strictly more than the faces and is therefore
//! sound about them. `Violated` may report a sub-`c` approach between
//! two carrier windows at a place neither face actually occupies —
//! the same direction, and the same sentence, `topo::shell`'s own
//! closed-form `wall_clearance` gate states about its projected
//! footprints: it may refuse a body whose faces do not really face each
//! other; it cannot miss a pair that does. Tightening a window to the
//! trimmed region needs the face's boundary in CHART coordinates, which
//! is the pcurve layer's description work and not this module's.
//!
//! # No ε here, and two funnelled compares
//!
//! Every decision goes through the one `k_stats` funnel at a named
//! predicate with a [`Margin`] door, and there are exactly two:
//! [`CLEARANCE_MARGIN`] (`d − c`, metres minus metres) and
//! [`SELF_INTERSECTION_GAP`] (`d`, the separation a non-adjacent face
//! pair must classify strictly positive). Both carry a row in
//! `docs/predicate-dimension-audit.md`. Nothing here compares a float
//! raw, mints a tolerance, or reads a width as evidence.
//!
//! # Read-only (E8)
//!
//! Every door takes `&Doc` and returns a value. There is no `&mut` on
//! the path and no re-witnessing: this lane cannot express a document
//! write, which is a stronger statement than one it does not perform.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use bvh::{Aabb, Bvh};
use geom::Surface;
use geom_core::interval::Interval;
use geom_core::k_stats::decide;
use geom_core::{Band, Bounds, Margin, Point3, Real, Sign, Tol, Vec3};
use topo::Body;
use topo::entity::{EdgeKey, FaceKey, LoopBoundary, VertexKey};

use crate::analysis::{BoxAxis, ParamBox};
use crate::doc::{Doc, ParamName};
use crate::drive::{CertifiedLeaf, MeasureAccounting, ParamBoxVerdict, lane_opts, sliver};
use crate::eval::{CancelToken, EvalOptions, Evaluation, NodeResult, evaluate};
use crate::names::{EntityKey, Entry, StableName};
use crate::node::RecipeNodeId;
use crate::program::ProfileProgram;

/// The funnel site name of the clearance comparison — the separation
/// enclosure between two domain cells minus the requested clearance,
/// in metres against the linear band.
///
/// A roster carrier (`docs/K-REPORT.md`) rather than a literal at the
/// decide site, following [`crate::measure::ASSERT_BOUND`].
pub const CLEARANCE_MARGIN: &str = "clearance_margin";

/// The funnel site name of the self-intersection comparison — the
/// separation enclosure itself, which a non-adjacent face pair must
/// classify strictly positive.
///
/// This is E7's "certified strictly positive distance" spelled as a
/// funnelled compare rather than as an ε: the band's own coincidence
/// threshold is what "strictly" means, so `Sign::Zero` — two
/// non-adjacent faces coincident at the run's tolerance — is a
/// violation here, where under [`CLEARANCE_MARGIN`]'s non-strict `≥ c`
/// it is a discharge. Same margin shape, same band, two readings of
/// zero, one per site.
pub const SELF_INTERSECTION_GAP: &str = "self_intersection_gap";

/// How deep ONE cell pair may be subdivided before the engine refuses
/// the rest of its subtree rather than refining it further.
///
/// A recorded run dial (E6's "run config like K"), not a constant of
/// nature. 40 halvings take a domain axis to `2^-40` of its window —
/// twelve decimal digits, past the point where a carrier's own
/// evaluation carries information at `f64`.
pub const DEFAULT_MAX_CELL_DEPTH: u32 = 40;

/// The whole-query cell-pair budget: how many cell pairs one clearance
/// query may examine before the rest of its frontier refuses
/// [`CellBudget::Pairs`].
///
/// ENFORCED AT ADMISSION, like the driver's leaf budget: a split that
/// would commit the query past this number is refused instead of taken.
/// A recorded run dial; `65_536` matches the driver's leaf budget so a
/// leaf-times-pairs run stays a thing a report can hold.
pub const DEFAULT_MAX_CELL_PAIRS: usize = 65_536;

/// Which question the engine is answering — the two readings of a
/// definite `Sign::Zero`, each at its own funnel site.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClearanceBound {
    /// `separation ≥ c` (metres), non-strict: a separation equal to `c`
    /// at the run's tolerance HOLDS. The assertion lane's convention
    /// ([`crate::measure::decide_assertion`]).
    AtLeast(f64),
    /// Strictly positive separation — E7's self-intersection question.
    /// A coincidence at the run's tolerance is a VIOLATION, not a
    /// discharge.
    StrictlyPositive,
}

impl ClearanceBound {
    /// The metre bound the margin is taken against.
    fn c(self) -> f64 {
        match self {
            Self::AtLeast(c) => c,
            Self::StrictlyPositive => 0.0,
        }
    }

    /// The funnel site this bound decides at.
    fn predicate(self) -> &'static str {
        match self {
            Self::AtLeast(_) => CLEARANCE_MARGIN,
            Self::StrictlyPositive => SELF_INTERSECTION_GAP,
        }
    }

    /// Whether a definite `Zero` discharges (see the two site docs).
    fn zero_discharges(self) -> bool {
        matches!(self, Self::AtLeast(_))
    }
}

/// Whether the monotonicity accelerator may run (E7's "duals
/// accelerate, never decide").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Pruning {
    /// The leaf's box is examined whole. The default, and the state
    /// every verdict is defined against.
    #[default]
    Off,
    /// A parameter the oracle proves the separation monotone in is
    /// collapsed to the box facet where the separation is smallest.
    /// An ACCELERATOR: the verdict is the same, the work is less.
    Facets,
}

/// How one clearance query is run: the two budgets and the accelerator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClearanceConfig {
    /// Per-pair subdivision depth ([`DEFAULT_MAX_CELL_DEPTH`]).
    pub max_cell_depth: u32,
    /// Whole-query cell-pair budget ([`DEFAULT_MAX_CELL_PAIRS`]).
    pub max_cell_pairs: usize,
    /// Whether the monotonicity accelerator may restrict the leaf box
    /// to a facet ([`Pruning`]).
    pub pruning: Pruning,
}

impl Default for ClearanceConfig {
    fn default() -> Self {
        Self {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: DEFAULT_MAX_CELL_PAIRS,
            pruning: Pruning::Off,
        }
    }
}

/// **The monotonicity accelerator's seam** (E7: duals accelerate,
/// never decide; E9: a degraded tangent forfeits exactly its uses).
///
/// An implementor answers, for one parameter of the leaf's box,
/// whether the separation the query is about is monotone in that
/// parameter over the WHOLE leaf — `Some(Positive)` for
/// non-decreasing, `Some(Negative)` for non-increasing, `Some(Zero)`
/// for constant, and `None` for anything it cannot certify.
///
/// **`None` is the E9 state and costs exactly the pruning.** A degraded
/// tangent — a Clarke straddle hull, a kink-jump enclosure — reaches
/// this door as `None`, the leaf keeps its whole span on that axis, and
/// no other part of the verdict moves.
///
/// **The claim is the implementor's, and the engine cannot check it.**
/// A sign-definite `Dual<Interval>` enclosure of `∂d/∂pᵢ` composed with
/// the leaf's box is what earns a `Some` here; the seed door that
/// produces one is a sibling unit's, and until it lands the shipped
/// implementor is [`NoTangents`], which forfeits every pruning. That is
/// why the accelerator is defined as a value the caller supplies rather
/// than as machinery this module owns: an accelerator correctness can
/// never depend on must be REMOVABLE, and a seam is how that is
/// checked rather than asserted.
pub trait MonotoneOracle {
    /// The sign of `∂d/∂p` over the whole leaf, or `None`.
    fn monotone_in(&self, param: &ParamName) -> Option<Sign>;
}

/// The oracle that certifies nothing: E9's state, and the shipped
/// default until the tangent seed door lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NoTangents;

impl MonotoneOracle for NoTangents {
    fn monotone_in(&self, _param: &ParamName) -> Option<Sign> {
        None
    }
}

/// **How one clearance question is asked**: the bound, the run's
/// tolerance, the dials, and the accelerator seam.
///
/// One value rather than four arguments, because these four travel
/// together everywhere — the per-leaf door, the driver-level fold and
/// the witness verification all take the same four, and a caller that
/// got them out of step between two calls would be comparing two
/// different questions' answers.
pub struct ClearanceQuery<'a> {
    /// Which question ([`ClearanceBound`]).
    pub bound: ClearanceBound,
    /// The run's tolerance — the linear band every compare classifies
    /// against.
    pub tol: Tol,
    /// The budgets and the accelerator switch.
    pub config: ClearanceConfig,
    /// The monotonicity seam. [`NoTangents`] forfeits every pruning,
    /// which is the state every verdict is defined against.
    pub oracle: &'a dyn MonotoneOracle,
}

impl ClearanceQuery<'_> {
    /// The plain `separation ≥ c` question at the shipped dials, with
    /// no accelerator.
    pub fn at_least(c: f64, tol: Tol) -> Self {
        Self {
            bound: ClearanceBound::AtLeast(c),
            tol,
            config: ClearanceConfig::default(),
            oracle: &NoTangents,
        }
    }

    /// The self-intersection question at the shipped dials.
    pub fn strictly_positive(tol: Tol) -> Self {
        Self {
            bound: ClearanceBound::StrictlyPositive,
            tol,
            config: ClearanceConfig::default(),
            oracle: &NoTangents,
        }
    }
}

/// Which faces of which body a clearance question is about.
#[derive(Debug, Clone, PartialEq)]
pub struct Selection {
    /// The recipe node whose output the selection reads.
    pub at: RecipeNodeId,
    /// Which of that node's output bodies (0 for the common single-body
    /// payloads — [`crate::names::interrogate`]'s index).
    pub body: u32,
    /// Which faces of it.
    pub faces: FaceScope,
}

impl Selection {
    /// Every face of body 0 of a node — the whole-body question.
    pub fn body_of(at: RecipeNodeId) -> Self {
        Self {
            at,
            body: 0,
            faces: FaceScope::All,
        }
    }
}

/// Which faces of a selection's body take part.
#[derive(Debug, Clone, PartialEq)]
pub enum FaceScope {
    /// Every face of the body, in arena order.
    All,
    /// The faces the named entities resolve to, in the node's own name
    /// table. A name that does not resolve to a face of this body is a
    /// typed refusal, never a silently dropped face.
    Named(Vec<StableName>),
}

/// The E7 trichotomy.
#[derive(Debug, Clone, PartialEq)]
pub enum ClearanceVerdict {
    /// The bound holds over the whole leaf and both carrier windows.
    Holds,
    /// A definite violation, witnessed and re-verified at `f64`.
    Violated(Box<Violation>),
    /// Typed, never silence.
    Refused(ClearanceRefusal),
}

impl ClearanceVerdict {
    /// The verdict's state as a word, for reports.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Holds => "Holds",
            Self::Violated(_) => "Violated",
            Self::Refused(_) => "Refused",
        }
    }

    /// Does the bound hold? `None` when there is no verdict — the three
    /// states stay three at every reader, so nothing collapses a
    /// refusal into a silent pass ([`crate::measure::AssertionVerdict`]'s
    /// convention).
    pub fn holds(&self) -> Option<bool> {
        match self {
            Self::Holds => Some(true),
            Self::Violated(_) => Some(false),
            Self::Refused(_) => None,
        }
    }
}

/// A definite violation: where in the parameter box, and where on the
/// two carriers.
#[derive(Debug, Clone, PartialEq)]
pub struct Violation {
    /// The parameter point, re-evaluated at `f64` to verify.
    pub param: ParamWitness,
    /// The closest-point pair the `f64` rebuild found.
    pub geometry: GeometryWitness,
}

/// A concrete parameter point inside the leaf.
///
/// The leaf's own midpoint, through [`BoxAxis::midpoint`] — the same
/// door the driver's split rule and its K-telemetry replay use, so a
/// witness is always a point some part of the analysis actually stood
/// on.
#[derive(Debug, Clone, PartialEq)]
pub struct ParamWitness {
    /// Per parameter, the OFFSET from the document's nominal (the
    /// analysis lane's own currency — [`crate::analysis::AnalyzedParam`]).
    pub offsets: BTreeMap<ParamName, f64>,
}

/// A concrete pair of surface points, at `f64`, with the distance the
/// `f64` rebuild measured between them.
///
/// Equality is spelled rather than derived because [`Point3`] carries
/// no `PartialEq` — a geometric point is not something the kernel
/// compares by value. Here the comparison is over a REPORT's fields,
/// where bit equality is the only question a reader asks (did two runs
/// produce the same witness), so it is written out, coordinate by
/// coordinate, at this type and nowhere else.
#[derive(Debug, Clone, Copy)]
pub struct GeometryWitness {
    /// The first face.
    pub a: FaceKey,
    /// Its carrier parameters.
    pub a_uv: (f64, f64),
    /// The point there.
    pub a_point: Point3<f64>,
    /// The second face.
    pub b: FaceKey,
    /// Its carrier parameters.
    pub b_uv: (f64, f64),
    /// The point there.
    pub b_point: Point3<f64>,
    /// The distance between the two points, at `f64`.
    pub distance: f64,
}

impl PartialEq for GeometryWitness {
    fn eq(&self, other: &Self) -> bool {
        let same = |p: Point3<f64>, q: Point3<f64>| p.x == q.x && p.y == q.y && p.z == q.z;
        self.a == other.a
            && self.b == other.b
            && self.a_uv == other.a_uv
            && self.b_uv == other.b_uv
            && self.distance == other.distance
            && same(self.a_point, other.a_point)
            && same(self.b_point, other.b_point)
    }
}

/// Why a clearance query has no verdict. Typed and named, never a
/// silent partial.
#[derive(Debug, Clone, PartialEq)]
pub enum ClearanceRefusal {
    /// A genuine semantic sliver: the deciding enclosure sits WHOLLY
    /// inside the ambiguity band, so the separation being decided IS in
    /// the band and refinement cannot move it out. The driver's rule,
    /// through the driver's own door.
    Sliver {
        /// The funnel site that could not decide.
        predicate: &'static str,
    },
    /// A budget was exhausted, typed and named.
    Budget(CellBudget),
    /// A face whose carrier has no admitted window (E7: refuse typed,
    /// never downgrade to sampling).
    Unsupported {
        /// The carrier class, named.
        carrier: &'static str,
        /// The face that carries it.
        face: FaceKey,
    },
    /// The selection itself could not be read.
    Selection(SelectionRefusal),
    /// A cell pair the interval pass classified as a definite violation
    /// whose witness the `f64` rebuild did not confirm.
    ///
    /// Its own arm, and never a `Violated`: a witness is a claim about a
    /// concrete configuration, and one the independent recomputation
    /// contradicts is evidence about the analysis, not about the
    /// document. The prose carries what the rebuild found.
    WitnessUnverified {
        /// What the `f64` rebuild could not confirm.
        what: String,
    },
}

impl ClearanceRefusal {
    /// The refusal's stable class name, for reports and the goldening
    /// form.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Sliver { .. } => "sliver",
            Self::Budget(_) => "budget",
            Self::Unsupported { .. } => "unsupported",
            Self::Selection(_) => "selection",
            Self::WitnessUnverified { .. } => "witness_unverified",
        }
    }
}

/// Which bound stopped the subdivision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellBudget {
    /// One cell pair had already been split
    /// [`ClearanceConfig::max_cell_depth`] times.
    Depth {
        /// The bound.
        max_cell_depth: u32,
    },
    /// The query had reached [`ClearanceConfig::max_cell_pairs`]; this
    /// pair was still on the frontier and is refused rather than
    /// forgotten.
    Pairs {
        /// The bound.
        max_cell_pairs: usize,
    },
    /// The cell could not be bisected: its midpoint on the split axis
    /// lands on an endpoint, so the `f64` grid itself is the bound. A
    /// work bound like the other two, typed separately because widening
    /// a budget does not move it.
    Resolution,
}

/// How a selection could not be read.
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionRefusal {
    /// The node did not build in the leaf's replay.
    NodeDidNotBuild {
        /// The node.
        node: RecipeNodeId,
    },
    /// The node's payload carries no body at that index.
    NoSuchBody {
        /// The node.
        node: RecipeNodeId,
        /// The index asked for.
        index: u32,
    },
    /// A name in the scope does not resolve uniquely in the node's own
    /// table.
    Unresolved {
        /// The name, rendered.
        name: String,
    },
    /// A name resolves, but not to a face of the selected body.
    NotAFace {
        /// The name, rendered.
        name: String,
    },
}

impl core::fmt::Display for SelectionRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NodeDidNotBuild { node } => write!(
                f,
                "node {} did not build in this leaf's replay, so it has no faces to \
                 measure a clearance between",
                node.0
            ),
            Self::NoSuchBody { node, index } => write!(
                f,
                "node {}'s value carries no body at index {index}",
                node.0
            ),
            Self::Unresolved { name } => write!(
                f,
                "{name} does not resolve to a unique entity in the selected node's name table"
            ),
            Self::NotAFace { name } => {
                write!(f, "{name} resolves, but not to a face of the selected body")
            }
        }
    }
}

impl core::error::Error for SelectionRefusal {}

/// The counting receipt of one clearance query.
///
/// Each candidate pair is the root of its own binary subdivision: a
/// cell pair is discharged, violated, refused, or split into exactly
/// two. A forest of `candidates` binary trees with `splits` interior
/// nodes has exactly `splits + candidates` leaves, and every leaf is in
/// one of the three buckets — which is what "the discharged, violated
/// and refused cell pairs account for every candidate pair" means
/// arithmetically. It rides the shipped report, so a consumer re-checks
/// it without trusting this module ([`CellReceipt::holds`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CellReceipt {
    /// Candidate face pairs the interval BVH did not exclude.
    pub candidates: usize,
    /// Cell pairs discharged (separation definitely satisfies the
    /// bound).
    pub discharged: usize,
    /// Cell pairs whose separation definitely violates it.
    pub violated: usize,
    /// Cell pairs refused, typed.
    pub refused: usize,
    /// Cell pairs that were split.
    pub splits: usize,
}

impl CellReceipt {
    /// The receipt identity (see the type docs).
    pub fn holds(&self) -> bool {
        self.discharged + self.violated + self.refused == self.splits + self.candidates
    }

    /// Folds another query's receipt in — the driver-level fold's
    /// currency. The identity is additive: a sum of forests is a
    /// forest.
    fn add(&mut self, other: Self) {
        self.candidates += other.candidates;
        self.discharged += other.discharged;
        self.violated += other.violated;
        self.refused += other.refused;
        self.splits += other.splits;
    }
}

/// **The honest limit, measured rather than asserted** — at what cell
/// width the engine could actually discharge.
///
/// A `Holds` certificate says nothing about cost. These three numbers
/// do: the widest and narrowest 3-D cell diameters at which a pair
/// discharged, and the deepest subdivision any pair needed. A query
/// whose narrowest discharge is orders below its widest is a query
/// where one region did all the work.
///
/// The widening class those numbers belong to — enclosures whose width
/// is driven by the parameter box rather than by the domain cell — is
/// issue 1191's, observed here on a different family. This unit
/// measures and cites; it does not re-file.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DischargeWidths {
    /// The largest cell diameter at which a pair discharged, in metres.
    /// `None` when nothing discharged.
    pub widest: Option<f64>,
    /// The smallest, in metres.
    pub narrowest: Option<f64>,
    /// The deepest subdivision any candidate pair reached.
    pub deepest: u32,
}

impl DischargeWidths {
    fn empty() -> Self {
        Self {
            widest: None,
            narrowest: None,
            deepest: 0,
        }
    }

    fn discharged_at(&mut self, diameter: f64) {
        self.widest = Some(self.widest.map_or(diameter, |w: f64| w.max(diameter)));
        self.narrowest = Some(self.narrowest.map_or(diameter, |w: f64| w.min(diameter)));
    }

    fn fold(&mut self, other: Self) {
        if let Some(w) = other.widest {
            self.widest = Some(self.widest.map_or(w, |x: f64| x.max(w)));
        }
        if let Some(w) = other.narrowest {
            self.narrowest = Some(self.narrowest.map_or(w, |x: f64| x.min(w)));
        }
        self.deepest = self.deepest.max(other.deepest);
    }
}

/// What one clearance query answers: the verdict, its receipt, and the
/// measured limit.
#[derive(Debug, Clone, PartialEq)]
pub struct ClearanceReport {
    verdict: ClearanceVerdict,
    receipt: CellReceipt,
    widths: DischargeWidths,
}

impl ClearanceReport {
    /// The trichotomy.
    pub fn verdict(&self) -> &ClearanceVerdict {
        &self.verdict
    }

    /// The counting receipt.
    pub fn receipt(&self) -> CellReceipt {
        self.receipt
    }

    /// The measured discharge widths.
    pub fn widths(&self) -> DischargeWidths {
        self.widths
    }

    /// The report's goldening form: a deterministic, float-exact text
    /// rendering, in the driver's own idiom (every float as its exact
    /// bits, so the text is a faithful image rather than a rounded
    /// picture of the report).
    pub fn serialize(&self) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        let r = self.receipt;
        let _ = writeln!(
            s,
            "receipt candidates={} discharged={} violated={} refused={} splits={} holds={}",
            r.candidates,
            r.discharged,
            r.violated,
            r.refused,
            r.splits,
            r.holds()
        );
        match &self.verdict {
            ClearanceVerdict::Holds => {
                let _ = writeln!(s, "verdict holds");
            }
            ClearanceVerdict::Violated(v) => {
                let _ = writeln!(
                    s,
                    "verdict violated distance={:016x}",
                    v.geometry.distance.to_bits()
                );
                for (name, offset) in &v.param.offsets {
                    let _ = writeln!(s, "witness {} {:016x}", name.0, offset.to_bits());
                }
            }
            ClearanceVerdict::Refused(r) => {
                let _ = writeln!(s, "verdict refused {}", r.name());
            }
        }
        let render = |w: Option<f64>| {
            w.map_or_else(|| "none".to_owned(), |x| format!("{:016x}", x.to_bits()))
        };
        let _ = writeln!(
            s,
            "widths widest={} narrowest={} deepest={}",
            render(self.widths.widest),
            render(self.widths.narrowest),
            self.widths.deepest
        );
        s
    }

    /// A report that never got as far as a subdivision.
    fn refused(refusal: ClearanceRefusal) -> Self {
        Self {
            verdict: ClearanceVerdict::Refused(refusal),
            receipt: CellReceipt::default(),
            widths: DischargeWidths::empty(),
        }
    }
}

// ------------------------------------------------------------- doors

/// **The clearance question over one certified leaf** (E7): is the
/// separation between the two selections at least `c` metres,
/// everywhere in the leaf's parameter box?
///
/// The caller iterates leaves; [`clearance_over`] is the convenience
/// that folds a whole [`ParamBoxVerdict`].
pub fn clearance(
    doc: &Doc<ProfileProgram>,
    leaf: &ParamBox,
    a: &Selection,
    b: &Selection,
    c: f64,
    tol: Tol,
) -> ClearanceReport {
    clearance_with(doc, leaf, a, b, &ClearanceQuery::at_least(c, tol))
}

/// **Global self-intersection freedom over one certified leaf** (E7's
/// census made global and parametric): every NON-ADJACENT pair of the
/// selection's faces certified strictly positive separation.
///
/// Adjacent pairs are excluded exactly per the wedge rule — two faces
/// sharing a vertex (which every edge-sharing pair does) meet, and
/// their separation is legitimately zero; deciding that pair here would
/// report the body's own construction as a defect. Their business is
/// the wedge predicates', at the vertex.
pub fn self_intersection(
    doc: &Doc<ProfileProgram>,
    leaf: &ParamBox,
    of: &Selection,
    tol: Tol,
) -> ClearanceReport {
    clearance_with(doc, leaf, of, of, &ClearanceQuery::strictly_positive(tol))
}

/// The full door: both questions, both budgets, and the accelerator
/// seam.
///
/// `a` and `b` may name the same selection, which is what makes this
/// one engine rather than two: a query against itself pairs each face
/// with each LATER face of the same body and drops the adjacent pairs,
/// and a cross query pairs the two selections' faces without an
/// adjacency rule (two faces of different bodies are never adjacent in
/// the wedge sense — no vertex is shared).
///
/// The verdict combines the pairs' outcomes in one fixed order: a
/// definite violation outranks a refusal, which outranks `Holds`. A
/// finding is a stronger statement than an unknown, and an unknown is
/// never presented as a pass.
pub fn clearance_with(
    doc: &Doc<ProfileProgram>,
    leaf: &ParamBox,
    a: &Selection,
    b: &Selection,
    query: &ClearanceQuery<'_>,
) -> ClearanceReport {
    let Ok(band) = Band::linear(query.tol) else {
        return ClearanceReport::refused(ClearanceRefusal::WitnessUnverified {
            what: "the run's tolerance does not admit a linear band".to_owned(),
        });
    };
    // The accelerator, before anything is evaluated: a facet is a
    // sub-box of the leaf, so restricting to one restricts the geometry
    // the whole query is about. `Pruning::Off` is the identity.
    let queried = match query.config.pruning {
        Pruning::Off => leaf.clone(),
        Pruning::Facets => facet_restrict(leaf, query.oracle),
    };
    let opts = EvalOptions {
        param_box: Some(Arc::new(queried.clone())),
        ..lane_opts()
    };
    let ev: Evaluation<Interval> = evaluate(doc, None, &CancelToken::new(), &opts, query.tol);

    let windows_a = match windows_of(&ev, a) {
        Ok(w) => w,
        Err(r) => return ClearanceReport::refused(r),
    };
    // The SAME-BODY rule, not the same-selection one: two scopes of one
    // body can name one face twice, and a face is never at a distance
    // from itself.
    let same_body = a.at == b.at && a.body == b.body;
    let windows_b = if a == b {
        windows_a.clone()
    } else {
        match windows_of(&ev, b) {
            Ok(w) => w,
            Err(r) => return ClearanceReport::refused(r),
        }
    };

    let sweep = Sweep {
        bound: query.bound,
        band,
        config: query.config,
        deepest: 0,
    };
    sweep.run(doc, &queried, &windows_a, &windows_b, same_body, query.tol)
}

/// **The driver-level fold**: the clearance question over EVERY
/// certified leaf of a drive, with the drive's own accounting carried
/// through unaltered.
///
/// Thin by design. The engine's unit of answer is the leaf, and this
/// door does not invent a leaf-crossing claim: it runs the query per
/// certified leaf, combines the verdicts in the same fixed order one
/// query combines its pairs in, sums the receipts (a sum of forests is
/// a forest), and hands back the drive's [`MeasureAccounting`]
/// VERBATIM. The refused and tail mass in it is exactly the share of
/// the parameter box this certificate says nothing about — the honest
/// denominator for every sentence a report writes about the answer.
pub fn clearance_over(
    doc: &Doc<ProfileProgram>,
    verdict: &ParamBoxVerdict,
    a: &Selection,
    b: &Selection,
    query: &ClearanceQuery<'_>,
) -> LeafFold {
    let mut fold = LeafFold {
        verdict: ClearanceVerdict::Holds,
        receipt: CellReceipt::default(),
        widths: DischargeWidths::empty(),
        leaves: 0,
        accounting: verdict.accounting().clone(),
    };
    for leaf in verdict.certified() {
        let CertifiedLeaf { box_, .. } = leaf;
        let report = clearance_with(doc, box_, a, b, query);
        fold.leaves += 1;
        fold.receipt.add(report.receipt);
        fold.widths.fold(report.widths);
        fold.verdict = combine(fold.verdict, report.verdict);
    }
    fold
}

/// What [`clearance_over`] answers.
#[derive(Debug, Clone, PartialEq)]
pub struct LeafFold {
    /// The combined verdict over every certified leaf.
    pub verdict: ClearanceVerdict,
    /// The summed receipt.
    pub receipt: CellReceipt,
    /// The measured limit, folded.
    pub widths: DischargeWidths,
    /// How many certified leaves were examined.
    pub leaves: usize,
    /// The drive's accounting, verbatim — what the certificate does NOT
    /// cover.
    pub accounting: MeasureAccounting,
}

/// The fixed combination order: a definite violation outranks a
/// refusal, which outranks `Holds`. Within a rank the FIRST is kept, so
/// the answer is a function of the deterministic pair order.
fn combine(acc: ClearanceVerdict, next: ClearanceVerdict) -> ClearanceVerdict {
    match (&acc, &next) {
        (ClearanceVerdict::Violated(_), _) => acc,
        (_, ClearanceVerdict::Violated(_)) => next,
        (ClearanceVerdict::Refused(_), _) => acc,
        (_, ClearanceVerdict::Refused(_)) => next,
        _ => ClearanceVerdict::Holds,
    }
}

// ------------------------------------------------- the accelerator

/// Collapses every axis the oracle proves the separation monotone in to
/// the box facet where the separation is SMALLEST (E7's pruning; E9's
/// forfeit).
///
/// Monotone non-decreasing in `p` puts the minimum at `p = lo`;
/// non-increasing puts it at `p = hi`; constant puts it at either, and
/// `lo` is chosen so the restriction is a function of the box alone. An
/// axis the oracle does not decide keeps its whole span, and nothing
/// else about the query changes — which is the whole content of "an
/// accelerator only".
fn facet_restrict(box_: &ParamBox, oracle: &dyn MonotoneOracle) -> ParamBox {
    ParamBox::from_axes(
        box_.axes()
            .iter()
            .map(|(name, axis)| {
                let (lo, hi) = axis.span();
                let collapsed = match (axis, oracle.monotone_in(name)) {
                    (BoxAxis::Varying { .. }, Some(Sign::Positive | Sign::Zero)) => {
                        Some(BoxAxis::Varying { lo, hi: lo })
                    }
                    (BoxAxis::Varying { .. }, Some(Sign::Negative)) => {
                        Some(BoxAxis::Varying { lo: hi, hi })
                    }
                    _ => None,
                };
                (name.clone(), collapsed.unwrap_or(*axis))
            })
            .collect(),
    )
}

// ------------------------------------------------- carrier windows

/// A face's domain window: the carrier it sits on, the rectangle of
/// carrier parameters the engine subdivides, and the vertices that
/// decide adjacency.
#[derive(Debug, Clone)]
struct Window {
    /// Which node's value the face was read from, and which of its
    /// bodies — the address the `f64` witness rebuild resolves the same
    /// face at. Two nodes of one recipe can carry the SAME arena key
    /// (a rigid transform preserves them), so a rebuild that searched
    /// for a key rather than resolving it at its own node would
    /// silently measure the wrong body's face.
    at: RecipeNodeId,
    body: u32,
    face: FaceKey,
    /// Which world axis the planar re-chart crossed the normal with
    /// ([`in_plane_axis`]), so the `f64` witness rebuild can name the
    /// same chart. `None` for every non-planar carrier, which keeps its
    /// stored chart.
    chart_axis: Option<usize>,
    surface: Surface<Interval>,
    u: (f64, f64),
    v: (f64, f64),
    /// The face's boundary vertices — the wedge rule's currency, read
    /// off topology and never off geometry.
    vertices: BTreeSet<VertexKey>,
}

impl Window {
    /// The window's whole-face enclosure, as the tree's item box.
    fn root_box(&self) -> Aabb {
        cell_box(&self.surface, self.u, self.v)
    }
}

/// Reads a selection's faces out of the leaf's replay, one window each.
fn windows_of(ev: &Evaluation<Interval>, sel: &Selection) -> Result<Vec<Window>, ClearanceRefusal> {
    let refuse = ClearanceRefusal::Selection;
    let Some(NodeResult::Ok(value)) = ev.nodes.get(&sel.at) else {
        return Err(refuse(SelectionRefusal::NodeDidNotBuild { node: sel.at }));
    };
    let body = crate::names::interrogate::output_body(&value.payload, sel.body).map_err(|_| {
        refuse(SelectionRefusal::NoSuchBody {
            node: sel.at,
            index: sel.body,
        })
    })?;
    let keys: Vec<FaceKey> = match &sel.faces {
        // Arena order — the deterministic order every derived list in
        // this kernel inherits (D9).
        FaceScope::All => body.faces().map(|(k, _)| k).collect(),
        FaceScope::Named(names) => {
            let mut out = Vec::new();
            for name in names {
                let rendered = || format!("{name:?}");
                let Some(Entry::Unique(ent)) = value.name_table.lookup(name) else {
                    return Err(refuse(SelectionRefusal::Unresolved { name: rendered() }));
                };
                match ent.key {
                    EntityKey::Face(k) if ent.body == sel.body => out.push(k),
                    _ => return Err(refuse(SelectionRefusal::NotAFace { name: rendered() })),
                }
            }
            out.sort_unstable();
            out.dedup();
            out
        }
    };
    keys.into_iter()
        .map(|k| window_of(body, sel.at, sel.body, k))
        .collect()
}

/// The canonical full turn, one ulp wider at each end so the rounding
/// of `TAU` itself can never shave a sliver of azimuth off a periodic
/// window. An ulp guard is not a tolerance — the same idiom
/// [`bvh::Aabb::padded`] uses, for the same reason.
fn full_turn() -> (f64, f64) {
    (0.0f64.next_down(), core::f64::consts::TAU.next_up())
}

/// The canonical latitude range, widened the same way. Evaluating a
/// hair past a pole lands on the sphere's far side, which is still on
/// the sphere: the widening can only ADD points to an enclosure.
fn full_latitude() -> (f64, f64) {
    (
        (-core::f64::consts::FRAC_PI_2).next_down(),
        core::f64::consts::FRAC_PI_2.next_up(),
    )
}

/// **Which carriers are admitted, and the argument for each window.**
///
/// A window must be a SUPERSET of the face's region in carrier
/// parameters, and the argument is the same in every arm: a coordinate
/// that is an affine function of position attains its extremes over a
/// compact region on that region's BOUNDARY, so the boundary's own
/// enclosure bounds it; a coordinate with an interior extremum, or a
/// periodic one no boundary bounds, takes its whole canonical range.
///
/// - **Plane**: `u` and `v` are arc-length coordinates along the stored
///   frame, both affine — the rectangle hull of the boundary's `(u, v)`
///   enclosure is the window.
/// - **Cylinder**: `v` is the axial coordinate, affine, and takes the
///   boundary's range; `u` is the azimuth and takes the whole turn.
/// - **Cone**: `v` is the slant arc length from the apex, affine along
///   each generator, so its extremes are on the boundary — and there
///   `|v| = ‖p − apex‖`, which is why the window is the symmetric range
///   around the apex rather than a signed one. That admits the mirror
///   nappe: loose, and sound, because it is a superset. `u` takes the
///   whole turn.
/// - **Sphere**: latitude has an interior extremum (a cap's boundary is
///   one circle while the region reaches the pole), so both coordinates
///   take their canonical ranges.
/// - **Torus**: both coordinates are periodic and no boundary bounds
///   either.
/// - **Everything else** — a NURBS patch, a fitted approximation, the
///   placeholder — refuses [`ClearanceRefusal::Unsupported`] naming the
///   class. Not because the evaluator would not run, but because a
///   free-form patch's knot domain is not a certified superset of a
///   TRIMMED region without the boundary in chart coordinates, and
///   inventing one is exactly the downgrade E7 forbids.
fn window_of(
    body: &Body<Interval>,
    at: RecipeNodeId,
    index: u32,
    face: FaceKey,
) -> Result<Window, ClearanceRefusal> {
    let unsupported = |carrier| Err(ClearanceRefusal::Unsupported { carrier, face });
    let Some(f) = body.get_face(face) else {
        return unsupported("an unreadable face");
    };
    let Some(surface) = body.get_surface(f.surface) else {
        return unsupported("an unreadable surface");
    };
    let Some((bounds, vertices)) = boundary_of(body, face) else {
        return unsupported("a face whose boundary carries no sound enclosure");
    };
    let extent = Point3::new(
        Interval::from_bounds(bounds.min_x, bounds.max_x),
        Interval::from_bounds(bounds.min_y, bounds.max_y),
        Interval::from_bounds(bounds.min_z, bounds.max_z),
    );
    let along = |origin: Point3<Interval>, dir: Vec3<Interval>| -> (f64, f64) {
        let d = (extent - origin).dot(dir);
        (d.lo(), d.hi())
    };
    let mut chart_axis = None;
    let charted = match surface {
        Surface::Plane { origin, normal, .. } => {
            let axis = in_plane_axis(*normal);
            let Some(u_ref) = chart_frame(*normal, axis) else {
                return unsupported("a plane whose interval normal admits no certified frame");
            };
            chart_axis = Some(axis);
            Surface::Plane {
                origin: *origin,
                normal: *normal,
                u_ref,
            }
        }
        Surface::Nurbs(_) => return unsupported("a free-form face"),
        Surface::Approx(_) => return unsupported("an approximated face"),
        other => other.clone(),
    };
    let (u, v) = match &charted {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => (along(*origin, *u_ref), along(*origin, normal.cross(*u_ref))),
        Surface::Cylinder { origin, axis, .. } => (full_turn(), along(*origin, *axis)),
        Surface::Cone { apex, .. } => {
            let reach = (extent - *apex).norm().hi();
            (full_turn(), (-reach, reach))
        }
        Surface::Sphere { .. } => (full_turn(), full_latitude()),
        Surface::Torus { .. } => (full_turn(), full_turn()),
        Surface::Nurbs(_) | Surface::Approx(_) => return unsupported("a free-form face"),
    };
    if !(u.0.is_finite() && u.1.is_finite() && v.0.is_finite() && v.1.is_finite()) {
        return unsupported("a face whose carrier window is not a finite rectangle");
    }
    if !refines(&charted, u, v) {
        return unsupported(
            "a face whose interval chart does not refine — halving its window leaves the \
             enclosure where it was, so no subdivision can decide anything about it",
        );
    }
    Ok(Window {
        at,
        body: index,
        face,
        chart_axis,
        surface: charted,
        u,
        v,
        vertices,
    })
}

/// **A certified in-plane direction, minted here rather than read off
/// the stored chart.**
///
/// The stored `u_ref` of a plane comes from the branchless orthonormal
/// basis, whose first step is `copysign(1, n.z)` — and at the interval
/// scalar a normal with `n.z` enclosing zero (every vertical wall of an
/// extruded prism) takes that function's zero-containing arm, so
/// `u_ref` comes back as a SIGN-HULLED enclosure. A chart on such a
/// frame does not refine: halving its `u` leaves the evaluated
/// enclosure exactly where it was, because the frame vector itself
/// spans both signs. Filed as
/// `work/issues/interval-orthonormal-basis-sign-hull.md`.
///
/// Re-charting is sound and is not a repair of the stored surface: a
/// plane's LOCUS does not depend on which orthonormal in-plane frame
/// names its points, and this module's window and enclosure are both
/// computed in whichever frame it returns. The stored surface is not
/// touched.
///
/// The axis is chosen by the widest cross product under `total_cmp` —
/// a chart choice, never a semantic one, in the same spirit as the
/// spatial index's split-axis rule: every choice yields a sound
/// superset, and the choice is a function of the enclosure's own bits,
/// so it is deterministic (D9). `None` when no candidate normalizes to
/// a certified direction.
fn in_plane_axis<T: Bounds>(normal: Vec3<T>) -> usize {
    let mut best = (f64::NEG_INFINITY, 0usize);
    for k in 0..3 {
        let lo = normal.cross(unit_axis::<T>(k)).norm().lo();
        if lo.total_cmp(&best.0) == core::cmp::Ordering::Greater {
            best = (lo, k);
        }
    }
    best.1
}

/// The `k`-th world axis at the caller's scalar.
///
/// The bound is SOLE `Bounds` at both callers (`Bounds: Real` carries
/// the arithmetic), which is the form the bounds gate is written for.
fn unit_axis<T: Real>(k: usize) -> Vec3<T> {
    let (zero, one) = (T::zero(), T::one());
    match k {
        0 => Vec3::new(one, zero, zero),
        1 => Vec3::new(zero, one, zero),
        _ => Vec3::new(zero, zero, one),
    }
}

/// The re-chart's `u_ref`: the normal crossed with [`in_plane_axis`]'s
/// choice, normalized. `None` when that does not come out finite, which
/// is the honest answer for a normal nothing can frame.
fn chart_frame<T: Bounds>(normal: Vec3<T>, axis: usize) -> Option<Vec3<T>> {
    let u = normal.cross(unit_axis::<T>(axis)).normalize();
    let finite = |x: T| x.lo().is_finite() && x.hi().is_finite();
    (finite(u.x) && finite(u.y) && finite(u.z)).then_some(u)
}

/// Whether halving the window on either axis actually narrows the
/// carrier's enclosure — the door that turns a chart the subdivision
/// could not refine into a typed refusal rather than a budget burn.
///
/// A single test on the widest halving is enough: if neither half of
/// either axis moves a bound, no descendant cell can either, because
/// interval enclosures shrink monotonically under sub-boxes.
fn refines(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64)) -> bool {
    let whole = cell_box(surface, u, v);
    let mid = |(lo, hi): (f64, f64)| 0.5 * (lo + hi);
    let half_u = cell_box(surface, (u.0, mid(u)), v);
    let half_v = cell_box(surface, u, (v.0, mid(v)));
    let narrower = |b: &Aabb| {
        b.max_x < whole.max_x
            || b.max_y < whole.max_y
            || b.max_z < whole.max_z
            || b.min_x > whole.min_x
            || b.min_y > whole.min_y
            || b.min_z > whole.min_z
    };
    narrower(&half_u) && narrower(&half_v)
}

/// The face's boundary enclosure and its vertices.
///
/// The enclosure is the hull of every boundary edge's carrier evaluated
/// over its OWN parameter interval — interval arithmetic does the
/// bounding, so a circular edge's bulge is enclosed rather than chorded
/// away — together with the boundary's vertex points. `None` for a face
/// whose boundary this cannot walk or one of whose edges carries no
/// certified curve: an unclaimable extent, never a silently smaller
/// one.
fn boundary_of(body: &Body<Interval>, face: FaceKey) -> Option<(Aabb, BTreeSet<VertexKey>)> {
    let f = body.get_face(face)?;
    let mut pts: Vec<Point3<Interval>> = Vec::new();
    let mut vertices = BTreeSet::new();
    let mut edges: BTreeSet<EdgeKey> = BTreeSet::new();
    for &lk in core::iter::once(&f.outer).chain(&f.rings) {
        let l = body.get_loop(lk)?;
        match l.boundary {
            LoopBoundary::Empty { vertex } => {
                vertices.insert(vertex);
                pts.push(*body.get_point(body.get_vertex(vertex)?.point)?);
            }
            LoopBoundary::Cycle { first } => {
                for he in body.loop_cycle(first)? {
                    let h = body.get_half_edge(he)?;
                    vertices.insert(h.start);
                    pts.push(*body.get_point(body.get_vertex(h.start)?.point)?);
                    edges.insert(h.edge);
                }
            }
        }
    }
    for ek in edges {
        let e = body.get_edge(ek)?;
        let curve = body.get_curve_geom(e.curve)?.certified()?;
        let (t0, t1) = curve.params();
        // The whole parameter span as ONE interval: the carrier's
        // interval evaluation over it encloses the entire arc, which is
        // what makes a curved boundary bound its own face's window.
        pts.push(
            curve
                .carrier()
                .eval(Interval::from_bounds(t0.lo(), t1.hi())),
        );
    }
    Aabb::from_points(pts).map(|b| (b, vertices))
}

// --------------------------------------------------- the subdivision

/// One cell of a window: a sub-rectangle of its carrier parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell {
    u: (f64, f64),
    v: (f64, f64),
}

impl Cell {
    fn of(w: &Window) -> Self {
        Self { u: w.u, v: w.v }
    }

    /// The relative width of each axis against the root window — the
    /// driver's split currency, so the two subdivisions choose axes by
    /// the same rule.
    fn relative(&self, root: &Window) -> (f64, f64) {
        let rel = |(lo, hi): (f64, f64), (rlo, rhi): (f64, f64)| {
            let span = rhi - rlo;
            if span > 0.0 { (hi - lo) / span } else { 0.0 }
        };
        (rel(self.u, root.u), rel(self.v, root.v))
    }
}

/// One cell pair on the frontier, and how many times it has been split.
#[derive(Debug, Clone, Copy)]
struct CellPair {
    a: Cell,
    b: Cell,
    depth: u32,
}

/// One cell's interval enclosure on its carrier.
fn enclosure(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64)) -> Point3<Interval> {
    surface.eval(
        Interval::from_bounds(u.0, u.1),
        Interval::from_bounds(v.0, v.1),
    )
}

/// The enclosure of a cell as a box — one interval surface evaluation,
/// which is already a box, folded through the crate's one bracket
/// reader.
fn cell_box(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64)) -> Aabb {
    Aabb::from_points([enclosure(surface, u, v)]).unwrap_or_else(Aabb::poison)
}

/// A box's diameter — the measured currency of [`DischargeWidths`].
/// Poison answers NaN, which the widths fold reports rather than hides.
fn diameter(b: &Aabb) -> f64 {
    Vec3::new(b.max_x - b.min_x, b.max_y - b.min_y, b.max_z - b.min_z).norm()
}

/// The separation enclosure between two cells, in metres: the interval
/// norm of the difference of the two enclosures.
///
/// Its `lo` is a lower bound on the CLOSEST approach between the two
/// cells and its `hi` an upper bound on the FURTHEST, so a definitely
/// negative `d − c` means every realized pair of points is inside the
/// bound — which is what makes the violation arm a claim about the
/// whole cell pair rather than about one point of it.
fn separation(a: &Point3<Interval>, b: &Point3<Interval>) -> Interval {
    let d = *a - *b;
    (d.x.powi(2) + d.y.powi(2) + d.z.powi(2)).sqrt()
}

/// The sweep's state: the question, the band, the dials, and the
/// deepest subdivision reached.
struct Sweep {
    bound: ClearanceBound,
    band: Band,
    config: ClearanceConfig,
    deepest: u32,
}

impl Sweep {
    /// The whole inner subdivision: candidate pairs from the interval
    /// BVH, then a binary subdivision per pair.
    fn run(
        mut self,
        doc: &Doc<ProfileProgram>,
        leaf: &ParamBox,
        wa: &[Window],
        wb: &[Window],
        same_body: bool,
        tol: Tol,
    ) -> ClearanceReport {
        let c = self.bound.c();
        // The interval BVH's ONLY job: which pairs need examining. Its
        // boxes enclose every real configuration in the leaf's box, so a
        // pair it excludes is separated by more than `c` throughout —
        // and the engine still decides every survivor at its own funnel
        // (no discharge rides a BVH test alone).
        let tree_a = Bvh::build(&wa.iter().map(Window::root_box).collect::<Vec<_>>());
        let tree_b = Bvh::build(&wb.iter().map(Window::root_box).collect::<Vec<_>>());
        let mut seen: BTreeSet<(FaceKey, FaceKey)> = BTreeSet::new();
        let candidates: Vec<(usize, usize)> = tree_a
            .pairs_within(&tree_b, c)
            .into_iter()
            .filter(|&(i, j)| {
                let (Some(x), Some(y)) = (wa.get(i), wb.get(j)) else {
                    return false;
                };
                if !same_body {
                    // Two faces of different bodies are never adjacent
                    // in the wedge sense — no vertex is shared — so the
                    // cross product stands as it is.
                    return true;
                }
                // The wedge rule: a face is never at a distance from
                // itself, and two faces sharing a vertex meet, so their
                // separation is legitimately zero and belongs to the
                // wedge predicates at that vertex. The unordered
                // de-duplication is the same-body case's own: two scopes
                // of one body can carry a pair in both orders.
                x.face != y.face
                    && x.vertices.is_disjoint(&y.vertices)
                    && seen.insert((x.face.min(y.face), x.face.max(y.face)))
            })
            .collect();

        let mut receipt = CellReceipt {
            candidates: candidates.len(),
            ..CellReceipt::default()
        };
        let mut widths = DischargeWidths::empty();
        let mut first_refusal: Option<ClearanceRefusal> = None;
        let mut violation_seed: Option<(usize, usize, Cell, Cell)> = None;

        for (index, &(i, j)) in candidates.iter().enumerate() {
            let (Some(x), Some(y)) = (wa.get(i), wb.get(j)) else {
                continue;
            };
            let remaining = candidates.len() - index - 1;
            let mut stack = vec![CellPair {
                a: Cell::of(x),
                b: Cell::of(y),
                depth: 0,
            }];
            while let Some(pair) = stack.pop() {
                self.deepest = self.deepest.max(pair.depth);
                let pa = enclosure(&x.surface, pair.a.u, pair.a.v);
                let pb = enclosure(&y.surface, pair.b.u, pair.b.v);
                let margin = separation(&pa, &pb) - Interval::from_f64(c);
                let discharge = |receipt: &mut CellReceipt, widths: &mut DischargeWidths| {
                    receipt.discharged += 1;
                    let da = diameter(&cell_box(&x.surface, pair.a.u, pair.a.v));
                    let db = diameter(&cell_box(&y.surface, pair.b.u, pair.b.v));
                    widths.discharged_at(da.max(db));
                };
                match decide(self.bound.predicate(), Margin::of(margin), self.band) {
                    Ok(Sign::Positive) => discharge(&mut receipt, &mut widths),
                    Ok(Sign::Zero) if self.bound.zero_discharges() => {
                        discharge(&mut receipt, &mut widths);
                    }
                    Ok(Sign::Zero | Sign::Negative) => {
                        receipt.violated += 1;
                        violation_seed.get_or_insert((i, j, pair.a, pair.b));
                    }
                    Err(source) => {
                        if let Some(predicate) = sliver(&source) {
                            receipt.refused += 1;
                            first_refusal.get_or_insert(ClearanceRefusal::Sliver { predicate });
                            continue;
                        }
                        if pair.depth >= self.config.max_cell_depth {
                            receipt.refused += 1;
                            first_refusal.get_or_insert(ClearanceRefusal::Budget(
                                CellBudget::Depth {
                                    max_cell_depth: self.config.max_cell_depth,
                                },
                            ));
                            continue;
                        }
                        // The budget is enforced AT ADMISSION, exactly
                        // as the driver's leaf budget is: a split turns
                        // one cell pair into two, so it is refused
                        // unless the leaf count it commits to still
                        // fits. What is already final, what is still on
                        // this pair's stack, what every later candidate
                        // owes (at least one leaf each), and the two
                        // halves this split would add.
                        let committed = receipt.discharged
                            + receipt.violated
                            + receipt.refused
                            + stack.len()
                            + remaining;
                        if committed + 2 > self.config.max_cell_pairs {
                            receipt.refused += 1;
                            first_refusal.get_or_insert(ClearanceRefusal::Budget(
                                CellBudget::Pairs {
                                    max_cell_pairs: self.config.max_cell_pairs,
                                },
                            ));
                            continue;
                        }
                        match split(pair, x, y) {
                            Some((lo, hi)) => {
                                receipt.splits += 1;
                                stack.push(hi);
                                stack.push(lo);
                            }
                            None => {
                                receipt.refused += 1;
                                first_refusal.get_or_insert(ClearanceRefusal::Budget(
                                    CellBudget::Resolution,
                                ));
                            }
                        }
                    }
                }
            }
        }
        widths.deepest = self.deepest;

        // The witness is minted from the FIRST violating cell pair in
        // the deterministic candidate order, and only then: a receipt
        // counts every interval-definite violation, a verdict carries
        // ONE, and the one it carries is the one the `f64` rebuild
        // confirmed.
        let mut violation = None;
        if let Some((i, j, ca, cb)) = violation_seed {
            match (wa.get(i), wb.get(j)) {
                (Some(x), Some(y)) => {
                    match verify_witness(doc, leaf, (x, ca, y, cb), self.bound, self.band, tol) {
                        Ok(w) => violation = Some(w),
                        Err(what) => {
                            first_refusal = Some(ClearanceRefusal::WitnessUnverified { what });
                        }
                    }
                }
                _ => {
                    first_refusal = Some(ClearanceRefusal::WitnessUnverified {
                        what: "the violating pair's windows are no longer addressable".to_owned(),
                    });
                }
            }
        }

        let verdict = match (violation, first_refusal) {
            (Some(geometry), _) => ClearanceVerdict::Violated(Box::new(Violation {
                param: witness_point(leaf),
                geometry,
            })),
            (None, Some(refusal)) => ClearanceVerdict::Refused(refusal),
            (None, None) => ClearanceVerdict::Holds,
        };
        ClearanceReport {
            verdict,
            receipt,
            widths,
        }
    }
}

/// **The split rule** (D9: fixed, total): split the cell whose
/// enclosure box is larger in diameter — ties to the first — on its
/// axis of greatest RELATIVE width against its own root window, ties to
/// `u`, bisected at the midpoint.
///
/// Relative width rather than absolute is what makes the rule
/// scale-free across two axes in different units (metres against
/// radians on a quadric), and it is the driver's own split currency
/// applied one level down.
///
/// `None` when the chosen axis cannot be bisected — its midpoint lands
/// on an endpoint, so the `f64` grid itself is the bound.
fn split(pair: CellPair, x: &Window, y: &Window) -> Option<(CellPair, CellPair)> {
    let da = diameter(&cell_box(&x.surface, pair.a.u, pair.a.v));
    let db = diameter(&cell_box(&y.surface, pair.b.u, pair.b.v));
    // `total_cmp` so the choice stays total on a poison diameter;
    // strictly-greater keeps the tie on the first cell.
    let split_b = db.total_cmp(&da) == core::cmp::Ordering::Greater;
    let (cell, root) = if split_b { (pair.b, y) } else { (pair.a, x) };
    let (ru, rv) = cell.relative(root);
    let axis_v = rv.total_cmp(&ru) == core::cmp::Ordering::Greater;
    let (lo, hi) = if axis_v { cell.v } else { cell.u };
    let mid = 0.5 * (lo + hi);
    if !(lo < mid && mid < hi) {
        return None;
    }
    let halves = |lo: f64, mid: f64, hi: f64| {
        if axis_v {
            (
                Cell {
                    u: cell.u,
                    v: (lo, mid),
                },
                Cell {
                    u: cell.u,
                    v: (mid, hi),
                },
            )
        } else {
            (
                Cell {
                    u: (lo, mid),
                    v: cell.v,
                },
                Cell {
                    u: (mid, hi),
                    v: cell.v,
                },
            )
        }
    };
    let (first, second) = halves(lo, mid, hi);
    let depth = pair.depth + 1;
    Some(if split_b {
        (
            CellPair {
                a: pair.a,
                b: first,
                depth,
            },
            CellPair {
                a: pair.a,
                b: second,
                depth,
            },
        )
    } else {
        (
            CellPair {
                a: first,
                b: pair.b,
                depth,
            },
            CellPair {
                a: second,
                b: pair.b,
                depth,
            },
        )
    })
}

/// The leaf's own midpoint, as offsets — the parameter witness.
///
/// [`BoxAxis::midpoint`] is the door, which is the same point the
/// driver's split rule cuts at and its K-telemetry replay samples: a
/// witness is a point the analysis actually stood on, not a new one
/// minted here.
fn witness_point(leaf: &ParamBox) -> ParamWitness {
    ParamWitness {
        offsets: leaf
            .axes()
            .iter()
            .map(|(name, axis)| (name.clone(), axis.midpoint()))
            .collect(),
    }
}

/// **The witness verification** (E7's witness clause): rebuild the
/// document at `f64` over the leaf's midpoint, evaluate both carriers
/// at the violating cells' midpoints, and put the distance through the
/// SAME funnel site at the `f64` lane.
///
/// Independent of the interval pass in the way that matters: a
/// different scalar, a different evaluation, its own decision. A
/// witness the rebuild does not classify as definitely violating is
/// never reported — the caller refuses
/// [`ClearanceRefusal::WitnessUnverified`] carrying what the rebuild
/// found.
///
/// # Errors
///
/// The prose of what the rebuild could not confirm.
fn verify_witness(
    doc: &Doc<ProfileProgram>,
    leaf: &ParamBox,
    at: (&Window, Cell, &Window, Cell),
    bound: ClearanceBound,
    band: Band,
    tol: Tol,
) -> Result<GeometryWitness, String> {
    let (x, ca, y, cb) = at;
    let mid: BTreeMap<ParamName, BoxAxis> = leaf
        .axes()
        .iter()
        .map(|(n, a)| {
            let m = a.midpoint();
            (n.clone(), BoxAxis::Varying { lo: m, hi: m })
        })
        .collect();
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::from_axes(mid))),
        ..lane_opts()
    };
    let ev: Evaluation<f64> = evaluate(doc, None, &CancelToken::new(), &opts, tol);
    // The SAME chart the interval pass subdivided in, rebuilt at `f64`
    // from the axis that pass chose: a witness's `(u, v)` are
    // coordinates in that chart, and reading them in the stored one
    // would name a different point.
    let surface_at = |w: &Window| -> Option<Surface<f64>> {
        let NodeResult::Ok(value) = ev.nodes.get(&w.at)? else {
            return None;
        };
        let body = crate::names::interrogate::output_body(&value.payload, w.body).ok()?;
        let f = body.get_face(w.face)?;
        let stored = body.get_surface(f.surface)?;
        match (stored, w.chart_axis) {
            (Surface::Plane { origin, normal, .. }, Some(axis)) => Some(Surface::Plane {
                origin: *origin,
                normal: *normal,
                u_ref: chart_frame(*normal, axis)?,
            }),
            _ => Some(stored.clone()),
        }
    };
    let (Some(sa), Some(sb)) = (surface_at(x), surface_at(y)) else {
        return Err(
            "the f64 rebuild does not carry both faces of the violating pair — the leaf's \
             key identity did not survive the replay"
                .to_owned(),
        );
    };
    let (au, av) = (mid_of(ca.u), mid_of(ca.v));
    let (bu, bv) = (mid_of(cb.u), mid_of(cb.v));
    let (pa, pb) = (sa.eval(au, av), sb.eval(bu, bv));
    let d = separation_f64(&pa, &pb);
    let definite = match decide(bound.predicate(), Margin::of(d - bound.c()), band) {
        Ok(Sign::Negative) => true,
        Ok(Sign::Zero) => !bound.zero_discharges(),
        _ => false,
    };
    if !definite {
        return Err(format!(
            "the f64 rebuild measures {d} between the witness points, which does not classify \
             as a violation of the bound {} at this run's tolerance",
            bound.c()
        ));
    }
    Ok(GeometryWitness {
        a: x.face,
        a_uv: (au, av),
        a_point: pa,
        b: y.face,
        b_uv: (bu, bv),
        b_point: pb,
        distance: d,
    })
}

/// A span's midpoint, through the analysis lane's one door so a witness
/// and a split cannot drift apart.
fn mid_of(span: (f64, f64)) -> f64 {
    BoxAxis::Varying {
        lo: span.0,
        hi: span.1,
    }
    .midpoint()
}

/// The `f64` distance between two points — the witness lane's own
/// recomputation, in the same association order the interval lane uses.
fn separation_f64(a: &Point3<f64>, b: &Point3<f64>) -> f64 {
    let d = *a - *b;
    (d.x.powi(2) + d.y.powi(2) + d.z.powi(2)).sqrt()
}
