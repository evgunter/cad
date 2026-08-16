//! **RETIRED §2b doors, kept compiling for the consumer re-spell**
//! (LIB-RESPELL PR-2 deletes this file with its last callers).
//!
//! `at_on` / `to_on` / `at_toward` are the §2c family's split
//! spellings: each door re-arms the state the fused verbs would have
//! produced and drives EXACTLY the same kernel rows, so its geometry is
//! bit-identical to both the old register and the new family — and the
//! program it records is the NEW fused vocabulary (the old step forms
//! no longer exist). Every door is `#[doc(hidden)]`: this is compat
//! plumbing, not surface.

use geom_core::{Decide, Point2};

use super::arc_fillet::{self, ArcCarrierScalar, carrier_tangent};
use super::family::{resolve_arc_arrival, resolve_arc_close};
use super::program::{ArcData, ClosedLoop, Step, Target};
use super::verbs::{Pending, PendingArc};
use super::{
    Core, HasAng, HasPos, NoAng, NoPos, Open, PartialPath, PathError, Plain, PosData, Start, Tip,
    in_state, linear_band,
};
use crate::sugar::ArcSweep;

impl Open {
    /// RETIRED spelling of `Open.arc_fillet(Center { c, winding, p }, r)`
    /// (the fused entry verb): binds the entry ON a carrier; the
    /// `.fillet(r)` that follows completes the fused act.
    #[doc(hidden)]
    pub fn at_on<T: Decide>(
        self,
        p: Point2<T>,
        centre: Point2<T>,
        winding: ArcSweep,
    ) -> Result<PartialPath<T, HasPos<Plain>, HasAng>, PathError<T>> {
        let band = linear_band()?;
        let dir = carrier_tangent(p, centre, winding, band)?;
        let mut core = Core::empty();
        core.seed(p);
        core.start_ang = Some(dir);
        // Recording is DEFERRED to the fused step the following
        // `.fillet(r)` completes (the arrival door's program surgery).
        Ok(in_state(
            core,
            Tip {
                pos: Some(PosData {
                    at: p,
                    incoming: None,
                    on: Some((centre, winding)),
                }),
                ang: Some(dir),
                ang_by_tangent: false,
            },
        ))
    }
}

/// Re-arms a compat-carrier pending as the fused-verb state and
/// rewrites the recorded program to the fused vocabulary: the `Fillet`
/// step a compat `.fillet(r)` recorded becomes the `ArcFillet` step
/// the fused verb records.
///
/// The incoming spec is state-keyed exactly as the admissibility
/// matrix demands. At the ENTRY (the `Fillet` is the program's first
/// step, because the entry `at_on` defers recording) the spec is
/// `Center` — the authored centre, the admissible Entry row. MID-CHAIN
/// (the replayed state is OnArc) the spec is `Radius { r, side }`, the
/// one admissible OnArc row: `r = |anchor − centre|` is DERIVED here —
/// a bridge-only exception to record-authored-data, honest because the
/// author's old spelling authored the centre and this shim is deleted
/// with its callers (PR-2). Replay re-derives the centre from the tip's
/// bits, which can differ from the authored centre by ulps; nothing
/// pins compat replay bits (direct execution keeps the authored centre
/// below), and the program's own geometry is self-consistent.
fn re_arm<T: ArcCarrierScalar>(core: &mut Core<T>) -> Result<(), PathError<T>> {
    let meta = core
        .pending_meta
        .as_mut()
        .ok_or(PathError::OverdeterminedJunction {
            site: "retired arc-carrier door without an opened fillet",
        })?;
    let Some((centre, winding)) = meta.compat_carrier.take() else {
        return Ok(());
    };
    let Some(Pending::Ray(ray)) = core.pending.take() else {
        return Err(PathError::OverdeterminedJunction {
            site: "retired arc-carrier door on a non-ray pending",
        });
    };
    core.pending = Some(Pending::Arc(PendingArc {
        anchor: ray.origin,
        centre,
        winding,
        radius: ray.radius,
        resolver: arc_fillet::resolve::<T>,
    }));
    let entry = core.program.len() == 1;
    let spec = if entry {
        ArcData::Center {
            c: centre,
            winding,
            target: Target::Point(ray.origin),
        }
    } else {
        let v = ray.origin - centre;
        ArcData::Radius {
            r: v.norm_squared().sqrt(),
            side: match winding {
                ArcSweep::Ccw => super::verbs::ArcSide::Left,
                ArcSweep::Cw => super::verbs::ArcSide::Right,
            },
        }
    };
    match core.program.last_mut() {
        Some(step @ Step::Fillet { .. }) => {
            *step = Step::ArcFillet {
                spec,
                radius: ray.radius,
            };
            Ok(())
        }
        _ => Err(PathError::OverdeterminedJunction {
            site: "retired arc-carrier door without a recorded fillet step",
        }),
    }
}

