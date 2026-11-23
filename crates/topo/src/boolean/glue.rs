//! **Booleans glue on Zero** (D10, Booleans): every cross-operand face
//! pair whose carriers a margin decides one, or decides tangent along a
//! locus, is glued, declared or not.
//!
//! The operation's door ([`decided_glue`]) reads every pair of
//! faces of the two operands whose boxes meet. A pair the carrier
//! ladder ([`super::carrier_pair_decided`], the face-pair door's
//! reading) decides one carrier joins the operation's glued pairs
//! under the class its senses decide, with the margin it was decided
//! on: opposed senses a `Rest` contact, aligned senses a continuation. A pair of distinct carriers the
//! tangent witness lane verifies tangent along its locus joins them as
//! one `Tangent` coincidence, which claims nothing of the materials
//! (C4): the consumers read which side each lies on. From there the
//! pair takes the arm its class opens, the declaration door records
//! the decision ([`crate::coincidence`]), and every consuming site reads
//! the pair's class from that record, measuring it nowhere again. The decision is
//! kept only where the two faces meet, at a point, an edge or an area
//! ([`touched`]): the boxes offer a frame-dependent superset of the
//! pairs that do.
//!
//! A pair whose one-carrier coincidence does not decide (in band, or
//! poisoned) is left to the stage that meets it, which refuses it there
//! on its own reading: two faces whose carriers do not decide one may
//! never touch.

use std::borrow::Cow;
use std::collections::BTreeSet;

use geom_core::{Band, Bounds, Decide};

use super::section_table::SectionTable;
use super::{
    BooleanError, DeclarationRead, Glue, GluedPair, Operand, PairUnread, TangencyFailure,
    carrier_pair_decided, verify_distinct,
};
use crate::body::Body;
use crate::boxes;
use crate::entity::FaceKey;
use geom_brep::{CarrierRelation, CarrierVerdict};

/// **The first-written operand**, A: an n-ary operation folds its
/// members in list order, each step's accumulation its operand A. Where
/// the two operands' copies of one cell meet, the first-written side's
/// copy is the one the result holds — the face region the lump keeps of
/// a coincident pair ([`super::tables::surviving_copy`]), a seam's edges
/// and points (`zip`), a vertex-vertex record's point
/// (`ops::PointOwners`) — because the result arena is A's clone and the
/// lump and the containment fallback cut only B's copies. The record a
/// one-carrier pair's faces end on is the census's ([`GlueCensus`]),
/// which falls back to this side where the census does not choose.
pub(crate) const FIRST_WRITTEN: Operand = Operand::A;

/// A pair's two sides as `(first-written, other)` ([`FIRST_WRITTEN`]):
/// given `side`'s `x` and the other operand's `y`.
pub(crate) fn written_first<X: Copy>((side, x): (Operand, X), y: X) -> (X, X) {
    if side == FIRST_WRITTEN {
        (x, y)
    } else {
        (y, x)
    }
}

/// **What a one-carrier glue of `fa` on `a` and `fb` on `b` keeps and
/// moves**, a census of keys (`[coincidence.door]`; this crate's README
/// `[topo.one-record.kept]`): the side whose record is kept — the one
/// whose record alone is also worn by a face of its operand outside
/// `decided`, each operand's faces in a pair this glue leaves on one
/// record, else [`FIRST_WRITTEN`]'s ([`crate::one_record::kept_of`]) —
/// and the faces of the other side the verdict moves: its decided face
/// and that face's closure under same-record adjacency
/// ([`crate::one_record::moved_closure`]). The verdict is read over the
/// two decided faces and every face it moves
/// ([`super::carrier_pair::glue_extent`]).
#[derive(Clone, Debug)]
pub(super) struct GlueCensus {
    /// The side whose record the moved faces are left on.
    pub(super) kept: Operand,
    /// The moved side's faces the verdict moves, in its keys.
    pub(super) moved: BTreeSet<FaceKey>,
}

