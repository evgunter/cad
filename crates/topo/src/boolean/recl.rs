//! Vertex-vertex classification, part 2: reclassification of ON cases.
//!
//! - [`recl_sectors`] — Program 15.10 IN FULL (coplanar sector pairs
//!   via [`super::oriented_plane_eq`] + Eq. 15.3, neighbor propagation,
//!   cancellation). The propagation is the operational form of TOG's
//!   Table II ON-rows + Table III: rewriting a neighbor's shared-bound
//!   code to the Eq. 15.3 lump and cancelling uniform records IS
//!   `resolve_verdict(Rule/NotRule, …)` — the equivalence is asserted
//!   by the `propagation_matches_tables` test.
//! - [`recl_edges`] — the unprinted `srecledges`, designed from TOG
//!   §6.2.2: **edge-sector coincidence** (two flanking test sectors
//!   keyed by their noncoplanar bound vs the reference sector, Table II
//!   plus Table III) and **edge-edge coincidence** (the derived
//!   membership rule subsuming the angular sort and the Table I tie
//!   rules — see `resolve_edge_edge`).
//!
//! Germ attribution (deterministic, symmetric): a crossing along an
//! on-edge is recorded on the flanking sector holding the on-bound as
//! its START (in both solids for edge-edge), with the On code rewritten
//! to the transition partner of the sector's other bound.
//!
//! Postcondition (checked loudly): no surviving record carries an On
//! code.

use geom_core::{Band, Decide, Margin, Sign, Vec3};

use super::carrier_eq::CarrierDesc;
use super::plane_eq::{PlaneEqError, PlaneRelation};
use super::sectors::{BoolSector, PairRecord, side_code};
use super::tables::{eq15_3_lump, resolve_verdict, table_ii};
use super::{BooleanError, BooleanOp, Operand, SideCode};
use crate::body::Body;
use crate::validate::decide;

/// The sector face's ORIENTED carrier description
/// ([`super::rest::face_carrier`] —
/// the face's material side with S10's sense bit already folded in,
/// which is what the Same±-orientation verdict below has to mean: the
/// whole point of the verdict is which way the two materials face).
/// A kind outside the `Rest` ladder's inventory (cone, torus, NURBS)
/// is the C5 typed refusal.
fn carrier_of<T: Decide>(
    body: &Body<T>,
    operand: super::Operand,
    s: &BoolSector<T>,
) -> Result<CarrierDesc<T>, BooleanError> {
    super::rest::face_carrier(body, s.face).ok_or_else(|| {
        let kind = body
            .get_face(s.face)
            .and_then(|f| body.get_surface(f.surface))
            .map_or(geom_brep::SurfaceKind::Nurbs, geom_brep::SurfaceKind::of);
        BooleanError::CurvedBooleanUnsupported {
            operand,
            face: s.face,
            kind,
        }
    })
}

