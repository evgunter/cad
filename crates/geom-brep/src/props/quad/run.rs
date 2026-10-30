//! **When a lane stops** (module docs of [`super`]): the one home
//! every face lane runs its rounds through ([`Run`]), the work wall and
//! the floor share it stops on, the vocabulary a caller drives a lane
//! with ([`RoundWindow`], [`Accept`], [`LaneRun`]), the funnel wrapper
//! the lanes decide through, the face-extent gate an accepted round
//! passes, and the convergence meter a caller reads.

use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom_core::{Band, Decide, Margin, Sign};

use crate::descent::Work;
use crate::props::bracket::{lo_or_refuse, mid};
use crate::props::{PropsCheck, PropsError};

/// Pieces per trim edge at round 0, doubling per round: the composite
/// rule of the cylinder lane and of the spiric lane.
pub(in crate::props) const EDGE_INIT_PIECES: usize = 16;

/// **The work a face may spend over all its rounds**, in weighted cell
/// evaluations: the unit is a tenth of one cell of the rational lane's
/// composite arm, and each lane weighs its own cells by a fixed factor
/// in that unit, so a unit costs about the same in every lane. Spent
/// cumulatively ([`crate::descent::Work`]): a round runs only if every
/// round before it and it fit.
///
/// **Where it sits: calibrated, not derived.** It is the cumulative
/// work of the costliest face in the measured corpus that converges
/// under the per-lane round caps it replaces. The corpus is the
/// geom-brep, topo and sweep suites with the slow set, the tour and the
/// python suite, at ε = 1e-9, 1e-12 and 1e-6. That face is `demos/tour`'s
/// `lily_leaf_a`, the 17-station lofted long leaf: its knot-cut
/// composite wall certifies at round 7 at ε = 1e-9 after 14,243,680.
/// The wall is that figure rounded up.
///
/// **It bounds work, not rounds**, so it gives a face fewer rounds the
/// more each of its rounds costs. A face costlier per round than the
/// calibrating one — more knots, chords, edges or spirics — stops at
/// [`LaneStop::Wall`] a round or more before the round cap it replaces
/// would have, and hands back that round's sound bracket rather than
/// its number. The review of PR 5589 executed two: a quarter torus at
/// 12× rebuilt on 9×9 conic spans (its cap certified it at round 7; it
/// stops at round 6) and a flat chart trimmed by a hexagon of curved
/// arcs (round 4).
///
/// **A rectangle lane's face takes it once per knot-span cell**
/// ([`span_work_wall`]): its rounds are laid per span cell, so its
/// round's work is the span cells' sum, and a patch read whole stops at
/// the round its per-span tiles would.
pub const FACE_WORK_WALL: usize = 14_250_000;

/// **The rectangle lanes' work wall**: [`FACE_WORK_WALL`] per knot-span
/// cell the rectangle meets. Their schedules are per span cell
/// ([`super::cells::knot_aligned_cuts`]), so a round's work is the sum
/// of its span cells', and this wall is the sum of theirs.
pub(super) fn span_work_wall(span_cells: usize) -> usize {
    FACE_WORK_WALL.saturating_mul(span_cells)
}

/// **The floor's share**, `2⁻¹⁰`: a lane is at its floor once the part
/// of a round's width a later round removes is at most this share of
/// the width. A stop there can cost a convergence only for a face whose
/// target lies within that share above its floor, or within what its
/// rest moves by between rounds (below), about `1·ε` against
/// the band's `K·ε` at the reporting target `1024·ε`, so such a face
/// reads in-band, never certified.
///
/// The stop reads that round alone, and it assumes the rest does not
/// shrink much in a later round: the pads move as the pieces do, by
/// 1–2% on a sliver wall, and the arithmetic grows with the cell count.
/// Nothing asserts that: a lane whose rest did shrink would floor a round
/// early, with a sound but wider bracket.
pub const FLOOR_SHARE: f64 = 1.0 / FLOOR_SHARE_INV;
/// `1 /` [`FLOOR_SHARE`], exact: the floor test is `removable · 1024 ≤ width`.
const FLOOR_SHARE_INV: f64 = 1024.0;

