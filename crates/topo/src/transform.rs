//! Rigid placement of a whole body (M4 PR 2, spec D3's Transform
//! clause): apply an isometry to every geometric datum of a [`Body`]
//! while leaving the topology — and every arena key — untouched. The
//! same map, at the improper hand, is the geometry half of a mirror
//! ([`crate::mirror`]); everything below is stated for the rigid door
//! and says where the reflection differs.
//!
//! Landed here (not in editor-core) per the spec's report-or-land
//! path: no public rigid-transform op existed on `Body`, and bolting
//! geometry mapping into the editor layer would violate G1 layering.
//!
//! # Contract
//!
//! The map is an [`Isometry`], exact apart from rounding: a
//! [`RigidMotion`] (rigidity carried by the witness it was minted as,
//! or admitted within rounding at the `f64` door,
//! [`RigidMotion::check_rigid`]) or a [`crate::Reflection`] (a
//! Householder matrix decided at its own door). A scale has no spelling
//! as either; a uniform one is affinely self-consistent, so no check on
//! the image would catch it, and the type is what keeps it out.
//!
//! Every certificate is **carried**, never re-derived. A certificate
//! states distances (an edge's `{samples, max_residual}`, a pcurve
//! row's norms, an offset fit's limbs) and a turn in the carrier's own
//! parameter, and an isometry keeps every distance, so each certificate
//! of the source is one of the image. It keeps every parameter too, but
//! for one rescale rounding forces: a line whose image direction a chain
//! of maps carried past its rounding is squared, and its parameter, arc
//! length, stretched by the length divided by. That edge's span, its
//! description's chart image and its half-edges' pcurve rows are read on
//! the stretched parameter ([`map_carrier_stretched`]).
//! The map decides nothing: [`transform_rigid`] and [`crate::mirror`]
//! return construction state, which the finish that follows decides,
//! and a finished body's own door ([`crate::FinishedBody::moved`])
//! carries its verdict as well.
//!
//! # What maps how
//!
//! - points: `p ↦ map(p)`;
//! - surfaces: origins/centers/apexes by the full affine map,
//!   direction frames (`axis`, `u_ref`, `normal`) by the linear part,
//!   radii/angles untouched (rigid invariants);
//! - curve carriers: same treatment per [`Curve3`] variant;
//! - edge descriptions: [`EdgeDescription::Intersection`] keeps its
//!   surface keys (the arenas are key-stable) and **RE-MINTS its
//!   witness** from the mapped carrier at the pinned mid parameter, the
//!   formula construction uses (ruled with Ev on PR #83), never the
//!   mapped stored witness: one evaluation, no decision;
//!   [`MappedCurve`] pre-composes the isometry into its rigid
//!   placement (`place ↦ map ∘ place`) and maps its world-space
//!   vectors/axis data — sketch-space payloads are untouched;
//! - a `Nurbs` carrier or surface maps by its CONTROL POINTS, with knots and weights carried over verbatim: a rigid map
//!   is affine, and the Euclidean-storage section of `geom`'s
//!   `curves::nurbs` data model is where the commutation that makes
//!   the mapped net the exact IMAGE — never a re-fit — is argued;
//! - a pending surface slot has no geometry and stays pending;
//! - approximating surfaces: the offset description's base net and the
//!   fit net by the full affine map, `d` negated under a reflection,
//!   the window and the certificate carried (`map_approx`).

use std::sync::Arc;

use geom::Curve3;
use geom::Surface;
use geom_brep::{EdgeDescription, MappedCurve, MappedSource};
use geom_core::{Affine3, Decide, Real, RigidMotion, Vec3};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::EdgeKey;
use crate::geometry::{CurveKey, SurfaceSlot};
use crate::null::CurveGeom;

/// Typed failure of [`transform_rigid`] (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum TransformError {
    /// The body carries a null-scaffold edge: what an unfinished edit
    /// leaves behind (a lone `mev_null` on a fresh face, say). At a
    /// build, a caller reaches it by moving a body in the middle of its
    /// own edit, so it ends in that edit's lever
    /// ([`crate::SCAFFOLDING_RECOURSE`]). Tier 2 refuses it
    /// at rest, and the import door mints none, so read there it is a
    /// stored contradiction and ends in the door's defect ending.
    NullScaffold {
        /// The offending edge.
        edge: EdgeKey,
    },
    /// The body is corrupt (the validators would refuse it too): a
    /// reference that does not resolve, an arena entry nothing
    /// references, or an image no door builds.
    Corrupt {
        /// What the walk found, as a clause.
        found: &'static str,
    },
}

impl TransformError {
    /// This refusal's text, ended at `at` where it is `Some`: a nested
    /// pcurve refusal reads at that door, a defect ends in the door's
    /// defect ending ([`geom_brep::recourse::defect_ending`]),
    /// and scaffolding ends in its edit's lever at a build. `None` writes
    /// the payload alone, its `Display`.
    fn write(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        at: Option<geom_brep::recourse::ReadAt>,
    ) -> core::fmt::Result {
        use geom_brep::recourse::ReadAt;
        let defect = at.map(geom_brep::recourse::defect_ending);
        let ended = |defect: Option<&str>| defect.map(|d| format!(". {d}")).unwrap_or_default();
        match self {
            Self::NullScaffold { .. } if at.is_some_and(ReadAt::is_build) => write!(
                f,
                "the body still carries what an edit left behind, which the map cannot \
                 move. Recourse: {}",
                crate::SCAFFOLDING_RECOURSE
            ),
            Self::NullScaffold { edge } => match defect {
                Some(defect) => write!(
                    f,
                    "edge {edge:?} carries a transient null-scaffold curve, which a body at \
                     rest never does. {defect}"
                ),
                None => write!(
                    f,
                    "edge {edge:?} carries a transient null-scaffold curve, which the map \
                     cannot move"
                ),
            },
            Self::Corrupt { found } => {
                write!(f, "the body is corrupt: {found}{}", ended(defect))
            }
        }
    }
}

geom_brep::ended_by_write!(TransformError);

// No `source()`, matching every error type in this crate: each arm's
// `Display` renders its nested payload's own `Display` in full, so the
// chain is already in the message and a walker would re-read what it
// just printed.
impl std::error::Error for TransformError {}

fn map_vec<T: Real>(map: &Affine3<T>, v: Vec3<T>) -> Vec3<T> {
    map.linear * v
}

/// Whether an isometry keeps orientation ([`Hand::Proper`], a rigid
/// motion) or reverses it ([`Hand::Improper`], a reflection —
/// [`crate::mirror`]). Under an improper map every datum is written so
/// the result is the INSIDE-OUT image: a plane's stored normal, an
/// analytic face's outward side and every carrier's parameter are
/// those of the reflected body turned inside out, which
/// [`crate::mirror`]'s reversal then turns right side out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Hand {
    /// Determinant `+1`.
    Proper,
    /// Determinant `−1`.
    Improper,
}

