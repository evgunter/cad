//! **The E7 clearance engine's document half**: the trichotomy over one
//! certified leaf of the E6 driver, and the same engine run at `c = 0⁺`
//! as the global parametric self-intersection check.
//!
//! Two nested subdivisions. The OUTER one is [`mod@crate::drive`]'s: it
//! hands this module a leaf whose topology is provably the witness
//! build's. This module never re-litigates that certification and never
//! touches the mass accounting except to consume it. The INNER one is
//! [`topo::clearance`]'s — a subdivision of the two faces' GEOMETRY
//! domains with interval exclusion. This module evaluates the document
//! with the leaf's parameters bound as intervals, resolves the two
//! selections, reads both bodies into the frame the question is asked
//! in, hands the engine their faces, and verifies at `f64` each witness
//! the engine offers. The windows, the cells, the sweep, its one-way
//! looseness and its two funnelled compares are the engine's, and its
//! module header states them.
//!
//! # What each verdict claims, exactly
//!
//! [`ClearanceVerdict::Holds`] is a certificate over the leaf's whole
//! parameter box AND the whole of both selections: at every parameter
//! point in the box and every pair of points on the two carrier
//! windows **minus the cells decided outside their faces**
//! ([`topo::cell_region`]), the separation satisfies the bound. That
//! set still CONTAINS both trimmed faces — a cell is dropped only when
//! no boundary of its face meets it, decided in metres at `K · ε`, and
//! the face's point door places a station of it outside — so the
//! certificate is about the faces, and the places neither face
//! occupies that it also covers are fewer. Nothing
//! samples, nothing averages, and no probability enters — mass
//! accounting applies to LEAVES (E6), never inside one.
//!
//! [`ClearanceVerdict::Violated`] carries a concrete parameter point
//! and a concrete pair of surface points, and their distance is
//! **re-verified at `f64`** by an independent rebuild before the
//! verdict is minted (`verify_witness`, the verifier this module hands
//! the engine). A witness the `f64` recomputation does not confirm is
//! never reported as one: the engine refuses
//! [`EngineRefusal::WitnessUnverified`] instead.
//!
//! What a witness IS, precisely: a pair of points, one on each FACE —
//! stations the faces' point doors place `In`, `OnEdge` or `OnVertex`
//! ([`topo::cell_region::station_region`]) — whose `f64` separation the
//! same funnel site calls a violation. It is the closest pair the rebuild's station lattice
//! FOUND, not the infimum over the two cells — on a flat pair the
//! lattice attains the true closest approach, on a curved one it
//! returns a near pair. For the strictly-positive question the witness
//! reports a COINCIDENCE and never a signed penetration depth; that
//! gap is scoped in `work/curved/signed-penetration-depth.md`.
//!
//! [`ClearanceVerdict::Refused`] is typed, in fourteen classes rather
//! than the spec's two. Nine are the engine's ([`EngineRefusal`],
//! carried as [`ClearanceRefusal::Engine`]): a terminal sliver (the
//! driver's rule shape — the deciding enclosure sits wholly inside the
//! funnel's band, so refinement provably cannot move it), a named
//! budget, a carrier with no admitted window, an enclosure that did not
//! evaluate, a scope that names nothing, a witness the `f64` rebuild
//! would not confirm, and the injectivity question's three — a touching
//! pair no local certificate discharged, a collapsed spline chart, a
//! torus not decided a ring. Five are this module's: a selection that
//! does not resolve, a fold over a drive that certified nothing, a bound
//! that is not a distance, a tolerance the funnel cannot band, and a
//! pair with a recorded contact. Each type carries the argument for
//! each of its arms; the suite's own module door says which have no
//! row and why.
//!
//! The minimum-separation question ([`topo::clearance::min_separation`])
//! is asked by the `min_clearance` measure from inside an evaluation,
//! where the body is already in hand, and this module does not stand
//! between them.
//!
//! # Read-only (E8)
//!
//! Every door takes `&Doc` and returns a value. There is no `&mut` on
//! the path and no re-witnessing: this lane cannot express a document
//! write, which is a stronger statement than one it does not perform.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom::Surface;
use geom_core::{Band, BandError, Point3, Sign, Tol};
use topo::Body;
use topo::clearance::{Ask, BoundVerdict, Side, Stations, Sweep};
use topo::entity::FaceKey;

use crate::analysis::{AnalyzedBox, BoxAxis, MeasureUnavailable, ParamBox};
use crate::doc::Doc;
use crate::drive::{CertifiedLeaf, MeasureAccounting, ParamBoxVerdict, lane_opts};
use crate::eval::{CancelToken, EvalOptions, Evaluation, NodeStanding, evaluate};
use crate::names::{EntityKey, EntityKind, EntityRef, Entry, StableName};
use crate::node::RecipeNodeId;
use crate::program::ProfileProgram;
use crate::var::VarId;

/// The engine's vocabulary, which this module's questions and reports
/// carry: the funnel sites, the budgets and the depth dial, the bound,
/// the receipt, the widths and the engine's refusals.
pub use topo::clearance::{
    Budgets, CLEARANCE_MARGIN, CellBudget, CellReceipt, CertificateArm, ClearanceBound,
    DEFAULT_MAX_CELL_DEPTH, DischargeWidths, EngineRefusal, MinSeparationRefusal,
    SELF_INTERSECTION_GAP, SharedCell, Which,
};

/// The whole-query cell-pair budget: how many cell pairs one clearance
/// query may examine before the rest of its frontier refuses
/// [`CellBudget::Pairs`].
///
/// ENFORCED AT ADMISSION, like the driver's leaf budget: a split that
/// would commit the query past this number is refused instead of taken.
/// A recorded run dial, and it is the DRIVER's number rather than a
/// second copy of it: a leaf-times-pairs run stays a thing a report can
/// hold only if the two budgets are the same order, and two constants
/// kept level by hand would not stay that way.
pub const DEFAULT_MAX_CELL_PAIRS: usize = crate::drive::DEFAULT_MAX_LEAVES;

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
    /// The sweep's budgets ([`DEFAULT_MAX_CELL_DEPTH`],
    /// [`DEFAULT_MAX_CELL_PAIRS`]).
    pub budgets: Budgets,
    /// Whether the monotonicity accelerator may restrict the leaf box
    /// to a facet ([`Pruning`]).
    pub pruning: Pruning,
}

impl Default for ClearanceConfig {
    fn default() -> Self {
        Self {
            budgets: Budgets {
                max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
                max_cell_pairs: DEFAULT_MAX_CELL_PAIRS,
            },
            pruning: Pruning::Off,
        }
    }
}

/// **The monotonicity accelerator's seam** (E7: duals accelerate,
/// never decide; E9: a degraded tangent forfeits exactly its uses).
///
/// An implementor answers, for one parameter of the leaf's box,
/// whether the separation is monotone in that parameter over the WHOLE
/// leaf — `Some(Positive)` for non-decreasing, `Some(Negative)` for
/// non-increasing, `Some(Zero)` for constant, and `None` for anything
/// it cannot certify.
///
/// **"The separation" is EVERY candidate pair's, not one.** A query
/// carries many face pairs and the answer is a minimum over all of
/// them, so a facet restriction is sound only if every pair's
/// separation is monotone in that parameter with the same sign. An
/// oracle that is right about one pair and silent about the rest must
/// answer `None`: a `Some` here is a claim about the whole query.
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
///
/// **What "the engine cannot check it" costs, stated as a limit.** The
/// suites contain a deliberately LYING oracle — one that claims
/// `Negative` on every axis — and on every fixture this unit can build
/// it is indistinguishable from the truthful [`NoTangents`]: same
/// verdict, same receipt. That is not the seam working, it is the seam
/// being untested, and the reason is the same one that keeps the
/// accelerator from buying anything today: at ε-scale parameter boxes
/// no parameter-driven width comes near a clearance margin, so
/// restricting a box to a facet cannot move an answer either way. A
/// wrong claim here is therefore silent on today's kernel, and will
/// stop being silent for exactly the fixtures that make the
/// accelerator worth having. Until then the contract above is a
/// promise the implementor keeps, not one this module enforces.
pub trait MonotoneOracle {
    /// The sign of `∂d/∂p` over the whole leaf, or `None`.
    fn monotone_in(&self, param: VarId) -> Option<Sign>;
}

/// The oracle that certifies nothing: E9's state, and the shipped
/// default until the tangent seed door lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NoTangents;

