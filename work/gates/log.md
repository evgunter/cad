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
