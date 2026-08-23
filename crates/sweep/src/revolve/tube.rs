//! **`tube_along_arc` — the world-coordinate tube/torus door** (M6-3
//! Leg F; the Evan-ratified rider, #175 thread). A ring-torus body
//! from its INTENT parameters: spine centre/axis/reference direction,
//! major radius, an arc window (or a full ring), and the tube's minor
//! radius — **stored exactly**, with no profile→bulge→radius
//! arithmetic anywhere on the path. This is what retires the lily
//! findings' silent sketch-frame placement (finding 11) and the
//! revolve minor-radius reconstruction drift (56 ulps on the review
//! donut): the number the caller gives IS the number the body stores.
//!
//! # No semantic fork
//!
//! The body is assembled by the SAME machinery as a revolve —
//! [`super::full`]'s lamina case for [`TubeWindow::Full`],
//! [`super::partial`]'s wedge caps for a window — fed a directly
//! constructed swept traversal (two half-circle arcs whose stored
//! centre/radius are the exact intent values) instead of a validated
//! profile. Classification, sense derivation, seam placement, the
//! ring-torus convention (`R > r > 0`, the `axis_arc_clearance`
//! decide) and the final whole-body pcurve mint are all the revolve's
//! own code, not copies.
//!
//! **No cross-door bit-identity contract**: a tube-door body and a
//! revolve-door body of the same torus may differ by ulps (the
//! revolve derives its radii from profile bulges; this door stores
//! the given ones) — same census, same certified statements, not the
//! same bits.
//!
//! # The hollow sibling
//!
//! [`tube_along_arc_hollow`] is the same door for a tube with a
//! WALL: outer minor radius plus wall thickness, and the annular
//! section built internally the way the solid door builds its
//! circle — a second directly constructed traversal at the inner
//! radius, fed to the same revolve machinery as a hole loop. The
//! solid door's exact-intent posture carries over unchanged for the
//! outer wall (`major_radius`, `minor_radius`, centre, axis, `u_ref`
//! stored verbatim); the inner wall stores `minor_radius - wall`,
//! ONE IEEE subtraction of the caller's own two numbers rather than
//! a profile→bulge→radius reconstruction, so a caller recovers it by
//! writing the same subtraction.
//!
//! Its window policy is the solid door's, mirrored: a
//! [`TubeWindow::Arc`] is an ordinary open elbow with an annular
//! cross-section (two wedge caps, each an annulus), and
//! [`TubeWindow::Full`] closes the inner wall into a CAVITY — a
//! torus shell — inserted through the shared void-insertion door
//! ([`topo::insert_void`]) by the revolve's own holed-profile path,
//! never by a second construction here. The containment evidence
//! that path carries is the annulus's own: the two circles are
//! concentric and `0 < minor_radius - wall < minor_radius` is
//! decided at the door below, so the inner circle is strictly inside
//! the outer in the sketch, and revolution about the shared axis
//! maps that to 3-D verbatim.

use geom_core::k_stats::decide;
use geom_core::predicate::BandError;
use geom_core::{
    Affine3, Band, Bounds, Decide, Indeterminate, Margin, Mat3, Point2, Point3, Real, Sign, Tol,
    Vec2, Vec3,
};

use super::axis::AxisFrame;
use super::{RevolveAxis, RevolveError, Revolved, SweptSeg, full, partial};
use crate::swept::SweptKind;

/// The traversed window of the spine arc.
#[derive(Clone, Copy, Debug)]
pub enum TubeWindow<T> {
    /// The full ring (the donut).
    Full,
    /// The arc from angle `t0` to `t1` (radians about the spine axis
    /// from `u_ref`, right-handed; forward means `t1 > t0`, span
    /// definitely under one period). Wedge caps close the ends.
    Arc {
        /// The window's start angle.
        t0: T,
        /// The window's end angle.
        t1: T,
    },
}

