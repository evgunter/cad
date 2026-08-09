//! **The profile-program payload (LIB-SWITCH §4, PROFILES-V2 §§V1–V4):
//! the program IS the profile's definition.**
//!
//! [`ProfileProgram`] replaces the retired opaque `ProfileDesc` as
//! `Node::Profile`'s payload: plane placement (stored `f64` in its own
//! struct — the U4/VQ8 seam stays visible) plus one [`LoopProgram`] per
//! loop. A loop-program is the constructor-call sequence as data — the
//! chain vocabulary's steps with every CONTINUOUS argument an [`Expr`]
//! (V2's dimension table) and every STRUCTURAL argument a literal tag
//! (verb identity, winding, `Start`, the `circle_split` count).
//!
//! # The strict door (V1, wire.rs's rule at the program layer)
//!
//! Nothing here can mint a `profile::ProfileLoop`. Evaluation (and the
//! authoring-time check) RESOLVES the expressions at `f64` and hands
//! the resolved steps to `profile::replay` — the driver, hence the
//! typed binders and every check they carry. Deserialization rebuilds
//! THIS type (through `Expr`'s dimension doors); the only path from
//! steps to geometry runs through the driver.
//!
//! # f64 resolution (V2, the verified asymmetry)
//!
//! Program expressions resolve at **f64**, never at the evaluation
//! scalar: profile geometry feeds C6 structure selection (junction
//! classes, fillet fits), which must be decided once, identically for
//! every lane — exactly the stored-f64-bits behavior the retired
//! representation had. Node MAGNITUDE slots stay lane-live; the
//! asymmetry is inherited from `Doc::param_env`, not invented here.
//!
//! # Caches (V3)
//!
//! Replayed segments are the node's evaluated payload in the existing
//! per-node memo (`eval/mod.rs`'s prior-Evaluation reuse); NOTHING new
//! is persisted — the derived-value list in `persist`'s module docs
//! gains the segments. D9 makes the load-time rebuild exact.

use geom_core::Point2;
use profile::{ArcSweep, SketchPlane, Step};

use crate::expr::{Dimension, DimensionError, EvalError, Expr, ParamEnv, eval};
use crate::node::{SlotId, StepArg};

/// Where a target-taking step ends: an authored point (two Length
/// expressions) or the entry vertex (`Start` — structural; targeting it
/// closes the loop). Mirrors `profile::Target`.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramTarget {
    /// An authored absolute point in the profile frame.
    Point([Expr; 2]),
    /// The entry vertex: this step closes the loop.
    Start,
}

/// One Expr-bearing recorded verb — the document-layer mirror of
/// [`profile::Step`], structural tags literal, continuous args [`Expr`]
/// (V2's table: coordinates/lengths/radii `Length`, angle/turn/phase
/// `Angle`, bulge and director components `Scalar`).
///
/// Fields are public data (the node-slot pattern: dimensions are
/// checked at the edit door via [`ProfileProgram::slots`] +
/// [`StepArg::dimension`], and at the persistence doors' shared
/// validator — never trusted from a parsed file).
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramStep {
    /// `.at(p)`.
    At([Expr; 2]),
    /// **G2** `.at_on(p, centre, winding)`.
    AtOn {
        /// The anchor, on the carrier.
        p: [Expr; 2],
        /// The carrier's centre.
        centre: [Expr; 2],
        /// Travel sense (structural).
        winding: ArcSweep,
    },
    /// `.angle(θ)` (radians).
    Angle(Expr),
    /// **G1** `.toward(dx, dy)` — exact components, ratio-only.
    Toward {
        /// x component (Scalar).
        dx: Expr,
        /// y component (Scalar).
        dy: Expr,
    },
    /// `.tangent()` — structural, no arguments.
    Tangent,
    /// `.turn(δ)`.
    Turn(Expr),
    /// `line(len)`.
    Line(Expr),
    /// `line_to(target)`.
    LineTo(ProgramTarget),
    /// `arc_to(target, bulge)` — the bulge is AUTHORED data here.
    ArcTo {
        /// Where the leg ends.
        target: ProgramTarget,
        /// The authored bulge (M2 convention, Scalar).
        bulge: Expr,
    },
    /// `arc_via(via, target)` — bulge derived at replay.
    ArcVia {
        /// A point the arc passes through.
        via: [Expr; 2],
        /// Where the leg ends.
        target: ProgramTarget,
    },
    /// `arc_center(centre, target, winding)` — bulge derived at replay.
    ArcCenter {
        /// The arc's centre.
        centre: [Expr; 2],
        /// Where the leg ends.
        target: ProgramTarget,
        /// Travel sense (structural).
        winding: ArcSweep,
    },
    /// `tangent_arc_to(target)`.
    TangentArcTo(ProgramTarget),
    /// `arc_continue(target)` — the declared-subdivision step
    /// (LIB-SWITCH §5-1): a STRUCTURAL vertex on the incoming carrier.
    ArcContinue([Expr; 2]),
    /// `.fillet(r)`.
    Fillet(Expr),
    /// **G1** `.to(anchor)` — the far-end anchor.
    FarEndTo([Expr; 2]),
    /// `.to(Start)` — the seam-fillet close (structural).
    CloseTo,
    /// **G2** `.to_on(Start, centre, winding)`.
    CloseToOn {
        /// The arrival carrier's centre.
        centre: [Expr; 2],
        /// Travel sense (structural).
        winding: ArcSweep,
    },
}

