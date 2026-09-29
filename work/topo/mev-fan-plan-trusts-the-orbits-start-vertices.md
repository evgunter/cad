---
id: mev-fan-plan-trusts-the-orbits-start-vertices
kind: issue
title: mev_fan_plan trusts the vertex orbit's start vertices: a torn orbit is carried through a fan mev instead of refused typed, the gap kev_plan closed in PR 3161
status: open
opened: 2026-09-29
refs: [kevs-fan-merge-needs-a-re-describing-kill-door, 3161]
priority: P2
cost: E
pr: 3472
branch: topo/mev-fan-orbit-proof
---

## What

Found by PR 3161's fix pass while closing that PR's MAJOR, and
disclosed in its body. It is filed here because a PR-body disclosure
is not a slate row.

`Body::kev_plan` (`crates/topo/src/euler_kill.rs`) now proves that
every half of the merged fan starts at the dying vertex, and refuses
`OrbitBroken` otherwise. Without that check, two `next` tears put the
killed half into the dying vertex's orbit, and `kev_describing`'s write
reached an `unreachable!`. `mev_fan_plan` (`crates/topo/src/euler.rs`)
walks the same `vertex_orbit` (`next(mate(x))`) and has the same gap:
it trusts the orbit's start vertices. It cannot panic, because no
mutation-phase `unreachable!` depends on it. But on a torn body it
carries the corruption through a fan `mev` instead of refusing it
typed, which D1's plan-phase contract requires.

## The shape to give

The same proof in `mev_fan_plan`: every half of the moved run starts
at the vertex being split. Refuse `OrbitBroken` otherwise. Pin it with
a torn-orbit row on the `review_d18` fixture set, as PR 3161 did for
`kev`.

## Brief (TOPO, 2026-09-29): review tier SINGLE (full)

A plan-phase correctness check on corrupt input that one full review
can falsify.

1. **The answer to give.** `mev_fan_plan` proves, before any
   mutation, what `mev_fan_execute` and the re-basing gate assume
   about the run: every half-edge of the moved run starts at the
   vertex being split, and the run is the contiguous `[he1 .. he2)`
   slice of that vertex's orbit. A torn orbit refuses typed
   (`OrbitBroken`, or the variant the code's other orbit refusals
   use) in every door that reaches the plan: `mev`, `mev_line`,
   `mev_null`, and the kernel run sites through `mev_null`. Say
   whether `null.rs`'s fan path shares `mev_fan_plan` (if not, give
   it the same proof), and whether `mev_fan_site` reads the orbit
   before or after the proof (it must be after).
2. **Measure first, on the merge base.** Build the smallest torn body
   where a fan `mev` walks an orbit through a half that does not
   start at the split vertex, and show what the base does: carries
   the corruption through `Ok`, panics, or refuses. Then run a seeded
   tear search in the style of the `kev` PR's over `review_d18`'s
   fixtures, `NextForeign` tears, and `mev`/`mev_null` fan calls.
   Report the counts and pin the first counterexample as a
   deterministic row. No varying-seed test.
3. **Rows:** red-first, the torn orbit refusing typed with the body
   untouched (`deep_snapshot`), through `mev` and `mev_null`; the
   pinned counterexample; a control on a valid body, unchanged bit
   for bit.
4. **The doc.** `mev`'s precondition-order paragraph and `mev_null`'s
   docs name the new refusal at its slot.
5. **Receipt.** Every plan that walks `vertex_orbit`, or the
   `next(mate(x))` idiom, and trusts start vertices: proven,
   trusting-and-panicking, or trusting-and-carrying. File a row for
   any trusting site not fixed here.
