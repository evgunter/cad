use geom::Surface;
use geom_core::{Point3, Vec3};

/// **A wrap tip's reading asks the chart for its singular set**: a
/// sphere, a cone and a spindle sheet (lemon, apple and horn) have
/// one, so a tip off it reads `Off`; a plane, a cylinder and a ring
/// torus have none, so a tip there reads `NoSingularSet`. A sheet
/// left off reads its tip as having no set to be on.
#[test]
fn the_sheet_has_a_singular_set_its_tips_are_read_against() {
    let (o, z, x) = (
        Point3::new(0.2, -0.1, 0.3),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    let sheet = |major_radius: f64| Surface::Spindle {
        center: o,
        axis: z,
        major_radius,
        minor_radius: 1.0,
        u_ref: x,
    };
    for (surface, has) in [
        (sheet(-0.4), true),
        (sheet(0.4), true),
        (sheet(1.0), true),
        (
            Surface::Sphere {
                center: o,
                radius: 1.0,
                axis: z,
                u_ref: x,
            },
            true,
        ),
        (
            Surface::Cone {
                apex: o,
                axis: z,
                half_angle: 0.4,
                u_ref: x,
            },
            true,
        ),
        (
            Surface::Plane {
                origin: o,
                normal: z,
                u_ref: x,
            },
            false,
        ),
        (
            Surface::Cylinder {
                origin: o,
                axis: z,
                radius: 1.0,
                u_ref: x,
            },
            false,
        ),
        (
            Surface::Torus {
                center: o,
                axis: z,
                major_radius: 2.0,
                minor_radius: 0.5,
                u_ref: x,
            },
            false,
        ),
    ] {
        assert_eq!(super::has_singular_set(&surface), has, "{surface:?}");
    }
}