/// One loop's program: a CHAIN step list, or one of the complete-loop
/// carrier forms (`circle` / `circle_split` — one-step programs whose
/// form is structural). The chain-vs-carrier distinction is the enum,
/// so "a circle program is exactly one step" is unrepresentable to
/// violate.
#[derive(Debug, Clone, PartialEq)]
pub enum LoopProgram {
    /// A chain-vocabulary step list (must end in a `Start`-targeting
    /// verb — checked by replay, not representation).
    Chain(Vec<ProgramStep>),
    /// `circle(centre, r)` — the seamless closed carrier.
    Circle {
        /// The circle's centre.
        centre: [Expr; 2],
        /// The circle's radius.
        radius: Expr,
    },
    /// `circle_split(centre, r, n, phase)` — the declared-subdivision
    /// closed carrier (corpus ruling (a); `n` STRUCTURAL).
    CircleSplit {
        /// The carrier's centre.
        centre: [Expr; 2],
        /// The carrier's radius.
        radius: Expr,
        /// The subdivision count (structural, ≥ 2 at replay).
        n: u32,
        /// The first vertex's angle from +x.
        phase: Expr,
    },
}

/// The profile node's payload: plane placement (stored `f64`, its own
/// struct — VQ8's visible seam) plus the loop programs, outer first
/// then holes in description order.
///
/// # Equality is BIT equality
///
/// `PartialEq` compares plane floats by BITS and expressions by
/// [`Expr::bit_eq`] — the canonical payload's equality IS the D7
/// replay-identity comparator, exactly the retired payload's contract
/// (`Node::bit_eq` inherits it). Display units are invisible to it
/// (they are invisible to `bit_eq` itself, D7).
#[derive(Debug, Clone)]
pub struct ProfileProgram {
    /// The sketch-plane placement (stored `f64`; Expr-izing placement
    /// is the U4 pose conversation, not this switch — VQ8).
    pub plane: SketchPlane<f64>,
    /// The loop programs.
    pub loops: Vec<LoopProgram>,
}

/// The canonical `Doc` instantiation (the retired `ProfileDesc` seat).
pub type ProfileDoc = crate::doc::Doc<ProfileProgram>;

// ------------------------------------------------------------------
// The payload trait (Node<P> genericity's seam)
// ------------------------------------------------------------------

/// What `Node<P>` needs from a profile payload so slot addressing and
/// the authoring-time check stay generic (`Doc<P>` keeps its fake test
/// payloads — the defaults are the slot-free, check-free behavior the
/// retired opaque payload had).
pub trait ProfilePayload {
    /// Every program expression slot, deterministic (loop, step, arg)
    /// order.
    fn slots(&self) -> Vec<SlotId> {
        Vec::new()
    }
    /// The expression a profile slot addresses, `None` off the program.
    fn expr(&self, _slot: SlotId) -> Option<&Expr> {
        None
    }
    /// Mutable twin of [`ProfilePayload::expr`].
    fn expr_mut(&mut self, _slot: SlotId) -> Option<&mut Expr> {
        None
    }
    /// The authoring-time check (VQ9): resolve + replay + validate
    /// under the CURRENT parameter environment, refusing typed at the
    /// edit door. The evaluation-time twin re-checks under every
    /// binding that is ever evaluated.
    fn check(&self, _env: &ParamEnv<f64>) -> Result<(), ProgramRefusal> {
        Ok(())
    }
}

