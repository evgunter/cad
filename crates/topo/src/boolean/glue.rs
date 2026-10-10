//! **Booleans glue on Zero** (D10, Booleans): every cross-operand face
//! pair whose carriers a margin decides one, or decides tangent along a
//! locus, is glued, declared or not.
//!
//! The operation's door ([`decided_declarations`]) reads every pair of
//! faces of the two operands whose boxes meet. A pair the carrier
//! ladder ([`super::carrier_pair_relation`], the face-pair door's
//! reading) decides one carrier joins the operation's declared pairs
//! under the class its senses decide: opposed senses a `Rest` contact,
//! aligned senses a continuation. A pair of distinct carriers the
//! tangent witness lane verifies tangent along its locus joins them as
//! a `Tangent` contact or, its senses aligned, a seam. From there the
//! pair takes the arm a verified declaration of that class opens, and
//! the declaration door records the decision ([`crate::coincidence`]),
//! so a declared and an undeclared scene are one body. The decision is
//! kept only where the two faces meet ([`touched`]): the boxes offer a
//! frame-dependent superset of the pairs that do.
//!
//! A pair whose coincidence does not decide (in band, or poisoned) is
//! left to the stage that meets it, which refuses it there: two faces
//! whose carriers do not decide one may never touch.

use std::borrow::Cow;

use geom_core::{Band, Bounds, Decide};

use super::{
    BooleanDeclarations, BooleanError, CarrierRelation, FacePairDeclaration, Operand, PairUnread,
    Tangency, boxes, carrier_pair_relation, verify_tangency_declaration,
};
use crate::body::Body;
use crate::contact::BooleanCoincidence;
use crate::entity::FaceKey;

/// `decls` with every undeclared pair the operation decides one carrier
/// added under its class, in A's face order and B's within each.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] where the face tree returns
/// an index past its input; a face box's own refusal.
pub(crate) fn decided_declarations<'d, T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &'d BooleanDeclarations,
    band: Band,
) -> Result<Cow<'d, BooleanDeclarations>, BooleanError> {
    if decls.verdicts == super::Verdicts::Given {
        return Ok(Cow::Borrowed(decls));
    }
    let declared = |fa, fb| {
        decls
            .coincident_faces
            .iter()
            .any(|d| d.a == fa && d.b == fb)
    };
    let mut found = Vec::new();
    for (fa, fb) in overlapping_pairs(a, b, |_| true, band)? {
        if declared(fa, fb) {
            continue;
        }
        let class = match carrier_pair_relation(a, fa, b, fb, false, band) {
            Ok(Ok(CarrierRelation::SameOpposite)) => Some(BooleanCoincidence::REST),
            Ok(Ok(CarrierRelation::SameOriented)) => Some(BooleanCoincidence::Continuation),
            Ok(Ok(CarrierRelation::Distinct)) | Err(PairUnread::OutsideInventory) => {
                tangency(a, fa, b, fb, band)?
            }
            Ok(Err(_)) | Err(PairUnread::Extent(_)) => None,
        };
        if let Some(class) = class {
            found.push(FacePairDeclaration::new(fa, fb, class));
        }
    }
    if found.is_empty() {
        return Ok(Cow::Borrowed(decls));
    }
    let mut out = decls.clone();
    out.coincident_faces.extend(found);
    Ok(Cow::Owned(out))
}

/// **The tangency the witness lane verifies between `fa` and `fb`**: a
/// `Tangent` contact where the pair verifies as one, else a seam where
/// it verifies as one, else `None`. Asked only of a pair the witness
/// lane has a locus for: a plane and a cylinder, two cylinders, or two
/// faces ending on one circle.
fn tangency<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<Option<BooleanCoincidence>, BooleanError> {
    use geom::SurfaceKind::{Cylinder, Plane};
    let kind = |body: &Body<T>, f| {
        body.get_face(f)
            .and_then(|face| body.get_surface(face.surface))
            .map(geom::Surface::kind)
            .ok_or(BooleanError::ClassificationInvariant {
                what: "the glue door: a face it enumerated has no surface",
            })
    };
    let ruled = matches!(
        (kind(a, fa)?, kind(b, fb)?),
        (Plane, Cylinder) | (Cylinder, Plane) | (Cylinder, Cylinder)
    );
    if !ruled
        && !matches!(
            super::rim_wedge::shared_rim(a, fa, b, fb, band),
            Ok(Some(_))
        )
    {
        return Ok(None);
    }
    Ok([Tangency::Contact, Tangency::Seam]
        .into_iter()
        .find(|&claim| verify_tangency_declaration(a, fa, b, fb, claim, band).is_ok())
        .map(Tangency::coincidence))
}

