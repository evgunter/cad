//! **A profile drawn on a frame, reading the traces of faces** (D10,
//! "Constructing a coincidence"; `[variables.kinds]`).
//!
//! A trace is a face's carrier cut by the frame's `xy` plane, read in
//! closed form from the carrier the evaluation holds at its lane scalar
//! (`topo::boolean::trace`). It is never a variable and never stored:
//! every resolution reads it again, so a moved or resized traced body
//! moves the profile with it.
//!
//! Three document forms read one, and each lowers to a step of the
//! profile's own vocabulary, so the replay and everything after it see
//! an ordinary program:
//!
//! - a straight piece running to [`ProgramTarget::OnTrace`] lowers to
//!   `line(len)`, its length the distance along its heading to the
//!   crossing; [`ProgramTarget::AtX`] and [`ProgramTarget::AtY`] the same
//!   to a written sketch coordinate;
//! - an arc about a centre running to [`ProgramTarget::OnTrace`] lowers
//!   to `arc_to(Center)`, its endpoint the crossing;
//! - [`ProgramStep::AlongTrace`] lowers to `toward(dx, dy)`, the trace's
//!   tangent at the crossing the previous piece reached.
//!
//! A piece's pen and heading are the replay's own: the prefix before
//! the step is replayed ([`profile::pen`]), guided by the first pass's
//! record on a lane's second pass. A prefix that refuses is left to the
//! program's own replay, which refuses at the same step.
//!
//! A written coordinate reads no trace, so a free profile reaches one
//! too; what every such step reads is an evaluation's replay of its
//! prefix ([`Traces`]), and a resolution outside one refuses it
//! ([`TraceStepFault::Unevaluated`]).

use geom_core::{Band, Decide, Margin, Point2, Point3, RigidMotion, Sign, Tol, Vec2, Vec3};
use profile::{ArcSweep, Step};
use topo::boolean::trace::{self as kernel, Passage, TraceFault};

use crate::expr::VarEnv;
use crate::program::{
    ProgramArcData, ProgramResolveFault, ProgramStep, ProgramTarget, TraceCrossing, TracePassage,
    TraceSense, program_index,
};

/// The funnel row a piece reaching a written coordinate is decided on:
/// its heading's component across that coordinate's line, levered by
/// the distance to it, and then the distance along the heading.
pub const TRACE_REACHES: &str = "trace_reaches";

/// **One traced face, as the evaluation read it** at its lane scalar:
/// the carrier in its body's coordinates, the face's sense, and the
/// maps between the sketch's coordinates (the frame's) and the body's.
#[derive(Debug, Clone)]
pub struct TracedFace<T: geom_core::Real> {
    /// The face's carrier, in its body's coordinates, or the kind of a
    /// carrier with no closed form, which a step reading it refuses.
    pub surface: Result<geom::Surface<T>, geom::SurfaceKind>,
    /// The face's orientation sense.
    pub sense: bool,
    /// The sketch's coordinates into the body's.
    pub sketch_to_body: RigidMotion<T>,
    /// The body's coordinates into the sketch's.
    pub body_to_sketch: RigidMotion<T>,
}

/// **What an evaluation's resolution of a profile reads** for the steps
/// that lower against the pen: its traced faces, by place
/// ([`crate::DrawnOn::faces`]), where it is drawn on a frame; each
/// loop's replay record where a lane's second pass replays guided; and
/// the run's tolerance.
#[derive(Debug, Clone, Copy)]
pub struct Traces<'a, T: geom_core::Real> {
    /// The traced faces, or none for a profile drawn on no frame.
    pub faces: Option<&'a [TracedFace<T>]>,
    /// Each loop's record, on a guided pass.
    pub records: Option<&'a [profile::ReplayStructure]>,
    /// The run's tolerance, and its linear band.
    pub tol: Tol,
    /// The band every trace decision is read under.
    pub band: Band,
}

/// **Why a step reading a trace refused**, naming the step and, where
/// it reads one, the face.
#[derive(Debug, Clone, PartialEq)]
pub struct TraceRefusal {
    /// The loop.
    pub loop_: u32,
    /// The authored step.
    pub step: u32,
    /// The traced face, by place, where the step reads one.
    pub face: Option<u32>,
    /// Why.
    pub fault: TraceStepFault,
}