/// Rewrites the fused step's ARRIVAL half in place: `Fillet` becomes
/// `FilletArc`, `ArcFillet` becomes `ArcFilletArc` — the program comes
/// out exactly as the fused verbs would have recorded it.
fn attach_arrival<T: Decide>(core: &mut Core<T>, spec2: ArcData<T>) -> Result<(), PathError<T>> {
    match core.program.last_mut() {
        Some(step) => match *step {
            Step::Fillet { radius } => {
                *step = Step::FilletArc {
                    radius,
                    spec: spec2,
                };
                Ok(())
            }
            Step::ArcFillet { spec, radius } => {
                *step = Step::ArcFilletArc {
                    spec,
                    radius,
                    spec2,
                };
                Ok(())
            }
            _ => Err(PathError::OverdeterminedJunction {
                site: "retired arc-carrier arrival without a recorded fillet step",
            }),
        },
        None => Err(PathError::OverdeterminedJunction {
            site: "retired arc-carrier arrival without a recorded fillet step",
        }),
    }
}

impl<T: ArcCarrierScalar> PartialPath<T, NoPos, NoAng> {
    /// RETIRED spelling of `fillet_arc(r, Center { c, winding, p })` /
    /// `arc_fillet_arc(…, Center { c, winding, p })`: the arc-carrier
    /// fillet arrival.
    #[doc(hidden)]
    pub fn at_on(
        mut self,
        p: Point2<T>,
        centre: Point2<T>,
        winding: ArcSweep,
    ) -> Result<PartialPath<T, HasPos<Plain>, HasAng>, PathError<T>> {
        re_arm(&mut self.core)?;
        attach_arrival(
            &mut self.core,
            ArcData::Center {
                c: centre,
                winding,
                target: Target::Point(p),
            },
        )?;
        let dir =
            resolve_arc_arrival(&mut self.core, arc_fillet::resolve::<T>, p, centre, winding)?;
        Ok(in_state(
            self.core,
            Tip {
                pos: Some(PosData {
                    at: p,
                    incoming: None,
                    on: Some((centre, winding)),
                }),
                ang: Some(dir),
                ang_by_tangent: false,
            },
        ))
    }

    /// RETIRED spelling of `arc_fillet(…, r).at(p).toward(dx, dy)`: the
    /// straight arrival off an arc departure (LB10 route 3). Its
    /// straight-departure fence survives with it.
    #[doc(hidden)]
    pub fn at_toward(
        mut self,
        p: Point2<T>,
        dx: T,
        dy: T,
    ) -> Result<PartialPath<T, HasPos<Plain>, HasAng>, PathError<T>> {
        let straight = matches!(self.core.pending, Some(Pending::Ray(_)))
            && self
                .core
                .pending_meta
                .as_ref()
                .is_none_or(|m| m.compat_carrier.is_none());
        if straight {
            return Err(PathError::ArcCarrierSpelling {
                site: "a fillet departing on a STRAIGHT side binds a straight arrival with \
                       .at(p).toward(dx, dy) (or .at(p).angle(θ)) — .at_toward is the \
                       arc-departure door",
            });
        }
        re_arm(&mut self.core)?;
        self.at(p)?.toward(dx, dy)
    }

    /// RETIRED spelling of `fillet_arc(r, Center { c, winding, p: Start })`
    /// / `arc_fillet_arc(…, Center { …, p: Start })`: the arc-carrier
    /// close that keeps the entry vertex.
    #[doc(hidden)]
    pub fn to_on(
        mut self,
        target: Start,
        centre: Point2<T>,
        winding: ArcSweep,
    ) -> Result<ClosedLoop<T>, PathError<T>> {
        let Start = target;
        re_arm(&mut self.core)?;
        attach_arrival(
            &mut self.core,
            ArcData::Center {
                c: centre,
                winding,
                target: Target::Start,
            },
        )?;
        resolve_arc_close(&mut self.core, arc_fillet::resolve::<T>, centre, winding)
    }
}
