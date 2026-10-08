//! **The consumed extent** — where a carrier verdict is consumed, and
//! the lever arm it gives an angular datum (D4 ¶1: an angle means the
//! displacement it induces at the extent over which the decision is
//! consumed): a ball enclosing the consumed points ([`ExtentBall`]), and
//! what a section classifier reads its axis rows across ([`Reach`]).
//!
//! A ladder that pins a carrier's position at a PIVOT and its
//! direction by an angle reads a relative tilt θ as a displacement of
//! at most `θ · |x − pivot|` at a consumed point `x`, ON TOP OF what
//! the position datum reads at the pivot. [`ExtentBall::lever_from`] is
//! the supremum of `|x − pivot|` over the ball, so the displacement at
//! every point the ball covers is at most the position datum plus the
//! tilt levered there: a reader that bridges a residue decides that
//! SUM, never the two terms one at a time (each just inside the band
//! would sum to nearly twice it). The section rows that serve a verdict
//! beside such a datum read that sum on each verdict's own side
//! (`decide_across` in `crate::intersect`): `pc_parallel_gap`,
//! `cc_coaxial`, `cc_parallel_gap`, `tangent_locus_internal_gap`,
//! `coc_coaxial`, `pt_spiric_two_ovals`, `pt_cap_gap` and
//! `pn_apex_section`. The tightest lever reads the position
//! datum at the pivot nearest the ball's centre ([`ExtentBall::foot_on`]
//! for an axis), where the lever is little more than the ball's radius.
//!
//! An extent that UNDER-states the consumed region makes a tilt read
//! smaller than it is, which is the wrong-answer direction, so every
//! constructor here encloses the region it is handed. A ball says
//! nothing about where the consumed region actually reaches, so a
//! displacement read at its far side is an upper bound only, never
//! evidence that a consumed point stands that far off.
//!
//! **An OVER-stated extent is not safe either**, and that is what
//! [`Reach`] is for. On a one-sided row a lever past the consumed
//! extent only escalates more. On a two-sided row whose definite side
//! is a SERVED class (a tilt that names an ellipse, a sine that names
//! crossing axes) it decides a reading that is in the band at the
//! consumed extent, and serves a class the arm cannot tell from its
//! neighbour. The rule the section classifiers' callers hold: **a lever
//! is never shorter than the region it consumes, and no longer than the
//! caller's own measure of that region from the point it is read at.**
//! No ball chosen around the consumed region is a lever. An edge's span
//! ([`Reach::Span`]) is levered by its per-carrier farthest distance
//! from the pivot that makes that distance least on each axis; a face's
//! measures read a conic edge over the span it holds instead
//! ([`Reach::span_reach_from`]), since a short arc of a large rim
//! reaches nowhere near its rim's size. The
//! cylinder pair's caller ([`Reach::Measured`]) hands a length it
//! measured, and only that pair floors it ([`Reach::lever_between`]).
//! A plane×cylinder face's caller ([`Reach::Face`]) hands what it
//! measured of the face from `at`: how far it reaches either way along
//! the axis ([`Reach::range_along`] of each boundary span) and how far
//! it reaches at all. The plane×cylinder row reads them at the RULINGS'
//! HINGE, the point a face lever is read from: the axial reach from the
//! hinge's station ([`Reach::hinge_lever`]), and the reach across the
//! wall from the hinge, `at`'s own offset from it included
//! ([`Reach::turn_lever`]). The classifier adds no length of its own.
//!
//! The tangent-locus witness reads a ball, [`Reach::Ball`]: the one its
//! callers, the carrier doors, hand it.
//!
//! Everything is comparison-free: `max` and `min` are the [`Real`]
//! lattice operations.

use geom::{Curve3, Surface};
use geom_core::{Point3, Real, Vec3, is_finite_length};

/// A closed ball `|x − center| ≤ radius` enclosing a consumed region
/// (module docs).
#[derive(Clone, Copy, Debug)]
pub struct ExtentBall<T: Real> {
    center: Point3<T>,
    radius: T,
}

impl<T: Real> ExtentBall<T> {
    /// The ball of `radius` about `center`. `radius` is a length the
    /// caller vouches encloses the region it names.
    #[must_use]
    pub fn new(center: Point3<T>, radius: T) -> Self {
        Self { center, radius }
    }

    /// The single point `p`.
    #[must_use]
    pub fn point(p: Point3<T>) -> Self {
        Self::new(p, T::zero())
    }

    /// A ball enclosing every ball of `parts`: about the mean of their
    /// centres, reaching the far side of each. `None` for no parts.
    #[must_use]
    pub fn enclosing(parts: &[Self]) -> Option<Self> {
        let first = parts.first()?;
        let n = T::from_f64(parts.len() as f64);
        let sum = parts
            .iter()
            .fold(Vec3::new(T::zero(), T::zero(), T::zero()), |acc, b| {
                acc + (b.center - first.center)
            });
        let center = first.center + sum / n;
        let radius = parts
            .iter()
            .fold(T::zero(), |r, b| r.max(b.lever_from(center)));
        Some(Self::new(center, radius))
    }

    /// The ball's centre.
    #[must_use]
    pub fn center(self) -> Point3<T> {
        self.center
    }

    /// The ball's radius: its lever from its own centre.
    #[must_use]
    pub fn radius(self) -> T {
        self.radius
    }

    /// The farthest the ball reaches from `pivot`: the lever arm at
    /// which an angular datum pinned at `pivot` is metered.
    #[must_use]
    pub fn lever_from(self, pivot: Point3<T>) -> T {
        (pivot - self.center).norm() + self.radius
    }

    /// The point of the line `origin + s·axis` (`axis` unit) nearest
    /// the ball's centre: where a datum on that line is read so that
    /// the tilt's lever from it is least.
    #[must_use]
    pub fn foot_on(self, origin: Point3<T>, axis: Vec3<T>) -> Point3<T> {
        foot(self.center, origin, axis)
    }

    /// The ball, if it reads: `None` where its centre or radius is
    /// poison or infinite (a box with no claim to make), whose lever
    /// would meter nothing.
    #[must_use]
    pub fn readable(self) -> Option<Self> {
        let c = self.center;
        [c.x, c.y, c.z, self.radius]
            .into_iter()
            .all(is_finite_length)
            .then_some(self)
    }

    /// The ball enclosing the box `[lo, hi]`: its centre, out to a
    /// corner.
    #[must_use]
    pub fn of_box(lo: Point3<T>, hi: Point3<T>) -> Self {
        let half = (hi - lo) * T::from_f64(0.5);
        Self::new(lo + half, half.norm())
    }

    /// The ball a sphere or a torus fits in, which encloses every face
    /// on it whatever its trim: the sphere's own, the torus's of radius
    /// `R + r` about its centre. `None` for every other kind, whose
    /// faces a box of their own encloses.
    #[must_use]
    pub fn of_carrier(surface: &Surface<T>) -> Option<Self> {
        match surface {
            Surface::Sphere { center, radius, .. } => Some(Self::new(*center, *radius)),
            Surface::Torus {
                center,
                major_radius,
                minor_radius,
                ..
            } => Some(Self::new(*center, *major_radius + *minor_radius)),
            Surface::Plane { .. }
            | Surface::Cylinder { .. }
            | Surface::Cone { .. }
            | Surface::Nurbs(_)
            | Surface::Approx(_) => None,
        }
    }
}

