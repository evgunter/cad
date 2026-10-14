//! **A value the plane × NURBS lane computes in `f64` enters the
//! caller's scalar as a computed value** (`Real::from_computed`), never
//! as a constant: at `Sym<f64>` each is its own unknown, so no
//! symbolic `Zero` is proved over it and it re-values to nothing
//! (`geom_core::sym::revalue`). Through `from_f64` each would be a
//! `Lit`, and re-value to the bits it was built with.
//!
//! One fixture, the M7-8 quarter-cylinder ruling lifted to `Sym<f64>`,
//! driven through `plane_nurbs_limbs`; each row reads the decisions or
//! the field one site feeds.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::fixture::segment;
use crate::shared::fixture::{quarter_cylinder_wall, transverse_plane};
use crate::shared::tol::band;
use geom_brep::plane_nurbs_limbs;
use geom_brep::ssi::SsiTube;
use geom_core::sym::revalue::{record, revalue};
use geom_core::sym::with_session;
use geom_core::{Point3, Real, Sym, SymBudget};

type S = Sym<f64>;

#[test]
fn the_plane_nurbs_lanes_computed_values_are_unknowns() {
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    let none = |_| None;
    let ((by_predicate, fields), _) = with_session(budget, || {
        let lift = S::from_f64;
        let wall = quarter_cylinder_wall().map_scalar(lift);
        let plane = transverse_plane().map_scalar(lift);
        let carrier =
            segment(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0)).map_scalar(lift);
        let (limbs, decisions) = record(|| {
            plane_nurbs_limbs::<S>(
                &geom::Curve3::Nurbs(std::sync::Arc::new(carrier.clone())),
                carrier.knot_values().domain(),
                &plane,
                &wall,
                S::one(),
                band(),
            )
            .expect("the true locus certifies at Sym")
        });
        let revalued = |predicate: &str| -> Vec<Option<f64>> {
            decisions
                .iter()
                .filter(|d| d.predicate == predicate)
                .map(|d| revalue(d.node, &none))
                .collect()
        };
        let by_predicate = [
            "plane_nurbs_transversality",
            "ssi_hull_sup",
            "ssi_hull_sup_chart",
            "ssi_tube_transversality",
        ]
        .map(|p| (p, revalued(p)));
        let SsiTube::Chart { rung, pad_u, pad_v } = limbs.tube else {
            panic!(
                "plane × NURBS proves its tube on the chart: {:?}",
                limbs.tube
            )
        };
        let fields = [
            ("tube.rung", revalue(rung.node(), &none)),
            ("tube.pad_u", revalue(pad_u.node(), &none)),
            ("tube.pad_v", revalue(pad_v.node(), &none)),
        ];
        (by_predicate, fields)
    });
    // Each predicate's margin reads a site: the wall's jet at the
    // measured foot (`edge_nurbs`'s `foot.x` and `foot.y` together; the
    // jet reads both, so `measured_foot`'s own row tells them apart), the analytic
    // operand's hull bound (`hull.sup`), the NURBS operand's composite
    // bound (`sup`) and the tube's clearance (`clearance`).
    for (predicate, values) in by_predicate {
        assert!(
            !values.is_empty(),
            "{predicate}: the fixture decides it at least once"
        );
        assert!(
            values.iter().all(Option::is_none),
            "{predicate}: its margin reads a computed value, so it re-values to nothing: {values:?}"
        );
    }
    // The tube's rung and pads are the ladder's (`rung.tube.map`).
    for (field, value) in fields {
        assert_eq!(value, None, "{field}: a computed value, not a constant");
    }
}
