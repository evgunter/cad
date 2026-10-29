//! **The torus pairs' closed-form classifications** ([`super::torus`]):
//! the components of a sphere, a parallel- or square-axis cylinder, and
//! a parallel-axis torus against a ring torus, each listed with whether
//! it winds about each carrier and, where the arm names one, a witness.
//! Each reads a pose whose critical gap was decided first, so a margin
//! here is classification, and one decided `Zero` or undecided refuses
//! the count ([`ArmRefusal`]); where the gap decided a tangency only the
//! touch readings run.

use geom_core::k_stats::decide;
use geom_core::{
    Band, Circles, Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, Vec2, Vec3,
};

use super::{ClassCarrier, ComponentShape, SectionComponent};
use crate::intersect::decide_across;

/// **Why an arm did not count the section.**
#[derive(Clone, Copy, Debug)]
pub(super) enum ArmRefusal {
    /// A margin decided `Zero`, or undecided: the components pinch.
    Pinch(Pinch),
    /// A crossing of the carriers the closed form cannot place on either
    /// side of a line it reads the section by, named.
    Unordered(&'static str),
}

impl From<Pinch> for ArmRefusal {
    fn from(p: Pinch) -> Self {
        Self::Pinch(p)
    }
}

/// **A margin a row reading could not take as a side**: decided `Zero`
/// (`undecided` `None`), or undecided, with its diagnostics.
#[derive(Clone, Copy, Debug)]
pub struct Pinch {
    /// The row, or the predicate left undecided.
    pub name: &'static str,
    /// The margin's place in the list [`signs`] decided.
    pub index: usize,
    /// The diagnostics, where the margin was undecided.
    pub undecided: Option<Indeterminate>,
}

impl Pinch {
    /// A pinch named `name` with no margin of its own: a reading the arm
    /// takes as a tangency without deciding one.
    const fn named(name: &'static str) -> Self {
        Self {
            name,
            index: 0,
            undecided: None,
        }
    }
}

fn sign<T: Decide>(name: &'static str, m: Margin<T>, band: Band) -> Option<Sign> {
    decide(name, m, band).ok()
}

/// Reads every row by `read`, which names it and gives its verdict; the
/// first `Zero` or undecided one is the pinch, named by the row left
/// undecided where one was.
fn first_pinch<R, const N: usize>(
    rows: [R; N],
    read: impl Fn(R) -> (&'static str, Result<Sign, Indeterminate>),
) -> Result<[bool; N], Pinch> {
    let mut out = [false; N];
    for (index, (slot, row)) in out.iter_mut().zip(rows).enumerate() {
        let (name, verdict) = read(row);
        let (name, undecided) = match verdict {
            Ok(Sign::Positive) => {
                *slot = true;
                continue;
            }
            Ok(Sign::Negative) => continue,
            Ok(Sign::Zero) => (name, None),
            Err(diag) => (diag.predicate.unwrap_or(name), Some(diag)),
        };
        return Err(Pinch {
            name,
            index,
            undecided,
        });
    }
    Ok(out)
}

/// **Decides every margin in `ms`**, the first `Zero` or undecided one
/// the pinch ([`first_pinch`]).
pub fn signs<T: Decide, const N: usize>(
    ms: [(&'static str, T); N],
    band: Band,
) -> Result<[bool; N], Pinch> {
    first_pinch(ms, |(name, m)| (name, decide(name, Margin::of(m), band)))
}

/// [`signs`], each row `(far, floor, datum)` read across the reach beside
/// `slack` ([`decide_across`]).
pub fn signs_across<T: Decide, const N: usize>(
    rows: [(&'static str, &'static str, T); N],
    slack: T,
    band: Band,
) -> Result<[bool; N], Pinch> {
    first_pinch(rows, |(far, floor, m)| {
        (far, decide_across([far, floor], m, slack, band))
    })
}

/// **One loop a closed-form arm lists**: whether it winds about the
/// torus and about the partner (it is essential there), in that order
/// (the pose puts them in the order the section was asked of), one point
/// of it where the arm names one, and the pair's carrier.
fn arm_loop<T: Real>(
    [torus, partner]: [bool; 2],
    witness: Option<Point3<T>>,
    carrier: &ClassCarrier<T>,
) -> SectionComponent<T> {
    SectionComponent {
        shape: ComponentShape::Loop,
        winds: [torus, partner],
        witness,
        carrier: carrier.clone(),
    }
}

/// One inessential loop through `witness`.
fn lone<T: Real>(witness: Point3<T>, carrier: &ClassCarrier<T>) -> Vec<SectionComponent<T>> {
    vec![arm_loop([false, false], Some(witness), carrier)]
}

/// Two loops, each essential as `winds` says.
fn essential_pair<T: Real>(
    winds: [bool; 2],
    carrier: &ClassCarrier<T>,
) -> Vec<SectionComponent<T>> {
    vec![arm_loop(winds, None, carrier); 2]
}

// ---------------------------------------------------------------------
// Torus × sphere
// ---------------------------------------------------------------------

/// The sphere arm's near-tube margin: the sphere cuts the tube circle
/// `T₊` of the meridian half-plane `Π` toward its centre.
pub(super) const SPHERE_NEAR: &str = "section_torus_sphere_near_tube";
/// The sphere arm's far-tube margin: it cuts `T₋`.
pub(super) const SPHERE_FAR: &str = "section_torus_sphere_far_tube";

/// **The torus `(c, a, R, r)` against the sphere `(cs, ρ)` in its
/// meridian half-plane**: `s` the centre's distance from the axis and
/// `ca` its height, with `Π`'s map to space; the centre's distances from
/// the tube centres `(±R, 0)`.
#[derive(Clone, Copy)]
pub(super) struct SphereMeridian<T: Real> {
    c: Point3<T>,
    a: Vec3<T>,
    perp: Vec3<T>,
    pub(super) s: T,
    pub(super) ca: T,
    pub(super) d: [T; 2],
}

impl<T: Real> SphereMeridian<T> {
    pub(super) fn of(c: Point3<T>, a: Vec3<T>, big_r: T, cs: Point3<T>) -> Self {
        let w = cs - c;
        let ca = w.dot(a);
        let (perp, s) = super::square_part(w, a);
        let dist = |x: T| Vec2::new(x, ca).norm();
        Self {
            c,
            a,
            perp,
            s,
            ca,
            d: [dist(s - big_r), dist(s + big_r)],
        }
    }

    /// `Π`'s point `(x, y)`: `x` along the centre's part square to the
    /// axis, `y` along the axis.
    pub(super) fn at(&self, x: T, y: T) -> Point3<T> {
        self.c + self.perp / self.s * x + self.a * y
    }
}

/// **Torus × sphere, off the torus axis.** In `Π` let `T₊`, `T₋` be the
/// tube circles at the near and far core points. The section has roots
/// at `u′ = 0` iff the sphere cuts `T₊`, and at `u′ = π` iff it cuts
/// `T₋`. Both cut: two `(1,0)` curves (essential on the torus). One cut:
/// a single null oval, whose points on `Π` are the sphere's crossings
/// with that tube circle. Neither: two `(0,1)` curves when the sphere
/// separates `T₊` from `T₋`, else none.
pub(super) fn sphere_arm<T: Decide>(
    [big_r, r]: [T; 2],
    m: &SphereMeridian<T>,
    rho: T,
    band: Band,
) -> Result<SphereParts<T>, Pinch> {
    let [d_near, d_far] = m.d;
    let cuts = |d: T| (d + r - rho).min(rho - (d - r).abs());
    let [near, far] = signs(
        [(SPHERE_NEAR, cuts(d_near)), (SPHERE_FAR, cuts(d_far))],
        band,
    )?;
    Ok(match (near, far) {
        (true, true) => SphereParts::AroundAxis,
        (false, false) => {
            let [between] = signs(
                [(
                    "section_torus_sphere_between",
                    (rho - d_near - r).min(d_far - r - rho),
                )],
                band,
            )?;
            if between {
                SphereParts::AroundTube
            } else {
                SphereParts::Parts(Vec::new())
            }
        }
        (true, false) | (false, true) => {
            let sigma = if near { T::one() } else { -T::one() };
            let d = if near { d_near } else { d_far };
            // Circle against circle in Π: T_σ centred (σR, 0) radius r,
            // the sphere's great circle centred (s, c_a) radius ρ.
            let (ex, ey) = ((m.s - sigma * big_r) / d, m.ca / d);
            let along = (d.powi(2) + r.powi(2) - rho.powi(2)) / (d + d);
            let half = ((r - along) * (r + along)).sqrt();
            SphereParts::Parts(lone(
                m.at(
                    sigma * big_r + ex * along - ey * half,
                    ey * along + ex * half,
                ),
                &ClassCarrier::General,
            ))
        }
    })
}

/// What [`sphere_arm`] counts.
pub(super) enum SphereParts<T: Real> {
    /// Two `(1,0)` curves, essential on the torus.
    AroundAxis,
    /// Two `(0,1)` curves, essential on the torus, mirror images across
    /// `Π`.
    AroundTube,
    /// Every other count, each part listed.
    Parts(Vec<SectionComponent<T>>),
}

/// **Where a sphere tangent to the tube circle `T_σ` the pinch names
/// touches the torus, if at one point**: `(x, y)` in `Π`.
///
/// The section is the level `ρ` of `f = |x − cs|` on the torus. With
/// `cs` off the axis (`s > 0`), `f`'s critical points are the four
/// points of `T₊` and `T₋` on their normals through `cs`: on `T_σ`, its
/// nearest point to `cs` at `|d_σ − r|` and its farthest at `d_σ + r`.
/// The sphere is tangent at whichever of the two stands nearer `ρ`. That
/// point is a touch when its value is `f`'s strict minimum or maximum
/// over the torus, clear of the other three by a decided margin: in
/// every pose the band admits, `{f ≤ ρ}` (or `{f ≥ ρ}`) is then one disc
/// about it or empty, on which the carriers stand within the band of
/// each other. It must also stand on the tube's outer half, where the
/// torus is elliptic. A centre on the core (`d_σ = 0`) has every point of
/// `T_σ` on a normal through it: no one point, and the offset decides
/// that before anything divides by it.
///
/// Which of the two is a choice, not a decision: `ρ` stands within the
/// band of one of them and they are `2·min(d_σ, r)` apart, so wherever
/// the choice is in doubt the other stands within the band too and the
/// extreme margin refuses.
pub(super) fn sphere_touch<T: Decide>(
    tan: Pinch,
    [big_r, r]: [T; 2],
    m: &SphereMeridian<T>,
    rho: T,
    band: Band,
) -> Option<Point3<T>> {
    let [d_near, d_far] = m.d;
    let (sigma, d, d_other) = if tan.index == 0 {
        (T::one(), d_near, d_far)
    } else {
        (-T::one(), d_far, d_near)
    };
    let positive = |name, v: T| sign(name, Margin::of(v), band) == Some(Sign::Positive);
    if !positive("section_torus_sphere_touch_offset", d) {
        return None;
    }
    // The midpoint of T_σ's two values is max(d_σ, r); `kappa` is +1 at
    // the nearest point, −1 at the farthest.
    let split = rho - d.max(r);
    let (nearest, farthest) = ((d - r).abs(), d + r);
    let kappa = split.select_le_zero(T::one(), -T::one());
    let value = split.select_le_zero(nearest, farthest);
    let [o1, o2, o3] = [
        split.select_le_zero(farthest, nearest),
        (d_other - r).abs(),
        d_other + r,
    ]
    .map(|v| kappa * (v - value));
    // T_σ's point along ±(cs − C_σ), C_σ = (σR, 0).
    let (ux, uy) = ((m.s - sigma * big_r) / d, m.ca / d);
    let (x, y) = (sigma * big_r + kappa * r * ux, kappa * r * uy);
    (positive("section_torus_sphere_touch_extreme", o1.min(o2).min(o3))
        && positive("section_torus_sphere_touch_elliptic", sigma * x - big_r))
    .then(|| m.at(x, y))
}

// ---------------------------------------------------------------------
// Torus × parallel-axis cylinder
// ---------------------------------------------------------------------

/// The parallel wall's side of the torus axis: its circle holds the axis.
const WALL_ROUND_AXIS: &str = "section_torus_offset_wall_round_axis";

/// The parallel wall's touch row: its nearest ruling on the torus's
/// outer equator.
pub(super) const WALL_NEAR_OUTER: &str = "section_torus_offset_wall_near_outer";

/// **Torus × a cylinder whose axis is parallel to the torus's**, at
/// offset `e` in the direction `toward`, each row read across the
/// `slack` the pose admitted: with `ρ ∈ [|e − ρc|, e + ρc]` against
/// `[R − r, R + r]`, inside gives two loops encircling the cylinder,
/// across one bound a single null loop (witness at the extreme ruling),
/// across both two `(0,1)` loops.
#[allow(clippy::too_many_arguments)]
pub(super) fn parallel_cylinder_arm<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    [big_r, r]: [T; 2],
    (e, slack): (T, T),
    toward: Vec3<T>,
    rc: T,
    band: Band,
) -> Result<Vec<SectionComponent<T>>, Pinch> {
    let (rho_min, rho_max) = ((e - rc).abs(), e + rc);
    let (lo, hi) = (big_r - r, big_r + r);
    let [min_over_lo, min_under_hi, max_over_lo, max_under_hi] = signs_across(
        [
            (
                "section_torus_offset_wall_near_inner",
                "section_torus_offset_wall_near_inner_floor",
                rho_min - lo,
            ),
            (
                WALL_NEAR_OUTER,
                "section_torus_offset_wall_near_outer_floor",
                hi - rho_min,
            ),
            (
                "section_torus_offset_wall_far_inner",
                "section_torus_offset_wall_far_inner_floor",
                rho_max - lo,
            ),
            (
                "section_torus_offset_wall_far_outer",
                "section_torus_offset_wall_far_outer_floor",
                hi - rho_max,
            ),
        ],
        slack,
        band,
    )?;
    let height = |rho: T| {
        let dr = rho - big_r;
        ((r - dr.abs()) * (r + dr.abs())).sqrt()
    };
    Ok(
        match (min_over_lo, min_under_hi, max_over_lo, max_under_hi) {
            // The whole ρ-range inside the tube's shadow: two loops, an
            // upper and a lower, each a graph over the wall's circle, so
            // each encircles the cylinder. On the torus each lies on one
            // half of the tube, an annulus about the axis, and turns once
            // about the axis exactly where the wall's circle holds it,
            // `e < ρc`: then it is a longitude, essential, and otherwise
            // null. `|ρc − e| = ρ_min` stands past `R − r` by the inner
            // row just read, so its sign is decided.
            (true, true, true, true) => {
                let round_axis = match sign(WALL_ROUND_AXIS, Margin::of(rc - e), band) {
                    Some(Sign::Positive) => true,
                    Some(Sign::Negative) => false,
                    _ => return Err(Pinch::named(WALL_ROUND_AXIS)),
                };
                essential_pair([round_axis, true], &ClassCarrier::General)
            }
            // Wholly inside the hole, or wholly outside the torus.
            (_, _, false, _) | (_, false, _, _) => Vec::new(),
            // Across both bounds: two `(0,1)` loops on the torus.
            (false, true, true, false) => essential_pair([true, false], &ClassCarrier::General),
            // Across the inner bound only: the null loop at θ = 0.
            (false, true, true, true) => lone(
                c + toward * rho_max + a * height(rho_max),
                &ClassCarrier::General,
            ),
            // Across the outer bound only: the null loop at θ = π.
            (true, true, true, false) => lone(
                c + toward * (e - rc) + a * height(rho_min),
                &ClassCarrier::General,
            ),
        },
    )
}

/// **The parallel wall's touch**: its nearest ruling on the outer
/// equator, the rest of the wall beyond it. That is the extreme of the
/// distance from the wall's axis over the torus (its minimum beside the
/// torus, its maximum about it), at an elliptic point, clear of the next
/// critical value by `2r` beside the torus and by `min(2r, 2e)` about
/// it, both decided upstream: `r` by the ring guard, `e` by the pose's
/// `Positive` offset floor, the one pose that reaches this arm.
pub(super) fn parallel_cylinder_touch<T: Real>(
    tan: Pinch,
    c: Point3<T>,
    (e, toward): (T, Vec3<T>),
    rc: T,
) -> Option<Point3<T>> {
    (tan.undecided.is_none() && tan.name == WALL_NEAR_OUTER).then(|| c + toward * (e - rc))
}

// ---------------------------------------------------------------------
// Torus × parallel-axis torus
// ---------------------------------------------------------------------

/// One strand of the matching curve `W` of [`parallel_torus_arm`].
#[derive(Clone, Copy)]
struct Strand {
    /// The point stands on the torus's outer sheet.
    outer: bool,
    /// It stands on the partner's outer sheet.
    outer2: bool,
    /// The loop runs up the strand.
    up: bool,
}

const fn strand(outer: bool, outer2: bool, up: bool) -> Strand {
    Strand { outer, outer2, up }
}

const PARALLEL_TORI_ORDER: &str = "section_torus_parallel_tori_order";

/// **Torus × a torus whose axis is parallel to its own**, the partner's
/// axis at offset `e` along `toward` and its centre at height `h`, each
/// row read across the `slack` the pose admitted. `rings` is
/// `[[R, r], [R₂, r₂]]`.
///
/// Both tori are symmetric under the reflection `σ` through the plane
/// `Π` holding both axes. A point of the section has heights, and
/// distances `ρ`, `ρ₂` from the two axes, lying on the two meridian
/// circles `(ρ − R)² + y² = r²` and `(ρ₂ − R₂)² + (y − h)² = r₂²`; given
/// those, the point is one of a mirror pair across `Π` when
/// `|ρ − ρ₂| < e < ρ + ρ₂` and on `Π` at equality. So the section is two
/// mirror copies of the part of the matching curve
/// `W = {(ρ, ρ₂, y)}` inside that strict triangle, glued at the points
/// of `Π` on both tori, which are the crossings of the tori's meridian
/// circles in `Π` (closed form, at most eight).
///
/// `W`'s strands are the sheets `ρ = R ± √(r² − y²)` and the partner's
/// over the heights both tubes span, joined where that span ends at a
/// tube's top or bottom: two loops where one tube owns both ends, one
/// loop of four strands otherwise. `W` is smooth except where a tube's
/// end stands level with the other's: there it pinches at `(R, R₂)`, all
/// four strands meeting at a level top or bottom, and `W` that one point
/// at level slabs. The arm reads past the pinch where it clears the
/// triangle by the pinch's neck, and refuses where it does not. A
/// crossing of the meridian circles is transverse (the rows below decide
/// it), so the section crosses `Π` square to it there and `W` passes from
/// inside the triangle to outside. A loop of `W` meeting `Π` in `2k` points is
/// therefore cut into arcs alternately inside and outside, and each arc
/// inside closes with its mirror into one component: `k` components,
/// each holding exactly one of every other point in the loop's cyclic
/// order, its witness. A loop of `W` meeting no point of `Π` lies wholly
/// inside the triangle or wholly outside, read at one height: inside,
/// it is two components, mirror images, witnessed there. Essentiality
/// is not read: every component carries a witness.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(super) fn parallel_torus_arm<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    [[big_r, r], [big_r2, r2]]: [[T; 2]; 2],
    h: T,
    (e, slack): (T, T),
    toward: Vec3<T>,
    band: Band,
) -> Result<Vec<SectionComponent<T>>, Pinch> {
    let side = a.cross(toward);
    let at = |x: T, y: T, z: T| c + toward * x + a * y + side * z;
    // A level row is one tube's top or bottom against the other's. Left
    // pinched with datum `d`, its two ends stand within
    // `δ = |d| + slack + escalate` of level at every consumed pose (the
    // escalation covering the datum's rounding; a poisoned datum poisons
    // `δ`, and the clearance below then refuses). Near the level ends a
    // tube's sheets stand `√(2·rᵢ·t)` from its radius at depth `t` below
    // its own end, so `W` stays near the node `(R, R₂)`. How near depends
    // on the row's shape: the `neck` each branch passes `clears`, which
    // decides the node's distance outside the closed triangle against it.
    // Where it does not clear, a level end lies in or within the neck of
    // the triangle: there both tori stand with their normals along the
    // axes, a tangency, and the row's pinch stands.
    let node_off = (e - (big_r + big_r2)).max((big_r - big_r2).abs() - e);
    let clears = |d: T, shape: T| {
        let delta = d.abs() + slack + T::from_f64(band.escalate());
        let neck = (T::from_f64(2.0) * shape * delta).sqrt();
        matches!(
            signs_across(
                [(
                    "section_torus_parallel_tori_node",
                    "section_torus_parallel_tori_node_floor",
                    node_off - neck,
                )],
                slack,
                band,
            ),
            Ok([true])
        )
    };
    // Slabs level: `W` spans only the `δ` of heights where one tube's top
    // meets the other's bottom, its sheets at depths `t` and `δ − t` from
    // their ends, so each edge of the triangle reads it within
    // `√(2r·t) + √(2r₂·(δ − t)) ≤ √(2(r + r₂)·δ)` of the node. Clear by
    // that, all of `W` lies outside: no section.
    let slabs = r + r2 - h.abs();
    match signs_across(
        [(
            "section_torus_parallel_tori_slabs",
            "section_torus_parallel_tori_slabs_floor",
            slabs,
        )],
        slack,
        band,
    ) {
        Ok([true]) => {}
        Ok([false]) => return Ok(Vec::new()),
        Err(_) if clears(slabs, r + r2) => return Ok(Vec::new()),
        Err(pinch) => return Err(pinch),
    }
    // Which tube owns the top and the bottom of the heights both span. A
    // level end: at the span's true end one tube's sheets have met, at its
    // radius, and the other's stand within `√(2·rᵢ·δ)` of its own. Clear
    // by `√(2·max(r, r₂)·δ)`, every strand ends there outside the
    // triangle, each arc inside lies in one strand's interior, and any
    // joining of the strands at that end cuts the same arcs: the arm
    // takes the first tube's.
    let owns = |name: [&'static str; 2], d: T, level: bool| match signs_across(
        [(name[0], name[1], d)],
        slack,
        band,
    ) {
        Ok([first]) => Ok(first == level),
        Err(_) if clears(d, r.max(r2)) => Ok(true),
        Err(pinch) => Err(pinch),
    };
    let own_top = owns(
        [
            "section_torus_parallel_tori_top",
            "section_torus_parallel_tori_top_floor",
        ],
        h + r2 - r,
        true,
    );
    let own_bottom = owns(
        [
            "section_torus_parallel_tori_bottom",
            "section_torus_parallel_tori_bottom_floor",
        ],
        h - r2 + r,
        false,
    );
    let (own_top, own_bottom) = (own_top?, own_bottom?);
    let y_hi = if own_top { r } else { h + r2 };
    let y_lo = if own_bottom { -r } else { h - r2 };
    let run = y_hi - y_lo;
    // The station at the start of a loop's `n`th strand.
    let runs = |n: usize| run * T::from_f64(n as f64);
    // The loops of `W`, each its strands in order: a strand's sheet on
    // the tube owning the end it runs into turns there.
    let loops: Vec<Vec<Strand>> = match (own_top, own_bottom) {
        (true, true) => [true, false]
            .map(|s2| vec![strand(true, s2, true), strand(false, s2, false)])
            .to_vec(),
        (false, false) => [true, false]
            .map(|s1| vec![strand(s1, true, true), strand(s1, false, false)])
            .to_vec(),
        (true, false) => vec![vec![
            strand(true, true, true),
            strand(false, true, false),
            strand(false, false, true),
            strand(true, false, false),
        ]],
        (false, true) => vec![vec![
            strand(true, true, true),
            strand(true, false, false),
            strand(false, false, true),
            strand(false, true, false),
        ]],
    };
    // The points of `Π` on both tori, each on its loop at its station
    // along it.
    let mut on_pi: Vec<Vec<(T, Point3<T>)>> = vec![Vec::new(); loops.len()];
    for (xa, far1) in [(big_r, false), (-big_r, true)] {
        for (xb, far2) in [(e + big_r2, false), (e - big_r2, true)] {
            let (ca, cb) = (Point2::new(xa, T::zero()), Point2::new(xb, h));
            let tubes = Circles::new((cb - ca).norm(), r, r2);
            if signs_across(
                [
                    (
                        "section_torus_parallel_tori_reach",
                        "section_torus_parallel_tori_reach_floor",
                        tubes.reach(),
                    ),
                    (
                        "section_torus_parallel_tori_nest",
                        "section_torus_parallel_tori_nest_floor",
                        tubes.internal(),
                    ),
                ],
                slack,
                band,
            )? != [true, true]
            {
                continue;
            }
            for Point2 { x, y, .. } in tubes.at(ca, cb).crossings() {
                // The sheet each tube's circle holds the point on, by
                // the side of the circle's centre it stands, read bare
                // of the slack: the side changes only at the tube's top
                // or bottom. A tube that owns no end of the span stands
                // clear of both by the top and bottom rows' floors,
                // decided across the slack, or at a level end by the
                // node's clearance of the triangle, on whose edges `Π`'s
                // points lie; so its side is decided or the pair
                // refuses. A tube that owns an end can stand
                // undecided only at that end (its other end lies past
                // the span by the same floors), where its two strands
                // meet and read one station: either sheet places the
                // point there.
                let outer =
                    |off: T, owns: bool| match sign(PARALLEL_TORI_ORDER, Margin::of(off), band) {
                        Some(Sign::Positive) => Some(true),
                        Some(Sign::Negative) => Some(false),
                        _ => owns.then_some(true),
                    };
                let (Some(s1), Some(s2)) = (
                    outer(if far1 { xa - x } else { x - xa }, own_top || own_bottom),
                    outer(if far2 { xb - x } else { x - xb }, !own_top || !own_bottom),
                ) else {
                    return Err(Pinch::named(PARALLEL_TORI_ORDER));
                };
                let Some((i, j, up)) = loops.iter().enumerate().find_map(|(i, l)| {
                    l.iter()
                        .position(|t| (t.outer, t.outer2) == (s1, s2))
                        .map(|j| (i, j, l[j].up))
                }) else {
                    unreachable!("every pair of sheets is a strand of one loop of `W`")
                };
                let station = runs(j) + if up { y - y_lo } else { y_hi - y };
                on_pi[i].push((station, at(x, y, T::zero())));
            }
        }
    }
    let mut parts = Vec::new();
    let carrier = ClassCarrier::General;
    let component = |w: Point3<T>| arm_loop([false, false], Some(w), &carrier);
    for (strands, mut points) in loops.iter().zip(on_pi) {
        if points.is_empty() {
            let Strand {
                outer: s1,
                outer2: s2,
                ..
            } = strands[0];
            let y = (y_lo + y_hi) / T::from_f64(2.0);
            let (rho, rho2) = (sheet([big_r, r], y, s1), sheet([big_r2, r2], y - h, s2));
            // Read at a fixed height, the rows move with the partner by
            // one slack through `e`, and through `ρ₂` by at most the
            // slack times `√(1 + slope²) ≤ 1 + slope`, `slope` being
            // the sheet's `|dρ₂/dy|` there: a point of the partner moved
            // by `δ` moves its sheet's radius at that height by at most
            // that much.
            let slope = (y - h).abs() / ((r2 - (y - h).abs()) * (r2 + (y - h).abs())).sqrt();
            let sheets = Circles::new(e, rho, rho2);
            if signs_across(
                [
                    (
                        "section_torus_parallel_tori_gap",
                        "section_torus_parallel_tori_gap_floor",
                        sheets.internal(),
                    ),
                    (
                        "section_torus_parallel_tori_span",
                        "section_torus_parallel_tori_span_floor",
                        sheets.reach(),
                    ),
                ],
                slack * (T::from_f64(2.0) + slope),
                band,
            )? == [true, true]
            {
                let (x, z) = (sheets.radical(), sheets.half_chord());
                parts.extend([z, -z].map(|z| component(at(x, y, z))));
            }
            continue;
        }
        let cycle = runs(strands.len());
        along_w(&mut points, cycle, band)?;
        if points.len() % 2 == 1 {
            return Err(Pinch::named(PARALLEL_TORI_ORDER));
        }
        parts.extend(points.iter().step_by(2).map(|&(_, w)| component(w)));
    }
    Ok(parts)
}

/// Sorts points of a loop of `W` by their station along it, every
/// neighbour pair decided apart, the last and the first round the
/// `cycle` too: two in one place within the band pinch.
///
/// The stations are read off the turned image, bare of the pose's slack,
/// and that is sound: the points are distinct crossings of the meridian
/// circles (two `A` circles never meet, nor two `B` circles, a ring
/// torus's tube circles standing apart), so two stations trade places
/// along the loop only by coinciding, a tangency of one circle pair,
/// which its `reach` and `nest` rows decide across the slack; and the
/// loop's strands are fixed by the top and bottom rows, decided across
/// it too.
fn along_w<T: Decide>(xs: &mut [(T, Point3<T>)], cycle: T, band: Band) -> Result<(), Pinch> {
    apart_in_order(xs, PARALLEL_TORI_ORDER, Margin::of, band)?;
    if let [first, .., last] = xs
        && sign(
            PARALLEL_TORI_ORDER,
            Margin::of(first.0 + cycle - last.0),
            band,
        ) != Some(Sign::Positive)
    {
        return Err(Pinch::named(PARALLEL_TORI_ORDER));
    }
    Ok(())
}

/// Sorts `xs` by their keys, every pair the sort compares decided apart
/// on `margin` of their difference: two in one place within the band,
/// or undecided, pinch naming `name`.
fn apart_in_order<T: Decide, X>(
    xs: &mut [(T, X)],
    name: &'static str,
    margin: impl Fn(T) -> Margin<T>,
    band: Band,
) -> Result<(), Pinch> {
    crate::order::insertion_sort_by(xs, |a, b| match sign(name, margin(a.0 - b.0), band) {
        Some(Sign::Positive) => Ok(true),
        Some(Sign::Negative) => Ok(false),
        _ => Err(Pinch::named(name)),
    })
}

// ---------------------------------------------------------------------
// Torus × square-axis cylinder
// ---------------------------------------------------------------------

/// **A cylinder whose axis stands square to the torus axis**, read in the
/// meridian plane `Π` through the torus centre normal to it: `d` the
/// wall's unit axis, `across = a × d`, and `(e, z0)` the axis's trace in
/// `Π`, `e` along `across` and `z0` along `a`. `slop` bounds how far the
/// rounding leaves `e` and `z0` from their exact values.
#[derive(Clone, Copy, Debug)]
pub struct SquareWall<T: Real> {
    d: Vec3<T>,
    across: Vec3<T>,
    e: T,
    z0: T,
    slop: T,
}

/// The ulps of its operands' size a coordinate of the trace carries.
const SQUARE_TRACE_ULPS: f64 = 2.0;

/// Decides whether the partner axis `(o, d)` stands square to the torus
/// axis `a`: `a·d` levered as [`super::torus::axis_pose`] levers a tilt,
/// over `ball`. Square, the wall is read with its axis turned square to
/// `a` about the pivot, which moves it by less than the band anywhere the
/// section can matter. An undecided margin is not a pose this arm claims.
pub(super) fn square_pose<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    (o, d): (Point3<T>, Vec3<T>),
    ball: crate::ExtentBall<T>,
    band: Band,
) -> Option<SquareWall<T>> {
    if sign(
        "section_torus_square_wall_tilt",
        Margin::levered(a.dot(d), super::torus::lever(ball.foot_on(o, d), ball)),
        band,
    ) != Some(Sign::Zero)
    {
        return None;
    }
    // The pivot's offset from the torus centre, summed from the stored
    // operands so that neither's distance from the origin is rounded in.
    let reach_along = (ball.center() - o).dot(d);
    let w = (o - c) + d * reach_along;
    let (along, n) = super::square_part(d, a);
    let d = along / n;
    let across = a.cross(d);
    Some(SquareWall {
        d,
        across,
        e: w.dot(across),
        z0: w.dot(a),
        slop: ((o - c).norm() + reach_along.abs() + w.norm())
            * T::from_f64(SQUARE_TRACE_ULPS * f64::EPSILON),
    })
}

/// A point where the wall's trace `K` crosses a tube circle, `T₊` at
/// `y = R` (`plus`) or `T₋`, on its outer half (`|y| > R`) or its inner.
#[derive(Clone, Copy, Debug)]
struct Fold<T: Real> {
    plus: bool,
    y: T,
    z: T,
    outer: bool,
}

/// A point where `K` crosses an edge of the strip `|z| ≤ r` (`top`:
/// `z = r`), whether it stands over the torus's parallel there
/// (`|y| < R`), and its side of `K`'s centre (`right`: along `across`).
#[derive(Clone, Copy, Debug)]
struct Edge {
    top: bool,
    over: bool,
    right: bool,
}

const SQUARE_WALK: &str = "section_torus_square_wall_walk";
const SQUARE_ORDER: &str = "section_torus_square_wall_order";
const SQUARE_FOLD_HALF: &str = "section_torus_square_wall_fold_half";
const SQUARE_EDGE_OVER: &str = "section_torus_square_wall_edge_over";

/// **Torus × a cylinder whose axis is square to the torus's**, the wall
/// `wall` of radius `rc`.
///
/// `Π` holds the torus axis and stands normal to the wall's, so the wall
/// is the union of the lines `P + t·d` normal to `Π` through the points
/// `P = (Y, Z)` of its trace `K`, the circle of radius `rc` about
/// `(e, z0)`. Such a line stays at height `Z` and at distance
/// `√(t² + Y²)` from the torus axis, so it meets the torus where
/// `√(t² + Y²) = R ± √(r² − Z²)`: on each SHEET `ρ = R ± √(r² − Z²)` over
/// the strip `|Z| ≤ r`, at `t = ±√(ρ² − Y²)` where `ρ ≥ |Y|`. The sheets
/// join on the strip's edges, at `ρ = R`. So the section is the double
/// cover, `t ↦ −t`, of the curve `γ` of points `(P, sheet)` with
/// `ρ ≥ |Y|`, folded where `ρ = |Y|`: where `P` lies on a tube circle
/// `T±` (centred `(±R, 0)` in `Π`), on the outer sheet on the circle's
/// outer half and on the inner sheet on its inner half.
///
/// `γ`'s components are `K`'s arcs in the strip, each an oval running
/// out along the outer sheet and back along the inner, or with `K` inside
/// the open strip the two sheets as two loops. On each, `ρ ≥ |Y|` holds on
/// alternate runs between folds; a run between two folds lifts to ONE
/// closed component through `t = 0` at those two folds, and a fold-free
/// component wholly in `ρ > |Y|` lifts to TWO, one at `t > 0` and its
/// mirror. Their classes:
///
/// - a component through two folds is its own mirror, so the mirror acts
///   on it as a reflection with those two folds fixed. It is null on the
///   wall (an essential curve the mirror fixes would lie in `t = 0`), and
///   its tube angle retraces itself (`q = 0`). It winds once about the
///   torus axis iff its folds lie on different tube circles: it crosses
///   `Π` only there, on the half-planes `u = ±π/2`;
/// - a fold-free lift lies in one half-space `±t > 0`, where the torus is
///   a tube segment (`p = 0`). A loop of a sheet is essential on the
///   wall and stays on one half of the tube (null on the torus); an oval
///   is null on the wall, and winds once about the tube iff it runs from
///   one strip edge to the other.
///
/// Every fold and strip crossing is a circle meeting a circle or a line
/// in `Π`, in closed form; each component's witness is one of its folds,
/// or for a fold-free lift a point where its sheets stand apart. A margin
/// `Zero` or undecided where the carriers are tangent — `K` tangent to a
/// tube circle, or to a strip edge over the parallel, a fold too shallow
/// to place within the band, two crossings in one place along `K` —
/// pinches; a fold or strip crossing at a tube circle's top or bottom,
/// where they cross, is unordered.
pub(super) fn square_cylinder_arm<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    [big_r, r]: [T; 2],
    wall: &SquareWall<T>,
    rc: T,
    band: Band,
) -> Result<Vec<SectionComponent<T>>, ArmRefusal> {
    let &SquareWall {
        d,
        across,
        e,
        z0,
        slop,
    } = wall;
    let at = |y: T, z: T, t: T| c + across * y + a * z + d * t;
    let k = (e, z0, rc);
    let top = reaches_edge(
        "section_torus_square_wall_top",
        z0 + rc - r,
        (e, big_r),
        band,
    )?;
    let bottom = reaches_edge(
        "section_torus_square_wall_bottom",
        rc - z0 - r,
        (e, big_r),
        band,
    )?;
    let folds = square_folds([big_r, r], k, slop, band)?;
    if top || bottom {
        across_strip([big_r, r], k, [top, bottom], &folds, at, band)
    } else {
        inside_strip([big_r, r], k, &folds, at, band)
    }
}