/// Typed refusal of the tube door (D4 ¶3).
#[derive(Clone, Debug)]
pub enum TubeError {
    /// The run's tolerance could not form a classification band.
    Band(BandError),
    /// The spine axis is not unit length at tolerance (the door
    /// STORES the given axis — it never normalizes silently, so a
    /// non-unit axis must refuse instead).
    NonUnitAxis,
    /// The reference direction is not unit length at tolerance (same
    /// store-exactly posture).
    NonUnitURef,
    /// Axis and reference direction are not perpendicular at
    /// tolerance: no honest chart frame stores both exactly.
    FrameNotOrthogonal,
    /// The window is degenerate (zero/sliver span) or reversed.
    DegenerateWindow,
    /// The window reaches (or exceeds) one full period: an exactly
    /// full tube must say [`TubeWindow::Full`].
    FullRangeWindow,
    /// [`tube_along_arc_hollow`] only: the wall thickness is not
    /// definitely positive.
    ///
    /// **Not a metered predicate, deliberately** (the chamfer's
    /// `NonpositiveSize` precedent): whether the caller handed in a
    /// positive thickness is a fact about the REQUEST, not a
    /// geometric quantity of a body, so it takes no `k_stats` name
    /// and no band. A hollow tube with no wall is the solid door's
    /// job or nothing at all.
    NonpositiveWall {
        /// The thickness as handed in, meters: its bracket's low end,
        /// so a straddling or poisoned enclosure reports the end that
        /// fails.
        wall: f64,
    },
    /// [`tube_along_arc_hollow`] only: the wall is not definitely
    /// thinner than the outer minor radius, so `minor_radius - wall`
    /// is not a positive inner radius — there is no annulus to
    /// revolve. Same request-fact posture as
    /// [`TubeError::NonpositiveWall`].
    WallExceedsRadius {
        /// The thickness as handed in, meters (bracket high end).
        wall: f64,
        /// The outer minor radius as handed in, meters (bracket low
        /// end).
        minor_radius: f64,
    },
    /// [`tube_along_arc_hollow`] only: the wall is positive and below
    /// the outer minor radius, yet `minor_radius - wall` does not
    /// come out definitely BELOW `minor_radius` — the thickness is
    /// finer than the outer radius's own representation at this
    /// magnitude, so the two circles would be stored as one. Refused
    /// rather than built as a zero-thickness wall; same request-fact
    /// posture as [`TubeError::NonpositiveWall`].
    WallBelowResolution {
        /// The thickness as handed in, meters (bracket high end).
        wall: f64,
        /// The outer minor radius as handed in, meters (bracket low
        /// end).
        minor_radius: f64,
    },
    /// A frame/window classification escalated.
    Escalated {
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// The shared revolve machinery refused (ring-torus convention,
    /// operator/certification failures, the pcurve mint — every arm
    /// typed on [`RevolveError`]).
    Revolve(RevolveError),
}

impl core::fmt::Display for TubeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Band(e) => write!(f, "tube_along_arc: {e}"),
            Self::NonUnitAxis => write!(
                f,
                "tube_along_arc: the spine axis is not unit length at tolerance — inputs \
                 are stored exactly, so the door refuses rather than normalizing"
            ),
            Self::NonUnitURef => write!(
                f,
                "tube_along_arc: the reference direction is not unit length at tolerance \
                 — inputs are stored exactly, so the door refuses rather than normalizing"
            ),
            Self::FrameNotOrthogonal => write!(
                f,
                "tube_along_arc: axis and reference direction are not perpendicular at \
                 tolerance — no honest chart frame stores both exactly"
            ),
            Self::DegenerateWindow => write!(
                f,
                "tube_along_arc: the arc window is degenerate or reversed (t1 must \
                 definitely exceed t0, metered at the outer-equator arm)"
            ),
            Self::FullRangeWindow => write!(
                f,
                "tube_along_arc: the arc window reaches one full period — an exactly \
                 full tube says TubeWindow::Full"
            ),
            Self::NonpositiveWall { wall } => write!(
                f,
                "tube_along_arc_hollow: the wall thickness {wall} m is not definitely \
                 positive — supply a positive thickness, or call tube_along_arc for the \
                 solid tube. A nonpositive wall is refused as the invalid input it is \
                 rather than reported as a fact about the body"
            ),
            Self::WallExceedsRadius { wall, minor_radius } => write!(
                f,
                "tube_along_arc_hollow: the wall thickness {wall} m is not definitely \
                 thinner than the outer minor radius {minor_radius} m, so \
                 minor_radius - wall is no inner radius and there is no annulus to \
                 revolve — supply a thinner wall, or call tube_along_arc for the solid tube"
            ),
            Self::WallBelowResolution { wall, minor_radius } => write!(
                f,
                "tube_along_arc_hollow: the wall thickness {wall} m is positive but \
                 minor_radius - wall does not fall below the outer minor radius \
                 {minor_radius} m — the wall is finer than that radius's own \
                 representation here, so the two circles would be stored as one. \
                 Supply a thicker wall or a smaller outer radius"
            ),
            Self::Escalated { source } => write!(f, "tube_along_arc escalated: {source}"),
            Self::Revolve(e) => write!(f, "tube_along_arc: {e}"),
        }
    }
}

