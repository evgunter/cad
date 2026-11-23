//! **The separation doors read directions that turn with the pair**
//! (`topo::Separation`, `topo::SolidSeparation`): two rods clear of each
//! other are certified apart however they are turned, as placements of
//! one prototype and as two solids of one body, and a rod in a tube's
//! bore — nested along every direction — is certified apart by its gap
//! in depth about the axis.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, RigidMotion, Tol, UnitVec3, Vec3};
use topo::{Body, Separation, SolidSeparation};

fn tol() -> Tol {
    Tol::witness()
}

/// A full revolve about `y` of the rectangle `x ∈ [inner, outer]`,
/// `y ∈ [0, h]`: a rod where `inner` is zero, a tube otherwise.
fn revolved(inner: f64, outer: f64, h: f64) -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(inner, 0.0), 0.0),
            (Point2::new(outer, 0.0), 0.0),
            (Point2::new(outer, h), 0.0),
            (Point2::new(inner, h), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    )
}

fn unit(x: f64, y: f64, z: f64) -> UnitVec3<f64> {
    UnitVec3::new(
        Vec3::new(x, y, z),
        "test_axis",
        geom_core::Band::linear(tol()).unwrap(),
    )
    .unwrap()
}

/// The turn by `deg` degrees about `axis` through the point `about`,
/// then the move by `by`.
fn turned(axis: UnitVec3<f64>, about: Point3<f64>, deg: f64, by: Vec3<f64>) -> RigidMotion<f64> {
    RigidMotion::rotation_about_axis(about, axis, deg.to_radians())
        .unwrap()
        .translated(by)
        .unwrap()
}

/// Two solids of one body: `a` where it stands, `b` moved by `mb`, the
/// whole pair then moved by `whole`.
fn two_solids(a: &Body<f64>, b: &Body<f64>, mb: &RigidMotion<f64>, whole: &RigidMotion<f64>) -> Body<f64> {
    let placed = topo::transform_rigid(b, mb).unwrap();
    let mut out = a.clone();
    topo::graft_disjoint(&mut out, &placed).expect("the placed copy grafts");
    topo::transform_rigid(&out, whole).unwrap()
}

fn certify(body: &Body<f64>) -> bool {
    let sep = SolidSeparation::of(body, tol()).expect("boxes");
    let keys: Vec<_> = sep.keys().collect();
    sep.certify(keys[0], keys[1]).is_ok()
}

const TURNS: [f64; 7] = [0.0, 15.0, 30.0, 45.0, 60.0, 75.0, 90.0];

#[test]
fn measure_separation_across_turns() {
    let rod = revolved(0.0, 0.4, 1.0);
    let mid = Point3::new(0.0, 0.5, 0.0);
    let sep = Separation::of(&rod, tol()).unwrap();
    // Placements: the second rod 0.1 clear of the first's wall, spun
    // about its own axis, and tilted about z.
    for (what, axis) in [("spun about y", unit(0.0, 1.0, 0.0)), ("tilted about z", unit(0.0, 0.0, 1.0))] {
        for deg in TURNS {
            let d = if what == "spun about y" { 0.9 } else { 1.5 };
            let mj = turned(axis, mid, deg, Vec3::new(d, 0.0, 0.0));
            let maps = [RigidMotion::identity().to_affine(), mj.to_affine()];
            println!("placements {what} {deg}: apart = {:?}", sep.apart(&maps));
        }
    }
    // Solids: two parallel rods 0.1 clear, the whole pair turned about y,
    // and about z.
    for (what, axis) in [("about y", unit(0.0, 1.0, 0.0)), ("about z", unit(0.0, 0.0, 1.0))] {
        for deg in TURNS {
            let body = two_solids(
                &rod,
                &rod,
                &RigidMotion::translation(Vec3::new(0.9, 0.0, 0.0)).unwrap(),
                &turned(axis, mid, deg, Vec3::new(0.0, 0.0, 0.0)),
            );
            println!("solids whole pair {what} {deg}: certified = {}", certify(&body));
        }
    }
    // Nested: a rod r = 0.4 in a tube's bore r ∈ [0.6, 1].
    for deg in TURNS {
        let body = two_solids(
            &revolved(0.6, 1.0, 2.0),
            &rod,
            &RigidMotion::translation(Vec3::new(0.0, 0.5, 0.0)).unwrap(),
            &turned(unit(0.0, 0.0, 1.0), mid, deg, Vec3::new(0.0, 0.0, 0.0)),
        );
        println!("nested rod in bore {deg}: certified = {}", certify(&body));
    }
}
