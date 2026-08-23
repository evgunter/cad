//! Vertex-vertex classification, part 1: the typed sector arrays and
//! the all-pairs intersecting-sector search (Programs 15.7–15.9
//! re-derived under OUR conventions — nothing sign-copied).
//!
//! # Sector representation (derived, PR 2's geometry reused)
//!
//! The orbit visits half-edges leaving the base vertex CW-from-outside;
//! the sector after orbit edge `he_i` (the corner of
//! `face(loop(mate(he_i)))`) sweeps CCW around that face's outward
//! normal **from `dir(he_{i+1})` to `dir(he_i)`**. We store each sector
//! with explicit `start`/`end` bound VECTORS (start = the CCW-first
//! bound = the next orbit chord), so `sectors[k].start ==
//! sectors[k+1].end` — the shared-bound chain the reclassification
//! neighbor propagation walks. Wide (≥ 180°) sectors are convexly
//! subdivided at an interior direction (PR 2's derivation: the cone
//! argument needs < 180°), pushed as two chained entries. The arm /
//! wideness / subdivision-direction rungs themselves are
//! [`crate::sector_shape`] — one implementation, shared with the
//! splitting lane, called here under this lane's K names.
//!
//! # Side codes (the F3 chain — the 15.7 sign resolution)
//!
//! A bound direction's code against the other sector's face is
//! [`geom_brep::enters_material`]: `Enters ⇒ In`, `Exits ⇒ Out`,
//! `Tangent ⇒ On`. Program 15.7's printed `IN = +1` (positive dot ⇒ IN)
//! is coherent only for inward normals; under TOG §2's (and our)
//! outward normals the derived mapping is the opposite — mirror-pinned
//! by `mirror_check_side_codes`.
//!
//! # Intersection test (15.9 re-derived)
//!
//! Pair (a, b) intersects iff the face-plane intersection direction
//! `±(n_a × n_b)` lies within both (convex) sectors — `within` =
//! both boundary triples `(start × dir)·n`, `(dir × end)·n` not
//! definitely negative (a Zero graze counts as within: boundary hits
//! are exactly what must flow into the ON machinery). Coplanar pairs
//! (`n_a × n_b` ≈ 0) go to `sector_overlap` — the unprinted procedure,
//! designed here: overlap iff some bound of one lies strictly within
//! the other, or the two sectors' bounds are pairwise parallel (the
//! identical / identical-reversed region cases); touch-only sharing of
//! a single bound is NOT overlap.

use geom_brep::{EntersMaterial, OutwardNormal, enters_material};
use geom_core::{Band, Decide, Margin, Sign, Vec3};

use super::{BooleanError, Operand, SideCode};
use crate::body::Body;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::sector_face::{SectorCarrier, SectorFaceError};
use crate::sector_shape::{SectorShape, sector_shape};
use crate::validate::decide;

/// One (convex) sector of a vertex neighborhood.
#[derive(Clone, Debug)]
pub(super) struct BoolSector<T: geom_core::Real> {
    /// The orbit half-edge the sector follows (CW-after `he`).
    pub he: HalfEdgeKey,
    /// CCW-first bound direction (the next orbit chord, or a bisector).
    pub start: Vec3<T>,
    /// CCW-last bound direction (this entry's own chord, or a bisector).
    pub end: Vec3<T>,
    /// Whether `start` is a real edge chord (false: subdivision bisector).
    pub start_edge: bool,
    /// Whether `end` is a real edge chord.
    pub end_edge: bool,
    /// The sector's face and outward normal.
    pub face: FaceKey,
    /// The face's outward unit normal at the base vertex, minted once
    /// in [`sector_face`] from the CHART normal and the face's `sense`
    /// bit (S10). Consumers pair it with the stored orbit order and
    /// must NOT re-apply the sense — the type says the sense is in.
    pub normal: OutwardNormal<T>,
    /// The metering arm (shorter bounding chord, in meters).
    pub arm: T,
}

