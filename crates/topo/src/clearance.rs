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
//! One walk, and the difference is what it does with a cell pair (the
//! cut differs too: the injective arm bisects by metric axes and splits
//! a cell against itself in three): the bound question DECIDES each
//! pair at a funnel site; the minimum question takes minima of
//! enclosure endpoints. Both drop the cells decided outside their faces.
//! The minimum question's region readings choose only which pairs feed
//! its bracket, so they are taken in [`k_stats::unlogged`]'s scope; that
//! door's own header carries the argument.
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
//! # No ε here, and the funnelled compares
//!
//! Every decision goes through the one `k_stats` funnel at a named
//! predicate with a [`Margin`] door. The distance test decides at two:
//! [`CLEARANCE_MARGIN`] (`d − c`, metres minus metres) and
//! [`SELF_INTERSECTION_GAP`] (`d`, the separation a cell pair that does
//! not touch in M must classify strictly positive). Under the
//! injectivity question a TOUCHING pair is decided instead by a local
//! certificate ([`crate::injective`]), at that module's own sites. The
//! cell and station verdicts the sweep asks for decide on
//! [`crate::cell_region`]'s rows, inside those doors. All carry a row
//! in `docs/predicate-dimension-audit.md`. No DECISION here compares a
//! float raw, mints a tolerance, or reads a width as evidence.
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
use std::ops::ControlFlow;

use bvh::{Aabb, Bvh};
use geom::{NurbsSurface, Surface};
use geom_brep::{Channel, Pcurve};
use geom_core::interval::Interval;
use geom_core::k_stats::{self, decide};
use geom_core::{Band, Bounds, CertifiedBounds, Decide, Margin, Point2, Point3, Real, Sign, Vec3};

