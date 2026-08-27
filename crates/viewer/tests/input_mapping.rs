//! The input mapping, replayed as synthetic event streams.
//!
//! The claim under test is G1's: the toolkit contributes events, and
//! *which operation an event is* is a decision this library takes and
//! CI can check. Every test here runs with no window, no toolkit and
//! no GPU.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use bvh::Aabb;
use viewer::camera::{self, Camera, CameraOp};
use viewer::input::{self, InputMap, PointerButton, ViewportEvent, ViewportSize};

fn plate_bounds() -> Aabb {
    Aabb {
        min_x: 0.0,
        min_y: 0.0,
        min_z: 0.0,
        max_x: 0.060,
        max_y: 0.040,
        max_z: 0.008,
    }
}

fn framed() -> Camera {
    Camera::framing(&plate_bounds(), 1.6).expect("the plate frames")
}

fn viewport() -> ViewportSize {
    ViewportSize {
        width_px: 1600.0,
        height_px: 1000.0,
    }
}

/// Round rates, so the expected operations are exact numbers rather
/// than a restatement of the implementation.
fn round_map() -> InputMap {
    InputMap {
        orbit_radians_per_px: 0.01,
        zoom_rate_per_notch: 0.1,
        orbit_button: PointerButton::Middle,
        pan_button: PointerButton::Secondary,
    }
}

fn drag(button: PointerButton, shift: bool, dx: f64, dy: f64) -> ViewportEvent {
    ViewportEvent::Drag {
        button,
        shift,
        delta_px: [dx, dy],
    }
}

/// The headline test: a stream of pointer events folds to a known
/// sequence of typed operations.
#[test]
fn a_synthetic_event_stream_folds_to_the_expected_operations() {
    let map = round_map();
    let camera = framed();
    let events = vec![
        drag(PointerButton::Middle, false, 10.0, 0.0),
        drag(PointerButton::Middle, false, 0.0, -5.0),
        ViewportEvent::Scroll { units: 1.0 },
        drag(PointerButton::Primary, false, 20.0, 20.0),
        drag(PointerButton::Middle, false, 0.0, 0.0),
        ViewportEvent::Scroll { units: 0.0 },
    ];
    let (_, ops) = input::map_stream(&map, &camera, viewport(), &events).expect("finite events");
    assert_eq!(
        ops,
        vec![
            // Dragging right spins the model right: the camera orbits
            // the other way.
            CameraOp::Orbit {
                yaw: -0.1,
                pitch: 0.0
            },
            CameraOp::Orbit {
                yaw: 0.0,
                pitch: -0.05
            },
            // One notch toward the viewer.
            CameraOp::Dolly {
                factor: (-0.1f64).exp()
            },
        ],
        "the unbound primary drag, the zero drag and the zero scroll \
         are bound to no operation, and everything else is"
    );
}

/// The modifier is part of the binding, not part of the operation.
#[test]
fn shift_turns_the_orbit_binding_into_a_pan() {
    let map = round_map();
    let camera = framed();
    let plain = map
        .map(
            &drag(PointerButton::Middle, false, 4.0, 3.0),
            viewport(),
            &camera,
        )
        .expect("the orbit button is bound");
    let shifted = map
        .map(
            &drag(PointerButton::Middle, true, 4.0, 3.0),
            viewport(),
            &camera,
        )
        .expect("shift binds it to pan");
    assert!(matches!(plain, CameraOp::Orbit { .. }));
    assert!(matches!(shifted, CameraOp::Pan { .. }));
    // The dedicated pan button ignores the modifier.
    let secondary = map
        .map(
            &drag(PointerButton::Secondary, false, 4.0, 3.0),
            viewport(),
            &camera,
        )
        .expect("the pan button is bound");
    assert_eq!(secondary, shifted);
}

