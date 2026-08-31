//! Planar-face tessellation: projection into the face plane, CDT with
//! boundary constraints, even-odd region classification (rings and
//! non-convex boundaries), outward winding from the outer loop's
//! projected orientation.
//!
//! The plane is affine, so no deviation certificate is needed
//! (deviation 0 in exact arithmetic; boundary chord points carry the
//! kernel's ≤ ε carrier residual, crate docs). Interior sampling is
//! unnecessary — boundary points alone triangulate the region.
//!
//! # Region classification is combinatorial, not geometric
//!
//! Which CDT triangles are *inside* the face is decided by a flood fill
//! over the triangulation's face-adjacency graph, seeded at the outer
//! face (outside every loop) and toggling only when the walk crosses a
//! boundary constraint edge. This computes exactly the even-odd rule —
//! parity of boundary crossings on a path from infinity — but as a
//! graph traversal, with **no floating-point decision at all**.
//!
//! The predecessor tested each triangle's centroid against the boundary
//! polylines by ray casting. That is unsound for needle triangles
//! riding the boundary, and issue #111 caught it in the wild: an A×Z
//! boolean face carries four boundary vertices on one carrier line, one
//! of them 1 ulp off it (an exact 5/8 crossing reached through
//! non-dyadic slope-3/5 arithmetic). The needle the CDT correctly
//! builds there is ~3e-18 thick, while merely *forming* a centroid
//! rounds its coordinates by ~5e-17 — the constructed point lands on
//! the wrong side of the boundary edge before any predicate runs, so no
//! amount of exactness in the test could rescue it. The classification
//! had to stop depending on a constructed point.
//!
//! Two structural consequences, both load-bearing:
//!
//! * Across a boundary edge the classification necessarily *differs*,
//!   so every boundary segment is an edge of exactly one kept triangle
//!   on this face — watertightness by construction rather than by
//!   numerical luck.
//! * A degenerate needle may still be kept (a zero-area sliver inside
//!   the region) or dropped (outside it), but never emitted on the
//!   outside — which is the defect that leaks meshes.
//!
//! # The chart frame is re-derived from the boundary (issue #284)
//!
//! A plane's chart frame is pure *convention* — nothing downstream
//! depends on which orthonormal in-plane basis is picked (the same
//! status as seam placement), and chart coordinates only decide CDT
//! connectivity; mesh positions come from the 3-D chords. The stored
//! `Surface::Plane` axes are therefore **not** used here: translator
//! noise in imported axes (components ~1e-33 where an exact axis has
//! 0.0) projects boundary coordinates to ~1e-67 — nonzero but below
//! spade's coordinate domain (`MIN_ALLOWED_VALUE` = 2⁻¹⁴²), and the
//! near-degenerate chart refuses CDT insertion on bodies that are
//! perfectly valid at rest (#284: two wild-corpus imports).
//!
//! Instead the frame is a pure function of the outer boundary's own
//! 3-D chord points, in walk order ([`chart_frame`]):
//!
//! * **anchor** — the outer walk's FIRST point, verbatim;
//! * **normal** — the anchor-translated Newell cross-sum over
//!   consecutive point pairs (left-to-right, wrapping), normalized.
//!   Newell orients the normal so the walk is CCW about it, and by
//!   interior-left the outer walk is CCW about the face's OUTWARD
//!   normal — so this is the outward normal by construction (the
//!   section below says on what);
//! * **u** — toward the walk point farthest from the anchor (the
//!   first such point in walk order on exact ties), shed of its
//!   normal component and normalized; **v** = normal × u.
//!
//! Well-conditioned by construction — u's length before normalizing
//! is the loop's own extent scale, and the off-plane component it
//! sheds is bounded by the carrier residual (≤ ε) — with no new
//! tolerance and no value snapping.
//!
//! The anchor being an INPUT point rather than a constructed centroid
//! is load-bearing, not a style choice, because of spade's coordinate
//! floor (`MIN_ALLOWED_VALUE` = 2⁻¹⁴²: a coordinate must be exactly 0
//! or at least that large). Every quantity in the frame and in the
//! projection is then built from *differences of input floats*, and
//! for the class this fix closes — noisy stored AXES over clean
//! positions, the #284 wild-corpus class, whose exact-zero structure
//! lives in exactly-equal input coordinate columns (an axis-aligned
//! face's normal-axis column) — the guarantee is by construction:
//! equal columns difference to exact 0, the Newell sum's in-plane
//! components are exactly 0, the frame's out-of-plane components are
//! exactly 0, and an exact-zero chart coordinate comes out as float
//! 0.0 or as a residue at the ulp scale of real-sized terms
//! (~1e-19·extent) — both spade-legal (pinned by the exactness unit
//! falsifier in this module's tests). A constructed centroid instead
//! seeds ulp-scale residues (its rounding) into the Newell sum, which
//! manufactures ~1e-33-scale frame components, and a projection term
//! `w·residue` then lands around 1e-51 — nonzero but far below the
//! floor, the very refusal this frame exists to remove (measured on
//! the wild OLED's bottom face while this fix was being built).
//!
//! # The far point's v-coordinate is WRITTEN as the zero it is
//!
//! That construction guarantee covers the noisy-AXES class and only
//! it. When the boundary POSITIONS themselves carry off-plane noise ν
//! there is no exactly-equal input column to lean on — and one chart
//! coordinate is an exact zero regardless: `u` is the far point's own
//! rejection from the normal, so `far` lies in span{normal, u} and its
//! `v = far · (normal × u)` is a determinant with a repeated row, zero
//! in exact arithmetic. Evaluated in floats it lands at ~ν², and for
//! ν ≲ √(2⁻¹⁴²) ≈ 4e-22 that is nonzero BELOW spade's coordinate
//! floor — which `insert` refuses, so the face returns
//! [`TessellateError::Triangulation`] at **every** δ and no caller can
//! turn a tolerance to escape it.
//!
//! So [`tessellate_planar`] WRITES that one coordinate as `0.0`
//! instead of reading it back off the dot product.
//!
//! **That is not the value snapping this module refuses.** Value
//! snapping moves a coordinate the construction genuinely produced
//! onto a nearby one it did not: it manufactures an agreement, and a
//! manufactured agreement is indistinguishable downstream from a real
//! one. The far point's v is the opposite case. The construction
//! produced zero — exactly, and for a stated structural reason — and
//! it is the float residue that is invented. Writing `0.0` there
//! refuses to invent a nonzero the frame never had, which is the
//! doctrine's own argument rather than an exception carved out of it.
//!
//! That argument is also what SITES the write here, at the one
//! coordinate whose exactness is known from how the frame was built,
//! rather than as a blanket `mitigate_underflow` filter over every
//! chart coordinate. A blanket filter cannot tell the far point's
//! structural zero from a neighbouring point's small-but-real
//! v-coordinate, so it would snap the second on the strength of an
//! argument that holds only for the first — value snapping exactly as
//! banned, wearing the structural zero's justification. Nor would it
//! rescue the case it looks built for: a near-degenerate chart (#284's
//! class) hands the CDT a zero-area region whether or not its
//! coordinates are snapped, which is why the fix there had to be the
//! frame, and was.
//!
//! What the write does NOT claim is that every chart coordinate now
//! clears the floor. A boundary point that happens to lie on the
//! anchor→far diagonal has an exact-zero v as well — but by GEOMETRY,
//! not by construction: the frame does not know it, this write does
//! not reach it, and its residue may still refuse typed (held to
//! tessellate-or-typed by `probe_e2` in `tests/newell_probes.rs`). The
//! anchor needs no write at all: its `w` is the exactly-zero vector,
//! so both its coordinates are already float `0.0`.
//!
//! Deterministic (D9): fixed evaluation order over the walk, so the
//! frame is bit-identical across rebuilds and debug/release. Once the
//! frame is fixed the projection is a pure per-point function, so a
//! repeated 3-D point projects to bitwise-identical chart coordinates
//! — the slit cancellation below depends on exactly that, and it is
//! why the write above selects its point by the BITS of the position
//! rather than by walk index: the slit's two traversals of the far
//! point are one 3-D point met twice and must stay one chart point. A
//! degenerate outer loop (zero extent or zero area vector) yields
//! non-finite frame components, which spade's `insert` refuses — the
//! typed [`TessellateError::Triangulation`] path, fail-loud.
//!
//! Slit note: a full-2π revolve's annulus wall is a *slit* polygon —
//! one loop traversing its seam segment twice with bitwise-identical
//! projected points. The CDT dedupes the repeated points to the same
//! handles (same positions ⇒ same ids), so both traversals land on the
//! same constraint edges. Crossing multiplicity is therefore counted,
//! not merely recorded: the fill toggles only on **odd** multiplicity,
//! so the slit's doubled traversal cancels and classification proceeds
//! exactly as if the slit were absent — the same cancellation the
//! even-odd crossing count used to get, now exact.
//!
//! # What "the outward normal by construction" rests on
//!
//! One rule, stated once in `topo::entity` and not restated here: by
//! interior-left, a face's outer loop runs CCW about the face's
//! **outward** normal. Newell orients its cross-sum so the walk is CCW
//! about it, so the Newell normal of the outer walk IS the outward
//! normal whenever the body honours the rule.
//!
//! **The S10 identity does not enter.** Outward = `sense_sign` · chart
//! normal only relates the outward normal to a STORED chart, and
//! [`chart_frame`] reads no chart — only walk indices and 3-D points.
//! So the premise that S10 retired (*"a face's stored normal is the
//! outward normal"*) is a different claim from this one, and its
//! retirement does not reach here.
//!
//! **What checks the rule, and what does not.** A body whose stored
//! `sense` disagrees with its stored winding violates it, and `topo`'s
//! tier-3 validator refuses such a body by name (check 6,
//! `LoopRoleInverted`). **This crate does not run that check and does
//! not re-derive it** — tessellation does not re-validate, as
//! [`chart_frame`] says in its own docs. So the honest statement is:
//! the rule is ratified, its violation is *refusable* upstream, and
//! the mesher assumes a body that has been through the door rather
//! than certifying one that has not.