impl MonotoneOracle for NoTangents {
    fn monotone_in(&self, _param: VarId) -> Option<Sign> {
        None
    }
}

/// **How one clearance question is asked**: the bound, the run's
/// tolerance, the dials, the accelerator seam, the part resolver and
/// the tier the leaf was certified at.
///
/// One value rather than six arguments, because these travel together
/// everywhere — the per-leaf door, the driver-level fold and the
/// witness verification all take the same ones, and a caller that got
/// them out of step between two calls would be comparing two different
/// questions' answers.
pub struct ClearanceQuery<'a> {
    /// Which question ([`ClearanceBound`]). A document asks a bounded
    /// clearance as an assertion ([`crate::Claim::Clearance`]) through
    /// [`crate::drive::assertion_at`]; this door is the engine's, at its
    /// dials.
    pub bound: ClearanceBound,
    /// The run's tolerance — the linear band every compare classifies
    /// against.
    pub tol: Tol,
    /// The budgets and the accelerator switch.
    pub config: ClearanceConfig,
    /// The monotonicity seam. [`NoTangents`] forfeits every pruning,
    /// which is the state every verdict is defined against.
    pub oracle: &'a dyn MonotoneOracle,
    /// The seam a document's instantiated parts resolve through, as
    /// an evaluation over options carrying it would
    /// ([`EvalOptions::resolver`]); `None` for a document that
    /// instantiates none. An assembly's mates solve over the leaf's box
    /// at its scalar (`ASSEMBLY.md` A11 (5)), so a face frame's pose
    /// encloses over the box like any other geometry.
    pub resolver: Option<&'a Arc<dyn crate::part::PartResolver>>,
    /// **The tier the leaf was certified at**, which is the lane it is
    /// replayed on ([`crate::drive::ParamBoxVerdict::symbolic`]): a leaf
    /// the symbolic tier certified can carry a node a numeric replay
    /// refuses. [`clearance_over`] reads it off the verdict instead.
    pub symbolic: crate::drive::SymbolicDials,
}

impl ClearanceQuery<'_> {
    /// The self-intersection question at the shipped dials.
    pub fn strictly_positive(tol: Tol) -> Self {
        Self {
            bound: ClearanceBound::StrictlyPositive,
            tol,
            config: ClearanceConfig::default(),
            oracle: &NoTangents,
            resolver: None,
            symbolic: crate::drive::SymbolicDials::default(),
        }
    }
}

impl<'a> ClearanceQuery<'a> {
    /// The same question at other dials ([`ClearanceConfig`]).
    #[must_use]
    pub fn with_config(self, config: ClearanceConfig) -> Self {
        Self { config, ..self }
    }

    /// The same question with the accelerator's seam answered by
    /// `oracle` ([`MonotoneOracle`]).
    #[must_use]
    pub fn with_oracle(self, oracle: &'a dyn MonotoneOracle) -> Self {
        Self { oracle, ..self }
    }

    /// The same question over a document whose parts resolve through
    /// `resolver` ([`ClearanceQuery::resolver`]).
    #[must_use]
    pub fn resolved_by(self, resolver: &'a Arc<dyn crate::part::PartResolver>) -> Self {
        Self {
            resolver: Some(resolver),
            ..self
        }
    }

    /// The same question over a leaf certified at `symbolic`'s tier:
    /// a drive's own, [`crate::drive::ParamBoxVerdict::symbolic`].
    #[must_use]
    pub fn certified_at(self, symbolic: crate::drive::SymbolicDials) -> Self {
        Self { symbolic, ..self }
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
    /// Whether the scope names no face at all. A `Holds` over nothing
    /// is vacuously true and tells a caller nothing, so the door
    /// refuses it ([`EngineRefusal::EmptyScope`]).
    fn is_empty_scope(&self) -> bool {
        matches!(&self.faces, FaceScope::Named(names) if names.is_empty())
    }

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
    /// typed refusal, never a silently dropped face; an EMPTY list is
    /// [`EngineRefusal::EmptyScope`] at the door.
    ///
    /// **The resolved faces are sorted by arena key and de-duplicated**,
    /// so the authoring order of the names does not reach the answer and
    /// naming one face twice is naming it once. That is what makes the
    /// candidate order — and therefore which of several equally true
    /// violations is reported — a function of the body rather than of
    /// how the caller typed the query.
    Named(Vec<StableName>),
}

/// The E7 trichotomy: two holding arms, two violated arms, and the
/// typed third state.
#[derive(Debug, Clone, PartialEq)]
pub enum ClearanceVerdict {
    /// A bound with no upper end holds over the whole box and both
    /// carrier windows.
    Holds,
    /// A bound with an upper end holds: any lower end as [`Self::Holds`]
    /// says, and this pair, verified at `f64`, lies within the upper
    /// end, which proves it.
    Within(Box<Witness>),
    /// The lower bound is violated: this pair, verified at `f64`, is
    /// closer than it.
    Violated(Box<Witness>),
    /// The upper bound is violated: every pair of the two scopes is
    /// certified clear of it.
    Exceeded,
    /// Typed, never silence.
    Refused(ClearanceRefusal),
}

impl ClearanceVerdict {
    /// The verdict's state as a word, for reports.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Holds | Self::Within(_) => "Holds",
            Self::Violated(_) | Self::Exceeded => "Violated",
            Self::Refused(_) => "Refused",
        }
    }

    /// Does the bound hold? `None` when there is no verdict — the three
    /// states stay three at every reader, so nothing collapses a
    /// refusal into a silent pass ([`crate::measure::AssertionVerdict`]'s
    /// convention).
    pub fn holds(&self) -> Option<bool> {
        match self {
            Self::Holds | Self::Within(_) => Some(true),
            Self::Violated(_) | Self::Exceeded => Some(false),
            Self::Refused(_) => None,
        }
    }
}

/// A verified pair: where in the parameter box, and where on the two
/// faces.
#[derive(Debug, Clone, PartialEq)]
pub struct Witness {
    /// The parameter point, re-evaluated at `f64` to verify.
    pub param: ParamWitness,
    /// The closest-point pair the `f64` rebuild found.
    pub geometry: GeometryWitness,
}

impl Witness {
    /// The goldening lines of this pair, under `verdict`'s word.
    fn serialize(&self, verdict: &str, s: &mut String) {
        use core::fmt::Write as _;
        let g = &self.geometry;
        let _ = writeln!(
            s,
            "verdict {verdict} distance={:016x} faces=first:{:?}/second:{:?}",
            g.distance.to_bits(),
            g.a,
            g.b
        );
        let uv = |(u, v): (f64, f64)| format!("{:016x},{:016x}", u.to_bits(), v.to_bits());
        let pt = |p: Point3<f64>| {
            format!(
                "{:016x},{:016x},{:016x}",
                p.x.to_bits(),
                p.y.to_bits(),
                p.z.to_bits()
            )
        };
        let _ = writeln!(s, "witness a uv={} at={}", uv(g.a_uv), pt(g.a_point));
        let _ = writeln!(s, "witness b uv={} at={}", uv(g.b_uv), pt(g.b_point));
        for (name, offset) in &self.param.offsets {
            let _ = writeln!(s, "witness param {} {:016x}", name.full(), offset.to_bits());
        }
    }

    /// The pair in words, each face named by its side.
    fn render(&self, s: &mut String) {
        use core::fmt::Write as _;
        let g = &self.geometry;
        let _ = writeln!(
            s,
            "    {} m between face {:?} of the first scope and face {:?} of the second, \
             verified at f64",
            g.distance, g.a, g.b
        );
        for (name, offset) in &self.param.offsets {
            let _ = writeln!(s, "    at variable {name} = nominal {offset:+}");
        }
    }
}

