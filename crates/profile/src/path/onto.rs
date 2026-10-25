//! **`arc_onto(target)` — the arc known by how it ARRIVES** (PATHS-DESIGN
//! §2a item 6): `tangent_arc_to`'s mirror. The tip is a point, the target
//! names where the arc ends, and the ONE director that follows names its
//! end tangent; the departure is derived (the arrival reflected across the
//! chord) and junction-checked like any arc leg's start tangent. With
//! `Start.arrives_tangent()` as the target the token supplies the end
//! tangent — `Start`'s own direction — and the close completes at the verb.
//!
//! The geometry is [`super::onto_arc_geom`]: the tangent arc's kernel run
//! from the target backwards, so this verb's degeneracies are
//! `tangent_arc_to`'s, through the same gates.

use geom_core::{Decide, Point2, Real, Tol};

use super::program::{ClosedLoop, Step, Target};
use super::{
    ArrivesTangent, Core, Dir, FirstSeg, Flavor, HasPos, Incoming, NoAng, PartialPath, PathError,
    PosData, SeamArrival, Start, Tip, WithIncoming, in_state, leg_arm, leg_end_tip, onto_arc_geom,
    sealed, seam_joint,
};
use crate::structure::PieceRole;

/// An [`arc_onto`](PartialPath::arc_onto) target: an authored absolute
/// point or [`Start`], each then awaiting its one director, or
/// [`Start::arrives_tangent`], which supplies the director and closes.
/// Sealed.
pub trait OntoTarget<T: Decide, F: Flavor>: sealed::Sealed {
    /// The pending state awaiting the director, or the closed loop for
    /// the token.
    type Out;
    #[doc(hidden)]
    fn onto_from(path: PartialPath<T, HasPos<F>, NoAng>, target: Self, tol: Tol) -> Self::Out;
}

impl<T: Decide, F: Flavor> OntoTarget<T, F> for Point2<T> {
    type Out = Result<ArcOnto<T>, PathError<T>>;
    fn onto_from(mut path: PartialPath<T, HasPos<F>, NoAng>, target: Self, _tol: Tol) -> Self::Out {
        path.core.record(Step::ArcOnto(Target::Point(target)));
        refuse_open_fillet(&path.core)?;
        Ok(ArcOnto {
            bound_at: path.core.current_step()?,
            core: path.core,
            tip: path.tip,
            p: target,
        })
    }
}

impl<T: Decide, F: Flavor> OntoTarget<T, F> for Start {
    type Out = Result<ArcOntoStart<T>, PathError<T>>;
    fn onto_from(
        mut path: PartialPath<T, HasPos<F>, NoAng>,
        _target: Self,
        _tol: Tol,
    ) -> Self::Out {
        path.core.record(Step::ArcOnto(Target::Start));
        refuse_open_fillet(&path.core)?;
        Ok(ArcOntoStart {
            bound_at: path.core.current_step()?,
            core: path.core,
            tip: path.tip,
        })
    }
}

impl<T: Decide, F: Flavor> OntoTarget<T, F> for ArrivesTangent {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn onto_from(mut path: PartialPath<T, HasPos<F>, NoAng>, _target: Self, tol: Tol) -> Self::Out {
        path.core.record(Step::ArcOnto(Target::StartArriving));
        refuse_open_fillet(&path.core)?;
        let bound_at = path.core.current_step()?;
        close(path.core, &path.tip, bound_at, None, tol)
    }
}

/// A fillet arrival's bare anchor has no departure to leave from: its
/// arrival is authored WITH the fillet that trims it, as for `arc_to`.
fn refuse_open_fillet<T: Real>(core: &Core<T>) -> Result<(), PathError<T>> {
    if core.pending.is_some() {
        return Err(PathError::ArcLegOnOpenFillet {
            site: "an arc that arrives along a direction from a fillet's arrival anchor is the \
                   fillet's ARC ARRIVAL — fillet_arc(r, spec) / arc_fillet_arc(spec, r, spec2) \
                   — not an arc_onto leg from an already-positioned arrival point",
        });
    }
    Ok(())
}

/// `arc_onto(p)`: the target is bound and the ONE director that names
/// the arc's end tangent is pending.
#[derive(Clone, Debug)]
pub struct ArcOnto<T: Real> {
    core: Core<T>,
    /// The `arc_onto` step, which the leg is named for.
    bound_at: usize,
    tip: Tip<T>,
    p: Point2<T>,
}

/// `arc_onto(Start)`: the sharp seam, its arrival director pending.
#[derive(Clone, Debug)]
pub struct ArcOntoStart<T: Real> {
    core: Core<T>,
    /// The `arc_onto(Start)` step, which the leg is named for.
    bound_at: usize,
    tip: Tip<T>,
}

