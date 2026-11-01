
/// Reviewer 6257: the tangent slanted rims' union and `A ∖ B` read
/// point by point against the operands' membership, beside the touch.
#[test]
fn rev6257_slanted_rims_tangent_point_in_solid() {
    for tilt in [0.0f64, 0.3] {
        let a = cut(&rod(1.0, 1.0), tilt, true);
        let b = cut(&rod(2.0, 0.0), tilt, false);
        let k = tilt.tan();
        let in_a = |p: Point3<f64>| {
            ((p.x - 1.0).powi(2) + p.y.powi(2) < 1.0) && p.z < p.y * k && p.z.abs() < H
        };
        let in_b = |p: Point3<f64>| (p.x.powi(2) + p.y.powi(2) < 4.0) && p.z > p.y * k && p.z.abs() < H;
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        for (o, name) in [(BooleanOp::Union, "A∪B"), (BooleanOp::Subtract, "A−B")] {
            let Ok(BooleanResult::Body(res)) = op(o, &a, &b) else {
                panic!("tilt {tilt} {name}: refused")
            };
            let mut samples = Vec::new();
            for &dx in &[-1e-2, -1e-3, -1e-5, 1e-5, 1e-3, 1e-2] {
                for &dy in &[-0.3, -1e-3, 0.0, 1e-3, 0.3] {
                    for &dz in &[-0.7, -1e-3, 1e-3, 0.7] {
                        samples.push(Point3::new(2.0 + dx, dy, dy * k + dz));
                    }
                }
            }
            for &x in &[-1.5, -0.5, 0.5, 1.0, 1.5, 1.9] {
                for &y in &[-1.2, -0.5, 0.0, 0.5, 1.2] {
                    for &dz in &[-1.0, -0.2, 0.2, 1.0] {
                        samples.push(Point3::new(x, y, y * k + dz));
                    }
                }
            }
            let (mut wrong, mut undecided) = (Vec::new(), 0);
            for p in samples {
                let want = match name {
                    "A∪B" => in_a(p) || in_b(p),
                    _ => in_a(p) && !in_b(p),
                };
                match topo::point_in_solid(&res.body, p, band, tol()) {
                    Ok(topo::SolidContainment::In) if want => {}
                    Ok(topo::SolidContainment::Out) if !want => {}
                    Ok(topo::SolidContainment::In | topo::SolidContainment::Out) => wrong.push(p),
                    _ => undecided += 1,
                }
            }
            eprintln!("tilt {tilt} {name}: wrong {} undecided {undecided}", wrong.len());
            assert!(wrong.is_empty(), "tilt {tilt} {name}: wrong at {wrong:?}");
        }
    }
}
