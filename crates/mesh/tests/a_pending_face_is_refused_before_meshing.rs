//! **The tessellator refuses a face with no surface yet, naming it**: an
//! Euler cube whose six faces are all pending is refused
//! `UnsupportedSurface` at its first face, and the text says pending.
#![allow(clippy::unwrap_used, clippy::panic)]
use crate::euler_cube_support as cube;

#[test]
fn a_pending_face_is_refused_before_meshing() {
    let mut body = topo::Body::<f64>::new();
    let c = cube::build_cube_pub(&mut body);
    let got = mesh::tessellate(&body, 0.01, geom_core::Tol::witness());
    let Err(e @ mesh::TessellateError::UnsupportedSurface { face }) = got.map(|_| ()) else {
        panic!("a pending face is refused typed")
    };
    assert_eq!(
        face, c.seed.face,
        "the first face in arena order, the seed's"
    );
    assert!(e.to_string().contains("still pending"), "{e}");
}