use crate::Body;
use crate::FaceContainment;
use crate::cell_region::{
    Ball, BoundaryStop, CellRegion, cell_region_lifted, centre, lift_boundary, station_region,
};
use crate::entity::{EdgeKey, Face, FaceKey, HalfEdgeKey, LoopBoundary, VertexKey};
use crate::injective::{
    self, Axis, Boundary, Iso, Local, Near, Outcome, Piece, Rim, Shared, Use, full_turn,
};

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
/// (`work/cleartail/the-second-order-arm-certifies-a-declared-wedge-0-edge.md`)
/// retires the dial. A recorded run dial.
pub const CUSP_DEPTH: u32 = 10;

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
    /// in the order of [`Self::faces`]. `which` is the side this one is
    /// of its question, which a refusal names the face by.
    ///
    /// # Errors
    ///
    /// [`EngineRefusal::Unsupported`] for the first face whose carrier
    /// has no admitted window.
    pub fn windows(&self, which: Which) -> Result<Windows<'b, T>, EngineRefusal> {
        let of = self
            .faces
            .iter()
            .map(|&k| window_of(self.body, k, which))
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
    },
    /// A budget was exhausted, typed and named.
    Budget(CellBudget),
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
    /// A face on a torus not decided a ring (`R > r`): the carrier meets
    /// itself, so the face is not injective by its kind.
    TorusNotRing {
        /// The face.
        face: FaceKey,
        /// The side it is on.
        side: Which,
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
            Self::Unsupported {
                carrier,
                face,
                side,
            } => format!("{carrier} {}={face:?}", side.name()),
            Self::WitnessUnverified { what } => what.clone(),
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
            Self::TorusNotRing { face, side } => format!("{}={face:?}", side.name()),
            Self::EmptyScope { side } => side.name().to_owned(),
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
            Self::EmptyScope { .. } => "empty_scope",
            Self::Uncertified { .. } => "uncertified",
            Self::CollapsedChart { .. } => "collapsed_chart",
            Self::TorusNotRing { .. } => "torus_not_ring",
        }
    }

    /// **The refusal in words**, each face and scope named with its
    /// side as `sides` names it: the first side's, then the second's.
    pub fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        sides: [&dyn core::fmt::Display; 2],
    ) -> core::fmt::Result {
        let [first, second] = sides;
        match self {
            Self::Sliver { predicate } => write!(
                f,
                "the separation sits wholly inside {predicate}'s ambiguity band, so no \
                 refinement can decide it"
            ),
            Self::Budget(CellBudget::Depth { max_cell_depth }) => write!(
                f,
                "a cell pair was split {max_cell_depth} times, the depth budget, and is still \
                 undecided"
            ),
            Self::Budget(CellBudget::Pairs { max_cell_pairs }) => write!(
                f,
                "the query examined {max_cell_pairs} cell pairs, its budget, with pairs still \
                 undecided"
            ),
            Self::Budget(CellBudget::Resolution) => f.write_str(
                "a cell pair could not be bisected: its midpoint lands on an endpoint, so the \
                 f64 grid itself is the bound",
            ),
            Self::Unsupported {
                carrier,
                face,
                side,
            } => write!(
                f,
                "the clearance engine cannot answer for face {face:?} of {}: it is {carrier}",
                side.of(sides)
            ),
            Self::WitnessUnverified { what } => write!(
                f,
                "a violation the interval pass found was not confirmed by the verifier: {what}"
            ),
            Self::PoisonEnclosure { a, b } => write!(
                f,
                "the enclosure between face {a:?} of {first} and face {b:?} of {second} did \
                 not evaluate, which is geometry, not a budget"
            ),
            Self::EmptyScope { side } => write!(
                f,
                "the scope of {} names no face, so there is nothing to measure",
                side.of(sides)
            ),
            Self::Uncertified { a, b, shared, arm } => write!(
                f,
                "face {a:?} of {first} and face {b:?} of {second} touch through {}, and the \
                 {} certificate did not discharge them",
                shared.payload(),
                arm.name()
            ),
            Self::CollapsedChart { face, side, vertex } => write!(
                f,
                "the chart of face {face:?} of {} collapses an edge to its vertex {vertex:?} \
                 near a touching pair, and the monotone certificate reads no collapsed chart",
                side.of(sides)
            ),
            Self::TorusNotRing { face, side } => write!(
                f,
                "face {face:?} of {} lies on a torus not decided a ring (R > r), so its \
                 carrier meets itself",
                side.of(sides)
            ),
        }
    }
}

/// The refusal where no caller names the sides: the first side and
/// the second.
impl core::fmt::Display for EngineRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.say(f, [&"the first side", &"the second side"])
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
        .then_some(source.predicate.unwrap_or("<unnamed>"))
}

