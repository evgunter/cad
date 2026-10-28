//! **The E7 clearance engine's body-level half**: the inner subdivision
//! of two sets of faces' GEOMETRY domains, which answers `separation ≥
//! c` as a trichotomy ([`Sweep`]) and "what IS the minimum separation"
//! as a certified bracket ([`min_separation`]).
//!
//! It reads a body, the proximity tree and the carriers, and nothing
//! else. Which document, node or parameter box a body came from is the
//! caller's: a [`Side`] carries a body and the faces in scope, no node
//! address and no frame, so a caller reads both operands into one frame
//! before handing them over and says itself whether the two sides are
//! one body ([`Sweep::run`]'s `same_body`).
//!
//! The door is generic over the lane scalar and the arithmetic is not.
//! Every surface, curve and point the door reads is lifted through
//! [`Interval::from_certified`], and cells, enclosures, the funnelled
//! `d − c` and the bracket are interval arithmetic; at `T = Interval`
//! the lift keeps every bound. On a window that is a superset of its
//! face, the cells' and stations' point-in-face decisions run at `T`
//! ([`crate::cell_region`]); a window that IS its face (an iso-bounded
//! NURBS patch) asks neither door, since every cell and station of it
//! is on the face.
//!
//! # Two questions, one subdivision
//!
//! Same split rule, and the difference is what the walk does with a
//! cell pair: the bound question DECIDES each pair at a funnel site;
//! the minimum question decides nothing at all — it takes minima of
//! enclosure endpoints, so it funnels no predicate and consults no
//! tolerance. That door's own header carries the argument.
//!
//! **The windows and the cells are not the same, either**, and for
//! that same reason. The bound question's sweep drops the cells decided
//! outside their faces; [`min_separation`] keeps every cell of the
//! carrier rectangles, because a drop is a decision and that door takes
//! none (its own header). So
//! `window_hi` is a minimum over strictly more of the carrier than a
//! `Holds` covers, and the asymmetry is in the direction its own docs
//! already state — except between windows that are their faces, where
//! the two sets are one.
//!
//! **The sweep stops at the first VERIFIED violation.** A cell pair the
//! interval pass classifies as violating is handed to the caller's
//! verifier ([`Candidate`]), and a witness is the deliverable:
//! continuing past one buys a bigger receipt and no more truth. What
//! was on the frontier when it stopped is counted
//! [`CellReceipt::abandoned`] — never folded into `refused`, which
//! would be a claim that it was tried. A candidate the verifier does
//! not confirm is [`EngineRefusal::WitnessUnverified`], and the sweep
//! goes on.
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
//! a superset: it is admitted only where the face is the whole patch
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
//! # No ε here, and two funnelled compares of its own
//!
//! Every decision goes through the one `k_stats` funnel at a named
//! predicate with a [`Margin`] door. This module decides exactly two:
//! [`CLEARANCE_MARGIN`] (`d − c`, metres minus metres) and
//! [`SELF_INTERSECTION_GAP`] (`d`, the separation a non-adjacent face
//! pair must classify strictly positive). The cell and station verdicts
//! it asks for decide on [`crate::cell_region`]'s rows, inside those
//! doors. All carry a row in `docs/predicate-dimension-audit.md`. No
//! DECISION here compares a float raw, mints a tolerance, or reads a
//! width as evidence.
//!
//! One raw compare survives, and it is not a decision: the proximity
//! tree's admission test, which decides which pairs are EXAMINED
//! rather than what any of them means. It is padded by the funnel's
//! own escalate threshold (`c + band.escalate()`), so a pair it drops
//! is one the funnel would have discharged definitely — the two agree
//! in the only direction that matters, and everything inside the band
//! reaches the funnel. Stated because a reviewer should meet it here
//! rather than find it.

use std::collections::{BTreeMap, BTreeSet};

use bvh::{Aabb, Bvh};
use geom::{NurbsSurface, Surface};
use geom_brep::{Channel, Pcurve};
use geom_core::interval::Interval;
use geom_core::k_stats::decide;
use geom_core::{Band, Bounds, CertifiedBounds, Decide, Margin, Point2, Point3, Real, Sign, Vec3};

use crate::Body;
use crate::FaceContainment;
use crate::cell_region::{Ball, CellRegion, cell_region, station_region};
use crate::entity::{EdgeKey, Face, FaceKey, HalfEdgeKey, LoopBoundary, VertexKey};
use crate::injective::{self, Axis, Boundary, Iso, Local, Near, Outcome, Piece, Rim, Shared, Use};

/// The funnel site name of the clearance comparison — the separation
/// enclosure between two domain cells minus the requested clearance,
/// in metres against the linear band.
///
/// A roster carrier (`docs/K-REPORT.md`) rather than a literal at the
/// decide site.
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

/// The whole-query cell-pair budget of ONE [`min_separation`] query.
///
/// A recorded run dial. The bracket a budget buys is honest at any
/// width ([`MinSeparation`]), so the dial trades tightness for time and
/// never truth.
pub const DEFAULT_MAX_MIN_SEP_PAIRS: usize = 512;

/// Which question the engine is answering — the two readings of a
/// definite `Sign::Zero`, each at its own funnel site.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClearanceBound {
    /// `separation ≥ c` (metres), non-strict: a separation equal to `c`
    /// at the run's tolerance HOLDS. The assertion lane's convention.
    AtLeast(f64),
    /// Strictly positive separation — E7's self-intersection question.
    /// A coincidence at the run's tolerance is a VIOLATION, not a
    /// discharge.
    StrictlyPositive,
}

impl ClearanceBound {
    /// The metre bound the margin is taken against.
    pub fn c(self) -> f64 {
        match self {
            Self::AtLeast(c) => c,
            Self::StrictlyPositive => 0.0,
        }
    }

    /// The funnel site this bound decides at.
    pub fn predicate(self) -> &'static str {
        match self {
            Self::AtLeast(_) => CLEARANCE_MARGIN,
            Self::StrictlyPositive => SELF_INTERSECTION_GAP,
        }
    }

    /// Whether a definite `Zero` discharges (see the two site docs).
    pub fn zero_discharges(self) -> bool {
        matches!(self, Self::AtLeast(_))
    }
}

/// **One side of a clearance question**: a body, and the faces of it in
/// scope.
///
/// No node address and no frame. Where the body came from is the
/// caller's to keep, and the two sides of one question are read into
/// one frame before they reach the engine.
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
    /// in the order of [`Self::faces`].
    ///
    /// # Errors
    ///
    /// [`EngineRefusal::Unsupported`] for the first face whose carrier
    /// has no admitted window.
    pub fn windows(&self) -> Result<Windows<'b, T>, EngineRefusal> {
        let of = self
            .faces
            .iter()
            .map(|&k| window_of(self.body, k))
            .collect::<Result<_, _>>()?;
        Ok(Windows { of })
    }
}

/// One side's faces read into carrier windows: what [`Sweep::run`]
/// subdivides.
#[derive(Debug, Clone)]
pub struct Windows<'b, T: Real> {
    of: Vec<Window<'b, T>>,
}

