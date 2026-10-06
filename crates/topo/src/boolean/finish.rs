//! `setopfinish` (ch. 15 §15.8, Program 15.15 re-derived): promote
//! each completed section-polygon pair into IN/OUT section faces in
//! BOTH solids, distribute components, select per **Eq. 15.1**, carve
//! the kept components, weld each kept pinch into one vertex
//! (`weld_pinches`), `revert` the B side for ∖, and drive the
//! combine door — everything keyed by the F9 records, never by index
//! offsets into correlated arrays (the book's `sonfa[i+inda]`
//! bookkeeping is replaced by side data). It also hosts one pass the
//! op stage runs after the zips, `weld_pierce_copies`, which reads the
//! pierce copies `setopfinish` collects (`FinishOut::pierce_copies`).
//!
//! # Promotion (lmfkrh both copies)
//!
//! Per completed pair and per solid, the null face's current ring loop
//! is promoted to its own face (`mfkrh`, surface **inherited** — the
//! section faces are transients that die in the seam zip; a boolean
//! intersection polygon is in general non-planar, so no honest plane
//! exists to mint). Which promoted/remaining face is the IN copy is
//! read from the [`crate::null::NullFacePair::Boolean`] loop roles — the F3-chain
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
use super::discard::{DiscardRow, HeldInto, discard_row};
use super::join::CompletedPolygonPair;
use super::shell_witness::{
    ShellVerdict, check_mutual, debug_assert_contacts_undecisive, kept_shells, shell_verdict,
};
use super::zip::{Fusions, Joint, SeamCorrespondence, fuse_by_joint};
use super::{BooleanError, BooleanOp, BooleanReduction, Operand, SideCode, one_vertex};
use crate::body::Body;
use crate::entity::{
    EntityId, Face, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, SolidKey, VertexKey,
};
use crate::euler::FaceSurface;
use crate::live::proven;
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
    /// The pinch welds' face fragment rows `(new face, divided-from
    /// face)`, A-clone keys.
    pub weld_fragments_a: Vec<(FaceKey, FaceKey)>,
    /// The pinch welds' face fragment rows, B-clone keys.
    pub weld_fragments_b: Vec<(FaceKey, FaceKey)>,
    /// The A-side pinch welds' vertex fusions `(dead, kept)`, result
    /// keys.
    pub weld_merges_a: Fusions,
    /// The B-side pinch welds' vertex fusions, B-clone keys: they ran
    /// before the graft, so a dead key has no result key.
    pub weld_merges_b: Fusions,
    /// The kept copies of each pierce several runs cut, both operands',
    /// in result keys ([`weld_pierce_copies`]).
    pub pierce_copies: Vec<Vec<VertexKey>>,
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
        body.clear_null_face_pair(face);
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
    let mut a_kept = carve(&red.a, a_solid, &a_kept_shells)
        .map_err(|_| desync("carving the kept A component failed"))?;
    let mut b_kept = carve(&red.b, b_solid, &b_kept_shells)
        .map_err(|_| desync("carving the kept B component failed"))?;

    // ---- Pinches: one vertex where two pierces of a kept face meet. ----
    let a_welds = weld_pinches(
        &mut a_kept,
        (Operand::A, &connected.a_fragments, &a_sides),
        &red,
        band,
        tol,
    )?;
    let b_welds = weld_pinches(
        &mut b_kept,
        (Operand::B, &connected.b_fragments, &b_sides),
        &red,
        band,
        tol,
    )?;

    // ---- ∖: revert the kept B side (Eq. 15.1's (BinA)⁻¹). ----
    if op == BooleanOp::Subtract {
        b_kept = b_kept.revert().map_err(BooleanError::Revert)?;
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
        let (a_below, a_above) = (a_welds.kept(aa.below_end), a_welds.kept(aa.above_end));
        let a_survivor = match (
            body.get_vertex(a_below).is_some(),
            body.get_vertex(a_above).is_some(),
        ) {
            (true, false) => a_below,
            (false, true) => a_above,
            _ => return Err(desync("pair A edge has not exactly one surviving end")),
        };
        let b_below = graft.vertices.get(b_welds.kept(ba.below_end)).copied();
        let b_above = graft.vertices.get(b_welds.kept(ba.above_end)).copied();
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
        // A welded pinch lies on a seam once per pierce it fused, with
        // B's vertex of each; an A vertex several crossing pairs cut
        // lies on one seam per pair, with that pair's B vertex: every
        // B correspondent is one pair's, and the pairs share their A
        // vertex. A piercing vertex that keeps several In runs lies on
        // its seam once per run, with that run's ring copy.
        let shared_cut = one_each
            && pairs
                .keys()
                .all(|p| pairs.keys().next().is_some_and(|q| q.0 == p.0))
            && pairs.values().copied().collect::<BTreeSet<_>>() == *bs;
        let one_pierce = pierce_runs.get(&a_survivor).is_some_and(|runs| {
            runs.iter().all(|&(site, _)| site == runs[0].0)
                && runs.iter().map(|&(_, b)| b).collect::<BTreeSet<_>>() == *bs
        });
        if bs.len() > 1
            && !shared_cut
            && !one_pierce
            && !a_welds.merges.rows().iter().any(|&(_, k)| k == a_survivor)
        {
            return Err(desync("conflicting seam vertex correspondence"));
        }
    }

    // A kept vertex in result keys: A's survive the carve in place,
    // B's through the graft.
    let a_kept = |v: VertexKey| {
        let v = a_welds.kept(v);
        body.get_vertex(v).is_some().then_some(v)
    };
    let b_kept = |v: VertexKey| graft.vertices.get(b_welds.kept(v)).copied();
    // The kept copies of each pierce several runs cut.
    let mut pierces: Vec<(super::PairSite, usize, Vec<VertexKey>)> = Vec::new();
    for pair in &red.null_pairs {
        if !matches!(
            pair.site,
            super::PairSite::VertexAOnFaceB(_) | super::PairSite::VertexBOnFaceA(_)
        ) {
            continue;
        }
        let (aa, ba) = match (a_attr.get(pair.a_edge), b_attr.get(pair.b_edge)) {
            (Some(&aa), Some(&ba)) => (aa, ba),
            _ => return Err(desync("pair edge without attribute")),
        };
        let kept = [aa.below_end, aa.above_end]
            .into_iter()
            .filter_map(a_kept)
            .chain([ba.below_end, ba.above_end].into_iter().filter_map(b_kept));
        match pierces.iter_mut().find(|(site, ..)| *site == pair.site) {
            Some((_, runs, copies)) => {
                *runs += 1;
                copies.extend(kept);
            }
            None => pierces.push((pair.site, 1, kept.collect())),
        }
    }
    let pierce_copies = pierces
        .into_iter()
        .filter(|&(_, runs, _)| runs > 1)
        .map(|(.., copies)| copies)
        .collect();
    let mut discards = discarded(
        &red,
        a_solid,
        &a_kept_shells,
        (&a_sides, &a_in_out),
        (Operand::A, &a_kept),
        (&connected.a_fragments, &b_kept),
    )?;
    discards.extend(discarded(
        &red,
        b_solid,
        &b_kept_shells,
        (&b_sides, &b_in_out),
        (Operand::B, &b_kept),
        (&connected.b_fragments, &a_kept),
    )?);

    Ok(FinishOut {
        body,
        seams,
        vertex_map,
        graft,
        discards,
        weld_fragments_a: a_welds.fragments,
        weld_fragments_b: b_welds.fragments,
        weld_merges_a: a_welds.merges,
        weld_merges_b: b_welds.merges,
        pierce_copies,
    })
}

