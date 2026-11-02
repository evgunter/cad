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
//!   it is `On` where that margin is not decided positive. Its sign along
//!   the curve is read on a refined hull ([`refined_sign`]): a piece
//!   whose `h` coefficients straddle zero is halved until they are
//!   one-signed; opposite signs, or a halving that does not narrow a
//!   piece's hull, read no sign, and a side whose halvings pass
//!   [`SIDE_SIGN_HALVINGS`] refuses ([`SsiError::SideSignBudget`]).
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
//! knot rectangle (`super::boundary`). Limb 3's side arm reads any
//! stretch of a side through [`SectionReader`]: the same pieces cut to
//! the stretch ([`Stretch`]), its distance and its refined sign read as
//! the whole side's are.

use geom::NurbsCurve3;
use geom_core::Bounds;
use geom_core::interval::certification::Certification;
use geom_core::interval::max_bound;
use geom_core::k_stats::decide;
use geom_core::spline::compose::{CurveCertData, canonical_composite};
use geom_core::spline::weight_scale::unit_scale_exact;
use geom_core::spline::{RingRefinement, restrict_segment};
use geom_core::{Band, Decide, Indeterminate, Interval, Margin, Point3, Sign, SupSpeed, Vec3};

use super::SsiError;
use super::exhaust::{ParamSpan, SweepFloor, isolate_section};
use super::residual::Curved;
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
    /// `0` where the bound straddles, `NaN` where it is refused.
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
    /// How `φ` reads in metres, for a curved partner; `None` for a
    /// plane, whose `φ` is its distance.
    scale: Option<Scale>,
}

