//! **Read-back doors: what did the model choose?**
//!
//! A construction answers questions its author asked implicitly —
//! where a cap plane landed, what frame a wall's carrier sits in,
//! where a corner vertex ended up. Without these doors the only way to
//! ask is to hand-scan the body's arenas (or to transcribe the answer
//! as a literal and hope it stayed true). These doors ask the model
//! instead.
//!
//! # The three rules these doors keep
//!
//! 1. **Values, never verdicts.** A door answers "this face's carrier
//!    frame is (o, n, u)". No door answers "is this face planar", "is
//!    this edge convex", "is this at z ≈ 1" — geometric predicates are
//!    decided-predicate sites under the margins discipline and stay
//!    deferred. Nothing here decides anything: every answer is stored
//!    data, copied out.
//! 2. **Definitional re-read carries no pad.** The produced surface IS
//!    the definition (DESIGN Q8) — reading a plane's stored origin and
//!    normal back is a re-read of authored data, not a measurement, so
//!    there is no residual to certify. The rule's other half binds
//!    too: any answer sourced from an APPROXIMATING or quadrature
//!    construction must carry its certified residual, exactly as
//!    [`crate::MassProperties`] carries `volume_pad`. No door here
//!    reads such a source; when one does, it grows a pad field.
//! 3. **No invented conventions.** Where the stored geometry fixes no
//!    frame — a NURBS patch has no canonical origin or axis, a line
//!    has no distinguished perpendicular — the door says so
//!    ([`ReadbackError::NoCanonicalFrame`], [`Pose::u_ref`]'s `None`)
//!    rather than fabricating one that would then be quoted back as
//!    though the model had chosen it.
//!
//! # Layering
//!
//! These are KERNEL doors, and their home is what they read: a
//! [`Body`], its geometry arenas, and nothing else. No operation crate
//! is involved on either side — reading a face's plane costs a caller
//! a dependency on the topology crate, not on whichever op happened to
//! build the face. The document-layer twins that take a `StableName`
//! instead of an arena key live in `editor-core` and delegate here, so
//! there is one reading of any given piece of geometry, not two.
//!
//! Op-specific doors — "where did THIS extrusion's caps land" — belong
//! with their op, phrased in its own result vocabulary, and delegate
//! to [`face_pose`] for the read itself.

use geom::Curve3;
use geom::Surface;
use geom_core::{Point3, Real, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, GeomRef, VertexKey};

/// **A frame read off stored geometry**: an origin plus the carrier's
/// own reference directions, verbatim.
///
/// The triad is right-handed where it is complete: `u_ref`,
/// `v_ref = axis × u_ref` ([`Pose::v_ref`]), `axis`.
///
/// - `origin` is the carrier's own distinguished point — a plane's
///   `origin`, a cylinder's `v = 0` axis point, a cone's apex, a
///   sphere's or torus's centre, a circle's centre, a line's `t = 0`
///   point. It is the CARRIER's, not the trimmed face's or edge's:
///   a plane face's origin need not lie inside the face (the plane it
///   names is the same plane either way).
/// - `axis` is the carrier's principal direction: a plane's normal,
///   every other analytic surface's axis, a circle's or ellipse's
///   plane normal, a line's direction. It is the CHART's direction,
///   NOT corrected by a face's orientation sense — the sense is a
///   separate fact about the face, and folding it in silently would
///   make two different questions share one answer.
/// - `u_ref` is the in-frame reference direction where the carrier's
///   convention fixes one (the seam of every closed chart, θ = 0 of a
///   circle, an ellipse's semi-major direction). It is `None` where
///   the convention fixes none: a line has a direction and no
///   distinguished perpendicular, and inventing one would be a
///   fabricated convention (rule 3).
#[derive(Clone, Copy, Debug)]
pub struct Pose<T: Real> {
    /// The carrier's distinguished point (see the type docs).
    pub origin: Point3<T>,
    /// The carrier's principal direction, chart sense (see the type
    /// docs).
    pub axis: Vec3<T>,
    /// The in-frame reference direction, where the carrier fixes one.
    pub u_ref: Option<Vec3<T>>,
}

impl<T: Real> Pose<T> {
    /// The third leg of the right-handed triad, `axis × u_ref` —
    /// computed, never stored, exactly as the carriers do it. `None`
    /// when [`Pose::u_ref`] is.
    #[must_use]
    pub fn v_ref(&self) -> Option<Vec3<T>> {
        self.u_ref.map(|u| self.axis.cross(u))
    }
}