/// A concrete parameter point inside the leaf.
///
/// The leaf's own midpoint, through [`BoxAxis::midpoint`] — the same
/// door the driver's split rule and its K-telemetry replay use, so a
/// witness is always a point some part of the analysis actually stood
/// on.
#[derive(Debug, Clone, PartialEq)]
pub struct ParamWitness {
    /// Per variable, the OFFSET from the document's nominal (the
    /// analysis lane's own currency — [`crate::analysis::AnalyzedParam`]),
    /// in the document's declaration order.
    pub offsets: Vec<(VarId, f64)>,
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
    /// Its carrier parameters, in the face's STORED chart — the one
    /// the carrier's own `u_ref` names, which a consumer rebuilds by
    /// reading the face's surface.
    ///
    /// On a periodic carrier `u` is an azimuth **on the walk's own
    /// branch**, a real number and never folded into `[0, τ)`: a face
    /// swept by a negative-angle revolve occupies `[θ, 0]` with `θ`
    /// negative, and a witness there reports a negative `u`. Fold it
    /// and it names a different point of the same surface.
    pub a_uv: (f64, f64),
    /// The point there.
    pub a_point: Point3<f64>,
    /// The second face.
    pub b: FaceKey,
    /// Its carrier parameters, in the second face's stored chart.
    pub b_uv: (f64, f64),
    /// The point there.
    pub b_point: Point3<f64>,
    /// The distance between the two points, at `f64`.
    ///
    /// **The closest pair FOUND, not the closest pair that exists.** It
    /// is the smallest distance over the two violating cells' nine-point
    /// lattices (see [`verify_witness`]), so it is a real configuration
    /// under the bound and an upper bound on the true closest approach
    /// within those cells — never a claim to be the minimum.
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
///
/// The engine's own refusals ride [`Self::Engine`]; the other five are
/// this door's, raised before the engine is asked or around it.
#[derive(Debug, Clone, PartialEq)]
pub enum ClearanceRefusal {
    /// The engine refused ([`EngineRefusal`]): a sliver, a budget, a
    /// carrier with no admitted window, an enclosure that did not
    /// evaluate, a scope naming no face, a witness the `f64` rebuild
    /// would not confirm, or one of the injectivity question's own.
    Engine {
        /// The engine's refusal.
        refusal: EngineRefusal,
        /// Each selection's node and output-body index: the engine's
        /// first side, then its second, which the refusal's words name
        /// a face of a pair by.
        sides: [(RecipeNodeId, u32); 2],
    },
    /// The selection itself could not be read.
    Selection(SelectionRefusal),
    /// The drive certified no leaf at all, so there is no leaf for the
    /// engine to answer over.
    ///
    /// The arm exists because the alternative is the state E7 forbids:
    /// a fold over an empty leaf list would return `Holds` — a pass, on
    /// a document about which nothing was proved. The drive's own
    /// accounting rides on the fold beside it, so a reader can price
    /// exactly how much of the box went unexamined.
    NothingCertified {
        /// How many leaves the drive refused instead.
        refused_leaves: usize,
    },
    /// The bound is not a distance: `c` is NaN, infinite, or negative.
    /// Refused at the door rather than subdivided against — a budget
    /// refusal after a full sweep would be the wrong name for it.
    NotADistance {
        /// The bound's offending end, as evaluated.
        c: f64,
    },
    /// **This build's scalar cannot certify.** A clearance is decided by
    /// the certified sweep, which reads enclosures; a point scalar has
    /// none, and a pair it found closest is an upper bound on the
    /// minimum, not a verdict. The requirement stays recorded and is
    /// answered at the certifying lane.
    NeedsCertifyingLane {
        /// The scalar this build ran at ([`geom_core::Real::NAME`]).
        scalar: &'static str,
    },
    /// The run's tolerance admits no linear band, so nothing here can be
    /// classified at all.
    ToleranceHasNoBand(
        /// Why: a threshold that overflowed, or a band that collapsed.
        BandError,
    ),
    /// The injectivity question over a body that carries a contact
    /// record between two cells of the scope: the record says the two
    /// cells' interiors meet, so the boundary is not injective there by
    /// declaration. Refused until the hand-off to CONTACT-DESIGN's
    /// records is designed, naming the record's two cells.
    RecordedContact {
        /// The record's first cell.
        a: topo::Cell,
        /// Its second.
        b: topo::Cell,
    },
}

impl ClearanceRefusal {
    /// The refusal's own payload, rendered for the goldening form — the
    /// half `name` drops, so two runs that refuse for the same CLASS on
    /// different evidence do not serialize alike. A machine channel: a
    /// node id prints in full ([`SelectionRefusal::payload`]).
    pub fn payload(&self) -> String {
        match self {
            Self::Engine { refusal, .. } => refusal.payload(),
            Self::Selection(r) => r.payload(),
            Self::NothingCertified { refused_leaves } => format!("refused_leaves={refused_leaves}"),
            Self::NotADistance { c } => format!("{:016x}", c.to_bits()),
            Self::NeedsCertifyingLane { scalar } => (*scalar).to_owned(),
            Self::ToleranceHasNoBand(error) => format!("{error:?}"),
            Self::RecordedContact { a, b } => format!("{a:?}/{b:?}"),
        }
    }

    /// The refusal's stable class name, for reports and the goldening
    /// form: an engine refusal's own.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Engine { refusal, .. } => refusal.name(),
            Self::Selection(_) => "selection",
            Self::NothingCertified { .. } => "nothing_certified",
            Self::NotADistance { .. } => "not_a_distance",
            Self::NeedsCertifyingLane { .. } => "needs_certifying_lane",
            Self::ToleranceHasNoBand(_) => "tolerance_has_no_band",
            Self::RecordedContact { .. } => "recorded_contact",
        }
    }
}

// It holds ids, said by whichever speaker renders it; an engine
// refusal's pair is named by the selection each face is of.
impl crate::spoken::Say for ClearanceRefusal {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Engine {
                refusal,
                sides: [(first, i), (second, j)],
            } => refusal.say(
                f,
                [
                    &format_args!("body {i} of {}", by.node(*first)),
                    &format_args!("body {j} of {}", by.node(*second)),
                ],
            ),
            Self::Selection(r) => r.say(f, by),
            Self::NothingCertified { refused_leaves } => write!(
                f,
                "the drive certified no leaf ({refused_leaves} refused), so there is no leaf to \
                 answer the clearance over"
            ),
            Self::NotADistance { c } => write!(
                f,
                "the bound {c} is not a distance: a clearance bound is finite and non-negative. \
                 Recourse: write the bound as a non-negative length"
            ),
            Self::NeedsCertifyingLane { scalar } => write!(
                f,
                "a clearance is decided by the certified sweep, which the {scalar} scalar cannot \
                 run: a point search finds a near pair, not a verdict. Recourse: certify it at \
                 the {interval} lane",
                interval = <geom_core::Interval as geom_core::Real>::NAME,
            ),
            Self::ToleranceHasNoBand(error) => {
                write!(f, "the run's tolerance admits no linear band: {error}")
            }
            Self::RecordedContact { a, b } => write!(
                f,
                "the body records a contact between {a:?} and {b:?}, so its boundary meets \
                 itself there by declaration and the injectivity question is not asked"
            ),
        }
    }
}

/// The refusal where no document is at hand: each node by its tag.
impl core::fmt::Display for ClearanceRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl core::error::Error for ClearanceRefusal {}

/// How a selection could not be read.
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionRefusal {
    /// The node did not build in the leaf's replay.
    NodeDidNotBuild(
        /// The node's standing in that replay.
        NodeStanding,
    ),
    /// The node's payload carries bodies, but none at that index.
    NoSuchBody {
        /// The node.
        node: RecipeNodeId,
        /// The index asked for.
        index: u32,
    },
    /// The node's payload carries no bodies at all, so no index of it
    /// is one.
    NoBodies {
        /// The node.
        node: RecipeNodeId,
        /// The payload family.
        payload: &'static str,
    },
    /// No row of the node's name table answers to a name in the scope.
    Unresolved {
        /// The name.
        name: Box<StableName>,
    },
    /// A name in the scope denotes another kind of entity than a face.
    NotAFace {
        /// The name.
        name: Box<StableName>,
        /// The kind it denotes.
        found: EntityKind,
    },
    /// A face name in the scope is an N2 tie: several faces answer to
    /// it, and a tie is never broken by picking.
    Tied {
        /// The name.
        name: Box<StableName>,
        /// The faces that answer to it, each body by the node's output
        /// index, the index a [`Selection`] names it by.
        candidates: Vec<EntityRef>,
    },
    /// A name in the scope denotes a face of another output body of
    /// the node than the selected one.
    OtherBody {
        /// The name.
        name: Box<StableName>,
        /// The node's output body its face is on, as a [`Selection`]
        /// names it.
        body: u32,
    },
    /// A face name whose table row holds a key of another kind: the
    /// name table seated a row against its name's kind, which every
    /// door that seats one refuses. A kernel defect.
    KeyOfAnotherKind {
        /// The name.
        name: Box<StableName>,
        /// The kind of the key its row holds.
        found: EntityKind,
    },
    /// The two selections live in different spaces (D10, Spaces), and
    /// nothing in one is compared with the other.
    AcrossSpaces {
        /// One selection's space.
        a: crate::VarId,
        /// The other's.
        b: crate::VarId,
    },
    /// The second selection's body could not be read into the first's
    /// frame, the frame the question is asked in (D10
    /// `[placement.frame]`).
    Frame {
        /// The second selection's node.
        node: RecipeNodeId,
        /// The refusal, as an operation reading the same operand would
        /// raise it.
        refusal: crate::NodeRefusal,
    },
}

