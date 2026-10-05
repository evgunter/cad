#!/usr/bin/env python3
"""Claim 5's mutant: put the arm switch back to the TILT form
`speed_hi·|n̂ × â|` against a wall (sphere keeps A₂). Apply to a head
tree: `tilt_mutant.py <tree>`."""
import sys
p = f"{sys.argv[1]}/crates/topo/src/boolean/conic_quadric/mod.rs"
s = open(p).read()
old = "    if let Ok(Sign::Zero) = decide(SECOND_HARMONIC, Margin::of(second), band) {"
new = '''    let switch = match (carrier, surface) {
        (
            &geom::Curve3::Circle { axis: n, .. } | &geom::Curve3::Ellipse { axis: n, .. },
            &geom::Surface::Cylinder { axis: a, .. },
        ) => conic.speed_hi() * n.cross(a).norm(),
        _ => second,
    };
    if let Ok(Sign::Zero) = decide(SECOND_HARMONIC, Margin::of(switch), band) {'''
assert s.count(old) == 1
s = s.replace(old, new)
open(p, "w").write(s)