/// A typed authoring-time program refusal (VQ9; `EditError`'s payload).
///
/// The resolve and validate classes carry their causes UNALTERED
/// (`EvalError`/`ProfileError` are `PartialEq`, as `EditError`
/// requires). The geometry-replay class cannot: `profile::PathError`
/// deliberately derives no equality, so [`ProgramRefusal::Geometry`]
/// carries the driver's rendered refusal plus the typed coordinates —
/// the full typed error remains the EVALUATION surface's contract
/// (`NodeErrorKind` carries it unaltered); the edit door is the early
/// ergonomic mirror. REPORTED shape, not silent (LIB-SWITCH §10).
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramRefusal {
    /// A program expression failed to resolve at f64 (unknown param,
    /// dimension drift, non-finite result).
    Resolve {
        /// The failing slot.
        slot: SlotId,
        /// The evaluator's refusal, unaltered.
        source: EvalError,
    },
    /// The resolved steps are not a legal lattice walk (corrupt or
    /// hand-built program — no recording surface produces this).
    Transition {
        /// The loop whose replay refused.
        loop_: u32,
        /// The offending step index (one past the end for an unclosed
        /// chain).
        step: u32,
        /// The tip's lattice state.
        state: profile::TipState,
        /// The ill-typed verb, `None` for end-of-program.
        verb: Option<profile::Verb>,
    },
    /// The chain is well-typed but the geometry refuses under the
    /// current binding (V1 class 2 — legal at rest; the CURRENT env
    /// refuses it at the door for fail-loud-early ergonomics).
    Geometry {
        /// The loop whose replay refused.
        loop_: u32,
        /// The offending step index.
        step: u32,
        /// The driver's rendered refusal (see enum docs for why this
        /// class is rendered here and typed at evaluation).
        rendered: String,
    },
    /// The replayed loops refused profile validation under the current
    /// binding (also V1 class 2).
    Validate(profile::ProfileError),
}

// ------------------------------------------------------------------
// Slot access
// ------------------------------------------------------------------

/// The argument roles a target contributes ([] for `Start`).
fn target_slots(t: &ProgramTarget, out: &mut Vec<StepArg>) {
    if let ProgramTarget::Point(_) = t {
        out.push(StepArg::TargetX);
        out.push(StepArg::TargetY);
    }
}

/// The argument roles of one chain step, enumeration order = the
/// step's own field order (deterministic; pinned by tests).
fn step_slots(step: &ProgramStep, out: &mut Vec<StepArg>) {
    use ProgramStep as P;
    use StepArg as A;
    match step {
        P::At(_) | P::FarEndTo(_) => out.extend([A::PointX, A::PointY]),
        P::AtOn { .. } => out.extend([A::PointX, A::PointY, A::CenterX, A::CenterY]),
        P::Angle(_) => out.push(A::AngleVal),
        P::Toward { .. } => out.extend([A::DirX, A::DirY]),
        P::Tangent | P::CloseTo => {}
        P::Turn(_) => out.push(A::TurnVal),
        P::Line(_) => out.push(A::Length),
        P::LineTo(t) | P::TangentArcTo(t) => target_slots(t, out),
        P::ArcContinue(_) => out.extend([A::TargetX, A::TargetY]),
        P::ArcTo { target, .. } => {
            target_slots(target, out);
            out.push(A::Bulge);
        }
        P::ArcVia { via: _, target } => {
            out.extend([A::ViaX, A::ViaY]);
            target_slots(target, out);
        }
        P::ArcCenter { target, .. } => {
            out.extend([A::CenterX, A::CenterY]);
            target_slots(target, out);
        }
        P::Fillet(_) => out.push(A::Radius),
        P::CloseToOn { .. } => out.extend([A::CenterX, A::CenterY]),
    }
}