/// The reference a read-back followed and could not resolve, in the
/// crate's own vocabulary rather than in prose.
///
/// A read-back walks from a topological key to the geometry key it
/// names; either step can come back empty, and which one did is the
/// difference between a stale handle and a corrupt body. Callers with
/// their own stale-reference vocabulary (the operator layer's
/// `EulerOpError::StaleKey` / `StaleGeometry`) map the two arms
/// straight across.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DanglingRef {
    /// A topological key that does not resolve.
    Entity(EntityId),
    /// A geometry key, reached from a live entity, that does not
    /// resolve.
    Geometry(GeomRef),
}

impl From<DanglingRef> for crate::euler::EulerOpError {
    fn from(what: DanglingRef) -> Self {
        match what {
            DanglingRef::Entity(key) => Self::StaleKey { key },
            DanglingRef::Geometry(key) => Self::StaleGeometry { key },
        }
    }
}

/// Typed refusal of a read-back (closed enum, D4 ¶3). Every arm is a
/// fact about the model, not a lane to swallow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadbackError {
    /// A key does not resolve in this body — a stale key, or a key
    /// from another body's lineage (foreign keys are not caught; see
    /// the [`Body`] docs).
    Dangling {
        /// Which lookup came back empty.
        what: DanglingRef,
    },
    /// The carrier stores no canonical frame, so there is none to
    /// report: a NURBS patch or curve has no distinguished origin or
    /// axis, and picking one would fabricate a convention the model
    /// never chose (rule 3).
    NoCanonicalFrame {
        /// The carrier kind that has none.
        carrier: &'static str,
    },
    /// The edge carries M3 null-edge scaffolding rather than a
    /// certified carrier — a transient state tier 2 refuses at rest,
    /// surfaced rather than guessed around.
    NoCarrier,
}

/// **A face's carrier frame** — the stored plane/axis data of the
/// surface the face is a region of, copied out.
///
/// This is rule 2's definitional re-read: no measurement, no pad. It
/// is also rule 1's line — the answer is the frame, never a verdict
/// about what kind of frame it is.
///
/// # Errors
///
/// [`ReadbackError::Dangling`] for a stale face or surface key;
/// [`ReadbackError::NoCanonicalFrame`] for a NURBS carrier.
///
/// ```
/// use geom_core::{Point3, Vec3};
/// use topo::readback::{ReadbackError, face_pose};
/// use topo::{Body, FaceSurface, Surface};
///
/// let mut body = Body::<f64>::new();
/// let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0)).expect("mvfs has no preconditions");
///
/// // A seed face carries the "no description yet" placeholder, which
/// // fixes no frame — and the door says so rather than inventing one.
/// assert!(matches!(
///     face_pose(&body, seed.face),
///     Err(ReadbackError::NoCanonicalFrame { .. })
/// ));
///
/// // Attach a real plane, and the door hands back what was attached.
/// body.set_face_surface(
///     seed.face,
///     FaceSurface::New(Surface::Plane {
///         origin: Point3::new(0.0, 0.0, 1.0),
///         normal: Vec3::new(0.0, 0.0, 1.0),
///         u_ref: Vec3::new(1.0, 0.0, 0.0),
///     }),
/// )
/// .expect("a live face takes a surface");
///
/// let pose = face_pose(&body, seed.face).expect("a planar carrier");
/// assert_eq!(pose.origin.z, 1.0);
/// assert_eq!(pose.axis.z, 1.0);
/// // A plane fixes its in-plane reference direction; the triad is
/// // right-handed.
/// assert_eq!(pose.v_ref().expect("a complete triad").y, 1.0);
/// ```
pub fn face_pose<T: Real>(body: &Body<T>, face: FaceKey) -> Result<Pose<T>, ReadbackError> {
    let f = body.get_face(face).ok_or(ReadbackError::Dangling {
        what: DanglingRef::Entity(EntityId::Face(face)),
    })?;
    let surface = body.get_surface(f.surface).ok_or(ReadbackError::Dangling {
        what: DanglingRef::Geometry(GeomRef::Surface(f.surface)),
    })?;
    let frame = |origin: Point3<T>, axis: Vec3<T>, u_ref: Vec3<T>| Pose {
        origin,
        axis,
        u_ref: Some(u_ref),
    };
    match surface {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => Ok(frame(*origin, *normal, *u_ref)),
        Surface::Cylinder {
            origin,
            axis,
            u_ref,
            ..
        } => Ok(frame(*origin, *axis, *u_ref)),
        Surface::Cone {
            apex, axis, u_ref, ..
        } => Ok(frame(*apex, *axis, *u_ref)),
        Surface::Sphere {
            center,
            axis,
            u_ref,
            ..
        } => Ok(frame(*center, *axis, *u_ref)),
        Surface::Torus {
            center,
            axis,
            u_ref,
            ..
        } => Ok(frame(*center, *axis, *u_ref)),
        Surface::Nurbs(_) => Err(ReadbackError::NoCanonicalFrame {
            carrier: "nurbs surface",
        }),
    }
}

