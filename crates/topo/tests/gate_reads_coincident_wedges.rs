//! **The Boolean's result gate reads the material of coincident edges.**
//!
//! Where two or more of a result's edges lie on one line, each bounds
//! its own wedge of material (D1), so no two wedges overlap; the gate
//! reads every such line (`census::line_wedges`) and refuses an overlap
//! typed, at every op. The rows that refuse are pinned beside their
//! fixtures; these are the ones that must keep building.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished, prism_z};
use geom_core::Tol;
use topo::{AtRestBody, BooleanResult, union};

fn t() -> Tol {
    Tol::witness()
}

/// **An edge leaving the line at one of its ends is not on the line.**
/// A box `[0, 2] × [0, 1] × [0, 1]` and a prism over `z ∈ [1, 2]` kiss
/// along the box's top back edge, `y = z = 1`. The prism's bottom face
/// leaves one end of that line along a slanted edge, at 45° in plan,
/// whose projection on the line runs half its length; the mirrored
/// prism leaves the other end. The two edges on the line bound 90° each
/// on opposite sides, and the union builds both ways round at
/// `2 + 1.5`.
#[test]
fn a_slanted_edge_leaving_a_kiss_line_is_not_read_on_it() {
    let block = finished(
        "block",
        brick::<f64>((0.0, 2.0), (0.0, 1.0), (0.0, 1.0), t()),
        t(),
    );
    for (what, profile) in [
        (
            "leaving the near end",
            [(0.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)],
        ),
        (
            "leaving the far end",
            [(0.0, 1.0), (2.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
        ),
    ] {
        let prism: AtRestBody<f64> = finished("prism", prism_z(&profile, 1.0, 2.0, t()).body, t());
        for (order, a, b) in [
            ("block ∪ prism", &block, &prism),
            ("prism ∪ block", &prism, &block),
        ] {
            let BooleanResult::Body(r) =
                union(a, b, t()).unwrap_or_else(|e| panic!("{what}, {order}: {e:?}"))
            else {
                panic!("{what}, {order}: empty");
            };
            let v = topo::test_support::pinned_volume("the measured body", &r.body, t()).volume;
            assert!((v - 3.5).abs() < 1e-9, "{what}, {order}: volume {v}");
        }
    }
}
