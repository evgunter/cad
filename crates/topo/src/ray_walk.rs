//! **The ray walk** — the one home for how the crate's containment doors
//! cast rays from a query point `q`: the driver that tries a fixed
//! schedule's rays in order ([`walk`]), the vocabulary of what one ray
//! says ([`RayFault`]), the one ranking of what unsettled rays kept
//! ([`Evidence`]), the one sentence for a schedule no ray settled
//! ([`NoRaySettled`]), and the two readings a ray takes.
//!
//! # The driver
//!
//! A walk first asks where `q` itself stands — its boundary pre-pass,
//! whose in-band rows refuse the query. Past it, every reading is about
//! ONE RAY, so the rays are tried in schedule order and each ends one of
//! these ways ([`RayFault`]): a verdict, which ends the walk; a graze, or
//! a member that gave no ray to read, which keep nothing; a reading set
//! aside for this ray alone — an in-band margin, or a limit it definitely
//! met — which is kept as the walk's refusal should no ray decide
//! ([`Evidence`]); or a refusal of the whole query, which ends the walk
//! at once. No direction is ever picked
//! by coordinate comparison — a comparison-picked basis could diverge
//! between the f64 and interval lanes; a fixed schedule whose degenerate
//! members are detected *by predicate* cannot.
//!
//! **Why a ray-level margin only sets the ray aside.** A verdict is read
//! only off a ray whose every decision on it is definite, so setting a
//! ray aside on an in-band one — as a graze is — can turn a refusal into
//! an answer, never into a wrong one. What it does NOT license is
//! reading an in-band margin as a definite one: a ray that runs along a
//! carrier within the band may or may not meet the face on it, and is
//! set aside too, never skipped past the face.
//!
//! # The two readings
//!
//! - **Parity** ([`on_boundary`], [`ray_crossings`], [`ray_verdict`]):
//!   count proper crossings of a closed planar boundary; odd ⇒ inside.
//!   Orientation-blind, which the chord join's ring re-homing and
//!   validation's ring nesting rely on. Served to a planar loop in
//!   3-space (`splitting::containment`), a chart-space polygon
//!   (`chart_region`) and a chart polygon under its own rows
//!   (`chart_bound`).
//! - **The closest crossing** ([`Crossings`], [`advance`]): keep the
//!   crossing nearest `q` and read the material side from the ray's
//!   heading there — the 3-D sweep (`boolean::solid_contain`) and a
//!   trimmed sphere face's great circles (`boolean::sphere_region`).
//!
//! Each reader keeps what genuinely differs: its schedule and the frame
//! it builds from each member, its crossing arithmetic, what a ray that
//! meets nothing says, its K rows and its typed error.
//!
//! # Predicate rows are the CALLER's
//!
//! Every decision here funnels through [`crate::validate::decide`]
//! under a name the caller supplies ([`ParityRows`], or the row passed
//! to [`advance`] and [`Crossings::closest`]). The K ledger meters each
//! consumer's margins separately — a 3-D loop's metres and a chart
//! polygon's metres are different populations — so the shared code
//! must not pool them under one name. Sharing the walk and sharing the
//! ledger row are independent decisions, and this module makes only
//! the first.
//!
//! [`ParityRows`] carries **two** names for the boundary pre-pass,
//! because it asks two questions: `segment` decides whether a segment
//! is degenerate (the margin is the segment's own length), `boundary`
//! decides whether `q` lies on it (the margin is a point-to-segment
//! distance). One name for both would meter two populations as one.
//!
//! # The escalation seam
//!
//! The shared readings return the raw [`Indeterminate`], never a
//! caller-supplied wrapper: the caller's `escalate` stays visible beside
//! the call, and a wrapper parameter here would be the one joint where
//! `|_| RayExhausted` typechecks and relabels an escalation as
//! exhaustion. [`walk`]'s exhaustion argument receives nothing to
//! relabel.

use geom_core::{
    Band, Decide, Indeterminate, Margin, NO_DECLARATION_RECOURSE, Point2, Point3, Sign, Vec2, Vec3,
};

use crate::validate::decide;