use std::collections::HashMap;

use geom_core::{Point3, Vec3};
use spade::{
    ConstrainedDelaunayTriangulation, Point2 as SpadePoint, Triangulation,
    handles::{DirectedEdgeHandle, FixedFaceHandle, FixedVertexHandle, InnerTag},
};
use topo::{Body, EdgeKey, FaceKey, LoopKey};

use crate::types::TessellateError;
use crate::walk::loop_edges;

/// Tessellates one planar face into outward-wound triangles. The
/// chart frame comes from [`chart_frame`], not from the face's stored
/// `Surface::Plane` axes (module docs, issue #284).
pub(crate) fn tessellate_planar(
    body: &Body<f64>,
    fk: FaceKey,
    chords: &HashMap<EdgeKey, Vec<u32>>,
    positions: &[Point3<f64>],
) -> Result<Vec<[u32; 3]>, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;

    // Loop id cycles (outer first, then rings in face order).
    let mut loops: Vec<Vec<u32>> = Vec::with_capacity(1 + face.rings.len());
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        loops.push(loop_ids(body, fk, lk, chords)?);
    }

    // The frame from the outer boundary alone (rings lie inside the
    // outer loop, so its extent governs the conditioning), then the
    // pure per-point projection of every loop.
    let frame = chart_frame(&loops[0], positions);
    let project = |id: &u32| -> [f64; 2] { frame.project(positions[*id as usize]) };
    let polygons: Vec<Vec<[f64; 2]>> = loops
        .iter()
        .map(|ids| ids.iter().map(project).collect())
        .collect();

    triangulate_chart(fk, &loops, &polygons)
}

/// Bitwise position identity: two mesh ids denote the same 3-D point
/// when their coordinates agree bit for bit. Stricter than `==` (which
/// equates ±0.0 and refuses NaN), and deliberately so — it is what
/// makes [`ChartFrame::project`] a pure function of the point rather
/// than of its walk position.
fn same_position(a: Point3<f64>, b: Point3<f64>) -> bool {
    a.x.to_bits() == b.x.to_bits()
        && a.y.to_bits() == b.y.to_bits()
        && a.z.to_bits() == b.z.to_bits()
}

/// The boundary-derived chart of a planar face: anchor, in-plane axes,
/// and the far point `u` was built from.
struct ChartFrame {
    origin: Point3<f64>,
    u_ref: Vec3<f64>,
    v_ref: Vec3<f64>,
    /// The point `u_ref` is the rejection of, verbatim. Its
    /// v-coordinate is an exact zero by that construction, so
    /// [`ChartFrame::project`] writes it rather than reading the
    /// sub-floor residue back off the dot product (module docs).
    far_at: Point3<f64>,
}

impl ChartFrame {
    /// One point's chart coordinates. Pure in the point: equal
    /// positions project to bitwise-equal coordinates, which is what
    /// lets the CDT dedupe a slit seam's two traversals.
    fn project(&self, p: Point3<f64>) -> [f64; 2] {
        let w = p - self.origin;
        // The far point's v is an engineered exact zero: `u_ref` is
        // that point's own rejection from the normal, so the dot
        // product is a determinant with a repeated row (module docs).
        // Written, not read back — the float residue is ~ν², which
        // goes sub-floor for spade and refuses the whole face.
        let v = if same_position(p, self.far_at) {
            0.0
        } else {
            w.dot(self.v_ref)
        };
        [w.dot(self.u_ref), v]
    }
}

