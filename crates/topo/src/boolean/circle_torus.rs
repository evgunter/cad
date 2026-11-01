//! **The circle × torus root door**: the certified crossings of a
//! CIRCLE carrier with a torus, beside the line × torus quartic
//! ([`super::solid_contain::line_torus_roots`]).
//!
//! # The polynomial is a quartic, not an octic
//!
//! With `q(θ) = C(θ) − c` for the carrier
//! `C(θ) = C₀ + ρ(û cos θ + v̂ sin θ)`, `v̂ = n̂ × û`, the torus
//! `(c, â, R, r)` is the zero set of
//! `F = (|q|² + R² − r²)² − 4R²(|q|² − (q·â)²)`. Both `S = |q|²` and
//! `h = q·â` are FIRST harmonics in `θ` (with `e(θ) = û cos θ +
//! v̂ sin θ`, the cross term `2ρ(C₀ − c)·e(θ)` is the only
//! `θ`-dependence of `S`, since `|e(θ)| = 1`), so `F` is a
//! trigonometric polynomial of degree TWO, and the shared half-angle
//! ladder ([`geom_brep::half_angle::half_angle_roots`]) solves it — Bézout's
//! eight for a conic against a quartic surface loses four to the
//! circular points at infinity, through which both the circle and the
//! (bicircular) torus pass. At most four crossings per turn. The
//! harmonics are read from the torus's one home along a conic
//! ([`geom_brep::ConicTorusHarmonics`], of degree four on an ellipse);
//! on a circle its third and fourth harmonics are rounding, charged to
//! the noise.
//!
//! The door's rows are the special poses' (`bool_circle_torus_coaxial_*`,
//! the parallel arm's `bool_circle_torus_contour_residual`,
//! `_contour_side`, `_plane_height` and `_root_slack`, and
//! `bool_circle_torus_meridian`, below), then the ladder's:
//! `bool_circle_torus_pole`, `_pole_conditioning`, `_noise` and the
//! quartic's `bool_circle_torus_*`. An in-band classifying sign
//! escalates as [`BooleanDecision::ArcTorusRoots`]; an in-band coaxial
//! tilt or offset only routes the pose ([`circle_torus_roots`],
//! "Errors"). The ladder's answer is the certified subdivision's
//! (`bool_circle_torus_sub_*`), each root metered for its slack
//! (`bool_circle_torus_sub_root_slack`, "The root slack" below).
//!
//! # The harmonics' noise, and the jet
//!
//! `F`'s harmonics are sums of terms as large as `(|C₀ − c|² + ρ²)²` —
//! `ρ⁴` for a large circle — which cancel to the small values `F` takes
//! where the carrier passes the tube. `F = 2r·res·Q` with `Q ≥ R² − r²`
//! on a ring torus, so the floor on `|F|` per metre of residual is
//! `2r(R² − r²)`, and the harmonics' rounding is a residual error past
//! the band once `ρ⁴ ≳ 10ε·r·R²/(u·SHORT_CHAIN_ULPS)`: from
//! `ρ ≈ 50` at the default band against a torus `R = 1, r = 0.25`.
//!
//! So the subdivision does not read its Taylor terms from the harmonics.
//! The jet ([`geom_brep::conic_torus_jet`]) reads `F`, `F′`, `F″` and
//! `F‴` at the carrier point, by the chain rule through `|q|²` and
//! `q·â`: every term is at the point's own scale (`|q| ≤ R + 2r` near the
//! tube, one power of the carrier's speed per derivative). Its point is
//! formed as `(C₀ − c) + E`, the torus's offset first, so it rounds at
//! its own scale rather than the frame's `|C₀|`: `C₀ − c` is exact
//! wherever the two centres agree within a factor two per coordinate
//! (Sterbenz), and `|E|` is at most the carrier's size.
//! Nothing is truncated: Taylor's remainder takes the one global bound on
//! `|F⁗|` the harmonics give, `Σ k⁴Aₖ` with their noise charged, which is
//! rounded only relative to the amplitudes it bounds. What the walk's
//! decisions carry then grows like `u·ρ`, not `u·ρ⁴`. The noise still
//! meters the ladder, which reads the harmonics and only escalates
//! ([`super::circle_roots`], "The half-angle ladder, and the subdivision
//! that answers").
//!
//! What remains is the residual read at the point in [`geom_brep::Conic`]'s
//! own order, which the walk's piece ends and the root slack take: it
//! rounds at `u·|C|`. At `ρ = 1000` a crossing as shallow as a `1.6e-8` m
//! dip is placed that over its slope from the truth, past the default
//! band, and its root slack refuses
//! (`no_wrong_certified_answer_across_circle_radii`), as a graze of a
//! torus at the box's edge does
//! (`a_graze_in_a_far_frame_is_answered_truly_or_not_at_all`). The jet's
//! own charge is pinned on thin tori inside the box
//! (`a_thin_torus_is_answered_truly_or_not_at_all_inside_the_box`).
//!
//! # The lever
//!
//! The ladder's lever is the length its roots spread over in
//! `τ = 2ρ·t`: the smaller of the carrier's own `2ρ` and the torus's
//! extent `R + r`. A circle smaller than the torus has its roots spread
//! over its own size (the torus's extent there made the margins scale
//! like `ρ¹⁰/(R + r)¹¹`, and small generic circles escalated), and one
//! larger than the torus meets it only where it passes through the
//! torus's extent. The biquadratic arm's accuracy is metered by the same
//! lever, and taking the smaller length keeps its dropped term within
//! the band on the spread the roots actually have.
//!
//! # The root slack
//!
//! The subdivision bisects a root on the `f64` residual and checks that
//! it reads ON the torus. At a shallow crossing `F`'s slope along the
//! carrier is small, so the residual's rounding moves its sign change
//! along the arc by that rounding over the slope, far past the band while
//! the point still reads on the surface. Each certified root therefore
//! carries its slack (`bool_circle_torus_sub_root_slack`,
//! [`geom_brep::half_angle::RootSlack`]): `F` at the root, read in its
//! factored form `((ρ − R)² + h² − r²)·((ρ + R)² + h² − r²)` with a
//! running bound on its rounding ([`geom_brep::conic_torus_implicit`]),
//! bounds the true `|F|` there, so the true root lies within that over
//! `F`'s least slope near it; at the top speed `ρ` that is an arc length,
//! and a root whose arc the band does not read as zero is refused. The
//! reading is `F` itself, so no ceiling on `|F|` per metre of residual
//! enters.
//!
//! # The special poses
//!
//! - **Coaxial** (the carrier's axis parallel to `â` and its centre on
//!   the torus axis): `S` and `h` are constant, so `F` is constant and
//!   the quartic is `F·(1 + t²)²` — a repeated complex pair the ladder
//!   cannot answer. It is decided FIRST, geometrically
//!   (`bool_circle_torus_coaxial_tilt`, `_offset`, both metres), and the
//!   constant residual then decides it, its spread from the in-band
//!   offset and tilt charged (`bool_circle_torus_coaxial_residual`,
//!   [`super::circle_roots`], "A constant residual"): zero is
//!   [`Roots::On`] (a rim circle of the torus), and
//!   definite a [`CircleRoots::Miss`].
//! - **Parallel axes** (the circle's plane perpendicular to `â`, off
//!   the axis — the lily's pose): `h` is constant, the plane meets the
//!   tube in two contour circles, and the crossings are circle ×
//!   contour intersections in closed form, decided on the surface's
//!   RESIDUAL at the carrier's two extremes, with the band's residual
//!   tilt charged ([`parallel_axes_roots`]). The quartic does not degenerate here,
//!   but its discriminant is the wrong instrument: it also measures the
//!   separation of the COMPLEX roots, so a carrier passing a few
//!   millimetres clear of a contour reads as a tangency at a coarse
//!   band (measured on the lily at ε = 1e-6: the arch's outer seam,
//!   8 mm inside the stem's outer contour).
//! - **A meridian** (the carrier's plane through the torus axis, its
//!   centre on the centre circle, its radius the minor radius): `F ≡
//!   0`, so the pole is on the torus at every anchor and the ladder
//!   cannot answer. It is decided before the ladder, on one margin
//!   bounding every carrier point's distance from the torus
//!   (`bool_circle_torus_meridian`, [`meridian_deviation`]): Zero is
//!   [`Roots::On`], in band an escalation, and a definite
//!   deviation takes the ladder. The reduction records an
//!   `On` arc only under `reduce::lying_on`'s certificates.
//! - **A Villarceau circle** also has `F ≡ 0` and no rung: the ladder
//!   answers it `Unsettled`, so no arm records it as lying on the
//!   carrier. The reduction still reads an `Unsettled` refusal span through
//!   `carrier_touch::off_face`, which can place every touch off the
//!   face.
//! - **A tangency** — the carrier grazing the tube, a double root — is a
//!   contour-reach margin in band on the parallel arm, and on the
//!   general arm a piece neither clear nor monotone down to the band's
//!   width, refused `Unsettled` (module docs of [`super::circle_roots`],
//!   "The half-angle ladder, and the subdivision that answers").

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::circle_roots::{
    CircleRoots, constant_residual_roots, door_half_angle_roots, door_refusal,
};
use super::{BooleanDecision, BooleanError};
use crate::validate::decide;
use geom_brep::half_angle::{
    DegreeTwo, HalfAngleFrame, HalfAngleRows, RootSlack, SubdivisionFrame, SubdivisionRows,
};
use geom_brep::quartic::QuarticRows;
use geom_brep::roots::{OnCarrier, RootRefusal, Roots, Unsettled};
use geom_core::rounding_charge;

/// The circle × torus lane's ladder rows.
const CIRCLE_TORUS_ROWS: HalfAngleRows = HalfAngleRows {
    pole: "bool_circle_torus_pole",
    conditioning: "bool_circle_torus_pole_conditioning",
    noise: "bool_circle_torus_noise",
    quartic: QuarticRows {
        disc: "bool_circle_torus_disc",
        shape: "bool_circle_torus_shape",
        depth: "bool_circle_torus_depth",
        odd: "bool_circle_torus_odd",
        split: "bool_circle_torus_split",
        split_lead: "bool_circle_torus_split_lead",
    },
    verify: SubdivisionRows {
        clear: "bool_circle_torus_sub_clear",
        monotone: "bool_circle_torus_sub_monotone",
        side: "bool_circle_torus_sub_side",
        width: "bool_circle_torus_sub_width",
    },
};

/// The decision this door's root finders escalate as.
const DECISION: BooleanDecision = BooleanDecision::ArcTorusRoots;

