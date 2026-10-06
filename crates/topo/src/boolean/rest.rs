//! **The declared-REST union zip** (M5 S1, #102's crosslap frontier —
//! the M3 envelope's boundary-on-boundary class (iii)).
//!
//! A *pure REST contact* is a mate whose interiors are DISJOINT and
//! whose shared geometry lies entirely on both operands' boundaries:
//! the contact region R is a union of coincident opposite-oriented
//! face patches — on ANY carrier the ladder certifies (plane, sphere,
//! cylinder, torus; the C4 `Rest` inventory) — and its boundary ∂R — the
//! seam — runs along operand edges or across single faces, never
//! through material. The chord joining ([`super::join`]) matches the
//! germs into segments that name one cell per solid at both of their
//! ends (germs carry their loci; a segment along an edge of a solid is
//! that edge, and at an edge-edge site each solid folds it by its own
//! membership), and its surgery refuses typed where it cannot build a
//! segment's chord — for a seam the join has no section arm for, its
//! per-kind refusal. This lane takes over those refusals, on the
//! join's own segments.
//!
//! This lane replaces the chord/null-face machinery for exactly that
//! frontier, **union only**, reached ONLY when (a) the op carries
//! declared coincident faces (the ladder is law — the undeclared mate
//! keeps refusing at the coincidence door, inside the reduction) and
//! (b) the normal join has already refused typed. The reduction —
//! gates, coincidence doors, sweep splitting, classification — runs
//! unchanged first; this lane consumes its RECORDS:
//!
//! 1. **REST-contact surfaces**: the `Rest` pairs the declaration door
//!    verified one carrier with opposed senses (the reduction's
//!    `rest_contacts`) name the REST-contact surfaces. The door verified
//!    every declaration, and refused a false one
//!    ([`BooleanError::ContactContradicted`]), before the reduction ran,
//!    so this lane verifies nothing again. With no such pair the lane
//!    is not this frontier, before matching runs.
//! 2. **Segments**: the join's own enumeration
//!    ([`super::join::section_segments`]), read from the germ records
//!    before the scaffolding is undone (step 3): each segment's two end
//!    sites and the cell it lies in on each operand, an edge
//!    ([`super::Locus::OnEdge`]) or a face ([`super::Locus::InFace`]).
//!    The lane never pairs germs itself. A matching that leaves germs
//!    loose is not this frontier: the join's refusal stands.
//! 3. **Undo the scaffolding**, inside step 2 once the segments are
//!    read, so the segments' edges and host faces are the operands'
//!    own: the classification's null-edge struts are removed
//!    (`kev`, reverse mint order) from clones of the annotated operands
//!    — the sweep's edge splits and the pierce-ring vertices remain
//!    (both are load-bearing: they make the seam vertex sets congruent
//!    across the mate). A segment end is a strut's `at_vertex`, read
//!    before the undo, and a nested strut's is its holder's copy, which
//!    the undo kills: each end is read through the undo's fusions to
//!    the vertex they leave standing.
//! 4. **Seam realization** (splitting machinery reused): per segment
//!    and per solid, a segment whose cell is an edge already IS that
//!    edge (reused as the seam, minted nowhere), and one whose cell is
//!    a face is minted ONCE as a real chord through the standard
//!    `mef`/`mekr` machinery across that face, on the other solid's
//!    edge along it where there is one. No new region algebra: a
//!    segment that does not resolve structurally refuses typed
//!    ([`BooleanError::RestZipUnsupported`]) or falls back to the
//!    original join refusal (pre-identification phases).
//! 5. **Patch discovery** (structural): the seam partitions each
//!    solid's face-adjacency graph; a region is a contact patch iff
//!    every face lies on a verified opposite-oriented declared
//!    surface (fragments inherit their parent's surface key, so
//!    chord splits keep the license). Patches pair across the mate
//!    by exact vertex-cycle congruence (antiparallel, through the
//!    contact-record vertex correspondence) — verified, never
//!    assumed.
//! 6. **The zip**: operand B grafts whole through the combine door
//!    (interiors are disjoint — nothing is discarded, vol(A∪B) =
//!    vol(A)+vol(B) exactly), then each patch pair is glued: the
//!    contact patches are removed as interior and the seam edges are
//!    fused to single result edges ([`super::zip::zip_seam`] for
//!    pairs sharing nothing; the slit zip below for pairs adjacent
//!    along already-fused seam runs — ONE run or several: a closed
//!    cosurface band's last panel shares a run on each side, and the
//!    band closure kills the later runs by the configuration each is
//!    found in). Glue order is a BFS over the patch adjacency.
//!
//! The result passes the same output stages as every seamed boolean:
//! declared coplanar merge, D6 edge descriptions, contact remapping
//! (REST rests are consumed into structure — the census's consumed
//! class), tier gates, and the volume backstop.

use geom_brep::{EdgeCurveSpec, ExtentBall};
use geom_core::{Band, Bounds, Decide, Point3};
use slotmap::SecondaryMap;

use super::RestZipFrontier;
use super::carrier_eq::{CarrierDesc, CarrierEqError, CarrierRelation};
use super::combine::graft_solid;
use super::ops::{
    Descendants, KeyView, declared_surface_pairs, describe_minted_edges, gate, graft_rows,
    merge_rows, of_merge, remap_carried, remap_contacts,
};
use super::plane_eq::{PlaneEqError, PlaneIdentity, PlaneRelation};
use super::reduce::{face_oriented_source, face_plane};
use super::zip::{Joint, SeamCorrespondence, ZipReport, fuse_by_joint, survivor_checked, zip_seam};
use super::{
    BooleanBody, BooleanDeclarations, BooleanError, BooleanNaming, BooleanOp, BooleanReduction,
    BooleanResult, BooleanResultKind, Locus, Operand, OperandKeys,
};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::{FaceSurface, MefSite};
use crate::euler_ring::MekrSite;
use crate::face_normal::plane_outward_normal;
use crate::geometry::SurfaceKey;
use crate::live::{BoundaryMember, Proven, linked, proven};
use crate::splitting::finish::single_solid;
use geom_core::Tol;

/// A desync inside the lane (after the frontier is positively
/// identified) — same posture as the join's lockstep refusals: a key
/// the lane carries across its own surgery that no longer resolves, or
/// bookkeeping that disagrees with the body. The lane kills edges,
/// vertices and faces mid-operation, so no premise proves those keys
/// live, and they answer typed. Two exceptions panic: a hop past a
/// record the lane just resolved is a link ([`loop_boundary`]), and a
/// segment end has a premise ([`SEGMENT_ENDS_SURVIVE`]).
fn desync(what: &'static str) -> BooleanError {
    BooleanError::JoinDesync { what }
}

/// The boundary of loop `l`, which `holder`'s field `field` names: a
/// link, so a miss panics, as does a walk of it ([`Body::loop_walk`]
/// closed; [`crate::live::OPERATORS_KEEP_LINKS`]).
fn loop_boundary<T: Decide>(
    body: &Body<T>,
    l: LoopKey,
    holder: EntityId,
    field: &'static str,
) -> LoopBoundary {
    linked(&body.loops, l, EntityId::Loop, holder, field).boundary
}

/// The closed loop walk from `first`, a loop's anchor this call read.
fn cycle<T: Decide>(body: &Body<T>, first: HalfEdgeKey) -> Vec<HalfEdgeKey> {
    body.loop_walk(first).closed("loop", first)
}

/// A named sub-frontier the lane declines (honest typed refusal,
/// never a laundered catch-all).
fn unsupported(what: RestZipFrontier) -> BooleanError {
    BooleanError::RestZipUnsupported { what }
}

/// One seam segment: the two end sites, as vertex keys per operand
/// (each the vertex its site's strut copies fuse into across the
/// scaffolding undo), and the cell it lies in on each operand: an edge
/// the segment already is, or the face it crosses.
#[derive(Clone, Copy, Debug)]
struct Segment {
    a_u: VertexKey,
    a_v: VertexKey,
    b_u: VertexKey,
    b_v: VertexKey,
    a_cell: Locus,
    b_cell: Locus,
}

