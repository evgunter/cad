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

## 2026-11-19 — another seed (ONEROUND, `oneround/rounded-room` lane)

`certified_torus_answers_hold_against_the_true_distance` at the default
ε drew `CAD_FUZZ_SEED=0xb0caff8e574eeaa1` (`CAD_FUZZ_EFFORT=1`) and
panicked in `circle_roots::door_answer` on a `ring_torus_convention`
refusal in its 1e-6 pass (margin `9.674e-6` against the band
`1e-6 / 1e-5`). Replayed on `origin/main` at `b80352cd` it fails the
same way. Two of fourteen runs of the row here drew a failing seed.

## Evidence (germ/co-surface-table, 2026-11-23)

`geom-brep`'s `curve_face::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance` is red on `origin/main` on seed `0x3a0790b1c3cbfab2`, the panic at `circle_roots.rs:285` (re-run on a main worktree, 2026-11-23, from PR 7431's review).

## Evidence (germ/spiric-near-miss-off-q, 2026-11-23)

Two more seeds panic the same way (`ring_torus_convention` undecided, margins 6.3e-6 and 4.6e-6 in the band `(1e-6, 1e-5)`): `0x8f3af7fd0193fe1c` in a run at `CAD_TOLERANCE_EPS=1e-6` and `0x2721258bd8b3b6b0` in a run at `1e-12`.

- 2026-11-23 (GERM, PR 7553's review): another seed draws the same failure. At `CAD_FUZZ_SEED=0x12ef5c0007e519b4`, `ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance` fails with `ring_torus_convention` Undecided, and it fails the same way on the merge base, outside PR 7553's diff.
- 2026-11-23 (GERM, PR 7553's battery): a third seed. At `CAD_FUZZ_SEED=0xf16f58a1203054fa`, ε 1e-12, the same panic: `ring_torus_convention` Undecided, margin 5.0e-6 at band zero 1e-6.

<<<<<<< HEAD
- 2026-11-24 (GERM orchestrator) — another seed: `0x9deec1ee782114a3` reds the slow-set `ellipse_torus` fuzz identically on main (`9c79b8502`) and on PR 7577's head. A tiny torus's ring convention reads in band before any arc is read (the `Torus(..)` refusal in `door_answer`). Found by GERM's U4a lane after it restored the partial-arc fuzz draws.
=======
## 2026-11-25 — another seed (INSIDE, `inside/face-door-spline-arm`)

`CAD_FUZZ_SEED=0x24d54ed8f5337e1f` reds
`certified_torus_answers_hold_against_the_true_distance` the same way
(`door_answer` panics on `TorusRefusal { convention: Ring, verdict:
Undecided(… predicate: "ring_torus_convention") }`, band `1e-6`), on a
branch that does not touch the ellipse × torus arm.
>>>>>>> origin/main
