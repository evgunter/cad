# MEASURE: is `review_pick_r2`'s tally nondeterministic?

Test: `viewer::all` `review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus`.
It is in the `ci` profile's slow set (`.config/nextest.toml`), so it runs nightly
under the default profile (`.github/workflows/nightly.yml`:
`cargo nextest run --workspace`, the unoptimized test profile). That is the
profile measured here. Each run is one process, as nextest runs it, and takes
about 34 s.

Trees, each built in its own `CARGO_TARGET_DIR`:
- **head**: `origin/join/3-segment-curve` = `6cc46300`
- **mb**: `merge-base(origin/main, head)` = `66bbdaa6`

The thread knob is `RAYON_NUM_THREADS` (the global pool; `mesh::tessellate`'s
per-face map runs on it). The box has 4 vCPUs, as CI does, and "default" means
the variable is unset, so the pool is 4 wide. Instrumentation was local only:
the test's landing callback printed a per-document tally and an FNV hash of
every part's mesh (position bits, triangle indices, patch order).

## Runs

| tree | threads | runs | tally (rays, grazes, refused, rays with a refusal) | pin |
|---|---|---|---|---|
| head | default (4) | 20 | `(442782, 141983, 12774, 6870)` × 20 | fails |
| head | 1 | 20 | `(442782, 141983, 12774, 6870)` × 20 | fails |
| head | 8 | 20 | `(442782, 141983, 12774, 6870)` × 20 | fails |
| mb | default (4) | 20 | `(442782, 141983, 12786, 6882)` × 20 | passes |
| mb | 1 | 20 | `(442782, 141983, 12786, 6882)` × 20 | passes |
| mb | 8 | 20 | `(442782, 141983, 12786, 6882)` × 20 | passes |

The per-document output, including every landing's mesh hash, was
byte-identical across all 60 runs on head, and across all 60 on mb.
**Neither tree is nondeterministic.** The tally does not move with the run or
the thread count. The two trees differ, and they differ deterministically.

## Where the 12 go

Per document, the only tally difference is `die_composed_tour`:

| landing | mb refused / rays with a refusal | head |
|---|---|---|
| `die_composed_tour` open | 102 / 78 | 96 / 72 |
| `die_composed_tour` after the first edit | 102 / 78 | 96 / 72 |

That is 2 × 6 = 12 on each count. Rays and grazes are unchanged everywhere.

The meshes of `die_pips`, `die_tool`, `die_composed` and `die_composed_tour`
differ between the trees (both landings each). They have the same topology,
position count and triangle count. What moves is the pip rim vertices on the
die's top face, by 1 ulp, e.g. `(0.5766766276757043, 0.47508625342744654, 1.0)`
→ `(0.5766766276757043, 0.4750862534274465, 1.0)`. Only `die_composed_tour`'s
aim happens to cross candidates whose determinant certification flips with that
ulp.

## Source: JOIN-3's first commit

I built and ran the instrumented test at points along JOIN-3's first-parent
history:

| commit | tally | `die_pips` open mesh |
|---|---|---|
| `0b6e39ca` (main, parent of JOIN-3's first commit) | `(…, 12786, 6882)` | `c8c2c62b…` (= mb) |
| `3191f2ee` JOIN-3: a matched segment carries its chord curve | `(…, 12774, 6870)` | `57d5693c…` (= head) |
| `e222129c`, `086e749b`, `3be364cb`, `2cc1455c`, `16a83c32`, `f2eb081d`, `1c791a9d`, `ea20aeed` | `(…, 12774, 6870)` | `57d5693c…` |

`3191f2ee` makes the boolean join compute a matched segment's chord curve once
(`chord_join::SegmentCurve`) and mint both chords on it. Before it, the second
chord recomputed its section arc from another run
(`work/cleave/split-lane-second-chord-recomputes-the-first-chords-arc.md`).
The pip rim is such a segment: the subtracted pip's edge on the die's top
plane. Its chord points now come from the one shared curve, which moves them
by an ulp. This reaches the mesh through `crates/topo/src/chord_join.rs` and
`crates/topo/src/boolean/join.rs`, both in the asked-about diff (`crates/topo`).
No HashMap iteration, parallel reduction or timing cutoff is involved, because
the thread count moves nothing.

Every JOIN-3 commit from `3191f2ee` onwards carries the moved tally, but the pin
was never re-derived. The test is in the slow set, so per-PR CI never ran it
and only a nightly or direct run sees it.

The reported passes on head (three, in a build directory shared with main) did
not reproduce in 60 runs here. Running a binary that a shared target dir had
last linked from main's sources would give exactly main's tally, which is a
likely account of them, though I did not verify it.

Note for the re-pin: `origin/main` (`5ea1a9f0`) has since moved this pin's
graze count to `141_968`, so the re-derivation after the next main merge must
be taken on the merged tree.

## Verdict

**(a) JOIN-3 introduced it**, at `3191f2ee`, as a deterministic mesh change
rather than nondeterminism. `review_pick_r2` needs a re-pin on JOIN-3
(`die_composed_tour` −6/−6 at each landing; no genuine crossing refused). Main
(merge-base `66bbdaa6`) is stable at the pin in 60/60 runs.

MEASURE COMPLETE