/// **Whether flux enclosure `a` is narrower than `b`** — the one rule
/// a face keeps its narrowest round by, within a lane's call
/// ([`Run`]) and across the windows a caller runs it in.
#[must_use]
pub fn narrower(a: Interval, b: Interval) -> bool {
    a.width() < b.width()
}

/// **The rounds one call of a face lane runs.**
///
/// The rounds of a lane are INDEPENDENT: each recomputes its composite
/// sum from scratch over twice the previous round's pieces, and nothing
/// carries over but the piece count. So a window is exactly a sub-range
/// of the one schedule, its enclosures are the enclosures that round
/// would have produced inside an uninterrupted run, and a face run as
/// `0..=k` then `k+1..=…` runs every round the uninterrupted run runs,
/// once each; each entry re-derives the face's setup, which the
/// uninterrupted run derives once. What a round's stop reads is a function of the
/// round alone (its own split of the width, and the work through it), so
/// a window stops where the uninterrupted run would.
#[derive(Clone, Copy, Debug)]
pub struct RoundWindow {
    /// The first round to run. `0` starts the schedule; a higher value
    /// resumes it.
    pub first: usize,
    /// The last round to run. `usize::MAX` runs until the caller accepts
    /// a round or the lane stops ([`LaneStop`]); a lower value also stops
    /// at that round.
    pub last: usize,
}

impl RoundWindow {
    /// Every round, until the caller accepts one or the lane stops.
    pub const SCHEDULE: Self = Self {
        first: 0,
        last: usize::MAX,
    };

    /// Exactly one round of the schedule.
    #[must_use]
    pub const fn at(round: usize) -> Self {
        Self {
            first: round,
            last: round,
        }
    }
}

/// **Why a face lane would stop after a round**, read off that round
/// before its caller is asked about it. The lane knows no target: whether
/// a round's enclosure is enough is the caller's question ([`Accept`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaneStop {
    /// The next round fits in the work wall and can narrow.
    Refinable,
    /// Refinement stopped paying: the part of the width a later round
    /// removes (the rule's remainder) is at most [`FLOOR_SHARE`] of the
    /// width, the rest being the part the rule does not remove (the
    /// honesty pads and the arithmetic). That rest is the round's own: it
    /// moves between rounds as the pieces move, by 1–2% on a sliver wall,
    /// so a later round may be a little narrower or wider. A lane with no
    /// rounds is at its floor.
    Floor,
    /// The next round would take the face past its work wall
    /// ([`FACE_WORK_WALL`], per knot-span cell on a rectangle lane).
    Wall,
}

/// **The caller's question, asked of every round a lane runs**: is this
/// enclosure enough? It is handed the round's enclosure and the lane's
/// own stop for it, and answers `true` to end the call at that round.
/// Its refusal ends the call.
pub type Accept<'a> = dyn FnMut(&FaceCutBounds, LaneStop) -> Result<bool, PropsError> + 'a;

/// **Where one call of a face lane ended.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaneEnd {
    /// The caller accepted a round, which the lane would have ended on
    /// for this reason of its own (a later round exists only where it
    /// is [`LaneStop::Refinable`]).
    Accepted(LaneStop),
    /// The window's last round, with rounds remaining.
    Window,
    /// The lane's floor ([`LaneStop::Floor`]).
    Floor,
    /// The work wall ([`LaneStop::Wall`]).
    Wall,
}

/// **One call of a face lane**: its enclosure, the round it is taken at,
/// and why the call ended.
///
/// An accepted call answers with the round accepted. Any other answers
/// with the NARROWEST round, by flux width, the call ran: the rounds are
/// independent sound enclosures, so a later one is no truer than an
/// earlier one, and past the floor a later round can be wider.
#[derive(Clone, Copy, Debug)]
pub struct LaneRun {
    /// The enclosure.
    pub bounds: FaceCutBounds,
    /// The round `bounds` is taken at.
    pub round: usize,
    /// How many refinement rounds the lane ran to get here, the window's
    /// first round counted from `0`: `0` for a lane with no rounds.
    pub rounds: usize,
    /// Why the call ended.
    pub end: LaneEnd,
}

