//! Every public `viewer` error renders as prose, not as a struct dump.
//!
//! Two properties, and the second is the one that matters. First, each
//! type HAS a `Display` and the sentence names what failed with the
//! specific value its payload carries. Second, a wrapper arm whose
//! payload has its own `Display` FORWARDS to it rather than composing a
//! sentence about somebody else's refusal — asserted by containment, so
//! the inner layer stays free to reword itself.
//!
//! For every arm covered below, a cross-crate payload forwards exactly
//! as a local one does, so `debug_shaped` applies throughout.
//!
//! This is not a crate-wide claim. Two `Display` impls outside this
//! file still render a payload through `Debug`, each for a stated
//! reason: `WebStartupError::Runner` (a `JsValue` the orphan rule
//! forecloses writing a `Display` for) and `Disagreement` (a role path,
//! whose `RoleSeg` has none). `PreviewError::Transition` renders a
//! `profile::path::Verb` that has no `Display` yet.

use bvh::Aabb;
use editor_core::{HitTestError, InterrogateError, MateSide, NodePickError};
use pncad::document::{EditError, RecipeNodeId};
use pncad::mesh::TessellateError;
use viewer::camera::{CameraError, CameraOp, CameraOpError};
use viewer::history::ReplayError;
use viewer::matetool::MateToolError;
use viewer::pick::{EdgeNameFault, IdMapError, PatchId, PickError, PickIndexError};
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

/// The tessellation arm forwards `mesh`'s own refusal: the kernel
/// named that failure and the scene layer does not re-diagnose it.
#[test]
fn scene_error_forwards_its_tessellation_arm() {
    let inner = TessellateError::InvalidChordalTolerance { value: -1.0 };
    let outer = SceneError::NotTessellated(inner.clone()).to_string();
    assert!(outer.contains(&inner.to_string()), "{outer}");
    prose(&outer, "InvalidChordalTolerance");
}

/// A picked face whose frame cannot be derived carries the
/// interrogation door's own refusal, plus the side this layer knows.
#[test]
fn mate_tool_error_forwards_its_frame_arm() {
    let inner = InterrogateError::NoSuchName;
    let outer = MateToolError::Frame {
        side: MateSide::A,
        error: inner,
    }
    .to_string();
    assert!(outer.contains(&inner.to_string()), "{outer}");
    prose(&outer, "NoSuchName");
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

/// The indexing arm carries `editor-core`'s own refusal, and the root
/// it names is this layer's contribution — both reach the reader.
#[test]
fn pick_index_error_forwards_its_node_arm() {
    let node = RecipeNodeId(7);
    let inner = NodePickError::NotABody { node };
    let outer = PickIndexError::Node {
        node,
        error: inner.clone(),
    }
    .to_string();
    assert!(outer.contains(&inner.to_string()), "{outer}");
    assert!(outer.contains('7'), "{outer}");
    prose(&outer, "NotABody");
}

/// The layout arm is this layer's OWN finding — no payload to forward
/// — so it names the address it refused, which is the only thing a
/// reader can act on.
#[test]
fn pick_index_error_names_the_body_drawn_twice() {
    let drawn_twice = PickIndexError::DrawnTwice {
        node: RecipeNodeId(7),
        body: 2,
    }
    .to_string();
    assert!(drawn_twice.contains('7'), "{drawn_twice}");
    assert!(drawn_twice.contains('2'), "{drawn_twice}");
    prose(&drawn_twice, "DrawnTwice");
}

#[test]
fn pick_error_forwards_its_camera_arm() {
    let inner = CameraError::UnusableBounds;
    let outer = PickError::Camera(inner).to_string();
    assert!(outer.contains(&inner.to_string()), "{outer}");
    prose(&outer, "UnusableBounds");
}

/// The hit-test arm forwards `editor-core`'s words rather than
/// composing a sentence about somebody else's refusal.
#[test]
fn pick_error_forwards_its_hit_test_arm() {
    let inner = HitTestError::NodeFailed {
        node: RecipeNodeId(4),
    };
    let outer = PickError::HitTest(inner).to_string();
    assert!(outer.contains(&inner.to_string()), "{outer}");
    prose(&outer, "NodeFailed");
}

/// A drawn edge with no name carries the naming layer's own report.
#[test]
fn edge_name_fault_forwards_its_unnamed_arm() {
    let inner = HitTestError::NodeFailed {
        node: RecipeNodeId(4),
    };
    let outer = EdgeNameFault::Unnamed(inner).to_string();
    assert!(outer.contains(&inner.to_string()), "{outer}");
    prose(&outer, "Unnamed");
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

/// The indeterminate-resolution status line forwards the cause's own
/// words and contributes only the noun it is talking about.
#[cfg(feature = "app")]
#[test]
fn indeterminate_wording_forwards_the_causes_own_words() {
    use editor_core::ResolveIndeterminate;
    use viewer::app::indeterminate_wording;

    let cause = ResolveIndeterminate::TargetFailed {
        node: RecipeNodeId(6),
    };
    let shown = indeterminate_wording("face", &cause);
    assert!(shown.contains("face"), "{shown}");
    assert!(shown.contains(&cause.to_string()), "{shown}");
    prose(&shown, "TargetFailed");
}

/// **Loud skip.** The row below needs `viewer::app`, which is not in a
/// default-feature build; say so rather than letting the run report
/// one fewer test and nothing else. Its seat is the hosted row
/// `cargo nextest run -p viewer --features app`
/// (`.github/workflows/ci.yml`).
///
/// **This row closes no gate and cannot fail** — its payload is its
/// NAME in the PASS list. It names ONE row, so a second `app`-gated
/// row added to this file leaves the marker quietly incomplete;
/// nothing mechanical says so.
#[cfg(not(feature = "app"))]
#[test]
fn app_lane_skipped_startup_error_arms_not_checked_here() {
    println!(
        "SKIPPED (no --features app): startup_error_forwards_every_payload_arm \
         does not run - `StartupError`'s forwarding of the camera, scene and \
         document arms is unchecked in this build."
    );
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