/// **An isometry exact apart from rounding** (`[topology.tiers]`): a
/// rigid motion, or a reflection across a plane. These are the only two
/// maps a finished body's verdict survives ([`crate::FinishedBody::moved`]):
/// every finding but the session box decides a fact of the shape up to
/// isometry. A map that is not one (a scale, a shear, an offset) has no
/// spelling here, so its image is construction state and only
/// [`crate::finish()`] makes it a finished body: a scale changes every
/// length the gate compares against ε.
///
/// Each arm is exact apart from rounding at its door: a motion is
/// minted by its own constructors, or admitted within rounding with its
/// bits kept ([`RigidMotion::check_rigid`]); a reflection is decided at
/// [`crate::Reflection::across`].
#[derive(Clone, Copy, Debug)]
pub enum Isometry<T: Real> {
    /// A rigid motion (determinant `+1`).
    Proper(RigidMotion<T>),
    /// A reflection (determinant `−1`).
    Improper(crate::Reflection<T>),
}

impl<T: Real> Isometry<T> {
    /// The map as an affine one.
    #[must_use]
    pub fn to_affine(&self) -> Affine3<T> {
        match self {
            Self::Proper(motion) => motion.to_affine(),
            Self::Improper(reflection) => reflection.to_affine(),
        }
    }

    /// The map as a rigid motion, where it is one.
    fn motion(&self) -> Option<&RigidMotion<T>> {
        match self {
            Self::Proper(motion) => Some(motion),
            Self::Improper(_) => None,
        }
    }

    /// Whether the map keeps orientation.
    fn hand(&self) -> Hand {
        match self {
            Self::Proper(_) => Hand::Proper,
            Self::Improper(_) => Hand::Improper,
        }
    }
}

/// Every datum of one surface under `map` at `hand` (module docs: what
/// maps how).
///
/// **Sense** (M5 S10 audit). Under [`Hand::Proper`] every face's `sense`
/// is copied unchanged, and correctly so: a rigid motion carries the
/// chart normal to the transformed chart normal and the material side
/// along with it, so the bit's *meaning* ("does the material side agree
/// with the chart normal?") survives because both move together. Under
/// [`Hand::Improper`] the map writes the inside-out image ([`Hand`]): a
/// plane's stored normal, its outward side, is the axial `−M·n`; an
/// analytic chart's frame maps forward, so its chart normal crosses as
/// `M·n` while the material side crosses inside out, and
/// [`map_isometry`] flips that face's `sense`; a net's chart normal
/// crosses as `−M·n`, already inside out, and its `sense` is kept.
/// [`crate::Body::revert`]'s map then reverses every loop's cycle and
/// every outward side once ([`crate::mirror`]).
///
/// `motion` is the map as the rigid motion it is, where it is one: an
/// analytic frame is then that frame moved ([`geom::Frame::moved`]), a
/// rigid image of a rigid frame, as [`map_carrier_stretched`] moves a conic's.
/// Under a reflection, where `motion` is `None`, the frame's image is the
/// reflected frame ([`geom::Frame::reflected`]), the plane's normal as the
/// axial image.
/// **A carrier's frame under the map**: under a proper motion the
/// frame's own image ([`geom::Frame::moved`]); under a reflection its
/// reflected image ([`geom::Frame::reflected`]), kept right-handed by
/// reversing its `w` where `axial` (a plane's normal, a conic's axis, so
/// the image keeps its side and its parameter) and its `v` elsewhere.
fn map_frame<T: Real>(
    map: &Affine3<T>,
    hand: Hand,
    motion: Option<&RigidMotion<T>>,
    f: &geom::Frame<T>,
    axial: bool,
) -> geom::Frame<T> {
    match motion {
        Some(motion) => f.moved(motion),
        // The axis is the axial image `−M·a` where `axial` (`(−M·a) × (M·u)
        // = M·(a × u)` when `det M = −1`, so the turning is kept), so the
        // frame's `w` is reversed; elsewhere the axis is `M·a` and the parameter
        // keeps `M·u`, so `v` is. Either is the frame `Frame::new` squared
        // from those data, held as a rigid image rather than squared.
        None => {
            debug_assert!(
                matches!(hand, Hand::Improper),
                "a rigid map carries its motion"
            );
            f.reflected(
                map,
                if axial {
                    geom_core::Negated::W
                } else {
                    geom_core::Negated::V
                },
            )
        }
    }
}

fn map_surface<T: Real>(
    map: &Affine3<T>,
    hand: Hand,
    motion: Option<&RigidMotion<T>>,
    s: &Surface<T>,
) -> Result<Surface<T>, TransformError> {
    let mapped = |f: &geom::Frame<T>, axial: bool| map_frame(map, hand, motion, f, axial);
    Ok(match *s {
        // The normal is the face's outward side, so the inside-out
        // image's is the axial one; the chart is then `M∘S` exactly.
        Surface::Plane {
            frame: carrier_frame,
        } => Surface::Plane {
            frame: mapped(&carrier_frame, true),
        },
        Surface::Cylinder {
            frame: carrier_frame,
            radius,
        } => Surface::Cylinder {
            frame: mapped(&carrier_frame, false),
            radius,
        },
        Surface::Cone {
            frame: carrier_frame,
            half_angle,
        } => Surface::Cone {
            frame: mapped(&carrier_frame, false),
            half_angle,
        },
        Surface::Sphere {
            frame: carrier_frame,
            radius,
        } => Surface::Sphere {
            frame: mapped(&carrier_frame, false),
            radius,
        },
        Surface::Torus {
            frame: carrier_frame,
            major_radius,
            minor_radius,
        } => Surface::Torus {
            frame: mapped(&carrier_frame, false),
            major_radius,
            minor_radius,
        },
        Surface::Spindle {
            frame: carrier_frame,
            major_radius,
            minor_radius,
        } => Surface::Spindle {
            frame: mapped(&carrier_frame, false),
            major_radius,
            minor_radius,
        },
        // A net maps by its control points, weights and knots
        // untouched — the exact image, not a re-fit, so there is no
        // certificate to re-derive and no fit door to reach
        // (`NurbsSurface::map_points` states what the caller owes).
        Surface::Nurbs(ref n) => Surface::Nurbs(Arc::new(n.map_points(|p| map.transform_point(p)))),
        Surface::Approx(ref a) => Surface::Approx(Arc::new(map_approx(map, hand, a))),
    })
}

