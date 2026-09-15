//! Does `to_affine`'s third column equal `u x v` for an aim-minted frame?
use pncad::geom_core::{Band, Mat3, OrthoFrame, Point3, Tol, UnitVec3, Vec3};

fn main() {
    let band = Band::linear(Tol::witness()).unwrap();
    let aim = UnitVec3::new(Vec3::new(0.3, -0.7, 1.1), "probe_aim", band).unwrap();
    // A perpendicular BY CONSTRUCTION, exactly as both kernel callers build one.
    let perp = Vec3::new(1.0, 2.0, 3.0).cross(aim.get());
    let f = OrthoFrame::from_aim(Point3::origin(), aim, perp, "probe_perp", band).unwrap();
    let (u, v, w) = (f.u().get(), f.v().get(), f.w().get());
    let cross = u.cross(v);
    let bits = |x: Vec3<f64>| [x.x.to_bits(), x.y.to_bits(), x.z.to_bits()];
    println!("w      = {:?}", bits(w));
    println!("u x v  = {:?}", bits(cross));
    println!("equal? {}", bits(w) == bits(cross));
    let door = f.to_affine();
    let hand = pncad::geom_core::Affine3::from_parts(
        Mat3::from_cols(u, v, cross),
        Point3::origin() - Point3::origin(),
    );
    println!(
        "to_affine c2 = {:?} vs hand c2 = {:?}",
        bits(door.linear.c2),
        bits(hand.linear.c2)
    );
}
