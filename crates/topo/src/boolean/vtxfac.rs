//! `vtxfacclassify` — the vertex-on-face classifier the book never
//! prints ("for space reasons", §15.6.1): the ch. 14 classifier with
//! the three deltas, DESIGNED here:
//!
//! 1. **datum := the pierced face's TANGENT plane at the pierce
//!    point**; classes are [`SideCode`]s with OUT/IN derived from the
//!    F3 chain (a chord's class is its signed elevation off that plane
//!    against the OUTWARD normal — `side_code`; 15.7's printed
//!    `IN = +1` is never consulted).
//!
//!    The datum is a DIRECTION, never an origin: `side_code`
//!    (`sectors.rs`) takes `(dir, reach, OutwardNormal, lever, band)`
//!    and the germ direction takes a cross product, so both are
//!    first-order primitives (charged for the face's curvature through
//!    `lever`) and generalize to a curved pierced face by substituting
//!    the per-point outward normal
//!    ([`crate::face_normal::face_outward_normal_at`]). On a plane that
//!    normal is the plane's own, so the planar lane's arithmetic is
//!    bit-identical. What does NOT generalize is Delta 2's carrier
//!    algebra, which needs an origin — it refuses typed on a curved
//!    pierced face rather than inventing one.
//! 2. **On-sectors reclassify per Eq. 15.3** (op-dependent), behind
//!    [`super::oriented_plane_eq`] — a coplanar sector must be
//!    *declaredly* coplanar with the pierced face or the op refuses.
//! 3. **The ring insertion** (the unprinted Euler sequence, designed):
//!    in the pierced body,
//!    `mev(Fan{anchor, anchor})` a **chord strut** from a deterministic
//!    boundary vertex of the face to the pierce point (a real,
//!    certified line edge, dangling — structurally legal mid-op), then
//!    `kemr(chord.he_plus, chord.he_minus)` kills the chord and leaves
//!    the pierce vertex as an **empty-loop ring** inside the face
//!    (KemrResult: he1's strictly-between side is empty ⇒ the ring is
//!    the lone new vertex — the dangling chord exists only between the
//!    two ops), then one `mev_null` **strut per piercing-side run**
//!    hangs the paired null edges as a tree from the ring vertex
//!    ([`crate::null::ring_tree`]: `MevSite::Lone` for the first,
//!    `Fan{he,he}` after, at the ring vertex or another strut's far
//!    end). Dangling
//!    ring null edges are the documented ch. 15 transient; joining
//!    consumes them in PR 5. Tier 1 holds after every op (pinned by the
//!    acceptance test).
//!
//! **On-edges** (an edge through the pierce vertex lying IN the face's
//! plane): resolved by the flanking classes — `(In,·,In) → In`,
//! `(Out,·,Out) → Out`, mixed → `In`, the one fold rule
//! ([`super::sectors::fold_on_bound`]) the vertex-vertex attribution
//! shares. The split lane's F4 table agrees for a convex edge and
//! deliberately DIVERGES for a reflex one
//! (`BOB → ABOVE`): the split must mint copies to keep the two pieces'
//! fans representable, but a boolean tangential contact is a *legal 3′
//! touching* (edge-on-face, both flanking faces the same side) already
//! carried by the declared contact records — TOG Table II rows 5/9
//! (`(In,In)`/`(Out,Out)` ⇒ no intersection) confirm no crossing is
//! recorded. Mixed keeps the In side (both witnesses' choice for the
//! split analogue).
//!
//! **Read twice.** A piercing vertex that also pairs with a vertex of
//! the other solid — which then holds its own contact at the point — is
//! read by both passes ([`pierced_and_paired`]). It is read again
//! only where its pierce touches the face, so its orbit is unwritten;
//! a pierce that would cross refuses before it writes, and so does a
//! second pierce. The pair's partner must lie strictly on one side of
//! the face ([`partner_side`]). The vertex's edges are then classed
//! against the face and its partners together ([`touch_classes`]): the
//! partners' cones nest or lie apart on each side, and crossing each
//! boundary flips the side ([`layered`]); an edge it cannot class
//! refuses. A vertex in pairs alone layers them the same way, its
//! outermost cones read off which hold which ([`pair_classes`]).

use geom_brep::OutwardNormal;
use geom_core::{Band, Decide, Margin, Sign, Vec3};

use super::plane_eq::PlaneEqError;
use super::reduce::face_plane;
use std::collections::BTreeMap;

use super::sectors::{BoolSector, build_sectors, side_code};
use super::tables::{eq15_3_lump, lump_keeps_one};
use super::{
    BoolNullEdgeRecord, BooleanError, BooleanOp, ContactRecords, NullEdgePairRecord, Operand,
    PairSite, PierceRingRecord, SectorRead, SideCode, VfContact,
};
use super::{Coincide, Contradiction, DeclarationRead};
use crate::body::{Body, WALKS_CLOSE};
use crate::contact::BooleanCoincidence;
use crate::entity::{EntityId, HalfEdgeKey, VertexKey};
use crate::euler::{MevSite, RunSite};
use crate::live::{Proven, linked, proven};
use crate::null::NewVertexSide;
use crate::validate::decide;
use geom_core::Tol;

/// **Each vertex that pierces a face and pairs with a vertex too**, with
/// its first pair; refuses [`BooleanError::VertexReadTwice`] where a
/// vertex pierces two faces, before any pass writes. A pierce
/// hangs struts at its vertex only where it crosses the face, so such a
/// vertex is read again only where its pierce touches
/// ([`classify_vertex_on_face`] refuses before it writes, and
/// [`partner_side`] and [`touch_classes`] read the touch). A vertex in
/// several pairs and in no pierce is not returned, since the
/// vertex-vertex passes read every pair before the first writes.
pub(super) fn pierced_and_paired(
    contacts: &ContactRecords,
) -> Result<BTreeMap<(Operand, VertexKey), SectorRead>, BooleanError> {
    let mut rereads = BTreeMap::new();
    for (operand, pierced) in [
        (Operand::A, &contacts.a_on_b),
        (Operand::B, &contacts.b_on_a),
    ] {
        let mut pierces = BTreeMap::new();
        for c in pierced {
            if let Some(&face) = pierces.get(&c.vertex) {
                return Err(BooleanError::VertexReadTwice {
                    operand,
                    vertex: c.vertex,
                    reads: [SectorRead::Pierce(face), SectorRead::Pierce(c.face)],
                });
            }
            pierces.insert(c.vertex, c.face);
        }
        for c in &contacts.vv {
            let (own, other) = match operand {
                Operand::A => (c.a, c.b),
                Operand::B => (c.b, c.a),
            };
            if pierces.contains_key(&own) {
                rereads
                    .entry((operand, own))
                    .or_insert(SectorRead::Pair(other));
            }
        }
    }
    Ok(rereads)
}

/// The pierced face's oriented datum at a pierce point: its outward
/// normal and the lever its side verdicts charge ([`side_code`]).
#[derive(Clone, Copy, Debug)]
pub(super) struct PierceDatum<T: geom_core::Real> {
    pub normal: OutwardNormal<T>,
    pub lever: T,
}

/// **The side of a pierced face a paired vertex's link lies on**, where
/// the piercing vertex only touches the face: `In` or `Out` where every
/// bound of `partner`'s sectors reads strictly that side of the face's
/// datum, the reading the touch itself was decided by; `None` where any
/// bound reads on the face, in band, or the bounds read both sides.
///
/// `None` refuses: [`touch_classes`] and the vertex-vertex pass read
/// the touch as the face and the partner's cone meeting only at the
/// point (`work/tang/a-touching-vertex-beside-a-partner-along-the-face-refuses.md`).
pub(super) fn partner_side<T: Decide>(
    partner: &[BoolSector<T>],
    datum: PierceDatum<T>,
    band: Band,
) -> Option<SideCode> {
    let mut side = None;
    for s in partner {
        for (dir, reach) in [(s.start, s.start_reach), (s.end, s.end_reach)] {
            match side_code(dir, reach, datum.normal, datum.lever, band) {
                Ok(c @ (SideCode::In | SideCode::Out)) if side.is_none_or(|k| k == c) => {
                    side = Some(c);
                }
                _ => return None,
            }
        }
    }
    side
}

/// One pair of a vertex read again: its partner, the partner's side of
/// the face the vertex touches ([`partner_side`]; `None` where it
/// touches none), what [`super::sectors::wedge_classes`] read against the
/// partner (`None` where it read nothing), and the partner's sectors.
#[derive(Debug, Clone)]
pub(super) struct PairRead<T: geom_core::Real> {
    pub partner: VertexKey,
    pub side: Option<SideCode>,
    pub read: Option<super::sectors::WedgeRead>,
    pub sectors: Vec<BoolSector<T>>,
}

/// Whether a partner's reading of an edge puts it inside the partner's
/// cone: In its material where that is the cone (met), Out of it where
/// that is the cone's complement.
fn held(class: SideCode, met: bool) -> bool {
    matches!((class, met), (SideCode::In, true) | (SideCode::Out, false))
}