/// **The mapped approximating surface**: the offset description's base
/// net and the fit net by the full affine map (weights and knots are
/// invariants of it), `d` negated under a reflection, the window and
/// the certificate carried.
///
/// The image is a pair by the composition law: an isometry carries unit
/// normals to unit normals, so `M(S + d·n) = M(S) + d·M·n`, and `M·n` is
/// the mapped net's chart normal under a rigid map and its negation
/// under a reflection. The certificate bounds the distance between the
/// fit and the description, which the isometry keeps, so it is the
/// image's. Re-deriving it would read the same bound back to within the
/// image's rounding (`geom-brep`'s
/// `a_rigid_map_of_an_offset_is_the_offset_of_the_rigid_map`), which is
/// all a face minted at its target could lose (`[simple.conditioned]`).
fn map_approx<T: Real>(
    map: &Affine3<T>,
    hand: Hand,
    a: &geom::ApproxSurface<T>,
) -> geom::ApproxSurface<T> {
    let old = a.spec();
    let geom::SurfaceDescription::Offset { ref base, d } = old.description;
    let base = Arc::new(base.map_points(|p| map.transform_point(p)));
    let d = match hand {
        Hand::Proper => d,
        Hand::Improper => -d,
    };
    let spec = geom::SurfaceSpec {
        description: geom::SurfaceDescription::Offset { base, d },
        fit: old.fit.map_points(|p| map.transform_point(p)),
        window: old.window,
    };
    let certificate = *a.certificate();
    match geom::ApproxSurface::certify(spec, |_, _, _| {
        Ok::<_, core::convert::Infallible>(certificate)
    }) {
        Ok(image) => image,
        Err(never) => match never {},
    }
}

/// The carrier `map ∘ c`, under either hand, its stretch dropped
/// ([`map_carrier_stretched`]): the rows that read a mapped carrier's
/// kind and form, not a span on it.
#[cfg(test)]
pub(crate) fn map_carrier<T: Real>(
    iso: &Isometry<T>,
    c: &Curve3<T>,
) -> Result<Curve3<T>, TransformError> {
    map_carrier_stretched(iso, c).map(|(c, _)| c)
}

/// The carrier `map ∘ c` under either hand, and the factor its parameter
/// is the old one's times. A carrier has no outward side, so the map
/// keeps its parameter, every edge's range and its forward half-edge, but
/// for one rescale rounding can force: a line whose image direction a
/// chain of maps carried past its rounding, which the door squares
/// ([`geom_core::UnitVec3::through_isometry`]), so arc length along the
/// squared direction is the old parameter times the length divided by.
/// The factor is `one` everywhere else.
pub(crate) fn map_carrier_stretched<T: Real>(
    iso: &Isometry<T>,
    c: &Curve3<T>,
) -> Result<(Curve3<T>, T), TransformError> {
    let (map, hand) = (&iso.to_affine(), iso.hand());
    // A conic's frame: the frame's own image under a motion, and under a
    // reflection the axial axis, so the image keeps its parameter.
    let motion = match iso {
        Isometry::Proper(motion) => Some(motion),
        Isometry::Improper(_) => None,
    };
    let frame = |f: &geom::Frame<T>| map_frame(map, hand, motion, f, true);
    Ok((
        match *c {
            Curve3::Line(geom::Line { origin, dir }) => {
                // An isometry's image of a unit direction: measured, and
                // squared past its rounding with the parameter stretched
                // to match.
                let (dir, stretch) = dir.through_isometry(map.linear);
                return Ok((Curve3::line(map.transform_point(origin), dir), stretch));
            }
            // A rigid map preserves the sizes and the frame's orthonormality —
            // the ellipse's axis ordering transfers unchanged, so no
            // re-decision is needed (the constructor decided at mint; this is
            // evaluation-lane data motion).
            Curve3::Circle(c) => Curve3::Circle(c.placed(frame)),
            Curve3::Ellipse(e) => Curve3::Ellipse(e.mapped(frame)),
            // The same argument: a rigid map preserves both radii, the
            // stand-off and the frame's orthonormality, so the two-oval
            // regime and the carried side transfer unchanged — no
            // re-decision (the mint decided; this is data motion).
            Curve3::Spiric {
                frame: carrier_frame,
                major_radius,
                minor_radius,
                offset,
            } => {
                // `(axis, u_ref, offset)` and `(−axis, −u_ref, −offset)` name
                // one oval; under a reflection the image keeps its `v` with
                // `axis` mapped and the plane's side negated.
                // Under a motion the frame is moved, as a conic's is.
                let (frame, offset) = match iso {
                    Isometry::Proper(motion) => (carrier_frame.moved(motion), offset),
                    // `(M·a, −M·u)`, its `v` then `M·v`: the frame
                    // with `u` reversed.
                    Isometry::Improper(_) => {
                        (carrier_frame.reflected(map, geom_core::Negated::U), -offset)
                    }
                };
                Curve3::Spiric {
                    frame,
                    major_radius,
                    minor_radius,
                    offset,
                }
            }
            // A net maps by its control points, weights and knots verbatim,
            // as a periodic cubic's cyclic net does.
            Curve3::Nurbs(ref n) => {
                Curve3::Nurbs(Arc::new(n.map_points(|p| map.transform_point(p))))
            }
            Curve3::Periodic(ref n) => {
                Curve3::Periodic(Arc::new(n.map_points(|p| map.transform_point(p))))
            }
        },
        T::one(),
    ))
}

/// Applies the rigid motion to every geometric datum of `body`, returning
/// a NEW body with identical topology and identical arena keys (the
/// three geometry arenas are rewritten in place of the clone; nothing
/// is re-minted, so downstream key handles remain meaningful). The
/// operand's contact records therefore carry as
/// [`crate::ContactRecords::carried_from`]`(0)`.
///
/// It is the construction-state map: every certificate is carried as
/// data and nothing is decided, so the body is what the finish that
/// follows it decides. A finished body moves through its own door,
/// [`crate::FinishedBody::moved`], which carries the verdict too.
///
/// # Errors
///
/// [`TransformError`] — closed and typed.
pub fn transform_rigid<T: Decide>(
    body: &Body<T>,
    motion: &RigidMotion<T>,
) -> Result<Body<T>, TransformError> {
    map_isometry(body, &Isometry::Proper(*motion))
}

/// **One surface read at a rigid motion**: `motion ∘ s`, the datum a
/// body moved by `motion` ([`transform_rigid`]) holds in its place, to
/// the bit. A reader that relates two bodies through their relative
/// motion maps the carriers it reads and leaves the body as it is
/// (`[placement.frame]`).
///
/// # Errors
///
/// [`TransformError`], as [`transform_rigid`] refuses the surface.
pub fn surface_at<T: Real>(
    motion: &RigidMotion<T>,
    s: &Surface<T>,
) -> Result<Surface<T>, TransformError> {
    map_surface(&motion.to_affine(), Hand::Proper, Some(motion), s)
}

/// **One curve carrier read at a rigid motion**: `motion ∘ c`, as
/// [`surface_at`] reads a surface, with the factor its parameter is
/// `c`'s times, `k` ([`map_carrier_stretched`]): `one`, but for a line
/// whose image direction the door squared. The point `c` reads at `t`
/// moves to the point the image reads at `k·t`, so a span `(t0, t1)` on
/// `c` is the span `(k·t0, k·t1)` on the image, as [`transform_rigid`]
/// scales an edge's.
///
/// # Errors
///
/// [`TransformError`], as [`transform_rigid`] refuses the carrier.
pub fn carrier_at<T: Real>(
    motion: &RigidMotion<T>,
    c: &Curve3<T>,
) -> Result<(Curve3<T>, T), TransformError> {
    map_carrier_stretched(&Isometry::Proper(*motion), c)
}