/// The counting receipt of one clearance query.
///
/// Each candidate pair is the root of its own subdivision: a cell pair
/// is discharged, violated, refused, ABANDONED unexamined, or split —
/// into two, or into three where the injective walk splits a cell
/// against itself — and `splits` counts each split's children less
/// one. A forest of `candidates` trees counted so has exactly
/// `splits + candidates` leaves, and every leaf is in one of the four
/// buckets. It rides the shipped report, so a consumer re-checks it
/// without trusting this module ([`CellReceipt::holds`]).
///
/// **`abandoned` is what the walk did not examine, spelled rather than
/// hidden.** Calling it `refused` would claim it was tried. The bound
/// sweep abandons the frontier it held when it stops at a verified
/// violation, so a bound query that runs to completion has
/// `abandoned == 0`. [`min_separation`] abandons a level that does not
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
    /// sweep verified a violation and stopped, and under
    /// [`min_separation`] a level that did not fit the pair budget, each
    /// pair priced into the bracket's `lo` unexamined.
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
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl<T: Real> Candidate<'_, '_, T> {
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
        if self.strict
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
        verify: F,
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
        let mut q = Bound {
            sweep: self,
            injective,
            shared: &shared,
            region: RegionMemo::at(self.band),
            locals: LocalMemo::default(),
            across: BTreeMap::new(),
            cornered: 0,
            widths: DischargeWidths::empty(),
            first_refusal: None,
            violation: None,
            verify,
        };

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
                    sh.cell(&x.boundary).unwrap_or(SharedCell::Face(x.face))
                };
                match decide(
                    self.bound.predicate(),
                    Margin::of(Interval::from_bounds(-c, -c)),
                    self.band,
                ) {
                    Ok(Sign::Positive) => receipt.discharged += 1,
                    Ok(Sign::Zero) if self.bound.zero_discharges() => receipt.discharged += 1,
                    // An exact margin in the band: no refinement moves a
                    // distance topology fixed at 0.
                    Err(source) => {
                        receipt.refused += 1;
                        q.first_refusal.get_or_insert(EngineRefusal::Sliver {
                            predicate: source.predicate.unwrap_or(self.bound.predicate()),
                        });
                    }
                    Ok(Sign::Negative | Sign::Zero) => {
                        receipt.violated += 1;
                        let stations = meeting_stations(x, y, sh, cell, self.band);
                        match (q.verify)(Candidate {
                            x,
                            ca: a,
                            ia: i,
                            y,
                            cb: b,
                            ib: j,
                            region: &mut q.region,
                            meeting: Some((cell, stations.0, stations.1)),
                            strict: false,
                        }) {
                            Ok(w) => {
                                q.violation = Some(w);
                                receipt.abandoned += (candidates.len() - k - 1) + frontier.len();
                                frontier.clear();
                                break;
                            }
                            Err(what) => {
                                q.first_refusal
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
                        q.first_refusal.get_or_insert(r);
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

        let (receipt, deepest) = walk(self.budgets, receipt, frontier, wa, wb, &mut q);
        let Bound {
            region,
            across,
            cornered,
            mut widths,
            first_refusal,
            violation,
            ..
        } = q;
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
            across,
            cornered,
            decided,
        }
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

/// **The bound question, per cell pair**: what [`Sweep::run`] carries
/// across its walk.
struct Bound<'q, W, F> {
    sweep: &'q Sweep,
    injective: bool,
    /// What each candidate's two faces share, in M, by candidate.
    shared: &'q [Shared],
    region: RegionMemo,
    locals: LocalMemo,
    across: BTreeMap<(FaceKey, FaceKey), usize>,
    cornered: usize,
    widths: DischargeWidths,
    first_refusal: Option<EngineRefusal>,
    violation: Option<W>,
    verify: F,
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
            // band, where only a certificate decides it. The
            // strict bound's `c` is 0.
            let apart = matches!(
                decide(
                    bound.predicate(),
                    Margin::of(separation(&pair.a.enc, &pair.b.enc)),
                    band
                ),
                Ok(Sign::Positive)
            );
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
                        injective::certify_self(&px, &py, near, band)
                    } else {
                        injective::certify_pair(&px, &py, near, band)
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
                        self.first_refusal.get_or_insert(r);
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
                            self.first_refusal.get_or_insert(match stop {
                                CellBudget::Depth { .. } => EngineRefusal::Uncertified {
                                    a: x.face,
                                    b: y.face,
                                    shared,
                                    arm,
                                },
                                other => EngineRefusal::Budget(other),
                            });
                        }
                    }
                }
                return ControlFlow::Continue(());
            }
        }

        let c = bound.c();
        let margin = separation(&pair.a.enc, &pair.b.enc) - Interval::from_f64(c);
        let strict = !bound.zero_discharges();
        let verify = &mut self.verify;
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
                strict,
            })
        };
        match decide(bound.predicate(), Margin::of(margin), band) {
            Ok(Sign::Positive) => discharge(&mut walk.receipt, &mut self.widths),
            Ok(Sign::Zero) if bound.zero_discharges() => {
                discharge(&mut walk.receipt, &mut self.widths);
            }
            Ok(Sign::Zero | Sign::Negative) => {
                walk.receipt.violated += 1;
                // Verified HERE, not after the sweep: a verified
                // witness ends the query, and an unverified one
                // must not.
                match candidate(&mut self.region) {
                    Ok(w) => {
                        self.violation = Some(w);
                        return ControlFlow::Break(());
                    }
                    Err(what) => {
                        self.first_refusal
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
                let probe_here = pair.fresh || pair.depth >= walk.budgets.max_cell_depth;
                if probe_here && let Ok(w) = candidate(&mut self.region) {
                    walk.receipt.violated += 1;
                    self.violation = Some(w);
                    return ControlFlow::Break(());
                }
                if source.margin.is_invalid() {
                    // A poison enclosure is not an indeterminacy
                    // refinement could settle, and it is not a
                    // budget: it is geometry that did not
                    // evaluate. Its own class.
                    walk.receipt.refused += 1;
                    self.first_refusal
                        .get_or_insert(EngineRefusal::PoisonEnclosure {
                            a: x.face,
                            b: y.face,
                        });
                    return ControlFlow::Continue(());
                }
                if let Some(predicate) = sliver(&source) {
                    walk.receipt.refused += 1;
                    self.first_refusal
                        .get_or_insert(EngineRefusal::Sliver { predicate });
                    return ControlFlow::Continue(());
                }
                // The injectivity question halves a cell along its
                // length in metres; every other question keeps the
                // relative rule its receipts were taken under.
                let axes = if self.injective {
                    Axes::Metric
                } else {
                    Axes::Relative
                };
                if let Err(stop) =
                    walk.split(task, || split_by(pair, x, y, axes).map(|(lo, hi)| [lo, hi]))
                {
                    self.first_refusal
                        .get_or_insert(EngineRefusal::Budget(stop));
                }
            }
        }
        ControlFlow::Continue(())
    }

    /// **The bound sweep REFUSES a level that does not fit**: its answer
    /// is a verdict over every cell pair, and these are pairs it could
    /// not classify, so the verdict refuses for them
    /// ([`CellBudget::Pairs`]).
    fn overflow(&mut self, receipt: &mut CellReceipt, frontier: Vec<Task>) {
        receipt.refused += frontier.len();
        self.first_refusal
            .get_or_insert(EngineRefusal::Budget(CellBudget::Pairs {
                max_cell_pairs: self.sweep.budgets.max_cell_pairs,
            }));
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
    let at = |m| Interval::from_bounds(m, m);
    Ball::holding(surface.eval(at(u), at(v)), c.enc)
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
/// which pairs feed the bracket and decide nothing a census must see, so
/// they run in [`k_stats::unlogged`]'s scope (audit row F28), and
/// [`MinSeparation::decided`] counts them.
///
/// # Cost
///
/// The readings are taken only where they can change the answer: a
/// station pair that would lower the incumbent, and an `Outside` test on
/// a pair that would otherwise be split or floor the bracket. Even so
/// they are most of a call's work, and the `Outside` test, asked of
/// every pair the walk would split, is most of that. Measured on
/// M10-6's `min_clearance` rows (debug build, one test at a time), a
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
                return Err(MinSeparationRefusal::PoisonEnclosure {
                    a: x.face,
                    b: y.face,
                });
            }
            roots.push((i, j, sep));
        }
    }
    // The incumbent: the least upper bound on the FACES' separation the
    // root pairs witness ([`Bracket::witness`]), which is what makes the
    // very first bracket true.
    let mut q = Bracket {
        hi: None,
        floor: f64::INFINITY,
        poison: None,
        region: RegionMemo::at(band),
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
}