/// Each operand's faces in a pair the glue leaves on one record: A's,
/// then B's.
pub(super) type Decided = [BTreeSet<FaceKey>; 2];

impl GlueCensus {
    /// The census of `fa` on `a` and `fb` on `b`, `kept` where the
    /// deciding reading took it already.
    ///
    /// # Panics
    ///
    /// Where a face's loop does not walk ([`crate::one_record::moved_closure`]).
    pub(super) fn of<T: geom_core::Real>(
        (a, fa): (&Body<T>, FaceKey),
        (b, fb): (&Body<T>, FaceKey),
        decided: &Decided,
        kept: Option<Operand>,
    ) -> Self {
        let kept = kept.unwrap_or_else(|| {
            crate::one_record::kept_of(
                &[
                    (
                        Operand::A,
                        crate::one_record::worn_outside(a, fa, &decided[0]),
                    ),
                    (
                        Operand::B,
                        crate::one_record::worn_outside(b, fb, &decided[1]),
                    ),
                ],
                FIRST_WRITTEN,
            )
        });
        let moved = match kept {
            Operand::A => crate::one_record::moved_closure(b, &[fb]),
            Operand::B => crate::one_record::moved_closure(a, &[fa]),
        };
        Self { kept, moved }
    }

    /// The side whose faces move.
    pub(super) fn moving(&self) -> Operand {
        match self.kept {
            Operand::A => Operand::B,
            Operand::B => Operand::A,
        }
    }

    /// The moving side's decided face, of `fa` and `fb`.
    fn decided_face(&self, (fa, fb): (FaceKey, FaceKey)) -> FaceKey {
        match self.kept {
            Operand::A => fb,
            Operand::B => fa,
        }
    }
}

/// `decls` with every undeclared pair the operation decides one carrier
/// added under its class, in A's face order and B's within each. `decls`
/// itself comes back where none is added, and an owned copy where one
/// is.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] where the face tree returns
/// an index past its input; a face box's own refusal.
pub(crate) fn decided_glue<'d, T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &'d Glue<T>,
    band: Band,
) -> Result<Cow<'d, Glue<T>>, BooleanError> {
    if decls.verdicts == super::Verdicts::Given {
        return Ok(Cow::Borrowed(decls));
    }
    let declared = |fa, fb| decls.pairs.iter().any(|d| d.a == fa && d.b == fb);
    // Each pair is read over its two faces first: which pairs are one
    // carrier is what says which faces the glue decides.
    // Each pair's slot, in enumeration order: a tangency or nothing now,
    // a one-carrier pair once its census is read.
    let mut found: Vec<Option<GluedPair<T>>> = Vec::new();
    let mut ones = Vec::new();
    for (fa, fb) in overlapping_pairs(GLUE_PAIRS, a, b, |_| true, band)? {
        if declared(fa, fb) {
            continue;
        }
        match carrier_pair_decided(a, fa, b, fb, band) {
            Ok(Ok(CarrierVerdict::One { relation, margin })) => {
                ones.push((found.len(), fa, fb, relation, margin));
                found.push(None);
            }
            Ok(Ok(CarrierVerdict::Apart(_) | CarrierVerdict::TwoKinds))
            | Err(PairUnread::OutsideInventory) => found
                .push(distinct(a, fa, b, fb, band)?.map(|glue| GluedPair::distinct(fa, fb, glue))),
            // In band, or unread: left to the stage that meets the pair
            // (module doc).
            Ok(Err(_)) | Err(PairUnread::Extent(_)) => {}
        }
    }
    // A one-carrier pair's verdict is the one read over every face it
    // moves, under the census of the pairs this glue leaves on one record:
    // read again over those that survive until none drops.
    // The pair's own reading where its census moves the decided face
    // alone; read again over the decided faces and the closure where it
    // moves more. Apart, in band or Distinct: left to the stage that meets
    // the pair (module doc).
    let verdict = |census: &GlueCensus, (fa, fb), first| {
        let verdict =
            if census.moved.len() == 1 && census.moved.contains(&census.decided_face((fa, fb))) {
                Some(first)
            } else {
                match super::carrier_pair::carrier_pair_decided_over(
                    a,
                    fa,
                    b,
                    fb,
                    (census.moving(), &census.moved),
                    band,
                ) {
                    Ok(Ok(CarrierVerdict::One { relation, margin })) => Some((relation, margin)),
                    _ => None,
                }
            };
        verdict.filter(|&(relation, _)| relation != CarrierRelation::Distinct)
    };
    let declared_faces = decls.one_carrier_faces();
    let mut live = ones;
    let glued = loop {
        let mut decided = declared_faces.clone();
        for &(_, fa, fb, ..) in &live {
            decided[0].insert(fa);
            decided[1].insert(fb);
        }
        let read: Vec<_> = live
            .iter()
            .map(|&(slot, fa, fb, relation, margin)| {
                let census = GlueCensus::of((a, fa), (b, fb), &decided, None);
                let verdict = verdict(&census, (fa, fb), (relation, margin));
                ((slot, fa, fb, relation, margin), census.kept, verdict)
            })
            .collect();
        if read.iter().all(|(_, _, verdict)| verdict.is_some()) {
            break read;
        }
        live = read
            .into_iter()
            .filter(|(_, _, verdict)| verdict.is_some())
            .map(|(pair, ..)| pair)
            .collect();
    };
    for ((slot, fa, fb, ..), kept, verdict) in glued {
        found[slot] = verdict
            .and_then(|(relation, margin)| GluedPair::one_carrier(fa, fb, relation, margin, kept));
    }
    let found: Vec<GluedPair<T>> = found.into_iter().flatten().collect();
    if found.is_empty() {
        return Ok(Cow::Borrowed(decls));
    }
    let mut out = decls.clone();
    out.pairs.extend(found);
    Ok(Cow::Owned(out))
}

