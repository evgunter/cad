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
//!    frame is (o, n, u)", "this face's carrier is a plane", "this
//!    face's sense is reversed". No door answers "is this edge convex"
//!    or "is this at z ≈ 1" — NUMERIC predicates are decided-predicate
//!    sites under the margins discipline and stay deferred. A stored
//!    TAG is not one of those: "is this face planar" is a comparison
//!    of the carrier's kind tag against `Plane`, the same exact read
//!    `select_where`'s surface-kind filter makes, and
//!    [`face_carrier_kind`] hands the tag out for exactly that
//!    comparison. Nothing here decides anything: every answer is
//!    stored data, copied out.
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
use geom_brep::SurfaceKind;
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
///   make two different questions share one answer. That second fact
///   travels BESIDE the axis as [`Pose::sense`], so a reader that
///   wants the outward normal forms it as `sense · axis` in the open
///   rather than receiving it pre-folded.
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
    /// **The face's orientation sense**, copied out of the face record
    /// ([`crate::entity::Face::sense`]): `true` when the face's outward
    /// normal is `+axis`, `false` when it is `-axis`. This is the
    /// second fact [`Pose::axis`] deliberately does not fold in — the
    /// axis stays the chart's, and the outward normal is `sense ·
    /// axis`, formed by the reader.
    ///
    /// An EDGE has no orientation sense, so [`edge_pose`] carries
    /// `true` here — the sign that leaves `axis` exactly as the chart
    /// stores it — and the field says nothing about the edge.
    pub sense: bool,
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

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM in read-back's own vocabulary — which lookup came back
// empty, and what that emptiness means about the model. The two
// `Dangling` lanes are kept apart in the prose because they are
// different facts: a topological key that does not resolve is a stale
// or foreign handle, while a geometry key reached FROM a live entity
// that does not resolve is a dangling reference inside the body. The
// keys render through [`EntityId`]/[`GeomRef`]'s own `Display`, this
// crate's noun functions, so a read-back refusal reads exactly like
// the euler-layer stale-key refusal its arms map across to.
impl core::fmt::Display for ReadbackError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Dangling {
                what: DanglingRef::Entity(key),
            } => write!(
                f,
                "read-back: {key} does not resolve in this body — the handle is \
                 stale, or it belongs to another body's lineage"
            ),
            Self::Dangling {
                what: DanglingRef::Geometry(key),
            } => write!(
                f,
                "read-back: a live entity names {key}, which does not resolve — \
                 the body's own geometry reference is dangling"
            ),
            Self::NoCanonicalFrame { carrier } => write!(
                f,
                "read-back: a {carrier} carrier stores no canonical frame, so \
                 there is none to report — it has no distinguished origin or \
                 axis, and picking one would fabricate a convention the model \
                 never chose"
            ),
            Self::NoCarrier => f.write_str(
                "read-back: the edge carries null-edge scaffolding rather than a \
                 certified carrier — a transient state tier 2 refuses at rest; \
                 let the body reach rest before reading it back",
            ),
        }
    }
}

impl std::error::Error for ReadbackError {}

/// **A face's carrier frame** — the stored plane/axis data of the
/// surface the face is a region of, copied out.
///
/// This is rule 2's definitional re-read: no measurement, no pad. It
/// is also rule 1's line — the answer is the frame, never a verdict
/// about what kind of frame it is (that kind is its own read,
/// [`face_carrier_kind`]).
///
/// The face's orientation sense comes back BESIDE the frame
/// ([`Pose::sense`]): `axis` stays the chart's direction, and the
/// outward normal is `sense · axis`, formed by the caller.
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
/// // The sense is the face's stored flag, beside the chart axis —
/// // a seed face is minted agreeing with its chart.
/// assert!(pose.sense);
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
        sense: f.sense,
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
        // No canonical frame: neither the fit (a spline) nor the
        // description (an offset of one) fixes an origin and an axis.
        Surface::Approx(_) => Err(ReadbackError::NoCanonicalFrame {
            carrier: "approximating surface",
        }),
    }
}

