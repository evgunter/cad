//! **Every refusal the feature tree draws fits where it is drawn.**
//!
//! The feature tree's fault line and the status line render a failed
//! node's `NodeError` `Display` verbatim, so each `NodeErrorKind` arm is
//! a sentence the person holding the mouse reads, and a forwarding arm
//! (`Extrude(e)`, `Split(e)`, `Mate(fault)`, …) reads the forwarded
//! refusal's own sentence inside its wrapper. These rows render every
//! `NodeErrorKind` arm, and every arm of each refusal a forwarding arm
//! carries, the way the viewer draws it — `node 5 failed:` and the
//! wrapper included — on a representative payload, and hold each to
//! the budget [`refusal_concision`](crate::refusal_concision) states.
//!
//! **What "every arm" covers.** One level of forwarding is exhaustive:
//! each `NodeErrorKind` arm, and each arm of the enum it forwards
//! (a thin wrapper — `SplitError`, `ReplayError` — is looked through to
//! the enum it wraps). A refusal forwarded two levels down (an
//! `EulerOpError` inside `ExtrudeError::Op`, a `PcurveMintError` inside
//! `RevolveError::Pcurve`) is rendered on one representative arm; its
//! own enum's lengths are held by the `*-refusal-prose-outgrows-the-viewer`
//! rows on its owner's slate.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{NodeError, NodeErrorKind, RecipeNodeId};

/// A `NodeErrorKind` as the feature tree's fault line draws it.
pub(crate) fn as_the_viewer_shows_it(kind: NodeErrorKind) -> String {
    NodeError {
        node: RecipeNodeId(5),
        kind,
        escalations: std::sync::Arc::new(Vec::new()),
    }
    .to_string()
}

/// The rows that may name an arena key: each reports a corrupt body, a
/// kernel invariant or a kernel finding, where the key is what the bug
/// report needs. Every other row names what it is about in words.
const KERNEL_KEYED: &[&str] = &[
    "Extrude/Op",
    "Revolve/VoidInsertion",
    "Revolve/Op",
    "Revolve/Pcurve",
    "Split/Reduce/ScaffoldingOperand",
    "Split/Reduce/ConsecutiveOnSectors",
    "Split/Reduce/CorruptOperand",
    "Split/Reduce/CrossingInsertion",
    "Split/Reduce/Euler",
    "Split/Join/SectionLoopMixed",
    "Split/Join/CutInvariant",
    "Split/Join/Corrupt",
    "Split/Join/Euler",
    "Split/Join/SectionInvariant",
    "Boolean/Join/SectionLoopMixed",
    "Boolean/Join/CutInvariant",
    "Boolean/Join/Corrupt",
    "Boolean/Join/Euler",
    "Boolean/Join/SectionInvariant",
    "Split/Finish/TornComponent",
    "Split/Finish/UnclassifiableComponent",
    "Split/Finish/Euler",
    "Split/Pcurves",
    "Transform/Pcurve",
    "Transform/Certify",
    "Transform/NullScaffold",
    "Loft/Euler",
    "Loft/Pcurve",
    "Blend/BodyNotIntact",
    "Blend/SurgeryInvariant",
    "Blend/Certify",
    "Blend/Op",
    "Naming/SplitLineage",
    "Naming/FragmentLineage",
    "Naming/SeamVertexParentage",
    "Naming/SharedRim",
    "FaceFrameReadback/Dangling",
    "Shell/Partition",
    "Shell/Insert",
    "Shell/Rim",
    "Shell/Corrupt",
    "Shell/Pcurve",
    // `topo::ShellClassifyError` and `topo::MassPropsError` still name
    // their shell and face by key; both sit in `topo/src/props.rs`,
    // which open PRs are reworking, and are filed rather than edited.
    "Shell/Roles",
    "Check/Unsupported",
];

/// The clause labels a refusal legitimately opens with that read, by
/// shape, like a stage prefix: one or two lowercase words and a colon.
/// A label here is English the person reads, not a pipeline stage.
pub(crate) const ALLOWED_LABELS: &[&str] = &[
    // The checks window's finding labels (`check separation: …`): the
    // check the person ran, named as the menu names it.
    "check separation",
    "check connectedness",
    "check chart-coherence",
];

/// The rows whose stage prefix sits in a file an open PR is reworking,
/// each with the one prefix it may still carry. An exact row id and an
/// exact label: a new prefix on the same row, or the same prefix on
/// another row, is still red. Each is filed with its owner, named here.
pub(crate) const FILED: &[(&str, &str)] = &[
    // `topo/src/replace_face.rs` (#2861), SHELL's:
    // work/shell/replace-face-refusals-open-with-a-stage-prefix-and-name-keys.md
    ("Shell/Face", "replace_face_offset"),
    ("Shell/Lift", "replace_face_offset"),
    // `topo/src/props.rs` (#2861, #3049), unowned:
    // work/issues/unowned-refusal-prose-outgrows-the-viewer.md
    ("Shell/Roles", "shell classification"),
    ("Check/Unsupported", "shell classification"),
    ("Check/Unsupported", "mass properties"),
    // `geom-brep/src/certify.rs` (#2861), unowned: the same row.
    ("Transform/Certify", "certification"),
    // `geom/src/curves.rs` (#2861): `EllipseInvalid` opens with
    // "ellipse construction:" and offers "declare" under a split:
    // work/chrome/the-refusal-shape-guard-has-blind-spots.md
    ("Split/Join/Section(Carrier)", "ellipse construction"),
    ("Boolean/Join/Section(Carrier)", "ellipse construction"),
];

/// The split rows that may still offer "declare", which a split has no
/// door for, each filed with its owner (the note above `FILED`'s
/// `EllipseInvalid` entries).
pub(crate) const FILED_DECLARE: &[&str] = &["Split/Join/Section(Carrier)"];

/// The rows that render a `Debug` form — a struct, or an arena key —
/// by exact row id, each filed with its owner.
pub(crate) const FILED_DEBUG: &[&str] = &[
    // `ClearanceRefusal::payload` (`editor-core/src/clearance.rs`)
    // renders the faces of `Unsupported` and `PoisonEnclosure`, the two
    // arms `min_separation` refuses with that carry evidence, as
    // `FaceKey` `Debug`, and this arm prints it; PROPS's:
    // work/props/props-refusal-prose-outgrows-the-viewer.md
    "MeasureClearanceRefused",
];

/// Every way the rows among `rows` fall short of the standard:
/// [`test_utils::refusal::problems`] on each, with the labels
/// [`ALLOWED_LABELS`] and [`FILED`] admit, the `Debug` rows [`FILED_DEBUG`]
/// admits, and the keys
/// [`KERNEL_KEYED`] admits.
pub(crate) fn over_budget(rows: &[(String, String)]) -> Vec<String> {
    let mut problems = Vec::new();
    for (name, text) in rows {
        eprintln!("MEASURE {} {name}: {text}", text.split_whitespace().count());
        let mut allowed = ALLOWED_LABELS.to_vec();
        allowed.extend(FILED.iter().filter(|(row, _)| row == name).map(|(_, l)| *l));
        let debug_filed = [
            format!("{name} renders a Debug struct"),
            format!("{name} dumps an arena key"),
        ];
        problems.extend(
            test_utils::refusal::problems(
                name,
                text,
                &allowed,
                KERNEL_KEYED.contains(&name.as_str()),
            )
            .into_iter()
            .filter(|p| {
                !(FILED_DEBUG.contains(&name.as_str())
                    && debug_filed.iter().any(|d| p.starts_with(d.as_str())))
            }),
        );
    }
    problems
}

mod payloads {
    use geom_core::{Band, Indeterminate, MarginDiag, Tol};

    pub(super) fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness band")
    }

    /// An in-band margin on a named predicate: the escalation a person
    /// meets most.
    pub(super) fn diag() -> Indeterminate {
        Indeterminate {
            margin: MarginDiag::Value(3.0e-10),
            band: band(),
            predicate: Some("side_of_plane"),
        }
    }

    pub(super) fn band_error() -> geom_core::BandError {
        geom_core::BandError::Empty {
            zero: 1.0e-6,
            escalate: 1.0e-9,
        }
    }

    pub(super) fn euler() -> topo::EulerOpError {
        topo::EulerOpError::StaleKey {
            key: topo::EntityId::Face(topo::FaceKey::default()),
        }
    }

    pub(super) fn pcurve() -> topo::PcurveMintError {
        topo::PcurveMintError::LoopNotClosed {
            face: topo::FaceKey::default(),
        }
    }

    pub(super) fn newell() -> geom_brep::NewellError {
        geom_brep::NewellError::NotPlanar { vertex: 3 }
    }
}

#[test]
fn every_node_refusal_renders_within_the_budget() {
    let rows: Vec<(String, String)> = node_refusals()
        .into_iter()
        .map(|(name, kind)| (name, as_the_viewer_shows_it(kind)))
        .collect();
    let problems = over_budget(&rows);
    assert!(problems.is_empty(), "{}", problems.join("\n"));
    // The join is shared; its recourse is its caller's. A split takes
    // no declaration and a Boolean does, so the same arm offers
    // "declare" under the one and not under the other.
    for (name, text) in &rows {
        if name.starts_with("Split/") && !FILED_DECLARE.contains(&name.as_str()) {
            assert!(!text.contains("declare"), "{name}: {text}");
        }
        // Every arm whose split rendering offers the join's recourse
        // offers the declaration under the Boolean instead.
        if let Some(arm) = name.strip_prefix("Boolean/Join/") {
            let split = rows
                .iter()
                .find(|(n, _)| *n == format!("Split/Join/{arm}"))
                .map(|(_, t)| t)
                .expect("every Boolean join row has its split twin");
            if split.contains(geom_core::NO_DECLARATION_RECOURSE) {
                assert!(
                    text.contains(geom_core::COINCIDENCE_RECOURSE),
                    "{name}: {text}"
                );
            }
        }
    }
}

/// Every `NodeErrorKind` arm, and every arm of each refusal it forwards.
fn node_refusals() -> Vec<(String, NodeErrorKind)> {
    let mut rows = own_arms();
    rows.extend(extrude());
    rows.extend(revolve());
    rows.extend(tube());
    rows.extend(split());
    rows.extend(transform());
    rows.extend(skin());
    rows.extend(loft());
    rows.extend(blend());
    rows.extend(profile());
    rows.extend(profile_replay());
    rows.extend(editor_payloads());
    rows.extend(document_arms());
    rows.extend(mate());
    rows.extend(shell());
    rows.extend(found_arms());
    rows
}

fn row(name: &str, kind: NodeErrorKind) -> (String, NodeErrorKind) {
    (name.to_owned(), kind)
}

