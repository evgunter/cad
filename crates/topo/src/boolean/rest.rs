//! **The declared-REST union zip** (M5 S1, #102's crosslap frontier —
//! the M3 envelope's boundary-on-boundary class (iii)).
//!
//! A *pure REST contact* is a mate whose interiors are DISJOINT and
//! whose shared geometry lies entirely on both operands' boundaries:
//! the contact region R is a union of coincident opposite-oriented
//! face patches — on ANY carrier the ladder certifies (plane, sphere,
//! cylinder; the C4 `Rest` inventory) — and its boundary ∂R — the
//! seam — runs along operand edges or across single faces, never
//! through material. The chord joining ([`super::join`]) cannot
//! complete such seams: at a REST site a germ direction lies in FOUR
//! coincident tangent planes (two per solid, cosurface via the
//! declared rung), the two end records of one segment can resolve
//! that ambiguity onto different face pairs, and the germ-identity
//! match (face pairs agree) then never fires — the typed
//! `Join(UnpairedLooseEnds)` / `JoinDesync` refusals (and, for
//! curved-adjacent seams the join has no section arm for, its typed
//! per-kind refusal).
//!
//! This lane replaces the chord/null-face machinery for exactly that
//! frontier, **union only**, reached ONLY when (a) the op carries
//! declared coincident faces (the ladder is law — the undeclared mate
//! keeps refusing at the coincidence door, inside the reduction) and
//! (b) the normal join has already refused typed. The reduction —
//! gates, coincidence doors, sweep splitting, classification — runs
//! unchanged first; this lane consumes its RECORDS:
//!
//! 1. **Segments**: the null-pair germ records are matched into seam
//!    segments by the SAME mutual-facing/nearest tests as the join
//!    (`bool_join_chord` / `bool_join_facing` / `bool_join_nearest` —
//!    reused predicate funnels, no new numeric predicate), with the
//!    ambiguous face-pair identity dropped. Incomplete matching ⇒
//!    not this frontier (the original join refusal stands).
//! 2. **Lane door**: every declared face pair is verified through
//!    [`super::oriented_plane_eq`]'s declared rung — a false
//!    declaration refuses [`BooleanError::ContactContradicted`]
//!    here, never a silent no-op. Opposite-oriented verified pairs
//!    name the REST-contact surfaces.
//! 3. **Undo the scaffolding**: the classification's null-edge struts
//!    are removed (`kev`, reverse mint order) from clones of the
//!    annotated operands — the sweep's edge splits and the pierce-ring
//!    vertices remain (both are load-bearing: they make the seam
//!    vertex sets congruent across the mate).
//! 4. **Seam realization** (splitting machinery reused): per segment
//!    and per solid, either the segment already IS an operand edge
//!    (structural fan walk — reused as the seam, minted nowhere), or
//!    it is minted ONCE as a real chord through the standard
//!    `mef`/`mekr` machinery in the unique face bounded by both
//!    endpoints. No new region algebra: a segment that does not
//!    resolve structurally refuses typed
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

use geom_core::{Band, Bounds, Decide, Margin, Sign};
use slotmap::SecondaryMap;

use super::carrier_eq::{CarrierDesc, CarrierEqError, CarrierRelation};
use super::combine::graft_solid;
use super::ops::{
    Descendants, KeyView, declared_surface_pairs, describe_minted_edges, gate, graft_rows,
    merge_rows, remap_carried, remap_contacts, volume_backstop,
};
use super::plane_eq::{PlaneEqError, PlaneIdentity, PlaneRelation};
use super::reduce::{face_plane, face_plane_source};
use super::zip::{ZipReport, zip_seam};
use super::{
    BoolNullEdgeRecord, BooleanBody, BooleanDeclarations, BooleanError, BooleanNaming, BooleanOp,
    BooleanReduction, BooleanResult, BooleanResultKind, FacePairDeclaration, Operand, OperandKeys,
};
use crate::body::Body;
use crate::contact::ContactClass;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::{FaceSurface, MefSite};
use crate::euler_ring::MekrSite;
use crate::geometry::SurfaceKey;
use crate::splitting::finish::single_solid;
use crate::validate::decide;
use geom_core::Tol;

/// A kernel-bug-class desync inside the lane (after the frontier is
/// positively identified) — same posture as the join's lockstep
/// refusals.
fn desync(what: &'static str) -> BooleanError {
    BooleanError::JoinDesync { what }
}

/// A named sub-frontier the lane declines (honest typed refusal,
/// never a laundered catch-all).
fn unsupported(what: &'static str) -> BooleanError {
    BooleanError::RestZipUnsupported { what }
}

/// One seam segment: the two end sites, as vertex keys per operand
/// (the pre-insertion site vertices — they survive the scaffolding
/// undo).
#[derive(Clone, Copy, Debug)]
struct Segment {
    a_u: VertexKey,
    a_v: VertexKey,
    b_u: VertexKey,
    b_v: VertexKey,
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
pub(super) fn try_rest_union<T: Decide + Bounds + geom_brep::PcurveFittedLane>(
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

    // ---- 1. Segments from the germ records (A-side geometry — the
    // site points are bitwise-shared between the solids). ----
    let Some(segments) = enumerate_segments(&red, band)? else {
        return Ok(None);
    };

    // ---- 2. Lane door: verify every declared pair; collect the
    // REST-contact (opposite-oriented) surface sets. ----
    let (a_rest, b_rest) = verify_declared_pairs(a_pristine, b_pristine, decls, band)?;
    if a_rest.is_empty() || b_rest.is_empty() {
        return Ok(None); // no opposite-oriented contact declared
    }

    // Vertex correspondence across the mate, operand keys — record
    // data end to end (F9): the segment end sites, EXTENDED by the
    // reduction's own v-v contact records. A coincident vertex pair
    // INTERIOR to the contact region (e.g. a peg-root rim vertex on
    // the mating plane) has a v-v record but no crossing at its site,
    // so no null pair and no segment names it — and the glue still
    // fuses it when the interior curve network zips. Never geometric
    // point matching.
    let mut vcorr: SecondaryMap<VertexKey, VertexKey> = SecondaryMap::new();
    let mut correspond = |a: VertexKey, b: VertexKey| -> Option<()> {
        match vcorr.get(a) {
            Some(&prev) if prev != b => None, // mis-paired: not ours
            _ => {
                vcorr.insert(a, b);
                Some(())
            }
        }
    };
    for s in &segments {
        for (a, b) in [(s.a_u, s.b_u), (s.a_v, s.b_v)] {
            if correspond(a, b).is_none() {
                return Ok(None);
            }
        }
    }
    for c in &red.contacts.vv {
        if correspond(c.a, c.b).is_none() {
            return Ok(None);
        }
    }

    // ---- 3. Undo the null-edge scaffolding (kev, reverse order). ----
    undo_struts(&mut red)?;

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
        &segments.iter().map(|s| (s.a_u, s.a_v)).collect::<Vec<_>>(),
        &a_rings,
        &mut a_fragments,
        tol,
    )?;
    let Some(a_seam) = a_seam else {
        return Ok(None);
    };
    let b_seam = realize_seam(
        &mut red.b,
        &segments.iter().map(|s| (s.b_u, s.b_v)).collect::<Vec<_>>(),
        &b_rings,
        &mut b_fragments,
        tol,
    )?;
    let Some(b_seam) = b_seam else {
        return Ok(None);
    };