/// `K`'s crossings with the tube circles `T±`: none or two each.
///
/// A fold is where `K` crosses a tube circle at the angle `θ` between
/// them, `sin θ = dist·half / (r·rc)` for centres `dist` apart and the
/// crossings `half` off their line. Where the angle is shallow the
/// rounding `slop` of `K`'s centre moves the crossing `slop / sin θ` along
/// both circles while it stays on each: a fold whose drift the band does
/// not decide `Zero` is a near-tangency at the fold (the carriers' normals
/// there stand `θ` apart) and pinches. A fold within the band of a tube
/// circle's top or bottom point lies on neither half; the carriers cross
/// there, and it is unordered.
fn square_folds<T: Decide>(
    [big_r, r]: [T; 2],
    (e, z0, rc): (T, T, T),
    slop: T,
    band: Band,
) -> Result<Vec<Fold<T>>, ArmRefusal> {
    let mut out = Vec::new();
    for plus in [true, false] {
        let sigma = if plus { T::one() } else { -T::one() };
        // Circle against circle: `T_σ` about `(σR, 0)` radius `r`, `K`
        // about `(e, z0)` radius `rc`.
        let (c1, c2) = (Point2::new(sigma * big_r, T::zero()), Point2::new(e, z0));
        let dist = (c2 - c1).norm();
        let tubes = Circles::new(dist, r, rc);
        let [cut] = signs([("section_torus_square_wall_tube", tubes.secant())], band)?;
        if !cut {
            continue;
        }
        let half = tubes.half_chord();
        let drift = (slop + (e.abs() + big_r + rc) * T::from_f64(SQUARE_TRACE_ULPS * f64::EPSILON))
            * r
            * rc
            / (dist * half);
        if sign(
            "section_torus_square_wall_fold_drift",
            Margin::of(drift),
            band,
        ) != Some(Sign::Zero)
        {
            return Err(Pinch::named("section_torus_square_wall_fold_drift").into());
        }
        for Point2 { x: y, y: z, .. } in tubes.at(c1, c2).crossings() {
            let [outer] = signs([(SQUARE_FOLD_HALF, sigma * y - big_r)], band)
                .map_err(|_| ArmRefusal::Unordered(SQUARE_FOLD_HALF))?;
            out.push(Fold { plus, y, z, outer });
        }
    }
    Ok(out)
}

