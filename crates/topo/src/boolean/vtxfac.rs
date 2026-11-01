//! `vtxfacclassify` — the vertex-on-face classifier the book never
//! prints ("for space reasons", §15.6.1): the ch. 14 classifier with
//! the three deltas, DESIGNED here:
//!
//! 1. **datum := the pierced face's TANGENT plane at the pierce
//!    point**; classes are [`SideCode`]s with OUT/IN derived from the
//!    F3 chain (a chord's class is its signed elevation off that plane
//!    against the OUTWARD normal — `sectors::pierce_side`; 15.7's
//!    printed `IN = +1` is never consulted).
//!
//!    The datum is a DIRECTION, never an origin: `pierce_side`
//!    (`sectors.rs`) takes `(dir, reach, OutwardNormal, shape, lever,
//!    band)` and the germ direction takes a cross product. Both are
//!    first-order readings (charged for the face's curvature through
//!    `lever`), except that a conic tied with a plane face is read by
//!    its bend, one order down; and both generalize to a curved pierced
//!    face by substituting
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
//!    in the pierced body (and, for a pierce point inside a face of the
//!    piercing body's own, in that face too: [`faces_holding_pierce`]),
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
//!    end). One ring serves every pierce of the face at one point
//!    ([`ring_groups`], [`mint_ring`]): its tree holds a strut per run of
//!    each, and the pass mints it once every pierce is read. Dangling
//!    ring null edges are the documented ch. 15 transient; joining
//!    consumes them in PR 5. Tier 1 holds after every op (pinned by the
//!    acceptance test).
//! 4. **The node** ([`mint_node`], called from [`super::node`]): a
//!    crossing of an equal cylinder pair's two section ellipses inside
//!    a face of each operand has no operand vertex, so one is minted:
//!    `A`'s face gets a lone ring vertex with a strut per run, and that
//!    vertex then pierces `B`'s face through the same [`RingPlan`] as
//!    any other, its struts paired with `B`'s across the operands. The
//!    runs and their sides are fixed by construction once the walls'
//!    senses are read ([`mint_node`] states the rule).
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
//! carried by the contact records — TOG Table II rows 5/9
//! (`(In,In)`/`(Out,Out)` ⇒ no intersection) confirm no crossing is
//! recorded. Mixed keeps the In side (both witnesses' choice for the
//! split analogue).
//!
//! **Read twice.** A piercing vertex that also pairs with a vertex of
//! the other solid — which then holds its own contact at the point — is
//! read by both passes ([`pierced_and_paired`]). It is read again
//! only where its pierce touches the face, so its orbit is unwritten;
//! a pierce that would cross refuses before it writes, and so does a
//! second pierce. The pair's partner must lie on one side of the face,
//! meeting it along rays at most ([`partner_side`]). The vertex's edges
//! are then classed against the face and its partners together
//! ([`touch_classes`]): the partners' cones nest or lie apart on each
//! side, and crossing each boundary flips the side ([`layered`]); an
//! edge it cannot class refuses. A vertex in pairs alone layers them
//! the same way, its outermost cones read off which hold which
//! ([`pair_classes`]).

use geom_brep::OutwardNormal;
use geom_core::k_stats::NonzeroSign;
use geom_core::{Band, Decide, Margin, Sign, Vec3};

use super::contain::FaceContainment;
use super::plane_eq::PlaneEqError;
use super::reduce::face_plane;
use std::collections::BTreeMap;

use super::sectors::{BoolSector, PierceLever, build_sectors, pierce_side_reported};
use super::tables::eq15_3_lump;
use super::{AlongFace, Coincide, DeclarationRead};
use super::{
    BoolNullEdgeRecord, BooleanError, BooleanOp, NullEdgePairRecord, Operand, PairSite,
    PierceRingRecord, ReductionContacts, SectorRead, SideCode, VfContact,
};
use crate::body::{Body, WALKS_CLOSE};
use crate::contact::GlueClass;
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
    contacts: &ReductionContacts,
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
/// normal, its shape, and the lever its side verdicts charge
/// ([`super::sectors::pierce_side`]).
#[derive(Clone, Copy, Debug)]
pub(super) struct PierceDatum<T: geom_core::Real> {
    pub normal: OutwardNormal<T>,
    pub shape: super::sectors::FaceShape,
    pub lever: PierceLever<T>,
}

impl<T: Decide> PierceDatum<T> {
    /// A bound's side of the face, as the touch reads its partners
    /// ([`partner_side`]): its lever the face's within the bound's own
    /// reach ([`super::sectors::pierce_side`]).
    pub(super) fn side(
        self,
        dir: Vec3<T>,
        reach: super::sectors::Reach<T>,
        band: Band,
    ) -> Result<SideCode, BooleanError> {
        let lever = self.lever.within(reach.length());
        super::sectors::pierce_side(dir, reach, self.normal, self.shape, lever, band)
            .map(|read| read.code)
    }

    /// Whether the face holds a line along `dir`, read off its bound
    /// ([`Self::side`]): a decided zero holds it, a definite side does
    /// not, and a reading within the band refuses typed.
    pub(super) fn holds_line(
        self,
        dir: Vec3<T>,
        reach: super::sectors::Reach<T>,
        band: Band,
    ) -> Result<bool, BooleanError> {
        Ok(self.side(dir, reach, band)? == SideCode::On)
    }

    /// The refusal of a bound lying on the face where its side is
    /// second order: on a curved face, a bound in the tangent plane
    /// leaves it by the two carriers' relative curvature.
    pub(super) fn curved_along(self, on: bool) -> Option<AlongFace> {
        (on && self.shape == super::sectors::FaceShape::Curved).then_some(AlongFace::Curved)
    }
}

/// What [`partner_side`] read of a partner beside a touch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PartnerRead {
    /// Its link lies on `side` of the face, `along` it where a bound
    /// reads on a planar face.
    Side { side: SideCode, along: bool },
    /// Its bounds read both sides of the face.
    Across,
    /// Its link runs along the face where the side is not read.
    Along(AlongFace),
}

/// **The side of a pierced face a paired vertex's link lies on**, where
/// the piercing vertex only touches the face, read by the reading the
/// touch itself was decided by: the side every bound of `partner`'s
/// sectors reads, or reads on a planar face, where every sector has a
/// bound off it.
///
/// Each sector is a wedge under a half-turn, so the link lies in that
/// closed half-space and meets the face's plane only along the rays
/// read on it, on its cone's boundary; [`touch_classes`] layers the cone
/// as for a partner off the face.
///
/// Every bound is read before any verdict, so the verdict is the bounds'
/// whatever their order: a bound in band escalates (the first in orbit
/// order is quoted); else bounds on both sides read [`PartnerRead::Across`];
/// else a bound on a curved face, a sector on the face at both bounds,
/// or a bound on it beside a touch `flush` with the face, read
/// [`PartnerRead::Along`], in that order.
pub(super) fn partner_side<T: Decide>(
    partner: &[BoolSector<T>],
    datum: PierceDatum<T>,
    flush: bool,
    band: Band,
) -> Result<PartnerRead, BooleanError> {
    let mut codes = Vec::with_capacity(partner.len());
    for s in partner {
        codes.push([
            datum.side(s.start, s.start_reach, band)?,
            datum.side(s.end, s.end_reach, band)?,
        ]);
    }
    let mut side = None;
    for c in codes.iter().flatten().filter(|&&c| c != SideCode::On) {
        if side.is_some_and(|k| k != *c) {
            return Ok(PartnerRead::Across);
        }
        side = Some(*c);
    }
    let along = codes.iter().flatten().any(|&c| c == SideCode::On);
    let refused = datum.curved_along(along).or_else(|| {
        if codes.iter().any(|b| b.iter().all(|&c| c == SideCode::On)) {
            Some(AlongFace::FaceOnFace)
        } else {
            (along && flush).then_some(AlongFace::Flush)
        }
    });
    Ok(match (refused, side) {
        (Some(how), _) => PartnerRead::Along(how),
        (None, Some(side)) => PartnerRead::Side { side, along },
        (None, None) => PartnerRead::Across,
    })
}

