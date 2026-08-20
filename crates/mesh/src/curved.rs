//! Curved-face tessellation: UV-rectangle domain from the boundary
//! walk, interior grid sampling, CDT in parameter space, pole-fan
//! collapse, per-triangle certificates.
//!
//! This lane's domain contract is the swept UV rectangle: the boundary
//! polygon from [`crate::walk`] IS its own UV bounding box, so the
//! domain is convex and no inside/outside classification is needed —
//! every CDT triangle is kept except pole-degenerate ones, and the
//! interior grid is strictly inside every boundary constraint. Every
//! sweep-authored face satisfies it; it is not a property of
//! iso-bounded input in general (a keyway is iso-bounded and is a U),
//! so it is CHECKED here rather than assumed —
//! [`TessellateError::UnsupportedCurvedDomain`], S28.
//!
//! The check is BANDED, in metres, not exact. It had to be: an iso
//! side carried by more than one edge used to be only *analytically*
//! straight, because each sub-edge derived the side's constant
//! coordinate from its own carrier point (issue #653). Exactness there
//! was a claim, it was false, and it false-refused valid parts. #653's
//! option 2 then made the claim true at the source
//! (`walk::iso_side_starts`), so the band separates nothing in tree any
//! more and is kept as a backstop with a synthetic witness — the whole
//! argument is at `entries_off_bbox`.
//!
//! Boundary polyline segments are inserted as CDT **constraints**, so
//! the triangulation conforms to the shared chord segments in both
//! adjacent faces (the watertightness guarantee).
//!
//! Pole fans: pole corners enter the CDT at their (distinct) UV
//! locations but map to the single pole mesh vertex; any triangle with
//! two corners of the same mesh vertex is degenerate in 3-D and is
//! dropped, which collapses the strip along the pole side into a fan
//! around the pole (one dropped triangle per collapsed side; its two
//! non-collapsed edges become the identified fan edges — manifoldness
//! is re-checked by the mesh validator).
//!
//! **That count — ONE dropped triangle per collapsed side — is a
//! claim about the grid, not a theorem** (issue #678). It holds only
//! while the interior grid SEPARATES the two pole entries, and
//! `nu == 2` does not: the single interior column at `u = (u0+u1)/2`
//! is equidistant from both, the CDT fans both of them over it, and
//! the identified edge `(pole, column)` is then used FOUR times —
//! silently, because `tessellate` does not run the validator.
//! [`pole_columns`] is what makes the premise true (it floors `nu` at
//! 3 on a walk carrying a pole), and a `debug_assert` over the
//! emitted patch re-derives the conclusion (D2 addendum row 5). Read
//! the sentence above as conditional on both.
//!
//! Grid sizing (heuristic; the certificates are the guarantee), from
//! δ_s = δ/2 and φ = [`crate::sizing::sagitta_step`]:
//! cylinder — hu = φ(δ_s, r), no interior rows (ruled in v);
//! cone — hu = φ(δ_s, ρ_max), rows every ρ_max·hu slant meters (ruled
//! in v, but rows keep triangles azimuth-local so the radius-scaled
//! certificate stays tight); sphere — hu = hv = φ(δ_s, r); torus —
//! hu = hv = √(δ_s/(3(R+2r))) (matching the boundary chord
//! tightening in [`crate::chords`]).

use std::collections::HashMap;

use geom::Surface;
use geom_core::Point3;
use spade::{ConstrainedDelaunayTriangulation, Point2 as SpadePoint, Triangulation};
use topo::{Body, EdgeKey, FaceKey};

use crate::cert;
use crate::sizing::{Tol, cap_angular, ceil_count, sagitta_step, torus_grid_step};
use crate::types::TessellateError;
use crate::walk::{Chart, ChartKind, UvPoint, gap_is_noise, loop_polygon};