/// What [`TraceRefusal`] refuses.
#[derive(Debug, Clone, PartialEq)]
pub enum TraceStepFault {
    /// The step reads a trace, and the profile is drawn on no frame.
    Undrawn,
    /// The step lowers against the pen its prefix leaves (a trace, or a
    /// written coordinate along a heading), and was resolved outside an
    /// evaluation, which is the only resolution that replays the prefix
    /// (a form's preview, the pieces an edit door reads).
    Unevaluated,
    /// The step reads a face the profile does not trace.
    NoSuchFace {
        /// How many faces it traces.
        traced: usize,
    },
    /// The pen has no position here: the entry, or a direction bound
    /// first.
    NoPen,
    /// The piece has no heading to run along: a straight piece to a
    /// crossing runs along a director (`line_to`) or the incoming
    /// tangent (`continue_to`).
    NoHeading,
    /// The verb does not run along a heading or about a centre to a
    /// point it finds: a trace target in a tangent arc, an arc onto, a
    /// fillet's spec, or a coordinate target in an arc.
    NotARun,
    /// The step heads along a trace the previous piece did not run onto.
    OffTrace,
    /// The frame's plane misses the face's carrier: there is no trace.
    NoTrace,
    /// The piece makes fewer such crossings than the one written.
    NoSuchCrossing {
        /// The crossing written.
        crossing: TraceCrossing,
        /// How many crossings of that passage the piece makes.
        found: usize,
    },
    /// The piece runs onto the trace where it has no tangent: a cone's
    /// apex. (A node of the trace, or the plane tangent to the carrier
    /// there, is a crossing the piece grazes:
    /// [`TraceFault::Grazing`].)
    NoTangent,
    /// The heading never reaches the written coordinate, or reaches it
    /// behind the pen.
    Unreached,
    /// The carrier could not be read along the piece
    /// ([`TraceFault`]): no closed form, a grazing crossing, a sign in
    /// the sliver band.
    Carrier(TraceFault),
}

impl core::fmt::Display for TraceRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "loop {} step {}", self.loop_, self.step)?;
        if let Some(face) = self.face {
            write!(f, " (traced face {face})")?;
        }
        write!(f, ": {}", self.fault)
    }
}

impl core::error::Error for TraceRefusal {}

impl core::fmt::Display for TraceStepFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::sentence::Recourse;
        let recourse = match self {
            TraceStepFault::Undrawn => {
                f.write_str("reads a trace, and the profile is drawn on no frame")?;
                "draw the profile on a frame of the traced face's space"
            }
            TraceStepFault::Unevaluated => {
                f.write_str("lowers against the pen, which only an evaluation replays")?;
                "evaluate the document to read the step"
            }
            TraceStepFault::NoSuchFace { traced } => {
                write!(
                    f,
                    "reads a face the profile does not trace (it traces {traced})"
                )?;
                "add the face to the faces the profile traces, or read one it traces"
            }
            TraceStepFault::NoPen => {
                f.write_str("the pen has no position here")?;
                "start the chain at a point before it runs to a trace"
            }
            TraceStepFault::NoHeading => {
                f.write_str("the piece has no heading to run along")?;
                "bind a heading before `line_to`, or `continue_to` off a piece's end"
            }
            TraceStepFault::NotARun => {
                f.write_str("this verb does not run along a heading to a point it finds")?;
                "run to it with `line_to`, `continue_to`, or an arc about a centre"
            }
            TraceStepFault::OffTrace => {
                f.write_str("heads along a trace the previous piece did not run onto")?;
                "run the previous piece onto the trace first"
            }
            TraceStepFault::NoTrace => {
                f.write_str("the frame's plane misses the face")?;
                "move the frame so its plane cuts the face"
            }
            TraceStepFault::NoSuchCrossing { crossing, found } => {
                write!(
                    f,
                    "the piece {} the face {found} time(s), not {}",
                    crossing.passage.word(),
                    crossing.nth
                )?;
                "name a crossing the piece makes, or turn its heading"
            }
            TraceStepFault::NoTangent => {
                f.write_str("the trace has no tangent at the crossing")?;
                "run onto the trace at another crossing, or move the frame"
            }
            TraceStepFault::Unreached => {
                f.write_str("the heading never reaches the coordinate")?;
                "turn the heading toward the coordinate"
            }
            TraceStepFault::Carrier(TraceFault::Unsupported { carrier }) => {
                return write!(
                    f,
                    "a {} carrier has no closed-form trace. There is no way through yet",
                    carrier.name()
                );
            }
            // The escalation states its own recourse.
            TraceStepFault::Carrier(fault @ TraceFault::Escalated(_)) => {
                return write!(f, "{fault}");
            }
            TraceStepFault::Carrier(fault) => {
                write!(f, "{fault}")?;
                "turn the piece so it crosses the face, or move the frame"
            }
        };
        write!(f, ". {}", Recourse(recourse))
    }
}

