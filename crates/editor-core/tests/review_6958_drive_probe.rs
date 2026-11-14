//! **Reviewer probe: a slab under a taker, driven over boxes from a
//! millionth to a tenth of its depth** (`topo`'s side reader,
//! `side_reader::aimed_ray`, read by the union's take in both lanes).
//!
//! `onehome_switch_parity_interval` pins this document at half-widths
//! `1e-6` and `1e-9`, where every ray reads in both lanes. Over wider
//! boxes the box lane's first aimed ray is set aside before another
//! reads, where the witness reads by its first: this prints what the
//! drive makes of that (the receipt and each refusal's reason), and
//! asserts only that the receipt holds and that no leaf is refused as a
//! flip, which would be a finding about a document whose structure does
//! not change over the box.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::Recorder;
use crate::m10_3_driver_interval::{param, uniform, unit_square};

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DEFAULT_MAX_DEPTH, DriveConfig, drive};
use editor_core::{Bodies, Dimension, DocEdit, FreeVar, Node, ProfileProgram, UnitSym, VarName};
use geom_core::Tol;

/// The slab of depth `1 ± half`, under a one-member union where
/// `taken`.
fn slab(half: f64, taken: bool) -> editor_core::ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("depth"),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value: 1.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(uniform(half)),
        }),
    });
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![unit_square()],
        ids: Vec::new(),
        on: None,
    }));
    let body = r.insert(Node::Extrude {
        profile: p.into(),
        distance: param("depth"),
    });
    if taken {
        r.insert(Node::Union {
            members: Bodies::Spelled(vec![body.into()]),
        });
    }
    r.doc
}

#[test]
fn a_slab_under_a_taker_over_wider_boxes_is_never_refused_as_a_flip() {
    for taken in [true, false] {
        for half in [1e-6, 1e-4, 1e-3, 3e-3, 1e-2, 2.5e-2, 5e-2, 0.1] {
            let doc = slab(half, taken);
            let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
            let v = drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    max_depth: DEFAULT_MAX_DEPTH,
                    max_leaves: 64,
                    ..DriveConfig::default()
                },
                &editor_core::Stop::default(),
                Tol::witness(),
            )
            .expect("the drive runs");
            assert!(v.receipt().holds(), "{:?}", v.receipt());
            let reasons: Vec<String> = v
                .refused()
                .iter()
                .map(|l| format!("{:?}", l.reason).chars().take(200).collect())
                .collect();
            println!(
                "[review 6958] ε {:e} taken={taken} half {half:e}: {:?}; refused: {reasons:#?}",
                Tol::witness().eps(),
                v.receipt()
            );
            assert!(
                reasons.iter().all(|r| !r.contains("FlipCrossing")),
                "taken={taken}, half {half}: a leaf refused as a flip: {reasons:#?}"
            );
        }
    }
}