/// Shared shape of [`step_expr`]/[`step_expr_mut`] — one table, two
/// borrows, via a macro so the (step, arg) pairing is written once.
macro_rules! step_arg_access {
    ($step:expr, $arg:expr, $($ref_kw:tt)*) => {{
        use ProgramStep as P;
        use StepArg as A;
        match ($step, $arg) {
            (P::At(p), A::PointX) | (P::FarEndTo(p), A::PointX) => Some($($ref_kw)* p[0]),
            (P::At(p), A::PointY) | (P::FarEndTo(p), A::PointY) => Some($($ref_kw)* p[1]),
            (P::ArcContinue(p), A::TargetX) => Some($($ref_kw)* p[0]),
            (P::ArcContinue(p), A::TargetY) => Some($($ref_kw)* p[1]),
            (P::AtOn { p, .. }, A::PointX) => Some($($ref_kw)* p[0]),
            (P::AtOn { p, .. }, A::PointY) => Some($($ref_kw)* p[1]),
            (P::AtOn { centre, .. }, A::CenterX)
            | (P::ArcCenter { centre, .. }, A::CenterX)
            | (P::CloseToOn { centre, .. }, A::CenterX) => Some($($ref_kw)* centre[0]),
            (P::AtOn { centre, .. }, A::CenterY)
            | (P::ArcCenter { centre, .. }, A::CenterY)
            | (P::CloseToOn { centre, .. }, A::CenterY) => Some($($ref_kw)* centre[1]),
            (P::Angle(e), A::AngleVal) => Some(e),
            (P::Toward { dx, .. }, A::DirX) => Some(dx),
            (P::Toward { dy, .. }, A::DirY) => Some(dy),
            (P::Turn(e), A::TurnVal) => Some(e),
            (P::Line(e), A::Length) => Some(e),
            (P::LineTo(ProgramTarget::Point(p)), A::TargetX)
            | (P::TangentArcTo(ProgramTarget::Point(p)), A::TargetX)
            | (P::ArcTo { target: ProgramTarget::Point(p), .. }, A::TargetX)
            | (P::ArcVia { target: ProgramTarget::Point(p), .. }, A::TargetX)
            | (P::ArcCenter { target: ProgramTarget::Point(p), .. }, A::TargetX) => {
                Some($($ref_kw)* p[0])
            }
            (P::LineTo(ProgramTarget::Point(p)), A::TargetY)
            | (P::TangentArcTo(ProgramTarget::Point(p)), A::TargetY)
            | (P::ArcTo { target: ProgramTarget::Point(p), .. }, A::TargetY)
            | (P::ArcVia { target: ProgramTarget::Point(p), .. }, A::TargetY)
            | (P::ArcCenter { target: ProgramTarget::Point(p), .. }, A::TargetY) => {
                Some($($ref_kw)* p[1])
            }
            (P::ArcTo { bulge, .. }, A::Bulge) => Some(bulge),
            (P::ArcVia { via, .. }, A::ViaX) => Some($($ref_kw)* via[0]),
            (P::ArcVia { via, .. }, A::ViaY) => Some($($ref_kw)* via[1]),
            (P::Fillet(e), A::Radius) => Some(e),
            _ => None,
        }
    }};
}

/// The expression a (step, arg) pair addresses.
fn step_expr(step: &ProgramStep, arg: StepArg) -> Option<&Expr> {
    step_arg_access!(step, arg, &)
}

/// Mutable twin of [`step_expr`].
fn step_expr_mut(step: &mut ProgramStep, arg: StepArg) -> Option<&mut Expr> {
    step_arg_access!(step, arg, &mut)
}

impl LoopProgram {
    /// This loop's argument roles per step, deterministic order.
    fn step_args(&self) -> Vec<(u32, StepArg)> {
        let mut out = Vec::new();
        match self {
            LoopProgram::Chain(steps) => {
                for (i, step) in steps.iter().enumerate() {
                    let mut args = Vec::new();
                    step_slots(step, &mut args);
                    out.extend(args.into_iter().map(|a| (i as u32, a)));
                }
            }
            LoopProgram::Circle { .. } => {
                out.extend([
                    (0, StepArg::CenterX),
                    (0, StepArg::CenterY),
                    (0, StepArg::Radius),
                ]);
            }
            LoopProgram::CircleSplit { .. } => {
                out.extend([
                    (0, StepArg::CenterX),
                    (0, StepArg::CenterY),
                    (0, StepArg::Radius),
                    (0, StepArg::Phase),
                ]);
            }
        }
        out
    }

