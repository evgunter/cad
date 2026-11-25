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
//! (`decide_across` in `geom_core::k_stats`): `pc_parallel_gap`,
//! `cc_coaxial`, `cc_parallel_gap`, `tangent_locus_internal_gap`,
//! `coc_coaxial`, `pt_spiric_two_ovals` (`geom::spiric_regime`), `pt_cap_gap` and
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
//! No ball chosen around the consumed region is a lever. An edge is
//! held as its region, its cover over its span ([`Reach::Region`]), and
//! levered by the cover's far distance ([`Reach::lever_from`]) from the
//! middle of its window along the axis read ([`Reach::foot_on`]), a
//! pivot derived from the region and never chosen per kind; a face is
//! levered at its own cover (`topo`'s `face_cover`), which holds what
//! bulges past its boundary. A germ frame handed without its faces
//! ([`Reach::of_frame`]) is levered at its wall's radius, which a pair
//! of lines floors ([`Reach::lever_between`]).
//! A plane×cylinder face's caller ([`Reach::Face`]) hands what it
//! measured of the face from `at`: how far it reaches either way along
//! the axis ([`Reach::range_along`] of each boundary span) and how far
//! it reaches at all. The plane×cylinder row reads them at the RULINGS'
//! HINGE, the point a face lever is read from: the axial reach from the
//! hinge's station (`hinge_lever`), and the reach across the
//! wall from the hinge, `at`'s own offset from it included
//! (`turn_lever`). The classifier adds no length of its own.
//! A plane×cone caller hands the same measures against the cone's axis,
//! read at the apex: the face's farthest distance from `at` is its
//! lever, and its window along the axis ([`Reach::range_along`]) bounds
//! how near the apex, and on which nappe, a consumed point stands.
//!
//! The tangent-locus witness reads a ball, [`Reach::Ball`]: the one its
//! callers, the carrier doors, hand it.
//!
//! Everything is comparison-free: `max` and `min` are the [`Real`]
//! lattice operations.

use geom::cover::{BoxAxes, Cover, ParamWindow, Site, Uncovered};
use geom::{Conic, Curve3, Line};
use geom_core::UnitVec3;
use geom_core::spline::SpanLocate;
use geom_core::{Lever, Point3, Real, Vec3, is_finite_length, poison};

/// **The farthest of the consumed `points` from `at`**: the largest
/// `|p − at|`, read as the root of the largest square (`norm_squared`
/// squares each coordinate, which an enclosure never reads below zero).
/// Zero for no points.
pub fn points_lever_from<T: Real>(
    at: Point3<T>,
    points: impl IntoIterator<Item = Point3<T>>,
) -> Lever<T> {
    let far = points
        .into_iter()
        .fold(T::zero(), |far, p| far.max((p - at).norm_squared()));
    Lever::of_consumed_extent(far.sqrt())
}

/// **The farthest two of the consumed `points` stand apart**: their
/// diameter ([`farthest_pair`]), zero for fewer than two.
pub fn points_diameter<T: Real>(points: &[Point3<T>]) -> Lever<T> {
    Lever::of_consumed_extent(farthest_pair(points))
}

/// **The farthest two points of the region `cover` holds stand apart,
/// from above**: the diagonal of the cover's box along the world axes
/// ([`Cover::box_in`]), which holds the region. The lever of a tilt
/// between two directions over the whole region, from any pivot in it.
#[must_use]
pub fn cover_diameter<T: Real>(cover: &Cover<T>) -> Lever<T> {
    let (lo, hi) = cover.box_in(&BoxAxes::World);
    Lever::of_consumed_extent((hi - lo).norm())
}

/// **What a stored datum's kind forgets**: the orbit its symmetry lets
/// the datum slide along without moving the carrier ([`anchored_at`]).
#[derive(Clone, Copy, Debug)]
pub enum Forgets<T: Real> {
    /// A point of a plane, anywhere in the plane of this normal.
    Plane(Vec3<T>),
    /// A point of an axis, anywhere along this direction.
    Axis(UnitVec3<T>),
}

/// **A datum re-anchored within an extent**: `datum` slid along the
/// orbit its kind forgets ([`Forgets`]) to the point of it nearest `at`
/// — a plane's origin to `at`'s foot on the plane, an axis point to
/// `at`'s foot on the axis. The carrier is unmoved; what moves is where
/// a reading that consumes the datum takes it from, so a reading over a
/// face or a verdict over an extent levers nothing past that extent
/// (`[tolerance.epsilon]`). The one re-anchoring: the coincidence
/// verdict passes the extent's centre, a face's measure a point of the
/// face ([`surface_anchored_at`]).
#[must_use]
pub fn anchored_at<T: Real>(datum: Point3<T>, forgets: Forgets<T>, at: Point3<T>) -> Point3<T> {
    match forgets {
        Forgets::Plane(normal) => at - normal * ((at - datum).dot(normal) / normal.norm_squared()),
        Forgets::Axis(dir) => Line { origin: datum, dir }.foot(at),
    }
}

/// **`surface` with each datum its kind forgets re-anchored at `at`**
/// ([`anchored_at`]): a plane's origin and a cylinder's axis origin.
/// Every other datum is intrinsic (a sphere's or a torus's centre, a
/// cone's apex) and stays, as does a spline's net.
#[must_use]
pub fn surface_anchored_at<T: Real>(surface: &geom::Surface<T>, at: Point3<T>) -> geom::Surface<T> {
    let mut out = surface.clone();
    match &mut out {
        geom::Surface::Plane { frame, .. } => {
            *frame = frame.with_origin(anchored_at(
                frame.origin(),
                Forgets::Plane(frame.axis()),
                at,
            ));
        }
        geom::Surface::Cylinder { frame, .. } => {
            *frame = frame.with_origin(anchored_at(
                frame.origin(),
                Forgets::Axis(frame.axis_unit()),
                at,
            ));
        }
        _ => {}
    }
    out
}

/// A closed ball `|x − center| ≤ radius` enclosing a consumed region
/// (module docs).
#[derive(Clone, Copy, Debug)]
pub struct ExtentBall<T: Real> {
    center: Point3<T>,
    radius: T,
}

impl<T: Real> ExtentBall<T> {
    /// The ball of `radius` about `center`, `radius` read off the
    /// geometry it encloses by the constructors here.
    #[must_use]
    fn new(center: Point3<T>, radius: T) -> Self {
        Self { center, radius }
    }

    /// **A ball no geometry is named for**: `radius` about `center`, a
    /// length a call site vouches for. Each reference is pinned with its
    /// reason by `scripts/gates/lever-sources.sh`, a list that only
    /// shrinks.
    #[doc(hidden)]
    #[must_use]
    pub fn unsourced(center: Point3<T>, radius: T) -> Self {
        Self::new(center, radius)
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
        let n = T::count(parts.len());
        let sum = parts
            .iter()
            .fold(Vec3::new(T::zero(), T::zero(), T::zero()), |acc, b| {
                acc + (b.center - first.center)
            });
        let center = first.center + sum / n;
        let radius = parts
            .iter()
            .fold(T::zero(), |r, b| r.max(b.reach_from(center)));
        Some(Self::new(center, radius))
    }

    /// The ball's centre.
    #[must_use]
    pub fn center(self) -> Point3<T> {
        self.center
    }

    /// **The ball read through a rigid motion**: its centre mapped, its
    /// radius kept, since a rigid motion keeps every distance it bounds.
    #[must_use]
    pub fn moved(self, motion: &geom_core::RigidMotion<T>) -> Self {
        Self::new(motion.transform_point(self.center), self.radius)
    }

    /// The ball's radius: its lever from its own centre.
    #[must_use]
    pub fn radius(self) -> T {
        self.radius
    }

    /// The ball's radius as a lever: its reach from its own centre.
    #[must_use]
    pub fn lever(self) -> Lever<T> {
        Lever::of_consumed_extent(self.radius)
    }

    /// The farthest the ball reaches from `pivot`: the lever arm at
    /// which an angular datum pinned at `pivot` is metered.
    #[must_use]
    pub fn lever_from(self, pivot: Point3<T>) -> Lever<T> {
        Lever::of_consumed_extent(self.reach_from(pivot))
    }

    fn reach_from(self, pivot: Point3<T>) -> T {
        (pivot - self.center).norm() + self.radius
    }

    /// **The farthest the ball reaches from either of two pivots**: the
    /// larger of its reaches from `p` and from `q`. A reading whose
    /// position datum is the two pivots' separation (two cones' apexes;
    /// a point of one axis and its foot on the other) and whose tilt
    /// turns either carrier about its own pivot levers here: a consumed
    /// point of either carrier stands no farther from that carrier's
    /// pivot.
    ///
    /// The LARGER, where [`Reach::lever_between`] takes the lesser of two
    /// feet's levers: here the reading bounds how far a point of EACH
    /// carrier is displaced, each about its own pivot, so the arm must
    /// reach from both; there one tilt between two lines is bounded
    /// twice over, once about each foot, and either bound alone holds.
    #[must_use]
    pub(crate) fn lever_from_either(self, p: Point3<T>, q: Point3<T>) -> Lever<T> {
        Lever::of_consumed_extent(self.reach_from(p).max(self.reach_from(q)))
    }

    /// **The farthest the ball reaches from `pivot` and from the plane
    /// through `p0` with unit normal `n`, summed**: the lever of a reading
    /// pivoted at both at once (a cone's generator at its apex and a
    /// plane at its point nearest the ball's centre).
    #[must_use]
    pub fn lever_from_point_and_plane(
        self,
        pivot: Point3<T>,
        (p0, n): (Point3<T>, Vec3<T>),
    ) -> Lever<T> {
        Lever::of_consumed_extent(
            (self.center - pivot).norm()
                + (self.center - p0).dot(n).abs()
                + self.radius
                + self.radius,
        )
    }

    /// **The section certificate's tilt lever about `pivot`**: the
    /// ball's farthest distance from it, doubled, so a tilt decided Zero
    /// there moves the partner by at most half the band across the ball
    /// and leaves the other half to an offset (`topo`'s
    /// `boolean::section_cert` module docs, the lever and its pivot).
    #[must_use]
    pub fn twice_lever_from(self, pivot: Point3<T>) -> Lever<T> {
        Lever::of_consumed_extent(self.reach_from(pivot) * T::from_f64(2.0))
    }