/// **What the glue door glues of `fa` and `fb` on distinct carriers**
/// ([`verify_distinct`]), asked only of a pair it has a locus for: a
/// plane and a cylinder or two cylinders, tangent along a line the
/// witness lane certifies through both faces' interiors (`Tangent`), or
/// two faces that both end on one circle. The second is a `Junction`
/// where the routing reads the π seam with the faces leaving on opposite
/// sides and each operand's own rim edge is convex: the rim's identity
/// and its wedge, the smooth seam the zip builds, with no touch claimed
/// along it. A rim that is not one is read as before: the table along
/// its turn, and the routing after a touch, a π seam whose operands'
/// edges are not both convex gluing as `Tangent`.
///
/// A verdict that the pair is not tangent, a pair outside the lane and a
/// junction the routing places outside what the zip builds are no glue
/// (`None`). A refusal no reading of the pair settles (an in-band
/// margin, an undecidable rim identity, a rim that is not one wedge, an
/// unreadable face) is the operation's: a pair the lane has a locus for
/// never reaches a later stage unglued on a reading the door could not
/// take (C8).
///
/// # Errors
///
/// That refusal, typed.
///
/// # Panics
///
/// Where `fa` or `fb`, faces the glue door enumerated, does not resolve.
fn distinct<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<Option<super::DistinctGlue<T>>, BooleanError> {
    let kind = |body: &Body<T>, f| body.proven_face(f).surface().kind();
    let read = DeclarationRead::Moot;
    if !ruled((kind(a, fa), kind(b, fb)))
        && !crate::rim_wedge::rim_exists(a, fa, b, fb, band)
            .map_err(|diag| BooleanError::coincidence(super::Coincide::Rim, read, diag))?
    {
        return Ok(None);
    }
    match verify_distinct(a, fa, b, fb, read, band) {
        Ok(glue) => Ok(Some(glue)),
        Err(TangencyFailure::Refused(e)) => Err(e),
        Err(
            TangencyFailure::NotTangent | TangencyFailure::Unsupported | TangencyFailure::Unrouted,
        ) => Ok(None),
    }
}