/// Whether `K` reaches past a strip edge's line, by the margin `m` of its
/// reach there. `K` within the band of tangent to the line touches it at
/// `(e, ±r)`: over the parallel (`|e| ≤ R`) the wall is tangent to the
/// torus there, a pinch; beyond it no line of the wall near that point
/// meets the torus, and the section is the one of `K` held just short of
/// the line, a pose the band admits.
fn reaches_edge<T: Decide>(
    name: &'static str,
    m: T,
    (e, big_r): (T, T),
    band: Band,
) -> Result<bool, Pinch> {
    match signs([(name, m)], band) {
        Ok([past]) => Ok(past),
        Err(_)
            if sign(
                "section_torus_square_wall_edge_clear",
                Margin::of(e.abs() - big_r),
                band,
            ) == Some(Sign::Positive) =>
        {
            Ok(false)
        }
        Err(pinch) => Err(pinch),
    }
}

/// The radius of the sheet `ρ = R ± √(r² − z²)` at height `z`.
fn sheet<T: Real>([big_r, r]: [T; 2], z: T, outer: bool) -> T {
    let h = ((r - z.abs()) * (r + z.abs())).sqrt();
    if outer { big_r + h } else { big_r - h }
}

/// The two lifts of a fold-free component of `γ` through `(y, z)` on the
/// sheet of radius `rho > |y|`, essential as `[torus, wall]` says.
fn lifts<T: Real>(
    (y, z, rho): (T, T, T),
    [torus, partner]: [bool; 2],
    at: impl Fn(T, T, T) -> Point3<T>,
) -> [SectionComponent<T>; 2] {
    let t = ((rho - y) * (rho + y)).sqrt();
    [t, -t].map(|t| arm_loop([torus, partner], Some(at(y, z, t)), &ClassCarrier::General))
}

