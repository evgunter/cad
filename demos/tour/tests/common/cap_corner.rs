//! **A cap against a wall, asked alone**: the injectivity question on
//! one plane cap and one spline wall of a lofted or swept body, the
//! pair `work/clear/injectivity-departure-refuses-at-depth-on-loft-cap-corners-and-twisted-seams.md`
//! pins at the corner where the cap's boundary curve gives way to the
//! next one.

use pncad::geom_core::{Band, Tol};
use pncad::topo::clearance::{
    BoundVerdict, Budgets, CertificateArm, ClearanceBound, DEFAULT_MAX_CELL_DEPTH, EngineRefusal,
    SharedCell, Side, Sweep, Which,
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

/// **The pair `(cap, wall)` refuses `Uncertified { Departure }` across
/// an edge the two share, at the depth budget**: the corner's cells
/// split to `DEFAULT_MAX_CELL_DEPTH` and stop there, rather than
/// refusing at the root. `what` names the body in the messages.
pub fn assert_refuses_at_depth(what: &str, body: &Body<f64>, (cap, wall): (FaceKey, FaceKey)) {
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
    let refused = matches!(
        report.verdict,
        BoundVerdict::Refused(EngineRefusal::Uncertified {
            a,
            b,
            shared: SharedCell::Edge(_),
            arm: CertificateArm::Departure,
        }) if (a, b) == (cap, wall)
    );
    assert!(
        refused,
        "{what}: cap {cap:?} against wall {wall:?} refuses Uncertified Departure across their \
         shared edge (the row's cap-corner locus); a fix flips this row: {:?} {:?}",
        report.verdict, report.receipt
    );
    assert_eq!(
        report.widths.deepest, DEFAULT_MAX_CELL_DEPTH,
        "{what}: cap {cap:?} against wall {wall:?} refuses at the depth budget, not at the root: \
         {:?}",
        report.receipt
    );
}
