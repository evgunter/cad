//! **A chain's shape, with values erased**: the lattice walk over its
//! verbs, target forms and arc modes, and the pieces that structure can
//! draw.
//!
//! A replay ([`super::program::replay`]) answers both at one set of
//! values, and refuses where those values do; the shape answers what no
//! value moves. A step's numbers are never read: only its verb, the
//! form of each target ([`TargetKind`]), the mode of each arc spec
//! ([`ArcMode`]) and a `circle_split`'s written count.
//!
//! Legality is the transition table's ([`Verb::states`],
//! [`arc_specs_at`]); the state each row lands in, and what each row
//! emits, are stated per row below. That they are the replay's is held
//! cell by cell against it (`tests/chain_shape_census.rs`).

use super::program::{ArcData, ArcMode, Step, Target, TargetKind, TipState, Verb, arc_specs_at};
use crate::structure::{Piece, PieceRole};
use crate::{Real, Refused, Vocabulary};

/// **A step order the lattice refuses**: the tip was in `state` at
/// `step` and `verb` has no row there, or its arc spec a form the row
/// does not take (`None` for a chain that ends without closing, `step`
/// then one past its last).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LatticeFault {
    /// The step the walk stopped at.
    pub step: usize,
    /// The tip's lattice state there.
    pub state: TipState,
    /// The step's verb, `None` for end of program.
    pub verb: Option<Verb>,
}

impl LatticeFault {
    /// The one sentence for this pair ([`Refused`]).
    #[must_use]
    pub fn refused(self, vocabulary: Vocabulary) -> Refused {
        self.state.refused(self.verb, vocabulary)
    }
}

/// **What a closed chain's structure draws**: every piece some values
/// draw, in drawing order, one per segment the chain emits where every
/// fillet's runs fit with length to spare.
///
/// A piece absent here is absent at every value: its step's verb has
/// no such role, a `circle_split` count leaves it out, or another
/// piece takes its segment by construction — a fillet's run out takes
/// the first straight segment after the fillet, whichever step emits
/// it, and a fillet closing the chain at its seam draws no run out. A
/// piece present here may still go undrawn at some values: a fillet's
/// run in or run out that fits `Zero`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChainShape {
    /// The pieces, in the order the chain emits their segments.
    pub pieces: Vec<Piece>,
}

/// The kind of segment an emission is, which is all a run out's claim
/// on it reads.
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

#[derive(Default)]
struct Walk {
    pieces: Vec<Piece>,
    opened: Option<Opened>,
    run_out: Option<usize>,
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
}

fn kind_of<T: Real>(target: &Target<T>) -> TargetKind {
    target.kind()
}

/// Where a step aimed at `target` lands: on at a point, closed at the
/// start.
fn reaches<T: Real>(target: &Target<T>, on: TipState) -> TipState {
    match kind_of(target) {
        TargetKind::Point => on,
        TargetKind::Start | TargetKind::StartArriving => TipState::Closed,
    }
}

/// The state an arrival spec lands its fillet's chain in.
fn arrival_lands<T: Real>(spec: &ArcData<T>) -> TipState {
    let target = spec.target().map(Target::kind);
    match (spec.mode(), target) {
        (ArcMode::Radius, _) => TipState::RadiusArrival,
        (ArcMode::Via, Some(TargetKind::Point)) => TipState::ViaArrival,
        (ArcMode::Via, Some(TargetKind::Start)) => TipState::ViaArrivalStart,
        (_, Some(TargetKind::Point)) => TipState::DirectedPoint,
        _ => TipState::Closed,
    }
}

/// Whether the transition table takes `step` at `state`: a row for its
/// verb, and each arc spec a form that row's dispatcher admits.
fn admitted<T: Real>(state: TipState, step: &Step<T>) -> bool {
    let verb = step.verb();
    if !verb.states().contains(&state) {
        return false;
    }
    let specs: Vec<&ArcData<T>> = match step {
        Step::ArcTo(spec) | Step::FilletArc { spec, .. } | Step::ArcFillet { spec, .. } => {
            vec![spec]
        }
        Step::ArcFilletArc { spec, spec2, .. } => vec![spec, spec2],
        _ => Vec::new(),
    };
    let forms = arc_specs_at(verb, state);
    specs.iter().enumerate().all(|(i, spec)| {
        forms
            .get(i)
            .is_some_and(|f| f.admits(spec.mode(), spec.target().map(Target::kind)))
    })
}