/// Whether two carriers' kinds are the witness lane's line pairs: a
/// plane and a cylinder, or two cylinders.
fn ruled(kinds: (geom::SurfaceKind, geom::SurfaceKind)) -> bool {
    use geom::SurfaceKind::{Cylinder, Plane};
    matches!(
        kinds,
        (Plane, Cylinder) | (Cylinder, Plane) | (Cylinder, Cylinder)
    )
}

/// **The cross pairs whose face boxes overlap**: each face of `a` against
/// each face of `b` that `keep` admits, in `a`'s face order and `b`'s
/// within each — the candidates both of this door's scans read, marked
/// under the caller's `site`.
///
/// # Errors
///
/// A face box's own refusal; [`BooleanError::ClassificationInvariant`]
/// where the face tree returns an index past its input.
pub(super) fn overlapping_pairs<T: Decide + Bounds>(
    site: geom_core::k_stats::QuerySite,
    a: &Body<T>,
    b: &Body<T>,
    keep: impl Fn(&geom::Surface<T>) -> bool,
    band: Band,
) -> Result<Vec<(FaceKey, FaceKey)>, BooleanError> {
    let pad = boxes::sweep_pad(band);
    let boxed = |body: &Body<T>| -> Result<Vec<(FaceKey, bvh::Aabb)>, BooleanError> {
        body.face_views()
            .filter(|face| keep(face.surface()))
            .map(|face| Ok((face.key(), boxes::face_box(body, face.key(), pad, band))))
            .collect()
    };
    pairs_among(site, &boxed(a)?, &boxed(b)?)
}

/// [`decided_glue`]'s face pairs.
const GLUE_PAIRS: geom_core::k_stats::QuerySite = geom_core::k_stats::QuerySite::new(
    "bool_glue_pairs",
    "the face pairs whose boxes meet, read for glue",
);
/// [`section_door`]'s face pairs the section reads.
const SECTION_PAIRS: geom_core::k_stats::QuerySite = geom_core::k_stats::QuerySite::new(
    "bool_section_pairs",
    "the face pairs whose boxes meet, read for their sections",
);
/// [`section_door`]'s plane and cone pairs.
const PLANE_CONE_PAIRS: geom_core::k_stats::QuerySite = geom_core::k_stats::QuerySite::new(
    "bool_plane_cone_pairs",
    "the plane and cone face pairs whose boxes meet",
);

/// **The cross pairs among two lists of face boxes whose boxes
/// overlap**, in `a`'s order and `b`'s within each.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] where the face tree returns
/// an index past its input.
fn pairs_among(
    site: geom_core::k_stats::QuerySite,
    a: &[(FaceKey, bvh::Aabb)],
    b: &[(FaceKey, bvh::Aabb)],
) -> Result<Vec<(FaceKey, FaceKey)>, BooleanError> {
    let b_boxes: Vec<bvh::Aabb> = b.iter().map(|&(_, bx)| bx).collect();
    let tree = bvh::Bvh::build(&b_boxes);
    let mut out = Vec::new();
    for &(fa, box_a) in a {
        for i in tree.overlapping(site, &box_a) {
            let &(fb, _) = b.get(i).ok_or(BooleanError::ClassificationInvariant {
                what: "the glue door: the face tree returned an index past its input",
            })?;
            out.push((fa, fb));
        }
    }
    Ok(out)
}