/// Tessellates one curved face into outward-wound triangles,
/// appending interior grid points to `positions`.
pub(crate) fn tessellate_curved(
    body: &Body<f64>,
    fk: FaceKey,
    surface: &Surface<f64>,
    chords: &HashMap<EdgeKey, Vec<u32>>,
    positions: &mut Vec<Point3<f64>>,
    tol: &Tol,
) -> Result<Vec<[u32; 3]>, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    if !face.rings.is_empty() {
        return Err(TessellateError::RingOnCurvedFace { face: fk });
    }
    let chart = Chart::of(surface).ok_or(TessellateError::MissingEntity {
        what: "curved chart",
    })?;
    let polygon = loop_polygon(body, &chart, chords, positions, fk, face.outer, tol.eps)?;
    if polygon.len() < 3 {
        return Err(TessellateError::MissingEntity {
            what: "degenerate curved boundary",
        });
    }

    // Domain bbox and orientation.
    let (mut u0, mut u1, mut v0, mut v1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    let mut area2 = 0.0;
    for (i, e) in polygon.iter().enumerate() {
        u0 = u0.min(e.u);
        u1 = u1.max(e.u);
        v0 = v0.min(e.v);
        v1 = v1.max(e.v);
        let n = &polygon[(i + 1) % polygon.len()];
        area2 += e.u * n.v - n.u * e.v;
    }
    // The swept-UV-rectangle contract, CHECKED rather than assumed.
    // Everything below — and the interior grid in particular — is a
    // function of this box, not of the polygon; see
    // `require_swept_rectangle`.
    // The chart's lever arms per entry, so the check below can measure
    // in metres (`entries_off_bbox`); u's varies over the loop on a
    // cone and a sphere, which is why it reads the entry's own point.
    let levers: Vec<(f64, f64)> = polygon
        .iter()
        .map(|e| (chart.radial(positions[e.id as usize]), chart.v_lever()))
        .collect();
    require_swept_rectangle(fk, &polygon, &levers, (u0, u1, v0, v1), tol.eps)?;

    // S10 CATEGORY B — do NOT multiply by the face's `sense_sign`.
    // `area2` is the UV shoelace of the boundary walk, so its sign is
    // derived entirely from the loop's STORED TRAVERSAL order, which
    // interior-left already ties to the face's outward normal: the
    // polygon runs CCW in the chart's UV plane iff the outward normal
    // agrees with the chart normal, i.e. iff `sense`. A reversed face
    // therefore lands here with a negative `area2` and flips, which is
    // exactly right. `revert` reverses the loops AND flips `sense`
    // together, so multiplying would double-count the reversal and
    // emit inward-wound triangles. (The one place the sense IS read on
    // this path is the pole-to-pole band's azimuth in `walk` — that
    // reads a DIRECTION in the chart frame, not a winding, and the two
    // reads do not overlap.)
    let flip = area2 < 0.0;

    // The walk's chart singularity, as `grid_counts` reads it: whether
    // a pole/apex entry is present, and how many INTERIOR chord points
    // the meridian sides carry between the pole row and the opposite
    // row. Zero on a cone (its meridian is a straight ruling); always
    // several on a sphere (an arc) — which is the occlusion #678's
    // sphere-arm unreachability argument rests on, so it is measured
    // and asserted rather than assumed. `u0`/`u1` are the polygon's
    // own extremes and an iso side is bitwise straight since #653, so
    // the side test is exact.
    let has_pole = polygon.iter().any(|e| e.pole);
    let meridian_interior = polygon
        .iter()
        .filter(|e| !e.pole && e.v > v0 && e.v < v1 && (e.u <= u0 || e.u >= u1))
        .count();

    // Grid counts per kind (module docs).
    let (nu, nv) = grid_counts(
        &chart,
        tol.delta_s,
        u1 - u0,
        v1 - v0,
        v0.abs().max(v1.abs()),
        has_pole,
        meridian_interior,
    )?;

    // CDT: boundary entries (fixed walk order) + constraints + grid.
    let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
        ConstrainedDelaunayTriangulation::new();
    // Per-CDT-vertex metadata, indexed by handle index (insertion order).
    let mut meta: Vec<(f64, f64, u32, bool)> = Vec::new();
    let insert = |cdt: &mut ConstrainedDelaunayTriangulation<SpadePoint<f64>>,
                  meta: &mut Vec<(f64, f64, u32, bool)>,
                  u: f64,
                  v: f64,
                  id: u32,
                  pole: bool|
     -> Result<spade::handles::FixedVertexHandle, TessellateError> {
        let h = cdt
            .insert(SpadePoint::new(u, v))
            .map_err(|_| TessellateError::Triangulation { face: fk })?;
        if h.index() == meta.len() {
            meta.push((u, v, id, pole));
        }
        Ok(h)
    };
    let mut handles = Vec::with_capacity(polygon.len());
    for e in &polygon {
        handles.push(insert(&mut cdt, &mut meta, e.u, e.v, e.id, e.pole)?);
    }
    for i in 0..handles.len() {
        let (a, b) = (handles[i], handles[(i + 1) % handles.len()]);
        if a != b && !cdt.exists_constraint(a, b) {
            // `add_constraint` panics on crossing constraints (corrupt
            // input geometry) — pre-check to keep failure typed.
            if !cdt.can_add_constraint(a, b) {
                return Err(TessellateError::Triangulation { face: fk });
            }
            cdt.add_constraint(a, b);
        }
    }
    // GRID AFTER CONSTRAINTS, and it costs this lane nothing: the open
    // ranges below put every grid point strictly inside every
    // constraint BECAUSE `require_swept_rectangle` above has already
    // established that the boundary polygon IS this box — to within
    // the ε band it measures against, which is the honest statement of
    // what the GUARD proves. Since #653 the walk proves more: every
    // iso side gets one coordinate however many edges carry it, so the
    // polygon is its own box bitwise and the residual ulps that used to
    // make the CDT emit slivers here are gone at the source. The
    // premise, not the ordering, is what carries the argument — which
    // is why it is a refusal rather than a comment. (The hazard
    // `planar::triangulate_chart`'s header warns about is a
    // precondition of PLANAR's crossing bookkeeping, which this lane
    // does not build; S28 in `docs/SMELL-SCAN-2026-08.md` is the one
    // home for that history and for what spade does on a split.)
    let (uspan, vspan) = (u1 - u0, v1 - v0);
    for j in 1..nv {
        #[allow(clippy::cast_precision_loss)]
        let v = v0 + vspan * (j as f64 / nv as f64);
        for i in 1..nu {
            #[allow(clippy::cast_precision_loss)]
            let u = u0 + uspan * (i as f64 / nu as f64);
            let p = surface.eval(u, v);
            #[allow(clippy::cast_possible_truncation)]
            let id = positions.len() as u32;
            let h = insert(&mut cdt, &mut meta, u, v, id, false)?;
            if h.index() == meta.len() - 1 && meta[meta.len() - 1].2 == id {
                positions.push(p);
            }
        }
    }

    // Emit: drop pole-degenerate triangles, certify the rest.
    let mut triangles = Vec::new();
    let mut worst: f64 = 0.0;
    for f in cdt.inner_faces() {
        let vs = f.vertices();
        let m: Vec<(f64, f64, u32, bool)> = vs.iter().map(|v| meta[v.fix().index()]).collect();
        let ids = [m[0].2, m[1].2, m[2].2];
        if ids[0] == ids[1] || ids[1] == ids[2] || ids[0] == ids[2] {
            continue; // pole-collapsed sliver
        }
        let tri = [
            positions[ids[0] as usize],
            positions[ids[1] as usize],
            positions[ids[2] as usize],
        ];
        let uv = [[m[0].0, m[0].1], [m[1].0, m[1].1], [m[2].0, m[2].1]];
        let pole = [m[0].3, m[1].3, m[2].3];
        let bound = match chart.kind {
            ChartKind::Cylinder { r } => cert::cert_cylinder(chart.anchor, chart.axis, r, tri),
            ChartKind::Sphere { r } => cert::cert_sphere(chart.anchor, r, tri),
            ChartKind::Cone { half_angle } => cert::cert_cone(half_angle, uv, pole),
            ChartKind::Torus { major, minor } => cert::cert_torus(major, minor, uv),
        };
        // Sticky-NaN accumulation: `f64::max` would silently drop a
        // poisoned bound.
        if bound.is_nan() || worst.is_nan() || bound > worst {
            worst = bound;
        }
        triangles.push(if flip { [ids[0], ids[2], ids[1]] } else { ids });
    }

    // D2 addendum row 5 (`DESIGN.md`: kernel bug detectable only by
    // re-derivation -> `debug_assert`). The pole-fan argument in the
    // module header is structural, and the one thing that ever
    // falsified it — #678's two entries fanning over one shared
    // column — surfaced by accident, in a review of an unrelated PR.
    // Nothing else re-derives manifoldness in any build: `tessellate`
    // does not run `check_mesh`, so with `pole_columns` in place a
    // three-line arithmetic coincidence is otherwise the only thing
    // between a pole face and a silently non-manifold mesh. Re-derive
    // the conclusion here, over THIS patch's pole-incident edges only
    // (the class's whole footprint, and O(triangles)): a fan edge is
    // interior to the patch and used twice, or on its boundary and
    // used once with the neighbouring face supplying the other use.
    // Four uses is the #678 signature.
    #[cfg(debug_assertions)]
    if has_pole {
        let poles: std::collections::HashSet<u32> =
            polygon.iter().filter(|e| e.pole).map(|e| e.id).collect();
        let mut uses: HashMap<(u32, u32), usize> = HashMap::new();
        for t in &triangles {
            for k in 0..3 {
                let (a, b) = (t[k], t[(k + 1) % 3]);
                if poles.contains(&a) || poles.contains(&b) {
                    *uses.entry((a.min(b), a.max(b))).or_insert(0) += 1;
                }
            }
        }
        let over = uses.iter().find(|&(_, &n)| n > 2).map(|(&e, &n)| (e, n));
        debug_assert!(
            over.is_none(),
            "face {fk:?}: pole-fan edge {:?} used {} times in one patch \
             (nu = {nu}, nv = {nv}); the collapse-and-drop argument is \
             falsified — see curved::pole_columns, issue #678",
            over.map(|(e, _)| e),
            over.map_or(0, |(_, n)| n)
        );
    }

    if worst.is_nan() || worst > tol.delta {
        return Err(TessellateError::CertificateExceeded {
            face: fk,
            bound: worst,
            requested: tol.delta,
        });
    }
    Ok(triangles)
}

/// The walk entries that do NOT lie on the boundary of the UV bounding
/// rectangle `[u0, u1] × [v0, v1]`, each with **how far off it is in
/// metres** — empty ⟺ the domain IS that rectangle.
///
/// Sufficient, and it is the shape that matters: a rectilinear simple
/// polygon has a re-entrant corner exactly when some vertex sits
/// strictly inside its bounding box, and a re-entrant corner is what
/// lets an interior grid point land on — or across — a boundary
/// constraint.
///
/// **THE COMPARISON IS BANDED, IN METRES, AND IS NOW A BACKSTOP.** An
/// earlier form compared exactly, on the premise that [`crate::walk`]
/// makes every rectangle side bitwise straight. That premise was FALSE
/// when #648 relied on it: the walk assigned the constant coordinate
/// once per EDGE, which makes a side bitwise straight only when the
/// side IS one edge, and an iso side carried by two or more edges had
/// each sub-edge derive its own column from an `atan2` of a DIFFERENT
/// point of the same carrier — analytically equal, ulps apart under a
/// general rigid placement. A split frustum wedge, placed obliquely,
/// put an entry 6.2e-17 m off its own box and the exact form refused a
/// domain that IS the swept rectangle.
///
/// **#653 made the premise true**: `walk::iso_side_starts` groups the
/// sub-edges into runs and gives each run one coordinate, so every
/// walk this build produces sits on its box BITWISE. The band did not
/// become wrong, it became a backstop — and the honest consequence is
/// that it lost its live evidence. No in-tree fixture needs it, and
/// `a_split_then_placed_swept_face_is_not_refused` (#648's own
/// regression row) no longer discriminates banded from exact. The row
/// that does is synthetic and labelled as such:
/// `the_band_admits_a_sub_eps_entry_that_the_exact_form_refuses`.
///
/// It is kept rather than reverted because the direction of a wrong
/// answer is asymmetric here: this guard REFUSES, so an over-tight
/// comparison rejects a valid part, while an over-loose one lets
/// through a wobble six orders of magnitude smaller than the feature it
/// would have to be confused with.
///
/// The entry's distance from the box is measured through the chart's
/// own lever arms ([`Chart::radial`] for u — the point's distance from
/// the axis — and [`Chart::v_lever`] for v), against the run's `eps`,
/// by calling [`crate::walk::gap_is_noise`] — the crate's ONE ε band,
/// which the walk's three detectors also call. It used to be spelled
/// out again here; two spellings of one predicate is how the two halves
/// of a rule drift apart. The band admits ulp wobble on a straight side
/// and nothing else: a genuine re-entrant corner (a keyway, a milled
/// flat) is a FEATURE width off the box, six or more orders of
/// magnitude outside ε.
///
/// It takes the caller's own bounding box rather than recomputing one,
/// so the rectangle it checks is exactly the box the interior grid
/// spans. `levers` is parallel to `poly`.
fn entries_off_bbox(
    poly: &[UvPoint],
    levers: &[(f64, f64)],
    (u0, u1, v0, v1): (f64, f64, f64, f64),
    eps: f64,
) -> Vec<(usize, f64)> {
    poly.iter()
        .zip(levers)
        .enumerate()
        .filter_map(|(i, (e, &(lu, lv)))| {
            // Every entry is inside the box by construction (the box is
            // the entries' own min/max), so the spatial distance to the
            // box BOUNDARY is the smallest of the four side gaps, each
            // converted to metres by its axis' lever arm — so the entry
            // is ON the box iff the nearest of the four is noise. Taken
            // per axis (nearest of the two sides, then the band) so the
            // lever arm goes to the shared predicate rather than being
            // applied here: `lu`, `lv >= 0`, so per-axis and
            // all-four-at-once select the same gap either way.
            let du = (e.u - u0).abs().min((e.u - u1).abs());
            let dv = (e.v - v0).abs().min((e.v - v1).abs());
            // NaN-safe by construction: `gap_is_noise` is a `<`, so a
            // poisoned coordinate is false on both axes and the entry
            // stays a refusal rather than being admitted.
            if gap_is_noise(du, lu, eps) || gap_is_noise(dv, lv, eps) {
                None
            } else {
                Some((i, (du * lu).min(dv * lv)))
            }
        })
        .collect()
}