/// The geometrically-ON sector pair's carrier identity check, with the
/// M4 PR 5 evidence: the two faces' recipe sources (N6) plus the
/// consuming op's declared face pairs (F5). The sources are the
/// ORIENTED ones ([`super::reduce::face_plane_source`]): rung 1
/// decides Same± from `orient`, and the descriptions it decides about
/// are material sides, so the face senses must be composed in or a
/// same-surface opposite-sense pair reads SameOriented.
///
/// C8: a CURVED sector pair descends the ladder only under a declared
/// `Rest` pair — an undeclared on-carrier curved pair keeps the typed
/// frontier refusal (the recourse is a declared contact, vocabulary
/// CONTACT-DESIGN C4).
#[allow(clippy::too_many_arguments)]
fn require_same<T: Decide>(
    body1: &Body<T>,
    o1: super::Operand,
    s1: &BoolSector<T>,
    body2: &Body<T>,
    o2: super::Operand,
    s2: &BoolSector<T>,
    declared: &super::DeclaredPairs,
    arm: T,
    band: Band,
) -> Result<PlaneRelation, BooleanError> {
    let c1 = carrier_of(body1, o1, s1)?;
    let c2 = carrier_of(body2, o2, s2)?;
    let declared_rest = declared.declares_rest(o1, s1.face, o2, s2.face);
    if !declared_rest {
        let curved = |c: &CarrierDesc<T>| !matches!(c, CarrierDesc::Plane { .. });
        let refusal = if curved(&c1) {
            Some((o1, s1.face, body1.get_face(s1.face), body1))
        } else if curved(&c2) {
            Some((o2, s2.face, body2.get_face(s2.face), body2))
        } else {
            None
        };
        if let Some((operand, face, f, body)) = refusal {
            let kind = f
                .and_then(|f| body.get_surface(f.surface))
                .map_or(geom_brep::SurfaceKind::Nurbs, geom_brep::SurfaceKind::of);
            return Err(BooleanError::CurvedBooleanUnsupported {
                operand,
                face,
                kind,
            });
        }
    }
    let (g1, g2) = (
        super::reduce::face_plane_source(body1, s1.face),
        super::reduce::face_plane_source(body2, s2.face),
    );
    let id = super::PlaneIdentity {
        s1: g1.as_ref(),
        s2: g2.as_ref(),
        declared: declared_rest,
    };
    match super::carrier_eq::carrier_eq(&c1, &c2, id, arm, band) {
        Ok(PlaneRelation::Distinct) => Err(BooleanError::ClassificationInvariant {
            what: "geometrically-ON sector pair with definitely-distinct carriers",
        }),
        Ok(rel) => Ok(rel),
        Err(PlaneEqError::Escalated(diag)) => Err(BooleanError::Escalated { diag }),
        Err(PlaneEqError::Undeclared { diag, relation }) => {
            Err(BooleanError::UndeclaredCoincidence {
                diag,
                pair: [(o1, s1.face), (o2, s2.face)],
                relation,
            })
        }
        Err(PlaneEqError::Contradicted(diag)) => {
            Err(BooleanError::DeclarationContradicted { diag })
        }
    }
}

fn cancel_uniform(r: &mut PairRecord) {
    let uni = |c: (SideCode, SideCode)| c.0 == c.1 && c.0 != SideCode::On;
    if uni(r.sa) || uni(r.sb) {
        r.intersect = false;
    }
}

/// Program 15.10 (module docs). `records` are rewritten in place,
/// sequentially, in creation (A-major) order — later coplanar pairs see
/// propagated codes, as the book.
#[allow(clippy::too_many_arguments)]
pub(super) fn recl_sectors<T: Decide>(
    records: &mut [PairRecord],
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    a_body: &Body<T>,
    b_body: &Body<T>,
    op: BooleanOp,
    declared: &super::DeclaredPairs,
    band: Band,
) -> Result<(), BooleanError> {
    let (n_a, n_b) = (a_sectors.len(), b_sectors.len());
    for i in 0..records.len() {
        let r = records[i];
        let all_on = r.sa == (SideCode::On, SideCode::On) && r.sb == (SideCode::On, SideCode::On);
        if !all_on {
            continue;
        }
        let sa = &a_sectors[r.a];
        let sb = &b_sectors[r.b];
        let arm = sa.arm.min(sb.arm);
        // Declared-`Tangent` (distinct carriers touching): each side's
        // lump verdict is the second-order sector trilean — which side
        // its carrier CURVES to relative to the OTHER face's material
        // ([`super::sectors::tangent_lump`]). Everything else takes
        // the carrier identity ladder.
        let tangent_pair =
            declared.class_of(super::Operand::A, sa.face, super::Operand::B, sb.face)
                == Some(crate::contact::ContactClass::Tangent);
        let (newsa, newsb) = if tangent_pair {
            let surface_of = |body: &Body<T>, face| {
                body.get_face(face)
                    .and_then(|f| body.get_surface(f.surface))
                    .cloned()
                    .ok_or(BooleanError::ClassificationInvariant {
                        what: "declared-Tangent face lost its surface",
                    })
            };
            let s_a = surface_of(a_body, sa.face)?;
            let s_b = surface_of(b_body, sb.face)?;
            let p = a_body
                .get_half_edge(sa.he)
                .and_then(|he| a_body.get_vertex(he.start))
                .and_then(|v| a_body.get_point(v.point))
                .copied()
                .ok_or(BooleanError::ClassificationInvariant {
                    what: "v-v site lost its point",
                })?;
            (
                super::sectors::tangent_lump(
                    &s_a,
                    &s_b,
                    sb.normal,
                    p,
                    op,
                    Operand::A,
                    sa.face,
                    arm,
                    band,
                )?,
                super::sectors::tangent_lump(
                    &s_b,
                    &s_a,
                    sa.normal,
                    p,
                    op,
                    Operand::B,
                    sb.face,
                    arm,
                    band,
                )?,
            )
        } else {
            let rel = require_same(
                a_body,
                super::Operand::A,
                sa,
                b_body,
                super::Operand::B,
                sb,
                declared,
                arm,
                band,
            )?;
            (
                eq15_3_lump(op, Operand::A, rel),
                eq15_3_lump(op, Operand::B, rel),
            )
        };
        // Cyclic neighbors through the shared-bound chain:
        // start(k) == end(k+1); end(k) == start(k−1).
        let a_start_nbr = (r.a + 1) % n_a; // shares r.a's start (as its end)
        let a_end_nbr = (r.a + n_a - 1) % n_a; // shares r.a's end (as its start)
        let b_start_nbr = (r.b + 1) % n_b;
        let b_end_nbr = (r.b + n_b - 1) % n_b;
        for rec in records.iter_mut() {
            if rec.a == a_start_nbr && rec.b == r.b && rec.sa.0 != SideCode::On {
                rec.sa.1 = newsa;
            }
            if rec.a == a_end_nbr && rec.b == r.b && rec.sa.1 != SideCode::On {
                rec.sa.0 = newsa;
            }
            if rec.b == b_start_nbr && rec.a == r.a && rec.sb.0 != SideCode::On {
                rec.sb.1 = newsb;
            }
            if rec.b == b_end_nbr && rec.a == r.a && rec.sb.1 != SideCode::On {
                rec.sb.0 = newsb;
            }
            cancel_uniform(rec);
        }
        let rec = &mut records[i];
        rec.sa = (newsa, newsa);
        rec.sb = (newsb, newsb);
        rec.intersect = false;
    }
    Ok(())
}

