//! **Plane × one boundary curve** ([`boundary_section`]): where a NURBS
//! curve meets a plane, decided once, as a one-dimensional problem.
//!
//! The plane distance along the curve is `φ(t) = n·(C(t) − p₀)`. On a
//! rational curve `φ = h / W` with `h = Σ Nᵢ·wᵢ·dᵢ`, `W = Σ Nᵢ·wᵢ` and
//! `dᵢ = n·(Pᵢ − p₀)`, and the weights are positive, so `φ` and the
//! polynomial `h` share their zeros and their sign. The section reads
//! both from Bernstein coefficients in interval arithmetic: a cell of
//! the curve's parameter is clear when `h`'s coefficients over it are
//! one-signed, and the cells left at the floor hold every root.
//!
//! Two readings come out:
//!
//! - **`On`**: the curve lies within the band of the plane. The margin
//!   is the curve's certified sup distance from the plane, `max |dᵢ|`
//!   (the rational hull: `φ` is a convex combination of the `dᵢ`), and
//!   it is `On` where that margin is not decided positive.
//! - **Roots**: otherwise, each root isolated to the floor, the curve's
//!   accounting floor in metres minted over its parameter domain at its
//!   speed. A root away from the curve's ends is decided transversal
//!   along the curve: the slope `|dφ/ds|`, a sine levered by the
//!   caller's arm, must clear the band, or the plane grazes the curve
//!   there ([`SsiError::BoundaryGraze`]). A root within the floor of an
//!   end is reported as such and left to the caller, which knows what
//!   that end is (a corner of a wall, a vertex of a face).
//!
//! The SSI's boundary pass calls this on the four sides of a wall's
//! knot rectangle (`super::boundary`); the boolean's NURBS crossing
//! layer is its second caller once it is wired.

use geom::NurbsCurve3;
use geom_core::Bounds;
use geom_core::interval::certification::Certification;
use geom_core::interval::max_bound;
use geom_core::k_stats::decide;
use geom_core::spline::algebra::refine_plan_homogeneous;
use geom_core::{Band, Decide, Indeterminate, Interval, Margin, Point3, Sign, SupSpeed, Vec3};

use super::SsiError;
use super::exhaust::{ParamSpan, SweepFloor, isolate_section};
use crate::dihedral::decide_reported;
use crate::recourse::{Refused, RefusedArm};

/// What a plane is to one boundary curve ([`boundary_section`]).
#[derive(Clone, Debug)]
pub enum BoundarySection {
    /// The curve lies within the band of the plane along its whole
    /// length: its certified sup distance from the plane, `sup`, in
    /// metres, is not decided positive.
    On {
        /// The certified sup of `|φ|` over the curve, in metres.
        sup: f64,
        /// The sign of `φ` along the whole curve, where its hull, refined
        /// where it straddles zero, is certified one-signed: `Some(true)`
        /// for the plane's positive side.
        side_of_plane: Option<bool>,
    },
    /// The curve meets the plane at isolated roots, or not at all.
    Roots {
        /// The roots away from the curve's ends, each decided
        /// transversal, in ascending parameter order.
        interior: Vec<SectionRoot>,
        /// A root within the floor of the curve's first end, if any.
        at_start: Option<SectionRoot>,
        /// A root within the floor of the curve's last end, if any.
        at_end: Option<SectionRoot>,
    },
}

/// One isolated root of the plane distance along a boundary curve.
#[derive(Clone, Copy, Debug)]
pub struct SectionRoot {
    /// The parameter interval the root lies in, no wider than the floor
    /// (a run of floor cells, for a root on a cell's end).
    pub bracket: (f64, f64),
    /// A certified lower bound on `|dφ/ds|` over the bracket where `φ`
    /// vanishes: the sine of the angle between the curve and the plane.
    /// `0` where the bound straddles.
    pub slope: f64,
    /// The sign of `dφ/dt` over the bracket, where it is one-signed.
    pub rising: Option<bool>,
}

/// How a band-decided refusal was read: definitely in the zero band, or
/// in the band's escalation zone.
#[derive(Clone, Debug)]
pub enum BandVerdict {
    /// The margin decided zero, where zero does not pass.
    Refused(Refused),
    /// The margin landed in the band, or was poisoned.
    Undecided(Indeterminate),
}