fn corrupt(operand: Operand, vertex: VertexKey) -> BooleanError {
    BooleanError::CorruptOperand { operand, vertex }
}

/// Builds the sector array of `vertex`'s neighborhood (module docs).
pub(super) fn build_sectors<T: Decide>(
    body: &Body<T>,
    operand: Operand,
    vertex: VertexKey,
    band: Band,
) -> Result<Vec<BoolSector<T>>, BooleanError> {
    let anchor = body
        .get_vertex(vertex)
        .and_then(|v| v.emanating)
        .ok_or_else(|| corrupt(operand, vertex))?;
    let orbit = body
        .vertex_orbit(anchor)
        .ok_or_else(|| corrupt(operand, vertex))?;
    // The outgoing direction of an orbit half-edge, scaled to the
    // edge's honest extent — the M3 chord for `Line` carriers
    // (bit-identical), the carrier's outgoing TANGENT at the base
    // vertex scaled by `edge_extent` for conic carriers (M5 PR 9: the
    // ON-set machinery consumes curved carrier tangents instead of
    // assuming straight edges — the splitting lane's C12.2 idiom).
    let chord = |he: HalfEdgeKey| -> Result<Vec3<T>, BooleanError> {
        let end = body
            .half_edge_end(he)
            .ok_or_else(|| corrupt(operand, vertex))?;
        let p_base = *body
            .get_point(
                body.get_vertex(vertex)
                    .ok_or_else(|| corrupt(operand, vertex))?
                    .point,
            )
            .ok_or_else(|| corrupt(operand, vertex))?;
        let p_end = *body
            .get_point(
                body.get_vertex(end)
                    .ok_or_else(|| corrupt(operand, vertex))?
                    .point,
            )
            .ok_or_else(|| corrupt(operand, vertex))?;
        let he_data = body
            .get_half_edge(he)
            .ok_or_else(|| corrupt(operand, vertex))?;
        let edge = body
            .get_edge(he_data.edge)
            .ok_or_else(|| corrupt(operand, vertex))?;
        let curve = body
            .get_curve_geom(edge.curve)
            .and_then(crate::null::CurveGeom::certified)
            .ok_or_else(|| corrupt(operand, vertex))?;
        match curve.carrier() {
            geom::Curve3::Line { .. } | geom::Curve3::Nurbs(_) => Ok(p_end - p_base),
            geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {
                let (t0, t1) = curve.params();
                let tangent = if he == edge.he_plus {
                    curve.carrier().deriv(t0)
                } else {
                    -curve.carrier().deriv(t1)
                };
                let extent =
                    geom_brep::edge_extent(curve.carrier(), t0, t1, p_end.distance(p_base));
                Ok(tangent.normalize() * extent)
            }
        }
    };
    let mut sectors = Vec::with_capacity(orbit.len() + 2);
    for (i, &he) in orbit.iter().enumerate() {
        let next_he = orbit[(i + 1) % orbit.len()];
        let dir_end = chord(he)?; // this entry's own chord = CCW-last
        let dir_start = chord(next_he)?; // next chord = CCW-first
        let (face, normal) = sector_face(body, operand, vertex, he)?;
        // The three sector-shape rungs — metering arm, wideness, and
        // the subdivision direction (PR 2's derivation: the cone
        // argument needs < 180°) — are [`crate::sector_shape`]: ONE
        // implementation, called from here and from the splitting
        // lane's neighborhood walk, under the one pooled set of K names
        // (smell scan S5; #652 pooled them). This is a call, not a
        // copy.
        //
        // The sense-invariance argument for the `normal` passed here is
        // NOT restated: it is the contract of `sector_shape`'s `normal`
        // parameter, which is the one place a caller has to read it.
        // The value arrives typed, so it cannot be the wrong one.
        let SectorShape {
            arm,
            unit_own: u_end,
            unit_next: u_start,
            bisector: bisec,
        } = sector_shape(dir_end, dir_start, normal, he == next_he, band)
            .map_err(|diag| BooleanError::Escalated { diag })?;
        match bisec {
            None => sectors.push(BoolSector {
                he,
                start: u_start,
                end: u_end,
                start_edge: true,
                end_edge: true,
                face,
                normal,
                arm,
            }),
            Some(b) => {
                // Chained order (module docs): the end-sharing half
                // first, then the start-sharing half.
                sectors.push(BoolSector {
                    he,
                    start: b,
                    end: u_end,
                    start_edge: false,
                    end_edge: true,
                    face,
                    normal,
                    arm,
                });
                sectors.push(BoolSector {
                    he,
                    start: u_start,
                    end: b,
                    start_edge: true,
                    end_edge: false,
                    face,
                    normal,
                    arm,
                });
            }
        }
    }
    Ok(sectors)
}

