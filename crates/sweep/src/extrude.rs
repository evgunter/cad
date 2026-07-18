//! The extrude operation: a validated profile swept along its sketch
//! plane's normal into a closed solid (M2 PR 4).
//!
//! Algorithm (Mäntylä §12.3's translational sweep, re-derived under our
//! counterclockwise convention — see the crate docs for the direction
//! conventions this crate owns):
//!
//! 1. **Base lamina.** The outer loop's chain is laid down on the
//!    sketch plane (`mvfs` + a `mev` chain + one closing `mef`), giving
//!    two faces: the seed face (which will be swept and survive as the
//!    **top cap**) keeps the swept-traversal winding; the `mef`-minted
//!    face is the **bottom cap** (reversed winding, outward normal
//!    opposite the extrusion), and receives its Newell plane at the
//!    `mef` itself.
//! 2. **Holes.** Each hole is planted in the seed face as a ring
//!    (bridge `mev` + `kemr` — the empty-ring hole-planting state),
//!    grown by a `mev` chain, closed by a `mef` whose transient disc
//!    face is immediately consumed by a same-shell `kfmrh` demoting the
//!    disc loop into the bottom cap. Genus rises by one per hole.
//! 3. **Sweep.** Per loop of the seed face (outer, then rings in
//!    canonical hole order): one strut `mev` per vertex (the
//!    `he1 == he2` case — raised vertex, `ExtrudedPoint` description),
//!    then one side-quad `mef` per segment (the new edge is the **top
//!    rim**, `PlacedSegment` at the translated placement; the new face
//!    is the side wall, plane or cylinder), the last `mef` closing
//!    against the first top rim. The original loop survives raised —
//!    the swept face becomes the top cap.
//! 4. **Joins.** Per strut: identical side-surface keys (cosurface
//!    sharing) keep the conventional description structurally;
//!    otherwise `classify_dihedral` at the strut midpoint decides —
//!    Transverse upgrades to `Intersection` via `set_edge_curve`,
//!    Smooth keeps `MappedCurve`, Indeterminate is a typed sliver
//!    error.
//! 5. **Top cap.** The seed face's surface (the honest `Nurbs`
//!    placeholder since `mvfs`) is replaced by the raised loop's Newell
//!    plane.
//!
//! Everything runs in a fixed, documented order (D9): loops outer
//! first then holes in canonical order; per loop, struts in traversal
//! order, then side faces, then join classification. Two calls with
//! identical inputs replay byte-identically.

use core::fmt;

use geom_brep::{
    DihedralClass, EdgeCurveSpec, EdgeGeometry, MappedCurve, NewellError, SketchSegment,
    classify_dihedral, newell_plane,
};
use geom_core::{
    Affine3, Band, BandError, Decide, Indeterminate, Point2, Point3, Real, Sign, Vec3,
};
use geom_curves::Curve3;
use geom_surfaces::Surface;
use profile::{SegmentKind, ValidatedLoop, ValidatedProfile};
use topo::{
    Body, EdgeKey, EulerOpError, FaceKey, FaceSurface, MefSite, MevCreated, MevSite, ShellKey,
    SolidKey, SurfaceKey,
};

/// How far (and which way) to extrude — the operation's second input.
#[derive(Clone, Copy, Debug)]
pub enum Extrusion<T: Real> {
    /// An explicit extrusion vector in world meters. Must be
    /// trilean-parallel to the sketch plane's normal (crate docs:
    /// oblique extrusion is deferred past M2 and refused as a typed
    /// error); a definite in-plane or sliver-normal vector is refused
    /// likewise.
    Vector(Vec3<T>),
    /// A signed distance along the sketch plane's normal `n = u × v`
    /// (meters): the extrusion vector is `n · d`. Positive extrudes
    /// along `+n`, negative along `−n`.
    Distance(T),
}

/// Everything [`extrude`] built, keyed: the body plus the handles tests
/// and downstream passes (tessellation, mass properties) address it by.
#[derive(Debug)]
pub struct Extruded<T: Real> {
    /// The built body — a closed solid passing tiers 1–3.
    pub body: Body<T>,
    /// The solid.
    pub solid: SolidKey,
    /// Its single shell.
    pub shell: ShellKey,
    /// The top cap (the swept face — on the sketch plane translated by
    /// the extrusion vector; carries the profile's canonical winding
    /// when extruding along `+n`, crate docs).
    pub top: FaceKey,
    /// The bottom cap (on the sketch plane).
    pub bottom: FaceKey,
    /// Side-wall faces, per loop (outer first, then holes in canonical
    /// order), per segment in swept-traversal order.
    pub side_faces: Vec<Vec<FaceKey>>,
    /// Strut (join) edges, per loop, per vertex in swept-traversal
    /// order: strut `j` joins the side walls of segments `j − 1` and
    /// `j`. Corner joins carry `Intersection` descriptions after the
    /// upgrade pass; smooth joins keep `MappedCurve`.
    pub strut_edges: Vec<Vec<EdgeKey>>,
}