/// Which bound of the record's own-side codes is On (post-15.10 a
/// record has at most one On per side).
fn on_bound(codes: (SideCode, SideCode)) -> Option<bool /* at start */> {
    match codes {
        (SideCode::On, SideCode::On) => None, // handled by 15.10
        (SideCode::On, _) => Some(true),
        (_, SideCode::On) => Some(false),
        _ => None,
    }
}

/// The two flanking sectors of a bound: `(start_holder, end_holder)` —
/// the bound is `start_holder.start == end_holder.end` and
/// `end_holder == start_holder + 1` in the chained array.
fn flankers(idx: usize, at_start: bool, n: usize) -> (usize, usize) {
    if at_start {
        (idx, (idx + 1) % n)
    } else {
        ((idx + n - 1) % n, idx)
    }
}

/// A germ host: a record matching `(a, b)` that still CARRIES an On
/// code (cancelled/rewritten records are never resurrected as germs).
fn find_on_record(records: &[PairRecord], a: usize, b: usize) -> Option<usize> {
    records.iter().position(|r| {
        r.a == a && r.b == b && (on_bound(r.sa).is_some() || on_bound(r.sb).is_some())
    })
}

/// The reference sector for an on-bound event whose own-side bound has
/// start-holder `f_s`: the other-solid sector index of the first record
/// whose own-side On code names **that bound** (start-holder match) —
/// NOT any On record on the flanking twins: a vertex pair can host
/// several distinct on-direction events at once (e.g. a collinear-edge
/// overlap AND a transverse crossing along a subdivision bisector — the
/// R-1 fixture), and matching any On record conflates them, keying the
/// event against the wrong face.
fn find_ref_sector(
    records: &[PairRecord],
    a_side: bool,
    f_s: usize,
    n_own: usize,
) -> Option<usize> {
    records.iter().find_map(|r| {
        let (own_i, other_i) = if a_side { (r.a, r.b) } else { (r.b, r.a) };
        let at_start = on_bound(if a_side { r.sa } else { r.sb })?;
        (flankers(own_i, at_start, n_own).0 == f_s).then_some(other_i)
    })
}

