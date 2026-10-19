# BAND — the plan

the blend and rim bands: what a roll builds over a rim, and what it may call the result

Opened 2026-09-20 by CARVE's priority-seam cut
(`work/README.md`, Track size).

## The slate

The opening slate (four P0 and six P1 rows) is built and closed; what
is open is every live item in this directory, banded in its own
header, and `work/STATUS.md` lists them. In outline: one P2 H unit
(step 4 of the band-feature rule), a P2 M Interval refusal, three P2 E
error-propagation rows, and a P3 tail of conservative refusals and wide
meters.

## Order

Step 4 of `a-concave-fill-and-a-whole-shell-inside-a-bands-region`
first if the track is picked up for its rule: whole shells (a void in
`R`, an island in `F`) and the seal (a through-hole turned blind, a
blind hole turned void) share one carve, `topo::insert_void`'s
bookkeeping, and `docs/DESIGN.md`'s void-insertion convention already
names the blend's seal as a producer. It costs H and gets a concurrent
dual review. The three P2 E rows are independent and cheap; the anti-seam
Interval refusal should share one helper with `surgery::arc_window`
(its item says so). The P3 tail has no order; take a row when a user
or a neighbouring unit meets it.

## Review posture

Per unit, by the review tiers in `memories/orchestration-model.md`;
the model A/B the opening text inherited is suspended, so there is no
triage question left to answer. Each dispatch names its tier in the
log.

## Note from SCALAR (2026-09-29)

LANE-4 (PR #3194) merged on 2026-09-29, so the wait above is over and `S90-impl` is due its re-read against the tree.
