//! **Which declared pairs meet along a curve.** A seam or `Tangent`
//! declaration names two faces that touch along a curve, and the
//! declaration door refuses a pair that touches along none of it
//! (`seam_locus_untouched`). A suite that declares a seam over two FACE
//! SETS (two half-walls against two half-caps) therefore names only the
//! pairs that meet: [`meeting`] keeps those, read here by the suite, not
//! by the kernel. A pair meets when a point inside a boundary edge of
//! one face lies on the other face (on its carrier and in its region),
//! sampled at five points along each edge, either way round, or when
//! the two faces share a vertex: a half-wall and the half-cap across
//! the rim meet at the rim's vertex, and the door reads that pair there.

use geom::SurfaceKind;
use geom_core::{Band, Point3, Tol};
use topo::{Body, BooleanCoincidence, BooleanDeclarations, FaceContainment, FaceKey};

/// `decls` (A faces from `x`, B faces from `y`) with every seam or
/// `Tangent` pair whose faces do not meet dropped; every other pair kept.
pub fn meeting(
    x: &Body<f64>,
    y: &Body<f64>,
    mut decls: BooleanDeclarations,
) -> BooleanDeclarations {
    decls.coincident_faces.retain(|d| {
        !matches!(d.class, BooleanCoincidence::Seam) && d.class != BooleanCoincidence::TANGENT
            || meets(x, d.a, y, d.b)
    });
    decls
}

/// Whether faces `fa` of `x` and `fb` of `y` meet along a curve.
pub fn meets(x: &Body<f64>, fa: FaceKey, y: &Body<f64>, fb: FaceKey) -> bool {
    let (va, vb) = (vertices(x, fa), vertices(y, fb));
    va.iter()
        .any(|p| vb.iter().any(|q| (*p - *q).norm() < 1e-9))
        || edge_points(x, fa).into_iter().any(|p| on_face(y, fb, p))
        || edge_points(y, fb).into_iter().any(|p| on_face(x, fa, p))
}

/// The ends of `face`'s boundary edges.
fn vertices(b: &Body<f64>, face: FaceKey) -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    for (_, e) in b.edges() {
        if ![e.he_plus, e.he_minus]
            .into_iter()
            .any(|h| b.face_of_half_edge(h) == Some(face))
        {
            continue;
        }
        if let Some(c) = b.get_curve_geom(e.curve).and_then(|g| g.certified()) {
            let (t0, t1) = c.params();
            out.push(c.carrier().eval(t0));
            out.push(c.carrier().eval(t1));
        }
    }
    out
}

/// Five points inside each boundary edge of `face`.
fn edge_points(b: &Body<f64>, face: FaceKey) -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    for (_, e) in b.edges() {
        let bounds = [e.he_plus, e.he_minus]
            .into_iter()
            .any(|h| b.face_of_half_edge(h) == Some(face));
        if !bounds {
            continue;
        }
        let Some(c) = b.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        let (t0, t1) = c.params();
        for k in 1..6 {
            out.push(c.carrier().eval(t0 + (t1 - t0) * f64::from(k) / 6.0));
        }
    }
    out
}

/// Whether `p` lies on `face`: on its carrier, and in or on its region.
fn on_face(b: &Body<f64>, face: FaceKey, p: Point3<f64>) -> bool {
    let band = Band::linear(Tol::witness()).expect("the witness band");
    let Some(s) = b.get_face(face).and_then(|f| b.get_surface(f.surface)) else {
        return false;
    };
    if geom_brep::implicit_residual(s, p).abs() > 1e-9 {
        return false;
    }
    let read = match s {
        geom::Surface::Plane { normal, .. } => topo::contfp(b, face, *normal, p, band).ok(),
        _ if s.kind() != SurfaceKind::Plane => topo::curved_face_containment(b, face, p, band)
            .ok()
            .flatten(),
        _ => None,
    };
    matches!(read, Some(r) if r != FaceContainment::Out)
}