/// The declared-REST union lane (module docs). `red` is the finished
/// reduction whose normal join REFUSED; its bodies must be the
/// pre-join annotated clones. Returns `Ok(None)` when the
/// configuration is not this lane's frontier — the caller then
/// surfaces the original join refusal unchanged.
///
/// # Errors
///
/// [`BooleanError`] — [`BooleanError::ContactContradicted`] for
/// false declarations at the lane door,
/// [`BooleanError::RestZipUnsupported`] for named sub-frontiers, and
/// the shared output-stage refusals.
///
/// The `Decide + Bounds` compound bound is the boolean-seam bound
/// (ratified 2026-07-29 — see geom-core `real.rs`, Bounds scope
/// rule); this module is part of that seam alongside `ops`/`reduce`.
pub(super) fn try_rest_union<T: Decide + Bounds + crate::props::AtRestPolicy>(
    mut red: BooleanReduction<T>,
    a_pristine: &Body<T>,
    b_pristine: &Body<T>,
    decls: &BooleanDeclarations,
    band: Band,
    tol: Tol,
) -> Result<Option<BooleanResult<T>>, BooleanError> {
    debug_assert_eq!(red.op, BooleanOp::Union);
    if decls.coincident_faces.is_empty() || red.null_pairs.is_empty() {
        return Ok(None);
    }

    // ---- 1. The REST-contact (opposite-oriented) surface sets. ----
    let (a_rest, b_rest) = rest_surfaces(a_pristine, b_pristine, &red.rest_contacts)?;
    if a_rest.is_empty() || b_rest.is_empty() {
        return Ok(None); // no opposite-oriented contact declared
    }

    // ---- 2. Segments: the join's matching of the germ records. ----
    let Some(segments) = read_segments(&mut red, band, tol)? else {
        return Ok(None);
    };

    // Vertex correspondence across the mate, operand keys — record
    // data end to end (F9): the segment end sites, EXTENDED by the
    // reduction's own v-v contact records. A coincident vertex pair
    // INTERIOR to the contact region (e.g. a peg-root rim vertex on
    // the mating plane) has a v-v record but no crossing at its site,
    // so no null pair and no segment names it — and the glue still
    // fuses it when the interior curve network zips. Never geometric
    // point matching. The patches pair and glue one-to-one, so a
    // vertex meeting two of the other solid's, which lie at its one
    // point (a pinch apex), refuses here in either order, before any
    // chord is minted.
    let mut vcorr: SecondaryMap<VertexKey, VertexKey> = SecondaryMap::new();
    let mut a_of: SecondaryMap<VertexKey, VertexKey> = SecondaryMap::new();
    let mut correspond = |a: VertexKey, b: VertexKey| -> Result<(), BooleanError> {
        let other_b = vcorr.get(a).is_some_and(|&prev| prev != b);
        let other_a = a_of.get(b).is_some_and(|&prev| prev != a);
        if other_b || other_a {
            return Err(unsupported(RestZipFrontier::PinchApex));
        }
        vcorr.insert(a, b);
        a_of.insert(b, a);
        Ok(())
    };
    for s in &segments {
        for (a, b) in [(s.a_u, s.b_u), (s.a_v, s.b_v)] {
            correspond(a, b)?;
        }
    }
    for c in &red.contacts.vv {
        correspond(c.a, c.b)?;
    }

    // Pierce-ring vertices: ring vertex → host face, per operand.
    let mut a_rings: SecondaryMap<VertexKey, FaceKey> = SecondaryMap::new();
    let mut b_rings: SecondaryMap<VertexKey, FaceKey> = SecondaryMap::new();
    for r in &red.pierce_rings {
        match r.operand {
            Operand::A => a_rings.insert(r.ring_vertex, r.face),
            Operand::B => b_rings.insert(r.ring_vertex, r.face),
        };
    }

    // ---- 4. Seam realization, per solid. ----
    let mut a_fragments = Vec::new();
    let mut b_fragments = Vec::new();
    let a_seam = realize_seam(
        &mut red.a,
        &red.b,
        &segments
            .iter()
            .map(|s| Span {
                ends: (s.a_u, s.a_v),
                cell: s.a_cell,
                theirs: (s.b_u, s.b_v),
                twin: s.b_cell.edge(),
            })
            .collect::<Vec<_>>(),
        &a_rings,
        &mut a_fragments,
        tol,
    )?;
    let Some(a_seam) = a_seam else {
        return Ok(None);
    };
    let b_seam = realize_seam(
        &mut red.b,
        &red.a,
        &segments
            .iter()
            .zip(&a_seam.per_segment)
            .map(|(s, &ea)| Span {
                ends: (s.b_u, s.b_v),
                cell: s.b_cell,
                theirs: (s.a_u, s.a_v),
                twin: Some(ea),
            })
            .collect::<Vec<_>>(),
        &b_rings,
        &mut b_fragments,
        tol,
    )?;
    let Some(b_seam) = b_seam else {
        return Ok(None);
    };

    // ---- 5. Patch discovery, the interior curve networks made
    // congruent, cross-mate pairing. ----
    let Some(a_patch) = patch_faces(&red.a, &a_seam, &a_rest)? else {
        return Ok(None);
    };
    let Some(b_patch) = patch_faces(&red.b, &b_seam, &b_rest)? else {
        return Ok(None);
    };
    let a_interior = interior_edges(&red.a, &a_patch, &a_seam)?;
    let b_interior = interior_edges(&red.b, &b_patch, &b_seam)?;
    let Some(a_patch) = mirror_edges(
        &mut red.a,
        &red.b,
        &b_interior,
        &a_of,
        &a_patch,
        &a_rings,
        &mut a_fragments,
        tol,
    )?
    else {
        return Ok(None);
    };
    let Some(b_patch) = mirror_edges(
        &mut red.b,
        &red.a,
        &a_interior,
        &vcorr,
        &b_patch,
        &b_rings,
        &mut b_fragments,
        tol,
    )?
    else {
        return Ok(None);
    };
    if a_patch.len() != b_patch.len() {
        return Ok(None);
    }
    let pairs = pair_patches(&red.a, &red.b, &a_patch, &b_patch, &vcorr)?;

    // The frontier is positively identified from here on: failures are
    // the lane's own typed refusals, never silently swapped back.

    // ---- 6. Graft B whole (disjoint interiors: nothing discarded),
    // then glue every patch pair in BFS order. ----
    let glue_order = bfs_order(&red.a, &a_patch, &a_seam)?;
    // A REST union's patches are opposed contact faces, whose lump keeps
    // neither copy, so it holds no region through a coincident copy and
    // its rows have no held stretches (`DiscardRow::held`).
    if !red.held.is_empty() {
        return Err(BooleanError::JoinDesync {
            what: "a declared-REST union holds a region through a coincident copy",
        });
    }
    // The contact patches are the faces this union discards; A's keys
    // are the result's, so its rows are taken here, before the zip.
    let mut discards = patch_discards(&red.a, &a_patch, Operand::A, &|u, w| Ok((u, w)))?;
    // The zip, the merge and the closing mint are one door's surgery
    // (`crate::surgery`): tier 1 is paid once, over the body `gate`
    // below certifies, rather than once per operator. The guard owns
    // the borrow, so a refusal on the way closes the scope by dropping
    // it.
    let mut zipped = red.a;
    let mut body = zipped.begin_surgery();
    let solid = single_solid(&body).map_err(|_| desync("REST lane: operand A not one solid"))?;
    let graft = graft_solid(&mut body, solid, &red.b, tol)?;
    let graft_end = |v: VertexKey| {
        graft
            .vertices
            .get(v)
            .copied()
            .ok_or_else(|| desync("REST lane: a patch vertex is missing from the graft"))
    };
    discards.extend(patch_discards(&red.b, &b_patch, Operand::B, &|u, w| {
        Ok((graft_end(u)?, graft_end(w)?))
    })?);

    // Result-key views of the correspondence and the patch pairs.
    let mut vmap: SecondaryMap<VertexKey, VertexKey> = SecondaryMap::new();
    for (a, &b) in vcorr.iter() {
        let b_result = *graft
            .vertices
            .get(b)
            .ok_or_else(|| desync("REST lane: seam vertex missing from the graft"))?;
        vmap.insert(a, b_result);
    }
    let fb_of = |fa: FaceKey| -> Result<FaceKey, BooleanError> {
        let fb = pairs
            .iter()
            .find(|&&(pa, _)| pa == fa)
            .map(|&(_, pb)| pb)
            .ok_or_else(|| desync("REST lane: unpaired patch face in glue order"))?;
        graft
            .faces
            .get(fb)
            .copied()
            .ok_or_else(|| desync("REST lane: patch face missing from the graft"))
    };

    let mut vertex_merges: Vec<(VertexKey, VertexKey)> = Vec::new();
    let mut seam_edges: Vec<EdgeKey> = a_seam.per_segment.clone();
    let mut desc = Descendants::default();
    for &fa in &glue_order {
        let fb = fb_of(fa)?;
        let rep = glue_pair(&mut body, fa, fb, &vmap, tol)?;
        settle_glue(&body, &mut seam_edges, &rep.interior_edges)?;
        desc.absorb_zip(&rep);
        vertex_merges.extend(rep.vertex_merges.iter().copied());
    }

    // ---- Output stages (shared with every seamed boolean). ----
    let contacts = red.contacts.clone();
    let reduction_contacts = red.contacts;
    let covered = red.covered;
    let declared_pairs = declared_surface_pairs(&body, a_pristine, b_pristine, decls, &graft);
    let merged = body
        .merge_coplanar_faces_declared(&declared_pairs, tol)
        .map_err(of_merge)?;
    desc.absorb_merge(&merged);
    describe_minted_edges(&mut body, &seam_edges, &merged, band, tol)?;
    let mut contacts = remap_contacts(
        &body,
        &contacts,
        KeyView::Direct,
        KeyView::Graft(&graft),
        &desc,
    )?;
    remap_carried(
        &mut contacts,
        &body,
        decls,
        &KeyView::Direct,
        &KeyView::Graft(&graft),
        &desc,
    )?;
    body.sweep_and_close();
    let body = gate(zipped, band, tol)?;
    T::gate_volume_backstop(BooleanOp::Union, a_pristine, b_pristine, &body, band, tol)?;
    let (graft_vertices, graft_edges, graft_dead_edges, graft_faces) = graft_rows(&graft);
    let naming = BooleanNaming {
        a_keys: OperandKeys::Direct,
        b_keys: OperandKeys::Grafted,
        graft_vertices,
        graft_edges,
        graft_dead_edges,
        graft_faces,
        seam_edges,
        vertex_merges,
        weld_merges_b: Vec::new(),
        merge_groups: merge_rows(&merged),
        merge_skipped: merged.skipped.clone(),
        face_fragments_a: a_fragments,
        face_fragments_b: b_fragments,
        reduction_contacts,
        discards,
        covered,
    };
    Ok(Some(BooleanResult::Body(BooleanBody {
        body,
        kind: BooleanResultKind::Seamed,
        contacts,
        naming,
    })))
}

/// Settles one glue's deaths against the seam. The surviving seam is
/// the A-side per-segment edges (the A arena IS the result arena); a
/// segment INTERIOR to the contact region — an already-fused run the
/// glue consumed — dies with R's interior, and the glue reports it so. Every edge the
/// glue reports interior is dead after it, and every seam edge that
/// died in it is one it reports: anything else is a lane desync. `seam`
/// holds the seam edges alive before the glue and keeps those alive
/// after it, in segment order.
fn settle_glue<T: Decide>(
    body: &Body<T>,
    seam: &mut Vec<EdgeKey>,
    interior: &[EdgeKey],
) -> Result<(), BooleanError> {
    if interior.iter().any(|&e| body.get_edge(e).is_some()) {
        return Err(desync(
            "REST lane: an edge the glue reported interior survived it",
        ));
    }
    if seam
        .iter()
        .any(|&e| body.get_edge(e).is_none() && !interior.contains(&e))
    {
        return Err(desync("REST lane: a seam segment edge did not survive"));
    }
    seam.retain(|&e| body.get_edge(e).is_some());
    Ok(())
}

/// One operand's discarded faces (`boolean::discard`): its contact
/// patch, removed as interior by the glue. A patch face borders a kept
/// face along a seam segment, whose ends `result_ends` maps into result
/// keys.
fn patch_discards<T: Decide>(
    body: &Body<T>,
    patch: &[FaceKey],
    operand: Operand,
    result_ends: &dyn Fn(VertexKey, VertexKey) -> Result<(VertexKey, VertexKey), BooleanError>,
) -> Result<Vec<super::DiscardRow>, BooleanError> {
    let kept_across = |f: FaceKey| !patch.contains(&f);
    patch
        .iter()
        .map(|&f| {
            let ends = |_, u, w| result_ends(u, w);
            super::discard::discard_row(body, f, operand, &kept_across, &ends, None)
        })
        .collect()
}

// ---------------------------------------------------------------
// 2. Segments.
// ---------------------------------------------------------------

/// The seam segments, read from the join's one enumeration
/// ([`super::join::section_segments`]) while the germ records still
/// stand; the null-edge scaffolding is then undone (step 3), so the
/// segments' edges and host faces are the operands' own, and each end
/// is read through the undo's fusions. `None`: the matching left germs
/// loose, so the join's refusal stands.
fn read_segments<T: Decide + crate::props::AtRestPolicy>(
    red: &mut BooleanReduction<T>,
    band: Band,
    tol: Tol,
) -> Result<Option<Vec<Segment>>, BooleanError> {
    let matched = super::join::section_segments(red, band)?;
    if matched.len() != red.null_pairs.len() {
        return Ok(None);
    }
    let at = |operand: Operand, edge: EdgeKey| {
        red.null_edges
            .iter()
            .find(|r| r.operand == operand && r.edge == edge)
            .map(|r| r.at_vertex)
            .ok_or_else(|| desync("REST lane: pair edge without a record"))
    };
    let mut sites = Vec::with_capacity(red.null_pairs.len());
    for p in &red.null_pairs {
        sites.push((at(Operand::A, p.a_edge)?, at(Operand::B, p.b_edge)?));
    }
    let fused = undo_struts(red, tol)?;
    let segments = matched
        .iter()
        .map(|m| {
            let [(u, _), (v, _)] = m.ends;
            let ((a_u, b_u), (a_v, b_v)) = (sites[u], sites[v]);
            Ok(Segment {
                a_u: fused.end(Operand::A, a_u)?,
                a_v: fused.end(Operand::A, a_v)?,
                b_u: fused.end(Operand::B, b_u)?,
                b_v: fused.end(Operand::B, b_v)?,
                a_cell: m.germ.a_locus,
                b_cell: m.germ.b_locus,
            })
        })
        .collect::<Result<_, BooleanError>>()?;
    Ok(Some(segments))
}

// ---------------------------------------------------------------
// 1. The REST-contact surfaces.
// ---------------------------------------------------------------

/// The per-operand verified REST-contact (opposite-oriented declared)
/// surface sets.
type RestSurfaces = (SecondaryMap<SurfaceKey, ()>, SecondaryMap<SurfaceKey, ()>);

/// **The one flush-pair door**: the C4 verify ladder for a single
/// cross-body PLANAR face pair — [`carrier_pair_relation`] restricted
/// to two planes (descriptions through [`face_plane`], outward and
/// sense-folded; S10's oriented sources, so rung 1's `orient` tags
/// carry the face senses too — REST contact is precisely the
/// `SameOpposite` verdict). It is that door, not a mirror of it: the
/// verdict, the `decide` sites and the lever are the ones every carrier
/// pair gets.
///
/// **This door has NO in-tree consumer.** Verify-at-use stopped
/// calling it at M9-1 and the flush detector followed when its scope
/// became the `Rest` ladder's; what to do about a published door with
/// no caller is `work/seat/flush-pair-relation-has-no-caller.md`.
///
/// `Err`: a face that is not a plane, which this door has no
/// description for ([`PairUnread::OutsideInventory`]; the REST lane
/// treats it as an invariant violation at its own site, and
/// [`carrier_pair_relation`] is where a caller asks the same question
/// of any carrier the ladder names), or whose extent cannot be read.
///
/// # Errors
///
/// [`PairUnread`], as above.
///
/// # Panics
///
/// Where a face resolves and its surface does not, or a link
/// [`carrier_pair_relation`] reads does not resolve: a torn surface is
/// not one outside the inventory.
pub fn flush_pair_relation<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    declared: bool,
    band: Band,
) -> Result<Result<PlaneRelation, PlaneEqError>, PairUnread> {
    face_plane(a, fa).ok_or(PairUnread::OutsideInventory)?;
    face_plane(b, fb).ok_or(PairUnread::OutsideInventory)?;
    carrier_pair_relation(a, fa, b, fb, declared, band)
}

/// Which face of a pair, in the order a door took them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairFace {
    /// The first face.
    First,
    /// The second face.
    Second,
}

/// Why a face pair has no carrier reading: the door's input, not a
/// verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairUnread {
    /// A face's surface kind is outside the ladder's inventory (cone,
    /// NURBS, `Approx`), or its key does not resolve: there is no
    /// description to compare.
    OutsideInventory,
    /// The face's consumed extent cannot be read ([`pair_extent`]): its
    /// key does not resolve, its box has no claim to make (a NURBS
    /// placeholder, a boundary edge with no sound box), or the ball
    /// around it does not read.
    Extent(PairFace),
}

/// **A face's consumed extent**: a ball enclosing every point of it,
/// the region over which a verdict about its carrier is consumed
/// ([`pair_extent`]). A sphere's or a torus's own ball
/// ([`ExtentBall::of_carrier`]: the torus's `R + r`, whatever the
/// trim); otherwise the ball around the face's certified box, from the
/// kernel's one kind→box rule (`census::face_reach`). `None` where
/// `face`, the caller's key, does not resolve, where that box has no
/// claim to make, or where the ball does not read. The face's surface
/// is a link, and its miss panics (on an at-rest operand by tier 1,
/// mid-operation by [`crate::live::OPERATORS_KEEP_LINKS`]).
fn face_ball<T: Decide>(body: &Body<T>, face: FaceKey, band: Band) -> Option<ExtentBall<T>> {
    let f = body.get_face(face)?;
    let ball = match ExtentBall::of_carrier(body.face_surface_linked(face, f)) {
        Some(ball) => ball,
        None => {
            let (lo, hi) = crate::census::face_reach(body, face, band)?;
            ExtentBall::of_box(lo, hi)
        }
    };
    ball.readable()
}