/// Why the engine has no answer. Typed and named, never a silent
/// partial.
///
/// These are the arms the engine itself raises. A caller that resolves
/// selections, folds leaves or checks the bound before it asks wraps
/// them beside its own.
#[derive(Debug, Clone, PartialEq)]
pub enum EngineRefusal {
    /// A genuine semantic sliver: the deciding enclosure sits WHOLLY
    /// inside the ambiguity band, so the separation being decided IS in
    /// the band and refinement cannot move it out ([`sliver`]).
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
    /// A carrier enclosure that did not evaluate: the margin came back
    /// [`geom_core::MarginKind::Invalid`] (NaI, or an empty enclosure),
    /// which is neither an indeterminacy refinement could settle nor a
    /// budget. Its own class so a reader is not sent looking for a
    /// bigger dial.
    PoisonEnclosure {
        /// The first face of the pair.
        a: FaceKey,
        /// The second.
        b: FaceKey,
    },
    /// A scope naming no face. `Holds` over an empty selection is
    /// vacuously true and useless; a caller that asked about nothing is
    /// told so.
    EmptyScope,
    /// A touching cell pair of the injectivity question that no local
    /// certificate discharged down to the depth floor, or that no arm
    /// can read at any depth: the two faces, the cell they share, and
    /// the arm that failed.
    Uncertified {
        /// The first face of the pair.
        a: FaceKey,
        /// The second.
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
        /// The vertex the chart collapses to.
        vertex: VertexKey,
    },
    /// A face on a torus not decided a ring (`R > r`): the carrier meets
    /// itself, so the face is not injective by its kind.
    TorusNotRing {
        /// The face.
        face: FaceKey,
    },
    /// A cell pair the interval pass classified as a definite violation
    /// whose witness the caller's verifier did not confirm. The engine
    /// reports what the verifier returned, verbatim, and goes on with
    /// the sweep; whether the arm can fire is the verifier's property,
    /// stated at the verifier.
    ///
    /// Its own arm, and never a violation: a witness is a claim about a
    /// concrete configuration, and one the independent recomputation
    /// contradicts is evidence about the analysis, not about the
    /// geometry.
    WitnessUnverified {
        /// What the verifier could not confirm.
        what: String,
    },
}

impl EngineRefusal {
    /// The refusal's own payload, rendered for the goldening form — the
    /// half `name` drops, so two runs that refuse for the same CLASS on
    /// different evidence do not serialize alike.
    pub fn payload(&self) -> String {
        match self {
            Self::Sliver { predicate } => (*predicate).to_owned(),
            Self::Budget(k) => format!("{k:?}"),
            Self::Unsupported { carrier, face } => format!("{carrier} {face:?}"),
            Self::WitnessUnverified { what } => what.clone(),
            Self::PoisonEnclosure { a, b } => format!("{a:?}/{b:?}"),
            Self::Uncertified { a, b, shared, arm } => {
                format!("{a:?}/{b:?} {} {}", shared.payload(), arm.name())
            }
            Self::CollapsedChart { face, vertex } => format!("{face:?} {vertex:?}"),
            Self::TorusNotRing { face } => format!("{face:?}"),
            Self::EmptyScope => String::new(),
        }
    }

    /// The refusal's stable class name, for reports and the goldening
    /// form.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Sliver { .. } => "sliver",
            Self::Budget(_) => "budget",
            Self::Unsupported { .. } => "unsupported",
            Self::WitnessUnverified { .. } => "witness_unverified",
            Self::PoisonEnclosure { .. } => "poison_enclosure",
            Self::EmptyScope => "empty_scope",
            Self::Uncertified { .. } => "uncertified",
            Self::CollapsedChart { .. } => "collapsed_chart",
            Self::TorusNotRing { .. } => "torus_not_ring",
        }
    }
}

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

/// Which bound stopped the subdivision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellBudget {
    /// One cell pair had already been split
    /// [`Sweep::max_cell_depth`] times.
    Depth {
        /// The bound.
        max_cell_depth: u32,
    },
    /// The query had reached [`Sweep::max_cell_pairs`]; this pair was
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
        .then_some(source.predicate.unwrap_or("<unnamed>"))
}

/// The counting receipt of one clearance query.
///
/// Each candidate pair is the root of its own binary subdivision: a
/// cell pair is discharged, violated, refused, split into exactly two,
/// or ABANDONED where the sweep stopped. A forest of `candidates`
/// binary trees with `splits` interior nodes has exactly
/// `splits + candidates` leaves, and every leaf is in one of the four
/// buckets. It rides the shipped report, so a consumer re-checks it
/// without trusting this module ([`CellReceipt::holds`]).
///
/// **`abandoned` is what early exit costs the receipt, spelled rather
/// than hidden.** The sweep stops at the first verified violation, so
/// the frontier it was holding is neither discharged nor refused — it
/// was never tried, and calling it `refused` would be a claim that it
/// was. A query that runs to completion has `abandoned == 0`.
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
    /// Cell pairs the sweep never classified because it had already
    /// verified a violation and stopped.
    pub abandoned: usize,
}

impl CellReceipt {
    /// The receipt identity (see the type docs).
    pub fn holds(&self) -> bool {
        self.discharged + self.violated + self.refused + self.abandoned
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

/// **The bound question's walk**: the question, the band it is decided
/// at, and the two budgets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sweep {
    /// Which question ([`ClearanceBound`]).
    pub bound: ClearanceBound,
    /// The linear band every compare classifies against: the clearance
    /// site's and the cells' and stations' ([`crate::cell_region`]).
    pub band: Band,
    /// Per-pair subdivision depth ([`DEFAULT_MAX_CELL_DEPTH`]).
    pub max_cell_depth: u32,
    /// Whole-query cell-pair budget, ENFORCED AT ADMISSION: a split that
    /// would commit the query past this number is refused instead of
    /// taken.
    pub max_cell_pairs: usize,
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
    /// **How many readings of the faces' regions the answer rests on**:
    /// distinct (window, cell) pairs [`crate::cell_region`] read as
    /// `Inside` or `Outside`, whichever partner they were paired with,
    /// and distinct witness stations a face's point door placed
    /// ([`crate::cell_region::station_region`]). Zero says the faces'
    /// regions were never read into the answer.
    pub decided: usize,
}

/// The E7 trichotomy, with the verifier's witness in the violated arm.
#[derive(Debug, Clone, PartialEq)]
pub enum BoundVerdict<W> {
    /// The bound holds over both carrier windows minus the cells decided
    /// outside their faces.
    Holds,
    /// A definite violation, confirmed by the verifier.
    Violated(W),
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
    /// The strict question, whose witness is a coincidence.
    strict: bool,
}

impl<T: Decide + CertifiedBounds> Candidate<'_, '_, T> {
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
        if self.strict
            && let Some((p, q)) = coincidence(self.x, self.ca, &a, self.y, self.cb, &b)
            && self.region.placed(false, self.ia, self.x, p)
            && self.region.placed(true, self.ib, self.y, q)
        {
            a.push(p);
            b.push(q);
        }
        (a, b)
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
    ///   ([`Candidate::meeting`]).
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
    /// **It stops at the first VERIFIED violation.** `verify` is asked,
    /// in that shallowest-first order, about every cell pair the funnel
    /// classifies violating, and about an indeterminate pair that does
    /// not touch, at the root and at the depth floor; its first `Ok` is
    /// the verdict. An `Err` on a pair classified violating is
    /// [`EngineRefusal::WitnessUnverified`] carrying its prose. A
    /// touching pair is never offered: its two cells share a point of M,
    /// and a pair of coincident stations there is no violation.
    pub fn run<T, W, F>(
        &self,
        wa: &Windows<'_, T>,
        wb: &Windows<'_, T>,
        same_body: bool,
        mut verify: F,
    ) -> BoundReport<W>
    where
        T: Decide + CertifiedBounds,
        F: FnMut(Candidate<'_, '_, T>) -> Result<W, String>,
    {
        let (wa, wb) = (&wa.of[..], &wb.of[..]);
        let c = self.bound.c();
        let injective = same_body && self.bound == ClearanceBound::StrictlyPositive;
        // **The prune threshold carries the BAND.** The tree excludes a
        // pair on a raw `separation_lo > pad`, and the engine decides
        // through the funnel, which calls anything within `escalate` of
        // `c` indeterminate. Padding by the band is what makes the two
        // agree in the only direction that matters: a pair this drops is
        // separated by more than `c` PLUS the funnel's own escalation
        // threshold, so it is one the funnel would have discharged
        // definitely, and a pair inside the band survives to be
        // classified or refused rather than silently held.
        let reach = c + self.band.escalate();
        let root_a: Vec<Cell> = wa.iter().map(Cell::of).collect();
        let root_b: Vec<Cell> = wb.iter().map(Cell::of).collect();
        let cloud = |c: &Cell| [c.enc];
        let tree_a = Bvh::build_bounded(root_a.iter().map(cloud), 0.0);
        let tree_b = Bvh::build_bounded(root_b.iter().map(cloud), 0.0);
        // The unordered de-duplication is the same-body case's own: two
        // scopes of one body can carry a pair in both orders.
        let mut seen: BTreeSet<(FaceKey, FaceKey)> = BTreeSet::new();
        let candidates: Vec<(usize, usize)> = tree_a
            .pairs_within(&tree_b, reach)
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
        let mut widths = DischargeWidths::empty();
        let mut region = RegionMemo::at(self.band);
        let mut locals = LocalMemo::default();
        let mut deepest = 0;
        let mut first_refusal: Option<EngineRefusal> = None;
        let mut violation: Option<W> = None;

        let mut frontier: Vec<Task> = Vec::with_capacity(candidates.len());
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
                // whole pair is one leaf, `0 − c` decided at the bound's
                // own site.
                let cell = if x.face == y.face {
                    SharedCell::Face(x.face)
                } else {
                    sh.cell(&x.boundary)
                        .unwrap_or(SharedCell::Face(x.face))
                };
                match decide(self.bound.predicate(), Margin::of(Interval::from_bounds(-c, -c)), self.band) {
                    Ok(Sign::Positive) => receipt.discharged += 1,
                    Ok(Sign::Zero) if self.bound.zero_discharges() => receipt.discharged += 1,
                    _ => {
                        receipt.violated += 1;
                        let stations = meeting_stations(x, y, sh, cell, self.band);
                        match verify(Candidate {
                            x,
                            ca: a,
                            ia: i,
                            y,
                            cb: b,
                            ib: j,
                            region: &mut region,
                            meeting: Some((cell, stations.0, stations.1)),
                            strict: false,
                        }) {
                            Ok(w) => {
                                violation = Some(w);
                                receipt.abandoned += (candidates.len() - k - 1) + frontier.len();
                                frontier.clear();
                                break;
                            }
                            Err(what) => {
                                first_refusal
                                    .get_or_insert(EngineRefusal::WitnessUnverified { what });
                            }
                        }
                    }
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
                        first_refusal.get_or_insert(r);
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
            });
        }