impl core::error::Error for TraceStepFault {}

/// Whether `t` is a target that reads a face's trace.
fn on_trace<S>(t: &ProgramTarget<S>) -> bool {
    matches!(t, ProgramTarget::OnTrace { .. })
}

/// Whether `t` is a target found from the pen: a trace's crossing, or a
/// written coordinate along a heading.
fn from_the_pen<S>(t: &ProgramTarget<S>) -> bool {
    on_trace(t) || matches!(t, ProgramTarget::AtX(_) | ProgramTarget::AtY(_))
}

/// **Whether `step` reads a face's trace**: only a profile drawn on a
/// frame has one to read.
pub(crate) fn step_reads_a_trace<S>(step: &ProgramStep<S>) -> bool {
    matches!(step, ProgramStep::AlongTrace { .. }) || step_targets(step, on_trace)
}

/// **Whether `step` lowers against the pen its prefix leaves**: a trace,
/// or a written coordinate along a heading. Only an evaluation's
/// resolution replays the prefix ([`Traces`]).
pub(crate) fn step_runs_from_the_pen<S>(step: &ProgramStep<S>) -> bool {
    matches!(step, ProgramStep::AlongTrace { .. }) || step_targets(step, from_the_pen)
}

/// Whether a target of `step` is one `is` names.
fn step_targets<S>(step: &ProgramStep<S>, is: fn(&ProgramTarget<S>) -> bool) -> bool {
    let spec_reads = |spec: &ProgramArcData<S>| match spec {
        ProgramArcData::Bulge { target, .. }
        | ProgramArcData::Via { target, .. }
        | ProgramArcData::Center { target, .. } => is(target),
        ProgramArcData::Radius { .. }
        | ProgramArcData::Sweep { .. }
        | ProgramArcData::ArcLen { .. } => false,
    };
    match step {
        ProgramStep::LineTo(t)
        | ProgramStep::ContinueTo(t)
        | ProgramStep::TangentArcTo(t)
        | ProgramStep::ArcOnto(t) => is(t),
        ProgramStep::ArcTo(spec)
        | ProgramStep::FilletArc { spec, .. }
        | ProgramStep::ArcFillet { spec, .. } => spec_reads(spec),
        ProgramStep::ArcFilletArc { spec, spec2, .. } => spec_reads(spec) || spec_reads(spec2),
        ProgramStep::AlongTrace { .. }
        | ProgramStep::At(_)
        | ProgramStep::Angle(_)
        | ProgramStep::Toward { .. }
        | ProgramStep::Tangent
        | ProgramStep::Cusp
        | ProgramStep::Turn(_)
        | ProgramStep::Line(_)
        | ProgramStep::Fillet(_)
        | ProgramStep::FarEndTo(_)
        | ProgramStep::CloseTo => false,
    }
}

