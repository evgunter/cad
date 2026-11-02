//! `demo_tour::teapot`'s tests.

mod wall_probes_run_here {
    //! The scene's wall pair, driven by the TEST SUITE.
    //!
    //! [`wall_probes`] is where the two unions are attempted for real;
    //! until this test existed its only caller was [`stops`], whose
    //! only caller is the tour's render walk. So the frontier the
    //! scene pins was exercised when somebody RENDERED the tour and
    //! never under `cd demos/tour && cargo test --release` — the
    //! command the spec-level local acceptance runs and the one CI's
    //! "demos tour suite" row names. A union could start succeeding,
    //! or start refusing for a different reason, with the suite green.
    //!
    //! There is nothing here to assert that the probes do not already
    //! assert: `demo_tour::walls::wall` panics on BOTH off-nominal
    //! outcomes — a different refusal, and no refusal at all. Running
    //! it IS the check, which is why the missing caller was the whole
    //! defect.
    //!
    //! What it costs is the document and one evaluation of it —
    //! [`evaluated`], the same pair `stops` opens with — and not the
    //! stop list, whose census, mass properties and tessellations are
    //! about the bodies rather than about the frontier.

    use demo_tour::teapot::*;
    use pncad::geom_core::Tol;

    #[test]
    fn every_teapot_wall_is_attempted_by_the_suite() {
        let (r, ev) = evaluated(Tol::witness());
        wall_probes(&ev, &r);
    }
}
