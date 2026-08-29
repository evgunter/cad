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
//! concentric, and the door decides the thickness, the bore AND the
//! realized gap between the two radii the walls will store definitely
//! positive at the run's band before anything is minted, so the inner
//! circle is strictly inside the outer in the sketch — and revolution
//! about the shared axis maps that to 3-D verbatim.

use geom_core::k_stats::decide;
use geom_core::predicate::BandError;
use geom_core::{
    Affine3, Band, Decide, Indeterminate, Margin, Mat3, Point2, Point3, Real, Sign, Tol, Vec2, Vec3,
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
    /// definitely positive at tolerance — a wall thinner than the
    /// run's ε is not a wall.
    ///
    /// The payload is the floor the thickness had to clear, not the
    /// thickness itself: under this door's `T: Decide` bound there is
    /// no f64 door out of a `T` (that is `Bounds`, and the
    /// compound-`Bounds` seam rule keeps it out of this file), so the
    /// arm reports the number it CAN state exactly. The chamfer's
    /// `NonpositiveSize` reports the caller's value because its file
    /// is an allowlisted seam.
    NonpositiveWall {
        /// The run's coincidence threshold, meters: a thickness at or
        /// under this is not a wall.
        eps: f64,
    },
    /// [`tube_along_arc_hollow`] only: `minor_radius - wall` is not a
    /// definitely positive inner radius, so there is no bore and no
    /// annulus to revolve.
    ///
    /// Payload as [`TubeError::NonpositiveWall`]: the floor, not the
    /// value.
    WallExceedsRadius {
        /// The run's coincidence threshold, meters: an inner radius
        /// at or under this is not a bore.
        eps: f64,
    },
    /// [`tube_along_arc_hollow`] only: the wall is a wall and the bore
    /// is a bore, yet the REALIZED gap between the two stored radii —
    /// `minor_radius - (minor_radius - wall)` — is not definitely
    /// positive.
    ///
    /// The requested wall and the wall the body would store are not
    /// the same number: at large `minor_radius` a thickness above ε
    /// can still fall under that radius's own ulp, so the subtraction
    /// rounds the inner radius back ONTO the outer and the two
    /// circles would be stored as one. Deciding `wall` and the bore
    /// separately cannot see it — only the difference of the two
    /// STORED radii can, which is what this arm meters
    /// (`tube_wall_gap`). It is also what the full period's cavity
    /// evidence rests on: without it `Carried { Positive }` would be
    /// carried for a pair of coincident circles.
    ///
    /// Payload as [`TubeError::NonpositiveWall`]: the floor, not the
    /// value.
    WallGapCollapsed {
        /// The run's coincidence threshold, meters: the realized gap
        /// between the two stored radii did not clear it.
        eps: f64,
    },
    /// A frame/window/wall classification escalated.
    Escalated {
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// The shared revolve machinery refused (ring-torus convention,
    /// operator/certification failures, the pcurve mint — every arm
    /// typed on [`RevolveError`]).
    Revolve(RevolveError),
}

/// The `k_stats` names only [`tube_along_arc_hollow`] can reach. An
/// escalation carrying one of these came from the hollow door, and
/// its message must say so — the solid door cannot produce it.
const HOLLOW_PREDICATES: [&str; 3] = ["tube_wall", "tube_wall_bore", "tube_wall_gap"];

/// Which door a refusal belongs to, as far as the refusal itself can
/// honestly say.
///
/// A wall escalation names the hollow door outright. Everything else
/// on this enum is reachable through BOTH doors — the band is the
/// run's, the frame and window predicates are shared verbatim, and
/// the revolve machinery is one body of code — so those arms say
/// "tube door" rather than picking one and being wrong half the time.
/// (The alternative, threading a hollow flag onto every arm, would
/// put the door's identity in the payload of refusals that do not
/// depend on it.)
fn door(e: &TubeError) -> &'static str {
    match e {
        TubeError::Escalated { source } => match source.predicate {
            Some(p) if HOLLOW_PREDICATES.contains(&p) => "tube_along_arc_hollow",
            _ => "tube door",
        },
        TubeError::NonpositiveWall { .. }
        | TubeError::WallExceedsRadius { .. }
        | TubeError::WallGapCollapsed { .. } => "tube_along_arc_hollow",
        _ => "tube door",
    }
}

