//! Second-review probes for PR 4252: word counts of the operand-gate
//! refusals as the feature tree draws them. Not for merge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{NodeError, NodeErrorKind, RecipeNodeId};
use sweep::blend::{BlendError, BlendKind};
use test_utils::refusal::tagged;
use topo::{EdgeKey, FaceKey, LoopKey, SolidKey, ValidationError, VertexKey};

fn shown(kind: NodeErrorKind) -> String {
    NodeError {
        node: RecipeNodeId(tagged(5)),
        kind,
        escalations: std::sync::Arc::new(Vec::new()),
    }
    .to_string()
}

#[test]
fn probe_operand_gate_refusals_against_the_budget() {
    let findings = [
        ValidationError::NegativeVolume {
            solid: SolidKey::default(),
        },
        ValidationError::ScaffoldingStrutVertex {
            vertex: VertexKey::default(),
        },
        ValidationError::ScaffoldingEmptyLoop {
            loop_: LoopKey::default(),
        },
        ValidationError::DescriptionNotAdjacent {
            edge: EdgeKey::default(),
        },
        ValidationError::ScaffoldAtRest {
            edge: EdgeKey::default(),
        },
    ];
    let mut rows: Vec<(String, String)> = Vec::new();
    for verb in [BlendKind::Fillet, BlendKind::Chamfer] {
        for (n, e) in [
            (
                "ScaffoldingOperand",
                BlendError::ScaffoldingOperand {
                    errors: vec![findings[1].clone()],
                },
            ),
            (
                "InsideOutOperand",
                BlendError::InsideOutOperand {
                    errors: vec![findings[0].clone()],
                },
            ),
        ] {
            rows.push((
                format!("Blend/{verb:?}/{n}"),
                shown(NodeErrorKind::Blend { verb, error: e }),
            ));
        }
    }
    for f in &findings {
        rows.push((
            format!("UnfinishedOperand/{f:?}"),
            shown(NodeErrorKind::UnfinishedOperand {
                input: RecipeNodeId(tagged(3)),
                errors: vec![f.clone()],
            }),
        ));
    }
    let _ = FaceKey::default();
    let mut bad = Vec::new();
    for (name, text) in &rows {
        let n = text.split_whitespace().count();
        eprintln!("MEASURE {n} {name}: {text}");
        let p = test_utils::refusal::problems(name, text, &[], false);
        if !p.is_empty() {
            bad.push(format!("{name}: {p:?}"));
        }
    }
    eprintln!("PROBLEMS:\n{}", bad.join("\n"));
}