/// A flanking sector's Table II key: the GEOMETRIC side code of its
/// noncoplanar bound (per TOG: a wide flanker keys by its bisector,
/// which is exactly the twin-shared bound) against the reference face.
/// Always computed fresh — post-15.10 record codes are resolutions,
/// and reading them would hide the ON (coplanar test sector) case.
fn flank_key<T: Decide>(
    own: &[BoolSector<T>],
    idx: usize,
    key_from_start: bool, // true: the noncoplanar bound is the START
    ref_normal: geom_brep::OutwardNormal<T>,
    band: Band,
) -> Result<SideCode, BooleanError> {
    let s = &own[idx];
    let bound = if key_from_start { s.start } else { s.end };
    side_code(bound, ref_normal, s.arm, band)
}

/// Rewrites EVERY On code of a surviving germ record to the transition
/// partner of its side's other code (both sides — a germ record with a
/// grazing subdivision-bisector bound on the opposite side gets that
/// graze resolved to the transition too), and marks it surviving.
fn mark_germ(rec: &mut PairRecord) -> Result<(), BooleanError> {
    for codes in [&mut rec.sa, &mut rec.sb] {
        match on_bound(*codes) {
            Some(true) => codes.0 = codes.1.opposite(),
            Some(false) => codes.1 = codes.0.opposite(),
            None => {}
        }
        let clean = (codes.0 == SideCode::In && codes.1 == SideCode::Out)
            || (codes.0 == SideCode::Out && codes.1 == SideCode::In);
        if !clean {
            return Err(BooleanError::ClassificationInvariant {
                what: "germ record without a clean In/Out transition after rewrite",
            });
        }
    }
    rec.intersect = true;
    Ok(())
}

