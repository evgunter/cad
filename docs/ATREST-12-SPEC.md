# ATREST-12 — check 9's nesting arm reaches ArcParity and NoWalk

**Binds one implementer lane.** Deleted at merge; `work/atrest/ATREST-12.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `atrest/12-check9-arcparity`. Row carried:
`work/atrest/check-9-nesting-arc-parity-and-no-walk-wait-on-the-arc-aware-walk`
(parked on TANG's `#1076` until now — its blocker is answered below).

## Why it is dispatchable now

Check 9's nesting arm (ATREST-5) decides a ring's placement for the
`Polygon` class through `point_in_loop` and for the `Disc` class through
`contain::disc_side`, and stays SILENT on `ArcParity` (arcs over at
least three vertices) and `NoWalk` (arc-bearing, fewer than three
vertices), because the only walk was the vertex polygon, which is
unsound on an arc-bearing loop. ATREST-9 (PR #3204) built the walk that
reads each edge on its carrier — circles and ellipses — in one home,
`splitting::containment::point_in_carrier_loop`, and measured it
adversarially (~43,000 probes across two reviews, zero wrong).

## Settled design

**D-A. Place ring vertices on an `ArcParity` or `NoWalk` outer loop with
`point_in_carrier_loop`**, in the same arm and with the same postures
as the `Polygon` path: an `Out` refuses `RingOutsideOuter` naming the
face and loop; an escalation reports `RingNestingUndecided`; the
no-crossing premise is ATREST-11's checked premise (line and circle
edges) — for ellipse-bearing loops, say which part of it is checked and
which is assumed. Loops carrying spiric or spline edges stay silent
(the walk refuses them), named as residue.

**D-B. One dispatch, not a parallel one.** ATREST-5's review found
`NestingRegion` a narrower second spelling of `LoopShape`. Now that
every class but spiric/spline has an instrument, consider dispatching on
the walk's own answer (`point_in_carrier_loop` for every class, with
`disc_side` kept only if it is materially better for the one-circle
class) — and delete what becomes a second spelling. Justify the choice
at the site.

**D-C. Not this unit:** `boolean::contain::contfp` (CONTACT's ground —
CONTACT has an active orchestrator, who is told the walk exists) and
`chord_join::rehome_rings` (REACH's row). Leave both; do not touch
`contain.rs` beyond what the walk already exposes.

## Measure first

Run the widened arm over every face with rings the corpus builds (all
suites, the tour, the wild corpus) on a throwaway branch before
landing. Expected surface: empty. **If any body a verb produces on
purpose refuses, stop and report.**

## What you owe

Rows that go red without the change: a slot-shaped outer loop
(`ArcParity`) and a half-disc outer loop (`NoWalk`) each with a ring
placed outside refuse by name; the same outlines with rings inside
certify; `ATREST-5`'s `the_silent_classes_are_silent_in_both_directions`
row re-baselined (its classes are no longer silent — say what moved).
Update the banner, the not-yet-checked list, `KERNEL-VERBS.md`'s check-9
paragraph, and add a dated line to `work/tang/arc-aware-point-in-loop.md`
naming this site closed and the two left (`contfp`, `rehome_rings`).
Set the carried row to `review` when you open the PR.

## Verification and landing

Hosted CI: **six** `test (eps = …, n/2)` and **five** `k-lint (gate, …)`,
read at STEP level. At landing: merge `origin/main`; if textually clean,
a local `cargo check --all-targets` on the crates both touched is the
gate on the merge; a hand-resolved merge needs a fresh hosted run. Own
`CARGO_TARGET_DIR` outside the worktree; private scratch; never hold the
build slot for a battery; never `pkill -f`; never end a turn with
background work live. Do not touch `work/atrest/log.md`. Do not merge.

## Review tier

**Single, FULL**: a new refusal on a reviewed walk. Class S / NUMERIC.