/// The point of the line `origin + s·axis` (`axis` unit) nearest `p`.
fn foot<T: Real>(p: Point3<T>, origin: Point3<T>, axis: Vec3<T>) -> Point3<T> {
    origin + axis * (p - origin).dot(axis)
}

/// **How far a conic arc reaches along a direction, either way**: the
/// least and the greatest `(x − pivot)·dir` over
/// `x = c + u·a·cos t + v·b·sin t`, `t ∈ [t0, t1]`. Along `dir` the arc
/// reads `C + A·cos t + B·sin t`, whose crests `C ± R`
/// (`R = √(A² + B²)`) stand where `cos(t − t*) = ±1`; a crest lies in
/// the span iff its cosine against the span's middle `m` is at least
/// `cos h`, `h` the half-span (capped at a half-turn, past which every
/// crest is in). So each extreme is an end's reading or a crest, the
/// crest read only where the test puts it in the span.
/// Comparison-free ([`Real::select_le_zero`]): an enclosure that cannot
/// tell takes the hull of the crest and the end.
fn conic_arc_range<T: Real>(
    (c, u, v): (Point3<T>, Vec3<T>, Vec3<T>),
    (a, b): (T, T),
    (t0, t1): (T, T),
    pivot: Point3<T>,
    dir: Vec3<T>,
) -> (T, T) {
    let mid = (c - pivot).dot(dir);
    let (pa, pb) = (a * u.dot(dir), b * v.dot(dir));
    let read = |t: T| {
        let (s, co) = t.sin_cos();
        mid + pa * co + pb * s
    };
    let crest = (pa.powi(2) + pb.powi(2)).sqrt();
    let (r0, r1) = (read(t0), read(t1));
    let ends = (r0.min(r1), r0.max(r1));
    let high = crest_in_span((pa, pb), (t0, t1)).select_le_zero(mid + crest, ends.1);
    let low = crest_in_span((-pa, -pb), (t0, t1)).select_le_zero(mid - crest, ends.0);
    (ends.0.min(low), ends.1.max(high))
}

/// **Whether the crest of `A·cos t + B·sin t` lies in `[t0, t1]`**, as a
/// reading at most zero iff it does: `R·cos h − (A·cos m + B·sin m)`, `m`
/// the span's middle and `h` its half-width capped at a half-turn (past
/// which every crest is in), `R = √(A² + B²)`. The crest stands where
/// `cos(t − t*) = 1`, so it is in the span iff its cosine against `m` is
/// at least `cos h`. Read with [`Real::select_le_zero`], an enclosure
/// that cannot tell takes both arms.
fn crest_in_span<T: Real>((pa, pb): (T, T), (t0, t1): (T, T)) -> T {
    let half = T::from_f64(0.5);
    let (sm, cm) = ((t0 + t1) * half).sin_cos();
    let edge = (pa.powi(2) + pb.powi(2)).sqrt()
        * ((t1 - t0) * half)
            .abs()
            .min(T::from_f64(core::f64::consts::PI))
            .cos();
    edge - (pa * cm + pb * sm)
}

/// **How far a circle arc reaches from a point, exactly**: the farthest
/// `|x − pivot|` over `x = c + r·(u·cos t + v·sin t)`, `t ∈ [t0, t1]`.
/// With `pivot` standing `h` off the circle's plane and `ρ` from its
/// centre within it, `|x − pivot|² = h² + r² + ρ² − 2·|r|·ρ·cos(t − φ)`,
/// whose one interior maximum is the crest opposite the pivot,
/// `√(h² + (|r| + ρ)²)`. So the reach is the larger of the two ends' own
/// distances and, where the crest lies in the span ([`crest_in_span`]
/// along `−(pivot − c)`), that crest: every term a sum of positives,
/// none a difference of carrier-sized squares, so a short arc far from
/// its centre loses nothing to cancellation. A pivot on the axis (`ρ`
/// zero) reads every point at the crest's distance. Capped at the whole
/// turn, `|c − pivot| + |r|`, which the crest reaches only to rounding.
fn circle_arc_reach<T: Real>(
    (c, u, v): (Point3<T>, Vec3<T>, Vec3<T>),
    r: T,
    (t0, t1): (T, T),
    pivot: Point3<T>,
) -> T {
    let at = |t: T| {
        let (s, co) = t.sin_cos();
        c + (u * co + v * s) * r
    };
    let w = pivot - c;
    let k = u.cross(v);
    let h = w.dot(k);
    let rho = (w - k * h).norm();
    let ends = (at(t0) - pivot).norm().max((at(t1) - pivot).norm());
    let crest = (h.powi(2) + (r.abs() + rho).powi(2)).sqrt();
    let away = (-(r * u.dot(w)), -(r * v.dot(w)));
    ends.max(crest_in_span(away, (t0, t1)).select_le_zero(crest, T::zero()))
        .min(w.norm() + r.abs())
}

/// **How far an ellipse arc reaches from a point**: an upper bound on
/// `|x − pivot|` over `x = c + u·a·cos t + v·b·sin t`, `t ∈ [t0, t1]`,
/// never past the whole turn's `|c − pivot| + max(|a|, |b|)`. Its
/// farthest point from a point is the root of a quartic, so it is
/// bounded rather than found (a circle's is exact, [`circle_arc_reach`]).
///
/// The span (capped at one turn either way) is cut into four pieces of
/// half-width `h ≤ π/4`. On the unit circle a piece about its middle
/// `m` lies in the rectangle its chord spans out to the arc's crest:
/// across it within the chord's ends, and along `e(m)` within
/// `[cos h, 1]`. The conic is that circle's affine image, so each piece
/// lies in the parallelogram on its ends shifted by
/// `S = (1 − cos h)·(x(m) − c)`, and a distance from a point is convex,
/// so it peaks at a corner. The bound is never short of the arc, to
/// rounding, and passes it by at most a piece's bulge `|S|`; it is read
/// from the ends and the shift, never from a difference of carrier-sized
/// squares, so a short arc far from its centre loses nothing to
/// cancellation.
fn conic_arc_reach<T: Real>(
    (c, u, v): (Point3<T>, Vec3<T>, Vec3<T>),
    (a, b): (T, T),
    (t0, t1): (T, T),
    pivot: Point3<T>,
) -> T {
    let turn = T::from_f64(core::f64::consts::TAU);
    let quarter = ((t1 - t0).max(-turn).min(turn)) * T::from_f64(0.25);
    let at = |t: T| {
        let (s, co) = t.sin_cos();
        c + u * (a * co) + v * (b * s)
    };
    let shift = T::one() - (quarter * T::from_f64(0.5)).abs().cos();
    let far = (0..4).fold((at(t0) - pivot).norm(), |far, k| {
        let s0 = t0 + quarter * T::from_f64(f64::from(k));
        let s1 = s0 + quarter;
        let bulge = (at(s0 + quarter * T::from_f64(0.5)) - c) * shift;
        let (x0, x1) = (at(s0), at(s1));
        [x0 + bulge, x1, x1 + bulge]
            .into_iter()
            .fold(far, |far, x| far.max((x - pivot).norm()))
    });
    far.min((c - pivot).norm() + a.abs().max(b.abs()))
}