/// **A pierce's copies are one vertex where a face meets them.** A
/// vertex several runs pierce mints a null edge per run on each side,
/// and each run's zip fuses that run's two kept copies. Where both
/// sides keep a copy per run, the runs' vertices stay apart on one
/// point, and two of them are joined across a face whose boundary runs
/// through both, by [`weld_pair`], the fusion [`weld_pinches`] makes.
/// Copies no face meets stay apart, on their one point, as unwelded
/// pierces do. Two things differ from [`weld_pinches`], which runs
/// before the zips: it admits only the pierced face's fragments, where
/// this pass, after them, admits any face (the section faces are gone,
/// and both vertices are one pierce's copies); and a weld here that
/// would divide a face is refused, as no fragment row can record it
/// after the graft. Copies of one pierce not on one point refuse too.
/// `fused` is the zips' fusions `(dead, kept)` in result keys, which
/// `groups` (from [`FinishOut::pierce_copies`]) is read through;
/// returns this pass's fusions. It runs from the op stage after the
/// zips (`ops::boolean_op_recut`), not from [`setopfinish`].
pub(super) fn weld_pierce_copies<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    groups: &[Vec<VertexKey>],
    fused: &Fusions,
    tol: Tol,
) -> Result<Fusions, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let band = Band::linear(tol)?;
    let mut merges = fused.clone();
    let mut welds = Fusions::default();
    for group in groups {
        loop {
            let mut live: Vec<VertexKey> = Vec::new();
            for &v in group {
                let k = merges.survivor(v);
                if body.get_vertex(k).is_some() && !live.contains(&k) {
                    live.push(k);
                }
            }
            let mut site = None;
            'pairs: for (i, &u) in live.iter().enumerate() {
                for &w in &live[i + 1..] {
                    if let Some((_, joint)) = pinch_site(body, u, w, |_| true)? {
                        site = Some((u, w, joint));
                        break 'pairs;
                    }
                }
            }
            let Some((u, w, joint)) = site else {
                break;
            };
            // Both live by the filter above, so only the point is a read.
            let point = |v| body.resolve_vertex_point(v, crate::live::Proven);
            let p = point(u);
            if !one_vertex(p, point(w), band).map_err(|diag| BooleanError::Escalated {
                decision: super::BooleanDecision::VertexOnVertex,
                diag,
            })? {
                return Err(desync("a pierce's copies are not on one point"));
            }
            if let Joint::Chord { .. } = joint {
                return Err(desync("a pierce's copies divide a face the zips kept"));
            }
            let (fusion, _) = weld_pair(body, (u, w), joint, p, tol)?;
            merges.push(fusion)?;
            welds.push(fusion)?;
        }
    }
    Ok(welds)
}

