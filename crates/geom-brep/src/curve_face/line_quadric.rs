//! **The line × quadric door** ([`super::Door::LineRoots`]): the method
//! of a line against a cylinder wall, a cone, a sphere or a torus — the
//! roots of the surface's residual along the line, once the pose
//! (`super::pose`) has decided the class: a wall's and a sphere's
//! quadratic roots, the cone's quadratic read about the line's nearest
//! point to its apex, each root metered, and the torus's quartic ladder
//! ([`line_torus_roots`]).
//!
//! Its older entries read the pose and then the method, deciding
//! nothing of their own: the sweep's `line_quadric_roots` (the door rows' reader), in the root
//! doors' shape ([`super::circle_roots::CircleRoots`]), refusing in its
//! own words ([`LineQuadricRefusal`], the arm that refused, which the
//! caller names a decision of its own), and the point-in-solid ray's
//! wall ([`line_wall_roots`]), sphere ([`line_sphere_roots`]) and cone
//! ([`line_cone_roots`]).

use super::circle_roots::{CircleRoots, DoorRefusal};
use crate::curve_pair::MeetFault;
use crate::quartic::{QuarticRoots, depressed_quartic_roots, quadratic_roots};
use crate::roots::{RootRefusal, Roots, Unsettled};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

/// **Why `line_quadric_roots` (the door rows' reader) refused**: the arm, and what its finder
/// refused with. The wall, sphere and cone arms only escalate; the torus
/// arm reads [`RootRefusal`], whose fault is a kernel defect
/// ([`DoorRefusal::Invariant`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineQuadricRefusal {
    /// A wall's rung in band ([`line_wall_roots`]), with the rung.
    Wall(WallRootFault),
    /// The sphere's discriminant in band ([`line_sphere_roots`]).
    Sphere(Indeterminate),
    /// The torus's quartic ladder ([`line_torus_roots`]).
    Torus(DoorRefusal),
    /// A cone rung in band (the edge × cone quadratic over the run).
    Cone(Indeterminate),
}

impl LineQuadricRefusal {
    /// The refusal without its arm, for a reader that names no decision
    /// per arm, in any door's refusal shape.
    #[must_use]
    pub const fn door_refusal<X>(self) -> DoorRefusal<X> {
        match self {
            Self::Wall(WallRootFault { diag, .. }) | Self::Sphere(diag) | Self::Cone(diag) => {
                DoorRefusal::Escalated(diag)
            }
            Self::Torus(DoorRefusal::Escalated(diag)) => DoorRefusal::Escalated(diag),
            Self::Torus(DoorRefusal::Invariant(what)) => DoorRefusal::Invariant(what),
        }
    }
}

/// **The certified roots of the LINE against `surface`** over the run
/// `(t0, t1)` of the line's parameter, in the root doors' shape: the
/// door rows' reading, a test reader since the sweep reads [`super::cross()`]: the pose
/// ([`super::pose()`]), then the arm's method ([`line_method`]), mapped
/// back. The pose's `Lies` (a ruling of a wall, a generator of a cone) is
/// [`CircleRoots::On`]; a line parallel to a wall's axis off it, or any
/// other `Apart`, is [`CircleRoots::Miss`]; a graze is
/// [`Unsettled::Tangency`], its curvature in band too; a line through a
/// cone's apex is the apex's double root; an apex in band is
/// [`Unsettled::AtApex`]. `None` for a surface with no line root lane.
///
/// # Errors
///
/// [`LineQuadricRefusal`], the arm with the pose's or the method's
/// escalation.
#[cfg(test)]
pub fn line_quadric_roots<T: Decide>(
    line: geom::Line<T>,
    (t0, t1): (T, T),
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Option<Result<CircleRoots<T>, Unsettled>>, LineQuadricRefusal> {
    use super::pose::{Pose, PoseRefusal, PoseRung};
    let arm = |diag: Indeterminate, rung: PoseRung| match *surface {
        geom::Surface::Cylinder { .. } => LineQuadricRefusal::Wall(WallRootFault {
            rung: if rung == PoseRung::AxisParallel {
                WallRung::AxisParallel
            } else {
                WallRung::Discriminant
            },
            diag,
        }),
        geom::Surface::Sphere { .. } => LineQuadricRefusal::Sphere(diag),
        geom::Surface::Torus { .. } => LineQuadricRefusal::Torus(DoorRefusal::Escalated(diag)),
        _ => LineQuadricRefusal::Cone(diag),
    };
    if !matches!(
        surface,
        geom::Surface::Cylinder { .. }
            | geom::Surface::Sphere { .. }
            | geom::Surface::Cone { .. }
            | geom::Surface::Torus { .. }
    ) {
        return Ok(None);
    }
    let carrier = geom::Curve3::Line(line);
    let span = crate::curve_pair::Span::new(&carrier, (t0, t1));
    let pose = match super::pose::pose(&span, surface, band).map_err(|u| u.why) {
        Ok(pose) => pose,
        Err(refusal) if let Some(why) = refusal.unsettled() => return Ok(Some(Err(why))),
        // An apex spread the band cannot call is the apex's, refused
        // typed, never escalated.
        Err(
            PoseRefusal::InBand {
                rung: PoseRung::ApexSpread,
                ..
            }
            | PoseRefusal::Meter {
                rung: PoseRung::ApexSpread,
                ..
            },
        ) => return Ok(Some(Err(Unsettled::AtApex))),
        Err(PoseRefusal::InBand { rung, diag }) => return Err(arm(diag, rung)),
        Err(PoseRefusal::Meter { row, .. }) => return Ok(Some(Err(Unsettled::Meter { row }))),
        // A kernel fault fails loud, never "not this door's".
        Err(PoseRefusal::Fault(what)) => {
            return Err(LineQuadricRefusal::Torus(DoorRefusal::Invariant(what)));
        }
        Err(PoseRefusal::Enclosed | PoseRefusal::Unbuilt(_) | PoseRefusal::Torus(_)) => {
            return Ok(None);
        }
    };
    Ok(Some(match pose {
        Pose::Lies => Ok(CircleRoots::On(crate::roots::OnCarrier)),
        Pose::Apart => Ok(CircleRoots::Miss),
        Pose::Tangent(_) => Err(Unsettled::Tangency),
        Pose::Through(apex) => Ok(Roots::<T, 2>::all([apex.at, apex.at]).widen()),
        Pose::Isolated(form) => {
            let read = line_method(line, (t0, t1), surface, form.0, band)
                .and_then(|walked| {
                    match super::circle_roots::door_shape::<_, core::convert::Infallible>(
                        &span, &walked, band,
                    ) {
                        Ok(found) => Ok(Ok(found)),
                        Err(DoorRefusal::Escalated(diag)) => Ok(Err(diag)),
                        Err(DoorRefusal::Invariant(what)) => Err(what),
                        Err(DoorRefusal::Torus(never)) => match never {},
                    }
                })
                .map_err(|what| LineQuadricRefusal::Torus(DoorRefusal::Invariant(what)))?;
            match read {
                Ok(found) => found,
                Err(diag) => return Err(arm(diag, PoseRung::Discriminant)),
            }
        }
    }))
}

/// **The line arm's method**: the roots of an [`super::pose::Pose::Isolated`]
/// line, from what its pose computed. A wall's or a sphere's two roots
/// are the pose's own; a cone's are its quadratic's, each metered, a root
/// whose slack the meter refuses a window over the root ± its slack
/// widened by `Kε` ([`cone_roots_read`]); a torus's are its quartic
/// ladder's ([`line_torus_roots`]), whose refusals are the whole span's.
///
/// # Errors
///
/// A kernel fault's readings.
pub(super) fn line_method<T: Decide>(
    line: geom::Line<T>,
    (t0, t1): (T, T),
    surface: &geom::Surface<T>,
    form: super::pose::Formed<T>,
    band: Band,
) -> Result<super::cross::Walked<T>, &'static str> {
    use super::cross::Walked;
    use super::pose::Formed;
    match (form, surface.clone()) {
        (Formed::LineQuadratic { roots }, _) => Ok(Walked::found(roots.to_vec())),
        (Formed::LineCone { parallel: true }, _) => Ok(Walked::whole(
            (t0, t1),
            MeetFault::Unsettled(Unsettled::Degenerate),
        )),
        (
            Formed::LineCone { parallel: false },
            geom::Surface::Cone {
                frame: cone_frame,
                half_angle,
                ..
            },
        ) => {
            let (apex, axis) = (cone_frame.origin(), cone_frame.axis_unit());
            let cone = geom::Line {
                origin: apex,
                dir: axis,
            };
            let mut walked = Walked::found(Vec::new());
            for (t, slack, held) in cone_roots_read(line, (cone, half_angle), (t0, t1), band) {
                if held {
                    walked.roots.push(t);
                } else {
                    walked.windows.push((
                        (t - slack, t + slack),
                        MeetFault::Unsettled(Unsettled::Meter {
                            row: "line_quadric_cone_root_slack",
                        }),
                    ));
                }
            }
            Ok(walked)
        }
        (
            Formed::LineTorus,
            geom::Surface::Torus {
                frame: torus_frame,
                major_radius,
                minor_radius,
                ..
            },
        ) => {
            let (center, axis) = (torus_frame.origin(), torus_frame.axis_unit());
            let found = line_torus_roots(
                line,
                geom::Line {
                    origin: center,
                    dir: axis,
                },
                major_radius,
                minor_radius,
                band,
            );
            Walked::of_core((t0, t1), found.map(|f| -> CircleRoots<T> { f.widen() }))
        }
        _ => Err("the line arm's method was handed a form of another arm"),
    }
}

/// What [`line_wall_roots`] found. The three non-root answers are kept
/// APART rather than collapsed into one "no roots": each caller owes a
/// different thing to each of them — a ray retries a tangent and skips
/// a miss, an edge sweep refuses a tangent and clears a miss — and a
/// merged variant would let one caller silently inherit the other's
/// posture.
#[derive(Debug, Clone, Copy)]
pub enum WallRoots<T> {
    /// The line is parallel to the axis: its residual is constant, so
    /// the quadratic degenerates and there is no root to report.
    AxisParallel,
    /// The discriminant is in the zero band — the line is TANGENT to
    /// the wall, and a tangency is not a crossing at any order this
    /// function can see.
    Tangent,
    /// A definitely negative discriminant: no real root.
    Miss,
    /// Two definite roots of the carrier's own parameter, ascending,
    /// and a bound on either's rounding in that parameter
    /// ([`Centred::roots`]).
    Two([T; 2], T),
}

/// **Where a line × quadric quadratic is read from** ([`Centred`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum About {
    /// The line's own origin, for a line given at the body's own scale
    /// (an edge's carrier line): its residual there is already of that
    /// scale, and the move buys nothing. It costs: at `Interval` the
    /// shift is itself an enclosure, and the moved offset `w₀ + d·shift`
    /// and the root `s + shift` each carry its width, so a root read
    /// about the shift is wider than one read where the line starts. The
    /// edge door reads a wall and a sphere from here.
    Start,
    /// The line's closest approach to the quadric's axis, apex or
    /// centre, held to the run its caller reads where it names one (a
    /// wall's run either side of the start, a cone's span): where the
    /// residual over the run is read at the run's own scale, for a ray
    /// whose line's origin may lie far from the quadric and for every
    /// reading of a cone ([`line_cone_roots`]).
    Nearest,
}

