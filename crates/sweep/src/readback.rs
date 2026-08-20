//! **Read-back doors: what did the model choose?** (LIB-U5).
//!
//! Every op in this crate answers a question the author asked
//! implicitly — where a cap plane landed, what frame a wall's carrier
//! sits in, which arc a corner fillet became. Until this module the
//! only way to ask was to hand-scan the result body's arenas (or to
//! transcribe the answer as a literal and hope it stayed true). These
//! doors ask the model instead.
//!
//! # The three rules these doors keep
//!
//! 1. **Values, never verdicts.** A door answers "this face's carrier
//!    frame is (o, n, u)". No door answers "is this face planar", "is
//!    this edge convex", "is this at z ≈ 1" — geometric predicates are
//!    decided-predicate sites under the margins discipline and stay
//!    deferred (LIB-LOG LB7). Nothing here decides anything: every
//!    answer is stored data, copied out.
//! 2. **Definitional re-read carries no pad.** The produced surface IS
//!    the definition (DESIGN Q8) — reading a plane's stored origin and
//!    normal back is a re-read of authored data, not a measurement, so
//!    there is no residual to certify. The rule's other half binds
//!    too: any answer sourced from an APPROXIMATING or quadrature
//!    construction must carry its certified residual, exactly as
//!    `topo::MassProperties` carries `volume_pad`. No door here reads
//!    such a source; when one does, it grows a pad field.
//! 3. **No invented conventions.** Where the stored geometry fixes no
//!    frame — a NURBS patch has no canonical origin or axis, a line
//!    has no distinguished perpendicular — the door says so
//!    ([`ReadbackError::NoCanonicalFrame`], [`Pose::u_ref`]'s `None`)
//!    rather than fabricating one that would then be quoted back as
//!    though the model had chosen it.
//!
//! # Layering
//!
//! These are KERNEL doors: body plus key in, values out. The
//! document-layer twins that take a `StableName` instead of an arena
//! key live in `editor-core` and delegate here, so there is one
//! reading of any given piece of geometry, not two.

use geom::Curve3;
use geom::Surface;
use geom_core::{Point3, Real, Vec3};
use profile::{SegmentKind, ValidatedLoop};
use topo::{Body, EdgeKey, FaceKey, VertexKey};

use crate::extrude::Extruded;
use crate::loft::Lofted;
use crate::revolve::{Revolved, RevolvedKind};

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

