//! Every public `viewer` error renders as prose, not as a struct dump.
//!
//! Two properties, and the second is the one that matters. First, each
//! type HAS a `Display` and the sentence names what failed with the
//! specific value its payload carries. Second, a wrapper arm whose
//! payload has its own `Display` FORWARDS to it rather than composing a
//! sentence about somebody else's refusal — asserted by containment, so
//! the inner layer stays free to reword itself.
//!
//! Where a message still shows a debug rendering it is a payload from
//! another crate that has no `Display` (issue #1111 covers those);
//! `debug_shaped` below is deliberately applied only to the arms whose
//! payloads are all local.

use bvh::Aabb;
use pncad::document::{EditError, RecipeNodeId};
use viewer::camera::{CameraError, CameraOp, CameraOpError};
use viewer::history::ReplayError;
use viewer::pick::{IdMapError, PatchId, PickError, PickIndexError};
use viewer::scene::{SceneDocError, SceneError};

/// Whether a rendering looks like a derived `Debug` rather than prose:
/// a struct-variant brace, or a variant identifier left standing.
fn debug_shaped(message: &str, variant: &str) -> bool {
    message.contains(" { ") || message.contains(variant)
}

fn prose(message: &str, variant: &str) {
    assert!(
        !debug_shaped(message, variant),
        "{message:?} reads as a debug rendering of {variant}"
    );
    assert!(!message.is_empty(), "{variant} renders as an empty string");
}

#[test]
fn camera_error_names_the_offending_value() {
    let not_finite = CameraError::NotFinite {
        what: "distance",
        value: f64::NAN,
    }
    .to_string();
    assert!(not_finite.contains("distance"), "{not_finite}");
    assert!(not_finite.contains("NaN"), "{not_finite}");
    prose(&not_finite, "NotFinite");

    let degenerate = CameraError::DegenerateScene { radius: 0.0 }.to_string();
    assert!(degenerate.contains('0'), "{degenerate}");
    prose(&degenerate, "DegenerateScene");

    let fov = CameraError::FieldOfViewOutOfRange { fov_y: 4.5 }.to_string();
    assert!(fov.contains("4.5"), "{fov}");
    prose(&fov, "FieldOfViewOutOfRange");

    prose(&CameraError::UnusableBounds.to_string(), "UnusableBounds");

    let unfittable = CameraError::Unfittable {
        required: 12.5,
        max_distance: 8.25,
        aspect: 0.125,
    }
    .to_string();
    for value in ["12.5", "8.25", "0.125"] {
        assert!(unfittable.contains(value), "{unfittable} lacks {value}");
    }
    prose(&unfittable, "Unfittable");
}

#[test]
fn camera_op_error_forwards_its_framing_arm() {
    let inner = CameraError::UnusableBounds;
    let outer = CameraOpError::Unframeable(inner).to_string();
    assert_eq!(outer, inner.to_string());

    let dolly = CameraOpError::NonPositiveDolly { factor: -2.0 }.to_string();
    assert!(dolly.contains("-2"), "{dolly}");
    prose(&dolly, "NonPositiveDolly");

    let not_finite = CameraOpError::NotFinite {
        what: "yaw",
        value: f64::INFINITY,
    }
    .to_string();
    assert!(not_finite.contains("yaw"), "{not_finite}");
    assert!(not_finite.contains("inf"), "{not_finite}");
    prose(&not_finite, "NotFinite");
}

#[test]
fn camera_op_renders_as_the_move_it_is() {
    let orbit = CameraOp::Orbit {
        yaw: 0.5,
        pitch: -0.25,
    }
    .to_string();
    assert!(orbit.contains("0.5") && orbit.contains("-0.25"), "{orbit}");
    prose(&orbit, "Orbit");

    let pan = CameraOp::Pan {
        right: 1.5,
        up: 2.5,
    }
    .to_string();
    assert!(pan.contains("1.5") && pan.contains("2.5"), "{pan}");
    prose(&pan, "Pan");

    let dolly = CameraOp::Dolly { factor: 0.75 }.to_string();
    assert!(dolly.contains("0.75"), "{dolly}");
    prose(&dolly, "Dolly");

    let frame = CameraOp::Frame {
        bounds: Aabb {
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: 1.0,
            max_y: 1.0,
            max_z: 1.0,
        },
        aspect: 1.5,
    }
    .to_string();
    assert!(frame.contains("1.5"), "{frame}");
    prose(&frame, "Frame");
}