    // ---- 5. Patch discovery + cross-mate pairing. ----
    let Some(a_patch) = patch_faces(&red.a, &a_seam, &a_rest)? else {
        return Ok(None);
    };
    let Some(b_patch) = patch_faces(&red.b, &b_seam, &b_rest)? else {
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
    let mut body = red.a;
    let solid = single_solid(&body).map_err(|_| desync("REST lane: operand A not one solid"))?;
    let graft = graft_solid(&mut body, solid, &red.b, tol)?;

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
    let mut interior: SecondaryMap<EdgeKey, ()> = SecondaryMap::new();
    let mut desc = Descendants::default();
    for &fa in &glue_order {
        let fb = fb_of(fa)?;
        let rep = glue_pair(&mut body, fa, fb, &vmap, tol)?;
        desc.absorb_zip(&rep);
        vertex_merges.extend(rep.vertex_merges.iter().copied());
        for &e in &rep.interior_edges {
            interior.insert(e, ());
        }
    }

    // The surviving seam: the A-side per-segment edges (the A arena IS
    // the result arena). A segment INTERIOR to the contact region —
    // an already-fused run a later glue consumed, e.g. the meridian
    // seams of a closed cosurface band — legitimately dies with R's
    // interior and is dropped here; a segment that died any other way
    // is a lane desync.
    let mut seam_edges: Vec<EdgeKey> = Vec::new();
    for &e in &a_seam.per_segment {
        if body.get_edge(e).is_none() {
            if interior.contains_key(e) {
                continue;
            }
            return Err(desync("REST lane: a seam segment edge did not survive"));
        }
        seam_edges.push(e);
    }

    // ---- Output stages (shared with every seamed boolean). ----
    let contacts = red.contacts.clone();
    let reduction_contacts = red.contacts;
    let declared_pairs = declared_surface_pairs(&body, a_pristine, b_pristine, decls, &graft);
    let merged = body
        .merge_coplanar_faces_declared(&declared_pairs, tol)
        .map_err(BooleanError::Merge)?;
    desc.absorb_merge(&merged);
    describe_minted_edges(&mut body, &seam_edges, &merged, band, tol)?;
    let mut contacts = remap_contacts(
        &body,
        &contacts,
        KeyView::Direct,
        KeyView::Graft(&graft),
        &desc,
    );
    remap_carried(
        &mut contacts,
        &body,
        decls,
        &KeyView::Direct,
        &KeyView::Graft(&graft),
        &desc,
    );
    gate(&body)?;
    volume_backstop(BooleanOp::Union, a_pristine, b_pristine, &body, band, tol)?;
    let (graft_vertices, graft_edges, graft_faces) = graft_rows(&graft);
    let naming = BooleanNaming {
        a_keys: OperandKeys::Direct,
        b_keys: OperandKeys::Grafted,
        graft_vertices,
        graft_edges,
        graft_faces,
        seam_edges,
        vertex_merges,
        merge_groups: merge_rows(&merged),
        merge_skipped: merged.skipped.clone(),
        face_fragments_a: a_fragments,
        face_fragments_b: b_fragments,
        reduction_contacts,
    };
    Ok(Some(BooleanResult::Body(BooleanBody {
        body,
        kind: BooleanResultKind::Seamed,
        contacts,
        naming,
    })))
}

// ---------------------------------------------------------------
// 1. Segment enumeration.
// ---------------------------------------------------------------

/// Matches the germ records into seam segments — [`super::join`]'s
/// mutual-facing/nearest tests with the (REST-ambiguous) face-pair
/// identity dropped. `None`: matching did not complete — not this
/// lane's frontier.
fn enumerate_segments<T: Decide>(
    red: &BooleanReduction<T>,
    band: Band,
) -> Result<Option<Vec<Segment>>, BooleanError> {
    let mut a_by_edge: SecondaryMap<EdgeKey, &BoolNullEdgeRecord<T>> = SecondaryMap::new();
    let mut b_by_edge: SecondaryMap<EdgeKey, &BoolNullEdgeRecord<T>> = SecondaryMap::new();
    for r in &red.null_edges {
        match r.operand {
            Operand::A => a_by_edge.insert(r.edge, r),
            Operand::B => b_by_edge.insert(r.edge, r),
        };
    }
    // One germ entry per (pair, slot): site point + outgoing direction
    // + the site vertex keys of both operands.
    struct Germ<T2: geom_core::Real> {
        pair: usize,
        point: geom_core::Point3<T2>,
        dir: geom_core::Vec3<T2>,
        used: bool,
    }
    let mut germs: Vec<Germ<T>> = Vec::new();
    let mut sites: Vec<(VertexKey, VertexKey)> = Vec::new(); // per pair
    for (i, p) in red.null_pairs.iter().enumerate() {
        let a_rec = a_by_edge
            .get(p.a_edge)
            .ok_or_else(|| desync("REST lane: pair A edge without a record"))?;
        let b_rec = b_by_edge
            .get(p.b_edge)
            .ok_or_else(|| desync("REST lane: pair B edge without a record"))?;
        sites.push((a_rec.at_vertex, b_rec.at_vertex));
        for g in &a_rec.germs {
            let v = red
                .a
                .get_half_edge(g.he)
                .ok_or_else(|| desync("REST lane: germ half no longer resolves"))?
                .start;
            let point = *red
                .a
                .get_vertex(v)
                .and_then(|vd| red.a.get_point(vd.point))
                .ok_or_else(|| desync("REST lane: germ vertex has no point"))?;
            germs.push(Germ {
                pair: i,
                point,
                dir: g.dir,
                used: false,
            });
        }
    }
    let escalate = |diag| BooleanError::Escalated { diag };
    let mut segments = Vec::new();
    loop {
        // Globally nearest mutually-facing unused pair (the join's
        // scan order and tie discipline).
        let mut best: Option<(T, usize, usize)> = None;
        for i in 0..germs.len() {
            if germs[i].used {
                continue;
            }
            for j in 0..germs.len() {
                if i == j || germs[j].used || germs[i].pair == germs[j].pair {
                    continue;
                }
                let chord = germs[j].point - germs[i].point;
                let dist = chord.norm();
                match decide("bool_join_chord", Margin::of(dist), band).map_err(escalate)? {
                    Sign::Positive => {}
                    _ => continue,
                }
                // Facing margins in METRES: unit germ dir · chord =
                // cos × separation (rim-dimensional audit: the former
                // `/ dist` compared a bare cosine against the length
                // band — class (c)).
                let f1 = germs[i].dir.dot(chord);
                let f2 = germs[j].dir.dot(-chord);
                if decide("bool_join_facing", Margin::of(f1), band).map_err(escalate)?
                    != Sign::Positive
                    || decide("bool_join_facing", Margin::of(f2), band).map_err(escalate)?
                        != Sign::Positive
                {
                    continue;
                }
                best = match best {
                    None => Some((dist, i, j)),
                    Some((bd, bi, bj)) => {
                        match decide("bool_join_nearest", Margin::of(dist - bd), band)
                            .map_err(escalate)?
                        {
                            Sign::Negative => Some((dist, i, j)),
                            _ => Some((bd, bi, bj)),
                        }
                    }
                };
            }
        }
        let Some((_, i, j)) = best else {
            break;
        };
        germs[i].used = true;
        germs[j].used = true;
        let (au, bu) = sites[germs[i].pair];
        let (av, bv) = sites[germs[j].pair];
        segments.push(Segment {
            a_u: au,
            a_v: av,
            b_u: bu,
            b_v: bv,
        });
    }
    if germs.iter().any(|g| !g.used) {
        return Ok(None); // leftover germs: not a pure REST seam
    }
    Ok(Some(segments))
}

// ---------------------------------------------------------------
// 2. The lane door.
// ---------------------------------------------------------------

/// The per-operand verified REST-contact (opposite-oriented declared)
/// surface sets.
type RestSurfaces = (SecondaryMap<SurfaceKey, ()>, SecondaryMap<SurfaceKey, ()>);

/// **The one flush-pair door**: the C4 verify ladder for a single
/// cross-body face pair — descriptions through [`face_plane`]
/// (outward, sense-folded), identity through [`face_plane_source`]
/// (oriented sources, S10: the descriptions compared are the two
/// faces' OUTWARD normals, so rung 1's `orient` tags carry the face
/// senses too — REST contact is precisely the `SameOpposite`
/// verdict), and the verdict through [`super::oriented_plane_eq`] at
/// the verification arm, which lives HERE and nowhere else: **1 m** —
/// the declared rung contradicts only on DEFINITE margins, so the arm
/// only meters the angular sliver band (exact fixtures decide
/// definitely either way).
///
/// **Who calls this, exactly one caller, and where the shared arm
/// really is.** This door has ONE consumer today: the flush
/// detector's candidate-generation mode ([`crate::flush`],
/// `declared: false`). Verify-at-use stopped calling it at M9-1, when
/// [`verify_declared_pairs`] and the op's front door moved to the
/// kind-generalized [`carrier_pair_relation`]. The anti-twin property
/// (SELECT-DESIGN §3b) survives that move rather than resting on it,
/// because the two doors CONVERGE one link down: `carrier_pair_relation`
/// builds the same sense-folded plane description through
/// [`face_carrier`]'s Plane arm and the same identity through
/// [`face_plane_source`], and its `(Plane, Plane)` case delegates to
/// [`oriented_plane_eq_verdict`](super::plane_eq::oriented_plane_eq_verdict)
/// — the very function this door's [`super::oriented_plane_eq`] wraps.
/// One verdict function, one set of `decide` sites, one verification
/// arm, reached by two spellings of the same three inputs. The #304
/// review's planted-drift probe showed a hand-mirrored arm passes
/// every axis-aligned suite, which is why the arm is shared rather
/// than mirrored — and why the chain above is stated rather than
/// summarized as "the same door".
///
/// `None`: not a planar pair — there is no description to compare
/// (the detector's honest "not a v1 candidate"; the REST lane treats
/// it as an invariant violation at its own site).
pub fn flush_pair_relation<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    declared: bool,
    band: Band,
) -> Option<Result<PlaneRelation, PlaneEqError>> {
    let (pa, pb) = (face_plane(a, fa)?, face_plane(b, fb)?);
    let (ga, gb) = (face_plane_source(a, fa), face_plane_source(b, fb));
    let id = PlaneIdentity {
        s1: ga.as_ref(),
        s2: gb.as_ref(),
        declared,
    };
    Some(super::oriented_plane_eq(&pa, &pb, id, T::one(), band))
}

