//! **The v1 → program lift** (PROFILES-V2 §V5, LIB-SWITCH §7).
//!
//! A development-side authoring tool: it takes a v1-form
//! [`ProfileLoop`] — vertices and canonical segments — and mints an
//! equivalent chain- (or carrier-) vocabulary program. It is **not a load path and never runs at load** (LQ7a's
//! clean break: a v1-form document predates the `id:` header line, so
//! the persistence door refuses it `PersistError::HeaderId` with the
//! regenerate recourse and never reads its body; nothing in that door
//! reaches this module).
//!
//! # What the tangent junctions pin
//!
//! PATHS-DESIGN's harmonization paragraph says the v1 flags are what
//! make the lift well-defined: "declared junctions become `.tangent()`
//! calls, fillet-authored arcs become `.fillet(r)`". A loop stores no
//! flag; its tangent junctions are what validation derives from its
//! carriers (D1: every zero-turn joint is a tangent joint), and the
//! lift reads that set. A fillet leaves no marker of its own — it is
//! exactly *an arc whose two joints are both tangent*, which is also
//! what a hand-authored tangent arc looks like. Recovering `.fillet(r)`
//! would mean un-trimming the corner (inference, not a set read), and
//! the reconstruction is anchor-sensitive in precisely the way finding
//! F10 describes. This tool therefore spells every tangent junction
//! `.tangent()` and leaves the fillet spelling banked; the census below
//! counts the fillet-shaped loops it meets so the cost is measured
//! rather than assumed.
//!
//! # Two refusal layers, deliberately
//!
//! Mirroring [`ReplayErrorKind`]'s own split:
//!
//! - **Structural** walls are this tool's: a loop the chain vocabulary
//!   has no shape for at all, or whose tangent junctions cannot be
//!   derived ([`LiftRefusal`]).
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
//! A chain binds its entry with `.at(p)`, which constructs nothing.
//! Since a loop is cyclic and the seam is authoring freedom, the lift
//! ROTATES to the first joint that is not tangent and reports the
//! rotation it used; the
//! differential comparison is against the correspondingly rotated
//! source, which is pure reindexing (no arithmetic, so bit-exactness is
//! preserved).
//!
//! A loop whose every joint is tangent — a fully filleted outline, a
//! stadium — is seamed the other way round: the entry still constructs
//! nothing, but the CLOSING TARGET does
//! ([`Start::arrives_tangent`](crate::path::Start::arrives_tangent)),
//! so the chain seams at 0 and joint 0's tangency rides the arrival.
//! Every closing verb takes that target, including the continuation —
//! which is why a tangent joint whose leaving segment closes the loop
//! straight is `continue_to` and not `.tangent().line(len)`: the
//! continuation verb constructs the joint it mints, and it closes.
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
//! means). An arc is written about its stored centre
//! (`arc_to(Center)`), in the winding its stored sweep turns.

use crate::path::PathError;
use crate::path::program::{ArcData, ReplayError, ReplayErrorKind, Step, Target, replay};
use crate::{ArcSweep, ProfileLoop, Segment};
use geom_core::Tol;

/// How faithfully a lifted program reproduces its source loop, up to
/// the reported seam rotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fidelity {
    /// Every vertex coordinate and stored segment field matches bit for
    /// bit.
    BitIdentical,
    /// The shape agrees but some DERIVED value differs in its last
    /// bits — the F10/W1 classes of PROFILES-V2 §V5. The lifted program
    /// changes what is SAID, not what is drawn.
    ValueEqual,
}

/// A structural wall: a loop the chain vocabulary has no shape for, or
/// whose tangent junctions cannot be derived.
///
/// Geometric walls of the SPELLING are NOT here — those are the
/// driver's refusals, surfaced verbatim through
/// [`LiftOutcome::ReplayRefused`].
#[derive(Clone, Debug, PartialEq)]
pub enum LiftRefusal {
    /// Fewer than two vertices: the chain vocabulary spells a loop
    /// from two vertices up, so a one-segment loop (D1's full turn) has
    /// no spelling here yet.
    TooFewVertices {
        /// How many the loop carried.
        vertices: usize,
    },
    /// A coordinate, or a stored arc's centre, radius or sweep, is not
    /// finite. Authored data must be real numbers before any spelling
    /// question arises.
    NonFinite {
        /// The offending vertex index.
        vertex: usize,
    },
    /// The loop's tangent junctions could not be derived: a segment or
    /// a joint validation refuses or cannot decide, so there is no set
    /// to spell.
    Unclassified(crate::ProfileError),
    /// A same-carrier arc run reaches the SEAM. `arc_continue` has no
    /// closing form (it mints a structural subdivision vertex mid-chain
    /// only), and closing with `arc_to(Start)` on the incoming carrier
    /// leaves the seam's own zero-turn junction unconstructed, which
    /// refuses `SeamTangent`; the seam's construction is
    /// `Start.arrives_tangent()`.
    SameCarrierClose {
        /// The joint's vertex index in the SOURCE loop.
        joint: usize,
    },
}