fn through<T: Real>(
    f: Fold<T>,
    torus: bool,
    at: impl Fn(T, T, T) -> Point3<T>,
) -> SectionComponent<T> {
    arm_loop(
        [torus, false],
        Some(at(f.y, f.z, T::zero())),
        &ClassCarrier::General,
    )
}

/// **`K` inside the open strip**: the sheets are two loops over all of
/// `K`, the outer meeting the folds on outer halves, the inner those on
/// inner halves. A tube circle's two folds share a half: `K` meets the
/// segment `y = ±R` of the strip only inside that circle's disc. On the
/// outer loop each tube circle's folds bound `K`'s arc inside its disc,
/// in `ρ ≥ |Y|`: one component each. On the inner loop, one circle's
/// folds bound the rest of `K`, which then stands between the discs;
/// both circles' folds bound the two arcs of `K` between the discs, each
/// from `T₊` to `T₋` and winding about the torus axis. (Folds on both
/// outer halves would need `K` to reach past `y = ±R` inside `|z| < r`,
/// `rc > R > r`.) On a fold-free loop `ρ − |Y|` keeps one sign, its zeros
/// being folds, so any point of `K` decides it: the arm reads the point
/// it witnesses with, `(e − rc, z0)`.
fn inside_strip<T: Decide>(
    ring: [T; 2],
    (e, z0, rc): (T, T, T),
    folds: &[Fold<T>],
    at: impl Fn(T, T, T) -> Point3<T> + Copy,
    band: Band,
) -> Result<Vec<SectionComponent<T>>, ArmRefusal> {
    let mut parts = Vec::new();
    for outer in [true, false] {
        let on = |plus: bool| -> Vec<Fold<T>> {
            folds
                .iter()
                .copied()
                .filter(|f| f.outer == outer && f.plus == plus)
                .collect()
        };
        match (on(true).as_slice(), on(false).as_slice()) {
            ([], []) => {
                let (y, rho) = (e - rc, sheet(ring, z0, outer));
                if signs([("section_torus_square_wall_sheet", rho - y.abs())], band)? == [true] {
                    parts.extend(lifts((y, z0, rho), [false, true], at));
                }
            }
            ([f, _], []) | ([], [f, _]) => parts.push(through(*f, false, at)),
            ([p1, p2], [_, _]) if !outer => {
                parts.push(through(*p1, true, at));
                parts.push(through(*p2, true, at));
            }
            _ => return Err(Pinch::named(SQUARE_WALK).into()),
        }
    }
    Ok(parts)
}