/// **An edge's class among layered partners**: `base` where it lies in
/// none of their cones, flipped once for each cone that holds it.
///
/// Each partner's material is a convex cone (met) or its complement
/// (joined) whose boundary meets the others' only at the point, so the
/// cones nest or lie apart. Every face bounds the other solid's
/// material on one side only, so crossing a partner's boundary flips
/// it. A partner's row says on which side of its own boundary the edge
/// lies, so which cones hold the edge, and how many, is read off the
/// rows alone. An edge on one partner's boundary is on the solid's,
/// whatever depth that boundary lies at; one on two, or beside a
/// partner that read no rows, is not decided here.
fn layered<'a, T: geom_core::Real>(
    base: SideCode,
    he: HalfEdgeKey,
    partners: impl Iterator<Item = &'a PairRead<T>>,
) -> Option<SideCode> {
    let (mut holding, mut on) = (0usize, 0usize);
    for p in partners {
        let read = p.read.as_ref()?;
        let &(_, class) = read.rows.iter().find(|&&(h, _)| h == he)?;
        if class == SideCode::On {
            on += 1;
        } else if held(class, read.met) {
            holding += 1;
        }
    }
    match (on, holding % 2, base) {
        (0, 0, _) => Some(base),
        (0, _, SideCode::Out) => Some(SideCode::In),
        (0, _, SideCode::In) => Some(SideCode::Out),
        (1, _, _) => Some(SideCode::On),
        _ => None,
    }
}

/// **The classes of a touching vertex's edges against the other solid**,
/// from its pierce's classes (`touch`) and its pairs'. `Err` where an
/// edge is left undecided, which refuses: the partner on that edge's
/// side that read nothing, or failing that the first there. An edge is
/// undecided only beside a partner, so `None` breaks that.
///
/// Near the point the other solid is the pierced face's half-space `H`
/// and the material of each partner, whose boundary lies strictly on
/// one side of the face. The side of the face an edge leaves on is
/// outside every cone on the other side, so only its own side's
/// partners read it, [`layered`] over its pierce's class: outside `H`
/// beside no cone, inside `H` beside no void. An edge on the face is on
/// the solid's boundary, and no partner's cone reaches it.
pub(super) fn touch_classes<T: geom_core::Real>(
    touch: &[(HalfEdgeKey, SideCode)],
    pairs: &[PairRead<T>],
) -> Result<Vec<(HalfEdgeKey, SideCode)>, Option<VertexKey>> {
    touch
        .iter()
        .map(|&(he, own)| {
            let side = || pairs.iter().filter(move |p| p.side == Some(own));
            layered(own, he, side()).map(|c| (he, c)).ok_or_else(|| {
                side()
                    .find(|p| p.read.is_none())
                    .or_else(|| side().next())
                    .map(|p| p.partner)
            })
        })
        .collect()
}

/// **The classes of a vertex's edges against the other solid, where
/// the vertex is in pairs alone**, touching no face.
///
/// Several partners read [`layered`]: the cones that no other holds
/// are outermost, and the material beyond them is outside the solid
/// where they are met and inside it where they are joined. Where a
/// partner reads nothing, an edge is left undecided, or the outermost
/// disagree, each pair's rows stand as read, as for one pair. A nesting
/// read in band refuses.
pub(super) fn pair_classes<T: Decide>(
    pairs: &[PairRead<T>],
    band: Band,
) -> Result<Vec<(HalfEdgeKey, SideCode)>, BooleanError> {
    let alone = || {
        pairs
            .iter()
            .filter_map(|p| p.read.as_ref())
            .flat_map(|r| r.rows.iter().copied())
            .collect()
    };
    Ok(match pairs {
        [_] => alone(),
        _ => layered_alone(pairs, band)?.unwrap_or_else(alone),
    })
}

/// [`pair_classes`]' layering, or `None` where it cannot decide.
fn layered_alone<T: Decide>(
    pairs: &[PairRead<T>],
    band: Band,
) -> Result<Option<Vec<(HalfEdgeKey, SideCode)>>, BooleanError> {
    // Partner `i`'s cone lies inside `j`'s where `j` holds every edge
    // of `i` (strictly: cones whose boundaries meet only at the point).
    let inside = |i: usize, j: usize| -> Result<Option<bool>, BooleanError> {
        let read = super::sectors::wedge_classes(&pairs[i].sectors, &pairs[j].sectors, band)?;
        Ok(read.map(|r| !r.rows.is_empty() && r.rows.iter().all(|&(_, c)| held(c, r.met))))
    };
    let mut base = None;
    for i in 0..pairs.len() {
        let mut outermost = true;
        for j in (0..pairs.len()).filter(|&j| j != i) {
            match inside(i, j)? {
                None => return Ok(None),
                Some(true) => outermost = false,
                Some(false) => {}
            }
        }
        if outermost {
            let Some(read) = pairs[i].read.as_ref() else {
                return Ok(None);
            };
            let here = if read.met {
                SideCode::Out
            } else {
                SideCode::In
            };
            // Outermost cones that disagree fall back: main's
            // `a_dangling_null_edge_inside_another_along_one_end_builds_in_every_op`
            // reaches it, and builds on the per-pair rows.
            if base.is_some_and(|b| b != here) {
                return Ok(None);
            }
            base = Some(here);
        }
    }
    let (Some(base), Some(first)) = (base, pairs[0].read.as_ref()) else {
        return Ok(None);
    };
    Ok(first
        .rows
        .iter()
        .map(|&(he, _)| Some((he, layered(base, he, pairs.iter())?)))
        .collect())
}

/// Output of one vertex-on-face classification.
#[derive(Debug)]
pub(super) struct VtxFacOut<T: geom_core::Real> {
    /// Minted null edges (piercing-side runs + pierced-side ring struts).
    pub edges: Vec<BoolNullEdgeRecord<T>>,
    /// Cross-body correspondence pairs.
    pub pairs: Vec<NullEdgePairRecord>,
    /// The ring insertion, if surgery happened.
    pub ring: Option<PierceRingRecord>,
    /// `(A face, B face)` for each coincident sector whose lump keeps
    /// one copy of the region (`BooleanReduction::covered`).
    pub covered: Vec<(crate::entity::FaceKey, crate::entity::FaceKey)>,
    /// Each edge at the piercing vertex, by the half-edge leaving it,
    /// with its side of the pierced face as first read, before any
    /// lump (`BooleanReduction::edge_classes`).
    pub classes: Vec<(HalfEdgeKey, SideCode)>,
    /// The pierced face's datum the classes were read against.
    pub datum: PierceDatum<T>,
}

#[derive(Clone, Copy, Debug)]
struct Entry {
    he: HalfEdgeKey,
    is_edge: bool,
    class: SideCode,
    /// The class is Delta 2's lump, not this bound's own reading.
    lumped: bool,
}

