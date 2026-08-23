//! Paired null-edge insertion (Programs 15.11/15.12 motion, F9/F12).
//!
//! Surviving records — each a section-polygon edge germ with one In and
//! one Out code per side — are paired **consecutively in A-major
//! order** (the book's consumption), and each pair mints one null edge
//! in each solid spanning the orbit run between the two germs, with:
//!
//! - **F9 attributes as data**: the run's side (the shared code between
//!   the paired records) decides the minted copy's side —
//!   `NewVertexSide::Below` for an In-run (below ≙ IN), `Above` for an
//!   Out-run — derived from the F3 chain, never a he1/he2 slot.
//! - **Explicit cross-body correspondence keys**
//!   ([`super::NullEdgePairRecord`]): the A-edge and B-edge of a pair
//!   are tied by key, never by array position (`ssortnulledges` is
//!   engineered out).
//! - **The 15.11 consecutive-pairing invariant, GUARDED (F12)**: the
//!   book consumes surviving records two at a time and never argues
//!   that A-consecutive pairs are also B-consecutive for > 2 crossings.
//!   We check it: each pair must be cyclically adjacent among survivors
//!   in B-order too, and the run-side codes must agree at both ends
//!   (`r.own_start_code == r'.own_end_code` per solid). Violation ⇒
//!   typed [`super::BooleanError::PairingMismatch`] — fail-loud, never
//!   a mis-joined seam. The 4-crossing stress fixtures pin the passing
//!   cases.
//!
//! Run extraction: a germ in sector `k` transitions that sector's codes
//! `end → start` walking the array forward (array order follows the
//! orbit; the forward-crossed bound is the sector's START). The run
//! from germ r to germ r′ therefore spans sectors `r.own+1 ..= r′.own`,
//! whose orbit half-edges (deduplicated across subdivision twins) form
//! the `mev_null` fan; an empty span (both germs in one sector) is the
//! strut/dangling case (`Fan { he, he }` at the next sector's edge).

use geom_core::{Band, Decide, Margin, Sign, Vec3};

use super::sectors::{BoolSector, PairRecord, within};
use super::{
    BoolNullEdgeRecord, BooleanError, NullEdgePairRecord, Operand, PairSite, SideCode, VvContact,
};
use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, VertexKey};
use crate::euler::MevSite;
use crate::null::{NewVertexSide, NullEdge};

/// Output of one vertex-pair insertion.
#[derive(Debug)]
pub(super) struct InsertOut<T: geom_core::Real> {
    /// Minted edges, both operands.
    pub edges: Vec<BoolNullEdgeRecord<T>>,
    /// The correspondence pairs.
    pub pairs: Vec<NullEdgePairRecord>,
}