/// **The chain `steps`, walked with values erased** (module docs).
///
/// # Errors
///
/// [`LatticeFault`] at the first step the table refuses, at a step
/// after the chain closed, or one past the end of a chain that does not
/// close — exactly where [`super::program::replay`] refuses as
/// [`super::program::ReplayErrorKind::Transition`] at any values that
/// reach that step.
pub fn chain_shape<T: Real>(steps: &[Step<T>]) -> Result<ChainShape, LatticeFault> {
    use TipState as S;
    let mut walk = Walk::default();
    let mut state = S::Entry;
    for (i, step) in steps.iter().enumerate() {
        let verb = step.verb();
        if state == S::Closed || !admitted(state, step) {
            return Err(LatticeFault {
                step: i,
                state,
                verb: Some(verb),
            });
        }
        state = match (state, step) {
            (S::Entry | S::Open, Step::At(_)) => S::PlainPoint,
            (S::Angle, Step::At(_)) => {
                walk.resolve(Arrival::Continues);
                S::DirectedPlain
            }
            (S::RadiusArrival, Step::At(_)) => S::RadiusArrivalAt,
            (S::RadiusArrivalDir, Step::At(_)) => S::DirectedPoint,
            (_, Step::At(_)) => unreachable!("`at` has a row at {state:?}: admitted above"),
            (s, Step::Angle(_) | Step::Toward { .. }) => match s {
                S::Entry | S::Open => S::Angle,
                S::PlainPoint => {
                    walk.resolve(Arrival::Continues);
                    S::DirectedPlain
                }
                S::DirectedPoint => S::DirectedIncoming,
                S::RadiusArrival => S::RadiusArrivalDir,
                S::ArcOntoStart | S::ViaArrivalStart => S::Closed,
                _ => S::DirectedPoint,
            },
            (_, Step::Tangent | Step::Cusp | Step::Turn(_)) => S::DirectedIncoming,
            (_, Step::Line(_)) => {
                walk.leg(i, Seg::Straight);
                S::DirectedPoint
            }
            (_, Step::LineTo(target) | Step::ContinueTo(target)) => {
                walk.resolve(Arrival::Continues);
                walk.leg(i, Seg::Straight);
                reaches(target, S::DirectedPoint)
            }
            (S::DirectedPlain | S::DirectedIncoming, Step::ArcTo(_)) => {
                walk.leg(i, Seg::Arc);
                S::DirectedPoint
            }
            (_, Step::ArcTo(spec)) => {
                walk.leg(i, Seg::Arc);
                spec.target()
                    .map_or(S::DirectedPoint, |t| reaches(t, S::DirectedPoint))
            }
            (_, Step::TangentArcTo(target)) => {
                walk.leg(i, Seg::Arc);
                reaches(target, S::DirectedPoint)
            }
            (_, Step::ArcOnto(target)) => {
                walk.leg(i, Seg::Arc);
                match kind_of(target) {
                    TargetKind::Point => S::ArcOnto,
                    TargetKind::Start => S::ArcOntoStart,
                    TargetKind::StartArriving => S::Closed,
                }
            }
            (_, Step::Fillet { .. }) => {
                walk.opened = Some(Opened {
                    step: i,
                    run_in: Seg::Straight,
                });
                S::Open
            }
            (_, Step::ArcFillet { .. }) => {
                walk.opened = Some(Opened {
                    step: i,
                    run_in: Seg::Arc,
                });
                S::Open
            }
            (_, Step::FilletArc { spec, .. }) => {
                walk.authored_arrival(i, Seg::Straight);
                arrival_lands(spec)
            }
            (_, Step::ArcFilletArc { spec2, .. }) => {
                walk.authored_arrival(i, Seg::Arc);
                arrival_lands(spec2)
            }
            (_, Step::FarEndTo(_)) => {
                walk.resolve(Arrival::Continues);
                walk.leg(i, Seg::Straight);
                S::DirectedPoint
            }
            (_, Step::CloseTo) => {
                walk.resolve(Arrival::Seam);
                S::Closed
            }
            (_, Step::Circle { .. } | Step::CircleSplit { .. } | Step::Rectangle { .. }) => {
                for k in 0..step.pieces() {
                    walk.pieces.push(Piece {
                        step: i,
                        role: PieceRole::Piece(k),
                    });
                }
                S::Closed
            }
        };
    }
    if state != S::Closed {
        return Err(LatticeFault {
            step: steps.len(),
            state,
            verb: None,
        });
    }
    Ok(ChainShape {
        pieces: walk.pieces,
    })
}
