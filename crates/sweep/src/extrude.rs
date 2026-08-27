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
//!    is the side wall, plane or cylinder — a CONCAVE arc's wall is
//!    attached `sense: false`, M5 S11: its material lies outside the
//!    carrier, against the chart normal), the last `mef` closing
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
//! 6. **Rim upgrades.** With both cap planes in place, every cap–wall
//!    rim edge (bottom and top, outer and ring loops) upgrades to
//!    `Intersection { cap plane, side surface, witness }` through the
//!    same `classify_dihedral` → `set_edge_curve` pattern as the corner
//!    joins (the ratified rim decision — M2-LOG, 2026-07-19). The
//!    witness is the **carrier's mid-parameter point** (the S2 witness
//!    contract: for arc rims the chord midpoint lies off the carrier by
//!    the sagitta, so `carrier(mid)` is the only honest mint); the
//!    certified carrier and interval are kept verbatim. Every rim of a
//!    normal extrusion is definitely transverse (cap plane ⊥ wall), so
//!    Transverse ⇒ upgrade is the only reachable arm; Smooth keeps the
//!    conventional description (kept total per the D2 split, believed
//!    unreachable here); Indeterminate is the typed
//!    [`ExtrudeError::SliverRim`].
//!
//! Everything runs in a fixed, documented order (D9): loops outer
//! first then holes in canonical order; per loop, struts in traversal
//! order, then side faces, then join classification; finally rim
//! upgrades per loop, per segment, bottom before top. Two calls with
//! identical inputs replay byte-identically.

use core::fmt;

use geom::Curve3;
use geom::Surface;
use geom_brep::{DihedralClass, EdgeCurveSpec, EdgeDescriptionSpec, MappedCurve, NewellError, classify_dihedral, newell_plane};
use geom_core::{
    Affine3, Band, BandError, Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, Tol, Vec3,
};
use profile::{SegmentKind, ValidatedLoop, ValidatedProfile};
use topo::{
    Body, EdgeKey, EulerOpError, FaceKey, FaceSurface, MefSite, MevCreated, MevSite, ShellKey,
    SolidKey, SurfaceKey,
};

use crate::swept;
use crate::swept::{
    CosurfaceNames, SweptChord, SweptKind, cap_points, cosurface, decide, face_surface_key,
    placed_segment_spec, turn_axis,
};