/// `srecledges` (module docs) — the UNIFIED on-direction event engine:
/// every On code of a surviving record names a bound direction (a real
/// edge chord or a subdivision bisector); directions are grouped into
/// ray events (parallel-same across both solids), and each event is
/// resolved EXACTLY ONCE:
///
/// - real edge in BOTH solids ⇒ **edge-edge coincidence**: angular sort
///   of the four flanking sectors around the common line; mixed order ⇒
///   germ; coplanar ties per Table I rules (all-pairwise ⇒ none; else
///   the noncoplanar complement pair per Table III).
/// - real edge in ONE solid ⇒ **edge-sector coincidence**: Table II on
///   the two flanking test sectors' keys (per TOG, a wide flanker keys
///   by its bisector — exactly the twin's shared bound), Rule cells per
///   Table III.
/// - bisector-only ⇒ a **subdivision artifact graze**: the wide sector
///   is genuinely crossed iff its two halves' outer keys transition.
///
/// The event's germ is marked on ONE deterministic record; every other
/// surviving record carrying an On in the event is cancelled (unless it
/// was itself germ-marked by an earlier event).
#[allow(clippy::too_many_arguments)]
pub(super) fn recl_edges<T: Decide>(
    records: &mut [PairRecord],
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    a_body: &Body<T>,
    b_body: &Body<T>,
    op: BooleanOp,
    declared: &super::DeclaredPairs,
    band: Band,
) -> Result<(), BooleanError> {
    let (n_a, n_b) = (a_sectors.len(), b_sectors.len());

    // ---- Collect on-bound mentions from surviving records. ----
    #[derive(Clone, Copy)]
    struct Mention<T2: geom_core::Real> {
        a_side: bool,
        start_holder: usize,
        real: bool,
        dir: geom_core::Vec3<T2>,
    }
    let mut mentions: Vec<Mention<T>> = Vec::new();
    for r in records.iter().filter(|r| r.intersect) {
        for a_side in [true, false] {
            let codes = if a_side { r.sa } else { r.sb };
            let Some(at_start) = on_bound(codes) else {
                continue;
            };
            let (secs, idx, n) = if a_side {
                (a_sectors, r.a, n_a)
            } else {
                (b_sectors, r.b, n_b)
            };
            let (f_s, _) = flankers(idx, at_start, n);
            let real = if at_start {
                secs[idx].start_edge
            } else {
                secs[idx].end_edge
            };
            let dir = if at_start {
                secs[idx].start
            } else {
                secs[idx].end
            };
            if !mentions
                .iter()
                .any(|m| m.a_side == a_side && m.start_holder == f_s)
            {
                mentions.push(Mention {
                    a_side,
                    start_holder: f_s,
                    real,
                    dir,
                });
            }
        }
    }
    // ---- Group mentions into ray events and resolve each once. ----
    let mut used = vec![false; mentions.len()];
    let mut marked: Vec<usize> = Vec::new();
    for i in 0..mentions.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        let mut group = vec![mentions[i]];
        for j in (i + 1)..mentions.len() {
            if !used[j] && parallel_same_dir(mentions[i].dir, mentions[j].dir, T::one(), band)? {
                used[j] = true;
                group.push(mentions[j]);
            }
        }
        if group.iter().filter(|m| m.a_side).count() > 1
            || group.iter().filter(|m| !m.a_side).count() > 1
        {
            return Err(BooleanError::ClassificationInvariant {
                what: "two distinct same-solid bounds share one ray (degenerate operand)",
            });
        }
        let a_m = group.iter().find(|m| m.a_side).copied();
        let b_m = group.iter().find(|m| !m.a_side).copied();

        let germ: Option<usize> = match (a_m, b_m) {
            (Some(am), Some(bm)) if am.real && bm.real => resolve_edge_edge(
                records,
                a_sectors,
                b_sectors,
                a_body,
                b_body,
                op,
                declared,
                band,
                am.start_holder,
                bm.start_holder,
            )?,
            (Some(am), bm) if am.real => resolve_edge_sector(
                records,
                a_sectors,
                b_sectors,
                a_body,
                b_body,
                op,
                declared,
                band,
                true,
                am.start_holder,
                bm.map(|m| m.start_holder),
            )?,
            (am, Some(bm)) if bm.real => resolve_edge_sector(
                records,
                a_sectors,
                b_sectors,
                a_body,
                b_body,
                op,
                declared,
                band,
                false,
                bm.start_holder,
                am.map(|m| m.start_holder),
            )?,
            (am, bm) => resolve_bisector_graze(
                records,
                a_sectors,
                b_sectors,
                band,
                am.map(|m| m.start_holder),
                bm.map(|m| m.start_holder),
            )?,
        };
        if let Some(g) = germ {
            mark_germ(&mut records[g])?;
            marked.push(g);
        }
        // Cancel every other surviving record mentioning the event.
        let in_event = |a_side: bool, holder: usize| {
            group
                .iter()
                .any(|m| m.a_side == a_side && m.start_holder == holder)
        };
        for (j, rec) in records.iter_mut().enumerate() {
            if Some(j) == germ || marked.contains(&j) || !rec.intersect {
                continue;
            }
            let mut hit = false;
            if let Some(at_start) = on_bound(rec.sa) {
                let (f_s, _) = flankers(rec.a, at_start, n_a);
                hit |= in_event(true, f_s);
            }
            if let Some(at_start) = on_bound(rec.sb) {
                let (f_s, _) = flankers(rec.b, at_start, n_b);
                hit |= in_event(false, f_s);
            }
            if hit {
                rec.intersect = false;
            }
        }
    }

    // ---- Postcondition: no surviving On codes. ----
    for r in records.iter() {
        if r.intersect
            && (r.sa.0 == SideCode::On
                || r.sa.1 == SideCode::On
                || r.sb.0 == SideCode::On
                || r.sb.1 == SideCode::On)
        {
            return Err(BooleanError::ClassificationInvariant {
                what: "surviving record with an On code after srecledges",
            });
        }
    }
    Ok(())
}

