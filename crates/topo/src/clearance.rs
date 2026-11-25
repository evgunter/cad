//! **The E7 clearance engine's body-level half**: the inner subdivision
//! of two sets of faces' GEOMETRY domains, which answers a separation
//! bounded below, above or both as a trichotomy ([`Sweep`]), the two
//! bounds classified together in one subdivision. "What IS the minimum
//! separation" is a certified bracket ([`min_separation`]) no
//! evaluation asks: a clearance is a requirement, decided at its bounds.
//!
//! It reads a body, the proximity tree and the carriers, and nothing
//! else. Which document, node or parameter box a body came from is the
//! caller's: a [`Side`] carries a body and the faces in scope, no node
//! address, and a caller says itself whether the two sides are one body
//! ([`Sweep::run`]'s `same_body`). A side's body is read as it stands:
//! where the two sides are in two frames, each window's carrier is
//! mapped into the question's frame at the reading ([`Side::windows_at`],
//! `[placement.frame]`), and which part of a face a cell holds is read
//! off the face in its own coordinates.
//!
//! The door is generic over the lane scalar and the arithmetic is not.
//! Every surface, curve and point the door reads is lifted through
//! [`Interval::from_certified`], and cells, enclosures, the funnelled
//! `d − c` and the bracket are interval arithmetic; at `T = Interval`
//! the lift keeps every bound, and a face any of whose lifted values
//! does not certify is refused (`window_of`). On a window that is a superset of its
//! face, the cells' and stations' point-in-face decisions run at `T`
//! ([`crate::cell_region`]); a window that IS its face (an iso-bounded
//! NURBS patch) asks neither door, since every cell and station of it
//! is on the face. Two faces' depths are read at `T` too, off their
//! covers (`parted_in_depth`).
//!
//! # Two questions, one subdivision
//!
//! One walk, and the difference is what it does with a cell pair (the
//! cut differs too: the injective arm bisects by metric axes and splits
//! a cell against itself in three): the bound question DECIDES each
//! pair at a funnel site; the minimum question takes minima of
//! enclosure endpoints. Both drop the cells decided outside their faces.
//! Each entry point takes its readings in [`k_stats::unlogged`]'s scope
//! itself (audit row F41): a cell reading answers the question, never a
//! caller's log, so a caller keeps no scope of its own.
//!
//! **The sweep stops once every bound it asks is ANSWERED**: at the
//! first VERIFIED violation of a lower bound, or, for an upper bound
//! alone, at its first VERIFIED proof (a band runs on until its lower
//! bound is cleared or broken too). A cell pair the interval pass
//! classifies as answering is handed to the caller's verifier
//! ([`Candidate`]), and a verified pair is the deliverable: continuing
//! past one buys a bigger receipt and no more truth. What was on the
//! frontier when it stopped is counted [`CellReceipt::abandoned`] —
//! never folded into `refused`, which would be a claim that it was
//! tried. A candidate the verifier does not confirm is
//! [`EngineRefusal::WitnessUnverified`] (or, for an upper bound the box
//! decides in part only, [`EngineRefusal::BoxDependent`]).
//!
//! **What a refusal does to the walk is the question's.** The
//! injectivity question ends at its first FINDING, a verified crossing
//! or a refusal, and reports that one: the refusal names the two faces
//! of the cell pair it was made on, and the rest of the frontier is
//! abandoned as it is at a crossing. A metric bound keeps the first
//! refusal of each bound and reads on, so a definite violation still
//! outranks a pair met earlier that the band or a budget stopped.
//! Which finding is first is fixed by the walk's order
//! ([`Sweep::run`]).
//!
//! # The one place a verdict is loose, stated at the door
//!
//! The inner subdivision runs over each face's **carrier window** — the
//! rectangle of surface parameters derived in `window_of`. A window
//! is a conservative SUPERSET of the face's trimmed region, and every
//! CELL of it [`crate::cell_region`] decides [`CellRegion::Outside`] is
//! discharged vacuously rather than classified. The door reads the
//! face's edges on their own carriers in model space and its point
//! door, on every carrier kind this engine admits, so the same rule
//! holds on a plane, a cylinder, a cone, a sphere and a torus. A cell
//! straddling the boundary, within `K · ε` of it, or one whose station
//! the point door does not place, is kept: the set the sweep covers
//! still contains the face. A NURBS face's window is exact rather than
//! a superset (an approximated face's is its fit's, the fit being its
//! geometry): such a window is admitted only where the face is the whole patch
//! (`iso_bounded_domain`), so its cells read `Inside` and its stations
//! are admitted with neither door asked (`Region::Whole`).
//!
//! So the looseness runs one way, and it is the safe way for a defect
//! gate. `Holds` covers strictly more than the faces and is therefore
//! sound about them. A violation's candidate offers the verifier only
//! stations the faces' point doors place on the faces
//! ([`Candidate::on_face_stations`]), so a witness is a pair of face
//! points. What a `Holds` still covers beyond the faces is the cells the
//! door cannot decide: a band around each boundary that shrinks with the
//! cells, and the cells of a face whose point door does not read its
//! region. [`BoundReport::decided`] counts the cells and witness
//! stations the doors decided.
//!
//! # No ε here, and the funnelled compares
//!
//! Every decision goes through the one `k_stats` funnel at a named
//! predicate with a [`Margin`] door. The distance test decides at
//! three: [`CLEARANCE_MARGIN`] (`d − c`, metres minus metres) and
//! [`SELF_INTERSECTION_GAP`] (`d`, the separation a cell pair that does
//! not touch in M must classify strictly positive) on the cells' boxes,
//! and, where the box margin is indeterminate, [`CLEARANCE_SUPPORT_GAP`]
//! on the gap between the cells' supports along one direction, and at a
//! face pair's root [`CLEARANCE_DEPTH_GAP`] on the gap between the two
//! faces' depths about an axis, a centre or a core circle
//! (`parted_in_depth`): two lower bounds of the separation, which only
//! discharge. Under the injectivity question a TOUCHING pair is
//! decided instead by a local certificate ([`crate::injective`]), at
//! that module's own sites. The
//! cell and station verdicts the sweep asks for decide on
//! [`crate::cell_region`]'s rows, inside those doors. All carry a row
//! in `docs/predicate-dimension-audit.md`. No DECISION here compares a
//! float raw, mints a tolerance, or reads a width as evidence.
//!
//! Three raw compares survive, and none is a decision. The first is
//! the proximity tree's admission test, which decides which pairs are EXAMINED
//! rather than what any of them means. It is padded by the funnel's
//! own escalate threshold (`c + band.escalate()`), so a pair it drops
//! is one the funnel would have discharged definitely — the two agree
//! in the only direction that matters, and everything inside the band
//! reaches the funnel. The second is the window door's [`refines`],
//! whose bound compares decide which charts are ADMITTED to the sweep,
//! never what a pair means: a chart admitted wrongly costs depth or
//! budget, which the sweep refuses typed. The third is
//! [`support_separation`]'s choice of direction, which picks which true
//! lower bound is decided and decides nothing itself. Stated because a
//! reviewer should meet them here rather than find them.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::ControlFlow;

use bvh::{Aabb, Bvh};
use geom::cover::Cover;
use geom::{NurbsSurface, Surface};
use geom_brep::{Channel, Pcurve, SpiricImage, WallTorus};
use geom_core::interval::Interval;
use geom_core::k_stats::{self, decide};
use geom_core::{
    Band, Bounds, CertifiedBounds, Decide, Margin, Point2, Point3, Real, RigidMotion, Sign, Vec3,
};

use crate::Body;
use crate::FaceContainment;
use crate::cell_region::{
    Ball, BoundaryStop, CellRegion, cell_region_lifted, centre, lift_boundary, station_region,
};
use crate::census::depth_gap;
use crate::entity::{EdgeKey, EntityId, Face, FaceKey, HalfEdgeKey, VertexKey};
use crate::injective::{
    self, Axis, Boundary, Iso, Local, Near, Outcome, Piece, Rim, Shared, Use, full_turn,
};
use crate::live::{BoundaryMember, FaceView, proven};
use crate::validate::definitely_positive;
use lift::{Lift, certified_lift};

/// The funnel site name of the clearance comparison — the separation
/// enclosure between two domain cells minus the requested clearance,
/// in metres against the linear band.
///
/// A roster carrier (`docs/K-REPORT.md`) rather than a literal at the
/// decide site.
pub const CLEARANCE_MARGIN: &str = "clearance_margin";

/// The funnel site name of the self-intersection comparison — the
/// separation enclosure itself, which a cell pair that does not touch
/// in M must classify strictly positive.
///
/// This is E7's "certified strictly positive distance" spelled as a
/// funnelled compare rather than as an ε: the band's own coincidence
/// threshold is what "strictly" means, so `Sign::Zero` — two cells
/// that do not touch, coincident at the run's tolerance — is a
/// violation here, where under [`CLEARANCE_MARGIN`]'s non-strict `≥ c`
/// it is a discharge. Same margin shape, same band, two readings of
/// zero, one per site.
pub const SELF_INTERSECTION_GAP: &str = "self_intersection_gap";

/// The funnel site name of the support comparison — a lower bound on
/// the separation between two domain cells, read off their supports
/// along one direction, minus the requested clearance (`0` for the
/// strict question), in metres against the linear band. Only a
/// [`Sign::Positive`] is read here, and it discharges the pair under
/// either bound: the comparand bounds the separation from below only,
/// so a `Zero` here is no coincidence and no violation.
pub const CLEARANCE_SUPPORT_GAP: &str = "clearance_support_gap";

/// The funnel site name of the depth comparison — a lower bound on the
/// separation between two faces, read off their depths about one site
/// (the gap of [`crate::census::depth_gap`], less its pads), minus the
/// requested clearance (`0` for the strict question), in metres against
/// the linear band. Read as [`CLEARANCE_SUPPORT_GAP`] is: only a
/// [`Sign::Positive`], which discharges the pair's root under either
/// bound. Its own name, so the faces' depth margins and the cells'
/// support margins are two populations.
pub const CLEARANCE_DEPTH_GAP: &str = "clearance_depth_gap";

/// How deep ONE cell pair may be subdivided before the engine refuses
/// the rest of its subtree rather than refining it further.
///
/// A recorded run dial (E6's "run config like K"), not a constant of
/// nature. 40 halvings take a domain axis to `2^-40` of its window —
/// twelve decimal digits, past the point where a carrier's own
/// evaluation carries information at `f64`.
pub const DEFAULT_MAX_CELL_DEPTH: u32 = 40;

/// **How deep a touching pair read at wedge 0 or 2π splits before it
/// refuses**, naming the second-order arm. Above it the pair splits like
/// any uncertified one, so the cells it holds away from the edge are
/// still decided — the walk is breadth-first, and a violation there is
/// reported first; past it the cells at the edge stop multiplying along
/// it.
///
/// **Why ten, a trade and not a derivation.** The split alternates the
/// two cells, so ten halvings narrow each to `2^-5` of its window: a
/// violation off the edge that wide, a thirty-second of the face, is
/// reached before the refusal. The pairs AT the edge multiply along it
/// as the cells narrow — some `2^5` of them per cusp edge by then —
/// which is small against [`DEFAULT_MAX_CELL_DEPTH`]'s floor and the
/// default pair budget, so the refusal arrives in a few hundred pairs
/// (the kiss row's 213) rather than by budget exhaustion. A deeper dial
/// reaches smaller violations beside a cusp at an exponential cost in
/// pairs; the second-order arm
/// (`work/embed/the-second-order-arm-certifies-a-declared-wedge-0-edge.md`)
/// retires the dial. A recorded run dial.
pub const CUSP_DEPTH: u32 = 10;

/// The whole-query cell-pair budget of ONE [`min_separation`] query.
///
/// A recorded run dial. The bracket a budget buys is honest at any
/// width ([`MinSeparation`]), so the dial trades tightness for time and
/// never truth.
pub const DEFAULT_MAX_MIN_SEP_PAIRS: usize = 512;

/// **Which question the engine is answering**: a clearance bounded
/// below, above or both, or E7's self-intersection question.
///
/// A clearance bound is an ENCLOSURE: the bound a document writes is a
/// length formula, evaluated at the certifying scalar over the run's
/// box, and every margin is taken against all of it. `= c` is
/// [`Self::Within`] with `lo = hi = c`.
#[derive(Debug, Clone, Copy)]
pub enum ClearanceBound {
    /// `separation ≥ lo`, non-strict: a separation equal to the bound
    /// at the run's tolerance HOLDS.
    AtLeast(Interval),
    /// `separation ≤ hi`, non-strict: it holds when one verified pair
    /// of face points lies within `hi`, and is violated when every cell
    /// pair is certified clear of it.
    AtMost(Interval),
    /// `lo ≤ separation ≤ hi`: both questions, classified in ONE
    /// subdivision.
    Within {
        /// The lower bound.
        lo: Interval,
        /// The upper bound.
        hi: Interval,
    },
    /// Strictly positive separation — E7's self-intersection question.
    /// A coincidence at the run's tolerance is a VIOLATION, not a
    /// discharge.
    StrictlyPositive,
}

impl ClearanceBound {
    /// `separation ≥ c` at an exact `c`.
    #[must_use]
    pub fn at_least(c: f64) -> Self {
        Self::AtLeast(Interval::from_f64(c))
    }

    /// The lower bound a cell pair's separation is decided against:
    /// `0` for the strict question, `None` where only an upper bound is
    /// asked.
    #[must_use]
    pub fn lower(self) -> Option<Interval> {
        match self {
            Self::AtLeast(lo) | Self::Within { lo, .. } => Some(lo),
            Self::StrictlyPositive => Some(Interval::from_f64(0.0)),
            Self::AtMost(_) => None,
        }
    }

    /// The upper bound, where one is asked.
    #[must_use]
    pub fn upper(self) -> Option<Interval> {
        match self {
            Self::AtMost(hi) | Self::Within { hi, .. } => Some(hi),
            Self::AtLeast(_) | Self::StrictlyPositive => None,
        }
    }

    /// The farthest a bound reaches: the upper end of the larger bound.
    /// A pair separated by more than this, plus the band, decides every
    /// question the bound asks without being examined.
    fn reach(self) -> f64 {
        [self.lower(), self.upper()]
            .into_iter()
            .flatten()
            .map(Interval::hi)
            .fold(0.0, f64::max)
    }

    /// The funnel site this bound decides at.
    pub fn predicate(self) -> &'static str {
        match self {
            Self::AtLeast(_) | Self::AtMost(_) | Self::Within { .. } => CLEARANCE_MARGIN,
            Self::StrictlyPositive => SELF_INTERSECTION_GAP,
        }
    }

    /// Whether a definite `Zero` against the lower bound discharges
    /// (see the two site docs).
    pub fn zero_discharges(self) -> bool {
        !matches!(self, Self::StrictlyPositive)
    }
}

/// **What a [`Candidate`] is offered for**: the question its verifier
/// confirms or declines.
#[derive(Debug, Clone, Copy)]
pub enum Ask {
    /// A pair of face points closer than the lower bound — a violation.
    /// `strict` is the self-intersection question, whose violation is a
    /// coincidence: a definite `Zero` violates it.
    Below {
        /// The lower bound.
        bound: Interval,
        /// Whether a definite `Zero` violates.
        strict: bool,
    },
    /// A pair of face points within the upper bound — its proof.
    Within {
        /// The upper bound.
        bound: Interval,
    },
}

impl Ask {
    /// The bound this question is decided against.
    #[must_use]
    pub fn bound(self) -> Interval {
        match self {
            Self::Below { bound, .. } | Self::Within { bound } => bound,
        }
    }

    /// The bound this question asks as the sweep names it, whose funnel
    /// site ([`ClearanceBound::predicate`]) it is decided at.
    fn as_bound(self) -> ClearanceBound {
        match self {
            Self::Below { strict: true, .. } => ClearanceBound::StrictlyPositive,
            Self::Below { bound, .. } => ClearanceBound::AtLeast(bound),
            Self::Within { bound } => ClearanceBound::AtMost(bound),
        }
    }

    /// **The one sign table**: a separation `d` read against this
    /// question at its bound's funnel site with the run's band. A lower
    /// bound is answered (violated) below it, and at it too for the
    /// strict question, whose `Zero` is a coincidence; an upper bound is
    /// answered (proved) at or below it. Both the sweep's reading of a
    /// cell pair's enclosure ([`Bound::read`]) and a verifier's reading
    /// of the pair it found ([`Self::confirms`]) are this table.
    fn reading(self, d: Interval, band: Band) -> Reading {
        match decide(
            self.as_bound().predicate(),
            Margin::of(d - self.bound()),
            band,
        ) {
            Err(source) => Reading::Undecided(source),
            Ok(sign) => match (self, sign) {
                (Self::Below { .. }, Sign::Negative)
                | (Self::Below { strict: true, .. }, Sign::Zero)
                | (Self::Within { .. }, Sign::Negative | Sign::Zero) => Reading::Answers,
                (Self::Below { .. } | Self::Within { .. }, _) => Reading::Clear,
            },
        }
    }

    /// **Whether a measured separation `d` answers this question**:
    /// [`Self::reading`]'s table, `Ok` where it answers.
    ///
    /// # Errors
    ///
    /// The funnel's indeterminacy, or the definite sign that does not
    /// answer, in words.
    pub fn confirms(self, d: Interval, band: Band) -> Result<(), String> {
        match self.reading(d, band) {
            Reading::Answers => Ok(()),
            Reading::Clear => Err(format!(
                "the separation {d:?} does not answer {self:?} at this run's tolerance"
            )),
            Reading::Undecided(source) => Err(format!(
                "the separation {d:?} against {:?}: {source}",
                self.bound()
            )),
        }
    }
}

/// **One side of a clearance question**: a body, and the faces of it in
/// scope.
///
/// No node address and no frame. Where the body came from is the
/// caller's to keep; a side in another frame than the question's is
/// read through its motion ([`Self::windows_at`]), its body as it
/// stands.
#[derive(Debug, Clone)]
pub struct Side<'b, T: Real> {
    /// The body, at the lane's scalar.
    pub body: &'b Body<T>,
    /// The faces in scope, in the order the walk pairs them. A caller
    /// that resolved them from names sorts and de-duplicates them, so
    /// the candidate order — and so which of several equally true
    /// violations is reported — is a function of the body.
    pub faces: Vec<FaceKey>,
}

impl<'b, T: Decide + CertifiedBounds> Side<'b, T> {
    /// The side's faces read into carrier windows for [`Sweep::run`],
    /// in the order of [`Self::faces`]. `which` is the side this one is
    /// of its question, which a refusal names the face by.
    ///
    /// # Errors
    ///
    /// [`EngineRefusal::Unsupported`] for the first face whose carrier
    /// has no admitted window.
    pub fn windows(&self, which: Which) -> Result<Windows<'b, T>, EngineRefusal> {
        self.windows_at(which, None)
    }

    /// [`Self::windows`] of a side read from another frame: each window's
    /// carrier mapped through `at`, the rigid motion from the body's
    /// coordinates into the question's ([`crate::transform::surface_at`]),
    /// and which part of its face a cell holds read off the face in its
    /// own coordinates. Two sides of one body are in one frame, `at`
    /// `None`.
    ///
    /// # Errors
    ///
    /// As [`Self::windows`]; a face whose carrier, read through `at`,
    /// does not certify.
    pub fn windows_at(
        &self,
        which: Which,
        at: Option<&RigidMotion<T>>,
    ) -> Result<Windows<'b, T>, EngineRefusal> {
        let of = self
            .faces
            .iter()
            .map(|&k| window_of(self.body, k, which, at))
            .collect::<Result<_, _>>()?;
        Ok(Windows { of })
    }
}

/// **Which side of a clearance question** a face of a refusal is on:
/// the first [`Side`] the question names, or the second.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Which {
    /// The first side.
    First,
    /// The second side.
    Second,
}

impl Which {
    /// The side's word in a payload.
    pub fn name(self) -> &'static str {
        match self {
            Self::First => "first",
            Self::Second => "second",
        }
    }

    /// Of `sides`, the one this names.
    pub fn of<S>(self, [first, second]: [S; 2]) -> S {
        match self {
            Self::First => first,
            Self::Second => second,
        }
    }
}

/// One side's faces read into carrier windows: what [`Sweep::run`]
/// subdivides.
#[derive(Debug, Clone)]
pub struct Windows<'b, T: Real> {
    of: Vec<Window<'b, T>>,
}

#[cfg(any(test, feature = "test-support"))]
impl<T: Real> Windows<'_, T> {
    /// **How many posed boundaries each face built in this ask**
    /// ([`injective::FacePoses`]), by face: a sphere face builds one per
    /// pose, once, however many cells are read in it.
    #[doc(hidden)]
    pub fn posed_builds(&self) -> Vec<(FaceKey, usize)> {
        self.of.iter().map(|w| (w.face, w.poses.builds())).collect()
    }

    /// **How many readings each spline chart's and spline edge's memo
    /// holds** ([`geom::NurbsSurface::memo_entries`],
    /// [`geom::NurbsCurve3::memo_entries`]): the charts', then the
    /// edges', `None` for one that keeps no memo.
    #[doc(hidden)]
    pub fn spline_memos(&self) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
        let charts = self
            .of
            .iter()
            .filter_map(|w| match &w.surface {
                Surface::Nurbs(net) => Some(net.memo_entries()),
                _ => None,
            })
            .collect();
        let edges = self
            .of
            .iter()
            .flat_map(|w| &w.boundary.edges)
            .filter_map(|rim| match rim.arc.as_ref()? {
                (geom::Curve3::Nurbs(n), _) => Some(n.memo_entries()),
                _ => None,
            })
            .collect();
        (charts, edges)
    }
}

/// Why the engine has no answer. Typed and named, never a silent
/// partial.
///
/// These are the arms the engine itself raises. A caller that resolves
/// selections, folds leaves or checks the bound before it asks wraps
/// them beside its own.
///
/// **A face is named by side.** In every arm that carries two faces,
/// `a` is a face of the question's first [`Side`] and `b` of its
/// second, and an arm that carries one face or one scope carries its
/// [`Which`]. A `FaceKey` keys one body's arena, so two sides over two
/// bodies routinely carry equal keys for different faces; the
/// rendering ([`Self::say`]) names each face with its side, and a
/// caller that knows what each side is names it there.
#[derive(Debug, Clone, PartialEq)]
pub enum EngineRefusal {
    /// A genuine semantic sliver: the deciding enclosure sits WHOLLY
    /// inside the ambiguity band, so the separation being decided IS in
    /// the band and refinement cannot move it out ([`sliver`]).
    Sliver {
        /// The funnel site that could not decide.
        predicate: &'static str,
        /// The pair's face on the first side.
        a: FaceKey,
        /// Its face on the second side.
        b: FaceKey,
    },
    /// A budget was exhausted, typed and named, on a cell pair of two
    /// faces: the pair whose split it refused, or the first pair of a
    /// level it could not start.
    Budget {
        /// Which bound stopped the subdivision.
        stop: CellBudget,
        /// The pair's face on the first side.
        a: FaceKey,
        /// Its face on the second side.
        b: FaceKey,
    },
    /// A face whose carrier the engine does not read (E7: refuse
    /// typed, never downgrade to sampling): no admitted window, or, for
    /// the injectivity question, no kind that makes it injective.
    Unsupported {
        /// The carrier class, named.
        carrier: &'static str,
        /// The face that carries it.
        face: FaceKey,
        /// The side it is on.
        side: Which,
    },
    /// A carrier enclosure that did not evaluate: the margin came back
    /// [`geom_core::MarginKind::Invalid`] (NaI, or an empty enclosure),
    /// which is neither an indeterminacy refinement could settle nor a
    /// budget. Its own class so a reader is not sent looking for a
    /// bigger dial.
    PoisonEnclosure {
        /// The pair's face on the first side.
        a: FaceKey,
        /// Its face on the second side.
        b: FaceKey,
    },
    /// A scope naming no face. `Holds` over an empty selection is
    /// vacuously true and useless; a caller that asked about nothing is
    /// told so.
    EmptyScope {
        /// The side whose scope it is (the first, when both are empty).
        side: Which,
    },
    /// A touching cell pair of the injectivity question that no local
    /// certificate discharged down to the depth floor, or that no arm
    /// can read at any depth: the two faces, the cell they share, and
    /// the arm that failed.
    Uncertified {
        /// The pair's face on the first side.
        a: FaceKey,
        /// Its face on the second side.
        b: FaceKey,
        /// The cell of M the two cells touch through.
        shared: SharedCell,
        /// The arm that failed.
        arm: CertificateArm,
    },
    /// A face whose chart collapses an edge to a point (a null edge's
    /// vertex) near a touching pair: the monotone arm reads no
    /// collapsed spline chart.
    CollapsedChart {
        /// The face.
        face: FaceKey,
        /// The side it is on.
        side: Which,
        /// The vertex of the face's body the chart collapses to.
        vertex: VertexKey,
    },
    /// A face on a torus not decided a ring (`R > r > 0`), so not
    /// injective by its kind: a half of the convention decided outside it
    /// (a tube not shown thicker than zero, or not shown thinner than its
    /// ring), or one the band cannot decide, which escalates as its own
    /// margin. The convention's one refusal carries which, and says it in
    /// the half's own words ([`geom_brep::TorusRefusal`]).
    TorusNotRing {
        /// The face.
        face: FaceKey,
        /// The side it is on.
        side: Which,
        /// The convention's half and its verdict, boxed: its undecided
        /// verdict carries an [`geom_core::Indeterminate`], which would
        /// make every engine refusal as large.
        refusal: Box<geom_brep::TorusRefusal>,
    },
    /// A cell pair the interval pass classified as a definite violation
    /// of a lower bound, or as within an upper one, whose witness the
    /// caller's verifier did not confirm. The engine
    /// reports what the verifier returned, verbatim; whether the arm can
    /// fire is the verifier's property, stated at the verifier.
    ///
    /// Its own arm, and never a violation: a witness is a claim about a
    /// concrete configuration, and one the independent recomputation
    /// contradicts is evidence about the analysis, not about the
    /// geometry.
    WitnessUnverified {
        /// What the verifier could not confirm.
        what: String,
        /// The pair's face on the first side.
        a: FaceKey,
        /// Its face on the second side.
        b: FaceKey,
    },
    /// **An upper bound the box decides only in part.** The closest
    /// pair of the cells' stations, its separation enclosed over every
    /// realization the lane covers ([`Enclosure::separation`]),
    /// straddles the bound: within it at some realizations, past it at
    /// others, as far as the enclosure can tell. Over a parameter box
    /// `≤ c` is a claim about EVERY point of the box, and a pair within
    /// it at one point proves it there and nowhere else, so this is no
    /// verdict over the box: a caller that can narrow the box bisects
    /// on it, and one that cannot refuses. Splitting a CELL does not
    /// help, because it narrows the stations and never the box.
    BoxDependent {
        /// The proving pair's face on the first side.
        a: FaceKey,
        /// Its face on the second side.
        b: FaceKey,
        /// The pair's separation over the lane's realizations, `(lo, hi)`.
        separation: (f64, f64),
    },
}

impl EngineRefusal {
    /// The refusal's own payload, rendered for the goldening form — the
    /// half `name` drops, so two runs that refuse for the same CLASS on
    /// different evidence do not serialize alike.
    pub fn payload(&self) -> String {
        match self {
            Self::Sliver { predicate, a, b } => format!("{predicate} first={a:?} second={b:?}"),
            Self::Budget { stop, a, b } => format!("{stop:?} first={a:?} second={b:?}"),
            Self::Unsupported {
                carrier,
                face,
                side,
            } => format!("{carrier} {}={face:?}", side.name()),
            Self::WitnessUnverified { what, a, b } => {
                format!("first={a:?} second={b:?} {what}")
            }
            Self::BoxDependent { a, b, separation } => {
                format!(
                    "first={a:?} second={b:?} [{}, {}]",
                    separation.0, separation.1
                )
            }
            Self::PoisonEnclosure { a, b } => format!("first={a:?} second={b:?}"),
            Self::Uncertified { a, b, shared, arm } => {
                format!(
                    "first={a:?} second={b:?} {} {}",
                    shared.payload(),
                    arm.name()
                )
            }
            Self::CollapsedChart { face, side, vertex } => {
                format!("{}={face:?} {vertex:?}", side.name())
            }
            Self::TorusNotRing { face, side, .. } => format!("{}={face:?}", side.name()),
            Self::EmptyScope { side } => side.name().to_owned(),
        }
    }

    /// The refusal's stable class name, for reports and the goldening
    /// form.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Sliver { .. } => "sliver",
            Self::Budget { .. } => "budget",
            Self::Unsupported { .. } => "unsupported",
            Self::WitnessUnverified { .. } => "witness_unverified",
            Self::BoxDependent { .. } => "box_dependent",
            Self::PoisonEnclosure { .. } => "poison_enclosure",
            Self::EmptyScope { .. } => "empty_scope",
            Self::Uncertified { .. } => "uncertified",
            Self::CollapsedChart { .. } => "collapsed_chart",
            Self::TorusNotRing { .. } => "torus_not_ring",
        }
    }

    /// **The refusal in words, into `f`**, each face and scope named with
    /// its side as `sides` writes it: the first side's, then the
    /// second's. A side is written by its caller, so a caller writing a
    /// sentence of its own says each side in its own terms.
    ///
    /// # Errors
    ///
    /// The writer's.
    pub fn say<W: core::fmt::Write + ?Sized>(
        &self,
        f: &mut W,
        sides: [&dyn Fn(&mut W) -> core::fmt::Result; 2],
    ) -> core::fmt::Result {
        let [first, second] = sides;
        match self {
            Self::Sliver { predicate, a, b } => {
                write!(f, "the separation of face {a:?} of ")?;
                first(f)?;
                write!(f, " and face {b:?} of ")?;
                second(f)?;
                write!(
                    f,
                    " sits wholly inside {predicate}'s ambiguity band, so no refinement can \
                     decide it"
                )
            }
            Self::Budget { stop, a, b } => {
                let pair = |f: &mut W| -> core::fmt::Result {
                    write!(f, "face {a:?} of ")?;
                    first(f)?;
                    write!(f, " and face {b:?} of ")?;
                    second(f)
                };
                match stop {
                    CellBudget::Depth { max_cell_depth } => {
                        f.write_str("a cell pair of ")?;
                        pair(f)?;
                        write!(
                            f,
                            " was split {max_cell_depth} times, the depth budget, and is still \
                             undecided"
                        )
                    }
                    CellBudget::Pairs { max_cell_pairs } => {
                        write!(
                            f,
                            "the query examined {max_cell_pairs} cell pairs, its budget, with \
                             pairs still undecided, the first of them of "
                        )?;
                        pair(f)
                    }
                    CellBudget::Resolution => {
                        f.write_str("a cell pair of ")?;
                        pair(f)?;
                        f.write_str(
                            " could not be bisected: its midpoint lands on an endpoint, so the \
                             f64 grid itself is the bound",
                        )
                    }
                }
            }
            Self::Unsupported {
                carrier,
                face,
                side,
            } => {
                write!(
                    f,
                    "the clearance engine cannot answer for face {face:?} of "
                )?;
                side.of(sides)(f)?;
                write!(f, ": it is {carrier}")
            }
            Self::WitnessUnverified { what, a, b } => {
                write!(f, "a pair of face {a:?} of ")?;
                first(f)?;
                write!(f, " and face {b:?} of ")?;
                second(f)?;
                write!(
                    f,
                    " the interval pass found below the lower bound or within the upper one was \
                     not confirmed by the verifier: {what}"
                )
            }
            Self::PoisonEnclosure { a, b } => {
                write!(f, "the enclosure between face {a:?} of ")?;
                first(f)?;
                write!(f, " and face {b:?} of ")?;
                second(f)?;
                f.write_str(" did not evaluate, which is geometry, not a budget")
            }
            Self::BoxDependent { a, b, separation } => {
                write!(f, "the closest pair of face {a:?} of ")?;
                first(f)?;
                write!(f, " and face {b:?} of ")?;
                second(f)?;
                write!(
                    f,
                    " reaches [{}, {}] over the parameter box, within the upper bound at some \
                     of its points and past it at others, so the box does not decide it; a \
                     narrower box can",
                    separation.0, separation.1
                )
            }
            Self::EmptyScope { side } => {
                f.write_str("the scope of ")?;
                side.of(sides)(f)?;
                f.write_str(" names no face, so there is nothing to measure")
            }
            Self::Uncertified { a, b, shared, arm } => {
                write!(f, "face {a:?} of ")?;
                first(f)?;
                write!(f, " and face {b:?} of ")?;
                second(f)?;
                write!(
                    f,
                    " touch through {}, and the {} certificate did not discharge them",
                    shared.payload(),
                    arm.name()
                )
            }
            Self::CollapsedChart { face, side, vertex } => {
                write!(f, "the chart of face {face:?} of ")?;
                side.of(sides)(f)?;
                write!(
                    f,
                    " collapses an edge to its vertex {vertex:?} near a touching pair, and the \
                     monotone certificate reads no collapsed chart"
                )
            }
            Self::TorusNotRing {
                face,
                side,
                refusal,
            } => {
                write!(f, "face {face:?} of ")?;
                side.of(sides)(f)?;
                write!(
                    f,
                    " lies on a torus not decided a ring (R > r > 0): {refusal}"
                )
            }
        }
    }
}

/// The refusal where no caller names the sides: the first side and
/// the second.
impl core::fmt::Display for EngineRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.say(
            f,
            [
                &|f: &mut core::fmt::Formatter<'_>| f.write_str("the first side"),
                &|f: &mut core::fmt::Formatter<'_>| f.write_str("the second side"),
            ],
        )
    }
}

impl core::error::Error for EngineRefusal {}

/// **A cell of the boundary complex two scopes share**: where a
/// touching pair touches, and where two scopes that meet meet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedCell {
    /// A vertex both faces bound on.
    Vertex(VertexKey),
    /// An edge both faces bound on, or one face's own seam.
    Edge(EdgeKey),
    /// One face, in both scopes or against itself.
    Face(FaceKey),
}

impl SharedCell {
    /// The cell, for a payload.
    pub fn payload(&self) -> String {
        match self {
            Self::Vertex(v) => format!("vertex {v:?}"),
            Self::Edge(e) => format!("edge {e:?}"),
            Self::Face(f) => format!("face {f:?}"),
        }
    }
}