/// The chart image `image` on `surface` of `body` under a map of `hand`:
/// itself under a proper map, whose mapped surface carries the whole map,
/// and reflected with its chart under an improper one.
fn mapped_image<T: Real>(
    body: &Body<T>,
    surface: geom_brep::SurfaceKey,
    image: &geom_brep::Pcurve<T>,
    hand: Hand,
) -> Result<geom_brep::Pcurve<T>, TransformError> {
    match hand {
        Hand::Proper => Ok(image.clone()),
        Hand::Improper => {
            let class = match body.get_surface(surface) {
                Ok(s) => OrientationClass::of(s),
                Err(crate::SurfaceMiss::Stale) => {
                    return Err(TransformError::Corrupt {
                        found: "a chart image lies on a missing surface",
                    });
                }
                Err(crate::SurfaceMiss::Pending) => unreachable!(
                    "{surface:?} is pending under a chart image: certification refuses a \
                     pending key"
                ),
            };
            reflected_image(image, class)
        }
    }
}
/// **An edge's chart image under a reflection, in the inside-out
/// image's own terms** ([`map_isometry`] at [`Hand::Improper`]).
///
/// A spiric image is restated for its mapped carrier, whose stand-off
/// is `−d` ([`map_carrier_stretched`]): `f(t) = √((R + r·cos t)² − d²)` is even in
/// `d`, so a plane cap keeps its coefficients (its chart is unmoved),
/// and a torus wall, whose chart reverses its azimuth, reads
/// `−u0 − σμ·atan2(f, d) = u0′ + σμ·atan2(f, −d)` with
/// `u0′ = −u0 − σμπ`, since `atan2(f, −d) = π − atan2(f, d)` for the
/// `f > 0` of the two-oval regime; its minor angle, its sense and its
/// torus are unmoved. Every other image on an analytic chart has its
/// azimuth negated, and one on a plane or a net is unmoved.
///
/// # Errors
///
/// [`TransformError::Corrupt`] for an image whose winding count has no
/// negation in `i32`, which no body a door built carries.
fn reflected_image<T: Real>(
    p: &geom_brep::Pcurve<T>,
    class: OrientationClass,
) -> Result<geom_brep::Pcurve<T>, TransformError> {
    use geom_brep::{Pcurve, SpiricImage};
    if let Pcurve::Spiric {
        major,
        minor,
        offset,
        image,
    } = p
    {
        let image = match *image {
            SpiricImage::Cap { .. } => *image,
            SpiricImage::Wall {
                u0,
                v0,
                sense,
                torus,
            } => SpiricImage::Wall {
                u0: -u0 - sense * torus.mu::<T>() * T::pi(),
                v0,
                sense,
                torus,
            },
        };
        return Ok(Pcurve::Spiric {
            major: *major,
            minor: *minor,
            offset: -*offset,
            image,
        });
    }
    match class {
        OrientationClass::Analytic => p
            .map_affine(geom_brep::ChartMap {
                u: geom_brep::AxisMap::reflect(),
                v: geom_brep::AxisMap::identity(),
            })
            .map_err(|refusal| TransformError::Corrupt {
                found: match refusal {
                    geom_brep::ChartMapRefusal::WindingUnnegatable => {
                        "a chart image's winding count has no negation"
                    }
                    geom_brep::ChartMapRefusal::WallReflected => {
                        "a spiric wall image reached a reflection that cannot restate it"
                    }
                },
            }),
        OrientationClass::Plane | OrientationClass::Net => Ok(p.clone()),
    }
}

/// **How a reflection maps a surface's chart.** The reflection's own
/// classification: how a face reverses is revert's to say
/// ([`crate::revert`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OrientationClass {
    /// A plane: a reflection writes it so its chart is unmoved
    /// ([`map_surface`]).
    Plane,
    /// A cylinder, cone, sphere, torus or spindle: a reflection maps
    /// the frame forward, reversing the azimuth `u` (MIRROR-DESIGN P1).
    Analytic,
    /// A spline net or a fit: a reflection maps the net by its points,
    /// its chart unmoved.
    Net,
}

impl OrientationClass {
    /// The class of `s`.
    fn of<T: Real>(s: &Surface<T>) -> Self {
        match s {
            Surface::Plane { .. } => Self::Plane,
            Surface::Cylinder { .. }
            | Surface::Cone { .. }
            | Surface::Sphere { .. }
            | Surface::Torus { .. }
            | Surface::Spindle { .. } => Self::Analytic,
            Surface::Nurbs(_) | Surface::Approx(_) => Self::Net,
        }
    }
}

