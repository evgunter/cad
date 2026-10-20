---
id: census-link-fuzz-escalates-in-its-prism-fixture-at-1e-6
kind: issue
title: census link_fuzz panics at 1e-6 on a seed whose random prism reads one edge's dihedral in band inside describe_as_intersections
status: open
opened: 2026-10-20
priority: P3
cost: E
---


Found by the cleave slit-line lane's slow-set runs (PR 5479). It does not
touch that change: the test makes no Boolean call.

## What

`census::link_fuzz::a_pinch_link_agrees_with_the_prisms_own_inequalities`
(`crates/topo/src/census/link_fuzz.rs`) panicked at ε = 1e-6 on
seed `0x32572264a5332df9`:

    CAD_TOLERANCE_EPS=1e-6 CAD_FUZZ_SEED=0x32572264a5332df9 \
      cargo nextest run -p topo --ignore-default-filter \
      -E 'test(=census::link_fuzz::a_pinch_link_agrees_with_the_prisms_own_inequalities)'

The panic is the fixture's
`geom_brep::classify_dihedral(..).unwrap()` in
`describe_as_intersections` (`crates/topo/src/test_support_fixtures.rs`):
`LeverEscalation { rung: Reading, margin 9.2e-6 }`, in the band
(zero 1e-6, escalate 1e-5). One of the random prism's edges has a
dihedral the run cannot tell from flat. The same test passed at the
default ε and at 1e-12 in the same runs.

## Fix

The fuzzer draws a prism whose edge reads in band, which no fixture can
describe. Either skip that draw loudly (`loud_skip_marker!`) and count it,
or draw the prism away from flat by more than the band. Then pin this
seed as a deterministic row.