/// The face's boundary vertex positions (outer loop then rings; an
/// empty loop contributes its lone vertex): points known to lie on the
/// face. `None` where `face`, the caller's key, does not resolve.
///
/// Every hop past the face is a link (its loops, their walks, each
/// member's edge, start vertex and its point, a lone vertex's point; a null strut's half-edges walk
/// like any other), and a miss panics (on an at-rest body by tier 1,
/// mid-operation by [`crate::live::OPERATORS_KEEP_LINKS`]).
pub(crate) fn face_witnesses<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<Vec<Point3<T>>> {
    let f = body.get_face(face)?;
    Some(
        body.face_boundary_linked(face, f)
            .map(|member| match member {
                BoundaryMember::Isolated { point, .. } => point,
                BoundaryMember::Edge { he, half, .. } => {
                    body.linked_vertex_point(half.start, EntityId::HalfEdge(he), "start")
                }
            })
            .collect(),
    )
}

/// **A face pair's consumed extent**, as the carrier doors read it:
/// one ball enclosing both faces ([`face_ball`]), since the verdict is
/// consumed on each, and each face's boundary vertices, the points
/// known to be consumed ([`super::carrier_eq::ConsumedExtent`]).
#[derive(Clone, Debug)]
pub(crate) struct PairExtent<T: geom_core::Real> {
    /// The ball enclosing both faces.
    pub(crate) reach: ExtentBall<T>,
    /// The first face's boundary vertices, then the second's.
    pub(crate) on: [Vec<Point3<T>>; 2],
}

impl<T: geom_core::Real> PairExtent<T> {
    /// The extent as the ladder reads it, its faces in the order
    /// measured, or the other way round when `swapped`.
    pub(crate) fn consumed(&self, swapped: bool) -> super::carrier_eq::ConsumedExtent<'_, T> {
        let [first, second] = &self.on;
        super::carrier_eq::ConsumedExtent {
            reach: self.reach,
            on: if swapped {
                [second, first]
            } else {
                [first, second]
            },
        }
    }
}

/// [`PairExtent`] of `fa` on `a` and `fb` on `b`. Read on the operands
/// at rest: mid-operation, a face whose boundary carries null
/// scaffolding has no box to read, and the sites there take a declared
/// pair's extent from [`super::DeclaredPairs::consumed`].
///
/// # Errors
///
/// The face whose extent cannot be read ([`PairUnread::Extent`]).
pub(crate) fn pair_extent<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<PairExtent<T>, PairFace> {
    let read = |body, face| face_ball(body, face, band).zip(face_witnesses(body, face));
    let (ball_a, on_a) = read(a, fa).ok_or(PairFace::First)?;
    let (ball_b, on_b) = read(b, fb).ok_or(PairFace::Second)?;
    // Two readable balls enclose readably short of overflow, which the
    // larger reach is what tips; the second face takes the blame
    // rather than neither.
    let reach = ExtentBall::enclosing(&[ball_a, ball_b])
        .and_then(ExtentBall::readable)
        .ok_or(PairFace::Second)?;
    Ok(PairExtent {
        reach,
        on: [on_a, on_b],
    })
}

/// The face's **oriented carrier description** — the curved
/// generalization of [`face_plane`], folding the face's sense into
/// the material side exactly as that door does (S10).
///
/// `None` for a surface kind outside the `Rest` ladder's inventory
/// (cone, NURBS, `Approx`): the C4 table names the kinds
/// [`mod@super::carrier_eq`] carries a rung for, and a kind it cannot
/// compare refuses typed at the caller rather than being approximated
/// by one it can. `None` too where `face`, the caller's key, does not
/// resolve.
///
/// # Panics
///
/// Where the face's surface does not resolve: a link, which every
/// public door keeps live, and which the reduction's operators keep
/// live mid-operation ([`crate::live::OPERATORS_KEEP_LINKS`]).
pub fn face_carrier<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<CarrierDesc<T>> {
    let f = body.get_face(face)?;
    // `sense` is the material-side bit: true means the face's outward
    // normal IS the chart normal, which for a sphere/cylinder chart
    // points away from the centre/axis. Read as a BIT, never as a
    // comparison on `T` — the scalar backends order intervals, not
    // signs (S10's exact-bit discipline).
    let outward = f.sense;
    match body.face_surface_linked(face, f) {
        geom::Surface::Plane { origin, normal, .. } => Some(CarrierDesc::Plane {
            origin: *origin,
            normal: plane_outward_normal(f, *normal).vec(),
        }),
        geom::Surface::Sphere { center, radius, .. } => Some(CarrierDesc::Sphere {
            center: *center,
            radius: *radius,
            outward,
        }),
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => Some(CarrierDesc::Cylinder {
            origin: *origin,
            axis: *axis,
            radius: *radius,
            outward,
        }),
        geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => Some(CarrierDesc::Torus {
            center: *center,
            axis: *axis,
            major_radius: *major_radius,
            minor_radius: *minor_radius,
            outward,
        }),
        _ => None,
    }
}

/// **The one carrier-pair door**: [`flush_pair_relation`] for every
/// carrier kind the `Rest` table names.
///
/// Descriptions through [`face_carrier`], identity through
/// [`face_oriented_source`], and the pair's consumed extent through
/// [`pair_extent`]: a declared verdict that bridges is one whose
/// displacement stays in band at every point of both faces
/// ([`super::carrier_eq::pair_door_verdict`]). One door for the
/// verify-at-use site and the detector's candidate-generation mode.
///
/// # Errors
///
/// [`PairUnread`]: a face whose surface kind is outside the ladder's
/// inventory — there is no description to compare — or whose extent
/// cannot be read. The ladder's own refusals ride inside the `Ok`.
///
/// # Panics
///
/// Where a link of either face does not resolve: its surface
/// ([`face_carrier`]), its loops, their walks, or a boundary vertex's
/// point.
pub fn carrier_pair_relation<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    declared: bool,
    band: Band,
) -> Result<Result<CarrierRelation, CarrierEqError>, PairUnread> {
    Ok(carrier_pair_verdict(a, fa, b, fb, declared, band)?.map(|(rel, _)| rel))
}

/// [`carrier_pair_relation`] plus the AQ6 trilean — the door the
/// CONTACT verification uses, since only a caller that can see the
/// bridged residue can enforce C4's "trusted exactly there" invariant.
/// One traversal, two projections.
///
/// # Errors
///
/// As [`carrier_pair_relation`].
///
/// # Panics
///
/// As [`carrier_pair_relation`].
pub fn carrier_pair_verdict<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    declared: bool,
    band: Band,
) -> Result<Result<(CarrierRelation, crate::contact::ContactVerdict), CarrierEqError>, PairUnread> {
    let ca = face_carrier(a, fa).ok_or(PairUnread::OutsideInventory)?;
    let cb = face_carrier(b, fb).ok_or(PairUnread::OutsideInventory)?;
    let extent = pair_extent(a, fa, b, fb, band).map_err(PairUnread::Extent)?;
    let (ga, gb) = (face_oriented_source(a, fa), face_oriented_source(b, fb));
    let id = PlaneIdentity {
        s1: ga.as_ref(),
        s2: gb.as_ref(),
        declared,
    };
    Ok(super::carrier_eq::pair_door_verdict(
        &ca,
        &cb,
        id,
        &extent.consumed(false),
        band,
    ))
}

/// The REST-contact surface sets per operand: the surfaces of the
/// `Rest` pairs the declaration door verified one carrier with opposed
/// senses ([`BooleanReduction`]'s `rest_contacts`). The door refused
/// every false declaration before the reduction ran, so this lane reads
/// its certificates and verifies nothing again.
fn rest_surfaces<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    pairs: &[(FaceKey, FaceKey)],
) -> Result<RestSurfaces, BooleanError> {
    let surface = |body: &Body<T>, f: FaceKey| {
        body.get_face(f)
            .map(|face| face.surface)
            .ok_or(BooleanError::ClassificationInvariant {
                what: "REST lane: a verified declared face vanished",
            })
    };
    let mut a_rest: SecondaryMap<SurfaceKey, ()> = SecondaryMap::new();
    let mut b_rest: SecondaryMap<SurfaceKey, ()> = SecondaryMap::new();
    for &(fa, fb) in pairs {
        a_rest.insert(surface(a, fa)?, ());
        b_rest.insert(surface(b, fb)?, ());
    }
    Ok((a_rest, b_rest))
}

// ---------------------------------------------------------------
// 3. Scaffolding undo.
// ---------------------------------------------------------------

/// Per operand, the fusions `(copy, site)` of [`undo_struts`]' kills,
/// in kill order.
#[derive(Default)]
struct Fused {
    a: Vec<(VertexKey, VertexKey)>,
    b: Vec<(VertexKey, VertexKey)>,
}

impl Fused {
    /// The vertex `w` of `operand` stands as once the undo is done
    /// ([`survivor_checked`]): a nested strut's site is its holder's
    /// copy, which fuses into the holder's own site.
    ///
    /// # Errors
    ///
    /// [`BooleanError::JoinDesync`] on a log that is not well-ordered,
    /// in every build: it would fold `w` onto a dead key.
    fn end(&self, operand: Operand, w: VertexKey) -> Result<VertexKey, BooleanError> {
        survivor_checked(
            match operand {
                Operand::A => &self.a,
                Operand::B => &self.b,
            },
            w,
        )
    }
}

/// Removes every classification-minted null-edge strut (the band
/// `kev`, `kev_describing`, so a loop a strut's kill releases is minted
/// whole; reverse mint order), fusing each copy into the strut's site
/// vertex (`at_vertex`), and answers those fusions. Sweep splits and
/// pierce-ring vertices remain.
fn undo_struts<T: Decide + crate::props::AtRestPolicy>(
    red: &mut BooleanReduction<T>,
    tol: Tol,
) -> Result<Fused, BooleanError> {
    let mut fused = Fused::default();
    for r in red.null_edges.iter().rev() {
        let (body, fused) = match r.operand {
            Operand::A => (&mut red.a, &mut fused.a),
            Operand::B => (&mut red.b, &mut fused.b),
        };
        let copy = r
            .attr
            .copy_at(r.at_vertex)
            .ok_or_else(|| desync("REST lane: strut without its site vertex as an end"))?;
        let edge = body
            .get_edge(r.edge)
            .ok_or_else(|| desync("REST lane: strut edge no longer resolves"))?
            .clone();
        let (he, field) = if body.half_edge_end(edge.he_plus) == Some(copy) {
            (edge.he_plus, "he_plus")
        } else if body.half_edge_end(edge.he_minus) == Some(copy) {
            (edge.he_minus, "he_minus")
        } else {
            return Err(desync("REST lane: strut halves do not reach the copy"));
        };
        // `kev(he)` kills `he`'s end, the copy, and keeps its start.
        let start = linked(
            &body.half_edges,
            he,
            EntityId::HalfEdge,
            EntityId::Edge(r.edge),
            field,
        );
        if start.start != r.at_vertex {
            return Err(desync("REST lane: strut does not join its site vertex"));
        }
        body.kev_describing(he, &[], tol)
            .map_err(|_| desync("REST lane: strut undo kev refused"))?;
        fused.push((copy, r.at_vertex));
    }
    Ok(fused)
}

// ---------------------------------------------------------------
// 4. Seam realization.
// ---------------------------------------------------------------

/// Why every segment end resolves where [`realize_seam`] looks it up
/// ([`read_segments`], [`undo_struts`]; the chords are `mef`/`mekr`).
const SEGMENT_ENDS_SURVIVE: &str = "each segment end is its strut site read through the strut \
     undo's fusions, the undo checks every site live at its kill and logs every vertex it kills, \
     and the lane kills none between the undo and the seam's chords, which kill none";

/// One solid's realized seam.
struct SeamSet {
    /// Every seam edge (structural set).
    set: SecondaryMap<EdgeKey, ()>,
    /// The seam edge of each segment, in segment order.
    per_segment: Vec<EdgeKey>,
}

/// An edge's two vertices, `(u, v)`.
type VertexPair = (VertexKey, VertexKey);

/// One segment as one solid realizes it: its ends here, the cell it
/// lies in here, its ends in the other solid, and the other solid's
/// edge along it where there is one (the chord's carrier).
struct Span {
    ends: VertexPair,
    cell: Locus,
    theirs: VertexPair,
    twin: Option<EdgeKey>,
}