impl BandVerdict {
    /// The refused arm this verdict is, for the decision's ending.
    #[must_use]
    pub fn arm(&self) -> RefusedArm<'_> {
        match self {
            Self::Refused(r) => r.arm(),
            Self::Undecided(cause) => RefusedArm::Undecided(cause),
        }
    }

    /// The verdict's margin as text.
    #[must_use]
    pub fn margin_text(&self) -> String {
        match self {
            Self::Refused(r) => format!("{:e}", r.margin()),
            Self::Undecided(cause) => cause.payload().to_string(),
        }
    }
}

/// The verdict of the decision `predicate` on `margin` where it does
/// not pass: refused in the band or definitely, or undecided. `None`
/// where it passes positive.
pub(crate) fn band_verdict<T: Decide>(
    predicate: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Option<BandVerdict> {
    match decide_reported(predicate, margin, band) {
        Ok(decided) => Refused::of(decided, band).map(BandVerdict::Refused),
        Err(cause) => Some(BandVerdict::Undecided(cause)),
    }
}

/// One Bézier piece: its parameter interval, and the Bernstein
/// coefficients of `h` and of `W` over it.
type Piece = ((f64, f64), Vec<Interval>, Vec<Interval>);

/// The Bernstein coefficients of `h` and `W` over each Bézier piece of
/// a curve, with each piece's parameter interval.
struct Pieces {
    /// The pieces, ascending.
    pieces: Vec<Piece>,
}

impl Pieces {
    /// The curve's pieces: every interior knot raised to multiplicity
    /// `p` (exact in ℝ, applied in interval arithmetic), so the control
    /// net is the pieces' Bernstein coefficients end to end.
    fn of(curve: &NurbsCurve3<f64>, origin: Point3<f64>, normal: Vec3<f64>) -> Option<Self> {
        let n = [normal.x, normal.y, normal.z].map(Interval::point);
        let o = [origin.x, origin.y, origin.z].map(Interval::point);
        let control: Vec<[Interval; 3]> = curve
            .control()
            .iter()
            .map(|c| [c.x, c.y, c.z].map(Interval::point))
            .collect();
        Self::enclosed(curve.knots(), &control, curve.weights(), (o, n))
    }

    /// [`Pieces::of`] from a curve's parts, its control points enclosed:
    /// the one arithmetic both the boundary pass and limb 3's side arm
    /// read ([`SectionReader`]).
    fn enclosed(
        kv: &geom_core::spline::KnotVector,
        control: &[[Interval; 3]],
        weights: &[f64],
        (o, n): ([Interval; 3], [Interval; 3]),
    ) -> Option<Self> {
        let p = kv.degree();
        // The weights scaled by a power of two near their largest, which
        // is exact and moves neither the curve, `φ = h/W` nor `h`'s sign,
        // so a net of tiny weights does not underflow `h`.
        let top = weights.iter().copied().fold(0.0, f64::max);
        #[allow(clippy::cast_possible_truncation)]
        let k = if top > 0.0 && top.is_finite() {
            -(top.log2().floor() as i32)
        } else {
            0
        };
        // Two halves, so a subnormal's exponent does not overflow the
        // power it is scaled by.
        let (k1, k2) = (k / 2, k - k / 2);
        let weights: Vec<f64> = weights
            .iter()
            .map(|w| w * 2.0f64.powi(k1) * 2.0f64.powi(k2))
            .collect();
        let h: Vec<Interval> = control
            .iter()
            .zip(&weights)
            .map(|(c, &w)| {
                let d = c
                    .iter()
                    .zip(n.iter().zip(o.iter()))
                    .fold(Interval::point(0.0), |acc, (x, (nk, ok))| {
                        acc + *nk * (*x - *ok)
                    });
                Interval::point(w) * d
            })
            .collect();
        // `W` from the same scaled weights as `h`: `φ = h/W` only when
        // both carry the one factor.
        let w: Vec<Interval> = weights.iter().map(|&w| Interval::point(w)).collect();
        let add: Vec<f64> = kv
            .interior_knots()
            .flat_map(|(k, mult)| core::iter::repeat_n(k, p.saturating_sub(mult)))
            .collect();
        let plans = refine_plan_homogeneous(kv, &add).ok()?;
        let (mut h, mut w, mut knots) = (h, w, kv.clone());
        for plan in &plans {
            h = plan.apply_certified(&h);
            w = plan.apply_certified(&w);
            knots = plan.knots().clone();
        }
        let breaks: Vec<f64> = knots.knot_runs().map(|(k, _)| k).collect();
        let mut pieces = Vec::new();
        for (i, pair) in breaks.windows(2).enumerate() {
            let lo = i * p;
            let (Some(hc), Some(wc)) = (h.get(lo..=lo + p), w.get(lo..=lo + p)) else {
                return None;
            };
            pieces.push(((pair[0], pair[1]), hc.to_vec(), wc.to_vec()));
        }
        Some(Self { pieces })
    }

    /// The plane distance `φ = h/W` over the whole curve, enclosed
    /// ([`ratio_hull`]).
    fn distance(&self) -> Interval {
        ratio_hull(self.pieces.iter().map(|(_, hc, wc)| (hc, wc)))
    }

    /// The sign of `φ` along the whole curve, read on its refined hull
    /// ([`refined_sign`]).
    fn sign(&self) -> Result<Option<bool>, SsiError> {
        refined_sign(
            self.pieces
                .iter()
                .map(|(_, hc, wc)| (hc.clone(), wc.clone())),
        )
    }

    /// For each piece overlapping `[a, b]`: the Bernstein coefficients
    /// of `h` and `W` over the overlap, and of `dh/dt` there.
    fn over(&self, a: f64, b: f64) -> Vec<(Vec<Interval>, Vec<Interval>, Vec<Interval>)> {
        let mut out = Vec::new();
        for ((k0, k1), hc, wc) in &self.pieces {
            let (lo, hi) = (a.max(*k0), b.min(*k1));
            if lo > hi || (lo == hi && a != b) {
                continue;
            }
            let span = Interval::point(*k1) - Interval::point(*k0);
            let ra = (Interval::point(lo) - Interval::point(*k0)) / span;
            let rb = (Interval::point(hi) - Interval::point(*k0)) / span;
            let p = hc.len() - 1;
            #[allow(clippy::cast_precision_loss)]
            let pf = Interval::point(p as f64);
            let dh: Vec<Interval> = hc.windows(2).map(|c| pf * (c[1] - c[0]) / span).collect();
            out.push((
                sub_piece(hc, ra, rb),
                sub_piece(wc, ra, rb),
                sub_piece(&dh, ra, rb),
            ));
        }
        out
    }
}

/// The hull of the Bernstein ratios `h_i/W_i` of pieces (every weight
/// positive): an enclosure of `φ = h/W` over them.
fn ratio_hull<'a>(
    pieces: impl Iterator<Item = (&'a Vec<Interval>, &'a Vec<Interval>)>,
) -> Interval {
    pieces
        .flat_map(|(hc, wc)| hc.iter().zip(wc).map(|(h, w)| *h / *w))
        .reduce(Interval::hull)
        .unwrap_or_else(Interval::refused)
}

