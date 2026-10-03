# Verify: PR #3976 (reach/lily-leaf-1e12) at a17c1efbd0

**Verdict: VERIFIED.** All five mutants are red. The PR's rows are green at 1e-12, 1e-9 and 1e-6. Every lane claim holds. The branch head has not moved past a17c1efbd (re-fetched at the end).

Local runs on 2026-10-03, with my own target dirs. The release profile was used for the demo roots, as CI's tour job does. ε is set with `CAD_TOLERANCE_EPS`.

## Mutants (one at a time, each reverted)

| mutant | edit | row(s) | result |
|---|---|---|---|
| (a) point bracket | `main.rs` `continued`: Bracket → `volume_hi = volume_lo` | `k_probe_brackets` 1e-12 | **red**: `lily_leaf_b: [3.1104e-3, 3.1104e-3] is not an enclosure` (1e-9 and 1e-6 green, as expected: no bracket there) |
| (b) displaced range | `continued`: Bracket → `[hi, 2hi-lo]` (lo<hi, off Pappus) | `k_probe_brackets` 1e-12 | **red**: `lily_leaf_b: Pappus 3.13441e-3 outside [3.15907e-3, 3.20770e-3]` |
| (c1) third body | `probe.rs` kind: `lily_leaf_a` reported `bracket` | all 3 `k_probe_brackets` rows | **red** ×3: bracketed set `{leaf_a, leaf_b, leaf_c}` ≠ expected (1e-12); `{leaf_a}` ≠ `{}` (1e-9, 1e-6) |
| (c2) drop a leaf | `probe.rs` kind: `lily_leaf_c` Bracket reported `number` | `k_probe_brackets` 1e-12 | **red**: `{leaf_b}` ≠ `{leaf_b, leaf_c}` |
| (d) lily row, displaced | `lily.rs` Pappus row: `(lo,hi) = (hi, 2hi-lo)` | `finding_13_tessellation_table_reproduces` at 1e-9 / 1e-6 / 1e-12 | **red** ×3: `Pappus … outside the certified […]` at every ε, including the wide ones (no skip) |
| (e) record reworded (review Q7) | `probe.rs` eprintln `\tvolume\t` → `\tvol\t` | all 3 `k_probe_brackets` rows | **red** ×3: `no volume record for lily_leaf_b`. The parse fails loud, not vacuous |

## ε results (head a17c1efbd)

| row | 1e-12 | 1e-9 | 1e-6 |
|---|---|---|---|
| `demos/tour` `k_probe_brackets` (3 rows, `--features probe`) | green | green | green |
| lily `finding_13_tessellation_table_reproduces` (Pappus containment) | green (wide, contains, prints "weak evidence") | green | green (wide, contains) |
| `scripts/k_probe_sweep.sh` (corpus + M2 + driver + demos) | **exit 0** (main: demo k-probe panics `lily_leaf_b: mass at Probe … QuadratureBudget { width_len: 1.5433e-8, target_len: 1.024e-9 }`, so confirmed red on main and green here) | exit 0 | exit 0 |
| `demos/wild` (8 cells) | exit 101 at import: `nist_ftc_09_asme1_rd` edge #2928 attachment gate. **Also red on origin/main, same panic, so not this PR's** (filed in the PR as `wild-nist-ftc-09-refuses-import-at-eps-1e-12`) | 8/8, exit 0 | 8/8, exit 0 |

Changed crates' suites at 1e-9:
- `cargo nextest run -p topo -p pncad` (debug): 2266 passed, 112 skipped.
- `demos/tour` `cargo nextest run --release --features probe`: 98/98 passed.
- fmt and clippy `-D warnings` are clean on `demos/tour` (`--all-targets --features probe`) and `demos/wild` (`--all-targets`).

Readings at the head: leaf_b at 1e-12 is the bracket [3.110441e-3, 3.159072e-3]; leaf_c is [1.685620e-3, 1.704619e-3]. The Pappus volumes are 3.134413e-3 and 1.695118e-3. At 1e-9 both leaves are numbers of width about 1.4e-4 and 8.2e-5 of Pappus. Each label appears once among the 41 volume records, so `find` by label is unambiguous.

## Claim checks

| claim | check | holds? |
|---|---|---|
| `k_probe_brackets.rs`: lo≤hi at every ε, strict for brackets; both leaves read and contain Pappus; bracket set exactly {b, c} at 1e-12 and ∅ at 1e-9 and 1e-6 | read `check()`; mutants a, b, c1, c2 | yes |
| (a) and (b) now fail with "is not an enclosure" and "Pappus … outside" | mutants a, b, same messages | yes |
| lily Pappus row asserts at any width, no skip; green at all three ε | diff `lily.rs:3373-3393`; mutant d red at all three; baseline green at all three | yes |
| `k_report.rs` reverted to main | `git diff origin/main a17c1efbd -- crates/sweep/tests/k_report.rs` empty (also vs the merge base) | yes |
| wild's bracket branch removed; wild requires a number; 8/8 exit 0 | diff: `enclosure.unwrap_or_else(panic …)`, no bracket arm. 8/8 at 1e-9 and 1e-6. At 1e-12 it dies earlier, at import, same as main | yes. Note: wild reads `StepImport::Solid::enclosure` (a `Result`) directly, not through `VolumeReading`, and the claim does not say otherwise |
| `topo::VolumeReading` is the one home; tour gate, lily row and klein use it; teapot keeps its own match | `props.rs` adds `VolumeReading::of`/`enclosure`; `main.rs` `continued`/`gated` (probe goes through `gated`), `lily.rs`, `klein.rs` use it. `grep 'bracket: Some'` over demos and crates outside props finds only `teapot.rs:2159` | yes |
| `k_probe_sweep.sh` exits 0 at all three ε on this head; red on main at 1e-12 at lily_leaf_b | full sweep exit 0; main demo k-probe at 1e-12 panics at lily_leaf_b | yes |

Review findings against the fix pass:
- **F1** (rows blind to a degraded bracket) is closed (mutants a and b).
- **F2** (the M2 loosening) is closed by the revert.
- **F3** is moot: the M2 harness is reverted, so its population no longer moves.
- **F4** (wild's dead bracket arm and midpoint slack) is closed: the arm is deleted.
- **F5** is the kernel's, left to QUAD as the review asked.
- **Style:**
  - Q1 is closed: one home, with teapot's bit-exact match a deliberate exception.
  - Q5 is closed: the module doc names `gated` and the bracket.
  - Q7 is closed: tab-separated records, and the parse fails loud (mutant e).
  - Q6 (the nightly reruns the demo sweep in debug after `k_probe_sweep.sh`) still stands as the review left it: "unsure", not blocking.

## Not checked
- Hosted CI on a17c1efbd: not read here; the local runs above are evidence, not the record.
- `tools/k-lint` over the fresh CSVs: not run.