/// The arms whose sentence `NodeErrorKind` writes itself, each on a
/// representative payload where it forwards.
fn own_arms() -> Vec<(String, NodeErrorKind)> {
    use editor_core::{Dimension, EvalError, ParamName, SlotId};
    use payloads::*;
    vec![
        row(
            "Expr",
            NodeErrorKind::Expr {
                slot: SlotId::Distance,
                source: EvalError::UnknownParam(ParamName::new("width")),
            },
        ),
        row(
            "ProfileLaneReplay(None)",
            NodeErrorKind::ProfileLaneReplay {
                loop_: 0,
                step: 2,
                structure: None,
            },
        ),
        row("ProfileAnchor", NodeErrorKind::ProfileAnchor { loop_: 0 }),
        row(
            "CurvedSolidFrontier",
            NodeErrorKind::CurvedSolidFrontier {
                what: "a sweep along a curved path",
            },
        ),
        row(
            "MissingInput",
            NodeErrorKind::MissingInput {
                input: RecipeNodeId(3),
            },
        ),
        row(
            "ToleranceConflict",
            NodeErrorKind::ToleranceConflict {
                document_eps: 1.0e-7,
                process_eps: 1.0e-9,
            },
        ),
        row(
            "WrongOperand",
            NodeErrorKind::WrongOperand {
                input: RecipeNodeId(3),
                expected: "body",
                found: "profile",
            },
        ),
        row(
            "EmptyOperand",
            NodeErrorKind::EmptyOperand {
                input: RecipeNodeId(3),
            },
        ),
        row(
            "EmptyHalf",
            NodeErrorKind::EmptyHalf {
                input: RecipeNodeId(3),
                half: editor_core::SplitHalf::Above,
            },
        ),
        row(
            "InstanceOutOfRange",
            NodeErrorKind::InstanceOutOfRange {
                input: RecipeNodeId(3),
                index: 7,
                count: 4,
            },
        ),
        row(
            "DegenerateDirection",
            NodeErrorKind::DegenerateDirection {
                role: "extrude direction",
            },
        ),
        row(
            "NonFiniteDirection",
            NodeErrorKind::NonFiniteDirection {
                role: "extrude direction",
            },
        ),
        row(
            "UnderflowedDirection",
            NodeErrorKind::UnderflowedDirection {
                role: "extrude direction",
            },
        ),
        row("Band", NodeErrorKind::Band(band_error())),
        row(
            "MissingSlot",
            NodeErrorKind::MissingSlot {
                slot: SlotId::Distance,
            },
        ),
        row(
            "VerbArity",
            NodeErrorKind::VerbArity {
                verb: verbs::VerbKind::Fillet,
                given: verbs::Arity::Two,
            },
        ),
        row(
            "Escalated",
            NodeErrorKind::Escalated {
                predicate: "side_of_plane",
                source: diag(),
            },
        ),
        row(
            "AxisInDifferentPlane",
            NodeErrorKind::AxisInDifferentPlane {
                axis: RecipeNodeId(3),
                axis_plane: Some(RecipeNodeId(1)),
                profile_plane: Some(RecipeNodeId(2)),
            },
        ),
        row(
            "NonPositiveCount",
            NodeErrorKind::NonPositiveCount { count: 0 },
        ),
        row(
            "PlacementsUncertified",
            NodeErrorKind::PlacementsUncertified { i: 0, j: 1 },
        ),
        row("UnschedulableCycle", NodeErrorKind::UnschedulableCycle),
        row(
            "SeedPinnedSection",
            NodeErrorKind::SeedPinnedSection {
                section: RecipeNodeId(3),
                param: ParamName::new("width"),
            },
        ),
        row(
            "DeclareSiteNotAnOperand",
            NodeErrorKind::DeclareSiteNotAnOperand {
                at: RecipeNodeId(3),
            },
        ),
        row(
            "DeclareUnsupportedPair",
            NodeErrorKind::DeclareUnsupportedPair {
                kinds: (
                    editor_core::EntityKind::Edge,
                    editor_core::EntityKind::Vertex,
                ),
                cross_operand: true,
            },
        ),
        row(
            "BlendSelectionEmpty",
            NodeErrorKind::BlendSelectionEmpty {
                verb: sweep::blend::BlendKind::Fillet,
            },
        ),
        row(
            "ShellLaneUnsupported",
            NodeErrorKind::ShellLaneUnsupported { lane: "interval" },
        ),
        row(
            "FaceFrameNotPlanar",
            NodeErrorKind::FaceFrameNotPlanar {
                carrier: geom_brep::SurfaceKind::Cylinder,
            },
        ),
        row(
            "DerivedFrameSection",
            NodeErrorKind::DerivedFrameSection {
                profile: RecipeNodeId(3),
                frame: RecipeNodeId(2),
            },
        ),
        row(
            "MeasureNotParallel",
            NodeErrorKind::MeasureNotParallel {
                verb: "distance",
                a: "plane",
                b: "plane",
                predicate: "measure_parallel",
            },
        ),
        row(
            "MeasureNonFinite",
            NodeErrorKind::MeasureNonFinite {
                source: EvalError::NonFiniteResult,
            },
        ),
        row(
            "PayloadExpr",
            NodeErrorKind::PayloadExpr {
                what: "placement",
                index: 2,
                source: EvalError::NonFiniteResult,
            },
        ),
        row(
            "AssertionDimension",
            NodeErrorKind::AssertionDimension {
                measured: Dimension::Length,
                bound: Dimension::Angle,
            },
        ),
    ]
}

fn extrude() -> Vec<(String, NodeErrorKind)> {
    use payloads::*;
    use sweep::ExtrudeError as E;
    [
        ("Band", E::Band(band_error())),
        ("DegenerateExtrusion", E::DegenerateExtrusion),
        ("ObliqueExtrusion", E::ObliqueExtrusion),
        (
            "ExtrusionEscalated",
            E::ExtrusionEscalated { source: diag() },
        ),
        (
            "CosurfaceEscalated",
            E::CosurfaceEscalated {
                loop_index: 0,
                vertex_index: 3,
                source: diag(),
            },
        ),
        (
            "SliverJoin",
            E::SliverJoin {
                loop_index: 0,
                vertex_index: 3,
                source: diag(),
            },
        ),
        (
            "SliverRim",
            E::SliverRim {
                loop_index: 0,
                segment_index: 3,
                source: diag(),
            },
        ),
        ("CapPlane", E::CapPlane { source: newell() }),
        (
            "SidePlane",
            E::SidePlane {
                loop_index: 0,
                segment_index: 3,
                source: newell(),
            },
        ),
        ("Op", E::Op { source: euler() }),
    ]
    .into_iter()
    .map(|(n, e)| row(&format!("Extrude/{n}"), NodeErrorKind::Extrude(e)))
    .collect()
}

fn revolve_arms() -> Vec<(&'static str, sweep::RevolveError)> {
    use payloads::*;
    use sweep::RevolveError as E;
    vec![
        ("Band", E::Band(band_error())),
        ("NonFiniteAxis", E::NonFiniteAxis),
        ("UnderflowedAxis", E::UnderflowedAxis),
        ("DegenerateAxis", E::DegenerateAxis),
        ("AxisEscalated", E::AxisEscalated { source: diag() }),
        ("DegenerateAngle", E::DegenerateAngle),
        ("FullRangeAngle", E::FullRangeAngle),
        ("AngleEscalated", E::AngleEscalated { source: diag() }),
        (
            "VertexCrossesAxis",
            E::VertexCrossesAxis {
                loop_index: 0,
                vertex_index: 3,
            },
        ),
        (
            "SliverRadius",
            E::SliverRadius {
                loop_index: 0,
                vertex_index: 3,
                source: diag(),
            },
        ),
        (
            "ArcCrossesAxis",
            E::ArcCrossesAxis {
                loop_index: 0,
                segment_index: 3,
            },
        ),
        (
            "SliverAxisClearance",
            E::SliverAxisClearance {
                loop_index: 0,
                segment_index: 3,
                source: diag(),
            },
        ),
        (
            "UnsupportedToroid",
            E::UnsupportedToroid {
                loop_index: 0,
                segment_index: 3,
            },
        ),
        (
            "NonManifoldAxisContact",
            E::NonManifoldAxisContact {
                loop_index: 0,
                vertex_index: 3,
            },
        ),
        ("MultipleAxisRuns", E::MultipleAxisRuns { loop_index: 0 }),
        ("HoleTouchesAxis", E::HoleTouchesAxis { loop_index: 1 }),
        (
            "VoidInsertion",
            E::VoidInsertion {
                loop_index: 1,
                source: topo::VoidInsertError::NotStrictlyContained {
                    shell: topo::ShellKey::default(),
                },
            },
        ),
        (
            "CosurfaceEscalated",
            E::CosurfaceEscalated {
                loop_index: 0,
                vertex_index: 3,
                source: diag(),
            },
        ),
        (
            "SliverJoin",
            E::SliverJoin {
                loop_index: 0,
                vertex_index: 3,
                source: diag(),
            },
        ),
        (
            "SliverRim",
            E::SliverRim {
                loop_index: 0,
                segment_index: 3,
                source: diag(),
            },
        ),
        ("CapPlane", E::CapPlane { source: newell() }),
        ("Op", E::Op { source: euler() }),
        ("Pcurve", E::Pcurve(pcurve())),
    ]
}

fn revolve() -> Vec<(String, NodeErrorKind)> {
    revolve_arms()
        .into_iter()
        .map(|(n, e)| row(&format!("Revolve/{n}"), NodeErrorKind::Revolve(e)))
        .collect()
}

fn tube() -> Vec<(String, NodeErrorKind)> {
    use payloads::*;
    use sweep::TubeError as E;
    let hollow = |p: &'static str| geom_core::Indeterminate {
        predicate: Some(p),
        ..diag()
    };
    [
        ("Band", E::Band(band_error())),
        ("DegenerateWindow", E::DegenerateWindow),
        ("FullRangeWindow", E::FullRangeWindow),
        ("NonpositiveWall", E::NonpositiveWall { eps: 1.0e-7 }),
        ("WallExceedsRadius", E::WallExceedsRadius { eps: 1.0e-7 }),
        ("WallGapCollapsed", E::WallGapCollapsed { eps: 1.0e-7 }),
        ("Escalated", E::Escalated { source: diag() }),
        (
            "Escalated(hollow)",
            E::Escalated {
                source: hollow("tube_wall_gap"),
            },
        ),
        ("Revolve", E::Revolve(sweep::RevolveError::DegenerateAxis)),
    ]
    .into_iter()
    .map(|(n, e)| row(&format!("Tube/{n}"), NodeErrorKind::Tube(Box::new(e))))
    .collect()
}

