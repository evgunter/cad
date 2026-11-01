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

/// **The Klein loop's caps hold against its walls where a wall seam
/// meets them.** The loop's section circle is split into four arcs,
/// one per wall, so each cap is a plane bounded by four arcs of one
/// circle. A cap cell holding a vertex where two arcs meet is read by
/// no presentation; the wall's departure from the cap's plane certifies
/// it, the wall's side on the shared edge read off its own restricted
/// iso net. The tour builds at `f64` only; the `Interval` twin of this
/// locus is `self_intersection_proves_injectivity`'s lens-loft row.
mod cap_corner_departure {
    use crate::common::cap_corner::{assert_holds, cap_wall_pairs};
    use demo_tour::klein::bottle;
    use pncad::geom_core::Tol;

    #[test]
    fn each_cap_of_the_loop_holds_against_its_walls_at_its_arc_vertices() {
        let tol = Tol::witness();
        let float = bottle::<f64>(tol).top;
        let pairs = cap_wall_pairs(&float);
        assert_eq!(pairs.len(), 8, "two caps against four walls: {pairs:?}");
        let receipts: Vec<(usize, u32)> = pairs
            .iter()
            .map(|&pair| assert_holds("klein loop", &float, pair))
            .collect();
        assert_eq!(
            receipts,
            [
                (1, 1),
                (1, 1),
                (1, 1),
                (1, 1),
                (4, 3),
                (4, 3),
                (4, 3),
                (4, 3)
            ],
            "each pair's (splits, deepest cell)"
        );
    }
}

/// **The loop's near-parallel walls hold against each other within the
/// default budget.** The outer and inner walls are about 0.05 apart over
/// a 2.8 body. Two of their pairs, 3v3 against 6v1 and 4v1 against 5v1,
/// face each other tilted against the axes, where the distance test's
/// boxes stay apart only for cells about as small as the gap; the
/// support along a cell's normal separates them at cells of about
/// √(gap · radius). Each pair holds under the strict question at the
/// default depth and pair budget, in at most 4096 cell pairs, each
/// pair's count printed.
mod wall_gap_support {
    use demo_tour::klein::bottle;
    use pncad::geom_core::{Band, Tol};
    use pncad::topo::FaceKey;
    use pncad::topo::clearance::{
        BoundVerdict, Budgets, ClearanceBound, DEFAULT_MAX_CELL_DEPTH, Side, Sweep, Which,
    };

    #[test]
    fn the_loops_facing_walls_hold_against_each_other_within_the_default_budget() {
        let tol = Tol::witness();
        let body = bottle::<f64>(tol).top;
        let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        let named = |n: &str| {
            *faces
                .iter()
                .find(|f| format!("{f:?}") == format!("FaceKey({n})"))
                .unwrap_or_else(|| panic!("the loop has face {n}: {faces:?}"))
        };
        let window = |f, which| {
            Side {
                body: &body,
                faces: vec![f],
            }
            .windows(which)
            .expect("a wall has a window")
        };
        let sweep = Sweep {
            bound: ClearanceBound::StrictlyPositive,
            band: Band::linear(tol).expect("the run's tolerance forms a band"),
            budgets: Budgets {
                max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
                // `editor_core::clearance::DEFAULT_MAX_CELL_PAIRS`.
                max_cell_pairs: 65_536,
            },
        };
        for (a, b) in [("3v3", "6v1"), ("4v1", "5v1")] {
            let report = sweep.run(
                &window(named(a), Which::First),
                &window(named(b), Which::Second),
                true,
                |_| Err::<(), _>("no witness asked".to_owned()),
            );
            let r = &report.receipt;
            let examined = r.discharged + r.violated + r.refused;
            println!(
                "[dirsupport] klein loop {a} against {b}: {:?} {examined} pairs {:?} deepest {}",
                report.verdict, r, report.widths.deepest
            );
            assert!(
                matches!(report.verdict, BoundVerdict::Holds),
                "{a} against {b} holds: {:?} {r:?}",
                report.verdict
            );
            assert!(
                examined <= 4096,
                "{a} against {b} holds in at most 4096 cell pairs: {examined} {r:?}"
            );
        }
    }
}