/// Validates codes, pairs survivors, mints the null edges in both
/// solids (module docs).
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_arguments)]
pub(super) fn insert_null_pairs<T: Decide>(
    a_body: &mut Body<T>,
    b_body: &mut Body<T>,
    contact: VvContact,
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    records: &[PairRecord],
    declared: &super::DeclaredPairs,
    band: Band,
) -> Result<InsertOut<T>, BooleanError> {
    let survivors: Vec<&PairRecord> = records.iter().filter(|r| r.intersect).collect();
    let mut out = InsertOut {
        edges: Vec::new(),
        pairs: Vec::new(),
    };
    if survivors.is_empty() {
        return Ok(out); // touching without crossing: 3′ contact only
    }
    if !survivors.len().is_multiple_of(2) {
        return Err(BooleanError::ClassificationInvariant {
            what: "odd number of surviving crossing records at a vertex pair",
        });
    }
    for r in &survivors {
        let clean = |c: (SideCode, SideCode)| {
            (c.0 == SideCode::In && c.1 == SideCode::Out)
                || (c.0 == SideCode::Out && c.1 == SideCode::In)
        };
        if !clean(r.sa) || !clean(r.sb) {
            return Err(BooleanError::ClassificationInvariant {
                what: "surviving record without one In and one Out code per side",
            });
        }
    }
    // Survivors arrive in creation order = A-major; pair consecutively
    // (cyclically, starting at the first survivor).
    let mismatch = || BooleanError::PairingMismatch {
        a_vertex: contact.a,
        b_vertex: contact.b,
    };
    // B-order of survivors for the adjacency guard.
    let mut b_order: Vec<usize> = (0..survivors.len()).collect();
    b_order.sort_by_key(|&i| (survivors[i].b, survivors[i].a));
    let b_pos = |i: usize| b_order.iter().position(|&j| j == i).unwrap_or(usize::MAX);

    for pair_idx in 0..survivors.len() / 2 {
        let (i0, i1) = (2 * pair_idx, 2 * pair_idx + 1);
        let (r0, r1) = (survivors[i0], survivors[i1]);
        // F12 guard 1: B-cyclic adjacency of the pair among survivors.
        let (p0, p1) = (b_pos(i0), b_pos(i1));
        let n = survivors.len();
        let adjacent = (p0 + 1) % n == p1 || (p1 + 1) % n == p0;
        if !adjacent {
            return Err(mismatch());
        }
        // Which forward run is the corner's wedge is decided by DATA,
        // not index parity (ambiguous at two survivors): default to the
        // forward run r0 → r1 (the book's consumption order); if that
        // run would swallow the entire orbit — impossible for the true
        // wedge, whose far side the complementary germ bounds — the
        // wedge is the other direction (r1 → r0). Applied per solid;
        // the run-side agreement guard runs against whichever
        // direction is chosen.
        let g0_faces = (a_sectors[r0.a].face, b_sectors[r0.b].face);
        let g1_faces = (a_sectors[r1.a].face, b_sectors[r1.b].face);
        let g0_dir = record_germ_dir(
            a_body,
            b_body,
            &a_sectors[r0.a],
            &b_sectors[r0.b],
            declared,
            band,
        )?;
        let g1_dir = record_germ_dir(
            a_body,
            b_body,
            &a_sectors[r1.a],
            &b_sectors[r1.b],
            declared,
            band,
        )?;
        let (a_rec, a_swapped) = mint_directed(
            a_body,
            Operand::A,
            contact.a,
            a_sectors,
            (r0.a, r0.sa, g0_faces, g0_dir),
            (r1.a, r1.sa, g1_faces, g1_dir),
            mismatch,
            band,
        )?;
        let (b_rec, b_swapped) = mint_directed(
            b_body,
            Operand::B,
            contact.b,
            b_sectors,
            (r0.b, r0.sb, g0_faces, g0_dir),
            (r1.b, r1.sb, g1_faces, g1_dir),
            mismatch,
            band,
        )?;
        // Slot canonicalization (the joining's slot lock): germ slot i
        // of the A and B records must be the SAME spatial germ; a
        // degeneracy swap in one solid only would misalign them, so
        // align B's array to A's (entries carry their halves — the
        // germ↔half binding is untouched).
        let mut b_rec = b_rec;
        if a_swapped != b_swapped {
            b_rec.germs.swap(0, 1);
        }
        out.pairs.push(NullEdgePairRecord {
            a_edge: a_rec.edge,
            b_edge: b_rec.edge,
            site: PairSite::VertexVertex(contact),
        });
        out.edges.push(a_rec);
        out.edges.push(b_rec);
    }
    Ok(out)
}

/// Chooses the run direction for one solid (doc at the call site) and
/// mints: forward `g0 → g1` unless that run swallows the whole orbit
/// (detected structurally: the fan's orbit successor of its last edge
/// is its first — the true wedge cannot, its far side being bounded by
/// the complementary germ), in which case `g1 → g0`. The F12 run-side
/// agreement guard (`entry germ's exit code == closing germ's entry
/// code`) applies to whichever direction is chosen.
type Germ<T> = (usize, (SideCode, SideCode), (FaceKey, FaceKey), Vec3<T>);