/// **The sign of `φ` over pieces, read on a refined hull.** Each piece's
/// hull of Bernstein ratios ([`ratio_hull`]) is read; where it straddles
/// zero the piece is halved (de Casteljau in interval arithmetic, `h` and
/// `W` alike) and the halves are read, until every piece's hull is
/// certified one-signed, all of one sign (`Some`). It is `None` (in band,
/// not one-signed) at the first piece whose hull is refused, two pieces of
/// opposite sign, or a straddling piece whose halves' hull is not
/// narrower than its own: the arithmetic's floor at that piece. A loose
/// net's hull straddles zero where `φ` does not, and its halves' hulls
/// converge to `φ`'s values, so a piece clear of zero turns one-signed; a
/// piece holding a zero of `φ` narrows until rounding stops it, and a side
/// flush with the plane, `φ` at rounding along it, stops at its first
/// halving.
///
/// # Errors
///
/// [`SsiError::CellBudget`] where the pieces read pass
/// [`super::SSI_MAX_CELLS`].
fn refined_sign(
    pieces: impl Iterator<Item = (Vec<Interval>, Vec<Interval>)>,
) -> Result<Option<bool>, SsiError> {
    let mut stack: Vec<(Vec<Interval>, Vec<Interval>)> = pieces.collect();
    stack.reverse();
    let mut found = None;
    let mut reads = 0usize;
    while let Some((hc, wc)) = stack.pop() {
        reads += 1;
        #[cfg(test)]
        probe_r2::READS.with(|r| r.set(r.get() + 1));
        if reads > super::SSI_MAX_CELLS {
            return Err(SsiError::CellBudget {
                budget: super::SSI_MAX_CELLS,
            });
        }
        let phi = ratio_hull(core::iter::once((&hc, &wc)));
        if let Some(s) = sign(phi) {
            if found.is_some_and(|f| f != s) {
                return Ok(None);
            }
            found = Some(s);
            continue;
        }
        let ((hl, hr), (wl, wr)) = (halves(&hc), halves(&wc));
        let split = Interval::hull(
            ratio_hull(core::iter::once((&hl, &wl))),
            ratio_hull(core::iter::once((&hr, &wr))),
        );
        let (before, after) = (Certification::width(phi), Certification::width(split));
        if before.is_nan() || after.is_nan() || after >= before {
            return Ok(None);
        }
        stack.push((hr, wr));
        stack.push((hl, wl));
    }
    Ok(found)
}