/// The data map behind [`transform_rigid`], [`crate::mirror`] and
/// [`crate::FinishedBody::moved`]: every datum of `body` under `iso`,
/// topology and keys untouched, every certificate carried. Nothing is
/// decided and nothing is certified again: an isometry exact apart from
/// rounding keeps every distance a certificate states, so each
/// certificate of the source is one of the image. A witness is re-minted
/// from the mapped carrier at its pinned mid parameter, as construction
/// mints it.
///
/// Under [`Hand::Improper`] the result is the reflected body INSIDE OUT
/// ([`Hand`]): the analytic faces' outward side is flipped on their
/// `sense`, since their mapped frames carry the outward normal across,
/// and an edge image on an analytic chart has its azimuth negated with
/// the chart. Its pcurve rows and joints are left as found, for the
/// mirror to drop and the image's completion to derive again
/// ([`crate::mirror`]).
pub(crate) fn map_isometry<T: Decide>(
    body: &Body<T>,
    iso: &Isometry<T>,
) -> Result<Body<T>, TransformError> {
    let (map, hand) = (&iso.to_affine(), iso.hand());
    let mut out = body.clone();

    for (_k, p) in &mut out.points {
        *p = map.transform_point(*p);
    }
    let mut mapped_surfaces = Vec::new();
    for (k, slot) in &out.surfaces {
        // A pending slot has no geometry to move.
        if let SurfaceSlot::Described(s) = slot {
            let moved = map_surface(map, hand, iso.motion(), s)?;
            mapped_surfaces.push((k, SurfaceSlot::Described(moved)));
        }
    }
    for (k, s) in mapped_surfaces {
        out.surfaces[k] = s;
    }

    // Curves: walk the EDGES (each edge names its curve; tier 1 keeps
    // the reference well-formed). Track which curve keys were rewritten
    // so an orphaned curve entry, which the walk would leave UNMAPPED,
    // is refused as corruption instead.
    let mut rewritten: SecondaryMap<CurveKey, ()> = SecondaryMap::new();
    let edge_keys: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    for ek in edge_keys {
        let edge = out.edges.get(ek).ok_or(TransformError::Corrupt {
            found: "an edge vanished mid-walk",
        })?;
        let curve_key = edge.curve;
        let old = match body.curves.get(curve_key) {
            Some(CurveGeom::Certified(ec)) => ec,
            Some(CurveGeom::NullScaffold(_)) => {
                return Err(TransformError::NullScaffold { edge: ek });
            }
            None => {
                return Err(TransformError::Corrupt {
                    found: "an edge references a missing curve",
                });
            }
        };
        if rewritten.contains_key(curve_key) {
            continue;
        }
        let (param_start, param_end) = old.params();
        let (carrier, stretch) = map_carrier_stretched(iso, old.stored_carrier())?;
        let (param_start, param_end) = (param_start * stretch, param_end * stretch);
        // A squared line's parameter is stretched: its chart images, the
        // description's and each half-edge's pcurve row, are read on the
        // new one ([`geom_brep::Pcurve::stretched`]). Every image a line
        // carries is affine in its parameter, so one that is not is no
        // image a door builds.
        let stretched = stretch.structure_bracket() != Some((1.0, 1.0));
        const UNSTRETCHED: &str = "a line's chart image is not affine in its parameter";
        if stretched {
            let edge = out.edges.get(ek).ok_or(TransformError::Corrupt {
                found: "an edge vanished mid-walk",
            })?;
            for he in [edge.he_plus, edge.he_minus] {
                if let Some(row) = out.pcurves.get(he) {
                    let row = row
                        .stretched(stretch)
                        .ok_or(TransformError::Corrupt { found: UNSTRETCHED })?;
                    out.pcurves.insert(he, row);
                }
            }
        }
        let description = match old.description() {
            EdgeDescription::Intersection { pair, selector } => EdgeDescription::Intersection {
                pair: *pair,
                selector: match selector {
                    witness @ geom_brep::CurveSelector::Witness(_) => {
                        witness.repinned(&carrier, param_start, param_end)
                    }
                    // Each wall's row maps as a chart image does.
                    geom_brep::CurveSelector::Structural(geom_brep::Structural::SharedRow(row)) => {
                        let [ka, kb] = pair.keys();
                        let mapped = |k| -> Result<_, TransformError> {
                            let image = row.image_on(*pair, k).ok_or(TransformError::Corrupt {
                                found: "a shared row names a surface outside its pair",
                            })?;
                            Ok((k, mapped_image(body, k, image, hand)?))
                        };
                        let row = geom_brep::SharedRow::new(*pair, mapped(ka)?, mapped(kb)?)
                            .ok_or(TransformError::Corrupt {
                                found: "a shared row's mapped image is no row",
                            })?;
                        geom_brep::CurveSelector::Structural(geom_brep::Structural::SharedRow(row))
                    }
                },
            },
            EdgeDescription::TangentIntersection { pair, .. } => {
                EdgeDescription::TangentIntersection {
                    pair: *pair,
                    witness: carrier.mid_point(param_start, param_end),
                }
            }
            EdgeDescription::OnPair { pair } => EdgeDescription::OnPair { pair: *pair },
            // A chart image is a PARAMETER-SPACE fact: under a proper map
            // the mapped surface carries the whole map, and under a
            // reflection the image is restated with its chart.
            EdgeDescription::Chart(c) => EdgeDescription::Chart(geom_brep::ChartCurve {
                surface: c.surface,
                pcurve: {
                    let image = mapped_image(body, c.surface, &c.pcurve, hand)?;
                    if stretched {
                        image
                            .stretched(stretch)
                            .ok_or(TransformError::Corrupt { found: UNSTRETCHED })?
                    } else {
                        image
                    }
                },
                wrap: c.wrap,
            }),
            EdgeDescription::Scaffold(mc) => {
                EdgeDescription::Scaffold(map_mapped_curve(map, hand, mc))
            }
        };
        // A sketch pushforward is stated in 3-space, so the authority
        // record travels with the map.
        let authority = match old.authority() {
            geom_brep::EdgeAuthority::Derived => geom_brep::EdgeAuthority::Derived,
            geom_brep::EdgeAuthority::Declared(mc) => {
                geom_brep::EdgeAuthority::Declared(map_mapped_curve(map, hand, &mc))
            }
        };
        out.curves[curve_key] =
            CurveGeom::Certified(old.carried(carrier, stretch, description, authority));
        rewritten.insert(curve_key, ());
    }
    if out.curves.iter().any(|(k, _)| !rewritten.contains_key(k)) {
        return Err(TransformError::Corrupt {
            found: "a curve entry is referenced by no edge",
        });
    }

    if hand == Hand::Improper {
        let flipped: Vec<crate::entity::FaceKey> = out
            .faces
            .iter()
            .filter(|(_, f)| {
                out.get_surface(f.surface)
                    .is_ok_and(|s| OrientationClass::of(s) == OrientationClass::Analytic)
            })
            .map(|(k, _)| k)
            .collect();
        for k in flipped {
            out.faces[k].sense = !out.faces[k].sense;
        }
    }
    Ok(out)
}