/// **What a section classifier reads its axis rows across**: where on
/// an axis the gap is read ([`Reach::foot_on`]), and the lever a tilt
/// pinned there is metered at ([`Reach::lever_from`]). The module docs
/// state the rule its callers hold.
#[derive(Clone, Debug)]
pub enum Reach<T: Real> {
    /// A ball enclosing the consumed region, read at the foot of its
    /// centre and levered out to its far side
    /// ([`ExtentBall::lever_from`]): the tangent-locus witness's reading,
    /// the ball its callers (the carrier doors) hand it.
    Ball(ExtentBall<T>),
    /// A length the caller measured from `at` (the germ frame's walls'
    /// span or the larger radius), read at the foot of `at`. Its lever
    /// is that length ([`Self::lever_from`]), floored for the cylinder
    /// pair at the foot's distance from `at` ([`Self::lever_between`]).
    Measured {
        /// The point the caller measured from.
        at: Point3<T>,
        /// The caller's length.
        lever: T,
    },
    /// A face on the cylinder the caller measured from `at`, read at the
    /// foot of `at`: how far it reaches either way along the cylinder's
    /// axis from `at` ([`Self::hinge_lever`]), and its farthest distance
    /// from `at` ([`Self::turn_lever`]). The plane×cylinder callers hand
    /// it.
    Face {
        /// The point the caller measured from.
        at: Point3<T>,
        /// How far the face reaches from `at` against the axis.
        below: T,
        /// How far the face reaches from `at` along the axis.
        above: T,
        /// The face's farthest distance from `at`.
        across: T,
    },
    /// An edge's carrier over `[t0, t1]`, read at the axis point whose
    /// lever to it is least ([`Self::foot_on`]) and levered by the
    /// carrier's per-carrier farthest distance from the pivot
    /// ([`Self::lever_from`]).
    Span {
        /// The edge's carrier.
        carrier: Curve3<T>,
        /// The span's start parameter.
        t0: T,
        /// The span's end parameter.
        t1: T,
    },
}

impl<T: Real> Reach<T> {
    /// The lever a tilt pinned at `pivot` is metered at: an upper bound
    /// on the consumed region's distance from `pivot`.
    ///
    /// - [`Self::Ball`]: the ball's far side from `pivot`.
    /// - [`Self::Measured`]: the caller's length, whatever the pivot:
    ///   the germ frame's longer of the larger radius and the walls'
    ///   span, which the cylinder pair also floors at the foot's distance
    ///   from `at` ([`Self::lever_between`]).
    /// - [`Self::Face`]: the larger of its two axial sides. No caller
    ///   reads it: the plane×cylinder row reads a face from its hinge
    ///   ([`Self::hinge_lever`]), and no caller hands the cylinder pair a
    ///   face.
    /// - [`Self::Span`]: per carrier, never an underestimate and exact
    ///   where the carrier allows:
    ///   - a **line** segment: its endpoints (distance to a point is
    ///     convex along a line, so a segment attains its maximum at an
    ///     end);
    ///   - a **circle** or **ellipse**: centre distance plus the radius,
    ///     or the larger semi-axis MAGNITUDE (the mint certifies an
    ///     ellipse stored with `minor > major` or a negative `major`),
    ///     whatever the parameter span — a closed rim, whose two
    ///     endpoints coincide, is exactly the case sampling misses;
    ///   - a **spiric** (a curve on a torus): centre distance plus
    ///     `R + r`;
    ///   - a **NURBS** carrier: its control points (the convex-hull
    ///     property of positive weights).
    #[must_use]
    pub fn lever_from(&self, pivot: Point3<T>) -> T {
        match self {
            Self::Ball(ball) => ball.lever_from(pivot),
            Self::Measured { lever, .. } => *lever,
            Self::Face { below, above, .. } => below.max(*above),
            Self::Span { carrier, .. } => match carrier {
                Curve3::Circle { center, radius, .. } => (*center - pivot).norm() + radius.abs(),
                Curve3::Ellipse {
                    center,
                    major,
                    minor,
                    ..
                } => (*center - pivot).norm() + major.abs().max(minor.abs()),
                Curve3::Spiric {
                    center,
                    major_radius,
                    minor_radius,
                    ..
                } => (*center - pivot).norm() + major_radius.abs() + minor_radius.abs(),
                Curve3::Line { .. } | Curve3::Nurbs(_) => self
                    .span_points()
                    .iter()
                    .fold(T::zero(), |m, &p| m.max((p - pivot).norm())),
            },
        }
    }

    /// **The farthest the consumed region stands from `pivot`, over the
    /// span it holds**: [`Self::lever_from`], except that a circle or
    /// ellipse [`Self::Span`] is read over `[t0, t1]` rather than round
    /// its whole turn, which on a short arc of a large conic is the
    /// conic's size, not the arc's: a circle arc exactly, to rounding
    /// ([`circle_arc_reach`]), and an ellipse arc never short of it, to
    /// rounding, and past it by at most a quarter of its span's bulge
    /// ([`conic_arc_reach`]). Never past [`Self::lever_from`]. The face measures read it
    /// (`splitting::rules::face_reach_from` in `topo`); the section
    /// arms' anchors read [`Self::lever_from`].
    #[must_use]
    pub fn span_reach_from(&self, pivot: Point3<T>) -> T {
        match self {
            Self::Span {
                carrier:
                    Curve3::Circle {
                        center,
                        axis: k,
                        radius,
                        u_ref,
                    },
                t0,
                t1,
            } => circle_arc_reach(
                (*center, *u_ref, k.cross(*u_ref)),
                *radius,
                (*t0, *t1),
                pivot,
            ),
            Self::Span {
                carrier:
                    Curve3::Ellipse {
                        center,
                        axis: k,
                        major,
                        minor,
                        u_ref,
                    },
                t0,
                t1,
            } => conic_arc_reach(
                (*center, *u_ref, k.cross(*u_ref)),
                (*major, *minor),
                (*t0, *t1),
                pivot,
            ),
            Self::Ball(_) | Self::Measured { .. } | Self::Face { .. } | Self::Span { .. } => {
                self.lever_from(pivot)
            }
        }
    }