/// The certified roots of the LINE `q + d·t` against the INFINITE
/// cylinder wall `(origin, axis, radius)` — the quadratic in metres
/// that the ray lane has solved since M5 PR 9, factored out so the
/// edge-sweep lane solves the same one.
///
/// **Roots, not hits.** Trimming to a face, ordering by advance and
/// counting them are the RAY's concerns and stay in
/// the ray's `cast_ray`; a line-edge span needs both roots, unordered by sign,
/// and folding them here would have made this the ray's function with
/// a second caller rather than a shared primitive.
///
/// **Both trileans are lengths.**
///
/// - **`curve_face_pose_axis_parallel`**, the axis-parallel rung: `|d⊥|`,
///   the line's speed off the axis, levered by `span`, the run of the
///   line's own parameter over which the caller reads roots. Their
///   product is how far the line drifts off its distance from the axis
///   over that run. Zero ⇒ it drifts less than the band, so the residual
///   is constant there and no root lands in the run.
/// - **`curve_face_pose_disc`**: the discriminant over `|d⊥|²` is
///   the squared half-chord in the plane across the axis, `r² − p²` for
///   a line passing `p` from it, and over `2r` it is the depth the line
///   reaches inside the circle, the sphere arm's form
///   ([`line_sphere_roots`]). Positive ⇒ two definite roots; Zero ⇒
///   tangent; Negative ⇒ definite miss.
///
/// # Errors
///
/// [`WallRootFault`] — an in-band axis-parallel test or an in-band
/// discriminant, with the rung that escalated. The caller wraps it in
/// its own error type.
#[allow(clippy::too_many_arguments)] // the line, the wall, the run and the band, each named
pub fn line_wall_roots<T: Decide>(
    line: geom::Line<T>,
    wall: geom::Line<T>,
    radius: T,
    span: T,
    band: Band,
    about: About,
) -> Result<WallRoots<T>, WallRootFault> {
    use super::pose::LineQuadric;
    match super::pose::line_wall_read(line, (wall, radius), (T::zero(), span), about, band) {
        Ok(LineQuadric::Parallel { .. }) => Ok(WallRoots::AxisParallel),
        Ok(LineQuadric::Apart) => Ok(WallRoots::Miss),
        Ok(LineQuadric::Graze { .. }) => Ok(WallRoots::Tangent),
        Ok(LineQuadric::Two(roots, slack)) => Ok(WallRoots::Two(roots, slack)),
        Ok(LineQuadric::Undecided { diag, .. }) => Err(WallRootFault {
            rung: WallRung::Discriminant,
            diag,
        }),
        Err(diag) => Err(WallRootFault {
            rung: WallRung::AxisParallel,
            diag,
        }),
    }
}

/// **The line × wall quadratic** `|d⊥|²s² + 2(w⊥·d⊥)s + (|w⊥|² − r²)`,
/// read `about` the line's start or its closest approach to the axis,
/// held to the run `span` ([`Centred`]).
pub(super) fn wall_quadratic<T: Decide>(
    line: geom::Line<T>,
    (wall, radius): (geom::Line<T>, T),
    span: T,
    about: About,
) -> Centred<T> {
    let geom::Line { origin: q, dir: d } = line;
    let geom::Line { origin, dir: axis } = wall;
    let w0 = q - origin;
    let w0p = wall.offset(q);
    let dp = d.get() - axis.get() * d.get().dot(axis.get());
    let a2 = dp.norm_squared();
    let Moved {
        shift,
        slope,
        square,
    } = match about {
        About::Start => Moved::at_start(w0p, dp),
        About::Nearest => Moved::nearest(w0p, dp, Some((T::zero() - span, span))),
    };
    let w = w0p + dp * shift;
    Centred::new(
        (w0, d.get(), shift, w),
        [a2, slope, square - radius.powi(2)],
        radius,
    )
}

/// **An offset `w₀` moved along `d` by `shift`**, as a quadratic read
/// about the moved point needs it: `w·d` and `|w|²` for `w = w₀ + d·shift`.
struct Moved<T> {
    shift: T,
    /// `w·d`.
    slope: T,
    /// `|w|²`.
    square: T,
}

impl<T: Decide> Moved<T> {
    /// Not moved: both read off `w₀` itself.
    fn at_start(w0: Vec3<T>, d: Vec3<T>) -> Self {
        Self {
            shift: T::zero(),
            slope: w0.dot(d),
            square: w0.norm_squared(),
        }
    }

    /// **Moved to the closest approach to `w₀`'s origin, held to `run`**
    /// where there is one. With `w_free` the approach, square to `d`, and
    /// the run moving it by `held` along `d`, `w = w_free + d·held`, so
    /// `w·d = |d|²·held` and `|w|² = (|w₀ × d|² + (w·d)²)/|d|²`: each the
    /// fact, read off `w₀` once, rather than recomputed from a `w` that
    /// counts `w₀` twice (directly and through the shift). With `g = w₀·d`,
    /// `|d|²·held` is `min(|d|²·hi + g, 0) − min(−g − |d|²·lo, 0)`: zero
    /// wherever the approach `−g/|d|²` is inside the run, and the slope at
    /// the end that holds it back elsewhere, with no division by `|d|²`.
    fn nearest(w0: Vec3<T>, d: Vec3<T>, run: Option<(T, T)>) -> Self {
        let (a2, g) = (d.norm_squared(), w0.dot(d));
        let free = (T::zero() - g) / a2;
        let (shift, slope) = match run {
            Some((lo, hi)) => (
                free.max(lo).min(hi),
                (a2 * hi + g).min(T::zero()) - (T::zero() - g - a2 * lo).min(T::zero()),
            ),
            None => (free, T::zero()),
        };
        Self {
            shift,
            slope,
            square: (w0.cross(d).norm_squared() + slope.powi(2)) / a2,
        }
    }
}

/// **The line × sphere quadratic** `|d|²s² + 2(w·d)s + (|w|² − r²)`,
/// read `about` the line's start or its closest approach to the centre
/// ([`Centred`]).
pub(super) fn sphere_quadratic<T: Decide>(
    line: geom::Line<T>,
    (center, radius): (Point3<T>, T),
    about: About,
) -> Centred<T> {
    let geom::Line { origin: q, dir: d } = line;
    let w0 = q - center;
    let a2 = d.get().norm_squared();
    let Moved {
        shift,
        slope,
        square,
    } = match about {
        About::Start => Moved::at_start(w0, d.get()),
        About::Nearest => Moved::nearest(w0, d.get(), None),
    };
    let w = w0 + d.get() * shift;
    Centred::new(
        (w0, d.get(), shift, w),
        [a2, slope, square - radius.powi(2)],
        radius,
    )
}

/// **A line × quadric quadratic read where the line passes the
/// quadric**: `a·s² + 2b·s + c` in `s = t − shift`, the line's parameter
/// moved to `shift` ([`About`]). Read about the line's start, `c` is the
/// residual there, the square of the start's distance less the body's
/// own squares: a start `D` out loses `u·D²` of it, which moves a root by
/// about `u·D²/(2√disc)`, the body's scale squared over a root's
/// sensitivity (`D = 4·10⁵` m and a unit wall: `10⁻⁵` m). About the
/// closest approach, `c` is read at the body's own scale, and what the
/// start's distance leaves is the rounding of the shift itself, linear
/// in `D` ([`Self::roots`], [`Self::charged`]). The discriminant and the
/// lead are the same quantities either way, so each margin metered on
/// them is.
#[derive(Clone, Copy, Debug)]
pub struct Centred<T> {
    /// Where the parameter is read from.
    pub shift: T,
    /// `[a, b, c]`.
    pub coeffs: [T; 3],
    /// A bound on the lengths the moved residual was formed from:
    /// `|w₀| + |d|·|shift|`, `w₀` the start's offset from the quadric.
    far: T,
    /// The moved offset's length, with the quadric's radius where it
    /// has one: the residual's gradient against the offset is at most
    /// `2·near`, for a wall, a sphere and a cone alike (the cone's,
    /// `2((w·â)â − cos²α·w)`, is at most `2|w|·max(sin²α, cos²α)`).
    near: T,
    /// `|d|`.
    speed: T,
}

impl<T: Decide> Centred<T> {
    /// The quadratic `coeffs`, read `shift` along `d` from a start
    /// offset `w0` from the quadric, at the moved offset `w` (across the
    /// axis, for a wall), of a quadric of `radius` (`0` for a cone).
    fn new((w0, d, shift, w): (Vec3<T>, Vec3<T>, T, Vec3<T>), coeffs: [T; 3], radius: T) -> Self {
        Self {
            shift,
            coeffs,
            far: w0.norm() + d.norm() * shift.abs(),
            near: w.norm() + radius,
            speed: d.norm(),
        }
    }

    /// `b² − a·c`.
    pub fn disc(&self) -> T {
        let [a, b, c] = self.coeffs;
        b.powi(2) - a * c
    }

    /// **`disc` held to its rounding from the shift**, for a decision
    /// on it: moved toward zero by that rounding, and zero within it,
    /// where the line passes too near tangency for its sign to be read.
    /// The residual `c` read about the shift is off by
    /// [`geom_core::rounding_charge`] of `far·near` (as at a
    /// root, [`Self::roots`]), and `b` by that of `far·|d|`, so `disc` by
    /// `|a|` and `2|b|` times them. Read about the start
    /// ([`About::Start`]) it is the plain `disc`.
    pub fn charged(&self, disc: T, about: About) -> T {
        geom_core::Rounded {
            value: disc,
            error: self.disc_error(about),
        }
        .toward_zero()
    }

    /// **`|disc|` with its rounding from the shift added**, a bound above
    /// it ([`Self::charged`]'s charge, the other way).
    pub fn disc_bound(&self, disc: T, about: About) -> T {
        disc.abs() + self.disc_error(about)
    }

    /// The rounding [`Self::charged`] holds `disc` to.
    fn disc_error(&self, about: About) -> T {
        if about == About::Start {
            return T::zero();
        }
        let [a, b, _] = self.coeffs;
        a.abs() * geom_core::rounding_charge(self.far * self.near)
            + T::from_f64(2.0) * b.abs() * geom_core::rounding_charge(self.far * self.speed)
    }

    /// **Both roots in the line's own parameter, and a bound on either's
    /// rounding there**, for a definitely positive `disc`. The residual
    /// at a root `s` is read off offsets of length at most `far + |d|·|s|`
    /// (each rounded by `u` of that), against a gradient of at most
    /// `2·(near + |d|·|s|)`: so it is off by at most
    /// [`geom_core::rounding_charge`] of their product, and the root by
    /// that over the residual's slope there ([`over_root_slope`]).
    /// Moving the root back by `shift` rounds it once more.
    pub fn roots(&self, disc: T) -> ([T; 2], T) {
        let [a, b, c] = self.coeffs;
        let roots = quadratic_roots(a, b, c, disc);
        let slack = self.slack_at(roots[0].abs().max(roots[1].abs()), disc);
        (roots.map(|r| r + self.shift), slack)
    }

