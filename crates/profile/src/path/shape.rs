//! **A chain's shape, with values erased**: the walk over its verbs,
//! target forms and arc modes, the refusals its step order earns at
//! every value, the pieces that structure can draw, and which of their
//! joints it constructs onto one carrier.
//!
//! A replay ([`super::program::replay`]) answers all three at one set
//! of values, and refuses where those values do; the shape answers what
//! no value moves. Its input, [`StepForm`], holds no number: a step's
//! verb, the form of each target ([`TargetKind`]), the mode of each arc
//! spec ([`ArcMode`]) with its side or sense, and a `circle_split`'s
//! written count.
//!
//! Legality is the transition table's ([`Verb::states`],
//! [`arc_specs_at`]); the state each row lands in, what it emits, and
//! the structural facts the replay's order refusals read
//! ([`super::order`]) are stated per row below. That they are the
//! replay's is held cell by cell against it (`tests/arc_spec_census.rs`)
//! and over random chains (`tests/chain_shape_random.rs`).

use super::order::{self, OrderFault};
use super::program::{ArcData, ArcMode, Step, TargetKind, TipState, Verb, arc_specs_at};
use super::verbs::ArcSide;
use crate::ArcSweep;
use crate::structure::{Piece, PieceRole, RoleList, carrier_pieces};
use crate::{Real, Refused, Vocabulary};

/// **A target form a [`StepForm`] can carry**: the kernel's own
/// [`TargetKind`], or a caller's richer form (an editor's trace
/// crossing) that names the kernel target it lowers to.
pub trait FormTarget: Copy {
    /// The kernel's target form this one lowers to.
    fn kernel(self) -> TargetKind;
}

impl FormTarget for TargetKind {
    fn kernel(self) -> Self {
        self
    }
}

/// **A caller's step the kernel has no verb for**, which it walks as
/// the verb it lowers to ([`StepForm::Lowered`]). The kernel's own
/// forms hold none ([`core::convert::Infallible`]).
pub trait FormLowered: Copy {
    /// The kernel verb this step lowers to.
    fn verb(self) -> Verb;
}

impl FormLowered for core::convert::Infallible {
    fn verb(self) -> Verb {
        match self {}
    }
}

/// **An arc spec, values erased**: its mode, its target's form, and the
/// structural tags its mode carries (a side, a travel sense).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArcForm<K = TargetKind> {
    /// `Radius`, and the side its centre sits on.
    Radius(ArcSide),
    /// `Bulge`, and its target.
    Bulge(K),
    /// `Via`, and its target.
    Via(K),
    /// `Center`, its travel sense and its target.
    Center(ArcSweep, K),
    /// `Sweep`, and the side its centre sits on.
    Sweep(ArcSide),
    /// `ArcLen`, and the side its centre sits on.
    ArcLen(ArcSide),
}

impl<K: Copy> ArcForm<K> {
    /// The spec's mode.
    #[must_use]
    pub const fn mode(self) -> ArcMode {
        match self {
            Self::Radius(_) => ArcMode::Radius,
            Self::Bulge(_) => ArcMode::Bulge,
            Self::Via(_) => ArcMode::Via,
            Self::Center(..) => ArcMode::Center,
            Self::Sweep(_) => ArcMode::Sweep,
            Self::ArcLen(_) => ArcMode::ArcLen,
        }
    }

    /// Its target's form, `None` for the endpoint-free modes.
    #[must_use]
    pub const fn target(self) -> Option<K> {
        match self {
            Self::Bulge(t) | Self::Via(t) | Self::Center(_, t) => Some(t),
            Self::Radius(_) | Self::Sweep(_) | Self::ArcLen(_) => None,
        }
    }

