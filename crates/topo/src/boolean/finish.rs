//! `setopfinish` (ch. 15 §15.8, Program 15.15 re-derived): promote
//! each completed section-polygon pair into IN/OUT section faces in
//! BOTH solids, distribute components, select per **Eq. 15.1**, carve
//! the kept components, `revert` the B side for ∖, and drive the
//! combine door — everything keyed by the F9 records, never by index
//! offsets into correlated arrays (the book's `sonfa[i+inda]`
//! bookkeeping is replaced by side data).
//!
//! # Promotion (lmfkrh both copies)
//!
//! Per completed pair and per solid, the null face's current ring loop
//! is promoted to its own face (`mfkrh`, surface **inherited** — the
//! section faces are transients that die in the seam zip; a boolean
//! intersection polygon is in general non-planar, so no honest plane
//! exists to mint). Which promoted/remaining face is the IN copy is
//! read from the [`CompletedPolygonPair`] loop roles — the F3-chain
//! derivation carried as data ("consistent orientation of null edges"
//! is never consulted).
//!
//! # Component selection (Eq. 15.1)
//!
//! ∩ keeps AinB + BinA; ∪ keeps AoutB + BoutA; ∖ keeps AoutB +
//! `revert`(BinA) — PR 1's functional `revert` op on the carved
//! B-side body (which flips its section faces' loops too; the glue
//! then "does the right thing", pinned by the ∖ acceptance trace).
//! Shells carrying section faces classify by them (mixed ⇒ typed
//! error); uncut shells (components the other body never touched —
//! e.g. an operand void away from the seam) classify by the uncut-shell
//! witness ([`super::shell_witness`]) against the *pristine* other
//! operand.

use geom_core::{Band, Decide};
use slotmap::SecondaryMap;

use super::combine::{GraftMap, graft_solid};
use super::discard::{ChordsAcross, DiscardRow, discard_row};
use super::join::CompletedPolygonPair;
use super::shell_witness::{
    ShellVerdict, check_mutual, debug_assert_contacts_undecisive, kept_shells, shell_verdict,
};
use super::zip::SeamCorrespondence;
use super::{BooleanError, BooleanOp, BooleanReduction, Operand, SideCode};
use crate::body::Body;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, ShellKey, SolidKey, VertexKey};
use crate::euler::FaceSurface;
use crate::live::{BoundaryMember, proven};
use crate::splitting::finish::{carve, single_solid};
use geom_core::Tol;
use std::collections::{BTreeMap, BTreeSet};

/// The finish product: the combined result body (still un-zipped) plus
/// the seam bookkeeping the zip consumes.
pub(super) struct FinishOut<T: geom_core::Real> {
    /// The combined body: one solid, A-kept shells + B-kept shells.
    pub body: Body<T>,
    /// Per completed polygon: the kept A section face and the kept B
    /// section face, in RESULT keys, in completion order.
    pub seams: Vec<(FaceKey, FaceKey)>,
    /// Result-key vertex correspondence across the seam (A-side
    /// surviving end → B-side surviving ends), from the pair records.
    pub vertex_map: SeamCorrespondence,
    /// The B-side graft bridge (contact-record remapping).
    pub graft: GraftMap,
    /// The faces the selection discarded (`BooleanNaming::discards`).
    pub discards: Vec<DiscardRow>,
    /// The split records each operand's carve killed, A's then B's, in
    /// clone keys (`BooleanNaming::carved_lineage`).
    pub carved_lineage: [BTreeMap<crate::entity::EdgeKey, crate::entity::EdgeKey>; 2],
}

/// Which side each operand keeps (Eq. 15.1 as data).
pub(super) fn kept_side(op: BooleanOp, operand: Operand) -> SideCode {
    match (op, operand) {
        (BooleanOp::Union, _) => SideCode::Out,
        (BooleanOp::Intersect, _) => SideCode::In,
        (BooleanOp::Subtract, Operand::A) => SideCode::Out,
        (BooleanOp::Subtract, Operand::B) => SideCode::In,
    }
}