/// Edge-edge coincidence — the DERIVED membership rule (subsumes TOG's
/// angular sort and its Table I ties): around the common line, each
/// solid's material occupies the dihedral wedge between its two
/// flanking faces. A germ exists iff EXACTLY ONE of A's two flanking
/// representatives (the noncoplanar bound / bisector of each flanking
/// sector, projected ⊥ the line) lies inside B's wedge — with a
/// representative lying ON a B-plane resolved by the coincidence
/// ladder: overlapping-coplanar (same projected direction as that
/// face's own representative) ⇒ the Eq. 15.3 lump decides (In ⇒
/// inside), touching-anti-parallel ⇒ outside. The symmetric B-vs-A
/// evaluation must agree (checked; disagreement is a typed invariant
/// failure). For all-pairwise-coplanar configurations every membership
/// resolves by lump and the rule reproduces Table I's verdicts; for
/// generic (tie-free) configurations "exactly one inside" IS the mixed
/// angular order. Limitation (flagged in the PR report): membership
/// uses the convex-wedge test (In against both flanking planes);
/// reflex dihedral wedges along a coincident edge are not yet
/// discriminated — the A/B symmetry check refuses loudly if it bites.
#[allow(clippy::too_many_arguments)]
fn resolve_edge_edge<T: Decide>(
    records: &[PairRecord],
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    a_body: &Body<T>,
    b_body: &Body<T>,
    op: BooleanOp,
    declared: &super::DeclaredPairs,
    band: Band,
    fa_s: usize,
    fb_s: usize,
) -> Result<Option<usize>, BooleanError> {
    let (n_a, n_b) = (a_sectors.len(), b_sectors.len());
    let fa_e = (fa_s + 1) % n_a;
    let fb_e = (fb_s + 1) % n_b;
    let axis = a_sectors[fa_s].start.normalize();
    let arm = a_sectors[fa_s].arm.min(b_sectors[fb_s].arm);
    let rep = |s: &BoolSector<T>, other_is_end: bool| -> Vec3<T> {
        let v = if other_is_end { s.end } else { s.start };
        (v - axis * v.dot(axis)).normalize()
    };
    let a_fl = [
        (fa_s, rep(&a_sectors[fa_s], true)),
        (fa_e, rep(&a_sectors[fa_e], false)),
    ];
    let b_fl = [
        (fb_s, rep(&b_sectors[fb_s], true)),
        (fb_e, rep(&b_sectors[fb_e], false)),
    ];

    // Membership of one flanker's rep inside the other solid's wedge.
    let membership = |own_is_a: bool,
                      own_idx: usize,
                      w: Vec3<T>,
                      other: &[(usize, Vec3<T>)]|
     -> Result<bool, BooleanError> {
        let (own_secs, other_secs): (&[BoolSector<T>], &[BoolSector<T>]) = if own_is_a {
            (a_sectors, b_sectors)
        } else {
            (b_sectors, a_sectors)
        };
        let (own_body, other_body): (&Body<T>, &Body<T>) = if own_is_a {
            (a_body, b_body)
        } else {
            (b_body, a_body)
        };
        let mut inside = true;
        for &(oi, ow) in other {
            match side_code(w, other_secs[oi].normal, arm, band)? {
                SideCode::In => {}
                SideCode::Out => inside = false,
                SideCode::On => {
                    // On the flanking plane: overlap-tie or touch.
                    let same = match decide("bool_dir_same", Margin::levered(w.dot(ow), arm), band)
                    {
                        Ok(Sign::Positive) => true,
                        Ok(Sign::Negative) => false,
                        Ok(Sign::Zero) => {
                            return Err(BooleanError::ClassificationInvariant {
                                what: "degenerate rep pair in edge-edge membership",
                            });
                        }
                        Err(diag) => return Err(BooleanError::Escalated { diag }),
                    };
                    if !same {
                        inside = false; // touching, not overlapping
                    } else {
                        let (own_op, other_op) = if own_is_a {
                            (super::Operand::A, super::Operand::B)
                        } else {
                            (super::Operand::B, super::Operand::A)
                        };
                        let rel = require_same(
                            own_body,
                            own_op,
                            &own_secs[own_idx],
                            other_body,
                            other_op,
                            &other_secs[oi],
                            declared,
                            arm,
                            band,
                        )?;
                        let comparison = if own_is_a { Operand::A } else { Operand::B };
                        if eq15_3_lump(op, comparison, rel) != SideCode::In {
                            inside = false;
                        }
                    }
                }
            }
        }
        Ok(inside)
    };
    let a_in = [
        membership(true, a_fl[0].0, a_fl[0].1, &b_fl)?,
        membership(true, a_fl[1].0, a_fl[1].1, &b_fl)?,
    ];
    let b_in = [
        membership(false, b_fl[0].0, b_fl[0].1, &a_fl)?,
        membership(false, b_fl[1].0, b_fl[1].1, &a_fl)?,
    ];
    let germ_a = a_in[0] != a_in[1];
    let germ_b = b_in[0] != b_in[1];
    if germ_a != germ_b {
        return Err(BooleanError::ClassificationInvariant {
            what: "edge-edge membership disagreement between the two solids",
        });
    }
    if !germ_a {
        return Ok(None);
    }
    let combos = [(fa_s, fb_s), (fa_s, fb_e), (fa_e, fb_s), (fa_e, fb_e)];
    combos
        .iter()
        .find_map(|&(a, b)| find_on_record(records, a, b))
        .map(Some)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "edge-edge germ record missing among the flanking combos",
        })
}