/// The face's **oriented carrier description** — the curved
/// generalization of [`face_plane`], folding the face's sense into
/// the material side exactly as that door does (S10).
///
/// `None` for a surface kind outside the `Rest` ladder's inventory
/// (cone, NURBS, `Approx`): the C4 table names the kinds
/// [`mod@super::carrier_eq`] carries a rung for, and a kind it cannot
/// compare refuses typed at the caller rather than being approximated
/// by one it can.
pub fn face_carrier<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<CarrierDesc<T>> {
    let f = body.get_face(face)?;
    let sign = f.sense_sign::<T>();
    // `sense` is the material-side bit: true means the face's outward
    // normal IS the chart normal, which for a sphere/cylinder chart
    // points away from the centre/axis. Read as a BIT, never as a
    // comparison on `T` — the scalar backends order intervals, not
    // signs (S10's exact-bit discipline).
    let outward = f.sense;
    match body.get_surface(f.surface) {
        Some(geom::Surface::Plane { origin, normal, .. }) => Some(CarrierDesc::Plane {
            origin: *origin,
            normal: *normal * sign,
        }),
        Some(geom::Surface::Sphere { center, radius, .. }) => Some(CarrierDesc::Sphere {
            center: *center,
            radius: *radius,
            outward,
        }),
        Some(geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        }) => Some(CarrierDesc::Cylinder {
            origin: *origin,
            axis: *axis,
            radius: *radius,
            outward,
        }),
        Some(geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        }) => Some(CarrierDesc::Torus {
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
/// Same descriptions-plus-identity construction, same verification
/// arm (**1 m**, living here and nowhere else), same shared-by-
/// construction contract between the verify-at-use site and the
/// detector's candidate-generation mode — only the carrier kind
/// widens. The planar case reaches exactly the same numbers it
/// reached before ([`mod@super::carrier_eq`]'s plane arm delegates), so
/// this door is a superset of the old one rather than a replacement
/// for it.
///
/// `None`: a face whose surface kind is outside the ladder's
/// inventory — there is no description to compare.
pub fn carrier_pair_relation<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    declared: bool,
    band: Band,
) -> Option<Result<CarrierRelation, CarrierEqError>> {
    Some(carrier_pair_verdict(a, fa, b, fb, declared, band)?.map(|(rel, _)| rel))
}

/// [`carrier_pair_relation`] plus the AQ6 trilean — the door the
/// CONTACT verification uses, since only a caller that can see the
/// bridged residue can enforce C4's "trusted exactly there" invariant.
/// One traversal, two projections.
pub fn carrier_pair_verdict<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    declared: bool,
    band: Band,
) -> Option<Result<(CarrierRelation, crate::contact::ContactVerdict), CarrierEqError>> {
    let (ca, cb) = (face_carrier(a, fa)?, face_carrier(b, fb)?);
    let (ga, gb) = (face_plane_source(a, fa), face_plane_source(b, fb));
    let id = PlaneIdentity {
        s1: ga.as_ref(),
        s2: gb.as_ref(),
        declared,
    };
    Some(super::carrier_eq::carrier_eq_verdict(
        &ca,
        &cb,
        id,
        T::one(),
        band,
    ))
}