/// Typed failure of [`extrude`] (closed enum, D4 ¶3). Loop indices
/// reference the **canonical** (validated) profile — loop 0 the outer,
/// then holes in canonical order; vertex/segment indices reference the
/// canonical chain of that loop.
#[derive(Clone, Debug, PartialEq)]
pub enum ExtrudeError {
    /// The run's tolerance could not form a classification band (see
    /// [`BandError`]).
    Band(BandError),
    /// The extrusion is definitely degenerate: the vector's normal
    /// component (or the signed distance) is coincident with zero at
    /// tolerance — an in-plane vector or a sliver-thin extrusion.
    DegenerateExtrusion,
    /// The extrusion vector has a definite in-plane component: oblique
    /// extrusion is deferred past M2 (under shear, arc segments sweep
    /// elliptic cylinders, which the D3 surface set does not carry) and
    /// refused rather than approximated.
    ObliqueExtrusion,
    /// The extrusion-vector classification escalated: the normal or
    /// in-plane component landed in the sliver band or was poisoned
    /// (named predicate on the diagnostic).
    ExtrusionEscalated {
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// A cosurface-sharing predicate escalated at a join: the geometry
    /// is neither definitely one carrier nor definitely two.
    CosurfaceEscalated {
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the join vertex.
        vertex_index: usize,
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// The dihedral classification at a profile-corner join escalated:
    /// a sliver dihedral, certifiable as neither a corner nor a smooth
    /// join (D2's ratified text — a conventional description is not an
    /// escape hatch from ill-conditioned geometry).
    SliverJoin {
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the join vertex.
        vertex_index: usize,
        /// The classifier's diagnostic.
        source: Indeterminate,
    },
    /// A cap plane failed Newell certification (non-planar or
    /// degenerate loop data — unreachable for validated profiles,
    /// surfaced rather than trusted).
    CapPlane {
        /// The Newell failure.
        source: NewellError,
    },
    /// A side-wall plane failed Newell certification.
    SidePlane {
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the segment.
        segment_index: usize,
        /// The Newell failure.
        source: NewellError,
    },
    /// An Euler operator or attachment gate refused — including every
    /// D4 ¶2 certification failure, which arrives as
    /// [`EulerOpError::Certification`] carrying the certification
    /// report.
    Op {
        /// The operator-layer failure.
        source: EulerOpError,
    },
}

impl fmt::Display for ExtrudeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band(e) => write!(f, "extrude could not form a band: {e}"),
            Self::DegenerateExtrusion => f.write_str(
                "extrusion vector/distance has no definite normal component: in-plane or \
                 sliver-thin extrusion (D4)",
            ),
            Self::ObliqueExtrusion => f.write_str(
                "extrusion vector has a definite in-plane component: oblique extrusion is \
                 deferred past M2 (arc segments would sweep elliptic cylinders)",
            ),
            Self::ExtrusionEscalated { source } => {
                write!(f, "extrusion-vector classification escalated: {source}")
            }
            Self::CosurfaceEscalated {
                loop_index,
                vertex_index,
                source,
            } => write!(
                f,
                "cosurface sharing at loop {loop_index} vertex {vertex_index} escalated: \
                 {source}"
            ),
            Self::SliverJoin {
                loop_index,
                vertex_index,
                source,
            } => write!(
                f,
                "sliver dihedral at loop {loop_index} vertex {vertex_index}: the join is \
                 neither a definite corner nor definitely smooth: {source}"
            ),
            Self::CapPlane { source } => write!(f, "cap plane: {source}"),
            Self::SidePlane {
                loop_index,
                segment_index,
                source,
            } => write!(
                f,
                "side plane at loop {loop_index} segment {segment_index}: {source}"
            ),
            Self::Op { source } => write!(f, "extrude operator step failed: {source}"),
        }
    }
}

impl std::error::Error for ExtrudeError {}