/// **A face's carrier kind** — the [`SurfaceKind`] tag of the surface
/// the face is a region of, copied out.
///
/// A tag read, not a verdict (rule 1): the answer is which closed
/// variant the stored surface IS, which is where the model's intent is
/// kept, and comparing it against a kind is the same exact comparison
/// `select_where`'s surface-kind filter makes. "Is this face planar"
/// is `face_carrier_kind(..)? == SurfaceKind::Plane`, and no number
/// is consulted on the way. The total flattening
/// [`crate::query::face_surface_kind`] reads through this door and
/// answers `None` where it refuses typed; the predicate seat wants an honest
/// NO, a read-back wants to know WHICH lookup came back empty.
///
/// # Errors
///
/// [`ReadbackError::Dangling`] for a stale face or surface key — the
/// only refusals: every carrier, NURBS and approximating included,
/// has a kind.
///
/// ```
/// use geom_brep::SurfaceKind;
/// use geom_core::{Point3, Vec3};
/// use topo::readback::face_carrier_kind;
/// use topo::{Body, FaceSurface, Surface};
///
/// let mut body = Body::<f64>::new();
/// let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0)).expect("mvfs has no preconditions");
/// body.set_face_surface(
///     seed.face,
///     FaceSurface::New(Surface::Plane {
///         origin: Point3::new(0.0, 0.0, 0.0),
///         normal: Vec3::new(0.0, 0.0, 1.0),
///         u_ref: Vec3::new(1.0, 0.0, 0.0),
///     }),
/// )
/// .expect("a live face takes a surface");
///
/// assert_eq!(face_carrier_kind(&body, seed.face), Ok(SurfaceKind::Plane));
/// ```
pub fn face_carrier_kind<T: Real>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<SurfaceKind, ReadbackError> {
    let f = body.get_face(face).ok_or(ReadbackError::Dangling {
        what: DanglingRef::Entity(EntityId::Face(face)),
    })?;
    let surface = body.get_surface(f.surface).ok_or(ReadbackError::Dangling {
        what: DanglingRef::Geometry(GeomRef::Surface(f.surface)),
    })?;
    Ok(SurfaceKind::of(surface))
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
            sense: true,
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
            sense: true,
        }),
        Curve3::Nurbs(_) => Err(ReadbackError::NoCanonicalFrame {
            carrier: "nurbs curve",
        }),
    }
}

/// **The Euler–Poincaré census** — the five arena counts the identity
/// `v − e + f − r = 2(s − h)` relates, read off a whole body.
///
/// A count is stored data counted (rule 1: nothing here is decided),
/// and the identity is exact integer arithmetic on the counts (rule
/// 2: no measurement, so no pad). The one thing the census can SAY
/// beyond its numbers is [`EulerCounts::genus`], and it says it typed.
///
/// - `v`, `e`, `f`: the vertex, edge and face arenas' lengths.
/// - `r`: the ring count — every face's ring loops, summed. A face's
///   outer loop is not a ring; only its holes are.
/// - `s`: the SHELL count, which is the identity's `S`. Not the solid
///   count: a solid holding a void has one solid and two shells, and it
///   is the second shell the `2s` term pays for. The two agree on every
///   body that has one shell per solid, which is why a copy that reads
///   the solid arena passes until the first operator separates them —
///   the shell partition and the shell fusion both move `s` and leave
///   the solid count alone.
///
/// The counts are `i64` so that a delta between two censuses, or the
/// identity's own subtraction, needs no cast at the site.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EulerCounts {
    /// Vertices.
    pub v: i64,
    /// Edges.
    pub e: i64,
    /// Faces.
    pub f: i64,
    /// Rings: every face's ring loops, summed.
    pub r: i64,
    /// Shells — the identity's `S`.
    pub s: i64,
}

