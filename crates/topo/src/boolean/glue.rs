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
//! poisoned) is left to the stage that meets it, which refuses it there:
//! two faces whose carriers do not decide one may never touch. The door
//! keeps what it read of each such pair ([`Glue::in_band`]), for a
//! corner that meets the pair to quote where that reading binds.

use std::borrow::Cow;

use geom_core::{Band, Bounds, Decide};

use super::section_table::SectionTable;
use super::{
    BooleanError, DeclarationRead, Glue, GluedPair, InBandPair, Operand, PairUnread,
    TangencyFailure, boxes, carrier_pair_decided, glued_tangency,
};
use crate::body::Body;
use crate::contact::GlueClass;
use crate::entity::FaceKey;

/// **Which operand's description a glued pair keeps** (`docs/DESIGN.md`
/// [coincidence.booleans], its one place [one-way.places]): the
/// first-written operand's, A's, a fixed rule that reads no values. An
/// n-ary operation folds its members in list order, each step's
/// accumulation its operand A, so the first-written member's
/// description is the one kept.
pub(crate) const KEPT: Operand = Operand::A;

const _: () = assert!(
    matches!(KEPT, Operand::A),
    "[coincidence.booleans] keeps the first-written operand's description, \
     and the boolean assumes it is A's: the result arena is A's clone, the \
     lump and the containment fallback cut only B's copies"
);

/// A glued pair's two sides as `(kept, other)` ([`KEPT`]): given
/// `side`'s `x` and the other operand's `y`. The boolean hands the
/// merge each glued pair kept side first (`ops::GluedSurfaces::of_result`).
pub(crate) fn kept_first<X: Copy>((side, x): (Operand, X), y: X) -> (X, X) {
    if side == KEPT { (x, y) } else { (y, x) }
}

/// `decls` with every undeclared pair the operation decides one carrier
/// added under its class, and every pair it reads in band added to
/// [`Glue::in_band`] with that reading, each in A's face order and B's
/// within each. `decls` itself comes back where neither adds anything,
/// and an owned copy where either does, an in-band pair alone included.
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
    let (mut found, mut in_band) = (Vec::new(), Vec::new());
    for (fa, fb) in overlapping_pairs(a, b, |_| true, band)? {
        if declared(fa, fb) {
            continue;
        }
        let glued = match carrier_pair_decided(a, fa, b, fb, band) {
            Ok(Ok((relation, Some(margin)))) => GlueClass::of_senses(relation)
                .map(|class| GluedPair::decided(fa, fb, class, margin)),
            // One carrier on no margin is no decision this door makes:
            // every rung that reads one carrier reads it on a margin.
            Ok(Ok((relation, None))) if GlueClass::of_senses(relation).is_some() => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "the glue door: a pair read one carrier on no margin",
                });
            }
            Ok(Ok(_)) | Err(PairUnread::OutsideInventory) => {
                tangency(a, fa, b, fb, band)?.then(|| GluedPair::new(fa, fb, GlueClass::Tangent))
            }
            Ok(Err(reading)) => {
                in_band.push(InBandPair {
                    a: fa,
                    b: fb,
                    reading,
                });
                None
            }
            // Unread, like in band: left to the stage that meets the pair (module doc).
            Err(PairUnread::Extent(_)) => None,
        };
        found.extend(glued);
    }
    if found.is_empty() && in_band.is_empty() {
        return Ok(Cow::Borrowed(decls));
    }
    let mut out = decls.clone();
    out.pairs.extend(found);
    out.in_band.extend(in_band);
    Ok(Cow::Owned(out))
}