fn split() -> Vec<(String, NodeErrorKind)> {
    use payloads::*;
    use topo::{
        EdgeKey, EntityId, FaceKey, LoopKey, ShellKey, SplitError, SplitFinishError as F,
        SplitJoinError as J, SplitReduceError as R, VertexKey,
    };
    let (face, edge, vertex) = (FaceKey::default(), EdgeKey::default(), VertexKey::default());
    let reduce = [
        ("Band", R::Band(band_error())),
        (
            "CurvedBooleanUnsupported",
            R::CurvedBooleanUnsupported {
                face,
                kind: geom_brep::SurfaceKind::Nurbs,
            },
        ),
        ("CurvedEdgeUnsupported", R::CurvedEdgeUnsupported { edge }),
        (
            "CrossingEscalated",
            R::CrossingEscalated { edge, diag: diag() },
        ),
        (
            "TangencyUnsupported",
            R::TangencyUnsupported { face, vertex },
        ),
        ("ScaffoldingOperand", R::ScaffoldingOperand { edge }),
        (
            "SliverVertex",
            R::SliverVertex {
                vertex,
                diag: diag(),
            },
        ),
        (
            "SliverSector",
            R::SliverSector {
                vertex,
                face,
                diag: diag(),
            },
        ),
        (
            "NonFiniteSectorChord",
            R::NonFiniteSectorChord { vertex, face },
        ),
        (
            "UnderflowedSectorChord",
            R::UnderflowedSectorChord { vertex, face },
        ),
        ("ConsecutiveOnSectors", R::ConsecutiveOnSectors { vertex }),
        ("CorruptOperand", R::CorruptOperand { vertex }),
        (
            "CrossingInsertion",
            R::CrossingInsertion {
                edge,
                endpoints: (vertex, vertex),
                source: euler(),
            },
        ),
        ("Euler", R::Euler(euler())),
    ]
    .map(|(n, e)| (format!("Reduce/{n}"), SplitError::Reduce(e)));
    // The join is shared by the split and the Boolean, and each wraps it
    // in its own words and recourse, so both chains are rendered.
    let join_arms = || {
        [
            ("OrderEscalated", J::OrderEscalated { diag: diag() }),
            ("Escalated", J::Escalated { face, diag: diag() }),
            ("DegenerateSection", J::DegenerateSection { face }),
            (
                "RingHoming",
                J::RingHoming(topo::PointInLoopError::RayExhausted {
                    r#loop: LoopKey::default(),
                }),
            ),
            (
                "RingHomingAmbiguous",
                J::RingHomingAmbiguous {
                    ring: LoopKey::default(),
                },
            ),
            ("UnpairedLooseEnds", J::UnpairedLooseEnds { count: 3 }),
            ("SectionLoopMixed", J::SectionLoopMixed { face }),
            ("CutInvariant", J::CutInvariant { edge }),
            (
                "Corrupt",
                J::Corrupt {
                    entity: EntityId::Edge(edge),
                },
            ),
            ("Band", J::Band(band_error())),
            ("Euler", J::Euler(euler())),
            // The two section refusals a plane-inclusive pair can reach
            // (the join reads no other): a lane bug, and the conic
            // carrier's own refusal at a near-circular tilt.
            (
                "Section",
                J::Section {
                    face,
                    source: geom_brep::SectionError::WrongLane {
                        expected: "plane×cylinder",
                    },
                },
            ),
            (
                "Section(Carrier)",
                J::Section {
                    face,
                    source: geom_brep::SectionError::Carrier(geom::EllipseInvalid::CircularAxes),
                },
            ),
            (
                "SectionArcWindow",
                J::SectionArcWindow {
                    face,
                    case: topo::ArcWindowCase::NeitherContained,
                    band: band(),
                },
            ),
            (
                "SectionInvariant",
                J::SectionInvariant {
                    face,
                    what: "a section arc with no endpoint on the face's boundary",
                },
            ),
        ]
    };
    let join = join_arms().map(|(n, e)| (format!("Join/{n}"), SplitError::Join(e)));
    let boolean_join = join_arms().map(|(n, e)| {
        row(
            &format!("Boolean/Join/{n}"),
            NodeErrorKind::Boolean(topo::BooleanError::Join(e)),
        )
    });
    let shell = ShellKey::default();
    let finish = [
        ("NotSingleSolid", F::NotSingleSolid { count: 2 }),
        (
            "DegenerateSide",
            F::DegenerateSide {
                shell,
                side: topo::PlaneSide::Above,
            },
        ),
        ("TornComponent", F::TornComponent { shell }),
        (
            "UnclassifiableComponent",
            F::UnclassifiableComponent { shell },
        ),
        ("Corrupt", F::Corrupt),
        ("Euler", F::Euler(euler())),
        ("Band", F::Band(band_error())),
        (
            "DescribeEscalated",
            F::DescribeEscalated { edge, diag: diag() },
        ),
    ]
    .map(|(n, e)| (format!("Finish/{n}"), SplitError::Finish(e)));
    reduce
        .into_iter()
        .chain(join)
        .chain(finish)
        .chain([("Pcurves".to_owned(), SplitError::Pcurves(pcurve()))])
        .map(|(n, e)| row(&format!("Split/{n}"), NodeErrorKind::Split(e)))
        .chain(boolean_join)
        .collect()
}

fn transform() -> Vec<(String, NodeErrorKind)> {
    use payloads::*;
    use topo::TransformError as E;
    [
        ("Pcurve", E::Pcurve { source: pcurve() }),
        ("Band", E::Band(band_error())),
        (
            "Certify",
            E::Certify {
                edge: topo::EdgeKey::default(),
                source: geom_brep::CertifyError::IntervalNotForward,
            },
        ),
        (
            "NotRigid",
            E::NotRigid {
                check: "transform_rigid_col0_unit",
            },
        ),
        (
            "NonFiniteMap",
            E::NonFiniteMap {
                check: "transform_rigid_col0_unit",
            },
        ),
        (
            "NullScaffold",
            E::NullScaffold {
                edge: topo::EdgeKey::default(),
            },
        ),
        ("NurbsPlaceholder", E::NurbsPlaceholder),
        (
            "ApproxLaneUnsupported",
            E::ApproxLaneUnsupported { lane: "interval" },
        ),
        (
            "ApproxRecertify",
            E::ApproxRecertify {
                source: geom_brep::OffsetFitError::InvalidRequest {
                    d: 0.0,
                    tolerance: 1.0e-6,
                },
            },
        ),
        ("Corrupt", E::Corrupt { what: "face" }),
    ]
    .into_iter()
    .map(|(n, e)| row(&format!("Transform/{n}"), NodeErrorKind::Transform(e)))
    .collect()
}

fn skin_arms() -> Vec<(&'static str, sweep::SkinError)> {
    use sweep::SkinError as E;
    vec![
        ("TooFewSections", E::TooFewSections { have: 1, need: 2 }),
        (
            "SectionShapeMismatch",
            E::SectionShapeMismatch {
                section: 2,
                expected: 4,
                found: 5,
                what: "segments",
            },
        ),
        (
            "SectionProfile",
            E::SectionProfile {
                section: 2,
                source: profile::ProfileError::EmptyProfile,
            },
        ),
        (
            "DomainNotUnit",
            E::DomainNotUnit {
                section: 2,
                domain: (0.0, 2.0),
            },
        ),
        (
            "DegenerateSection",
            E::DegenerateSection {
                section: 2,
                what: "a zero-length segment",
            },
        ),
        (
            "BadDegree",
            E::BadDegree {
                degree: 3,
                sections: 2,
            },
        ),
        ("PathTangentReversal", E::PathTangentReversal { station: 4 }),
        (
            "Fit",
            E::Fit(geom::FitError::TooFewPoints { have: 1, need: 2 }),
        ),
        (
            "KnotAlgebra",
            E::KnotAlgebra(geom_core::spline::KnotAlgebraError::KnotNotPresent { u: 0.5 }),
        ),
        (
            "Structure",
            E::Structure(geom_core::SplineError::DomainInvalid { lo: 1.0, hi: 0.0 }),
        ),
    ]
}

fn skin() -> Vec<(String, NodeErrorKind)> {
    skin_arms()
        .into_iter()
        .map(|(n, e)| row(&format!("Skin/{n}"), NodeErrorKind::Skin(e)))
        .collect()
}

fn loft() -> Vec<(String, NodeErrorKind)> {
    use payloads::*;
    use sweep::LoftError as E;
    [
        ("Band", E::Band(band_error())),
        (
            "Skin",
            E::Skin(sweep::SkinError::TooFewSections { have: 1, need: 2 }),
        ),
        ("Euler", E::Euler(euler())),
        ("CapPlane", E::CapPlane(newell())),
        ("Pcurve", E::Pcurve(pcurve())),
        (
            "SeamStructure",
            E::SeamStructure {
                source: geom_core::SplineError::DomainInvalid { lo: 1.0, hi: 0.0 },
            },
        ),
        ("SectionStructure", E::SectionStructure),
        ("ReversedStacking", E::ReversedStacking { slab: 1 }),
        ("DegenerateStacking", E::DegenerateStacking { slab: 1 }),
        (
            "StackingEscalated",
            E::StackingEscalated {
                slab: 1,
                source: diag(),
            },
        ),
    ]
    .into_iter()
    .map(|(n, e)| row(&format!("Loft/{n}"), NodeErrorKind::Loft(e)))
    .collect()
}

