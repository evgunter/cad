//! **The v1 → program lift** (PROFILES-V2 §V5, LIB-SWITCH §7).
//!
//! A development-side authoring tool: it takes a v1-form
//! [`ProfileLoop`] — vertices, bulges, and the declared-tangent joint
//! set — and mints an equivalent chain- (or carrier-) vocabulary
//! program. It is **not a load path and never runs at load** (LQ7a's
//! clean break: a v1-form document predates the `id:` header line, so
//! the persistence door refuses it `PersistError::HeaderId` with the
//! regenerate recourse and never reads its body; nothing in that door
//! reaches this module).
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
//! the arc-arrival close) retrim vertex 0. So a loop whose joint 0 is
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
use geom_core::Tol;

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
        /// How far the seam was rotated from the source's vertex 0.
        rotation: usize,
        /// Bit-identical, or value-equal with derived bits shifted.
        fidelity: Fidelity,
        /// The largest ulp distance over all compared values. NOT a
        /// restatement of `fidelity`: a pair straddling zero (0 against
        /// sin(pi)) is ulp-far and value-near, so this reads huge while
        /// `worst_abs` reads ~1e-16.
        worst_ulps: u64,
        /// The largest absolute difference over all compared values.
        worst_abs: f64,
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
        /// The largest absolute difference over all compared values.
        worst_abs: f64,
    },
}

/// The largest ulp gap still called [`Fidelity::ValueEqual`] rather
/// than a mismatch.
///
/// This is the RELATIVE criterion: an ulp count is magnitude-scaled by
/// construction, so 2^12 bounds the disagreement at ~9.1e-13 of the
/// value's own size. Headroom over the measured worst — the corpus's
/// largest genuine shift is the bracket's 2 ulps, and even a trim
/// closed form's lever arm keeps its propagation in that neighbourhood
/// — while staying well inside the "last bits moved" claim the class
/// makes. It was 2^20 at PR-open, which admitted ~2.3e-10 m at
/// magnitude 1: generous enough that a real defect could have been
/// censused as value-equal. Tightened deliberately.
const VALUE_EQUAL_ULPS: u64 = 1 << 12;

/// Absolute floor beneath which ulp distance stops being meaningful.
///
/// This is the ABSOLUTE criterion, disjunctive with the relative one:
/// values straddling zero (0 against sin(pi)) are ulp-far and
/// value-near, and no relative measure can see that.
const VALUE_EQUAL_ABS: f64 = 1e-12;

// Neither threshold is the honesty backstop. Both are the coarse
// LiftOutcome bucketing; the tool's actual accuracy claim is pinned
// independently by the census suite, which asserts each value-equal
// row's `worst_abs` against its own tight ceiling (<1e-12 for the
// bracket, <1e-15 for the carrier-phase residue). A regression that
// stayed inside these constants but left those ceilings would fail
// there.

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
pub fn lift(loop_: &ProfileLoop<f64>, tol: Tol) -> Result<Vec<Step<f64>>, LiftRefusal> {
    lift_seamed(loop_, tol).map(|(program, _)| program)
}