/// **The local certificate a touching pair needs**: the arms of
/// ERROR-DESIGN's *Self-intersection scope* clause, and the injectivity a
/// face has by its carrier's kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateArm {
    /// A face on an analytic carrier, one-to-one by kind.
    ByKind,
    /// Within one face: a strongly monotone projection.
    Monotone,
    /// Across a shared edge or vertex: departure to opposite sides.
    Departure,
    /// At a wedge-0/2π edge: the second inward derivative's sign. Not
    /// built; an edge that needs it refuses.
    SecondOrder,
}

impl CertificateArm {
    /// The arm's stable name.
    pub fn name(self) -> &'static str {
        match self {
            Self::ByKind => "by_kind",
            Self::Monotone => "monotone",
            Self::Departure => "departure",
            Self::SecondOrder => "second_order",
        }
    }
}

/// **Why [`min_separation`] has no bracket**: the four arms of
/// [`EngineRefusal`] its walk can raise. It decides no cell and holds
/// no budget it could exhaust, so a sliver, a budget and an unverified
/// witness are the bound sweep's and never this door's.
#[derive(Debug, Clone, PartialEq)]
pub enum MinSeparationRefusal {
    /// [`EngineRefusal::Unsupported`].
    Unsupported {
        /// The carrier class, named.
        carrier: &'static str,
        /// The face that carries it.
        face: FaceKey,
        /// The side it is on.
        side: Which,
    },
    /// [`EngineRefusal::PoisonEnclosure`].
    PoisonEnclosure {
        /// The pair's face on the first side.
        a: FaceKey,
        /// Its face on the second side.
        b: FaceKey,
    },
    /// [`EngineRefusal::EmptyScope`].
    EmptyScope {
        /// The side whose scope it is (the first, when both are empty).
        side: Which,
    },
}

impl MinSeparationRefusal {
    /// The refusal as the engine's whole vocabulary spells it.
    pub fn engine(&self) -> EngineRefusal {
        match *self {
            Self::Unsupported {
                carrier,
                face,
                side,
            } => EngineRefusal::Unsupported {
                carrier,
                face,
                side,
            },
            Self::PoisonEnclosure { a, b } => EngineRefusal::PoisonEnclosure { a, b },
            Self::EmptyScope { side } => EngineRefusal::EmptyScope { side },
        }
    }

    /// [`EngineRefusal::payload`], for this arm.
    pub fn payload(&self) -> String {
        self.engine().payload()
    }

    /// [`EngineRefusal::name`], for this arm.
    pub fn name(&self) -> &'static str {
        self.engine().name()
    }
}

/// [`EngineRefusal`]'s `Display`, for this arm.
impl core::fmt::Display for MinSeparationRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.engine(), f)
    }
}

impl core::error::Error for MinSeparationRefusal {}

/// Which bound stopped the subdivision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellBudget {
    /// One cell pair had already been split
    /// [`Budgets::max_cell_depth`] times.
    Depth {
        /// The bound.
        max_cell_depth: u32,
    },
    /// The query had reached [`Budgets::max_cell_pairs`]; this pair was
    /// still on the frontier and is refused rather than forgotten.
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

/// The predicate name of an escalation whose enclosure sits WHOLLY
/// inside the ambiguity band `(ε, Kε)` — the ratified terminal-sliver
/// test, the classifier's own verdict
/// ([`geom_core::Indeterminate::terminal_sliver`]) — or `None` when
/// refinement could still decide it.
///
/// One home for both subdivisions: the sweep here refuses by it, and
/// so does the parameter-box driver that hands the engine its leaves,
/// so the two cannot drift apart on what a sliver is.
pub fn sliver(source: &geom_core::Indeterminate) -> Option<&'static str> {
    source
        .terminal_sliver
        .then(|| source.predicate.unwrap_or(geom_core::UNNAMED_PREDICATE))
}

/// The counting receipt of one clearance query.
///
/// Each candidate pair is the root of its own subdivision: a cell pair
/// is discharged, violated, refused, ABANDONED unexamined, or split —
/// into two, or into three where the injective walk splits a cell
/// against itself — and `splits` counts each split's children less
/// one. A forest of `candidates` trees counted so has exactly
/// `splits + candidates` leaves, and every leaf is in one of the five
/// buckets. It rides the shipped report, so a consumer re-checks it
/// without trusting this module ([`CellReceipt::holds`]).
///
/// **`abandoned` is what the walk did not examine, spelled rather than
/// hidden.** Calling it `refused` would claim it was tried. The bound
/// sweep abandons the frontier it held when it stops, at a verified
/// violation or, for the injectivity question, at its first refusal;
/// a level that does not fit its pair budget, which it never starts;
/// and a pair that only an upper bound still wanted once
/// that bound is proved, so a bound query that runs to completion with
/// no upper bound has `abandoned == 0`. [`min_separation`] abandons a level that does not
/// fit its pair budget and prices it into the bracket, so a completed
/// bracket can carry `abandoned > 0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CellReceipt {
    /// Candidate face pairs: what the interval BVH did not exclude,
    /// minus the second copy of any pair two scopes of one body named
    /// twice. A face against itself is a candidate. It is the number of
    /// subdivision roots.
    pub candidates: usize,
    /// Cell pairs discharged (separation definitely satisfies the
    /// bound).
    pub discharged: usize,
    /// Cell pairs whose separation definitely violates it.
    pub violated: usize,
    /// Cell pairs whose separation lies definitely within an upper
    /// bound: offered to the verifier as its proof.
    pub within: usize,
    /// Cell pairs refused, typed.
    pub refused: usize,
    /// Cell pairs that were split.
    pub splits: usize,
    /// **A sub-count of `discharged`, not a bucket of its own**: cell
    /// PAIRS discharged VACUOUSLY, because one side's cell was decided
    /// outside its face ([`crate::cell_region`]). Nothing
    /// was measured and nothing decided at the clearance site, so the
    /// pair contributes no discharge width; `outside <= discharged`
    /// ([`CellReceipt::check_outside_is_a_sub_count`], enforced) and
    /// the receipt identity is untouched.
    ///
    /// **Pairs, not cells.** The question the door answers is
    /// per (window, cell) and the sweep asks it per pair, so one cell
    /// proven empty is counted once for every partner it was paired
    /// with. The sweep memoizes the ANSWER so it is computed once, but
    /// the count is a receipt count and receipts count cell pairs.
    pub outside: usize,
    /// **A sub-count of `discharged`**: touching cell pairs discharged by
    /// a local certificate ([`CertificateArm::Monotone`],
    /// [`CertificateArm::Departure`]) rather than by distance.
    pub certified: usize,
    /// **A sub-count of `discharged`**: faces against themselves
    /// discharged whole as injective by their carrier's kind
    /// ([`CertificateArm::ByKind`]), one per face, never subdivided.
    pub by_kind: usize,
    /// Cell pairs the walk never classified: what it still held when the
    /// sweep stopped at its finding, a level that did not fit the pair
    /// budget, and under
    /// [`min_separation`] a level that did not fit the pair budget, each
    /// pair priced into the bracket's `lo` unexamined.
    pub abandoned: usize,
}

impl CellReceipt {
    /// The receipt identity (see the type docs).
    pub fn holds(&self) -> bool {
        self.discharged + self.violated + self.within + self.refused + self.abandoned
            == self.splits + self.candidates
    }

    /// **`outside <= discharged`, checked rather than described.**
    ///
    /// A vacuous discharge that was not also counted a discharge is a
    /// bug in the sweep, not a verdict about geometry: it breaks the
    /// forest identity's meaning silently, because
    /// [`CellReceipt::holds`] does not read `outside` and cannot. So
    /// the relation is asserted at every site that mints or folds a
    /// receipt, and a receipt that violates it fails loud instead of
    /// riding a report.
    ///
    /// # Panics
    ///
    /// When `outside > discharged`.
    fn check_outside_is_a_sub_count(&self) {
        assert!(
            self.outside + self.certified + self.by_kind <= self.discharged,
            "a receipt's `outside`, `certified` and `by_kind` are disjoint SUB-COUNTs of \
             `discharged` and this one has outside = {}, certified = {}, by_kind = {} \
             against discharged = {}: a pair discharged without being counted a discharge \
             is a bug in the sweep, not a verdict",
            self.outside,
            self.certified,
            self.by_kind,
            self.discharged
        );
    }

    /// This receipt with another query's folded in. The identity is
    /// additive: a sum of forests is a forest.
    ///
    /// # Panics
    ///
    /// When either receipt has `outside > discharged`.
    #[must_use]
    pub fn plus(self, other: Self) -> Self {
        self.check_outside_is_a_sub_count();
        other.check_outside_is_a_sub_count();
        Self {
            candidates: self.candidates + other.candidates,
            discharged: self.discharged + other.discharged,
            violated: self.violated + other.violated,
            within: self.within + other.within,
            refused: self.refused + other.refused,
            splits: self.splits + other.splits,
            outside: self.outside + other.outside,
            certified: self.certified + other.certified,
            by_kind: self.by_kind + other.by_kind,
            abandoned: self.abandoned + other.abandoned,
        }
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
/// issue 1191's, observed here on a different family.
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
    /// Nothing discharged, nothing subdivided.
    pub fn empty() -> Self {
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

    /// These widths with another query's folded in.
    #[must_use]
    pub fn plus(self, other: Self) -> Self {
        let mut out = self;
        if let Some(w) = other.widest {
            out.widest = Some(out.widest.map_or(w, |x: f64| x.max(w)));
        }
        if let Some(w) = other.narrowest {
            out.narrowest = Some(out.narrowest.map_or(w, |x: f64| x.min(w)));
        }
        out.deepest = out.deepest.max(other.deepest);
        out
    }
}

// --------------------------------------------------- the bound walk

/// **The two budgets of one subdivision**, whichever question it
/// answers: [`Sweep`]'s, [`min_separation`]'s, and every caller's that
/// dials either.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Budgets {
    /// Per-pair subdivision depth ([`DEFAULT_MAX_CELL_DEPTH`]).
    pub max_cell_depth: u32,
    /// Whole-query cell-pair budget, ENFORCED AT ADMISSION: a split that
    /// would commit the query past this number is refused instead of
    /// taken.
    pub max_cell_pairs: usize,
}

impl Budgets {
    /// [`min_separation`]'s shipped dials: [`DEFAULT_MAX_CELL_DEPTH`] and
    /// [`DEFAULT_MAX_MIN_SEP_PAIRS`].
    pub const MIN_SEPARATION: Self = Self {
        max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
        max_cell_pairs: DEFAULT_MAX_MIN_SEP_PAIRS,
    };
}

/// **The bound question's walk**: the question, the band it is decided
/// at, and the two budgets.
#[derive(Debug, Clone, Copy)]
pub struct Sweep {
    /// Which question ([`ClearanceBound`]).
    pub bound: ClearanceBound,
    /// The linear band every compare classifies against: the clearance
    /// site's and the cells' and stations' ([`crate::cell_region`]).
    pub band: Band,
    /// How far the walk may subdivide.
    pub budgets: Budgets,
}

/// What [`Sweep::run`] answers: the verdict, its receipt, and the
/// measured limit.
#[derive(Debug, Clone, PartialEq)]
pub struct BoundReport<W> {
    /// The trichotomy.
    pub verdict: BoundVerdict<W>,
    /// The counting receipt.
    pub receipt: CellReceipt,
    /// The measured discharge widths.
    pub widths: DischargeWidths,
    /// **The certified cell pairs whose two cells are on DIFFERENT
    /// faces, per face pair** — departure across an edge or vertex the
    /// two faces share — keyed by the pair's faces in key order. A
    /// sub-count of [`CellReceipt::certified`]. A row that pins a count
    /// for every face pair that shares a vertex goes red when the sweep
    /// stops examining any one such pair, which neither the monotone
    /// self-pairs nor another pair's certificates can hide.
    pub across: BTreeMap<(FaceKey, FaceKey), usize>,
    /// **The certified cell pairs across faces whose certificate read a
    /// face at a corner** of its boundary, through the vertex where a
    /// curved edge meets another ([`injective`]'s corner presentation).
    /// A sub-count of the [`BoundReport::across`] counts' sum.
    pub cornered: usize,
    /// **How many readings of the faces' regions the answer rests on**:
    /// distinct (window, cell) pairs [`crate::cell_region`] read as
    /// `Inside` or `Outside`, whichever partner they were paired with,
    /// and distinct witness stations a face's point door placed
    /// ([`crate::cell_region::station_region`]). Zero says the faces'
    /// regions were never read into the answer.
    pub decided: usize,
}

/// **The E7 trichotomy**, with the verifier's witness where a pair
/// answers: the holding arms, the violated arms, and the typed refusal.
///
/// A lower bound's violation outranks everything: a pair closer than
/// the lower bound is closer than any upper one too. A refusal outranks
/// a holding answer, which is never presented over an unknown. For a
/// metric bound that ranking is over the whole walk; the injectivity
/// question ends at its first finding, so its violation is one met
/// before any refusal ([`Sweep::run`]).
#[derive(Debug, Clone, PartialEq)]
pub enum BoundVerdict<W> {
    /// A bound with no upper end holds over both carrier windows minus
    /// the cells decided outside their faces.
    Holds,
    /// A bound with an upper end holds: any lower end holds as
    /// [`Self::Holds`] says, and the verifier's pair lies within the
    /// upper end, which proves it.
    Within(W),
    /// A definite violation of the lower bound, confirmed by the
    /// verifier.
    Violated(W),
    /// The upper bound is violated: every cell pair is certified clear
    /// of it, so no pair of the two scopes lies within it.
    Exceeded,
    /// Typed, never silence.
    Refused(EngineRefusal),
}

/// **A cell pair the interval pass found violating, or could not
/// decide**, as the sweep hands it to the verifier.
///
/// The verifier's duty: answer `Ok` only for a concrete pair of points,
/// one in each of the two cells, whose separation it has itself decided
/// violates the bound, by a computation independent of the interval
/// pass; otherwise `Err` with what it found. The sweep stops at the
/// first `Ok` and takes it as the witness without re-checking it. For
/// the witness to be a pair of FACE points rather than carrier-window
/// points, the verifier draws its points from
/// [`Candidate::on_face_stations`]; the sweep cannot tell whether it
/// did.
pub struct Candidate<'s, 'b, T: Real> {
    x: &'s Window<'b, T>,
    ca: Cell,
    ia: usize,
    y: &'s Window<'b, T>,
    cb: Cell,
    ib: usize,
    region: &'s mut RegionMemo,
    /// Two scopes that meet: the cell they meet on, and its station on
    /// each face.
    meeting: Option<(SharedCell, Stations, Stations)>,
    /// What the verifier is asked to confirm.
    ask: Ask,
    /// Where the verifier says an upper bound's pair held at its own
    /// point only ([`Enclosure::box_dependent`]).
    box_dependent: &'s core::cell::Cell<Option<(f64, f64)>>,
}

impl<'s, T: Real> Candidate<'s, '_, T> {
    /// The two faces, the first side's then the second's.
    pub fn faces(&self) -> (FaceKey, FaceKey) {
        (self.x.face, self.y.face)
    }

    /// **The cell two scopes meet on**, where this candidate is their
    /// meeting rather than a cell pair: their separation is 0 there, and
    /// [`Self::on_face_stations`] offers that cell's own stations.
    pub fn meeting(&self) -> Option<SharedCell> {
        self.meeting.as_ref().map(|(cell, _, _)| *cell)
    }

    /// What this candidate is offered for: a violation of the lower
    /// bound, or the upper bound's proof.
    pub fn ask(&self) -> Ask {
        self.ask
    }

    /// **The pair's reading over every realization the lane covers**:
    /// the two windows' carriers at the lane's enclosure, from which a
    /// fixed pair of stations' separation is enclosed over the whole
    /// parameter box ([`Enclosure::separation`]).
    pub fn enclosure(&self) -> Enclosure<'s> {
        let coincident = self
            .meeting
            .as_ref()
            .and_then(|(_, a, b)| match (&a[..], &b[..]) {
                ([a], [b]) => Some((*a, *b)),
                _ => None,
            });
        Enclosure {
            a: self.x.surface.clone(),
            b: self.y.surface.clone(),
            coincident,
            box_dependent: self.box_dependent,
        }
    }
}

/// **A candidate's carriers at the lane's enclosure** ([`Candidate::enclosure`]).
///
/// An upper bound over a parameter box is universal: at EVERY point of
/// the box some pair is within it. A pair the verifier confirms on one
/// realization (an `f64` rebuild at one point) proves it at that point;
/// the same two stations read on these carriers prove it at every
/// point, because a station the point door placed on its face is placed
/// for every realization the lane covers
/// ([`Candidate::on_face_stations`]) and the enclosure of its point
/// covers each realization's point.
#[derive(Debug, Clone)]
pub struct Enclosure<'s> {
    a: Surface<Interval>,
    b: Surface<Interval>,
    /// A meeting's two stations ([`Candidate::meeting`]): one point of
    /// M — the shared vertex, or one vertex of one face — read on each
    /// side.
    coincident: Option<((f64, f64), (f64, f64))>,
    box_dependent: &'s core::cell::Cell<Option<(f64, f64)>>,
}

impl Enclosure<'_> {
    /// The separation of station `a` of the first face and station `b`
    /// of the second, enclosed over every realization the lane covers.
    ///
    /// A meeting's own two stations read one point of M on each face —
    /// the vertex they share, on both faces at every realization — so
    /// the faces' separation there is 0, read off topology. Enclosed as
    /// the difference of two independent evaluations it would widen with
    /// the box (the dependency problem) where nothing varies.
    pub fn separation(&self, a: (f64, f64), b: (f64, f64)) -> Interval {
        let bits = |(u, v): (f64, f64)| (u.to_bits(), v.to_bits());
        if self
            .coincident
            .is_some_and(|(p, q)| (bits(p), bits(q)) == (bits(a), bits(b)))
        {
            return Interval::from_f64(0.0);
        }
        let at = |x: f64| Interval::from_bounds(x, x);
        (self.a.eval(at(a.0), at(a.1)) - self.b.eval(at(b.0), at(b.1))).norm()
    }

    /// **The verifier's word that the box decides an upper bound's pair
    /// only in part**, the pair's separation over the lane's
    /// realizations being `separation`, which straddles the bound. The
    /// sweep reads it when the verifier declines, and refuses the pair
    /// [`EngineRefusal::BoxDependent`] instead of splitting it: splitting
    /// a cell narrows the stations, never the box.
    pub fn box_dependent(&self, separation: Interval) {
        self.box_dependent
            .set(Some((separation.lo(), separation.hi())));
    }
}

impl<T: Decide + CertifiedBounds> Candidate<'_, '_, T> {
    /// **The stations of each cell its face's point door places on the
    /// face**, the first side's then the second's, each a `(u, v)` of
    /// its face's stored chart.
    ///
    /// The lattice is each cell's three `u` and three `v` stations — the
    /// two ends and the midpoint the split rule cuts at — `u`-major, so
    /// at most nine a side. A station is kept when the door places it
    /// `In`, `OnEdge` or `OnVertex` ([`station_region`]). The door reads
    /// at the lane's scalar, so a definite verdict there holds for every
    /// realization it covers, an `f64` rebuild's own chart included.
    ///
    /// Each side's nine stations are placed ONCE, before any pairing:
    /// the question is per (window, station), and asking it inside a
    /// loop over pairs would ask the second side's nine questions once
    /// per admitted station of the first.
    ///
    /// For a meeting ([`Self::meeting`]) the stations are the shared
    /// cell's own: its vertex's chart point on each face, or, for one
    /// face in both scopes, the same stations on both sides.
    ///
    /// **Under the strict question each side gains one station more**,
    /// where it can: a violation there is a coincidence, which a lattice
    /// almost never holds, so the lattice's closest pair is carried by
    /// Gauss–Newton toward a common point of the two cells
    /// ([`coincidence`]) and offered when both its ends are placed on
    /// their faces. It is a candidate like any station: the verifier
    /// decides it.
    pub fn on_face_stations(self) -> (Stations, Stations) {
        if let Some((_, a, b)) = self.meeting {
            return (a, b);
        }
        let mut a = admitted(&mut *self.region, false, self.ia, self.x, self.ca);
        let mut b = admitted(&mut *self.region, true, self.ib, self.y, self.cb);
        if matches!(self.ask, Ask::Below { strict: true, .. })
            && let Some((p, q)) = coincidence(self.x, self.ca, &a, self.y, self.cb, &b)
            && Self::placed(&mut *self.region, false, self.ia, self.x, p)
            && Self::placed(&mut *self.region, true, self.ib, self.y, q)
        {
            a.push(p);
            b.push(q);
        }
        (a, b)
    }

    /// Whether the point door places `(u, v)` of window `index` on its
    /// face — in the window, for a window that is its face.
    fn placed(
        region: &mut RegionMemo,
        side_b: bool,
        index: usize,
        w: &Window<'_, T>,
        (u, v): (f64, f64),
    ) -> bool {
        match w.region {
            Region::Whole => w.u.0 <= u && u <= w.u.1 && w.v.0 <= v && v <= w.v.1,
            Region::Trimmed => matches!(
                region.station(side_b, index, w, (u, v)),
                Some(
                    FaceContainment::In | FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_)
                )
            ),
        }
    }
}

/// The `(u, v)` stations of one cell, each in its face's stored chart.
pub type Stations = Vec<(f64, f64)>;

fn admitted<T: Decide + CertifiedBounds>(
    region: &mut RegionMemo,
    side_b: bool,
    index: usize,
    w: &Window<'_, T>,
    c: Cell,
) -> Stations {
    let stations = |(lo, hi): (f64, f64)| [lo, geom::mid_param(lo, hi), hi];
    let mut out = Vec::with_capacity(9);
    for u in stations(c.u) {
        for v in stations(c.v) {
            if w.region == Region::Whole
                || matches!(
                    region.station(side_b, index, w, (u, v)),
                    Some(
                        FaceContainment::In
                            | FaceContainment::OnEdge(_)
                            | FaceContainment::OnVertex(_)
                    )
                )
            {
                out.push((u, v));
            }
        }
    }
    out
}

/// The clearance walk's query.
const CLEARANCE_PAIRS: geom_core::k_stats::QuerySite = geom_core::k_stats::QuerySite::new(
    "clearance_face_pairs",
    "the face pairs of two scopes whose boxes come within reach",
);

impl Sweep {
    /// The whole inner subdivision: candidate pairs from the interval
    /// BVH, then ONE level-synchronous frontier over all of them.
    ///
    /// `same_body` says the two sides are faces of one body, read into
    /// one frame, and a pair two scopes name in both orders is taken
    /// once. A face against itself is a pair. What the walk does with a
    /// pair whose faces share a cell of M depends on the question:
    ///
    /// - **Strictly positive, one body: injectivity** (ERROR-DESIGN's
    ///   *Self-intersection scope*). A face on an analytic carrier is
    ///   discharged against itself by kind. A cell pair is decided by
    ///   distance only where it does not touch: where neither cell's
    ///   ball is clear of a cell of M the two faces share, or the two
    ///   cells of one face meet, a local certificate
    ///   ([`crate::injective`]) discharges it or it splits, and at the
    ///   depth floor it refuses [`EngineRefusal::Uncertified`].
    /// - **Every other question is metric.** Two faces that share a cell
    ///   of M, or one face in both scopes, are at distance 0, read off
    ///   topology: `0 ≥ c` decides the whole pair, a violation offered
    ///   to `verify` with the shared cell's stations
    ///   ([`Candidate::meeting`]), and `0 ≤ c` is an upper bound's
    ///   proof at every realization, those stations' separation read as
    ///   that 0 ([`Enclosure::separation`]).
    ///
    /// **Breadth, not depth, and the reason is the answer's stability.**
    /// A depth-first walk reaches a violation only after exhausting
    /// whatever it descended into first, so which of two equally true
    /// violations a query reports — and whether it reports one at all
    /// before its budget runs out — depends on the candidate order. A
    /// level-synchronous frontier classifies every cell pair at depth
    /// `d` before any at `d + 1`, so the violation a query reports is
    /// the SHALLOWEST one, and the driver's own frontier idiom (D9) is
    /// the shape it borrows. Order dependence is reduced, not removed:
    /// two violations at the same depth are still separated by the
    /// candidate order, which is the order of the sides' faces.
    ///
    /// **It stops at the first VERIFIED violation, and the injectivity
    /// question at its first refusal too.** `verify` is asked,
    /// in that shallowest-first order, about every cell pair the funnel
    /// classifies violating, and about an indeterminate pair that does
    /// not touch, at the root and at the depth floor; its first `Ok` is
    /// the verdict. An `Err` on a pair classified violating is
    /// [`EngineRefusal::WitnessUnverified`] carrying its prose. A
    /// touching pair is never offered: its two cells share a point of M,
    /// and a pair of coincident stations there is no violation.
    ///
    /// **The order that makes a finding the first** is the same for a
    /// refusal as for a violation: the faces injective by kind, each
    /// against itself, in the order of the side's faces; then the
    /// frontier, a level at a time, the candidates in ascending order of
    /// their two faces' places in the sides, a split's children in its
    /// cut's order. It reads no address and no hash, so one body asked
    /// twice in one order of its faces reports one finding.
    ///
    /// **Whether a body passes is the shape's; which finding a body that
    /// does not pass is told is the walk's.** A cell's enclosure is a box
    /// on the frame's axes, so the level at which a pair is decided or
    /// refused moves with the pose, and the candidates' order is the
    /// caller's order of faces, which is the body's order of
    /// construction. A body that crosses itself and also holds a place
    /// the engine cannot read is therefore told the crossing in one pose
    /// or order and the refusal in another, and never that it holds.
    /// A metric bound keeps its first refusal and reads on to the end
    /// of the frontier, where only a verified violation outranks it.
    ///
    /// **Its readings are its report's, not the caller's log** (audit
    /// row F41): every decision the walk and `verify` take runs in
    /// [`k_stats::unlogged`]'s scope, so no caller has to keep one.
    pub fn run<T, W, F>(
        &self,
        wa: &Windows<'_, T>,
        wb: &Windows<'_, T>,
        same_body: bool,
        verify: F,
    ) -> BoundReport<W>
    where
        T: Decide + CertifiedBounds,
        F: FnMut(Candidate<'_, '_, T>) -> Result<W, String>,
    {
        // Two sides of one body are in one frame: the same-body arms read
        // each window's boundary beside its carrier, both in that frame.
        debug_assert!(
            !same_body || wa.of.iter().chain(&wb.of).all(|w| w.own.is_none()),
            "two sides of one body are read through no motion"
        );
        k_stats::unlogged("F41", move || {
            let (wa, wb) = (&wa.of[..], &wb.of[..]);
            let injective = same_body && matches!(self.bound, ClearanceBound::StrictlyPositive);
            // **The prune threshold carries the BAND.** The tree excludes a
            // pair on a raw `separation_lo > pad`, and the engine decides
            // through the funnel, which calls anything within `escalate` of
            // a bound indeterminate. Padding the farthest bound by the band
            // is what makes the two agree in the only direction that
            // matters: a pair this drops is separated by more than every
            // bound PLUS the funnel's own escalation threshold, so it is one
            // the funnel would have decided clear of each, and a pair inside
            // the band survives to be classified or refused rather than
            // silently held.
            let reach = self.bound.reach() + self.band.escalate();
            let root_a: Vec<Cell> = wa.iter().map(Cell::of).collect();
            let root_b: Vec<Cell> = wb.iter().map(Cell::of).collect();
            let cloud = |c: &Cell| [c.enc];
            // A zero pad still steps every box out an ulp
            // (`Aabb::padded`); here that can only keep a candidate pair,
            // never drop one, and nothing is split at these bounds.
            let tree_a = Bvh::build_bounded(root_a.iter().map(cloud), 0.0);
            let tree_b = Bvh::build_bounded(root_b.iter().map(cloud), 0.0);
            // The unordered de-duplication is the same-body case's own: two
            // scopes of one body can carry a pair in both orders.
            let mut seen: BTreeSet<(FaceKey, FaceKey)> = BTreeSet::new();
            let candidates: Vec<(usize, usize)> = tree_a
                .pairs_within(CLEARANCE_PAIRS, &tree_b, reach)
                .into_iter()
                .filter(|&(i, j)| {
                    let (Some(x), Some(y)) = (wa.get(i), wb.get(j)) else {
                        return false;
                    };
                    !same_body || seen.insert((x.face.min(y.face), x.face.max(y.face)))
                })
                .collect();
            // What each candidate's two faces share, in M.
            let shared: Vec<Shared> = candidates
                .iter()
                .map(|&(i, j)| match (wa.get(i), wb.get(j)) {
                    (Some(x), Some(y)) if same_body => {
                        Shared::of(&x.boundary, &y.boundary, x.face == y.face)
                    }
                    _ => Shared::default(),
                })
                .collect();

            let mut receipt = CellReceipt {
                candidates: candidates.len(),
                ..CellReceipt::default()
            };
            let mut q = Bound {
                sweep: self,
                injective,
                shared: &shared,
                at_refusal: if injective {
                    AtRefusal::End
                } else {
                    AtRefusal::ReadOn
                },
                faces: WindowFaces::of(wa, wb),
                region: RegionMemo::at(self.band),
                locals: LocalMemo::default(),
                across: BTreeMap::new(),
                cornered: 0,
                widths: DischargeWidths::empty(),
                lower_refusal: None,
                upper_refusal: None,
                violation: None,
                within: None,
                box_dependent: core::cell::Cell::new(None),
                verify,
            };
            let open = Open {
                below: self.bound.lower().is_some(),
                above: self.bound.upper().is_some(),
            };

            let mut frontier: Vec<Task> = Vec::with_capacity(candidates.len());
            // The candidate the sweep ended on, before any walk.
            let mut ended_at = None;
            for (k, &(i, j)) in candidates.iter().enumerate() {
                let (Some(x), Some(y), Some(&a), Some(&b), Some(sh)) = (
                    wa.get(i),
                    wb.get(j),
                    root_a.get(i),
                    root_b.get(j),
                    shared.get(k),
                ) else {
                    // A candidate whose windows are not addressable would
                    // leave the receipt short by one; counted, not lost.
                    receipt.abandoned += 1;
                    continue;
                };
                let meets = same_body && (x.face == y.face || !sh.is_empty());
                if meets && !injective {
                    // **Two scopes that meet are at distance 0**, read off
                    // topology rather than approached by subdivision: the
                    // whole pair is one leaf, `0` decided against each bound
                    // at its own site.
                    let cell = if x.face == y.face {
                        SharedCell::Face(x.face)
                    } else {
                        sh.cell(&x.boundary).unwrap_or(SharedCell::Face(x.face))
                    };
                    let zero = Interval::from_f64(0.0);
                    let stations = meeting_stations(x, y, sh, cell, self.band);
                    let at = Offer {
                        x,
                        ca: a,
                        ia: i,
                        y,
                        cb: b,
                        ib: j,
                        meeting: Some((cell, stations.0, stations.1)),
                    };
                    let readings = q.read(open, zero);
                    // An exact margin in the band: no refinement moves a
                    // distance topology fixed at 0.
                    let sliver = |r: &Reading| match r {
                        Reading::Undecided(source) => Some(EngineRefusal::Sliver {
                            predicate: source.predicate.unwrap_or(geom_core::UNNAMED_PREDICATE),
                            a: x.face,
                            b: y.face,
                        }),
                        _ => None,
                    };
                    if let Some(r) = readings.below.as_ref().and_then(sliver) {
                        q.keep_below(r);
                    }
                    if let Some(r) = readings.above.as_ref().and_then(sliver) {
                        q.upper_refusal.get_or_insert(r);
                    }
                    match q.settle(&readings, &at) {
                        ControlFlow::Break(bucket) => {
                            bucket.count(&mut receipt);
                            ended_at = Some(k);
                            break;
                        }
                        ControlFlow::Continue(Bucket::Discharged)
                            if [&readings.below, &readings.above]
                                .into_iter()
                                .any(|r| matches!(r, Some(Reading::Undecided(_)))) =>
                        {
                            receipt.refused += 1;
                        }
                        ControlFlow::Continue(bucket) => bucket.count(&mut receipt),
                    }
                    if q.answered() {
                        ended_at = Some(k);
                        break;
                    }
                    continue;
                }
                if injective && x.face == y.face && x.region == Region::Trimmed {
                    // **Injective by its carrier's kind**: one leaf, never
                    // subdivided.
                    match injective::by_kind(x.face, &x.surface, self.band) {
                        Ok(()) => {
                            receipt.discharged += 1;
                            receipt.by_kind += 1;
                        }
                        Err(r) => {
                            receipt.refused += 1;
                            if q.refuse_below(r).is_break() {
                                ended_at = Some(k);
                                break;
                            }
                        }
                    }
                    continue;
                }
                frontier.push(Task {
                    k,
                    i,
                    j,
                    pair: CellPair {
                        a,
                        b,
                        depth: 0,
                        fresh: true,
                    },
                    open,
                });
            }

            if let Some(k) = ended_at {
                receipt.abandoned += (candidates.len() - k - 1) + frontier.len();
                frontier.clear();
            }
            let (receipt, deepest) = walk(self.budgets, receipt, frontier, wa, wb, &mut q);
            let Bound {
                region,
                across,
                cornered,
                mut widths,
                lower_refusal,
                upper_refusal,
                violation,
                within,
                ..
            } = q;
            widths.deepest = deepest;
            // One count per window the sweep ran over, both sides: a query
            // against itself was handed two copies of one list and counts
            // each face twice, which is what "per window" means here.
            let decided = region.decided();

            let verdict = match (violation, lower_refusal, self.bound.upper(), within) {
                (Some(w), ..) => BoundVerdict::Violated(w),
                (None, Some(refusal), ..) => BoundVerdict::Refused(refusal),
                (None, None, None, _) => BoundVerdict::Holds,
                (None, None, Some(_), Some(w)) => BoundVerdict::Within(w),
                (None, None, Some(_), None) => {
                    upper_refusal.map_or(BoundVerdict::Exceeded, BoundVerdict::Refused)
                }
            };
            receipt.check_outside_is_a_sub_count();
            BoundReport {
                verdict,
                receipt,
                widths,
                across,
                cornered,
                decided,
            }
        })
    }
}

// ------------------------------------------------ the frontier walk

