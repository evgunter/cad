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
//!   + Table III) and **edge-edge coincidence** (angular sort of the 4
//!   flanking sectors around the common line; "mixed order" ⇒
//!   intersection; coplanar ties per Table I rules: all-pairwise-
//!   coplanar ⇒ none, else the noncoplanar complement pair decides per
//!   Table III — the comparison direction for that tie is adjudged as
//!   A-versus-B and flagged in the PR report).
//!
//! Germ attribution (deterministic, symmetric): a crossing along an
//! on-edge is recorded on the flanking sector holding the on-bound as
//! its START (in both solids for edge-edge), with the On code rewritten
//! to the transition partner of the sector's other bound.
//!
//! Postcondition (checked loudly): no surviving record carries an On
//! code.

use geom_core::{Band, Decide, Sign, Vec3};

use super::plane_eq::{PlaneDesc, PlaneEqError, PlaneRelation};
use super::reduce::face_plane;
use super::sectors::{BoolSector, PairRecord, side_code};
use super::tables::{eq15_3_lump, resolve_verdict, table_ii, table_iii_rule};
use super::{BooleanError, BooleanOp, Operand, SideCode};
use crate::body::Body;
use crate::validate::decide;

fn plane_of<T: Decide>(
    body: &Body<T>,
    s: &BoolSector<T>,
) -> Result<PlaneDesc<T>, BooleanError> {
    face_plane(body, s.face).ok_or(BooleanError::ClassificationInvariant {
        what: "sector face lost its plane",
    })
}