/// **Why one ray of a walk gave no verdict.** A ray's reading is
/// `Result<V, RayFault<E>>`, `E` the reader's own typed error.
///
/// There is no `From<E>`: every `?` on a reader's error inside a ray
/// names which of the last three it is, so a stray one cannot widen a
/// ray's reading into a refusal of the query, nor narrow a refusal of
/// the query into one ray's.
///
/// **What a tighter tolerance could change sorts the last three.** A
/// reading a tighter `ε` could decide otherwise is never [`Self::Blocked`]:
/// a `Zero` grazes, an in-band margin is [`Self::InBand`], and only a
/// limit the ray met on definite decisions alone — which stay definite
/// at every tighter `ε` — blocks it.
#[derive(Debug)]
pub(crate) enum RayFault<E> {
    /// A `Zero` on one of the ray's rows — a vertex on the ray line, a
    /// crossing at `q`, a tie, a tangency, a ray within the band of a
    /// face or edge the reader cannot cross. Nothing is kept.
    Graze,
    /// This member gave no ray to read: it casts none in the reader's
    /// frame (it lies within the band of the frame's normal, or is aimed
    /// at the point itself), or its ray meets nothing a side is read
    /// from. Nothing is kept.
    Unread,
    /// An in-band margin about this ray alone, kept as evidence with its
    /// value: a tighter tolerance could decide it.
    InBand(E),
    /// A limit of the reader this ray definitely meets — a face or an
    /// edge it has no exact reading for, met on definite decisions
    /// alone, so no tolerance moves it — and another ray might miss.
    Blocked(E),
    /// A refusal of the whole query.
    Fatal(E),
}

impl<E> RayFault<E> {
    /// The same fault over another error type.
    pub(crate) fn map<F>(self, f: impl FnOnce(E) -> F) -> RayFault<F> {
        match self {
            Self::Graze => RayFault::Graze,
            Self::Unread => RayFault::Unread,
            Self::InBand(e) => RayFault::InBand(f(e)),
            Self::Blocked(e) => RayFault::Blocked(f(e)),
            Self::Fatal(e) => RayFault::Fatal(f(e)),
        }
    }

    /// One shared reading on this reader's ray: `None` grazes, and an
    /// in-band margin sets the ray aside as `escalate(diag)`.
    pub(crate) fn of<V>(
        reading: Result<Option<V>, Indeterminate>,
        escalate: impl FnOnce(Indeterminate) -> E,
    ) -> Result<V, Self> {
        reading
            .map_err(|diag| Self::InBand(escalate(diag)))?
            .ok_or(Self::Graze)
    }
}

/// **The one ranking of what a run of readings kept** when none of them
/// decided — over one point's rays ([`walk`]), and over a ladder's
/// witnesses, each of which kept its own walk's refusal
/// (`boolean::shell_witness`, `boolean::join`'s loop roles):
///
/// 1. the first [`RayFault::Blocked`] — a limit no tolerance moves,
///    true of every reading it stopped whatever the band, and its own
///    recourse is the one that frees those readings;
/// 2. else the first [`RayFault::InBand`] reading — a margin a tighter
///    tolerance would decide, carried with its value. It is not ranked
///    first because the reading it stopped went no further: a tighter
///    tolerance may decide it and still leave it to meet the limit;
/// 3. else neither, and the caller says what that means (for a walk,
///    no ray settled: [`NoRaySettled`]).
#[derive(Debug)]
pub(crate) struct Evidence<E> {
    blocked: Option<E>,
    in_band: Option<E>,
}

/// [`Evidence`]'s verdict.
#[derive(Debug)]
pub(crate) enum Ranked<E> {
    /// The first limit met.
    Blocked(E),
    /// No limit met; the first in-band reading.
    InBand(E),
    /// Neither was kept.
    Neither,
}

impl<E> Default for Evidence<E> {
    fn default() -> Self {
        Self {
            blocked: None,
            in_band: None,
        }
    }
}

impl<E> Evidence<E> {
    /// Keeps `e` as a limit met, if it is the first.
    pub(crate) fn blocked(&mut self, e: E) {
        self.blocked.get_or_insert(e);
    }

    /// Keeps `e` as an in-band reading, if it is the first.
    pub(crate) fn in_band(&mut self, e: E) {
        self.in_band.get_or_insert(e);
    }

    /// `self`'s readings first, then `later`'s: the evidence of two runs
    /// read in that order.
    pub(crate) fn then(self, later: Self) -> Self {
        Self {
            blocked: self.blocked.or(later.blocked),
            in_band: self.in_band.or(later.in_band),
        }
    }

    /// The one ranking (the type's docs).
    pub(crate) fn ranked(self) -> Ranked<E> {
        match (self.blocked, self.in_band) {
            (Some(e), _) => Ranked::Blocked(e),
            (None, Some(e)) => Ranked::InBand(e),
            (None, None) => Ranked::Neither,
        }
    }
}

/// **The walk**: `read` each ray of `rays` in order until one decides.
/// When none does, the refusal is [`Evidence`]'s ranking of what the
/// rays kept, and `exhausted()` where they kept nothing — every ray
/// grazed or gave nothing to read ([`NoRaySettled`]). A
/// [`RayFault::Fatal`] ends the walk with its error at once.
///
/// # Errors
///
/// As above.
pub(crate) fn walk<R, V, E>(
    rays: impl IntoIterator<Item = R>,
    mut read: impl FnMut(R) -> Result<V, RayFault<E>>,
    exhausted: impl FnOnce() -> E,
) -> Result<V, E> {
    let mut kept = Evidence::default();
    for ray in rays {
        match read(ray) {
            Ok(verdict) => return Ok(verdict),
            Err(RayFault::Graze | RayFault::Unread) => {}
            Err(RayFault::InBand(e)) => kept.in_band(e),
            Err(RayFault::Blocked(e)) => kept.blocked(e),
            Err(RayFault::Fatal(e)) => return Err(e),
        }
    }
    Err(match kept.ranked() {
        Ranked::Blocked(e) | Ranked::InBand(e) => e,
        Ranked::Neither => exhausted(),
    })
}

