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
//! marker — plus one line per finding.
//!
//! **The wrappers rendered.** Each at its longest: a one-finding
//! refusal naming no mate; the same finding attributed to a mate of
//! ANOTHER document, which renders the route it was carried by (every
//! arm `editor_core::assembly`'s attribution can pin on a declaration);
//! the `Uncertified` frontier's header over a declined, carried
//! declaration (the arm it can decline); and the product gate's
//! refusal, alone and behind the at-rest badge.
//!
//! **What a row here cannot see.** A refusal listing several findings
//! is several lines under one header, and its length grows with the
//! count; a route through several instances adds a hop per instance;
//! a field a raise site fills with prose is rendered at the values the
//! samples name, not every value a run can write (`topo`'s
//! `test_support_samples` docs say which).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    AssemblyError, AtRestFinding, Attribution, DocumentId, EntityKind, MintedDeclaration,
    ProductError, RecipeNodeId, Relation, RoleSeg, Route, StableName,
};
use topo::{ContactClass, FaceKey, ValidationError};

/// The labels a finding legitimately opens with: the two badges'
/// names, as the viewer writes them.
const LABELS: &[&str] = &["at rest", "product"];

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
    "NullEdgeAtRest",
    "NullFaceAtRest",
];

fn minted() -> MintedDeclaration {
    let name = |node| StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(node),
        path: vec![RoleSeg::OutputBody],
    };
    MintedDeclaration {
        mate: RecipeNodeId(7),
        a: name(1),
        b: name(2),
        class: ContactClass::Rest,
        faces: (FaceKey::default(), FaceKey::default()),
    }
}

/// A mate of another document, reached through one instance.
fn carried(relation: Relation) -> Attribution {
    Attribution::Carried {
        route: Route {
            through: RecipeNodeId(4),
            of: DocumentId::derive("the bracket part"),
            via: Vec::new(),
        },
        declaration: minted(),
        relation,
    }
}

/// Every wrapper `error` can arrive in, rendered as the viewer draws it.
fn renderings(error: &ValidationError) -> Vec<(&'static str, String)> {
    let at_rest = |attribution: Attribution| {
        let refusal = AssemblyError::AtRest {
            findings: vec![AtRestFinding {
                attribution,
                error: error.clone(),
            }],
        };
        format!("at rest: {refusal}")
    };
    let product = ProductError::SolidInvalid {
        node: RecipeNodeId(5),
        errors: vec![error.clone()],
    };
    let mut out = vec![
        ("unattributed", at_rest(Attribution::Unattributed)),
        ("product", product.to_string()),
        (
            "at rest, product",
            format!("at rest: {}", AssemblyError::Product(Box::new(product))),
        ),
    ];
    // The arms attribution can pin on a declaration it refutes.
    if matches!(
        error,
        ValidationError::ContactContradicted { .. }
            | ValidationError::StaleContactDeclaration { .. }
            | ValidationError::CensusUnsupported { .. }
    ) {
        out.push(("refuted, carried", at_rest(carried(Relation::Refuted))));
    }
    // The arm the census declines on a declared pair: the frontier.
    if matches!(error, ValidationError::CensusUnsupported { .. }) {
        let refusal = AssemblyError::Uncertified {
            contacts: Box::default(),
            findings: vec![AtRestFinding {
                attribution: carried(Relation::Declined),
                error: error.clone(),
            }],
        };
        out.push(("declined, carried", format!("at rest: {refusal}")));
    }
    out
}

#[test]
fn every_at_rest_finding_renders_to_the_standard() {
    let mut problems = Vec::new();
    for (label, error) in topo::test_support::validation_error_samples() {
        let keyed = KERNEL_KEYED.contains(&label.as_str());
        for (route, text) in renderings(&error) {
            let name = format!("{label} ({route})");
            eprintln!("MEASURE {} {name}: {text}", text.split_whitespace().count());
            problems.extend(test_utils::refusal::problems(&name, &text, LABELS, keyed));
            if test_utils::refusal::recourse_markers(&text) == 0 {
                problems.push(format!("{name} states no recourse: {text}"));
            }
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
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}
