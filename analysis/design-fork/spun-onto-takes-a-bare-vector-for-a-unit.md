# Design fork: a door takes a table's written direction for a unit

Item: `work/boxfinish/spun-onto-takes-a-bare-vector-for-a-unit.md` (on `boxfinish/signed-radius` until that unit's PR lands). Protocol `ddd201f411`. Orchestrator: BOXFINISH.

## Draws (2026-11-22 09:40 UTC)

| Byte | Value | Meaning |
|---|---|---|
| arm | 182 | 86 or above: two designers |
| labels | 203 | two designers; labels byte 203: odd: Fable is A, Opus is B |

## Sessions

Dispatched concurrently 2026-11-22 09:35 UTC, the same prompt to both (`probe/briefs/designer-unit-claim.md` on `boxfinish/briefs` at `e006088853`: the problem, what was measured and what was not, where to look; no options from the orchestrator). **A framing exposure:** the item file both are pointed at (the finding lane's row on `boxfinish/signed-radius`) carries that lane's repair shape; the brief calls the row's framing and its repair shape hypotheses.

| Label | Model (configured) | Session |
|---|---|---|
| A | Fable 5.2 (`claude-fable-5-2`) | session_013QoYWUCDiArwq6Mw4jFscX |
| B | Opus 5.5 (`claude-opus-5-5`) | session_013QoYWYaDtvTdGWWq5zAtCf |

## First reports (recorded as delivered, before any reconciliation)

- **A, 09:42 UTC.** Answer: not a fork; one answer follows from ratified text and needs no new DESIGN text. A recorded direction is a claim ("this is a unit") and `[scalars]` says a claim is held as its witness type: build `UnitVec2` in `geom-core` beside `UnitVec3` with private contents and a closed list of makers (the exact axes; cos and sin of an angle; a vector over its own length; negation and the quarter turn; a unit turned by or reflected across a unit; the chord lowering's two radials), no maker on trust and none that "checks it is already unit"; the profile's record holds that type (`Dir` and the sweep's `Unit2` become it); a table cannot write a direction (it hands over a vector, divided by its length on the way in, or an axis); `spun_onto` takes a `UnitVec2` with no arithmetic and `spun_toward` is divide-then-`spun_onto`; nobody reads the frame afterwards, so ONEHOME's removal stands; lines get the same answer. One line of argument: with the 2-D witness the hole is unwritable and the lane's symbolic identity is kept. Confidence: sure on the witness as the answer; likely that it closes every maker without a new trusted door. Framing rejected: in part (the table door is public only under `test` / `test-support`, with two test files as its only callers, so the hole is a statement about types and not a reachable defect; the census missed a reachable maker, a profile validated at `f64` and lifted to another scalar through `from_f64`, a point box around a rounded unit; the brief's "not measured" items need no measuring under this answer). Ratified text challenged: n (what contradicts it is agent-written code documentation: `SegmentKind`'s "an accepted direction is stored as written, never normalised" and `UnitVec3`'s note that the 2-D analogue "is left where it is"). Rejected: a constructed/written mark on each stored direction; normalizing at the table door with the types left alone; keeping tier 3's reading.
