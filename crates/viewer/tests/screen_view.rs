//! **The view a camera lends a pane** (`viewer::screen::view_of`): what
//! a screen-sized mark is sized through, and the window it refuses.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use pncad::geom_core::Point3;
use viewer::camera::{Camera, CameraError};
use viewer::input::ViewportSize;
use viewer::screen::view_of;

/// **What `view_of` answers for a window it takes**: the camera's eye,
/// and the vertical field over the vertical pixel count.
///
/// A wide window and a tall one,
/// so a door that swapped the sides, or scaled by the larger one,
/// reds one of them.
#[test]
fn view_of_reports_the_camera_and_the_window_it_is_given() {
    let camera = common::framed(16.0 / 9.0);
    for (width_px, height_px) in [(1280.0, 720.0), (600.0, 900.0)] {
        let view = view_of(
            &camera,
            ViewportSize {
                width_px,
                height_px,
            },
        )
        .expect("a finite window with area has a view");
        // Component-wise: the geometry types carry no `PartialEq`.
        assert_eq!(view.eye.to_array(), camera.eye().to_array());
        let scale = 2.0 * (camera.fov_y() * 0.5).tan() / height_px;
        assert!(
            (view.metres_per_pixel_at_one_metre - scale).abs() <= scale * 1.0e-15,
            "a {width_px} x {height_px} window scaled {} against {scale}",
            view.metres_per_pixel_at_one_metre,
        );
    }
}

/// **`view_of` refuses a window that is not a number of pixels,
/// in the camera's own words.**
///
/// The claim is not that it refuses — an `Option` would carry that —
/// but that what a caller reads is what the SIBLING door on the same
/// two quantities says. `Camera::ray_through` takes the same
/// `ViewportSize` and answers a `CameraError` about it, naming the
/// side that was not a number and carrying its value; this door is
/// the other one on those inputs, and a caller holding both must not
/// have to learn two vocabularies for one fact.
///
/// Compared through `Debug` rather than by `==`, because half of
/// these values are `NaN` and a `CameraError` carrying one is not
/// equal to itself.
#[test]
fn view_of_refuses_a_window_the_way_the_cameras_own_door_does() {
    let camera = refusing_camera();
    for (width_px, height_px) in [
        (1280.0, f64::NAN),
        (f64::NAN, 800.0),
        (1280.0, 0.0),
        (0.0, 800.0),
        (1280.0, f64::INFINITY),
        (f64::INFINITY, 800.0),
    ] {
        let viewport = ViewportSize {
            width_px,
            height_px,
        };
        let Err(refused) = view_of(&camera, viewport) else {
            panic!("a {width_px} x {height_px} px window came back as a view");
        };
        let Err(sibling) = camera.ray_through([0.0, 0.0], viewport) else {
            panic!("a {width_px} x {height_px} px window is one the camera's own door takes");
        };
        assert_eq!(
            format!("{refused:?}"),
            format!("{sibling:?}"),
            "a {width_px} x {height_px} px window: this door says {refused}, the camera's says {sibling}",
        );
        // And the words reach a reader, which is the whole of what a
        // named refusal buys over a `None`. Asserted per ARM: both of
        // `CameraError`'s sentences happen to contain the word
        // "viewport", so one predicate over both checks neither.
        let words = refused.to_string();
        match refused {
            CameraError::NotFinite { what, value } => {
                // The side named is the side that was wrong, which is
                // the claim a single shared `what` would break.
                let offending = if width_px.is_finite() {
                    "viewport height"
                } else {
                    "viewport width"
                };
                assert_eq!(
                    what, offending,
                    "a {width_px} x {height_px} px window was refused as {words}",
                );
                assert!(
                    words.contains(what) && words.contains(&value.to_string()),
                    "the refusal reads {words}, which does not carry both {what} and {value}",
                );
            }
            other => {
                assert_eq!(
                    other,
                    CameraError::UnusableBounds,
                    "a {width_px} x {height_px} px window was refused as {words}",
                );
                assert!(
                    words.contains("viewport aspect"),
                    "the zero-area refusal reads {words}, which never reaches the viewport",
                );
            }
        }
    }
}

/// **A camera for the rows above**: finite, small, and nothing else
/// about it matters — every refusal they measure is about the window.
fn refusing_camera() -> Camera {
    Camera::new(
        Point3::new(0.0, 0.0, 0.0),
        0.15,
        0.0,
        0.0,
        core::f64::consts::FRAC_PI_4,
        0.05,
    )
    .expect("a finite camera")
}

/// **The pane's guard does not cover this door, and the gap is the
/// INFINITE extent.**
///
/// Both rows this pair closes say the app never reaches
/// `view_of`'s refusal, because `viewport_ui` returns when
/// `ViewportSize::aspect` refuses a pane with no area. `aspect` asks
/// whether both sides are above zero — and `inf` is above zero, so a
/// pane of infinite extent HAS an aspect and reaches this door. The
/// premise holds for zero and for `NaN` and not in general, which is
/// why this door owes its own answer rather than inheriting one.
#[test]
fn the_panes_aspect_guard_admits_an_extent_this_door_refuses() {
    for (width_px, height_px) in [(1280.0, f64::INFINITY), (f64::INFINITY, 800.0)] {
        let viewport = ViewportSize {
            width_px,
            height_px,
        };
        assert!(
            viewport.aspect().is_some(),
            "a {width_px} x {height_px} px pane has no aspect, so the pane's guard covers this door after all",
        );
        assert!(
            view_of(&refusing_camera(), viewport).is_err(),
            "and this door took it",
        );
    }
    // The other two shapes, for the contrast the sentence above
    // makes: these the pane's guard does stop.
    for (width_px, height_px) in [(1280.0, 0.0), (1280.0, f64::NAN)] {
        assert!(
            ViewportSize {
                width_px,
                height_px
            }
            .aspect()
            .is_none(),
            "a {width_px} x {height_px} px pane has an aspect",
        );
    }
}