impl std::fmt::Display for LiftRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewVertices { vertices } => {
                write!(
                    f,
                    "the chain vocabulary spells a loop of at least two vertices; found {vertices}"
                )
            }
            Self::NonFinite { vertex } => {
                write!(
                    f,
                    "vertex {vertex} carries a non-finite coordinate or arc field"
                )
            }
            Self::Unclassified(error) => {
                write!(f, "the loop's tangent junctions cannot be derived: {error}")
            }
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
    /// Lifted, and replay reproduces the source loop (up to
    /// `rotation`): its vertex table to `fidelity`, and its tangent
    /// junctions exactly, each one the replay's constructors made.
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
    lift_seamed(loop_, tol).map(|lifted| lifted.program)
}

/// **The differential harness**: lift, replay, and compare against the
/// source loop.
///
/// Total by construction — every path produces a census row rather than
/// an error the caller must interpret.
pub fn lift_checked(loop_: &ProfileLoop<f64>, tol: Tol) -> LiftOutcome {
    let Seamed {
        program,
        rotation,
        tangent,
    } = match lift_seamed(loop_, tol) {
        Ok(lifted) => lifted,
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
    let verdict = compare(&rotated(loop_, &tangent, rotation), &replayed);
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

/// What the lift minted: the program, the seam rotation it authored
/// at, and the source's tangent junctions it spelled.
struct Seamed {
    program: Vec<Step<f64>>,
    rotation: usize,
    tangent: Vec<usize>,
}

/// The lift proper ([`Seamed`]).
fn lift_seamed(loop_: &ProfileLoop<f64>, tol: Tol) -> Result<Seamed, LiftRefusal> {
    let n = loop_.vertices.len();
    if n < 2 {
        return Err(LiftRefusal::TooFewVertices { vertices: n });
    }
    for (i, (v, segment)) in loop_.vertices.iter().zip(&loop_.segments).enumerate() {
        let arc_finite = match segment {
            Segment::Line => true,
            Segment::Arc(arc) => [arc.centre.x, arc.centre.y, arc.radius, arc.sweep]
                .iter()
                .all(|x| x.is_finite()),
        };
        if !(v.x.is_finite() && v.y.is_finite() && arc_finite) {
            return Err(LiftRefusal::NonFinite { vertex: i });
        }
    }
    let tangent =
        crate::validate::table_tangent_joints(loop_, tol).map_err(LiftRefusal::Unclassified)?;

    // The closed-carrier forms first: a loop that IS a carrier has no
    // seam to author, and `circle`/`circle_split` say so in one step.
    if let Some((program, rotation)) = carrier_form(loop_, &tangent, tol) {
        return Ok(Seamed {
            program,
            rotation,
            tangent,
        });
    }

    // The seam is authoring freedom, so the lift rotates to a joint
    // `.at(p)` can carry — one that is not tangent — and reports the
    // rotation it used. When every joint is tangent there is no such
    // rotation, and the seam is authored the other way round: `.at(p)`
    // still constructs nothing, but the CLOSER does
    // (`Start.arrives_tangent()`), so the chain seams at 0 and the
    // arrival carries joint 0's tangency.
    let mut is_tangent = vec![false; n];
    for &j in &tangent {
        is_tangent[j] = true;
    }
    let rotation = is_tangent.iter().position(|t| !*t).unwrap_or(0);
    let program = chain_form(loop_, &is_tangent, rotation, tol)?;
    Ok(Seamed {
        program,
        rotation,
        tangent,
    })
}

/// Try the one-step carrier spellings, VERIFYING each by replay rather
/// than by re-deriving a carrier-identity predicate.
///
/// The seam is searched, not assumed: `circle` fixes its own seam at
/// the +x pole and `circle_split` at `phase`, so a hand-authored carrier
/// loop generally corresponds to one of them ROTATED. Returns the
/// program and the rotation it matched at, preferring an exact match.
fn carrier_form(
    loop_: &ProfileLoop<f64>,
    tangent: &[usize],
    tol: Tol,
) -> Option<(Vec<Step<f64>>, usize)> {
    let n = loop_.vertices.len();
    // Only a loop that is arcs all the way round can be one carrier;
    // this guard keeps the search off every polygon.
    if n < 2 || loop_.segments.iter().any(|s| matches!(s, Segment::Line)) {
        return None;
    }
    let mut best: Option<(Vec<Step<f64>>, usize, u64)> = None;
    for r in 0..n {
        let a = loop_.vertices[r];
        let Segment::Arc(arc) = loop_.segments[r] else {
            continue;
        };
        let (centre, radius) = (arc.centre, arc.radius);
        let phase = (a.y - centre.y).atan2(a.x - centre.x);
        let want = rotated(loop_, tangent, r);
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
    is_tangent: &[bool],
    rotation: usize,
    tol: Tol,
) -> Result<Vec<Step<f64>>, LiftRefusal> {
    let n = loop_.vertices.len();
    let at = |k: usize| loop_.vertices[(rotation + k) % n];
    // Which SOURCE segment each step belongs to, for refusal reporting.
    let mut origin = vec![rotation];
    let mut program = vec![Step::At(at(0))];

    for k in 0..n {
        let src = (rotation + k) % n;
        let (here, line) = (at(k), matches!(loop_.segments[src], Segment::Line));
        // The seam joint is the one the entry cannot construct, so the
        // closing target carries its tangency instead.
        let target = if k + 1 == n {
            // `is_tangent` is per-vertex and `rotation < n`, both by
            // construction at the caller — indexing is the honest read.
            if is_tangent[rotation] {
                Target::StartArriving
            } else {
                Target::Start
            }
        } else {
            Target::Point(at(k + 1))
        };
        if k > 0 && is_tangent[src] {
            if line {
                // A straight leg off a tangent joint IS the
                // continuation verb, and the continuation verb
                // constructs the joint it mints — so no `.tangent()` precedes it,
                // and unlike `.line(len)` it closes.
                origin.push(src);
                if k + 1 == n {
                    program.push(Step::ContinueTo(target));
                } else {
                    program.push(Step::Tangent);
                    origin.push(src);
                    program.push(Step::Line(here.distance(at(k + 1))));
                }
            } else {
                origin.push(src);
                program.push(Step::Tangent);
                origin.push(src);
                program.push(Step::TangentArcTo(target));
            }
        } else {
            origin.push(src);
            program.push(match loop_.segments[src] {
                Segment::Line => Step::LineTo(target),
                Segment::Arc(arc) => Step::ArcTo(ArcData::Center {
                    c: arc.centre,
                    winding: if arc.sweep > 0.0 {
                        ArcSweep::Ccw
                    } else {
                        ArcSweep::Cw
                    },
                    target,
                }),
            });
        }
    }

    repair_same_carrier(program, &origin, tol)
}

/// Construct the zero-turn joint wherever the DRIVER says an arc leg
/// arrives at one unconstructed (§5-1's class, met by the binder's own
/// refusal rather than by a re-derived predicate).
///
/// A cocircular arc/arc junction has zero turn, so `arc_to` classifies
/// it `JunctionTangent` — the unconstructed zero-turn junction, which
/// is the one trigger left. The re-spelling is the lattice's own: the
/// leg becomes `.tangent().tangent_arc_to(p)`, its joint constructed and
/// its arc derived from the inherited tangent and the authored target
/// — which mints the raw run's vertex and a carrier the raw run's own
/// satisfies, the tangent-chord derivation being one it meets. Whether the derived arc reproduces the raw one is the
/// census's comparison, as for every lift. The substitution is kept
/// only if it makes PROGRESS (the next refusal, if any, is later in
/// the program), so a wall this spelling does not move is left in the
/// driver's own words rather than laundered.
fn repair_same_carrier(
    mut program: Vec<Step<f64>>,
    origin: &[usize],
    tol: Tol,
) -> Result<Vec<Step<f64>>, LiftRefusal> {
    // The construction is two steps where the leg was one, so the
    // source-segment map grows alongside the program.
    let mut origin = origin.to_vec();
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
            Some(Step::ArcTo(ArcData::Center {
                target: Target::Point(p),
                ..
            })) => {
                let p = *p;
                let saved = program.clone();
                program[error.step] = Step::TangentArcTo(Target::Point(p));
                program.insert(error.step, Step::Tangent);
                let src = origin.get(error.step).copied().unwrap_or_default();
                origin.insert(error.step, src);
                match replay(&program, tol) {
                    Ok(_) => return Ok(program),
                    // Progress means past BOTH steps of the construction.
                    Err(next) if next.step > error.step + 1 => {}
                    Err(_) => return Ok(saved),
                }
            }
            Some(Step::ArcTo(ArcData::Center {
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
///
/// One arm since the 2026-09-02 ruling: every zero-turn joint is a
/// tangent joint, so the algebra no longer refuses carrier IDENTITY at
/// all and `JunctionTangent` — the unconstructed zero-turn junction —
/// is the whole of what this asks about.
fn is_carrier_continuation(kind: &ReplayErrorKind<f64>) -> bool {
    matches!(
        kind,
        ReplayErrorKind::Path(PathError::JunctionTangent { .. })
    )
}

// ------------------------------------------------------------------
// Comparison
// ------------------------------------------------------------------

/// The source loop re-seamed at `rotation`, with its tangent junctions:
/// its (vertex, segment) chain reindexed, every stored bit carried
/// verbatim, and each junction carried to its new index.
fn rotated(loop_: &ProfileLoop<f64>, tangent: &[usize], rotation: usize) -> Source {
    let n = loop_.vertices.len();
    let r = if n == 0 { 0 } else { rotation % n };
    let chain = (0..n).map(|k| {
        let i = (r + k) % n;
        (loop_.vertices[i], loop_.segments[i])
    });
    let mut tangent: Vec<usize> = tangent.iter().map(|&j| (j % n + n - r) % n).collect();
    tangent.sort_unstable();
    Source {
        loop_: ProfileLoop::from_chain(chain),
        tangent,
    }
}

/// A source loop as the comparison reads it: the table and its tangent
/// junctions, ascending.
struct Source {
    loop_: ProfileLoop<f64>,
    tangent: Vec<usize>,
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

/// The source against the replay: the replay's constructors made
/// exactly the source's tangent junctions, and its table holds the
/// source's geometry.
fn compare(want: &Source, got: &crate::ConstructedLoop<f64>) -> Verdict {
    let (want_table, got_table) = (&want.loop_, got.as_loop());
    if want_table.vertices.len() != got_table.vertices.len()
        || want.tangent != got.constructed_joints()
    {
        return Verdict::incomparable();
    }
    let mut verdict = Verdict {
        equal: true,
        bit_identical: true,
        worst_ulps: 0,
        worst_abs: 0.0,
    };
    // Each vertex with its leaving segment's stored fields: a line
    // stores none, an arc its centre, radius and sweep. Two loops whose
    // segments differ in kind do not correspond.
    let rows = |l: &ProfileLoop<f64>| {
        l.vertices
            .iter()
            .zip(&l.segments)
            .map(|(p, segment)| match segment {
                Segment::Line => vec![p.x, p.y],
                Segment::Arc(arc) => {
                    vec![p.x, p.y, arc.centre.x, arc.centre.y, arc.radius, arc.sweep]
                }
            })
            .collect::<Vec<_>>()
    };
    let (want_rows, got_rows) = (rows(want_table), rows(got_table));
    if want_rows
        .iter()
        .zip(&got_rows)
        .any(|(w, g)| w.len() != g.len())
    {
        return Verdict::incomparable();
    }
    for (w, g) in want_rows.into_iter().zip(got_rows) {
        for (x, y) in w.into_iter().zip(g) {
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

#[cfg(test)]
mod tests {
    use super::{Source, compare};
    use crate::{ConstructedLoop, ProfileLoop, Segment};
    use geom_core::Point2;

    fn square() -> ProfileLoop<f64> {
        let corners = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        ProfileLoop::from_chain(corners.map(|(x, y)| (Point2::new(x, y), Segment::Line)))
    }

    /// **The comparator reads a tangent junction the replay did not
    /// construct, or one it constructed that the source does not have,
    /// as a different loop.** One table throughout; only the joint sets
    /// differ.
    ///
    /// Red if a joint-set difference compares in either direction, or
    /// equal sets do not.
    #[test]
    fn a_joint_set_difference_either_way_does_not_compare() {
        let source = |tangent: Vec<usize>| Source {
            loop_: square(),
            tangent,
        };
        let replay = |joints: Vec<usize>| ConstructedLoop::fixture(square(), joints);
        let added = compare(&source(vec![]), &replay(vec![1]));
        assert!(!added.equal, "added: a joint the source has not");
        let dropped = compare(&source(vec![1]), &replay(vec![]));
        assert!(!dropped.equal, "dropped: a joint the replay lost");
        let kept = compare(&source(vec![2]), &replay(vec![2]));
        assert!(kept.equal && kept.bit_identical, "kept: one loop");
    }
}