/// The Bernstein coefficients of the two halves of a Bézier piece, by de
/// Casteljau at `½` in interval arithmetic.
fn halves(c: &[Interval]) -> (Vec<Interval>, Vec<Interval>) {
    let half = Interval::point(0.5);
    let mut level = c.to_vec();
    let (mut left, mut right) = (Vec::with_capacity(c.len()), Vec::with_capacity(c.len()));
    while let (Some(&first), Some(&last)) = (level.first(), level.last()) {
        left.push(first);
        right.push(last);
        level = level.windows(2).map(|w| half * (w[0] + w[1])).collect();
    }
    right.reverse();
    (left, right)
}

/// **A side's plane distance, read as [`boundary_section`] reads it**,
/// over any stretch of the side: the side's Bernstein pieces
/// ([`Pieces::enclosed`], the arithmetic [`boundary_section`] reads a
/// whole side by), cut to the stretch. Limb 3's side arm reads a side
/// through it, so the two doors read one input.
pub(crate) struct SectionReader {
    pieces: Pieces,
}

impl SectionReader {
    /// The reader of a curve given by its parts, its control points
    /// enclosed; `None` where the knot algebra refuses its Bernstein
    /// form.
    pub(crate) fn of(
        knots: &geom_core::spline::KnotVector,
        control: &[[Interval; 3]],
        weights: &[f64],
        (origin, normal): ([Interval; 3], [Interval; 3]),
    ) -> Option<Self> {
        Pieces::enclosed(knots, control, weights, (origin, normal)).map(|pieces| Self { pieces })
    }

    /// The stretch `[a, b]` of the curve: its pieces cut to it.
    pub(crate) fn stretch(&self, (a, b): (f64, f64)) -> Stretch {
        Stretch {
            pieces: self
                .pieces
                .over(a, b)
                .into_iter()
                .map(|(hc, wc, _)| (hc, wc))
                .collect(),
        }
    }

    /// The plane distance `φ` over the stretch `[a, b]` of the curve,
    /// enclosed.
    pub(crate) fn over(&self, t: (f64, f64)) -> Interval {
        self.stretch(t).distance()
    }
}

/// A stretch of a curve, as the Bernstein coefficients of `h` and `W`
/// over its pieces cut to it ([`SectionReader::stretch`]).
pub(crate) struct Stretch {
    pieces: Vec<(Vec<Interval>, Vec<Interval>)>,
}

impl Stretch {
    /// The plane distance `φ` over the stretch, enclosed as
    /// [`Pieces::distance`] encloses the whole curve's.
    pub(crate) fn distance(&self) -> Interval {
        ratio_hull(self.pieces.iter().map(|(hc, wc)| (hc, wc)))
    }

    /// The sign of `φ` over the stretch, read on its refined hull as
    /// [`boundary_section`] reads a whole side's.
    ///
    /// # Errors
    ///
    /// As [`refined_sign`].
    pub(crate) fn sign(&self) -> Result<Option<bool>, SsiError> {
        refined_sign(self.pieces.iter().cloned())
    }
}