impl LaneRun {
    /// The same run with its enclosure mapped (the traversal winding a
    /// caller carries into the flux).
    #[must_use]
    pub fn map_bounds(self, f: impl FnOnce(FaceCutBounds) -> FaceCutBounds) -> Self {
        Self {
            bounds: f(self.bounds),
            ..self
        }
    }
}

/// Certified enclosures of one curved-cut face's contributions.
#[derive(Clone, Copy, Debug)]
pub struct FaceCutBounds {
    /// Enclosure of `∮ p·n dA` (n outward; volume = Σ flux/3).
    pub flux: Interval,
    /// Enclosure of the unsigned face area.
    pub area: Interval,
}

/// Funnel wrapper (the `props_*` idiom of [`crate::props::curved`]): margins
/// are certification-substrate `f64` lengths lifted to the caller's
/// scalar so every lane (f64 / Probe / Interval) records identically.
pub(super) fn classify_len<T: Decide>(
    name: &'static str,
    margin: Margin<f64>,
    band: Band,
    check: PropsCheck,
) -> Result<Sign, PropsError> {
    geom_core::k_stats::decide(name, margin.lift::<T>(), band)
        .map_err(|cause| PropsError::Escalated { cause, check })
}

/// **The face-extent gate**, asked of the enclosure a lane answers an
/// accepting caller with: the area must be definitely positive, metered
/// as the face's mean width `area/perimeter` — a length (the
/// closed-form lanes' `props_face_extent` lever, quadrature-shaped).
pub(in crate::props) fn face_extent<T: Decide>(
    area: Interval,
    perimeter: f64,
    band: Band,
) -> Result<(), PropsError> {
    match classify_len::<T>(
        "props_quad_face_extent",
        Margin::over_lever(lo_or_refuse(area), perimeter),
        band,
        PropsCheck::Extent,
    )? {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(PropsError::DegenerateFace),
    }
}

/// **Every round a face lane has run in this process** — test
/// vocabulary for what a measurement pays, since a resolution records
/// no decision a test could count instead. Process-wide: a reading is
/// one test's only where that test has its process to itself, as
/// nextest runs every test ([`rounds_run_by`]).
#[cfg(any(test, feature = "test-support"))]
static ROUNDS_RUN: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

/// `run`'s answer, and the face-lane rounds it ran ([`ROUNDS_RUN`]).
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn rounds_run_by<R>(run: impl FnOnce() -> R) -> (R, usize) {
    let before = ROUNDS_RUN.load(core::sync::atomic::Ordering::Relaxed);
    let answer = run();
    let after = ROUNDS_RUN.load(core::sync::atomic::Ordering::Relaxed);
    (answer, after - before)
}

/// **One call's rounds, and the stop every lane reads** — one home for
/// the five lanes, which differ in it only by what a round costs and by
/// how they split a round's width.
///
/// Per round a lane hands [`Run::stop`] its enclosure and the part of its
/// flux width a later round removes (the rule's remainder); everything
/// else in the width is the part no round removes. The stop is
/// [`LaneStop::Floor`] once the removable part is at most
/// [`FLOOR_SHARE`] of the width, [`LaneStop::Wall`] when the next round would pass
/// the face's work wall, and [`LaneStop::Refinable`] otherwise. Both read
/// the round alone: the work through a round is the sum of its rounds'
/// costs, which `cost` states without running them, so a window entered
/// at any round stops exactly where the whole schedule would.
pub(in crate::props) struct Run<'c> {
    window: RoundWindow,
    work: Work,
    cost: &'c dyn Fn(usize) -> usize,
    narrowest: Option<(usize, FaceCutBounds)>,
}

impl<'c> Run<'c> {
    /// The call over `window` under [`FACE_WORK_WALL`], its earlier
    /// rounds' work already spent. `cost(round)` is that round's
    /// weighted cell count.
    pub(in crate::props) fn enter(window: RoundWindow, cost: &'c dyn Fn(usize) -> usize) -> Self {
        Self::enter_walled(window, FACE_WORK_WALL, cost)
    }