/// **The section door**: the operation's section table
/// ([`super::section_table::SectionTable`]), read over every cross pair
/// of faces whose boxes overlap and whose kinds the section value builds
/// ([`geom_brep::pair_section::builds`]): by classification, or a
/// spline wall and a partner with an implicit residual, held unread
/// ([`geom_brep::SplineImplicitPose::of`]), at the operation's band and
/// region, and the coincidences the join reads,
/// per face pair of the two operands: each coincidence decided `Zero`
/// that the pair's section stands on one row, read from the table, at
/// the coincidence's own site. A sphere's centre on a cylinder's or a
/// torus's axis, and a cylinder or a torus coaxial with a torus, are one
/// row each; two cylinders at their bisector pose are two, the radii
/// decided equal and the axes decided meeting, the classification its
/// listed nodes stand on. The rows in A's face order and B's within
/// each, none where the operation's verdicts are given; the table is
/// built either way, carrying through `lane`.
///
/// # Errors
///
/// A face box's own refusal; [`BooleanError::ClassificationInvariant`]
/// where the face tree returns an index past its input. Under given
/// verdicts too: the reduction's sweep that follows boxes every face of
/// both operands at this door's pad and band (`reduce::face_tree`), so a
/// face box refused here would refuse there, and the door only meets it
/// first.
#[allow(clippy::type_complexity)] // (table, rows) — one door's reading
pub(crate) fn section_door<T: crate::props::AtRestPolicy + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &Glue<T>,
    lane: Option<T::SsiLane>,
    band: Band,
) -> Result<(SectionTable<T>, Vec<crate::Coincidence>), BooleanError> {
    let recorded = decls.verdicts != super::Verdicts::Given;
    // Every face read here is one this door enumerated from its body.
    let surface = |body: &Body<T>, f| {
        let face = body.proven_face(f);
        (face.record().surface, face.surface().clone())
    };
    // A face that is a side of a pair read here: one whose kind pairs
    // with some kind on the other operand in a section the value builds
    // ([`face_read`]).
    let kinds = |body: &Body<T>| {
        let mut kinds: Vec<geom::SurfaceKind> = Vec::new();
        for f in body.face_views() {
            let kind = f.surface().kind();
            if !kinds.contains(&kind) {
                kinds.push(kind);
            }
        }
        kinds
    };
    let pair_read = |sa: &geom::Surface<T>, sb: &geom::Surface<T>| {
        geom_brep::pair_section::builds(sa.kind(), sb.kind())
    };
    let row = |(fa, fb), relation, site, margin| crate::Coincidence {
        cells: [
            crate::RowCell::face(Operand::A, fa),
            crate::RowCell::face(Operand::B, fb),
        ],
        more: [None; 3],
        relation,
        site,
        margin,
        discharge: crate::Discharge::of(margin),
    };
    // The operation's region (C5): where the two operands' covers meet,
    // each the hull of its faces' covers, within the band's sweep
    // tolerance ([`crate::cover::meet_within`]), which holds every section
    // point the operation can use; none where their boxes do not
    // overlap, so that no face pair does. Each face's cover once: every
    // face's for the region, the served faces' boxes for the pairs.
    let pad = boxes::sweep_pad(band);
    let covered = |body: &Body<T>| -> Vec<(FaceKey, crate::cover::FaceCovered<T>)> {
        body.face_views()
            .map(|face| (face.key(), crate::cover::face_cover(body, face.key(), band)))
            .collect()
    };
    let (a_covers, b_covers) = (covered(a), covered(b));
    let boxed = |covers: &[(FaceKey, crate::cover::FaceCovered<T>)]| -> Vec<(FaceKey, bvh::Aabb)> {
        covers
            .iter()
            .map(|(k, c)| (*k, boxes::cover_box(c, pad)))
            .collect()
    };
    let (a_boxes, b_boxes) = (boxed(&a_covers), boxed(&b_covers));
    let box_hull = |boxed: &[(FaceKey, bvh::Aabb)]| {
        boxed.iter().map(|&(_, bx)| bx).reduce(|h, bx| h.hull(&bx))
    };
    let hull = |covers: Vec<(FaceKey, crate::cover::FaceCovered<T>)>| {
        crate::cover::hull_of_covers(covers.into_iter().map(|(_, c)| c))
    };
    let region = match (box_hull(&a_boxes), box_hull(&b_boxes)) {
        (Some(ha), Some(hb)) if ha.overlaps(&hb) => hull(a_covers)
            .zip(hull(b_covers))
            .map(|(ca, cb)| crate::cover::meet_within(&ca, &cb, pad)),
        _ => None,
    };
    let served = |body: &Body<T>,
                  boxed: &[(FaceKey, bvh::Aabb)],
                  partner: &Body<T>|
     -> Vec<(FaceKey, bvh::Aabb)> {
        let partner_kinds = kinds(partner);
        boxed
            .iter()
            .filter(|&&(k, _)| face_read(surface(body, k).1.kind(), &partner_kinds))
            .copied()
            .collect()
    };
    let mut table = SectionTable::reader(lane, region, band);
    let mut rows = Vec::new();
    for (fa, fb) in pairs_among(
        SECTION_PAIRS,
        &served(a, &a_boxes, b),
        &served(b, &b_boxes, a),
    )? {
        let ((ka, sa), (kb, sb)) = (surface(a, fa), surface(b, fb));
        if !pair_read(&sa, &sb) {
            continue;
        }
        if geom_brep::SplineImplicitPose::of(&sa, &sb).is_some() {
            table.hold((ka, &sa), (kb, &sb));
            continue;
        }
        // A refusal adds no row: where the faces meet, a later reading of
        // the pair refuses (`honeone/coaxiality-of-two-revolution-carriers-is-tested-five-ways`).
        let Some(Ok(section)) = table.read((ka, &sa), (kb, &sb)) else {
            continue;
        };
        if recorded {
            for (decided, margin) in section.coincidences() {
                let (relation, site) = coincidence_site(decided);
                rows.push(row((fa, fb), relation, site, margin));
            }
        }
    }
    // Each plane×cone surface pair whose faces' boxes meet, read once
    // for the operation at the surface pair's reach
    // (`join::surface_pair_reach`), taken over every met face pair on it,
    // so no one face's extent sets the lever its pieces and siblings
    // read at. A pair whose reach does not read is not held; the join
    // reads it face by face, and refuses there.
    let plane_or_cone = |s: &geom::Surface<T>| {
        matches!(s, geom::Surface::Plane { .. } | geom::Surface::Cone { .. })
    };
    let kept = |body: &Body<T>, boxed: &[(FaceKey, bvh::Aabb)]| -> Vec<(FaceKey, bvh::Aabb)> {
        boxed
            .iter()
            .filter(|&&(k, _)| plane_or_cone(&surface(body, k).1))
            .copied()
            .collect()
    };
    let mut met: std::collections::BTreeMap<_, (geom::Surface<T>, geom::Surface<T>, Vec<_>)> =
        std::collections::BTreeMap::new();
    for (fa, fb) in pairs_among(PLANE_CONE_PAIRS, &kept(a, &a_boxes), &kept(b, &b_boxes))? {
        let ((ka, sa), (kb, sb)) = (surface(a, fa), surface(b, fb));
        if super::join::plane_and_cone(&sa, &sb).is_none() {
            continue;
        }
        met.entry((ka, kb))
            .or_insert_with(|| (sa, sb, Vec::new()))
            .2
            .push((fa, fb));
    }
    for ((ka, kb), (sa, sb, faces)) in met {
        let cone_first = matches!(sa, geom::Surface::Cone { .. });
        let (bodies, pairs): (_, Vec<(FaceKey, FaceKey)>) = if cone_first {
            ((a, b), faces)
        } else {
            ((b, a), faces.into_iter().map(|(fa, fb)| (fb, fa)).collect())
        };
        let Some((plane, cone)) = super::join::plane_and_cone(&sa, &sb) else {
            continue;
        };
        let Ok(reach) = super::join::surface_pair_reach(bodies, &pairs, cone, band) else {
            continue;
        };
        table.read_plane_cone((ka, kb), || {
            super::join::pk_section(plane, cone, &reach, band)
        });
    }
    Ok((table.finish(), rows))
}