/// **One chain loop resolved, reading its traces** at the lane scalar.
/// Each step that reads no trace resolves as every step does; each that
/// reads one lowers as the module docs say.
///
/// # Errors
///
/// [`ProgramResolveFault::Expr`] for an expression that does not resolve, and
/// [`ProgramResolveFault::Trace`] for a step whose trace refuses.
pub(crate) fn resolve_chain<T: profile::ArcCarrierScalar>(
    steps: &[ProgramStep],
    env: &VarEnv<T>,
    loop_: u32,
    traces: Option<&Traces<'_, T>>,
) -> Result<Vec<Step<T>>, ProgramResolveFault> {
    let mut out: Vec<Step<T>> = Vec::with_capacity(steps.len());
    // The crossing the previous piece ran onto: its face, and the face's
    // outward gradient there, in the body's coordinates.
    let mut reached: Option<(u32, Vec3<T>)> = None;
    for (i, s) in steps.iter().enumerate() {
        let step = program_index(i);
        let ran_onto = reached.take();
        if !step_runs_from_the_pen(s) {
            out.push(crate::program::res_chain_step(s, env, loop_, step)?);
            continue;
        }
        let refuse = |face: Option<u32>, fault| {
            ProgramResolveFault::Trace(TraceRefusal {
                loop_,
                step,
                face,
                fault,
            })
        };
        let Some(traces) = traces else {
            return Err(refuse(None, TraceStepFault::Unevaluated));
        };
        let band = traces.band;
        let face = |at: u32| {
            let faces = traces
                .faces
                .ok_or_else(|| refuse(Some(at), TraceStepFault::Undrawn))?;
            faces.get(at as usize).ok_or_else(|| {
                refuse(
                    Some(at),
                    TraceStepFault::NoSuchFace {
                        traced: faces.len(),
                    },
                )
            })
        };
        // The pen after the prefix. A prefix that refuses is returned
        // as the resolution: `pen` replays exactly `out` under the same
        // record and tolerance the program's own replay runs (`replay`
        // or `replay_guided`), and the replay is sequential, so that
        // replay refuses at the same step with the same `ReplayError`,
        // typed as every profile's replay refusal is. Refusing here
        // instead would be a second, differently typed report of one
        // refusal (`ReplayError<T>` is the lane's, and this fault is
        // the document's).
        let record = traces.records.and_then(|r| r.get(loop_ as usize));
        let pen = match profile::pen(&out, record, traces.tol) {
            Ok(pen) => pen,
            Err(_) => return Ok(out),
        };
        let Some(pen) = pen else {
            return Err(refuse(None, TraceStepFault::NoPen));
        };
        let res = |e: &crate::VarId, arg: crate::StepArg| {
            crate::expr::eval_var::<T>(*e, arg.dimension(), env).map_err(|source| {
                ProgramResolveFault::Expr {
                    slot: crate::SlotId::Profile { loop_, step, arg },
                    source,
                }
            })
        };
        let lowered = match s {
            ProgramStep::AlongTrace { sense } => {
                let Some((at, outward)) = ran_onto else {
                    return Err(refuse(None, TraceStepFault::OffTrace));
                };
                let tangent = tangent(face(at)?, outward, *sense);
                Step::Toward {
                    dx: tangent.x,
                    dy: tangent.y,
                }
            }
            ProgramStep::LineTo(target) | ProgramStep::ContinueTo(target) => {
                // A director binds `line_to`'s heading; `continue_to`
                // runs along the incoming tangent.
                let wants_director = matches!(s, ProgramStep::LineTo(_));
                let heading = match pen.heading {
                    Some(h) if pen.directed == wants_director => h,
                    _ => return Err(refuse(None, TraceStepFault::NoHeading)),
                };
                let len = match target {
                    ProgramTarget::OnTrace { face: at, crossing } => {
                        let hit = ray_to(face(*at)?, pen.at, heading, *crossing, band)
                            .map_err(|fault| refuse(Some(*at), fault))?;
                        reached = Some((*at, hit.outward));
                        hit.along
                    }
                    ProgramTarget::AtX(x) => {
                        let x = res(x, crate::StepArg::TargetX)?;
                        reach(x - pen.at.x, heading.x, band).map_err(|f| refuse(None, f))?
                    }
                    ProgramTarget::AtY(y) => {
                        let y = res(y, crate::StepArg::TargetY)?;
                        reach(y - pen.at.y, heading.y, band).map_err(|f| refuse(None, f))?
                    }
                    ProgramTarget::Point(_)
                    | ProgramTarget::Start
                    | ProgramTarget::StartArriving => {
                        unreachable!("a step reading a trace reads one of the three forms")
                    }
                };
                Step::Line(len)
            }
            ProgramStep::ArcTo(ProgramArcData::Center {
                c,
                winding,
                target: ProgramTarget::OnTrace { face: at, crossing },
            }) => {
                let centre = Point2::new(
                    res(&c[0], crate::StepArg::CenterX)?,
                    res(&c[1], crate::StepArg::CenterY)?,
                );
                let (point, outward) =
                    arc_to(face(*at)?, pen.at, centre, *winding, *crossing, band)
                        .map_err(|fault| refuse(Some(*at), fault))?;
                reached = Some((*at, outward));
                Step::ArcTo(profile::ArcData::Center {
                    c: centre,
                    winding: *winding,
                    target: profile::Target::Point(point),
                })
            }
            _ => return Err(refuse(None, TraceStepFault::NotARun)),
        };
        out.push(lowered);
    }
    Ok(out)
}

fn lift<T: Decide>(p: Point2<T>) -> Point3<T> {
    Point3::new(p.x, p.y, T::zero())
}

fn lift_vec<T: Decide>(v: Vec2<T>) -> Vec3<T> {
    Vec3::new(v.x, v.y, T::zero())
}