/// **What a walk asks of the question it answers**: what one cell pair
/// is, and what a level that does not fit the pair budget counts as.
/// The frontier, the budgets and the split are [`walk`]'s, the same for
/// both questions.
trait Question<T: Real> {
    /// Classify one cell pair of windows `x` and `y`: count it a leaf on
    /// `walk.receipt`, or hand it to [`Walk::split`]. `Break` when the
    /// query has its answer.
    fn pair(
        &mut self,
        walk: &mut Walk,
        task: Task,
        x: &Window<'_, T>,
        y: &Window<'_, T>,
    ) -> ControlFlow<()>;

    /// A level whose every task, at one leaf each, would not fit the pair
    /// budget: none of it is started, and each question counts it by what
    /// it means for that question's answer.
    fn overflow(&mut self, receipt: &mut CellReceipt, frontier: Vec<Task>);
}

/// One walk's running state, as a [`Question`] reads and extends it.
struct Walk {
    budgets: Budgets,
    receipt: CellReceipt,
    /// Tasks of the current level not yet classified, after the one
    /// being classified.
    left: usize,
    /// The next level.
    next: Vec<Task>,
}

impl Walk {
    /// **Split one cell pair into the `N` children `cut` yields, or
    /// refuse it** naming the bound that stopped it: the depth floor,
    /// then the pair budget — enforced AT ADMISSION, exactly as the
    /// driver's leaf budget is, so a split is refused unless the leaf
    /// count it commits to still fits — then the `f64` grid, where `cut`
    /// cannot bisect. A refusal is counted here; what it reports is the
    /// question's. `N` is `cut`'s array length, so the count admission
    /// prices is the count the split commits.
    fn split<const N: usize>(
        &mut self,
        task: Task,
        cut: impl FnOnce() -> Option<[CellPair; N]>,
    ) -> Result<(), CellBudget> {
        let r = &self.receipt;
        // What is already final, what this level has left to fold, and
        // what is queued for the next level.
        let committed = r.discharged + r.violated + r.refused + self.left + self.next.len();
        let bound = if task.pair.depth >= self.budgets.max_cell_depth {
            CellBudget::Depth {
                max_cell_depth: self.budgets.max_cell_depth,
            }
        } else if committed + N > self.budgets.max_cell_pairs {
            CellBudget::Pairs {
                max_cell_pairs: self.budgets.max_cell_pairs,
            }
        } else if let Some(children) = cut() {
            self.next.extend(children.map(|pair| Task { pair, ..task }));
            self.receipt.splits += N - 1;
            return Ok(());
        } else {
            CellBudget::Resolution
        };
        self.receipt.refused += 1;
        Err(bound)
    }
}

/// **The one subdivision both questions run**: a level-synchronous
/// frontier from `frontier`, each level checked whole against the pair
/// budget before it starts, each cell pair classified by `q`. Answers
/// the receipt and the deepest pair reached.
///
/// A `Break` from `q` ends the walk, and what the walk still held — the
/// rest of the level and the next one — is counted
/// [`CellReceipt::abandoned`].
fn walk<T: Real, Q: Question<T>>(
    budgets: Budgets,
    receipt: CellReceipt,
    mut frontier: Vec<Task>,
    wa: &[Window<'_, T>],
    wb: &[Window<'_, T>],
    q: &mut Q,
) -> (CellReceipt, u32) {
    let mut w = Walk {
        budgets,
        receipt,
        left: 0,
        next: Vec::new(),
    };
    let mut deepest = 0;
    while !frontier.is_empty() {
        // The whole-frontier budget check, the driver's own: every task
        // on it becomes at least one leaf, so when they cannot fit none
        // is started.
        let r = &w.receipt;
        if r.discharged + r.violated + r.refused + frontier.len() > budgets.max_cell_pairs {
            q.overflow(&mut w.receipt, frontier);
            break;
        }
        let mut tasks = frontier.into_iter();
        while let Some(task) = tasks.next() {
            w.left = tasks.len();
            let (Some(x), Some(y)) = (wa.get(task.i), wb.get(task.j)) else {
                // Unreachable: the frontier was built from these very
                // indices. Counted rather than skipped, so no path out
                // of this loop can break the receipt.
                w.receipt.abandoned += 1;
                continue;
            };
            deepest = deepest.max(task.pair.depth);
            if q.pair(&mut w, task, x, y).is_break() {
                w.receipt.abandoned += w.left + w.next.len();
                return (w.receipt, deepest);
            }
        }
        frontier = core::mem::take(&mut w.next);
    }
    (w.receipt, deepest)
}

/// **What a refusal of the lower bound does to the walk.**
///
/// A refusal is a finding. Whether the walk ends at one is the
/// question's policy, apart from what kind of question it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AtRefusal {
    /// The walk ends at its first finding, a refusal as a verified
    /// violation, and that is the verdict: the injectivity question's.
    /// What it would read on is the rest of a proof the refusal has
    /// already denied.
    End,
    /// The first refusal is kept and the walk reads on: a metric bound's.
    /// Its report is a requirement's, and a definite violation elsewhere
    /// outranks a pair the band or a budget stopped.
    ReadOn,
}

/// **Each side's window faces, by window index**: what a cell pair the
/// walk never examined is named by.
struct WindowFaces(Vec<FaceKey>, Vec<FaceKey>);

impl WindowFaces {
    fn of<T: Real>(wa: &[Window<'_, T>], wb: &[Window<'_, T>]) -> Self {
        Self(
            wa.iter().map(|w| w.face).collect(),
            wb.iter().map(|w| w.face).collect(),
        )
    }

    /// The two faces of `task`'s windows.
    fn of_task(&self, task: &Task) -> (FaceKey, FaceKey) {
        match (self.0.get(task.i), self.1.get(task.j)) {
            (Some(&a), Some(&b)) => (a, b),
            _ => unreachable!("a task's windows are the walk's"),
        }
    }
}

/// **The bound question, per cell pair**: what [`Sweep::run`] carries
/// across its walk.
struct Bound<'q, W, F> {
    sweep: &'q Sweep,
    /// Whether the question is injectivity: the strict bound over one
    /// body. Every other question is metric.
    injective: bool,
    /// What each candidate's two faces share, in M, by candidate.
    shared: &'q [Shared],
    /// What a refusal of the lower bound does to the walk.
    at_refusal: AtRefusal,
    /// What a level that does not fit is named by.
    faces: WindowFaces,
    region: RegionMemo,
    locals: LocalMemo,
    across: BTreeMap<(FaceKey, FaceKey), usize>,
    cornered: usize,
    widths: DischargeWidths,
    /// The lower bound's first refusal ([`Self::keep_below`]).
    lower_refusal: Option<EngineRefusal>,
    /// The upper bound's first refusal.
    upper_refusal: Option<EngineRefusal>,
    /// The lower bound's verified violation, which ends the sweep.
    violation: Option<W>,
    /// The upper bound's verified proof, which closes its question.
    within: Option<W>,
    /// The verifier's word on the last offer ([`Enclosure::box_dependent`]).
    box_dependent: core::cell::Cell<Option<(f64, f64)>>,
    verify: F,
}

/// **Which bounds a cell pair is still examined for.** A pair settled
/// against one bound and undecided against the other splits, and its
/// children carry only the open one: one subdivision, both questions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Open {
    below: bool,
    above: bool,
}

/// One cell pair's separation read against one bound.
enum Reading {
    /// Definitely clear of the bound: at or above a lower bound (above
    /// zero for the strict question), above an upper one.
    Clear,
    /// Definitely on the bound's answering side: below a lower bound
    /// (at zero too for the strict question), within an upper one. The
    /// verifier is offered the pair.
    Answers,
    /// The band holds it.
    Undecided(geom_core::Indeterminate),
}

/// One cell pair's readings, `None` for a bound it is not examined for.
struct Readings {
    below: Option<Reading>,
    above: Option<Reading>,
}

/// What a [`Candidate`] is built from, before its question is chosen.
struct Offer<'s, 'b, T: Real> {
    x: &'s Window<'b, T>,
    ca: Cell,
    ia: usize,
    y: &'s Window<'b, T>,
    cb: Cell,
    ib: usize,
    meeting: Option<(SharedCell, Stations, Stations)>,
}

impl<W, F> Bound<'_, W, F> {
    /// The pair's separation `d` read against each bound it is still
    /// examined for, each at its own funnel site.
    fn read(&self, open: Open, d: Interval) -> Readings {
        let (bound, band) = (self.sweep.bound, self.sweep.band);
        let below = (bound.lower().is_some() && open.below && self.violation.is_none())
            .then(|| self.below().reading(d, band));
        let above = (bound.upper().is_some() && open.above && self.within.is_none())
            .then(|| self.above().reading(d, band));
        Readings { below, above }
    }

    /// The lower bound's question, as a candidate is offered for it.
    fn below(&self) -> Ask {
        Ask::Below {
            bound: self
                .sweep
                .bound
                .lower()
                .unwrap_or_else(|| unreachable!("read only where a lower bound is examined")),
            strict: !self.sweep.bound.zero_discharges(),
        }
    }

    /// The upper bound's question.
    fn above(&self) -> Ask {
        Ask::Within {
            bound: self
                .sweep
                .bound
                .upper()
                .unwrap_or_else(|| unreachable!("read only where an upper bound is examined")),
        }
    }

    /// Whether every question the bound asks has its answer: a lower
    /// violation, or an upper bound's proof where there is no lower one.
    fn answered(&self) -> bool {
        self.violation.is_some() || (self.sweep.bound.lower().is_none() && self.within.is_some())
    }

    /// The refusal an undecided reading is, where refinement cannot
    /// decide it: a poison enclosure, or a terminal sliver. `None` where
    /// a split may.
    fn terminal(
        source: &geom_core::Indeterminate,
        a: FaceKey,
        b: FaceKey,
    ) -> Option<EngineRefusal> {
        if source.margin.is_invalid() {
            // A poison enclosure is not an indeterminacy refinement
            // could settle, and it is not a budget: it is geometry that
            // did not evaluate. Its own class.
            Some(EngineRefusal::PoisonEnclosure { a, b })
        } else {
            sliver(source).map(|predicate| EngineRefusal::Sliver { predicate, a, b })
        }
    }

    /// The lower bound's refusal, kept where it is the first.
    fn keep_below(&mut self, refusal: EngineRefusal) {
        self.lower_refusal.get_or_insert(refusal);
    }

    /// **A refusal of the lower bound on a cell pair the walk examined**:
    /// kept, and `Break` where the question ends at it ([`AtRefusal`]).
    /// The policy is read here and nowhere else. The pair's count is its
    /// site's: a split the walk refused has counted it, and every other
    /// site lands it in its [`Bucket`].
    fn refuse_below(&mut self, refusal: EngineRefusal) -> ControlFlow<()> {
        self.keep_below(refusal);
        match self.at_refusal {
            AtRefusal::End => ControlFlow::Break(()),
            AtRefusal::ReadOn => ControlFlow::Continue(()),
        }
    }

    /// **Offer a candidate to the verifier**, for `ask`.
    fn offer<T>(&mut self, at: &Offer<'_, '_, T>, ask: Ask) -> Result<W, String>
    where
        T: Real,
        F: FnMut(Candidate<'_, '_, T>) -> Result<W, String>,
    {
        self.box_dependent.set(None);
        (self.verify)(Candidate {
            x: at.x,
            ca: at.ca,
            ia: at.ia,
            y: at.y,
            cb: at.cb,
            ib: at.ib,
            region: &mut self.region,
            meeting: at.meeting.clone(),
            ask,
            box_dependent: &self.box_dependent,
        })
    }

    /// **Why the verifier declined an upper bound's pair**: the box
    /// decides it in part only ([`EngineRefusal::BoxDependent`]), or the
    /// verifier found no pair within it.
    fn upper_declined<T: Real>(&self, what: String, at: &Offer<'_, '_, T>) -> EngineRefusal {
        match self.box_dependent.take() {
            Some(separation) => EngineRefusal::BoxDependent {
                a: at.x.face,
                b: at.y.face,
                separation,
            },
            None => EngineRefusal::WitnessUnverified {
                what,
                a: at.x.face,
                b: at.y.face,
            },
        }
    }

    /// **The definite readings, settled**: a pair below the lower bound
    /// is offered as a violation, one within the upper bound as its
    /// proof. A candidate the verifier declines is
    /// [`EngineRefusal::WitnessUnverified`] for its question. `Break`
    /// where the sweep ends on the pair, with the bucket it lands in:
    /// violated where the violation is verified, refused where the
    /// verifier declined it and the question ends at a refusal
    /// ([`Self::refuse_below`]). Otherwise which of the two the interval
    /// pass classified.
    fn settle<T>(&mut self, r: &Readings, at: &Offer<'_, '_, T>) -> ControlFlow<Bucket, Bucket>
    where
        T: Real,
        F: FnMut(Candidate<'_, '_, T>) -> Result<W, String>,
    {
        let mut bucket = Bucket::Discharged;
        if matches!(r.below, Some(Reading::Answers)) {
            match self.offer(at, self.below()) {
                Ok(w) => {
                    self.violation = Some(w);
                    return ControlFlow::Break(Bucket::Violated);
                }
                Err(what) => {
                    let refusal = EngineRefusal::WitnessUnverified {
                        what,
                        a: at.x.face,
                        b: at.y.face,
                    };
                    if self.refuse_below(refusal).is_break() {
                        return ControlFlow::Break(Bucket::Refused);
                    }
                }
            }
            bucket = Bucket::Violated;
        }
        if matches!(r.above, Some(Reading::Answers)) {
            match self.offer(at, self.above()) {
                Ok(w) => self.within = Some(w),
                Err(what) => {
                    let refusal = self.upper_declined(what, at);
                    self.upper_refusal.get_or_insert(refusal);
                }
            }
            bucket.unless_settled(Bucket::Within);
        }
        ControlFlow::Continue(bucket)
    }
}

/// Which receipt bucket a leaf the interval pass settled lands in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bucket {
    Discharged,
    Violated,
    Within,
    Refused,
}

impl Bucket {
    /// Lands the leaf in `to` unless a reading already settled it in
    /// another bucket: a pair the lower bound violated stays violated,
    /// and one only the band holds goes where the upper bound put it.
    fn unless_settled(&mut self, to: Self) {
        if *self == Self::Discharged {
            *self = to;
        }
    }

    fn count(self, receipt: &mut CellReceipt) {
        match self {
            Self::Discharged => receipt.discharged += 1,
            Self::Violated => receipt.violated += 1,
            Self::Within => receipt.within += 1,
            Self::Refused => receipt.refused += 1,
        }
    }
}

impl<T, W, F> Question<T> for Bound<'_, W, F>
where
    T: Decide + CertifiedBounds,
    F: FnMut(Candidate<'_, '_, T>) -> Result<W, String>,
{
    fn pair(
        &mut self,
        walk: &mut Walk,
        task: Task,
        x: &Window<'_, T>,
        y: &Window<'_, T>,
    ) -> ControlFlow<()> {
        let Some(sh) = self.shared.get(task.k) else {
            // Unreachable: the frontier was built from these very
            // indices. Counted rather than skipped.
            walk.receipt.abandoned += 1;
            return ControlFlow::Continue(());
        };
        let (bound, band) = (self.sweep.bound, self.sweep.band);
        let pair = task.pair;
        // **Off the face, before anything is enclosed.** A cell
        // decided outside its face is a leaf, and the discharge
        // is VACUOUS: no separation is measured, nothing is
        // decided at the clearance site, and no discharge width
        // is recorded, because none was paid. The two roots
        // minus cells decided outside still contain the faces.
        if self.region.ask(false, task.i, x, pair.a) == CellRegion::Outside
            || self.region.ask(true, task.j, y, pair.b) == CellRegion::Outside
        {
            walk.receipt.discharged += 1;
            walk.receipt.outside += 1;
            return ControlFlow::Continue(());
        }
        let discharge = |receipt: &mut CellReceipt, widths: &mut DischargeWidths| {
            receipt.discharged += 1;
            let da = diameter(&cell_box(&pair.a.enc));
            let db = diameter(&cell_box(&pair.b.enc));
            widths.discharged_at(da.max(db));
        };

        // Whether the support test separates the pair, once it is read.
        let mut along: Option<bool> = None;

        // **Touching, in M.** The injectivity question decides a
        // touching pair by a local certificate, never by
        // distance.
        if self.injective {
            let same = x.face == y.face;
            let meet = same && injective::rects_meet((pair.a.u, pair.a.v), (pair.b.u, pair.b.v));
            // Two cells the distance test discharges hold no
            // common point, of M or of space: this selects that
            // test and decides nothing it would not. Disjoint
            // enclosures are not enough — two carriers that hold
            // a shared edge only to rounding enclose it an ulp
            // apart, and the distance test reads that within the
            // band, where only a certificate decides it. Read against
            // the strict bound's own lower end, which is zero.
            let c = bound
                .lower()
                .unwrap_or_else(|| unreachable!("the injectivity question is the strict bound's"));
            let apart = matches!(
                decide(
                    bound.predicate(),
                    Margin::of(separation(&pair.a.enc, &pair.b.enc) - c),
                    band
                ),
                Ok(Sign::Positive)
            );
            let apart = apart || {
                if meet || sh.is_empty() {
                    false
                } else {
                    let s = separated_by_support(pair, x, y, c, band);
                    along = Some(s);
                    s
                }
            };
            let near = if meet || sh.is_empty() || apart {
                None
            } else {
                let la = self.locals.get(false, task.i, x, pair.a, band);
                let lb = self.locals.get(true, task.j, y, pair.b, band);
                let near = Near::of(sh, &la, &lb);
                (!near.is_empty()).then_some((near, la, lb))
            };
            if meet || near.is_some() {
                let outcome = if meet {
                    let hull = (
                        (pair.a.u.0.min(pair.b.u.0), pair.a.u.1.max(pair.b.u.1)),
                        (pair.a.v.0.min(pair.b.v.0), pair.a.v.1.max(pair.b.v.1)),
                    );
                    if injective::monotone(&x.surface, hull.0, hull.1, band) {
                        Outcome::Certified { corner: false }
                    } else {
                        Outcome::Split {
                            arm: CertificateArm::Monotone,
                            shared: SharedCell::Face(x.face),
                        }
                    }
                } else if let Some((near, la, lb)) = &near {
                    let (px, py) = (piece_of(x, pair.a, la), piece_of(y, pair.b, lb));
                    if same {
                        injective::certify_self(&px, &py, near, band, &x.poses)
                    } else {
                        injective::certify_pair_in(&px, &py, near, band, (&x.poses, &y.poses))
                    }
                } else {
                    unreachable!("a touching pair meets or is near")
                };
                let outcome = match outcome {
                    Outcome::Cusp { shared } if pair.depth >= CUSP_DEPTH => {
                        Outcome::Refuse(EngineRefusal::Uncertified {
                            a: x.face,
                            b: y.face,
                            shared,
                            arm: CertificateArm::SecondOrder,
                        })
                    }
                    Outcome::Cusp { shared } => Outcome::Split {
                        arm: CertificateArm::SecondOrder,
                        shared,
                    },
                    other => other,
                };
                match outcome {
                    Outcome::Certified { corner } => {
                        discharge(&mut walk.receipt, &mut self.widths);
                        walk.receipt.certified += 1;
                        if x.face != y.face {
                            self.cornered += usize::from(corner);
                            *self
                                .across
                                .entry((x.face.min(y.face), x.face.max(y.face)))
                                .or_default() += 1;
                        }
                    }
                    Outcome::Refuse(r) => {
                        walk.receipt.refused += 1;
                        return self.refuse_below(r);
                    }
                    Outcome::Cusp { .. } => {
                        unreachable!("read as a split or a refusal above")
                    }
                    Outcome::Split { arm, shared } => {
                        // A cell paired with itself splits into its two
                        // halves each against itself and against each
                        // other: three pairs, two splits' worth of
                        // leaves. Each child roots a subtree the distance
                        // test may decide.
                        let itself = same && pair.a.same_cell(&pair.b);
                        let fresh = |pair: CellPair| CellPair {
                            fresh: true,
                            ..pair
                        };
                        let split = if itself {
                            walk.split(task, || split_self(pair, x).map(|c| c.map(fresh)))
                        } else {
                            walk.split(task, || {
                                split_by(pair, x, y, Axes::Metric)
                                    .map(|(p, q)| [fresh(p), fresh(q)])
                            })
                        };
                        if let Err(stop) = split {
                            return self.refuse_below(match stop {
                                CellBudget::Depth { .. } => EngineRefusal::Uncertified {
                                    a: x.face,
                                    b: y.face,
                                    shared,
                                    arm,
                                },
                                stop => EngineRefusal::Budget {
                                    stop,
                                    a: x.face,
                                    b: y.face,
                                },
                            });
                        }
                    }
                }
                return ControlFlow::Continue(());
            }
        }

        let pending = Open {
            below: task.open.below && self.violation.is_none(),
            above: task.open.above && self.within.is_none(),
        };
        if !pending.below && !pending.above {
            // Kept only for a question that has since been answered.
            walk.receipt.abandoned += 1;
            return ControlFlow::Continue(());
        }
        let mut r = self.read(pending, separation(&pair.a.enc, &pair.b.enc));
        // **Parted in depth, or apart along a direction.** Where the
        // boxes leave a reading indeterminate, the cells' supports may
        // still put the pair clear of that bound, and so may the two
        // faces' depths. Those are a fact of the faces, the same at
        // every cell, so they are read once: at the pair's root, the
        // two windows whole, which is the one pair of a candidate no
        // split has made (`depth` counts the splits above a pair).
        let root = pair.depth == 0;
        let support = |c: Interval| {
            (root && parted_in_depth(x, y, c, band, Interval::from_certified))
                || separated_by_support(pair, x, y, c, band)
        };
        if let (Some(Reading::Undecided(source)), Some(lo)) = (&r.below, bound.lower())
            && !source.margin.is_invalid()
            && along.unwrap_or_else(|| support(lo))
        {
            r.below = Some(Reading::Clear);
        }
        if let (Some(Reading::Undecided(source)), Some(hi)) = (&r.above, bound.upper())
            && !source.margin.is_invalid()
            && support(hi)
        {
            r.above = Some(Reading::Clear);
        }
        let at = Offer {
            x,
            ca: pair.a,
            ia: task.i,
            y,
            cb: pair.b,
            ib: task.j,
            meeting: None,
        };
        // Verified HERE, not after the sweep: a verified violation ends
        // the query, and an unverified one ends only the question that
        // ends at a refusal.
        let mut bucket = match self.settle(&r, &at) {
            ControlFlow::Break(bucket) => {
                bucket.count(&mut walk.receipt);
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(bucket) => bucket,
        };
        if self.answered() {
            bucket.count(&mut walk.receipt);
            return ControlFlow::Break(());
        }
        // **The exhibit arm.** An indeterminate margin is not a
        // verdict, and for the STRICT question it is very often not
        // reachable by refinement either: `d - 0` is the norm of a
        // difference, so it can never classify Negative, and the only
        // definite violation is a `Zero` — which needs both cells
        // narrower than ε, thirty halvings down from a metre window. A
        // witness, though, does not need the enclosure to be narrow: it
        // needs ONE pair of points, and so does an upper bound's proof.
        // So an indeterminate cell pair is offered to the verifier
        // before it is split, at the two places a probe is worth its
        // cost — the root of the subtree the distance test decides (a
        // candidate's root, or the first pair below a touching one),
        // where a contact, an overlap or a near pair shows up
        // immediately, and the depth floor, where the alternative is a
        // refusal.
        //
        // It only ever ADDS answers, each confirmed by the verifier: a
        // probe that finds nothing changes no answer, so no `Holds` and
        // no `Exceeded` can rest on it. A violation and a `Within` DO
        // rest on it, and they mean different things over a parameter
        // box: a violation is existential (one point of the box breaks
        // the lower bound, and the verifier's point is one), while
        // `Within` is universal (at EVERY point some pair is within the
        // upper bound). So an upper bound's pair is confirmed by the
        // verifier over every realization the lane covers
        // (`Enclosure::separation`), not at one point; a pair the box
        // decides in part only is refused `BoxDependent` here rather than
        // split, because a split narrows the cells and never the box.
        // That is what keeps it an exhibit rather than a sample.
        let probe_here = pair.fresh || pair.depth >= walk.budgets.max_cell_depth;
        let mut split = Open {
            below: false,
            above: false,
        };
        if let Some(Reading::Undecided(source)) = &r.below {
            if probe_here && let Ok(w) = self.offer(&at, self.below()) {
                walk.receipt.violated += 1;
                self.violation = Some(w);
                return ControlFlow::Break(());
            }
            match Self::terminal(source, x.face, y.face) {
                Some(refusal) => {
                    bucket = Bucket::Refused;
                    if self.refuse_below(refusal).is_break() {
                        bucket.count(&mut walk.receipt);
                        return ControlFlow::Break(());
                    }
                }
                None => split.below = true,
            }
        }
        if let Some(Reading::Undecided(source)) = &r.above
            && self.within.is_none()
        {
            let offered = if probe_here {
                Some(self.offer(&at, self.above()))
            } else {
                None
            };
            if let Some(Ok(w)) = offered {
                self.within = Some(w);
                if self.answered() {
                    walk.receipt.within += 1;
                    return ControlFlow::Break(());
                }
                bucket.unless_settled(Bucket::Within);
            } else if let Some(Err(what)) = offered
                && let refusal @ EngineRefusal::BoxDependent { .. } = self.upper_declined(what, &at)
            {
                self.upper_refusal.get_or_insert(refusal);
                bucket.unless_settled(Bucket::Refused);
            } else {
                match Self::terminal(source, x.face, y.face) {
                    Some(refusal) => {
                        self.upper_refusal.get_or_insert(refusal);
                        bucket.unless_settled(Bucket::Refused);
                    }
                    None => split.above = true,
                }
            }
        }
        if !split.below && !split.above {
            if bucket == Bucket::Discharged {
                discharge(&mut walk.receipt, &mut self.widths);
            } else {
                bucket.count(&mut walk.receipt);
            }
            return ControlFlow::Continue(());
        }
        // The injectivity question halves a cell along its length in
        // metres; every other question keeps the relative rule its
        // receipts were taken under.
        let axes = if self.injective {
            Axes::Metric
        } else {
            Axes::Relative
        };
        let task = Task {
            open: split,
            ..task
        };
        if let Err(stop) = walk.split(task, || split_by(pair, x, y, axes).map(|(lo, hi)| [lo, hi]))
        {
            let refusal = EngineRefusal::Budget {
                stop,
                a: x.face,
                b: y.face,
            };
            if split.above {
                self.upper_refusal.get_or_insert(refusal.clone());
            }
            if split.below {
                return self.refuse_below(refusal);
            }
        }
        ControlFlow::Continue(())
    }

    /// **A level that does not fit refuses the verdict and is counted
    /// abandoned**: the answer is a verdict over every cell pair, and
    /// these are pairs nobody examined, so each bound they were still
    /// open for refuses ([`CellBudget::Pairs`]), naming the first of
    /// them, and none is counted refused, which would say it was tried.
    /// The walk ends here whatever the question's policy.
    fn overflow(&mut self, receipt: &mut CellReceipt, frontier: Vec<Task>) {
        let refusal = |task: &Task| {
            let (a, b) = self.faces.of_task(task);
            EngineRefusal::Budget {
                stop: CellBudget::Pairs {
                    max_cell_pairs: self.sweep.budgets.max_cell_pairs,
                },
                a,
                b,
            }
        };
        let below = frontier.iter().find(|t| t.open.below).map(refusal);
        let above = frontier.iter().find(|t| t.open.above).map(refusal);
        if let Some(refusal) = below {
            self.keep_below(refusal);
        }
        if let Some(refusal) = above
            && self.within.is_none()
        {
            self.upper_refusal.get_or_insert(refusal);
        }
        receipt.abandoned += frontier.len();
    }
}

/// **A common point of two cells, sought**: Gauss–Newton on
/// `S_a(p) − S_b(q)` from the closest pair of the two station lattices,
/// each step the least-norm one (`J` is 3×4) with a vanishing damping
/// term, every iterate clamped into its cell. `None` where there is no
/// lattice or the search ends no closer than it began. It chooses a
/// candidate and decides nothing: the `f64` reading of the carriers'
/// enclosures it walks on is a heuristic's.
fn coincidence<T: Real>(
    x: &Window<'_, T>,
    ca: Cell,
    sa: &[(f64, f64)],
    y: &Window<'_, T>,
    cb: Cell,
    sb: &[(f64, f64)],
) -> Option<((f64, f64), (f64, f64))> {
    let mid = |i: Interval| geom::mid_param(i.lo(), i.hi());
    let jet = |s: &Surface<Interval>, (u, v): (f64, f64)| {
        let j = s.point_jet(u, v);
        (j.point.map(mid), j.du.map(mid), j.dv.map(mid))
    };
    let point = |s: &Surface<Interval>, uv| jet(s, uv).0;
    // The closest pair so far: its distance and its two stations.
    type Pair = (f64, (f64, f64), (f64, f64));
    let mut best: Option<Pair> = None;
    for &p in sa {
        let pa = point(&x.surface, p);
        for &q in sb {
            let d = (pa - point(&y.surface, q)).norm();
            if best.is_none_or(|(e, _, _)| d < e) {
                best = Some((d, p, q));
            }
        }
    }
    let (start, mut p, mut q) = best?;
    let clamp = |(u, v): (f64, f64), c: Cell| (u.clamp(c.u.0, c.u.1), v.clamp(c.v.0, c.v.1));
    let mut gap = start;
    for _ in 0..32 {
        let (pa, au, av) = jet(&x.surface, p);
        let (pb, bu, bv) = jet(&y.surface, q);
        let f = pa - pb;
        gap = f.norm();
        if !gap.is_finite() || gap == 0.0 {
            break;
        }
        // M = J·Jᵀ, damped; y = M⁻¹f; δ = −Jᵀy.
        let cols = [au, av, -bu, -bv];
        let mut m = [[0.0f64; 3]; 3];
        for c in cols {
            let c = [c.x, c.y, c.z];
            for (r, row) in m.iter_mut().enumerate() {
                for (k, slot) in row.iter_mut().enumerate() {
                    *slot += c[r] * c[k];
                }
            }
        }
        let trace = m[0][0] + m[1][1] + m[2][2];
        for (r, row) in m.iter_mut().enumerate() {
            row[r] += 1e-12 * trace;
        }
        let Some(yv) = solve3(m, [f.x, f.y, f.z]) else {
            break;
        };
        let yv = Vec3::new(yv[0], yv[1], yv[2]);
        let step = |c: Vec3<f64>| -c.dot(yv);
        p = clamp((p.0 + step(au), p.1 + step(av)), ca);
        q = clamp((q.0 - step(bu), q.1 - step(bv)), cb);
    }
    let (pa, _, _) = jet(&x.surface, p);
    let (pb, _, _) = jet(&y.surface, q);
    gap = gap.min((pa - pb).norm());
    ((pa - pb).norm() < start && gap.is_finite()).then_some((p, q))
}

/// `m·y = r` for a 3×3 `m`, by Cramer's rule; `None` where `m` is
/// singular in `f64`.
fn solve3(m: [[f64; 3]; 3], r: [f64; 3]) -> Option<[f64; 3]> {
    let det = |m: [[f64; 3]; 3]| {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    let d = det(m);
    if !(d.is_finite() && d != 0.0) {
        return None;
    }
    let column = |k: usize| {
        let mut n = m;
        for (row, &x) in n.iter_mut().zip(&r) {
            row[k] = x;
        }
        det(n) / d
    };
    Some([column(0), column(1), column(2)])
}

/// **The stations of the cell two scopes meet on**: a shared vertex's
/// chart point on each face, read off each face's own boundary images,
/// or, where no image carries it, its carrier's closed chart inverse
/// ([`Surface::chart_point`]); for one face, one of its vertices on both
/// sides. Empty where neither reads it, which leaves the verifier
/// nothing to confirm.
fn meeting_stations<T: Real>(
    x: &Window<'_, T>,
    y: &Window<'_, T>,
    shared: &Shared,
    cell: SharedCell,
    band: Band,
) -> (Stations, Stations) {
    let vertex = match cell {
        SharedCell::Face(_) => x.boundary.vertices.keys().next().copied(),
        _ => shared.vertex(&x.boundary),
    };
    let Some(v) = vertex else {
        return (Vec::new(), Vec::new());
    };
    let at = |w: &Window<'_, T>| -> Stations {
        w.boundary
            .edges
            .iter()
            .find_map(|e| {
                let end = e.ends.iter().position(|&k| k == v)?;
                let image = e.uses.first()?.image?;
                let p = image.get(end)?;
                let mid = |i: Interval| geom::mid_param(i.lo(), i.hi());
                Some(vec![(mid(p.x), mid(p.y))])
            })
            .or_else(|| {
                let p = *w.boundary.vertices.get(&v)?;
                let uv = w.surface.chart_point(p, band).ok()?.uv;
                let mid = |i: Interval| geom::mid_param(i.lo(), i.hi());
                Some(vec![(mid(uv.x), mid(uv.y))])
            })
            .unwrap_or_default()
    };
    (at(x), at(y))
}

/// **Each cell's local boundary**, memoized per (window, cell): the
/// question depends only on the face and the cell.
#[derive(Default)]
struct LocalMemo {
    seen: BTreeMap<(bool, usize, u64, u64, u64, u64), Local>,
}

impl LocalMemo {
    fn get<T: Real>(
        &mut self,
        side_b: bool,
        index: usize,
        w: &Window<'_, T>,
        c: Cell,
        band: Band,
    ) -> Local {
        let key = (
            side_b,
            index,
            c.u.0.to_bits(),
            c.u.1.to_bits(),
            c.v.0.to_bits(),
            c.v.1.to_bits(),
        );
        self.seen
            .entry(key)
            .or_insert_with(|| Local::of(&w.boundary, ball_of(&w.surface, c), band))
            .clone()
    }
}

/// One window's cell, as the injectivity arms read it.
fn piece_of<'a, T: Real>(w: &'a Window<'_, T>, c: Cell, local: &'a Local) -> Piece<'a> {
    Piece {
        face: w.face,
        surface: &w.surface,
        whole: w.region == Region::Whole,
        boundary: &w.boundary,
        u: c.u,
        v: c.v,
        local,
        ball: ball_of(&w.surface, c),
    }
}