/// The Bernstein coefficients of the polynomial with coefficients `c` on
/// `[0, 1]`, restricted to `[ra, rb]`: coefficient `j` is the blossom at
/// `p − j` copies of `ra` and `j` of `rb`. In interval arithmetic, so
/// every coefficient encloses the exact one for every `ra`, `rb` in the
/// enclosures given.
fn sub_piece(c: &[Interval], ra: Interval, rb: Interval) -> Vec<Interval> {
    let p = c.len().saturating_sub(1);
    (0..=p)
        .map(|j| {
            let mut level = c.to_vec();
            for k in 0..p {
                let t = if k < p - j { ra } else { rb };
                let s = Interval::point(1.0) - t;
                level = level.windows(2).map(|w| s * w[0] + t * w[1]).collect();
            }
            level.first().copied().unwrap_or_else(Interval::refused)
        })
        .collect()
}

/// The hull of a coefficient list.
fn hull(c: &[Interval]) -> Interval {
    c.iter()
        .copied()
        .reduce(Interval::hull)
        .unwrap_or_else(Interval::refused)
}

/// Whether an enclosure is certified and one-signed.
fn one_signed(i: Interval) -> bool {
    sign(i).is_some()
}

/// The sign of a certified enclosure clear of zero: `Some(true)` above
/// it. A refused bracket carries ordinary endpoints, so it is asked
/// first.
pub(crate) fn sign(i: Interval) -> Option<bool> {
    (i.is_certified() && (i.lo() > 0.0 || i.hi() < 0.0)).then(|| i.lo() > 0.0)
}

/// The largest magnitude in a certified enclosure; `NaN` for a refused
/// one.
pub(crate) fn magnitude(i: Interval) -> f64 {
    if i.is_certified() {
        max_bound(i.lo().abs(), i.hi().abs())
    } else {
        f64::NAN
    }
}

/// **Plane × one NURBS curve**: whether the curve lies in the plane
/// (`On`), or where it crosses it (module docs).
///
/// `normal` is the plane's unit normal and `origin` a point on it.
/// `speed` is a certified sup of the curve's speed `‖C′‖`, which
/// crosses `floor_meters` into the curve's parameter; `arm` is the lever
/// arm a crossing's slope is levered by, in metres.
///
/// # Errors
///
/// [`SsiError::FloorUnresolvable`] for a floor the curve's domain
/// cannot resolve at that speed; [`SsiError::BoundaryGraze`] (with no
/// side named) where the plane grazes the curve away from its ends;
/// [`SsiError::Escalated`] naming
/// [`super::TraceDecision::BoundarySection`] where the sup distance is
/// poisoned; [`SsiError::UnsupportedCertificate`] for a curve whose
/// Bernstein form the knot algebra refuses (unreachable for a validated
/// curve); [`SsiError::CellBudget`].
pub fn boundary_section(
    curve: &NurbsCurve3<f64>,
    origin: Point3<f64>,
    normal: Vec3<f64>,
    speed: SupSpeed<f64>,
    floor_meters: f64,
    arm: f64,
    band: Band,
) -> Result<BoundarySection, SsiError> {
    let unsupported = SsiError::UnsupportedCertificate {
        what: "a boundary curve's Bernstein form was refused by the knot algebra",
    };
    let pieces = Pieces::of(curve, origin, normal).ok_or(unsupported)?;
    // ---- On: the curve's sup distance from the plane ----
    let d = pieces.distance();
    let sup = magnitude(d);
    let on = || -> Result<BoundarySection, SsiError> {
        Ok(BoundarySection::On {
            sup,
            side_of_plane: pieces.sign()?,
        })
    };
    match decide("ssi_boundary_on_plane", Margin::of(sup), band) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) => return on(),
        Err(cause) if cause.margin.is_invalid() => {
            return Err(super::TraceDecision::BoundarySection.escalated(cause));
        }
        // In band: the curve lies within the band's reach of the plane,
        // and the caller reports it as a region, not a crossing.
        Err(_) => return on(),
    }

    roots(curve, &pieces, speed, floor_meters, arm, band)
}

/// **The roots of the plane distance along a curve**, isolated to the
/// floor and decided as [`boundary_section`] decides them, whatever the
/// curve's sup distance: for a curve shorter than the band, whose
/// distance from the plane is in band along its whole length though the
/// plane crosses it.
///
/// # Errors
///
/// As [`boundary_section`].
pub(crate) fn boundary_roots(
    curve: &NurbsCurve3<f64>,
    origin: Point3<f64>,
    normal: Vec3<f64>,
    speed: SupSpeed<f64>,
    floor_meters: f64,
    arm: f64,
    band: Band,
) -> Result<BoundarySection, SsiError> {
    let pieces = Pieces::of(curve, origin, normal).ok_or(SsiError::UnsupportedCertificate {
        what: "a boundary curve's Bernstein form was refused by the knot algebra",
    })?;
    roots(curve, &pieces, speed, floor_meters, arm, band)
}