fn blend() -> Vec<(String, NodeErrorKind)> {
    use geom_core::{Indeterminate, MarginDiag, Sign};
    use payloads::*;
    use sweep::blend::{BlendError as E, BlendKind, BlendSite, ClassifiedMargin, CornerConfig};
    use topo::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};
    let (face, edge, vertex) = (FaceKey::default(), EdgeKey::default(), VertexKey::default());
    let decided = |predicate, m: f64, sign| ClassifiedMargin {
        predicate,
        reading: MarginDiag::Value(m),
        band: band(),
        sign,
    };
    let escalated = |predicate| Indeterminate {
        predicate: Some(predicate),
        ..diag()
    };
    [
        ("Band", E::Band(band_error())),
        ("ChainNotConnected", E::ChainNotConnected { edge }),
        (
            "RadiusHeadroom",
            E::RadiusHeadroom {
                face,
                margin: decided("fillet3_radius_headroom", -1e-3, Sign::Negative),
                radius: 0.5,
            },
        ),
        (
            "FaceClearanceUncertified",
            E::FaceClearanceUncertified {
                face,
                margin: decided("fillet3_face_clearance", -1e-3, Sign::Negative),
                gap: MarginDiag::Value(0.2),
                cross_chain: false,
            },
        ),
        (
            "FaceClearanceUncertified(cross-chain)",
            E::FaceClearanceUncertified {
                face,
                margin: decided("fillet3_face_clearance", -1e-3, Sign::Negative),
                gap: MarginDiag::Value(0.2),
                cross_chain: true,
            },
        ),
        (
            "TangentialEdge",
            E::TangentialEdge {
                edge,
                margin: decided("fillet3_convexity_sign", 0.0, Sign::Zero),
            },
        ),
        (
            "SpineIrregular",
            E::SpineIrregular {
                margin: decided("fillet3_spine_regularity", -1e-3, Sign::Negative),
                radius: 0.5,
            },
        ),
        (
            "ChainNotG1",
            E::ChainNotG1 {
                vertex,
                margin: decided("fillet3_chain_g1", 1e-3, Sign::Positive),
                arm: MarginDiag::Value(0.5),
            },
        ),
        (
            "ConvexitySignFlip",
            E::ConvexitySignFlip {
                edge,
                margin: decided("fillet3_convexity_sign", -1e-3, Sign::Negative),
                chain: sweep::blend::Convexity::Convex,
            },
        ),
        (
            "UnsupportedCorner(policy)",
            E::UnsupportedCorner {
                vertex,
                corner: CornerConfig::NEdgeVertex { valence: 4 },
                policy: CornerConfig::NEdgeVertex { valence: 4 }.policy(),
            },
        ),
        (
            "UnsupportedCorner(seam)",
            E::UnsupportedCorner {
                vertex,
                corner: CornerConfig::SeamVertex,
                policy: CornerConfig::SeamVertex.policy(),
            },
        ),
        (
            "UnsupportedCorner(mixed)",
            E::UnsupportedCorner {
                vertex,
                corner: CornerConfig::MixedConvexity { convex: 1 },
                policy: CornerConfig::MixedConvexity { convex: 1 }.policy(),
            },
        ),
        (
            "SpineUnsupported",
            E::SpineUnsupported {
                edge,
                supports: "cone–torus",
            },
        ),
        (
            "ChamferArmUnsupported",
            E::ChamferArmUnsupported {
                edge,
                supports: "cylinder–sphere",
            },
        ),
        (
            "Escalated(radius)",
            E::Escalated {
                site: BlendSite::Link { edge },
                source: escalated("fillet3_radius_headroom"),
            },
        ),
        (
            "Escalated(clearance)",
            E::Escalated {
                site: BlendSite::Link { edge },
                source: escalated("fillet3_face_clearance"),
            },
        ),
        (
            "Escalated(chain)",
            E::Escalated {
                site: BlendSite::Joint { vertex },
                source: escalated("fillet3_chain_g1"),
            },
        ),
        (
            "Escalated(contact)",
            E::Escalated {
                site: BlendSite::Link { edge },
                source: escalated("tangent_second_order"),
            },
        ),
        (
            "Escalated(corner)",
            E::Escalated {
                site: BlendSite::Joint { vertex },
                source: escalated("fillet3_corner_independence"),
            },
        ),
        (
            "Escalated(ring)",
            E::Escalated {
                site: BlendSite::Chain,
                source: escalated("fillet3_ring_clearance"),
            },
        ),
        (
            "Escalated(unrouted)",
            E::Escalated {
                site: BlendSite::Chain,
                source: diag(),
            },
        ),
        ("RepeatedEdge", E::RepeatedEdge { edge }),
        ("NonpositiveSize", E::NonpositiveSize { size: 0.0 }),
        (
            "UnsupportedBody",
            E::UnsupportedBody {
                solids: 2,
                shells: 2,
            },
        ),
        (
            "UnsupportedChain",
            E::UnsupportedChain {
                edge,
                // One detail, as the feature tree draws it; every detail
                // the blend module raises is rendered by
                // `every_blend_detail_renders_within_the_budget`.
                detail: "a curved support does not carry exactly its own rim arc, as the band \
                         replacement needs",
            },
        ),
        (
            "UnsupportedRunOut",
            E::UnsupportedRunOut {
                at: EntityId::Vertex(vertex),
                detail: "a chain terminates at a trivalent vertex whose three edges are not all \
                         requested; run-outs at such corners are not implemented",
            },
        ),
        (
            "UnsupportedGeometry",
            E::UnsupportedGeometry {
                at: EntityId::Face(face),
                detail: "a split edge's stored window is not under one period, so the split \
                         parameter would alias by a turn",
            },
        ),
        (
            "BodyNotIntact",
            E::BodyNotIntact {
                at: EntityId::HalfEdge(HalfEdgeKey::default()),
                detail: "a twin half-edge the plan followed",
            },
        ),
        (
            "SurgeryInvariant",
            E::SurgeryInvariant {
                at: EntityId::Face(face),
                detail: "a trimline that the carve's own split placed",
            },
        ),
        (
            "RingClearance",
            E::RingClearance {
                face,
                margin: decided("fillet3_ring_clearance", -1e-3, Sign::Negative),
            },
        ),
        (
            "Certify",
            E::Certify {
                site: "blend face pcurves",
                source: pcurve(),
            },
        ),
        (
            "Op",
            E::Op {
                site: "strut mev",
                source: euler(),
            },
        ),
    ]
    .into_iter()
    .map(|(n, e)| {
        row(
            &format!("Blend/{n}"),
            NodeErrorKind::Blend {
                verb: BlendKind::Fillet,
                error: e,
            },
        )
    })
    .collect()
}

fn profile() -> Vec<(String, NodeErrorKind)> {
    use payloads::*;
    use profile::{ContactKind, EscalationSite, ProfileError as E, SegmentRef};
    let (a, b) = (
        SegmentRef {
            loop_index: 0,
            segment_index: 1,
        },
        SegmentRef {
            loop_index: 0,
            segment_index: 3,
        },
    );
    [
        ("Band", E::Band(band_error())),
        ("EmptyProfile", E::EmptyProfile),
        (
            "TooFewVertices",
            E::TooFewVertices {
                loop_index: 0,
                count: 2,
            },
        ),
        ("DegenerateSegment", E::DegenerateSegment(a)),
        ("NearFullArc", E::NearFullArc(a)),
        (
            "NonSimple",
            E::NonSimple {
                first: a,
                second: b,
                kind: ContactKind::Crossing,
            },
        ),
        (
            "TangentialContact",
            E::TangentialContact {
                first: a,
                second: b,
            },
        ),
        (
            "TangentJointOutOfRange",
            E::TangentJointOutOfRange {
                loop_index: 0,
                joint: 9,
                count: 4,
            },
        ),
        (
            "UndeclaredTangency",
            E::UndeclaredTangency {
                first: a,
                second: b,
                joint: 2,
                suggestion: "declare_tangent(loop=0, joint=2)".to_owned(),
            },
        ),
        (
            "TangencyContradicted",
            E::TangencyContradicted {
                first: a,
                second: b,
                joint: 2,
            },
        ),
        ("SliverLoop", E::SliverLoop { loop_index: 1 }),
        (
            "MultipleOuterLoops",
            E::MultipleOuterLoops {
                outer_loops: vec![0, 2],
            },
        ),
        (
            "NestingTooDeep",
            E::NestingTooDeep {
                loop_index: 2,
                depth: 2,
            },
        ),
        (
            "RayCastingExhausted",
            E::RayCastingExhausted {
                loop_index: 1,
                against_loop: 0,
            },
        ),
        (
            "Escalated(segment)",
            E::Escalated {
                site: EscalationSite::Segment(a),
                source: diag(),
            },
        ),
        (
            "Escalated(pair)",
            E::Escalated {
                site: EscalationSite::SegmentPair(a, b),
                source: diag(),
            },
        ),
        ("Structure", E::Structure(structure_refusal())),
    ]
    .into_iter()
    .map(|(n, e)| row(&format!("Profile/{n}"), NodeErrorKind::Profile(e)))
    .collect()
}

fn structure_refusal() -> profile::StructureRefusal {
    use profile::structure::{Decision, DecisionValue, StructureRefusalKind};
    profile::StructureRefusal {
        decision: Decision::SegmentShape {
            loop_: 0,
            segment: 2,
        },
        kind: StructureRefusalKind::Flipped {
            recorded: DecisionValue::Index(1),
            found: DecisionValue::Index(2),
        },
    }
}

fn structure_refusal_indeterminate() -> profile::StructureRefusal {
    use profile::structure::{Decision, StructureRefusalKind};
    profile::StructureRefusal {
        decision: Decision::Containment {
            loop_: 1,
            against: 0,
        },
        kind: StructureRefusalKind::Indeterminate(payloads::diag()),
    }
}