    /// This spec with its target's form mapped by `f`.
    #[must_use]
    pub fn map<L>(self, f: impl FnOnce(K) -> L) -> ArcForm<L> {
        match self {
            Self::Radius(s) => ArcForm::Radius(s),
            Self::Bulge(t) => ArcForm::Bulge(f(t)),
            Self::Via(t) => ArcForm::Via(f(t)),
            Self::Center(w, t) => ArcForm::Center(w, f(t)),
            Self::Sweep(s) => ArcForm::Sweep(s),
            Self::ArcLen(s) => ArcForm::ArcLen(s),
        }
    }
}

impl<T: Real> From<&ArcData<T>> for ArcForm {
    fn from(spec: &ArcData<T>) -> Self {
        match spec {
            ArcData::Radius { side, .. } => Self::Radius(*side),
            ArcData::Bulge { target, .. } => Self::Bulge(target.kind()),
            ArcData::Via { target, .. } => Self::Via(target.kind()),
            ArcData::Center {
                winding, target, ..
            } => Self::Center(*winding, target.kind()),
            ArcData::Sweep { side, .. } => Self::Sweep(*side),
            ArcData::ArcLen { side, .. } => Self::ArcLen(*side),
        }
    }
}

/// **One step, values erased**: its verb and the structural tags it
/// carries (target forms, arc modes, sides, senses, a split's count),
/// which is what [`chain_shape`] reads, and all it can. `K` is the
/// target form and `X` a caller's step with no kernel verb: the kernel
/// itself walks `StepForm<TargetKind>`, and a caller with a richer
/// vocabulary (the editor's traced steps) holds its own `K` and `X` and
/// lowers them ([`StepForm::lower`]). One form, so the value-erased
/// address of a step and the walk's input cannot drift apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StepForm<K = TargetKind, X = core::convert::Infallible> {
    /// `at`.
    At,
    /// `angle`.
    Angle,
    /// `toward`.
    Toward,
    /// `tangent`.
    Tangent,
    /// `cusp`.
    Cusp,
    /// `turn`.
    Turn,
    /// `line`.
    Line,
    /// `line_to`.
    LineTo(K),
    /// `continue_to`.
    ContinueTo(K),
    /// `arc_to`.
    ArcTo(ArcForm<K>),
    /// `tangent_arc_to`.
    TangentArcTo(K),
    /// `arc_onto`.
    ArcOnto(K),
    /// `fillet`.
    Fillet,
    /// `fillet_arc`, and its arrival spec.
    FilletArc(ArcForm<K>),
    /// `arc_fillet`, and its incoming spec.
    ArcFillet(ArcForm<K>),
    /// `arc_fillet_arc`, its incoming spec and its arrival spec.
    ArcFilletArc(ArcForm<K>, ArcForm<K>),
    /// `to (far end)`.
    FarEndTo,
    /// `to Start (close)`.
    CloseTo,
    /// `circle`.
    Circle,
    /// `circle_split`, and its written count.
    CircleSplit {
        /// The count.
        n: usize,
    },
    /// `rectangle`.
    Rectangle,
    /// A caller's step with no kernel verb, walked as the verb it
    /// lowers to.
    Lowered(X),
}

impl<K: Copy, X: FormLowered> StepForm<K, X> {
    /// The verb this step names, a lowered step's the one it lowers to.
    #[must_use]
    pub fn verb(self) -> Verb {
        match self {
            Self::At => Verb::At,
            Self::Angle => Verb::Angle,
            Self::Toward => Verb::Toward,
            Self::Tangent => Verb::Tangent,
            Self::Cusp => Verb::Cusp,
            Self::Turn => Verb::Turn,
            Self::Line => Verb::Line,
            Self::LineTo(_) => Verb::LineTo,
            Self::ContinueTo(_) => Verb::ContinueTo,
            Self::ArcTo(_) => Verb::ArcTo,
            Self::TangentArcTo(_) => Verb::TangentArcTo,
            Self::ArcOnto(_) => Verb::ArcOnto,
            Self::Fillet => Verb::Fillet,
            Self::FilletArc(_) => Verb::FilletArc,
            Self::ArcFillet(_) => Verb::ArcFillet,
            Self::ArcFilletArc(..) => Verb::ArcFilletArc,
            Self::FarEndTo => Verb::FarEndTo,
            Self::CloseTo => Verb::CloseTo,
            Self::Circle => Verb::Circle,
            Self::CircleSplit { .. } => Verb::CircleSplit,
            Self::Rectangle => Verb::Rectangle,
            Self::Lowered(x) => x.verb(),
        }
    }