/// The boundary-derived chart frame of a planar face — a pure function
/// of the outer loop's chord points in walk order (module docs: the
/// convention, why the anchor is an input point, conditioning,
/// determinism, and why the stored axes are not consulted).
///
/// The Newell cross-sum follows `geom_brep::newell_plane`'s fixed
/// left-to-right evaluation order (D9) but translates to the walk's
/// first point instead of the centroid (module docs: the coordinate
/// floor argument); certification is deliberately absent —
/// tessellation does not re-validate (crate docs), and the in-plane
/// axis is this lane's own extent-aligned composition, not
/// `newell_plane`'s ambient `orthonormal_basis` pick. A degenerate
/// loop propagates non-finite components to the CDT, which refuses
/// typed.
fn chart_frame(outer: &[u32], positions: &[Point3<f64>]) -> ChartFrame {
    let at = |id: u32| positions[id as usize];
    // Anchor: the walk's first point, verbatim.
    let origin = at(outer[0]);
    // Newell normal: anchor-translated cross-sum over consecutive
    // pairs, left-to-right, wrapping. Slit walks cancel their doubled
    // segments here exactly as they do in the shoelace.
    let mut normal_sum = Vec3::zero();
    for (i, &id) in outer.iter().enumerate() {
        let next = at(outer[(i + 1) % outer.len()]);
        normal_sum = normal_sum + (at(id) - origin).cross(next - origin);
    }
    let normal = normal_sum.normalize();
    // u: toward the walk's farthest point from the anchor (strict >
    // keeps the FIRST maximum in walk order — deterministic on exact
    // ties), shed of its off-plane component.
    let mut far = Vec3::zero();
    let mut far_at = origin;
    let mut far_d2 = f64::NEG_INFINITY;
    for &id in outer {
        let d = at(id) - origin;
        let d2 = d.norm_squared();
        if d2 > far_d2 {
            far_d2 = d2;
            far = d;
            far_at = at(id);
        }
    }
    let u_ref = far.reject_from(normal).normalize();
    ChartFrame {
        origin,
        u_ref,
        v_ref: normal.cross(u_ref),
        far_at,
    }
}

/// Triangulates a face already reduced to its chart: `loops[i][j]` is
/// the mesh id of `polygons[i][j]`, outer loop first. Split out from
/// [`tessellate_planar`] so that a face's exact projected coordinates
/// can be replayed in isolation (the #111 regression is a property of
/// the chart alone).
fn triangulate_chart(
    fk: FaceKey,
    loops: &[Vec<u32>],
    polygons: &[Vec<[f64; 2]>],
) -> Result<Vec<[u32; 3]>, TessellateError> {
    // CDT: every loop's points first, then the boundary constraints.
    // The two passes must not interleave: inserting a vertex that lands
    // exactly on an existing constraint edge splits it, which would
    // invalidate the crossing bookkeeping built below.
    let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
        ConstrainedDelaunayTriangulation::new();
    let mut meta: Vec<u32> = Vec::new(); // handle index -> mesh id
    let mut handles: Vec<Vec<FixedVertexHandle>> = Vec::new();
    for (ids, poly) in loops.iter().zip(polygons) {
        let mut hs = Vec::with_capacity(ids.len());
        for (&id, &[u, v]) in ids.iter().zip(poly) {
            let h = cdt
                .insert(SpadePoint::new(u, v))
                .map_err(|_| TessellateError::Triangulation { face: fk })?;
            if h.index() == meta.len() {
                meta.push(id);
            }
            hs.push(h);
        }
        handles.push(hs);
    }

    // Boundary constraints, counting how often each resulting CDT edge
    // is traversed. A loop segment may be realised by several constraint
    // edges (an intervening vertex splits it), and the slit's two
    // traversals return the same edges twice — both handled by keying on
    // the sub-edges spade reports rather than on the segment endpoints.
    let mut crossings: HashMap<(usize, usize), u32> = HashMap::new();
    for hs in &handles {
        for i in 0..hs.len() {
            let (a, b) = (hs[i], hs[(i + 1) % hs.len()]);
            if a == b {
                continue; // slit-collapsed segment: no boundary at all
            }
            // `try_add_constraint` reports the constraint edges joining
            // `a` and `b`, including pre-existing ones, and leaves the
            // triangulation untouched (returning empty) when the segment
            // would cross an existing constraint — corrupt input
            // geometry, kept a typed failure rather than a panic.
            let realised = cdt.try_add_constraint(a, b);
            if realised.is_empty() {
                return Err(TessellateError::Triangulation { face: fk });
            }
            for e in realised {
                let e = cdt.directed_edge(e);
                *crossings.entry(edge_key(e)).or_insert(0) += 1;
            }
        }
    }

    // Outer-loop orientation in the chart frame decides the flip.
    //
    // S10 CATEGORY B — do NOT multiply by the face's `sense_sign`. The
    // sign here is derived from the loop's STORED TRAVERSAL (the
    // projected shoelace), and by interior-left the outer loop winds
    // CCW about the face's OUTWARD normal — so the shoelace comes out
    // negative in the chart frame precisely when the face is reversed,
    // and the flip it then triggers is already the right one. `revert`
    // reverses the loops AND flips `sense` together; multiplying here
    // would apply that reversal twice and emit inward-wound triangles
    // on exactly the reversed faces the multiply was meant to serve.
    //
    // On the float sign itself: unreified, but not in-band — getting
    // it wrong needs a face whose PROJECTED area is ~0, and it flips
    // the whole patch at once, so `check_mesh` catches it as
    // `MismatchedWinding` against the neighbouring faces (unlike the
    // #111 per-triangle decision, which could fail on one sliver and
    // leak past every downstream check but `check_mesh`'s edge
    // census). Read that caveat honestly: "~0 projected area" bounds
    // when the SIGN READ is unreliable, and was never a claim that the
    // sign is knowable in advance. A negative outer shoelace is an
    // ordinary, well-conditioned input — it is what a reversed face
    // looks like — so the code reads the sign rather than assuming it,
    // and would have to keep doing so even if the degeneracy argument
    // were dropped entirely.
    //
    // Under the boundary-derived frame (#284) the Newell normal is
    // oriented BY the outer walk, so the outer shoelace is positive by
    // construction and the flip is expected false — for charts built
    // by [`tessellate_planar`]. The sign read stays: it is what makes
    // that claim checkable rather than assumed, and replayed charts
    // (this fn's other caller is the #111 stored-chart regression) may
    // carry any frame.
    let flip = shoelace2(&polygons[0]) < 0.0;

    let inside = classify_faces(&cdt, &crossings);

    // WHAT MAKES #678's FAN INERT HERE: this lane's CDT holds the
    // boundary polygons' vertices and nothing else, so there is no
    // column for two entries sharing a mesh vertex to be fanned over.
    // TWO checks, because one of them is not enough and a review of
    // #887 showed exactly how.
    //
    // The first says every CDT vertex is accounted for in `meta`.
    // `meta` grows only in the insertion pass above, guarded by
    // `h.index() == meta.len()`, and `try_add_constraint` returns
    // empty rather than inserting — so this reds for a vertex that
    // arrived WITHOUT bookkeeping, which is the foreign-insertion
    // case. It does NOT red for an interior grid added the way
    // `trimmed` adds one (insert, then push to `meta`), and the
    // comment that claimed it did was over-claiming: an interior grid
    // is the likely edit, and it would arrive WITH bookkeeping.
    //
    // The second closes that: `meta` can never hold more entries than
    // the boundary walk offered, so an interior point pushed into it
    // pushes the count past the loops' own total. Together they say
    // what the paragraph above says (D2 addendum row 5).
    let boundary_entries: usize = loops.iter().map(Vec::len).sum();
    debug_assert_eq!(
        cdt.num_vertices(),
        meta.len(),
        "face {fk:?}: the planar CDT carries {} vertices for {} tracked ones — \
         something other than the boundary walk inserted one",
        cdt.num_vertices(),
        meta.len()
    );
    debug_assert!(
        meta.len() <= boundary_entries,
        "face {fk:?}: {} tracked CDT vertices for {boundary_entries} boundary \
         entries — this lane grew an interior point, and the duplicate-id skip \
         below is `curved`'s pole-fan idiom without `curved`'s separation \
         argument (#678)",
        meta.len()
    );

    // Emit the interior triangles.
    let mut triangles = Vec::new();
    for f in cdt.inner_faces() {
        if !inside[f.fix().index()] {
            continue;
        }
        let vs = f.vertices();
        let ids = [
            meta[vs[0].fix().index()],
            meta[vs[1].fix().index()],
            meta[vs[2].fix().index()],
        ];
        // A triangle with two corners on one mesh vertex is
        // degenerate. Dropping it is `curved`'s idiom, and there it
        // collapses a pole fan (#678); here the drop is all it is —
        // the duplicate ids this lane sees are a slit's two
        // traversals, at the SAME UV and deduped by spade to one CDT
        // vertex, and there is no interior column for anything to fan
        // over. That second half is asserted above, by the pair of
        // checks and not by either alone.
        if ids[0] == ids[1] || ids[1] == ids[2] || ids[0] == ids[2] {
            continue; // slit-degenerate sliver
        }
        triangles.push(if flip { [ids[0], ids[2], ids[1]] } else { ids });
    }
    Ok(triangles)
}