/// **The one sentence for a walk none of whose rays kept anything**, and
/// its one recourse, which each reader's exhaustion variant renders
/// after its own subject. Every walk asks it only of a point its
/// boundary pre-pass placed off the boundary, and only where each ray
/// grazed ([`RayFault::Graze`]) or gave nothing to read
/// ([`RayFault::Unread`]). The rays are the kernel's, so no declaration
/// reaches them, and neither carries a margin to size a tolerance by:
/// the geometry is the lever.
#[derive(Clone, Copy, Debug)]
pub(crate) struct NoRaySettled;

impl core::fmt::Display for NoRaySettled {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "the point is off its boundary, but no test ray from it settled where it \
             lies: each grazed the boundary, at a vertex, along an edge or at a \
             tangency, or could not be cast or read from the point. Recourse: \
             {NO_DECLARATION_RECOURSE}"
        )
    }
}

/// **Where a crossing lies along the ray from `q`**, on the caller's
/// `row`: `Positive` ahead, `Negative` behind. A `Zero` is a crossing
/// at `q`, which the boundary pre-pass has ruled out, so it grazes; an
/// in-band one sets the ray aside.
///
/// # Errors
///
/// [`RayFault::Graze`] or [`RayFault::InBand`], over the raw
/// [`Indeterminate`].
pub(crate) fn advance<T: Decide>(
    row: &'static str,
    at: Margin<T>,
    band: Band,
) -> Result<Sign, RayFault<Indeterminate>> {
    match decide(row, at, band) {
        Ok(Sign::Zero) => Err(RayFault::Graze),
        Ok(sign) => Ok(sign),
        Err(diag) => Err(RayFault::InBand(diag)),
    }
}

/// **Is a crossing apart from `q`** along a ray that has no behind — a
/// great circle, whose every point is ahead — on the caller's `row`,
/// `at` the crossing's distance from `q` (nonnegative: a magnitude, the
/// shorter way round). Only that distance is read: a `Zero` is a
/// crossing at `q`, which the boundary pre-pass has ruled out, so it
/// grazes, as in [`advance`].
///
/// # Errors
///
/// [`RayFault::Graze`] or [`RayFault::InBand`], over the raw
/// [`Indeterminate`].
pub(crate) fn apart<T: Decide>(
    row: &'static str,
    at: Margin<T>,
    band: Band,
) -> Result<(), RayFault<Indeterminate>> {
    match geom_core::k_stats::decide_magnitude(row, at, band) {
        Ok(geom_core::k_stats::Magnitude::Zero) => Err(RayFault::Graze),
        Ok(geom_core::k_stats::Magnitude::Positive) => Ok(()),
        Err(diag) => Err(RayFault::InBand(diag)),
    }
}

/// **One ray's crossings ahead of `q`, read to the closest** — the fold
/// the closest-crossing readers share. Each crossing is offered with its
/// distance along the ray in metres, already decided ahead of `q`
/// ([`advance`]), and whether its material side cannot be read there.
pub(crate) struct Crossings<T, H> {
    ahead: Vec<(T, H, bool)>,
}

impl<T: Decide, H> Crossings<T, H> {
    /// No crossing yet.
    pub(crate) const fn new() -> Self {
        Self { ahead: Vec::new() }
    }

    /// A crossing `at` metres ahead of `q`. `sideless` for one whose
    /// material side cannot be read where the ray meets it: it lands on
    /// a boundary of what it crosses (a trim's edge, an arc's end), or
    /// meets it at a tangential incidence.
    pub(crate) fn push(&mut self, at: T, hit: H, sideless: bool) {
        self.ahead.push((at, hit, sideless));
    }

