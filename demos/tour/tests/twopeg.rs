//! `demo_tour::twopeg`'s tests.

/// **The rounded mate builds, and it decides a wall pair and the mating
/// plane.** [`build`] asserts the volumes and the full-engagement
/// removal; this row runs it outside the render, so the mate is pinned
/// by the suite, and checks the union records an outer-wall
/// continuation and the mating plane's contact among the coincidences
/// it decided.
mod rounded_mate {
    use demo_tour::scalar::Scalar;
    use demo_tour::twopeg::*;
    use pncad::geom_core::Tol;
    use pncad::topo::Body;
    use pncad::topo::{Cell, Operand, Relation, RowCell};

    #[test]
    fn the_rounded_mate_builds_and_decides_a_wall_continuation_and_the_mating_plane() {
        let tol = Tol::witness();
        let (p, q, mated, _) = build::<f64>(tol);
        let (pegs_p, pegs_q) = (peg_walls(&p), peg_walls(&q));
        let wall_continuation = mated.coincidences.iter().any(|row| {
            row.relation == Relation::SameOriented
                && matches!(
                    row.cells,
                    [
                        RowCell::Input { input: Operand::A, cell: Cell::Face(fa) },
                        RowCell::Input { input: Operand::B, cell: Cell::Face(fb) },
                    ] if !pegs_p.contains(&fa) && !pegs_q.contains(&fb)
                )
        });
        assert!(
            wall_continuation,
            "the mate decides an outer-wall continuation, not a peg fit: {:?}",
            mated.coincidences
        );
        let (top_p, bottom_q) = (
            plane_face(&p, PLATE.2, true),
            plane_face(&q, PLATE.2, false),
        );
        let mating = mated.coincidences.iter().any(|row| {
            row.relation == Relation::SameOpposite
                && matches!(
                    row.cells,
                    [
                        RowCell::Input { input: Operand::A, cell: Cell::Face(fa) },
                        RowCell::Input { input: Operand::B, cell: Cell::Face(fb) },
                    ] if fa == top_p && fb == bottom_q
                )
        });
        assert!(
            mating,
            "the mate decides P's top resting on Q's bottom: {:?}",
            mated.coincidences
        );
    }

    /// The one planar face of `body` at height `z` whose outward normal
    /// points up (`up`) or down — a POSITIONAL pick, by stored plane
    /// parameters, and it stays one.
    ///
    /// Same finding as [`cylinders`]: nothing on the plain body API says
    /// "the mating face" — a document would say it with a `GeoSelect` — so
    /// the scene says it in coordinates.
    fn plane_face<S: Scalar>(body: &Body<S>, z: f64, up: bool) -> pncad::topo::FaceKey {
        let hits: Vec<_> = body
            .faces()
            .filter(|(_, f)| match body.get_surface(f.surface) {
                Some(pncad::geom::Surface::Plane { origin, normal, .. }) => {
                    (origin.z.f() - z).abs() < 1e-12 && (normal.z.f() > 0.5) == up
                }
                _ => false,
            })
            .map(|(k, _)| k)
            .collect();
        let [f] = hits[..] else {
            panic!("expected exactly one z = {z} face (up = {up}), got {hits:?}");
        };
        f
    }
}
