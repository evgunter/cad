# MEASURE: is `review_pick_r2`'s tally nondeterministic? (in progress)

Test: `viewer::all` `review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus`.
It is in the `ci` profile's slow set, so it runs nightly under the default
profile (`cargo nextest run --workspace`, dev/test profile, unoptimized).
That is the profile measured here. One process per run, as nextest runs it.

Trees:
- head: `origin/join/3-segment-curve` = 6cc46300, built in its own `CARGO_TARGET_DIR`
- mb: `merge-base(origin/main, head)` = 66bbdaa6, built in its own `CARGO_TARGET_DIR`

## Early result (more runs in flight)

The first 19 default-thread runs on each tree give the same tally every time, and the two trees differ:

| tree | runs | tally |
|---|---|---|
| head | 19/19 | `(442782, 141983, 12774, 6870)`, which fails against the pin |
| mb   | 19/19 | `(442782, 141983, 12786, 6882)`, which passes |

The per-document diff is entirely in `die_composed_tour`: 96/72 on head against
102/78 on mb, at open and again after the first edit, so 2 × 6 = 12. The meshes of
`die_pips`, `die_tool`, `die_composed` and `die_composed_tour` differ between the trees.
They have the same topology and counts, but the pip rim vertices on the die's top
face move by 1 ulp (e.g. `0.47508625342744654` → `0.4750862534274465`).
