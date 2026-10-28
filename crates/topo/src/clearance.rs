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
//! the lift keeps every bound. The cells' and stations' point-in-face
//! decisions run at `T` ([`crate::cell_region`]).
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
//! carrier rectangles, because a drop is a decision and that door may
//! take none (its own header, and
//! `work/clear/min-separation-tightening-crosses-the-drive.md`). So
//! `window_hi` is a minimum over strictly more of the carrier than a
//! `Holds` covers, and the asymmetry is in the direction its own docs
//! already state.
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
//! still contains the face.
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
use geom::Surface;
use geom_core::interval::Interval;
use geom_core::k_stats::decide;
use geom_core::{Band, Bounds, CertifiedBounds, Decide, Margin, Point3, Real, Sign, Vec3};

use crate::Body;
use crate::FaceContainment;
use crate::cell_region::{CellRegion, cell_region, station_region};
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, VertexKey};

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
/// A recorded run dial, and deliberately far below a clearance query's
/// pair budget, because the two doors are asked at different rates: a
/// clearance query is asked once per certified leaf by a fold a
/// consumer calls deliberately, while this one is asked INSIDE the
/// evaluation of a measure node — so a drive of a few hundred leaves
/// pays it a few hundred times. The bracket a budget buys is honest at
/// any width ([`MinSeparation`]), so the dial trades tightness for time
/// and never truth.
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
    /// each cell of which the sweep decides at `band`
    /// ([`crate::cell_region`]).
    ///
    /// # Errors
    ///
    /// [`EngineRefusal::Unsupported`] for the first face whose carrier
    /// has no admitted window.
    pub fn windows(&self, band: Band) -> Result<Windows<'b, T>, EngineRefusal> {
        read(self, Some(band))
    }
}

/// One side's faces read into carrier windows: what [`Sweep::run`]
/// subdivides.
#[derive(Debug, Clone)]
pub struct Windows<'b, T: Real> {
    of: Vec<Window<'b, T>>,
}

fn read<'b, T: Decide + CertifiedBounds>(
    side: &Side<'b, T>,
    band: Option<Band>,
) -> Result<Windows<'b, T>, EngineRefusal> {
    let of = side
        .faces
        .iter()
        .map(|&k| window_of(side.body, k, band))
        .collect::<Result<_, _>>()?;
    Ok(Windows { of })
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
    /// [`min_separation`] was asked about two sides with no admitted
    /// pair between them: every candidate was dropped by the wedge rule
    /// (a face against itself, or two faces sharing a vertex), so there
    /// is no separation to enclose.
    ///
    /// Its own class rather than [`Self::EmptyScope`], which is about a
    /// scope naming no FACE: here both scopes named faces and the
    /// PAIRING is empty. The minimum over an empty set is `+∞`, and
    /// reporting that as a clearance would be a number no geometry
    /// backs.
    NoAdmittedPair,
    /// A cell pair the interval pass classified as a definite violation
    /// whose witness the caller's verifier did not confirm.
    ///
    /// **Not reachable on any geometry built so far**, and the reason is
    /// structural: a verifier evaluates points INSIDE the cell pair the
    /// interval pass proved violating, so the distance it recomputes
    /// lies inside an enclosure already classified definite. It is kept
    /// because "the interval pass and the verifier disagree" is exactly
    /// the thing a witness claim must not paper over, and a
    /// reachable-looking arm nobody can fire is better than an
    /// `unreachable!` that is wrong one day.
    ///
    /// Its own arm, and never a violation: a witness is a claim about a
    /// concrete configuration, and one the independent recomputation
    /// contradicts is evidence about the analysis, not about the
    /// geometry. The prose carries what the verifier found.
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
            Self::EmptyScope | Self::NoAdmittedPair => String::new(),
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
            Self::NoAdmittedPair => "no_admitted_pair",
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
    /// MINUS what the wedge rule dropped (a shared vertex, a face
    /// against itself) and minus the second copy of any pair two scopes
    /// of one body named twice. It is the number of subdivision roots,
    /// not the number of pairs the tree returned.
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
            self.outside <= self.discharged,
            "a receipt's `outside` is a SUB-COUNT of `discharged` and this one has \
             outside = {} against discharged = {}: a cell pair discharged off the face \
             that was not counted a discharge is a bug in the sweep, not a verdict",
            self.outside,
            self.discharged
        );
    }

    /// Folds another query's receipt in. The identity is additive: a
    /// sum of forests is a forest.
    ///
    /// # Panics
    ///
    /// When either receipt has `outside > discharged`.
    pub fn add(&mut self, other: Self) {
        self.check_outside_is_a_sub_count();
        other.check_outside_is_a_sub_count();
        self.candidates += other.candidates;
        self.discharged += other.discharged;
        self.violated += other.violated;
        self.refused += other.refused;
        self.splits += other.splits;
        self.outside += other.outside;
        self.abandoned += other.abandoned;
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

    /// Folds another query's widths in.
    pub fn fold(&mut self, other: Self) {
        if let Some(w) = other.widest {
            self.widest = Some(self.widest.map_or(w, |x: f64| x.max(w)));
        }
        if let Some(w) = other.narrowest {
            self.narrowest = Some(self.narrowest.map_or(w, |x: f64| x.min(w)));
        }
        self.deepest = self.deepest.max(other.deepest);
    }
}

