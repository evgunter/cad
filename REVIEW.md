# Review: PR 4396, three small `join.rs` rows (frozen head `0c093837`, base `c00b7708`)

**Verdict: APPROVE-WITH-FIXES** · MAJOR 0 · MINOR 1 · NOTE 5 (+ style)

## Claims
1. **Arc to first order, no amplified formula: holds** (sure). `n·(p−site)` expands to `s|n·t̂| + ½s²κ(n·N)`. With `|n·N| = r cos ψ` and `|n·t̂| = r sin ψ`, that gives `s(1 + ½sκ cot ψ)`. Probe (`margin2.py`, float64 with an arc-length root find), new/s at s = 1e-9: circle 1.000000 and old = new; k = 6, 10, 60 at the flank 1.000000 (old 0.3243 / 0.1980 / 0.0333 = 2k/(k²+1)); both vertices (ψ = 90°) 1.000000, old = new to first order. The divisor is `rotational_sense`'s quantity, decided nonzero for both germs in `germs_face_each_other` (`join.rs:2582-2583`) before `Turn` is built (`join.rs:1577`). That covers `germ_arm`'s `(p_c, rga.dir)` and `nearer_along`'s `(p_e, ega.dir)`, and `turned_past` has no other caller. The sign is preserved because `n·(site−c) = 0` and the divisor is positive. Sign probe: old and new margins were logged on every `turned_past` call over the debug sweep suite and classed against the 1e-9 band. They class alike on all 390 828 circle reads and 18 985 other conic reads. All 431 reads that class differently come from the new row's own poses (sin ψ 0.198 and 0.0333). The suite passed: 2 475 passed, 0 failed.
2. **The row is a real gate: holds narrowly** (sure). Reverting `turned_past` alone (M1) turns the row red at k = 10 only (`bool_join_arc_travel`, −4.78e-9). See MINOR-1 for what it cannot see.
3. **Completed-face invariant, assert reachable: holds empirically** (likely); the structural argument is unproven (unsure). With the assert live, the debug sweep suite passes at ε 1e-9 (2 475 passed, 0 failed, 1 103 s), as does the probe family under claim 4. The assert is also compiled into release (`Cargo.toml:298`, `debug-assertions = true`), so the release batteries below ran it too. CI's debug nextest leg runs the sweep suite, so a trip would go red there. I could not build a path where a later polygon's halves sit in a completed face's loop. The prose argument at `chord_join.rs:3528-3534` ignores rings re-homed into a sliver before it completes; only `pending` rings are guarded.
4. **Forced-order closure: justified as *unreached*, not as impossible** (likely). Missed family, tried: single non-convex sections through a block's top face. That is a 2-turn rectangular spiral, a serpentine, a plus and an oblique zigzag, blind and through, at 0°, 13°, 45° and 71°, every op in both orders (192 runs). All 192 built (the 12 "BAD" volumes at spiral 45° are my oracle's: the turned spiral overhangs the block by 0.03). There were 276 ring-lane reads, all `Clean`, and in every one the arc the winding rejected was `Clean` too. No across-edge read was taken. Still unsearched by the PR or by me: the sphere and cone `QuadricRing` lane.
5. **Nothing else moved: holds** (sure). Release, base against head: `pinch_runs_battery` (3 024 lines) and `rc_wide` shards 6/84 and 50/84 (480 lines each) moved 0 lines.

## Findings
- **MINOR-1: the row gates escalation, not the order** (sure). File: `crates/sweep/tests/a_steep_ellipse_orders_band_apart_sites_along_its_arc.rs:176-196`. Three mutants in `nearer_along` (`join.rs:1296-1300`) leave the row green, with every run stopping at the same filed doors:
  - M2 reverses the travel order;
  - M3 always keeps the incumbent;
  - M4 forces a `Zero` tie and so falls back to the chord, which is main's k = 60 behaviour.

  The k = 60 half passes on the reverted formula (M1), so only k = 10 gates. No run builds, so the closed-form oracle (`shared`) is never compared. The row title says "ordered along its arc", but the row only shows that nothing escalates at `bool_join_arc_travel`.