/// **The certified-lane tangent LOCUS** (M9-2, the M9-1 PR-2 DEV-1
/// ruling): the closed-form contact line of a tangent carrier pair,
/// for exactly the configurations whose locus IS closed-form — a
/// plane and a cylinder tangent along a ruling, and two PARALLEL
/// cylinders tangent along the line between closest generators.
///
/// This is GEOMETRY, and it lives beside [`carrier_pair_relation`]
/// because its consumers are that door's: the LIB flush detector's
/// `Tangent` arm (its named follow-up — a tangency finding without a
/// locus is one the verifier cannot check, so the detector waits on
/// THIS helper) and any at-rest verification that must mint the
/// witness a `Tangent` declaration is verified along.
#[derive(Clone, Copy, Debug)]
pub enum TangentLocus<T: geom_core::Real> {
    /// The tangent line: `origin + t·dir`, `dir` unit (both certified
    /// carriers are ruled along it).
    Line {
        /// A point on the locus.
        origin: geom_core::Point3<T>,
        /// The locus direction (the shared ruling / axis direction).
        dir: geom_core::Vec3<T>,
    },
}

/// Typed refusal of [`tangent_locus`] (closed enum, D3 style).
#[derive(Debug)]
pub enum TangentLocusError {
    /// A margin landed in the sliver band.
    Escalated(geom_core::Indeterminate),
    /// The pair is definitely NOT tangent: `apart` distinguishes the
    /// definite-clearance side from the definite-crossing side.
    NotTangent {
        /// `true`: definite clearance; `false`: definite crossing.
        apart: bool,
    },
    /// The configuration is outside the closed-form lane (kinds other
    /// than plane×cylinder / parallel cylinders, or a non-parallel
    /// axis relation): the demanded set IS the certifiable set — no
    /// sampled locus, ever.
    Unsupported {
        /// Why the configuration has no closed-form locus.
        what: &'static str,
    },
}

/// The closed-form tangent locus of two carriers (see
/// [`TangentLocus`]). Kind dispatch is structural; every numeric
/// decision is a named three-outcome row:
///
/// - `tangent_locus_axis_parallel` — the axis/plane (or axis/axis)
///   angular deviation `|d × n̂|` (a sine of unit vectors) levered by
///   the **1 m verification arm** ([`carrier_pair_relation`]'s own,
///   living here and nowhere else): tangency along an unbounded
///   ruling is a carrier-level claim, metered at the same arm the
///   carrier ladder meters its parallelism rungs.
/// - `tangent_locus_gap` — the metre gap at the tangency: for
///   plane×cylinder the axis-to-plane distance minus the radius; for
///   parallel cylinders the axis-to-axis distance minus `r1 + r2`
///   (external) falling back to `|r1 − r2|` (internal). Zero ⇒
///   tangent (the locus mints); Positive ⇒ definitely apart;
///   Negative ⇒ definitely crossing.
///
/// **CONTRACT — the separation invariant** (consumed by the reduce
/// sweep's declared-cover rung): every configuration this lane mints
/// a locus for has each carrier wholly in ONE closed residual
/// half-space of the other — a plane tangent to a cylinder has the
/// whole cylinder on one side; each of two externally (or
/// internally) tangent parallel cylinders is one-signed against the
/// other — so an on-carrier edge under a verified declaration never
/// crosses the partner surface. A new arm may NOT land here without
/// restating its own residual-sign story: the coaxial
/// cylinder×sphere circle arm's residuals are one-signed in OPPOSITE
/// orientations per direction, which is exactly why it is blocked on
/// that story (issue #974).
///
/// # Errors
///
/// [`TangentLocusError`] — escalation, definite non-tangency, or a
/// configuration outside the closed-form lane.
pub fn tangent_locus<T: Decide>(
    a: &geom::Surface<T>,
    b: &geom::Surface<T>,
    band: Band,
) -> Result<TangentLocus<T>, TangentLocusError> {
    use geom::Surface;
    let arm = T::one();
    let escalate = TangentLocusError::Escalated;
    match (a, b) {
        (
            Surface::Plane { origin, normal, .. },
            Surface::Cylinder {
                origin: co,
                axis,
                radius,
                ..
            },
        )
        | (
            Surface::Cylinder {
                origin: co,
                axis,
                radius,
                ..
            },
            Surface::Plane { origin, normal, .. },
        ) => {
            // Ruling tangency needs the axis IN the plane's direction
            // space: |axis · n̂| is the sine of the axis' elevation.
            match decide(
                "tangent_locus_axis_parallel",
                Margin::levered(axis.dot(*normal).abs(), arm),
                band,
            )
            .map_err(escalate)?
            {
                Sign::Zero => {}
                _ => {
                    return Err(TangentLocusError::Unsupported {
                        what: "plane×cylinder tangency is closed-form only along a ruling — \
                               the axis must lie in the plane's direction space",
                    });
                }
            }
            // Signed axis-to-plane height; its SIGN picks the tangent
            // generator, its magnitude minus r is the tangency gap.
            let h = (*co - *origin).dot(*normal);
            let side = match decide("tangent_locus_side", Margin::of(h), band).map_err(escalate)? {
                Sign::Positive => T::one(),
                Sign::Negative => T::zero() - T::one(),
                Sign::Zero => {
                    // Axis ON the plane: the cylinder definitely
                    // crosses (both sides pierce).
                    return Err(TangentLocusError::NotTangent { apart: false });
                }
            };
            match decide("tangent_locus_gap", Margin::of(h.abs() - *radius), band)
                .map_err(escalate)?
            {
                Sign::Zero => Ok(TangentLocus::Line {
                    origin: *co - *normal * (side * *radius),
                    dir: *axis,
                }),
                Sign::Positive => Err(TangentLocusError::NotTangent { apart: true }),
                Sign::Negative => Err(TangentLocusError::NotTangent { apart: false }),
            }
        }
        (
            Surface::Cylinder {
                origin: o1,
                axis: a1,
                radius: r1,
                ..
            },
            Surface::Cylinder {
                origin: o2,
                axis: a2,
                radius: r2,
                ..
            },
        ) => {
            match decide(
                "tangent_locus_axis_parallel",
                Margin::levered(a1.cross(*a2).norm(), arm),
                band,
            )
            .map_err(escalate)?
            {
                Sign::Zero => {}
                _ => {
                    return Err(TangentLocusError::Unsupported {
                        what: "cylinder×cylinder tangency is closed-form only for PARALLEL \
                               axes (the generator line); skew/crossing axes are outside \
                               the lane",
                    });
                }
            }
            // Perpendicular axis-to-axis offset (the axis LINE datum,
            // the carrier ladder's own construction).
            let delta = *o2 - *o1;
            let w = delta - *a1 * delta.dot(*a1);
            let dist = w.norm();
            // External tangency first (|w| = r1 + r2): the common case
            // and the flush detector's; internal (|w| = |r1 − r2|)
            // second. Fixed probe order (D9).
            match decide("tangent_locus_gap", Margin::of(dist - (*r1 + *r2)), band)
                .map_err(escalate)?
            {
                Sign::Zero => {
                    let w_hat = w.normalize();
                    return Ok(TangentLocus::Line {
                        origin: *o1 + w_hat * *r1,
                        dir: *a1,
                    });
                }
                Sign::Positive => return Err(TangentLocusError::NotTangent { apart: true }),
                Sign::Negative => {}
            }
            match decide(
                "tangent_locus_gap",
                Margin::of((*r1 - *r2).abs() - dist),
                band,
            )
            .map_err(escalate)?
            {
                Sign::Zero => {
                    // Internal tangency: the smaller cylinder rests
                    // inside the larger; the generator sits on the
                    // offset direction at the LARGER radius from the
                    // larger axis. With coaxial axes (dist in the
                    // zero band AND radii in the zero band) the locus
                    // direction is ill-posed — refuse typed.
                    match decide("tangent_locus_side", Margin::of(dist), band).map_err(escalate)? {
                        Sign::Positive => {}
                        _ => {
                            return Err(TangentLocusError::Unsupported {
                                what: "coaxial equal-radius cylinders have no isolated \
                                       tangent generator (conformal contact is Rest, \
                                       not Tangent)",
                            });
                        }
                    }
                    // Which cylinder contains which decides the
                    // generator's side (derived: with ŵ = o1→o2 and
                    // |w| = |r1 − r2|, the touch point is
                    // P = o1 + ŵ·r1 when r1 > r2 — c2 inside c1 —
                    // and P = o1 − ŵ·r1 when r1 < r2, both from
                    // collinearity of P, o1, o2 with |P−o1| = r1,
                    // |P−o2| = r2). The side is DECIDED, never an
                    // evaluation-lane comparison.
                    let w_hat = w.normalize();
                    let sign = match decide("tangent_locus_side", Margin::of(*r1 - *r2), band)
                        .map_err(escalate)?
                    {
                        Sign::Positive => T::one(),
                        Sign::Negative => T::zero() - T::one(),
                        Sign::Zero => {
                            return Err(TangentLocusError::Unsupported {
                                what: "equal-radius internal tangency contradicts the \
                                       definite axis offset — no closed-form generator",
                            });
                        }
                    };
                    Ok(TangentLocus::Line {
                        origin: *o1 + w_hat * (sign * *r1),
                        dir: *a1,
                    })
                }
                // dist < |r1 − r2|: one cylinder NESTED strictly
                // inside the other — the surfaces definitely do NOT
                // meet (their minimum distance is |r1 − r2| − dist,
                // definitely positive here): APART, not crossing
                // (union fix F3 — the pre-fix arm labeled this
                // definite clearance a crossing).
                Sign::Positive => Err(TangentLocusError::NotTangent { apart: true }),
                // |r1 − r2| < dist < r1 + r2 (the external row already
                // refused the ≥ side): the surfaces definitely cross.
                Sign::Negative => Err(TangentLocusError::NotTangent { apart: false }),
            }
        }
        _ => Err(TangentLocusError::Unsupported {
            what: "the closed-form tangent-locus lane holds plane×cylinder and parallel \
                   cylinder pairs only (the DEV-1 certified set)",
        }),
    }
}

