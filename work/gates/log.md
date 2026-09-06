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