/// One pair of a vertex read again: its partner, the partner's side of
/// the face the vertex touches ([`partner_side`]; `None` where it
/// touches none), what [`super::sectors::wedge_classes`] read against the
/// partner (`None` where it read nothing), and the partner's sectors.
#[derive(Debug, Clone)]
pub(super) struct PairRead<T: geom_core::Real> {
    pub partner: VertexKey,
    pub side: Option<SideCode>,
    /// The partner's reading, or why it read nothing.
    pub read: Result<super::sectors::WedgeRead, super::UnreadNeighbourhood>,
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
/// (joined) whose boundary meets the others' at the point, or touches
/// them along rays, never crossing them (beside a touch, along rays on
/// the touched face: [`partner_side`]), so the cones nest or lie
/// apart. Every face bounds the other solid's material on one side
/// only, so crossing a partner's boundary flips it. A partner's row
/// says on which side of its own boundary the edge
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
        let read = p.read.as_ref().ok()?;
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

/// The shape of a partner's face, read on the partners' body: what
/// [`along_partners`] needs to tell faces glued flush along an edge from
/// faces that may be tangent there. A face it cannot read refuses.
pub(super) type ShapeOf<'s> =
    &'s dyn Fn(crate::entity::FaceKey) -> Result<super::sectors::FaceShape, BooleanError>;

/// Why [`touch_classes`] leaves a touching vertex's edge undecided.
#[derive(Debug)]
pub(super) enum TouchUndecided {
    /// Beside this partner on the edge's side, which read nothing or
    /// whose cone lies in no half-space the reading decides
    /// (`WedgeRead::pointed`), or failing that the first there; `None`
    /// where no partner is on that side.
    Beside(Option<VertexKey>),
    /// The edge lies on two or more partners' boundaries, and
    /// [`along_partners`] refuses, or the vertex's own sectors name no
    /// edge for it, an invariant ([`partners_class`]).
    Along(BooleanError),
}

/// **The classes of a touching vertex's edges against the other solid**,
/// from its pierce's classes (`touch`), its pairs', and its own sectors
/// (`own`). An edge is left undecided ([`TouchUndecided`]) beside a
/// partner on its side that read nothing or whose cone lies in no
/// half-space the reading decides (`WedgeRead::pointed`), or where
/// the partners' faces along it refuse.
///
/// Near the point the other solid is the pierced face's half-space `H`
/// and the material of each partner, whose link lies in one closed
/// side of the face and meets its plane only along rays
/// ([`partner_side`]). Its cone is the side of the link in that
/// half-space, and meets the plane only along those rays, on its
/// boundary. An edge off the face is outside every cone on the other
/// side, so only its own side's partners read it, [`layered`] over its
/// pierce's class: outside `H` beside no cone, inside `H` beside no
/// void. An edge on two or more of those partners' boundaries reads
/// [`along_partners`], as [`pair_classes`]' does: off the face, `H` is
/// on one side of the edge everywhere around it, so the material turns
/// where the partners' does. An edge on the face is on the solid's
/// boundary, along a partner's ray or not: `H` is on one side of it and
/// outside every cone on the other, so no partner reads it.
pub(super) fn touch_classes<T: Decide>(
    own: &[BoolSector<T>],
    touch: &[(HalfEdgeKey, SideCode)],
    pairs: &[PairRead<T>],
    shape: ShapeOf<'_>,
    band: Band,
) -> Result<Vec<(HalfEdgeKey, SideCode)>, TouchUndecided> {
    touch
        .iter()
        .map(|&(he, own_side)| {
            let side = || pairs.iter().filter(move |p| p.side == Some(own_side));
            if let Some(p) = side().find(|p| !p.read.as_ref().is_ok_and(|r| r.pointed)) {
                return Err(TouchUndecided::Beside(Some(p.partner)));
            }
            let class = partners_class(own, he, side(), shape, band, || {
                layered(own_side, he, side())
            })
            .map_err(TouchUndecided::Along)?
            .ok_or_else(|| TouchUndecided::Beside(side().next().map(|p| p.partner)))?;
            Ok((he, class))
        })
        .collect()
}

/// **The classes of a vertex's edges against the other solid, where
/// the vertex is in pairs alone**, touching no face, its own edges
/// `own` (its sectors); refuses [`BooleanError::VertexReadTwice`],
/// naming the partner and one beside it, where a partner's cone reads
/// nothing beside another's: against the vertex's edges, or against
/// the other's in reading which cone holds which. One pair alone keeps
/// its rows; where its partner reads nothing it has none, and names the
/// partner and its neighbourhood instead ([`super::UnreadPartner`]).
///
/// Several partners read [`layered`]: the cones that no other holds
/// are outermost, and the material beyond them is outside the solid
/// where they are met and inside it where they are joined. Which side
/// of a link a partner calls its cone does not matter here: just beyond
/// an outermost cone lies outside every cone, since no other holds its
/// link. A cone lies inside another where every edge of it is held by
/// the other or lies on the other's boundary, and some of it is held:
/// an edge, or, where every edge lies on the other's faces (an island
/// touching each face of its void), a direction strictly inside it
/// ([`interior`]); where none reads before its steps reach the band,
/// the corner is thinner than ε_input wherever probed, and it refuses
/// [`BooleanError::NestingUnread`]. An edge on two or more partners' boundaries reads
/// [`along_partners`]. The layering relies on cones that touch but
/// never cross, as one solid's sheets at rest do: they nest or lie
/// apart, so some cone is outermost, and outermost cones agree (a met
/// one beside a joined one would lie in its material). No outermost
/// cone, outermost cones that disagree, and an edge some partner's
/// rows miss (every pair reads the vertex's one orbit) each break that
/// and refuse as an invariant.
///
/// A partner reads nothing (`sectors::wedge_read`'s
/// [`super::UnreadNeighbourhood`]) where its sectors lie on fewer than
/// two faces; where they lie on two with no bisector to read the wedge
/// by; or where its edge or corner is read as a polygon cone and some
/// edge has no reference whose every reading is decided, with none
/// escalating (`sectors::cone_side`'s `None`). A nesting read in band
/// refuses.
pub(super) fn pair_classes<T: Decide>(
    operand: Operand,
    vertex: VertexKey,
    own: &[BoolSector<T>],
    pairs: &[PairRead<T>],
    shape: ShapeOf<'_>,
    band: Band,
) -> Result<PairClasses, BooleanError> {
    let refuse = |unread: usize, beside: usize| BooleanError::VertexReadTwice {
        operand,
        vertex,
        reads: [
            SectorRead::Pair(pairs[unread].partner),
            SectorRead::Pair(pairs[beside].partner),
        ],
    };
    if let [one] = pairs {
        return Ok(match &one.read {
            Ok(read) => (read.rows.clone(), None),
            Err(why) => (Vec::new(), Some((one.partner, *why))),
        });
    }
    if let Some(unread) = pairs.iter().position(|p| p.read.is_err()) {
        return Err(refuse(unread, usize::from(unread == 0)));
    }
    // Every partner reads, so this keeps one read per pair.
    let reads: Vec<_> = pairs.iter().filter_map(|p| p.read.as_ref().ok()).collect();
    let unwitnessed = |inner: usize, outer: usize| BooleanError::NestingUnread {
        operand,
        vertex,
        partners: [pairs[inner].partner, pairs[outer].partner],
    };
    Ok((
        layered_alone(own, pairs, &reads, shape, band, refuse, unwitnessed)?,
        None,
    ))
}

/// [`pair_classes`]' answer: the vertex's edge classes, and the lone
/// partner it read nothing of, with that partner's neighbourhood
/// ([`super::UnreadPartner`]), where there is one.
pub(super) type PairClasses = (
    Vec<(HalfEdgeKey, SideCode)>,
    Option<(VertexKey, super::UnreadNeighbourhood)>,
);

/// [`pair_classes`]' layering of partners that each read (`reads`, one
/// per pair); `refuse(j, i)` where partner `j`'s cone reads nothing
/// against partner `i`'s edges.
#[allow(clippy::too_many_arguments)]
fn layered_alone<T: Decide>(
    own: &[BoolSector<T>],
    pairs: &[PairRead<T>],
    reads: &[&super::sectors::WedgeRead],
    shape: ShapeOf<'_>,
    band: Band,
    refuse: impl Fn(usize, usize) -> BooleanError,
    unwitnessed: impl Fn(usize, usize) -> BooleanError,
) -> Result<Vec<(HalfEdgeKey, SideCode)>, BooleanError> {
    let invariant = |what| BooleanError::ClassificationInvariant { what };
    // Partner `i`'s cone lies inside `j`'s where every edge of `i` is
    // held by `j` or on its boundary, and some of `i` lies inside `j`:
    // a held edge, or, where every edge lies on `j`'s faces, a
    // direction strictly inside `i`'s cone ([`interior`]).
    let inside = |i: usize, j: usize| -> Result<bool, BooleanError> {
        let read = super::sectors::wedge_classes(&pairs[i].sectors, &pairs[j].sectors, band)?
            .ok_or_else(|| refuse(j, i))?;
        let mut rows = read.rows.iter().map(|&(_, c)| c);
        if !rows.clone().all(|c| c == SideCode::On || held(c, read.met)) {
            return Ok(false);
        }
        if rows.any(|c| c != SideCode::On) {
            return Ok(true);
        }
        let probe = interior(&pairs[i].sectors, band)?.ok_or_else(|| unwitnessed(i, j))?;
        let read =
            super::sectors::wedge_classes(std::slice::from_ref(&probe), &pairs[j].sectors, band)?
                .ok_or_else(|| refuse(j, i))?;
        Ok(read.rows.iter().all(|&(_, c)| held(c, read.met)))
    };
    let mut base = None;
    for (i, read) in reads.iter().enumerate() {
        let mut outermost = true;
        for j in (0..pairs.len()).filter(|&j| j != i) {
            if inside(i, j)? {
                outermost = false;
            }
        }
        if outermost {
            let here = if read.met {
                SideCode::Out
            } else {
                SideCode::In
            };
            if base.is_some_and(|b| b != here) {
                return Err(invariant(
                    "a vertex's outermost partners disagree on the material beyond them",
                ));
            }
            base = Some(here);
        }
    }
    let base = base.ok_or_else(|| invariant("each of a vertex's partners lies inside another"))?;
    reads[0]
        .rows
        .iter()
        .map(|&(he, _)| {
            let class = partners_class(own, he, pairs.iter(), shape, band, || {
                layered(base, he, pairs.iter())
            })?
            .ok_or_else(|| invariant("a partner's rows miss an edge of the vertex"))?;
            Ok((he, class))
        })
        .collect()
}

/// **The class of the vertex's edge `he` among `partners`**: read by
/// their faces ([`along_partners`]) where it lies on two or more of
/// their boundaries, else `layer`'s; `Ok(None)` where `layer` leaves it
/// undecided. Refuses as an invariant where the vertex's own sectors
/// (`own`) name no edge for `he`.
fn partners_class<'a, T: Decide + 'a>(
    own: &[BoolSector<T>],
    he: HalfEdgeKey,
    partners: impl Iterator<Item = &'a PairRead<T>>,
    shape: ShapeOf<'_>,
    band: Band,
    layer: impl FnOnce() -> Option<SideCode>,
) -> Result<Option<SideCode>, BooleanError> {
    let on: Vec<_> = partners
        .filter(|p| {
            p.read
                .as_ref()
                .is_ok_and(|r| r.rows.contains(&(he, SideCode::On)))
        })
        .collect();
    if let [_, _, ..] = on.as_slice() {
        let edge = own.iter().find(|s| s.he == he && s.end_edge()).ok_or(
            BooleanError::ClassificationInvariant {
                what: "a vertex's row names no edge of its own sectors",
            },
        )?;
        return along_partners(edge, &on, shape, band).map(Some);
    }
    Ok(layer())
}

/// **A direction strictly inside a partner's cone**, as a probe edge
/// read against other corners: the first candidate the corner's own
/// reading holds (inside its material where its cone is met, outside
/// where joined). The candidates are the link mean `c`
/// ([`super::sectors::link_mean`]) and `−c`, which serve a convex or
/// hollow corner, whose bounds lie on its cone's boundary; then, for a
/// polygon cone whose mean may leave it, a direction off each sector's
/// middle to either side of its face, a quarter of the middle's length
/// off at first and a fifth of that at each next step, so a corner as
/// thin as the band allows is reached. The steps stop where the band
/// takes them: at the first step at which no candidate reads decidedly
/// off its own face. Each is levered at the corner's shortest arm. A
/// candidate read in band is passed over, since another may still read
/// inside; any other refusal is the reading's own and is returned.
/// `None` where no candidate holds before the band takes the steps:
/// the corner is thinner than the band wherever it was probed.
fn interior<T: Decide>(
    sectors: &[BoolSector<T>],
    band: Band,
) -> Result<Option<BoolSector<T>>, BooleanError> {
    let (Some(first), Some((mean, arm))) = (sectors.first(), super::sectors::link_mean(sectors))
    else {
        return Ok(None);
    };
    let reach = super::sectors::Reach::Extent(arm);
    let probe = |dir: Vec3<T>| BoolSector {
        start: dir,
        end: dir,
        start_reach: reach,
        end_reach: reach,
        ..first.clone()
    };
    let holds = |dir: Vec3<T>| -> Result<Option<BoolSector<T>>, BooleanError> {
        let candidate = probe(dir);
        match super::sectors::wedge_classes(std::slice::from_ref(&candidate), sectors, band) {
            Ok(Some(r)) if r.rows.iter().all(|&(_, c)| held(c, r.met)) => Ok(Some(candidate)),
            Ok(_) | Err(BooleanError::Escalated { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    };
    for dir in [mean, -mean] {
        if let Some(candidate) = holds(dir)? {
            return Ok(Some(candidate));
        }
    }
    let fifth = T::from_f64(0.2);
    let mut step = T::from_f64(0.25);
    loop {
        let mut clear = false;
        for s in sectors {
            let middle = s.middle();
            let off = s.normal.vec() * (step * middle.norm());
            for dir in [middle - off, middle + off] {
                clear |= matches!(
                    super::sectors::plane_side_code(dir, reach, s.normal, band),
                    Ok(SideCode::In | SideCode::Out)
                );
                if let Some(candidate) = holds(dir)? {
                    return Ok(Some(candidate));
                }
            }
        }
        if !clear {
            return Ok(None);
        }
        step = step * fifth;
    }
}

/// **The class of an edge on two or more partners' boundaries** (`on`):
/// `On` where the material their faces bound changes side somewhere
/// around it, or where their faces are glued along it. Around the edge
/// each partner's material turns at the half-planes its faces leave the
/// edge in ([`super::sectors::half_planes`]), so the solid's material
/// turns at a half-plane one partner alone leaves it in: the edge is on
/// the solid's boundary where one such is decidedly apart from every
/// other partner's.
///
/// Where every half-plane meets another's at a decided zero, the
/// partners lie flush along the edge. Where each such meeting is of two
/// plane faces (`shape`) bounding material on opposite sides, the faces
/// are glued, coincident only to within the band: a sub-band tilt parts
/// them by a sliver whose boundary the edge lies on, so only `On` is
/// decided there (D4), as the polygon-cone oracle reads any direction
/// on a face (`sectors::cone_fuzz`). Matched plane faces bounding
/// material on one side overlap the partners' materials to within the
/// band, as a lump thinner than the band glued on a face reads; a
/// curved face in a meeting may be tangent to its match rather than
/// glued, which these first-order readings do not tell apart; and a
/// chord in band of zero is neither apart nor flush. Each refuses with
/// [`super::BooleanDecision::PartnerFaces`]. These flush readings are
/// reached only where no half-plane is decidedly apart from every other
/// partner's, which reads `On` first. Faces flush along the edge part by
/// the half-planes' chord at the farthest reach of the edge and the
/// partners' sectors.
pub(super) fn along_partners<T: Decide>(
    edge: &BoolSector<T>,
    on: &[&PairRead<T>],
    shape: ShapeOf<'_>,
    band: Band,
) -> Result<SideCode, BooleanError> {
    use super::sectors::FaceShape;
    let invariant = |what| BooleanError::ClassificationInvariant { what };
    let mut planes = Vec::new();
    for (k, p) in on.iter().enumerate() {
        let read = super::sectors::half_planes(edge.end, edge.end_reach, &p.sectors, band)?;
        planes.extend(read.into_iter().map(|(t, s)| (k, t, s)));
    }
    let reach = on
        .iter()
        .flat_map(|p| p.sectors.iter().map(BoolSector::span))
        .fold(edge.end_reach.length(), |a, b| a.max(b));
    let mut refused = None;
    for &(k, t, s) in &planes {
        let mut apart = true;
        for &(_, u, r) in planes.iter().filter(|&&(j, ..)| j != k) {
            let chord = Margin::levered((t - u).norm(), reach);
            let Err(diag) = crate::validate::decide_nonzero("bool_partner_half_plane", chord, band)
            else {
                continue;
            };
            apart = false;
            let planar = shape(s.face)? == FaceShape::Plane && shape(r.face)? == FaceShape::Plane;
            if diag.margin.rejected_sign() == Some(Sign::Zero) && planar {
                // Each face's normal is square to the one plane both
                // lie in, so they are opposed (glued) or alike.
                let sides = Margin::of(s.normal.vec().dot(r.normal.vec()));
                match crate::validate::decide("bool_partner_glued_side", sides, band) {
                    Ok(Sign::Negative) => continue,
                    Ok(Sign::Positive | Sign::Zero) => {}
                    Err(sided) => {
                        refused.get_or_insert(sided);
                        continue;
                    }
                }
            }
            refused.get_or_insert(diag);
        }
        if apart {
            return Ok(SideCode::On);
        }
    }
    match refused {
        Some(diag) => Err(BooleanError::Escalated {
            decision: super::BooleanDecision::PartnerFaces,
            diag,
        }),
        None if planes.is_empty() => Err(invariant(
            "an edge on two partners' boundaries lies in none of their faces",
        )),
        None => Ok(SideCode::On),
    }
}

/// Output of one vertex-on-face classification.
#[derive(Debug)]
pub(super) struct VtxFacOut<T: geom_core::Real> {
    /// Minted null edges (piercing-side runs + pierced-side ring struts).
    pub edges: Vec<BoolNullEdgeRecord<T>>,
    /// The pierce's half of its ring, if it crosses the face
    /// ([`mint_ring`] mints it with the face's other pierces there).
    pub ring: Option<RingPlan<T>>,
    /// `(A face, B face)` for each coincident sector whose lump keeps
    /// one copy of the region (`BooleanReduction::covered`).
    pub covered: Vec<(crate::entity::FaceKey, crate::entity::FaceKey)>,
    /// What the covered sectors owe the dropped copy
    /// ([`super::fate`]).
    pub fates: super::fate::Fates<T>,
    /// Each edge at the piercing vertex, by the half-edge leaving it,
    /// with its side of the pierced face as first read, before any
    /// lump (`BooleanReduction::edge_classes`).
    pub classes: Vec<(HalfEdgeKey, SideCode)>,
    /// The pierced face's datum the classes were read against.
    pub datum: PierceDatum<T>,
    /// Whether a sector reads on the face at both bounds: lies in its
    /// plane, the lump's candidate.
    pub flush: bool,
}

#[derive(Clone, Copy, Debug)]
struct Entry {
    he: HalfEdgeKey,
    is_edge: bool,
    class: SideCode,
    /// The bound's first-order reading tied
    /// ([`super::sectors::PierceSide::tied`]).
    tied: bool,
    /// The Zero this bound's own first-order reading decided, where it
    /// read `On` there.
    on: Option<super::sectors::OnReading>,
    /// The class is Delta 2's lump, not this bound's own reading.
    lumped: bool,
}

/// Classifies `contact.vertex` (in the piercing body) against
/// `contact.face` (in the pierced body) and performs the piercing
/// side's insertion (module docs), returning its half of the ring for
/// [`mint_ring`]. Where the vertex is read again (`reread`,
/// from [`pierced_and_paired`]) and crosses the face, refuses
/// [`BooleanError::VertexReadTwice`] before it writes.
#[allow(clippy::too_many_arguments)]
pub(super) fn classify_vertex_on_face<T: Decide + crate::props::AtRestPolicy>(
    piercing_body: &mut Body<T>,
    pierced_body: &Body<T>,
    piercing: Operand,
    contact: VfContact,
    op: BooleanOp,
    declared: &super::GluedPairs<T>,
    contacts: &super::ReductionContacts,
    reread: Option<SectorRead>,
    branch: Option<super::Branch>,
    chart: Option<(f64, f64)>,
    tips: &crate::sector_shape::SlitTips,
    band: Band,
) -> Result<VtxFacOut<T>, BooleanError> {
    let vertex = contact.vertex;
    let pierced_op = piercing.other();
    // The pierce point, read BEFORE the classification: on a curved
    // pierced face the oriented datum is point-dependent, so `p` is an
    // input to the sector algebra rather than only to Delta 3's ring.
    let p = piercing_body.resolve_vertex_point(vertex, Proven);
    // `contact.face` is a key the sweep recorded and carries here, so
    // its miss is typed; its surface is a link. This pass mints its rings
    // only once every pierce is read ([`mint_ring`]), so the face is as
    // the operand and the passes before this one left it.
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
    let unbuilt = || BooleanError::CurvedBooleanUnsupported {
        operand: pierced_op,
        face: contact.face,
        kind: pierced_kind,
    };
    let refused = |refusal| BooleanError::of_pierced_normal(refusal, pierced_op, contact.face);
    let n_pierced =
        match crate::face_normal::face_outward_normal_at(pierced_body, contact.face, p, band) {
            Ok(Some(n)) => n,
            // A spline face, read at the chart point the sweep placed the
            // pierce at (`chart`); one it placed at none refuses C5's
            // typed refusal, naming the kind. That `p` has no second foot
            // on another in-trim sheet within the band is the placement's
            // claim (`work/spliceface/the-spline-placement-read-certifies-no-second-in-trim-foot`).
            Ok(None) => {
                use crate::face_normal::SplineNormalError;
                let uv = chart.ok_or_else(unbuilt)?;
                crate::face_normal::spline_outward_normal_at(
                    pierced_body,
                    contact.face,
                    uv,
                    p,
                    band,
                )
                .map_err(|e| match e {
                    SplineNormalError::At(refusal) => refused(refusal),
                    // A placeholder chart has no locus: the kind has no arm.
                    SplineNormalError::PlaceholderChart => unbuilt(),
                    SplineNormalError::StaleFace | SplineNormalError::NotSpline => {
                        BooleanError::ClassificationInvariant {
                            what: "a pierced face the point door read no arm for is not a \
                                   live spline face",
                        }
                    }
                })?
            }
            Err(refusal) => return Err(refused(refusal)),
        };
    let pierced_shape = super::sectors::FaceShape::of(pierced_surface);
    let sectors = build_sectors(piercing_body, piercing, vertex, tips, band)?;
    let n = sectors.len();
    // The pierced face's smallest radius of curvature within each
    // bound's reach — the lever the sector side verdicts charge their
    // sagitta against (`pierce_side`'s argument), so it must bound the
    // tightest bend, not the chart's scale: on a fat torus those differ,
    // and on a cone the bend tightens toward the axis. A plane reports
    // `f64::MAX`, so its charge is vacuous and the planar lane's
    // verdicts are unmoved. A spline face's is its chart's certified arm
    // over the sectors' farthest reach, which rests on the placement's
    // single sheet as the normal does; where none is read, no side is.
    let pierced_lever = match pierced_surface.spline_chart() {
        None => PierceLever::of(pierced_surface, p),
        Some(chart) => {
            let reach = sectors.iter().map(BoolSector::span).reduce(T::max).ok_or(
                BooleanError::ClassificationInvariant {
                    what: "a pierce has no sector",
                },
            )?;
            let arm = T::spline_chart_arm(chart, p, reach).map_err(|cause| {
                BooleanError::PierceLeverUnread {
                    operand: pierced_op,
                    face: contact.face,
                    cause,
                }
            })?;
            PierceLever::Within(T::from_computed(arm))
        }
    };

    // Entries = the bounds in orbit order (entry k = sector k's END
    // bound: real chord or subdivision bisector), classed against the
    // pierced face's plane via the F3 primitive. `n_pierced` is the
    // pierced face's OUTWARD normal (S10, minted by
    // `face_outward_normal`) — In/Out here is a material verdict and
    // would read backwards off a chart normal on a reversed face,
    // which is why the primitive takes the typed one.
    //
    // **The pierced wall through the bisector plane the sweep found the
    // pierce on** (`branch`), for a sector of a wall that crosses the
    // pierced wall as an equal pair ([`super::node::bisector_datum`]).
    // There the two walls' normals stand only about the node's distance
    // apart, and the plane's signed normal stands in for the pierced
    // face's tangent plane: a bound on that wall reads its side across
    // the plane, with no curvature charge, and the sector's germ rides
    // the plane's ellipse. A pierce the sweep recorded no branch for
    // reads nothing here.
    let on_wall: Vec<Option<OutwardNormal<T>>> = match branch {
        None => vec![None; n],
        Some(branch) => sectors
            .iter()
            .map(|s| {
                let Some(planes) = super::node::bisectors(
                    (&*piercing_body, piercing, s.face),
                    (pierced_body, contact.face),
                    band,
                ) else {
                    return Ok(None);
                };
                let faces = match piercing {
                    Operand::A => (s.face, contact.face),
                    Operand::B => (contact.face, s.face),
                };
                super::node::bisector_datum(
                    planes,
                    branch,
                    (piercing, faces),
                    p,
                    (pierced_surface, n_pierced),
                    band,
                )
            })
            .collect::<Result<_, BooleanError>>()?,
    };
    // A conic tied with a plane face whose bend reads no side
    // (`BooleanDecision::ConicBend`) is held as its first-order `On`
    // until the lumps have run: a tangent sector's lump reads its whole
    // sector off the two carriers and overwrites the bound's reading,
    // so the bend's refusal stands only for a bound no lump took.
    let mut entries = Vec::with_capacity(n);
    let mut unread_bends = Vec::new();
    for (k, s) in sectors.iter().enumerate() {
        // A bound lies on the wall of either sector it bounds: entry `k`
        // is sector `k`'s end and sector `k − 1`'s start.
        let side = match on_wall[k].or(on_wall[(k + n - 1) % n]) {
            Some(plane) => pierce_side_reported(
                s.end,
                s.end_reach,
                plane,
                super::sectors::FaceShape::Plane,
                super::sectors::NO_CURVATURE(),
                band,
            ),
            None => pierce_side_reported(
                s.end,
                s.end_reach,
                n_pierced,
                pierced_shape,
                pierced_lever.within(s.end_reach.length()),
                band,
            ),
        };
        let (read, on) = match side {
            Ok(read) => read,
            Err(
                refusal @ BooleanError::Escalated {
                    decision: super::BooleanDecision::ConicBend,
                    ..
                },
            ) => {
                unread_bends.push((entries.len(), refusal));
                let unread = super::sectors::PierceSide {
                    code: SideCode::On,
                    tied: true,
                };
                (unread, None)
            }
            Err(e) => return Err(e),
        };
        entries.push(Entry {
            he: s.he,
            is_edge: s.end_edge(),
            class: read.code,
            tied: read.tied,
            on,
            lumped: false,
        });
    }

    // Delta 2 (rule (a) analogue): coplanar sectors lump per Eq. 15.3.
    // "Coplanar" is against the pierced face's TANGENT plane at `p`,
    // whose normal is `n_pierced` — the same vector `face_plane` hands
    // back on a planar face, so the planar lane's margin is unmoved.
    //
    // Lumping overwrites both bounds' readings, so both must tie at
    // first order, each at its reach — a line bound at its far vertex,
    // in metres; a bound read definitely off the plane decides the
    // sector off it. A conic bound that ties may read its bend off the
    // plane, and the lump's second order supersedes it.
    // Where both read On, the normals' parallelism proposes coplanarity:
    // at the sector's reach, the ball the carrier ladder reads an
    // unglued pair over below; a glued pair's at the arm, its ladder
    // reading over the glue's extent. A tilt the bounds
    // read On and the normals do not (in
    // band, or decided off: a thin sector tilted about one bound rises
    // at its arm by more than at its other bound) leaves the question
    // undecided at this tolerance, and the margin that decides it is the
    // bounds' own: a smaller tolerance reads the steeper bound off the
    // plane and the sector with it.
    let read: Vec<SideCode> = entries.iter().map(|e| e.class).collect();
    let tied: Vec<bool> = entries.iter().map(|e| e.tied).collect();
    let classes = entries
        .iter()
        .filter(|e| e.is_edge)
        .map(|e| (e.he, e.class))
        .collect();
    let flat = |k: usize| tied[k] && tied[(k + 1) % n];
    let flush = (0..n).any(flat);
    let mut covered = Vec::new();
    let mut fates = super::fate::Fates::default();
    for (k, s) in sectors.iter().enumerate() {
        if !flat(k) {
            continue;
        }
        // A glued pair is read through its record: a tangency lumps by
        // its curving, a one-carrier pair by the relation the glue door
        // recorded, and neither reads the sector's tilt again.
        let class = declared.class_of(piercing, s.face, pierced_op, contact.face);
        if class == Some(GlueClass::Tangent) {
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
                &s_sector,
                &s_pierced,
                reach,
                n_pierced,
                s.normal,
                p,
                op,
                piercing,
                s.face,
                s.reached_bounds(),
                s.arm,
                read,
                band,
            )?;
            entries[k].class = lump;
            entries[(k + 1) % n].class = lump;
            entries[k].lumped = true;
            entries[(k + 1) % n].lumped = true;
            continue;
        }
        let rel = match declared.glued_relation(piercing, s.face, pierced_op, contact.face) {
            Some(rel) => rel,
            None => {
                return Err(unglued_flat_sector(
                    (piercing_body, piercing, s),
                    (pierced_op, contact.face, pierced_kind),
                    plane,
                    (p, n_pierced.vec()),
                    declared,
                    band,
                ));
            }
        };
        if let Some(keeper) = super::tables::kept_copy(op, rel) {
            covered.push(match piercing {
                Operand::A => (s.face, contact.face),
                Operand::B => (contact.face, s.face),
            });
            let aligned = matches!(rel, super::PlaneRelation::SameOriented);
            if keeper == piercing {
                for (edge, _) in super::sectors::bound_edges(piercing_body, s) {
                    fates.edges.push(super::fate::FateEdge {
                        holder: piercing,
                        edge,
                        kept: s.face,
                        face: contact.face,
                        aligned,
                    });
                }
            } else {
                let inside = super::fate::bisector(s, band)?;
                fates.corners.push(super::fate::FateCorner {
                    holder: pierced_op,
                    dropped: super::fate::SectorAt {
                        he: s.he,
                        face: s.face,
                        dir: inside,
                    },
                    kept: super::fate::KeptAt::Pierced(contact.face, vertex, inside),
                });
            }
        }
        let lump = eq15_3_lump(op, piercing, rel);
        entries[k].class = lump;
        entries[(k + 1) % n].class = lump;
        entries[k].lumped = true;
        entries[(k + 1) % n].lumped = true;
    }

    if let Some((_, refusal)) = unread_bends.into_iter().find(|&(k, _)| !entries[k].lumped) {
        return Err(refusal);
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
        ring: None,
        covered,
        fates,
        classes,
        datum: PierceDatum {
            normal: n_pierced,
            shape: pierced_shape,
            lever: pierced_lever,
        },
        flush,
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
    let germ_of = |t: usize, out: OutBound| -> Result<Germ<T>, BooleanError> {
        let s = &sectors[t];
        let own = super::sectors::germ_locus(
            super::sectors::GermSide {
                body: piercing_body,
                operand: piercing,
                site: vertex,
                sector: s,
                read: (read[(t + 1) % n], read[t]),
            },
            (pierced_body, contact.face),
            contacts,
            band,
        )?;
        let pierced = super::Locus::InFace(contact.face);
        let cells = match piercing {
            Operand::A => ((s.face, contact.face), (own, pierced)),
            Operand::B => ((contact.face, s.face), (pierced, own)),
        };
        let (dir, branch) = match (on_wall[t], branch) {
            (Some(plane), Some(branch)) => (pierce_germ_dir(s, plane.vec(), out, band)?, branch),
            _ => (
                pierce_germ_dir(s, n_pierced.vec(), out, band)?,
                germ_branch(
                    &declared.sections,
                    (&*piercing_body, pierced_body),
                    piercing,
                    cells,
                    p,
                    band,
                )?,
            ),
        };
        Ok((cells, dir, branch))
    };
    let run_germs = runs
        .iter()
        .map(|&run| {
            let [(t0, o0), (t1, o1)] = run_transitions(run, n);
            Ok((germ_of(t0, o0)?, germ_of(t1, o1)?))
        })
        .collect::<Result<Vec<_>, BooleanError>>()?;
    // The germs the ring orders its struts by ([`mint_ring`]). A germ
    // riding a bisector plane's ellipse (`on_wall`) has no bend in the
    // face.
    let bend_of = |t: usize, d: Vec3<T>| {
        let face = sectors[t].face;
        let surface = piercing_body
            .face_surface_linked(face, proven(&piercing_body.faces, face, EntityId::Face));
        on_wall[t].is_none().then(|| GermBend {
            bend: geom_brep::implicit_section_acceleration(
                pierced_surface,
                surface,
                p,
                d,
                n_pierced.vec().cross(d),
            ),
            face,
            kind: surface.kind(),
        })
    };
    let order_germs: Vec<_> = runs
        .iter()
        .zip(&run_germs)
        .map(|(&run, (s, e))| {
            let [(t0, _), (t1, _)] = run_transitions(run, n);
            (
                [(s.1, sectors[t0].arm), (e.1, sectors[t1].arm)],
                [bend_of(t0, s.1), bend_of(t1, e.1)],
            )
        })
        .collect();
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
        // or bisector entries alone, inside the sector before `after`.
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
                // The run is bisector entries alone, inside one corner P:
                // a wide sector's one bisector, or one or two of a slit
                // tip's full turn's three quarters. A corner holds at most
                // one such run: the run lies between two changes of the
                // face's side of the pierced plane inside P, which a
                // tangent plane changes at most twice round P (once in a
                // wide sector; twice in a full turn, which the plane
                // meets along one line). Every entry of P carries P's
                // half-edge, so `after` is the first real entry past the
                // run, P's orbit successor Q, and runs being maximal,
                // neither P's real entry nor Q's is Out. The other run's
                // mint moves only Out real halves, and splices only before
                // its first half, the successor of its last, or its own
                // `after`, whose predecessor is another corner: none is
                // Q. So the corner is still `after`'s at `vertex` whichever
                // run mints first, and the table's read stands (minting
                // struts first would reorder `out.edges` and the ring
                // struts with them).
                let after = piercing_body.strut_before_next_real(
                    (vertex, sectors[run.0].he),
                    (run.0 + run.1..run.0 + run.1 + n)
                        .map(|j| (entries[j % n].he, entries[j % n].is_edge)),
                    || format!("the bisector run {run:?}"),
                );
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
                // A pierce's germs lie in the pierced face, through no
                // edge of the pierced solid: their cuts run beside none.
                let germ =
                    |t: usize, (cells, dir, _): Germ<T>| (super::insert::Cut::apart(t, dir), cells);
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

    out.ring = Some(RingPlan {
        contact,
        piercing,
        point: p,
        normal: n_pierced.vec(),
        runs: run_edges
            .iter()
            .zip(run_germs)
            .zip(order_germs)
            .map(|((rec, germs), (order, bends))| PlannedRun {
                edge: rec.edge,
                germs,
                order,
                bends,
            })
            .collect(),
    });
    Ok(out)
}

/// One pierce's half of its ring: the piercing side's run null edges
/// and their germs, which the ring's struts pair with ([`mint_ring`]).
#[derive(Debug)]
pub(super) struct RingPlan<T: geom_core::Real> {
    pub contact: VfContact,
    pub piercing: Operand,
    /// The pierce point. The ring vertex copies its lead pierce's point
    /// bitwise; every other member's is one with it by
    /// `bool_contact_vertex` ([`ring_groups`]).
    pub point: geom_core::Point3<T>,
    /// The pierced face's outward normal at `point`.
    pub normal: Vec3<T>,
    runs: Vec<PlannedRun<T>>,
}

#[derive(Debug)]
struct PlannedRun<T: geom_core::Real> {
    /// The run's null edge in the piercing body.
    edge: crate::entity::EdgeKey,
    /// Its start and end germs.
    germs: (Germ<T>, Germ<T>),
    /// Its start and end germs' directions, each with its arm, for
    /// [`germ_order`].
    order: [(Vec3<T>, T); 2],
    /// Its start and end germs' bends, where [`germ_order`] reads one.
    bends: [Option<GermBend<T>>; 2],
}

/// **How a pierce germ's section curves in the pierced face**: how fast
/// it leaves its direction `d` toward `n × d`, `n` the face's outward
/// normal ([`geom_brep::implicit_section_acceleration`] of the pierced
/// surface and the face it is the section of), and that face, which a
/// bend that does not read names.
#[derive(Clone, Copy, Debug)]
struct GermBend<T> {
    bend: T,
    face: crate::entity::FaceKey,
    kind: geom::SurfaceKind,
}

/// What [`mint_ring`] minted.
pub(super) struct RingOut<T: geom_core::Real> {
    /// The ring's struts, in mint order.
    pub edges: Vec<BoolNullEdgeRecord<T>>,
    /// Each strut paired with its run's null edge, in mint order.
    pub pairs: Vec<NullEdgePairRecord>,
    pub ring: PierceRingRecord,
}

/// **The pierces that share a ring**: indices into `plans`, the
/// classes of "pierces one face at one point", each in index order and
/// the classes by their first member; and the pairs whose decision
/// joined two classes, each with the margin `bool_contact_vertex`
/// decided Zero.
///
/// Two pierces of one face whose points read one vertex are one class,
/// closed transitively: edges of the piercing body that coincide there
/// (a contact line, or a pinch's cones). The joining pairs span each
/// class, so every member is tied to every other through decided pairs,
/// which the ledger records ([`super::RingTie`]).
///
/// # Errors
///
/// [`BooleanError::Escalated`] (`VertexOnVertex`) where two pierces of
/// one face lie in the band of one point (pinned by
/// `ring_groups_close_transitively_and_escalate_in_band`).
#[allow(clippy::type_complexity)]
pub(super) fn ring_groups<T: Decide>(
    plans: &[&RingPlan<T>],
    band: Band,
) -> Result<(Vec<Vec<usize>>, Vec<(usize, usize, geom_core::MarginDiag)>), BooleanError> {
    let mut roots = super::zip::Roots::new();
    let mut ties = Vec::new();
    for (i, a) in plans.iter().enumerate() {
        for (j, b) in plans.iter().enumerate().skip(i + 1) {
            if a.contact.face != b.contact.face {
                continue;
            }
            let at = super::one_vertex_at(a.point, b.point, band).map_err(|diag| {
                BooleanError::Escalated {
                    decision: super::BooleanDecision::VertexOnVertex,
                    diag,
                }
            })?;
            if let Some(margin) = at
                && roots.union(i, j)
            {
                ties.push((i, j, margin));
            }
        }
    }
    let mut classes: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for i in 0..plans.len() {
        classes.entry(roots.find(i)).or_default().push(i);
    }
    let mut groups: Vec<Vec<usize>> = classes.into_values().collect();
    groups.sort_by_key(|g| g[0]);
    Ok((groups, ties))
}

/// **A lone ring vertex at `p` inside `face`** (Delta 3's first two
/// steps), struck from the outer loop's anchor by [`Body::ring_vertex`]:
/// `mev` then `kemr`, with the face's rows standing as found on every
/// chart. Returns the vertex and its ring.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] where the face's outer loop
/// is no cycle; [`Body::ring_vertex`]'s refusals.
pub(super) fn ring_vertex_in<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    face: crate::entity::FaceKey,
    p: geom_core::Point3<T>,
    tol: Tol,
) -> Result<(VertexKey, crate::entity::LoopKey), BooleanError> {
    let face_data = proven(&body.faces, face, EntityId::Face);
    let crate::entity::LoopBoundary::Cycle { first: anchor } = linked(
        &body.loops,
        face_data.outer,
        EntityId::Loop,
        EntityId::Face(face),
        "outer",
    )
    .boundary
    else {
        return Err(BooleanError::ClassificationInvariant {
            what: "a face a ring vertex is minted in has no outer cycle",
        });
    };
    Ok(body.ring_vertex(anchor, p, tol)?)
}

/// **Delta 3, the ring** (module docs): one ring vertex in the pierced
/// face at the pierce point, and one null-edge strut per piercing-side
/// run of every pierce in `plans`, each paired with its run's null edge.
///
/// `plans` are the pierces of one face at one point, so the ring holds
/// one vertex per corner of the face round the point, the corners
/// every member's runs cut: edges of the other body that coincide (a
/// contact line, or a pinch's cones) and pierce the face there mint
/// one ring on one point key, whose struts nest by the germs of all
/// their runs together. Where the struts leave both operands one
/// vertex at a pinch, `zip::split_cones` splits it per cone before the
/// zips.
///
/// # Errors
///
/// [`germ_order`]'s refusals, and
/// [`BooleanError::ClassificationInvariant`] where the runs read as
/// crossing chords about the face's normal or the face's outer loop is
/// no cycle.
pub(super) fn mint_ring<T: Decide + crate::props::AtRestPolicy>(
    pierced_body: &mut Body<T>,
    plans: &[&RingPlan<T>],
    band: Band,
    tol: Tol,
) -> Result<RingOut<T>, BooleanError> {
    let first = plans.first().ok_or(BooleanError::ClassificationInvariant {
        what: "a pierce ring with no pierce",
    })?;
    let (face, p, piercing) = (first.contact.face, first.point, first.piercing);
    if plans
        .iter()
        .any(|plan| plan.piercing != piercing || plan.contact.face != face)
    {
        return Err(BooleanError::ClassificationInvariant {
            what: "a pierce ring's pierces do not share one operand and one face",
        });
    }
    // Every member reads the face's normal at its own point, one point
    // with the lead's by `ring_groups`; the ring orders all their germs
    // about the lead's.
    // Each run with its pierce, and the pierce's index in `plans`.
    let runs: Vec<(usize, &RingPlan<T>, &PlannedRun<T>)> = plans
        .iter()
        .enumerate()
        .flat_map(|(k, &plan)| plan.runs.iter().map(move |run| (k, plan, run)))
        .collect();
    // The ring's struts ([`crate::null::ring_tree`]), read before any
    // write.
    let (ring, ties) = ring_struts(&runs, first.normal, (piercing, face), band)?;
    let pierced = piercing.other();
    let (w, ring_loop) = ring_vertex_in(pierced_body, face, p, tol)?;
    let mut out = RingOut {
        edges: Vec::new(),
        pairs: Vec::new(),
        ring: PierceRingRecord {
            operand: pierced,
            face,
            ring_vertex: w,
            pierces: plans.iter().map(|plan| plan.contact.vertex).collect(),
            ties: Vec::new(),
        },
    };
    // (3) one ring null-edge strut per piercing-side run, hung at the
    // ring vertex or at another strut's far end, as the tree of the
    // ring's corners says ([`crate::null::ring_tree`]). A strut hangs at
    // its node just before the half leaving it towards the root, after
    // the node's earlier struts: at the ring vertex, before the first
    // strut's leaving half. The half leaving its node faces the germ its
    // run's corner meets first clockwise, so facings alternate by depth;
    // with one run either half may face either germ. The op does not
    // enter.
    let mut root_anchor: Option<HalfEdgeKey> = None;
    // Per run: its strut's far end, and the half leaving it towards
    // the root.
    let mut nodes: BTreeMap<usize, (VertexKey, HalfEdgeKey)> = BTreeMap::new();
    for strut in &ring {
        let (i, leaving_faces_start) = (strut.run, strut.plus_faces_start);
        let (_, plan, run) = runs[i];
        let (start_germ, end_germ) = run.germs;
        let (at, site) = match strut.parent {
            None => (
                w,
                match root_anchor {
                    None => MevSite::Lone { r#loop: ring_loop },
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
        out.edges.push(BoolNullEdgeRecord {
            operand: pierced,
            at_vertex: at,
            edge: created.edge,
            attr,
            dangling: true,
            germs: [half_germ(start_he, start_germ), half_germ(end_he, end_germ)],
        });
        let (a_edge, b_edge, site) = match plan.piercing {
            Operand::A => (
                run.edge,
                created.edge,
                PairSite::VertexAOnFaceB(plan.contact),
            ),
            Operand::B => (
                created.edge,
                run.edge,
                PairSite::VertexBOnFaceA(plan.contact),
            ),
        };
        out.pairs.push(NullEdgePairRecord {
            a_edge,
            b_edge,
            site,
        });
    }
    // Run `r`'s two germs are `2r` and `2r + 1` of the germs
    // `germ_order` was handed, and its strut record's `germs` face them.
    for tie in ties {
        let half = |g: usize| {
            let at = ring.iter().position(|strut| strut.run == g / 2)?;
            out.edges.get(at).map(|record| record.germs[g % 2].he)
        };
        let (Some(first), Some(second)) = (half(tie[0]), half(tie[1])) else {
            return Err(BooleanError::ClassificationInvariant {
                what: "a tied germ's run has no strut",
            });
        };
        out.ring.ties.push([first, second]);
    }
    Ok(out)
}

/// **A pierce ring's struts** ([`crate::null::ring_tree`]) over its
/// runs, each with its pierce's index in its ring's plans, ordered about
/// the pierced face's outward `normal`, and the ties of germs of
/// different pierces ([`germ_order`]). One run has nothing to order, and
/// either half may face either germ, so no germ is read; its strut faces
/// its end germ first.
#[allow(clippy::type_complexity)] // (struts, ties)
fn ring_struts<T: Decide>(
    runs: &[(usize, &RingPlan<T>, &PlannedRun<T>)],
    normal: Vec3<T>,
    site: (Operand, crate::entity::FaceKey),
    band: Band,
) -> Result<(Vec<crate::null::RingStrut>, Vec<[usize; 2]>), BooleanError> {
    if runs.len() == 1 {
        return Ok((
            vec![crate::null::RingStrut {
                run: 0,
                parent: None,
                plus_faces_start: false,
            }],
            Vec::new(),
        ));
    }
    let germs: Vec<_> = runs.iter().flat_map(|(.., run)| run.order).collect();
    let bends: Vec<_> = runs.iter().flat_map(|(.., run)| run.bends).collect();
    let member: Vec<usize> = runs.iter().flat_map(|&(k, ..)| [k, k]).collect();
    let curving = |Turn { from, to }: Turn, arm: T| -> Result<Option<Sign>, BooleanError> {
        let (Some(x), Some(y)) = (bends[from], bends[to]) else {
            return Ok(None);
        };
        match geom_brep::tied_germ_turn(normal, germs[from].0, [x.bend, y.bend], arm, band) {
            Ok(Sign::Zero) => Ok(None),
            Ok(sign) => Ok(Some(sign)),
            Err(geom_brep::TiedGermsError::Singular) => {
                let at = if x.bend.is_bounded() { y } else { x };
                Err(BooleanError::CurvedBooleanUnsupported {
                    operand: site.0,
                    face: at.face,
                    kind: at.kind,
                })
            }
            Err(geom_brep::TiedGermsError::Lever(escalation)) => Err(BooleanError::coincidence(
                Coincide::Sectors,
                DeclarationRead::Moot,
                escalation.diag(),
            )),
        }
    };
    let (order, ties) = germ_order(&germs, &member, normal, site, curving, band)?;
    let struts = crate::null::ring_tree(&order, crate::null::ring_root()).ok_or(
        BooleanError::ClassificationInvariant {
            what: "a pierce's runs read as crossing chords about the pierced face's normal",
        },
    )?;
    Ok((struts, ties))
}

/// **A node of an equal cylinder pair's section, built** (the sibling of
/// the pierce ring, [`mint_ring`]): the point `node` where the two
/// bisector-plane ellipses cross, inside A's face `faces.0` and B's face
/// `faces.1`, which the walls are tangent at. Neither operand has a
/// vertex there, so each wall takes a ring vertex at the node
/// ([`ring_vertex_in`]) and the section's four arcs leave it.
///
/// The arcs' tangents are `tangents` (`e₁`'s, then `e₂`'s, either way
/// along each), `axes` the walls' axes, A's first, and `radial` the
/// node less the axes' meeting point. Round the node each wall's four
/// sectors between the arcs alternate sides of the other cylinder (the
/// section table's `nodes`): on A's wall the sectors about `±â₁` lie
/// outside B's cylinder and those about `±â₂` inside; on B's wall the
/// same sectors, the other way round. Which side of a cylinder is its
/// solid's material is the face's sense. So the node is a pierce of B's
/// face by A's ring vertex whose two runs are the sectors of B's wall
/// inside A's material — about `±â₁` where A's face bounds a rod, `±â₂`
/// where it bounds a bore — each bounded by one arc of each ellipse.
/// B's ring, minted as the pierce's ring ([`mint_ring`]), spans them In,
/// and A's struts span the same sectors on A's wall. Both rings hang
/// their struts by one tree ([`ring_struts`]), ordered about B's normal,
/// A's facing each germ with the other half of its strut from B's, as a
/// held pierce's strut does ([`faces_holding_pierce`]); where the walls'
/// senses differ (a rod against a bore) A's normal at the node is B's
/// reversed, so A's struts read each run in A's own turn: from B's end
/// germ, In. `arm` meters the germs' order.
///
/// The germs ride their arcs' ellipses: `e₁`'s the split's positive
/// branch, `e₂`'s its negative one ([`crate::ring_path::LoopSplit`]'s
/// bisector).
///
/// # Errors
///
/// [`BooleanError::NodeUnbuilt`] for arcs whose turn toward the axis
/// reads Zero at the walls' radius; either
/// face's normal refusing at the node; [`germ_order`]'s and
/// [`mint_ring`]'s refusals; the Euler operators'.
#[allow(clippy::too_many_arguments)]
pub(super) fn mint_node<T: Decide + crate::props::AtRestPolicy>(
    (a, b): (&mut Body<T>, &mut Body<T>),
    faces: (crate::entity::FaceKey, crate::entity::FaceKey),
    (node, radial): (geom_core::Point3<T>, Vec3<T>),
    tangents: [Vec3<T>; 2],
    axes: [Vec3<T>; 2],
    arm: T,
    band: Band,
    tol: Tol,
) -> Result<RingOut<T>, BooleanError> {
    // Each face's outward normal at the node, which it lies on.
    let outward = |operand, body: &Body<T>, face| -> Result<Vec3<T>, BooleanError> {
        match crate::face_normal::face_outward_normal_at(body, face, node, band) {
            Ok(Some(n)) => Ok(n.vec()),
            Ok(None) => Err(BooleanError::ClassificationInvariant {
                what: "a node's face has no outward normal",
            }),
            Err(refusal) => Err(BooleanError::of_pierced_normal(refusal, operand, face)),
        }
    };
    let normal = outward(Operand::B, b, faces.1)?;
    let cells = (
        faces,
        (super::Locus::InFace(faces.0), super::Locus::InFace(faces.1)),
    );
    // **The runs, by the walls' senses.** Round the node the sectors
    // about `±â₁` on A's wall lie outside B's cylinder and those about
    // `±â₂` inside it; on B's wall those about `±â₁` lie inside A's
    // cylinder and those about `±â₂` outside it. The runs are B's
    // sectors inside A's material: those about `±â₁` where A's face
    // bounds a rod (its material on its axis's side), about `±â₂` where
    // it bounds a bore. Each wall's sense is its outward normal against
    // the radial `node − p`, which the walls, tangent at the node, are
    // both square to: `±r`.
    let sense = |operand, body: &Body<T>, face| -> Result<bool, BooleanError> {
        super::node::wall_sense(outward(operand, body, face)?, radial, band)
    };
    let (a_rod, b_rod) = (
        sense(Operand::A, a, faces.0)?,
        sense(Operand::B, b, faces.1)?,
    );
    let run_axis = if a_rod { axes[0] } else { axes[1] };
    // The walls' outward normals at the node are both `±radial`, so
    // they agree where the senses do and are opposed where they differ.
    let normals_agree = a_rod == b_rod;
    // Each tangent turned toward that axis, so a run's two germs bound
    // the sector about it and their opposites the one about its
    // opposite. `t·â` is `cos(θ/2)` or `sin(θ/2)` for the angle `θ`
    // between the axes: the pair's parallel gate decided `θ` apart at
    // the walls' span, and a turn too small to read at their radius is a
    // node this pass cannot build.
    let toward = |t: Vec3<T>| -> Result<Vec3<T>, BooleanError> {
        match decide(
            "bool_node_tangent_toward_axis",
            Margin::levered(t.dot(run_axis), arm),
            band,
        ) {
            Ok(Sign::Positive) => Ok(t),
            Ok(Sign::Negative) => Ok(-t),
            Ok(Sign::Zero) => Err(BooleanError::NodeUnbuilt {
                a_face: faces.0,
                b_face: faces.1,
                why: super::NodeWhy::NearlyParallel,
            }),
            Err(diag) => Err(BooleanError::coincidence(
                Coincide::Sectors,
                DeclarationRead::Moot,
                diag,
            )),
        }
    };
    let (g1, g2) = (toward(tangents[0])?, toward(tangents[1])?);
    let germ = |d: Vec3<T>, branch| -> Germ<T> { (cells, d, branch) };
    let (on_e1, on_e2) = (super::Branch::Positive, super::Branch::Negative);
    // A run's start is the germ its sector is entered by, turning
    // counterclockwise about B's outward normal: `e₂`'s where `e₁`'s
    // tangent turns to `e₂`'s that way.
    let e1_first = match decide(
        "bool_node_run_turn",
        Margin::levered(g1.cross(g2).dot(normal), arm),
        band,
    ) {
        Ok(Sign::Positive) => false,
        Ok(Sign::Negative) => true,
        // The arcs cross square, `g₁ ⟂ g₂` in the tangent plane whose
        // normal this reads, so the margin is `±r`: definite wherever the
        // section table decided the ellipses' minor semi-axis `r`
        // (`geom`'s `ellipse_minor_positive`) on this band.
        Ok(Sign::Zero) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "a node's two arcs leave it along one line",
            });
        }
        Err(diag) => {
            return Err(BooleanError::coincidence(
                Coincide::Sectors,
                DeclarationRead::Moot,
                diag,
            ));
        }
    };
    let run = |d1: Vec3<T>, d2: Vec3<T>| {
        let (x, y) = (germ(d1, on_e1), germ(d2, on_e2));
        if e1_first { (x, y) } else { (y, x) }
    };
    let run_germs = [run(g1, g2), run(-g1, -g2)];
    // A's struts, one per run, hung by the ring's tree.
    let (wa, loop_a) = ring_vertex_in(a, faces.0, node, tol)?;
    let provisional: Vec<PlannedRun<T>> = run_germs
        .iter()
        .map(|&(s, e)| PlannedRun {
            edge: crate::entity::EdgeKey::default(),
            germs: (s, e),
            order: [(s.1, arm), (e.1, arm)],
            bends: [None, None],
        })
        .collect();
    let contact = VfContact {
        vertex: wa,
        face: faces.1,
    };
    let plan = RingPlan {
        contact,
        piercing: Operand::A,
        point: node,
        normal,
        runs: provisional,
    };
    let runs: Vec<(usize, &RingPlan<T>, &PlannedRun<T>)> =
        plan.runs.iter().map(|run| (0, &plan, run)).collect();
    let (tree, _) = ring_struts(&runs, normal, (Operand::A, faces.1), band)?;
    let mut edges = Vec::new();
    let mut strut_edge = BTreeMap::new();
    let mut root_anchor: Option<HalfEdgeKey> = None;
    let mut tips: BTreeMap<usize, (VertexKey, HalfEdgeKey)> = BTreeMap::new();
    // The runs' starts and the tree's facing turn about B's normal, and
    // `mev_null_run` reads a run in A's own turn (the sense theorem's
    // forward wedge, `join`'s module docs). Where the normals agree that
    // is the same turn, and A's runs lie out of B's material: two rods'
    // runs about `±â₁` outside B's cylinder, two bores' about `±â₂`
    // inside it. Where they are opposed A's turn is B's reversed, so each
    // run starts at the germ it ends at in B's, the tree's facing names
    // A's start as it names B's, and the run lies in B's material: a rod's about `±â₁`
    // outside the bore's cylinder, a bore's about `±â₂` inside the rod's.
    let run_side = if normals_agree {
        NewVertexSide::Above
    } else {
        NewVertexSide::Below
    };
    for strut in &tree {
        let (start_germ, end_germ) = run_germs[strut.run];
        let (at, site) = match strut.parent {
            None => (
                wa,
                match root_anchor {
                    None => MevSite::Lone { r#loop: loop_a },
                    Some(he) => MevSite::Fan { he1: he, he2: he },
                },
            ),
            Some(parent) => {
                let &(tip, back) =
                    tips.get(&parent)
                        .ok_or(BooleanError::ClassificationInvariant {
                            what: "a node strut's parent is minted after it",
                        })?;
                (
                    tip,
                    MevSite::Fan {
                        he1: back,
                        he2: back,
                    },
                )
            }
        };
        let mint = a.mev_null_run(site, at, run_side, strut.plus_faces_start)?;
        // The halves face A's start and end; the record keeps B's slot
        // order, slot k one germ in both (the null-edge pair's contract).
        let (created, attr, [a_start, a_end]) = (mint.created, mint.attr, mint.halves);
        let (start_he, end_he) = if normals_agree {
            (a_start, a_end)
        } else {
            (a_end, a_start)
        };
        if strut.parent.is_none() {
            root_anchor.get_or_insert(created.he_plus);
        }
        tips.insert(strut.run, (created.vertex, created.he_minus));
        strut_edge.insert(strut.run, created.edge);
        edges.push(BoolNullEdgeRecord {
            operand: Operand::A,
            at_vertex: at,
            edge: created.edge,
            attr,
            dangling: true,
            germs: [half_germ(start_he, start_germ), half_germ(end_he, end_germ)],
        });
    }
    let plan = RingPlan {
        runs: plan
            .runs
            .into_iter()
            .enumerate()
            .map(|(i, run)| {
                Ok(PlannedRun {
                    edge: *strut_edge
                        .get(&i)
                        .ok_or(BooleanError::ClassificationInvariant {
                            what: "a node's run has no strut on A's side",
                        })?,
                    ..run
                })
            })
            .collect::<Result<_, BooleanError>>()?,
        ..plan
    };
    let mut out = mint_ring(b, &[&plan], band, tol)?;
    edges.append(&mut out.edges);
    out.edges = edges;
    Ok(out)
}

/// A pierce [`faces_holding_pierce`] adds: its strut, its half of the
/// ring, and the margin that put the point on the face's plane, which
/// the ring's tie to the pierce it shares the point with cites.
#[derive(Debug)]
pub(super) struct HeldPierce<T: geom_core::Real> {
    /// The face of the piercing body the ring vertex is minted in.
    pub face: crate::entity::FaceKey,
    pub strut: BoolNullEdgeRecord<T>,
    pub plan: RingPlan<T>,
    pub on: geom_core::MarginDiag,
}

/// **A pierce point inside a face of the piercing body's own** that
/// does not bound the piercing vertex: an operand whose lumps touch,
/// one's edge lying in another's face, pierces the other operand's
/// face where that face crosses its own. The section of the two faces
/// passes the point, and the result holds one vertex there on both
/// lumps, so the face the point lies in takes a ring vertex at it: a
/// second pierce of the pierced face at the point, whose one Out run
/// is the half of the face above the pierced one. [`ring_groups`] puts
/// it in the pierce's ring, and the zips fuse it with the piercing
/// vertex through the ring's vertex.
///
/// Mints each such ring vertex and its run's strut in `piercing_body`
/// and returns them with the pierce's half of its ring. A plane face is
/// read where the point is on its plane or in band of it, a curved face
/// whose box `box_holds` says holds the point, on a carrier none of the
/// piercing vertex's own faces lies on (their siblings on one carrier,
/// which a valid body never lets overlap them), and where the point is
/// not definitely off that carrier.
///
/// **Every pierce of `plans` is read ([`held_reads`]) before any is
/// minted ([`mint_held`])**. A minted ring vertex's strut stands in its
/// face as a ring, so a face holding two points, read at the second
/// after the first's mint, would read its trim unplaced. Two pierces of
/// one face at one point, a pose no row reaches, refuses
/// [`BooleanError::HeldPierceUnbuilt`]. A definite refusal
/// at any face outranks an escalation at any other, as in the pierce
/// pass this serves.
///
/// **On a curved face, or over one**, the run is the plane case's. Its
/// germs are the two faces' section's tangents at the point, `±n × n_p`
/// across their tangent planes, exact for every smooth pair as
/// [`pierce_germ_dir`] argues. Its half is charged for the curved face's
/// bend as a pierce sector's side is (`sectors::sagitta_charge`): what
/// the first order puts above the pierced face must stand off it past
/// the sagitta within the pierce's arm. [`BooleanError::HeldPierceUnbuilt`]
/// refuses before anything is written where a curved face's trim cannot
/// say whether it holds the point (nothing is guessed), and where both
/// faces are curved or the pierced face is a spline, poses no row
/// reaches.
///
/// # Errors
///
/// [`BooleanError::HeldPierceUnbuilt`] as above; a coincidence or an
/// escalation where the point's place on a face's carrier, its place in
/// a face, or the two faces' angle is in band;
/// [`BooleanError::CurvedSectorSideUnsupported`], or an escalation of
/// [`super::BooleanDecision::PierceCurvature`], where the bends' charge
/// does not clear the band; and
/// [`BooleanError::ClassificationInvariant`] where a face holding the
/// point lies in the pierced face's tangent plane, which a valid operand
/// cannot reach.
pub(super) fn faces_holding_pierce<T: Decide + crate::props::AtRestPolicy>(
    sections: &super::section_table::SectionTable<T>,
    piercing_body: &mut Body<T>,
    pierced_body: &Body<T>,
    box_holds: &dyn Fn(crate::entity::FaceKey, geom_core::Point3<T>) -> bool,
    plans: &[&RingPlan<T>],
    band: Band,
    tol: Tol,
) -> Result<Vec<HeldPierce<T>>, BooleanError> {
    let mut reads: Vec<HeldRead<T>> = Vec::new();
    let mut escalated = None;
    for plan in plans {
        for read in held_reads(
            sections,
            &*piercing_body,
            pierced_body,
            box_holds,
            plan,
            band,
        )? {
            match read {
                // Two pierces of one face at one point would mint two ring
                // vertices there. No row reaches the pose, so no answer
                // here is checked: it refuses, before anything is written.
                Ok(read) => {
                    for earlier in reads.iter().filter(|r| r.face == read.face) {
                        match super::one_vertex_at(earlier.point, read.point, band) {
                            Ok(Some(_)) => {
                                let kind = |body: &Body<T>, face| {
                                    body.face_surface_linked(
                                        face,
                                        proven(&body.faces, face, EntityId::Face),
                                    )
                                    .kind()
                                };
                                return Err(BooleanError::HeldPierceUnbuilt {
                                    operand: read.piercing,
                                    face: read.face,
                                    pierced: read.pierced,
                                    kind: kind(piercing_body, read.face),
                                    pierced_kind: kind(pierced_body, read.pierced),
                                    placed: true,
                                });
                            }
                            Ok(None) => {}
                            Err(diag) => {
                                escalated.get_or_insert(BooleanError::Escalated {
                                    decision: super::BooleanDecision::VertexOnVertex,
                                    diag,
                                });
                            }
                        }
                    }
                    reads.push(read);
                }
                // **A definite refusal outranks an escalation**, as in the
                // pierce pass this serves: every face of every pierce is
                // read, so which is read first does not choose between
                // the two.
                Err(e @ BooleanError::Escalated { .. }) => {
                    escalated.get_or_insert(e);
                }
                Err(definite) => return Err(definite),
            }
        }
    }
    if let Some(escalated) = escalated {
        return Err(escalated);
    }
    reads
        .into_iter()
        .map(|read| mint_held(piercing_body, read, tol))
        .collect()
}

/// What [`held_reads`] decided of one face holding one pierce point,
/// for [`mint_held`] to write.
struct HeldRead<T: geom_core::Real> {
    face: crate::entity::FaceKey,
    piercing: Operand,
    pierced: crate::entity::FaceKey,
    point: geom_core::Point3<T>,
    /// The pierced face's outward normal at the point.
    normal: Vec3<T>,
    /// The run's start germ direction, `n × n_pierced`, normalized.
    dir: Vec3<T>,
    arm: T,
    cells: (
        (crate::entity::FaceKey, crate::entity::FaceKey),
        (super::Locus, super::Locus),
    ),
    branch: super::Branch,
    on: geom_core::MarginDiag,
}

/// [`faces_holding_pierce`]'s reading of `plan`'s point against every
/// face of `piercing_body`, writing nothing: each face that holds the
/// point, or that refuses. The outer error is the plan's own.
fn held_reads<T: Decide + crate::props::AtRestPolicy>(
    sections: &super::section_table::SectionTable<T>,
    piercing_body: &Body<T>,
    pierced_body: &Body<T>,
    box_holds: &dyn Fn(crate::entity::FaceKey, geom_core::Point3<T>) -> bool,
    plan: &RingPlan<T>,
    band: Band,
) -> Result<Vec<Result<HeldRead<T>, BooleanError>>, BooleanError> {
    let (p, piercing, pierced) = (plan.point, plan.piercing, plan.contact.face);
    // The pierce's own neighbourhood meters each reading: the shortest
    // arm its runs were ordered at.
    let arm = plan
        .runs
        .iter()
        .flat_map(|run| run.order.map(|(_, arm)| arm))
        .reduce(T::min)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "a pierce ring's pierce has no run",
        })?;
    let faces: Vec<_> = piercing_body.faces().map(|(k, _)| k).collect();
    let kind_of = |body: &Body<T>, face| {
        body.face_surface_linked(face, proven(&body.faces, face, EntityId::Face))
            .kind()
    };
    let unbuilt = |face, kind, placed| BooleanError::HeldPierceUnbuilt {
        operand: piercing,
        face,
        pierced,
        kind,
        pierced_kind: kind_of(pierced_body, pierced),
        placed,
    };
    // The faces the piercing vertex bounds hold the point on their
    // boundary, and are not read.
    let own: Vec<_> = piercing_body
        .vertex_orbit_linked(plan.contact.vertex)
        .into_iter()
        .map(|he| piercing_body.face_of_linked(he))
        .collect();
    let extent = super::carrier_eq::ConsumedExtent::new(geom_brep::ExtentBall::new(p, arm));
    let read_face = |face| -> Result<Option<HeldRead<T>>, BooleanError> {
        // The face's outward normal at the point, its bend there (`None`
        // on a plane), and the reading that puts the point on it.
        let (held_normal, held_lever, on) = match face_plane(piercing_body, face) {
            None => {
                // A curved face is read only where its box can hold the
                // point, as a plane is only where the point is on it.
                if !box_holds(face, p) {
                    return Ok(None);
                }
                // Off the carrier, the face holds nothing: decided first,
                // as the plane arm decides its plane first. A spline face
                // has no residual, and its trim is asked.
                let surface = piercing_body
                    .face_surface_linked(face, proven(&piercing_body.faces, face, EntityId::Face));
                let side = (!matches!(
                    surface.kind(),
                    geom::SurfaceKind::Nurbs | geom::SurfaceKind::Approx
                ))
                .then(|| {
                    geom_core::k_stats::decide_reported(
                        "bool_curved_contain_carrier",
                        Margin::of(geom_brep::implicit_residual(surface, p)),
                        band,
                    )
                });
                if matches!(side, Some(Ok(ref d)) if d.sign != Sign::Zero) {
                    return Ok(None);
                }
                let held_lever = PierceLever::of(surface, p).within(arm);
                // A face on one of their carriers is their sibling there,
                // which a valid body never lets overlap them: it holds
                // the point outside.
                if own
                    .iter()
                    .any(|&o| one_carrier(piercing_body, o, face, &extent, band))
                {
                    return Ok(None);
                }
                let (placement, _, _) =
                    super::contain::curved_face_placement_decided(piercing_body, face, p, band)
                        .map_err(|e| super::reduce::esc(e, piercing, face))?;
                let kind = kind_of(piercing_body, face);
                match placement {
                    super::contain::CurvedPlacement::Trim(Some(FaceContainment::In)) => {}
                    // Where the trim cannot say, nothing is guessed: the
                    // face may hold the point (D4).
                    super::contain::CurvedPlacement::Trim(None) => {
                        return Err(unbuilt(face, kind, false));
                    }
                    // Off the face, or on its boundary, which the sweep's
                    // pierces and splits hold.
                    super::contain::CurvedPlacement::OffCarrier
                    | super::contain::CurvedPlacement::Trim(Some(_)) => return Ok(None),
                }
                // A spline face's trim never places a point In; were one
                // to, the face has no residual and no normal to build on.
                let Some(side) = side else {
                    return Err(unbuilt(face, kind, true));
                };
                let normal = match crate::face_normal::face_outward_normal_at(
                    piercing_body,
                    face,
                    p,
                    band,
                ) {
                    Ok(Some(n)) => n.vec(),
                    Ok(None) => return Err(unbuilt(face, kind, true)),
                    Err(refusal) => {
                        return Err(BooleanError::of_pierced_normal(refusal, piercing, face));
                    }
                };
                (normal, Some(held_lever), side)
            }
            Some(plane) => {
                let side = geom_core::k_stats::decide_reported(
                    "bool_vertex_face_side",
                    Margin::of((p - plane.origin).dot(plane.normal)),
                    band,
                );
                if matches!(side, Ok(ref d) if d.sign != Sign::Zero) {
                    return Ok(None);
                }
                // A plane the point is in band of matters only where the
                // point lies in the face. On the face's boundary the point
                // is a vertex or an edge of the piercing body: the sweep's
                // pierces and splits hold it.
                let (at, _) =
                    super::contain::contfp_decided(piercing_body, face, plane.normal, p, band)
                        .map_err(|e| super::reduce::esc(e, piercing, face))?;
                if at != FaceContainment::In {
                    return Ok(None);
                }
                (plane.normal, None, side)
            }
        };
        // The point lies in the face: its place on the face's carrier
        // in band is a coincidence, decided nowhere else.
        let on = on
            .map_err(|diag| {
                BooleanError::coincidence(Coincide::VertexOnFace, DeclarationRead::Moot, diag)
            })?
            .margin;
        // The bend the run is charged for: the curved face's of the two.
        // Both curved is a pose no row reaches: the Boolean refuses each
        // such pair before any pierce (a rod against a lying cylinder, a
        // coaxial ball or cone), so no answer here is checked, and it
        // refuses. A spline pierced face has no bend to read.
        let lever = match (held_lever, kind_of(pierced_body, pierced)) {
            (held, geom::SurfaceKind::Plane) => held,
            (None, geom::SurfaceKind::Nurbs | geom::SurfaceKind::Approx) => {
                return Err(unbuilt(face, geom::SurfaceKind::Plane, true));
            }
            (None, _) => Some(
                PierceLever::of(
                    pierced_body.face_surface_linked(
                        pierced,
                        proven(&pierced_body.faces, pierced, EntityId::Face),
                    ),
                    p,
                )
                .within(arm),
            ),
            (Some(_), _) => return Err(unbuilt(face, kind_of(piercing_body, face), true)),
        };
        let int = held_normal.cross(plan.normal);
        match decide("bool_germ_line", Margin::levered(int.norm(), arm), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "a pierce point lies inside a face of the piercing operand in the \
                           pierced face's tangent plane",
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
        // **The run's half, charged for the bend.** The face's half
        // above the pierced face leaves it at the sine `|n × n_pierced|`
        // along the face's direction across the section. Past the curved
        // face's sagitta, `l²/ρ` at `l` (`sectors::sagitta_charge`
        // carries why without the ½), it stands at least `sin·l − l²/ρ`
        // off the pierced face: the charge, read where it peaks within
        // the pierce's arm. Two planes have no bend, and are not charged.
        // The normal doors refuse the points with no bend radius (a
        // cone's apex, a degenerate torus) before this reads one.
        if let Some(lever) = lever {
            match crate::validate::decide_reported(
                "bool_held_pierce_curved",
                Margin::of(super::sectors::sagitta_charge(int.norm(), lever, arm)),
                band,
            ) {
                Ok(decided) => {
                    if let Some(refused) = geom_brep::recourse::Refused::of(decided, band) {
                        return Err(BooleanError::CurvedSectorSideUnsupported { verdict: refused });
                    }
                }
                Err(diag) => {
                    return Err(BooleanError::Escalated {
                        decision: super::BooleanDecision::PierceCurvature,
                        diag,
                    });
                }
            }
        }
        // The face's one run is its half above the pierced face, from
        // the germ `n × n_pierced` round to its opposite, as a sector's
        // transitions read (`pierce_germ_dir`).
        let cells = match piercing {
            Operand::A => (
                (face, pierced),
                (super::Locus::InFace(face), super::Locus::InFace(pierced)),
            ),
            Operand::B => (
                (pierced, face),
                (super::Locus::InFace(pierced), super::Locus::InFace(face)),
            ),
        };
        Ok(Some(HeldRead {
            face,
            piercing,
            pierced,
            point: p,
            normal: plan.normal,
            dir: int.normalize(),
            arm,
            cells,
            branch: germ_branch(
                sections,
                (piercing_body, pierced_body),
                piercing,
                cells,
                p,
                band,
            )?,
            on,
        }))
    };
    Ok(faces
        .into_iter()
        .filter(|face| !own.contains(face))
        .filter_map(|face| read_face(face).transpose())
        .collect())
}

/// Mints [`HeldRead`]'s ring vertex in its face and its run's strut,
/// and returns them with the pierce's half of its ring.
fn mint_held<T: Decide + crate::props::AtRestPolicy>(
    piercing_body: &mut Body<T>,
    read: HeldRead<T>,
    tol: Tol,
) -> Result<HeldPierce<T>, BooleanError> {
    let HeldRead {
        face,
        piercing,
        pierced,
        point,
        normal,
        dir: d,
        arm,
        cells,
        branch,
        on,
    } = read;
    let (w, ring_loop) = ring_vertex_in(piercing_body, face, point, tol)?;
    let germs: (Germ<T>, Germ<T>) = ((cells, d, branch), (cells, -d, branch));
    let mint = piercing_body.mev_null_run(
        MevSite::Lone { r#loop: ring_loop },
        w,
        NewVertexSide::Above,
        true,
    )?;
    let (created, attr, [start_he, end_he]) = (mint.created, mint.attr, mint.halves);
    let rec = BoolNullEdgeRecord {
        operand: piercing,
        at_vertex: w,
        edge: created.edge,
        attr,
        dangling: true,
        germs: [half_germ(start_he, germs.0), half_germ(end_he, germs.1)],
    };
    Ok(HeldPierce {
        face,
        strut: rec,
        on,
        plan: RingPlan {
            contact: VfContact {
                vertex: w,
                face: pierced,
            },
            piercing,
            point,
            normal,
            runs: vec![PlannedRun {
                edge: created.edge,
                germs,
                order: [(d, arm), (-d, arm)],
                bends: [None, None],
            }],
        },
    })
}

/// Whether faces `a` and `b` of `body` lie on one carrier, decided: by
/// the carrier ladder ([`super::carrier_eq`](mod@super::carrier_eq)) over `extent` where it has
/// an arm for both, else datum by datum on their stored descriptions,
/// each within the band (`bool_held_carrier_datum`). Anything undecided
/// is `false`, which reads the face rather than passing it over.
fn one_carrier<T: Decide>(
    body: &Body<T>,
    a: crate::entity::FaceKey,
    b: crate::entity::FaceKey,
    extent: &super::carrier_eq::ConsumedExtent<T>,
    band: Band,
) -> bool {
    use super::carrier_eq::{CarrierRelation, carrier_eq};
    if let (Some(x), Some(y)) = (
        super::carrier_pair::face_carrier(body, a),
        super::carrier_pair::face_carrier(body, b),
    ) {
        return matches!(
            carrier_eq(&x, &y, extent, band),
            Ok(CarrierRelation::SameOriented | CarrierRelation::SameOpposite)
        );
    }
    let surface = |f| body.face_surface_linked(f, proven(&body.faces, f, EntityId::Face));
    match surface(a).paired_with(surface(b)) {
        geom::SurfacePairing::Analytic(data) => data.pairs().all(|(_, x, y)| {
            x.scalars().zip(y.scalars()).all(|(p, q)| {
                matches!(
                    decide("bool_held_carrier_datum", Margin::of(p - q), band),
                    Ok(Sign::Zero)
                )
            })
        }),
        _ => false,
    }
}

/// **The germs' order round the ring vertex**: indices into `germs`
/// (each a direction and its arm, a run's two germs at `2r` and
/// `2r + 1`), clockwise about the pierced face's outward `normal` from
/// `germs[0]`. Every comparison is [`super::insert::strut_order`]'s
/// from `germs[0]`, levered at the shortest of its three germs' arms. A
/// germ along `germs[0]` has no place before or after it at first
/// order, so each germ's side of `germs[0]` is read first at every arm
/// a reading of it uses, its own pair's and each comparison's: in band
/// with its sense along (`bool_strut_side`, `bool_dir_same`), it
/// refuses as the sectors' coincidence rather than stand at `germs[0]`.
///
/// Two germs along one direction, decided so, are ordered by the rule
/// for their tie:
///
/// - of one pierce, by how their sections curve off it in the face
///   (`curving`, the sign of a [`Turn`]; `None` where it reads nothing,
///   which refuses as the first-order tie);
/// - of different pierces (`member`), two cones touching along a ray in
///   the face: each cone's run keeps its own side of the ray, so the two
///   take the order in which their runs' chords nest
///   ([`tie_breaks_after`]). A third germ on that ray has no such order,
///   and refuses [`BooleanError::PinchRaysInAFace`] (`site`: the
///   piercing operand and the face).
///
/// Any other reading that does not decide refuses; nothing orders the
/// germs otherwise. Every reading is levered at the shortest arm among
/// the directions it reads. Beside the order it returns each tie of
/// different pierces, the germ first clockwise first.
fn germ_order<T: Decide>(
    germs: &[(Vec3<T>, T)],
    member: &[usize],
    normal: Vec3<T>,
    site: (Operand, crate::entity::FaceKey),
    curving: impl Fn(Turn, T) -> Result<Option<Sign>, BooleanError>,
    band: Band,
) -> Result<(Vec<usize>, Vec<[usize; 2]>), BooleanError> {
    let (from, from_arm) = germs[0];
    let refuse = |diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag);
    let side = |i: usize, j: usize, arm: T| -> Result<GermSide, BooleanError> {
        let (g, h) = (germs[i].0, germs[j].0);
        let side = Margin::levered(g.cross(h).dot(normal), arm);
        match crate::validate::decide_nonzero("bool_strut_side", side, band) {
            Ok(sign) => Ok(GermSide::Apart(sign)),
            Err(diag) if diag.margin.rejected_sign() == Some(Sign::Zero) => {
                Ok(if super::sectors::direction_sense(g, h, arm, band)? {
                    GermSide::Along(diag)
                } else {
                    GermSide::Opposite
                })
            }
            Err(diag) => Err(refuse(diag)),
        }
    };
    let along = |i: usize, j: usize, arm: T| -> Result<Option<_>, BooleanError> {
        Ok(match side(i, j, arm)? {
            GermSide::Along(diag) => Some(diag),
            GermSide::Apart(_) | GermSide::Opposite => None,
        })
    };
    // The ties of different pierces: a germ in a second one puts three
    // on one ray.
    let mut ties: Vec<[usize; 2]> = Vec::new();
    // Of germs `i` and `j` along one direction, whether `i` comes after
    // `j` clockwise.
    let mut after = |i: usize, j: usize, diag| -> Result<bool, BooleanError> {
        if member[i] != member[j] {
            if ties.iter().flatten().any(|&k| k == i || k == j) {
                return Err(BooleanError::PinchRaysInAFace {
                    operand: site.0,
                    face: site.1,
                });
            }
            ties.push([i, j]);
            return tie_breaks_after(germs, (i, j), normal, band);
        }
        match curving(Turn { from: j, to: i }, germs[i].1.min(germs[j].1))? {
            Some(Sign::Negative) => Ok(true),
            Some(Sign::Positive) => Ok(false),
            Some(Sign::Zero) | None => Err(refuse(diag)),
        }
    };
    let apart = |g: Vec3<T>, arm: T| -> Result<(), BooleanError> {
        let side = Margin::levered(g.cross(from).dot(normal), arm);
        match crate::validate::decide_nonzero("bool_strut_side", side, band) {
            Err(diag) if super::sectors::direction_sense(g, from, arm, band)? => Err(refuse(diag)),
            _ => Ok(()),
        }
    };
    // `order` holds the germs strictly off `germs[0]`'s direction; a
    // germ along it stands just after it (`head`) or, a full turn
    // round, just before it (`tail`), each in its own order.
    let (mut order, mut head, mut tail): (Vec<usize>, Vec<usize>, Vec<usize>) =
        (vec![0], Vec::new(), Vec::new());
    for (i, &(g, g_arm)) in germs.iter().enumerate().skip(1) {
        if let Some(diag) = along(i, 0, from_arm.min(g_arm))? {
            let ray = if after(0, i, diag)? {
                &mut tail
            } else {
                &mut head
            };
            let mut at = ray.len();
            for (slot, &j) in ray.iter().enumerate() {
                // Two germs each along `germs[0]` may still read apart.
                let later = match side(i, j, g_arm.min(germs[j].1))? {
                    GermSide::Along(diag) => after(i, j, diag)?,
                    GermSide::Apart(sign) => sign == NonzeroSign::Positive,
                    GermSide::Opposite => {
                        return Err(BooleanError::ClassificationInvariant {
                            what: "two germs along a ring's first germ point opposite ways",
                        });
                    }
                };
                if !later {
                    at = slot;
                    break;
                }
            }
            ray.insert(at, i);
            continue;
        }
        apart(g, from_arm.min(g_arm))?;
        let mut at = order.len();
        for (slot, &j) in order.iter().enumerate().skip(1) {
            let (h, h_arm) = germs[j];
            let arm = from_arm.min(g_arm).min(h_arm);
            if let Some(diag) = along(i, j, g_arm.min(h_arm))? {
                if !after(i, j, diag)? {
                    at = slot;
                    break;
                }
                continue;
            }
            apart(g, arm)?;
            apart(h, arm)?;
            let tie = || {
                if member[i] == member[j] {
                    curving(Turn { from: i, to: j }, arm)
                } else {
                    Ok(None)
                }
            };
            if super::insert::strut_order((from, arm), normal, (g, h), arm, tie, band)? {
                at = slot;
                break;
            }
        }
        order.insert(at, i);
    }
    let order = [&order[..1], &head, &order[1..], &tail].concat();
    // A tie's two germs stand next to each other round the vertex.
    let mut at = vec![0; order.len()];
    for (p, &g) in order.iter().enumerate() {
        at[g] = p;
    }
    let next = |k: usize, l: usize| (at[k] + 1) % order.len() == at[l];
    let ties = ties
        .into_iter()
        .map(|[i, j]| match (next(i, j), next(j, i)) {
            (true, _) => Ok([i, j]),
            (_, true) => Ok([j, i]),
            _ => Err(BooleanError::ClassificationInvariant {
                what: "a tie's two germs do not stand next to each other round the ring vertex",
            }),
        })
        .collect::<Result<_, _>>()?;
    Ok((order, ties))
}

/// Where germ `j` lies from germ `i` about the pierced face's normal
/// ([`germ_order`], `bool_strut_side`).
enum GermSide {
    /// Off `i`'s line: the sign of `(gᵢ × gⱼ)·n`.
    Apart(NonzeroSign),
    /// Along `i`'s direction, decided so by this reading.
    Along(geom_core::Indeterminate),
    /// Along the ray opposite `i`.
    Opposite,
}

/// Two germs along one direction, `to` read turned from `from` one
/// order down: `Positive` where `to` curves counterclockwise of `from`
/// about the pierced face's normal ([`geom_brep::tied_germ_turn`]'s
/// germ 0 and germ 1).
#[derive(Clone, Copy, Debug)]
struct Turn {
    from: usize,
    to: usize,
}

/// Of two germs `i` and `j` along one direction, whether `i` comes
/// after `j` clockwise: where `i`'s partner (the other germ of its run)
/// lies nearer clockwise from the shared direction than `j`'s. Then
/// `i`'s chord nests inside `j`'s; the other order would cross them.
fn tie_breaks_after<T: Decide>(
    germs: &[(Vec3<T>, T)],
    (i, j): (usize, usize),
    normal: Vec3<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    let (at, at_arm) = germs[i];
    let ((pi, pi_arm), (pj, pj_arm)) = (germs[i ^ 1], germs[j ^ 1]);
    let arm = at_arm.min(pi_arm).min(pj_arm);
    super::insert::strut_order((at, arm), normal, (pi, pj), arm, || Ok(None), band)
}

/// **The refusal of a flat sector whose pair the glue door did not
/// glue.** The tilt between the sector's normal and the pierced face's
/// at `p`, levered at the sector's reach, proposes coplanarity. A tilt
/// the bounds read On and the normals do not (in band, or decided off:
/// a thin sector tilted about one bound rises at its arm by more than
/// at its other bound) leaves the question undecided at this tolerance,
/// and the margin that decides it is the bounds' own: a smaller
/// tolerance reads the steeper bound off the plane and the sector with
/// it. A tilt decided Zero reads the pair one carrier about the corner,
/// which no glue backs ([`super::unglued_corner`]).
///
/// C8: a CURVED on-carrier sector is opened by the glue door's verdict
/// and by nothing else. A tangency the witness lane verifies is glued
/// and lumps by its curving, so what reaches here unglued lies outside
/// the lane (a torus, a cone, a spline), or is a pair whose reading the
/// door swallowed: an in-band tangency surfaces here, not as its
/// escalation (filed:
/// `work/contacthold/the-glue-doors-tangency-swallows-its-escalations.md`).
/// An unglued PLANAR sector on a curved pierced face would be a plane
/// tangent to the curved face with a bound leaving along it, and no
/// public operand reaches that: the crossing layer refuses the bound's
/// tangent touch first (a prism tangent to a cylinder along a ruling
/// refuses `CurvedPierceUnsupported` there, in both operand orders).
/// The guard keeps this door for that state all the same.
fn unglued_flat_sector<T: Decide>(
    (piercing_body, piercing, s): (&Body<T>, Operand, &BoolSector<T>),
    (pierced_op, pierced, pierced_kind): (Operand, crate::entity::FaceKey, geom::SurfaceKind),
    plane: Option<super::plane_eq::PlaneDesc<T>>,
    (p, n_pierced): (geom_core::Point3<T>, Vec3<T>),
    declared: &super::GluedPairs<T>,
    band: Band,
) -> BooleanError {
    let pair = (piercing, s.face, pierced_op, pierced);
    let m = Margin::levered(s.normal.vec().cross(n_pierced).norm(), s.span());
    let tilt = decide("bool_sector_coplanar", m, band);
    if !matches!(tilt, Ok(Sign::Zero)) {
        // The refusal names the one-carrier class the pair's senses make
        // it against a planar pierced face, or none where it cannot read
        // them. A decided tilt names no class: no glue settles it.
        let decided_tilt = matches!(tilt, Ok(Sign::Positive | Sign::Negative));
        let senses = (plane.is_some() && !decided_tilt)
            .then(|| super::plane_eq::senses(s.normal.vec(), n_pierced, s.arm, band))
            .flatten();
        let read = super::sectors_read(declared, pair, senses);
        return match s.bounds_off(n_pierced, band) {
            Err(diag) => BooleanError::coincidence(Coincide::Sectors, read, diag),
            Ok(_) => BooleanError::ClassificationInvariant {
                what: "a sector bound read On departs definitely from the plane",
            },
        };
    }
    let curved = |body: &Body<T>, operand, face| {
        let kind = body
            .get_face(face)
            .and_then(|f| body.get_surface(f.surface))
            .map_or(geom::SurfaceKind::Nurbs, geom::Surface::kind);
        BooleanError::CurvedBooleanUnsupported {
            operand,
            face,
            kind,
        }
    };
    let Some(super::carrier_eq::CarrierDesc::Plane {
        origin: sector_origin,
        normal: sector_normal,
    }) = super::carrier_pair::face_carrier(piercing_body, s.face)
    else {
        return curved(piercing_body, piercing, s.face);
    };
    let Some(plane) = plane else {
        return BooleanError::CurvedBooleanUnsupported {
            operand: pierced_op,
            face: pierced,
            kind: pierced_kind,
        };
    };
    let senses = super::plane_eq::senses(s.normal.vec(), n_pierced, s.arm, band);
    // Over the sector about `p`, out to its farther bound's reach, where
    // the tilt above read it coplanar.
    let extent = super::carrier_eq::ConsumedExtent::new(geom_brep::ExtentBall::new(p, s.span()));
    match super::carrier_eq::carrier_eq_reading(
        &super::carrier_eq::CarrierDesc::Plane {
            origin: sector_origin,
            normal: sector_normal,
        },
        &super::carrier_eq::CarrierDesc::Plane {
            origin: plane.origin,
            normal: plane.normal,
        },
        &extent,
        band,
    ) {
        // The parallel rung reads the tilt above's margin, decided zero,
        // and the offset is `p`'s standing off two planes it lies on.
        Ok((super::PlaneRelation::Distinct, _)) => BooleanError::ClassificationInvariant {
            what: "geometrically coplanar sector with definitely-distinct plane",
        },
        // One plane at the sector's arm, for a pair the glue door did
        // not glue over the faces.
        Ok((rel, margin)) => super::unglued_corner(
            declared,
            pair,
            senses,
            s.bounds_off(n_pierced, band).err(),
            // The ladder decides one carrier only on a margin, so a
            // reading without one is a broken invariant.
            || match margin {
                Some(margin) => {
                    super::unglued_coincidence(margin, declared.carriers_read(pair, rel), band)
                }
                None => BooleanError::ClassificationInvariant {
                    what: "a corner pair the ladder read one carrier on no margin",
                },
            },
        ),
        // In band, unreachable here: `bool_sector_coplanar` read this
        // same margin (the two normals' cross, at the sector's reach)
        // zero above. The door is `recl`'s, which reaches it at another
        // reach.
        Err(PlaneEqError::Escalated { rung, diag }) => {
            BooleanError::plane_identity(rung, declared.on_pair_door(pair, senses), diag)
        }
        Err(PlaneEqError::Undecided {
            coincidence,
            relation,
        }) => super::undecided_coincidence(
            coincidence,
            [(piercing, s.face), (pierced_op, pierced)],
            declared.carriers_read(pair, relation),
        ),
    }
}

/// On-edge resolution (module docs; the deliberate divergence), after
/// Delta 2's lumps.
///
/// An edge entry's On is a real one for a LINE edge: its far vertex is
/// within the band of the plane (`pierce_side`, `Reach::Chord`), so the
/// edge lies in it to the tolerance and the resolution is ε-true of it.
/// A conic edge's On against a plane is as real: its height bound over
/// the whole arc reads within the band (`Reach::Conic`,
/// `sectors::conic_side`); a tie whose bend reads no side is lumped or
/// refuses before this runs. Against a curved face it is a first-order
/// tie
/// (`work/contacthold/conic-ties-outside-the-vertex-on-face-pass-still-read-first-order.md`).
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
            let Some(on) = entries[k].on else {
                unreachable!(
                    "a bisector classed On with no Zero of its own: a bisector reads On only at \
                     its own first-order Zero, or from a lump of two bounds both read On ({:?})",
                    entries[k].he
                )
            };
            return Err(super::sectors::bisector_zero_refusal(on, band));
        }
        entries[k].class = super::sectors::fold_on_bound(prev, next);
    }
    Ok(())
}

/// The germ direction at a pierce-site transition: the unit
/// intersection direction of the transition sector's face plane with
/// the pierced face's TANGENT plane at the pierce point, signed by which
/// of the sector's bounds the Out run holds ([`OutBound`]).
///
/// **The sign is the transition's, not a second reading.** The sector
/// sweeps CCW about its outward normal `n_s` from `start` to `end`, and
/// its plane's directions on the pierced normal's side are the half-plane
/// about `u`, the pierced normal's component in that plane. A quarter
/// turn CCW carries `u` to `n_s × u = n_s × n_p`, so a sweep starting
/// above the section line leaves that half-plane along `n_s × n_p`, and
/// one starting below enters it along `−n_s × n_p`; a sweep under a
/// full turn crosses the line once more only after passing its end. An
/// Out start bound (above ≙ Out) and an In end bound give `n_s × n_p`,
/// the reverse its negation. Where a bound read On folded into the In
/// run, the line runs along that bound to within the band, and since a
/// sector is convex (under a half turn) the ray this picks is the one
/// along that bound, not its opposite: the germ the fold put there.
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
/// Both normals arrive outward already (S10) — `s.normal` from
/// `sectors::sector_face`, `pierced_normal` from the one door
/// ([`crate::face_normal`]) — and the sign above reads them as such, so
/// neither is multiplied again here.
pub(super) fn pierce_germ_dir<T: Decide>(
    s: &super::sectors::BoolSector<T>,
    pierced_normal: geom_core::Vec3<T>,
    out: OutBound,
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
    let germ = match out {
        OutBound::Start => d,
        OutBound::End => -d,
    };
    // D9 row 5: where membership singles out one ray it is this one.
    // Calibrated on JOIN's near-tangent census at ε = 1e-9, 1e-6 and
    // 1e-12: no disagreement over 855k such readings (PR 4581's review).
    // Where a bound reads On, membership at the arm can read both rays
    // out, which is no disagreement.
    #[cfg(debug_assertions)]
    {
        let moot = DeclarationRead::Moot;
        let member = |dir| super::sectors::within(s, dir, false, moot, band);
        if let (Ok(along), Ok(against)) = (member(germ), member(-germ)) {
            debug_assert!(
                along || !against,
                "the transition's germ {germ:?} ({out:?}) is outside the sector \
                 {:?}→{:?} (normal {:?}) and its opposite inside, against {pierced_normal:?}",
                s.start,
                s.end,
                s.normal.vec(),
            );
        }
    }
    Ok(germ)
}

/// Which bound of a pierce transition sector the Out run holds: the
/// other reads In.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum OutBound {
    /// The sector's CCW-first bound: the sector precedes the run.
    Start,
    /// The sector's CCW-last bound: the sector closes the run.
    End,
}