/// **The differential harness**: lift, replay, and compare against the
/// source loop.
///
/// Total by construction — every path produces a census row rather than
/// an error the caller must interpret.
pub fn lift_checked(loop_: &ProfileLoop<f64>, tol: Tol) -> LiftOutcome {
    let (program, rotation) = match lift_seamed(loop_, tol) {
        Ok(pair) => pair,
        Err(refusal) => return LiftOutcome::Refused(refusal),
    };
    let replayed = match replay(&program, tol) {
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
    let verdict = compare(&want, &replayed);
    if verdict.equal {
        LiftOutcome::Lifted {
            program,
            rotation,
            fidelity: if verdict.bit_identical {
                Fidelity::BitIdentical
            } else {
                Fidelity::ValueEqual
            },
            worst_ulps: verdict.worst_ulps,
            worst_abs: verdict.worst_abs,
        }
    } else {
        LiftOutcome::Mismatch {
            program,
            rotation,
            worst_ulps: verdict.worst_ulps,
            worst_abs: verdict.worst_abs,
        }
    }
}

// ------------------------------------------------------------------
// Minting
// ------------------------------------------------------------------

/// The lift proper: the program AND the seam rotation it authored at.
fn lift_seamed(loop_: &ProfileLoop<f64>, tol: Tol) -> Result<(Vec<Step<f64>>, usize), LiftRefusal> {
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
        && let Some(found) = carrier_form(loop_, tol)
    {
        return Ok(found);
    }

    let rotation = match declared.iter().position(|d| !*d) {
        Some(r) => r,
        None => return Err(LiftRefusal::AllJointsDeclared { joints: n }),
    };
    chain_form(loop_, &declared, rotation, tol).map(|program| (program, rotation))
}

/// Try the one-step carrier spellings, VERIFYING each by replay rather
/// than by re-deriving a carrier-identity predicate.
///
/// The seam is searched, not assumed: `circle` fixes its own seam at
/// the +x pole and `circle_split` at `phase`, so a hand-authored carrier
/// loop generally corresponds to one of them ROTATED. Returns the
/// program and the rotation it matched at, preferring an exact match.
fn carrier_form(loop_: &ProfileLoop<f64>, tol: Tol) -> Option<(Vec<Step<f64>>, usize)> {
    let n = loop_.vertices.len();
    // Only a loop that is arcs all the way round can be one carrier;
    // this guard keeps the search off every polygon.
    if n < 2 || loop_.vertices.iter().any(|v| v.bulge == 0.0) {
        return None;
    }
    let mut best: Option<(Vec<Step<f64>>, usize, u64)> = None;
    for r in 0..n {
        let a = loop_.vertices[r];
        let b = loop_.vertices[(r + 1) % n];
        let Some((centre, radius)) = arc_carrier(a.pos, b.pos, a.bulge) else {
            continue;
        };
        let phase = (a.pos.y - centre.y).atan2(a.pos.x - centre.x);
        let want = rotated(loop_, r);
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
        for program in candidates {
            let Ok(replayed) = replay(&program, tol) else {
                continue;
            };
            let verdict = compare(&want, &replayed);
            if !verdict.equal {
                continue;
            }
            if verdict.bit_identical {
                return Some((program, r));
            }
            if best
                .as_ref()
                .is_none_or(|(_, _, u)| verdict.worst_ulps < *u)
            {
                best = Some((program, r, verdict.worst_ulps));
            }
        }
    }
    best.map(|(program, r, _)| (program, r))
}

/// The chain spelling, seamed at `rotation`, with the same-carrier
/// repair driven by the DRIVER's own refusals.
fn chain_form(
    loop_: &ProfileLoop<f64>,
    declared: &[bool],
    rotation: usize,
    tol: Tol,
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
                Step::ArcTo(crate::path::program::ArcData::Bulge {
                    target,
                    b: here.bulge,
                })
            });
        }
    }

    repair_same_carrier(program, &origin, tol)
}