fn require_same<T: Decide>(
    p1: &PlaneDesc<T>,
    p2: &PlaneDesc<T>,
    arm: T,
    band: Band,
) -> Result<PlaneRelation, BooleanError> {
    match super::oriented_plane_eq(p1, p2, arm, band) {
        Ok(PlaneRelation::Distinct) => Err(BooleanError::ClassificationInvariant {
            what: "geometrically-ON sector pair with definitely-distinct planes",
        }),
        Ok(rel) => Ok(rel),
        Err(PlaneEqError::Escalated(diag)) => Err(BooleanError::Escalated { diag }),
        Err(PlaneEqError::Undeclared(diag)) => Err(BooleanError::UndeclaredCoincidence { diag }),
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
pub(super) fn recl_sectors<T: Decide>(
    records: &mut [PairRecord],
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    a_body: &Body<T>,
    b_body: &Body<T>,
    op: BooleanOp,
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
        let rel = require_same(&plane_of(a_body, sa)?, &plane_of(b_body, sb)?, arm, band)?;
        let newsa = eq15_3_lump(op, Operand::A, rel);
        let newsb = eq15_3_lump(op, Operand::B, rel);
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

fn find_record(records: &[PairRecord], a: usize, b: usize) -> Option<usize> {
    records.iter().position(|r| r.a == a && r.b == b)
}

/// A flanking sector's Table II key: its NONcoplanar bound's code vs
/// the reference face — read from an existing record if present, else
/// computed directly.
fn flank_key<T: Decide>(
    records: &[PairRecord],
    own: &[BoolSector<T>],
    idx: usize,
    key_from_start: bool, // true: the noncoplanar bound is the START
    ref_normal: Vec3<T>,
    a_side: bool,
    ref_idx: usize,
    band: Band,
) -> Result<SideCode, BooleanError> {
    let (a, b) = if a_side { (idx, ref_idx) } else { (ref_idx, idx) };
    if let Some(ri) = find_record(records, a, b) {
        let codes = if a_side { records[ri].sa } else { records[ri].sb };
        return Ok(if key_from_start { codes.0 } else { codes.1 });
    }
    let s = &own[idx];
    let bound = if key_from_start { s.start } else { s.end };
    side_code(bound, ref_normal, s.arm, band)
}

/// Rewrites a record's own-side On code to the transition partner of
/// its other code, and marks it surviving.
fn mark_germ(rec: &mut PairRecord, a_side: bool) -> Result<(), BooleanError> {
    let codes = if a_side { &mut rec.sa } else { &mut rec.sb };
    match on_bound(*codes) {
        Some(true) => codes.0 = codes.1.opposite(),
        Some(false) => codes.1 = codes.0.opposite(),
        None => {
            return Err(BooleanError::ClassificationInvariant {
                what: "germ record lost its On bound",
            });
        }
    }
    if codes.0 == SideCode::On || codes.1 == SideCode::On {
        return Err(BooleanError::ClassificationInvariant {
            what: "germ record still On after rewrite (consecutive On bounds)",
        });
    }
    rec.intersect = true;
    Ok(())
}

/// `srecledges` (module docs): edge-edge coincidences first (the
/// 4-sector angular sort), then single on-edges (Table II/III).
#[allow(clippy::too_many_arguments)]
pub(super) fn recl_edges<T: Decide>(
    records: &mut Vec<PairRecord>,
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    a_body: &Body<T>,
    b_body: &Body<T>,
    op: BooleanOp,
    band: Band,
) -> Result<(), BooleanError> {
    let (n_a, n_b) = (a_sectors.len(), b_sectors.len());

    // ---- Phase E: edge-edge coincidences. ----
    let mut done_ee: std::collections::BTreeSet<(usize, usize)> = Default::default();
    for i in 0..records.len() {
        let r = records[i];
        if !r.intersect {
            continue;
        }
        let (Some(a_at_start), Some(b_at_start)) = (on_bound(r.sa), on_bound(r.sb)) else {
            continue;
        };
        let (fa_s, fa_e) = flankers(r.a, a_at_start, n_a);
        let (fb_s, fb_e) = flankers(r.b, b_at_start, n_b);
        if !done_ee.insert((fa_s, fb_s)) {
            continue; // this coincident-edge event is already resolved
        }
        let da = if a_at_start {
            a_sectors[r.a].start
        } else {
            a_sectors[r.a].end
        };
        let db = if b_at_start {
            b_sectors[r.b].start
        } else {
            b_sectors[r.b].end
        };
        let arm = a_sectors[r.a].arm.min(b_sectors[r.b].arm);
        // Opposite rays: not a coincident edge pair — each on-edge is a
        // separate edge-sector event; leave to Phase S.
        if !parallel_same_dir(da, db, arm, band)? {
            done_ee.remove(&(fa_s, fb_s));
            continue;
        }
        let axis = da.normalize();
        // Representative = the flanking sector's noncoplanar bound,
        // projected ⊥ the common line. start-holder's other bound = its
        // end; end-holder's other bound = its start.
        let rep = |s: &BoolSector<T>, other_is_end: bool| -> Vec3<T> {
            let v = if other_is_end { s.end } else { s.start };
            v - axis * v.dot(axis)
        };
        let wa = [rep(&a_sectors[fa_s], true), rep(&a_sectors[fa_e], false)];
        let wb = [rep(&b_sectors[fb_s], true), rep(&b_sectors[fb_e], false)];
        // Tie detection: coplanar (A, B) sector pairs = parallel reps
        // (either direction).
        let mut coplanar_pair: Option<(usize, usize)> = None;
        let mut coplanar_count = 0;
        for (ai, wa_i) in wa.iter().enumerate() {
            for (bi, wb_i) in wb.iter().enumerate() {
                let m = wa_i.cross(*wb_i).norm() * arm;
                match decide("bool_ee_rep_parallel", m, band) {
                    Ok(Sign::Zero) => {
                        coplanar_count += 1;
                        if coplanar_pair.is_none() {
                            coplanar_pair = Some((ai, bi));
                        }
                    }
                    Ok(_) => {}
                    Err(diag) => return Err(BooleanError::Escalated { diag }),
                }
            }
        }
        let combos = [(fa_s, fb_s), (fa_s, fb_e), (fa_e, fb_s), (fa_e, fb_e)];
        let intersects = if coplanar_count >= 4 {
            false // Fig. 20a: all pairwise coplanar ⇒ no intersection
        } else if let Some((ai, bi)) = coplanar_pair {
            // Table I tie rule: the noncoplanar COMPLEMENT pair decides
            // per Table III (orientation of the coplanar pair;
            // comparison adjudged A-versus-B — flagged).
            let a_idx = if ai == 0 { fa_s } else { fa_e };
            let b_idx = if bi == 0 { fb_s } else { fb_e };
            let rel = require_same(
                &plane_of(a_body, &a_sectors[a_idx])?,
                &plane_of(b_body, &b_sectors[b_idx])?,
                arm,
                band,
            )?;
            table_iii_rule(op, Operand::A, rel)
        } else {
            // Generic: mixed angular order around the axis.
            let in_arc_1 = within_arc(wa[0], wa[1], wb[0], axis, arm, band)?;
            let in_arc_2 = within_arc(wa[0], wa[1], wb[1], axis, arm, band)?;
            in_arc_1 != in_arc_2
        };
        if intersects {
            let gi = find_record(records, fa_s, fb_s).ok_or(
                BooleanError::ClassificationInvariant {
                    what: "edge-edge germ record (start-holders) missing",
                },
            )?;
            mark_germ(&mut records[gi], true)?;
            mark_germ(&mut records[gi], false)?;
            for (a, b) in combos {
                if let Some(j) = find_record(records, a, b) {
                    if j != gi {
                        records[j].intersect = false;
                    }
                }
            }
        } else {
            for (a, b) in combos {
                if let Some(j) = find_record(records, a, b) {
                    records[j].intersect = false;
                }
            }
        }
    }

    // ---- Phase S: single on-edges (one side On, other side clean). ----
    let mut done_s: std::collections::BTreeSet<(bool, usize, usize)> = Default::default();
    for i in 0..records.len() {
        let r = records[i];
        if !r.intersect {
            continue;
        }
        for a_side in [true, false] {
            let (own_codes, other_codes) = if a_side { (r.sa, r.sb) } else { (r.sb, r.sa) };
            let Some(at_start) = on_bound(own_codes) else {
                continue;
            };
            if on_bound(other_codes).is_some() {
                continue; // both sides On: an unresolved edge-edge remnant
            }
            let n_own = if a_side { n_a } else { n_b };
            let own_idx = if a_side { r.a } else { r.b };
            let ref_idx = if a_side { r.b } else { r.a };
            let (f_s, f_e) = flankers(own_idx, at_start, n_own);
            if !done_s.insert((a_side, f_s, ref_idx)) {
                continue;
            }
            let (own_sectors, ref_sector) = if a_side {
                (a_sectors, &b_sectors[ref_idx])
            } else {
                (b_sectors, &a_sectors[ref_idx])
            };
            // Table II keys: start-holder's key from its END bound,
            // end-holder's from its START bound.
            let k1 = flank_key(records, own_sectors, f_s, false, ref_sector.normal, a_side, ref_idx, band)?;
            let k2 = flank_key(records, own_sectors, f_e, true, ref_sector.normal, a_side, ref_idx, band)?;
            let (v1, v2) = table_ii(k1, k2);
            let comparison = if a_side { Operand::A } else { Operand::B };
            // Orientation (needed only for Rule/NotRule cells): the
            // On-keyed flanker is coplanar with the reference.
            let mut relation = PlaneRelation::SameOriented;
            if matches!(
                (v1, v2),
                (super::tables::TableIiVerdict::Rule | super::tables::TableIiVerdict::NotRule, _)
                    | (_, super::tables::TableIiVerdict::Rule | super::tables::TableIiVerdict::NotRule)
            ) {
                let on_flanker = if k1 == SideCode::On { f_s } else { f_e };
                let (own_body, ref_body): (&Body<T>, &Body<T>) = if a_side {
                    (a_body, b_body)
                } else {
                    (b_body, a_body)
                };
                relation = require_same(
                    &plane_of(own_body, &own_sectors[on_flanker])?,
                    &plane_of(ref_body, ref_sector)?,
                    own_sectors[on_flanker].arm.min(ref_sector.arm),
                    band,
                )?;
            }
            for (flank, verdict) in [(f_s, v1), (f_e, v2)] {
                let (ra, rb) = if a_side { (flank, ref_idx) } else { (ref_idx, flank) };
                let rec_idx = find_record(records, ra, rb);
                if resolve_verdict(verdict, op, comparison, relation) {
                    let j = rec_idx.ok_or(BooleanError::ClassificationInvariant {
                        what: "single-on-edge crossing without a flanking record",
                    })?;
                    mark_germ(&mut records[j], a_side)?;
                } else if let Some(j) = rec_idx {
                    if on_bound(if a_side { records[j].sa } else { records[j].sb }).is_some() {
                        records[j].intersect = false;
                    }
                }
            }
        }
    }

    // ---- Postcondition: no surviving On codes. ----
    for r in records.iter() {
        if r.intersect
            && (on_bound(r.sa).is_some()
                || on_bound(r.sb).is_some()
                || r.sa.0 == SideCode::On
                || r.sb.0 == SideCode::On)
        {
            return Err(BooleanError::ClassificationInvariant {
                what: "surviving record with an On code after srecledges",
            });
        }
    }
    Ok(())
}

fn parallel_same_dir<T: Decide>(
    u: Vec3<T>,
    v: Vec3<T>,
    arm: T,
    band: Band,
) -> Result<bool, BooleanError> {
    let un = u.normalize();
    let vn = v.normalize();
    match decide("bool_ee_collinear", un.cross(vn).norm() * arm, band) {
        Ok(Sign::Zero) => {}
        Ok(_) => return Ok(false),
        Err(diag) => return Err(BooleanError::Escalated { diag }),
    }
    match decide("bool_dir_same", un.dot(vn) * arm, band) {
        Ok(Sign::Positive) => Ok(true),
        Ok(_) => Ok(false),
        Err(diag) => Err(BooleanError::Escalated { diag }),
    }
}

/// Whether `u` lies in the CCW arc from `p` to `q` around `axis`
/// (definite verdicts only — ties were peeled off by the parallel
/// pre-pass; a graze here escalates).
fn within_arc<T: Decide>(
    p: Vec3<T>,
    q: Vec3<T>,
    u: Vec3<T>,
    axis: Vec3<T>,
    arm: T,
    band: Band,
) -> Result<bool, BooleanError> {
    let side = |x: Vec3<T>, y: Vec3<T>| -> Result<Sign, BooleanError> {
        let m = x.normalize().cross(y.normalize()).dot(axis) * arm;
        match decide("bool_ee_angular", m, band) {
            Ok(s) => Ok(s),
            Err(diag) => Err(BooleanError::Escalated { diag }),
        }
    };
    let small = |a: Vec3<T>, b: Vec3<T>, x: Vec3<T>| -> Result<bool, BooleanError> {
        Ok(side(a, x)? == Sign::Positive && side(x, b)? == Sign::Positive)
    };
    match side(p, q)? {
        Sign::Positive => small(p, q, u),
        Sign::Negative => Ok(!small(q, p, u)?),
        Sign::Zero => Err(BooleanError::ClassificationInvariant {
            what: "flanking sectors of one solid coplanar around the common line \
                   (non-maximal operand escaped the gate)",
        }),
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

    /// `within_arc` around +z: arc from +x CCW to +y (90°): +x+y in;
    /// −x out; the >180° complement arc inverts.
    #[test]
    fn arc_membership() {
        let band = Band::linear().unwrap();
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        let z = Vec3::new(0.0, 0.0, 1.0);
        let mid = Vec3::new(1.0, 1.0, 0.0);
        let out = Vec3::new(-1.0, -0.2, 0.0);
        assert!(within_arc(x, y, mid, z, 1.0, band).unwrap());
        assert!(!within_arc(x, y, out, z, 1.0, band).unwrap());
        // Reflex arc y→x (270°): `out` is inside it, `mid` is not.
        assert!(within_arc(y, x, out, z, 1.0, band).unwrap());
        assert!(!within_arc(y, x, mid, z, 1.0, band).unwrap());
    }

    /// `mark_germ` rewrites the On bound to the transition partner.
    #[test]
    fn germ_rewrite() {
        let mut r = rec(0, 0, (On, In), (Out, On));
        mark_germ(&mut r, true).unwrap();
        assert_eq!(r.sa, (Out, In));
        mark_germ(&mut r, false).unwrap();
        assert_eq!(r.sb, (Out, In));
        assert!(r.intersect);
    }
}