/// The sector's face + outward normal at the base vertex.
///
/// The walk and the normals are [`crate::sector_face`] — ONE
/// implementation, called from here and from the splitting lane's
/// sector walk (smell scan S5). What stays here is this lane's
/// adaptation of it, and only that: the boolean error type, whose
/// every arm carries the [`Operand`] the shared walk has no notion of.
/// All three wired arms — `Plane`, `Cylinder`, `Sphere` (M5 PR 9) —
/// are live on this side; kinds without one refuse typed (C12.1, per
/// arm).
///
/// The normal arrives as an [`OutwardNormal`] with the face's `sense`
/// folded in (S10), minted at the shared chokepoint — which makes that
/// chokepoint the source for the whole vertex-vertex lane. Everything
/// downstream of [`BoolSector::normal`] — `within`, `side_code`,
/// `sector_overlap`, the wideness/bisector algebra above,
/// `insert::germ_dir`, `vtxfac::pierce_germ_dir` — is sense-invariant
/// GIVEN this source and must not multiply again: those sites pair the
/// normal with the STORED orbit/loop traversal, which `revert` flips in
/// the same breath as the sense bit, so a second factor would cancel
/// the first and re-break what this fixes.
pub(super) fn sector_face<T: Decide>(
    body: &Body<T>,
    operand: Operand,
    vertex: VertexKey,
    he: HalfEdgeKey,
) -> Result<(FaceKey, OutwardNormal<T>), BooleanError> {
    let resolved = crate::sector_face::resolve(body, vertex, he).map_err(|e| match e {
        // The shared walk names the entity that did not resolve; this
        // lane's corruption arm carries the operand and a VERTEX, so
        // the payload is narrowed here the same way the splitting
        // lane's is — a vertex names itself, anything else falls back
        // to the base vertex (issue #695).
        SectorFaceError::Corrupt(EntityId::Vertex(v)) => corrupt(operand, v),
        SectorFaceError::Corrupt(_) => corrupt(operand, vertex),
        SectorFaceError::Unsupported { face, kind } => BooleanError::CurvedBooleanUnsupported {
            operand,
            face,
            kind,
        },
    })?;
    // Exhaustive on purpose, exactly as the splitting wrapper is: a
    // fifth carrier arm added to the shared walk must be a compile
    // error in BOTH lanes, not silently accepted by the one whose
    // downstream algebra happens not to read the carrier.
    match resolved.carrier {
        SectorCarrier::Plane | SectorCarrier::Cylinder | SectorCarrier::Sphere => {}
    }
    Ok((resolved.face, resolved.normal))
}

fn invalid_escalation(band: Band, predicate: &'static str) -> BooleanError {
    BooleanError::Escalated {
        diag: geom_core::Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some(predicate),
        },
    }
}

/// A bound direction's side code against a face — the F3 primitive
/// applied (module docs; the 15.7 sign resolution).
pub(super) fn side_code<T: Decide>(
    dir: Vec3<T>,
    face_normal: OutwardNormal<T>,
    arm: T,
    band: Band,
) -> Result<SideCode, BooleanError> {
    match enters_material(dir, face_normal, arm, band) {
        Ok(EntersMaterial::Enters) => Ok(SideCode::In),
        Ok(EntersMaterial::Exits) => Ok(SideCode::Out),
        Ok(EntersMaterial::Tangent) => Ok(SideCode::On),
        Err(diag) => Err(BooleanError::Escalated { diag }),
    }
}

