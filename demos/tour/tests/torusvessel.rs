//! `demo_tour::torusvessel`'s tests.

mod wall_probes_run_here {
    //! The scene's wall, driven by the TEST SUITE.
    //!
    //! [`wall_probes`] is where the sectioned vessel's hollow is
    //! attempted; until this test existed its only caller was
    //! [`stops`], whose only caller is the tour's render walk. So the
    //! frontier this scene pins was exercised when somebody RENDERED
    //! the tour and never under `cd demos/tour && cargo test --release`
    //! — the command CI's "demos tour suite" row runs. The rim carrier
    //! could grow the torus arm, or the refusal could move to a
    //! different variant, with the suite green either way.
    //!
    //! There is nothing here to assert that the probe does not already
    //! assert: `demo_tour::walls::wall` panics on BOTH off-nominal
    //! outcomes — a different refusal, and no refusal at all. Running
    //! it IS the check.

    use demo_tour::torusvessel::*;
    use pncad::geom_core::Tol;

    #[test]
    fn the_sectioned_vessels_wall_is_attempted_by_the_suite() {
        wall_probes(Tol::witness());
    }
}
