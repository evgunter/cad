//! **The v1 → program lift** (PROFILES-V2 §V5, LIB-SWITCH §7).
//!
//! A development-side authoring tool: it takes a v1-form
//! [`ProfileLoop`] — vertices, bulges, and the declared-tangent joint
//! set — and mints an equivalent chain- (or carrier-) vocabulary
//! program. It is **not a load path and never runs at load** (LQ7a's
//! clean break: v1 documents refuse `SchemaTooOld`; nothing in the
//! persistence door reaches this module).
//!
//! # What the declared flags can pin
//!
//! PATHS-DESIGN's harmonization paragraph says the v1 flags are what
//! make the lift well-defined: "declared junctions become `.tangent()`
//! calls, fillet-authored arcs become `.fillet(r)`". Measured against
//! the actual v1 form, only the first half is a flag read:
//! [`ProfileLoop::tangent_joints`] is the ONLY declared datum, and a
//! fillet leaves no marker of its own — it is exactly *an arc whose
//! two joints are both declared*, which is also what a hand-declared
//! tangent arc looks like. Recovering `.fillet(r)` would mean
//! un-trimming the corner (inference, not a flag read), and the
//! reconstruction is anchor-sensitive in precisely the way finding F10
//! describes. This tool therefore spells every declared junction
//! `.tangent()` and leaves the fillet spelling banked; the census
//! below counts the fillet-shaped loops it meets so the cost is
//! measured rather than assumed.
//!
//! # Two refusal layers, deliberately
//!
//! Mirroring [`ReplayErrorKind`]'s own split:
//!
//! - **Structural** walls are this tool's: a loop the chain vocabulary
//!   has no shape for at all ([`LiftRefusal`]).
//! - **Geometric** walls are the DRIVER's. The lift does not
//!   re-implement a single predicate; it spells the natural program and
//!   lets the binders refuse. A same-carrier junction, a tangent-line
//!   close, a nonpositive radius — all of them arrive as the driver's
//!   own typed [`ReplayError`] through
//!   [`LiftOutcome::ReplayRefused`], so the wall of record is the
//!   binder's, not a guess about it.
//!
//! # The seam
//!
//! A chain binds its entry with `.at(p)`, which declares nothing, and
//! the two closers that DO declare the seam joint (`.to(Start)`,
//! `.to_on(Start, …)`) retrim vertex 0. So a loop whose joint 0 is
//! declared cannot be lifted at that seam. Since a loop is cyclic and
//! the seam is authoring freedom, the lift ROTATES to the first
//! undeclared joint and reports the rotation it used; the differential
//! comparison is against the correspondingly rotated source, which is
//! pure reindexing (no arithmetic, so bit-exactness is preserved). A
//! loop with NO undeclared joint — a fully filleted outline — has no
//! seam to author at and refuses [`LiftRefusal::AllJointsDeclared`].
//!
//! # Directors
//!
//! The lift emits **no director at all** in the chain form: `.at(p)`
//! lands on a plain point, from which `line_to`/`arc_to` bind position
//! and direction together from AUTHORED points. That is VQ4/W1's
//! "prefer chord-derived spellings" taken to its limit — no `.angle(θ)`
//! is ever minted, so the `sin_cos` quantization class cannot enter a
//! lifted program. `.toward` is likewise unnecessary here (it earns its
//! place at authoring time, where a direction is what the author
//! means).

use geom_core::{Point2, Vec2};

use crate::ProfileLoop;
use crate::path::PathError;
use crate::path::program::{ReplayError, ReplayErrorKind, Step, Target, replay};

/// How faithfully a lifted program reproduces its source loop, up to
/// the reported seam rotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fidelity {
    /// Every vertex coordinate and bulge matches bit for bit, and the
    /// declared-joint sets agree.
    BitIdentical,
    /// The shape agrees but some DERIVED value differs in its last
    /// bits — the F10/W1 classes of PROFILES-V2 §V5. The lifted program
    /// changes what is SAID, not what is drawn.
    ValueEqual,
}