/// Joins `u` and `w` at `joint` ([`fuse_by_joint`]) and checks that the
/// pair fused: one of them dead, the other live. Returns the fusion
/// `(dead, kept)` and the face a chord divided off.
fn weld_pair<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    (u, w): (VertexKey, VertexKey),
    joint: Joint,
    p: geom_core::Point3<T>,
    tol: Tol,
) -> Result<((VertexKey, VertexKey), Option<FaceKey>), BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let ((dead, kept), made) = fuse_by_joint(body, joint, p, desync, tol)?;
    if ![[u, w], [w, u]].contains(&[dead, kept])
        || body.get_vertex(dead).is_some()
        || body.get_vertex(kept).is_none()
    {
        return Err(desync("a pinch weld did not fuse its pair"));
    }
    Ok(((dead, kept), made))
}

/// The pinch welds of one kept side: each fusion `(dead, kept)` and the
/// face each one divided.
#[derive(Default)]
struct Welds {
    merges: Fusions,
    fragments: Vec<(FaceKey, FaceKey)>,
}

impl Welds {
    /// The vertex `v` survives as.
    fn kept(&self, v: VertexKey) -> VertexKey {
        self.merges.survivor(v)
    }
}

/// **Two pierces of one face that meet at a point are one vertex.**
///
/// Two edges of the piercing body that coincide (a contact the other
/// operand recorded) pierce a face at one point, and each pierce mints
/// its own ring vertex. Where both survive on one kept fragment of that
/// face, the fragment's boundary meets itself there, and the order that
/// met the point as an existing vertex built that meeting as one
/// vertex. So the two are joined by a zero-length edge and the edge
/// collapsed: across the outer loop it divides the fragment, two
/// regions meeting at the vertex; across one ring it divides the hole,
/// two holes meeting there; across two loops (holes touching at a
/// corner) it joins them into one.
///
/// After the zips, [`weld_pierce_copies`] joins one pierce's own copies
/// by the same fusion ([`weld_pair`]).
///
/// The site is read from lineage: the pierced face's fragments
/// (`lineage`, `(new face, divided-from face)` rows), section faces
/// (`sections`) aside. Pierces that survive on different fragments, or
/// meet only on a section face, stay apart, as the contact's own
/// vertices do.
fn weld_pinches<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    (operand, lineage, sections): (
        Operand,
        &[(FaceKey, FaceKey)],
        &SecondaryMap<FaceKey, SideCode>,
    ),
    red: &BooleanReduction<T>,
    band: Band,
    tol: Tol,
) -> Result<Welds, BooleanError> {
    // A pierce survives as whichever of its null-edge copies the kept
    // side holds.
    let copies = super::NullCopies::of_operand(&red.null_edges, operand);
    let copies_of = |v: VertexKey| -> BTreeSet<VertexKey> { copies.of(v).into_iter().collect() };
    let mut by_face: BTreeMap<FaceKey, Vec<BTreeSet<VertexKey>>> = BTreeMap::new();
    for r in red.pierce_rings.iter().filter(|r| r.operand == operand) {
        by_face
            .entry(r.face)
            .or_default()
            .push(copies_of(r.ring_vertex));
    }
    let mut welds = Welds::default();
    // Every pair of one face's pierces, through their copies: quadratic,
    // over the few edges of the other operand that pierce one face, and a
    // pair costs one point comparison unless it coincides.
    for (&pierced, pierces) in &by_face {
        for (i, us) in pierces.iter().enumerate() {
            for (&u0, &w0) in pierces[i + 1..]
                .iter()
                .flat_map(|ws| us.iter().flat_map(move |u| ws.iter().map(move |w| (u, w))))
            {
                let (u, w) = (welds.kept(u0), welds.kept(w0));
                if u == w || body.get_vertex(u).is_none() || body.get_vertex(w).is_none() {
                    continue;
                }
                // Both resolved just above, so only the point is a read.
                let point = |v| body.resolve_vertex_point(v, crate::live::Proven);
                let pu = point(u);
                // No row reaches the escalation: two pierces a band
                // apart need the piercing operand's two edges a band
                // apart. Seen from one germ the two sites lie in one
                // half-turn unless they straddle its ends, and the join
                // orders them there before the weld runs — escalating
                // the waist at `bool_join_arc_travel` along a conic, at
                // `bool_join_nearest` on a straight line; a narrower one
                // the profile insert refuses at the operand's build. An
                // operand built elsewhere, or two sites the half-turn
                // parts, can still bring them here.
                if !one_vertex(pu, point(w), band).map_err(|diag| BooleanError::Escalated {
                    decision: super::BooleanDecision::VertexOnVertex,
                    diag,
                })? {
                    continue;
                }
                let fragments = descendants(pierced, lineage.iter().chain(&welds.fragments));
                let in_lineage = |f: FaceKey| fragments.contains(&f) && !sections.contains_key(f);
                let Some((face, joint)) = pinch_site(body, u, w, in_lineage)? else {
                    continue;
                };
                let ((dead, kept), made) = weld_pair(body, (u, w), joint, pu, tol)?;
                if let Some(made) = made {
                    welds.fragments.push((made, face));
                }
                welds.merges.push((dead, kept))?;
            }
        }
    }
    Ok(welds)
}

