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

use editor_core::NodeStanding;
use editor_core::{NodeError, NodeErrorKind, RecipeNodeId};
use test_utils::refusal::Admission;
use test_utils::refusal::tagged;

/// A `NodeErrorKind` as the feature tree's fault line draws it.
pub(crate) fn as_the_viewer_shows_it(kind: NodeErrorKind) -> String {
    NodeError {
        node: RecipeNodeId(tagged(5)),
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
    "Split/Finish/NestingContradiction",
    "Split/Pcurves",
    "Transform/Pcurve",
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
];

/// The clause labels a surface legitimately opens with that read, by
/// shape, like a stage prefix — a clause with no word only a sentence
/// has — each on the row namespace whose surface writes it. A label
/// here is English the person reads, not a pipeline stage.
pub(crate) const ALLOWED_LABELS: &[(&str, &str)] = &[
    // The checks window's finding labels (`check separation: root 4
    // output 0: …`): the check the person ran, named as the menu names
    // it, and the root it ran on.
    ("Check/", "check separation"),
    ("Check/", "check connectedness"),
    ("Check/", "check chart-coherence"),
    ("Check/", "root 000000000004 output 0"),
    // The mate solve names the mate it refused (`mate 9: …`).
    ("Mate/", "mate 000000000009"),
    // A pair's corner list names each corner it could not fillet.
    ("ProfileReplay/Path/NoCornerOfPair(", "at corner"),
];

/// The rows whose stage prefix is filed with its owner, each with the
/// one prefix it may still carry. An exact row id and an exact label:
/// a new prefix on the same row, or the same prefix on another row, is
/// still red.
pub(crate) const FILED: &[(&str, &str)] = &[
    // `topo/src/replace_face.rs`, SHELL's:
    // work/shell/replace-face-refusals-open-with-a-stage-prefix-and-name-keys.md
    ("Shell/Face", "replace_face_offset"),
    ("Shell/Lift", "replace_face_offset"),
    // `geom/src/curves.rs` (#2861): `EllipseInvalid` opens with
    // "ellipse construction:" and offers "declare" under a split:
    // work/chrome/the-refusal-shape-guard-has-blind-spots.md
    ("Split/Join/Section(Carrier)", "ellipse construction"),
    ("Boolean/Join/Section(Carrier)", "ellipse construction"),
    // A stage for a subject (`<gerund> … refused:`), each on its
    // owner's row:
    // work/shell/shell-refusals-short-of-the-shape-guard.md
    ("Shell/Face", "offsetting a face inward refused"),
    (
        "Shell/Lift",
        "lifting the rim back onto a designated open face refused",
    ),
    ("Shell/Insert", "inserting the cavity refused"),
    // work/hone/reach-refusals-short-of-the-shape-guard.md
    (
        "Split/Reduce/CrossingInsertion",
        "inserting the plane crossing on edge EdgeKey    refused",
    ),
    // work/carve/carve-refusals-short-of-the-shape-guard.md
    (
        "Revolve/VoidInsertion",
        "inserting the cavity of hole loop 1 refused",
    ),
    // work/paths/paths-refusals-short-of-the-shape-guard.md
    (
        "ProfileReplay/Path/CircleSplitCount",
        "circle_split needs between 2 and 4294967295 arcs",
    ),
    ("ProfileReplay/Path/SeamRetrimsArcFirstSide", "p"),
    (
        "Profile/TangentialContact",
        "tangential contact between loop 0 segment 1 and loop 0 segment 3",
    ),
    (
        "Profile/RayCastingExhausted",
        "containment of loop 1 in loop 0",
    ),
    // work/hone/reach-refusals-short-of-the-shape-guard.md
    (
        "Split/Join/SectionInvariant",
        "curved-section invariant at face FaceKey",
    ),
    (
        "Boolean/Join/SectionInvariant",
        "curved-section invariant at face FaceKey",
    ),
    // work/carve/carve-refusals-short-of-the-shape-guard.md
    ("Blend/SurgeryInvariant", "at face FaceKey"),
    // work/issues/unowned-viewer-refusals-short-of-the-shape-guard.md
    (
        "Naming/SplitLineage",
        "split lineage of edge EdgeKey  cycles",
    ),
    (
        "Naming/FragmentLineage",
        "fragment lineage of face FaceKey  cycles",
    ),
];

/// Row namespaces admitted a prefix by namespace rather than row by
/// row, each with the one label its rows may carry, and — where the
/// flag says so — leave to name a key. Any other prefix, a `Debug`
/// struct, or a row outside the namespace is still red.
pub(crate) const FILED_NAMESPACES: &[(&str, &str, bool)] = &[
    // A generated family, one row per `OffsetFitError` sample
    // ([`offset_fit_routes`]), whose wrapper sits in
    // `topo/src/replace_face.rs`, SHELL's:
    // work/shell/replace-face-refusals-open-with-a-stage-prefix-and-name-keys.md
    ("Shell/Face/Fit/", "replace_face_offset", true),
    ("Shell/Face/Fit/", "offsetting a face inward refused", false),
];

/// The split rows that may still offer "declare", which a split has no
/// door for, each filed with its owner (the note above `FILED`'s
/// `EllipseInvalid` entries).
pub(crate) const FILED_DECLARE: &[&str] = &["Split/Join/Section(Carrier)"];

/// The rows that render a key or a hex id, by exact row id and the
/// exact span, each filed with its owner.
pub(crate) const ADMISSIONS: &[Admission<'static>] = &[
    // `ClearanceRefusal::payload` (`editor-core/src/clearance.rs`)
    // renders the faces of `Unsupported` and `PoisonEnclosure`, the two
    // arms `min_separation` refuses with that carry evidence, as
    // `FaceKey` `Debug`, and this arm prints it.
    Admission {
        row: "MeasureClearanceRefused",
        span: "FaceKey(null)",
        filed: "work/props/props-refusal-prose-outgrows-the-viewer.md",
    },
    // A document named by its hex id: the reference loop.
    Admission {
        row: "Part/ReferenceCycle",
        span: "11c1eee0e02516b19e263d060a3c9f80@9515831d455a",
        filed: "work/doctail/part-refusals-name-documents-by-hex-id.md",
    },
];

/// The rows that state no recourse — no `Recourse:`, no "There is no way
/// through", and none of the shared unlabelled repairs — by exact row
/// id, grouped under the row that files them with their owner.
pub(crate) const FILED_NO_RECOURSE: &[&str] = &[
    // work/wire/wire-refusals-short-of-the-shape-guard.md
    "AssertionDimension",
    "AxisInDifferentPlane",
    "BlendSelectionEmpty",
    "BlendSelectionKind",
    "BlendSelectionResolve/Ambiguous",
    "BlendSelectionResolve/NodeGone",
    "BlendSelectionResolve/Vanished",
    "CrossingUnverified",
    "CurvedSolidFrontier",
    "DeclareResolve/Ambiguous",
    "DeclareResolve/NodeGone",
    "DeclareResolve/Vanished",
    "DeclareSiteNotAnOperand",
    "DeclareUnsupportedPair",
    "DegenerateDirection",
    "DerivedFrameSection",
    "EmptyHalf",
    "EmptyOperand",
    "Expr",
    "Expr/ContinuousExprInCountEval",
    "Expr/CountExprInContinuousEval",
    "Expr/CountOverflow",
    "Expr/CountToScalarOutOfRange",
    "Expr/NonFiniteResult",
    "Expr/ParamDimensionMismatch",
    "Expr/UnknownParam",
    "FaceFrameKind",
    "FaceFrameNotPlanar",
    "FaceFrameReadback/Dangling",
    "FaceFrameReadback/NoCanonicalFrame",
    "FaceFrameReadback/NoCarrier",
    "FaceFrameResolve/Ambiguous",
    "FaceFrameResolve/NodeGone",
    "FaceFrameResolve/Vanished",
    "FrameDirection/Degenerate",
    "InstanceOutOfRange",
    "MeasureClearanceRefused",
    "MeasureMalformed",
    "MeasureNonFinite",
    "MeasureNotParallel",
    "MeasureRefResolve/Ambiguous",
    "MeasureRefResolve/NodeGone",
    "MeasureRefResolve/Vanished",
    "MeasureRefUnreadable/Ambiguous",
    "MeasureRefUnreadable/NoBodies",
    "MeasureRefUnreadable/NoSuchBody",
    "MeasureRefUnreadable/NoSuchName",
    "MeasureRefUnreadable/NodeFailed",
    "MeasureRefUnreadable/NodeNotEvaluated",
    "MeasureRefUnreadable/NodePoisoned",
    "MeasureRefUnreadable/Readback",
    "MeasureRefUnreadable/WholeBody",
    "MeasureRefUnreadable/WrongKind",
    "MeasureSelectionKind",
    "MeasureUnsupported",
    "MissingInput",
    "MissingSlot",
    "Naming/Duplicate",
    "Naming/Emission",
    "Naming/FragmentLineage",
    "Naming/MissingUpstream",
    "Naming/SeamVertexParentage",
    "Naming/SharedRim",
    "Naming/SplitLineage",
    "Naming/Unnamed",
    "NonPositiveCount",
    "ParamBox/AxisUnrepresentable",
    "ParamBox/UnknownParam",
    "ParamSourceAttach/FieldNotOnKind",
    "ParamSourceAttach/StaleKey",
    "PayloadExpr",
    "PlacementRule/CountSpelling",
    "PlacementRule/ImproperFrame",
    "PlacementRule/NoPlacements",
    "PlacementRule/NonFiniteFrame",
    "PlacementsUncertified",
    "ProfileAnchor",
    "ProfileLaneReplay(Flipped)",
    "ProfileLaneReplay(None)",
    "ProfilePieces",
    "Seed/CountParam",
    "Seed/TangentUnrepresentable",
    "Seed/UnknownParam",
    "SeedPinnedSection",
    "ShellLaneUnsupported",
    "ShellOpenKind",
    "ShellOpenResolve/Ambiguous",
    "ShellOpenResolve/NodeGone",
    "ShellOpenResolve/Vanished",
    "ToleranceConflict",
    "UnschedulableCycle",
    "VerbArity",
    "WitnessBifurcation",
    "WrongOperand",
    // work/paths/paths-refusals-short-of-the-shape-guard.md
    "Profile/DegenerateSegment",
    "Profile/EmptyProfile",
    "Profile/MultipleOuterLoops",
    "Profile/NearFullArc",
    "Profile/NestingTooDeep",
    "Profile/NonSimple",
    "Profile/RayCastingExhausted",
    "Profile/SliverLoop",
    "Profile/Structure",
    "Profile/TangencyContradicted",
    "Profile/TangentJointOutOfRange",
    "Profile/TooFewVertices",
    "Profile/UndeclaredTangency",
    "ProfileReplay/Path/ArcCenterNotEquidistant",
    "ProfileReplay/Path/ArcLegOnOpenFillet",
    "ProfileReplay/Path/ArcViaCollinear",
    "ProfileReplay/Path/CircleSplitCount",
    "ProfileReplay/Path/DegenerateArcCenter",
    "ProfileReplay/Path/DegenerateArcChord",
    "ProfileReplay/Path/DegenerateArcSpec",
    "ProfileReplay/Path/NoCornerForFillet",
    "ProfileReplay/Path/NoCornerForFillet(disjoint)",
    "ProfileReplay/Path/NoCornerOfPair",
    "ProfileReplay/Path/NonpositiveCircleRadius",
    "ProfileReplay/Path/NonpositiveFilletRadius",
    "ProfileReplay/Path/NonpositiveLeg",
    "ProfileReplay/Path/OverdeterminedJunction",
    "ProfileReplay/Path/PolygonTooFewVertices",
    "ProfileReplay/Path/SeamRetrimsArcFirstSide",
    "ProfileReplay/Path/Structure",
    "ProfileReplay/Path/UnderdeterminedLeg",
    "ProfileReplay/Path/ZeroDirection",
    "ProfileReplay/Transition",
    // work/hone/reach-refusals-short-of-the-shape-guard.md
    "Boolean/Join/Corrupt",
    "Boolean/Join/CutInvariant",
    "Boolean/Join/Euler",
    "Boolean/Join/Section",
    "Boolean/Join/SectionInvariant",
    "Boolean/Join/SectionLoopMixed",
    "Boolean/Join/UnpairedLooseEnds",
    "Split/Finish/Corrupt",
    "Split/Finish/Euler",
    "Split/Finish/NotSingleSolid",
    "Split/Finish/TornComponent",
    "Split/Finish/UnclassifiableComponent",
    "Split/Join/Corrupt",
    "Split/Join/CutInvariant",
    "Split/Join/Euler",
    "Split/Join/Section",
    "Split/Join/SectionInvariant",
    "Split/Join/SectionLoopMixed",
    "Split/Join/UnpairedLooseEnds",
    "Split/Reduce/ConsecutiveOnSectors",
    "Split/Reduce/CorruptOperand",
    "Split/Reduce/CrossingInsertion",
    "Split/Reduce/Euler",
    "Split/Reduce/ScaffoldingOperand",
    // work/carve/carve-refusals-short-of-the-shape-guard.md
    "Blend/Op",
    "Blend/SurgeryInvariant",
    "Extrude/CapPlane",
    "Extrude/Op",
    "Extrude/SidePlane",
    "Loft/CapPlane",
    "Loft/Euler",
    "Loft/SectionStructure",
    "Revolve/CapPlane",
    "Revolve/FullRangeAngle",
    "Revolve/Op",
    "Revolve/VoidInsertion",
    "Skin/BadDegree",
    "Skin/DomainNotUnit",
    "Skin/KnotAlgebra",
    "Skin/SectionProfile",
    "Tube/DegenerateWindow",
    "Tube/FullRangeWindow",
    // work/shell/shell-refusals-short-of-the-shape-guard.md
    "Shell/ChartSenseMixed",
    "Shell/Corrupt",
    "Shell/Face",
    "Shell/Insert",
    "Shell/Lift",
    "Shell/NoSolid",
    "Shell/NotValid",
    "Shell/OpenFaceRimNotExpressible",
    "Shell/OpenFaceStale",
    "Shell/OperandOuterShells",
    "Shell/Partition",
    "Shell/Rim",
    // work/issues/unowned-viewer-refusals-short-of-the-shape-guard.md
    "Check/ChartCoherence(meridian closure)",
    "Check/ChartCoherence(rim)",
    "Check/ChartCoherenceUnexamined(corrupt)",
    "Check/ChartCoherenceUnexamined(non-iso)",
    "Check/ChartCoherenceUnexamined(scaffold)",
    "Check/SeparationUnavailable/Containment(CorruptFace)",
    "Check/SeparationUnavailable/Containment(NoSuchSolid)",
    "Check/SeparationUnavailable/Containment(ZeroVolumeBody)",
];

/// The rows that render a door's generic fallback subject
/// (`geom_core::UNNAMED_DECISION`) on purpose, by exact row id, each
/// with its reason. Every other row's escalation says in words what was
/// decided.
pub(crate) const FILED_SUBJECTLESS: &[(&str, &str)] = &[];

/// Which admission, if any, lets `problem` — one line
/// [`test_utils::refusal::problems`] reported on row `name`, rendered
/// `text` — stand, as the entry's own id.
fn admission(name: &str, text: &str, problem: &str) -> Option<String> {
    use test_utils::refusal::stage_prefixes;
    let labels = stage_prefixes(text, &[]);
    let rest = problem.strip_prefix(name)?;
    if rest.starts_with(" carries the stage prefix ") {
        let label = labels.iter().map(|l| l.trim_end_matches(':')).find(|l| {
            rest.starts_with(&format!(" carries the stage prefix {:?}", format!("{l}:")))
        })?;
        if let Some((ns, _)) = ALLOWED_LABELS
            .iter()
            .find(|(ns, l)| name.starts_with(ns) && *l == label)
        {
            return Some(format!("ALLOWED_LABELS {ns} {label}"));
        }
        if FILED.contains(&(name, label)) {
            return Some(format!("FILED {name} {label}"));
        }
        return FILED_NAMESPACES
            .iter()
            .find(|(ns, l, _)| name.starts_with(ns) && *l == label)
            .map(|(ns, _, _)| format!("FILED_NAMESPACES {ns} {label}"));
    }
    if rest.starts_with(" dumps an arena key") {
        if KERNEL_KEYED.contains(&name) {
            return Some(format!("KERNEL_KEYED {name}"));
        }
        return FILED_NAMESPACES
            .iter()
            .find(|(ns, _, keyed)| *keyed && name.starts_with(ns))
            .map(|(ns, _, _)| format!("FILED_NAMESPACES {ns} key"));
    }
    if rest.starts_with(" escalates without saying what was decided")
        && FILED_SUBJECTLESS.iter().any(|(row, _)| *row == name)
    {
        return Some(format!("FILED_SUBJECTLESS {name}"));
    }
    if rest.starts_with(" states no recourse") && FILED_NO_RECOURSE.contains(&name) {
        return Some(format!("FILED_NO_RECOURSE {name}"));
    }
    None
}

/// Every way the rows among `rows` fall short of the standard that no
/// admission lets stand: [`test_utils::refusal::problems_admitting`] on
/// each, with the exact spans [`ADMISSIONS`] names read as admitted (an
/// admission its row no longer holds is itself a problem), less what
/// [`admission`] admits. Each admission used is added to `used`.
pub(crate) fn short_of_the_standard(
    rows: &[(String, String)],
    used: &mut std::collections::BTreeSet<String>,
) -> Vec<String> {
    let mut problems = Vec::new();
    for (name, text) in rows {
        eprintln!("MEASURE {} {name}: {text}", text.split_whitespace().count());
        for problem in test_utils::refusal::problems_admitting(name, text, &[], false, ADMISSIONS) {
            match admission(name, text, &problem) {
                Some(entry) => {
                    used.insert(entry);
                }
                None => problems.push(problem),
            }
        }
    }
    problems
}

/// [`short_of_the_standard`], for a test that reads only the problems.
pub(crate) fn over_budget(rows: &[(String, String)]) -> Vec<String> {
    short_of_the_standard(rows, &mut std::collections::BTreeSet::new())
}

/// `test_utils::refusal::BARE_RECOURSES` restates `geom_core`'s shared
/// unlabelled repairs (`test-utils` is a dependency-free leaf); this
/// holds the copy equal to the constants, longest first.
#[test]
fn the_bare_recourses_are_geom_cores_constants() {
    assert_eq!(
        test_utils::refusal::BARE_RECOURSES,
        [
            geom_core::COINCIDENCE_RECOURSE,
            geom_core::SPLIT_PLANE_RECOURSE,
            geom_core::NO_DECLARATION_RECOURSE,
        ]
    );
}

/// `test_utils::refusal::GENERIC_SUBJECTS` restates the doors' shared
/// fallback subject; this holds the copy equal.
#[test]
fn the_generic_subject_is_geom_cores_constant() {
    assert_eq!(
        test_utils::refusal::GENERIC_SUBJECTS[0],
        geom_core::UNNAMED_DECISION
    );
}

/// **Every admission is used.** An entry above that no rendered row
/// needs is stale — its owner's fix landed, or the row it names was
/// renamed — and a stale entry would admit the same shortfall again the
/// day it came back. So each list's every entry must admit something on
/// the rows the feature tree and the checks window render.
#[test]
fn every_admission_admits_a_row_it_is_needed_for() {
    let mut rows: Vec<(String, String)> = node_refusals()
        .into_iter()
        .map(|(name, kind)| (name, as_the_viewer_shows_it(kind)))
        .collect();
    rows.extend(
        check_findings()
            .into_iter()
            .map(|(name, finding)| (format!("Check/{name}"), finding.to_string())),
    );
    let mut used = std::collections::BTreeSet::new();
    let _ = short_of_the_standard(&rows, &mut used);
    let mut stale: Vec<String> = Vec::new();
    stale.extend(
        KERNEL_KEYED
            .iter()
            .map(|n| format!("KERNEL_KEYED {n}"))
            .filter(|e| !used.contains(e)),
    );
    stale.extend(
        ALLOWED_LABELS
            .iter()
            .map(|(ns, l)| format!("ALLOWED_LABELS {ns} {l}"))
            .filter(|e| !used.contains(e)),
    );
    stale.extend(
        FILED
            .iter()
            .map(|(n, l)| format!("FILED {n} {l}"))
            .filter(|e| !used.contains(e)),
    );
    for (ns, label, keyed) in FILED_NAMESPACES {
        let mut entries = vec![format!("FILED_NAMESPACES {ns} {label}")];
        if *keyed {
            entries.push(format!("FILED_NAMESPACES {ns} key"));
        }
        stale.extend(entries.into_iter().filter(|e| !used.contains(e)));
    }
    stale.extend(
        FILED_NO_RECOURSE
            .iter()
            .map(|n| format!("FILED_NO_RECOURSE {n}"))
            .filter(|e| !used.contains(e)),
    );
    stale.extend(
        FILED_SUBJECTLESS
            .iter()
            .map(|(n, _)| format!("FILED_SUBJECTLESS {n}"))
            .filter(|e| !used.contains(e)),
    );
    for row in FILED_DECLARE {
        let offers = rows.iter().any(|(n, t)| n == row && t.contains("declare"));
        if !offers {
            stale.push(format!("FILED_DECLARE {row}"));
        }
    }
    assert!(
        stale.is_empty(),
        "admissions no rendered row needs (remove each, and close its row if it was the \
         last):\n{}",
        stale.join("\n")
    );
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
            margin: MarginDiag::value(3.0e-10),
            band: band(),
            predicate: Some("side_of_plane"),
            terminal_sliver: false,
        }
    }

    /// [`diag`] under a predicate name a door raises it with.
    pub(super) fn named(predicate: &'static str) -> Indeterminate {
        Indeterminate {
            predicate: Some(predicate),
            ..diag()
        }
    }

    /// A shell whose volume bracket classified zero, with the zero
    /// end's reporting margin.
    pub(super) fn zero_volume(shell: topo::ShellKey) -> topo::ShellClassifyError {
        topo::ShellClassifyError::ZeroVolume {
            shell,
            verdict: geom_brep::recourse::Classified {
                margin: MarginDiag::value(5.0e-10),
                band: band(),
            },
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
    let mut problems = over_budget(&rows);
    problems.extend(test_utils::refusal::unclaimed_admissions(
        ADMISSIONS,
        rows.iter().map(|(name, _)| name.as_str()),
    ));
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

/// A failure as the feature tree draws it: the failed node's own line,
/// then each refusal it carries as a line of its own, one per level
/// ([`NodeErrorKind::carried_chain`]).
fn drawn_lines(kind: NodeErrorKind) -> Vec<String> {
    let error = NodeError {
        node: RecipeNodeId(tagged(5)),
        kind,
        escalations: std::sync::Arc::new(Vec::new()),
    };
    core::iter::once(error.to_string())
        .chain(error.kind.carried_chain().map(|level| level.line()))
        .collect()
}

/// **Every line a carried refusal draws fits where it is drawn.** A
/// part whose root failed and a mate whose placer refused each name
/// that node and point at it; the refusal they carry is drawn as its
/// own line, one per document level, and never inside theirs.
///
/// Held on the payloads that stress it: the longest refusal on this
/// roster, carried one document down, three documents down, and under
/// a mate's placer. Each carrying line is held to the budget and must
/// not quote what it carries; the carried line is the roster row's own
/// text, so it keeps that row's name and its filed allowances.
#[test]
fn every_carried_refusal_draws_within_the_budget_at_every_line() {
    use editor_core::{MateFault, MateSide, PartFault};
    let longest = node_refusals()
        .into_iter()
        .map(|(name, kind)| {
            (
                as_the_viewer_shows_it(kind).split_whitespace().count(),
                name,
            )
        })
        .max()
        .map(|(_, name)| name)
        .expect("the roster is not empty");
    let inner = || {
        node_refusals()
            .into_iter()
            .find(|(name, _)| *name == longest)
            .map(|(_, kind)| kind)
            .expect("the longest row is on the roster")
    };
    let inner_text = drawn_lines(inner()).remove(0);
    let inner_sentence = inner_text
        .strip_prefix("node 000000000005 failed: ")
        .expect("a node line opens with its node")
        .to_owned();
    let part = |node: u64, refusal: NodeErrorKind| NodeErrorKind::Part {
        doc_ref: doc_ref(),
        fault: PartFault::PartRootFailed {
            node: RecipeNodeId(tagged(node)),
            refusal: refusal.into(),
        },
    };
    let cases = [
        ("Carried/Part", 1, part(7, inner())),
        (
            "Carried/Part/depth-3",
            3,
            part(7, part(4, part(3, inner()))),
        ),
        (
            "Carried/PlacerRefused",
            1,
            NodeErrorKind::Mate(Box::new(MateFault::PlacerRefused {
                mate: RecipeNodeId(tagged(9)),
                side: MateSide::B,
                placer: RecipeNodeId(tagged(4)),
                error: inner().into(),
                placer_row: editor_core::PlacerRow::Silent,
            })),
        ),
    ];
    let mut rows = Vec::new();
    for (case, depth, kind) in cases {
        let lines = drawn_lines(kind);
        assert_eq!(
            lines.len(),
            depth + 1,
            "{case}: one line per carrying level, then the carried refusal: {lines:#?}"
        );
        let (last, carrying) = lines.split_last().expect("at least one line");
        assert!(
            last.ends_with(&inner_sentence),
            "{case}: the last line is the carried refusal as its own tree draws it: {last}"
        );
        for (level, line) in carrying.iter().enumerate() {
            assert!(
                !line.contains(&inner_sentence),
                "{case}: level {level} quotes the refusal it carries: {line}"
            );
            rows.push((format!("{case}/level-{level}"), line.clone()));
        }
        rows.push((longest.clone(), last.clone()));
    }
    let problems = over_budget(&rows);
    assert!(problems.is_empty(), "{}", problems.join("\n"));

    // A placer whose own row states its refusal is pointed at and not
    // carried: the refusal is drawn once, on the placer's row.
    let placer = |placer_row| {
        NodeErrorKind::Mate(Box::new(MateFault::PlacerRefused {
            mate: RecipeNodeId(tagged(9)),
            side: MateSide::B,
            placer: RecipeNodeId(tagged(4)),
            error: inner().into(),
            placer_row,
        }))
    };
    assert_eq!(
        drawn_lines(placer(editor_core::PlacerRow::States)).len(),
        1,
        "a placer that states its own refusal is not carried"
    );
    // A mate's carried level inside a part is numbered in that part, as
    // the part's own level is.
    let in_part = part(7, placer(editor_core::PlacerRow::Silent));
    let documents: Vec<_> = in_part
        .carried_chain()
        .map(|level| (level.document, level.node))
        .collect();
    let part_ref = doc_ref();
    assert_eq!(
        documents,
        vec![
            (
                editor_core::CarriedIn::Part(&part_ref),
                RecipeNodeId(tagged(7))
            ),
            (
                editor_core::CarriedIn::Part(&part_ref),
                RecipeNodeId(tagged(4))
            ),
        ],
        "the placer's level is in the part the mate is in"
    );
}

/// Every offset-fit refusal the feature tree shows ends exactly once,
/// on every route: a labelled repair, or the shared dead end. The
/// carrier the fit forwards whole (`PatchBoundError`) labels its own
/// repair, so a wrapper that added one of its own would read here as
/// two. The interpolation's carriers (`Fit/`, `Structure/`) are NOT
/// forwarded: the kernel chose their samples and knots, so a builder's
/// repair would name nothing the user supplied, and those rows end in
/// the kernel-defect ending instead.
///
/// Every meter refusal ends in its decision's one recourse for its
/// verdict ([`meter_escalations`], [`meter_verdicts`]), never the
/// coincidence menu (a face's meter has nothing to declare), and never
/// advises lowering the tolerance.
#[test]
fn every_offset_fit_refusal_ends_exactly_once() {
    let rows = offset_fit_routes();
    assert!(!rows.is_empty(), "the offset-fit roster is empty");
    let routed = meter_escalations();
    let verdicts = meter_verdicts();
    let mut pinned = 0;
    for (name, kind) in rows {
        let text = as_the_viewer_shows_it(kind);
        assert_eq!(
            test_utils::refusal::recourse_markers(&text),
            1,
            "{name}: {text}"
        );
        assert!(
            !text.contains(geom_core::COINCIDENCE_RECOURSE),
            "{name}: {text}"
        );
        let arm = name
            .strip_prefix("Shell/Face/Fit/")
            .or_else(|| name.strip_prefix("Transform/ApproxRecertify/"))
            .expect("every offset-fit row is on one of the two routes");
        let ending = routed
            .iter()
            .find(|(route, _, _)| arm == format!("Meter/Escalated/{route}"))
            .map(|(_, _, ending)| ending)
            .or_else(|| {
                verdicts
                    .iter()
                    .find(|(row, _)| arm == *row)
                    .map(|(_, ending)| ending)
            });
        if let Some(ending) = ending {
            assert!(text.ends_with(&format!(". {ending}")), "{name}: {text}");
            pinned += 1;
        }
        if arm.starts_with("Meter/") {
            assert!(!text.contains("lower"), "{name}: {text}");
        }
        if arm.starts_with("Fit/") || arm.starts_with("Structure/") {
            assert!(
                text.ends_with(geom_core::KERNEL_DEFECT_ENDING),
                "{name}: {text}"
            );
        }
    }
    // Both routes raise `Meter`, so every pinned ending has two rows.
    assert_eq!(
        pinned,
        2 * (routed.len() + verdicts.len()),
        "a pinned meter row went missing"
    );
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
                source: EvalError::UnknownParam(ParamName::from_static("width")),
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
            "ProfilePieces",
            NodeErrorKind::ProfilePieces {
                fault: editor_core::PiecesFault::Length {
                    loop_: 0,
                    recorded: 4,
                    anchored: 5,
                },
            },
        ),
        row(
            "CurvedSolidFrontier",
            NodeErrorKind::CurvedSolidFrontier {
                what: "a sweep along a curved path",
            },
        ),
        row(
            "MissingInput",
            NodeErrorKind::MissingInput {
                input: RecipeNodeId(tagged(3)),
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
                input: RecipeNodeId(tagged(3)),
                expected: "body",
                found: "profile",
            },
        ),
        row(
            "EmptyOperand",
            NodeErrorKind::EmptyOperand {
                input: RecipeNodeId(tagged(3)),
            },
        ),
        row(
            "EmptyHalf",
            NodeErrorKind::EmptyHalf {
                input: RecipeNodeId(tagged(3)),
                half: editor_core::SplitHalf::Above,
            },
        ),
        row(
            "InstanceOutOfRange",
            NodeErrorKind::InstanceOutOfRange {
                input: RecipeNodeId(tagged(3)),
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
                predicate: "revolve_full_vs_partial",
                source: named("revolve_full_vs_partial"),
            },
        ),
        row(
            "AxisInDifferentPlane",
            NodeErrorKind::AxisInDifferentPlane {
                axis: RecipeNodeId(tagged(3)),
                axis_plane: Some(RecipeNodeId(tagged(1))),
                profile_plane: Some(RecipeNodeId(tagged(2))),
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
                section: RecipeNodeId(tagged(3)),
                param: ParamName::from_static("width"),
            },
        ),
        row(
            "DeclareSiteNotAnOperand",
            NodeErrorKind::DeclareSiteNotAnOperand {
                at: RecipeNodeId(tagged(3)),
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
            NodeErrorKind::ShellLaneUnsupported { scalar: "interval" },
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
                profile: RecipeNodeId(tagged(3)),
                frame: RecipeNodeId(tagged(2)),
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
        (
            "Escalated",
            E::Escalated {
                source: named("tube_window_span"),
            },
        ),
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
            R::CrossingEscalated {
                edge,
                fault: topo::ConicRootFault::CrossingInterior(diag()),
            },
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
            (
                "RingHomingUncrossable",
                J::RingHomingUncrossable {
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
            ("SectionNotPolar", J::SectionNotPolar { face, band: band() }),
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
        ("SectionCusp", F::SectionCusp { edge, face }),
        (
            "SectionWindingUndecided",
            F::SectionWindingUndecided {
                face,
                diag: Some(diag()),
            },
        ),
        (
            "NestingContradiction",
            F::NestingContradiction { hole: face },
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
                source: geom_brep::CertifyError::IntervalNotForward {
                    verdict: geom_brep::recourse::Refused::Negative {
                        margin: geom_core::MarginDiag::value(-1.0),
                    },
                },
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
            E::ApproxLaneUnsupported { scalar: "interval" },
        ),
        ("Corrupt", E::Corrupt { what: "face" }),
    ]
    .into_iter()
    .map(|(n, e)| row(&format!("Transform/{n}"), NodeErrorKind::Transform(e)))
    .chain(certify_refusal_routes())
    .chain(offset_fit_routes())
    .collect()
}

/// One certification refusal per ending the certifier's typed routing
/// gives (D4 ¶1), with that whole ending:
/// - a sized decision's lever and the tolerance below `m/K`, on its
///   in-band arm, and the same lever and conditional on its definite
///   zero arm, which has no margin to quote;
/// - the span's own lever;
/// - a poisoned margin on a sized decision: the lever, and what it may
///   mean;
/// - an exact residual's kernel-defect ending;
/// - an approximation's last resort.
///
/// The band is fixed rather than the run's witness band, so the rendered
/// band numbers and the quoted `m/K` are the same at every eps row.
fn certify_refusals() -> Vec<(&'static str, geom_brep::CertifyError, &'static str)> {
    use geom_brep::{CertCheck, CertifyError};
    use geom_core::{Band, Indeterminate, MarginDiag};
    let band = Band::new(1.0e-9, 1.0e-8).expect("a fixed, ordered band");
    let escalated = |check, margin| CertifyError::Escalated {
        check,
        sample: 4,
        cause: Indeterminate {
            margin,
            band,
            predicate: Some("dihedral_wedge"),
            terminal_sliver: false,
        },
    };
    let in_band = MarginDiag::value(5.0e-9);
    vec![
        (
            "transversality",
            escalated(CertCheck::Transversality, in_band),
            "Recourse: move the geometry so the surfaces cross at a clearer angle, or, if this \
             angle is intended, tighten the tolerance below 5e-10 m",
        ),
        (
            "not-transverse",
            CertifyError::NotTransverse {
                sample: 4,
                verdict: geom_brep::recourse::Refused::Zero(geom_brep::recourse::Classified {
                    margin: MarginDiag::value(5.0e-10),
                    band,
                }),
            },
            "Recourse: move the geometry so the surfaces cross at a clearer angle, or, if this \
             angle is intended, tighten the tolerance below 5e-11 m",
        ),
        (
            "not-transverse, tangent",
            CertifyError::NotTransverse {
                sample: 4,
                verdict: geom_brep::recourse::Refused::Zero(geom_brep::recourse::Classified {
                    margin: MarginDiag::value(0.0),
                    band,
                }),
            },
            "Recourse: move the geometry so the surfaces cross at a clearer angle",
        ),
        (
            "span",
            escalated(CertCheck::ParamSpan, in_band),
            "Recourse: move the geometry so this edge is not vanishingly short, or, if this \
             length is intended, tighten the tolerance below 5e-10 m",
        ),
        (
            "invalid",
            escalated(CertCheck::Transversality, MarginDiag::INVALID),
            "Recourse: move the geometry so the surfaces cross at a clearer angle; an unreadable or \
             collapsed margin may indicate a kernel bug worth reporting",
        ),
        (
            "endpoint",
            escalated(CertCheck::EndpointStart, in_band),
            geom_core::KERNEL_DEFECT_ENDING,
        ),
        (
            "surface-residual",
            escalated(CertCheck::Surface1Residual, in_band),
            geom_core::KERNEL_LIMIT_RECOURSE,
        ),
    ]
}

/// [`certify_refusals`] as the feature tree meets them: a transform's
/// re-certification of a mapped edge.
fn certify_refusal_routes() -> Vec<(String, NodeErrorKind)> {
    certify_refusals()
        .into_iter()
        .map(|(route, source, _)| {
            row(
                &format!("Transform/Certify/Routed/{route}"),
                NodeErrorKind::Transform(topo::TransformError::Certify {
                    edge: topo::EdgeKey::default(),
                    source,
                }),
            )
        })
        .collect()
}

/// A certification refusal ends in the one ending its decision and
/// verdict route it to, whole, with one recourse marker and no
/// declaration: certification takes none.
#[test]
fn every_certify_refusal_ends_in_its_routed_sentence() {
    let rows = certify_refusal_routes();
    let routed = certify_refusals();
    assert_eq!(rows.len(), routed.len());
    for ((name, kind), (route, _, ending)) in rows.into_iter().zip(routed) {
        let text = as_the_viewer_shows_it(kind);
        assert!(text.ends_with(ending), "{name}: {text}");
        if route == "transversality" {
            assert_eq!(
                text,
                "node 000000000005 failed: the transform op refused: an edge the map moved failed \
                 re-certification: the transversality margin at sample 4 escalated: margin 5e-9 \
                 lies inside the ambiguity band (1e-9, 1e-8). Recourse: move the geometry so the surfaces cross at a clearer \
                 angle, or, if this angle is intended, tighten the tolerance below 5e-10 m"
            );
        }
        assert_eq!(
            test_utils::refusal::recourse_markers(&text),
            1,
            "{name}: {text}"
        );
        assert!(!text.contains("declare"), "{name}: {text}");
    }
}

const SPLIT: &str = "Recourse: split the face clear of any pole, cusp or pinch";
const DISTANCE: &str =
    "Recourse: use an offset distance of smaller magnitude, or offset to the other side";

/// One `Meter/Escalated` sample per ending its meter's decision gives an
/// undecided margin (D4 ¶1), with that whole ending: each meter's lever
/// and the tolerance below `m/K` on a positive margin in band; the lever
/// alone on an enclosure straddling zero, which no smaller tolerance
/// passes; and on a poisoned margin the lever and what it may mean.
///
/// The band is fixed rather than the run's witness band, so the quoted
/// `m/K` is the same at every eps row.
fn meter_escalations() -> Vec<(&'static str, geom_brep::OffsetFitError, String)> {
    use geom_brep::offset_meters::{Meter, MeterError};
    use geom_core::{Band, Indeterminate, MarginDiag};
    let band = Band::new(1.0e-9, 1.0e-8).expect("a fixed, ordered band");
    let escalated = |meter: Meter, margin| {
        geom_brep::OffsetFitError::Meter(MeterError::Escalated {
            meter,
            source: Indeterminate {
                margin,
                band,
                predicate: Some(meter.predicate()),
                terminal_sliver: false,
            },
        })
    };
    let in_band = MarginDiag::value(5.0e-9);
    let wide = MarginDiag::enclosure(-2.0e-9, 4.0e-9);
    let tighten = |lever: &str, size: &str| {
        format!("{lever}, or, if this {size} is intended, tighten the tolerance below 5e-10 m")
    };
    vec![
        (
            "curvature",
            escalated(Meter::CurvatureHeadroom, in_band),
            tighten(DISTANCE, "clearance"),
        ),
        (
            "curvature-enclosure",
            escalated(Meter::CurvatureHeadroom, wide),
            DISTANCE.to_owned(),
        ),
        (
            "floor",
            escalated(Meter::NormalFloor, in_band),
            tighten(SPLIT, "thinness"),
        ),
        (
            "floor-enclosure",
            escalated(Meter::NormalFloor, wide),
            SPLIT.to_owned(),
        ),
        (
            "invalid",
            escalated(Meter::CurvatureHeadroom, MarginDiag::INVALID),
            format!(
                "{DISTANCE}; an unreadable or collapsed margin may indicate a kernel bug worth \
                 reporting"
            ),
        ),
    ]
}

/// The ending of each definite meter sample `topo`'s roster carries, by
/// its row's arm: a zero verdict with a positive margin inside the zero
/// band is band-decided and names the tolerance below `m/K`; a floor of
/// exactly zero, which no tolerance resolves, names the lever and says a
/// face with no degeneracy is worth reporting; a sign-certain fold names
/// the lever alone.
fn meter_verdicts() -> [(&'static str, String); 4] {
    [
        (
            "Meter/NormalFloor",
            format!(
                "{SPLIT}, or, if this thinness is intended, tighten the tolerance below 5e-11 m"
            ),
        ),
        (
            "Meter/NormalFloor#2",
            format!("{SPLIT}; if it has none, this may indicate a kernel bug worth reporting"),
        ),
        ("Meter/CurvatureHeadroom", DISTANCE.to_owned()),
        (
            "Meter/CurvatureHeadroom#2",
            format!(
                "{DISTANCE}, or, if this clearance is intended, tighten the tolerance below \
                 5e-11 m"
            ),
        ),
    ]
}

/// Every `geom_brep::OffsetFitError` arm, through each feature-tree
/// route that can raise it.
///
/// **Which route renders which arms.** The offset fit's refusals reach
/// the feature tree two ways:
///
/// - **The shell op's face replacement** (`Shell/Face/Fit/…`): the
///   fit lane's mint runs the whole fit loop and then certifies, so it
///   raises every arm but `WindowUnsupported` (the mint certifies over
///   the chart rectangle it fitted) and `Band` (below). The loop's own
///   terminations — `BudgetExhausted`, `SampleCapReached`, `BoundNotFinite`,
///   `RefinementStalled` — and the interpolation's `Fit`, `Structure`
///   and `NonFiniteSample` reach the user by this route and by the
///   transform's re-fit. Its wrapper still opens with a stage prefix and names the face by key;
///   both are SHELL's and filed
///   (`work/shell/replace-face-refusals-open-with-a-stage-prefix-and-name-keys.md`),
///   so [`FILED_NAMESPACES`] admits exactly that label and that key on
///   these rows and nothing else.
/// - **The transform op** (`Transform/ApproxRecertify/…`) raises the
///   certifier's arms on the image — `certify_offset_over` runs the
///   meters and the certificate limbs on a fit it did not make, so
///   `Meter`, `PatchBound`, `WindowUnsupported`, `Limb` and
///   `Elevation` — and the mint's arms on a re-fit, when a sound face's
///   image refuses a limb and the map fits the mapped description
///   afresh: every arm the shell route raises. So every arm but `Band`.
///
/// `Band` reaches neither route. The door derives the run's band from
/// the witness, and both ops derive the same band before they reach the
/// door and refuse on it themselves (`ShellError::Band`,
/// `TransformError::Band`), so it has no row here.
///
/// The roster is `topo`'s: every `OffsetFitError` sample
/// `validation_error_samples` carries, which `topo`'s coverage row holds
/// complete over the enum's variants, over `MeterError`'s and (by
/// `strum`) over `PatchBoundError`'s. `Meter/Escalated` ends by its
/// meter and margin, so the rows add one per ending
/// ([`meter_escalations`]).
fn offset_fit_routes() -> Vec<(String, NodeErrorKind)> {
    use geom_brep::OffsetFitError as O;
    use topo::{FaceKey, ReplaceFaceError, ShellError};
    let mut roster: Vec<(String, O)> = topo::test_support::validation_error_samples()
        .into_iter()
        .filter_map(|(_, e)| match e {
            topo::ValidationError::ApproxCertification { error, .. } => Some(error),
            _ => None,
        })
        .map(|source| {
            // The variant and the variant it carries, read off `Debug`:
            // `Meter(NormalFloor`, `BoundNotFinite`.
            let debug = format!("{source:?}");
            let arm: String = debug
                .chars()
                .take_while(|c| c.is_alphanumeric() || matches!(c, '_' | '('))
                .collect();
            (arm.trim_end_matches('(').replace('(', "/"), source)
        })
        .collect();
    // The roster is borrowed, so its reach is checked on the roster
    // itself: a sample list that stopped carrying an arm would
    // otherwise shrink these rows silently. `BudgetExhausted`,
    // `BoundNotFinite` and `Limb` each carry two samples (both
    // `LastRound` readings, both `best` cases, both limbs).
    for (arm, samples) in [
        ("Meter/NormalFloor", 2),
        ("Meter/CurvatureHeadroom", 2),
        ("Meter/Escalated", 1),
        ("PatchBound/", 7),
        ("Fit/", 1),
        ("Structure/", 1),
        ("InvalidRequest", 1),
        ("NonFiniteSample", 1),
        ("BudgetExhausted", 2),
        ("SampleCapReached", 1),
        ("BoundNotFinite", 2),
        ("RefinementStalled", 1),
        ("WindowUnsupported", 1),
        ("Limb", 2),
        ("Elevation/", 1),
    ] {
        let have = roster.iter().filter(|(n, _)| n.starts_with(arm)).count();
        assert!(
            have >= samples,
            "the roster carries {have} OffsetFitError::{arm} sample(s), under {samples}"
        );
    }
    roster.extend(
        meter_escalations()
            .into_iter()
            .map(|(route, source, _)| (format!("Meter/Escalated/{route}"), source)),
    );
    let face = FaceKey::default();
    let mut rows = Vec::new();
    for (arm, source) in roster {
        let transform = !matches!(source, O::Band(_));
        if !matches!(source, O::WindowUnsupported { .. } | O::Band(_)) {
            rows.push(row(
                &format!("Shell/Face/Fit/{arm}"),
                NodeErrorKind::Shell(Box::new(ShellError::Face {
                    face,
                    error: Box::new(ReplaceFaceError::<f64>::Fit {
                        face,
                        error: source.clone(),
                    }),
                })),
            ));
        }
        if transform {
            rows.push(row(
                &format!("Transform/ApproxRecertify/{arm}"),
                NodeErrorKind::Transform(topo::TransformError::ApproxRecertify { source }),
            ));
        }
    }
    // Two samples of one arm are two different sentences; the row id
    // says which by position.
    let mut seen = std::collections::BTreeMap::<String, usize>::new();
    for (name, _) in &mut rows {
        let n = seen.entry(name.clone()).or_default();
        *n += 1;
        if *n > 1 {
            name.push_str(&format!("#{n}"));
        }
    }
    rows
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
    use sweep::blend::{
        BlendDecision, BlendError as E, BlendKind, BlendSite, ClassifiedMargin, CornerConfig,
    };
    use topo::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};
    let (face, edge, vertex) = (FaceKey::default(), EdgeKey::default(), VertexKey::default());
    let decided = |predicate, m: f64, sign| ClassifiedMargin {
        predicate,
        reading: MarginDiag::value(m),
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
                gap: MarginDiag::value(0.2),
                cross_chain: false,
            },
        ),
        (
            "FaceClearanceUncertified(cross-chain)",
            E::FaceClearanceUncertified {
                face,
                margin: decided("fillet3_face_clearance", -1e-3, Sign::Negative),
                gap: MarginDiag::value(0.2),
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
                arm: MarginDiag::value(0.5),
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
                decision: BlendDecision::RadiusHeadroom,
                source: escalated("fillet3_radius_headroom"),
            },
        ),
        (
            "Escalated(clearance)",
            E::Escalated {
                site: BlendSite::Link { edge },
                decision: BlendDecision::FaceClearance,
                source: escalated("fillet3_face_clearance"),
            },
        ),
        (
            "Escalated(chain)",
            E::Escalated {
                site: BlendSite::Joint { vertex },
                decision: BlendDecision::ChainG1,
                source: escalated("fillet3_chain_g1"),
            },
        ),
        (
            "Escalated(contact)",
            E::Escalated {
                site: BlendSite::Link { edge },
                decision: BlendDecision::ContactSecondOrder,
                source: escalated("tangent_second_order"),
            },
        ),
        (
            "Escalated(corner)",
            E::Escalated {
                site: BlendSite::Joint { vertex },
                decision: BlendDecision::CornerIndependence,
                source: escalated("fillet3_corner_independence"),
            },
        ),
        (
            "Escalated(ring)",
            E::Escalated {
                site: BlendSite::Chain,
                decision: BlendDecision::RingClearance,
                source: escalated("fillet3_ring_clearance"),
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
                chain: sweep::blend::Convexity::Convex,
                margin: decided("fillet3_ring_clearance", -1e-3, Sign::Negative),
                bounded: false,
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
                source: named("vertex_separation"),
            },
        ),
        (
            "Escalated(pair)",
            E::Escalated {
                site: EscalationSite::SegmentPair(a, b),
                source: named("collinear_overlap"),
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
        (
            "Escalated",
            P::Escalated {
                source: named("ray_side"),
            },
        ),
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
        Diagnosis, Dimension, EntityKind, EvalError, NamingError, ParamBoxError, ParamName,
        PlacementRuleFault, RecipeEditRef, ResolveError, RimShare, SeedError, SlotId, StableName,
        TieWitness,
    };
    use geom_core::Sign;
    use payloads::*;
    let name = || StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(tagged(3)),
        path: Vec::new(),
    };
    let eval: Vec<(&str, EvalError)> = vec![
        (
            "UnknownParam",
            EvalError::UnknownParam(ParamName::from_static("width")),
        ),
        (
            "ParamDimensionMismatch",
            EvalError::ParamDimensionMismatch {
                name: ParamName::from_static("width"),
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
                param: ParamName::from_static("width"),
            },
        ),
        (
            "AxisUnrepresentable",
            ParamBoxError::AxisUnrepresentable {
                param: ParamName::from_static("width"),
                lo: 1.0,
                hi: 0.0,
            },
        ),
    ];
    let seed = [
        (
            "UnknownParam",
            SeedError::UnknownParam {
                param: ParamName::from_static("width"),
            },
        ),
        (
            "CountParam",
            SeedError::CountParam {
                param: ParamName::from_static("n"),
            },
        ),
        (
            "TangentUnrepresentable",
            SeedError::TangentUnrepresentable {
                param: ParamName::from_static("width"),
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
                node: RecipeNodeId(tagged(3)),
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
                node: RecipeNodeId(tagged(3)),
                face: topo::FaceKey::default(),
                other: topo::FaceKey::default(),
                found: RimShare::Several,
            },
        ),
        ("Band", NamingError::Band(band_error())),
        (
            "Escalated",
            NamingError::Escalated {
                predicate: "name_frag_order_along",
                source: named("name_frag_order_along"),
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
                        node: RecipeNodeId(tagged(4)),
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
                        node: RecipeNodeId(tagged(3)),
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
        node: RecipeNodeId(tagged(node)),
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
        MeasureNodeFault, PartFault, SitedRef, WitnessAge, WitnessBifurcation,
    };
    use geom_core::UnitVec3Error;
    use payloads::*;
    use topo::{DanglingRef, EntityId, FaceKey, ReadbackError};
    let face = || stable(EntityKind::Face, 3);
    let sited = |node| SitedRef {
        at: RecipeNodeId(tagged(node)),
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
            "Unplaced/NoOffset",
            NodeErrorKind::Unplaced {
                group: RecipeNodeId(6),
                cause: editor_core::Unplaced::NoOffset,
            },
        ),
        row(
            "Unplaced/DeadGauge",
            NodeErrorKind::Unplaced {
                group: RecipeNodeId(6),
                cause: editor_core::Unplaced::DeadGauge {
                    gauge: RecipeNodeId(2),
                },
            },
        ),
        row(
            "PlacementRefused",
            NodeErrorKind::PlacementRefused {
                node: RecipeNodeId(2),
                error: NodeErrorKind::EmptyOperand {
                    input: RecipeNodeId(1),
                }
                .into(),
            },
        ),
        row(
            "CrossingUnverified",
            NodeErrorKind::CrossingUnverified {
                instance: RecipeNodeId(tagged(6)),
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
                profile: RecipeNodeId(tagged(4)),
                frame: RecipeNodeId(tagged(2)),
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
            InterrogateError::Standing(NodeStanding::NotEvaluated {
                node: RecipeNodeId(tagged(3)),
            }),
        ),
        (
            "NodeFailed",
            InterrogateError::Standing(NodeStanding::Failed {
                node: RecipeNodeId(tagged(3)),
            }),
        ),
        (
            "NodePoisoned",
            InterrogateError::Standing(NodeStanding::Poisoned {
                node: RecipeNodeId(tagged(3)),
                through: RecipeNodeId(tagged(2)),
            }),
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
    // A part's root failure carries the part's own node refusal, typed,
    // and its sentence never renders it: the carried refusal is its own
    // line ([`every_carried_refusal_draws_within_the_budget_at_every_line`]).
    //
    // `PartFault::Unresolved` renders its resolver's own sentence, so its
    // rows are the shipped store's real refusals, raised through the
    // viewer's resolver (`viewer/tests/instance_authoring.rs`,
    // `every_unresolved_part_badge_meets_the_refusal_standard`).
    let parts = [
        ("NoResolver", PartFault::NoResolver),
        (
            "PartRootFailed",
            PartFault::PartRootFailed {
                node: RecipeNodeId(tagged(7)),
                refusal: NodeErrorKind::Extrude(sweep::ExtrudeError::DegenerateExtrusion).into(),
            },
        ),
        (
            "PartRootFailed(part)",
            PartFault::PartRootFailed {
                node: RecipeNodeId(tagged(7)),
                refusal: NodeErrorKind::Part {
                    doc_ref: doc_ref(),
                    fault: PartFault::DepthExceeded,
                }
                .into(),
            },
        ),
        (
            "PartRootPoisoned",
            PartFault::PartRootPoisoned {
                root: RecipeNodeId(tagged(8)),
                through: RecipeNodeId(tagged(7)),
                refusal: NodeErrorKind::Extrude(sweep::ExtrudeError::DegenerateExtrusion).into(),
            },
        ),
        (
            "RootFailureUnrecorded",
            PartFault::RootFailureUnrecorded {
                node: RecipeNodeId(tagged(7)),
            },
        ),
        (
            "ReferenceCycle",
            PartFault::ReferenceCycle {
                cycle: vec![doc_ref(), doc_ref()],
            },
        ),
        ("DepthExceeded", PartFault::DepthExceeded),
        ("NotEntered", PartFault::NotEntered),
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
    rows.extend(part_products());
    rows.extend(part_products_forwarding());
    rows
}

/// The instance rows of a part with no product, raised through real
/// documents so each carries the gather's own sentence: a sketch-only
/// part, whose roots denote no body; a part that places its body under
/// two roots; and a part whose split and a move of the split's target
/// are both roots, so the two alias the block's strict wall names.
fn part_products() -> Vec<(String, NodeErrorKind)> {
    use crate::fixture::resolver::{PartStore, with_resolver};
    use crate::fixture::{ang, insert, len, on_frame, scl, square};
    use editor_core::{
        CancelToken, Datum, DocumentId, Node, NodeResult, PartFault, ProductErrorKind, ProfileDoc,
        evaluate,
    };
    use geom_core::Tol;
    let tol = Tol::witness();
    let sketch = |label| {
        on_frame(
            ProfileDoc::empty(DocumentId::derive(label), tol),
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(0.0, 0.0, 0.5)],
        )
    };
    let moved = |doc, input, dx| {
        insert(
            doc,
            Node::transform(
                input,
                editor_core::Step::Rigid {
                    translation: [len(dx), len(0.0), len(0.0)],
                    axis: [scl(0.0), scl(0.0), scl(1.0)],
                    angle: ang(0.0),
                },
            ),
        )
        .0
    };
    let twice = {
        let (doc, profile) = sketch("concision-part-twice");
        let (doc, body) = insert(
            doc,
            Node::Extrude {
                profile,
                distance: len(1.0),
            },
        );
        moved(moved(doc, body, 2.0), body, 4.0)
    };
    let aliased = {
        let (doc, profile) = sketch("concision-part-aliased");
        let (doc, block) = insert(
            doc,
            Node::Extrude {
                profile,
                distance: len(1.0),
            },
        );
        let (doc, plane) = insert(
            doc,
            Node::Datum(Datum::Plane {
                origin: [len(0.25), len(0.0), len(0.0)],
                normal: [scl(1.0), scl(0.0), scl(0.0)],
            }),
        );
        let (doc, _) = insert(
            doc,
            Node::Split {
                target: block,
                tool: plane,
            },
        );
        moved(doc, block, 2.0)
    };
    let mut store = PartStore::new();
    let parts = [
        (
            "Part/PartProduct",
            store.insert(sketch("concision-part-sketch").0, tol),
            ProductErrorKind::NoBodyRoots,
        ),
        (
            "Part/PartProduct(PlacedUnderTwoRoots)",
            store.insert(twice, tol),
            ProductErrorKind::PlacedUnderTwoRoots,
        ),
        (
            "Part/PartProduct(Naming)",
            store.insert(aliased, tol),
            ProductErrorKind::Naming,
        ),
    ];
    let opts = with_resolver(store);
    parts
        .into_iter()
        .map(|(name, part, class)| {
            let (doc, instance) = insert(
                ProfileDoc::empty(DocumentId::derive(name), tol),
                Node::instantiate_part(part),
            );
            match evaluate::<f64>(&doc, None, &CancelToken::new(), &opts, tol).result(instance) {
                Some(NodeResult::Failed(error)) => match &error.kind {
                    NodeErrorKind::Part {
                        doc_ref,
                        fault: fault @ PartFault::PartProduct { refusal },
                    } if refusal.kind() == class => row(
                        name,
                        NodeErrorKind::Part {
                            doc_ref: *doc_ref,
                            fault: fault.clone(),
                        },
                    ),
                    other => panic!("{name}: the instance refuses as a {class:?} part: {other:?}"),
                },
                other => panic!("{name}: the instance refuses: {other:?}"),
            }
        })
        .collect()
}

/// The instance rows of the three gather classes whose sentence
/// forwards the kernel's own refusal, which no document reaches: a root
/// the at-rest gate refuses, an aggregate it refuses, and a graft the
/// kernel refuses. Each is built as the instance carries it, the
/// gather's refusal whole.
fn part_products_forwarding() -> Vec<(String, NodeErrorKind)> {
    use editor_core::{PartFault, ProductError, SourceFinding};
    let inside_out = || topo::ValidationError::NegativeVolume {
        solid: topo::SolidKey::default(),
    };
    [
        (
            "Part/PartProduct(RootInvalid)",
            ProductError::RootInvalid {
                findings: vec![SourceFinding {
                    node: RecipeNodeId(tagged(3)),
                    output: 0,
                    errors: vec![inside_out()],
                }],
            },
        ),
        (
            "Part/PartProduct(ProductInvalid)",
            ProductError::ProductInvalid {
                errors: vec![inside_out()],
            },
        ),
        (
            "Part/PartProduct(Graft)",
            ProductError::Graft {
                node: RecipeNodeId(tagged(5)),
                source: Box::new(topo::BooleanError::Band(geom_core::BandError::Empty {
                    zero: 1.0,
                    escalate: 0.5,
                })),
            },
        ),
    ]
    .into_iter()
    .map(|(name, error)| {
        row(
            name,
            NodeErrorKind::Part {
                doc_ref: doc_ref(),
                fault: PartFault::PartProduct {
                    refusal: error.into(),
                },
            },
        )
    })
    .collect()
}

fn mate() -> Vec<(String, NodeErrorKind)> {
    use editor_core::{Clash, DocumentId, LeverRefusal, MateFault as M, MateSide, Subgroup};
    use geom_core::{FrameError, FrameInput};
    use payloads::*;
    let n = |id| RecipeNodeId(tagged(id));
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
                placer_row: editor_core::PlacerRow::Silent,
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
            "FaceUnresolved",
            M::FaceUnresolved {
                mate: n(9),
                side: MateSide::A,
                refusal: Box::new(editor_core::FaceRefusal::Reach {
                    instance: n(6),
                    part: doc_ref(),
                    face: editor_core::FaceName::new(editor_core::StableName {
                        kind: editor_core::EntityKind::Face,
                        node: n(3),
                        path: vec![],
                    })
                    .expect("a face"),
                    refusal: editor_core::FacePoseRefusal::NoSuchName,
                }),
            },
        ),
        (
            "Unleverable",
            M::Unleverable {
                mate: n(9),
                refusal: Box::new(LeverRefusal::Reach {
                    instance: n(6),
                    part: doc_ref(),
                    refusal: editor_core::ReachRefusal::NoExtent,
                }),
            },
        ),
        (
            "OffsetDisagrees",
            M::OffsetDisagrees {
                instance: n(7),
                root: n(6),
                predicate: "mate_member_translation_zero",
                clash: Clash::Length { metres: 0.002 },
            },
        ),
        (
            "OffsetUnchecked/Placement",
            M::OffsetUnchecked {
                instance: n(7),
                cause: Box::new(editor_core::OffsetCheck::Placement {
                    node: n(3),
                    error: NodeErrorKind::EmptyOperand { input: n(2) }.into(),
                }),
            },
        ),
        (
            "OffsetUnchecked/Unleverable",
            M::OffsetUnchecked {
                instance: n(7),
                cause: Box::new(editor_core::OffsetCheck::Unleverable(LeverRefusal::Reach {
                    instance: n(7),
                    part: doc_ref(),
                    refusal: editor_core::ReachRefusal::NoExtent,
                })),
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
                error: payloads::zero_volume(shell),
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
        CancelToken, CapEnd, Datum, EvalOptions, Node, NodeResult, ProfileDoc, SitedRef, evaluate,
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
    let vertex = fixture::cap_vertex(body, CapEnd::End, crate::fixture::vpiece(&doc, body, 0, 0));
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

/// **The checks window's escalated evidence ends in the decision that
/// escalated** (D4 ¶1 (i)). The shell-role sign passes on either
/// definite sign and its margin is a thickness, so every band-decided
/// arm ends in its lever plus the tolerance that decides it, valued at
/// `|m|/K` where the verdict carries a margin and without a value where
/// it does not (a zero, a straddle); an enclosure across zero is passed
/// by no tolerance and names the lever alone. A source that is not that decision's
/// ends in its own payload's recourse and no invented lever. No row the
/// window writes itself advises lowering the tolerance.
#[test]
fn every_escalated_check_finding_ends_in_its_decisions_recourse() {
    use editor_core::{CheckEvidence, CheckFinding, CheckId};
    use geom_core::{Band, Indeterminate, MarginDiag};
    use topo::{ShellClassifyError as S, ShellKey};
    const LEVER: &str = "Recourse: thicken or remove the degenerate geometry";
    let shell = ShellKey::default();
    // `K = 10`: a margin `m` is decided at every tolerance below `|m|/10`.
    let band = Band::new(1e-9, 1e-8).expect("a band");
    let escalated = |margin| S::Escalated {
        shell,
        source: Indeterminate {
            margin,
            band,
            predicate: Some("chk_shell_volume_sign"),
            terminal_sliver: false,
        },
    };
    let render = |source| {
        CheckFinding {
            check: CheckId::Connectedness,
            root: RecipeNodeId(tagged(4)),
            output_ix: 0,
            evidence: CheckEvidence::Escalated { source },
        }
        .to_string()
    };
    let head =
        "check connectedness: root 000000000004 output 0: the component count is unknowable: ";
    let sign = "the sign of a shell's volume is too close to call: ";
    let in_band =
        |m: &str| format!("{head}{sign}margin {m} lies inside the ambiguity band (1e-9, 1e-8). ");
    let pinned = [
        (
            "in band, outer side",
            escalated(MarginDiag::value(5e-9)),
            format!(
                "{}{LEVER}, or, if this thickness is intended, tighten the tolerance below 5e-10 m",
                in_band("5e-9")
            ),
        ),
        (
            "in band, void side",
            escalated(MarginDiag::value(-2e-9)),
            format!(
                "{}{LEVER}, or, if this thickness is intended, tighten the tolerance below 2e-10 m",
                in_band("-2e-9")
            ),
        ),
        (
            "in band, bracket across zero",
            escalated(MarginDiag::enclosure(-2e-9, 3e-9)),
            format!(
                "{head}{sign}enclosure [-2e-9, 3e-9] cannot be classified against the ambiguity band (1e-9, 1e-8). {LEVER}"
            ),
        ),
        (
            "invalid margin",
            escalated(MarginDiag::INVALID),
            format!(
                "{head}{sign}margin is invalid (NaN or a poisoned enclosure) against the ambiguity band (1e-9, 1e-8). {LEVER}; an \
                 unreadable or collapsed margin may indicate a kernel bug worth reporting"
            ),
        ),
        (
            "zero",
            S::ZeroVolume {
                shell,
                verdict: geom_brep::recourse::Classified {
                    margin: MarginDiag::value(-5.0e-10),
                    band,
                },
            },
            format!(
                "{head}a shell's signed volume, or an end of its certified bracket, is zero at \
                 this tolerance. {LEVER}, or, if this thickness is intended, tighten the \
                 tolerance below 5e-11 m"
            ),
        ),
        (
            "zero, of no size",
            S::ZeroVolume {
                shell,
                verdict: geom_brep::recourse::Classified {
                    margin: MarginDiag::value(0.0),
                    band,
                },
            },
            format!(
                "{head}a shell's signed volume, or an end of its certified bracket, is zero at \
                 this tolerance. {LEVER}"
            ),
        ),
        (
            "straddle",
            S::Straddles { shell },
            format!(
                "{head}a shell's certified volume bracket straddles zero at this tolerance. \
                 {LEVER}"
            ),
        ),
    ];
    for (name, source, want) in pinned {
        let text = render(source.clone());
        assert_eq!(text, want, "{name}");
        // The payload's own Display ends in the same one ending.
        let ending = source.ending().expect("the shell-role decision's refusal");
        assert!(text.ends_with(&ending), "{name}: {text}");
        let whole = source.to_string();
        assert!(whole.ends_with(&format!(". {ending}")), "{name}: {whole}");
        assert_eq!(
            test_utils::refusal::recourse_markers(&whole),
            1,
            "{name}: {whole}"
        );
    }
    // A band failure is the run's configuration, not the shell-role
    // decision: the finding forwards its payload and adds no lever.
    let error = payloads::band_error();
    let unowned = render(S::Band { error });
    assert!(unowned.ends_with(&error.to_string()), "{unowned}");
    assert!(!unowned.contains("thicken"), "{unowned}");
    for (name, finding) in &check_findings() {
        let text = finding.to_string();
        // The separation arm forwards the Boolean's own sentence, whose
        // coincidence ending is `geom_core::COINCIDENCE_RECOURSE`'s to
        // repair (work/props/coincidence-recourse-says-lower-where-d4-says-tighten.md).
        if !name.starts_with("SeparationUnavailable/") {
            assert!(!text.contains("lower"), "{name}: {text}");
        }
    }
}

fn check_findings() -> Vec<(String, editor_core::CheckFinding)> {
    use editor_core::{CheckEvidence as E, CheckFinding, CheckId};
    use geom_core::{Indeterminate, MarginDiag};
    use payloads::*;
    use topo::{
        BooleanError, CoherenceCondition, CoherenceFinding, EdgeKey, FaceKey, LoopKey,
        PointInSolidError, ShellClassifyError, ShellKey, StructureRead, Unexaminable, Unexamined,
        VertexKey,
    };
    let shell = ShellKey::default();
    let finding = |check, evidence| CheckFinding {
        check,
        root: RecipeNodeId(tagged(4)),
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
                    source: payloads::zero_volume(shell),
                },
            ),
        ),
        (
            "Escalated(straddle)",
            finding(
                CheckId::Connectedness,
                E::Escalated {
                    source: ShellClassifyError::Straddles { shell },
                },
            ),
        ),
        (
            "Escalated(invalid margin)",
            finding(
                CheckId::Connectedness,
                E::Escalated {
                    source: ShellClassifyError::Escalated {
                        shell,
                        source: Indeterminate {
                            margin: MarginDiag::INVALID,
                            ..diag()
                        },
                    },
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
                    other_root: RecipeNodeId(tagged(7)),
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
            "EdgeCarrierUnsupported",
            PointInSolidError::EdgeCarrierUnsupported { face },
        ),
        (
            "WallOutlineUnsupported",
            PointInSolidError::WallOutlineUnsupported { face },
        ),
        (
            "NoSuchSolid",
            PointInSolidError::NoSuchSolid {
                solid: topo::SolidKey::default(),
            },
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
/// `sweep/src/blend`, and the `detail:` of each of those arms built
/// directly, a literal or a `const` resolved in the same tree — and
/// renders each through the feature tree's chain, under both verbs.
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
    // A reader that found nothing, or lost most of what it reads, would
    // pass vacuously: every helper is read, and the count holds its
    // floor — the sites the module raised when this floor was set. A
    // floor that a real removal crosses is lowered with it.
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
    assert!(
        details.len() >= BLEND_DETAIL_FLOOR,
        "{} blend details read, below the floor of {BLEND_DETAIL_FLOOR}",
        details.len()
    );
    // `BodyNotIntact` names the entity that did not resolve: a corrupt
    // body, whose report needs the key. Keyed by the arm the detail is
    // raised with, not by the row's name.
    let problems: Vec<String> = rows
        .iter()
        .zip(details.iter().flat_map(|d| [d, d]))
        .flat_map(|((name, text), (helper, _, _))| {
            eprintln!("MEASURE {} {name}: {text}", text.split_whitespace().count());
            test_utils::refusal::problems(name, text, &[], *helper == "not_intact")
        })
        .collect();
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// How many detail-carrying raise sites [`blend_details`] read from
/// `sweep/src/blend` when this floor was set.
const BLEND_DETAIL_FLOOR: usize = 187;

/// Every `(helper, file:line, detail)` the blend module raises a
/// detail-carrying refusal with, read from its source: each helper call,
/// and each of the helpers' arms built directly with a `detail:` field.
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
        // An arm built directly rather than through its helper, with its
        // detail as a field: `BlendError::BodyNotIntact { at, detail: … }`.
        // A pattern (`{ .. }`, `{ detail, .. }`) or the helper's own
        // shorthand (`{ at, detail }`) states no `detail:` field and is
        // not a raise site.
        for (arm, helper) in [
            ("UnsupportedChain", "unbuilt_chain"),
            ("UnsupportedRunOut", "unbuilt_run_out"),
            ("UnsupportedGeometry", "unbuilt_geometry"),
            ("BodyNotIntact", "not_intact"),
        ] {
            let needle = format!("{arm} {{");
            for (at, _) in code.match_indices(&needle) {
                // A path to the arm, not the enum's own declaration.
                if !(code[..at].ends_with("BlendError::") || code[..at].ends_with("Self::")) {
                    continue;
                }
                let open = at + arm.len() + 1;
                let close = balanced_end(code, open).expect("the braces close");
                for field in top_level_split(&code[open + 1..close], ',') {
                    let field = field.start + open + 1..field.end + open + 1;
                    let text = &code[field.clone()];
                    if !text.trim_start().starts_with("detail:") {
                        continue;
                    }
                    let value_at = field.start + text.find("detail:").unwrap() + "detail:".len();
                    let arg = view[value_at..field.end].trim();
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