        'sweep: while violation.is_none() && !frontier.is_empty() {
            // The whole-frontier budget check, the driver's own: every
            // task on it becomes at least one leaf, so when they cannot
            // fit they are refused together rather than started.
            let final_so_far = receipt.discharged + receipt.violated + receipt.refused;
            if final_so_far + frontier.len() > self.max_cell_pairs {
                receipt.refused += frontier.len();
                frontier.clear();
                first_refusal.get_or_insert(EngineRefusal::Budget(CellBudget::Pairs {
                    max_cell_pairs: self.max_cell_pairs,
                }));
                break;
            }
            let level = frontier.len();
            let mut next: Vec<Task> = Vec::new();
            for (n, task) in frontier.drain(..).enumerate() {
                let (Some(x), Some(y), Some(sh)) =
                    (wa.get(task.i), wb.get(task.j), shared.get(task.k))
                else {
                    // Unreachable: the frontier was built from these
                    // very indices. Counted rather than skipped, so no
                    // path out of this loop can break the receipt.
                    receipt.abandoned += 1;
                    continue;
                };
                let pair = task.pair;
                deepest = deepest.max(pair.depth);
                let trace = std::env::var_os("SELFX_TRACE").is_some();
                let t0 = std::time::Instant::now();
                if trace { eprintln!("[task] d={} {:?}/{:?} a={:?}x{:?} b={:?}x{:?}", pair.depth, x.face, y.face, pair.a.u, pair.a.v, pair.b.u, pair.b.v); }
                // **Off the face, before anything is enclosed.** A cell
                // decided outside its face is a leaf, and the discharge
                // is VACUOUS: no separation is measured, nothing is
                // decided at the clearance site, and no discharge width
                // is recorded, because none was paid. The two roots
                // minus cells decided outside still contain the faces.
                if region.ask(false, task.i, x, pair.a) == CellRegion::Outside
                    || region.ask(true, task.j, y, pair.b) == CellRegion::Outside
                {
                    receipt.discharged += 1;
                    receipt.outside += 1;
                    continue;
                }
                if trace { eprintln!("[time] region {:?}", t0.elapsed()); }
                let discharge = |receipt: &mut CellReceipt, widths: &mut DischargeWidths| {
                    receipt.discharged += 1;
                    let da = diameter(&cell_box(&pair.a.enc));
                    let db = diameter(&cell_box(&pair.b.enc));
                    widths.discharged_at(da.max(db));
                };
                // What is already final, what this level has left to
                // fold, and what is queued for the next level: the
                // admission arithmetic every split below reads.
                let committed = |receipt: &CellReceipt, next: &Vec<Task>| {
                    receipt.discharged
                        + receipt.violated
                        + receipt.refused
                        + (level - n - 1)
                        + next.len()
                };

                // **Touching, in M.** The injectivity question decides a
                // touching pair by a local certificate, never by
                // distance.
                if injective {
                    let same = x.face == y.face;
                    let meet = same && injective::rects_meet((pair.a.u, pair.a.v), (pair.b.u, pair.b.v));
                    // Two cells whose enclosures are disjoint hold no
                    // common point, of M or of space: the enclosures are
                    // rigorous, so this selects the distance test and
                    // decides nothing it would not.
                    let apart = separation(&pair.a.enc, &pair.b.enc).lo() > 0.0;
                    let near = if meet || sh.is_empty() || apart {
                        None
                    } else {
                        let la = locals.get(false, task.i, x, pair.a, self.band);
                        let lb = locals.get(true, task.j, y, pair.b, self.band);
                        let near = Near::of(sh, &la, &lb);
                        (!near.is_empty()).then_some((near, la, lb))
                    };
                    if trace { eprintln!("[time] near {:?} meet={meet} near={}", t0.elapsed(), near.is_some()); }
                    if meet || near.is_some() {
                        let outcome = if meet {
                            let hull = (
                                (pair.a.u.0.min(pair.b.u.0), pair.a.u.1.max(pair.b.u.1)),
                                (pair.a.v.0.min(pair.b.v.0), pair.a.v.1.max(pair.b.v.1)),
                            );
                            if injective::monotone(&x.surface, hull.0, hull.1, self.band) {
                                Outcome::Certified
                            } else {
                                Outcome::Split {
                                    arm: CertificateArm::Monotone,
                                    shared: SharedCell::Face(x.face),
                                }
                            }
                        } else if let Some((near, la, lb)) = &near {
                            let (px, py) = (piece_of(x, pair.a, la), piece_of(y, pair.b, lb));
                            if same {
                                injective::certify_self(&px, &py, near, self.band)
                            } else {
                                injective::certify_pair(&px, &py, near, self.band)
                            }
                        } else {
                            unreachable!("a touching pair meets or is near")
                        };
                        match outcome {
                            Outcome::Certified => {
                                discharge(&mut receipt, &mut widths);
                                receipt.certified += 1;
                            }
                            Outcome::Refuse(r) => {
                                receipt.refused += 1;
                                first_refusal.get_or_insert(r);
                            }
                            Outcome::Split { arm, shared } => {
                                if pair.depth >= 36 && std::env::var_os("SELFX_DEEP").is_some() {
                                    eprintln!("[deep] d={} {:?}/{:?} {arm:?} {shared:?} a={:?}x{:?} b={:?}x{:?}", pair.depth, x.face, y.face, pair.a.u, pair.a.v, pair.b.u, pair.b.v);
                                }
                                let refuse = |receipt: &mut CellReceipt,
                                              first: &mut Option<EngineRefusal>,
                                              r: EngineRefusal| {
                                    receipt.refused += 1;
                                    first.get_or_insert(r);
                                };
                                if pair.depth >= self.max_cell_depth {
                                    refuse(
                                        &mut receipt,
                                        &mut first_refusal,
                                        EngineRefusal::Uncertified {
                                            a: x.face,
                                            b: y.face,
                                            shared,
                                            arm,
                                        },
                                    );
                                    continue;
                                }
                                // A cell paired with itself splits into
                                // its two halves each against itself and
                                // against each other: three pairs, two
                                // splits' worth of leaves.
                                let itself = same && pair.a.same_cell(&pair.b);
                                let halves = if itself { 3 } else { 2 };
                                if committed(&receipt, &next) + halves > self.max_cell_pairs {
                                    refuse(
                                        &mut receipt,
                                        &mut first_refusal,
                                        EngineRefusal::Budget(CellBudget::Pairs {
                                            max_cell_pairs: self.max_cell_pairs,
                                        }),
                                    );
                                    continue;
                                }
                                let split = if itself {
                                    split_self(pair, x).map(|[p, q, r]| vec![p, q, r])
                                } else {
                                    split(pair, x, y).map(|(p, q)| vec![p, q])
                                };
                                match split {
                                    Some(children) => {
                                        receipt.splits += children.len() - 1;
                                        next.extend(children.into_iter().map(|pair| Task {
                                            k: task.k,
                                            i: task.i,
                                            j: task.j,
                                            pair: CellPair {
                                                fresh: true,
                                                ..pair
                                            },
                                        }));
                                    }
                                    None => refuse(
                                        &mut receipt,
                                        &mut first_refusal,
                                        EngineRefusal::Budget(CellBudget::Resolution),
                                    ),
                                }
                            }
                        }
                        continue;
                    }
                }

                let margin = separation(&pair.a.enc, &pair.b.enc) - Interval::from_f64(c);
                let mut candidate = |region: &mut RegionMemo| {
                    verify(Candidate {
                        x,
                        ca: pair.a,
                        ia: task.i,
                        y,
                        cb: pair.b,
                        ib: task.j,
                        region,
                        meeting: None,
                        strict: !self.bound.zero_discharges(),
                    })
                };
                match decide(self.bound.predicate(), Margin::of(margin), self.band) {
                    Ok(Sign::Positive) => discharge(&mut receipt, &mut widths),
                    Ok(Sign::Zero) if self.bound.zero_discharges() => {
                        discharge(&mut receipt, &mut widths);
                    }
                    Ok(Sign::Zero | Sign::Negative) => {
                        receipt.violated += 1;
                        // Verified HERE, not after the sweep: a verified
                        // witness ends the query, and an unverified one
                        // must not.
                        match candidate(&mut region) {
                            Ok(w) => {
                                violation = Some(w);
                                receipt.abandoned += (level - n - 1) + next.len();
                                break 'sweep;
                            }
                            Err(what) => {
                                first_refusal
                                    .get_or_insert(EngineRefusal::WitnessUnverified { what });
                            }
                        }
                    }
                    Err(source) => {
                        // **The exhibit arm.** An indeterminate margin
                        // is not a `Holds`, and for the STRICT question
                        // it is very often not reachable by refinement
                        // either: `d - 0` is the norm of a difference,
                        // so it can never classify Negative, and the
                        // only definite violation is a `Zero` — which
                        // needs both cells narrower than ε, thirty
                        // halvings down from a metre window. A witness,
                        // though, does not need the enclosure to be
                        // narrow: it needs ONE pair of points. So an
                        // indeterminate cell pair is offered to the
                        // verifier before it is split, at the two
                        // places a probe is worth its cost — the root of
                        // the subtree the distance test decides (a
                        // candidate's root, or the first pair below a
                        // touching one), where a contact or an overlap
                        // shows up immediately, and the depth floor,
                        // where the alternative is a refusal.
                        //
                        // It only ever ADDS violations, each confirmed
                        // by the verifier: a probe that finds nothing
                        // changes no answer, so no `Holds` can rest on
                        // it. That is what keeps it an exhibit rather
                        // than a sample.
                        let probe_here = (pair.fresh && std::env::var_os("SELFX_NOPROBE").is_none()) || pair.depth >= self.max_cell_depth;
                        if std::env::var_os("SELFX_TRACE").is_some() { eprintln!("[ind] d={} fresh={} probe={probe_here} {:?}/{:?}", pair.depth, pair.fresh, x.face, y.face); }
                        if probe_here && let Ok(w) = candidate(&mut region) {
                            receipt.violated += 1;
                            violation = Some(w);
                            receipt.abandoned += (level - n - 1) + next.len();
                            break 'sweep;
                        }
                        if source.margin.is_invalid() {
                            // A poison enclosure is not an indeterminacy
                            // refinement could settle, and it is not a
                            // budget: it is geometry that did not
                            // evaluate. Its own class.
                            receipt.refused += 1;
                            first_refusal.get_or_insert(EngineRefusal::PoisonEnclosure {
                                a: x.face,
                                b: y.face,
                            });
                            continue;
                        }
                        if let Some(predicate) = sliver(&source) {
                            receipt.refused += 1;
                            first_refusal.get_or_insert(EngineRefusal::Sliver { predicate });
                            continue;
                        }
                        if pair.depth >= self.max_cell_depth {
                            receipt.refused += 1;
                            first_refusal.get_or_insert(EngineRefusal::Budget(CellBudget::Depth {
                                max_cell_depth: self.max_cell_depth,
                            }));
                            continue;
                        }
                        // The budget is enforced AT ADMISSION, exactly
                        // as the driver's leaf budget is: a split turns
                        // one cell pair into two, so it is refused
                        // unless the leaf count it commits to still
                        // fits.
                        if committed(&receipt, &next) + 2 > self.max_cell_pairs {
                            receipt.refused += 1;
                            first_refusal.get_or_insert(EngineRefusal::Budget(CellBudget::Pairs {
                                max_cell_pairs: self.max_cell_pairs,
                            }));
                            continue;
                        }
                        match split(pair, x, y) {
                            Some((lo, hi)) => {
                                receipt.splits += 1;
                                next.push(Task {
                                    k: task.k,
                                    i: task.i,
                                    j: task.j,
                                    pair: lo,
                                });
                                next.push(Task {
                                    k: task.k,
                                    i: task.i,
                                    j: task.j,
                                    pair: hi,
                                });
                            }
                            None => {
                                receipt.refused += 1;
                                first_refusal
                                    .get_or_insert(EngineRefusal::Budget(CellBudget::Resolution));
                            }
                        }
                    }
                }
            }
            frontier = next;
        }
        widths.deepest = deepest;
        // One count per window the sweep ran over, both sides: a query
        // against itself was handed two copies of one list and counts
        // each face twice, which is what "per window" means here.
        let decided = region.decided();

        let verdict = match (violation, first_refusal) {
            (Some(w), _) => BoundVerdict::Violated(w),
            (None, Some(refusal)) => BoundVerdict::Refused(refusal),
            (None, None) => BoundVerdict::Holds,
        };
        receipt.check_outside_is_a_sub_count();
        BoundReport {
            verdict,
            receipt,
            widths,
            decided,
        }
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
    let at = |x: f64| Interval::from_bounds(x, x);
    let jet = |s: &Surface<Interval>, (u, v): (f64, f64)| {
        let j = s.jet(at(u), at(v));
        (j.point.map(mid), j.du.map(mid), j.dv.map(mid))
    };
    let point = |s: &Surface<Interval>, uv| jet(s, uv).0;
    let mut best: Option<(f64, (f64, f64), (f64, f64))> = None;
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
    fn get<T: Real>(&mut self, side_b: bool, index: usize, w: &Window<'_, T>, c: Cell, band: Band) -> Local {
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
    let at = |(lo, hi): (f64, f64)| {
        let m = geom::mid_param(lo, hi);
        Interval::from_bounds(m, m)
    };
    Ball::holding(surface.eval(at(c.u), at(c.v)), c.enc)
}