    /// **The ball's pivot on `axis`**: the point of the axis nearest its
    /// centre, which is the middle of the ball's window along the axis
    /// and where a tilt's lever from the axis is least. [`Reach::foot_on`]
    /// reads a ball here.
    #[must_use]
    pub fn foot_on(self, axis: Line<T>) -> Point3<T> {
        axis.foot(self.center)
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

    /// **The box `[lo, hi]`'s diagonal**: the farthest two of its points
    /// stand apart, the diameter of [`Self::of_box`]'s ball.
    #[must_use]
    pub fn box_diagonal(lo: Point3<T>, hi: Point3<T>) -> Lever<T> {
        Lever::of_consumed_extent((hi - lo).norm())
    }

    /// **The ball about `at` holding a region**: out to `cover`'s far
    /// distance from `at` ([`Cover::distance_range`]). A region with no
    /// cover (`None`) reads the poison ball ([`Self::of_covered`]), which
    /// a reading levered at it escalates.
    #[must_use]
    pub fn of_cover_from(cover: Option<&Cover<T>>, at: Point3<T>) -> Self {
        cover.map_or_else(
            || Self::of_covered(None),
            |c| Self::new(at, c.distance_range(&Site::Point(at)).1),
        )
    }

    /// **The ball a cover names** ([`Cover::ball`]): it holds the
    /// cover's region, and so the entity the cover was read of.
    #[must_use]
    pub fn of_cover(cover: &Cover<T>) -> Self {
        let (center, radius) = cover.ball();
        Self::new(center, radius)
    }

    /// **The ball holding a region and every point within `tolerance`
    /// of it**: the cover's ball ([`Self::of_cover`]) widened by a
    /// geometric tolerance, such as the band a point of two regions'
    /// section is read within.
    #[must_use]
    pub fn of_cover_within(cover: &Cover<T>, tolerance: T) -> Self {
        let (center, radius) = cover.ball();
        Self::new(center, radius + tolerance)
    }

    /// **The ball an entity's cover reading names** ([`Self::of_cover`]
    /// of `covered`, a carrier's [`geom::Surface::cover`] or
    /// [`Curve3::cover`], an edge's or a face's): poison where the entity
    /// has no cover (`None`), which a reading levered at it escalates.
    /// The one spelling of a ball with no cover.
    #[must_use]
    pub fn of_covered(covered: Option<&Cover<T>>) -> Self {
        covered.map_or_else(
            || {
                let p = poison();
                Self::new(Point3::new(p, p, p), p)
            },
            Self::of_cover,
        )
    }

    /// **A fixture's ball**: `radius` about `center`, a test's stated
    /// region with no geometry behind it. Counted by
    /// `scripts/gates/lever-sources.sh`; production builds a ball only
    /// from the geometry it holds.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn fixture(center: Point3<T>, radius: T) -> Self {
        Self::new(center, radius)
    }

    /// **The ball holding two cylinders' crossing, with the common
    /// perpendicular it is read off**: the meet of two walls about the
    /// axes `l1` and `l2` (each direction unit), decided to cross, with
    /// radii `r₁` and `r₂`. With the axes' sine `sin` and their common
    /// perpendicular `gap` long from `foot` along the unit `normal`
    /// ([`Reach::common_perpendicular`], read at `reach`'s pivots), a
    /// point within `r₁` of the first axis and `r₂` of the second stands
    /// within `(r₁ + r₂)/sin` of the perpendicular's midpoint across it,
    /// and within `min r + |gap|/2` along it.
    #[must_use]
    pub(crate) fn of_crossing_cylinders(
        [l1, l2]: [Line<T>; 2],
        [r1, r2]: [T; 2],
        reach: &Reach<T>,
    ) -> (CommonPerpendicular<T>, Self) {
        let perpendicular = reach.common_perpendicular(l1, l2);
        let CommonPerpendicular {
            foot,
            normal,
            sin,
            gap,
        } = perpendicular;
        let half = T::from_f64(0.5);
        let across = (r1 + r2) / sin;
        let along = r1.min(r2) + gap.abs() * half;
        let ball = Self::new(
            foot + normal * (gap * half),
            (across.powi(2) + along.powi(2)).sqrt(),
        );
        (perpendicular, ball)
    }
}

/// **A cone's reach along its generators**
/// ([`Reach::generator_lever`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct GeneratorReach<T: Real> {
    /// The reach's window along the cone's axis from its apex.
    pub(crate) window: (T, T),
    /// The half-angle's sine and cosine.
    pub(crate) sin_cos: (T, T),
    /// The farthest a consumed point stands from the apex along a
    /// generator.
    pub(crate) lever: Lever<T>,
}

/// **Two crossing lines' common perpendicular**
/// ([`Reach::common_perpendicular`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct CommonPerpendicular<T: Real> {
    /// The perpendicular's foot on the first line.
    pub(crate) foot: Point3<T>,
    /// Its unit direction, `a₁ × a₂` normalized.
    pub(crate) normal: Vec3<T>,
    /// The lines' sine, `|a₁ × a₂|`.
    pub(crate) sin: T,
    /// Its signed length along `normal`, from the first line to the
    /// second.
    pub(crate) gap: T,
}

impl ExtentBall<f64> {
    /// **The ball's diameter, read on the `Interval` lane**: the farthest
    /// two of its points stand apart, entering as a certified value.
    #[must_use]
    pub fn certified_diameter(self) -> Lever<geom_core::Interval> {
        Lever::of_consumed_extent(geom_core::Interval::from_certified(self.radius * 2.0))
    }
}

/// **The farthest an arc of `carrier` over `[t0, t1]` stands from
/// `pivot`**: its cover's far distance ([`Cover::distance_range`]),
/// poison where the carrier has no cover over the span.
fn cover_far<T: SpanLocate>(carrier: &Curve3<T>, (t0, t1): (T, T), pivot: Point3<T>) -> T {
    carrier
        .cover_over(ParamWindow::new(t0, t1))
        .map_or_else(|_| poison(), |c| c.distance_range(&Site::Point(pivot)).1)
}

/// How many pieces an edge's span is cut into where its extent is read
/// from points of it ([`edge_extent_inf`]'s sampled pairs, the
/// Lipschitz arms of [`edge_extent_sup`], and the samples' reach from
/// below of [`edge_reach_bounds`]).
const EXTENT_PIECES: u32 = 16;

/// How many pieces [`edge_extent_sup`]'s ellipse arm reads the arc over,
/// each from its middle ([`cover_far`] twice per piece).
const ELLIPSE_PIECES: u32 = 16;

/// The farthest distance between two of `points` (zero for fewer than
/// two). The one spelling of a point set's diameter: a control net's
/// (whose hull holds a positive-weight carrier) or a schedule's.
pub(crate) fn farthest_pair<T: Real>(points: &[Point3<T>]) -> T {
    points.iter().enumerate().fold(T::zero(), |far, (i, a)| {
        points[i + 1..]
            .iter()
            .fold(far, |far, b| far.max((*a - *b).norm()))
    })
}

/// The carrier's points at `t₀ + k·(t₁ − t₀)/n`, `k = 0..=n`, the ends
/// exact.
fn schedule<T: SpanLocate>(carrier: &Curve3<T>, (t0, t1): (T, T), n: u32) -> Vec<Point3<T>> {
    let step = (t1 - t0) / T::count(n);
    (0..=n)
        .map(|k| match k {
            0 => carrier.eval(t0),
            k if k == n => carrier.eval(t1),
            k => carrier.eval(t0 + step * T::count(k)),
        })
        .collect()
}

/// A certified upper bound on `|C′(t)|` over the whole carrier, in
/// metres per parameter: a line's `|dir|`, a circle's radius, an
/// ellipse's larger semi-axis, a spiric's rate bound over its period
/// ([`geom::curves::spiric_rate_bounds`], `ρ ∈ [R − r, R + r]`, the sine
/// held to 1), a spline's [`geom::NurbsCurve3::speed_upper_bound`].
///
/// Each is the closed-form bound in the scalar's own arithmetic
/// (`[simple.conditioned]`): the real bound computed as if exact at
/// `f64`, an enclosure of it at `Interval`. Nothing is charged for
/// rounding.
fn speed_sup<T: SpanLocate>(carrier: &Curve3<T>) -> T {
    match carrier {
        Curve3::Line(geom::Line { dir, .. }) => dir.get().norm(),
        Curve3::Circle(_) | Curve3::Ellipse(_) => match carrier.conic() {
            Some(k) => k.speed_hi(),
            None => unreachable!("a circle or an ellipse is a conic"),
        },
        Curve3::Spiric {
            major_radius,
            minor_radius,
            offset,
            ..
        } => {
            let (big, r) = (major_radius.abs(), minor_radius.abs());
            geom::curves::spiric_rate_bounds(r, *offset, (big - r, big + r), T::one()).0
        }
        Curve3::Nurbs(_) | Curve3::Periodic(_) => match carrier.whole_net() {
            Some(net) => net.speed_upper_bound().get(),
            None => unreachable!("a spline has a net"),
        },
    }
}

/// The extent of an edge's own point set bounded from below: no more
/// than its diameter `D`, the farthest distance between two of its
/// points, and so no more than its arc. The lever arm of a reading
/// consumed along the edge (D4 ¶1) whose definite verdict is a
/// nonzero sign and whose Zero only refuses: a transversality or a
/// uniqueness tube a certificate claims, the root a face move's corner
/// accepts as transversal (`topo`'s `replace_face`, where a graze serves
/// no point), a section's crossing. Such a reading's convention is
/// that a Zero is a refusal, never a served class; under it an arm
/// longer than `D` would certify a crossing too shallow to pin within
/// the edge, and a shorter one only refuses more. Being no longer than
/// the arc, a point that far along the edge lies on it
/// ([`crate::conic_bend_side`]'s witness). Never the wedge between the
/// edge's two faces, which is levered at the faces' extent
/// ([`crate::folded_lever_arm`]'s `reach`).
///
/// A reading that acts on its Zero and only refuses on a definite sign
/// levers at [`edge_extent_sup`]: an arm short of `D` turns a reading
/// that is Positive at `D` into a definite Zero. A reading that acts on
/// every sign reads at both bounds and acts only where they agree
/// ([`LeverBounds`]; from a point of the edge, [`edge_reach_bounds`]).
///
/// Per carrier kind, the max of the chord between the carrier's points
/// at the span's two ends and:
///
/// - **Line** — nothing more: the chord is `D`.
/// - **Circle** (radius r, span Δt = t₁ − t₀): `|r|·(1 − cos(Δt/2))`,
///   an even, branch-free function of Δt that never exceeds `D` (for
///   |Δt| ≤ π, `1 − cos(u) ≤ 2·sin(u)` on `u ∈ [0, π/2]`, i.e.
///   `tan(u/2) ≤ 2`, so it is below the chord's own `2r·sin(Δt/2)`; for
///   |Δt| ∈ (π, τ] it is ≤ 2r) and at closure (Δt → τ, chord → 0)
///   reaches the full carrier diameter `2r`. The winding bound
///   (|Δt| ≤ τ, [`crate::CertifyError::WindingExceeded`]) keeps the
///   cosine in its honest range.
/// - **Ellipse** (semi-axes a ≥ b): the circle fold at the MINOR
///   semi-axis, `b·(1 − cos(Δt/2))` — the ellipse is the image of the
///   radius-b circle (same parameter θ) under a one-axis expansion by
///   a/b ≥ 1, which never shrinks a distance, so its arc's `D`
///   dominates that circle arc's — and the sampled pairs below.
/// - **Spiric** (minor radius r, span Δv): the circle fold at the MINOR
///   radius, `r·(1 − cos(Δv/2))` — in the `(m, axis)` plane the spiric
///   is the image of the minor circle `(r cos v, r sin v)` under
///   `(x, y) ↦ (g(x), y)`, `g(x) = √((R + x)² − d²)`, whose derivative
///   `g′ = ρ/f ≥ 1` never shrinks a distance — and the sampled pairs.
/// - **Nurbs** — the sampled pairs.
///
/// **The sampled pairs** are the farthest pair among the carrier's
/// points at `t₀ + k·(t₁ − t₀)/16`, `k = 0..=16`: each is a distance
/// between two of the edge's points. Where the farthest pair is an end
/// and an interior point (a steep conic arc about its vertex, `D` four
/// to ten times its chord) they read it, and on a closed or nearly
/// closed edge, whose chord collapses, the middle stands across from
/// the ends.
///
/// Total and comparison-free (`max` is the [`Real`] lattice operation).
pub fn edge_extent_inf<T: SpanLocate>(carrier: &Curve3<T>, t0: T, t1: T) -> Lever<T> {
    Lever::of_consumed_extent(extent_inf(carrier, t0, t1, chord(carrier, t0, t1)))
}

