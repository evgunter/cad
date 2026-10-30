//! Review probe for PR 6054: how a side escalation reads.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::chart_singularity::{LoopWalk, Singularity, loop_holds_on_left, nappe};
use geom_core::Band;

#[test]
fn probe_side_escalations_render_with_the_boundary_distance_ending() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    // A cone loop ringing the axis whose slant range is centred in band.
    let walk = LoopWalk {
        winding: 1,
        area: 0.0,
        length: 1.0,
        mid_v: 3e-9,
    };
    let cause = nappe(&walk, band).unwrap_err();
    let e = topo::ValidationError::ChartSingularityNotAVertex {
        face: Default::default(),
        point: Singularity::Apex,
        lies: topo::SingularityLies::Undecided { cause },
    };
    eprintln!("NAPPE: {e}");
    // A winding-free loop whose image reads no orientation.
    let flat = LoopWalk {
        winding: 0,
        area: 0.0,
        length: 1.0,
        mid_v: 1.0,
    };
    let cause = loop_holds_on_left(&flat, Singularity::North, band).unwrap_err();
    eprintln!("SIDE cause: {cause:?}");
    let e = topo::ValidationError::ChartSingularityNotAVertex {
        face: Default::default(),
        point: Singularity::North,
        lies: topo::SingularityLies::Undecided { cause },
    };
    eprintln!("SIDE: {e}");
}