// ------------------------------------- the minimum-separation door

/// The two budgets of one [`min_separation`] query. No accelerator
/// switch: the door is handed bodies that were already evaluated over
/// whatever parameter box their caller had.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinSeparationConfig {
    /// Per-pair subdivision depth ([`DEFAULT_MAX_CELL_DEPTH`]).
    pub max_cell_depth: u32,
    /// Whole-query cell-pair budget ([`DEFAULT_MAX_MIN_SEP_PAIRS`]).
    pub max_cell_pairs: usize,
}

impl Default for MinSeparationConfig {
    fn default() -> Self {
        Self {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: DEFAULT_MAX_MIN_SEP_PAIRS,
        }
    }
}

/// **A bracket on the minimum separation between two sides**, over
/// whatever parameter box the two bodies were evaluated over — with its
/// two ends certified about DIFFERENT SETS, which is the thing to read
/// before anything else here.
///
/// # What the two numbers mean, exactly
///
/// Write `m(p)` for the minimum distance between the two sides' carrier
/// WINDOWS at parameter point `p`, `M(p)` for the same over the trimmed
/// FACES, and let the bodies have been evaluated over a box `B`. The
/// windows contain the faces (this module's header), so
///
/// ```text
///     m(p) ≤ M(p)   for every p
/// ```
///
/// and the two ends of this bracket inherit that asymmetry:
///
/// - `lo` is a certified LOWER bound on `inf{ m(p) : p ∈ B }` — the
///   least `separation_lo` over every cell pair the sweep did not
///   discharge, together with the pairs it excluded at admission (by
///   their own `separation_lo`, which is what admission compared).
///   Because `m ≤ M`, it is EQUALLY a lower bound on `inf M` — the
///   faces' own worst-case clearance. **This is the end a defect gate
///   reads, and it is sound for the faces.**
/// - `hi` is a certified upper bound on `sup{ m(p) : p ∈ B }` — the
///   least, over every cell pair the sweep examined, of that pair's
///   `separation_hi` AND of its two centre cells' furthest separation.
///   Each bounds the distance between some realizable point of each
///   WINDOW at every `p ∈ B`. **It is not a bound on `M` in either
///   direction**: the window points that attain it may lie in the trim
///   holes, so `hi` can sit strictly below the faces' true clearance.
///   It is named [`MinSeparation::window_hi`] for that reason.
///
/// **Between two windows that ARE their faces**, `m = M`: an
/// iso-bounded NURBS face's window is its whole patch
/// (`Region::Whole`), so a window point is a face point and `hi`
/// bounds the faces' clearance from above too. A side mixing such
/// faces with analytic ones keeps the asymmetry above, and this door
/// does not say which case a query is; the caller that knows its faces
/// are whole patches may read `hi`.
///
/// So `[lo, window_hi]` is a containment-true enclosure of `m`, and
/// `[lo, ∞)` is what it certifies about `M`. Both readings matter and
/// they are not the same statement: an assertion over this bracket may
/// read `lo` and may not read `hi`.
///
/// A worked counterexample, in editor-core's suite: an L-shaped cap
/// under a block parked over its notch brackets `[0.100…, 0.100…]`
/// while the faces are `0.269` apart — the window pair straight across
/// the notch belongs to neither face
/// (`m10_6_r1_probes_interval::the_notch_bracket_is_the_windows_not_the_faces`).
/// The bound sweep beside this door no longer offers that pair; THIS
/// door still does, because it runs inside an evaluation and its
/// windows may not decide (see [`min_separation`]).
///
/// # Budget-honest, and no width rule anywhere
///
/// Refinement narrows the bracket and can never falsify it: `lo` only
/// rises as cells shrink, `window_hi` only falls, and every
/// intermediate state is a true bracket on `m`. So a query that runs
/// out of budget REPORTS — there is no width threshold below which
/// this door starts answering and above which it refuses, no ε, and no
/// decision: nothing here classifies a margin, which is why this door
/// funnels no predicate.
///
/// # What closes the gap
///
/// An upper end read only off cells [`crate::cell_region`] decides
/// [`CellRegion::Inside`] and stations the face's point door places
/// on the face, which bound the FACES' separation from above; the
/// lower end already bounds it from below. That needs this door to
/// decide cells, and it decides none. The asymmetry is typed rather than
/// papered over.
///
/// # How WIDE the bracket actually is
///
/// "Budget-honest" says the bracket never lies. It does not say the
/// bracket is tight, and on curved carriers it is not. Measured, two
/// cylinders whose axes are 1 m apart (editor-core's
/// `r2_m10_6_probes_interval::min_separation_brackets_a_curved_pair_at_every_budget`):
///
/// ```text
///   max_cell_pairs =   1   [1, 5.385]   width 4.385
///   max_cell_pairs =   4   [1, 5.385]   width 4.385
///   max_cell_pairs =  32   [1, 2.062]   width 1.062
///   max_cell_pairs = 512   [1, 1.586]   width 0.586   <- the default
/// ```
///
/// So at the shipped budget the upper end of a curved pair's bracket
/// is still ~59% above the truth. A planar neck does far better (~7%),
/// because a plane's window IS its cell. A consumer who needs a tight
/// upper bound on a curved pair must raise
/// [`MinSeparationConfig::max_cell_pairs`] and pay for it; a consumer
/// who needs a sound LOWER bound — which is what gates — has one at
/// every budget, including the first.
///
/// # `window_hi` is a CELL-BOX distance, not an attained point pair
///
/// Stated because "the least distance of any station pair found"
/// reads as though a witness exists. It does not: `hi` is the least
/// over the examined pairs of that pair's `separation_hi` and its two
/// centre CELLS' furthest separation, each an interval-enclosed
/// quantity. Every one of them bounds `m` from above at every `p`, so
/// the fold is sound — but none of them is a pair of points anyone
/// evaluated, and the number should not be reported as one.
///
/// # Cost, and where it is paid
///
/// One subdivision search of up to [`MinSeparationConfig::max_cell_pairs`]
/// cell pairs per call. A caller that asks once per leaf of a drive pays
/// it once per leaf, and nothing here caps that.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinSeparation {
    lo: f64,
    hi: f64,
    receipt: CellReceipt,
    excluded: usize,
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
    /// **The lower bound, certified for the FACES** — the end a gate
    /// reads. See the type docs for why this end and not the other.
    pub fn lo(&self) -> f64 {
        self.lo
    }

    /// **The upper bound, certified only over the carrier WINDOWS.**
    /// Not a bound on the trimmed faces' separation in either
    /// direction; the name is the warning. See the type docs.
    pub fn window_hi(&self) -> f64 {
        self.hi
    }

    /// The bracket as the interval it is: a containment-true enclosure
    /// of the WINDOW separation. What an assertion may conclude from it
    /// is its reader's question, not this door's.
    pub fn enclosure(&self) -> Interval {
        Interval::from_bounds(self.lo, self.hi)
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

    /// **Where the two scopes meet**, when they do: the bracket is then
    /// `[0, 0]`, and this is its witness.
    pub fn meets(&self) -> Option<Meets> {
        self.meets
    }

    /// **The human form**: the bracket as numbers, what it cost, and
    /// what the two ends mean — stated at the report rather than left
    /// to the type's documentation, because the asymmetry (`lo` sound
    /// for the faces, `hi` for the windows) is the thing a consumer
    /// must not have to look up.
    pub fn render(&self) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        let _ = writeln!(
            s,
            "minimum separation: FACES ≥ {} (certified over the whole parameter box); \
             WINDOWS ∈ [{}, {}] (width {:e})",
            self.lo,
            self.lo,
            self.hi,
            self.hi - self.lo
        );
        let r = self.receipt;
        let _ = writeln!(
            s,
            "  {} candidate pair(s) subdivided, {} excluded at admission; {} discharged \
             ({} of them vacuously, off the face), {} refused, {} split, {} abandoned",
            r.candidates, self.excluded, r.discharged, r.outside, r.refused, r.splits, r.abandoned
        );
        let _ = writeln!(
            s,
            "  the bracket is honest at any budget: refinement narrows it and cannot \
             falsify it. `lo` bounds the FACES' clearance from below and is what gates; \
             `window_hi` is over the carrier WINDOWS, which contain the faces, so it \
             bounds the faces in NEITHER direction and no assertion arm may read it \
             (see Certified)."
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
            "min_separation lo={:016x} hi={:016x} excluded={}\nreceipt candidates={} \
             discharged={} violated={} refused={} splits={} abandoned={} outside={} \
             holds={}\n",
            self.lo.to_bits(),
            self.hi.to_bits(),
            self.excluded,
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
/// # No tolerance argument, and that is not an omission
///
/// Nothing here decides. The bound comparison the sweep funnels
/// (`d − c` at [`CLEARANCE_MARGIN`]) has no counterpart in a question
/// with no `c` in it: this door computes enclosures and takes minima of
/// their endpoints, so there is no margin to classify, no band to
/// classify it against, and no ε. The pruning comparison
/// (`separation_lo ≥ hi`) is not a decision either — the same
/// distinction the module header draws about the proximity tree's
/// admission test: it decides which pairs are EXAMINED, never what any
/// of them means, and [`MinSeparation`]'s `lo` is floored at `hi` so a
/// pair dropped at the moment it tied cannot narrow the bracket past
/// the truth.
///
/// # Errors
///
/// The engine's own, typed: a carrier with no admitted window
/// ([`EngineRefusal::Unsupported`]), a side naming no face
/// ([`EngineRefusal::EmptyScope`]), and an enclosure that did not
/// evaluate ([`EngineRefusal::PoisonEnclosure`]). Never a width, and
/// never a budget: a budget is what the bracket's width is FOR.
pub fn min_separation<T: Decide + CertifiedBounds>(
    a: &Side<'_, T>,
    b: &Side<'_, T>,
    same_body: bool,
    config: MinSeparationConfig,
) -> Result<MinSeparation, EngineRefusal> {
    if a.faces.is_empty() || b.faces.is_empty() {
        return Err(EngineRefusal::EmptyScope);
    }
    // **No cell is decided here**: this walk holds no band, so it asks
    // neither point-in-face door anything and keeps every cell of the
    // carrier rectangles.
    let wa = a.windows()?.of;
    let wb = b.windows()?.of;
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
                        hi: 0.0,
                        receipt: CellReceipt::default(),
                        excluded: 0,
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
    // Two scopes of one body can carry a pair in both orders.
    let mut seen: BTreeSet<(FaceKey, FaceKey)> = BTreeSet::new();
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
            if !(sep.lo().is_finite() && sep.hi().is_finite()) {
                // Geometry that did not evaluate. Not an indeterminacy
                // refinement could settle and not a budget — the
                // bound sweep's own class, for the same reason.
                return Err(EngineRefusal::PoisonEnclosure {
                    a: x.face,
                    b: y.face,
                });
            }
            roots.push((i, j, sep));
        }
    }
    // The incumbent: the least certified UPPER bound over the root
    // pairs. Every pair's own `separation_hi` bounds the minimum, so
    // the smallest of them is a bound before any subdivision happens —
    // which is what makes the very first bracket true.
    let mut hi = roots
        .iter()
        .fold(f64::INFINITY, |acc, (_, _, s)| acc.min(s.hi()));
    // Admission: a pair whose separation is entirely above the
    // incumbent cannot hold the minimum. It is priced into the floor by
    // its own `separation_lo` and never subdivided.
    let mut floor = f64::INFINITY;
    let mut frontier: Vec<Task> = Vec::new();
    let mut excluded = 0usize;
    for (i, j, sep) in &roots {
        if sep.lo() >= hi {
            floor = floor.min(sep.lo());
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
        });
    }
    let mut receipt = CellReceipt {
        candidates: frontier.len(),
        ..CellReceipt::default()
    };

    while !frontier.is_empty() {
        // The whole-frontier budget check, the driver's own: every task
        // on it becomes at least one leaf, so when they cannot fit they
        // are accounted together rather than started. They are
        // ABANDONED and not refused — the sweep never classified them —
        // and each one's `separation_lo` still floors the bracket,
        // which is the whole of "honest at any budget".
        let final_so_far = receipt.discharged + receipt.refused;
        if final_so_far + frontier.len() > config.max_cell_pairs {
            for task in frontier.drain(..) {
                floor = floor.min(separation(&task.pair.a.enc, &task.pair.b.enc).lo());
                receipt.abandoned += 1;
            }
            break;
        }
        let level = frontier.len();
        let mut next: Vec<Task> = Vec::new();
        for (n, task) in frontier.drain(..).enumerate() {
            let (Some(x), Some(y)) = (wa.get(task.i), wb.get(task.j)) else {
                // Unreachable: the frontier was built from these very
                // indices. Counted rather than skipped, so no path out
                // of this loop can break the receipt identity.
                receipt.abandoned += 1;
                continue;
            };
            let pair = task.pair;
            let sep = separation(&pair.a.enc, &pair.b.enc);
            if !(sep.lo().is_finite() && sep.hi().is_finite()) {
                return Err(EngineRefusal::PoisonEnclosure {
                    a: x.face,
                    b: y.face,
                });
            }
            // Every examined pair tightens the upper bound, INCLUDING
            // the ones dropped below: a cell pair that cannot hold the
            // minimum still witnesses a realizable separation.
            //
            // **The STATION pair is what makes that bound tight**, and
            // it is the same enclosure one level down. `sep.hi()` is
            // the furthest two points of the two CELLS can be, which
            // only approaches the separation as both cells shrink to
            // points — on a metre-scale wall pair that is a very slow
            // number. A pair of single stations, one on each carrier,
            // enclosed over the whole parameter box, bounds the
            // distance between THOSE TWO POINTS at every parameter
            // point in it; the minimum over the windows is at most that
            // distance at every parameter point, so its `hi` is an
            // upper bound on the whole measure and is tight the moment
            // the two stations sit across from each other. Two extra
            // surface evaluations a pair, and the bracket stops being
            // dominated by cell size.
            let station = |w: &Window<'_, T>, c: Cell| {
                let u = geom::mid_param(c.u.0, c.u.1);
                let v = geom::mid_param(c.v.0, c.v.1);
                enclosure(&w.surface, (u, u), (v, v))
            };
            let stations = separation(&station(x, pair.a), &station(y, pair.b));
            if stations.hi().is_finite() {
                hi = hi.min(stations.hi());
            }
            hi = hi.min(sep.hi());
            if sep.lo() >= hi {
                receipt.discharged += 1;
                continue;
            }
            if pair.depth >= config.max_cell_depth {
                receipt.refused += 1;
                floor = floor.min(sep.lo());
                continue;
            }
            // What is already final, what this level has left to fold,
            // what is queued for the next level, and the two halves
            // this split would add — the bound sweep's admission
            // arithmetic, one question over.
            let committed = receipt.discharged + receipt.refused + (level - n - 1) + next.len();
            if committed + 2 > config.max_cell_pairs {
                receipt.refused += 1;
                floor = floor.min(sep.lo());
                continue;
            }
            match split(pair, x, y) {
                Some((lo_half, hi_half)) => {
                    receipt.splits += 1;
                    next.push(Task {
                        k: 0,
                        i: task.i,
                        j: task.j,
                        pair: lo_half,
                    });
                    next.push(Task {
                        k: 0,
                        i: task.i,
                        j: task.j,
                        pair: hi_half,
                    });
                }
                None => {
                    // The `f64` grid itself is the bound: the chosen
                    // axis cannot be bisected. A work bound like the
                    // other two, and priced into the floor the same way.
                    receipt.refused += 1;
                    floor = floor.min(sep.lo());
                }
            }
        }
        frontier = next;
    }

    // **The floor is capped at the incumbent**, and that cap is what
    // makes the branch-and-bound sound rather than merely plausible. A
    // pair dropped at `separation_lo ≥ hi` has a true minimum at least
    // `hi`, so it can only hold the answer when the answer IS `hi` —
    // and then `lo = hi` is the exact bracket rather than one that
    // walked past the truth. When every pair discharges, that is
    // precisely what comes out.
    let lo = floor.min(hi);
    Ok(MinSeparation {
        lo,
        hi,
        receipt,
        excluded,
        meets: None,
    })
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
    surface: Surface<Interval>,
    u: (f64, f64),
    v: (f64, f64),
    /// **The body the face is read from**, which [`cell_region`] reads
    /// the face's region off: a cell the door decides
    /// [`CellRegion::Outside`] holds no point of the FACE, so
    /// classifying it would classify a place the body does not occupy.
    body: &'b Body<T>,
    /// The face's boundary, lifted: what the injectivity question reads
    /// two faces' shared cells and a cell's local boundary off.
    boundary: Boundary,
}

