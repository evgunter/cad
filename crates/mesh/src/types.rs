//! The [`Mesh`] value and the typed tessellation error.

use geom_core::Point3;
use topo::{EdgeKey, FaceKey, VertexKey};

/// A tessellated body: one shared position buffer, per-face triangle
/// patches, and per-edge boundary polylines.
///
/// Per-face patches stay individually addressable (ratified patch
/// separability — future content-keyed reuse transfers unchanged
/// faces' patches across rebuilds); back-references complete the
/// picking chain triangle→face, segment→edge, endpoint→vertex.
///
/// Watertightness contract: patches of adjacent faces share the same
/// position indices along their common boundary polylines, so the
/// triangle set is a closed 2-manifold for closed input bodies
/// ([`crate::validate::check_mesh`] verifies).
///
/// No `PartialEq`: positions are floats; D9 comparisons are bitwise
/// (compare `f64::to_bits` of positions plus the index/key structure —
/// the determinism suite does exactly that).
#[derive(Clone, Debug)]
pub struct Mesh {
    /// Mesh vertex positions, shared by all patches. Minted in fixed
    /// order: topology vertices (arena order), then per-edge interior
    /// chord points (arena order, `he_plus` direction), then per-face
    /// interior grid points (arena order, row-major).
    pub positions: Vec<Point3<f64>>,
    /// One patch per face, in face-arena iteration order.
    pub patches: Vec<FacePatch>,
    /// One polyline per edge, in edge-arena iteration order.
    pub boundaries: Vec<BoundaryPolyline>,
}

/// One face's triangles (indices into [`Mesh::positions`]).
#[derive(Clone, Debug, PartialEq)]
pub struct FacePatch {
    /// The source face (picking back-reference).
    pub face: FaceKey,
    /// Triangles with **outward** winding (counterclockwise viewed
    /// from outside the material, per the D1 loop conventions).
    ///
    /// "Outward" means the *material* side, not the chart-normal side.
    /// A face's outward normal is
    /// `topo::Face::sense_sign() · chart_normal`, and this contract is
    /// stated in the outward frame: on a face with `sense: false` the
    /// emitted triangles wind CCW about `−chart_normal`. The
    /// tessellator reaches that without consulting the bit on this
    /// path — the winding comes from the loop's stored traversal,
    /// which interior-left already ties to the outward normal (see
    /// `planar`/`curved`) — so the guarantee holds for either sense
    /// with no per-consumer correction. Downstream consumers (STL
    /// facet normals, signed volumes) may therefore keep deriving
    /// orientation from this winding alone, and must NOT re-apply the
    /// sense on top of it.
    pub triangles: Vec<[u32; 3]>,
}

/// One edge's chord polyline (indices into [`Mesh::positions`]).
///
/// Runs in the edge's intrinsic (`he_plus`-forward) direction; each
/// consecutive index pair is one mesh boundary segment whose source is
/// [`BoundaryPolyline::edge`]. The first/last indices are the points of
/// [`BoundaryPolyline::start_vertex`] / [`BoundaryPolyline::end_vertex`]
/// bitwise (a full-period self-loop edge repeats its single vertex at
/// both ends).
#[derive(Clone, Debug, PartialEq)]
pub struct BoundaryPolyline {
    /// The source edge (picking back-reference for every segment).
    pub edge: EdgeKey,
    /// Chord point indices, `he_plus`-forward, length ≥ 2.
    pub points: Vec<u32>,
    /// Source vertex of the first point (picking back-reference).
    pub start_vertex: VertexKey,
    /// Source vertex of the last point (picking back-reference).
    pub end_vertex: VertexKey,
}

