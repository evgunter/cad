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

use geom_core::k_stats::decide;
use geom_core::predicate::BandError;
use geom_core::{
    Affine3, Band, Decide, Indeterminate, Margin, Mat3, Point2, Point3, Sign, Tol, Vec2, Vec3,
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
            Self::Escalated { source } => write!(f, "tube_along_arc escalated: {source}"),
            Self::Revolve(e) => write!(f, "tube_along_arc: {e}"),
        }
    }
}

impl std::error::Error for TubeError {}

/// The tube body (module docs). On success the returned body passes
/// tiers 1–3 with stored certified pcurves, exactly as a revolve.
///
/// # Errors
///
/// [`TubeError`] — every door named on the enum; the ring-torus
/// convention (`R > r > 0`) refuses through the shared
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

    // ---- The swept traversal, constructed DIRECTLY (module docs):
    // the full tube circle as two half-circle arcs whose stored
    // centre `(R, 0)` and radius `r` are the exact intent values, in
    // the REVERSED order the positive-θ sweep traverses.
    //
    // A hand-written COPY OF `swept::swept_segments`' reversal
    // involution — endpoints swapped, bulge negated, turn flipped —
    // written out for a known two-arc input instead of computed.
    // (Phrased with the marker vocabulary on purpose: a duplication
    // declared in words the tree's own greps do not carry is a
    // duplication nothing will find. S131.)
    // It cannot call the shared builder: that takes a `ValidatedLoop`,
    // whose arc centre and radius come back from bulge arithmetic,
    // and storing the caller's numbers instead of reconstructing them
    // is this door's entire reason to exist (module docs). So the
    // convention is shared with `swept_segments` and the code is not:
    // **a change to that involution is a change to these constants.**
    // ----
    let (rr, r) = (major_radius, minor_radius);
    let c_sk = Point2::new(rr, T::zero());
    let (lo, hi) = (
        Point2::new(rr - r, T::zero()),
        Point2::new(rr + r, T::zero()),
    );
    let arc = |a, b, canonical_vertex, canonical_segment| SweptSeg {
        a,
        b,
        bulge: T::zero() - T::one(),
        kind: SweptKind::Arc {
            center: c_sk,
            radius: r,
            turn: Sign::Negative,
        },
        canonical_vertex,
        canonical_segment,
    };
    let segs = vec![arc(hi, lo, 0, 1), arc(lo, hi, 1, 0)];
    let classes =
        super::axis::classify_loop(&segs, &frame, 0, true, band).map_err(TubeError::Revolve)?;

    let loops = [segs];
    let classes_arr = [classes];
    let mut out = if full {
        full::build_full(&frame, &loops, &classes_arr, theta, band, tol)
    } else {
        partial::build_partial(&frame, &loops, &classes_arr, theta, true, band, tol)
    }
    .map_err(TubeError::Revolve)?;
    // The same final pass as every constructor since M6-3: stored
    // certified pcurves at rest.
    topo::mint_pcurves(&mut out.body, tol)
        .map_err(|e| TubeError::Revolve(RevolveError::Pcurve(e)))?;
    Ok(out)
}