/// The pushforward under `map ∘ place`: the sketch datum stays as
/// authored, so under a reflection `place` is improper and reads it
/// back unchanged through its inverse.
fn map_mapped_curve<T: Real>(map: &Affine3<T>, hand: Hand, mc: &MappedCurve<T>) -> MappedCurve<T> {
    MappedCurve {
        source: match mc.source {
            MappedSource::PlacedSegment { segment, place } => MappedSource::PlacedSegment {
                segment,
                place: *map * place,
            },
            MappedSource::ExtrudedPoint { point, place, vec } => MappedSource::ExtrudedPoint {
                point,
                place: *map * place,
                vec: map_vec(map, vec),
            },
            MappedSource::RevolvedPoint {
                point,
                place,
                axis,
                angle,
            } => MappedSource::RevolvedPoint {
                point,
                place: *map * place,
                axis: geom::Line {
                    origin: map.transform_point(axis.origin),
                    // An isometry's image of the unit axis, its sense
                    // kept under either hand (negated under an improper
                    // map, `(−M·a) × (M·u) = M·(a × u)` when `det M = −1`);
                    // an axis has no span to stretch.
                    dir: match hand {
                        Hand::Proper => axis.dir.through_isometry(map.linear).0,
                        Hand::Improper => -axis.dir.through_isometry(map.linear).0,
                    },
                },
                angle,
            },
        },
        range: mc.range,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! The rigid map's rows: each surface and carrier kind maps to its
    //! exact image.

    use super::*;
    use geom::{NurbsCurve3, NurbsSurface};
    use geom_core::Point3;
    use geom_core::spline::KnotVector;

    /// **A spiric wall image under a reflection is its source's with the
    /// azimuth negated, point for point** ([`reflected_image`]): the
    /// restated `u0′ = −u0 − σμπ` reads `−u(t)` on the mapped carrier,
    /// whose stand-off is `−d`, on both tori and both senses. Without the
    /// `−σμπ` term the image stands half a turn off its locus.
    #[test]
    fn a_reflected_spiric_wall_image_negates_its_azimuth_point_for_point() {
        use core::f64::consts::{PI, TAU};
        use geom_brep::{Pcurve, SpiricImage, WallTorus};
        // `f = √((R + r·cos t)² − d²) > 0` at every `t`: the two-oval
        // regime the restatement is derived for.
        let (major, minor, offset) = (2.0, 1.0, 0.5);
        for torus in [WallTorus::Own, WallTorus::Mirror] {
            for sense in [1.0, -1.0] {
                let source = Pcurve::Spiric {
                    major,
                    minor,
                    offset,
                    image: SpiricImage::Wall {
                        u0: 0.3,
                        v0: 0.0,
                        sense,
                        torus,
                    },
                };
                let image = reflected_image(&source, OrientationClass::Analytic)
                    .expect("a wall image has a reflection");
                for k in 0..16 {
                    let t = TAU * f64::from(k) / 16.0;
                    let (a, b) = (source.eval(t), image.eval(t));
                    let turn = (b.x + a.x + PI).rem_euclid(TAU) - PI;
                    assert!(
                        turn.abs() <= 1e-12,
                        "{torus:?}, σ = {sense}, t = {t}: the image's azimuth {} is not \
                         the negated {}",
                        b.x,
                        a.x
                    );
                    assert!((b.y - a.y).abs() <= 1e-12, "the minor angle is unmoved");
                }
            }
        }
    }

    /// A translation: rigid, with exact entries.
    fn aside() -> Affine3<f64> {
        aside_motion().to_affine()
    }

    /// [`aside`] as the witness the body door takes.
    fn aside_motion() -> RigidMotion<f64> {
        RigidMotion::translation(Vec3::new(3.0, -1.5, 0.25)).expect("a finite translation")
    }

    /// A described bilinear patch on the unit square — four live
    /// corners, unit weights. The minimal counterexample to "a `Nurbs`
    /// payload evaluates to poison".
    fn described_surface() -> Surface<f64> {
        let corners = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.5),
        ];
        Surface::Nurbs(Arc::new(
            NurbsSurface::new(
                KnotVector::unit_segment(core::num::NonZeroUsize::MIN),
                KnotVector::unit_segment(core::num::NonZeroUsize::MIN),
                corners,
                vec![1.0; 4],
            )
            .expect("the bilinear patch validates"),
        ))
    }

    /// A described segment as a degree-1 rational curve.
    fn described_carrier() -> Curve3<f64> {
        Curve3::Nurbs(Arc::new(
            NurbsCurve3::new(
                KnotVector::unit_segment(core::num::NonZeroUsize::MIN),
                vec![Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 1.0, 0.0)],
                vec![1.0, 2.0],
            )
            .expect("the segment validates"),
        ))
    }

    #[test]
    fn a_described_surface_maps_by_its_control_points() {
        let map = aside();
        let before = described_surface();
        let after = map_surface(&map, Hand::Proper, None, &before).expect("a described net maps");
        let (Surface::Nurbs(b), Surface::Nurbs(a)) = (&before, &after) else {
            panic!("the variant changed under the map");
        };
        assert_eq!(a.weights(), b.weights(), "weights are rigid-invariant");
        assert_eq!(
            a.knots_u_f64_interim(),
            b.knots_u_f64_interim(),
            "knots are rigid-invariant"
        );
        assert_eq!(
            a.knots_v_f64_interim(),
            b.knots_v_f64_interim(),
            "knots are rigid-invariant"
        );
        for (p, q) in b.control().iter().zip(a.control()) {
            let want = map.transform_point(*p);
            assert_eq!((q.x, q.y, q.z), (want.x, want.y, want.z));
        }
    }

    #[test]
    fn a_described_carrier_maps_by_its_control_points() {
        let map = aside();
        let before = described_carrier();
        let after =
            map_carrier(&Isometry::Proper(aside_motion()), &before).expect("a described net maps");
        let (Curve3::Nurbs(b), Curve3::Nurbs(a)) = (&before, &after) else {
            panic!("the variant changed under the map");
        };
        assert_eq!(a.weights(), b.weights(), "weights are rigid-invariant");
        assert_eq!(
            a.knots_f64_interim(),
            b.knots_f64_interim(),
            "knots are rigid-invariant"
        );
        for (p, q) in b.control().iter().zip(a.control()) {
            let want = map.transform_point(*p);
            assert_eq!((q.x, q.y, q.z), (want.x, want.y, want.z));
        }
    }

    /// **A carrier net that is not finite is mapped as found**: the map
    /// reads no geometry, and the poison travels to the image, where the
    /// at-rest check refuses it (`PoisonedCurveDatum`). Whether the map
    /// should refuse it up front is SHELL's call.
    #[test]
    fn a_poisoned_carrier_net_is_mapped_as_found() {
        let poisoned: Curve3<f64> = geom::test_support::poisoned_nurbs_segment();
        let mapped = map_carrier(&Isometry::Proper(aside_motion()), &poisoned);
        let Ok(Curve3::Nurbs(n)) = mapped else {
            panic!("expected the mapped poisoned net, got {mapped:?}");
        };
        assert!(
            n.control()
                .iter()
                .all(|p| p.x.is_nan() && p.y.is_nan() && p.z.is_nan()),
            "the poison is carried through the map"
        );
    }

    /// A reflection across a plane tilted off every world axis.
    fn reflection() -> crate::Reflection<f64> {
        let band = geom_core::Band::linear(geom_core::Tol::witness()).expect("the witness band");
        let n = geom_core::UnitVec3::new(Vec3::new(1.0, 2.0, 2.0) / 3.0, "mirror", band)
            .expect("a unit normal");
        crate::mirror::Reflection::across_unit(Point3::new(0.3, -0.2, 0.1), n)
    }

    /// **A reflection keeps each carrier kind's parameter**: the image
    /// carrier at `t` is the reflected point at `t`, by each kind's own
    /// rule (a conic's axial axis; a spiric's seam and stand-off negated).
    #[test]
    fn a_reflection_keeps_a_carriers_parameter_kind_by_kind() {
        let r = reflection();
        let m = r.to_affine();
        let (c, a) = (Point3::new(0.5, -1.0, 2.0), Vec3::new(2.0, 2.0, 1.0) / 3.0);
        let u = Vec3::new(1.0, -2.0, 2.0) / 3.0;
        let frame = geom::Frame::new(c, a, u);
        let kinds = [
            ("circle", Curve3::Circle(geom::Circle::new(frame, 0.7))),
            (
                "ellipse",
                Curve3::Ellipse(geom::Ellipse::new(frame, 0.9, 0.4)),
            ),
            (
                "spiric",
                Curve3::Spiric {
                    frame: geom::Frame::new(c, a, u),
                    major_radius: 2.0,
                    minor_radius: 0.5,
                    offset: 0.2,
                },
            ),
        ];
        for (kind, carrier) in kinds {
            let image = map_carrier(&Isometry::Improper(r), &carrier).expect("an analytic carrier");
            for k in 0..12 {
                let t = f64::from(k) * 0.53 - 2.0;
                let off = (image.eval(t) - m.transform_point(carrier.eval(t))).norm();
                assert!(
                    off < 1e-12,
                    "{kind}: the image at {t} is {off} off the reflected point"
                );
            }
        }
    }

    /// **A frame's image under a reflection is the frame its mapped data
    /// state, hand for hand**: a plane's (its normal axial, `w`
    /// reversed), a chart of revolution's (its axis forward, `v`
    /// reversed) and a spiric's (`u` reversed) each read, axis by axis,
    /// what squaring the mapped origin, axis and seam stated before
    /// (`Frame::new(M·o, ∓M·a, ±M·u)`), to rounding, and each is
    /// right-handed. Red with any other axis reversed.
    #[test]
    fn a_reflected_frame_is_the_frame_its_mapped_data_state() {
        let r = reflection();
        let m = r.to_affine();
        let (o, a) = (Point3::new(0.5, -1.0, 2.0), Vec3::new(2.0, 2.0, 1.0) / 3.0);
        let u_ref = Vec3::new(1.0, -2.0, 2.0) / 3.0;
        let frame = geom::Frame::new(o, a, u_ref);
        let mv = |v: Vec3<f64>| m.linear * v;
        let close = |x: Vec3<f64>, y: Vec3<f64>| (x - y).norm() < 1e-15;
        let plane = map_frame(&m, Hand::Improper, None, &frame, true);
        let chart = map_frame(&m, Hand::Improper, None, &frame, false);
        let spiric = match map_carrier(
            &Isometry::Improper(r),
            &Curve3::Spiric {
                frame,
                major_radius: 2.0,
                minor_radius: 0.5,
                offset: 0.3,
            },
        )
        .expect("an analytic carrier")
        {
            Curve3::Spiric { frame, .. } => frame,
            other => panic!("a spiric maps to a spiric: {other:?}"),
        };
        for (kind, image, axis, seam) in [
            ("plane", plane, -mv(a), mv(u_ref)),
            ("chart", chart, mv(a), mv(u_ref)),
            ("spiric", spiric, mv(a), -mv(u_ref)),
        ] {
            let stated = geom::Frame::new(m.transform_point(o), axis, seam);
            assert!(close(image.axis(), stated.axis()), "{kind}: axis");
            assert!(close(image.u_ref(), stated.u_ref()), "{kind}: u_ref");
            assert!(close(image.v_ref(), stated.v_ref()), "{kind}: v_ref");
            assert!(
                (image.origin() - stated.origin()).norm() < 1e-15,
                "{kind}: origin"
            );
        }
    }

    /// **A reflection's chart images, kind by kind**: a plane's chart
    /// maps forward (its normal axial), every analytic chart of
    /// revolution with its azimuth negated (its frame forward).
    #[test]
    fn a_reflection_maps_each_analytic_chart_kind_by_kind() {
        let r = reflection();
        let m = r.to_affine();
        let (o, a) = (Point3::new(0.5, -1.0, 2.0), Vec3::new(2.0, 2.0, 1.0) / 3.0);
        let u_ref = Vec3::new(1.0, -2.0, 2.0) / 3.0;
        let kinds = [
            (
                "plane",
                Surface::Plane {
                    frame: geom::Frame::new(o, a, u_ref),
                },
                1.0,
            ),
            (
                "cylinder",
                Surface::Cylinder {
                    frame: geom::Frame::new(o, a, u_ref),
                    radius: 0.7,
                },
                -1.0,
            ),
            (
                "cone",
                Surface::Cone {
                    frame: geom::Frame::new(o, a, u_ref),
                    half_angle: 0.4,
                },
                -1.0,
            ),
            (
                "sphere",
                Surface::Sphere {
                    frame: geom::Frame::new(o, a, u_ref),
                    radius: 0.7,
                },
                -1.0,
            ),
            (
                "torus",
                Surface::Torus {
                    frame: geom::Frame::new(o, a, u_ref),
                    major_radius: 2.0,
                    minor_radius: 0.5,
                },
                -1.0,
            ),
        ];
        for (kind, surface, azimuth) in kinds {
            let image =
                map_surface(&m, Hand::Improper, None, &surface).expect("an analytic surface");
            for k in 0..9 {
                let (s, t) = (f64::from(k) * 0.61 - 2.0, f64::from(k) * 0.17 - 0.6);
                let want = m.transform_point(surface.eval(azimuth * s, t));
                let off = (image.eval(s, t) - want).norm();
                assert!(off < 1e-12, "{kind}: the image at ({s}, {t}) is {off} off");
            }
        }
    }

    /// The public door on a body whose only surface slot is pending:
    /// the slot has no geometry to move and stays pending, and the same
    /// body carrying a described patch instead maps.
    #[test]
    fn the_body_door_keeps_a_pending_slot_pending_and_maps_the_description() {
        let mut pending: Body<f64> = Body::new();
        let key = pending.add_pending_surface();
        let moved = transform_rigid(&pending, &aside_motion()).expect("a pending slot maps");
        assert!(
            moved.surface_slot(key).is_some_and(SurfaceSlot::is_pending),
            "the slot is still pending"
        );

        let mut described: Body<f64> = Body::new();
        described.add_surface(described_surface());
        assert!(transform_rigid(&described, &aside_motion()).is_ok());
    }

    /// A curve entry no edge names is refused, and the refusal says the
    /// entry is unreferenced rather than that a reference dangles.
    #[test]
    fn an_orphan_curve_entry_reads_as_unreferenced() {
        let mut orphaned: Body<f64> = Body::new();
        orphaned.add_null_curve(crate::null::NullEdge {
            below_end: crate::VertexKey::default(),
            above_end: crate::VertexKey::default(),
        });
        let err = transform_rigid(&orphaned, &aside_motion())
            .expect_err("an orphan curve entry is corruption");
        let shown = err.render(geom_brep::recourse::Reading::Build);
        assert!(
            shown.starts_with("the body is corrupt: a curve entry is referenced by no edge."),
            "{shown}"
        );
        assert!(!shown.contains("missing"), "{shown}");
    }
}

