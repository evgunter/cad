---
id: an-interval-census-of-the-reshaped-rod-finds-what-f64-does-not-at-1e-12
kind: issue
title: at 1e-12 the census door finds a defect on the reshaped rod at Interval that it does not find at f64
status: open
priority: P1
opened: 2026-10-19
---


Found by the PATHS chord-director lane (PR #5370) at its merge base and confirmed with that PR's profile changes reverted, so it is red on main independently of PATHS.

`crates/editor-core/tests/m10_di_dual_corpus.rs` `assemble_census_verdicts_match_f64_at_interval` fails at `CAD_TOLERANCE_EPS=1e-12` on the corpus entry `reshaped_rod`: the census door returns no finding at `f64` and a finding at `Interval`. The row asserts that the two scalars' verdicts agree, so one of the two readings is wrong at the tightest ε. Either the Interval reading escalates where the reals have no defect, or the f64 reading misses one.

To do: reproduce (`CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p editor-core --test m10_di_dual_corpus`), name the finding, and decide which scalar is right. The nightly's 1e-12 row is red on it, so P1 (a red main row). Territory for that test file is scalar/tcost/tint; filed on CENSUS as the door whose verdict differs. Re-home it if the cause sits in the scalar lane.

## 2026-10-20 — the commit that turned it red (CONTACTHOLD's nightly-reds lane)

The 2026-10-20 nightly (run 39375288176, head `37d8dfdecf`) carries it at 1e-12; the 2026-10-19 nightly (head `07a57312bf`) did not. The finding the Interval lane prints is two `NotCertifiable` contact refusals (`topo::validate`'s `classify_contact_lane`):

```
reshaped_rod: the census door disagreed across certifying scalars (f64 None vs Interval Some("2 finding(s) against this assembly:\n  no declaration: an edge could not be checked: the kernel cannot yet check a contact between faces of these kinds. There is no way through yet\n  no declaration: an edge could not be checked: ..."))
```

A first-parent bisect of main over `07a57312bf..37d8dfdecf`, building `editor-core`'s `all` binary and running this row at 1e-12, names `8b15425279` — the merge of PATHS's PR 5068 (`paths/sketch-plane-is-its-frame`, the f64 lane becomes pass 1 and every lane elaborates Guided). Its first parent passes. That PR also edited `m10_di_dual_corpus.rs`. So the Interval lane's profile now elaborates to faces whose contact `topo` cannot certify, or the f64 lane no longer reaches the contact check; PATHS is the first place to look. Still red on `cd4243bcfd`.

**Evidence, the finding named (INTENT stage 3 D, 2026-10-20).** On D's branch at `ccf433d0e4` the row reaches `reshaped_rod` at ε = 1e-12 (it stopped first at `die_composed` before D's exact-turn lane). The `Interval` census returns two `CensusUnsupported` findings on two edges of the rod's product, each `ContactLane(NotCertifiable { what: "the meeting reading of two curved edges could not settle a count" })`: pass 5′'s `geom_brep::curve_pair::meet` refusing `MeetFault::Unsettled` (not `Exhausted`) in `crates/topo/src/census/curved.rs` `interior_meeting`. The `f64` census finds nothing. The rod is placed by the identity bundle (no map), so no placement's turn is involved. Every other corpus document agrees at ε = 1e-12 on that head.

## 2026-10-24 (INTENT nightly triage)

Still red at 1e-12 only (nightly run 40066431545, main `3dfb96b91c`):
`reshaped_rod: the census door disagreed across certifying scalars (f64
None vs Interval Some("2 finding(s) ... the kernel cannot yet check a
contact between faces of these kinds ...`.
