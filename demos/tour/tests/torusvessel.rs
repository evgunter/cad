//! `demo_tour::torusvessel`'s tests.

mod the_sectioned_vessel {
    //! The scene's third panel, driven by the TEST SUITE as well as the
    //! render walk: [`sectioned`]'s only other caller is [`stops`], whose
    //! only caller is the tour's render walk, so without this row a mesh
    //! of the cutaway that stopped enclosing its body would show only in
    //! a frame.

    use demo_tour::torusvessel::*;
    use pncad::geom_core::Tol;

    /// **The sectioned vessel meshes to its own volume.** Its cavity's
    /// torus wall is bounded by spirics and meshes on the trimmed lane's
    /// torus row; the mesh passes `check_mesh` and encloses the hollow's
    /// measured volume to within its chordal deficit, `δ` times its area
    /// (`mesh::validate::check_volume`), which a cavity wall meshed as its
    /// complement or wound inward misses by the wall's worth.
    #[test]
    fn the_sectioned_vessel_meshes_to_its_measured_volume() {
        let tol = Tol::witness();
        let body = sectioned(tol);
        let mesh = pncad::mesh::tessellate(&body, SCENE_DELTA, tol)
            .unwrap_or_else(|e| panic!("the sectioned vessel meshes: {e:?}"));
        pncad::mesh::validate::check_mesh(&mesh)
            .unwrap_or_else(|e| panic!("the sectioned vessel's mesh: {e:?}"));
        let props = pncad::topo::mass_properties(
            &pncad::topo::finish(body, tol).expect("the sectioned vessel is a finished body"),
            tol,
            demo_tour::printed(),
        )
        .expect("the sectioned hollow's props")
        .enclosure;
        pncad::mesh::validate::check_volume(&mesh, props.volume, props.surface_area, SCENE_DELTA)
            .unwrap_or_else(|e| panic!("the mesh encloses the hollow: {e:?}"));
    }
}