/// **What a window is of its face.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Region {
    /// The window IS the face: an iso-bounded NURBS face's knot domain
    /// ([`iso_bounded_domain`]). Every cell of it lies in the closed
    /// face and every station of it is a point of the face, so a cell
    /// reads [`CellRegion::Inside`] and a station is admitted with no
    /// point-in-face door asked. A cell on the domain's edge holds
    /// boundary points of the face too; what the walk reads off
    /// `Inside` is only that the cell is not outside, which holds.
    Whole,
    /// The window is a superset of the face (every analytic carrier):
    /// [`cell_region`] reads each cell, and [`station_region`] each
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
}

impl RegionMemo {
    fn at(band: Band) -> Self {
        Self {
            band,
            seen: BTreeMap::new(),
            stations: BTreeMap::new(),
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
        *self.seen.entry(key).or_insert_with(|| match w.region {
            Region::Whole => CellRegion::Inside,
            Region::Trimmed => cell_region(w.body, w.face, c.u, c.v, self.band),
        })
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

    /// Whether the point door places `(u, v)` of window `index` on its
    /// face — in the window, for a window that is its face.
    fn placed<T: Decide + CertifiedBounds>(
        &mut self,
        side_b: bool,
        index: usize,
        w: &Window<'_, T>,
        (u, v): (f64, f64),
    ) -> bool {
        match w.region {
            Region::Whole => w.u.0 <= u && u <= w.u.1 && w.v.0 <= v && v <= w.v.1,
            Region::Trimmed => matches!(
                self.station(side_b, index, w, (u, v)),
                Some(FaceContainment::In | FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_))
            ),
        }
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

/// The canonical full turn, one ulp wider at each end so the rounding
/// of `TAU` itself can never shave a sliver of azimuth off a periodic
/// window. An ulp guard is not a tolerance — the same idiom
/// [`bvh::Aabb::padded`] uses, for the same reason.
fn full_turn() -> (f64, f64) {
    (0.0f64.next_down(), TURN_CEILING)
}

/// **The widest azimuth extent a periodic window may carry**: one whole
/// turn, one ulp wider, so the rounding of `TAU` itself can never make
/// a band that really is a turn read as more than one.
const TURN_CEILING: f64 = {
    // `next_up` is not const on this toolchain; the bit pattern above
    // `TAU` is, and it is the same number `f64::next_up` returns for a
    // positive finite value.
    f64::from_bits(core::f64::consts::TAU.to_bits() + 1)
};

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
/// - **Cone**: `v` is the signed slant arc length from the apex, and
///   `(p − apex)·axis = v·cos α`, so `v` is an affine function of
///   position with no critical point on the carrier but the apex: it
///   takes the boundary's range of that projection over `cos α`, widened
///   to the apex's `0`, which a face may hold inside it. A window
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
/// - **NURBS**: a face whose boundary is its chart's own isos, run
///   once around the knot domain — every loft and sweep wall — is the
///   whole patch, and its window is that domain
///   ([`iso_bounded_domain`]). A trimmed face, one with a boundary edge
///   that is not an iso, and one whose isos do not cover the domain's
///   boundary refuse [`EngineRefusal::Unsupported`] naming which.
/// - **A fitted approximation** refuses, named.
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
) -> Result<Window<'b, T>, EngineRefusal> {
    let unsupported = |carrier| Err(EngineRefusal::Unsupported { carrier, face });
    let Some(f) = body.get_face(face) else {
        return unsupported("an unreadable face");
    };
    let Some(stored) = body.get_surface(f.surface) else {
        return unsupported("an unreadable surface");
    };
    let Some((bounds, boundary)) = boundary_of(body, face) else {
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
    match stored {
        Surface::Approx(_) => return unsupported(APPROXIMATED),
        Surface::Nurbs(net) if net.is_placeholder() => {
            return unsupported("the placeholder free-form face");
        }
        _ => {}
    }
    let charted = stored.map_scalar(Interval::from_certified);
    let superset = |u, v| (u, v, Region::Trimmed);
    let (u, v, region) = match &charted {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => superset(along(*origin, *u_ref), along(*origin, normal.cross(*u_ref))),
        Surface::Cylinder { origin, axis, .. } => superset(full_turn(), along(*origin, *axis)),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let slant = (extent - *apex).dot(*axis) / half_angle.cos();
            superset(full_turn(), (slant.lo().min(0.0), slant.hi().max(0.0)))
        }
        Surface::Sphere { .. } => superset(full_turn(), full_latitude()),
        Surface::Torus { .. } => superset(full_turn(), full_turn()),
        Surface::Nurbs(net) => match iso_bounded_domain(body, f, net) {
            Ok((u, v)) => (u, v, Region::Whole),
            Err(carrier) => return unsupported(carrier),
        },
        Surface::Approx(_) => return unsupported(APPROXIMATED),
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
        face,
        region,
        surface: charted,
        u,
        v,
        body,
        boundary,
    })
}