/// Promotes every completed null face of one solid; returns the
/// per-face side map and, per pair, the (in_face, out_face) keys.
type PromotedSides = (SecondaryMap<FaceKey, SideCode>, Vec<(FaceKey, FaceKey)>);

fn promote_solid<T: Decide>(
    body: &mut Body<T>,
    completed: &[CompletedPolygonPair],
    operand: Operand,
) -> Result<PromotedSides, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut side_of: SecondaryMap<FaceKey, SideCode> = SecondaryMap::new();
    let mut in_out = Vec::with_capacity(completed.len());
    for pair in completed {
        let (face, in_loop, out_loop) = match operand {
            Operand::A => (pair.a_face, pair.a_in_loop, pair.a_out_loop),
            Operand::B => (pair.b_face, pair.b_in_loop, pair.b_out_loop),
        };
        let outer = body
            .get_face(face)
            .ok_or_else(|| desync("completed null face no longer resolves"))?
            .outer;
        let ring = if outer == in_loop {
            out_loop
        } else if outer == out_loop {
            in_loop
        } else {
            return Err(desync("null-face outer loop is neither role loop"));
        };
        // The transient section faces inherit the null face's surface
        // (module docs — they die in the zip).
        let promoted = body.mfkrh(ring, FaceSurface::Inherit)?;
        let (in_face, out_face) = if ring == in_loop {
            (promoted.face, face)
        } else {
            (face, promoted.face)
        };
        side_of.insert(in_face, SideCode::In);
        side_of.insert(out_face, SideCode::Out);
        in_out.push((in_face, out_face));
    }
    Ok((side_of, in_out))
}

/// Classifies one distributed shell: section-face seeds first (mixed ⇒
/// typed error), else the uncut-shell verdict against the pristine
/// other operand.
#[allow(clippy::too_many_arguments)]
fn classify_shell<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    shell: ShellKey,
    side_of: &SecondaryMap<FaceKey, SideCode>,
    other: &Body<T>,
    operand: Operand,
    coincident: &[super::SettledPair],
    band: Band,
    tol: Tol,
) -> Result<ShellVerdict, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let shell_data = body
        .get_shell(shell)
        .ok_or_else(|| desync("distributed shell no longer resolves"))?;
    let mut side: Option<SideCode> = None;
    for &face in &shell_data.faces {
        if let Some(&s) = side_of.get(face) {
            match side {
                None => side = Some(s),
                Some(prev) if prev != s => {
                    return Err(BooleanError::TornComponent { operand, shell });
                }
                Some(_) => {}
            }
        }
    }
    if let Some(s) = side {
        return Ok(ShellVerdict::Side(s));
    }
    shell_verdict((body, shell, operand), other, coincident, band, tol)
}

/// Distributes and classifies one solid's shells; returns every
/// distributed shell with its verdict.
#[allow(clippy::too_many_arguments)]
fn classify_solid<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    solid: SolidKey,
    side_of: &SecondaryMap<FaceKey, SideCode>,
    other: &Body<T>,
    operand: Operand,
    coincident: &[super::SettledPair],
    band: Band,
    tol: Tol,
) -> Result<Vec<(ShellKey, ShellVerdict)>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let shells: Vec<ShellKey> = body
        .shells_of_solid(solid)
        .ok_or_else(|| desync("operand solid no longer resolves"))?
        .to_vec();
    let mut all = Vec::new();
    for shell in shells {
        all.extend(body.movefac(shell)?);
    }
    all.into_iter()
        .map(|shell| {
            let verdict =
                classify_shell(body, shell, side_of, other, operand, coincident, band, tol)?;
            Ok((shell, verdict))
        })
        .collect()
}