/// A structural wall: a loop the chain vocabulary has no shape for.
///
/// Geometric walls are NOT here — those are the driver's refusals,
/// surfaced verbatim through [`LiftOutcome::ReplayRefused`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiftRefusal {
    /// Fewer than two vertices: there is no loop to lift.
    TooFewVertices {
        /// How many the loop carried.
        vertices: usize,
    },
    /// A coordinate or bulge is not finite. Authored data must be
    /// real numbers before any spelling question arises.
    NonFinite {
        /// The offending vertex index.
        vertex: usize,
    },
    /// A declared-joint index does not name a vertex of this loop
    /// (the validator's `TangentJointOutOfRange`, met earlier).
    JointIndexOutOfRange {
        /// The offending index.
        joint: usize,
        /// How many vertices the loop has.
        vertices: usize,
    },
    /// **Every** joint is declared tangent — a fully filleted outline.
    /// There is no sharp joint to seam the chain at, and `.at(p)`
    /// cannot declare. The spelling that would work is the seam fillet
    /// (`.fillet(r).…to(Start)`), which retrims vertex 0 and so needs
    /// the un-trimming this tool does not do.
    AllJointsDeclared {
        /// How many joints the loop has (all of them declared).
        joints: usize,
    },
    /// A declared joint whose LEAVING segment is straight AND which
    /// closes the loop. After `.tangent()` the only straight verb is
    /// `.line(len)`, and `.line` never closes — it always lands on a
    /// directed point.
    DeclaredJointBeforeClosingLine {
        /// The declared joint's vertex index in the SOURCE loop.
        joint: usize,
    },
    /// A same-carrier arc run reaches the SEAM. `arc_continue` has no
    /// closing form (it mints a structural subdivision vertex mid-chain
    /// only), and closing with `arc_to(Start)` on the incoming carrier
    /// is the `SameCarrierJunction` refusal.
    SameCarrierClose {
        /// The joint's vertex index in the SOURCE loop.
        joint: usize,
    },
}

impl std::fmt::Display for LiftRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewVertices { vertices } => {
                write!(f, "a loop needs at least two vertices; found {vertices}")
            }
            Self::NonFinite { vertex } => {
                write!(
                    f,
                    "vertex {vertex} carries a non-finite coordinate or bulge"
                )
            }
            Self::JointIndexOutOfRange { joint, vertices } => write!(
                f,
                "declared joint {joint} is out of range for a {vertices}-vertex loop"
            ),
            Self::AllJointsDeclared { joints } => write!(
                f,
                "all {joints} joints are declared tangent: no sharp seam to author the chain at"
            ),
            Self::DeclaredJointBeforeClosingLine { joint } => write!(
                f,
                "joint {joint} is declared and its leaving segment closes the loop straight; \
                 .tangent().line(len) cannot close"
            ),
            Self::SameCarrierClose { joint } => write!(
                f,
                "the same-carrier arc run at joint {joint} reaches the seam; arc_continue has \
                 no closing form"
            ),
        }
    }
}

impl std::error::Error for LiftRefusal {}

/// One loop's census row: what a lift attempt produced.
///
/// This is the acceptance instrument PROFILES-V2 §V5 asks for — "each
/// new binding mode turns some refusals into lifts, measurably". A run
/// over a corpus tallies these.
#[derive(Clone, Debug)]
pub enum LiftOutcome {
    /// Lifted, and replay reproduces the source loop exactly (up to
    /// `rotation`).
    Lifted {
        /// The minted program.
        program: Vec<Step<f64>>,
        /// How far the chain's seam was rotated from the source's
        /// vertex 0 (0 for the carrier forms, which author no seam).
        rotation: usize,
        /// Bit-identical, or value-equal with derived bits shifted.
        fidelity: Fidelity,
        /// The largest ulp distance over all compared values (0 iff
        /// [`Fidelity::BitIdentical`]).
        worst_ulps: u64,
    },
    /// A structural wall: the chain vocabulary has no shape for it.
    Refused(LiftRefusal),
    /// A GEOMETRIC wall: the spelling is well-formed but a binder
    /// refused it. The driver's own typed error is the wall of record
    /// (§5-1's same-carrier class lands here).
    ReplayRefused {
        /// The program that was tried.
        program: Vec<Step<f64>>,
        /// The seam rotation that program used.
        rotation: usize,
        /// The driver's refusal, unaltered.
        error: ReplayError<f64>,
    },
    /// The lifted program replayed to a DIFFERENT loop. A defect in
    /// this tool — surfaced, never silently counted as a lift.
    Mismatch {
        /// The program that was tried.
        program: Vec<Step<f64>>,
        /// The seam rotation that program used.
        rotation: usize,
        /// How far off the worst compared value was, in ulps
        /// ([`u64::MAX`] when the shapes do not even correspond).
        worst_ulps: u64,
    },
}