/// Turn `arc_to` into `arc_continue` wherever the DRIVER says the
/// junction is a carrier continuation (§5-1's class, met by the
/// binder's own refusal rather than by a re-derived predicate).
///
/// A cocircular arc/arc junction has zero turn, so `arc_to` classifies
/// it `JunctionTangent` before any carrier-identity question is asked;
/// `SameCarrierJunction` is the spelling the tangent-arc and fillet
/// doors use for the same fact. Both are triggers. The substitution is
/// kept only if it makes PROGRESS (the next refusal, if any, is later
/// in the program), so a genuine two-carrier tangency — which wants a
/// declaration, not a subdivision — is never laundered into one.
fn repair_same_carrier(
    mut program: Vec<Step<f64>>,
    origin: &[usize],
    tol: Tol,
) -> Result<Vec<Step<f64>>, LiftRefusal> {
    // Each accepted substitution moves the refusal strictly later, so
    // the program's length bounds the number of passes.
    for _ in 0..=program.len() {
        let error = match replay(&program, tol) {
            Ok(_) => return Ok(program),
            Err(e) => e,
        };
        if !is_carrier_continuation(&error.kind) {
            // Some other geometric wall: leave it for the census to
            // record in the driver's own words.
            return Ok(program);
        }
        match program.get(error.step) {
            Some(Step::ArcTo(crate::path::program::ArcData::Bulge {
                target: Target::Point(p),
                ..
            })) => {
                let p = *p;
                let saved = program.clone();
                if let Some(slot) = program.get_mut(error.step) {
                    *slot = Step::ArcContinue(p);
                }
                match replay(&program, tol) {
                    Ok(_) => return Ok(program),
                    Err(next) if next.step > error.step => {}
                    Err(_) => return Ok(saved),
                }
            }
            Some(Step::ArcTo(crate::path::program::ArcData::Bulge {
                target: Target::Start,
                ..
            })) => {
                return Err(LiftRefusal::SameCarrierClose {
                    joint: origin.get(error.step).copied().unwrap_or_default(),
                });
            }
            _ => return Ok(program),
        }
    }
    Ok(program)
}

/// Is this refusal the "the incoming carrier just continues" fact?
fn is_carrier_continuation(kind: &ReplayErrorKind<f64>) -> bool {
    matches!(
        kind,
        ReplayErrorKind::Path(
            PathError::SameCarrierJunction { .. } | PathError::JunctionTangent { .. }
        )
    )
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
struct Verdict {
    /// The two loops agree to within the value-equal thresholds.
    equal: bool,
    /// Every compared value matched bit for bit.
    bit_identical: bool,
    /// Largest ulp gap seen.
    worst_ulps: u64,
    /// Largest absolute gap seen.
    worst_abs: f64,
}

impl Verdict {
    /// The verdict for loops that do not even correspond structurally.
    fn incomparable() -> Self {
        Self {
            equal: false,
            bit_identical: false,
            worst_ulps: u64::MAX,
            worst_abs: f64::INFINITY,
        }
    }
}

fn compare(want: &ProfileLoop<f64>, got: &ProfileLoop<f64>) -> Verdict {
    if want.vertices.len() != got.vertices.len() {
        return Verdict::incomparable();
    }
    let joints = |l: &ProfileLoop<f64>| {
        let mut js = l.tangent_joints.clone();
        js.sort_unstable();
        js.dedup();
        js
    };
    if joints(want) != joints(got) {
        return Verdict::incomparable();
    }
    let mut verdict = Verdict {
        equal: true,
        bit_identical: true,
        worst_ulps: 0,
        worst_abs: 0.0,
    };
    for (w, g) in want.vertices.iter().zip(got.vertices.iter()) {
        for (x, y) in [(w.pos.x, g.pos.x), (w.pos.y, g.pos.y), (w.bulge, g.bulge)] {
            if x.to_bits() != y.to_bits() {
                verdict.bit_identical = false;
            }
            let gap = ulps(x, y);
            let abs = (x - y).abs();
            verdict.worst_ulps = verdict.worst_ulps.max(gap);
            if abs > verdict.worst_abs {
                verdict.worst_abs = abs;
            }
            if gap > VALUE_EQUAL_ULPS && abs > VALUE_EQUAL_ABS {
                verdict.equal = false;
            }
        }
    }
    verdict
}

/// Distance in representable doubles, via the standard monotone
/// total-order key (no casts, no overflow).
fn ulps(a: f64, b: f64) -> u64 {
    // Bit equality, not `==`: the key below separates -0.0 from +0.0 by
    // one step, and reporting that as 1 rather than 0 keeps the figure
    // consistent with the `bit_identical` flag, which also sees it.
    if a.to_bits() == b.to_bits() {
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
    let b2 = bulge.powi(2);
    let four_b = 4.0 * bulge;
    let mid = a.lerp(b, 0.5);
    Some((
        mid + normal * (len * (1.0 - b2) / four_b),
        (len * (1.0 + b2) / four_b).abs(),
    ))
}
