//! **The whole-frame product oracle** — what a mate SEATED means,
//! measured on the body a consumer actually gathers.
//!
//! A mate's two named faces are read out of the product's own name
//! table, each as a rigid frame, and the seat is the relative map
//! between them. That map is a CONSTANT of the alignment and the two
//! part documents: each mate frame sits at a fixed offset from its
//! own face's frame in part coordinates, so no transform, no pattern
//! copy and no nesting anywhere in either chain can move it. A row
//! that measures only the normal gap and the normal dot leaves a spin
//! about the seat normal and a slide in it unmeasured; this leaves
//! nothing.
//!
//! Shared rather than copied because the control it compares against
//! is derived per suite (the same document with no placer anywhere),
//! and two copies of the comparison could drift apart while both
//! kept passing.

use editor_core::{Evaluation, ProfileDoc, StableName, product_named};
use geom_core::Tol;
use geom_core::linalg::{Affine3, Mat3, Point3};

/// **A named planar face's own frame, read out of the body the
/// product GATHERS and its own name table**: a point on it, its
/// OUTWARD normal (the surface's chart normal times the face's
/// orientation sense — the direction material is not), and the
/// in-plane reference the surface carries.
///
/// This is what a consumer sees: the gathered body and its table, no
/// solved frame read by eye.
///
/// # Panics
///
/// If the product does not gather, if `name` is absent from its table
/// or ambiguous there, or if the entity it names is not a planar
/// face — each of which is a test failure about the document, not a
/// value to hand back.
pub fn product_face_frame(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    name: &StableName,
) -> Affine3<f64> {
    let (body, names) = product_named(doc, ev, Tol::witness()).expect("the product gathers");
    let entry = names
        .lookup(name)
        .unwrap_or_else(|| panic!("{name:?} is absent from the product's table"));
    let editor_core::Entry::Unique(ent) = entry else {
        panic!("expected a unique entry, got {entry:?}")
    };
    let editor_core::EntityKey::Face(f) = ent.key else {
        panic!("expected a face, got {:?}", ent.key)
    };
    let face = body.get_face(f).expect("the face");
    match body.get_surface(face.surface).expect("the surface") {
        topo::Surface::Plane {
            origin,
            normal,
            u_ref,
        } => {
            let n = if face.sense { *normal } else { -*normal };
            // A right-handed frame on the face: u, n x u, n.
            Affine3::from_parts(
                Mat3::from_cols(*u_ref, n.cross(*u_ref), n),
                *origin - Point3::origin(),
            )
        }
        other => panic!("expected a plane, got {other:?}"),
    }
}

/// **The SEAT, as one rigid map**: the `b` face's product frame
/// expressed in the `a` face's — `F_a⁻¹ ∘ F_b`.
pub fn seat_map(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    a: &StableName,
    b: &StableName,
) -> Affine3<f64> {
    product_face_frame(doc, ev, a).inverse() * product_face_frame(doc, ev, b)
}

/// The largest absolute difference between two rigid maps, over all
/// twelve numbers.
pub fn map_gap(x: &Affine3<f64>, y: &Affine3<f64>) -> f64 {
    let cols = |m: &Affine3<f64>| [m.linear.c0, m.linear.c1, m.linear.c2, m.translation];
    let (cx, cy) = (cols(x), cols(y));
    (0..4)
        .flat_map(|i| {
            let (u, v) = (cx[i], cy[i]);
            [(u.x - v.x).abs(), (u.y - v.y).abs(), (u.z - v.z).abs()]
        })
        .fold(0.0_f64, f64::max)
}

/// The seat, measured in the product and checked three ways: the two
/// named faces are COPLANAR, their outward normals are OPPOSED (the
/// upper body stands ON the lower, not through it), and the whole
/// relative frame is the one the CONTROL — the same document with no
/// placer anywhere — puts them in.
///
/// The third check is the one a rotation or a lateral slide cannot
/// pass: it pins the spin about the seat normal and the in-plane
/// offset as well as the standoff.
///
/// # Panics
///
/// On any of the three, naming `what` and the measurement.
pub fn assert_seated(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    a: &StableName,
    b: &StableName,
    control: &Affine3<f64>,
    what: &str,
) {
    let (fa, fb) = (
        product_face_frame(doc, ev, a),
        product_face_frame(doc, ev, b),
    );
    let (na, nb) = (fa.linear.c2, fb.linear.c2);
    let gap = (fb.translation - fa.translation).dot(na).abs();
    assert!(
        gap <= Tol::witness().eps(),
        "{what}: the mated faces are {gap} apart in the product"
    );
    assert!(
        (na.dot(nb) + 1.0).abs() <= 1e-9,
        "{what}: the outward normals are not opposed (dot {}) — the blocks \
         interpenetrate rather than seat",
        na.dot(nb)
    );
    let moved = map_gap(&(fa.inverse() * fb), control);
    assert!(
        moved <= 1e-9,
        "{what}: the seat's whole relative frame moved by {moved} from the \
         control's — a spin about the seat normal or a slide in it is a \
         transform the solve did not absorb"
    );
}
