//! The f64-first seam.
//!
//! The kernel is generic over [`Real`] because evaluation must be able
//! to run at certified intervals and at dual numbers, not only at
//! `f64`. Authoring code does not care: a scene wants to write
//! `1.5`, not `S::from_f64(1.5)`, and a coordinate pair, not a
//! turbofished constructor.
//!
//! This module is that seam and nothing more. Every function here
//! takes plain `f64` literals and embeds them with [`Real::from_f64`]
//! — an *exact* embedding for every implementor, so nothing here can
//! change a number. Five of the seven are one kernel constructor
//! call; the other two are the seam's fallible pair — [`validated`]
//! is `Profile::new` + `Profile::validate`, and [`polygon`] is the
//! lattice chain a coordinate table lowers to. There is no
//! arithmetic, no defaulting, and no panicking: both return the
//! kernel's own `Result` with the kernel's own error.
//!
//! Before this seam existed, the demo corpus carried six
//! near-identical `p2` helpers and four `validated` wrappers, one per
//! scene. Those are now gone — deleted in favor of the forms below.
//! # What this seam does *not* yet do
//!
//! It offers an f64-first door; it does not by itself remove the
//! conversion tax from code already written against the generic
//! constructors. The tour still spells `S::from_f64` 150 times
//! (measured 2026-08-09 across `demos/tour/src`), because its scene
//! bodies stay generic over the scalar so one source runs at `f64`
//! and at certified intervals — those conversions live in code that
//! deliberately is not f64-only. What landed here is the door and the
//! deletion of the per-scene helper duplication, not a corpus-wide
//! sweep. Read any claim that "the `from_f64` tax is paid once" as
//! being about *this seam's* call sites, not about the corpus.
//!
//! The generic parameter stays: these are the *entry* points, so the
//! same source still instantiates at `f64` for a normal build and at a
//! telemetry or interval scalar for a certified one.

use ::profile::{
    Open, PathError, Profile, ProfileError, ProfileLoop, SketchPlane, Start, ValidatedProfile,
};
use geom_core::Tol;
use geom_core::{Decide, Point2, Point3, Real, Vec2, Vec3};

/// Embeds an `f64` literal as the working scalar.
///
/// The one-argument case of everything else in this module — for
/// scalar operands such as an extrusion distance.
///
/// ```
/// use pncad::prelude::*;
///
/// let d: f64 = real(0.75);
/// assert_eq!(d, 0.75);
/// let body = Extrusion::Distance(real::<f64>(0.75));
/// assert!(matches!(body, Extrusion::Distance(_)));
/// ```
#[inline]
pub fn real<T: Real>(x: f64) -> T {
    T::from_f64(x)
}

/// A 2-D sketch point from plain coordinates.
///
/// ```
/// use pncad::prelude::*;
///
/// let p = p2::<f64>(1.5, -2.25);
/// assert_eq!((p.x, p.y), (1.5, -2.25));
/// ```
#[inline]
pub fn p2<T: Real>(x: f64, y: f64) -> Point2<T> {
    Point2::new(T::from_f64(x), T::from_f64(y))
}

/// A 3-D point from plain coordinates.
///
/// ```
/// use pncad::prelude::*;
///
/// let p = p3::<f64>(0.0, 1.0, -0.5);
/// assert_eq!((p.x, p.y, p.z), (0.0, 1.0, -0.5));
/// ```
#[inline]
pub fn p3<T: Real>(x: f64, y: f64, z: f64) -> Point3<T> {
    Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}

/// A 2-D vector from plain components.
///
/// The sketch-plane companion to [`p2`] — a direction or offset in
/// sketch coordinates rather than a position.
///
/// ```
/// use pncad::prelude::*;
///
/// let v = v2::<f64>(3.0, 4.0);
/// assert_eq!((v.x, v.y), (3.0, 4.0));
/// ```
#[inline]
pub fn v2<T: Real>(x: f64, y: f64) -> Vec2<T> {
    Vec2::new(T::from_f64(x), T::from_f64(y))
}

/// A 3-D vector from plain components.
///
/// ```
/// use pncad::prelude::*;
///
/// let up = v3::<f64>(0.0, 0.0, 1.0);
/// assert_eq!((up.x, up.y, up.z), (0.0, 0.0, 1.0));
/// ```
#[inline]
pub fn v3<T: Real>(x: f64, y: f64, z: f64) -> Vec3<T> {
    Vec3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}