/// A CDT edge's identity as an unordered pair of vertex indices.
pub(crate) fn edge_key(
    e: DirectedEdgeHandle<'_, SpadePoint<f64>, (), spade::CdtEdge<()>, ()>,
) -> (usize, usize) {
    let (p, q) = (e.from().fix().index(), e.to().fix().index());
    (p.min(q), p.max(q))
}

/// Even-odd interior flag per CDT face, indexed by face index (0 is
/// spade's outer face, which is outside every loop).
///
/// A breadth-first walk of the face-adjacency graph from the outer face,
/// toggling the flag exactly when it steps across a boundary segment
/// traversed an odd number of times. Purely combinatorial: the result
/// depends on the triangulation's connectivity and on integer crossing
/// counts, never on a coordinate comparison.
pub(crate) fn classify_faces(
    cdt: &ConstrainedDelaunayTriangulation<SpadePoint<f64>>,
    crossings: &HashMap<(usize, usize), u32>,
) -> Vec<bool> {
    let toggles = |e| crossings.get(&edge_key(e)).is_some_and(|n| n % 2 == 1);
    let mut inside = vec![false; cdt.num_all_faces()];
    let mut seen = vec![false; cdt.num_all_faces()];
    let mut queue: Vec<FixedFaceHandle<InnerTag>> = Vec::new();

    // Seed: step inwards across every convex-hull edge. The hull bounds
    // the outer face, which is outside every loop; a hull edge that is
    // itself a boundary segment toggles on the way in.
    for hull in cdt.convex_hull() {
        let e = if hull.face().is_outer() {
            hull.rev()
        } else {
            hull
        };
        let Some(f) = e.face().as_inner() else {
            continue; // both sides outer: a degenerate, edge-only CDT
        };
        let i = f.fix().index();
        if !seen[i] {
            seen[i] = true;
            inside[i] = toggles(e);
            queue.push(f.fix());
        }
    }

    let mut head = 0;
    while head < queue.len() {
        let f = queue[head];
        head += 1;
        let here = inside[f.index()];
        for e in cdt.face(f).adjacent_edges() {
            let Some(g) = e.rev().face().as_inner() else {
                continue; // the outer face, already the walk's origin
            };
            let j = g.fix().index();
            if !seen[j] {
                seen[j] = true;
                inside[j] = here ^ toggles(e);
                queue.push(g.fix());
            }
        }
    }
    inside
}

/// The loop's chord ids in walk order (each traversal contributes its
/// points minus the last, which the next traversal repeats).
fn loop_ids(
    body: &Body<f64>,
    fk: FaceKey,
    lk: LoopKey,
    chords: &HashMap<EdgeKey, Vec<u32>>,
) -> Result<Vec<u32>, TessellateError> {
    let mut out = Vec::new();
    for (ek, forward) in loop_edges(body, lk, fk)? {
        let ids = chords.get(&ek).ok_or(TessellateError::MissingEntity {
            what: "edge chords",
        })?;
        if forward {
            out.extend_from_slice(&ids[..ids.len() - 1]);
        } else {
            out.extend(ids[1..].iter().rev());
        }
    }
    Ok(out)
}

