---
id: an-ellipse-torus-fuzz-seed-draws-a-torus-whose-ring-convention-is-in-band
kind: issue
title: The ellipse x torus fuzz row panics on a seed that draws a torus whose ring convention is in band: the door's typed refusal is read as outside its decision
status: open
opened: 2026-11-11
priority: P2
cost: E
---


Filed by the SECTARMS plane-partial lane (branch
`sectarms/plane-partial-closed-form`, 2026-11-11). It turned up in that
branch's geom-brep slow set and reproduces on main.

## What

`curve_face::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance`
(`crates/geom-brep/src/curve_face/ellipse_torus.rs`) is a counterexample
search with a fresh seed each run. Seed `0x3ea80f26c3a065f6` panics at ε
1e-6, at `curve_face::circle_roots::door_answer`
(`crates/geom-brep/src/curve_face/circle_roots.rs`):

```
the door refused outside its decision: Torus(TorusRefusal { convention: Ring,
  verdict: Undecided(Indeterminate { margin: 8.878612850789409e-6, band (1e-6, 1e-5),
  predicate: "ring_torus_convention" … }) })
```

The pose generator drew a torus whose `R − r` lies in the band. The door
refuses that typed, on the torus's ring convention, before any root is
read. `door_answer` admits only `DoorRefusal::Escalated` as a refusal and
panics on any other.

## Reproduce

On main `9b4f0a7f47`:

```
CAD_FUZZ_SEED=0x3ea80f26c3a065f6 cargo nextest run -p geom-brep --all-features --lib \
  -E 'test(=curve_face::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance)'
```

It also failed once in a slow-set run on an unseeded draw, so roughly
one draw in a few dozen hits it.

## What the change could be

Either the pose generator keeps `R − r` past the band at the ε it runs
at, or the row reads a convention refusal in band as the escalation it
is. Which one depends on whether the door's typed refusal is meant to
be distinct from `Escalated`; that is SPLICE's call. Once it is fixed,
pin the seed as a deterministic row beside the fix.

A second seed, from the ONEDOOR wedge-fold lane's default-ε run
(2026-11-13), reproduces on `origin/main` at `31cba239`: seed
`0x9fd7a6243bf09f33` (`CAD_FUZZ_SEED=0x9fd7a6243bf09f33`), the same
panic at `circle_roots::door_answer` with `ring_torus_convention`'s
margin `9.631336879257417e-6` in the band (1e-6, 1e-5).
A third, from the same lane's re-run (2026-11-14): seed
`0xb27e31258ec89b00` reproduces on `origin/main` at `8eb399d8`, margin
`7.579386505001449e-6`.
A fourth, from BOXFINISH's edge-turns lane (2026-11-14), on a branch
whose diff does not touch `curve_face`: seed `0xda3ffbd3eaed595f`
(`CAD_FUZZ_SEED=0xda3ffbd3eaed595f`), margin `9.914e-6`.

Three more, from BOXFINISH's slow-set measurement (`boxfinish/chart-ends-slow-rows`, 2026-11-15), on trees whose diff does not touch `curve_face`, each with the same `ring_torus_convention` `Undecided` payload at ε 1e-6. The first two were replayed red on `origin/main` `dbb21b4a`:
- seed `0x31f1331db90e9f43`, from the slow set at `dbb21b4a`;
- seed `0xa00da6f2a06eefcb`, from the slow set at `bdb498df`, before PR 6959;
- seed `0x18c721b4cca50dfa`, from the slow set on `boxfinish/chart-ends-slow-rows` (main `dbb21b4a` merged forward, with test-only edits).

## Another seed (2026-11-18, ONEHOME fork 275 step 3b's workspace suite)

`CAD_FUZZ_SEED=0xe071b34d3cca0e69` panics under the suite's default
tolerance, the payload at band zero `1e-6`: `TorusRefusal { convention: Ring,
verdict: Undecided(.. ring_torus_convention ..) }` read as outside the
door's decision (`crates/geom-brep/src/curve_face/circle_roots.rs:419`).
It reds identically on the merge of PR 7048 (`0c2b761e91`), so it is this
item's, not the step's.