/// The frame plane's point and normal, in the body's coordinates.
fn plane_in_body<T: Decide>(face: &TracedFace<T>) -> (Point3<T>, Vec3<T>) {
    (
        face.sketch_to_body.transform_point(Point3::origin()),
        face.sketch_to_body
            .transform_vec(Vec3::new(T::zero(), T::zero(), T::one())),
    )
}

/// A length the size of what a piece from `from` runs through: the lever
/// its decisions about `face` are read over.
fn reach_of<T: Decide>(face: &TracedFace<T>, surface: &geom::Surface<T>, from: Point3<T>) -> T {
    // A carrier with no closed form refused before any piece runs
    // through it (`traced`).
    let (anchor, size) = kernel::carrier_extent(surface, None).unwrap_or((from, T::zero()));
    (anchor - from).norm() + size + (from - plane_in_body(face).0).norm()
}

/// The face's carrier, where it has a closed form.
fn carrier<T: geom_core::Real>(face: &TracedFace<T>) -> Result<&geom::Surface<T>, TraceStepFault> {
    face.surface
        .as_ref()
        .map_err(|&carrier| TraceStepFault::Carrier(TraceFault::Unsupported { carrier }))
}

/// The face's carrier, refusing where the frame's plane misses it.
fn traced<T: Decide>(
    face: &TracedFace<T>,
    band: Band,
) -> Result<&geom::Surface<T>, TraceStepFault> {
    let surface = carrier(face)?;
    let (point, normal) = plane_in_body(face);
    match kernel::plane_misses(surface, point, normal, band) {
        Ok(true) => Err(TraceStepFault::NoTrace),
        Ok(false) => Ok(surface),
        Err(fault) => Err(TraceStepFault::Carrier(fault)),
    }
}

/// A carrier's refusal along a piece: a crossing at a cone's apex is a
/// point of the trace with no tangent.
fn carrier_fault(fault: TraceFault) -> TraceStepFault {
    match fault {
        TraceFault::Apex => TraceStepFault::NoTangent,
        fault => TraceStepFault::Carrier(fault),
    }
}

/// The written crossing among `crossings`, or how many there were.
fn pick<T: geom_core::Real>(
    crossings: Vec<kernel::Crossing<T>>,
    crossing: TraceCrossing,
) -> Result<kernel::Crossing<T>, TraceStepFault> {
    let passage = match crossing.passage {
        TracePassage::Enter => Passage::Enter,
        TracePassage::Leave => Passage::Leave,
    };
    let of: Vec<_> = crossings
        .into_iter()
        .filter(|c| c.passage == passage)
        .collect();
    let found = of.len();
    crossing
        .nth
        .checked_sub(1)
        .and_then(|k| of.get(k as usize).copied())
        .ok_or(TraceStepFault::NoSuchCrossing { crossing, found })
}

/// The written crossing along `heading` from `pen`: its length along the
/// heading, and the face's outward gradient there, in the body's
/// coordinates.
fn ray_to<T: Decide>(
    face: &TracedFace<T>,
    pen: Point2<T>,
    heading: Vec2<T>,
    crossing: TraceCrossing,
    band: Band,
) -> Result<kernel::Crossing<T>, TraceStepFault> {
    let surface = traced(face, band)?;
    let origin = face.sketch_to_body.transform_point(lift(pen));
    let dir = face.sketch_to_body.transform_vec(lift_vec(heading));
    let crossings = kernel::ray_crossings(
        surface,
        face.sense,
        origin,
        dir,
        reach_of(face, surface, origin),
        band,
    )
    .map_err(carrier_fault)?;
    pick(crossings, crossing)
}

/// The written crossing of the arc about `centre` from `pen`, in sketch
/// coordinates, and the face's outward gradient there, in the body's.
fn arc_to<T: Decide>(
    face: &TracedFace<T>,
    pen: Point2<T>,
    centre: Point2<T>,
    winding: ArcSweep,
    crossing: TraceCrossing,
    band: Band,
) -> Result<(Point2<T>, Vec3<T>), TraceStepFault> {
    let surface = traced(face, band)?;
    let radial = pen - centre;
    let radius = radial.norm();
    let up = match winding {
        ArcSweep::Ccw => T::one(),
        ArcSweep::Cw => T::zero() - T::one(),
    };
    let map = &face.sketch_to_body;
    let circle = (
        map.transform_point(lift(centre)),
        map.transform_vec(Vec3::new(T::zero(), T::zero(), up)),
        radius,
        map.transform_vec(lift_vec(radial / radius)),
    );
    let crossings =
        kernel::circle_crossings(surface, face.sense, circle, band).map_err(carrier_fault)?;
    let hit = pick(crossings, crossing)?;
    let at = face.body_to_sketch.transform_point(hit.point);
    Ok((Point2::new(at.x, at.y), hit.outward))
}