    /// The expression at (step, arg), `None` off the loop.
    fn expr(&self, step: u32, arg: StepArg) -> Option<&Expr> {
        use StepArg as A;
        match self {
            LoopProgram::Chain(steps) => step_expr(steps.get(step as usize)?, arg),
            LoopProgram::Circle { centre, radius } if step == 0 => match arg {
                A::CenterX => Some(&centre[0]),
                A::CenterY => Some(&centre[1]),
                A::Radius => Some(radius),
                _ => None,
            },
            LoopProgram::CircleSplit {
                centre,
                radius,
                phase,
                ..
            } if step == 0 => match arg {
                A::CenterX => Some(&centre[0]),
                A::CenterY => Some(&centre[1]),
                A::Radius => Some(radius),
                A::Phase => Some(phase),
                _ => None,
            },
            _ => None,
        }
    }

    /// Mutable twin of [`LoopProgram::expr`].
    fn expr_mut(&mut self, step: u32, arg: StepArg) -> Option<&mut Expr> {
        use StepArg as A;
        match self {
            LoopProgram::Chain(steps) => step_expr_mut(steps.get_mut(step as usize)?, arg),
            LoopProgram::Circle { centre, radius } if step == 0 => match arg {
                A::CenterX => Some(&mut centre[0]),
                A::CenterY => Some(&mut centre[1]),
                A::Radius => Some(radius),
                _ => None,
            },
            LoopProgram::CircleSplit {
                centre,
                radius,
                phase,
                ..
            } if step == 0 => match arg {
                A::CenterX => Some(&mut centre[0]),
                A::CenterY => Some(&mut centre[1]),
                A::Radius => Some(radius),
                A::Phase => Some(phase),
                _ => None,
            },
            _ => None,
        }
    }
}

// ------------------------------------------------------------------
// Resolution (V2: at f64, the C6-pinned lane)
// ------------------------------------------------------------------

/// Resolves one expression at f64, tagging failures with the slot.
fn res(
    e: &Expr,
    env: &ParamEnv<f64>,
    loop_: u32,
    step: u32,
    arg: StepArg,
) -> Result<f64, (SlotId, EvalError)> {
    eval::<f64>(e, env).map_err(|source| (SlotId::Profile { loop_, step, arg }, source))
}

/// Resolves a target's expressions.
fn res_target(
    t: &ProgramTarget,
    env: &ParamEnv<f64>,
    loop_: u32,
    step: u32,
) -> Result<profile::Target<f64>, (SlotId, EvalError)> {
    Ok(match t {
        ProgramTarget::Start => profile::Target::Start,
        ProgramTarget::Point(p) => profile::Target::Point(Point2::new(
            res(&p[0], env, loop_, step, StepArg::TargetX)?,
            res(&p[1], env, loop_, step, StepArg::TargetY)?,
        )),
    })
}

/// Resolves one chain step to its scalar-valued mirror.
fn res_step(
    s: &ProgramStep,
    env: &ParamEnv<f64>,
    loop_: u32,
    i: u32,
) -> Result<Step<f64>, (SlotId, EvalError)> {
    use StepArg as A;
    let pt = |p: &[Expr; 2], ax: StepArg, ay: StepArg| -> Result<Point2<f64>, _> {
        Ok(Point2::new(
            res(&p[0], env, loop_, i, ax)?,
            res(&p[1], env, loop_, i, ay)?,
        ))
    };
    Ok(match s {
        ProgramStep::At(p) => Step::At(pt(p, A::PointX, A::PointY)?),
        ProgramStep::AtOn { p, centre, winding } => Step::AtOn {
            p: pt(p, A::PointX, A::PointY)?,
            centre: pt(centre, A::CenterX, A::CenterY)?,
            winding: *winding,
        },
        ProgramStep::Angle(e) => Step::Angle(res(e, env, loop_, i, A::AngleVal)?),
        ProgramStep::Toward { dx, dy } => Step::Toward {
            dx: res(dx, env, loop_, i, A::DirX)?,
            dy: res(dy, env, loop_, i, A::DirY)?,
        },
        ProgramStep::Tangent => Step::Tangent,
        ProgramStep::Turn(e) => Step::Turn(res(e, env, loop_, i, A::TurnVal)?),
        ProgramStep::Line(e) => Step::Line(res(e, env, loop_, i, A::Length)?),
        ProgramStep::LineTo(t) => Step::LineTo(res_target(t, env, loop_, i)?),
        ProgramStep::ArcTo { target, bulge } => Step::ArcTo {
            target: res_target(target, env, loop_, i)?,
            bulge: res(bulge, env, loop_, i, A::Bulge)?,
        },
        ProgramStep::ArcVia { via, target } => Step::ArcVia {
            via: pt(via, A::ViaX, A::ViaY)?,
            target: res_target(target, env, loop_, i)?,
        },
        ProgramStep::ArcCenter {
            centre,
            target,
            winding,
        } => Step::ArcCenter {
            centre: pt(centre, A::CenterX, A::CenterY)?,
            target: res_target(target, env, loop_, i)?,
            winding: *winding,
        },
        ProgramStep::TangentArcTo(t) => Step::TangentArcTo(res_target(t, env, loop_, i)?),
        ProgramStep::ArcContinue(p) => Step::ArcContinue(pt(p, A::TargetX, A::TargetY)?),
        ProgramStep::Fillet(e) => Step::Fillet {
            radius: res(e, env, loop_, i, A::Radius)?,
        },
        ProgramStep::FarEndTo(p) => Step::FarEndTo(pt(p, A::PointX, A::PointY)?),
        ProgramStep::CloseTo => Step::CloseTo,
        ProgramStep::CloseToOn { centre, winding } => Step::CloseToOn {
            centre: pt(centre, A::CenterX, A::CenterY)?,
            winding: *winding,
        },
    })
}