/// Verifies every declared face pair through the declared rung and
/// returns the opposite-oriented (REST-contact) surface sets per
/// operand. A definitely-distinct declared pair is the typed
/// [`BooleanError::ContactContradicted`] — a false REST
/// declaration refuses at the lane, never a silent no-op.
fn verify_declared_pairs<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
    band: Band,
) -> Result<RestSurfaces, BooleanError> {
    let mut a_rest: SecondaryMap<SurfaceKey, ()> = SecondaryMap::new();
    let mut b_rest: SecondaryMap<SurfaceKey, ()> = SecondaryMap::new();
    for &FacePairDeclaration {
        a: fa,
        b: fb,
        class,
    } in &decls.coincident_faces
    {
        // Only the CONFORMAL class names REST-contact surfaces; a
        // `Tangent` pair touches along a locus, was verified by its
        // own C4 table at the front door, and licenses no patch.
        if class != ContactClass::Rest {
            continue;
        }
        // The one flush-pair door ([`flush_pair_relation`]): oriented
        // sources, sense-folded descriptions, and the verification
        // arm all live inside it — shared with the LIB-SEL2 detector
        // by construction.
        // The generalized door: planar pairs reach exactly the numbers
        // the plane ladder always reached (its plane arm delegates),
        // and a curved declared pair is verified rather than being
        // silently outside the lane.
        let relation = carrier_pair_relation(a, fa, b, fb, true, band).ok_or(
            BooleanError::ClassificationInvariant {
                what: "REST lane: a declared face lost its carrier",
            },
        )?;
        match relation {
            Ok(PlaneRelation::SameOpposite) => {
                let sa = a
                    .get_face(fa)
                    .ok_or(BooleanError::ClassificationInvariant {
                        what: "REST lane: declared A face vanished",
                    })?
                    .surface;
                let sb = b
                    .get_face(fb)
                    .ok_or(BooleanError::ClassificationInvariant {
                        what: "REST lane: declared B face vanished",
                    })?
                    .surface;
                a_rest.insert(sa, ());
                b_rest.insert(sb, ());
            }
            Ok(PlaneRelation::SameOriented) => {} // merge-stage pair
            Ok(PlaneRelation::Distinct) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "REST lane: declared rung returned Distinct instead of contradicting",
                });
            }
            Err(PlaneEqError::Contradicted(diag)) => {
                // C4's verify-at-use: the refusal names the pair, the
                // CLASS that was claimed and the margin that decided —
                // and steers to the class that would fit when the
                // counter-evidence is a separation (AQ6).
                return Err(BooleanError::ContactContradicted {
                    declaration: crate::contact::DeclaredContact {
                        a: fa,
                        b: fb,
                        class: ContactClass::Rest,
                    },
                    steer: super::contact_verify::fit_steer(&diag),
                    margin: diag,
                });
            }
            Err(PlaneEqError::Escalated(diag)) => {
                return Err(BooleanError::Escalated { diag });
            }
            Err(PlaneEqError::Undeclared { diag, relation }) => {
                // Unreachable with declared=true; refuse loudly anyway.
                return Err(BooleanError::UndeclaredCoincidence {
                    diag,
                    pair: [(Operand::A, fa), (Operand::B, fb)],
                    relation,
                });
            }
        }
    }
    Ok((a_rest, b_rest))
}

// ---------------------------------------------------------------
// 3. Scaffolding undo.
// ---------------------------------------------------------------