/// The trace's tangent at a crossing, in `sense`: the face's outward
/// gradient there (the crossing's own, [`kernel::Crossing::outward`]),
/// in the sketch's plane, turned a quarter turn the sketch's way
/// (material on the left), or against it. It is not normalized:
/// `toward` reads the components' ratio and normalizes once. The
/// crossing was decided transverse to a piece in the plane, so the
/// gradient's part in the plane is not zero: a point of the trace with
/// no tangent (a node, a cone's apex, the plane tangent to the carrier)
/// refused as the piece ran onto it.
fn tangent<T: Decide>(face: &TracedFace<T>, outward: Vec3<T>, sense: TraceSense) -> Vec2<T> {
    let normal = face.body_to_sketch.transform_vec(outward);
    let with = Vec2::new(T::zero() - normal.y, normal.x);
    match sense {
        TraceSense::With => with,
        TraceSense::Against => Vec2::new(T::zero() - with.x, T::zero() - with.y),
    }
}

/// **One point of a face's trace, as a display reads it**: in the
/// sketch's coordinates, with the trace's unit tangent there, the face's
/// material on its left ([`TraceSense::With`]).
#[derive(Debug, Clone, Copy)]
pub struct TraceMark {
    /// The point.
    pub at: Point2<f64>,
    /// The unit tangent, with the trace.
    pub tangent: Vec2<f64>,
}

/// **A face's trace sampled across a window** ([`trace_marks`]): the
/// points the window's rays cross it at, and how many rays the trace
/// door refused (a ray grazing the carrier, lying on it, through a
/// cone's apex, or in the sliver band), which draw no point.
#[derive(Debug, Clone)]
pub struct TraceMarks {
    /// The points, ray by ray.
    pub marks: Vec<TraceMark>,
    /// The rays refused.
    pub refused: u32,
}

/// **`face`'s trace, sampled for display** across the square of half
/// side `half` about `centre`, in the sketch's coordinates: `rays` rays
/// along the sketch's `x` and as many along its `y`, evenly spaced
/// across the square, each read against the carrier by the door every
/// step reading a trace runs through (`topo::boolean::trace`). A
/// display's points, never a step's: nothing reads them back.
///
/// # Errors
///
/// [`TraceStepFault::NoTrace`] where the frame's plane misses the
/// carrier, and [`TraceStepFault::Carrier`] for a carrier with no
/// closed form or a miss the band cannot call: the refusals a step
/// reading this trace would carry before any piece runs.
pub fn trace_marks(
    face: &TracedFace<f64>,
    centre: Point2<f64>,
    half: f64,
    rays: u32,
    band: Band,
) -> Result<TraceMarks, TraceStepFault> {
    let surface = traced(face, band)?;
    let mut out = TraceMarks {
        marks: Vec::new(),
        refused: 0,
    };
    let span = 2.0 * half;
    for i in 0..rays {
        let across = -half + span * (f64::from(i) + 0.5) / f64::from(rays);
        for (from, heading) in [
            (
                Point2::new(centre.x - half, centre.y + across),
                Vec2::new(1.0, 0.0),
            ),
            (
                Point2::new(centre.x + across, centre.y - half),
                Vec2::new(0.0, 1.0),
            ),
        ] {
            let origin = face.sketch_to_body.transform_point(lift(from));
            let dir = face.sketch_to_body.transform_vec(lift_vec(heading));
            let reach = reach_of(face, surface, origin);
            let Ok(crossings) =
                kernel::ray_crossings(surface, face.sense, origin, dir, reach, band)
            else {
                out.refused += 1;
                continue;
            };
            for hit in crossings.into_iter().filter(|c| c.along <= span) {
                let at = face.body_to_sketch.transform_point(hit.point);
                let with = tangent(face, hit.outward, TraceSense::With);
                out.marks.push(TraceMark {
                    at: Point2::new(at.x, at.y),
                    tangent: with / with.norm(),
                });
            }
        }
    }
    Ok(out)
}

