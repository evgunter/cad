#!/usr/bin/env python3
"""Mutants for PR #3984 at 8abb6e7931: each reverts one fix; run the rows; restore."""
import subprocess, sys, os
R='/home/user/cad/'
MUT = {
 'M1_planar_unlaned_as_line': ('crates/topo/src/boolean/reduce.rs',
   """                    PlaneCrossingLane::Unlaned => {
                        return Err(BooleanError::CrossingCarrierUnsupported {
                            operand: x_is,
                            edge: edge_key,
                            face,
                        });
                    }
                    PlaneCrossingLane::Conic(meet) => Some(meet),""",
   """                    PlaneCrossingLane::Unlaned => None,
                    PlaneCrossingLane::Conic(meet) => Some(meet),"""),
 'M2_curved_arm_frontier': ('crates/topo/src/boolean/reduce.rs',
   """        (geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_), _) => {
            return Err(BooleanError::CrossingCarrierUnsupported {
                operand: x_is,
                edge: edge_key,
                face,
            });
        }""",
   """        (geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_), _) => {
            return Err(frontier());
        }"""),
 'M3_lane_spline_is_line': ('crates/topo/src/splitting/classify.rs',
   """        geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
            return PlaneCrossingLane::Unlaned;
        }""",
   """        geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
            return PlaneCrossingLane::Line;
        }"""),
 'M4_circle_only_unlaned_clear': ('crates/topo/src/boolean/reduce.rs',
   """                        PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Err(_)))
                        | PlaneCrossingLane::Unlaned => false,""",
   """                        PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Err(_))) => false,
                        PlaneCrossingLane::Unlaned => true,"""),
 'M5_split_unlaned_skips_clearance': ('crates/topo/src/splitting/classify.rs',
   """                if edge_clears(body, edge_key, plane, band) {
                    continue;
                }
                return Err(SplitReduceError::CurvedEdgeUnsupported { edge: edge_key });""",
   """                continue;"""),
 'M6_touch_guard_dropped': ('crates/topo/src/boolean/reduce.rs',
   "let touch_at_end = if meet.is_none() || covers.is_empty() {",
   "let touch_at_end = if covers.is_empty() {"),
}
names = sys.argv[1:] or list(MUT)
filt = os.environ.get('FILTER', 'all()')
for n in names:
    f, a, b = MUT[n]
    p = R + f; s = open(p).read()
    assert s.count(a) == 1, n
    open(p, 'w').write(s.replace(a, b))
    try:
        r = subprocess.run(['cargo', 'nextest', 'run', '-p', os.environ.get('PKG','topo'), '--no-fail-fast', '-E', filt],
                           cwd=R, capture_output=True, text=True, env={**os.environ, 'CARGO_INCREMENTAL': '0'})
        out = r.stdout + r.stderr
        fails = [l.strip() for l in out.splitlines() if l.strip().startswith(('FAIL [', 'error['))]
        summ = [l for l in out.splitlines() if 'Summary' in l or 'tests run' in l]
        print(f'== {n}: rc={r.returncode} {summ[-1:] }')
        for l in sorted(set(fails))[:15]: print('   ', l)
    finally:
        open(p, 'w').write(s)