/// Classifies `contact.vertex` (in the piercing body) against
/// `contact.face` (in the pierced body) and performs the paired
/// insertion (module docs). Where the vertex is read again (`reread`,
/// from [`pierced_and_paired`]) and crosses the face, refuses
/// [`BooleanError::VertexReadTwice`] before it writes.
#[allow(clippy::too_many_arguments)]
pub(super) fn classify_vertex_on_face<T: Decide + crate::props::AtRestPolicy>(
    piercing_body: &mut Body<T>,
    pierced_body: &mut Body<T>,
    piercing: Operand,
    contact: VfContact,
    op: BooleanOp,
    declared: &super::DeclaredPairs<T>,
    contacts: &super::ContactRecords,
    reread: Option<SectorRead>,
    band: Band,
    tol: Tol,
) -> Result<VtxFacOut<T>, BooleanError> {
    let vertex = contact.vertex;
    let pierced_op = piercing.other();
    // The pierce point, read BEFORE the classification: on a curved
    // pierced face the oriented datum is point-dependent, so `p` is an
    // input to the sector algebra rather than only to Delta 3's ring.
    let p = piercing_body.resolve_vertex_point(vertex, Proven);
    // `contact.face` is a key the sweep recorded and carries here, so
    // its miss is typed; its surface is a link, and nothing has written
    // the pierced body yet.
    let pierced_face =
        pierced_body
            .get_face(contact.face)
            .ok_or(BooleanError::ClassificationInvariant {
                what: "a vertex-on-face contact's face no longer resolves",
            })?;
    let pierced_surface = pierced_body.face_surface_linked(contact.face, pierced_face);
    // The kind every refusal below cites, read before any write.
    let pierced_kind = pierced_surface.kind();
    // The pierced face's oriented datum at `p`, from the one door.
    // `n_pierced` carries the material side, typed so the sense flip
    // cannot be dropped on the way; on a PLANE it is bit-identically
    // what [`face_plane`] reports as its `normal`
    // ([`super::reduce::face_plane`] builds it from this very door),
    // which is what keeps the planar lane's arithmetic unmoved.
    //
    // `plane` itself survives only for Delta 2's planar carrier
    // algebra, which needs an ORIGIN as well as a direction; the
    // curved lane refuses there rather than building one (below).
    let plane = face_plane(pierced_body, contact.face);
    let n_pierced =
        match crate::face_normal::face_outward_normal_at(pierced_body, contact.face, p, band) {
            Ok(Some(n)) => n,
            // Cone / NURBS pierced faces: the C5 typed refusal, naming
            // the kind that has no arm.
            Ok(None) => {
                return Err(BooleanError::CurvedBooleanUnsupported {
                    operand: pierced_op,
                    face: contact.face,
                    kind: pierced_kind,
                });
            }
            Err(refusal) => {
                return Err(BooleanError::of_pierced_normal(
                    refusal,
                    pierced_op,
                    contact.face,
                ));
            }
        };
    // The pierced face's smallest radius of curvature — the lever the
    // sector side verdicts charge their sagitta against (`side_code`'s
    // argument), so it must bound the tightest bend, not the chart's
    // scale: on a fat torus those differ. A plane reports `f64::MAX`, so
    // its charge is vacuous and the planar lane's verdicts are unmoved.
    let pierced_lever = geom_brep::min_radius_of_curvature(pierced_surface, p);
    let sectors = build_sectors(piercing_body, piercing, vertex, band)?;
    let n = sectors.len();

    // Entries = the bounds in orbit order (entry k = sector k's END
    // bound: real chord or subdivision bisector), classed against the
    // pierced face's plane via the F3 primitive. `n_pierced` is the
    // pierced face's OUTWARD normal (S10, minted by
    // `face_outward_normal`) — In/Out here is a material verdict and
    // would read backwards off a chart normal on a reversed face,
    // which is why the primitive takes the typed one.
    let mut entries = Vec::with_capacity(n);
    for s in &sectors {
        entries.push(Entry {
            he: s.he,
            is_edge: s.end_edge(),
            class: side_code(s.end, s.end_reach, n_pierced, pierced_lever, band)?,
            lumped: false,
        });
    }

    // Delta 2 (rule (a) analogue): coplanar sectors lump per Eq. 15.3.
    // "Coplanar" is against the pierced face's TANGENT plane at `p`,
    // whose normal is `n_pierced` — the same vector `face_plane` hands
    // back on a planar face, so the planar lane's margin is unmoved.
    //
    // Lumping overwrites both bounds' readings, so both must read On,
    // each at its reach — a line bound at its far vertex, in metres; a
    // bound read definitely off the plane decides the sector off it.
    // Where both read On, the normals' parallelism at the arm proposes
    // coplanarity. A tilt the bounds read On and the normals do not (in
    // band, or decided off: a thin sector tilted about one bound rises
    // at its arm by more than at its other bound) leaves the question
    // undecided at this tolerance, and the margin that decides it is the
    // bounds' own: a smaller tolerance reads the steeper bound off the
    // plane and the sector with it.
    let read: Vec<SideCode> = entries.iter().map(|e| e.class).collect();
    let classes = entries
        .iter()
        .filter(|e| e.is_edge)
        .map(|e| (e.he, e.class))
        .collect();
    let mut covered = Vec::new();
    for (k, s) in sectors.iter().enumerate() {
        if read[k] != SideCode::On || read[(k + 1) % n] != SideCode::On {
            continue;
        }
        let m = Margin::levered(s.normal.vec().cross(n_pierced.vec()).norm(), s.arm);
        let tilt = decide("bool_sector_coplanar", m, band);
        // The declaration is read before the tilt refuses: a declared
        // pair's lump takes an in-band residue (the carrier ladder's
        // declared rung bridges it on a planar pierced face; the
        // `Tangent` lump descends to the second order), so only an
        // undeclared pair refuses it here, and the class the door admits
        // for this pierced face would change its verdict. A tilt decided
        // off is no coincidence a declaration settles: a declared pair's
        // claim is contradicted by it, and an undeclared pair's sector is
        // undecided on its bounds' margin with no declaration to offer.
        let class = declared.class_of(piercing, s.face, pierced_op, contact.face);
        let decided_tilt = matches!(tilt, Ok(Sign::Positive | Sign::Negative));
        if let (true, Some(class)) = (decided_tilt, class) {
            let (a, b) = match piercing {
                Operand::A => (s.face, contact.face),
                Operand::B => (contact.face, s.face),
            };
            let planar = plane.is_some()
                && matches!(
                    super::rest::face_carrier(piercing_body, s.face),
                    Some(super::carrier_eq::CarrierDesc::Plane { .. })
                );
            let fact =
                (planar && class.is_one_carrier()).then_some(Contradiction::PlanesNotParallel);
            let margin = geom_core::Indeterminate {
                margin: geom_core::MarginDiag::INVALID,
                band,
                predicate: Some("bool_sector_coplanar"),
                terminal_sliver: false,
            };
            return Err(match class {
                BooleanCoincidence::Continuation => {
                    BooleanError::ContinuationContradicted { a, b, fact, margin }
                }
                BooleanCoincidence::Seam => BooleanError::SeamContradicted { a, b, fact, margin },
                BooleanCoincidence::Contact(class) => BooleanError::ContactContradicted {
                    declaration: crate::contact::DeclaredContact { a, b, class },
                    steer: fact.and_then(super::contact_verify::fit_steer),
                    fact,
                    margin,
                },
            });
        }
        // A declared one-carrier pair's lump bridges an in-band tilt, and
        // a `Tangent` pair's descends to the second order; a seam has no
        // lump arm here, so its in-band tilt refuses as an undeclared
        // one does.
        let refused = match (&tilt, class) {
            (Ok(Sign::Zero), _) => false,
            (Err(_), Some(BooleanCoincidence::Seam) | None)
            | (Ok(Sign::Positive | Sign::Negative), _) => true,
            (Err(_), Some(_)) => false,
        };
        if refused {
            // A one-carrier declaration bridges the residue only against
            // a planar pierced face (a curved one refuses below whatever
            // is declared), and the door admits the one the pair's senses
            // make it, or none where it cannot read them; `Tangent` only
            // where the door's witness lane derives the pair's tangency,
            // which it checks before it admits one.
            fn surface<T: Decide>(
                body: &Body<T>,
                f: crate::entity::FaceKey,
            ) -> Option<&geom::Surface<T>> {
                body.get_face(f)
                    .and_then(|face| body.get_surface(face.surface))
            }
            let tangent = match (
                surface(piercing_body, s.face),
                surface(pierced_body, contact.face),
                match class {
                    Some(_) => {
                        Some(declared.reach_of(piercing, s.face, pierced_op, contact.face)?)
                    }
                    // An undeclared pair's extent, read mid-operation:
                    // a face whose box no longer reads offers no
                    // `Tangent` (`work/tang/the-tangent-offer-drops-for-a-face-with-null-scaffolding-mid-op.md`).
                    None => super::rest::pair_extent(
                        piercing_body,
                        s.face,
                        pierced_body,
                        contact.face,
                        band,
                    )
                    .ok()
                    .map(|extent| extent.reach),
                },
            ) {
                (Some(a), Some(b), Some(reach)) => {
                    geom_brep::tangent_locus(a, b, reach, band).is_ok()
                }
                _ => false,
            };
            let one_carrier = plane
                .is_some()
                .then(|| {
                    super::plane_eq::senses(s.normal.vec(), n_pierced.vec(), s.arm, band)
                        .and_then(BooleanCoincidence::of_senses)
                })
                .flatten();
            let admitted: Vec<BooleanCoincidence> = one_carrier
                .into_iter()
                .chain(tangent.then_some(BooleanCoincidence::TANGENT))
                // A decided tilt admits no class: no declaration settles it.
                .filter(|_| !decided_tilt)
                .collect();
            let read = declared.read(
                &[(piercing, s.face, pierced_op, contact.face)],
                Coincide::Sectors,
                &admitted,
            );
            let nv = n_pierced.vec();
            let steeper = s
                .start_reach
                .departure(s.start, nv)
                .abs()
                .max(s.end_reach.departure(s.end, nv).abs());
            return match crate::validate::decide_nonzero(
                "bool_sector_coplanar",
                Margin::of(steeper),
                band,
            ) {
                Err(diag) => Err(BooleanError::coincidence(Coincide::Sectors, read, diag)),
                Ok(_) => Err(BooleanError::ClassificationInvariant {
                    what: "a sector bound read On departs definitely from the plane",
                }),
            };
        }
        // Declared-`Tangent` (distinct carriers touching): the lump
        // verdict is the second-order sector trilean — which side the
        // sector's carrier CURVES to relative to the pierced face's
        // material ([`super::sectors::tangent_lump`]), read for the WHOLE
        // sector. Per bound, the bound riding the locus reads `On`, which
        // the on-entry resolution below settles from its neighbours; but
        // an arc tangent at this vertex is split at the band's edge, and
        // the sliver's arm puts the arc's second-order margin in the zero
        // band too, so the two `On`s are the consecutive-`On` refusal
        // (`work/hone/an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band.md`;
        // pinned by `m9_3_zip::a_tangent_curved_sector_on_a_face_lumps_whole`).
        if class == Some(BooleanCoincidence::TANGENT) {
            let surface_of = |body: &Body<T>, f| {
                body.get_face(f)
                    .and_then(|face| body.get_surface(face.surface))
                    .cloned()
                    .ok_or(BooleanError::ClassificationInvariant {
                        what: "declared-Tangent face lost its surface",
                    })
            };
            let s_sector = surface_of(piercing_body, s.face)?;
            let s_pierced = surface_of(pierced_body, contact.face)?;
            let read = declared.read(
                &[(piercing, s.face, pierced_op, contact.face)],
                Coincide::TangentSide,
                &[],
            );
            let reach = declared.reach_of(piercing, s.face, pierced_op, contact.face)?;
            let lump = super::sectors::tangent_lump(
                &s_sector, &s_pierced, reach, n_pierced, p, op, piercing, s.face, s.arm, read, band,
            )?;
            entries[k].class = lump;
            entries[(k + 1) % n].class = lump;
            entries[k].lumped = true;
            entries[(k + 1) % n].lumped = true;
            continue;
        }
        // The conformal (carrier) lump. The sector's oriented carrier
        // description generalizes the plane one (`face_carrier` folds
        // the face sense exactly as `face_plane` does — S10); a kind
        // outside the `Rest` ladder's inventory (cone, torus, NURBS)
        // keeps the C5 typed refusal.
        let Some(sector_carrier) = super::rest::face_carrier(piercing_body, s.face) else {
            let kind = piercing_body
                .get_face(s.face)
                .and_then(|f| piercing_body.get_surface(f.surface))
                .map_or(geom::SurfaceKind::Nurbs, geom::Surface::kind);
            return Err(BooleanError::CurvedBooleanUnsupported {
                operand: piercing,
                face: s.face,
                kind,
            });
        };
        let declared_one_carrier =
            declared.declares_one_carrier(piercing, s.face, pierced_op, contact.face);
        // C8: a CURVED on-carrier sector is opened by a VERIFIED
        // declaration and by nothing else — undeclared touching keeps
        // this typed frontier refusal. The recourse is a declared
        // coincidence (C4: a `Rest` or `Tangent` contact, or a
        // continuation), under which classification descends to the
        // carrier ladder or the C7 sector trilean instead of refusing.
        if !declared_one_carrier
            && !matches!(sector_carrier, super::carrier_eq::CarrierDesc::Plane { .. })
        {
            let kind = piercing_body
                .get_face(s.face)
                .and_then(|f| piercing_body.get_surface(f.surface))
                .map_or(geom::SurfaceKind::Nurbs, geom::Surface::kind);
            return Err(BooleanError::CurvedBooleanUnsupported {
                operand: piercing,
                face: s.face,
                kind,
            });
        }
        // The pierced face's carrier: its plane, or — on a curved
        // pierced face — its curved carrier, which only a verified
        // one-carrier declaration may compare against the sector's. A
        // sector face on a curved pierced face at `p` is a cosurface
        // question, and CONTACT-DESIGN C2/C4 forbid inferring that gluing
        // at any ε: the declaration is the recourse, and with it the
        // carrier ladder's declared rung decides the relation exactly as
        // on a plane. An undeclared curved sector refused just above. An
        // undeclared PLANAR sector lumped here would be a plane tangent
        // to the curved face with a bound leaving along it, and no
        // public operand reaches that: the crossing layer refuses the
        // bound's tangent touch first (a prism tangent to a cylinder
        // along a ruling refuses `CurvedPierceUnsupported` there, in both
        // operand orders), and a declared `Tangent` lumps above. The
        // guard keeps this door for that state all the same.
        let pierced_carrier = match plane {
            Some(plane) => super::carrier_eq::CarrierDesc::Plane {
                origin: plane.origin,
                normal: plane.normal,
            },
            None => match super::rest::face_carrier(pierced_body, contact.face) {
                Some(carrier) if declared_one_carrier => carrier,
                _ => {
                    return Err(BooleanError::CurvedBooleanUnsupported {
                        operand: pierced_op,
                        face: contact.face,
                        kind: pierced_kind,
                    });
                }
            },
        };
        // Oriented sources (S10): both descriptions carry OUTWARD
        // material sides, so rung 1's syntactic Same± verdict has to
        // see the face senses as well as the surfaces' `orient` tags.
        let (g1, g2) = (
            super::reduce::face_oriented_source(piercing_body, s.face),
            super::reduce::face_oriented_source(pierced_body, contact.face),
        );
        let id = super::PlaneIdentity {
            s1: g1.as_ref(),
            s2: g2.as_ref(),
            declared: declared_one_carrier,
        };
        // A declared pair reads as the door read it at rest, over both
        // faces; an undeclared one at the sector's arm.
        let extent = if declared_one_carrier {
            declared.consumed(piercing, s.face, pierced_op, contact.face)?
        } else {
            super::carrier_eq::ConsumedExtent::unwitnessed(geom_brep::ExtentBall::new(
                geom_core::Point3::origin(),
                s.arm,
            ))
        };
        let rel = match super::carrier_eq::carrier_eq(
            &sector_carrier,
            &pierced_carrier,
            id,
            &extent,
            band,
        ) {
            Ok(super::PlaneRelation::Distinct) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "geometrically coplanar sector with definitely-distinct plane",
                });
            }
            Ok(rel) => rel,
            // In band, unreachable here: `bool_sector_coplanar` read
            // this same margin (the two normals' cross, at `s.arm`)
            // zero above for an undeclared pair, and a declared `Rest`
            // pair's rung bridges it. The door is `recl`'s, which
            // reaches it at another arm.
            Err(PlaneEqError::Escalated { rung, diag }) => {
                return Err(BooleanError::plane_identity(
                    rung,
                    declared.on_pair_door(
                        (piercing, s.face, pierced_op, contact.face),
                        super::plane_eq::senses(s.normal.vec(), n_pierced.vec(), s.arm, band),
                    ),
                    diag,
                ));
            }
            Err(PlaneEqError::Undeclared {
                coincidence,
                relation,
            }) => {
                return Err(super::undeclared_coincidence(
                    coincidence,
                    [(piercing, s.face), (pierced_op, contact.face)],
                    relation,
                ));
            }
            Err(PlaneEqError::Contradicted { fact, .. }) => {
                return Err(BooleanError::DeclarationContradicted { fact });
            }
            // Only a declared reading is unsettled.
            Err(PlaneEqError::Unsettled { diag }) => {
                return Err(super::unsettled_rest(
                    class.ok_or(BooleanError::ClassificationInvariant {
                        what: "an undeclared coplanar sector's carrier reading was unsettled",
                    })?,
                    diag,
                ));
            }
        };
        if lump_keeps_one(op, rel) {
            covered.push(match piercing {
                Operand::A => (s.face, contact.face),
                Operand::B => (contact.face, s.face),
            });
        }
        let lump = eq15_3_lump(op, piercing, rel);
        entries[k].class = lump;
        entries[(k + 1) % n].class = lump;
        entries[k].lumped = true;
        entries[(k + 1) % n].lumped = true;
    }

    // On-edge resolution (module docs; the deliberate divergence).
    for k in 0..n {
        if entries[k].class == SideCode::On && entries[(k + 1) % n].class == SideCode::On {
            return Err(BooleanError::ClassificationInvariant {
                what: "consecutive On entries after Eq. 15.3 lumping",
            });
        }
    }
    resolve_on_entries(&mut entries, band)?;

    // Out-runs (the copy takes the OUT side — above ≙ OUT).
    let runs = out_runs(&entries);
    let mut out = VtxFacOut {
        edges: Vec::new(),
        pairs: Vec::new(),
        ring: None,
        covered,
        classes,
        datum: PierceDatum {
            normal: n_pierced,
            lever: pierced_lever,
        },
    };
    if runs.is_empty() {
        return Ok(out); // tangential touch: 3′ contact only, no surgery
    }
    if let Some(next) = reread {
        return Err(BooleanError::VertexReadTwice {
            operand: piercing,
            vertex,
            reads: [SectorRead::Pierce(contact.face), next],
        });
    }

    // Piercing-side null edges, one per run (PR 2's insertion pattern).
    // Germ facings (F9 data): the run's two boundary transitions are
    // its germs; both lie in the pierced face's TANGENT plane at `p`,
    // so the germ's face pair = (transition sector's face, pierced
    // face) in operand order, and its loci are the transition sector's
    // cell and the pierced face. Parity: the class after crossing forward
    // — Out at the run's start germ, In at its end germ (site-shared
    // with the ring strut below).
    // Every run's germs are read before any run is minted: a mint moves
    // the orbit the germ loci are read from.
    let germ_of = |t: usize| -> Result<Germ<T>, BooleanError> {
        let s = &sectors[t];
        let own = super::sectors::germ_locus(
            super::sectors::GermSide {
                body: piercing_body,
                operand: piercing,
                site: vertex,
                sector: s,
                read: (read[(t + 1) % n], read[t]),
            },
            contacts,
        )?;
        let pierced = super::Locus::InFace(contact.face);
        let cells = match piercing {
            Operand::A => ((s.face, contact.face), (own, pierced)),
            Operand::B => ((contact.face, s.face), (pierced, own)),
        };
        Ok((cells, pierce_germ_dir(s, n_pierced.vec(), band)?))
    };
    let run_germs = runs
        .iter()
        .map(|run| {
            Ok((
                germ_of((run.0 + n - 1) % n)?,
                germ_of((run.0 + run.1 - 1) % n)?,
            ))
        })
        .collect::<Result<Vec<_>, BooleanError>>()?;
    // The ring's struts (step 3, [`crate::null::ring_tree`]), read
    // before any write.
    let ring = if runs.len() > 1 {
        let germs: Vec<_> = runs
            .iter()
            .zip(&run_germs)
            .flat_map(|(run, (s, e))| {
                [
                    (s.1, sectors[(run.0 + n - 1) % n].arm),
                    (e.1, sectors[(run.0 + run.1 - 1) % n].arm),
                ]
            })
            .collect();
        let order = germ_order(&germs, n_pierced.vec(), band)?;
        crate::null::ring_tree(&order, crate::null::ring_root()).ok_or(
            BooleanError::ClassificationInvariant {
                what: "a pierce's runs read as crossing chords about the pierced face's normal",
            },
        )?
    } else {
        // One run: nothing to order, and either half may face either
        // germ, so no germ is read; its strut faces its end germ first.
        vec![crate::null::RingStrut {
            run: 0,
            parent: None,
            plus_faces_start: false,
        }]
    };
    let mut run_edges = Vec::new();
    for (run, &(start_germ, end_germ)) in runs.iter().zip(&run_germs) {
        let members = (0..run.1).map(|j| entries[(run.0 + j) % n]);
        let mut real = members.filter(|e| e.is_edge);
        let first = real.next();
        let last = real.next_back().or(first);
        // `strut_corner`: the site is an empty fan, so `mev_null` splices
        // the null edge as a spike [he_plus, he_minus] into this corner: a
        // run holding every real edge of the orbit, which leaves the In
        // side strictly inside the one physical sector before `first`,
        // or a lone bisector, inside the sector before `after`.
        let (site, strut_corner) = match (first, last) {
            (Some(first), Some(last)) => {
                match piercing_body
                    .run_site(first.he, last.he)
                    .unwrap_or_else(|| {
                        unreachable!(
                            "the run {:?} ..= {:?} at {vertex:?} has no fan end: its keys are the \
                         sector walk's, and {WALKS_CLOSE}",
                            first.he, last.he
                        )
                    }) {
                    site @ RunSite::Fan { .. } => (site.mev_site(), None),
                    site @ RunSite::WholeOrbit { corner } => (site.mev_site(), Some(corner)),
                }
            }
            _ => {
                // The run is one bisector entry, P's second piece (a
                // sector has one bisector at most), so `after` is P's
                // orbit successor Q, and runs being maximal, neither
                // P's real entry nor Q's is Out. The other run's mint
                // moves only Out real halves, and splices only before
                // its first half, the successor of its last, or its
                // own `after`, whose predecessor is another sector:
                // none is Q. So the corner is still `after`'s at
                // `vertex` whichever run mints first, and the table's
                // read stands (minting struts first would reorder
                // `out.edges` and the ring struts with them).
                let after = entries[(run.0 + run.1) % n].he;
                if piercing_body.proven_orbit_step(sectors[run.0].he) != after
                    || proven(&piercing_body.half_edges, after, EntityId::HalfEdge).start != vertex
                {
                    unreachable!(
                        "the bisector run {run:?}'s corner before {after:?} left {vertex:?} or \
                         gained a half: the other run's mint touches neither"
                    );
                }
                (
                    MevSite::Fan {
                        he1: after,
                        he2: after,
                    },
                    Some(after),
                )
            }
        };
        // Whether he_plus (old → copy) faces the run's start germ: a fan
        // puts it at the start germ's cut, and a strut faces its germs
        // by the one facing rule ([`super::insert::strut_faces_first`]).
        let start_on_plus = match strut_corner {
            None => true,
            Some(corner) => {
                let corner_half = proven(&piercing_body.half_edges, corner, EntityId::HalfEdge);
                let arrival = linked(
                    &piercing_body.half_edges,
                    corner_half.prev,
                    EntityId::HalfEdge,
                    EntityId::HalfEdge(corner),
                    "prev",
                )
                .edge;
                let germ = |t: usize, (cells, dir): Germ<T>| (t, cells, dir);
                let facing = super::insert::strut_faces_first(
                    piercing_body,
                    (piercing, &sectors),
                    (arrival, corner_half.edge),
                    germ((run.0 + n - 1) % n, start_germ),
                    germ((run.0 + run.1 - 1) % n, end_germ),
                    band,
                )?;
                // At a closed edge's lone vertex with a germ along it the
                // edge names no half. No row reaches the pose (a section
                // along a closed piercing edge puts the whole edge on the
                // pierced face), so no answer here is checked: refused.
                // An answer needs a row that reaches it, with an oracle
                // independent of the facing rule.
                facing.ok_or(BooleanError::CurvedBooleanUnsupported {
                    operand: pierced_op,
                    face: contact.face,
                    kind: pierced_kind,
                })?
            }
        };
        // The piercing run is Out.
        let mint = piercing_body.mev_null_run(site, vertex, NewVertexSide::Above, start_on_plus)?;
        let (created, attr, [start_he, end_he]) = (mint.created, mint.attr, mint.halves);
        let rec = BoolNullEdgeRecord {
            operand: piercing,
            at_vertex: vertex,
            edge: created.edge,
            attr,
            dangling: strut_corner.is_some(),
            germs: [half_germ(start_he, start_germ), half_germ(end_he, end_germ)],
        };
        run_edges.push(rec);
        out.edges.push(rec);
    }

    // Delta 3: the pierce ring in the pierced face (module docs).
    let pierced = pierced_op;
    let face_data = proven(&pierced_body.faces, contact.face, EntityId::Face);
    let crate::entity::LoopBoundary::Cycle { first: anchor } = linked(
        &pierced_body.loops,
        face_data.outer,
        EntityId::Loop,
        EntityId::Face(contact.face),
        "outer",
    )
    .boundary
    else {
        return Err(BooleanError::ClassificationInvariant {
            what: "pierced face outer loop is not a cycle",
        });
    };
    let u = linked(
        &pierced_body.half_edges,
        anchor,
        EntityId::HalfEdge,
        EntityId::Loop(face_data.outer),
        "first",
    )
    .start;
    let p_u = pierced_body.linked_vertex_point(u, EntityId::HalfEdge(anchor), "start");
    // (1) chord strut u → pierce point (certified line, transient).
    let chord = pierced_body.mev(
        MevSite::Fan {
            he1: anchor,
            he2: anchor,
        },
        p,
        geom_brep::EdgeCurveSpec::line_between(p_u, p),
        tol,
    )?;
    // (2) detach as an empty-loop ring at the pierce vertex.
    let kemr = pierced_body.kemr(chord.he_plus, chord.he_minus)?;
    let w = chord.vertex;
    out.ring = Some(PierceRingRecord {
        operand: pierced,
        face: contact.face,
        ring_vertex: w,
    });
    // (3) one ring null-edge strut per piercing-side run, hung at the
    // ring vertex or at another strut's far end, as the tree of the
    // ring's corners says ([`crate::null::ring_tree`]). A strut hangs at
    // its node just before the half leaving it towards the root, after
    // the node's earlier struts: at the ring vertex, before the first
    // strut's leaving half. The half leaving its node faces the germ its
    // run's corner meets first clockwise; with one run either half may
    // face either germ. The op does not enter. Where the struts leave
    // both operands one vertex at a pinch, `zip::split_cones` splits it
    // per cone before the zips.
    let mut root_anchor: Option<HalfEdgeKey> = None;
    // Per run: its strut's far end, and the half leaving it towards
    // the root.
    let mut nodes: BTreeMap<usize, (VertexKey, HalfEdgeKey)> = BTreeMap::new();
    for strut in &ring {
        let (i, leaving_faces_start) = (strut.run, strut.plus_faces_start);
        let (run_edge, &(start_germ, end_germ)) = (&run_edges[i], &run_germs[i]);
        let (at, site) = match strut.parent {
            None => (
                w,
                match root_anchor {
                    None => MevSite::Lone { r#loop: kemr.ring },
                    Some(he) => MevSite::Fan { he1: he, he2: he },
                },
            ),
            Some(parent) => {
                let &(node, back) =
                    nodes
                        .get(&parent)
                        .ok_or(BooleanError::ClassificationInvariant {
                            what: "a ring strut's parent is minted after it",
                        })?;
                (
                    node,
                    MevSite::Fan {
                        he1: back,
                        he2: back,
                    },
                )
            }
        };
        // The pierced run is In; the half leaving the strut's node is
        // he_plus.
        let mint =
            pierced_body.mev_null_run(site, at, NewVertexSide::Below, leaving_faces_start)?;
        let (created, attr, [start_he, end_he]) = (mint.created, mint.attr, mint.halves);
        if strut.parent.is_none() {
            root_anchor.get_or_insert(created.he_plus);
        }
        nodes.insert(i, (created.vertex, created.he_minus));
        let rec = BoolNullEdgeRecord {
            operand: pierced,
            at_vertex: at,
            edge: created.edge,
            attr,
            dangling: true,
            germs: [half_germ(start_he, start_germ), half_germ(end_he, end_germ)],
        };
        out.edges.push(rec);
        let (a_edge, b_edge, site) = match piercing {
            Operand::A => (
                run_edge.edge,
                created.edge,
                PairSite::VertexAOnFaceB(contact),
            ),
            Operand::B => (
                created.edge,
                run_edge.edge,
                PairSite::VertexBOnFaceA(contact),
            ),
        };
        out.pairs.push(NullEdgePairRecord {
            a_edge,
            b_edge,
            site,
        });
    }
    Ok(out)
}