/// The **second-order lump** of a declared-`Tangent` sector pair
/// (CONTACT-DESIGN C7/C12.2 at the boolean lump sites): first-order
/// data ties along a tangency by definition, so the side a
/// geometrically-ON sector is treated on descends one order — which
/// side does the sector's face CURVE to, relative to the other face's
/// material?
///
/// The margin is the existing second-order sector trilean's
/// (`tangent_sector_order2{,_arm}`, [`enters_material_order2`]): the
/// relative transverse curvature of the two carriers — the departing
/// transverse curve's acceleration on the sector's carrier measured
/// RELATIVE to the other carrier's own curving, signed against the
/// other face's outward normal — as the displacement it induces at
/// the sector's lever arm. Against a plane the partner curvature is
/// zero and the margin is the departure's own normal curvature (the
/// trilean's documented planar reading, reached bit-identically).
/// The transverse direction comes from the DEV-1 closed-form locus
/// ([`super::rest::tangent_locus`], the same rows the door's witness
/// derivation runs): the descent exists exactly where the witness
/// lane reaches, and nowhere else.
///
/// Verdicts: definitely curving into the other body's material ⇒
/// `In`; definitely away ⇒ `Out`; an EXACT second-order zero is the
/// isolated osculating point whose residue the verified declaration
/// bridges (C4's #175 clause) — the pair is locally conformal to
/// every order the kernel measures, and the declaration's verified
/// opposed material sides make that the Eq. 15.3 ⁻ lump; an in-band
/// margin escalates (an osculating pair is a sliver at this ε — F6).
#[allow(clippy::too_many_arguments)]
pub(super) fn tangent_lump<T: Decide>(
    sector_surface: &geom::Surface<T>,
    other_surface: &geom::Surface<T>,
    other_outward: OutwardNormal<T>,
    p: geom_core::Point3<T>,
    op: super::BooleanOp,
    on_side: Operand,
    sector_face: FaceKey,
    arm: T,
    band: Band,
) -> Result<SideCode, BooleanError> {
    use super::rest::{TangentLocus, TangentLocusError, tangent_locus};
    let locus_dir = match tangent_locus(sector_surface, other_surface, band) {
        Ok(TangentLocus::Line { dir, .. }) => dir,
        Err(TangentLocusError::Escalated(diag)) => return Err(BooleanError::Escalated { diag }),
        // The sector pair read geometrically ON while the carriers are
        // definitely apart or crossing: the same self-contradiction
        // family as a coplanar sector with definitely-distinct planes.
        Err(TangentLocusError::NotTangent { .. }) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "declared-Tangent sector pair with definitely non-tangent carriers",
            });
        }
        // Outside the DEV-1 closed-form lane no witness exists and no
        // descent does either — the C5 typed refusal, same as every
        // unopened arm.
        Err(TangentLocusError::Unsupported { .. }) => {
            return Err(BooleanError::CurvedBooleanUnsupported {
                operand: on_side,
                face: sector_face,
                kind: geom_brep::SurfaceKind::of(sector_surface),
            });
        }
    };
    let n_ref = other_outward.vec();
    // Transverse in-tangent-plane direction (the jet family's d̂ =
    // n̂ × τ̂; quadratic consumption, so τ̂'s sign is immaterial).
    let d_hat = n_ref.cross(locus_dir).normalize();
    // The graph-over-the-shared-tangent-plane acceleration of each
    // carrier along d̂, signed against n̂_ref: differentiating
    // F(x, z(x)) = 0 twice gives z″ = −d̂ᵀ(∇²F)d̂ / (∇F·n̂_ref) — the
    // denominator carries the orientation, exactly the jet chain's
    // own construction ([`geom_brep::tangent_jet`]).
    let graph_accel = |s: &geom::Surface<T>| {
        let g = geom_brep::implicit_gradient(s, p);
        T::zero() - geom_brep::implicit_hessian_form(s, p, d_hat) / g.dot(n_ref)
    };
    let rel_accel = graph_accel(sector_surface) - graph_accel(other_surface);
    match geom_brep::enters_material_order2(
        n_ref * rel_accel,
        T::one(),
        geom_brep::ReferenceNormal::of_face_outward(other_outward),
        arm,
        band,
    ) {
        Ok(EntersMaterial::Enters) => Ok(SideCode::In),
        Ok(EntersMaterial::Exits) => Ok(SideCode::Out),
        // The exact-zero osculating residue, bridged by the verified
        // declaration (doc above): locally conformal with verified
        // opposed senses IS the Eq. 15.3 ⁻ posture.
        Ok(EntersMaterial::Tangent) => Ok(super::tables::eq15_3_lump(
            op,
            on_side,
            super::plane_eq::PlaneRelation::SameOpposite,
        )),
        Err(diag) => Err(BooleanError::Escalated { diag }),
    }
}

