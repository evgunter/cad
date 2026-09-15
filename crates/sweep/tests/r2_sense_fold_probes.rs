//! A reversed face through the public doors: what tier 3 says of a
//! body with one curved wall inside out — every error it raises for
//! the one defect, pinned by edge and by face.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::common::cavity::rod;
use geom_core::{Point2, Tol};
use topo::query;
use topo::{Body, EdgeKey, FaceKey, ValidationError, validate_geometric};

/// The faces of `body` whose carrier satisfies `pick`.
fn faces_where(body: &Body<f64>, pick: impl Fn(&geom::Surface<f64>) -> bool) -> Vec<FaceKey> {
    query::all_faces(body)
        .into_iter()
        .filter(|&f| {
            body.get_face(f)
                .and_then(|fd| body.get_surface(fd.surface))
                .is_some_and(&pick)
        })
        .collect()
}

/// Every edge on `face`'s boundary.
fn edges_of(body: &Body<f64>, face: FaceKey) -> BTreeSet<EdgeKey> {
    body.half_edges()
        .filter(|(_, h)| body.get_loop(h.parent_loop).map(|l| l.face) == Some(face))
        .map(|(_, h)| h.edge)
        .collect()
}

/// **A rod with one half-cylinder wall reversed answers three errors
/// for one defect.** Check 4 names the wall (`CurvedSenseInverted`,
/// the bit read directly), and the material-wedge arm reads each of
/// the two seams the wall shares with its honest twin — one cylinder,
/// osculating jets, opposed material sides — as conformal contact
/// (`LaminaWedge`): the same misnomer the reversed-band ball produces
/// and `work/props/m6-sense-gate-recorded-residuals.md` records.
/// Pinned so the day the wedge arm stops reading a reversed wall as
/// lamina, or check 4 stops naming it, this row says so.
#[test]
fn a_reversed_rod_wall_is_named_by_check_4_and_read_as_lamina_at_both_seams() {
    let r = rod(Point2::new(0.0, 0.0), 0.5, 0.0, 2.0);
    assert_eq!(
        validate_geometric(&r, Tol::witness()),
        Ok(()),
        "the honest rod"
    );
    // Two half-arc profile segments, so two half-cylinder walls on one
    // chart, sharing their two seam edges.
    let walls = faces_where(&r, |s| matches!(s, geom::Surface::Cylinder { .. }));
    assert_eq!(walls.len(), 2, "two half-cylinder walls");
    let (wall, other) = (walls[0], walls[1]);
    let seams: BTreeSet<EdgeKey> = edges_of(&r, wall)
        .intersection(&edges_of(&r, other))
        .copied()
        .collect();
    assert_eq!(seams.len(), 2, "the half walls share two seams");
    let flipped = r.flipped_face_sense_for_tests(wall).unwrap();
    let errs = validate_geometric(&flipped, Tol::witness()).unwrap_err();
    let lamina: BTreeSet<EdgeKey> = errs
        .iter()
        .filter_map(|e| match e {
            ValidationError::LaminaWedge { edge } => Some(*edge),
            _ => None,
        })
        .collect();
    assert_eq!(
        lamina, seams,
        "the wedge arm reads exactly the two wall–wall seams as lamina: {errs:?}"
    );
    let inverted: Vec<FaceKey> = errs
        .iter()
        .filter_map(|e| match e {
            ValidationError::CurvedSenseInverted { face } => Some(*face),
            _ => None,
        })
        .collect();
    assert_eq!(
        inverted,
        vec![wall],
        "check 4 names the wall once: {errs:?}"
    );
    assert_eq!(
        errs.len(),
        3,
        "three errors for the one defect, nothing else: {errs:?}"
    );
}
