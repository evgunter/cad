//! **Every refusal the feature tree draws fits where it is drawn.**
//!
//! The feature tree's fault line and the status line render a failed
//! node's `NodeError` `Display` verbatim, so each `NodeErrorKind` arm is
//! a sentence the person holding the mouse reads, and a forwarding arm
//! (`Extrude(e)`, `Split(e)`, `Mate(fault)`, …) reads the forwarded
//! refusal's own sentence inside its wrapper. These rows render every
//! `NodeErrorKind` arm, and the arms of what it forwards as far as the
//! next paragraph says, the way the viewer draws it — `node 5 failed:`
//! and the wrapper included — on a representative payload, and hold
//! each to the budget [`refusal_concision`](crate::refusal_concision) states.
//!
//! **What "every arm" covers.** Every `NodeErrorKind` arm has a row.
//! A forwarding arm's payload is welded level by level, as deep as
//! `forward` names a roster: `Split(e)` to `SplitError`'s arm and on to
//! the stage refusal it carries, `SelectResolve` to `ResolveError` and a
//! vanished name's `Diagnosis` (`every_forwarded_arm_has_a_row`). Below
//! a level's last roster — an `EulerOpError` inside `ExtrudeError::Op`,
//! a `PcurveMintError` inside `RevolveError::Pcurve` — a refusal is
//! rendered on one representative arm. The arms `forward` files as
//! unwelded (the D10-held mate, pose, part and placement refusals, and
//! the Boolean's) render here on samples, each citing its row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::NodeStanding;
use editor_core::{NodeError, NodeErrorKind, RecipeNodeId};
use test_utils::refusal::Admission;
use test_utils::refusal::tagged;

/// A `NodeErrorKind` as the feature tree's fault line draws it.
pub(crate) fn as_the_viewer_shows_it(kind: NodeErrorKind) -> String {
    NodeError {
        node: RecipeNodeId::new(0, tagged(5)),
        kind,
        escalations: std::sync::Arc::new(Vec::new()),
    }
    .to_string()
}

/// The rows that may name an arena key: each reports a corrupt body, a
/// kernel invariant or a kernel finding, where the key is what the bug
/// report needs. Every other row names what it is about in words.
const KERNEL_KEYED: &[&str] = &[
    "Revolve/VoidInsertion",
    "Revolve/Pcurve",
    "Extrude/Pcurve",
    "Split/Reduce/ConsecutiveOnSectors",
    "Split/Reduce/StaleVertex",
    "Split/Reduce/LoneVertex",
    "Split/Reduce/NullEdgeAtVertex",
    "Split/Reduce/UnrecordedSide",
    "Split/Reduce/UnboundedFace",
    "Split/Reduce/CrossingInsertion",
    "Split/Reduce/LineageRow",
    "Split/Join/LineageRow",
    "Boolean/Join/LineageRow",
    "Split/Join/SectionLoopMixed",
    "Split/Join/CutInvariant",
    "Split/Join/Corrupt",
    "Split/Join/SectionInvariant",
    "Boolean/Join/SectionLoopMixed",
    "Boolean/Join/CutInvariant",
    "Boolean/Join/Corrupt",
    "Boolean/Join/SectionInvariant",
    "Split/Finish/TornComponent",
    "Split/Finish/UnclassifiableComponent",
    "Split/Finish/NestingContradiction",
    "Split/Finish/SlitUnplaced",
    "Split/Finish/ResultInvalid",
    "Split/Pcurves",
    "Transform/Pcurve",
    "UnfinishedOperand/ScaffoldingEmptyLoop",
    "UnfinishedOperand/ScaffoldingStrutVertex",
    "Loft/Pcurve",
    "Blend/BodyNotIntact/Unresolved",
    "Blend/BodyNotIntact/Unwalked",
    "Blend/BodyNotIntact/Disagrees",
    "Blend/BodyNotIntact/Scaffolding",
    "Blend/BodyNotIntact/Degenerate",
    "Blend/SurgeryInvariant",
    "Blend/Certify",
    "Naming/SplitLineage",
    "Naming/UnsupportedSeamVertex",
    "Naming/SeamVertexPartners",
    "Naming/SharedRim",
    "Naming/ChordOffRim",
    "Naming/ConventionalVertex",
    "Naming/ClosedCarrierUnread",
    "Naming/CarrierUnplaced",
    "Naming/PartnerUnread",
    "Naming/UnsupportedCrossingSenses",
    "Naming/UnsupportedSeamSides",
    "Naming/UnsupportedJoin",
    "Naming/UnsupportedTurnedCell",
    "Naming/WayInZeroBand",
    "Naming/JoinedPieceOffRoot",
    "Naming/BandVerdictsDisagree",
    "Shell/Partition",
    "Shell/Insert",
    "Shell/Rim",
];

/// The rows admitted over the word budget, each at the most words it
/// renders today and filed with its owner: a row over its ceiling, or
/// under it with the ceiling standing, is red.
const OVER_BUDGET: &[(&str, usize)] = &[
    // work/profiletail/profile-arms-the-weld-renders-fall-short-of-the-shape-guard.md
    ("Profile/ArcBelowSceneResolution", 85),
    // work/honeword/split-finish-arms-the-weld-renders-fall-short-of-the-shape-guard.md
    ("Split/Finish/ResultEscalated", 90),
];

/// The clause labels a surface legitimately opens with that read, by
/// shape, like a stage prefix — a clause with no word only a sentence
/// has — each on the row namespace whose surface writes it. A label
/// here is English the person reads, not a pipeline stage.
pub(crate) const ALLOWED_LABELS: &[(&str, &str)] = &[
    // A check finding's labels as its own `Display` says them, with no
    // document at hand (`check at-rest: placement 000000000004 output
    // 0: …`): the check the person ran, named as the menu names it, and
    // the placement it ran on, by its tag.
    ("Check/", "check at-rest"),
    ("Check/", "check connectedness"),
    ("Check/", "check chart-coherence"),
    ("Check/", "placement 000000000004 output 0"),
    // The mate solve names the constraint it refused (`constraint #9 of
    // node 7: …`).
    ("Mate/", "constraint #9 of node 000000000007"),
    // A pair's corner list names each corner it could not fillet.
    ("ProfileReplay/Path/NoCornerOfPair(", "at corner"),
];

/// The rows whose stage prefix is filed with its owner, each with the
/// one prefix it may still carry. An exact row id and an exact label:
/// a new prefix on the same row, or the same prefix on another row, is
/// still red.
pub(crate) const FILED: &[(&str, &str)] = &[
    // A stage for a subject (`<gerund> … refused:`), each on its
    // owner's row:
    // work/hone/reach-refusals-short-of-the-shape-guard.md
    (
        "Split/Reduce/CrossingInsertion",
        "inserting the plane crossing on edge EdgeKey    refused",
    ),
    // work/carvetail/carve-refusals-short-of-the-shape-guard.md
    (
        "Revolve/VoidInsertion",
        "inserting the cavity of hole loop 1 refused",
    ),
    // work/profilerest/paths-refusals-short-of-the-shape-guard.md
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
    // work/carvetail/carve-refusals-short-of-the-shape-guard.md
    ("Blend/SurgeryInvariant", "at face FaceKey"),
    // work/honeword/split-finish-arms-the-weld-renders-fall-short-of-the-shape-guard.md
    ("Split/Finish/Merge", "merge_coplanar_faces"),
    // work/issues/unowned-viewer-refusals-short-of-the-shape-guard.md
    (
        "Naming/SplitLineage",
        "split lineage of edge EdgeKey  cycles",
    ),
];

/// Row namespaces admitted a prefix by namespace rather than row by
/// row, each with the one label its rows may carry, and — where the
/// flag says so — leave to name a key. Any other prefix, a `Debug`
/// struct, or a row outside the namespace is still red.
pub(crate) const FILED_NAMESPACES: &[(&str, &str, bool)] = &[];

/// The split rows that may still offer "declare", which a split has no
/// door for, each filed with its owner (the note above `FILED`'s
/// `EllipseInvalid` entries).
pub(crate) const FILED_DECLARE: &[&str] = &[];

/// The rows that render a key or a hex id, by exact row id and the
/// exact span, each filed with its owner.
pub(crate) const ADMISSIONS: &[Admission<'static>] = &[
    // `EngineRefusal::say` (`topo/src/clearance.rs`) names a face by
    // its side and its `FaceKey` `Debug`, and this arm says it.
    Admission {
        row: "MeasureClearanceRefused/Unsupported",
        span: "FaceKey(null)",
        filed: "work/cleartail/measure-clearance-refused-states-no-recourse-and-prints-its-goldening-payload.md",
    },
    Admission {
        row: "MeasureClearanceRefused/PoisonEnclosure",
        span: "FaceKey(null)",
        filed: "work/cleartail/measure-clearance-refused-states-no-recourse-and-prints-its-goldening-payload.md",
    },
    // A smooth-join refusal names its edge by key.
    Admission {
        row: "Extrude/SmoothJoinRefuted",
        span: "EdgeKey(null)",
        filed: "work/carvetail/sweep-arms-the-weld-renders-fall-short-of-the-shape-guard.md",
    },
    Admission {
        row: "Revolve/SmoothJoinRefuted",
        span: "EdgeKey(null)",
        filed: "work/carvetail/sweep-arms-the-weld-renders-fall-short-of-the-shape-guard.md",
    },
    // A shell-role escalation names the aimed ray's faces by key.
    Admission {
        row: "Split/Finish/ResultEscalated",
        span: "FaceKey(null) and FaceKey(null)",
        filed: "work/honeword/split-finish-arms-the-weld-renders-fall-short-of-the-shape-guard.md",
    },
    // A merge refusal names its edge by key.
    Admission {
        row: "Split/Finish/Merge",
        span: "EdgeKey(null)",
        filed: "work/honeword/split-finish-arms-the-weld-renders-fall-short-of-the-shape-guard.md",
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
    "CrossingUnverified",
    "CurvedSolidFrontier",
    "DegenerateDirection",
    "EmptyHalf",
    "EmptyOperand",
    "MeasureNotParallel",
    "MeasureRefUnreadable/Readback",
    // `NodeStanding`'s failed and poisoned repairs, which a mate
    // cluster's refusal makes false:
    // work/wirehold/kernel-standing-names-a-cluster-refused-node-as-its-own-failure.md
    "MeasureRefUnreadable/NodeFailed",
    "MeasureRefUnreadable/NodePoisoned",
    "MeasureUnsupported",
    "MissingInput",
    "ParamBox/AxisUnrepresentable",
    "ParamBox/UnknownParam",
    // work/wire/wire-refusals-short-of-the-shape-guard.md
    "ShellLaneUnsupported",
    "ToleranceConflict",
    "UnschedulableCycle",
    "WrongOperand",
    // work/profilerest/paths-refusals-short-of-the-shape-guard.md
    "Profile/DegenerateSegment",
    "Profile/EmptyLoop",
    "Profile/EmptyProfile",
    "Profile/MultipleOuterLoops",
    "Profile/NearFullArc",
    "Profile/NestingTooDeep",
    "Profile/NonSimple",
    "Profile/RayCastingExhausted",
    "Profile/SliverLoop",
    "Profile/TangencyContradicted",
    "ProfileReplay/Path/ArcCenterNotEquidistant",
    "ProfileReplay/Path/ArcLegOnOpenFillet",
    "ProfileReplay/Path/ArcViaCollinear",
    "ProfileReplay/Path/CircleSplitCount",
    "ProfileReplay/Path/DegenerateArcCenter",
    "ProfileReplay/Path/DegenerateArcSpec",
    "ProfileReplay/Path/ArcSweepNotShortOfFullTurn",
    "ProfileReplay/Path/NoCornerForFillet",
    "ProfileReplay/Path/NoCornerForFillet(disjoint)",
    "ProfileReplay/Path/NoCornerOfPair",
    "ProfileReplay/Path/NonpositiveCircleRadius",
    "ProfileReplay/Path/NonpositiveFilletRadius",
    "ProfileReplay/Path/NonpositiveLeg",
    "ProfileReplay/Path/OverdeterminedJunction",
    "ProfileReplay/Path/PolygonTooFewVertices",
    "ProfileReplay/Path/SeamRetrimsArcFirstSide",
    "ProfileReplay/Path/UnderdeterminedLeg",
    "ProfileReplay/Path/ZeroDirection",
    "ProfileReplay/Transition",
    // work/cleartail/measure-clearance-refused-states-no-recourse-and-prints-its-goldening-payload.md
    "MeasureClearanceRefused/EmptyScope",
    "MeasureClearanceRefused/PoisonEnclosure",
    "MeasureClearanceRefused/Unsupported",
    // work/profiletail/profile-arms-the-weld-renders-fall-short-of-the-shape-guard.md
    "Profile/InconsistentArc",
    // work/honeword/split-finish-arms-the-weld-renders-fall-short-of-the-shape-guard.md
    "Split/Finish/Merge",
    // work/carvetail/sweep-arms-the-weld-renders-fall-short-of-the-shape-guard.md
    "Revolve/PinnedRunStation",
    // work/hone/reach-refusals-short-of-the-shape-guard.md
    "Boolean/Join/Corrupt",
    "Boolean/Join/CutInvariant",
    "Boolean/Join/SectionInvariant",
    "Boolean/Join/SectionLoopMixed",
    "Boolean/Join/UnpairedLooseEnds",
    "Split/Finish/Corrupt",
    "Split/Finish/DegenerateSide",
    "Split/Finish/TornComponent",
    "Split/Finish/UnclassifiableComponent",
    "Split/Join/Corrupt",
    "Split/Join/CutInvariant",
    "Split/Join/SectionInvariant",
    "Split/Join/SectionLoopMixed",
    "Split/Join/UnpairedLooseEnds",
    "Split/Reduce/ConsecutiveOnSectors",
    "Split/Reduce/StaleVertex",
    "Split/Reduce/LoneVertex",
    "Split/Reduce/NullEdgeAtVertex",
    "Split/Reduce/UnrecordedSide",
    "Split/Reduce/UnboundedFace",
    // work/carvetail/carve-refusals-short-of-the-shape-guard.md
    "Blend/SurgeryInvariant",
    "Extrude/CapPlane",
    "Extrude/SidePlane",
    "Loft/CapPlane",
    "Loft/SectionStructure",
    "Revolve/CapPlane",
    "Revolve/FullRangeAngle",
    "Revolve/VoidInsertion",
    "Skin/BadDegree",
    "Skin/DomainNotUnit",
    "Skin/SectionProfile",
    "Tube/DegenerateWindow",
    "Tube/FullRangeWindow",
    // work/issues/unowned-viewer-refusals-short-of-the-shape-guard.md
    "Check/ChartCoherence(meridian closure)",
    "Check/ChartCoherence(rim)",
    "Check/ChartCoherenceUnexamined(corrupt)",
    "Check/ChartCoherenceUnexamined(non-iso)",
    "Check/ChartCoherenceUnexamined(scaffold)",
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
        let words = text.split_whitespace().count();
        eprintln!("MEASURE {words} {name}: {text}");
        let ceiling = OVER_BUDGET
            .iter()
            .find_map(|(row, most)| (*row == name.as_str()).then_some(*most));
        let breach = format!(
            "{name} renders {words} words, over {}",
            test_utils::refusal::BUDGET
        );
        for problem in test_utils::refusal::problems_admitting(name, text, &[], false, ADMISSIONS) {
            if let Some(most) = ceiling.filter(|_| problem.starts_with(&breach)) {
                if words > most {
                    problems.push(format!(
                        "{name} renders {words} words, over its admitted {most}: {text}"
                    ));
                } else {
                    used.insert(format!("OVER_BUDGET {name}"));
                }
                continue;
            }
            match admission(name, text, &problem) {
                Some(entry) => {
                    used.insert(entry);
                }
                None => problems.push(problem),
            }
        }
        if let Some(most) = ceiling.filter(|most| words < *most) {
            problems.push(format!(
                "{name} renders {words} words; lower its admission of {most}"
            ));
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
            geom_core::SPLIT_PLANE_RECOURSE,
            geom_core::COINCIDENCE_RECOURSE,
        ]
    );
}

