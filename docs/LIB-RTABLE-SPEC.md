# LIB-RTABLE spec — the four-projection transition table (RESPELL-TABLE; binding)

Mandate: complete the ratified drift-proofing invariant
(PATHS-DESIGN §2c, the round-13 CONSEQUENCE block, ruled round 15
and registered as RESPELL-TABLE by Evan's M2 ruling on #531): the
surface and the replay driver become TWO MECHANICAL PROJECTIONS
OF ONE DECLARATION — a single transition table, one row per
(state, verb, kernel fn, next state), macro-expanded into ALL
FOUR artifacts: the typed method, the driver match arm, the Step
variant, and the tag entry. The shipped interim
(`step_vocabulary!`, ruled on #531) derives only the three
enum-side projections; the typed methods and driver arms are
hand-written — that gap is what this unit closes. The row set is
the POST-DISSOLUTION §2c (OnArc gone — #608), so the table is
smaller than the M2-time estimate (~8 row-shapes, 500-700 macro
lines, ~45 rustdoc-carrying methods were measured PRE-dissolution).
Read first: PATHS-DESIGN §2c rounds 13-15 + the "Shipped form"
note (every constraint there is binding), the `step_vocabulary!`
macro as shipped, `crates/profile/src/path/program.rs` (the
driver arms to derive), `crates/profile/src/path/family.rs` +
`path.rs` (the typed methods to derive).

## 0. Discipline (absolute)

docs/LIB-PYG1-SPEC.md §0 verbatim and binding (foreground builds
one at a time, build slots, no parking + kill-your-own-waiter,
commit+push per chunk, NO Co-Authored-By, no model names,
merge-main-before-open + re-merge on movement, checks STARTED,
cold clippy CI scope both lanes, k-lint discipline, comments
state the INVARIANT). Feature-lane rule (#601 class): before the
PR, slot-wrapped `cargo check --tests --features probe -p
profile` and the pncad-py python lane, plus the local Python
suite.

## 1. Deliverables

1. **The table.** One declaration per transition row; the macro
   expands all four artifacts. Spelling: (a) the table-macro is
   the RULED default; (b) trait-impl rows are permitted ONLY if
   you MEASURE no compile-time cost (state the measurement:
   clean-build wall both spellings, same machine, same slot
   conditions) and it reads cleaner in situ — otherwise (a).
   The entry signatures genuinely differ (typed method vs step
   data), which is why the unification lives at the DECLARATION
   level; the rejected delegation alternative (typed methods
   calling through the driver) stays rejected — no unreachable!()
   where types should speak.
2. **Rustdoc survives.** Every doc comment on a hand-written
   typed method moves into the table and RENDERS on the generated
   method (verify with the doc gate; spot-check rendered output).
   Doc text preserved verbatim in substance.
3. **Drift becomes unwritable, demonstrated.** The executed
   falsification: delete one table row → typed method, driver
   arm, Step variant, and tag all break at COMPILE, consistently
   and loudly; restore. State it in the report as executed.
4. **The tautology retirement.** Per the ratified text, the V2
   drift-proofing differential census retires to ONE smoke row.
   Retire exactly what the doc names — the blanket replay
   differential row that remains meaningful stays.
5. **Zero behavior change.** Pure mechanism: every suite green
   unchanged; the record/replay `pinned` round-trips bit-stable;
   the wire format IDENTICAL (Step variants unchanged — NO
   schema claim; verify main's live SCHEMA_VERSION by eye at
   final re-merge per standing discipline); pncad-py untouched
   (the binding layer calls the same typed methods — if any
   generated signature would differ from the hand-written one,
   STOP and report).
6. **§2c doc note.** The "Shipped form ... WEAKER than the
   four-projection invariant" paragraph is rewritten to record
   the invariant as NOW HELD (state the mechanism in one
   sentence; history stays in git).

## 2. Fence

- NO surface changes: no verbs added/removed/renamed, no
  signature changes, no refusal changes.
- NO schema; NO Python changes; NO entry/seam or geometry work.
- The RESPELL-TABLE register entry closes at merge; nothing else
  from the register rides along.

## 3. Acceptance

1. Hosted matrix green; doc gate green with rendered docs on
   generated methods.
2. The §1.3 falsification executed and stated.
3. `step_vocabulary!`'s interim form fully subsumed (the macro
   either grows into or is replaced by the table — no second
   declaration surface survives).
4. Report ≤150 lines to
   ~/.local/share/cad-work/lib-rtable-report.md: deviations
   enumerated, spelling decision with measurement if (b) was
   considered, the row-count/line-count actuals vs the M2
   estimate, banked findings.

## 4. PR discipline

One PR, branch `lib/rtable`. Merge-main-before-open; re-merge on
movement; checks STARTED before handoff.