#[allow(clippy::too_many_arguments)]
fn mint_directed<T: Decide>(
    body: &mut Body<T>,
    operand: Operand,
    vertex: VertexKey,
    sectors: &[BoolSector<T>],
    g0: Germ<T>,
    g1: Germ<T>,
    mismatch: impl Fn() -> BooleanError,
    band: Band,
) -> Result<(BoolNullEdgeRecord<T>, bool), BooleanError> {
    let swapped = run_degenerates(body, sectors, g0.0, g1.0)?;
    let (gf, gt) = if swapped { (g1, g0) } else { (g0, g1) };
    // Strut spike ORDER (PR 5.5, the sort half of ssortnulledges): a
    // dangling strut's two halves splice consecutively into the loop
    // as [he_plus, he_minus]; interleaved (crossing) chords at
    // multi-germ corner sites wall pending pairs off, so the half the
    // loop walk meets FIRST (he_plus) must face the germ angularly
    // closest to the splice corner's arrival edge (the anchor bound;
    // convex-sector dot comparison). Senses follow the facing by the
    // sense theorem, so only the splice order moves. Run direction is
    // untouched (a strut's reverse run spans the whole orbit).
    let spike_from_first = if run_fan(sectors, gf.0, gt.0)?.is_empty() {
        let e_dir = anchor_dir(body, sectors[gf.0].he)?;
        // Metered at the shorter sector arm: the germ directions and
        // `e_dir` are all unit, so the bare dot difference was a
        // DIMENSIONLESS comparand against the length band
        // (rim-dimensional audit, class (c)); × arm makes it the
        // displacement the facing difference induces at the sectors'
        // own bounding-chord scale.
        let arm = sectors[gf.0].arm.min(sectors[gt.0].arm);
        let m = Margin::levered((gf.3 - gt.3).dot(e_dir), arm);
        match crate::validate::decide("bool_strut_order", m, band) {
            Ok(Sign::Positive) => true,
            Ok(_) => false,
            Err(diag) => return Err(BooleanError::Escalated { diag }),
        }
    } else {
        false
    };
    let (from, to, side, closing) = (gf.0, gt.0, gf.1.0, gt.1.1);
    // F12 guard 2: the closing germ must approach the run with the
    // run's own side as its entry code.
    if closing != side {
        return Err(mismatch());
    }
    // Germ facings as data (module docs): he_plus faces the from-germ,
    // he_minus the to-germ (the mev splice contract).
    let meta = [(gf.2, gf.3), (gt.2, gt.3)];
    let rec = mint_run(
        body,
        operand,
        vertex,
        sectors,
        from,
        to,
        side,
        meta,
        spike_from_first,
    )?;
    Ok((rec, swapped))
}

/// The unit direction of an orbit half-edge away from its start
/// vertex (the strut-order comparison's angular reference).
fn anchor_dir<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Result<Vec3<T>, BooleanError> {
    let corrupt = || BooleanError::ClassificationInvariant {
        what: "strut anchor edge no longer resolves",
    };
    let hd = body.get_half_edge(he).ok_or_else(corrupt)?;
    let p_of = |v: crate::entity::VertexKey| -> Result<geom_core::Point3<T>, BooleanError> {
        body.get_vertex(v)
            .and_then(|vd| body.get_point(vd.point).copied())
            .ok_or_else(corrupt)
    };
    let end = body.half_edge_end(he).ok_or_else(corrupt)?;
    let d = p_of(end)? - p_of(hd.start)?;
    Ok(d.normalize())
}

/// The record's germ direction, by declared class: a `Tangent` pair's
/// sector normals are PARALLEL along the contact (the tangency), so
/// its germ direction is the verified closed-form locus
/// ([`super::rest::tangent_locus`] — the DEV-1 witness the door
/// derived), signed into the sector pair by the same membership test;
/// every other pair takes the transverse normal cross ([`germ_dir`]).
fn record_germ_dir<T: Decide>(
    a_body: &Body<T>,
    b_body: &Body<T>,
    sa: &BoolSector<T>,
    sb: &BoolSector<T>,
    declared: &super::DeclaredPairs,
    band: Band,
) -> Result<Vec3<T>, BooleanError> {
    if declared.class_of(super::Operand::A, sa.face, super::Operand::B, sb.face)
        != Some(crate::contact::ContactClass::Tangent)
    {
        return germ_dir(sa, sb, band);
    }
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
    let d = match super::rest::tangent_locus(&s_a, &s_b, band) {
        Ok(super::rest::TangentLocus::Line { dir, .. }) => dir.normalize(),
        Err(super::rest::TangentLocusError::Escalated(diag)) => {
            return Err(BooleanError::Escalated { diag });
        }
        Err(_) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "declared-Tangent germ without a closed-form locus",
            });
        }
    };
    let plus = within(sa, d, false, band)? && within(sb, d, false, band)?;
    let minus = within(sa, -d, false, band)? && within(sb, -d, false, band)?;
    match (plus, minus) {
        (true, false) => Ok(d),
        (false, true) => Ok(-d),
        _ => Err(BooleanError::ClassificationInvariant {
            what: "germ direction not uniquely within its sector pair",
        }),
    }
}