impl EulerCounts {
    /// **The genus** `h` the identity assigns to the census:
    /// `h = s − (v − e + f − r) / 2`, the number of handles summed over
    /// the body's shells.
    ///
    /// This is the whole-body reading — one number for the body, not
    /// one per shell or per connected component. A shell whose faces
    /// have fallen into several components (the state between a plug
    /// promotion and the shell partition that follows it) still
    /// contributes one `s`, so the per-body `h` can come back NEGATIVE
    /// there; that is the identity's honest arithmetic on that census,
    /// not a refusal, and the door reports it as such. Ask the
    /// validator's component pass for the per-component statement.
    ///
    /// # Errors
    ///
    /// [`EulerParityError`] when `v − e + f − r` is odd. The identity's
    /// left side is even on EVERY body it applies to — every operator
    /// moves it by an even amount — so an odd census is not a body with
    /// a surprising genus, it is a store that is not a B-rep: an entity
    /// minted or killed outside the operators. Halving it would turn
    /// that into a plausible number, so the check comes before the
    /// divide and the refusal carries the census that failed it.
    ///
    /// ```
    /// use geom_core::Point3;
    /// use topo::Body;
    /// use topo::readback::euler_counts;
    ///
    /// let mut body = Body::<f64>::new();
    /// body.mvfs(Point3::new(0.0, 0.0, 0.0)).expect("mvfs has no preconditions");
    ///
    /// // One vertex, one face, one shell: v − e + f − r = 2 = 2(1 − 0).
    /// let counts = euler_counts(&body);
    /// assert_eq!((counts.v, counts.e, counts.f, counts.r, counts.s), (1, 0, 1, 0, 1));
    /// assert_eq!(counts.genus(), Ok(0));
    ///
    /// // A second seed is a second shell, and the identity's `s` counts
    /// // shells: both seeds together are still genus 0.
    /// body.mvfs(Point3::new(1.0, 0.0, 0.0)).expect("mvfs has no preconditions");
    /// let counts = euler_counts(&body);
    /// assert_eq!(counts.s, 2);
    /// assert_eq!(counts.genus(), Ok(0));
    /// ```
    pub fn genus(self) -> Result<i64, EulerParityError> {
        let chi = self.v - self.e + self.f - self.r;
        if chi.rem_euclid(2) != 0 {
            return Err(EulerParityError { counts: self });
        }
        Ok(self.s - chi / 2)
    }
}

/// Typed refusal of [`EulerCounts::genus`]: the census does not satisfy
/// the Euler–Poincaré identity's parity, so no genus follows from it.
///
/// Its own type rather than a [`ReadbackError`] arm because it is a
/// different kind of fact: every `ReadbackError` arm reports a lookup
/// that came back empty or a carrier that fixes no frame — a refusal
/// about one entity, from a door that takes its key. This one is about
/// the whole store, from a value that has already been read; and
/// [`euler_counts`] itself cannot refuse (an arena always has a
/// length), so folding the parity arm into the enum would hand every
/// `face_pose` caller an arm that door can never produce.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EulerParityError {
    /// The census that failed the parity check, verbatim.
    pub counts: EulerCounts,
}

impl core::fmt::Display for EulerParityError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let EulerCounts {
            v,
            e,
            f: faces,
            r,
            s,
        } = self.counts;
        write!(
            f,
            "read-back: the census v={v} e={e} f={faces} r={r} s={s} has odd \
             v − e + f − r = {}, which no Euler–Poincaré body has — the store is \
             torn (an entity was minted or killed outside the operators), so no \
             genus follows from it",
            v - e + faces - r
        )
    }
}

impl std::error::Error for EulerParityError {}