/// One potentially-intersecting sector pair with its four side codes
/// (the `sectors[]` record, typed).
#[derive(Clone, Copy, Debug)]
pub(super) struct PairRecord {
    /// Index into the A-side sector array.
    pub a: usize,
    /// Index into the B-side sector array.
    pub b: usize,
    /// A-sector (start, end) bound codes vs the B-sector's face.
    pub sa: (SideCode, SideCode),
    /// B-sector (start, end) bound codes vs the A-sector's face.
    pub sb: (SideCode, SideCode),
    /// Whether the pair still generates intersection geometry.
    pub intersect: bool,
}

/// Whether `dir` lies within the convex sector (Zero grazes count —
/// module docs). `strict` demands definite interior.
///
/// Sense-invariant given the sector: `start`/`end` are traversal-
/// derived and `normal` already carries the sense, and `revert` flips
/// both together — a second `sense_sign` factor here would cancel
/// [`sector_face`]'s and turn every membership test inside out.
pub(super) fn within<T: Decide>(
    s: &BoolSector<T>,
    dir: Vec3<T>,
    strict: bool,
    band: Band,
) -> Result<bool, BooleanError> {
    let c1 = Margin::levered(s.start.cross(dir).dot(s.normal.vec()), s.arm);
    let c2 = Margin::levered(dir.cross(s.end).dot(s.normal.vec()), s.arm);
    let t1 =
        decide("bool_sector_within", c1, band).map_err(|diag| BooleanError::Escalated { diag })?;
    let t2 =
        decide("bool_sector_within", c2, band).map_err(|diag| BooleanError::Escalated { diag })?;
    Ok(if strict {
        t1 == Sign::Positive && t2 == Sign::Positive
    } else {
        t1 != Sign::Negative && t2 != Sign::Negative
    })
}

/// Same-direction parallelism of two bound directions (unit-ish).
fn parallel_same<T: Decide>(
    u: Vec3<T>,
    v: Vec3<T>,
    arm: T,
    band: Band,
) -> Result<bool, BooleanError> {
    let cross_margin = Margin::levered(u.cross(v).norm(), arm);
    match decide("bool_dir_parallel", cross_margin, band) {
        Ok(Sign::Zero) => {}
        Ok(_) => return Ok(false),
        Err(diag) => return Err(BooleanError::Escalated { diag }),
    }
    match decide("bool_dir_same", Margin::levered(u.dot(v), arm), band) {
        Ok(Sign::Positive) => Ok(true),
        Ok(Sign::Negative) => Ok(false),
        Ok(Sign::Zero) => Err(invalid_escalation(band, "bool_dir_same")),
        Err(diag) => Err(BooleanError::Escalated { diag }),
    }
}