    /// [`Run::enter`] under the work wall `wall`.
    pub(in crate::props) fn enter_walled(
        window: RoundWindow,
        wall: usize,
        cost: &'c dyn Fn(usize) -> usize,
    ) -> Self {
        let mut work = Work::new(wall);
        for round in 0..window.first {
            work.spend(cost(round));
        }
        debug_assert!(
            window.first == 0 || work.admits(cost(window.first)),
            "a window resuming at round {} past the work wall",
            window.first
        );
        Self {
            window,
            work,
            cost,
            narrowest: None,
        }
    }

    /// The round the call starts at.
    pub(in crate::props) fn first(&self) -> usize {
        self.window.first
    }

    /// **The stop `round` ends on**, its work spent and its enclosure
    /// kept if it is the narrowest so far. `removable` is the part of
    /// `bounds.flux`'s width a later round removes.
    pub(in crate::props) fn stop(
        &mut self,
        round: usize,
        bounds: FaceCutBounds,
        removable: f64,
    ) -> LaneStop {
        #[cfg(any(test, feature = "test-support"))]
        ROUNDS_RUN.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        self.work.spend((self.cost)(round));
        let width = bounds.flux.width();
        if self
            .narrowest
            .is_none_or(|(_, held)| narrower(bounds.flux, held.flux))
        {
            self.narrowest = Some((round, bounds));
        }
        if removable * FLOOR_SHARE_INV <= width {
            LaneStop::Floor
        } else if self.work.admits((self.cost)(round + 1)) {
            LaneStop::Refinable
        } else {
            LaneStop::Wall
        }
    }

    /// **One round, asked of the caller**: the round's stop
    /// ([`Run::stop`]), the caller's answer to it, and on an accepted
    /// round the lane's `gate` on the enclosure it answers with. The
    /// stop and the answer are what [`Run::exit`] or [`Run::ringed`]
    /// reads next.
    pub(in crate::props) fn ask(
        &mut self,
        round: usize,
        bounds: FaceCutBounds,
        removable: f64,
        accept: &mut Accept<'_>,
        gate: impl FnOnce() -> Result<(), PropsError>,
    ) -> Result<(LaneStop, bool), PropsError> {
        let stop = self.stop(round, bounds, removable);
        let accepted = accept(&bounds, stop)?;
        if accepted {
            gate()?;
        }
        Ok((stop, accepted))
    }

    /// **The round loop's exit ladder**, asked after the caller read
    /// `round`: the accepted round, the narrowest round where the lane
    /// stopped or the window ends, or `None` to refine.
    pub(in crate::props) fn exit(
        &self,
        round: usize,
        stop: LaneStop,
        accepted: bool,
        bounds: FaceCutBounds,
    ) -> Option<LaneRun> {
        let rounds = round + 1;
        if accepted {
            return Some(LaneRun {
                bounds,
                round,
                rounds,
                end: LaneEnd::Accepted(stop),
            });
        }
        let end = match stop {
            LaneStop::Floor => LaneEnd::Floor,
            LaneStop::Wall => LaneEnd::Wall,
            LaneStop::Refinable if round >= self.window.last => LaneEnd::Window,
            LaneStop::Refinable => return None,
        };
        let (at, narrowest) = self.narrowest.unwrap_or((round, bounds));
        Some(LaneRun {
            bounds: narrowest,
            round: at,
            rounds,
            end,
        })
    }

    /// **The ringed lanes' ladder** (the cylinder's and the spiric's):
    /// `rings` is the lane's decision that every ring winds against its
    /// face, read off this round's per-loop enclosures. A face's rings
    /// are decided at the round the call ends.
    ///
    /// - **Accepted**: rings decided, the round is the answer. A ring
    ///   reading that is [`refinable`] at a round the lane can refine
    ///   past, inside the window, refines instead — the face's flux
    ///   needs no later round, but the ring's own enclosure narrows. At
    ///   any other round, or on any other refusal, the reading stands.
    /// - **Not accepted**: [`Run::exit`]'s ladder. The enclosure it
    ///   returns encloses the face's flux only if its rings are holes,
    ///   so the rings must be decided before it is returned.
    ///
    /// The escalation stands at a window's last round even with rounds
    /// left: [`LaneRun`] has no end for an enclosure whose rings are
    /// undecided, and one would claim them holes
    /// (`work/quad/a-ring-undecided-at-a-windows-last-round-refuses-a-sign-walk.md`).
    pub(in crate::props) fn ringed(
        &self,
        round: usize,
        stop: LaneStop,
        accepted: bool,
        bounds: FaceCutBounds,
        rings: impl FnOnce() -> Result<(), PropsError>,
    ) -> Result<Option<LaneRun>, PropsError> {
        if accepted {
            return match rings() {
                Err(PropsError::Escalated { cause, .. })
                    if stop == LaneStop::Refinable
                        && round < self.window.last
                        && refinable(&cause) =>
                {
                    Ok(None)
                }
                reading => reading.map(|()| self.exit(round, stop, true, bounds)),
            };
        }
        let Some(out) = self.exit(round, stop, false, bounds) else {
            return Ok(None);
        };
        rings()?;
        Ok(Some(out))
    }
}