impl<T: Real> ArcOnto<T> {
    /// **The steps recorded so far, in program order** — the last is the
    /// `arc_onto` that opened this state.
    ///
    /// The claim is [`PartialPath::recorded`]'s, unchanged.
    #[must_use]
    pub fn recorded(&self) -> &[Step<T>] {
        &self.core.program
    }
}

impl<T: Real> ArcOntoStart<T> {
    /// **The steps recorded so far, in program order** — the last is the
    /// `arc_onto(Start)` that opened this state.
    ///
    /// The claim is [`PartialPath::recorded`]'s, unchanged.
    #[must_use]
    pub fn recorded(&self) -> &[Step<T>] {
        &self.core.program
    }
}

impl<T: Decide> ArcOnto<T> {
    /// The kernel behind the table's two director rows: `arrival` is
    /// the director the row read (the components door's refusal rides
    /// it, so the step is recorded first either way).
    pub(super) fn director_kernel(
        mut self,
        step: Step<T>,
        arrival: Result<Dir<T>, PathError<T>>,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        self.core.record(step);
        land(self.core, &self.tip, self.bound_at, self.p, arrival?, tol)
    }
}

impl<T: Decide> ArcOntoStart<T> {
    /// The kernel behind the table's two director rows, as
    /// [`ArcOnto::director_kernel`]'s.
    pub(super) fn director_kernel(
        mut self,
        step: Step<T>,
        arrival: Result<Dir<T>, PathError<T>>,
        tol: Tol,
    ) -> Result<ClosedLoop<T>, PathError<T>> {
        self.core.record(step);
        close(self.core, &self.tip, self.bound_at, Some(arrival?), tol)
    }
}

fn tip_pos<T: Real>(tip: &Tip<T>) -> Result<&PosData<T>, PathError<T>> {
    tip.pos.as_ref().ok_or(PathError::UnderdeterminedLeg {
        site: "point tip without a position",
    })
}

/// The derived departure's junction: checked against a leg end's
/// incoming tangent, and the entry direction where it is the first leg.
/// Returns the entry direction, bound by now either way.
fn depart<T: Decide>(
    core: &mut Core<T>,
    pos: &PosData<T>,
    departure: Dir<T>,
    tol: Tol,
) -> Result<Dir<T>, PathError<T>> {
    if let Some(inc) = &pos.incoming {
        core.pend_junction(inc, departure, tol)?;
    }
    Ok(*core.start_ang.get_or_insert(departure))
}

/// The arc lands as a directed point at `p`, incoming the authored
/// arrival and its departure free.
fn land<T: Decide>(
    mut core: Core<T>,
    tip: &Tip<T>,
    bound_at: usize,
    p: Point2<T>,
    arrival: Dir<T>,
    tol: Tol,
) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
    let pos = tip_pos(tip)?;
    let (g, departure) = onto_arc_geom(pos.at, arrival, p, tol)?;
    depart(&mut core, pos, departure, tol)?;
    core.claim(bound_at, PieceRole::Leg);
    core.push_leg(p, g.arc)?;
    let arm = leg_arm(g.carrier, g.chord);
    Ok(in_state(core, leg_end_tip(p, arrival, arm, g.carrier)))
}

/// The close at `Start`: `arrival` is the authored director (the sharp
/// seam, classified), or `None` where the token supplies `Start`'s own
/// direction and the seam's joint is constructed.
fn close<T: Decide>(
    mut core: Core<T>,
    tip: &Tip<T>,
    bound_at: usize,
    arrival: Option<Dir<T>>,
    tol: Tol,
) -> Result<ClosedLoop<T>, PathError<T>> {
    let start_pos = core.start_pos.ok_or(PathError::UnderdeterminedLeg {
        site: "close before the entry position is bound",
    })?;
    let pos = tip_pos(tip)?;
    let (how, arrival) = match (arrival, core.start_ang) {
        (Some(arrival), _) => (SeamArrival::Undeclared, arrival),
        (None, Some(start_ang)) => (SeamArrival::Supplied, start_ang),
        // `Start`'s direction is unbound only while the tip IS the
        // entry, so the chord the token would close is the zero one.
        (None, None) => {
            return Err(PathError::DegenerateArcChord {
                chord: (start_pos - pos.at).norm_squared().sqrt(),
            });
        }
    };
    let (g, departure) = onto_arc_geom(pos.at, arrival, start_pos, tol)?;
    let start_ang = depart(&mut core, pos, departure, tol)?;
    let arriving = Incoming::fresh(arrival, leg_arm(g.carrier, g.chord), g.carrier);
    seam_joint(&mut core, how, &arriving, start_ang, tol)?;
    core.claim(bound_at, PieceRole::Leg);
    core.close_leaving(g.arc, FirstSeg::Arc)?;
    core.build(tol)
}