impl From<EulerOpError> for ExtrudeError {
    fn from(source: EulerOpError) -> Self {
        Self::Op { source }
    }
}

/// The one classification funnel of this crate (the `geom-brep`
/// pattern): names the predicate, classifies through the sanctioned
/// [`Decide`] door, tags any escalation. PR 7 wires these names to the
/// recording funnel.
fn decide<T: Decide>(name: &'static str, margin: T, band: Band) -> Result<Sign, Indeterminate> {
    margin.sign_within(band).map_err(|e| e.with_predicate(name))
}

/// A segment's carrier class in swept traversal order (the canonical
/// classification carried through the optional reversal — never
/// re-decided from scalar data here).
#[derive(Clone, Copy, Debug)]
enum SweptKind<T: Real> {
    Line,
    Arc {
        center: Point2<T>,
        radius: T,
        /// Turn sense in swept traversal: `Positive` = counterclockwise
        /// in sketch coordinates. Never `Zero` (upstream classification;
        /// kept total — a `Zero` would take the `Positive` arm).
        turn: Sign,
    },
}

/// One segment of a swept loop, in swept traversal order (canonical, or
/// reversed for extrusion along `−n` — crate docs), with its canonical
/// indices for error reporting.
#[derive(Clone, Copy, Debug)]
struct SweptSeg<T: Real> {
    /// Start point, sketch coordinates. Vertex `j` of the swept chain
    /// is segment `j`'s start.
    a: Point2<T>,
    /// End point.
    b: Point2<T>,
    /// The bulge in swept traversal (negated by reversal).
    bulge: T,
    kind: SweptKind<T>,
    /// Canonical index of the start vertex (for error reporting).
    canonical_vertex: usize,
    /// Canonical index of the segment (for error reporting).
    canonical_segment: usize,
}

/// Builds the swept traversal of one canonical loop: forward, or
/// reversed via the profile crate's reversal involution (endpoints
/// swapped, bulge negated, turn flipped).
fn swept_segments<T: Decide>(lp: &ValidatedLoop<T>, reverse: bool) -> Vec<SweptSeg<T>> {
    let segs = lp.segments();
    let n = segs.len();
    let mut out = Vec::with_capacity(n);
    if reverse {
        // Swept segment j retraces canonical segment n−1−j; swept
        // vertex j is canonical vertex (n−j) mod n.
        for j in 0..n {
            let s = &segs[n - 1 - j];
            out.push(SweptSeg {
                a: s.end,
                b: s.start,
                bulge: T::zero() - s.bulge,
                kind: match s.kind {
                    SegmentKind::Line => SweptKind::Line,
                    SegmentKind::Arc {
                        center,
                        radius,
                        turn,
                    } => SweptKind::Arc {
                        center,
                        radius,
                        turn: turn.flip(),
                    },
                },
                canonical_vertex: (n - j) % n,
                canonical_segment: n - 1 - j,
            });
        }
    } else {
        for (j, s) in segs.iter().enumerate() {
            out.push(SweptSeg {
                a: s.start,
                b: s.end,
                bulge: s.bulge,
                kind: match s.kind {
                    SegmentKind::Line => SweptKind::Line,
                    SegmentKind::Arc {
                        center,
                        radius,
                        turn,
                    } => SweptKind::Arc {
                        center,
                        radius,
                        turn,
                    },
                },
                canonical_vertex: j,
                canonical_segment: j,
            });
        }
    }
    out
}

impl<T: Real> SweptSeg<T> {
    /// The segment as a `geom-brep` sketch segment (the description's
    /// authoritative source data).
    fn sketch_segment(&self) -> SketchSegment<T> {
        match self.kind {
            SweptKind::Line => SketchSegment::Line {
                a: self.a,
                b: self.b,
            },
            SweptKind::Arc { .. } => SketchSegment::Arc {
                a: self.a,
                b: self.b,
                bulge: self.bulge,
            },
        }
    }
}

/// The arc apex (the profile crate's exact sagitta closed form:
/// `midpoint − n̂·(L·b/2)`, n̂ the left normal of the chord direction) —
/// an on-carrier, on-cap-plane interior point of the segment.
fn arc_apex<T: Real>(s: &SweptSeg<T>) -> Point2<T> {
    let chord = s.b - s.a;
    let len = chord.norm();
    let u = chord.normalize();
    let nhat = geom_core::Vec2::new(T::zero() - u.y, u.x);
    let mid = s.a.lerp(s.b, T::from_f64(0.5));
    mid - nhat * (len * s.bulge * T::from_f64(0.5))
}