/// The largest ulp gap still called [`Fidelity::ValueEqual`] rather
/// than a mismatch. Generous by design: the F10/W1 classes shift
/// derived values by a handful of ulps, but those shifts propagate
/// through trim closed forms with their own lever arms.
const VALUE_EQUAL_ULPS: u64 = 1 << 20;

/// Absolute floor beneath which ulp distance stops being meaningful
/// (values straddling zero are ulp-far and value-near).
const VALUE_EQUAL_ABS: f64 = 1e-12;

/// **The lift**: mint a program for a v1-form loop.
///
/// Returns the program in the chain or carrier vocabulary. Structural
/// walls refuse here; GEOMETRIC walls do not — a returned program is
/// well-shaped, not guaranteed to replay (see the module docs' two
/// layers, and [`lift_checked`], which is the instrument you almost
/// certainly want).
///
/// # Errors
///
/// [`LiftRefusal`], naming the structural wall.
pub fn lift(loop_: &ProfileLoop<f64>) -> Result<Vec<Step<f64>>, LiftRefusal> {
    lift_seamed(loop_).map(|(program, _)| program)
}

/// **The differential harness**: lift, replay, and compare against the
/// source loop.
///
/// Total by construction — every path produces a census row rather than
/// an error the caller must interpret.
pub fn lift_checked(loop_: &ProfileLoop<f64>) -> LiftOutcome {
    let (program, rotation) = match lift_seamed(loop_) {
        Ok(pair) => pair,
        Err(refusal) => return LiftOutcome::Refused(refusal),
    };
    let replayed = match replay(&program) {
        Ok(l) => l,
        Err(error) => {
            return LiftOutcome::ReplayRefused {
                program,
                rotation,
                error,
            };
        }
    };
    let want = rotated(loop_, rotation);
    match compare(&want, &replayed) {
        Comparison::Equal { worst_ulps } => LiftOutcome::Lifted {
            program,
            rotation,
            fidelity: if worst_ulps == 0 {
                Fidelity::BitIdentical
            } else {
                Fidelity::ValueEqual
            },
            worst_ulps,
        },
        Comparison::Differs { worst_ulps } => LiftOutcome::Mismatch {
            program,
            rotation,
            worst_ulps,
        },
    }
}

// ------------------------------------------------------------------
// Minting
// ------------------------------------------------------------------

/// The lift proper: the program AND the seam rotation it authored at.
fn lift_seamed(loop_: &ProfileLoop<f64>) -> Result<(Vec<Step<f64>>, usize), LiftRefusal> {
    let n = loop_.vertices.len();
    if n < 2 {
        return Err(LiftRefusal::TooFewVertices { vertices: n });
    }
    for (i, v) in loop_.vertices.iter().enumerate() {
        if !(v.pos.x.is_finite() && v.pos.y.is_finite() && v.bulge.is_finite()) {
            return Err(LiftRefusal::NonFinite { vertex: i });
        }
    }
    let mut declared = vec![false; n];
    for &j in &loop_.tangent_joints {
        match declared.get_mut(j) {
            Some(slot) => *slot = true,
            None => {
                return Err(LiftRefusal::JointIndexOutOfRange {
                    joint: j,
                    vertices: n,
                });
            }
        }
    }

    // The closed-carrier forms first: a loop that IS a carrier has no
    // seam to author, and `circle`/`circle_split` say so in one step.
    if !declared.iter().any(|d| *d)
        && let Some(program) = carrier_form(loop_)
    {
        return Ok((program, 0));
    }

    let rotation = match declared.iter().position(|d| !*d) {
        Some(r) => r,
        None => return Err(LiftRefusal::AllJointsDeclared { joints: n }),
    };
    chain_form(loop_, &declared, rotation).map(|program| (program, rotation))
}

