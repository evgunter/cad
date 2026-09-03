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
/// triangle set is a closed 2-manifold for closed input bodies.
/// [`crate::validate::check_mesh`] re-derives that over a `Mesh` and
/// is what would catch a violation — but **[`fn@crate::tessellate`]
/// does not run it**, so a consumer that needs the contract checked
/// rather than argued has to call it.
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
    /// box, by more than the band, on a face whose domain the SHAPE
    /// door ([`Self::UnsupportedCurvedShape`]) has already certified
    /// as an iso-rectangle.
    ///
    /// **This is the WALK-CONSISTENCY question, and only that.** The
    /// shape question is asked first, on rim structure, through
    /// `geom_brep::props::require_iso_rectangle`; what is left for this
    /// arm is a polygon that should be a rectangle and is not. The grid
    /// runs the open ranges `1..nu` × `1..nv` over the walk's own
    /// bounding box, which is strictly interior **iff the polygon IS
    /// that box**; on one that is not, the grid instead splits boundary
    /// constraints and `inner_faces()` emits triangles outside the face
    /// — a silently wrong mesh (a 3-D T-junction plus ghost geometry),
    /// which is what this refusal replaces. It is the `curved`-chart
    /// twin of [`Self::SelfTouchingTrimLoop`].
    ///
    /// **Two things can still trip it, and the payload separates
    /// them.** A walk that failed to keep an iso side straight lands a
    /// hair inside its box — sub-ε is absorbed by the band, and above
    /// it [`Self::UnsupportedCurvedDomain::max_distance`] near ε is a
    /// kernel bug report. An iso-bounded loop the rim predicate cannot
    /// see lands a FEATURE width inside it: a zero-width slit (two
    /// meridians up and down one column to an interior level) has
    /// every rim at an extreme and passes the shape door, and its tip
    /// is a walk entry strictly inside the box. That is valid input in
    /// a lane not built for it (D2 addendum row 2), and re-authoring
    /// the face is the recourse.
    ///
    /// **The band, and why it stays.** Bodies whose walk landed
    /// microscopically off their own UV box passed every upstream gate
    /// and reached this lane every day — a swept body plus
    /// `topo::Body::split_edge`, or a STEP file stating one face
    /// boundary as two collinear `EDGE_CURVE`s, either one placed
    /// obliquely. An iso side carried by several edges was only
    /// *analytically* straight, so on those an EXACT comparison refused
    /// valid parts — measured false refusals at 1e-17 m (issue #653).
    /// The comparison is therefore BANDED, in metres, against the run's
    /// ε. Issue 653's option 2 then fixed the wobble at its source
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
    /// A curved face's domain is not an iso-parameter rectangle, by
    /// props' named shape predicate — `geom_brep::props::
    /// require_iso_rectangle`, the S58 single home of `props_rim_level`
    /// and of the per-kind rim/meridian classification under it. The
    /// `source` is props' own refusal, carrying the `what` the flux
    /// lane would report for the same face.
    ///
    /// **This is the SHAPE question**, asked on the face's rim structure
    /// BEFORE the boundary walk runs. The swept-rectangle lane cites the
    /// predicate itself (issue 727): the line that changes when the
    /// certified-quadrature lane learns notched domains is
    /// `curved::require_iso_rectangle_face`'s call in this crate, not a
    /// floor in the boolean or in tier 3 that could vanish without a
    /// line of `mesh` moving. [`Self::UnsupportedCurvedDomain`] is the
    /// other question — did the walk trace the rectangle the door
    /// certified — and is asked after.
    ///
    /// Valid input, unbuilt lane (D2 addendum row 2): a keyway or a
    /// milled flat on a cylinder is iso-bounded and not a rectangle,
    /// and the lane that would mesh it is the one that would measure
    /// it, the certified-quadrature lane. An `Escalated` source is the
    /// same refusal one band-width away (D4: escalate, never guess).
    ///
    /// Naming: this arm is the SHAPE question and
    /// [`Self::UnsupportedCurvedDomain`] the WALK question, though
    /// "domain" would read as the shape. The older arm keeps its name
    /// because it is public API with a stable python tag
    /// (`unsupported_curved_domain`) and pinned rows in three crates;
    /// renaming it would move all of that for a word. The two docs
    /// carry the distinction instead.
    UnsupportedCurvedShape {
        /// The offending face.
        face: FaceKey,
        /// props' refusal: which structural expectation failed.
        source: geom_brep::props::PropsError,
    },
    /// The run's tolerance cannot form props' linear decision band —
    /// K·ε overflows. A configuration failure of the run rather than a
    /// statement about the body (the twin of
    /// `topo::MassPropsError::Band`), surfaced typed because the shape
    /// door decides against that band.
    ///
    /// Reachable only from the environment: ε within a factor K of
    /// `f64::MAX` (`CAD_TOLERANCE_EPS`), and then on EVERY body, the
    /// empty one included, because the band is minted once at
    /// operation entry (its documented calling convention) rather
    /// than on the first curved face. Minting lazily would let an
    /// all-planar body mesh under an ε no predicate could decide with,
    /// and make the refusal depend on face order; a run configured
    /// past `f64` is refused before it meshes anything, uniformly.
    /// Kept as a typed arm because `Band::linear` is fallible by
    /// contract and `Tol` promises only a finite positive ε.
    Band {
        /// The band construction failure.
        error: geom_core::BandError,
    },
}

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM in tessellation's own vocabulary — δ, chart, walk, trim
// loop, at-rest body — plus the recourse where a caller has one. Arena
// keys are topo-private handles that mean nothing to a person, so an
// arm names the entity by KIND ("a face", "an edge") and spends its
// words on the payload a caller can act on: the note naming the
// unbuilt lane, the counts, the distances. The arms whose own doc says
// no at-rest construction mints them state that they are kernel bugs
// rather than offering a recourse that does not exist.
impl core::fmt::Display for TessellateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidChordalTolerance { value } => write!(
                f,
                "tessellate: chordal tolerance δ = {value:e} is not a finite, \
                 strictly positive length — δ is refused rather than clamped; \
                 ask for a real sag budget",
            ),
            Self::UnsupportedSurface { .. } => f.write_str(
                "tessellate: a face's NURBS surface is still the mvfs \
                 placeholder — there is no description to tessellate against; \
                 give the face its surface before meshing the body",
            ),
            Self::UnsupportedNurbsFace { note, .. } => write!(
                f,
                "tessellate: a NURBS face is outside the trimmed-NURBS lane's \
                 certified inventory — {note}",
            ),
            Self::UnsupportedCurve { note, .. } => write!(
                f,
                "tessellate: an edge's carrier is outside the certified \
                 inventory — {note}",
            ),
            Self::NullScaffoldEdge { .. } => f.write_str(
                "tessellate: an edge is null-edge scaffolding and has no \
                 carrier by type — the body is mid-surgery; tessellation is \
                 defined on at-rest bodies, so finish the surgery first",
            ),
            Self::RingOnCurvedFace { .. } => f.write_str(
                "tessellate: a curved face carries an interior ring — curved \
                 patches are swept UV rectangles and no construction produces \
                 one, so this is a kernel bug rather than a mesh to guess at",
            ),
            Self::EmptyLoop { .. } => f.write_str(
                "tessellate: a face has an empty loop — that is a tier-1 \
                 scaffolding state; tessellation is defined on at-rest bodies, \
                 so finish the surgery first",
            ),
            Self::MissingEntity { what } => write!(
                f,
                "tessellate: a key the body's own topology references failed to \
                 resolve ({what}) — a tier-valid body cannot reach this, so the \
                 input is corrupt",
            ),
            Self::ResolutionOverflow { count } => write!(
                f,
                "tessellate: δ is so small that one edge or face would need \
                 {count:e} chords or grid divisions, past the ~2²⁴ sanity cap — \
                 refused before allocating; ask for a coarser δ",
            ),
            Self::CertificateExceeded {
                bound, requested, ..
            } => write!(
                f,
                "tessellate: a face's worst triangle certifies to only {bound:e} \
                 m of deviation against the requested δ = {requested:e} m — the \
                 conservative promise could not be established, and an \
                 uncertified mesh is never shipped; this is a kernel-side defect \
                 or degenerate geometry",
            ),
            Self::Triangulation { .. } => f.write_str(
                "tessellate: the CDT rejected a point insertion on a face — a \
                 non-finite or out-of-range chart coordinate, i.e. corrupt \
                 geometry",
            ),
            Self::SelfTouchingTrimLoop { .. } => f.write_str(
                "tessellate: a trimmed face's boundary passes exactly through \
                 another chord point of the same loop, so the neighbouring \
                 faces would disagree about that vertex — a 3-D T-junction no \
                 grid retry repairs. No at-rest construction mints a \
                 self-touching trim loop, so this is a kernel bug",
            ),
            Self::UnsupportedCurvedDomain {
                off_bbox,
                first_uv: (u, v),
                max_distance,
                ..
            } => write!(
                f,
                "tessellate: a curved face's boundary walk does not trace its \
                 own UV rectangle — {off_bbox} walk entries lie strictly \
                 inside it, the first at chart (u = {u:e}, v = {v:e}), by up to \
                 {max_distance:e} m. The interior grid assumes the swept \
                 rectangle, so a polygon that is not one is refused rather \
                 than meshed with ghost triangles. The face's rim structure \
                 passed props' iso-rectangle door, so a feature-sized distance \
                 means an iso-bounded loop that predicate cannot see (a \
                 zero-width slit) — re-author it — while a distance near ε is \
                 a kernel bug report",
            ),
            Self::UnsupportedCurvedShape { source, .. } => write!(
                f,
                "tessellate: a curved face's domain is not an iso-parameter \
                 rectangle by props' shape predicate ({source}) — the \
                 swept-rectangle lane meshes chart rectangles only, and a \
                 notched or L-shaped iso domain waits for the certified-\
                 quadrature lane; split the face into rectangles or re-author \
                 it",
            ),
            Self::Band { error } => write!(
                f,
                "tessellate: the run's tolerance cannot form the decision band \
                 props classifies with ({error}) — a configuration failure of \
                 the run, not a statement about the body",
            ),
        }
    }
}

impl core::error::Error for TessellateError {}