    /// How many pieces a complete-loop form draws; `0` for every other
    /// step, whose roles are not indexed ([`carrier_pieces`]).
    #[must_use]
    pub fn pieces(self) -> u32 {
        let n = match self {
            Self::CircleSplit { n } => n,
            _ => 0,
        };
        carrier_pieces(self.verb(), n)
    }

    /// The roles this step may draw: its verb's list.
    #[must_use]
    pub fn roles(self) -> RoleList {
        RoleList::of(self.verb())
    }

    /// Whether this step may draw `role` ([`RoleList::admits`]).
    #[must_use]
    pub fn admits(self, role: PieceRole) -> bool {
        self.roles().admits(role, self.pieces())
    }

    /// Its arc specs, in the step's field order.
    #[must_use]
    pub fn specs(self) -> Vec<ArcForm<K>> {
        match self {
            Self::ArcTo(a) | Self::FilletArc(a) | Self::ArcFillet(a) => vec![a],
            Self::ArcFilletArc(a, b) => vec![a, b],
            _ => Vec::new(),
        }
    }

    /// **This step in another vocabulary**: each target form mapped by
    /// `target`, a lowered step by `lowered`.
    #[must_use]
    pub fn lower<L: Copy, Y>(
        self,
        target: impl Fn(K) -> L,
        lowered: impl FnOnce(X) -> StepForm<L, Y>,
    ) -> StepForm<L, Y> {
        match self {
            Self::At => StepForm::At,
            Self::Angle => StepForm::Angle,
            Self::Toward => StepForm::Toward,
            Self::Tangent => StepForm::Tangent,
            Self::Cusp => StepForm::Cusp,
            Self::Turn => StepForm::Turn,
            Self::Line => StepForm::Line,
            Self::LineTo(t) => StepForm::LineTo(target(t)),
            Self::ContinueTo(t) => StepForm::ContinueTo(target(t)),
            Self::ArcTo(a) => StepForm::ArcTo(a.map(&target)),
            Self::TangentArcTo(t) => StepForm::TangentArcTo(target(t)),
            Self::ArcOnto(t) => StepForm::ArcOnto(target(t)),
            Self::Fillet => StepForm::Fillet,
            Self::FilletArc(a) => StepForm::FilletArc(a.map(&target)),
            Self::ArcFillet(a) => StepForm::ArcFillet(a.map(&target)),
            Self::ArcFilletArc(a, b) => StepForm::ArcFilletArc(a.map(&target), b.map(&target)),
            Self::FarEndTo => StepForm::FarEndTo,
            Self::CloseTo => StepForm::CloseTo,
            Self::Circle => StepForm::Circle,
            Self::CircleSplit { n } => StepForm::CircleSplit { n },
            Self::Rectangle => StepForm::Rectangle,
            Self::Lowered(x) => lowered(x),
        }
    }
}