    /// How far the consumed region reaches from `pivot` along `dir`,
    /// either way: bounds on the least and the greatest `(x − pivot)·dir`
    /// over the consumed points `x`, never inside the true ones. The
    /// plane×cylinder face callers measure a face with this, one
    /// [`Self::Span`] per boundary edge ([`Self::Face`]).
    ///
    /// - [`Self::Ball`]: the centre's reading, less and plus the radius
    ///   times `|dir|`.
    /// - [`Self::Measured`]: the caller's length either way;
    ///   [`Self::Face`]: its `below` and `above` (along its cylinder's
    ///   axis, which `dir` is).
    /// - [`Self::Span`], per carrier:
    ///   - a **line** segment or a **NURBS** carrier: its endpoints or
    ///     its whole control net (a linear function peaks over a segment
    ///     or a convex hull at a vertex of it); the net is the whole
    ///     carrier's, not the span's;
    ///   - a **circle** or **ellipse**: exact over `[t0, t1]`
    ///     ([`conic_arc_range`]);
    ///   - a **spiric**: its torus's support either way,
    ///     `(c − pivot)·dir ∓ (R·|dir × k| + r·|dir|)`, whatever the span.
    #[must_use]
    pub fn range_along(&self, pivot: Point3<T>, dir: Vec3<T>) -> (T, T) {
        let at = |p: Point3<T>| (p - pivot).dot(dir);
        let points = |ps: Vec<Point3<T>>| {
            let first = at(ps[0]);
            ps.iter().fold((first, first), |(lo, hi), &p| {
                (lo.min(at(p)), hi.max(at(p)))
            })
        };
        match self {
            Self::Ball(ball) => {
                let (c, r) = (at(ball.center()), ball.radius() * dir.norm());
                (c - r, c + r)
            }
            Self::Measured { lever, .. } => (-*lever, *lever),
            Self::Face { below, above, .. } => (-*below, *above),
            Self::Span { carrier, t0, t1 } => match carrier {
                Curve3::Circle {
                    center,
                    axis: k,
                    radius,
                    u_ref,
                } => conic_arc_range(
                    (*center, *u_ref, k.cross(*u_ref)),
                    (*radius, *radius),
                    (*t0, *t1),
                    pivot,
                    dir,
                ),
                Curve3::Ellipse {
                    center,
                    axis: k,
                    major,
                    minor,
                    u_ref,
                } => conic_arc_range(
                    (*center, *u_ref, k.cross(*u_ref)),
                    (*major, *minor),
                    (*t0, *t1),
                    pivot,
                    dir,
                ),
                Curve3::Spiric {
                    center,
                    axis: k,
                    major_radius,
                    minor_radius,
                    ..
                } => {
                    let c = at(*center);
                    let r =
                        major_radius.abs() * dir.cross(*k).norm() + minor_radius.abs() * dir.norm();
                    (c - r, c + r)
                }
                Curve3::Line { .. } | Curve3::Nurbs(_) => points(self.span_points()),
            },
        }
    }

    /// **The plane×cylinder row's lever from the rulings' hinge**: the
    /// farthest a consumed point stands along the axis from the hinge's
    /// station, `shift` along the axis from `pivot` (the foot the gap is
    /// read at). A [`Self::Face`] reads it exactly from its two sides,
    /// `max(|above − shift|, |below + shift|)`; every other reach adds
    /// `|shift|` to its lever from the foot.
    #[must_use]
    pub fn hinge_lever(&self, pivot: Point3<T>, shift: T) -> T {
        match self {
            Self::Face { below, above, .. } => (*above - shift).abs().max((*below + shift).abs()),
            Self::Ball(_) | Self::Measured { .. } | Self::Span { .. } => {
                self.lever_from(pivot) + shift.abs()
            }
        }
    }

    /// **The plane×cylinder row's lever for the tilt's turn across the
    /// wall**, zero for every reach but a [`Self::Face`]: a turn of sine
    /// `c` and cosine `cos` about the hinge through `hinge` moves a point
    /// standing `x` across the wall from it by `(1 − cos)·x`. The face
    /// bounds `x` by `across` plus `at`'s own distance across the wall
    /// from the hinge: `|(at − hinge)⊥·m|/|m|`, `(at − hinge)⊥` and `m`
    /// the offset's and the normal's parts off the axis, widened by the rounding in `m`'s direction, and never
    /// past `at − hinge` off the axis, which it reaches where `m` is too
    /// short to name a direction (the normal on the axis to rounding,
    /// where `cos` read from `c` is not). Returned as a lever,
    /// `(1 − cos)/|c| = |c|/(1 + cos)`, so nothing divides by `c`.
    #[must_use]
    pub fn turn_lever(&self, hinge: Point3<T>, (n, a): (Vec3<T>, Vec3<T>), c: T, cos: T) -> T {
        match self {
            Self::Face { at, across, .. } => {
                let off = *at - hinge;
                let off_perp = off - a * off.dot(a);
                let off_axis = off_perp.norm();
                let m = n - a * n.dot(a);
                let len = m.norm().max(T::from_f64(f64::MIN_POSITIVE));
                let along_normal =
                    (off_perp.dot(m).abs() + off_axis * T::from_f64(8.0 * f64::EPSILON)) / len;
                (*across + off_axis.min(along_normal)) * c.abs() / (T::one() + cos)
            }
            Self::Ball(_) | Self::Measured { .. } | Self::Span { .. } => T::zero(),
        }
    }

    /// The pivot on the line `origin + s·axis` (`axis` unit) the reach
    /// is read at.
    ///
    /// - [`Self::Ball`]: the foot of its centre; [`Self::Measured`]: the
    ///   foot of `at`.
    /// - [`Self::Span`] of a conic or a spiric: the foot of its centre,
    ///   which is where its lever (centre distance plus a constant) is
    ///   least.
    /// - [`Self::Span`] of a line or a NURBS carrier, whose lever is the
    ///   farthest of finitely many points `pᵢ` (endpoints, control
    ///   points): the axis point `s*` where that farthest distance is
    ///   least (`minimax_on_axis`). Its lever is therefore no longer
    ///   than from any other point of the axis, the cylinder's stored
    ///   origin among them.
    #[must_use]
    pub fn foot_on(&self, origin: Point3<T>, axis: Vec3<T>) -> Point3<T> {
        match self {
            Self::Span {
                carrier: Curve3::Line { .. } | Curve3::Nurbs(_),
                ..
            } => minimax_on_axis(&self.span_points(), origin, axis),
            Self::Span {
                carrier:
                    Curve3::Circle { center, .. }
                    | Curve3::Ellipse { center, .. }
                    | Curve3::Spiric { center, .. },
                ..
            } => foot(*center, origin, axis),
            Self::Ball(ball) => foot(ball.center(), origin, axis),
            Self::Measured { at, .. } | Self::Face { at, .. } => foot(*at, origin, axis),
        }
    }

    /// A point the reach stands for: a ball's centre, the point a length
    /// was measured from, a span's first consumed point or conic centre.
    /// Its distance from a pivot is at most a ball's or a span's lever
    /// from there, so the floor in [`Self::lever_between`] binds only on
    /// a caller's length ([`Self::Measured`]; no caller hands the cylinder
    /// pair a [`Self::Face`]).
    fn reading_point(&self) -> Point3<T> {
        match self {
            Self::Ball(ball) => ball.center(),
            Self::Measured { at, .. } | Self::Face { at, .. } => *at,
            Self::Span {
                carrier:
                    Curve3::Circle { center, .. }
                    | Curve3::Ellipse { center, .. }
                    | Curve3::Spiric { center, .. },
                ..
            } => *center,
            Self::Span { .. } => self.span_points()[0],
        }
    }

    /// The points a line or NURBS span's lever is the farthest of: the
    /// segment's endpoints, or the control points. Empty for the other
    /// variants, which this is not called on.
    fn span_points(&self) -> Vec<Point3<T>> {
        match self {
            Self::Span {
                carrier: Curve3::Line { origin, dir },
                t0,
                t1,
            } => vec![*origin + *dir * *t0, *origin + *dir * *t1],
            Self::Span {
                carrier: Curve3::Nurbs(n),
                ..
            } => n.control().to_vec(),
            _ => Vec::new(),
        }
    }

