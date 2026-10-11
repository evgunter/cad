//! **A loft's placements and skinned walls enter `T` as computed
//! values.** Both are composed in `f64` (the placements by the caller
//! or `sweep_places`, the walls by the skin), so at `Sym` each lifted
//! coordinate is an unknown (`Real::from_computed`), never a constant
//! that re-values to where the `f64` build put it, and a seam two
//! walls share is lifted once for both.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::sym::revalue::{record, revalue};
use geom_core::sym::with_session;
use geom_core::{Affine3, ParamSymbol, Point2, Sym, SymBudget, Tol, Vec3};
use profile::test_support::bulge_loop;
use sweep::loft_body;

type S = Sym<f64>;

#[test]
fn a_lofts_placements_and_walls_are_computed_values_at_sym() {
    let square = || {
        vec![bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ])]
    };
    let places = [
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.25, 0.0, 1.5)),
    ];
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let none = |_: ParamSymbol| None::<f64>;
    let ((stacking, wall, top, seam_shared), _) = with_session(budget, || {
        let (lofted, decisions) =
            record(|| loft_body::<S>(&[square(), square()], &places, 1, Tol::witness()));
        let lofted = lofted.expect("a square prism lofts");
        let stacking: Vec<_> = decisions
            .iter()
            .filter(|d| d.predicate == "loft_stacking")
            .map(|d| revalue(d.node, &none))
            .collect();
        let face_surface = |f| {
            let face = lofted.body.get_face(f).expect("a built face");
            lofted
                .body
                .get_surface(face.surface)
                .expect("a described face")
                .clone()
        };
        let wall = match face_surface(lofted.side_faces[0][0]) {
            Surface::Nurbs(n) => revalue(n.control()[0].x.node(), &none),
            other => panic!("a lofted wall is a NURBS surface, got {other:?}"),
        };
        let net = |f| match face_surface(f) {
            Surface::Nurbs(n) => n,
            other => panic!("a lofted wall is a NURBS surface, got {other:?}"),
        };
        let (w0, w1) = (net(lofted.side_faces[0][0]), net(lofted.side_faces[0][1]));
        let ((nu, nv), (_, nv1)) = (w0.control_counts(), w1.control_counts());
        assert_eq!(nv, nv1, "adjacent walls carry one v count");
        let seam_shared = (0..nv).all(|iv| {
            let (a, b) = (w0.control()[(nu - 1) * nv + iv], w1.control()[iv]);
            [a.x.node(), a.y.node(), a.z.node()] == [b.x.node(), b.y.node(), b.z.node()]
        });
        let top = match face_surface(lofted.top) {
            Surface::Plane { origin, .. } => revalue(origin.z.node(), &none),
            other => panic!("a loft cap is a plane, got {other:?}"),
        };
        (stacking, wall, top, seam_shared)
    });
    assert_eq!(
        (stacking, wall, top, seam_shared),
        (vec![None, None], None, None, true),
        "(the stacking fold's two margins, a skinned wall's control point, the top cap's \
         plane) are unknowns at Sym, not constants; and the seam two walls share is one set \
         of unknowns on both sides"
    );
}