/// **The cross pairs whose face boxes overlap**: each face of `a` against
/// each face of `b` that `keep` admits, in `a`'s face order and `b`'s
/// within each — the candidates both of this door's scans read.
///
/// # Errors
///
/// A face box's own refusal; [`BooleanError::ClassificationInvariant`]
/// where the face tree returns an index past its input.
fn overlapping_pairs<T: Decide + Bounds>(
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
    let b_keys: Vec<FaceKey> = b.faces().map(|(k, _)| k).filter(|&k| kept(b, k)).collect();
    let b_boxes = b_keys
        .iter()
        .map(|&k| boxes::face_box(b, k, pad, band))
        .collect::<Result<Vec<_>, BooleanError>>()?;
    let tree = bvh::Bvh::build(&b_boxes);
    let mut out = Vec::new();
    for (fa, _) in a.faces() {
        if !kept(a, fa) {
            continue;
        }
        let box_a = boxes::face_box(a, fa, pad, band)?;
        for i in tree.overlapping(&box_a) {
            let fb = *b_keys.get(i).ok_or(BooleanError::ClassificationInvariant {
                what: "the glue door: the face tree returned an index past its input",
            })?;
            out.push((fa, fb));
        }
    }
    Ok(out)
}

/// **The coaxial cylinder×sphere pairs of the two operands**, one row
/// each: the sphere's centre decided on the cylinder's axis by its
/// margin (`geom_brep::cylinder_sphere_section`), the classification
/// the join's germ frame reads for the pair. In A's face order and B's
/// within each; none where the operation's verdicts are given.
///
/// # Errors
///
/// A face box's own refusal; [`BooleanError::ClassificationInvariant`]
/// where the face tree returns an index past its input.
pub(crate) fn coaxial_rows<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
    band: Band,
) -> Result<Vec<crate::Coincidence>, BooleanError> {
    if decls.verdicts == super::Verdicts::Given {
        return Ok(Vec::new());
    }
    let surface = |body: &Body<T>, f| {
        body.get_face(f)
            .and_then(|face| body.get_surface(face.surface))
            .cloned()
    };
    let curved = |s: &geom::Surface<T>| {
        matches!(
            s,
            geom::Surface::Cylinder { .. } | geom::Surface::Sphere { .. }
        )
    };
    let mut rows = Vec::new();
    for (fa, fb) in overlapping_pairs(a, b, curved, band)? {
        let (Some(sa), Some(sb)) = (surface(a, fa), surface(b, fb)) else {
            continue;
        };
        let section = match (&sa, &sb) {
            (geom::Surface::Cylinder { .. }, geom::Surface::Sphere { .. }) => {
                geom_brep::cylinder_sphere_section(&sa, &sb, band)
            }
            (geom::Surface::Sphere { .. }, geom::Surface::Cylinder { .. }) => {
                geom_brep::cylinder_sphere_section(&sb, &sa, band)
            }
            _ => continue,
        };
        if let Ok((_, margin)) = section {
            rows.push(crate::Coincidence {
                cells: [
                    crate::RowCell::face(Operand::A, fa),
                    crate::RowCell::face(Operand::B, fb),
                ],
                relation: crate::Relation::OnCarrier,
                site: crate::DecisionSite::CoaxialSphere,
                margin,
                discharge: crate::Discharge::Numeric,
            });
        }
    }
    Ok(rows)
}

/// **The face-pair rows whose glue took effect**: each row deciding
/// faces `fa` of A and `fb` of B one carrier, tangent or coaxial is
/// kept only where the reduction placed a cell of `fa`'s closure on a
/// cell of `fb`'s ([`super::reduce::PendingRow`], read back to the
/// input's keys), so the two faces meet. A pair the boxes offered whose
/// faces never meet decided nothing the result holds, and is not a
/// coincidence (D1): which such pairs a box sweep offers depends on
/// the frame, and whether two faces meet does not. Rows of any other
/// shape are kept as they are.
pub(crate) fn touched<T: geom_core::Real>(
    rows: &[crate::Coincidence],
    pending: &[super::reduce::PendingRow],
    splits: &[super::EdgeSplit],
    [a, b]: [&Body<T>; 2],
) -> Vec<crate::Coincidence> {
    use crate::{Cell, RowCell};
    let body = |input| if input == Operand::A { a } else { b };
    // `face`'s closure in its input: the face, its edges and their ends.
    let closure = |input: Operand, face: FaceKey| {
        let body = body(input);
        let mut cells = vec![Cell::Face(face)];
        for (he, h) in body.half_edges() {
            if body.face_of_half_edge(he) == Some(face) {
                cells.extend([Cell::Edge(h.edge), Cell::Vertex(h.start)]);
            }
        }
        cells
    };
    let landed: Vec<[RowCell; 2]> = pending
        .iter()
        .map(|row| row.cells.map(|end| super::ops::input_cell(end, splits)))
        .collect();
    rows.iter()
        .filter(|row| {
            let [
                RowCell::Input {
                    input: Operand::A,
                    cell: Cell::Face(fa),
                },
                RowCell::Input {
                    input: Operand::B,
                    cell: Cell::Face(fb),
                },
            ] = row.cells
            else {
                return true;
            };
            let (on_a, on_b) = (closure(Operand::A, fa), closure(Operand::B, fb));
            let on = |cell: &RowCell, input: Operand, set: &[Cell]| {
                matches!(cell, RowCell::Input { input: i, cell } if *i == input && set.contains(cell))
            };
            landed.iter().any(|[x, y]| {
                (on(x, Operand::A, &on_a) && on(y, Operand::B, &on_b))
                    || (on(y, Operand::A, &on_a) && on(x, Operand::B, &on_b))
            })
        })
        .cloned()
        .collect()
}