/// Try the one-step carrier spellings, VERIFYING each by replay rather
/// than by re-deriving a carrier-identity predicate.
fn carrier_form(loop_: &ProfileLoop<f64>) -> Option<Vec<Step<f64>>> {
    let n = loop_.vertices.len();
    let a = loop_.vertices.first()?;
    let b = loop_.vertices.get(1 % n)?;
    let (centre, radius) = arc_carrier(a.pos, b.pos, a.bulge)?;
    let phase = (a.pos.y - centre.y).atan2(a.pos.x - centre.x);

    let mut candidates = Vec::with_capacity(2);
    if n == 2 {
        candidates.push(vec![Step::Circle { centre, radius }]);
    }
    candidates.push(vec![Step::CircleSplit {
        centre,
        radius,
        n,
        phase,
    }]);

    candidates.into_iter().find(|program| {
        replay(program)
            .is_ok_and(|replayed| matches!(compare(loop_, &replayed), Comparison::Equal { .. }))
    })
}

/// The chain spelling, seamed at `rotation`, with the same-carrier
/// repair driven by the DRIVER's own refusals.
fn chain_form(
    loop_: &ProfileLoop<f64>,
    declared: &[bool],
    rotation: usize,
) -> Result<Vec<Step<f64>>, LiftRefusal> {
    let n = loop_.vertices.len();
    let at = |k: usize| loop_.vertices[(rotation + k) % n];
    // Which SOURCE segment each step belongs to, for refusal reporting.
    let mut origin = vec![rotation];
    let mut program = vec![Step::At(at(0).pos)];

    for k in 0..n {
        let src = (rotation + k) % n;
        let here = at(k);
        let target = if k + 1 == n {
            Target::Start
        } else {
            Target::Point(at(k + 1).pos)
        };
        if k > 0 && declared[src] {
            origin.push(src);
            program.push(Step::Tangent);
            if here.bulge == 0.0 {
                if k + 1 == n {
                    return Err(LiftRefusal::DeclaredJointBeforeClosingLine { joint: src });
                }
                origin.push(src);
                program.push(Step::Line(here.pos.distance(at(k + 1).pos)));
            } else {
                origin.push(src);
                program.push(Step::TangentArcTo(target));
            }
        } else {
            origin.push(src);
            program.push(if here.bulge == 0.0 {
                Step::LineTo(target)
            } else {
                Step::ArcTo {
                    target,
                    bulge: here.bulge,
                }
            });
        }
    }

    repair_same_carrier(program, &origin)
}

/// Turn `arc_to` into `arc_continue` wherever the DRIVER says the
/// junction is carrier identity (§5-1's class, met by the binder's own
/// refusal rather than by a re-derived predicate).
fn repair_same_carrier(
    mut program: Vec<Step<f64>>,
    origin: &[usize],
) -> Result<Vec<Step<f64>>, LiftRefusal> {
    // Each pass converts at least one step, so the program's length
    // bounds the number of passes.
    for _ in 0..=program.len() {
        let error = match replay(&program) {
            Ok(_) => return Ok(program),
            Err(e) => e,
        };
        if !matches!(
            error.kind,
            ReplayErrorKind::Path(PathError::SameCarrierJunction { .. })
        ) {
            // Some other geometric wall: leave it for the census to
            // record with the driver's own words.
            return Ok(program);
        }
        match program.get(error.step) {
            Some(Step::ArcTo {
                target: Target::Point(p),
                ..
            }) => {
                let p = *p;
                if let Some(slot) = program.get_mut(error.step) {
                    *slot = Step::ArcContinue(p);
                }
            }
            Some(Step::ArcTo {
                target: Target::Start,
                ..
            }) => {
                return Err(LiftRefusal::SameCarrierClose {
                    joint: origin.get(error.step).copied().unwrap_or_default(),
                });
            }
            _ => return Ok(program),
        }
    }
    Ok(program)
}

