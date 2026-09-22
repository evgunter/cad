# TESS-2 (ordinal 5101) — dual review, adjudicated 2026-09-22

Frozen head `445257159`. R1 = OPUS, R2 = FABLE (byte 220). Both
APPROVE-WITH-FIXES; neither produced an escape (R1: 33,750 exact
containment checks; R2: 704,835 on the head, 12,615 escapes on the
reverted tree — the bulk on the INTEGRAL refined arm, new evidence);
both reproduced red-before/green-after and the lerp mutant.

| | MAJ/MIN/NOTE | devs reported / silent | idiom | tests | docs | wall-clock | tokens |
|---|---|---|---|---|---|---|---|
| R1 | 1 / 7 / 5 | 6 / 1 | 4 | 2 | 3 | ~1 h 35 m (~50 min slot) | 228k |
| R2 | 0 / 5 / 4 | 6 / 1 | 4 | 3 | 4 | ~1 h 55 m | (at report) |

R1's MAJOR (test-gap, demonstrated by revert): the spec-mandated
deterministic stratum row passes bit-identically with the fix
reverted — it compares through the 64-ulp allowance and its worst
patch is unit-weight (integral arm). R2 raised the same as MINOR-1.
**Tally candidates: none** (3a fails). The two "silent" deviations
differ: R1 names the stratum row's inability to fail, R2 the timing
fixtures (spec asked for tour bodies). Both briefs identical; no
relaxation; R1 one names-only glimpse (a `ps` chain and a listing
showed the other lane's paths and a report filename, no content);
R2 none. Pair FAIR.

Converged (both): stratum row blind; "convex form cannot bulge" false
(constant column leaves the hull — α_hi + β_hi > 1); PR-body AFTER
digits stale (A …68614, B …74851 on the head); referee names a
nonexistent row and its round-down argument is not what `Decimal.sqrt`
does; `RefinedWeightLostPositivity` untested and its prose names an
impossible mechanism (reachable only by underflow); `insert_once_ring`'s
non-unification argument invalidated; `point_at` recomputed per cell.
R1 only: dust re-pin admits both worlds; sampler-error method not
committed; a third `split_points` spelling in the algebra row.
R2 only: integral refined arm was unsound too; net.rs skeleton copy;
`Option<Ratio>` held by convention; "affine in v" doc; the `.py`
nobody runs; timing-fixture deviation.

Fix pass: union, executor implementer-inherited.