impl std::error::Error for TubeError {}

/// The solid tube body (module docs). On success the returned body
/// passes tiers 1–3 with stored certified pcurves, exactly as a
/// revolve.
///
/// # Errors
///
/// [`TubeError`] — every door named on the enum except the three
/// wall arms, which only [`tube_along_arc_hollow`] can raise; the
/// ring-torus convention (`R > r > 0`) refuses through the shared
/// `axis_arc_clearance`/`axis_vertex_radius` decides as
/// [`TubeError::Revolve`].
pub fn tube_along_arc<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    major_radius: T,
    window: TubeWindow<T>,
    minor_radius: T,
    tol: Tol,
) -> Result<Revolved<T>, TubeError> {
    build(
        center,
        axis,
        u_ref,
        major_radius,
        window,
        minor_radius,
        None,
        tol,
    )
}

/// The hollow tube body: `minor_radius` is the OUTER minor radius and
/// `wall` the wall thickness, both intent parameters (module docs,
/// "The hollow sibling"). A window builds an open elbow of annular
/// section; [`TubeWindow::Full`] builds a torus shell whose cavity is
/// inserted through the shared void-insertion door.
///
/// Everything the solid door refuses, this door refuses identically —
/// it is the same code with one more loop.
///
/// # Errors
///
/// [`TubeError`] — every door named on the enum. The three wall arms
/// ([`TubeError::NonpositiveWall`], [`TubeError::WallExceedsRadius`],
/// [`TubeError::WallBelowResolution`]) are plain input-validity
/// checks on the request, decided before anything is minted and
/// metered by nothing.
pub fn tube_along_arc_hollow<T: Decide + Bounds>(
    center: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    major_radius: T,
    window: TubeWindow<T>,
    minor_radius: T,
    wall: T,
    tol: Tol,
) -> Result<Revolved<T>, TubeError> {
    // The wall's three request facts, read off the brackets rather
    // than metered (the enum's own notes; the chamfer's
    // `NonpositiveSize` precedent). Written through `partial_cmp` so
    // the INCOMPARABLE case is an arm and not an accident: a poisoned
    // thickness is not definitely positive either, and it refuses
    // with the other two.
    let definitely = |x: f64, ord, y: f64| matches!(x.partial_cmp(&y), Some(o) if o == ord);
    if !definitely(wall.lo(), core::cmp::Ordering::Greater, 0.0) {
        return Err(TubeError::NonpositiveWall { wall: wall.lo() });
    }
    let inner = minor_radius - wall;
    if !definitely(inner.lo(), core::cmp::Ordering::Greater, 0.0) {
        return Err(TubeError::WallExceedsRadius {
            wall: wall.hi(),
            minor_radius: minor_radius.lo(),
        });
    }
    // The separation the annulus's strict containment rests on: the
    // subtraction has to have MOVED the radius. `wall > 0` alone does
    // not give that — a thickness far under the outer radius's ulp
    // rounds `minor_radius - wall` back to `minor_radius`, and two
    // coincident circles are not an annulus.
    if !definitely(inner.hi(), core::cmp::Ordering::Less, minor_radius.lo()) {
        return Err(TubeError::WallBelowResolution {
            wall: wall.hi(),
            minor_radius: minor_radius.lo(),
        });
    }
    build(
        center,
        axis,
        u_ref,
        major_radius,
        window,
        minor_radius,
        Some(inner),
        tol,
    )
}