/// **Whether the section door reads a face of kind `kind`** against an
/// operand whose faces are of `partner` kinds: where some pair it forms
/// with one of them is a section the value builds
/// ([`geom_brep::pair_section::builds`]), so no face that is a side of a
/// read pair is filtered out before its pair is asked.
fn face_read(kind: geom::SurfaceKind, partner: &[geom::SurfaceKind]) -> bool {
    partner
        .iter()
        .any(|&k| geom_brep::pair_section::builds(kind, k))
}

/// **The record a coincidence the section value decided is kept as**:
/// the relation it decides and its own site.
fn coincidence_site(decided: geom_brep::Coincident) -> (crate::Relation, crate::DecisionSite) {
    use crate::{DecisionSite as Site, Relation};
    use geom_brep::Coincident as C;
    match decided {
        C::CylinderSphere => (Relation::OnCarrier, Site::CoaxialSphere),
        C::SphereTorus => (Relation::OnCarrier, Site::SphereOnTorusAxis),
        C::CylinderTorus => (Relation::Coaxial, Site::CoaxialCylinderTorus),
        C::TorusTorus => (Relation::Coaxial, Site::CoaxialTori),
        C::CylinderRadii => (Relation::SameRadius, Site::CrossingCylinders),
        C::CylinderAxesMeet => (Relation::AxesMeet, Site::CrossingCylinders),
        C::ConeApex => unreachable!(
            "a cone's apex on its partner is recorded by a cone against a cylinder or a cone, \
             which `pair_section::serves` does not admit; the cone × torus pose it admits \
             records no coincidence"
        ),
    }
}