/// **Whether a later round can decide a ring reading this round left
/// undecided.** The predicates the ringed lanes' `rings` reach are
/// `props_chart_area_side` (the face's whole) and `props_ring_winding`
/// (each ring), both the mean width `2·A/P` of a loop through
/// `curved::require_holes_wound_against` at the interval scalar: `A` is
/// this round's sound enclosure of the loop's signed chart area,
/// recomputed over twice the pieces next round, and `P` an upper bound
/// on its length that no round changes. Each round encloses the same
/// true mean width, so a reading is refinable exactly when its
/// enclosure STRADDLES a decision boundary: one lying wholly inside an
/// open sliver band ([`geom_core::Indeterminate::terminal_sliver`]) has
/// its true value there, and every later sound enclosure holds that
/// value, so none decides it; a point or poisoned margin is no
/// enclosure to narrow. Read off the escalation itself, so a predicate
/// reaching here that is not enclosure-shaped refines nothing.
fn refinable(cause: &geom_core::Indeterminate) -> bool {
    cause.margin.kind() == geom_core::MarginKind::Enclosure && !cause.terminal_sliver
}

/// The convergence meter: the flux enclosure's width expressed as the
/// **mean boundary displacement** it corresponds to,
/// `width(flux)/(3·area)` (module docs), with the area enclosure's
/// midpoint as the lever arm. `pub` because its readers are the
/// callers' questions: `topo` compares it with a resolution.
///
/// INVARIANT: the returned length is FINITE and non-negative — so a
/// budget refusal carries a number the caller can act on *by
/// construction*, never the `+inf` a cancelled lever used to produce
/// and never a `NaN`.
///
/// **Why the lever needs a guard.** `area` is accumulated as
/// `widen(g_mid, pad)` per cell — a SYMMETRIC pad — so once the
/// Lipschitz pad dwarfs the true area (measured: pad ≈ 1.7e20 against
/// a true area of 1 m² on the extreme-weight bilinear square),
/// `g_mid ± pad` rounds to exactly `∓pad` and `area.lo() + area.hi()`
/// cancels to EXACTLY 0.0. The unguarded division then yielded `+inf`,
/// and the refusal reported the enclosure as "stalled" at that width
/// — while the flux enclosure it was measuring went on narrowing round
/// by round, as an O(h²) rule must. The payload lied, and the
/// convergence predicate was decided by a division artifact instead of
/// by the enclosure. When the flux width is also
/// zero — [`cylinder_cut_face`](super::cylinder::cylinder_cut_face)'s `area = [0, 0]` for a zero-UV-area
/// trim loop — the same division is `0/0 = NaN`, which the funnel
/// turns into `Indeterminate{margin: Invalid}`: a MIS-TYPED refusal,
/// since [`PropsError::Escalated`] means an *in-band* margin, not a
/// degenerate face.
///
/// **Why [`PropsError::DegenerateFace`] is the honest variant** for a
/// non-positive lever rather than a budget stall: `lever ≤ 0` forces
/// `area.lo() ≤ 0`, i.e. the enclosure does not certify that the face
/// has positive extent — exactly the verdict the
/// `props_quad_face_extent` gate reaches from the same `area.lo()` one
/// step later, reading the same number: definitely so when the
/// cancellation is large (an `area.lo()` of −1e20 is nowhere near any
/// band), and where `area` is exactly `[0, 0]` — the zero-UV-area trim
/// loop — the extent is not a tolerance question at all, it is zero.
/// The meter invents no refusal of its own; it reaches the extent
/// verdict one step earlier, on the same evidence.
///
/// **Why the midpoint stays the lever** — not `area.hi()`, not a
/// certified-positive lower lever like `area.lo()`: every face that
/// certifies today must keep certifying with BIT-IDENTICAL numbers
/// (D9), and both alternatives move the meter. `area.hi()` shrinks the
/// metered displacement, so faces the reviewed inventory refuses would
/// start certifying; `area.lo()` inflates it, de-certifying real
/// bodies. Which point of the area enclosure the meter divides by is a
/// metering decision (module docs) and would have to be re-measured
/// and re-ratified as one — the defect fixed here is the *unguarded
/// division*, and only that.
///
/// # Errors
///
/// [`PropsError::QuadratureUnsupported`] for a refused enclosure, and
/// [`PropsError::DegenerateFace`] for a lever that is not positive.
pub fn mean_boundary_displacement(flux: Interval, area: Interval) -> Result<f64, PropsError> {
    let width = flux.width();
    // Bit-for-bit the pre-guard expression: `(lo + hi)·0.5`, times 3.
    let denom = 3.0 * mid(area);
    // A refused enclosure has no midpoint, so it lands here rather
    // than in the degeneracy branch: it is not a statement about the
    // face's extent at all.
    if !width.is_finite() || !denom.is_finite() {
        return Err(PropsError::QuadratureUnsupported {
            what: "a quadrature enclosure with a non-finite width or area (a refused \
                   bracket) — the convergence meter has no honest length to report, \
                   and a refusal carrying a non-finite width would misstate the \
                   enclosure",
        });
    }
    if denom <= 0.0 {
        return Err(PropsError::DegenerateFace);
    }
    let len = width / denom;
    if !len.is_finite() {
        // Finite/finite still overflows on a subnormal lever against a
        // wide flux: the area is zero AT THE FLUX'S SCALE — the same
        // degeneracy the branch above names, reached by rounding.
        return Err(PropsError::DegenerateFace);
    }
    Ok(len)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Bounds;

    use super::super::net::PatchRect;
    use super::super::trimmed::{Curvature, OuterRule};
    use super::super::{cylinder, patch, rational};
    use crate::props::bracket::pt;
    use crate::props::spiric;

    /// A round of flux width `w` about zero over a unit area.
    fn of_width(w: f64) -> FaceCutBounds {
        FaceCutBounds {
            flux: Interval::from_bounds(-0.5 * w, 0.5 * w),
            area: pt(1.0),
        }
    }

    /// The last round [`Run`] admits under `cost`, run from round 0 with
    /// a width no round's remainder falls under.
    fn last_admitted(cost: &dyn Fn(usize) -> usize) -> usize {
        let mut run = Run::enter(RoundWindow::SCHEDULE, cost);
        (0..64)
            .find(|&round| run.stop(round, of_width(1.0), 1.0) == LaneStop::Wall)
            .expect("every lane meets the wall")
    }

    /// **The work wall sits at the composite arm's round 7**, and each
    /// lane's nominal face reaches the round its own weight buys on it.
    /// The weights are measured per cell on one box; these rounds are
    /// what they buy.
    #[test]
    fn the_work_wall_buys_each_lane_its_measured_rounds() {
        type Cost<'a> = &'a dyn Fn(usize) -> usize;
        let unit = PatchRect::new((0.0, 1.0, 0.0, 1.0)).unwrap();
        let bare: (&[f64], &[f64]) = (&[], &[]);
        let exact = OuterRule::Exact(Vec::new());
        let composite = OuterRule::Composite(Box::new(Curvature::of(None, None, None)));
        let rows: [(&str, Cost<'_>, usize); 12] = [
            (
                "rational composite",
                &|r| rational::round_cost(unit, bare, false, r),
                7,
            ),
            (
                "integral composite",
                &|r| patch::composite_round_cost(unit, bare, r),
                6,
            ),
            (
                "rational exact-v, its 4 v block cells",
                &|r| rational::round_cost(unit, bare, true, r),
                11,
            ),
            ("cylinder, 4 edges", &|r| cylinder::round_cost(4, r), 15),
            ("spiric, 1 edge", &|r| spiric::round_cost(1, 0, r), 13),
            (
                "rational rim, degree 2, 4 spans (the vase's cap)",
                &|r| spiric::round_cost(0, spiric::w_rational(2) * 4, r),
                9,
            ),
            (
                "rational rim, degree 7, 1 span",
                &|r| spiric::round_cost(0, spiric::w_rational(7), r),
                9,
            ),
            (
                "trimmed bilinear chart, 1 image and 2 iso chords",
                &|r| exact.round_cost((1, 1), 1, 2, r),
                10,
            ),
            (
                "trimmed biquadratic chart, 1 image and 2 iso chords",
                &|r| exact.round_cost((2, 2), 1, 2, r),
                7,
            ),
            (
                "trimmed bicubic composite, 1 image and 3 iso chords",
                &|r| composite.round_cost((3, 3), 1, 3, r),
                6,
            ),
            (
                "trimmed (3, 2) composite, 3 iso chords",
                &|r| composite.round_cost((3, 2), 0, 3, r),
                8,
            ),
            (
                "trimmed (5, 2) composite at the fallback's floor, 3 iso chords",
                &|r| composite.round_cost((5, 2), 0, 3, r),
                5,
            ),
        ];
        for (lane, cost, want) in rows {
            assert_eq!(last_admitted(cost), want, "{lane}");
        }
    }

    /// **A window stops where the whole schedule does**: the work a
    /// round's stop reads is the sum of every round's cost through it,
    /// which the window entering later spends before its first round.
    #[test]
    fn a_window_stops_where_the_whole_schedule_does() {
        let unit = PatchRect::new((0.0, 1.0, 0.0, 1.0)).unwrap();
        let cost = |r: usize| rational::round_cost(unit, (&[], &[]), false, r);
        for first in 0..=7 {
            let mut run = Run::enter(RoundWindow::at(first), &cost);
            let want = if first == 7 {
                LaneStop::Wall
            } else {
                LaneStop::Refinable
            };
            assert_eq!(run.stop(first, of_width(1.0), 1.0), want, "round {first}");
        }
    }

    /// **The floor is a removable share of at most 2⁻¹⁰**: a round
    /// whose remainder is that share of its width or less stops there;
    /// one whose remainder is more refines, half its width included.
    #[test]
    fn the_floor_is_a_removable_share_of_at_most_one_in_1024() {
        let cost = |_: usize| 1;
        for (removable, want) in [
            (FLOOR_SHARE, LaneStop::Floor),
            (0.0, LaneStop::Floor),
            (FLOOR_SHARE * 1.000_01, LaneStop::Refinable),
            (0.5, LaneStop::Refinable),
        ] {
            let mut run = Run::enter(RoundWindow::SCHEDULE, &cost);
            assert_eq!(
                run.stop(0, of_width(1.0), removable),
                want,
                "removable {removable}"
            );
        }
    }

    /// **An unaccepted call answers its narrowest round, never its
    /// last**: past the floor a later round can be wider than an earlier
    /// one, and the rounds are independent sound enclosures. An accepted
    /// round is the answer whatever the earlier ones were.
    #[test]
    fn an_unaccepted_call_answers_its_narrowest_round() {
        let cost = |_: usize| 1;
        let mut run = Run::enter(RoundWindow::SCHEDULE, &cost);
        let widths = [4.0, 1.0, 2.0];
        let removable = [3.0, 0.9, 1e-3];
        let mut out = None;
        for round in 0..3 {
            let stop = run.stop(round, of_width(widths[round]), removable[round]);
            out = run.exit(round, stop, false, of_width(widths[round]));
            if out.is_some() {
                break;
            }
        }
        let out = out.expect("round 2 is at its floor");
        assert_eq!(
            (out.end, out.round, out.rounds),
            (LaneEnd::Floor, 1, 3),
            "{out:?}"
        );
        assert_eq!(out.bounds.flux.hi(), 0.5, "round 1's enclosure: {out:?}");
        let accepted = run.exit(2, LaneStop::Floor, true, of_width(2.0)).unwrap();
        assert_eq!(
            (accepted.end, accepted.round),
            (LaneEnd::Accepted(LaneStop::Floor), 2)
        );
    }
}
