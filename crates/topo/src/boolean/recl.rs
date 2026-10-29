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
//!   rules — see [`resolve_edge_edge`]). Where one solid holds several
//!   mentions on the ray (a contact line), each solid's material about
//!   the line is wedges ([`Wedge`]): a real edge's between its flanking
//!   faces, and a face's that holds the line, a half-turn, where a
//!   subdivision bisector parts it on the ray or where the solid holds
//!   nothing else there. Each wedge of one solid meets each of the
//!   other's by the edge-edge rule.
//!
//! Germ attribution: a crossing along an on-edge is recorded on the
//! flanking sector the one fold rule
//! ([`super::sectors::fold_on_bound`], placed by
//! [`super::sectors::crossing_flank`]) makes the transition — the
//! on-bound joins the In run, so the germ goes on the flanker whose
//! other bound reads Out — with the On code rewritten to the transition
//! partner of that other bound. At an edge-edge site each solid applies
//! the rule to its OWN flankers' membership keys, apart: the germ's
//! record is the pair of the two solids' transition flankers, minted
//! when `pair_search` met no such pair (two coplanar flankers touching
//! along the edge never make a record of their own). An edge-sector
//! crossing's record is minted the same way where the search did not
//! meet the fold flanker (`resolve_edge_sector`). Two germs leave the
//! fold rule's flanker for the edge's other one, inside the other
//! solid's wedge: an edge two crossing pairs at one vertex both run a
//! germ along, which folds Out in each ([`Reversed`]); and a wedge that
//! crosses both wedges of a slit's line, whose germs then lie apart on
//! their own sides ([`onto_inside`]).
//!
//! Postcondition (checked loudly): no surviving record carries an On
//! code.

use geom_core::{Band, Decide, Margin, Sign, Vec3};

use super::carrier_eq::CarrierDesc;
use super::plane_eq::{PlaneEqError, PlaneRelation};
use super::refusal_routes::RAY_COLLINEAR;
use super::sectors::{
    BoolSector, Flank, PairRecord, Reach, crossing_flank, pair_codes, parallel_same, side_code,
};
use super::tables::{
    TableIiVerdict, eq15_3_lump, kept_copy, resolve_verdict, table_ii, table_iii_rule,
};
use super::{BooleanError, BooleanOp, Coincide, DeclarationRead, Operand, SideCode, SlitLineGap};
use crate::body::Body;
use crate::entity::EntityId;
use crate::live::{Proven, proven};
use crate::validate::decide;
use geom_core::k_stats::NonzeroSign;

/// The sector face's ORIENTED carrier description
/// ([`super::carrier_pair::face_carrier`] —
/// the face's material side with S10's sense bit already folded in,
/// which is what the Same±-orientation verdict below has to mean: the
/// whole point of the verdict is which way the two materials face).
/// A kind outside the `Rest` ladder's [`super::carrier_eq::INVENTORY`]
/// is the C5 typed refusal.
fn carrier_of<T: Decide>(
    body: &Body<T>,
    operand: super::Operand,
    s: &BoolSector<T>,
) -> Result<CarrierDesc<T>, BooleanError> {
    super::carrier_pair::face_carrier(body, s.face).ok_or_else(|| {
        let kind = body
            .get_face(s.face)
            .and_then(|f| body.get_surface(f.surface))
            .map_or(geom::SurfaceKind::Nurbs, geom::Surface::kind);
        BooleanError::CurvedBooleanUnsupported {
            operand,
            face: s.face,
            kind,
        }
    })
}

/// The geometrically-ON sector pair's carrier identity check. A pair
/// the operation glued (a verified `Rest` or continuation declaration,
/// or a pair the glue door decided one carrier and declared) reads as
/// the declaration door read it at rest, over both faces; any other
/// pair over the sectors `on` names about the corner, where a
/// coincidence decided Zero is one the door did not glue and refuses on
/// the reading that binds it ([`super::unglued_corner`]).
///
/// C8: a CURVED sector pair descends the ladder only as a glued
/// one-carrier pair; an unglued on-carrier curved pair keeps the typed
/// frontier refusal.
#[allow(clippy::too_many_arguments)]
pub(super) fn require_same<T: Decide>(
    body1: &Body<T>,
    o1: super::Operand,
    s1: &BoolSector<T>,
    body2: &Body<T>,
    o2: super::Operand,
    s2: &BoolSector<T>,
    declared: &super::GluedPairs<T>,
    on: ReadOn,
    arm: T,
    band: Band,
) -> Result<PlaneRelation, BooleanError> {
    // A glued tangency has no carrier-identity answer (its carriers are
    // distinct); every lane that meets one lumps it by its curving
    // ([`super::sectors::tangent_lump`]) before reaching here.
    if declared.declares_tangent(o1, s1.face, o2, s2.face) {
        return Err(BooleanError::ClassificationInvariant {
            what: "a glued tangency reached the carrier-identity check",
        });
    }
    let c1 = carrier_of(body1, o1, s1)?;
    let c2 = carrier_of(body2, o2, s2)?;
    let declared_one_carrier = declared.declares_one_carrier(o1, s1.face, o2, s2.face);
    if !declared_one_carrier {
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
                .map_or(geom::SurfaceKind::Nurbs, geom::Surface::kind);
            return Err(BooleanError::CurvedBooleanUnsupported {
                operand,
                face,
                kind,
            });
        }
    }
    let id = super::PlaneIdentity {
        declared: declared_one_carrier,
    };
    // A glued pair reads as the door read it at rest, over both faces;
    // any other over the region `on` read it coplanar on, about the
    // corner.
    let reach = match on {
        ReadOn::Faces => arm,
        ReadOn::FirstBounds => s1.span(),
    };
    let extent = if declared_one_carrier {
        declared.consumed(o1, s1.face, o2, s2.face)?
    } else {
        super::carrier_eq::ConsumedExtent::unwitnessed(geom_brep::ExtentBall::new(
            corner_of(body1, s1),
            reach,
        ))
    };
    let pair = (o1, s1.face, o2, s2.face);
    match super::carrier_eq::carrier_eq_reading(&c1, &c2, id, &extent, band) {
        Ok((PlaneRelation::Distinct, ..)) if !declared_one_carrier && on == ReadOn::FirstBounds => {
            Err(unglued_tilt(&c1, &c2, s1, reach, declared, pair, band))
        }
        Ok((PlaneRelation::Distinct, ..)) => Err(BooleanError::ClassificationInvariant {
            what: "geometrically-ON sector pair with definitely-distinct carriers",
        }),
        Ok((rel, ..)) if declared_one_carrier => Ok(rel),
        Ok((rel, _, margin)) => {
            let bounds = match &c2 {
                CarrierDesc::Plane { normal, .. } => s1.bounds_off(*normal, band).err(),
                _ => None,
            };
            Err(super::unglued_corner(
                declared,
                pair,
                super::plane_eq::senses(s1.normal.vec(), s2.normal.vec(), arm, band),
                bounds,
                // A reading with no margin is one carrier by structure (one key,
                // or bit-identical descriptions), which the glue door glues
                // wherever the two faces meet, as they do at this corner.
                || match margin {
                    Some(margin) => {
                        super::unglued_coincidence(margin, declared.carriers_read(pair, rel), band)
                    }
                    None => BooleanError::ClassificationInvariant {
                        what: "a corner pair one carrier by structure the glue door left unglued",
                    },
                },
            ))
        }
        Err(PlaneEqError::Escalated { rung, diag }) => Err(BooleanError::plane_identity(
            rung,
            declared.on_pair_door(
                pair,
                super::plane_eq::senses(s1.normal.vec(), s2.normal.vec(), arm, band),
            ),
            diag,
        )),
        Err(PlaneEqError::Undecided {
            coincidence,
            relation,
        }) => Err(super::undecided_coincidence(
            coincidence,
            [(o1, s1.face), (o2, s2.face)],
            declared.carriers_read(pair, relation),
        )),
        Err(PlaneEqError::Contradicted { fact, .. }) => {
            Err(BooleanError::DeclarationContradicted { fact })
        }
        // Only a declared reading is unsettled.
        Err(PlaneEqError::Unsettled { diag }) => Err(super::unsettled_rest(
            declared.class_of(o1, s1.face, o2, s2.face).ok_or(
                BooleanError::ClassificationInvariant {
                    what: "an undeclared sector pair's carrier reading was unsettled",
                },
            )?,
            diag,
        )),
    }
}

/// What read a [`require_same`] pair coplanar: the region its
/// coincidence speaks for, which the unglued pair's ladder reads over.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ReadOn {
    /// `pair_search` read the two faces parallel levered at the pair's
    /// arm (an all-On record): the ball of that radius about the corner.
    /// The ladder's parallel rung reads that same margin, so a
    /// `Distinct` is the corner's standing off two planes it lies on.
    Faces,
    /// The first sector's bounds, each On the second's plane at its own
    /// reach (a flanker's, or a wedge representative's): that sector,
    /// out to its farther bound.
    FirstBounds,
}

/// An unglued pair whose first sector's bounds read On the second's
/// plane and whose carriers the plane ladder read apart over that
/// sector's reach. Read at that reach, the two normals' tilt decided off
/// is a thin sector tilted about a bound: its bounds lie on the other's
/// plane to within the band and its plane does not, so the pair is
/// undecided at this tolerance on the bounds' own margin, which a
/// smaller tolerance reads off the plane (as `vtxfac`'s coplanar lump
/// refuses its twin). The offset, read at the corner, is the corner's
/// standing off two planes it lies on.
fn unglued_tilt<T: Decide>(
    c1: &CarrierDesc<T>,
    c2: &CarrierDesc<T>,
    s1: &BoolSector<T>,
    reach: T,
    declared: &super::GluedPairs<T>,
    (o1, f1, o2, f2): (
        Operand,
        crate::entity::FaceKey,
        Operand,
        crate::entity::FaceKey,
    ),
    band: Band,
) -> BooleanError {
    let defect = BooleanError::ClassificationInvariant {
        what: "geometrically-ON sector pair with definitely-distinct carriers",
    };
    let (CarrierDesc::Plane { normal: n1, .. }, CarrierDesc::Plane { normal: n2, .. }) = (c1, c2)
    else {
        return defect;
    };
    let tilted = decide(
        "bool_plane_parallel",
        Margin::levered(n1.cross(*n2).norm(), reach),
        band,
    );
    if !matches!(tilted, Ok(Sign::Positive)) {
        return defect;
    }
    match s1.bounds_off(*n2, band) {
        Err(diag) => BooleanError::coincidence(
            Coincide::Sectors,
            // A tilt decided off admits no class: no declaration settles it.
            super::sectors_read(declared, (o1, f1, o2, f2), None),
            diag,
        ),
        Ok(_) => BooleanError::ClassificationInvariant {
            what: "a sector bound read On departs definitely from the other's plane",
        },
    }
}

/// The point of the vertex a sector of `body` stands at: a line
/// bound's base, or else its orbit half-edge's start.
fn corner_of<T: Decide>(body: &Body<T>, s: &BoolSector<T>) -> geom_core::Point3<T> {
    match (s.start_reach, s.end_reach) {
        (Reach::Chord { base, .. }, _) | (_, Reach::Chord { base, .. }) => base,
        _ => body.resolve_vertex_point(
            proven(&body.half_edges, s.he, EntityId::HalfEdge).start,
            Proven,
        ),
    }
}

fn cancel_uniform(r: &mut PairRecord) {
    if super::sectors::one_side(r.sa) || super::sectors::one_side(r.sb) {
        r.intersect = false;
    }
}