/// The length along a heading whose component across a coordinate's
/// line is `rate` to the point `gap` along that coordinate.
fn reach<T: Decide>(gap: T, rate: T, band: Band) -> Result<T, TraceStepFault> {
    let escalated = |diag| TraceStepFault::Carrier(TraceFault::Escalated(diag));
    match geom_core::k_stats::decide(TRACE_REACHES, Margin::levered(rate, gap.abs()), band) {
        Ok(Sign::Positive | Sign::Negative) => {}
        Ok(Sign::Zero) => return Err(TraceStepFault::Unreached),
        Err(diag) => return Err(escalated(diag)),
    }
    let len = gap / rate;
    match geom_core::k_stats::decide(TRACE_REACHES, Margin::of(len), band) {
        Ok(Sign::Positive) => Ok(len),
        Ok(Sign::Zero | Sign::Negative) => Err(TraceStepFault::Unreached),
        Err(diag) => Err(escalated(diag)),
    }
}

/// The funnel row a sketch plane's squareness to a traced face's axis is
/// decided on: how far the picked plane turns off square across the
/// traced carrier's reach, and how far it stands off the frame's origin.
pub const TRACED_FRAME_SQUARE: &str = "traced_frame_square";

/// **Why no frame could be read for a sketch tracing a face**
/// ([`traced_frame`]).
#[derive(Debug)]
pub enum TracedFrameFault {
    /// A pick is not a face selection on one name.
    NotAFace,
    /// The picked plane's body could not be read.
    Body(crate::PinRefusal),
    /// The picked plane face has no vertex off the traced face's axis to
    /// stand the frame's origin by.
    NoVertex,
    /// A read refused as a profile drawn on the frame would: the traced
    /// face has no axis (a plane, a sphere), the picked face is gone.
    Read(Box<crate::NodeErrorKind>),
    /// The picked face is not a plane.
    NotAPlane {
        /// Its carrier's kind.
        carrier: geom::SurfaceKind,
    },
    /// The picked plane is not square to the traced face's axis.
    NotSquare,
    /// The band refused a decision.
    Escalated(geom_core::Indeterminate),
    /// The probe did not lower in the document.
    Lower(Box<crate::EditError>),
}

impl core::fmt::Display for TracedFrameFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::sentence::Recourse;
        let recourse = match self {
            Self::NotAFace => {
                f.write_str("a pick is not one face")?;
                "pick one face for each"
            }
            Self::Body(e) => return write!(f, "{e}"),
            Self::NoVertex => {
                f.write_str("the picked plane has no vertex off the traced face's axis")?;
                "pick a plane face with a corner or a seam"
            }
            Self::Read(e) => return write!(f, "{e}"),
            Self::NotAPlane { carrier } => {
                write!(
                    f,
                    "the picked face is on a {} carrier, not a plane",
                    carrier.name()
                )?;
                "pick a plane face square to the traced face's axis"
            }
            Self::NotSquare => {
                f.write_str("the picked plane is not square to the traced face's axis")?;
                "pick a plane face square to the traced face's axis"
            }
            Self::Escalated(diag) => return write!(f, "a squareness in the sliver band ({diag})"),
            Self::Lower(e) => return write!(f, "{e}"),
        };
        write!(f, ". {}", Recourse(recourse))
    }
}

impl core::error::Error for TracedFrameFault {}

/// The body and the one name of a face selection.
fn face_pick(
    pick: &crate::Operand,
) -> Result<(&crate::Operand, &crate::StableName), TracedFrameFault> {
    match pick {
        crate::Operand::Select { body, names } => match names.as_slice() {
            [name] => Ok((body, name)),
            _ => Err(TracedFrameFault::NotAFace),
        },
        _ => Err(TracedFrameFault::NotAFace),
    }
}

