//! Reviewer probes for PR 5699, head-only APIs.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stderr)]

use geom_core::Tol;
use crate::review_5699_probes::bodies;

#[test]
fn print_regions_carry_and_one_point_agreement() {
    let tol = Tol::witness();
    for (label, body) in bodies() {
        let regions = topo::test_support::region_carries_of(&body);
        let shape: Vec<Vec<(usize, Option<String>)>> = regions
            .iter()
            .map(|(_, rs)| rs.iter().map(|(f, c)| (f.len(), c.map(|(p, iv)| format!("{p:?} {iv:?}")))).collect())
            .collect();
        let at = topo::test_support::finished("m", body.clone(), tol);
        let carried = topo::mass_properties(&at, tol);
        let one = topo::test_support::mass_properties_one_point(&body, tol);
        let doors = topo::test_support::every_door(&body, tol);
        eprintln!("PROBEH {label}: regions {shape:?}");
        match (carried, one) {
            (Ok(c), Ok(o)) => eprintln!(
                "PROBEH {label}: carried {:.17e}±{:.3e} one {:.17e}±{:.3e} |d|={:.3e} agree {}",
                c.volume, c.volume_pad, o.volume, o.volume_pad, (c.volume - o.volume).abs(),
                (c.volume - o.volume).abs() <= c.volume_pad + o.volume_pad
            ),
            (c, o) => eprintln!("PROBEH {label}: carried {:?} one {:?}", c.err(), o.err()),
        }
        eprintln!("PROBEH {label}: doors {doors:?}");
    }
}
