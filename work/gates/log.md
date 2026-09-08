# GATES log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/gates/plan.md`.

## Opened (2026-09-06)

Opened in the tracker-wide cut of 2026-09-06 (Ev's direction,
in-chat; `docs/WORK-TRACKS-2026-09.md` addendum 2). Code-quality's
Track K was "this program's to dispatch" and had `smell/k-*` branches
dated 2026-09-03 with no PR on any of them; the letter is claimed
whole, this program taking `scripts/gates/*` and METER the
instruments. Fourteen rows moved by `git mv` with a `## Claimed by`
record each: ten with `track: K` and four unlettered rows on K's
fence. No branch exists yet; the first dispatch is unit 1.

## First wave dispatched (2026-09-06)

Orchestrator's first session, on a remote box (PR subscriptions rather
than the local away-channel monitor). Three lanes on three disjoint
scripts, each in an isolated worktree, implementer briefs pointing at
`docs/prompts/implementer-discipline.md` by path:

- `gates/bounds-small` — `trait-generic-sole-bracket` then
  `unanchored-definition-skip`, one PR, two commits, on
  `bounds-allowlist.sh`; the enumeration of every other exact-text skip
  under `scripts/gates/` rides in the PR body.
- `gates/debug-only-subjects` — the `;`-at-depth-zero parser fix, then
  the (subject, symbol) list with `product.rs`'s gather counter as the
  second subject, on `bit-identity-debug-only.sh`; name and command line
  unchanged so CIW's wiring needs no edit.
- `gates/mod-path-resolution` — population first, then rustc's
  resolution, a fixture per direction, the hit-set diff, on
  `interval-square-allowlist.sh`.

Posture: one style review per PR against
`docs/prompts/reviewer-style-lane.md`, adversarial (a planted breach
each), no A/B row. Standing rule to every lane: a fix that reds live
code stops and reports — no allowlist entry buys green. Log entries are
the orchestrator's; lanes edit only their own item headers.

## Second wave dispatched (2026-09-06)

Two more lanes on files the first wave does not hold:

