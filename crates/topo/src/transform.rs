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
//! parameter, and an isometry keeps every distance and every
//! parameter, so each certificate of the source is one of the image.
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
//! - a DESCRIBED `Nurbs` surface or carrier maps by its CONTROL
//!   POINTS, with knots and weights carried over verbatim: a rigid map
//!   is affine, and the Euclidean-storage section of `geom`'s
//!   `curves::nurbs` data model is where the commutation that makes
//!   the mapped net the exact IMAGE — never a re-fit — is argued;
//! - the `Nurbs` PLACEHOLDER — and only it — is refused typed (its
//!   evaluation is all-poison; transforming one would launder poison
//!   as geometry);
//! - approximating surfaces: the offset description's base net and the
//!   fit net by the full affine map, `d` negated under a reflection,
//!   the window and the certificate carried (`map_approx`).

use std::sync::Arc;

use geom::Curve3;
use geom::Surface;
use geom_brep::{EdgeDescription, MappedCurve, MappedSource};
use geom_core::Tol;
use geom_core::{Affine3, Decide, Real, RigidMotion, Vec3};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::EdgeKey;
use crate::geometry::CurveKey;
use crate::null::CurveGeom;

/// Typed failure of [`transform_rigid`] (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum TransformError {
    /// The mirror's re-mint of the image's pcurve rows refused
    /// ([`crate::mirror`]): a row on an analytic chart, restated with its
    /// azimuth, lands on a branch other than the one a fresh mint takes,
    /// so the mirror derives its rows afresh.
    Pcurve {
        /// The typed pcurve-pass refusal, nested whole.
        source: crate::pcurves::PcurveMintError,
    },
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
    /// A `Nurbs` placeholder surface or carrier — the "no description
    /// yet" net, whose control points are all-poison, so transforming
    /// it would launder poison as geometry. A DESCRIBED net is not
    /// this arm: it maps, by its control points.
    NurbsPlaceholder,
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
            Self::Pcurve { source } => write!(f, "{}", geom_brep::recourse::read_at(source, at)),
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
            Self::NurbsPlaceholder => write!(
                f,
                "a spline (NURBS) surface or carrier cannot be transformed yet. {}",
                geom_core::NOT_YET_ENDING
            ),
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
/// Both arms are exact by construction or re-minted at their door
/// ([`RigidMotion::check_rigid`], [`crate::Reflection::across`]).
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

    /// Whether the map keeps orientation.
    fn hand(&self) -> Hand {
        match self {
            Self::Proper(_) => Hand::Proper,
            Self::Improper(_) => Hand::Improper,
        }
    }
}