/// **The face-pair rows whose faces meet**: each row deciding faces
/// `fa` of A and `fb` of B one carrier, tangent or coaxial is kept only
/// where the reduction placed a cell of `fa`'s closure on a cell of
/// `fb`'s ([`super::reduce::PendingRow`], read back to the input's
/// keys), so the two faces meet, at a point, an edge or an area. Which
/// pairs a box sweep offers depends on the frame, and whether two faces
/// meet does not. A row is kept where the faces meet whether or not the
/// decision shaped the result: two faces meeting at one corner keep
/// theirs (`intent/a-face-pair-row-only-where-its-decision-shaped-the-result`).
/// Rows of any other shape are kept as they are. Returns the kept rows'
/// indices in `rows`, ascending.
pub(crate) fn touched<T: geom_core::Real>(
    rows: &[crate::Coincidence],
    pending: &[super::reduce::PendingRow],
    splits: &super::Mints,
    [a, b]: [&Body<T>; 2],
) -> Vec<usize> {
    use crate::{Cell, RowCell};
    let (of_a, of_b) = (face_closures(a), face_closures(b));
    let closure = |input: Operand, face: FaceKey| {
        let of = if input == Operand::A { &of_a } else { &of_b };
        of.get(&face).map_or(&[][..], Vec::as_slice)
    };
    let landed: Vec<[RowCell; 2]> = pending
        .iter()
        .map(|row| row.cells.map(|end| super::ops::input_cell(end, splits)))
        .collect();
    rows.iter()
        .enumerate()
        .filter(|(_, row)| {
            let Some((fa, fb)) = row.face_pair() else {
                return true;
            };
            let (on_a, on_b) = (closure(Operand::A, fa), closure(Operand::B, fb));
            let on = |cell: &RowCell, input: Operand, set: &[Cell]| {
                matches!(cell, RowCell::Input { input: i, cell } if *i == input && set.contains(cell))
            };
            landed.iter().any(|[x, y]| {
                (on(x, Operand::A, on_a) && on(y, Operand::B, on_b))
                    || (on(y, Operand::A, on_a) && on(x, Operand::B, on_b))
            })
        })
        .map(|(i, _)| i)
        .collect()
}

/// Each face's closure in `body`, read once: the face, its edges and
/// its vertices, a lone one included ([`crate::live::BoundaryMember`]).
fn face_closures<T: geom_core::Real>(
    body: &Body<T>,
) -> std::collections::BTreeMap<FaceKey, Vec<crate::Cell>> {
    use crate::Cell;
    body.face_views()
        .map(|view| {
            let mut cells = vec![Cell::Face(view.key())];
            for member in view.boundary() {
                if let crate::live::BoundaryMember::Edge { edge, .. } = member {
                    let ek = edge.key();
                    cells.push(Cell::Edge(ek));
                }
                cells.push(Cell::Vertex(member.vertex()));
            }
            (view.key(), cells)
        })
        .collect()
}