/// **The body's Euler–Poincaré census** — [`EulerCounts`], read off the
/// arenas of the whole body.
///
/// Infallible: an arena always has a length, and a ring count is a
/// length summed. What the census then says about itself is
/// [`EulerCounts::genus`], which is where the identity's one refusal
/// lives.
///
/// ```
/// use geom_core::Point3;
/// use topo::Body;
/// use topo::readback::euler_counts;
///
/// let mut body = Body::<f64>::new();
/// let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0)).expect("mvfs has no preconditions");
/// let counts = euler_counts(&body);
/// assert_eq!(counts.v, 1);
/// assert_eq!(counts.s, 1);
/// assert_eq!(body.get_face(seed.face).map(|face| face.rings.len()), Some(0));
/// assert_eq!(counts.r, 0);
/// ```
#[must_use]
pub fn euler_counts<T: Real>(body: &Body<T>) -> EulerCounts {
    EulerCounts {
        v: body.vertices().count() as i64,
        e: body.edges().count() as i64,
        f: body.faces().count() as i64,
        r: body.faces().map(|(_, face)| face.rings.len() as i64).sum(),
        s: body.shells().count() as i64,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::{Point3, Tol};

    use super::{EulerCounts, EulerParityError, euler_counts};
    use crate::entity::Vertex;
    use crate::fixtures::{ops_cube, ops_holed_box, prov};

    #[test]
    fn cube_counts_and_genus_zero() {
        let body = ops_cube(Tol::witness()).body;
        let counts = euler_counts(&body);
        assert_eq!(
            counts,
            EulerCounts {
                v: 8,
                e: 12,
                f: 6,
                r: 0,
                s: 1
            }
        );
        assert_eq!(counts.genus(), Ok(0));
    }

    #[test]
    fn holed_box_counts_and_genus_one() {
        // v − e + f − r = 16 − 24 + 10 − 2 = 0 = 2(1 − 1): the through-hole
        // leaves a ring on each of the top and bottom faces.
        let body = ops_holed_box(Tol::witness()).body;
        let counts = euler_counts(&body);
        assert_eq!(
            counts,
            EulerCounts {
                v: 16,
                e: 24,
                f: 10,
                r: 2,
                s: 1
            }
        );
        assert_eq!(counts.genus(), Ok(1));
    }

    /// The shell term: a second seed beside the cube is a second shell
    /// (and, minted by `mvfs`, a second solid). Re-homing that shell
    /// into the cube's solid — the same-solid two-shell shape the
    /// fusion form exists for, reachable only by raw in-crate write —
    /// moves the solid count and NOT the census: `s` counts shells,
    /// and the genus is where it was.
    #[test]
    fn two_shells_count_as_two_whatever_the_solid_count() {
        let t = ops_cube(Tol::witness());
        let mut body = t.body;
        let other = body.mvfs(Point3::new(9.0, 9.0, 9.0)).unwrap();
        let two_solids = euler_counts(&body);
        assert_eq!((two_solids.s, body.solids().count()), (2, 2));
        assert_eq!((two_solids.v, two_solids.f), (9, 7));
        assert_eq!(two_solids.genus(), Ok(0));

        let cube_solid = body.get_shell(t.seed.shell).unwrap().solid;
        body.get_shell_mut(other.shell).unwrap().solid = cube_solid;
        body.get_solid_mut(cube_solid)
            .unwrap()
            .shells
            .push(other.shell);
        body.get_solid_mut(other.solid).unwrap().shells.clear();
        assert_eq!(body.solids().count(), 2, "the emptied solid still exists");
        assert_eq!(
            body.get_solid(cube_solid).unwrap().shells.len(),
            2,
            "one solid now holds both shells"
        );
        assert_eq!(euler_counts(&body), two_solids, "the census reads shells");
        assert_eq!(euler_counts(&body).genus(), Ok(0));
    }

    /// Red-first: a vertex minted outside the operators tears the
    /// store's parity, and the genus refuses typed with the census that
    /// failed rather than halving an odd number into a plausible one.
    #[test]
    fn torn_store_refuses_typed() {
        let mut body = ops_cube(Tol::witness()).body;
        let point = body.add_point(Point3::new(0.5, 0.5, 0.5));
        body.add_vertex(
            Vertex {
                point,
                emanating: None,
            },
            prov(),
        );
        let counts = euler_counts(&body);
        assert_eq!(counts.v, 9);
        let refusal = counts.genus().expect_err("9 − 12 + 6 − 0 = 3 is odd");
        assert_eq!(refusal, EulerParityError { counts });
        let text = refusal.to_string();
        assert!(text.contains("v=9 e=12 f=6 r=0 s=1"), "{text}");
        assert!(text.contains("torn"), "{text}");
    }
}