    /// The closest crossing's hit, `None` when the ray met nothing.
    ///
    /// The closest is the least by the caller's `order` row. Every other
    /// crossing is then asked against IT, not against whichever was
    /// least when it was offered: a tie with the closest grazes, and an
    /// undecided one sets the ray aside as `escalate(diag, hit)`, the
    /// hit the closest was compared with; a tie between two crossings
    /// beyond it decides nothing the verdict reads. The same holds of a
    /// sideless crossing: the closest one grazes, one beyond it does not
    /// matter.
    ///
    /// # Errors
    ///
    /// [`RayFault::Graze`] or [`RayFault::InBand`].
    pub(crate) fn closest<E>(
        self,
        order: &'static str,
        band: Band,
        escalate: impl FnOnce(Indeterminate, &H) -> E,
    ) -> Result<Option<H>, RayFault<E>> {
        let gap =
            |k: usize, b: usize| decide(order, Margin::of(self.ahead[k].0 - self.ahead[b].0), band);
        let mut best = 0;
        for k in 1..self.ahead.len() {
            if gap(k, best) == Ok(Sign::Negative) {
                best = k;
            }
        }
        for k in (0..self.ahead.len()).filter(|&k| k != best) {
            match gap(k, best) {
                Ok(Sign::Positive) => {}
                // A tie, or a crossing the scan above left behind the
                // one it kept: the closest is not certain.
                Ok(Sign::Zero | Sign::Negative) => return Err(RayFault::Graze),
                Err(diag) => return Err(RayFault::InBand(escalate(diag, &self.ahead[k].1))),
            }
        }
        match self.ahead.into_iter().nth(best) {
            Some((_, _, true)) => Err(RayFault::Graze),
            Some((_, hit, false)) => Ok(Some(hit)),
            None => Ok(None),
        }
    }
}

/// The point/displacement algebra the walk needs, so that one body of
/// code serves the 2-D and 3-D consumers without either projecting
/// into the other's space. Every operation is the caller's own
/// arithmetic, unreassociated: a shared walk must not perturb a
/// margin.
///
/// The pre-pass uses all six operations; the parity walk uses two.
/// It is one trait rather than two because the two halves are one
/// procedure — the pre-pass is what licenses the walk's zero-advance
/// retry — and splitting it would let a consumer implement half the
/// space.
pub(crate) trait RaySpace<T: Decide>: Copy {
    /// The displacement between two points of this space.
    type Disp: Copy;

    /// `self - from`.
    fn disp(self, from: Self) -> Self::Disp;

    /// `self + d`.
    fn offset(self, d: Self::Disp) -> Self;

    /// `d * t`.
    fn scale(d: Self::Disp, t: T) -> Self::Disp;

    /// The inner product.
    fn dot(a: Self::Disp, b: Self::Disp) -> T;

    /// The squared norm through the `powi(2)` door: a zero-straddling
    /// enclosure squared via `Mul` gets a spurious negative lower
    /// bound, `powi` keeps the tight nonnegative one.
    fn norm_squared(d: Self::Disp) -> T;

    /// The length of `d` through this space's dimensional norm door
    /// (`Margin::norm2` / `Margin::norm3`).
    fn length_margin(d: Self::Disp) -> Margin<T>;
}

impl<T: Decide> RaySpace<T> for Point3<T> {
    type Disp = Vec3<T>;

    fn disp(self, from: Self) -> Vec3<T> {
        self - from
    }

    fn offset(self, d: Vec3<T>) -> Self {
        self + d
    }

    fn scale(d: Vec3<T>, t: T) -> Vec3<T> {
        d * t
    }

    fn dot(a: Vec3<T>, b: Vec3<T>) -> T {
        a.dot(b)
    }

    fn norm_squared(d: Vec3<T>) -> T {
        d.norm_squared()
    }

    fn length_margin(d: Vec3<T>) -> Margin<T> {
        Margin::norm3(d)
    }
}

impl<T: Decide> RaySpace<T> for Point2<T> {
    type Disp = Vec2<T>;

    fn disp(self, from: Self) -> Vec2<T> {
        self - from
    }

    fn offset(self, d: Vec2<T>) -> Self {
        self + d
    }

    fn scale(d: Vec2<T>, t: T) -> Vec2<T> {
        d * t
    }

    fn dot(a: Vec2<T>, b: Vec2<T>) -> T {
        a.dot(b)
    }

    fn norm_squared(d: Vec2<T>) -> T {
        d.norm_squared()
    }

    fn length_margin(d: Vec2<T>) -> Margin<T> {
        Margin::norm2(d)
    }
}

/// The four K rows one consumer meters its walk through. Distinct
/// names per consumer are the point: see the module docs.
///
/// **These name literals are the greppable K roster entry for the
/// walk.** A name passed as a parameter is invisible to a grep for a
/// literal at the funnel site, so this type is named explicitly in
/// `K-REPORT.md`'s inventory method. It is one of several such
/// carriers, not the only one — the restated method there enumerates
/// the classes and their measured residue. A new `ParityRows` value is
/// a roster change and belongs in that document.
pub(crate) struct ParityRows {
    /// A segment's own length — the degeneracy gate. Zero ⇒ the
    /// segment is null scaffolding and the point distance is measured
    /// exactly (the foot division below would poison on it, `w·e/0`);
    /// a certified-short-but-nonzero segment is a genuine sliver and
    /// escalates like any in-band comparison.
    pub segment: &'static str,
    /// The distance from `q` to a closed segment — perpendicular at
    /// an interior foot, endpoint otherwise. Zero ⇒ on the boundary.
    pub boundary: &'static str,
    /// A vertex's signed offset from the ray line. Zero ⇒ grazing ⇒
    /// next ray.
    pub side: &'static str,
    /// A straddling segment's crossing advance along the ray. Zero
    /// would be a crossing at `q` itself, contradicting the boundary
    /// pre-pass ⇒ next ray.
    pub advance: &'static str,
}

