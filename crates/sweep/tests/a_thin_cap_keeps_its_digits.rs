//! **A thin cap keeps its volume's digits** — the precision row for the
//! sphere's solid-angle form.
//!
//! Gauss–Bonnet reads the region left of a loop as `2π − turning`, whose
//! rounding is a few ulps of `2π` however small the region: a cap of
//! height `h` off a ball of radius `R` keeps only `h/R` of its area's
//! digits that way, and the divergence theorem's own cancellation
//! between the cap and its disc costs another `R/h` of the volume's. The
//! flux arm reads a small region as a fan of triangles inside it instead
//! (`props::curved::solid_angle`), so the volume loses only the second
//! — measured alongside main's former reading of the cap as a chart
//! rectangle, the two land within the same few ulps of `R/h`, either
//! one ahead at a given height.
//! The ball's chart pole is on the slab's normal, so the cap's rim is a
//! latitude with the pole inside the face.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled, brick, finished};

#[test]
fn a_thin_cap_measures_to_its_closed_form() {
    let tol = Tol::witness();
    let none = topo::BooleanDeclarations::none();
    let pi = core::f64::consts::PI;
    let ball = finished(
        "the ball",
        ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), tol),
        tol,
    );
    for h in [1e-4_f64, 5e-5, 2e-5, 1e-5] {
        let slab = finished(
            "the slab",
            brick((-2.0, 2.0), (-2.0, 2.0), (1.0 - h, 3.0), tol),
            tol,
        );
        let body = topo::intersect_with(&slab, &ball, &none, tol)
            .expect("the slab meets the ball")
            .body()
            .expect("a body")
            .body
            .clone();
        let props = topo::mass_properties(&body, tol).unwrap();
        // The height the slab's stored level leaves, exactly.
        let h = 1.0 - (1.0 - h);
        let volume = pi * h * h * (3.0 - h) / 3.0;
        let area = 2.0 * pi * h + pi * (2.0 * h - h * h);
        let (dv, da) = (
            (props.volume - volume) / volume,
            (props.surface_area - area) / area,
        );
        // The area to a few ulps; the volume to that times the
        // divergence theorem's own `R/h`, which main's iso-rectangle
        // reading met too (`R = 1`).
        assert!(
            da.abs() < 4.0 * f64::EPSILON,
            "h = {h:e}: area off by {da:e}"
        );
        assert!(
            dv.abs() < 4.0 * f64::EPSILON / h,
            "h = {h:e}: volume off by {dv:e}"
        );
    }
}