/// Refuses a curved face whose boundary walk is not its own UV bounding
/// rectangle — the swept-UV-rectangle contract this lane's interior
/// grid rests on, made a typed refusal.
///
/// **Why a refusal and not a comment.** The grid runs the OPEN ranges
/// `1..nu` × `1..nv` over the walk's own bounding box, which is
/// strictly interior iff the polygon IS that box. When it is not, the
/// grid splits boundary constraints and `inner_faces()` — which this
/// lane keeps wholesale, having no inside/outside classification —
/// emits triangles outside the face: a silently wrong mesh, and
/// [`fn@crate::tessellate`] does not run `check_mesh`, so it would
/// reach the caller unannounced. D2's addendum row 2 puts a
/// reachable-by-input, valid-but-unbuilt state behind a typed
/// `Unsupported*` error; D9's *"silent discard is never an answer"*
/// says the same. `trimmed` already refuses the same hazard typed
/// ([`TessellateError::SelfTouchingTrimLoop`]), and the structural
/// twin of this arm is [`TessellateError::RingOnCurvedFace`] sixty
/// lines up, whose stated reason is this very contract.
///
/// **Nothing in tree trips it** (the tests below sweep every chart this
/// build authors plus a boolean-cut face, as authored AND under a
/// general rigid placement with a multiply-carried iso side). What the
/// check buys is that the premise is enforced where it is USED: today
/// a notched iso domain is kept out of this lane only by other
/// modules' limits — the boolean refuses `CurvedPierceUnsupported`,
/// and `import_step`'s tier-3 at-rest gate refuses
/// `PropsError::NotIsoRectangle` because props' volume closed form
/// requires **the same property this check requires**, named since S58
/// as one predicate: `geom_brep::props`' `props_rim_level`, *every rim
/// at one of the face's two extreme `v`-levels*. The two lanes derive
/// it differently — `props` from rim structure, this one from the
/// walked UV polygon — but they are not testing coincidentally
/// similar things. Both of those upstream limits can still move
/// without a line changing in `mesh`; this cannot.
///
/// The check here still earns its place, because it also answers a
/// SECOND question `props` cannot (*did the walk produce a consistent
/// polygon* — #653's ulp wobble, which is why the bar is spatial);
/// folding the first question into a call on the face-level predicate
/// is the open follow-up, **issue #726**, not this unit.
///
/// **With one qualification `mesh` now owes itself.**
/// `walk::iso_side_starts` (#653) is a line IN `mesh` that can defeat
/// this check, because it collapses consecutive same-kind traversals
/// onto one coordinate on a premise `walk::classify` never verifies:
/// that every boundary edge is an iso-curve of the chart. Where that
/// premise fails — an obliquely-cut SPHERE face, whose every plane
/// section is a `Circle` and so is not diverted to the trimmed lane —
/// the collapse can turn a polygon this guard would have REFUSED into
/// one that is its own bounding rectangle and is admitted. Two
/// upstream gates keep it unreachable today (`topo::boolean` refuses
/// the tilted plane × sphere section typed; `import_step`'s tier-3
/// `props::curved::sphere_boundary` admits only coaxial rims and
/// centre-centred great circles), which is the same shape as the
/// sentence above rather than an exception to it. Stated in full,
/// with what would harden it, at `walk::iso_side_starts`.
///
/// **The bar is spatial** ([`entries_off_bbox`]): comparing exactly
/// produced FALSE REFUSALS on bodies whose domain is the swept
/// rectangle to within an ulp (issue #653).
///
/// **#653 removed the band's live evidence, and the band is kept
/// anyway.** `walk` now assigns one coordinate per ISO SIDE rather than
/// per edge, so every walk entry this build produces sits on its box
/// **bitwise**: the banded and exact forms agree everywhere the suite
/// looks, and what the band guards is asserted directly upstream by
/// `a_multiply_carried_iso_side_is_bitwise_straight_and_meshes_watertight`
/// (`== 0.0`). So the band's own red-when-reverted row had to be
/// SYNTHETIC —
/// `the_band_admits_a_sub_eps_entry_that_the_exact_form_refuses` — and
/// [`entries_off_bbox`] states why keeping it is the right call. What a
/// reader must NOT do is read
/// `a_split_then_placed_swept_face_is_not_refused` as evidence about
/// the band: since #653 that row is evidence about the invariant.
///
/// **A class note, because this fix invalidated more than one
/// sentence** (§C13's `face_box` precedent). Every doc in `crates/mesh`
/// that argued a tolerance from a MEASURED number measured a
/// population #653 has eliminated, and no convention covers those
/// sentences. The two that argued from a NUMBER were
/// `entries_off_bbox`'s 1.4985e-15 m and `the_band_separates_…`'s
/// hardcoded copy of it; both now derive their live half from the tree
/// and label the sweep figure as historical. **The rule: a measured
/// constant in this crate is re-derivable from the tree, or it says it
/// is not.**
///
/// The sweep for the weaker form — prose asserting the per-edge
/// premise without a number — took five more sites, in three files:
/// this module's header, the interior-grid comment in
/// [`tessellate_curved`], [`entries_off_bbox`],
/// `TessellateError::UnsupportedCurvedDomain`'s doc in `types.rs`, and
/// `mesh/lib.rs`'s crate header. Only two of those are in the file the
/// guard lives in, which is the §C10 point: a claim lives wherever it
/// was written down.
fn require_swept_rectangle(
    fk: FaceKey,
    poly: &[UvPoint],
    levers: &[(f64, f64)],
    bbox: (f64, f64, f64, f64),
    eps: f64,
) -> Result<(), TessellateError> {
    let off = entries_off_bbox(poly, levers, bbox, eps);
    let Some(&(first, _)) = off.first() else {
        return Ok(());
    };
    // The payload has to separate "re-author your part" from "kernel
    // bug, file it", and a COUNT cannot: `off_bbox: 1` is what a
    // one-corner keyway reports and what a 6e-17 m wobble reported.
    // The distance is the number that tells them apart (S19's
    // postmortem lesson — the day a refusal fires is the day someone
    // needs its payload), and the (u, v) says where to look. Had the
    // sweep that "proved" the exact form unreachable printed margins
    // instead of pass/fail, the exactness claim would have been
    // visibly fragile before it shipped.
    Err(TessellateError::UnsupportedCurvedDomain {
        face: fk,
        off_bbox: off.len(),
        first_uv: (poly[first].u, poly[first].v),
        max_distance: off.iter().fold(0.0_f64, |m, &(_, d)| {
            if d.is_nan() || m.is_nan() || d > m {
                d
            } else {
                m
            }
        }),
    })
}

