# SYM-1 — the profile inside the normal form (spec)

**Program:** SYM (`work/sym/plan.md`, the cost lane). **Item:**
`work/sym/symbolic-tier-costs-95-percent-of-the-m10-3-drive.md`, asks 1
and 2 — and only those. **Track:** a MEASUREMENT unit. It moves nothing
the tier decides, so it is outside the A/B experiment: no ordinal, one
review (style lane plus the claims below to falsify), no A/B row. The
fix it makes possible is a later unit and runs under the full dual.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item above and `work/tcost/m10-3-chamber-row-reads-ten-times-its-recorded-cost.md`
(the measurement this unit explains — S-TCOST's, cited and never
re-taken as a result of record); `crates/geom-core/src/sym.rs`'s header,
the `# Freezing` section especially; `crates/editor-core/src/drive.rs`'s
`SymbolicDials` note.

## The question

The E12 tier is 95 % of the M10-3 interval suite's wall (S-TCOST, local,
20.8×), and nobody has measured where inside the normal form that time
goes. The item names three candidates and says none is measured:

1. **degree growth from carried denominators** — cross-multiplication up
   the DAG, every normalisation walking a form that reached degree 32+;
2. **the coefficient ring** — `Rat` over `num-bigint` past the `i128`
   inline path (`Int::Small` → `Int::Big`), how often the promotion fires;
3. **term storage and allocation** — the `BTreeMap`/`HashMap` per form,
   rebuilt per normalisation.

**The deliverable is the ranking of those three, with numbers, on the
M10-3 slab, plus the freeze and promotion population read off the
counters** — and a method a later unit can re-run for a hosted
before/after. Not a fix. If the profile makes a fix obvious, write the
proposal in the item's body as the next unit's input; do not implement
it here. The one exception is stated under Scope.

## Phase 1 — the instrument

Two instruments, because they answer different halves:

- **Function-level time**: `valgrind --tool=callgrind` is installed on
  this box (`perf` is not). Run it over ONE replay, not the suite: a
  single whole-box leaf of the M10-3 chamber at its nominal, or one
  bisection step — the fixture is `bounded_chamber` / `slab_with` in
  `crates/editor-core/tests/m10_3_r1_probes_interval.rs`, driven through
  the same doors the suite uses. Instructions-retired split by function
  under `geom_core::sym` (`combine`, `form_in`, `Poly`/`Mono` ops,
  `mono_mul`, `Rat`/`Int` arithmetic incl. gcd, `algebra::reduce_steps`,
  `trig`, the memo maps' hashing) is the answer to "which of the three
  dominates". State the profile's method in the item body so it can be
  re-run: the exact command, the row, the build profile, the box.
- **Structural counters, in-tree and re-runnable**: behind a cargo
  feature on `geom-core` (`sym-profile`, the shape of
  `identity-pass-testing`: test-only, forwarded through `editor-core`'s
  dev-dependency edge exactly as that feature is, never on the ordinary
  edge), a per-session profile recording per op kind: forms combined;
  terms in → terms out; total degree in → out; freezes by CAUSE (term
  budget / degree budget / coefficient bound / overflow) and by op, with
  the kids' degrees at the freeze; `Int::Small → Int::Big` promotions
  and the widest coefficient seen; and per walk (`plain_form` /
  `early_form` / `door_form`) the count of forms built and the wall time
  by `std::time::Instant`. **With the feature off, none of it is
  compiled** — no branch, no counter, no cost; the shipped tier is bit
  for bit what it was, and the review checks that by diff.

Read `SymCounts::frozen` and the shape report (`sym::report`,
`start_shape_report` / `take_shape_report`, `FormSize`) BEFORE building
anything: they exist and the item says nobody has looked. Report what
they say on the slab first; build only what they cannot say.

## Phase 2 — the measurement

On the M10-3 slab (`bounded_chamber`, the row S-TCOST bisected on), at
the nominal and at one macroscopic box, `test` profile as S-TCOST
measured and `release` beside it if the split differs:

1. **The ranking** of the three candidates, with the instruction share
   from callgrind and the counts from the profile. Say which dominates
   and by how much; say whether the answer is the same at the nominal
   and over the box (the header says every form is built at the nominal
   and skipped where the numeric channel already answers).
2. **The freeze population**: how many forms freeze, on which cause, at
   what degree, from which op, and what the frozen forms' KIDS look like
   (degree, term count) — the mechanism the derived-frame item
   (`work/sym/derived-frame-placement-freezes-on-the-symbolic-lane`)
   diagnosed as "each normalisation adds a denominator and each square
   doubles the degree" should be visible or absent here, and the item
   body says which.
3. **The promotion count**: how often the coefficient ring leaves
   `i128`, and whether the `Big` arithmetic's share of the time matches
   its share of the operations.
4. **Where the walks spend it**: plain vs early vs door, and inside the
   early walk how much is `reduce_steps` (rules A/B per node) and how
   much is rule D's `trig`.

Then, **once more on the two-hole plate at its nominal**
(`demos/tour` stop 1's document; `editor-core/tests/m10_7_plate.rs` or
the M10-10 pins build it), the freeze population only (the header
records 1,056 frozen forms there), so the slab's numbers have one
comparison point on the document the tier was built for.

## Phase 3 — the record

- **The item body** (`symbolic-tier-costs-95-percent-of-the-m10-3-drive`)
  gains a `## The profile (SYM-1)` section: the ranking, the tables, the
  method to re-run, and what a fix would target — one proposal per
  candidate that the numbers support, each with the number that argues
  for it. Nothing in that section is a claim without a table behind it.
- **The coverage observation** the item recorded without an argument
  ("all nine M10-3 rows pass with `enabled: false`") gets its answer in
  one paragraph: which rows in the tree DO go red with the tier off —
  `rg -n "SymbolicDials::off|without_the_algebra|SymBudget::none" crates/*/tests`
  and the M10-10 pins are where to look — so the reader knows the
  tier's answers are pinned somewhere even if not in the M10-3 suite.
- **The PR body** carries the same tables and the callgrind command.
- **`sym.rs`'s header**: one paragraph under `# Freezing` (or a new
  `# Cost` heading) stating the measured ranking as a present-tense
  fact with its instrument named — the design's own words today are a
  hypothesis (`drive.rs`'s "the degree that matters is the degree AFTER
  those denominators have been carried up") and this unit either
  confirms or corrects them. Correct `drive.rs`'s note only if it is
  wrong, by announced seam to PROPS (the file is theirs).

## Scope, and the one exception

- **No change to the normal form, the rules, the dials, the budgets or
  `COEFF_BITS`.** A patch written from reading the code is exactly how
  the degree-16 hypothesis got refuted (the item's own words).
- **The exception:** if Phase 1 finds a defect in an EXISTING counter —
  `frozen` counted wrong, a shape-report column that lies — fix it in
  this unit, with a row that reds on the old behaviour, because the
  measurement rests on it. Say so in the PR body.
- **No new gating row.** Every row this unit adds is `#[ignore]` with a
  reason or gated behind `sym-profile`; the hosted gate's cost does not
  move by a second. `crates/editor-core/tests/all.rs` is the one test
  binary and is S-TCOST's/S-TINT's ground; register the file there and
  nothing else. New editor-core rows go in a file under the program's
  glob (`crates/editor-core/tests/m10_*`), named by subject
  (`m10_sym_profile_interval.rs`).
- **Territory:** `crates/geom-core/src/sym.rs` and `sym/*` are this
  program's; `crates/geom-core/Cargo.toml` and
  `crates/editor-core/Cargo.toml` carry the feature and are announced
  (PROPS, in `work/props/log.md` at dispatch, already done). Run
  `python3 scripts/work.py territory --base origin/main` before the PR
  and list what it names in the PR body.

## Acceptance

- The three candidates ranked with instruction shares and counts, on
  the slab, at two scales; the freeze population by cause/op/degree;
  the promotion count; the per-walk split. All in the item body with
  the re-run method.
- `sym-profile` off: `geom-core` and `editor-core` compile to the same
  code paths (no `cfg` leaves a runtime branch behind; the review diffs
  the feature-off build's behaviour by the existing suites — the
  M10-10 pins and `SymbolicDials::off()`'s bit-identity rows are the
  pins, and they do not move).
- Hosted CI green on the full matrix; the gate's wall on the
  editor-core interval shard within noise of `main`'s (read both runs'
  job timings and state them).
- The item's asks 1 and 2 answered; ask 3 (the change) explicitly NOT
  taken, with the proposals it would start from.

## Review

One reviewer, outside the experiment. Claims to falsify: (1) the ranking
— the reviewer re-runs the callgrind command on the same row and gets
the same order; (2) the feature-off build changes nothing (diff of the
feature-off `sym.rs` code paths; the pins); (3) every number in the
item body has a table and a method behind it; plus
`docs/prompts/reviewer-style-lane.md` in full. Fix pass on the
implementer's lane; the unit's log entry rides the PR last.

## Landing

Status `review` on `work/sym/SYM-1.md` when the PR opens; the
orchestrator closes the unit and deletes this spec at merge.