/// The unit directions [`edges_extent_inf`] reads a point set's width
/// along: the three axes, the six face diagonals and the four body
/// diagonals of the world box. Some one of them stands within 27.6° of
/// any direction, so the widest of the thirteen widths is at least 0.886
/// of the set's diameter, at a cost linear in the points where the
/// farthest pair ([`farthest_pair`]) is quadratic.
fn width_directions<T: Real>() -> impl Iterator<Item = Vec3<T>> {
    let (o, l) = (T::zero(), T::one());
    let m = o - l;
    [
        [l, o, o],
        [o, l, o],
        [o, o, l],
        [l, l, o],
        [l, m, o],
        [l, o, l],
        [l, o, m],
        [o, l, l],
        [o, l, m],
        [l, l, l],
        [l, l, m],
        [l, m, l],
        [l, m, m],
    ]
    .into_iter()
    .map(|[x, y, z]| Vec3::new(x, y, z).normalize())
}

/// **The extent of several edges' point set, bounded from below**: no
/// more than the farthest two points of the edges stand apart. The lever
/// of a reading consumed over every edge at once (the boundaries of an
/// edge's two faces) whose definite sign is acted on and whose Zero only
/// refuses, as [`edge_extent_inf`] is one edge's. The measure never
/// over-states the edges' reach, the unsafe direction for such a lever.
///
/// It is the largest of each edge's own extent from below
/// ([`edge_extent_inf`]) and the set's width along each of thirteen
/// fixed directions, read off the edges' sampled points
/// (`t₀ + k·(t₁ − t₀)/16`): the two samples a width is read between are
/// points of the edges that far apart along it, so no farther apart than
/// the diameter. Zero for no edge.
///
/// Total and comparison-free (`min`/`max` are the [`Real`] lattice
/// operations).
pub fn edges_extent_inf<'a, T: SpanLocate + 'a>(
    edges: impl IntoIterator<Item = (&'a Curve3<T>, (T, T))>,
) -> Lever<T> {
    let mut own = T::zero();
    let mut samples = Vec::new();
    for (carrier, (t0, t1)) in edges {
        own = own.max(extent_inf(carrier, t0, t1, chord(carrier, t0, t1)));
        samples.extend(schedule(carrier, (t0, t1), EXTENT_PIECES));
    }
    let Some(first) = samples.first().copied() else {
        return Lever::of_consumed_extent(own);
    };
    let width = width_directions::<T>().fold(T::zero(), |widest, dir| {
        let along = |p: &Point3<T>| (*p - first).dot(dir);
        let (lo, hi) = samples.iter().fold((T::zero(), T::zero()), |(lo, hi), p| {
            (lo.min(along(p)), hi.max(along(p)))
        });
        widest.max(hi - lo)
    });
    Lever::of_consumed_extent(own.max(width))
}

/// The chord between the carrier's points at the span's two ends.
fn chord<T: SpanLocate>(carrier: &Curve3<T>, t0: T, t1: T) -> T {
    carrier.eval(t0).distance(carrier.eval(t1))
}

fn extent_inf<T: SpanLocate>(carrier: &Curve3<T>, t0: T, t1: T, chord: T) -> T {
    let half_span = (t1 - t0) * T::from_f64(0.5);
    let fold = |r: T| r * (T::one() - half_span.cos());
    let pairs = || farthest_pair(&schedule(carrier, (t0, t1), EXTENT_PIECES));
    match carrier {
        Curve3::Line(_) => chord,
        Curve3::Circle(circle) => {
            let radius = circle.radius();
            chord.max(fold(radius))
        }
        Curve3::Ellipse(e) => chord.max(fold(e.speed_lo())).max(pairs()),
        Curve3::Spiric { minor_radius, .. } => chord.max(fold(*minor_radius)).max(pairs()),
        Curve3::Nurbs(_) | Curve3::Periodic(_) => chord.max(pairs()),
    }
}

/// The extent of an edge's own point set bounded from above: no two of
/// its points lie farther apart, at every carrier kind and scalar. The
/// lever arm of a reading consumed over the whole edge whose Zero is
/// acted on and whose definite signs only refuse (a tangency verified,
/// with any defect a contradiction), where [`edge_extent_inf`]'s shorter
/// arm would read a definite Zero that the edge's own diameter reads
/// Positive. A reading that acts on its definite signs too reads at
/// both bounds and acts only where they agree ([`LeverBounds`]).
///
/// **Its tightness depends on the carrier.** It is the diameter on a
/// line and a circle arc, and within a few percent of it on an ellipse
/// arc. A spiric's and a spline's Lipschitz arm reads a whole-carrier
/// speed bound, so near a spiric's pinch and on a spline window whose
/// parameter runs unevenly it can pass the diameter many times over
/// (the spline's control net caps it at the whole carrier's diameter).
///
/// Per carrier kind, the max of the chord and the least of:
///
/// - **Line** — nothing more: the chord is the diameter.
/// - **Circle** (radius r, span Δt): `2|r|·sin(min(|Δt|, π)/2)`, the
///   arc's diameter exactly.
/// - **Ellipse** (semi-axes a ≥ b): `2a·sin(min(|Δt|, π)/2)` — the
///   ellipse is the image of the radius-a circle (same parameter θ)
///   under a one-axis contraction by b/a ≤ 1, which never lengthens a
///   distance — and, cutting the span into 16 pieces `Eₖ` about their
///   middles `mₖ`, `maxₖ [reach(E, mₖ) + reach(Eₖ, mₖ)]`, each reach the
///   arc's cover's far distance ([`cover_far`]): a point `x ∈ Eₖ` stands within
///   `reach(Eₖ, mₖ)` of `mₖ`, which stands within `reach(E, mₖ)` of every
///   point.
/// - **Spiric** (major radius R, minor r, stand-off d): the oval's
///   diagonal `√((g(r) − g(−r))² + (2r)²)`, `g(x) = √((R + x)² − d²)`
///   (it lies in `m ∈ [g(−r), g(r)]`, `|axis| ≤ r`); the minor circle's
///   arc stretched by `L = (R − r)/g(−r)`, `L·2r·sin(min(|Δv|, π)/2)`
///   (in the `(m, axis)` plane the spiric is the image of the minor
///   circle under `(x, y) ↦ (g(x), y)`, whose stretch `g′` is greatest at
///   `x = −r`); and the Lipschitz pairs below.
/// - **Nurbs** — the farthest pair of its control points (with positive
///   weights, a construction invariant of the curve, the whole carrier
///   lies in its net's convex hull, whose diameter is the farthest pair
///   of its vertices), and the Lipschitz pairs below, which read the
///   edge's own window where the net is the whole carrier's.
///
/// **The Lipschitz pairs**: the sampled pairs ([`edge_extent_inf`]) plus
/// `s·|Δt|/16`, `s` the carrier's speed bound (a spiric's
/// [`geom::curves::spiric_rate_bounds`] over its period, a spline's
/// [`geom::NurbsCurve3::speed_upper_bound`]): every point of the edge
/// lies within `s·|Δt|/32` of a sample, so no two lie farther apart than
/// two samples and twice that.
///
/// Total and comparison-free (`min`/`max` are the [`Real`] lattice
/// operations, which propagate poison: a spline whose speed bound or
/// control net is poison reads poison here, and the reading levered at
/// it escalates).
pub fn edge_extent_sup<T: SpanLocate>(carrier: &Curve3<T>, t0: T, t1: T) -> Lever<T> {
    Lever::of_consumed_extent(extent_sup(carrier, t0, t1, chord(carrier, t0, t1)))
}

fn extent_sup<T: SpanLocate>(carrier: &Curve3<T>, t0: T, t1: T, chord: T) -> T {
    let two = T::from_f64(2.0);
    let span = (t1 - t0).abs();
    // `sin(min(|Δt|, π)/2)`: the half-span folded to a half turn, past
    // which an arc of a circle holds a diameter.
    let fold = (span.min(T::pi()) * T::from_f64(0.5)).sin();
    let lipschitz = || {
        farthest_pair(&schedule(carrier, (t0, t1), EXTENT_PIECES))
            + speed_sup(carrier) * span / T::count(EXTENT_PIECES)
    };
    match carrier {
        Curve3::Line(_) => chord,
        Curve3::Circle(circle) => {
            let radius = circle.radius();
            chord.max(two * radius * fold)
        }
        Curve3::Ellipse(e) => {
            let step = (t1 - t0) / T::count(ELLIPSE_PIECES);
            let pieces = (0..ELLIPSE_PIECES).fold(T::zero(), |far, k| {
                let s0 = t0 + step * T::count(k);
                let x = carrier.eval(s0 + step * T::from_f64(0.5));
                far.max(cover_far(carrier, (t0, t1), x) + cover_far(carrier, (s0, s0 + step), x))
            });
            chord.max((two * e.speed_hi() * fold).min(pieces))
        }
        Curve3::Spiric {
            major_radius,
            minor_radius,
            offset,
            ..
        } => {
            let (big, r, d) = (major_radius.abs(), minor_radius.abs(), *offset);
            let g = |x: T| ((big + x).powi(2) - d.powi(2)).sqrt();
            let (inner, outer) = (g(-r), g(r));
            let stretch = (big - r) / inner;
            let diagonal = ((outer - inner).powi(2) + (two * r).powi(2)).sqrt();
            chord.max((stretch * two * r * fold).min(diagonal).min(lipschitz()))
        }
        // A poison speed bound (a rational degree past its ceiling) or a
        // poison net reaches the result: `min`/`max` propagate it, and the
        // reading levered at it escalates rather than taking the other arm.
        Curve3::Nurbs(_) | Curve3::Periodic(_) => match carrier.whole_net() {
            Some(net) => chord.max(farthest_pair(net.control()).min(lipschitz())),
            None => unreachable!("a spline has a net"),
        },
    }
}

/// The largest `measure` of the edge's points at `t₀ + k·(t₁ − t₀)/16`,
/// `k = 0..=16`: each a point of the edge, so the edge's largest
/// `measure` is never below it.
fn sampled_reach<T: SpanLocate>(
    carrier: &Curve3<T>,
    span: (T, T),
    measure: impl Fn(Point3<T>) -> T,
) -> T {
    schedule(carrier, span, EXTENT_PIECES)
        .into_iter()
        .fold(T::zero(), |far, x| far.max(measure(x)))
}

/// **Whether the region an edge's cover holds is the edge's own**, so a
/// far distance or a coordinate range read off it is exact: on a line
/// and on a circle arc, on no other kind. The one home of that split,
/// which [`edge_reach_bounds`] and [`edge_coordinate_reach_below`] both
/// read.
fn cover_is_exact<T: Real>(carrier: &Curve3<T>) -> bool {
    match carrier {
        Curve3::Line(_) | Curve3::Circle(..) => true,
        Curve3::Ellipse(..) | Curve3::Spiric { .. } | Curve3::Nurbs(_) | Curve3::Periodic(_) => {
            false
        }
    }
}

/// **An edge's largest `|coordinate|`, bounded from below**: the session
/// box's measure (`[tolerance.units]`: no coordinate of a model is larger
/// in magnitude than the box). `world_box` is the box the region the edge
/// occupies spans along the world axes, which the caller reads off that
/// region (topo reads its edge's cover, `Cover::box_in` at
/// `BoxAxes::World`). Where the cover is the edge's own
/// ([`cover_is_exact`]) the box's largest `|coordinate|` is exact;
/// otherwise, and where there is no box, the samples' largest, each a
/// point of the edge. So a missing cover reads the edge's own points,
/// never poison.
pub fn edge_coordinate_reach_below<T: SpanLocate>(
    carrier: &Curve3<T>,
    span: (T, T),
    world_box: Option<(Point3<T>, Point3<T>)>,
) -> T {
    let origin = Point3::origin();
    match world_box.filter(|_| cover_is_exact(carrier)) {
        Some((lo, hi)) => (lo - origin).norm_inf().max((hi - origin).norm_inf()),
        None => sampled_reach(carrier, span, |x| (x - origin).norm_inf()),
    }
}

/// **A lever known exactly, or only between two bounds.** A reading
/// that acts on every sign and is levered at a length known only from
/// both sides is read at each bound, and decides only where the two
/// readings agree: an arm short of the length turns a reading that is
/// Positive at it into a definite Zero, and an arm past it turns an
/// in-band reading Positive, so neither bound alone answers for it.
/// Where they disagree the reading is the bounds' straddle of the band
/// (`geom_core::k_stats::straddle`), refused as an in-band reading is.
#[derive(Clone, Copy, Debug)]
pub enum LeverBounds<T> {
    /// The length itself: one reading answers for it.
    Exact(Lever<T>),
    /// A length known to lie in `[below, above]`.
    Between {
        /// A length never past it.
        below: Lever<T>,
        /// A length never short of it.
        above: Lever<T>,
    },
}

impl<T: Real> LeverBounds<T> {
    /// The bound from below (the length itself where exact).
    pub fn below(self) -> Lever<T> {
        match self {
            Self::Exact(x) | Self::Between { below: x, .. } => x,
        }
    }

    /// The bound from above (the length itself where exact).
    pub fn above(self) -> Lever<T> {
        match self {
            Self::Exact(x) | Self::Between { above: x, .. } => x,
        }
    }

    /// The bounds on the lesser of two lengths: each bound the lesser of
    /// the two's, exact where both are.
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        match (self, other) {
            (Self::Exact(a), Self::Exact(b)) => Self::Exact(a.min(b)),
            _ => Self::Between {
                below: self.below().min(other.below()),
                above: self.above().min(other.above()),
            },
        }
    }
}