/// The pole-fan separation rule on the u column count (issue #678).
///
/// The module header's fan argument — one dropped triangle per
/// collapsed side — needs the interior grid to SEPARATE the walk's two
/// pole entries, and `nu == 2` is the one column count that does not:
/// the single interior column at `u = (u0 + u1)/2` is equidistant from
/// both, the CDT fans both over its upper half, and the identified
/// mesh edge `(pole, column)` is used FOUR times. `check_mesh` reports
/// `NonManifoldEdge`; `tessellate`, which does not run it, returns
/// that mesh as `Ok`.
///
/// `nu == 2` is singular, which is why this is an equality and not
/// `max(3)`. At `nu == 1` the inner grid range `1..nu` is empty, so
/// the entries fan over the BOUNDARY walk, which is ordered along the
/// rim and splits between them at one shared vertex. At `nu >= 3` each
/// entry's nearest column occludes the other's.
///
/// # Corrective on the cone, prophylactic on the sphere
///
/// A/B over 281 public-API configurations (cone wedges, sphere lunes,
/// sphere caps): **7 rows go dirty -> clean, every one of them cone**;
/// 57 clean rows re-size, **49 of them sphere**; and **no sphere row
/// has ever been measured dirty**, out of ~200.
///
/// **Cone — corrective, and genuinely clean-by-luck when it is clean.**
/// A cone's meridian is a straight RULING, so it carries ZERO interior
/// chord points (`npoly = 5` = 2 apex + 3 rim) and nothing sits between
/// an apex entry and the rim. `nv` alone then decides: `nu == 2` is
/// clean at `nv <= 3` — too few interior rows for the overlap to
/// contain an edge — and dirty at `nv >= 4`. `cone_wedge(1, pi/4)` is
/// clean at delta = 0.1 (`nv = 3`) and dirty at delta = 0.05
/// (`nv = 4`): one delta-step from the defect.
///
/// **Sphere — prophylactic, and NOT clean by luck.** The same reading
/// is false here, and a single row falsifies it: `sphere_wedge(pi/4)`
/// at delta = 0.05 is `nu = 2, nv = 8` — seven interior column
/// vertices, an overlap that could hold six edges — and it is clean. A
/// sphere's meridian is an ARC and always carries interior chord
/// points (lune `npoly = 10`, cap `npoly = 6`); those points occlude
/// the cross-fan. Structural, not lucky: dirty needs `nv >= 3`, and
/// the meridian's chord step `phi(delta_s, r)` and the grid's
/// `phi(delta_s / SPHERE_SIZING_MARGIN, r)` are never more than ~12%
/// apart with both capped, so `nv >= 3` FORCES at least two interior
/// meridian points. The sphere arm is kept anyway (Evan, 2026-08-19) so the
/// rule reads as one sentence rather than one chart but not the other
/// — and what it rests on is CHECKED, not just written down: see the
/// `debug_assert` in [`grid_counts`]'s sphere arm, which asserts the
/// mechanism (the occluding points exist) rather than the conclusion.
///
/// # The option this doc used to exclude by silence
///
/// The choice here is three-way, and an earlier draft of this comment
/// offered two — floor-as-written versus per-face manifoldness
/// re-derivation — which makes the missing one look unconsidered
/// rather than rejected. It is `nu == 2 && nv >= 3`: available right
/// here, since both counts are in the same arm, and it would preserve
/// all eight cone re-sizings. Not taken, for two reasons — it makes
/// `nu` a function of `nv`, so the schedule's two axes stop being
/// independent, and it leaves the `ceil` knife edge live at `nv <= 3`
/// instead of removing the class. The remaining option, per-face
/// manifoldness re-derivation over the emitted patch, is the
/// D2-addendum row-5
/// mechanism (`DESIGN.md`'s row 5 says `debug_assert`, not "make it
/// unreachable"), which now ships BESIDE this floor in
/// [`tessellate_curved`]'s emit pass rather than instead of it.
///
/// # Blast radius, and #678's two open questions
///
/// Only pole faces with `nu == 2` re-size. A full revolve can never be
/// one: [`sagitta_step`] hard-caps at
/// [`crate::sizing::MAX_ANGULAR_STEP`] on both branches, and
/// [`torus_grid_step`] is capped against the same value here, so a
/// `2*pi` span gives `nu >= 8` — confirmed in the A/B, where no
/// full-revolve face appears with `nu <= 2`.
///
/// `Fixes #678` closes the only other home of that issue's two open
/// questions, so both answers live here. *Does the sphere lane reach
/// the defect in practice?* **No** — ~200 configurations, including
/// the cap shape the issue named and nobody had built. *Is any corpus
/// body in the changed class?* **No** — `demos/renders`,
/// `demos/renders-wild` and `demos/renders-freecad` all report zero
/// drift.
fn pole_columns(nu: usize, has_pole: bool) -> usize {
    if has_pole && nu == 2 { 3 } else { nu }
}

/// The sphere arm's extra sizing margin: it sizes at δ_s divided by
/// this rather than at δ_s itself.
///
/// Near the equator a full-step grid triangle's true deviation
/// approaches 2·δ_s = δ from below, so sizing at exactly δ_s would
/// lean on [`ceil_count`]'s step-shrink (`span/⌈span/h⌉ < h`) as the
/// only slack. The margin buys real headroom cheaply (≈12% more counts
/// per axis) and keeps a future sizing change from silently landing on
/// the certificate boundary; the certificate remains the backstop.
const SPHERE_SIZING_MARGIN: f64 = 1.25;