/// `face` and every face divided from it, through `rows` (`(new face,
/// divided-from face)`, in any order).
fn descendants<'r>(
    face: FaceKey,
    rows: impl Iterator<Item = &'r (FaceKey, FaceKey)>,
) -> BTreeSet<FaceKey> {
    let rows: Vec<_> = rows.collect();
    let mut out = BTreeSet::from([face]);
    let mut todo = vec![face];
    while let Some(f) = todo.pop() {
        for &&(new, from) in &rows {
            if from == f && out.insert(new) {
                todo.push(new);
            }
        }
    }
    out
}

/// The one face `allowed` admits whose boundary runs through both `u`
/// and `w`, each once, and the joint between the half-edges leaving
/// them: a chord when the outer loop holds both, a hole when one ring
/// does, else across their two loops, into the face's outer loop when
/// it is one of them. `None` when no such face holds both.
///
/// # Panics
///
/// Where `u`, which both callers resolved just before, or a record
/// past it does not resolve, or a walk does not close (D2 row 4): its
/// orbit, each face and loop on it, and each member's start. `body` is
/// an operand mid-operation: carved, whose links the carve leaves
/// resolving ([`carve`] checks that it drops only records no kept
/// record names), then partly welded, whose links hold by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
pub(super) fn pinch_site<T: Decide>(
    body: &Body<T>,
    u: VertexKey,
    w: VertexKey,
    allowed: impl Fn(FaceKey) -> bool,
) -> Result<Option<(FaceKey, Joint)>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut site = None;
    for face in body
        .faces_of_vertex_linked(u)
        .into_iter()
        .filter(|&f| allowed(f))
    {
        let f = proven(&body.faces, face, EntityId::Face);
        let mut hus = Vec::new();
        let mut hws = Vec::new();
        for (l, boundary, members) in face_cycles(body, face, f) {
            if let LoopBoundary::Empty { vertex } = boundary
                && (vertex == u || vertex == w)
            {
                return Err(desync("a kept pierce vertex stands alone on its face"));
            }
            for he in members {
                let v = proven(&body.half_edges, he, EntityId::HalfEdge).start;
                if v == u {
                    hus.push((l, he));
                } else if v == w {
                    hws.push((l, he));
                }
            }
        }
        let here = match (hus.as_slice(), hws.as_slice()) {
            (_, []) => continue,
            (&[(lu, hu)], &[(lw, hw)]) if lu == lw && lu == f.outer => {
                Joint::Chord { he1: hu, he2: hw }
            }
            (&[(lu, hu)], &[(lw, hw)]) if lu == lw => Joint::Hole {
                face,
                he1: hu,
                he2: hw,
            },
            (&[(_, hu)], &[(lw, hw)]) if lw == f.outer => Joint::Loops {
                target: hw,
                ring: hu,
            },
            (&[(_, hu)], &[(_, hw)]) => Joint::Loops {
                target: hu,
                ring: hw,
            },
            _ => return Err(desync("a pinch face runs through a pierce vertex twice")),
        };
        if site.replace((face, here)).is_some() {
            return Err(desync("two fragments of a pierced face meet one pinch"));
        }
    }
    Ok(site)
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
/// Where a record past `face` does not resolve or a loop walk does not
/// close (D2 row 4). `body` is an operand mid-operation, whose links
/// hold by [`crate::live::OPERATORS_KEEP_LINKS`].
fn section_boundary<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<HalfEdgeKey>, BooleanError> {
    let f = body.get_face(face).ok_or(BooleanError::JoinDesync {
        what: "a section face no longer resolves",
    })?;
    Ok(face_cycles(body, face, f)
        .flat_map(|(_, _, members)| members)
        .collect())
}