/// Both doors' body (module docs). `inner_radius` present ⇔ hollow;
/// it is already decided to lie strictly in `(0, minor_radius)` by
/// the hollow door, which is the annulus's containment evidence.
///
/// The bound stays `Decide` — the wall's bracket reads live in the
/// hollow door alone, so the solid door keeps its signature.
#[allow(clippy::too_many_arguments)]
fn build<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    major_radius: T,
    window: TubeWindow<T>,
    minor_radius: T,
    inner_radius: Option<T>,
    tol: Tol,
) -> Result<Revolved<T>, TubeError> {
    let band = Band::linear(tol).map_err(TubeError::Band)?;
    // The angle lever arm: the outer equator (D4 ¶1).
    let arm = major_radius + minor_radius;
    let esc = |source| TubeError::Escalated { source };
    let unit = |v: Vec3<T>, err: TubeError| -> Result<(), TubeError> {
        match decide(
            "tube_frame_unit",
            Margin::levered(v.norm() - T::one(), arm),
            band,
        )
        .map_err(esc)?
        {
            Sign::Zero => Ok(()),
            Sign::Positive | Sign::Negative => Err(err),
        }
    };
    unit(axis, TubeError::NonUnitAxis)?;
    unit(u_ref, TubeError::NonUnitURef)?;
    match decide(
        "tube_frame_orthogonal",
        Margin::levered(axis.dot(u_ref), arm),
        band,
    )
    .map_err(esc)?
    {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => return Err(TubeError::FrameNotOrthogonal),
    }

    // ---- The window (mirrors revolve's angle classification). ----
    let (theta, full) = match window {
        TubeWindow::Full => (T::tau(), true),
        TubeWindow::Arc { t0, t1 } => {
            let span = t1 - t0;
            match decide("tube_window_span", Margin::levered(span, arm), band).map_err(esc)? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(TubeError::DegenerateWindow),
            }
            let headroom = Margin::levered(T::tau() - span, arm);
            match decide("tube_window_headroom", headroom, band).map_err(esc)? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(TubeError::FullRangeWindow),
            }
            (span, false)
        }
    };

    // ---- The sketch frame: origin at the spine centre, sketch x the
    // window-start radial direction, sketch y the axis. The axis line
    // is (0,0)→(0,1) in sketch coordinates, so the frame's world
    // anchors are the given centre/axis/u_ref VERBATIM (`foot3` of
    // the tube centre is the placement origin — zero arithmetic). A
    // window's start direction is the one derived quantity
    // (`u_ref` rotated by t0; exactly `u_ref` for Full and t0 = 0). ----
    let x_dir = match window {
        TubeWindow::Full => u_ref,
        TubeWindow::Arc { t0, .. } => {
            let (s, c) = t0.sin_cos();
            u_ref * c + axis.cross(u_ref) * s
        }
    };
    let normal = x_dir.cross(axis);
    let place = Affine3::from_parts(
        Mat3::from_cols(x_dir, axis, normal),
        center - Point3::origin(),
    );
    let frame = AxisFrame::build(
        place,
        &RevolveAxis {
            origin: Point2::new(T::zero(), T::zero()),
            dir: Vec2::new(T::zero(), T::one()),
        },
        band,
    )
    .map_err(TubeError::Revolve)?;

    // ---- The swept traversals, constructed DIRECTLY (module docs):
    // each tube circle as two half-circle arcs whose stored centre
    // `(R, 0)` and radius are the exact intent values (see
    // [`circle_traversal`] for the reversal involution this hand-
    // copies, and for the sense/labelling of each loop).
    //
    // The outer loop is the canonical CCW boundary REVERSED, which is
    // what a positive-θ sweep traverses. A hollow tube adds the inner
    // circle as the revolve's HOLE loop, in the traversal that
    // construction expects: forward (still clockwise) for a full
    // period, where the hole builds as its own hole-as-outer solid
    // before the void door reverses it; reversed (counterclockwise)
    // for a window, where it is an ordinary ring in the start cap.
    // ----
    let c_sk = Point2::new(major_radius, T::zero());
    let mut loops = vec![circle_traversal(c_sk, minor_radius, Sign::Negative, true)];
    if let Some(inner) = inner_radius {
        loops.push(if full {
            circle_traversal(c_sk, inner, Sign::Negative, false)
        } else {
            circle_traversal(c_sk, inner, Sign::Positive, true)
        });
    }
    let mut classes = Vec::with_capacity(loops.len());
    for (li, segs) in loops.iter().enumerate() {
        classes.push(
            super::axis::classify_loop(segs, &frame, li, true, band).map_err(TubeError::Revolve)?,
        );
    }

    let mut out = if full {
        full::build_full(&frame, &loops, &classes, theta, band, tol)
    } else {
        partial::build_partial(&frame, &loops, &classes, theta, true, band, tol)
    }
    .map_err(TubeError::Revolve)?;
    // The same final pass as every constructor since M6-3: stored
    // certified pcurves at rest.
    topo::mint_pcurves(&mut out.body, tol)
        .map_err(|e| TubeError::Revolve(RevolveError::Pcurve(e)))?;
    Ok(out)
}