/// **The frame a sketch tracing `traced` stands on, in the plane of
/// `plane`**: `Through` the traced face's own axis and a vertex of the
/// picked plane face, its origin that vertex's foot on the axis, its `z`
/// the axis. Nothing in it is a number read off the evaluation, so every
/// evaluation reads it again: a moved or resized body moves the frame,
/// and a sweep of a profile drawn on it, along a wall the axis is
/// read off, is tangent to that wall by construction (the coincidence
/// door's `SameConstruction`).
///
/// The picked face is checked against `ev`, as a profile drawn on the
/// frame reads it: a plane, square to the axis, with a vertex off it.
/// `plane` and `traced` are face selections (`Operand::select` of one
/// name).
///
/// # Errors
///
/// [`TracedFrameFault`].
pub fn traced_frame(
    doc: &crate::ProfileDoc,
    ev: &crate::Evaluation<f64>,
    plane: &crate::Operand,
    traced: &crate::Operand,
    tol: Tol,
) -> Result<crate::Operand, TracedFrameFault> {
    use crate::names::{EntityKey, EntityRef};
    use crate::pose::{PoseDef, RoleRead};
    let (plane_body, plane_name) = face_pick(plane)?;
    face_pick(traced)?;
    let band = Band::linear(tol)
        .map_err(|e| TracedFrameFault::Read(Box::new(crate::NodeErrorKind::Band(e))))?;
    let (_, framed, table, _) =
        crate::mate::corner::read_of(doc, ev, plane_body).map_err(TracedFrameFault::Body)?;
    let solid = framed.body();
    let name = |key| table.name_of(&EntityRef { body: 0, key }).cloned();
    let Some((face, _)) = solid
        .faces()
        .find(|&(face, _)| name(EntityKey::Face(face)).as_ref() == Some(plane_name))
    else {
        return Err(TracedFrameFault::NotAFace);
    };
    let read = |def: PoseDef<crate::Operand, crate::Formula>| {
        RoleRead::plain(crate::Operand::Pose(Box::new(def)))
    };
    let mut last = TracedFrameFault::NoVertex;
    for (vertex, _) in solid.vertices() {
        if !solid
            .faces_of_vertex(vertex)
            .is_some_and(|faces| faces.contains(&face))
        {
            continue;
        }
        let Some(vertex_name) = name(EntityKey::Vertex(vertex)) else {
            continue;
        };
        let frame = crate::Operand::Pose(Box::new(PoseDef::Through {
            axis: read(PoseDef::Axis { of: traced.clone() }),
            point: read(PoseDef::Point {
                of: crate::Operand::select(plane_body.clone(), vec![vertex_name]),
            }),
        }));
        match square(doc, ev, &frame, plane, traced, band, tol) {
            Ok(()) => return Ok(frame),
            // A vertex on the axis has no foot to stand the origin by:
            // the next may.
            Err(fault @ TracedFrameFault::Read(_)) => last = fault,
            Err(fault) => return Err(fault),
        }
    }
    Err(last)
}

/// Whether `plane` is a plane square to `frame`'s `z` through its
/// origin, read as a profile drawn on `frame` tracing `plane` and
/// `traced` reads them.
fn square(
    doc: &crate::ProfileDoc,
    ev: &crate::Evaluation<f64>,
    frame: &crate::Operand,
    plane: &crate::Operand,
    traced: &crate::Operand,
    band: Band,
    tol: Tol,
) -> Result<(), TracedFrameFault> {
    let mut scratch = doc.clone();
    let probe = crate::Node::Profile(crate::program::ProfileProgram {
        loops: Vec::new(),
        ids: Vec::new(),
        on: Some(crate::DrawnOn {
            frame: frame.clone(),
            faces: vec![plane.clone(), traced.clone()],
        }),
    });
    let crate::Node::Profile(program) = crate::edit::lower_node_into(&mut scratch, &probe)
        .map_err(|e| TracedFrameFault::Lower(Box::new(e)))?
    else {
        unreachable!("a profile lowers to a profile")
    };
    let faces = crate::eval::traced_faces(&scratch, ev, &program, tol)
        .map_err(|e| TracedFrameFault::Read(Box::new(e)))?;
    let [picked, axial] = faces.as_slice() else {
        unreachable!("a probe tracing two faces reads two")
    };
    let (origin, normal) = match picked.surface {
        Ok(geom::Surface::Plane { origin, normal, .. }) => (origin, normal),
        Ok(ref s) => return Err(TracedFrameFault::NotAPlane { carrier: s.kind() }),
        Err(carrier) => return Err(TracedFrameFault::NotAPlane { carrier }),
    };
    let (at, z) = plane_in_body(picked);
    // The traced carrier's reach about the frame's origin: a cylinder's
    // radius, a cone's distance to its apex.
    let reach = axial.surface.as_ref().map_or(0.0, |s| {
        let at = axial.sketch_to_body.transform_point(Point3::origin());
        reach_of(axial, s, at)
    });
    let decide = |margin| {
        geom_core::k_stats::decide(TRACED_FRAME_SQUARE, margin, band)
            .map_err(TracedFrameFault::Escalated)
    };
    let tilt = decide(Margin::levered(normal.cross(z).norm(), reach))?;
    let off = decide(Margin::of((origin - at).dot(z)))?;
    match (tilt, off) {
        (Sign::Zero, Sign::Zero) => Ok(()),
        _ => Err(TracedFrameFault::NotSquare),
    }
}