- **NOTE-1: sites off the conic are amplified by `cot ψ`** (unsure whether reachable). A site displaced δ along the conic normal reads δ·cot ψ in the new margin, against ≈δ in the old (probe: 30.0× at k = 60, 4.95× at k = 10). The signal-to-noise ratio is unchanged, but measured against the band, a site off the curve by more than ε·tan ψ (3e-14 at ε = 1e-12, k = 60, coordinates near 42) can decide a sign for two sites within the band. `turned_past`'s "cancels nothing" (`join.rs:1336-1338`) covers cancellation only.
- **NOTE-2: the divisor re-spells `rotational_sense`** (likely). `|(axis×radial)·t̂|` at `join.rs:1343-1348` and `axis·((p−c)×dir)` at `join.rs:1405` agree only for a unit `axis`. The `Ellipse` axis is documented unit but "conventional, unchecked" (`crates/geom/src/curves.rs:153`). Only prose ties them together. DR-74 recorded "one home for the rotational sense".
- **NOTE-3: the PR says "debug CI", but the assert is release-live too** (sure). The PR body and the row's `## Built` frame the assert as debug-only, but release keeps `debug_assert!` (`Cargo.toml:274-299`).
- **NOTE-4: `Turn` mixes two germs** (likely). `sense` comes from the candidate germ at `p_c`, and `tangent` is the entry germ's `ega.dir`, which points back along the conic. `germ_arm` rebinds `site` to the germ's own site (`join.rs:1371-1375`). Only the `abs()` makes this correct.
- **NOTE-5: first-order prose read out to the half-turn** (unsure). `germ_arm` reads the margin as far as the half-turn. At the antipode it is arc again to first order, by central symmetry, but in between it is neither arc nor plane distance. The tie prose at `join.rs:1286-1288` holds near the site only.

## Style (Q1, Q2, Q3, Q4, Q5, Q6, Q8 exercised; Q8: `join.rs` read 1-3600 in full, 3600-5646 skimmed)
- `join.rs:1343-1348` / `1405`: the triple product has a second spelling (NOTE-2), a fresh instance of a duplication a structural fix closed. Also check the prose re-spellings at 1396, 2477 and 2555. Confidence: likely.
- `join.rs:1337`, `1577-1588`: "decided nonzero before ranking" is a call-order invariant inside `partners`. No type carries it, and `turned_past` accepts any `Turn`. Confidence: likely.
- `join.rs:1262`, `1339-1340`: the metres argument needs `HalfGerm.dir` to be unit, which is prose only (`boolean/mod.rs:1584`). That adds a reader to `work/pin/boolean-predicates-read-a-direction-as-unit-by-prose.md`. Confidence: likely.
- Q3, row `:176-196`: the assertion is satisfied by any run that refuses at one of the two doors, whatever order it chose (MINOR-1). Confidence: sure.
- Q4, `work/join/the-walk-order-is-spelled-twice.md` cites `turned_past`, which now reads arc like `splitting/join.rs:519`'s gap. The PR neither updates that row nor holds the two spellings against each other. Confidence: unsure.
- Repeats: three sites map `rotational_sense`'s `Zero` to `RADIAL_GERM` (`join.rs:641`, `1588`, `2570`); `nearer_along` and `nearer` open with the same arm test; the `escalate` closure is redefined about six times. Confidence: sure.
- Q8: about 900 lines of kind-keyed section-frame dispatch (`join.rs:1660-2540`), plus its tests, live in the join and are absent from the module header. Confidence: sure.
- History in present-tense docs ("fix pass, dev 4", "M5 PR 9") at `join.rs:1555`, `2545`, `2785`. Confidence: likely.
- `work/issues/an-ellipse-span-...md` "A note for JOIN": the argument that the old fallback never shipped a body rests on the certifier's meter staying conservative. It is unguarded, which the PR body admits. Confidence: likely.
- Q6: row 2 is "pinned by a debug assertion, not a row", and that deviation is disclosed with no schedule. If the invariant is ever broken, no row exists to show it. Confidence: likely.

Probes: `review-probes/pr4396-probes.patch` (forced-lane logging, the steep-ellipse logger, the non-convex family) and `review-probes/margin2.py`.

REVIEW COMPLETE
