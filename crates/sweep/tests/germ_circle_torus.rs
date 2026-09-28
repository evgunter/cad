//! GERM: the circle × torus root lane at the boolean's crossing layer.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;

use crate::mate7a_r1_probes::{arch, stem, weld_declarations};

#[test]
fn measure_lily() {
    let (s, a) = (stem(), arch());
    let (decls, _) = weld_declarations(&s, &a);
    let r = topo::union_with(&s, &a, &decls, Tol::witness());
    match &r {
        Ok(b) => println!("OK {:?}", b.body().is_some()),
        Err(e) => {
            println!("ERR {e:?}");
            if let topo::BooleanError::CurvedPierceUnsupported { operand, face, edge, .. } = e {
                let (x, y) = if *operand == topo::Operand::A { (&s, &a) } else { (&a, &s) };
                let ed = x.get_edge(*edge).unwrap();
                let c = x.get_curve_geom(ed.curve).unwrap().certified().unwrap();
                println!("edge carrier {:?} params {:?}", c.carrier(), c.params());
                println!("face surface {:?}", y.get_surface(y.get_face(*face).unwrap().surface));
            }
        }
    }
}