/// A length known exactly.
impl<T> From<Lever<T>> for LeverBounds<T> {
    fn from(x: Lever<T>) -> Self {
        Self::Exact(x)
    }
}

/// **How far an edge reaches from `pivot`, bounded from both sides**,
/// given `above`, the far distance from `pivot` of the region the edge
/// occupies, which the caller reads off that region (topo reads its
/// edge's cover, the carrier's hulled with the edge's ends): the lever
/// of a reading that acts on every sign, read from a point of the edge
/// (a corner's departure, a tangent at an interior point). On a line or
/// a circle arc that far distance is the reach itself; otherwise the
/// reach lies between the samples' reach (each sample a point of the
/// edge) and `above`, so a consumer reads at both and acts only on a
/// verdict they share.
pub fn edge_reach_bounds<T: SpanLocate>(
    carrier: &Curve3<T>,
    (t0, t1): (T, T),
    pivot: Point3<T>,
    above: Lever<T>,
) -> LeverBounds<T> {
    if cover_is_exact(carrier) {
        LeverBounds::Exact(above)
    } else {
        LeverBounds::Between {
            below: Lever::of_consumed_extent(sampled_reach(carrier, (t0, t1), |x| {
                (x - pivot).norm()
            })),
            above,
        }
    }
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
    /// A length measured from `at` (the germ frame's wall's radius,
    /// [`Self::of_frame`]), read at the foot of `at`. Its lever
    /// is that length ([`Self::lever_from`]), floored for the cylinder
    /// pair at the foot's distance from `at` ([`Self::lever_between`]).
    /// Built only by [`Self::of_frame`].
    #[non_exhaustive]
    Measured {
        /// The point the caller measured from.
        at: Point3<T>,
        /// The caller's length.
        lever: T,
    },
    /// A face the caller measured from `at` against a cylinder or a
    /// cone: how far it reaches either way along that surface's axis
    /// from `at`, and its farthest distance from `at`. The plane×cylinder
    /// callers hand it, read at the foot of `at`
    /// (`hinge_lever`, `turn_lever`); the plane×cone
    /// callers hand it, read at the apex ([`Self::lever_from`],
    /// [`Self::range_along`]). Built only by [`Self::of_face`].
    #[non_exhaustive]
    Face {
        /// The point the caller measured from.
        at: Point3<T>,
        /// How far the face reaches from `at` against the axis.
        below: T,
        /// How far the face reaches from `at` along the axis.
        above: T,
        /// The face's farthest distance from `at`.
        across: Lever<T>,
    },
    /// A region the caller holds as its cover: an edge over its span
    /// ([`Self::of_span`], or topo's edge cover), read at the middle of
    /// its window along an axis ([`Self::foot_on`]) and levered by the
    /// cover's far distance from there ([`Self::lever_from`]).
    Region(Cover<T>),
}

impl<T: Real> Reach<T> {
    /// **A plane×cylinder germ frame's reach, read at `at`**
    /// ([`Self::Measured`]): the cylinder wall's radius, for a pair
    /// handed without its faces. Any other surface has no frame here and reads poison,
    /// which a reading levered at it escalates.
    #[must_use]
    pub fn of_frame(at: Point3<T>, wall: &geom::Surface<T>) -> Self {
        let lever = match wall {
            geom::Surface::Cylinder { radius, .. } => *radius,
            _ => poison(),
        };
        Self::Measured { at, lever }
    }

    /// **A face measured from `at`** ([`Self::Face`]): its window along
    /// the surface's axis either way from `at`, `below` and `above` (the
    /// axial positions of its boundary, as its reader reads them), and
    /// its farthest distance from `at`, a read of its boundary.
    #[must_use]
    pub fn of_face(at: Point3<T>, (below, above): (T, T), across: Lever<T>) -> Self {
        Self::Face {
            at,
            below,
            above,
            across,
        }
    }

    /// **A fixture's measured reach**: `length` from `at`, a test's
    /// stated scale with no frame behind it. Counted by
    /// `scripts/gates/lever-sources.sh`; production builds a measured
    /// reach only through [`Self::of_frame`].
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn fixture(at: Point3<T>, length: T) -> Self {
        Self::Measured { at, lever: length }
    }
}

impl<T: SpanLocate> Reach<T> {
    /// **An edge over its span, as its region** ([`Self::Region`]): the
    /// carrier's cover over `[t0, t1]` ([`Curve3::cover_over`]). For a
    /// reader with a carrier and a span but no edge record; a reader of
    /// an edge hands its edge's own cover.
    ///
    /// # Errors
    ///
    /// [`Uncovered`] where the carrier has no cover over the span.
    pub fn of_span(carrier: &Curve3<T>, (t0, t1): (T, T)) -> Result<Self, Uncovered> {
        carrier
            .cover_over(ParamWindow::new(t0, t1))
            .map(Self::Region)
    }
}

impl<T: Real> Reach<T> {
    /// The lever a tilt pinned at `pivot` is metered at. For a reach
    /// that holds its region ([`Self::Ball`], [`Self::Region`]), an
    /// upper bound on the region's distance from `pivot`; for a length
    /// the caller measured ([`Self::Measured`], [`Self::Face`]), that
    /// length, whatever the pivot: the region's own reach from a point
    /// of it, which is what a tilt that moves the region by ε is ε over.
    ///
    /// - [`Self::Ball`]: the ball's far side from `pivot`.
    /// - [`Self::Measured`]: the caller's length, which a pair of lines
    ///   also floors at the foot's distance from `at`
    ///   ([`Self::lever_between`]).
    /// - [`Self::Face`]: its farthest distance from `at`: the plane×cone
    ///   arm levers its trileans at it. The plane×cylinder row reads a
    ///   face from its hinge instead (`hinge_lever`).
    /// - [`Self::Region`]: the cover's far distance from `pivot`
    ///   ([`Cover::distance_range`]): exact on a segment and a circle arc.
    #[must_use]
    pub fn lever_from(&self, pivot: Point3<T>) -> Lever<T> {
        Lever::of_consumed_extent(self.reach_from(pivot))
    }

    fn reach_from(&self, pivot: Point3<T>) -> T {
        match self {
            Self::Ball(ball) => ball.reach_from(pivot),
            Self::Measured { lever, .. } => *lever,
            Self::Face { across, .. } => across.length(),
            Self::Region(cover) => cover.distance_range(&Site::Point(pivot)).1,
        }
    }