/// The predicate names this verb's cosurface decision reports under
/// (the side walls of an extrusion: planes from lines, cylinders from
/// arcs).
const SIDE_COSURFACE: CosurfaceNames = CosurfaceNames {
    lines: "side_planes_cosurface",
    arcs: "side_cylinders_cosurface",
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
    ///
    /// Defense-in-depth (the `CapPlane` posture): unreachable from
    /// validated profiles, because profile simplicity classifies the
    /// same displacements first — an in-band carrier near-identity
    /// already escalated at the profile gate. Surfaced rather than
    /// trusted.
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
    /// The dihedral classification at a cap–wall rim edge escalated
    /// during the rim upgrade pass (module docs, step 6) — the rim's
    /// counterpart of [`ExtrudeError::SliverJoin`].
    ///
    /// Defense-in-depth (the `CapPlane` posture): a normal extrusion's
    /// rims meet their walls at a right angle through definite lever
    /// arms, so an escalation here means the inputs already carried
    /// something a validated profile cannot — surfaced rather than
    /// trusted.
    SliverRim {
        /// Canonical index of the loop.
        loop_index: usize,
        /// Canonical index of the rim's segment.
        segment_index: usize,
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
    ///
    /// Defense-in-depth (the `CapPlane` posture): a validated segment's
    /// swept quad is non-degenerate whenever the extrusion itself is
    /// (already classified) — unreachable from validated profiles,
    /// surfaced rather than trusted.
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
            Self::DegenerateExtrusion => write!(
                f,
                "extrusion vector/distance has no definite normal component: in-plane or \
                 sliver-thin extrusion — {} (D4)",
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::ObliqueExtrusion => f.write_str(
                "extrusion vector has a definite in-plane component: oblique extrusion is \
                 not supported (arc segments would sweep elliptic cylinders)",
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
            Self::SliverRim {
                loop_index,
                segment_index,
                source,
            } => write!(
                f,
                "sliver dihedral at loop {loop_index} segment {segment_index}'s cap-wall rim: \
                 the rim is neither a definite corner nor definitely smooth: {source}"
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

/// A swept segment plus the wall face it sweeps.
///
/// The traversal itself is [`crate::swept::SweptSeg`], built by
/// [`crate::swept::swept_segments`] like every other verb's; this
/// record wraps it to carry the one thing extrude decides that a
/// traversal does not. Shared bodies see only the chord, through
/// [`SweptChord`] below — which is why the bit can live here without
/// reaching any of them.
#[derive(Clone, Copy, Debug)]
pub(crate) struct WallSeg<T: Real> {
    /// The swept traversal record: endpoints, bulge, carrier class and
    /// canonical indices, all in swept traversal order.
    pub(crate) chord: swept::SweptSeg<T>,
    /// The wall face's orientation sense (M5 S11): `false` iff the
    /// segment's wall has its material AGAINST the cylinder's chart
    /// normal — exactly the concave arcs. Exact stored structure, not
    /// a numeric decision: the profile's canonical winding is
    /// material-left (outers counterclockwise, holes clockwise), and
    /// an arc's center lies left of its chord traversal iff its turn
    /// is `Positive` — so material sits inside the carrier (chart
    /// normal outward = away from material, sense `true`) iff the
    /// **canonical** turn is `Positive`, and outside it (concave —
    /// chart normal into material, sense `false`) iff `Negative`.
    /// Concavity is a property of the 2-D region against the carrier
    /// circle alone, so the extrusion direction (the swept reversal)
    /// never enters. Line walls are Newell-outward by construction:
    /// always `true`.
    pub(crate) wall_sense: bool,
}

/// Builds the swept traversal of one canonical loop and attaches each
/// segment's wall sense.
///
/// The traversal is [`crate::swept::swept_segments`], shared with every
/// verb that sweeps a validated loop. All this adds is
/// [`wall_sense_of`], read off the CANONICAL segment the traversal
/// records, so the bit is the same whichever direction the loop is
/// swept.
pub(crate) fn wall_segments<T: Real>(lp: &ValidatedLoop<T>, reverse: bool) -> Vec<WallSeg<T>> {
    let canonical = lp.segments();
    swept::swept_segments(lp, reverse)
        .into_iter()
        .map(|chord| WallSeg {
            wall_sense: wall_sense_of(&canonical[chord.canonical_segment]),
            chord,
        })
        .collect()
}

/// The wall sense of one CANONICAL segment ([`WallSeg::wall_sense`]):
/// `false` iff it is a concave arc. An extrude wall's chart normal is
/// the outward radial of its carrier cylinder, so the face's sense is
/// `true` exactly when the material is on the far side of the carrier
/// from the chord — which is [`swept::centre_on_material_side`], the
/// same rule and the same body revolve's arc walls use. Line walls are
/// Newell-outward by construction.
fn wall_sense_of<T: Real>(s: &profile::ValidatedSegment<T>) -> bool {
    match s.kind {
        SegmentKind::Line => true,
        SegmentKind::Arc { turn, .. } => swept::centre_on_material_side(turn),
    }
}

impl<T: Real> SweptChord<T> for WallSeg<T> {
    fn a(&self) -> Point2<T> {
        self.chord.a
    }
    fn b(&self) -> Point2<T> {
        self.chord.b
    }
    fn bulge(&self) -> T {
        self.chord.bulge
    }
    fn kind(&self) -> SweptKind<T> {
        self.chord.kind
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
        description: EdgeDescriptionSpec::Scaffold(MappedCurve::ExtrudedPoint {
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
    tol: Tol,
) -> Result<Extruded<T>, ExtrudeError> {
    let band = Band::linear(tol).map_err(ExtrudeError::Band)?;
    let place = profile.plane().placement;
    let normal = place.linear.c2;

    // ---- Direction classification (crate docs; named predicates). ----
    let (w, reverse) = match extrusion {
        Extrusion::Distance(d) => {
            let sign = decide("extrusion_normal_component", Margin::of(d), band)
                .map_err(|source| ExtrudeError::ExtrusionEscalated { source })?;
            match sign {
                Sign::Zero => return Err(ExtrudeError::DegenerateExtrusion),
                Sign::Positive => (normal * d, false),
                Sign::Negative => (normal * d, true),
            }
        }
        Extrusion::Vector(v) => {
            let nc = v.dot(normal);
            let sign = decide("extrusion_normal_component", Margin::of(nc), band)
                .map_err(|source| ExtrudeError::ExtrusionEscalated { source })?;
            let reverse = match sign {
                Sign::Zero => return Err(ExtrudeError::DegenerateExtrusion),
                Sign::Positive => false,
                Sign::Negative => true,
            };
            // In-plane component: must be coincident with zero (normal
            // extrusion only at M2 — crate docs).
            let in_plane = Margin::norm3(v - normal * nc);
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
    let loops: Vec<Vec<WallSeg<T>>> = profile
        .loops()
        .iter()
        .map(|lp| wall_segments(lp, reverse))
        .collect();
    let world = |p: Point2<T>| place.transform_point(Point3::new(p.x, p.y, T::zero()));
    let points: Vec<Vec<Point3<T>>> = loops
        .iter()
        .map(|segs| segs.iter().map(|s| world(s.chord.a)).collect())
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
        placed_segment_spec(&outer[0], place, normal, qs[0], qs[1 % n]),
        tol,
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
            placed_segment_spec(&outer[j - 1], place, normal, qs[j - 1], qs[j]),
            tol,
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
        placed_segment_spec(&outer[n - 1], place, normal, qs[n - 1], qs[0]),
        FaceSurface::New(bottom_plane),
        tol,
    )?;
    hes.push(close.he_plus);
    let top_face = seed.face;
    let bottom_face = close.face;
    let bottom_surface = face_surface_key(&body, bottom_face)?;
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
            tol,
        )?;
        let ring = body.kemr(bridge.he_plus, bridge.he_minus)?.ring;
        // Grow the hole chain inside the ring loop.
        let mut hole_hes = Vec::with_capacity(m);
        let first = body.mev(
            MevSite::Lone { r#loop: ring },
            hq[1 % m],
            placed_segment_spec(&segs[0], place, normal, hq[0], hq[1 % m]),
            tol,
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
                placed_segment_spec(&segs[j - 1], place, normal, hq[j - 1], hq[j]),
                tol,
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
            placed_segment_spec(&segs[m - 1], place, normal, hq[m - 1], hq[0]),
            FaceSurface::Shared(bottom_surface),
            tol,
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
    let mut top_rims = Vec::with_capacity(loops.len());
    for (li, (segs, base)) in loops.iter().zip(&bases).enumerate() {
        let qs = &points[li];
        let swept = sweep_loop(
            &mut body, li, segs, &base.hes, qs, place, top_place, normal, w, w_norm, band, tol,
        )?;
        side_faces.push(swept.faces);
        strut_edges.push(swept.struts);
        top_rims.push(swept.top_rims);
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
    let top_surface = body.set_face_surface(top_face, FaceSurface::New(top_plane))?;

    // ---- Phase 6: rim upgrades (module docs — the ratified rim
    // decision). Both cap planes now exist, so every cap–wall rim edge
    // re-describes as their `Intersection` through the certified
    // setter: loops in canonical order, segments in swept order, the
    // bottom rim before the top (D9 — fixed order). ----
    for (li, (segs, base)) in loops.iter().zip(&bases).enumerate() {
        let qs = &points[li];
        let n = segs.len();
        for j in 0..n {
            let wall = face_surface_key(&body, side_faces[li][j])?;
            let q_from = qs[j];
            let q_to = qs[(j + 1) % n];
            let bottom_rim = body
                .get_half_edge(base.hes[j])
                .ok_or(EulerOpError::StaleKey {
                    key: topo::EntityId::HalfEdge(base.hes[j]),
                })?
                .edge;
            upgrade_rim(
                &mut body,
                bottom_rim,
                bottom_surface,
                wall,
                q_from,
                q_to,
                li,
                segs[j].chord.canonical_segment,
                band,
                tol,
            )?;
            upgrade_rim(
                &mut body,
                top_rims[li][j],
                top_surface,
                wall,
                q_from + w,
                q_to + w,
                li,
                segs[j].chord.canonical_segment,
                band,
                tol,
            )?;
        }
    }

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
/// classification (module docs, steps 3–4). Returns the side faces,
/// strut edges, and top-rim edges in swept order.
#[allow(clippy::too_many_arguments)] // one internal call site; the
// arguments are the sweep's fixed context, not a configuration surface.
fn sweep_loop<T: Decide>(
    body: &mut Body<T>,
    loop_index: usize,
    segs: &[WallSeg<T>],
    hes: &[topo::HalfEdgeKey],
    qs: &[Point3<T>],
    place: Affine3<T>,
    top_place: Affine3<T>,
    normal: Vec3<T>,
    w: Vec3<T>,
    w_norm: T,
    band: Band,
    tol: Tol,
) -> Result<LoopSwept, ExtrudeError> {
    let n = segs.len();

    // Cosurface run structure, decided UP FRONT from the segments alone
    // (deterministic — no body state): `pair[j]` says whether segment
    // `j` continues segment `j − 1 mod n`'s carrier, so `pair[0]` is the
    // wrap join at vertex 0. Deciding all n pairs before any face is
    // minted is what lets a same-carrier run that CROSSES the canonical
    // start vertex (…, n−1, 0) resolve to ONE surface key: with the
    // lazy per-face checks, the prev join at j = n−1 returned before the
    // wrap join could reconcile the run with `faces[0]` (M2 PR 4 review,
    // SHOULD-1 — the run was split into two identical-by-construction
    // cylinders).
    let mut pair = Vec::with_capacity(n);
    for j in 0..n {
        let prev = &segs[(j + n - 1) % n];
        pair.push(
            cosurface(prev, &segs[j], SIDE_COSURFACE, band).map_err(|source| {
                ExtrudeError::CosurfaceEscalated {
                    loop_index,
                    vertex_index: segs[j].chord.canonical_vertex,
                    source,
                }
            })?,
        );
    }

    // Struts: one raised vertex per loop vertex, in traversal order.
    let mut struts: Vec<MevCreated> = Vec::with_capacity(n);
    for j in 0..n {
        let m = body.mev(
            MevSite::Fan {
                he1: hes[j],
                he2: hes[j],
            },
            qs[j] + w,
            strut_spec(segs[j].chord.a, place, qs[j], w, w_norm),
            tol,
        )?;
        struts.push(m);
    }

    // Side quads: the new edge is the top rim (v_j′ → v_{j+1}′), the
    // new face the side wall; the last mef closes against the first top
    // rim (which replaced the consumed strut-minus navigation).
    let mut faces: Vec<FaceKey> = Vec::with_capacity(n);
    let mut top_rims: Vec<EdgeKey> = Vec::with_capacity(n);
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
            body, loop_index, segs, &pair, &faces, j, qs, place, normal, w, band,
        )?;
        let top_q_from = qs[j] + w;
        let top_q_to = qs[(j + 1) % n] + w;
        let mef = body.mef(
            MefSite::Chords {
                he1: struts[j].he_minus,
                he2,
            },
            placed_segment_spec(&segs[j], top_place, normal, top_q_from, top_q_to),
            surface,
            tol,
        )?;
        if j == 0 {
            first_top = Some(mef.he_plus);
        }
        // The honest orientation bit (M5 S11): a concave arc's wall
        // has its material OUTSIDE the carrier cylinder, so the chart
        // normal (unconditionally the outward radial) points into the
        // solid and the face's sense is `false`. Decided at
        // classification time from the profile's stored winding
        // ([`WallSeg::wall_sense`]); attached here because `mef`
        // cannot know the material side (it sees chords, not the
        // profile).
        if !segs[j].wall_sense {
            body.set_face_sense(mef.face, false)?;
        }
        faces.push(mef.face);
        top_rims.push(mef.edge);
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
        let s_prev = body
            .get_surface(k_prev)
            .cloned()
            .ok_or(EulerOpError::StaleGeometry {
                key: topo::GeomRef::Surface(k_prev),
            })?;
        let s_next = body
            .get_surface(k_next)
            .cloned()
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
                    description: EdgeDescriptionSpec::Intersection {
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
                body.set_edge_curve(struts[j].edge, spec, tol)?;
            }
            Ok(DihedralClass::Smooth) => {
                // OQ7's must-carry, applied at construction (M5 PR 9):
                // a definitely-smooth join whose SECOND-ORDER
                // separation is definite is a jet-determinate tangency
                // (the surfaces determine the locus — the fillet-grade
                // line–arc profile join), upgraded to
                // `TangentIntersection` exactly as the transverse arm
                // upgrades to `Intersection`. A zero-side second order
                // (G2/under-determined) keeps the conventional
                // description BY THE PREDICATE; in-band escalates as
                // the same typed sliver (F6).
                let jet = geom_brep::tangent_jet(&s_prev, &s_next, mid, w);
                let arm = geom_brep::curvature_lever_arm(&s_prev, mid)
                    .min(geom_brep::curvature_lever_arm(&s_next, mid))
                    .min(w_norm);
                let margin = Margin::sagitta(jet.kappa_rel.abs(), arm);
                match geom_core::k_stats::decide("tangent_second_order", margin, band) {
                    Ok(geom_core::Sign::Positive) => {
                        let spec = EdgeCurveSpec {
                            description: EdgeDescriptionSpec::TangentIntersection {
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
                        body.set_edge_curve(struts[j].edge, spec, tol)?;
                    }
                    Ok(geom_core::Sign::Zero | geom_core::Sign::Negative) => {}
                    Err(source) => {
                        return Err(ExtrudeError::SliverJoin {
                            loop_index,
                            vertex_index: segs[j].chord.canonical_vertex,
                            source,
                        });
                    }
                }
            }
            Err(source) => {
                return Err(ExtrudeError::SliverJoin {
                    loop_index,
                    vertex_index: segs[j].chord.canonical_vertex,
                    source,
                });
            }
        }
    }

    Ok(LoopSwept {
        faces,
        struts: struts.iter().map(|m| m.edge).collect(),
        top_rims,
    })
}

/// One loop's sweep products, in swept order (see [`sweep_loop`]).
struct LoopSwept {
    /// Side-wall faces, one per segment.
    faces: Vec<FaceKey>,
    /// Strut edges, one per vertex.
    struts: Vec<EdgeKey>,
    /// Top-rim edges, one per segment (the side `mef`s' new edges).
    top_rims: Vec<EdgeKey>,
}

/// The side wall's surface spec for segment `j`, from the precomputed
/// cosurface run structure `pair` (see [`sweep_loop`]): `Shared` with
/// the previous wall when segment `j` continues its carrier, `Shared`
/// with `faces[0]` when `j` starts (or continues into) a run that
/// reaches segment 0 through the wrap join — so a same-carrier run
/// crossing the canonical start vertex resolves to ONE key, whose
/// `u_ref` comes from segment 0, the run's first segment in sweep
/// order; otherwise a freshly built plane (Newell over the quad corners
/// in loop order — outward by the orientation contract) or cylinder
/// (turn-signed axis, crate docs).
#[allow(clippy::too_many_arguments)] // one internal call site (see sweep_loop).
fn side_surface<T: Decide>(
    body: &Body<T>,
    loop_index: usize,
    segs: &[WallSeg<T>],
    pair: &[bool],
    faces: &[FaceKey],
    j: usize,
    qs: &[Point3<T>],
    place: Affine3<T>,
    normal: Vec3<T>,
    w: Vec3<T>,
    band: Band,
) -> Result<FaceSurface<T>, ExtrudeError> {
    let n = segs.len();
    if j > 0 {
        // The prev join: segment j continues segment j − 1's carrier.
        if pair[j] {
            let key = face_surface_key(body, faces[j - 1])?;
            return Ok(FaceSurface::Shared(key));
        }
        // The wrap run: segments j, j+1, …, n−1 chain onto segment 0
        // through the wrap join (`pair[0]`), so segment j belongs to
        // `faces[0]`'s carrier even though its prev join is fresh. (For
        // j = n−1 the chain condition is vacuous and this is exactly
        // the plain wrap join.)
        if pair[0] && ((j + 1)..n).all(|k| pair[k]) {
            let key = face_surface_key(body, faces[0])?;
            return Ok(FaceSurface::Shared(key));
        }
    }
    match segs[j].chord.kind {
        SweptKind::Line => {
            // Quad corners in the side loop's next order starting at
            // the raised start vertex: v_j′, v_j, v_{j+1}, v_{j+1}′.
            let corners = [qs[j] + w, qs[j], qs[(j + 1) % n], qs[(j + 1) % n] + w];
            let plane = newell_plane(&corners, band).map_err(|source| ExtrudeError::SidePlane {
                loop_index,
                segment_index: segs[j].chord.canonical_segment,
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
            // carrier makes, so cylinder and BOTTOM rim circle agree
            // bit-identically. TOP rim centers are computed through
            // `top_place = translation(w) ∘ place`, which associates
            // differently than `c_world + w` — the agreement there is
            // ulp-level, not bitwise (M2 PR 4 review, A7). Certified
            // and deterministic either way: the residual gates bound
            // the drift and identical inputs replay identical bits.
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

/// Upgrades one cap–wall rim edge to `Intersection { cap, wall,
/// witness }` (module docs, step 6 — the ratified rim decision). The
/// witness is minted as the **carrier's mid-parameter point** — the S2
/// witness contract (`WitnessMidpoint`): for arc rims the chord
/// midpoint lies off the carrier by the bulge height, so `carrier(mid)`
/// is the only honest mint (for line rims it coincides with the chord
/// midpoint to rounding). The certified carrier and parameter interval
/// are kept verbatim; the re-description goes through `topo`'s
/// certified `set_edge_curve` door. `classify_dihedral` at the witness
/// decides, metered through the edge's honest extent
/// ([`geom_brep::edge_extent`] — the carrier diameter for near-closed
/// arc rims, whose chord collapses): Transverse upgrades (the only
/// reachable arm for a normal extrusion — cap plane ⊥ wall); Smooth
/// keeps the conventional description (kept total per the D2 split);
/// Indeterminate is the typed [`ExtrudeError::SliverRim`].
#[allow(clippy::too_many_arguments)] // two call sites in one loop; the
// arguments are the upgrade's fixed context, not a configuration
// surface.
fn upgrade_rim<T: Decide>(
    body: &mut Body<T>,
    edge: EdgeKey,
    cap: SurfaceKey,
    wall: SurfaceKey,
    q_from: Point3<T>,
    q_to: Point3<T>,
    loop_index: usize,
    segment_index: usize,
    band: Band,
    tol: Tol,
) -> Result<(), ExtrudeError> {
    let curve_key = body
        .get_edge(edge)
        .ok_or(EulerOpError::StaleKey {
            key: topo::EntityId::Edge(edge),
        })?
        .curve;
    let curve = body
        .get_curve_geom(curve_key)
        .ok_or(EulerOpError::StaleGeometry {
            key: topo::GeomRef::Curve(curve_key),
        })?
        .certified()
        .ok_or(EulerOpError::NullScaffoldCurve { curve: curve_key })?;
    let carrier = curve.carrier().clone();
    let (t0, t1) = curve.params();
    // The mid-parameter point — the same association the certification
    // schedule's middle sample uses (t₀ + (t₁ − t₀)·½, exact dyadic
    // fraction), so the WitnessMidpoint residual is zero by
    // construction.
    let witness = carrier.eval(t0 + (t1 - t0) * T::from_f64(0.5));
    let extent = geom_brep::edge_extent(&carrier, t0, t1, q_from.distance(q_to));
    let s_cap = body
        .get_surface(cap)
        .cloned()
        .ok_or(EulerOpError::StaleGeometry {
            key: topo::GeomRef::Surface(cap),
        })?;
    let s_wall = body
        .get_surface(wall)
        .cloned()
        .ok_or(EulerOpError::StaleGeometry {
            key: topo::GeomRef::Surface(wall),
        })?;
    match classify_dihedral(&s_cap, &s_wall, witness, extent, band) {
        Ok(DihedralClass::Transverse) => {
            let spec = EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    s1: cap,
                    s2: wall,
                    witness,
                },
                carrier,
                param_start: t0,
                param_end: t1,
            };
            body.set_edge_curve(edge, spec, tol)?;
            Ok(())
        }
        // Believed unreachable for a normal extrusion (the cap plane is
        // perpendicular to every wall along the rim); kept total per
        // the D2 conventional split rather than papered over with a
        // panic — tier 3's prefer-intrinsic enforcement exempts
        // definitely-smooth edges, so a Smooth rim stays valid.
        Ok(DihedralClass::Smooth) => Ok(()),
        Err(source) => Err(ExtrudeError::SliverRim {
            loop_index,
            segment_index,
            source,
        }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// S6 (two-tolerance, D4 ¶1 addendum): the extrusion pair —
    /// definitely-degenerate (`DegenerateExtrusion`) and in-band
    /// (`ExtrusionEscalated`) — is one user situation; both arms carry
    /// the shared recourse fragment.
    #[test]
    fn extrusion_pair_carries_the_shared_recourse() {
        let msg = ExtrudeError::DegenerateExtrusion.to_string();
        assert_eq!(
            msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
            1,
            "{msg}"
        );

        let msg = ExtrudeError::ExtrusionEscalated {
            source: Indeterminate {
                margin: geom_core::MarginDiag::Value(5e-9),
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("extrusion_normal_component"),
            },
        }
        .to_string();
        assert_eq!(
            msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
            1,
            "{msg}"
        );
    }
}
