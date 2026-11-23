"""Make the junction's reads selectable at run time (the PR 7432 fix pass).

`python3 probes/heldrecords-elbow/mutate.py crates/topo/src/boolean/mod.rs`
rewrites `junction_of` so that `ELBOW_MUT` picks one mutant; unset, the
code is the head's. Build once, then run the door rows under each value
(`README.md`, the mutant table). Never committed applied.

The mutants, the reviewers' names in brackets:
  T_GLUE        a corner all round glues [R1 T_GLUE, R2 transverse_glues]
  SAME_GLUE     the routing's seam glues with the faces leaving on one side [R1 SAME_GLUE]
  NO_ROUTE      no routing or departures read, only the convex edges [R1 NO_ROUTE]
  SEAM_UNROUTED the routing's seam never glues a junction [R1 SEAM_UNROUTED, R2 seam_never]
  CUSP_GLUES    the routing's cusp glues [R2 cusp_glues]
  ANY_DEPARTURE the routing's seam glues whatever the departures read [R2 seam_any_departure]
  NO_CONVEX     the convex-edge certificate skipped
"""
import sys

path = sys.argv[1]
s = open(path).read()
old = """    (matches!(routing, Ok(crate::rim_wedge::RimRouting::Seam))
        && matches!(departure(), Ok(crate::rim_wedge::Departure::Opposite))
        && crate::rim_wedge::rim_edges_convex(a, fa, rim, band)
        && crate::rim_wedge::rim_edges_convex(b, fb, rim, band))"""
new = """    let m = std::env::var("ELBOW_MUT").unwrap_or_default();
    let m = m.as_str();
    use crate::rim_wedge::{Departure as D, RimRouting as R};
    let routed = match (&routing, m) {
        (_, "NO_ROUTE") => true,
        (Ok(R::Seam), "SEAM_UNROUTED") => false,
        (Ok(R::Seam), _) => true,
        (Ok(R::Transverse), "T_GLUE") => true,
        (Ok(R::Cusp(_)), "CUSP_GLUES") => true,
        _ => false,
    };
    let departed = || match (departure(), m) {
        (_, "NO_ROUTE" | "ANY_DEPARTURE" | "T_GLUE" | "CUSP_GLUES") => true,
        (Ok(D::Opposite), _) => true,
        (Ok(D::Same), "SAME_GLUE") => true,
        _ => false,
    };
    let convex = || {
        m == "NO_CONVEX"
            || (crate::rim_wedge::rim_edges_convex(a, fa, rim, band)
                && crate::rim_wedge::rim_edges_convex(b, fb, rim, band))
    };
    (routed && departed() && convex())"""
assert s.count(old) == 1, "junction_of moved: re-derive the patch"
open(path, "w").write(s.replace(old, new))