/// Removes every classification-minted null-edge strut (`kev`,
/// reverse mint order), fusing the site copies back into the original
/// vertices. Sweep splits and pierce-ring vertices remain.
fn undo_struts<T: Decide>(red: &mut BooleanReduction<T>) -> Result<(), BooleanError> {
    for r in red.null_edges.iter().rev() {
        let body = match r.operand {
            Operand::A => &mut red.a,
            Operand::B => &mut red.b,
        };
        let copy = if r.attr.below_end == r.at_vertex {
            r.attr.above_end
        } else if r.attr.above_end == r.at_vertex {
            r.attr.below_end
        } else {
            return Err(desync("REST lane: strut without its site vertex as an end"));
        };
        let edge = body
            .get_edge(r.edge)
            .ok_or_else(|| desync("REST lane: strut edge no longer resolves"))?
            .clone();
        let he = if body.half_edge_end(edge.he_plus) == Some(copy) {
            edge.he_plus
        } else if body.half_edge_end(edge.he_minus) == Some(copy) {
            edge.he_minus
        } else {
            return Err(desync("REST lane: strut halves do not reach the copy"));
        };
        body.kev(he)
            .map_err(|_| desync("REST lane: strut undo kev refused"))?;
    }
    Ok(())
}

// ---------------------------------------------------------------
// 4. Seam realization.
// ---------------------------------------------------------------

/// One solid's realized seam.
struct SeamSet {
    /// Every seam edge (structural set).
    set: SecondaryMap<EdgeKey, ()>,
    /// The seam edge of each segment, in segment order.
    per_segment: Vec<EdgeKey>,
}

/// Realizes the seam in one solid: per segment, the existing operand
/// edge (fan walk) or a minted chord through the standard splitting
/// machinery. `Ok(None)`: a segment does not resolve structurally —
/// not this lane's frontier (pre-identification phase).
fn realize_seam<T: Decide>(
    body: &mut Body<T>,
    segments: &[(VertexKey, VertexKey)],
    rings: &SecondaryMap<VertexKey, FaceKey>,
    fragments: &mut Vec<(FaceKey, FaceKey)>,
    tol: Tol,
) -> Result<Option<SeamSet>, BooleanError> {
    let mut out = SeamSet {
        set: SecondaryMap::new(),
        per_segment: Vec::with_capacity(segments.len()),
    };
    for &(u, v) in segments {
        let edge = match fan_edge_between(body, u, v)? {
            Some(e) => e,
            None => match mint_chord(body, u, v, rings, fragments, tol)? {
                Some(e) => e,
                None => return Ok(None),
            },
        };
        out.set.insert(edge, ());
        out.per_segment.push(edge);
    }
    Ok(Some(out))
}

/// The existing edge from `u` to `v`, if any (structural fan walk —
/// zero numerics). Two parallel such edges refuse typed.
fn fan_edge_between<T: Decide>(
    body: &Body<T>,
    u: VertexKey,
    v: VertexKey,
) -> Result<Option<EdgeKey>, BooleanError> {
    let Some(anchor) = body.get_vertex(u).and_then(|vd| vd.emanating) else {
        return Ok(None); // isolated ring vertex
    };
    let orbit = body
        .vertex_orbit(anchor)
        .ok_or_else(|| desync("REST lane: site vertex orbit not walkable"))?;
    let mut found: Option<EdgeKey> = None;
    for he in orbit {
        if body.half_edge_end(he) == Some(v) {
            let e = body
                .get_half_edge(he)
                .ok_or_else(|| desync("REST lane: orbit half no longer resolves"))?
                .edge;
            match found {
                None => found = Some(e),
                Some(prev) if prev == e => {}
                Some(_) => {
                    return Err(unsupported(
                        "two parallel operand edges span one seam segment",
                    ));
                }
            }
        }
    }
    Ok(found)
}

/// Mints the seam chord `u → v` through the standard splitting
/// machinery (`mef` same-loop, `mekr` for ring loops / pierce-ring
/// vertices) in the unique face incident to both endpoints.
/// `Ok(None)`: no unique host face — not this lane's frontier.
fn mint_chord<T: Decide>(
    body: &mut Body<T>,
    u: VertexKey,
    v: VertexKey,
    rings: &SecondaryMap<VertexKey, FaceKey>,
    fragments: &mut Vec<(FaceKey, FaceKey)>,
    tol: Tol,
) -> Result<Option<EdgeKey>, BooleanError> {
    let fu = incident_faces(body, u, rings)?;
    let fv = incident_faces(body, v, rings)?;
    let common: Vec<FaceKey> = fu.iter().filter(|f| fv.contains(f)).copied().collect();
    let [face] = common[..] else {
        return Ok(None); // zero or ambiguous host face
    };
    let hu = halves_at(body, face, u)?;
    let hv = halves_at(body, face, v)?;
    let ring_loop_of = |body: &Body<T>, w: VertexKey| -> Option<LoopKey> {
        let f = body.get_face(face)?;
        f.rings.iter().copied().find(|&l| {
            matches!(
                body.get_loop(l).map(|ld| ld.boundary),
                Some(LoopBoundary::Empty { vertex }) if vertex == w
            )
        })
    };
    let loop_of = |body: &Body<T>, he: HalfEdgeKey| -> Result<LoopKey, BooleanError> {
        Ok(body
            .get_half_edge(he)
            .ok_or_else(|| desync("REST lane: chord half no longer resolves"))?
            .parent_loop)
    };
    let created = match (&hu[..], &hv[..]) {
        ([hu], [hv]) => {
            let (lu, lv) = (loop_of(body, *hu)?, loop_of(body, *hv)?);
            if lu == lv {
                let created = body
                    .mef_chord(MefSite::Chords { he1: *hu, he2: *hv }, tol)
                    .map_err(|_| unsupported("seam chord mef refused on its host face"))?;
                fragments.push((created.face, face));
                created.edge
            } else {
                // Outer ↔ ring (or ring ↔ ring): mekr absorbs the ring.
                let outer = body
                    .get_face(face)
                    .ok_or_else(|| desync("REST lane: chord host face vanished"))?
                    .outer;
                let (target, ring) = if lv == outer { (*hv, *hu) } else { (*hu, *hv) };
                body.mekr_chord(MekrSite::Cycles { target, ring }, tol)
                    .map_err(|_| unsupported("seam chord mekr refused on its host face"))?
                    .edge
            }
        }
        ([], [hv]) => {
            let ring = ring_loop_of(body, u)
                .ok_or_else(|| unsupported("seam chord endpoint has no boundary presence"))?;
            body.mekr_chord(MekrSite::EmptyRing { target: *hv, ring }, tol)
                .map_err(|_| unsupported("seam chord mekr (pierce ring) refused"))?
                .edge
        }
        ([hu], []) => {
            let ring = ring_loop_of(body, v)
                .ok_or_else(|| unsupported("seam chord endpoint has no boundary presence"))?;
            body.mekr_chord(MekrSite::EmptyRing { target: *hu, ring }, tol)
                .map_err(|_| unsupported("seam chord mekr (pierce ring) refused"))?
                .edge
        }
        ([], []) => {
            return Err(unsupported("seam chord between two isolated pierce points"));
        }
        _ => {
            return Err(unsupported(
                "seam chord endpoint revisited by its host face boundary",
            ));
        }
    };
    Ok(Some(created))
}

