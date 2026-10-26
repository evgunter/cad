//! **Every at-rest finding the viewer shows fits where it is shown.**
//!
//! A kernel `topo::ValidationError` reaches the viewer as one line of a
//! gate's refusal, under a header naming the gate: the at-rest badge
//! draws `AssemblyError`'s `Display` behind "at rest: "
//! (`viewer::frame::at_rest_badge`), and the product badge draws
//! `ProductError`'s, which opens "product: " (`viewer::frame::product_badge`).
//! These rows render every sample —
//! `topo::test_support::validation_error_samples`, one per arm and one
//! per variant of every nested refusal an arm renders, which `topo`'s
//! own coverage row holds complete — through every one of those
//! wrappers a finding of that arm can arrive in, and hold each rendering
//! to the standard [`refusal_concision`](crate::refusal_concision)
//! states, through the shared checker `test_utils::refusal::problems`:
//! the word budget, no stage prefix, no `Debug` struct, no arena key
//! (the tier-1/2 structure arms excepted: they report a damaged body,
//! and the key is what the bug report needs), and exactly ONE recourse
//! — plus one line per finding. Every admission must admit something a
//! sample renders, so one its owner's fix made stale goes red.
//!
//! **The wrappers rendered.** A one-finding at-rest refusal, and the
//! product gate's refusal, alone and behind the at-rest badge.
//!
//! **What a row here cannot see.** A refusal listing several findings
//! is several lines under one header, and its length grows with the
//! count; a field a raise site fills with prose is rendered at the values the
//! samples name, not every value a run can write (`topo`'s
//! `test_support_samples` docs say which).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{AssemblyError, AtRestFinding, ProductError, RecipeNodeId, SourceFinding};
use test_utils::refusal::tagged;
use topo::ValidationError;

/// The labels a finding legitimately opens with, each on the routes
/// (or the one sample) whose rendering writes it: the two badges' names
/// as the viewer writes them, and the product gate's placement address.
const LABELS: &[(&str, &str)] = &[
    ("at rest", "at rest"),
    ("at rest, product", "at rest"),
    ("product", "product"),
    ("at rest, product", "product"),
    ("product", "placement 000000000005 output 0"),
    ("at rest, product", "placement 000000000005 output 0"),
    // A sentence whose clause carries no word the shape check reads as
    // a sentence's: a numeral subject and a bare verb.
    ("InstanceInterference", "two instances overlap"),
];

/// The tier-1/2 structure arms: each reports a damaged body, where the
/// arena key is what the bug report needs. Every other arm names what
/// it is about in words.
const KERNEL_KEYED: &[&str] = &[
    "DanglingTopology",
    "DanglingGeometry",
    "DanglingDescription",
    "NextPrevMismatch",
    "LoopCycleOverrun",
    "ParentLoopMismatch",
    "UnreachableHalfEdge",
    "EdgeHalvesIdentical",
    "EdgeSlotBackpointerMismatch",
    "HalfEdgeUnclaimed",
    "HalfEdgeMultiplyClaimed",
    "EdgeNotAntiparallel",
    "EmanatingStartMismatch",
    "EmptyLoopVertexWithEmanating",
    "LoneVertexWithIncidence",
    "VertexOrbitOverrun",
    "OrbitForeignMember",
    "SplitVertexOrbit",
    "OuterListedAsRing",
    "BackPointerMismatch",
    "OrphanEntity",
    "MultiplyOwned",
    "OrphanGeometry",
    "SolidWithoutShells",
    "ShellWithoutFaces",
    "EdgeAcrossShells",
    "ComponentEulerViolation",
    "MissingProvenance",
    "LeakedProvenance",
    "ScaffoldingEmptyLoop",
    "ScaffoldingStrutVertex",
    "ShellDisconnected",
    "NullScaffoldShared",
    "LeakedNullFaceRecord",
    "StaleNullFaceLoop",
    "StaleNullFaceOwnership",
    "NullEdgeAtRest",
    "NullFaceAtRest",
];

/// Every wrapper `error` can arrive in, rendered as the viewer draws it.
fn renderings(error: &ValidationError) -> Vec<(&'static str, String)> {
    let at_rest = AssemblyError::AtRest {
        findings: vec![AtRestFinding {
            error: error.clone(),
        }],
    };
    let product = ProductError::RootInvalid {
        findings: vec![SourceFinding {
            node: RecipeNodeId::new(0, tagged(5)),
            output: 0,
            errors: vec![error.clone()],
        }],
    };
    vec![
        ("at rest", format!("at rest: {at_rest}")),
        ("product", product.to_string()),
        (
            "at rest, product",
            format!("at rest: {}", AssemblyError::Product(Box::new(product))),
        ),
    ]
}

#[test]
fn every_at_rest_finding_renders_to_the_standard() {
    let mut problems = Vec::new();
    let mut used = std::collections::BTreeSet::new();
    for (label, error) in topo::test_support::validation_error_samples() {
        let keyed = KERNEL_KEYED.contains(&label.as_str());
        if keyed
            && renderings(&error)
                .iter()
                .any(|(_, t)| test_utils::refusal::arena_key(t))
        {
            used.insert(format!("KERNEL_KEYED {label}"));
        }
        for (route, text) in renderings(&error) {
            let name = format!("{label} ({route})");
            eprintln!("MEASURE {} {name}: {text}", text.split_whitespace().count());
            let allowed: Vec<&str> = LABELS
                .iter()
                .filter(|(scope, _)| *scope == route || *scope == label)
                .map(|(_, l)| *l)
                .collect();
            for prefix in test_utils::refusal::stage_prefixes(&text, &[]) {
                let prefix = prefix.trim_end_matches(':');
                if let Some((scope, l)) = LABELS
                    .iter()
                    .find(|(scope, l)| (*scope == route || *scope == label) && *l == prefix)
                {
                    used.insert(format!("LABELS {scope} {l}"));
                }
            }
            problems.extend(test_utils::refusal::problems(&name, &text, &allowed, keyed));
            // A header and ONE finding line: a line break inside a
            // finding (a `\` continuation left inside a literal) would
            // split it across the list.
            if text.lines().count() != 2 {
                problems.push(format!(
                    "{name} renders {} lines, not a header and one finding: {text}",
                    text.lines().count()
                ));
            }
        }
    }
    // Every admission is used: an entry no sample needs is stale.
    for entry in KERNEL_KEYED
        .iter()
        .map(|l| format!("KERNEL_KEYED {l}"))
        .chain(LABELS.iter().map(|(s, l)| format!("LABELS {s} {l}")))
    {
        if !used.contains(&entry) {
            problems.push(format!(
                "the admission {entry} admits nothing a sample renders"
            ));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}