/// `ProfileReplay` looks through `ReplayError` (a step and a kind) to
/// the path refusal it carries; every `PathError` arm is a sketch edit
/// the person made.
fn profile_replay() -> Vec<(String, NodeErrorKind)> {
    use geom_core::Point2;
    use payloads::*;
    use profile::path::{PathError as P, PathNoCornerReason};
    use profile::{CornerReason, CornerRefusal, FilletLegCarrier, NoCornerReason};
    use profile::{FilletLeg, ReplayError, ReplayErrorKind, TipState};
    let path: Vec<(&str, P<f64>)> = vec![
        (
            "JunctionTangent",
            P::JunctionTangent {
                margin: 1e-12,
                arm: 0.5,
            },
        ),
        (
            "JunctionCusp",
            P::JunctionCusp {
                margin: 1e-12,
                arm: 0.5,
            },
        ),
        ("SeamTangent", P::SeamTangent { margin: 1e-12 }),
        (
            "SeamArrivalOffDirection",
            P::SeamArrivalOffDirection {
                margin: 1e-3,
                arm: 0.5,
            },
        ),
        (
            "SeamArrivalLeverTooShort",
            P::SeamArrivalLeverTooShort { arm: 1e-12 },
        ),
        (
            "ContinuationTargetOffRay",
            P::ContinuationTargetOffRay {
                across: 1e-3,
                along: 0.5,
            },
        ),
        (
            "NoCornerForFillet",
            P::NoCornerForFillet {
                reason: PathNoCornerReason::CarriersParallel,
                radius: 0.1,
            },
        ),
        (
            "NoCornerOfPair",
            P::NoCornerOfPair {
                radius: 0.1,
                corners: Vec::new(),
            },
        ),
        (
            "FilletOffsetLeverTooShort",
            P::FilletOffsetLeverTooShort {
                side: FilletLeg::Incoming,
                carrier_radius: 0.2,
                offset_radius: 0.3,
                least_lever: 1e-12,
                margin: 1e-3,
            },
        ),
        (
            "FilletArcFlattenedInStorage",
            P::FilletArcFlattenedInStorage {
                turn: 1e-9,
                radius: 0.1,
                arc_length: 1e-10,
                predicate: "fillet_arc_storage",
                margin: 1e-12,
            },
        ),
        (
            "FilletCarrierBelowSceneResolution",
            P::FilletCarrierBelowSceneResolution {
                turn: 0.5,
                radius: 1e-9,
                scale: 10.0,
                resolution: 1e-8,
                predicate: "fillet_scene_resolution",
                margin: 1e-12,
            },
        ),
        (
            "ArcLegOnOpenFillet",
            P::ArcLegOnOpenFillet { site: "line_to" },
        ),
        ("SeamRetrimsArcFirstSide", P::SeamRetrimsArcFirstSide),
        ("NonpositiveLeg", P::NonpositiveLeg { length: -0.1 }),
        (
            "NonpositiveFilletRadius",
            P::NonpositiveFilletRadius { radius: -0.1 },
        ),
        (
            "NonpositiveCircleRadius",
            P::NonpositiveCircleRadius { radius: -0.1 },
        ),
        ("DegenerateArcSpec", P::DegenerateArcSpec { value: 0.0 }),
        ("CircleSplitCount", P::CircleSplitCount { n: 1 }),
        (
            "PolygonTooFewVertices",
            P::PolygonTooFewVertices { given: 2 },
        ),
        ("ZeroDirection", P::ZeroDirection { dx: 0.0, dy: 0.0 }),
        (
            "NonFiniteDirection",
            P::NonFiniteDirection {
                dx: f64::INFINITY,
                dy: 0.0,
            },
        ),
        (
            "UnderflowedDirection",
            P::UnderflowedDirection {
                dx: 1e-300,
                dy: 0.0,
            },
        ),
        ("ArcViaCollinear", P::ArcViaCollinear { offset: 1e-12 }),
        ("DegenerateArcChord", P::DegenerateArcChord { chord: 1e-12 }),
        (
            "ArcCenterNotEquidistant",
            P::ArcCenterNotEquidistant {
                tip_radius: 0.5,
                end_radius: 0.6,
            },
        ),
        (
            "DegenerateArcCenter",
            P::DegenerateArcCenter { radius: 1e-12 },
        ),
        ("FarEndAnchorWithoutFillet", P::FarEndAnchorWithoutFillet),
        ("Escalated", P::Escalated { source: diag() }),
        (
            "NoCornerForFillet(disjoint)",
            P::NoCornerForFillet {
                reason: PathNoCornerReason::CarriersDoNotMeet,
                radius: 0.1,
            },
        ),
        (
            "NoCornerOfPair(swallows)",
            P::NoCornerOfPair {
                radius: 0.3,
                corners: vec![CornerRefusal {
                    at: Point2::new(0.25, -0.5),
                    reason: CornerReason::EnclosesLegCarrier {
                        side: Some(FilletLeg::Incoming),
                        carrier_radius: 0.2,
                        offset_radius: -0.1,
                        largest_tangent_radius: Some(0.15),
                    },
                }],
            },
        ),
        (
            "NoCornerOfPair(two)",
            P::NoCornerOfPair {
                radius: 0.3,
                corners: vec![
                    CornerRefusal {
                        at: Point2::new(0.25, -0.5),
                        reason: CornerReason::AnchorOutsideTrimmedExtent {
                            side: FilletLeg::Outgoing,
                            carrier: FilletLegCarrier::Line,
                            setback: 0.4,
                            available: 0.3,
                        },
                    },
                    CornerRefusal {
                        at: Point2::new(1.25, 0.5),
                        reason: CornerReason::NoTangentCircle(
                            NoCornerReason::OffsetCarriersDisjoint,
                        ),
                    },
                ],
            },
        ),
        (
            "NoCornerOfPair(two swallows, bounded)",
            P::NoCornerOfPair {
                radius: 0.312_345_678,
                corners: vec![
                    CornerRefusal {
                        at: Point2::new(-1.234_567, 0.987_654),
                        reason: CornerReason::EnclosesLegCarrier {
                            side: Some(FilletLeg::Incoming),
                            carrier_radius: 0.123_456_789,
                            offset_radius: -0.176_543_21,
                            largest_tangent_radius: Some(0.151_234_567),
                        },
                    },
                    CornerRefusal {
                        at: Point2::new(1.234_567, -0.987_654),
                        reason: CornerReason::EnclosesLegCarrier {
                            side: None,
                            carrier_radius: 0.123_456_789,
                            offset_radius: -0.176_543_21,
                            largest_tangent_radius: Some(0.141_234_567),
                        },
                    },
                ],
            },
        ),
        (
            "NoCornerOfPair(two swallows, unbounded)",
            P::NoCornerOfPair {
                radius: 0.312_345_678,
                corners: vec![
                    CornerRefusal {
                        at: Point2::new(-1.234_567, 0.987_654),
                        reason: CornerReason::EnclosesLegCarrier {
                            side: Some(FilletLeg::Outgoing),
                            carrier_radius: 0.123_456_789,
                            offset_radius: -0.176_543_21,
                            largest_tangent_radius: None,
                        },
                    },
                    CornerRefusal {
                        at: Point2::new(1.234_567, -0.987_654),
                        reason: CornerReason::EnclosesLegCarrier {
                            side: Some(FilletLeg::Incoming),
                            carrier_radius: 0.123_456_789,
                            offset_radius: -0.176_543_21,
                            largest_tangent_radius: None,
                        },
                    },
                ],
            },
        ),
        (
            "NoCornerOfPair(swallow and anchor)",
            P::NoCornerOfPair {
                radius: 0.312_345_678,
                corners: vec![
                    CornerRefusal {
                        at: Point2::new(1.234_567, -0.987_654),
                        reason: CornerReason::EnclosesLegCarrier {
                            side: None,
                            carrier_radius: 0.123_456_789,
                            offset_radius: -0.176_543_21,
                            largest_tangent_radius: Some(0.151_234_567),
                        },
                    },
                    CornerRefusal {
                        at: Point2::new(-1.234_567, 0.987_654),
                        reason: CornerReason::AnchorOutsideTrimmedExtent {
                            side: FilletLeg::Outgoing,
                            carrier: FilletLegCarrier::Line,
                            setback: 0.412_345_678,
                            available: 0.301_234_567,
                        },
                    },
                ],
            },
        ),
        ("Band", P::Band(band_error())),
        ("Structure", P::Structure(structure_refusal())),
        (
            "UnderdeterminedLeg",
            P::UnderdeterminedLeg {
                site: "line_at_angle",
            },
        ),
        (
            "OverdeterminedJunction",
            P::OverdeterminedJunction { site: "arc_to" },
        ),
    ];
    let routed = [
        "fillet_corner_turn",
        "fillet_corner_arm",
        "fillet_leg_reach",
        "fillet_leg_fit",
        "fillet_offset_lever",
        "fillet_enclosing_carrier",
        "path_continuation_target_offset",
        "path_seam_arrival_turn",
        "path_leg_length",
        "vertex_separation",
        "path_junction_turn",
    ];
    let path: Vec<(String, P<f64>)> = path
        .into_iter()
        .map(|(n, p)| (n.to_owned(), p))
        .chain(routed.into_iter().map(|predicate| {
            (
                format!("Escalated({predicate})"),
                P::Escalated {
                    source: geom_core::Indeterminate {
                        predicate: Some(predicate),
                        ..diag()
                    },
                },
            )
        }))
        .collect();
    let transition = ReplayError {
        step: 2,
        kind: ReplayErrorKind::Transition {
            state: TipState::Closed,
            verb: None,
        },
    };
    path.into_iter()
        .map(|(n, p)| {
            (
                format!("ProfileReplay/Path/{n}"),
                ReplayError {
                    step: 2,
                    kind: ReplayErrorKind::Path(p),
                },
            )
        })
        .chain([("ProfileReplay/Transition".to_owned(), transition)])
        .map(|(n, error)| (n, NodeErrorKind::ProfileReplay { loop_: 0, error }))
        .chain([
            row(
                "ProfileLaneReplay(Flipped)",
                NodeErrorKind::ProfileLaneReplay {
                    loop_: 0,
                    step: 2,
                    structure: Some(structure_refusal()),
                },
            ),
            row(
                "ProfileLaneReplay(Indeterminate)",
                NodeErrorKind::ProfileLaneReplay {
                    loop_: 0,
                    step: 2,
                    structure: Some(structure_refusal_indeterminate()),
                },
            ),
        ])
        .collect()
}