/// **The germs' order round the ring vertex**: indices into `germs`
/// (each a direction and its arm), clockwise about the pierced face's
/// outward `normal` from `germs[0]`. Every comparison is
/// [`super::insert::strut_order`]'s from `germs[0]`, levered at the
/// shortest of its three germs' arms, and a germ along `germs[0]` has no
/// place before or after it: its side of `germs[0]` read in band
/// (`bool_strut_side`, its sense along) refuses as the sectors'
/// coincidence. Any reading that does not decide refuses; nothing orders
/// the germs otherwise.
fn germ_order<T: Decide>(
    germs: &[(Vec3<T>, T)],
    normal: Vec3<T>,
    band: Band,
) -> Result<Vec<usize>, BooleanError> {
    let (from, from_arm) = germs[0];
    let mut order = vec![0];
    for (i, &(g, g_arm)) in germs.iter().enumerate().skip(1) {
        let arm = from_arm.min(g_arm);
        let side = Margin::levered(g.cross(from).dot(normal), arm);
        if let Err(diag) = crate::validate::decide_nonzero("bool_strut_side", side, band)
            && super::sectors::direction_sense(g, from, arm, band)?
        {
            return Err(BooleanError::coincidence(
                Coincide::Sectors,
                DeclarationRead::Moot,
                diag,
            ));
        }
        let mut at = order.len();
        for (slot, &j) in order.iter().enumerate().skip(1) {
            let (h, h_arm) = germs[j];
            if super::insert::strut_order(
                from,
                normal,
                (g, h),
                from_arm.min(g_arm).min(h_arm),
                band,
            )? {
                at = slot;
                break;
            }
        }
        order.insert(at, i);
    }
    Ok(order)
}