/// **A vertex's position** — the stored point, copied out. The
/// simplest definitional re-read there is.
///
/// This is the one body for the two-step walk vertex → point: the
/// operator layer reaches it through [`vertex_point_ref`] and renames
/// the refusal in its own vocabulary.
///
/// # Errors
///
/// [`ReadbackError::Dangling`] for a stale vertex or point key.
///
/// ```
/// use geom_core::Point3;
/// use topo::Body;
/// use topo::readback::vertex_point;
///
/// let mut body = Body::<f64>::new();
/// let seed = body.mvfs(Point3::new(1.0, 2.0, 3.0)).expect("mvfs has no preconditions");
///
/// assert_eq!(vertex_point(&body, seed.vertex).expect("a live vertex").y, 2.0);
/// ```
pub fn vertex_point<T: Real>(
    body: &Body<T>,
    vertex: VertexKey,
) -> Result<Point3<T>, ReadbackError> {
    vertex_point_ref(body, vertex).map_err(|what| ReadbackError::Dangling { what })
}

/// [`vertex_point`] with the refusal left as the unresolved reference
/// itself, for callers whose own error vocabulary names stale
/// topological and geometry keys separately.
///
/// # Errors
///
/// The [`DanglingRef`] naming whichever of the two lookups — vertex,
/// then its point — came back empty.
pub fn vertex_point_ref<T: Real>(
    body: &Body<T>,
    vertex: VertexKey,
) -> Result<Point3<T>, DanglingRef> {
    let v = body
        .get_vertex(vertex)
        .ok_or(DanglingRef::Entity(EntityId::Vertex(vertex)))?;
    body.get_point(v.point)
        .copied()
        .ok_or(DanglingRef::Geometry(GeomRef::Point(v.point)))
}

/// **An edge's carrier frame** — the certified carrier's own stored
/// frame, copied out.
///
/// The carrier is a cache certified against the edge's intensional
/// description (D4 ¶2), so what comes back is the concrete curve the
/// model actually holds, and its `u_ref` is the seam convention that
/// curve carries. A [`Curve3::Line`] answers with `u_ref: None`: it
/// fixes a direction and no perpendicular (rule 3).
///
/// # Errors
///
/// [`ReadbackError::Dangling`] for a stale edge or curve key;
/// [`ReadbackError::NoCarrier`] for null-edge scaffolding;
/// [`ReadbackError::NoCanonicalFrame`] for a NURBS carrier.
pub fn edge_pose<T: Real>(body: &Body<T>, edge: EdgeKey) -> Result<Pose<T>, ReadbackError> {
    let e = body.get_edge(edge).ok_or(ReadbackError::Dangling {
        what: DanglingRef::Entity(EntityId::Edge(edge)),
    })?;
    let geom = body
        .get_curve_geom(e.curve)
        .ok_or(ReadbackError::Dangling {
            what: DanglingRef::Geometry(GeomRef::Curve(e.curve)),
        })?;
    match geom.certified().ok_or(ReadbackError::NoCarrier)?.carrier() {
        Curve3::Line { origin, dir } => Ok(Pose {
            origin: *origin,
            axis: *dir,
            u_ref: None,
        }),
        Curve3::Circle {
            center,
            axis,
            u_ref,
            ..
        }
        | Curve3::Ellipse {
            center,
            axis,
            u_ref,
            ..
        } => Ok(Pose {
            origin: *center,
            axis: *axis,
            u_ref: Some(*u_ref),
        }),
        Curve3::Nurbs(_) => Err(ReadbackError::NoCanonicalFrame {
            carrier: "nurbs curve",
        }),
    }
}