/// The roots, to the floor ([`boundary_roots`]).
fn roots(
    curve: &NurbsCurve3<f64>,
    pieces: &Pieces,
    speed: SupSpeed<f64>,
    floor_meters: f64,
    arm: f64,
    band: Band,
) -> Result<BoundarySection, SsiError> {
    let (t0, t1) = curve.domain();
    let root = ParamSpan { t: (t0, t1) };
    let floor = SweepFloor::section(root, floor_meters, speed)?;
    let cells = isolate_section(floor, |cell| {
        let over = pieces.over(cell.t.0, cell.t.1);
        Ok(!over.is_empty() && over.iter().all(|(h, _, _)| one_signed(hull(h))))
    })?;
    // Contiguous survivors are one cluster: a root on a cell's end
    // survives in both cells beside it.
    let mut clusters: Vec<(f64, f64)> = Vec::new();
    for c in cells {
        match clusters.last_mut() {
            Some(last) if last.1 == c.t.0 => last.1 = c.t.1,
            _ => clusters.push(c.t),
        }
    }
    let (mut interior, mut at_start, mut at_end) = (Vec::new(), None, None);
    for (a, b) in clusters {
        let over = pieces.over(a, b);
        let dh = over
            .iter()
            .map(|(_, _, d)| hull(d))
            .reduce(Interval::hull)
            .unwrap_or_else(Interval::refused);
        let w_hi = over
            .iter()
            .map(|(_, w, _)| magnitude(hull(w)))
            .fold(0.0, max_bound);
        // `h` at the cluster's two ends: the first and last Bernstein
        // coefficients of the overlaps there.
        let h_a = over.first().and_then(|(h, _, _)| h.first().copied());
        let h_b = over.last().and_then(|(h, _, _)| h.last().copied());
        let rising = sign(dh);
        // At a root `h = 0`, so `dφ/dt = h′/W` there and
        // `|dφ/ds| ≥ inf|h′| / (sup W · sup‖C′‖)`.
        let slope = if rising.is_some() {
            let num = super::enclose::zero_free_lower_bound(dh);
            let den = magnitude(Interval::point(w_hi) * Interval::point(speed.get()));
            geom_core::interval::div_down(num, den)
        } else {
            0.0
        };
        let found = SectionRoot {
            bracket: (a, b),
            slope,
            rising,
        };
        if a == t0 {
            at_start = Some(found);
            continue;
        }
        if b == t1 {
            at_end = Some(found);
            continue;
        }
        // A monotone `h` whose two ends are one sign has no root here.
        if rising.is_some()
            && let (Some(ha), Some(hb)) = (h_a.and_then(sign), h_b.and_then(sign))
            && ha == hb
        {
            continue;
        }
        decide_crossing(found, arm, band)?;
        interior.push(found);
    }
    Ok(BoundarySection::Roots {
        interior,
        at_start,
        at_end,
    })
}

/// A root's transversality along its curve: the slope, a sine, levered
/// by `arm` (`ssi_boundary_crossing`). It passes on a positive sign;
/// zero or in band is a graze.
///
/// # Errors
///
/// [`SsiError::BoundaryGraze`] with no side named.
pub(crate) fn decide_crossing(root: SectionRoot, arm: f64, band: Band) -> Result<(), SsiError> {
    let margin = Margin::levered(root.slope, arm);
    let Some(verdict) = band_verdict("ssi_boundary_crossing", margin, band) else {
        return Ok(());
    };
    Err(SsiError::BoundaryGraze {
        side: None,
        bracket: root.bracket,
        verdict,
    })
}