/// `setopfinish` (module docs): promotion → distribution → Eq. 15.1
/// selection → carve → ∖-revert → the combine door. Consumes the
/// joined reduction; the original operands are read-only witnesses for
/// uncut-component containment.
pub(super) fn setopfinish<T: Decide + crate::props::AtRestPolicy>(
    op: BooleanOp,
    mut red: BooleanReduction<T>,
    connected: &super::join::Connected,
    a_pristine: &Body<T>,
    b_pristine: &Body<T>,
    band: Band,
    tol: Tol,
) -> Result<FinishOut<T>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let completed = &connected.completed[..];

    // **The phase boundary, asserted.** The join holds a scope on each
    // operand body through the one guardless pair in `boolean`
    // (`BooleanReduction::enter_join_surgery`), and nothing about a
    // guardless pair is checked by the compiler — so a close deleted
    // there shows up here, as an operand arriving still inside a
    // scope. The depth is per-BODY and these two are the pipeline's
    // own clones, so the answer is 0 whatever door the pipeline itself
    // is nested in.
    debug_assert_eq!(
        (red.a.open_surgery_scopes(), red.b.open_surgery_scopes()),
        (0, 0),
        "setopfinish: a reduction operand arrived with a surgery scope still open — the \
         join opened one and did not close it, and every operator run on that body from \
         here on skips D1's tier-1 postcondition",
    );

    // ---- Promotion, both solids (F9 roles as data). ----
    let (a_sides, a_in_out) = promote_solid(&mut red.a, completed, Operand::A)?;
    let (b_sides, b_in_out) = promote_solid(&mut red.b, completed, Operand::B)?;

    // ---- Distribution + selection, both solids. ----
    let a_solid =
        single_solid(&red.a).map_err(|_| desync("operand A is not a single-solid body"))?;
    let b_solid =
        single_solid(&red.b).map_err(|_| desync("operand B is not a single-solid body"))?;
    debug_assert_contacts_undecisive(
        &red.contacts,
        (&red.a, b_pristine),
        (&red.b, a_pristine),
        band,
        tol,
    );
    let a_verdicts = classify_solid(
        &mut red.a,
        a_solid,
        &a_sides,
        b_pristine,
        Operand::A,
        &red.coincident,
        band,
        tol,
    )?;
    let b_verdicts = classify_solid(
        &mut red.b,
        b_solid,
        &b_sides,
        a_pristine,
        Operand::B,
        &red.coincident,
        band,
        tol,
    )?;
    check_mutual(
        [(&red.a, &a_verdicts), (&red.b, &b_verdicts)],
        [a_pristine, b_pristine],
    )?;
    let a_kept_shells = kept_shells(op, Operand::A, &a_verdicts);
    let b_kept_shells = kept_shells(op, Operand::B, &b_verdicts);
    if a_kept_shells.is_empty() || b_kept_shells.is_empty() {
        // With ≥ 1 completed polygon both solids hold both components.
        return Err(desync("a seamed operand lost its kept component"));
    }

    // ---- Carve both kept sub-bodies (keys preserved). ----
    let a_kept = carve(&red.a, a_solid, &a_kept_shells)
        .map_err(|_| desync("carving the kept A component failed"))?;
    let mut b_kept = carve(&red.b, b_solid, &b_kept_shells)
        .map_err(|_| desync("carving the kept B component failed"))?;
    let carved_lineage = [
        super::discard::carved_lineage(&red.a, &a_kept),
        super::discard::carved_lineage(&red.b, &b_kept),
    ];

    // ---- ∖: revert the kept B side (Eq. 15.1's (BinA)⁻¹). ----
    if op == BooleanOp::Subtract {
        b_kept = b_kept.reverted();
    }

    // ---- The combine door. ----
    let mut body = a_kept;
    let solid = single_solid(&body).map_err(|_| desync("kept A component is not one solid"))?;
    let graft = graft_solid(&mut body, solid, &b_kept, tol)?;

    // ---- Seam bookkeeping in result keys. ----
    let keep_a = kept_side(op, Operand::A);
    let keep_b = kept_side(op, Operand::B);
    let mut seams = Vec::with_capacity(completed.len());
    for (i, _) in completed.iter().enumerate() {
        let a_face = match keep_a {
            SideCode::In => a_in_out[i].0,
            _ => a_in_out[i].1,
        };
        let b_face_src = match keep_b {
            SideCode::In => b_in_out[i].0,
            _ => b_in_out[i].1,
        };
        let b_face = graft
            .faces
            .get(b_face_src)
            .copied()
            .ok_or_else(|| desync("kept B section face missing from the graft"))?;
        if body.get_face(a_face).is_none() {
            return Err(desync("kept A section face missing from the carve"));
        }
        seams.push((a_face, b_face));
    }

    // ---- Seam vertex correspondence from the pair records: the
    // surviving end of each pair's A edge ↔ the surviving end of its
    // B edge (exactly one each — the other went with the discarded
    // component). ----
    let mut a_attr: SecondaryMap<crate::entity::EdgeKey, crate::null::NullEdge> =
        SecondaryMap::new();
    let mut b_attr: SecondaryMap<crate::entity::EdgeKey, crate::null::NullEdge> =
        SecondaryMap::new();
    for r in &red.null_edges {
        match r.operand {
            Operand::A => a_attr.insert(r.edge, r.attr),
            Operand::B => b_attr.insert(r.edge, r.attr),
        };
    }
    let mut vertex_map = SeamCorrespondence::new();
    // Each A survivor's v-v pairs, each with the B survivor it gave.
    let mut vv_partners: BTreeMap<VertexKey, BTreeMap<(VertexKey, VertexKey), VertexKey>> =
        BTreeMap::new();
    // Each A survivor's pierce runs, each with the B survivor it gave.
    let mut pierce_runs: BTreeMap<VertexKey, Vec<(super::PairSite, VertexKey)>> = BTreeMap::new();
    for pair in &red.null_pairs {
        let aa = a_attr
            .get(pair.a_edge)
            .ok_or_else(|| desync("pair A edge without attribute"))?;
        let ba = b_attr
            .get(pair.b_edge)
            .ok_or_else(|| desync("pair B edge without attribute"))?;
        let (a_below, a_above) = (aa.below_end, aa.above_end);
        let a_survivor = match (
            body.get_vertex(a_below).is_some(),
            body.get_vertex(a_above).is_some(),
        ) {
            (true, false) => a_below,
            (false, true) => a_above,
            _ => return Err(desync("pair A edge has not exactly one surviving end")),
        };
        let b_below = graft.vertices.get(ba.below_end).copied();
        let b_above = graft.vertices.get(ba.above_end).copied();
        let b_survivor = match (b_below, b_above) {
            (Some(v), None) => v,
            (None, Some(v)) => v,
            _ => return Err(desync("pair B edge has not exactly one surviving end")),
        };
        let bs = vertex_map.entry(a_survivor).or_default();
        bs.insert(b_survivor);
        let pairs = vv_partners.entry(a_survivor).or_default();
        let mut one_each = true;
        if let super::PairSite::VertexVertex(c) = pair.site {
            one_each = pairs
                .insert((c.a, c.b), b_survivor)
                .is_none_or(|old| old == b_survivor);
        }
        if let super::PairSite::VertexAOnFaceB(_) | super::PairSite::VertexBOnFaceA(_) = pair.site {
            pierce_runs
                .entry(a_survivor)
                .or_default()
                .push((pair.site, b_survivor));
        }
        // An A vertex several crossing pairs cut lies on one seam per
        // pair, with that pair's B vertex: every B correspondent is one
        // pair's, and the pairs share their A vertex. A piercing vertex
        // that keeps several In runs lies on its seam once per run, with
        // that run's ring copy; a ring copy, once per run it holds, of
        // the pierces of its face at its point (`vtxfac::mint_ring`).
        let shared_cut = one_each
            && pairs
                .keys()
                .all(|p| pairs.keys().next().is_some_and(|q| q.0 == p.0))
            && pairs.values().copied().collect::<BTreeSet<_>>() == *bs;
        let one_ring = pierce_runs.get(&a_survivor).is_some_and(|runs| {
            runs.iter().all(|&(site, _)| red.one_ring(site, runs[0].0))
                && runs.iter().map(|&(_, b)| b).collect::<BTreeSet<_>>() == *bs
        });
        if bs.len() > 1 && !shared_cut && !one_ring {
            return Err(desync("conflicting seam vertex correspondence"));
        }
    }

    // A kept vertex in result keys: A's survive the carve in place,
    // B's through the graft.
    let a_kept = |v: VertexKey| body.get_vertex(v).is_some().then_some(v);
    let b_kept = |v: VertexKey| graft.vertices.get(v).copied();
    let mut discards = discarded(
        &red,
        a_solid,
        &a_kept_shells,
        (&a_sides, &a_in_out),
        (Operand::A, &a_kept),
        (&connected.fates, &b_kept),
    )?;
    discards.extend(discarded(
        &red,
        b_solid,
        &b_kept_shells,
        (&b_sides, &b_in_out),
        (Operand::B, &b_kept),
        (&connected.fates, &a_kept),
    )?);

    Ok(FinishOut {
        body,
        seams,
        vertex_map,
        graft,
        discards,
        carved_lineage,
    })
}