/// The boundary pre-pass: is `q` within the band of any closed
/// segment of the cycle `verts`?
///
/// Run once, before any ray: a `true` here is the `OnBoundary`
/// verdict, and it is also what licenses [`ray_verdict`] to treat a
/// zero advance as a grazing retry rather than a real answer.
///
/// # Errors
///
/// The raw [`Indeterminate`] of an in-band margin on the
/// [`ParityRows::segment`] or [`ParityRows::boundary`] row. It is
/// returned rather than wrapped: the caller's own
/// `.map_err(escalate)` stays visible at the call site, and this
/// module has no way to drop the diagnostic on the way out.
pub(crate) fn on_boundary<T, P>(
    verts: &[P],
    q: P,
    rows: &ParityRows,
    band: Band,
) -> Result<bool, Indeterminate>
where
    T: Decide,
    P: RaySpace<T>,
{
    let n = verts.len();
    for i in 0..n {
        if on_segment(verts[i], verts[(i + 1) % n], q, rows, band)? {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Is `q` within the band of the closed segment `[a, b]`? One step of
/// [`on_boundary`], for a caller whose cycle is not all segments — a
/// loop with arcs asks it of its straight edges only, since an arc's
/// chord is not boundary.
///
/// # Errors
///
/// As [`on_boundary`].
pub(crate) fn on_segment<T, P>(
    a: P,
    b: P,
    q: P,
    rows: &ParityRows,
    band: Band,
) -> Result<bool, Indeterminate>
where
    T: Decide,
    P: RaySpace<T>,
{
    on_segment_margin(a, b, q, rows, band).map(|on| on.is_some())
}

/// [`on_segment`], answering the margin that decided `q` on the segment
/// (`None` off it).
///
/// # Errors
///
/// As [`on_segment`].
pub(crate) fn on_segment_margin<T, P>(
    a: P,
    b: P,
    q: P,
    rows: &ParityRows,
    band: Band,
) -> Result<Option<geom_core::MarginDiag>, Indeterminate>
where
    T: Decide,
    P: RaySpace<T>,
{
    let e = b.disp(a);
    let w = q.disp(a);
    let len2 = P::norm_squared(e);
    let gap = match decide(rows.segment, P::length_margin(e), band)? {
        Sign::Zero => w,
        _ => {
            // Foot parameter clamped to the span — evaluation lane
            // (no comparison): t = clamp(w·e / e·e, 0, 1) via
            // min/max.
            let t = (P::dot(w, e) / len2).max(T::zero()).min(T::one());
            let foot = a.offset(P::scale(e, t));
            q.disp(foot)
        }
    };
    // The distance stays a DISPLACEMENT until the door, so the
    // norm is taken inside `Margin::norm2`/`norm3` rather than
    // handed to `Margin::of` already rooted.
    // Zero ⇒ on boundary; Positive (Negative unreachable for a
    // distance) ⇒ strictly off this segment.
    let decided = geom_core::k_stats::decide_reported(rows.boundary, P::length_margin(gap), band)?;
    Ok((decided.sign == Sign::Zero).then_some(decided.margin))
}

/// One schedule member's parity walk, in the in-plane orthonormal
/// frame `(d, side_axis)` the caller derived for that member: `d` is
/// the ray direction, `side_axis` its in-plane perpendicular.
///
/// `Ok(None)` is a **graze** — this ray is unusable and the caller
/// must try the next schedule member (or report its typed exhaustion
/// once the schedule runs out). `Ok(Some(inside))` is the verdict.
///
/// # Errors
///
/// The raw [`Indeterminate`] of an in-band margin on the
/// [`ParityRows::side`] or [`ParityRows::advance`] row — see
/// [`on_boundary`] on why it is not wrapped here.
pub(crate) fn ray_verdict<T, P>(
    verts: &[P],
    q: P,
    d: P::Disp,
    side_axis: P::Disp,
    rows: &ParityRows,
    band: Band,
) -> Result<Option<bool>, Indeterminate>
where
    T: Decide,
    P: RaySpace<T>,
{
    Ok(ray_crossings(verts, q, d, side_axis, rows, band, |_| true)?
        .map(|crossings| !crossings.is_multiple_of(2)))
}

/// [`ray_verdict`]'s count, over the cycle's segments `i → i + 1` for
/// which `counted(i)` holds — every vertex is still asked the side row,
/// so a vertex on the ray line grazes whichever edges it bounds. A
/// loop with arcs counts its straight edges here and crosses its arcs
/// on their own carriers.
///
/// # Errors
///
/// As [`ray_verdict`].
pub(crate) fn ray_crossings<T, P>(
    verts: &[P],
    q: P,
    d: P::Disp,
    side_axis: P::Disp,
    rows: &ParityRows,
    band: Band,
    counted: impl Fn(usize) -> bool,
) -> Result<Option<usize>, Indeterminate>
where
    T: Decide,
    P: RaySpace<T>,
{
    let n = verts.len();

    // Signed frame coordinates of each vertex relative to q.
    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);
    let mut sides = Vec::with_capacity(n);
    for p in verts {
        let w = p.disp(q);
        xs.push(P::dot(w, d));
        let y = P::dot(w, side_axis);
        ys.push(y);
        match decide(rows.side, Margin::of(y), band)? {
            Sign::Zero => return Ok(None), // vertex on the ray line
            s => sides.push(s),
        }
    }

    let mut crossings = 0usize;
    for i in 0..n {
        let j = (i + 1) % n;
        if !counted(i) || sides[i] == sides[j] {
            continue; // not a segment of this count, or no straddle
        }
        // Straddling: the crossing's advance along the ray.
        let advance = Margin::over_lever(xs[i] * ys[j] - xs[j] * ys[i], ys[j] - ys[i]);
        match decide(rows.advance, advance, band)? {
            Sign::Positive => crossings += 1,
            Sign::Negative => {}
            // A crossing at q itself contradicts the boundary
            // pre-pass — treat as a graze and retry.
            Sign::Zero => return Ok(None),
        }
    }
    Ok(Some(crossings))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const ROWS: ParityRows = ParityRows {
        segment: "test_ray_walk_segment",
        boundary: "test_ray_walk_boundary",
        side: "test_ray_walk_side",
        advance: "test_ray_walk_advance",
    };

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    fn lift(p: Point2<f64>) -> Point3<f64> {
        Point3::new(p.x, p.y, 0.0)
    }

    /// A square carrying a **repeated vertex**, so one segment is
    /// zero-length. Neither consumer's fixtures build one, and it is
    /// the arm where the clamped foot would divide by `|e|² = 0` — the
    /// pre-pass has to take the endpoint distance instead of poisoning.
    fn null_scaffold() -> [Point2<f64>; 5] {
        [
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(1.0, 0.0), // null scaffolding
            Point2::new(1.0, 1.0),
            Point2::new(0.0, 1.0),
        ]
    }

    #[test]
    fn a_zero_length_segment_measures_the_endpoint_distance_in_both_spaces() {
        let poly = null_scaffold();
        let lifted: Vec<Point3<f64>> = poly.iter().map(|p| lift(*p)).collect();

        // Interior: the null segment must not poison the verdict.
        let inside = Point2::new(0.5, 0.5);
        assert_eq!(on_boundary(&poly, inside, &ROWS, band()), Ok(false));
        assert_eq!(on_boundary(&lifted, lift(inside), &ROWS, band()), Ok(false));

        // The null segment's own point: distance zero, on boundary —
        // reached through the degenerate arm, not the foot division.
        let at_null = Point2::new(1.0, 0.0);
        assert_eq!(on_boundary(&poly, at_null, &ROWS, band()), Ok(true));
        assert_eq!(on_boundary(&lifted, lift(at_null), &ROWS, band()), Ok(true));

        // Off the boundary and nearest to the null vertex: the
        // endpoint distance answers, and it is finite.
        let off = Point2::new(2.0, -1.0);
        assert_eq!(on_boundary(&poly, off, &ROWS, band()), Ok(false));
        assert_eq!(on_boundary(&lifted, lift(off), &ROWS, band()), Ok(false));
    }

    /// A vertex sitting exactly on the `+x` ray line from `(1, 1)`:
    /// that member must GRAZE and a later one must decide. Every
    /// consumer fixture today is decided by its FIRST schedule member,
    /// so nothing else in the crate exercises the retry.
    fn grazing_pentagon() -> [Point2<f64>; 5] {
        [
            Point2::new(0.0, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(2.0, 1.0), // on the +x ray from (1, 1)
            Point2::new(2.0, 2.0),
            Point2::new(0.0, 2.0),
        ]
    }

    #[test]
    fn the_first_ray_grazes_and_a_later_one_decides_in_2d() {
        let poly = grazing_pentagon();
        let q = Point2::new(1.0, 1.0);
        assert_eq!(on_boundary(&poly, q, &ROWS, band()), Ok(false));

        let x = Vec2::new(1.0, 0.0);
        let graze = ray_verdict(&poly, q, x, Vec2::new(0.0, 1.0), &ROWS, band());
        assert_eq!(graze, Ok(None), "+x must graze the (2, 1) vertex");

        let y = Vec2::new(0.0, 1.0);
        let sy = Vec2::new(-1.0, 0.0);
        assert_eq!(ray_verdict(&poly, q, y, sy, &ROWS, band()), Ok(Some(true)));

        let out = Point2::new(3.0, 1.5);
        assert_eq!(
            ray_verdict(&poly, out, y, sy, &ROWS, band()),
            Ok(Some(false))
        );
    }

    #[test]
    fn the_first_ray_grazes_and_a_later_one_decides_in_3d() {
        let poly: Vec<Point3<f64>> = grazing_pentagon().iter().map(|p| lift(*p)).collect();
        let q = Point3::new(1.0, 1.0, 0.0);
        assert_eq!(on_boundary(&poly, q, &ROWS, band()), Ok(false));

        // The frame the 3-D consumer builds: side_axis = normal x d.
        let x = Vec3::new(1.0, 0.0, 0.0);
        let graze = ray_verdict(&poly, q, x, Vec3::new(0.0, 1.0, 0.0), &ROWS, band());
        assert_eq!(graze, Ok(None), "+x must graze the (2, 1, 0) vertex");

        let y = Vec3::new(0.0, 1.0, 0.0);
        let sy = Vec3::new(-1.0, 0.0, 0.0);
        assert_eq!(ray_verdict(&poly, q, y, sy, &ROWS, band()), Ok(Some(true)));

        let out = Point3::new(3.0, 1.5, 0.0);
        assert_eq!(
            ray_verdict(&poly, out, y, sy, &ROWS, band()),
            Ok(Some(false))
        );
    }

    /// One procedure in two spaces, so on a lifted planar fixture they
    /// must agree not merely on the verdict but on the GRAZE — the
    /// thing a reassociation moves first.
    #[test]
    fn the_two_spaces_agree_ray_for_ray_on_the_lifted_fixture() {
        let poly = grazing_pentagon();
        let lifted: Vec<Point3<f64>> = poly.iter().map(|p| lift(*p)).collect();
        let schedule = [
            (1.0, 0.0),
            (0.0, 1.0),
            (0.5, 1.0),
            (1.0, -0.5),
            (-0.75, 1.0),
        ];
        for q in [(1.0, 1.0), (0.5, 1.5), (3.0, 1.5), (-1.0, -1.0)] {
            let q2 = Point2::new(q.0, q.1);
            let q3 = lift(q2);
            assert_eq!(
                on_boundary(&poly, q2, &ROWS, band()),
                on_boundary(&lifted, q3, &ROWS, band()),
                "pre-pass at {q:?}"
            );
            for (dx, dy) in schedule {
                let d2 = Vec2::new(dx, dy).normalize();
                let s2 = Vec2::new(-d2.y, d2.x);
                let d3 = Vec3::new(d2.x, d2.y, 0.0);
                let s3 = Vec3::new(s2.x, s2.y, 0.0);
                assert_eq!(
                    ray_verdict(&poly, q2, d2, s2, &ROWS, band()),
                    ray_verdict(&lifted, q3, d3, s3, &ROWS, band()),
                    "member ({dx}, {dy}) at {q:?}"
                );
            }
        }
    }

    // ---- The driver ----

    /// One ray's scripted outcome, for the driver rows.
    #[derive(Clone, Copy)]
    enum Says {
        Verdict(u8),
        Graze,
        Unread,
        InBand(u8),
        Blocked(u8),
        Fatal(u8),
    }

    fn run(rays: &[Says]) -> Result<u8, u8> {
        walk(
            rays.iter().copied(),
            |r| match r {
                Says::Verdict(v) => Ok(v),
                Says::Graze => Err(RayFault::Graze),
                Says::Unread => Err(RayFault::Unread),
                Says::InBand(e) => Err(RayFault::InBand(e)),
                Says::Blocked(e) => Err(RayFault::Blocked(e)),
                Says::Fatal(e) => Err(RayFault::Fatal(e)),
            },
            || 0,
        )
    }

    /// **The one ranking**: a limit met outranks an in-band reading
    /// whichever ray met it first, and of each kind the FIRST is kept.
    #[test]
    fn the_walk_ranks_a_limit_before_an_in_band_reading_and_keeps_the_first() {
        use Says::*;
        assert_eq!(run(&[InBand(1), Blocked(2)]), Err(2));
        assert_eq!(run(&[Blocked(2), InBand(1)]), Err(2));
        assert_eq!(run(&[InBand(1), Graze, InBand(2)]), Err(1));
        assert_eq!(run(&[Blocked(3), Blocked(4), InBand(1)]), Err(3));
        assert_eq!(run(&[Unread, InBand(5), Graze, InBand(6)]), Err(5));
        // Nothing kept: the caller's exhaustion.
        assert_eq!(run(&[Graze, Unread, Graze]), Err(0));
        assert_eq!(run(&[]), Err(0));
    }

    /// **A verdict ends the walk, and so does a refusal of the query** —
    /// whatever the rays before it kept, and before any ray after it is
    /// read.
    #[test]
    fn a_verdict_or_a_fatal_refusal_ends_the_walk() {
        use Says::*;
        assert_eq!(run(&[Blocked(2), InBand(1), Graze, Verdict(7)]), Ok(7));
        assert_eq!(run(&[InBand(1), Fatal(9), Verdict(7)]), Err(9));
        assert_eq!(run(&[Verdict(7), Fatal(9)]), Ok(7));
    }

    /// [`Evidence`] over two runs read in order: the first run's of each
    /// kind, then the second's, ranked as one walk's.
    #[test]
    fn evidence_over_two_runs_keeps_each_kinds_first() {
        let rank = |e: Evidence<u8>| match e.ranked() {
            Ranked::Blocked(e) => (1, e),
            Ranked::InBand(e) => (2, e),
            Ranked::Neither => (3, 0),
        };
        let mut a = Evidence::default();
        a.in_band(1);
        let mut b = Evidence::default();
        b.blocked(2);
        b.in_band(3);
        assert_eq!(rank(a.then(b)), (1, 2));
        let (mut a, mut b) = (Evidence::default(), Evidence::default());
        a.in_band(1);
        b.in_band(3);
        assert_eq!(rank(a.then(b)), (2, 1));
        assert_eq!(rank(Evidence::<u8>::default()), (3, 0));
    }

    // ---- The fold ----

    fn fold(crossings: &[(f64, u8, bool)]) -> Result<Option<u8>, &'static str> {
        let mut c = Crossings::new();
        for &(at, hit, sideless) in crossings {
            c.push(at, hit, sideless);
        }
        c.closest("test_ray_walk_order", band(), |_, _| ())
            .map_err(|f| match f {
                RayFault::Graze => "graze",
                RayFault::InBand(()) => "in band",
                _ => "other",
            })
    }

    /// **The closest crossing answers**, wherever it was offered.
    #[test]
    fn the_closest_crossing_answers() {
        assert_eq!(fold(&[]), Ok(None));
        assert_eq!(fold(&[(2.0, 1, false)]), Ok(Some(1)));
        assert_eq!(
            fold(&[(3.0, 1, false), (1.0, 2, false), (2.0, 3, false)]),
            Ok(Some(2))
        );
    }

    /// **A tie with the closest grazes; a tie beyond it does not.** The
    /// later pair is asked against the closest, not against the running
    /// best when it was offered: `[2, 2, 1]` reads its first two as a tie
    /// only on the running best, and its closest is `1`.
    #[test]
    fn a_tie_grazes_only_at_the_closest() {
        assert_eq!(fold(&[(1.0, 1, false), (1.0, 2, false)]), Err("graze"));
        assert_eq!(
            fold(&[(2.0, 1, false), (1.0, 2, false), (1.0, 3, false)]),
            Err("graze")
        );
        assert_eq!(
            fold(&[(2.0, 1, false), (2.0, 2, false), (1.0, 3, false)]),
            Ok(Some(3))
        );
        assert_eq!(
            fold(&[(1.0, 1, false), (2.0, 2, false), (2.0, 3, false)]),
            Ok(Some(1))
        );
        // Within the band's zero is a tie; in its band, undecided.
        assert_eq!(
            fold(&[(1.0, 1, false), (1.0 + 5e-10, 2, false)]),
            Err("graze")
        );
        assert_eq!(
            fold(&[(1.0, 1, false), (1.0 + 5e-9, 2, false)]),
            Err("in band")
        );
    }

    /// **A sideless crossing grazes only as the closest**: one beyond a
    /// definite closest crossing does not matter (the far-edge rule).
    #[test]
    fn a_sideless_crossing_grazes_only_at_the_closest() {
        assert_eq!(fold(&[(1.0, 1, true)]), Err("graze"));
        assert_eq!(fold(&[(2.0, 1, false), (1.0, 2, true)]), Err("graze"));
        assert_eq!(fold(&[(1.0, 1, false), (2.0, 2, true)]), Ok(Some(1)));
        assert_eq!(
            fold(&[(2.0, 1, true), (1.0, 2, false), (3.0, 3, true)]),
            Ok(Some(2))
        );
    }

    /// [`advance`] and [`apart`]: a crossing at `q` grazes, an in-band one
    /// is set aside, and only [`advance`] reads a side of `q`.
    #[test]
    fn a_crossing_at_q_grazes() {
        let a = |m: f64| advance("test_ray_walk_advance", Margin::of(m), band());
        assert!(matches!(a(1.0), Ok(Sign::Positive)));
        assert!(matches!(a(-1.0), Ok(Sign::Negative)));
        assert!(matches!(a(5e-10), Err(RayFault::Graze)));
        assert!(matches!(a(5e-9), Err(RayFault::InBand(_))));
        let p = |m: f64| apart("test_ray_walk_advance", Margin::of(m), band());
        assert!(matches!(p(1.0), Ok(())));
        assert!(matches!(p(5e-10), Err(RayFault::Graze)));
        assert!(matches!(p(5e-9), Err(RayFault::InBand(_))));
    }
}