#[test]
fn scene_error_names_the_counts_it_carries() {
    let delta = SceneError::InvalidDisplayTolerance { delta: -1.0 }.to_string();
    assert!(delta.contains("-1"), "{delta}");
    prose(&delta, "InvalidDisplayTolerance");

    prose(&SceneError::EmptyMesh.to_string(), "EmptyMesh");

    let mispaired = SceneError::MispairedIds { ids: 2, patches: 3 }.to_string();
    assert!(
        mispaired.contains('2') && mispaired.contains('3'),
        "{mispaired}"
    );
    prose(&mispaired, "MispairedIds");

    let broken = SceneError::BrokenPatchIndex {
        index: 9,
        positions: 4,
    }
    .to_string();
    assert!(broken.contains('9') && broken.contains('4'), "{broken}");
    prose(&broken, "BrokenPatchIndex");
}

#[test]
fn scene_doc_error_renders_its_postcondition_arm() {
    prose(&SceneDocError::NoNodeMinted.to_string(), "NoNodeMinted");
}

#[test]
fn id_map_error_names_the_patch_and_the_count() {
    let duplicate = IdMapError::Duplicate {
        key: PatchId {
            node: RecipeNodeId(7),
            body: 1,
            patch: 2,
        },
    }
    .to_string();
    for value in ["7", "1", "2"] {
        assert!(duplicate.contains(value), "{duplicate} lacks {value}");
    }
    prose(&duplicate, "Duplicate");
    assert!(!duplicate.contains("PatchId"), "{duplicate}");

    let too_many = IdMapError::TooManyPatches { patches: 5_000 }.to_string();
    assert!(too_many.contains("5000"), "{too_many}");
    prose(&too_many, "TooManyPatches");
}

#[test]
fn pick_index_error_forwards_its_id_arm() {
    let inner = IdMapError::TooManyPatches { patches: 9 };
    let outer = PickIndexError::Ids(inner).to_string();
    assert_eq!(outer, inner.to_string());
}

#[test]
fn pick_error_forwards_its_camera_arm() {
    let inner = CameraError::UnusableBounds;
    let outer = PickError::Camera(inner).to_string();
    assert!(outer.contains(&inner.to_string()), "{outer}");
    prose(&outer, "UnusableBounds");
}

#[test]
fn replay_error_names_the_log_position_and_forwards_the_refusal() {
    let inner = EditError::UnknownNode {
        id: RecipeNodeId(4),
    };
    let outer = ReplayError::Refused {
        index: 3,
        error: inner.clone(),
    }
    .to_string();
    assert!(outer.contains('3'), "{outer}");
    assert!(outer.contains(&inner.to_string()), "{outer}");
    prose(&outer, "Refused");
}

#[cfg(feature = "app")]
#[test]
fn startup_error_forwards_every_payload_arm() {
    use viewer::app::StartupError;

    let camera = CameraError::UnusableBounds;
    let outer = StartupError::Camera(camera).to_string();
    assert!(outer.contains(&camera.to_string()), "{outer}");

    let scene = SceneError::EmptyMesh;
    let outer = StartupError::Scene(SceneError::EmptyMesh).to_string();
    assert!(outer.contains(&scene.to_string()), "{outer}");

    let doc = StartupError::Document(SceneDocError::NoNodeMinted).to_string();
    assert!(
        doc.contains(&SceneDocError::NoNodeMinted.to_string()),
        "{doc}"
    );

    prose(
        &StartupError::NoWgpuRenderState.to_string(),
        "NoWgpuRenderState",
    );
}