/// On-edge resolution (module docs; the deliberate divergence), after
/// Delta 2's lumps.
///
/// An edge entry's On is a real one for a LINE edge: its far vertex is
/// within the band of the plane (`side_code`, `Reach::Chord`), so the
/// edge lies in it to the tolerance and the resolution is ε-true of it.
/// A curved edge's On is its departure's, to first order
/// (`Reach::Extent`; the residue is
/// `work/contact/boolean-conic-side-code-zero-is-first-order`).
///
/// A bisector entry's On is a direction's, levered at its sector's arm,
/// and it is resolved only where its code changes no topology. Its two
/// neighbours are its physical sector's real bounds. Mixed, the
/// bisector's code only picks which twin of that one sector holds the
/// transition; the fan it moves crosses no edge. Both definitely on one
/// side S (readings, not Delta 2 lumps), the bisector cannot read Zero
/// when K > 2: with `a` the arm, which is the shorter bound's length,
/// and `s` that bound's slope, the bound's own reading gives
/// `s·a ≥ K·zero`. A reflex bisector `−(â + b̂)/|â + b̂|` then reads at
/// least `(s_a + s_b)·a/2 ≥ K·zero/2`. A straight-band one, `n × b̂`,
/// reads at least `a·cos δ` (δ, the deviation from π, has `sin δ·a`
/// inside the band). So a Zero there is the ambiguity band's (K ≤ 2),
/// and the reflex case would read the wrong side: it refuses
/// ([`super::sectors::bisector_zero_refusal`]) rather than resolve.
fn resolve_on_entries(entries: &mut [Entry], band: Band) -> Result<(), BooleanError> {
    let n = entries.len();
    for k in 0..n {
        if entries[k].class == SideCode::On && entries[(k + 1) % n].class == SideCode::On {
            return Err(BooleanError::ClassificationInvariant {
                what: "consecutive On entries after Eq. 15.3 lumping",
            });
        }
    }
    for k in 0..n {
        if entries[k].class != SideCode::On {
            continue;
        }
        let (before, after) = (&entries[(k + n - 1) % n], &entries[(k + 1) % n]);
        let (prev, next) = (before.class, after.class);
        if !entries[k].is_edge && !before.lumped && !after.lumped && prev == next {
            return Err(super::sectors::bisector_zero_refusal(band));
        }
        entries[k].class = super::sectors::fold_on_bound(prev, next);
    }
    Ok(())
}

