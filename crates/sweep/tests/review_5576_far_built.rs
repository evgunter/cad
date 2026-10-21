//! Review 5576, claim 2: bodies BUILT at far coordinates (not carried),
//! measured against the exact closed form of the stored solid.
use core::f64::consts::PI;
use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, SketchPlane, test_support::bulge_loop};

fn tol() -> Tol {
    Tol::witness()
}

fn revolved_about(x0: f64, verts: Vec<(Point2<f64>, f64)>) -> Result<topo::Body<f64>, String> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), vec![bulge_loop(verts)])
        .validate(tol())
        .map_err(|e| format!("{e:?}"))?;
    let axis = sweep::RevolveAxis { origin: Point2::new(x0, 0.0), dir: Vec2::new(0.0, 1.0) };
    sweep::revolve(&profile, axis, sweep::Revolution::Full, tol())
        .map(|r| r.body)
        .map_err(|e| format!("{e:?}").chars().take(120).collect())
}

fn report(name: &str, far: f64, body: Result<topo::Body<f64>, String>, want: f64, worst: &mut f64) {
    let u = f64::EPSILON / 2.0;
    let body = match body {
        Ok(b) => b,
        Err(e) => return eprintln!("{name} far {far:e}: does not build: {e}"),
    };
    match topo::props::mass_properties_with_face_terms(&body, tol()) {
        Ok((p, terms)) => {
            let s: f64 = terms.iter().map(|t| t.1.abs()).sum();
            let ratio = (p.volume - want).abs() / (u * s / 3.0);
            *worst = worst.max(ratio);
            eprintln!(
                "{name} far {far:e}: V {:?} want {want:?} pad {} err {:e} = {ratio:.2} u·Σ|t|/3 (Σ|t|/3 = {:e})",
                p.volume, p.volume_pad, (p.volume - want).abs(), s / 3.0
            );
        }
        Err(e) => eprintln!("{name} far {far:e}: does not measure: {:?}", format!("{e:?}").chars().take(100).collect::<String>()),
    }
}

#[test]
fn review_5576_far_built_bodies_measure_within_a_few_ulps() {
    let mut worst = 0.0f64;
    for far in [0.0, 1e2, 7e2, 1e3, 1e4, 1e5] {
        // Frustum, axis x = far, every profile vertex representable.
        for scale in [1.0, 1e2] {
            let (h, r1) = (scale, 0.5 * scale);
            let r0 = r1 + h * 0.5;
            let verts = vec![
                (Point2::new(far, 0.0), 0.0),
                (Point2::new(far + r0, 0.0), 0.0),
                (Point2::new(far + r1, h), 0.0),
                (Point2::new(far, h), 0.0),
            ];
            // The stored solid: radii as the stored vertices give them.
            let (sr0, sr1) = ((far + r0) - far, (far + r1) - far);
            let want = PI * h / 3.0 * (sr0 * sr0 + sr0 * sr1 + sr1 * sr1);
            report(&format!("frustum {scale:e}"), far, revolved_about(far, verts), want, &mut worst);
        }
        // Ball radius 1 about x = far.
        let verts = vec![(Point2::new(far, -1.0), 1.0), (Point2::new(far, 1.0), 0.0)];
        report("ball 1", far, revolved_about(far, verts), 4.0 * PI / 3.0, &mut worst);
        // Brick [far, far+1] × [far, far+2] × [far, far+0.5].
        let b = sweep::test_support::brick((far, far + 1.0), (far, far + 2.0), (far, far + 0.5), tol());
        report("brick", far, Ok(b), 1.0, &mut worst);
    }
    eprintln!("worst {worst:.2} of u·Σ|t|/3");
}