/// The world points determining a cap plane, in forward swept order:
/// every loop vertex, plus every arc segment's apex. The apexes keep
/// 2-vertex loops (the minimal circle) plane-determining — Newell needs
/// three points and a 2-vertex cap has only two vertices — and they
/// carry the traversal's winding faithfully (each sits between its
/// segment's endpoints in loop order).
fn cap_points<T: Real>(
    segs: &[SweptSeg<T>],
    qs: &[Point3<T>],
    place: Affine3<T>,
) -> Vec<Point3<T>> {
    let mut pts = Vec::with_capacity(segs.len() * 2);
    for (j, s) in segs.iter().enumerate() {
        pts.push(qs[j]);
        if matches!(s.kind, SweptKind::Arc { .. }) {
            let apex = arc_apex(s);
            pts.push(place.transform_point(Point3::new(apex.x, apex.y, T::zero())));
        }
    }
    pts
}

/// A rim-edge spec (bottom or top, per `place`): `PlacedSegment`
/// description, line/circle carrier per the crate docs' carrier
/// conventions (arc axis = turn-signed plane normal, span θ =
/// 4·atan|bulge| from the stored bulge).
fn rim_spec<T: Real>(
    seg: &SweptSeg<T>,
    place: Affine3<T>,
    normal: Vec3<T>,
    q_from: Point3<T>,
    q_to: Point3<T>,
) -> EdgeCurveSpec<T> {
    let description = EdgeGeometry::MappedCurve(MappedCurve::PlacedSegment {
        segment: seg.sketch_segment(),
        place,
    });
    match seg.kind {
        SweptKind::Line => EdgeCurveSpec {
            description,
            carrier: Curve3::Line {
                origin: q_from,
                dir: (q_to - q_from).normalize(),
            },
            param_start: T::zero(),
            param_end: q_from.distance(q_to),
        },
        SweptKind::Arc {
            center,
            radius,
            turn,
        } => {
            let c_world = place.transform_point(Point3::new(center.x, center.y, T::zero()));
            EdgeCurveSpec {
                description,
                carrier: Curve3::Circle {
                    center: c_world,
                    axis: turn_axis(turn, normal),
                    radius,
                    u_ref: (q_from - c_world).normalize(),
                },
                param_start: T::zero(),
                param_end: arc_span(seg.bulge),
            }
        }
    }
}

/// The arc parameter span θ = 4·atan|bulge| (the sanctioned bulge
/// re-inspection — never endpoint `atan2`).
fn arc_span<T: Real>(bulge: T) -> T {
    T::from_f64(4.0) * bulge.abs().atan()
}

/// The turn-signed carrier axis (crate docs): `+normal` for a
/// counterclockwise segment, `−normal` for a clockwise one. `Zero` is
/// unreachable for classified arcs (a zero turn classified as a line);
/// kept total by taking the positive arm.
fn turn_axis<T: Real>(turn: Sign, normal: Vec3<T>) -> Vec3<T> {
    match turn {
        Sign::Positive | Sign::Zero => normal,
        Sign::Negative => Vec3::zero() - normal,
    }
}

/// A strut-edge spec: `ExtrudedPoint` description, straight-line
/// carrier from the bottom point along the extrusion vector.
fn strut_spec<T: Real>(
    point: Point2<T>,
    place: Affine3<T>,
    q_bottom: Point3<T>,
    w: Vec3<T>,
    w_norm: T,
) -> EdgeCurveSpec<T> {
    EdgeCurveSpec {
        description: EdgeGeometry::MappedCurve(MappedCurve::ExtrudedPoint {
            point,
            place,
            vec: w,
        }),
        carrier: Curve3::Line {
            origin: q_bottom,
            dir: w.normalize(),
        },
        param_start: T::zero(),
        param_end: w_norm,
    }
}