/// A pierce germ as a run reads it: `(A face, B face)` and
/// `(A locus, B locus)`, then its direction and its branch.
type Germ<T> = (
    (
        (crate::entity::FaceKey, crate::entity::FaceKey),
        (super::Locus, super::Locus),
    ),
    geom_core::Vec3<T>,
    super::Branch,
);

/// A pierce germ's branch ([`super::join::germ_branch`]), read at the
/// pierce point `p` with the piercing and pierced bodies in that order.
fn germ_branch<T: Decide>(
    sections: &super::section_table::SectionTable<T>,
    (piercing_body, pierced_body): (&Body<T>, &Body<T>),
    piercing: Operand,
    (faces, loci): (
        (crate::entity::FaceKey, crate::entity::FaceKey),
        (super::Locus, super::Locus),
    ),
    p: geom_core::Point3<T>,
    band: Band,
) -> Result<super::Branch, BooleanError> {
    let (a, b) = match piercing {
        Operand::A => (piercing_body, pierced_body),
        Operand::B => (pierced_body, piercing_body),
    };
    super::join::germ_branch(sections, (a, faces.0), (b, faces.1), loci, p, band)
}

fn half_germ<T: geom_core::Real>(
    he: HalfEdgeKey,
    ((faces, loci), dir, branch): Germ<T>,
) -> super::HalfGerm<T> {
    super::HalfGerm {
        he,
        a_face: faces.0,
        b_face: faces.1,
        a_locus: loci.0,
        b_locus: loci.1,
        dir,
        branch,
    }
}