/// Realizes the seam in one solid: per segment, the edge it already is
/// (an `OnEdge` cell), else a chord minted through the standard
/// splitting machinery across the face it lies in, on the other solid's
/// edge for the segment where it has one. A chord that divides a face
/// leaves its rings on the old face (`mef`), and a chord between two
/// pierce-ring vertices joined to nothing has no boundary to start from.
/// So the seam grows outward from the face's boundary: a segment is
/// taken only once one of its ends is joined to it, those that join a
/// ring vertex first, and realizing one joins its other end. A segment
/// both of whose ends stay unjoined once no other can be taken refuses
/// typed ([`RestZipFrontier::SegmentsBetweenIsolatedPierces`]).
/// `Ok(None)`: a segment does not resolve structurally — not this lane's
/// frontier (pre-identification phase).
///
/// # Panics
///
/// Where a segment's end does not resolve, before any segment is
/// taken: [`SEGMENT_ENDS_SURVIVE`].
fn realize_seam<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    other: &Body<T>,
    spans: &[Span],
    rings: &SecondaryMap<VertexKey, FaceKey>,
    fragments: &mut Vec<(FaceKey, FaceKey)>,
    tol: Tol,
) -> Result<Option<SeamSet>, BooleanError> {
    let mut per_segment: Vec<Option<EdgeKey>> = vec![None; spans.len()];
    loop {
        let unjoined_end = |w: VertexKey| {
            let end = body.get_vertex(w).unwrap_or_else(|| {
                unreachable!(
                    "REST lane: segment end {} does not resolve: {SEGMENT_ENDS_SURVIVE}",
                    EntityId::Vertex(w)
                )
            });
            usize::from(end.emanating.is_none())
        };
        let open = || (0..spans.len()).filter(|&i| per_segment[i].is_none());
        let unjoined: Vec<(usize, usize)> = open()
            .map(|i| {
                let (u, v) = spans[i].ends;
                (i, unjoined_end(u) + unjoined_end(v))
            })
            .collect();
        let taking = |n: usize| unjoined.iter().find(|&&(_, k)| k == n).map(|&(i, _)| i);
        let next = taking(1).or_else(|| taking(0));
        let Some(i) = next else {
            if open().next().is_some() {
                return Err(unsupported(RestZipFrontier::SegmentsBetweenIsolatedPierces));
            }
            break;
        };
        let span = &spans[i];
        let (u, v) = span.ends;
        per_segment[i] = Some(match span.cell {
            Locus::OnEdge(e) => {
                let ends = body
                    .edge_vertices(e)
                    .ok_or_else(|| desync("REST lane: a seam edge no longer resolves"))?;
                if ends != (u, v) && ends != (v, u) {
                    return Err(desync("REST lane: a segment's edge does not join its ends"));
                }
                e
            }
            Locus::InFace(face) => {
                let twin = match span.twin {
                    Some(e) => Twin::of(other, e, span.theirs, span.ends)?,
                    None => None,
                };
                let Some(host) = fragment_holding(body, face, fragments, u, v, rings)? else {
                    return Ok(None);
                };
                mint_chord(body, host, u, v, twin.as_ref(), fragments, tol)?
            }
        });
    }
    let per_segment: Vec<EdgeKey> = per_segment.into_iter().flatten().collect();
    Ok(Some(SeamSet {
        set: per_segment.iter().map(|&e| (e, ())).collect(),
        per_segment,
    }))
}

/// The face holding both `u` and `v` among `face` and the fragments
/// chords already minted split off it: a segment's cell names the face
/// as the reduction left it, and an earlier chord may have divided it.
/// `None` when not exactly one does.
fn fragment_holding<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    fragments: &[(FaceKey, FaceKey)],
    u: VertexKey,
    v: VertexKey,
    rings: &SecondaryMap<VertexKey, FaceKey>,
) -> Result<Option<FaceKey>, BooleanError> {
    let lineage = crate::chord_join::lineage(face, fragments);
    let at_u: Vec<FaceKey> = incident_faces(body, u, rings)?
        .into_iter()
        .filter(|f| lineage.contains(f))
        .collect();
    Ok(super::sectors::sole_common_face(
        &at_u,
        &incident_faces(body, v, rings)?,
    ))
}

/// The edges of `body` interior to its contact patch, with their
/// endpoints: both sides on patch faces, and not on the seam. Arena
/// order. Each edge comes off the arena walk, so its halves, and their
/// faces ([`Body::face_of_linked`]), are links, and a miss panics
/// ([`crate::live::OPERATORS_KEEP_LINKS`]).
fn interior_edges<T: Decide>(
    body: &Body<T>,
    patch: &[FaceKey],
    seam: &SeamSet,
) -> Result<Vec<(EdgeKey, VertexKey, VertexKey)>, BooleanError> {
    let in_patch = |he| patch.contains(&body.face_of_linked(he));
    let mut out = Vec::new();
    for (key, edge) in body.edges() {
        if seam.set.contains_key(key) || !in_patch(edge.he_plus) || !in_patch(edge.he_minus) {
            continue;
        }
        let ends = |he, slot| {
            linked(
                &body.half_edges,
                he,
                EntityId::HalfEdge,
                EntityId::Edge(key),
                slot,
            )
            .start
        };
        out.push((
            key,
            ends(edge.he_plus, "he_plus"),
            ends(edge.he_minus, "he_minus"),
        ));
    }
    Ok(out)
}

/// **The two operands' interior curve networks, made congruent.** The
/// seam bounds the contact region on both solids alike, but each solid
/// divides the region into faces its own way: a full-turn bore is one
/// face where the shaft against it is three. Patches pair by vertex
/// cycles, so every edge interior to the OTHER solid's patch is given
/// a twin in `body`'s: found between the corresponding vertices, or
/// minted in the patch face holding both, on the other edge's carrier.
/// The zip removes both patches as interior, so what the twins must get
/// right is how they divide the faces.
///
/// Returns the patch grown by the faces the twins split off.
/// `Ok(None)`: an interior edge whose ends have no counterpart here (a
/// vertex of the other solid interior to the region), a closed one, or
/// one whose host is not a single patch face — not this lane's frontier.
#[allow(clippy::too_many_arguments)] // both solids, the vertex map, and the split bookkeeping `mint_chord` takes
fn mirror_edges<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    other: &Body<T>,
    other_interior: &[(EdgeKey, VertexKey, VertexKey)],
    here: &SecondaryMap<VertexKey, VertexKey>,
    patch: &[FaceKey],
    rings: &SecondaryMap<VertexKey, FaceKey>,
    fragments: &mut Vec<(FaceKey, FaceKey)>,
    tol: Tol,
) -> Result<Option<Vec<FaceKey>>, BooleanError> {
    let mut patch = patch.to_vec();
    for &(edge, ou, ov) in other_interior {
        let (Some(&u), Some(&v)) = (here.get(ou), here.get(ov)) else {
            return Ok(None);
        };
        if u == v {
            return Ok(None);
        }
        if joined(body, u, v)? {
            continue;
        }
        let Some(host) = super::sectors::sole_common_face(
            &incident_faces(body, u, rings)?,
            &incident_faces(body, v, rings)?,
        ) else {
            return Ok(None);
        };
        if !patch.contains(&host) {
            return Ok(None);
        }
        let minted = fragments.len();
        let twin = Twin::of(other, edge, (ou, ov), (u, v))?;
        mint_chord(body, host, u, v, twin.as_ref(), fragments, tol)?;
        patch.extend(fragments[minted..].iter().map(|&(new, _)| new));
    }
    Ok(Some(patch))
}

/// Whether an edge of `body` joins `u` to `v` (structural fan walk —
/// zero numerics). An isolated ring vertex has an empty orbit.
fn joined<T: Decide>(body: &Body<T>, u: VertexKey, v: VertexKey) -> Result<bool, BooleanError> {
    if body.get_vertex(u).is_none() {
        return Err(desync(
            "REST lane: a mirrored edge's end no longer resolves",
        ));
    }
    Ok(body
        .vertex_orbit_linked(u)
        .into_iter()
        .any(|he| body.proven_half_edge_end(he) == v))
}

/// The other solid's edge between the vertices a chord joins, read for
/// the chord: a line (the straight chord IS its locus) or a circle, with
/// its parameter span and which of this solid's two vertices that span
/// starts at.
enum Twin<T: geom_core::Real> {
    Line,
    Circle {
        carrier: geom::Curve3<T>,
        t0: T,
        t1: T,
        start: VertexKey,
    },
}

impl<T: Decide> Twin<T> {
    /// The other solid's `edge` from `ou` to `ov`, read for this
    /// solid's `u` and `v` (`ou`, `ov` correspond to them). An edge whose
    /// curve is uncertified, or neither a line nor a circle, refuses
    /// typed: no chord this lane can mint is its twin. No union reaches
    /// that refusal today: an ellipse or spline seam edge comes from a
    /// curved face's boundary, where the crossing layer answers `Unread`
    /// first, or from an oblique planar cut, whose body the containment
    /// door refuses `VolumeUncertified` first (an obliquely capped rod
    /// resting on a plate, declared `Rest`, in both operand orders); an
    /// uncertified edge is turned away at the operand gate. An edge that
    /// does not join `ou` to `ov` is a kernel bug.
    fn of(
        other: &Body<T>,
        edge: EdgeKey,
        (ou, ov): (VertexKey, VertexKey),
        (u, v): (VertexKey, VertexKey),
    ) -> Result<Option<Self>, BooleanError> {
        let ed = other
            .get_edge(edge)
            .ok_or_else(|| desync("REST lane: twin edge no longer resolves"))?;
        let ends = |he, slot| {
            linked(
                &other.half_edges,
                he,
                EntityId::HalfEdge,
                EntityId::Edge(edge),
                slot,
            )
            .start
        };
        let start = match (ends(ed.he_plus, "he_plus"), ends(ed.he_minus, "he_minus")) {
            found if found == (ou, ov) => u,
            found if found == (ov, ou) => v,
            _ => return Err(desync("REST lane: a chord's twin does not join its ends")),
        };
        let curve = other
            .edge_curve_linked(edge, ed)
            .certified()
            .ok_or_else(|| unsupported(RestZipFrontier::TwinCarrierUnsupported))?;
        let (t0, t1) = curve.params();
        match curve.carrier() {
            geom::Curve3::Line { .. } => Ok(Some(Self::Line)),
            carrier @ geom::Curve3::Circle { .. } => Ok(Some(Self::Circle {
                carrier: carrier.clone(),
                t0,
                t1,
                start,
            })),
            _ => Err(unsupported(RestZipFrontier::TwinCarrierUnsupported)),
        }
    }

    /// The twin's curve run from `from`: `None` for a line, which the
    /// straight chord mints as it stands; a circle's own arc in the
    /// direction asked ([`geom::Curve3::reversed`] runs it back).
    fn spec(&self, from: VertexKey) -> Option<EdgeCurveSpec<T>> {
        let Self::Circle {
            carrier,
            t0,
            t1,
            start,
        } = self
        else {
            return None;
        };
        if from == *start {
            EdgeCurveSpec::arc_of_circle(carrier.clone(), *t0, *t1)
        } else {
            EdgeCurveSpec::arc_of_circle(carrier.reversed()?, -*t1, -*t0)
        }
    }
}

/// Mints the seam chord `u → v` across `face`, which holds both,
/// through the standard splitting machinery (`mef` same-loop, `mekr`
/// for ring loops / pierce-ring vertices), on its `twin`'s carrier where
/// the other solid has the edge (a rim arc stays an arc), and as a
/// straight chord where it has none.
fn mint_chord<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    face: FaceKey,
    u: VertexKey,
    v: VertexKey,
    twin: Option<&Twin<T>>,
    fragments: &mut Vec<(FaceKey, FaceKey)>,
    tol: Tol,
) -> Result<EdgeKey, BooleanError> {
    let hu = halves_at(body, face, u)?;
    let hv = halves_at(body, face, v)?;
    // `halves_at` resolved `face` above, and nothing writes the body
    // before `mef` or `mekr`, so the face is proven until then and its
    // rings are links.
    let ring_loop_of = |body: &Body<T>, w: VertexKey| -> Option<LoopKey> {
        let f = proven(&body.faces, face, EntityId::Face);
        f.rings.iter().copied().find(|&l| {
            matches!(
                loop_boundary(body, l, EntityId::Face(face), "rings"),
                LoopBoundary::Empty { vertex } if vertex == w
            )
        })
    };
    let loop_of = |body: &Body<T>, he: HalfEdgeKey| {
        proven(&body.half_edges, he, EntityId::HalfEdge).parent_loop
    };
    // The new edge's curve from `from` to `to`, where the twin states
    // one; `None` takes the straight chord.
    // The new edge's curve from `from`; `None` takes the straight chord.
    let spec = |from: VertexKey| twin.and_then(|t| t.spec(from));
    let mef = |body: &mut Body<T>, he1: HalfEdgeKey, he2: HalfEdgeKey, from| {
        let site = MefSite::Chords { he1, he2 };
        match spec(from) {
            Some(curve) => body.mef(site, curve, FaceSurface::Inherit, tol),
            None => body.mef_chord(site, tol),
        }
    };
    let mekr = |body: &mut Body<T>, site: MekrSite, from| match spec(from) {
        Some(curve) => body.mekr(site, curve, tol),
        None => body.mekr_chord(site, tol),
    };
    let created = match (&hu[..], &hv[..]) {
        ([hu], [hv]) => {
            let (lu, lv) = (loop_of(body, *hu), loop_of(body, *hv));
            if lu == lv {
                let created = mef(body, *hu, *hv, u)
                    .map_err(|_| unsupported(RestZipFrontier::ChordMefRefused))?;
                fragments.push((created.face, face));
                created.edge
            } else {
                // Outer ↔ ring (or ring ↔ ring): mekr absorbs the ring.
                let outer = proven(&body.faces, face, EntityId::Face).outer;
                let ((target, from), ring) = if lv == outer {
                    ((*hv, v), *hu)
                } else {
                    ((*hu, u), *hv)
                };
                mekr(body, MekrSite::Cycles { target, ring }, from)
                    .map_err(|_| unsupported(RestZipFrontier::ChordMekrRefused))?
                    .edge
            }
        }
        ([], [hv]) => {
            let ring = ring_loop_of(body, u)
                .ok_or_else(|| unsupported(RestZipFrontier::ChordEndpointAbsent))?;
            mekr(body, MekrSite::EmptyRing { target: *hv, ring }, v)
                .map_err(|_| unsupported(RestZipFrontier::PierceRingMekrRefused))?
                .edge
        }
        ([hu], []) => {
            let ring = ring_loop_of(body, v)
                .ok_or_else(|| unsupported(RestZipFrontier::ChordEndpointAbsent))?;
            mekr(body, MekrSite::EmptyRing { target: *hu, ring }, u)
                .map_err(|_| unsupported(RestZipFrontier::PierceRingMekrRefused))?
                .edge
        }
        ([], []) => {
            return Err(unsupported(RestZipFrontier::ChordBetweenIsolatedPierces));
        }
        _ => {
            return Err(unsupported(RestZipFrontier::ChordEndpointRevisited));
        }
    };
    Ok(created)
}