/// Twice the signed area of a 2-D polygon.
pub(crate) fn shoelace2(poly: &[[f64; 2]]) -> f64 {
    let mut s = 0.0;
    for (i, p) in poly.iter().enumerate() {
        let q = poly[(i + 1) % poly.len()];
        s += p[0] * q[1] - q[0] * p[1];
    }
    s
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{shoelace2, triangulate_chart};
    use topo::FaceKey;

    /// Face `19v3` of the issue-#111 A×Z intersect (A's left inner-leg
    /// slant), captured as the exact projected chart the tessellator
    /// builds — bit patterns, because the defect lives in the last ulp.
    ///
    /// Boundary walk `48 → 34 → 1 → 2 → 13 → 52 → 24 → 48`. Vertices
    /// `2`, `13`, `48` are exactly collinear (A's leg-top carrier);
    /// vertex `24`, the boolean's crossing of that carrier with Z's
    /// slope-3/5 diagonal, is exactly 5/8 in real arithmetic but lands
    /// 1 ulp off the carrier, and `13 → 52 → 24` is the #93 seam
    /// hexagon's notch dipping in between. The CDT therefore builds a
    /// needle triangle `[24, 13, 48]` of twice-area 4e-18 that lies
    /// OUTSIDE the face — verified present in the triangulation of this
    /// very chart, and the exact triangle the predecessor kept: forming
    /// a centroid rounds it by ~5e-17, ten times the needle's ~3e-18
    /// half-thickness, so the constructed point crosses to the interior
    /// side of boundary edge `24→48` and every point-in-polygon test,
    /// f64 or exact, then answers "inside".
    const FACE_19V3: [(u64, u64); 7] = [
        (4581431797661461788, 4602803313439314012),  // 48
        (4601278341968785480, 13826922315894092423), // 34
        (4606183720244804921, 13827710779582984058), // 1
        (4607527494452543132, 4599358972061524235),  // 2
        (4603999415142092400, 4600811405172640406),  // 13
        (4603471649560759920, 4590705563545250395),  // 52
        (4602000164000415209, 4601433876505975908),  // 24
    ];
    const FACE_19V3_IDS: [u32; 7] = [48, 34, 1, 2, 13, 52, 24];

    fn chart(bits: &[(u64, u64)]) -> Vec<[f64; 2]> {
        bits.iter()
            .map(|&(u, v)| [f64::from_bits(u), f64::from_bits(v)])
            .collect()
    }

    fn sorted(t: [u32; 3]) -> [u32; 3] {
        let mut t = t;
        t.sort_unstable();
        t
    }

    /// Twice the signed area of a triangle in the chart.
    fn tri2(poly: &[[f64; 2]], ids: &[u32], t: [u32; 3]) -> f64 {
        let at = |id: u32| poly[ids.iter().position(|&x| x == id).unwrap()];
        let (a, b, c) = (at(t[0]), at(t[1]), at(t[2]));
        (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
    }

    /// Every boundary segment must be an edge of exactly one emitted
    /// triangle — the per-face half of watertightness, and precisely
    /// what the centroid-parity filter broke (the shared segment went
    /// to 3 uses, its neighbours to 1).
    fn assert_covers_boundary(loops: &[Vec<u32>], tris: &[[u32; 3]]) {
        let mut uses: std::collections::HashMap<(u32, u32), u32> = std::collections::HashMap::new();
        for t in tris {
            for k in 0..3 {
                let (a, b) = (t[k], t[(k + 1) % 3]);
                *uses.entry((a.min(b), a.max(b))).or_insert(0) += 1;
            }
        }
        for ids in loops {
            for i in 0..ids.len() {
                let (a, b) = (ids[i], ids[(i + 1) % ids.len()]);
                if a == b {
                    continue;
                }
                let key = (a.min(b), a.max(b));
                assert_eq!(
                    uses.get(&key).copied().unwrap_or(0),
                    1,
                    "boundary segment {key:?} used {:?} times, want 1",
                    uses.get(&key)
                );
            }
        }
    }

    #[test]
    fn issue111_needle_on_the_az_leg_carrier_is_not_emitted() {
        let poly = chart(&FACE_19V3);
        let loops = vec![FACE_19V3_IDS.to_vec()];
        let tris = triangulate_chart(FaceKey::default(), &loops, std::slice::from_ref(&poly))
            .expect("face 19v3 triangulates");

        // The exterior needle the old centroid test kept.
        assert!(
            !tris.iter().any(|&t| sorted(t) == [13, 24, 48]),
            "REGRESSION (issue #111): exterior needle [24, 13, 48] emitted; \
             triangles = {tris:?}"
        );
        assert_covers_boundary(&loops, &tris);

        // The kept triangles must tile the region exactly: signed areas
        // sum to the loop's own signed area.
        let total: f64 = tris.iter().map(|&t| tri2(&poly, &FACE_19V3_IDS, t)).sum();
        let want = shoelace2(&poly);
        assert!(
            (total.abs() - want.abs()).abs() < 1e-12,
            "emitted area {total} does not tile the loop area {want}"
        );
    }

    /// The anchor-choice falsifier (#284, R1 MAJOR-2 of PR #301): the
    /// module docs' input-exactness argument, pinned as executable
    /// assertions on an OLED-bottom-shaped axis-aligned loop (equal z
    /// column whose sum/n rounds inexactly). The frame must preserve
    /// input exactness — anchor a boundary point verbatim, Newell
    /// in-plane components exact 0, frame out-of-plane components
    /// exact 0, and every projected chart coordinate either exactly
    /// 0.0 or at spade-legal magnitude (≥ 2⁻¹⁴²). Re-anchoring at the
    /// constructed centroid turns every one of these assertions red
    /// (its z rounds off the input plane and seeds the Newell sum) —
    /// this is the in-suite guard for the mutation that only the wild
    /// generator's OLED cell used to catch.
    #[test]
    fn chart_frame_preserves_input_exactness_on_axis_aligned_loops() {
        use geom_core::Point3;
        let z = -0.0016;
        let positions: Vec<Point3<f64>> = [
            (0.002, 0.0),
            (0.032036, 0.0),
            (0.034036, 0.002),
            (0.034036, 0.018574),
            (0.032036, 0.020574),
            (0.002, 0.020574),
            (0.0, 0.018574),
            (0.0, 0.002),
        ]
        .iter()
        .map(|&(x, y)| Point3::new(x, y, z))
        .collect();
        #[allow(clippy::cast_possible_truncation)]
        let outer: Vec<u32> = (0..positions.len() as u32).collect();

        // The centroid of this loop is NOT an input point: its z
        // rounds off the plane. Guard the guard: if a future edit
        // picks coordinates where the centroid happens to be exact,
        // this test would stop discriminating — fail it loudly then.
        let n = positions.len() as f64;
        let cz = positions.iter().map(|p| p.z).sum::<f64>() / n;
        assert_ne!(
            cz.to_bits(),
            z.to_bits(),
            "loop no longer discriminates the centroid anchor — pick new coordinates"
        );

        let frame = super::chart_frame(&outer, &positions);
        let (origin, u_ref, v_ref) = (frame.origin, frame.u_ref, frame.v_ref);
        assert_eq!(
            (origin.x.to_bits(), origin.y.to_bits(), origin.z.to_bits()),
            (
                positions[0].x.to_bits(),
                positions[0].y.to_bits(),
                positions[0].z.to_bits()
            ),
            "anchor must be the first walk point, verbatim"
        );
        assert_eq!(u_ref.z, 0.0, "u out-of-plane component must be exact 0");
        assert_eq!(v_ref.z, 0.0, "v out-of-plane component must be exact 0");
        // The floor: every chart coordinate exactly 0.0 or spade-legal.
        let floor = 2.0_f64.powi(-142);
        for p in &positions {
            let w = *p - origin;
            for c in [w.dot(u_ref), w.dot(v_ref)] {
                assert!(
                    c == 0.0 || c.abs() >= floor,
                    "chart coordinate {c:e} in spade's forbidden (0, 2^-142) band"
                );
            }
        }
        // The far point is a real boundary point of this loop, and its
        // v is the coordinate the projection writes rather than reads.
        assert!(
            positions
                .iter()
                .any(|p| super::same_position(*p, frame.far_at)),
            "far must be an input point, verbatim"
        );
    }

    /// Issue 555, the MECHANISM rather than the outcome: on a boundary
    /// carrying off-plane position noise ν the far point's raw
    /// `w · v_ref` lands in spade's forbidden `(0, 2⁻¹⁴²)` band — the
    /// residue of an exact zero — and the projection emits `0.0`
    /// there instead. Both halves are asserted, because asserting only
    /// the second would still pass if the frame stopped producing a
    /// sub-floor residue for some unrelated reason, and the row would
    /// then be pinning nothing.
    #[test]
    fn the_far_points_subfloor_v_residue_is_written_as_zero() {
        use geom_core::Point3;
        let floor = 2.0_f64.powi(-142);
        for exp in [-30i32, -45, -60] {
            let nu = 10.0f64.powi(exp);
            // The skewed rectangle: z = x·ν, no exactly-equal column.
            let positions: Vec<Point3<f64>> = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]
                .iter()
                .map(|&(x, y)| Point3::new(x, y, x * nu))
                .collect();
            #[allow(clippy::cast_possible_truncation)]
            let outer: Vec<u32> = (0..positions.len() as u32).collect();
            let frame = super::chart_frame(&outer, &positions);

            // The far point is (2, 1, 2ν) — the diagonal corner.
            let far = frame.far_at;
            assert!(
                super::same_position(far, positions[2]),
                "1e{exp}: far drifted off the diagonal corner: {far:?}"
            );

            // Half one: the raw residue is sub-floor, so an unwritten
            // projection would hand spade a coordinate it refuses.
            let raw = (far - frame.origin).dot(frame.v_ref);
            assert!(
                raw != 0.0 && raw.abs() < floor,
                "1e{exp}: raw v residue {raw:e} is not sub-floor — this row \
                 no longer exercises the defect, pick a new noise scale"
            );

            // Half two: what the projection actually emits.
            let [_, v] = frame.project(far);
            assert_eq!(v, 0.0, "1e{exp}: far point's v must be written zero");

            // Purity in the point: the same position, met again (a
            // slit seam's second traversal), projects identically.
            assert_eq!(frame.project(far), frame.project(positions[2]));
        }
    }

    /// The write is scoped to the far point alone: every other
    /// boundary point keeps the v its dot product produced. A blanket
    /// filter would flatten these too, which is the difference the
    /// module docs' siting argument turns on.
    #[test]
    fn only_the_far_points_v_is_written() {
        use geom_core::Point3;
        let nu = 1.0e-30;
        let positions: Vec<Point3<f64>> = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]
            .iter()
            .map(|&(x, y)| Point3::new(x, y, x * nu))
            .collect();
        #[allow(clippy::cast_possible_truncation)]
        let outer: Vec<u32> = (0..positions.len() as u32).collect();
        let frame = super::chart_frame(&outer, &positions);
        for (i, p) in positions.iter().enumerate() {
            let [_, v] = frame.project(*p);
            if super::same_position(*p, frame.far_at) {
                continue;
            }
            let raw = (*p - frame.origin).dot(frame.v_ref);
            assert_eq!(
                v.to_bits(),
                raw.to_bits(),
                "point {i}: v was rewritten but is not the far point"
            );
        }
    }

    #[test]
    fn slit_traversed_twice_does_not_toggle_the_region() {
        // A unit square with a slit cut in from the left edge to its
        // centre, traversed out and back — the revolve-annulus seam
        // pattern in miniature. The slit segment carries crossing
        // multiplicity 2, so the fill must NOT treat it as a boundary;
        // the whole square stays interior.
        let poly = vec![
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            [0.0, 0.5],
            [0.5, 0.5],
            [0.0, 0.5],
        ];
        let ids = vec![0_u32, 1, 2, 3, 4, 5, 4];
        let loops = vec![ids];
        let tris = triangulate_chart(FaceKey::default(), &loops, std::slice::from_ref(&poly))
            .expect("slit square triangulates");
        let total: f64 = tris
            .iter()
            .map(|&t| {
                let at = |id: u32| poly[id as usize];
                let (a, b, c) = (at(t[0]), at(t[1]), at(t[2]));
                ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])).abs()
            })
            .sum();
        assert!(
            (total - 2.0).abs() < 1e-12,
            "slit square tiled to twice-area {total}, want 2 (the full square)"
        );
    }
}

