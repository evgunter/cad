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
//! whose `RoleSeg` has none, printed for the operator who replays a
//! picking-path bug). `PreviewError::Transition` is not one of
//! them and is not covered here either: it needs a replayed chain to
//! carry a verb at all, so its prose row sits beside the chain that
//! produces it (`tests/path_authoring.rs`,
//! `an_illegal_walk_refuses_at_the_preview_and_at_the_door`).

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used, clippy::panic)]

use bvh::Aabb;
use editor_core::{HitTestError, NodeStanding, UnnamedEntity};
use pncad::document::{RecipeNodeId, Said, Speaker};
use pncad::geom_core::Point3;
use pncad::mesh::TessellateError;
use test_utils::refusal::tagged;
use viewer::camera::{Camera, CameraError, CameraOp, CameraOpError};
use viewer::matetool::MateToolError;
use viewer::pickindex::{
    EdgeNameFault, EdgeNamesRefused, IdMapError, PatchId, PickError, PickIndexError,
};
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

    // A finite refused field is spelled to be read: plain `Display`
    // writes this radius as three hundred and one characters, and this
    // field of view as `3.3000000000000003`.
    let degenerate = CameraError::DegenerateScene { radius: -1.0e300 }.to_string();
    assert_eq!(
        degenerate,
        "a scene radius of -1e300 is not a positive extent to frame against"
    );
    prose(&degenerate, "DegenerateScene");

    let fov = CameraError::FieldOfViewOutOfRange { fov_y: 3.0 * 1.1 }.to_string();
    assert_eq!(
        fov,
        "a vertical field of view of 3.3 rad is not strictly inside (0, pi)"
    );
    prose(&fov, "FieldOfViewOutOfRange");

    prose(&CameraError::UnusableBounds.to_string(), "UnusableBounds");

    // The one arm whose fact is about the SEAM rather than the
    // camera: the algebra succeeded and the GPU cannot hold what it
    // produced, so the sentence names the bound rather than a field.
    let undrawable = CameraError::UndrawableProjection.to_string();
    assert!(undrawable.contains("3.4e38"), "{undrawable}");
    prose(&undrawable, "UndrawableProjection");

    // The one aspect-dependent refusal says its cause and the remedy,
    // through the door that raises it, at every input that reaches it:
    // never the aspect's seventeen digits, the stand-off nobody can act
    // on, a ratio that is not one ("about 0 times", "about inf times"),
    // or a shape when the lens is what binds.
    let unit = Some(Aabb {
        min_x: 0.0,
        min_y: 0.0,
        min_z: 0.0,
        max_x: 1.0,
        max_y: 1.0,
        max_z: 1.0,
    });
    let refusal = |camera: Result<Camera, CameraError>| match camera {
        Err(error @ CameraError::Unfittable { required, .. }) => (error.to_string(), required),
        other => panic!("expected an unfittable refusal, got {other:?}"),
    };
    let tall = 1.0 / 36.6;
    let (shape, required) = refusal(Camera::framing(unit, tall));
    for wanted in ["37 times taller", "widen it"] {
        assert!(shape.contains(wanted), "{shape} lacks {wanted:?}");
    }
    for unwanted in [tall.to_string(), required.to_string()] {
        assert!(!shape.contains(&unwanted), "{shape} renders {unwanted}");
    }
    prose(&shape, "Unfittable");

    let lens = Camera::new(Point3::new(0.0, 0.0, 0.0), 1.0, 0.0, 0.0, 0.01, 1.0)
        .expect("a narrow lens is a camera");
    for aspect in [2.0, 1.0, 0.2] {
        let (said, _) = refusal(lens.fitted(unit, aspect));
        assert!(
            said.contains("field of view") && !said.contains("times"),
            "at aspect {aspect} the narrow lens is the cause: {said}"
        );
        prose(&said, "Unfittable");
    }

    for aspect in [1.0e-200, 1.0e-310] {
        let (said, _) = refusal(Camera::framing(unit, aspect));
        assert!(
            said.contains("too narrow for its height") && !said.contains("times"),
            "at aspect {aspect:e} no ratio is readable: {said}"
        );
        assert!(!said.contains("inf") && !said.contains("e-"), "{said}");
    }
}