impl<T: Real> Step<T> {
    /// **This step with its values erased.**
    #[must_use]
    pub fn form(&self) -> StepForm {
        match self {
            Self::At(_) => StepForm::At,
            Self::Angle(_) => StepForm::Angle,
            Self::Toward { .. } => StepForm::Toward,
            Self::Tangent => StepForm::Tangent,
            Self::Cusp => StepForm::Cusp,
            Self::Turn(_) => StepForm::Turn,
            Self::Line(_) => StepForm::Line,
            Self::LineTo(t) => StepForm::LineTo(t.kind()),
            Self::ContinueTo(t) => StepForm::ContinueTo(t.kind()),
            Self::ArcTo(spec) => StepForm::ArcTo(spec.into()),
            Self::TangentArcTo(t) => StepForm::TangentArcTo(t.kind()),
            Self::ArcOnto(t) => StepForm::ArcOnto(t.kind()),
            Self::Fillet { .. } => StepForm::Fillet,
            Self::FilletArc { spec, .. } => StepForm::FilletArc(spec.into()),
            Self::ArcFillet { spec, .. } => StepForm::ArcFillet(spec.into()),
            Self::ArcFilletArc { spec, spec2, .. } => {
                StepForm::ArcFilletArc(spec.into(), spec2.into())
            }
            Self::FarEndTo(_) => StepForm::FarEndTo,
            Self::CloseTo => StepForm::CloseTo,
            Self::Circle { .. } => StepForm::Circle,
            Self::CircleSplit { n, .. } => StepForm::CircleSplit { n: *n },
            Self::Rectangle { .. } => StepForm::Rectangle,
        }
    }
}

/// **Where a chain's shape refuses**: at `step`, with the tip in
/// `state` (`step` one past the last for a chain that does not close).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShapeFault {
    /// The step the walk stopped at.
    pub step: usize,
    /// The tip's lattice state there.
    pub state: TipState,
    /// What is wrong.
    pub kind: ShapeFaultKind,
}

/// What a [`ShapeFault`] is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeFaultKind {
    /// The table has no row for `verb` at the state, or its arc spec a
    /// form the row does not take; `None` for a chain that ends without
    /// closing.
    Lattice {
        /// The step's verb.
        verb: Option<Verb>,
    },
    /// The row takes the step, and the replay refuses its order at
    /// every value.
    Order(OrderFault),
}

impl ShapeFault {
    /// The lattice sentence for a [`ShapeFaultKind::Lattice`] fault
    /// ([`Refused`]); `None` for an order fault, whose sentence is its
    /// own ([`OrderFault`]'s `Display`).
    #[must_use]
    pub fn refused(self, vocabulary: Vocabulary) -> Option<Refused> {
        match self.kind {
            ShapeFaultKind::Lattice { verb } => Some(self.state.refused(verb, vocabulary)),
            ShapeFaultKind::Order(_) => None,
        }
    }
}

/// **What a closed chain's structure draws, and the states it walks**.
///
/// A piece absent from `pieces` is absent at every value: its step's
/// verb has no such role, a `circle_split` count leaves it out, or
/// another piece takes its segment by construction (a fillet's run out
/// takes the next segment its arrival emits, if that segment is
/// straight, whichever step emits it, and a fillet closing the chain at its seam draws no run
/// out). A piece present is drawn at the values where every fillet's
/// runs fit with length to spare, and may go undrawn at others (a run
/// that fits `Zero`). Whether the chain replays at any value at all is
/// geometry's question, not the shape's.
///
/// A piece `continues` the one before it where the step order
/// constructs it onto that piece's carrier: a straight segment departing
/// along a straight segment's own ray (a straight continuation, a
/// station kept on a side, a fillet's run in after its leg), every
/// station of a `circle` or a `circle_split`, and both joints of a
/// loop of two arcs whose closing `tangent_arc_to` leaves the first
/// along its own tangent. Every other joint is a corner at generic
/// values: no verb reads an incoming arc's circle but that closer,
/// and a `line_to` computes its direction, so two pieces on one carrier
/// there is a coincidence of values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChainShape {
    /// The pieces, in the order the chain emits their segments.
    pub pieces: Vec<Piece>,
    /// Per piece, whether it continues the carrier of the piece before
    /// it (the last, for the first) by construction.
    pub continues: Vec<bool>,
    /// The tip's state before each step, in step order.
    pub entered: Vec<TipState>,
}

