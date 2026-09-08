# GATES exit walk — criteria vs evidence

**Status: DRAFT (2026-09-08), for Ev's ratification.** GATES opened on
2026-09-06 in the tracker-wide cut (`docs/WORK-TRACKS-2026-09.md`,
addendum 2), claiming code-quality Track K's `scripts/gates/*` half:
fourteen rows on opening, twelve of them named in `work/gates/plan.md`'s
unit order, plus the rows the lanes filed on the slate as they found
them. Every unit ran under the S-TCOST posture — one style review per
unit against `docs/prompts/reviewer-style-lane.md`, every review
planting a breach the gate had to catch — followed by a fix pass, a
closing state-sync commit and a merge. The record is
`work/gates/log.md` (the orchestrator's) and each row's `## Landed`
section; this walk quotes the plan's criteria verbatim and says what
happened to each.

The standing rule the charter set — a gate change carries the
before/after hit-set diff on the live tree, and a matcher that widens
counts and grandfathers the population it reds — held in every unit:
every PR below reports its live output byte-identical to the merge
base or says exactly which count moved and why.

## The walk

| # | Criterion (verbatim from plan.md) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | "`trait-generic-sole-bracket` — skip a balanced `<…>` after the trait name; plant the `trait` form." | MET | PR 2029 (2026-09-06). The declaration reader skips the balanced generic list; the `trait` form planted; live 26 files / 183 occurrences unmoved. |
| 2 | "`unanchored-definition-skip` — a `DEFINITION_HOME_RE` anchor on the definition skip, the `no-extra-real-bounds.sh` shape; enumerate the other exact-text skips under `scripts/gates/`." | MET, then superseded | PR 2029 anchored the skip; the enumeration found the same shape hand-spelled in three gates and filed `anchored-exact-text-skip-has-three-homes`, which PR 2064 closed with one `lib.sh` home (`gate_exact_skip`), the record shape read out of the reader and the escaping out of `gate_ere_escape`, a missing home a red in every caller (D103's class). The third "home" turned out to be a pattern-at-a-site-count exemption, not a text skip, and kept its own shape. |
| 3 | "`D211` — the readings nothing re-derives: the interval-square allowlist's "none of these is live" scan gets an executable home; GAP 3's count re-derived or de-counted." | MET | PR 2063. The five-shape scan is a census re-derived each run against a register of dispositioned sites (the `S49` shape); the header's one-shot 21 never reproduced because its pattern was unrecorded (today: 0 + 0 + 1 + 3, the review having caught three adjacent repeats the live matcher already saw). GAP 3's count was gone by then (PR 2056 rewrote it to "grep it"). The backend witness stays disclosed prose with its re-run written out, because the discipline job runs no cargo. |
| 4 | "`D103` — allowlist granularity: line- or symbol-scoped entries, or a pinned per-file count, chosen and applied." | MET (pinned per-file count) | PR 2042. Each `BOUNDS_ALLOWLIST` entry pins the count its file carries, re-derived every run; a file that gains or loses a compound bound reds naming the entry. The pattern spread to every register in the directory (S49's, the alias roster's, the census's). |
| 5 | "`D102` + `bounds-tripwire-blind-to-named-alias` — the matcher redesign (where-clauses, rustfmt multi-line, named compound bounds), with the grandfathering count; the one paragraph in `real.rs` (PROPS', by note) closes the tripwire item." | MET | PR 2056. The matcher reads the statement view and groups constraints by bound target, so `T: A + B`, `where T: A, T: B` and a generic list plus a `where` clause are one hit; measured first as a bijection (163 records → 163, 26 files, 183 occurrences, per-file pins identical — no population to grandfather). The alias census found `EvalScalar` and rostered it; the `real.rs` paragraph landed (PROPS' file, one paragraph, announced). KNOWN GAP 7 registers the shapes the old regex caught that a target-keyed reader is blind to (`dyn A + B`, `-> impl A + B`, type aliases; zero population, with the ERE that shows it). |
| 6 | "`bit-identity-debug-only-gate-ends-an-item-at-a-semicolon` + `debug-only-counters-have-no-gate` — one lane: the awk's `;` branch fixed at depth zero, then the gate generalised to a (subject, symbol) list with `product.rs`'s gather counter as the second subject; wiring announced to CIW." | MET, and the row family it opened landed | PR 2030 (the `;` at depth zero; the subject list). The generalised gate then grew, each step its own row and review: helpers outside the subject list (PR 2049), the statement-position attribute over a braced call — the rule that an item under one has no body brace (PR 2059), the `topo` arena-delta class with both spellings and the gate reading without `cfg(test)` modules (PR 2066), the bit-witness callers and the self-test restructured to run each case once over all subjects, 110 s → 14 s (PR 2069). Subjects 6 → 15, uses scanned 28 → 97; four dropped attributes a consumer's release build refuses are red now and were green. |
| 7 | "`gate-mod-path-resolved-textually` — `#[cfg(test)] mod` resolved the way rustc does; measure the population first; a fixture per direction." | MET | PR 2033 (the resolver, measured: one file moved each way); PR 2058 gave the resolution one home in `lib.sh` (`gate_production_sources`), retired two textual copies that had mis-resolved a module declared from a non-root file, and spelled the test-only `cfg` attribute once. PR 2174 then made the resolver read a record's FILE column by the `:LINE:` the reader emitted, so a colon-carrying declarer is registered too. |
| 8 | "`D109` — the F3 sweep's four blind spots in `lib.sh` and the two roster gates; each member its own small edit, (e) a cost note." | MET | PR 2038. Nested block comments, the `gate_error`-inside-a-substitution case, the roster gates' wiring reads; (e) the hosted-runner cost recorded in the row. The instrumented reading it introduced — which `gate_error` sites fire under some self-test — drove `viewer-module-kinds.sh`'s six unreached guards to zero (PR 2057) and is the reading `scripts/doc-gate.sh` still lacks (filed on CIW's slate). |
| 9 | "`S13` — the greps' known defects (the `x*x` lookahead, `Real +` not stripping comments, the `self.x * self.x` blind spot), then the AST-lint alternative actually evaluated and written down." | MET; the ruling Ev's | PR 2063. All three defects established closed by planted probe (PR 849, 2026-08-21). `scripts/gates/README.md` evaluates `dylint`, `clippy::disallowed_*`, a proc-macro and a `syn` binary against the four grep gates; only `dylint` closes anything real and at a cost the page states. Ev ratified the recommendation (2026-09-06, PR 2067): the four gates stay greps, the alias gap registered where it is disclosed. The page is the first design page outside `crates/<crate>/README.md` and is in DESIGN.md's companion table. |
| 10 | "`clippy-panic-gate-blind-in-macros` — direction chosen by this program (a token-grep gate over `macro_rules!` bodies with a `#[cfg(test)]` allow …), built as `panic-free-macro-bodies.sh` (PR 2032)." | MET | PR 2032, wired by one announced `ci.yml` line. PR 2174 found and fixed its fence reading the record by the first colon, under which a panic token in a macro body in a colon-carrying file was seen by nothing (zero population). |
| 11 | "`S49` — the deferral-register gate over every `LoopBoundary` discard, `probe-suite-census.sh`'s shape; the audit of the 26+15 sites is riders on their owners." | MET (the audit rode) | PR 2044, `loop-boundary-discards.sh`: 80 sites, each registered at its pinned count. The `awk -v` escape-processing hazard it hit is in `memories/`' candidate list, not written (the walk's honesty row 3). Its line-view duplicate filter, dead after PR 2058's window fix, retired there. |
| 12 | "`D212` — rides `G4`; lands with it and not before." | CARRIED | `G4` is code-quality Track V's and has not run; `D212` re-homes beside it (`work/code-quality/`) at the sweep, unchanged. |
| 13 | Exit shape: "The twelve land, Track K's `scripts/gates/*` half is empty; the walk convention applies." | MET-WITH-RECORDED-HONESTY | Eleven of the twelve landed, the twelfth carried by construction (row 12). The slate grew fourteen rows past the twelve — every one a lane's or a review's finding on its own territory, filed at the moment of disclosure per `work/README.md` — and twelve of those landed too (rows 14–15). Two halves stay open at exit (honesty rows 1–2). |
| 14 | Rows the program grew and landed (the reader's homes) | MET | `test-module-resolution-has-three-homes` and `window-view-emits-a-record-for-a-comment-only-line` (PR 2058); `anchored-exact-text-skip-has-three-homes` (2064); `home-anchored-file-skip-is-unescaped` (2065); `whole-file-skips-are-hand-spelled-not-anchored` (2077: twenty-six homes over seven gates through `gate_record_anchor_any`); `whole-file-skips-do-not-check-their-subject` (2156: `gate_require_homes`, membership in the scan set, a missing home a red); `bounds-allowlist-select-cuts-at-the-first-colon` (2157); `record-file-column-read-by-first-colon-split` (2174). Each measured byte-identical live and mutation-proved in both directions. |
| 15 | Rows the program grew and landed (the debug-only gate) | MET | `debug-only-helpers-outside-the-subject-list` (2049); `debug-only-reader-cannot-place-a-statement-attribute-over-a-braced-call` (2059); `debug-only-assert-euler-postcondition-is-on-no-row` (2066); `debug-only-bit-witness-callers-are-on-no-row` (2069); `viewer-module-kinds-six-unreached-guards` (2057). |

## Honesty rows

1. **`directory-prefix-skips-have-no-subject-check` closes by half.**
   PR 2170 proves the two named directory skips in
   `witness-not-ambient.sh` by the file each directory is about (a
   crate root, a module root) — a directory check is satisfied by the
   very file the skip exempts, so the subject is the root. The third
   prefix, `crates/*/src/bin/`, is cargo's convention, and whether a
   convention-class skip needs a subject at all is a ruling: `[ev]` PR
   2171 puts three shapes to Ev with a cost each. The row stays open
   on that half, `blocked_on` 2171, and re-homes with the fence.
2. **`D212` rides `G4`** and was never GATES' to unpark (row 12).
3. **What the program learned and did not write to `memories/`.**
   Three hazards every lane hit and the orchestrator adjudicated: `awk
   -v` processes backslash escapes (pass regexes through `ENVIRON`);
   a refusal's `exit` inside `$(…)` is the substitution's, so a
   terminal refusal writes `GATE_MATCHER_FAILED` and `gate_ok` refuses
   to print over it; and a byte-identity claim across a merge goes
   stale the moment main adds a source file, so it is measured as a
   same-tree differential. All three now live in `lib.sh`'s own
   headers beside the code that holds them, which is where the
   discipline says finished design goes; none earned a memory by
   `memories/cad-working-style.md`'s criteria.
4. **The self-test suite's cost.** The directory's full `--selftest`
   sweep is ~90 s, most of it `bit-identity-debug-only.sh` (14 s after
   PR 2069's restructure, from 110 s) and `bounds-allowlist.sh`
   (~11 s); `gated-suite-paths.sh` reds locally when lanes' worktrees
   sit inside the checkout (it walks `.claude/worktrees/*`), never in
   CI — noted, not fixed, because the fix is a scan-set decision
   (`git ls-files` vs `find`) that belongs to whoever next opens the
   fence.
5. **Two lanes' scratch collided.** Lanes sharing the session
   scratchpad overwrote each other's scripts twice; both noticed and
   nothing wrong shipped. Reviewers were pointed at worktree-local
   scratch from PR 2156 on. Environment, recorded here because it is a
   route for one lane to read another's baseline as its own.
6. **Two waves were logged as dispatched before their lanes ran**
   (2026-09-08, the twelfth and thirteenth); corrected in the log the
   same hour. No work was lost; the entries' "branched from" sentences
   are superseded by the correction.

## Residue re-homed before the sweep

The fence `scripts/gates/*` returns to code-quality Track K, where
GATES claimed it from; the two open halves go with it.

| item | home | why |
|---|---|---|
| `D212` | `work/code-quality/` beside `G4` | rides `G4`, Track V's, unchanged |
| `directory-prefix-skips-have-no-subject-check` (the `src/bin/` half) | `work/code-quality/` (Track K), `blocked_on` `[ev]` PR 2171 | a ruling on a convention-class skip; the two-directory half closed with PR 2170 |
| `work-set-accepts-a-scalar-for-a-list-field` | already on META's slate (PR 2175) | `scripts/work.py` is META's |
| `doc-gate-error-sites-outside-the-gate-population` | already on CIW's slate | `scripts/doc-gate.sh` is CIW's |
| `bound-list-readers-have-three-homes` | already on code-quality's slate | `crates/geom-core/tests/bounds_census.rs` is in no program's fence |

Nothing goes to `work/issues/`.