/// The germ's outgoing direction: the unit intersection direction of
/// the two sector faces' planes, signed to lie within both sectors
/// (grazes count — an on-bound germ's direction IS the bound). An
/// ambiguous or coplanar configuration refuses loudly.
///
/// Sense-invariant given its sources (S10): `±(n_a × n_b)` is a LINE,
/// and the sign is chosen by sector membership, not by either normal —
/// flipping a normal flips the raw cross product and the `within`
/// verdicts pick the same ray back out. The normals arrive already
/// oriented from `sectors::sector_face`; nothing is multiplied here.
fn germ_dir<T: Decide>(
    sa: &BoolSector<T>,
    sb: &BoolSector<T>,
    band: Band,
) -> Result<Vec3<T>, BooleanError> {
    let int = sa.normal.vec().cross(sb.normal.vec());
    let arm = sa.arm.min(sb.arm);
    match crate::validate::decide("bool_germ_line", Margin::levered(int.norm(), arm), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "surviving crossing record on coplanar sector faces",
            });
        }
        Err(diag) => return Err(BooleanError::Escalated { diag }),
    }
    let d = int.normalize();
    let plus = within(sa, d, false, band)? && within(sb, d, false, band)?;
    let minus = within(sa, -d, false, band)? && within(sb, -d, false, band)?;
    match (plus, minus) {
        (true, false) => Ok(d),
        (false, true) => Ok(-d),
        _ => Err(BooleanError::ClassificationInvariant {
            what: "germ direction not uniquely within its sector pair",
        }),
    }
}

/// Whether the forward run `from → to` would swallow the entire orbit
/// (a nonempty fan whose orbit successor wraps to its first member).
fn run_degenerates<T: Decide>(
    body: &Body<T>,
    sectors: &[BoolSector<T>],
    from: usize,
    to: usize,
) -> Result<bool, BooleanError> {
    let hes = run_fan(sectors, from, to)?;
    let Some((&first, &last)) = hes.first().zip(hes.last()) else {
        return Ok(false); // empty fan: a valid strut
    };
    let mate = body
        .mate(last)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "run edge without a mate",
        })?;
    let successor = body
        .get_half_edge(mate)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "run edge mate no longer resolves",
        })?
        .next;
    Ok(successor == first)
}

/// The real edge bounds crossed walking the entry chain forward from
/// entry `from` (exclusive) to entry `to` (inclusive).
fn run_fan<T: Decide>(
    sectors: &[BoolSector<T>],
    from: usize,
    to: usize,
) -> Result<Vec<HalfEdgeKey>, BooleanError> {
    let n = sectors.len();
    let mut hes: Vec<HalfEdgeKey> = Vec::new();
    if from != to {
        let mut k = (from + 1) % n;
        loop {
            if sectors[k].end_edge {
                hes.push(sectors[k].he);
            }
            if k == to {
                break;
            }
            k = (k + 1) % n;
            if k == (from + 1) % n {
                return Err(BooleanError::ClassificationInvariant {
                    what: "run walk wrapped without reaching its closing germ",
                });
            }
        }
    }
    Ok(hes)
}