#[test]
fn camera_op_error_forwards_its_framing_arm() {
    let inner = CameraError::UnusableBounds;
    let outer = CameraOpError::Unframeable(inner).to_string();
    assert_eq!(outer, inner.to_string());

    let dolly = CameraOpError::NonPositiveDolly {
        factor: -(0.1 + 0.2),
    }
    .to_string();
    assert_eq!(
        dolly,
        "a dolly factor of -0.3 is not a positive scale for a viewing distance"
    );
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
    // Each move as `apply` refuses it: the field it refuses for, and a
    // finite sibling whose plain `Display` is float noise
    // (`0.30000000000000004`, `1.2100000000000002`) spelled to be read.
    let orbit = CameraOp::Orbit {
        yaw: f64::NAN,
        pitch: 0.1 + 0.2,
    }
    .to_string();
    assert_eq!(orbit, "orbit by yaw NaN rad, pitch 0.3 rad");
    prose(&orbit, "Orbit");

    let pan = CameraOp::Pan {
        right: f64::INFINITY,
        up: 1.1 * 1.1,
    }
    .to_string();
    assert_eq!(pan, "pan by right inf, up 1.21");
    prose(&pan, "Pan");

    let dolly = CameraOp::Dolly {
        factor: -(0.1 + 0.2),
    }
    .to_string();
    assert_eq!(dolly, "dolly by a factor of -0.3");
    prose(&dolly, "Dolly");

    // The button's name, whatever the fit was handed: the error beside
    // it says what was wrong, so neither field is rendered.
    let unit = Aabb {
        min_x: 0.0,
        min_y: 0.0,
        min_z: 0.0,
        max_x: 1.0,
        max_y: 1.0,
        max_z: 1.0,
    };
    for extent in [Some(unit), None] {
        let frame = CameraOp::Frame {
            extent,
            aspect: 1.475_426_319_176_319_3,
        }
        .to_string();
        assert_eq!(frame, "zoom to fit", "{extent:?}");
        prose(&frame, "Frame");
    }
}