/// The circle of `radius` about `center` as a two-arc swept
/// traversal: the half-circle from `(cx + radius, 0)` to
/// `(cx - radius, 0)` and the one back, with `center` and `radius`
/// stored as given.
///
/// A hand-written COPY OF `swept::swept_segments` for a known two-arc
/// input instead of a call: that builder takes a `ValidatedLoop`,
/// whose arc centre and radius come back from bulge arithmetic, and
/// storing the caller's numbers instead of reconstructing them is
/// this door's entire reason to exist (module docs). So the
/// convention is shared with `swept_segments` and the code is not:
/// **a change to that builder is a change to this function.**
/// (Phrased with the marker vocabulary on purpose: a duplication
/// declared in words the tree's own greps do not carry is a
/// duplication nothing will find. S131.)
///
/// The two arguments are the two bits `swept_segments` carries. `turn`
/// is the TRAVERSAL's own sense (its bulge follows: `+1` for a
/// positive half-turn, `-1` for a negative one). `reversed` says
/// whether this traversal is the reversal of its canonical chain,
/// which is what permutes the canonical labels — the involution's
/// `(n - j) % n` / `n - 1 - j` written out for `n = 2`. The three
/// combinations the doors use: the outer boundary (canonical
/// counterclockwise, reversed for a positive-θ sweep) is
/// `(Negative, true)`; a full period's hole loop (canonical
/// clockwise, traversed forward) is `(Negative, false)`; a window's
/// hole loop (canonical clockwise, reversed with the outer) is
/// `(Positive, true)`.
fn circle_traversal<T: Real>(
    center: Point2<T>,
    radius: T,
    turn: Sign,
    reversed: bool,
) -> Vec<SweptSeg<T>> {
    let (lo, hi) = (
        Point2::new(center.x - radius, T::zero()),
        Point2::new(center.x + radius, T::zero()),
    );
    let bulge = match turn {
        Sign::Positive => T::one(),
        Sign::Negative | Sign::Zero => T::zero() - T::one(),
    };
    let arc = |a, b, canonical_vertex, canonical_segment| SweptSeg {
        a,
        b,
        bulge,
        kind: SweptKind::Arc {
            center,
            radius,
            turn,
        },
        canonical_vertex,
        canonical_segment,
    };
    if reversed {
        vec![arc(hi, lo, 0, 1), arc(lo, hi, 1, 0)]
    } else {
        vec![arc(hi, lo, 0, 0), arc(lo, hi, 1, 1)]
    }
}