/// The faces incident to `u` ([`super::sectors::faces_at`]); a
/// pierce-ring vertex joined to nothing contributes its host face.
fn incident_faces<T: Decide>(
    body: &Body<T>,
    u: VertexKey,
    rings: &SecondaryMap<VertexKey, FaceKey>,
) -> Result<Vec<FaceKey>, BooleanError> {
    let faces = super::sectors::faces_at(body, u).map_err(super::sectors::stale_site)?;
    Ok(if faces.is_empty() {
        rings.get(u).copied().into_iter().collect()
    } else {
        faces
    })
}

/// The face-boundary halves of `face` starting at `u` (outer + rings).
///
/// # Panics
///
/// Where a boundary hop past `face` (a loop, a member's edge, a lone vertex's
/// point) does not resolve, or a loop walk does not close
/// ([`crate::live::NAMES_ONLY_LIVE`] / [`crate::body::WALKS_CLOSE`];
/// [`crate::live::OPERATORS_KEEP_LINKS`]).
fn halves_at<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    u: VertexKey,
) -> Result<Vec<HalfEdgeKey>, BooleanError> {
    let f = body
        .get_face(face)
        .ok_or_else(|| desync("REST lane: chord host face vanished"))?;
    let mut out = Vec::new();
    for member in body.face_boundary_linked(face, f) {
        let BoundaryMember::Edge { he, half, .. } = member else {
            continue;
        };
        if half.start == u {
            out.push(he);
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------
// 5. Patch discovery + pairing.
// ---------------------------------------------------------------

/// The contact-patch faces of one solid: the seam partitions the
/// face-adjacency graph; a region qualifies iff every face's surface
/// is a verified REST-contact surface AND the region touches the
/// seam. `Ok(None)`: no qualifying region — not this frontier.
fn patch_faces<T: Decide>(
    body: &Body<T>,
    seam: &SeamSet,
    rest: &SecondaryMap<SurfaceKey, ()>,
) -> Result<Option<Vec<FaceKey>>, BooleanError> {
    let mut assigned: SecondaryMap<FaceKey, ()> = SecondaryMap::new();
    let mut patch: Vec<FaceKey> = Vec::new();
    let mut found = false;
    let all_faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    for &root in &all_faces {
        if assigned.contains_key(root) {
            continue;
        }
        // Flood this region (DFS worklist, deterministic arena-seeded
        // order; membership is order-independent).
        let mut region = vec![root];
        assigned.insert(root, ());
        let mut queue = vec![root];
        let mut touches_seam = false;
        while let Some(f) = queue.pop() {
            let fd = proven(&body.faces, f, EntityId::Face);
            for l in core::iter::once(fd.outer).chain(fd.rings.iter().copied()) {
                let LoopBoundary::Cycle { first } =
                    loop_boundary(body, l, EntityId::Face(f), "loops")
                else {
                    continue;
                };
                for he in cycle(body, first) {
                    let mate = body.proven_mate(he, Proven);
                    if seam.set.contains_key(mate.edge) {
                        touches_seam = true;
                        continue;
                    }
                    let nf = body.face_of_linked(mate.mate);
                    if !assigned.contains_key(nf) {
                        assigned.insert(nf, ());
                        region.push(nf);
                        queue.push(nf);
                    }
                }
            }
        }
        // Every region face came off the arena walk or a link.
        let qualified = region
            .iter()
            .all(|&f| rest.contains_key(proven(&body.faces, f, EntityId::Face).surface));
        if qualified && touches_seam {
            found = true;
            patch.extend(region);
        }
    }
    if !found {
        return Ok(None);
    }
    Ok(Some(patch))
}

/// The outer-cycle start vertices of a face.
fn cycle_starts<T: Decide>(body: &Body<T>, face: FaceKey) -> Result<Vec<VertexKey>, BooleanError> {
    let f = body
        .get_face(face)
        .ok_or_else(|| desync("REST lane: cycle face vanished"))?;
    let LoopBoundary::Cycle { first } = loop_boundary(body, f.outer, EntityId::Face(face), "outer")
    else {
        return Err(desync("REST lane: patch outer loop is empty"));
    };
    Ok(cycle(body, first)
        .into_iter()
        .map(|he| proven(&body.half_edges, he, EntityId::HalfEdge).start)
        .collect())
}

/// Pairs the patch faces across the mate by exact antiparallel vertex-
/// cycle congruence through the seam correspondence — verified, never
/// assumed. Failures after this point in the pipeline are lane-owned.
fn pair_patches<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    a_patch: &[FaceKey],
    b_patch: &[FaceKey],
    vcorr: &SecondaryMap<VertexKey, VertexKey>,
) -> Result<Vec<(FaceKey, FaceKey)>, BooleanError> {
    let mut used: SecondaryMap<FaceKey, ()> = SecondaryMap::new();
    let mut pairs = Vec::with_capacity(a_patch.len());
    for &fa in a_patch {
        let starts = cycle_starts(a, fa)?;
        let mapped: Vec<VertexKey> = starts
            .iter()
            .map(|&v| {
                vcorr
                    .get(v)
                    .copied()
                    .ok_or_else(|| unsupported(RestZipFrontier::PatchVertexUnmatched))
            })
            .collect::<Result<_, _>>()?;
        let n = mapped.len();
        let mut matched = None;
        'cand: for &fb in b_patch {
            if used.contains_key(fb) {
                continue;
            }
            let bs = cycle_starts(b, fb)?;
            if bs.len() != n {
                continue;
            }
            let Some(idx) = bs.iter().position(|&w| w == mapped[0]) else {
                continue;
            };
            // Antiparallel congruence: B walks the mapped cycle in
            // reverse.
            for (t, m) in mapped.iter().enumerate() {
                if bs[(idx + n - (t % n)) % n] != *m {
                    continue 'cand;
                }
            }
            matched = Some(fb);
            break;
        }
        let Some(fb) = matched else {
            return Err(unsupported(RestZipFrontier::PatchCyclesIncongruent));
        };
        used.insert(fb, ());
        pairs.push((fa, fb));
    }
    Ok(pairs)
}

/// BFS glue order over the A-side patch adjacency (regions rooted in
/// arena order): a pair sharing one contiguous already-fused run with
/// the glued set takes the slit zip's fold; a pair sharing several
/// (the closed cosurface band's last panel — a CYCLIC patch graph)
/// takes the same zip's band closure, which kills the later runs by
/// the configuration each is found in.
fn bfs_order<T: Decide>(
    body: &Body<T>,
    patch: &[FaceKey],
    seam: &SeamSet,
) -> Result<Vec<FaceKey>, BooleanError> {
    let in_patch: SecondaryMap<FaceKey, ()> = patch.iter().map(|&f| (f, ())).collect();
    let mut visited: SecondaryMap<FaceKey, ()> = SecondaryMap::new();
    let mut order = Vec::with_capacity(patch.len());
    for &root in patch {
        if visited.contains_key(root) {
            continue;
        }
        visited.insert(root, ());
        let mut queue = std::collections::VecDeque::from([root]);
        while let Some(f) = queue.pop_front() {
            order.push(f);
            let fd = body
                .get_face(f)
                .ok_or_else(|| desync("REST lane: BFS face vanished"))?;
            let LoopBoundary::Cycle { first } =
                loop_boundary(body, fd.outer, EntityId::Face(f), "outer")
            else {
                continue;
            };
            for he in cycle(body, first) {
                let mate = body.proven_mate(he, Proven);
                if seam.set.contains_key(mate.edge) {
                    continue;
                }
                let nf = body.face_of_linked(mate.mate);
                if in_patch.contains_key(nf) && !visited.contains_key(nf) {
                    visited.insert(nf, ());
                    queue.push_back(nf);
                }
            }
        }
    }
    Ok(order)
}

// ---------------------------------------------------------------
// 6. The slit zip (patch pairs adjacent along an already-fused run).
// ---------------------------------------------------------------

/// The edges shared between the two faces' outer cycles (already-fused
/// seam runs), in `fa`-cycle order.
fn shared_run<T: Decide>(
    body: &Body<T>,
    fa: FaceKey,
    fb: FaceKey,
) -> Result<Vec<EdgeKey>, BooleanError> {
    let cycle_edges = |f: FaceKey| -> Result<Vec<EdgeKey>, BooleanError> {
        let fd = body
            .get_face(f)
            .ok_or_else(|| desync("REST lane: glue face vanished"))?;
        let LoopBoundary::Cycle { first } =
            loop_boundary(body, fd.outer, EntityId::Face(f), "outer")
        else {
            return Err(desync("REST lane: glue face outer loop is empty"));
        };
        Ok(cycle(body, first)
            .into_iter()
            .map(|he| proven(&body.half_edges, he, EntityId::HalfEdge).edge)
            .collect())
    };
    let ea = cycle_edges(fa)?;
    let eb = cycle_edges(fb)?;
    let eb_set: SecondaryMap<EdgeKey, ()> = eb.iter().map(|&e| (e, ())).collect();
    Ok(ea.into_iter().filter(|e| eb_set.contains_key(*e)).collect())
}

/// Glues one patch pair, rings included. A multiply-connected patch
/// face's interior boundaries (rings — e.g. the peg-root rims inside
/// a mating plane) are each their own antiparallel-congruent cycle
/// pair across the mate: every ring on both sides is PROMOTED to a
/// transient face first (`mfkrh`), the outer pair glues through the
/// seam zip or the slit zip (as the already-fused runs dictate), and
/// the promoted pairs then glue the same way — the same-shell
/// `kfmrh` inside those glues is where the mate's genus bookkeeping
/// lives (a filled through-peg's handle).
///
/// Rings pair one for one, so differing counts are holes that do not
/// match, refused as a ring with no congruent partner is.
///
/// On a PLANE a ring vertex always has a seam correspondent: the patch
/// flood stops only at seam edges, so a ring is bounded by seam edges,
/// whose ends are segment ends, or encloses another patch face whose
/// outer cycle the patch pairing has already mapped (the ring is that
/// face's outer boundary, by Jordan). Neither half holds on a periodic
/// carrier: a cylinder band's two boundary circles are outer and ring by
/// designation only, so a ring can border a neighbouring patch face's
/// RING, whose vertices the pairing never reads (two stacked bands mated
/// against a band split at another height). The unmatched-vertex
/// refusal answers that configuration; no row builds it yet. A ring
/// that is an isolated vertex (a pierce ring no segment reached) is
/// outside both arguments: the flood passes over it, and its promotion
/// or its cycle read below refuses as a lane desync.
fn glue_pair<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    fa: FaceKey,
    fb: FaceKey,
    vmap: &SecondaryMap<VertexKey, VertexKey>,
    tol: Tol,
) -> Result<ZipReport, BooleanError> {
    let corr: SeamCorrespondence = vmap
        .iter()
        .map(|(a, &b)| (a, std::collections::BTreeSet::from([b])))
        .collect();
    let rings_of = |body: &Body<T>, f: FaceKey| -> Result<Vec<LoopKey>, BooleanError> {
        Ok(body
            .get_face(f)
            .ok_or_else(|| desync("REST lane: glue face vanished"))?
            .rings
            .clone())
    };
    let mut ga: Vec<FaceKey> = Vec::new();
    let mut gb: Vec<FaceKey> = Vec::new();
    for r in rings_of(body, fa)? {
        ga.push(
            body.mfkrh(r, FaceSurface::Inherit)
                .map_err(|_| desync("REST lane: ring promotion refused"))?
                .face,
        );
    }
    for r in rings_of(body, fb)? {
        gb.push(
            body.mfkrh(r, FaceSurface::Inherit)
                .map_err(|_| desync("REST lane: ring promotion refused"))?
                .face,
        );
    }
    if ga.len() != gb.len() {
        return Err(unsupported(RestZipFrontier::HoleCyclesIncongruent));
    }
    let shared = shared_run(body, fa, fb)?;
    let mut report = if shared.is_empty() {
        zip_seam(body, fa, fb, &corr, tol)?
    } else {
        slit_zip(body, fa, fb, &shared, vmap, tol)?
    };
    // Pair the promoted transients by exact antiparallel vertex-cycle
    // congruence through the seam correspondence (the same test the
    // patch pairing ran on the outers) and glue each pair.
    let mut used: SecondaryMap<FaceKey, ()> = SecondaryMap::new();
    for &da in &ga {
        let starts = cycle_starts(body, da)?;
        let mapped: Vec<VertexKey> = starts
            .iter()
            .map(|&v| {
                vmap.get(v)
                    .copied()
                    .ok_or_else(|| unsupported(RestZipFrontier::HoleVertexUnmatched))
            })
            .collect::<Result<_, _>>()?;
        let n = mapped.len();
        let mut matched = None;
        'cand: for &db in &gb {
            if used.contains_key(db) {
                continue;
            }
            let bs = cycle_starts(body, db)?;
            if bs.len() != n {
                continue;
            }
            let Some(idx) = bs.iter().position(|&w| w == mapped[0]) else {
                continue;
            };
            for (t, m) in mapped.iter().enumerate() {
                if bs[(idx + n - (t % n)) % n] != *m {
                    continue 'cand;
                }
            }
            matched = Some(db);
            break;
        }
        let Some(db) = matched else {
            return Err(unsupported(RestZipFrontier::HoleCyclesIncongruent));
        };
        used.insert(db, ());
        let shared = shared_run(body, da, db)?;
        let rep = if shared.is_empty() {
            zip_seam(body, da, db, &corr, tol)?
        } else {
            slit_zip(body, da, db, &shared, vmap, tol)?
        };
        report
            .vertex_merges
            .extend(rep.vertex_merges.iter().copied());
        report.seam_edges.extend(rep.seam_edges.iter().copied());
        report
            .interior_edges
            .extend(rep.interior_edges.iter().copied());
    }
    Ok(report)
}