/// The editor-core payloads `NodeErrorKind` forwards: expressions,
/// analysis seeds, placement rules, naming, the name ladder.
fn editor_payloads() -> Vec<(String, NodeErrorKind)> {
    use editor_core::{
        Diagnosis, Dimension, EntityKind, EvalError, FlipSource, NamingError, ParamBoxError,
        ParamName, PlacementRuleFault, RecipeEditRef, ResolveError, RimShare, SeedError, SlotId,
        StableName, TieWitness,
    };
    use geom_core::Sign;
    use payloads::*;
    let name = || StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(3),
        path: Vec::new(),
    };
    let eval: Vec<(&str, EvalError)> = vec![
        (
            "UnknownParam",
            EvalError::UnknownParam(ParamName::new("width")),
        ),
        (
            "ParamDimensionMismatch",
            EvalError::ParamDimensionMismatch {
                name: ParamName::new("width"),
                expected: Dimension::Length,
                found: Dimension::Angle,
            },
        ),
        (
            "CountExprInContinuousEval",
            EvalError::CountExprInContinuousEval,
        ),
        (
            "ContinuousExprInCountEval",
            EvalError::ContinuousExprInCountEval {
                found: Dimension::Length,
            },
        ),
        ("CountOverflow", EvalError::CountOverflow),
        (
            "CountToScalarOutOfRange",
            EvalError::CountToScalarOutOfRange(1 << 60),
        ),
        ("NonFiniteResult", EvalError::NonFiniteResult),
    ];
    let param_box = [
        (
            "UnknownParam",
            ParamBoxError::UnknownParam {
                param: ParamName::new("width"),
            },
        ),
        (
            "AxisUnrepresentable",
            ParamBoxError::AxisUnrepresentable {
                param: ParamName::new("width"),
                lo: 1.0,
                hi: 0.0,
            },
        ),
    ];
    let seed = [
        (
            "UnknownParam",
            SeedError::UnknownParam {
                param: ParamName::new("width"),
            },
        ),
        (
            "CountParam",
            SeedError::CountParam {
                param: ParamName::new("n"),
            },
        ),
        (
            "TangentUnrepresentable",
            SeedError::TangentUnrepresentable {
                param: ParamName::new("width"),
            },
        ),
    ];
    let placement = [
        ("CountSpelling", PlacementRuleFault::CountSpelling),
        ("NoPlacements", PlacementRuleFault::NoPlacements),
        (
            "NonFiniteFrame",
            PlacementRuleFault::NonFiniteFrame { index: 2 },
        ),
        (
            "ImproperFrame",
            PlacementRuleFault::ImproperFrame {
                index: 2,
                determinant: -1.0,
            },
        ),
    ];
    let naming = [
        (
            "Duplicate",
            NamingError::Duplicate {
                name: Box::new(name()),
            },
        ),
        (
            "Unnamed",
            NamingError::Unnamed {
                kind: EntityKind::Edge,
                body: 0,
            },
        ),
        (
            "MissingUpstream",
            NamingError::MissingUpstream {
                node: RecipeNodeId(3),
            },
        ),
        (
            "Emission",
            NamingError::Emission {
                what: "a cap face with no profile loop behind it",
            },
        ),
        (
            "SplitLineage",
            NamingError::SplitLineage(topo::SplitLineageCycle {
                edge: topo::EdgeKey::default(),
            }),
        ),
        (
            "FragmentLineage",
            NamingError::FragmentLineage {
                face: topo::FaceKey::default(),
            },
        ),
        (
            "SeamVertexParentage",
            NamingError::SeamVertexParentage {
                vertex: topo::VertexKey::default(),
            },
        ),
        (
            "SharedRim",
            NamingError::SharedRim {
                node: RecipeNodeId(3),
                face: topo::FaceKey::default(),
                other: topo::FaceKey::default(),
                found: RimShare::Several,
            },
        ),
        ("Band", NamingError::Band(band_error())),
        (
            "Escalated",
            NamingError::Escalated {
                predicate: "side_of_plane",
                source: diag(),
            },
        ),
    ];
    let resolve = || {
        [
            (
                "Vanished",
                ResolveError::Vanished {
                    name: name(),
                    diagnosis: Diagnosis::PredicateFlip {
                        predicate: "side_of_plane",
                        from: Sign::Positive,
                        to: Sign::Negative,
                        source: FlipSource::VerdictLog,
                    },
                    last_good: None,
                },
            ),
            (
                "Ambiguous",
                ResolveError::Ambiguous {
                    name: name(),
                    candidates: vec![name(), name()],
                    tie: TieWitness {
                        node: RecipeNodeId(4),
                        at: name(),
                        width: 2,
                    },
                },
            ),
            (
                "NodeGone",
                ResolveError::NodeGone {
                    name: name(),
                    edit: RecipeEditRef::NodeDeleted {
                        node: RecipeNodeId(3),
                    },
                },
            ),
        ]
    };
    let mut rows: Vec<(String, NodeErrorKind)> = Vec::new();
    for (n, e) in eval {
        rows.push(row(
            &format!("Expr/{n}"),
            NodeErrorKind::Expr {
                slot: SlotId::Distance,
                source: e,
            },
        ));
    }
    for (n, source) in param_box {
        rows.push(row(
            &format!("ParamBox/{n}"),
            NodeErrorKind::ParamBox { source },
        ));
    }
    for (n, source) in seed {
        rows.push(row(&format!("Seed/{n}"), NodeErrorKind::Seed { source }));
    }
    for (n, e) in placement {
        rows.push(row(
            &format!("PlacementRule/{n}"),
            NodeErrorKind::PlacementRule(e),
        ));
    }
    for (n, e) in naming {
        rows.push(row(&format!("Naming/{n}"), NodeErrorKind::Naming(e)));
    }
    for (n, e) in [
        ("StaleKey", topo::ParamAttachError::StaleKey),
        (
            "FieldNotOnKind",
            topo::ParamAttachError::FieldNotOnKind {
                field: topo::SurfaceField::TorusMinorRadius,
            },
        ),
    ] {
        rows.push(row(
            &format!("ParamSourceAttach/{n}"),
            NodeErrorKind::ParamSourceAttach(e),
        ));
    }
    type Wrap = fn(Box<ResolveError>) -> NodeErrorKind;
    let wraps: [(&str, Wrap); 5] = [
        ("DeclareResolve", |error| NodeErrorKind::DeclareResolve {
            error,
        }),
        ("BlendSelectionResolve", |error| {
            NodeErrorKind::BlendSelectionResolve {
                verb: sweep::blend::BlendKind::Chamfer,
                error,
            }
        }),
        ("ShellOpenResolve", |error| {
            NodeErrorKind::ShellOpenResolve { error }
        }),
        ("FaceFrameResolve", |error| {
            NodeErrorKind::FaceFrameResolve { error }
        }),
        ("MeasureRefResolve", |error| {
            NodeErrorKind::MeasureRefResolve { error }
        }),
    ];
    for (wrap, build) in wraps {
        for (n, e) in resolve() {
            rows.push(row(&format!("{wrap}/{n}"), build(Box::new(e))));
        }
    }
    rows
}

fn stable(kind: editor_core::EntityKind, node: u64) -> editor_core::StableName {
    editor_core::StableName {
        kind,
        node: RecipeNodeId(node),
        path: Vec::new(),
    }
}

fn doc_ref() -> editor_core::DocRef {
    editor_core::DocRef {
        id: editor_core::DocumentId::derive("bracket"),
        pin: editor_core::ContentPin::of_bytes(b"bracket v3"),
    }
}

/// The contact, frame, witness, part and measure arms.
fn document_arms() -> Vec<(String, NodeErrorKind)> {
    use editor_core::clearance::ClearanceRefusal;
    use editor_core::{
        BifurcationKind, BranchMarginEvidence, ContactClass, DirectionRefusal, EntityKind,
        FaceName, FlushEvidence, FlushFinding, FlushRung, Implicated, InterrogateError,
        MeasureNodeFault, PartFault, ResolveFault, SitedRef, WitnessAge, WitnessBifurcation,
    };
    use geom_core::UnitVec3Error;
    use payloads::*;
    use topo::{DanglingRef, EntityId, FaceKey, ReadbackError};
    let face = || stable(EntityKind::Face, 3);
    let sited = |node| SitedRef {
        at: RecipeNodeId(node),
        name: stable(EntityKind::Face, node),
    };
    let finding = |relation| FlushFinding {
        pair: (sited(2), sited(3)),
        class: ContactClass::Rest,
        evidence: FlushEvidence {
            relation,
            rung: FlushRung::DecidedCoincident,
        },
    };
    let mut rows = vec![
        row(
            "UndeclaredContact(rest)",
            NodeErrorKind::UndeclaredContact {
                finding: Box::new(finding(topo::PlaneRelation::SameOpposite)),
                merged: Box::new((Vec::new(), Vec::new())),
                diag: diag(),
            },
        ),
        row(
            "UndeclaredContact(flush, merged)",
            NodeErrorKind::UndeclaredContact {
                finding: Box::new(finding(topo::PlaneRelation::SameOriented)),
                merged: Box::new((vec![sited(2), sited(4)], Vec::new())),
                diag: diag(),
            },
        ),
        row(
            "UndeclarableContact",
            NodeErrorKind::UndeclarableContact {
                row: Box::new(face()),
                diag: diag(),
            },
        ),
        row(
            "WitnessBifurcation",
            NodeErrorKind::WitnessBifurcation(WitnessBifurcation {
                kind: BifurcationKind::FoldProximity,
                margin: BranchMarginEvidence {
                    margin: 3.0e-10,
                    band_zero: 1.0e-9,
                    band_escalate: 1.0e-8,
                },
                implicated: vec![Implicated::Entity(face()), Implicated::Constraint(2)],
                witness_age: WitnessAge {
                    solved_under: Vec::new(),
                    at_solve: Vec::new(),
                },
            }),
        ),
        row(
            "CrossingUnverified",
            NodeErrorKind::CrossingUnverified {
                instance: RecipeNodeId(6),
                outer: Box::new(FaceName::new(face()).unwrap()),
                name: Box::new(stable(EntityKind::Edge, 3)),
            },
        ),
        row(
            "MeasureUnsupported",
            NodeErrorKind::MeasureUnsupported(editor_core::eval::measure::MeasureUnsupported {
                verb: "distance",
                a: "cylinder",
                b: "torus",
            }),
        ),
        row(
            "MeasureMalformed",
            NodeErrorKind::MeasureMalformed(MeasureNodeFault::RefIndexOutOfRange {
                verb: "min_clearance",
                index: 2,
                refs: 2,
            }),
        ),
        row(
            "MeasureClearanceRefused",
            NodeErrorKind::MeasureClearanceRefused(ClearanceRefusal::Unsupported {
                carrier: "a free-form face",
                face: FaceKey::default(),
            }),
        ),
    ];
    for (n, error) in [
        (
            "Dangling",
            ReadbackError::Dangling {
                what: DanglingRef::Entity(EntityId::Face(FaceKey::default())),
            },
        ),
        (
            "NoCanonicalFrame",
            ReadbackError::NoCanonicalFrame { carrier: "torus" },
        ),
        ("NoCarrier", ReadbackError::NoCarrier),
    ] {
        rows.push(row(
            &format!("FaceFrameReadback/{n}"),
            NodeErrorKind::FaceFrameReadback { error },
        ));
    }
    for (n, error) in [
        ("Degenerate", UnitVec3Error::Degenerate),
        ("NonFiniteLength", UnitVec3Error::NonFiniteLength),
        ("UnderflowedLength", UnitVec3Error::UnderflowedLength),
        ("Escalated", UnitVec3Error::Escalated(diag())),
    ] {
        rows.push(row(
            &format!("FrameDirection/{n}"),
            NodeErrorKind::FrameDirection {
                profile: RecipeNodeId(4),
                frame: RecipeNodeId(2),
                refusal: DirectionRefusal {
                    role: "frame normal",
                    error,
                },
            },
        ));
    }
    let interrogate = [
        (
            "NodeNotEvaluated",
            InterrogateError::NodeNotEvaluated {
                node: RecipeNodeId(3),
            },
        ),
        (
            "NodeFailed",
            InterrogateError::NodeFailed {
                node: RecipeNodeId(3),
            },
        ),
        (
            "NodePoisoned",
            InterrogateError::NodePoisoned {
                node: RecipeNodeId(3),
                through: RecipeNodeId(2),
            },
        ),
        ("NoSuchName", InterrogateError::NoSuchName),
        ("Ambiguous", InterrogateError::Ambiguous { candidates: 2 }),
        (
            "WrongKind",
            InterrogateError::WrongKind {
                wanted: EntityKind::Face,
                found: EntityKind::Edge,
            },
        ),
        ("WholeBody", InterrogateError::WholeBody),
        (
            "NoBodies",
            InterrogateError::NoBodies { payload: "profile" },
        ),
        ("NoSuchBody", InterrogateError::NoSuchBody { index: 2 }),
        (
            "Readback",
            InterrogateError::Readback(ReadbackError::NoCarrier),
        ),
    ];
    for (n, error) in interrogate {
        rows.push(row(
            &format!("MeasureRefUnreadable/{n}"),
            NodeErrorKind::MeasureRefUnreadable {
                name: Box::new(face()),
                error,
            },
        ));
    }
    // A part's root failure carries the part's own node refusal as its
    // message: the representative one is an extrude refusal.
    let inner = NodeErrorKind::Extrude(sweep::ExtrudeError::DegenerateExtrusion).to_string();
    let parts = [
        ("NoResolver", PartFault::NoResolver),
        (
            "Unresolved(PinMismatch)",
            PartFault::Unresolved {
                fault: ResolveFault::PinMismatch,
                message: "the document on disk has changed since the reference was pinned"
                    .to_owned(),
            },
        ),
        (
            "Unresolved(EpsilonSeam)",
            PartFault::Unresolved {
                fault: ResolveFault::EpsilonSeam,
                message: "the part records ε = 1e-6 and this process runs at ε = 1e-7".to_owned(),
            },
        ),
        (
            "Unresolved(Unresolved)",
            PartFault::Unresolved {
                fault: ResolveFault::Unresolved,
                message: "no document with this id is registered".to_owned(),
            },
        ),
        (
            "PartRootFailed(message)",
            PartFault::PartRootFailed {
                node: RecipeNodeId(7),
                cause: None,
                message: inner,
            },
        ),
        (
            "PartRootFailed(cause)",
            PartFault::PartRootFailed {
                node: RecipeNodeId(7),
                cause: Some(Box::new(PartFault::DepthExceeded)),
                message: String::new(),
            },
        ),
        (
            "PartProduct",
            PartFault::PartProduct {
                kind: editor_core::product::ProductErrorKind::NoBodyRoots,
                message: "the document declares no body root".to_owned(),
            },
        ),
        (
            "ReferenceCycle",
            PartFault::ReferenceCycle {
                cycle: vec![doc_ref(), doc_ref()],
            },
        ),
        ("DepthExceeded", PartFault::DepthExceeded),
    ];
    for (n, fault) in parts {
        rows.push(row(
            &format!("Part/{n}"),
            NodeErrorKind::Part {
                doc_ref: doc_ref(),
                fault,
            },
        ));
    }
    rows
}