/// The general arm's root-slack row (module docs, "The root slack").
const ROOT_SLACK: &str = "bool_circle_torus_sub_root_slack";

/// An in-band sign of this door's own rows, escalated as its decision.
fn escalated(diag: Indeterminate) -> BooleanError {
    BooleanError::Escalated {
        decision: BooleanDecision::ArcTorusRoots,
        diag,
    }
}

/// The certified crossings of the `carrier` circle with the `torus`,
/// anchored so that the arc `[t0, t1]` avoids the substitution's pole
/// (module docs).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when `carrier` is not a
/// circle or `torus` not a torus — the caller dispatched on those kinds.
/// An escalation as [`BooleanDecision::ArcTorusRoots`] for an in-band
/// classifying sign: a coaxial carrier's constant residual, a rung of the
/// parallel arm, a meridian deviation, or one of the ladder's. An
/// in-band sign at the coaxial tilt is not an error: the pose takes the
/// ladder. A meridian deviation read negative is a
/// [`BooleanError::ClassificationInvariant`]. A count the door cannot
/// certify is the inner [`Unsettled`].
pub(super) fn circle_torus_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    torus: &geom::Surface<T>,
    band: Band,
) -> Result<Result<CircleRoots<T>, Unsettled>, BooleanError> {
    let (
        &geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        },
        &geom::Surface::Torus {
            center: t_center,
            axis: t_axis,
            major_radius,
            minor_radius,
            ..
        },
    ) = (carrier, torus)
    else {
        return Err(BooleanError::ClassificationInvariant {
            what: "the circle × torus root door was handed a carrier that is not a circle \
                   or a surface that is not a torus",
        });
    };
    let two = T::from_f64(2.0);
    let v_ref = axis.cross(u_ref);
    let w0 = center - t_center;
    let point_at = |theta: T| {
        let (s, c) = theta.sin_cos();
        center + u_ref * (radius * c) + v_ref * (radius * s)
    };

    // The parallel-axes poses, decided on the geometry in metres: the
    // tilt of the carrier's axis times its radius is the amplitude of
    // the carrier's height about the torus axis, and the carrier
    // centre's distance from that axis separates coaxial from merely
    // parallel.
    let tilt = radius * axis.cross(t_axis).norm();
    let h0 = w0.dot(t_axis);
    let w_perp = w0 - t_axis * h0;
    let offset = w_perp.norm();
    let parallel = matches!(
        decide("bool_circle_torus_coaxial_tilt", Margin::of(tilt), band),
        Ok(Sign::Zero)
    );
    if parallel {
        return match decide("bool_circle_torus_coaxial_offset", Margin::of(offset), band) {
            // Coaxial: the residual is one value along the carrier, to
            // within its spread about the reading at `θ = 0`. Every
            // carrier point lies within `δ = offset + √2·tilt` of the
            // coaxial circle of the same radius (a translation by the
            // offset, then a turn of at most a right angle, whose chord
            // `2ρ·sin(α/2)` is at most `√2·ρ·sin α`), on which the
            // residual is constant. The residual is `(g² − r²)/2r` in
            // the distance `g` from the core circle, which is
            // 1-Lipschitz, so between two points `d` apart with `g` at
            // most `G` it moves at most `d·G/r`; every point in play has
            // `g ≤ g₀ + 2δ`. The reading is within `δ·G/r` of the
            // coaxial value and so is every carrier point: the spread is
            // twice that.
            Ok(Sign::Zero) => {
                let constant = geom_brep::implicit_residual(torus, point_at(T::zero()));
                let delta = offset + T::from_f64(core::f64::consts::SQRT_2) * tilt;
                let g0 = (minor_radius.powi(2) + two * minor_radius * constant)
                    .max(T::zero())
                    .sqrt();
                let spread = two * delta * (g0 + two * delta) / minor_radius;
                Ok(door_refusal(
                    constant_residual_roots(
                        constant,
                        spread,
                        "bool_circle_torus_coaxial_residual",
                        band,
                    ),
                    DECISION,
                )?
                .map(Roots::widen))
            }
            Ok(Sign::Positive) => door_refusal(
                parallel_axes_roots(
                    &ParallelPose {
                        center,
                        radius,
                        u_ref,
                        v_ref,
                        w_perp,
                        offset,
                        h0,
                        tilt,
                        t0,
                        t1,
                    },
                    torus,
                    major_radius,
                    minor_radius,
                    band,
                ),
                DECISION,
            ),
            // An in-band offset is a near-coaxial pose neither arm can
            // stand behind.
            Err(diag) => Err(BooleanError::Escalated {
                decision: DECISION,
                diag,
            }),
            Ok(Sign::Negative) => unreachable!("a norm reads no negative offset"),
        };
    }

    // A meridian: `F ≡ 0`, which the ladder cannot answer, decided on
    // its point deviation (module docs, "The special poses"). Its bound
    // leans on the ring (`R − r > 0`); a torus off D3's convention takes
    // the ladder.
    if matches!(
        geom::ring_torus(major_radius, minor_radius, band).map(|d| d.sign),
        Ok(Sign::Positive)
    ) {
        // The centre term's rounding: `w₀` and its split along the axis
        // are read off coordinates as large as `|C| + |c|`.
        let origin = Point3::new(T::zero(), T::zero(), T::zero());
        let rounding =
            rounding_charge((center - origin).norm() + (t_center - origin).norm() + major_radius);
        let deviation = meridian_deviation(
            axis,
            radius,
            w_perp,
            offset,
            h0,
            t_axis,
            major_radius,
            minor_radius,
        ) + rounding;
        match decide("bool_circle_torus_meridian", Margin::of(deviation), band) {
            Ok(Sign::Zero) => return Ok(Ok(Roots::On(OnCarrier))),
            Ok(Sign::Positive) => {}
            Ok(Sign::Negative) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "a meridian deviation, a sum of lengths, read negative",
                });
            }
            Err(diag) => return Err(escalated(diag)),
        }
    }

    // `F` along the carrier from the torus's one home along a conic
    // ([`geom_brep::ConicTorusHarmonics`]): of degree two on a circle, its
    // third and fourth harmonics no more than the frame's rounding. The
    // walk's bound on `|F⁗|` charges `2⁴·noise` (Bernstein at degree two),
    // and theirs as computed is up to `4⁴·dropped`, so they are charged
    // sixteen-fold. The rounding share covers the rest of the degree-four
    // `F` the jet reads: the exact third and fourth harmonics against the
    // computed ones, and the jet's exact `v̂ = n̂ × û` against the rounded
    // `v_ref` the harmonics are built from.
    let conic = geom_brep::Conic::circle(center, axis, radius, u_ref);
    let h = geom_brep::conic_torus_harmonics(&conic, t_center, t_axis, major_radius, minor_radius);
    let hypot = |x: T, y: T| (x.powi(2) + y.powi(2)).sqrt();
    let dropped = hypot(h.cos[3], h.sin[3]) + hypot(h.cos[4], h.sin[4]);
    let harmonics = DegreeTwo::new(h.cos[0], h.cos[1], h.sin[1], h.cos[2], h.sin[2]);
    // The lever (module docs, "The lever").
    let lever = (two * radius).min(major_radius + minor_radius);
    // The walk's Taylor terms, read at the point (module docs, "The
    // harmonics' noise, and the jet").
    let jet = |theta| {
        geom_brep::conic_torus_jet(&conic, t_center, t_axis, major_radius, minor_radius, theta)
    };
    door_half_angle_roots(
        &harmonics,
        |theta| geom_brep::implicit_residual(torus, point_at(theta)),
        HalfAngleFrame {
            walk: SubdivisionFrame {
                t0,
                t1,
                speed_hi: radius,
                noise: rounding_charge(h.terms) + T::from_f64(16.0) * dropped,
                f_per_metre: h.f_per_metre_lo,
                // The clear margin is read through the FLOOR, which
                // overstates it (`work/hone/circle-torus-clear-margin-reads-the-floor.md`).
                f_per_metre_hi: h.f_per_metre_lo,
                residual_reach: None,
                jet: Some(&jet),
            },
            speed_lo: radius,
            lever,
        },
        &CIRCLE_TORUS_ROWS,
        &RootSlack {
            row: ROOT_SLACK,
            residual: &|theta| {
                geom_brep::conic_torus_implicit(
                    &conic,
                    t_center,
                    t_axis,
                    major_radius,
                    minor_radius,
                    theta,
                )
            },
            f_per_metre_hi: T::one(),
        },
        DECISION,
        band,
    )
}

/// **A bound on every carrier point's distance from the torus, read as
/// a meridian's** (where the centre is at least `R/2` from the axis;
/// below): the sum of three point deviations, each the most one
/// condition of a meridian failing moves a carrier point off the torus.
/// The carrier is compared with the circle of radius `r` about `C*`,
/// the foot of its centre on the centre circle, in its own plane: a
/// meridian once that plane holds the axis.
///
/// - **The centre on the centre circle**: `|C − C*| = √(h₀² + (d −
///   R)²)` in the centre's height `h₀` and distance `d` from the axis.
///   Every point moves with the centre.
/// - **The radius**: `|ρ − r|`, compared as lengths. The squared
///   difference `ρ² − r²` is `(ρ + r)` times it, a different length.
/// - **The plane through the axis**: the meridian plane at `C*` has
///   normal `t̂ = â × ê`, `ê` the radial direction, and a point of the
///   compared circle is `r·e` from `C*`, `e = a·ê + b·â + s·t̂` with `|s|
///   ≤ σ = |n̂ × t̂|`. Its distance `x` from the centre circle has `x² −
///   r² = −r²s² + 2r·a·η + η²`, where `0 ≤ η ≤ r²s²/2(R − r)` is the
///   growth of its distance from the axis along `t̂`. So `|x² − r²| ≤ X
///   = r²σ² + 2r·η + η²`, and `|x − r| = |x² − r²|/(x + r) ≤ X/(r +
///   √(r² − X))`. A tilt moves points along the centre circle's
///   tangent, so it costs `≈ rσ²·R/2(R − r)`, second order: the
///   tilt's point deviation, levered by the tube radius it acts on.
///
/// The three compose by the triangle inequality into one margin, so a
/// Zero reading puts the whole carrier within the band of the torus;
/// deciding them one at a time would accept a carrier up to three
/// bands off. A coaxial carrier (`n̂ ∥ â`) has `σ = 1` and a tilt term
/// of at least `r`; a centre on the axis has a centre term of `R`.
/// Neither is a meridian. The caller has decided `R − r` positive, so
/// `R` is past the escalation threshold.
///
/// **It is a bound only where `d ≥ R/2`.** Nearer the axis `t̂` is read
/// over `R/2` rather than `d`, so `σ` and the tilt term may be short.
/// The sum is then no bound, but it is never Zero, because its centre
/// term alone is at least `R/2`, which is past the band.
#[allow(clippy::too_many_arguments)]
fn meridian_deviation<T: geom_core::Real>(
    axis: Vec3<T>,
    radius: T,
    w_perp: Vec3<T>,
    offset: T,
    h0: T,
    t_axis: Vec3<T>,
    major_radius: T,
    minor_radius: T,
) -> T {
    let r = minor_radius;
    let off_centre = (h0.powi(2) + (offset - major_radius).powi(2)).sqrt();
    // `t̂` over the centre's distance from the axis, floored at `R/2`:
    // exact wherever the centre term can read Zero, and finite on the
    // axis, where the centre term alone is `R`.
    let normal = t_axis.cross(w_perp) / offset.max(major_radius * T::from_f64(0.5));
    let sigma_sq = axis.cross(normal).norm().powi(2);
    let eta = r.powi(2) * sigma_sq / (T::from_f64(2.0) * (major_radius - r));
    let x = r.powi(2) * sigma_sq + T::from_f64(2.0) * r * eta + eta.powi(2);
    let tilt = x / (r + (r.powi(2) - x).max(T::zero()).sqrt());
    off_centre + (radius - r).abs() + tilt
}