/// Glues one patch pair adjacent along ALREADY-FUSED seam runs: the
/// run edges die (they are interior to the contact region R), the
/// remaining coincident edge pairs fuse to the surviving A copies,
/// the remaining coincident vertex pairs fuse, both faces die. The
/// same loopglue scaffolding discipline as [`zip_seam`] (self-loop
/// scaffolding edges between bitwise-coincident vertices, `kev`
/// fusions, `kef` retirements), driven along the folded loop the run
/// kef leaves behind.
///
/// **Multiple disjoint runs are the band-closure case** (a closed
/// cosurface band's last panel shares a run on each side): the first
/// run folds the mate in through `kef`; each later run's edges then
/// lie WITHIN the folded face's own loops and are killed by the
/// configuration each is found in — dangling (`kev`), doubled in one
/// cycle (`kemr`, which mints a ring), or spanning two loops of the
/// one face (`mfkrh`-then-`kef`, the kernel's own prescription for
/// that shape). Every ring the kills leave behind is promoted to its
/// own transient face (`mfkrh`) and zipped by the same folded-loop
/// zipper that finishes the outer cycle — the genus drop of closing a
/// band lives in those promotions, never in ad-hoc surgery.
///
/// **Every edge it kills is certified**: the operands arrive at rest,
/// where tier 2 admits no null edge, and [`undo_struts`] has killed
/// every null edge the reduction minted. So the keys-only `kev` and
/// `kemr` here release no loop a null edge held open, and never refuse
/// `KeysOnly`.
fn slit_zip<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    fa: FaceKey,
    fb: FaceKey,
    shared: &[EdgeKey],
    vmap: &SecondaryMap<VertexKey, VertexKey>,
    tol: Tol,
) -> Result<ZipReport, BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    let mut report = ZipReport::default();
    // fb's cycle edges = the b side (dies); fa's = the a side
    // (survives). Snapshot before surgery.
    let cycle_halves = |body: &Body<T>, f: FaceKey| -> Result<Vec<HalfEdgeKey>, BooleanError> {
        let fd = body
            .get_face(f)
            .ok_or_else(|| desync("REST lane: slit face vanished"))?;
        if !fd.rings.is_empty() {
            return Err(unsupported(RestZipFrontier::SlitFaceHoles));
        }
        let LoopBoundary::Cycle { first } =
            loop_boundary(body, fd.outer, EntityId::Face(f), "outer")
        else {
            return Err(desync("REST lane: slit face outer loop is empty"));
        };
        Ok(cycle(body, first))
    };
    let oa = cycle_halves(body, fa)?;
    let ob = cycle_halves(body, fb)?;
    if oa.len() != ob.len() {
        return Err(corr("slit-zip cycles differ in length"));
    }
    let shared_set: SecondaryMap<EdgeKey, ()> = shared.iter().map(|&e| (e, ())).collect();
    let flags: Vec<bool> = oa
        .iter()
        .map(|&he| Ok(shared_set.contains_key(edge_of(body, he)?)))
        .collect::<Result<_, BooleanError>>()?;
    let k = flags.iter().filter(|&&s| s).count();
    if k != shared.len() || k == flags.len() {
        return Err(unsupported(RestZipFrontier::WholeBoundaryShared));
    }
    let n = flags.len();
    // Rotate so a run occupies a prefix: find i with flags[i] &&
    // !flags[(i+n-1)%n], then collect every maximal run in cycle
    // order from there (deterministic — D9).
    let Some(start) = (0..n).find(|&i| flags[i] && !flags[(i + n - 1) % n]) else {
        return Err(unsupported(RestZipFrontier::WholeBoundaryShared));
    };
    let mut runs: Vec<Vec<HalfEdgeKey>> = Vec::new();
    let mut t = 0;
    while t < n {
        let i = (start + t) % n;
        if flags[i] {
            let mut run = vec![oa[i]];
            t += 1;
            while t < n && flags[(start + t) % n] {
                run.push(oa[(start + t) % n]);
                t += 1;
            }
            runs.push(run);
        } else {
            t += 1;
        }
    }

    let a_edges: SecondaryMap<EdgeKey, ()> = oa
        .iter()
        .map(|&he| Ok((edge_of(body, he)?, ())))
        .collect::<Result<_, BooleanError>>()?;
    let b_edges: SecondaryMap<EdgeKey, ()> = ob
        .iter()
        .map(|&he| Ok((edge_of(body, he)?, ())))
        .collect::<Result<_, BooleanError>>()?;

    // ---- Kill the first run: kef the first run edge from the fb side
    // (kills fb, merges the cycles into fa's folded loop); each
    // further run edge then dangles at a dead run-interior vertex —
    // kev it (the interior vertex dies with it, as R-interior
    // structure must). ----
    let run = &runs[0];
    let first_run_edge = edge_of(body, run[0])?;
    report.interior_edges.push(first_run_edge);
    let fb_half = {
        let ed = proven(&body.edges, first_run_edge, EntityId::Edge);
        if ed.he_plus == run[0] {
            ed.he_minus
        } else {
            ed.he_plus
        }
    };
    body.kef_minting(fb_half, tol)
        .map_err(|_| desync("REST lane: run kef refused"))?;
    for &he in run.iter().skip(1) {
        // The shared vertex with the previous (now dead) run edge is
        // this half's START (run halves run start→end along fa's
        // cycle; the previous edge ended where this one starts).
        report.interior_edges.push(edge_of(body, he)?);
        let hd = body
            .get_half_edge(he)
            .ok_or_else(|| desync("REST lane: run half no longer resolves"))?;
        let dead_end = hd.start;
        // The far vertex must hold ONLY this edge now (a T-junction
        // interior vertex is a sub-frontier, refused before surgery).
        let orbit = body.vertex_orbit_linked(dead_end);
        if orbit.is_empty() {
            return Err(desync("REST lane: run vertex lost its fan"));
        }
        if orbit.len() != 1 {
            return Err(unsupported(RestZipFrontier::RunVertexBranches));
        }
        let mate = body.proven_mate(he, Proven).mate;
        body.kev(mate)
            .map_err(|_| desync("REST lane: run kev refused"))?;
    }

    // ---- Later runs (the band closure): every edge now lies within
    // the folded face's own loop set; kill each by the configuration
    // it is found in. ----
    for run in runs.iter().skip(1) {
        for &he in run {
            let e = edge_of(body, he)?;
            report.interior_edges.push(e);
            let ed = body
                .get_edge(e)
                .ok_or_else(|| desync("REST lane: band run edge no longer resolves"))?
                .clone();
            let (h, m) = (ed.he_plus, ed.he_minus);
            let loop_of = |body: &Body<T>, half, slot| {
                linked(
                    &body.half_edges,
                    half,
                    EntityId::HalfEdge,
                    EntityId::Edge(e),
                    slot,
                )
                .parent_loop
            };
            let (lh, lm) = (loop_of(body, h, "he_plus"), loop_of(body, m, "he_minus"));
            if lh == lm {
                // Dangling (a valence-1 end) → kev that half; doubled
                // deeper in the cycle → kemr (the split-off side
                // becomes a ring, disposed below).
                let dangle_half = {
                    let valence = |body: &Body<T>, half| -> Result<usize, BooleanError> {
                        let orbit = body.vertex_orbit_linked(body.proven_half_edge_end(half));
                        if orbit.is_empty() {
                            return Err(desync("REST lane: band run vertex lost its fan"));
                        }
                        Ok(orbit.len())
                    };
                    if valence(body, h)? == 1 {
                        Some(h)
                    } else if valence(body, m)? == 1 {
                        Some(m)
                    } else {
                        None
                    }
                };
                match dangle_half {
                    Some(dh) => {
                        body.kev(dh)
                            .map_err(|_| desync("REST lane: band run kev refused"))?;
                    }
                    None => {
                        body.kemr(h, m)
                            .map_err(|_| desync("REST lane: band run kemr refused"))?;
                    }
                }
            } else {
                // Two loops of the ONE folded face: the kernel's own
                // prescription — promote the ring, then kef from the
                // promoted side (the remnant merges into the other
                // loop; the transient face dies with the edge).
                let fd = body
                    .get_face(fa)
                    .ok_or_else(|| desync("REST lane: folded face vanished"))?;
                let (ring_half, ring) = if fd.rings.contains(&lh) {
                    (h, lh)
                } else if fd.rings.contains(&lm) {
                    (m, lm)
                } else {
                    return Err(unsupported(RestZipFrontier::BandRunOffLoops));
                };
                body.mfkrh(ring, FaceSurface::Inherit)
                    .map_err(|_| desync("REST lane: band run mfkrh refused"))?;
                body.kef_minting(ring_half, tol)
                    .map_err(|_| desync("REST lane: band run kef refused"))?;
            }
        }
    }

    // ---- Dispose the rings the band kills left behind: promote each
    // to a transient face and zip it with the same folded-loop zipper
    // that finishes the outer cycle. ----
    let mut ring_steps = 0usize;
    loop {
        ring_steps += 1;
        if ring_steps > shared.len() + 2 {
            return Err(desync("REST lane: band ring disposal did not terminate"));
        }
        let ring = body
            .get_face(fa)
            .ok_or_else(|| desync("REST lane: folded face vanished"))?
            .rings
            .first()
            .copied();
        let Some(ring) = ring else { break };
        let created = body
            .mfkrh(ring, FaceSurface::Inherit)
            .map_err(|_| desync("REST lane: band ring mfkrh refused"))?;
        zip_folded(
            body,
            created.face,
            &a_edges,
            &b_edges,
            vmap,
            &mut report,
            tol,
        )?;
    }
    zip_folded(body, fa, &a_edges, &b_edges, vmap, &mut report, tol)?;
    Ok(report)
}

/// The edge of a half-edge (shared lookup for the zip family).
fn edge_of<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<EdgeKey, BooleanError> {
    Ok(body
        .get_half_edge(he)
        .ok_or_else(|| desync("REST lane: slit half no longer resolves"))?
        .edge)
}