/// **`K` across a strip edge**: `γ` is one oval per arc of `K` in the
/// strip. Read along `K` from a point outside the strip (its top when it
/// crosses `z = r`, else its bottom), the strip crossings alternate in
/// and out, and with two arcs (`K` across both edges) they are `K`'s
/// left and right flanks. Every fold lies inside its arc, in the open
/// strip, so folds are ordered among themselves only, never against a
/// strip crossing. An arc's oval runs out along the outer sheet
/// through its outer-half folds and back along the inner through its
/// inner-half ones; at its strip crossings both sheets lie in `ρ ≥ |Y|`
/// iff the crossing stands over the parallel (`|y| < R`).
fn across_strip<T: Decide>(
    ring: [T; 2],
    (e, z0, rc): (T, T, T),
    [top, bottom]: [bool; 2],
    folds: &[Fold<T>],
    at: impl Fn(T, T, T) -> Point3<T> + Copy,
    band: Band,
) -> Result<Vec<SectionComponent<T>>, ArmRefusal> {
    let [big_r, r] = ring;
    if top != bottom {
        let m = if top { r - (z0 - rc) } else { z0 + rc + r };
        if !reaches_edge("section_torus_square_wall_meets", m, (e, big_r), band)? {
            return Ok(Vec::new());
        }
    }
    let kappa = if top { T::one() } else { -T::one() };
    // The angle along `K` from the cut, `(−π, π)` everywhere off it.
    let psi = |y: T, z: T| (kappa * (y - e)).atan2(-kappa * (z - z0));
    let mut edges = Vec::new();
    for (edge, crossed) in [(true, top), (false, bottom)] {
        if !crossed {
            continue;
        }
        let z = if edge { r } else { -r };
        let h = (z - z0).abs();
        let w = ((rc - h) * (rc + h)).sqrt();
        for right in [true, false] {
            let y = if right { e + w } else { e - w };
            let [over] = signs([(SQUARE_EDGE_OVER, big_r - y.abs())], band)
                .map_err(|_| ArmRefusal::Unordered(SQUARE_EDGE_OVER))?;
            edges.push((
                psi(y, z),
                Edge {
                    top: edge,
                    over,
                    right,
                },
            ));
        }
    }
    along_k(&mut edges, rc, band)?;
    let arcs: Vec<[Edge; 2]> = edges.chunks(2).map(|p| [p[0].1, p[1].1]).collect();
    let mut on_arc: Vec<Vec<(T, Fold<T>)>> = vec![Vec::new(); arcs.len()];
    for &f in folds {
        let k = if arcs.len() == 1 {
            0
        } else {
            let [right] = signs([("section_torus_square_wall_fold_side", f.y - e)], band)?;
            usize::from(arcs[0][0].right != right)
        };
        on_arc[k].push((psi(f.y, f.z), f));
    }
    let mut parts = Vec::new();
    for ([start, end], mut on) in arcs.into_iter().zip(on_arc) {
        along_k(&mut on, rc, band)?;
        // Out along the outer sheet, back along the inner.
        let (outward, inner): (Vec<Fold<T>>, Vec<Fold<T>>) =
            on.into_iter().map(|(_, f)| f).partition(|f| f.outer);
        let crossed_out = outward.len();
        let ring_order: Vec<Fold<T>> = outward.into_iter().chain(inner.into_iter().rev()).collect();
        if (crossed_out % 2 == 0) != (start.over == end.over) || ring_order.len() % 2 == 1 {
            return Err(Pinch::named(SQUARE_WALK).into());
        }
        if ring_order.is_empty() {
            if start.over {
                // Edge to edge: `K`'s point at `z = 0`, where the sheets
                // stand `2r` apart; else the arc's extreme.
                let essential = start.top != end.top;
                let (y, z) = if essential {
                    let h = z0.abs();
                    let w = ((rc - h) * (rc + h)).sqrt();
                    (if start.right { e + w } else { e - w }, T::zero())
                } else {
                    (e, z0 - kappa * rc)
                };
                parts.extend(lifts((y, z, sheet(ring, z, true)), [essential, false], at));
            }
            continue;
        }
        // The runs in `ρ ≥ |Y|` pair the folds, the first run wrapping
        // through `start` when it stands over the parallel.
        let n = ring_order.len();
        let skip = usize::from(start.over);
        for k in (0..n).step_by(2) {
            let (f1, f2) = (ring_order[(k + skip) % n], ring_order[(k + skip + 1) % n]);
            parts.push(through(f1, f1.plus != f2.plus, at));
        }
    }
    Ok(parts)
}

/// Sorts points of `K` by their angle along it, every final neighbour
/// pair decided apart (as arc length): two in one place within the band
/// pinch.
fn along_k<T: Decide, X: Copy>(xs: &mut [(T, X)], rc: T, band: Band) -> Result<(), Pinch> {
    apart_in_order(xs, SQUARE_ORDER, |m| Margin::levered(m, rc), band)
}
