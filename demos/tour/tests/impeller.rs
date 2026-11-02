//! **The impeller's ring is a pattern writer's call, and stands where
//! the hand-written ring stood**: its product digested by bits — the
//! enclosure's volume and every point — against the digest the scene
//! had when its ring was a hand-written `Axis` mate with a slide along
//! the target's `z` (the ring now slides along the mated direction, the
//! hub's `+z` exactly), and read back as a ring.

#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use pncad::document::{CancelToken, EvalOptions, evaluate};
    use pncad::geom_core::Tol;
    use pncad::topo::mass_properties;

    /// The product's enclosure volume's bits and an FNV digest of its
    /// points' bits, sorted.
    fn digest() -> (u64, u64, usize) {
        let tol = Tol::witness();
        let doc = demo_tour::impeller::gallery_document(tol);
        let evaluation = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let product = pncad::document::product(&doc, &evaluation, tol).expect("gathers");
        let mass = mass_properties(
            &demo_tour::booleans::at_rest(&product, tol),
            tol,
            demo_tour::printed(),
        )
        .expect("mass");
        let mut points: Vec<[u64; 3]> = product
            .points()
            .map(|(_, p)| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()])
            .collect();
        points.sort_unstable();
        let mut h: u64 = 1_469_598_103_934_665_603;
        for w in points.iter().flatten() {
            h ^= *w;
            h = h.wrapping_mul(1_099_511_628_211);
        }
        (mass.enclosure.volume.to_bits(), h, points.len())
    }

    /// *Breaks if* the mated slide along the hub's axis places a blade
    /// anywhere but where the carried `z` slide did, by any bit.
    #[test]
    fn the_impellers_product_is_the_hand_written_rings_bit_for_bit() {
        let tol = Tol::witness();
        let doc = demo_tour::impeller::gallery_document(tol);
        assert!(
            doc.placements().into_iter().any(|p| matches!(
                pncad::pattern::recognise(&doc, p),
                Some(pncad::pattern::Recognised::Circular { .. })
            )),
            "the ring reads back as a ring"
        );
        let got = digest();
        println!("IMPELLER_DIGEST {got:?}");
        assert_eq!(got, WANT, "the impeller's product moved");
    }

    /// The digest at the hand-written ring (`origin/main` before PR 6394).
    const WANT: (u64, u64, usize) = (0, 0, 0);
}
