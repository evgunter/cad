//! `demo_tour::klein`'s tests.

mod loop_volume_grid {
    //! **The loop's volume over its knots, its degree and its place.**
    //! The same tube, swept with two semicircles or with eighth arcs a
    //! wall, at 9 or 17 stations, at v-degree 2 or 3, here or moved
    //! across the world origin: tier 3 decides each one's sign, the
    //! continuation meets the printed precision, and the bracket holds
    //! the body's own volume from an independent quadrature of its
    //! patches ([`demo_tour::patch_quadrature`]). The sweeps differ between
    //! cells (a coarse skin differs from A·L by up to 1 %), so each
    //! cell is held to its own body's oracle, not to A·L.

    use core::f64::consts::PI;
    use demo_tour::klein::*;
    use pncad::authoring::p2;
    use pncad::geom_core::Vec3;
    use pncad::geom_core::{RigidMotion, Tol};
    use pncad::prelude::circle_split;
    use pncad::prelude::{ConstructedLoop, circle};

    #[test]
    fn the_loops_volume_holds_its_own_quadrature_whatever_its_knots_degree_or_place() {
        let tol = Tol::witness();
        let m = meridian();
        let spine = exact_spine(&m);
        let ring = PI * (m.ro.powi(2) - m.ri.powi(2));
        // Across the origin: the loop's arcs straddle it.
        let across = Vec3::new(-1.2, 0.0, -1.8);
        for (arcs, stations, v_degree, moved) in [
            (2, 9, 2, false),
            (2, 9, 3, false),
            (4, 9, 3, false),
            (8, 9, 3, false),
            (2, 17, 3, true),
        ] {
            let cell = format!(
                "{arcs} arcs a wall, {stations} stations, v-degree {v_degree}{}",
                if moved {
                    ", moved across the origin"
                } else {
                    ""
                }
            );
            let section: Vec<ConstructedLoop<f64>> = [m.ro, m.ri]
                .into_iter()
                .map(|r| {
                    if arcs == 2 {
                        circle(p2(0.0, 0.0), r, tol).expect("a wall").into()
                    } else {
                        circle_split(p2(0.0, 0.0), r, arcs, 0.0, tol)
                            .expect("a wall")
                            .into()
                    }
                })
                .collect();
            let body = swept_loop::<f64>(&spine, &section, stations, v_degree, tol)
                .unwrap_or_else(|e| panic!("{cell}: the loop sweeps: {e:?}"));
            let body = if moved {
                let motion = RigidMotion::translation(across).expect("a translation");
                pncad::topo::transform_rigid(&body, &motion, tol)
                    .unwrap_or_else(|e| panic!("{cell}: the loop moves: {e:?}"))
            } else {
                body
            };
            let measured = pncad::topo::validate_geometric_certificate(&body, tol)
                .unwrap_or_else(|e| panic!("{cell}: tier 3 refuses: {e:?}"))
                .measure(demo_tour::printed())
                .unwrap_or_else(|e| panic!("{cell}: the volume refuses: {e:?}"));
            assert_eq!(
                measured.stop,
                pncad::topo::Stop::Met,
                "{cell}: the volume stops short of the printed precision at ε = {:e}",
                tol.eps()
            );
            let props = measured.enclosure;
            let c = if moved {
                across
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            };
            let about = [[c.x, c.y, c.z], [RLOOP + c.x, c.y, ZTOP + c.z]];
            let coarse = demo_tour::patch_quadrature::measure(&body, 16, &about, |_, _| ring);
            let oracle = demo_tour::patch_quadrature::measure(&body, 24, &about, |_, _| ring);
            let [v0, v1] = oracle.volume[..] else {
                unreachable!("two reference points")
            };
            assert!(
                (coarse.volume[0] - v0).abs() <= 1e-12 * v0 && (v0 - v1).abs() <= 1e-12 * v0,
                "{cell}: the oracle converges and its shell closes: {coarse:?} vs {oracle:?}"
            );
            println!(
                "{cell} at ε = {:e}: V = {} ± {:.1e}, oracle {v0}",
                tol.eps(),
                props.volume,
                props.volume_pad
            );
            assert!(
                (props.volume - v0).abs() <= props.volume_pad,
                "{cell}: the loop's V = {} ± {:.1e} misses its own quadrature's {v0}",
                props.volume,
                props.volume_pad
            );
        }
    }
}

