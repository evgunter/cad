
/// Review probes for PR 6385 (reviewer lane; not part of the PR).
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod rev6385_probes {
    use super::*;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A unit cylinder about `x`, its sector's bisector at `p = (0,0,1)`
    /// along `d` (45° off the axis), and a plane through `p` holding `d`
    /// whose normal is `cos ψ·z + sin ψ·w` (`w = z × d`).
    fn bisector_against_plane(cos_psi: f64, sin_psi: f64, arm: f64) -> (Tie<f64>, f64) {
        let (c, s) = (core::f64::consts::FRAC_1_SQRT_2, core::f64::consts::FRAC_1_SQRT_2);
        let p = Point3::new(0.0, 0.0, 1.0);
        let (z, d) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(c, s, 0.0));
        let w = z.cross(d);
        let m = z * cos_psi + w * sin_psi;
        let cyl = geom::Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
            u_ref: z,
        };
        let face = FaceDatum::of_plane(geom_brep::ReferenceNormal::of_face_outward(
            OutwardNormal::from_chart(m.normalize(), true),
        ));
        let bisector = Reach::bisector(arm, d, z, FaceBend::at(&cyl, p));
        let FirstOrder::Tied(tie) = read(d, bisector, face, band()).unwrap() else {
            panic!("the bisector lies in the plane at first order");
        };
        // The bisector's normal section: the cylinder cut by the plane
        // through `p` spanned by `d` and `z`, at `u` along `d`:
        // `p + d·u + z·v`, `(u·s)² + (1 + v)² = 1`.
        let v = (1.0 - (arm * s).powi(2)).sqrt() - 1.0;
        let section_offset = (d * arm + z * v).dot(m.normalize());
        (tie, section_offset)
    }

    /// The plane holds the bisector's normal section exactly (`ψ = π/2`):
    /// the parting is zero and the bound rides; the witness, the foot of
    /// `p + d·l` on the cylinder, stands far off the plane.
    #[test]
    fn rev6385_a_plane_holding_a_bisectors_normal_section_reads_its_foot_off_it() {
        let (tie, section) = bisector_against_plane(0.0, 1.0, 0.3);
        eprintln!("psi=pi/2: section {section:e}");
        assert!(section.abs() < 1e-15, "the normal section lies in the plane");
        assert_eq!(tie.descend(band()).unwrap(), Descent::Rides);
    }

    /// A parting in the gap (the normal section stands `~5e-9` below
    /// the plane at the arm) whose foot witness is decided ABOVE: the
    /// head reads `Leaves(Above)`, a side the bound's normal section
    /// never stands on within the arm.
    #[test]
    fn rev6385_a_gap_parting_leaves_on_its_foots_side_not_its_sections() {
        let arm: f64 = 0.1;
        // parting = κ·cos ψ·l²/2 with κ = s²/R = 1/2: 5e-9 at l = 0.1.
        let cos_psi: f64 = 5e-9 / (0.25 * arm * arm);
        let sin_psi = -(1.0 - cos_psi * cos_psi).sqrt();
        let (tie, section) = bisector_against_plane(cos_psi, sin_psi, arm);
        let got = tie.descend(band());
        eprintln!("MAIN gap: section {section:e}, descent {got:?}");
        assert!(section < 0.0, "the section stands below: {section:e}");
        
    }

    /// A decided parting below whose foot witness is decided above: the
    /// head refuses (`ReadFault::Witness`) a bisector whose normal
    /// section stands decidedly below all along the arm.
    #[test]
    fn rev6385_a_decided_parting_refuses_on_its_foot() {
        let (tie, section) = bisector_against_plane(0.1, -(1.0f64 - 0.01).sqrt(), 1.0);
        let got = tie.descend(band());
        eprintln!("MAIN decided: section {section:e}, descent {got:?}");
        assert!(section < -1e-3, "the section stands below: {section:e}");
        
    }

    /// A unit ball tie, the line in its tangent plane, at the origin and
    /// translated: at `ε = 1e-12` the translated one refuses on the
    /// witness's rounding where the one at the origin leaves.
    #[test]
    fn rev6385_a_ball_tie_moved_off_the_origin_refuses_at_a_tight_band() {
        let b = Band::new(1e-12, 1e-11).unwrap();
        let z = Vec3::new(0.0, 0.0, 1.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        for shift in [0.0, 10.0, 100.0, 1000.0] {
            let o = Point3::new(shift, 0.0, 0.0);
            let ball = geom::Surface::Sphere {
                center: o - z,
                radius: 1.0,
                axis: z,
                u_ref: x,
            };
            let face = FaceDatum::of_face(
                OutwardNormal::from_chart(z, true),
                FaceBend::at(&ball, o),
                1.0,
            );
            let line = Reach::Chord {
                base: o,
                far: o + x * 0.1,
            };
            let FirstOrder::Tied(tie) = read(x, line, face, b).unwrap() else {
                panic!("tangent line ties");
            };
            eprintln!("shift {shift}: {:?}", tie.descend(b));
        }
    }

    /// The same for a bisector of a cylinder against a plane holding it
    /// (the plane-height arm's rounding is the coordinates' alone).
    #[test]
    fn rev6385_a_bisector_tie_moved_off_the_origin_at_a_tight_band() {
        let b = Band::new(1e-12, 1e-11).unwrap();
        let z = Vec3::new(0.0, 0.0, 1.0);
        let (c, s) = (core::f64::consts::FRAC_1_SQRT_2, core::f64::consts::FRAC_1_SQRT_2);
        let d = Vec3::new(c, s, 0.0);
        let w = z.cross(d);
        for shift in [0.0, 10.0, 100.0, 1000.0] {
            let o = Point3::new(shift, 0.0, 0.0);
            let p = o + z;
            let cyl = geom::Surface::Cylinder {
                origin: o,
                axis: Vec3::new(1.0, 0.0, 0.0),
                radius: 1.0,
                u_ref: z,
            };
            // a plane tilted so the parting is decided below and the
            // foot agrees (sin ψ > 0)
            let m = (z * 0.5 + w * 0.5f64.sqrt()).normalize();
            let face = FaceDatum::of_plane(geom_brep::ReferenceNormal::of_face_outward(
                OutwardNormal::from_chart(m, true),
            ));
            let bisector = Reach::bisector(0.1, d, z, FaceBend::at(&cyl, p));
            let FirstOrder::Tied(tie) = read(d, bisector, face, b).unwrap() else {
                panic!("ties");
            };
            eprintln!("bisector shift {shift}: {:?}", tie.descend(b));
        }
    }
}