/// The parallel-axes pose's data ([`parallel_axes_roots`]).
struct ParallelPose<T: geom_core::Real> {
    center: Point3<T>,
    radius: T,
    u_ref: Vec3<T>,
    v_ref: Vec3<T>,
    /// The carrier centre's offset from the torus axis, perpendicular
    /// to it, and its length.
    w_perp: Vec3<T>,
    offset: T,
    /// The carrier centre's height along the torus axis.
    h0: T,
    /// The amplitude of the carrier's height about `h0` (in band here).
    tilt: T,
    /// The arc: roots are reported within `π` of its midpoint.
    t0: T,
    t1: T,
}

/// Where one extreme of the carrier's distance from the torus axis sits
/// among the plane's contour circles `ρ− < ρ+`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Band3 {
    /// `ρ < ρ−`: in the hole.
    Hole,
    /// `ρ− < ρ < ρ+`: inside the tube.
    Tube,
    /// `ρ > ρ+`: beyond the tube.
    Beyond,
}

/// **The parallel-axes pose, in closed form, decided on normal
/// distance.** A carrier whose axis is parallel to the torus's lies in a
/// plane at height `h₀`, which meets the tube in the two contour circles
/// `ρ± = R ± √(r² − h₀²)` about the torus axis. The carrier's distance
/// from that axis sweeps `[ρmin, ρmax] = [|ρc − d|, ρc + d]`
/// monotonically on each half-turn about `θ₀` (its farthest point), so
/// it crosses a contour — twice, at `θ₀ ± acos((ρ±² − d² − ρc²)/(2ρc·d))`
/// — exactly when that contour lies strictly between `ρmin` and
/// `ρmax`.
///
/// **Every classifying margin is the surface's RESIDUAL, not an in-plane
/// reach.** Where the contour is crossed, the residual along the carrier
/// changes sign, and the bump it makes between the two roots is the
/// residual at the extreme point, `C(θ₀)` or `C(θ₀ + π)`. Near the top
/// or bottom of the tube the surface is nearly parallel to the plane, so
/// an in-plane reach of `x` past a contour is a normal distance of only
/// `x·√(r² − h₀²)/r`: deciding on the reach certified crossings whose
/// bump the band cannot see. Each extreme is therefore placed by its own
/// residual (`bool_circle_torus_contour_residual`), and a bump in band
/// is refused `Unsettled`.
///
/// **The tilt the pose was admitted with is carried, not dropped.** The
/// pose is parallel only to within the band: the carrier's height
/// varies by up to `tilt` about `h₀`. The residual `((ρ−R)² + h² −
/// r²)/2r` then differs from the untilted one by at most
/// `tilt·(2|h₀| + tilt)/2r` in `h`, plus the in-plane shortening of a
/// tilted circle, at most `tilt²/ρc` in `ρ` at a slope of at most
/// `(ρmax + R)/r` — and every residual margin is charged that much.
///
/// The remaining decisions are the plane's depth into the tube,
/// `(r² − h₀²)/2r` — the residual of the tube's centre circle at that
/// height, negated (`bool_circle_torus_plane_height`) — and which side
/// of the tube's centre circle an extreme outside the tube lies
/// (`bool_circle_torus_contour_side`, `ρ − R`, which is at least
/// `√(r² − h₀²)` from zero there).
fn parallel_axes_roots<T: Decide>(
    pose: &ParallelPose<T>,
    torus: &geom::Surface<T>,
    major_radius: T,
    minor_radius: T,
    band: Band,
) -> Result<CircleRoots<T>, RootRefusal> {
    let &ParallelPose {
        center,
        radius,
        u_ref,
        v_ref,
        w_perp,
        offset,
        h0,
        tilt,
        t0,
        t1,
    } = pose;
    let two = T::from_f64(2.0);
    let charge_h = tilt * (two * h0.abs() + tilt) / (two * minor_radius);
    let depth = (minor_radius.powi(2) - h0.powi(2)) / (two * minor_radius);
    match geom_brep::roots::spread_signs("bool_circle_torus_plane_height", depth, charge_h, band)? {
        (Sign::Positive, _) => {}
        // Definitely past the top or bottom of the tube, even charged.
        (_, Sign::Negative) => return Ok(CircleRoots::Miss),
        _ => return Err(Unsettled::Tangency.into()),
    }
    let rho_max = radius + offset;
    let rho_min = (radius - offset).abs();
    // Doubled: the bound above is for a point whose height and in-plane
    // radius the tilt moves once; an interior point of the arc between
    // the two extremes is moved by both the tilt of its own position and
    // the tilt's shift of where the extremes fall, which the doubling
    // covers.
    let charge = two * (charge_h + tilt.powi(2) / radius * (rho_max + major_radius) / minor_radius);
    // `θ₀`: the carrier's farthest point from the torus axis, along
    // `+w_perp`.
    let theta0 = w_perp.dot(v_ref).atan2(w_perp.dot(u_ref));
    let place = |theta: T, rho: T| -> Result<Option<Band3>, Indeterminate> {
        let (s, c) = theta.sin_cos();
        let res = geom_brep::implicit_residual(
            torus,
            center + u_ref * (radius * c) + v_ref * (radius * s),
        );
        match geom_brep::roots::spread_signs(
            "bool_circle_torus_contour_residual",
            res,
            charge,
            band,
        )? {
            (_, Sign::Negative) => return Ok(Some(Band3::Tube)),
            (Sign::Positive, _) => {}
            _ => return Ok(None),
        }
        Ok(
            match decide(
                "bool_circle_torus_contour_side",
                Margin::of(rho - major_radius),
                band,
            )? {
                Sign::Positive => Some(Band3::Beyond),
                Sign::Negative => Some(Band3::Hole),
                Sign::Zero => None,
            },
        )
    };
    let (Some(far), Some(near)) = (place(theta0, rho_max)?, place(theta0 + T::pi(), rho_min)?)
    else {
        return Err(Unsettled::Tangency.into());
    };
    let half = ((minor_radius.powi(2) - h0.powi(2)).max(T::zero())).sqrt();
    // Contour ρ− lies between the extremes iff the near one is in the
    // hole and the far one is not; ρ+ iff the far one is beyond and the
    // near one is not.
    let crossed = [
        (
            major_radius - half,
            near == Band3::Hole && far != Band3::Hole,
        ),
        (
            major_radius + half,
            far == Band3::Beyond && near != Band3::Beyond,
        ),
    ];
    let mut thetas = [T::zero(); 2 * geom_brep::half_angle::MAX_DEGREE];
    let mut count = 0usize;
    for (contour, hit) in crossed {
        if !hit {
            continue;
        }
        // Inside `[-1, 1]` by the placements above; the clamp is the
        // rounding guard, not a decision.
        let c = ((contour.powi(2) - offset.powi(2) - radius.powi(2)) / (two * radius * offset))
            .max(T::zero() - T::one())
            .min(T::one());
        let spread = c.acos();
        // **Where the roots are, not just that they are.** The roots are
        // placed as if the carrier were untilted; the tilt moves the
        // residual by up to `charge`, and the residual's slope along the
        // carrier at a contour crossing is
        // `(|ρ± − R|/r)·(ρc·d·sin spread/ρ±)` per radian, so the true
        // roots lie within `charge/slope` radians of these — plus the
        // `acos` argument's own rounding, `δc/sin spread`. That arc
        // length must be inside the band, or a caller's span and trim
        // decisions are on the wrong point.
        let sin_spread = (T::one() - c.powi(2)).max(T::zero()).sqrt();
        let slope = half / minor_radius * radius * offset * sin_spread / contour;
        let rounding = rounding_charge(contour.powi(2) + offset.powi(2) + radius.powi(2))
            / (two * radius * offset);
        let slack = radius * (charge / slope + rounding / sin_spread);
        match decide("bool_circle_torus_root_slack", Margin::of(slack), band) {
            Ok(Sign::Positive) => return Err(Unsettled::Meter.into()),
            // A NaN slack (a zero slope, which the placements above have
            // already refused as a graze) is `Err` and refuses too.
            Ok(Sign::Zero | Sign::Negative) => {}
            Err(_) => return Err(Unsettled::Meter.into()),
        }
        // Each root on the branch of the span's middle: sound at every
        // scalar by `ring_path::branch`'s window re-centring argument.
        // The one reader, `boolean::reduce`'s wall crossing, decides a
        // root at either end of a closed edge's whole-turn span alike
        // (`at_end`), and reads it only against the span's ends, never as
        // an identity.
        for theta in [theta0 + spread, theta0 - spread] {
            thetas[count] = super::circle_roots::near_span(theta, (t0, t1));
            count += 1;
        }
    }
    // `thetas` is the capacity's own size, so `count` fits.
    Ok(CircleRoots::of(&thetas[..count])
        .unwrap_or_else(|| unreachable!("the roots array is the capacity")))
}