impl LoopProgram {
    /// Resolves this loop's program at f64 (module docs: the verified
    /// asymmetry — profile geometry is f64-pinned; C6 structure
    /// selection must be lane-identical).
    ///
    /// # Errors
    ///
    /// The failing slot plus the evaluator's refusal, unaltered.
    pub fn resolve(
        &self,
        env: &ParamEnv<f64>,
        loop_: u32,
    ) -> Result<Vec<Step<f64>>, (SlotId, EvalError)> {
        use StepArg as A;
        match self {
            LoopProgram::Chain(steps) => steps
                .iter()
                .enumerate()
                .map(|(i, s)| res_step(s, env, loop_, i as u32))
                .collect(),
            LoopProgram::Circle { centre, radius } => Ok(vec![Step::Circle {
                centre: Point2::new(
                    res(&centre[0], env, loop_, 0, A::CenterX)?,
                    res(&centre[1], env, loop_, 0, A::CenterY)?,
                ),
                radius: res(radius, env, loop_, 0, A::Radius)?,
            }]),
            LoopProgram::CircleSplit {
                centre,
                radius,
                n,
                phase,
            } => Ok(vec![Step::CircleSplit {
                centre: Point2::new(
                    res(&centre[0], env, loop_, 0, A::CenterX)?,
                    res(&centre[1], env, loop_, 0, A::CenterY)?,
                ),
                radius: res(radius, env, loop_, 0, A::Radius)?,
                n: *n as usize,
                phase: res(phase, env, loop_, 0, A::Phase)?,
            }]),
        }
    }
}

// ------------------------------------------------------------------
// ProfileProgram: resolution, equality, the payload impl
// ------------------------------------------------------------------

impl ProfileProgram {
    /// Resolves every loop at f64 — the evaluation pipeline's first
    /// stage (then `profile::replay` per loop, then embed + validate).
    ///
    /// # Errors
    ///
    /// The failing slot plus the evaluator's refusal, unaltered.
    pub fn resolve(&self, env: &ParamEnv<f64>) -> Result<Vec<Vec<Step<f64>>>, (SlotId, EvalError)> {
        self.loops
            .iter()
            .enumerate()
            .map(|(li, lp)| lp.resolve(env, li as u32))
            .collect()
    }

    /// The authoring-time check's body (VQ9): resolve under `env`,
    /// replay every loop, validate the assembled profile — all under
    /// the run tolerance (VQ6: the same `Tolerance::get()` evaluation
    /// pins). Used by the edit door; evaluation re-runs the same
    /// ladder per binding with full typed errors.
    pub fn check(&self, env: &ParamEnv<f64>) -> Result<(), ProgramRefusal> {
        let resolved = self
            .resolve(env)
            .map_err(|(slot, source)| ProgramRefusal::Resolve { slot, source })?;
        let mut loops = Vec::with_capacity(resolved.len());
        for (li, steps) in resolved.iter().enumerate() {
            let lp = profile::replay(steps).map_err(|e| match e.kind {
                profile::ReplayErrorKind::Transition { state, verb } => {
                    ProgramRefusal::Transition {
                        loop_: li as u32,
                        step: e.step as u32,
                        state,
                        verb,
                    }
                }
                profile::ReplayErrorKind::Path(ref source) => ProgramRefusal::Geometry {
                    loop_: li as u32,
                    step: e.step as u32,
                    rendered: source.to_string(),
                },
            })?;
            loops.push(lp);
        }
        profile::Profile::new(self.plane, loops)
            .validate(geom_core::Tolerance::get())
            .map(|_| ())
            .map_err(ProgramRefusal::Validate)
    }
}