/// The germ direction at a pierce-site transition: the unit
/// intersection direction of the transition sector's face plane with
/// the pierced face's TANGENT plane at the pierce point, signed to lie
/// within the sector (grazes count — an on-edge germ IS a bound).
/// Ambiguity refuses loudly.
///
/// **Why the tangent plane is the exact datum, not an approximation of
/// one.** A germ direction is the initial direction of the curve along
/// which the two faces meet, taken AT `p`. That curve is the section of
/// the sector's carrier with the pierced carrier, and the tangent of a
/// section conic at a point on it is the intersection LINE of the two
/// surfaces' tangent planes there — a first-order statement about a
/// first-order object, exact for every smooth pair, not a linearization
/// with an error term. A plane is its own tangent plane everywhere, so
/// the planar lane is the special case where `pierced_normal` happens
/// not to vary with `p`; substituting the per-point normal computes the
/// same cross product on a curved carrier and gets the section's
/// tangent rather than a chord.
///
/// Sense-invariant given its sources (S10): the cross product names a
/// LINE and `within` picks the ray, so neither normal's sign survives
/// into the answer. Both arrive oriented already — `s.normal` from
/// `sectors::sector_face`, `pierced_normal` from the one door
/// ([`crate::face_normal`]) — and neither is multiplied again here.
pub(super) fn pierce_germ_dir<T: Decide>(
    s: &super::sectors::BoolSector<T>,
    pierced_normal: geom_core::Vec3<T>,
    band: Band,
) -> Result<geom_core::Vec3<T>, BooleanError> {
    let int = s.normal.vec().cross(pierced_normal);
    // Levered at the sector's farther reach. A transition sector has a
    // bound read definitely off the pierced plane, beyond `K·zero`, at
    // its reach `L`. That reading is at most `L·|n_s × n_p|` plus the
    // departures of the vertices it is taken between from the sector's
    // own plane, each up to `zero` in a valid body, so this gate reads
    // at least `(K − 2)·zero`: never zero, but in band where the reading
    // lies within `2·zero` of the band's edge and the vertices stand off
    // their face by as much. That is the sector's tilt against the
    // pierced plane undecided, which a smaller tolerance decides.
    match decide(
        "bool_germ_line",
        Margin::levered(int.norm(), s.span()),
        band,
    ) {
        Ok(Sign::Positive) => {}
        Ok(_) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "pierce transition on a coplanar sector",
            });
        }
        Err(diag) => {
            return Err(BooleanError::coincidence(
                Coincide::Sectors,
                DeclarationRead::Moot,
                diag,
            ));
        }
    }
    let d = int.normalize();
    let moot = DeclarationRead::Moot;
    let plus = super::sectors::within(s, d, false, moot, band)?;
    let minus = super::sectors::within(s, -d, false, moot, band)?;
    match (plus, minus) {
        (true, false) => Ok(d),
        (false, true) => Ok(-d),
        _ => Err(BooleanError::ClassificationInvariant {
            what: "pierce germ direction not uniquely within its sector",
        }),
    }
}

/// A pierce germ as a run reads it: `(A face, B face)` and
/// `(A locus, B locus)`, then its direction.
type Germ<T> = (
    (
        (crate::entity::FaceKey, crate::entity::FaceKey),
        (super::Locus, super::Locus),
    ),
    geom_core::Vec3<T>,
);

fn half_germ<T: geom_core::Real>(
    he: HalfEdgeKey,
    ((faces, loci), dir): Germ<T>,
) -> super::HalfGerm<T> {
    super::HalfGerm {
        he,
        a_face: faces.0,
        b_face: faces.1,
        a_locus: loci.0,
        b_locus: loci.1,
        dir,
    }
}