/// Mints one null edge spanning the run from the germ in sector entry
/// `from` (exclusive) through entry `to` (inclusive).
///
/// The fan = the **real edge bounds crossed** walking the entry chain
/// forward from the first germ to the second: entering entry `k`
/// crosses the shared bound `sectors[k].end`, which is the orbit edge
/// `sectors[k].he` exactly when `end_edge` — a subdivision-twin
/// boundary (bisector) is crossed without moving any edge. (The
/// original per-entry `he` collection mis-moved fans whenever a germ
/// sat in a wide sector's twin — the bisector-graze lane of the
/// coplanar corpus; entries are pieces, not physical sectors.)
///
/// An empty fan — `from == to`, or germs in two twins of one physical
/// sector — is the dangling strut, spliced INSIDE that physical
/// sector: at the orbit successor of the sector's own half
/// (`next(mate(sectors[from].he))`), which is twin-stable (twins share
/// `he`).
#[allow(clippy::too_many_arguments)]
fn mint_run<T: Decide>(
    body: &mut Body<T>,
    operand: Operand,
    vertex: VertexKey,
    sectors: &[BoolSector<T>],
    from: usize,
    to: usize,
    run_side: SideCode,
    germ_meta: [((FaceKey, FaceKey), Vec3<T>); 2],
    spike_from_first: bool,
) -> Result<BoolNullEdgeRecord<T>, BooleanError> {
    let hes = run_fan(sectors, from, to)?;
    let successor = |body: &Body<T>, he: HalfEdgeKey| -> Result<HalfEdgeKey, BooleanError> {
        let mate = body
            .mate(he)
            .ok_or(BooleanError::CorruptOperand { operand, vertex })?;
        Ok(body
            .get_half_edge(mate)
            .ok_or(BooleanError::CorruptOperand { operand, vertex })?
            .next)
    };
    let (site, dangling) = if hes.is_empty() {
        // The dangling strut, inside `from`'s physical sector.
        let he = successor(body, sectors[from].he)?;
        (MevSite::Fan { he1: he, he2: he }, true)
    } else {
        let first = hes[0];
        let last = *hes.last().unwrap_or(&first);
        // he2 at execution time: current orbit successor of the run's
        // last half-edge (PR 2's pattern — robust against prior
        // splices).
        let he2 = successor(body, last)?;
        if he2 == first {
            // The run would swallow the whole orbit — the complementary
            // germ must bound it (kernel bug in run selection, loudly:
            // mev would silently degrade this site to a strut).
            return Err(BooleanError::ClassificationInvariant {
                what: "null-edge run spans the entire vertex orbit",
            });
        }
        (MevSite::Fan { he1: first, he2 }, false)
    };
    // The copy takes the run; its side is the run's side (F3-derived).
    let new_side = match run_side {
        SideCode::In => NewVertexSide::Below,
        SideCode::Out => NewVertexSide::Above,
        SideCode::On => {
            return Err(BooleanError::ClassificationInvariant {
                what: "run with On side reached insertion",
            });
        }
    };
    // Side attributes per the PR 5.5 sense theorem (join module docs):
    // the half FACING a germ is UP (starts at `below_end`) iff that
    // germ's own forward-wedge code is Out. Non-dangling: he_plus
    // (old → new) faces the from-germ whose forward code is the run
    // side, so `created` is the below end exactly for In-runs.
    // Dangling struts in the default spike order swap the facing
    // (he_minus at the from-germ), so the SIDE swaps with it; the
    // angular `spike_from_first` order restores the non-dangling
    // facing. The attribute is derived sense data, never a mint-slot
    // echo; the mint side follows so the body's scaffold attribute and
    // the pipeline record stay one datum.
    let attr_side = match (new_side, dangling && !spike_from_first) {
        (side, false) => side,
        (NewVertexSide::Below, true) => NewVertexSide::Above,
        (NewVertexSide::Above, true) => NewVertexSide::Below,
    };
    let created = body.mev_null(site, attr_side)?;
    let attr = match attr_side {
        NewVertexSide::Below => NullEdge {
            below_end: created.vertex,
            above_end: vertex,
        },
        NewVertexSide::Above => NullEdge {
            below_end: vertex,
            above_end: created.vertex,
        },
    };
    let germ = |i: usize, he: crate::entity::HalfEdgeKey| super::HalfGerm {
        he,
        a_face: germ_meta[i].0.0,
        b_face: germ_meta[i].0.1,
        dir: germ_meta[i].1,
    };
    // Germ ↔ half facing: for a fan the mev splice puts he_plus at the
    // from-germ cut and he_minus at the to-germ cut; a strut's spike
    // splices [he_plus, he_minus] into one corner, and which germ the
    // loop-first half (he_plus) faces is the angular spike order
    // decided at the mint site (`spike_from_first`; the default — the
    // corner walk arriving through the to-germ — was pinned
    // empirically by the joining fixtures).
    let germs = if dangling && !spike_from_first {
        [germ(0, created.he_minus), germ(1, created.he_plus)]
    } else {
        [germ(0, created.he_plus), germ(1, created.he_minus)]
    };
    Ok(BoolNullEdgeRecord {
        operand,
        at_vertex: vertex,
        edge: created.edge,
        attr,
        dangling,
        germs,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;

    /// The survivor-validation invariants: odd counts and dirty codes
    /// refuse loudly (unit-level; the geometric paths are pinned by the
    /// acceptance fixtures).
    #[test]
    fn survivor_validation() {
        let mk = |sa, sb| PairRecord {
            a: 0,
            b: 0,
            sa,
            sb,
            intersect: true,
        };
        let mut a = crate::fixtures::ops_cube(Tol::witness()).body;
        let mut b = crate::fixtures::ops_cube(Tol::witness()).body;
        let contact = VvContact {
            a: VertexKey::default(),
            b: VertexKey::default(),
        };
        use SideCode::{In, Out};
        let recs = vec![mk((In, Out), (In, Out))];
        let err = insert_null_pairs(
            &mut a,
            &mut b,
            contact,
            &[],
            &[],
            &recs,
            &crate::boolean::DeclaredPairs::default(),
            geom_core::Band::linear(Tol::witness()).unwrap(),
        )
        .unwrap_err();
        assert!(matches!(err, BooleanError::ClassificationInvariant { .. }));
        let recs = vec![mk((In, In), (In, Out)), mk((Out, In), (Out, In))];
        let err = insert_null_pairs(
            &mut a,
            &mut b,
            contact,
            &[],
            &[],
            &recs,
            &crate::boolean::DeclaredPairs::default(),
            geom_core::Band::linear(Tol::witness()).unwrap(),
        )
        .unwrap_err();
        assert!(matches!(err, BooleanError::ClassificationInvariant { .. }));
    }

    /// F12 guard: a 4-survivor set whose A-consecutive pair is NOT
    /// B-cyclically adjacent refuses as the typed PairingMismatch —
    /// the 15.11 consecutive-pairing invariant is never silently
    /// assumed (a plus-sign-interleaved b-order: A pairs (0,1) but in
    /// B-order the survivors interleave 0,2,1,3).
    #[test]
    fn f12_pairing_mismatch_guard() {
        use SideCode::{In, Out};
        let mk = |a: usize, b: usize, sa, sb| PairRecord {
            a,
            b,
            sa,
            sb,
            intersect: true,
        };
        let mut abody = crate::fixtures::ops_cube(Tol::witness()).body;
        let mut bbody = crate::fixtures::ops_cube(Tol::witness()).body;
        let contact = VvContact {
            a: VertexKey::default(),
            b: VertexKey::default(),
        };
        // A-order: a = 0,1,2,3; B-order by b: r0(b=0), r2(b=1),
        // r1(b=2), r3(b=3) — pair (r0, r1) is not B-adjacent.
        let recs = vec![
            mk(0, 0, (In, Out), (In, Out)),
            mk(1, 2, (Out, In), (Out, In)),
            mk(2, 1, (In, Out), (In, Out)),
            mk(3, 3, (Out, In), (Out, In)),
        ];
        let err = insert_null_pairs(
            &mut abody,
            &mut bbody,
            contact,
            &[],
            &[],
            &recs,
            &crate::boolean::DeclaredPairs::default(),
            geom_core::Band::linear(Tol::witness()).unwrap(),
        )
        .unwrap_err();
        assert!(
            matches!(err, BooleanError::PairingMismatch { .. }),
            "{err:?}"
        );
    }

    /// F12 stress, mechanism level: a valid 4-survivor (two-pair)
    /// neighborhood — two consecutive record pairs, each a strut pair
    /// in BOTH solids — mints 4 dangling null edges and 2
    /// correspondence pairs on real cube vertices, F9 attributes
    /// carried as data, tier 1 preserved. (A geometric 4-crossing
    /// operand fixture is not constructible from the prismatic corpus;
    /// the runtime guards above defend the invariant — PR report.)
    #[test]
    fn f12_four_survivor_pairing() {
        use SideCode::{In, Out};
        let mk = |a: usize, b: usize, sa, sb| PairRecord {
            a,
            b,
            sa,
            sb,
            intersect: true,
        };
        let mut abody = crate::fixtures::ops_cube(Tol::witness()).body;
        let mut bbody = crate::fixtures::ops_cube(Tol::witness()).body;
        // A and B sector fans on NON-parallel face planes (the germ
        // direction z×x = +y is uniquely within both — `germ_dir`
        // refuses coplanar sector pairs by design).
        let sectors_of = |body: &Body<f64>,
                          normal: geom_brep::OutwardNormal<f64>,
                          start: geom_core::Vec3<f64>,
                          end: geom_core::Vec3<f64>| {
            let (vk, v) = body.vertices().next().unwrap();
            let orbit = body.vertex_orbit(v.emanating.unwrap()).unwrap();
            let secs: Vec<BoolSector<f64>> = orbit
                .iter()
                .map(|&he| BoolSector {
                    he,
                    start,
                    end,
                    start_edge: true,
                    end_edge: true,
                    face: crate::entity::FaceKey::default(),
                    normal,
                    arm: 1.0,
                })
                .collect();
            (vk, secs)
        };
        // Proper quarter sectors on non-parallel planes: the germ line
        // z×x = +y is uniquely within both.
        let (va, a_sectors) = sectors_of(
            &abody,
            geom_brep::OutwardNormal::from_chart(geom_core::Vec3::new(0.0, 0.0, 1.0), true),
            geom_core::Vec3::new(1.0, 0.0, 0.0),
            geom_core::Vec3::new(0.0, 1.0, 0.0),
        );
        let (vb, b_sectors) = sectors_of(
            &bbody,
            geom_brep::OutwardNormal::from_chart(geom_core::Vec3::new(1.0, 0.0, 0.0), true),
            geom_core::Vec3::new(0.0, 1.0, 0.0),
            geom_core::Vec3::new(0.0, 0.0, 1.0),
        );
        let contact = VvContact { a: va, b: vb };
        // Two consecutive pairs, each pair a strut in both solids
        // (identical sector indices within the pair), codes mirrored.
        let recs = vec![
            mk(0, 0, (Out, In), (Out, In)),
            mk(0, 0, (In, Out), (In, Out)),
            mk(1, 1, (Out, In), (Out, In)),
            mk(1, 1, (In, Out), (In, Out)),
        ];
        let out = insert_null_pairs(
            &mut abody,
            &mut bbody,
            contact,
            &a_sectors,
            &b_sectors,
            &recs,
            &crate::boolean::DeclaredPairs::default(),
            geom_core::Band::linear(Tol::witness()).unwrap(),
        )
        .unwrap();
        assert_eq!(out.pairs.len(), 2);
        assert_eq!(out.edges.len(), 4);
        assert!(out.edges.iter().all(|e| e.dangling));
        for e in &out.edges {
            assert_ne!(e.attr.below_end, e.attr.above_end);
            // Dangling Out-run struts: the strut facing swap (he_minus
            // at the from-germ) swaps the side labels with it (PR 5.5
            // sense theorem — join module docs), so the minted copy is
            // the below end here, NOT an echo of the mint side.
            assert_eq!(
                e.attr.above_end,
                if e.operand == Operand::A { va } else { vb }
            );
        }
        crate::validate::validate(&abody).unwrap();
        crate::validate::validate(&bbody).unwrap();
    }
}