/// A rectangle of carrier parameters, `(u, v)` ranges.
type ParamRect = ((f64, f64), (f64, f64));

/// The refusal of a fitted approximation's face.
const APPROXIMATED: &str = "an approximated face";

/// **The window of a NURBS face bounded by its chart's own isos**: the
/// whole knot domain `[u0, u1] × [v0, v1]`, admitted only where the
/// face's boundary IS that domain's boundary, traversed once.
///
/// The test reads the face's one loop through its lift
/// ([`crate::pcurves::loop_lift`]): each half-edge's stored chart image,
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
/// - each image's end meets the next one's start, as intervals, so the
///   images chain into one closed curve on the domain's boundary;
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
    let rows = crate::pcurves::loop_lift(body, f.outer)
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
    let mut path: Vec<[(Interval, Interval); 2]> = Vec::with_capacity(rows.len());
    for row in &rows {
        if !matches!(
            row.row.pcurve(),
            Pcurve::IsoLine { .. } | Pcurve::IsoArc { .. }
        ) {
            return Err(NOT_ISO);
        }
        let (t0, t1) = row.row.params();
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
    let meet = |p: Interval, q: Interval| {
        p.is_certified() && q.is_certified() && p.lo() <= q.hi() && q.lo() <= p.hi()
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
/// BOTH axes must move a bound, and that is the admission rule rather
/// than the refusal one: a chart that refines on `u` alone gives a
/// subdivision that can only ever narrow one direction, which for a
/// two-dimensional window is not convergence. An analytic carrier's
/// interval evaluation shrinks monotonically under sub-boxes (inclusion
/// isotony), so an axis that does not move here never moves. A spline
/// chart's enclosure ([`Surface::rect_enclosure`]) is a hull of the
/// restricted control net, and a sub-rectangle's restricted net lies in
/// the hull of the rectangle's: the premise holds there too, up to the
/// rounding of the restriction's own interval steps, which can widen a
/// sub-box's hull by ulps.
fn refines(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64)) -> bool {
    let boxed = |u, v| cell_box(&enclosure(surface, u, v));
    let whole = boxed(u, v);
    let half_u = boxed((u.0, geom::mid_param(u.0, u.1)), v);
    let half_v = boxed(u, (v.0, geom::mid_param(v.0, v.1)));
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

/// The face's boundary enclosure, and the boundary itself lifted.
///
/// The enclosure is the hull of every boundary edge's carrier evaluated
/// over its OWN parameter interval — interval arithmetic does the
/// bounding, so a circular edge's bulge is enclosed rather than chorded
/// away — together with the boundary's vertex points, each lifted
/// through [`Interval::from_certified`]. `None` for a face whose
/// boundary this cannot walk or one of whose edges carries no certified
/// curve: an unclaimable extent, never a silently smaller one.
///
/// The lifted boundary carries each edge's carrier and span, its two
/// end vertices, and each of the face's half-edges on it with that
/// half-edge's chart image off the loop's lift
/// ([`crate::pcurves::loop_lift`]): its two end points and, where the
/// image is a coordinate line by variant, which coordinate it holds. A
/// loop with no lift carries no images, which leaves the face
/// unpresented rather than misread.
fn boundary_of<T: Decide + CertifiedBounds>(
    body: &Body<T>,
    face: FaceKey,
) -> Option<(Aabb, Boundary)> {
    let f = body.get_face(face)?;
    let point = |p: &Point3<T>| p.map(Interval::from_certified);
    let mut pts: Vec<Point3<Interval>> = Vec::new();
    let mut out = Boundary {
        sense: f.sense,
        ..Boundary::default()
    };
    let mut halves: BTreeMap<EdgeKey, Vec<HalfEdgeKey>> = BTreeMap::new();
    let mut images: BTreeMap<HalfEdgeKey, ([Point2<Interval>; 2], Option<(Axis, Interval, bool)>)> =
        BTreeMap::new();
    for &lk in core::iter::once(&f.outer).chain(&f.rings) {
        let l = body.get_loop(lk)?;
        match l.boundary {
            LoopBoundary::Empty { vertex } => {
                let p = point(body.get_point(body.get_vertex(vertex)?.point)?);
                out.vertices.insert(vertex, p);
                pts.push(p);
            }
            LoopBoundary::Cycle { first } => {
                for he in body.loop_cycle(first)? {
                    let h = body.get_half_edge(he)?;
                    let p = point(body.get_point(body.get_vertex(h.start)?.point)?);
                    out.vertices.insert(h.start, p);
                    pts.push(p);
                    halves.entry(h.edge).or_default().push(he);
                }
                if let Ok(rows) = crate::pcurves::loop_lift(body, lk) {
                    for row in rows {
                        let (t0, t1) = row.row.params();
                        let lift = |t: T| {
                            let q = row.pcurve.eval(t);
                            Point2::new(Interval::from_certified(q.x), Interval::from_certified(q.y))
                        };
                        images.insert(row.half_edge, ([lift(t0), lift(t1)], iso_of(&row.pcurve)));
                    }
                }
            }
        }
    }
    for (ek, hes) in halves {
        let e = body.get_edge(ek)?;
        let curve = body.get_curve_geom(e.curve)?.certified()?;
        let (t0, t1) = curve.params();
        let span = (Interval::from_certified(t0), Interval::from_certified(t1));
        let carrier = curve.carrier().map_scalar(Interval::from_certified);
        // The whole parameter span as ONE interval: the carrier's
        // interval evaluation over it encloses the entire arc, which is
        // what makes a curved boundary bound its own face's window.
        pts.push(carrier.eval(Interval::from_bounds(span.0.lo(), span.1.hi())));
        let ends = [
            body.get_half_edge(e.he_plus)?.start,
            body.get_half_edge(e.he_minus)?.start,
        ];
        let uses = hes
            .iter()
            .map(|he| {
                let found = images.get(he);
                Use {
                    plus: e.he_plus == *he,
                    image: found.map(|(image, _)| *image),
                    iso: found.and_then(|(_, iso)| *iso).map(|(held, at, angle)| Iso {
                        held,
                        at,
                        full: angle && ends[0] == ends[1],
                    }),
                }
            })
            .collect();
        out.edges.push(Rim {
            key: ek,
            arc: Some((carrier, span)),
            ends,
            uses,
        });
    }
    Aabb::from_points(pts).map(|b| (b, out))
}

/// **A chart image that is a coordinate line, by variant**: which
/// coordinate it holds, the value, and whether its moving coordinate is
/// an angle. The held coordinate is exact structure — a velocity, or a
/// harmonic channel's three coefficients, that lift to the point zero —
/// so no zero is decided here.
fn iso_of<T: CertifiedBounds>(p: &Pcurve<T>) -> Option<(Axis, Interval, bool)> {
    let zero = |x: T| {
        let i = Interval::from_certified(x);
        i.is_certified() && i.lo() == 0.0 && i.hi() == 0.0
    };
    let lift = Interval::from_certified;
    let line = |p0: Point2<T>, d: geom_core::Vec2<T>| {
        if zero(d.x) {
            Some((Axis::U, lift(p0.x), false))
        } else if zero(d.y) {
            Some((Axis::V, lift(p0.y), false))
        } else {
            None
        }
    };
    match p {
        Pcurve::IsoLine { p0, pl } => line(*p0, *pl),
        Pcurve::IsoArc { p0, pd, .. } => line(*p0, *pd),
        Pcurve::Harmonic { u, v } => {
            let constant = |ch: &Channel<T>| match *ch {
                Channel::Angle { phase, winding: 0 } => Some(lift(phase)),
                Channel::Free { c, a, b, l } if zero(a) && zero(b) && zero(l) => Some(lift(c)),
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
    let da = diameter(&cell_box(&pair.a.enc));
    let db = diameter(&cell_box(&pair.b.enc));
    // `total_cmp` so the choice stays total on a poison diameter;
    // strictly-greater keeps the tie on the first cell.
    let split_b = db.total_cmp(&da) == core::cmp::Ordering::Greater;
    let (cell, root) = if split_b { (pair.b, y) } else { (pair.a, x) };
    let (ru, rv) = cell.relative(root);
    let axis_v = rv.total_cmp(&ru) == core::cmp::Ordering::Greater;
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

/// **A cell paired with itself, split**: the cell halved by the split
/// rule's axis choice, and the three pairs of its halves — each against
/// itself and the two against each other. `None` where the axis cannot
/// be bisected.
fn split_self<T: Real>(pair: CellPair, w: &Window<'_, T>) -> Option<[CellPair; 3]> {
    let cell = pair.a;
    let (ru, rv) = cell.relative(w);
    let axis_v = rv.total_cmp(&ru) == core::cmp::Ordering::Greater;
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