/// Maximal cyclic Out-runs `(start, len)` (PR 2's `above_runs` on
/// [`SideCode`]; anchored at the first In entry; one-sided
/// neighborhoods have none).
/// A run's two transition sectors, of `n`: the one before its first
/// Out bound, whose start that bound is, and the one holding its last
/// as its end (entry `k` is sector `k`'s end, and sector `k`'s start is
/// entry `k + 1`).
fn run_transitions((first, len): (usize, usize), n: usize) -> [(usize, OutBound); 2] {
    [
        ((first + n - 1) % n, OutBound::Start),
        ((first + len - 1) % n, OutBound::End),
    ]
}

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
    use crate::boolean::sectors::FaceShape;

    /// [`germ_order`]'s order alone, for one pierce's germs.
    fn germ_order_only(
        germs: &[(Vec3<f64>, f64)],
        member: &[usize],
        normal: Vec3<f64>,
        site: (Operand, crate::entity::FaceKey),
        band: Band,
    ) -> Result<Vec<usize>, BooleanError> {
        germ_order(germs, member, normal, site, |_, _| Ok(None), band).map(|(order, _)| order)
    }
    use SideCode::{In, On, Out};

    /// A pierce of `face` by `vertex` at `(x, 0, 0)`, with no runs: what
    /// [`ring_groups`] reads of a plan.
    fn plan_at(vertex: VertexKey, face: crate::entity::FaceKey, x: f64) -> RingPlan<f64> {
        RingPlan {
            contact: VfContact { vertex, face },
            piercing: Operand::A,
            point: geom_core::Point3::new(x, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            runs: Vec::new(),
        }
    }

    /// **The pierces of one face at one point are one class, tied by the
    /// pairs that joined it.** Three pierces of one face at the origin,
    /// a fourth of that face a unit away and a fifth of another face at
    /// the origin: classes {0, 1, 2}, {3}, {4} in first-member order,
    /// and two ties, 0–1 and 0–2, each with its margin; 1–2 joins
    /// nothing new and ties nothing. Red if a class compares only with
    /// its lead and records every Zero (three ties), or groups across
    /// faces. Two pierces of one face a band apart (halfway between the
    /// band's zero and its escalation) escalate `VertexOnVertex`.
    #[test]
    fn ring_groups_close_transitively_and_escalate_in_band() {
        let mut vertices: slotmap::SlotMap<VertexKey, ()> = slotmap::SlotMap::with_key();
        let mut faces: slotmap::SlotMap<crate::entity::FaceKey, ()> = slotmap::SlotMap::with_key();
        let v: Vec<VertexKey> = (0..6).map(|_| vertices.insert(())).collect();
        let (f, g) = (faces.insert(()), faces.insert(()));
        let band = Band::linear(Tol::witness()).unwrap();
        let plans = [
            plan_at(v[0], f, 0.0),
            plan_at(v[1], f, 0.0),
            plan_at(v[2], f, 0.0),
            plan_at(v[3], f, 1.0),
            plan_at(v[4], g, 0.0),
        ];
        let refs: Vec<&RingPlan<f64>> = plans.iter().collect();
        let (groups, ties) = ring_groups(&refs, band).unwrap();
        assert_eq!(groups, vec![vec![0, 1, 2], vec![3], vec![4]], "the classes");
        assert_eq!(
            ties.iter().map(|&(i, j, _)| (i, j)).collect::<Vec<_>>(),
            vec![(0, 1), (0, 2)],
            "the spanning ties"
        );
        // Halfway into the band, at whichever tolerance row runs.
        let gap = 0.5 * (band.zero() + band.escalate());
        let near = [plan_at(v[0], f, 0.0), plan_at(v[5], f, gap)];
        let refs: Vec<&RingPlan<f64>> = near.iter().collect();
        assert!(
            matches!(
                ring_groups(&refs, band),
                Err(BooleanError::Escalated {
                    decision: super::super::BooleanDecision::VertexOnVertex,
                    ..
                })
            ),
            "two pierces a band apart escalate"
        );
    }

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
        let read = |c: &ReductionContacts| match pierced_and_paired(c) {
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
                ReductionContacts {
                    a_on_b: vec![
                        crate::Cited::new(vf(v, f), crate::Cites::decided(0)),
                        crate::Cited::new(vf(w, f), crate::Cites::decided(0)),
                        crate::Cited::new(vf(v, g), crate::Cites::decided(0)),
                    ],
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
                ReductionContacts {
                    b_on_a: vec![crate::Cited::new(vf(w, f), crate::Cites::decided(0))],
                    vv: vec![
                        crate::Cited::new(vv(x, v), crate::Cites::decided(0)),
                        crate::Cited::new(vv(v, w), crate::Cites::decided(0)),
                        crate::Cited::new(vv(x, w), crate::Cites::decided(0)),
                    ],
                    ..Default::default()
                },
                Ok(vec![((Operand::B, w), SectorRead::Pair(v))]),
            ),
            (
                "paired twice",
                ReductionContacts {
                    vv: vec![
                        crate::Cited::new(vv(v, w), crate::Cites::decided(0)),
                        crate::Cited::new(vv(v, x), crate::Cites::decided(0)),
                        crate::Cited::new(vv(x, w), crate::Cites::decided(0)),
                    ],
                    ..Default::default()
                },
                Ok(vec![]),
            ),
            (
                "one key on each operand, pierced by A's and paired by B's",
                ReductionContacts {
                    a_on_b: vec![crate::Cited::new(vf(v, f), crate::Cites::decided(0))],
                    vv: vec![crate::Cited::new(vv(w, v), crate::Cites::decided(0))],
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
    /// solid's, at any depth. Beside a partner that read nothing or
    /// whose cone lies in no half-space, it names a partner to refuse
    /// on (on two boundaries: the row below). Each input but the last
    /// two is a partner a scene of `a_vertex_read_by_two_sector_passes`
    /// reads: an arch, a void in the arch, the cavity's void, an island
    /// in it, and arches apart; no scene reaches the last two, which
    /// stand for the touch's two refusals beside a partner. One
    /// pair alone keeps its rows; pairs alone refuse beside a partner
    /// that reads nothing, against the vertex's edges or the other
    /// partner's (here a partner of no sectors, so on fewer than two
    /// faces).
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
            read: Ok(WedgeRead {
                pointed: true,
                met,
                rows: rows.to_vec(),
            }),
            sectors: Vec::new(),
        };
        let unread = |partner, side| PairRead::<f64> {
            partner,
            side,
            read: Err(crate::boolean::UnreadNeighbourhood::OneFace),
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
                "beside a partner that reads nothing",
                vec![(e, On), (f, Out)],
                vec![read(w, Some(Out), true, &three), unread(x, Some(Out))],
                Err(Some(x)),
            ),
            (
                "beside a partner whose cone lies in no half-space",
                vec![(e, On), (f, Out)],
                vec![
                    read(w, Some(Out), true, &three),
                    PairRead {
                        read: Ok(WedgeRead {
                            pointed: false,
                            met: true,
                            rows: vec![(e, On), (f, Out)],
                        }),
                        ..unread(x, Some(Out))
                    },
                ],
                Err(Some(x)),
            ),
        ];
        let band = Band::linear(Tol::witness()).unwrap();
        for (what, touch, pairs, want) in touches {
            let got = touch_classes::<f64>(&[], &touch, &pairs, &|_| Ok(FaceShape::Plane), band)
                .map_err(|u| match u {
                    TouchUndecided::Beside(p) => p,
                    TouchUndecided::Along(e) => panic!("{what}: {e:?}"),
                });
            assert_eq!(got, want, "{what}");
        }
        let own = VertexKey::from(key(6));
        let (pw, px) = (SectorRead::Pair(w), SectorRead::Pair(x));
        for (what, pairs, want) in [
            (
                "one arch",
                vec![read(w, None, true, &three)],
                Ok((three.to_vec(), None)),
            ),
            (
                "one void's apex",
                vec![read(w, None, false, &three)],
                Ok((three.to_vec(), None)),
            ),
            (
                "one partner that reads nothing names it and why",
                vec![unread(w, None)],
                Ok((
                    Vec::new(),
                    Some((w, crate::boolean::UnreadNeighbourhood::OneFace)),
                )),
            ),
            (
                "an arch beside a partner that reads nothing",
                vec![read(w, None, true, &three), unread(x, None)],
                Err([px, pw]),
            ),
            (
                "a partner that reads nothing beside an arch",
                vec![unread(x, None), read(w, None, true, &three)],
                Err([px, pw]),
            ),
            (
                "two arches, each on fewer than two faces",
                vec![read(w, None, true, &three), read(x, None, true, &three)],
                Err([px, pw]),
            ),
        ] {
            let got = pair_classes(
                Operand::B,
                own,
                &[],
                &pairs,
                &|_| Ok(FaceShape::Plane),
                band,
            );
            if let Err(e) = &got {
                assert!(
                    e.to_string()
                        .contains("two corners of the other solid meet at one point"),
                    "{what}: the refusal names its cause: {e}"
                );
            }
            match (got, want) {
                (Ok(got), Ok(want)) => assert_eq!(got, want, "{what}"),
                (
                    Err(BooleanError::VertexReadTwice {
                        operand: Operand::B,
                        vertex,
                        reads,
                    }),
                    Err(want),
                ) if vertex == own => assert_eq!(reads, want, "{what}"),
                (got, want) => panic!("{what}: got {got:?}, want {want:?}"),
            }
        }
    }

    /// **A touching edge on two partners' boundaries reads their faces**
    /// ([`along_partners`]): an edge along x, each partner a corner with
    /// one face leaving it, one in the plane z = 0 and the other's turned
    /// `tilt` about x. Their half-planes part by the tilt, so the edge is
    /// on the solid's boundary, `On`. Flush, as plane faces bounding
    /// material on opposite sides, they are glued, coincident only to
    /// within the band, and the edge is `On`, at ε 1e-9 and, both turned
    /// 3e-7 about x, at ε 1e-12. On one side, they overlap to within the
    /// band; with a curved face they may be tangent; and a chord in band
    /// of zero is neither apart nor flush: each refuses
    /// [`super::super::BooleanDecision::PartnerFaces`]. Beside a partner on
    /// the edge's side whose cone lies in no half-space, the edge still
    /// names it to refuse on, its faces unread.
    #[test]
    fn a_touching_edge_on_two_partners_boundaries_reads_their_faces() {
        use super::super::sectors::{Reach, WedgeRead};
        // The band at ε 1e-9, whatever the run's: the in-band chord below
        // is sized to it.
        let band = Band::new(1e-9, 1e-8).unwrap();
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let he = HalfEdgeKey::from(key(1));
        let (w, x) = (VertexKey::from(key(4)), VertexKey::from(key(5)));
        let o = geom_core::Point3::new(0.0, 0.0, 0.0);
        let along = Vec3::new(1.0, 0.0, 0.0);
        let chord = |d: Vec3<f64>| Reach::Chord {
            base: o,
            far: o + d,
        };
        let sector = |end: Vec3<f64>| BoolSector {
            he: HalfEdgeKey::default(),
            start: along,
            end,
            start_reach: chord(along),
            end_reach: chord(end),
            face: crate::entity::FaceKey::default(),
            normal: OutwardNormal::from_chart(along.cross(end), true),
            arm: 1.0,
        };
        let partner_of = |partner| PairRead {
            partner,
            side: Some(SideCode::Out),
            read: Ok(WedgeRead {
                pointed: true,
                met: true,
                rows: vec![(he, SideCode::On)],
            }),
            sectors: Vec::new(),
        };
        let partner = |partner, pointed, end| PairRead {
            partner,
            side: Some(SideCode::Out),
            read: Ok(WedgeRead {
                pointed,
                met: true,
                rows: vec![(he, SideCode::On)],
            }),
            sectors: vec![sector(end)],
        };
        let own = [BoolSector {
            he,
            start: Vec3::new(0.0, -1.0, 0.0),
            start_reach: chord(Vec3::new(0.0, -1.0, 0.0)),
            end: along,
            end_reach: chord(along),
            ..sector(Vec3::new(0.0, -1.0, 0.0))
        }];
        let touch = [(he, SideCode::Out)];
        let flat = partner(w, true, Vec3::new(0.0, 1.0, 0.0));
        let turned =
            |tilt: f64, pointed| partner(x, pointed, Vec3::new(0.0, tilt.cos(), tilt.sin()));
        // The face z = 0 turned `tilt` about x, seen from above (`up`)
        // or from below: the same quarter-plane, run the other way round
        // its normal.
        let face = |partner, tilt: f64, up: bool| {
            let y = Vec3::new(0.0, tilt.cos(), tilt.sin());
            let n = Vec3::new(0.0, -tilt.sin(), tilt.cos());
            let (start, end, n) = if up { (along, y, n) } else { (y, along, -n) };
            PairRead {
                sectors: vec![BoolSector {
                    start,
                    start_reach: chord(start),
                    end,
                    end_reach: chord(end),
                    normal: OutwardNormal::from_chart(n, true),
                    ..sector(along)
                }],
                ..partner_of(partner)
            }
        };
        let glued = face(x, 0.0, false);
        let (plane, curved) = (|_| Ok(FaceShape::Plane), |_| Ok(FaceShape::Curved));
        let classes = |partners: &[PairRead<f64>], shape: ShapeOf<'_>| {
            touch_classes(&own, &touch, partners, shape, band)
        };
        let got = classes(&[flat.clone(), turned(0.5, true)], &plane);
        assert!(
            matches!(got.as_deref(), Ok([(h, SideCode::On)]) if *h == he),
            "partners apart along it: {got:?}"
        );
        let got = classes(&[flat.clone(), glued], &plane);
        assert!(
            matches!(got.as_deref(), Ok([(h, SideCode::On)]) if *h == he),
            "plane faces glued along it: {got:?}"
        );
        let fine = Band::new(1e-12, 1e-11).unwrap();
        let got = touch_classes(
            &own,
            &touch,
            &[face(w, 3e-7, true), face(x, 3e-7, false)],
            &plane,
            fine,
        );
        assert!(
            matches!(got.as_deref(), Ok([(h, SideCode::On)]) if *h == he),
            "plane faces glued along it, turned 3e-7, at ε 1e-12: {got:?}"
        );
        let got = classes(&[flat.clone(), face(x, 3e-9, false)], &plane);
        assert!(
            matches!(
                got,
                Err(TouchUndecided::Along(BooleanError::Escalated {
                    decision: super::super::BooleanDecision::PartnerFaces,
                    ..
                }))
            ),
            "plane faces parted in band along it: {got:?}"
        );
        let got = classes(&[flat.clone(), turned(0.0, true)], &plane);
        assert!(
            matches!(
                got,
                Err(TouchUndecided::Along(BooleanError::Escalated {
                    decision: super::super::BooleanDecision::PartnerFaces,
                    ..
                }))
            ),
            "plane faces flush along it, material on one side: {got:?}"
        );
        let got = classes(&[flat.clone(), turned(0.0, true)], &curved);
        assert!(
            matches!(
                got,
                Err(TouchUndecided::Along(BooleanError::Escalated {
                    decision: super::super::BooleanDecision::PartnerFaces,
                    ..
                }))
            ),
            "a curved face flush along it: {got:?}"
        );
        let got = classes(&[flat, turned(0.5, false)], &plane);
        assert!(
            matches!(got, Err(TouchUndecided::Beside(Some(p))) if p == x),
            "beside a partner in no half-space: {got:?}"
        );
    }

    /// A bound's entry; an `On` carries the Zero `on_margin` it read.
    fn entry(is_edge: bool, class: SideCode, on_margin: f64) -> Entry {
        Entry {
            he: HalfEdgeKey::default(),
            is_edge,
            class,
            tied: class == SideCode::On,
            on: (class == On).then(|| super::super::sectors::OnReading {
                predicate: "enters_material",
                decided: geom_core::Decided {
                    sign: geom_core::Sign::Zero,
                    margin: geom_core::MarginDiag::value(on_margin),
                },
            }),
            lumped: false,
        }
    }

    /// A bisector reading On between two bounds read Out is the band's
    /// Zero (reachable only at K ≤ 2, which no suite runs), and it
    /// refuses as the nonzero gate's rejection of the Zero the bisector's
    /// own side reading decided, quoting that reading's margin, on the
    /// frame's log; an edge's On there, a real one, resolves; a bisector
    /// between mixed neighbours resolves, since its code moves no edge.
    ///
    /// The Zero is read, not supplied: a bisector of arm `1` tilted off
    /// the face's plane by `asin(f·ε)` reads `f·ε` through
    /// [`pierce_side_reported`], and the refusal quotes exactly that, on
    /// the log under `enters_material`, the predicate that decided it.
    #[test]
    fn a_bisector_on_between_one_sided_bounds_refuses() {
        use super::super::sectors::{FaceShape, NO_CURVATURE, Reach, pierce_side_reported};
        use geom_brep::OutwardNormal;
        use geom_core::{ErrorTextReading, Vec3};
        let band = Band::linear(Tol::witness()).unwrap();
        let up = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        for f in [-0.7, 0.2, 0.9] {
            let rise = f * band.zero();
            let dir = Vec3::new((1.0 - rise * rise).sqrt(), 0.0, rise);
            let (side, on) = pierce_side_reported(
                dir,
                Reach::Bisector(1.0),
                up,
                FaceShape::Plane,
                NO_CURVATURE(),
                band,
            )
            .unwrap();
            assert_eq!(side.code, On, "f {f}: the bisector reads On");
            for side in [Out, In] {
                let what = format!("f {f}, {side:?}");
                let bisector = Entry {
                    on,
                    ..entry(false, On, 0.0)
                };
                let mut one_sided = [entry(true, side, 0.0), bisector, entry(true, side, 0.0)];
                let bracket = geom_core::k_stats::Bracket::open();
                let got = resolve_on_entries(&mut one_sided, band);
                let log = bracket.finish();
                let Err(BooleanError::Escalated {
                    decision: super::super::BooleanDecision::BisectorSide,
                    diag,
                }) = got
                else {
                    panic!("{what}: the bisector's Zero refuses as its own decision: {got:?}");
                };
                assert_eq!(
                    diag.predicate,
                    Some("enters_material"),
                    "{what}: logged under the predicate that decided the Zero"
                );
                assert_eq!(
                    diag.margin.rejected_sign(),
                    Some(geom_core::Sign::Zero),
                    "{what}: the nonzero gate's rejection of the Zero"
                );
                let ErrorTextReading::Value(quoted) = diag.margin.diagnostic_f64_for_error_text()
                else {
                    panic!("{what}: a point reading: {diag:?}");
                };
                assert!(
                    (quoted - rise).abs() <= 4.0 * f64::EPSILON * band.zero(),
                    "{what}: the refusal quotes the bisector's own reading {rise:e}, not \
                     {quoted:e}"
                );
                assert_eq!(
                    log.escalations.iter().map(|e| e.source).collect::<Vec<_>>(),
                    vec![diag],
                    "{what}: the rejection is the one escalation on the log"
                );
            }
        }
        let mut edge = [
            entry(true, Out, 0.0),
            entry(true, On, 0.0),
            entry(true, Out, 0.0),
        ];
        resolve_on_entries(&mut edge, band).unwrap();
        assert_eq!(edge[1].class, Out, "an edge's On resolves");
        let mut mixed = [
            entry(true, Out, 0.0),
            entry(false, On, 0.0),
            entry(true, In, 0.0),
        ];
        resolve_on_entries(&mut mixed, band).unwrap();
        assert_eq!(
            mixed[1].class, In,
            "a bisector between mixed bounds resolves"
        );
    }

    /// **A touching partner's side reads its conic bound by its bend.**
    /// The partner's corner lies in `y = 0` over the face `z = 0`: a line
    /// edge up `+z`, and a unit arc leaving along `+x`, tangent to the
    /// face, bending up. To first order the arc ties and the partner's
    /// side is unread (`Along`, which refuses); read by its bend it is
    /// `Out`, as the line is, and the partner lies outside.
    #[test]
    fn a_touching_partner_reads_its_tangent_arc_by_its_bend() {
        use super::super::sectors::{BoolSector, FaceShape, NO_CURVATURE, PierceLever, Reach};
        use geom_brep::ConicBend;
        use geom_brep::OutwardNormal;
        use geom_core::{Point3, Vec3};
        let band = Band::linear(Tol::witness()).unwrap();
        let o = Point3::new(0.0, 0.0, 0.0);
        let (x, z) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let arc = geom::Curve3::Circle {
            center: o + z,
            axis: x.cross(z),
            radius: 1.0,
            u_ref: -z,
        };
        let span = core::f64::consts::FRAC_PI_2;
        let extent = geom_brep::edge_extent_inf(&arc, 0.0, span, 2f64.sqrt());
        let bend = ConicBend::of(&arc, arc.deriv(0.0), arc.deriv2(0.0), span);
        let partner = [BoolSector {
            he: HalfEdgeKey::default(),
            start: z,
            end: x,
            start_reach: Reach::Chord {
                base: o,
                far: o + z,
            },
            end_reach: Reach::Conic { extent, bend },
            face: crate::entity::FaceKey::default(),
            normal: OutwardNormal::from_chart(Vec3::new(0.0, -1.0, 0.0), true),
            arm: 1.0,
        }];
        let datum = |shape| PierceDatum {
            normal: OutwardNormal::from_chart(z, true),
            shape,
            lever: PierceLever::Everywhere(NO_CURVATURE()),
        };
        let side = |shape| partner_side(&partner, datum(shape), false, band).unwrap();
        assert_eq!(
            side(FaceShape::Plane),
            PartnerRead::Side {
                side: SideCode::Out,
                along: false
            },
            "against a plane the arc reads its bend"
        );
        assert_eq!(
            side(FaceShape::Curved),
            PartnerRead::Along(AlongFace::Curved),
            "to first order the arc ties"
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
        let err = pierce_germ_dir(
            &s,
            Vec3::new(mid.sin(), 0.0, mid.cos()),
            OutBound::Start,
            band,
        )
        .expect_err("in band");
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

    /// **A partner's side of a face is read off its bounds**, on the
    /// face's datum (outward +z): the side every bound reads, or reads on
    /// the face, where every sector has a bound off it; none where the
    /// bounds read both sides, a sector lies on the face, or a bound on it
    /// sits beside a touch flush with the face; and a bound in band, or
    /// whose side the face's bend leaves undecided, escalates.
    #[test]
    fn a_partners_side_of_a_face_is_read_off_its_bounds() {
        use super::super::sectors::{BoolSector, FaceShape, NO_CURVATURE, PierceLever, Reach};
        use geom_brep::OutwardNormal;
        use geom_core::{Point3, Vec3};
        let band = Band::linear(Tol::witness()).unwrap();
        let z = band.zero();
        let o = Point3::new(0.0, 0.0, 0.0);
        let normal = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        let cone = |rays: &[[f64; 3]]| -> Vec<BoolSector<f64>> {
            (0..rays.len())
                .map(|k| {
                    let [a, b] =
                        [rays[k], rays[(k + 1) % rays.len()]].map(|r| Vec3::new(r[0], r[1], r[2]));
                    BoolSector {
                        he: HalfEdgeKey::default(),
                        start: b,
                        end: a,
                        start_reach: Reach::Chord {
                            base: o,
                            far: o + b,
                        },
                        end_reach: Reach::Chord {
                            base: o,
                            far: o + a,
                        },
                        face: crate::entity::FaceKey::default(),
                        normal,
                        arm: 1.0,
                    }
                })
                .collect()
        };
        let plane = PierceDatum {
            normal,
            shape: FaceShape::Plane,
            lever: PierceLever::Everywhere(NO_CURVATURE()),
        };
        // Every rotation of the sector order reads alike.
        let read = |rays: &[[f64; 3]], datum, flush| {
            let mut sectors = cone(rays);
            let first = partner_side(&sectors, datum, flush, band);
            for _ in 1..sectors.len() {
                sectors.rotate_left(1);
                let again = partner_side(&sectors, datum, flush, band);
                assert_eq!(
                    format!("{again:?}"),
                    format!("{first:?}"),
                    "a rotation of {rays:?}"
                );
            }
            first
        };
        use PartnerRead::{Across, Along, Side};
        let side = |side, along| Side { side, along };
        let standing = [[1.0, 0.0, 1.0], [0.0, 1.0, 1.0], [-1.0, -1.0, 1.0]];
        let lying = [[1.0, 0.0, 0.0], [0.0, 1.0, 1.0], [0.0, -1.0, 1.0]];
        let across = [[1.0, 0.0, 1.0], [0.0, 1.0, -1.0], [-1.0, -1.0, 1.0]];
        let curved = PierceDatum {
            shape: FaceShape::Curved,
            ..plane
        };
        for (what, rays, datum, flush, want) in [
            ("standing", &standing[..], plane, false, side(Out, false)),
            (
                "hanging",
                &[[1.0, 0.0, -1.0], [0.0, 1.0, -1.0], [-1.0, -1.0, -1.0]][..],
                plane,
                false,
                side(In, false),
            ),
            (
                "lying, a ray on the face",
                &lying[..],
                plane,
                false,
                side(Out, true),
            ),
            (
                "lying under the face",
                &[[1.0, 0.0, 0.0], [0.0, 1.0, -1.0], [0.0, -1.0, -1.0]][..],
                plane,
                false,
                side(In, true),
            ),
            ("across the face", &across[..], plane, false, Across),
            (
                "a face on the face",
                &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [-1.0, -1.0, 1.0]][..],
                plane,
                false,
                Along(AlongFace::FaceOnFace),
            ),
            (
                "standing beside a flush touch",
                &standing[..],
                plane,
                true,
                side(Out, false),
            ),
            (
                "lying beside a flush touch",
                &lying[..],
                plane,
                true,
                Along(AlongFace::Flush),
            ),
            (
                "lying on a curved face",
                &lying[..],
                curved,
                false,
                Along(AlongFace::Curved),
            ),
            (
                "across a curved face, a ray on it",
                &[[1.0, 0.0, 0.0], [0.0, 1.0, -1.0], [-1.0, -1.0, 1.0]][..],
                curved,
                false,
                Across,
            ),
        ] {
            assert_eq!(read(rays, datum, flush).unwrap(), want, "{what}");
        }
        // A bound three zero bands off the face reads in band; a hundred
        // reads off it.
        for (lift, want) in [(3.0 * z, None), (100.0 * z, Some(side(Out, false)))] {
            let rays = [[1.0, 0.0, lift], [0.0, 1.0, 1.0], [0.0, -1.0, 1.0]];
            match (read(&rays, plane, false), want) {
                (Ok(got), Some(want)) => assert_eq!(got, want, "lifted {lift}"),
                (Err(BooleanError::Escalated { decision, .. }), None) => assert_eq!(
                    decision,
                    super::super::BooleanDecision::Coincidence(
                        Coincide::SectorSide,
                        DeclarationRead::Moot
                    ),
                    "lifted {lift}"
                ),
                (got, _) => panic!("lifted {lift}: {:?}", got.map_err(|e| e.to_string())),
            }
        }
        // A bound rising at a slope whose side a unit radius of curvature
        // bends back within the band: off a plane, and undecided on the
        // bent face, where the lever charges it.
        let slope = (2.0 * z).sqrt();
        let rays = [[1.0, 0.0, slope], [0.0, 1.0, 1.0], [0.0, -1.0, 1.0]];
        assert_eq!(
            read(&rays, plane, false).unwrap(),
            side(Out, false),
            "on a plane"
        );
        let bent = PierceDatum {
            normal,
            shape: FaceShape::Curved,
            lever: PierceLever::Everywhere(1.0),
        };
        assert!(
            read(&rays, bent, false).is_err(),
            "on a face bent at a unit radius"
        );
    }

    /// **A pierce germ is the section line's ray its transition names,
    /// a graze included.** Sectors of several openings on `z = 0`, from
    /// `x̂` to the bound at `φ`, against pierced planes through a line
    /// `ℓ` of `z = 0`, each tilted both ways. The bounds' codes are read
    /// as the pierce reads them (`side_code` at the bound's reach, an On
    /// folded into the In run), and the germ is `ℓ` in every case: the
    /// line inside the sector, or the line just outside one bound by an
    /// in-plane angle beyond the band at the sector's arm, whose bound
    /// reads On at its reach: there both rays read outside the sector by
    /// membership. A graze is a transition under one of its two tilts.
    #[test]
    fn a_pierce_germ_is_the_ray_its_transition_names() {
        use super::super::sectors::{BoolSector, NO_CURVATURE, Reach, fold_on_bound, side_code};
        use geom_brep::OutwardNormal;
        use geom_core::{Point3, Vec3};
        let band = Band::linear(Tol::witness()).unwrap();
        let o = Point3::new(0.3, -0.2, 0.7);
        let z = Vec3::new(0.0, 0.0, 1.0);
        let graze = 2.0 * band.escalate();
        let flat = 0.25 * band.zero() / graze;
        for phi in [20.0_f64, 90.0, 150.0, 178.0].map(f64::to_radians) {
            let (start, end) = (
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(phi.cos(), phi.sin(), 0.0),
            );
            let chord = |d: Vec3<f64>| Reach::Chord {
                base: o,
                far: o + d,
            };
            let s = BoolSector {
                he: HalfEdgeKey::default(),
                start,
                end,
                start_reach: chord(start),
                end_reach: chord(end),
                face: crate::entity::FaceKey::default(),
                normal: OutwardNormal::from_chart(z, true),
                arm: 1.0,
            };
            // `(ψ, the tilt's cosine, a graze)`.
            let lines = [
                (0.5 * phi, 0.6, false),
                (0.1 * phi, 0.3, false),
                (0.9 * phi, 0.95, false),
                (-graze, flat, true),
                (phi + graze, flat, true),
            ];
            for (psi, cos_tilt, grazes) in lines {
                let l = Vec3::new(psi.cos(), psi.sin(), 0.0);
                let across = z.cross(l);
                let mut transitions = 0;
                for sense in [1.0, -1.0] {
                    let n_p = across * (sense * cos_tilt) + z * (1.0 - cos_tilt * cos_tilt).sqrt();
                    let code = |d| {
                        let c = side_code(
                            d,
                            chord(d),
                            OutwardNormal::from_chart(n_p, true),
                            NO_CURVATURE(),
                            band,
                        )
                        .unwrap();
                        if c == On { fold_on_bound(Out, In) } else { c }
                    };
                    let label = format!("φ {phi:.3}, ψ {psi:e}, sense {sense}");
                    // A graze the fold puts on the In side of an In
                    // bound is no transition.
                    let out = match (code(start), code(end)) {
                        (Out, In) => OutBound::Start,
                        (In, Out) => OutBound::End,
                        _ => continue,
                    };
                    transitions += 1;
                    let germ = pierce_germ_dir(&s, n_p, out, band).unwrap();
                    assert!(
                        (germ - l).norm() < 1e-12,
                        "{label}: the germ {germ:?}, not {l:?}"
                    );
                }
                let want = if grazes { 1 } else { 2 };
                assert_eq!(transitions, want, "φ {phi:.3}, ψ {psi:e}: the transitions");
            }
        }
    }

    /// **A run's germs come from the sectors either side of it, signed
    /// by the bound that run holds**, read off a real orbit: the L
    /// prism's reflex top corner `(1, 1, 1)` (five sectors, the reflex
    /// one bisected) against planes through it turned about every way.
    /// For each Out run, [`run_transitions`] names a sector whose codes
    /// are the transition it is given, and the germ that sector reads
    /// lies inside it and its opposite does not.
    #[test]
    fn a_runs_germs_are_its_transition_sectors_rays() {
        use super::super::sectors::{NO_CURVATURE, build_sectors, side_code, within};
        use geom_brep::OutwardNormal;
        use geom_core::{Point3, Vec3};
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
        let body = crate::test_support_fixtures::prism::<f64>(&profile, 1.0, tol).body;
        let corner = Point3::new(1.0, 1.0, 1.0);
        let (vertex, _) = body
            .vertices()
            .find(|(v, _)| (body.resolve_vertex_point(*v, Proven) - corner).norm() < 1e-12)
            .expect("the reflex top corner");
        let sectors = build_sectors(
            &body,
            Operand::A,
            vertex,
            &crate::sector_shape::SlitTips::default(),
            band,
        )
        .unwrap();
        let n = sectors.len();
        let (mut runs_read, mut twin_runs) = (0, 0);
        for az in (0..16).map(|k| 0.1 + f64::from(k) * std::f64::consts::TAU / 16.0) {
            for el in [-1.2_f64, -0.5, 0.2, 0.7, 1.3] {
                let n_p = Vec3::new(el.cos() * az.cos(), el.cos() * az.sin(), el.sin());
                let label = format!("az {az:.3}, el {el}");
                let outward = OutwardNormal::from_chart(n_p, true);
                let mut entries: Vec<Entry> = sectors
                    .iter()
                    .map(|s| {
                        let class =
                            side_code(s.end, s.end_reach, outward, NO_CURVATURE(), band).unwrap();
                        Entry {
                            he: s.he,
                            is_edge: s.end_edge(),
                            class,
                            tied: class == SideCode::On,
                            on: None,
                            lumped: false,
                        }
                    })
                    .collect();
                resolve_on_entries(&mut entries, band).unwrap();
                let runs = out_runs(&entries);
                twin_runs += usize::from(runs.len() == 2);
                for run in runs {
                    runs_read += 1;
                    for (t, out) in run_transitions(run, n) {
                        let (start, end) = (entries[(t + 1) % n].class, entries[t].class);
                        let want = match out {
                            OutBound::Start => (Out, In),
                            OutBound::End => (In, Out),
                        };
                        assert_eq!(
                            (start, end),
                            want,
                            "{label}, run {run:?}: sector {t}'s codes"
                        );
                        let s = &sectors[t];
                        let germ = pierce_germ_dir(s, n_p, out, band).unwrap();
                        let member = |d| within(s, d, false, DeclarationRead::Moot, band).unwrap();
                        assert!(
                            member(germ) && !member(-germ),
                            "{label}, run {run:?}: sector {t}'s germ {germ:?} ({out:?})"
                        );
                    }
                }
            }
        }
        assert!(
            runs_read >= 60 && twin_runs >= 4,
            "{runs_read} runs, {twin_runs} pairs"
        );
    }

    /// A germ on the unit circle of `z = 0` at `deg` clockwise about +z,
    /// with a unit arm.
    fn germ_at(deg: f64) -> (Vec3<f64>, f64) {
        let t = -deg.to_radians();
        (Vec3::new(t.cos(), t.sin(), 0.0), 1.0)
    }

    /// **Two pierces' germs along one ray order by their chords; a third
    /// refuses.** Pierce 0's run spans 0° to 60° clockwise, pierce 1's
    /// 300° to 0°, both from the ray at 0°. Pierce 1's germ on the ray
    /// goes last, a full turn round, so the two chords nest, and the
    /// order reports the tie, that germ first clockwise, just before
    /// pierce 0's. Red if the tie keeps insertion order
    /// (`[0, 2, 1, 3]` crosses the chords). A third pierce with a germ on
    /// that ray refuses `PinchRaysInAFace`. Red if it is ordered by
    /// insertion instead.
    #[test]
    fn germs_of_two_pierces_on_one_ray_nest_and_a_third_refuses() {
        let band = Band::linear(Tol::witness()).unwrap();
        let n = Vec3::new(0.0, 0.0, 1.0);
        let site = (Operand::A, crate::entity::FaceKey::default());
        let two = [germ_at(0.0), germ_at(60.0), germ_at(0.0), germ_at(300.0)];
        assert_eq!(
            germ_order(&two, &[0, 0, 1, 1], n, site, |_, _| Ok(None), band).ok(),
            Some((vec![0, 1, 3, 2], vec![[2, 0]])),
            "the tie nests the chords, pierce 1's germ first clockwise"
        );
        let three = [
            germ_at(0.0),
            germ_at(60.0),
            germ_at(0.0),
            germ_at(300.0),
            germ_at(0.0),
            germ_at(180.0),
        ];
        assert!(
            matches!(
                germ_order(&three, &[0, 0, 1, 1, 2, 2], n, site, |_, _| Ok(None), band),
                Err(BooleanError::PinchRaysInAFace { .. })
            ),
            "three germs on one ray refuse"
        );
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
    ///   them is read in band;
    /// - a germ decided apart from the first at its own arm but in band
    ///   at a third germ's shorter arm, which the comparison reads at, in
    ///   either insertion order; ordered where every arm is long.
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
        // A germ apart from the first at its own arm but in band at a
        // third germ's shorter one: both insertion orders refuse, and with
        // every arm long it is ordered.
        let at = |deg: f64, arm: f64| (germ_at(deg).0, arm);
        // Twice the escalation threshold at an arm of 1, a fifth of the
        // zero band at an arm of 0.01.
        let d = (2.0 * band.escalate()).to_degrees();
        for (what, germs) in [
            (
                "short third arm, after",
                [at(0.0, 1.0), at(120.0, 0.01), at(360.0 - d, 1.0)],
            ),
            (
                "short third arm, before",
                [at(0.0, 1.0), at(360.0 - d, 1.0), at(120.0, 0.01)],
            ),
        ] {
            let r = germ_order_only(
                &germs,
                &vec![0; germs.len()],
                n,
                (Operand::A, crate::entity::FaceKey::default()),
                band,
            );
            assert!(sectors_coincide(&r), "{what}: {r:?}");
        }
        assert_eq!(
            germ_order_only(
                &[at(0.0, 1.0), at(120.0, 1.0), at(360.0 - d, 1.0)],
                &[0; 3],
                n,
                (Operand::A, crate::entity::FaceKey::default()),
                band
            )
            .ok(),
            Some(vec![0, 1, 2]),
            "short third arm, every arm long"
        );
        for (what, degs, decided) in rows {
            let germs = |d: f64| degs(d).into_iter().map(germ_at).collect::<Vec<_>>();
            assert_eq!(
                germ_order_only(
                    &germs(1.0),
                    &vec![0; germs(1.0).len()],
                    n,
                    (Operand::A, crate::entity::FaceKey::default()),
                    band
                )
                .ok(),
                Some(decided),
                "{what}, 1° apart"
            );
            for d in [1e-10, 1e-12] {
                let r = germ_order_only(
                    &germs(d),
                    &vec![0; germs(d).len()],
                    n,
                    (Operand::A, crate::entity::FaceKey::default()),
                    band,
                );
                assert!(sectors_coincide(&r), "{what}, {d}° apart: {r:?}");
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
        use crate::{AtRestBody, mass_properties};
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
                &cube_body,
                Operand::A,
                super::super::VfContact {
                    vertex: corner,
                    face: near,
                },
                super::super::BooleanOp::Union,
                &super::super::GluedPairs::default(),
                &super::super::ReductionContacts::default(),
                None,
                None,
                None,
                &crate::sector_shape::SlitTips::default(),
                band,
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
                r.unwrap().body().map_or(0.0, |b| {
                    mass_properties(&b.body, tol, crate::test_support::pinned_resolution(tol))
                        .unwrap()
                        .met()
                        .expect("the pinned resolution is met")
                        .volume
                })
            };
            volumes.push([
                volume(crate::union(&body, &cube, tol)),
                volume(crate::intersect(&body, &cube, tol)),
                volume(crate::subtract(&body, &cube, tol)),
                volume(crate::union(&cube, &body, tol)),
                volume(crate::intersect(&cube, &body, tol)),
                volume(crate::subtract(&cube, &body, tol)),
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

#[cfg(test)]
#[path = "vtxfac_spline_rows.rs"]
mod vtxfac_spline_rows;

#[cfg(test)]
#[path = "vtxfac_review_rows.rs"]
mod vtxfac_review_rows;