/// The pan rate is defined by a property, not by a constant: a drag
/// of `n` pixels must move the point under the cursor by `n` pixels.
/// This is the test that would catch a field-of-view or a
/// viewport-height factor going missing.
#[test]
fn a_pan_keeps_the_point_under_the_cursor() {
    let map = round_map();
    let camera = framed();
    let size = viewport();
    let aspect = size.aspect().expect("a viewport with area");
    let drag_px = 137.0;

    let op = map
        .map(
            &drag(PointerButton::Secondary, false, drag_px, 0.0),
            size,
            &camera,
        )
        .expect("the pan button is bound");
    let panned = camera::apply(&camera, &op).expect("a finite pan");

    // A point in the target plane, projected before and after.
    let before = camera
        .project(camera.target(), aspect)
        .expect("a finite aspect")
        .expect("the target is in front of the eye");
    let after = panned
        .project(camera.target(), aspect)
        .expect("a finite aspect")
        .expect("the target is still in front of the eye");
    // NDC x spans 2 across the viewport width.
    let moved_px = (after[0] - before[0]) * 0.5 * size.width_px;
    assert!(
        (moved_px - drag_px).abs() < 1e-6,
        "a {drag_px} px drag moved the world point {moved_px} px"
    );
}

/// Zoom is a fixed multiplicative step per notch, so a roll and its
/// reverse cancel exactly and the rate does not depend on where the
/// camera already is.
#[test]
fn scrolling_up_and_back_returns_the_same_distance() {
    let map = round_map();
    let camera = framed();
    let events = vec![
        ViewportEvent::Scroll { units: 3.0 },
        ViewportEvent::Scroll { units: -3.0 },
    ];
    let (back, ops) = input::map_stream(&map, &camera, viewport(), &events).expect("finite events");
    assert_eq!(ops.len(), 2);
    assert!(
        (back.distance() - camera.distance()).abs() < 1e-12 * camera.distance(),
        "scroll was not reversible: {} vs {}",
        back.distance(),
        camera.distance()
    );
}

/// A viewport with no area produces no pan: there is no world-per-pixel
/// rate to compute, and inventing one is how a zero-height pane turns
/// into a NaN camera.
#[test]
fn a_viewport_with_no_area_binds_no_pan() {
    let map = round_map();
    let camera = framed();
    let empty = ViewportSize {
        width_px: 800.0,
        height_px: 0.0,
    };
    assert_eq!(empty.aspect(), None);
    assert_eq!(
        map.map(
            &drag(PointerButton::Secondary, false, 5.0, 5.0),
            empty,
            &camera
        ),
        None
    );
    // Orbit does not need the viewport, so it is still bound.
    assert!(
        map.map(
            &drag(PointerButton::Middle, false, 5.0, 5.0),
            empty,
            &camera
        )
        .is_some()
    );
}

/// The stream folds the camera forward as it goes: a pan after a zoom
/// uses the distance the zoom left behind, which is what makes the
/// mapping faithful rather than merely typed.
#[test]
fn the_stream_folds_the_camera_between_events() {
    let map = round_map();
    let camera = framed();
    let size = viewport();
    let pan = drag(PointerButton::Secondary, false, 100.0, 0.0);

    let (_, plain) = input::map_stream(&map, &camera, size, &[pan]).expect("finite events");
    let (_, after_zoom) = input::map_stream(
        &map,
        &camera,
        size,
        &[ViewportEvent::Scroll { units: 5.0 }, pan],
    )
    .expect("finite events");

    let world_of = |op: &CameraOp| match *op {
        CameraOp::Pan { right, .. } => right,
        other => panic!("expected a pan, got {other:?}"),
    };
    let plain_pan = world_of(plain.first().expect("one operation"));
    let zoomed_pan = world_of(after_zoom.get(1).expect("two operations"));
    assert!(
        zoomed_pan.abs() < plain_pan.abs(),
        "zooming in did not shrink the pan step: {plain_pan} then {zoomed_pan}"
    );
}