/// Decides whether two consecutive segments' side walls lie on the
/// identical-by-construction surface (crate docs, cosurface sharing):
/// collinear lines share a plane, same-carrier arcs share a cylinder.
/// Mixed kinds never share (structurally); arcs with opposite turns
/// never share (structurally — a same-carrier opposite-turn pair is an
/// overlap the profile validator already refused; checked defensively).
fn cosurface<T: Decide>(
    prev: &SweptSeg<T>,
    next: &SweptSeg<T>,
    band: Band,
) -> Result<bool, Indeterminate> {
    match (&prev.kind, &next.kind) {
        (SweptKind::Line, SweptKind::Line) => {
            // Margin: perpendicular distance of the next chord's far
            // endpoint from the previous chord's carrier line (meters,
            // direct displacement).
            let t = (prev.b - prev.a).normalize();
            let d = next.b - prev.a;
            let margin = t.perp_dot(d);
            Ok(matches!(
                decide("side_planes_cosurface", margin, band)?,
                Sign::Zero
            ))
        }
        (
            SweptKind::Arc {
                center: c1,
                radius: r1,
                turn: t1,
            },
            SweptKind::Arc {
                center: c2,
                radius: r2,
                turn: t2,
            },
        ) => {
            if t1 != t2 {
                return Ok(false);
            }
            // Margin: center distance plus radius difference (meters,
            // direct — the profile crate's carrier-identity pattern).
            let margin = c1.distance(*c2) + (*r1 - *r2).abs();
            Ok(matches!(
                decide("side_cylinders_cosurface", margin, band)?,
                Sign::Zero
            ))
        }
        _ => Ok(false),
    }
}

/// One loop's construction record: the bottom rim half-edges of the
/// seed-face loop (half-edge `j` runs vertex `j` → vertex `j + 1`,
/// carrying segment `j`).
struct LoopBase {
    hes: Vec<topo::HalfEdgeKey>,
}