    /// The lever of a relative tilt between two lines `oᵢ + s·aᵢ`
    /// (`aᵢ` unit) whose gap is read between their feet
    /// ([`Self::foot_on`]): the lesser of the two feet's levers. Holding
    /// either line and turning the other about its own foot bounds the
    /// same displacement across the reach, so the lesser bound holds as
    /// well, and it does not depend on which line is named first.
    ///
    /// **Why a foot's lever, and not a ball's radius alone.** Axial
    /// travel along parallel lines does not move their gap, which
    /// suggests the radius would do. But the gap is read between the
    /// two FEET, and while the lines are parallel only within the band
    /// the feet stand apart along the axis as well: the foot-to-foot
    /// distance carries an axial part of up to `min(ρ)·θ`, `ρ` a foot's
    /// lever. A lever shorter than a foot's would let that part and the
    /// tilt together exceed what the gap row was told it bridges.
    ///
    /// **The lever here is floored at the foot's distance from the reach's
    /// point** (a ball's centre, the point a length was measured from, a
    /// span's consumed point); that binds only on a [`Self::Measured`]
    /// length, and only here. The caller measured its length along the
    /// axis from `at`; the foot-to-foot gap also carries the radial
    /// offset of `at` from each axis, which a tilt turns into axial
    /// travel between the feet, so this lever reaches at least that far.
    /// The plane×cylinder row moves its section by the tilt times the
    /// AXIAL distance from the foot, which a [`Self::Face`]'s length
    /// already measures, so it takes that length bare: floored there, a
    /// face shorter than the radius would be levered past its own
    /// measure, and a tilt the length leaves in the band served.
    #[must_use]
    pub fn lever_between(&self, line1: (Point3<T>, Vec3<T>), line2: (Point3<T>, Vec3<T>)) -> T {
        self.foot_lever(line1).1.min(self.foot_lever(line2).1)
    }

    /// One line's half of [`Self::lever_between`]: the foot on the line
    /// `origin + s·axis` ([`Self::foot_on`]) and its lever, floored at the
    /// foot's distance from the reach's point.
    #[must_use]
    pub fn foot_lever(&self, (origin, axis): (Point3<T>, Vec3<T>)) -> (Point3<T>, T) {
        let pivot = self.foot_on(origin, axis);
        let lever = self
            .lever_from(pivot)
            .max((self.reading_point() - pivot).norm());
        (pivot, lever)
    }
}