/// The kind of segment an emission is, which is all a run out's claim
/// on it, and a seam fillet's retrim, read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seg {
    Straight,
    Arc,
}

/// An opened fillet awaiting a straight arrival: its step, the kind
/// of its run in, and whether the run in departs along the incoming
/// segment's own ray.
#[derive(Clone, Copy, Debug)]
struct Opened {
    step: usize,
    run_in: Seg,
    along: bool,
}

/// How a straight arrival ends its fillet: riding on, so the run out
/// is the next segment, if straight (the far-end anchor's own included),
/// or at the seam, which draws no run out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arrival {
    Continues,
    Seam,
}

/// The walk's record: what it has emitted, and the structural facts
/// the order refusals read.
#[derive(Default)]
struct Walk {
    pieces: Vec<Piece>,
    continues: Vec<bool>,
    first: Option<Seg>,
    last: Option<Seg>,
    /// Whether the director that bound the tip's heading was
    /// `.tangent()`, which keeps the incoming segment's own ray.
    tangent: bool,
    opened: Option<Opened>,
    run_out: Option<usize>,
    /// Whether an `arc_onto(Start)` awaiting its director had a leg
    /// drawn before its own.
    onto_start_drawn: bool,
}

impl Walk {
    /// A segment `piece` would be, which a pending run out takes when
    /// it is straight; `along` where it departs along the tip's own
    /// ray, so a straight segment continues a straight one before it.
    fn emit(&mut self, piece: Piece, seg: Seg, along: bool) {
        let run_out = self.run_out.take();
        let piece = match run_out {
            Some(step) if seg == Seg::Straight && piece.role != PieceRole::Arc => Piece {
                step,
                role: PieceRole::RunOut,
            },
            _ => piece,
        };
        self.first.get_or_insert(seg);
        self.continues
            .push(along && seg == Seg::Straight && self.last == Some(Seg::Straight));
        self.last = Some(seg);
        self.pieces.push(piece);
    }

    fn leg(&mut self, step: usize, seg: Seg, along: bool) {
        self.emit(
            Piece {
                step,
                role: PieceRole::Leg,
            },
            seg,
            along,
        );
    }

    /// Whether a step departing from a tip in `state` along its bound
    /// heading departs along the incoming segment's own ray: a leg end
    /// with no director, or one `.tangent()` bound.
    fn along(&self, state: TipState) -> bool {
        match state {
            TipState::DirectedPoint => true,
            TipState::DirectedIncoming => self.tangent,
            _ => false,
        }
    }

    /// **A closing arc that lies on the first piece's circle**, where
    /// that piece is an arc and the only one drawn before it: an arc
    /// back to the entry tangent to that arc at the tip (`rounds`, a
    /// `tangent_arc_to` off its own ray) is the one circle through those
    /// two points with that tangent, so both of the loop's joints
    /// continue it. (`arc_onto(StartArriving)` would be too, and departs
    /// tangent to that arc, which its junction check refuses.)
    fn close_on_the_first_circle(&mut self, rounds: bool) {
        if rounds && self.first == Some(Seg::Arc) && self.pieces.len() == 2 {
            self.continues.fill(true);
        }
    }

    /// A fillet opened at a tip in `state`.
    fn open(&mut self, step: usize, run_in: Seg, state: TipState) {
        self.opened = Some(Opened {
            step,
            run_in,
            along: self.along(state),
        });
    }

    fn drawn(&self) -> bool {
        !self.pieces.is_empty()
    }

    /// The opened fillet's run in and arc, and its run out where the
    /// arrival rides on.
    fn resolve(&mut self, arrival: Arrival) {
        let Some(Opened {
            step,
            run_in,
            along,
        }) = self.opened.take()
        else {
            return;
        };
        self.fillet_runs(step, run_in, along);
        if arrival == Arrival::Continues {
            self.run_out = Some(step);
        }
    }