/// R2 probes for MESH-2 (PR #1421). Not part of the unit — reviewer
/// falsification attempts against the structural-zero derivation and
/// the bit-keying, kept in-module because `chart_frame`/`ChartFrame`
/// are private.
#[cfg(test)]
mod r2_mesh2_probes {
    use geom_core::{Point3, Vec3};

    const FLOOR: f64 = 1.793_662_034_335_766e-43; // spade MIN_ALLOWED_VALUE

    fn frame_of(pts: &[Point3<f64>]) -> super::ChartFrame {
        #[allow(clippy::cast_possible_truncation)]
        let outer: Vec<u32> = (0..pts.len() as u32).collect();
        super::chart_frame(&outer, pts)
    }

    /// A cheap deterministic LCG so the sweep is reproducible.
    struct Lcg(u64);
    impl Lcg {
        fn next_f64(&mut self) -> f64 {
            self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
        }
        fn range(&mut self, lo: f64, hi: f64) -> f64 {
            lo + (hi - lo) * self.next_f64()
        }
    }

    /// PROBE 1 (claim: the far point's v is an exact zero BY
    /// CONSTRUCTION on every input). If the derivation ever failed —
    /// `u` not built from the far point, or `far_at` not the point
    /// `far` came from — the raw dot product would carry a residue of
    /// the loop's own SCALE, not of its rounding. Sweep shapes, scales,
    /// noise laws and orientations; assert the raw residue is always
    /// within rounding of the terms that formed it.
    #[test]
    fn r2_far_v_residue_is_always_rounding_not_signal() {
        let mut rng = Lcg(0x5eed_1421);
        let mut worst: f64 = 0.0;
        let mut worst_case = String::new();
        for trial in 0..4000 {
            let n = 3 + (trial % 9);
            // Scale spans 1e-8 .. 1e8; noise law varies per trial.
            let scale = 10f64.powf(rng.range(-8.0, 8.0));
            let nu = 10f64.powf(rng.range(-60.0, 0.0));
            let pts: Vec<Point3<f64>> = (0..n)
                .map(|k| {
                    let t = 2.0 * core::f64::consts::PI * (k as f64) / (n as f64);
                    let r = scale * rng.range(0.3, 1.0);
                    let (x, y) = (r * t.cos(), r * t.sin());
                    // Off-plane noise, plus a tilt so the plane is not axis-aligned.
                    Point3::new(x, y, 0.25 * x + x * nu + y * nu * rng.range(-1.0, 1.0))
                })
                .collect();
            let f = frame_of(&pts);
            if !f.u_ref.x.is_finite() || !f.v_ref.x.is_finite() {
                continue; // degenerate loop: refused typed elsewhere
            }
            let w = f.far_at - f.origin;
            let raw = w.dot(f.v_ref);
            // The scale of the dot product's own rounding.
            let mag = w.norm_squared().sqrt() * f.v_ref.norm_squared().sqrt();
            let rel = if mag > 0.0 { raw.abs() / mag } else { 0.0 };
            if rel > worst {
                worst = rel;
                worst_case = format!("trial {trial} scale {scale:e} nu {nu:e} raw {raw:e} mag {mag:e}");
            }
            // The write always emits exactly 0.0 for that point.
            assert_eq!(f.project(f.far_at)[1], 0.0, "trial {trial}: far v not written");
        }
        assert!(
            worst < 1e-14,
            "far point's raw v is NOT rounding-level anywhere: worst relative {worst:e} ({worst_case}) \
             — the structural-zero derivation would be falsified"
        );
        println!("PROBE1: worst relative far-v residue over 4000 loops = {worst:e} ({worst_case})");
    }