/// The folded-loop zipper (the slit zip's finishing walk, shared with
/// the band closure's promoted transient faces): the face's outer
/// cycle holds interleaved a-side (surviving) and b-side (dying)
/// copies; per fold one scaffolding `mef` + `kev` fuses the vertex
/// pair and a `kef` retires the b copy, and the final coincident pair
/// retires face and b copy together (the a copy survives as a seam
/// edge, absorbed by the b-side neighbor's loop).
fn zip_folded<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    face: FaceKey,
    a_edges: &SecondaryMap<EdgeKey, ()>,
    b_edges: &SecondaryMap<EdgeKey, ()>,
    vmap: &SecondaryMap<VertexKey, VertexKey>,
    report: &mut ZipReport,
    tol: Tol,
) -> Result<(), BooleanError> {
    let corr = |what| BooleanError::ZipCorrespondence { what };
    let mut steps = 0usize;
    let cap = 2 * (a_edges.len() + b_edges.len()) + 2;
    loop {
        steps += 1;
        if steps > cap {
            return Err(desync("REST lane: slit zipper did not terminate"));
        }
        let fd = body
            .get_face(face)
            .ok_or_else(|| desync("REST lane: slit face vanished mid-zip"))?;
        let LoopBoundary::Cycle { first } =
            loop_boundary(body, fd.outer, EntityId::Face(face), "outer")
        else {
            return Err(desync("REST lane: slit loop emptied mid-zip"));
        };
        let walk = cycle(body, first);
        let edge_in = |he| proven(&body.half_edges, he, EntityId::HalfEdge).edge;
        if walk.len() == 2 {
            // The last coincident pair: kef the b copy from inside the
            // face (the face dies with it; the a copy survives as the
            // seam edge).
            let (e0, e1) = (edge_in(walk[0]), edge_in(walk[1]));
            let b_half = if b_edges.contains_key(e0) && a_edges.contains_key(e1) {
                walk[0]
            } else if b_edges.contains_key(e1) && a_edges.contains_key(e0) {
                walk[1]
            } else {
                return Err(corr("slit-zip final pair is not one copy per side"));
            };
            report
                .seam_edges
                .push(if b_edges.contains_key(e0) { e1 } else { e0 });
            body.kef_minting(b_half, tol)
                .map_err(|_| desync("REST lane: final slit kef refused"))?;
            break;
        }
        // Find the fold: an a-side half followed by a b-side half.
        let mut fold = None;
        for (i, &he) in walk.iter().enumerate() {
            let e = edge_in(he);
            let next = walk[(i + 1) % walk.len()];
            let en = edge_in(next);
            if a_edges.contains_key(e) && b_edges.contains_key(en) {
                fold = Some((he, next));
                break;
            }
        }
        let Some((ha, hb)) = fold else {
            return Err(corr("slit-zip fold not found"));
        };
        let sa = proven(&body.half_edges, ha, EntityId::HalfEdge).start;
        let eb = body.proven_half_edge_end(hb);
        if sa == eb {
            return Err(unsupported(RestZipFrontier::FoldVertexFused));
        }
        if vmap.get(sa).copied() != Some(eb) {
            return Err(corr("slit-zip vertex pair off the seam correspondence"));
        }
        let p = body.linked_vertex_point(sa, EntityId::HalfEdge(ha), "start");
        let hb_next = proven(&body.half_edges, hb, EntityId::HalfEdge).next;
        // Wall off the 3-edge sliver [ha, hb, scaffold], fuse the
        // vertex pair into the a copy, retire the b copy (its remnant a
        // copy lands in the b-side neighbor's loop — the fuse).
        let joint = Joint::Chord {
            he1: ha,
            he2: hb_next,
        };
        let (merge, _) = fuse_by_joint(body, joint, p, desync, tol)?;
        debug_assert_eq!(merge, (eb, sa), "the slit fuse keeps the a copy");
        report.vertex_merges.push(merge);
        report.seam_edges.push(edge_of(body, ha)?);
        body.kef_minting(hb, tol)
            .map_err(|_| desync("REST lane: slit pair kef refused"))?;
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **The witnesses and the chord halves panic on a ring link that
    /// does not resolve**, where they stepped over it.
    #[test]
    fn the_boundary_walks_panic_on_a_torn_ring_link() {
        use crate::live::OPERATORS_KEEP_LINKS;
        use crate::review_d18::{ROW_FOUR, assert_torn_op_panics, tear_ring};
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let face = body.faces().next().map(|(k, _)| k).unwrap();
        let u = body.vertices().next().map(|(k, _)| k).unwrap();
        assert!(face_witnesses(&body, face).is_some(), "the live face reads");
        assert!(halves_at(&body, face, u).is_ok(), "the live face walks");
        let named = tear_ring(&mut body, face);
        let premise = [named.as_str(), ROW_FOUR, OPERATORS_KEEP_LINKS];
        assert_torn_op_panics("face_witnesses", &mut body, &premise, |b| {
            face_witnesses(b, face)
        });
        assert_torn_op_panics("halves_at", &mut body, &premise, |b| halves_at(b, face, u));
    }

    /// **A glue's deaths settle against the seam, one glue at a time.**
    /// A seam edge that died and was reported interior leaves the seam;
    /// one that died unreported is the lane's desync; and an edge
    /// reported interior that is still alive is one too. The prism is a
    /// key scaffold: only edge liveness is read, so the death is an
    /// arena removal rather than a glue's surgery.
    ///
    /// This is the only coverage the unreported-death arm can have: a
    /// glue that reports every run edge it kills (as `slit_zip` does,
    /// pushing each before the kill) never feeds it one, so only a
    /// broken glue reaches it end to end. No suite row has a real glue
    /// kill a seam segment edge at all.
    #[test]
    fn a_glue_settles_each_seam_death_it_reports_and_no_other() {
        let prism = crate::fixtures::raw_prism(3, Tol::witness());
        let mut body = prism.body;
        let [kept, killed, other] = [prism.et[0], prism.et[1], prism.et[2]];
        body.edges.remove(killed);
        let what = |r: Result<(), BooleanError>| match r {
            Err(BooleanError::JoinDesync { what }) => what,
            other => panic!("expected a lane desync, got {other:?}"),
        };

        let mut seam = vec![kept, killed];
        settle_glue(&body, &mut seam, &[killed]).expect("a reported death settles");
        assert_eq!(seam, vec![kept], "the reported dead edge leaves the seam");

        let mut seam = vec![kept, killed];
        assert_eq!(
            what(settle_glue(&body, &mut seam, &[])),
            "REST lane: a seam segment edge did not survive",
            "an unreported seam death is a desync"
        );

        let mut seam = vec![kept, killed];
        assert_eq!(
            what(settle_glue(&body, &mut seam, &[killed, other])),
            "REST lane: an edge the glue reported interior survived it",
            "an interior report on a live edge is a desync"
        );
    }

    /// **The carrier reads answer a caller's stale face `None` and panic
    /// on a torn link past it.** A face the body once held, freed so the
    /// body around it is sound, reads no carrier, ball or witnesses; a
    /// live face whose surface was dropped panics in `face_carrier` and
    /// `face_ball` naming the surface, and one whose boundary vertex lost
    /// its point panics in `face_witnesses` naming the point. A read that
    /// took either tear for an absent record would answer instead.
    #[test]
    fn the_carrier_reads_answer_a_stale_face_none_and_panic_on_a_torn_link() {
        use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};
        let band = Band::linear(Tol::witness()).unwrap();
        let fresh = || crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let mut body = fresh();
        let (face, data) = body.faces().next().map(|(k, f)| (k, f.clone())).unwrap();
        let stale = body.faces.insert(data.clone());
        body.faces.remove(stale);
        assert!(
            face_carrier(&body, face).is_some()
                && face_ball(&body, face, band).is_some()
                && face_witnesses(&body, face).is_some(),
            "the live face reads"
        );
        assert!(face_carrier(&body, stale).is_none(), "face_carrier, stale");
        assert!(face_ball(&body, stale, band).is_none(), "face_ball, stale");
        assert!(
            face_witnesses(&body, stale).is_none(),
            "face_witnesses, stale"
        );

        body.surfaces.remove(data.surface);
        let surface = format!("{}'s surface names", EntityId::Face(face));
        assert_torn_op_panics("face_carrier", &mut body, &[&surface, ROW_FOUR], |b| {
            face_carrier(b, face)
        });
        assert_torn_op_panics("face_ball", &mut body, &[&surface, ROW_FOUR], |b| {
            face_ball(b, face, band)
        });

        let mut body = fresh();
        let LoopBoundary::Cycle { first } = body.get_loop(data.outer).unwrap().boundary else {
            panic!("the cube's faces are bounded by cycles");
        };
        let vertex = body.get_half_edge(first).unwrap().start;
        let point = body.get_vertex(vertex).unwrap().point;
        body.points.remove(point);
        let named = format!("{}'s point names", EntityId::Vertex(vertex));
        assert_torn_op_panics("face_witnesses", &mut body, &[&named, ROW_FOUR], |b| {
            face_witnesses(b, face)
        });
    }

    /// **Every open segment's ends are read before any is taken, and an
    /// end that does not resolve panics naming the undo's premise.** The
    /// first segment taken (both ends joined) lies in a face holding
    /// neither end, so alone it answers `Ok(None)`, the fallback to the
    /// join's own refusal; beside it, a segment whose ends do not
    /// resolve, which would be taken after it, panics with the body
    /// unchanged. A lane that read an end only once its segment was
    /// taken would answer `Ok(None)`, and one that read a miss as
    /// unjoined would refuse at the edge as not joining its ends.
    #[test]
    fn a_stale_end_panics_before_the_first_segment_is_taken() {
        use crate::review_d18::assert_torn_op_panics;
        let prism = crate::fixtures::raw_prism(3, Tol::witness());
        let mut body = prism.body;
        let data = body.get_vertex(prism.u[0]).unwrap().clone();
        let stale = body.vertices.insert(data);
        body.vertices.remove(stale);
        let other = body.clone();
        let taken_first = || Span {
            ends: (prism.u[0], prism.u[1]),
            cell: Locus::InFace(prism.face_top),
            theirs: (prism.u[0], prism.u[1]),
            twin: None,
        };
        let stale_ends = || Span {
            ends: (stale, prism.u[0]),
            cell: Locus::OnEdge(prism.et[0]),
            theirs: (stale, prism.u[0]),
            twin: None,
        };
        let realize = |body: &mut Body<f64>, spans: Vec<Span>| {
            realize_seam(
                body,
                &other,
                &spans,
                &SecondaryMap::new(),
                &mut Vec::new(),
                Tol::witness(),
            )
            .map(|seam| seam.map(|s| s.per_segment))
        };
        let alone = realize(&mut body, vec![taken_first()]);
        assert!(
            matches!(alone, Ok(None)),
            "the first segment alone: {alone:?}"
        );
        let named = format!("segment end {} does not resolve", EntityId::Vertex(stale));
        for (label, spans) in [
            ("a stale segment alone", vec![stale_ends()]),
            ("beside a stale segment", vec![taken_first(), stale_ends()]),
        ] {
            assert_torn_op_panics(label, &mut body, &[&named, SEGMENT_ENDS_SURVIVE], |b| {
                realize(b, spans)
            });
        }
    }

    /// The body of `red`'s `operand`.
    fn body_of(red: &mut BooleanReduction<f64>, operand: Operand) -> &mut Body<f64> {
        match operand {
            Operand::A => &mut red.a,
            Operand::B => &mut red.b,
        }
    }

    /// The reduction of a prism whose corner rests on the apex of a
    /// notched block holding a wedge in its notch (the two touching
    /// along the apex line), the prism `nest` (A or B): in the prism's
    /// corner the wedge pair's strut hangs at the tip of the notch
    /// pair's. With the notch strut's record and the wedge strut's.
    fn nested(
        nest: Operand,
        tol: Tol,
    ) -> (
        BooleanReduction<f64>,
        crate::boolean::BoolNullEdgeRecord<f64>,
        crate::boolean::BoolNullEdgeRecord<f64>,
    ) {
        use crate::test_support::{finished, flush_declarations, prism_z};
        let at = |deg: f64, r: f64| (r * deg.to_radians().cos(), r * deg.to_radians().sin());
        let mx = 1.5 / 60f64.to_radians().tan();
        let piece = |what, profile: &[(f64, f64)], z: (f64, f64)| {
            finished(what, prism_z::<f64>(profile, z.0, z.1, tol).body, tol)
        };
        let block = piece(
            "the notched block",
            &[
                (-1.0, -1.0),
                (1.0, -1.0),
                (1.0, 1.5),
                (mx, 1.5),
                (0.0, 0.0),
                (-mx, 1.5),
                (-1.0, 1.5),
            ],
            (0.0, 1.0),
        );
        let wedge = piece(
            "the wedge",
            &[(0.0, 0.0), at(75.0, 1.6), at(105.0, 1.6)],
            (0.0, 1.0),
        );
        let decls = flush_declarations(&block, &wedge, tol);
        let Ok(BooleanResult::Body(pinch)) = crate::union_with(&block, &wedge, &decls, tol) else {
            panic!("the wedge folds into the notch");
        };
        let top = piece(
            "the prism",
            &[(0.0, 0.0), (1.0, 1.4), (-1.0, 1.4)],
            (1.0, 2.0),
        );
        let carried = pinch
            .contacts
            .vv
            .iter()
            .map(|&pair| crate::CarriedVv {
                pair,
                class: crate::contact::ContactClass::Rest,
            })
            .collect();
        let pinch = pinch.body;
        let (a, b) = match nest {
            Operand::A => (&top, &pinch),
            Operand::B => (&pinch, &top),
        };
        let mut decls = flush_declarations(a, b, tol);
        match nest {
            Operand::A => decls.carried_b.vv = carried,
            Operand::B => decls.carried_a.vv = carried,
        }
        let red = crate::boolean_reduce_declared(BooleanOp::Union, a, b, &decls, tol).unwrap();

        let copy =
            |r: &crate::boolean::BoolNullEdgeRecord<f64>| r.attr.copy_at(r.at_vertex).unwrap();
        let (outer, inner) = red
            .null_edges
            .iter()
            .find_map(|o| {
                let i = red
                    .null_edges
                    .iter()
                    .find(|i| i.operand == o.operand && i.at_vertex == copy(o))?;
                Some((*o, *i))
            })
            .expect("the prism's corner nests the wedge pair's strut in the notch pair's");
        assert_eq!(outer.operand, nest, "the nest is the prism's");
        (red, outer, inner)
    }

    /// [`nested`]'s reduction, a third strut hung by hand at the wedge
    /// strut's tip, its record naming `site(c1, c2)` as its site; the
    /// notch strut's site; and the three copies, outermost first.
    fn hung_three_deep(
        nest: Operand,
        site: fn(VertexKey, VertexKey) -> VertexKey,
        tol: Tol,
    ) -> (BooleanReduction<f64>, VertexKey, [VertexKey; 3]) {
        let (mut red, outer, inner) = nested(nest, tol);
        let copy =
            |r: &crate::boolean::BoolNullEdgeRecord<f64>| r.attr.copy_at(r.at_vertex).unwrap();
        let (c1, c2) = (copy(&outer), copy(&inner));
        let body = body_of(&mut red, nest);
        let emanating = body.get_vertex(c2).unwrap().emanating.unwrap();
        let hand = body
            .mev_null(
                crate::euler::MevSite::Fan {
                    he1: emanating,
                    he2: emanating,
                },
                crate::null::NewVertexSide::Above,
            )
            .unwrap();
        let mut deeper = inner;
        deeper.edge = hand.edge;
        deeper.at_vertex = site(c1, c2);
        deeper.attr = crate::null::NullEdge {
            below_end: deeper.at_vertex,
            above_end: hand.vertex,
        };
        red.null_edges.push(deeper);
        (red, outer.at_vertex, [c1, c2, hand.vertex])
    }

    /// **Every strut site reads, through the undo's fusions, as a
    /// vertex the undo left standing, to two hops, in either operand.**
    /// [`nested`]'s reduction, with the prism as A and as B, and a third
    /// strut hung by hand at the inner one's tip, as a deeper nest would
    /// be. After the undo, which kills all three copies, each record's
    /// site reads as a live vertex, and the hand strut's site, two
    /// fusions deep, as the notch strut's own site. Red if a site is
    /// read through one fusion only, or not at all, or if either
    /// operand's kills go unlogged. A record naming a site its strut
    /// does not start at is a desync, before any kill: the fusions
    /// would log a site that is not the kill's survivor.
    #[test]
    fn every_strut_site_reads_through_the_undo_to_a_standing_vertex() {
        let tol = Tol::witness();
        for nest in [Operand::A, Operand::B] {
            let (mut red, site, [c1, c2, c3]) = hung_three_deep(nest, |_, c2| c2, tol);
            let fused = undo_struts(&mut red, tol).unwrap();
            let body = body_of(&mut red, nest);
            assert!(
                [c1, c2, c3].iter().all(|&c| body.get_vertex(c).is_none()),
                "{nest:?}: the undo kills the three copies"
            );
            for r in &red.null_edges {
                let body = match r.operand {
                    Operand::A => &red.a,
                    Operand::B => &red.b,
                };
                let end = fused.end(r.operand, r.at_vertex).unwrap();
                assert!(
                    body.get_vertex(end).is_some(),
                    "{nest:?}: {:?}'s site {:?} reads as {end:?}, which the undo killed",
                    r.edge,
                    r.at_vertex
                );
            }
            assert_eq!(
                (fused.end(nest, c1).unwrap(), fused.end(nest, c2).unwrap()),
                (site, site),
                "{nest:?}: one and two fusions deep, a nested site reads as the notch strut's site"
            );

            let (mut red, ..) = hung_three_deep(nest, |c1, _| c1, tol);
            let before = body_of(&mut red, nest).vertices.len();
            let got = undo_struts(&mut red, tol).map(|_| ());
            assert!(
                matches!(
                    got,
                    Err(BooleanError::JoinDesync {
                        what: "REST lane: strut does not join its site vertex"
                    })
                ),
                "{nest:?}: a misplaced site: {got:?}"
            );
            assert_eq!(
                body_of(&mut red, nest).vertices.len(),
                before,
                "{nest:?}: a misplaced site kills nothing"
            );
        }
    }

    /// **The segments' ends are the vertices the undo leaves standing.**
    /// [`nested`]'s reduction, the prism as A and as B: the wedge
    /// strut's site is the notch strut's copy, and a segment ends there.
    /// Every end [`read_segments`] answers is live, and the nested one
    /// reads as the notch strut's site. Red if the ends are taken as
    /// the raw sites. A fusion log that is not well-ordered refuses
    /// typed in every build, where an unchecked fold would land on a
    /// dead key.
    #[test]
    fn the_segment_ends_read_through_the_undo_to_standing_vertices() {
        let tol = Tol::witness();
        for nest in [Operand::A, Operand::B] {
            let (mut red, outer, inner) = nested(nest, tol);
            let copy = outer.attr.copy_at(outer.at_vertex).unwrap();
            assert!(
                red.null_pairs.iter().any(|p| match nest {
                    Operand::A => p.a_edge == inner.edge,
                    Operand::B => p.b_edge == inner.edge,
                }),
                "{nest:?}: the wedge strut is a pair's, so a segment ends at its site"
            );
            let segments = read_segments(&mut red, Band::linear(tol).unwrap(), tol)
                .unwrap()
                .unwrap();
            let ends: Vec<(Operand, VertexKey)> = segments
                .iter()
                .flat_map(|s| {
                    [
                        (Operand::A, s.a_u),
                        (Operand::A, s.a_v),
                        (Operand::B, s.b_u),
                        (Operand::B, s.b_v),
                    ]
                })
                .collect();
            for &(operand, w) in &ends {
                assert!(
                    body_of(&mut red, operand).get_vertex(w).is_some(),
                    "{nest:?}: segment end {w:?} of {operand:?} does not stand after the undo"
                );
            }
            assert!(
                ends.contains(&(nest, outer.at_vertex)) && !ends.contains(&(nest, copy)),
                "{nest:?}: the nested end reads as the notch strut's site {:?}: {ends:?}",
                outer.at_vertex
            );
        }

        let unordered = Fused {
            a: Vec::new(),
            b: vec![(VertexKey::default(), VertexKey::default())],
        };
        let got = unordered.end(Operand::B, VertexKey::default());
        assert!(
            matches!(got, Err(BooleanError::JoinDesync { .. })),
            "a fusion into itself: {got:?}"
        );
    }

    /// **The interior-edge walk panics on a torn half's face**: an edge
    /// of the arena walk whose half's loop was dropped panics naming
    /// that loop, where a read of the miss as off the patch would skip
    /// the edge and answer.
    #[test]
    fn the_interior_edge_walk_panics_on_a_half_whose_loop_does_not_resolve() {
        use crate::live::OPERATORS_KEEP_LINKS;
        use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let patch: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        let seam = SeamSet {
            set: SecondaryMap::new(),
            per_segment: Vec::new(),
        };
        assert_eq!(
            interior_edges(&body, &patch, &seam).unwrap().len(),
            12,
            "a patch of every face holds every edge"
        );
        let lost = body.get_face(patch[0]).unwrap().outer;
        body.loops.remove(lost);
        let named = format!("'s parent_loop names {}", EntityId::Loop(lost));
        assert_torn_op_panics(
            "interior_edges",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| interior_edges(b, &patch, &seam).map(|e| e.len()),
        );
    }
}

