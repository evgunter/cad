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
//! one `Tangent` coincidence, which claims nothing of the materials
//! (C4): the consumers read which side each lies on. From there the
//! pair takes the arm its class opens, and the declaration door records
//! the decision ([`crate::coincidence`]). The decision is
//! kept only where the two faces meet, at a point, an edge or an area
//! ([`touched`]): the boxes offer a frame-dependent superset of the
//! pairs that do.
//!
//! A pair whose one-carrier coincidence does not decide (in band, or
//! poisoned) is left to the stage that meets it, which refuses it there:
//! two faces whose carriers do not decide one may never touch.

use std::borrow::Cow;

use geom_core::{Band, Bounds, Decide};

use super::rim_wedge::Locus;
use super::{
    BooleanError, CarrierRelation, DeclarationRead, Glue, GluedPair, Operand, PairUnread, Tangency,
    boxes, carrier_pair_relation, verify_tangency,
};
use crate::body::Body;
use crate::contact::GlueClass;
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
    decls: &'d Glue,
    band: Band,
) -> Result<Cow<'d, Glue>, BooleanError> {
    if decls.verdicts == super::Verdicts::Given {
        return Ok(Cow::Borrowed(decls));
    }
    let declared = |fa, fb| decls.pairs.iter().any(|d| d.a == fa && d.b == fb);
    let mut found = Vec::new();
    for (fa, fb) in overlapping_pairs(a, b, |_| true, band)? {
        if declared(fa, fb) {
            continue;
        }
        let class = match carrier_pair_relation(a, fa, b, fb, false, band) {
            Ok(Ok(CarrierRelation::SameOpposite)) => Some(GlueClass::Rest),
            Ok(Ok(CarrierRelation::SameOriented)) => Some(GlueClass::Continuation),
            Ok(Ok(CarrierRelation::Distinct)) | Err(PairUnread::OutsideInventory) => {
                tangency(a, fa, b, fb, band)?.then_some(GlueClass::Tangent)
            }
            Ok(Err(_)) | Err(PairUnread::Extent(_)) => None,
        };
        if let Some(class) = class {
            found.push(GluedPair::new(fa, fb, class));
        }
    }
    if found.is_empty() {
        return Ok(Cow::Borrowed(decls));
    }
    let mut out = decls.clone();
    out.pairs.extend(found);
    Ok(Cow::Owned(out))
}

/// **Whether the witness lane verifies `fa` and `fb` tangent** along a
/// locus ([`verify_tangency`]), asked only of a pair it has a locus for:
/// a plane and a cylinder, two cylinders, or two faces ending on one
/// circle. Along a rim the routing must also read the junction the two
/// faces make as one the zip builds, the smooth seam
/// ([`super::route_rim`]). Any refusal of either is no tangency, an
/// escalation included (filed:
/// `work/contacthold/the-glue-doors-tangency-swallows-its-escalations.md`).
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
    if !ruled((kind(a, fa)?, kind(b, fb)?))
        && !matches!(
            super::rim_wedge::shared_rim(a, fa, b, fb, band),
            Ok(Some(_))
        )
    {
        return Ok(false);
    }
    OLD_VERDICT.with(|c| c.set("None"));
    let mut why = String::new();
    let verdict = match verify_tangency(a, fa, b, fb, DeclarationRead::Moot, band) {
        Ok(Tangency {
            locus: Locus::Rim(rim),
            ..
        }) => {
            let r = super::route_rim(a, fa, b, fb, rim, band);
            if let Err(e) = &r {
                why = format!("route_rim:{e:?}");
            }
            r.is_ok()
        }
        Ok(_) => true,
        Err(e) => {
            why = match e {
                super::TangencyFailure::Contradicted { margin, .. } => {
                    format!("contradicted:{:?}", margin.predicate)
                }
                super::TangencyFailure::Unsupported => "unsupported".into(),
                super::TangencyFailure::Refused(e) => format!("refused:{e:?}"),
            };
            false
        }
    };
    if std::env::var_os("CAD_GLUE_CMP").is_some() {
        let old = OLD_VERDICT.with(std::cell::Cell::get);
        let differ = (old != "None") != verdict;
        eprintln!(
            "GLUECMP\t{}\t{}\t{:?}/{:?}\told={old}\tnew={verdict}\t{}",
            if differ { "DIFF" } else { "same" },
            std::thread::current().name().unwrap_or("?"),
            kind(a, fa)?,
            kind(b, fb)?,
            why.chars().take(300).collect::<String>()
        );
    }
    Ok(verdict)
}

thread_local! {
    /// REVIEW PROBE (PR 4831): the old door's verdict on the pair being read.
    pub(crate) static OLD_VERDICT: std::cell::Cell<&'static str> = const { std::cell::Cell::new("None") };
}

/// Whether two carriers' kinds are the witness lane's line pairs: a
/// plane and a cylinder, or two cylinders.
pub(crate) fn ruled(kinds: (geom::SurfaceKind, geom::SurfaceKind)) -> bool {
    use geom::SurfaceKind::{Cylinder, Plane};
    matches!(
        kinds,
        (Plane, Cylinder) | (Cylinder, Plane) | (Cylinder, Cylinder)
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
    decls: &Glue,
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
                discharge: crate::Discharge::of(margin),
            });
        }
    }
    Ok(rows)
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
/// Rows of any other shape are kept as they are.
pub(crate) fn touched<T: geom_core::Real>(
    rows: &[crate::Coincidence],
    pending: &[super::reduce::PendingRow],
    splits: &[super::EdgeSplit],
    [a, b]: [&Body<T>; 2],
) -> Vec<crate::Coincidence> {
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
                (on(x, Operand::A, on_a) && on(y, Operand::B, on_b))
                    || (on(y, Operand::A, on_a) && on(x, Operand::B, on_b))
            })
        })
        .cloned()
        .collect()
}
