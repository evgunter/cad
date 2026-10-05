#!/usr/bin/env bash
# Claim 2's mutant: the first-harmonic arm stops charging the dropped A₂.
sed -i 's/let (a1, noise) = (hypot(h.c1, h.s1), noise + second);/let (a1, noise) = (hypot(h.c1, h.s1), noise);/' "$1/crates/topo/src/boolean/conic_quadric/mod.rs"