/// **A curved partner's `φ` in metres along a curve**: `φ` is the
/// partner's canonical form `f` ([`Curved`]), read against the band
/// through its divisor over the curve's control hull in the partner's
/// frame, which holds every point of the curve.
struct Scale {
    curved: Curved,
    hull: Vec3<Interval>,
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
        Self::enclosed(curve.knots_f64_interim(), &control, curve.weights(), (o, n))
    }

    /// **The curve's pieces against a curved partner**: `φ = f∘C`, the
    /// partner's canonical form along the curve, in rational Bernstein
    /// form over the curve's knot spans
    /// ([`geom_core::spline::compose::canonical_composite`], the curve ×
    /// face table's ring composite), the curve's net written in the
    /// partner's frame in certification arithmetic. Its denominator is a
    /// power of the weight function, of the numerator's degree, and its
    /// coefficients are positive, so `φ = h/W` as on the plane.
    fn implicit(curve: &NurbsCurve3<f64>, curved: &Curved) -> Option<Self> {
        let framed: Vec<Vec3<Interval>> = curve
            .control()
            .iter()
            .map(|&p| curved.framed_point(p))
            .collect();
        if !framed
            .iter()
            .all(|g| g.x.is_certified() && g.y.is_certified() && g.z.is_certified())
        {
            return None;
        }
        let coords = [
            framed.iter().map(|g| g.x).collect::<Vec<_>>(),
            framed.iter().map(|g| g.y).collect(),
            framed.iter().map(|g| g.z).collect(),
        ];
        // The weights at unit scale where that is exact, as on the plane:
        // numerator and denominator carry one power of the factor.
        let weights: Vec<f64> = unit_scale_exact(curve.weights())
            .map_or_else(|| curve.weights().to_vec(), |(_, scaled)| scaled);
        let data = CurveCertData::new(curve.knots_f64_interim(), &weights, &coords).ok()?;
        let form = canonical_composite(&data, &curved.form()).ok()?;
        let pieces: Vec<Piece> = form
            .num
            .breaks()
            .windows(2)
            .zip(form.num.spans().iter().zip(form.den.spans()))
            .map(|(k, (h, w))| ((k[0], k[1]), h.clone(), w.clone()))
            .collect();
        if pieces.is_empty() || pieces.iter().any(|(_, h, w)| h.len() != w.len()) {
            return None;
        }
        let hull = Vec3::new(hull(&coords[0]), hull(&coords[1]), hull(&coords[2]));
        Some(Self {
            pieces,
            scale: Some(Scale {
                curved: curved.clone(),
                hull,
            }),
        })
    }

    /// An upper bound in metres on the curve's distance from the
    /// partner where `|φ| ≤ sup` along it: `sup` itself on a plane.
    fn metres(&self, sup: f64) -> f64 {
        self.scale
            .as_ref()
            .map_or(sup, |s| s.curved.ceiling_metres(sup, s.hull))
    }

    /// An upper bound on `‖∇f‖` over the curve, which a slope of a
    /// curved partner's `φ` is divided by to read as a sine; `None` on a
    /// plane, whose normal is a unit.
    fn gradient_sup(&self) -> Option<f64> {
        self.scale.as_ref().map(|s| s.curved.gradient_sup(s.hull))
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
        // The weights at unit scale where that is exact, which moves
        // neither the curve, `φ = h/W` nor `h`'s sign, so a net of tiny
        // weights does not underflow `h`.
        let weights: Vec<f64> =
            unit_scale_exact(weights).map_or_else(|| weights.to_vec(), |(_, scaled)| scaled);
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
        let refinement = RingRefinement::new(kv, &add).ok()?;
        let (h, w) = (refinement.refine(&h), refinement.refine(&w));
        let knots = refinement.knots();
        let breaks: Vec<f64> = knots.knot_runs().map(|(k, _)| k).collect();
        let mut pieces = Vec::new();
        for (i, pair) in breaks.windows(2).enumerate() {
            let lo = i * p;
            let (Some(hc), Some(wc)) = (h.get(lo..=lo + p), w.get(lo..=lo + p)) else {
                return None;
            };
            pieces.push(((pair[0], pair[1]), hc.to_vec(), wc.to_vec()));
        }
        Some(Self {
            pieces,
            scale: None,
        })
    }

    /// The plane distance `φ = h/W` over the whole curve, enclosed
    /// ([`ratio_hull`]).
    fn distance(&self) -> Interval {
        ratio_hull(self.pieces.iter().map(|(_, hc, wc)| (hc, wc)))
    }

    /// The sign of `φ` along the whole curve ([`refined_sign`]).
    fn sign(&self, budget: &mut usize) -> Result<Option<bool>, SignBudget> {
        refined_sign(
            self.pieces.iter().map(|(_, hc, _)| hc.clone()).collect(),
            budget,
        )
    }

    /// Each piece overlapping `[a, b]`, with the overlap.
    fn cuts(&self, a: f64, b: f64) -> Vec<(&Piece, (f64, f64))> {
        let mut out = Vec::new();
        for piece in &self.pieces {
            let (k0, k1) = piece.0;
            let (lo, hi) = (a.max(k0), b.min(k1));
            if lo > hi || (lo == hi && a != b) {
                continue;
            }
            out.push((piece, (lo, hi)));
        }
        out
    }

    /// For each piece overlapping `[a, b]`: the Bernstein coefficients
    /// of `h` and `W` over the overlap.
    fn over(&self, a: f64, b: f64) -> Vec<(Vec<Interval>, Vec<Interval>)> {
        self.cuts(a, b)
            .into_iter()
            .map(|((k, hc, wc), cut)| {
                (restrict_segment(hc, *k, cut), restrict_segment(wc, *k, cut))
            })
            .collect()
    }

    /// For each piece overlapping `[a, b]`: the Bernstein coefficients
    /// of `dh/dt` over the overlap.
    fn slope_over(&self, a: f64, b: f64) -> Vec<Vec<Interval>> {
        self.cuts(a, b)
            .into_iter()
            .map(|(((k0, k1), hc, _), cut)| {
                let span = Interval::point(*k1) - Interval::point(*k0);
                #[allow(clippy::cast_precision_loss)]
                let pf = Interval::point((hc.len() - 1) as f64);
                let dh: Vec<Interval> = hc.windows(2).map(|c| pf * (c[1] - c[0]) / span).collect();
                restrict_segment(&dh, (*k0, *k1), cut)
            })
            .collect()
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

/// How many halvings one read of a side's sign may spend
/// ([`refined_sign`]): a named resource wall (D9), the size of the cell
/// budget.
pub(crate) const SIDE_SIGN_HALVINGS: usize = 200_000;

/// A side's sign was not resolved within its halvings
/// ([`SIDE_SIGN_HALVINGS`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SignBudget;

/// **The sign of `φ` over pieces, read on a refined hull.** `φ` shares
/// its sign with `h` (the weights are positive), so each piece's `h`
/// coefficients are read. A piece whose hull straddles zero is halved
/// ([`restrict_segment`]), spending one of `budget`, and its halves are read.
/// Each half is cut from the row it halves.
///
/// - `Some`: every piece reads one-signed, all of one sign.
/// - `None` (in band): two pieces read opposite signs, a hull is
///   refused, or a straddling piece's halves' hull is no narrower than
///   its own (the arithmetic's floor there).
///
/// The pieces as given are all read before any is halved, so two of
/// opposite sign read `None` in any order.
///
/// # Errors
///
/// [`SignBudget`] where a halving is due with `budget` spent.
fn refined_sign(
    pieces: Vec<Vec<Interval>>,
    budget: &mut usize,
) -> Result<Option<bool>, SignBudget> {
    let signs: Vec<Option<bool>> = pieces.iter().map(|c| sign(hull(c))).collect();
    if signs.contains(&Some(true)) && signs.contains(&Some(false)) {
        return Ok(None);
    }
    let mut found = signs.iter().find_map(|s| *s);
    for (piece, read) in pieces.into_iter().zip(signs) {
        if read.is_some() {
            continue;
        }
        let mut stack = vec![piece];
        while let Some(c) = stack.pop() {
            let h = hull(&c);
            if let Some(s) = sign(h) {
                if found.is_some_and(|f| f != s) {
                    return Ok(None);
                }
                found = Some(s);
                continue;
            }
            let before = Certification::width(h);
            if before.is_nan() {
                return Ok(None);
            }
            if *budget == 0 {
                return Err(SignBudget);
            }
            *budget -= 1;
            let (left, right) = (
                restrict_segment(&c, (0.0, 1.0), (0.0, 0.5)),
                restrict_segment(&c, (0.0, 1.0), (0.5, 1.0)),
            );
            if Certification::width(Interval::hull(hull(&left), hull(&right))) >= before {
                return Ok(None);
            }
            stack.push(right);
            stack.push(left);
        }
    }
    Ok(found)
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
            pieces: self.pieces.over(a, b),
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
    /// [`boundary_section`] reads a whole side's, spending `budget`'s
    /// halvings.
    ///
    /// # Errors
    ///
    /// As [`refined_sign`].
    pub(crate) fn sign(&self, budget: &mut usize) -> Result<Option<bool>, SignBudget> {
        refined_sign(
            self.pieces.iter().map(|(hc, _)| hc.clone()).collect(),
            budget,
        )
    }
}

/// The hull of a coefficient list.
fn hull(c: &[Interval]) -> Interval {
    c.iter()
        .copied()
        .reduce(Interval::hull)
        .unwrap_or_else(Interval::refused)
}

/// Whether an enclosure is certified and one-signed.
pub(crate) fn one_signed(i: Interval) -> bool {
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
/// curve); [`SsiError::CellBudget`] for the roots' cells;
/// [`SsiError::SideSignBudget`] (with no side named) where the curve's
/// sign along it is not resolved within its halvings.
pub fn boundary_section(
    curve: &NurbsCurve3<f64>,
    origin: Point3<f64>,
    normal: Vec3<f64>,
    speed: SupSpeed<f64>,
    floor_meters: f64,
    arm: f64,
    band: Band,
) -> Result<BoundarySection, SsiError> {
    section_within(
        (curve, origin, normal),
        speed,
        (floor_meters, arm, band),
        SIDE_SIGN_HALVINGS,
    )
}

/// [`boundary_section`] of a curved partner: `φ` the partner's form
/// along the curve ([`Pieces::implicit`]), its sup read against the band
/// as a ceiling in metres and each root's slope over `‖∇f‖` (module docs
/// of [`super::residual`]); everything else as on the plane.
///
/// # Errors
///
/// As [`boundary_section`].
pub(crate) fn implicit_section(
    curve: &NurbsCurve3<f64>,
    curved: &Curved,
    speed: SupSpeed<f64>,
    floor_meters: f64,
    arm: f64,
    band: Band,
) -> Result<BoundarySection, SsiError> {
    let pieces = Pieces::implicit(curve, curved).ok_or(UNSUPPORTED)?;
    read_section(
        curve,
        &pieces,
        speed,
        (floor_meters, arm, band),
        SIDE_SIGN_HALVINGS,
    )
}

/// [`boundary_roots`] of a curved partner ([`implicit_section`]).
///
/// # Errors
///
/// As [`boundary_section`].
pub(crate) fn implicit_roots(
    curve: &NurbsCurve3<f64>,
    curved: &Curved,
    speed: SupSpeed<f64>,
    floor_meters: f64,
    arm: f64,
    band: Band,
) -> Result<BoundarySection, SsiError> {
    let pieces = Pieces::implicit(curve, curved).ok_or(UNSUPPORTED)?;
    roots(curve, &pieces, speed, floor_meters, arm, band)
}

/// A curve whose Bernstein form the knot algebra, or the composite,
/// refused.
const UNSUPPORTED: SsiError = SsiError::UnsupportedCertificate {
    what: "a boundary curve's Bernstein form was refused by the knot algebra",
};

/// [`boundary_section`], its sign read within `halvings`.
fn section_within(
    (curve, origin, normal): (&NurbsCurve3<f64>, Point3<f64>, Vec3<f64>),
    speed: SupSpeed<f64>,
    (floor_meters, arm, band): (f64, f64, Band),
    halvings: usize,
) -> Result<BoundarySection, SsiError> {
    let pieces = Pieces::of(curve, origin, normal).ok_or(UNSUPPORTED)?;
    read_section(curve, &pieces, speed, (floor_meters, arm, band), halvings)
}

/// The section of a curve read off its pieces, its sign within
/// `halvings`.
fn read_section(
    curve: &NurbsCurve3<f64>,
    pieces: &Pieces,
    speed: SupSpeed<f64>,
    (floor_meters, arm, band): (f64, f64, Band),
    halvings: usize,
) -> Result<BoundarySection, SsiError> {
    // ---- On: the curve's sup distance from the plane ----
    let d = pieces.distance();
    let sup = magnitude(d);
    let on = || -> Result<BoundarySection, SsiError> {
        Ok(BoundarySection::On {
            sup,
            side_of_plane: pieces.sign(&mut { halvings }).map_err(|SignBudget| {
                SsiError::SideSignBudget {
                    side: None,
                    halvings,
                }
            })?,
        })
    };
    match decide(
        "ssi_boundary_on_plane",
        Margin::of(pieces.metres(sup)),
        band,
    ) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) => return on(),
        Err(cause) if cause.margin.is_invalid() => {
            return Err(super::TraceDecision::BoundarySection.escalated(cause));
        }
        // In band: the curve lies within the band's reach of the plane,
        // and the caller reports it as a region, not a crossing.
        Err(_) => return on(),
    }

    roots(curve, pieces, speed, floor_meters, arm, band)
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
        Ok(!over.is_empty() && over.iter().all(|(h, _)| one_signed(hull(h))))
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
        let dh = pieces
            .slope_over(a, b)
            .iter()
            .map(|d| hull(d))
            .reduce(Interval::hull)
            .unwrap_or_else(Interval::refused);
        let w_hi = over
            .iter()
            .map(|(_, w)| magnitude(hull(w)))
            .fold(0.0, max_bound);
        // `h` at the cluster's two ends: the first and last Bernstein
        // coefficients of the overlaps there.
        let h_a = over.first().and_then(|(h, _)| h.first().copied());
        let h_b = over.last().and_then(|(h, _)| h.last().copied());
        let rising = sign(dh);
        // At a root `h = 0`, so `dφ/dt = h′/W` there and
        // `|dφ/ds| ≥ inf|h′| / (sup W · sup‖C′‖)`.
        // A refused slope hull measured nothing: `NaN`, which
        // [`decide_crossing`] refuses where the slope is decided, rather
        // than a zero slope it would read as a graze.
        let slope = if rising.is_some() {
            let num = super::enclose::zero_free_lower_bound(dh);
            let den = Interval::point(w_hi) * Interval::point(speed.get());
            let den = magnitude(match pieces.gradient_sup() {
                Some(g) => den * Interval::point(g),
                None => den,
            });
            geom_core::interval::div_down(num, den)
        } else if dh.is_certified() {
            0.0
        } else {
            f64::NAN
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
/// [`SsiError::BoundaryGraze`] with no side named;
/// [`SsiError::UnsupportedCertificate`] for a refused slope, which
/// measured nothing.
pub(crate) fn decide_crossing(root: SectionRoot, arm: f64, band: Band) -> Result<(), SsiError> {
    // No root of a validated curve reaches this: its slope hull is
    // refused only from non-finite controls, and an overflow rounds to
    // a half-unbounded certified hull. The guard's row asks it directly.
    if root.slope.is_nan() {
        return Err(SsiError::UnsupportedCertificate {
            what: "a boundary curve's slope at a root has no certified bound",
        });
    }
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
    settle_by(
        |t| normal.dot(curve.eval(t) - origin),
        |t| normal.dot(curve.deriv(t)),
        bracket,
        tol,
    )
}

/// [`settle_root`] for a curved partner: the partner's residual in
/// metres along the curve ([`Curved::residual`]) settled to `tol`.
pub(crate) fn settle_implicit_root(
    curve: &NurbsCurve3<f64>,
    curved: &Curved,
    bracket: (f64, f64),
    tol: f64,
) -> Option<f64> {
    settle_by(
        |t| curved.residual(curve.eval(t)),
        |t| curved.residual_rate(curve.eval(t), curve.deriv(t)),
        bracket,
        tol,
    )
}

/// [`settle_root`]'s safeguarded Newton on `phi`, its rate `dphi`.
fn settle_by(
    phi: impl Fn(f64) -> f64,
    dphi: impl Fn(f64) -> f64,
    bracket: (f64, f64),
    tol: f64,
) -> Option<f64> {
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
        let df = dphi(t);
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
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use geom::NurbsCurve3;
    use geom_core::Bounds as _;
    use geom_core::interval::certification::Certification as _;
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Interval, Point3, SupSpeed, Vec3};

    use super::{BoundarySection, SIDE_SIGN_HALVINGS, SignBudget, SsiError, refined_sign};

    /// **A refused slope is no graze.** A root whose slope hull was
    /// refused (`NaN`) refuses typed where the slope is decided; a zero
    /// slope, which was measured, is still a graze.
    #[test]
    fn a_refused_slope_refuses_where_the_crossing_is_decided() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let root = |slope| super::SectionRoot {
            bracket: (0.25, 0.5),
            slope,
            rising: None,
        };
        let refused = super::decide_crossing(root(f64::NAN), 1.0, band);
        assert!(
            matches!(refused, Err(SsiError::UnsupportedCertificate { .. })),
            "{refused:?}"
        );
        let zero = super::decide_crossing(root(0.0), 1.0, band);
        assert!(
            matches!(zero, Err(SsiError::BoundaryGraze { .. })),
            "{zero:?}"
        );
    }

    /// One Bézier piece of `h`.
    fn piece(c: &[f64]) -> Vec<Interval> {
        c.iter().map(|&x| Interval::from_certified(x)).collect()
    }

    fn read(pieces: Vec<Vec<Interval>>) -> Result<Option<bool>, SignBudget> {
        refined_sign(pieces, &mut SIDE_SIGN_HALVINGS.clone())
    }

    /// `(P, −N, P)` with `P > N`: its hull holds `−N`, but `φ ≥ (P − N)/2`
    /// and one halving makes both halves one-signed.
    const P: f64 = 0.6e-9;
    const N: f64 = 0.2e-9;

    /// **A loose hull clear of zero reads its sign.** `m` pieces each
    /// `(P, −N, P)`, and mirrored. Red under no subdivision (`None`) and
    /// under the stop on one-signed removed (the budget refuses).
    #[test]
    fn a_loose_hull_clear_of_zero_reads_its_sign() {
        for (p, n) in [(P, N), (0.9e-9, 0.5e-9)] {
            for m in [1, 64, 1024] {
                let up = read(vec![piece(&[p, -n, p]); m]);
                assert_eq!(up, Ok(Some(true)), "(P, −N, P) = ({p:e}, {n:e}) × {m}");
                let down = read(vec![piece(&[-p, n, -p]); m]);
                assert_eq!(down, Ok(Some(false)), "(−P, N, −P) × {m}");
            }
        }
    }

    /// **A hull holding a zero of `φ` reads in band, not signed.** A
    /// linear piece through zero, a quadratic with two zeros, and a flush
    /// piece `h ≡ 0`: each reads `None` once a piece's halves stop
    /// narrowing it. Red under that stop removed: the pieces about the
    /// zero never turn one-signed, and the budget refuses.
    #[test]
    fn a_hull_holding_a_zero_of_phi_reads_in_band() {
        let e = 1e-9;
        for c in [
            vec![-0.5 * e, 0.5 * e],
            vec![0.3 * e, -0.5 * e, 0.3 * e],
            vec![0.0; 3],
        ] {
            assert_eq!(read(vec![piece(&c)]), Ok(None), "{c:?}");
        }
    }

    /// **A refused hull reads in band.** A piece one of whose
    /// coefficients is refused has no sign to read and no width to halve.
    /// Red under the refused stop removed: the halves stay refused, and
    /// the budget refuses.
    #[test]
    fn a_refused_hull_reads_in_band() {
        let c = vec![
            Interval::from_certified(P),
            Interval::refused(),
            Interval::from_certified(P),
        ];
        assert_eq!(read(vec![c]), Ok(None));
    }

    /// **Pieces of two signs read no sign, in either order**, whether
    /// they read one-signed as given or only once halved. Red under the
    /// opposite-sign stop removed (the last sign read wins) and, for the
    /// pieces as given, under it read only as they are reached: a piece
    /// that would spend the budget before the opposite one then refuses
    /// in one order and not the other.
    #[test]
    fn pieces_of_two_signs_read_no_sign_in_either_order() {
        let e = 1e-9;
        let rows = [
            ("as given", piece(&[e, e]), piece(&[-e, -e])),
            ("once halved", piece(&[P, -N, P]), piece(&[-P, N, -P])),
        ];
        for (at, a, b) in rows {
            assert_eq!(read(vec![a.clone(), b.clone()]), Ok(None), "{at}");
            assert_eq!(read(vec![b, a]), Ok(None), "{at}, reversed");
        }
        let costly = vec![piece(&[P, -N, P]); 4];
        let mut forward = costly.clone();
        forward.push(piece(&[-e, -e]));
        let mut reversed = vec![piece(&[-e, -e])];
        reversed.extend(costly);
        for (at, pieces) in [("forward", forward), ("reversed", reversed)] {
            assert_eq!(
                refined_sign(pieces, &mut 2),
                Ok(None),
                "{at}, budget 2 of 4"
            );
        }
    }

    /// **A sign a few ulps of the row clear of zero reads on halves of
    /// halves.** `(a, −b, c)` with `b² < a·c` in exact arithmetic, so `h`
    /// is positive with its least value near `9.4e−18`, about `2⁻⁵⁵·⁵` of
    /// the `0.49` row, and mirrored. Red under each half cut from the piece's own
    /// row rather than from the half it halves: the halves about the
    /// minimum then carry the whole row's rounding, and read `None`.
    #[test]
    fn a_sign_at_the_rows_floor_reads_on_halves_of_halves() {
        let row = [0.489_999_999_999_999_94, -0.21, 0.090_000_000_000_000_02];
        assert_eq!(read(vec![piece(&row)]), Ok(Some(true)), "{row:?}");
        let mirrored = row.map(|x| -x);
        assert_eq!(
            read(vec![piece(&mirrored)]),
            Ok(Some(false)),
            "{mirrored:?}"
        );
    }

    /// **The budget counts halvings, and only halvings.** `m` loose
    /// pieces spend one halving each: a budget of `m` reads their sign,
    /// `m − 1` refuses. Pieces one-signed as given spend none, however
    /// many. Red under the budget check removed, or charged per piece
    /// read.
    #[test]
    fn the_sign_spends_one_halving_per_piece_halved() {
        let m = 64;
        let loose = vec![piece(&[P, -N, P]); m];
        assert_eq!(refined_sign(loose.clone(), &mut { m }), Ok(Some(true)));
        assert_eq!(refined_sign(loose, &mut (m - 1)), Err(SignBudget));
        let tight = vec![piece(&[P, P, P]); 4 * m];
        assert_eq!(refined_sign(tight, &mut 0), Ok(Some(true)));
    }

    /// `(0, y, h(y))` over `m` C0 quadratic spans of `y ∈ [0, 1]`, each
    /// `(P, −N, P)` in `h`: the side of a loose wall, against `z = 0`.
    fn loose_side(m: u32) -> NurbsCurve3<f64> {
        let mut kv = vec![0.0, 0.0, 0.0];
        for i in 1..m {
            let t = f64::from(i) / f64::from(m);
            kv.extend([t, t]);
        }
        kv.extend([1.0, 1.0, 1.0]);
        let n = 2 * m + 1;
        let control = (0..n)
            .map(|j| {
                let h = if j % 2 == 0 { P } else { -N };
                Point3::new(0.0, f64::from(j) / f64::from(n - 1), h)
            })
            .collect();
        NurbsCurve3::new(
            KnotVector::clamped(kv, 2).unwrap(),
            control,
            vec![1.0; n as usize],
        )
        .unwrap()
    }

    /// **The boundary section reads a loose side's sign, and refuses one
    /// it cannot resolve within its halvings.** The side spends one
    /// halving per span: with as many it is `On` above the plane; with one
    /// fewer it refuses [`SsiError::SideSignBudget`]. Red under the
    /// refusal swallowed (read as no sign).
    #[test]
    fn the_boundary_section_refuses_a_sign_past_its_halvings() {
        let m = 16;
        let side = loose_side(m);
        let band = Band::new(1e-9, 1e-8).unwrap();
        let read = |halvings| {
            super::section_within(
                (&side, Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
                SupSpeed::new(1.0),
                (1e-9, 1.0, band),
                halvings,
            )
        };
        assert!(
            matches!(
                read(m as usize),
                Ok(BoundarySection::On {
                    side_of_plane: Some(true),
                    ..
                })
            ),
            "{:?}",
            read(m as usize)
        );
        assert!(
            matches!(
                read(m as usize - 1),
                Err(SsiError::SideSignBudget {
                    side: None,
                    halvings
                }) if halvings == m as usize - 1
            ),
            "{:?}",
            read(m as usize - 1)
        );
    }

    /// **A weight spread that a rescale would lose bits of is read as
    /// described.** Carrying `4` to one scales the smallest subnormal
    /// beside it to zero, which would make `W` vanish at the curve's
    /// end; the net's own weights reach the pieces instead.
    #[test]
    fn a_lossy_weight_spread_is_not_rescaled() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control =
            [[0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [2.0, 0.0, 0.0]].map(|c| c.map(Interval::point));
        let tiny = f64::from_bits(1);
        let origin = [0.0, -1.0, 0.0].map(Interval::point);
        let normal = [0.0, 1.0, 0.0].map(Interval::point);
        let pieces =
            super::Pieces::enclosed(&kv, &control, &[tiny, 4.0, tiny], (origin, normal)).unwrap();
        let w = &pieces.pieces[0].2;
        for end in [w[0], w[2]] {
            assert!(
                end.lo() == tiny && end.hi() == tiny,
                "an end weight reads {end:?}, not the described {tiny:e}"
            );
        }
    }
}