/// A cell's ball: its centre station on the carrier and the reach that
/// holds the cell's enclosure.
fn ball_of(surface: &Surface<Interval>, c: Cell) -> Ball {
    let (u, v) = centre(c.u, c.v);
    Ball::holding(surface.point_eval(u, v), c.enc)
}

// ------------------------------------- the minimum-separation door

/// **A bracket on the minimum separation between two sides' FACES**,
/// over whatever parameter box the two bodies were evaluated over.
///
/// # What the two numbers mean, exactly
///
/// Write `M(p)` for the minimum distance between the two sides' trimmed
/// faces at parameter point `p`, and let the bodies have been evaluated
/// over a box `B`.
///
/// - `lo` is a certified LOWER bound on `inf{ M(p) : p ∈ B }`: the least
///   `separation_lo` over every cell pair the walk did not discharge,
///   together with the pairs it excluded at admission (by their own
///   `separation_lo`, which is what admission compared). A pair with a
///   cell decided [`CellRegion::Outside`] its face holds no point of it
///   and is discharged; every other pair, a cell the door cannot decide
///   included, stays in, so the cells still standing cover the faces.
/// - `hi`, when present, is an upper bound on `sup{ M(p) : p ∈ B }` up
///   to the band: the least, over every pair examined, of its two centre
///   stations' separation where each station is placed on its face
///   ([`Bracket::witness`]). A station placed `In` is a point of the
///   face; one placed `OnEdge` or `OnVertex` is a decision within the
///   band, and may lie up to the band outside the face. Absent when no
///   examined pair witnessed one.
///
/// So `[lo, hi]` encloses the faces' separation, its upper end up to the
/// band. The window a trimmed face is subdivided over is a superset of
/// the face, and no point of it farther than the band outside the face
/// reaches either end. One consequence, which is the kernel's ε model
/// and not a defect: an `AtLeast c` over faces whose separation is
/// within the band above `c` can read `Violated`.
///
/// # Budget-honest, and no width rule anywhere
///
/// Refinement narrows the bracket and can never falsify it: `lo` only
/// rises as cells shrink, `hi` only falls, and every intermediate state
/// is a true bracket. So a query that runs out of budget REPORTS — there
/// is no width threshold below which this door starts answering and
/// above which it refuses.
///
/// # How WIDE the bracket actually is
///
/// "Budget-honest" says the bracket never lies. It does not say the
/// bracket is tight, and on curved carriers it is not. Measured, two
/// cylinders whose axes are 1 m apart (editor-core's
/// `r2_m10_6_probes_interval::min_separation_brackets_a_curved_pair_at_every_budget`):
///
/// ```text
///   max_cell_pairs =   1   [1, 2.062]   width 1.062
///   max_cell_pairs =   4   [1, 2.062]   width 1.062
///   max_cell_pairs =  32   [1, 2.062]   width 1.062
///   max_cell_pairs = 512   [1, 1.500]   width 0.500   <- the default
/// ```
///
/// So at the shipped budget the upper end of a curved pair's bracket
/// is still ~50% above the truth. A planar neck does far better (~7%),
/// because a plane's window IS its cell. A consumer who needs a tight
/// upper bound on a curved pair must raise [`Budgets::max_cell_pairs`]
/// and pay for it; a consumer who needs a sound LOWER bound has one at
/// every budget, including the first.
///
/// # `hi` is an enclosure, not an attained point pair
///
/// Each term of the fold is an interval-enclosed distance between two
/// cells or two stations of the faces, which bounds the minimum from
/// above at every parameter point, so the fold is sound — but none of
/// them is a pair of points anyone evaluated, and the number should not
/// be reported as one.
///
/// # Cost, and where it is paid
///
/// One subdivision search of up to [`Budgets::max_cell_pairs`]
/// cell pairs per call. A caller that asks once per leaf of a drive pays
/// it once per leaf, and nothing here caps that.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinSeparation {
    lo: f64,
    hi: Option<f64>,
    receipt: CellReceipt,
    excluded: usize,
    decided: usize,
    meets: Option<Meets>,
}

/// **Two scopes that meet**: a face of each, and the cell of M they
/// share — one face in both scopes, or an edge or a vertex two faces of
/// one body bound on. Their separation is 0, read off topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meets {
    /// The first scope's face.
    pub a: FaceKey,
    /// The second scope's.
    pub b: FaceKey,
    /// The cell they share.
    pub shared: SharedCell,
}

impl MinSeparation {
    /// The lower bound on the faces' separation.
    pub fn lo(&self) -> f64 {
        self.lo
    }

    /// The upper bound on the faces' separation, when some examined pair
    /// witnessed one ([`Bracket::witness`]).
    pub fn hi(&self) -> Option<f64> {
        self.hi
    }

    /// The bracket as the interval it is, a containment-true enclosure
    /// of the faces' separation, when it has an upper end.
    pub fn enclosure(&self) -> Option<Interval> {
        self.hi.map(|hi| Interval::from_bounds(self.lo, hi))
    }

    /// The counting receipt of the sweep that produced it.
    pub fn receipt(&self) -> CellReceipt {
        self.receipt
    }

    /// How many candidate face pairs were excluded at admission — the
    /// ones priced into `lo` by their own root `separation_lo` rather
    /// than subdivided.
    pub fn excluded(&self) -> usize {
        self.excluded
    }

    /// How many region readings the bracket rests on: cells decided
    /// `Inside` or `Outside`, and centre stations a point door placed.
    /// They chose which pairs fed the bracket, and are a receipt line
    /// rather than a census row ([`k_stats::unlogged`]).
    pub fn decided(&self) -> usize {
        self.decided
    }

    /// **Where the two scopes meet**, when they do: the bracket is then
    /// `[0, 0]`, and this is its witness.
    pub fn meets(&self) -> Option<Meets> {
        self.meets
    }

    /// **The human form**: the bracket as numbers and what it cost.
    pub fn render(&self) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        let _ = match self.hi {
            Some(hi) => writeln!(
                s,
                "minimum separation of the faces ∈ [{}, {hi}] (width {:e}), certified over the \
                 whole parameter box",
                self.lo,
                hi - self.lo
            ),
            None => writeln!(
                s,
                "minimum separation of the faces ≥ {}, certified over the whole parameter box; \
                 no examined pair witnessed an upper end",
                self.lo
            ),
        };
        let r = self.receipt;
        let _ = writeln!(
            s,
            "  {} candidate pair(s) subdivided, {} excluded at admission; {} discharged \
             ({} of them vacuously, off the face), {} refused, {} split, {} abandoned; \
             {} region reading(s) decided",
            r.candidates,
            self.excluded,
            r.discharged,
            r.outside,
            r.refused,
            r.splits,
            r.abandoned,
            self.decided
        );
        let _ = writeln!(
            s,
            "  the bracket is honest at any budget: refinement narrows it and cannot \
             falsify it."
        );
        if let Some(m) = self.meets {
            let _ = writeln!(
                s,
                "  the scopes meet: faces {:?} and {:?} share {}, so the separation is 0",
                m.a,
                m.b,
                m.shared.payload()
            );
        }
        s
    }

    /// The goldening form: float-exact bits, every float as its exact
    /// bits so the text is a faithful image of the bracket.
    pub fn serialize(&self) -> String {
        let r = self.receipt;
        format!(
            "min_separation lo={:016x} hi={} excluded={} decided={}\nreceipt candidates={} \
             discharged={} violated={} refused={} splits={} abandoned={} outside={} \
             holds={}\n",
            self.lo.to_bits(),
            self.hi
                .map_or_else(|| "none".to_owned(), |hi| format!("{:016x}", hi.to_bits())),
            self.excluded,
            self.decided,
            r.candidates,
            r.discharged,
            r.violated,
            r.refused,
            r.splits,
            r.abandoned,
            r.outside,
            r.holds()
        ) + &self.meets.map_or_else(String::new, |m| {
            format!("meets {:?}/{:?} {}\n", m.a, m.b, m.shared.payload())
        })
    }
}

/// **The minimum-separation door**: what IS the smallest separation
/// between these two sides, as a certified bracket.
///
/// The [`Sweep`] beside it answers `separation ≥ c` — a trichotomy at
/// one bound. This is the same subdivision run as a branch-and-bound
/// instead: a cell pair whose `separation_lo` has risen above the best
/// upper bound found so far cannot hold the minimum, so it is dropped
/// rather than refined, and what is left converges on the pairs that
/// can.
///
/// `same_body` is [`Sweep::run`]'s: the two sides are faces of one body.
/// Two scopes of one body that meet — a face in both, or two faces that
/// share an edge or a vertex — are at separation 0, and the bracket is
/// `[0, 0]` with the shared cell named ([`MinSeparation::meets`]),
/// before anything is enclosed.
///
/// `band` is the one the faces' regions are read at; `budgets` is all
/// the walk is dialled by ([`Budgets::MIN_SEPARATION`] ships): no
/// accelerator switch, since the door is handed bodies already evaluated
/// over whatever parameter box their caller had.
///
/// # What decides, and what it is allowed to decide
///
/// The bracket itself classifies no margin: it takes minima of enclosure
/// endpoints, and the pruning comparison (`separation_lo ≥ hi`) is not a
/// decision either — the same distinction the module header draws about
/// the proximity tree's admission test: it decides which pairs are
/// EXAMINED, never what any of them means, and [`MinSeparation`]'s `lo`
/// is floored at `hi` so a pair dropped at the moment it tied cannot
/// narrow the bracket past the truth.
///
/// The region readings ([`crate::cell_region`] for a cell, the face's
/// point door for a station) do decide, at `band`, and every outcome of
/// each leaves a bracket true to the band: a cell read `Outside` holds
/// no point of its face, so it drops out of the lower end; a station
/// placed `In` is a point of its face, and one placed `OnEdge` or
/// `OnVertex` is within the band of it, so either may bound the upper
/// end; a cell the door cannot decide stays in the lower end, and a
/// station the door does not place stays out of the upper. They choose
/// which pairs feed the bracket and decide nothing a census must see,
/// and [`MinSeparation::decided`] counts them. Like [`Sweep::run`]'s,
/// every reading runs in [`k_stats::unlogged`]'s scope (audit row F41).
///
/// # Cost
///
/// The readings are taken only where they can change the answer: a
/// station pair that would lower the incumbent, and an `Outside` test on
/// a pair that would otherwise be split or floor the bracket. Even so
/// they are most of a call's work, and the `Outside` test, asked of
/// every pair the walk would split, is most of that. Measured on
/// M10-6's measured-minimum rows (debug build, one test at a time; the
/// measure is retired, the number is the walk's), a
/// drive over the neck takes about 2.4× the wall time the walk took
/// with no readings (0.9 s against 2.2 s), and the corpus row that
/// drives every registered assertion about 1.25×.
///
/// # Errors
///
/// [`MinSeparationRefusal`]: a carrier with no admitted window, a side
/// naming no face, and an enclosure that did not evaluate. Never a
/// width, and never a budget: a budget is what the bracket's width is
/// FOR.
pub fn min_separation<T: Decide + CertifiedBounds>(
    a: &Side<'_, T>,
    b: &Side<'_, T>,
    same_body: bool,
    band: Band,
    budgets: Budgets,
) -> Result<MinSeparation, MinSeparationRefusal> {
    k_stats::unlogged("F41", || {
        for (side, which) in [(a, Which::First), (b, Which::Second)] {
            if side.faces.is_empty() {
                return Err(MinSeparationRefusal::EmptyScope { side: which });
            }
        }
        let narrow = |refusal| match refusal {
            EngineRefusal::Unsupported {
                carrier,
                face,
                side,
            } => MinSeparationRefusal::Unsupported {
                carrier,
                face,
                side,
            },
            other => unreachable!("`Side::windows` refuses only `Unsupported`, not {other:?}"),
        };
        let wa = a.windows(Which::First).map_err(narrow)?.of;
        let wb = b.windows(Which::Second).map_err(narrow)?.of;
        if same_body {
            for x in &wa {
                for y in &wb {
                    let shared = Shared::of(&x.boundary, &y.boundary, x.face == y.face);
                    let cell = if x.face == y.face {
                        Some(SharedCell::Face(x.face))
                    } else {
                        shared.cell(&x.boundary)
                    };
                    if let Some(shared) = cell {
                        return Ok(MinSeparation {
                            lo: 0.0,
                            hi: Some(0.0),
                            receipt: CellReceipt::default(),
                            excluded: 0,
                            decided: 0,
                            meets: Some(Meets {
                                a: x.face,
                                b: y.face,
                                shared,
                            }),
                        });
                    }
                }
            }
        }
        let root_a: Vec<Cell> = wa.iter().map(Cell::of).collect();
        let root_b: Vec<Cell> = wb.iter().map(Cell::of).collect();
        // **The root pass is quadratic in the admitted faces, and the tree
        // is not used here.** The bound sweep excludes pairs by
        // `pairs_within(reach)`, where `reach` is the bound the question
        // names; this question names none, and the only honest reach is an
        // upper bound on the answer — which is computed FROM these very
        // root separations. So the pass that would parameterize the tree is
        // the pass the tree would save, and running both would be work for
        // its own sake. What the tree would have excluded is excluded here
        // by the same inequality, and priced into `lo` by each dropped
        // pair's own `separation_lo` rather than by a single reach.
        let roots = root_separations((&wa, &root_a), (&wb, &root_b), same_body)?;
        // The incumbent: the least upper bound on the FACES' separation the
        // root pairs witness ([`Bracket::witness`]), which is what makes the
        // very first bracket true.
        let mut q = Bracket {
            hi: None,
            floor: f64::INFINITY,
            poison: None,
            region: RegionMemo::at(band),
            faces: WindowFaces::of(&wa, &wb),
        };
        for (i, j, _) in &roots {
            if let (Some(x), Some(y), Some(&ca), Some(&cb)) =
                (wa.get(*i), wb.get(*j), root_a.get(*i), root_b.get(*j))
            {
                q.witness((*i, x, ca), (*j, y, cb));
            }
        }
        // Admission: a pair whose separation is entirely above the
        // incumbent cannot hold the minimum. It is priced into the floor by
        // its own `separation_lo` and never subdivided.
        let mut frontier: Vec<Task> = Vec::new();
        let mut excluded = 0usize;
        for (i, j, sep) in &roots {
            if q.above(sep.lo()) {
                q.floor = q.floor.min(sep.lo());
                excluded += 1;
                continue;
            }
            let (Some(&a), Some(&b)) = (root_a.get(*i), root_b.get(*j)) else {
                continue;
            };
            frontier.push(Task {
                k: 0,
                i: *i,
                j: *j,
                pair: CellPair {
                    a,
                    b,
                    depth: 0,
                    fresh: false,
                },
                open: Open {
                    below: true,
                    above: false,
                },
            });
        }
        let receipt = CellReceipt {
            candidates: frontier.len(),
            ..CellReceipt::default()
        };
        let (receipt, _) = walk(budgets, receipt, frontier, &wa, &wb, &mut q);
        if let Some(poison) = q.poison {
            return Err(poison);
        }
        let Bracket {
            hi, floor, region, ..
        } = q;

        // **The floor is capped at the incumbent**, and that cap is what
        // makes the branch-and-bound sound rather than merely plausible. A
        // pair dropped at `separation_lo ≥ hi` has a true minimum at least
        // `hi`, so it can only hold the answer when the answer IS `hi` —
        // and then `lo = hi` is the exact bracket rather than one that
        // walked past the truth. When every pair discharges, that is
        // precisely what comes out.
        let lo = hi.map_or(floor, |hi| floor.min(hi));
        // A face holds a point, and a cell holding a point of its face is
        // never decided `Outside`. Every root cell holds its whole face, and
        // the halves of a split cell hold its points between them, so some
        // pair holding points of both faces is refused, abandoned or pruned
        // by a witnessed `hi`, and each of those leaves `lo` finite.
        assert!(
            lo.is_finite(),
            "kernel defect: `min_separation` priced no pair into its lower end and witnessed no \
             upper end, so a region door decided `Outside` a cell holding a point of its face \
             (receipt {receipt:?})"
        );
        Ok(MinSeparation {
            lo,
            hi,
            receipt,
            excluded,
            decided: region.decided(),
            meets: None,
        })
    })
}