/// Extrudes a validated profile into a closed solid.
///
/// The sketch placement is the profile's own
/// ([`profile::SketchPlane`]); the extrusion vector or signed distance
/// is classified against the plane normal per the crate docs' direction
/// conventions. On success the returned body passes tiers 1–3
/// (`topo::validate`, `validate_closed`, `validate_geometric`) — the
/// caller re-validates at rest per the workspace convention; tier 1 is
/// debug-asserted after every operator, tier 2 on the finished body.
///
/// # Errors
///
/// [`ExtrudeError`] — closed and typed: degenerate/oblique/sliver
/// extrusion vectors, sliver dihedrals at joins, escalated cosurface
/// decisions, Newell failures, and every operator/certification
/// refusal (D4 ¶2 reports surface inside
/// [`EulerOpError::Certification`]).
pub fn extrude<T: Decide>(
    profile: &ValidatedProfile<T>,
    extrusion: Extrusion<T>,
) -> Result<Extruded<T>, ExtrudeError> {
    let band = Band::linear().map_err(ExtrudeError::Band)?;
    let place = profile.plane().placement;
    let normal = place.linear.c2;

    // ---- Direction classification (crate docs; named predicates). ----
    let (w, reverse) = match extrusion {
        Extrusion::Distance(d) => {
            let sign = decide("extrusion_normal_component", d, band)
                .map_err(|source| ExtrudeError::ExtrusionEscalated { source })?;
            match sign {
                Sign::Zero => return Err(ExtrudeError::DegenerateExtrusion),
                Sign::Positive => (normal * d, false),
                Sign::Negative => (normal * d, true),
            }
        }
        Extrusion::Vector(v) => {
            let nc = v.dot(normal);
            let sign = decide("extrusion_normal_component", nc, band)
                .map_err(|source| ExtrudeError::ExtrusionEscalated { source })?;
            let reverse = match sign {
                Sign::Zero => return Err(ExtrudeError::DegenerateExtrusion),
                Sign::Positive => false,
                Sign::Negative => true,
            };
            // In-plane component: must be coincident with zero (normal
            // extrusion only at M2 — crate docs).
            let in_plane = (v - normal * nc).norm();
            match decide("extrusion_obliquity", in_plane, band)
                .map_err(|source| ExtrudeError::ExtrusionEscalated { source })?
            {
                Sign::Zero => {}
                Sign::Positive | Sign::Negative => return Err(ExtrudeError::ObliqueExtrusion),
            }
            (v, reverse)
        }
    };
    let w_norm = w.norm();
    let top_place = Affine3::translation(w) * place;

    // ---- Swept traversals and world points, per loop. ----
    let loops: Vec<Vec<SweptSeg<T>>> = profile
        .loops()
        .iter()
        .map(|lp| swept_segments(lp, reverse))
        .collect();
    let world = |p: Point2<T>| place.transform_point(Point3::new(p.x, p.y, T::zero()));
    let points: Vec<Vec<Point3<T>>> = loops
        .iter()
        .map(|segs| segs.iter().map(|s| world(s.a)).collect())
        .collect();

    // ---- Phase 1: outer lamina. ----
    let outer = &loops[0];
    let qs = &points[0];
    let n = outer.len();
    let mut body = Body::<T>::new();
    let seed = body.mvfs(qs[0])?;
    let mut hes = Vec::with_capacity(n);
    let first = body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        qs[1 % n],
        rim_spec(&outer[0], place, normal, qs[0], qs[1 % n]),
    )?;
    hes.push(first.he_plus);
    let mut prev = first;
    for j in 2..n {
        let m = body.mev(
            MevSite::Fan {
                he1: prev.he_minus,
                he2: prev.he_minus,
            },
            qs[j],
            rim_spec(&outer[j - 1], place, normal, qs[j - 1], qs[j]),
        )?;
        hes.push(m.he_plus);
        prev = m;
    }
    // Bottom cap plane: the mef-minted face's loop runs the chain
    // reversed — first cap point kept, the rest reversed (outward
    // normal opposite the extrusion). Cap points are the loop vertices
    // plus arc apexes (see `cap_points`).
    let forward = cap_points(outer, qs, place);
    let mut bottom_order: Vec<Point3<T>> = Vec::with_capacity(forward.len());
    if let Some(&p0) = forward.first() {
        bottom_order.push(p0);
    }
    for &p in forward.iter().skip(1).rev() {
        bottom_order.push(p);
    }
    let bottom_plane =
        newell_plane(&bottom_order, band).map_err(|source| ExtrudeError::CapPlane { source })?;
    let close = body.mef(
        MefSite::Chords {
            he1: prev.he_minus,
            he2: first.he_plus,
        },
        rim_spec(&outer[n - 1], place, normal, qs[n - 1], qs[0]),
        FaceSurface::New(bottom_plane),
    )?;
    hes.push(close.he_plus);
    let top_face = seed.face;
    let bottom_face = close.face;
    let bottom_surface = body
        .get_face(bottom_face)
        .ok_or(EulerOpError::StaleKey {
            key: topo::EntityId::Face(bottom_face),
        })?
        .surface;
    let mut bases = Vec::with_capacity(loops.len());
    bases.push(LoopBase { hes });

    // ---- Phase 2: holes (rings in the seed face + kfmrh into the
    // bottom cap). ----
    let anchor = bases[0].hes[0];
    for (li, segs) in loops.iter().enumerate().skip(1) {
        let hq = &points[li];
        let m = segs.len();
        // Plant the hole anchor: bridge strut, immediately killed into
        // an empty ring at the hole's first vertex (§9.3's state).
        let bridge = body.mev_line(
            MevSite::Fan {
                he1: anchor,
                he2: anchor,
            },
            hq[0],
        )?;
        let ring = body.kemr(bridge.he_plus, bridge.he_minus)?.ring;
        // Grow the hole chain inside the ring loop.
        let mut hole_hes = Vec::with_capacity(m);
        let first = body.mev(
            MevSite::Lone { r#loop: ring },
            hq[1 % m],
            rim_spec(&segs[0], place, normal, hq[0], hq[1 % m]),
        )?;
        hole_hes.push(first.he_plus);
        let mut prev = first;
        for j in 2..m {
            let mv = body.mev(
                MevSite::Fan {
                    he1: prev.he_minus,
                    he2: prev.he_minus,
                },
                hq[j],
                rim_spec(&segs[j - 1], place, normal, hq[j - 1], hq[j]),
            )?;
            hole_hes.push(mv.he_plus);
            prev = mv;
        }
        // Close the hole cycle: the ring keeps the forward chain; the
        // new face is the transient disc, on the bottom cap's plane.
        let close = body.mef(
            MefSite::Chords {
                he1: prev.he_minus,
                he2: first.he_plus,
            },
            rim_spec(&segs[m - 1], place, normal, hq[m - 1], hq[0]),
            FaceSurface::Shared(bottom_surface),
        )?;
        hole_hes.push(close.he_plus);
        // Consume the disc: its loop becomes the bottom cap's ring —
        // the same-shell genus supplier.
        body.kfmrh(bottom_face, close.face)?;
        bases.push(LoopBase { hes: hole_hes });
    }

    // ---- Phase 3 + 4: sweep each loop of the seed face; classify
    // joins. ----
    let mut side_faces = Vec::with_capacity(loops.len());
    let mut strut_edges = Vec::with_capacity(loops.len());
    for (li, (segs, base)) in loops.iter().zip(&bases).enumerate() {
        let qs = &points[li];
        let (faces, struts) = sweep_loop(
            &mut body, li, segs, &base.hes, qs, place, top_place, normal, w, w_norm, band,
        )?;
        side_faces.push(faces);
        strut_edges.push(struts);
    }

    // ---- Phase 5: the swept face survives as the top cap; attach its
    // plane (raised outer loop in next order ⇒ outward normal along the
    // extrusion). ----
    let raised: Vec<Point3<T>> = cap_points(&loops[0], &points[0], place)
        .iter()
        .map(|&q| q + w)
        .collect();
    let top_plane =
        newell_plane(&raised, band).map_err(|source| ExtrudeError::CapPlane { source })?;
    body.set_face_surface(top_face, FaceSurface::New(top_plane))?;

    #[cfg(debug_assertions)]
    debug_assert_eq!(
        topo::validate_closed(&body),
        Ok(()),
        "extrude postcondition: result is not tier-2 valid (kernel bug)",
    );

    Ok(Extruded {
        body,
        solid: seed.solid,
        shell: seed.shell,
        top: top_face,
        bottom: bottom_face,
        side_faces,
        strut_edges,
    })
}