/// Settles a root of `φ` inside `bracket` to `|φ| ≤ tol` by safeguarded
/// Newton along the curve: Newton where its step stays inside the
/// bracket, bisection otherwise, a fixed count (D9). `None` when no
/// iterate settles; the caller refuses the end it was settling.
/// `Some(NAN)` when the curve does not evaluate to a finite point at
/// the bracket's ends.
pub(crate) fn settle_root(
    curve: &NurbsCurve3<f64>,
    origin: Point3<f64>,
    normal: Vec3<f64>,
    bracket: (f64, f64),
    tol: f64,
) -> Option<f64> {
    let phi = |t: f64| normal.dot(curve.eval(t) - origin);
    let (mut lo, mut hi) = bracket;
    let (mut f_lo, f_hi) = (phi(lo), phi(hi));
    if !(f_lo.is_finite() && f_hi.is_finite()) {
        return Some(f64::NAN);
    }
    // The end nearer the plane, when an end already settles.
    for (t, f) in [(lo, f_lo), (hi, f_hi)] {
        if f.abs() <= tol {
            return Some(t);
        }
    }
    let mut t = 0.5 * (lo + hi);
    for _ in 0..SECTION_SETTLE_ITERS {
        let f = phi(t);
        if !f.is_finite() {
            return Some(f64::NAN);
        }
        if f.abs() <= tol {
            return Some(t);
        }
        // Keep the sign change bracketed where there is one.
        if (f < 0.0) == (f_lo < 0.0) {
            lo = t;
            f_lo = f;
        } else {
            hi = t;
        }
        let df = normal.dot(curve.deriv(t));
        let newton = t - f / df;
        t = if newton.is_finite() && newton > lo && newton < hi {
            newton
        } else {
            0.5 * (lo + hi)
        };
    }
    (phi(t).abs() <= tol).then_some(t)
}

/// The fixed iteration count of [`settle_root`] (D9): enough bisections
/// to cut any `f64` bracket to adjacent floats.
const SECTION_SETTLE_ITERS: usize = 128;

#[cfg(test)]
mod tests {
    use geom_core::Interval;

    use super::{SsiError, refined_sign};

    /// One Bézier piece of `h` with unit weights.
    fn piece(c: &[f64]) -> (Vec<Interval>, Vec<Interval>) {
        (
            c.iter().map(|&x| Interval::from_certified(x)).collect(),
            vec![Interval::from_certified(1.0); c.len()],
        )
    }

    fn read(pieces: Vec<(Vec<Interval>, Vec<Interval>)>) -> Result<Option<bool>, SsiError> {
        refined_sign(pieces.into_iter())
    }

    /// **A loose hull clear of zero reads its sign.** `m` pieces each
    /// `(P, −N, P)`, `P > N`: every hull holds `−N`, but `φ ≥ (P − N)/2`
    /// on each, and one halving makes both halves one-signed. Mirrored,
    /// it reads negative. Red under no subdivision (`None`) and under the
    /// stop on one-signed removed (the cell budget refuses).
    #[test]
    fn a_loose_hull_clear_of_zero_reads_its_sign() {
        for (p, n) in [(0.6e-9, 0.2e-9), (0.9e-9, 0.5e-9)] {
            for m in [1, 64, 1024] {
                let up = read(vec![piece(&[p, -n, p]); m]);
                assert_eq!(
                    up.ok(),
                    Some(Some(true)),
                    "(P, −N, P) = ({p:e}, {n:e}) × {m}"
                );
                let down = read(vec![piece(&[-p, n, -p]); m]);
                assert_eq!(down.ok(), Some(Some(false)), "(−P, N, −P) × {m}");
            }
        }
    }

    /// **A hull holding a zero of `φ` reads in band, not signed.** A
    /// linear piece through zero, a quadratic with two zeros, and a flush
    /// piece `h ≡ 0`: each reads `None`, the first two once the pieces
    /// about each zero stop narrowing under halving, the flush one at its
    /// first halving. Red under the floor stop removed: the pieces about
    /// the zero never turn one-signed, and the cell budget refuses.
    #[test]
    fn a_hull_holding_a_zero_of_phi_reads_in_band() {
        let e = 1e-9;
        for c in [
            vec![-0.5 * e, 0.5 * e],
            vec![0.3 * e, -0.5 * e, 0.3 * e],
            vec![0.0; 3],
        ] {
            assert_eq!(read(vec![piece(&c)]).ok(), Some(None), "{c:?}");
        }
    }

    /// **Pieces of two signs read no sign.** Each piece is one-signed, so
    /// no refinement runs; the side is not on one side of the plane.
    #[test]
    fn pieces_of_two_signs_read_no_sign() {
        let e = 1e-9;
        let got = read(vec![piece(&[e, e]), piece(&[-e, -e])]);
        assert_eq!(got.ok(), Some(None));
    }
}

#[cfg(test)]
#[path = "section_probe_r2.rs"]
mod probe_r2;