    /// PROBE 2 (bit-keying, the direction it can MISS). The same 3-D
    /// point carried at two mesh ids whose coordinates differ only in
    /// the SIGN OF A ZERO is one point geometrically and one point to
    /// spade (`-0.0 == 0.0`), but two points to `same_position`. The
    /// write then reaches one traversal and not the other, and the one
    /// it misses keeps the sub-floor residue that refuses the face.
    #[test]
    fn r2_signed_zero_twin_of_the_far_point_is_missed_by_bit_keying() {
        let nu = 1.0e-30_f64;
        // Anchor at the origin so that a ±0.0 x-coordinate survives the
        // `p - origin` subtraction as ±0.0.
        let pts = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 2.0, 0.0 * nu),
            Point3::new(-0.0, 2.0, -0.0 * nu), // the same 3-D point, -0.0 x
            Point3::new(0.0, 1.0, 0.0),
        ];
        let f = frame_of(&pts);
        let a = f.project(pts[1]);
        let b = f.project(pts[2]);
        println!("PROBE2: far_at={:?} a={a:?} b={b:?}", f.far_at);
        println!(
            "PROBE2: same_position(p1,p2) = {}, p1 == p2 (float) = {}",
            super::same_position(pts[1], pts[2]),
            pts[1].x == pts[2].x && pts[1].y == pts[2].y && pts[1].z == pts[2].z
        );
        // Documented, not asserted as a defect: record whether the two
        // representations of one point still project alike.
        if a != b {
            println!("PROBE2: SPLIT — one 3-D point projected to two chart points");
        } else {
            println!("PROBE2: no split on this configuration");
        }
    }

    /// PROBE 3 (premise failure: far point coincident with the anchor).
    /// Every loop point equal to the anchor — `far` is the zero vector,
    /// `far_at` is the anchor, and `u_ref` is NaN. The write must not
    /// launder that into something spade accepts.
    #[test]
    fn r2_all_points_coincident_still_fails_loud() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let pts = vec![p, p, p, p];
        let f = frame_of(&pts);
        let c = f.project(p);
        println!("PROBE3: u_ref={:?} v_ref={:?} project={c:?}", f.u_ref, f.v_ref);
        assert!(
            !c[0].is_finite() || !c[1].is_finite() || (c[0] == 0.0 && c[1] == 0.0),
            "PROBE3: degenerate frame produced a spade-legal finite nonzero {c:?}"
        );
    }

    /// PROBE 4 (premise failure: degenerate normal). A collinear loop
    /// has a zero Newell sum, so `normal` is NaN. Check the written
    /// zero does not mask the NaN in the OTHER coordinate.
    #[test]
    fn r2_collinear_loop_normal_is_degenerate_and_still_refuses() {
        let pts: Vec<Point3<f64>> = (0..4)
            .map(|k| Point3::new(f64::from(k), 2.0 * f64::from(k), 3.0 * f64::from(k)))
            .collect();
        let f = frame_of(&pts);
        let c = f.project(f.far_at);
        println!("PROBE4: normal-derived u_ref={:?} project(far)={c:?}", f.u_ref);
        assert!(
            !c[0].is_finite(),
            "PROBE4: collinear loop yielded a finite u {c:?} while v was written 0.0 — \
             a degenerate chart would reach spade looking legal"
        );
    }

    /// PROBE 5 (denormal / sub-floor COORDINATES, not just residues).
    /// A loop whose own coordinates are subnormal: the far point's v is
    /// written 0.0, but nothing writes anyone else's, so the chart is
    /// still refused. Records what actually comes out.
    #[test]
    fn r2_denormal_coordinate_loop() {
        let s = 1.0e-160_f64; // squares to ~1e-320, subnormal
        let pts = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(2.0 * s, 0.0, 0.0),
            Point3::new(2.0 * s, s, 0.0),
            Point3::new(0.0, s, 0.0),
        ];
        let f = frame_of(&pts);
        for (i, p) in pts.iter().enumerate() {
            let c = f.project(*p);
            let bad = |x: f64| x != 0.0 && x.abs() < FLOOR;
            println!(
                "PROBE5: pt{i} -> {c:?}  subfloor_u={} subfloor_v={}",
                bad(c[0]),
                bad(c[1])
            );
        }
    }

    /// PROBE 6 (the write's blast radius across a whole face, not one
    /// frame). Sweep the noise band and count how many boundary points
    /// come out sub-floor in EITHER coordinate after the write. The
    /// unit's `only_the_far_points_v_is_written` checks the write is
    /// scoped; this checks the remainder is spade-legal, which is the
    /// property the face actually needs.
    #[test]
    fn r2_no_boundary_point_is_left_subfloor_across_the_band() {
        for exp in [-15i32, -20, -22, -25, -30, -40, -45, -50, -60] {
            let nu = 10f64.powi(exp);
            let pts: Vec<Point3<f64>> = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]
                .iter()
                .map(|&(x, y): &(f64, f64)| Point3::new(x, y, x * nu))
                .collect();
            let f = frame_of(&pts);
            for (i, p) in pts.iter().enumerate() {
                let c = f.project(*p);
                for (k, x) in c.iter().enumerate() {
                    assert!(
                        *x == 0.0 || x.abs() >= FLOOR,
                        "1e{exp}: point {i} coord {k} = {x:e} is sub-floor after the write"
                    );
                }
            }
        }
    }

    /// PROBE 7 (the geometric-zero corner the prose disclaims). A
    /// boundary point ON the anchor→far diagonal has an exact-zero v by
    /// geometry. Does its residue go sub-floor, i.e. is the disclaimed
    /// corner a live refusal?
    #[test]
    fn r2_the_disclaimed_diagonal_corner() {
        for exp in [-25i32, -30, -40] {
            let nu = 10f64.powi(exp);
            // Midpoint of the anchor→far diagonal inserted as a real
            // boundary point: (1, 0.5) lies on the (0,0)→(2,1) line.
            let pts: Vec<Point3<f64>> = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 0.5), (0.0, 1.0)]
                .iter()
                .map(|&(x, y): &(f64, f64)| Point3::new(x, y, x * nu))
                .collect();
            let f = frame_of(&pts);
            let c = f.project(pts[3]);
            let sub = c[1] != 0.0 && c[1].abs() < FLOOR;
            println!("PROBE7: 1e{exp} diagonal point -> {c:?}  subfloor={sub}");
        }
    }

    /// PROBE 8 (Vec3 identity the derivation rests on). `far` and the
    /// projection's `w` must be bit-identical for the far point, or the
    /// determinant argument is about a different vector than the one
    /// the code writes over.
    #[test]
    fn r2_far_vector_and_projection_w_are_bit_identical() {
        let mut rng = Lcg(0xfa2_0055);
        for _ in 0..500 {
            let n = 3 + (rng.next_f64() * 6.0) as usize;
            let pts: Vec<Point3<f64>> = (0..n)
                .map(|_| Point3::new(rng.range(-5.0, 5.0), rng.range(-5.0, 5.0), rng.range(-5.0, 5.0)))
                .collect();
            let f = frame_of(&pts);
            let w: Vec3<f64> = f.far_at - f.origin;
            let u_from_w = w.reject_from(f.v_ref.cross(f.u_ref) * -1.0);
            let _ = u_from_w;
            assert_eq!(
                (w.x.to_bits(), w.y.to_bits(), w.z.to_bits()),
                {
                    let d = f.far_at - f.origin;
                    (d.x.to_bits(), d.y.to_bits(), d.z.to_bits())
                },
                "far vector is not reproducible from far_at"
            );
        }
    }
}