#[cfg(test)]
mod meridian_rows;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod shallow_sweep;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]
mod tests {
    //! The door against an independent oracle: `F` sampled densely over
    //! the whole turn and each sign change bisected — simple roots only,
    //! which is the regime the door answers in. Each named pose is one
    //! of the lane's classifications, so a branch answered wrongly goes
    //! red on the pose that reaches it.

    use super::*;
    use crate::boolean::circle_roots::door_answer;
    use geom_core::{Bounds, Interval, Real, Tol};

    /// The door on a circle given by its parts.
    #[allow(clippy::too_many_arguments)]
    fn roots_of<T: Decide>(
        center: Point3<T>,
        axis: Vec3<T>,
        radius: T,
        u_ref: Vec3<T>,
        t0: T,
        t1: T,
        torus: &geom::Surface<T>,
        band: Band,
    ) -> Result<Result<CircleRoots<T>, Unsettled>, BooleanError> {
        let carrier = geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        };
        circle_torus_roots(&carrier, t0, t1, torus, band)
    }

    const R: f64 = 1.0;
    const RT: f64 = 0.25;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The default row's band, pinned: the rows that place a margin
    /// against the band's own thresholds (a bump of 5.6e-10 m, a tilt of
    /// 9e-10 m) are claims about THIS band, and at a finer ε the same
    /// geometry is honestly decidable.
    fn fixed_band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    fn torus<T: geom_core::Real>() -> geom::Surface<T> {
        geom::Surface::Torus {
            center: Point3::new(T::zero(), T::zero(), T::zero()),
            axis: Vec3::new(T::zero(), T::zero(), T::one()),
            major_radius: T::from_f64(R),
            minor_radius: T::from_f64(RT),
            u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
        }
    }

    /// A circle pose: centre, unit axis, radius, unit `u_ref ⟂ axis`.
    #[derive(Clone, Copy)]
    struct Pose {
        c: [f64; 3],
        n: [f64; 3],
        rho: f64,
        u: [f64; 3],
    }

    fn v3<T: geom_core::Real>(a: [f64; 3]) -> Vec3<T> {
        Vec3::new(T::from_f64(a[0]), T::from_f64(a[1]), T::from_f64(a[2]))
    }

    fn point(pose: Pose, theta: f64) -> Point3<f64> {
        let (n, u) = (v3::<f64>(pose.n), v3::<f64>(pose.u));
        let v = n.cross(u);
        Point3::from_array(pose.c) + (u * theta.cos() + v * theta.sin()) * pose.rho
    }

    /// The torus's own implicit `F` at a point — the oracle's function.
    fn big_f(p: Point3<f64>) -> f64 {
        let s = p.x * p.x + p.y * p.y + p.z * p.z;
        (s + R * R - RT * RT).powi(2) - 4.0 * R * R * (s - p.z * p.z)
    }

    /// Independent roots over `[t0, t1]`: sign changes on a fine grid,
    /// bisected.
    fn oracle(pose: Pose, t0: f64, t1: f64) -> Vec<f64> {
        let n = 20_000;
        let f = |t: f64| big_f(point(pose, t));
        let mut out = Vec::new();
        for i in 0..n {
            let (mut a, mut b) = (
                t0 + (t1 - t0) * f64::from(i) / f64::from(n),
                t0 + (t1 - t0) * f64::from(i + 1) / f64::from(n),
            );
            // A root on a grid point is counted in the cell it ends.
            if !(f(a) * f(b) < 0.0 || f(b) == 0.0) {
                continue;
            }
            for _ in 0..80 {
                let m = 0.5 * (a + b);
                if f(a) * f(m) <= 0.0 {
                    b = m;
                } else {
                    a = m;
                }
            }
            out.push(0.5 * (a + b));
        }
        out
    }

    /// The door's answer ([`door_answer`]): its roots, or its refusal, a
    /// limit or `None` for an escalation.
    fn door(pose: Pose, t0: f64, t1: f64) -> Result<CircleRoots<f64>, Option<Unsettled>> {
        door_answer(roots_of(
            Point3::from_array(pose.c),
            v3(pose.n),
            pose.rho,
            v3(pose.u),
            t0,
            t1,
            &torus(),
            band(),
        ))
    }

    /// The door's roots that fall in `[t0, t1]`, sorted.
    fn in_arc(pose: Pose, t0: f64, t1: f64) -> (usize, Vec<f64>) {
        let Ok(CircleRoots::Certified(found)) = door(pose, t0, t1) else {
            panic!("expected a certified count");
        };
        let (count, thetas) = (found.count(), found.roots());
        let mut ts: Vec<f64> = thetas[..count]
            .iter()
            .copied()
            .filter(|t| (t0..=t1).contains(t))
            .collect();
        ts.sort_by(f64::total_cmp);
        (count, ts)
    }

    fn assert_matches_oracle(label: &str, pose: Pose, t0: f64, t1: f64, want_count: usize) {
        let (count, ts) = in_arc(pose, t0, t1);
        let want = oracle(pose, t0, t1);
        assert_eq!(count, want_count, "{label}: certified count over the turn");
        assert_eq!(
            ts.len(),
            want.len(),
            "{label}: roots in the arc {ts:?} vs {want:?}"
        );
        for (a, b) in ts.iter().zip(&want) {
            assert!((a - b).abs() < 1e-9, "{label}: root {a} vs oracle {b}");
            assert!(big_f(point(pose, *a)).abs() < 1e-9, "{label}: F at {a}");
        }
    }

    /// Midplane, off-axis: the carrier crosses the OUTER equator twice
    /// and the INNER equator twice — the inner-equator region, four
    /// roots, which a two-root reading would halve.
    #[test]
    fn a_midplane_circle_through_both_equators_has_four_roots() {
        let pose = Pose {
            c: [1.0, 0.0, 0.0],
            n: [0.0, 0.0, 1.0],
            rho: 1.0,
            u: [1.0, 0.0, 0.0],
        };
        assert_matches_oracle("full turn", pose, -3.0, 3.0, 4);
        // An arc holding only an inner-equator crossing (`ρ = 2|cos θ/2|`
        // meets `R − r` at `θ ≈ 2.37`, the outer equator at `≈ 1.79`).
        let (_, ts) = in_arc(pose, 2.0, 3.0);
        assert_eq!(ts.len(), 1, "one inner-equator root on the arc");
        for t in &ts {
            let p = point(pose, *t);
            assert!(
                ((p.x * p.x + p.y * p.y).sqrt() - (R - RT)).abs() < 1e-9,
                "an inner-equator root at {t}"
            );
        }
    }

    /// A tilted, off-centre carrier: the generic pose, two roots on the
    /// arc and the ladder's Ferrari arm (no symmetry sets `q̂` to zero).
    #[test]
    fn a_tilted_arc_crossing_the_tube_twice_matches_the_oracle() {
        let s = 0.5_f64.sqrt();
        let pose = Pose {
            c: [1.3, 0.1, 0.05],
            n: [0.0, s, s],
            rho: 0.4,
            u: [1.0, 0.0, 0.0],
        };
        let all = oracle(pose, -3.1, 3.1);
        assert!(!all.is_empty(), "the pose must cross the tube");
        assert_matches_oracle("tilted", pose, -3.1, 3.1, all.len());
        // A short arc holding exactly the FIRST crossing: the door
        // reports the whole turn's count, the arc's filter keeps one.
        let (a, b) = (all[0] - 0.05, all[0] + 0.05);
        assert_matches_oracle("tilted short arc", pose, a, b, all.len());
    }

    /// A circle in a meridian plane, crossing the tube's cross-section
    /// twice: the parallel-to-axis family, with its `h` a pure harmonic.
    #[test]
    fn a_meridian_plane_circle_crosses_the_tube_twice() {
        let pose = Pose {
            c: [1.4, 0.0, 0.0],
            n: [0.0, 1.0, 0.0],
            rho: 0.3,
            u: [1.0, 0.0, 0.0],
        };
        assert_matches_oracle("meridian", pose, -3.0, 3.0, 2);
    }

    /// Tangent to the outer equator from outside: a double root, which
    /// the ladder must refuse rather than read as two crossings or none.
    #[test]
    fn a_tangent_circle_is_uncertain() {
        let pose = Pose {
            c: [2.0, 0.0, 0.0],
            n: [0.0, 0.0, 1.0],
            rho: 2.0 - (R + RT),
            u: [1.0, 0.0, 0.0],
        };
        assert!(
            door(pose, 2.0, 4.0).is_err(),
            "a graze is not a certified count"
        );
    }

    /// **A near-coaxial carrier whose in-band offset and tilt carry its
    /// residual across zero is never a miss**, at any admissible `K`.
    /// The circle is coaxial to within `0.99·zero` of offset and of tilt,
    /// at a tube distance `δ = −8.2e-10` inside, and `θ = 0` is turned to
    /// where its residual is lowest, past the escalation threshold at
    /// `K = 1.5`; elsewhere the residual is positive, so the carrier
    /// crosses the tube. Escalating or answering `On` or
    /// `Unsettled` is sound; `Miss` is not.
    #[test]
    fn a_near_coaxial_carrier_spanning_zero_is_not_a_miss() {
        let zero = 1e-9;
        let delta = -8.2e-10;
        let (offset, tilt) = (0.99 * zero, 0.99 * zero);
        let (phi, psi) = (3.338_f64, 10f64.to_radians());
        let h0 = (RT + delta) / 2f64.sqrt();
        let rho = R + h0;
        let c = Point3::new(offset * phi.cos(), offset * phi.sin(), h0);
        let beta = tilt / rho;
        let n = Vec3::new(beta.sin() * psi.cos(), beta.sin() * psi.sin(), beta.cos());
        let u0 = n.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        let v0 = n.cross(u0);
        let surface = torus::<f64>();
        let at = |u: Vec3<f64>, t: f64| {
            let v = n.cross(u);
            geom_brep::implicit_residual(&surface, c + (u * t.cos() + v * t.sin()) * rho)
        };
        let samples: Vec<f64> = (0..3600)
            .map(|i| f64::from(i) * core::f64::consts::PI / 1800.0)
            .collect();
        let t_min = samples
            .iter()
            .copied()
            .min_by(|a, b| at(u0, *a).total_cmp(&at(u0, *b)))
            .unwrap();
        let u = u0 * t_min.cos() + v0 * t_min.sin();
        let high = samples.iter().map(|t| at(u, *t)).fold(f64::MIN, f64::max);
        assert!(
            at(u, 0.0) < -1.5 * zero && high > 0.0,
            "the pose's premise: read at θ = 0 past 1.5·zero, positive elsewhere: \
             {} .. {high}",
            at(u, 0.0)
        );
        for k in [1.5, 1.2, 2.0, 2.5, 3.0, 10.0] {
            let got = roots_of(
                c,
                n,
                rho,
                u,
                0.0,
                1.0,
                &surface,
                Band::new(zero, k * zero).unwrap(),
            );
            assert!(
                !matches!(got, Ok(Ok(CircleRoots::Miss))),
                "K = {k}: a carrier crossing the tube certified a miss"
            );
        }
    }

    /// Coaxial with the torus: constant residual, answered before the
    /// quartic (whose complex double pair it cannot answer) by that
    /// residual's sign: a carrier clear of the tube is a miss, and one
    /// lying ON it (the outer equator) is on the surface.
    #[test]
    fn a_coaxial_circle_is_decided_by_its_constant_residual() {
        for (z, rho, on) in [(0.1, 3.0, false), (0.0, R + RT, true)] {
            let pose = Pose {
                c: [0.0, 0.0, z],
                n: [0.0, 0.0, 1.0],
                rho,
                u: [1.0, 0.0, 0.0],
            };
            let got = door(pose, 0.0, 1.0);
            assert!(
                if on {
                    matches!(got, Ok(CircleRoots::On(_)))
                } else {
                    matches!(got, Ok(CircleRoots::Miss))
                },
                "coaxial at z {z}, radius {rho}: {got:?}"
            );
        }
    }

    /// A meridian circle of the tube lies ON the torus: `F ≡ 0`, which
    /// the meridian rung decides before the ladder.
    #[test]
    fn a_meridian_lying_on_the_torus_is_on_the_surface() {
        let pose = Pose {
            c: [R, 0.0, 0.0],
            n: [0.0, 1.0, 0.0],
            rho: RT,
            u: [1.0, 0.0, 0.0],
        };
        assert!(matches!(door(pose, 0.0, 1.0), Ok(CircleRoots::On(_))));
    }

    /// Clear of the torus: a certified zero count.
    #[test]
    fn a_clear_circle_is_a_miss() {
        let pose = Pose {
            c: [4.0, 0.0, 0.3],
            n: [0.3_f64.sin(), 0.0, 0.3_f64.cos()],
            rho: 1.0,
            u: [0.3_f64.cos(), 0.0, -(0.3_f64.sin())],
        };
        assert!(matches!(door(pose, 0.0, 1.0), Ok(CircleRoots::Miss)));
    }

    /// The pole lands ON the torus at the arc's antipode: the first
    /// anchor is refused and a shifted one answers, with the same roots.
    /// A tilted pose, so the quartic arm (not the parallel one) answers.
    #[test]
    fn a_pole_on_the_torus_moves_the_anchor() {
        let s = 0.5_f64.sqrt();
        let pose = Pose {
            c: [1.3, 0.1, 0.05],
            n: [0.0, s, s],
            rho: 0.4,
            u: [1.0, 0.0, 0.0],
        };
        let all = oracle(pose, -3.1, 3.1);
        let mid = all[0] - core::f64::consts::PI;
        assert_matches_oracle("pole on the torus", pose, mid - 0.5, mid + 0.5, all.len());
    }

    /// A meridian-plane circle touching the tube's cross-section from
    /// outside: a tangency on the QUARTIC arm (the parallel arm's own
    /// graze is `a_tangent_circle_is_uncertain`).
    #[test]
    fn a_tangent_circle_off_the_parallel_pose_is_uncertain() {
        let pose = Pose {
            c: [R + 2.0 * RT, 0.0, 0.0],
            n: [0.0, 1.0, 0.0],
            rho: RT,
            u: [1.0, 0.0, 0.0],
        };
        assert!(door(pose, 2.0, 4.0).is_err());
    }

    /// The parallel pose passing a few millimetres INSIDE a contour's
    /// reach (the lily's arch seam): a definite miss of that contour at
    /// every band, where the quartic's discriminant would read the
    /// near-double complex pair as a graze.
    #[test]
    fn a_parallel_circle_just_short_of_a_contour_is_decided() {
        // Reaches ρ ≤ 0.5 + 0.742 = 1.242 < R + r = 1.25 (8 mm short),
        // and crosses the inner contour twice.
        let pose = Pose {
            c: [0.5, 0.0, 0.0],
            n: [0.0, 0.0, 1.0],
            rho: 0.742,
            u: [1.0, 0.0, 0.0],
        };
        for eps in [1e-9, 1e-6] {
            let got = roots_of(
                Point3::new(0.5, 0.0, 0.0),
                v3(pose.n),
                pose.rho,
                v3(pose.u),
                -3.0,
                3.0,
                &torus(),
                Band::new(eps, 10.0 * eps).unwrap(),
            )
            .unwrap();
            let Ok(CircleRoots::Certified(found)) = got else {
                panic!("ε {eps}: a certified count, got {got:?}");
            };
            let count = found.count();
            assert_eq!(count, 2, "ε {eps}: the inner contour only");
        }
        assert_matches_oracle("just short", pose, -3.0, 3.0, 2);
    }

    /// The interval lane: the same poses, and every certified root
    /// enclosure contains the oracle's root.
    #[test]
    fn the_interval_lane_encloses_the_oracle_roots() {
        let s = 0.5_f64.sqrt();
        for pose in [
            Pose {
                c: [1.0, 0.0, 0.0],
                n: [0.0, 0.0, 1.0],
                rho: 1.0,
                u: [1.0, 0.0, 0.0],
            },
            Pose {
                c: [1.3, 0.1, 0.05],
                n: [0.0, s, s],
                rho: 0.4,
                u: [1.0, 0.0, 0.0],
            },
        ] {
            let got = roots_of::<Interval>(
                Point3::new(
                    Interval::from_f64(pose.c[0]),
                    Interval::from_f64(pose.c[1]),
                    Interval::from_f64(pose.c[2]),
                ),
                v3(pose.n),
                Interval::from_f64(pose.rho),
                v3(pose.u),
                Interval::from_f64(-3.1),
                Interval::from_f64(3.1),
                &torus(),
                band(),
            )
            .unwrap();
            let Ok(CircleRoots::Certified(found)) = got else {
                panic!("interval lane: expected a certified count");
            };
            let (count, thetas) = (found.count(), found.roots());
            let want = oracle(pose, -3.1, 3.1);
            assert_eq!(count, want.len(), "interval count");
            for w in want {
                assert!(
                    thetas[..count]
                        .iter()
                        .any(|t| t.lo() - 1e-12 <= w && w <= t.hi() + 1e-12),
                    "oracle root {w} in no enclosure {thetas:?}"
                );
            }
        }
    }

    /// **The pole next to a root off the arc** (review of PR 3375,
    /// executed). The arc's antipode — the first anchor's pole — sits
    /// `δ` from a root of the carrier that the arc does not hold, so
    /// `F(pole)` is non-zero but tiny and the monic quartic's
    /// coefficients blow up. Unconditioned, the ladder certified 2 roots
    /// with none in the arc where the arc holds 2. The door must answer
    /// the oracle's roots or refuse, never a wrong set; with the
    /// conditioning guard it moves the anchor and answers.
    #[test]
    fn a_pole_beside_an_off_arc_root_is_moved_not_trusted() {
        let n = {
            let k = (0.3_f64.powi(2) + 1.0).sqrt();
            [0.0, 0.3 / k, 1.0 / k]
        };
        let pose = Pose {
            c: [0.9, 0.0, 0.1],
            n,
            rho: 0.5,
            u: [1.0, 0.0, 0.0],
        };
        let all = oracle(pose, -core::f64::consts::PI, core::f64::consts::PI);
        let root = *all
            .iter()
            .min_by(|a, b| (*a + 1.758).abs().total_cmp(&(*b + 1.758).abs()))
            .unwrap();
        assert!((root + 1.758).abs() < 1e-3, "the named root: {all:?}");
        for delta in [-1e-3, -1e-4, 1e-5, 1e-6, -1e-7] {
            let mid = root + delta - core::f64::consts::PI;
            let (t0, t1) = (mid - 1.0, mid + 1.0);
            assert_matches_oracle(&format!("δ = {delta}"), pose, t0, t1, all.len());
        }
    }

    /// **The parallel arm near the tube's top** (review of PR 3375,
    /// executed). The carrier's plane sits 1e-4 below the top circle
    /// and its far side reaches 2e-8 m past the outer contour IN THE
    /// PLANE — but near the top the surface is nearly flat, so the
    /// normal distance of that bump is far inside the band. The outer
    /// pair is not certifiable, and the in-plane arm certified four
    /// roots on it (the reviewer's oracle, on its pose, found two; this
    /// pose's `f64` oracle happens to resolve all four, which is not a
    /// certificate). At zero tilt and at a tilt of 9e-10 m, the door must
    /// refuse — `Unsettled`, or an escalation.
    #[test]
    fn a_parallel_bump_past_a_contour_inside_the_band_is_uncertain() {
        let h = RT - 1e-4;
        let outer = R + (RT * RT - h * h).sqrt();
        let d = 0.5;
        let rho = outer + 2e-8 - d;
        for tilt in [0.0, 9e-10] {
            let a = tilt / rho;
            let pose = Pose {
                c: [d, 0.0, h],
                n: [0.0, -a.sin(), a.cos()],
                rho,
                u: [1.0, 0.0, 0.0],
            };
            // An in-band bump is a refusal either way: `Unsettled`, or
            // the band's own escalation when the margin lands in its gap.
            let got = roots_of(
                Point3::from_array(pose.c),
                v3(pose.n),
                pose.rho,
                v3(pose.u),
                -3.0,
                3.0,
                &torus(),
                fixed_band(),
            );
            assert!(
                matches!(
                    &got,
                    Ok(Err(_))
                        | Err(BooleanError::Escalated {
                            decision: BooleanDecision::ArcTorusRoots,
                            diag: Indeterminate {
                                predicate: Some("bool_circle_torus_contour_residual"),
                                ..
                            },
                        })
                ),
                "tilt {tilt}: the in-band bump is not two crossings, got {got:?}"
            );
        }
    }
    /// **Circles lying ON the torus, whatever their pose.** `F` is
    /// identically zero along them, so its coefficients are rounding
    /// noise and the conditioning ratio can read anything. A meridian
    /// circle, at several azimuths, is the meridian rung's
    /// `On`; a Villarceau circle has no rung, and the pole's own
    /// residual decision refuses it.
    #[test]
    fn circles_lying_on_the_torus_are_meridians_or_uncertain() {
        let mut poses = Vec::new();
        let tilt = (RT / R).asin();
        for k in 0..24 {
            let az = 0.261_799 * f64::from(k) + 0.1;
            let (s, c) = az.sin_cos();
            // A meridian circle of the tube.
            poses.push(Pose {
                c: [R * c, R * s, 0.0],
                n: [-s, c, 0.0],
                rho: RT,
                u: [c, s, 0.0],
            });
            // A Villarceau circle: radius `R`, centred `r` along the
            // midplane direction `az`, in the bitangent plane through
            // that direction tilted `asin(r/R)` off the midplane.
            let perp = [-s, c, 0.0];
            poses.push(Pose {
                c: [RT * c, RT * s, 0.0],
                n: [-tilt.sin() * perp[0], -tilt.sin() * perp[1], tilt.cos()],
                rho: R,
                u: [c, s, 0.0],
            });
        }
        for (i, pose) in poses.into_iter().enumerate() {
            for (t0, t1) in [(0.0, 1.0), (2.0, 4.5)] {
                let got = door(pose, t0, t1);
                let want_meridian = i % 2 == 0;
                assert!(
                    if want_meridian {
                        matches!(got, Ok(CircleRoots::On(_)))
                    } else {
                        got.is_err()
                    },
                    "pose {i} (a {}), arc [{t0}, {t1}]: {got:?}",
                    if want_meridian {
                        "meridian"
                    } else {
                        "Villarceau circle"
                    }
                );
            }
        }
    }

    /// **A nearly full arc whose antipode is ill conditioned.** The pole
    /// then moves INTO the arc, and roots near the far end of the arc
    /// come out of the half-angle map a whole turn away from it; they
    /// must be reported within `π` of the arc's midpoint or they are
    /// silently dropped from the arc.
    #[test]
    fn roots_are_reported_within_half_a_turn_of_the_arc() {
        // Two roots 0.04 rad apart (a shallow dip into the tube): with
        // the antipode beside one of them, the other sits just inside
        // the arc's far end, where the moved anchor's map puts it a
        // whole turn away.
        let n = {
            let k = (0.3_f64.powi(2) + 1.0).sqrt();
            [0.0, 0.3 / k, 1.0 / k]
        };
        let pose = Pose {
            c: [1.44, 0.0, 0.1],
            n,
            rho: 0.8,
            u: [1.0, 0.0, 0.0],
        };
        let all = oracle(pose, -core::f64::consts::PI, core::f64::consts::PI);
        for &star in &all {
            for delta in [1e-4, -1e-4] {
                let mid = star + delta - core::f64::consts::PI;
                assert_matches_oracle(
                    &format!("antipode beside {star}, δ = {delta}"),
                    pose,
                    mid - 3.1,
                    mid + 3.1,
                    all.len(),
                );
            }
        }
    }

    /// **A parallel plane above the tube is a miss**, even where the
    /// carrier's projection spans the tube's centre circle: the plane's
    /// depth decides it before any contour does.
    #[test]
    fn a_parallel_circle_above_the_tube_is_a_miss() {
        let pose = Pose {
            c: [0.5, 0.0, RT + 0.05],
            n: [0.0, 0.0, 1.0],
            rho: 1.0,
            u: [1.0, 0.0, 0.0],
        };
        assert!(matches!(door(pose, -3.0, 3.0), Ok(CircleRoots::Miss)));
    }

    /// **The tilt a parallel pose was admitted with is charged.** The far
    /// point's residual is 1.05e-8 m — past the band's escalation
    /// threshold on its own — but the carrier is tilted 9e-10 m, and the
    /// charged margin falls back into the band: the crossing pair is not
    /// certified (a refusal: `Unsettled`, or the band's escalation).
    #[test]
    fn the_admitted_tilt_is_charged_against_the_bump() {
        let h = RT - 1e-4;
        let target = 1.05e-8;
        let rho_max = R + (RT * RT - h * h + 2.0 * RT * target).sqrt();
        let d = 0.5;
        let rho = rho_max - d;
        let a = 9e-10 / rho;
        let pose = Pose {
            c: [d, 0.0, h],
            n: [0.0, -a.sin(), a.cos()],
            rho,
            u: [1.0, 0.0, 0.0],
        };
        let got = roots_of(
            Point3::from_array(pose.c),
            v3(pose.n),
            pose.rho,
            v3(pose.u),
            -3.0,
            3.0,
            &torus(),
            fixed_band(),
        );
        assert!(
            matches!(
                &got,
                Ok(Err(_))
                    | Err(BooleanError::Escalated {
                        decision: BooleanDecision::ArcTorusRoots,
                        diag: Indeterminate {
                            predicate: Some("bool_circle_torus_contour_residual"),
                            ..
                        },
                    })
            ),
            "the charged bump is inside the band: {got:?}"
        );
    }
    /// The linearized residual `(d² − r²)/2r` evaluated DIRECTLY at a
    /// point — no harmonics, no cancellation of `ρ⁴`-sized terms, so it
    /// is accurate to the point's own rounding (`~|p|·u`). The large-
    /// circle oracle's function.
    fn direct_res(p: Point3<f64>) -> f64 {
        let rho = (p.x * p.x + p.y * p.y).sqrt();
        ((rho - R).powi(2) + p.z * p.z - RT * RT) / (2.0 * RT)
    }

    /// Roots of the direct residual on `[t0, t1]`, bisected on a grid of
    /// `n` cells.
    fn direct_roots(pose: Pose, t0: f64, t1: f64, n: u32) -> Vec<f64> {
        let f = |t: f64| direct_res(point(pose, t));
        let mut out = Vec::new();
        for i in 0..n {
            let (mut a, mut b) = (
                t0 + (t1 - t0) * f64::from(i) / f64::from(n),
                t0 + (t1 - t0) * f64::from(i + 1) / f64::from(n),
            );
            if f(a) * f(b) >= 0.0 {
                continue;
            }
            for _ in 0..100 {
                let m = 0.5 * (a + b);
                if f(a) * f(m) <= 0.0 {
                    b = m;
                } else {
                    a = m;
                }
            }
            out.push(0.5 * (a + b));
        }
        out
    }

    /// **A large circle's dip, certified away** (delta review of PR 3375,
    /// executed): `ρ = 100` dipping 9.8e-8 m into the tube at `θ = 0`.
    /// `F`'s coefficients are `~ρ⁴`, so its `f64` rounding is about
    /// `ρ⁴·u` — a residual noise near 1e-7 m, above the dip — and the
    /// unmetered door certified `Miss`. Read at the point (module docs,
    /// "The harmonics' noise, and the jet"), the walk answers the dip's
    /// two roots or refuses, on both lanes.
    #[test]
    fn a_large_circles_dip_below_its_own_noise_is_not_certified_away() {
        let pose = Pose {
            c: [
                -32.093_101_916_043_01,
                53.385_951_445_980_05,
                -77.850_260_585_465_68,
            ],
            n: [
                -0.899_931_609_984_986_3,
                -0.427_790_360_567_627_76,
                0.084_371_231_798_816_28,
            ],
            rho: 100.0,
            u: [
                0.328_284_194_818_169_7,
                -0.537_396_029_460_319_1,
                0.776_810_784_524_05,
            ],
        };
        let dip = direct_roots(pose, -0.01, 0.01, 20_000);
        assert_eq!(
            dip.len(),
            2,
            "the pose dips into the tube at θ = 0: {dip:?}"
        );
        // The dip is chosen against the default band, pinned: at a band
        // wider than 9.8e-8 m it is a graze the band calls touching.
        let got = roots_of(
            Point3::from_array(pose.c),
            v3(pose.n),
            pose.rho,
            v3(pose.u),
            0.5017,
            3.7658,
            &torus(),
            fixed_band(),
        )
        .unwrap();
        assert!(
            !matches!(got, Ok(CircleRoots::Miss)),
            "a real dip is not a certified miss: {got:?}"
        );
        if let Ok(CircleRoots::Certified(found)) = got {
            let count = found.count();
            assert_eq!(count, 2, "the whole turn holds the dip's two roots");
        }
        // The interval lane encloses the rounding the meter estimates,
        // so it never certifies the miss either.
        let got = roots_of::<Interval>(
            Point3::new(
                Interval::from_f64(pose.c[0]),
                Interval::from_f64(pose.c[1]),
                Interval::from_f64(pose.c[2]),
            ),
            v3(pose.n),
            Interval::from_f64(pose.rho),
            v3(pose.u),
            Interval::from_f64(0.5017),
            Interval::from_f64(3.7658),
            &torus(),
            fixed_band(),
        );
        assert!(
            !matches!(got, Ok(Ok(CircleRoots::Miss))),
            "interval lane: a real dip is not a certified miss: {got:?}"
        );
    }

    /// A circle of radius `rho` that touches the torus's outer equator at
    /// `P = (R + r)·(cos α, sin α, 0)` from outside, offset by `depth`
    /// along the outward normal (`depth < 0` dips into the tube), in the
    /// plane of the normal and the tube's vertical tangent — so near `P`
    /// it bends away from the torus and its extremum there IS `depth`.
    /// `P` is the carrier's `θ = 0`.
    fn grazing(rho: f64, alpha: f64, depth: f64) -> Pose {
        let (s, c) = alpha.sin_cos();
        let p = [(R + RT) * c, (R + RT) * s, 0.0];
        let k = rho + depth;
        // `u = −normal` (so `C(0) = P + depth·normal`), `v = +z` (the
        // tube's vertical tangent at `P`), `n = u × v`.
        Pose {
            c: [p[0] + k * c, p[1] + k * s, p[2]],
            n: [-s, c, 0.0],
            rho,
            u: [-c, -s, 0.0],
        }
    }

    /// The door on [`grazing`] poses of radius `rho` at four azimuths and
    /// each of `depths`, under `band`. Every answer must match the
    /// direct-residual oracle on the arc — a dip's two roots, each within
    /// `band`'s escalation threshold of the truth (span and trim decide
    /// on it), or a clearance's none. Answers the number answered and
    /// each refusal with its depth.
    fn graze_answers(rho: f64, depths: &[f64], band: Band) -> (u32, Vec<(f64, Option<Unsettled>)>) {
        graze_answers_at([0.0; 3], rho, depths, band)
    }

    /// [`graze_answers`] with the torus and the carrier moved to `far`.
    /// The carrier's centre rounds there, and the oracle reads the pose
    /// it rounded to, moved back exactly (Sterbenz: each of `far`'s
    /// coordinates is zero or at least twice the pose's).
    fn graze_answers_at(
        far: [f64; 3],
        rho: f64,
        depths: &[f64],
        band: Band,
    ) -> (u32, Vec<(f64, Option<Unsettled>)>) {
        let (mut answered, mut refused) = (0, Vec::new());
        let far_torus = geom::Surface::Torus {
            center: Point3::from_array(far),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: R,
            minor_radius: RT,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        for alpha in [0.3_f64, 1.9, 3.4, 5.1] {
            for &depth in depths {
                let placed = grazing(rho, alpha, depth);
                let at_far: [f64; 3] = core::array::from_fn(|i| placed.c[i] + far[i]);
                let pose = Pose {
                    c: core::array::from_fn(|i| at_far[i] - far[i]),
                    ..placed
                };
                let (t0, t1) = (-0.4, 0.6);
                // Fine about the graze at `θ = 0`, whose two roots are a
                // few hundred micrometres apart at `ρ = 1000`.
                let truth: Vec<f64> = [
                    direct_roots(pose, t0, -1e-3, 5_000),
                    direct_roots(pose, -1e-3, 1e-3, 20_000),
                    direct_roots(pose, 1e-3, t1, 5_000),
                ]
                .concat();
                // Moved far, the centre's rounding can move a shallow
                // graze across the tube's surface.
                if far == [0.0; 3] {
                    assert_eq!(
                        truth.len(),
                        if depth < 0.0 { 2 } else { 0 },
                        "the fixture: ρ {rho}, α {alpha}, depth {depth}: {truth:?}"
                    );
                }
                let label = format!("ρ {rho}, α {alpha}, depth {depth}");
                let got = roots_of(
                    Point3::from_array(at_far),
                    v3(pose.n),
                    pose.rho,
                    v3(pose.u),
                    t0,
                    t1,
                    &far_torus,
                    band,
                );
                match got {
                    // An escalation is a refusal too; it names no `Unsettled`.
                    Err(BooleanError::Escalated { .. }) => refused.push((depth, None)),
                    Err(e) => panic!("{label}: {e:?}"),
                    Ok(Err(why)) => refused.push((depth, Some(why))),
                    Ok(Ok(CircleRoots::On(_))) => panic!("{label}: not on the torus"),
                    Ok(Ok(CircleRoots::Miss)) => {
                        assert!(truth.is_empty(), "{label}: a certified miss on a dip");
                        answered += 1;
                    }
                    Ok(Ok(CircleRoots::Certified(found))) => {
                        let (count, thetas) = (found.count(), found.roots());
                        let mut got: Vec<f64> = thetas[..count]
                            .iter()
                            .copied()
                            .filter(|t| (t0..=t1).contains(t))
                            .collect();
                        got.sort_by(f64::total_cmp);
                        assert_eq!(got.len(), truth.len(), "{label}: {got:?} vs {truth:?}");
                        for (a, b) in got.iter().zip(&truth) {
                            assert!(
                                (a - b).abs() * rho < band.escalate(),
                                "{label}: root {a} vs {b}, {} m apart",
                                (a - b).abs() * rho
                            );
                        }
                        answered += 1;
                    }
                }
            }
        }
        (answered, refused)
    }

    /// **The ρ-sweep: every radius answered, and truly** (delta review
    /// of PR 3375). At the default band, pinned (the depths are chosen
    /// against it: the shallowest just past its escalation threshold),
    /// circles grazing the tube with an extremum depth of
    /// ±{1.6e-8, 1e-7, 1e-6, 1e-5} m, at radii 10 to 1000 m
    /// ([`graze_answers`]). The walk reads its Taylor terms at the point
    /// (module docs, "The harmonics' noise, and the jet"):
    /// through the harmonics' `ρ⁴` noise it refused every pose from
    /// `ρ = 30`. At `ρ = 1000` the shallowest dip's crossings are too
    /// shallow for the point's own rounding, `u·|C|`, to place within
    /// the band, and its root slack refuses.
    #[test]
    fn no_wrong_certified_answer_across_circle_radii() {
        let depths = [-1e-5, -1e-6, -1e-7, -1.6e-8, 1.6e-8, 1e-7, 1e-6, 1e-5];
        for rho in [10.0, 30.0, 100.0, 300.0, 1000.0] {
            let (answered, refused) = graze_answers(rho, &depths, fixed_band());
            if rho < 1000.0 {
                assert_eq!(answered, 32, "ρ {rho}: refused {refused:?}");
            } else {
                assert_eq!(answered, 28, "ρ {rho}: refused {refused:?}");
                assert!(
                    refused
                        .iter()
                        .all(|&(d, why)| d == -1.6e-8 && why == Some(Unsettled::Meter)),
                    "ρ {rho}: refused {refused:?}"
                );
            }
        }
    }

    /// **A graze at the box's edge** (D4: no coordinate past 1 km): the
    /// torus at `(800, −600, 450)`, circles grazing it by ±{2e-11, 5e-11,
    /// 2e-10} m at `ε = 1e-12` are answered truly or refused
    /// ([`graze_answers_at`]). The dips refuse on their root slack, read
    /// at the point's `u·|C|`; the jet's charge is pinned by
    /// [`JET_CHARGE_POSES`].
    #[test]
    fn a_graze_in_a_far_frame_is_answered_truly_or_not_at_all() {
        let band = Band::new(1e-12, 1e-11).unwrap();
        let depths = [-2e-10, -5e-11, -2e-11, 2e-11, 5e-11, 2e-10];
        for rho in [1.0, 10.0, 100.0] {
            let (answered, refused) = graze_answers_at([800.0, -600.0, 450.0], rho, &depths, band);
            assert_eq!(answered, 12, "ρ {rho}: refused {refused:?}");
            assert!(
                refused
                    .iter()
                    .all(|&(d, why)| d < 0.0 && why == Some(Unsettled::Meter)),
                "ρ {rho}: refused {refused:?}"
            );
        }
    }

    /// Poses inside D4's 1 km box, each as `[ε, ρ | centre | axis | u_ref
    /// | torus centre | torus axis | R, r | t0, t1]` with its true
    /// crossings round the whole carrier, within `π` of the arc's
    /// midpoint (mpmath on these inputs' exact binary values,
    /// `v̂ = n̂ × û` exact, `F` solved as an octic in `e^{iθ}`).
    const JET_CHARGE_POSES: [(&str, [f64; 21], &[f64]); 3] = [
        (
            "a thin torus a kilometre out, missed by a unit circle",
            [
                1e-12,
                1.0,
                507.003_018_189_937_7,
                1_002.336_424_722_685_6,
                347.627_288_738_176_45,
                -0.330_415_993_805_077_9,
                -0.657_387_389_811_559_4,
                0.677_249_651_719_768_4,
                -0.387_114_112_182_133_43,
                -0.560_028_469_704_868_1,
                -0.732_468_959_935_820_4,
                505.195_595_650_155_84,
                999.541_120_017_644_7,
                345.582_532_419_900_9,
                0.523_514_970_858_270_3,
                -0.604_127_346_080_746_4,
                0.600_801_319_077_029_6,
                3.0,
                0.002,
                -2.286_650_806_909_016,
                0.937_455_736_282_058_8,
            ],
            &[],
        ),
        (
            "a unit circle grazing a 10 µm tube off its arc",
            [
                1e-12,
                1.0,
                -0.611_225_538_923_048,
                1.096_155_308_198_281_8,
                2.121_348_753_093_851,
                -0.102_888_123_316_417_13,
                -0.633_027_011_128_100_4,
                0.767_261_909_169_645_7,
                0.837_661_056_997_784,
                0.360_830_676_764_738_5,
                0.410_030_701_649_103_5,
                -2.241_119_860_598_858_3,
                0.401_977_901_178_566,
                1.308_154_963_926_711_8,
                0.714_488_144_006_292_8,
                -0.563_692_229_621_536,
                -0.414_436_680_735_121,
                1.0,
                1e-5,
                -0.822_786_698_571_043_9,
                -0.025_732_322_349_077_186,
            ],
            &[-3.366_017_259_199_048_8, -3.366_017_245_046_99],
        ),
        (
            "a 2.4 m circle grazing a 10 µm tube 19 m out",
            [
                1e-12,
                2.432_373_458_132_554_2,
                -10.756_577_737_356_77,
                -10.029_223_315_682_913,
                8.303_334_310_974_376,
                0.878_683_474_850_792_9,
                0.017_333_233_125_561_884,
                0.477_090_043_968_170_56,
                0.448_109_665_837_044_7,
                0.314_735_191_338_378,
                -0.836_743_381_638_962_8,
                -14.331_002_187_632_686,
                -8.526_749_186_199_282,
                9.002_118_538_301_69,
                0.001_328_256_357_129_369_8,
                -0.629_207_234_375_604_4,
                0.777_236_445_326_937,
                3.0,
                1e-5,
                0.238_173_809_235_491_47,
                4.148_659_795_226_928,
            ],
            &[1.820_029_089_088_455_8, 1.820_029_093_191_118],
        ),
    ];

    /// **The jet is read at the torus's offset, and charged**: on every
    /// [`JET_CHARGE_POSES`] row the door's answer is the truth, each root
    /// within `ε` of arc of one, or a refusal — and the far thin torus's
    /// miss is answered. Formed at the frame's coordinates, the jet's
    /// point charged `u·|C|` and that miss refused `Tangency`. With the
    /// jet's charge cut to `0.07×` or less, the last row is a certified
    /// miss; at `0.1×` none of the box's thin grazes found one.
    #[test]
    fn a_thin_torus_is_answered_truly_or_not_at_all_inside_the_box() {
        for (label, v, truth) in JET_CHARGE_POSES {
            let p = |i: usize| Point3::new(v[i], v[i + 1], v[i + 2]);
            let w = |i: usize| Vec3::new(v[i], v[i + 1], v[i + 2]);
            let (eps, rho) = (v[0], v[1]);
            let torus = geom::Surface::Torus {
                center: p(11),
                axis: w(14),
                major_radius: v[17],
                minor_radius: v[18],
                u_ref: w(14).orthonormal_basis().0,
            };
            let band = Band::new(eps, 10.0 * eps).unwrap();
            match roots_of(p(2), w(5), rho, w(8), v[19], v[20], &torus, band) {
                Ok(Ok(CircleRoots::Miss)) => assert!(truth.is_empty(), "{label}: a false miss"),
                Ok(Ok(CircleRoots::Certified(found))) => {
                    let mut got = found.roots()[..found.count()].to_vec();
                    got.sort_by(f64::total_cmp);
                    assert_eq!(got.len(), truth.len(), "{label}: {got:?} vs {truth:?}");
                    for (a, b) in got.iter().zip(truth) {
                        assert!((a - b).abs() * rho <= eps, "{label}: root {a} vs {b}");
                    }
                }
                Ok(Ok(CircleRoots::On(_))) => panic!("{label}: not on the torus"),
                Ok(Err(_)) | Err(BooleanError::Escalated { .. }) => {
                    assert!(!truth.is_empty(), "{label}: the miss refused");
                }
                Err(e) => panic!("{label}: {e:?}"),
            }
        }
    }

    /// **The same sweep at a coarse band**: at `ε = 1e-6` the harmonics'
    /// noise passed the band from `ρ ≈ 500`, and circles of 300 m and
    /// 1 km grazing the tube by ±{1.6e-5, 1e-4, 1e-3} m refused there.
    /// Read at the point, every one is answered, and truly.
    #[test]
    fn a_kilometre_circle_grazing_the_tube_is_answered_at_a_coarse_band() {
        let band = Band::new(1e-6, 1e-5).unwrap();
        let depths = [-1e-3, -1e-4, -1.6e-5, 1.6e-5, 1e-4, 1e-3];
        for rho in [300.0, 1000.0] {
            let (answered, refused) = graze_answers(rho, &depths, band);
            assert_eq!(answered, 24, "ρ {rho}: refused {refused:?}");
        }
    }

    /// The general arm's shallow crossings at `ε = 1e-12` that the `f64`
    /// lane once placed tens of bands from the truth, each as `[t0, t1 |
    /// centre | axis | ρ | u_ref | torus centre | torus axis | R, r]` with
    /// its true crossings over the turn (mpmath at 70 digits on these
    /// inputs' exact binary values, `v̂ = n̂ × û` exact).
    pub(super) const SHALLOW_POSES: [(&str, [f64; 20], &[f64]); 3] = [
        (
            "a near-parallel circle at the tube's top, its tilt in the gap",
            [
                -3.930_597_365_412_273_4,
                -1.609_445_425_789_279_2,
                0.019_185_322_485_892_85,
                -0.049_505_276_644_974_7,
                -0.249_999_999_829_764_8,
                3.045_264_808_137_040_3e-13,
                -1.136_254_354_833_023_5e-12,
                1.0,
                1.053_097_717_686_373_2,
                0.726_269_536_922_667_8,
                -0.687_410_037_559_922_2,
                -1.002_240_954_934_652_3e-12,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                1.0,
                0.25,
            ],
            &[2.685_922_411_422_998_5, 2.710_943_694_713_821_5],
        ),
        (
            "a ten-metre-scale torus",
            [
                8.492_969_774_488_93,
                10.356_489_392_302_237,
                5.463_512_492_317_101,
                -9.746_523_357_584_595,
                -8.536_346_470_744_599,
                -0.376_844_075_612_356_17,
                0.708_864_775_492_544_9,
                -0.596_237_597_558_031_3,
                0.998_902_123_005_966_1,
                0.234_919_854_312_025_83,
                -0.549_505_558_951_724_3,
                -0.801_783_201_826_509_6,
                5.890_357_356_241_01,
                -9.360_284_482_369_583,
                -6.886_263_665_719_765,
                0.266_750_385_940_342_6,
                -0.873_228_420_448_758_6,
                0.407_819_025_207_560_1,
                2.428_454_476_566_526_6,
                0.302_517_561_644_429_8,
            ],
            &[
                4.839_154_770_862_667_4e-5,
                0.027_670_534_202_302_89,
                1.475_588_767_894_032_7,
                6.283_137_001_569_704,
            ],
        ),
        (
            "the unit torus, an everyday pose",
            [
                2.133_241_833_704_526,
                2.721_386_152_157_228,
                -0.269_743_561_126_599_36,
                0.180_110_200_879_629_95,
                1.572_015_540_757_338_2,
                -0.822_678_302_897_906,
                -0.052_700_024_459_512_32,
                -0.566_059_287_851_536_1,
                2.209_443_825_872_272,
                0.561_611_431_474_315_7,
                0.079_275_393_383_984_45,
                -0.823_594_567_758_423_7,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                1.0,
                0.25,
            ],
            &[
                3.126_822_588_620_022e-5,
                0.589_305_912_820_224,
                6.278_594_383_972_917_5,
                6.283_153_825_613_722,
            ],
        ),
    ];

    /// The door on a [`SHALLOW_POSES`] row: its certified roots as
    /// `(lo, hi)` pairs (`pair` reads one), or what it answered instead.
    fn shallow_door<T: Decide>(
        v: &[f64; 20],
        band: Band,
        pair: fn(T) -> (f64, f64),
    ) -> Result<Vec<(f64, f64)>, String> {
        let at = T::from_f64;
        let p = |i: usize| Point3::new(at(v[i]), at(v[i + 1]), at(v[i + 2]));
        let w = |i: usize| Vec3::new(at(v[i]), at(v[i + 1]), at(v[i + 2]));
        let torus = geom::Surface::Torus {
            center: p(12),
            axis: w(15),
            major_radius: at(v[18]),
            minor_radius: at(v[19]),
            u_ref: {
                let (e, _) = Vec3::new(v[15], v[16], v[17]).orthonormal_basis();
                Vec3::new(at(e.x), at(e.y), at(e.z))
            },
        };
        match roots_of(p(2), w(5), at(v[8]), w(9), at(v[0]), at(v[1]), &torus, band) {
            Ok(Ok(CircleRoots::Certified(found))) => {
                Ok(found.roots().iter().map(|&t| pair(t)).collect())
            }
            Ok(Ok(other)) => Err(format!("{other:?}")),
            Ok(Err(why)) => Err(format!("unsettled: {why:?}")),
            Err(e) => Err(format!("refused: {e:?}")),
        }
    }

    /// **A shallow crossing's root is placed within the band, or the door
    /// refuses** (`bool_circle_torus_sub_root_slack`). At these poses the
    /// residual's slope along the carrier at a root is `1e-8`–`1e-6`, so
    /// the `f64` residual's rounding moves its sign change by up to
    /// 1.66e-9 m of arc, 166 bands at `ε = 1e-12`, while the root still
    /// reads ON the torus. Each pose is outside the band (at least
    /// 1.2 `Kε` deep); on both lanes a certified answer must hold every
    /// true root and each of its roots within `Kε` of arc of one.
    #[test]
    fn a_shallow_crossings_root_is_within_the_band_or_refused() {
        use core::f64::consts::TAU;
        let band = Band::new(1e-12, 1e-11).unwrap();
        let reach = 1e-11;
        for (label, v, truth) in SHALLOW_POSES {
            let rho = v[8];
            let off = |(lo, hi): (f64, f64), t: f64| {
                let (mid, half) = ((lo + hi) / 2.0, (hi - lo) / 2.0);
                let turn = (mid - t).rem_euclid(TAU);
                (turn.min(TAU - turn) - half).max(0.0) * rho
            };
            for (lane, got) in [
                ("f64", shallow_door(&v, band, |t: f64| (t, t))),
                (
                    "Interval",
                    shallow_door(&v, band, |t: Interval| (t.lo(), t.hi())),
                ),
            ] {
                let roots = match got {
                    Ok(roots) => roots,
                    Err(what) if what.starts_with("unsettled") || what.starts_with("refused") => {
                        continue;
                    }
                    Err(what) => {
                        panic!("{label} ({lane}): a refusal or the true roots, got {what}")
                    }
                };
                assert_eq!(
                    roots.len(),
                    truth.len(),
                    "{label} ({lane}): the certified count"
                );
                for &r in &roots {
                    let near = truth
                        .iter()
                        .map(|&t| off(r, t))
                        .fold(f64::INFINITY, f64::min);
                    assert!(
                        near <= reach,
                        "{label} ({lane}): certified root {r:?} lies {near:.3e} m of arc from \
                         every true root, past the band's {reach:e}"
                    );
                }
                for &t in truth {
                    let near = roots
                        .iter()
                        .map(|&r| off(r, t))
                        .fold(f64::INFINITY, f64::min);
                    assert!(
                        near <= reach,
                        "{label} ({lane}): the true root {t} lies {near:.3e} m of arc from every \
                         certified root"
                    );
                }
            }
        }
    }

    /// **The admitted tilt moves the parallel arm's ROOTS, not just its
    /// margins** (delta review of PR 3375). Near the tube's top the
    /// carrier crosses the outer contour shallowly — a bump of 5e-8 m,
    /// definite on its own — so the residual's slope along the carrier at
    /// the roots is tiny, and a tilt of 9e-10 m moves them by tens of
    /// micrometres of arc. Untilted, the door answers the oracle; tilted,
    /// it must refuse rather than hand a caller points that far off.
    #[test]
    fn the_admitted_tilt_moves_the_parallel_roots() {
        let h = RT - 1e-4;
        let half = (RT * RT - h * h).sqrt();
        let x = 5e-8 * RT / half;
        let d = 0.5;
        let rho = R + half + x - d;
        for tilt in [0.0, 9e-10] {
            let a = tilt / rho;
            let pose = Pose {
                c: [d, 0.0, h],
                n: [0.0, -a.sin(), a.cos()],
                rho,
                u: [1.0, 0.0, 0.0],
            };
            let got = roots_of(
                Point3::from_array(pose.c),
                v3(pose.n),
                pose.rho,
                v3(pose.u),
                -0.5,
                0.5,
                &torus(),
                fixed_band(),
            );
            if tilt == 0.0 {
                let Ok(Ok(CircleRoots::Certified(found))) = got else {
                    panic!("untilted: a certified count, got {got:?}");
                };
                let (count, thetas) = (found.count(), found.roots());
                assert_eq!(count, 4, "untilted: both contours");
                let mut ts: Vec<f64> = thetas[..count]
                    .iter()
                    .copied()
                    .filter(|t| (-0.5..=0.5).contains(t))
                    .collect();
                ts.sort_by(f64::total_cmp);
                let want = direct_roots(pose, -0.5, 0.5, 200_000);
                assert_eq!(ts.len(), want.len(), "untilted: {ts:?} vs {want:?}");
                for (a, b) in ts.iter().zip(&want) {
                    assert!((a - b).abs() * rho < 1e-8, "untilted: root {a} vs {b}");
                }
            } else {
                assert!(
                    matches!(got, Ok(Err(_))),
                    "tilt {tilt}: roots displaced past the band are not certified: {got:?}"
                );
            }
        }
    }
}