fn mate() -> Vec<(String, NodeErrorKind)> {
    use editor_core::{Clash, DocumentId, LeverRefusal, MateFault as M, MateSide, Subgroup};
    use geom_core::{FrameError, FrameInput};
    use payloads::*;
    let n = RecipeNodeId;
    [
        (
            "PosesOfAnotherDocument",
            M::PosesOfAnotherDocument {
                expected: DocumentId::derive("a"),
                found: DocumentId::derive("b"),
            },
        ),
        (
            "Frame",
            M::Frame {
                mate: n(9),
                side: MateSide::A,
                error: FrameError::Degenerate {
                    input: FrameInput::Aim,
                    indeterminate: None,
                },
            },
        ),
        ("ClassNotAdmitted", M::ClassNotAdmitted { mate: n(9) }),
        (
            "TableLacks",
            M::TableLacks {
                mate: n(9),
                what: "a clocking rider on a planar rest",
            },
        ),
        (
            "Indeterminate",
            M::Indeterminate {
                mate: n(9),
                diag: Box::new(diag()),
            },
        ),
        (
            "Band",
            M::Band {
                error: band_error(),
            },
        ),
        (
            "Contradictory",
            M::Contradictory {
                held: n(8),
                added: n(9),
                predicate: "mate_coaxial",
                clash: Clash::Length { metres: 0.002 },
            },
        ),
        (
            "Under",
            M::Under {
                mate: n(9),
                parent: n(6),
                child: n(7),
                residual: Subgroup::Se3,
            },
        ),
        (
            "DanglingHead",
            M::DanglingHead {
                mate: n(9),
                side: MateSide::B,
                head: n(4),
            },
        ),
        (
            "PlacerRefused",
            M::PlacerRefused {
                mate: n(9),
                side: MateSide::B,
                placer: n(4),
                error: NodeErrorKind::EmptyOperand { input: n(3) }.into(),
            },
        ),
        (
            "PartSelectsAnotherCopy",
            M::PartSelectsAnotherCopy {
                mate: n(9),
                side: MateSide::A,
                part: n(6),
                named: 2,
                selected: 5,
            },
        ),
        (
            "SelfMate",
            M::SelfMate {
                mate: n(9),
                instance: n(6),
            },
        ),
        (
            "Unleverable",
            M::Unleverable {
                mate: n(9),
                refusal: LeverRefusal::NoExtent {
                    instance: n(6),
                    part: doc_ref(),
                },
            },
        ),
    ]
    .into_iter()
    .map(|(name, fault)| {
        row(
            &format!("Mate/{name}"),
            NodeErrorKind::Mate(Box::new(fault)),
        )
    })
    .collect()
}

fn shell() -> Vec<(String, NodeErrorKind)> {
    use payloads::*;
    use topo::{EntityId, FaceKey, ReplaceFaceError, ShellError as S, ShellKey, SolidKey};
    let (face, other, shell) = (FaceKey::default(), FaceKey::default(), ShellKey::default());
    // `Corrupt`, the one `ReplaceFaceError` arm that names no key: the
    // wrapper's own sentence names none either, so a key on this row
    // would be the wrapper's.
    let replace = || Box::new(ReplaceFaceError::<f64>::Corrupt);
    [
        (
            "Band",
            S::Band {
                error: band_error(),
            },
        ),
        ("Thickness", S::Thickness { thickness: -0.1 }),
        ("NoSolid", S::NoSolid),
        (
            "Roles",
            S::Roles {
                error: topo::ShellClassifyError::ZeroVolume { shell },
            },
        ),
        (
            "OperandOuterShells",
            S::OperandOuterShells {
                solid: SolidKey::default(),
                outer: 2,
            },
        ),
        (
            "Partition",
            S::Partition {
                shell,
                error: euler(),
            },
        ),
        (
            "WallClearance",
            S::WallClearance {
                face,
                other,
                gap: 0.001,
                needed: 0.002,
            },
        ),
        ("ChartSpansSolids", S::ChartSpansSolids { face, other }),
        ("ChartSenseMixed", S::ChartSenseMixed { face, other }),
        (
            "Face",
            S::Face {
                face,
                error: replace(),
            },
        ),
        ("OpenFaceStale", S::OpenFaceStale { face }),
        ("OpenFaceRepeated", S::OpenFaceRepeated { face }),
        ("OpenFacesExhaustShell", S::OpenFacesExhaustShell { shell }),
        (
            "OpenFacesDisconnect",
            S::OpenFacesDisconnect {
                shell,
                components: 2,
            },
        ),
        (
            "OpenFaceRingUnsupported",
            S::OpenFaceRingUnsupported {
                face,
                kind: geom_brep::SurfaceKind::Torus,
            },
        ),
        (
            "OpenFaceChartPartial",
            S::OpenFaceChartPartial { face, other },
        ),
        (
            "Lift",
            S::Lift {
                face,
                error: replace(),
            },
        ),
        (
            "Insert",
            S::Insert {
                error: topo::VoidInsertError::NotStrictlyContained { shell },
            },
        ),
        (
            "OpenFaceRimNotExpressible",
            S::OpenFaceRimNotExpressible {
                face,
                what: "a rim on a cone that is not a circle",
            },
        ),
        (
            "Rim",
            S::Rim {
                face,
                error: euler(),
            },
        ),
        ("Escalated", S::Escalated { source: diag() }),
        (
            "Corrupt",
            S::Corrupt {
                key: EntityId::Face(face),
            },
        ),
        ("Pcurve", S::Pcurve { source: pcurve() }),
        (
            "NotValid",
            S::NotValid {
                errors: vec![topo::ValidationError::ShellDisconnected {
                    shell,
                    components: 2,
                }],
            },
        ),
    ]
    .into_iter()
    .map(|(n, e)| row(&format!("Shell/{n}"), NodeErrorKind::Shell(Box::new(e))))
    .collect()
}

/// The four arms that say what a designation turned out to be carry a
/// `Found` only the entity door can mint, so they are raised through
/// real documents: a square prism's face, edge and vertex designated
/// where another kind is wanted.
fn found_arms() -> Vec<(String, NodeErrorKind)> {
    use crate::fixture::{self, ang, fname, insert, len, on_frame, square, wall};
    use editor_core::measure::{MeasureExpr, MeasurePrimitive};
    use editor_core::{
        CancelToken, CapEnd, Datum, EvalOptions, Node, NodeResult, ProfileDoc, ProfileVertexRef,
        SitedRef, evaluate,
    };
    use geom_core::Tol;
    let doc = ProfileDoc::empty_derived("refusal_concision_chains", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 1.0)],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let face = fname(body, wall(&doc, body, 2));
    let edge = fixture::prism_edges(&doc, body, 4).remove(2);
    let vertex = fixture::cap_vertex(
        body,
        CapEnd::End,
        crate::fixture::vpiece(&doc, body, 0, 0),
    );
    let (doc, shell) = insert(doc, Node::shell(body, len(0.1), vec![edge.clone()]));
    let (doc, fillet) = insert(doc, Node::fillet(body, len(0.1), vec![face.clone()]));
    let (doc, frame) = insert(
        doc,
        Node::Datum(Datum::FaceFrame {
            at: body,
            face: edge,
            spin: ang(0.0),
        }),
    );
    let (doc, measure) = insert(
        doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::MinClearance { a: 0, b: 1 }),
            vec![SitedRef::at_mint(vertex), SitedRef::at_mint(face)],
        )
        .expect("both indices in range"),
    );
    let mut ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    [
        ("ShellOpenKind", shell),
        ("BlendSelectionKind", fillet),
        ("FaceFrameKind", frame),
        ("MeasureSelectionKind", measure),
    ]
    .into_iter()
    .map(|(n, node)| match ev.nodes.remove(&node) {
        Some(NodeResult::Failed(e)) => row(n, e.kind),
        other => panic!("{n}: the designation must refuse; got {other:?}"),
    })
    .collect()
}

/// **Every checks-window finding fits the window it is listed in.** The
/// checks window draws each finding's `Display` verbatim beside its
/// root's button, so each `CheckEvidence` arm is rendered as the window
/// draws it, on a representative payload, and held to the budget. The
/// separation arm forwards a Boolean refusal's own sentence; it is
/// rendered over every containment refusal the separation read can
/// raise.
#[test]
fn every_check_finding_renders_within_the_budget() {
    let rows: Vec<(String, String)> = check_findings()
        .into_iter()
        .map(|(name, finding)| (format!("Check/{name}"), finding.to_string()))
        .collect();
    let problems = over_budget(&rows);
    assert!(problems.is_empty(), "{}", problems.join("\n"));
    // A grazing ray is an ill-conditioned margin wherever it arrives
    // from — the solid's own boundary or one of its loops — so each
    // path states the recourse, once.
    for (name, text) in &rows {
        if name.contains("RayExhausted") {
            assert_eq!(text.matches("Recourse:").count(), 1, "{name}: {text}");
        }
    }
}