/// The offset face under a map: its certificate carried.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod approx_rows {
    use geom_core::{Affine3, Point3, Tol, Vec3};

    use super::map_approx;

    /// A rotation about `z`, which re-splits an ambient-frame hull bound
    /// across the axes.
    fn turned() -> Affine3<f64> {
        Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), 0.7)
    }

    /// **A mapped offset face is the image of its operand, its
    /// certificate carried**: the description's base net and the fit net
    /// by the map point for point, the certificate the operand's limb for
    /// limb, and the mapped pair re-derives within the run's ε, so the
    /// carried certificate's claim holds of the image.
    #[test]
    fn a_mapped_offset_face_carries_its_certificate() {
        let tol = Tol::witness();
        let map = turned();
        let approx = crate::fixtures::bowed_offset_approx::<f64>();
        let mapped = map_approx(&map, super::Hand::Proper, &approx);
        crate::fixtures::assert_certificates_agree(
            "the carried certificate",
            mapped.certificate(),
            approx.certificate(),
        );
        for (p, q) in approx.fit().control().iter().zip(mapped.fit().control()) {
            let want = map.transform_point(*p);
            assert_eq!(
                (q.x, q.y, q.z),
                (want.x, want.y, want.z),
                "the fit net maps by its points"
            );
        }
        let spec = mapped.spec();
        geom_brep::certify_offset_over(&spec.description, &spec.fit, spec.window, tol)
            .expect("the mapped pair certifies at the run's ε");
    }
}