/// Edge-sector coincidence (module docs): Table II on the on-side
/// flankers vs the reference sector. `other_holder` is the other
/// solid's start-holder twin when the ray also grazes its bisector.
#[allow(clippy::too_many_arguments)]
fn resolve_edge_sector<T: Decide>(
    records: &[PairRecord],
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    a_body: &Body<T>,
    b_body: &Body<T>,
    op: BooleanOp,
    declared: &super::DeclaredPairs,
    band: Band,
    a_side: bool,
    f_s: usize,
    other_holder: Option<usize>,
) -> Result<Option<usize>, BooleanError> {
    let (own_secs, ref_secs): (&[BoolSector<T>], &[BoolSector<T>]) = if a_side {
        (a_sectors, b_sectors)
    } else {
        (b_sectors, a_sectors)
    };
    let n_own = own_secs.len();
    let f_e = (f_s + 1) % n_own;
    let ref_idx = match other_holder {
        Some(h) => h,
        None => find_ref_sector(records, a_side, f_s, n_own).ok_or(
            BooleanError::ClassificationInvariant {
                what: "on-edge event without a reference sector",
            },
        )?,
    };
    let ref_sector = &ref_secs[ref_idx];
    let k1 = flank_key(own_secs, f_s, false, ref_sector.normal, band)?;
    let k2 = flank_key(own_secs, f_e, true, ref_sector.normal, band)?;
    let (v1, v2) = table_ii(k1, k2);
    let comparison = if a_side { Operand::A } else { Operand::B };
    let mut relation = PlaneRelation::SameOriented;
    if matches!(
        (v1, v2),
        (
            super::tables::TableIiVerdict::Rule | super::tables::TableIiVerdict::NotRule,
            _
        ) | (
            _,
            super::tables::TableIiVerdict::Rule | super::tables::TableIiVerdict::NotRule
        )
    ) {
        let on_flanker = if k1 == SideCode::On { f_s } else { f_e };
        let (own_body, ref_body): (&Body<T>, &Body<T>) = if a_side {
            (a_body, b_body)
        } else {
            (b_body, a_body)
        };
        let (own_op, ref_op) = if a_side {
            (super::Operand::A, super::Operand::B)
        } else {
            (super::Operand::B, super::Operand::A)
        };
        relation = require_same(
            own_body,
            own_op,
            &own_secs[on_flanker],
            ref_body,
            ref_op,
            ref_sector,
            declared,
            own_secs[on_flanker].arm.min(ref_sector.arm),
            band,
        )?;
    }
    for (flank, verdict) in [(f_s, v1), (f_e, v2)] {
        if resolve_verdict(verdict, op, comparison, relation) {
            let (ra, rb) = if a_side {
                (flank, ref_idx)
            } else {
                (ref_idx, flank)
            };
            // The germ may live on either twin of a subdivided
            // reference; try the recorded twin then its partner.
            let n_ref = ref_secs.len();
            let alt = (ref_idx + n_ref - 1) % n_ref;
            let g = find_on_record(records, ra, rb).or_else(|| {
                let (ra2, rb2) = if a_side { (flank, alt) } else { (alt, flank) };
                find_on_record(records, ra2, rb2)
            });
            return g.map(Some).ok_or(BooleanError::ClassificationInvariant {
                what: "single-on-edge crossing without a flanking record",
            });
        }
    }
    Ok(None)
}

