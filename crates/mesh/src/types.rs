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
    /// Since M5 S10 a face's outward normal is
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

/// Typed failure of [`crate::tessellate`] (closed enum, D4 ¶3).
#[derive(Clone, Debug, PartialEq)]
pub enum TessellateError {
    /// δ is not a finite, strictly positive number (zero, negative,
    /// NaN, or infinite chordal tolerances are refused, never clamped).
    InvalidChordalTolerance {
        /// The offending value.
        value: f64,
    },
    /// A face's surface is [`geom_surfaces::Surface::Nurbs`] — the
    /// unimplemented placeholder has no evaluable description to
    /// tessellate against (refused, D3 fallback lands at M5).
    UnsupportedSurface {
        /// The offending face.
        face: FaceKey,
    },
    /// An edge/carrier configuration outside the certified inventory:
    /// a rational or C⁰-kinked B-spline carrier (no hull sagitta), a
    /// trimmed face on a chart whose TRIMMED-TESSELLATION lane is not
    /// written (cone/sphere/torus — their pcurves mint since M6-3;
    /// the lane's geometry is still the cylinder chart's), or a
    /// trimmed face missing its stored pcurve caches. Since M5 PR 11 the conic-on-cylinder case is a
    /// CONSTRUCTION lane (`trimmed`), not a refusal.
    UnsupportedCurve {
        /// The offending edge.
        edge: EdgeKey,
        /// The frontier note — WHICH lane is missing and the PR that
        /// lands it (runtime-visible through Debug; review m2).
        note: &'static str,
    },
    /// An edge is M3 null-edge scaffolding (`topo::null` — no carrier
    /// by type): the body is mid-surgery; tier 2 refuses null entities
    /// at rest, and tessellation is defined on at-rest bodies.
    NullScaffoldEdge {
        /// The scaffolding edge.
        edge: EdgeKey,
    },
    /// A curved face carries interior rings — no M2 construction
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
    /// that a single edge or face would need more than ~2²⁴ chords or
    /// grid steps) — refused before allocating.
    ResolutionOverflow {
        /// The computed step count that overflowed.
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
    /// watertightness backstop's tripwire (M5 PR 11 review MIN-1),
    /// kept typed rather than silent.
    SelfTouchingTrimLoop {
        /// The face whose trim loop touches itself.
        face: FaceKey,
    },
}