/// Maximal cyclic Out-runs `(start, len)` (PR 2's `above_runs` on
/// [`SideCode`]; anchored at the first In entry; one-sided
/// neighborhoods have none).
fn out_runs(entries: &[Entry]) -> Vec<(usize, usize)> {
    let n = entries.len();
    let Some(anchor) = entries.iter().position(|e| e.class == SideCode::In) else {
        return Vec::new();
    };
    let mut runs = Vec::new();
    let mut k = 0;
    while k < n {
        let idx = (anchor + 1 + k) % n;
        if entries[idx].class == SideCode::Out {
            let start = idx;
            let mut len = 0;
            while k < n && entries[(anchor + 1 + k) % n].class == SideCode::Out {
                len += 1;
                k += 1;
            }
            runs.push((start, len));
        } else {
            k += 1;
        }
    }
    runs
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use SideCode::{In, On, Out};

    /// A vertex that pierces two faces is refused, naming both, on
    /// either operand; one that pierces a face and pairs is returned on
    /// its operand with its first pair; a vertex in several pairs and
    /// nowhere pierced is neither, nor are two operands' vertices that
    /// share a key.
    #[test]
    fn a_pierced_vertex_is_refused_at_a_second_pierce_and_returned_at_a_pair() {
        use super::super::VvContact;
        use crate::entity::FaceKey;
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let (v, w, x) = (
            VertexKey::from(key(1)),
            VertexKey::from(key(2)),
            VertexKey::from(key(3)),
        );
        let (f, g) = (FaceKey::from(key(4)), FaceKey::from(key(5)));
        let vf = |vertex, face| VfContact { vertex, face };
        let vv = |a, b| VvContact { a, b };
        let read = |c: &ContactRecords| match pierced_and_paired(c) {
            Err(BooleanError::VertexReadTwice {
                operand,
                vertex,
                reads,
            }) => Err((operand, vertex, reads)),
            Ok(rereads) => Ok(rereads.into_iter().collect::<Vec<_>>()),
            Err(e) => panic!("{e:?}"),
        };
        let rows = [
            (
                "pierced twice by A",
                ContactRecords {
                    a_on_b: vec![vf(v, f), vf(w, f), vf(v, g)],
                    ..Default::default()
                },
                Err((
                    Operand::A,
                    v,
                    [SectorRead::Pierce(f), SectorRead::Pierce(g)],
                )),
            ),
            (
                "pierced and paired, B",
                ContactRecords {
                    b_on_a: vec![vf(w, f)],
                    vv: vec![vv(x, v), vv(v, w), vv(x, w)],
                    ..Default::default()
                },
                Ok(vec![((Operand::B, w), SectorRead::Pair(v))]),
            ),
            (
                "paired twice",
                ContactRecords {
                    vv: vec![vv(v, w), vv(v, x), vv(x, w)],
                    ..Default::default()
                },
                Ok(vec![]),
            ),
            (
                "one key on each operand, pierced by A's and paired by B's",
                ContactRecords {
                    a_on_b: vec![vf(v, f)],
                    vv: vec![vv(w, v)],
                    ..Default::default()
                },
                Ok(vec![]),
            ),
        ];
        for (what, c, want) in rows {
            assert_eq!(read(&c), want, "{what}");
        }
    }

    /// A touching edge reads its own side's partners, layered over its
    /// pierce's class: flipped once per cone that holds it, a met cone
    /// holding what it reads In and a hollow one (read through its
    /// complement) what it reads Out; on one partner's boundary, on the
    /// solid's, at any depth. On two boundaries, or beside a partner
    /// that read nothing, it names a partner to refuse on. Each input is
    /// a partner a scene of `a_vertex_read_by_two_sector_passes` reads:
    /// an arch, a void in the arch, the cavity's void, an island in it,
    /// arches apart, arches sharing a ray, and a dart. One pair alone
    /// keeps its rows.
    #[test]
    fn a_vertex_read_again_layers_its_partners() {
        use super::super::sectors::WedgeRead;
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let (e, f, g) = (
            HalfEdgeKey::from(key(1)),
            HalfEdgeKey::from(key(2)),
            HalfEdgeKey::from(key(3)),
        );
        let (w, x) = (VertexKey::from(key(4)), VertexKey::from(key(5)));
        let read = |partner, side, met: bool, rows: &[(HalfEdgeKey, SideCode)]| PairRead::<f64> {
            partner,
            side,
            read: Some(WedgeRead {
                met,
                rows: rows.to_vec(),
            }),
            sectors: Vec::new(),
        };
        let unread = |partner, side| PairRead::<f64> {
            partner,
            side,
            read: None,
            sectors: Vec::new(),
        };
        let three = [(e, In), (f, Out), (g, On)];
        let touches = [
            (
                "Out, beside an arch",
                vec![(e, Out), (f, Out), (g, On)],
                vec![read(w, Some(Out), true, &three)],
                Ok(vec![(e, In), (f, Out), (g, On)]),
            ),
            (
                "Out, beside the cavity's void",
                vec![(e, Out), (f, Out)],
                vec![read(w, Some(In), false, &three)],
                Ok(vec![(e, Out), (f, Out)]),
            ),
            (
                "In, beside the cavity's void",
                vec![(e, In), (f, In), (g, In)],
                vec![read(w, Some(In), false, &three)],
                Ok(vec![(e, In), (f, Out), (g, On)]),
            ),
            (
                "In, beside the void and the island in it",
                vec![(e, In), (f, In), (g, In)],
                vec![
                    read(w, Some(In), false, &[(e, Out), (f, Out), (g, Out)]),
                    read(x, Some(In), true, &[(e, In), (f, On), (g, Out)]),
                ],
                Ok(vec![(e, In), (f, On), (g, Out)]),
            ),
            (
                "Out, beside two arches apart",
                vec![(e, Out), (f, Out)],
                vec![
                    read(w, Some(Out), true, &[(e, In), (f, Out)]),
                    read(x, Some(Out), true, &[(e, Out), (f, Out)]),
                ],
                Ok(vec![(e, In), (f, Out)]),
            ),
            (
                "Out, beside the arch and the void in it, on the void's face",
                vec![(e, Out), (f, Out), (g, Out)],
                vec![
                    read(w, Some(Out), true, &[(e, In), (f, In), (g, In)]),
                    read(x, Some(Out), false, &[(e, Out), (f, In), (g, On)]),
                ],
                Ok(vec![(e, Out), (f, In), (g, On)]),
            ),
            (
                "on two arches' shared ray",
                vec![(e, Out), (g, Out)],
                vec![
                    read(w, Some(Out), true, &[(e, Out), (g, On)]),
                    read(x, Some(Out), true, &[(e, Out), (g, On)]),
                ],
                Err(Some(w)),
            ),
            (
                "beside a dart, which reads nothing",
                vec![(e, On), (f, Out)],
                vec![read(w, Some(Out), true, &three), unread(x, Some(Out))],
                Err(Some(x)),
            ),
        ];
        for (what, touch, pairs, want) in touches {
            assert_eq!(touch_classes(&touch, &pairs), want, "{what}");
        }
        let band = Band::linear(Tol::witness()).unwrap();
        for (what, pairs, want) in [
            (
                "one arch",
                vec![read(w, None, true, &three)],
                three.to_vec(),
            ),
            (
                "one void's apex",
                vec![read(w, None, false, &three)],
                three.to_vec(),
            ),
        ] {
            assert_eq!(pair_classes(&pairs, band).unwrap(), want, "{what}");
        }
    }

    fn entry(is_edge: bool, class: SideCode) -> Entry {
        Entry {
            he: HalfEdgeKey::default(),
            is_edge,
            class,
            lumped: false,
        }
    }

    /// A bisector reading On between two bounds read Out is the band's
    /// Zero (reachable only at K ≤ 2, which no suite runs), and it
    /// refuses; an edge's On there, a real one, resolves; a bisector
    /// between mixed neighbours resolves, since its code moves no edge.
    #[test]
    fn a_bisector_on_between_one_sided_bounds_refuses() {
        let band = Band::linear(Tol::witness()).unwrap();
        let mut one_sided = [entry(true, Out), entry(false, On), entry(true, Out)];
        assert!(
            matches!(
                resolve_on_entries(&mut one_sided, band),
                Err(BooleanError::Escalated {
                    decision: super::super::BooleanDecision::BisectorSide,
                    diag,
                }) if diag.predicate == Some("bool_sector_bisector_side")
            ),
            "the bisector's Zero refuses as its own decision"
        );
        let mut edge = [entry(true, Out), entry(true, On), entry(true, Out)];
        resolve_on_entries(&mut edge, band).unwrap();
        assert_eq!(edge[1].class, Out, "an edge's On resolves");
        let mut mixed = [entry(true, Out), entry(false, On), entry(true, In)];
        resolve_on_entries(&mut mixed, band).unwrap();
        assert_eq!(
            mixed[1].class, In,
            "a bisector between mixed bounds resolves"
        );
    }

    /// **An in-band pierce germ line is the sector's tilt undecided**,
    /// not the kernel's: a transition sector reads it at least
    /// `(K − 2)·zero` (its vertices may stand off their face by up to
    /// the band), so a sector whose face parts from the pierced face by
    /// an in-band angle at its reach escalates as the corners' overlap,
    /// with its lever and the tolerance its margin gives.
    #[test]
    fn an_in_band_pierce_germ_line_is_the_sectors_tilt_undecided() {
        use super::super::sectors::{BoolSector, Reach};
        use geom_brep::OutwardNormal;
        use geom_core::{KERNEL_DEFECT_ENDING, Point3, Vec3};
        let band = Band::linear(Tol::witness()).unwrap();
        let mid = (band.zero() + band.escalate()) / 2.0;
        let o = Point3::new(0.0, 0.0, 0.0);
        let (x, y) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let s = BoolSector {
            he: HalfEdgeKey::default(),
            start: x,
            end: y,
            start_reach: Reach::Chord {
                base: o,
                far: o + x,
            },
            end_reach: Reach::Chord {
                base: o,
                far: o + y,
            },
            face: crate::entity::FaceKey::default(),
            normal: OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true),
            arm: 1.0,
        };
        let err =
            pierce_germ_dir(&s, Vec3::new(mid.sin(), 0.0, mid.cos()), band).expect_err("in band");
        assert!(
            matches!(
                err,
                BooleanError::Escalated {
                    decision: super::super::BooleanDecision::Coincidence(
                        Coincide::Sectors,
                        DeclarationRead::Moot
                    ),
                    ..
                }
            ),
            "{err:?}"
        );
        let text = err.to_string();
        assert!(
            !text.contains(KERNEL_DEFECT_ENDING) && text.contains("tighten the tolerance below"),
            "{text}"
        );
    }

    /// A germ on the unit circle of `z = 0` at `deg` clockwise about +z,
    /// with a unit arm.
    fn germ_at(deg: f64) -> (Vec3<f64>, f64) {
        let t = -deg.to_radians();
        (Vec3::new(t.cos(), t.sin(), 0.0), 1.0)
    }

    /// Whether `r` is the sectors' coincidence escalated, which a
    /// smaller tolerance decides.
    fn sectors_coincide(r: &Result<Vec<usize>, BooleanError>) -> bool {
        matches!(
            r,
            Err(BooleanError::Escalated {
                decision: super::super::BooleanDecision::Coincidence(Coincide::Sectors, _),
                ..
            })
        )
    }

    /// **A germ along the ring's first germ refuses as the sectors'
    /// coincidence, as does a pair along each other**, where `d` is in
    /// band and the germs are ordered where it is decided:
    /// - a star, two runs side by side, run 1's end `d` before run 0's
    ///   start (`[0, 1, 2, 3]`, every strut facing its start);
    /// - a star, run 1 nested under run 0, run 0's end `d` before its
    ///   own start (`[0, 3, 2, 1]`, every strut facing its end);
    /// - a path, `meeting::arch`'s germs mirrored, run 1's end `d` before
    ///   run 0's start (`[0, 1, 2, 5, 4, 3]`: run 1 between the others);
    /// - two germs neither of them the first, `d` apart: no order of
    ///   them is read in band.
    #[test]
    fn germs_in_band_of_each_other_refuse_as_the_sectors_coincidence() {
        let band = Band::linear(Tol::witness()).unwrap();
        let n = Vec3::new(0.0, 0.0, 1.0);
        type Row = (&'static str, fn(f64) -> Vec<f64>, Vec<usize>);
        let rows: [Row; 4] = [
            (
                "side by side",
                |d| vec![0.0, 90.0, 180.0, 360.0 - d],
                vec![0, 1, 2, 3],
            ),
            (
                "nested under one",
                |d| vec![30.0, 30.0 - d, 200.0, 100.0],
                vec![0, 3, 2, 1],
            ),
            (
                "the arch",
                |d| vec![30.0, 120.0, 150.0, 30.0 - d, 270.0, 240.0],
                vec![0, 1, 2, 5, 4, 3],
            ),
            (
                "two apart from the first",
                |d| vec![0.0, 90.0, 90.0 + d, 200.0],
                vec![0, 1, 2, 3],
            ),
        ];
        for (what, degs, decided) in rows {
            let germs = |d: f64| degs(d).into_iter().map(germ_at).collect::<Vec<_>>();
            assert_eq!(
                germ_order(&germs(1.0), n, band).ok(),
                Some(decided),
                "{what}, 1° apart"
            );
            for d in [1e-10, 1e-12] {
                let r = germ_order(&germs(d), n, band);
                assert!(sectors_coincide(&r), "{what}, {d}° apart: {r:?}");
            }
        }
    }

    /// PROBE (review 2): a germ decided apart from germ 0 at its own
    /// arm, but in the zero band at a third germ's shorter arm, is
    /// placed by its sense inside `strut_order` (`Sign::Zero` arm).
    #[test]
    fn probe_review2_a_shorter_third_arm_reads_a_decided_germ_as_zero() {
        let band = Band::linear(Tol::witness()).unwrap();
        let n = Vec3::new(0.0, 0.0, 1.0);
        let at = |deg: f64, arm: f64| (germ_at(deg).0, arm);
        let d = 2e-8f64.to_degrees();
        // Truth clockwise from germ 0: 120°, then 360° - d.
        let a = germ_order(&[at(0.0, 1.0), at(120.0, 0.01), at(360.0 - d, 1.0)], n, band);
        let b = germ_order(&[at(0.0, 1.0), at(360.0 - d, 1.0), at(120.0, 0.01)], n, band);
        // Control: all arms 1.
        let c = germ_order(&[at(0.0, 1.0), at(120.0, 1.0), at(360.0 - d, 1.0)], n, band);
        eprintln!("PROBE short-arm-third: a={a:?} b={b:?} control={c:?}");
        assert_eq!(c.as_ref().ok(), Some(&vec![0, 1, 2]), "control");
        for (what, r, truth) in [("a", a, vec![0, 1, 2]), ("b", b, vec![0, 2, 1])] {
            match r {
                Ok(o) => assert_eq!(o, truth, "{what}: placed by fiat, not refused"),
                Err(e) => eprintln!("{what}: refused {e:?}"),
            }
        }
    }

    /// **A bisector run minted after the other run hangs its strut in
    /// its own corner.** The L-prism's reflex corner pierces a cube's
    /// face in two Out runs, one of them the reflex sector's bisector
    /// alone; which run mints first follows the orbit's start. One of
    /// the corner's three starts mints the bisector run second (read
    /// off the classification's records); at every start, every op in
    /// both orders builds. The teeth are those builds and the arm's
    /// `unreachable!`: a corner moved by the other run panics there.
    /// The volumes (equal across starts, inclusion-exclusion) are a
    /// consistency check no mutant of the arm has reached.
    #[test]
    fn a_bisector_run_after_the_other_run_keeps_its_corner() {
        use crate::test_support_fixtures::{mapped_cube, prism};
        use crate::{AtRestBody, BooleanDeclarations, mass_properties};
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let profile = [
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ];
        // The cube of side 4 whose near face holds the corner `(1, 1, 1)`
        // at its centre, its normal tilted 0.05 rad up from the
        // horizontal at 0.37 of a twelfth-turn: two Out runs.
        let (theta, phi) = (std::f64::consts::TAU * 0.37 / 12.0, 0.05_f64);
        let m = [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()];
        let cube = {
            let u = {
                let c = [-m[1], m[0], 0.0];
                let l = (c[0] * c[0] + c[1] * c[1]).sqrt();
                [c[0] / l, c[1] / l, 0.0]
            };
            let w = [
                m[1] * u[2] - m[2] * u[1],
                m[2] * u[0] - m[0] * u[2],
                m[0] * u[1] - m[1] * u[0],
            ];
            mapped_cube::<f64>(
                move |x, y, z| {
                    let (a, b, c) = (4.0 * x - 2.0, 4.0 * y - 2.0, 4.0 * z);
                    let at = |i: usize| 1.0 + a * u[i] + b * w[i] + c * m[i];
                    geom_core::Point3::new(at(0), at(1), at(2))
                },
                tol,
            )
        };
        let near = cube
            .faces()
            .map(|(k, _)| k)
            .find(|&f| {
                face_plane(&cube, f).is_some_and(|p| {
                    p.normal.x * m[0] + p.normal.y * m[1] + p.normal.z * m[2] < -0.99
                })
            })
            .expect("the cube's near face, outward along -m");
        let cube_body = cube.clone();
        let cube = AtRestBody::validate(cube, tol).unwrap();
        let decls = BooleanDeclarations::default();
        let built = prism::<f64>(&profile, 1.0, tol);
        let corner = built.top[3];
        let orbit = built.body.vertex_orbit_linked(corner);
        assert_eq!(orbit.len(), 3, "the reflex corner is trivalent");
        let mut volumes = Vec::new();
        let mut orders = Vec::new();
        for start in &orbit {
            let mut body = built.body.clone();
            body.get_vertex_mut(corner).unwrap().emanating = Some(*start);
            let classified = classify_vertex_on_face(
                &mut body.clone(),
                &mut cube_body.clone(),
                Operand::A,
                super::super::VfContact {
                    vertex: corner,
                    face: near,
                },
                super::super::BooleanOp::Union,
                &super::super::DeclaredPairs::default(),
                &super::super::ContactRecords::default(),
                None,
                band,
                tol,
            )
            .unwrap();
            orders.push(
                classified
                    .edges
                    .iter()
                    .filter(|e| e.operand == Operand::A)
                    .map(|e| e.dangling)
                    .collect::<Vec<_>>(),
            );
            let body = AtRestBody::validate(body, tol).unwrap();
            let volume = |r: Result<crate::BooleanResult<f64>, BooleanError>| {
                r.unwrap()
                    .body()
                    .map_or(0.0, |b| mass_properties(&b.body, tol).unwrap().volume)
            };
            volumes.push([
                volume(crate::union_with(&body, &cube, &decls, tol)),
                volume(crate::intersect_with(&body, &cube, &decls, tol)),
                volume(crate::subtract_with(&body, &cube, &decls, tol)),
                volume(crate::union_with(&cube, &body, &decls, tol)),
                volume(crate::intersect_with(&cube, &body, &decls, tol)),
                volume(crate::subtract_with(&cube, &body, &decls, tol)),
            ]);
        }
        let fan_first = vec![false, true];
        assert!(
            orders.iter().filter(|&o| *o == fan_first).count() == 1
                && orders
                    .iter()
                    .all(|o| *o == fan_first || *o == [true, false]),
            "two runs at every start, the bisector's strut minted second at exactly one: \
             {orders:?}"
        );
        let close = |x: f64, y: f64| (x - y).abs() < 1e-9;
        for (k, v) in volumes.iter().enumerate() {
            let [u, i, s, cu, ci, cs] = *v;
            assert!(
                close(u + i, 67.0) && close(cu, u) && close(ci, i),
                "orbit start {k}: union and intersection keep 3 + 64 in both orders: {v:?}"
            );
            assert!(
                close(s, 3.0 - i) && close(cs, 64.0 - i) && i > 0.0 && i < 3.0,
                "orbit start {k}: each difference is its minuend less the intersection: {v:?}"
            );
            assert!(
                v.iter().zip(&volumes[0]).all(|(a, b)| close(*a, *b)),
                "orbit start {k} answers as orbit start 0: {v:?} against {:?}",
                volumes[0]
            );
        }
    }
}
