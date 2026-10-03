#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stderr)]
use crate::common;
use geom_core::Tol;
use topo::BooleanResult;

#[test]
fn r2_inside_out_wedge_at_dual_main() {
    use geom_core::Dual64;
    let tol = Tol::witness();
    let at = |deg: f64| (deg.to_radians().cos(), deg.to_radians().sin());
    for (what, prof) in [
        ("clockwise", [(0.0, 0.0), at(190.0), at(80.0)]),
        ("counterclockwise", [(0.0, 0.0), at(80.0), at(190.0)]),
    ] {
        let b = common::brick::<Dual64>((0.0, 1.0), (0.0, 1.0), (0.5, 1.5), tol);
        let w = common::prism_z::<Dual64>(&prof, 0.5, 1.0, tol).body;
        let decls = common::flush_declarations(&b, &w, tol);
        eprintln!("{what}:");
        for (name, r) in [
            ("brick − wedge", topo::subtract_with(&b, &w, &decls, tol)),
            ("brick ∩ wedge", topo::intersect_with(&b, &w, &decls, tol)),
        ] {
            match r {
                Ok(BooleanResult::Body(r)) => eprintln!(
                    "  {name}: answers, volume {:?}",
                    topo::mass_properties_structural(&r.body, tol).map(|m| m.volume.value)
                ),
                Ok(BooleanResult::Empty) => eprintln!("  {name}: empty"),
                Err(e) => eprintln!("  {name}: refused {e:?}"),
            }
        }
    }
}