    /// **The rounding of a root `s` from the shift**, in the line's
    /// parameter: [`Self::roots`]' bound read at that root alone.
    fn slack_at(&self, s: T, disc: T) -> T {
        let run = self.speed * s.abs();
        let residual = geom_core::rounding_charge((self.far + run) * (self.near + run));
        over_root_slope(residual, disc) + geom_core::rounding_charge(self.shift.abs() + s.abs())
    }
}

/// **A quadratic's root moved by a residual's error**: `error` over the
/// residual's slope at the root, `2√disc` for `a·s² + 2b·s + c`. The one
/// home of a line × quadric root's sensitivity ([`Centred::roots`]).
pub fn over_root_slope<T: Decide>(error: T, disc: T) -> T {
    error / (T::from_f64(2.0) * disc.sqrt())
}

/// **Which rung of [`line_wall_roots`] escalated**: the two questions
/// the line × wall quadratic asks before it has roots. Both margins are
/// lengths ([`line_wall_roots`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "test-support", derive(strum::EnumIter))]
pub enum WallRung {
    /// Whether the line runs parallel to the wall's axis
    /// (`curve_face_pose_axis_parallel`, `|d⊥|` levered by the run the
    /// caller reads): a positive margin has roots to find, a zero one a
    /// residual constant over that run.
    AxisParallel,
    /// Whether the line crosses the wall, grazes it or misses it
    /// (`curve_face_pose_disc`, `disc/|d⊥|²` over `2r`).
    Discriminant,
}

/// An escalation of [`line_wall_roots`], with the rung that raised it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallRootFault {
    /// The rung that could not decide.
    pub rung: WallRung,
    /// Its diagnostics.
    pub diag: Indeterminate,
}

/// **The line × cone quadratic**: the cone's quadric form along the
/// line `q + d·t`, negated, `(w·â)² − |w|²·cos²α = A s² + 2B s + C` with
/// `w = q + d·t − apex`, `s = t − shift` — `A` in `d`'s units squared,
/// `B` in metres times them, `C` in m² — read about the line's closest
/// approach to the apex held to the run `(lo, hi)` ([`About::Nearest`],
/// [`Centred`]), where it is well-conditioned over the run whatever the
/// line's start. Its zero set is the DOUBLE cone; which nappe a root
/// lands on is the caller's question.
/// `A = (d·â)² − |d|²cos²α` vanishes exactly when the line runs parallel
/// to a generator. Each square straddling zero is a `powi(2)`, the tight
/// square.
pub(super) fn line_cone_quadratic<T: Decide>(
    line: geom::Line<T>,
    cone: geom::Line<T>,
    half_angle: T,
    run: (T, T),
) -> Centred<T> {
    let geom::Line { origin: q, dir: d } = line;
    let geom::Line {
        origin: apex,
        dir: axis,
    } = cone;
    let w0 = q - apex;
    let Moved {
        shift,
        slope,
        square,
    } = Moved::nearest(w0, d.get(), Some(run));
    let w = w0 + d.get() * shift;
    let cos2 = half_angle.cos().powi(2);
    let (da, wa) = (d.get().dot(axis.get()), w.dot(axis.get()));
    let coeffs = [
        da.powi(2) - d.get().norm_squared() * cos2,
        da * wa - slope * cos2,
        wa.powi(2) - square * cos2,
    ];
    Centred::new((w0, d.get(), shift, w), coeffs, T::zero())
}

/// The certified roots of the LINE `q + d·t` against the sphere
/// `(center, radius)`: `|d|²t² + 2(w·d)t + (|w|² − r²) = 0`, `w = q − c`.
/// `d` need not be unit — a ray's is, and a `Line` carrier's is by a
/// convention nothing checks — so the roots are in `d`'s own parameter.
///
/// The discriminant is metered as a LENGTH: `disc/|d|²` is the squared
/// half-chord in metres, so `disc/(|d|²·2r)` is the D4 ¶1-honest
/// margin, the form [`line_wall_roots`] takes across the axis too.
///
/// A line is never parallel to a sphere in the sense a wall's ruling
/// is, so [`WallRoots::AxisParallel`] is never answered, and no line
/// lies on a sphere: two distinct roots certify the line is off it.
///
/// # Errors
///
/// [`geom_core::Indeterminate`] — an in-band discriminant decision the
/// band cannot call, or a zero-length `d` (no line at all). The caller
/// wraps it in its own error type.
pub fn line_sphere_roots<T: Decide>(
    line: geom::Line<T>,
    center: Point3<T>,
    radius: T,
    band: Band,
    about: About,
) -> Result<WallRoots<T>, geom_core::Indeterminate> {
    use super::pose::LineQuadric;
    match super::pose::line_sphere_read(line, (center, radius), about, band) {
        LineQuadric::Apart | LineQuadric::Parallel { .. } => Ok(WallRoots::Miss),
        LineQuadric::Graze { .. } => Ok(WallRoots::Tangent),
        LineQuadric::Two(roots, slack) => Ok(WallRoots::Two(roots, slack)),
        LineQuadric::Undecided { diag, .. } => Err(diag),
    }
}

/// The certified real roots of the LINE `q + d·t` (with `d` a UNIT
/// direction) against the torus `(center, axis, R, r)`.
///
/// # The quartic, and why its leading coefficient is not a posture
///
/// With `w = q − c + d·t`, `h = w·â` and `ρ² = |w|² − h²`, the torus is
/// the zero set of `F = (|w|² + R² − r²)² − 4R²ρ²`. Substituting the ray
/// and depressing by `t → y − b` (with `b = (q−c)·d`) gives the DEPRESSED
/// MONIC quartic `y⁴ + p y² + q̂ y + s` with, writing `u` for the
/// component of `q − c` perpendicular to `d`, `m = |u|²`, `n = u·â`,
/// `e = d·â` and `M = m + R² − r²`:
///
/// ```text
/// p = 2M − 4R²(1 − e²)      q̂ = 8R²·e·n      s = M² − 4R²(m − n²)
/// ```
///
/// **The ladder that solves it is [`depressed_quartic_roots`]**, whose
/// docs state its count, its band and its rungs; what is the RAY's is
/// stated here. A reader arriving from the cone arm will look for the
/// vanishing leading coefficient that arm makes a posture of, and there
/// is none to find: `d` is a unit direction, so the leading coefficient
/// is `|d|⁴` and the degree cannot drop. At `f64` that is exactly 1; at
/// the `Interval` scalar it is an enclosure of 1, a few ulps wide, since
/// `normalize` cannot round to a unit vector exactly — **bounded away
/// from zero either way**, which is all the monic form needs. The lever
/// is `ext = R + r`.
///
/// What the ray's symmetry alignments degrade is the root structure,
/// and they land on the ladder's rungs (rows `line_quadric_torus_*`):
///
/// * a ray along the axis (`e = ±1`, where `1 − e² = 0` and the `t²`
///   coefficient loses its `R²` term) and a ray in the midplane
///   (`n = 0`, `q̂ = 0`) each merge roots for the poses that touch the
///   tube: rung 1, `line_quadric_torus_disc` Zero. Uncertain count — graze,
///   retry.
/// * every axis-parallel ray (`n = 0`) and every midplane ray (`e = 0`)
///   has `q̂ = 8R²·e·n` zero, the biquadratic arm (rung 2,
///   `line_quadric_torus_odd`); the four-root ray through the hole is in it.
/// * a ray all but perpendicular to the axis has a resolvent root
///   `≈ q̂²/c1` that can fall in rung 3's band (`line_quadric_torus_split_lead`).
/// * an [`Unsettled::CountDisagrees`] blocks that ray
///   (`topo`'s `PointInSolidError::RootCountDisagrees`) and another may settle.
///
/// **So this door refuses over a BAND around a tangency, and the band is
/// wide.** A caller reading "a tangent ray grazes" will assume a
/// measure-zero nuisance; it is not, for the same reason the cone arm's
/// apex refusal is not. Where the band surrounds a feature of the BODY —
/// the tube's top and bottom circles, whose tangent plane is
/// perpendicular to the axis — a near-horizontal ray from any point
/// within a few tenths of a millimetre of that circle on a unit-sized
/// torus reads in it at the default ε. `point_in_solid` sets such a ray
/// aside and another answers, so the band leaves no shell about the body
/// (`bool3_torus_doors::the_clamp_floor_clears_the_shell_about_the_top_circle`).
///
/// # Errors
///
/// [`RootRefusal::Escalated`] — an in-band classifying sign, which the
/// caller wraps in its own error type — and [`RootRefusal::Unsettled`],
/// a count the ladder does not certify.
pub fn line_torus_roots<T: Decide>(
    line: geom::Line<T>,
    torus: geom::Line<T>,
    major_radius: T,
    minor_radius: T,
    band: Band,
) -> Result<QuarticRoots<T>, RootRefusal> {
    let ext = major_radius + minor_radius;
    let LineTorusQuartic { p, q_hat, s, shift } =
        LineTorusQuartic::of(line, torus, major_radius, minor_radius);
    match depressed_quartic_roots(p, q_hat, s, ext, &crate::rows::LINE_TORUS, band)? {
        // Undo the depression: `t = y − b`.
        Roots::Certified(ys) => Ok(Roots::Certified(ys.map(|y| y - shift))),
        Roots::Miss => Ok(Roots::Miss),
    }
}

/// **The depressed quartic [`line_torus_roots`] solves**: its
/// coefficients `p`, `q̂`, `s` (that function's docs) and the shift
/// `b = (q − c)·d` it was depressed by (`t = y − b`).
#[derive(Clone, Copy, Debug)]
pub struct LineTorusQuartic<T> {
    /// The `y²` coefficient.
    pub p: T,
    /// The `y` coefficient.
    pub q_hat: T,
    /// The constant term.
    pub s: T,
    /// The depression's shift.
    pub shift: T,
}

impl<T: Decide> LineTorusQuartic<T> {
    /// The line's quartic against the torus `(center, axis, R, r)`: the
    /// one formation, which [`line_torus_roots`] solves and
    /// [`line_torus_quartic`] hands a pin.
    pub(crate) fn of(
        line: geom::Line<T>,
        torus: geom::Line<T>,
        major_radius: T,
        minor_radius: T,
    ) -> Self {
        let geom::Line { origin: q, dir: d } = line;
        let geom::Line {
            origin: center,
            dir: axis,
        } = torus;
        let two = T::from_f64(2.0);
        let four = T::from_f64(4.0);
        let w0 = q - center;
        let e = d.get().dot(axis.get());
        let shift = w0.dot(d.get());
        // The component of `q − c` perpendicular to the ray: its norm is
        // the line's distance from the centre and its axial part is the
        // height at the line's closest approach.
        let perp = w0 - d.get() * shift;
        let m = perp.norm_squared();
        let n = perp.dot(axis.get());
        let rr = major_radius.powi(2);
        let big_m = m + rr - minor_radius.powi(2);
        Self {
            p: two * big_m - four * rr * (T::one() - e.powi(2)),
            q_hat: T::from_f64(8.0) * rr * e * n,
            s: big_m.powi(2) - four * rr * (m - n.powi(2)),
            shift,
        }
    }
}