    /// How far the consumed region reaches from `pivot` along `dir`,
    /// either way: bounds on the least and the greatest `(x − pivot)·dir`
    /// over the consumed points `x`, never inside the true ones. The
    /// plane×cylinder face callers measure a face with this, one
    /// boundary edge's [`Self::Region`] at a time ([`Self::Face`]).
    ///
    /// - [`Self::Ball`]: the centre's reading, less and plus the radius
    ///   times `|dir|`.
    /// - [`Self::Measured`]: the caller's length either way from `at`;
    ///   [`Self::Face`]: its `below` and `above` from `at` (along its
    ///   surface's axis, which `dir` is); each moved by `at`'s own
    ///   reading `(at − pivot)·dir`.
    /// - [`Self::Region`]: the cover's range along `dir`
    ///   ([`Cover::range_along`]): exact on a segment and a conic arc.
    #[must_use]
    pub fn range_along(&self, pivot: Point3<T>, dir: Vec3<T>) -> (T, T) {
        let at = |p: Point3<T>| (p - pivot).dot(dir);
        match self {
            Self::Ball(ball) => {
                let (c, r) = (at(ball.center()), ball.radius() * dir.norm());
                (c - r, c + r)
            }
            Self::Measured { at: from, lever } => (at(*from) - *lever, at(*from) + *lever),
            Self::Face {
                at: from,
                below,
                above,
                ..
            } => (at(*from) - *below, at(*from) + *above),
            Self::Region(cover) => {
                let base = (pivot - Point3::origin()).dot(dir);
                let (lo, hi) = cover.range_along(dir);
                (lo - base, hi - base)
            }
        }
    }

