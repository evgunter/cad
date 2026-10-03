
    /// VERIFIER PROBE (PR 3999, mutant 10, the edge walk's depth cut
    /// dropping its piece): a wall C0 along `u` at the knot `u = ½`
    /// (degree 1), `z = 0.1·|x − ½| − δ − (y − 0.2)`. `∂φ/∂y = −1`
    /// everywhere, so the graph hypothesis over `y`-slices holds; inside
    /// `[0, 1] × [0.2, 1]` the locus is a V dipping `δ` out through the
    /// edge `y = 0.2`: two arms, two components, four boundary crossings,
    /// two of them `20δ` apart about the kink. The kink is no tangency
    /// (slopes ±0.1), so the walk spends ~60 pieces, far under the cap;
    /// the piece holding both near crossings is unresolved at depth 40.
    /// Unmutated: `None` at δ = 1e-14, `Some(4)` at 1e-13 and 1e-12.
    /// Under the mutant: `Some(2)` at δ = 1e-14 — two components
    /// counted as one arc's two ends.
    #[test]
    fn verifier_probe_a_kink_dip_hides_two_crossings() {
        for delta in [1e-14, 1e-13, 1e-12] {
            let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
            let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
            let g = [0.05 - delta, -delta, 0.05 - delta];
            let h = [0.2, -0.3, -0.8];
            let control = (0..9)
                .map(|i| {
                    Point3::new(
                        0.5 * f64::from(i / 3),
                        0.5 * f64::from(i % 3),
                        g[(i / 3) as usize] + h[(i % 3) as usize],
                    )
                })
                .collect();
            let wall = NurbsSurface::new(ku, kv, control, vec![1.0; 9]).unwrap();
            let boxes = NurbsBoxes::new(&wall);
            let got = boundary_zeros(&boxes, ground(), rect((0.0, 1.0), (0.2, 1.0)));
            assert!(
                got != Some(2),
                "delta {delta:e}: two components counted as one arc's two ends"
            );
        }
    }

    /// VERIFIER PROBE (mutant 10, the touch variant): the same C0 wall
    /// with `z = 0.1·|x − ½| + (y − 0.2)(0.6 − y)`; `φ` touches the edge
    /// `y = 0.2` at the kink from above. Unmutated `None` (60 pieces);
    /// under the mutant `Some(2)` (81 pieces). The smooth touch
    /// (`a_touch_inside_an_edge_resolves_no_count`) uses 61 pieces
    /// unmutated and hits the 4096 cap under the mutant, as argued.
    #[test]
    fn verifier_probe_a_kink_touch_on_an_edge_resolves_no_count() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let g = [0.05, 0.0, 0.05];
        let h = [-0.12, 0.28, -0.32];
        let control = (0..9)
            .map(|i| {
                Point3::new(
                    0.5 * f64::from(i / 3),
                    0.5 * f64::from(i % 3),
                    g[(i / 3) as usize] + h[(i % 3) as usize],
                )
            })
            .collect();
        let wall = NurbsSurface::new(ku, kv, control, vec![1.0; 9]).unwrap();
        let boxes = NurbsBoxes::new(&wall);
        assert_eq!(
            boundary_zeros(&boxes, ground(), rect((0.0, 1.0), (0.1, 1.0))),
            Some(4),
            "control: the edge below cuts the V twice, the arc twice"
        );
        assert_eq!(
            boundary_zeros(&boxes, ground(), rect((0.0, 1.0), (0.2, 1.0))),
            None,
            "the kink touch was walked past"
        );
    }
}