impl Bracket {
    /// Whether a pair whose cells are `lo` apart at the least cannot
    /// hold a smaller separation than the incumbent.
    fn above(&self, lo: f64) -> bool {
        self.hi.is_some_and(|hi| lo >= hi)
    }

    /// Whether either cell of a pair is decided outside its face, read
    /// in the unlogged scope (audit row F28).
    fn outside<T: Decide + CertifiedBounds>(
        &mut self,
        (i, x, a): (usize, &Window<'_, T>, Cell),
        (j, y, b): (usize, &Window<'_, T>, Cell),
    ) -> bool {
        let region = &mut self.region;
        k_stats::unlogged("F28", || {
            region.ask(false, i, x, a) == CellRegion::Outside
                || region.ask(true, j, y, b) == CellRegion::Outside
        })
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
    ///
    /// The readings choose which pairs feed the incumbent and decide
    /// nothing else, so they run in the unlogged scope (audit row F28).
    fn witness<T: Decide + CertifiedBounds>(
        &mut self,
        (i, x, a): (usize, &Window<'_, T>, Cell),
        (j, y, b): (usize, &Window<'_, T>, Cell),
    ) {
        let (sa, sb) = (centre(a.u, a.v), centre(b.u, b.v));
        let at = |w: &Window<'_, T>, (u, v): (f64, f64)| enclosure(&w.surface, (u, u), (v, v));
        let stations = separation(&at(x, sa), &at(y, sb)).hi();
        // A region is read only where its answer could lower the
        // incumbent.
        if !(stations.is_finite() && self.hi.is_none_or(|hi| stations < hi)) {
            return;
        }
        let region = &mut self.region;
        let placed = k_stats::unlogged("F28", || {
            let mut on = |side_b, index, w: &Window<'_, T>, c: Cell, station| match region
                .ask(side_b, index, w, c)
            {
                CellRegion::Inside => true,
                CellRegion::Outside => false,
                CellRegion::Undecided => Candidate::placed(region, side_b, index, w, station),
            };
            on(false, i, x, a, sa) && on(true, j, y, b, sb)
        });
        if placed {
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
        if !(sep.lo().is_finite() && sep.hi().is_finite()) {
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
    /// of "honest at any budget". Not refused: this door refuses nothing
    /// for a budget.
    fn overflow(&mut self, receipt: &mut CellReceipt, frontier: Vec<Task>) {
        for task in frontier {
            self.floor = self
                .floor
                .min(separation(&task.pair.a.enc, &task.pair.b.enc).lo());
            receipt.abandoned += 1;
        }
    }
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
    /// **The body the face is read from**, which [`cell_region`](crate::cell_region::cell_region) reads
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
    /// Each window's stored carrier and boundary lifted, once: every
    /// cell of the window reads them.
    lifted: BTreeMap<(bool, usize), (Surface<Interval>, Vec<BoundaryStop>)>,
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
            Region::Trimmed => match w
                .body
                .get_face(w.face)
                .and_then(|f| w.body.get_surface(f.surface))
            {
                None => CellRegion::Undecided,
                Some(stored) => {
                    let (lifted, boundary) =
                        self.lifted.entry((side_b, index)).or_insert_with(|| {
                            (
                                stored.map_scalar(Interval::from_certified),
                                lift_boundary(w.body, w.face),
                            )
                        });
                    cell_region_lifted(
                        w.body,
                        w.face,
                        (stored, lifted, boundary),
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
    side: Which,
) -> Result<Window<'b, T>, EngineRefusal> {
    let unsupported = |carrier| {
        Err(EngineRefusal::Unsupported {
            carrier,
            face,
            side,
        })
    };
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
    let mut images: BTreeMap<HalfEdgeKey, Image> = BTreeMap::new();
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
                    if lk == f.outer {
                        let surface = body.get_surface(f.surface)?;
                        let periodic =
                            [Axis::U, Axis::V].map(|axis| injective::periodic(surface, axis));
                        out.extent = extent_of(
                            periodic,
                            rows.iter().map(|row| {
                                let (t0, t1) = row.row.params();
                                let end = |t: T| {
                                    let q = row.pcurve.eval(t);
                                    [Interval::from_certified(q.x), Interval::from_certified(q.y)]
                                };
                                row_of(&row.pcurve, (t0, t1), [end(t0), end(t1)])
                            }),
                        );
                    }
                    for row in rows {
                        let (t0, t1) = row.row.params();
                        let lift = |t: T| {
                            let q = row.pcurve.eval(t);
                            Point2::new(
                                Interval::from_certified(q.x),
                                Interval::from_certified(q.y),
                            )
                        };
                        images.insert(
                            row.half_edge,
                            (
                                [lift(t0), lift(t1)],
                                iso_of(&row.pcurve),
                                harmonic_of(&row.pcurve),
                            ),
                        );
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
                    image: found.map(|(image, _, _)| *image),
                    harmonic: found.and_then(|(_, _, h)| *h),
                    iso: found
                        .and_then(|(_, iso, _)| *iso)
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
            arc: Some((carrier, span)),
            ends,
            uses,
        });
    }
    Aabb::from_points(pts).map(|b| (b, out))
}

/// One row of a loop as [`extent_of`] reads it.
enum Row {
    /// A coordinate line whose moving coordinate is monotone
    /// ([`monotone_line`]): its two ends' chart points `[u, v]`, whose
    /// hull is the row's.
    Ends([[Interval; 2]; 2]),
    /// A harmonic image that may bulge past its ends: its enclosure over
    /// its whole span, `[u, v]`, which holds it.
    Bulging([Interval; 2]),
}

/// **One row of a loop's lift as [`extent_of`] reads it**: a monotone
/// coordinate line by its `ends`, its chart points at `t0` and `t1`; a
/// harmonic image by its enclosure over its whole span; `None` for any
/// other image.
fn row_of<T: CertifiedBounds>(
    pcurve: &Pcurve<T>,
    (t0, t1): (T, T),
    ends: [[Interval; 2]; 2],
) -> Option<Row> {
    if !monotone_line(pcurve) {
        let h = harmonic_of(pcurve)?;
        let t = Interval::from_bounds(
            Interval::from_certified(t0).lo(),
            Interval::from_certified(t1).hi(),
        );
        return Some(Row::Bulging(h.at(t)));
    }
    Some(Row::Ends(ends))
}

/// **The face's own extent in its chart** ([`Boundary::extent`]): the
/// hull, on each axis, of the outer loop's lifted chain. A row that is a
/// monotone coordinate line contributes its ends, which are its hull; a
/// harmonic row its enclosure over its span, which holds it, so the
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
fn monotone_line<T: CertifiedBounds>(p: &Pcurve<T>) -> bool {
    let zero = |x: T| injective::exactly(Interval::from_certified(x), 0.0);
    let moves_monotone = |ch: &Channel<T>| match *ch {
        Channel::Angle { .. } => true,
        Channel::Free { a, b, .. } => zero(a) && zero(b),
    };
    match p {
        Pcurve::IsoLine { .. } | Pcurve::IsoArc { .. } => iso_of(p).is_some(),
        Pcurve::Harmonic { u, v } => match iso_of(p) {
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
/// points, the coordinate line it is, and its harmonic form.
type Image = (
    [Point2<Interval>; 2],
    Option<IsoImage>,
    Option<injective::Harmonic>,
);

/// **A harmonic chart image's coefficients, lifted**
/// ([`injective::Harmonic`]); `None` for every other variant.
fn harmonic_of<T: CertifiedBounds>(p: &Pcurve<T>) -> Option<injective::Harmonic> {
    let (p0, pa, pb, pl) = p.harmonic_form()?;
    let lift = |x: T, y: T| [Interval::from_certified(x), Interval::from_certified(y)];
    Some(injective::Harmonic {
        p0: lift(p0.x, p0.y),
        pa: lift(pa.x, pa.y),
        pb: lift(pb.x, pb.y),
        pl: lift(pl.x, pl.y),
    })
}

/// **A chart image that is a coordinate line, by variant**: which
/// coordinate it holds, the value, and whether its moving coordinate is
/// an angle. The held coordinate is exact structure — a velocity, or a
/// harmonic channel's three coefficients, that lift to the point zero —
/// so no zero is decided here.
fn iso_of<T: CertifiedBounds>(p: &Pcurve<T>) -> Option<IsoImage> {
    let zero = |x: T| injective::exactly(Interval::from_certified(x), 0.0);
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
            let at = |x: f64| Interval::from_bounds(x, x);
            let (mut u_least, mut u_most) = (f64::INFINITY, 0.0f64);
            let (mut v_least, mut v_most) = (f64::INFINITY, 0.0f64);
            for u in stations(cell.u) {
                for v in stations(cell.v) {
                    let j = root.surface.jet(at(u), at(v));
                    let (su, sv) = (j.du.norm().hi(), j.dv.norm().hi());
                    (u_least, u_most) = (u_least.min(su), u_most.max(su));
                    (v_least, v_most) = (v_least.min(sv), v_most.max(sv));
                }
            }
            if 8.0 * u_least < u_most || 8.0 * v_least < v_most {
                cell.relative(root)
            } else {
                let j = root.surface.jet(
                    at(geom::mid_param(cell.u.0, cell.u.1)),
                    at(geom::mid_param(cell.v.0, cell.v.1)),
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
/// scalar and read the door off [`crate::AtRestPolicy::clearance_lane`],
/// the per-scalar policy home, refusing typed where it answers `None`.
///
/// Holding one is the statement that the scalar may certify: the one
/// constructor ([`ClearanceLane::certified`]) is bounded on
/// [`CertifiedBounds`], since every door it holds lifts what it reads
/// through [`Interval::from_certified`] (this module's header). A
/// scalar without that right cannot write the value:
///
/// ```compile_fail,E0599
/// use geom_core::Dual64;
/// use topo::ClearanceLane;
/// let _ = ClearanceLane::<Dual64>::certified();
/// ```
///
/// Why the code is `E0599`, and what the fence does not check:
/// [`crate::QuadLane`]'s paragraph.
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct ClearanceLane<T: Decide> {
    /// [`Side::windows`] (`wiring_rows` pins the pointer).
    windows: for<'b> fn(&Side<'b, T>, Which) -> Result<Windows<'b, T>, EngineRefusal>,
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
    /// [`min_separation`].
    min_separation: fn(
        &Side<'_, T>,
        &Side<'_, T>,
        bool,
        Band,
        Budgets,
    ) -> Result<MinSeparation, MinSeparationRefusal>,
}

impl<T: Decide + CertifiedBounds> ClearanceLane<T> {
    /// The certified engine: the whole inventory of this door, and the
    /// only constructor there is.
    #[must_use]
    pub const fn certified() -> Self {
        Self {
            windows: Self::windows_of,
            bound_sweep: Self::sweep_of,
            face_stations: Self::stations_of,
            min_separation: min_separation::<T>,
        }
    }

    fn windows_of<'b>(side: &Side<'b, T>, which: Which) -> Result<Windows<'b, T>, EngineRefusal> {
        side.windows(which)
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

impl<T: Decide> ClearanceLane<T> {
    /// [`Side::windows`], at this lane.
    ///
    /// # Errors
    ///
    /// [`Side::windows`]'s own.
    pub fn windows<'b>(
        self,
        side: &Side<'b, T>,
        which: Which,
    ) -> Result<Windows<'b, T>, EngineRefusal> {
        (self.windows)(side, which)
    }

    /// [`Sweep::run`], at this lane: the verdict, receipt and widths are
    /// the sweep's, and the witness is the first one `verify` answered.
    pub fn sweep<W>(
        self,
        sweep: &Sweep,
        wa: &Windows<'_, T>,
        wb: &Windows<'_, T>,
        same_body: bool,
        mut verify: impl FnMut(Candidate<'_, '_, T>) -> Result<W, String>,
    ) -> BoundReport<W> {
        let mut witness = None;
        let report = (self.bound_sweep)(sweep, wa, wb, same_body, &mut |candidate| {
            verify(candidate).map(|w| witness = Some(w))
        });
        let verdict = match report.verdict {
            BoundVerdict::Holds => BoundVerdict::Holds,
            BoundVerdict::Refused(r) => BoundVerdict::Refused(r),
            BoundVerdict::Violated(()) => BoundVerdict::Violated(
                witness.unwrap_or_else(|| unreachable!("the sweep stops at the verifier's `Ok`")),
            ),
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

    /// [`Candidate::on_face_stations`], at this lane.
    pub fn on_face_stations(self, candidate: Candidate<'_, '_, T>) -> (Stations, Stations) {
        (self.face_stations)(candidate)
    }

    /// [`min_separation`], at this lane, with its bracket re-entered as
    /// a value of `T`: the hull of `T::from_computed` of each end. At
    /// `Interval` that is [`MinSeparation::enclosure`]; at a symbolic
    /// scalar each end is its own unknown, so two measures with equal
    /// brackets are never proved equal. `None` when the bracket has no
    /// upper end ([`MinSeparation::hi`]).
    ///
    /// # Errors
    ///
    /// [`min_separation`]'s own.
    pub fn min_separation(
        self,
        a: &Side<'_, T>,
        b: &Side<'_, T>,
        same_body: bool,
        band: Band,
        budgets: Budgets,
    ) -> Result<Option<T>, MinSeparationRefusal> {
        let m = (self.min_separation)(a, b, same_body, band, budgets)?;
        Ok(m.hi.map(|hi| {
            geom_core::SpanLocate::enclosure_hull(T::from_computed(m.lo), T::from_computed(hi))
        }))
    }
}

/// **The door's WIRING**: which function each field of
/// [`ClearanceLane::certified`] holds. Why a wiring row compares
/// pointers rather than outputs: `certified_enclosure_impl_census`'s
/// module doc.
#[cfg(test)]
mod wiring_rows {
    use super::{ClearanceLane, min_separation};

    /// `Ok(())` when every field holds its door; otherwise the name of
    /// the first field that does not.
    fn holds_the_certified_clearance_lane<T: super::Decide + geom_core::CertifiedBounds>()
    -> Result<(), &'static str> {
        let lane = ClearanceLane::<T>::certified();
        if !std::ptr::fn_addr_eq(
            lane.windows,
            ClearanceLane::<T>::windows_of as fn(_, _) -> _,
        ) {
            return Err("windows is not `Side::windows`");
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
        if !std::ptr::fn_addr_eq(
            lane.min_separation,
            min_separation::<T> as fn(_, _, _, _, _) -> _,
        ) {
            return Err("min_separation is not `min_separation`");
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

    /// Which scalars hold the door, one row for all of them: an answer
    /// that flips at any scalar reds here by name.
    #[test]
    fn the_door_answers_at_the_enclosure_scalars_and_nowhere_else() {
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
    use geom_core::Bounds;

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
        let Some(Row::Bulging([u, v])) = row_of(&arch(), (0.0, PI), flat(PI)) else {
            panic!("the arch is read by its enclosure");
        };
        assert!(
            u.lo() <= 0.0 && PI <= u.hi() && v.hi() >= 1.0,
            "the enclosure holds the arch: {u:?} {v:?}"
        );
        let [(_, _), (_, top)] =
            extent_of(CYLINDER, [row_of(&arch(), (0.0, PI), flat(PI))].into_iter())
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
                [row_of(&arch(), (0.0, TAU), flat(TAU))].into_iter()
            )
            .is_none(),
            "a turn-wide superset reads unknown"
        );
        let wide = [row_of(&arch(), (0.0, TAU), flat(TAU))];
        assert!(
            extent_of([false, false], wide.into_iter()).is_some(),
            "a superset a turn wide on an axis that is not periodic is taken"
        );
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