/// A closed polygon from a coordinate table, authored through the
/// PATHS lattice.
///
/// `Open.at(p0)`, a [`line_to`](crate::prelude::PartialPath::line_to) per subsequent
/// vertex, and `line_to(Start)` as the sharp seam — the whole table
/// said as the lattice says it, so every corner is CLASSIFIED at
/// authoring. What that buys, and why this door is fallible where a
/// raw vertex table would not be:
///
/// - A corner whose departure lies within ε_input of the incoming
///   tangent, or of its reverse, refuses here
///   ([`PathError::JunctionTangent`], [`PathError::JunctionCusp`]) —
///   including at the seam, whose two junction checks run with both
///   directions known. A raw table cannot refuse: it carries no
///   junction, so the same geometry reaches
///   [`validate`](Profile::validate) as a loop that has
///   already been minted, and the refusal arrives a tier later and
///   further from the coordinates that caused it.
/// - Fewer than three vertices refuses
///   ([`PathError::PolygonTooFewVertices`]): a closed chain of
///   straight legs bounds nothing with fewer corners.
///
/// The emitted [`ProfileLoop`] is the authored table verbatim —
/// every point in order, bulge 0, no declared joints. The lattice
/// changes what is CHECKED, not what is minted.
///
/// Fails loud: the typed [`PathError`] is returned unchanged. A
/// demo may `.expect()` it — a library must not.
///
/// ```
/// use geom_core::Tol;
/// use pncad::prelude::*;
///
/// let tol = Tol::witness();
/// let square: ProfileLoop<f64> =
///     polygon(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], tol)?;
/// assert_eq!(square.vertices().len(), 4);
/// assert!(square.tangent_joints().is_empty());
///
/// // Three corners is the minimum a closed chain of straight legs
/// // can bound anything with.
/// let two = polygon::<f64>(&[(0.0, 0.0), (1.0, 0.0)], tol);
/// assert!(matches!(
///     two,
///     Err(PathError::PolygonTooFewVertices { given: 2 })
/// ));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn polygon<T: Decide>(points: &[(f64, f64)], tol: Tol) -> Result<ProfileLoop<T>, PathError<T>> {
    let too_few = || PathError::PolygonTooFewVertices {
        given: points.len(),
    };
    let [first, second, third, rest @ ..] = points else {
        return Err(too_few());
    };
    let at = |&(x, y): &(f64, f64)| Point2::new(T::from_f64(x), T::from_f64(y));
    let mut tip = Open
        .at(at(first))
        .line_to(at(second), tol)?
        .line_to(at(third), tol)?;
    for q in rest {
        tip = tip.line_to(at(q), tol)?;
    }
    Ok(tip.line_to(Start, tol)?.into())
}

/// Validates a profile at the ambient tolerance — the authoring
/// ladder's first rung.
///
/// Equivalent to `Profile::new(plane, loops).validate(tol)`,
/// which is the form every scene wrote by hand. The tolerance comes
/// from the environment ([`Tol::witness`]) so that a corpus can be
/// replayed at a different ε without editing a line; pass a tolerance
/// explicitly through [`Profile::validate`] when a call site needs to
/// pin one.
///
/// Fails loud: the typed [`ProfileError`] is returned unchanged. A
/// demo may `.expect()` it — a library must not.
///
/// The [`Decide`] bound is the kernel's own: validation makes
/// *decisions* (is this segment tangent? is this loop closed?), and
/// only a scalar that can answer trilean predicates may be asked.
///
/// ```
/// use geom_core::Tol;
/// use pncad::prelude::*;
///
/// let tol = Tol::witness();
/// let square: ClosedLoop<f64> = Open
///     .at(p2(0.0, 0.0))
///     .line_to(p2(1.0, 0.0), tol)?
///     .line_to(p2(1.0, 1.0), tol)?
///     .line_to(p2(0.0, 1.0), tol)?
///     .line_to(Start, tol)?;
/// let profile = validated(SketchPlane::<f64>::xy(), vec![square.into()], tol)?;
/// assert_eq!(profile.loops().len(), 1);
///
/// // Fail-loud: a degenerate profile refuses with a typed error
/// // rather than panicking or silently repairing itself.
/// let empty = validated(SketchPlane::<f64>::xy(), vec![], tol);
/// assert!(empty.is_err());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn validated<T: Decide>(
    plane: SketchPlane<T>,
    loops: Vec<ProfileLoop<T>>,
    tol: Tol,
) -> Result<ValidatedProfile<T>, ProfileError> {
    Profile::new(plane, loops).validate(tol)
}
