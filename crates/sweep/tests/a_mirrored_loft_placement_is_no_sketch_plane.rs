//! **A mirrored loft placement is no sketch plane.** `loft_body` reads
//! each section's loop sense in its sketch frame, so a placement that
//! mirrors would turn one ring clockwise in the world while its
//! neighbour runs counterclockwise, and the stacking fold, which reads
//! only the normals, would not see it. The placement a loft takes is a
//! [`SketchPlane`], which is a [`RigidMotion`]; twelve numbers become
//! one only at [`RigidMotion::check_rigid`], so the mirror refuses
//! there, before any section exists, and a proper rotation of the same
//! square builds.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{
    Affine3, Band, Decide, Interval, Mat3, Real, RigidMotion, RigidMotionError, Tol, Vec3,
};
use profile::SketchPlane;
use sweep::loft_body;
use sweep::test_support::loft_prism_sections;

/// The section-1 placement at `z = 1` whose linear columns are `cols`.
fn at_z1<T: Real>(cols: [[f64; 3]; 3]) -> Affine3<T> {
    let v = |c: [f64; 3]| Vec3::new(T::from_f64(c[0]), T::from_f64(c[1]), T::from_f64(c[2]));
    Affine3::from_parts(
        Mat3::from_cols(v(cols[0]), v(cols[1]), v(cols[2])),
        v([0.0, 0.0, 1.0]),
    )
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance forms a band")
}

/// The placements that are not rigid, each with the check that must
/// refuse it and the defect its message must name. The first is the
/// row's fixture: `(x, −y, z)`, whose third column is `+z` exactly as
/// the identity's, so the normals agree and only the determinant tells.
const NOT_RIGID: [(&str, [[f64; 3]; 3], &str, &str); 3] = [
    (
        "mirrored (x, -y, z)",
        [[1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]],
        "transform_rigid_det_plus_one",
        "it mirrors",
    ),
    (
        "scaled (2x, y, z)",
        [[2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        "transform_rigid_col0_unit",
        "it scales an axis",
    ),
    (
        "sheared, unit columns 0.6 apart",
        [[1.0, 0.0, 0.0], [0.6, 0.8, 0.0], [0.0, 0.0, 1.0]],
        "transform_rigid_col01_orth",
        "it shears",
    ),
];

fn refuses_at<T: Decide>(lane: &str) {
    for (label, cols, want, defect) in NOT_RIGID {
        match RigidMotion::<T>::check_rigid(&at_z1::<T>(cols), band()) {
            Err(e @ RigidMotionError::NotRigid { check }) => {
                assert_eq!(check, want, "{lane} {label}: the refusing check");
                let text = e.to_string();
                assert!(
                    text.contains(defect),
                    "{lane} {label}: the refusal names the defect `{defect}`, got {text}"
                );
            }
            other => panic!("{lane} {label}: refused NotRigid {want}, got {other:?}"),
        }
    }
}

#[test]
fn a_placement_that_mirrors_scales_or_shears_never_becomes_a_frame() {
    refuses_at::<f64>("f64");
    refuses_at::<Interval>("Interval");
}

/// The square rolled a quarter turn about `+z` at `z = 1`: proper, so
/// admitted. Corner `j` of the top ring stands over corner `j + 1` of
/// the bottom, so the walls are ruled and twisted, and the slice at
/// height `t` is the square inscribed in the base with its corners `t`
/// of the way along each side: area `4((1 − t)² + t²)`, which
/// integrates to `8/3`. A mirror read as a rotation would build no
/// solid there at all.
#[test]
fn a_proper_rotation_of_the_square_lofts_its_twisted_volume() {
    let quarter = [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
    let top = RigidMotion::check_rigid(&at_z1::<f64>(quarter), band())
        .expect("a quarter turn and a translation are rigid");
    let square = loft_prism_sections().swap_remove(0);
    let tol = Tol::witness();
    let lofted = loft_body::<f64>(
        &[square.clone(), square],
        &[SketchPlane::xy(), SketchPlane::from_frame(top)],
        1,
        tol,
    )
    .expect("the twisted square lofts");
    assert_eq!(
        topo::validate_geometric(&lofted.body, tol),
        Ok(()),
        "tier 3"
    );
    let m = topo::mass_properties(
        &topo::test_support::finished("the twisted square", lofted.body, tol),
        tol,
        topo::test_support::pinned_resolution(tol),
    )
    .expect("mass properties");
    let exact = 8.0 / 3.0;
    assert!(
        (m.enclosure.volume - exact).abs() <= m.enclosure.volume_pad + 1e-9,
        "volume {} ± {} brackets the derived {exact}",
        m.enclosure.volume,
        m.enclosure.volume_pad
    );
}