// It holds ids, said by whichever speaker renders it.
impl crate::spoken::Say for SelectionRefusal {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::NodeDidNotBuild(standing) => write!(
                f,
                "the selection has no faces to measure a clearance between in this leaf's \
                 replay: {}",
                crate::spoken::Said(standing, by)
            ),
            Self::NoSuchBody { node, index } => {
                write!(
                    f,
                    "{}'s value carries no body at index {index}",
                    by.node(*node)
                )
            }
            Self::NoBodies { node, payload } => write!(
                f,
                "{}'s value {}, so there is no face to measure a clearance between. {}",
                by.node(*node),
                crate::sentence::CarriesNoBodies(payload),
                crate::sentence::Recourse("select a node whose value is a body")
            ),
            Self::Unresolved { name } => write!(
                f,
                "nothing in the selected node's name table answers to {}",
                by.name(name)
            ),
            Self::NotAFace { name, found } => write!(
                f,
                "{} denotes {} {}, and a clearance is measured between faces",
                by.name(name),
                found.article(),
                found.noun()
            ),
            Self::Tied { name, candidates } => {
                write!(
                    f,
                    "{} faces answer to {} equally well (",
                    candidates.len(),
                    by.name(name)
                )?;
                for (n, c) in candidates.iter().enumerate() {
                    let sep = if n == 0 { "" } else { ", " };
                    match c.key.face() {
                        Some(k) => write!(f, "{sep}face {k:?} of body {}", c.body)?,
                        None => write!(f, "{sep}{:?} of body {}", c.key, c.body)?,
                    }
                }
                write!(
                    f,
                    "), and a tie is never broken by picking. {}",
                    crate::sentence::Recourse(crate::sentence::REFINE_THE_TIE)
                )
            }
            Self::OtherBody { name, body } => write!(
                f,
                "{} denotes a face of body {body}, not of the selected body",
                by.name(name)
            ),
            Self::KeyOfAnotherKind { name, found } => write!(
                f,
                "{} names a face, and its row in the name table holds {} {} — the table seated a \
                 row against its name's kind. {}",
                by.name(name),
                found.article(),
                found.noun(),
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::AcrossSpaces { a, b } => write!(
                f,
                "the two selections live in different spaces, {} and {}, and {}. {}",
                by.space(*a),
                by.space(*b),
                crate::sentence::ONLY_A_PLACEMENT_RELATES_TWO_SPACES,
                crate::sentence::Recourse(crate::sentence::place_into_one_space("measure within"))
            ),
            Self::Frame { node, refusal } => write!(
                f,
                "{}'s body could not be read into the frame the clearance is asked in: {}",
                by.node(*node),
                refusal.kind()
            ),
        }
    }
}

/// The refusal where no document is at hand: each node by its tag.
impl core::fmt::Display for SelectionRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl SelectionRefusal {
    /// Why `sel`'s node has no body at its index, as
    /// `interrogate::output_body` answered: an index its value does not
    /// have, or a value with no bodies at all.
    fn no_body(sel: &Selection, error: crate::names::InterrogateError) -> Self {
        use crate::names::InterrogateError as E;
        match error {
            E::NoBodies { payload } => Self::NoBodies {
                node: sel.at,
                payload,
            },
            E::NoSuchBody { .. } => Self::NoSuchBody {
                node: sel.at,
                index: sel.body,
            },
            other => unreachable!("`output_body` refuses only a missing body, not {other:?}"),
        }
    }

    /// **The refusal for the goldening form**: its class word and its
    /// fields, each node id in full ([`RecipeNodeId::full`]) — a machine
    /// channel, where two ids must never print alike.
    pub fn payload(&self) -> String {
        match self {
            Self::NodeDidNotBuild(standing) => {
                let (word, node, through) = match *standing {
                    NodeStanding::NotEvaluated { node } => ("not_evaluated", node, None),
                    NodeStanding::NotInDocument { node } => ("not_in_document", node, None),
                    NodeStanding::Failed { node } => ("failed", node, None),
                    NodeStanding::Poisoned { node, through } => ("poisoned", node, Some(through)),
                };
                let mut out = format!("node_did_not_build {word} node={}", node.full());
                if let Some(through) = through {
                    out.push_str(&format!(" through={}", through.full()));
                }
                out
            }
            Self::NoSuchBody { node, index } => {
                format!("no_such_body node={} index={index}", node.full())
            }
            Self::NoBodies { node, payload } => {
                format!("no_bodies node={} payload={payload}", node.full())
            }
            Self::Unresolved { name } => format!("unresolved {name:?}"),
            Self::NotAFace { name, found } => format!("not_a_face {name:?} found={found:?}"),
            Self::Tied { name, candidates } => format!("tied {name:?} candidates={candidates:?}"),
            Self::OtherBody { name, body } => format!("other_body {name:?} body={body}"),
            Self::KeyOfAnotherKind { name, found } => {
                format!("key_of_another_kind {name:?} found={found:?}")
            }
            Self::AcrossSpaces { a, b } => {
                let space = |s: &crate::VarId| match *s {
                    crate::VarId::WORLD => "world".to_owned(),
                    s => format!("space={}", s.full()),
                };
                format!("across_spaces {} {}", space(a), space(b))
            }
            Self::Frame { node, refusal } => {
                format!("frame node={} {:?}", node.full(), refusal.kind())
            }
        }
    }
}

impl core::error::Error for SelectionRefusal {}

/// What one clearance query answers: the verdict, its receipt, and the
/// measured limit.
#[derive(Debug, Clone, PartialEq)]
pub struct ClearanceReport {
    verdict: ClearanceVerdict,
    receipt: CellReceipt,
    widths: DischargeWidths,
    decided: usize,
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

    /// **How many readings of the faces' regions the answer rests on**:
    /// distinct (window, cell) pairs [`topo::cell_region`] read as
    /// `Inside` or `Outside`, whichever partner they were paired with,
    /// and distinct witness stations a face's point door placed
    /// ([`topo::cell_region::station_region`]). Zero says the faces'
    /// regions were never read into the answer.
    pub fn decided(&self) -> usize {
        self.decided
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
            "receipt candidates={} discharged={} violated={} within={} refused={} splits={} \
             abandoned={} outside={} certified={} by_kind={} holds={}",
            r.candidates,
            r.discharged,
            r.violated,
            r.within,
            r.refused,
            r.splits,
            r.abandoned,
            r.outside,
            r.certified,
            r.by_kind,
            r.holds()
        );
        match &self.verdict {
            ClearanceVerdict::Holds => {
                let _ = writeln!(s, "verdict holds");
            }
            ClearanceVerdict::Within(w) => w.serialize("within", &mut s),
            ClearanceVerdict::Violated(w) => w.serialize("violated", &mut s),
            ClearanceVerdict::Exceeded => {
                let _ = writeln!(s, "verdict exceeded");
            }
            ClearanceVerdict::Refused(r) => {
                let _ = writeln!(s, "verdict refused {} {}", r.name(), r.payload());
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
        let _ = writeln!(s, "regions decided={}", self.decided);
        s
    }

    /// **The report's content key** — the identity of everything
    /// [`Self::serialize`] renders.
    ///
    /// Added in M10-6's fix pass (R2's MINOR-10): §2 claims every
    /// derived report in this lane carries the two doors AND a key,
    /// and claim 3 names `ClearanceReport` among them, but this type
    /// shipped with `serialize`/`render` and no key at all. Tag
    /// `0xEE`, appended.
    pub fn content_key(&self) -> crate::eval::ContentKey {
        crate::eval::key_of(0xEE, &self.serialize())
    }

    /// **The human form** (M10-6 §2): the verdict in words, the
    /// witness as numbers rather than as bits, and the measured
    /// discharge widths that say what the certificate cost.
    pub fn render(&self) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        match &self.verdict {
            ClearanceVerdict::Holds => {
                let _ = writeln!(s, "HOLDS over the whole box and both carrier windows");
            }
            ClearanceVerdict::Within(w) => {
                let _ = writeln!(s, "HOLDS: a pair lies within the upper bound");
                w.render(&mut s);
            }
            ClearanceVerdict::Violated(w) => {
                let _ = writeln!(s, "VIOLATED: a pair is closer than the lower bound");
                w.render(&mut s);
            }
            ClearanceVerdict::Exceeded => {
                let _ = writeln!(
                    s,
                    "VIOLATED: every pair is certified clear of the upper bound"
                );
            }
            ClearanceVerdict::Refused(r) => {
                let _ = writeln!(s, "REFUSED ({}): {r}", r.name());
            }
        }
        let r = self.receipt;
        let _ = writeln!(
            s,
            "  {} candidate pair(s); {} discharged ({} of them vacuously, off the face, {} by \
             a local certificate, {} by kind), {} violated, {} within, {} refused, {} split, \
             {} abandoned",
            r.candidates,
            r.discharged,
            r.outside,
            r.certified,
            r.by_kind,
            r.violated,
            r.within,
            r.refused,
            r.splits,
            r.abandoned
        );
        let _ = writeln!(
            s,
            "  {} reading(s) of the faces' regions decided (cells and witness stations)",
            self.decided
        );
        let w = self.widths;
        let _ = writeln!(
            s,
            "  discharged at cell diameters {} … {} (deepest subdivision {})",
            w.widest
                .map_or_else(|| "none".to_owned(), |x| format!("{x}")),
            w.narrowest
                .map_or_else(|| "none".to_owned(), |x| format!("{x}")),
            w.deepest
        );
        s
    }

