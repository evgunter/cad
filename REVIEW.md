# Review 2: PR 4415 (frozen head a5f10476), DUAL tier, sequential arm, second review

**Verdict: APPROVE-WITH-FIXES** · MAJOR 0 · MINOR 2 · NOTE 3.

| Claim | Result |
|---|---|
| 1 | Falsified in one direction: a certified in-band shell stays `ResultInvalid` (MINOR-1). Never wrongly `Escalated`. |
| 2 | Holds |
| 3 | Holds |
| 4 | Holds (MINOR-2 is about the lever's words) |
| 5 | Holds |
| 6 | Holds |
| 7 | Holds |
| 8 | Real, and rightly out of scope |
| 9 | NOTE-1 |

**Method.** Base was the merge base `5bfec1a2` against the head, plus the first head `24aa90e7` for one pose. Each had its own release target outside the checkout, and every build was read through `differential::outcome`.

**Probes.**
- `crates/topo/tests/review2_door_typing_probes.rs`, gated by `R2_RUN=1`. The base variant drops the `sliver` field.
- `review-probes/4415-r2/`: the census runner, the base/head comparator, the battery runner, and `lump.py` (exact rationals).
- Mutants were env-gated in a scratch worktree, never committed.

I read the first review only after forming my view of claims 1–9. I read no other lane's branch or PR comment.

## The first review's findings, re-checked on a5f10476

| Finding | State | Evidence |
|---|---|---|
| MAJOR-1: the offer quoted the first shell | **resolved** | Mutant "first finding binds" (`ops.rs:3737`): `the_gate_types_only_…` and `every_offered_tolerance_passes_just_below_it` both go red. Clean head is green (4/4). |
| MINOR-1: dropped diagnostics | **partly** | Mutant "`others` = 0": the gate test goes red. But the named `solid`/`shell` are keys into a body that is never returned (`mod.rs` ResultInvalid doc: "no body below it is ever returned"). So the move-the-parts lever still cannot locate the piece; only the count is new information. Naming one shell is enough for the *offer*, which the binding shell decides for all. For the lever, neither one shell nor all keys is enough. A location would be. |
| MINOR-2: undisclosed walk-decided `Escalated` | **resolved as asked, and the revert re-opened a D10 typing** | Mutant "walk-decided ends carry the sliver": `a_bracket_refuses_by_its_decided_ends` goes red, so `classify_shells`, Connectedness and the tag answer as on main (`props.rs:3261-3275` is unchanged). But see MINOR-1 below. |
| S1: an identical-pair test | resolved | The two-enclosure table goes red under the first-binds mutant. |
| S2: two spellings | resolved | Mutant "`CertifiedSliver::of` ignores `terminal_sliver`": `only_an_enclosure_wholly_in_band_certifies_a_sliver` goes red. |
| S3: the pose spelled four times | resolved / filed | `basis3` normalizes (`offer_rows.rs:506`); P4 row filed. |
| S4: docs | resolved | `boolean/mod.rs:2167-2171` and `2879-2882`. |
| S5: the probe oracle | resolved | The probe prints `LUMP … v/a=3.288716e-9`, matching the certificate. |
| S6: a comment | resolved, but see S4 below | |
| S8: `structural_gate` | resolved | `ops.rs:3713-3716`: `structural_gate` returns `ResultInvalid` directly. |
| NOTE-1: the threshold straddle | filed, premise false | Filed as P3. Its walk-decided arm is now witnessed (MINOR-1). |
| NOTE-6 | recorded | Present on the P0 row. |

## Findings

**MINOR-1: the fix pass re-opened a certified in-band shell typed as the kernel's defect** (executed). I found it with a census family the PR did not run: ε = 1e-9, tilts ±6e-9.

The pose is `vee300 nt e0 a1 d6e-9`, ∩, both orders. Instrumenting `certify_role` shows:
- the certified enclosure is [1.6997276e-9, 1.6997280e-9], `terminal_sliver: true`, wholly in band;
- the probe's oracle reads V/A = 1.6997e-9;
- the f64 walk reads 8.716e-10 (pc) and 3.268e-10 (cp), which is **in the zero band**.

So `certified_by` sets the sliver, and `shell_role_refusal`'s `(Ok, Ok)` arm then discards it (`props.rs:3261-3275`). The door refuses `ResultInvalid{ShellRoleUndecided{ZeroVolume, margin 8.7e-10}}`, quoting a zero that the certificate refutes.

On the first head `24aa90e7` the same pose refused `Escalated{ShellRole}` on that enclosure. Base refuses `ResultInvalid` like head, so this is not a regression against main. But it falsifies claim 1's second direction on a reachable pose, inside the clause this PR says it builds. Frequency: 2 of the ≈380 in-band shells in my families, all at ε = 1e-9, none at 1e-6 or 1e-12.

The filed row `a-threshold-straddling-in-band-shell-is-typed-the-kernels.md:36-41` says this arm "takes a walk that is wrong by more than the band's width" and "no pose is known to reach it". Both are false. The walk need only cross ε: here it is 49% and 81% off, which is less than the band's width.

The fix the first review called "defensible" does not have to touch `ShellClassifyError`. The door could read the sliver that `RoleUnread` already holds (for example, carried beside the error in `ShellRoleUndecided`). That keeps MINOR-2's surfaces unchanged and honours D10. At least, the row needs this witness and its priority revisited.

**MINOR-2: the new lever quotes a false threshold** (executed). `refusal_routes.rs:844` says "move the parts so they leave no piece or cavity thinner than the tolerance". My thin-void family is a box less a unit-square cavity `h` deep, with a filler crossing it and no coplanar faces. At `h` = 1.02e-8 (10ε) the door refuses with this text, though the cavity is ten times the tolerance thick. The refusal holds up to `h` = 2e-8 = 2Kε, and the result builds from 2.001e-8.

The decided measure is V/A ≥ Kε, and V/A is half a slab's thickness, so the words are off by up to 2K. The sibling sized levers say "clearly" and quote no threshold. The same predicate's own check-10 ending says "thicken or remove".

**NOTE-1 (claim 9, likely): the door is the right home, but the judgement has two readers.** `finding_arm` only maps what `RoleUnread::certified_by` (`props.rs:889`) decided, which keeps one home for the judgement. But `shell_role_refusal` re-decides the variant off the walk's *uncertified* ends and overrides that home: that is MINOR-1. `certify_role`'s own comment, "The walk's decision is not certified, so the certified read is the refusal" (`props.rs:1036`), sits one arm away from the case that does the opposite.

**NOTE-2 (claim 8, sure): the P0 reproduces on head and should ride its row.** `piece ∩ cube` at d = 1e-8, 2e-8 and 3e-8 returns `Ok`, `shells-per-solid=[1]`. Its own `classify_shells` meanwhile says `Escalated`, with `sliver: Some([3.28872e-9, …])` (probe `r2_one_shell_sliver_and_layout`). The whole prism's ∩ is refused, so the door is inconsistent: as `ResultInvalid` on base, and as `Escalated` on head. The PR's typing makes the contrast sharper ("the operands are ill-conditioned" beside a shipped sliver), but it does not create it. It should not block.

New detail (likely): the witness is caught only because the result groups the unread sliver into the main lump's solid. At d = 1e-8 the binding shell is in solid `1v1` with the main lump. At d = 1e-7, where the role reads, the result is `[1, 1]`, two solids. So the P0 covers any sliver lump given its own solid, not only a one-shell result.

**NOTE-3 (claims 2–7, executed or inspected as marked).**
- **Claim 2.** `finding_arm` (`ops.rs:3765`) is a full match with no `_` arm, so it compiles only exhaustive. Each arm agrees with D10 and the filed P1 (point-margin escalations: nine `RingContactEscalated` witnesses in my ε = 1e-9 family). A mix gives `ResultInvalid` with every finding kept (the gate test checks `errors.len() == n`). The `Escalated` path drops the walk's `source` margin and the other shells' enclosures.
- **Claim 3.**
  - The exact-rational clip of the ideal pose (`lump.py`) gives V/A = 3.288716252e-9, 6.577432470e-9 and 9.866148635e-9 at d = 1e-8, 2e-8 and 3e-8. Each lies inside its certified enclosure. The walk reads 3.050e-9, and only the certificate types.
  - The thin void's exact V/A of the stored depth `2 − fl(2−h)` lies inside the enclosure at three depths.
  - Far from the origin, the witness moved `o` = 1 … 1e5 along x stays `Escalated`. Its enclosure drifts 2.5e-5 relative at 1e5, which is the inputs' own rounding.
- **Claim 4.**
  - Each executed offer passes on the clean head.
  - Moving the parts builds SOUND: `notch307 e0 a3` at d = 3e-7 and d = −3e-8, and the thin void past 2Kε.
  - `thickness()` is f64 code independent of the kernel.
- **Claim 5.** Downstream code keys off `BooleanErrorKind`, and the Python `escalated` tag is unchanged. Outside topo, `BooleanDecision` is matched only by `test_support::decision_key` (updated) and the probe. `ShellClassifyError`'s Display ignores `sliver`. No doc enumerates decisions.
- **Claim 6.** Census, base vs head, 80 640 runs:

  | ε | tilts | moved by type |
  |---|---|---|
  | 1e-9 | the PR's | 60 |
  | 1e-9 | ±2e-8, ±5e-8, ±1.5e-8, ±6e-9 (mine) | 139, plus the 2 MINOR-1 shells left unmoved |
  | 1e-12 | the PR's | 38, plus 20 oracle-only |
  | 1e-6 | ±1.5e-5, ±2e-5, ±5e-5 (mine) | 124 |

  Every move was `ResultInvalid{ShellRoleUndecided}` → `Escalated{ShellRole}`, and 0 others moved. My 170 probe rows: 115 moved by type alone, including the thin void (the dsc_checks shape) and the witness; 55 unchanged. `pinch_runs_battery` (3 024 lines) and `rc_wide` shards 21/84 and 62/84 (480 lines each) moved 0.
- **Claim 7.** The unit's diff against the merge base holds no ported line: no `nextest.toml`, no `cone_join_lane.rs`, and no `boolean_admitting_cones` hunk. The merge's only combined hunk is the `ShellRole` ending arm.

## Style (Q1–Q7 exercised; Q8 partially: the gate region, `finding_arm`, the props role read and refusal_routes' additions, not all 7 588 lines of `ops.rs`)

- **Q1, sure.** `MarginDiag::nearest_zero` (`predicate.rs:1429`) re-spells `MarginDiag::magnitudes().0` (`predicate.rs:1574`), 150 lines apart in one impl.
- **Q3, likely.** A mutant making `quoted_margin` read the *farther* end (`offer_rows.rs:2114`) leaves every offer row green: the fixtures' enclosures are about 1e-16 wide. The change cannot go red.
- **Q3, likely.** A mutant certifying an enclosure that straddles Kε (lo in band, hi ≥ Kε) leaves all four rows green. `only_an_enclosure_wholly_in_band_certifies_a_sliver` (`props.rs:4828`) probes only the zero edge. Only geom-core's classifier row pins the Kε edge, which is the worst direction (kernel → `Escalated`).
- **Q2/Q4, sure.** The comment at `props.rs:1036` (S6's fix) states a principle that the adjacent arm violates. See NOTE-1.
- **Q6, sure.** The P3 row's premise is false (MINOR-1). A disclosed deviation should rest on a true one.
- **Q7, likely.** `InBandShell` is a bare tuple read as `a.2.margin` (`ops.rs:3737`, `3760`). A struct would name the parts.
- **Q1, unsure.** `BooleanDecision::ShellRole` and the root-exported `topo::ShellRole` enum are different things with one name.
- **Q5, likely.** `ShellRole { solid, shell }`'s field docs promise "the binding shell's solid, in the refused result", a handle no caller can resolve (MINOR-1 of the table).

REVIEW COMPLETE