/// Each loop of `face` (whose record is `f`), outer first: its key, its
/// boundary, and its members in walk order (none for a lone vertex).
///
/// # Panics
///
/// Where a loop `f` names does not resolve or a loop walk does not
/// close (D2 row 4).
fn face_cycles<'a, T: Decide>(
    body: &'a Body<T>,
    face: FaceKey,
    f: &'a Face,
) -> impl Iterator<Item = (LoopKey, LoopBoundary, Vec<HalfEdgeKey>)> + 'a {
    body.face_loops_linked(face, f).map(|(l, lp)| {
        let members = match lp.boundary {
            LoopBoundary::Cycle { first } => body.loop_walk(first).closed("loop", first),
            LoopBoundary::Empty { .. } => Vec::new(),
        };
        (l, lp.boundary, members)
    })
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
/// the stretch's ends are the two copies one edge of the twin joins.
/// A vertex several runs of one pierce cut is that case; so are some
/// near-tangent poses of a single run (a convex corner tilted 1e-6
/// off an edge, in PR 4026's review), where the twin also passes
/// several copies of an end. `held` is this operand's chord-split rows
/// and the other operand's vertices in result keys, for the held
/// stretches (`DiscardRow::held`).
#[allow(clippy::type_complexity)]
fn discarded<T: Decide>(
    red: &BooleanReduction<T>,
    solid: SolidKey,
    kept: &[ShellKey],
    (sides, in_out): (&SecondaryMap<FaceKey, SideCode>, &[(FaceKey, FaceKey)]),
    (operand, kept_vertex): (Operand, &dyn Fn(VertexKey) -> Option<VertexKey>),
    held: (
        &[(FaceKey, FaceKey)],
        &dyn Fn(VertexKey) -> Option<VertexKey>,
    ),
) -> Result<Vec<DiscardRow>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let (body, holder_op, holder) = match operand {
        Operand::A => (&red.a, Operand::B, &red.b),
        Operand::B => (&red.b, Operand::A, &red.a),
    };
    let copy = super::NullCopies::of_operand(&red.null_edges, operand);
    let on_face = |face: FaceKey| -> Result<BTreeSet<VertexKey>, BooleanError> {
        Ok(section_boundary(body, face)?
            .into_iter()
            .map(|he| proven(&body.half_edges, he, EntityId::HalfEdge).start)
            .collect())
    };
    let edges_of = |face: FaceKey| -> Result<BTreeSet<(VertexKey, VertexKey)>, BooleanError> {
        Ok(section_boundary(body, face)?
            .into_iter()
            .map(|he| {
                (
                    proven(&body.half_edges, he, EntityId::HalfEdge).start,
                    body.proven_half_edge_end(he),
                )
            })
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
    let kept_ends = |across: FaceKey, u, w| {
        let (us, ws) = (kept_copies(u, across)?, kept_copies(w, across)?);
        if let (Some(ku), Some(kw)) = (one_kept(&us), one_kept(&ws)) {
            return Ok((ku, kw));
        }
        // Several copies survive on the twin: a vertex several runs of
        // one pierce cut, which the twin passes once per run. The
        // stretch's kept ends are the two copies one edge of the twin
        // joins.
        let joined = twin_of(across).map_or_else(|| Ok(BTreeSet::new()), edges_of)?;
        let ends: BTreeSet<(VertexKey, VertexKey)> = us
            .iter()
            .flat_map(|&(bu, ru)| ws.iter().map(move |&(bw, rw)| ((bu, bw), (ru, rw))))
            .filter(|&((bu, bw), _)| joined.contains(&(bu, bw)) || joined.contains(&(bw, bu)))
            .map(|(_, r)| r)
            .collect();
        match ends.first() {
            Some(&e) if ends.len() == 1 => Ok(e),
            _ => Err(desync(
                "a section vertex's null-edge copies have not exactly one kept end",
            )),
        }
    };
    let kept_across = |f: FaceKey| sides.contains_key(f);
    let held = HeldInto {
        entries: &red.held,
        fragments: held.0,
        holder_op,
        holder,
        to_result: held.1,
        copies: &copy,
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
            if !sides.contains_key(face) {
                out.push(discard_row(
                    body,
                    face,
                    operand,
                    &kept_across,
                    &kept_ends,
                    Some(&held),
                )?);
            }
        }
    }
    Ok(out)
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
                Err(BooleanError::JoinDesync { .. })
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