/// Typed refusal of a read-back (closed enum, D4 ¶3). Every arm is a
/// fact about the model, not a lane to swallow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadbackError {
    /// A key does not resolve in this body — a stale key, or a key
    /// from another body's lineage (foreign keys are not caught; see
    /// the `topo::Body` docs).
    Dangling {
        /// Which arena lookup came back empty.
        what: &'static str,
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
    /// The operation has no caps to read: a FULL revolve closes on
    /// itself (`RevolvedKind::Full`), so there is no start or end
    /// plane.
    NoCaps,
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
/// use geom_core::Tolerance;
/// use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
/// use sweep::{Extrusion, extrude, readback::face_pose};
/// use geom_core::Point2;
///
/// let square = ProfileLoop::polygon(
///     [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
///         .iter()
///         .map(|&(x, y)| Point2::new(x, y)),
/// );
/// let profile = Profile::new(SketchPlane::xy(), vec![square])
///     .validate(Tolerance::get())
///     .expect("the unit square validates");
/// let block = extrude::<f64>(&profile, Extrusion::Distance(1.0)).expect("it extrudes");
///
/// // The top cap sits on the sketch plane translated by the
/// // extrusion — z = 1, normal along +z. No literal was transcribed
/// // to learn that.
/// let top = face_pose(&block.body, block.top).expect("a planar cap");
/// assert_eq!(top.origin.z, 1.0);
/// assert_eq!(top.axis.z, 1.0);
/// // A plane fixes its in-plane reference direction; the triad is
/// // right-handed.
/// assert!(top.u_ref.is_some());
/// assert_eq!(top.v_ref().expect("a complete triad").y, 1.0);
/// ```
pub fn face_pose<T: Real>(body: &Body<T>, face: FaceKey) -> Result<Pose<T>, ReadbackError> {
    let f = body.get_face(face).ok_or(ReadbackError::Dangling {
        what: "face_pose: face",
    })?;
    let surface = body.get_surface(f.surface).ok_or(ReadbackError::Dangling {
        what: "face_pose: surface",
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
/// # Errors
///
/// [`ReadbackError::Dangling`] for a stale vertex or point key.
///
/// ```
/// use geom_core::{Point2, Tolerance};
/// use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
/// use sweep::{Extrusion, extrude, readback::vertex_point};
///
/// let square = ProfileLoop::polygon(
///     [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
///         .iter()
///         .map(|&(x, y)| Point2::new(x, y)),
/// );
/// let profile = Profile::new(SketchPlane::xy(), vec![square])
///     .validate(Tolerance::get())
///     .expect("the unit square validates");
/// let block = extrude::<f64>(&profile, Extrusion::Distance(1.0)).expect("it extrudes");
///
/// // Every corner of the unit block, read back off the model.
/// let mut zs: Vec<f64> = block
///     .body
///     .vertices()
///     .map(|(k, _)| vertex_point(&block.body, k).expect("a live vertex").z)
///     .collect();
/// zs.sort_by(f64::total_cmp);
/// assert_eq!(zs, vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]);
/// ```
pub fn vertex_point<T: Real>(
    body: &Body<T>,
    vertex: VertexKey,
) -> Result<Point3<T>, ReadbackError> {
    let v = body.get_vertex(vertex).ok_or(ReadbackError::Dangling {
        what: "vertex_point: vertex",
    })?;
    body.get_point(v.point)
        .copied()
        .ok_or(ReadbackError::Dangling {
            what: "vertex_point: point",
        })
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
///
/// ```
/// use geom_core::{Point2, Tolerance};
/// use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
/// use sweep::{Extrusion, extrude, readback::edge_pose};
///
/// let square = ProfileLoop::polygon(
///     [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
///         .iter()
///         .map(|&(x, y)| Point2::new(x, y)),
/// );
/// let profile = Profile::new(SketchPlane::xy(), vec![square])
///     .validate(Tolerance::get())
///     .expect("the unit square validates");
/// let block = extrude::<f64>(&profile, Extrusion::Distance(1.0)).expect("it extrudes");
///
/// // Every edge of a block is straight: a direction, and honestly no
/// // reference perpendicular.
/// for (k, _) in block.body.edges() {
///     let pose = edge_pose(&block.body, k).expect("a certified carrier");
///     assert!(pose.u_ref.is_none());
///     assert!(pose.v_ref().is_none());
/// }
/// ```
pub fn edge_pose<T: Real>(body: &Body<T>, edge: EdgeKey) -> Result<Pose<T>, ReadbackError> {
    let e = body.get_edge(edge).ok_or(ReadbackError::Dangling {
        what: "edge_pose: edge",
    })?;
    let geom = body
        .get_curve_geom(e.curve)
        .ok_or(ReadbackError::Dangling {
            what: "edge_pose: curve",
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

/// The two cap frames of a stacked sweep (extrude, loft), in the
/// operation's own vocabulary.
#[derive(Clone, Copy, Debug)]
pub struct CapFrames<T: Real> {
    /// The bottom cap's carrier frame (section 0's plane).
    pub bottom: Pose<T>,
    /// The top cap's carrier frame (the last section's plane).
    pub top: Pose<T>,
}

/// The two wedge-cap frames of a PARTIAL revolve, in the operation's
/// own vocabulary.
#[derive(Clone, Copy, Debug)]
pub struct WedgeFrames<T: Real> {
    /// The start cap's carrier frame (the sketch plane).
    pub start: Pose<T>,
    /// The end cap's carrier frame (the sketch plane rotated by θ).
    pub end: Pose<T>,
}

/// **Where did the extrusion's caps land?** — both cap planes, read
/// off the result's own face handles.
///
/// # Errors
///
/// Every [`face_pose`] refusal.
///
/// ```
/// use geom_core::{Point2, Tolerance};
/// use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
/// use sweep::{Extrusion, extrude, readback::extruded_caps};
///
/// let square = ProfileLoop::polygon(
///     [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
///         .iter()
///         .map(|&(x, y)| Point2::new(x, y)),
/// );
/// let profile = Profile::new(SketchPlane::xy(), vec![square])
///     .validate(Tolerance::get())
///     .expect("the unit square validates");
/// let block = extrude::<f64>(&profile, Extrusion::Distance(2.0)).expect("it extrudes");
///
/// let caps = extruded_caps(&block).expect("planar caps");
/// assert_eq!(caps.bottom.origin.z, 0.0);
/// assert_eq!(caps.top.origin.z, 2.0);
/// // Each cap's chart normal points out of its own end: the door
/// // reports the CHART's direction, not the face's outward sense
/// // (which is a separate question it deliberately does not answer).
/// assert_eq!(caps.bottom.axis.z, -1.0);
/// assert_eq!(caps.top.axis.z, 1.0);
/// ```
pub fn extruded_caps<T: Real>(e: &Extruded<T>) -> Result<CapFrames<T>, ReadbackError> {
    Ok(CapFrames {
        bottom: face_pose(&e.body, e.bottom)?,
        top: face_pose(&e.body, e.top)?,
    })
}

/// **Where did the loft's caps land?** — [`extruded_caps`] for the
/// stacked-section ops.
///
/// # Errors
///
/// Every [`face_pose`] refusal.
///
/// ```
/// use geom_core::{Affine3, Point2, Vec3};
/// use profile::RawLoop;
/// use sweep::{ProfileLoop, Section, loft_body, readback::lofted_caps};
///
/// let quad = |pts: [(f64, f64); 4]| -> Section {
///     vec![ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)))]
/// };
/// let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
/// let sections = vec![quad(square), quad(square), quad(square)];
/// let places: Vec<Affine3<f64>> = [0.0, 1.0, 2.5]
///     .iter()
///     .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
///     .collect();
/// let loft = loft_body::<f64>(&sections, &places, 2).expect("it lofts");
///
/// let caps = lofted_caps(&loft).expect("planar caps");
/// assert_eq!(caps.bottom.origin.z, 0.0);
/// assert_eq!(caps.top.origin.z, 2.5);
/// // And the v-parameters those sections landed at, from the same
/// // result — no hand derivation (LIB-U5 deliverable 1).
/// assert_eq!(loft.section_params.first(), Some(&0.0));
/// assert_eq!(loft.section_params.last(), Some(&1.0));
/// ```
pub fn lofted_caps<T: Real>(l: &Lofted<T>) -> Result<CapFrames<T>, ReadbackError> {
    Ok(CapFrames {
        bottom: face_pose(&l.body, l.bottom)?,
        top: face_pose(&l.body, l.top)?,
    })
}

/// **Where did the partial revolve's wedge caps land?** — the joint
/// frames of a tube: each cap plane's origin and normal, which is
/// exactly the tube's end tangent there when the profile is a
/// cross-section.
///
/// A FULL revolve closes on itself and has no caps: that is
/// [`ReadbackError::NoCaps`], a fact about the operation rather than
/// an empty answer.
///
/// # Errors
///
/// [`ReadbackError::NoCaps`] for a full revolve; every [`face_pose`]
/// refusal.
///
/// ```
/// use geom_core::{Point2, Tolerance, Vec2};
/// use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
/// use sweep::{Revolution, RevolveAxis, readback::revolved_caps, revolve};
///
/// // A quarter tube: a small circle a distance 5 from the axis,
/// // revolved a quarter turn about the sketch frame's +v.
/// let circle = profile::circle(Point2::new(5.0, 0.0), 0.5).expect("a positive radius");
/// // The complete-loop primitives answer with a `ClosedLoop` (the
/// // lowered loop plus its program); `Profile` takes the loop.
/// let sketch = Profile::new(SketchPlane::xy(), vec![circle.into()])
///     .validate(Tolerance::get())
///     .expect("the circle validates");
/// let axis = RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) };
/// let quarter = revolve::<f64>(
///     &sketch,
///     axis,
///     Revolution::Partial(std::f64::consts::FRAC_PI_2),
/// )
/// .expect("the tube revolves");
///
/// let caps = revolved_caps(&quarter).expect("a partial revolve has caps");
/// // The start cap IS the sketch plane, so its normal is the sketch
/// // normal (+z) — and that is the tube's end tangent there.
/// assert!(caps.start.axis.z.abs() > 0.99);
/// // A quarter turn about +y later, the end cap's normal has turned
/// // with it, onto x.
/// assert!(caps.end.axis.x.abs() > 0.99);
/// ```
pub fn revolved_caps<T: Real>(r: &Revolved<T>) -> Result<WedgeFrames<T>, ReadbackError> {
    let RevolvedKind::Partial {
        start_cap, end_cap, ..
    } = r.kind
    else {
        return Err(ReadbackError::NoCaps);
    };
    Ok(WedgeFrames {
        start: face_pose(&r.body, start_cap)?,
        end: face_pose(&r.body, end_cap)?,
    })
}

/// One blend arc of a validated loop: which canonical segment it is,
/// and the arc data the profile layer classified it as.
#[derive(Clone, Copy, Debug)]
pub struct BlendArc<T: Real> {
    /// The canonical segment index within the loop (segment `j` runs
    /// from vertex `j` to vertex `j + 1 mod n`).
    pub segment: usize,
    /// The classified carrier — always [`SegmentKind::Arc`] here.
    pub kind: SegmentKind<T>,
}

/// **Which segment is the fillet at corner k?** — every arc segment
/// the loop declares TANGENT at both of its junctions, in CANONICAL
/// segment order.
///
/// **Authored order is not preserved, and entry `k` is not the k-th
/// fillet you wrote.** Validation canonicalizes: it rotates the chain
/// to its lex-min vertex, and it REVERSES a loop wound the wrong way
/// for its role — so a hole authored with fillets `[r1, r2]` answers
/// `[r2, r1]`. Correlate by [`BlendArc::segment`] (a canonical index,
/// which is the only index this layer has) or by the arc data itself,
/// which is self-identifying; do not index by authoring position.
///
/// A corner fillet leaves exactly this shape behind: an arc that
/// meets both legs tangentially, with both junctions declared (and
/// verified — validation refuses an unmet declaration). So the door
/// is STRUCTURAL — it reads `ValidatedLoop::tangent_joints`, decides
/// nothing, and compares no floats. That is what makes it a
/// replacement for the alternative every caller wrote instead:
/// scanning the segments for "the arc whose radius equals R", which
/// finds the wrong arc as soon as two blends share a radius, and
/// silently finds nothing when the stored radius drifts an ulp.
///
/// **What it reports, exactly.** Any tangent-continuous arc answers,
/// not only arcs an author reached through fillet sugar — a full
/// circle lowered as two tangent semicircles presents the same shape
/// and is listed too. This is a read of the loop's structure, not a
/// verdict about intent (rule 1), and the docs say so rather than the
/// door guessing.
///
/// ```
/// use geom_core::{Point2, Tolerance};
/// use profile::{ArcSweep, Center, Open, Profile, SegmentKind, SketchPlane, Start};
/// use sweep::readback::blend_arcs;
///
/// // The tour rocker's eye slot: two R = 1 lobes meeting tip to tip,
/// // the TOP tip filleted at R = 1/4, the bottom left sharp.
/// let tip = 0.75f64.sqrt();
/// let eye = Open
///     .arc_fillet_arc(
///         Center {
///             c: Point2::new(-0.5, 0.0),
///             winding: ArcSweep::Ccw,
///             p: Point2::new(0.0, -tip),
///         },
///         0.25,
///         Center {
///             c: Point2::new(0.5, 0.0),
///             winding: ArcSweep::Ccw,
///             p: Start,
///         },
///     )
///     .expect("the near candidate resolves the tip");
/// let slot = Profile::new(SketchPlane::xy(), vec![eye.into()])
///     .validate(Tolerance::get())
///     .expect("the eye slot validates");
///
/// // One filleted corner, found by structure — no radius scan.
/// let blends = blend_arcs(&slot.loops()[0]);
/// assert_eq!(blends.len(), 1);
/// match blends[0].kind {
///     SegmentKind::Arc { center, radius, .. } => {
///         assert!((radius - 0.25).abs() < 1e-12);
///         assert!(center.x.abs() < 1e-12);
///     }
///     SegmentKind::Line => panic!("a fillet is an arc"),
/// }
/// ```
#[must_use]
pub fn blend_arcs<T: Real>(lp: &ValidatedLoop<T>) -> Vec<BlendArc<T>> {
    let segments = lp.segments();
    let n = segments.len();
    let joints = lp.tangent_joints();
    let tangent_at = |v: usize| joints.binary_search(&v).is_ok();
    (0..n)
        .filter(|&j| matches!(segments[j].kind, SegmentKind::Arc { .. }))
        .filter(|&j| tangent_at(j) && tangent_at((j + 1) % n))
        .map(|j| BlendArc {
            segment: j,
            kind: segments[j].kind,
        })
        .collect()
}