impl core::fmt::Display for TubeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let door = door(self);
        match self {
            Self::Band(e) => write!(f, "{door}: {e}"),
            Self::NonUnitAxis => write!(
                f,
                "{door}: the spine axis is not unit length at tolerance — inputs \
                 are stored exactly, so the door refuses rather than normalizing"
            ),
            Self::NonUnitURef => write!(
                f,
                "{door}: the reference direction is not unit length at tolerance \
                 — inputs are stored exactly, so the door refuses rather than normalizing"
            ),
            Self::FrameNotOrthogonal => write!(
                f,
                "{door}: axis and reference direction are not perpendicular at \
                 tolerance — no honest chart frame stores both exactly"
            ),
            Self::DegenerateWindow => write!(
                f,
                "{door}: the arc window is degenerate or reversed (t1 must \
                 definitely exceed t0, metered at the outer-equator arm)"
            ),
            Self::FullRangeWindow => write!(
                f,
                "{door}: the arc window reaches one full period — an exactly \
                 full tube says TubeWindow::Full"
            ),
            Self::NonpositiveWall { eps } => write!(
                f,
                "{door}: the wall thickness is not definitely positive at \
                 tolerance (metered at tube_wall; the run's threshold is {eps} m) — a wall \
                 thinner than that is not a wall. Supply a thicker one, or call \
                 tube_along_arc for the solid tube"
            ),
            Self::WallExceedsRadius { eps } => write!(
                f,
                "{door}: minor_radius - wall is not a definitely positive \
                 inner radius at tolerance (metered at tube_wall_bore; the run's threshold \
                 is {eps} m), so there is no bore and no annulus to revolve — supply a \
                 thinner wall, or call tube_along_arc for the solid tube"
            ),
            Self::WallGapCollapsed { eps } => write!(
                f,
                "{door}: the wall is positive and the bore is positive, but \
                 the gap between the two radii the body would STORE is not (metered at \
                 tube_wall_gap; the run's threshold is {eps} m) — at this outer radius the \
                 subtraction minor_radius - wall rounds back onto minor_radius, so the two \
                 circles would be stored as one. Supply a thicker wall, or a smaller outer \
                 radius"
            ),
            Self::Escalated { source } => write!(f, "{door} escalated: {source}"),
            Self::Revolve(e) => write!(f, "{door}: {e}"),
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
/// [`TubeError`] — every door named on the enum except the three wall
/// arms, which only [`tube_along_arc_hollow`] can raise; the
/// ring-torus convention (`R > r > 0`) refuses through the shared
/// `axis_arc_clearance`/`axis_vertex_radius` decides as
/// [`TubeError::Revolve`].
pub fn tube_along_arc<T: Decide + geom_brep::PcurveFittedLane>(
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
/// [`TubeError::WallGapCollapsed`]) are decided FIRST, before
/// anything is minted, and their verdicts are what the full period's
/// cavity insertion carries as its containment evidence.
// The solid door's seven intent parameters plus the wall — the list
// IS the door, and bundling any subset of it into a struct would hide
// which numbers the body stores verbatim.
#[allow(clippy::too_many_arguments)]
pub fn tube_along_arc_hollow<T: Decide + geom_brep::PcurveFittedLane>(
    center: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    major_radius: T,
    window: TubeWindow<T>,
    minor_radius: T,
    wall: T,
    tol: Tol,
) -> Result<Revolved<T>, TubeError> {
    build(
        center,
        axis,
        u_ref,
        major_radius,
        window,
        minor_radius,
        Some(wall),
        tol,
    )
}

/// Both doors' body (module docs). `wall` present ⇔ hollow.
#[allow(clippy::too_many_arguments)]
fn build<T: Decide + geom_brep::PcurveFittedLane>(
    center: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    major_radius: T,
    window: TubeWindow<T>,
    minor_radius: T,
    wall: Option<T>,
    tol: Tol,
) -> Result<Revolved<T>, TubeError> {
    let band = Band::linear(tol).map_err(TubeError::Band)?;
    // The angle lever arm: the outer equator (D4 ¶1).
    let arm = major_radius + minor_radius;
    let esc = |source| TubeError::Escalated { source };

    // ---- The wall, decided FIRST: nothing is minted behind it, and
    // the verdicts here are the annulus's containment evidence (fn
    // docs of the hollow door). Both margins are LENGTHS in meters,
    // so they take the plain linear band — no lever arm, unlike this
    // door's angular window and frame margins.
    //
    // Metered rather than bracket-read: the door already meters its
    // caller-supplied WINDOW the same way (`tube_window_span`), and a
    // thickness is not a mere request flag — `minor_radius - wall` is
    // the inner wall's stored radius, a geometric quantity of the
    // body, and "is the inner circle strictly inside the outer" is
    // exactly the kind of fact the funnel exists to decide.
    //
    // THREE decides, not two, and the third is not redundant: `wall`
    // and the bore are facts about the numbers the caller supplied,
    // while `tube_wall_gap` is a fact about the two radii the body
    // would STORE. They come apart — at a large outer radius a
    // thickness well above ε still falls under that radius's own ulp,
    // both of the first two decide Positive, and the subtraction
    // rounds the inner radius onto the outer. Nothing downstream may
    // be relied on to catch it: the observed refusals for that class
    // come from the pcurve mint and the cap-plane Newell fit, AFTER
    // the frame, both classifications and the mint have run, and the
    // cavity's `Carried { Positive }` would by then have been carried
    // for a pair of coincident circles. ----
    let eps = band.zero();
    let inner_radius = match wall {
        None => None,
        Some(wall) => {
            match decide("tube_wall", Margin::of(wall), band).map_err(esc)? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => {
                    return Err(TubeError::NonpositiveWall { eps });
                }
            }
            let inner = minor_radius - wall;
            match decide("tube_wall_bore", Margin::of(inner), band).map_err(esc)? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => {
                    return Err(TubeError::WallExceedsRadius { eps });
                }
            }
            // The REALIZED gap: `minor_radius - inner`, the difference
            // of the two numbers the walls will store, not of the two
            // the caller wrote.
            match decide("tube_wall_gap", Margin::of(minor_radius - inner), band).map_err(esc)? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => {
                    return Err(TubeError::WallGapCollapsed { eps });
                }
            }
            Some(inner)
        }
    };

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