    /// `verdict`, with the sweep's receipt, widths and region count.
    pub(crate) fn of<W>(
        verdict: ClearanceVerdict,
        report: &topo::clearance::BoundReport<W>,
    ) -> Self {
        Self {
            verdict,
            receipt: report.receipt,
            widths: report.widths,
            decided: report.decided,
        }
    }

    /// A report that never got as far as a subdivision.
    pub(crate) fn refused(refusal: ClearanceRefusal) -> Self {
        Self {
            verdict: ClearanceVerdict::Refused(refusal),
            receipt: CellReceipt::default(),
            widths: DischargeWidths::empty(),
            decided: 0,
        }
    }
}

// ------------------------------------------------------------- doors

/// **Global self-intersection freedom over one certified leaf** (E7's
/// census made global and parametric): the selection's boundary is
/// injective, ERROR-DESIGN's *Self-intersection scope*.
///
/// Every pair of the selection's faces is examined, a face against
/// itself included. A face on an analytic carrier is injective by its
/// kind; two cells that share no point of the boundary are certified at
/// strictly positive distance; two that do are certified by a local
/// certificate or refuse typed, naming the two faces, the shared cell
/// and the arm ([`topo::clearance::Sweep::run`]).
pub fn self_intersection(
    doc: &Doc<ProfileProgram>,
    leaf: &ParamBox,
    of: &Selection,
    tol: Tol,
) -> ClearanceReport {
    clearance_with(doc, leaf, of, of, &ClearanceQuery::strictly_positive(tol))
}

/// The full door: the strict question between two selections, both
/// budgets, and the accelerator seam.
///
/// `a` and `b` may name the same selection: a query against itself
/// pairs each face with itself and each later face of the same body,
/// and the strict bound over one body is the injectivity question.
/// Between two bodies it is metric.
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
    clearance_on(doc, leaf, a, b, query, query.symbolic.lane())
}

/// [`clearance_with`] with the leaf replayed on `lane`.
fn clearance_on(
    doc: &Doc<ProfileProgram>,
    leaf: &ParamBox,
    a: &Selection,
    b: &Selection,
    query: &ClearanceQuery<'_>,
    lane: crate::eval::LeafLane,
) -> ClearanceReport {
    for (sel, side) in [(a, Which::First), (b, Which::Second)] {
        if sel.is_empty_scope() {
            return ClearanceReport::refused(ClearanceRefusal::Engine {
                refusal: EngineRefusal::EmptyScope { side },
                sides: [(a.at, a.body), (b.at, b.body)],
            });
        }
    }
    let band = match Band::linear(query.tol) {
        Ok(band) => band,
        Err(error) => return ClearanceReport::refused(ClearanceRefusal::ToleranceHasNoBand(error)),
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
        resolver: query.resolver.cloned(),
        ..lane_opts()
    };
    let replayed = Replayed {
        doc,
        queried: &queried,
        a,
        b,
        query,
        band,
    };
    crate::eval::read_leaf_with(doc, &opts, lane, query.tol, replayed)
}

/// The clearance question over one leaf's replay, at the replay's
/// scalar.
struct Replayed<'q> {
    doc: &'q Doc<ProfileProgram>,
    queried: &'q ParamBox,
    a: &'q Selection,
    b: &'q Selection,
    query: &'q ClearanceQuery<'q>,
    band: Band,
}

impl crate::eval::AtLeaf for Replayed<'_> {
    type Out = ClearanceReport;

    fn at<T: geom_core::Decide + topo::AtRestPolicy>(
        self,
        ev: &Evaluation<T>,
        engine: topo::ClearanceLane<T>,
    ) -> ClearanceReport {
        let Self {
            doc,
            queried,
            a,
            b,
            query,
            band,
        } = self;
        if let Some((a, b)) = doc.across_spaces(a.at, b.at) {
            return ClearanceReport::refused(ClearanceRefusal::Selection(
                SelectionRefusal::AcrossSpaces { a, b },
            ));
        }
        // The two selections are in one space: the question is asked in
        // the frame `[placement.frame]` conditions on
        // (`Framed::computing_among`: the selection in its space's
        // coordinates, else the first), each body read into it through
        // `relative`.
        let (framed_a, framed_b) = match (framed_of(ev, a), framed_of(ev, b)) {
            (Ok(fa), Ok(fb)) => (fa, fb),
            (Err(r), _) | (_, Err(r)) => return ClearanceReport::refused(r),
        };
        let computing = crate::eval::Framed::computing_among([framed_a, framed_b])
            .unwrap_or_else(|| unreachable!("two operands"));
        let read = |framed: &crate::eval::Framed<T>, node: RecipeNodeId| {
            computing.read_in(framed, query.tol).map_err(|kind| {
                ClearanceRefusal::Selection(SelectionRefusal::Frame {
                    node,
                    refusal: crate::NodeRefusal::from(kind),
                })
            })
        };
        let (body_a, body_b) = match (read(framed_a, a.at), read(framed_b, b.at)) {
            (Ok(ba), Ok(bb)) => (ba, bb),
            (Err(r), _) | (_, Err(r)) => return ClearanceReport::refused(r),
        };
        let refused = |refusal| ClearanceRefusal::Engine {
            refusal,
            sides: [(a.at, a.body), (b.at, b.body)],
        };
        // Each selection's names resolve, then its faces are read into
        // windows, the first selection whole before the second.
        let faces_a = match faces_of(ev, a, &body_a) {
            Ok(faces) => faces,
            Err(r) => return ClearanceReport::refused(r),
        };
        let windows_a = match engine.windows(
            &Side {
                body: &body_a,
                faces: faces_a.clone(),
            },
            Which::First,
        ) {
            Ok(w) => w,
            Err(r) => return ClearanceReport::refused(refused(r)),
        };
        let faces_b = if a == b {
            faces_a.clone()
        } else {
            match faces_of(ev, b, &body_b) {
                Ok(faces) => faces,
                Err(r) => return ClearanceReport::refused(r),
            }
        };
        // The SAME-BODY rule, not the same-selection one: two scopes of
        // one body can name one face twice, and a face is never at a
        // distance from itself.
        let same_body = a.at == b.at && a.body == b.body;
        if same_body
            && matches!(query.bound, ClearanceBound::StrictlyPositive)
            && let Some((x, y)) = recorded_contact(ev, a, &body_a, &faces_a, &faces_b)
        {
            return ClearanceReport::refused(ClearanceRefusal::RecordedContact { a: x, b: y });
        }
        let windows_b = if a == b {
            windows_a.clone()
        } else {
            match engine.windows(
                &Side {
                    body: &body_b,
                    faces: faces_b,
                },
                Which::Second,
            ) {
                Ok(w) => w,
                Err(r) => return ClearanceReport::refused(refused(r)),
            }
        };

        let sweep = Sweep {
            bound: query.bound,
            band,
            budgets: query.config.budgets,
        };
        let report = engine.sweep(&sweep, &windows_a, &windows_b, same_body, |candidate| {
            let faces = candidate.faces();
            let ask = candidate.ask();
            verify_witness(
                doc,
                queried,
                (a, b),
                (faces, || engine.on_face_stations(candidate)),
                (ask, band),
                query.resolver,
                query.tol,
            )
        });
        let verdict = verdict_of(report.verdict, || witness_point(queried), refused);
        ClearanceReport {
            verdict,
            receipt: report.receipt,
            widths: report.widths,
            decided: report.decided,
        }
    }
}