/// The faces incident to `u`, deterministic orbit order (a pierce-ring
/// vertex contributes its host face).
fn incident_faces<T: Decide>(
    body: &Body<T>,
    u: VertexKey,
    rings: &SecondaryMap<VertexKey, FaceKey>,
) -> Result<Vec<FaceKey>, BooleanError> {
    let Some(anchor) = body.get_vertex(u).and_then(|vd| vd.emanating) else {
        return Ok(rings.get(u).copied().into_iter().collect());
    };
    let orbit = body
        .vertex_orbit(anchor)
        .ok_or_else(|| desync("REST lane: site vertex orbit not walkable"))?;
    let mut faces = Vec::new();
    for he in orbit {
        let l = body
            .get_half_edge(he)
            .ok_or_else(|| desync("REST lane: orbit half no longer resolves"))?
            .parent_loop;
        let f = body
            .get_loop(l)
            .ok_or_else(|| desync("REST lane: orbit loop no longer resolves"))?
            .face;
        if !faces.contains(&f) {
            faces.push(f);
        }
    }
    Ok(faces)
}

/// The face-boundary halves of `face` starting at `u` (outer + rings).
fn halves_at<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    u: VertexKey,
) -> Result<Vec<HalfEdgeKey>, BooleanError> {
    let f = body
        .get_face(face)
        .ok_or_else(|| desync("REST lane: chord host face vanished"))?;
    let mut out = Vec::new();
    for l in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body
            .get_loop(l)
            .ok_or_else(|| desync("REST lane: host loop no longer resolves"))?
            .boundary
        else {
            continue;
        };
        for he in body
            .loop_cycle(first)
            .ok_or_else(|| desync("REST lane: host loop not walkable"))?
        {
            if body
                .get_half_edge(he)
                .ok_or_else(|| desync("REST lane: host half no longer resolves"))?
                .start
                == u
            {
                out.push(he);
            }
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
            let fd = body
                .get_face(f)
                .ok_or_else(|| desync("REST lane: region face vanished"))?;
            for l in core::iter::once(fd.outer).chain(fd.rings.iter().copied()) {
                let LoopBoundary::Cycle { first } = body
                    .get_loop(l)
                    .ok_or_else(|| desync("REST lane: region loop no longer resolves"))?
                    .boundary
                else {
                    continue;
                };
                for he in body
                    .loop_cycle(first)
                    .ok_or_else(|| desync("REST lane: region loop not walkable"))?
                {
                    let hd = body
                        .get_half_edge(he)
                        .ok_or_else(|| desync("REST lane: region half no longer resolves"))?;
                    if seam.set.contains_key(hd.edge) {
                        touches_seam = true;
                        continue;
                    }
                    let mate = body
                        .mate(he)
                        .ok_or_else(|| desync("REST lane: region half has no mate"))?;
                    let nl = body
                        .get_half_edge(mate)
                        .ok_or_else(|| desync("REST lane: region mate no longer resolves"))?
                        .parent_loop;
                    let nf = body
                        .get_loop(nl)
                        .ok_or_else(|| desync("REST lane: region mate loop no longer resolves"))?
                        .face;
                    if !assigned.contains_key(nf) {
                        assigned.insert(nf, ());
                        region.push(nf);
                        queue.push(nf);
                    }
                }
            }
        }
        let qualified = region.iter().all(|&f| {
            body.get_face(f)
                .is_some_and(|fd| rest.contains_key(fd.surface))
        });
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
    let LoopBoundary::Cycle { first } = body
        .get_loop(f.outer)
        .ok_or_else(|| desync("REST lane: cycle loop no longer resolves"))?
        .boundary
    else {
        return Err(desync("REST lane: patch outer loop is empty"));
    };
    let mut out = Vec::new();
    for he in body
        .loop_cycle(first)
        .ok_or_else(|| desync("REST lane: cycle not walkable"))?
    {
        out.push(
            body.get_half_edge(he)
                .ok_or_else(|| desync("REST lane: cycle half no longer resolves"))?
                .start,
        );
    }
    Ok(out)
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
                vcorr.get(v).copied().ok_or_else(|| {
                    unsupported("patch boundary vertex without a seam correspondent")
                })
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
            return Err(unsupported(
                "patch face cycles not congruent across the mate",
            ));
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
            let LoopBoundary::Cycle { first } = body
                .get_loop(fd.outer)
                .ok_or_else(|| desync("REST lane: BFS loop no longer resolves"))?
                .boundary
            else {
                continue;
            };
            for he in body
                .loop_cycle(first)
                .ok_or_else(|| desync("REST lane: BFS loop not walkable"))?
            {
                let hd = body
                    .get_half_edge(he)
                    .ok_or_else(|| desync("REST lane: BFS half no longer resolves"))?;
                if seam.set.contains_key(hd.edge) {
                    continue;
                }
                let mate = body
                    .mate(he)
                    .ok_or_else(|| desync("REST lane: BFS half has no mate"))?;
                let nf = body
                    .get_loop(
                        body.get_half_edge(mate)
                            .ok_or_else(|| desync("REST lane: BFS mate no longer resolves"))?
                            .parent_loop,
                    )
                    .ok_or_else(|| desync("REST lane: BFS mate loop no longer resolves"))?
                    .face;
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
        let LoopBoundary::Cycle { first } = body
            .get_loop(fd.outer)
            .ok_or_else(|| desync("REST lane: glue loop no longer resolves"))?
            .boundary
        else {
            return Err(desync("REST lane: glue face outer loop is empty"));
        };
        body.loop_cycle(first)
            .ok_or_else(|| desync("REST lane: glue loop not walkable"))?
            .into_iter()
            .map(|he| {
                Ok(body
                    .get_half_edge(he)
                    .ok_or_else(|| desync("REST lane: glue half no longer resolves"))?
                    .edge)
            })
            .collect()
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
fn glue_pair<T: Decide>(
    body: &mut Body<T>,
    fa: FaceKey,
    fb: FaceKey,
    vmap: &SecondaryMap<VertexKey, VertexKey>,
    tol: Tol,
) -> Result<ZipReport, BooleanError> {
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
        return Err(unsupported(
            "patch pair carries differing interior-boundary counts",
        ));
    }
    let shared = shared_run(body, fa, fb)?;
    let mut report = if shared.is_empty() {
        zip_seam(body, fa, fb, vmap, tol)?
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
                    .ok_or_else(|| unsupported("ring boundary vertex without a seam correspondent"))
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
            return Err(unsupported("ring cycles not congruent across the mate"));
        };
        used.insert(db, ());
        let shared = shared_run(body, da, db)?;
        let rep = if shared.is_empty() {
            zip_seam(body, da, db, vmap, tol)?
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
fn slit_zip<T: Decide>(
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
            return Err(unsupported("slit-zip face carries rings"));
        }
        let LoopBoundary::Cycle { first } = body
            .get_loop(fd.outer)
            .ok_or_else(|| desync("REST lane: slit loop no longer resolves"))?
            .boundary
        else {
            return Err(desync("REST lane: slit face outer loop is empty"));
        };
        body.loop_cycle(first)
            .ok_or_else(|| desync("REST lane: slit loop not walkable"))
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
        return Err(unsupported("patch pair shares its whole boundary"));
    }
    let n = flags.len();
    // Rotate so a run occupies a prefix: find i with flags[i] &&
    // !flags[(i+n-1)%n], then collect every maximal run in cycle
    // order from there (deterministic — D9).
    let Some(start) = (0..n).find(|&i| flags[i] && !flags[(i + n - 1) % n]) else {
        return Err(unsupported("patch pair shares its whole boundary"));
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
        let ed = body
            .get_edge(first_run_edge)
            .ok_or_else(|| desync("REST lane: run edge no longer resolves"))?
            .clone();
        if ed.he_plus == run[0] {
            ed.he_minus
        } else {
            ed.he_plus
        }
    };
    body.kef(fb_half)
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
        let anchor = body
            .get_vertex(dead_end)
            .and_then(|vd| vd.emanating)
            .ok_or_else(|| desync("REST lane: run vertex lost its fan"))?;
        let orbit = body
            .vertex_orbit(anchor)
            .ok_or_else(|| desync("REST lane: run vertex orbit not walkable"))?;
        if orbit.len() != 1 {
            return Err(unsupported(
                "seam-run interior vertex holds edges beyond the run",
            ));
        }
        let mate = body
            .mate(he)
            .ok_or_else(|| desync("REST lane: run half has no mate"))?;
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
            let loop_of = |body: &Body<T>, half| -> Result<LoopKey, BooleanError> {
                Ok(body
                    .get_half_edge(half)
                    .ok_or_else(|| desync("REST lane: band run half no longer resolves"))?
                    .parent_loop)
            };
            let (lh, lm) = (loop_of(body, h)?, loop_of(body, m)?);
            if lh == lm {
                // Dangling (a valence-1 end) → kev that half; doubled
                // deeper in the cycle → kemr (the split-off side
                // becomes a ring, disposed below).
                let dangle_half = {
                    let valence = |body: &Body<T>, half| -> Result<usize, BooleanError> {
                        let end = body
                            .half_edge_end(half)
                            .ok_or_else(|| desync("REST lane: band run half has no end"))?;
                        let anchor = body
                            .get_vertex(end)
                            .and_then(|vd| vd.emanating)
                            .ok_or_else(|| desync("REST lane: band run vertex lost its fan"))?;
                        Ok(body
                            .vertex_orbit(anchor)
                            .ok_or_else(|| desync("REST lane: band run orbit not walkable"))?
                            .len())
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
                let ring_half = if fd.rings.contains(&lh) {
                    h
                } else if fd.rings.contains(&lm) {
                    m
                } else {
                    return Err(unsupported(
                        "band-closure run edge outside the folded face's loops",
                    ));
                };
                let ring = loop_of(body, ring_half)?;
                body.mfkrh(ring, FaceSurface::Inherit)
                    .map_err(|_| desync("REST lane: band run mfkrh refused"))?;
                body.kef(ring_half)
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
fn zip_folded<T: Decide>(
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
        let LoopBoundary::Cycle { first } = body
            .get_loop(fd.outer)
            .ok_or_else(|| desync("REST lane: slit loop vanished mid-zip"))?
            .boundary
        else {
            return Err(desync("REST lane: slit loop emptied mid-zip"));
        };
        let cycle = body
            .loop_cycle(first)
            .ok_or_else(|| desync("REST lane: slit loop not walkable mid-zip"))?;
        if cycle.len() == 2 {
            // The last coincident pair: kef the b copy from inside the
            // face (the face dies with it; the a copy survives as the
            // seam edge).
            let (e0, e1) = (edge_of(body, cycle[0])?, edge_of(body, cycle[1])?);
            let b_half = if b_edges.contains_key(e0) && a_edges.contains_key(e1) {
                cycle[0]
            } else if b_edges.contains_key(e1) && a_edges.contains_key(e0) {
                cycle[1]
            } else {
                return Err(corr("slit-zip final pair is not one copy per side"));
            };
            report
                .seam_edges
                .push(if b_edges.contains_key(e0) { e1 } else { e0 });
            body.kef(b_half)
                .map_err(|_| desync("REST lane: final slit kef refused"))?;
            break;
        }
        // Find the fold: an a-side half followed by a b-side half.
        let mut fold = None;
        for (i, &he) in cycle.iter().enumerate() {
            let e = edge_of(body, he)?;
            let next = cycle[(i + 1) % cycle.len()];
            let en = edge_of(body, next)?;
            if a_edges.contains_key(e) && b_edges.contains_key(en) {
                fold = Some((he, next));
                break;
            }
        }
        let Some((ha, hb)) = fold else {
            return Err(corr("slit-zip fold not found"));
        };
        let sa = body
            .get_half_edge(ha)
            .ok_or_else(|| desync("REST lane: fold half no longer resolves"))?
            .start;
        let eb = body
            .half_edge_end(hb)
            .ok_or_else(|| desync("REST lane: fold half has no end"))?;
        if sa == eb {
            return Err(unsupported("pre-fused vertex pair inside a slit-zip fold"));
        }
        if vmap.get(sa).copied() != Some(eb) {
            return Err(corr("slit-zip vertex pair off the seam correspondence"));
        }
        let p = *body
            .get_vertex(sa)
            .and_then(|vd| body.get_point(vd.point))
            .ok_or_else(|| desync("REST lane: fold vertex has no point"))?;
        let hb_next = body
            .get_half_edge(hb)
            .ok_or_else(|| desync("REST lane: fold half no longer resolves"))?
            .next;
        // Wall off the 3-edge sliver [ha, hb, scaffold], fuse the
        // vertex pair, retire the b copy (its remnant a copy lands in
        // the b-side neighbor's loop — the fuse).
        let made = body.mef(
            MefSite::Chords {
                he1: ha,
                he2: hb_next,
            },
            geom_brep::EdgeCurveSpec::self_loop_circle_at(p),
            FaceSurface::Inherit,
            tol,
        )?;
        body.kev(made.he_plus)
            .map_err(|_| desync("REST lane: slit fuse kev refused"))?;
        report.vertex_merges.push((eb, sa));
        report.seam_edges.push(edge_of(body, ha)?);
        body.kef(hb)
            .map_err(|_| desync("REST lane: slit pair kef refused"))?;
    }
    Ok(())
}