impl PartialEq for ProfileProgram {
    /// BIT equality (struct docs): plane floats by bits, expressions by
    /// [`Expr::bit_eq`], structure structurally.
    fn eq(&self, other: &Self) -> bool {
        plane_bits(&self.plane) == plane_bits(&other.plane)
            && self.loops.len() == other.loops.len()
            && self
                .loops
                .iter()
                .zip(&other.loops)
                .all(|(a, b)| loop_bit_eq(a, b))
    }
}

/// The 12 placement floats as bits, deterministic column order — also
/// the content key's plane feed (crate-internal).
pub(crate) fn plane_key_bits(p: &SketchPlane<f64>) -> [u64; 12] {
    plane_bits(p)
}

/// The 12 placement floats, as bits.
fn plane_bits(p: &SketchPlane<f64>) -> [u64; 12] {
    let a = &p.placement;
    let mut out = [0u64; 12];
    for (i, v) in [a.linear.c0, a.linear.c1, a.linear.c2, a.translation]
        .iter()
        .enumerate()
    {
        out[3 * i] = v.x.to_bits();
        out[3 * i + 1] = v.y.to_bits();
        out[3 * i + 2] = v.z.to_bits();
    }
    out
}

/// Structural equality with Exprs compared by bits.
fn loop_bit_eq(a: &LoopProgram, b: &LoopProgram) -> bool {
    match (a, b) {
        (LoopProgram::Chain(x), LoopProgram::Chain(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(s, t)| step_bit_eq(s, t))
        }
        (
            LoopProgram::Circle {
                centre: ca,
                radius: ra,
            },
            LoopProgram::Circle {
                centre: cb,
                radius: rb,
            },
        ) => pair_bit_eq(ca, cb) && ra.bit_eq(rb),
        (
            LoopProgram::CircleSplit {
                centre: ca,
                radius: ra,
                n: na,
                phase: pa,
            },
            LoopProgram::CircleSplit {
                centre: cb,
                radius: rb,
                n: nb,
                phase: pb,
            },
        ) => pair_bit_eq(ca, cb) && ra.bit_eq(rb) && na == nb && pa.bit_eq(pb),
        _ => false,
    }
}

fn pair_bit_eq(a: &[Expr; 2], b: &[Expr; 2]) -> bool {
    a[0].bit_eq(&b[0]) && a[1].bit_eq(&b[1])
}

fn target_bit_eq(a: &ProgramTarget, b: &ProgramTarget) -> bool {
    match (a, b) {
        (ProgramTarget::Start, ProgramTarget::Start) => true,
        (ProgramTarget::Point(x), ProgramTarget::Point(y)) => pair_bit_eq(x, y),
        _ => false,
    }
}

fn step_bit_eq(a: &ProgramStep, b: &ProgramStep) -> bool {
    use ProgramStep as P;
    match (a, b) {
        (P::At(x), P::At(y)) | (P::FarEndTo(x), P::FarEndTo(y)) => pair_bit_eq(x, y),
        (
            P::AtOn {
                p: pa,
                centre: ca,
                winding: wa,
            },
            P::AtOn {
                p: pb,
                centre: cb,
                winding: wb,
            },
        ) => pair_bit_eq(pa, pb) && pair_bit_eq(ca, cb) && wa == wb,
        (P::Angle(x), P::Angle(y))
        | (P::Turn(x), P::Turn(y))
        | (P::Line(x), P::Line(y))
        | (P::Fillet(x), P::Fillet(y)) => x.bit_eq(y),
        (P::Toward { dx: xa, dy: ya }, P::Toward { dx: xb, dy: yb }) => {
            xa.bit_eq(xb) && ya.bit_eq(yb)
        }
        (P::Tangent, P::Tangent) | (P::CloseTo, P::CloseTo) => true,
        (P::LineTo(x), P::LineTo(y)) | (P::TangentArcTo(x), P::TangentArcTo(y)) => {
            target_bit_eq(x, y)
        }
        (P::ArcContinue(x), P::ArcContinue(y)) => pair_bit_eq(x, y),
        (
            P::ArcTo {
                target: ta,
                bulge: ba,
            },
            P::ArcTo {
                target: tb,
                bulge: bb,
            },
        ) => target_bit_eq(ta, tb) && ba.bit_eq(bb),
        (
            P::ArcVia {
                via: va,
                target: ta,
            },
            P::ArcVia {
                via: vb,
                target: tb,
            },
        ) => pair_bit_eq(va, vb) && target_bit_eq(ta, tb),
        (
            P::ArcCenter {
                centre: ca,
                target: ta,
                winding: wa,
            },
            P::ArcCenter {
                centre: cb,
                target: tb,
                winding: wb,
            },
        ) => pair_bit_eq(ca, cb) && target_bit_eq(ta, tb) && wa == wb,
        (
            P::CloseToOn {
                centre: ca,
                winding: wa,
            },
            P::CloseToOn {
                centre: cb,
                winding: wb,
            },
        ) => pair_bit_eq(ca, cb) && wa == wb,
        _ => false,
    }
}