/// [`min_separation`]'s root pass: every pair of a window of `a` and one
/// of `b` with its root cells' separation, each unordered face pair once
/// where the scopes are `same_body`. A separation [`poisoned`] refuses.
fn root_separations<T: Real>(
    (wa, root_a): (&[Window<'_, T>], &[Cell]),
    (wb, root_b): (&[Window<'_, T>], &[Cell]),
    same_body: bool,
) -> Result<Vec<(usize, usize, Interval)>, MinSeparationRefusal> {
    // Two scopes of one body can carry a pair in both orders.
    let mut seen: BTreeSet<(FaceKey, FaceKey)> = BTreeSet::new();
    let mut roots: Vec<(usize, usize, Interval)> = Vec::new();
    for (i, x) in wa.iter().enumerate() {
        for (j, y) in wb.iter().enumerate() {
            if same_body && !seen.insert((x.face.min(y.face), x.face.max(y.face))) {
                continue;
            }
            let (Some(pa), Some(pb)) = (root_a.get(i), root_b.get(j)) else {
                continue;
            };
            let sep = separation(&pa.enc, &pb.enc);
            if poisoned(sep) {
                // Geometry that did not evaluate. Not an indeterminacy
                // refinement could settle and not a budget — the
                // bound sweep's own class, for the same reason.
                return Err(MinSeparationRefusal::PoisonEnclosure {
                    a: x.face,
                    b: y.face,
                });
            }
            roots.push((i, j, sep));
        }
    }
    Ok(roots)
}

/// **The minimum question, per cell pair**: the incumbent upper bound,
/// once some pair has witnessed one, the floor every pair not
/// discharged is priced into, the enclosure that did not evaluate, if
/// one did not, and the region readings the walk took.
struct Bracket {
    hi: Option<f64>,
    floor: f64,
    poison: Option<MinSeparationRefusal>,
    region: RegionMemo,
    /// What a poisoned abandoned pair is named by.
    faces: WindowFaces,
}

impl Bracket {
    /// Whether a pair whose cells are `lo` apart at the least cannot
    /// hold a smaller separation than the incumbent.
    fn above(&self, lo: f64) -> bool {
        self.hi.is_some_and(|hi| lo >= hi)
    }

    /// Whether either cell of a pair is decided outside its face.
    fn outside<T: Decide + CertifiedBounds>(
        &mut self,
        (i, x, a): (usize, &Window<'_, T>, Cell),
        (j, y, b): (usize, &Window<'_, T>, Cell),
    ) -> bool {
        self.region.ask(false, i, x, a) == CellRegion::Outside
            || self.region.ask(true, j, y, b) == CellRegion::Outside
    }

    /// **What a pair witnesses about the FACES' separation from above**,
    /// folded into the incumbent: the separation of its two centre
    /// stations, when each lies on its face. A station lies on its face
    /// when its cell is decided [`CellRegion::Inside`], whose verdict
    /// placed that very station `In`; not when the cell is decided
    /// `Outside`, whose verdict placed it `Out`; and otherwise when the
    /// face's point door places it `In`, `OnEdge` or `OnVertex`.
    ///
    /// `OnEdge` and `OnVertex` are decisions within the band: such a
    /// station lies on the face's boundary or up to the band outside it.
    /// So the station pair is the distance between two points within
    /// the band of the faces, enclosed over the whole parameter box, and
    /// it bounds the faces' minimum from above up to the band at every
    /// parameter point.
    fn witness<T: Decide + CertifiedBounds>(
        &mut self,
        (i, x, a): (usize, &Window<'_, T>, Cell),
        (j, y, b): (usize, &Window<'_, T>, Cell),
    ) {
        let (sa, sb) = (centre(a.u, a.v), centre(b.u, b.v));
        let at = |w: &Window<'_, T>, (u, v): (f64, f64)| enclosure(&w.surface, (u, u), (v, v));
        let stations = separation(&at(x, sa), &at(y, sb));
        // A region is read only where its answer could lower the
        // incumbent.
        if poisoned(stations) || self.hi.is_some_and(|hi| stations.hi() >= hi) {
            return;
        }
        let stations = stations.hi();
        let region = &mut self.region;
        let mut on = |side_b, index, w: &Window<'_, T>, c: Cell, station| match region
            .ask(side_b, index, w, c)
        {
            CellRegion::Inside => true,
            CellRegion::Outside => false,
            CellRegion::Undecided => Candidate::placed(region, side_b, index, w, station),
        };
        if on(false, i, x, a, sa) && on(true, j, y, b, sb) {
            self.hi = Some(stations);
        }
    }
}

impl<T: Decide + CertifiedBounds> Question<T> for Bracket {
    fn pair(
        &mut self,
        walk: &mut Walk,
        task: Task,
        x: &Window<'_, T>,
        y: &Window<'_, T>,
    ) -> ControlFlow<()> {
        let pair = task.pair;
        let sep = separation(&pair.a.enc, &pair.b.enc);
        if poisoned(sep) {
            self.poison = Some(MinSeparationRefusal::PoisonEnclosure {
                a: x.face,
                b: y.face,
            });
            return ControlFlow::Break(());
        }
        // Every examined pair may tighten the upper bound, INCLUDING the
        // ones dropped below: a pair that cannot hold the minimum still
        // witnesses a separation of the faces.
        self.witness((task.i, x, pair.a), (task.j, y, pair.b));
        if self.above(sep.lo()) {
            walk.receipt.discharged += 1;
            return ControlFlow::Continue(());
        }
        // **Off the face.** A cell decided outside its face holds no
        // point of it, so the pair is a leaf that neither floors the
        // bracket nor is refined.
        if self.outside((task.i, x, pair.a), (task.j, y, pair.b)) {
            walk.receipt.discharged += 1;
            walk.receipt.outside += 1;
            return ControlFlow::Continue(());
        }
        // A pair the walk cannot split — at the depth floor, past the
        // pair budget, or on the `f64` grid — is REFUSED: it was
        // examined, and its own `separation_lo` floors the bracket.
        if walk
            .split(task, || split(pair, x, y).map(|(lo, hi)| [lo, hi]))
            .is_err()
        {
            self.floor = self.floor.min(sep.lo());
        }
        ControlFlow::Continue(())
    }

    /// **The bracket ABANDONS a level that does not fit**: none of it
    /// was examined, so none of it is in the incumbent, and each pair's
    /// own `separation_lo` still floors the bracket — which is the whole
    /// of "honest at any budget". Not refused for the budget: this door
    /// refuses nothing for one. A pair whose separation is [`poisoned`]
    /// is refused as [`Self::pair`] refuses it.
    fn overflow(&mut self, receipt: &mut CellReceipt, frontier: Vec<Task>) {
        for task in frontier {
            let sep = separation(&task.pair.a.enc, &task.pair.b.enc);
            if poisoned(sep) {
                let (a, b) = self.faces.of_task(&task);
                self.poison
                    .get_or_insert(MinSeparationRefusal::PoisonEnclosure { a, b });
            }
            self.floor = self.floor.min(sep.lo());
            receipt.abandoned += 1;
        }
    }
}

/// Whether a cell pair's separation is no bracket the minimum may read:
/// one that does not certify, or one with an end that is not finite.
/// Behind [`window_of`]'s refusal a cell's enclosure is its certified
/// carrier's, so no admitted face is known to reach the first arm.
fn poisoned(sep: Interval) -> bool {
    !(sep.is_certified() && sep.lo().is_finite() && sep.hi().is_finite())
}

// ------------------------------------------------- carrier windows

/// A face's domain window: the carrier it sits on, lifted to the
/// interval scalar, the rectangle of carrier parameters the engine
/// subdivides, and the vertices that decide adjacency.
#[derive(Debug, Clone)]
struct Window<'b, T: Real> {
    face: FaceKey,
    /// What the window is of its face, which says who reads a cell's
    /// region.
    region: Region,
    /// The carrier, lifted, in the frame the question is read in.
    surface: Surface<Interval>,
    /// The carrier lifted in the body's own coordinates, where the side
    /// is read through a motion: what a cell's region is read against,
    /// beside the face's own boundary. `None` where `surface` is it.
    own: Option<Surface<Interval>>,
    u: (f64, f64),
    v: (f64, f64),
    /// **The body the face is read from**, which [`cell_region`](crate::cell_region::cell_region) reads
    /// the face's region off: a cell the door decides
    /// [`CellRegion::Outside`] holds no point of the FACE, so
    /// classifying it would classify a place the body does not occupy.
    body: &'b Body<T>,
    /// The face's boundary, lifted: what the injectivity question reads
    /// two faces' shared cells and a cell's local boundary off.
    boundary: Boundary,
    /// The face's poses, built once in the ask, when a cell is first read
    /// in one ([`injective::FacePoses`]).
    poses: injective::FacePoses,
}

/// **What a window is of its face.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Region {
    /// The window IS the face: an iso-bounded NURBS face's knot domain
    /// ([`iso_bounded_domain`]). Every cell of it lies in the closed
    /// face and every station of it is a point of the face, so a cell
    /// reads [`CellRegion::Inside`] and a station is admitted with no
    /// point-in-face door asked. A cell on the domain's edge holds
    /// boundary points of the face too, and every one of its points is
    /// still a point of the closed face, which is what the minimum walk
    /// reads off `Inside`: its centre station is a point of the face.
    Whole,
    /// The window is a superset of the face (every analytic carrier):
    /// [`cell_region`](crate::cell_region::cell_region) reads each cell, and [`station_region`] each
    /// station.
    Trimmed,
}

/// **The cell verdict is per (WINDOW, CELL); the sweep asks it per
/// PAIR.** This memo makes the difference cost nothing: a cell of
/// window `i` is paired with every admitted cell of the other side,
/// and the answer depends only on the window and the cell.
///
/// The KEY is the cell's `f64` bits, not its value: cell corners are
/// the split rule's own arithmetic, so two cells are the same cell
/// exactly when their four bounds are bit-identical, and a bitwise key
/// is total where a float comparison would have to decide something.
/// [`CellReceipt::outside`] still counts pairs.
struct RegionMemo {
    /// The band every cell and station is decided at: the sweep's.
    band: Band,
    seen: BTreeMap<(bool, usize, u64, u64, u64, u64), CellRegion>,
    /// The witness stations the point doors were asked to place, by the
    /// same bitwise key, and where they placed them.
    stations: BTreeMap<(bool, usize, u64, u64), Option<FaceContainment>>,
    /// Each window's boundary lifted, once: every cell of the window
    /// reads it.
    lifted: BTreeMap<(bool, usize), Vec<BoundaryStop>>,
}

impl RegionMemo {
    fn at(band: Band) -> Self {
        Self {
            band,
            seen: BTreeMap::new(),
            stations: BTreeMap::new(),
            lifted: BTreeMap::new(),
        }
    }

    fn ask<T: Decide + CertifiedBounds>(
        &mut self,
        side_b: bool,
        index: usize,
        w: &Window<'_, T>,
        c: Cell,
    ) -> CellRegion {
        let key = (
            side_b,
            index,
            c.u.0.to_bits(),
            c.u.1.to_bits(),
            c.v.0.to_bits(),
            c.v.1.to_bits(),
        );
        if let Some(&known) = self.seen.get(&key) {
            return known;
        }
        let region = match w.region {
            Region::Whole => CellRegion::Inside,
            Region::Trimmed => match w.body.face(w.face).map(|f| f.surface()) {
                None | Some(Err(_)) => CellRegion::Undecided,
                Some(Ok(stored)) => {
                    // The carrier is the window's own lift, which
                    // [`window_of`] admitted only where it certifies.
                    let boundary = self
                        .lifted
                        .entry((side_b, index))
                        .or_insert_with(|| lift_boundary(w.body, w.face));
                    cell_region_lifted(
                        w.body,
                        w.face,
                        (stored, w.own.as_ref().unwrap_or(&w.surface), boundary),
                        c.u,
                        c.v,
                        self.band,
                    )
                }
            },
        };
        self.seen.insert(key, region);
        region
    }

    /// Where the face's point door places the station `(u, v)` of
    /// window `index` ([`station_region`]).
    fn station<T: Decide + CertifiedBounds>(
        &mut self,
        side_b: bool,
        index: usize,
        w: &Window<'_, T>,
        (u, v): (f64, f64),
    ) -> Option<FaceContainment> {
        *self
            .stations
            .entry((side_b, index, u.to_bits(), v.to_bits()))
            .or_insert_with(|| station_region(w.body, w.face, (u, v), self.band))
    }

    /// How many readings of a face's region the answer rests on: cells
    /// decided `Inside` or `Outside`, and witness stations a point door
    /// placed.
    fn decided(&self) -> usize {
        let cells = self
            .seen
            .values()
            .filter(|&&r| r != CellRegion::Undecided)
            .count();
        cells + self.stations.values().filter(|p| p.is_some()).count()
    }
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

/// **A spindle sheet's canonical tube-angle range** `[−v*, v*]`, its
/// apexes' angle ([`geom::sheet_apex`]) taken at its widest over the
/// lifted datums, widened as [`full_latitude`] is. Evaluating a hair
/// past an apex lands a hair from it, on the torus's other sheet: the
/// widening can only ADD points to an enclosure.
fn sheet_latitude(major_radius: Interval, minor_radius: Interval) -> (f64, f64) {
    let apex = geom::sheet_apex(major_radius, minor_radius)
        .v
        .hi()
        .next_up();
    (-apex, apex)
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
/// - **Cone**: `v` is the signed slant arc length from the apex, and
///   `(p − apex)·axis = v·cos α`, so `v` is an affine function of
///   position with no critical point on the carrier but the apex: it
///   takes the boundary's range of that projection over `cos α`, widened
///   to the apex's `0` unless the boundary winds nowhere
///   ([`winds_nowhere`]): a face may hold the apex inside. A window
///   symmetric about the apex would hold the mirror
///   nappe, and on it halving `u` never narrows the enclosure (each
///   azimuth's generator meets its opposite's), so no subdivision could
///   decide anything. `u` takes the whole turn; a cone whose `cos α`
///   straddles zero has no finite window.
/// - **Sphere**: latitude has an interior extremum (a cap's boundary is
///   one circle while the region reaches the pole), so both coordinates
///   take their canonical ranges.
/// - **Torus**: both coordinates are periodic and no boundary bounds
///   either.
/// - **Spindle sheet**: as the sphere, `u` takes the whole turn and `v`
///   its canonical range between the apexes ([`sheet_latitude`]).
/// - **NURBS**: a face whose boundary is its chart's own isos, run
///   once around the knot domain — every loft and sweep wall — is the
///   whole patch, and its window is that domain
///   ([`iso_bounded_domain`]). A trimmed face, one with a boundary edge
///   that is not an iso, and one whose isos do not cover the domain's
///   boundary refuse [`EngineRefusal::Unsupported`] naming which.
/// - **An approximated face** is its fit, a NURBS face, and takes the
///   NURBS window.
///
/// The carrier is read in its STORED chart, for every carrier kind, and
/// lifted to the interval scalar ([`Interval::from_certified`]): a
/// plane's `u_ref` is the in-plane direction its construction handed
/// `newell_plane` (a swept segment, the sketch's `x` axis), read as
/// stored. `refines` is the door that refuses a chart subdivision
/// cannot narrow, whatever made it wide.
fn window_of<'b, T: Decide + CertifiedBounds>(
    body: &'b Body<T>,
    face: FaceKey,
    side: Which,
    at: Option<&RigidMotion<T>>,
) -> Result<Window<'b, T>, EngineRefusal> {
    let unsupported = |carrier| {
        Err(EngineRefusal::Unsupported {
            carrier,
            face,
            side,
        })
    };
    let Some(f) = body.face(face) else {
        return unsupported("an unreadable face");
    };
    let Ok(stored) = f.surface() else {
        return unsupported("the pending face");
    };
    let Some((cover, boundary)) = boundary_of(body, face) else {
        return unsupported("a face whose boundary carries no sound enclosure");
    };
    // The boundary's range of `(x − origin)·dir`, each piece's support
    // along `dir`. Read off the boundary's world box it would reach past
    // the boundary wherever `dir` is off the world's axes.
    let offset = |origin: Point3<Interval>, dir: Vec3<Interval>| -> Interval {
        let (lo, hi) = cover.range_along(dir);
        Interval::from_bounds(lo.lo(), hi.hi()) - (origin - Point3::origin()).dot(dir)
    };
    // The boundary's range of a chart parameter that the carrier
    // evaluates as `origin + dir·v`: `Line::param`, correct for any `dir`.
    let along = |origin: Point3<Interval>, dir: Vec3<Interval>| -> (f64, f64) {
        // `Line::param`'s coefficient, spelled out: `dir` is the chart's
        // own, not a unit direction.
        let d = offset(origin, dir) / dir.norm_squared();
        (d.lo(), d.hi())
    };
    let Some(charted) = carrier_lifted(stored) else {
        return unsupported("a face whose carrier does not certify");
    };
    let superset = |u, v| (u, v, Region::Trimmed);
    let (u, v, region) = match &charted {
        Surface::Plane { frame } => {
            let (origin, normal, u_ref) = (frame.origin(), frame.axis(), frame.u_ref());
            superset(along(origin, u_ref), along(origin, normal.cross(u_ref)))
        }
        Surface::Cylinder { frame, .. } => {
            let (origin, axis) = (frame.origin(), frame.axis());
            superset(full_turn(), along(origin, axis))
        }
        Surface::Cone {
            frame, half_angle, ..
        } => {
            let (apex, axis) = (frame.origin(), frame.axis());
            // `slant` encloses `v` over the boundary. The face's `v`
            // peaks on its boundary or at the apex, so `slant` widened
            // to `0` holds it, and `slant` alone does wherever it holds
            // `0` already. It does when the apex is on the boundary,
            // and when the boundary meets both nappes. Otherwise the
            // boundary lies on one nappe, off the apex; were the apex
            // inside the face, a small circle about it would be
            // homologous, on the punctured nappe, to the whole boundary,
            // which would wind once. So a boundary winding nowhere
            // leaves the apex off the face, or `0` inside `slant`.
            let slant = offset(apex, axis) / half_angle.cos();
            let v = if winds_nowhere(body, f) {
                (slant.lo(), slant.hi())
            } else {
                (slant.lo().min(0.0), slant.hi().max(0.0))
            };
            superset(full_turn(), v)
        }
        Surface::Sphere { .. } => superset(full_turn(), full_latitude()),
        Surface::Torus { .. } => superset(full_turn(), full_turn()),
        Surface::Spindle {
            major_radius,
            minor_radius,
            ..
        } => superset(full_turn(), sheet_latitude(*major_radius, *minor_radius)),
        Surface::Nurbs(net) => match iso_bounded_domain(body, f.record(), net) {
            Ok((u, v)) => (u, v, Region::Whole),
            Err(carrier) => return unsupported(carrier),
        },
        Surface::Approx(_) => unreachable!("an approximated face is read as its fit"),
    };
    if !(u.0.is_finite() && u.1.is_finite() && v.0.is_finite() && v.1.is_finite()) {
        return unsupported("a face whose carrier window is not a finite rectangle");
    }
    // Read through a motion, the window's carrier is the stored one
    // mapped into the question's frame, then lifted; its rectangle is the
    // stored one's, a rigid motion keeping every chart parameter.
    let (surface, own) = match at {
        None => (charted, None),
        Some(motion) => {
            let placed = crate::transform::surface_at(motion, stored)
                .ok()
                .and_then(|placed| carrier_lifted(&placed));
            let Some(placed) = placed else {
                return unsupported(
                    "a face whose carrier, read through its motion, does not certify",
                );
            };
            (placed, Some(charted))
        }
    };
    // The gate reads the carrier the window admits, the placed one where
    // the side is read through a motion.
    if !refines(&surface, u, v) {
        return unsupported(
            "a face whose interval chart does not refine — on one axis of its window, \
             neither half narrows the enclosure at one halving",
        );
    }
    Ok(Window {
        face,
        region,
        surface: surface.with_rect_memo(),
        own,
        u,
        v,
        body,
        boundary,
        poses: injective::FacePoses::default(),
    })
}

/// **Whether a face's boundary winds nowhere**: the elements of all its
/// joints, over every loop together, compose to the identity. A
/// missing joint, or an outer loop with no edges, reads as
/// winding; every hop of the walk is a link
/// ([`crate::live::FaceView::boundary_by_loop`]).
///
/// On a cone a joint on the apex stores the face's turn there
/// ([`crate::joint`]), so the composed `u` of the whole boundary is its
/// winding about the axis, a loop through the apex included. What a total of zero tells
/// [`window_of`] is argued there.
fn winds_nowhere<T: Decide>(body: &Body<T>, f: FaceView<'_, T>) -> bool {
    let mut elements = Vec::new();
    for (_, members, role) in f.boundary_by_loop() {
        for member in members {
            let Some(he) = member.half_edge() else {
                if role.is_outer() {
                    return false;
                }
                continue;
            };
            let Some(e) = body.joint(he) else {
                return false;
            };
            elements.push(e);
        }
    }
    crate::joint::Winding::of(elements).is_zero()
}

/// The crossing every lifted value of a face is read through, and the
/// one door that makes it ([`certified_lift`]): its field is private
/// here, so a reader taking a [`Lift`] cannot be handed one that asks
/// nothing.
mod lift {
    use geom_core::CertifiedBounds;
    use geom_core::interval::Interval;

    /// A crossing through [`Interval::from_certified`] whose refusals
    /// [`certified_lift`] records.
    pub(super) struct Lift<'a, T>(&'a dyn Fn(T) -> Interval);

    impl<T> Clone for Lift<'_, T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T> Copy for Lift<'_, T> {}

    impl<T> Lift<'_, T> {
        /// `x` crossed.
        pub(super) fn of(self, x: T) -> Interval {
            (self.0)(x)
        }
    }

    /// **`lift` with every scalar it reads crossed through
    /// [`Interval::from_certified`]**, `None` when any crossing does not
    /// certify: at `T = Interval` a refused value crosses with its
    /// ordinary endpoints under `Trv`, and the engine reads endpoints.
    pub(super) fn certified_lift<T: CertifiedBounds, X>(
        lift: impl FnOnce(Lift<'_, T>) -> X,
    ) -> Option<X> {
        let refused = core::cell::Cell::new(false);
        let cross = |x| {
            let i = Interval::from_certified(x);
            refused.set(refused.get() || !i.is_certified());
            i
        };
        let lifted = lift(Lift(&cross));
        (!refused.get()).then_some(lifted)
    }
}

/// **A face's carrier as the engine reads it, lifted**: a spline chart
/// as the NURBS surface it is, every other carrier as stored. An
/// approximated face's chart is its fit ([`Surface::spline_chart`]),
/// which is its geometry (OFFSET-DESIGN O2); the certified distance
/// bounds the fit from the surface it stands in for, a claim tier 3
/// re-derives, and says nothing about where the boundary is. That the
/// boundary lies on the fit is the face's own: tier 3 certifies each
/// edge against its chart image on the fit, and the window admits a
/// free-form face only where [`iso_bounded_domain`] finds those images
/// run once around the fit's knot domain. `None` where a crossing does
/// not certify ([`certified_lift`]).
fn carrier_lifted<T: CertifiedBounds>(stored: &Surface<T>) -> Option<Surface<Interval>> {
    certified_lift(|lift| match stored.spline_chart() {
        Some(net) => Surface::Nurbs(std::sync::Arc::new(net.map_scalar(|x| lift.of(x)))),
        None => stored.map_scalar(|x| lift.of(x)),
    })
}

/// A rectangle of carrier parameters, `(u, v)` ranges.
type ParamRect = ((f64, f64), (f64, f64));

/// **The window of a NURBS face bounded by its chart's own isos**: the
/// whole knot domain `[u0, u1] × [v0, v1]`, admitted only where the
/// face's boundary IS that domain's boundary, traversed once.
///
/// The test reads the face's one loop through its lift
/// ([`crate::pcurves::Lift::read`]): each half-edge's stored chart image,
/// moved by the joint elements summed along the loop, so the two halves
/// of a seam sit on the two columns they bound. It admits when
///
/// - the face has no ring, and every image is an iso by VARIANT
///   ([`Pcurve::IsoLine`], [`Pcurve::IsoArc`]): a straight segment of
///   the chart, a shape no scalar test has to decide;
/// - each image holds one chart coordinate at a bound of the exact
///   knot domain at both of its ends, so the whole segment lies on that
///   side. The held coordinate is exact structure by the variant (its
///   velocity is an exact zero); the moving one is read as the interval
///   the image evaluates to, since an edge's parameter span need not be
///   a point;
/// - each image's end meets the next one's start, as intervals within
///   a few ulps of their magnitude, so the images chain into one closed
///   curve on the domain's boundary;
/// - that curve winds once. Its shoelace is the winding times the
///   domain's area, so the interval quotient of the two encloses an
///   integer, and it is admitted only where that enclosure isolates
///   `±1`. The compare identifies an integer under interval bounds; it
///   decides nothing against a band.
///
/// A closed curve on a rectangle's boundary that winds once passes
/// every point of that boundary, so the loop is the whole boundary and
/// the face is the whole patch. Every other face refuses naming what
/// it found.
fn iso_bounded_domain<T: Decide + CertifiedBounds>(
    body: &Body<T>,
    f: &Face,
    net: &NurbsSurface<Interval>,
) -> Result<ParamRect, &'static str> {
    const NOT_ISO: &str = "a free-form face with a boundary edge that is not an iso of its chart";
    const NOT_COVERING: &str =
        "a free-form face whose iso boundary does not run once around its knot domain";
    if !f.rings.is_empty() {
        return Err("a trimmed free-form face (it carries an inner loop)");
    }
    let exact = |x: Interval| {
        (x.is_certified() && x.lo() == x.hi() && x.lo().is_finite()).then_some(x.lo())
    };
    let domain = |(lo, hi): (Interval, Interval)| Some((exact(lo)?, exact(hi)?));
    let (Some(u), Some(v)) = (
        domain(net.knot_values_u().domain()),
        domain(net.knot_values_v().domain()),
    ) else {
        return Err("a free-form face whose knot domain is not exact structure");
    };
    let rows = crate::pcurves::Lift::read(body, f.outer)
        .map_err(|_| "a free-form face whose boundary stores no chart image to lift")?;
    let on_side = |(x, y): (Interval, Interval)| {
        let (x, y) = (exact(x), exact(y));
        [
            x == Some(u.0),
            x == Some(u.1),
            y == Some(v.0),
            y == Some(v.1),
        ]
    };
    let mut path: Vec<[(Interval, Interval); 2]> = Vec::with_capacity(rows.rows().len());
    for row in &rows {
        if !matches!(&row.pcurve, Pcurve::IsoLine { .. } | Pcurve::IsoArc { .. }) {
            return Err(NOT_ISO);
        }
        let (t0, t1) = row.params();
        let point = |t: T| {
            let p = row.pcurve.eval(t);
            (Interval::from_certified(p.x), Interval::from_certified(p.y))
        };
        let (a, b) = (point(t0), point(t1));
        let (sa, sb) = (on_side(a), on_side(b));
        if !(0..4).any(|side| sa[side] && sb[side]) {
            return Err(NOT_COVERING);
        }
        let forward = crate::pcurves::is_plus(body, row.half_edge);
        path.push(if forward { [a, b] } else { [b, a] });
    }
    // Two ends meet where their enclosures overlap within a few ulps:
    // an image's end is its chart point evaluated at a parameter
    // (`p0 + pl·t`), whose rounding can leave the joint an ulp apart.
    let meet = |p: Interval, q: Interval| {
        let slack = |x: Interval| injective::few_ulps(x.lo().abs().max(x.hi().abs()));
        p.is_certified()
            && q.is_certified()
            && p.lo() - slack(p) <= q.hi()
            && q.lo() - slack(q) <= p.hi()
    };
    let closes = path
        .iter()
        .zip(path.iter().cycle().skip(1))
        .all(|(this, next)| meet(this[1].0, next[0].0) && meet(this[1].1, next[0].1));
    if path.is_empty() || !closes {
        return Err("a free-form face whose iso boundary does not close in its chart");
    }
    let at = |x: f64| Interval::from_bounds(x, x);
    let shoelace = path
        .iter()
        .map(|&[(xa, ya), (xb, yb)]| xa * yb - xb * ya)
        .fold(at(0.0), |acc, term| acc + term);
    let area = (at(u.1) - at(u.0)) * (at(v.1) - at(v.0));
    let winding = shoelace * at(0.5) / area;
    let isolates =
        |k: f64| winding.is_certified() && winding.lo() > k - 0.5 && winding.hi() < k + 0.5;
    if !(isolates(1.0) || isolates(-1.0)) {
        return Err(NOT_COVERING);
    }
    Ok((u, v))
}

/// Whether halving the window on either axis actually narrows the
/// carrier's enclosure — the door that turns a chart the subdivision
/// could not refine into a typed refusal rather than a budget burn.
///
/// BOTH axes must refine: a chart that refines on `u` alone gives a
/// subdivision that can only ever narrow one direction, which for a
/// two-dimensional window is not convergence. An axis refines when
/// EITHER of its halves [`narrows`] the whole window's box. One half
/// alone says nothing: every extreme of a wall can be attained in its
/// lower half, which then spans the whole box while the upper half
/// narrows. Inclusion isotony (an analytic carrier's interval
/// evaluation shrinks under sub-boxes; a spline chart's
/// [`Surface::rect_enclosure`] hulls a restricted net lying in its
/// parent's hull, up to the restriction's rounding) holds each half
/// inside the whole, so an axis neither of whose halves narrows is one
/// the subdivision gives no evidence of narrowing.
///
/// This is an admission rule, not a certificate: a chart admitted that
/// the subdivision then cannot decide costs depth or budget, which the
/// sweep refuses typed.
fn refines(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64)) -> bool {
    let boxed = |u, v| cell_box(&enclosure(surface, u, v));
    let whole = boxed(u, v);
    let (mu, mv) = (geom::mid_param(u.0, u.1), geom::mid_param(v.0, v.1));
    let axis = |lower: Aabb, upper: Aabb| narrows(&whole, &lower) || narrows(&whole, &upper);
    axis(boxed((u.0, mu), v), boxed((mu, u.1), v)) && axis(boxed(u, (v.0, mv)), boxed(u, (mv, v.1)))
}

/// Whether `half` moves a bound of `whole` inward. A box with a bound
/// that is not finite (poison, where the enclosure did not evaluate)
/// certifies nothing, so it gives no evidence of narrowing, on either
/// side of the comparison.
fn narrows(whole: &Aabb, half: &Aabb) -> bool {
    let finite = |b: &Aabb| {
        [b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z]
            .iter()
            .all(|x| x.is_finite())
    };
    finite(whole)
        && finite(half)
        && (half.max_x < whole.max_x
            || half.max_y < whole.max_y
            || half.max_z < whole.max_z
            || half.min_x > whole.min_x
            || half.min_y > whole.min_y
            || half.min_z > whole.min_z)
}

/// **Whether each pose of a sphere face holds `point` in its extent**
/// ([`injective::posed_extents_holding`]), the face read as the
/// certificate reads it. `None` where the face or its carrier does not
/// lift.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn posed_extents_holding(
    body: &Body<f64>,
    face: FaceKey,
    point: [f64; 3],
    band: Band,
) -> Option<Vec<Option<bool>>> {
    let stored = body.face(face)?.surface().ok()?;
    let (_, boundary) = boundary_of(body, face)?;
    let surface = certified_lift(|lift| stored.map_scalar(|x| lift.of(x)))?;
    let p = point.map(|x| Interval::from_bounds(x, x));
    Some(injective::posed_extents_holding(
        &surface,
        &boundary,
        Point3::new(p[0], p[1], p[2]),
        band,
    ))
}

/// The face's boundary region, and the boundary itself lifted.
///
/// The region is the hull of every boundary edge's region — its
/// lifted carrier's cover over its own span ([`crate::cover::arc_cover`],
/// so a curved edge's bulge is enclosed rather than chorded away, and a
/// spline edge's by its pieces' control nets) — together with the
/// boundary's vertex points, each lifted through
/// [`Interval::from_certified`]. `None` for a face that does not
/// resolve, one of whose edges carries no certified curve or no cover,
/// or one any of whose lifted values does not certify: an unclaimable
/// extent, never a silently smaller one. The walk past the face is
/// [`crate::live::FaceView::boundary_by_loop`]'s, every hop a link.
///
/// The lifted boundary carries each edge's carrier and span, its two
/// end vertices, and each of the face's half-edges on it with that
/// half-edge's chart image off the loop's lift
/// ([`crate::pcurves::Lift::read`]): its two end points and, where the
/// image is a coordinate line by variant, which coordinate it holds. A
/// loop with no lift carries no images, which leaves the face
/// unpresented rather than misread.
fn boundary_of<T: Decide + CertifiedBounds>(
    body: &Body<T>,
    face: FaceKey,
) -> Option<(Cover<Interval>, Boundary)> {
    certified_lift(|lift: Lift<'_, T>| -> Option<(Cover<Interval>, Boundary)> {
        let f = body.face(face)?;
        let point = |p: &Point3<T>| p.map(|x| lift.of(x));
        let mut pts: Vec<Point3<Interval>> = Vec::new();
        let mut arcs: Vec<Cover<Interval>> = Vec::new();
        let mut out = Boundary {
            sense: f.sense(),
            ..Boundary::default()
        };
        let mut halves: BTreeMap<EdgeKey, Vec<HalfEdgeKey>> = BTreeMap::new();
        let mut images: BTreeMap<HalfEdgeKey, Image> = BTreeMap::new();
        // The outer loop's half-edges in the walk's order; `None` once a
        // ring has edges.
        let mut outer: Option<Vec<HalfEdgeKey>> = Some(Vec::new());
        for (lk, members, role) in f.boundary_by_loop() {
            let mut cycle = false;
            for member in &members {
                let p = point(&body.member_point(member));
                out.vertices.insert(member.vertex(), p);
                pts.push(p);
                if let BoundaryMember::Edge { edge, he, .. } = *member {
                    let ek = edge.key();
                    halves.entry(ek).or_default().push(he);
                    cycle = true;
                    if role.is_outer() {
                        if let Some(o) = outer.as_mut() {
                            o.push(he);
                        }
                    } else {
                        outer = None;
                    }
                }
            }
            if cycle && let Ok(rows) = crate::pcurves::Lift::read(body, lk) {
                if role.is_outer() {
                    let surface = f.surface().ok()?;
                    let periodic =
                        [Axis::U, Axis::V].map(|axis| injective::periodic(surface, axis));
                    out.extent = extent_of(
                        periodic,
                        rows.iter().map(|row| {
                            let (t0, t1) = row.params();
                            let end = |t: T| {
                                let q = row.pcurve.eval(t);
                                [lift.of(q.x), lift.of(q.y)]
                            };
                            row_of(
                                &row.pcurve,
                                (lift.of(t0), lift.of(t1)),
                                [end(t0), end(t1)],
                                lift,
                            )
                        }),
                    );
                }
                for row in rows {
                    let (t0, t1) = row.params();
                    let end = |t: T| {
                        let q = row.pcurve.eval(t);
                        Point2::new(lift.of(q.x), lift.of(q.y))
                    };
                    images.insert(
                        row.half_edge,
                        (
                            [end(t0), end(t1)],
                            iso_of(&row.pcurve, lift),
                            harmonic_of(&row.pcurve, lift),
                            wall_of(&row.pcurve, lift),
                        ),
                    );
                }
            }
        }
        let edge_of: BTreeMap<HalfEdgeKey, EdgeKey> = halves
            .iter()
            .flat_map(|(&ek, hes)| hes.iter().map(move |&he| (he, ek)))
            .collect();
        for (ek, hes) in halves {
            let e = body.proven_edge(ek);
            let curve = e.certified()?;
            let (t0, t1) = curve.params();
            let span = (lift.of(t0), lift.of(t1));
            let carrier = curve.carrier().map_scalar(|x| lift.of(x)).with_span_memo();
            arcs.push(crate::cover::arc_cover(&carrier, span).ok()?);
            let ends = <[_; 2]>::from(crate::query::edge_ends(body, ek));
            let uses = hes
                .iter()
                .map(|he| {
                    let found = images.get(he);
                    Use {
                        plus: e.record().he_plus == *he,
                        image: found.map(|(image, ..)| *image),
                        harmonic: found.and_then(|(_, _, h, _)| *h),
                        wall: found.and_then(|(.., w)| *w),
                        iso: found
                            .and_then(|(_, iso, ..)| *iso)
                            .map(|(held, at, angle)| Iso {
                                held,
                                at,
                                full: angle && ends[0] == ends[1],
                            }),
                    }
                })
                .collect();
            out.edges.push(Rim {
                key: ek,
                stated: (T::WITNESS == geom_core::Witness::Inexact)
                    .then(|| curve.carrier().map_scalar(|x| Bounds::lo(x))),
                arc: Some((carrier, span)),
                ends,
                uses,
            });
        }
        out.outer = outer.filter(|o| !o.is_empty()).and_then(|o| {
            o.iter()
                .map(|&he| {
                    let ek = *edge_of.get(&he)?;
                    let i = out.edges.iter().position(|r| r.key == ek)?;
                    let e = proven(&body.edges, ek, EntityId::Edge);
                    Some((i, e.he_plus == he))
                })
                .collect()
        });
        let region = arcs.into_iter().fold(Cover::points(pts)?, Cover::hull);
        Some((region, out))
    })
    .flatten()
}

/// **One use of an edge, read off a chart image held at the interval
/// scalar**: [`boundary_of`]'s reading of a row, for an image no body
/// stores (a pose of a sphere's chart, `injective::present_posed`). Its
/// two ends at the edge's span `(t0, t1)`, and the coordinate line,
/// harmonic and wall images it is, by variant; `full` where its edge is
/// closed and the line holds an angle. `None` where a value it reads does
/// not certify.
pub(crate) fn chart_use(
    pcurve: &Pcurve<Interval>,
    (t0, t1): (Interval, Interval),
    plus: bool,
    closed: bool,
) -> Option<Use> {
    certified_lift(|lift: Lift<'_, Interval>| {
        let end = |t: Interval| {
            let q = pcurve.eval(t);
            Point2::new(lift.of(q.x), lift.of(q.y))
        };
        Use {
            plus,
            image: Some([end(t0), end(t1)]),
            harmonic: harmonic_of(pcurve, lift),
            wall: wall_of(pcurve, lift),
            iso: iso_of(pcurve, lift).map(|(held, at, angle)| Iso {
                held,
                at,
                full: angle && closed,
            }),
        }
    })
}

/// One row of a loop as [`extent_of`] reads it.
enum Row {
    /// A coordinate line whose moving coordinate is monotone
    /// ([`monotone_line`]): its two ends' chart points `[u, v]`, whose
    /// hull is the row's.
    Ends([[Interval; 2]; 2]),
    /// A harmonic or wall image that may bulge past its ends: its enclosure over
    /// its whole span, `[u, v]`, which holds it.
    Bulging([Interval; 2]),
}

/// **One row of a loop's lift as [`extent_of`] reads it**: a monotone
/// coordinate line by its `ends`, its chart points at `t0` and `t1`; a
/// harmonic or spiric wall image by its enclosure over its whole span,
/// its coefficients crossed by `lift`; `None` for any other image.
fn row_of<T: CertifiedBounds>(
    pcurve: &Pcurve<T>,
    (t0, t1): (Interval, Interval),
    ends: [[Interval; 2]; 2],
    lift: Lift<'_, T>,
) -> Option<Row> {
    if !monotone_line(pcurve, lift) {
        let t = Interval::from_bounds(t0.lo(), t1.hi());
        if let Some(w) = wall_of(pcurve, lift) {
            return Some(Row::Bulging(w.at(t)));
        }
        return Some(Row::Bulging(harmonic_of(pcurve, lift)?.at(t)));
    }
    Some(Row::Ends(ends))
}

/// **The face's own extent in its chart** ([`Boundary::extent`]): the
/// hull, on each axis, of the outer loop's lifted chain. A row that is a
/// monotone coordinate line contributes its ends, which are its hull; a
/// harmonic or wall row its enclosure over its span, which holds it, so the
/// extent is then a superset. A superset spanning a whole turn on an
/// axis `periodic` marks is not taken: a reader would read it as a loop
/// that wraps. On an axis that is not periodic no width means a wrap. Any
/// other row, or an end or enclosure not certified, is `None` (`None`
/// as an item: a row with no closed form), and no reader may take the
/// face's extent as known.
fn extent_of(
    periodic: [bool; 2],
    rows: impl Iterator<Item = Option<Row>>,
) -> Option<[(f64, f64); 2]> {
    let mut hull = [(f64::INFINITY, f64::NEG_INFINITY); 2];
    let mut bulged = false;
    for row in rows {
        let boxes: Vec<[Interval; 2]> = match row? {
            Row::Ends(ends) => ends.to_vec(),
            Row::Bulging(enclosure) => {
                bulged = true;
                vec![enclosure]
            }
        };
        for b in boxes {
            for (h, c) in hull.iter_mut().zip(b) {
                if !c.is_certified() {
                    return None;
                }
                *h = (h.0.min(c.lo()), h.1.max(c.hi()));
            }
        }
    }
    if bulged
        && hull
            .iter()
            .zip(periodic)
            .any(|(&(lo, hi), p)| p && injective::whole_turn(lo, hi))
    {
        return None;
    }
    hull.iter().all(|h| h.0 <= h.1).then_some(hull)
}

/// Whether a chart image is a coordinate line along which the moving
/// coordinate is monotone: a line or an iso arc, which move it
/// linearly, or a harmonic whose moving channel is an angle or a free
/// channel with no harmonic term (`c + l·t`). A free channel with one
/// (`c + a·cos t + b·sin t + l·t`) can turn back, so it is not.
fn monotone_line<T: CertifiedBounds>(p: &Pcurve<T>, lift: Lift<'_, T>) -> bool {
    let zero = |x: T| injective::exactly(Interval::from_certified(x), 0.0);
    let moves_monotone = |ch: &Channel<T>| match *ch {
        Channel::Angle { .. } => true,
        Channel::Free { a, b, .. } => zero(a) && zero(b),
    };
    match p {
        Pcurve::IsoLine { .. } | Pcurve::IsoArc { .. } => iso_of(p, lift).is_some(),
        Pcurve::Harmonic { u, v } => match iso_of(p, lift) {
            Some((Axis::U, _, _)) => moves_monotone(v),
            Some((Axis::V, _, _)) => moves_monotone(u),
            None => false,
        },
        _ => false,
    }
}

/// A coordinate line's held axis, its value, and whether its moving
/// coordinate is an angle ([`iso_of`]).
type IsoImage = (Axis, Interval, bool);

/// A half-edge's chart image as the boundary lifts it: its two end
/// points, the coordinate line it is, its harmonic form, and its spiric
/// wall form.
type Image = (
    [Point2<Interval>; 2],
    Option<IsoImage>,
    Option<injective::Harmonic>,
    Option<injective::Wall>,
);

/// **A harmonic chart image's coefficients, crossed by `lift`**
/// ([`injective::Harmonic`]); `None` for every other variant.
///
/// An angle channel's rate is its winding count, read off the count:
/// the rate is a ratio of two equal periods
/// ([`geom_brep::pcurve_cache::angle_rate`]), which an interval scalar
/// encloses wider than the integer it is.
fn harmonic_of<T: CertifiedBounds>(
    p: &Pcurve<T>,
    lift: Lift<'_, T>,
) -> Option<injective::Harmonic> {
    let Pcurve::Harmonic { u, v } = *p else {
        return None;
    };
    let channel = |ch: Channel<T>| match ch {
        Channel::Angle { phase, winding } => {
            let zero = Interval::from_f64(0.0);
            [
                lift.of(phase),
                zero,
                zero,
                Interval::from_f64(f64::from(winding)),
            ]
        }
        Channel::Free { c, a, b, l } => [c, a, b, l].map(|x| lift.of(x)),
    };
    let ([cu, au, bu, lu], [cv, av, bv, lv]) = (channel(u), channel(v));
    Some(injective::Harmonic {
        p0: [cu, cv],
        pa: [au, av],
        pb: [bu, bv],
        pl: [lu, lv],
    })
}

/// **A spiric's wall image, crossed by `lift`** ([`injective::Wall`]); `None` for
/// every other image, and for a wall whose `sense` does not lift to
/// exactly `±1`, which every minted image's does.
fn wall_of<T: CertifiedBounds>(p: &Pcurve<T>, lift: Lift<'_, T>) -> Option<injective::Wall> {
    let Pcurve::Spiric {
        major,
        minor,
        offset,
        image:
            SpiricImage::Wall {
                u0,
                v0,
                sense,
                torus,
            },
    } = *p
    else {
        return None;
    };
    let sigma = [1.0, -1.0]
        .into_iter()
        .find(|&s| injective::exactly(lift.of(sense), s))?;
    let mu = match torus {
        WallTorus::Own => 1.0,
        WallTorus::Mirror => -1.0,
    };
    Some(injective::Wall {
        u0: lift.of(u0),
        v0: lift.of(v0),
        sigma,
        kappa: sigma * mu,
        major: lift.of(major),
        minor: lift.of(minor),
        offset: lift.of(offset),
    })
}

/// **A chart image that is a coordinate line, by variant**: which
/// coordinate it holds, the value, and whether its moving coordinate is
/// an angle. An iso image states its line ([`Pcurve::held_line`]: the
/// coordinate is its variant's, nothing is read off a number). A
/// harmonic image holds a coordinate where that channel's coefficients
/// are the point zero, which is exact structure too, so no zero is
/// decided here. The value is crossed by `lift`.
fn iso_of<T: CertifiedBounds>(p: &Pcurve<T>, lift: Lift<'_, T>) -> Option<IsoImage> {
    let zero = |x: T| injective::exactly(Interval::from_certified(x), 0.0);
    match p {
        Pcurve::IsoLine { .. } | Pcurve::IsoArc { .. } => p
            .held_line()
            .map(|held| (held.line.axis.into(), lift.of(held.line.value), false)),
        Pcurve::Harmonic { u, v } => {
            let constant = |ch: &Channel<T>| match *ch {
                Channel::Angle { phase, winding: 0 } => Some(lift.of(phase)),
                Channel::Free { c, a, b, l } if zero(a) && zero(b) && zero(l) => Some(lift.of(c)),
                _ => None,
            };
            let angle = |ch: &Channel<T>| matches!(ch, Channel::Angle { .. });
            if let Some(at) = constant(u) {
                Some((Axis::U, at, angle(v)))
            } else {
                constant(v).map(|at| (Axis::V, at, angle(u)))
            }
        }
        _ => None,
    }
}

// --------------------------------------------------- the subdivision

/// One cell of a window: a sub-rectangle of its carrier parameters,
/// and its enclosure on the carrier, taken once when the cell is made
/// and read by every step that measures the cell.
#[derive(Debug, Clone, Copy)]
struct Cell {
    u: (f64, f64),
    v: (f64, f64),
    enc: Point3<Interval>,
}

impl Cell {
    fn of<T: Real>(w: &Window<'_, T>) -> Self {
        Self::on(&w.surface, w.u, w.v)
    }

    fn on(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64)) -> Self {
        Self {
            u,
            v,
            enc: enclosure(surface, u, v),
        }
    }

    /// Whether two cells are one: the split rule's own bits.
    fn same_cell(&self, other: &Self) -> bool {
        let bits = |c: &Self| {
            [
                c.u.0.to_bits(),
                c.u.1.to_bits(),
                c.v.0.to_bits(),
                c.v.1.to_bits(),
            ]
        };
        bits(self) == bits(other)
    }

    /// The relative width of each axis against the root window — the
    /// driver's split currency, so the two subdivisions choose axes by
    /// the same rule.
    fn relative<T: Real>(&self, root: &Window<'_, T>) -> (f64, f64) {
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
    /// The root of a subtree the distance test decides: a candidate's
    /// root, or the child of a touching pair. The exhibit arm probes it.
    fresh: bool,
}

/// One entry of a walk's frontier: which candidate pair it belongs to,
/// and the cell pair itself.
#[derive(Debug, Clone, Copy)]
struct Task {
    /// The candidate it descends from.
    k: usize,
    i: usize,
    j: usize,
    pair: CellPair,
    /// Which bounds it is examined for (the bound question's; the
    /// minimum question reads every pair alike).
    open: Open,
}

/// One cell's interval enclosure on its carrier.
fn enclosure(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64)) -> Point3<Interval> {
    surface.rect_enclosure(u, v)
}

