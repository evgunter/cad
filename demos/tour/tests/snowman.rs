//! `demo_tour::snowman`'s tests.

mod tests {
    use demo_tour::booleans::finished;
    use demo_tour::snowman::*;
    use pncad::authoring::v3;
    use pncad::geom_core::{RigidMotion, Tol};

    /// The scene's oracles, off the render walk: every volume against
    /// its closed form, the waist selection's census, and the band's
    /// torus.
    #[test]
    fn the_snowman_meets_its_closed_forms() {
        stops(Tol::witness());
    }

    /// **A head moved off the axis IN the balls' seam plane builds.**
    /// Moved 0.05 along `x`, the head tilts the radical plane against
    /// both balls' polar axes, and the section crosses the seams the two
    /// revolves share. The germs' arcs and the Gauss–Bonnet sphere arm
    /// carry it (the tilted sphere pair). Every op passes tier 3 and
    /// meets the two-ball closed form at the moved centre distance.
    #[test]
    fn a_head_moved_off_the_axis_in_the_seam_plane_builds_to_its_closed_form() {
        let tol = Tol::witness();
        let (bottom, head) = (ball(R1, 0.0, tol), ball(R2, D, tol));
        let moved = pncad::topo::transform_rigid(
            &head,
            &RigidMotion::translation(v3(0.05, 0.0, 0.0)).expect("a finite translation"),
            tol,
        )
        .expect("a rigid pose");
        let moved = finished("the moved head", moved, tol);
        let (va, vb, vl) = (
            ball_volume(R1),
            ball_volume(R2),
            lens_volume_at(D.hypot(0.05)),
        );
        for (op, out, expected) in [
            ("∪", pncad::topo::union(&bottom, &moved, tol), va + vb - vl),
            ("∖", pncad::topo::subtract(&bottom, &moved, tol), va - vl),
            ("∩", pncad::topo::intersect(&bottom, &moved, tol), vl),
        ] {
            let label = format!("head moved 0.05 along x, bottom {op} head");
            let bb = seamed(&label, out);
            pncad::topo::validate_geometric(&bb.body, tol)
                .unwrap_or_else(|e| panic!("{label}: tier 3, {e:?}"));
            assert_volume(&label, &bb.body, expected, tol);
        }
    }

    /// **A head spun about the shared axis builds.** Spun 0.9 rad, the
    /// head's seam leaves the bottom ball's plane, and the level section
    /// passes through the head's face as a ring; its chords take the
    /// arcs their paired germs leave along. Every op passes tier 3 and
    /// meets the coaxial closed form, which the spin does not move.
    #[test]
    fn a_head_spun_about_the_axis_builds_to_its_closed_form() {
        let tol = Tol::witness();
        let (bottom, head) = (ball(R1, 0.0, tol), ball(R2, D, tol));
        let spin = RigidMotion::rotation(pncad::geom_core::UnitVec3::exact_y(), 0.9)
            .expect("a finite angle");
        let moved = pncad::topo::transform_rigid(&head, &spin, tol).expect("a rigid pose");
        let moved = finished("the moved head", moved, tol);
        let (va, vb, vl) = (ball_volume(R1), ball_volume(R2), lens_volume_at(D));
        for (op, out, expected) in [
            ("∪", pncad::topo::union(&bottom, &moved, tol), va + vb - vl),
            ("∖", pncad::topo::subtract(&bottom, &moved, tol), va - vl),
            ("∩", pncad::topo::intersect(&bottom, &moved, tol), vl),
        ] {
            let label = format!("head spun 0.9 rad, bottom {op} head");
            let bb = seamed(&label, out);
            pncad::topo::validate_geometric(&bb.body, tol)
                .unwrap_or_else(|e| panic!("{label}: tier 3, {e:?}"));
            assert_volume(&label, &bb.body, expected, tol);
        }
    }

    /// **A head moved out of the seam plane builds**, under every op.
    /// Moved 0.05 along `z`, the tilted section passes through the balls'
    /// faces without crossing either seam, as a ring of each, and the
    /// chords take the arc the pierce germs name. Each op meets the
    /// two-ball closed form at the centres' new distance.
    #[test]
    fn the_head_moved_out_of_the_seam_plane_builds_through_its_ring() {
        let tol = Tol::witness();
        let (bottom, head) = (ball(R1, 0.0, tol), ball(R2, D, tol));
        let moved = pncad::topo::transform_rigid(
            &head,
            &RigidMotion::translation(v3(0.0, 0.0, 0.05)).expect("a finite translation"),
            tol,
        )
        .expect("a rigid pose");
        let moved = finished("the moved head", moved, tol);
        let d = (D * D + 0.05 * 0.05).sqrt();
        let (va, vb, vl) = (ball_volume(R1), ball_volume(R2), lens_volume_at(d));
        for (op, out, expected) in [
            ("∪", pncad::topo::union(&bottom, &moved, tol), va + vb - vl),
            ("∖", pncad::topo::subtract(&bottom, &moved, tol), va - vl),
            ("∩", pncad::topo::intersect(&bottom, &moved, tol), vl),
        ] {
            let label = format!("head moved along z, bottom {op} head");
            let bb = seamed(&label, out);
            pncad::topo::validate_geometric(&bb.body, tol)
                .unwrap_or_else(|e| panic!("{label}: tier 3, {e:?}"));
            assert_volume(&label, &bb.body, expected, tol);
        }
    }
}