    fn fillet_runs(&mut self, step: usize, run_in: Seg, along: bool) {
        self.emit(
            Piece {
                step,
                role: PieceRole::RunIn,
            },
            run_in,
            along,
        );
        self.emit(
            Piece {
                step,
                role: PieceRole::Arc,
            },
            Seg::Arc,
            false,
        );
    }

    /// A fillet whose arrival arc the step authors, opened at a tip in
    /// `state`: its three pieces, the run out that arc.
    fn authored_arrival(&mut self, step: usize, run_in: Seg, state: TipState) {
        let along = self.along(state);
        self.fillet_runs(step, run_in, along);
        self.emit(
            Piece {
                step,
                role: PieceRole::RunOut,
            },
            Seg::Arc,
            false,
        );
    }

    /// A leg closing at `target` needs a leg drawn before it: an arc
    /// spans a chord from the tip to the entry, zero with none drawn,
    /// and a straight leg classifies the seam, which reads the first
    /// leg's arm.
    fn classified_close(&self, target: TargetKind, seg: Seg) -> Result<(), OrderFault> {
        match (target, seg) {
            (TargetKind::Point, _) => Ok(()),
            (_, Seg::Arc) => order::a_closing_arc_leaves_its_entry(self.drawn()),
            (_, Seg::Straight) => order::seam_reads_a_first_leg(self.drawn()),
        }
    }
}

/// Where a step aimed at `target` lands: on at a point, closed at the
/// start.
fn reaches(target: TargetKind, on: TipState) -> TipState {
    match target {
        TargetKind::Point => on,
        TargetKind::Start | TargetKind::StartArriving => TipState::Closed,
    }
}

/// The state an arrival spec lands its fillet's chain in.
fn arrival_lands(spec: ArcForm) -> TipState {
    match (spec.mode(), spec.target()) {
        (ArcMode::Radius, _) => TipState::RadiusArrival,
        (ArcMode::Via, Some(TargetKind::Point)) => TipState::ViaArrival,
        (ArcMode::Via, Some(TargetKind::Start)) => TipState::ViaArrivalStart,
        (_, Some(TargetKind::Point)) => TipState::DirectedPoint,
        _ => TipState::Closed,
    }
}

/// Whether the transition table takes `step` at `state`: a row for its
/// verb, and each arc spec a form that row's dispatcher admits.
fn admitted(state: TipState, step: StepForm) -> bool {
    let verb = step.verb();
    if !verb.states().contains(&state) {
        return false;
    }
    let forms = arc_specs_at(verb, state);
    step.specs().iter().enumerate().all(|(i, spec)| {
        forms
            .get(i)
            .is_some_and(|f| f.admits(spec.mode(), spec.target()))
    })
}

/// **The chain `steps`, walked with values erased** (module docs).
///
/// # Errors
///
/// [`ShapeFault`] at the first step the table refuses
/// ([`ShapeFaultKind::Lattice`], also at a step after the chain closed,
/// or one past the end of a chain that does not close) or whose order
/// the replay refuses at every value ([`ShapeFaultKind::Order`]): at
/// any values that reach that step, [`super::program::replay`] refuses
/// there in the same class.
pub fn chain_shape(steps: &[StepForm]) -> Result<ChainShape, ShapeFault> {
    let mut entered = Vec::with_capacity(steps.len());
    let (pieces, continues) = walk_chain(steps, &mut entered)?;
    Ok(ChainShape {
        pieces,
        continues,
        entered,
    })
}

/// **The tip's state before each step of `steps`, as far as
/// [`chain_shape`] walks**: the step it refuses at included, so a
/// reader of the state a step meets ([`TipState::pen`]) reads it off
/// one walk.
#[must_use]
pub fn chain_states(steps: &[StepForm]) -> Vec<TipState> {
    let mut entered = Vec::with_capacity(steps.len());
    match walk_chain(steps, &mut entered) {
        Err(fault) if entered.len() == fault.step && fault.step < steps.len() => {
            entered.push(fault.state);
        }
        _ => {}
    }
    entered
}