/// **The driver-level fold**: the clearance question over EVERY
/// certified leaf of a drive, with the leaves' own mass re-priced by
/// what the clearance query said about them.
///
/// The engine's unit of answer is the leaf, and this door does not
/// invent a leaf-crossing claim: it runs the query per certified leaf,
/// combines the verdicts in the same fixed order one query combines its
/// pairs in, sums the receipts (a sum of forests is a forest), and
/// keeps every leaf's own verdict so a consumer can see WHICH leaves
/// answered what.
///
/// **A fold over zero certified leaves REFUSES.** The drive may certify
/// nothing at all — a real ±0.05 tolerance box does exactly that today,
/// every leaf `Budget`-refused — and a fold that started at `Holds` and
/// was never moved would report a pass over a document about which
/// nothing was proved. That is the fourth state E7's trichotomy exists
/// to forbid, and [`ClearanceVerdict::holds`]'s own contract forbids it
/// twice.
///
/// **A leaf the query refused is not certified mass.** The drive priced
/// it certified because the DRIVE certified it; this certificate did
/// not. [`ClearanceMass`] moves it out, by class, so the sentence a
/// report writes has an honest denominator.
///
/// # Errors
///
/// Nothing: the refusals are verdicts, not errors. The mass columns
/// carry [`MeasureUnavailable`] where a parameter's law cannot price a
/// leaf, exactly as the drive's own accounting does.
pub fn clearance_over(
    doc: &Doc<ProfileProgram>,
    analyzed: &AnalyzedBox,
    verdict: &ParamBoxVerdict,
    a: &Selection,
    b: &Selection,
    query: &ClearanceQuery<'_>,
) -> LeafFold {
    let mut fold = LeafFold {
        verdict: ClearanceVerdict::Holds,
        receipt: CellReceipt::default(),
        widths: DischargeWidths::empty(),
        decided: 0,
        leaves: Vec::new(),
        mass: ClearanceMass::empty(verdict.accounting()),
        drive_accounting: verdict.accounting().clone(),
    };
    if verdict.certified().is_empty() {
        fold.verdict = ClearanceVerdict::Refused(ClearanceRefusal::NothingCertified {
            refused_leaves: verdict.refused().len(),
        });
        return fold;
    }
    // On the drive's own lane, whatever the query's `symbolic` says.
    let lane = verdict.lane();
    for leaf in verdict.certified() {
        let CertifiedLeaf { box_, .. } = leaf;
        let report = clearance_on(doc, box_, a, b, query, lane);
        let mass = box_.mass(analyzed);
        fold.mass.price(&report.verdict, mass);
        fold.receipt = fold.receipt.plus(report.receipt);
        fold.widths = fold.widths.plus(report.widths);
        fold.decided += report.decided;
        fold.verdict = combine(fold.verdict, report.verdict.clone());
        fold.leaves.push(LeafAnswer {
            box_: box_.clone(),
            verdict: report.verdict,
        });
    }
    fold
}

/// One certified leaf's own answer, kept so a consumer can see which
/// leaves held and which did not.
#[derive(Debug, Clone, PartialEq)]
pub struct LeafAnswer {
    /// The leaf's parameter box.
    pub box_: ParamBox,
    /// What the clearance query said about it.
    pub verdict: ClearanceVerdict,
}

/// **Where the certified mass went once the clearance question was
/// asked of it** (E7: refusals "typed and priced by measure").
///
/// The drive's own accounting answers a different question — which
/// leaves it could certify — and a fold that handed it through as if it
/// answered this one would price a leaf whose clearance query refused
/// as covered. These four columns are unconditional masses under the
/// product measure, in the drive's own currency, and they sum to the
/// drive's certified column.
#[derive(Debug, Clone, PartialEq)]
pub struct ClearanceMass {
    /// Mass on certified leaves whose clearance verdict is `Holds`.
    pub holds: Result<f64, MeasureUnavailable>,
    /// Mass on certified leaves carrying a verified violation.
    pub violated: Result<f64, MeasureUnavailable>,
    /// Mass on certified leaves the clearance query refused, by refusal
    /// class ([`ClearanceRefusal::name`]).
    pub refused: BTreeMap<&'static str, Result<f64, MeasureUnavailable>>,
    /// The mass the DRIVE never certified — its refused leaves plus the
    /// tail outside the analyzed box, verbatim from
    /// [`MeasureAccounting::unresolved`]. This certificate says nothing
    /// about it either.
    pub uncertified_by_the_drive: Result<f64, MeasureUnavailable>,
}

impl ClearanceMass {
    fn empty(accounting: &MeasureAccounting) -> Self {
        Self {
            holds: Ok(0.0),
            violated: Ok(0.0),
            refused: BTreeMap::new(),
            uncertified_by_the_drive: accounting.unresolved(),
        }
    }

    /// Folds one leaf's mass into the column its verdict names.
    fn price(&mut self, verdict: &ClearanceVerdict, mass: Result<f64, MeasureUnavailable>) {
        let column = match verdict {
            ClearanceVerdict::Holds | ClearanceVerdict::Within(_) => &mut self.holds,
            ClearanceVerdict::Violated(_) | ClearanceVerdict::Exceeded => &mut self.violated,
            ClearanceVerdict::Refused(r) => self.refused.entry(r.name()).or_insert(Ok(0.0)),
        };
        // The FIRST refusal wins the column, exactly as the drive's own
        // accounting does: a column that cannot be priced names the
        // parameter that stopped it, and a later priceable leaf does not
        // un-refuse it.
        if let Ok(acc) = column {
            match mass {
                Ok(v) => *acc += v,
                Err(e) => *column = Err(e),
            }
        }
    }

    /// **The share of the whole measure this certificate does NOT
    /// cover**: everything but the leaves that held.
    ///
    /// # Errors
    ///
    /// The first [`MeasureUnavailable`] among the columns it sums.
    pub fn unresolved(&self) -> Result<f64, MeasureUnavailable> {
        let mut out = self.violated.clone()?;
        for m in self.refused.values() {
            out += m.clone()?;
        }
        Ok(out + self.uncertified_by_the_drive.clone()?)
    }
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
    /// The summed decided-cell counts
    /// ([`ClearanceReport::decided`]) over every leaf.
    pub decided: usize,
    /// Every certified leaf's own answer, in the drive's order.
    pub leaves: Vec<LeafAnswer>,
    /// Where the certified mass went once this question was asked.
    pub mass: ClearanceMass,
    /// The drive's own accounting, verbatim — what the DRIVE could and
    /// could not certify, which is a different question from this one
    /// and is kept beside it rather than merged into it.
    pub drive_accounting: MeasureAccounting,
}

