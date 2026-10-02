//! `setopfinish` (ch. 15 §15.8, Program 15.15 re-derived): promote
//! each completed section-polygon pair into IN/OUT section faces in
//! BOTH solids, distribute components, select per **Eq. 15.1**, carve
//! the kept components, weld each kept pinch into one vertex
//! (`weld_pinches`), `revert` the B side for ∖, and drive the
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
use super::shell_witness::{debug_assert_contacts_undecisive, shell_side};
use super::zip::{Joint, SeamCorrespondence, fuse_by_joint, survivor};
use super::{BooleanError, BooleanOp, BooleanReduction, Operand, SideCode, one_vertex};
use crate::body::Body;
use crate::entity::{FaceKey, LoopBoundary, ShellKey, SolidKey, VertexKey};
use crate::euler::FaceSurface;
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
    pub weld_merges_a: Vec<(VertexKey, VertexKey)>,
    /// The B-side pinch welds' vertex fusions, B-clone keys: they ran
    /// before the graft, so a dead key has no result key.
    pub weld_merges_b: Vec<(VertexKey, VertexKey)>,
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
/// typed error), else the uncut-shell witness against the pristine
/// other operand.
#[allow(clippy::too_many_arguments)]
fn classify_shell<T: Decide>(
    body: &Body<T>,
    shell: ShellKey,
    side_of: &SecondaryMap<FaceKey, SideCode>,
    other: &Body<T>,
    operand: Operand,
    band: Band,
    tol: Tol,
) -> Result<SideCode, BooleanError> {
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
        return Ok(s);
    }
    shell_side(body, shell, other, operand, band, tol)
}

/// Distributes, classifies, and selects one solid's kept shells;
/// returns (kept shells, all shells' sides for the invariant check).
#[allow(clippy::too_many_arguments)]
fn select_solid<T: Decide>(
    body: &mut Body<T>,
    solid: SolidKey,
    side_of: &SecondaryMap<FaceKey, SideCode>,
    other: &Body<T>,
    operand: Operand,
    keep: SideCode,
    band: Band,
    tol: Tol,
) -> Result<Vec<ShellKey>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let shells: Vec<ShellKey> = body
        .shells_of_solid(solid)
        .ok_or_else(|| desync("operand solid no longer resolves"))?
        .to_vec();
    let mut all = Vec::new();
    for shell in shells {
        all.extend(body.movefac(shell)?);
    }
    let mut kept = Vec::new();
    for shell in all {
        if classify_shell(body, shell, side_of, other, operand, band, tol)? == keep {
            kept.push(shell);
        }
    }
    Ok(kept)
}