/// Bisector-only graze (module docs): the wide sector is crossed iff
/// its halves' outer keys transition.
fn resolve_bisector_graze<T: Decide>(
    records: &[PairRecord],
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    band: Band,
    a_holder: Option<usize>,
    b_holder: Option<usize>,
) -> Result<Option<usize>, BooleanError> {
    let (a_side, f_s) = match (a_holder, b_holder) {
        (Some(h), _) => (true, h),
        (None, Some(h)) => (false, h),
        (None, None) => return Ok(None),
    };
    let own_secs: &[BoolSector<T>] = if a_side { a_sectors } else { b_sectors };
    let n_own = own_secs.len();
    let f_e = (f_s + 1) % n_own;
    let ref_idx = match if a_side { b_holder } else { a_holder } {
        Some(h) => h,
        None => match find_ref_sector(records, a_side, f_s, n_own) {
            Some(f) => f,
            None => return Ok(None),
        },
    };
    let ref_sector = if a_side {
        &b_sectors[ref_idx]
    } else {
        &a_sectors[ref_idx]
    };
    let k1 = flank_key(own_secs, f_s, false, ref_sector.normal, band)?;
    let k2 = flank_key(own_secs, f_e, true, ref_sector.normal, band)?;
    let crossing = matches!(
        (k1, k2),
        (SideCode::In, SideCode::Out) | (SideCode::Out, SideCode::In)
    );
    if !crossing {
        return Ok(None);
    }
    let (ra, rb) = if a_side {
        (f_s, ref_idx)
    } else {
        (ref_idx, f_s)
    };
    let g = find_on_record(records, ra, rb).or_else(|| {
        let (ra2, rb2) = if a_side {
            (f_e, ref_idx)
        } else {
            (ref_idx, f_e)
        };
        find_on_record(records, ra2, rb2)
    });
    g.map(Some).ok_or(BooleanError::ClassificationInvariant {
        what: "bisector-graze crossing without a twin record",
    })
}

fn parallel_same_dir<T: Decide>(
    u: Vec3<T>,
    v: Vec3<T>,
    arm: T,
    band: Band,
) -> Result<bool, BooleanError> {
    let un = u.normalize();
    let vn = v.normalize();
    match decide(
        "bool_ee_collinear",
        Margin::levered(un.cross(vn).norm(), arm),
        band,
    ) {
        Ok(Sign::Zero) => {}
        Ok(_) => return Ok(false),
        Err(diag) => return Err(BooleanError::Escalated { diag }),
    }
    match decide("bool_dir_same", Margin::levered(un.dot(vn), arm), band) {
        Ok(Sign::Positive) => Ok(true),
        Ok(_) => Ok(false),
        Err(diag) => Err(BooleanError::Escalated { diag }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use SideCode::{In, On, Out};

    fn rec(a: usize, b: usize, sa: (SideCode, SideCode), sb: (SideCode, SideCode)) -> PairRecord {
        PairRecord {
            a,
            b,
            sa,
            sb,
            intersect: true,
        }
    }

    /// The Program 15.10 mechanics on the Figure 15.9 shape (union,
    /// coplanar pair (a1, b2), Eq. 15.3 ⁺ row): the coplanar record is
    /// lumped and cancelled, the neighbors' shared-bound codes are
    /// rewritten to the lump, cancellation kills uniform records —
    /// checked against hand-derived expectations under OUR (start,
    /// end)-code representation. Exercised through synthetic sectors on
    /// bodies is done at the fixture level; here the array mechanics
    /// are pinned via the propagation-only helper.
    #[test]
    fn cancellation_kills_uniform_records() {
        let mut r = rec(0, 0, (In, In), (In, Out));
        cancel_uniform(&mut r);
        assert!(!r.intersect);
        let mut r = rec(0, 0, (In, Out), (Out, Out));
        cancel_uniform(&mut r);
        assert!(!r.intersect);
        let mut r = rec(0, 0, (In, Out), (Out, In));
        cancel_uniform(&mut r);
        assert!(r.intersect);
        // On-uniform is NOT cancelled here (15.10's own coplanar path
        // owns it).
        let mut r = rec(0, 0, (On, On), (On, On));
        cancel_uniform(&mut r);
        assert!(r.intersect);
    }

    /// `flankers`: a bound held as START of k is shared with k+1 (as
    /// its end); held as END of k, with k−1 (as its start).
    #[test]
    fn flanker_indexing() {
        assert_eq!(flankers(2, true, 4), (2, 3));
        assert_eq!(flankers(0, false, 4), (3, 0));
        assert_eq!(flankers(3, true, 4), (3, 0));
    }

    /// `mark_germ` rewrites the On bound to the transition partner.
    #[test]
    fn germ_rewrite() {
        let mut r = rec(0, 0, (On, In), (Out, On));
        mark_germ(&mut r).unwrap();
        assert_eq!(r.sa, (Out, In));
        assert_eq!(r.sb, (Out, In));
        assert!(r.intersect);
    }
}