/// Typed failure of [`fn@crate::tessellate`] (closed enum, D4 ¶3).
#[derive(Clone, Debug, PartialEq)]
pub enum TessellateError {
    /// δ is not a finite, strictly positive number (zero, negative,
    /// NaN, or infinite chordal tolerances are refused, never clamped).
    InvalidChordalTolerance {
        /// The offending value.
        value: f64,
    },
    /// A face's surface is the [`geom::Surface::Nurbs`]
    /// **placeholder** (`NurbsSurface::is_placeholder`) — the mvfs "no
    /// description yet" state has no evaluable description to
    /// tessellate against.
    ///
    /// A DESCRIBED `Nurbs` face never lands here: it routes through
    /// `trimmed` with the control-net Hessian certificate, and one
    /// outside the certified inventory refuses
    /// [`Self::UnsupportedNurbsFace`] instead, naming its class.
    UnsupportedSurface {
        /// The offending face.
        face: FaceKey,
    },
    /// A DESCRIBED NURBS face outside the trimmed-NURBS lane's
    /// certified inventory: an ILLEGAL rational description (a
    /// non-positive/non-finite weight voids the convex-combination
    /// licence — legal rational faces certify through the
    /// quotient-rule arm of `nurbs_cert`), a C⁰-creased direction
    /// (interior knot multiplicity ≥ degree — the interpolation
    /// Taylor bound needs C¹), or a degenerate degree-0 direction.
    /// Partial coverage stated typed beats a dishonest bound (D4).
    UnsupportedNurbsFace {
        /// The offending face.
        face: FaceKey,
        /// WHICH class refused and the real blocker.
        note: &'static str,
    },
    /// An edge/carrier configuration outside the certified inventory:
    /// an illegal-rational (non-positive weight) or C⁰-kinked
    /// B-spline carrier (legal rational carriers meter through the
    /// quotient-rule sagitta arm of `chords`), a trimmed face on a
    /// chart whose TRIMMED-TESSELLATION lane is not written
    /// (cone/sphere/torus — those charts mint stored pcurves, but the
    /// written trimmed lane's geometry is the cylinder chart's), or a
    /// trimmed face missing its stored pcurve caches. The
    /// conic-on-cylinder case is a CONSTRUCTION lane (`trimmed`), not
    /// a refusal.
    UnsupportedCurve {
        /// The offending edge.
        edge: EdgeKey,
        /// WHICH tessellation lane the carrier would need and why
        /// this one is not served — human-readable, and
        /// runtime-visible through `Debug`.
        note: &'static str,
    },
    /// An edge is null-edge scaffolding (`topo::null` — no carrier
    /// by type): the body is mid-surgery; tier 2 refuses null entities
    /// at rest, and tessellation is defined on at-rest bodies.
    NullScaffoldEdge {
        /// The scaffolding edge.
        edge: EdgeKey,
    },
    /// A curved face carries interior rings — no construction
    /// produces one (curved patches are swept UV rectangles); refused
    /// rather than guessed at.
    RingOnCurvedFace {
        /// The offending face.
        face: FaceKey,
    },
    /// A face has an empty loop (a tier-1 scaffolding state; closed
    /// tier-2 bodies never carry one at rest).
    EmptyLoop {
        /// The offending face.
        face: FaceKey,
    },
    /// A key referenced by the body's own topology failed to resolve —
    /// corrupt input (unreachable for tier-valid bodies), surfaced
    /// rather than trusted.
    MissingEntity {
        /// A human-readable description of the dangling reference.
        what: &'static str,
    },
    /// A requested resolution overflowed the sanity cap (δ so small
    /// that a single edge or face would need a chord or grid-division
    /// count above ~2²⁴) — refused before allocating.
    ResolutionOverflow {
        /// The computed count that overflowed.
        count: f64,
    },
    /// An emitted triangle failed its closed-form deviation
    /// certificate against δ — the certified-conservative promise
    /// could not be established (kernel-side defect or degenerate
    /// geometry; fail loud, never ship an uncertified mesh).
    CertificateExceeded {
        /// The face whose patch failed.
        face: FaceKey,
        /// The closed-form deviation bound of the worst triangle.
        bound: f64,
        /// The requested chordal tolerance δ.
        requested: f64,
    },
    /// The CDT rejected a point insertion (non-finite or out-of-range
    /// UV coordinate — corrupt geometry surfaced as a typed error).
    Triangulation {
        /// The face being triangulated.
        face: FaceKey,
    },
    /// A trimmed face's boundary polyline passes EXACTLY through
    /// another boundary chord point of the same loop (a self-touching
    /// trim loop): the CDT would realise one face's constraint through
    /// a vertex its neighbour does not share — a 3-D T-junction no
    /// grid-retry can repair. No at-rest construction mints one (split
    /// sections and boolean seams are simple loops); the arm is the
    /// watertightness backstop's tripwire, kept typed rather than
    /// silent.
    SelfTouchingTrimLoop {
        /// The face whose trim loop touches itself.
        face: FaceKey,
    },
    /// A curved face's boundary walk does not trace its own UV
    /// bounding rectangle: some walk entry lies strictly inside the
    /// box, so the domain is notched / L-shaped rather than the swept
    /// UV rectangle the `curved` lane's interior grid assumes.
    ///
    /// Valid input, unbuilt lane (D2 addendum row 2), NOT corruption.
    /// The grid runs the open ranges `1..nu` × `1..nv` over the walk's
    /// own bounding box, which is strictly interior **iff the polygon
    /// IS that box**; on a notched domain the grid instead splits
    /// boundary constraints and `inner_faces()` emits triangles
    /// outside the face — a silently wrong mesh (a 3-D T-junction plus
    /// ghost geometry), which is what this refusal replaces. It is the
    /// `curved`-chart twin of [`Self::SelfTouchingTrimLoop`].
    ///
    /// No at-rest construction in tree mints a genuinely NOTCHED such
    /// domain today: the boolean refuses `CurvedPierceUnsupported` and
    /// `import_step`'s tier-3 gate refuses `NotIsoRectangle` (S28).
    /// Both of those are *other modules'* limits; this arm is the
    /// check at the site that makes the assumption.
    ///
    /// **Those gates are not why the arm stays quiet, and an earlier
    /// form of this doc claimed otherwise.** Bodies whose walk landed
    /// microscopically off their own UV box passed both gates freely
    /// and reached this lane every day — a swept body plus
    /// `topo::Body::split_edge`, or a STEP file stating one face
    /// boundary as two collinear `EDGE_CURVE`s (what an exporter emits
    /// whenever a vertex lands on that edge), either one placed
    /// obliquely. An iso side carried by several edges was only
    /// *analytically* straight, so on those an EXACT comparison
    /// refused valid parts — measured false refusals at 1e-17 m
    /// (issue #653). The comparison is therefore BANDED, in metres,
    /// against the run's ε, and
    /// [`Self::UnsupportedCurvedDomain::max_distance`] is what says
    /// which side of that band a refusal came from.
    ///
    /// **#653's option 2 then fixed the wobble at its source**
    /// (`walk::iso_side_starts`: one coordinate per iso side, not per
    /// edge), so every walk this build produces sits on its box
    /// bitwise and the band no longer separates anything in tree. It
    /// is kept as a backstop — the argument for keeping it, and the
    /// synthetic row that witnesses it, are at
    /// `curved::entries_off_bbox`.
    UnsupportedCurvedDomain {
        /// The offending face.
        face: FaceKey,
        /// How many walk entries lie strictly inside the UV bounding
        /// box, by more than the band (≥ 1 when this fires).
        off_bbox: usize,
        /// The chart `(u, v)` of the first such entry — where to look.
        first_uv: (f64, f64),
        /// The largest distance FROM THE BOX, in metres, over those
        /// entries: the chart's own lever arms applied to the UV gap.
        ///
        /// This is the number that makes the refusal actionable. A
        /// feature-sized value (a keyway notch, a milled flat) means
        /// re-author the part or wait for the lane; a value near ε
        /// means the kernel handed this lane a domain it should have
        /// kept rectangular, and is a bug report.
        max_distance: f64,
    },
}