/// R2 probes, part 2: the ringed-chart mechanism and the rejected
/// alternative's actual behaviour.
#[cfg(test)]
mod r2_mesh2_probes_ring {
    use geom_core::Point3;

    const FLOOR: f64 = 1.793_662_034_335_766e-43;

    /// PROBE 9 — WHICH coordinate is sub-floor on a ringed chart. Frame
    /// from the outer square; project the concentric ring, whose
    /// corners are collinear with the outer anchor→far diagonal.
    #[test]
    fn r2_ringed_chart_subfloor_census() {
        for exp in [-15i32, -20, -22, -23, -25, -30, -45] {
            let nu = 10f64.powi(exp);
            let mk = |v: &[(f64, f64)]| -> Vec<Point3<f64>> {
                v.iter()
                    .map(|&(x, y): &(f64, f64)| Point3::new(x, y, x * nu))
                    .collect()
            };
            let outer = mk(&[(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]);
            let ring = mk(&[(-0.4, -0.4), (-0.4, 0.4), (0.4, 0.4), (0.4, -0.4)]);
            #[allow(clippy::cast_possible_truncation)]
            let ids: Vec<u32> = (0..outer.len() as u32).collect();
            let f = super::chart_frame(&ids, &outer);
            let mut bad = Vec::new();
            for (tag, pts) in [("outer", &outer), ("ring", &ring)] {
                for (i, p) in pts.iter().enumerate() {
                    let c = f.project(*p);
                    for (k, x) in c.iter().enumerate() {
                        if *x != 0.0 && x.abs() < FLOOR {
                            bad.push(format!("{tag}[{i}].{} = {x:e}", if k == 0 { 'u' } else { 'v' }));
                        }
                    }
                }
            }
            println!("PROBE9: nu 1e{exp} far_at={:?} sub-floor coords: {bad:?}", f.far_at);
        }
    }

    /// PROBE 10 — the rejected alternative, run. Would a blanket
    /// `mitigate_underflow` over every chart coordinate have closed the
    /// ringed case the write leaves refusing? And does it really fail
    /// to rescue #284's class, as the siting argument claims?
    #[test]
    fn r2_blanket_mitigate_underflow_on_both_classes() {
        use spade::{
            ConstrainedDelaunayTriangulation, Point2 as SP, Triangulation, mitigate_underflow,
        };
        // (a) The ringed chart at nu = 1e-30: does mitigation make every
        //     coordinate insertable, and does the chart stay non-degenerate?
        let nu = 1.0e-30_f64;
        let mk = |v: &[(f64, f64)]| -> Vec<Point3<f64>> {
            v.iter()
                .map(|&(x, y): &(f64, f64)| Point3::new(x, y, x * nu))
                .collect()
        };
        let outer = mk(&[(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]);
        let ring = mk(&[(-0.4, -0.4), (-0.4, 0.4), (0.4, 0.4), (0.4, -0.4)]);
        #[allow(clippy::cast_possible_truncation)]
        let ids: Vec<u32> = (0..outer.len() as u32).collect();
        let f = super::chart_frame(&ids, &outer);
        let mut cdt: ConstrainedDelaunayTriangulation<SP<f64>> =
            ConstrainedDelaunayTriangulation::new();
        let mut raw_ok = true;
        let mut mit_ok = true;
        let mut seen = std::collections::HashSet::new();
        for pts in [&outer, &ring] {
            for p in pts {
                let [u, v] = f.project(*p);
                if cdt.insert(SP::new(u, v)).is_err() {
                    raw_ok = false;
                }
                let m = mitigate_underflow(SP::new(u, v));
                if spade::validate_vertex(&m).is_err() {
                    mit_ok = false;
                }
                seen.insert((m.x.to_bits(), m.y.to_bits()));
            }
        }
        println!(
            "PROBE10a: ringed chart nu=1e-30 — raw insertable={raw_ok}, \
             mitigated all-legal={mit_ok}, distinct mitigated points={} of 8",
            seen.len()
        );

        // (b) #284's class: a chart whose coordinates are ALL ~1e-67
        //     (noisy stored axes over clean positions). Mitigation snaps
        //     every one of them to zero — a single point, zero area.
        let tiny = [
            (0.0_f64, 0.0_f64),
            (2.0e-67, 0.0),
            (2.0e-67, 1.0e-67),
            (0.0, 1.0e-67),
        ];
        let mut collapsed = std::collections::HashSet::new();
        for (u, v) in tiny {
            let m = mitigate_underflow(SP::new(u, v));
            collapsed.insert((m.x.to_bits(), m.y.to_bits()));
        }
        println!(
            "PROBE10b: #284-shaped chart (~1e-67) after mitigation -> {} distinct points of 4",
            collapsed.len()
        );
        assert_eq!(
            collapsed.len(),
            1,
            "PROBE10b: mitigation did NOT collapse the #284 chart — the siting \
             argument's second leg would be false"
        );
    }
}

/// R2 probes, part 3.
#[cfg(test)]
mod r2_mesh2_probes_scoping {
    use geom_core::Point3;

    /// PROBE 11 — the PR body says `only_the_far_points_v_is_written`
    /// "is the row a blanket filter would fail". Run that: apply
    /// spade's `mitigate_underflow` to each non-far point's raw v on
    /// that row's own fixture and see whether any bit moves.
    #[test]
    fn r2_would_a_blanket_filter_fail_the_scoping_row() {
        use spade::{Point2 as SP, mitigate_underflow};
        let nu = 1.0e-30_f64;
        let positions: Vec<Point3<f64>> = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]
            .iter()
            .map(|&(x, y): &(f64, f64)| Point3::new(x, y, x * nu))
            .collect();
        #[allow(clippy::cast_possible_truncation)]
        let outer: Vec<u32> = (0..positions.len() as u32).collect();
        let f = super::chart_frame(&outer, &positions);
        let mut moved = 0;
        for (i, p) in positions.iter().enumerate() {
            if super::same_position(*p, f.far_at) {
                continue;
            }
            let raw = (*p - f.origin).dot(f.v_ref);
            let u = (*p - f.origin).dot(f.u_ref);
            let m = mitigate_underflow(SP::new(u, raw));
            let same = m.y.to_bits() == raw.to_bits() && m.x.to_bits() == u.to_bits();
            println!("PROBE11: point {i} raw v={raw:e} mitigated={:e} unchanged={same}", m.y);
            if !same {
                moved += 1;
            }
        }
        println!(
            "PROBE11: a blanket mitigate_underflow would move {moved} of the row's \
             non-far coordinates (the row asserts bit-identity, so it fails only if moved > 0)"
        );
    }
}
