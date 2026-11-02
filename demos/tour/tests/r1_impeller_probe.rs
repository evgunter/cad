//! Review probe (placing-dual-6394-r1): the impeller's evaluated product,
//! digested by bits, to compare before and after PR 6394.

#[allow(clippy::expect_used, clippy::panic, clippy::print_stderr)]
mod tests {
    use pncad::document::{CancelToken, EvalOptions, evaluate};
    use pncad::geom_core::Tol;
    use pncad::topo::mass_properties;

    #[test]
    fn r1_impeller_digest() {
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
        let mut h: u64 = 1469598103934665603;
        for p in &points {
            for w in p {
                h ^= *w;
                h = h.wrapping_mul(1099511628211);
            }
        }
        let mut maxz = f64::NEG_INFINITY;
        let mut minz = f64::INFINITY;
        for (_, p) in product.points() {
            maxz = maxz.max(p.z);
            minz = minz.min(p.z);
        }
        let line = format!(
            "R1IMPELLER faces={} edges={} verts={} points={} volume={:.17e} vbits={:#x} area={:.17e} pointhash={:#x} z=[{:.17e},{:.17e}]\n",
            product.faces().count(),
            product.edges().count(),
            product.vertices().count(),
            points.len(),
            mass.enclosure.volume,
            mass.enclosure.volume.to_bits(),
            mass.enclosure.surface_area,
            h,
            minz,
            maxz
        );
        let path = std::env::var("R1_IMPELLER_OUT").expect("out path");
        std::fs::write(path, line).expect("write");
    }
}