#[test]
fn scene_error_names_the_counts_it_carries() {
    let delta = SceneError::InvalidDisplayTolerance {
        delta: -1.0,
        unit: pncad::quantity::MM,
    }
    .to_string();
    assert!(
        delta.contains("-1 mm"),
        "the δ in the unit it was written in: {delta}"
    );
    prose(&delta, "InvalidDisplayTolerance");

    // The second δ arm, whose whole point is that it is NOT the first:
    // the value it names is a finite, strictly positive length, and
    // what it lacks is a millimetre reading.
    let coarse = SceneError::DisplayToleranceOverflowsMillimetres {
        delta: 1.0e306,
        unit: pncad::quantity::M,
    }
    .to_string();
    assert!(coarse.contains("1e306 m"), "{coarse}");
    assert!(
        coarse.contains("millimetre"),
        "the arm says what the δ lacks, not that it is not a length: {coarse}"
    );
    prose(&coarse, "DisplayToleranceOverflowsMillimetres");

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

    // Scientific, like the δ-overflow arm above and for the same
    // reason: a coordinate that reaches this arm is past `f32::MAX`,
    // and the sentence has to be one a person can read.
    let undrawable = SceneError::UndrawablePosition {
        position: [1.0e39, 0.0, 0.0],
    }
    .to_string();
    assert!(undrawable.contains("1e39"), "{undrawable}");
    assert!(
        !undrawable.contains("10000000"),
        "the coordinate is spelled in exponent form: {undrawable}"
    );
    prose(&undrawable, "UndrawablePosition");
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

/// A value on a freedom the mate's kind fixes names the kind and the
/// freedom, in the kernel's own words for each — never a variant name.
#[test]
fn mate_tool_error_names_the_freedom_a_kind_fixes() {
    use pncad::document::{Coord, TargetAxis, VarKind};
    let outer = MateToolError::NotAFreedom {
        coord: Coord::Slide(TargetAxis::X),
        kind: VarKind::Axis,
    }
    .to_string();
    assert!(outer.contains(&VarKind::Axis.to_string()), "{outer}");
    assert!(
        outer.contains(Coord::Slide(TargetAxis::X).name()),
        "{outer}"
    );
    prose(&outer, "NotAFreedom");
}

#[test]
fn scene_doc_error_renders_its_postcondition_arm() {
    prose(&SceneDocError::NoNodeMinted.to_string(), "NoNodeMinted");
}

#[test]
fn id_map_error_names_the_patch_and_the_count() {
    let duplicate = IdMapError::Duplicate {
        key: PatchId {
            node: RecipeNodeId::new(0, tagged(7)),
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

/// The layout arm is this layer's OWN finding — no payload to forward
/// — so it names the address it refused, which is the only thing a
/// reader can act on.
#[test]
fn pick_index_error_names_the_body_drawn_twice() {
    let drawn_twice = PickIndexError::DrawnTwice {
        node: RecipeNodeId::new(0, tagged(7)),
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
    let inner = HitTestError::Standing(NodeStanding::Failed {
        node: RecipeNodeId::new(0, tagged(4)),
    });
    let outer = PickError::HitTest(inner.clone()).to_string();
    assert!(outer.contains(&inner.to_string()), "{outer}");
    prose(&outer, "NodeFailed");
}

/// A drawn edge with no name carries the naming layer's own report —
/// a lookup's, whole, and the sentence says no hit test ran, because
/// the index was built by a table read and the status line that
/// forwards this refusal must not name an event that did not happen.
#[test]
fn edge_name_fault_forwards_its_unnamed_arm() {
    let inner = UnnamedEntity {
        node: RecipeNodeId::new(0, tagged(4)),
        entity: editor_core::EntityRef {
            body: 0,
            key: editor_core::EntityKey::Edge(pncad::topo::EdgeKey::default()),
        },
    };
    let outer = EdgeNameFault::Unnamed(inner).to_string();
    assert!(outer.contains(&inner.to_string()), "{outer}");
    assert!(!outer.contains("hit test"), "{outer}");
    prose(&outer, "Unnamed");
    let picked = PickError::EdgeName(EdgeNameFault::Unnamed(inner)).to_string();
    assert!(picked.contains(&inner.to_string()), "{picked}");
    assert!(!picked.contains("hit test"), "{picked}");
}

/// A body's edge-name refusal names the body and its counts, and
/// forwards its refusal through [`EdgeNameFault::Unnamed`]'s own words
/// rather than saying "no name" again in its own.
#[test]
fn edge_names_refused_forwards_its_first_refusal() {
    let first = crate::common::unnamed_edge(RecipeNodeId::new(0, tagged(4)), 1);
    let said = EdgeNamesRefused {
        node: RecipeNodeId::new(0, tagged(4)),
        body: 1,
        first,
        named: 11,
        refused: 1,
    }
    .to_string();
    let fault = Said(
        &EdgeNameFault::Unnamed(first),
        Speaker::TAG.about(RecipeNodeId::new(0, tagged(4))),
    )
    .to_string();
    assert_eq!(
        said,
        format!(
            "the index names 11 of the 12 edges it draws on body 1 of node 000000000004; the first it cannot: {fault}"
        )
    );
    prose(&said, "EdgeNamesRefused");
}

/// The indeterminate-resolution status line forwards the cause's own
/// words and contributes only the noun it is talking about.
#[cfg(feature = "app")]
#[test]
fn indeterminate_wording_forwards_the_causes_own_words() {
    use editor_core::ResolveIndeterminate;
    use viewer::app::indeterminate_wording;

    let cause = ResolveIndeterminate {
        standing: NodeStanding::Failed {
            node: RecipeNodeId::new(0, tagged(6)),
        },
    };
    let shown = indeterminate_wording("face", &cause, Speaker::TAG);
    assert!(shown.contains("face"), "{shown}");
    assert!(shown.contains(&cause.to_string()), "{shown}");
    prose(&shown, "ResolveIndeterminate");
    prose(&shown, "Failed");
}

test_utils::loud_skip_marker!(
    feature = "app",
    row = app_lane_skipped_no_error_display_coverage_here,
    absent = "coverage of the app's error wording",
);

#[cfg(feature = "app")]
#[test]
fn startup_error_forwards_every_payload_arm() {
    use viewer::app::StartupError;

    let scene = SceneError::EmptyMesh;
    let outer = StartupError::Scene(SceneError::EmptyMesh).to_string();
    assert!(outer.contains(&scene.to_string()), "{outer}");

    let doc = StartupError::Document(SceneDocError::NoNodeMinted).to_string();
    assert!(
        doc.contains(&SceneDocError::NoNodeMinted.to_string()),
        "{doc}"
    );

    let entropy = || pncad::workspace::WorkspaceError::RandomnessUnavailable {
        message: "the entropy source is closed".to_owned(),
    };
    let id = StartupError::Identity(entropy()).to_string();
    assert!(id.contains(&entropy().to_string()), "{id}");

    prose(
        &StartupError::NoWgpuRenderState.to_string(),
        "NoWgpuRenderState",
    );
}