/// **Whether the witness lane verifies `fa` and `fb` tangent** along a
/// locus ([`glued_tangency`]), asked only of a pair it has a locus for:
/// a plane and a cylinder, two cylinders, or two faces ending on one
/// circle. Along a rim the routing must also read the junction the two
/// faces make as one the zip builds, the smooth seam.
///
/// Counter-evidence, a pair outside the lane and a junction the routing
/// places outside what the zip builds are no tangency. A refusal no
/// reading of the pair settles (an in-band margin, an undecidable rim
/// identity, an unreadable face) is the operation's: a pair the lane has
/// a locus for never reaches a later stage unglued on a reading the
/// door could not take (C8).
///
/// # Errors
///
/// That refusal, typed; [`BooleanError::ClassificationInvariant`] where
/// an enumerated face has no surface.
fn tangency<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<bool, BooleanError> {
    let kind = |body: &Body<T>, f| {
        body.get_face(f)
            .and_then(|face| body.get_surface(face.surface))
            .map(geom::Surface::kind)
            .ok_or(BooleanError::ClassificationInvariant {
                what: "the glue door: a face it enumerated has no surface",
            })
    };
    let read = DeclarationRead::Moot;
    if !ruled((kind(a, fa)?, kind(b, fb)?))
        && super::rim_wedge::shared_rim(a, fa, b, fb, band)
            .map_err(|diag| BooleanError::coincidence(super::Coincide::Rim, read, diag))?
            .is_none()
    {
        return Ok(false);
    }
    match glued_tangency(a, fa, b, fb, read, band) {
        Ok(_) => Ok(true),
        Err(TangencyFailure::Refused(e)) => Err(e),
        Err(
            TangencyFailure::Contradicted { .. }
            | TangencyFailure::Unsupported
            | TangencyFailure::Unrouted(_),
        ) => Ok(false),
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

/// **The ball about the overlap of two overlapping boxes**: its centre,
/// out to its half-diagonal. The boxes are `f64` hulls the face walk
/// computed, so the ball enters as computed values.
pub(super) fn overlap_ball<T: geom_core::Real>(
    a: &bvh::Aabb,
    b: &bvh::Aabb,
) -> geom_brep::ExtentBall<T> {
    use geom_core::{Point3, Vec3};
    let lo = Vec3::new(
        a.min_x.max(b.min_x),
        a.min_y.max(b.min_y),
        a.min_z.max(b.min_z),
    );
    let hi = Vec3::new(
        a.max_x.min(b.max_x),
        a.max_y.min(b.max_y),
        a.max_z.min(b.max_z),
    );
    geom_brep::ExtentBall::new(
        Point3::origin() + ((lo + hi) * 0.5).map(T::from_computed),
        T::from_computed((hi - lo).norm() * 0.5),
    )
}

/// **The cross pairs whose face boxes overlap**: each face of `a` against
/// each face of `b` that `keep` admits, in `a`'s face order and `b`'s
/// within each — the candidates both of this door's scans read.
///
/// # Errors
///
/// A face box's own refusal; [`BooleanError::ClassificationInvariant`]
/// where the face tree returns an index past its input.
pub(super) fn overlapping_pairs<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    keep: impl Fn(&geom::Surface<T>) -> bool,
    band: Band,
) -> Result<Vec<(FaceKey, FaceKey)>, BooleanError> {
    let kept = |body: &Body<T>, f| {
        body.get_face(f)
            .and_then(|face| body.get_surface(face.surface))
            .is_some_and(&keep)
    };
    let pad = boxes::sweep_pad(band);
    let boxed = |body: &Body<T>| -> Result<Vec<(FaceKey, bvh::Aabb)>, BooleanError> {
        body.faces()
            .map(|(k, _)| k)
            .filter(|&k| kept(body, k))
            .map(|k| Ok((k, boxes::face_box(body, k, pad, band)?)))
            .collect()
    };
    pairs_among(&boxed(a)?, &boxed(b)?)
}

/// **The cross pairs among two lists of face boxes whose boxes
/// overlap**, in `a`'s order and `b`'s within each.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] where the face tree returns
/// an index past its input.
fn pairs_among(
    a: &[(FaceKey, bvh::Aabb)],
    b: &[(FaceKey, bvh::Aabb)],
) -> Result<Vec<(FaceKey, FaceKey)>, BooleanError> {
    let b_boxes: Vec<bvh::Aabb> = b.iter().map(|&(_, bx)| bx).collect();
    let tree = bvh::Bvh::build(&b_boxes);
    let mut out = Vec::new();
    for &(fa, box_a) in a {
        for i in tree.overlapping(&box_a) {
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
/// ([`geom_brep::pair_section::serves`]) at the operation's band and
/// region, and the coincidences the join reads,
/// per face pair of the two operands: each coincidence decided `Zero`
/// that the pair's section stands on one row, read from the table, at
/// the coincidence's own site. A sphere's centre on a cylinder's or a
/// torus's axis, and a cylinder or a torus coaxial with a torus, are one
/// row each; two cylinders at their bisector pose are two, the radii
/// decided equal and the axes decided meeting, the classification the
/// join's bisector lane reads. The rows in A's face order and B's within
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
pub(crate) fn section_door<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &Glue<T>,
    lane: Option<geom_brep::SsiLane<T>>,
    band: Band,
) -> Result<(SectionTable<T>, Vec<crate::Coincidence>), BooleanError> {
    let recorded = decls.verdicts != super::Verdicts::Given;
    let surface = |body: &Body<T>, f| {
        body.get_face(f)
            .and_then(|face| Some((face.surface, body.get_surface(face.surface)?.clone())))
    };
    let read = |s: &geom::Surface<T>| geom_brep::pair_section::serves(s.kind());
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
    // The operation's region (C5): the ball about the overlap of the two
    // operands' boxes, each the hull of its faces' boxes at the sweep's
    // pad, which holds every section point the operation can use; none
    // where the boxes do not overlap, so that no face pair does.
    // Each face's box once: every face's for the region, the served
    // faces' for the pairs.
    let pad = boxes::sweep_pad(band);
    let boxed = |body: &Body<T>| -> Result<Vec<(FaceKey, bvh::Aabb)>, BooleanError> {
        body.faces()
            .map(|(k, _)| Ok((k, boxes::face_box(body, k, pad, band)?)))
            .collect()
    };
    let (a_boxes, b_boxes) = (boxed(a)?, boxed(b)?);
    let hull = |boxed: &[(FaceKey, bvh::Aabb)]| {
        boxed.iter().map(|&(_, bx)| bx).reduce(|h, bx| h.hull(&bx))
    };
    let region = match (hull(&a_boxes), hull(&b_boxes)) {
        (Some(ha), Some(hb)) if ha.overlaps(&hb) => Some(overlap_ball(&ha, &hb)),
        _ => None,
    };
    let served = |body: &Body<T>, boxed: &[(FaceKey, bvh::Aabb)]| -> Vec<(FaceKey, bvh::Aabb)> {
        boxed
            .iter()
            .filter(|&&(k, _)| surface(body, k).is_some_and(|(_, s)| read(&s)))
            .copied()
            .collect()
    };
    let mut table = SectionTable::reader(lane, region, band);
    let mut rows = Vec::new();
    for (fa, fb) in pairs_among(&served(a, &a_boxes), &served(b, &b_boxes))? {
        let (Some((ka, sa)), Some((kb, sb))) = (surface(a, fa), surface(b, fb)) else {
            continue;
        };
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
    Ok((table.finish(), rows))
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
    // Each face's closure in its input, read once: the face, its edges
    // and their ends.
    let closures = |body: &Body<T>| {
        let mut out: std::collections::BTreeMap<FaceKey, Vec<Cell>> =
            std::collections::BTreeMap::new();
        for (he, h) in body.half_edges() {
            if let Some(face) = body.face_of_half_edge(he) {
                out.entry(face)
                    .or_insert_with(|| vec![Cell::Face(face)])
                    .extend([Cell::Edge(h.edge), Cell::Vertex(h.start)]);
            }
        }
        out
    };
    let (of_a, of_b) = (closures(a), closures(b));
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod region_rows {
    use geom_core::{Band, Point3, Tol};

    use crate::test_support_fixtures::brick;

    /// **The operation's region is the ball about the operands' box
    /// overlap**: two unit-high bricks overlapping in `[0.5, 1]` along
    /// `x` read a ball about the padded overlap's centre, out to its
    /// half-diagonal, which holds the overlap and is far short of either
    /// brick's own ball; bricks apart read none.
    #[test]
    fn the_region_is_the_ball_about_the_operands_box_overlap() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let plane = geom::Surface::Plane {
            origin: Point3::origin(),
            normal: geom_core::Vec3::unit_z(),
            u_ref: geom_core::Vec3::unit_x(),
        };
        let region = |b: &crate::body::Body<f64>| {
            let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
            let (table, _) = super::section_door(&a, b, &super::Glue::none(), None, band).unwrap();
            table.region_for(&plane, &plane)
        };
        let got = region(&brick::<f64>((0.5, 3.0), (0.0, 1.0), (0.0, 1.0), tol))
            .expect("the boxes overlap");
        // Each face box is padded by the sweep's pad, so the overlap is
        // `[0.5 − p, 1 + p] × [−p, 1 + p]²`.
        let pad = super::boxes::sweep_pad(band);
        let centre = Point3::new(0.75, 0.5, 0.5);
        assert!((got.center() - centre).norm() < 1e-12, "{got:?}");
        let half = |w: f64| 0.5 * (w + 2.0 * pad);
        let radius = (half(0.5).powi(2) + 2.0 * half(1.0).powi(2)).sqrt();
        assert!(
            (got.radius() - radius).abs() < 1e-12,
            "{got:?} against the padded overlap's half-diagonal {radius}"
        );
        let far = brick::<f64>((5.0, 6.0), (0.0, 1.0), (0.0, 1.0), tol);
        assert!(region(&far).is_none(), "apart: no region");
    }
}
