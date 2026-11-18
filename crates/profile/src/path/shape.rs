//! **A chain's shape, with values erased**: the walk over its verbs,
//! target forms and arc modes, the refusals its step order earns at
//! every value, and the pieces that structure can draw.
//!
//! A replay ([`super::program::replay`]) answers all three at one set
//! of values, and refuses where those values do; the shape answers what
//! no value moves. Its input, [`StepForm`], holds no number: a step's
//! verb, the form of each target ([`TargetKind`]), the mode of each arc
//! spec ([`ArcMode`]) and a `circle_split`'s written count.
//!
//! Legality is the transition table's ([`Verb::states`],
//! [`arc_specs_at`]); the state each row lands in, what it emits, and
//! the structural facts the replay's order refusals read
//! ([`super::order`]) are stated per row below. That they are the
//! replay's is held cell by cell against it (`tests/arc_spec_census.rs`)
//! and over random chains (`tests/chain_shape_random.rs`).

use super::order::{self, OrderFault};
use super::program::{ArcData, ArcMode, Step, Target, TargetKind, TipState, Verb, arc_specs_at};
use crate::structure::{Piece, PieceRole, carrier_pieces};
use crate::{Real, Refused, Vocabulary};

/// **An arc spec, values erased**: its mode and its target's form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArcForm {
    /// The spec's mode.
    pub mode: ArcMode,
    /// Its target's form, `None` for the endpoint-free modes.
    pub target: Option<TargetKind>,
}

impl<T: Real> From<&ArcData<T>> for ArcForm {
    fn from(spec: &ArcData<T>) -> Self {
        Self {
            mode: spec.mode(),
            target: spec.target().map(Target::kind),
        }
    }
}

/// **One step, values erased**: what [`chain_shape`] reads, and all it
/// can.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepForm {
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
    LineTo(TargetKind),
    /// `continue_to`.
    ContinueTo(TargetKind),
    /// `arc_to`.
    ArcTo(ArcForm),
    /// `tangent_arc_to`.
    TangentArcTo(TargetKind),
    /// `arc_onto`.
    ArcOnto(TargetKind),
    /// `fillet`.
    Fillet,
    /// `fillet_arc`, and its arrival spec.
    FilletArc(ArcForm),
    /// `arc_fillet`, and its incoming spec.
    ArcFillet(ArcForm),
    /// `arc_fillet_arc`, its incoming spec and its arrival spec.
    ArcFilletArc(ArcForm, ArcForm),
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
}