- `gates/d109-reader-blind-spots` — `D109`'s four members in order,
  on `lib.sh`, `probe-suite-census.sh` and `gate-roster.sh` (the two
  roster gates are this program's now); every gate re-run after each
  member since all of them source the reader.
- `gates/panic-free-macro-bodies` — `clippy-panic-gate-blind-in-macros`,
  direction decided by the orchestrator: the row's option (2), a
  token-grep gate over `macro_rules!` bodies with a `#[cfg(test)]`
  allow, the one-time audit in the PR body, option (4) not taken as a
  convention. The one wiring line in `ci.yml` and `ci-local.sh` is the
  announced CIW line the charter allows.

`S49` waits for the first wave's reviews; `S13`, `D211`, `D103` and
`D102` queue behind `gates/bounds-small` on the same file.

## trait-generic-sole-bracket + unanchored-definition-skip landed (2026-09-06)

PR 2029, `gates/bounds-small`, one style review (MERGEABLE-WITH-FIXES,
fix pass landed). The third matcher alternative became a reader
(`gate_trait_declarations`, one function in two modes for the scan and
the alias census) that skips a balanced `<…>` after the trait name and
consumes `->` as two characters; the definition skip is anchored to
`real.rs` with the at-home passes-case the twin already had. Hit-set
diff on the live tree: 163 records / 26 files, identical before and
after, measured twice by two methods. The review corrected the dispatch
brief (a single-line `where` fires either side; the multi-line block is
`D102`'s gap), found the `->` order dependence and the missing
positive-direction fixture, and named the anchored-skip idiom as
hand-copied three times — filed as
`anchored-exact-text-skip-has-three-homes`. The out-of-fence
`interval-square-allowlist.sh:203` note was withdrawn as vacuous; the
real defect there is PR 2033's.

## bit-identity-debug-only-gate-ends-an-item-at-a-semicolon + debug-only-counters-have-no-gate landed (2026-09-06)

PR 2030, `gates/debug-only-subjects`, one style review
(MERGEABLE-WITH-FIXES, fix pass landed). The awk ends an attribute's
item at a `;` only at round/square bracket depth zero, and a lost
bracket depth is now a loud red (a `DESYNC` record, the
`GATE_MATCHER_FAILED` marker) rather than every later use reading as
gated — the review found that silent direction through the reader's
nested-comment gap. The gate scans a (subject, symbol) list with the
enclosure analysis written once; `product.rs`'s gather counter is the
second subject (1 → 5 uses scanned, 0 ungated). Symbol matches are
anchored on identifier boundaries; 15 cases per subject. The lane
declined one arm of the fix pass correctly (a `;` at open bracket
depth is exactly the signature shape row 1 exists to allow). Filed from
it: `debug-only-helpers-outside-the-subject-list` (five more
candidates), and on their owners' slates the `bits_witness` slice
workaround (TOPO), the `ci.yml` step title (CIW) and the
`landing_gathers.rs` source-text row (S-TCOST).

## clippy-panic-gate-blind-in-macros landed (2026-09-06)

PR 2032, `gates/panic-free-macro-bodies`, one style review
(MERGEABLE-WITH-FIXES, fix pass landed). A new gate,
`panic-free-macro-bodies.sh`: the stanza's six macro-blind lints
(`.unwrap`, `.expect`, `panic!`, `todo!`, `unimplemented!`, `dbg!`)
inside every `macro_rules!` body in the three delimiter forms, through
the code-only view, matched on the body field only; `unreachable!`
excluded because the D2 addendum removed it from the stanza — the
lane's deviation, an improvement. Allow: `#[cfg(test)]` on the item,
an enclosing module, or a file declared by `#[cfg(test)] mod x;` in
either spelling. The one-time audit: 24 bodies in 15 files, two carry a
token (`fit.rs:714`'s sanctioned `unreachable!`,
`review_m1_pr5_internal.rs:116` under test). UFCS `Option::unwrap(v)`
measured as parity with clippy, not a hole. One wiring step in
`ci.yml`, announced to CIW; `ci-local.sh` globs the directory. 18
cases. Its textual `mod` resolver is the third in the directory —
`test-module-resolution-has-three-homes`.

## Third wave dispatched (2026-09-06)

With PR 2029 merged and `bounds-allowlist.sh` free: `gates/d103-pinned-counts`
(`D103`, direction decided by the orchestrator — a per-file compound-bound
count pinned beside each allowlist entry, the `UNCONVERTED_TODAY` shape
per entry, so a file whose count moves in either direction reds and
names the entry's ratification paragraph), and
`gates/loop-boundary-discards` (`S49`, the deferral register as a
derived-census gate keyed by file and enclosing item, two reds — an
unregistered live discard and a registered site that is gone — with
the audited/unaudited counts in the OK line; the audit of the
unaudited sites is the owners' riders). `D211` waits on PRs 2033 and
2038, which hold two of its three files.

## gate-mod-path-resolved-textually landed (2026-09-06)

PR 2033, `gates/mod-path-resolution`, one style review
(MERGEABLE-WITH-FIXES with one MAJOR, fix pass landed).
`interval-square-allowlist.sh` resolves `#[cfg(test)] mod x;` the way
rustc does — `#[path]` wins, roots and `mod.rs` to the sibling, any
other declarer to `dir/foo/x.rs`, an inline `mod x { … }` to no file,
and (the review's MAJOR, implemented rather than refused) a
declaration inside an inline module mounted under that module's
directory; a declaration it cannot place is refused loudly with one
diagnosis, checked before the every-source guard. The exclusion filter
is a whole-path or prefix comparison, no longer a substring match.
Population measured first: 54 gated declarations, 17 outside roots, 12
inline and 5 real; live scan 397 → 395 files, the two leaving being the
test modules their declarations actually name, nothing entering. 27
cases, 11 of 12 mutations killed by their own fixture. The first
fix-pass head went red on the hosted runner only: the reader `exit`ed
with its answer while reading a pipe from `gate_rust_code`, and
`pipefail` reported the shared reader's failed write as a refusal — a
race this box won and the runner lost; fixed by consuming the whole
input, planted with a 1.5 MB view. Reported from it, `lib.sh`'s ground:
`gate_selftest_with_broken_tool` plants only the clean tree (a reader
failure path is unreachable from any selftest), and a stubbed `awk`
exits the gate with status 9 and no output — D109's class.

## Fourth wave dispatched (2026-09-06)

With PR 2030 merged and `bit-identity-debug-only.sh` free:
`gates/debug-only-subjects-2` (`debug-only-helpers-outside-the-subject-list`
— a per-helper decision on the five mesh and topo candidates, rows for
the ones that want a source-shape pin). Behind PR 2038 (`lib.sh`):
`D211`, `test-module-resolution-has-three-homes`,
`window-view-emits-a-record-for-a-comment-only-line`,
`viewer-module-kinds-six-unreached-guards`. Behind PR 2042
(`bounds-allowlist.sh`): `D102`, `S13`,
`anchored-exact-text-skip-has-three-homes`.

## D103 landed (2026-09-06)

PR 2042, `gates/d103-pinned-counts`, one style review
(MERGEABLE-WITH-FIXES, fix pass landed). Every `bounds-allowlist.sh`
entry is `PATH COUNT RULING`, one literal path per file (the
alternation regexes expanded; exclusion set proved identical over a
451-path universe), the count re-derived every run as compound-bound
OCCURRENCES per record — the review showed a record pin took a second
bound on the same line for free — and red in either direction with the
entry's ruling named; malformed entries (no ruling, a non-integer or
duplicate, a path with a space) refused before any scan. Live: 26 files,
163 records, 183 occurrences, survivors 0 both sides. The
`separation.rs` pin of four is stated as three by the M5 PR 8 ruling
plus one resting on `SolidSeparation`'s own doc and owing a ledger row
— filed on PROPS as
`separation-of-fourth-compound-bound-rides-the-module-admission`. The
same-count substitution stays KNOWN GAP 6, a stated cost. Filed from
the review: `pinned-count-re-derived-each-run-has-three-spellings`
(`viewer-module-kinds.sh`'s `FILE|NEEDLE|COUNT`, `reader_census.rs`'s
`UNCONVERTED_TODAY`, and `interval-square-allowlist.sh`'s allowlist
with no pin at all). 38 cases.

## D109 landed (2026-09-06)

PR 2038, `gates/d109-reader-blind-spots`, one style review
(MERGEABLE-WITH-FIXES with one MAJOR, fix pass landed). The shared
reader gained the `code_and_literals` view (`--keep-literals`) and
`probe-suite-census.sh` moved onto it off its column-zero anchor;
nested block comments are lexed with a depth counter that is a state
across lines (the review's MAJOR: the first cut counted an inner
opener only when its closer sat on the same line); the reader's other
blind spots are stated with their direction — `macro_rules!` and
`include!` bodies, non-`test` `#[cfg]`, an unterminated `/*` at end of
file, an awk that truncates with exit 0; an awk that dies is a loud
red, and the broken-tool selftest can now plant a tree so a reader
failure is reachable. Twelve `gate_error` sites were unreached at the
merge base (the row said six of 82; the lane measured 12 of 105 with
message-fragment scoring, since `BASH_LINENO` inside a command
substitution names the enclosing call): six planted in the two roster
gates, six in `viewer-module-kinds.sh` filed as
`viewer-module-kinds-six-unreached-guards`. Byte identity of every view
over every source file, before and after, under two awk
implementations. Cost: the census real pass 0.29 → 2.2 s and its modes
similarly, no job carrying a timeout. The nesting fix falsified PR
2030's desync fixture (a stray `(` planted inside a nested comment) —
the Q4 shape — so the desync arms are now proved by planting the stray
bracket as code, and `panic-free-macro-bodies.sh`'s claim that a brace
inside a nested comment reaches its tracker was corrected. Filed from
it: `window-view-emits-a-record-for-a-comment-only-line`.

## Sixth wave dispatched (2026-09-06)

With PR 2038 merged and `lib.sh` free: `gates/reader-homes`
(`test-module-resolution-has-three-homes` and
`window-view-emits-a-record-for-a-comment-only-line`, one lane because
both are `lib.sh` mechanisms — the rustc-correct test-module resolver
and an anchored path filter moved from `interval-square-allowlist.sh`
into the shared reader and the two textual copies retired; the
comment-only-line record fixed at the reader with the consumer's filter
retired), and `gates/viewer-module-kinds-guards`
(`viewer-module-kinds-six-unreached-guards`, fixtures for the six).
PR 2044 waits on its run; PR 2049 merges main behind 2038; `D102` is
in flight on `bounds-allowlist.sh`, with `D211`, `S13` and
`anchored-exact-text-skip-has-three-homes` behind it.

## S49 landed (2026-09-06)

PR 2044, `gates/loop-boundary-discards`, one style review
(MERGEABLE-WITH-FIXES with two MAJORs, fix pass landed). A new
derived-census gate, `loop-boundary-discards.sh`: every discarded
`LoopBoundary::Cycle`/`Empty` under `crates/*/src` — let-else with
`continue`/`break`/`return`, including one wrapping pattern layer
(`Some(…)`, the review's second MAJOR: three live sites missed), and
no-binding match arms in the spellings that compile — must match a
register entry `<file>|<fn>|<fragment>|<count>|<disposition>`, keyed by
the brace-aware ENCLOSING fn (the review found one live key naming a
neighbouring helper) and pinned at a count so a second discard in a
registered fn reds rather than inheriting its audit (the first MAJOR,
D103's shape one level down). 80 sites in 40 files under 75 entries, 2
marked audited (`census.rs`'s `sweep_cross_solid_backstop`, split by
fragment) and 73 unaudited — the audit is the owners' riders; the
`snapshot` discard is filed on TOPO. `#[cfg(test)]` skipped. The runner
red on the fix pass was `awk -v` processing backslash escapes in the
regexes (mawk tolerated the mangled pattern; the runner's awk died) —
the hazard PR 2030's header names; the patterns now travel through
`ENVIRON` with every metacharacter a bracket expression. The reader
artifact it found — a `--window` record for a comment-only line — is
`window-view-emits-a-record-for-a-comment-only-line`.

## debug-only-helpers-outside-the-subject-list landed (2026-09-06)

PR 2049, `gates/debug-only-subjects-2`, one style review
(MERGEABLE-WITH-FIXES, fix pass landed). Four of the five candidates
got rows — mesh's `identified_ids`, `overused_identified_edge`,
`unpaired_chord_segment` and `overused_identified_edge_in` (a row per
file whose uses it crosses) — taking the gate from 5 to 28 uses
scanned, none ungated. `ArenaDelta` was refused: twelve of its sites
put a statement-position attribute over a multi-line braced call the
reader cannot place — filed as
`debug-only-reader-cannot-place-a-statement-attribute-over-a-braced-call`.
From the review: every spelling of a row is now planted (the third
field was the only one proved), and every row pins its use count, the
program's shape since PR 2042, proved by perturbing the pin in both
directions; two miscounted sentences in the row corrected. 98 gate
invocations in the selftest.

## Seventh wave dispatched (2026-09-06)

With PR 2049 merged and `bit-identity-debug-only.sh` free:
`gates/statement-attribute-items`
(`debug-only-reader-cannot-place-a-statement-attribute-over-a-braced-call`
— the rule that an item under a statement-position attribute has no
body brace, then the `ArenaDelta` rows the previous lane could not
add). In flight: PR 2056 (`D102`) under review; `gates/reader-homes`
and `gates/viewer-module-kinds-guards` building. Landed today so far:
ten rows over eight PRs (2029, 2030, 2032, 2033, 2038, 2042, 2044,
2049).

## PR 2056 reviewed (2026-09-06)

Style review of `D102` + `bounds-tripwire-blind-to-named-alias`:
mergeable with fixes. The bijection holds (183 old line-records map
one-to-one onto 183 statement records; per-file pins identical), the
four silent spellings fire and the three near misses do not, the
mutations flip exactly the fixtures that guard them. Findings, all
minor, sent back as a fix pass: shapes `main`'s regex caught that the
parameter-keyed reader is silent on (`dyn A + B`, `-> impl A + B`,
`type S = dyn A + B;`, lifetimes, `;` inside a generic list — zero
population, undisclosed at the claim site); `?Sized` and a lifetime
treated oppositely; the reader's own paragraph carried over from the
line reader and false under statements; the `EvalScalar` roster entry
saying its uses are unchecked when `evalscalar-allowlist.sh` checks
them (and that gate's header listing six supertraits of ten); a stale
example in the `D102` row; the `real.rs` paragraph's "ratified here";
three stale comments and a duplicated header sentence; the OK line's
count wording. Out of fence, filed: three bracket-depth bound-list
readers (`bounds-allowlist.sh`'s awk, `bounds_census.rs`, `test_utils`)
none citing the others — `code-quality/bound-list-readers-have-three-homes`.

## PR 2057 reviewed (2026-09-06)

Style review of `viewer-module-kinds-six-unreached-guards`: mergeable.
The reviewer reproduced the reading independently (instrumented
`gate_error` with `BASH_SOURCE`/`BASH_LINENO`, all 18 self-tests, 113
sites in 19 files): base unreached exactly the row's six, head zero;
each of the six mutations reds on its own case only; live output
byte-identical; fence clean. Style findings sent back as a fix pass,
same class as the "check 7" message the PR corrected: the header's
check numbering (`:22-28`) has never matched the banners; check 3's
message quotes a README sentence ("exactly two drivers") that no
longer exists; the moved guard's "only place it can be answered"
overstates its constraint; the forbidden-path guard prevents a
wrong-file red, not a vacuous pass, and its planter says vacuity;
one ordering-dependent sentence in the fixture prose.

## PR 2058 opened (2026-09-06)

`gates/reader-homes` reported: PR 2058 (the resolver, its path filter
and the window view's comment-line record with one home in `lib.sh`).
Ten reader views over 433 files byte-identical across the resolver
move under mawk and gawk; the window fix moves no gate's output; two
textual gates' scanned set corrected by one file each way (a test
module declared from a non-root file that the sibling rule missed).
Sent back before review: the branch predates PR 2044's merge, so the
dead line-view filter it measured in `loop-boundary-discards.sh` is
retired in this PR rather than filed as residue. Reviewer dispatched
once that lands.

## PR 2059 opened (2026-09-06)

`gates/statement-attribute-items` reported: PR 2059 (the
statement-attribute rule — an item under one has no body brace — and
the seven `ArenaDelta` rows PR 2049 could not add; uses scanned 28 →
68, the body-brace desync arm deleted). Residue filed on this slate:
`assert_euler_postcondition` on no row. Reviewer dispatched. Note for
the record: the row it closes was created in this PR — PR 2049
disclosed it in prose and the file never reached `main`.
## Landed: PR 2057 (2026-09-06)

`viewer-module-kinds-six-unreached-guards` closed. The six `gate_error`
sites in `viewer-module-kinds.sh` that D109's reading found no
self-test fires (`:251 :263 :383 :427 :459 :464`) each have a `--root`
fixture now, mutation-proven one at a time (backing out a guard reds
exactly its case); directory-wide unreached count 6 → 0 over a
population of 113 sites in 19 files. The empty-vocabulary guard moved
from below check 4 (unreachable there: check 4 exits on empty tables)
to directly under check 1, with the position stated as load-bearing;
one message that named check 6 for check 7's path arm corrected; the
self-test's stale closing summary rewritten. Live run byte-identical.
Out of fence: `scripts/doc-gate.sh` sources `lib.sh` and carries 14
`gate_error` sites the population never reads — filed on CIW's slate
as `doc-gate-error-sites-outside-the-gate-population`.
Fix pass from the review: the header's check numbering (never in step
with the banners since the gate was written) corrected and every check
citation swept; check 3's message reads the driver count from the
roster it built instead of quoting a README sentence that no longer
exists; the moved guard's comment states its constraint (above check
4's exit) rather than "the only place"; the forbidden-path guard is
described as preventing a wrong-file red, not a vacuous pass, at the
guard and its planter; one ordering-dependent sentence in the
self-test prose replaced by the planters' names.


## PR 2059 reviewed (2026-09-06)

Style review of
`debug-only-reader-cannot-place-a-statement-attribute-over-a-braced-call`:
mergeable. The rule held on every probe (a `{ N }` const-generic default
still cry-wolf, KNOWN GAP 5, as stated), the seven pins re-derived, the
residue reproduced (the gate stays green with `voids.rs:313`'s
attribute dropped), all three fixtures bite. Self-test 15 s → 72 s
confirmed and shown quadratic in the subject count. Sent back as a fix
pass: the residue row records one instance of a fifteen-site class
(fourteen gated `let before = self.arena_counts();` sites are invisible
on the same terms), the PR body's mutation-D attribution, one dead
file-change desync arm, counts copied into prose, and the rule spelled
in five places.

## Landed: PR 2056 (2026-09-06)

`D102` and `bounds-tripwire-blind-to-named-alias` closed.
`bounds-allowlist.sh` matches a compound bound by bound target across a
whole statement (`lib.sh`'s statement view), not by a plus sign:
`T: A + B`, `where T: A, T: B` and a generic list plus a `where` clause
are one hit; a sole bracket bound is none; a lifetime and a relaxed
`?Trait` bound are not bound terms. The trait-declaration test stays a
separate reason to fire, and the pinned occurrence count is the same
reader's count mode, so it cannot drift from the matcher. Measured
before wiring: the old and new hit sets are a bijection (163 records,
26 files, 183 occurrences, per-file pins identical). The alias census
found `EvalScalar` (ten supertraits under one name, declared as a
multi-line list the line reader could not see) and rostered it against
the ratification its doc cites; the census now consumes the scan's own
records. `real.rs`'s Bounds scope rule gained one paragraph (PROPS'
file, announced): a named compound bound is the same obligation as the
literal spelling, ratified at whichever home the ruling has. KNOWN GAP
7 registers the shapes the old regex caught that the target-keyed reader
is blind to (`dyn A + B`, `-> impl A + B`, type aliases, `;` inside a
generic list — zero population, with the ERE that shows it), and GAP 3
narrows the S63 argument to `ArcCarrierScalar` now that
`evalscalar-allowlist.sh` is the counterexample for the other name.
Review: mergeable with fixes; thirteen items taken in a fix pass, the
two behaviour ones mutation-proved in both directions.
## PR 2058 re-based (2026-09-06)

`gates/reader-homes` merged main and retired
`loop-boundary-discards.sh`'s line-view anchor filter in-PR (four-cell
measurement: the filter was doing exactly what the window fix does and
nothing else; 80 sites green with it gone; the anchor test stays to
name the enclosing `fn`). Two corrections the re-verification forced,
both recorded in the PR: the statement views are not byte-identical
across the window fix — every record's text is unchanged but 12,266 of
113,112 move their reported line from a doc comment to the code line
under it (the same defect in a third view; `lib.sh`'s header now says
what LINE means); and the `bit-identity-debug-only.sh` rider the first
report named does not exist since PR 2049 rewrote that gate. Reviewer
dispatched on head `d1af6dece`.
## Landed: PR 2059 (2026-09-06)

`debug-only-reader-cannot-place-a-statement-attribute-over-a-braced-call`
closed. The rule in `bit-identity-debug-only.sh`'s reader: an item
under a statement-position attribute has no body brace, so whichever
delimiter arrives first at bracket depth zero says what the item is
(`{` there is the item entering, `;` there ends an item never entered,
`{` above depth zero is argument text and moves brace depth only). The
body-brace desync arm is deleted; a still-open bracket reports only at
end of file, and both re-planted desync fixtures red through that arm.
Seven `ArenaDelta` rows added with measured pins (euler 10, euler_ring
10, euler_kill 9, null 3, split 3, boolean/voids 1, movefac 4): no live
use ungated; subjects 6 → 13, uses scanned 28 → 68, the eight-file
probe 3 placed / 12 desync → 8 placed / 0 desync. Three fixtures, four
mutations each caught by the fixture named (the one-shot brace skip
only by the nested-braces one). Self-test 15 s → 72 s, stated. Residue
filed on this slate: `assert_euler_postcondition` is a second spelling
of the mechanism on no row, and one use (`boolean/voids.rs:314`,
passing a binding) is invisible to the voids row —
`debug-only-assert-euler-postcondition-is-on-no-row`.
Fix pass from the review: the residue row rewritten as its class
(fifteen gated `topo` statements naming no pinned spelling, the
fourteen `let before = self.arena_counts();` sites beside the one
`assert_euler_postcondition` binding; candidate fix stated for the
class, self-test cost stated as quadratic); the dead file-change
desync arm deleted so the end-of-file arm is the one place the
question is asked; two counts copied into prose removed; the rule
spelled once in the header with the awk comment a pointer to it; the
PR body's mutation-D attribution corrected (caught first by
`plant_desync_open_bracket`, then the nested fixture, then the live
tree through the pin self-test).


## PR 2058 reviewed (2026-09-06)

Style review of `test-module-resolution-has-three-homes` +
`window-view-emits-a-record-for-a-comment-only-line`: mergeable. File
sets, the `all(test, …)` plant, the record counts (68,792 line records
gone, 12,266 statement lines moved, text unchanged), the six mutations
and the mawk/gawk identity all reproduced against the merge-base. One
finding with teeth, sent back as a fix pass: the widened raw narrowing
(the `all(test, …)` first-stage fix) had no fixture — reverting it to
the literal left every self-test green. Also: the rustc rule restated
in prose at two consumers and the not-test-only rule at a third; a
stale planted comment; the two-dialect constraint on the shared
regexes unstated; the filter helper's mounts arriving as a global
while its sibling takes arguments.
## Landed: PR 2058 (2026-09-06)

`test-module-resolution-has-three-homes` and
`window-view-emits-a-record-for-a-comment-only-line` closed. rustc's
test-module resolution (`#[path]`, root/`mod.rs` to the sibling, other
declarers into `dir/foo/`, inline modules to no file) lives once in
`lib.sh` as `gate_test_only_mounts` / `gate_filter_test_only_paths` /
`gate_production_sources`, with the test-only `cfg` attribute spelled
once (`GATE_CFG_TEST_RE`) and read by the reader's skip, the resolver
and the raw narrowing (which was a fixed `#[cfg(test)]` string and now
closes the `all(test, …)` hole). Retired: `witness-not-ambient.sh`'s
and `panic-free-macro-bodies.sh`'s textual copies and
`interval-square-allowlist.sh`'s `production_sources`. One file moved
in the two textual gates: `boolean/solid_contain/r1_generic_poses.rs`,
declared from a non-root file, whose sibling-rule exclusion named a
path that does not exist — it was scanned as production; both gates
now scan 396 files, not 433 with records filtered after. The window
view starts a window only at a code line (a comment-only line is not
a record in any shape); statement views byte-identical, no gate's
output moves. PR 2033's resolver fixtures became
`gate_selftest_test_module_homes`, run by every caller of
`gate_production_sources`, because deleting a gate's textual copy
outright had left its self-test green. `loop-boundary-discards.sh`'s
line-view duplicate filter measured dead after the window fix and
retired in the same PR. Delimiter walkers left at three, stated as a
cost: a depth column serves two of three callers and not the one that
needs spans.
Fix pass from the review: the widened raw narrowing (`all(test, …)` in
the first stage) now has a must-NOT-fire fixture run by every caller
of `gate_production_sources`, mutation-proved on all three gates
(reverting the narrowing to the literal reds exactly that case in each);
the rustc rule and the test-only rule restated at three consumers
became pointers to `lib.sh`'s clauses; a stale planted comment fixed;
the two-dialect (grep-ERE and awk-ERE) constraint stated at the shared
regexes; the filter helper's global mount list named in its doc with
the reason the shape differs from its sibling. Gate loop green under
both awks on the merged head; scanned sets unmoved.


## Eighth wave dispatched (2026-09-06)

With PR 2056 merged and PR 2058 verified and closing, the last two
code lanes on the slate, both branched from `gates/reader-homes`'s
head so they build on `lib.sh` as PR 2058 leaves it:
`gates/anchored-skip` (`anchored-exact-text-skip-has-three-homes` —
one `lib.sh` helper for the anchored exact-text skip, the three gates
calling it, the subject-check rule decided) and
`gates/readings-re-derived` (`D211` — the interval-square gate's
five-shape scan and its 24 dispositions re-derived each run; `S13` —
each named grep defect probed against today's tree and the
greps-vs-lints evaluation written into `scripts/gates/README.md`,
with any lint-replaces-gate conclusion routed to Ev rather than
ruled). After these, `D212` remains, parked on `G4`.

## PR 2063 opened (2026-09-06)

`gates/readings-re-derived` reported: PR 2063 (`D211` + `S13`). The
interval-square gate's five-shape scan is a census re-derived each run
against a register of dispositioned sites; the header's one-shot 21
never reproduced because its pattern was unrecorded (today's tree
reads 0 + 0 + 4 + 3 over 84,513 statements, every candidate in one of
the three disposition classes the prose named). The backend witness
stays disclosed prose with the re-run written out as a test and a
pointer to the floor's home (no cargo in the discipline job). S13's
three named grep defects all closed by PR 849, each shown by probe.
`scripts/gates/README.md` written: the greps stay with their gaps
registered; `dylint` is the one candidate that closes anything real and
is not worth a nightly pin, a compile step in a cargo-free job and a
hole in roster parity — no lint replaces a gate, nothing for Ev.
Outside fence, noted for the exit walk: the new README is not yet in
`docs/DESIGN.md`'s companion table. Reviewer dispatched.

## PR 2064 opened (2026-09-06)

`gates/anchored-skip` reported: PR 2064
(`anchored-exact-text-skip-has-three-homes`). One `lib.sh` helper
family builds the anchored exact-text skip from the plain text — the
record shape read out of `gate_rust_code` in the caller's view, the
escaping out of `gate_ere_escape` — and `bounds-allowlist.sh` and
`no-extra-real-bounds.sh` call it; `viewer-module-kinds.sh` turned out
not to be a third home (its exemption is a pattern at a site count)
and takes only the anchor builder, which also escapes three
interpolations that were not. Subject-check rule decided: a missing
home is a red in every caller (D103's class — a skip whose home is
gone is a ratification the next file at that path inherits), each
clean fixture planting the home. Four gates byte-identical live.
Residue filed on this slate: `signed-zero-one-home.sh`'s
home-anchored whole-file skip is unescaped. Reviewer dispatched.

## Ninth wave dispatched (2026-09-06)

The two residue rows this week's lanes filed on their own slate, both
on files no open PR touches: `gates/file-skip-anchor`
(`home-anchored-file-skip-is-unescaped` — `signed-zero-one-home.sh`'s
whole-file skip through `gate_record_anchor`, branched from PR 2064's
head for the helper, plus the sweep for any other interpolated-path
skip) and `gates/debug-only-topo-class`
(`debug-only-assert-euler-postcondition-is-on-no-row` — both spellings
on the seven `topo` rows, pins re-taken, the self-test's quadratic cost
measured). In review: PR 2063 (`D211` + `S13`), PR 2064 (the anchored
skip).

## PR 2063 reviewed (2026-09-06)

Style review of `D211` + `S13`: mergeable, fix pass owed. The census
re-derived to the same sites and the S13 probes reproduced. Findings
sent back: the three-factor shape counts adjacent repeats the live
matcher already sees (three of the four candidates), so the OK line's
"cannot see" is false for them and the census quietly re-granulates
KNOWN GAP 4 for one spelling; two self-test `want`s satisfied by any
census red (the umbrella diagnosis carries both fragments), so
removing the ABSENT check stays green; the two-statement entry's
MISCOUNT/ABSENT paths under no fixture; the header's written-out
witness test names geom-core's API in a crate that cannot depend on
it; a bare `--register` dies silently; a count copied into the row.
On the README: two passages answer Ev's commissioned question as
settled — the dylint cost judgement becomes a recommendation, the
policy sentence comes out, and ratification rides a separate `[ev]`
PR that also lists the page in DESIGN.md's companion table.

## PR 2064 reviewed (2026-09-06)

Style review of `anchored-exact-text-skip-has-three-homes`: mergeable.
The helper's record derivation, the escaping (every ERE metacharacter
round-tripped), the missing-home rule held by `gate_selftest_clean`
rather than convention, the dead-grep distinction and all eight
mutations reproduced; four gates byte-identical. Fix pass sent: the
two-record refusal's `exit` is lost inside a nested substitution so
the gate continues past its own diagnosis (and no fixture reaches it);
the path-escaping half is unreachable by construction for every gate
scanning `*.rs` (only the extension dot is a metacharacter), which the
comments, the residue row and the PR body overstate as a live
widening — the `gates/file-skip-anchor` lane told the same, its
fixture re-aimed at the anchor's `^` and `:` shape; history in six
comments; the generic record prefix hand-spelled in five places beside
the new anchor builder; a stale D103 path in `viewer-module-kinds.sh`.
## Landed: PR 2063 (2026-09-06)

`D211` and `S13` closed. `interval-square-allowlist.sh` re-derives on
every run the census its header used to transcribe: the five square
shapes the matcher structurally cannot see are counted over the
statement view (the two-statement form by following one binding hop
within a file), each candidate must match a register entry pinning its
count, and an unregistered candidate, a moved count, a vanished site
or a malformed pin reds — the `S49` shape. The header's 21 was never
reproducible (pattern unrecorded); today's reading is 0 + 0 + 1 + 3
over 84,513 production statements in 396 files, the counts re-derived
and the prose pointing. The backend witness (`2^-481*1.5`) stays
disclosed prose with its re-run written out as a test and a pointer to
`TWO_PROD_VALID_MIN`'s home, because a cargo-free discipline job cannot
compile it. S13's three grep defects (the `x*x` lookahead, `Real +`
not stripping comments, `self.x * self.x`) were all closed by PR 849,
each re-established by probe. `scripts/gates/README.md` carries the
greps-vs-lints evaluation: `clippy::disallowed_*`, a proc-macro and a
`syn` binary close nothing the statement view does not; `dylint` closes
the alias/`include!`/macro-body cases at the cost of a nightly
`rustc_private` pin, a compile step in the discipline job and a hole
in roster parity — the greps stay, gaps registered, no ruling needed.
Fix pass from the review: the three-factor shape now requires the
wedged factor to differ from the operand, so the three adjacent
repeats the live matcher already sees left the census (4 register
entries, not 7) and an adjacent square in an allowlisted file cannot
arrive as unregistered; every self-test `want` is built from the
register entry itself, and the three register mutations run for a
binding-hop entry as well as a three-factor one; both halves of the
(file, shape) key have a fixture; the witness test rewritten in
`interval-transcendentals`' own API and compiled once; a bare
`--register` diagnosed; the README's dylint cost judgement made an
explicit recommendation and its unratified standing rule removed —
ratification rides PR 2067 (`[ev]`, one line: the DESIGN.md
companion-table row for `scripts/gates/README.md`), which waits for Ev.
## PR 2066 opened (2026-09-06)

`gates/debug-only-topo-class` reported: PR 2066
(`debug-only-assert-euler-postcondition-is-on-no-row`). Both spellings
on the seven `topo` rows, pins re-taken with the gate's own matcher
(68 → 105 uses scanned); the two dropped attributes green before and
red after; `source.rs`'s row gained the three bit-witness heads on the
same reading (green before, red after); the near-miss fixture split so
each anchor is proved alone. Self-test 68 s → 82 s, linear in
spellings as the row predicted. The class sweep leaves `ArenaCounts`
off (named in a `cfg(test)` module the reader reads as ungated code)
and three redundant attributes over `debug_assert_eq!`. Residue filed
on this slate: two statement-position attributes over bit-witness
calls in files with no row. Reviewer dispatched.
## Landed: PR 2064 (2026-09-06)

`anchored-exact-text-skip-has-three-homes` closed. `lib.sh` has
`gate_exact_skip` and its family: the anchored filter, the subject
check and the planted cases (at home passes; elsewhere fires; home
present but text gone reds; home gone reds) are built from the plain
text once — the record shape read out of `gate_rust_code` in the
caller's declared view, the escaping out of `gate_ere_escape` — so no
hand-escaped `_RE` twin exists to drift. `bounds-allowlist.sh` and
`no-extra-real-bounds.sh` call it and their copies and four subsumed
fixtures are deleted. The rule for a missing home is a red in every
caller: a skip whose home is gone exempts nothing today and is a
ratification the next file written at that path inherits without
argument, and the abstention's defence covered a moved home but not a
deleted or mistyped one. `viewer-module-kinds.sh` is not a third home
— its exemption is a pattern granted at an exact site count in both
directions — and takes only `gate_record_anchor`, which escapes the
three interpolations that were not (`forms.rs` matched `formsXrs`).
Off the live tree: the subject check no longer reads a dead `grep`
as "text gone". Four gates byte-identical live; a metacharacter text
planted deliberately. Residue on this slate:
`home-anchored-file-skip-is-unescaped` (`signed-zero-one-home.sh`).
Fix pass from the review: the one-record refusal ends the gate (the
record and the pattern are computed in their own statements, so the
refusal's exit is not lost inside a nested substitution — proved by a
fixture that runs the builder in a subprocess and reds if anything
runs past the refusal); the path-escaping half stated as construction
rather than a live widening (every scan set is `*.rs`, so only the
extension dot is a metacharacter and no sibling path is ever read);
the generic record prefix named once (`GATE_RECORD_PREFIX_RE`) and
read at its five sites; the pattern builders take their inputs as
arguments; six comments to present tense; a stale D103 path in
`viewer-module-kinds.sh` fixed. Five gates byte-identical live.


## PR 2066 reviewed (2026-09-06)

Style review of `debug-only-assert-euler-postcondition-is-on-no-row`:
mergeable. Every pin re-derived, the three dropped attributes green
before and red after, the near-miss mutations reproduced (and the old
one-line fixture shown to hold nothing under all three), the residue
shown to be a real consumer build break (`cargo check -p topo
--release` with debug assertions off fails on `plane_bits_witness`).
One finding changes the row's reasoning, sent back as a fix pass: the
reader already has `--skip-cfg-test` and this gate reads without it,
so `ArenaCounts` "cannot be served" was a choice, not a limit — the
gate switches to the skip (test code is what a consumer never
compiles), `ArenaCounts` joins `euler.rs`'s row, two pins re-baseline
(`curved.rs`, `tessellate.rs`). Also: header history, one fact in
three homes, the near-miss case planted per spelling rather than per
row, the pin harness's message when a gate reds with a different
count.
## Landed: PR 2066 (2026-09-06)

`debug-only-assert-euler-postcondition-is-on-no-row` closed. The seven
`topo` arena-delta rows in `bit-identity-debug-only.sh` pin
`assert_euler_postcondition` and `arena_counts` beside `ArenaDelta`,
pins re-taken with the gate's own matcher over its code view (the pin
is one use per delimiter-cut piece, so a one-line head naming two
spellings counts once); dropping the attribute at `voids.rs:313` or at
a `let before = self.arena_counts()` site was green before and is red
now. `source.rs`'s row gained `plane_bits_witness|vec3_bits_witness|
bits_witness` on the same reading. The near-miss fixture plants each
identifier anchor alone plus the CamelCase and SCREAMING_SNAKE forms,
each proved by mutation. Self-test 68 s → 87 s, linear in spellings.
Left open with a reason: three attributes over `debug_assert_eq!`
that compile out anyway. Residue on this slate:
`debug-only-bit-witness-callers-are-on-no-row` (`plane_eq.rs:173`,
`merge_faces.rs:1006`).
Fix pass from the review: the gate reads with `--skip-cfg-test` (the
reader has had it since PR 2058; a `cfg(test)` module is exactly what
a consumer's release build never compiles), so `ArenaCounts` joins
`euler.rs`'s row (pin 18 → 21; dropping the attribute at `euler.rs:246`
was green and is red) and two pins re-baseline because their test
modules carried gated uses — `mesh/curved.rs` 13 → 5,
`mesh/tessellate.rs` 8 → 2; live uses 105 → 94. `source.rs` named as
the third row whose pin is not the column sum. Header to present
tense; the near-miss case planted per spelling with derived forms
that are themselves a spelling dropped; the pin harness's message now
separates "did not red" from "red with a stale number"; the residue
row carries the consumer-build reproduction (`cargo check -p topo
--release` with debug assertions off fails on `plane_bits_witness`).


## Tenth wave dispatched (2026-09-06)

`gates/bit-witness-callers`
(`debug-only-bit-witness-callers-are-on-no-row` — rows for the two
caller files of the bit-channel witnesses, and the self-test loop
restructured to run each case once over all subjects if that is
mechanical, since 13 subjects cost 87 s and two more would cost 115),
branched from PR 2066's closing head. In flight: PR 2066 closing;
`gates/file-skip-anchor` building; PR 2067 (`[ev]`) waiting for Ev.

## Ev's ruling on PR 2067 (2026-09-06)

Ev, in chat: "2067 is fine, the gates can stay greps." The DESIGN.md
companion-table row for `scripts/gates/README.md` and the README's own
recommendation passage record the ratification (Ev, 2026-09-06, PR
2067) and PR 2067 merges. The standing rule the fix pass removed from
the README stays out — not asked for, not ratified.

## PR 2065 opened (2026-09-06)

`gates/file-skip-anchor` reported: PR 2065
(`home-anchored-file-skip-is-unescaped`). `signed-zero-one-home.sh`'s
whole-file skip goes through `gate_record_anchor`; live output
byte-identical. The row's premise corrected by probe in both
directions: a `signed_zeroXrs` sibling is never scanned, but a path
carrying a `:` inside it (`signed_zero_rs:9:x.rs`) is reachable and
was exempt — narrow, not empty. Three fixtures, one per part of the
anchor; the `^` had no witness in any gate before this. Sweep: no
other gate interpolates a path into an ERE; the hand-escaped literal
whole-file skips (thirteen lines naming eighteen homes, six gates) filed as
`whole-file-skips-are-hand-spelled-not-anchored`. One `lib.sh`
comment paragraph from PR 2064 corrected. Reviewer dispatched.

## PR 2065 reviewed (2026-09-06)

Style review of `home-anchored-file-skip-is-unescaped`: fix pass
needed. The colon-path exemption, the byte-identity and the `^`
witness (absent in all 19 gates before) reproduced. Findings sent
back: the third fixture holds the trailing `:` the old spelling
already had, not the `[0-9]+` it names (the discriminating path is
`signed_zero.rs:x.rs`, one more member of the colon-carrying set);
the over-claim corrected in `gate_record_anchor`'s header survives in
the escaping case's header 1,050 lines below; the residue row's
"fifteen" is 13 lines naming 18 homes; a one-grep measurement
deferred as unmeasured; the reachability argument spelled in four
places.

## PR 2069 opened (2026-09-06)

`gates/bit-witness-callers` reported: PR 2069
(`debug-only-bit-witness-callers-are-on-no-row`). Rows for
`boolean/plane_eq.rs` and `merge_faces.rs` with the witness spellings
each file names (pins 1 and 2; live 94 → 97 uses); both dropped
attributes green before, red after; no caller outside `crates/topo/src`
repo-wide. The self-test loop restructured as the row proposed: each
case plants every subject in one tree and runs the gate once,
asserting per subject — 110 s → 14 s over 15 subjects, with one
strengthening (a gate that stops at its first failing subject now
reds, which the per-subject form could not see). Reviewer dispatched.

## Eleventh wave dispatched (2026-09-06)

`gates/whole-file-skips` (`whole-file-skips-are-hand-spelled-not-anchored`
— the thirteen hand-escaped literal whole-file skips naming eighteen
homes in six gates go through `gate_record_anchor`, each gate's clean
fixture planting its homes and each gate getting the colon-path case;
`gate-roster.sh`'s dot-only escape through `gate_ere_escape`), from
main, on files no open PR touches. In review: PR 2069; in fix pass:
PR 2065. With these three, the slate is `D212` alone, parked on `G4`.

## PR 2069 reviewed (2026-09-06)

Style review of `debug-only-bit-witness-callers-are-on-no-row`:
mergeable. Every claim re-executed: both pins, both before/after
attributes, the (case, subject) outcome set enumerated on both sides
with the two harness falsifications (a subject that does not red is
noticed; a right-path wrong-text line is noticed), all 38 (row,
spelling) pairs planted, all eight mutations red, the strengthening
confirmed against the merge-base harness (a gate that stops at its
first failing subject passed there). Fix pass sent for style: the
fixtures header describing the deleted loop, a subject count in a
comment beside the line that derives it, history in the loop's
comments, the pass twin's message naming no index, four spellings of
"does the output carry a line naming this row and text", a substring
assumption on subject paths, an incomplete hit list.

## Landed: PR 2065 (2026-09-06)

`home-anchored-file-skip-is-unescaped` closed. `signed-zero-one-home.sh`'s
whole-file skip is built by `gate_record_anchor` (escaped path, the
`FILE:LINE:` shape pinned); live output byte-identical. The row's
premise was corrected by probe in both directions: a sibling differing
at the extension dot is never scanned (`*.rs`), so the escaping half
is construction; but a path carrying a `:` inside it after the home
(`signed_zero.rs:x.rs`) is legal, reachable, and was exempt — the
anchor's `[0-9]+` is what refuses it. Three fixtures, one per part of
the anchor, each red under exactly its own mutation; the `^` had no
witness in any of the 19 gates before this. The reachability argument
has one home, `gate_record_anchor`'s header. The class the sweep
found — every other whole-file skip is a hand-escaped literal, none
pinning the shape, none checking its home exists — is on the slate as
`whole-file-skips-are-hand-spelled-not-anchored`, defined by its grep
rather than a count.


## Stall (2026-09-06 15:57 – 2026-09-08)

The orchestrator session was suspended after PR 2065's merge with two
PRs open and green: PR 2069 (fix pass landed, awaiting close) and
PR 2077 (`gates/whole-file-skips`, reported and awaiting review). Both
resumed on 2026-09-08; main moved by other programs' PRs in between,
none touching `scripts/gates/*`.

## PR 2077 opened (2026-09-06)

`gates/whole-file-skips` reported: PR 2077
(`whole-file-skips-are-hand-spelled-not-anchored`, conversion half).
Thirteen literal skip lines naming eighteen homes in six gates now
build their anchors with `gate_record_anchor`, each gate declaring its
homes once for both the filter and the clean fixture, each clean
fixture planting every home with the very use its gate forbids, each
gate carrying the colon-path case; `gate-roster.sh`'s dot-only escape
through `gate_ere_escape`. Seven gates byte-identical live; per gate,
the literal restored reds at the colon case only and an over-narrow
anchor reds the clean fixture. Two residues filed on this slate: the
subject half (none of the six checks its home exists) and a live hole
the sweep found — `bounds-allowlist.sh`'s per-file select cuts the
FILE column at the first colon, so a compound bound at
`boxes.rs:x.rs` rides `boxes.rs`'s ratification. A `lib.sh` change
wanted and not made (the shared every-source-excluded planter assumes
the clean tree holds only the two shared sources). Reviewer dispatched.
## Landed: PR 2069 (2026-09-06)

`debug-only-bit-witness-callers-are-on-no-row` closed.
`bit-identity-debug-only.sh` has rows for the two caller files of the
bit-channel witnesses — `boolean/plane_eq.rs` (`plane_bits_witness`,
pin 1) and `merge_faces.rs` (`plane_bits_witness|vec3_bits_witness`,
pin 2) — so dropping either statement-position attribute, which a
consumer's release build refuses, is red here now; 15 subjects, 38
spellings, 97 uses scanned. No caller of the witnesses exists outside
`crates/topo/src`. The self-test runs each case once over all
subjects, asserting per subject that a diagnosis names its path and
the wanted text (spellings taken by index; `plant_subject_gone` stays
per subject because `gate_require_file` stops at the first missing
file): 110 s → 14 s, and a gate that stops at its first failing
subject now reds where the per-subject form could not tell.
Fix pass from the review: the fixtures header describes the loop that
runs; no subject count in a comment; history out of the loop's
comments; the pass twin names planter and spelling index; one shared
run skeleton and one diagnosis matcher, the pin harness reading
through it with both arms shown to fire; the substring assumption on
subject paths proved by a distinctness guard before any case runs; the
sweep's hit list completed (twelve files). Twelve mutations red.
Follow-up stated for the exit walk: the all-subjects run skeleton is
gate-agnostic and belongs in `lib.sh` beside `gate_selftest_case`.


## PR 2077 reviewed (2026-09-08)

Style review of `whole-file-skips-are-hand-spelled-not-anchored`:
mergeable. The class grep (13 lines, 18 homes → none), the two-direction
mutations on three gates, the seven-gate byte-identity, the roster
escape's value identity and the `bounds-allowlist.sh` live hole all
reproduced. One correction: "single source" holds mechanically only
fixture→filter (an over-narrow anchor reds the clean fixture); a home
dropped from the fixture but kept in the filter is green, which is the
subject-half residue's job. Fix pass sent, and since `lib.sh` is free
now it takes the two `lib.sh` items the PR had deferred: the
every-source-excluded planter clears what the gate's clean fixture
planted (the two local overrides deleted), and one
`gate_record_anchor_any` replaces the four-copy alternation builder.
Also: counts in OK lines derived from the arrays, the colon fixture
over every home, `march.rs` read from the array, the roster comment
trimmed, the residue row's "last one" given its grep, and a re-sweep
over the two gates main gained since the branch.

## Twelfth wave dispatched (2026-09-08)

`gates/bounds-select-anchor`
(`bounds-allowlist-select-cuts-at-the-first-colon` — the per-file
select reads the FILE column the way the record prefix defines it
rather than cutting at the first colon, with the fixture that a
compound bound at `<entry>.rs:x.rs` fires while the entry stays exempt
at its pin), from main, on the one gate no open PR touches. In fix
pass: PR 2077 (with the `lib.sh` planter fix and the shared
alternation builder folded in). Closing: PR 2069. Remaining after
these: the subject-half residue
(`whole-file-skips-do-not-check-their-subject`, on the six gates PR
2077 holds) and `D212`, parked on `G4`.

## Landed: PR 2077 (2026-09-08)

`whole-file-skips-are-hand-spelled-not-anchored` closed (its conversion
half; the subject half is its own row). Every whole-file skip in the
directory builds its pattern with `gate_record_anchor` — twenty-six
homes over seven gates, `register-equal-allowlist.sh` included after
the re-sweep over the gates main gained (its two `_RE` variables were
the class in the one spelling both greps miss, a literal assigned to a
variable) — each gate declaring its homes once for the filter and the
clean fixture, each clean fixture planting every home with the very
use its gate forbids, each gate carrying the colon-path case over
every home. `lib.sh` gained `gate_ere_alternation` and
`gate_record_anchor_any` (refusing an empty list), and its
every-source-excluded planter clears what the gate's clean fixture
planted, so the two local overrides are gone and
`panic-free-macro-bodies.sh` stays correct without one; the
dot-only escape in `gate-roster.sh` reads `gate_ere_escape`. Per gate,
the literal restored reds at the colon case only and an over-narrow
anchor reds the clean fixture; live output byte-identical as a
same-tree differential (one OK line now derives a count it used to
spell). The single-source claim holds mechanically only
fixture→filter; the other direction is
`whole-file-skips-do-not-check-their-subject`'s job. Noted for the
walk: `gated-suite-paths.sh` walks `.claude/worktrees/*` when lanes'
worktrees sit inside the checkout, so it reds locally and not in CI.
