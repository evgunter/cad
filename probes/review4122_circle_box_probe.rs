    /// Reviewer probe (PR #4122): `circle_box` encloses the circle and is
    /// its own per-axis extent at every orientation, near-axis included.
    #[test]
    fn review4122_circle_box_at_every_orientation() {
        let mut st = 0x4122u64;
        let mut rnd = || {
            st = st.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
            ((st >> 11) as f64) / ((1u64 << 53) as f64)
        };
        let mut normals = vec![
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1e-8, 0.0, 1.0).normalize(),
            Vec3::new(1e-12, 1e-12, 1.0).normalize(),
            Vec3::new(1.0, 1e-9, -1e-9).normalize(),
            Vec3::new(1.0, 1.0, 1.0).normalize(),
        ];
        for _ in 0..400 {
            normals.push(Vec3::new(rnd() - 0.5, rnd() - 0.5, rnd() - 0.5).normalize());
        }
        let (mut worst_out, mut worst_loose) = (0.0f64, 0.0f64);
        for (k, n) in normals.into_iter().enumerate() {
            let helper = if n.x.abs() < 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
            let u = n.cross(helper).normalize();
            let v = n.cross(u);
            for (rho, cscale) in [(1e-3, 1.0), (1.0, 1.0), (1e3, 1e3), (0.7, 1e3)] {
                let c = Point3::new(cscale * (rnd() - 0.5), cscale * (rnd() - 0.5), cscale * (rnd() - 0.5));
                let b = circle_box(c, u, v, rho, 0.0);
                let (lo, hi) = ([b.min_x, b.min_y, b.min_z], [b.max_x, b.max_y, b.max_z]);
                let cc = [c.x, c.y, c.z];
                let nn = [n.x, n.y, n.z];
                for i in 0..3 {
                    let half = rho * (nn[(i + 1) % 3].powi(2) + nn[(i + 2) % 3].powi(2)).sqrt();
                    let loose = ((cc[i] - half) - lo[i]).max(hi[i] - (cc[i] + half));
                    worst_loose = worst_loose.max(loose / rho);
                    assert!(loose <= 1e-9 * rho + 1e-12 * cscale && loose >= -4.0 * f64::EPSILON * (cscale + rho), "normal {k} axis {i}: loose by {loose}");
                }
                for j in 0..4096 {
                    let t = f64::from(j) * core::f64::consts::TAU / 4096.0;
                    let p = c + u * (rho * t.cos()) + v * (rho * t.sin());
                    let pp = [p.x, p.y, p.z];
                    for i in 0..3 {
                        let out = (lo[i] - pp[i]).max(pp[i] - hi[i]);
                        worst_out = worst_out.max(out);
                        assert!(out <= 4.0 * f64::EPSILON * (cscale + rho), "normal {k} axis {i}: point out by {out}");
                    }
                }
            }
        }
        println!("circle_box: worst sample outside {worst_out:e}, worst looseness {worst_loose:e}·ρ");
    }

