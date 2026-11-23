//! Reviewer probe: mesh's doors on an all-pending cube.
#[path = "probe_support/cube.rs"]
mod cube;
#[test]
fn probe_pending_mesh() {
    let mut body = topo::Body::<f64>::new();
    let _c = cube::build_cube_pub(&mut body);
    let tol = geom_core::Tol::witness();
    let r = mesh::tessellate(&body, 0.01, tol);
    println!("DOOR tessellate: {:?}", r.as_ref().map(|_| ()));
    if let Err(e) = r { println!("DOOR tessellate display: {e}"); }
}