/// A cell's enclosure as a box: the enclosure is already a box, folded
/// through the crate's one bracket reader. Poison where it did not
/// evaluate.
fn cell_box(enc: &Point3<Interval>) -> Aabb {
    Aabb::from_points([*enc]).unwrap_or_else(Aabb::poison)
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

/// **Whether two cells are apart by more than `c` along a direction**:
/// [`support_separation`] minus `c`, decided [`Sign::Positive`] at
/// [`CLEARANCE_SUPPORT_GAP`] (audit row F17). Only that sign is read:
/// the support gap bounds the separation from below and from nowhere
/// above, so no other sign says anything about the pair.
fn separated_by_support<T: Real>(
    pair: CellPair,
    x: &Window<'_, T>,
    y: &Window<'_, T>,
    c: Interval,
    band: Band,
) -> bool {
    support_separation((x, pair.a), (y, pair.b)).is_some_and(|d| {
        matches!(
            decide(CLEARANCE_SUPPORT_GAP, Margin::of(d - c), band),
            Ok(Sign::Positive)
        )
    })
}

/// **Whether two faces' depths part them by more than `c`**: a gap
/// between the two faces' depths about an axis, a centre or a core
/// circle one of them proposes ([`depth_gap::gaps_in_depth`]: nested
/// walls, a tube inside a tube, a pair no direction separates), minus
/// `c`, definitely positive at [`CLEARANCE_DEPTH_GAP`] (audit row F17).
/// Such a gap bounds the separation of the two faces from below, so it
/// clears a lower bound and an upper one alike; it bounds nothing from
/// above, and no other sign is read.
///
/// The depths are read off the faces' covers at the lane scalar, as
/// the cells' regions are, with no rounding charged
/// (`[simple.conditioned]`), and each gap is less the pads
/// [`depth_gap`] takes off it: a tolerance for each face's placement
/// misread within the band and its boundary's vertices within ε of it,
/// taken as the census takes it and not derived again for this reader.
/// `lift` crosses a gap to the bracket it is compared in; a gap that
/// does not certify clears nothing, since the funnel refuses its
/// margin. It is the caller's, which holds the certification bound
/// this function does not name, and the unit row passes one that
/// refuses.
fn parted_in_depth<T: Decide>(
    x: &Window<'_, T>,
    y: &Window<'_, T>,
    c: Interval,
    band: Band,
    lift: impl Fn(T) -> Interval,
) -> bool {
    // [`depth_gap::gaps_in_depth`]'s precondition: a side read through
    // a motion has its faces' covers in another frame than the
    // question's, so it is not asked, and its pairs are read by their
    // cells.
    if x.own.is_some() || y.own.is_some() {
        return false;
    }
    let read = |w: &Window<'_, T>| depth_gap::face_read(w.body, w.face, band);
    let (Some(rx), Some(ry)) = (read(x), read(y)) else {
        return false;
    };
    depth_gap::gaps_in_depth((x.body, x.face, &rx), (y.body, y.face, &ry), band)
        .any(|gap| definitely_positive(CLEARANCE_DEPTH_GAP, lift(gap) - c, band))
}

/// **What a cell's support along a direction is read off**: the
/// control points of its carrier restricted to it, an analytic
/// carrier's mean-value form about the cell's centre, or, for any net
/// [`NurbsSurface::rect_net`] refuses, the cell's box.
enum Hull {
    Net(Vec<Point3<Interval>>),
    /// The carrier's point at the cell's centre, its two partials
    /// enclosed over the whole cell, and the cell's offset from the
    /// centre along each axis.
    Jet {
        at: Point3<Interval>,
        du: Vec3<Interval>,
        dv: Vec3<Interval>,
        u: Interval,
        v: Interval,
    },
    Box(Point3<Interval>),
}

impl Hull {
    fn of(surface: &Surface<Interval>, c: Cell) -> Self {
        let Some(net) = surface.spline_chart() else {
            let span = |(lo, hi): (f64, f64)| Interval::from_bounds(lo, hi);
            let point = |x: f64| Interval::from_bounds(x, x);
            let (mu, mv) = centre(c.u, c.v);
            let j = surface.jet(span(c.u), span(c.v));
            return Self::Jet {
                at: surface.point_eval(mu, mv),
                du: j.du,
                dv: j.dv,
                u: span(c.u) - point(mu),
                v: span(c.v) - point(mv),
            };
        };
        match net.rect_net(c.u, c.v) {
            Some(points) => Self::Net(points),
            None => Self::Box(c.enc),
        }
    }

    /// An enclosure of `⟨P, n⟩` over every point `P` of the cell.
    ///
    /// With every restricted weight certified positive (which
    /// [`NurbsSurface::rect_net`] requires), each point of the cell is a
    /// convex combination of one knot cell's restricted points, so the
    /// linear `⟨P, n⟩` lies in the hull of its values at them.
    ///
    /// An analytic chart is smooth, and the cell is convex, so for each
    /// `(u, v)` of the cell the mean-value theorem, applied to
    /// `f = ⟨S, n⟩` along the segment from the centre `m`, gives a point
    /// `ξ` of the cell with
    /// `f(u, v) = f(m) + ⟨S_u(ξ), n⟩·(u − m_u) + ⟨S_v(ξ), n⟩·(v − m_v)`.
    /// [`Surface::jet`] at the cell's two intervals encloses `S_u` and
    /// `S_v` at every point of the cell, `ξ` among them, and the offsets
    /// lie in the cell less its centre, so the interval form holds
    /// `f(u, v)`. Along the cell's own normal both partial terms vanish
    /// at `m`, which leaves them second order in the cell. A box holds
    /// the cell, so its own support does.
    fn support(&self, n: Vec3<Interval>) -> Interval {
        let dot = |p: Vec3<Interval>| p.x * n.x + p.y * n.y + p.z * n.z;
        let at = |p: Point3<Interval>| dot(Vec3::new(p.x, p.y, p.z));
        match *self {
            Self::Net(ref points) => points
                .iter()
                .map(|&p| at(p))
                .reduce(geom_core::SpanLocate::enclosure_hull)
                .unwrap_or_else(|| unreachable!("`rect_net` answers no empty net")),
            Self::Jet {
                at: p,
                du,
                dv,
                u,
                v,
            } => at(p) + u * dot(du) + v * dot(dv),
            Self::Box(enc) => at(enc),
        }
    }
}

/// **A lower bound on the distance between two cells, read along a
/// direction**: with `n ≠ 0`, every point `p` of one cell and `q` of the
/// other satisfy `|p − q| ≥ ⟨p − q, n⟩ / |n|`, so the gap between the
/// two cells' supports along `n` over `|n|` bounds their separation
/// from below, whichever way `n` points. `n` is the best of three
/// directions by that bound — each cell's normal at its centre, and the
/// line between the two boxes' centres — and only chooses: the bound is
/// true along any `n`, so `n` itself is not certified. The raw compares
/// here are that choice and each direction's choice of orientation;
/// neither decides anything. `None` where no direction is finite.
fn support_separation(
    (x, a): (&Window<'_, impl Real>, Cell),
    (y, b): (&Window<'_, impl Real>, Cell),
) -> Option<Interval> {
    let mid = |i: Interval| geom::mid_param(i.lo(), i.hi());
    let normal = |s: &Surface<Interval>, c: Cell| {
        let (u, v) = centre(c.u, c.v);
        let n = s.point_normal(u, v);
        Vec3::new(mid(n.x), mid(n.y), mid(n.z))
    };
    let centres = Vec3::new(
        mid(b.enc.x) - mid(a.enc.x),
        mid(b.enc.y) - mid(a.enc.y),
        mid(b.enc.z) - mid(a.enc.z),
    );
    let (ha, hb) = (Hull::of(&x.surface, a), Hull::of(&y.surface, b));
    [normal(&x.surface, a), normal(&y.surface, b), centres]
        .into_iter()
        .filter(|n| n.checked_nonzero_length().is_ok())
        .filter_map(|n| {
            let n = n.map(Interval::from_f64);
            let (sa, sb) = (ha.support(n), hb.support(n));
            if poisoned(sa) || poisoned(sb) {
                return None;
            }
            let gap = |lo: Interval, hi: Interval| {
                (Interval::from_f64(lo.lo()) - Interval::from_f64(hi.hi())) / n.norm()
            };
            let (ab, ba) = (gap(sb, sa), gap(sa, sb));
            Some(if ab.lo() >= ba.lo() { ab } else { ba })
        })
        .reduce(|p, q| if q.lo() > p.lo() { q } else { p })
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
fn split<T: Real>(
    pair: CellPair,
    x: &Window<'_, T>,
    y: &Window<'_, T>,
) -> Option<(CellPair, CellPair)> {
    split_by(pair, x, y, Axes::Relative)
}

/// How a split chooses the axis it halves.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Axes {
    /// The axis of greatest relative width against the root window: the
    /// distance test's rule ([`split`]).
    Relative,
    /// The axis of greatest extent in metres at the cell's centre: a
    /// touching pair's, whose certificates fail on what the surface does
    /// along its length, not across a narrow strip. Where the chart
    /// collapses in the cell — an axis whose speed at one of the cell's
    /// nine stations is under an eighth of its most, as at a cone's apex
    /// — the relative rule instead: there the length across shrinks with
    /// the length along, so metres alone would only ever halve toward
    /// the collapse and never narrow the turn the certificates need.
    Metric,
}

/// The axis `cell` of `root` is halved on: `true` for `v`.
fn split_axis<T: Real>(cell: Cell, root: &Window<'_, T>, axes: Axes) -> bool {
    let (ru, rv) = match axes {
        Axes::Relative => cell.relative(root),
        Axes::Metric => {
            let stations = |(lo, hi): (f64, f64)| [lo, geom::mid_param(lo, hi), hi];
            let (mut u_least, mut u_most) = (f64::INFINITY, 0.0f64);
            let (mut v_least, mut v_most) = (f64::INFINITY, 0.0f64);
            for u in stations(cell.u) {
                for v in stations(cell.v) {
                    let j = root.surface.point_jet(u, v);
                    let (su, sv) = (j.du.norm().hi(), j.dv.norm().hi());
                    (u_least, u_most) = (u_least.min(su), u_most.max(su));
                    (v_least, v_most) = (v_least.min(sv), v_most.max(sv));
                }
            }
            if 8.0 * u_least < u_most || 8.0 * v_least < v_most {
                cell.relative(root)
            } else {
                let j = root.surface.point_jet(
                    geom::mid_param(cell.u.0, cell.u.1),
                    geom::mid_param(cell.v.0, cell.v.1),
                );
                (
                    j.du.norm().hi() * (cell.u.1 - cell.u.0),
                    j.dv.norm().hi() * (cell.v.1 - cell.v.0),
                )
            }
        }
    };
    rv.total_cmp(&ru) == core::cmp::Ordering::Greater
}

/// [`split`], with the axis chosen by `axes`.
fn split_by<T: Real>(
    pair: CellPair,
    x: &Window<'_, T>,
    y: &Window<'_, T>,
    axes: Axes,
) -> Option<(CellPair, CellPair)> {
    let da = diameter(&cell_box(&pair.a.enc));
    let db = diameter(&cell_box(&pair.b.enc));
    // `total_cmp` so the choice stays total on a poison diameter;
    // strictly-greater keeps the tie on the first cell.
    let split_b = db.total_cmp(&da) == core::cmp::Ordering::Greater;
    let (cell, root) = if split_b { (pair.b, y) } else { (pair.a, x) };
    let axis_v = split_axis(cell, root, axes);
    let (lo, hi) = if axis_v { cell.v } else { cell.u };
    let mid = geom::mid_param(lo, hi);
    if !(lo < mid && mid < hi) {
        return None;
    }
    let halves = |lo: f64, mid: f64, hi: f64| {
        let on = |u, v| Cell::on(&root.surface, u, v);
        if axis_v {
            (on(cell.u, (lo, mid)), on(cell.u, (mid, hi)))
        } else {
            (on((lo, mid), cell.v), on((mid, hi), cell.v))
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
                fresh: false,
            },
            CellPair {
                a: pair.a,
                b: second,
                depth,
                fresh: false,
            },
        )
    } else {
        (
            CellPair {
                a: first,
                b: pair.b,
                depth,
                fresh: false,
            },
            CellPair {
                a: second,
                b: pair.b,
                depth,
                fresh: false,
            },
        )
    })
}

/// **A cell paired with itself, split**: the cell halved on its
/// metrically longer axis ([`Axes::Metric`]), and the three pairs of its
/// halves — each against itself and the two against each other. `None`
/// where the axis cannot be bisected.
fn split_self<T: Real>(pair: CellPair, w: &Window<'_, T>) -> Option<[CellPair; 3]> {
    let cell = pair.a;
    let axis_v = split_axis(cell, w, Axes::Metric);
    let (lo, hi) = if axis_v { cell.v } else { cell.u };
    let mid = geom::mid_param(lo, hi);
    if !(lo < mid && mid < hi) {
        return None;
    }
    let on = |u, v| Cell::on(&w.surface, u, v);
    let (p, q) = if axis_v {
        (on(cell.u, (lo, mid)), on(cell.u, (mid, hi)))
    } else {
        (on((lo, mid), cell.v), on((mid, hi), cell.v))
    };
    let depth = pair.depth + 1;
    let of = |a, b| CellPair {
        a,
        b,
        depth,
        fresh: false,
    };
    Some([of(p, p), of(p, q), of(q, q)])
}

// ------------------------------------------------------ the lane door

/// **The engine's door, as a value the document layer takes rather
/// than a trait a scalar implements**: [`crate::QuadLane`]'s shape, for
/// its reason. The layers above it stay generic over every evaluation
/// scalar and read the measure door off
/// [`crate::AtRestPolicy::clearance_lane`], the per-scalar policy home,
/// refusing typed where it answers `None`; the verdict door
/// ([`embedded`]) holds one at every scalar with certification rights.
///
/// Holding one is the statement that the scalar may certify: the one
/// constructor ([`ClearanceLane::certified`]) is bounded on
/// [`CertifiedBounds`], since the lane holds the one crossing every
/// stored value enters the engine through, [`Interval::from_certified`]
/// (this module's header). The doors it holds take that crossing
/// themselves, and a verifier reads a candidate's carriers already
/// crossed ([`Candidate::enclosure`]), so no caller spells a lift. A
/// scalar that does not certify has no such type ([`crate::QuadLane`]'s
/// paragraph):
///
/// ```compile_fail,E0277
/// use geom_core::Dual64;
/// use topo::ClearanceLane;
/// let _ = ClearanceLane::<Dual64>::certified();
/// ```
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct ClearanceLane<T: geom_core::Certifies> {
    /// [`Side::windows`] (`wiring_rows` pins the pointer).
    windows: for<'b> fn(
        &Side<'b, T>,
        Which,
        Option<&RigidMotion<T>>,
    ) -> Result<Windows<'b, T>, EngineRefusal>,
    /// [`Sweep::run`], with the verifier's witness kept by the caller
    /// ([`ClearanceLane::sweep`]).
    bound_sweep: fn(
        &Sweep,
        &Windows<'_, T>,
        &Windows<'_, T>,
        bool,
        &mut dyn FnMut(Candidate<'_, '_, T>) -> Result<(), String>,
    ) -> BoundReport<()>,
    /// [`Candidate::on_face_stations`].
    face_stations: fn(Candidate<'_, '_, T>) -> (Stations, Stations),
}

impl<T: geom_core::Certifies + CertifiedBounds> ClearanceLane<T> {
    /// The certified engine: the whole inventory of this door, and the
    /// only constructor there is.
    #[must_use]
    pub const fn certified() -> Self {
        Self {
            windows: Self::windows_of,
            bound_sweep: Self::sweep_of,
            face_stations: Self::stations_of,
        }
    }

    fn windows_of<'b>(
        side: &Side<'b, T>,
        which: Which,
        at: Option<&RigidMotion<T>>,
    ) -> Result<Windows<'b, T>, EngineRefusal> {
        side.windows_at(which, at)
    }

    fn sweep_of(
        sweep: &Sweep,
        wa: &Windows<'_, T>,
        wb: &Windows<'_, T>,
        same_body: bool,
        verify: &mut dyn FnMut(Candidate<'_, '_, T>) -> Result<(), String>,
    ) -> BoundReport<()> {
        sweep.run(wa, wb, same_body, verify)
    }

    fn stations_of(candidate: Candidate<'_, '_, T>) -> (Stations, Stations) {
        candidate.on_face_stations()
    }
}

/// **A [`ClearanceLane`] as a pass that runs at every scalar holds it**:
/// the lane at a scalar that certifies, [`geom_core::NoLane`] (no
/// values) at one that does not (`geom-core` README, DL5.3). Sealed ([`crate::RegionLaneOf`]'s
/// seal); every method is written out below, so the seal is what
/// refuses it:
///
/// ```compile_fail,E0277
/// use geom_core::RigidMotion;
/// use topo::clearance::{BoundReport, Candidate, EngineRefusal, Side, Stations, Sweep, Which, Windows};
/// #[derive(Clone, Copy)]
/// struct Mine;
/// impl topo::ClearanceLaneOf<f64> for Mine {
///     fn windows<'b>(self, _s: &Side<'b, f64>, _w: Which) -> Result<Windows<'b, f64>, EngineRefusal> {
///         unimplemented!()
///     }
///     fn windows_at<'b>(
///         self,
///         _s: &Side<'b, f64>,
///         _w: Which,
///         _at: Option<&RigidMotion<f64>>,
///     ) -> Result<Windows<'b, f64>, EngineRefusal> {
///         unimplemented!()
///     }
///     fn sweep<W>(
///         self,
///         _sweep: &Sweep,
///         _wa: &Windows<'_, f64>,
///         _wb: &Windows<'_, f64>,
///         _same_body: bool,
///         _verify: impl FnMut(Candidate<'_, '_, f64>) -> Result<W, String>,
///     ) -> BoundReport<W> {
///         unimplemented!()
///     }
///     fn on_face_stations(self, _c: Candidate<'_, '_, f64>) -> (Stations, Stations) {
///         unimplemented!()
///     }
/// }
/// ```
pub trait ClearanceLaneOf<T: Decide>: crate::lane_seal::Sealed + Copy + Send + Sync {
    /// A side's windows in its own frame.
    ///
    /// # Errors
    /// The lane's own.
    fn windows<'b>(self, side: &Side<'b, T>, which: Which)
    -> Result<Windows<'b, T>, EngineRefusal>;

    /// A side's windows moved by `at`.
    ///
    /// # Errors
    /// The lane's own.
    fn windows_at<'b>(
        self,
        side: &Side<'b, T>,
        which: Which,
        at: Option<&RigidMotion<T>>,
    ) -> Result<Windows<'b, T>, EngineRefusal>;

    /// The bound sweep of two sides' windows, `verify` asked of each
    /// candidate; the first `Ok` per question is kept.
    fn sweep<W>(
        self,
        sweep: &Sweep,
        wa: &Windows<'_, T>,
        wb: &Windows<'_, T>,
        same_body: bool,
        verify: impl FnMut(Candidate<'_, '_, T>) -> Result<W, String>,
    ) -> BoundReport<W>;

    /// A candidate's stations on its two faces.
    fn on_face_stations(self, candidate: Candidate<'_, '_, T>) -> (Stations, Stations);
}

impl<T: geom_core::Certifies> ClearanceLaneOf<T> for ClearanceLane<T> {
    fn windows<'b>(
        self,
        side: &Side<'b, T>,
        which: Which,
    ) -> Result<Windows<'b, T>, EngineRefusal> {
        (self.windows)(side, which, None)
    }

    fn windows_at<'b>(
        self,
        side: &Side<'b, T>,
        which: Which,
        at: Option<&RigidMotion<T>>,
    ) -> Result<Windows<'b, T>, EngineRefusal> {
        (self.windows)(side, which, at)
    }

    fn sweep<W>(
        self,
        sweep: &Sweep,
        wa: &Windows<'_, T>,
        wb: &Windows<'_, T>,
        same_body: bool,
        mut verify: impl FnMut(Candidate<'_, '_, T>) -> Result<W, String>,
    ) -> BoundReport<W> {
        // The sweep keeps the first `Ok` per question, so the two
        // questions' witnesses are kept apart here.
        let (mut violation, mut within) = (None, None);
        let report = (self.bound_sweep)(sweep, wa, wb, same_body, &mut |candidate| {
            let ask = candidate.ask();
            verify(candidate).map(|w| match ask {
                Ask::Below { .. } => violation = Some(w),
                Ask::Within { .. } => {
                    within.get_or_insert(w);
                }
            })
        });
        let verdict = match report.verdict {
            BoundVerdict::Holds => BoundVerdict::Holds,
            BoundVerdict::Exceeded => BoundVerdict::Exceeded,
            BoundVerdict::Refused(r) => BoundVerdict::Refused(r),
            BoundVerdict::Violated(()) => BoundVerdict::Violated(
                violation.unwrap_or_else(|| unreachable!("the sweep stops at the verifier's `Ok`")),
            ),
            BoundVerdict::Within(()) => {
                BoundVerdict::Within(within.unwrap_or_else(|| {
                    unreachable!("an upper bound's proof is the verifier's `Ok`")
                }))
            }
        };
        BoundReport {
            verdict,
            receipt: report.receipt,
            widths: report.widths,
            across: report.across,
            cornered: report.cornered,
            decided: report.decided,
        }
    }

    fn on_face_stations(self, candidate: Candidate<'_, '_, T>) -> (Stations, Stations) {
        (self.face_stations)(candidate)
    }
}

impl<T: Decide> ClearanceLaneOf<T> for geom_core::NoLane {
    fn windows<'b>(
        self,
        _side: &Side<'b, T>,
        _which: Which,
    ) -> Result<Windows<'b, T>, EngineRefusal> {
        match self {}
    }

    fn windows_at<'b>(
        self,
        _side: &Side<'b, T>,
        _which: Which,
        _at: Option<&RigidMotion<T>>,
    ) -> Result<Windows<'b, T>, EngineRefusal> {
        match self {}
    }

    fn sweep<W>(
        self,
        _sweep: &Sweep,
        _wa: &Windows<'_, T>,
        _wb: &Windows<'_, T>,
        _same_body: bool,
        _verify: impl FnMut(Candidate<'_, '_, T>) -> Result<W, String>,
    ) -> BoundReport<W> {
        match self {}
    }

    fn on_face_stations(self, _candidate: Candidate<'_, '_, T>) -> (Stations, Stations) {
        match self {}
    }
}

/// **What the verdict door answers** ([`embedded`]): the yes and the no
/// of E7's self-intersection question, and the typed refusal that is
/// neither.
#[derive(Debug, Clone, PartialEq)]
pub enum Embedding {
    /// Every pair of the scope's faces, each face against itself
    /// included, is certified apart off the cells of M it shares.
    Embedded,
    /// Two points of the boundary that are not one point of it, which
    /// the verifier decided coincide.
    Crosses(Crossing),
    /// The engine could not certify the scope embedded: never read as
    /// [`Self::Embedded`].
    Refused(EngineRefusal),
}

/// **A verified self-intersection**: the verdict door's witness.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Crossing {
    /// The face holding the first point, then the face holding the
    /// second: the same face when one face folds through itself.
    pub faces: (FaceKey, FaceKey),
    /// The two points, each as a `(u, v)` of its face's stored chart.
    pub stations: [(f64, f64); 2],
    /// How far apart the two points are at most, in metres: the upper
    /// end of the separation decided not strictly positive at
    /// [`SELF_INTERSECTION_GAP`].
    pub distance: f64,
}

/// [`embedded`]'s answer and the cells it rests on.
#[derive(Debug, Clone, PartialEq)]
pub struct EmbeddingReport {
    /// The verdict.
    pub verdict: Embedding,
    /// The sweep's counting receipt.
    pub receipt: CellReceipt,
}

/// **The verdict door**: whether `side`'s faces, one body's, are
/// embedded off what they share — E7's self-intersection question, every
/// face against every face and itself, at the strict bound.
///
/// A function bounded on certification rights, not a per-scalar
/// `Option`: it answers at every scalar that may certify (`f64`,
/// `Probe`, `Interval`, and `Sym` over each), each through the lane's
/// one crossing ([`ClearanceLane`]), so `Sym<f64>` answers as `f64`
/// does, through the crossing `Sym<Interval>` takes. A dual cannot name
/// it:
///
/// ```compile_fail,E0277
/// use geom_core::Dual64;
/// let _ = topo::clearance::embedded::<Dual64>;
/// ```
///
/// The minimum-clearance measure is the other door
/// ([`crate::AtRestPolicy::clearance_lane`]): a point build finds a near
/// pair there, not a verdict, so it answers `None` at a point scalar.
/// A yes or no needs no box: a crossing at one point of the family is a
/// crossing.
///
/// **The witness is the verifier's own**: of the stations the engine
/// places on the two faces, the pair whose separation, read on the
/// candidate's carriers ([`Candidate::enclosure`]), is least at its
/// upper end, decided at the strict site, where a separation in the band
/// is a coincidence.
pub fn embedded<T: geom_core::Certifies + CertifiedBounds>(
    side: &Side<'_, T>,
    band: Band,
    budgets: Budgets,
) -> EmbeddingReport {
    let lane = ClearanceLane::<T>::certified();
    let windows = match lane.windows(side, Which::First) {
        Ok(windows) => windows,
        Err(refusal) => {
            return EmbeddingReport {
                verdict: Embedding::Refused(refusal),
                receipt: CellReceipt::default(),
            };
        }
    };
    let sweep = Sweep {
        bound: ClearanceBound::StrictlyPositive,
        band,
        budgets,
    };
    let report = lane.sweep(&sweep, &windows, &windows, true, |candidate| {
        let (faces, ask, over) = (candidate.faces(), candidate.ask(), candidate.enclosure());
        let (la, lb) = lane.on_face_stations(candidate);
        let mut best: Option<([(f64, f64); 2], Interval)> = None;
        for &a in &la {
            for &b in &lb {
                let d = over.separation(a, b);
                if best.is_none_or(|(_, w)| d.hi() < w.hi()) {
                    best = Some(([a, b], d));
                }
            }
        }
        let (stations, distance) = best.ok_or("no station of either cell is on its face")?;
        ask.confirms(distance, band).map(|()| Crossing {
            faces,
            stations,
            distance: distance.hi(),
        })
    });
    let verdict = match report.verdict {
        BoundVerdict::Holds => Embedding::Embedded,
        BoundVerdict::Violated(crossing) => Embedding::Crosses(crossing),
        BoundVerdict::Refused(refusal) => Embedding::Refused(refusal),
        BoundVerdict::Within(_) | BoundVerdict::Exceeded => {
            unreachable!("the strict question has no upper bound")
        }
    };
    EmbeddingReport {
        verdict,
        receipt: report.receipt,
    }
}

/// **The door's WIRING**: which function each field of
/// [`ClearanceLane::certified`] holds. Why a wiring row compares
/// pointers rather than outputs: `certified_enclosure_impl_census`'s
/// module doc.
#[cfg(test)]
mod wiring_rows {
    use super::ClearanceLane;