/// The interior grid counts `(nu, nv)` for the face's UV spans.
///
/// `has_pole` and `meridian_interior` describe the boundary walk's
/// chart singularity: whether it carries a pole/apex entry at all, and
/// how many interior chord points its meridian sides carry between the
/// pole row and the opposite row ([`pole_columns`]). Only the cone and
/// sphere arms read them, and that is STRUCTURAL rather than an
/// oversight — [`Chart::poles`] returns an empty vector for a cylinder
/// and for a torus, so `has_pole` is false on those two arms for every
/// input there is, and the `debug_assert`s below say so in a form that
/// fails if it ever stops being true.
fn grid_counts(
    chart: &Chart,
    delta_s: f64,
    uspan: f64,
    vspan: f64,
    v_absmax: f64,
    has_pole: bool,
    meridian_interior: usize,
) -> Result<(usize, usize), TessellateError> {
    match chart.kind {
        ChartKind::Cylinder { r } => {
            debug_assert!(!has_pole, "Chart::poles() is empty for a cylinder");
            let hu = sagitta_step(delta_s, r);
            Ok((ceil_count(uspan, hu)?, 1))
        }
        ChartKind::Cone { half_angle } => {
            let rho_max = v_absmax * half_angle.sin();
            let hu = sagitta_step(delta_s, rho_max);
            let hv = rho_max * hu;
            Ok((
                pole_columns(ceil_count(uspan, hu)?, has_pole),
                ceil_count(vspan, hv)?,
            ))
        }
        ChartKind::Sphere { r } => {
            let h = sagitta_step(delta_s / SPHERE_SIZING_MARGIN, r);
            let (nu, nv) = (ceil_count(uspan, h)?, ceil_count(vspan, h)?);
            // D2 addendum row 5 — and the whole warrant for calling
            // this arm PROPHYLACTIC rather than corrective
            // (`pole_columns`). Assert the MECHANISM, not the
            // conclusion: a sphere pole face that reaches `nu == 2`
            // with enough rows to be dirty (`nv >= 3`) is claimed to
            // be saved by the interior chord points its ARC meridians
            // always carry. If one ever arrives without them, the
            // unreachability argument is false — and we want that
            // from this line rather than from another accident, since
            // the class was found by accident once already.
            debug_assert!(
                !(has_pole && nu == 2) || nv < 3 || meridian_interior >= 2,
                "sphere pole face at nu == 2, nv = {nv} carries \
                 {meridian_interior} interior meridian chord points; \
                 #678's unreachability argument needs >= 2 (one per \
                 meridian side)"
            );
            Ok((pole_columns(nu, has_pole), nv))
        }
        ChartKind::Torus { major, minor } => {
            debug_assert!(!has_pole, "Chart::poles() is empty for a torus");
            let h = cap_angular(torus_grid_step(delta_s, major, minor));
            Ok((ceil_count(uspan, h)?, ceil_count(vspan, h)?))
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! **S28 — the swept-UV-rectangle premise this lane's grid rests
    //! on, pinned by the refusal that now enforces it.**
    //!
    //! The history (why `planar`'s ordering warning was never a claim
    //! about this lane, and what spade actually does on a split) has
    //! one home: S28 in `docs/SMELL-SCAN-2026-08.md`. These rows pin
    //! facts, not prose — the two spade behaviours the inertness
    //! argument rests on, the refusal itself, and the sweep showing
    //! nothing this build authors trips it.

    use super::*;
    use geom_core::{Affine3, Point2, Tolerance, Vec2, Vec3};
    use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane, ValidatedProfile};
    use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};

    fn p2(x: f64, y: f64) -> Point2<f64> {
        Point2::new(x, y)
    }

    fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
        Profile::new(SketchPlane::xy(), loops)
            .validate(Tolerance::get())
            .unwrap()
    }

    fn axis_y() -> RevolveAxis<f64> {
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        }
    }

    /// The UV bounding box of a walk polygon — the same `(u0, u1, v0,
    /// v1)` [`tessellate_curved`] derives, so a row that feeds a
    /// synthetic polygon to [`require_swept_rectangle`] checks it
    /// against the box the real grid would span.
    fn bbox(poly: &[UvPoint]) -> (f64, f64, f64, f64) {
        let (mut u0, mut u1, mut v0, mut v1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        for e in poly {
            u0 = u0.min(e.u);
            u1 = u1.max(e.u);
            v0 = v0.min(e.v);
            v1 = v1.max(e.v);
        }
        (u0, u1, v0, v1)
    }

    /// The run's kernel ε — the length this lane's band measures
    /// against, read from the same place `tessellate` reads it.
    fn eps() -> f64 {
        Tolerance::get().eps
    }

    /// Unit lever arms for a synthetic polygon, so its UV units ARE
    /// metres and the band's margin can be read straight off the
    /// fixture's coordinates.
    fn unit_levers(n: usize) -> Vec<(f64, f64)> {
        vec![(1.0, 1.0); n]
    }

    fn uv(poly: &[(f64, f64)]) -> Vec<UvPoint> {
        poly.iter()
            .enumerate()
            .map(|(i, &(u, v))| UvPoint {
                u,
                v,
                #[allow(clippy::cast_possible_truncation)]
                id: i as u32,
                pole: false,
            })
            .collect()
    }

    /// One face's walk as the rows below consume it: the face, its UV
    /// polygon, and the per-entry lever arms `tessellate_curved` builds.
    type Walked = (FaceKey, Vec<UvPoint>, Vec<(f64, f64)>);

    /// Every face this lane would take, walked to its UV polygon — the
    /// `tessellate` prologue (mesh ids, then the chord pass) run for
    /// the walk alone. The two `continue`s below are exactly the
    /// router's own screens (`tessellate.rs`), so a face that survives
    /// them is a face `tessellate_curved` receives.
    fn curved_walks(body: &Body<f64>) -> Vec<Walked> {
        let eps = Tolerance::get().eps;
        let mut positions = Vec::new();
        let mut vids = HashMap::new();
        for (vk, v) in body.vertices() {
            #[allow(clippy::cast_possible_truncation)]
            vids.insert(vk, positions.len() as u32);
            positions.push(*body.get_point(v.point).unwrap());
        }
        let chords = crate::chords::compute_chords(
            body,
            0.025,
            &vids,
            &mut positions,
            &mut crate::nurbs_cert::FaceBounds::new(),
        )
        .unwrap()
        .ids;
        let mut out = Vec::new();
        for (fk, face) in body.faces() {
            if crate::trimmed::has_trim_carrier(body, fk).unwrap() {
                continue;
            }
            let Some(chart) = Chart::of(body.get_surface(face.surface).unwrap()) else {
                continue;
            };
            let poly =
                loop_polygon(body, &chart, &chords, &positions, fk, face.outer, eps).unwrap();
            // The same lever arms `tessellate_curved` builds.
            let levers = poly
                .iter()
                .map(|e| (chart.radial(positions[e.id as usize]), chart.v_lever()))
                .collect();
            out.push((fk, poly, levers));
        }
        out
    }

    fn ball() -> Body<f64> {
        let lp = ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -1.0), 1.0),
            ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
            .unwrap()
            .body
    }

    /// A partial-revolve sphere band — the pole-to-pole wedge whose
    /// walk is hardest (`revolves.rs`'s `survives_sphere_wedges_...`
    /// shape), absent from the sweep until the review asked for it.
    fn sphere_band(theta: f64) -> Body<f64> {
        let lp = ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -1.0), 1.0),
            ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Partial(theta))
            .unwrap()
            .body
    }

    fn cone_body() -> Body<f64> {
        let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
            .unwrap()
            .body
    }

    fn washer() -> Body<f64> {
        let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
            .unwrap()
            .body
    }

    fn donut() -> Body<f64> {
        let lp = ProfileLoop::new(vec![
            ProfileVertex::new(p2(2.0, -0.5), 1.0),
            ProfileVertex::new(p2(2.0, 0.5), 1.0),
        ]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
            .unwrap()
            .body
    }

    fn wedge(theta: f64) -> Body<f64> {
        let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Partial(theta))
            .unwrap()
            .body
    }

    /// Axis-touching partial wedge (`tests/common::axis_wedge`): the
    /// axis edge is an ordinary boundary edge shared by the two caps,
    /// which is one of the two shapes an earlier review lane built
    /// *because the walk is hardest there*.
    fn axis_wedge(theta: f64) -> Body<f64> {
        let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Partial(theta))
            .unwrap()
            .body
    }

    /// The mirror-nappe diamond (`review_m2_pr6_walk_shapes.rs`'s
    /// `diamond_profile`): a downward-opening cone under a partial
    /// revolve, so the nappe walls carry junction u/v assignments —
    /// the other hardest-walk shape.
    fn mirror_nappe(theta: f64) -> Body<f64> {
        let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 1.0), p2(1.0, 2.0)]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Partial(theta))
            .unwrap()
            .body
    }

    fn rounded_prism() -> Body<f64> {
        let b = core::f64::consts::FRAC_PI_8.tan();
        let r = 0.5;
        let v = |pos: Point2<f64>, bulge: f64| ProfileVertex::new(pos, bulge);
        let mut lp = ProfileLoop::new(vec![
            v(p2(r, 0.0), 0.0),
            v(p2(2.0 - r, 0.0), b),
            v(p2(2.0, r), 0.0),
            v(p2(2.0, 2.0 - r), b),
            v(p2(2.0 - r, 2.0), 0.0),
            v(p2(r, 2.0), b),
            v(p2(0.0, 2.0 - r), 0.0),
            v(p2(0.0, r), b),
        ]);
        let n = lp.vertices().len();
        lp = lp.with_tangent_joints((0..n).collect());
        extrude(&validated(vec![lp]), Extrusion::Distance(1.0))
            .unwrap()
            .body
    }

    /// The die pip (`sweep`'s `m5_s13_pips` shape): a 4 × 4 × 1 slab
    /// with a radius-0.5 ball subtracted 0.2 above its top face. The
    /// cavity's two sphere faces are the only curved faces in this
    /// crate's reach that a BOOLEAN produced rather than a sweep.
    fn die_pip() -> Body<f64> {
        let lp = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
            p2(0.0, 0.0),
            p2(4.0, 0.0),
            p2(4.0, 4.0),
            p2(0.0, 4.0),
        ]);
        let slab = extrude(&validated(vec![lp]), Extrusion::Distance(1.0))
            .unwrap()
            .body;
        let half = ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -0.5), 1.0),
            ProfileVertex::new(p2(0.0, 0.5), 0.0),
        ]);
        let ball = revolve(&validated(vec![half]), axis_y(), Revolution::Full)
            .unwrap()
            .body;
        let ball =
            topo::transform_rigid(&ball, &Affine3::translation(Vec3::new(2.0, 2.0, 1.2))).unwrap();
        topo::boolean::subtract(&slab, &ball)
            .expect("the die pip cuts")
            .body()
            .expect("a pip is a dent, not a void")
            .body
            .clone()
    }

    fn fixtures() -> Vec<(&'static str, Body<f64>)> {
        let pi = core::f64::consts::PI;
        vec![
            ("ball", ball()),
            ("sphere band (pi/2)", sphere_band(pi / 2.0)),
            (
                "sphere band (2pi - 0.08)",
                sphere_band(core::f64::consts::TAU - 0.08),
            ),
            ("cone", cone_body()),
            ("mirror nappe (pi + 0.05)", mirror_nappe(pi + 0.05)),
            ("washer", washer()),
            ("donut", donut()),
            ("wedge(pi/2)", wedge(pi / 2.0)),
            ("wedge(pi)", wedge(pi)),
            ("wedge(2pi - 0.05)", wedge(core::f64::consts::TAU - 0.05)),
            ("axis wedge(pi/2)", axis_wedge(pi / 2.0)),
            ("axis wedge(pi + 0.5)", axis_wedge(pi + 0.5)),
            ("rounded prism", rounded_prism()),
            ("die pip (boolean cut)", die_pip()),
        ]
    }

    /// Every edge of `body` split at `fracs` of its own parameter
    /// range, then the whole body placed by an oblique rigid map —
    /// #653's two ingredients, applied to a fixture wholesale.
    ///
    /// One body per (edge, fracs) pair, because splitting every edge of
    /// one body at once would also change which faces are adjacent to
    /// which and stop being a controlled comparison.
    ///
    /// Returns `(placed bodies, skipped edges)`. **The skips are
    /// returned rather than `continue`d silently**: an edge that drops
    /// out here is an edge the row above never covers, and a helper
    /// that swallows them lets a fixture stop contributing without any
    /// row going red. The caller asserts the skip list is empty.
    fn split_each_edge_then_place(
        body: &Body<f64>,
        fracs: &[f64],
    ) -> (Vec<(usize, Body<f64>)>, Vec<String>) {
        // Deliberately irrational-ish (see `split_and_placed_frustum_wedge`):
        // an axis-aligned or dyadic placement keeps the sub-edge
        // azimuths bitwise equal and the rows below go green for the
        // wrong reason.
        let irr = Vec3::new(0.317_8_f64, 0.941_2, -0.110_9);
        let irr = irr * (1.0 / irr.norm());
        let placement = Affine3::from_parts(
            Affine3::rotation_about_axis(Point3::origin(), irr, 1.0 / 3.0).linear,
            Vec3::new(0.117, -0.339_1, 5.001_7),
        );
        let (mut out, mut skipped) = (Vec::new(), Vec::new());
        for (i, ek) in body.edges().map(|(k, _)| k).enumerate() {
            let Some(curve) = body
                .get_edge(ek)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(|g| g.certified())
            else {
                skipped.push(format!("edge {i}: no certified curve geometry"));
                continue;
            };
            let (t0, t1) = curve.params();
            let mut b = body.clone();
            let mut failed = None;
            for &f in fracs {
                // Not `ok &= …`: the second split of a failed first is
                // meaningless, and the first failure is the reportable
                // one.
                if let Err(e) = b.split_edge(ek, t0 + (t1 - t0) * f) {
                    failed = Some(format!("edge {i} @{f}: split_edge {e:?}"));
                    break;
                }
            }
            if let Some(why) = failed {
                skipped.push(why);
                continue;
            }
            match topo::transform_rigid(&b, &placement) {
                Ok(placed) => out.push((i, placed)),
                Err(e) => skipped.push(format!("edge {i}: transform_rigid {e:?}")),
            }
        }
        (out, skipped)
    }

    /// The largest distance, in metres, from any walk entry to its own
    /// face's UV bounding box, over every curved face of `body`.
    ///
    /// **It calls [`entries_off_bbox`] with `eps = 0.0` rather than
    /// re-deriving the distance.** `gap_is_noise` is a strict `<`, so a
    /// zero band admits nothing and every entry comes back carrying the
    /// distance production measures, in production's own per-axis
    /// spelling. There is one spelling of this metric in the crate and
    /// it is the one that ships; a hand transcription in a test goes
    /// stale the moment the kernel's changes.
    fn worst_entry_off_box(body: &Body<f64>) -> f64 {
        let mut worst: f64 = 0.0;
        for (_, poly, levers) in curved_walks(body) {
            for (_, d) in entries_off_bbox(&poly, &levers, bbox(&poly), 0.0) {
                worst = worst.max(d);
            }
        }
        worst
    }

    /// **THE #653 ROW.** An iso side carried by several edges is
    /// bitwise straight, and the mesh over it is watertight — for
    /// every chart class, under an oblique placement, with every edge
    /// of every fixture split in turn.
    ///
    /// **The assertion is `== 0.0`, and that is the point.** Before
    /// this fix each sub-edge derived its own constant coordinate from
    /// its own `mid_azimuth` point, so the side's entries landed
    /// ULPS apart and the polygon was not its own bounding rectangle
    /// to the last bit. Under an oblique placement that produced two
    /// consequences: the exact form of [`entries_off_bbox`] refused
    /// valid parts (the row above), and — the live defect — the CDT
    /// saw a sliver-generating wobble and emitted a **silently
    /// non-watertight** mesh, `NonManifoldEdge` under
    /// [`crate::validate::check_mesh`] while `tessellate` returned
    /// `Ok`. A banded margin cannot separate those from the correct
    /// case (measured: the two populations interleave at ~1e-16 m), so
    /// the columns are fixed at the source instead and the residue is
    /// **identically zero**, asserted here rather than inferred.
    ///
    /// Reverting `walk::iso_side_starts` to `vec![true; m]` turns both
    /// halves red.
    ///
    /// # The floor is PER FIXTURE, and derived, not pinned
    ///
    /// `every_curved_walk_is_its_own_bounding_rectangle` two hundred
    /// lines down argues the case (§C10): a global floor lets a fixture
    /// that stopped contributing hide behind its siblings. So this row
    /// asserts, per fixture and per split pattern, that the number of
    /// placed bodies EQUALS the fixture's edge count — every edge
    /// participates — and that `split_each_edge_then_place` skipped
    /// nothing. That floor derives itself from the fixture list, so
    /// adding a fixture cannot silently shrink the sweep, which a
    /// transcribed total (§C14) cannot promise.
    ///
    /// The totals are still REPORTED (in the failure message and by
    /// the two constants below) because a reader wants the scale; they
    /// are not what the row rests on.
    ///
    /// # What this row cannot see (§C15)
    ///
    /// - **Trim-carrier faces.** `curved_walks` skips them exactly as
    ///   the router does, and `crate::trimmed` runs its own walk. An
    ///   iso side carried by several edges on a TRIMMED face is not
    ///   covered here or anywhere.
    /// - **Bodies `revolve` refuses at construction** — horn and
    ///   spindle tori, and every profile that fails its checks. The
    ///   chart classes below are the constructible ones, which is not
    ///   the same as all of them.
    /// - **Rim runs from DISTINCT carrier circles.** This is the real
    ///   gap. `split_edge` keeps ONE carrier, so the two sub-edges of a
    ///   split rim read the same centre and radius and agree bitwise
    ///   even WITHOUT the fix: the Rim arm of `iso_side_starts` is
    ///   executed on every rim split here and can never differ. So the
    ///   swept sibling ships with no red-when-reverted evidence — only
    ///   the meridian half has any. What would mint it is two
    ///   independently stated co-`v` `CIRCLE`s, which is a STEP
    ///   authoring job (the fixture generator in
    ///   `crates/step-import/tests/fixtures/split-iso/` is the obvious
    ///   place); §C10's sweep entry records it as scheduled work.
    #[test]
    fn a_multiply_carried_iso_side_is_bitwise_straight_and_meshes_watertight() {
        // Single and double splits: two sub-edges and three, the second
        // being what `split_and_placed_frustum_wedge` mints.
        const PATTERNS: [&[f64]; 2] = [&[0.5], &[0.312_9, 0.156_45]];
        let (mut checked, mut refused) = (0usize, 0usize);
        let (mut crooked, mut dirty): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
        for (name, body) in fixtures() {
            let edges = body.edges().count();
            let mut here = 0usize;
            for fracs in PATTERNS {
                let (placed, skipped) = split_each_edge_then_place(&body, fracs);
                assert!(
                    skipped.is_empty(),
                    "{name} @{fracs:?}: the sweep dropped edges silently — {skipped:?}"
                );
                assert_eq!(
                    placed.len(),
                    edges,
                    "{name} @{fracs:?}: every one of the fixture's {edges} edges must \
                     produce a split, placed body"
                );
                for (i, placed) in placed {
                    let worst = worst_entry_off_box(&placed);
                    if worst != 0.0 {
                        crooked.push(format!("{name} edge {i} @{fracs:?}: {worst} m"));
                    }
                    // A typed refusal is not a counterexample: a few of
                    // these exceed the chord certificate at this δ and
                    // refuse identically before and after. What must
                    // never happen is `Ok` plus a dirty mesh — the #653
                    // defect's signature.
                    match crate::tessellate(&placed, 0.1) {
                        Ok(mesh) => {
                            checked += 1;
                            here += 1;
                            if let Err(e) = crate::validate::check_mesh(&mesh) {
                                dirty.push(format!("{name} edge {i} @{fracs:?}: {e:?}"));
                            }
                        }
                        Err(_) => refused += 1,
                    }
                }
            }
            assert!(
                here > 0,
                "{name} contributed no meshed configuration at all — it is in the \
                 fixture list but exercises nothing"
            );
        }
        // Both halves reported together: reverting the fix makes the
        // first list long and the second non-empty, and a reader of the
        // failure should see the whole population, not its first member.
        assert!(
            crooked.is_empty() && dirty.is_empty(),
            "of {checked} meshed configurations ({refused} refused typed): \
             {} have a walk entry off their own UV box {crooked:?}; \
             {} meshed non-watertight {dirty:?}",
            crooked.len(),
            dirty.len()
        );
        // Reported, not rested on: the per-fixture floors above are
        // what fail when the sweep shrinks. This line exists so the
        // scale is visible without running the row.
        assert_eq!(
            (checked, refused),
            (TOTAL_MESHED, TOTAL_REFUSED),
            "the sweep's totals moved; if a fixture was added or removed this is the \
             one line to update, and the per-fixture floors above are the guarantee"
        );
    }

    /// The #653 row's totals, measured. They are asserted so that a
    /// change in the fixture list is VISIBLE rather than silent — the
    /// row's actual guarantee is its per-fixture floor, not these.
    const TOTAL_MESHED: usize = 254;
    /// Typed refusals in the same sweep: `CertificateExceeded` on
    /// bodies whose split geometry exceeds the chord certificate at
    /// δ = 0.1, identically before and after #653.
    const TOTAL_REFUSED: usize = 4;

    /// **The premise, swept.** Every curved face this build can put in
    /// front of [`tessellate_curved`] walks to a UV polygon that IS its
    /// own bounding rectangle, so [`require_swept_rectangle`] admits it
    /// — i.e. the guard this lane now carries refuses nothing in tree.
    ///
    /// Participation is asserted **per fixture**, not as a global
    /// count: `curved_walks` reaches its body through two silent
    /// `continue`s (`has_trim_carrier`, `Chart::of → None`), so a
    /// global floor lets a fixture that stopped contributing hide
    /// behind its siblings' faces. The die pip is the one this matters
    /// most for (§C10, the sweep this finding asks for): the boolean's
    /// chart re-cut is what keeps a CUT sphere face iso-rectangular,
    /// and if a future re-cut gave the cavity rim an ellipse carrier it
    /// would drop out of the sweep silently.
    #[test]
    fn every_curved_walk_is_its_own_bounding_rectangle() {
        for (name, body) in fixtures() {
            let walks = curved_walks(&body);
            assert!(
                !walks.is_empty(),
                "{name} contributes no curved walk — it is no longer sweeping this lane"
            );
            for (fk, poly, levers) in walks {
                // Asserted through the production refusal, with the
                // predicate consulted only for the diagnostic.
                assert_eq!(
                    require_swept_rectangle(fk, &poly, &levers, bbox(&poly), eps()),
                    Ok(()),
                    "{name} face {fk:?}: of {} walk entries, these lie strictly inside \
                     the UV bounding box (uv, metres off): {:?}",
                    poly.len(),
                    entries_off_bbox(&poly, &levers, bbox(&poly), eps())
                        .iter()
                        .map(|&(i, d)| ((poly[i].u, poly[i].v), d))
                        .collect::<Vec<_>>()
                );
            }
        }
    }

    /// The whole pipeline agrees: no fixture refuses. A face-level
    /// walk check could pass while `tessellate` failed for an unrelated
    /// reason, and the guard is new code on the hot path — this row is
    /// the in-tree regression evidence itself.
    #[test]
    fn no_fixture_refuses_through_the_public_entry_point() {
        for (name, body) in fixtures() {
            for delta in [0.25, 0.04] {
                let mesh = crate::tessellate(&body, delta)
                    .unwrap_or_else(|e| panic!("{name} at delta={delta} refused: {e:?}"));
                crate::validate::check_mesh(&mesh)
                    .unwrap_or_else(|e| panic!("{name} at delta={delta} meshed dirty: {e:?}"));
            }
        }
    }

    /// The U-shaped domain the red rows use: `[0, 4] × [0, 4]` with
    /// `[1, 3] × [2, 4]` bitten out of the top — every side iso, so
    /// nothing upstream of this lane would reroute it.
    fn notched_domain() -> Vec<(f64, f64)> {
        vec![
            (0.0, 0.0),
            (4.0, 0.0),
            (4.0, 4.0),
            (3.0, 4.0),
            (3.0, 2.0),
            (1.0, 2.0),
            (1.0, 4.0),
            (0.0, 4.0),
        ]
    }

    fn rectangle_domain() -> Vec<(f64, f64)> {
        vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]
    }

    /// **The refusal.** A notched iso domain is refused TYPED, naming
    /// both re-entrant corners, where they are, and how far off the box
    /// they sit; the swept rectangle passes. Asserting the production
    /// refusal rather than a predicate's return value is what makes
    /// this a guarantee about the lane instead of about a helper.
    ///
    /// The fixture is a synthetic polygon because no public
    /// construction mints such a body: the boolean refuses
    /// `CurvedPierceUnsupported`, and `import_step`'s tier-3 gate
    /// refuses `PropsError::NotIsoRectangle` before adoption (S28).
    /// That is precisely why the guard is here — the mesher is
    /// otherwise protected only by other modules' limits.
    #[test]
    fn a_notched_domain_is_refused_typed() {
        let fk = fixtures()
            .into_iter()
            .next()
            .and_then(|(_, b)| b.faces().next().map(|(fk, _)| fk))
            .unwrap();
        let rect = uv(&rectangle_domain());
        assert_eq!(
            require_swept_rectangle(fk, &rect, &unit_levers(rect.len()), bbox(&rect), eps()),
            Ok(())
        );
        let notch = uv(&notched_domain());
        assert_eq!(
            require_swept_rectangle(fk, &notch, &unit_levers(notch.len()), bbox(&notch), eps()),
            Err(TessellateError::UnsupportedCurvedDomain {
                face: fk,
                off_bbox: 2,
                first_uv: (3.0, 2.0),
                max_distance: 1.0,
            }),
            "the two re-entrant corners are the entries strictly inside the box"
        );
    }

    /// **THE BAND'S WITNESS.** A sub-ε off-box entry must be ADMITTED,
    /// and the exact comparison this replaced must refuse the same
    /// entry — passed through [`entries_off_bbox`] itself, not
    /// compared as two literals.
    ///
    /// This row exists because #653 removed the band's live evidence.
    /// Every walk this build produces now sits on its box bitwise, so
    /// the banded and exact forms agree on every real fixture and
    /// `a_split_then_placed_swept_face_is_not_refused` — #648's own
    /// regression row — no longer discriminates between them. A guard
    /// with no red-when-reverted row is a guard nobody is testing, so
    /// the witness is synthetic and says so: the fixture is one
    /// rectangle vertex nudged strictly inside the box by a few ulps of
    /// its own coordinate, which is the exact shape a multiply-carried
    /// iso side produced before #653.
    ///
    /// The nudge is derived from the tree (`next_up` on the fixture's
    /// own coordinate), not transcribed from a sweep — see the doc on
    /// `require_swept_rectangle` for why measured constants are not
    /// allowed to be the argument here any more.
    #[test]
    fn the_band_admits_a_sub_eps_entry_that_the_exact_form_refuses() {
        let fk = fixtures()
            .into_iter()
            .next()
            .and_then(|(_, b)| b.faces().next().map(|(fk, _)| fk))
            .unwrap();
        // The rectangle with a MID-SIDE vertex pulled a few ulps inside
        // the box — the shape a multiply-carried iso side produced
        // before #653, where the second sub-edge's column missed the
        // first's by the last bits of an `atan2`. A nudged CORNER would
        // not do: it still sits on the box's other axis and
        // `entries_off_bbox` takes the nearer of the two.
        // `unit_levers` makes UV units metres, so the wobble reads
        // straight off the coordinates.
        let mut inside = 4.0_f64;
        for _ in 0..4 {
            inside = f64::from_bits(inside.to_bits() - 1);
        }
        let poly = uv(&[
            (0.0, 0.0),
            (4.0, 0.0),
            (inside, 2.0),
            (4.0, 4.0),
            (0.0, 4.0),
        ]);
        let levers = unit_levers(poly.len());
        let b = bbox(&poly);
        let wobble = 4.0 - inside;
        assert!(
            wobble > 0.0 && wobble < eps(),
            "fixture: {wobble} m must be a sub-eps nudge"
        );

        // The EXACT form. `gap_is_noise` is a strict `<`, so `eps = 0`
        // bands nothing at all and every entry comes back — the ones
        // ON the box at distance 0.0. Dropping those reproduces the
        // pre-band comparison exactly, and it sees the nudged entry:
        // one off-box vertex, at index 2, `wobble` metres out.
        let exact: Vec<_> = entries_off_bbox(&poly, &levers, b, 0.0)
            .into_iter()
            .filter(|&(_, d)| d > 0.0)
            .collect();
        assert_eq!(
            exact,
            vec![(2, wobble)],
            "the exact form refuses this entry"
        );

        // The band admits it, and so does the production refusal.
        assert!(
            entries_off_bbox(&poly, &levers, b, eps()).is_empty(),
            "a {wobble} m wobble is float noise, not a re-entrant corner"
        );
        assert_eq!(
            require_swept_rectangle(fk, &poly, &levers, b, eps()),
            Ok(()),
            "the banded guard must not refuse a rectangle that is straight to ulps"
        );
    }

    /// **THE MARGIN — the whole argument for banding in one row.**
    ///
    /// Between what the band must admit and what it must refuse there
    /// are orders of magnitude, so no calibration question arises:
    ///
    /// - a genuine re-entrant corner sits a FEATURE width off the box.
    ///   The notch fixture's corners are 1 m off; the shallowest
    ///   keyway anyone machines is millimetres. Refused.
    /// - the wobble of a multiply-carried iso side was the last bits of
    ///   an `atan2` — an ULP of a UV coordinate, which is what the
    ///   fixture below computes from the tree rather than quoting.
    ///   Admitted.
    ///
    /// ε = 1e-9 m sits between them. This row pins that the band is
    /// placed on the ε scale and not on either population's scale, so a
    /// future ε change is visible here rather than silently
    /// re-classifying parts.
    ///
    /// **On the number that used to be here.** This row read
    /// `wobble = 1.4985e-15`, the worst entry over the PR's 1524-row
    /// split × placement sweep. That sweep is not in the tree and its
    /// population no longer exists — #653 eliminated it — so the
    /// constant could not be re-derived by anyone reading this, which
    /// is §C13's `face_box` shape exactly. It is kept below only as a
    /// labelled historical upper bound, and the live half of the
    /// argument is an ulp the row computes for itself.
    #[test]
    fn the_band_separates_a_feature_from_an_ulp_by_orders_of_magnitude() {
        let e = eps();
        let notch = uv(&notched_domain());
        let corner = entries_off_bbox(&notch, &unit_levers(notch.len()), bbox(&notch), e);
        assert_eq!(corner.len(), 2);
        let feature = corner.iter().map(|&(_, d)| d).fold(0.0_f64, f64::max);
        // The admit side, derived here rather than quoted: one ulp of a
        // UV coordinate at the notch fixture's own scale. Unit levers,
        // so this is metres.
        let ulp = f64::from_bits(4.0_f64.to_bits() + 1) - 4.0;
        assert!(
            ulp < e && e < feature,
            "the band must sit strictly between an ulp of a UV coordinate ({ulp} m) \
             and a feature-sized notch ({feature} m); eps = {e}"
        );
        assert!(
            feature / ulp > 1e14,
            "the two populations must be separated by orders of magnitude, not tuning"
        );
        // HISTORICAL, not re-derivable from this tree: the worst such
        // entry over the pre-#653 sweep in the PR was 1.4985e-15 m,
        // 6.7e5 inside eps. Recorded as the upper bound the band was
        // actually shown to clear, and NOT asserted from — the
        // population it came from is gone.
        assert!(1.498_5e-15_f64 < e);
    }

    /// The counterexample body: a frustum wedge (`revolve` of a
    /// trapezoid through π/2) whose first boundary edge is split TWICE
    /// with the public [`topo::Body::split_edge`], then placed by a
    /// rigid rotation+translation with no special structure.
    ///
    /// Both halves matter. The split makes one iso side of the
    /// cylindrical/conical face carry three edges, so its column comes
    /// from three different `mid_azimuth` points of the same carrier;
    /// the oblique placement is what stops those three `atan2`s from
    /// landing on the same bits. Either alone meshes clean under an
    /// EXACT comparison — it is the pair that produced the false
    /// refusal (#653).
    ///
    /// Nothing here is a kernel back door: `split_edge` is public API,
    /// and the same topology is what every STEP exporter emits when a
    /// vertex lands on an otherwise-straight face boundary.
    fn split_and_placed_frustum_wedge() -> Body<f64> {
        let lp = ProfileLoop::polygon([p2(0.5, 0.0), p2(2.0, 0.0), p2(1.0, 2.0), p2(0.5, 2.0)]);
        let mut body = revolve(
            &validated(vec![lp]),
            axis_y(),
            Revolution::Partial(core::f64::consts::FRAC_PI_2),
        )
        .unwrap()
        .body;
        let ek = body.edges().map(|(k, _)| k).next().unwrap();
        let (t0, t1) = body
            .get_curve_geom(body.get_edge(ek).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap()
            .params();
        body.split_edge(ek, t0 + (t1 - t0) * 0.312_9).unwrap();
        body.split_edge(ek, t0 + (t1 - t0) * 0.156_45).unwrap();
        // Deliberately irrational-ish: an axis-aligned or dyadic
        // placement keeps the sub-edge azimuths bitwise equal and the
        // row goes green for the wrong reason.
        let irr = Vec3::new(0.317_8_f64, 0.941_2, -0.110_9);
        let irr = irr * (1.0 / irr.norm());
        let placement = Affine3::from_parts(
            Affine3::rotation_about_axis(Point3::origin(), irr, 1.0 / 3.0).linear,
            Vec3::new(0.117, -0.339_1, 5.001_7),
        );
        topo::transform_rigid(&body, &placement).unwrap()
    }

    /// **THE REGRESSION ROW (issue #653).** This body meshed
    /// watertight before the guard existed and must keep doing so.
    ///
    /// Under an EXACT comparison it refused
    /// `UnsupportedCurvedDomain { off_bbox: 1 }` on a face whose bbox
    /// is `u = [-7.633e-16, 1.5708]` with the offending entry at
    /// `u = -6.384e-16` — 1.249e-16 rad off, which on that entry's
    /// 0.5 m lever arm is 6.245e-17 m, seven orders of magnitude
    /// inside ε. The domain IS the swept rectangle; the exact test was
    /// measuring float noise.
    ///
    /// **This row no longer goes red if the band is removed** — it did
    /// when it was written, and #653's walk fix is what changed that:
    /// the three sub-edges now share one column bitwise, so `worst`
    /// below is exactly `0.0` and an exact comparison would admit this
    /// body too. Kept as the pre-guard triangle count's pin and as the
    /// oblique-placement fixture; the band's own case is argued in
    /// [`require_swept_rectangle`].
    #[test]
    fn a_split_then_placed_swept_face_is_not_refused() {
        let body = split_and_placed_frustum_wedge();
        for (fk, poly, levers) in curved_walks(&body) {
            assert_eq!(
                require_swept_rectangle(fk, &poly, &levers, bbox(&poly), eps()),
                Ok(()),
                "face {fk:?} of the split+placed wedge is the swept rectangle"
            );
        }
        // The residue, measured through the production path rather than
        // asserted from the issue.
        let worst = worst_entry_off_box(&body);
        assert_eq!(
            worst, 0.0,
            "since #653 the three sub-edges share one column bitwise, so this \
             body's walk IS its bounding rectangle exactly — {worst} m off the \
             box means the iso-side runs stopped working"
        );

        let mesh = crate::tessellate(&body, 0.1).expect("must not refuse");
        crate::validate::check_mesh(&mesh).expect("must mesh watertight");
        let n: usize = mesh.patches.iter().map(|p| p.triangles.len()).sum();
        assert_eq!(n, 42, "the pre-guard triangle count, unchanged");
    }

    /// **What the refusal replaces**, replayed through this lane's own
    /// insertion order. On the swept rectangle the grid touches the
    /// boundary not at all; on the notched domain it splits boundary
    /// constraints and lands on a boundary VERTEX. Both are properties
    /// of the GRID (they are counted against zero on the rectangle,
    /// where the same `inner_faces()` runs), which is why the
    /// ghost-triangle count that used to sit here is gone: with the
    /// grid loop emptied (`nu = nv = 1`) a notched domain still emits
    /// 2 of 8 triangles in the notch, because `inner_faces()` fills the
    /// convex hull — so that assertion could not tell the grid's doing
    /// from this lane's keep-every-inner-face policy. The ghost
    /// geometry is real and is why the refusal exists; it is just not
    /// evidence about the ordering.
    #[test]
    fn the_grid_reaches_the_boundary_only_when_the_premise_fails() {
        assert_eq!(
            replay(&rectangle_domain()),
            (0, 0),
            "on the swept rectangle the grid must not touch the boundary at all"
        );
        let (splits, hits) = replay(&notched_domain());
        assert!(
            splits > 0,
            "a notched domain must split boundary constraints"
        );
        assert!(
            hits > 0,
            "a notched domain must put a grid point on a boundary vertex"
        );
    }

    /// **The two spade facts the inertness argument rests on**, pinned
    /// rather than asserted in prose. `spade` is a CARET requirement,
    /// so a 2.x bump could move either with nothing else in this crate
    /// going red:
    ///
    /// 1. inserting a point that lands exactly ON a constraint splits
    ///    it and re-flags **both** halves as constraints — which is why
    ///    a split corrupts nothing the CDT is later asked about here;
    /// 2. the new vertex takes the **next** handle index — which is
    ///    what keeps `meta`, indexed by handle index, aligned.
    ///
    /// Note the scope: (1) is about what this lane READS. It is not a
    /// claim that a split is harmless at the crate's altitude — a split
    /// boundary constraint breaks the watertightness contract in
    /// `lib.rs` (both adjacent faces conforming to the same segments),
    /// which is the 3-D T-junction `trimmed` refuses as
    /// `SelfTouchingTrimLoop`. That is the reason the domain check is a
    /// refusal and not a comment.
    #[test]
    fn spade_splits_a_constraint_into_two_constraints_and_appends_the_vertex() {
        let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
            ConstrainedDelaunayTriangulation::new();
        let hs: Vec<_> = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]
            .iter()
            .map(|&(u, v)| cdt.insert(SpadePoint::new(u, v)).unwrap())
            .collect();
        for i in 0..hs.len() {
            cdt.add_constraint(hs[i], hs[(i + 1) % hs.len()]);
        }
        let (a, b) = (hs[0], hs[1]);
        assert!(cdt.exists_constraint(a, b));
        let n = cdt.num_vertices();
        let m = cdt.insert(SpadePoint::new(2.0, 0.0)).unwrap();

        assert_eq!(m.index(), n, "the split vertex must take the NEXT index");
        assert!(
            !cdt.exists_constraint(a, b),
            "the whole segment must no longer be a constraint"
        );
        assert!(
            cdt.exists_constraint(a, m) && cdt.exists_constraint(m, b),
            "BOTH halves must be re-flagged as constraints"
        );
    }

    /// Replays this lane's order — boundary points, constraints, then a
    /// 4 × 4 interior grid over the polygon's own bounding box — and
    /// reports (constraint splits, grid points that landed on an
    /// existing vertex).
    fn replay(poly: &[(f64, f64)]) -> (usize, usize) {
        let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
            ConstrainedDelaunayTriangulation::new();
        let hs: Vec<_> = poly
            .iter()
            .map(|&(u, v)| cdt.insert(SpadePoint::new(u, v)).unwrap())
            .collect();
        for i in 0..hs.len() {
            let (a, b) = (hs[i], hs[(i + 1) % hs.len()]);
            if a != b && !cdt.exists_constraint(a, b) {
                assert!(
                    cdt.can_add_constraint(a, b),
                    "the fixture must not self-cross"
                );
                cdt.add_constraint(a, b);
            }
        }
        let (u0, u1, v0, v1) = bbox(&uv(poly));
        let before = cdt.num_constraints();
        let (mut hits, nu, nv) = (0usize, 4usize, 4usize);
        for j in 1..nv {
            #[allow(clippy::cast_precision_loss)]
            let v = v0 + (v1 - v0) * (j as f64 / nv as f64);
            for i in 1..nu {
                #[allow(clippy::cast_precision_loss)]
                let u = u0 + (u1 - u0) * (i as f64 / nu as f64);
                let n = cdt.num_vertices();
                if cdt.insert(SpadePoint::new(u, v)).unwrap().index() != n {
                    hits += 1;
                }
            }
        }
        (cdt.num_constraints() - before, hits)
    }
}