// ------------------------------------------------------------------
// Comparison
// ------------------------------------------------------------------

/// The source loop re-seamed at `rotation` — pure reindexing, so every
/// stored bit survives.
fn rotated(loop_: &ProfileLoop<f64>, rotation: usize) -> ProfileLoop<f64> {
    let n = loop_.vertices.len();
    if rotation == 0 || n == 0 {
        return loop_.clone();
    }
    let r = rotation % n;
    ProfileLoop {
        vertices: (0..n).map(|k| loop_.vertices[(r + k) % n]).collect(),
        tangent_joints: loop_
            .tangent_joints
            .iter()
            .map(|&j| (j % n + n - r) % n)
            .collect(),
    }
}

/// The differential verdict for one loop pair.
enum Comparison {
    /// The same shape, `worst_ulps` = 0 iff bit for bit.
    Equal {
        /// Largest ulp gap seen.
        worst_ulps: u64,
    },
    /// Not the same shape.
    Differs {
        /// Largest ulp gap seen.
        worst_ulps: u64,
    },
}

fn compare(want: &ProfileLoop<f64>, got: &ProfileLoop<f64>) -> Comparison {
    if want.vertices.len() != got.vertices.len() {
        return Comparison::Differs {
            worst_ulps: u64::MAX,
        };
    }
    let joints = |l: &ProfileLoop<f64>| {
        let mut js = l.tangent_joints.clone();
        js.sort_unstable();
        js.dedup();
        js
    };
    if joints(want) != joints(got) {
        return Comparison::Differs {
            worst_ulps: u64::MAX,
        };
    }
    let mut worst = 0u64;
    let mut equal = true;
    for (w, g) in want.vertices.iter().zip(got.vertices.iter()) {
        for (x, y) in [(w.pos.x, g.pos.x), (w.pos.y, g.pos.y), (w.bulge, g.bulge)] {
            let gap = ulps(x, y);
            worst = worst.max(gap);
            if gap > VALUE_EQUAL_ULPS && (x - y).abs() > VALUE_EQUAL_ABS {
                equal = false;
            }
        }
    }
    if equal {
        Comparison::Equal { worst_ulps: worst }
    } else {
        Comparison::Differs { worst_ulps: worst }
    }
}

/// Distance in representable doubles, via the standard monotone
/// total-order key (no casts, no overflow).
fn ulps(a: f64, b: f64) -> u64 {
    if a == b {
        return 0;
    }
    if !(a.is_finite() && b.is_finite()) {
        return u64::MAX;
    }
    let key = |x: f64| {
        let bits = x.to_bits();
        if bits & (1u64 << 63) == 0 {
            bits | (1u64 << 63)
        } else {
            !bits
        }
    };
    key(a).abs_diff(key(b))
}

/// The carrier circle a chord and its bulge imply (the crate docs'
/// closed form, arithmetic only — no predicate is fired, so this adds
/// no call site to the `k_stats` funnel).
fn arc_carrier(a: Point2<f64>, b: Point2<f64>, bulge: f64) -> Option<(Point2<f64>, f64)> {
    if bulge == 0.0 || !bulge.is_finite() {
        return None;
    }
    let chord = b - a;
    let len = chord.norm_squared().sqrt();
    if len <= 0.0 || !len.is_finite() {
        return None;
    }
    let unit = chord / len;
    let normal = Vec2::new(-unit.y, unit.x);
    let b2 = bulge * bulge;
    let four_b = 4.0 * bulge;
    let mid = a.lerp(b, 0.5);
    Some((
        mid + normal * (len * (1.0 - b2) / four_b),
        (len * (1.0 + b2) / four_b).abs(),
    ))
}