    /// `Ok(())` when every field holds its door; otherwise the name of
    /// the first field that does not.
    fn holds_the_certified_clearance_lane<T: geom_core::Certifies + geom_core::CertifiedBounds>()
    -> Result<(), &'static str> {
        let lane = ClearanceLane::<T>::certified();
        if !std::ptr::fn_addr_eq(
            lane.windows,
            ClearanceLane::<T>::windows_of as fn(_, _, _) -> _,
        ) {
            return Err("windows is not `Side::windows_at`");
        }
        if !std::ptr::fn_addr_eq(
            lane.bound_sweep,
            ClearanceLane::<T>::sweep_of as fn(_, _, _, _, _) -> _,
        ) {
            return Err("bound_sweep is not `Sweep::run`");
        }
        if !std::ptr::fn_addr_eq(
            lane.face_stations,
            ClearanceLane::<T>::stations_of as fn(_) -> _,
        ) {
            return Err("face_stations is not `Candidate::on_face_stations`");
        }
        Ok(())
    }

    #[test]
    fn f64_is_wired_to_the_certified_clearance_lane() {
        assert_eq!(
            holds_the_certified_clearance_lane::<f64>(),
            Ok(()),
            "`ClearanceLane::<f64>::certified()` holds something other than the engine"
        );
    }

    /// The symbolic tier holds the base scalar's doors: the same
    /// pointers, instantiated at `Sym<f64>`.
    #[test]
    fn sym_over_f64_is_wired_to_the_certified_clearance_lane() {
        assert_eq!(
            holds_the_certified_clearance_lane::<geom_core::Sym<f64>>(),
            Ok(()),
            "`ClearanceLane::<Sym<f64>>::certified()` holds something other than the engine"
        );
    }

    #[cfg(feature = "probe")]
    #[test]
    fn probe_is_wired_to_the_certified_clearance_lane() {
        assert_eq!(
            holds_the_certified_clearance_lane::<geom_core::Probe>(),
            Ok(()),
            "`ClearanceLane::<Probe>::certified()` holds something other than the engine"
        );
    }

    /// Which scalars hold the measure door, one row for all of them: an
    /// answer that flips at any scalar reds here by name.
    #[test]
    fn the_measure_door_answers_at_the_enclosure_scalars_and_nowhere_else() {
        use crate::AtRestPolicy;
        use geom_core::interval::Interval;
        let answers = |name, held: bool| (name, held);
        #[cfg_attr(not(feature = "probe"), allow(unused_mut))]
        let mut table = vec![
            answers("f64", f64::clearance_lane().is_some()),
            answers("Dual64", geom_core::Dual64::clearance_lane().is_some()),
            answers("Interval", Interval::clearance_lane().is_some()),
            answers(
                "Sym<f64>",
                geom_core::Sym::<f64>::clearance_lane().is_some(),
            ),
            answers(
                "Sym<Interval>",
                geom_core::Sym::<Interval>::clearance_lane().is_some(),
            ),
        ];
        #[cfg_attr(not(feature = "probe"), allow(unused_mut))]
        let mut expected = vec![
            ("f64", false),
            ("Dual64", false),
            ("Interval", true),
            ("Sym<f64>", false),
            ("Sym<Interval>", true),
        ];
        #[cfg(feature = "probe")]
        {
            table.push(answers(
                "Probe",
                geom_core::Probe::clearance_lane().is_some(),
            ));
            expected.push(("Probe", false));
        }
        assert_eq!(table, expected, "(scalar, holds the clearance door)");
    }

    /// The verdict door asked of a unit cube at `T`: whether it answers
    /// embedded, and whether that answer rests on cell pairs it read.
    #[allow(clippy::expect_used)]
    fn the_verdict_door_on_a_cube<T>() -> (bool, bool)
    where
        T: crate::props::CertifiedPolicy + geom_core::CertifiedBounds,
    {
        let tol = geom_core::Tol::witness();
        let cube: crate::Body<T> =
            crate::test_support_fixtures::brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let side = super::Side {
            body: &cube,
            faces: cube.faces().map(|(k, _)| k).collect(),
        };
        let report = super::embedded(
            &side,
            geom_core::Band::linear(tol).expect("the witness tolerance has a band"),
            super::Budgets {
                max_cell_depth: super::DEFAULT_MAX_CELL_DEPTH,
                max_cell_pairs: 65_536,
            },
        );
        (
            report.verdict == super::Embedding::Embedded,
            report.receipt.candidates > 0,
        )
    }

    /// Which scalars the verdict door answers at: every one with
    /// certification rights, each having read the cube's cell pairs. A
    /// dual cannot name it ([`super::embedded`]'s `compile_fail` row).
    #[test]
    fn the_verdict_door_answers_at_every_certifying_scalar() {
        use geom_core::interval::Interval;
        #[cfg_attr(not(feature = "probe"), allow(unused_mut))]
        let mut table = vec![
            ("f64", the_verdict_door_on_a_cube::<f64>()),
            ("Interval", the_verdict_door_on_a_cube::<Interval>()),
            (
                "Sym<f64>",
                the_verdict_door_on_a_cube::<geom_core::Sym<f64>>(),
            ),
            (
                "Sym<Interval>",
                the_verdict_door_on_a_cube::<geom_core::Sym<Interval>>(),
            ),
        ];
        #[cfg(feature = "probe")]
        table.push(("Probe", the_verdict_door_on_a_cube::<geom_core::Probe>()));
        let expected: Vec<_> = table
            .iter()
            .map(|&(name, _)| (name, (true, true)))
            .collect();
        assert_eq!(
            table, expected,
            "(scalar, (answers embedded, read the cube's cell pairs))"
        );
    }

    #[test]
    fn interval_is_wired_to_the_certified_clearance_lane() {
        assert_eq!(
            holds_the_certified_clearance_lane::<geom_core::interval::Interval>(),
            Ok(()),
            "`ClearanceLane::<Interval>::certified()` holds something other than the engine"
        );
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod extent_rows {
    use super::{Row, extent_of, row_of};
    use core::f64::consts::{PI, TAU};
    use geom_brep::{Channel, Pcurve};
    use geom_core::{Bounds, Interval, Real};

    /// The harmonic image `u = t`, `v = sin t`, which bulges to `v = 1`
    /// between its ends over `[0, π]`.
    fn arch() -> Pcurve<f64> {
        Pcurve::Harmonic {
            u: Channel::Angle {
                phase: 0.0,
                winding: 1,
            },
            v: Channel::Free {
                c: 0.0,
                a: 0.0,
                b: 1.0,
                l: 0.0,
            },
        }
    }

    /// A cylinder's chart: its azimuth periodic, its height not.
    const CYLINDER: [bool; 2] = [true, false];

    /// The arch's chart points at `0` and `t`.
    fn flat(t: f64) -> [[geom_core::interval::Interval; 2]; 2] {
        let at = |x: f64, y: f64| {
            [
                geom_core::interval::Interval::from_bounds(x, x),
                geom_core::interval::Interval::from_bounds(y, y),
            ]
        };
        [at(0.0, 0.0), at(t, t.sin())]
    }

    /// **A harmonic row's extent holds its bulge, not its ends alone.**
    /// `v = sin t` is `0` at both ends of `[0, π]` and `1` between, so a
    /// hull of its ends misses the face's points over the bulge. Red if
    /// a row that is no monotone line is read by its ends.
    #[test]
    fn a_harmonic_rows_extent_holds_its_bulge() {
        let Some(Row::Bulging([u, v])) = super::certified_lift(|lift| {
            row_of(
                &arch(),
                (Interval::from_f64(0.0), Interval::from_f64(PI)),
                flat(PI),
                lift,
            )
        })
        .flatten() else {
            panic!("the arch is read by its enclosure");
        };
        assert!(
            u.lo() <= 0.0 && PI <= u.hi() && v.hi() >= 1.0,
            "the enclosure holds the arch: {u:?} {v:?}"
        );
        let [(_, _), (_, top)] = extent_of(
            CYLINDER,
            [super::certified_lift(|lift| {
                row_of(
                    &arch(),
                    (Interval::from_f64(0.0), Interval::from_f64(PI)),
                    flat(PI),
                    lift,
                )
            })
            .flatten()]
            .into_iter(),
        )
        .expect("an extent");
        assert!(top >= 1.0, "the extent reaches the bulge: {top}");
    }

    /// **A superset extent a whole turn wide is not taken on a periodic
    /// axis**: it would read as a loop that wraps it, which bounds
    /// nothing. On an axis that is not periodic it is taken. Red if the
    /// whole-turn check on a bulging hull is dropped, or read on every
    /// axis.
    #[test]
    fn a_bulging_extent_a_whole_turn_wide_is_unknown() {
        assert!(
            extent_of(
                CYLINDER,
                [super::certified_lift(|lift| row_of(
                    &arch(),
                    (Interval::from_f64(0.0), Interval::from_f64(TAU)),
                    flat(TAU),
                    lift
                ))
                .flatten()]
                .into_iter()
            )
            .is_none(),
            "a turn-wide superset reads unknown"
        );
        let wide = [super::certified_lift(|lift| {
            row_of(
                &arch(),
                (Interval::from_f64(0.0), Interval::from_f64(TAU)),
                flat(TAU),
                lift,
            )
        })
        .flatten()];
        assert!(
            extent_of([false, false], wide.into_iter()).is_some(),
            "a superset a turn wide on an axis that is not periodic is taken"
        );
    }

    /// A spiric's wall image: `R = 2`, `r = 0.5`, `d = 0.3`, on the
    /// carrier's own torus or its mirror, wound with `sense`.
    fn wall(sense: f64, torus: geom_brep::WallTorus) -> Pcurve<f64> {
        Pcurve::Spiric {
            major: 2.0,
            minor: 0.5,
            offset: 0.3,
            image: geom_brep::SpiricImage::Wall {
                u0: 0.4,
                v0: TAU,
                sense,
                torus,
            },
        }
    }

    /// **A wall row's lift is the pcurve's own image**
    /// ([`super::wall_of`]): its chart point encloses the pcurve's at
    /// each parameter, wound either way on either torus, and a sense
    /// that is no unit sign is not lifted. Red if the azimuth's sign
    /// forgets the torus's.
    #[test]
    fn a_wall_rows_lift_is_its_pcurves_image() {
        use geom_brep::WallTorus::{Mirror, Own};
        for (sense, torus) in [(1.0, Own), (-1.0, Own), (1.0, Mirror), (-1.0, Mirror)] {
            let p = wall(sense, torus);
            let w = super::certified_lift(|lift| super::wall_of(&p, lift))
                .flatten()
                .expect("a wall image");
            for k in 0..8 {
                let t = -2.0 + 0.6 * f64::from(k);
                let q = p.eval(t);
                let [u, v] = w.at(geom_core::interval::Interval::from_bounds(t, t));
                assert!(
                    u.lo() <= q.x && q.x <= u.hi() && v.lo() <= q.y && q.y <= v.hi(),
                    "{sense} {torus:?} at {t}: {q:?} outside {u:?} × {v:?}"
                );
            }
        }
        assert!(
            super::certified_lift(|lift| super::wall_of(&wall(0.5, Own), lift))
                .flatten()
                .is_none(),
            "no unit sign"
        );
    }

    /// **A wall row's extent holds its bulge**: over `t ∈ [−1, 1]` its
    /// azimuth `atan2(f, d)` is greatest at `t = 0`, between its ends.
    /// Red if a wall row is read by its ends.
    #[test]
    fn a_wall_rows_extent_holds_its_bulge() {
        let p = wall(1.0, geom_brep::WallTorus::Own);
        let end = |t: f64| {
            let q = p.eval(t);
            [q.x, q.y].map(|x| geom_core::interval::Interval::from_bounds(x, x))
        };
        let Some(Row::Bulging([u, _])) = super::certified_lift(|lift| {
            row_of(
                &p,
                (Interval::from_f64(-1.0), Interval::from_f64(1.0)),
                [end(-1.0), end(1.0)],
                lift,
            )
        })
        .flatten() else {
            panic!("the wall is read by its enclosure");
        };
        let top = p.eval(0.0).x;
        assert!(
            top > p.eval(1.0).x && u.hi() >= top,
            "the enclosure holds the bulge {top}: {u:?}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod winding_rate {
    use super::harmonic_of;
    use crate::injective;
    use geom_brep::{Channel, Pcurve};
    use geom_core::Real;
    use geom_core::interval::Interval;

    /// **An angle channel's rate is its winding count, exactly**, at an
    /// interval scalar. Red if it is read off the quotient of two
    /// enclosures of a turn, which is wider than the integer.
    #[test]
    fn a_windings_rate_is_its_count() {
        for winding in [-2, -1, 1, 3] {
            let zero = Interval::from_f64(0.0);
            let image = Pcurve::Harmonic {
                u: Channel::Angle {
                    phase: Interval::from_f64(core::f64::consts::PI),
                    winding,
                },
                v: Channel::Free {
                    c: zero,
                    a: zero,
                    b: zero,
                    l: zero,
                },
            };
            let h = super::certified_lift(|lift| harmonic_of(&image, lift))
                .flatten()
                .expect("a harmonic image");
            assert!(
                injective::exactly(h.pl[0], f64::from(winding)),
                "winding {winding}: rate {:?}",
                h.pl[0]
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod receipt_rule {
    use super::CellReceipt;

    /// **`outside <= discharged` fails loud.** A sweep that counted a
    /// vacuous discharge without counting the discharge would break the
    /// receipt's meaning while `holds()` — which cannot read `outside`
    /// — went on answering `true`.
    #[test]
    #[should_panic(expected = "SUB-COUNT")]
    fn a_receipt_whose_outside_exceeds_discharged_fails_loud() {
        let bad = CellReceipt {
            candidates: 1,
            discharged: 1,
            outside: 2,
            ..CellReceipt::default()
        };
        assert!(bad.holds(), "the identity itself cannot see the defect");
        bad.check_outside_is_a_sub_count();
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod cone_window {
    use geom_core::Tol;

    use super::{Which, window_of};
    use crate::Body;
    use crate::entity::FaceKey;

    const ALPHA: f64 = 0.5;

    /// The signed slant of the cone's circle at height `z`.
    fn slant(z: f64) -> f64 {
        z / ALPHA.cos()
    }

    /// A cone cap at height `z` on the cone of half-angle [`ALPHA`]
    /// (`test_support_fixtures::cone_cap`).
    fn cone_cap(z: f64) -> (Body<f64>, FaceKey) {
        crate::test_support_fixtures::cone_cap(ALPHA, z, Tol::witness())
    }

    /// **A cone face's window ends where the face does, unless the face
    /// holds the apex.** A wall over `z ∈ [0.5, 1]` misses its apex: its
    /// `v` window ends at its rims' slants, short of the apex's `0`, and
    /// holds every point of the face. A cap bounded by its circle at
    /// `z = 1` holds the apex inside, though none of its boundary reaches
    /// it: its window runs from the apex to its rim, and so it does with
    /// any one of its joints missing. Red if the decision reads every
    /// face as reaching the apex, or none, or reads past a missing joint,
    /// or if the wall's window ends anywhere but at its rims.
    #[test]
    fn a_cone_window_ends_at_the_face_unless_the_face_holds_the_apex() {
        let (mut body, face) = crate::boxes::tests::cone_wall(ALPHA, 0.0, 1.0, 0.5, 1.0);
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        let w = window_of(&body, face, Which::First, None).unwrap();
        assert!(
            (w.v.0 - slant(0.5)).abs() < 1e-12 && (w.v.1 - slant(1.0)).abs() < 1e-12,
            "the wall's window ends at its rims, short of the apex: {:?}",
            w.v
        );
        for z in [0.5, 0.75, 1.0] {
            assert!(
                w.v.0 <= slant(z) && slant(z) <= w.v.1,
                "the wall's window holds its points at z = {z}: {:?}",
                w.v
            );
        }
        let (body, face) = cone_cap(1.0);
        let lp = body.get_face(face).unwrap().outer;
        let joints: Option<Vec<_>> = body
            .loop_lift(lp)
            .unwrap()
            .iter()
            .map(|row| body.joint(row.half_edge))
            .collect();
        let winding = crate::joint::Winding::of(joints.expect("the cap's joints are stored"));
        assert!(
            winding.deck.u.abs() == 1,
            "the cap's circle winds once about the axis: {winding:?}"
        );
        let w = window_of(&body, face, Which::First, None).unwrap();
        assert!(
            w.v.0 <= 0.0 && slant(1.0) <= w.v.1,
            "the cap's window runs from the apex it holds to its rim: {:?}",
            w.v
        );
        let cycle = body
            .loop_lift(lp)
            .unwrap()
            .iter()
            .map(|row| row.half_edge)
            .collect::<Vec<_>>();
        for he in cycle {
            let (mut body, face) = cone_cap(1.0);
            body.detach_joint(he);
            let w = window_of(&body, face, Which::First, None).unwrap();
            assert!(
                w.v.0 <= 0.0,
                "the cap missing its joint into {he:?} still reaches the apex: {:?}",
                w.v
            );
        }
    }

    /// **A cone face's window ends where its boundary does in a turned
    /// frame too**: the cap above, turned about a generic axis and
    /// moved. Its `v` window runs from the apex's `0` to its rim's slant.
    /// Red if the window is read off a box of the boundary in world
    /// axes, which reaches past the rim along an axis off the world's.
    #[test]
    fn a_cone_window_ends_at_its_boundary_in_a_turned_frame() {
        use geom_core::{Affine3, Point3, Vec3};
        let turn = Affine3::rotation_about_axis(
            Point3::new(0.3, -1.1, 2.9),
            Vec3::new(1.0, 2.0, 3.0),
            0.7,
        );
        let (cap, face) = cone_cap(1.0);
        let cap = crate::transform::transform_rigid(
            &cap,
            &geom_core::test_support::rigid(&turn, Tol::witness()),
        )
        .unwrap();
        let w = window_of(&cap, face, Which::First, None).unwrap();
        assert!(
            w.v.0 == 0.0 && (w.v.1 - slant(1.0)).abs() < 1e-12,
            "the turned cap's window runs from the apex to its rim: {:?}",
            w.v
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod rational_window {
    use std::sync::Arc;

    use geom::{Curve3, NurbsCurve3, Surface};
    use geom_core::spline::{KnotVector, SplineError};
    use geom_core::{Point3, Vec3};

    use super::{Which, window_of};
    use crate::Body;
    use crate::entity::FaceKey;

    /// The cap's plane `x = D` and the cone's half-angle: the f64
    /// conical wedge's cavity cap.
    const D: f64 = 0.0078125;
    const ALPHA: f64 = 0.245;

    /// **A plane cap cut from a cone off its axis**: the cone of
    /// half-angle [`ALPHA`] about `z` met by the plane `x = D` along the
    /// hyperbola `(D, D·sinh s, (D/tan α)·cosh s)`, `s ∈ [−s0, s0]`,
    /// stored as the rational quadratic whose middle weight is
    /// `cosh s0`, then elevated `raise` times and refined at `insert`.
    /// Two chords through a point above the arc close the cap. Returns
    /// the cap, the stored arc and its control net.
    fn hyperbola_cap(
        s0: f64,
        raise: usize,
        insert: &[f64],
    ) -> (Body<f64>, FaceKey, NurbsCurve3<f64>) {
        let top = D / ALPHA.tan();
        let at = |s: f64| Point3::new(D, D * s.sinh(), top * s.cosh());
        let arc = NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![at(-s0), Point3::new(D, 0.0, top / s0.cosh()), at(s0)],
            vec![1.0, s0.cosh(), 1.0],
        )
        .unwrap()
        .elevate_degree(raise)
        .unwrap()
        .refine_knots(insert)
        .unwrap();
        let plane = Surface::Plane {
            frame: geom::Frame::new(Point3::new(D, 0.0, 0.0), Vec3::unit_x(), Vec3::unit_y()),
        };
        let cone = Surface::Cone {
            frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
            half_angle: ALPHA,
        };
        let carrier = Curve3::Nurbs(Arc::new(arc.clone()));
        let (body, face) = crate::boxes::tests::conic_sector(
            plane,
            cone,
            carrier.clone(),
            (0.0, 1.0),
            carrier.eval(0.5),
            (at(-s0), at(s0), Point3::new(D, 0.0, top * s0.cosh() + D)),
        );
        (body, face, arc)
    }

    /// **A face bounded by a rational edge has a finite window that
    /// holds the edge and stays inside its net**: the cap's chart is
    /// `(y, z)`, so its window's `u` holds every sampled `y` of the
    /// arc and its `v` every sampled `z`, and neither reaches past the
    /// hull of the arc's control points and the cap's apex. Rows: the
    /// wedge's own weights `[1, 1.0722, 1]`; a wider arc (middle
    /// weight `cosh 1.5 ≈ 2.35`) elevated to degree 4 and refined to
    /// three spans, whose weights spread unevenly. Red if the edge is
    /// enclosed by a reading that is unbounded over its whole span (the
    /// window is refused), wider than the net, or narrower than the
    /// arc.
    #[test]
    fn a_cap_bounded_by_a_rational_arc_has_a_window_inside_the_arcs_net() {
        for (what, s0, raise, insert) in [
            ("the wedge's quadratic", 1.0722f64.acosh(), 0, &[][..]),
            ("a wide quartic over three spans", 1.5, 2, &[0.3, 0.7][..]),
        ] {
            let (body, face, arc) = hyperbola_cap(s0, raise, insert);
            let w = window_of(&body, face, Which::First, None)
                .unwrap_or_else(|e| panic!("{what}: the cap has a window, got {e:?}"));
            let top = D / ALPHA.tan() * s0.cosh() + D;
            let net = arc.control().iter().map(|p| (p.y, p.z));
            let (lo, hi) = net.fold(((0.0f64, top), (0.0f64, top)), |(lo, hi), (y, z)| {
                ((lo.0.min(y), lo.1.min(z)), (hi.0.max(y), hi.1.max(z)))
            });
            let slack = 1e-15;
            assert!(
                lo.0 - slack <= w.u.0 && w.u.1 <= hi.0 + slack,
                "{what}: u = {:?} within the net's y [{}, {}]",
                w.u,
                lo.0,
                hi.0
            );
            assert!(
                lo.1 - slack <= w.v.0 && w.v.1 <= hi.1 + slack,
                "{what}: v = {:?} within the net's z [{}, {}]",
                w.v,
                lo.1,
                hi.1
            );
            for k in 0..=64 {
                let p = arc.eval(f64::from(k) / 64.0);
                assert!(
                    w.u.0 <= p.y && p.y <= w.u.1 && w.v.0 <= p.z && p.z <= w.v.1,
                    "{what}: the arc's point {p:?} at t = {k}/64 is in the window {:?} × {:?}",
                    w.u,
                    w.v
                );
            }
        }
    }

    /// **A spline edge on part of its carrier bounds its face by that part
    /// alone**: the cap's edge runs over `t ∈ [0, ½]` of the hyperbola
    /// arc, from its end at `y < 0` to its vertex at `y = 0`, and the
    /// chords close it on `y ≤ 0`. The window's `u` (the chart's `y`)
    /// stops at the vertex, where the arc's whole net reaches
    /// `D·sinh s0`. Red if an edge is read over its whole carrier rather
    /// than its span.
    #[test]
    fn a_spline_edge_on_part_of_its_carrier_bounds_its_face_by_that_part() {
        let s0 = 1.0722f64.acosh();
        let (_, _, arc) = hyperbola_cap(s0, 0, &[]);
        let top = D / ALPHA.tan();
        let carrier = Curve3::Nurbs(Arc::new(arc.clone()));
        let at = |s: f64| Point3::new(D, D * s.sinh(), top * s.cosh());
        let (body, face) = crate::boxes::tests::conic_sector(
            Surface::Plane {
                frame: geom::Frame::new(Point3::new(D, 0.0, 0.0), Vec3::unit_x(), Vec3::unit_y()),
            },
            Surface::Cone {
                frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
                half_angle: ALPHA,
            },
            carrier.clone(),
            (0.0, 0.5),
            carrier.eval(0.25),
            (
                at(-s0),
                carrier.eval(0.5),
                Point3::new(D, 0.0, top * s0.cosh() + D),
            ),
        );
        let w = window_of(&body, face, Which::First, None).unwrap();
        let reach = D * s0.sinh();
        assert!(
            w.u.1 < 0.5 * reach,
            "the window stops near the vertex at y = 0, short of the whole net's {reach}: {:?}",
            w.u
        );
        for k in 0..=32 {
            let p = arc.eval(0.5 * f64::from(k) / 32.0);
            assert!(
                w.u.0 <= p.y && p.y <= w.u.1 && w.v.0 <= p.z && p.z <= w.v.1,
                "the edge's point {p:?} is in the window {:?} × {:?}",
                w.u,
                w.v
            );
        }
    }

    /// **A face bounded by closed forms has the window its bounds give,
    /// bit for bit**: the half disc of radius ½ on `z = 0`, its rim the
    /// half circle over `[0, π]` and its diameter two radii, reads
    /// `u = (−½, ½)` and `v` up to `½`, the circle's own bounds, not a
    /// step past them. Red if the edges' boxes are padded, even by zero.
    #[test]
    fn a_half_disc_reads_its_circles_own_bounds_as_its_window() {
        let (body, face) = crate::boxes::tests::arc_sector(0.5, core::f64::consts::PI);
        let w = window_of(&body, face, Which::First, None).unwrap();
        assert_eq!(
            [w.u.0, w.u.1, w.v.1].map(f64::to_bits),
            [-0.5f64, 0.5, 0.5].map(f64::to_bits),
            "the half disc's window: {:?} × {:?}",
            w.u,
            w.v
        );
        assert!(
            w.v.0 <= 0.0 && w.v.0 > -1e-15,
            "the diameter's side of the window is at y = 0: {:?}",
            w.v
        );
    }

    /// **A rational edge has no zero or negative weight to enclose**:
    /// construction refuses one typed, so the net hull a cap's window
    /// reads is a hull of points the arc is a convex combination of.
    #[test]
    fn a_non_positive_weight_is_refused_at_construction() {
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control = vec![
            Point3::new(D, -1.0, 1.0),
            Point3::new(D, 0.0, 0.5),
            Point3::new(D, 1.0, 1.0),
        ];
        for w in [0.0, -1.0, f64::NAN] {
            let got = NurbsCurve3::new(knots.clone(), control.clone(), vec![1.0, w, 1.0]);
            assert!(
                matches!(got, Err(SplineError::NonPositiveWeight { index: 1, .. })),
                "the weight {w} is refused typed at index 1, got {got:?}"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod refines_rows {
    use std::sync::Arc;

    use geom::{NurbsSurface, Surface};
    use geom_core::spline::{KnotVector, Knots};
    use geom_core::{Interval, Point3, Real};

    use super::{Aabb, narrows, refines};

    /// A patch over `[0, 1]²` linear in `v`, lifted to `Interval`: the
    /// polyline `along` in `u` (degree 1, knots at the even splits),
    /// raised by `v` in `z`.
    fn ribbon(along: &[(f64, f64, f64)], rise: f64) -> Surface<Interval> {
        let n = along.len();
        let mut knots = vec![0.0];
        knots.extend((0..n).map(|k| k as f64 / (n - 1) as f64));
        knots.push(1.0);
        let ku = KnotVector::clamped(knots, 1).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = along
            .iter()
            .flat_map(|&(x, y, z)| [Point3::new(x, y, z), Point3::new(x, y, z + rise)])
            .map(|p| p.map(Interval::from_f64))
            .collect();
        let knots = |k: KnotVector| Knots::from(k).lift(Interval::from_f64);
        Surface::Nurbs(Arc::new(
            NurbsSurface::new(knots(ku), knots(kv), control, vec![Interval::one(); 2 * n]).unwrap(),
        ))
    }

    const WHOLE: (f64, f64) = (0.0, 1.0);

    /// **A wall whose extremes all lie in one `u` half refines**, by the
    /// other half moving a bound: a `min` bound for the first pair, a
    /// `max` bound only for the second. Red if the door reads only the
    /// lower halves, only the upper ones, asks both halves to move, or
    /// counts only one side's bounds.
    #[test]
    fn a_wall_reaching_its_extremes_in_one_half_refines() {
        let o = (0.0, 0.0, 0.0);
        let (i, ii) = ((1.0, 1.0, 0.0), (2.0, 2.0, 0.0));
        for (what, along) in [
            ("lower half, the upper moving a min", [o, ii, i]),
            ("upper half, the lower moving a min", [i, ii, o]),
            ("lower half, the upper moving a max", [ii, o, i]),
            ("upper half, the lower moving a max", [i, o, ii]),
        ] {
            assert!(refines(&ribbon(&along, 1.0), WHOLE, WHOLE), "{what}");
        }
    }

    /// **The x = 1 wall of a loft whose top faces down does not refine.**
    /// `PRISM_SQUARE`'s edge `(1, -1) → (1, 1)` at `z = 0`, and the same
    /// edge of a copy turned half a turn about `x` at `z = 1`, where it
    /// runs `(1, 1, 1) → (1, -1, 1)`: the degree-1 wall between them,
    /// its net written out (the loft door refuses the body at its fold,
    /// before this door). It is a bow-tie whose `u` iso collapses to
    /// `(1, 0, ½)` at `v = ½`, so each `u` half spans the whole box and
    /// neither narrows it, whichever halves the door reads.
    #[test]
    fn a_bow_tie_wall_does_not_refine() {
        let at = |x, y, z| Point3::new(x, y, z).map(Interval::from_f64);
        let kv = || Knots::from(KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap());
        let wall = Surface::Nurbs(Arc::new(
            NurbsSurface::new(
                kv().lift(Interval::from_f64),
                kv().lift(Interval::from_f64),
                vec![
                    at(1.0, -1.0, 0.0),
                    at(1.0, 1.0, 1.0),
                    at(1.0, 1.0, 0.0),
                    at(1.0, -1.0, 1.0),
                ],
                vec![Interval::one(); 4],
            )
            .unwrap(),
        ));
        assert!(!refines(&wall, WHOLE, WHOLE));
    }

    /// **A box that does not certify narrows nothing.** A half with a
    /// poison bound beside one that moves inward, a poison half, and a
    /// half inside an unbounded whole each give no evidence. Red if a
    /// half's or the whole's finiteness goes unread.
    #[test]
    fn a_box_that_does_not_certify_narrows_nothing() {
        let unit = Aabb {
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: 1.0,
            max_y: 1.0,
            max_z: 1.0,
        };
        let inward = Aabb { max_x: 0.5, ..unit };
        assert!(narrows(&unit, &inward), "the finite control narrows");
        let partly_poison = Aabb {
            min_y: f64::NAN,
            ..inward
        };
        assert!(!narrows(&unit, &partly_poison), "a half with a NaN bound");
        assert!(!narrows(&unit, &Aabb::poison()), "a poison half");
        let unbounded = Aabb {
            max_x: f64::INFINITY,
            ..unit
        };
        assert!(!narrows(&unbounded, &inward), "an unbounded whole");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod trv_rows {
    use super::{
        Bracket, Budgets, Cell, CellPair, CellReceipt, MinSeparationRefusal, Question, RegionMemo,
        Side, Task, Walk, Which, Window, min_separation, poisoned, root_separations, window_of,
    };
    use crate::test_support_fixtures::trv;
    use crate::{Body, FaceKey};
    use geom::Surface;
    use geom_core::{Band, Bounds, Interval, Point3, Real, Tol};

    fn brick(x: (f64, f64)) -> Body<Interval> {
        crate::test_support_fixtures::brick::<Interval>(x, (0.0, 1.0), (0.0, 1.0), Tol::witness())
    }

    fn faces(body: &Body<Interval>) -> Vec<FaceKey> {
        body.faces().map(|(k, _)| k).collect()
    }

    /// The minimum separation of the unit cube and a copy `1` along `x`
    /// that `poison` has changed.
    fn bracket(
        poison: impl FnOnce(&mut Body<Interval>),
    ) -> Result<super::MinSeparation, MinSeparationRefusal> {
        let a = brick((0.0, 1.0));
        let mut b = brick((2.0, 3.0));
        poison(&mut b);
        min_separation(
            &Side {
                body: &a,
                faces: faces(&a),
            },
            &Side {
                body: &b,
                faces: faces(&b),
            },
            false,
            Band::linear(Tol::witness()).unwrap(),
            Budgets::MIN_SEPARATION,
        )
    }

    /// **A face whose lifted carrier or boundary does not certify is
    /// refused at its window** (`window_of`, `boundary_of`): a plane
    /// whose origin, or a vertex whose point, has one coordinate a
    /// refused value with the certified one's endpoints, each coordinate
    /// in turn. The control brackets the bricks' separation of `1`. Red
    /// if either lift reads a refused coordinate's bracket and the
    /// engine brackets the minimum off it.
    #[test]
    fn a_face_that_does_not_certify_is_refused_at_its_window() {
        let control = bracket(|_| {}).expect("the certified control");
        assert!(
            control.lo() <= 1.0 && control.hi().is_some_and(|hi| 1.0 <= hi),
            "the certified control brackets 1: {}",
            control.render()
        );
        let refusal = |poison: &dyn Fn(&mut Body<Interval>)| match bracket(poison) {
            Err(MinSeparationRefusal::Unsupported { carrier, .. }) => carrier,
            other => panic!("refused as unsupported, not {other:?}"),
        };
        let refuse = |p: &mut Point3<Interval>, axis: usize| {
            let c = [&mut p.x, &mut p.y, &mut p.z];
            let [x, y, z] = c;
            let at = [x, y, z].into_iter().nth(axis).expect("an axis");
            *at = trv(at.lo());
        };
        for axis in 0..3 {
            let carrier = refusal(&|b| {
                let Some(crate::SurfaceSlot::Described(Surface::Plane { frame, .. })) =
                    b.surfaces.values_mut().next()
                else {
                    panic!("a brick's faces are planes");
                };
                let mut origin = frame.origin();
                refuse(&mut origin, axis);
                *frame = frame.with_origin(origin);
            });
            assert_eq!(
                carrier, "a face whose carrier does not certify",
                "axis {axis}"
            );
            let carrier = refusal(&|b| {
                refuse(
                    b.points.values_mut().next().expect("a brick has vertices"),
                    axis,
                );
            });
            assert_eq!(
                carrier, "a face whose boundary carries no sound enclosure",
                "axis {axis}"
            );
        }
    }

    /// **A separation that does not certify is poison** to the minimum
    /// (`poisoned`). Red if it reads only finiteness.
    #[test]
    fn a_separation_that_does_not_certify_is_poison() {
        assert!(!poisoned(Interval::from_bounds(1.0, 2.0)), "certified");
        assert!(poisoned(trv(1.0)), "refused, ends finite");
        assert!(
            poisoned(Interval::from_bounds(1.0, f64::INFINITY)),
            "unbounded"
        );
        assert!(poisoned(Interval::from_bounds(f64::NAN, 1.0)), "NaI");
    }

    /// `i` with its bounds intact and its decoration refused.
    fn refused(i: Interval) -> Interval {
        i + trv(0.0)
    }

    /// The unit cube's `+x` face and its copy's `-x` face, `1` apart, as
    /// windows; their root cells, the first one's enclosure refused when
    /// `refuse` says so; and the bracket the walk over them would start.
    struct Pair {
        a: Body<Interval>,
        b: Body<Interval>,
    }

    impl Pair {
        fn new() -> Self {
            Self {
                a: brick((0.0, 1.0)),
                b: brick((2.0, 3.0)),
            }
        }

        fn windows(&self) -> (Window<'_, Interval>, Window<'_, Interval>) {
            let facing = |body: &Body<Interval>, x: f64| {
                faces(body)
                    .into_iter()
                    .find(|&f| {
                        window_of(body, f, Which::First, None).is_ok_and(|w| {
                            let p = w.surface.eval(
                                Interval::from_f64(geom::mid_param(w.u.0, w.u.1)),
                                Interval::from_f64(geom::mid_param(w.v.0, w.v.1)),
                            );
                            (p.x.lo() - x).abs() < 1e-9 && (p.x.hi() - x).abs() < 1e-9
                        })
                    })
                    .expect("the facing face")
            };
            let wa = window_of(&self.a, facing(&self.a, 1.0), Which::First, None).unwrap();
            let wb = window_of(&self.b, facing(&self.b, 2.0), Which::Second, None).unwrap();
            (wa, wb)
        }
    }

    fn cells(wa: &Window<'_, Interval>, wb: &Window<'_, Interval>, refuse: bool) -> (Cell, Cell) {
        let mut ca = Cell::of(wa);
        if refuse {
            ca.enc.x = refused(ca.enc.x);
        }
        (ca, Cell::of(wb))
    }

    fn task(ca: Cell, cb: Cell) -> Task {
        Task {
            k: 0,
            i: 0,
            j: 0,
            pair: CellPair {
                a: ca,
                b: cb,
                depth: 0,
                fresh: false,
            },
            open: super::Open {
                below: true,
                above: false,
            },
        }
    }

    fn start(wa: &Window<'_, Interval>, wb: &Window<'_, Interval>) -> Bracket {
        Bracket {
            hi: None,
            floor: f64::INFINITY,
            poison: None,
            region: RegionMemo::at(Band::linear(Tol::witness()).unwrap()),
            faces: super::WindowFaces(vec![wa.face], vec![wb.face]),
        }
    }

    fn is_poison(r: &Option<MinSeparationRefusal>) -> bool {
        matches!(r, Some(MinSeparationRefusal::PoisonEnclosure { .. }))
    }

    /// **Each of the minimum's four reads of a separation refuses one
    /// that does not certify**, reached with a root cell whose enclosure
    /// is refused with its bounds intact: the root pass and a walked pair
    /// refuse `PoisonEnclosure`, an abandoned pair refuses it as the
    /// walked pair does, and the witness takes no incumbent off it. Each
    /// control is the same cells certified. Red if any one of the four
    /// reads only finiteness.
    #[test]
    fn each_read_of_a_separation_refuses_one_that_does_not_certify() {
        let pair = Pair::new();
        let (wa, wb) = pair.windows();
        let (wa_s, wb_s) = (core::slice::from_ref(&wa), core::slice::from_ref(&wb));
        for refuse in [false, true] {
            let (ca, cb) = cells(&wa, &wb, refuse);
            let roots = root_separations((wa_s, &[ca]), (wb_s, &[cb]), false);
            assert_eq!(
                matches!(roots, Err(MinSeparationRefusal::PoisonEnclosure { .. })),
                refuse,
                "the root pass, refused: {refuse}"
            );
            let mut q = start(&wa, &wb);
            let mut walk = Walk {
                budgets: Budgets::MIN_SEPARATION,
                receipt: CellReceipt::default(),
                left: 0,
                next: Vec::new(),
            };
            let _ =
                <Bracket as Question<Interval>>::pair(&mut q, &mut walk, task(ca, cb), &wa, &wb);
            assert_eq!(
                is_poison(&q.poison),
                refuse,
                "a walked pair, refused: {refuse}"
            );
            let mut q = start(&wa, &wb);
            let mut receipt = CellReceipt::default();
            <Bracket as Question<Interval>>::overflow(&mut q, &mut receipt, vec![task(ca, cb)]);
            assert_eq!(
                is_poison(&q.poison),
                refuse,
                "an abandoned pair, refused: {refuse}"
            );
        }
        for refuse in [false, true] {
            let mut x = wa.clone();
            if refuse {
                let Surface::Plane { frame, .. } = &mut x.surface else {
                    panic!("a brick's face is a plane");
                };
                let mut origin = frame.origin();
                origin.x = refused(origin.x);
                *frame = frame.with_origin(origin);
            }
            let (ca, cb) = (Cell::of(&wa), Cell::of(&wb));
            let mut q = start(&x, &wb);
            q.witness((0, &x, ca), (0, &wb, cb));
            assert_eq!(
                q.hi.is_none(),
                refuse,
                "the witness, refused: {refuse}: {:?}",
                q.hi
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod support_rows {
    use std::sync::Arc;

    use geom::{NurbsSurface, Surface};
    use geom_core::spline::{KnotVector, Knots};
    use geom_core::{Bounds, Interval, Point3, Real, Vec3};

    use super::{Cell, CellPair, Hull, Which, poisoned, separated_by_support, window_of};
    use crate::test_support_fixtures::trv;
    use geom_core::{Band, Tol};

    /// The quarter of the unit cylinder from `x` to `y`, `z ∈ [0, 1]`:
    /// a rational quadratic arc in `u`, its middle weight `middle`,
    /// linear in `v`.
    fn quarter(middle: Interval) -> Surface<Interval> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
            .iter()
            .flat_map(|&(x, y)| [Point3::new(x, y, 0.0), Point3::new(x, y, 1.0)])
            .map(|p| p.map(Interval::from_f64))
            .collect();
        let one = Interval::one();
        let knots = |k: KnotVector| Knots::from(k).lift(Interval::from_f64);
        Surface::Nurbs(Arc::new(
            NurbsSurface::new(
                knots(ku),
                knots(kv),
                control,
                vec![one, one, middle, middle, one, one],
            )
            .unwrap(),
        ))
    }

    fn middle_weight() -> Interval {
        Interval::from_f64(core::f64::consts::FRAC_1_SQRT_2)
    }

    /// **A cell's support along a direction holds the patch's points
    /// over the cell, and is narrower than its box's.** Three cells of
    /// the quarter cylinder, each read along its centre normal and two
    /// other directions, meet the enclosure of `⟨S(u, v), n⟩` at each
    /// point of an 11 × 3 grid of the cell, which both hold; along its centre normal each is narrower than the box's
    /// support. Red if the hull reads a subset of the restricted points
    /// (the bulge of the arc between its ends is lost), or if the
    /// support is the box's.
    #[test]
    fn a_cells_support_holds_its_points_and_is_narrower_than_its_box() {
        let s = quarter(middle_weight());
        for u in [(0.0, 1.0), (0.25, 0.5), (0.6, 0.65)] {
            let cell = Cell::on(&s, u, (0.0, 1.0));
            let hull = Hull::of(&s, cell);
            assert!(matches!(hull, Hull::Net(_)), "a certified net is read");
            let (um, vm) = (0.5 * (u.0 + u.1), 0.5);
            let normal = s.normal(Interval::from_f64(um), Interval::from_f64(vm));
            let mid = |i: Interval| 0.5 * (i.lo() + i.hi());
            let centre_normal = Vec3::new(mid(normal.x), mid(normal.y), mid(normal.z));
            for n in [
                centre_normal,
                Vec3::new(1.0, 0.3, -0.2),
                Vec3::new(-0.4, 1.0, 0.7),
            ] {
                let n = n.map(Interval::from_f64);
                let support = hull.support(n);
                assert!(!poisoned(support), "{u:?} {n:?}: {support:?}");
                for i in 0..=10 {
                    for j in 0..=2 {
                        let pu = u.0 + (u.1 - u.0) * f64::from(i) / 10.0;
                        let pv = f64::from(j) / 2.0;
                        let p = s.eval(Interval::from_f64(pu), Interval::from_f64(pv));
                        let at = p.x * n.x + p.y * n.y + p.z * n.z;
                        assert!(
                            support.lo() <= at.hi() && at.lo() <= support.hi(),
                            "{u:?} along {n:?}: the support {support:?} meets the point's \
                             enclosure at ({pu}, {pv}), {at:?}"
                        );
                    }
                }
            }
            let n = centre_normal.map(Interval::from_f64);
            let (support, boxed) = (hull.support(n), Hull::Box(cell.enc).support(n));
            assert!(
                support.hi() - support.lo() < 0.75 * (boxed.hi() - boxed.lo()),
                "{u:?}: along the centre normal the net's support {support:?} is narrower than \
                 the box's {boxed:?}"
            );
        }
    }

    /// **An analytic cell's support holds its points, and along its own
    /// normal is narrower than its box's.** A cell of a cone, a torus, a
    /// cylinder and a sphere, each read along its centre normal and two
    /// other directions, meets the enclosure of `⟨S(u, v), n⟩` at each
    /// point of an 11 × 11 grid of the cell, which both hold; a cell a
    /// quarter turn wide is held too. Along the centre normal a small
    /// cell's support is under a quarter of its box's. Red if the
    /// mean-value form drops a partial's enclosure over the cell (read
    /// at the centre only, a wide cell's points fall outside it), or if
    /// an analytic cell is read by its box.
    #[test]
    fn an_analytic_cells_support_holds_its_points_and_is_narrower_than_its_box() {
        let o = Point3::new(0.1, -0.2, 0.3);
        let (axis, u_ref) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let surfaces = [
            (
                "cone",
                Surface::Cone {
                    frame: geom::Frame::new(o, axis, u_ref),
                    half_angle: 30f64.to_radians(),
                },
                (1.0, 1.1),
            ),
            (
                "torus",
                Surface::Torus {
                    frame: geom::Frame::new(o, axis, u_ref),
                    major_radius: 1.6,
                    minor_radius: 0.8,
                },
                (2.5, 2.6),
            ),
            (
                "cylinder",
                Surface::Cylinder {
                    frame: geom::Frame::new(o, axis, u_ref),
                    radius: 0.7,
                },
                (0.2, 0.3),
            ),
            (
                "sphere",
                Surface::Sphere {
                    frame: geom::Frame::new(o, axis, u_ref),
                    radius: 0.9,
                },
                (0.4, 0.5),
            ),
        ];
        for (what, s, v) in surfaces {
            let s = s.map_scalar(Interval::from_f64);
            for (u, v, small) in [
                ((0.30, 0.35), v, true),
                ((0.0, core::f64::consts::FRAC_PI_2), (v.0, v.0 + 0.5), false),
            ] {
                let cell = Cell::on(&s, u, v);
                let hull = Hull::of(&s, cell);
                assert!(
                    matches!(hull, Hull::Jet { .. }),
                    "{what}: an analytic cell is read by its jet"
                );
                let (um, vm) = (0.5 * (u.0 + u.1), 0.5 * (v.0 + v.1));
                let normal = s.normal(Interval::from_f64(um), Interval::from_f64(vm));
                let mid = |i: Interval| 0.5 * (i.lo() + i.hi());
                let centre_normal = Vec3::new(mid(normal.x), mid(normal.y), mid(normal.z));
                for n in [
                    centre_normal,
                    Vec3::new(1.0, 0.3, -0.2),
                    Vec3::new(-0.4, 1.0, 0.7),
                ] {
                    let n = n.map(Interval::from_f64);
                    let support = hull.support(n);
                    assert!(!poisoned(support), "{what} {u:?} {n:?}: {support:?}");
                    for i in 0..=10 {
                        for j in 0..=10 {
                            let pu = u.0 + (u.1 - u.0) * f64::from(i) / 10.0;
                            let pv = v.0 + (v.1 - v.0) * f64::from(j) / 10.0;
                            let p = s.eval(Interval::from_f64(pu), Interval::from_f64(pv));
                            let at = p.x * n.x + p.y * n.y + p.z * n.z;
                            assert!(
                                support.lo() <= at.hi() && at.lo() <= support.hi(),
                                "{what} {u:?} × {v:?} along {n:?}: the support {support:?} \
                                 meets the point's enclosure at ({pu}, {pv}), {at:?}"
                            );
                        }
                    }
                }
                if small {
                    let n = centre_normal.map(Interval::from_f64);
                    let (support, boxed) = (hull.support(n), Hull::Box(cell.enc).support(n));
                    assert!(
                        support.hi() - support.lo() < 0.25 * (boxed.hi() - boxed.lo()),
                        "{what} {u:?} × {v:?}: along the centre normal the jet's support \
                         {support:?} is under a quarter of the box's {boxed:?}"
                    );
                }
            }
        }
    }

    /// **A net whose weights do not certify is not read**: the quarter
    /// cylinder with its middle weights carrying a refused decoration
    /// reads the cell's box, whose support does not certify, against
    /// the certified control. Red if the support reads a restricted net
    /// without every weight certified positive.
    #[test]
    fn a_net_whose_weights_do_not_certify_reads_the_box() {
        let n = Vec3::new(1.0, 1.0, 0.0).map(Interval::from_f64);
        for (refused, middle) in [
            (false, middle_weight()),
            (true, trv(core::f64::consts::FRAC_1_SQRT_2)),
        ] {
            let s = quarter(middle);
            let cell = Cell::on(&s, (0.25, 0.5), (0.0, 1.0));
            let hull = Hull::of(&s, cell);
            assert_eq!(
                matches!(hull, Hull::Box(_)),
                refused,
                "the box is read where a weight is refused: {refused}"
            );
            assert_eq!(
                poisoned(hull.support(n)),
                refused,
                "the support certifies where every weight does: {refused}"
            );
        }
    }

    /// **The support compare discharges only off the band.** The unit
    /// cube's `+x` face and its copy's `-x` face are `1` apart, and
    /// their planes' supports read that exactly. Against a bound `1 − m`
    /// with `m` inside the band, the margin is in band and the pair is
    /// not separated; against `0.5` it is.
    #[test]
    fn the_support_compare_discharges_only_off_the_band() {
        let brick = |x| {
            crate::test_support_fixtures::brick::<Interval>(
                x,
                (0.0, 1.0),
                (0.0, 1.0),
                Tol::witness(),
            )
        };
        let (a, b) = (brick((0.0, 1.0)), brick((2.0, 3.0)));
        fn facing(
            body: &crate::Body<Interval>,
            x: f64,
            which: Which,
        ) -> super::Window<'_, Interval> {
            body.faces()
                .find_map(|(f, _)| {
                    let w = window_of(body, f, which, None).ok()?;
                    let p = w.surface.eval(
                        Interval::from_bounds(w.u.0, w.u.0),
                        Interval::from_bounds(w.v.0, w.v.0),
                    );
                    let normal = w.surface.normal(
                        Interval::from_bounds(w.u.0, w.u.0),
                        Interval::from_bounds(w.v.0, w.v.0),
                    );
                    ((p.x.lo() - x).abs() < 1e-9 && normal.x.lo().abs() > 0.5).then_some(w)
                })
                .expect("the facing face")
        }
        let (wa, wb) = (
            facing(&a, 1.0, Which::First),
            facing(&b, 2.0, Which::Second),
        );
        let pair = CellPair {
            a: Cell::of(&wa),
            b: Cell::of(&wb),
            depth: 0,
            fresh: false,
        };
        let band = Band::linear(Tol::witness()).unwrap();
        let in_band = 0.5 * (band.zero() + band.escalate());
        for (c, separated) in [(1.0 - in_band, false), (0.5, true)] {
            let read = separated_by_support(pair, &wa, &wb, Interval::from_f64(c), band);
            assert_eq!(read, separated, "separated against {c}");
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod depth_rows {
    use geom_core::{Band, Interval, Real, Tol};

    use super::{
        BoundVerdict, Budgets, Cell, CellPair, ClearanceBound, DEFAULT_MAX_CELL_DEPTH, Side, Sweep,
        Which, Window, parted_in_depth, separated_by_support, window_of,
    };
    use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
    use crate::{Body, FaceKey};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// Two cylinder wall sheets in one body, over one azimuth span and
    /// height, sharing nothing.
    fn walls(a: CylFrame, b: CylFrame) -> (Body<f64>, FaceKey, FaceKey) {
        let mut body = Body::<f64>::new();
        let mut wall =
            |frame| cyl_wall_sheet(&mut body, frame, (0.2, 1.4), (0.0, 1.0), Tol::witness());
        let (fa, fb) = (wall(a), wall(b));
        (body, fa, fb)
    }

    /// The injectivity question over the two walls, with a verifier
    /// that confirms nothing.
    fn injective(body: &Body<f64>, faces: [FaceKey; 2]) -> super::BoundReport<()> {
        let side = Side {
            body,
            faces: faces.to_vec(),
        };
        let windows = side.windows(Which::First).expect("each wall has a window");
        let sweep = Sweep {
            bound: ClearanceBound::StrictlyPositive,
            band: band(),
            budgets: Budgets {
                max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
                max_cell_pairs: 128,
            },
        };
        sweep.run(&windows, &windows, true, |_| Err("no witness".to_owned()))
    }

    /// **Two walls a thin gap apart that share nothing are apart at
    /// their root, off the band only, and two that cross are not.**
    /// Coaxial cylinder sheets of radius 1 and 1.001: the pair is apart
    /// in the axis's depth against a bound of zero and of half the gap,
    /// and not against one within the band of the gap, though the
    /// supports along a direction separate the two sheets at no bound.
    /// The injectivity question holds without a split. Three sheets are
    /// not apart in depth: one a gap inside the band away, one whose
    /// gap is past the census's pads by less than the band, and a unit
    /// sheet about an axis 0.1 off the first's, which crosses it at
    /// mid-span. Of the first and the last the question does not hold.
    /// Red where the root's depth gap is not asked (the pair then spends
    /// the row's 128 pairs), where the gap is read by its sign rather
    /// than against the band, where a margin of zero is read as parting,
    /// where the bound is not taken off it, or where a gap that does not
    /// certify parts the pair.
    #[test]
    fn walls_a_thin_gap_apart_are_apart_at_their_root_and_only_off_the_band() {
        let band = band();
        let in_band = 0.5 * (band.zero() + band.escalate());
        let at = Interval::from_f64;
        let apart = |wa: &Window<'_, f64>, wb: &Window<'_, f64>, c: f64| {
            parted_in_depth(wa, wb, at(c), band, Interval::from_certified)
        };
        // The gap the reading takes its bound off, as `depth_gap`
        // computes it: the walls' radii apart, less both faces' pads.
        // Written out here so that a change to the pad reds this row,
        // which is the certificate's guard on a number it does not own.
        let pads = 2.0 * crate::boxes::sweep_pad(band);
        let padded_gap = 1.001 - 1.0 - pads;
        let read = |b: CylFrame| {
            let (body, fa, fb) = walls(CylFrame::canonical(1.0), b);
            let window = |f| window_of(&body, f, Which::First, None).expect("a wall has a window");
            let (wa, wb) = (window(fa), window(fb));
            let pair = CellPair {
                a: Cell::of(&wa),
                b: Cell::of(&wb),
                depth: 0,
                fresh: false,
            };
            (
                [0.0, 5e-4, 1e-3 - in_band, padded_gap].map(|c| apart(&wa, &wb, c)),
                separated_by_support(pair, &wa, &wb, at(0.0), band),
                injective(&body, [fa, fb]),
                parted_in_depth(&wa, &wb, at(0.0), band, crate::test_support_fixtures::trv),
            )
        };

        let (parted, along, report, uncertified) = read(CylFrame::canonical(1.001));
        assert_eq!(
            parted,
            [true, true, false, false],
            "a gap of 0.001, against 0, 0.0005, the gap less half the band, and the padded gap \
             itself, a margin of zero"
        );
        assert!(
            !uncertified,
            "a gap whose crossing does not certify parts nothing"
        );
        assert!(
            !along,
            "the supports along a direction do not separate the whole sheets"
        );
        assert!(
            matches!(report.verdict, BoundVerdict::Holds),
            "the walls hold: {:?}",
            report.verdict
        );
        assert_eq!(
            report.receipt.splits, 0,
            "at their roots: {:?}",
            report.receipt
        );

        let past_the_pads = pads + in_band;
        for (what, other, never_holds) in [
            (
                "a gap in the band",
                CylFrame::canonical(1.0 + in_band),
                true,
            ),
            (
                "a gap past the pads by less than the band",
                CylFrame::canonical(1.0 + past_the_pads),
                false,
            ),
            (
                "a wall that crosses",
                CylFrame {
                    origin: geom_core::Point3::new(0.0718, -0.0696, 0.0),
                    ..CylFrame::canonical(1.0)
                },
                true,
            ),
        ] {
            let (parted, _, report, _) = read(other);
            assert_eq!(parted, [false; 4], "{what}: not apart in depth");
            assert!(
                !(never_holds && matches!(report.verdict, BoundVerdict::Holds)),
                "{what}: the question does not hold: {:?}",
                report.verdict
            );
        }
    }

    /// **No wall pair is parted in depth past its sampled distance.**
    /// 600 pairs of trimmed cylinder wall sheets, each in its own body:
    /// coaxial a thin or a wide gap apart, the axis nudged off or
    /// tilted, or in general position. No pair is parted by more than
    /// the least distance between 49 × 13 points of each, and none is
    /// parted at all with two of those points within the pads. The
    /// draws are one fixed stream: the row is the review's probe of PR
    /// 7119 kept as it was run, and its floor (over 300 pairs drawn,
    /// over 50 parted) says the stream still reaches the reading.
    #[test]
    fn no_wall_pair_is_parted_in_depth_past_its_sampled_distance() {
        use geom_core::{Point3, Vec3};
        struct Lcg(u64);
        impl Lcg {
            fn next(&mut self) -> f64 {
                self.0 = self
                    .0
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
            }
            fn sym(&mut self) -> f64 {
                2.0 * self.next() - 1.0
            }
            fn unit(&mut self) -> Vec3<f64> {
                loop {
                    let v = Vec3::new(self.sym(), self.sym(), self.sym());
                    let n = v.norm();
                    if n > 0.2 && n < 1.0 {
                        return v * (1.0 / n);
                    }
                }
            }
        }
        let band = band();
        let at = Interval::from_f64;
        let mut rng = Lcg(0x7119);
        let (mut parted, mut crossing_near, mut draws) = (0usize, 0usize, 0usize);
        for i in 0..600 {
            let axis = rng.unit();
            let u_ref = axis.cross(rng.unit()).normalize();
            let origin = Point3::new(0.0, 0.0, 0.0);
            let ra = 0.3 + 1.5 * rng.next();
            let fa = CylFrame {
                origin,
                axis,
                radius: ra,
                u_ref,
            };
            // The second wall: coaxial a thin or wide gap away, the axis
            // nudged off or tilted, or a wall in general position.
            let kind = i % 4;
            let gap = match i % 3 {
                0 => 1e-3 * rng.next(),
                1 => 0.2 * rng.next(),
                _ => 1e-7 * rng.next(),
            };
            let fb = match kind {
                0 => CylFrame {
                    radius: ra + gap,
                    ..fa
                },
                1 => CylFrame {
                    origin: origin + u_ref * (gap * 1.5 * rng.sym()) + axis * rng.sym(),
                    radius: ra + gap,
                    u_ref: (u_ref + axis.cross(u_ref) * rng.sym()).normalize(),
                    ..fa
                },
                2 => {
                    let tilt = (axis + u_ref * (0.01 * rng.sym())).normalize();
                    CylFrame {
                        origin,
                        axis: tilt,
                        radius: ra + gap,
                        u_ref: tilt.cross(axis.cross(u_ref)).normalize(),
                    }
                }
                _ => {
                    let ax = rng.unit();
                    CylFrame {
                        origin: origin + rng.unit() * (2.0 * rng.next()),
                        axis: ax,
                        radius: 0.3 + 1.5 * rng.next(),
                        u_ref: ax.cross(rng.unit()).normalize(),
                    }
                }
            };
            let span = |rng: &mut Lcg| {
                let u0 = 0.1 + 2.0 * rng.next();
                let v0 = rng.sym();
                (
                    (u0, u0 + 0.3 + 3.0 * rng.next()),
                    (v0, v0 + 0.2 + 1.5 * rng.next()),
                )
            };
            let (sa, sb) = (span(&mut rng), span(&mut rng));
            let build = |frame: CylFrame, (us, vs): ((f64, f64), (f64, f64))| {
                std::panic::catch_unwind(|| {
                    let mut body = Body::<f64>::new();
                    let f = cyl_wall_sheet(&mut body, frame, us, vs, Tol::witness());
                    (body, f)
                })
                .ok()
            };
            let (Some((ba, ka)), Some((bb, kb))) = (build(fa, sa), build(fb, sb)) else {
                continue;
            };
            let (Ok(wa), Ok(wb)) = (
                window_of(&ba, ka, Which::First, None),
                window_of(&bb, kb, Which::Second, None),
            ) else {
                continue;
            };
            draws += 1;
            let sample = |f: &CylFrame, (us, vs): ((f64, f64), (f64, f64))| -> Vec<Point3<f64>> {
                let n = 48;
                let m = 12;
                (0..=n)
                    .flat_map(|a| {
                        (0..=m).map(move |b| {
                            f.at::<f64>(
                                us.0 + (us.1 - us.0) * (a as f64) / (n as f64),
                                vs.0 + (vs.1 - vs.0) * (b as f64) / (m as f64),
                            )
                        })
                    })
                    .collect()
            };
            let (pa, pb) = (sample(&fa, sa), sample(&fb, sb));
            let least = pa
                .iter()
                .flat_map(|p| pb.iter().map(move |q| (*p - *q).norm()))
                .fold(f64::INFINITY, f64::min);
            let apart = |c: f64| {
                parted_in_depth(&wa, &wb, at(c), band, Interval::from_certified)
                    || parted_in_depth(&wb, &wa, at(c), band, Interval::from_certified)
            };
            let zero = apart(0.0);
            parted += usize::from(zero);
            crossing_near += usize::from(least < 1e-2);
            assert!(
                !apart(least),
                "draw {i} kind {kind}: read apart by more than the sampled least distance {least:e}\n{fa:?} {sa:?}\n{fb:?} {sb:?}"
            );
            assert!(
                !(zero && least < 2.0 * crate::boxes::sweep_pad(band)),
                "draw {i} kind {kind}: read apart with a sampled pair {least:e} apart"
            );
        }
        println!("draws {draws}, read apart {parted}, sampled within 1e-2: {crossing_near}");
        assert!(
            draws > 300 && parted > 50,
            "the probe draws and parts pairs"
        );
    }

    /// **Each face's depths are read in its own body, under a metric
    /// bound as under the strict one.** The two walls in two bodies,
    /// the second after walls of its own body's that the first body has
    /// no face for: the pair is parted in depth, either way round, and
    /// the two-body sweep answers at the root, `≥ 0.0005` holding and
    /// `≤ 0.0005` exceeded with no split. Red where one body reads both
    /// faces, or where a metric bound is not asked the depth gap (each
    /// then spends the row's 128 pairs).
    #[test]
    fn walls_in_two_bodies_are_each_read_in_their_own_under_either_bound() {
        let (inner, fa, _) = walls(CylFrame::canonical(1.0), CylFrame::canonical(3.0));
        let (outer, fb) = {
            let mut body = Body::<f64>::new();
            let mut wall = |r| {
                cyl_wall_sheet(
                    &mut body,
                    CylFrame::canonical(r),
                    (0.2, 1.4),
                    (0.0, 1.0),
                    Tol::witness(),
                )
            };
            let faces = [wall(5.0), wall(7.0), wall(1.001)];
            (body, faces[2])
        };
        assert!(
            inner.faces().all(|(k, _)| k != fb),
            "the first body has no face of the second wall's key"
        );
        let wa = window_of(&inner, fa, Which::First, None).expect("a wall has a window");
        let wb = window_of(&outer, fb, Which::Second, None).expect("a wall has a window");
        let at = Interval::from_f64;
        for (x, y) in [(&wa, &wb), (&wb, &wa)] {
            assert!(
                parted_in_depth(x, y, at(0.0), band(), Interval::from_certified),
                "walls 0.001 apart, each in its own body"
            );
        }
        let side = |body, face| Side {
            body,
            faces: vec![face],
        };
        let (a, b) = (side(&inner, fa), side(&outer, fb));
        let xa = a.windows(Which::First).expect("a wall has a window");
        let xb = b.windows(Which::Second).expect("a wall has a window");
        for (bound, holds) in [
            (ClearanceBound::AtLeast(at(5e-4)), true),
            (ClearanceBound::AtMost(at(5e-4)), false),
        ] {
            let sweep = Sweep {
                bound,
                band: band(),
                budgets: Budgets {
                    max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
                    max_cell_pairs: 128,
                },
            };
            let report: super::BoundReport<()> =
                sweep.run(&xa, &xb, false, |_| Err("no witness".to_owned()));
            assert_eq!(
                (
                    matches!(report.verdict, BoundVerdict::Holds),
                    matches!(report.verdict, BoundVerdict::Exceeded),
                    report.receipt.splits
                ),
                (holds, !holds, 0),
                "{bound:?} answers at the root: {:?} {:?}",
                report.verdict,
                report.receipt
            );
        }
    }

    /// **A side read through a motion is not asked its depths.** Two
    /// coaxial walls of radius 1 and 1.001, each in its own body: in one
    /// frame they are parted in depth. With the second read through a
    /// shift of 0.5 across the axis the two cross, and they are not
    /// parted, either way round, nor are they with the shift's inverse
    /// on the first. Red where a placed window's cover is read in its
    /// body's frame against the other's in the question's: the crossing
    /// walls then read 0.001 apart.
    #[test]
    fn walls_read_through_a_motion_are_not_parted_in_depth() {
        let wall = |r| {
            let mut body = Body::<f64>::new();
            let f = cyl_wall_sheet(
                &mut body,
                CylFrame::canonical(r),
                (0.2, 1.4),
                (0.0, 1.0),
                Tol::witness(),
            );
            (body, f)
        };
        let ((inner, fa), (outer, fb)) = (wall(1.0), wall(1.001));
        let shift = |x: f64| {
            geom_core::RigidMotion::translation(geom_core::Vec3::new(x, 0.0, 0.0))
                .expect("a finite shift")
        };
        let parted = |a: Option<f64>, b: Option<f64>| {
            let (ma, mb) = (a.map(shift), b.map(shift));
            let wa = window_of(&inner, fa, Which::First, ma.as_ref()).expect("a wall's window");
            let wb = window_of(&outer, fb, Which::Second, mb.as_ref()).expect("a wall's window");
            let at = |x: &Window<'_, f64>, y: &Window<'_, f64>| {
                parted_in_depth(
                    x,
                    y,
                    Interval::from_f64(0.0),
                    band(),
                    Interval::from_certified,
                )
            };
            (at(&wa, &wb), at(&wb, &wa))
        };
        assert_eq!(parted(None, None), (true, true), "in one frame");
        assert_eq!(
            parted(None, Some(0.5)),
            (false, false),
            "the second shifted"
        );
        assert_eq!(
            parted(Some(-0.5), None),
            (false, false),
            "the first shifted"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod seated_windows {
    use geom_core::{Band, Point3, RigidMotion, Tol, UnitVec3, Vec3};

    use super::{Budgets, ClearanceBound, DEFAULT_MAX_CELL_DEPTH, Side, Sweep, Which};

    /// **A trimmed face read across a turn sweeps as the placed face does**
    /// (`[placement.frame]`; [`Side::windows_at`]). A torus patch, its
    /// window the whole torus and its face a quarter of it, against a
    /// brick that cuts the torus away from the patch, so the only cells
    /// of the window near the brick hold none of the face, and the sweep
    /// holds only by reading them off it.
    /// Read with the patch built where a skew motion puts it, and with the
    /// patch held in its own coordinates and its window read through that
    /// motion,
    /// the sweep holds in both, each reading cells off the face as off it
    /// and deciding some; and a cell straddling the patch's edge reads its
    /// region alike in both, undecided. The straddling cell's region carries
    /// the pin against a window read without the placed face's own lift:
    /// the Holds and Violated asserts pass under that mutant, the region
    /// assert does not.
    ///
    /// A second brick stands 0.1 off the patch's middle, and `≥ 0.3` is
    /// violated in both frames: the cells there read on the face.
    ///
    /// *Breaks if* a cell's region is read against the placed carrier and
    /// the face's own boundary together (the window's own lift dropped):
    /// the two frames disagree, every cell reads clear of the boundary, and
    /// a cell straddling the patch's edge reads by its centre alone.
    #[test]
    fn a_trimmed_face_read_across_a_turn_sweeps_as_the_placed_face() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let axis = UnitVec3::new(Vec3::new(1.0, -2.0, 0.5), "a skew axis", band).unwrap();
        let m = RigidMotion::translation(Vec3::new(7.0, -3.0, 4.0))
            .unwrap()
            .compose(RigidMotion::rotation(axis, 0.9).unwrap())
            .unwrap();
        let (patch, face) = crate::boxes::tests::torus_wall(
            Point3::origin(),
            Vec3::unit_z(),
            Vec3::unit_x(),
            2.0,
            0.5,
            (0.3, 1.9),
            (-0.7, 0.8),
        );
        let (moved, moved_face) = crate::boxes::tests::torus_wall(
            m.transform_point(Point3::origin()),
            m.transform_vec(Vec3::unit_z()),
            m.transform_vec(Vec3::unit_x()),
            2.0,
            0.5,
            (0.3, 1.9),
            (-0.7, 0.8),
        );
        let brick: crate::Body<f64> = crate::transform_rigid(
            &crate::test_support_fixtures::brick((2.0, 2.6), (-1.5, -0.9), (-0.3, 0.3), tol),
            &m,
        )
        .unwrap();
        let brick_side = Side {
            body: &brick,
            faces: brick.faces().map(|(k, _)| k).collect(),
        };
        let wb = brick_side.windows(Which::Second).unwrap();
        let sweep = Sweep {
            bound: ClearanceBound::at_least(0.3),
            band,
            budgets: Budgets {
                max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
                max_cell_pairs: 200_000,
            },
        };
        fn windows<'b>(
            (body, face): (&'b crate::Body<f64>, crate::FaceKey),
            at: Option<&RigidMotion<f64>>,
        ) -> super::Windows<'b, f64> {
            Side {
                body,
                faces: vec![face],
            }
            .windows_at(Which::First, at)
            .unwrap()
        }
        let run = |placed, at| {
            sweep.run(&windows(placed, at), &wb, false, |_| {
                Err::<(), _>("no witness".to_owned())
            })
        };
        let (held, seated) = (
            run((&moved, moved_face), None),
            run((&patch, face), Some(&m)),
        );
        for (what, report) in [("placed", &held), ("read through its motion", &seated)] {
            assert_eq!(
                report.verdict,
                super::BoundVerdict::Holds,
                "{what}: the brick cuts the torus off the patch, so only cells off the face \
                 come near it: {:?}",
                report.receipt
            );
            assert!(
                report.receipt.outside > 0 && report.decided > 0,
                "{what}: the sweep reads cells off the face as off it: {:?}",
                report.receipt
            );
        }
        // A second brick, 0.1 past the patch's middle along its outward
        // radial: the cells there hold the face, so `≥ 0.3` is violated,
        // and the sweep sees it only if it reads those cells on the face.
        let radial = Vec3::new(1.1f64.cos(), 1.1f64.sin(), 0.0);
        let at = Point3::origin() + radial * 2.6;
        let near: crate::Body<f64> = crate::transform_rigid(
            &crate::test_support_fixtures::brick(
                (at.x - 0.02, at.x + 0.02),
                (at.y - 0.02, at.y + 0.02),
                (-0.02, 0.02),
                tol,
            ),
            &m,
        )
        .unwrap();
        let wn = Side {
            body: &near,
            faces: near.faces().map(|(k, _)| k).collect(),
        }
        .windows(Which::Second)
        .unwrap();
        // One cell straddling the patch's `u = 1.9` edge reads its region
        // alike in both frames: undecided, the edge inside its reach, its
        // centre just off the face.
        let cell_u = (1.88, 1.98);
        let cell_v = (-0.05, 0.05);
        let region = |placed: (&crate::Body<f64>, crate::FaceKey), motion| {
            let w = super::window_of(placed.0, placed.1, Which::First, motion).unwrap();
            let cell = super::Cell::on(&w.surface, cell_u, cell_v);
            super::RegionMemo::at(band).ask(false, 0, &w, cell)
        };
        let here = region((&moved, moved_face), None);
        assert_eq!(
            here,
            crate::cell_region::CellRegion::Undecided,
            "the premise: the cell straddles the patch's edge"
        );
        assert_eq!(
            region((&patch, face), Some(&m)),
            here,
            "the straddling cell's region, read through the motion"
        );
        for (what, placed, motion) in [
            ("placed", (&moved, moved_face), None),
            ("read through its motion", (&patch, face), Some(&m)),
        ] {
            let report = sweep.run(&windows(placed, motion), &wn, false, |_| {
                Ok::<(), String>(())
            });
            assert!(
                matches!(report.verdict, super::BoundVerdict::Violated(())),
                "{what}: a brick 0.1 off the face's middle violates `≥ 0.3`: {:?} {:?}",
                report.verdict,
                report.receipt
            );
        }
    }
}

/// **The stop at two sites no built body reaches**, driven at the
/// engine: a face its kind does not make injective, and a witness the
/// verifier declines.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod first_finding_rows {
    use std::collections::BTreeMap;
    use std::ops::ControlFlow;

    use super::{
        AtRefusal, Bound, BoundVerdict, Bucket, Budgets, Candidate, Cell, CellPair, CellReceipt,
        ClearanceBound, DischargeWidths, EngineRefusal, LocalMemo, Offer, Open, Question, Reading,
        Readings, RegionMemo, Shared, Side, Sweep, Task, Walk, Which, WindowFaces, Windows,
    };
    use crate::Body;
    use geom::Surface;
    use geom_core::{Band, Interval, Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn brick() -> Body<Interval> {
        crate::test_support_fixtures::brick::<Interval>(
            (0.0, 1.0),
            (0.0, 1.0),
            (0.0, 1.0),
            Tol::witness(),
        )
    }

    fn windows(body: &Body<Interval>) -> Windows<'_, Interval> {
        Side {
            body,
            faces: body.faces().map(|(k, _)| k).collect(),
        }
        .windows(Which::First)
        .unwrap()
    }

    fn strict() -> Sweep {
        Sweep {
            bound: ClearanceBound::StrictlyPositive,
            band: band(),
            budgets: Budgets {
                max_cell_depth: super::DEFAULT_MAX_CELL_DEPTH,
                max_cell_pairs: 65_536,
            },
        }
    }

    /// **A face its kind does not make injective ends the ask where it
    /// is met.** A brick's second window is given a torus whose tube is
    /// as wide as its ring. The first face is discharged by its kind, the
    /// second refuses, and every other candidate is abandoned unread. Red
    /// where the site keeps its refusal and reads on: nothing is
    /// abandoned.
    #[test]
    fn a_face_refused_by_its_kind_ends_the_ask_before_the_walk() {
        let body = brick();
        let mut w = windows(&body);
        let p = |x: f64| Interval::from_bounds(x, x);
        w.of[1].surface = Surface::Torus {
            frame: geom::Frame::new(
                Point3::new(p(0.0), p(0.0), p(0.0)),
                Vec3::new(p(0.0), p(0.0), p(1.0)),
                Vec3::new(p(1.0), p(0.0), p(0.0)),
            ),
            major_radius: p(1.0),
            minor_radius: p(1.0),
        };
        let face = w.of[1].face;
        let report = strict().run(&w, &w, true, |_| Err::<(), _>("unasked".to_owned()));
        assert!(
            matches!(
                &report.verdict,
                BoundVerdict::Refused(EngineRefusal::TorusNotRing { face: f, .. }) if *f == face
            ),
            "{:?}",
            report.verdict
        );
        let r = report.receipt;
        assert!(r.holds(), "{r:?}");
        assert_eq!(
            (r.discharged, r.by_kind, r.refused, r.splits),
            (1, 1, 1, 0),
            "one face by its kind, then the refusal: {r:?}"
        );
        assert_eq!(r.abandoned, r.candidates - 2, "the rest unread: {r:?}");
    }

    /// **A witness the verifier declines is a refusal on its two faces,
    /// and ends the question that ends at one.** A pair the interval pass
    /// classed violating, offered to a verifier that declines: under
    /// [`AtRefusal::End`] the sweep ends with the pair counted refused;
    /// under [`AtRefusal::ReadOn`] it goes on, the pair counted as the
    /// interval pass classed it. Red where the site reads on under `End`,
    /// or lands the pair it ends on among the violated.
    #[test]
    fn a_declined_witness_names_its_faces_and_ends_the_question_that_ends_at_a_refusal() {
        let body = brick();
        let w = windows(&body);
        let (x, y) = (&w.of[0], &w.of[1]);
        let sweep = strict();
        for (at_refusal, flow) in [
            (AtRefusal::End, ControlFlow::Break(Bucket::Refused)),
            (AtRefusal::ReadOn, ControlFlow::Continue(Bucket::Violated)),
        ] {
            let mut q = Bound {
                sweep: &sweep,
                injective: true,
                shared: &[],
                at_refusal,
                faces: WindowFaces::of(&w.of, &w.of),
                region: RegionMemo::at(band()),
                locals: LocalMemo::default(),
                across: BTreeMap::new(),
                cornered: 0,
                widths: DischargeWidths::empty(),
                lower_refusal: None,
                upper_refusal: None,
                violation: None,
                within: None,
                box_dependent: core::cell::Cell::new(None),
                verify: |_: Candidate<'_, '_, Interval>| Err::<(), _>("declined".to_owned()),
            };
            let offer = Offer {
                x,
                ca: Cell::of(x),
                ia: 0,
                y,
                cb: Cell::of(y),
                ib: 1,
                meeting: None,
            };
            let readings = Readings {
                below: Some(Reading::Answers),
                above: None,
            };
            assert_eq!(q.settle(&readings, &offer), flow, "{at_refusal:?}");
            assert_eq!(
                q.lower_refusal,
                Some(EngineRefusal::WitnessUnverified {
                    what: "declined".to_owned(),
                    a: x.face,
                    b: y.face,
                }),
                "{at_refusal:?}"
            );
        }
    }

    /// **A pair the walk ends on for a declined witness is counted
    /// refused, once.** Two faces of a brick that share nothing, their
    /// cells' enclosures set to one point, so the interval pass classes
    /// the pair at zero and offers it; the verifier declines. Through the
    /// question's own `pair`: the walk breaks, and the receipt reads one
    /// refused and none violated. Red where the pair is counted among
    /// the violated.
    #[test]
    fn a_pair_ended_on_for_a_declined_witness_is_counted_refused() {
        let body = brick();
        let w = windows(&body);
        let (i, j) = (0..w.of.len())
            .flat_map(|i| (i + 1..w.of.len()).map(move |j| (i, j)))
            .find(|&(i, j)| Shared::of(&w.of[i].boundary, &w.of[j].boundary, false).is_empty())
            .expect("a brick has opposite faces");
        let (x, y) = (&w.of[i], &w.of[j]);
        let at = Interval::from_bounds(0.5, 0.5);
        let point = |mut c: Cell| {
            c.enc = Point3::new(at, at, at);
            c
        };
        let sweep = strict();
        let shared = [Shared::default()];
        let mut q = Bound {
            sweep: &sweep,
            injective: true,
            shared: &shared,
            at_refusal: AtRefusal::End,
            faces: WindowFaces::of(&w.of, &w.of),
            region: RegionMemo::at(band()),
            locals: LocalMemo::default(),
            across: BTreeMap::new(),
            cornered: 0,
            widths: DischargeWidths::empty(),
            lower_refusal: None,
            upper_refusal: None,
            violation: None,
            within: None,
            box_dependent: core::cell::Cell::new(None),
            verify: |_: Candidate<'_, '_, Interval>| Err::<(), _>("declined".to_owned()),
        };
        let mut walk = Walk {
            budgets: sweep.budgets,
            receipt: CellReceipt {
                candidates: 1,
                ..CellReceipt::default()
            },
            left: 0,
            next: Vec::new(),
        };
        let task = Task {
            k: 0,
            i,
            j,
            pair: CellPair {
                a: point(Cell::of(x)),
                b: point(Cell::of(y)),
                depth: 0,
                fresh: true,
            },
            open: Open {
                below: true,
                above: false,
            },
        };
        assert!(q.pair(&mut walk, task, x, y).is_break(), "the walk ends");
        assert!(
            matches!(
                q.lower_refusal,
                Some(EngineRefusal::WitnessUnverified { a, b, .. }) if a == x.face && b == y.face
            ),
            "{:?}",
            q.lower_refusal
        );
        let r = walk.receipt;
        assert_eq!((r.refused, r.violated), (1, 0), "{r:?}");
        assert!(r.holds(), "{r:?}");
    }
}