/// A vanished name's sentence ends as its diagnosis says: a cascade
/// sends the reader to the name upstream, and a diagnosis that states
/// its own repair (the sketch's branch selection) is not followed by a
/// second one.
#[test]
fn a_vanished_names_ending_is_its_diagnosiss() {
    let rows: std::collections::BTreeMap<String, String> = editor_payloads()
        .into_iter()
        .map(|(name, kind)| (name, as_the_viewer_shows_it(kind)))
        .collect();
    let cascade = &rows["SelectResolve/Vanished(Cascade)"];
    assert!(
        cascade.ends_with(
            "a face it derives from vanished upstream first, and that is the root cause. \
             Recourse: repair the face it derives from first"
        ),
        "{cascade}"
    );
    let witness = &rows["SelectResolve/Vanished(WitnessBifurcation)"];
    assert!(
        witness.ends_with(
            "Recourse: move the sketch off the near-singular configuration, or re-solve to \
             record a fresh witness"
        ),
        "{witness}"
    );
    for text in [cascade, witness] {
        assert_eq!(text.matches("Recourse:").count(), 1, "{text}");
    }
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
        OVER_BUDGET
            .iter()
            .map(|(n, _)| format!("OVER_BUDGET {n}"))
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

    /// A refusal an operator gives a kernel driver: a fact about the
    /// operation, never a bad argument, which a driver's own keys cannot
    /// be ([`topo::EulerOpError::from_driver`]).
    pub(super) fn euler() -> topo::EulerOpError {
        topo::EulerOpError::DescriptionNotAdjacent { edge: None }
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

/// The chains that carry an operator's refusal out of a kernel
/// driver. None is filed short of a recourse: a driver's keys are ones
/// it read or minted, so an argument miss never reaches one typed
/// ([`topo::EulerOpError::from_driver`] panics on it), and what reaches
/// them is an operation refusal, which states its own recourse. That
/// recourse is addressed to the driver's call, not to the person at
/// the GUI — on [`payloads::euler`] it reads "describe it against the
/// surfaces its faces wear" — which is
/// `work/honeword/a-driver-carried-operation-refusal-shows-the-user-the-drivers-recourse.md`.
const DRIVER_CARRIED: &[&str] = &[
    "Blend/Op",
    "Boolean/Join/Euler",
    "Extrude/Op",
    "Loft/Euler",
    "Revolve/Op",
    "Shell/Face/Op",
    "Shell/Face/Op(rechart)",
    "Shell/Partition",
    "Shell/Rim",
    "Split/Finish/Euler",
    "Split/Join/Euler",
    "Split/Reduce/CrossingInsertion",
    "Split/Reduce/Euler",
];

#[test]
fn every_driver_carried_chain_ends_in_the_operators_own_recourse() {
    let carried = payloads::euler().to_string();
    let (_, recourse) = carried
        .split_once("Recourse: ")
        .expect("the sample operation refusal states a recourse");
    let rows: Vec<(String, String)> = node_refusals()
        .into_iter()
        .filter(|(name, _)| DRIVER_CARRIED.contains(&name.as_str()))
        .map(|(name, kind)| (name, as_the_viewer_shows_it(kind)))
        .collect();
    assert_eq!(
        rows.len(),
        DRIVER_CARRIED.len(),
        "every driver-carried chain is rendered: {:?}",
        rows.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );
    for (name, text) in &rows {
        assert!(
            text.ends_with(recourse) && !text.contains("kernel bug"),
            "{name} draws the carried operation refusal's recourse, and no kernel-bug \
             claim beside it: {text}"
        );
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
        // Every arm whose split rendering offers the join's SHARED
        // recourse offers the declaration under the Boolean instead.
        //
        // The shared one is the escalated arm's: `Indeterminate::ending`
        // composed over the levers the door has, so the split's text
        // carries the margin payload beside it. An arm with a lever of
        // its own — "move the geometry", for a definite verdict no
        // declaration would change — is NOT it, and the levers alone no
        // longer tell the two apart now that `COINCIDENCE_RECOURSE`
        // is the lever and not a sentence.
        if let Some(arm) = name.strip_prefix("Boolean/Join/") {
            let split = rows
                .iter()
                .find(|(n, _)| *n == format!("Split/Join/{arm}"))
                .map(|(_, t)| t)
                .expect("every Boolean join row has its split twin");
            let escalated = [
                "lies inside the ambiguity band",
                "lies within the zero band",
                "cannot be classified against",
                "margin is invalid",
            ]
            .iter()
            .any(|p| split.contains(p));
            if escalated && split.contains(geom_core::COINCIDENCE_RECOURSE) {
                assert!(
                    text.contains(geom_core::COINCIDENCE_RECOURSE),
                    "{name}: {text}"
                );
            }
        }
    }
}

use editor_core::clearance::MinSeparationRefusal;
use editor_core::{BifurcationKind, Diagnosis, EvalError, ParamBoxError, ResolveError, SeedError};

test_utils::f6_variants! {
    /// `EvalError`'s arms, for the `Expr/`, `PoseScalar/`,
    /// `MeasureNonFinite/` and `PayloadExpr/` rows.
    const EVAL_ERROR: EvalError = [
        UnresolvedVar,
        OutputRead,
        DefinitionRefused,
        IndexOutsideMember,
        ModByNonPositive,
        VarKindMismatch,
        CountExprInContinuousEval,
        ContinuousExprInCountEval,
        CountOverflow,
        CountToScalarOutOfRange,
        NonFiniteResult,
        Unlowered,
    ];
}

test_utils::f6_variants! {
    /// `ParamBoxError`'s arms, for the `ParamBox/` rows.
    const PARAM_BOX_ERROR: ParamBoxError = [UnknownParam, AxisUnrepresentable, SymbolsOverABox];
}

test_utils::f6_variants! {
    /// `SeedError`'s arms, for the `Seed/` rows.
    const SEED_ERROR: SeedError = [UnknownVar, CountVar, SeedOnNonFreeVar, TangentUnrepresentable];
}

test_utils::f6_variants! {
    /// `ResolveError`'s arms, for the `SelectResolve/` rows.
    const RESOLVE_ERROR: ResolveError = [Vanished, Ambiguous, ReadGone, NodeGone];
}

test_utils::f6_variants! {
    /// `NodeErrorKind`'s arms: the roster [`forward`] classifies and the
    /// census rows are checked against, so an arm with no row reds and
    /// an arm added to the enum stops this compiling until it is here.
    const NODE_ERROR_KIND: NodeErrorKind = [
        Expr, Profile, ProfileReplay, ProfileLaneReplay, Trace, Extrude, Revolve, Tube, Split, ContactCarry, Blend,
        Boolean, Transform, Mirror, Motion, Skin, Loft, CurvedSolidFrontier, MissingInput, UnresolvedRead, SelectResolve,
        ToleranceConflict, ParamBox, Seed, WrongOperand, EmptyOperand,
        UnfinishedOperand, EmptyHalf, MemberOutOfRange, MemberRefused, IndicesUnrelated,
        IndexCount, IndexCountOutOfRange, NoSuchPiece, PieceShape,
        DegenerateDirection, NonFiniteDirection, UnderflowedDirection, Band, MissingSlot,
        Escalated, MeasureEscalated, PlacementAxisEscalated, PoseDirectionEscalated,
        PoseRead, PoseDegenerate, PoseScalar, CountOutOfRange, UnschedulableCycle,
        Naming, UnionFoldStep, Shell, ShellLaneUnsupported,
        WitnessBifurcation, Part, Mate,
        SpaceMismatch, CrossingUnverified, MeasureRefUnreadable, MeasureNonFinite,
        MeasureNotParallel, MeasureUnsupported, PayloadExpr, MeasureClearanceRefused,
        AssertionDimension,
    ];
}

/// The kernel refusals the census welds, each an `f6_variants!` roster
/// for the rows named in its doc. A generic one is read at `f64`, the
/// scalar every row is built at.
mod kernel {
    use profile::ProfileError;
    use sweep::ExtrudeError;
    use sweep::LoftError;
    use sweep::RevolveError;
    use sweep::SkinError;
    use sweep::TubeError;
    use sweep::blend::BlendError;
    pub(super) type ReplayErrorKind = profile::ReplayErrorKind<f64>;
    pub(super) type PathError = profile::path::PathError<f64>;
    use editor_core::TraceStepFault;
    use profile::StructureRefusalKind;
    use topo::SplitError;
    use topo::SplitFinishError;
    use topo::SplitJoinError;
    use topo::SplitReduceError;
    use topo::boolean::trace::TraceFault;
    pub(super) type ShellError = topo::ShellError<f64>;
    pub(super) type ReplaceFaceError = topo::ReplaceFaceError<f64>;
    use geom_core::BandError;

    test_utils::f6_variants! {
        /// `ExtrudeError`'s arms, for the `Extrude/` rows.
        pub(super) const EXTRUDE_ERROR: ExtrudeError = [
            Band, DegenerateExtrusion, NegativeDepth, ObliqueExtrusion,
            ExtrusionEscalated, CosurfaceEscalated, SliverJoin, SliverRim,
            SmoothJoinRefuted, CapPlane, SidePlane, Op, Pcurve,
        ];
    }

    test_utils::f6_variants! {
        /// `RevolveError`'s arms, for the `Revolve/` rows.
        pub(super) const REVOLVE_ERROR: RevolveError = [
            Band, NonFiniteAxis, UnderflowedAxis, DegenerateAxis, AxisEscalated, Turn,
            DegenerateAngle, FullRangeAngle, AngleEscalated, VertexCrossesAxis,
            SliverRadius, ArcCrossesAxis, SliverAxisClearance, UnsupportedToroid,
            NonManifoldAxisContact, AxisGapEscalated, HoleTouchesAxis, VoidInsertion,
            CosurfaceEscalated, SliverJoin, SliverRim, SmoothJoinRefuted,
            PinnedRunStation, CapPlane, Op, Pcurve,
        ];
    }

    test_utils::f6_variants! {
        /// `TubeError`'s arms, for the `Tube/` rows.
        pub(super) const TUBE_ERROR: TubeError = [
            Band, DegenerateWindow, FullRangeWindow, NonpositiveWall, WallExceedsRadius,
            WallGapCollapsed, Escalated, Revolve,
        ];
    }

    test_utils::f6_variants! {
        /// `SkinError`'s arms, for the `Skin/` rows.
        pub(super) const SKIN_ERROR: SkinError = [
            TooFewSections, SectionShapeMismatch, SectionProfile, DomainNotUnit,
            DegenerateSection, NoParameterStep, BadDegree, PathTangentVanishes,
            PathStationsCoincide, PathFoldsBack, PathUndecided, StationNotFinite, Band,
            RowMotionUndecided, ArcSweepUndecided, Fit, KnotAlgebra, Structure,
        ];
    }

    test_utils::f6_variants! {
        /// `LoftError`'s arms, for the `Loft/` rows.
        pub(super) const LOFT_ERROR: LoftError = [
            Band, Skin, Euler, CapPlane, Pcurve, SeamStructure, SectionStructure,
            SectionRim, SectionNotFinite, PlacementNotFinite, ReversedStacking, DegenerateStacking,
            PinnedCorner, PinnedCornerUnsupported, CornerEscalated, SectionsTouch,
            ClearanceUndecided, ClearanceFault, FarSectionNotForward, FarStackingEscalated,
            StackingEscalated,
        ];
    }

    test_utils::f6_variants! {
        /// `BlendError`'s arms, for the `Blend/` rows.
        pub(super) const BLEND_ERROR: BlendError = [
            Band, ChainNotConnected, RadiusHeadroom, FaceClearanceUncertified,
            FaceClearance, TangentialEdge, SpineIrregular, ChainNotG1,
            ConvexitySignFlip, UnsupportedCorner, SpineUnsupported,
            ChamferArmUnsupported, Escalated, RepeatedEdge, NonpositiveSize,
            UnsupportedChain, UnsupportedRunOut, UnsupportedGeometry, BodyNotIntact,
            ScaffoldingOperand, InsideOutOperand, UnjoinedOperand, SurgeryInvariant,
            RingClearance, WhollyInside, Certify, Op, Join,
        ];
    }

    test_utils::f6_variants! {
        /// `ProfileError`'s arms, for the `Profile/` rows.
        pub(super) const PROFILE_ERROR: ProfileError = [
            Band, EmptyProfile, PlaneNotFinite, EmptyLoop, DegenerateSegment,
            NearFullArc, InconsistentArc, ArcBelowSceneResolution, NonSimple,
            TangentialContact, TangentJointOutOfRange, TangentJointOnFullTurn,
            TangencyContradicted, SliverLoop, MultipleOuterLoops, NestingTooDeep,
            RayCastingExhausted, Escalated, Structure,
        ];
    }

    test_utils::f6_variants! {
        /// `ReplayErrorKind`'s arms, for the `ProfileReplay/` rows.
        pub(super) const REPLAY_ERROR_KIND: ReplayErrorKind = [
            Transition, Path,
        ];
    }

    test_utils::f6_variants! {
        /// `PathError`'s arms, for the `ProfileReplay/Path/` rows.
        pub(super) const PATH_ERROR: PathError = [
            JunctionTangent, JunctionCusp, SeamTangent, SeamArrivalOffDirection,
            SeamArrivalLeverTooShort, ContinuationTargetOffRay, NoCornerForFillet,
            NoCornerOfPair, FilletOffsetLeverTooShort, FilletArcFlattenedInStorage,
            FilletCarrierBelowSceneResolution, ArcLegOnOpenFillet,
            SeamRetrimsArcFirstSide, NonpositiveLeg, NonpositiveFilletRadius,
            NonpositiveCircleRadius, DegenerateArcSpec, ArcSweepNotShortOfFullTurn,
            CircleSplitCount, PolygonTooFewVertices, ZeroDirection, NonFiniteDirection,
            UnderflowedDirection, ArcViaCollinear, DegenerateArcChord,
            ArcCenterNotEquidistant, DegenerateArcCenter, ArcMissesAuthoredPoint,
            ArcMissBelowSceneResolution, FarEndAnchorWithoutFillet,
            Escalated, Band, Structure, UnderdeterminedLeg, OverdeterminedJunction,
        ];
    }

    test_utils::f6_variants! {
        /// `StructureRefusalKind`'s arms, for the `ProfileLaneReplay/` rows.
        pub(super) const STRUCTURE_REFUSAL_KIND: StructureRefusalKind = [
            Indeterminate, Flipped,
        ];
    }

    test_utils::f6_variants! {
        /// `TraceStepFault`'s arms, for the `Trace/` rows.
        pub(super) const TRACE_STEP_FAULT: TraceStepFault = [
            Undrawn, Unevaluated, NoSuchFace, NoPen, NoHeading, NotARun, OffTrace, NoTrace, NoSuchCrossing,
            NoTangent, Unreached, Carrier,
        ];
    }

    test_utils::f6_variants! {
        /// `TraceFault`'s arms, for the `Trace/Carrier/` rows.
        pub(super) const TRACE_FAULT: TraceFault = [
            Unsupported, Grazing, Apex, OnCarrier, Escalated, Unsettled,
        ];
    }

    test_utils::f6_variants! {
        /// `SplitError`'s arms, for the `Split/` rows.
        pub(super) const SPLIT_ERROR: SplitError = [
            Reduce, Join, Finish, Pcurves, Pieces,
        ];
    }

    test_utils::f6_variants! {
        /// `SplitReduceError`'s arms, for the `Split/Reduce/` rows.
        pub(super) const SPLIT_REDUCE_ERROR: SplitReduceError = [
            Band, CurvedBooleanUnsupported, CurvedEdgeUnsupported, CrossingEscalated,
            TangencyUnsupported, KnifeEdge, ScaffoldingOperand, InsideOutOperand,
            UnjoinedOperand, SliverVertex, SliverSector, SectionGermsEscalated,
            SectionGermsUnordered, ApexSection, ApexUndecided, NonFiniteSectorChord,
            UnderflowedSectorChord,
            ConsecutiveOnSectors, StaleVertex, LoneVertex, NullEdgeAtVertex,
            UnrecordedSide, UnboundedFace, CrossingInsertion, Euler, LineageRow,
        ];
    }

    test_utils::f6_variants! {
        /// `SplitJoinError`'s arms, for the `Split/Join/` rows.
        pub(super) const SPLIT_JOIN_ERROR: SplitJoinError = [
            OrderEscalated, Escalated, DegenerateSection, CurvedSlitTrace, RingHoming,
            RingHomingAmbiguous, UnpairedLooseEnds, SingleSiteSectionLoop,
            SectionLoopMixed, SectionLoopUndecided, CutInvariant, Corrupt, LineageRow,
            Band, Euler, SectionEndEscalated, SectionUnresolved, Section, OpenConicChord,
            ApexUnlifted,
            PoleEdgesCoincide, SectionInvariant, SectionUnarmed, RunImageNotYet,
            RunImageIncidence, RingIslandUnread,
            RunEdgeCarrierUnsupported,
            TangentChordEscalated, TangentChordBendEscalated, TangentChordRefuted,
            SectionCrossings, SectionDepartureGrazes,
        ];
    }

    test_utils::f6_variants! {
        /// `SplitFinishError`'s arms, for the `Split/Finish/` rows.
        pub(super) const SPLIT_FINISH_ERROR: SplitFinishError = [
            DegenerateSide, TornComponent, UnclassifiableComponent, Corrupt, Euler,
            Band, ConeCornerEscalated, ConeCornerUnread, ConeFusionDesync, DescribeEscalated, DescribeSideEscalated, DescribeBendEscalated,
            EdgeOnFacesEscalated, SmoothJoinRefuted, SectionWindingUndecided, KnifeEdge,
            NestingContradiction, SlitUnplaced, SlitHoming, ResultInvalid,
            ResultNotAtRest, ResultEscalated, EdgeJoin, Merge,
        ];
    }

    test_utils::f6_variants! {
        /// `ShellError`'s arms, for the `Shell/` rows.
        pub(super) const SHELL_ERROR: ShellError = [
            Band, Thickness, NoSolid, Roles, Partition, WallClearance, OffsetsCross,
            CavityCrosses, CavityUncertified,
            ChartSenseMixed, Face, OpenFaceStale, OpenFaceRepeated,
            OpenFacesExhaustShell, OpenFacesDisconnect, OpenFaceChartPartial, Lift,
            Insert, OpenFaceRimNotExpressible, Rim, Escalated, Pcurve, RimSeamMixed,
            Join, NotValid,
        ];
    }

    test_utils::f6_variants! {
        /// `ReplaceFaceError`'s arms, for the `Shell/Face/` rows.
        pub(super) const REPLACE_FACE_ERROR: ReplaceFaceError = [
            Band, StaleFace, Offset, Fit, ApproxLaneUnsupported, SharedSurfaceKey,
            EmptyGroup, GroupChartsDiffer, PlaceholderSurface, ApexWindow,
            NappeStraddles, ApexWindowUnknown, NeighborPairUnroutable,
            NeighborPoseUnroutable, FittedBoundaryUnsupported, CarrierLaneUnsupported,
            IsoRow, Structure, VertexDisagreement, EdgeSection, CornerSection,
            DeclaredEdgeTilted, MixedEdge, ReanchorPastCarrierEnd, ReanchorCollapse,
            ReanchorInconclusive, NurbsLaneUnsupported, TogetherNonPlanar,
            TogetherPartialSet, TogetherCorner, TogetherChartMixed,
            TogetherFaceRepeated, TogetherEdgeDisagreement, TogetherAxialUnsupported,
            TogetherNotAxial, TogetherAxialCorner, TogetherAxialEdge, TogetherConicArc,
            DegenerateTorus,
            Escalated, Op, Pcurve, Join, ResultNotClosed,
        ];
    }

    test_utils::f6_variants! {
        /// `BandError`'s arms, for the `Band/` rows.
        pub(super) const BAND_ERROR: BandError = [
            InvalidValue, InvalidLeverArm, Empty,
        ];
    }
}

test_utils::f6_variants! {
    /// `Diagnosis`'s arms, for the `SelectResolve/Vanished(…)` rows.
    const DIAGNOSIS: Diagnosis = [
        PredicateFlip, BorderDelta, GroupResized, StructuralParam, IndexCount, RecipeEdit, Cascade,
        WitnessBifurcation, Upstream,
    ];
}

test_utils::f6_variants! {
    /// `BifurcationKind`'s arms, for the `WitnessBifurcation/` rows.
    const BIFURCATION_KIND: BifurcationKind = [FoldProximity, AmbiguousBasin, ResidualFailure];
}

test_utils::f6_variants! {
    /// `MinSeparationRefusal`'s arms, for the `MeasureClearanceRefused/` rows.
    const MIN_SEPARATION_REFUSAL: MinSeparationRefusal =
        [Unsupported, PoisonEnclosure, EmptyScope];
}

/// Every roster the weld checks, by the refusal it is the census of.
/// [`forward`] names a level's roster here, and the weld reds on an
/// entry no row reached, so an arm reclassified away from its roster
/// drops nothing silently.
const ROSTERS: &[(&str, &[&str])] = {
    use kernel::*;
    &[
        ("EvalError", EVAL_ERROR.identifiers()),
        ("ParamBoxError", PARAM_BOX_ERROR.identifiers()),
        ("SeedError", SEED_ERROR.identifiers()),
        ("ResolveError", RESOLVE_ERROR.identifiers()),
        ("Diagnosis", DIAGNOSIS.identifiers()),
        ("BifurcationKind", BIFURCATION_KIND.identifiers()),
        ("MinSeparationRefusal", MIN_SEPARATION_REFUSAL.identifiers()),
        (
            "NamingError",
            crate::display_contract::NAMING_ERROR.identifiers(),
        ),
        (
            "InterrogateError",
            crate::display_contract::INTERROGATE_ERROR.identifiers(),
        ),
        ("ExtrudeError", EXTRUDE_ERROR.identifiers()),
        ("RevolveError", REVOLVE_ERROR.identifiers()),
        ("TubeError", TUBE_ERROR.identifiers()),
        ("SkinError", SKIN_ERROR.identifiers()),
        ("LoftError", LOFT_ERROR.identifiers()),
        ("BlendError", BLEND_ERROR.identifiers()),
        ("ProfileError", PROFILE_ERROR.identifiers()),
        ("ReplayErrorKind", REPLAY_ERROR_KIND.identifiers()),
        ("PathError", PATH_ERROR.identifiers()),
        ("StructureRefusalKind", STRUCTURE_REFUSAL_KIND.identifiers()),
        ("TraceStepFault", TRACE_STEP_FAULT.identifiers()),
        ("TraceFault", TRACE_FAULT.identifiers()),
        ("SplitError", SPLIT_ERROR.identifiers()),
        ("SplitReduceError", SPLIT_REDUCE_ERROR.identifiers()),
        ("SplitJoinError", SPLIT_JOIN_ERROR.identifiers()),
        ("SplitFinishError", SPLIT_FINISH_ERROR.identifiers()),
        ("ShellError", SHELL_ERROR.identifiers()),
        ("ReplaceFaceError", REPLACE_FACE_ERROR.identifiers()),
        ("BandError", BAND_ERROR.identifiers()),
    ]
};

/// One level of a forwarded payload: the [`ROSTERS`] entry of the
/// refusal it carries at that depth, and the arm this row's payload
/// takes there.
struct Level {
    roster: &'static str,
    arm: String,
}

fn level(roster: &'static str, payload: &impl core::fmt::Debug) -> Level {
    Level {
        roster,
        arm: test_utils::f6::variant_identifier(payload),
    }
}

/// What the census does with one `NodeErrorKind` arm's payload.
enum Forward {
    /// A refusal welded level by level, outermost first: every arm of
    /// each level's roster has a row. When `labelled`, a row is named
    /// `{family}/` and then its arm at each level ([`names_arms`]);
    /// what follows names a sample of a refusal no roster here holds.
    Welded { levels: Vec<Level>, labelled: bool },
    /// A refusal the weld does not cover, and the row that files it.
    Unwelded(&'static str),
    /// A kernel refusal sampled here and rendered arm by arm by
    /// another census: the file that holds it.
    RenderedIn(&'static str),
    /// No forwarded refusal: the arm writes its own sentence over plain
    /// values, or carries one struct whose shape has no arms.
    Own,
}

const UNWELDED_HELD: &str = "work/wirehold/mate-and-pose-forwarders-the-census-does-not-weld.md";
const UNWELDED_PLACEMENT: &str =
    "work/wirehold/placement-and-part-forwarders-the-census-does-not-weld.md";
const UNWELDED_BOOLEAN: &str = "work/tint/boolean-refusal-concision-list-is-kept-by-hand.md";

/// The one classification of every `NodeErrorKind` arm, and of each
/// level below it: `match`es with no wildcard, so a new arm at any
/// welded level is a decision here before it compiles.
fn forward(kind: &NodeErrorKind) -> Forward {
    let welded = |levels| Forward::Welded {
        levels,
        labelled: true,
    };
    let one = |roster, payload: &dyn core::fmt::Debug| welded(vec![level(roster, &payload)]);
    match kind {
        NodeErrorKind::Expr { source, .. }
        | NodeErrorKind::PoseScalar { source, .. }
        | NodeErrorKind::MeasureNonFinite { source }
        | NodeErrorKind::PayloadExpr { source, .. }
        | NodeErrorKind::IndexCount { source, .. } => one("EvalError", source),
        NodeErrorKind::ParamBox { source } => one("ParamBoxError", source),
        NodeErrorKind::Seed { source } => one("SeedError", source),
        NodeErrorKind::Naming(e) => one("NamingError", e),
        NodeErrorKind::SelectResolve { error, .. } => {
            let mut levels = vec![level("ResolveError", error)];
            match &**error {
                ResolveError::Vanished { diagnosis, .. } => {
                    levels.push(level("Diagnosis", diagnosis));
                }
                ResolveError::Ambiguous { .. }
                | ResolveError::ReadGone { .. }
                | ResolveError::NodeGone { .. } => {}
            }
            welded(levels)
        }
        NodeErrorKind::WitnessBifurcation(e) => one("BifurcationKind", &e.kind),
        NodeErrorKind::MeasureClearanceRefused(e) => one("MinSeparationRefusal", e),
        // Labelled by the standing each carries, the names the filed
        // rows cite them by.
        NodeErrorKind::MeasureRefUnreadable { error, .. } => Forward::Welded {
            levels: vec![level("InterrogateError", error)],
            labelled: false,
        },
        NodeErrorKind::Profile(e) => one("ProfileError", e),
        NodeErrorKind::ProfileReplay { error, .. } => {
            let mut levels = vec![level("ReplayErrorKind", &error.kind)];
            match &error.kind {
                profile::ReplayErrorKind::Path(e) => levels.push(level("PathError", e)),
                profile::ReplayErrorKind::Transition { .. } => {}
            }
            welded(levels)
        }
        NodeErrorKind::ProfileLaneReplay { structure, .. } => match structure {
            Some(s) => one("StructureRefusalKind", &s.kind),
            None => welded(Vec::new()),
        },
        NodeErrorKind::Trace(refusal) => {
            let mut levels = vec![level("TraceStepFault", &refusal.fault)];
            if let editor_core::TraceStepFault::Carrier(fault) = &refusal.fault {
                levels.push(level("TraceFault", fault));
            }
            welded(levels)
        }
        NodeErrorKind::Extrude(e) => one("ExtrudeError", e),
        NodeErrorKind::Revolve(e) => one("RevolveError", e),
        NodeErrorKind::Tube(e) => one("TubeError", e),
        NodeErrorKind::Split(e) => {
            let mut levels = vec![level("SplitError", e)];
            match e {
                topo::SplitError::Reduce(e) => levels.push(level("SplitReduceError", e)),
                topo::SplitError::Join(e) => levels.push(level("SplitJoinError", e)),
                topo::SplitError::Finish(e) => levels.push(level("SplitFinishError", e)),
                topo::SplitError::Pcurves(_) | topo::SplitError::Pieces(_) => {}
            }
            welded(levels)
        }
        NodeErrorKind::Blend { error, .. } => one("BlendError", error),
        NodeErrorKind::Skin(e) => one("SkinError", e),
        NodeErrorKind::Loft(e) => one("LoftError", e),
        NodeErrorKind::Band(e) => one("BandError", e),
        NodeErrorKind::Shell(e) => {
            use topo::ShellError as S;
            let mut levels = vec![level("ShellError", e)];
            match &**e {
                S::Face { error, .. } => levels.push(level("ReplaceFaceError", error)),
                S::Band { .. }
                | S::Thickness { .. }
                | S::NoSolid
                | S::Roles { .. }
                | S::Partition { .. }
                | S::WallClearance { .. }
                | S::OffsetsCross { .. }
                | S::CavityCrosses { .. }
                | S::CavityUncertified { .. }
                | S::ChartSenseMixed { .. }
                | S::OpenFaceStale { .. }
                | S::OpenFaceRepeated { .. }
                | S::OpenFacesExhaustShell { .. }
                | S::OpenFacesDisconnect { .. }
                | S::OpenFaceChartPartial { .. }
                | S::Lift { .. }
                | S::Insert { .. }
                | S::OpenFaceRimNotExpressible { .. }
                | S::Rim { .. }
                | S::Escalated { .. }
                | S::Pcurve { .. }
                | S::RimSeamMixed { .. }
                | S::Join { .. }
                | S::NotValid { .. } => {}
            }
            welded(levels)
        }
        NodeErrorKind::Boolean(_) => Forward::Unwelded(UNWELDED_BOOLEAN),
        NodeErrorKind::UnfinishedOperand { .. } => {
            Forward::RenderedIn("crates/editor-core/tests/refusal_concision_at_rest.rs")
        }
        NodeErrorKind::PoseRead { .. }
        | NodeErrorKind::Mate(_)
        | NodeErrorKind::SpaceMismatch { .. } => Forward::Unwelded(UNWELDED_HELD),
        NodeErrorKind::Part { .. }
        | NodeErrorKind::MemberRefused { .. }
        | NodeErrorKind::Transform(_)
        | NodeErrorKind::Mirror(_)
        | NodeErrorKind::Motion(_) => Forward::Unwelded(UNWELDED_PLACEMENT),
        NodeErrorKind::CurvedSolidFrontier { .. }
        | NodeErrorKind::MissingInput { .. }
        | NodeErrorKind::UnresolvedRead { .. }
        | NodeErrorKind::ToleranceConflict { .. }
        | NodeErrorKind::WrongOperand { .. }
        | NodeErrorKind::EmptyOperand { .. }
        | NodeErrorKind::EmptyHalf { .. }
        | NodeErrorKind::ContactCarry { .. }
        | NodeErrorKind::MemberOutOfRange { .. }
        | NodeErrorKind::IndicesUnrelated { .. }
        | NodeErrorKind::NoSuchPiece { .. }
        | NodeErrorKind::PieceShape { .. }
        | NodeErrorKind::DegenerateDirection { .. }
        | NodeErrorKind::NonFiniteDirection { .. }
        | NodeErrorKind::UnderflowedDirection { .. }
        | NodeErrorKind::MissingSlot { .. }
        | NodeErrorKind::Escalated { .. }
        | NodeErrorKind::MeasureEscalated { .. }
        | NodeErrorKind::PlacementAxisEscalated { .. }
        | NodeErrorKind::PoseDirectionEscalated { .. }
        | NodeErrorKind::PoseDegenerate { .. }
        | NodeErrorKind::CountOutOfRange { .. }
        | NodeErrorKind::IndexCountOutOfRange { .. }
        | NodeErrorKind::UnschedulableCycle
        | NodeErrorKind::UnionFoldStep { .. }
        | NodeErrorKind::ShellLaneUnsupported { .. }
        | NodeErrorKind::CrossingUnverified { .. }
        | NodeErrorKind::MeasureNotParallel { .. }
        | NodeErrorKind::MeasureUnsupported(_)
        | NodeErrorKind::AssertionDimension { .. } => Forward::Own,
    }
}

/// Whether `label` names `arms` in order, outermost first: each arm
/// ends at the label's end, at a `/`, at a `(` that opens the next
/// level, or at the `)` closing one opened before it, so a level reads
/// as `{arm}/{arm}` or `{arm}({arm})`. What follows the last is free.
fn names_arms(label: &str, arms: &[&str]) -> bool {
    let (mut rest, mut open) = (label, 0usize);
    for arm in arms {
        let Some(after) = rest.strip_prefix(arm) else {
            return false;
        };
        let mut chars = after.chars();
        match chars.next() {
            None | Some('/') => {}
            Some('(') => open += 1,
            Some(')') if open > 0 => open -= 1,
            Some(_) => return false,
        }
        rest = chars.as_str();
    }
    true
}

/// **Every `NodeErrorKind` arm has a row, and so does every arm of
/// each refusal [`forward`] welds, at every level it welds**, and every
/// [`ROSTERS`] entry is reached by some row.
///
/// Each roster is an `f6_variants!` census, whose `match` rustc holds
/// exhaustive (`NamingError`'s and `InterrogateError`'s are
/// `display_contract`'s). [`forward`] picks each level's roster by an
/// exhaustive `match`, and a roster no row reaches is reported, so an
/// arm moved off its roster (to `Own`, or a level dropped) reds rather
/// than leaving the roster's arms unchecked. The family a payload's
/// arms are checked under is its row's `NodeErrorKind` arm followed by
/// the arms of the levels above it (`Split/Reduce`), so a family cannot
/// drop out of the check while the arm above it has a row, and the
/// first census makes every arm have one.
///
/// What it does not weld: the arms [`forward`] files as `Unwelded` or
/// `RenderedIn`, and any refusal below a level's last roster (a
/// `PcurveMintError` inside `Split/Pcurves`, the `RevolveError` inside
/// `Tube/Revolve`), each rendered on one sample.
///
/// Each set difference guards one direction that can fire: an arm with
/// no row. The other — a row whose arm is not in the census — cannot,
/// since the census's `match` names every arm; it is left in place for
/// a `Debug` written by hand, which `variant_identifier` would misread.
#[test]
fn every_forwarded_arm_has_a_row() {
    use test_utils::f6::variant_identifier;
    let rows = node_refusals();
    let kinds: Vec<String> = rows.iter().map(|(_, k)| variant_identifier(k)).collect();
    let kinds: Vec<&str> = kinds.iter().map(String::as_str).collect();
    let mut reports: Vec<String> = test_utils::census::set_difference(
        NODE_ERROR_KIND.identifiers(),
        &kinds,
        "NodeErrorKind's arms and the census rows disagree",
        "rendered by a row and not an arm of the census",
        "an arm with no row: give it one in `node_refusals`",
    )
    .into_iter()
    .collect();
    let mut families: std::collections::BTreeMap<String, (&'static str, Vec<String>)> =
        std::collections::BTreeMap::new();
    let mut record = |name: &str, family: &str, levels: Vec<Level>, reports: &mut Vec<String>| {
        let mut path = family.to_owned();
        for Level { roster, arm } in levels {
            let entry = families.entry(path.clone()).or_insert((roster, Vec::new()));
            if entry.0 != roster {
                reports.push(format!(
                    "{name}: the {path}/ family is read as {} and as {roster}",
                    entry.0
                ));
            }
            entry.1.push(arm.clone());
            path = format!("{path}/{arm}");
        }
    };
    let gone = |name: &str, filed: &str| {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(filed);
        (!path.is_file()).then(|| format!("{name}: its payload cites {filed}, which is gone"))
    };
    for ((name, kind), family) in rows.iter().zip(&kinds) {
        let (levels, labelled) = match forward(kind) {
            Forward::Welded { levels, labelled } => (levels, labelled),
            Forward::Unwelded(filed) | Forward::RenderedIn(filed) => {
                reports.extend(gone(name, filed));
                continue;
            }
            Forward::Own => continue,
        };
        let label = name.strip_prefix(&format!("{family}/"));
        let arms: Vec<&str> = levels.iter().map(|l| l.arm.as_str()).collect();
        if label.is_none() && !levels.is_empty() {
            reports.push(format!("{name}: a {family} row opens with {family}/"));
        } else if labelled && !names_arms(label.unwrap_or(""), &arms) {
            reports.push(format!(
                "{name}: a row is named by its payload's arm at each level, {}",
                arms.join("/")
            ));
        }
        record(name, family, levels, &mut reports);
    }
    for (family, (roster, arms)) in &families {
        let Some((_, census)) = ROSTERS.iter().find(|(r, _)| r == roster) else {
            reports.push(format!(
                "the {family}/ rows read {roster}, which ROSTERS lacks"
            ));
            continue;
        };
        let arms: Vec<&str> = arms.iter().map(String::as_str).collect();
        reports.extend(test_utils::census::set_difference(
            census,
            &arms,
            &format!("the {family}/ rows and their payload's arms disagree"),
            "rendered by a row and not an arm of the census",
            "an arm with no row: give it one",
        ));
    }
    for (roster, _) in ROSTERS {
        if !families.values().any(|(r, _)| r == roster) {
            reports.push(format!(
                "no row reaches the {roster} roster: `forward` no longer reads it"
            ));
        }
    }
    assert!(reports.is_empty(), "{}", reports.join("\n"));
}

/// A failure as the feature tree draws it: the failed node's own line,
/// then each refusal it carries as a line of its own, one per level
/// ([`NodeErrorKind::carried_chain`]).
fn drawn_lines(kind: NodeErrorKind) -> Vec<String> {
    let error = NodeError {
        node: RecipeNodeId::new(0, tagged(5)),
        kind,
        escalations: std::sync::Arc::new(Vec::new()),
    };
    core::iter::once(error.to_string())
        .chain(error.kind.carried_chain().map(|level| level.line()))
        .collect()
}

/// **Every line a carried refusal draws fits where it is drawn.** A
/// part whose piece failed and a mate whose pose did not evaluate each
/// name that node and point at it; the refusal they carry is drawn as
/// its own line, one per document level, and never inside theirs.
///
/// Held on the payloads that stress it: the longest refusal on this
/// roster, carried one document down, three documents down, and under
/// a mate's unevaluated pose. Each carrying line is held to the budget
/// and must not quote what it carries; the carried line is the roster
/// row's own text, so it keeps that row's name and its filed
/// allowances.
///
/// *Breaks if* a carrying line quotes what it carries, outgrows the
/// budget, or a mate's carried level is numbered outside its part.
#[test]
fn every_carried_refusal_draws_within_the_budget_at_every_line() {
    use editor_core::{MateFault, MateSide, PartFault};
    let unevaluated = |refusal: NodeErrorKind| {
        NodeErrorKind::Mate(Box::new(MateFault::SideUnevaluated {
            mate: editor_core::MateRef::new(
                RecipeNodeId::new(0, tagged(9)),
                editor_core::ConstraintId(editor_core::MintId::new(1, 0)),
            ),
            side: MateSide::To,
            refusal: Box::new(refusal.into()),
        }))
    };
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
        fault: PartFault::PartProductFailed {
            held: Default::default(),
            node: RecipeNodeId::new(0, tagged(node)),
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
        ("Carried/SideUnevaluated", 1, unevaluated(inner())),
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

    // A mate's carried level inside a part is numbered in that part, as
    // the part's own level is.
    let in_part = part(7, unevaluated(inner()));
    let documents: Vec<_> = in_part
        .carried_chain()
        .map(|level| (level.document.doc_ref(), level.node))
        .collect();
    let part_ref = doc_ref();
    assert_eq!(
        documents,
        vec![
            (Some(&part_ref), RecipeNodeId::new(0, tagged(7))),
            (Some(&part_ref), RecipeNodeId::new(0, tagged(9))),
        ],
        "the mate's carried level is in the part the mate is in"
    );
}

/// Every offset-fit refusal the feature tree shows ends exactly once,
/// on every route: a labelled repair, or the shared dead end. The
/// carrier the fit forwards whole (`PatchBoundError`) labels its own
/// repair, so a wrapper that added one of its own would read here as
/// two. The interpolation's carriers (`Fit/`, `Structure/`) are NOT
/// forwarded: the kernel chose their samples and knots, so a builder's
/// repair would name nothing the user supplied. A seed that ran short
/// (`Fit/TooFewPoints`) ends in the narrow knot span's lever; the
/// structure's arm and the mint's limb end in the last resort.
///
/// Every meter refusal ends in its decision's one recourse for its
/// verdict ([`meter_escalations`], [`meter_verdicts`]), never the
/// coincidence menu (a face's meter has nothing to declare), and never
/// advises lowering the tolerance.
#[test]
fn every_offset_fit_refusal_ends_exactly_once() {
    let rows = offset_fit_routes();
    assert!(!rows.is_empty(), "the offset-fit roster is empty");
    let (mut pinned, mut mint_limbs, mut at_rest_limbs) = (0, 0, 0);
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
        // The shell user sets the curvature meter's size as a wall
        // thickness, the transform's as an offset distance.
        let (arm, curvature) = match name.strip_prefix("Shell/Face/Fit/") {
            Some(arm) => (arm, WALL),
            None => (
                name.strip_prefix("Transform/ApproxRecertify/")
                    .expect("every offset-fit row is on one of the two routes"),
                DISTANCE,
            ),
        };
        let (routed, verdicts) = (meter_escalations(curvature), meter_verdicts(curvature));
        let wall = (curvature == WALL).then(wall_endings);
        let ending = routed
            .iter()
            .find(|(route, _, _)| arm == format!("Meter/Escalated/{route}"))
            .map(|(_, _, ending)| ending)
            .or_else(|| {
                verdicts
                    .iter()
                    .chain(wall.iter().flatten())
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
        if curvature == WALL {
            assert!(
                !text.contains("offset distance") && !text.contains("other side"),
                "{name}: a shell user sets a wall, not a distance or a side: {text}"
            );
        }
        if arm.starts_with("Fit/TooFewPoints") {
            assert!(
                text.ends_with(geom_brep::recourse::NARROW_SPAN_RECOURSE),
                "{name}: {text}"
            );
        } else if arm.starts_with("Structure/") || arm.starts_with("MintLimb") {
            assert!(
                text.ends_with(geom_core::KERNEL_LIMIT_RECOURSE),
                "{name}: {text}"
            );
        }
        // At rest a limb's repair is the re-fit; at the mint the re-fit
        // is the call that refused, and nothing is stored.
        if arm.starts_with("MintLimb") {
            mint_limbs += 1;
            assert!(!text.contains("stored"), "{name}: {text}");
        } else if arm.starts_with("Limb") {
            at_rest_limbs += 1;
            assert!(
                text.ends_with(geom_brep::offset_fit::LIMB_REFIT_RECOURSE),
                "{name}: {text}"
            );
        }
    }
    // `MintLimb` on both routes, `Limb` on the transform's alone. A
    // count over the rows `offset_fit_routes` builds: it holds the
    // roster's route split, not which arms the ops raise.
    assert_eq!(
        (mint_limbs, at_rest_limbs),
        (4, 2),
        "a limb row went missing"
    );
    // Both routes raise `Meter`, so every pinned ending has two rows.
    assert_eq!(
        pinned,
        2 * (meter_escalations(DISTANCE).len() + meter_verdicts(DISTANCE).len())
            + wall_endings().len(),
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
    rows.extend(motion());
    rows.extend(skin());
    rows.extend(loft());
    rows.extend(blend());
    rows.extend(profile());
    rows.extend(profile_replay());
    rows.extend(trace());
    rows.extend(editor_payloads());
    rows.extend(document_arms());
    rows.extend(mate());
    rows.extend(shell());
    rows
}

fn row(name: &str, kind: NodeErrorKind) -> (String, NodeErrorKind) {
    (name.to_owned(), kind)
}

/// The arms whose sentence `NodeErrorKind` writes itself, each on a
/// representative payload where it forwards.
fn own_arms() -> Vec<(String, NodeErrorKind)> {
    use editor_core::{Dimension, SlotId};
    use payloads::*;
    vec![
        row(
            "ProfileLaneReplay(NoStructure)",
            NodeErrorKind::ProfileLaneReplay {
                loop_: 0,
                step: 2,
                structure: None,
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
                input: RecipeNodeId::new(0, tagged(3)),
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
                input: RecipeNodeId::new(0, tagged(3)),
                expected: "body",
                found: "profile",
            },
        ),
        row(
            "EmptyOperand",
            NodeErrorKind::EmptyOperand {
                input: RecipeNodeId::new(0, tagged(3)),
            },
        ),
        row(
            "UnfinishedOperand/NegativeVolume",
            NodeErrorKind::UnfinishedOperand {
                input: RecipeNodeId::new(0, tagged(3)),
                errors: vec![topo::ValidationError::NegativeVolume {
                    solid: topo::SolidKey::default(),
                }],
            },
        ),
        row(
            "UnfinishedOperand/ScaffoldAtRest",
            NodeErrorKind::UnfinishedOperand {
                input: RecipeNodeId::new(0, tagged(3)),
                errors: vec![topo::ValidationError::ScaffoldAtRest {
                    edge: topo::EdgeKey::default(),
                }],
            },
        ),
        row(
            "UnfinishedOperand/ScaffoldingEmptyLoop",
            NodeErrorKind::UnfinishedOperand {
                input: RecipeNodeId::new(0, tagged(3)),
                errors: vec![topo::ValidationError::ScaffoldingEmptyLoop {
                    loop_: topo::LoopKey::default(),
                }],
            },
        ),
        row(
            "UnfinishedOperand/ScaffoldingStrutVertex",
            NodeErrorKind::UnfinishedOperand {
                input: RecipeNodeId::new(0, tagged(3)),
                errors: vec![topo::ValidationError::ScaffoldingStrutVertex {
                    vertex: topo::VertexKey::default(),
                }],
            },
        ),
        row(
            "UnfinishedOperand/DescriptionNotAdjacent",
            NodeErrorKind::UnfinishedOperand {
                input: RecipeNodeId::new(0, tagged(3)),
                errors: vec![topo::ValidationError::DescriptionNotAdjacent {
                    edge: topo::EdgeKey::default(),
                }],
            },
        ),
        row(
            "EmptyHalf",
            NodeErrorKind::EmptyHalf {
                input: RecipeNodeId::new(0, tagged(3)),
                half: editor_core::SplitHalf::Above,
            },
        ),
        row(
            "ContactCarry",
            NodeErrorKind::ContactCarry {
                refusal: topo::CarryRefusal::Stale { record: 0 },
            },
        ),
        row(
            "MemberOutOfRange",
            NodeErrorKind::MemberOutOfRange {
                input: RecipeNodeId::new(0, tagged(3)),
                at: vec![7, 2],
            },
        ),
        row(
            "IndicesUnrelated",
            NodeErrorKind::IndicesUnrelated {
                a: editor_core::VarId::new(0, tagged(2)),
                b: editor_core::VarId::new(0, tagged(4)),
            },
        ),
        row(
            "MemberRefused",
            NodeErrorKind::MemberRefused {
                at: vec![(editor_core::VarId::new(0, tagged(2)), 0)],
                source: Box::new(NodeErrorKind::MemberOutOfRange {
                    input: RecipeNodeId::new(0, tagged(3)),
                    at: vec![7],
                }),
            },
        ),
        row(
            "NoSuchPiece",
            NodeErrorKind::NoSuchPiece {
                pick: editor_core::VarId::new(0, tagged(2)),
                at: 0,
            },
        ),
        row(
            "PieceShape/Family",
            NodeErrorKind::PieceShape {
                pick: editor_core::VarId::new(0, tagged(2)),
                family: true,
            },
        ),
        row(
            "PieceShape/OneBody",
            NodeErrorKind::PieceShape {
                pick: editor_core::VarId::new(0, tagged(2)),
                family: false,
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
        row(
            "Band/InvalidValue",
            NodeErrorKind::Band(geom_core::BandError::InvalidValue {
                field: geom_core::BandField::Zero,
                value: f64::NAN,
            }),
        ),
        row(
            "Band/InvalidLeverArm",
            NodeErrorKind::Band(geom_core::BandError::InvalidLeverArm { value: -1.0 }),
        ),
        row("Band/Empty", NodeErrorKind::Band(band_error())),
        row(
            "MissingSlot",
            NodeErrorKind::MissingSlot {
                slot: SlotId::Distance,
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
            "MeasureEscalated",
            NodeErrorKind::MeasureEscalated {
                predicate: "bool_plane_parallel",
                source: named("bool_plane_parallel"),
            },
        ),
        row(
            "PlacementAxisEscalated",
            NodeErrorKind::PlacementAxisEscalated {
                predicate: "eval_direction_norm",
                source: named("eval_direction_norm"),
            },
        ),
        row(
            "PoseDirectionEscalated",
            NodeErrorKind::PoseDirectionEscalated {
                predicate: topo::POSE_UNIT_NORM,
                source: named(topo::POSE_UNIT_NORM),
            },
        ),
        row(
            "UnresolvedRead",
            NodeErrorKind::UnresolvedRead {
                slot: editor_core::OperandSlot::Selection,
                var: editor_core::VarId::new(0, tagged(4)),
            },
        ),
        row(
            "PoseRead",
            NodeErrorKind::PoseRead {
                pose: editor_core::VarKind::Axis,
                fault: editor_core::pose::PoseReadFault::NoAxis {
                    carrier: editor_core::pose::Carrier::Surface(geom::SurfaceKind::Plane),
                },
            },
        ),
        row(
            "PoseDegenerate",
            NodeErrorKind::PoseDegenerate {
                construction: editor_core::pose::PoseConstruction::Meet,
            },
        ),
        row(
            "IndexCountOutOfRange",
            NodeErrorKind::IndexCountOutOfRange {
                index: editor_core::VarId::new(0, tagged(2)),
                count: -3,
            },
        ),
        row(
            "CountOutOfRange",
            NodeErrorKind::CountOutOfRange {
                slot: SlotId::VDegree,
                count: -1,
                admits: 0..=i64::MAX,
            },
        ),
        row("UnschedulableCycle", NodeErrorKind::UnschedulableCycle),
        row(
            "ShellLaneUnsupported",
            NodeErrorKind::ShellLaneUnsupported { scalar: "interval" },
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
        (
            "NegativeDepth",
            E::NegativeDepth {
                side: sweep::ExtrudeSide::Along,
                depth: geom_core::MarginDiag::value(-0.25),
            },
        ),
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
                reading: topo::DihedralReading::Lever(geom_brep::LeverRung::Reading),
                source: named("dihedral_wedge"),
            },
        ),
        (
            "SliverJoin(bend)",
            E::SliverJoin {
                loop_index: 0,
                vertex_index: 3,
                reading: topo::DihedralReading::Bend,
                source: named("tangent_second_order"),
            },
        ),
        (
            "SliverRim",
            E::SliverRim {
                loop_index: 0,
                segment_index: 3,
                reading: topo::DihedralReading::Lever(geom_brep::LeverRung::Reading),
                source: named("dihedral_wedge"),
            },
        ),
        (
            "SliverRim(bend)",
            E::SliverRim {
                loop_index: 0,
                segment_index: 3,
                reading: topo::DihedralReading::Bend,
                source: named("tangent_second_order"),
            },
        ),
        (
            "CapPlane",
            E::CapPlane {
                source: sweep::CapPlaneError::Newell(newell()),
            },
        ),
        (
            "CapPlane(Orientation)",
            E::CapPlane {
                source: sweep::CapPlaneError::Orientation(named("cap_plane_orientation")),
            },
        ),
        (
            "CapPlane(EdgeOn)",
            E::CapPlane {
                source: sweep::CapPlaneError::EdgeOn,
            },
        ),
        (
            "SidePlane",
            E::SidePlane {
                loop_index: 0,
                segment_index: 3,
                source: newell(),
            },
        ),
        (
            "SmoothJoinRefuted",
            E::SmoothJoinRefuted {
                edge: topo::EdgeKey::default(),
                mixed: Some(geom_brep::MixedStations {
                    transverse: 3,
                    smooth: 4,
                }),
            },
        ),
        ("Op", E::Op { source: euler() }),
        ("Pcurve", E::Pcurve(pcurve())),
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
                decision: sweep::ClearanceDecision::ArcApex,
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
        (
            "AxisGapEscalated",
            E::AxisGapEscalated {
                loop_index: 0,
                vertex_index: 3,
                source: diag(),
            },
        ),
        ("HoleTouchesAxis", E::HoleTouchesAxis { loop_index: 1 }),
        (
            "VoidInsertion",
            E::VoidInsertion {
                cavity: sweep::CavityOrigin::Hole { loop_index: 1 },
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
                reading: topo::DihedralReading::Lever(geom_brep::LeverRung::Reading),
                source: named("dihedral_wedge"),
            },
        ),
        (
            "SliverJoin(bend)",
            E::SliverJoin {
                loop_index: 0,
                vertex_index: 3,
                reading: topo::DihedralReading::Bend,
                source: named("tangent_second_order"),
            },
        ),
        (
            "SliverRim",
            E::SliverRim {
                loop_index: 0,
                segment_index: 3,
                reading: topo::DihedralReading::Lever(geom_brep::LeverRung::Reading),
                source: named("dihedral_wedge"),
            },
        ),
        (
            "SliverRim(bend)",
            E::SliverRim {
                loop_index: 0,
                segment_index: 3,
                reading: topo::DihedralReading::Bend,
                source: named("tangent_second_order"),
            },
        ),
        (
            "CapPlane",
            E::CapPlane {
                source: sweep::CapPlaneError::Newell(newell()),
            },
        ),
        (
            "CapPlane(Orientation)",
            E::CapPlane {
                source: sweep::CapPlaneError::Orientation(named("cap_plane_orientation")),
            },
        ),
        (
            "CapPlane(EdgeOn)",
            E::CapPlane {
                source: sweep::CapPlaneError::EdgeOn,
            },
        ),
        (
            "Turn",
            E::Turn(geom_core::RigidMotionError::NonFinite(
                geom_core::MotionPart::Angle,
            )),
        ),
        (
            "SmoothJoinRefuted",
            E::SmoothJoinRefuted {
                edge: topo::EdgeKey::default(),
                mixed: Some(geom_brep::MixedStations {
                    transverse: 3,
                    smooth: 4,
                }),
            },
        ),
        (
            "PinnedRunStation",
            E::PinnedRunStation {
                loop_index: 0,
                vertex_index: 2,
            },
        ),
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
                decision: sweep::TubeDecision::WindowSpan,
                source: named("tube_window_span"),
            },
        ),
        (
            "Escalated(hollow)",
            E::Escalated {
                decision: sweep::TubeDecision::WallGap,
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
                kind: geom::SurfaceKind::Nurbs,
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
        (
            "KnifeEdge",
            R::KnifeEdge(topo::KnifeEdge {
                wall: face,
                at: topo::KnifeEdgeSite::Vertex(vertex),
            }),
        ),
        (
            "ScaffoldingOperand",
            R::ScaffoldingOperand {
                errors: vec![topo::ValidationError::ScaffoldingStrutVertex { vertex }],
            },
        ),
        (
            "InsideOutOperand",
            R::InsideOutOperand {
                errors: vec![topo::ValidationError::NegativeVolume {
                    solid: topo::SolidKey::default(),
                }],
            },
        ),
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
                reading: topo::SectorReading::Escalated(diag()),
            },
        ),
        (
            "SliverSector/GrazeAgainstNeighbours",
            R::SliverSector {
                vertex,
                face,
                reading: topo::SectorReading::GrazeAgainstNeighbours {
                    margin: geom_core::MarginDiag::value(-3.0e-8),
                },
            },
        ),
        (
            "SliverSector/SmoothEdgeDisagrees",
            R::SliverSector {
                vertex,
                face,
                reading: topo::SectorReading::SmoothEdgeDisagrees(topo::TangencyReadings {
                    tangent: geom_core::MarginDiag::value(3.0e-10),
                    clear: geom_core::MarginDiag::value(3.0e-8),
                    band: band(),
                }),
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
        (
            "SectionGermsEscalated(Line)",
            R::SectionGermsEscalated {
                vertex,
                reading: topo::SectionGermReading::Line,
                diag: diag(),
            },
        ),
        (
            "SectionGermsEscalated(Order)",
            R::SectionGermsEscalated {
                vertex,
                reading: topo::SectionGermReading::Order,
                diag: diag(),
            },
        ),
        (
            "SectionGermsUnordered(NoLine)",
            R::SectionGermsUnordered {
                vertex,
                fault: topo::SectionGermFault::NoLine,
            },
        ),
        (
            "SectionGermsUnordered(Coincident)",
            R::SectionGermsUnordered {
                vertex,
                fault: topo::SectionGermFault::Coincident,
            },
        ),
        (
            "SectionGermsUnordered(Singular)",
            R::SectionGermsUnordered {
                vertex,
                fault: topo::SectionGermFault::Singular,
            },
        ),
        (
            "SectionGermsUnordered(Alternation)",
            R::SectionGermsUnordered {
                vertex,
                fault: topo::SectionGermFault::Alternation,
            },
        ),
        (
            "SectionGermsUnordered(Crossing)",
            R::SectionGermsUnordered {
                vertex,
                fault: topo::SectionGermFault::Crossing,
            },
        ),
        (
            "ApexSection",
            R::ApexSection {
                vertex,
                face,
                reading: topo::ApexSectionReading::NoRulingInside,
            },
        ),
        (
            "ApexUndecided",
            R::ApexUndecided {
                vertex,
                face,
                diag: diag(),
            },
        ),
        ("ConsecutiveOnSectors", R::ConsecutiveOnSectors { vertex }),
        ("StaleVertex", R::StaleVertex { vertex }),
        ("LoneVertex", R::LoneVertex { vertex }),
        (
            "NullEdgeAtVertex",
            R::NullEdgeAtVertex {
                vertex,
                edge: topo::EdgeKey::default(),
            },
        ),
        ("UnrecordedSide", R::UnrecordedSide { vertex }),
        ("UnboundedFace", R::UnboundedFace { face, vertex }),
        (
            "CrossingInsertion",
            R::CrossingInsertion {
                edge,
                endpoints: (vertex, vertex),
                source: Box::new(euler()),
            },
        ),
        ("Euler", R::Euler(euler())),
        (
            "UnjoinedOperand",
            R::UnjoinedOperand {
                errors: vec![topo::ValidationError::JoinableVertexAtRest { vertex }],
            },
        ),
        (
            "LineageRow",
            R::LineageRow(topo::RowRefusal {
                rows: topo::LineageRows::Fusions,
                defect: topo::RowDefect::SelfRow,
                key: topo::Cell::Vertex(vertex),
            }),
        ),
    ]
    .map(|(n, e)| (format!("Reduce/{n}"), SplitError::Reduce(e)));
    // The join is shared by the split and the Boolean, and each wraps it
    // in its own words and recourse, so both chains are rendered.
    let join_arms = || {
        [
            ("OrderEscalated", J::OrderEscalated { diag: diag() }),
            ("Escalated", J::Escalated { face, diag: diag() }),
            ("DegenerateSection", J::DegenerateSection { face }),
            ("CurvedSlitTrace", J::CurvedSlitTrace { face }),
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
                "RingHoming(Uncrossable)",
                J::RingHoming(topo::PointInLoopError::Uncrossable(topo::Uncrossable {
                    r#loop: LoopKey::default(),
                    edge: Default::default(),
                    carrier: topo::UncrossableCarrier::Spiric,
                })),
            ),
            (
                "RingHoming(OffPlane)",
                J::RingHoming(topo::PointInLoopError::OffPlane(topo::OffPlane {
                    r#loop: LoopKey::default(),
                    cause: topo::OffPlaneCause::Query,
                })),
            ),
            ("UnpairedLooseEnds", J::UnpairedLooseEnds { count: 3 }),
            (
                "SingleSiteSectionLoop",
                J::SingleSiteSectionLoop { count: 2 },
            ),
            ("SectionLoopMixed", J::SectionLoopMixed { face }),
            ("SectionLoopUndecided", J::SectionLoopUndecided { face }),
            ("CutInvariant", J::CutInvariant { edge }),
            (
                "Corrupt",
                J::Corrupt {
                    entity: EntityId::Edge(edge),
                },
            ),
            ("Band", J::Band(band_error())),
            ("Euler", J::Euler(euler())),
            (
                "LineageRow",
                J::LineageRow(topo::RowRefusal {
                    rows: topo::LineageRows::FaceDivisions,
                    defect: topo::RowDefect::NotFresh,
                    key: topo::Cell::Face(face),
                }),
            ),
            (
                "SectionEndEscalated(OnPlane)",
                J::SectionEndEscalated {
                    face,
                    reading: topo::SectionEndReading::OnPlane,
                    diag: diag(),
                },
            ),
            (
                "SectionEndEscalated(SquareToAxis)",
                J::SectionEndEscalated {
                    face,
                    reading: topo::SectionEndReading::SquareToAxis,
                    diag: diag(),
                },
            ),
            (
                "SectionUnresolved",
                J::SectionUnresolved {
                    face,
                    miss: topo::SectionPinMiss::Undecided {
                        check: geom_brep::CertCheck::EndpointStart,
                        diag: diag(),
                        resolution: geom_core::MarginDiag::value(4.6e-9),
                    },
                },
            ),
            (
                "SectionUnresolved(Past)",
                J::SectionUnresolved {
                    face,
                    miss: topo::SectionPinMiss::Past {
                        check: geom_brep::CertCheck::EndpointEnd,
                        sample: 8,
                        margin: geom_core::MarginDiag::value(3.5e-10),
                    },
                },
            ),
            // The two section refusals a plane-inclusive pair can reach
            // (the join reads no other): a lane bug, and the conic
            // carrier's own escalation on its minor semi-axis.
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
                    source: geom_brep::SectionError::Carrier(geom::EllipseInvalid::Escalated(
                        payloads::named("ellipse_minor_positive"),
                    )),
                },
            ),
            (
                "OpenConicChord",
                J::OpenConicChord {
                    face,
                    conic: geom_brep::OpenConic::Hyperbola,
                },
            ),
            ("ApexUnlifted", J::ApexUnlifted { face }),
            ("PoleEdgesCoincide", J::PoleEdgesCoincide { face }),
            (
                "SectionInvariant",
                J::SectionInvariant {
                    face,
                    what: "a section arc with no endpoint on the face's boundary",
                },
            ),
            (
                "SectionUnarmed",
                J::SectionUnarmed {
                    face,
                    divided: geom::SurfaceKind::Plane,
                    wall: geom::SurfaceKind::Nurbs,
                },
            ),
            (
                "RunImageNotYet",
                J::RunImageNotYet {
                    face,
                    kind: geom::SurfaceKind::Sphere,
                },
            ),
            (
                "RunImageIncidence",
                J::RunImageIncidence {
                    face,
                    kind: geom::SurfaceKind::Cone,
                    refusal: geom_brep::PcurveCertifyError::CarrierOffChart {
                        chart: geom::SurfaceKind::Cone,
                        carrier: geom::CurveKind::Circle,
                        why: "the rim's height is not shown away from the apex's",
                        margin: Some(geom_brep::OffMargin::InBand(geom_core::MarginDiag::value(
                            4e-10,
                        ))),
                    },
                },
            ),
            (
                "RingIslandUnread",
                J::RingIslandUnread {
                    face,
                    kind: geom::SurfaceKind::Cone,
                },
            ),
            (
                "RunEdgeCarrierUnsupported",
                J::RunEdgeCarrierUnsupported {
                    face,
                    edge: topo::EdgeKey::default(),
                },
            ),
            (
                "TangentChordEscalated",
                J::TangentChordEscalated { face, diag: diag() },
            ),
            (
                "TangentChordBendEscalated",
                J::TangentChordBendEscalated { face, diag: diag() },
            ),
            (
                "TangentChordRefuted",
                J::TangentChordRefuted {
                    face,
                    mixed: Some(geom_brep::MixedStations {
                        transverse: 3,
                        smooth: 4,
                    }),
                },
            ),
            (
                "SectionCrossings",
                J::SectionCrossings {
                    face,
                    case: topo::ConicCrossingsCase::NotAlternating,
                    band: band(),
                },
            ),
            (
                "SectionDepartureGrazes",
                J::SectionDepartureGrazes { face, band: band() },
            ),
        ]
    };
    let join = join_arms().map(|(n, e)| (format!("Join/{n}"), SplitError::Join(e)));
    let boolean_join = join_arms().map(|(n, e)| {
        row(
            &format!("Boolean/Join/{n}"),
            NodeErrorKind::Boolean(topo::BooleanError::from(e)),
        )
    });
    let shell = ShellKey::default();
    let finish = [
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
        (
            "ConeCornerEscalated",
            F::ConeCornerEscalated {
                vertex,
                source: diag(),
            },
        ),
        ("ConeCornerUnread", F::ConeCornerUnread { vertex }),
        (
            "ConeFusionDesync",
            F::ConeFusionDesync {
                what: "a joint half-edge has no end",
            },
        ),
        (
            "DescribeSideEscalated",
            F::DescribeSideEscalated { edge, diag: diag() },
        ),
        (
            "DescribeBendEscalated",
            F::DescribeBendEscalated { edge, diag: diag() },
        ),
        (
            "EdgeOnFacesEscalated",
            F::EdgeOnFacesEscalated { edge, diag: diag() },
        ),
        (
            "SmoothJoinRefuted",
            F::SmoothJoinRefuted {
                edge,
                mixed: Some(geom_brep::MixedStations {
                    transverse: 3,
                    smooth: 4,
                }),
            },
        ),
        (
            "KnifeEdge",
            F::KnifeEdge(topo::KnifeEdge {
                wall: face,
                at: topo::KnifeEdgeSite::Edge(edge),
            }),
        ),
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
        (
            "SlitUnplaced",
            F::SlitUnplaced {
                slit: face,
                in_hole: true,
            },
        ),
        (
            "SlitHoming",
            F::SlitHoming {
                slit: face,
                error: topo::PointInLoopError::RayExhausted {
                    r#loop: LoopKey::default(),
                },
            },
        ),
        (
            "EdgeJoin",
            F::EdgeJoin {
                side: topo::PlaneSide::Above,
                refusal: join_refusal(),
            },
        ),
        (
            "ResultNotAtRest",
            F::ResultNotAtRest {
                side: topo::PlaneSide::Above,
                errors: vec![topo::ValidationError::JoinableVertexAtRest { vertex }],
            },
        ),
        (
            "ResultEscalated",
            F::ResultEscalated {
                side: topo::PlaneSide::Below,
                decision: topo::BooleanDecision::ShellRole {
                    near: face,
                    far: face,
                    others: 1,
                },
                // A margin so deep in the band that no tolerance above
                // `EPS_MIN` decides it, so the sentence offers none at
                // any eps row and its length is one number.
                diag: geom_core::Indeterminate {
                    margin: geom_core::MarginDiag::value(1.0e-20),
                    ..diag()
                },
            },
        ),
        (
            "Merge",
            F::Merge {
                side: topo::PlaneSide::Above,
                error: topo::MergeCoplanarError::PeriodClosure { edge },
            },
        ),
        (
            "ResultInvalid",
            F::ResultInvalid {
                side: topo::PlaneSide::Below,
                errors: vec![topo::ValidationError::ScaffoldingEmptyLoop {
                    loop_: topo::LoopKey::default(),
                }],
            },
        ),
    ]
    .map(|(n, e)| (format!("Finish/{n}"), SplitError::Finish(e)));
    reduce
        .into_iter()
        .chain(join)
        .chain(finish)
        .chain([
            ("Pcurves".to_owned(), SplitError::Pcurves(pcurve())),
            (
                "Pieces".to_owned(),
                SplitError::Pieces(topo::PieceSortError::NoOwner { shell }),
            ),
        ])
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
                source: geom_brep::CertifyError::StatedInterval {
                    refused: geom_brep::StatedSpan::Forward(geom_brep::ForwardReading::Decided(
                        geom_brep::recourse::Refused::Negative {
                            margin: geom_core::MarginDiag::value(-1.0),
                        },
                    )),
                },
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
        (
            "Corrupt",
            E::Corrupt {
                found: "a vertex references a missing point",
            },
        ),
    ]
    .into_iter()
    .map(|(n, e)| row(&format!("Transform/{n}"), NodeErrorKind::Transform(e)))
    .chain([row(
        "Mirror/NurbsPlaceholder",
        NodeErrorKind::Mirror(E::NurbsPlaceholder),
    )])
    .chain(certify_refusal_routes())
    .chain(offset_fit_routes())
    .collect()
}

/// A placement's motion: each mint's non-finite part, and the decided
/// door's four refusals.
fn motion() -> Vec<(String, NodeErrorKind)> {
    use geom_core::{Band, Indeterminate, MarginDiag, MotionPart, RigidMotionError as E};
    let band = Band::new(1.0e-9, 1.0e-8).expect("a fixed, ordered band");
    [
        ("NonFinite/Angle", E::NonFinite(MotionPart::Angle)),
        (
            "NonFinite/Translation",
            E::NonFinite(MotionPart::Translation),
        ),
        ("NonFinite/Anchor", E::NonFinite(MotionPart::Anchor)),
        ("NonFinite/Product", E::NonFinite(MotionPart::Product)),
        ("NonFinite/Inverse", E::NonFinite(MotionPart::Inverse)),
        (
            "NotRigid/scale",
            E::NotRigid {
                check: "transform_rigid_col0_unit",
            },
        ),
        (
            "NotRigid/shear",
            E::NotRigid {
                check: "transform_rigid_col01_orth",
            },
        ),
        (
            "NotRigid/mirror",
            E::NotRigid {
                check: "transform_rigid_det_plus_one",
            },
        ),
        (
            "Undecided",
            E::Undecided {
                check: "transform_rigid_col01_orth",
                source: Indeterminate {
                    margin: MarginDiag::value(5.0e-9),
                    band,
                    predicate: Some("transform_rigid_col01_orth"),
                    terminal_sliver: false,
                },
            },
        ),
    ]
    .into_iter()
    .map(|(n, e)| row(&format!("Motion/{n}"), NodeErrorKind::Motion(e)))
    .collect()
}

/// One certification refusal per ending the certifier's typed routing
/// gives (D4 ¶1), with that whole ending:
/// - a sized decision's lever and the tolerance below `m/K`, on its
///   in-band arm, and the same lever and conditional on its definite
///   zero arm, which has no margin to quote;
/// - the span's own lever;
/// - a collapse gate's own decision (a lever arm, a spline's metered
///   length), undecided and decided: its lever, with what a floor at or
///   below zero means on a spline meter, and on a poisoned margin what
///   that may mean;
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
                lever: None,
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
                lever: None,
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
            "Recourse: move the geometry so this edge runs forward over a clear length, or, if \
             this length is intended, tighten the tolerance below 5e-10 m",
        ),
        (
            "invalid",
            escalated(CertCheck::Transversality, MarginDiag::INVALID),
            "Recourse: move the geometry so the surfaces cross at a clearer angle; an unreadable margin may indicate a kernel bug worth reporting",
        ),
        (
            "lever arm",
            escalated(CertCheck::TransversalityArm, in_band),
            "Recourse: move the geometry so that edge's faces clearly run farther and none curves \
             tightly there, or, if this width or the gap its faces open is intended, \
             tighten the tolerance below 5e-10 m",
        ),
        (
            "lever arm, enclosure",
            escalated(
                CertCheck::TransversalityArm,
                MarginDiag::enclosure(2.0e-9, 5.0e-9),
            ),
            "Recourse: move the geometry so that edge's faces clearly run farther and none curves \
             tightly there, or, if this width or the gap its faces open is intended, \
             tighten the tolerance below 2e-10 m",
        ),
        (
            "no lever arm",
            CertifyError::ArmCollapsed {
                sample: 4,
                verdict: geom_brep::recourse::Refused::Zero(geom_brep::recourse::Classified {
                    margin: MarginDiag::value(0.0),
                    band,
                }),
            },
            "Recourse: move the geometry so that edge's faces clearly run farther and none curves \
             tightly there; faces of no extent or a cone's point leave no angle to measure",
        ),
        (
            "negative lever arm",
            CertifyError::ArmCollapsed {
                sample: 4,
                verdict: geom_brep::recourse::Refused::Negative {
                    margin: MarginDiag::value(-1e-3),
                },
            },
            "Recourse: move the geometry so that edge's faces clearly run farther and none curves \
             tightly there; faces of no extent or a cone's point leave no angle to measure",
        ),
        (
            "spline meter turns back",
            CertifyError::SpanMeterCollapsed {
                verdict: geom_brep::recourse::Refused::Negative {
                    margin: MarginDiag::value(-1.0),
                },
            },
            "Recourse: move the geometry so this spline edge runs steadily forward, never stalling \
             or turning back; if it does neither, the speed floor has reached its limit, which is \
             worth reporting",
        ),
        (
            "spline meter, invalid",
            escalated(CertCheck::ParamSpanMeter, MarginDiag::INVALID),
            "Recourse: move the geometry so this spline edge runs steadily forward, never stalling \
             or turning back; an unreadable margin may indicate a kernel bug worth \
             reporting",
        ),
        (
            "endpoint",
            escalated(CertCheck::EndpointStart, in_band),
            geom_core::KERNEL_DEFECT_ENDING,
        ),
        (
            "surface-residual",
            escalated(
                CertCheck::SurfaceResidual {
                    surface: topo::SurfaceKey::default(),
                },
                in_band,
            ),
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
                 re-certification: the crossing angle at sample 4 escalated: margin 5e-9 \
                 lies inside the ambiguity band (1e-9, 1e-8). Recourse: move the geometry so the \
                 surfaces cross at a clearer angle, or, if this angle is intended, tighten the \
                 tolerance below 5e-10 m"
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
const WALL: &str = "Recourse: use a thinner wall";

/// One `Meter/Escalated` sample per ending its meter's decision gives an
/// undecided margin (D4 ¶1), with that whole ending: each meter's lever
/// and the tolerance below `m/K` on a positive margin in band; the lever
/// alone on an enclosure straddling zero, which no smaller tolerance
/// passes; and on a poisoned margin the lever and what it may mean.
///
/// The band is fixed rather than the run's witness band, so the quoted
/// `m/K` is the same at every eps row. `curvature` is the curvature
/// meter's lever on the route read ([`DISTANCE`] or [`WALL`]).
fn meter_escalations(curvature: &str) -> Vec<(&'static str, geom_brep::OffsetFitError, String)> {
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
            tighten(curvature, "clearance"),
        ),
        (
            "curvature-enclosure",
            escalated(Meter::CurvatureHeadroom, wide),
            curvature.to_owned(),
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
                "{curvature}; an unreadable margin may indicate a kernel bug worth \
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
/// the lever alone. `curvature` is as [`meter_escalations`] takes it.
fn meter_verdicts(curvature: &str) -> [(&'static str, String); 4] {
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
        ("Meter/CurvatureHeadroom", curvature.to_owned()),
        (
            "Meter/CurvatureHeadroom#2",
            format!(
                "{curvature}, or, if this clearance is intended, tighten the tolerance below \
                 5e-11 m"
            ),
        ),
    ]
}

/// The endings the shell route alone gives the fit's other arms about
/// the move's length, by row: the unbounded fit's `best: None` sample
/// names a thicker wall, and an invalid request is the op's own defect
/// (its thickness gate and its tolerance witness never make one).
fn wall_endings() -> [(&'static str, String); 2] {
    [
        (
            "BoundNotFinite#2",
            "Recourse: use a thicker wall".to_owned(),
        ),
        ("InvalidRequest", geom_core::KERNEL_DEFECT_ENDING.to_owned()),
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
///   the chart rectangle it fitted), `Limb` (a limb refusing at the
///   mint is `MintLimb`) and `Band` (below). The loop's own
///   terminations — `SeedOverCap`, `WallReached`, `BoundNotFinite`,
///   `RefinementStalled` — and the interpolation's `Fit`, `Structure`
///   and `NonFiniteSample` reach the user by this route and by the
///   transform's re-fit, and so does `MintLimb`, whose re-fit is the
///   call that refused, so it ends in the kernel-defect ending.
/// - **The transform op** (`Transform/ApproxRecertify/…`) raises the
///   certifier's arms on the image — `certify_offset_over` runs the
///   meters and the certificate limbs on a fit it did not make, so
///   `Meter`, `PatchBound`, `WindowUnsupported` and `Limb` — and the
///   mint's arms on a re-fit, when a sound face's image refuses a limb
///   and the map fits the mapped description afresh: every arm the
///   shell route raises. So every arm but `Band`.
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
    // otherwise shrink these rows silently. `WallReached`,
    // `BoundNotFinite`, `Limb` and `MintLimb` each carry two samples
    // (both walls, both `best` cases, both limbs).
    for (arm, samples) in [
        ("Meter/NormalFloor", 2),
        ("Meter/CurvatureHeadroom", 2),
        ("Meter/Escalated", 1),
        ("PatchBound/", 6),
        ("Fit/", 1),
        ("Structure/", 1),
        ("InvalidRequest", 1),
        ("NonFiniteSample", 1),
        ("SeedOverCap", 1),
        ("WallReached", 2),
        ("BoundNotFinite", 2),
        ("RefinementStalled", 1),
        ("WindowUnsupported", 1),
        ("Limb", 2),
        ("MintLimb", 2),
    ] {
        let have = roster.iter().filter(|(n, _)| n.starts_with(arm)).count();
        assert!(
            have >= samples,
            "the roster carries {have} OffsetFitError::{arm} sample(s), under {samples}"
        );
    }
    roster.extend(
        // The sources alone: a source does not depend on the lever,
        // which only the endings read, per route, in the test.
        meter_escalations(DISTANCE)
            .into_iter()
            .map(|(route, source, _ending)| (format!("Meter/Escalated/{route}"), source)),
    );
    let face = FaceKey::default();
    let mut rows = Vec::new();
    for (arm, source) in roster {
        let transform = !matches!(source, O::Band(_));
        if !matches!(
            source,
            O::WindowUnsupported { .. } | O::Limb { .. } | O::Band(_)
        ) {
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
        ("PathFoldsBack", E::PathFoldsBack { station: 4 }),
        (
            "PathStationsCoincide",
            E::PathStationsCoincide { station: 4 },
        ),
        ("PathTangentVanishes", E::PathTangentVanishes { station: 4 }),
        ("RowMotionUndecided", E::RowMotionUndecided { row: 1 }),
        (
            "ArcSweepUndecided",
            E::ArcSweepUndecided {
                section: 1,
                which: sweep::ArcSweepDecision::SubArcCount,
                margin: geom_core::MarginDiag::enclosure(-0.25, 0.5),
            },
        ),
        ("NoParameterStep", E::NoParameterStep { section: 2 }),
        (
            "Fit",
            E::Fit(geom::FitError::TooFewPoints { have: 1, need: 2 }),
        ),
        (
            "KnotAlgebra",
            E::KnotAlgebra(geom_core::spline::KnotAlgebraError::KnotNotPresent { u: 0.5 }),
        ),
        (
            "PathUndecided",
            E::PathUndecided {
                station: 2,
                which: sweep::PathDecision::FoldBack,
                source: payloads::diag(),
            },
        ),
        ("StationNotFinite", E::StationNotFinite { station: 2 }),
        ("Band", E::Band(payloads::band_error())),
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
    let segment = |loop_index, segment| sweep::LoopSegment {
        loop_index,
        segment,
    };
    [
        ("Band", E::Band(band_error())),
        (
            "Skin",
            E::Skin(sweep::SkinError::TooFewSections { have: 1, need: 2 }),
        ),
        ("Euler", E::Euler(euler())),
        (
            "CapPlane",
            E::CapPlane(sweep::CapPlaneError::Newell(newell())),
        ),
        (
            "CapPlane(Orientation)",
            E::CapPlane(sweep::CapPlaneError::Orientation(named(
                "cap_plane_orientation",
            ))),
        ),
        (
            "CapPlane(EdgeOn)",
            E::CapPlane(sweep::CapPlaneError::EdgeOn),
        ),
        ("Pcurve", E::Pcurve(pcurve())),
        (
            "SeamStructure",
            E::SeamStructure {
                source: geom_core::SplineError::DomainInvalid { lo: 1.0, hi: 0.0 },
            },
        ),
        ("SectionStructure", E::SectionStructure),
        (
            "SectionRim",
            E::SectionRim {
                section: 1,
                source: None,
            },
        ),
        (
            "SectionRim(Escalated)",
            E::SectionRim {
                section: 1,
                source: Some(diag()),
            },
        ),
        ("PlacementNotFinite", E::PlacementNotFinite { section: 1 }),
        ("SectionNotFinite", E::SectionNotFinite { section: 1 }),
        ("ReversedStacking", E::ReversedStacking { slab: 1 }),
        ("DegenerateStacking", E::DegenerateStacking { slab: 1 }),
        (
            "PinnedCorner",
            E::PinnedCorner {
                loop_index: 1,
                at: sweep::PinnedSite::Segment(12),
                slab: 10,
                moving: 11,
            },
        ),
        (
            "PinnedCornerUnsupported",
            E::PinnedCornerUnsupported {
                loop_index: 1,
                at: sweep::PinnedSite::Segment(12),
                slab: 10,
            },
        ),
        (
            "CornerEscalated",
            E::CornerEscalated {
                loop_index: 1,
                corner: 12,
                slab: 10,
                source: diag(),
            },
        ),
        (
            "SectionsTouch",
            E::SectionsTouch {
                slab: 10,
                segments: [segment(1, 12), segment(2, 13)],
                shared: 1,
            },
        ),
        (
            "ClearanceUndecided",
            E::ClearanceUndecided {
                slab: 10,
                segments: [segment(1, 12), segment(2, 13)],
                source: diag(),
            },
        ),
        (
            "ClearanceFault",
            E::ClearanceFault {
                slab: 10,
                segments: [segment(1, 12), segment(2, 13)],
                readings: "the pose's discriminant and its root count",
            },
        ),
        ("FarSectionNotForward", E::FarSectionNotForward { slab: 1 }),
        (
            "FarStackingEscalated",
            E::FarStackingEscalated {
                slab: 1,
                source: diag(),
            },
        ),
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
        BlendDecision, BlendError as E, BlendKind, BlendSite, Breach, ClassifiedMargin,
        CornerConfig,
    };
    use topo::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopKey, VertexKey};
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
            "FaceClearance",
            E::FaceClearance {
                at: topo::EntityId::Face(face),
                chain: sweep::blend::Convexity::Concave,
                margin: decided("fillet3_face_clearance", -1e-3, Sign::Negative),
                bounded: false,
            },
        ),
        (
            "FaceClearance(bounded, another band)",
            E::FaceClearance {
                at: topo::EntityId::Edge(edge),
                chain: sweep::blend::Convexity::Convex,
                margin: decided("fillet3_face_clearance", -1e-3, Sign::Negative),
                bounded: true,
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
            "Escalated(contact-arm)",
            E::Escalated {
                site: BlendSite::Link { edge },
                decision: BlendDecision::ContactArm,
                source: escalated("dihedral_arm"),
            },
        ),
        (
            "Escalated(contact-wedge)",
            E::Escalated {
                site: BlendSite::Link { edge },
                decision: BlendDecision::ContactWedge,
                source: escalated("dihedral_wedge"),
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
                detail: sweep::blend::battery::END_FACE_CURVED,
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
            "BodyNotIntact/Unresolved",
            E::BodyNotIntact {
                at: EntityId::HalfEdge(HalfEdgeKey::default()),
                breach: Breach::Unresolved,
                detail: "a twin half-edge the plan followed",
            },
        ),
        (
            "BodyNotIntact/Unwalked",
            E::BodyNotIntact {
                at: EntityId::Loop(LoopKey::default()),
                breach: Breach::Unwalked,
                detail: "a cycle of a face a chord spans does not walk",
            },
        ),
        (
            "BodyNotIntact/Disagrees",
            E::BodyNotIntact {
                at: EntityId::Vertex(vertex),
                breach: Breach::Disagrees,
                detail: "an end face's cycle around the old vertex is not flanked by the two feet \
                         just split",
            },
        ),
        (
            "BodyNotIntact/Scaffolding",
            E::BodyNotIntact {
                at: EntityId::Edge(edge),
                breach: Breach::Scaffolding,
                detail: "a rim edge carries no certified carrier",
            },
        ),
        (
            "BodyNotIntact/Degenerate",
            E::BodyNotIntact {
                at: EntityId::Face(face),
                breach: Breach::Degenerate,
                detail: "a face's stored surface, which its box reads",
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
            "WhollyInside",
            E::WhollyInside {
                at: topo::EntityId::Face(face),
                chain: sweep::blend::Convexity::Convex,
                margin: decided("fillet3_face_clearance", -1e-3, Sign::Negative),
            },
        ),
        (
            "ScaffoldingOperand",
            E::ScaffoldingOperand {
                errors: vec![topo::ValidationError::ScaffoldingStrutVertex { vertex }],
            },
        ),
        (
            "InsideOutOperand",
            E::InsideOutOperand {
                errors: vec![topo::ValidationError::NegativeVolume {
                    solid: topo::SolidKey::default(),
                }],
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
            "UnjoinedOperand",
            E::UnjoinedOperand {
                errors: vec![topo::ValidationError::JoinableVertexAtRest { vertex }],
            },
        ),
        (
            "Join",
            E::Join {
                refusal: join_refusal(),
            },
        ),
        (
            "Op",
            E::Op {
                site: "strut mev",
                source: Box::new(euler()),
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
        ("PlaneNotFinite", E::PlaneNotFinite),
        ("EmptyLoop", E::EmptyLoop { loop_index: 0 }),
        ("DegenerateSegment", E::DegenerateSegment(a)),
        (
            "ArcBelowSceneResolution",
            E::ArcBelowSceneResolution {
                at: a,
                check: profile::ArcCheck::Landing,
                margin: geom_core::MarginDiag::value(3.0e-10),
                headroom: geom_core::MarginDiag::value(1.0e-9),
            },
        ),
        (
            "InconsistentArc",
            E::InconsistentArc {
                at: a,
                check: profile::ArcCheck::OnCarrier,
            },
        ),
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
            "TangentJointOnFullTurn",
            E::TangentJointOnFullTurn { loop_index: 0 },
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
        (
            "ArcSweepNotShortOfFullTurn",
            P::ArcSweepNotShortOfFullTurn { angle: 7.0 },
        ),
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
        (
            "ArcMissesAuthoredPoint",
            P::ArcMissesAuthoredPoint {
                piece: profile::Piece {
                    step: 1,
                    role: profile::PieceRole::Leg,
                },
                verb: profile::Verb::TangentArcTo,
                predicate: "path_arc_landing",
                point: Point2::new(-2.7, 2.7e-4),
                miss: 3.4e-8,
                scale: 1.36e4,
                resolution: 4.8e-11,
                turn: 6.2,
                authored: None,
            },
        ),
        (
            "ArcMissBelowSceneResolution",
            P::ArcMissBelowSceneResolution {
                piece: profile::Piece {
                    step: 1,
                    role: profile::PieceRole::Leg,
                },
                verb: profile::Verb::ArcTo,
                predicate: "path_arc_through_point",
                point: Point2::new(0.0, 2.74e8),
                miss: 3.0e-8,
                scale: 2.74e8,
                resolution: 4.9e-7,
                turn: 6.2,
                authored: None,
            },
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
                "ProfileLaneReplay/Flipped",
                NodeErrorKind::ProfileLaneReplay {
                    loop_: 0,
                    step: 2,
                    structure: Some(structure_refusal()),
                },
            ),
            row(
                "ProfileLaneReplay/Indeterminate",
                NodeErrorKind::ProfileLaneReplay {
                    loop_: 0,
                    step: 2,
                    structure: Some(structure_refusal_indeterminate()),
                },
            ),
        ])
        .collect()
}

/// A profile step reading a trace, refusing each way it can.
fn trace() -> Vec<(String, NodeErrorKind)> {
    use editor_core::{TraceCrossing, TracePassage, TraceRefusal, TraceStepFault as F};
    use topo::boolean::trace::TraceFault;
    let diag = || payloads::named(topo::boolean::trace::TRACE_CROSSING_SENSE);
    let faults = vec![
        ("Undrawn", F::Undrawn),
        ("Unevaluated", F::Unevaluated),
        ("NoSuchFace", F::NoSuchFace { traced: 1 }),
        ("NoPen", F::NoPen),
        ("NoHeading", F::NoHeading),
        ("NotARun", F::NotARun),
        ("OffTrace", F::OffTrace),
        ("NoTrace", F::NoTrace),
        (
            "NoSuchCrossing",
            F::NoSuchCrossing {
                crossing: TraceCrossing {
                    passage: TracePassage::Leave,
                    nth: 2,
                },
                found: 1,
            },
        ),
        ("NoTangent", F::NoTangent),
        ("Unreached", F::Unreached),
        (
            "Carrier/Unsupported",
            F::Carrier(TraceFault::Unsupported {
                carrier: geom::SurfaceKind::Nurbs,
            }),
        ),
        ("Carrier/Grazing", F::Carrier(TraceFault::Grazing)),
        ("Carrier/Apex", F::Carrier(TraceFault::Apex)),
        ("Carrier/OnCarrier", F::Carrier(TraceFault::OnCarrier)),
        (
            "Carrier/Escalated",
            F::Carrier(TraceFault::Escalated(diag())),
        ),
        (
            "Carrier/Unsettled",
            F::Carrier(TraceFault::Unsettled(geom_brep::roots::Unsettled::Meter)),
        ),
    ];
    faults
        .into_iter()
        .map(|(name, fault)| {
            row(
                &format!("Trace/{name}"),
                NodeErrorKind::Trace(TraceRefusal {
                    loop_: 0,
                    step: 3,
                    face: Some(0),
                    fault,
                }),
            )
        })
        .collect()
}

/// The editor-core payloads `NodeErrorKind` forwards: expressions,
/// analysis seeds, naming, the name ladder.
fn editor_payloads() -> Vec<(String, NodeErrorKind)> {
    use editor_core::{
        Diagnosis, Dimension, EntityKind, EvalError, NamingError, ParamBoxError, RecipeEditRef,
        ResolveError, RimShare, SeedError, SlotId, StableName, TieWitness, VarName,
    };
    use geom_core::Sign;
    use payloads::*;
    let name = || StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId::new(0, tagged(3)),
        path: Vec::new(),
    };
    let eval = || -> Vec<(&str, EvalError)> {
        vec![
            (
                "UnresolvedVar",
                EvalError::UnresolvedVar {
                    var: editor_core::VarId::new(0, tagged(7)),
                },
            ),
            (
                "OutputRead",
                EvalError::OutputRead {
                    var: editor_core::VarId::new(0, tagged(7)),
                },
            ),
            (
                "DefinitionRefused",
                EvalError::DefinitionRefused {
                    var: editor_core::VarId::new(0, tagged(7)),
                    source: Box::new(EvalError::NonFiniteResult),
                },
            ),
            (
                "VarKindMismatch",
                EvalError::VarKindMismatch {
                    var: editor_core::VarId::new(0, tagged(7)),
                    bound: Dimension::Angle,
                    read: Dimension::Length,
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
            (
                "IndexOutsideMember",
                EvalError::IndexOutsideMember {
                    var: editor_core::VarId::new(0, tagged(7)),
                },
            ),
            ("ModByNonPositive", EvalError::ModByNonPositive(0)),
            // No node raises it — a lowered document holds no authored
            // leaf, so a node's expression never reaches it — but a formula
            // evaluated outside a document does, and it renders through the
            // same `Display` within the same budget.
            (
                "Unlowered",
                EvalError::Unlowered(editor_core::LowerFault::Quantity {
                    dim: Dimension::Length,
                }),
            ),
        ]
    };
    let param_box = [
        (
            "UnknownParam",
            ParamBoxError::UnknownParam {
                param: editor_core::SpokenVar::new(
                    editor_core::VarId::new(0, tagged(7)),
                    Some(VarName::from_static("width")),
                ),
            },
        ),
        (
            "AxisUnrepresentable",
            ParamBoxError::AxisUnrepresentable {
                param: editor_core::SpokenVar::new(
                    editor_core::VarId::new(0, tagged(7)),
                    Some(VarName::from_static("width")),
                ),
                lo: 1.0,
                hi: 0.0,
            },
        ),
        ("SymbolsOverABox", ParamBoxError::SymbolsOverABox),
    ];
    let seed = [
        (
            "UnknownVar",
            SeedError::UnknownVar {
                var: editor_core::SpokenVar::new(
                    editor_core::VarId::new(0, tagged(7)),
                    Some(VarName::from_static("width")),
                ),
            },
        ),
        (
            "CountVar",
            SeedError::CountVar {
                var: editor_core::SpokenVar::new(
                    editor_core::VarId::new(0, tagged(7)),
                    Some(VarName::from_static("n")),
                ),
            },
        ),
        (
            "SeedOnNonFreeVar",
            SeedError::SeedOnNonFreeVar {
                var: editor_core::SpokenVar::new(
                    editor_core::VarId::new(0, tagged(7)),
                    Some(VarName::from_static("area")),
                ),
            },
        ),
        (
            "TangentUnrepresentable",
            SeedError::TangentUnrepresentable {
                var: editor_core::SpokenVar::new(
                    editor_core::VarId::new(0, tagged(7)),
                    Some(VarName::from_static("width")),
                ),
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
            "EntityNamed",
            NamingError::EntityNamed {
                name: Box::new(name()),
                held: Box::new(stable(EntityKind::Face, 4)),
            },
        ),
        (
            "KindMismatch",
            NamingError::KindMismatch {
                name: Box::new(name()),
                found: EntityKind::Vertex,
            },
        ),
        (
            "ShortTie",
            NamingError::ShortTie {
                name: Box::new(name()),
                candidates: 1,
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
                node: RecipeNodeId::new(0, tagged(3)),
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
            "UnsupportedSeamVertex",
            NamingError::UnsupportedSeamVertex {
                vertex: topo::VertexKey::default(),
                shape: editor_core::SeamVertexShape::OneOperandSeam,
            },
        ),
        (
            "UnsupportedCrossingSenses",
            NamingError::UnsupportedCrossingSenses {
                vertex: topo::VertexKey::default(),
                shape: editor_core::CrossingSensesShape::NameReadsTwoSenses,
            },
        ),
        (
            "UnsupportedSeamSides",
            NamingError::UnsupportedSeamSides {
                edge: topo::EdgeKey::default(),
                shape: editor_core::SeamSidesShape::NameSpellsTwoSeams,
            },
        ),
        (
            "UnsupportedJoin",
            NamingError::UnsupportedJoin {
                edge: topo::EdgeKey::default(),
                shape: editor_core::JoinShape::ChordBesideMergedFace,
            },
        ),
        (
            "UnsupportedUnionName",
            NamingError::UnsupportedUnionName {
                name: Box::new(stable(EntityKind::Vertex, 3)),
                shape: editor_core::UnionNameShape::SeveralParents,
            },
        ),
        (
            "UnsupportedTurnedCell",
            NamingError::UnsupportedTurnedCell {
                read: editor_core::VarId::new(0, tagged(38)),
                cell: topo::Cell::Vertex(topo::VertexKey::default()),
            },
        ),
        (
            "WayInZeroBand",
            NamingError::WayInZeroBand {
                edge: topo::EdgeKey::default(),
                shape: editor_core::WayShape::Carrier,
            },
        ),
        (
            "JoinedPieceOffRoot",
            NamingError::JoinedPieceOffRoot {
                edge: topo::EdgeKey::default(),
            },
        ),
        (
            "BandVerdictsDisagree",
            NamingError::BandVerdictsDisagree {
                vertex: topo::VertexKey::default(),
                shape: editor_core::DisagreeShape::PiecesAtOneVertex,
            },
        ),
        (
            "SharedRim",
            NamingError::SharedRim {
                node: RecipeNodeId::new(0, tagged(3)),
                face: topo::FaceKey::default(),
                other: topo::FaceKey::default(),
                found: RimShare::Several,
            },
        ),
        (
            "Settle",
            NamingError::Settle(topo::SettleError::Dead {
                vertex: topo::VertexKey::default(),
                reached: topo::Cell::Vertex(topo::VertexKey::default()),
            }),
        ),
        (
            "SeamVertexPartners",
            NamingError::SeamVertexPartners {
                vertex: topo::VertexKey::default(),
                candidates: vec![stable(EntityKind::Vertex, 3), stable(EntityKind::Vertex, 4)],
            },
        ),
        (
            "ChordOffRim",
            NamingError::ChordOffRim {
                edge: topo::EdgeKey::default(),
                node: RecipeNodeId::new(0, tagged(3)),
                face: topo::FaceKey::default(),
            },
        ),
        (
            "ConventionalVertex",
            NamingError::ConventionalVertex {
                vertex: topo::VertexKey::default(),
                body: 0,
            },
        ),
        (
            "ClosedCarrierUnread",
            NamingError::ClosedCarrierUnread {
                edge: topo::EdgeKey::default(),
                carrier: geom::CurveKind::Nurbs,
            },
        ),
        (
            "CarrierUnplaced",
            NamingError::CarrierUnplaced {
                edge: topo::EdgeKey::default(),
                carrier: geom::CurveKind::Nurbs,
            },
        ),
        (
            "PartnerUnread",
            NamingError::PartnerUnread {
                vertex: topo::VertexKey::default(),
                partner: topo::VertexKey::default(),
                unread: topo::UnreadPartner {
                    operand: topo::Operand::B,
                    vertex: topo::VertexKey::default(),
                    partner: topo::VertexKey::default(),
                },
            },
        ),
        (
            "MemberEdgeTied",
            NamingError::MemberEdgeTied {
                member: editor_core::VarId::new(0, tagged(7)),
                edge: Box::new(stable(EntityKind::Edge, 3)),
            },
        ),
        ("Band", NamingError::Band(band_error())),
        (
            "Escalated",
            NamingError::Escalated {
                predicate: "name_frag_on_member_edge",
                source: named("name_frag_on_member_edge"),
            },
        ),
    ];
    let at = RecipeNodeId::new(0, tagged(8));
    let vanished = |diagnosis| ResolveError::Vanished {
        name: name(),
        diagnosis,
        last_good: None,
    };
    let resolve = || {
        [
            (
                "Vanished(Cascade)",
                ResolveError::Vanished {
                    name: name(),
                    diagnosis: Diagnosis::Cascade { through: name() },
                    last_good: None,
                },
            ),
            (
                "Vanished(WitnessBifurcation)",
                ResolveError::Vanished {
                    name: name(),
                    diagnosis: Diagnosis::WitnessBifurcation(Box::new(
                        editor_core::WitnessBifurcation {
                            kind: editor_core::BifurcationKind::AmbiguousBasin,
                            margin: editor_core::BranchMarginEvidence {
                                margin: 3.0e-10,
                                band_zero: 1.0e-9,
                                band_escalate: 1.0e-8,
                            },
                            implicated: vec![editor_core::Implicated::Constraint(2)],
                            witness_age: editor_core::WitnessAge {
                                solved_under: Vec::new(),
                                at_solve: Vec::new(),
                            },
                        },
                    )),
                    last_good: None,
                },
            ),
            (
                "Vanished(PredicateFlip)",
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
                "Vanished(BorderDelta)",
                vanished(Diagnosis::BorderDelta {
                    node: at,
                    gone: vec![name()],
                    new: Vec::new(),
                }),
            ),
            (
                "Vanished(GroupResized)",
                vanished(Diagnosis::GroupResized {
                    node: at,
                    was: 2,
                    now: 1,
                    cutters: editor_core::GroupCutters::NotSeamBounded,
                }),
            ),
            (
                "Vanished(StructuralParam)",
                vanished(Diagnosis::StructuralParam {
                    node: at,
                    param: SlotId::Stations,
                }),
            ),
            (
                "Vanished(IndexCount)",
                vanished(Diagnosis::IndexCount {
                    node: at,
                    index: editor_core::VarId::new(0, tagged(2)),
                }),
            ),
            (
                "Vanished(RecipeEdit)",
                vanished(Diagnosis::RecipeEdit {
                    edit: RecipeEditRef::NodeDeleted { node: at },
                }),
            ),
            (
                "Vanished(Upstream)",
                vanished(Diagnosis::Upstream {
                    node: RecipeNodeId::new(0, tagged(9)),
                    cause: editor_core::UpstreamCause::PredicateFlip {
                        predicate: "side_of_plane",
                        at,
                        from: Sign::Negative,
                        to: Sign::Positive,
                    },
                }),
            ),
            (
                "Ambiguous",
                ResolveError::Ambiguous {
                    name: name(),
                    candidates: vec![name(), name()],
                    tie: TieWitness {
                        node: RecipeNodeId::new(0, tagged(4)),
                        at: name(),
                        width: 2,
                    },
                },
            ),
            (
                "ReadGone",
                ResolveError::ReadGone {
                    name: name(),
                    read: editor_core::VarId::new(0, tagged(7)),
                    edit: editor_core::ReadEdit::Deleted,
                },
            ),
            (
                "NodeGone",
                ResolveError::NodeGone {
                    name: name(),
                    edit: RecipeEditRef::NodeDeleted {
                        node: RecipeNodeId::new(0, tagged(3)),
                    },
                },
            ),
        ]
    };
    let mut rows: Vec<(String, NodeErrorKind)> = Vec::new();
    type EvalWrap = fn(EvalError) -> NodeErrorKind;
    let eval_wraps: [(&str, EvalWrap); 5] = [
        ("Expr", |source| NodeErrorKind::Expr {
            slot: SlotId::Distance,
            source,
        }),
        ("PoseScalar", |source| NodeErrorKind::PoseScalar {
            var: editor_core::VarId::new(0, tagged(4)),
            source,
        }),
        ("MeasureNonFinite", |source| {
            NodeErrorKind::MeasureNonFinite { source }
        }),
        ("PayloadExpr", |source| NodeErrorKind::PayloadExpr {
            what: "placement",
            index: 2,
            source,
        }),
        ("IndexCount", |source| NodeErrorKind::IndexCount {
            index: editor_core::VarId::new(0, tagged(2)),
            source,
        }),
    ];
    for (wrap, build) in eval_wraps {
        for (n, e) in eval() {
            rows.push(row(&format!("{wrap}/{n}"), build(e)));
        }
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
    for (n, e) in naming {
        rows.push(row(&format!("Naming/{n}"), NodeErrorKind::Naming(e)));
    }
    type Wrap = fn(Box<ResolveError>) -> NodeErrorKind;
    let wraps: [(&str, Wrap); 1] = [("SelectResolve", |error| NodeErrorKind::SelectResolve {
        slot: editor_core::OperandSlot::Selection,
        var: editor_core::VarId::new(0, 7),
        error,
        reference: 0,
    })];
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
        node: RecipeNodeId::new(0, tagged(node)),
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
    use editor_core::{
        BifurcationKind, BranchMarginEvidence, EntityKind, FaceName, Implicated, InterrogateError,
        PartFault, WitnessAge, WitnessBifurcation,
    };
    use payloads::*;
    use topo::{FaceKey, ReadbackError};
    let face = || stable(EntityKind::Face, 3);
    let mut rows = vec![
        row(
            "UnionFoldStep",
            NodeErrorKind::UnionFoldStep {
                member: 2,
                read: editor_core::VarId::new(0, tagged(2)),
                refusal: Box::new(NodeErrorKind::Boolean(topo::BooleanError::Escalated {
                    decision: topo::BooleanDecision::Coincidence(
                        topo::Coincide::Carriers,
                        topo::DeclarationRead::Moot,
                    ),
                    diag: diag(),
                })),
            },
        ),
        row(
            "SpaceMismatch/WorldAndAnother",
            NodeErrorKind::SpaceMismatch {
                a: editor_core::VarId::WORLD,
                b: editor_core::VarId::new(0, tagged(6)),
            },
        ),
        row(
            "SpaceMismatch/TwoOthers",
            NodeErrorKind::SpaceMismatch {
                a: editor_core::VarId::new(0, tagged(6)),
                b: editor_core::VarId::new(0, tagged(7)),
            },
        ),
        row(
            "CrossingUnverified",
            NodeErrorKind::CrossingUnverified {
                instance: RecipeNodeId::new(0, tagged(6)),
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
    ];
    for (n, kind) in [
        ("FoldProximity", BifurcationKind::FoldProximity),
        ("AmbiguousBasin", BifurcationKind::AmbiguousBasin),
        ("ResidualFailure", BifurcationKind::ResidualFailure),
    ] {
        rows.push(row(
            &format!("WitnessBifurcation/{n}"),
            NodeErrorKind::WitnessBifurcation(WitnessBifurcation {
                kind,
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
        ));
    }
    {
        for (n, refusal) in [
            (
                "Unsupported",
                MinSeparationRefusal::Unsupported {
                    carrier: "an approximated face",
                    face: FaceKey::default(),
                    side: topo::clearance::Which::First,
                },
            ),
            (
                "PoisonEnclosure",
                MinSeparationRefusal::PoisonEnclosure {
                    a: FaceKey::default(),
                    b: FaceKey::default(),
                },
            ),
            (
                "EmptyScope",
                MinSeparationRefusal::EmptyScope {
                    side: topo::clearance::Which::First,
                },
            ),
        ] {
            rows.push(row(
                &format!("MeasureClearanceRefused/{n}"),
                NodeErrorKind::MeasureClearanceRefused(refusal),
            ));
        }
    }
    let interrogate = [
        (
            "NodeNotEvaluated",
            InterrogateError::Standing(NodeStanding::NotEvaluated {
                node: RecipeNodeId::new(0, tagged(3)),
            }),
        ),
        (
            "NodeFailed",
            InterrogateError::Standing(NodeStanding::Failed {
                node: RecipeNodeId::new(0, tagged(3)),
            }),
        ),
        (
            "NodePoisoned",
            InterrogateError::Standing(NodeStanding::Poisoned {
                node: RecipeNodeId::new(0, tagged(3)),
                through: RecipeNodeId::new(0, tagged(2)),
            }),
        ),
        (
            "NodeNotInDocument",
            InterrogateError::Standing(NodeStanding::NotInDocument {
                node: RecipeNodeId::new(0, tagged(3)),
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
            "SeveralOutputs",
            InterrogateError::SeveralOutputs {
                node: RecipeNodeId::new(0, tagged(3)),
            },
        ),
        (
            "Readback",
            InterrogateError::Readback(ReadbackError::NoCarrier),
        ),
    ];
    for (n, error) in interrogate {
        rows.push(row(
            &format!("MeasureRefUnreadable/{n}"),
            NodeErrorKind::MeasureRefUnreadable {
                slot: editor_core::OperandSlot::Measured(editor_core::MeasureVerb::Distance, 0),
                var: editor_core::VarId::new(0, 7),
                error,
            },
        ));
    }
    // A part's piece failure carries the part's own node refusal, typed,
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
            "PartProductFailed",
            PartFault::PartProductFailed {
                held: Default::default(),
                node: RecipeNodeId::new(0, tagged(7)),
                refusal: NodeErrorKind::Extrude(sweep::ExtrudeError::DegenerateExtrusion).into(),
            },
        ),
        (
            "PartProductFailed(part)",
            PartFault::PartProductFailed {
                held: Default::default(),
                node: RecipeNodeId::new(0, tagged(7)),
                refusal: NodeErrorKind::Part {
                    doc_ref: doc_ref(),
                    fault: PartFault::DepthExceeded,
                }
                .into(),
            },
        ),
        (
            "PartProductPoisoned",
            PartFault::PartProductPoisoned {
                held: Default::default(),
                node: RecipeNodeId::new(0, tagged(8)),
                through: RecipeNodeId::new(0, tagged(7)),
                refusal: NodeErrorKind::Extrude(sweep::ExtrudeError::DegenerateExtrusion).into(),
            },
        ),
        (
            "PartProductFailureUnrecorded",
            PartFault::PartProductFailureUnrecorded {
                held: Default::default(),
                node: RecipeNodeId::new(0, tagged(7)),
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
/// part, whose world is empty, and a part whose one placement's body
/// was deleted.
fn part_products() -> Vec<(String, NodeErrorKind)> {
    use crate::fixture::resolver::{PartStore, with_resolver};
    use crate::fixture::{insert, len, on_frame, square};
    use editor_core::{
        CancelToken, DocumentId, Node, NodeResult, PartFault, ProductErrorKind, ProfileDoc,
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
    let stranded = {
        let (doc, profile) = sketch("concision-part-stranded");
        let (doc, body) = insert(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(1.0),
            },
        );
        // The product reads the placement's copy, and the placement
        // goes: the slot reads a body that is gone.
        let (doc, placement) = crate::fixture::place(doc, body);
        crate::fixture::step(doc, editor_core::DocEdit::DeleteNode { id: placement }).0
    };
    let mut store = PartStore::new();
    let parts = [
        (
            "Part/PartProduct",
            store.insert(sketch("concision-part-sketch").0, tol),
            ProductErrorKind::EmptyProduct,
        ),
        (
            "Part/PartProduct(StrandedProduct)",
            store.insert(stranded, tol),
            ProductErrorKind::StrandedProduct,
        ),
    ];
    let opts = with_resolver(store);
    parts
        .into_iter()
        .map(|(name, part, class)| {
            let (doc, instance) = insert(
                ProfileDoc::empty(DocumentId::derive(name), tol),
                crate::fixture::mated_instance(part),
            );
            match evaluate::<f64>(&doc, None, &CancelToken::new(), &opts, tol).result(instance) {
                Some(NodeResult::Failed(error)) => match &error.kind {
                    NodeErrorKind::Part {
                        doc_ref,
                        fault: fault @ PartFault::PartProduct { refusal, .. },
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
/// forwards the kernel's own refusal, which no document reaches: a
/// placement the at-rest gate refuses, an aggregate it refuses, and a graft the
/// kernel refuses. Each is built as the instance carries it, the
/// gather's refusal whole.
fn part_products_forwarding() -> Vec<(String, NodeErrorKind)> {
    use editor_core::{PartFault, ProductError, SourceFinding};
    let inside_out = || topo::ValidationError::NegativeVolume {
        solid: topo::SolidKey::default(),
    };
    [
        (
            "Part/PartProduct(PieceInvalid)",
            ProductError::PieceInvalid {
                findings: vec![SourceFinding {
                    node: RecipeNodeId::new(0, tagged(3)),
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
                node: RecipeNodeId::new(0, tagged(5)),
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
                    held: Default::default(),
                    refusal: error.into(),
                },
            },
        )
    })
    .collect()
}

fn mate() -> Vec<(String, NodeErrorKind)> {
    use editor_core::{DocumentId, LeverRefusal, MateFault as M, MateSide, NoChartWhy, VarKind};
    let n = |id| RecipeNodeId::new(0, tagged(id));
    let c = |id: u32| editor_core::ConstraintId(editor_core::MintId::new(id, 0));
    // Constraint `id` of placement 7, the copy every row's bundle places.
    let r = |id: u32| editor_core::MateRef::new(n(7), c(id));
    [
        (
            "PosesOfAnotherDocument",
            M::PosesOfAnotherDocument {
                expected: DocumentId::derive("a"),
                found: DocumentId::derive("b"),
            },
        ),
        (
            "Overconstrained",
            M::Overconstrained {
                placement: n(7),
                constraints: vec![c(8), c(9)],
                free: 0,
            },
        ),
        ("NotOnTheCopy", M::NotOnTheCopy { mate: r(9) }),
        (
            "KindsDiffer",
            M::KindsDiffer {
                mate: r(9),
                on: VarKind::Plane,
                to: VarKind::Axis,
            },
        ),
        (
            "NoChart",
            M::NoChart {
                value: r(9),
                why: NoChartWhy::SlideTurnsABall,
            },
        ),
        (
            "SideUnevaluated",
            M::SideUnevaluated {
                mate: r(9),
                side: MateSide::To,
                refusal: Box::new(
                    NodeErrorKind::PoseDegenerate {
                        construction: editor_core::pose::PoseConstruction::Through,
                    }
                    .into(),
                ),
            },
        ),
        (
            "Unleverable",
            M::Unleverable {
                mate: r(9),
                refusal: Box::new(LeverRefusal::Reach {
                    instance: n(6),
                    part: doc_ref(),
                    refusal: editor_core::ReachRefusal::NoExtent,
                }),
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
    use topo::{FaceKey, ReplaceFaceError, ShellError as S, ShellKey};
    let (face, other, shell) = (FaceKey::default(), FaceKey::default(), ShellKey::default());
    let mut rows: Vec<(String, S<f64>)> = replace_face()
        .into_iter()
        .map(|(arm, error)| {
            (
                format!("Face/{arm}"),
                S::Face {
                    face,
                    error: Box::new(error),
                },
            )
        })
        .collect();
    // The lift's refusals are the same door's; one that names an edge
    // stands for them.
    rows.push((
        "Lift".into(),
        S::Lift {
            face,
            error: Box::new(ReplaceFaceError::ReanchorPastCarrierEnd {
                edge: topo::EdgeKey::default(),
                gap: 0.25,
            }),
        },
    ));
    rows.extend(
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
            (
                "OffsetsCross",
                S::OffsetsCross {
                    face,
                    other,
                    overlap: 0.001,
                    thickness: 0.002,
                },
            ),
            (
                "CavityCrosses",
                S::CavityCrosses {
                    face,
                    other,
                    stations: [(0.5, 0.25), (0.75, 0.5)],
                    distance: 1e-16,
                },
            ),
            (
                "CavityUncertified",
                S::CavityUncertified {
                    refusal: topo::clearance::EngineRefusal::Uncertified {
                        a: face,
                        b: other,
                        shared: topo::clearance::SharedCell::Edge(topo::EdgeKey::default()),
                        arm: topo::clearance::CertificateArm::Departure,
                    },
                },
            ),
            ("ChartSenseMixed", S::ChartSenseMixed { face, other }),
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
                "OpenFaceChartPartial",
                S::OpenFaceChartPartial { face, other },
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
            ("Pcurve", S::Pcurve { source: pcurve() }),
            (
                "RimSeamMixed",
                S::RimSeamMixed {
                    edge: topo::EdgeKey::default(),
                    stations: geom_brep::MixedStations {
                        transverse: 3,
                        smooth: 4,
                    },
                },
            ),
            (
                "Join",
                S::Join {
                    refusal: join_refusal(),
                },
            ),
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
        .map(|(n, e)| (n.to_owned(), e)),
    );
    rows.into_iter()
        .map(|(n, e)| row(&format!("Shell/{n}"), NodeErrorKind::Shell(Box::new(e))))
        .collect()
}

/// **The offset door's join refusal, typed**: the refusal a regularity
/// reading in the band raises, carried as the door carries it
/// ([`topo::JoinRefusal`]) and rendered through its own `Display`, so a
/// regression in the door's words reds this row. A body whose reading
/// lands in the band is the STEP fixture `halfcap_eps6` (its split
/// vertex is 1e-8 m off the pole), which `step-import`'s rows feed to
/// the door itself; this crate cannot build one at the run's one
/// tolerance, since `split_edge` refuses inside the same band.
fn join_refused_offset() -> topo::ReplaceFaceError<f64> {
    topo::ReplaceFaceError::Join {
        refusal: join_refusal(),
    }
}

/// The in-band join reading every door that ends with the join carries
/// typed (`topo::JoinRefusal`).
fn join_refusal() -> topo::JoinRefusal {
    topo::JoinRefusal::Undecided(topo::JoinUndecided {
        vertex: topo::VertexKey::default(),
        reading: topo::JoinReading::Regularity(payloads::named("join_regular_point")),
    })
}

/// Every `topo::ReplaceFaceError` arm but `Fit` ([`offset_fit_routes`]
/// renders that one per fit refusal), and every `OffsetError` arm
/// through `Offset`. `TogetherAxialCorner` carries the longest `what`
/// any raise site passes.
fn replace_face() -> Vec<(String, topo::ReplaceFaceError<f64>)> {
    use geom::SurfaceKind::{Cone, Plane, Torus};
    use geom_brep::OffsetError as O;
    use payloads::*;
    use topo::{EdgeKey, FaceKey, ReplaceFaceError as R, VertexKey};
    let (face, other) = (FaceKey::default(), FaceKey::default());
    let (edge, vertex) = (EdgeKey::default(), VertexKey::default());
    let offset = |arm: &str, error| (format!("Offset/{arm}"), R::Offset { face, error });
    let mut rows = vec![
        offset(
            "RadiusFloor",
            O::RadiusFloor {
                kind: geom::SurfaceKind::Cylinder,
                realized: -0.1,
            },
        ),
        offset(
            "TorusRing",
            O::TorusRing {
                realized_minor: 2.0,
            },
        ),
        offset("NotClosedUnderOffset", O::NotClosedUnderOffset),
        offset("ApproxNesting", O::ApproxNesting),
        offset("Escalated", O::Escalated { source: diag() }),
    ];
    rows.extend(
        [
            (
                "Band",
                R::Band {
                    error: band_error(),
                },
            ),
            ("StaleFace", R::StaleFace { face }),
            (
                "ApproxLaneUnsupported",
                R::ApproxLaneUnsupported {
                    face,
                    scalar: "interval",
                },
            ),
            ("SharedSurfaceKey", R::SharedSurfaceKey { face, other }),
            ("EmptyGroup", R::EmptyGroup),
            ("GroupChartsDiffer", R::GroupChartsDiffer { face, other }),
            ("PlaceholderSurface", R::PlaceholderSurface { face }),
            (
                "ApexWindow",
                R::ApexWindow {
                    face,
                    v_min: 0.5,
                    v_max: 2.0,
                    shift: -0.75,
                },
            ),
            (
                "NappeStraddles",
                R::NappeStraddles {
                    face,
                    station_min: -0.5,
                    station_max: 1.5,
                    what: "faces of one cone that do not all lie on the same side of its apex, so \
                           one offset cannot move them all the same way",
                },
            ),
            ("ApexWindowUnknown", R::ApexWindowUnknown { face }),
            (
                "NeighborPairUnroutable",
                R::NeighborPairUnroutable {
                    edge,
                    kind: Torus,
                    other_kind: Cone,
                },
            ),
            (
                "NeighborPoseUnroutable",
                R::NeighborPoseUnroutable {
                    edge,
                    kind: Plane,
                    other_kind: Cone,
                    why: "the plane no longer passes through the cone's apex",
                },
            ),
            (
                "FittedBoundaryUnsupported",
                R::FittedBoundaryUnsupported {
                    edge,
                    what: "a curve on this face's fit that does not run along its fitted rows",
                },
            ),
            (
                "CarrierLaneUnsupported",
                R::CarrierLaneUnsupported {
                    edge,
                    what: "it is drawn on a fitted or cone surface, where the offset has no exact \
                           shift for it",
                },
            ),
            (
                "IsoRow",
                R::IsoRow {
                    edge,
                    error: geom_brep::IsoRowError::Escalated { source: diag() },
                },
            ),
            (
                "Structure",
                R::Structure {
                    edge,
                    error: geom_core::spline::SplineError::WeightCountMismatch {
                        weights: 3,
                        control: 4,
                    },
                },
            ),
            (
                "VertexDisagreement",
                R::VertexDisagreement { vertex, gap: 0.01 },
            ),
            (
                "EdgeSection",
                R::EdgeSection {
                    edge,
                    kind: geom::SurfaceKind::Plane,
                    other_kind: geom::SurfaceKind::Nurbs,
                    verdict: topo::SectionVerdict::NoBranch,
                },
            ),
            (
                "CornerSection",
                R::CornerSection {
                    vertex,
                    edge,
                    verdict: topo::CornerVerdict::NoRoot,
                },
            ),
            ("DeclaredEdgeTilted", R::DeclaredEdgeTilted { edge }),
            (
                "MixedEdge",
                R::MixedEdge {
                    edge,
                    stations: geom_brep::MixedStations {
                        transverse: 3,
                        smooth: 4,
                    },
                },
            ),
            (
                "ReanchorPastCarrierEnd",
                R::ReanchorPastCarrierEnd { edge, gap: 0.25 },
            ),
            (
                "ReanchorCollapse",
                R::ReanchorCollapse { edge, offset: -1.0 },
            ),
            (
                "ReanchorInconclusive",
                R::ReanchorInconclusive {
                    edge,
                    error: geom::ProjectionInconclusive {
                        iterations: 50,
                        last_t: 0.5,
                        last_orthogonality: 1e-3,
                        last_distance: 0.1,
                    },
                },
            ),
            (
                "NurbsLaneUnsupported",
                R::NurbsLaneUnsupported {
                    edge,
                    scalar: "interval",
                },
            ),
            (
                "TogetherNonPlanar",
                R::TogetherNonPlanar { face, kind: Torus },
            ),
            ("TogetherPartialSet", R::TogetherPartialSet { face }),
            (
                "TogetherCorner",
                R::TogetherCorner {
                    vertex,
                    planes: 2,
                    what: "fewer than three distinct planes meet here, so no point is \
                           determined",
                },
            ),
            ("TogetherChartMixed", R::TogetherChartMixed { face, other }),
            ("TogetherFaceRepeated", R::TogetherFaceRepeated { face }),
            (
                "TogetherEdgeDisagreement",
                R::TogetherEdgeDisagreement { edge, gap: 0.05 },
            ),
            (
                "TogetherAxialUnsupported",
                R::TogetherAxialUnsupported {
                    face,
                    kind: geom::SurfaceKind::Nurbs,
                },
            ),
            (
                "TogetherNotAxial",
                R::TogetherNotAxial {
                    face,
                    what: "a cylinder whose axis is skew to the body's",
                },
            ),
            (
                "TogetherAxialCorner",
                R::TogetherAxialCorner {
                    vertex,
                    surfaces: 2,
                    what: "a line profile and a plane parallel to the axis meet here off the \
                           axis: the plane fixes an azimuth and the line one coordinate, and \
                           nothing records where along the line the corner stands",
                },
            ),
            (
                "TogetherAxialEdge",
                R::TogetherAxialEdge {
                    edge,
                    what: "a rim whose moved cap stands at or past the torus wall's inner equator, \
                           or out of its reach — the section there is a node, one folded loop or \
                           nothing, not two ovals",
                },
            ),
            (
                "TogetherConicArc",
                R::TogetherConicArc {
                    edge,
                    conic: geom_brep::OpenConic::Hyperbola,
                },
            ),
            (
                "DegenerateTorus/Undecided",
                R::DegenerateTorus {
                    edge,
                    refusal: geom_brep::TorusRefusal {
                        convention: geom_brep::TorusConvention::Ring,
                        verdict: geom_brep::TorusVerdict::Undecided(named("ring_torus_convention")),
                    },
                },
            ),
            (
                "DegenerateTorus/Refused",
                R::DegenerateTorus {
                    edge,
                    refusal: geom_brep::TorusRefusal {
                        convention: geom_brep::TorusConvention::Tube,
                        verdict: geom_brep::TorusVerdict::Refused(
                            geom_brep::recourse::Refused::Zero(geom_brep::recourse::Classified {
                                margin: geom_core::MarginDiag::value(5.0e-10),
                                band: band(),
                            }),
                        ),
                    },
                },
            ),
            ("Escalated", R::Escalated { source: diag() }),
            (
                "Op",
                R::Op {
                    edge: Some(edge),
                    error: euler(),
                },
            ),
            (
                "Op(rechart)",
                R::Op {
                    edge: None,
                    error: euler(),
                },
            ),
            ("Pcurve", R::Pcurve { source: pcurve() }),
            ("Join", join_refused_offset()),
            (
                "ResultNotClosed",
                R::ResultNotClosed {
                    errors: vec![topo::ValidationError::ShellDisconnected {
                        shell: topo::ShellKey::default(),
                        components: 2,
                    }],
                },
            ),
        ]
        .map(|(n, e)| (n.to_owned(), e)),
    );
    rows
}

/// **Every checks-window finding fits the window it is listed in.** The
/// checks window draws each finding's `Display` verbatim beside its
/// placement's button, so each `CheckEvidence` arm is rendered as the window
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

/// **The checks window's shell-role evidence ends in the reading that
/// refused it** (D4 ¶1 (i)), on refusals real bodies make through the
/// real doors.
///
/// A shell's role off its volume is the sign of the volume's enclosure
/// alone (D10). At ε 1e-9 a brick less a needle `1 × 10.5ε × 10.5ε`,
/// turned off the axes, holds the needle as a cavity no aimed ray reads
/// (each meets a wedge); read through the structural
/// door, which holds no interval lane, its volume lies within its sums'
/// rounding, so its role is unresolved in that scalar: a capability limit
/// of the measurement, with no tolerance offered and no defect claimed.
/// Read through the certifying door, its enclosure decides it a void. A
/// certified enclosure that holds zero is the kernel's or the file's, and
/// the one real body that reaches it, a cube whose planes are NaN, is
/// pinned at topo's own door (`pieces::tests::a_poisoned_role_beside_two_pieces_offers_no_tolerance`).
/// No row the window writes itself advises lowering the tolerance.
#[test]
fn every_escalated_check_finding_ends_in_its_decisions_recourse() {
    use editor_core::{CheckEvidence, CheckFinding, CheckId};
    use geom_core::Tol;
    let tol = Tol::witness();
    let render = |evidence| {
        CheckFinding {
            check: CheckId::Connectedness,
            subject: editor_core::FindingSubject::Output {
                placement: RecipeNodeId::new(0, tagged(4)),
                output_ix: 0,
            },
            evidence,
            quiet: None,
        }
        .to_string()
    };
    let head = "check connectedness: placement 000000000004 output 0: ";
    if (tol.eps() - 1e-9).abs() < 1e-15 {
        let at_rest = |b| topo::AtRestBody::validate(b, tol).expect("the brick is at rest");
        let outer = at_rest(topo::test_support::brick::<f64>(
            (-1.0, 2.0),
            (-1.0, 2.0),
            (-1.0, 2.0),
            tol,
        ));
        let w = 10.5 * tol.eps();
        // Turned off the axes: every aimed ray at the cavity meets a wedge
        // (a face it passes within the band of two edges of), where along
        // the axes a ray aimed at the cavity's end reads it.
        let (x, y, z) = {
            let n = (37.0_f64 * 37.0 + 21.0 * 21.0 + 13.0 * 13.0).sqrt();
            (37.0 / n, -21.0 / n, 13.0 / n)
        };
        let (s, c) = 0.7_f64.sin_cos();
        let k = 1.0 - c;
        let r = [
            [c + x * x * k, x * y * k - z * s, x * z * k + y * s],
            [y * x * k + z * s, c + y * y * k, y * z * k - x * s],
            [z * x * k - y * s, z * y * k + x * s, c + z * z * k],
        ];
        let needle = at_rest(topo::test_support::mapped_cube::<f64>(
            move |a, b, cc| {
                let p = [a, b * w, cc * w];
                let at = |i: usize| r[i][0] * p[0] + r[i][1] * p[1] + r[i][2] * p[2];
                geom_core::Point3::new(at(0), at(1), at(2))
            },
            tol,
        ));
        let cut = topo::subtract(&outer, &needle, tol).expect("the needle cavity builds");
        let body = &cut.body().expect("a body").body;
        // The door names the first ray set aside at a corner or a wedge
        // beside the volume's refusal; the refusal is the volume's.
        let source = match topo::classify_shells_structural(body, tol)
            .expect_err("the lane-free door reads no role for the needle cavity")
        {
            topo::ShellClassifyError::Corner { volume, .. } => *volume,
            other => other,
        };
        assert!(
            matches!(
                source,
                topo::ShellClassifyError::SignUnresolved {
                    limit: topo::SignLimit::Scalar,
                    ..
                }
            ),
            "{source:?}"
        );
        let words = "the sign of a shell's volume cannot be read: this scalar carries no \
                     interval to certify its sums, and the volume lies within their rounding";
        let ending = geom_core::NOT_YET_ENDING;
        assert_eq!(source.to_string(), format!("{words}. {ending}"));
        let text = render(CheckEvidence::Unsupported {
            source: source.clone(),
        });
        assert!(text.starts_with(head), "{text}");
        assert!(text.ends_with(&format!("{words}. {ending}")), "{text}");
        assert!(
            !text.contains("tolerance") && !text.contains("defect"),
            "{text}"
        );
        let roles: Vec<topo::ShellRole> = topo::classify_shells(body, tol)
            .expect("the certifying door reads the cavity")
            .iter()
            .map(|c| c.role)
            .collect();
        assert_eq!(roles, [topo::ShellRole::Outer, topo::ShellRole::Void]);
    }
    // A band failure is the run's configuration, not the shell-role
    // decision: the finding forwards its payload and adds no lever.
    let error = payloads::band_error();
    let unowned = render(CheckEvidence::Escalated {
        source: topo::ShellClassifyError::Band { error },
    });
    assert!(unowned.ends_with(&error.to_string()), "{unowned}");
    assert!(!unowned.contains("thicken"), "{unowned}");
    for (name, finding) in &check_findings() {
        let text = finding.to_string();
        assert!(!text.contains("lower"), "{name}: {text}");
    }
}

fn check_findings() -> Vec<(String, editor_core::CheckFinding)> {
    use editor_core::{CheckEvidence as E, CheckFinding, CheckId};
    use geom_core::{Indeterminate, MarginDiag};
    use payloads::*;
    use topo::{
        CoherenceCondition, CoherenceFinding, EdgeKey, FaceKey, LoopKey, ShellClassifyError,
        ShellKey, StructureRead, Unexaminable, Unexamined, VertexKey,
    };
    let shell = ShellKey::default();
    let copy = |n| editor_core::CopyRef {
        var: editor_core::VarId::new(0, tagged(n)),
        member: 0,
    };
    let interference = |overlap| editor_core::InterferenceFinding {
        a: copy(7),
        b: copy(8),
        evidence: Vec::new(),
        overlap,
        quiet: None,
        quiet_at: Vec::new(),
        quietable: true,
    };
    let finding = |check, evidence| CheckFinding {
        check,
        subject: editor_core::FindingSubject::Output {
            placement: RecipeNodeId::new(0, tagged(4)),
            output_ix: 0,
        },
        evidence,
        quiet: None,
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
    vec![
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
            "NotAtRest",
            finding(
                CheckId::Connectedness,
                E::NotAtRest {
                    errors: vec![topo::ValidationError::NegativeVolume {
                        solid: topo::SolidKey::default(),
                    }],
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
                    source: ShellClassifyError::Straddles {
                        shell,
                        low: MarginDiag::value(-1e-18),
                        high: MarginDiag::value(1e-18),
                    },
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
            "Interference(bounded)",
            finding(
                CheckId::AtRest,
                E::Interference {
                    finding: Box::new(interference(editor_core::Overlap::Bounded {
                        faces: vec![editor_core::FaceSite {
                            copy: copy(7),
                            face: editor_core::StableName {
                                kind: editor_core::EntityKind::Face,
                                node: RecipeNodeId::new(0, tagged(9)),
                                path: vec![editor_core::RoleSeg::Cap(editor_core::CapEnd::End)],
                            },
                        }],
                    })),
                },
            ),
        ),
        (
            "Interference(unlocalized)",
            finding(
                CheckId::AtRest,
                E::Interference {
                    finding: Box::new(interference(editor_core::Overlap::Unlocalized(
                        editor_core::Unlocalized::Lane,
                    ))),
                },
            ),
        ),
        (
            "AtRest(not run)",
            finding(
                CheckId::AtRest,
                E::AtRest {
                    evidence: editor_core::AtRestEvidence::NotRun,
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
    .collect::<Vec<_>>()
}

/// **Every detail a blend refusal is raised with fits its chain.** The
/// `UnsupportedChain`, `UnsupportedRunOut`, `UnsupportedGeometry` and
/// `BodyNotIntact` arms each render a raise site's `detail` in front of
/// their recourse, and the rows above render one representative detail
/// apiece. So this reads every detail the blend module raises those arms
/// with — the second argument of each `unbuilt_chain`,
/// `unbuilt_run_out`, `unbuilt_geometry`, `unresolved`, `unwalked`,
/// `disagrees`, `scaffolding` and `degenerate` call in
/// `sweep/src/blend`, and the `detail:` of each of those arms built
/// directly, a literal or a `const` resolved in the same tree — and
/// renders each through the feature tree's chain, under both verbs.
/// A raise site added later is read the day it is written.
#[test]
fn every_blend_detail_renders_within_the_budget() {
    use sweep::blend::{BlendError as E, BlendKind, Breach};
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
            "unresolved" => E::BodyNotIntact {
                at,
                breach: Breach::Unresolved,
                detail,
            },
            "unwalked" => E::BodyNotIntact {
                at,
                breach: Breach::Unwalked,
                detail,
            },
            "disagrees" => E::BodyNotIntact {
                at,
                breach: Breach::Disagrees,
                detail,
            },
            "scaffolding" => E::BodyNotIntact {
                at,
                breach: Breach::Scaffolding,
                detail,
            },
            "degenerate" => E::BodyNotIntact {
                at,
                breach: Breach::Degenerate,
                detail,
            },
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
        "unresolved",
        "unwalked",
        "disagrees",
        "scaffolding",
        "degenerate",
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
    // `BodyNotIntact` names the entity where the body did not hold
    // together: a corrupt body, whose report needs the key. Keyed by the
    // arm the detail is raised with, not by the row's name.
    let problems: Vec<String> = rows
        .iter()
        .zip(details.iter().flat_map(|d| [d, d]))
        .flat_map(|((name, text), (helper, _, _))| {
            eprintln!("MEASURE {} {name}: {text}", text.split_whitespace().count());
            let keyed = matches!(
                *helper,
                "unresolved" | "unwalked" | "disagrees" | "scaffolding" | "degenerate"
            );
            test_utils::refusal::problems(name, text, &[], keyed)
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
            "unresolved",
            "unwalked",
            "disagrees",
            "scaffolding",
            "degenerate",
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
        // not a raise site. A direct `BodyNotIntact` is rendered as its
        // longest breach, so its detail is measured at the worst case.
        for (arm, helper) in [
            ("UnsupportedChain", "unbuilt_chain"),
            ("UnsupportedRunOut", "unbuilt_run_out"),
            ("UnsupportedGeometry", "unbuilt_geometry"),
            ("BodyNotIntact", longest_breach()),
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

/// The helper of the breach whose `BodyNotIntact` rendering is longest,
/// measured rather than assumed, so a kind added or reworded moves it.
/// The match is exhaustive: a new kind does not compile here until it
/// is named, and then belongs in the list.
fn longest_breach() -> &'static str {
    use sweep::blend::{BlendError, Breach};
    let helper = |breach| match breach {
        Breach::Unresolved => "unresolved",
        Breach::Unwalked => "unwalked",
        Breach::Disagrees => "disagrees",
        Breach::Scaffolding => "scaffolding",
        Breach::Degenerate => "degenerate",
    };
    let at = topo::EntityId::Face(topo::FaceKey::default());
    [
        Breach::Unresolved,
        Breach::Unwalked,
        Breach::Disagrees,
        Breach::Scaffolding,
        Breach::Degenerate,
    ]
    .into_iter()
    .max_by_key(|&breach| {
        let detail = "";
        BlendError::BodyNotIntact { at, breach, detail }
            .to_string()
            .split_whitespace()
            .count()
    })
    .map(helper)
    .expect("a breach to measure")
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
