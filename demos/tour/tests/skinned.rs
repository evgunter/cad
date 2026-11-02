//! `demo_tour::skinned`'s tests.

mod start_frame {
    use demo_tour::skinned::{TUBE_ROLL, TUBE_STATIONS, cubic_spine, tube_place};
    use pncad::geom_core::Tol;
    use pncad::geom_core::linalg::frame::path_start_frame;

    /// **What every station of the `twisted_tube` loft is**, as the
    /// things that are true of it rather than as the recipe that builds
    /// it: the local +Z is the unit tangent there, the origin is the
    /// spine point exactly, the unrolled local +X is the one the kernel
    /// hands out (`path_start_frame`), and the authored roll turns that
    /// +X about the tangent by exactly the authored angle.
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
        for i in 0..TUBE_STATIONS {
            #[allow(clippy::cast_precision_loss)]
            let u = i as f64 / (TUBE_STATIONS - 1) as f64;
            let t = (hi - lo).mul_add(u, lo);
            let (p, d) = (spine.eval(t), spine.deriv(t));
            let tangent = d.normalize();
            let door = path_start_frame(p, d, tol).expect("the spine's tangent fixes a frame");

            for roll in [0.0, TUBE_ROLL] {
                let m = tube_place(&spine, i, roll, tol);

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

                // The turn from the door's +X to this one, measured
                // about the tangent, IS the roll the scene authored at
                // this station — zero where it authored none.
                let want = roll * u;
                let turn = x.dot(door.v().get()).atan2(x.dot(door.u().get()));
                assert!(
                    (turn - want).abs() < 1e-14,
                    "station {i} must be the kernel's frame turned about the tangent \
                     by the authored {want}, measured {turn} — roll is a composition, \
                     not a second door"
                );
            }

            // At roll 0 the station IS the door's map, entry for entry
            // and exactly: the zero angle's rotation is the identity
            // matrix, so the composition is an identity product. Not
            // bitwise — an identity product carries a stored `-0.0`
            // out as `+0.0`, which names the same axis — so the claim
            // is exact equality of the numbers. A scene that went back
            // to a hand recipe would fail it even where the recipe
            // picked the same helper axis, because a hand recipe
            // spells the third column as a cross product and the door
            // stores the unit tangent.
            let unrolled = tube_place(&spine, i, 0.0, tol);
            #[allow(clippy::float_cmp)] // exact by construction, not a tolerance
            for (got, want) in [
                (unrolled.u().get(), door.u().get()),
                (unrolled.v().get(), door.v().get()),
                (unrolled.normal().get(), door.w().get()),
            ] {
                assert!(
                    got.x == want.x && got.y == want.y && got.z == want.z,
                    "station {i} without a roll must be the door itself: got \
                     ({}, {}, {}) for the door's ({}, {}, {})",
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
}