/// [`LineTorusQuartic::of`], for a pin outside this crate.
#[cfg(any(test, feature = "test-support"))]
#[must_use]
pub fn line_torus_quartic<T: Decide>(
    line: geom::Line<T>,
    torus: geom::Line<T>,
    major_radius: T,
    minor_radius: T,
) -> LineTorusQuartic<T> {
    LineTorusQuartic::of(line, torus, major_radius, minor_radius)
}

/// **How far along a line its roots against a cone lie from its point
/// `e` off the apex** (`curve_face_pose_cone_apex_spread`, a length): the
/// cone's quadric form along `e + τ·dir`, read about `e` itself,
/// `Q(e,e) + 2τ·Q(e,dir) + τ²·Q(dir,dir)` with
/// `Q(x,y) = cos²α(x·y − (x·â)(y·â)) − sin²α(x·â)(y·â)`. Every term is
/// built from the small `e`, so nothing cancels: its roots `τ±` give the
/// spread `max |τ±|·|dir|`, and where it has none the line's closest
/// approach `|Q(e,dir)/Q(dir,dir)|·|dir|` stands for them. Along the axis
/// it is the line's offset itself. A degenerate `Q(dir,dir)` (a ruling)
/// is the lead rung's, read before.
pub(super) fn apex_root_spread<T: Decide>(
    e: geom_core::Vec3<T>,
    dir: geom_core::Vec3<T>,
    (axis, half_angle): (geom_core::Vec3<T>, T),
) -> T {
    let (sin, cos) = half_angle.sin_cos();
    let (cc, ss) = (cos.powi(2), sin.powi(2));
    let q = |x: geom_core::Vec3<T>, y: geom_core::Vec3<T>| {
        let (xa, ya) = (x.dot(axis), y.dot(axis));
        cc * (x.dot(y) - xa * ya) - ss * xa * ya
    };
    let (a, b, c) = (q(dir, dir), q(e, dir), q(e, e));
    // No real roots: the closest approach, `disc` read as zero.
    let root = (b.powi(2) - a * c).max(T::zero()).sqrt();
    let speed = dir.norm();
    let (t1, t2) = ((T::zero() - b + root) / a, (T::zero() - b - root) / a);
    t1.abs().max(t2.abs()) * speed
}

/// What [`line_cone_roots`] reads a line's run against a cone.
#[derive(Clone, Copy, Debug)]
pub enum ConeRoots<T> {
    /// Two definite roots of the line's own parameter, unordered (`A`
    /// may be negative), each with a bound on its rounding in that
    /// parameter ([`Centred::roots`]' bound, read at that root).
    Two([(T, T); 2]),
    /// The run reaches the apex, where the cone has no tangent plane: the
    /// line's one root, a double one, at the end the apex is decided at,
    /// else at the line's foot from it. An edge reads it as its incidence
    /// with the apex vertex; a ray, which needs an outward side at every
    /// crossing, as a graze.
    Apex(T),
    /// A definite miss.
    Miss,
}

#[cfg(test)]
impl<T: geom_core::Real> ConeRoots<T> {
    /// The answer as an edge reads it: the apex's double root an
    /// incidence, placed twice.
    fn edge(self) -> CircleRoots<T> {
        match self {
            Self::Two(ts) => Roots::<T, 2>::all(ts.map(|(t, _)| t)).widen(),
            Self::Apex(t) => Roots::<T, 2>::all([t, t]).widen(),
            Self::Miss => CircleRoots::Miss,
        }
    }
}

/// **The certified roots of the LINE `origin + dir·t` against a CONE**,
/// over the run `(t0, t1)` of the line's parameter: the one reading of a
/// line against a cone, which an edge reads over its span and a ray over
/// the run of its line inside the ball holding the face. The cone's
/// quadric form along it is the quadratic [`line_cone_quadratic`]
/// `G = A s² + 2B s + C` (`G = −Q`, `Q = cos²α·ρ² − sin²α·h²`), read
/// about the line's closest approach to the apex held to the run
/// ([`About::Nearest`]), whose zero set is the double cone. Its
/// rungs, each a length:
///
/// - **`curve_face_pose_cone_apex`** — the run's distance from the apex.
///   Not definitely positive: the run reaches the apex, where the cone's
///   gradient vanishes. A run that ends there (an end's distance Zero)
///   and is no ruling meets the cone at that end alone, its residual
///   `A·(t − t_end)²`: one double root at the end. A run through the
///   apex inside it is the same double root at its foot there
///   ([`ConeRoots::Apex`]).
/// - **`curve_face_pose_cone_apex_spread`** — within the band of the apex is
///   not at it: how far along the line its roots lie from the point the
///   double root is placed at, the end or the foot ([`apex_root_spread`],
///   read about that point, so exact to rounding). Not definitely in the
///   band (a needle cone and a line near its axis), or in the band's
///   gap: the lane refuses [`Unsettled::AtApex`] rather than place the
///   roots there.
/// - **`curve_face_pose_cone_lead`** — `A/|dir|²` levered by the run's
///   length: how far the line turns off a generator over the run. Zero:
///   the quadratic degenerates (a ruling, or a line parallel to one) and
///   the lane does not answer ([`Unsettled::Degenerate`]).
/// - **`curve_face_pose_cone_depth`** — the depth the line's vertex `t*`
///   reaches: `G(t*) = −disc/A`, and on the near nappe
///   `Q = res·(ρ cos α + |h| sin α)` with that factor at most `|q|`, so
///   `disc/(|A|·R)`, `R ≥ |q(t*)|`, is a lower bound on the residual's
///   magnitude there, signed as the crossing count; `disc` held to the
///   rounding the shift brings into `B` and `C` ([`Centred::charged`]),
///   while that of `A` itself and of `cos²α` is charged nowhere
///   (`work/germ/the-line-cone-depth-charge-leaves-the-lead-and-cos2-rounding-uncharged.md`).
///   Positive: two roots; Zero: a tangency, [`Unsettled::Tangency`];
///   Negative: a miss.
/// - **`line_quadric_cone_root_slack`** — each root's slack along the
///   line ([`Centred::roots`]' bound read at that root, at the line's
///   speed), less the root's
///   distance outside the run. Not definitely inside the band:
///   [`Unsettled::Meter`]. A root inside the run is placed within the
///   band; one outside it is placed no nearer the run than the band,
///   which is all a reader of the run reads of it.
///
/// # Errors
///
/// A rung other than the spread in band, with its diagnostics.
pub fn line_cone_roots<T: Decide>(
    line: geom::Line<T>,
    (cone, half_angle): (geom::Line<T>, T),
    (t0, t1): (T, T),
    band: Band,
) -> Result<Result<ConeRoots<T>, Unsettled>, Indeterminate> {
    use super::pose::{ConeRead, PoseRefusal, PoseRung};
    match super::pose::line_cone_read(line, (cone, half_angle), (t0, t1), band) {
        // A ruling, or a line parallel to one: the quadratic degenerates.
        Ok(ConeRead::Lies | ConeRead::Parallel) => Ok(Err(Unsettled::Degenerate)),
        Ok(ConeRead::Through { at, .. }) => Ok(Ok(ConeRoots::Apex(at))),
        // The apex or its spread in band: refused as the rung refused it.
        Ok(ConeRead::ApexUndecided {
            why:
                PoseRefusal::InBand {
                    rung: PoseRung::ApexSpread,
                    ..
                },
            ..
        }) => Ok(Err(Unsettled::AtApex)),
        Ok(ConeRead::ApexUndecided {
            why: PoseRefusal::InBand { diag, .. },
            ..
        }) => Err(diag),
        Ok(ConeRead::ApexUndecided { .. }) => Ok(Err(Unsettled::AtApex)),
        Ok(ConeRead::Apart) => Ok(Ok(ConeRoots::Miss)),
        Ok(ConeRead::Graze { .. }) => Ok(Err(Unsettled::Tangency)),
        Ok(ConeRead::Two) => Ok(match cone_roots(line, (cone, half_angle), (t0, t1), band) {
            Ok(placed) => Ok(ConeRoots::Two(placed)),
            Err(why) => Err(why),
        }),
        // A spread past or in the band is the apex's, refused typed,
        // never escalated.
        Err(
            PoseRefusal::Meter {
                rung: PoseRung::ApexSpread,
                ..
            }
            | PoseRefusal::InBand {
                rung: PoseRung::ApexSpread,
                ..
            },
        ) => Ok(Err(Unsettled::AtApex)),
        Err(refusal) if let Some(why) = refusal.unsettled() => Ok(Err(why)),
        Err(PoseRefusal::InBand { diag, .. }) => Err(diag),
        Err(other) => unreachable!("the cone's rungs refuse in band only: {other:?}"),
    }
}

/// **The line × cone method**: a crossing line's two roots, from the
/// quadratic about its closest approach to the apex held to the run,
/// each metered (`line_quadric_cone_root_slack`): its slack along the
/// line ([`Centred::roots`]' bound read at that root, in metres, the
/// line's parameter being arc length), less the root's distance outside
/// the run, must be definitely
/// inside the band. A root inside the run is placed within the band; one
/// outside it is placed no nearer the run than the band, which is all a
/// reader of the run reads of it.
///
/// # Errors
///
/// [`Unsettled::Meter`] for a slack not definitely inside the band.
fn cone_roots<T: Decide>(
    line: geom::Line<T>,
    cone: (geom::Line<T>, T),
    run: (T, T),
    band: Band,
) -> Result<[(T, T); 2], Unsettled> {
    let read = cone_roots_read(line, cone, run, band);
    if read.iter().all(|&(_, _, held)| held) {
        Ok(read.map(|(t, slack, _)| (t, slack)))
    } else {
        Err(Unsettled::Meter {
            row: "line_quadric_cone_root_slack",
        })
    }
}

/// [`cone_roots`]' reading: each root, its slack in the line's
/// parameter, and whether that slack, less the root's distance outside
/// the run, reads definitely inside the band.
fn cone_roots_read<T: Decide>(
    line: geom::Line<T>,
    (cone, half_angle): (geom::Line<T>, T),
    (t0, t1): (T, T),
    band: Band,
) -> [(T, T, bool); 2] {
    let (lo, hi) = super::ascending((t0, t1));
    let centred = line_cone_quadratic(line, cone, half_angle, (lo, hi));
    let disc = centred.disc();
    let (roots, _) = centred.roots(disc);
    roots.map(|t| {
        let slack = centred.slack_at(t - centred.shift, disc);
        let outside = (lo - t).max(t - hi).max(T::zero());
        let held = matches!(
            decide(
                "line_quadric_cone_root_slack",
                Margin::of(slack - outside),
                band
            ),
            Ok(Sign::Zero | Sign::Negative)
        );
        (t, slack, held)
    })
}

#[cfg(test)]
#[allow(clippy::panic)]
mod wall_root_rows {
    //! [`line_wall_roots`]' two rungs are lengths: the axis-parallel
    //! rung reads the line's drift off its distance from the axis over
    //! the run its caller reads, so a wall's size does not decide it.