impl LeafFold {
    /// **The goldening form** of a whole fold (M10-6 §2): the combined
    /// verdict, the summed receipt, and where the certified mass went,
    /// every float as its exact bits.
    ///
    /// Per-leaf verdicts ride the fold and are NOT serialized: a drive
    /// of tens of thousands of leaves would produce a golden nobody
    /// re-blesses, and the per-leaf answers are already reachable
    /// through [`LeafFold::leaves`]. What is goldened is the ANSWER and
    /// its accounting.
    pub fn serialize(&self) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        let _ = writeln!(s, "fold leaves={}", self.leaves.len());
        let _ = write!(
            s,
            "{}",
            ClearanceReport {
                verdict: self.verdict.clone(),
                receipt: self.receipt,
                widths: self.widths,
                decided: self.decided,
            }
            .serialize()
        );
        let m = &self.mass;
        let _ = writeln!(s, "mass holds {}", crate::report::mass_bits(&m.holds));
        let _ = writeln!(s, "mass violated {}", crate::report::mass_bits(&m.violated));
        for (class, v) in &m.refused {
            let _ = writeln!(s, "mass refused {class} {}", crate::report::mass_bits(v));
        }
        let _ = writeln!(
            s,
            "mass uncertified_by_the_drive {}",
            crate::report::mass_bits(&m.uncertified_by_the_drive)
        );
        s
    }

    /// The fold's content key — the identity of everything
    /// [`Self::serialize`] renders.
    pub fn content_key(&self) -> crate::eval::ContentKey {
        crate::eval::key_of(0xED, &self.serialize())
    }

    /// **The human form**: the verdict, then the four mass columns as
    /// percentages with the drive's uncertified share on its own line,
    /// so no reader takes "holds" for "holds everywhere".
    pub fn render(&self) -> String {
        use core::fmt::Write as _;
        let mut s = ClearanceReport {
            verdict: self.verdict.clone(),
            receipt: self.receipt,
            widths: self.widths,
            decided: self.decided,
        }
        .render();
        let m = &self.mass;
        let _ = writeln!(s, "  over {} certified leaf/leaves:", self.leaves.len());
        let _ = writeln!(
            s,
            "    {} of the measure holds",
            crate::report::percent(&m.holds)
        );
        let _ = writeln!(s, "    {} violated", crate::report::percent(&m.violated));
        for (class, v) in &m.refused {
            let _ = writeln!(s, "    {} refused ({class})", crate::report::percent(v));
        }
        let _ = writeln!(
            s,
            "    {} the DRIVE never certified — this certificate says nothing about it",
            crate::report::percent(&m.uncertified_by_the_drive)
        );
        s
    }
}

/// The fixed combination order: a definite violation outranks a
/// refusal, which outranks `Holds`. Within a rank the FIRST is kept, so
/// the answer is a function of the deterministic pair order.
fn combine(acc: ClearanceVerdict, next: ClearanceVerdict) -> ClearanceVerdict {
    match (&acc, &next) {
        (ClearanceVerdict::Violated(_) | ClearanceVerdict::Exceeded, _) => acc,
        (_, ClearanceVerdict::Violated(_) | ClearanceVerdict::Exceeded) => next,
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
    ParamBox::from_axes_in(
        box_.axes()
            .iter()
            .map(|(name, axis)| {
                let (lo, hi) = axis.span();
                let collapsed = match (axis, oracle.monotone_in(*name)) {
                    (BoxAxis::Varying { .. }, Some(Sign::Positive | Sign::Zero)) => {
                        Some(BoxAxis::Varying { lo, hi: lo })
                    }
                    (BoxAxis::Varying { .. }, Some(Sign::Negative)) => {
                        Some(BoxAxis::Varying { lo: hi, hi })
                    }
                    _ => None,
                };
                (*name, collapsed.unwrap_or(*axis))
            })
            .collect(),
        box_.order(),
    )
}

/// **The body a selection reads**, out of the leaf's replay, in its own
/// frame.
fn framed_of<'e, T: geom_core::Decide>(
    ev: &'e Evaluation<T>,
    sel: &Selection,
) -> Result<&'e crate::eval::Framed<T>, ClearanceRefusal> {
    let refuse = ClearanceRefusal::Selection;
    let value = ev
        .usable(sel.at)
        .map_err(|standing| refuse(SelectionRefusal::NodeDidNotBuild(standing)))?;
    crate::names::interrogate::output_body(&value.payload, sel.body)
        .map_err(|e| refuse(SelectionRefusal::no_body(sel, e)))
}

/// The faces a selection names in `body`, the selection's body in the
/// frame the question is asked in.
fn faces_of<T: geom_core::Decide>(
    ev: &Evaluation<T>,
    sel: &Selection,
    body: &Body<T>,
) -> Result<Vec<FaceKey>, ClearanceRefusal> {
    let refuse = ClearanceRefusal::Selection;
    Ok(match &sel.faces {
        // Arena order — the deterministic order every derived list in
        // this kernel inherits (D9).
        FaceScope::All => body.faces().map(|(k, _)| k).collect(),
        FaceScope::Named(names) => {
            let mut out = Vec::new();
            // What the name denotes before how many answer to it: a
            // name of another kind is not a face however narrowly it is
            // referenced, and every candidate of a tie carries the
            // name's kind.
            for name in names {
                let named = || Box::new(name.clone());
                let found = ev
                    .body_read_of(sel.at, sel.body)
                    .and_then(|(var, index)| Some((ev.table(var)?.lookup(name)?, index)));
                let Some((entry, index)) = found else {
                    return Err(refuse(SelectionRefusal::Unresolved { name: named() }));
                };
                if name.kind != EntityKind::Face {
                    return Err(refuse(SelectionRefusal::NotAFace {
                        name: named(),
                        found: name.kind,
                    }));
                }
                // A row's body is its table's index: a split's half reads
                // its one body as 0. A refusal says the selection's.
                let output = |body: u32| if body == index { sel.body } else { body };
                let ent = match entry {
                    Entry::Unique(ent) => ent,
                    Entry::Tied(candidates) => {
                        return Err(refuse(SelectionRefusal::Tied {
                            name: named(),
                            candidates: candidates
                                .iter()
                                .map(|c| EntityRef {
                                    body: output(c.body),
                                    key: c.key,
                                })
                                .collect(),
                        }));
                    }
                };
                match ent.key {
                    EntityKey::Face(k) if ent.body == index => out.push(k),
                    EntityKey::Face(_) => {
                        return Err(refuse(SelectionRefusal::OtherBody {
                            name: named(),
                            body: output(ent.body),
                        }));
                    }
                    other => {
                        debug_assert!(
                            false,
                            "a face name's row holds {other:?}: every `NameTable` door that \
                             seats a row admits it only at its name's kind"
                        );
                        return Err(refuse(SelectionRefusal::KeyOfAnotherKind {
                            name: named(),
                            found: other.kind(),
                        }));
                    }
                }
            }
            out.sort_unstable();
            out.dedup();
            out
        }
    })
}

/// **A contact record between the two scopes**: the first record of the
/// selection's body whose one cell bounds a face of one scope and whose
/// other bounds a face of the other. The records are the selected
/// body's, as its value's carry holds them: a family member's or a
/// split side's own ([`crate::Carry::member`]), else the one body's.
fn recorded_contact<T: geom_core::Decide>(
    ev: &Evaluation<T>,
    sel: &Selection,
    body: &Body<T>,
    fa: &[FaceKey],
    fb: &[FaceKey],
) -> Option<(topo::Cell, topo::Cell)> {
    let value = ev.value(sel.at)?;
    let carry = match &value.carry.members {
        Some(_) => value.carry.member(usize::try_from(sel.body).ok()?),
        None if sel.body == 0 => &value.carry,
        None => return None,
    };
    let records = Arc::clone(&carry.contacts);
    let loop_face = |he| {
        body.get_half_edge(he)
            .and_then(|h| body.get_loop(h.parent_loop))
            .map(|l| l.face)
    };
    let faces = |c: topo::Cell| -> Vec<FaceKey> {
        match c {
            topo::Cell::Face(f) => vec![f],
            topo::Cell::Edge(e) => body
                .get_edge(e)
                .map(|e| {
                    [e.he_plus, e.he_minus]
                        .into_iter()
                        .filter_map(loop_face)
                        .collect()
                })
                .unwrap_or_default(),
            topo::Cell::Vertex(v) => body
                .half_edges()
                .filter(|(_, h)| h.start == v)
                .filter_map(|(he, _)| loop_face(he))
                .collect(),
        }
    };
    let meets = |c: topo::Cell, scope: &[FaceKey]| faces(c).iter().any(|f| scope.contains(f));
    records
        .cell_pairs()
        .find(|&(x, y)| (meets(x, fa) && meets(y, fb)) || (meets(x, fb) && meets(y, fa)))
}

/// The leaf's own midpoint, as offsets — the parameter witness.
///
/// [`BoxAxis::midpoint`] is the door, which is the same point the
/// driver's split rule cuts at and its K-telemetry replay samples: a
/// witness is a point the analysis actually stood on, not a new one
/// minted here.
pub(crate) fn witness_point(leaf: &ParamBox) -> ParamWitness {
    ParamWitness {
        offsets: leaf
            .order()
            .iter()
            .filter_map(|&name| Some((name, leaf.get(name)?.midpoint())))
            .collect(),
    }
}

