//! The same exercise against the MERGE BASE's API, for the bit comparison.
use pncad::geom_core::linalg::frame::point_at;
use pncad::geom_core::{Affine3, Point3, Tol, Vec3};
use pncad::profile::SketchPlane;
use pncad::sweep::{TubeWindow, tube_along_arc, tube_along_arc_hollow};

fn aff_bits(a: &Affine3<f64>) -> [u64; 12] {
    let l = a.linear;
    [
        l.c0.x, l.c0.y, l.c0.z, l.c1.x, l.c1.y, l.c1.z, l.c2.x, l.c2.y, l.c2.z, a.translation.x,
        a.translation.y, a.translation.z,
    ]
    .map(f64::to_bits)
}

fn main() {
    let t = tube_along_arc::<f64>(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::unit_x(),
        3.0,
        TubeWindow::Full,
        0.5,
        Tol::witness(),
    )
    .unwrap();
    let mp = pncad::topo::mass_properties(&t.body, Tol::witness()).unwrap();
    println!(
        "TUBE volume bits {:#x} faces {}",
        mp.volume.to_bits(),
        t.body.faces().count()
    );

    let h = tube_along_arc_hollow::<f64>(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::unit_x(),
        3.0,
        TubeWindow::Full,
        0.5,
        0.1,
        Tol::witness(),
    )
    .unwrap();
    let mph = pncad::topo::mass_properties(&h.body, Tol::witness()).unwrap();
    println!(
        "HOLLOW volume bits {:#x} faces {}",
        mph.volume.to_bits(),
        h.body.faces().count()
    );

    // The old door's frame refusals, for the record.
    println!(
        "OLD skew tube -> {:?}",
        tube_along_arc::<f64>(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 1.0).normalize(),
            3.0,
            TubeWindow::Full,
            0.5,
            Tol::witness(),
        )
        .err()
        .map(|e| e.to_string())
    );
    println!(
        "OLD non-unit u_ref -> {:?}",
        tube_along_arc::<f64>(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(2.0, 0.0, 0.0),
            3.0,
            TubeWindow::Full,
            0.5,
            Tol::witness(),
        )
        .err()
        .map(|e| e.to_string())
    );

    let exact = SketchPlane::<f64>::xy();
    println!("XY plane bits {:?}", aff_bits(&exact.placement));

    let pa = point_at(
        Point3::new(1.0, 2.0, 3.0),
        Point3::new(4.0, 6.0, 3.0),
        Vec3::unit_z(),
        Tol::witness(),
    )
    .unwrap();
    println!("POINT_AT bits {:?}", aff_bits(&pa));
    println!("done");
}