    use super::*;
    use geom_core::{Band, Tol};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap_or_else(|e| panic!("the witness band: {e:?}"))
    }

    /// The roots of the line `q + d·t` on the wall of radius `r` about
    /// the `z` axis, over a run of `span`.
    fn roots(q: Point3<f64>, d: Vec3<f64>, r: f64, span: f64) -> WallRoots<f64> {
        let z = Vec3::new(0.0, 0.0, 1.0);
        line_wall_roots(
            geom::Line {
                origin: q,
                dir: geom::stated_direction(d),
            },
            geom::Line {
                origin: Point3::new(0.0, 0.0, 0.0),
                dir: geom::stated_direction(z),
            },
            r,
            span,
            band(),
            About::Start,
        )
        .unwrap_or_else(|f| panic!("the rungs decide: {f:?}"))
    }

    /// **A ray straight across a wide wall's axis has its two roots.**
    /// The radius is `1/ε`, so the retired `|d⊥|²/2r` read `ε/2` here,
    /// in the zero band, and the wall arm skipped every ray of the
    /// schedule: a point on the axis of a rod that wide read `Out`
    /// (a scaled oblique rod at ε = 1e-6).
    #[test]
    fn a_ray_across_a_wide_walls_axis_has_both_roots() {
        let r = 1.0 / Tol::witness().eps();
        let got = roots(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            r,
            4.0 * r,
        );
        let WallRoots::Two([t0, t1], _) = got else {
            panic!("both roots: {got:?}");
        };
        assert!(
            (t0 + r).abs() <= r * 1e-15 && (t1 - r).abs() <= r * 1e-15,
            "the roots are the radius either side: {t0}, {t1}"
        );
    }

    /// **A near-axis ray's in-span exit root is right to its row's ε.**
    /// Each line runs all but parallel to the axis, so the lead
    /// `|d⊥|²` is tiny, and its exit root's numerator `−b + √disc`
    /// cancels: spelled that way, the root is off by `1.1e-11` (against
    /// ε = 1e-12) and `8.4e-8` (against ε = 1e-9), where
    /// [`quadratic_roots`]' `c/(−b − √disc)` is off by `4.5e-14` and
    /// `4.5e-11`. Each expected root is the exact root of the row's own
    /// `f64` data, rounded once. Read about the closest approach, the
    /// second row's run holds the approach back and its residual is read
    /// at the run's end, `2·10⁶` along the line, where the offset's
    /// rounding costs the root `1.2·10⁻⁷`: that reading is held to
    /// `10⁻¹²` of the root, the row's ε being the cancellation's.
    #[test]
    fn a_near_axis_rays_exit_root_is_right_to_the_band() {
        let tol = Tol::witness();
        let z = Vec3::new(0.0, 0.0, 1.0);
        let rows = [
            // ε, radius, q, d⊥, span, the exit root, the bound about the
            // closest approach
            (
                1e-12,
                0.5,
                (0.484_375, 0.0625),
                (2f64.powi(-18), 2f64.powi(-19)),
                4096.0,
                2_878.535_214_595_601,
                1e-12,
            ),
            (1e-9, 1e3, (999.0, 0.0), (1e-6, 0.0), 2e6, 1e6, 1e-6),
        ];
        let mut off = Vec::new();
        for (eps, r, (qx, qy), (dx, dy), span, exit, nearest) in rows {
            let band =
                Band::linear_at(tol, eps).unwrap_or_else(|e| panic!("the band at {eps}: {e:?}"));
            let d = Vec3::new(dx, dy, (1.0 - dx * dx - dy * dy).sqrt());
            for about in [About::Start, About::Nearest] {
                let got = line_wall_roots(
                    geom::Line {
                        origin: Point3::new(qx, qy, 0.0),
                        dir: geom::stated_direction(d),
                    },
                    geom::Line {
                        origin: Point3::new(0.0, 0.0, 0.0),
                        dir: geom::stated_direction(z),
                    },
                    r,
                    span,
                    band,
                    about,
                )
                .unwrap_or_else(|f| panic!("the rungs decide at {eps}: {f:?}"));
                let WallRoots::Two([_, t1], _) = got else {
                    panic!("both roots at {eps}, {about:?}: {got:?}");
                };
                let bound = match about {
                    About::Start => eps,
                    About::Nearest => nearest,
                };
                if (t1 - exit).abs() > bound {
                    off.push(format!(
                        "at ε = {eps}, {about:?}: {t1} against {exit}, off by {:e} (bound {bound:e})",
                        (t1 - exit).abs()
                    ));
                }
            }
        }
        assert!(off.is_empty(), "the exit roots: {off:?}");
    }

    /// **A ray started far from a thin wall keeps its roots.** A ray
    /// `D` out from the axis of a wall of radius `r`, across it `x0` off
    /// the axis, crosses it at `D ∓ √(r² − x0²)`. Read about the ray's
    /// start, the residual `D² + x0² − r²` loses `u·D²` against `r²`:
    /// the duplicate row's unit wall `4·10⁵` m out lost `10⁻⁵` m of its
    /// root, and a 1 mm wall 2 km out loses `10⁻⁷` m. Read about the
    /// closest approach, each is right to `10⁻¹²` m, and the slack holds
    /// the error that is left, linear in the start's distance.
    #[test]
    fn a_ray_from_far_keeps_a_walls_roots() {
        for (big, r, x0) in [(4e5, 1.0_f64, 0.6), (2e3, 1e-3, 6e-4)] {
            let half = (r * r - x0 * x0).sqrt();
            let (q, d) = (Point3::new(x0, -big, 0.3), Vec3::new(0.0, 1.0, 0.0));
            let z = Vec3::new(0.0, 0.0, 1.0);
            let o = Point3::new(0.0, 0.0, 0.0);
            let read = |about| {
                line_wall_roots(
                    geom::Line {
                        origin: q,
                        dir: geom::stated_direction(d),
                    },
                    geom::Line {
                        origin: o,
                        dir: geom::stated_direction(z),
                    },
                    r,
                    4.0 * big,
                    band(),
                    about,
                )
            };
            let Ok(WallRoots::Two([s0, s1], _)) = read(About::Start) else {
                panic!("both roots {big} out, read about the start");
            };
            let start_off = (s0 - (big - half)).abs().max((s1 - (big + half)).abs());
            assert!(
                start_off > 1e-8,
                "{big} out, read about the start, the roots lose {start_off:e}"
            );
            let got = read(About::Nearest);
            let Ok(WallRoots::Two([t0, t1], slack)) = got else {
                panic!("both roots {big} out: {got:?}");
            };
            let off = (t0 - (big - half)).abs().max((t1 - (big + half)).abs());
            assert!(
                off <= 1e-12 && off <= slack && slack <= 1e-13 * big,
                "{big} out: the roots {t0}, {t1} are off by {off:e}, the slack {slack:e}"
            );
        }
    }

    /// **The sphere's and the cone's quadratics keep their roots from
    /// far too**: a 1 mm ball 2 km out, and the right cone about `z`
    /// (`x² + y² = z²`) crossed at height 1, `x0 = 0.6` off its axis,
    /// `4·10⁵` m out.
    #[test]
    fn a_ray_from_far_keeps_a_spheres_and_a_cones_roots() {
        let d = Vec3::new(0.0, 1.0, 0.0);
        let (big, r, x0) = (2e3, 1e-3_f64, 6e-4);
        let half = (r * r - x0 * x0).sqrt();
        let got = line_sphere_roots(
            geom::Line {
                origin: Point3::new(x0, -big, 0.0),
                dir: geom::stated_direction(d),
            },
            Point3::new(0.0, 0.0, 0.0),
            r,
            band(),
            About::Nearest,
        )
        .unwrap_or_else(|e| panic!("the sphere's disc decides: {e:?}"));
        let WallRoots::Two([t0, t1], slack) = got else {
            panic!("both of the sphere's roots: {got:?}");
        };
        let off = (t0 - (big - half)).abs().max((t1 - (big + half)).abs());
        assert!(
            off <= 1e-12 && off <= slack && slack <= 1e-13 * big,
            "the sphere's roots {t0}, {t1} are off by {off:e}, the slack {slack:e}"
        );
        let big = 4e5;
        let half = (1.0 - 0.36_f64).sqrt();
        let cone = line_cone_quadratic(
            geom::Line {
                origin: Point3::new(0.6, -big, 1.0),
                dir: geom::stated_direction(d),
            },
            geom::Line {
                origin: Point3::new(0.0, 0.0, 0.0),
                dir: geom::stated_direction(Vec3::new(0.0, 0.0, 1.0)),
            },
            core::f64::consts::FRAC_PI_4,
            (0.0, 2.0 * big),
        );
        let (ts, slack) = cone.roots(cone.disc());
        let (t0, t1) = (ts[0].min(ts[1]), ts[0].max(ts[1]));
        let off = (t0 - (big - half)).abs().max((t1 - (big + half)).abs());
        assert!(
            off <= 1e-12 && off <= slack && slack <= 1e-13 * big,
            "the cone's roots {t0}, {t1} are off by {off:e}, the slack {slack:e}"
        );
    }

    /// **The axis-parallel rung reads the drift over the run.** A ray
    /// from the axis of a unit wall, tilted off it so that over a unit
    /// run it drifts `ε/4` from the axis, stays inside the band there and
    /// is axis-parallel; tilted to drift `4Kε`, it is not, and has its
    /// roots far beyond the run.
    #[test]
    fn the_axis_parallel_rung_reads_the_drift_over_the_run() {
        let tol = Tol::witness();
        let ray = |drift: f64| Vec3::new(drift, 0.0, 1.0).normalize();
        let q = Point3::new(0.0, 0.0, 0.0);
        let near = roots(q, ray(tol.eps() / 4.0), 1.0, 1.0);
        assert!(
            matches!(near, WallRoots::AxisParallel),
            "a drift of ε/4 over the run: {near:?}"
        );
        let clear = roots(q, ray(4.0 * tol.k() * tol.eps()), 1.0, 1.0);
        let WallRoots::Two([t0, t1], _) = clear else {
            panic!("a drift of 4Kε over the run has roots: {clear:?}");
        };
        assert!(
            t0 < -1.0 && t1 > 1.0,
            "the roots lie beyond the unit run: {t0}, {t1}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod line_cone_rows {
    //! **The line × cone root rungs**, each pose read against the cone's
    //! quadric form evaluated at the line's points.

    use super::*;
    use geom_core::{Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The cone about `+z` from the origin, half-angle `π/4`.
    const APEX: (f64, f64, f64) = (0.0, 0.0, 0.0);

    fn door(
        origin: [f64; 3],
        dir: [f64; 3],
        span: (f64, f64),
    ) -> Result<Result<CircleRoots<f64>, Unsettled>, Indeterminate> {
        let tip = Point3::new(APEX.0, APEX.1, APEX.2);
        let found = line_cone_roots(
            geom::Line {
                origin: Point3::from_array(origin),
                dir: geom::stated_direction(Vec3::from_array(dir)),
            },
            (
                geom::Line {
                    origin: tip,
                    dir: geom::stated_direction(Vec3::new(0.0, 0.0, 1.0)),
                },
                core::f64::consts::FRAC_PI_4,
            ),
            span,
            band(),
        )?;
        Ok(found.map(ConeRoots::edge))
    }

    fn roots(
        origin: [f64; 3],
        dir: [f64; 3],
        span: (f64, f64),
    ) -> Result<CircleRoots<f64>, Unsettled> {
        door(origin, dir, span).unwrap()
    }

    /// **A stated segment `origin + dir·t` over `span`, on its unit
    /// direction**: the same points, its span in metres, and the
    /// direction's length, the factor a root's metres are read back by.
    fn unit_segment(dir: [f64; 3], span: (f64, f64)) -> ([f64; 3], (f64, f64), f64) {
        let len = Vec3::from_array(dir).norm();
        (dir.map(|x| x / len), (span.0 * len, span.1 * len), len)
    }

    /// `ρ² − h²` at `origin + dir·t`: the quadric form of the cone.
    fn form(origin: [f64; 3], dir: [f64; 3], t: f64) -> f64 {
        let p: Vec<f64> = (0..3).map(|i| origin[i] + dir[i] * t).collect();
        p[0].powi(2) + p[1].powi(2) - p[2].powi(2)
    }

    /// **Two crossings, one on each nappe**: a line square to the axis
    /// beside it crosses one nappe twice; a line steeper than a
    /// generator, beside the apex, crosses each nappe once. Each root is
    /// a zero of the quadric form to the scalar's resolution, the two on
    /// opposite sides of the apex in the second pose.
    #[test]
    fn crossings_are_zeros_of_the_quadric_form() {
        for (label, origin, dir, nappes) in [
            (
                "square to the axis",
                [-2.0, 0.3, 1.0],
                [1.0, 0.0, 0.0],
                false,
            ),
            (
                "steep beside the apex",
                [0.2, 0.0, -1.0],
                [0.0, 0.6, 0.8],
                true,
            ),
        ] {
            let Ok(CircleRoots::Certified(found)) = roots(origin, dir, (0.0, 4.0)) else {
                panic!(
                    "{label}: two roots, got {:?}",
                    roots(origin, dir, (0.0, 4.0))
                );
            };
            let thetas: [_; 2] = found.roots().try_into().unwrap();
            for &t in &thetas {
                let f = form(origin, dir, t);
                assert!(f.abs() < 1e-14, "{label}: the form is {f} at the root {t}");
            }
            let heights: Vec<f64> = thetas.iter().map(|t| origin[2] + dir[2] * t).collect();
            assert_eq!(
                heights[0].signum() != heights[1].signum(),
                nappes,
                "{label}: the roots' heights {heights:?}"
            );
        }
    }

    /// **A miss, a graze and a ruling.** A line outside the cone that
    /// keeps clear of it is a `Miss`. A line tangent to the cone along a
    /// generator's tangent plane, moved off by a fraction of the zero
    /// band, is not a miss and not two roots: a tangency. A line parallel
    /// to a generator degenerates the quadratic, and the lane does not
    /// answer.
    #[test]
    fn a_miss_a_graze_and_a_ruling() {
        let b = band();
        assert!(
            matches!(
                roots([-2.0, 3.0, 1.0], [1.0, 0.0, 0.0], (0.0, 4.0)),
                Ok(CircleRoots::Miss)
            ),
            "a line 3 m off the axis at height 1 misses"
        );
        // Along `x` at `y = 1`, height 1: tangent to the cone's circle of
        // radius 1 there, at `(0, 1, 1)`.
        let graze = roots(
            [-2.0, 1.0 + 0.1 * b.zero(), 1.0],
            [1.0, 0.0, 0.0],
            (0.0, 4.0),
        );
        assert!(
            matches!(graze, Err(Unsettled::Tangency)),
            "a graze within the band, got {graze:?}"
        );
        let s = core::f64::consts::FRAC_1_SQRT_2;
        let ruling = roots([0.5, 0.0, 0.0], [s, 0.0, s], (0.0, 2.0));
        assert!(
            matches!(ruling, Err(Unsettled::Degenerate)),
            "a line parallel to a generator, got {ruling:?}"
        );
    }

    /// **A line a micrometre off the cone, a centiradian off its
    /// generator, is decided**: in the tangent plane at `(1, 0, 1)`,
    /// turned `0.01` rad off the generator there and moved `10⁻⁶` m off
    /// the cone along its normal, it misses outward and crosses twice
    /// inward, read over two metres about its closest approach to the
    /// apex. Its discriminant is about `|A|` times the depth it reaches,
    /// `|A| ≈ 5·10⁻⁵` here, so a depth that did not divide by `|A|` would
    /// read both in the zero band and decide neither.
    #[test]
    fn a_line_near_a_generator_and_just_off_the_cone_is_decided() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let s = core::f64::consts::FRAC_1_SQRT_2;
        let (g, n) = (Vec3::new(s, 0.0, s), Vec3::new(s, 0.0, -s));
        let dir = g * 0.01f64.cos() + Vec3::new(0.0, 1.0, 0.0) * 0.01f64.sin();
        let cone = (
            geom::Line {
                origin: Point3::new(0.0, 0.0, 0.0),
                dir: geom::stated_direction(Vec3::new(0.0, 0.0, 1.0)),
            },
            core::f64::consts::FRAC_PI_4,
        );
        for (label, off, two) in [("outward", 1e-6, false), ("inward", -1e-6, true)] {
            let line = geom::Line {
                origin: Point3::new(1.0, 0.0, 1.0) + n * off,
                dir: geom::stated_direction(dir),
            };
            let near = line.param(cone.0.origin);
            let got = line_cone_roots(line, cone, (near - 1.0, near + 1.0), band);
            assert!(
                if two {
                    matches!(got, Ok(Ok(ConeRoots::Two(..))))
                } else {
                    matches!(got, Ok(Ok(ConeRoots::Miss)))
                },
                "{label}: got {got:?}"
            );
        }
    }

    /// A line from its coordinates' bits, as the pinned poses below
    /// were drawn.
    fn bits_line(q: [u64; 3], d: [u64; 3]) -> geom::Line<f64> {
        geom::Line {
            origin: Point3::from_array(q.map(f64::from_bits)),
            dir: geom::stated_direction(Vec3::from_array(d.map(f64::from_bits))),
        }
    }

    /// The cone about `+z` from the origin, half-angle `alpha`.
    fn upright(alpha: f64) -> (geom::Line<f64>, f64) {
        (
            geom::Line {
                origin: Point3::new(0.0, 0.0, 0.0),
                dir: geom::stated_direction(Vec3::new(0.0, 0.0, 1.0)),
            },
            alpha,
        )
    }

    /// **A far line within rounding of a tangency is not read past it**
    /// (the depth rung's charge). A line `10⁷` m out, in the tangent plane
    /// of the right cone at a point of it moved off by a few of its
    /// coordinates' ulps, read over six metres about the touch. Its exact
    /// discriminant is negative (a miss, by exact rational arithmetic on
    /// these bits and a 140-digit `cos²α`), while the one the door reads
    /// about the run is positive and, uncharged, would put the depth 26
    /// escalate thresholds past zero: two roots where there are none. The
    /// shift's rounding is what moves it, and the charge holds it.
    #[test]
    fn a_far_line_within_rounding_of_a_tangency_is_not_read_as_crossing() {
        let line = bits_line(
            [
                0x4140_93ea_aece_cc42,
                0x4159_ab39_3153_23da,
                0xc15a_f956_5e6a_2fc7,
            ],
            [
                0xbfcb_d01c_f52f_5a8d,
                0xbfe5_8857_f28d_681c,
                0x3fe6_a09e_6653_8034,
            ],
        );
        let run = (
            f64::from_bits(0x4163_12cf_a000_0000),
            f64::from_bits(0x4163_12d0_6000_0000),
        );
        let band = Band::new(1e-12, 1e-11).unwrap();
        let got = line_cone_roots(line, upright(core::f64::consts::FRAC_PI_4), run, band);
        assert!(
            matches!(got, Ok(Ok(ConeRoots::Miss) | Err(Unsettled::Tangency))),
            "a miss within rounding of a tangency reads a miss or a tangency, got {got:?}"
        );
    }

    /// **Each root lies within its slack of the exact one** (the
    /// root-slack rung's bound). A line beside a narrow cone, `α = 0.3`,
    /// crossing it twice within a metre and a half, read over its two
    /// metres: each root the door places lies within the slack it carries
    /// of the exact root (by exact rational arithmetic on these bits and a
    /// 140-digit `cos²α`), and one of them lies `4.9·10⁻¹¹` from it, so
    /// a slack of nothing would not hold it.
    #[test]
    fn each_root_lies_within_its_slack_of_the_exact_one() {
        let line = bits_line(
            [
                0x3fb6_3402_4573_0020,
                0x3f96_1fbd_3d6f_348c,
                0x3fd2_58a7_c1fd_c17c,
            ],
            [
                0x3fd1_5f62_e73e_a71e,
                0x3fbe_0897_9795_168f,
                0x3fee_919f_b7dc_4306,
            ],
        );
        let band = Band::new(1e-9, 1e-8).unwrap();
        let got = line_cone_roots(line, upright(0.3), (0.0, 2.0), band);
        let Ok(Ok(ConeRoots::Two(ts))) = got else {
            panic!("two crossings, got {got:?}");
        };
        let exact = [0x3fef_8c31_efd1_e9e2_u64, 0x3ff0_3a89_fb06_8e3c].map(f64::from_bits);
        let mut ts = ts;
        ts.sort_by(|a, b| a.0.total_cmp(&b.0));
        for ((t, slack), x) in ts.into_iter().zip(exact) {
            assert!(
                (t - x).abs() <= slack,
                "the root {t} lies {:e} from the exact {x}, past its slack {slack:e}",
                (t - x).abs()
            );
        }
    }

    /// **A root the parameter cannot place within the band is not
    /// placed** (the root-slack rung). A line from `10⁷` m crossing a
    /// wide cone, `α = 1.2`, twice within its two-metre run: the exact
    /// roots lie `1.8·10⁻¹⁰` and `5.1·10⁻¹⁰` from the nearest `f64`s to
    /// them, past the `10⁻¹²` band, so no root in the line's own
    /// parameter is within the band of them, and the meter refuses.
    #[test]
    fn a_root_the_parameter_cannot_place_within_the_band_is_refused() {
        let line = bits_line(
            [
                0xc14e_9bbc_854f_ac84,
                0x4160_a9df_7b9d_e124,
                0xc145_009e_0823_8925,
            ],
            [
                0x3fd9_ad17_2f34_9ce7,
                0xbfeb_f4f3_1b8b_f71d,
                0x3fd1_9e3c_4702_d96e,
            ],
        );
        let run = (
            f64::from_bits(0x4163_12cf_e000_0000),
            f64::from_bits(0x4163_12d0_2000_0000),
        );
        let band = Band::new(1e-12, 1e-11).unwrap();
        let got = line_cone_roots(line, upright(1.2), run, band);
        assert!(
            matches!(
                got,
                Ok(Err(Unsettled::Meter {
                    row: "line_quadric_cone_root_slack"
                }))
            ),
            "the meter refuses, got {got:?}"
        );
    }

    /// **An edge through the apex has its one double root there**, as one
    /// passing within the band of it: the cone's form is homogeneous about
    /// the apex, so the caller places the root on the face's apex vertex.
    /// The apex rung reads the edge's segment, not its line: a segment
    /// ending half a metre short of the apex is not read there (its line's
    /// quadratic has its double root at the apex, which the depth rung
    /// reads as a tangency). A line through the band of the apex whose
    /// roots spread past the band along it, or into the band's gap,
    /// refuses [`Unsettled::AtApex`] at the foot (the spread rung).
    #[test]
    fn an_edge_through_the_apex_has_its_double_root_there() {
        let b = band();
        for (label, origin) in [
            ("through the apex", [-1.0, 0.0, 0.3]),
            // Its roots' spread about the foot ([`apex_root_spread`])
            // is a small multiple of `δ`: in the band.
            ("in band of the apex", [-1.0, 0.05 * b.zero(), 0.3]),
        ] {
            let (dir, span, len) = unit_segment([1.0, 0.0, -0.3], (0.0, 2.0));
            let got = roots(origin, dir, span);
            let Ok(CircleRoots::Certified(found)) = got else {
                panic!("{label}: got {got:?}");
            };
            assert!(
                found.roots().len() == 2 && found.roots().iter().all(|t| (t - len).abs() < 1e-6),
                "{label}: a double root at the apex, got {found:?}"
            );
        }
        let (dir, span, _) = unit_segment([1.0, 0.0, -0.3], (0.0, 0.5));
        let short = roots([-1.0, 0.0, 0.3], dir, span);
        assert!(
            matches!(short, Err(Unsettled::Tangency)),
            "a segment ending half a metre short of the apex, got {short:?}"
        );
        // The foot's spread rung refuses, as the end's does: a line beside
        // a generator passing in band of the apex has its roots spread
        // far along it (its spread past the band), and one whose spread
        // lies in the band's gap (about `2.3e-9` at the witness band) is
        // the apex's, refused typed, never escalated. Neither is placed
        // at the foot.
        for (label, origin, dir) in [
            (
                "beside a generator",
                [-1.0, 0.5 * b.zero(), -1.001],
                [1.0, 0.0, 1.001],
            ),
            (
                "its spread in the gap",
                [-1.0, 0.5 * b.zero(), -1.05],
                [1.0, 0.0, 1.05],
            ),
        ] {
            let (dir, span, _) = unit_segment(dir, (0.0, 2.0));
            let got = roots(origin, dir, span);
            assert!(
                matches!(got, Err(Unsettled::AtApex)),
                "through the apex, {label}: got {got:?}"
            );
        }
    }

    /// **An edge ending at the apex meets it at that end alone**: its one
    /// root is a double one there, at either end, at the apex or within a
    /// twentieth of the zero band of it. A ruling ending there is
    /// degenerate (the lead rung), and an end in the band whose roots can
    /// spread past it along the edge refuses at the apex (the spread rung):
    /// a line within a milliradian of a generator, half a zero band off.
    #[test]
    fn an_edge_ending_at_the_apex_has_its_double_root_at_that_end() {
        let b = band();
        for (label, origin, span, end) in [
            ("ending at the apex", [-1.0, 0.0, 0.3], (0.0, 1.0), 1.0),
            ("starting at the apex", [-1.0, 0.0, 0.3], (1.0, 2.0), 1.0),
            (
                "ending in band of it",
                [-1.0, 0.05 * b.zero(), 0.3],
                (0.0, 1.0),
                1.0,
            ),
        ] {
            let (dir, span, len) = unit_segment([1.0, 0.0, -0.3], span);
            let got = roots(origin, dir, span);
            let Ok(CircleRoots::Certified(found)) = got else {
                panic!("{label}: got {got:?}");
            };
            assert!(
                found.roots().len() == 2 && found.roots().iter().all(|&t| t == end * len),
                "{label}: a double root at the end, got {found:?}"
            );
        }
        let (dir, span, _) = unit_segment([1.0, 0.0, 1.0], (0.0, 1.0));
        let ruling = roots([-1.0, 0.0, -1.0], dir, span);
        assert!(
            matches!(ruling, Err(Unsettled::Degenerate)),
            "a ruling ending at the apex, got {ruling:?}"
        );
        let (dir, span, _) = unit_segment([1.0, 0.0, 1.001], (0.0, 1.0));
        let steep = roots([-1.0, 0.5 * b.zero(), -1.001], dir, span);
        assert!(
            matches!(steep, Err(Unsettled::AtApex)),
            "a line beside a generator ending in band of the apex, got {steep:?}"
        );
        // Its spread in the band's gap (about `2.3e-9` at the witness
        // band): the apex's, refused typed, never escalated.
        let (dir, span, _) = unit_segment([1.0, 0.0, 1.05], (0.0, 1.0));
        let gap = roots([-1.0, 0.5 * b.zero(), -1.05], dir, span);
        assert!(
            matches!(gap, Err(Unsettled::AtApex)),
            "a spread in the escalation band, got {gap:?}"
        );
        // Along the axis the spread is the line's offset itself: half a
        // zero band off, the end is the apex's.
        let axial = roots([0.5 * b.zero(), 0.0, -1.0], [0.0, 0.0, 1.0], (0.0, 1.0));
        let Ok(CircleRoots::Certified(found)) = axial else {
            panic!("an axial line ending half a band off the apex, got {axial:?}");
        };
        assert!(found.roots().iter().all(|&t| t == 1.0), "{found:?}");
    }

    /// **Each root is placed along the edge** (the root-slack rung's row),
    /// against an independent oracle: the double cone's residual
    /// `ρ cos α − |h| sin α`, read from each point of the line and
    /// bisected where it changes sign. The poses are steep lines beside
    /// the apex — an edge along the axis, `δ` off it — on a narrow, a
    /// right and a wide cone at a millimetre, a metre and a kilometre, at
    /// each of the three ε rows. A root on a shallow crossing can lie on
    /// the cone and still be far along the edge from the true one, so the
    /// row reads the root's PLACE, never its distance from the cone:
    ///
    /// - a certified pair has exactly the oracle's crossings in a window
    ///   one span wide either side of the edge;
    /// - each lies within the zero band, along the edge, of the oracle's,
    ///   or past the span by at least its distance from it (the rung's
    ///   contract for a root outside the span), widened only by the
    ///   oracle's own resolution;
    /// - a `Miss` has no crossing in the window.
    ///
    /// Without the rung, a narrow cone at a kilometre places roots up to
    /// two micrometres off at ε 1e-9.
    #[test]
    fn roots_beside_the_apex_are_placed_along_the_edge() {
        use crate::curve_face::conic_oracle::crossings;
        let axis = Vec3::new(0.0, 0.6, 0.8);
        let (u, _) = axis.orthonormal_basis();
        let (mut certified, mut judged) = (0, 0);
        for eps in [1e-6, 1e-9, 1e-12] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            for alpha in [0.01f64, 0.785, 1.5] {
                for scale in [1e-3, 1.0, 1e3] {
                    let apex = Point3::new(0.2 * scale, 0.1 * scale, -0.3 * scale);
                    for p in 0..30 {
                        let delta = scale * 10f64.powf(-15.0 + 0.5 * f64::from(p));
                        let origin = apex + u * delta - axis * scale;
                        let span = (0.0, 2.0 * scale);
                        let label = format!("ε {eps}, α {alpha}, scale {scale}, δ {delta:e}");
                        let res = |t: f64| {
                            let q = origin + axis * t - apex;
                            let h = q.dot(axis);
                            (q - axis * h).norm() * alpha.cos() - h.abs() * alpha.sin()
                        };
                        let length = span.1 - span.0;
                        let window = (span.0 - length, span.1 + length);
                        let inside = |t: &f64| {
                            *t > window.0 + 1e-3 * length && *t < window.1 - 1e-3 * length
                        };
                        let truth: Vec<f64> = crossings(res, window.0, window.1)
                            .into_iter()
                            .filter(inside)
                            .collect();
                        match line_cone_roots(
                            geom::Line {
                                origin,
                                dir: geom::stated_direction(axis),
                            },
                            (
                                geom::Line {
                                    origin: apex,
                                    dir: geom::stated_direction(axis),
                                },
                                alpha,
                            ),
                            span,
                            band,
                        )
                        .map(|found| found.map(ConeRoots::edge))
                        {
                            Ok(Ok(CircleRoots::Certified(found))) => {
                                let (count, thetas) = (found.count(), found.roots());
                                certified += 1;
                                let got: Vec<f64> =
                                    thetas[..count].iter().copied().filter(inside).collect();
                                assert_eq!(
                                    got.len(),
                                    truth.len(),
                                    "{label}: {got:?} vs the oracle's {truth:?}"
                                );
                                for (g, t) in got.iter().zip(&truth) {
                                    // The oracle's own resolution: its
                                    // reading's rounding over its slope.
                                    let step = 1e-6 * scale;
                                    let slope = (res(t + step) - res(t - step)) / (2.0 * step);
                                    let resolved =
                                        8.0 * f64::EPSILON * (scale + t.abs()) / slope.abs();
                                    let outside = (span.0 - g).max(g - span.1).max(0.0);
                                    let off = (g - t).abs();
                                    judged += 1;
                                    assert!(
                                        off <= eps + outside + resolved,
                                        "{label}: the root {g} lies {off:e} along the edge from \
                                         the oracle's {t} ({resolved:e} unresolved)"
                                    );
                                }
                            }
                            Ok(Ok(CircleRoots::Miss)) => {
                                assert!(truth.is_empty(), "{label}: a Miss, but {truth:?}");
                            }
                            Ok(_) | Err(_) => {}
                        }
                    }
                }
            }
        }
        assert!(
            certified > 50 && judged > 100,
            "the row reaches the rung: {certified} certified pairs, {judged} roots judged"
        );
    }
}