impl ProfilePayload for ProfileProgram {
    fn slots(&self) -> Vec<SlotId> {
        let mut out = Vec::new();
        for (li, lp) in self.loops.iter().enumerate() {
            for (step, arg) in lp.step_args() {
                out.push(SlotId::Profile {
                    loop_: li as u32,
                    step,
                    arg,
                });
            }
        }
        out
    }

    fn expr(&self, slot: SlotId) -> Option<&Expr> {
        let SlotId::Profile { loop_, step, arg } = slot else {
            return None;
        };
        self.loops.get(loop_ as usize)?.expr(step, arg)
    }

    fn expr_mut(&mut self, slot: SlotId) -> Option<&mut Expr> {
        let SlotId::Profile { loop_, step, arg } = slot else {
            return None;
        };
        self.loops.get_mut(loop_ as usize)?.expr_mut(step, arg)
    }

    fn check(&self, env: &ParamEnv<f64>) -> Result<(), ProgramRefusal> {
        ProfileProgram::check(self, env)
    }
}

// ------------------------------------------------------------------
// Authoring helpers (VQ5: builder sugar expands AT AUTHORING into core
// steps — LineTo-class chains sharing Expr subtrees; nothing here adds
// program vocabulary)
// ------------------------------------------------------------------

/// A Length literal (canonical meters), for the literal-authoring
/// helpers below.
fn len_lit(v: f64) -> Result<Expr, DimensionError> {
    Expr::literal(v, Dimension::Length)
}

impl LoopProgram {
    /// A literal polygon: `At(p0)`, `LineTo(p1)`, …, `LineTo(Start)` —
    /// the VQ5 expansion of the polygon builder, at literal points
    /// (corpus/fixture authoring; parametric authors write the steps
    /// with their own Exprs).
    ///
    /// # Errors
    ///
    /// A non-finite coordinate (the literal door's refusal).
    pub fn polygon(points: impl IntoIterator<Item = (f64, f64)>) -> Result<Self, DimensionError> {
        let mut steps = Vec::new();
        for (i, (x, y)) in points.into_iter().enumerate() {
            let p = [len_lit(x)?, len_lit(y)?];
            steps.push(if i == 0 {
                ProgramStep::At(p)
            } else {
                ProgramStep::LineTo(ProgramTarget::Point(p))
            });
        }
        steps.push(ProgramStep::LineTo(ProgramTarget::Start));
        Ok(LoopProgram::Chain(steps))
    }

    /// A literal circle loop.
    ///
    /// # Errors
    ///
    /// A non-finite argument.
    pub fn circle(cx: f64, cy: f64, r: f64) -> Result<Self, DimensionError> {
        Ok(LoopProgram::Circle {
            centre: [len_lit(cx)?, len_lit(cy)?],
            radius: len_lit(r)?,
        })
    }

    /// A literal declared-subdivision circle loop.
    ///
    /// # Errors
    ///
    /// A non-finite argument.
    pub fn circle_split(
        cx: f64,
        cy: f64,
        r: f64,
        n: u32,
        phase: f64,
    ) -> Result<Self, DimensionError> {
        Ok(LoopProgram::CircleSplit {
            centre: [len_lit(cx)?, len_lit(cy)?],
            radius: len_lit(r)?,
            n,
            phase: Expr::literal(phase, Dimension::Angle)?,
        })
    }
}
