
#[test]
fn r2_probe_stranded_names_its_own_entities() {
    let up = Vec3::new(0.0, 0.0, 1.0);
    for eps_mult in [1e3, 20.0, 1e5] {
        let offset = eps_mult * Tol::witness().get().eps;
        let body = super::tests::top_split_redescribed(|p0, along, _| plane_through(p0 + up * offset, along, up));
        let stranded: Vec<_> = body
            .faces()
            .map(|(k, _)| k)
            .filter(|&k| matches!(crate::face_carrier(&body, k), Some(crate::CarrierDesc::Plane { origin, .. }) if (origin.z - 1.0).abs() > 0.5 * offset))
            .collect();
        let errors = crate::AtRestBody::validate(body.clone(), Tol::witness()).expect_err("stranded");
        eprintln!("offset {eps_mult}eps: stranded faces {stranded:?}; findings:");
        for e in &errors {
            eprintln!("   {e:?}");
        }
    }
}