/// **Poses the line × quadric rows share**, here and in `topo`'s.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub mod fixtures {
    use geom_core::{Point3, Vec3};

    /// A unit ring's tube `10⁻⁴` thick: a hundred tolerances at
    /// `ε = 10⁻⁶`, where a line through it reads its quartic's count in
    /// the band ([`oblique_through_the_tube`]).
    pub const THIN_TUBE: (f64, f64) = (1.0, 1e-4);

    /// The torus of major radius `rr` and minor radius `rm` about `+y`,
    /// centred on the origin.
    #[must_use]
    pub fn torus_about_y(rr: f64, rm: f64) -> geom::Surface<f64> {
        geom::Surface::Torus {
            frame: geom::Frame::new(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            major_radius: rr,
            minor_radius: rm,
        }
    }

    /// A generic oblique line through [`torus_about_y`]'s tube near
    /// azimuth 0, with a unit direction and a nonzero odd coefficient
    /// (`e ≠ 0`, `n ≠ 0`): the quartic's cube-root branch.
    #[must_use]
    pub fn oblique_through_the_tube(rr: f64, rm: f64) -> geom::Line<f64> {
        geom::Line {
            origin: Point3::new(rr + 3.0 * rm, 2.0 * rm, -0.7 * rm),
            dir: geom_core::UnitVec3::squared(Vec3::new(-0.8, -0.33, 0.31)),
        }
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod door_rows {
    //! [`line_quadric_roots`] names the arm that refused.

    use super::fixtures::{THIN_TUBE, oblique_through_the_tube, torus_about_y};
    use super::*;

    /// **A line through a thin tube escalates as the torus arm**
    /// ([`THIN_TUBE`]): the door's refusal is the torus arm's, under one
    /// of the quartic's own rows.
    #[test]
    fn a_line_through_a_thin_tube_escalates_as_the_torus_arm() {
        let (rr, rm) = THIN_TUBE;
        let band = Band::new(1e-6, 1e-5).unwrap_or_else(|e| panic!("the band: {e:?}"));
        let got = line_quadric_roots(
            oblique_through_the_tube(rr, rm),
            (-1.0, 1.0),
            &torus_about_y(rr, rm),
            band,
        );
        let Err(LineQuadricRefusal::Torus(DoorRefusal::Escalated(diag))) = got else {
            panic!("the torus arm escalates: {got:?}");
        };
        assert!(
            diag.predicate
                .is_some_and(|p| p.starts_with("line_quadric_torus_")),
            "named by the quartic's rows: {diag:?}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod nearest_rows {
    //! **The quadratic about a line's closest approach**: its middle
    //! coefficient is zero by construction where the approach is inside
    //! the run, at every scalar, and is what it is where the run holds
    //! the approach back.

    use super::*;
    use geom_core::sym::{ParamSymbol, with_session};
    use geom_core::{Bounds, Interval, Real, Sym, SymBudget};

    /// A wall of radius 1.2 mm about the `z` axis, and a ray tilted 10°
    /// off it whose start moves with one box parameter `u ∈ [−1, 1]`,
    /// 0.5 mm by 1.2 mm across the box: the shape of a hole's aimed ray
    /// over a parameter box.
    fn boxed<T: Real>(u: T) -> (geom::Line<T>, (geom::Line<T>, T)) {
        let lift = |x: f64| T::from_f64(x);
        let (tilt, turn) = (10f64.to_radians(), 200f64.to_radians());
        let d = Vec3::new(tilt.sin() * turn.cos(), tilt.sin() * turn.sin(), tilt.cos());
        let q = Point3::new(
            lift(-2.26e-4) + lift(2.5e-4) * u,
            lift(-5.83e-4) + lift(6.0e-4) * u,
            lift(0.0),
        );
        let ray = geom::Line {
            origin: q,
            dir: geom_core::UnitVec3::squared(d.map(lift)),
        };
        let wall = geom::Line {
            origin: Point3::new(0.0, 0.0, 0.0).map(lift),
            dir: geom_core::UnitVec3::exact_z(),
        };
        (ray, (wall, lift(1.2e-3)))
    }

    /// **About its closest approach inside the run, the wall's `b` is
    /// zero over the box**, at `Interval` and at `Sym<Interval>` as at
    /// `f64`: the box enters the moved offset twice (directly and through
    /// the shift), and the two cancel exactly in `b` at every point.
    #[test]
    fn the_nearest_points_b_is_zero_over_the_box() {
        let span = 0.1;
        let point = wall_quadratic(boxed(0.0).0, boxed(0.0).1, span, About::Nearest);
        let over = |u: Interval| {
            let (ray, wall) = boxed(u);
            wall_quadratic(ray, wall, Interval::from_f64(span), About::Nearest).coeffs[1]
        };
        let interval = over(Interval::from_bounds(-1.0, 1.0));
        let budget = SymBudget {
            max_terms: 64,
            max_degree: 8,
        };
        let (sym, _) = with_session(budget, || {
            let u = Sym::param_over(
                ParamSymbol::new(0xb0),
                Interval::from_bounds(-1.0, 1.0),
                -1.0,
                1.0,
            );
            let (ray, wall) = boxed(u);
            wall_quadratic(ray, wall, Sym::from_f64(span), About::Nearest).coeffs[1].value()
        });
        let mut off = Vec::new();
        if point.coeffs[1].abs() > 1e-19 {
            off.push(format!("f64: b = {:e}", point.coeffs[1]));
        }
        for (scalar, b) in [("Interval", interval), ("Sym<Interval>", sym)] {
            if !(b.lo() == 0.0 && b.hi() == 0.0) {
                off.push(format!("{scalar}: b = [{:e}, {:e}]", b.lo(), b.hi()));
            }
        }
        assert!(off.is_empty(), "the nearest point's b: {off:?}");
    }

    /// A ray tilted 10° off the wall's axis in the `xz` plane, passing
    /// 0.12 mm from the axis, whose start the box parameter `u` moves along
    /// `d⊥` by `±1 cm`: every member is the same line, started elsewhere.
    fn along<T: Real>(u: T) -> (geom::Line<T>, (geom::Line<T>, T)) {
        let lift = |x: f64| T::from_f64(x);
        let tilt = 10f64.to_radians();
        let d = Vec3::new(tilt.sin(), 0.0, tilt.cos());
        let ray = geom::Line {
            origin: Point3::new(lift(1e-2) * u, lift(1.2e-4), lift(0.0)),
            dir: geom_core::UnitVec3::squared(d.map(lift)),
        };
        let wall = geom::Line {
            origin: Point3::new(0.0, 0.0, 0.0).map(lift),
            dir: geom_core::UnitVec3::exact_z(),
        };
        (ray, (wall, lift(1.2e-3)))
    }

    /// **About its closest approach inside the run, the wall's `c` is the
    /// line's distance from the axis, squared, less the radius's, over the
    /// box**, at `Interval` and at `Sym<Interval>`: a start moved along
    /// `d⊥` moves the shift and not the moved offset, which counts the
    /// start twice (directly and through the shift) and read `±4·10⁻⁴`
    /// against a `c` of `−1.4·10⁻⁶`.
    #[test]
    fn the_nearest_points_c_is_the_lines_distance_over_the_box() {
        let span = 0.1;
        let want = 1.2e-4f64.powi(2) - 1.2e-3f64.powi(2);
        let point = wall_quadratic(along(0.0).0, along(0.0).1, span, About::Nearest);
        let interval = {
            let (ray, wall) = along(Interval::from_bounds(-1.0, 1.0));
            wall_quadratic(ray, wall, Interval::from_f64(span), About::Nearest).coeffs[2]
        };
        let budget = SymBudget {
            max_terms: 64,
            max_degree: 8,
        };
        let (sym, _) = with_session(budget, || {
            let u = Sym::param_over(
                ParamSymbol::new(0xb1),
                Interval::from_bounds(-1.0, 1.0),
                -1.0,
                1.0,
            );
            let (ray, wall) = along(u);
            wall_quadratic(ray, wall, Sym::from_f64(span), About::Nearest).coeffs[2].value()
        });
        let mut off = Vec::new();
        let near = 1e-9 * want.abs();
        if (point.coeffs[2] - want).abs() > near {
            off.push(format!("f64: c = {:e} against {want:e}", point.coeffs[2]));
        }
        for (scalar, c) in [("Interval", interval), ("Sym<Interval>", sym)] {
            if !(c.lo() <= want && want <= c.hi() && c.hi() - c.lo() <= near) {
                off.push(format!("{scalar}: c = [{:e}, {:e}]", c.lo(), c.hi()));
            }
        }
        assert!(
            off.is_empty(),
            "the nearest point's c against {want:e}: {off:?}"
        );
    }

    /// **Where the run holds the approach back, `b` is the residual's
    /// slope at the run's end**, `(w₀⊥ + d⊥·shift)·d⊥` with the shift at
    /// that end, at `f64` and inside the box's enclosure at every point
    /// sampled.
    #[test]
    fn a_held_back_approachs_b_is_the_slope_at_the_runs_end() {
        let span = 1e-4;
        let enclosure = {
            let (ray, wall) = boxed(Interval::from_bounds(-1.0, 1.0));
            wall_quadratic(ray, wall, Interval::from_f64(span), About::Nearest)
        };
        let mut off = Vec::new();
        for i in 0..=8 {
            let u = -1.0 + f64::from(i) / 4.0;
            let (ray, (wall, radius)) = boxed(u);
            let got = wall_quadratic(ray, (wall, radius), span, About::Nearest);
            let w0p = wall.offset(ray.origin);
            let axis = wall.dir.get();
            let dp = ray.dir.get() - axis * ray.dir.get().dot(axis);
            let free = -w0p.dot(dp) / dp.norm_squared();
            let held = free.clamp(-span, span);
            let want = (w0p + dp * held).dot(dp);
            let square = (w0p + dp * held).norm_squared() - radius.powi(2);
            if (got.coeffs[2] - square).abs() > 1e-18 {
                off.push(format!("u = {u}: c {:e} against {square:e}", got.coeffs[2]));
            }
            if free.abs() <= span {
                off.push(format!("u = {u}: the approach {free:e} is inside the run"));
            }
            if (got.shift - held).abs() > 0.0 || (got.coeffs[1] - want).abs() > 1e-18 {
                off.push(format!(
                    "u = {u}: shift {:e}, b {:e} against {held:e}, {want:e}",
                    got.shift, got.coeffs[1]
                ));
            }
            let b = enclosure.coeffs[1];
            if !(b.lo() <= got.coeffs[1] && got.coeffs[1] <= b.hi()) {
                off.push(format!(
                    "u = {u}: b {:e} outside [{:e}, {:e}]",
                    got.coeffs[1],
                    b.lo(),
                    b.hi()
                ));
            }
        }
        assert!(off.is_empty(), "the held-back approach's b: {off:#?}");
    }

    /// **The wall's crossings of the boxed ray are bounded and hold every
    /// sampled member's**, over a tenth of the box: the door's roots over
    /// the nearest point's held `b`.
    #[test]
    fn the_boxed_rays_wall_crossings_are_bounded() {
        let band = geom_core::Band::linear(geom_core::Tol::witness()).unwrap();
        let read = |u: Interval| {
            let (ray, (wall, radius)) = boxed(u);
            line_wall_roots(
                ray,
                wall,
                radius,
                Interval::from_f64(0.1),
                band,
                About::Nearest,
            )
        };
        let got = read(Interval::from_bounds(-0.1, 0.1));
        let Ok(WallRoots::Two([t0, t1], slack)) = got else {
            panic!("two crossings over the box: {got:?}");
        };
        println!(
            "t0 [{:e}, {:e}], t1 [{:e}, {:e}]",
            t0.lo(),
            t0.hi(),
            t1.lo(),
            t1.hi()
        );
        let mut off = Vec::new();
        for (k, t) in [t0, t1].iter().enumerate() {
            if !(t.lo().is_finite() && t.hi().is_finite() && slack.hi().is_finite()) {
                off.push(format!(
                    "root {k}: [{:e}, {:e}], slack {:e}",
                    t.lo(),
                    t.hi(),
                    slack.hi()
                ));
            }
        }
        // Each member's crossings in closed form, `q + d·t` on the circle
        // of radius `r` about the `z` axis: an oracle the door does not
        // share.
        for i in 0..=16 {
            let u = -0.1 + f64::from(i) / 80.0;
            let (ray, (_, radius)) = boxed(u);
            let (q, d) = (ray.origin, ray.dir.get());
            let (a, b, c) = (
                d.x * d.x + d.y * d.y,
                q.x * d.x + q.y * d.y,
                q.x * q.x + q.y * q.y - radius * radius,
            );
            let root = (b * b - a * c).sqrt();
            let want = [(-b - root) / a, (-b + root) / a];
            for (k, (t, w)) in [t0, t1].iter().zip(want).enumerate() {
                let near = 1e-12 * w.abs().max(1e-3);
                if !(t.lo() - near <= w && w <= t.hi() + near) {
                    off.push(format!(
                        "u = {u}: root {k} {w:e} outside [{:e}, {:e}]",
                        t.lo(),
                        t.hi()
                    ));
                }
            }
        }
        assert!(off.is_empty(), "the boxed ray's crossings: {off:#?}");
    }
}