/// `sectoroverlap` — coplanar sectors overlap test (module docs).
fn sector_overlap<T: Decide>(
    a: &BoolSector<T>,
    b: &BoolSector<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    for (s, dir) in [(a, b.start), (a, b.end), (b, a.start), (b, a.end)] {
        if within(s, dir, true, band)? {
            return Ok(true);
        }
    }
    let arm = a.arm.min(b.arm);
    // Identical region (same or crossed bound pairing).
    let straight =
        parallel_same(a.start, b.start, arm, band)? && parallel_same(a.end, b.end, arm, band)?;
    let crossed =
        parallel_same(a.start, b.end, arm, band)? && parallel_same(a.end, b.start, arm, band)?;
    Ok(straight || crossed)
}

/// The all-pairs search (15.7): every intersecting (A-sector, B-sector)
/// pair becomes a [`PairRecord`] with its four side codes, in
/// deterministic A-major order.
pub(super) fn pair_search<T: Decide>(
    a_sectors: &[BoolSector<T>],
    b_sectors: &[BoolSector<T>],
    band: Band,
) -> Result<Vec<PairRecord>, BooleanError> {
    let mut records = Vec::new();
    for (i, sa) in a_sectors.iter().enumerate() {
        for (j, sb) in b_sectors.iter().enumerate() {
            let int = sa.normal.vec().cross(sb.normal.vec());
            let arm = sa.arm.min(sb.arm);
            let coplanar = match decide(
                "bool_faces_parallel",
                Margin::levered(int.norm(), arm),
                band,
            ) {
                Ok(Sign::Zero) => true,
                Ok(Sign::Positive) => false,
                Ok(Sign::Negative) => {
                    return Err(invalid_escalation(band, "bool_faces_parallel"));
                }
                Err(diag) => return Err(BooleanError::Escalated { diag }),
            };
            let hit = if coplanar {
                sector_overlap(sa, sb, band)?
            } else {
                let d = int.normalize();
                (within(sa, d, false, band)? && within(sb, d, false, band)?)
                    || (within(sa, -d, false, band)? && within(sb, -d, false, band)?)
            };
            if !hit {
                continue;
            }
            let sa_codes = (
                side_code(sa.start, sb.normal, arm, band)?,
                side_code(sa.end, sb.normal, arm, band)?,
            );
            let sb_codes = (
                side_code(sb.start, sa.normal, arm, band)?,
                side_code(sb.end, sa.normal, arm, band)?,
            );
            records.push(PairRecord {
                a: i,
                b: j,
                sa: sa_codes,
                sb: sb_codes,
                intersect: true,
            });
        }
    }
    Ok(records)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The 15.7 sign resolution, mirror-pinned (F3): against a face
    /// with outward normal +z (material below), a direction with
    /// negative z ENTERS material ⇒ In; positive z ⇒ Out; in-plane ⇒
    /// On. Program 15.7's printed IN=+1 would flip the first two — the
    /// suspect side of ch. 15 erratum 4, resolved by derivation.
    #[test]
    fn mirror_check_side_codes() {
        let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let b = band();
        assert_eq!(
            side_code(Vec3::new(0.3, 0.0, -1.0), n, 1.0, b).unwrap(),
            SideCode::In
        );
        assert_eq!(
            side_code(Vec3::new(0.3, 0.0, 1.0), n, 1.0, b).unwrap(),
            SideCode::Out
        );
        assert_eq!(
            side_code(Vec3::new(1.0, 2.0, 0.0), n, 1.0, b).unwrap(),
            SideCode::On
        );
    }

    fn sector(start: [f64; 3], end: [f64; 3], normal: [f64; 3]) -> BoolSector<f64> {
        BoolSector {
            he: HalfEdgeKey::default(),
            start: Vec3::new(start[0], start[1], start[2]),
            end: Vec3::new(end[0], end[1], end[2]),
            start_edge: true,
            end_edge: true,
            face: FaceKey::default(),
            normal: OutwardNormal::from_chart(Vec3::new(normal[0], normal[1], normal[2]), true),
            arm: 1.0,
        }
    }

    /// `within`: interior yes, exterior no, boundary graze counts
    /// non-strictly and not strictly.
    #[test]
    fn within_convex_sector() {
        let s = sector([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        let b = band();
        let mid = Vec3::new(1.0, 1.0, 0.0).normalize();
        assert!(within(&s, mid, true, b).unwrap());
        assert!(!within(&s, Vec3::new(-1.0, -0.5, 0.0), false, b).unwrap());
        assert!(within(&s, Vec3::new(1.0, 0.0, 0.0), false, b).unwrap());
        assert!(!within(&s, Vec3::new(1.0, 0.0, 0.0), true, b).unwrap());
    }

    /// `sectoroverlap`: strict overlap yes; identical sectors yes;
    /// touch-only bound sharing NO (flows to the ON machinery, not to a
    /// fake coplanar pair).
    #[test]
    fn coplanar_overlap_cases() {
        let b = band();
        let s1 = sector([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        let deep = sector([0.5, 0.5, 0.0], [-0.5, 0.5, 0.0], [0.0, 0.0, 1.0]);
        assert!(sector_overlap(&s1, &deep, b).unwrap());
        assert!(sector_overlap(&s1, &s1.clone(), b).unwrap());
        let touch = sector([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!(!sector_overlap(&s1, &touch, b).unwrap());
    }

    /// The generic pair search on two orthogonal quarter-sector fans:
    /// records carry transition codes.
    #[test]
    fn pair_search_generic_crossing() {
        // A-sector in the xy-plane (normal +z), sweeping +x → +y CCW
        // around +z; B-sector in the zx-plane (normal +y), sweeping
        // +z → +x CCW around +y. They share the boundary ray +x.
        let a = sector([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        let bsec = sector([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let recs = pair_search(&[a], &[bsec], band()).unwrap();
        assert_eq!(recs.len(), 1);
        let r = recs[0];
        // A's start (+x) lies in B's face plane: On; A's end (+y) has
        // dot(+y, n_b=+y) > 0: Exits ⇒ Out. B's start (+z): dot with
        // n_a=+z > 0 ⇒ Out; B's end (+x): On.
        assert_eq!(r.sa, (SideCode::On, SideCode::Out));
        assert_eq!(r.sb, (SideCode::Out, SideCode::On));
    }

    // -----------------------------------------------------------------
    // The second-order Tangent lump (M9-3 PR-A item 4): three-outcome
    // honest on the existing `tangent_sector_order2` rows, driven on
    // raw carriers through the DEV-1 locus.
    // -----------------------------------------------------------------

    fn plate_top() -> geom::Surface<f64> {
        crate::fixtures::plane_surface(
            geom_core::Point3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        )
    }

    /// A y-axis cylinder at height `zc`, radius `r`.
    fn y_cyl(zc: f64, r: f64) -> geom::Surface<f64> {
        geom::Surface::Cylinder {
            origin: geom_core::Point3::new(2.0, 0.0, zc),
            axis: Vec3::new(0.0, 1.0, 0.0),
            radius: r,
            u_ref: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    /// A cylinder resting ON a plate top (external tangency along the
    /// ruling): the wall sector definitely curves AWAY from the
    /// plate's material ⇒ Out; the mirrored question (the plate's
    /// sector against the cylinder's material) is Out too.
    #[test]
    fn tangent_lump_external_tangency_is_out_both_ways() {
        let b = band();
        let p = geom_core::Point3::new(2.0, 0.5, 1.0);
        // The plate top's outward normal at the touch: +z.
        let plate_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let lump = tangent_lump(
            &y_cyl(1.5, 0.5),
            &plate_top(),
            plate_out,
            p,
            super::super::BooleanOp::Union,
            Operand::B,
            FaceKey::default(),
            0.5,
            b,
        )
        .unwrap();
        assert_eq!(lump, SideCode::Out);
        // The cylinder wall's outward normal at the bottom ruling: -z
        // (radially away from the axis, solid wall).
        let wall_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, -1.0), true);
        let lump = tangent_lump(
            &plate_top(),
            &y_cyl(1.5, 0.5),
            wall_out,
            p,
            super::super::BooleanOp::Union,
            Operand::A,
            FaceKey::default(),
            0.5,
            b,
        )
        .unwrap();
        assert_eq!(lump, SideCode::Out);
    }

    /// Internal tangency with the sector INSIDE the other body's
    /// material (a thin solid cylinder internally tangent inside a
    /// fat one): the thin wall curves definitely INTO the fat one's
    /// material ⇒ In.
    #[test]
    fn tangent_lump_nested_internal_tangency_is_in() {
        let b = band();
        // Fat solid cylinder r=0.5 about (x=2, z=1.5); thin r=0.25
        // about (x=2, z=1.25); both touch z=1 at the shared ruling.
        let p = geom_core::Point3::new(2.0, 0.5, 1.0);
        let fat_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, -1.0), true);
        let lump = tangent_lump(
            &y_cyl(1.25, 0.25),
            &y_cyl(1.5, 0.5),
            fat_out,
            p,
            super::super::BooleanOp::Union,
            Operand::A,
            FaceKey::default(),
            0.25,
            b,
        )
        .unwrap();
        assert_eq!(lump, SideCode::In);
    }

    /// Three-outcome honesty on the metered row: the SAME external
    /// tangency at three lever arms — definite (Out), in-band
    /// (escalates, an osculating pair is a sliver at this ε), and
    /// exactly-zero displacement (the isolated osculating residue the
    /// verified declaration bridges: the Eq. 15.3 minus-lump, In for
    /// a Union A-sector).
    #[test]
    fn tangent_lump_is_three_outcome_honest_on_the_arm() {
        let b = band();
        let p = geom_core::Point3::new(2.0, 0.5, 1.0);
        let plate_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let run = |arm: f64| {
            tangent_lump(
                &y_cyl(1.5, 0.5),
                &plate_top(),
                plate_out,
                p,
                super::super::BooleanOp::Union,
                Operand::A,
                FaceKey::default(),
                arm,
                b,
            )
        };
        // sagitta = kappa_rel * arm^2 / 2 = arm^2 (kappa_rel = 2 here).
        assert_eq!(run(0.5).unwrap(), SideCode::Out);
        match run(5e-5) {
            Err(BooleanError::Escalated { diag }) => {
                assert_eq!(diag.predicate, Some("tangent_sector_order2"));
            }
            other => panic!("an in-band sagitta must escalate: {other:?}"),
        }
        assert_eq!(run(3e-5).unwrap(), SideCode::In);
    }

    /// The self-contradiction and out-of-lane arms stay typed: a
    /// definitely-apart pair at the lump site is the classification
    /// invariant family; a kind pair outside the DEV-1 lane keeps the
    /// C5 typed refusal.
    #[test]
    fn tangent_lump_refuses_typed_off_the_lane() {
        let b = band();
        let p = geom_core::Point3::new(2.0, 0.5, 1.0);
        let plate_out = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        match tangent_lump(
            &y_cyl(2.5, 0.5),
            &plate_top(),
            plate_out,
            p,
            super::super::BooleanOp::Union,
            Operand::A,
            FaceKey::default(),
            0.5,
            b,
        ) {
            Err(BooleanError::ClassificationInvariant { .. }) => {}
            other => panic!("definitely-apart carriers at a lump site: {other:?}"),
        }
        let sphere = geom::Surface::Sphere {
            center: geom_core::Point3::new(2.0, 0.5, 2.0),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        match tangent_lump(
            &sphere,
            &plate_top(),
            plate_out,
            p,
            super::super::BooleanOp::Union,
            Operand::A,
            FaceKey::default(),
            0.5,
            b,
        ) {
            Err(BooleanError::CurvedBooleanUnsupported { .. }) => {}
            other => panic!("sphere tangency is outside the DEV-1 lane: {other:?}"),
        }
    }
}