impl StepForm {
    /// The verb this step names.
    #[must_use]
    pub const fn verb(self) -> Verb {
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
        }
    }

    /// Its arc specs, in the step's field order.
    fn specs(self) -> Vec<ArcForm> {
        match self {
            Self::ArcTo(a) | Self::FilletArc(a) | Self::ArcFillet(a) => vec![a],
            Self::ArcFilletArc(a, b) => vec![a, b],
            _ => Vec::new(),
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
/// takes the first straight segment after the fillet, whichever step
/// emits it, and a fillet closing the chain at its seam draws no run
/// out). A piece present is drawn at the values where every fillet's
/// runs fit with length to spare, and may go undrawn at others (a run
/// that fits `Zero`). Whether the chain replays at any value at all is
/// geometry's question, not the shape's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChainShape {
    /// The pieces, in the order the chain emits their segments.
    pub pieces: Vec<Piece>,
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

/// An opened fillet awaiting a straight arrival: its step and the kind
/// of its run in.
#[derive(Clone, Copy, Debug)]
struct Opened {
    step: usize,
    run_in: Seg,
}

/// How a straight arrival ends its fillet: riding on, so the run out
/// is the next straight segment (the far-end anchor's own included),
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
    first: Option<Seg>,
    opened: Option<Opened>,
    run_out: Option<usize>,
    /// Whether an `arc_onto(Start)` awaiting its director had a leg
    /// drawn before its own.
    onto_start_drawn: bool,
}

impl Walk {
    /// A segment `piece` would be, which a pending run out takes when
    /// it is straight.
    fn emit(&mut self, piece: Piece, seg: Seg) {
        let run_out = self.run_out.take();
        let piece = match run_out {
            Some(step) if seg == Seg::Straight && piece.role != PieceRole::Arc => Piece {
                step,
                role: PieceRole::RunOut,
            },
            _ => piece,
        };
        self.first.get_or_insert(seg);
        self.pieces.push(piece);
    }

    fn leg(&mut self, step: usize, seg: Seg) {
        self.emit(
            Piece {
                step,
                role: PieceRole::Leg,
            },
            seg,
        );
    }

    fn drawn(&self) -> bool {
        !self.pieces.is_empty()
    }

    /// The opened fillet's run in and arc, and its run out where the
    /// arrival rides on.
    fn resolve(&mut self, arrival: Arrival) {
        let Some(Opened { step, run_in }) = self.opened.take() else {
            return;
        };
        self.fillet_runs(step, run_in);
        if arrival == Arrival::Continues {
            self.run_out = Some(step);
        }
    }

    fn fillet_runs(&mut self, step: usize, run_in: Seg) {
        self.emit(
            Piece {
                step,
                role: PieceRole::RunIn,
            },
            run_in,
        );
        self.emit(
            Piece {
                step,
                role: PieceRole::Arc,
            },
            Seg::Arc,
        );
    }

    /// A fillet whose arrival arc the step authors: its three pieces,
    /// the run out that arc.
    fn authored_arrival(&mut self, step: usize, run_in: Seg) {
        self.fillet_runs(step, run_in);
        self.emit(
            Piece {
                step,
                role: PieceRole::RunOut,
            },
            Seg::Arc,
        );
    }

    /// A leg closing at `target` classifies the seam, which reads the
    /// first leg's arm; `StartArriving` on a leg that supplies its own
    /// end direction does not.
    fn classified_close(&self, target: TargetKind) -> Result<(), OrderFault> {
        match target {
            TargetKind::Point => Ok(()),
            TargetKind::Start | TargetKind::StartArriving => {
                order::seam_reads_a_first_leg(self.drawn())
            }
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
    match (spec.mode, spec.target) {
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
            .is_some_and(|f| f.admits(spec.mode, spec.target))
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
    use TipState as S;
    let mut walk = Walk::default();
    let mut entered = Vec::with_capacity(steps.len());
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
                S::DirectedPoint => S::DirectedIncoming,
                S::RadiusArrival => S::RadiusArrivalDir,
                S::ViaArrivalStart => S::Closed,
                S::ArcOntoStart => {
                    order::seam_reads_a_first_leg(walk.onto_start_drawn).map_err(order)?;
                    S::Closed
                }
                _ => S::DirectedPoint,
            },
            (_, StepForm::Tangent | StepForm::Cusp | StepForm::Turn) => S::DirectedIncoming,
            (_, StepForm::Line) => {
                walk.leg(i, Seg::Straight);
                S::DirectedPoint
            }
            (_, StepForm::LineTo(target) | StepForm::ContinueTo(target)) => {
                walk.resolve(Arrival::Continues);
                walk.classified_close(target).map_err(order)?;
                walk.leg(i, Seg::Straight);
                reaches(target, S::DirectedPoint)
            }
            (S::DirectedPlain | S::DirectedIncoming, StepForm::ArcTo(_)) => {
                walk.leg(i, Seg::Arc);
                S::DirectedPoint
            }
            (_, StepForm::ArcTo(spec)) => {
                let target = spec.target.unwrap_or(TargetKind::Point);
                let site = match target {
                    TargetKind::Point => order::site::ARC_TO,
                    TargetKind::Start | TargetKind::StartArriving => order::site::ARC_TO_START,
                };
                order::arc_leg_leaves_no_open_fillet(walk.opened.is_some(), site).map_err(order)?;
                walk.classified_close(target).map_err(order)?;
                walk.leg(i, Seg::Arc);
                reaches(target, S::DirectedPoint)
            }
            (_, StepForm::TangentArcTo(target)) => {
                walk.classified_close(target).map_err(order)?;
                walk.leg(i, Seg::Arc);
                reaches(target, S::DirectedPoint)
            }
            (_, StepForm::ArcOnto(target)) => {
                order::arc_leg_leaves_no_open_fillet(walk.opened.is_some(), order::site::ARC_ONTO)
                    .map_err(order)?;
                let drawn = walk.drawn();
                walk.onto_start_drawn = drawn;
                match target {
                    TargetKind::Point => {
                        walk.leg(i, Seg::Arc);
                        S::ArcOnto
                    }
                    TargetKind::Start => {
                        walk.leg(i, Seg::Arc);
                        S::ArcOntoStart
                    }
                    TargetKind::StartArriving => {
                        order::onto_start_leaves_its_entry(drawn).map_err(order)?;
                        walk.leg(i, Seg::Arc);
                        S::Closed
                    }
                }
            }
            (_, StepForm::Fillet) => {
                walk.opened = Some(Opened {
                    step: i,
                    run_in: Seg::Straight,
                });
                S::Open
            }
            (_, StepForm::ArcFillet(_)) => {
                fused_leaves_no_open_fillet(&walk, state).map_err(order)?;
                walk.opened = Some(Opened {
                    step: i,
                    run_in: Seg::Arc,
                });
                S::Open
            }
            (_, StepForm::FilletArc(spec)) => {
                walk.authored_arrival(i, Seg::Straight);
                arrival_lands(spec)
            }
            (_, StepForm::ArcFilletArc(_, spec2)) => {
                fused_leaves_no_open_fillet(&walk, state).map_err(order)?;
                walk.authored_arrival(i, Seg::Arc);
                arrival_lands(spec2)
            }
            (_, StepForm::FarEndTo) => {
                order::far_end_ends_an_open_fillet(walk.opened.is_some()).map_err(order)?;
                walk.resolve(Arrival::Continues);
                walk.leg(i, Seg::Straight);
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
                carrier(&mut walk, i, carrier_pieces(Verb::CircleSplit, n))
            }
            (_, StepForm::Circle | StepForm::Rectangle) => {
                carrier(&mut walk, i, carrier_pieces(step.verb(), 0))
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
    Ok(ChainShape {
        pieces: walk.pieces,
        entered,
    })
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
fn carrier(walk: &mut Walk, step: usize, pieces: u32) -> TipState {
    for k in 0..pieces {
        walk.pieces.push(Piece {
            step,
            role: PieceRole::Piece(k),
        });
    }
    TipState::Closed
}