mod verbs_gate_r1_probes {
    //! The bottle's two boolean walls, as a TEST rather than only as
    //! a run-the-whole-tour probe.
    //!
    //! `wall_probes` reaches these two through a full render pass, so
    //! a change to the boxes under the operand gate could only be
    //! re-measured by rebuilding and re-running the tour. That is a
    //! twenty-minute answer to a one-second question, and the walls
    //! are exactly what an operand-gate change moves.
    //!
    //! Same pins as the walls themselves — one matcher each,
    //! `wall3_pinned` and `wall4_pinned`, so the two cannot drift.
    //!
    //! **Why the operand gate's covered-pair rung cannot reach wall 4.**
    //! It is a SUBTRACT, and the revert roster it refuses at, on the
    //! loop's NURBS wall, has no covered rung at all: a decided
    //! coincidence supplies the verdict a germ arm would have, and it
    //! cannot supply a seam lane to revert through. (Wall 3, a union,
    //! passes that roster and refuses in the sweep.)

    use demo_tour::klein::*;
    use pncad::geom_core::Tol;

    #[test]
    fn the_bottles_two_boolean_walls_name_the_pairs_the_scene_claims() {
        let tol = Tol::witness();
        let Bottle {
            bulb: bulb_body,
            top,
            ..
        } = bottle::<f64>(tol);

        let joined = pncad::topo::union(&bulb_body, &top, tol)
            .expect_err("the bottle's pieces still cannot be joined");
        println!("klein wall 3: {joined:?}");
        assert!(
            wall3_pinned(&joined, &bulb_body, &top),
            "wall 3 must refuse an edge of the bulb on a crease of the loop's NURBS wall: \
             {joined:?}"
        );

        let trimmed = pncad::topo::subtract(&bulb_body, &top, tol)
            .expect_err("the self-intersection still cannot be trimmed");
        println!("klein wall 4: {trimmed:?}");
        assert!(
            wall4_pinned(&trimmed),
            "wall 4 must name a NURBS wall of the loop against a planar face of the bulb: \
             {trimmed:?}"
        );
    }
}

mod r1_mesh2_review_probes {
    //! R1 review probes for MESH-2 (PR #1421, issue 555): the
    //! flare-angle × rim-radius lottery cells, exercised through the
    //! public `mesh::tessellate` door on the real Klein bulb. The
    //! probe PRINTS each cell's outcome rather than asserting, so the
    //! identical file runs on the merge base (where the four cells
    //! should refuse) and on the head (where they should mesh).

    use core::f64::consts::PI;
    use demo_tour::klein::*;
    use pncad::geom_core::Tol;
    use pncad::sweep::Revolution;

    #[test]
    fn r1_lottery_cells_outcomes() {
        let tol = Tol::witness();
        for (alpha_deg, rim) in [
            (24.0_f64, 0.85_f64), // PR re-sweep cell
            (26.0, 0.75),         // PR re-sweep cell
            (30.0, 0.85),         // issue 555 cell
            (34.0, 1.00),         // issue 555 cell
            (30.0, 0.80),         // the shipped bottle: control, always meshed
            (28.0, 0.85),         // an "ok" neighbour in both sweeps
        ] {
            let body = bulb::<f64>(
                band::<f64>(&meridian_at(alpha_deg * PI / 180.0, RF, rim, RLOOP), tol),
                Revolution::Full,
                tol,
            );
            match pncad::mesh::tessellate(&body, 1e-2, tol) {
                Ok(m) => println!(
                    "R1PROBE cell ({alpha_deg}, {rim}): MESHED {} triangles",
                    pncad::mesh::validate::triangle_count(&m)
                ),
                Err(e) => println!("R1PROBE cell ({alpha_deg}, {rim}): REFUSED {e:?}"),
            }
        }
    }
}