#[cfg(test)]
mod face_read_rows {
    use geom::SurfaceKind::{Approx, Cone, Cylinder, Nurbs, Plane, Sphere, Torus};

    /// **A face is read wherever a pair it forms is built**: a torus, a
    /// cylinder or a sphere against a body of spline walls (with planar
    /// caps or with cone faces) is read through the march door, as a
    /// plane is; against planes alone a torus forms no pair the value
    /// builds, and against a cone it forms the cone × torus pair. Red
    /// where the filter reads only classified pairs and excepts planes
    /// alone.
    #[test]
    fn a_face_is_read_wherever_a_pair_it_forms_is_built() {
        for kind in [Torus, Cylinder, Sphere, Plane] {
            for partner in [&[Nurbs, Plane][..], &[Approx], &[Nurbs, Cone]] {
                assert!(
                    super::face_read(kind, partner),
                    "{kind:?} against {partner:?}"
                );
            }
        }
        assert!(!super::face_read(Torus, &[Plane]));
        assert!(super::face_read(Torus, &[Cone, Plane]));
        assert!(super::face_read(Cone, &[Torus]));
        assert!(!super::face_read(Nurbs, &[Nurbs, Cone]));
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod region_rows {
    use geom_core::{Band, Point3, Tol};

    use crate::test_support_fixtures::brick;

    /// **The operation's region is where the operands' covers meet,
    /// within the band**: two unit-high bricks overlapping in `[0.5, 1]`
    /// along `x` read the ball about the overlap's centre, out to its
    /// half-diagonal widened by the sweep's pad, which holds the overlap
    /// and is far short of either brick's own ball; bricks apart read
    /// none. Red where one operand's hull stands in for the meet.
    #[test]
    fn the_region_is_the_ball_about_the_operands_box_overlap() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let plane = geom::Surface::Plane {
            frame: geom::Frame::new(
                Point3::origin(),
                geom_core::Vec3::unit_z(),
                geom_core::Vec3::unit_x(),
            ),
        };
        let region = |b: &crate::body::Body<f64>| {
            let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
            let (table, _) = super::section_door(&a, b, &super::Glue::none(), None, band).unwrap();
            table.region_for(&plane, &plane)
        };
        let got = region(&brick::<f64>((0.5, 3.0), (0.0, 1.0), (0.0, 1.0), tol))
            .expect("the boxes overlap");
        // The meet of the two hulls is the overlap `[0.5, 1] × [0, 1]²`.
        let pad = crate::boxes::sweep_pad(band);
        let centre = Point3::new(0.75, 0.5, 0.5);
        assert!((got.center() - centre).norm() < 1e-12, "{got:?}");
        let radius = (0.25f64.powi(2) + 2.0 * 0.5f64.powi(2)).sqrt() + pad;
        assert!(
            (got.radius() - radius).abs() < 1e-12,
            "{got:?} against the overlap's half-diagonal within the band, {radius}"
        );
        let far = brick::<f64>((5.0, 6.0), (0.0, 1.0), (0.0, 1.0), tol);
        assert!(region(&far).is_none(), "apart: no region");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod closure_rows {
    use geom_core::Tol;

    use crate::Cell;
    use crate::test_support_fixtures::box_with_a_lone_ring;

    /// **A face's closure holds its lone vertex**: a box's top face that
    /// bounds a one-vertex ring holds that vertex, so a reduction that
    /// lands a cell on it is a meeting of the face. Red where the closure
    /// was read off the face's half-edges, which a lone vertex has none of.
    #[test]
    fn a_faces_closure_holds_its_lone_vertex() {
        let (body, top, lone) = box_with_a_lone_ring(Tol::witness());
        let closures = super::face_closures(&body);
        let top_cells = closures.get(&top).expect("the top face's closure");
        assert!(
            top_cells.contains(&Cell::Vertex(lone)),
            "the top face's closure holds its lone vertex: {top_cells:?}"
        );
    }
}
