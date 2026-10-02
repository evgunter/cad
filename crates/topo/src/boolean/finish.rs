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
use super::{BooleanError, BooleanOp, BooleanReduction, Operand, SideCode};
use crate::body::Body;
use crate::entity::{FaceKey, ShellKey, SolidKey, VertexKey};
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
    /// surviving end → B-side surviving end), from the pair records.
    pub vertex_map: SecondaryMap<VertexKey, VertexKey>,
    /// The B-side graft bridge (contact-record remapping).
    pub graft: GraftMap,
    /// The faces the selection discarded (`BooleanNaming::discards`).
    pub discards: Vec<DiscardRow>,
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
    let a_kept = carve(&red.a, a_solid, &a_kept_shells)
        .map_err(|_| desync("carving the kept A component failed"))?;
    let mut b_kept = carve(&red.b, b_solid, &b_kept_shells)
        .map_err(|_| desync("carving the kept B component failed"))?;

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
    let mut vertex_map: SecondaryMap<VertexKey, VertexKey> = SecondaryMap::new();
    for pair in &red.null_pairs {
        let aa = a_attr
            .get(pair.a_edge)
            .ok_or_else(|| desync("pair A edge without attribute"))?;
        let ba = b_attr
            .get(pair.b_edge)
            .ok_or_else(|| desync("pair B edge without attribute"))?;
        let a_survivor = match (
            body.get_vertex(aa.below_end).is_some(),
            body.get_vertex(aa.above_end).is_some(),
        ) {
            (true, false) => aa.below_end,
            (false, true) => aa.above_end,
            _ => return Err(desync("pair A edge has not exactly one surviving end")),
        };
        let b_below = graft.vertices.get(ba.below_end).copied();
        let b_above = graft.vertices.get(ba.above_end).copied();
        let b_survivor = match (b_below, b_above) {
            (Some(v), None) => v,
            (None, Some(v)) => v,
            _ => return Err(desync("pair B edge has not exactly one surviving end")),
        };
        if let Some(&existing) = vertex_map.get(a_survivor) {
            if existing != b_survivor {
                return Err(desync("conflicting seam vertex correspondence"));
            }
        } else {
            vertex_map.insert(a_survivor, b_survivor);
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
    })
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
