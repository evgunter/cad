//! S393 R2 end-to-end: a user sweeps a section along a path, getting
//! the start frame from the kernel's door through the façade, then
//! lofts the same section with a composed roll.
use pncad::authoring::polygon;
use pncad::geom_core::linalg::frame::path_start_frame;
use pncad::geom_core::{Affine3, Point3, Tol, Vec3};

const TURNS: f64 = 0.4;
fn main() {
    let tol = Tol::witness();
    // A helix-ish path a user might author: 33 points, degree 3.
    let pts: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let s = f64::from(k) / 32.0;
            let a = std::f64::consts::TAU * TURNS * s;
            Point3::new(a.cos(), a.sin(), 0.6 * s)
        })
        .collect();
    let path = pncad::geom::NurbsCurve3::interpolate(&pts, 3).expect("path");
    let (t0, t1) = path.domain();

    // 1. The door, as a user reaches it.
    let place = path_start_frame(path.eval(t0), path.deriv(t0), tol).expect("start frame");
    println!("start frame: {place:?}");
    let section =
        vec![polygon(&[(-0.1, -0.1), (0.1, -0.1), (0.1, 0.1), (-0.1, 0.1)], tol).expect("square")];
    for stations in [9usize, 13, 17, 25, 33] {
        match pncad::sweep::sweep_body::<f64>(&section, place, &path, stations, 3, tol) {
            Ok(swept) => {
                let m = pncad::topo::mass_properties(&swept.body, tol).expect("props");
                println!(
                    "{stations} stations: swept volume {} ± {:.1e}; validate {:?}",
                    m.volume,
                    m.volume_pad,
                    pncad::topo::validate_closed(&swept.body)
                );
            }
            Err(e) => println!("{stations} stations: refused {e:?}"),
        }
    }

    // 2. What happens if a user hands the door a zero tangent, or a pole tangent?
    println!(
        "zero tangent: {:?}",
        path_start_frame(Point3::<f64>::origin(), Vec3::new(0.0, 0.0, 0.0), tol).err()
    );
    println!(
        "pole tangent: {:?}",
        path_start_frame(Point3::<f64>::origin(), Vec3::unit_z(), tol).map(|f| f.linear)
    );
    // A tangent a few band-widths off the pole: decided or refused?
    let eps = tol.eps();
    for k in [0.5, 1.0, 5.0, 10.0, 20.0] {
        let t = Vec3::new(k * eps, 0.0, 1.0);
        let r = path_start_frame(Point3::<f64>::origin(), t, tol);
        println!(
            "tangent ({k}·eps, 0, 1): {}",
            match &r {
                Ok(f) => format!("x̂ = {:?}", f.linear.c0),
                Err(e) => format!("refused {e:?}"),
            }
        );
    }

    // 3. Composed roll, the way the narration says: rotation about the tangent * the frame.
    let stations: u32 = 6;
    let places: Vec<Affine3<f64>> = (0..stations)
        .map(|i| {
            let u = f64::from(i) / f64::from(stations - 1);
            let t = (t1 - t0).mul_add(u, t0);
            let (p, d) = (path.eval(t), path.deriv(t));
            let plane = path_start_frame(p, d, tol).expect("station frame");
            Affine3::rotation_about_axis(p, d, 0.8 * u) * plane
        })
        .collect();
    let sections: Vec<_> = (0..stations).map(|_| section.clone()).collect();
    let loft = pncad::sweep::loft_body::<f64>(&sections, &places, 3, tol).expect("loft");
    let ml = pncad::topo::mass_properties(&loft.body, tol).expect("props");
    println!(
        "rolled loft volume {} ± {:.1e}; validate {:?}",
        ml.volume,
        ml.volume_pad,
        pncad::topo::validate_closed(&loft.body)
    );
    let mut drift = 0.0f64;
    for (i, pl) in (0u32..).zip(places.iter()) {
        let u = f64::from(i) / f64::from(stations - 1);
        let t = (t1 - t0).mul_add(u, t0);
        let p = path.eval(t);
        drift = drift.max((pl.translation - (p - Point3::origin())).norm());
    }
    println!("composed-roll origin drift off the spine: {drift:.2e}");
}
