//! **A cap against a wall, asked alone**: the injectivity question on
//! one plane cap and one spline wall of a lofted or swept body, at the
//! corner where the cap's boundary curve gives way to the next one.

use pncad::geom_core::{Band, Tol};
use pncad::topo::clearance::{
    BoundVerdict, Budgets, ClearanceBound, DEFAULT_MAX_CELL_DEPTH, Side, Sweep, Which,
};
use pncad::topo::{Body, FaceKey, Surface};

/// Each pair of a plane face and a spline face of `body`.
pub fn cap_wall_pairs(body: &Body<f64>) -> Vec<(FaceKey, FaceKey)> {
    let kind = |f: FaceKey| {
        let s = body.get_surface(body.get_face(f).expect("a live face").surface);
        match s.expect("a live surface") {
            Surface::Plane { .. } => Some(true),
            Surface::Nurbs(_) => Some(false),
            _ => None,
        }
    };
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    let caps = faces.iter().filter(|&&f| kind(f) == Some(true));
    caps.flat_map(|&c| {
        faces
            .iter()
            .filter(move |&&w| kind(w) == Some(false))
            .map(move |&w| (c, w))
    })
    .collect()
}

/// **The pair `(cap, wall)` holds**: the departure arm certifies it,
/// the cap's plane as the separator and the wall's side on the shared
/// edge read off its own restricted iso net, well under the depth
/// budget. `what` names the body in the messages. The receipt's splits
/// and the deepest cell are returned for the caller to pin.
pub fn assert_holds(what: &str, body: &Body<f64>, (cap, wall): (FaceKey, FaceKey)) -> (usize, u32) {
    let window = |f| {
        Side {
            body,
            faces: vec![f],
        }
        .windows(Which::First)
        .expect("a face has a window")
    };
    let sweep = Sweep {
        bound: ClearanceBound::StrictlyPositive,
        band: Band::linear(Tol::witness()).expect("the run's tolerance forms a band"),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: 8192,
        },
    };
    // No candidate is witnessed: the pair's verdict is the certificate's.
    let report = sweep.run(&window(cap), &window(wall), true, |_| {
        Err::<(), _>("no witness asked".to_owned())
    });
    println!(
        "[zeroside] {what}: {cap:?} against {wall:?}: {:?} {:?} deepest {}",
        report.verdict, report.receipt, report.widths.deepest
    );
    assert!(
        matches!(report.verdict, BoundVerdict::Holds),
        "{what}: cap {cap:?} against wall {wall:?} holds: {:?} {:?}",
        report.verdict,
        report.receipt
    );
    assert!(
        report.widths.deepest < 20,
        "{what}: cap {cap:?} against wall {wall:?} holds well under the depth budget \
         {DEFAULT_MAX_CELL_DEPTH}: deepest {} {:?}",
        report.widths.deepest,
        report.receipt
    );
    (report.receipt.splits, report.widths.deepest)
}