/// The pieces a closed chain draws, and per piece whether it continues
/// the carrier before it.
type Drawn = (Vec<Piece>, Vec<bool>);

fn walk_chain(steps: &[StepForm], entered: &mut Vec<TipState>) -> Result<Drawn, ShapeFault> {
    use TipState as S;
    let mut walk = Walk::default();
    let mut state = S::Entry;
    for (i, &step) in steps.iter().enumerate() {
        let fault = |kind| ShapeFault {
            step: i,
            state,
            kind,
        };
        let order = |fault: OrderFault| ShapeFault {
            step: i,
            state,
            kind: ShapeFaultKind::Order(fault),
        };
        if state == S::Closed || !admitted(state, step) {
            return Err(fault(ShapeFaultKind::Lattice {
                verb: Some(step.verb()),
            }));
        }
        entered.push(state);
        state = match (state, step) {
            (S::Entry | S::Open, StepForm::At) => S::PlainPoint,
            (S::Angle, StepForm::At) => {
                walk.resolve(Arrival::Continues);
                S::DirectedPlain
            }
            (S::RadiusArrival, StepForm::At) => S::RadiusArrivalAt,
            (S::RadiusArrivalDir, StepForm::At) => S::DirectedPoint,
            (_, StepForm::At) => unreachable!("`at` has a row at {state:?}: admitted above"),
            (s, StepForm::Angle | StepForm::Toward) => match s {
                S::Entry | S::Open => S::Angle,
                S::PlainPoint => {
                    walk.resolve(Arrival::Continues);
                    S::DirectedPlain
                }
                S::DirectedPoint => {
                    walk.tangent = false;
                    S::DirectedIncoming
                }
                S::RadiusArrival => S::RadiusArrivalDir,
                S::ViaArrivalStart => S::Closed,
                S::ArcOntoStart => {
                    order::a_closing_arc_leaves_its_entry(walk.onto_start_drawn).map_err(order)?;
                    S::Closed
                }
                _ => S::DirectedPoint,
            },
            (_, StepForm::Tangent | StepForm::Cusp | StepForm::Turn) => {
                walk.tangent = step == StepForm::Tangent;
                S::DirectedIncoming
            }
            (_, StepForm::Line) => {
                let along = walk.along(state);
                walk.leg(i, Seg::Straight, along);
                S::DirectedPoint
            }
            (_, StepForm::LineTo(target) | StepForm::ContinueTo(target)) => {
                walk.resolve(Arrival::Continues);
                walk.classified_close(target, Seg::Straight)
                    .map_err(order)?;
                let along = matches!(step, StepForm::ContinueTo(_));
                walk.leg(i, Seg::Straight, along);
                reaches(target, S::DirectedPoint)
            }
            (S::DirectedPlain | S::DirectedIncoming, StepForm::ArcTo(_)) => {
                walk.leg(i, Seg::Arc, false);
                S::DirectedPoint
            }
            (_, StepForm::ArcTo(spec)) => {
                let target = spec.target().unwrap_or(TargetKind::Point);
                let site = match target {
                    TargetKind::Point => order::site::ARC_TO,
                    TargetKind::Start | TargetKind::StartArriving => order::site::ARC_TO_START,
                };
                order::arc_leg_leaves_no_open_fillet(walk.opened.is_some(), site).map_err(order)?;
                walk.classified_close(target, Seg::Arc).map_err(order)?;
                walk.leg(i, Seg::Arc, false);
                reaches(target, S::DirectedPoint)
            }
            (_, StepForm::TangentArcTo(target)) => {
                walk.classified_close(target, Seg::Arc).map_err(order)?;
                let rounds = target != TargetKind::Point && walk.along(state);
                walk.leg(i, Seg::Arc, false);
                walk.close_on_the_first_circle(rounds);
                reaches(target, S::DirectedPoint)
            }
            (_, StepForm::ArcOnto(target)) => {
                order::arc_leg_leaves_no_open_fillet(walk.opened.is_some(), order::site::ARC_ONTO)
                    .map_err(order)?;
                let drawn = walk.drawn();
                walk.onto_start_drawn = drawn;
                match target {
                    TargetKind::Point => {
                        walk.leg(i, Seg::Arc, false);
                        S::ArcOnto
                    }
                    TargetKind::Start => {
                        walk.leg(i, Seg::Arc, false);
                        S::ArcOntoStart
                    }
                    TargetKind::StartArriving => {
                        order::a_closing_arc_leaves_its_entry(drawn).map_err(order)?;
                        walk.leg(i, Seg::Arc, false);
                        S::Closed
                    }
                }
            }
            (_, StepForm::Fillet) => {
                walk.open(i, Seg::Straight, state);
                S::Open
            }
            (_, StepForm::ArcFillet(_)) => {
                fused_leaves_no_open_fillet(&walk, state).map_err(order)?;
                walk.open(i, Seg::Arc, state);
                S::Open
            }
            (_, StepForm::FilletArc(spec)) => {
                walk.authored_arrival(i, Seg::Straight, state);
                arrival_lands(spec)
            }
            (_, StepForm::ArcFilletArc(_, spec2)) => {
                fused_leaves_no_open_fillet(&walk, state).map_err(order)?;
                walk.authored_arrival(i, Seg::Arc, state);
                arrival_lands(spec2)
            }
            (_, StepForm::FarEndTo) => {
                order::far_end_ends_an_open_fillet(walk.opened.is_some()).map_err(order)?;
                walk.resolve(Arrival::Continues);
                walk.leg(i, Seg::Straight, false);
                S::DirectedPoint
            }
            (_, StepForm::CloseTo) => {
                order::seam_fillet_retrims_a_straight_side(walk.first == Some(Seg::Straight))
                    .map_err(order)?;
                walk.resolve(Arrival::Seam);
                S::Closed
            }
            (_, StepForm::CircleSplit { n }) => {
                order::circle_split_count(n).map_err(order)?;
                carrier(&mut walk, i, carrier_pieces(Verb::CircleSplit, n), true)
            }
            (_, StepForm::Circle) => carrier(&mut walk, i, carrier_pieces(Verb::Circle, 0), true),
            (_, StepForm::Rectangle) => {
                carrier(&mut walk, i, carrier_pieces(Verb::Rectangle, 0), false)
            }
        };
    }
    if state != S::Closed {
        return Err(ShapeFault {
            step: steps.len(),
            state,
            kind: ShapeFaultKind::Lattice { verb: None },
        });
    }
    Ok((walk.pieces, walk.continues))
}

/// A fused verb's incoming arc from a plain point, which is an open
/// fillet's arrival anchor where one is open.
fn fused_leaves_no_open_fillet(walk: &Walk, state: TipState) -> Result<(), OrderFault> {
    if state != TipState::PlainPoint {
        return Ok(());
    }
    order::arc_leg_leaves_no_open_fillet(walk.opened.is_some(), order::site::ARC_FILLET)
}

/// A complete-loop form's `pieces`, drawn whole: the chain is closed.
/// Where `one_carrier`, the form is one circle cut at stations (a
/// `circle`'s, a `circle_split`'s), so each joint of two pieces or more
/// continues it.
fn carrier(walk: &mut Walk, step: usize, pieces: u32, one_carrier: bool) -> TipState {
    for k in 0..pieces {
        walk.pieces.push(Piece {
            step,
            role: PieceRole::Piece(k),
        });
        walk.continues.push(one_carrier && pieces >= 2);
    }
    TipState::Closed
}