/// A body under a chain of maps: its lines stay unit and on their
/// vertices.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod chain_rows {
    use geom_core::{Tol, Vec3};

    /// **A brick moved forty times keeps its lines unit and on their
    /// vertices** (`[topology.tiers]`): sixty poses, each a chain of forty
    /// rigid motions about skew axes, each with a short translation,
    /// applied to a long brick about the origin in turn, so it stays in
    /// the session box. After every chain each line carrier's direction
    /// is unit to its rounding ([`UnitVec3::stated`]), each line edge's
    /// ends lie on its vertices, and the body validates, structurally and
    /// at tier 3, at the run's ε. Some chain carries a direction past its
    /// rounding, so the door squares it and the edge's span is stretched.
    /// Red under the door keeping every image's bits (the lines drift off
    /// unit), and under a squared line keeping its span, which no chain
    /// then stretches. The stretch itself is the span times a squaring's
    /// `≤ 8ε`, the size of the points' own drift over the chain (both
    /// read under `8e-13 m` here), so the ends' bound holds with it or
    /// without; it is kept because it is the map's exact reading.
    #[test]
    fn a_brick_moved_forty_times_keeps_its_lines_unit_and_on_their_vertices() {
        use super::{Body, Curve3, CurveGeom, RigidMotion, transform_rigid};
        use geom_core::UnitVec3;
        let tol = Tol::witness();
        let mut state = 0x9e37_79b9_7f4a_7c15_u64;
        let mut next = || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
        };
        let start = crate::test_support_fixtures::brick::<f64>(
            (-300.0, 300.0),
            (-1.0, 2.0),
            (-1.0, 1.0),
            tol,
        );
        let spans = |body: &Body<f64>| -> Vec<(u64, u64)> {
            body.edges()
                .filter_map(|(_, e)| match body.curves.get(e.curve) {
                    Some(CurveGeom::Certified(c)) => {
                        let (a, b) = c.params();
                        Some((a.to_bits(), b.to_bits()))
                    }
                    _ => None,
                })
                .collect()
        };
        let mut stretched = 0;
        for pose in 0..60 {
            let mut body = start.clone();
            for _ in 0..40 {
                let axis = UnitVec3::squared(Vec3::new(next(), next(), next()));
                let motion = RigidMotion::rotation_then_translation(
                    axis,
                    3.0 * next(),
                    Vec3::new(next(), next(), next()) * 5.0,
                )
                .expect("a rotation about a unit axis");
                body = transform_rigid(&body, &motion).expect("a brick moves");
            }
            for (ek, edge) in body.edges() {
                let Some(CurveGeom::Certified(c)) = body.curves.get(edge.curve) else {
                    panic!("pose {pose}: a certified edge");
                };
                let Curve3::Line(line) = c.carrier() else {
                    continue;
                };
                assert!(
                    UnitVec3::stated(line.dir.get()).is_some(),
                    "pose {pose}: {line:?} is unit to its rounding"
                );
                let (v0, v1) = body.edge_vertices(ek).expect("an edge has two ends");
                let at = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                let (t0, t1) = c.params();
                for (t, v) in [(t0, v0), (t1, v1)] {
                    let off = (line.at(t) - at(v)).norm();
                    assert!(off <= 1e-12, "pose {pose}: an end {off:e} m off its vertex");
                }
            }
            crate::validate(&body).unwrap_or_else(|e| panic!("pose {pose}: {e:?}"));
            crate::validate_geometric(&body, tol).unwrap_or_else(|e| panic!("pose {pose}: {e:?}"));
            stretched += usize::from(spans(&body) != spans(&start));
        }
        assert!(
            stretched > 0,
            "some chain squares a line and stretches its span"
        );
    }

    /// **A brick moved a thousand times by a motion a little off rigid
    /// keeps its frames and lines rigid and its ends on its vertices**:
    /// the motion is a rotation whose columns are scaled by `1 + 4ε`,
    /// which [`RigidMotion::check_rigid`] admits (each column `8ε` off
    /// unit, within its `16ε`), applied a thousand times to a `2 m`
    /// brick. Unmeasured, the frames' axes drift by about `8ε` a move
    /// (`809ε` at a hundred) and fail the door; measured, each surface's
    /// frame stays within the door's rounding, each line unit to its
    /// rounding, each line edge's ends within `2e-13 m` of its vertices,
    /// and the body valid at tier 3. Red under the frame door keeping
    /// what it should square (the frames fail the door), under the line
    /// door the same, and under a squared line keeping its span: a line
    /// is squared every second move or so, and its ends then stand about
    /// `2e-12 m` off their vertices.
    #[test]
    fn a_brick_moved_by_a_motion_off_rigid_keeps_its_frames_and_its_ends() {
        // Each move translates the brick; a small step keeps a thousand
        // of them inside the session box.
        use super::{Curve3, CurveGeom, RigidMotion, transform_rigid};
        use geom_core::{Affine3, Point3, UnitVec3};
        let tol = Tol::witness();
        let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.3, -0.7, 0.64), 0.83);
        let off = 1.0 + 4.0 * f64::EPSILON;
        let scaled =
            Affine3::from_parts(turn.linear.map(|x| x * off), Vec3::new(0.01, -0.02, 0.03));
        let motion =
            RigidMotion::check_rigid(&scaled).expect("the door admits a column 8ε off unit");
        let mut body =
            crate::test_support_fixtures::brick::<f64>((-1.0, 1.0), (-0.5, 0.5), (0.0, 0.5), tol);
        for _ in 0..1000 {
            body = transform_rigid(&body, &motion).expect("a brick moves");
        }
        for (_, s) in body.described_surfaces() {
            if let Some(frame) = s.frame() {
                assert!(
                    RigidMotion::check_rigid(&frame.motion().to_affine()).is_ok(),
                    "a surface's frame stays within the door's rounding: {frame:?}"
                );
            }
        }
        let mut worst = 0.0_f64;
        for (ek, edge) in body.edges() {
            let Some(CurveGeom::Certified(c)) = body.curves.get(edge.curve) else {
                panic!("a certified edge");
            };
            let Curve3::Line(line) = c.carrier() else {
                continue;
            };
            assert!(
                UnitVec3::stated(line.dir.get()).is_some(),
                "{line:?} is unit"
            );
            let (v0, v1) = body.edge_vertices(ek).expect("an edge has two ends");
            let at = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let (t0, t1) = c.params();
            for (t, v) in [(t0, v0), (t1, v1)] {
                worst = worst.max((line.at(t) - at(v)).norm());
            }
        }
        assert!(worst <= 2e-13, "an end stands {worst:e} m off its vertex");
        crate::validate(&body).unwrap_or_else(|e| panic!("{e:?}"));
        crate::validate_geometric(&body, tol).unwrap_or_else(|e| panic!("{e:?}"));
    }
}
