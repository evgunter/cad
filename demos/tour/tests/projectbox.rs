//! `demo_tour::projectbox`'s tests.

mod tests {
    use core::f64::consts::PI;
    use demo_tour::projectbox::*;
    use pncad::authoring::p2;
    use pncad::geom_core::Tol;
    use pncad::prelude::{ConstructedLoop, circle_split};

    /// **The bored box, its halves and the spring pass their tiers,
    /// and the above half is three outer shells.** Building the stop
    /// asserts the chain's per-op volume oracle and Seamed kind, the
    /// halves' section rings, `plane_section`'s areas against their
    /// closed forms and the spring's volume and footing; this adds what `demo_tour::run_body`
    /// asserts on the tour pass (tier 3′ on the box with its contact
    /// records, tier 3 on every other body), and the above half's
    /// pieces: the cut frees the two bored bosses' tops, so that half is
    /// the walls' piece and two caps, each bounding material, and each a
    /// solid of its own.
    #[test]
    fn the_bored_box_its_halves_and_the_spring_pass_their_tiers_and_the_above_half_is_three_solids()
    {
        use pncad::topo::ShellRole;
        let tol = Tol::witness();
        let stop = stop(tol);
        let [boxbody, spring, above, below, spring_below] = &stop.bodies[..] else {
            panic!("the box, its spring, the box's two halves and the spring again");
        };
        let contacts = boxbody
            .contacts
            .as_ref()
            .expect("the box is a boolean result");
        pncad::topo::validate_pseudomanifold_certificate(&boxbody.body, contacts, tol)
            .expect("the box: tier 3′ with its contact records");
        for part in [spring, above, below, spring_below] {
            pncad::topo::validate_geometric_certificate(&part.body, tol)
                .unwrap_or_else(|e| panic!("{}: tier 3: {e:?}", part.name));
        }
        let shells: Vec<_> = above.body.shells().map(|(k, _)| k).collect();
        let roles: Vec<ShellRole> = pncad::topo::classify_shells_of(
            &demo_tour::booleans::at_rest(&above.body, tol),
            &shells,
            tol,
        )
        .expect("the above half's shells classify")
        .into_iter()
        .map(|c| c.role)
        .collect();
        assert_eq!(roles, [ShellRole::Outer; 3], "the above half's shell roles");
        assert_eq!(above.body.solids().count(), 3, "one solid per piece");
    }

    /// The round-wire coil BODY's own volume and area, from an
    /// independent quadrature of its patches
    /// ([`demo_tour::patch_quadrature`]), against the [`ROUND_COIL_V`] and
    /// [`ROUND_COIL_A`] that [`standing_spring`] holds the kernel's
    /// certified bracket to.
    #[test]
    fn the_round_coils_own_quadrature_is_the_pinned_oracle() {
        let tol = Tol::witness();
        let round: Vec<ConstructedLoop<f64>> = vec![
            circle_split(p2(0.0, 0.0), WIRE_H, 4, 0.0, tol)
                .expect("a round wire")
                .into(),
        ];
        let body = spring::<f64>(&round, PER_TURN, tol).expect("the round-wire coil builds");
        let cap = |_o: [f64; 3], _n: [f64; 3]| PI * WIRE_H * WIRE_H;
        let about = [[0.0; 3], [SPRING_AXIS.0, SPRING_AXIS.1, 1.1]];
        let coarse = demo_tour::patch_quadrature::measure(&body, 24, &about, cap);
        let fine = demo_tour::patch_quadrature::measure(&body, 32, &about, cap);
        println!("round coil oracle: coarse {coarse:?}, fine {fine:?}");
        let rel = |a: f64, b: f64| (a - b).abs() / b.abs();
        assert!(
            rel(coarse.volume[0], fine.volume[0]) <= 1e-12
                && rel(coarse.area, fine.area) <= 1e-12
                && rel(fine.volume[0], fine.volume[1]) <= 1e-12,
            "the oracle converges and its shell closes: {coarse:?} vs {fine:?}"
        );
        assert!(
            rel(fine.volume[0], ROUND_COIL_V) <= 1e-12 && rel(fine.area, ROUND_COIL_A) <= 1e-12,
            "the round coil measures V = {:e}, A = {:e} against the pinned {ROUND_COIL_V:e}, \
             {ROUND_COIL_A:e}",
            fine.volume[0],
            fine.area
        );
    }

    /// **The coil's gap to `A·L` is discretization, not a volume
    /// error.** The stop caps the gap at [`COIL_GAP`]; a cap alone
    /// cannot tell a skinning gap from a wrong volume, so this doubles
    /// the stations a turn up to [`PER_TURN`] and asserts the gap stays
    /// negative (the slabs fall short of the tube), shrinks at least at
    /// second order (measured: 13×), and lands under an absolute cap: a
    /// volume bias of either sign larger than the cap reds it. The
    /// doubling stops at [`PER_TURN`]: past it the gap meets the
    /// spine's own 1.8e-6 shortfall, which no station count moves.
    #[test]
    fn the_coils_volume_gap_converges_at_least_at_second_order() {
        let tol = Tol::witness();
        let want = coil_volume(4.0 * WIRE_H * WIRE_H);
        let gap = |per_turn| {
            let body = spring::<f64>(&[square_wire(tol)], per_turn, tol).expect("the coil");
            let v = pncad::topo::mass_properties(
                &demo_tour::booleans::at_rest(&body, tol),
                tol,
                demo_tour::printed(),
            )
            .expect("the coil's volume")
            .enclosure
            .volume;
            (v - want) / want
        };
        let (coarse, fine) = (gap(PER_TURN / 2), gap(PER_TURN));
        assert!(
            (-COARSE_GAP..0.0).contains(&coarse)
                && (-COIL_GAP..0.0).contains(&fine)
                && fine.abs() <= coarse.abs() / 4.0,
            "the coil's A·L gap went {coarse:.3e} -> {fine:.3e} on doubling the stations"
        );
    }

    /// The cap on the gap at half [`PER_TURN`] stations a turn, measured
    /// 5.4e-5.
    const COARSE_GAP: f64 = 7e-5;
}