    /// **The plane×cylinder row's lever from the rulings' hinge**: the
    /// farthest a consumed point stands along the axis from the hinge's
    /// station, `shift` along the axis from `pivot` (the foot the gap is
    /// read at). A [`Self::Face`] reads it exactly from its two sides,
    /// `max(|above − shift|, |below + shift|)`; every other reach adds
    /// `|shift|` to its lever from the foot.
    #[must_use]
    fn hinge_lever(&self, pivot: Point3<T>, shift: T) -> Lever<T> {
        Lever::of_consumed_extent(match self {
            Self::Face { below, above, .. } => (*above - shift).abs().max((*below + shift).abs()),
            Self::Ball(_) | Self::Measured { .. } | Self::Region(_) => {
                self.reach_from(pivot) + shift.abs()
            }
        })
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
    fn turn_lever(
        &self,
        hinge: Point3<T>,
        (n, a): (Vec3<T>, UnitVec3<T>),
        c: T,
        cos: T,
    ) -> Lever<T> {
        Lever::of_consumed_extent(match self {
            Self::Face { at, across, .. } => {
                let off_perp = Line {
                    origin: hinge,
                    dir: a,
                }
                .offset(*at);
                let off_axis = off_perp.norm();
                let m = n - a.get() * n.dot(a.get());
                let len = m.norm().max(T::from_f64(f64::MIN_POSITIVE));
                let along_normal =
                    (off_perp.dot(m).abs() + off_axis * T::from_f64(8.0 * f64::EPSILON)) / len;
                (across.length() + off_axis.min(along_normal)) * c.abs() / (T::one() + cos)
            }
            Self::Ball(_) | Self::Measured { .. } | Self::Region(_) => T::zero(),
        })
    }

    /// **The plane×cylinder row's frame** for the pair `pc`: the reach's
    /// pivot on the cylinder's axis ([`Self::foot_on`]), the axis's
    /// normal component `c`, the pivot's signed height over the plane,
    /// and the row's whole lever from there, its lever from the rulings'
    /// hinge (`hinge_lever`) plus the tilt's turn across the wall
    /// (`turn_lever`). The hinge stands on the plane under the pivot,
    /// `−gap·c` along the axis from it. The gap and the lever are read
    /// at the one pivot because they are read here together.
    #[must_use]
    pub(crate) fn ruled_pivot_lever(
        &self,
        pc: &crate::intersect::PlaneCylinder<T>,
    ) -> crate::intersect::RuledFrame<T> {
        let &crate::intersect::PlaneCylinder { q, n, o, a, .. } = pc;
        let o = self.foot_on(Line { origin: o, dir: a });
        let c = a.get().dot(n);
        let gap_signed = (o - q).dot(n);
        let hinge = o - n * gap_signed;
        let cos = (T::one() - c.powi(2)).max(T::zero()).sqrt();
        let lever = Lever::of_consumed_extent(
            self.hinge_lever(o, -(gap_signed * c)).length()
                + self.turn_lever(hinge, (n, a), c, cos).length(),
        );
        crate::intersect::RuledFrame {
            o,
            c,
            gap_signed,
            lever,
        }
    }

    /// **A cone's reach along its generators**, for the cone about
    /// `axis` (its origin the apex, its direction unit) of half-angle
    /// `half_angle`: the reach's window along the axis from the apex
    /// ([`Self::range_along`]), the half-angle's sine and cosine, and the
    /// farthest a consumed point of the cone stands from the apex along
    /// a generator, the window's larger end over the cosine. One read of
    /// each, for a lane that needs the window and the cosine beside the
    /// lever.
    #[must_use]
    pub(crate) fn generator_lever(&self, axis: Line<T>, half_angle: T) -> GeneratorReach<T> {
        let window = self.range_along(axis.origin, axis.dir.get());
        let sin_cos = half_angle.sin_cos();
        let (lo, hi) = window;
        GeneratorReach {
            window,
            sin_cos,
            lever: Lever::of_consumed_extent(lo.abs().max(hi.abs()) / sin_cos.1),
        }
    }

    /// **The common perpendicular of two crossing lines**, read at the
    /// reach's pivots on them ([`Self::foot_on`]): its unit direction
    /// `a₁ × a₂`, the lines' sine, its signed length, and its foot on
    /// `l1`, stepped along `l1` from the pivot there. For lines decided
    /// to cross: it divides by the sine's square.
    #[must_use]
    pub(crate) fn common_perpendicular(&self, l1: Line<T>, l2: Line<T>) -> CommonPerpendicular<T> {
        let (a1, a2) = (l1.dir.get(), l2.dir.get());
        let cross = a1.cross(a2);
        let (normal, sin) = cross.direction_and_length();
        let f1 = self.foot_on(l1);
        let w = self.foot_on(l2) - f1;
        let gap = w.dot(normal);
        let foot = f1 + a1 * (w.cross(a2).dot(cross) / sin.powi(2));
        CommonPerpendicular {
            foot,
            normal,
            sin,
            gap,
        }
    }

    /// **The pivot on the line `axis` the reach is read at**: the origin
    /// of the tilt bound its lever meters, not a claim about the region,
    /// so any point of the axis is sound provided the datum and its
    /// lever are read at it (`geom-core`'s README, "Float rounding").
    ///
    /// - [`Self::Face`]: the foot of its own `at`, which its measures are
    ///   read from.
    /// - [`Self::Ball`]: the foot of its centre ([`ExtentBall::foot_on`]),
    ///   the middle of its window along the axis.
    /// - [`Self::Measured`] and [`Self::Region`]: the middle of the
    ///   window along the axis ([`Self::range_along`]); for a measured
    ///   length that is the foot of `at` in exact arithmetic. A region's
    ///   lever from there is at most `√2` times its cover's least far
    ///   distance from any point of the axis.
    #[must_use]
    pub fn foot_on(&self, axis: Line<T>) -> Point3<T> {
        match self {
            Self::Face { at, .. } => axis.foot(*at),
            Self::Ball(ball) => ball.foot_on(axis),
            Self::Measured { .. } | Self::Region(_) => {
                let (lo, hi) = self.range_along(axis.origin, axis.dir.get());
                axis.at((lo + hi) * T::from_f64(0.5) / axis.dir.get().norm_squared())
            }
        }
    }

    /// **A ball holding the consumed region**: a ball's own, a region's
    /// cover's ([`Cover::ball`]), and about a measured point out to the
    /// caller's length.
    #[must_use]
    pub fn ball(&self) -> ExtentBall<T> {
        match self {
            Self::Ball(ball) => *ball,
            Self::Region(cover) => ExtentBall::of_cover(cover),
            Self::Measured { at, .. } | Self::Face { at, .. } => {
                ExtentBall::new(*at, self.reach_from(*at))
            }
        }
    }

    /// The lever of a relative tilt between two lines `oᵢ + s·aᵢ`
    /// (`aᵢ` unit) whose gap is read at their feet ([`Self::foot_on`]):
    /// the lesser of the two feet's levers. Holding either line and
    /// turning the other about its own foot bounds the same displacement
    /// across the reach, so the lesser bound holds as well, and it does
    /// not depend on which line is named first. (Two carriers each
    /// displaced about its own pivot are a different question, whose arm
    /// is the larger of the two reaches: [`ExtentBall::lever_from_either`].)
    ///
    /// **Why a foot's lever, and not a ball's radius alone.** Axial
    /// travel along parallel lines does not move their gap, which
    /// suggests the radius would do. But the gap is read at a foot, each
    /// foot's distance to the other line, and while the lines are
    /// parallel only within the band that distance changes along the
    /// axis at the tilt's rate: at a consumed point a foot's lever `ρ`
    /// away it has moved by up to `ρ·θ`. A lever shorter than a foot's
    /// would let that change exceed what the gap row was told it
    /// bridges.
    ///
    /// **The lever here is floored at the foot's distance from `at`** for
    /// a length the caller measured ([`Self::Measured`], [`Self::Face`]),
    /// and only here; a ball's or a region's lever from the foot already
    /// reaches every point it holds. The caller measured its length along
    /// the axis from `at`, and the foot stands off `at` across the axis
    /// too, which a tilt about the foot turns into travel at `at`, so
    /// this lever reaches at least that far. The plane×cylinder row
    /// moves its section by the tilt times the AXIAL distance from the
    /// foot, which a [`Self::Face`]'s length already measures, so it
    /// takes that length bare: floored there, a face shorter than the
    /// radius would be levered past its own measure, and a tilt the
    /// length leaves in the band served.
    #[must_use]
    pub fn lever_between(&self, line1: Line<T>, line2: Line<T>) -> Lever<T> {
        self.foot_lever(line1).1.min(self.foot_lever(line2).1)
    }

    /// One line's half of [`Self::lever_between`]: the foot on the line
    /// `axis` ([`Self::foot_on`]) and its lever, floored for a measured
    /// length at the foot's distance from `at`.
    #[must_use]
    pub fn foot_lever(&self, axis: Line<T>) -> (Point3<T>, Lever<T>) {
        let pivot = self.foot_on(axis);
        let reach = self.reach_from(pivot);
        let lever = match self {
            Self::Measured { at, .. } | Self::Face { at, .. } => reach.max((*at - pivot).norm()),
            Self::Ball(_) | Self::Region(_) => reach,
        };
        (pivot, Lever::of_consumed_extent(lever))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom::Surface;

    /// **The sampled arm reads coordinates, not distances**: an ellipse
    /// out on the box's diagonal, centred at `(700, 700, 0)` with semi-axes
    /// 150 and 100 in the `xy` plane, has every coordinate within 850 m and
    /// points up to about 1.14 km from the origin. Its largest `|coordinate|`
    /// from below is the samples' (the `x` extreme, 850, is a sample), in
    /// the box; a distance read there is past it. Red against the sampled
    /// arm reading `norm()`. A line or arc with no cover reads its samples
    /// too, never poison.
    #[test]
    fn an_edge_out_on_the_diagonal_reads_its_largest_coordinate() {
        let ellipse = Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(
                Point3::new(700.0, 700.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            150.0,
            100.0,
        ));
        let reach = edge_coordinate_reach_below(&ellipse, (0.0, core::f64::consts::TAU), None);
        assert!((reach - 850.0).abs() < 1e-9, "{reach}");
        assert!(reach < geom_core::tolerance::SESSION_BOX_EXTENT);
        let line = Curve3::Line(geom::Line {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: geom_core::UnitVec3::squared(Vec3::new(1.0, 1.0, 0.0)),
        });
        let reach = edge_coordinate_reach_below(&line, (0.0, 1200.0), None);
        assert!(
            reach.is_finite(),
            "an uncovered line reads its samples: {reach}"
        );
        assert!((reach - 1200.0 / 2f64.sqrt()).abs() < 1e-9, "{reach}");
    }

    /// **A conic span's lever reaches every point of the span** from any
    /// pivot, and over a whole turn of a circle it is that circle's
    /// farthest point.
    #[test]
    fn a_conic_spans_lever_reaches_every_point() {
        let pivot = Point3::new(0.3, -0.2, 0.7);
        for carrier in [
            Curve3::circle(
                Point3::new(1.0, 2.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                1.5,
                Vec3::new(1.0, 0.0, 0.0),
            ),
            Curve3::Ellipse(geom::Ellipse::new(
                geom::Frame::new(
                    Point3::new(-1.0, 0.5, 0.2),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 0.0),
                ),
                0.6,
                1.4,
            )),
        ] {
            for t1 in [1.0, core::f64::consts::TAU] {
                let lever = Reach::of_span(&carrier, (0.0, t1))
                    .expect("the span is covered")
                    .lever_from(pivot)
                    .length();
                let mut farthest = 0.0_f64;
                for k in 0..=720 {
                    let t = t1 * f64::from(k) / 720.0;
                    let far = (carrier.eval(t) - pivot).norm();
                    farthest = farthest.max(far);
                    assert!(
                        lever >= far,
                        "{carrier:?}: {lever} falls short of {far} at {t}"
                    );
                }
                if let (Curve3::Circle(c), true) = (&carrier, t1 > 1.0) {
                    // The whole circle's farthest point from the pivot, exactly:
                    // `√(h² + (ρ + r)²)`, `h` off its plane, `ρ` across it.
                    let w = pivot - c.frame().origin();
                    let h = w.dot(c.frame().axis());
                    let rho = (w - c.frame().axis() * h).norm();
                    let exact = (h * h + (rho + c.radius()).powi(2)).sqrt();
                    assert!(
                        (lever - exact).abs() <= 1e-12 && farthest <= exact,
                        "{carrier:?}: {lever} is not the circle's farthest point {exact}"
                    );
                }
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
                    let g = geom_core::GOLDEN_SECTION;
                    let (m1, m2) = (lo + (hi - lo) * (1.0 - g), lo + (hi - lo) * g);
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

    #[test]
    fn a_conic_arcs_reach_is_its_farthest_point_over_the_span() {
        let off = Point3::new(1e3, -1e3, 1e3);
        let tilted = Vec3::new(0.6, 0.0, 0.8);
        for r in [1e-3, 1.0, 1e3] {
            let carriers = [
                Curve3::circle(off, tilted, r, Vec3::new(0.0, 1.0, 0.0)),
                Curve3::Ellipse(geom::Ellipse::new(
                    geom::Frame::new(off, tilted, Vec3::new(-0.8, 0.0, 0.6)),
                    2.0 * r,
                    0.7 * r,
                )),
            ];
            for carrier in carriers {
                let (a, b) = match carrier {
                    Curve3::Circle(circle) => {
                        let radius = circle.radius();
                        (radius, radius)
                    }
                    Curve3::Ellipse(ellipse) => {
                        let (semi_u, semi_v) = (ellipse.semi_u(), ellipse.semi_v());
                        (semi_u, semi_v)
                    }
                    _ => unreachable!(),
                };
                let big = a.max(b);
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
                            Curve3::Circle(_) | Curve3::Ellipse(_) => {
                                let frame = carrier.frame().unwrap();
                                (frame.u_ref(), frame.axis())
                            }
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
                        let reach =
                            Reach::of_span(&carrier, (t0, t1)).expect("the span is covered");
                        let got = reach.lever_from(pivot).length();
                        let far = farthest_over(&carrier, (t0, t1), pivot);
                        let ulps = 1e-15
                            * ((off - Point3::origin()).norm()
                                + (pivot - Point3::origin()).norm()
                                + big);
                        let span = (t1 - t0).abs().min(core::f64::consts::TAU);
                        let slack = match carrier {
                            Curve3::Circle(..) => 0.0,
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
            Curve3::circle(
                Point3::new(1.0, 2.0, 0.0),
                tilted,
                1.5,
                Vec3::new(0.0, 1.0, 0.0),
            ),
            Curve3::Ellipse(geom::Ellipse::new(
                geom::Frame::new(
                    Point3::new(-1.0, 0.5, 0.2),
                    tilted,
                    Vec3::new(-0.8, 0.0, 0.6),
                ),
                2.0,
                0.7,
            )),
            Curve3::Ellipse(geom::Ellipse::new(
                geom::Frame::new(
                    Point3::new(-1.0, 0.5, 0.2),
                    tilted,
                    Vec3::new(-0.8, 0.0, 0.6),
                ),
                0.7,
                2.0,
            )),
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
                        &Reach::of_span(carrier, (span.0, span.1)).expect("the span is covered"),
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
            frame: geom::Frame::new(Point3::new(0.0, 0.0, 0.0), z, Vec3::new(1.0, 0.0, 0.0)),
            major_radius: 2.0,
            minor_radius: 0.5,
            offset: 0.7,
        };
        let tau = core::f64::consts::TAU;
        let lever = axial(
            &Reach::of_span(&spiric, (0.0, tau)).expect("the span is covered"),
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
            &Reach::of_span(&spline, (0.0, 1.0)).expect("the span is covered"),
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
        let face = Reach::of_face(at, (0.0, 0.0), ExtentBall::point(Point3::origin()).lever());
        let a = Vec3::new(0.0, 0.0, 1.0);
        let tilts = [1e-4_f64, 0.3, 1.0, core::f64::consts::FRAC_PI_2].map(f64::sin_cos);
        for (c, cos) in tilts.into_iter().chain([(1.0, 0.0)]) {
            let beta = c.atan2(cos);
            let n = Vec3::new(cos, 0.0, c);
            let lever = face
                .turn_lever(
                    Point3::new(0.0, 0.0, 0.0),
                    (n, geom::stated_direction(a)),
                    c,
                    cos,
                )
                .length();
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
        let leaning = Reach::of_face(
            Point3::origin() + off,
            (0.0, 0.0),
            ExtentBall::point(Point3::origin()).lever(),
        );
        let lever = leaning
            .turn_lever(
                Point3::origin(),
                (tilted, geom::stated_direction(tilted)),
                c,
                cos,
            )
            .length();
        let want = c / (1.0 + cos);
        assert!(
            lever >= want * (1.0 - 1e-12),
            "a normal on the axis to rounding: lever {lever}, the point's turn {want}"
        );
        let measured = Reach::fixture(at, 1.0);
        let n = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(
            measured
                .turn_lever(
                    Point3::new(0.0, 0.0, 0.0),
                    (n, geom::stated_direction(a)),
                    1.0,
                    0.0
                )
                .length(),
            0.0
        );
    }

    fn xyz(p: Point3<f64>) -> [f64; 3] {
        [p.x, p.y, p.z]
    }

    /// The torus's lever from its own centre is `R + r`, to the cover's
    /// rounding charge and never short of it: the farthest any point of
    /// the ring stands from the pivot its axis turns on.
    #[test]
    fn a_torus_levers_at_its_outer_radius() {
        let torus = Surface::Torus {
            frame: geom::Frame::new(
                Point3::new(1.0, 2.0, 3.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            major_radius: 2.0,
            minor_radius: 0.5,
        };
        let ball = ExtentBall::of_cover(&torus.cover().unwrap());
        for (pivot, want) in [((1.0, 2.0, 3.0), 2.5), ((1.0, 2.0, 7.0), 6.5)] {
            let got = ball
                .lever_from(Point3::new(pivot.0, pivot.1, pivot.2))
                .length();
            assert!(
                got >= want && got - want < 1e-13,
                "from {pivot:?}: {got} against {want}"
            );
        }
    }

    /// **An entity with no cover reads the poison ball**, which no
    /// lever reads past: [`ExtentBall::of_covered`] of `None`, and the
    /// ball about a point of a region with no cover. A finite ball here
    /// would lever a reading over a region nobody bounded.
    #[test]
    fn an_entity_with_no_cover_reads_the_poison_ball() {
        let at = Point3::new(1.0, 2.0, 3.0);
        for (label, ball) in [
            ("of_covered", ExtentBall::<f64>::of_covered(None)),
            ("of_cover_from", ExtentBall::of_cover_from(None, at)),
        ] {
            assert!(ball.readable().is_none(), "{label}: {ball:?} reads");
            assert!(
                ball.lever_from(at).length().is_poison(),
                "{label}: its lever from a point is not poison"
            );
        }
        let covered = Surface::Sphere {
            frame: geom::Frame::new(at, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)),
            radius: 2.0,
        }
        .cover()
        .unwrap();
        let ball = ExtentBall::of_covered(Some(&covered));
        assert!(
            (ball.radius() - 2.0).abs() < 1e-12,
            "a sphere's cover reads its own ball, got {ball:?}"
        );
    }

    /// **A short arc far from its centre levers from a point of the
    /// arc**: a 1 m arc of a 1 km circle. Its ball and its foot's lever
    /// are read about the arc, so they reach a metre and not the
    /// kilometre to the circle's centre, a point outside the region
    /// consumed: on an axis across its plane through the arc's middle,
    /// and on an axis in its plane through the circle's centre, whose
    /// foot of the centre levers about 1000 m. Red against a conic's
    /// pivot read at the foot of its centre.
    #[test]
    fn a_short_arc_far_from_its_centre_levers_from_the_arc() {
        let r = 1000.0;
        let arc = Curve3::circle(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            r,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let reach = Reach::of_span(&arc, (0.0, 1.0 / r)).expect("the span is covered");
        let ball = reach.ball();
        assert!(
            ball.radius() < 1.01,
            "the arc's ball reaches {}",
            ball.radius()
        );
        for (what, axis) in [
            (
                "across its plane through its middle",
                Line {
                    origin: Point3::new(r, 0.5, 0.0),
                    dir: geom::stated_direction(Vec3::new(0.0, 0.0, 1.0)),
                },
            ),
            (
                "in its plane through the circle's centre",
                Line {
                    origin: Point3::origin(),
                    dir: geom::stated_direction(Vec3::new(1.0, 0.0, 0.0)),
                },
            ),
        ] {
            let (_, lever) = reach.foot_lever(axis);
            assert!(
                lever.length() < 1.1,
                "{what}: the foot's lever reaches {}, past the arc",
                lever.length()
            );
        }
    }

    /// The pivot on `axis` and its lever, read at `reach`.
    fn pivot_and_lever(reach: &Reach<f64>, axis: Line<f64>) -> (Point3<f64>, f64) {
        let (foot, lever) = reach.foot_lever(axis);
        (foot, lever.length())
    }

    /// **A spline's sub-span is read at its own window, not its whole
    /// net's**: a straight quadratic standing 1 off the axis from `z = 0`
    /// to `z = 10`, read over its first tenth (`z ∈ [0, 1]`). The pivot
    /// is the window's middle, `z = 0.5`, and its lever `√1.25`, the
    /// least any axis point gives a region at one distance from the
    /// axis. Red against a pivot read off the whole control net, which
    /// stands at `z = 5` and levers `√26`.
    #[test]
    fn a_splines_sub_span_is_read_at_its_own_window() {
        use geom_core::KnotVector;
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control = vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 5.0),
            Point3::new(1.0, 0.0, 10.0),
        ];
        let spline = Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(knots, control, vec![1.0; 3]).unwrap(),
        ));
        let reach = Reach::of_span(&spline, (0.0, 0.1)).expect("the span is covered");
        let axis = Line {
            origin: Point3::new(0.0, 0.0, -3.0),
            dir: geom::stated_direction(Vec3::new(0.0, 0.0, 1.0)),
        };
        let (foot, lever) = pivot_and_lever(&reach, axis);
        assert!((foot.z - 0.5).abs() < 1e-12, "the pivot stands at {foot:?}");
        assert!(
            (lever - 1.25_f64.sqrt()).abs() < 1e-12,
            "the sub-span's lever is {lever}"
        );
    }

    /// **A region at unequal distances from the axis is read at its
    /// window's middle**, a pinned value: a polyline through points
    /// 0.2, 1.0 and 1.8 off the axis at `z = 0, 1, 2`. The pivot is
    /// `z = 1` and its lever `√(1.8² + 1)` ≈ 2.059, 13.7% over the least
    /// any axis point gives (≈ 1.811, at `z = 1.8`) and under the `√2`
    /// bound. Pinned at the midpoint's value, so a pivot that tightens
    /// it is a visible change here. Red against any other rule for the
    /// pivot: the least-lever point, a control point's foot, the net's
    /// mean.
    #[test]
    fn a_region_at_unequal_distances_is_read_at_its_windows_middle() {
        use geom_core::KnotVector;
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
        let control = vec![
            Point3::new(0.2, 0.0, 0.0),
            Point3::new(0.0, 1.0, 1.0),
            Point3::new(1.8, 0.0, 2.0),
        ];
        let polyline = Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(knots, control, vec![1.0; 3]).unwrap(),
        ));
        let reach = Reach::of_span(&polyline, (0.0, 1.0)).expect("the span is covered");
        let axis = Line {
            origin: Point3::new(0.0, 0.0, 5.0),
            dir: geom::stated_direction(Vec3::new(0.0, 0.0, 1.0)),
        };
        let (foot, lever) = pivot_and_lever(&reach, axis);
        let want = (1.8_f64.powi(2) + 1.0).sqrt();
        let least = (0.2_f64.powi(2) + 1.8_f64.powi(2)).sqrt();
        assert!((foot.z - 1.0).abs() < 1e-12, "the pivot stands at {foot:?}");
        assert!(
            (lever - want).abs() < 1e-12,
            "the lever is {lever}, not the midpoint's {want}"
        );
        assert!(
            lever > least && lever <= least * 2_f64.sqrt(),
            "{lever} against the least {least}"
        );
    }

    /// **A rigid turn of the frame turns the pivot with it**: a spline
    /// region and an axis read in a frame turned 0.7 rad about a skew
    /// line give the turned pivot and the same lever. Red against a pivot
    /// read at the foot of the cover's ball centre, whose box is the
    /// world frame's.
    #[test]
    fn a_rigid_turn_does_not_move_the_pivot() {
        use geom_core::KnotVector;
        let (c, s) = (0.7_f64.cos(), 0.7_f64.sin());
        let k = Vec3::new(1.0, 2.0, 2.0) / 3.0;
        let at = Point3::new(0.3, -1.1, 0.4);
        // Rodrigues' turn by 0.7 rad about `k` through `at`.
        let turn_v = |v: Vec3<f64>| v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c));
        let turn = |p: Point3<f64>| at + turn_v(p - at);
        let control = [
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(2.0, 1.5, 1.0),
            Point3::new(0.5, 3.0, 2.5),
            Point3::new(1.5, 0.5, 4.0),
        ];
        let spline = |net: Vec<Point3<f64>>| {
            let knots =
                KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
            Curve3::Nurbs(std::sync::Arc::new(
                geom::NurbsCurve3::new(knots, net, vec![1.0; 4]).unwrap(),
            ))
        };
        let axis = Line {
            origin: Point3::new(-0.5, 0.2, 0.0),
            dir: geom_core::UnitVec3::squared(Vec3::new(0.1, 0.3, 1.0)),
        };
        let here = Reach::of_span(&spline(control.to_vec()), (0.2, 0.9)).expect("covered");
        let there =
            Reach::of_span(&spline(control.map(turn).to_vec()), (0.2, 0.9)).expect("covered");
        let (foot, lever) = pivot_and_lever(&here, axis);
        let turned_axis = Line {
            origin: turn(axis.origin),
            dir: geom::stated_direction(turn_v(axis.dir.get())),
        };
        let (turned_foot, turned_lever) = pivot_and_lever(&there, turned_axis);
        assert!(
            (turned_foot - turn(foot)).norm() < 1e-12,
            "the pivot moves by {:e}",
            (turned_foot - turn(foot)).norm()
        );
        assert!(
            (turned_lever - lever).abs() < 1e-12,
            "the lever moves from {lever} to {turned_lever}"
        );
    }

    /// **A parallel pair's gap is read at one station**: axes 3 apart,
    /// read over a segment from the first to the second, 4.5 up it. Each
    /// foot's distance to the other line is 3, whichever operand is
    /// first. Red against the gap read foot to foot at each axis's own
    /// least-lever point, which split the feet 2 apart along the axes and
    /// read 3.61.
    #[test]
    fn a_parallel_pairs_gap_is_read_at_one_station() {
        let (chord, len) = Line::chord(Point3::new(0.0, 0.0, 0.0), Point3::new(3.0, 0.0, 4.5));
        let segment = Curve3::Line(chord);
        let reach = Reach::of_span(&segment, (0.0, len)).expect("the span is covered");
        let z = Vec3::new(0.0, 0.0, 1.0);
        let l1 = Line {
            origin: Point3::new(0.0, 0.0, -7.0),
            dir: geom::stated_direction(z),
        };
        let l2 = Line {
            origin: Point3::new(3.0, 0.0, 11.0),
            dir: geom::stated_direction(z),
        };
        for (what, (a, b)) in [("in order", (l1, l2)), ("swapped", (l2, l1))] {
            let axes = crate::intersect::parallel_axes_at(&reach, a, b);
            assert!((axes.d - 3.0).abs() < 1e-12, "{what}: d = {}", axes.d);
            assert!(
                (axes.d_vec.norm() - 3.0).abs() < 1e-12
                    && axes.d_vec.dot(a.dir.get()).abs() < 1e-12,
                "{what}: d_vec = {:?}",
                axes.d_vec
            );
        }
    }

    /// **A near-parallel pair's gap is the same in either operand
    /// order, and holds no axial part**: axes 3 apart at `z = 0`, the
    /// second tilted 0.02 rad, read over an arc far up them, where the
    /// two feet stand at different stations. `d` is the greater of each
    /// foot's distance to the other line, in either order. Red against
    /// `d` read from the first foot alone (it moves with the order) and
    /// against `d` read foot to foot (it picks up the feet's axial
    /// offset).
    #[test]
    fn a_near_parallel_pairs_gap_does_not_depend_on_operand_order() {
        let arc = Curve3::circle(
            Point3::new(1.5, 0.0, 40.0),
            Vec3::new(1.0, 0.0, 0.0),
            2.0,
            Vec3::new(0.0, 1.0, 0.0),
        );
        let reach = Reach::of_span(&arc, (0.0, 1.0)).expect("the span is covered");
        let l1 = Line {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: geom::stated_direction(Vec3::new(0.0, 0.0, 1.0)),
        };
        let l2 = Line {
            origin: Point3::new(3.0, 0.0, 0.0),
            dir: geom::stated_direction(Vec3::new(0.02_f64.sin(), 0.0, 0.02_f64.cos())),
        };
        let (f1, f2) = (reach.foot_on(l1), reach.foot_on(l2));
        let want = l2.distance(f1).max(l1.distance(f2));
        let ab = crate::intersect::parallel_axes_at(&reach, l1, l2).d;
        let ba = crate::intersect::parallel_axes_at(&reach, l2, l1).d;
        assert_eq!(ab, ba, "d moves with the operand order");
        assert!((ab - want).abs() < 1e-12, "d = {ab}, not {want}");
        let foot_to_foot = (f2 - f1).norm();
        assert!(
            foot_to_foot - want > 1e-6,
            "the fixture's feet stand at one station: {foot_to_foot} against {want}"
        );
        assert!(
            (l2.distance(f1) - l1.distance(f2)).abs() > 1e-6,
            "the fixture's two distances agree, so the order cannot show"
        );
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
        assert_eq!(square.lever_from(square.center()).length(), 2.0f64.sqrt());
        let mut parts = corners.to_vec();
        parts.push(ExtentBall::new(Point3::new(6.0, 1.0, 0.0), 1.0));
        let both = ExtentBall::enclosing(&parts).unwrap();
        assert_eq!(xyz(both.center()), [2.0, 1.0, 0.0]);
        assert_eq!(both.lever_from(both.center()).length(), 5.0);
        assert!(ExtentBall::<f64>::enclosing(&[]).is_none());
        let slab = ExtentBall::of_box(Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 2.0, 0.0));
        assert_eq!(xyz(slab.center()), xyz(square.center()));
        assert_eq!(slab.lever_from(slab.center()).length(), 2.0f64.sqrt());
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod geometry_door_rows {
    use super::*;

    const X: Vec3<f64> = Vec3::new(1.0, 0.0, 0.0);
    const Y: Vec3<f64> = Vec3::new(0.0, 1.0, 0.0);
    const Z: Vec3<f64> = Vec3::new(0.0, 0.0, 1.0);

    fn segment(a: Point3<f64>, b: Point3<f64>) -> Reach<f64> {
        Reach::Region(Cover::segment(a, b).expect("a segment's cover"))
    }

    /// **A ball's lever from two pivots is its reach from the farther**,
    /// whichever is named first: the unit ball about the origin stands 4
    /// from `(3, 0, 0)` and 6 from `(0, 5, 0)`. Red against the reach from
    /// the first pivot alone (4), against the lesser of the two (4), and
    /// against the first pivot's reach widened by the pivots' distance
    /// (`4 + √34`).
    #[test]
    fn a_ball_levered_from_two_pivots_reaches_from_the_farther() {
        let ball = ExtentBall::new(Point3::origin(), 1.0);
        let (p, q) = (Point3::new(3.0, 0.0, 0.0), Point3::new(0.0, 5.0, 0.0));
        assert_eq!(ball.lever_from_either(p, q).length(), 6.0);
        assert_eq!(ball.lever_from_either(q, p).length(), 6.0);
        assert_eq!(
            ball.lever_from_either(p, p).length(),
            ball.lever_from(p).length(),
            "one pivot named twice is that pivot's lever"
        );
    }

    /// **The plane×cylinder row's pivot is the reach's own on the axis,
    /// and its lever is read from there.** The plane `z = 0`, a cylinder
    /// whose axis runs along `(cos β, 0, sin β)` through a stored origin
    /// 100 m back along it, and an edge `[4, 6]` along the axis: the
    /// pivot is the edge's middle, wherever the stored origin sits, and
    /// the lever is the edge's half-length plus the hinge's shift
    /// `|gap·c|` (`gap` the pivot's height, `c = sin β`). Red against a
    /// pivot at the stored origin (a lever of about 106), and against the
    /// shift dropped.
    #[test]
    fn the_ruled_rows_lever_is_read_at_the_reachs_pivot_on_the_axis() {
        let beta = 0.3_f64;
        let a = Vec3::new(beta.cos(), 0.0, beta.sin());
        let on_axis = |s: f64| Point3::new(1.0, 2.0, 0.5) + a * s;
        let pc = crate::intersect::PlaneCylinder {
            q: Point3::origin(),
            n: Z,
            o: on_axis(-100.0),
            a: geom::stated_direction(a),
            r: 1.0,
        };
        let reach = segment(on_axis(4.0), on_axis(6.0));
        let frame = reach.ruled_pivot_lever(&pc);
        let (pivot, lever) = (frame.o, frame.lever);
        let middle = on_axis(5.0);
        assert!((pivot - middle).norm() < 1e-12, "{pivot:?}");
        let gap = middle.z;
        assert!((frame.gap_signed - gap).abs() < 1e-12 && (frame.c - beta.sin()).abs() < 1e-15);
        let want = 1.0 + (gap * beta.sin()).abs();
        assert!(
            (lever.length() - want).abs() < 1e-12,
            "lever {}, the half-length and the hinge's shift {want}",
            lever.length()
        );
    }

    /// **A face's ruled lever is read from the rulings' hinge, turn
    /// included.** The plane `z = 0` and an axis leaning `β` out of it;
    /// a face measured from a point 0.7 off the axis along `y`, at the
    /// station 5: 1.5 back and 2.5 on along the axis, 3 at its farthest.
    /// The pivot is the point's foot on the axis, `gap` above the plane;
    /// the hinge stands `−gap·sin β` along the axis from it, so the face
    /// reaches `2.5 + gap·sin β` from the hinge's station; and the point
    /// stands `gap·cos β` across the wall from the hinge, so the turn
    /// adds `(3 + gap·cos β)·sin β/(1 + cos β)`. Red against the turn
    /// dropped, the shift's sign, and the hinge left at the foot.
    #[test]
    fn a_faces_ruled_lever_is_read_from_the_hinge_with_its_turn() {
        let beta = 0.3_f64;
        let (sin, cos) = beta.sin_cos();
        let a = Vec3::new(cos, 0.0, sin);
        let on_axis = |s: f64| Point3::new(1.0, 2.0, 0.5) + a * s;
        let pc = crate::intersect::PlaneCylinder {
            q: Point3::origin(),
            n: Z,
            o: on_axis(-100.0),
            a: geom::stated_direction(a),
            r: 1.0,
        };
        let face = Reach::of_face(
            on_axis(5.0) + Y * 0.7,
            (1.5, 2.5),
            ExtentBall::new(Point3::origin(), 3.0).lever(),
        );
        let frame = face.ruled_pivot_lever(&pc);
        assert!((frame.o - on_axis(5.0)).norm() < 1e-12, "{:?}", frame.o);
        let gap = on_axis(5.0).z;
        let want = 2.5 + gap * sin + (3.0 + gap * cos) * sin / (1.0 + cos);
        assert!(
            (frame.lever.length() - want).abs() < 1e-12,
            "lever {}, from the hinge with its turn {want}",
            frame.lever.length()
        );
    }

    /// **A cone's generator lever is read from its apex along its own
    /// axis**: a cone with its apex at `(0, 0, 10)` opening along `−z` at
    /// half-angle 0.4, and a reach over the heights `z ∈ [1, 3]`, whose
    /// consumed points stand 7 to 9 along the axis from the apex, so no
    /// farther than `9/cos 0.4` along a generator. Red against a window
    /// read from the world origin (`3/cos`), against the window's near
    /// end (`7/cos`), and against the cosine dropped.
    #[test]
    fn a_cones_generator_lever_is_read_from_its_apex() {
        let axis = Line {
            origin: Point3::new(0.0, 0.0, 10.0),
            dir: geom::stated_direction(-Z),
        };
        let reach = segment(Point3::new(0.5, 0.0, 1.0), Point3::new(0.5, 0.0, 3.0));
        let read = reach.generator_lever(axis, 0.4);
        let want = 9.0 / 0.4_f64.cos();
        let lever = read.lever.length();
        assert!((lever - want).abs() < 1e-12, "{lever} against {want}");
        let (lo, hi) = read.window;
        assert!(
            (lo - 7.0).abs() < 1e-12 && (hi - 9.0).abs() < 1e-12,
            "{lo}, {hi}"
        );
        assert_eq!(read.sin_cos, 0.4_f64.sin_cos());
    }

    /// **Two crossing cylinders' ball is the pair's, whatever reach it is
    /// read across.** Radius 1 along `x` through the origin and radius 2
    /// along `y` through `(0, 0, 3)`: the common perpendicular runs 3 up
    /// `z` from the origin, so the ball is centred at `(0, 0, 1.5)` out
    /// to `√(3² + 2.5²)`. Read across an edge near the crossing and one
    /// 40 m along the first axis, it is the same ball. Red against the
    /// perpendicular's foot left at the reach's pivot (a centre 40 m
    /// off), and against the radii's larger for their lesser along the
    /// perpendicular.
    #[test]
    fn two_crossing_cylinders_ball_does_not_move_with_the_reach() {
        let axes = [
            Line {
                origin: Point3::new(-7.0, 0.0, 0.0),
                dir: geom::stated_direction(X),
            },
            Line {
                origin: Point3::new(0.0, 9.0, 3.0),
                dir: geom::stated_direction(Y),
            },
        ];
        let want_r = (3.0_f64.powi(2) + 2.5_f64.powi(2)).sqrt();
        for reach in [
            segment(Point3::new(-1.0, 0.0, 1.0), Point3::new(1.0, 1.0, 2.0)),
            segment(Point3::new(40.0, 0.0, 1.0), Point3::new(41.0, 1.0, 1.0)),
        ] {
            let (perp, ball) = ExtentBall::of_crossing_cylinders(axes, [1.0, 2.0], &reach);
            assert!(
                (ball.center() - Point3::new(0.0, 0.0, 1.5)).norm() < 1e-12,
                "{:?}",
                ball.center()
            );
            assert!((ball.radius() - want_r).abs() < 1e-12, "{}", ball.radius());
            assert!((perp.foot - Point3::origin()).norm() < 1e-12);
            assert!((perp.gap - 3.0).abs() < 1e-12 && (perp.sin - 1.0).abs() < 1e-12);
        }
    }

    /// **Several edges' extent is read from below.** Three short posts,
    /// one standing 2 m out along each axis: no two of their points are
    /// farther apart than the far corners of two posts, about `2√2`,
    /// where the box about them is `2√3` corner to corner. The reading
    /// is never past the farthest pair and is most of it. Red against
    /// the box's diagonal (a bound from above), against the edges' own
    /// extents alone (0.1), and against one width in place of the
    /// widest. No edge reads zero.
    #[test]
    fn several_edges_extent_never_passes_their_farthest_pair() {
        let post = |foot: Point3<f64>, up: Vec3<f64>| {
            Curve3::Line(Line {
                origin: foot,
                dir: geom::stated_direction(up),
            })
        };
        let posts = [
            post(Point3::new(2.0, 0.0, 0.0), Y),
            post(Point3::new(0.0, 2.0, 0.0), Z),
            post(Point3::new(0.0, 0.0, 2.0), X),
        ];
        let ends: Vec<Point3<f64>> = posts
            .iter()
            .flat_map(|c| [c.eval(0.0), c.eval(0.1)])
            .collect();
        let farthest = farthest_pair(&ends);
        let read = edges_extent_inf(posts.iter().map(|c| (c, (0.0, 0.1)))).length();
        assert!(read <= farthest * (1.0 + 1e-12), "{read} past {farthest}");
        assert!(read >= 0.95 * farthest, "{read} well short of {farthest}");
        assert_eq!(edges_extent_inf::<f64>([]).length(), 0.0);
    }

    /// **A cover's diameter is read from above**: a segment's box
    /// diagonal is its length, and a unit sphere's, `2√3`, is no shorter
    /// than the 2 its farthest two points stand apart. Red against the
    /// cover's ball's radius (1) and against one box side (2 on the
    /// segment `(0,0,0)–(1,2,2)`, whose length is 3).
    #[test]
    fn a_covers_diameter_is_its_boxs_diagonal() {
        let seg = Cover::segment(Point3::origin(), Point3::new(1.0, 2.0, 2.0)).unwrap();
        assert!((cover_diameter(&seg).length() - 3.0).abs() < 1e-12);
        let sphere =
            Cover::sphere(&geom::Frame::new(Point3::new(5.0, 5.0, 5.0), Z, X), 1.0).unwrap();
        let d = cover_diameter(&sphere).length();
        assert!((d - 2.0 * 3.0_f64.sqrt()).abs() < 1e-9, "{d}");
    }
}