/// **The box collapsed to its midpoint** ([`witness_point`]'s point), as
/// a box an `f64` rebuild evaluates over.
pub(crate) fn midpoint_of(leaf: &ParamBox) -> ParamBox {
    let mid: BTreeMap<VarId, BoxAxis> = leaf
        .axes()
        .iter()
        .map(|(&n, a)| {
            let m = a.midpoint();
            (n, BoxAxis::Varying { lo: m, hi: m })
        })
        .collect();
    ParamBox::from_axes_in(mid, leaf.order())
}

/// **The witness verification** (E7's witness clause), the verifier the
/// engine's sweep asks about each candidate violation: rebuild the
/// document at `f64` over the leaf's midpoint, search the two violating
/// cells' on-face stations ([`Candidate::on_face_stations`](topo::clearance::Candidate::on_face_stations)) for the
/// closest pair of points it can find, and put THAT distance through
/// the SAME funnel site at the `f64` lane.
///
/// It is a SEARCH, not a solve: every pair it offers is inside the cell
/// pair the interval pass proved violating, so any of them verifies,
/// and taking the closest is what makes the REPORT worth reading. The
/// true closest-point pair on two trimmed patches is a different unit's
/// problem, and [`GeometryWitness::distance`] says so at the field.
///
/// Independent of the interval pass in the way that matters: a
/// different scalar, a different evaluation, its own decision. A
/// witness the rebuild does not classify as definitely violating is
/// never reported — the engine refuses
/// [`EngineRefusal::WitnessUnverified`] carrying what the rebuild
/// found.
///
/// # Errors
///
/// The prose of what the rebuild could not confirm.
fn verify_witness(
    doc: &Doc<ProfileProgram>,
    leaf: &ParamBox,
    (sel_a, sel_b): (&Selection, &Selection),
    ((face_a, face_b), stations): ((FaceKey, FaceKey), impl FnOnce() -> (Stations, Stations)),
    (ask, band): (Ask, Band),
    resolver: Option<&Arc<dyn crate::part::PartResolver>>,
    tol: Tol,
) -> Result<GeometryWitness, String> {
    let opts = EvalOptions {
        param_box: Some(Arc::new(midpoint_of(leaf))),
        resolver: resolver.cloned(),
        ..lane_opts()
    };
    let ev: Evaluation<f64> = evaluate(doc, None, &CancelToken::new(), &opts, tol);
    // The SAME chart the interval pass subdivided in: the stored one.
    // A witness's `(u, v)` are coordinates in that chart, and the `f64`
    // replay of the same node mints the same frame, so reading them
    // needs nothing carried across from the interval pass.
    let framed_at = |sel: &Selection| -> Result<&crate::eval::Framed<f64>, String> {
        let value = ev.usable(sel.at).map_err(|standing| {
            format!("the f64 rebuild has no value for a face of the violating pair: {standing}")
        })?;
        crate::names::interrogate::output_body(&value.payload, sel.body).map_err(|e| {
            format!(
                "the f64 rebuild has no body for a face of the violating pair: {}",
                SelectionRefusal::no_body(sel, e)
            )
        })
    };
    let (fx, fy) = (framed_at(sel_a)?, framed_at(sel_b)?);
    // In the frame the interval pass asked it in: the same structural
    // choice over the same two operands (`Framed::computing_among`).
    let computing = crate::eval::Framed::computing_among([fx, fy])
        .unwrap_or_else(|| unreachable!("two operands"));
    let (bx, by) = match (computing.read_in(fx, tol), computing.read_in(fy, tol)) {
        (Ok(bx), Ok(by)) => (bx, by),
        (Err(kind), _) | (_, Err(kind)) => {
            return Err(format!(
                "the f64 rebuild could not read a face of the violating pair into the frame the \
                 pair is decided in: {}",
                crate::NodeRefusal::from(kind)
            ));
        }
    };
    closest_pair((&bx, &by), (face_a, face_b), stations(), (ask, band))
}

/// **The verifier's core**: the closest pair of the two faces' admitted
/// stations on the `f64` rebuild's own charts, confirmed against `ask`
/// ([`Ask::confirms`]).
///
/// **The closest pair on the cells' own lattice**, not their midpoints:
/// both are inside the cells the interval pass classified, so either
/// confirms; the midpoint pair is just a much worse REPORT. Chosen by
/// the smallest `f64` distance under `total_cmp`. **Only a station the
/// face's point door places on the face is a candidate**
/// ([`topo::clearance::Candidate::on_face_stations`]), so a witness is a
/// pair of points of the two FACES, not of their carrier windows.
///
/// # Errors
///
/// The prose of what the rebuild could not confirm.
pub(crate) fn closest_pair(
    (bx, by): (&Body<f64>, &Body<f64>),
    (face_a, face_b): (FaceKey, FaceKey),
    (lattice_a, lattice_b): (Stations, Stations),
    (ask, band): (Ask, Band),
) -> Result<GeometryWitness, String> {
    let surface_at = |body: &Body<f64>, face: FaceKey| -> Option<Surface<f64>> {
        let f = body.get_face(face)?;
        Some(body.get_surface(f.surface)?.clone())
    };
    let (Some(sa), Some(sb)) = (surface_at(bx, face_a), surface_at(by, face_b)) else {
        return Err(
            "the f64 rebuild does not carry both faces of the pair — the key identity did not \
             survive the rebuild"
                .to_owned(),
        );
    };
    let mut best: Option<Station> = None;
    for &(au, av) in &lattice_a {
        let pa = sa.eval(au, av);
        for &(bu, bv) in &lattice_b {
            let pb = sb.eval(bu, bv);
            let d = separation_f64(&pa, &pb);
            if best
                .as_ref()
                .is_none_or(|b| d.total_cmp(&b.d) == core::cmp::Ordering::Less)
            {
                best = Some(Station {
                    d,
                    a_uv: (au, av),
                    b_uv: (bu, bv),
                    a_point: pa,
                    b_point: pb,
                });
            }
        }
    }
    let Some(Station {
        d,
        a_uv,
        b_uv,
        a_point: pa,
        b_point: pb,
    }) = best
    else {
        return Err("the cells carry no station on their faces to verify at".to_owned());
    };
    ask.confirms(<geom_core::Interval as geom_core::Real>::from_f64(d), band)
        .map_err(|why| {
            format!("the f64 rebuild measures {d} between the closest stations: {why}")
        })?;
    Ok(GeometryWitness {
        a: face_a,
        a_uv,
        a_point: pa,
        b: face_b,
        b_uv,
        b_point: pb,
        distance: d,
    })
}

/// **The sweep's verdict as the document reads it**, each verified pair
/// stood at the parameter point `param` names and each engine refusal
/// carried by `refused` with its sides.
pub(crate) fn verdict_of(
    verdict: BoundVerdict<GeometryWitness>,
    param: impl Fn() -> ParamWitness,
    refused: impl FnOnce(EngineRefusal) -> ClearanceRefusal,
) -> ClearanceVerdict {
    let at = |geometry| {
        Box::new(Witness {
            param: param(),
            geometry,
        })
    };
    match verdict {
        BoundVerdict::Holds => ClearanceVerdict::Holds,
        BoundVerdict::Within(geometry) => ClearanceVerdict::Within(at(geometry)),
        BoundVerdict::Violated(geometry) => ClearanceVerdict::Violated(at(geometry)),
        BoundVerdict::Exceeded => ClearanceVerdict::Exceeded,
        BoundVerdict::Refused(r) => ClearanceVerdict::Refused(refused(r)),
    }
}

/// One candidate of [`verify_witness`]'s lattice search: a pair of
/// stations, one on each cell, and the `f64` distance between them.
///
/// Named rather than left a five-tuple so the comparison that picks the
/// smallest reads as the field it compares.
struct Station {
    /// The `f64` distance between the two points.
    d: f64,
    /// The first face's carrier parameters.
    a_uv: (f64, f64),
    /// The second face's.
    b_uv: (f64, f64),
    /// The point at `a_uv`.
    a_point: Point3<f64>,
    /// The point at `b_uv`.
    b_point: Point3<f64>,
}

/// The `f64` distance between two points — the witness lane's own
/// recomputation, in the same association order the interval lane uses.
fn separation_f64(a: &Point3<f64>, b: &Point3<f64>) -> f64 {
    let d = *a - *b;
    (d.x.powi(2) + d.y.powi(2) + d.z.powi(2)).sqrt()
}