/// The one result key `kept` holds, if it holds exactly one.
fn one_kept(kept: &[(VertexKey, VertexKey)]) -> Option<VertexKey> {
    let keys: BTreeSet<VertexKey> = kept.iter().map(|&(_, r)| r).collect();
    keys.first().copied().filter(|_| keys.len() == 1)
}

/// The members of every cycle bounding `face`, a section face the
/// discard carries as a key: its miss is
/// [`BooleanError::JoinDesync`], typed.
///
/// # Panics
///
/// Where a record past `face` (a loop, a member's edge, a lone vertex's
/// point) does not resolve or a loop walk does not close (D2 row 4). `body` is an operand mid-operation, whose links
/// hold by [`crate::live::OPERATORS_KEEP_LINKS`].
fn section_boundary<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<HalfEdgeKey>, BooleanError> {
    let f = body.get_face(face).ok_or(BooleanError::JoinDesync {
        what: "a section face no longer resolves",
    })?;
    Ok(body
        .face_boundary_linked(face, f)
        .filter_map(|member| match member {
            BoundaryMember::Edge { he, .. } => Some(he),
            BoundaryMember::Isolated { .. } => None,
        })
        .collect())
}

/// The discarded faces of one operand solid (`boolean::discard`): every
/// face of a shell the selection dropped, the section faces aside. A
/// stretch it bordered a kept face along runs along a section face; the
/// kept side's copy of each end is the other end of one of that end's
/// null edges — the one end among them that survived into the result,
/// which `kept_vertex` reads in result keys, as the seam vertex map
/// picks its survivor. A vertex that several crossing pairs cut keeps
/// a copy per pair, and the stretch's is the one on the section face's
/// twin (`in_out`); where the twin passes several copies of an end,
/// the stretch's ends are those of its own copy on the twin
/// ([`twin_copy`]). A vertex several runs of one pierce cut is that
/// case; so are some near-tangent poses of a single run (a convex
/// corner tilted 1e-6 off an edge, in PR 4026's review), and a section
/// face that passes one vertex twice (a curved wall's arc and a line
/// between the same two points), where the twin also passes several
/// copies of an end. `chords` is the fate cuts and the other operand's
/// vertices in result keys, for the stretches a face borders across a
/// one-sided chord ([`ChordsAcross`]).
#[allow(clippy::type_complexity)]
fn discarded<T: Decide>(
    red: &BooleanReduction<T>,
    solid: SolidKey,
    kept: &[ShellKey],
    (sides, in_out): (&SecondaryMap<FaceKey, SideCode>, &[(FaceKey, FaceKey)]),
    (operand, kept_vertex): (Operand, &dyn Fn(VertexKey) -> Option<VertexKey>),
    chords: (
        &super::fate::FateCuts,
        &dyn Fn(VertexKey) -> Option<VertexKey>,
    ),
) -> Result<Vec<DiscardRow>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let body = match operand {
        Operand::A => &red.a,
        Operand::B => &red.b,
    };
    let copy = super::NullCopies::of_operand(&red.null_edges, operand);
    let on_face = |face: FaceKey| -> Result<BTreeSet<VertexKey>, BooleanError> {
        Ok(section_boundary(body, face)?
            .into_iter()
            .map(|he| proven(&body.half_edges, he, EntityId::HalfEdge).start)
            .collect())
    };
    let twin_of = |across: FaceKey| {
        in_out
            .iter()
            .find_map(|&(i, o)| match (i == across, o == across) {
                (true, _) => Some(o),
                (_, true) => Some(i),
                _ => None,
            })
    };
    // The copies of `v` that survive, each as its body key and its
    // result key, narrowed to the twin's where several do.
    let kept_copies =
        |v: VertexKey, across: FaceKey| -> Result<Vec<(VertexKey, VertexKey)>, BooleanError> {
            let copies: Vec<VertexKey> = copy.of(v).into_iter().filter(|&k| k != v).collect();
            if copies.is_empty() {
                return Err(desync("a section vertex has no null-edge copy"));
            }
            let survivors = |twin: Option<&BTreeSet<VertexKey>>| -> Vec<(VertexKey, VertexKey)> {
                copies
                    .iter()
                    .filter(|&k| twin.is_none_or(|t| t.contains(k)))
                    .filter_map(|&k| kept_vertex(k).map(|r| (k, r)))
                    .collect()
            };
            let kept = survivors(None);
            if one_kept(&kept).is_some() {
                return Ok(kept);
            }
            Ok(survivors(Some(
                &twin_of(across).map_or_else(|| Ok(BTreeSet::new()), on_face)?,
            )))
        };
    let kept_ends = |mate: HalfEdgeKey, u, w| {
        let across = body
            .face_of_half_edge(mate)
            .ok_or_else(|| desync("a section half-edge has no face"))?;
        let (us, ws) = (kept_copies(u, across)?, kept_copies(w, across)?);
        if let (Some(ku), Some(kw)) = (one_kept(&us), one_kept(&ws)) {
            return Ok((ku, kw));
        }
        // Several copies survive on the twin, which passes an end more
        // than once: the stretch's kept ends are its own copy's.
        let refuse = || desync("a section vertex's null-edge copies have not exactly one kept end");
        let twin = twin_of(across).ok_or_else(refuse)?;
        let at = twin_copy(
            (&section_loops(body, across)?, &section_loops(body, twin)?),
            mate,
            |he| proven(&body.half_edges, he, EntityId::HalfEdge).start,
            |x, y| copy.of(x).contains(&y),
        )
        .ok_or_else(refuse)?;
        let start = proven(&body.half_edges, at, EntityId::HalfEdge).start;
        let end = body.proven_half_edge_end(at);
        let kept = |b: VertexKey, of: &[(VertexKey, VertexKey)]| {
            of.iter().find(|&&(k, _)| k == b).map(|&(_, r)| r)
        };
        kept(start, &us).zip(kept(end, &ws)).ok_or_else(refuse)
    };
    let kept_across = |f: FaceKey| sides.contains_key(f);
    let own_to_result =
        |v: VertexKey| super::discard::own_in_result(red, operand, (kept_vertex, chords.1), v);
    let chords = ChordsAcross {
        cuts: chords.0,
        to_result: chords.1,
        own_to_result: &own_to_result,
    };
    let mut out = Vec::new();
    for &shell in body
        .shells_of_solid(solid)
        .ok_or_else(|| desync("an operand solid no longer resolves"))?
    {
        if kept.contains(&shell) {
            continue;
        }
        for &face in &body
            .get_shell(shell)
            .ok_or_else(|| desync("a discarded shell no longer resolves"))?
            .faces
        {
            if !sides.contains_key(face) && !chords.cuts.is_substituted(operand, face) {
                out.push(discard_row(
                    body,
                    face,
                    operand,
                    &kept_across,
                    &kept_ends,
                    Some(&chords),
                )?);
            }
        }
    }
    Ok(out)
}