impl Hand {
    /// `v` under `map`'s linear part, negated under an improper map:
    /// the axis that keeps a right-handed frame's sense of turning
    /// (`(−M·a) × (M·u) = M·(a × u)` when `det M = −1`).
    fn axial<T: Real>(self, map: &Affine3<T>, v: Vec3<T>) -> Vec3<T> {
        match self {
            Hand::Proper => map_vec(map, v),
            Hand::Improper => -map_vec(map, v),
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
fn map_surface<T: Real>(
    map: &Affine3<T>,
    hand: Hand,
    s: &Surface<T>,
) -> Result<Surface<T>, TransformError> {
    Ok(match *s {
        // The normal is the face's outward side, so the inside-out
        // image's is the axial one; the chart is then `M∘S` exactly.
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => Surface::Plane {
            origin: map.transform_point(origin),
            normal: hand.axial(map, normal),
            u_ref: map_vec(map, u_ref),
        },
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => Surface::Cylinder {
            origin: map.transform_point(origin),
            axis: map_vec(map, axis),
            radius,
            u_ref: map_vec(map, u_ref),
        },
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => Surface::Cone {
            apex: map.transform_point(apex),
            axis: map_vec(map, axis),
            half_angle,
            u_ref: map_vec(map, u_ref),
        },
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => Surface::Sphere {
            center: map.transform_point(center),
            radius,
            axis: map_vec(map, axis),
            u_ref: map_vec(map, u_ref),
        },
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => Surface::Torus {
            center: map.transform_point(center),
            axis: map_vec(map, axis),
            major_radius,
            minor_radius,
            u_ref: map_vec(map, u_ref),
        },
        Surface::Spindle {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => Surface::Spindle {
            center: map.transform_point(center),
            axis: map_vec(map, axis),
            major_radius,
            minor_radius,
            u_ref: map_vec(map, u_ref),
        },
        // A DESCRIBED net maps by its control points, weights and
        // knots untouched — the exact image, not a re-fit, so there is
        // no certificate to re-derive and no fit door to reach
        // (`NurbsSurface::map_points` states what the caller owes).
        // The PLACEHOLDER is the state this refusal's text describes
        // and the only state it refuses: its net is all-poison, so
        // mapping it would launder poison as geometry.
        Surface::Nurbs(ref n) => {
            if n.is_placeholder() {
                return Err(TransformError::NurbsPlaceholder);
            }
            Surface::Nurbs(Arc::new(n.map_points(|p| map.transform_point(p))))
        }
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
/// image's. Re-deriving it would read hull bounds assembled in the
/// ambient frame, which move under a rotation, and could refuse a sound
/// face's image.
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

/// The carrier `map ∘ c` on `c`'s own parameter, under either hand: a
/// carrier has no outward side, and keeping its parameter keeps every
/// edge's range and its forward half-edge.
fn map_carrier<T: Real>(iso: &Isometry<T>, c: &Curve3<T>) -> Result<Curve3<T>, TransformError> {
    let (map, hand) = (&iso.to_affine(), iso.hand());
    // A conic's frame: the frame's own image under a motion, and under a
    // reflection the axial axis, so the image keeps its parameter.
    let frame = |f: &geom::Frame<T>| match iso {
        Isometry::Proper(motion) => f.moved(motion),
        Isometry::Improper(_) => geom::Frame::new(
            map.transform_point(f.origin),
            hand.axial(map, f.axis),
            map_vec(map, f.u_ref),
        ),
    };
    Ok(match *c {
        Curve3::Line(geom::Line { origin, dir }) => {
            Curve3::line(map.transform_point(origin), map_vec(map, dir))
        }
        // A rigid map preserves the sizes and the frame's orthonormality —
        // the ellipse's axis ordering transfers unchanged, so no
        // re-decision is needed (the constructor decided at mint; this is
        // evaluation-lane data motion).
        Curve3::Circle(c) => Curve3::Circle(geom::Circle {
            frame: frame(&c.frame),
            ..c
        }),
        Curve3::Ellipse(e) => Curve3::Ellipse(geom::Ellipse {
            frame: frame(&e.frame),
            ..e
        }),
        // The same argument: a rigid map preserves both radii, the
        // stand-off and the frame's orthonormality, so the two-oval
        // regime and the carried side transfer unchanged — no
        // re-decision (the mint decided; this is data motion).
        Curve3::Spiric {
            center,
            axis,
            u_ref,
            major_radius,
            minor_radius,
            offset,
        } => {
            // `(axis, u_ref, offset)` and `(−axis, −u_ref, −offset)` name
            // one oval; under a reflection the image keeps its `v` with
            // `axis` mapped and the plane's side negated.
            let (u_ref, offset) = match hand {
                Hand::Proper => (map_vec(map, u_ref), offset),
                Hand::Improper => (-map_vec(map, u_ref), -offset),
            };
            Curve3::Spiric {
                center: map.transform_point(center),
                axis: map_vec(map, axis),
                u_ref,
                major_radius,
                minor_radius,
                offset,
            }
        }
        // The surface arm's argument, one dimension down: a described
        // net maps by its control points (weights and knots verbatim)
        // and the placeholder alone is refused.
        Curve3::Nurbs(ref n) => {
            if n.is_placeholder() {
                return Err(TransformError::NurbsPlaceholder);
            }
            Curve3::Nurbs(Arc::new(n.map_points(|p| map.transform_point(p))))
        }
    })
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
    _tol: Tol,
) -> Result<Body<T>, TransformError> {
    map_isometry(body, &Isometry::Proper(*motion))
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
            let class = body.surfaces.get(surface).map(OrientationClass::of).ok_or(
                TransformError::Corrupt {
                    found: "a chart image lies on a missing surface",
                },
            )?;
            reflected_image(image, class)
        }
    }
}
/// **An edge's chart image under a reflection, in the inside-out
/// image's own terms** ([`map_isometry`] at [`Hand::Improper`]).
///
/// A spiric image is restated for its mapped carrier, whose stand-off
/// is `−d` ([`map_carrier`]): `f(t) = √((R + r·cos t)² − d²)` is even in
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

/// **How a surface's face carries its orientation**, the one
/// classification [`crate::Body::revert`] and the reflection read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OrientationClass {
    /// A plane: its stored normal is the outward side, so it carries
    /// its own reversal, and a reflection writes it so its chart is
    /// unmoved ([`map_surface`]).
    Plane,
    /// A cylinder, cone, sphere, torus or spindle: the outward side rides the
    /// face's `sense`, and a reflection maps the frame forward,
    /// reversing the azimuth `u` (MIRROR-DESIGN P1).
    Analytic,
    /// A spline net or a fit: the outward side rides `sense`, and a
    /// reflection maps the net by its points, its chart unmoved.
    Net,
}

impl OrientationClass {
    /// The class of `s`.
    pub(crate) fn of<T: Real>(s: &Surface<T>) -> Self {
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
/// mirror to re-mint ([`crate::mirror`]).
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
    for (k, s) in &out.surfaces {
        mapped_surfaces.push((k, map_surface(map, hand, s)?));
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
        let carrier = map_carrier(iso, old.carrier())?;
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
                pcurve: mapped_image(body, c.surface, &c.pcurve, hand)?,
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
        out.curves[curve_key] = CurveGeom::Certified(old.carried(carrier, description, authority));
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
                out.surfaces
                    .get(f.surface)
                    .is_some_and(|s| OrientationClass::of(s) == OrientationClass::Analytic)
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
                    dir: hand.axial(map, axis.dir),
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
    //! The NURBS gate: the discriminator is the placeholder STATE, not
    //! the `Nurbs` variant. These rows pin both directions, so the gate
    //! cannot silently invert — a described net refusing and a
    //! placeholder mapping are each one edit away from each other, and
    //! only one of them is loud on its own.

    use super::*;
    use geom::{NurbsCurve3, NurbsSurface};
    use geom_core::Point3;
    use geom_core::spline::KnotVector;

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
    fn the_surface_placeholder_is_what_refuses() {
        assert!(matches!(
            map_surface(&aside(), Hand::Proper, &Surface::nurbs_placeholder()),
            Err(TransformError::NurbsPlaceholder)
        ));
    }

    #[test]
    fn the_carrier_placeholder_is_what_refuses() {
        assert!(matches!(
            map_carrier(
                &Isometry::Proper(aside_motion()),
                &Curve3::nurbs_placeholder()
            ),
            Err(TransformError::NurbsPlaceholder)
        ));
    }

    #[test]
    fn a_described_surface_maps_by_its_control_points() {
        let map = aside();
        let before = described_surface();
        let after = map_surface(&map, Hand::Proper, &before).expect("a described net maps");
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
    /// A reflection across a plane tilted off every world axis.
    fn reflection() -> crate::Reflection<f64> {
        let band = geom_core::Band::linear(Tol::witness()).expect("the witness band");
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
                    center: c,
                    axis: a,
                    u_ref: u,
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
                    origin: o,
                    normal: a,
                    u_ref,
                },
                1.0,
            ),
            (
                "cylinder",
                Surface::Cylinder {
                    origin: o,
                    axis: a,
                    radius: 0.7,
                    u_ref,
                },
                -1.0,
            ),
            (
                "cone",
                Surface::Cone {
                    apex: o,
                    axis: a,
                    half_angle: 0.4,
                    u_ref,
                },
                -1.0,
            ),
            (
                "sphere",
                Surface::Sphere {
                    center: o,
                    radius: 0.7,
                    axis: a,
                    u_ref,
                },
                -1.0,
            ),
            (
                "torus",
                Surface::Torus {
                    center: o,
                    axis: a,
                    major_radius: 2.0,
                    minor_radius: 0.5,
                    u_ref,
                },
                -1.0,
            ),
        ];
        for (kind, surface, azimuth) in kinds {
            let image = map_surface(&m, Hand::Improper, &surface).expect("an analytic surface");
            for k in 0..9 {
                let (s, t) = (f64::from(k) * 0.61 - 2.0, f64::from(k) * 0.17 - 0.6);
                let want = m.transform_point(surface.eval(azimuth * s, t));
                let off = (image.eval(s, t) - want).norm();
                assert!(off < 1e-12, "{kind}: the image at ({s}, {t}) is {off} off");
            }
        }
    }

    /// The gate as the public door reports it: a body whose only
    /// geometry is a placeholder surface refuses, and the same body
    /// carrying a described patch instead maps.
    #[test]
    fn the_body_door_refuses_the_placeholder_and_admits_the_description() {
        let mut placeheld: Body<f64> = Body::new();
        placeheld.add_surface(Surface::nurbs_placeholder());
        assert!(matches!(
            transform_rigid(&placeheld, &aside_motion(), Tol::witness()),
            Err(TransformError::NurbsPlaceholder)
        ));

        let mut described: Body<f64> = Body::new();
        described.add_surface(described_surface());
        assert!(transform_rigid(&described, &aside_motion(), Tol::witness()).is_ok());
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
        let err = transform_rigid(&orphaned, &aside_motion(), Tol::witness())
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