/// `setopfinish` (module docs): promotion → distribution → Eq. 15.1
/// selection → carve → ∖-revert → the combine door. Consumes the
/// joined reduction; the original operands are read-only witnesses for
/// uncut-component containment.
pub(super) fn setopfinish<T: Decide>(
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
    let a_kept_shells = select_solid(
        &mut red.a,
        a_solid,
        &a_sides,
        b_pristine,
        Operand::A,
        kept_side(op, Operand::A),
        band,
        tol,
    )?;
    let b_kept_shells = select_solid(
        &mut red.b,
        b_solid,
        &b_sides,
        a_pristine,
        Operand::B,
        kept_side(op, Operand::B),
        band,
        tol,
    )?;
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
        // A welded pinch lies on a seam once per pierce it fused, with
        // B's vertex of each.
        if bs.len() > 1 && !a_welds.merges.iter().any(|&(_, k)| k == a_survivor) {
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
    let mut discards = discarded(
        &red,
        a_solid,
        &a_kept_shells,
        &a_sides,
        (Operand::A, &a_kept),
        (&connected.a_fragments, &b_kept),
    )?;
    discards.extend(discarded(
        &red,
        b_solid,
        &b_kept_shells,
        &b_sides,
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
    })
}

/// The pinch welds of one kept side: each fusion `(dead, kept)` and the
/// face each one divided.
#[derive(Default)]
struct Welds {
    merges: Vec<(VertexKey, VertexKey)>,
    fragments: Vec<(FaceKey, FaceKey)>,
}

impl Welds {
    /// The vertex `v` survives as.
    fn kept(&self, v: VertexKey) -> VertexKey {
        survivor(&self.merges, v)
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
/// collapsed: across one loop it divides the fragment, two regions
/// meeting at the vertex; across two loops (holes touching at a corner)
/// it joins them into one.
///
/// The site is read from lineage: the pierced face's fragments
/// (`lineage`, `(new face, divided-from face)` rows), section faces
/// (`sections`) aside. Pierces that survive on different fragments, or
/// meet only on a section face, stay apart, as the contact's own
/// vertices do.
fn weld_pinches<T: Decide>(
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
    let desync = |what| BooleanError::JoinDesync { what };
    // A pierce survives as whichever of its null-edge copies the kept
    // side holds.
    let mut copies: BTreeMap<VertexKey, BTreeSet<VertexKey>> = BTreeMap::new();
    for r in red.null_edges.iter().filter(|r| r.operand == operand) {
        let (u, w) = (r.attr.below_end, r.attr.above_end);
        copies.entry(u).or_default().insert(w);
        copies.entry(w).or_default().insert(u);
    }
    let copies_of = |v: VertexKey| -> BTreeSet<VertexKey> {
        let mut seen = BTreeSet::from([v]);
        let mut todo = vec![v];
        while let Some(x) = todo.pop() {
            for &y in copies.get(&x).into_iter().flatten() {
                if seen.insert(y) {
                    todo.push(y);
                }
            }
        }
        seen
    };
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
                let point = |v| {
                    crate::readback::vertex_point_ref(body, v)
                        .map_err(|_| desync("a kept pierce vertex has no point"))
                };
                let pu = point(u)?;
                // No row reaches the escalation: two pierces a band
                // apart need the piercing operand's two edges a band
                // apart, and the join escalates that waist at
                // `bool_join_nearest` before the weld runs; a narrower
                // one the profile insert refuses at the operand's build.
                // An operand built elsewhere can still bring them here.
                if !one_vertex(pu, point(w)?, band).map_err(|diag| BooleanError::Escalated {
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
                let ((dead, kept), made) = fuse_by_joint(body, joint, pu, desync, tol)?;
                if let Some(made) = made {
                    welds.fragments.push((made, face));
                }
                if ![[u, w], [w, u]].contains(&[dead, kept])
                    || body.get_vertex(dead).is_some()
                    || body.get_vertex(kept).is_none()
                {
                    return Err(desync("a pinch weld did not fuse its pair"));
                }
                welds.merges.push((dead, kept));
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
/// them: a chord when one loop holds both, else across their two loops,
/// into the face's outer loop when it is one of them. `None` when no
/// such face holds both.
fn pinch_site<T: Decide>(
    body: &Body<T>,
    u: VertexKey,
    w: VertexKey,
    allowed: impl Fn(FaceKey) -> bool,
) -> Result<Option<(FaceKey, Joint)>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let faces = body
        .faces_of_vertex(u)
        .ok_or_else(|| desync("a pierce vertex no longer resolves"))?;
    let mut site = None;
    for face in faces.into_iter().filter(|&f| allowed(f)) {
        let f = body
            .get_face(face)
            .ok_or_else(|| desync("a pierce vertex's face no longer resolves"))?;
        let mut hus = Vec::new();
        let mut hws = Vec::new();
        for &l in core::iter::once(&f.outer).chain(&f.rings) {
            let first = match body
                .get_loop(l)
                .ok_or_else(|| desync("a face's loop no longer resolves"))?
                .boundary
            {
                LoopBoundary::Cycle { first } => first,
                LoopBoundary::Empty { vertex } if vertex == u || vertex == w => {
                    return Err(desync("a kept pierce vertex stands alone on its face"));
                }
                LoopBoundary::Empty { .. } => continue,
            };
            for he in body
                .loop_cycle(first)
                .ok_or_else(|| desync("a face's loop is not walkable"))?
            {
                match body.get_half_edge(he).map(|h| h.start) {
                    Some(v) if v == u => hus.push((l, he)),
                    Some(v) if v == w => hws.push((l, he)),
                    _ => {}
                }
            }
        }
        let here = match (hus.as_slice(), hws.as_slice()) {
            (_, []) => continue,
            (&[(lu, hu)], &[(lw, hw)]) if lu == lw => Joint::Chord { he1: hu, he2: hw },
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

/// The discarded faces of one operand solid (`boolean::discard`): every
/// face of a shell the selection dropped, the section faces aside. A
/// stretch it bordered a kept face along runs along a section face; the
/// kept side's copy of each end is the other end of one of that end's
/// null edges — the one end among them that survived into the result,
/// which `kept_vertex` reads in result keys, as the seam vertex map
/// picks its survivor. `held` is this operand's chord-split rows and the
/// other operand's vertices in result keys, for the held stretches
/// (`DiscardRow::held`).
#[allow(clippy::type_complexity)]
fn discarded<T: Decide>(
    red: &BooleanReduction<T>,
    solid: SolidKey,
    kept: &[ShellKey],
    sides: &SecondaryMap<FaceKey, SideCode>,
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
    let mut copy: BTreeMap<VertexKey, BTreeSet<VertexKey>> = BTreeMap::new();
    for r in red.null_edges.iter().filter(|r| r.operand == operand) {
        copy.entry(r.attr.below_end)
            .or_default()
            .insert(r.attr.above_end);
        copy.entry(r.attr.above_end)
            .or_default()
            .insert(r.attr.below_end);
    }
    let kept_end = |v: VertexKey| -> Result<VertexKey, BooleanError> {
        let survivors: BTreeSet<VertexKey> = copy
            .get(&v)
            .ok_or_else(|| desync("a section vertex has no null-edge copy"))?
            .iter()
            .filter_map(|&k| kept_vertex(k))
            .collect();
        match survivors.first() {
            Some(&k) if survivors.len() == 1 => Ok(k),
            _ => Err(desync(
                "a section vertex's null-edge copies have not exactly one kept end",
            )),
        }
    };
    let kept_ends = |u, w| Ok((kept_end(u)?, kept_end(w)?));
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