/// The half-edge cycle of each loop of `face`, outer first: as
/// [`section_boundary`], one list per loop.
///
/// # Panics
///
/// As [`section_boundary`].
fn section_loops<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<Vec<HalfEdgeKey>>, BooleanError> {
    let f = body.get_face(face).ok_or(BooleanError::JoinDesync {
        what: "a section face no longer resolves",
    })?;
    Ok(body
        .face_boundary_by_loop(face, f)
        .map(|(_, members)| {
            members
                .into_iter()
                .filter_map(|member| match member {
                    BoundaryMember::Edge { he, .. } => Some(he),
                    BoundaryMember::Isolated { .. } => None,
                })
                .collect::<Vec<_>>()
        })
        .filter(|cycle| !cycle.is_empty())
        .collect())
}

/// **A section half-edge's copy on its twin**: `face` is the cycles of a
/// section face's loops and `twin` its twin's, a null face's two role
/// loops, which run one polygon antiparallel, each vertex of one a
/// null-edge copy of the other's at the mirrored place (`copies`).
/// Returns the twin half-edge at `he`'s mirrored place on the one twin
/// loop that mirrors `he`'s loop, which runs between copies of `he`'s
/// ends the other way; `None` where no twin loop mirrors it, or more
/// than one mirror does.
///
/// The place, not the ends, says which copy it is: a face may pass one
/// vertex twice (an arc and a line between the same two points), and
/// then two twin half-edges join copies of the same two ends.
fn twin_copy(
    (face, twin): (&[Vec<HalfEdgeKey>], &[Vec<HalfEdgeKey>]),
    he: HalfEdgeKey,
    start: impl Fn(HalfEdgeKey) -> VertexKey,
    copies: impl Fn(VertexKey, VertexKey) -> bool,
) -> Option<HalfEdgeKey> {
    let own = face.iter().find(|l| l.contains(&he))?;
    let m = own.len();
    let i = own.iter().position(|&h| h == he)?;
    // A mirror `(t, s)`: the twin loop `t`'s vertex `j` copies `own`'s
    // `s − j`.
    let mut mirrors = twin.iter().filter(|t| t.len() == m).flat_map(|t| {
        (0..m)
            .filter(|&s| (0..m).all(|j| copies(start(own[(s + m - j) % m]), start(t[j]))))
            .map(move |s| (t, s))
    });
    let (t, s) = mirrors.next()?;
    if mirrors.next().is_some() {
        return None;
    }
    // `he` runs `x_i → x_{i+1}`; its copy runs from `x_{i+1}`'s copy, at
    // `j = s − i − 1`.
    Some(t[(s + 2 * m - i - 1) % m])
}

