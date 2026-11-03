//! `demo_tour::skinned`'s tests.

mod start_frame {
    use demo_tour::scalar::frame_across;
    use demo_tour::skinned::{TUBE_ROLL, TUBE_STATIONS, cubic_spine, tube_places};
    use pncad::geom_core::Tol;
    use pncad::profile::SketchPlane;
    use pncad::sweep::sweep_places;

    /// **What every station of the `twisted_tube` loft is**, as the
    /// things that are true of it rather than as the recipe that builds
    /// it: the local +Z is the unit tangent there, the origin is the
    /// spine point exactly, the unrolled local +X is the start frame
    /// carried there by the sweep's own carry (`sweep_places`), and the
    /// authored roll turns that +X about the tangent by exactly the
    /// authored angle.
    ///
    /// This is the tour's answer to *"does a caller wanting a different
    /// roll need a second door"* — no; roll is a composition, and the
    /// composition is MEASURED here as a turn rather than restated as
    /// the expression the scene evaluates.
    ///
    /// ANTI-VACUITY: the turn is asserted to BE the authored angle, and
    /// that angle is nonzero at every station but the first, so a
    /// placement that dropped the roll reads 0 and fails.
    #[test]
    fn every_tube_station_is_the_normal_plane_turned_by_the_authored_roll() {
        let tol = Tol::witness();
        let spine = cubic_spine();
        let (lo, hi) = spine.domain();
        let (start, start_tangent) = spine.ders1(lo);
        let carried = sweep_places(
            SketchPlane::from_frame(frame_across(start, start_tangent, tol)),
            &spine,
            TUBE_STATIONS,
            tol,
        )
        .expect("the spine carries its start frame");
        let places = [
            (0.0, tube_places(&spine, 0.0, tol)),
            (TUBE_ROLL, tube_places(&spine, TUBE_ROLL, tol)),
        ];
        for i in 0..TUBE_STATIONS {
            #[allow(clippy::cast_precision_loss)]
            let u = i as f64 / (TUBE_STATIONS - 1) as f64;
            let t = (hi - lo).mul_add(u, lo);
            let (p, d) = (spine.eval(t), spine.deriv(t));
            let tangent = d.normalize();
            let carried_frame = carried[i].frame();

            for (roll, at_roll) in &places {
                let (roll, m) = (*roll, &at_roll[i]);

                // The origin is the spine point, to the bit: a station
                // whose profile plane is a few ulps off the spine is
                // not the plane AT that station.
                let o = m.origin();
                assert_eq!(
                    [o.x.to_bits(), o.y.to_bits(), o.z.to_bits()],
                    [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()],
                    "station {i} at roll {roll} must sit exactly on the spine"
                );

                // The local +Z is the unit tangent, so the profile is
                // drawn in the plane normal to the path.
                let off_z = (m.normal().get() - tangent).norm();
                assert!(
                    off_z < 1e-15,
                    "station {i} at roll {roll} must carry the unit tangent as its \
                     local +Z, off by {off_z}"
                );

                // In-plane axes: unit, normal to the tangent, and
                // right-handed with it.
                let x = m.u().get();
                assert!(
                    (x.norm() - 1.0).abs() < 1e-15 && x.dot(tangent).abs() < 1e-15,
                    "station {i} at roll {roll} must have a unit local +X in the \
                     normal plane"
                );
                assert!(
                    (m.v().get() - tangent.cross(x)).norm() < 1e-15,
                    "station {i} at roll {roll} must be right-handed"
                );

                // The turn from the carried +X to this one, measured
                // about the tangent, IS the roll the scene authored at
                // this station — zero where it authored none.
                let want = roll * u;
                let turn = x
                    .dot(carried_frame.v().get())
                    .atan2(x.dot(carried_frame.u().get()));
                assert!(
                    (turn - want).abs() < 1e-14,
                    "station {i} must be the kernel's frame turned about the tangent \
                     by the authored {want}, measured {turn} — roll is a composition, \
                     not a second door"
                );
            }

            // At roll 0 the station IS the carried frame, entry for entry
            // and exactly: the zero angle's rotation is the identity
            // matrix, so the composition is an identity product. Not
            // bitwise — an identity product carries a stored `-0.0`
            // out as `+0.0`, which names the same axis — so the claim
            // is exact equality of the numbers. A scene that placed its
            // stations by a recipe of its own rather than the sweep's
            // carry fails it.
            let unrolled = &places[0].1[i];
            #[allow(clippy::float_cmp)] // exact by construction, not a tolerance
            for (got, want) in [
                (unrolled.u().get(), carried_frame.u().get()),
                (unrolled.v().get(), carried_frame.v().get()),
                (unrolled.normal().get(), carried_frame.w().get()),
            ] {
                assert!(
                    got.x == want.x && got.y == want.y && got.z == want.z,
                    "station {i} without a roll must be the carried frame itself: got \
                     ({}, {}, {}) for the carried ({}, {}, {})",
                    got.x,
                    got.y,
                    got.z,
                    want.x,
                    want.y,
                    want.z
                );
            }
        }
    }

    /// **The tube's START frame is the world frame turned onto the start
    /// tangent by the least turn**, read against the closed form rather
    /// than against the door that builds it. With `t̂ = (a, b, c)` the
    /// unit start tangent, the least turn taking `ẑ` onto `t̂` is the
    /// rotation about `ẑ × t̂` (Rodrigues), which takes `x̂` to
    /// `(1 − a²/(1+c), −ab/(1+c), −a)` and `ŷ` to
    /// `(−ab/(1+c), 1 − b²/(1+c), −b)`.
    ///
    /// The carried-frame oracle above is built from the same start
    /// frame as `tube_places`, so it cannot see a wrong start roll; this
    /// row can. ANTI-VACUITY: the spine's start tangent is asserted off
    /// `±ẑ` by a margin, so the closed form's `x̂` is not `x̂` itself.
    #[test]
    fn the_tube_starts_from_the_world_frame_turned_onto_its_start_tangent() {
        let tol = Tol::witness();
        let spine = cubic_spine();
        let (lo, _) = spine.domain();
        let t = spine.deriv(lo).normalize();
        let (a, b, c) = (t.x, t.y, t.z);
        assert!(
            a.hypot(b) > 0.1 && c > -0.9,
            "the start tangent ({a}, {b}, {c}) leans off ±z, so the turn is not trivial"
        );
        let k = 1.0 / (1.0 + c);
        let want_u = [1.0 - a * a * k, -a * b * k, -a];
        let want_v = [-a * b * k, 1.0 - b * b * k, -b];
        let start = &tube_places(&spine, 0.0, tol)[0];
        for (axis, got, want) in [
            ("u", start.u().get(), want_u),
            ("v", start.v().get(), want_v),
        ] {
            let off = [got.x - want[0], got.y - want[1], got.z - want[2]]
                .iter()
                .fold(0.0_f64, |m, e| m.max(e.abs()));
            assert!(
                off < 1e-14,
                "the start frame's {axis} must be the least turn's, off by {off:e}: got \
                 ({}, {}, {}), closed form {want:?}",
                got.x,
                got.y,
                got.z
            );
        }
        let off_w = (start.normal().get() - t).norm();
        assert!(
            off_w < 1e-15,
            "the start frame's +Z is the start tangent, off by {off_w:e}"
        );
    }
}
