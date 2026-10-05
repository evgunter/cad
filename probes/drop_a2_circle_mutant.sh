#!/usr/bin/env bash
# The circle-only variant: the arm stops charging A₂ on a CIRCLE carrier.
sed -i 's/let (a1, noise) = (hypot(h.c1, h.s1), noise + second);/let (a1, noise) = (hypot(h.c1, h.s1), if matches!(carrier, geom::Curve3::Circle { .. }) { noise } else { noise + second });/' "$1/crates/topo/src/boolean/conic_quadric/mod.rs"
