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

mod spout_belly_section_probe {
    //! **The NURBS × implicit arm on the pair wall 3 meets on**: the
    //! belly's sphere against each of the spout's NURBS walls, classified
    //! through the value's door with the march door. A measurement, not
    //! a gate: wall 3 stops upstream of the section (the spline seam
    //! split), so nothing reads this pair yet. Run with
    //! `cd demos/tour && cargo test --release --test all -- --ignored
    //! spout_belly_section_probe --nocapture`.

    use demo_tour::teapot::*;
    use pncad::geom::Surface;
    use pncad::geom_brep::{SsiLane, pair_section};
    use pncad::geom_core::{Band, Tol};

    #[test]
    #[ignore = "a measurement: prints the classification of the teapot's wall-3 pair and its cost"]
    fn spout_belly_section_probe() {
        let tol = Tol::witness();
        let (r, ev) = evaluated(tol);
        let (spheres, walls) = spout_belly_carriers(&ev, &r, tol);
        let band = Band::linear(tol).expect("the run's tolerance forms a band");
        println!(
            "{} sphere carrier(s), {} spout wall(s)",
            spheres.len(),
            walls.len()
        );
        for s in &spheres {
            for w in &walls {
                let Surface::Nurbs(n) = w else { continue };
                let spans =
                    |k: &pncad::geom_core::spline::KnotVector| k.interior_knots().count() + 1;
                let (ku, kv) = (n.knots_u_f64_interim(), n.knots_v_f64_interim());
                let (p, q) = (ku.degree(), kv.degree());
                let start = std::time::Instant::now();
                let read = pair_section(s, w, None, Some(SsiLane::marched()), band);
                println!(
                    "wall of degree ({p}, {q}) over {}×{} knot spans: composite cells {}, \
                     composite degree ({}, {}); read in {:?}: {:?}",
                    spans(ku),
                    spans(kv),
                    spans(ku) * spans(kv),
                    2 * p,
                    2 * q,
                    start.elapsed(),
                    read.map(|r| match r {
                        pncad::geom_brep::SectionRead::Built(s) => format!("{:?}", s.components()),
                        pncad::geom_brep::SectionRead::Elsewhere(e) => format!("elsewhere {e:?}"),
                    })
                );
            }
        }
    }
}