/// **The axis point whose farthest distance to `points` is least**: the
/// `s*` minimising `maxᵢ √(ρᵢ² + (s − sᵢ)²)` on the line
/// `origin + s·axis` (`axis` unit), `sᵢ` and `ρᵢ` each point's axial
/// coordinate and distance from the line. Exact, and comparison-free
/// (lattice `max`/`min` and [`Real::select_le_zero`]).
///
/// The sublevel sets `{s : ρᵢ² + (s − sᵢ)² ≤ R}` are intervals, so by
/// 1-D Helly they share a point iff every PAIR does. The least common
/// `R` is therefore the largest pairwise one, `R* = max_{i,j} Rᵢⱼ`, and
/// at `R*` the intersection is the single point
/// `s* = maxᵢ (sᵢ − √(R* − ρᵢ²))`. A pair `i, j` at axial distance `d`
/// with `Δ = |ρᵢ² − ρⱼ²|` and smaller `ρ²` of `q` has
/// `Rᵢⱼ = max(ρᵢ², ρⱼ²)` when `d² ≤ Δ` (one interval holds the other's
/// centre), else `q + ((d² + Δ)/(2d))²` (they meet where the two
/// distances are equal, between the two points).
fn minimax_on_axis<T: Real>(points: &[Point3<T>], origin: Point3<T>, axis: Vec3<T>) -> Point3<T> {
    let sq: Vec<(T, T)> = points
        .iter()
        .map(|&p| {
            let w = p - origin;
            let s = w.dot(axis);
            (s, (w - axis * s).dot(w - axis * s))
        })
        .collect();
    let two = T::from_f64(2.0);
    let mut r_star = T::zero();
    for (i, &(si, qi)) in sq.iter().enumerate() {
        r_star = r_star.max(qi);
        for &(sj, qj) in &sq[i + 1..] {
            let d = (si - sj).abs();
            let delta = (qi - qj).abs();
            // `a = (d² + Δ)/(2d)` is at most `d` wherever the crossing is
            // the one taken (`d² > Δ`), so the crossing is clamped at
            // `q + d²` and the divisor floored: neither moves a definite
            // reading, and an enclosure whose `d` holds zero (coincident
            // or near-coincident points) stays bounded instead of
            // dividing by zero.
            let a = (d.powi(2) + delta) / (two * d).max(T::from_f64(f64::MIN_POSITIVE));
            let q = qi.min(qj);
            let crossing = (q + a.powi(2)).min(q + d.powi(2));
            r_star = r_star.max((d.powi(2) - delta).select_le_zero(qi.max(qj), crossing));
        }
    }
    let lower = |&(s, q): &(T, T)| s - (r_star - q).max(T::zero()).sqrt();
    let s_star = sq[1..].iter().fold(lower(&sq[0]), |m, p| m.max(lower(p)));
    origin + axis * s_star
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **A conic span's lever reaches every point of it** from any pivot:
    /// centre distance plus the radius (or the larger semi-axis), round
    /// the whole turn.
    #[test]
    fn a_conic_spans_lever_reaches_every_point() {
        let pivot = Point3::new(0.3, -0.2, 0.7);
        for carrier in [
            Curve3::Circle {
                center: Point3::new(1.0, 2.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: 1.5,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            Curve3::Ellipse {
                center: Point3::new(-1.0, 0.5, 0.2),
                axis: Vec3::new(0.0, 0.0, 1.0),
                major: 0.6,
                minor: 1.4,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
        ] {
            let lever = Reach::Span {
                carrier: carrier.clone(),
                t0: 0.0,
                t1: 1.0,
            }
            .lever_from(pivot);
            for k in 0..=720 {
                let t = core::f64::consts::TAU * f64::from(k) / 720.0;
                let far = (carrier.eval(t) - pivot).norm();
                assert!(
                    lever >= far,
                    "{carrier:?}: {lever} falls short of {far} at {t}"
                );
            }
        }
    }

    /// The farthest `carrier` stands from `pivot` over `[t0, t1]`: 4,000
    /// samples, each local maximum refined by golden section.
    fn farthest_over(carrier: &Curve3<f64>, (t0, t1): (f64, f64), pivot: Point3<f64>) -> f64 {
        let d = |t: f64| (carrier.eval(t) - pivot).norm();
        let n = 4_000;
        let ts: Vec<f64> = (0..=n)
            .map(|k| t0 + (t1 - t0) * f64::from(k) / f64::from(n))
            .collect();
        let ds: Vec<f64> = ts.iter().map(|&t| d(t)).collect();
        let mut best = ds.iter().copied().fold(0.0_f64, f64::max);
        for k in 0..ds.len() {
            let (below, above) = (k.saturating_sub(1), (k + 1).min(ds.len() - 1));
            if ds[k] >= ds[below] && ds[k] >= ds[above] {
                let (mut lo, mut hi) = (ts[below], ts[above]);
                for _ in 0..80 {
                    let (m1, m2) = (lo + (hi - lo) * 0.381_966, lo + (hi - lo) * 0.618_034);
                    if d(m1) < d(m2) {
                        lo = m1;
                    } else {
                        hi = m2;
                    }
                }
                best = best.max(d(0.5 * (lo + hi)));
            }
        }
        best
    }

    /// **A conic arc's reach from a point is its farthest point over the
    /// span, within a quarter's bulge.** Circles and an ellipse of scale
    /// `r ∈ {1e-3, 1, 1e3}` (offset `1e3` from the origin), arcs of span
    /// 0.01 to 0.5 rad and stored backwards, a whole turn and a turn
    /// and a half, each read from its own end (where the far end binds),
    /// from the far side of its centre opposite a point inside a quarter
    /// (near, and a million radii out against the conic's normal there,
    /// where the crest of the bulge binds and a quarter's chord moves the
    /// reach by less than the bulge does), from inside its bulge (an end binds), and from off
    /// its plane. Against the farthest point (sampled, each local maximum
    /// refined) the reach is never short (to the coordinates' rounding);
    /// a circle's is never past it either, an ellipse's never by more than
    /// a quarter's bulge `(1 − cos(span/8))·max(|a|, |b|)`, and neither
    /// past the whole turn's [`Reach::lever_from`]. A circle's crest
    /// dropped falls short; its crest read on the near side, or its span
    /// test inverted, passes the farthest point. An ellipse's bulge
    /// dropped, set on the chord's inner side, or aimed from a quarter's
    /// start rather than its middle falls short at the crest; the span
    /// read the other way
    /// round reaches past the slack.
    #[test]
    fn a_conic_arcs_reach_is_its_farthest_point_over_the_span() {
        let off = Point3::new(1e3, -1e3, 1e3);
        let tilted = Vec3::new(0.6, 0.0, 0.8);
        for r in [1e-3, 1.0, 1e3] {
            let carriers = [
                Curve3::Circle {
                    center: off,
                    axis: tilted,
                    radius: r,
                    u_ref: Vec3::new(0.0, 1.0, 0.0),
                },
                Curve3::Ellipse {
                    center: off,
                    axis: tilted,
                    major: 2.0 * r,
                    minor: 0.7 * r,
                    u_ref: Vec3::new(-0.8, 0.0, 0.6),
                },
            ];
            for carrier in carriers {
                let (a, b) = match carrier {
                    Curve3::Circle { radius, .. } => (radius, radius),
                    Curve3::Ellipse { major, minor, .. } => (major, minor),
                    _ => unreachable!(),
                };
                let big = a.abs().max(b.abs());
                for (t0, t1) in [
                    (0.3, 0.31),
                    (1.0, 1.05),
                    (-0.2, -0.1),
                    (2.0, 2.5),
                    (2.5, 2.0),
                    (0.4, 0.4 + core::f64::consts::TAU),
                    (0.4, 0.4 + 1.5 * core::f64::consts::TAU),
                ] {
                    let start = carrier.eval(t0);
                    let tm = t0 + 0.375 * (t1 - t0);
                    let outward = carrier.eval(tm) - off;
                    // The conic's outward normal at `tm`: a pivot far
                    // against it has its farthest point there.
                    let crest_normal = {
                        let (u, k) = match carrier {
                            Curve3::Circle { u_ref, axis, .. }
                            | Curve3::Ellipse { u_ref, axis, .. } => (u_ref, axis.normalize()),
                            _ => unreachable!(),
                        };
                        (u * (tm.cos() / a) + k.cross(u) * (tm.sin() / b)).normalize()
                    };
                    let normal = tilted.normalize();
                    for pivot in [
                        start,
                        off - outward * 3.0,
                        off - crest_normal * (1e6 * big),
                        off + outward * 0.999,
                        start + normal * (2.0 * r),
                    ] {
                        let reach = Reach::Span {
                            carrier: carrier.clone(),
                            t0,
                            t1,
                        };
                        let got = reach.span_reach_from(pivot);
                        let far = farthest_over(&carrier, (t0, t1), pivot);
                        let ulps = 1e-15
                            * ((off - Point3::origin()).norm()
                                + (pivot - Point3::origin()).norm()
                                + big);
                        let span = (t1 - t0).abs().min(core::f64::consts::TAU);
                        let slack = match carrier {
                            Curve3::Circle { .. } => 0.0,
                            _ => (1.0 - (span / 8.0).cos()) * big,
                        };
                        let label = format!("{carrier:?} over [{t0}, {t1}] from {pivot:?}");
                        assert!(
                            got >= far - ulps,
                            "{label}: {got} falls short of the sampled {far}"
                        );
                        assert!(
                            got <= far + slack + ulps,
                            "{label}: {got} passes the sampled {far} by more than {slack}"
                        );
                        assert!(
                            got <= reach.lever_from(pivot),
                            "{label}: {got} passes the whole turn"
                        );
                    }
                }
            }
        }
    }

    /// The farthest `|(x − pivot)·axis|` over `carrier` sampled on
    /// `[t0, t1]`.
    fn sampled_along(
        carrier: &Curve3<f64>,
        (t0, t1): (f64, f64),
        pivot: Point3<f64>,
        axis: Vec3<f64>,
    ) -> f64 {
        (0..=20_000)
            .map(|k| {
                let t = t0 + (t1 - t0) * f64::from(k) / 20_000.0;
                (carrier.eval(t) - pivot).dot(axis).abs()
            })
            .fold(0.0_f64, f64::max)
    }

    /// The farthest `reach` stands from `pivot` along `axis`, either way.
    fn axial(reach: &Reach<f64>, pivot: Point3<f64>, axis: Vec3<f64>) -> f64 {
        let (lo, hi) = reach.range_along(pivot, axis);
        hi.max(-lo)
    }

    /// **A conic arc's axial lever is the farthest it reaches along the
    /// axis over its span**, to sampling: never short of a sampled
    /// point, and within sampling of the farthest, on arcs whose crests
    /// lie inside, outside and astride the span, a whole turn, a span
    /// past one, and a span stored backwards, read from a pivot above
    /// the conics (the low crest binds) and one far below them (the high
    /// crest binds), with a short arc about each crest. A whole-turn
    /// support over-reaches every arc that misses a crest; a lever that
    /// dropped either crest falls short of the arc about it.
    #[test]
    fn a_conic_arcs_axial_lever_is_its_reach_over_the_span() {
        let axis = Vec3::new(0.2, -0.3, 1.0).normalize();
        let tilted = Vec3::new(0.6, 0.0, 0.8);
        let carriers = [
            Curve3::Circle {
                center: Point3::new(1.0, 2.0, 0.0),
                axis: tilted,
                radius: 1.5,
                u_ref: Vec3::new(0.0, 1.0, 0.0),
            },
            Curve3::Ellipse {
                center: Point3::new(-1.0, 0.5, 0.2),
                axis: tilted,
                major: 2.0,
                minor: 0.7,
                u_ref: Vec3::new(-0.8, 0.0, 0.6),
            },
            Curve3::Ellipse {
                center: Point3::new(-1.0, 0.5, 0.2),
                axis: tilted,
                major: 0.7,
                minor: 2.0,
                u_ref: Vec3::new(-0.8, 0.0, 0.6),
            },
        ];
        let tau = core::f64::consts::TAU;
        let spans = [
            (0.0, tau),
            (0.0, 1.0),
            (1.0, 2.5),
            (2.0, 4.0),
            (3.5, 6.0),
            (-1.0, 0.4),
            (5.0, 5.0 + tau + 0.5),
            (4.0, 1.0),
        ];
        let mut short = 0;
        let mut crests = [0, 0];
        for (side, pivot) in [Point3::new(0.3, -0.2, 0.7), Point3::new(0.0, 0.0, -5.0)]
            .into_iter()
            .enumerate()
        {
            for carrier in &carriers {
                // The arcs about each crest of the sinusoid along the axis.
                let along = |t: f64| (carrier.eval(t) - pivot).dot(axis);
                let grid = (0..7200).map(|k| tau * f64::from(k) / 7200.0);
                let high = grid
                    .clone()
                    .fold(0.0, |b, t| if along(t) > along(b) { t } else { b });
                let low = grid.fold(0.0, |b, t| if along(t) < along(b) { t } else { b });
                let about = [(high - 0.3, high + 0.3), (low - 0.3, low + 0.3)];
                for span in spans.into_iter().chain(about) {
                    let lever = axial(
                        &Reach::Span {
                            carrier: carrier.clone(),
                            t0: span.0,
                            t1: span.1,
                        },
                        pivot,
                        axis,
                    );
                    let far = sampled_along(carrier, span, pivot, axis);
                    assert!(
                        lever >= far * (1.0 - 1e-12) && lever - far < 1e-6,
                        "{carrier:?} over {span:?} from {pivot:?}: lever {lever} against the \
                         farthest sampled {far}"
                    );
                    let whole = sampled_along(carrier, (0.0, tau), pivot, axis);
                    short += usize::from(far < whole - 1e-3);
                }
                // Which crest binds the arc about the high one, past its ends.
                let (t0, t1) = about[0];
                let ends = along(t0).abs().max(along(t1).abs());
                if along(high).abs() > ends + 1e-3 {
                    crests[side] += 1;
                }
            }
        }
        assert!(
            crests[1] == 3,
            "the high crest binds every arc about it from below: {crests:?}"
        );
        assert!(short >= 6, "the arcs that miss a crest: {short}");
    }

    /// **A spiric's and a spline's axial levers are their bounds' own
    /// support, and reach the carrier.** A spiric oval read along its
    /// torus's axis reaches the torus's support `r` at `v = π/2`, so its
    /// lever is the farthest sampled point exactly; a lever short of
    /// the minor radius, or one round the torus (`R + r`), misses it. A
    /// clamped cubic read along `z` peaks at its last control point,
    /// which it interpolates; its lever is that, where the control
    /// points' Euclidean reach over-reaches.
    #[test]
    fn a_spiric_and_a_splines_axial_levers_reach_the_carrier() {
        let z = Vec3::new(0.0, 0.0, 1.0);
        let pivot = Point3::new(0.4, -0.3, 0.25);
        let spiric = Curve3::Spiric {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: z,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            major_radius: 2.0,
            minor_radius: 0.5,
            offset: 0.7,
        };
        let tau = core::f64::consts::TAU;
        let lever = axial(
            &Reach::Span {
                carrier: spiric.clone(),
                t0: 0.0,
                t1: tau,
            },
            pivot,
            z,
        );
        let far = sampled_along(&spiric, (0.0, tau), pivot, z);
        assert!(
            lever >= far && lever - far < 1e-6,
            "spiric: lever {lever} against the farthest sampled {far}"
        );
        use geom_core::KnotVector;
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(2.0, 1.0, 0.5),
            Point3::new(1.0, -2.0, 1.0),
            Point3::new(0.5, 0.5, 2.0),
        ];
        let spline = Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(knots, control, vec![1.0; 4]).unwrap(),
        ));
        let lever = axial(
            &Reach::Span {
                carrier: spline.clone(),
                t0: 0.0,
                t1: 1.0,
            },
            pivot,
            z,
        );
        let far = sampled_along(&spline, (0.0, 1.0), pivot, z);
        assert!(
            lever >= far && lever - far < 1e-12,
            "spline: lever {lever} against the farthest sampled {far}"
        );
    }

    /// **A face's turn lever covers its reading point's own distance
    /// across the wall from the hinge.** A face read at `(1, 0, 0)` that
    /// reaches nothing from there, against a hinge through the origin on
    /// the plane of normal `(cos β, 0, sin β)`: the reading point stands 1
    /// across the wall from the hinge, which the turn moves by
    /// `(1 − cos β)`, so the lever is `(1 − cos β)/sin β`, at every tilt up
    /// to a right angle. At an exact right angle, whose normal names no
    /// direction across the wall (`n − c·a` is zero), it falls back to the
    /// point's distance off the axis rather than reading that zero, as it
    /// does where the normal lies on a tilted axis to rounding and `cos`
    /// read from `c` is ~1e-8. Every reach but a face reads no turn.
    #[test]
    fn a_faces_turn_lever_covers_its_reading_points_offset_from_the_hinge() {
        let at = Point3::new(1.0, 0.0, 0.0);
        let face = Reach::Face {
            at,
            below: 0.0,
            above: 0.0,
            across: 0.0,
        };
        let a = Vec3::new(0.0, 0.0, 1.0);
        let tilts = [1e-4_f64, 0.3, 1.0, core::f64::consts::FRAC_PI_2].map(f64::sin_cos);
        for (c, cos) in tilts.into_iter().chain([(1.0, 0.0)]) {
            let beta = c.atan2(cos);
            let n = Vec3::new(cos, 0.0, c);
            let lever = face.turn_lever(Point3::new(0.0, 0.0, 0.0), (n, a), c, cos);
            let want = c / (1.0 + cos);
            assert!(
                (lever - want).abs() <= 1e-12 * want.max(1e-12),
                "β = {beta}: lever {lever}, the point's turn {want}"
            );
        }
        // The normal on a tilted axis, `c = n·a` a rounding under 1, so
        // `cos` read from `c` is ~1e-8 while `n − c·a` names no direction
        // across the wall: the lever reaches the point's distance off the
        // axis.
        let tilted = Vec3::new(0.3, 0.4, 0.5).normalize();
        let c = tilted.dot(tilted);
        let cos = (1.0 - c * c).max(0.0).sqrt();
        assert!(cos > 0.0, "the witness's `c` rounds under 1");
        let off = tilted.cross(Vec3::new(1.0, 0.0, 0.0)).normalize();
        let leaning = Reach::Face {
            at: Point3::origin() + off,
            below: 0.0,
            above: 0.0,
            across: 0.0,
        };
        let lever = leaning.turn_lever(Point3::origin(), (tilted, tilted), c, cos);
        let want = c / (1.0 + cos);
        assert!(
            lever >= want * (1.0 - 1e-12),
            "a normal on the axis to rounding: lever {lever}, the point's turn {want}"
        );
        let measured = Reach::Measured { at, lever: 1.0 };
        let n = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(
            measured.turn_lever(Point3::new(0.0, 0.0, 0.0), (n, a), 1.0, 0.0),
            0.0
        );
    }

    /// **The minimax stays bounded at the certified scalar** where two
    /// points coincide or nearly do along the axis: duplicate control
    /// points, a zero-length segment, and two points an enclosure's width
    /// apart. Dividing by an axial gap that holds zero widened the pivot
    /// to an infinite enclosure and the lever with it.
    #[test]
    fn the_minimax_stays_bounded_at_the_certified_scalar() {
        use geom_core::{Bounds, Interval};
        let iv = Interval::from_f64;
        // Each coordinate an enclosure a few ulp wide, as a computed one is.
        let w = |v: f64| Interval::from_bounds(v - 4.0 * f64::EPSILON, v + 4.0 * f64::EPSILON);
        let p = |x: f64, z: f64| Point3::new(w(x), iv(0.0), w(z));
        let axis = Vec3::new(iv(0.0), iv(0.0), iv(1.0));
        let origin = Point3::new(iv(0.0), iv(0.0), iv(0.0));
        let near = 0.3 + f64::EPSILON;
        for (label, pts) in [
            ("duplicates", vec![p(0.5, 0.3), p(0.5, 0.3)]),
            ("zero-length", vec![p(1.0, 0.3), p(1.0, 0.3)]),
            ("near", vec![p(0.5, 0.3), p(0.7, near)]),
            (
                "net",
                vec![p(1.0, -1.0), p(0.5, 0.3), p(0.5, 0.3), p(1.0, 1.0)],
            ),
        ] {
            let foot = minimax_on_axis(&pts, origin, axis);
            let lever = pts.iter().fold(iv(0.0), |m, q| m.max((*q - foot).norm()));
            for (what, v) in [("foot", foot.z), ("lever", lever)] {
                assert!(
                    v.lo().is_finite() && v.hi().is_finite() && v.hi() - v.lo() < 1e-3,
                    "{label}: the {what} is a bounded enclosure, got [{}, {}]",
                    v.lo(),
                    v.hi()
                );
            }
        }
    }

    /// **The span's pivot is the axis point whose lever is least**, to
    /// rounding: brute force over a fine axial grid never finds a pivot
    /// with a shorter lever, on uneven control nets and on segments.
    #[test]
    fn a_spans_pivot_minimises_its_lever() {
        let axis = Vec3::new(0.0, 0.0, 1.0);
        let origin = Point3::new(0.0, 0.0, 3.0);
        let nets: [&[(f64, f64, f64)]; 5] = [
            &[(1.0, 0.0, -1.0), (1.0, 0.0, 0.8), (1.0, 0.0, 1.0)],
            &[
                (1.0, 0.0, -1.0),
                (1.0, 0.0, 0.9),
                (1.0, 0.0, 0.95),
                (1.0, 0.0, 1.0),
            ],
            &[(10.0, 0.0, 0.0), (0.0, 0.0, 1.0)],
            &[
                (0.5, 0.2, -3.0),
                (2.0, -1.0, 0.0),
                (0.1, 0.0, 0.1),
                (3.0, 3.0, 4.0),
            ],
            &[(1.0, 0.0, 2.0), (1.0, 0.0, 2.0)],
        ];
        for net in nets {
            let pts: Vec<Point3<f64>> = net.iter().map(|&(x, y, z)| Point3::new(x, y, z)).collect();
            let lever = |p: Point3<f64>| pts.iter().fold(0.0_f64, |m, q| m.max((*q - p).norm()));
            let best = lever(minimax_on_axis(&pts, origin, axis));
            for k in -4000..=4000 {
                let z = f64::from(k) * 0.002;
                let other = lever(Point3::new(0.0, 0.0, z));
                assert!(
                    best <= other * (1.0 + 1e-12),
                    "{net:?}: z = {z} levers {other} < {best}"
                );
            }
        }
    }

    fn xyz(p: Point3<f64>) -> [f64; 3] {
        [p.x, p.y, p.z]
    }

    /// The torus's lever from its own centre is `R + r`: the farthest
    /// any point of the ring stands from the pivot its axis turns on.
    #[test]
    fn a_torus_levers_at_its_outer_radius() {
        let torus = Surface::Torus {
            center: Point3::new(1.0, 2.0, 3.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let ball = ExtentBall::of_carrier(&torus).unwrap();
        assert_eq!(ball.lever_from(Point3::new(1.0, 2.0, 3.0)), 2.5);
        assert_eq!(ball.lever_from(Point3::new(1.0, 2.0, 7.0)), 6.5);
    }

    /// The enclosing ball sits at the parts' mean and reaches each
    /// one's far side: a square's corners give its circumscribed ball,
    /// and a circle beside them widens it to the circle's far side.
    #[test]
    fn an_enclosing_ball_reaches_every_parts_far_side() {
        let corners = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]
            .map(|(x, y)| ExtentBall::point(Point3::new(x, y, 0.0)));
        let square = ExtentBall::enclosing(&corners).unwrap();
        assert_eq!(xyz(square.center()), [1.0, 1.0, 0.0]);
        assert_eq!(square.lever_from(square.center()), 2.0f64.sqrt());
        let mut parts = corners.to_vec();
        parts.push(ExtentBall::new(Point3::new(6.0, 1.0, 0.0), 1.0));
        let both = ExtentBall::enclosing(&parts).unwrap();
        assert_eq!(xyz(both.center()), [2.0, 1.0, 0.0]);
        assert_eq!(both.lever_from(both.center()), 5.0);
        assert!(ExtentBall::<f64>::enclosing(&[]).is_none());
        let slab = ExtentBall::of_box(Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 2.0, 0.0));
        assert_eq!(xyz(slab.center()), xyz(square.center()));
        assert_eq!(slab.lever_from(slab.center()), 2.0f64.sqrt());
    }
    /// A lopsided pair: a point beside a far larger ball, inside it. The
    /// enclosure sits at the mean of the centres, so it is looser than
    /// the minimal ball (the large ball itself) by the distance from the
    /// mean to that ball's centre — and still reaches the large ball's
    /// far side, which is the direction that matters.
    #[test]
    fn a_lopsided_enclosure_reaches_the_large_parts_far_side() {
        let point = ExtentBall::point(Point3::new(0.0, 0.0, 0.0));
        let large = ExtentBall::new(Point3::new(10.0, 0.0, 0.0), 100.0);
        let both = ExtentBall::enclosing(&[point, large]).unwrap();
        assert_eq!(xyz(both.center()), [5.0, 0.0, 0.0]);
        assert_eq!(both.radius(), 105.0, "the mean's lever to the far side");
        for far in [
            Point3::new(110.0, 0.0, 0.0),
            Point3::new(-90.0, 0.0, 0.0),
            Point3::new(10.0, 100.0, 0.0),
        ] {
            assert!(
                (far - both.center()).norm() <= both.radius(),
                "{far:?} of the large ball is enclosed"
            );
        }
    }

    /// **Rounding.** Parts far from the origin, at coordinates whose
    /// ulp is coarse next to their radii: the mean rounds, and each
    /// lever rounds. Sample points on every part's sphere stand within
    /// a few ulps of the radius — a rounding the band dwarfs, where a
    /// centre or radius that dropped a part's far side would miss by
    /// that part's whole radius.
    #[test]
    fn an_enclosure_far_from_the_origin_rounds_within_ulps() {
        let base = 1.0e8;
        let parts = [
            ExtentBall::new(Point3::new(base + 0.3, base - 0.7, 1.0), 0.1),
            ExtentBall::new(Point3::new(base + 3.1, base + 0.2, -2.0), 1.7),
            ExtentBall::point(Point3::new(base - 1.9, base + 2.3, 0.5)),
        ];
        let ball = ExtentBall::enclosing(&parts).unwrap();
        let slack = 4.0 * f64::EPSILON * (base + ball.radius());
        let dirs = [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
        ];
        for part in parts {
            for d in dirs {
                let p = part.center() + d * part.radius();
                let over = (p - ball.center()).norm() - ball.radius();
                assert!(
                    over <= slack,
                    "{p:?} stands {over:e} past the enclosure (slack {slack:e})"
                );
            }
        }
    }
}