/// **`section_boundary`: a stale section face refuses typed; a broken
/// walk past one that resolves panics**, where it refused `JoinDesync`
/// ("not walkable").
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_hop_rows {
    use super::*;
    use crate::live::OPERATORS_KEEP_LINKS;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};

    #[test]
    fn a_stale_section_face_refuses_and_a_broken_walk_panics() {
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let face = body.faces().next().map(|(k, _)| k).unwrap();
        let members = section_boundary(&body, face).unwrap();
        assert_eq!(members.len(), 4, "a cube face's four sides");
        let mut stale = body.clone();
        stale.faces.remove(face);
        assert!(
            matches!(
                section_boundary(&stale, face),
                Err(BooleanError::JoinDesync {
                    what: "a section face no longer resolves"
                })
            ),
            "a section face that does not resolve refuses typed"
        );
        body.half_edges.remove(members[1]);
        assert_torn_op_panics(
            "section_boundary",
            &mut body,
            &["the loop walk from", ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| section_boundary(b, face).map(|m| m.len()),
        );
    }
}

/// **`twin_copy` reads a half-edge's copy off its place, not its ends.**
/// Each twin is built by running each loop of the face backwards on fresh
/// copies of its vertices (one per visit), each twin half-edge made as
/// the copy of one face half-edge; the twin's cycles are then turned and
/// their order changed, so the expected copy is known from the build.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod twin_copy_rows {
    use super::twin_copy;
    use crate::entity::{HalfEdgeKey, VertexKey};
    use std::collections::BTreeMap;

    /// A face's loops and its twin's, each half-edge's start, each copy
    /// pair, and each face half-edge's copy.
    #[derive(Default)]
    struct Pair {
        face: Vec<Vec<HalfEdgeKey>>,
        twin: Vec<Vec<HalfEdgeKey>>,
        start: BTreeMap<HalfEdgeKey, VertexKey>,
        copies: Vec<(VertexKey, VertexKey)>,
        copy_of: BTreeMap<HalfEdgeKey, HalfEdgeKey>,
    }

    /// `loops[l]` lists loop `l`'s vertices (equal labels are one vertex,
    /// across loops too); twin loop `l` is turned by `turn[l]` places,
    /// and the twin's loops come in reverse order.
    fn pair(loops: &[&[usize]], turn: &[usize]) -> Pair {
        let mut hk = slotmap::SlotMap::<HalfEdgeKey, ()>::with_key();
        let mut vk = slotmap::SlotMap::<VertexKey, ()>::with_key();
        let mut vertex: BTreeMap<usize, VertexKey> = BTreeMap::new();
        let mut out = Pair::default();
        for (l, labels) in loops.iter().enumerate() {
            let m = labels.len();
            let xs: Vec<VertexKey> = labels
                .iter()
                .map(|&x| *vertex.entry(x).or_insert_with(|| vk.insert(())))
                .collect();
            let face: Vec<HalfEdgeKey> = (0..m).map(|_| hk.insert(())).collect();
            // One copy per visit, of the vertex the face visits there.
            let ys: Vec<VertexKey> = (0..m).map(|_| vk.insert(())).collect();
            for i in 0..m {
                out.start.insert(face[i], xs[i]);
                out.copies.push((xs[i], ys[i]));
            }
            // The face's `x_i → x_{i+1}`, backwards on copies: from the
            // copy of visit `i + 1` to that of visit `i`.
            let mut twin: Vec<HalfEdgeKey> = (0..m)
                .rev()
                .map(|i| {
                    let t = hk.insert(());
                    out.start.insert(t, ys[(i + 1) % m]);
                    out.copy_of.insert(face[i], t);
                    t
                })
                .collect();
            twin.rotate_left(turn[l] % m);
            out.face.push(face);
            out.twin.push(twin);
        }
        out.twin.reverse();
        out
    }

    impl Pair {
        fn read(&self, he: HalfEdgeKey) -> Option<HalfEdgeKey> {
            twin_copy(
                (&self.face, &self.twin),
                he,
                |h| self.start[&h],
                |x, y| self.copies.contains(&(x, y)),
            )
        }
    }

    /// A face `a b c d e d` passes `d` twice (a line `d → e` and an arc
    /// `e → d` between the same points): each maps to its own copy, at
    /// every turn of the twin, though both join a copy of `d` to the copy
    /// of `e`. So does every other half-edge.
    #[test]
    fn a_face_passing_a_vertex_twice_maps_each_half_edge_to_its_own_copy() {
        for turn in 0..6 {
            let p = pair(&[&[0, 1, 2, 3, 4, 3]], &[turn]);
            for &he in &p.face[0] {
                assert_eq!(p.read(he), Some(p.copy_of[&he]), "turn {turn}");
            }
        }
    }

    /// A face with a ring: each half-edge, outer or ring, maps to its copy
    /// on the twin loop that mirrors its own, the twin's loops in the
    /// other order. Flattening the loops into one cycle misreads these.
    #[test]
    fn a_face_with_a_ring_maps_each_loop_onto_its_own_twin_loop() {
        for (t0, t1) in [(0, 0), (2, 1), (5, 3)] {
            let p = pair(&[&[0, 1, 2, 3, 4, 5], &[6, 7, 8, 9]], &[t0, t1]);
            for he in p.face.iter().flatten() {
                assert_eq!(p.read(*he), Some(p.copy_of[he]), "turns {t0} {t1}");
            }
        }
    }

    /// No single mirror: a loop `a b a b` aligns two ways, and a loop
    /// whose twin loops are all of other lengths aligns none.
    #[test]
    fn a_loop_no_single_mirror_aligns_reads_none() {
        let p = pair(&[&[0, 1, 0, 1]], &[0]);
        assert_eq!(p.read(p.face[0][0]), None, "a symmetric loop");
        let mut q = pair(&[&[0, 1, 2]], &[0]);
        q.twin[0].pop();
        assert_eq!(q.read(q.face[0][0]), None, "loops of two lengths");
    }
}