/// Program 15.10 (module docs). `records` are rewritten in place,
/// sequentially, in creation (A-major) order — later coplanar pairs see
/// propagated codes, as the book.
///
/// Each coincident pair whose lump keeps one copy of the region is
/// pushed onto `covered` as `(A face, B face)`
/// (`BooleanReduction::covered`), and what it owes the dropped copy
/// onto `fates` ([`super::fate`]): each edge of the kept copy's sector
/// that runs into the dropped copy's, or the dropped sector the kept
/// one holds wholly. `first_read` holds the codes
/// as first read, parallel to `records`, which the germ loci are derived
/// from ([`super::sectors::germ_locus`]); a declared-`Tangent` record's
/// is overwritten with its second-order reading.
#[allow(clippy::too_many_arguments)]
pub(super) fn recl_sectors<T: Decide>(
    records: &mut [PairRecord],
    first_read: &mut [PairRecord],
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    a_body: &Body<T>,
    b_body: &Body<T>,
    op: BooleanOp,
    declared: &super::GluedPairs<T>,
    band: Band,
    covered: &mut Vec<(crate::entity::FaceKey, crate::entity::FaceKey)>,
    fates: &mut super::fate::Fates<T>,
) -> Result<(), BooleanError> {
    let (n_a, n_b) = (a_sectors.len(), b_sectors.len());
    for i in 0..records.len() {
        let r = records[i];
        if !r.coplanar() {
            continue;
        }
        let sa = &a_sectors[r.a];
        let sb = &b_sectors[r.b];
        let arm = sa.arm.min(sb.arm);
        // Declared-`Tangent` (distinct carriers touching): the all-on
        // reading is the tangent-plane degeneracy — the sectors do
        // NOT coincide, only the tangency locus does — so the record
        // descends PER BOUND to the second-order side
        // ([`super::sectors::tangent_relative_side`]): a bound along
        // the locus honestly stays `On` (the recl edge engine
        // adjudicates it, exactly as a first-order On), other bounds
        // take the side their carrier curves to over the bound's own
        // reach, the first-order code's lever. No Eq. 15.3 lump and
        // no neighbor propagation: nothing here is a coincident-
        // sector identity.
        let read = declared.read(
            &[(super::Operand::A, sa.face, super::Operand::B, sb.face)],
            Coincide::TangentSide,
            &[],
        );
        if read == DeclarationRead::Spent(crate::contact::GlueClass::Tangent) {
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
            let p = a_body.half_edge_start_point(sa.he).ok_or(
                BooleanError::ClassificationInvariant {
                    what: "v-v site lost its point",
                },
            )?;
            let side = |own: &geom::Surface<T>, other: &geom::Surface<T>, n, (d, reached)| {
                super::sectors::tangent_relative_side(own, other, n, p, d, reached, read, band)
            };
            let [a_start, a_end] = sa.reached_bounds();
            let [b_start, b_end] = sb.reached_bounds();
            let ca = (
                side(&s_a, &s_b, sb.normal, a_start)?,
                side(&s_a, &s_b, sb.normal, a_end)?,
            );
            let cb = (
                side(&s_b, &s_a, sa.normal, b_start)?,
                side(&s_b, &s_a, sa.normal, b_end)?,
            );
            // Per-bound propagation, the Rest loop's shape with the
            // SHARED bound's own code (definite codes only — an `On`
            // is this record's own edge-engine business): the sector
            // sharing r.a's start holds it as its END, and so on.
            let a_start_nbr = (r.a + 1) % n_a;
            let a_end_nbr = (r.a + n_a - 1) % n_a;
            let b_start_nbr = (r.b + 1) % n_b;
            let b_end_nbr = (r.b + n_b - 1) % n_b;
            for rec in records.iter_mut() {
                if rec.a == a_start_nbr
                    && rec.b == r.b
                    && rec.sa.0 != SideCode::On
                    && ca.0 != SideCode::On
                {
                    rec.sa.1 = ca.0;
                }
                if rec.a == a_end_nbr
                    && rec.b == r.b
                    && rec.sa.1 != SideCode::On
                    && ca.1 != SideCode::On
                {
                    rec.sa.0 = ca.1;
                }
                if rec.b == b_start_nbr
                    && rec.a == r.a
                    && rec.sb.0 != SideCode::On
                    && cb.0 != SideCode::On
                {
                    rec.sb.1 = cb.0;
                }
                if rec.b == b_end_nbr
                    && rec.a == r.a
                    && rec.sb.1 != SideCode::On
                    && cb.1 != SideCode::On
                {
                    rec.sb.0 = cb.1;
                }
                cancel_uniform(rec);
            }
            let rec = &mut records[i];
            rec.sa = ca;
            rec.sb = cb;
            cancel_uniform(rec);
            // The first-order reading of a tangent pair is all On; its
            // second-order one is the first that decides anything, and
            // the germ loci are read from it.
            first_read[i].sa = ca;
            first_read[i].sb = cb;
            continue;
        }
        let rel = require_same(
            a_body,
            super::Operand::A,
            sa,
            b_body,
            super::Operand::B,
            sb,
            declared,
            ReadOn::Faces,
            arm,
            band,
        )?;
        if let Some(keeper) = kept_copy(op, rel) {
            covered.push((sa.face, sb.face));
            let (body, own, other) = match keeper {
                Operand::A => (a_body, sa, sb),
                Operand::B => (b_body, sb, sa),
            };
            let mut runs = false;
            for (edge, dir, reach) in super::sectors::bound_reaches(body, own) {
                if super::sectors::runs_into(other, (dir, reach), arm, band)? {
                    runs = true;
                    fates.edges.push(super::fate::FateEdge {
                        holder: keeper,
                        edge,
                        kept: own.face,
                        face: other.face,
                        aligned: matches!(rel, super::PlaneRelation::SameOriented),
                    });
                }
            }
            if !runs {
                // The kept copy holds the dropped sector wholly, so the
                // dropped sector's direction names the piece of the kept
                // face over it, wherever the join divides that face after;
                // the kept sector's own may lie in another piece.
                let inside = super::fate::bisector(other, band)?;
                fates.corners.push(super::fate::FateCorner {
                    holder: keeper,
                    dropped: super::fate::SectorAt {
                        he: other.he,
                        face: other.face,
                        dir: inside,
                    },
                    kept: super::fate::KeptAt::Sector(super::fate::SectorAt {
                        he: own.he,
                        face: own.face,
                        dir: inside,
                    }),
                });
            }
        }
        let (newsa, newsb) = (
            eq15_3_lump(op, Operand::A, rel),
            eq15_3_lump(op, Operand::B, rel),
        );
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
/// start-holder `f_s`: the other-solid sector index of the first record,
/// in `records`, whose own-side On code names **that bound**
/// (start-holder match) — NOT any On record on the flanking twins: a
/// vertex pair can host
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

/// `idx` and its twins: the entries of one physical sector split at a
/// subdivision bisector, `idx + 1` across a bisector start (an entry's
/// start is the next one's end) and `idx − 1` across a bisector end.
/// A germ on a subdivided sector may be recorded on either.
fn twins<T: geom_core::Real>(secs: &[BoolSector<T>], idx: usize) -> Vec<usize> {
    let n = secs.len();
    let mut out = vec![idx];
    if !secs[idx].start_edge() {
        out.push((idx + 1) % n);
    }
    if !secs[idx].end_edge() {
        out.push((idx + n - 1) % n);
    }
    out
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
    let (bound, reach) = if key_from_start {
        (s.start, s.start_reach)
    } else {
        (s.end, s.end_reach)
    };
    side_code(
        bound,
        reach,
        ref_normal,
        super::sectors::NO_CURVATURE(),
        band,
    )
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
/// - several mentions of one solid ⇒ a **contact line** that solid
///   holds: each solid's material about the line is wedges, each of one
///   solid's against each of the other's by the edge-edge rule
///   ([`resolve_edge_edge`]). A real edge bounds a wedge between its
///   flanking faces; a face that holds the line bounds a half-turn,
///   where a bisector parts it on the ray, or, for a solid with no
///   mention on the ray, where the other's mentions were read On
///   against it. A solid with an edge on the ray folds a plane face of
///   its own holding the line into that edge's wedge ([`Folded`]), and
///   meets a curved one on its own. Two crossings on the ray refuse
///   typed ([`BooleanError::CrossingsAlongAContactLine`]), whether two
///   wedge pairs cross or one composed material's boundary crosses
///   twice, except on a solid's own slit or ring line
///   ([`crate::sector_shape::SlitTips`]). There the slit's two edges are two
///   wedges lying apart, so the material sectors decide each germ: one
///   per wedge pair that crosses, a wedge crossing two of the other's
///   holding each on its flanker inside the wedge it crosses
///   ([`onto_inside`]), so the two lie apart along the line on their
///   own sides. Where the result keeps only the slit's face there, on
///   both sides, the zip glues the face's halves of the two edges into
///   one edge and the output stage closes the face across it
///   (`slit::close_struts`).
///
/// `held` is each solid's face through the point that the other's
/// vertex touches ([`HeldFace`]), read with every edge-edge meeting of
/// that solid's wedges, as a folded face is.
///
/// A group is every mention the first one reads on its ray; three or
/// more must read so pairwise too, or the event refuses.
///
/// Each germ of the event is marked on ONE deterministic record (a
/// slit's line holds one per crossing wedge pair); every other
/// surviving record carrying an On in the event is cancelled (unless it
/// was itself germ-marked by an earlier event).
///
/// An edge-edge germ along an edge in `reversed` goes on that solid's
/// other flanker, folding the edge into the run on its Out side
/// ([`Reversed`]).
///
/// An edge-edge event that reads a touch ([`EdgeEdge::Touch`]) puts
/// its two half-edges at this vertex, A's then B's, on `touches`, for
/// the reduction's edge-edge row.
#[allow(clippy::too_many_arguments)]
pub(super) fn recl_edges<T: Decide>(
    records: &mut Vec<PairRecord>,
    first_read: &mut Vec<PairRecord>,
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    a_body: &Body<T>,
    b_body: &Body<T>,
    op: BooleanOp,
    declared: &super::GluedPairs<T>,
    band: Band,
    reversed: &Reversed,
    held: [Option<HeldFace<T>>; 2],
    touches: &mut Vec<[crate::entity::HalfEdgeKey; 2]>,
    tips: &[crate::sector_shape::SlitTips; 2],
) -> Result<(), BooleanError> {
    let (n_a, n_b) = (a_sectors.len(), b_sectors.len());

    // ---- Collect on-bound mentions from surviving records. ----
    #[derive(Clone, Copy)]
    struct Mention<T2: geom_core::Real> {
        a_side: bool,
        start_holder: usize,
        real: bool,
        dir: geom_core::Vec3<T2>,
        /// What lies behind the direction ([`Reach`]).
        reach: Reach<T2>,
        /// The mentioning sector's arm, which its direction's readings
        /// are metered at.
        arm: T2,
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
                secs[idx].start_edge()
            } else {
                secs[idx].end_edge()
            };
            let (dir, reach) = if at_start {
                (secs[idx].start, secs[idx].start_reach)
            } else {
                (secs[idx].end, secs[idx].end_reach)
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
                    reach,
                    arm: secs[idx].arm,
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
            // Metered at the shorter of the two mentioning sectors' arms,
            // as every other reading of two directions at a corner is.
            // No row reaches this margin with a direction the band alone
            // groups: the reduction splits a line edge at a vertex of the
            // other's lying on it, onto that vertex's own bits
            // (`split_edge_onto`), so coincident line edges reach this
            // site between the same two points, their chords bit-equal.
            let arm = mentions[i].arm.min(mentions[j].arm);
            if !used[j]
                && parallel_same(mentions[i].dir, mentions[j].dir, arm, RAY_COLLINEAR, band)?
            {
                used[j] = true;
                group.push(mentions[j]);
            }
        }
        // Each member was read against the seed alone; three or more are
        // one ray only if every pair reads so.
        for (p, &m) in group.iter().enumerate().skip(1) {
            for &n in &group[p + 1..] {
                if !parallel_same(m.dir, n.dir, m.arm.min(n.arm), RAY_COLLINEAR, band)? {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "a ray event's directions are each on the seed's ray but not \
                               on one another's",
                    });
                }
            }
        }
        let a_ms: Vec<Mention<T>> = group.iter().filter(|m| m.a_side).copied().collect();
        let b_ms: Vec<Mention<T>> = group.iter().filter(|m| !m.a_side).copied().collect();
        // The common edge's half-edge at this vertex: the start bound
        // of the start holder is the next sector's own chord.
        let bound = |secs: &[BoolSector<T>], f_s: usize| secs[(f_s + 1) % secs.len()].he;
        // Whether this ray is a solid's own slit or ring line: the
        // solid's vertex is the slit's tip ([`crate::sector_shape::SlitTips`])
        // and an edge of its on this ray bounds the face the slit lies
        // in, which only the slit's two edges do at its tip. Its refusal
        // is a construction gap ([`BooleanError::SlitLineCrossed`]): the
        // recourse `CrossingsAlongAContactLine` names, the pieces before
        // they are joined, is no lever on the kernel's own slit.
        let slit_line = |a_vertex, b_vertex| {
            let sides = [
                (Operand::A, a_vertex, &a_ms, a_sectors, a_body),
                (Operand::B, b_vertex, &b_ms, b_sectors, b_body),
            ];
            for (operand, vertex, ms, secs, body) in sides {
                let Some(face) = tips[usize::from(operand == Operand::B)].face_at(vertex) else {
                    continue;
                };
                let bounds_face = |he: crate::entity::HalfEdgeKey| {
                    [Some(he), body.mate(he)]
                        .into_iter()
                        .flatten()
                        .any(|h| body.face_of_half_edge(h) == Some(face))
                };
                if ms
                    .iter()
                    .any(|m| m.real && bounds_face(bound(secs, m.start_holder)))
                {
                    return Some(SlitLine {
                        operand,
                        vertex,
                        face,
                    });
                }
            }
            None
        };
        let on_slit = |e: BooleanError| -> BooleanError {
            let BooleanError::CrossingsAlongAContactLine { a_vertex, b_vertex } = e else {
                return e;
            };
            slit_line(a_vertex, b_vertex).map_or(e, |slit| slit.refusal(SlitLineGap::CrossedTwice))
        };
        // One wedge of each solid by the edge-edge rule
        // ([`resolve_edge_edge`]): its germ, an edge in `reversed` folded
        // to its other flanker; or, between two edges whose flanking
        // faces the op keeps, a touch. Each solid's material is its
        // wedge with a plane face of its own holding the line, held or
        // folded, but blind to its other edges' wedges: a touch's other
        // end may be a lone pair that holds no more (`touch_rows` takes
        // it only where both ends read it), and where another edge's
        // wedge swallows an edge, the carry drops it (`ops::Descendants`).
        let mut meet = |aw: Wedge, bw: Wedge, faces: [&[Folded<T>]; 2], fallback| {
            let (read, flanks) = resolve_edge_edge(
                a_sectors, b_sectors, a_body, b_body, op, declared, band, held, faces, aw, bw,
                fallback,
            )
            .map_err(&on_slit)?;
            let edge =
                |secs: &[BoolSector<T>], w: Wedge| w.edge.then(|| bound(secs, w.flankers[0]));
            let (ha, hb) = (edge(a_sectors, aw), edge(b_sectors, bw));
            let germ = match read {
                EdgeEdge::Germ(mut germ) => {
                    if ha.is_some_and(|h| reversed.a.contains(&h)) {
                        (germ.a, germ.sa) = reflank(germ.a, germ.sa, n_a);
                    }
                    if hb.is_some_and(|h| reversed.b.contains(&h)) {
                        (germ.b, germ.sb) = reflank(germ.b, germ.sb, n_b);
                    }
                    Some(germ)
                }
                EdgeEdge::Touch => {
                    if let (Some(ha), Some(hb)) = (ha, hb) {
                        touches.push([ha, hb]);
                    }
                    None
                }
                EdgeEdge::Dropped => None,
            };
            Ok::<_, BooleanError>((germ, flanks))
        };

        let germs: Vec<usize> = if a_ms.len() > 1 || b_ms.len() > 1 {
            // Each solid's material about the line is wedges lying apart,
            // so a wedge the other solid's crosses is crossed whatever
            // else lies round the line. A solid with no mention on the ray
            // holds it inside a face: the sector each of the other's
            // mentions was read On against.
            let wedges = |ms: &[Mention<T>], other: &[Mention<T>], a_side: bool| {
                let secs = if a_side { a_sectors } else { b_sectors };
                if !ms.is_empty() {
                    return Ok(ms
                        .iter()
                        .map(|m| Wedge::at_bound(secs, m.start_holder))
                        .collect());
                }
                let mut faces: Vec<Wedge> = Vec::new();
                for m in other {
                    let n = if m.a_side { n_a } else { n_b };
                    let idx = find_ref_sector(first_read, m.a_side, m.start_holder, n).ok_or(
                        BooleanError::ClassificationInvariant {
                            what: "a contact line in a face without a reference sector",
                        },
                    )?;
                    let w = Wedge::in_sector(idx);
                    if !faces.iter().any(|f| f.flankers == w.flankers) {
                        faces.push(w);
                    }
                }
                Ok::<_, BooleanError>(faces)
            };
            // Two wedges neither of which is an edge's meet along the
            // ray's edge, or the ray itself where it holds none.
            let line = group.iter().find(|m| m.real).unwrap_or(&group[0]);
            let line = (line.dir, line.reach);
            // A solid with an edge on the line folds a plane face of its
            // own holding the line into that edge's wedge ([`Folded`]); a
            // curved face is met on its own
            // (`work/tang/a-line-held-by-a-curved-touched-face-refuses-at-the-edge-edge-event.md`).
            // The face is here because its bisector was read on the
            // line's ray, or the line read On it: a definite side of its
            // plane contradicts that reading.
            let split = |ws: Vec<Wedge>,
                         secs: &[BoolSector<T>],
                         body: &Body<T>|
             -> Result<(Vec<Wedge>, Vec<Folded<T>>), BooleanError> {
                let has_edge = ws.iter().any(|w| w.edge);
                let (mut met, mut folded) = (Vec::new(), Vec::new());
                for w in ws {
                    let face = if has_edge && !w.edge {
                        Folded::of(secs, body, w)?
                    } else {
                        None
                    };
                    match face {
                        Some(face) if face.datum.holds_line(line.0, line.1, band)? => {
                            folded.push(face);
                        }
                        Some(_) => {
                            return Err(BooleanError::ClassificationInvariant {
                                what: "a face read on a contact line's ray lies off the line",
                            });
                        }
                        None => met.push(w),
                    }
                }
                Ok((met, folded))
            };
            let (a_ws, a_faces) = split(wedges(&a_ms, &b_ms, true)?, a_sectors, a_body)?;
            let (b_ws, b_faces) = split(wedges(&b_ms, &a_ms, false)?, b_sectors, b_body)?;
            let mut germs = Vec::new();
            for (i, &aw) in a_ws.iter().enumerate() {
                for (j, &bw) in b_ws.iter().enumerate() {
                    let faces: [&[Folded<T>]; 2] = [
                        if aw.edge { &a_faces } else { &[] },
                        if bw.edge { &b_faces } else { &[] },
                    ];
                    let (germ, flanks) = meet(aw, bw, faces, Some(line))?;
                    germs.extend(germ.map(|g| (g, [i, j], flanks)));
                }
            }
            let vertex = |body: &Body<T>, secs: &[BoolSector<T>]| {
                proven(&body.half_edges, secs[0].he, EntityId::HalfEdge).start
            };
            // Two germs along the ray lie apart only on a slit's line,
            // whose two edges are two wedges of material, apart.
            let slit = if germs.len() > 1 {
                let Some(slit) = slit_line(vertex(a_body, a_sectors), vertex(b_body, b_sectors))
                else {
                    return Err(BooleanError::CrossingsAlongAContactLine {
                        a_vertex: vertex(a_body, a_sectors),
                        b_vertex: vertex(b_body, b_sectors),
                    });
                };
                Some(slit)
            } else {
                None
            };
            // A wedge crossing two of the other's along the slit's line
            // holds one germ per wedge it crosses, each on its flanker
            // lying in that wedge, not the fold rule's Out flanker: the two
            // lie apart along the line, on the sides of the material each
            // crosses into. The flanker is set, not toggled, so an edge
            // `reversed` already moved there stays.
            let crossed =
                |side: usize, w: usize| germs.iter().filter(|(_, ws, _)| ws[side] == w).count();
            // A germ whose crossing is a face folded into its wedge, not
            // the wedge's own flanks, has no flanker of the wedge to sit
            // on: one material crossing twice.
            let off_wedge = || {
                slit.map_or(
                    BooleanError::ClassificationInvariant {
                        what: "a wedge crossed twice off a slit's line",
                    },
                    |slit| slit.refusal(SlitLineGap::CrossedTwice),
                )
            };
            let mut moved = Vec::with_capacity(germs.len());
            for &(mut g, [i, j], [a_in, b_in]) in &germs {
                if crossed(0, i) > 1 {
                    (g.a, g.sa) =
                        onto_inside(a_ws[i], a_in, (g.a, g.sa), n_a).ok_or_else(off_wedge)?;
                }
                if crossed(1, j) > 1 {
                    (g.b, g.sb) =
                        onto_inside(b_ws[j], b_in, (g.b, g.sb), n_b).ok_or_else(off_wedge)?;
                }
                moved.push(g);
            }
            let mut placed = Vec::with_capacity(moved.len());
            for g in moved {
                let at = EdgeGerm::Folded(g).place(records, first_read, &marked)?;
                mark_germ(&mut records[at])?;
                marked.push(at);
                placed.push(at);
            }
            placed
        } else {
            let germ = match (a_ms.first().copied(), b_ms.first().copied()) {
                (Some(am), Some(bm)) if am.real && bm.real => meet(
                    Wedge::at_bound(a_sectors, am.start_holder),
                    Wedge::at_bound(b_sectors, bm.start_holder),
                    [&[], &[]],
                    None,
                )?
                .0
                .map(|germ| EdgeGerm::Folded(germ).place(records, first_read, &marked))
                .transpose()?,
                (Some(am), bm) if am.real => resolve_edge_sector(
                    records,
                    first_read,
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
                )?
                .map(|germ| germ.place(records, first_read, &marked))
                .transpose()?,
                (am, Some(bm)) if bm.real => resolve_edge_sector(
                    records,
                    first_read,
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
                )?
                .map(|germ| germ.place(records, first_read, &marked))
                .transpose()?,
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
            germ.into_iter().collect()
        };
        // Cancel every other surviving record mentioning the event.
        let in_event = |a_side: bool, holder: usize| {
            group
                .iter()
                .any(|m| m.a_side == a_side && m.start_holder == holder)
        };
        for (j, rec) in records.iter_mut().enumerate() {
            if germs.contains(&j) || marked.contains(&j) || !rec.intersect {
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

/// **The edges along which a vertex pair's edge-edge germs fold Out.**
/// Two crossing pairs sharing a vertex may each hold a germ along one
/// of its edges: an operand's two vertices at one point (a pinch) meet
/// the other operand's vertex, whose edge runs along the pinch line.
/// [`fold_on_bound`](super::sectors::fold_on_bound) folds the edge
/// into each pair's In run, and the two In runs lie on either side of
/// it, so the two pairs' cuts interleave around the shared vertex. The
/// pinch's pieces meet only along the line, so the edge's Out side is
/// one run for both: folded Out, each pair cuts on its own side of the
/// edge.
///
/// A section segment along the edge puts its two ends' null edges into
/// one face only if the fold is reversed at both ends, and each end is
/// a v-v site of the edge's vertex with both pieces' vertices. An end
/// at a vertex of the pinch is one already. An end inside the pinch's
/// coincident edges, where the edge's own vertex meets them, or inside
/// the edge where a pinch vertex meets it, starts as a vertex-on-edge
/// event, and the reduction splits the edges there before this lane
/// reads `contacts.vv`, so that end too pairs one vertex with both
/// pieces' (the witness's top end does: its prism corner meets the
/// pinch edges' interiors). The pieces' membership about the edge
/// reads the same along it, so both ends hold the same two crossing
/// germs, and both reverse.
#[derive(Clone, Debug, Default)]
pub(super) struct Reversed {
    /// Bound half-edges in A's orbit (each the end bound's `he` of its
    /// after-flanker).
    pub a: Vec<crate::entity::HalfEdgeKey>,
    /// Bound half-edges in B's orbit.
    pub b: Vec<crate::entity::HalfEdgeKey>,
}

/// The bound half-edge, in `a_side`'s orbit, of a germ record (codes as
/// first read) that runs along an edge of both solids: the edge's half
/// at that vertex.
pub(super) fn edge_edge_bound<T: geom_core::Real>(
    read: &PairRecord,
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    a_side: bool,
) -> Option<crate::entity::HalfEdgeKey> {
    let bound = |secs: &[BoolSector<T>], idx: usize, codes| match on_bound(codes)? {
        true => secs[idx]
            .start_edge()
            .then(|| secs[(idx + 1) % secs.len()].he),
        false => secs[idx].end_edge().then_some(secs[idx].he),
    };
    let a = bound(a_sectors, read.a, read.sa)?;
    let b = bound(b_sectors, read.b, read.sb)?;
    Some(if a_side { a } else { b })
}

/// **A solid's own slit or ring line through a vertex pair**
/// ([`recl_edges`]): the solid, its tip there ([`crate::sector_shape::SlitTips`])
/// and the face the slit lies in.
#[derive(Clone, Copy, Debug)]
struct SlitLine {
    operand: Operand,
    vertex: crate::entity::VertexKey,
    face: crate::entity::FaceKey,
}

impl SlitLine {
    /// The refusal where the line lacks `gap`.
    const fn refusal(self, gap: SlitLineGap) -> BooleanError {
        BooleanError::SlitLineCrossed {
            operand: self.operand,
            vertex: self.vertex,
            face: self.face,
            gap,
        }
    }
}

/// `inside`'s side code: a flank in the other's material reads `In`.
const fn side_of(inside: bool) -> SideCode {
    if inside { SideCode::In } else { SideCode::Out }
}

/// Whether `op` keeps `operand`'s flank that lies `inside` the other's
/// material or not: its side is the operand's kept side.
fn kept_by(op: BooleanOp, operand: Operand, inside: bool) -> bool {
    side_of(inside) == super::finish::kept_side(op, operand)
}

/// A germ's flanker `at` (with its codes as read) set onto the flanker
/// of wedge `w` that lies inside the other's wedge (`inside`, per flank),
/// moved across the edge ([`reflank`]) where the fold rule put it on the
/// other. `None` where `at` is neither of `w`'s flankers (a crossing of
/// a face folded into it) or `w` is no edge's.
fn onto_inside(
    w: Wedge,
    inside: [bool; 2],
    at: (usize, (SideCode, SideCode)),
    n: usize,
) -> Option<(usize, (SideCode, SideCode))> {
    if !w.edge || inside[0] == inside[1] || !w.flankers.contains(&at.0) {
        return None;
    }
    let target = w.flankers[usize::from(inside[1])];
    if at.0 == target {
        return Some(at);
    }
    let moved = reflank(at.0, at.1, n);
    (moved.0 == target).then_some(moved)
}

/// An edge-edge germ moved to its edge's other flanker, the edge read
/// On at the bound that flanker holds and its other bound reading the
/// opposite key, so [`mark_germ`] folds it to the same transition.
fn reflank(idx: usize, codes: (SideCode, SideCode), n: usize) -> (usize, (SideCode, SideCode)) {
    match codes {
        (SideCode::On, k) => ((idx + 1) % n, (k.opposite(), SideCode::On)),
        (k, _) => ((idx + n - 1) % n, (SideCode::On, k.opposite())),
    }
}

/// Each flank of an edge-edge event's two wedges read against the
/// other's material, A's wedge's then B's ([`resolve_edge_edge`]).
pub(super) type FlankRead = [[bool; 2]; 2];

/// What an edge-edge event reads ([`resolve_edge_edge`]).
#[derive(Clone, Copy, Debug)]
pub(super) enum EdgeEdge {
    /// The boundaries cross along the common line: its germ, on the
    /// fold flanker's record with the edge On at its bound.
    Germ(PairRecord),
    /// No germ, and the op keeps all four flanking faces: both edges
    /// stay on the result's boundary, the result's material there two
    /// wedges touching along them.
    Touch,
    /// No germ, and the op drops a flanking face, so one edge leaves the
    /// result's boundary there.
    Dropped,
}

/// Where an on-edge event's germ goes.
#[derive(Clone, Copy, Debug)]
pub(super) enum EdgeGerm {
    /// On this flanking record as it stands.
    Held(usize),
    /// On the record of this sector pair with these codes as read: the
    /// fold flanker, the edge On at its bound.
    Folded(PairRecord),
}

impl EdgeGerm {
    /// The index of the record the germ goes on ([`place_germ`]).
    fn place(
        self,
        records: &mut Vec<PairRecord>,
        first_read: &mut Vec<PairRecord>,
        marked: &[usize],
    ) -> Result<usize, BooleanError> {
        match self {
            Self::Held(g) => Ok(g),
            Self::Folded(germ) => place_germ(records, first_read, marked, germ),
        }
    }
}

/// Puts a folded germ on the record of its sector pair, minting the
/// record when `pair_search` met no such pair, with its codes as read
/// (the edge On at its bound) in both arrays; `mark_germ` folds them.
/// Returns the record's index.
///
/// An existing record of the pair is overwritten only when no earlier
/// event marked it its germ: a record that crosses along one ray of
/// the pair's two planes crosses along no other, so its codes then are
/// this event's On codes (cancelled below whatever they read) or a
/// lump with nothing to cross. An edge-sector germ is minted on the
/// pair of its fold flanker and the reference itself, and only where no
/// record of the fold flanker with the reference or any of its
/// [`twins`] carries an On code: a record of the minted pair that exists
/// has had its codes resolved, by 15.10 (a lump with nothing to cross)
/// or by an earlier event's germ. A record an earlier event already
/// holds as its germ would lose that germ, which is refused, never
/// overwritten.
fn place_germ(
    records: &mut Vec<PairRecord>,
    first_read: &mut Vec<PairRecord>,
    marked: &[usize],
    germ: PairRecord,
) -> Result<usize, BooleanError> {
    match records.iter().position(|r| r.a == germ.a && r.b == germ.b) {
        Some(g) if marked.contains(&g) => Err(BooleanError::ClassificationInvariant {
            what: "a germ's sector pair already holds another event's germ",
        }),
        Some(g) => {
            records[g] = germ;
            first_read[g] = germ;
            Ok(g)
        }
        None => {
            records.push(germ);
            first_read.push(germ);
            Ok(records.len() - 1)
        }
    }
}

/// A flanker's representative direction and the reach behind it.
type Rep<T> = (Vec3<T>, Reach<T>);

/// A wedge's two flankers with their representatives.
type Flanks<T> = [(usize, Rep<T>); 2];

/// A boundary of a solid's about a line that crosses the other solid's
/// material: its two flankers, and which of them lie in it.
type Crossing<T> = (Flanks<T>, [bool; 2]);

/// A solid's own face holding the line beside its edge's wedge
/// ([`Folded`]), as [`resolve_edge_edge`] reads it: its half-turn's
/// flanks, the side of it the wedge lies on, and whether its rays bound
/// the composed material.
struct Part<T: geom_core::Real> {
    fl: Flanks<T>,
    link: SideCode,
    bounding: bool,
}

/// **A flanker's representative**: its noncoplanar bound `v` projected
/// perpendicular to the unit common line `axis`, normalized only once its
/// length decides positive at the bound's own reach
/// (`bool_flank_offset`) — the bound's offset off the line where
/// [`side_code`] reads it: a line edge's far vertex, a curved edge's
/// extent, a bisector's arm ([`Reach::length`]), in metres. A bound
/// within the band of the line, or on it, has no side of the line to
/// name: it refuses with the decided margin, as the wedge's extent
/// does.
fn flank_rep<T: Decide>(
    axis: Vec3<T>,
    v: Vec3<T>,
    reach: Reach<T>,
    band: Band,
) -> Result<Rep<T>, BooleanError> {
    let off = v - axis * v.dot(axis);
    crate::validate::decide_positive(
        "bool_flank_offset",
        Margin::levered(off.norm(), reach.length()),
        band,
    )
    .map_err(|diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag))?;
    Ok((off.normalize(), reach))
}

/// **Whether a solid's dihedral wedge about the common line is
/// reflex**: whether its second flanker's representative lies on the
/// outer side of its first flanker's plane (`bool_wedge_reflex`), the
/// sine of the dihedral angle levered at `arm` — its second face's
/// distance at that arm from the first face's plane. A convex wedge's
/// material is the intersection of its flanking half-spaces, a reflex
/// one's their union. A decided zero (a wedge of a half-turn, or of
/// none) refuses: a representative that splits the two planes' verdicts
/// there has nothing to read it by. [`resolve_edge_edge`]'s membership
/// answers a representative lying in a half-turn's plane by the flank
/// it lies along, before asking here.
fn wedge_is_reflex<T: Decide>(
    secs: &[BoolSector<T>],
    fl: &[(usize, Rep<T>); 2],
    arm: T,
    band: Band,
) -> Result<bool, BooleanError> {
    let ((first, _), (_, (second, _))) = (fl[0], fl[1]);
    Ok(crate::validate::decide_nonzero(
        "bool_wedge_reflex",
        Margin::levered(second.dot(secs[first].normal.vec()), arm),
        band,
    )
    .map_err(|diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag))?
        == NonzeroSign::Positive)
}

/// One flanking plane's reading of a representative
/// ([`resolve_edge_edge`]).
#[derive(Clone, Copy)]
enum Half {
    /// Off the plane: whether on its material side.
    Off(bool),
    /// On the plane, along the flank's own representative: whether the
    /// coplanar verdict puts it in the material.
    Along(bool),
    /// On the plane, pointing away from the flank: touching it.
    Away,
}

impl Half {
    /// The half-space verdict: a touch is not inside.
    const fn inside(self) -> bool {
        match self {
            Self::Off(inside) | Self::Along(inside) => inside,
            Self::Away => false,
        }
    }
}

/// **A face of one solid's through a vertex pair's point**: the face
/// the other solid's vertex there touches (`vtxfac`'s pierce read
/// again). It carries the datum that touch was read against, and the
/// touch's vertices, which a refusal along the face names
/// ([`BooleanError::PartnerAlongFace`]).
///
/// The face is the solid's but no sector of its vertex's, since the
/// vertex lies on its own face. Where the face holds a common line
/// through the point, the solid's material about the line is its wedge
/// composed with the half-turn under the face, as with a face of its
/// own it folds ([`Folded`], [`resolve_edge_edge`]).
#[derive(Clone, Copy, Debug)]
pub(super) struct HeldFace<T: geom_core::Real> {
    /// The face's datum at the point.
    pub datum: super::vtxfac::PierceDatum<T>,
    /// The other operand, whose vertex touches the face.
    pub operand: Operand,
    /// Its touching vertex.
    pub vertex: crate::entity::VertexKey,
    /// The face.
    pub face: crate::entity::FaceKey,
    /// The face's solid's vertex at the point.
    pub partner: crate::entity::VertexKey,
}

impl<T: Decide> HeldFace<T> {
    /// Whether the face holds the common line `axis`, read off the
    /// line's own bound (`reach`) as the touch reads its partners
    /// (`PierceDatum::side`): a decided zero holds it, a definite side
    /// does not, and a reading within the band refuses typed. A curved
    /// face holding the line refuses as a partner along it does
    /// (`PierceDatum::curved_along`), so a face that holds a line is a
    /// plane.
    fn holds(&self, axis: Vec3<T>, reach: Reach<T>, band: Band) -> Result<bool, BooleanError> {
        let on = self.datum.holds_line(axis, reach, band)?;
        match self.datum.curved_along(on) {
            Some(how) => Err(self.along(how)),
            None => Ok(on),
        }
    }

    /// Whether a flank lies under the face, read as [`Self::holds`]
    /// reads the line, on the plane it holds. A flank on the face is a
    /// face of the touching vertex's lying on it, which the touch does
    /// not read.
    fn under(&self, w: Vec3<T>, reach: Reach<T>, band: Band) -> Result<bool, BooleanError> {
        match self.datum.side(w, reach, band)? {
            SideCode::In => Ok(true),
            SideCode::Out => Ok(false),
            SideCode::On => Err(self.along(super::AlongFace::Flush)),
        }
    }

    /// The refusal of a touch whose vertex is read again by its pair
    /// while the pair's own vertex touches a face of the touching
    /// solid's: two solids each holding the line in a face the other's
    /// vertex touches cross along it, and each touch's reading of the
    /// other's material is blind to the other touch.
    const fn read_twice(&self) -> BooleanError {
        BooleanError::VertexReadTwice {
            operand: self.operand,
            vertex: self.vertex,
            reads: [
                super::SectorRead::Pierce(self.face),
                super::SectorRead::Pair(self.partner),
            ],
        }
    }

    /// The refusal of a line on the face whose reading `along` names.
    const fn along(&self, along: super::AlongFace) -> BooleanError {
        BooleanError::PartnerAlongFace {
            operand: self.operand,
            vertex: self.vertex,
            face: self.face,
            partner: self.partner,
            along,
        }
    }
}

/// **A plane face of a solid's own vertex holding the line beside its
/// edge**, which [`recl_edges`] folds into that edge's wedge: the face's
/// half-turn about the line ([`Wedge`]: a bisector's, whose two sectors
/// are the face's halves) and its datum, read as a held face's is
/// ([`HeldFace`], `vtxfac::PierceDatum::side`).
#[derive(Clone, Copy, Debug)]
pub(super) struct Folded<T: geom_core::Real> {
    wedge: Wedge,
    datum: super::vtxfac::PierceDatum<T>,
}

impl<T: Decide> Folded<T> {
    /// The face of the half-turn wedge `w` (no edge's), where it lies on
    /// a plane; `None` for a curved face. A face with no carrier is an
    /// invariant, as it is to the touch's partners.
    fn of(secs: &[BoolSector<T>], body: &Body<T>, w: Wedge) -> Result<Option<Self>, BooleanError> {
        let sector = &secs[w.flankers[0]];
        let shape = super::sectors::FaceShape::of_face(
            body,
            sector.face,
            "a face holding a contact line has no carrier",
        )?;
        Ok((shape == super::sectors::FaceShape::Plane).then(|| Self {
            wedge: w,
            datum: super::vtxfac::PierceDatum {
                normal: sector.normal,
                shape,
                lever: super::sectors::PierceLever::Everywhere(super::sectors::NO_CURVATURE()),
            },
        }))
    }
}

/// **A wedge of one solid's material about a common line**
/// ([`resolve_edge_edge`]): its two flankers `[before, after]`, the first's
/// end bound and the second's start bound its flanks.
#[derive(Clone, Copy, Debug)]
pub(super) struct Wedge {
    /// The flankers.
    flankers: [usize; 2],
    /// Whether `before`'s start bound, on the line, is a real edge.
    edge: bool,
}

impl Wedge {
    /// The wedge a bound on the line bounds, `start_holder` the sector
    /// it starts: a real edge's, between the sectors either side of it;
    /// a subdivision bisector's, the half-turn of the face it parts (a
    /// bisector bounds no face, so its two sectors are one of the face's).
    pub(super) fn at_bound<T: geom_core::Real>(
        secs: &[BoolSector<T>],
        start_holder: usize,
    ) -> Self {
        Self {
            flankers: [start_holder, (start_holder + 1) % secs.len()],
            edge: secs[start_holder].start_edge(),
        }
    }

    /// The half-turn of the face whose sector `idx` the line runs
    /// inside, flanked by that sector's own two bounds.
    const fn in_sector(idx: usize) -> Self {
        Self {
            flankers: [idx, idx],
            edge: false,
        }
    }
}

/// **The common line two wedges meet along**, with the reach behind it:
/// an edge's start bound, A's where both are edges — `sector_shape`'s
/// unit direction of a chord its arm rung decided long, so the axis as
/// it stands — else `fallback`.
/// Two wedges neither of which is an edge's, with no fallback, have no
/// line to meet along: no caller passes them.
fn common_line<T: geom_core::Real>(
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    aw: Wedge,
    bw: Wedge,
    fallback: Option<Rep<T>>,
) -> Result<Rep<T>, BooleanError> {
    let bound = |s: &BoolSector<T>| (s.start, s.start_reach);
    match (aw.edge, bw.edge, fallback) {
        (true, _, _) => Ok(bound(&a_sectors[aw.flankers[0]])),
        (false, true, _) => Ok(bound(&b_sectors[bw.flankers[0]])),
        (false, false, Some(line)) => Ok(line),
        (false, false, None) => Err(BooleanError::ClassificationInvariant {
            what: "an edge-edge site's common line is an edge of neither solid",
        }),
    }
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
/// angular order.
///
/// "Inside" is read by the wedge's actual extent. A representative
/// reads one half-space verdict per flanking plane (In, or an On
/// resolved as above); two alike settle it at any wedge angle. Split
/// verdicts put it in the material iff the wedge is reflex, the union
/// of its two half-spaces rather than their intersection — which is
/// [`wedge_is_reflex`]'s question, asked only then — except where the
/// representative lies on both planes, along one flank and away from
/// the other: the wedge is a half-turn, and the flank it lies along
/// decides.
///
/// Each solid's wedge is a [`Wedge`]. One that is a face holding the
/// line, a half-turn, has both flanks on that face's plane with one
/// normal: a representative's two half-space readings agree, and one
/// lying on the plane reads `[Along, Away]`.
///
/// A solid's material about the line is its wedge composed with each
/// plane face of its own holding the line beside it: a face its vertex
/// lies on, which the other solid's vertex touches ([`HeldFace`]), or a
/// face among its vertex's own sectors beside its edge (`faces`, which
/// [`recl_edges`] folds in). Each composes by the side of it the wedge
/// lies on: out of it (a pyramid lying on a plate), a representative is
/// in the material when it is in the wedge or under the face; in it (a
/// void below the top, the wedge the void's complement), when it is in
/// both. A face is read only where it holds the line to a decided zero;
/// a line within the band of it refuses typed.
///
/// With no germ, the event is a touch where the op keeps every
/// flanking face, which leaves both edges on the result's boundary
/// ([`EdgeEdge`]).
///
/// Returns the reading with each wedge's two flanks' membership in the
/// other's material, A's then B's.
#[allow(clippy::too_many_arguments)]
pub(super) fn resolve_edge_edge<T: Decide>(
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    a_body: &Body<T>,
    b_body: &Body<T>,
    op: BooleanOp,
    declared: &super::GluedPairs<T>,
    band: Band,
    held: [Option<HeldFace<T>>; 2],
    faces: [&[Folded<T>]; 2],
    aw: Wedge,
    bw: Wedge,
    fallback: Option<Rep<T>>,
) -> Result<(EdgeEdge, FlankRead), BooleanError> {
    let (axis, axis_reach) = common_line(a_sectors, b_sectors, aw, bw, fallback)?;
    // Each solid's face holding the line, A's then B's.
    let mut holding = [None; 2];
    for (slot, face) in holding.iter_mut().zip(held) {
        if let Some(face) = face
            && face.holds(axis, axis_reach, band)?
        {
            *slot = Some(face);
        }
    }
    // Each solid holding the line in a face the other's vertex touches:
    // the two cross along the line, and each touch's reading of the
    // other's material is blind to the other touch.
    if let [Some(face), Some(_)] = holding {
        return Err(face.read_twice());
    }
    let ([fa_s, _], [fb_s, _]) = (aw.flankers, bw.flankers);
    let arm = a_sectors[fa_s].arm.min(b_sectors[fb_s].arm);
    // A line bound's side of the other solid's flanking plane is its
    // far vertex's, in metres (`side_code`); that plane holds the
    // common line, so the far vertex's side is the projected
    // direction's.
    let rep = |s: &BoolSector<T>, other_is_end: bool| {
        let (v, reach) = if other_is_end {
            (s.end, s.end_reach)
        } else {
            (s.start, s.start_reach)
        };
        flank_rep(axis, v, reach, band)
    };

    // Membership of one flanker's rep inside one wedge of the other
    // solid's.
    let in_wedge_of = |own_is_a: bool,
                       own_idx: usize,
                       (w, reach): Rep<T>,
                       other: &[(usize, Rep<T>); 2]|
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
        let mut halves = [Half::Off(true); 2];
        for (inside, &(oi, (ow, _))) in halves.iter_mut().zip(other) {
            match side_code(
                w,
                reach,
                other_secs[oi].normal,
                super::sectors::NO_CURVATURE(),
                band,
            )? {
                SideCode::In => {}
                SideCode::Out => *inside = Half::Off(false),
                SideCode::On => {
                    // On the flanking plane: overlap-tie or touch. No
                    // declaration the door verifies settles it: two
                    // planar flanks are conformal, and its screen refuses
                    // `Tangent` there, while `Rest` changes nothing here;
                    // a curved flank goes on, on either sense, to a face
                    // pair the Boolean cannot yet meet. A decided zero
                    // refuses as the in-band arm does.
                    let (own_op, other_op) = if own_is_a {
                        (super::Operand::A, super::Operand::B)
                    } else {
                        (super::Operand::B, super::Operand::A)
                    };
                    let own_sec = &own_secs[own_idx];
                    let other_sec = &other_secs[oi];
                    let same = crate::validate::decide_nonzero(
                        "bool_dir_same",
                        Margin::levered(w.dot(ow), arm),
                        band,
                    )
                    .map_err(|diag| {
                        let planar = [(own_body, own_sec.face), (other_body, other_sec.face)]
                            .iter()
                            .all(|&(body, face)| {
                                body.get_face(face)
                                    .and_then(|f| body.get_surface(f.surface))
                                    .map(geom::Surface::kind)
                                    == Some(geom::SurfaceKind::Plane)
                            });
                        let which = if planar {
                            Coincide::FlankSense
                        } else {
                            Coincide::CurvedFlankSense
                        };
                        let read = declared.read(
                            &[(own_op, own_sec.face, other_op, other_sec.face)],
                            which,
                            &[],
                        );
                        BooleanError::coincidence(which, read, diag)
                    })? == NonzeroSign::Positive;
                    if !same {
                        *inside = Half::Away;
                    } else {
                        let comparison = if own_is_a { Operand::A } else { Operand::B };
                        // Declared-`Tangent` flanking pairs (distinct
                        // carriers touching along the on-edge): the
                        // membership verdict is the second-order
                        // descent — which side the OWN sector's
                        // carrier curves to relative to the other
                        // face's material — not the carrier identity
                        // ladder (whose Same± question is ill-posed
                        // across distinct carriers).
                        let read = declared.read(
                            &[(own_op, own_sec.face, other_op, other_sec.face)],
                            Coincide::TangentSide,
                            &[],
                        );
                        let lump =
                            if read == DeclarationRead::Spent(crate::contact::GlueClass::Tangent) {
                                let surface_of = |body: &Body<T>, face| {
                                    body.get_face(face)
                                        .and_then(|f| body.get_surface(f.surface))
                                        .cloned()
                                        .ok_or(BooleanError::ClassificationInvariant {
                                            what: "declared-Tangent face lost its surface",
                                        })
                                };
                                let s_own = surface_of(own_body, own_sec.face)?;
                                let s_other = surface_of(other_body, other_sec.face)?;
                                let p = own_body.half_edge_start_point(own_sec.he).ok_or(
                                    BooleanError::ClassificationInvariant {
                                        what: "edge-edge site lost its point",
                                    },
                                )?;
                                let reach = declared.reach_of(
                                    own_op,
                                    own_sec.face,
                                    other_op,
                                    other_sec.face,
                                )?;
                                super::sectors::tangent_lump(
                                    &s_own,
                                    &s_other,
                                    reach,
                                    other_sec.normal,
                                    own_sec.normal,
                                    p,
                                    op,
                                    comparison,
                                    own_sec.face,
                                    own_sec.reached_bounds(),
                                    arm,
                                    read,
                                    band,
                                )?
                            } else {
                                let rel = require_same(
                                    own_body,
                                    own_op,
                                    own_sec,
                                    other_body,
                                    other_op,
                                    other_sec,
                                    declared,
                                    ReadOn::FirstBounds,
                                    arm,
                                    band,
                                )?;
                                eq15_3_lump(op, comparison, rel)
                            };
                        *inside = Half::Along(lump == SideCode::In);
                    }
                }
            }
        }
        Ok(match halves {
            // On both planes the rep lies in a half-turn wedge's one
            // plane, on the face it lies along: that face's coplanar
            // verdict is its membership, and the flank it points away
            // from says nothing about it.
            [Half::Along(inside), Half::Away] | [Half::Away, Half::Along(inside)] => inside,
            [h0, h1] if h0.inside() == h1.inside() => h0.inside(),
            _ => wedge_is_reflex(other_secs, other, arm, band)?,
        })
    };
    let flanks = |secs: &[BoolSector<T>], w: Wedge| -> Result<Flanks<T>, BooleanError> {
        let [f_s, f_e] = w.flankers;
        Ok([
            (f_s, rep(&secs[f_s], true)?),
            (f_e, rep(&secs[f_e], false)?),
        ])
    };
    let a_fl = flanks(a_sectors, aw)?;
    let b_fl = flanks(b_sectors, bw)?;
    // Each solid's faces holding the line beside its wedge, held or
    // folded: the side of each the wedge lies on, read off the wedge's
    // own bounds as the touch reads its partners'
    // (`vtxfac::PierceDatum::side`); for a folded face, its flanks, and
    // whether its rays bound the composed material — out of the face
    // always, in it only where the wedge is reflex, the face then
    // splitting the wedge's material.
    let mut beside: [Vec<Part<T>>; 2] = [Vec::new(), Vec::new()];
    let mut held_link = [None; 2];
    for (slot, (secs, w, fl, fs)) in [
        (a_sectors, aw, a_fl, faces[0]),
        (b_sectors, bw, b_fl, faces[1]),
    ]
    .into_iter()
    .enumerate()
    {
        let [f_s, f_e] = w.flankers;
        let bounds = [
            (secs[f_s].end, secs[f_s].end_reach),
            (secs[f_e].start, secs[f_e].start_reach),
        ];
        let link_of = |datum: super::vtxfac::PierceDatum<T>| match bounds
            .map(|(dir, reach)| datum.side(dir, reach, band))
        {
            [Ok(SideCode::In), Ok(SideCode::In)] => Ok(SideCode::In),
            [Ok(SideCode::Out), Ok(SideCode::Out)] => Ok(SideCode::Out),
            [Err(e), _] | [_, Err(e)] => Err(e),
            _ => Err(BooleanError::ClassificationInvariant {
                what: "a solid's wedge about a line lies across or on its own face holding the line",
            }),
        };
        held_link[slot] = holding[slot].map(|face| link_of(face.datum)).transpose()?;
        for face in fs {
            let link = link_of(face.datum)?;
            let bounding = link == SideCode::Out || wedge_is_reflex(secs, &fl, arm, band)?;
            beside[slot].push(Part {
                fl: flanks(secs, face.wedge)?,
                link,
                bounding,
            });
        }
        // Two faces of one solid holding one line through its vertex lie
        // in one plane (the line and the vertex's wedge fix it): with
        // opposite normals the solid touches itself face to face across
        // the line, and with one normal they are one face left unmerged.
        // Neither is a solid the door admits, and a held face's partner
        // with a face on it refuses first (`AlongFace::FaceOnFace`).
        if beside[slot].len() + usize::from(holding[slot].is_some()) > 1 {
            return Err(BooleanError::ClassificationInvariant {
                what: "two faces of one solid hold one line through its vertex",
            });
        }
    }
    // Membership of one flanker's rep in the other solid's material:
    // its wedge composed with each of its faces holding the line beside
    // it, by the side of the face the wedge lies on — out of it, the
    // wedge or under the face; in it, the wedge and under the face.
    let membership = |own_is_a: bool,
                      own_idx: usize,
                      (w, reach): Rep<T>,
                      other: &[(usize, Rep<T>); 2]|
     -> Result<bool, BooleanError> {
        let slot = usize::from(own_is_a);
        let (mut any, mut all) = (in_wedge_of(own_is_a, own_idx, (w, reach), other)?, true);
        let mut compose = |under: bool, link| match link {
            SideCode::Out => {
                any |= under;
                Ok(())
            }
            SideCode::In => {
                all &= under;
                Ok(())
            }
            SideCode::On => Err(BooleanError::ClassificationInvariant {
                what: "a face holding the line reads its solid's link on it",
            }),
        };
        for part in &beside[slot] {
            compose(
                in_wedge_of(own_is_a, own_idx, (w, reach), &part.fl)?,
                part.link,
            )?;
        }
        if let (Some(face), Some(link)) = (holding[slot], held_link[slot]) {
            compose(face.under(w, reach, band)?, link)?;
        }
        Ok(any && all)
    };
    let a_in = [
        membership(true, a_fl[0].0, a_fl[0].1, &b_fl)?,
        membership(true, a_fl[1].0, a_fl[1].1, &b_fl)?,
    ];
    let b_in = [
        membership(false, b_fl[0].0, b_fl[0].1, &a_fl)?,
        membership(false, b_fl[1].0, b_fl[1].1, &a_fl)?,
    ];
    // Each solid's boundary about the line that crosses the other's
    // material: its wedge's flanks, and each face of its own folded in
    // whose rays bound the composed material. A held face bounds none of
    // it that the other crosses: the other's vertex only touches it, so
    // its flanks lie on one side of the face.
    let crossings = |own_is_a: bool,
                     fl: Flanks<T>,
                     inside: [bool; 2],
                     other: &Flanks<T>|
     -> Result<Vec<Crossing<T>>, BooleanError> {
        let mut crossing = Vec::new();
        if inside[0] != inside[1] {
            crossing.push((fl, inside));
        }
        for part in beside[usize::from(!own_is_a)].iter().filter(|p| p.bounding) {
            let rays = [
                membership(own_is_a, part.fl[0].0, part.fl[0].1, other)?,
                membership(own_is_a, part.fl[1].0, part.fl[1].1, other)?,
            ];
            if rays[0] != rays[1] {
                crossing.push((part.fl, rays));
            }
        }
        Ok(crossing)
    };
    let flanks: FlankRead = [a_in, b_in];
    let cross_a = crossings(true, a_fl, a_in, &b_fl)?;
    let cross_b = crossings(false, b_fl, b_in, &a_fl)?;
    if cross_a.len() > 1 || cross_b.len() > 1 {
        let vertex = |body: &Body<T>, secs: &[BoolSector<T>]| {
            proven(&body.half_edges, secs[0].he, EntityId::HalfEdge).start
        };
        return Err(BooleanError::CrossingsAlongAContactLine {
            a_vertex: vertex(a_body, a_sectors),
            b_vertex: vertex(b_body, b_sectors),
        });
    }
    if cross_a.is_empty() != cross_b.is_empty() {
        return Err(BooleanError::ClassificationInvariant {
            what: "edge-edge membership disagreement between the two solids",
        });
    }
    if cross_a.is_empty() {
        // A flanking face the op keeps lies on its operand's kept side.
        let both = a_in.iter().all(|&i| kept_by(op, Operand::A, i))
            && b_in.iter().all(|&i| kept_by(op, Operand::B, i));
        let read = if both {
            EdgeEdge::Touch
        } else {
            EdgeEdge::Dropped
        };
        return Ok((read, flanks));
    }
    // The one fold rule per solid, on that solid's own membership keys:
    // the common edge joins the run `fold_on_bound` names, so the
    // crossing is recorded on the flanker `crossing_flank` picks, with
    // the edge read On at its bound — `mark_germ` then folds it. The two
    // solids decide apart, so the record is the pair of their two
    // flankers whether or not `pair_search` met that pair (two coplanar
    // flankers touching along the edge are never a record of their own).
    let pick = |fl: &[(usize, Rep<T>); 2], inside: [bool; 2]| {
        let (before, after) = (side_of(inside[0]), side_of(inside[1]));
        match crossing_flank(before, after) {
            Some(Flank::Before) => Ok((fl[0].0, (SideCode::On, before))),
            Some(Flank::After) => Ok((fl[1].0, (after, SideCode::On))),
            None => Err(BooleanError::ClassificationInvariant {
                what: "an edge-edge germ whose flankers read one side",
            }),
        }
    };
    let (a, sa) = pick(&cross_a[0].0, cross_a[0].1)?;
    let (b, sb) = pick(&cross_b[0].0, cross_b[0].1)?;
    let germ = PairRecord {
        a,
        b,
        sa,
        sb,
        intersect: true,
    };
    Ok((EdgeEdge::Germ(germ), flanks))
}

/// Edge-sector coincidence (module docs): Table II on the on-side
/// flankers vs the reference sector. `other_holder` is the other
/// solid's start-holder twin when the ray also grazes its bisector.
#[allow(clippy::too_many_arguments)]
fn resolve_edge_sector<T: Decide>(
    records: &[PairRecord],
    first_read: &[PairRecord],
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    a_body: &Body<T>,
    b_body: &Body<T>,
    op: BooleanOp,
    declared: &super::GluedPairs<T>,
    band: Band,
    a_side: bool,
    f_s: usize,
    other_holder: Option<usize>,
) -> Result<Option<EdgeGerm>, BooleanError> {
    let (own_secs, ref_secs): (&[BoolSector<T>], &[BoolSector<T>]) = if a_side {
        (a_sectors, b_sectors)
    } else {
        (b_sectors, a_sectors)
    };
    let n_own = own_secs.len();
    let f_e = (f_s + 1) % n_own;
    let ref_idx = match other_holder {
        Some(h) => h,
        // Read off the codes as first read: an edge read On decides
        // which flanker carries its crossing, so it is read against the
        // face it was read On against even where another event's germ,
        // sharing its record, has since rewritten that On (`mark_germ`
        // rewrites every On of a germ record, a second edge's on a ray
        // `parallel_same` reads apart among them).
        None => find_ref_sector(first_read, a_side, f_s, n_own).ok_or(
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
    // Table III's rule: whether the on-flanker lumps Out. A glued
    // tangency's on-flanker lumps by the side its carrier curves to
    // ([`super::sectors::tangent_lump`]); any other by its carrier
    // identity with the reference (Eq. 15.3).
    let mut lumps_out = false;
    if matches!(
        (v1, v2),
        (TableIiVerdict::Rule | TableIiVerdict::NotRule, _)
            | (_, TableIiVerdict::Rule | TableIiVerdict::NotRule)
    ) {
        let on_flanker = if k1 == SideCode::On { f_s } else { f_e };
        let own_sec = &own_secs[on_flanker];
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
        let arm = own_sec.arm.min(ref_sector.arm);
        lumps_out = if declared.declares_tangent(own_op, own_sec.face, ref_op, ref_sector.face) {
            let read = declared.read(
                &[(own_op, own_sec.face, ref_op, ref_sector.face)],
                Coincide::TangentSide,
                &[],
            );
            let surface_of = |body: &Body<T>, face| {
                body.get_face(face)
                    .and_then(|f| body.get_surface(f.surface))
                    .cloned()
                    .ok_or(BooleanError::ClassificationInvariant {
                        what: "a glued tangency's face lost its surface",
                    })
            };
            let p = own_body.half_edge_start_point(own_sec.he).ok_or(
                BooleanError::ClassificationInvariant {
                    what: "edge-sector site lost its point",
                },
            )?;
            super::sectors::tangent_lump(
                &surface_of(own_body, own_sec.face)?,
                &surface_of(ref_body, ref_sector.face)?,
                declared.reach_of(own_op, own_sec.face, ref_op, ref_sector.face)?,
                ref_sector.normal,
                own_sec.normal,
                p,
                op,
                comparison,
                own_sec.face,
                own_sec.reached_bounds(),
                arm,
                read,
                band,
            )? == SideCode::Out
        } else {
            let relation = require_same(
                own_body,
                own_op,
                own_sec,
                ref_body,
                ref_op,
                ref_sector,
                declared,
                ReadOn::FirstBounds,
                arm,
                band,
            )?;
            table_iii_rule(op, comparison, relation)
        };
    }
    // A `Yes` cell is Table II's mixed row, which names the flanker keyed
    // In. The one fold rule decides which flanker the crossing is
    // recorded on ([`super::sectors::crossing_flank`]): the on-bound
    // joins the In run, so the transition is across the flanker keyed
    // Out.
    let germ_flank = |flank: usize, verdict| match verdict {
        TableIiVerdict::Yes => match crossing_flank(k1, k2) {
            Some(Flank::Before) => f_s,
            Some(Flank::After) => f_e,
            None => flank,
        },
        _ => flank,
    };
    for (flank, verdict) in [(f_s, v1), (f_e, v2)] {
        if resolve_verdict(verdict, lumps_out) {
            let flank = germ_flank(flank, verdict);
            let pair = |r: usize| if a_side { (flank, r) } else { (r, flank) };
            let held = twins(ref_secs, ref_idx).into_iter().find_map(|r| {
                let (ra, rb) = pair(r);
                find_on_record(records, ra, rb)
            });
            if let Some(g) = held {
                return Ok(Some(EdgeGerm::Held(g)));
            }
            // The reference holds the on-bound's ray (its record named
            // the bound, or the ray runs along its bisector), and both of
            // a `Yes` cell's flankers hold the bound, so the reference
            // meets the fold flanker along it whether or not
            // `pair_search`'s common line, which swings off the bound
            // where the two planes are nearly parallel, met that pair.
            if verdict != TableIiVerdict::Yes {
                return Err(BooleanError::ClassificationInvariant {
                    what: "single-on-edge crossing without a flanking record",
                });
            }
            let (a, b) = pair(ref_idx);
            let (sa, sb) = pair_codes(&a_sectors[a], &b_sectors[b], band)?;
            return Ok(Some(EdgeGerm::Folded(PairRecord {
                a,
                b,
                sa,
                sb,
                intersect: true,
            })));
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
        // Read off the codes as they stand: a bisector's On only relays
        // its sector's side between twins, and `mark_germ` resolves a
        // grazing bisector on its germ record's other side into that
        // germ's transition, so a graze no surviving record names is
        // already resolved.
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
    // Both outer keys definitely on one side: the grazing bisector
    // cannot read Zero when K > 2 (the argument at `vtxfac`'s on-edge
    // resolution — the keys are the physical sector's own bounds, read
    // at their far vertices, and the bisector at the sector's arm), and
    // a reflex sector with both bounds on one side does cross the
    // plane. So this Zero is the ambiguity band's, and it refuses
    // rather than read "not crossed".
    // The bisector's `On` rides a record's code, which keeps no margin
    // (a coplanar pair's codes are `On` with no bound read at all), so
    // the Zero the gate rejects is the one every `On` states: within
    // `±zero`.
    if k1 == k2 && k1 != SideCode::On {
        let on = super::sectors::OnReading {
            predicate: "bool_sector_bisector_side",
            decided: geom_core::Decided {
                sign: geom_core::Sign::Zero,
                margin: geom_core::MarginDiag::enclosure(-band.zero(), band.zero()),
            },
        };
        return Err(super::sectors::bisector_zero_refusal(on, band));
    }
    let crossing = matches!(
        (k1, k2),
        (SideCode::In, SideCode::Out) | (SideCode::Out, SideCode::In)
    );
    if !crossing {
        return Ok(None);
    }
    let g = twins(if a_side { b_sectors } else { a_sectors }, ref_idx)
        .into_iter()
        .flat_map(|r| [f_s, f_e].map(|own| if a_side { (own, r) } else { (r, own) }))
        .find_map(|(ra, rb)| find_on_record(records, ra, rb));
    g.map(Some).ok_or(BooleanError::ClassificationInvariant {
        what: "bisector-graze crossing without a twin record",
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::contact::GlueClass;
    use SideCode::{In, On, Out};

    /// The wedge's extent is its second face's side of its first face's
    /// plane: a quarter-turn is convex, three quarters reflex, and a
    /// half-turn (the second face in the first face's plane) refuses.
    /// Each wedge is two sectors with their own planes, flanking the
    /// common line +z the way a site's flankers do: the first holds it
    /// as its start, the second as its end.
    #[test]
    fn a_wedge_reads_reflex_by_its_second_faces_side_and_refuses_flat() {
        use super::super::sectors::Reach;
        use geom_brep::OutwardNormal;
        use geom_core::Tol;
        let band = Band::linear(Tol::witness()).unwrap();
        let v = |c: [f64; 3]| Vec3::new(c[0], c[1], c[2]);
        let axis = v([0.0, 0.0, 1.0]);
        let sector = |start: Vec3<f64>, end: Vec3<f64>, normal: [f64; 3]| BoolSector {
            he: crate::entity::HalfEdgeKey::default(),
            start,
            end,
            start_reach: Reach::Bisector(1.0),
            end_reach: Reach::Bisector(1.0),
            face: crate::entity::FaceKey::default(),
            normal: OutwardNormal::from_chart(v(normal), true),
            arm: 1.0,
        };
        // Material lies off the first plane's +y side; `second` and its
        // outward normal bound the wedge's other side.
        let first_ray = v([1.0, 0.0, 0.0]);
        let wedge = |second: [f64; 3], second_normal: [f64; 3]| {
            let secs = [
                sector(axis, first_ray, [0.0, -1.0, 0.0]),
                sector(v(second), axis, second_normal),
            ];
            let fl = [
                (0, (secs[0].end, Reach::Bisector(1.0))),
                (1, (secs[1].start, Reach::Bisector(1.0))),
            ];
            wedge_is_reflex(&secs, &fl, 1.0, band)
        };
        assert!(
            !wedge([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]).unwrap(),
            "a quarter-turn is convex"
        );
        assert!(
            wedge([0.0, -1.0, 0.0], [1.0, 0.0, 0.0]).unwrap(),
            "three quarters are reflex"
        );
        assert!(
            matches!(
                wedge([-1.0, 0.0, 0.0], [0.0, -1.0, 0.0]),
                Err(BooleanError::Escalated {
                    decision: crate::boolean::BooleanDecision::Coincidence(Coincide::Sectors, _),
                    diag,
                }) if diag.predicate == Some("bool_wedge_reflex")
            ),
            "a half-turn refuses"
        );
    }

    /// A grazing bisector between keys read Out is the band's Zero
    /// (reachable only at K ≤ 2), and it refuses rather than read "not
    /// crossed"; between keys In and Out it is a crossing.
    #[test]
    fn a_grazing_bisector_between_one_sided_keys_refuses() {
        use super::super::sectors::Reach;
        use geom_brep::OutwardNormal;
        use geom_core::{Point3, Tol};
        let band = Band::linear(Tol::witness()).unwrap();
        let o = Point3::new(0.0, 0.0, 0.0);
        let chord = |v: [f64; 3]| Reach::Chord {
            base: o,
            far: Point3::from_array(v),
        };
        let twin = |start: Vec3<f64>, sr, end: Vec3<f64>, er| BoolSector {
            he: crate::entity::HalfEdgeKey::default(),
            start,
            end,
            start_reach: sr,
            end_reach: er,
            face: crate::entity::FaceKey::default(),
            normal: OutwardNormal::from_chart(Vec3::new(0.0, 1.0, 0.0), true),
            arm: 1.0,
        };
        let bis = Vec3::new(0.0, 0.0, -1.0);
        let twins = |far_end: [f64; 3], far_start: [f64; 3]| {
            [
                twin(
                    bis,
                    Reach::Bisector(1.0),
                    Vec3::from_array(far_end).normalize(),
                    chord(far_end),
                ),
                twin(
                    Vec3::from_array(far_start).normalize(),
                    chord(far_start),
                    bis,
                    Reach::Bisector(1.0),
                ),
            ]
        };
        let reference = [BoolSector {
            normal: OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true),
            ..twin(bis, Reach::Bisector(1.0), bis, Reach::Bisector(1.0))
        }];
        let one_sided = twins([1.0, 0.0, 0.5], [-1.0, 0.0, 0.5]);
        let bracket = geom_core::k_stats::Bracket::open();
        let got = resolve_bisector_graze(&[], &one_sided, &reference, band, Some(0), Some(0));
        let log = bracket.finish();
        let Err(BooleanError::Escalated {
            decision: crate::boolean::BooleanDecision::BisectorSide,
            diag,
        }) = got
        else {
            panic!(
                "the graze between two Out keys refuses as the bisector's own decision: {got:?}"
            );
        };
        assert_eq!(diag.predicate, Some("bool_sector_bisector_side"));
        assert_eq!(
            diag.margin.rejected_sign(),
            Some(geom_core::Sign::Zero),
            "the nonzero gate's rejection of the graze's Zero"
        );
        assert_eq!(
            diag.margin.diagnostic_f64_for_error_text(),
            geom_core::ErrorTextReading::Enclosure {
                lo: -band.zero(),
                hi: band.zero()
            },
            "the graze quotes the zero band an `On` states"
        );
        assert_eq!(
            log.escalations.last().map(|e| e.source),
            Some(diag),
            "the rejection is on the log"
        );
        let crossing = twins([1.0, 0.0, 0.5], [-1.0, 0.0, -0.5]);
        let records = [rec(0, 0, (On, Out), (Out, In))];
        assert_eq!(
            resolve_bisector_graze(&records, &crossing, &reference, band, Some(0), Some(0))
                .unwrap(),
            Some(0),
            "between keys Out and In the graze is a crossing"
        );
    }

    /// **A flanker decides its offset off the common line at its own
    /// reach.** One tilt off the line clears the band on a metre-long
    /// edge and lands in it on a short one; a bound along the line
    /// refuses with its decided margin; a clear one comes back unit and
    /// perpendicular to the line, its reach carried through.
    #[test]
    fn a_flanker_decides_its_offset_off_the_common_line_at_its_reach() {
        use crate::boolean::BooleanDecision;
        use geom_core::{Point3, Tol};
        let band = Band::linear(Tol::witness()).unwrap();
        let o = Point3::new(0.0, 0.0, 0.0);
        let axis = Vec3::new(0.0, 0.0, 1.0);
        let tilt = 1e3 * band.escalate();
        let bound = Vec3::new(tilt, 0.0, 1.0).normalize();
        let chord = |v: Vec3<f64>, len: f64| Reach::Chord {
            base: o,
            far: o + v * len,
        };
        let (w, reach) = flank_rep(axis, bound, chord(bound, 1.0), band)
            .expect("a metre-long edge stands a thousand bands off the line");
        assert!(
            (w - Vec3::new(1.0, 0.0, 0.0)).norm() < 1e-9,
            "unit, perpendicular, on the bound's side: {w:?}"
        );
        assert!(
            (reach.length() - 1.0).abs() < 1e-15,
            "the reach rides through"
        );
        let mid = (band.zero() + band.escalate()) / 2.0;
        for (v, len, in_band, label) in [
            (
                bound,
                mid / tilt,
                true,
                "the same tilt on an edge whose far end stands in band",
            ),
            (axis, 1.0, false, "a bound along the line"),
        ] {
            let err = flank_rep(axis, v, chord(v, len), band).expect_err(label);
            let BooleanError::Escalated { decision, diag } = &err else {
                panic!("{label}: an escalation: {err:?}");
            };
            assert_eq!(
                *decision,
                BooleanDecision::Coincidence(Coincide::Sectors, DeclarationRead::Moot),
                "{label}"
            );
            assert_eq!(diag.predicate, Some("bool_flank_offset"), "{label}");
            let margin = diag
                .margin
                .diagnostic_f64_for_error_text()
                .value()
                .expect("the margin rides the payload");
            if in_band {
                assert!(
                    band.zero() < margin && margin < band.escalate(),
                    "{label}: the margin {margin:e} lies in the band"
                );
            } else {
                assert_eq!(margin, 0.0, "{label}: the decided zero rides the payload");
            }
        }
    }

    /// **A bisector on the common line reads its face as a half-turn
    /// wedge.** A's face is the plane y = 0, its material y < 0, parted
    /// along +z by a subdivision bisector; B is a convex wedge about +z
    /// with a real edge there. B inside A's half-turn is dropped by a
    /// union, B outside it touches the face and is kept, and B across
    /// the face's plane crosses it, the germ on the flankers keyed Out
    /// by the one fold rule: A's 0° half and B's 150° face. Two
    /// half-turns with no line given refuse loudly.
    #[test]
    fn a_bisector_on_the_common_line_reads_its_face_as_a_half_turn() {
        use crate::boolean::{Glue, GluedPairs};
        use crate::test_support_fixtures::prism_z;
        use geom_brep::OutwardNormal;
        use geom_core::{Point3, Tol};
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let o = Point3::new(0.0, 0.0, 0.0);
        let z = Vec3::new(0.0, 0.0, 1.0);
        let ray = |deg: f64| Vec3::new(deg.to_radians().cos(), deg.to_radians().sin(), 0.0);
        let sector = |start: Vec3<f64>, end: Vec3<f64>, normal: Vec3<f64>| BoolSector {
            he: crate::entity::HalfEdgeKey::default(),
            start,
            end,
            start_reach: Reach::Chord {
                base: o,
                far: o + start,
            },
            end_reach: Reach::Chord {
                base: o,
                far: o + end,
            },
            face: crate::entity::FaceKey::default(),
            normal: OutwardNormal::from_chart(normal, true),
            arm: 1.0,
        };
        let up = Vec3::new(0.0, 1.0, 0.0);
        let mut half_turn = [sector(z, ray(0.0), up), sector(ray(180.0), z, up)];
        half_turn[0].start_reach = Reach::Bisector(1.0);
        half_turn[1].end_reach = Reach::Bisector(1.0);
        let p = prism_z::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            0.0,
            1.0,
            tol,
        );
        let declared = GluedPairs::<f64>::without_struts(&Glue::none(), Default::default());
        // B's convex wedge from `b1` to `b2` (degrees), each face's
        // outward normal turned away from it.
        let resolve = |b1: f64, b2: f64| {
            let (b1, b2) = (ray(b1), ray(b2));
            let b = [sector(z, b1, b1.cross(z)), sector(b2, z, z.cross(b2))];
            resolve_edge_edge(
                &half_turn,
                &b,
                &p.body,
                &p.body,
                BooleanOp::Union,
                &declared,
                band,
                [None, None],
                [&[], &[]],
                Wedge::at_bound(&half_turn, 0),
                Wedge::at_bound(&b, 0),
                None,
            )
            .map(|(read, _)| read)
        };
        let inside = resolve(200.0, 250.0);
        assert!(
            matches!(inside, Ok(EdgeEdge::Dropped)),
            "B within A's half-turn is dropped: {inside:?}"
        );
        let outside = resolve(30.0, 80.0);
        assert!(
            matches!(outside, Ok(EdgeEdge::Touch)),
            "B outside A's half-turn touches its face: {outside:?}"
        );
        let across = resolve(150.0, 250.0);
        assert!(
            matches!(across, Ok(EdgeEdge::Germ(PairRecord { a: 0, b: 0, .. }))),
            "B across A's face crosses it, on A's 0° half and B's 150° face: {across:?}"
        );
        // Two half-turns, given no line to meet along, have none.
        let neither = resolve_edge_edge(
            &half_turn,
            &half_turn,
            &p.body,
            &p.body,
            BooleanOp::Union,
            &declared,
            band,
            [None, None],
            [&[], &[]],
            Wedge::at_bound(&half_turn, 0),
            Wedge::at_bound(&half_turn, 0),
            None,
        )
        .map(|(read, _)| read);
        assert!(
            matches!(
                neither,
                Err(BooleanError::ClassificationInvariant {
                    what: "an edge-edge site's common line is an edge of neither solid"
                })
            ),
            "two bisectors and no line: {neither:?}"
        );
    }

    fn rec(a: usize, b: usize, sa: (SideCode, SideCode), sb: (SideCode, SideCode)) -> PairRecord {
        PairRecord {
            a,
            b,
            sa,
            sb,
            intersect: true,
        }
    }

    /// **An edge-edge membership tie offers a declaration only where one
    /// the door admits would settle it** (the second review's probe P2,
    /// adopted). Two corners along a common line `z`, each flanker's
    /// representative `x` lying exactly on the other corner's flanking
    /// plane `y = 0`, at an arm whose reading of their sense lies in the
    /// band, and at a zero-band arm (a decided zero refuses as the same
    /// question, with its margin, on the frame's escalation log):
    ///
    /// - **planar flanks** (two prisms' shared side plane): the door
    ///   verifies `Rest` there, and the tie refuses the declared pair
    ///   all the same; it refuses `Tangent` (the conformal screen). No
    ///   admitted declaration settles it, so none is offered, and the
    ///   refusal ends in the corner's lever. The flanks overlap on this
    ///   side, so a smaller tolerance leads to the door asking whether
    ///   they are one face: the tolerance their arm gives is offered to
    ///   the declared `Rest` pair alone (executed: `offer_rows`'
    ///   `flanks_along_a_short_arm` and its `_declared_rest` twin).
    /// - **curved flanks** (a cylinder resting on a plane along the
    ///   line): the door refuses `Tangent` there
    ///   (`verify_tangent_declaration`), so no declaration is offered;
    ///   either sense goes on to the curved flank, which the Boolean
    ///   cannot yet meet (executed: `offer_rows`'
    ///   `membership_along_a_curved_flank`), so no tolerance is offered
    ///   either, and the refusal ends in the corner's lever alone.
    #[test]
    fn an_edge_edge_membership_tie_offers_a_declaration_only_where_one_settles_it() {
        use super::super::sectors::Reach;
        use crate::boolean::{BooleanDecision, Glue, GluedPair, GluedPairs};
        use crate::entity::FaceKey;
        use crate::test_support_fixtures::prism_z;
        use geom_brep::OutwardNormal;
        use geom_core::k_stats::Bracket;
        use geom_core::{Point3, Tol};
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let o = Point3::new(0.0, 0.0, 0.0);
        let (x, y, z) = (
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        let sector = |start: Vec3<f64>, end: Vec3<f64>, arm: f64, face: FaceKey| BoolSector {
            he: crate::entity::HalfEdgeKey::default(),
            start,
            end,
            start_reach: Reach::Chord {
                base: o,
                far: o + start,
            },
            end_reach: Reach::Chord {
                base: o,
                far: o + end,
            },
            face,
            normal: OutwardNormal::from_chart(y, true),
            arm,
        };
        let run = |a: (&crate::body::Body<f64>, FaceKey),
                   b: (&crate::body::Body<f64>, FaceKey),
                   arm: f64,
                   class: Option<GlueClass>| {
            let corner = |face| [sector(z, x, arm, face), sector(x, z, arm, face)];
            let decls = Glue {
                pairs: class
                    .map(|class| vec![GluedPair::new(a.1, b.1, class)])
                    .unwrap_or_default(),
                ..Glue::none()
            };
            let one = crate::boolean::verify_declared_contacts(a.0, b.0, &decls, band)
                .expect("the door verifies the declaration");
            let declared = GluedPairs::build(&decls, one, a.0, b.0, Default::default(), band)
                .expect("the declared faces' extents read");
            let (sa, sb) = (corner(a.1), corner(b.1));
            resolve_edge_edge(
                &sa,
                &sb,
                a.0,
                b.0,
                BooleanOp::Union,
                &declared,
                band,
                [None, None],
                [&[], &[]],
                Wedge::at_bound(&sa, 0),
                Wedge::at_bound(&sb, 0),
                None,
            )
            .map(|(read, _)| read)
        };
        let mid = (band.zero() + band.escalate()) / 2.0;
        let arms = [
            (mid, "an in-band arm"),
            (0.5 * band.zero(), "a zero-band arm"),
        ];

        // Planar flanks: two unit prisms side by side on the plane x = 1.
        let pa = prism_z::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            0.0,
            1.0,
            tol,
        );
        let pb = prism_z::<f64>(
            &[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)],
            0.0,
            1.0,
            tol,
        );
        let (fa, fb) = (pa.side_faces[1], pb.side_faces[3]);
        assert!(
            crate::boolean::rest_pair_verdict(&pa.body, fa, &pb.body, fb, band).is_ok(),
            "the door verifies Rest on the planar flanks"
        );
        assert!(
            crate::boolean::verify_tangent_declaration(&pa.body, fa, &pb.body, fb, band).is_err(),
            "the door refuses Tangent on the planar flanks"
        );
        for (arm, label) in arms {
            for (class, read) in [
                (None, DeclarationRead::Moot),
                (
                    Some(GlueClass::Rest),
                    DeclarationRead::Spent(GlueClass::Rest),
                ),
            ] {
                let bracket = Bracket::open();
                let err =
                    run((&pa.body, fa), (&pb.body, fb), arm, class).expect_err("the tie refuses");
                let log = bracket.finish();
                let BooleanError::Escalated { decision, diag } = &err else {
                    panic!("{label}, {class:?}: an escalation: {err:?}");
                };
                assert_eq!(
                    *decision,
                    BooleanDecision::Coincidence(Coincide::FlankSense, read),
                    "{label}, {class:?}"
                );
                assert_eq!(diag.predicate, Some("bool_dir_same"), "{label}");
                assert_eq!(
                    diag.margin.diagnostic_f64_for_error_text().value(),
                    Some(arm),
                    "{label}: the margin the funnel read rides the payload"
                );
                assert!(
                    log.escalations
                        .iter()
                        .any(|e| e.source.predicate == Some("bool_dir_same")),
                    "{label}: the refusal is on the frame's escalation log"
                );
                let text = err.to_string();
                let lever = "make the edges at the corner where the two faces meet span clearly \
                             more than the tolerance";
                // The sized ending is the margin's own sentence for this
                // decision's words; the moot declaration ends in the lever.
                let ending = match class {
                    None => format!("Recourse: {lever}"),
                    Some(_) => geom_core::MarginDiag::value(arm).sized_recourse(
                        band,
                        geom_core::SizedWords {
                            lever,
                            size: "length of the corner's shorter edge",
                            passes: geom_core::SizedPass::NonZero,
                            no_size: None,
                            unreadable: geom_core::UNREADABLE_MARGIN_NOTE,
                            refused: geom_core::RefusedReading::AsShown,
                        },
                    ),
                };
                assert!(
                    !text.contains("declare") && text.ends_with(&ending),
                    "{label}, {class:?}: {text}"
                );
            }
        }

        // Curved flanks: a unit cylinder along z resting on the plane
        // y = 0, tangent along the common line.
        let (ca, fca) = face_on(geom::Surface::Cylinder {
            origin: Point3::new(0.0, 1.0, 0.0),
            axis: z,
            radius: 1.0,
            u_ref: x,
        });
        let (cb, fcb) = face_on(crate::test_support_fixtures::plane(&[o, o + x, o + z], tol));
        assert!(
            crate::boolean::verify_tangent_declaration(&ca, fca, &cb, fcb, band).is_err(),
            "the door refuses Tangent on the curved flanks"
        );
        for (arm, label) in arms {
            let err = run((&ca, fca), (&cb, fcb), arm, None).expect_err("the tie refuses");
            let BooleanError::Escalated { decision, .. } = &err else {
                panic!("{label}: an escalation: {err:?}");
            };
            assert_eq!(
                *decision,
                BooleanDecision::Coincidence(Coincide::CurvedFlankSense, DeclarationRead::Moot),
                "{label}"
            );
            assert!(
                err.to_string().ends_with(
                    "Recourse: reshape the parts so the faces along that edge are planes that \
                     only touch there"
                ),
                "{label}: {err}"
            );
        }
    }

    /// **Two bound directions at an edge event are grouped at their
    /// corners' own arm**: two corners along a common line `z`, both of
    /// whose arms are an in-band length, mention the same direction. The
    /// grouping reads their sense at those arms, where it cannot tell,
    /// and refuses as the corner's sense; metered at a metre it would read
    /// the sense and go on to the membership.
    #[test]
    fn edge_event_mentions_are_grouped_at_their_corners_arm() {
        use super::super::sectors::Reach;
        use crate::boolean::{BooleanDecision, Glue, GluedPairs};
        use geom_brep::OutwardNormal;
        use geom_core::{Point3, Tol};
        let band = Band::linear(Tol::witness()).unwrap();
        let o = Point3::new(0.0, 0.0, 0.0);
        let (x, y, z) = (
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        let arm = (band.zero() + band.escalate()) / 2.0;
        let sector = |start: Vec3<f64>, end: Vec3<f64>| BoolSector {
            he: crate::entity::HalfEdgeKey::default(),
            start,
            end,
            start_reach: Reach::Chord {
                base: o,
                far: o + start,
            },
            end_reach: Reach::Chord {
                base: o,
                far: o + end,
            },
            face: crate::entity::FaceKey::default(),
            normal: OutwardNormal::from_chart(y, true),
            arm,
        };
        let corner = [sector(z, x), sector(x, z)];
        let mut records = vec![rec(0, 0, (On, Out), (On, Out))];
        let mut first_read = records.clone();
        let declared = GluedPairs::<f64>::without_struts(&Glue::none(), Default::default());
        let body = crate::body::Body::<f64>::new();
        let err = recl_edges(
            &mut records,
            &mut first_read,
            &corner,
            &corner,
            &body,
            &body,
            BooleanOp::Union,
            &declared,
            band,
            &Reversed::default(),
            [None, None],
            &mut Vec::new(),
            &Default::default(),
        )
        .expect_err("the sense at an in-band arm refuses");
        let BooleanError::Escalated { decision, diag } = &err else {
            panic!("an escalation: {err:?}");
        };
        assert_eq!(*decision, BooleanDecision::DirectionSense, "{err}");
        assert_eq!(
            diag.margin.diagnostic_f64_for_error_text().value(),
            Some(arm),
            "the sense is metered at the corners' arm"
        );
    }

    /// **A solid with an edge and a face on one line holds the half-turn
    /// under the face as well as its edge's wedge**
    /// (`work/tang/an-edge-along-a-partners-ray-through-a-touch-has-no-record.md`).
    /// A's corner is a wedge above the plane `y = 0` about the line `z`
    /// (a pyramid lying on a plate), and A's face on that plane holds the
    /// line, its material below; B's corner is a wedge below the plane
    /// about the same line (a pyramid hanging in the plate). Read by the
    /// edges' wedges alone, each lies outside the other, so `A − B`
    /// drops B's flanks and `A ∪ B` keeps them all; under the face, B's
    /// flanks are in A, so `A − B` keeps them, a touch along the line,
    /// and `A ∪ B` swallows them. A face the line leaves at a definite
    /// slope holds nothing of it, and one the line leaves within the band
    /// refuses as the side of a bound does.
    ///
    /// Where A's link lies under the face instead (a void below the top
    /// whose ray lies on it), A's wedge is the void's complement, and its
    /// material is the wedge and under the face: B's corner in the void
    /// lies outside A, as the wedges alone read it.
    ///
    /// A flank of B's on the face is a face of the touching vertex's
    /// lying on it, and refuses `PartnerAlongFace { Flush }`: a guard
    /// only, since the touch's own reading refuses that first
    /// (`vtxfac::partner_side`, `AlongFace::Flush`). A curved face holding
    /// the line refuses `PartnerAlongFace { Curved }`.
    ///
    /// Where the face is one of A's own at its vertex, parted by a
    /// bisector on the line beside A's edge, the edge's wedge reads it
    /// as it reads the held face, so a touch along the line reads alike
    /// at its two ends (`touch_rows`). A corner of B's straddling the
    /// face's plane crosses A's material along the face's half-turn, and
    /// its germ goes on the face's flanker, where the half-turn read
    /// alone places it.
    #[test]
    fn a_solid_with_an_edge_and_a_face_on_the_line_holds_the_half_turn_under_the_face() {
        use super::super::sectors::{FaceShape, PierceLever, Reach};
        use super::super::vtxfac::PierceDatum;
        use crate::boolean::{BooleanDecision, Glue, GluedPairs};
        use geom_brep::OutwardNormal;
        use geom_core::{Point3, Tol};
        let band = Band::linear(Tol::witness()).unwrap();
        let o = Point3::new(0.0, 0.0, 0.0);
        let z = Vec3::new(0.0, 0.0, 1.0);
        let h = std::f64::consts::FRAC_1_SQRT_2;
        let (u, v) = (1.0 / 5f64.sqrt(), 2.0 / 5f64.sqrt());
        let sector = |start: Vec3<f64>, end: Vec3<f64>, normal: Vec3<f64>| BoolSector {
            he: crate::entity::HalfEdgeKey::default(),
            start,
            end,
            start_reach: Reach::Chord {
                base: o,
                far: o + start,
            },
            end_reach: Reach::Chord {
                base: o,
                far: o + end,
            },
            face: crate::entity::FaceKey::default(),
            normal: OutwardNormal::from_chart(normal, true),
            arm: 1.0,
        };
        // Each corner's two faces through the line, the wedge between
        // `p` and `q` its material.
        let corner = |p: Vec3<f64>, q: Vec3<f64>, np: Vec3<f64>, nq: Vec3<f64>| {
            [sector(z, p, np), sector(q, z, nq)]
        };
        let above = corner(
            Vec3::new(u, v, 0.0),
            Vec3::new(-u, v, 0.0),
            Vec3::new(v, -u, 0.0),
            Vec3::new(-v, -u, 0.0),
        );
        let below = corner(
            Vec3::new(h, -h, 0.0),
            Vec3::new(-h, -h, 0.0),
            Vec3::new(h, h, 0.0),
            Vec3::new(-h, h, 0.0),
        );
        // A void's complement: everything about the line but the quarter
        // turn below between `(h, -h)` and `(-h, -h)`; and a narrow wedge
        // inside the void.
        let reflex = corner(
            Vec3::new(-h, -h, 0.0),
            Vec3::new(h, -h, 0.0),
            Vec3::new(h, -h, 0.0),
            Vec3::new(-h, -h, 0.0),
        );
        let (s, c) = (0.3 / 1.09f64.sqrt(), 1.0 / 1.09f64.sqrt());
        let in_void = corner(
            Vec3::new(s, -c, 0.0),
            Vec3::new(-s, -c, 0.0),
            Vec3::new(c, s, 0.0),
            Vec3::new(-c, s, 0.0),
        );
        let declared = GluedPairs::<f64>::without_struts(&Glue::none(), Default::default());
        let body = crate::body::Body::<f64>::new();
        // A's face through the point, its outward normal `n`.
        let face = |n: Vec3<f64>| HeldFace {
            datum: PierceDatum {
                normal: OutwardNormal::from_chart(n, true),
                shape: FaceShape::Plane,
                lever: PierceLever::Everywhere(super::super::sectors::NO_CURVATURE()),
            },
            operand: Operand::B,
            vertex: crate::entity::VertexKey::default(),
            face: crate::entity::FaceKey::default(),
            partner: crate::entity::VertexKey::default(),
        };
        let meet = |a: &[BoolSector<f64>], b: &[BoolSector<f64>], op, held| {
            let mut records = vec![rec(0, 0, (On, Out), (On, Out))];
            let mut first_read = records.clone();
            let mut touches = Vec::new();
            recl_edges(
                &mut records,
                &mut first_read,
                a,
                b,
                &body,
                &body,
                op,
                &declared,
                band,
                &Reversed::default(),
                [held, None],
                &mut touches,
                &Default::default(),
            )
            .map(|()| touches.len())
        };
        let run = |op, held| meet(&above, &below, op, held);
        let flat = face(Vec3::new(0.0, 1.0, 0.0));
        for (op, wedges, under) in [(BooleanOp::Subtract, 0, 1), (BooleanOp::Union, 1, 0)] {
            assert_eq!(run(op, None).unwrap(), wedges, "{op:?} by the wedges");
            assert_eq!(run(op, Some(flat)).unwrap(), under, "{op:?} under the face");
            // Tipped so the line rises off the face at a definite slope.
            let tipped = face(Vec3::new(0.0, 0.8, 0.6));
            assert_eq!(
                run(op, Some(tipped)).unwrap(),
                wedges,
                "{op:?} off the face"
            );
        }
        let mid = (band.zero() + band.escalate()) / 2.0;
        let err = run(
            BooleanOp::Subtract,
            Some(face(Vec3::new(0.0, (1.0 - mid * mid).sqrt(), mid))),
        )
        .expect_err("a line within the band of the face refuses");
        let BooleanError::Escalated { decision, .. } = &err else {
            panic!("an escalation: {err:?}");
        };
        assert_eq!(
            *decision,
            BooleanDecision::Coincidence(Coincide::SectorSide, DeclarationRead::Moot),
            "{err}"
        );
        let along = |err: BooleanError| match err {
            BooleanError::PartnerAlongFace { along, .. } => along,
            other => panic!("a partner along the face: {other:?}"),
        };
        // B's corner below the face, one flank on it.
        let flush = corner(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-h, -h, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(-h, h, 0.0),
        );
        for op in [BooleanOp::Subtract, BooleanOp::Union] {
            let err = meet(&above, &flush, op, Some(flat)).expect_err("a flank on the face");
            assert_eq!(along(err), super::super::AlongFace::Flush, "{op:?}");
            let curved = HeldFace {
                datum: PierceDatum {
                    shape: FaceShape::Curved,
                    ..flat.datum
                },
                ..flat
            };
            let err = run(op, Some(curved)).expect_err("a curved face holding the line");
            assert_eq!(along(err), super::super::AlongFace::Curved, "{op:?}");
        }
        // The same material read where the face is one of A's own: A's
        // vertex there has its edge's wedge and the face's two halves,
        // parted by a bisector on the line. Its edge's wedge reads the
        // face with it, so the line's two ends read alike.
        let x = Vec3::new(1.0, 0.0, 0.0);
        let (plane_body, plane) = face_on(crate::test_support_fixtures::plane(
            &[o, o + x, o + z],
            Tol::witness(),
        ));
        let half = |start: Vec3<f64>, end: Vec3<f64>, bisector_first: bool| BoolSector {
            start_reach: if bisector_first {
                Reach::Bisector(1.0)
            } else {
                Reach::Chord {
                    base: o,
                    far: o + start,
                }
            },
            end_reach: if bisector_first {
                Reach::Chord {
                    base: o,
                    far: o + end,
                }
            } else {
                Reach::Bisector(1.0)
            },
            face: plane,
            ..sector(start, end, Vec3::new(0.0, 1.0, 0.0))
        };
        let own = [
            above[0].clone(),
            above[1].clone(),
            half(z, x, true),
            half(-x, z, false),
        ];
        let at_own_against = |b: &[BoolSector<f64>], op| {
            let mut records = vec![
                rec(0, 0, (On, Out), (On, Out)),
                rec(2, 0, (On, Out), (On, Out)),
            ];
            let mut first_read = records.clone();
            let mut touches = Vec::new();
            recl_edges(
                &mut records,
                &mut first_read,
                &own,
                b,
                &plane_body,
                &body,
                op,
                &declared,
                band,
                &Reversed::default(),
                [None, None],
                &mut touches,
                &Default::default(),
            )
            .map(|()| (touches.len(), records))
        };
        let at_own = |op| at_own_against(&below, op).map(|(touches, _)| touches);
        for (op, touch) in [(BooleanOp::Subtract, 1), (BooleanOp::Union, 0)] {
            assert_eq!(run(op, Some(flat)).unwrap(), touch, "{op:?} at the touch");
            assert_eq!(
                at_own(op).unwrap(),
                touch,
                "{op:?} at the face's own vertex"
            );
        }
        // B's corner straddling the face's plane about the line, over
        // 330°..30°: A's composed material's boundary crosses B's along
        // the face's half-turn, not along A's edge's wedge, and the germ
        // goes on the face's flanker, as the face's half-turn read alone
        // places it.
        let (s30, c30) = 30f64.to_radians().sin_cos();
        let straddle = corner(
            Vec3::new(c30, s30, 0.0),
            Vec3::new(c30, -s30, 0.0),
            Vec3::new(-0.5, c30, 0.0),
            Vec3::new(-0.5, -c30, 0.0),
        );
        for op in [BooleanOp::Subtract, BooleanOp::Union, BooleanOp::Intersect] {
            let (touches, records) = at_own_against(&straddle, op).unwrap();
            assert_eq!(touches, 0, "{op:?} across the face");
            assert!(
                records.iter().any(|r| r.a == 3 && r.b == 0 && r.intersect),
                "{op:?}: the germ on the face's flanker, {records:?}"
            );
        }
        // The void's corners are disjoint, under the face as by the
        // wedges.
        let top = face(Vec3::new(0.0, 1.0, 0.0));
        for (op, disjoint) in [(BooleanOp::Subtract, 0), (BooleanOp::Union, 1)] {
            for held in [None, Some(top)] {
                assert_eq!(
                    meet(&reflex, &in_void, op, held).unwrap(),
                    disjoint,
                    "{op:?} in the void, the face {held:?}"
                );
            }
        }
        // A corner over `lo°..hi°` about the line, counterclockwise.
        let wedge = |lo: f64, hi: f64| {
            let at = |deg: f64| {
                let (s, c) = deg.to_radians().sin_cos();
                Vec3::new(c, s, 0.0)
            };
            corner(at(hi), at(lo), at(hi + 90.0), at(lo - 90.0))
        };
        // Each solid holding the line in a face the other's vertex
        // touches: A's wedge over 80°..100° with its face's half-turn
        // under y = 0, B's over 120°..140° with its face's half-turn over
        // 225°..405°. The two faces cross along the line, which neither
        // touch reads: it refuses as a vertex read twice.
        let b_face = HeldFace {
            datum: PierceDatum {
                normal: OutwardNormal::from_chart(
                    Vec3::new(
                        (135f64).to_radians().cos(),
                        (135f64).to_radians().sin(),
                        0.0,
                    ),
                    true,
                ),
                ..flat.datum
            },
            operand: Operand::A,
            ..flat
        };
        let (wa, wb) = (wedge(80.0, 100.0), wedge(120.0, 140.0));
        for op in [BooleanOp::Subtract, BooleanOp::Union, BooleanOp::Intersect] {
            let mut records = vec![rec(0, 0, (On, Out), (On, Out))];
            let mut first_read = records.clone();
            let err = recl_edges(
                &mut records,
                &mut first_read,
                &wa,
                &wb,
                &body,
                &body,
                op,
                &declared,
                band,
                &Reversed::default(),
                [Some(flat), Some(b_face)],
                &mut Vec::new(),
                &Default::default(),
            )
            .expect_err("the faces cross along the line");
            assert!(
                matches!(err, BooleanError::VertexReadTwice { .. }),
                "{op:?}: {err:?}"
            );
        }
    }

    /// **Two crossings along a contact line refuse typed**: A's edge's
    /// wedge over 63°..117° above the plane `y = 0`, with a face of its
    /// own on that plane (its half-turn below) folded in; B's corner over
    /// 330°..90° crosses both the face's 0° ray and A's 63° flank, two
    /// crossings of A's material's boundary along one line, which the
    /// insertion does not place apart (`CrossingsAlongAContactLine`).
    #[test]
    fn two_crossings_of_a_composed_material_along_a_contact_line_refuse_typed() {
        use super::super::sectors::Reach;
        use crate::boolean::{Glue, GluedPairs};
        use crate::test_support_fixtures::prism_z;
        use geom_brep::OutwardNormal;
        use geom_core::{Point3, Tol};
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let o = Point3::new(0.0, 0.0, 0.0);
        let (x, z) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let (pa, pb) = (
            prism_z::<f64>(&square, 0.0, 1.0, tol),
            prism_z::<f64>(&square, 0.0, 1.0, tol),
        );
        let he = |body: &crate::body::Body<f64>| body.half_edges.keys().next().unwrap();
        let (ha, hb) = (he(&pa.body), he(&pb.body));
        let chord = |d: Vec3<f64>| Reach::Chord {
            base: o,
            far: o + d,
        };
        let sector = |he, start: Vec3<f64>, end: Vec3<f64>, normal: Vec3<f64>, face| BoolSector {
            he,
            start,
            end,
            start_reach: chord(start),
            end_reach: chord(end),
            face,
            normal: OutwardNormal::from_chart(normal, true),
            arm: 1.0,
        };
        let at = |deg: f64| {
            let (s, c) = deg.to_radians().sin_cos();
            Vec3::new(c, s, 0.0)
        };
        let none = crate::entity::FaceKey::default();
        let (lo, hi) = (63.0, 117.0);
        let top = pa.side_faces[0];
        let y = Vec3::new(0.0, 1.0, 0.0);
        let own = [
            sector(ha, z, at(lo), at(lo - 90.0), none),
            sector(ha, at(hi), z, at(hi + 90.0), none),
            BoolSector {
                start_reach: Reach::Bisector(1.0),
                ..sector(ha, z, x, y, top)
            },
            BoolSector {
                end_reach: Reach::Bisector(1.0),
                ..sector(ha, -x, z, y, top)
            },
        ];
        let b = [
            sector(hb, z, at(90.0), at(180.0), none),
            sector(hb, at(330.0), z, at(240.0), none),
        ];
        let declared = GluedPairs::<f64>::without_struts(&Glue::none(), Default::default());
        for op in [BooleanOp::Subtract, BooleanOp::Union, BooleanOp::Intersect] {
            let mut records = vec![
                rec(0, 0, (On, Out), (On, Out)),
                rec(2, 0, (On, Out), (On, Out)),
            ];
            let mut first_read = records.clone();
            let err = recl_edges(
                &mut records,
                &mut first_read,
                &own,
                &b,
                &pa.body,
                &pb.body,
                op,
                &declared,
                band,
                &Reversed::default(),
                [None, None],
                &mut Vec::new(),
                &Default::default(),
            )
            .expect_err("two crossings along the line");
            let vertex = |body: &crate::body::Body<f64>, h| body.get_half_edge(h).unwrap().start;
            let want = (vertex(&pa.body, ha), vertex(&pb.body, hb));
            assert!(
                matches!(
                    err,
                    BooleanError::CrossingsAlongAContactLine { a_vertex, b_vertex }
                        if (a_vertex, b_vertex) == want
                ),
                "{op:?}: {err:?}"
            );
            // The same vertex as a slit's tip: where the ray's edge bounds
            // the face the slit lies in, the line is the slit's and the
            // refusal is the slit's construction gap; where the slit lies
            // in another face, the line is not the slit's and keeps its
            // lever.
            let slit_face = pa
                .body
                .face_of_half_edge(ha)
                .expect("the ray's edge bounds a face");
            let other_face = pa
                .body
                .faces()
                .map(|(f, _)| f)
                .find(|&f| {
                    f != slit_face
                        && pa.body.mate(ha).and_then(|m| pa.body.face_of_half_edge(m)) != Some(f)
                })
                .expect("a face the ray's edge does not bound");
            for (face, slit) in [(slit_face, true), (other_face, false)] {
                let tips = [
                    crate::sector_shape::SlitTips::of_pairs([(want.0, face)]),
                    crate::sector_shape::SlitTips::default(),
                ];
                let mut records = vec![
                    rec(0, 0, (On, Out), (On, Out)),
                    rec(2, 0, (On, Out), (On, Out)),
                ];
                let mut first_read = records.clone();
                let err = recl_edges(
                    &mut records,
                    &mut first_read,
                    &own,
                    &b,
                    &pa.body,
                    &pb.body,
                    op,
                    &declared,
                    band,
                    &Reversed::default(),
                    [None, None],
                    &mut Vec::new(),
                    &tips,
                )
                .expect_err("two crossings along the line");
                let ok = if slit {
                    matches!(
                        err,
                        BooleanError::SlitLineCrossed {
                            operand: Operand::A,
                            vertex,
                            face: f,
                            gap: SlitLineGap::CrossedTwice,
                        } if vertex == want.0 && f == face
                    )
                } else {
                    matches!(err, BooleanError::CrossingsAlongAContactLine { .. })
                };
                assert!(ok, "{op:?}, the slit in {face:?}: {err:?}");
            }
        }
    }

    /// A body whose one skeletal face carries `surface`.
    fn face_on(surface: geom::Surface<f64>) -> (crate::body::Body<f64>, crate::entity::FaceKey) {
        let st = crate::fixtures::mvfs_state();
        let mut body = st.body;
        body.set_face_surface(
            st.face,
            crate::euler::FaceSurface::New {
                surface,
                sense: true,
            },
        )
        .expect("a skeletal face takes any surface");
        (body, st.face)
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

    /// A sector's twins lie across its bisector bounds: the next entry
    /// across a bisector start, the previous across a bisector end,
    /// wrapping round the orbit, and none across a real edge.
    #[test]
    fn twins_lie_across_the_bisector_bounds() {
        use super::super::sectors::Reach;
        let sector = |start_bisector: bool, end_bisector: bool| {
            let reach = |bisector| {
                if bisector {
                    Reach::Bisector(1.0)
                } else {
                    Reach::Extent(1.0)
                }
            };
            BoolSector {
                he: crate::entity::HalfEdgeKey::default(),
                start: Vec3::new(1.0, 0.0, 0.0),
                end: Vec3::new(0.0, 1.0, 0.0),
                start_reach: reach(start_bisector),
                end_reach: reach(end_bisector),
                face: crate::entity::FaceKey::default(),
                normal: geom_brep::OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true),
                arm: 1.0,
            }
        };
        // Two halved sectors, one split across the wrap.
        let halves = [
            sector(false, true),
            sector(true, false),
            sector(false, true),
            sector(true, false),
        ];
        let got: Vec<Vec<usize>> = (0..4).map(|i| twins(&halves, i)).collect();
        assert_eq!(got, [vec![0, 3], vec![1, 2], vec![2, 1], vec![3, 0]]);
        // A sector split in three: the middle entry has a twin each way.
        let thirds = [sector(true, false), sector(true, true), sector(false, true)];
        assert_eq!(twins(&thirds, 1), [1, 2, 0]);
        assert_eq!(twins(&thirds, 0), [0, 1]);
        assert_eq!(twins(&thirds, 2), [2, 1]);
        // No bisector bound: no twin.
        assert_eq!(twins(&[sector(false, false), sector(false, false)], 0), [0]);
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