fn check_findings() -> Vec<(String, editor_core::CheckFinding)> {
    use editor_core::{CheckEvidence as E, CheckFinding, CheckId};
    use payloads::*;
    use topo::{
        BooleanError, CoherenceCondition, CoherenceFinding, EdgeKey, FaceKey, LoopKey,
        PointInSolidError, ShellClassifyError, ShellKey, StructureRead, Unexaminable, Unexamined,
        VertexKey,
    };
    let shell = ShellKey::default();
    let finding = |check, evidence| CheckFinding {
        check,
        root: RecipeNodeId(4),
        output_ix: 0,
        evidence,
    };
    let coherence = |condition| CoherenceFinding {
        face: FaceKey::default(),
        r#loop: LoopKey::default(),
        edge: EdgeKey::default(),
        condition,
        gap: 1.0e-10,
        lever: 100.0,
        metres: 1.0e-8,
        eps: 1.0e-9,
    };
    let unexamined = |why| Unexamined {
        face: FaceKey::default(),
        r#loop: LoopKey::default(),
        why,
    };
    let mut rows = vec![
        (
            "Connectedness",
            finding(
                CheckId::Connectedness,
                E::Connectedness {
                    actual: 2,
                    expected: 1,
                },
            ),
        ),
        (
            "Escalated",
            finding(
                CheckId::Connectedness,
                E::Escalated {
                    source: ShellClassifyError::Escalated {
                        shell,
                        source: diag(),
                    },
                },
            ),
        ),
        (
            "Escalated(zero volume)",
            finding(
                CheckId::Connectedness,
                E::Escalated {
                    source: ShellClassifyError::ZeroVolume { shell },
                },
            ),
        ),
        (
            "Unsupported",
            finding(
                CheckId::Connectedness,
                E::Unsupported {
                    source: ShellClassifyError::Props {
                        shell,
                        source: topo::MassPropsError::RingOnCurvedFace {
                            face: FaceKey::default(),
                        },
                    },
                },
            ),
        ),
        (
            "StaleExpectation",
            finding(CheckId::Connectedness, E::StaleExpectation { expected: 2 }),
        ),
        (
            "NotSeparated",
            finding(
                CheckId::Separation,
                E::NotSeparated {
                    other_root: RecipeNodeId(7),
                    other_output: 0,
                },
            ),
        ),
        (
            "ChartCoherence(meridian closure)",
            finding(
                CheckId::ChartCoherence,
                E::ChartCoherence {
                    finding: coherence(CoherenceCondition::MeridianClosure {
                        vertex: VertexKey::default(),
                    }),
                },
            ),
        ),
        (
            "ChartCoherence(rim)",
            finding(
                CheckId::ChartCoherence,
                E::ChartCoherence {
                    finding: coherence(CoherenceCondition::RimContinuation {
                        opens: EdgeKey::default(),
                    }),
                },
            ),
        ),
        (
            "ChartCoherenceUnexamined(corrupt)",
            finding(
                CheckId::ChartCoherence,
                E::ChartCoherenceUnexamined {
                    unexamined: unexamined(Unexaminable::Corrupt {
                        at: StructureRead::Cycle,
                    }),
                },
            ),
        ),
        (
            "ChartCoherenceUnexamined(scaffold)",
            finding(
                CheckId::ChartCoherence,
                E::ChartCoherenceUnexamined {
                    unexamined: unexamined(Unexaminable::NullScaffoldEdge {
                        edge: EdgeKey::default(),
                    }),
                },
            ),
        ),
        (
            "ChartCoherenceUnexamined(non-iso)",
            finding(
                CheckId::ChartCoherence,
                E::ChartCoherenceUnexamined {
                    unexamined: unexamined(Unexaminable::NonIsoCarrier {
                        edge: EdgeKey::default(),
                    }),
                },
            ),
        ),
        (
            "ChartCoherenceUnavailable",
            finding(CheckId::ChartCoherence, E::ChartCoherenceUnavailable),
        ),
    ]
    .into_iter()
    .map(|(n, f)| (n.to_owned(), f))
    .collect::<Vec<_>>();
    let face = FaceKey::default();
    let separation_reasons = [
        (
            "Escalated",
            PointInSolidError::Escalated { face, diag: diag() },
        ),
        ("RayExhausted", PointInSolidError::RayExhausted),
        (
            "Loop(RayExhausted)",
            PointInSolidError::Loop(topo::PointInLoopError::RayExhausted {
                r#loop: topo::LoopKey::default(),
            }),
        ),
        (
            "Loop(Escalated)",
            PointInSolidError::Loop(topo::PointInLoopError::Escalated {
                r#loop: topo::LoopKey::default(),
                diag: diag(),
            }),
        ),
        ("ZeroVolumeBody", PointInSolidError::ZeroVolumeBody),
        ("CorruptFace", PointInSolidError::CorruptFace { face }),
        (
            "KindUnsupported",
            PointInSolidError::KindUnsupported {
                face,
                kind: geom_brep::SurfaceKind::Nurbs,
            },
        ),
        ("VolumeUncertified", PointInSolidError::VolumeUncertified),
        (
            "PartialSphereFace",
            PointInSolidError::PartialSphereFace { face },
        ),
        (
            "PartialConeFace",
            PointInSolidError::PartialConeFace { face },
        ),
        (
            "PartialTorusFace",
            PointInSolidError::PartialTorusFace { face },
        ),
        (
            "NoSuchSolid",
            PointInSolidError::NoSuchSolid {
                solid: topo::SolidKey::default(),
            },
        ),
        (
            "SurfaceSharedOutsideSolid",
            PointInSolidError::SurfaceSharedOutsideSolid { face, other: face },
        ),
    ];
    for (n, e) in separation_reasons {
        let source = BooleanError::Containment(e);
        rows.push((
            format!("SeparationUnavailable/Containment({n})"),
            finding(
                CheckId::Separation,
                E::SeparationUnavailable {
                    kind: source.kind(),
                    reason: source.to_string(),
                },
            ),
        ));
    }
    rows
}

/// **Every detail a blend refusal is raised with fits its chain.** The
/// `UnsupportedChain`, `UnsupportedRunOut`, `UnsupportedGeometry` and
/// `BodyNotIntact` arms each render a raise site's `detail` in front of
/// their recourse, and the rows above render one representative detail
/// apiece. So this reads every detail the blend module raises those arms
/// with — the second argument of each `unbuilt_chain`,
/// `unbuilt_run_out`, `unbuilt_geometry` and `not_intact` call in
/// `sweep/src/blend`, a literal or a `const` resolved in the same tree —
/// and renders each through the feature tree's chain, under both verbs.
/// A raise site added later is read the day it is written.
#[test]
fn every_blend_detail_renders_within_the_budget() {
    use sweep::blend::{BlendError as E, BlendKind};
    use topo::{EdgeKey, EntityId, FaceKey};
    let details = blend_details();
    let mut rows = Vec::new();
    for (helper, site, detail) in &details {
        let detail: &'static str = Box::leak(detail.clone().into_boxed_str());
        let at = EntityId::Face(FaceKey::default());
        let error = || match *helper {
            "unbuilt_chain" => E::UnsupportedChain {
                edge: EdgeKey::default(),
                detail,
            },
            "unbuilt_run_out" => E::UnsupportedRunOut { at, detail },
            "unbuilt_geometry" => E::UnsupportedGeometry { at, detail },
            "not_intact" => E::BodyNotIntact { at, detail },
            other => panic!("no arm for {other}"),
        };
        for verb in [BlendKind::Fillet, BlendKind::Chamfer] {
            rows.push((
                format!("Blend/{helper}@{site}"),
                as_the_viewer_shows_it(NodeErrorKind::Blend {
                    verb,
                    error: error(),
                }),
            ));
        }
    }
    // A reader that found nothing would pass vacuously.
    for helper in [
        "unbuilt_chain",
        "unbuilt_run_out",
        "unbuilt_geometry",
        "not_intact",
    ] {
        assert!(
            details.iter().any(|(h, _, _)| *h == helper),
            "no `{helper}` detail was read from sweep/src/blend"
        );
    }
    let keyed: Vec<(String, String)> = rows
        .into_iter()
        .map(|(n, t)| {
            // `BodyNotIntact` names the entity that did not resolve: a
            // corrupt body, whose report needs the key.
            let n = if n.starts_with("Blend/not_intact@") {
                "Blend/BodyNotIntact".to_owned() + &n["Blend/not_intact".len()..]
            } else {
                n
            };
            (n, t)
        })
        .collect();
    let problems: Vec<String> = keyed
        .iter()
        .flat_map(|(name, text)| {
            eprintln!("MEASURE {} {name}: {text}", text.split_whitespace().count());
            test_utils::refusal::problems(
                name,
                text,
                ALLOWED_LABELS,
                name.starts_with("Blend/BodyNotIntact@"),
            )
        })
        .collect();
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// Every `(helper, file:line, detail)` the blend module raises a
/// detail-carrying refusal with, read from its source.
fn blend_details() -> Vec<(&'static str, String, String)> {
    use test_utils::source::{
        balanced_end, boundary_before, code_and_literals, code_only, crate_dir, line, rust_sources,
        top_level_split,
    };
    let dir = crate_dir(env!("CARGO_MANIFEST_DIR")).join("../sweep/src/blend");
    // Each file in two views, blanked in place so their bytes line up:
    // the code alone, where every bracket and comma is real, locates a
    // call and its arguments; the code with its literals reads them.
    let files: Vec<(std::path::PathBuf, String, String)> = rust_sources(&dir)
        .into_iter()
        .map(|p| {
            let text = std::fs::read_to_string(&p).expect("a blend source reads");
            let code = test_utils::source::blanked(code_only, "a blend source", &text);
            let view = test_utils::source::blanked(code_and_literals, "a blend source", &text);
            (p, code, view)
        })
        .collect();
    let constant = |name: &str| -> String {
        let decl = format!("const {name}: &str =");
        let found: Vec<String> = files
            .iter()
            .flat_map(|(_, code, view)| {
                test_utils::source::initializers(code, &decl)
                    .into_iter()
                    .map(|r| view[r].to_owned())
            })
            .collect();
        assert!(
            found.len() == 1,
            "`{name}` is declared {} times, not once",
            found.len()
        );
        decode(&found[0])
    };
    let mut out = Vec::new();
    for (path, code, view) in &files {
        for helper in [
            "unbuilt_chain",
            "unbuilt_run_out",
            "unbuilt_geometry",
            "not_intact",
        ] {
            let call = format!("{helper}(");
            for (at, _) in code.match_indices(&call) {
                // A call, not the definition.
                if code[..at].ends_with("fn ") || !boundary_before(code, at) {
                    continue;
                }
                let open = at + helper.len();
                let close = balanced_end(code, open).expect("the call closes");
                let parts = top_level_split(&code[open + 1..close], ',');
                let detail = parts[1].start + open + 1..parts[1].end + open + 1;
                let arg = view[detail].trim();
                let site = format!(
                    "{}:{}",
                    path.file_name().unwrap().to_string_lossy(),
                    line(code, at)
                );
                let detail = if arg.starts_with('"') {
                    decode(arg)
                } else {
                    constant(arg)
                };
                out.push((helper, site, detail));
            }
        }
    }
    out
}

/// A plain Rust string literal's value: `\`-newline continuations and
/// `\"` decoded, anything else refused rather than guessed.
fn decode(literal: &str) -> String {
    let inner = test_utils::source::plain_string_literal(literal)
        .unwrap_or_else(|| panic!("not a plain string literal: {literal}"));
    let mut out = String::new();
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\n') => {
                while chars.peek().is_some_and(|c| c.is_whitespace()) {
                    chars.next();
                }
            }
            Some('"') => out.push('"'),
            other => panic!("an escape this reader does not decode: \\{other:?} in {literal}"),
        }
    }
    out
}