/// Sweeps one loop of the seed face: struts, side quads, join
/// classification (module docs, steps 3–4). Returns the side faces and
/// strut edges in swept order.
#[allow(clippy::too_many_arguments)] // one internal call site; the
// arguments are the sweep's fixed context, not a configuration surface.
fn sweep_loop<T: Decide>(
    body: &mut Body<T>,
    loop_index: usize,
    segs: &[SweptSeg<T>],
    hes: &[topo::HalfEdgeKey],
    qs: &[Point3<T>],
    place: Affine3<T>,
    top_place: Affine3<T>,
    normal: Vec3<T>,
    w: Vec3<T>,
    w_norm: T,
    band: Band,
) -> Result<(Vec<FaceKey>, Vec<EdgeKey>), ExtrudeError> {
    let n = segs.len();

    // Struts: one raised vertex per loop vertex, in traversal order.
    let mut struts: Vec<MevCreated> = Vec::with_capacity(n);
    for j in 0..n {
        let m = body.mev(
            MevSite::Fan {
                he1: hes[j],
                he2: hes[j],
            },
            qs[j] + w,
            strut_spec(segs[j].a, place, qs[j], w, w_norm),
        )?;
        struts.push(m);
    }

    // Side quads: the new edge is the top rim (v_j′ → v_{j+1}′), the
    // new face the side wall; the last mef closes against the first top
    // rim (which replaced the consumed strut-minus navigation).
    let mut faces: Vec<FaceKey> = Vec::with_capacity(n);
    let mut first_top = None;
    for j in 0..n {
        let he2 = match (j + 1 < n, first_top) {
            (true, _) => struts[j + 1].he_minus,
            (false, Some(top)) => top,
            // n == 1 is unreachable (validated loops have ≥ 2
            // vertices); fall back to the strut's own minus half so the
            // operator surfaces the malformed site as a typed error
            // rather than this code panicking.
            (false, None) => struts[j].he_minus,
        };
        let surface = side_surface(
            body, loop_index, segs, &faces, j, qs, place, normal, w, band,
        )?;
        let top_q_from = qs[j] + w;
        let top_q_to = qs[(j + 1) % n] + w;
        let mef = body.mef(
            MefSite::Chords {
                he1: struts[j].he_minus,
                he2,
            },
            rim_spec(&segs[j], top_place, normal, top_q_from, top_q_to),
            surface,
        )?;
        if j == 0 {
            first_top = Some(mef.he_plus);
        }
        faces.push(mef.face);
    }

    // Join classification: strut j joins the side walls of segments
    // j − 1 and j. Identical surface keys are conventional joins
    // structurally; distinct surfaces classify by dihedral at the strut
    // midpoint (extent: the strut chord).
    for j in 0..n {
        let f_prev = faces[(j + n - 1) % n];
        let f_next = faces[j];
        let k_prev = face_surface_key(body, f_prev)?;
        let k_next = face_surface_key(body, f_next)?;
        if k_prev == k_next {
            continue;
        }
        let s_prev = *body
            .get_surface(k_prev)
            .ok_or(EulerOpError::StaleGeometry {
                key: topo::GeomRef::Surface(k_prev),
            })?;
        let s_next = *body
            .get_surface(k_next)
            .ok_or(EulerOpError::StaleGeometry {
                key: topo::GeomRef::Surface(k_next),
            })?;
        let mid = qs[j] + w * T::from_f64(0.5);
        match classify_dihedral(&s_prev, &s_next, mid, w_norm, band) {
            Ok(DihedralClass::Transverse) => {
                // The prefer-intrinsic upgrade: mint-time Intersection
                // was impossible (the side surfaces did not exist);
                // re-describe through the certified setter.
                let spec = EdgeCurveSpec {
                    description: EdgeGeometry::Intersection {
                        s1: k_prev,
                        s2: k_next,
                        witness: mid,
                    },
                    carrier: Curve3::Line {
                        origin: qs[j],
                        dir: w.normalize(),
                    },
                    param_start: T::zero(),
                    param_end: w_norm,
                };
                body.set_edge_curve(struts[j].edge, spec)?;
            }
            Ok(DihedralClass::Smooth) => {}
            Err(source) => {
                return Err(ExtrudeError::SliverJoin {
                    loop_index,
                    vertex_index: segs[j].canonical_vertex,
                    source,
                });
            }
        }
    }

    Ok((faces, struts.iter().map(|m| m.edge).collect()))
}