// --------------------------------------------------- the bound walk

/// **The bound question's walk**: the question, the band it is decided
/// at, and the two budgets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sweep {
    /// Which question ([`ClearanceBound`]).
    pub bound: ClearanceBound,
    /// The linear band every compare classifies against. The windows
    /// the walk runs over decide their cells at the same band
    /// ([`Side::windows`]).
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

/// **A cell pair the interval pass found violating**, as the sweep hands
/// it to the verifier.
///
/// The verifier answers whether a concrete pair of points of the two
/// faces violates the bound, by a computation independent of the
/// interval pass; the sweep stops at the first one it confirms.
pub struct Candidate<'s, 'b, T: Real> {
    x: &'s Window<'b, T>,
    ca: Cell,
    ia: usize,
    y: &'s Window<'b, T>,
    cb: Cell,
    ib: usize,
    region: &'s mut RegionMemo,
}

impl<T: Decide + CertifiedBounds> Candidate<'_, '_, T> {
    /// The two faces, the first side's then the second's.
    pub fn faces(&self) -> (FaceKey, FaceKey) {
        (self.x.face, self.y.face)
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
    pub fn on_face_stations(&mut self) -> (Stations, Stations) {
        let a = admitted(self.region, false, self.ia, self.x, self.ca);
        let b = admitted(self.region, true, self.ib, self.y, self.cb);
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
    let stations = |span: (f64, f64)| [span.0, mid_of(span), span.1];
    let mut out = Vec::with_capacity(9);
    for u in stations(c.u) {
        for v in stations(c.v) {
            if matches!(
                region.station(side_b, index, w, (u, v)),
                Some(
                    FaceContainment::In | FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_)
                )
            ) {
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
    /// one frame: the wedge rule then drops a face against itself and
    /// two faces sharing a vertex, and a pair two scopes name in both
    /// orders is taken once. Faces of different bodies are never
    /// adjacent in the wedge sense, so the cross product stands.
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
    /// classifies violating, and about an indeterminate pair at the root
    /// and at the depth floor; its first `Ok` is the verdict. An `Err`
    /// on a pair classified violating is
    /// [`EngineRefusal::WitnessUnverified`] carrying its prose.
    pub fn run<T, W, F>(
        &self,
        wa: &Windows<'_, T>,
        wb: &Windows<'_, T>,
        same_body: bool,
        mut verify: F,
    ) -> BoundReport<W>
    where
        T: Decide + CertifiedBounds,
        F: FnMut(&mut Candidate<'_, '_, T>) -> Result<W, String>,
    {
        let (wa, wb) = (&wa.of[..], &wb.of[..]);
        let c = self.bound.c();
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
        let cloud = |w: &Window<'_, T>| [enclosure(&w.surface, w.u, w.v)];
        let tree_a = Bvh::build_bounded(wa.iter().map(cloud), 0.0);
        let tree_b = Bvh::build_bounded(wb.iter().map(cloud), 0.0);
        let mut seen: BTreeSet<(FaceKey, FaceKey)> = BTreeSet::new();
        let candidates: Vec<(usize, usize)> = tree_a
            .pairs_within(&tree_b, reach)
            .into_iter()
            .filter(|&(i, j)| {
                let (Some(x), Some(y)) = (wa.get(i), wb.get(j)) else {
                    return false;
                };
                if !same_body {
                    return true;
                }
                // The wedge rule: a face is never at a distance from
                // itself, and two faces sharing a vertex meet, so their
                // separation is legitimately zero and belongs to the
                // wedge predicates at that vertex. The unordered
                // de-duplication is the same-body case's own: two scopes
                // of one body can carry a pair in both orders.
                //
                // **The exclusion is GLOBAL where the justification is
                // LOCAL**, and that is a gap rather than a
                // simplification: two faces that share one vertex and
                // ALSO approach each other far from it are examined by
                // neither instrument — the wedge predicates look at the
                // vertex, and this engine has dropped the pair. A cone's
                // apex, where every lateral face shares one vertex,
                // removes the whole lateral check. Stated here rather
                // than papered over with a distance test that would
                // re-report every legitimate meeting.
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
        let mut region = RegionMemo::default();
        let mut deepest = 0;
        let mut first_refusal: Option<EngineRefusal> = None;
        let mut violation: Option<W> = None;

        let mut frontier: Vec<Task> = candidates
            .iter()
            .filter_map(|&(i, j)| {
                let (x, y) = (wa.get(i)?, wb.get(j)?);
                Some(Task {
                    i,
                    j,
                    pair: CellPair {
                        a: Cell::of(x),
                        b: Cell::of(y),
                        depth: 0,
                    },
                })
            })
            .collect();
        // A candidate whose windows are not addressable would leave the
        // receipt short by one; the filter above cannot drop one without
        // saying so, so the shortfall is counted rather than lost.
        receipt.abandoned = candidates.len() - frontier.len();

        'sweep: while !frontier.is_empty() {
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
                let (Some(x), Some(y)) = (wa.get(task.i), wb.get(task.j)) else {
                    // Unreachable: the frontier was built from these
                    // very indices. Counted rather than skipped, so no
                    // path out of this loop can break the receipt.
                    receipt.abandoned += 1;
                    continue;
                };
                let pair = task.pair;
                deepest = deepest.max(pair.depth);
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
                let pa = enclosure(&x.surface, pair.a.u, pair.a.v);
                let pb = enclosure(&y.surface, pair.b.u, pair.b.v);
                let margin = separation(&pa, &pb) - Interval::from_f64(c);
                let discharge = |receipt: &mut CellReceipt, widths: &mut DischargeWidths| {
                    receipt.discharged += 1;
                    let da = diameter(&cell_box(&x.surface, pair.a.u, pair.a.v));
                    let db = diameter(&cell_box(&y.surface, pair.b.u, pair.b.v));
                    widths.discharged_at(da.max(db));
                };
                let mut candidate = |region: &mut RegionMemo| {
                    verify(&mut Candidate {
                        x,
                        ca: pair.a,
                        ia: task.i,
                        y,
                        cb: pair.b,
                        ib: task.j,
                        region,
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
                        // places a probe is worth its cost — the root,
                        // where a contact or an overlap shows up
                        // immediately, and the depth floor, where the
                        // alternative is a refusal.
                        //
                        // It only ever ADDS violations, each confirmed
                        // by the verifier: a probe that finds nothing
                        // changes no answer, so no `Holds` can rest on
                        // it. That is what keeps it an exhibit rather
                        // than a sample.
                        let probe_here = pair.depth == 0 || pair.depth >= self.max_cell_depth;
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
                        // fits. What is already final, what this level
                        // has left to fold, what is queued for the next
                        // level, and the two halves this split adds.
                        let committed = receipt.discharged
                            + receipt.violated
                            + receipt.refused
                            + (level - n - 1)
                            + next.len();
                        if committed + 2 > self.max_cell_pairs {
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
                                    i: task.i,
                                    j: task.j,
                                    pair: lo,
                                });
                                next.push(Task {
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
/// decide cells, which it may not while its decisions cross the drive
/// on one lane only (`work/clear/min-separation-tightening-crosses-the-drive.md`).
/// Until then the asymmetry is typed rather than papered over.
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
/// The door runs once per call, and a measure that calls it inside
/// evaluation pays it once per leaf of a drive: at a 65,536-leaf
/// budget that is up to 65,536 subdivision searches of up to
/// [`DEFAULT_MAX_MIN_SEP_PAIRS`] cell pairs each. Nothing here caps it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinSeparation {
    lo: f64,
    hi: f64,
    receipt: CellReceipt,
    excluded: usize,
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
        )
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
/// `same_body` is [`Sweep::run`]'s: the two sides are faces of one body,
/// and the wedge rule applies.
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
/// ([`EngineRefusal::EmptyScope`]), a pairing the wedge rule empties
/// ([`EngineRefusal::NoAdmittedPair`]), and an enclosure that did not
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
    // **No cell is decided here.** A measure calls this door from inside
    // an evaluation, and every funnel row a cell verdict records lands
    // in that leaf's census, while the `f64` witness build asks none —
    // so the two builds would differ `0 -> N` on every box and the leaf
    // would refuse `flip_crossing`. Details in
    // `work/clear/min-separation-tightening-crosses-the-drive.md`.
    let wa = read(a, None)?.of;
    let wb = read(b, None)?.of;
    // The SAME-BODY rule the bound sweep uses, and for the same
    // reason: a face is never at a distance from itself, and two faces
    // sharing a vertex meet, so their separation is legitimately zero
    // and belongs to the wedge predicates at that vertex — reporting it
    // as the minimum clearance would report the body's own
    // construction as its tightest gap.
    let mut seen: BTreeSet<(FaceKey, FaceKey)> = BTreeSet::new();
    let root_a: Vec<Point3<Interval>> =
        wa.iter().map(|w| enclosure(&w.surface, w.u, w.v)).collect();
    let root_b: Vec<Point3<Interval>> =
        wb.iter().map(|w| enclosure(&w.surface, w.u, w.v)).collect();

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
            if same_body
                && !(x.face != y.face
                    && x.vertices.is_disjoint(&y.vertices)
                    && seen.insert((x.face.min(y.face), x.face.max(y.face))))
            {
                continue;
            }
            let (Some(pa), Some(pb)) = (root_a.get(i), root_b.get(j)) else {
                continue;
            };
            let sep = separation(pa, pb);
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
    if roots.is_empty() {
        return Err(EngineRefusal::NoAdmittedPair);
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
        let (Some(x), Some(y)) = (wa.get(*i), wb.get(*j)) else {
            continue;
        };
        frontier.push(Task {
            i: *i,
            j: *j,
            pair: CellPair {
                a: Cell::of(x),
                b: Cell::of(y),
                depth: 0,
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
                let (Some(x), Some(y)) = (wa.get(task.i), wb.get(task.j)) else {
                    receipt.abandoned += 1;
                    continue;
                };
                let pa = enclosure(&x.surface, task.pair.a.u, task.pair.a.v);
                let pb = enclosure(&y.surface, task.pair.b.u, task.pair.b.v);
                floor = floor.min(separation(&pa, &pb).lo());
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
            let pa = enclosure(&x.surface, pair.a.u, pair.a.v);
            let pb = enclosure(&y.surface, pair.b.u, pair.b.v);
            let sep = separation(&pa, &pb);
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
                let u = mid_of(c.u);
                let v = mid_of(c.v);
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
                        i: task.i,
                        j: task.j,
                        pair: lo_half,
                    });
                    next.push(Task {
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
    })
}

// ------------------------------------------------- carrier windows

/// A face's domain window: the carrier it sits on, lifted to the
/// interval scalar, the rectangle of carrier parameters the engine
/// subdivides, and the vertices that decide adjacency.
#[derive(Debug, Clone)]
struct Window<'b, T: Real> {
    face: FaceKey,
    surface: Surface<Interval>,
    u: (f64, f64),
    v: (f64, f64),
    /// **The body the face is read from, and the band its cells are
    /// decided at** ([`cell_region`]): a cell the door decides
    /// [`CellRegion::Outside`] holds no point of the FACE, so
    /// classifying it would classify a place the body does not occupy.
    /// `None` where the door passed no band and so asks nothing of any
    /// cell ([`min_separation`]).
    region: Option<(&'b Body<T>, Band)>,
    /// The face's boundary vertices — the wedge rule's currency, read
    /// off topology and never off geometry.
    vertices: BTreeSet<VertexKey>,
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
#[derive(Default)]
struct RegionMemo {
    seen: BTreeMap<(bool, usize, u64, u64, u64, u64), CellRegion>,
    /// The witness stations the point doors were asked to place, by the
    /// same bitwise key, and where they placed them.
    stations: BTreeMap<(bool, usize, u64, u64), Option<FaceContainment>>,
}

impl RegionMemo {
    fn ask<T: Decide + CertifiedBounds>(
        &mut self,
        side_b: bool,
        index: usize,
        w: &Window<'_, T>,
        c: Cell,
    ) -> CellRegion {
        let Some((body, band)) = w.region else {
            return CellRegion::Undecided;
        };
        let key = (
            side_b,
            index,
            c.u.0.to_bits(),
            c.u.1.to_bits(),
            c.v.0.to_bits(),
            c.v.1.to_bits(),
        );
        *self
            .seen
            .entry(key)
            .or_insert_with(|| cell_region(body, w.face, c.u, c.v, band))
    }

    /// Where the face's point door places the station `(u, v)` of
    /// window `index` ([`station_region`]); `None` for a window built
    /// with no band.
    fn station<T: Decide + CertifiedBounds>(
        &mut self,
        side_b: bool,
        index: usize,
        w: &Window<'_, T>,
        (u, v): (f64, f64),
    ) -> Option<FaceContainment> {
        let (body, band) = w.region?;
        *self
            .stations
            .entry((side_b, index, u.to_bits(), v.to_bits()))
            .or_insert_with(|| station_region(body, w.face, (u, v), band))
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
///   placeholder — refuses [`EngineRefusal::Unsupported`] naming the
///   class. Not because the evaluator would not run, but because a
///   free-form patch's knot domain is not a certified superset of a
///   TRIMMED region without the boundary in chart coordinates, and
///   inventing one is exactly the downgrade E7 forbids.
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
    band: Option<Band>,
) -> Result<Window<'b, T>, EngineRefusal> {
    let unsupported = |carrier| Err(EngineRefusal::Unsupported { carrier, face });
    let Some(f) = body.get_face(face) else {
        return unsupported("an unreadable face");
    };
    let Some(stored) = body.get_surface(f.surface) else {
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
    let charted = match stored {
        Surface::Nurbs(_) => return unsupported("a free-form face"),
        Surface::Approx(_) => return unsupported("an approximated face"),
        other => other.map_scalar(Interval::from_certified),
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
        face,
        surface: charted,
        u,
        v,
        region: band.map(|band| (body, band)),
        vertices,
    })
}

/// Whether halving the window on either axis actually narrows the
/// carrier's enclosure — the door that turns a chart the subdivision
/// could not refine into a typed refusal rather than a budget burn.
///
/// BOTH axes must move a bound, and that is the admission rule rather
/// than the refusal one: a chart that refines on `u` alone gives a
/// subdivision that can only ever narrow one direction, which for a
/// two-dimensional window is not convergence. Interval enclosures
/// shrink monotonically under sub-boxes, so an axis that does not move
/// here never moves.
fn refines(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64)) -> bool {
    let whole = cell_box(surface, u, v);
    let half_u = cell_box(surface, (u.0, mid_of(u)), v);
    let half_v = cell_box(surface, u, (v.0, mid_of(v)));
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
/// away — together with the boundary's vertex points, each lifted
/// through [`Interval::from_certified`]. `None` for a face whose
/// boundary this cannot walk or one of whose edges carries no certified
/// curve: an unclaimable extent, never a silently smaller one.
fn boundary_of<T: Decide + CertifiedBounds>(
    body: &Body<T>,
    face: FaceKey,
) -> Option<(Aabb, BTreeSet<VertexKey>)> {
    let f = body.get_face(face)?;
    let point = |p: &Point3<T>| p.map(Interval::from_certified);
    let mut pts: Vec<Point3<Interval>> = Vec::new();
    let mut vertices = BTreeSet::new();
    let mut edges: BTreeSet<EdgeKey> = BTreeSet::new();
    for &lk in core::iter::once(&f.outer).chain(&f.rings) {
        let l = body.get_loop(lk)?;
        match l.boundary {
            LoopBoundary::Empty { vertex } => {
                vertices.insert(vertex);
                pts.push(point(body.get_point(body.get_vertex(vertex)?.point)?));
            }
            LoopBoundary::Cycle { first } => {
                for he in body.loop_cycle(first)? {
                    let h = body.get_half_edge(he)?;
                    vertices.insert(h.start);
                    pts.push(point(body.get_point(body.get_vertex(h.start)?.point)?));
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
        let span = Interval::from_bounds(
            Interval::from_certified(t0).lo(),
            Interval::from_certified(t1).hi(),
        );
        pts.push(
            curve
                .carrier()
                .map_scalar(Interval::from_certified)
                .eval(span),
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
    fn of<T: Real>(w: &Window<'_, T>) -> Self {
        Self { u: w.u, v: w.v }
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
}

/// One entry of a walk's frontier: which candidate pair it belongs to,
/// and the cell pair itself.
#[derive(Debug, Clone, Copy)]
struct Task {
    i: usize,
    j: usize,
    pair: CellPair,
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
    let da = diameter(&cell_box(&x.surface, pair.a.u, pair.a.v));
    let db = diameter(&cell_box(&y.surface, pair.b.u, pair.b.v));
    // `total_cmp` so the choice stays total on a poison diameter;
    // strictly-greater keeps the tie on the first cell.
    let split_b = db.total_cmp(&da) == core::cmp::Ordering::Greater;
    let (cell, root) = if split_b { (pair.b, y) } else { (pair.a, x) };
    let (ru, rv) = cell.relative(root);
    let axis_v = rv.total_cmp(&ru) == core::cmp::Ordering::Greater;
    let (lo, hi) = if axis_v { cell.v } else { cell.u };
    let mid = mid_of((lo, hi));
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

/// A span's midpoint, `0.5 * (lo + hi)`: where the split rule cuts and
/// where a witness lattice and a min-separation station stand, one
/// spelling so a station and a split cannot drift apart.
fn mid_of(span: (f64, f64)) -> f64 {
    0.5 * (span.0 + span.1)
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