/// **A declared `Rest` bridges only a displacement that stays in band
/// at every point of the faces it is consumed on.** Each row poses a
/// pair a datum-by-datum reading bridged — each datum in band at its
/// own lever — while some consumed point stands at or past the band;
/// the door reads the pair as one displacement and refuses it.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod lever_rows {
    use super::*;
    use crate::boolean::boxes::tests::torus_wall;
    use crate::contact::ContactRefusal;
    use crate::test_support::{CylFrame, cyl_wall_sheet};
    use geom_core::{Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn door(a: &Body<f64>, fa: FaceKey, b: &Body<f64>, fb: FaceKey) -> Result<(), ContactRefusal> {
        crate::boolean::contact_pair_verdict(
            a,
            fa,
            b,
            fb,
            crate::contact::ContactClass::Rest,
            None,
            band(),
        )
        .map(|_| ())
    }

    /// A torus of `R + r = 2.5 m` against its twin tilted by `θ` about
    /// `y` through the shared centre. At `θ = 0.6·Kε`, in band at one
    /// metre, the tilt moves the tube's core circle by `θ·R = 1.2·Kε`
    /// while no corner of the patch stands past the band: the door
    /// refuses unsettled rather than bridging. At `2·Kε` a corner stands
    /// past it, and the door contradicts.
    #[test]
    fn a_torus_tilt_is_read_at_the_ring() {
        for (k, unsettled) in [(0.6, true), (2.0, false)] {
            let theta = k * band().escalate();
            let (u, v) = ((0.3, 1.9), (-0.7, 0.8));
            let (a, fa) = torus_wall(
                Point3::origin(),
                Vec3::unit_z(),
                Vec3::unit_x(),
                2.0,
                0.5,
                u,
                v,
            );
            let (mut b, fb) = torus_wall(
                Point3::origin(),
                Vec3::new(theta.sin(), 0.0, theta.cos()),
                Vec3::new(theta.cos(), 0.0, -theta.sin()),
                2.0,
                0.5,
                u,
                v,
            );
            b.set_face_sense(fb, false).unwrap();
            let read = door(&a, fa, &b, fb);
            if unsettled {
                assert!(
                    matches!(read, Err(ContactRefusal::Escalated { .. })),
                    "{k}·Kε: the swing at the ring is unsettled: {read:?}"
                );
            } else {
                assert!(
                    matches!(read, Err(ContactRefusal::Contradicted { .. })),
                    "{k}·Kε: a corner past the band contradicts: {read:?}"
                );
            }
        }
    }

    /// A 10 m cylinder wall against its twin tilted by `0.5·K·ε` about
    /// `y` through the shared axis point at its foot: `0.5·Kε` at one
    /// metre, while the far rim stands `5·Kε` off.
    #[test]
    fn a_cylinder_tilt_is_read_at_the_far_rim() {
        let tol = Tol::witness();
        let theta = 0.5 * band().escalate();
        let (u, v) = ((0.2, 1.6), (0.0, 10.0));
        let mut a = Body::<f64>::new();
        let fa = cyl_wall_sheet(&mut a, CylFrame::canonical(1.0), None, u, v, tol);
        let mut b = Body::<f64>::new();
        let fb = cyl_wall_sheet(&mut b, CylFrame::tilted(1.0, theta), None, u, v, tol);
        let sense = a.get_face(fa).unwrap().sense;
        b.set_face_sense(fb, !sense).unwrap();
        let read = door(&a, fa, &b, fb);
        assert!(
            matches!(read, Err(ContactRefusal::Contradicted { .. })),
            "the tilt at the far rim contradicts the declaration: {read:?}"
        );
    }

    /// **The offset and the tilt add.** A 1 m band of cylinder wall at
    /// `9 ≤ v ≤ 10` against its twin tilted by `0.105·K·ε` about `y`
    /// through the axis' foot at the origin: the axis offset at the
    /// patch's middle reads `≈ 1.0·Kε` (just in band), the tilt over the
    /// patch's extent another fraction of it, each in band on its own,
    /// while the rim stands `1.05·Kε` off. Read datum by datum it
    /// bridged; read as one displacement it does not.
    #[test]
    fn an_offset_and_a_tilt_in_band_each_do_not_bridge_their_sum() {
        let tol = Tol::witness();
        let theta = 0.105 * band().escalate();
        let (u, v) = ((-0.3, 0.3), (9.0, 10.0));
        let mut a = Body::<f64>::new();
        let fa = cyl_wall_sheet(&mut a, CylFrame::canonical(1.0), None, u, v, tol);
        let mut b = Body::<f64>::new();
        let fb = cyl_wall_sheet(&mut b, CylFrame::tilted(1.0, theta), None, u, v, tol);
        let sense = a.get_face(fa).unwrap().sense;
        b.set_face_sense(fb, !sense).unwrap();
        let read = door(&a, fa, &b, fb);
        assert!(
            matches!(read, Err(ContactRefusal::Contradicted { .. })),
            "the rim stands past the band, so the declaration does not bridge: {read:?}"
        );
    }
}