/// The side wall's surface spec for segment `j`: `Shared` with the
/// previous wall (or, on the wrap join, the first) when the cosurface
/// predicates say one carrier; otherwise a freshly built plane
/// (Newell over the quad corners in loop order — outward by the
/// orientation contract) or cylinder (turn-signed axis, crate docs).
#[allow(clippy::too_many_arguments)] // one internal call site (see sweep_loop).
fn side_surface<T: Decide>(
    body: &Body<T>,
    loop_index: usize,
    segs: &[SweptSeg<T>],
    faces: &[FaceKey],
    j: usize,
    qs: &[Point3<T>],
    place: Affine3<T>,
    normal: Vec3<T>,
    w: Vec3<T>,
    band: Band,
) -> Result<FaceSurface<T>, ExtrudeError> {
    let n = segs.len();
    let escalated = |source: Indeterminate, seg: &SweptSeg<T>| ExtrudeError::CosurfaceEscalated {
        loop_index,
        vertex_index: seg.canonical_vertex,
        source,
    };
    if j > 0 {
        if cosurface(&segs[j - 1], &segs[j], band).map_err(|e| escalated(e, &segs[j]))? {
            let key = face_surface_key(body, faces[j - 1])?;
            return Ok(FaceSurface::Shared(key));
        }
        // The wrap join: the last segment may continue the first
        // segment's carrier (join at vertex 0).
        if j == n - 1
            && cosurface(&segs[n - 1], &segs[0], band).map_err(|e| escalated(e, &segs[0]))?
        {
            let key = face_surface_key(body, faces[0])?;
            return Ok(FaceSurface::Shared(key));
        }
    }
    match segs[j].kind {
        SweptKind::Line => {
            // Quad corners in the side loop's next order starting at
            // the raised start vertex: v_j′, v_j, v_{j+1}, v_{j+1}′.
            let corners = [qs[j] + w, qs[j], qs[(j + 1) % n], qs[(j + 1) % n] + w];
            let plane = newell_plane(&corners, band).map_err(|source| ExtrudeError::SidePlane {
                loop_index,
                segment_index: segs[j].canonical_segment,
                source,
            })?;
            Ok(FaceSurface::New(plane))
        }
        SweptKind::Arc {
            center,
            radius,
            turn,
        } => {
            // The carrier axis line is the arc's center extruded:
            // `place · (center, 0)` — the same computation the rim
            // carrier makes, so cylinder and rim circle agree
            // bit-identically.
            let c_world = place.transform_point(Point3::new(center.x, center.y, T::zero()));
            Ok(FaceSurface::New(Surface::Cylinder {
                origin: c_world,
                axis: turn_axis(turn, normal),
                radius,
                u_ref: (qs[j] - c_world).normalize(),
            }))
        }
    }
}

/// Resolves a face's surface key (total: stale keys surface as the
/// operator-layer typed error).
fn face_surface_key<T: Real>(body: &Body<T>, face: FaceKey) -> Result<SurfaceKey, ExtrudeError> {
    Ok(body
        .get_face(face)
        .ok_or(EulerOpError::StaleKey {
            key: topo::EntityId::Face(face),
        })?
        .surface)
}
