# S-TINT log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/tint/plan.md`.

## Opened (2026-09-11)

Opened on Ev's direction (in-chat, 2026-09-11) out of S-TCOST's re-sort
of its own board. The sort's question was which of that program's rows
survive the repository going public on 2026-09-03, and the answer split
the board three ways: rows whose subject is LATENCY (which survive), rows
that were buying BILLED MINUTES (which do not), and a third group that
was never about either. This program is that third group, given a
charter of its own rather than left on a cost program's slate.

**Thirty rows moved by `git mv`**, ids unchanged, each carrying a
`## Moved to S-TINT (2026-09-11)` record. The list was derived from the
tree, not from a document: the 42 live items in `work/tcost/` at
`origin/main` `f55ee213`, less the twelve that are cost rows or gate
mechanism. Those twelve stay, and are named here so the split is
readable from one side:

| stays with S-TCOST | why |
|---|---|
| `rust-cache-never-restores-across-branches` | a cold ~300-unit compile on the run's longest job — the pole itself |
| `nextest-shard-count-needs-remeasure` | re-opened 2026-09-11; the N=2 verdict was per-job billed-minute rounding |
| `tcost-area-pad-lever`, `offset-composite-lazy-sign-gate` | kernel constant factors the shipped program pays too |
| `edge-nurbs-computes-the-chart-image-and-discards-it` | a compute-and-discard candidate, unmeasured |
| `nightly-demotions-c1-c3-were-bought-with-billed-minutes` | the three landed demotions to re-cost |
| `gated-marker-omits-sibling-helper-imports`, `gated-marker-path-mount` | defects IN the per-file gate, which is S-TCOST's mechanism |
| `proptest-modules-in-src-ungated`, `r1-probe-seeds-are-not-on-the-fuzz-dial` | the fuzz-gating policy question, one ruling |
| `consider-proptest-for-randomized-sweeps` | parked on S-TCOST's own harness |
| `ci-filter-cites-a-path-the-ledger-recipe-cannot-open` | `scripts/ci-filter.py` is S-TCOST's file |

**The two territories overlap and that is deliberate.** Both programs
claim `crates/*/tests/*` and `crates/test-utils/*`; the fence is the
question, not the path, and `work.py territory` warns rather than blocks.
The rule is written into the plan's §*The fence with S-TCOST* so no lane
has to guess: a row justified by a second goes back to S-TCOST, a row
justified by a claim that cannot fail stays here, and the gate mechanism,
the fuzz-gating policy and everything under `scripts/` are S-TCOST's
whatever they look like.

**What this program inherits and does not re-open.** The re-home of
2026-09-04 that put these rows on S-TCOST routed them by path glob; one
of them (`malformed-ambient-eps-reds-review-m2-pr7-k`) says in its own
body that it *"is not a cost lever, so it does not belong on S-TCOST's
slate either"*. The re-home was not wrong to move them — they had no
other home — and nothing about any row's CONTENT is re-litigated by this
opening. Ids, titles, bodies, line citations and floors are unchanged;
several of those citations are frozen at a named SHA and are to be
re-derived at dispatch, not trusted.

**Nothing is dispatched.** No unit is cut, no branch is live, and the
slate's order is the first orchestrator's to set. The plan records the
two pairings worth not losing (`D383`+`S230`; `H12`/`S216`/`C18`/`D113`)
and the three rows that are Ev's decisions rather than work.

**Band 3500–3599** claimed in `docs/MODEL-AB-LOG.md` in this same
commit, per that entry's rule; the posture is inherited from S-TCOST's
test-only track, which records no A/B row, so the band is bookkeeping
until Ev says otherwise.

**(SYM orchestrator) Seam announced, 2026-09-13 — SYM-1** (`sym/1-profile`,
`docs/SYM-1-SPEC.md`): the profile inside `geom_core::sym` that
`work/sym/symbolic-tier-costs-95-percent-of-the-m10-3-drive` asks for
first. One new row file under `crates/editor-core/tests/m10_*`
(subject-named, `m10_sym_profile_interval.rs`), registered in
`tests/all.rs`, every row `#[ignore]` with a reason or gated behind a
test-only feature — the hosted gate's wall does not move, and the PR
states both runs' editor-core interval shard timings. S-TCOST's
measurement stays the result of record; nothing here re-takes it.

**(SYM orchestrator) Seam announced, 2026-09-14 — SYM-5** (`sym/5-unit-vector`,
`docs/SYM-5-SPEC.md`): one new row file under `crates/editor-core/tests/m10_*`
(`m10_derived_frame_interval.rs`, DOCM's two red derived-frame rows
ported and, once answered, un-ignored as the unit's pins at the
nominal and one width), registered in `tests/all.rs`; the evidence
rows stay `#[ignore]`; the pins' cost is stated in the PR body.

## First orchestration session opens (2026-09-15)

S-TINT has run no lane since it opened: no `tint/` branch exists, no PR
carries the prefix, every one of the 42 rows is `open`, and no row in
this directory sets `needs_ev`. So nothing was waiting on Ev at the
start of this session, and the slate's order is still unset.

**Review posture ratified (Ev, in-chat, 2026-09-15).** The plan's
§*Review posture* recorded the S-TCOST inheritance as *"open for Ev to
reset"*; Ev's answer resets nothing and confirms both halves: **no A/B
row and no A/B protocol**, and **one style review per unit** against
`docs/prompts/reviewer-style-lane.md` by path, with a full
(claims-falsification) review reserved for **the hardest units only** —
a per-unit judgement named in the unit's PR with its reason, not a
default. The band 3500–3599 stays bookkeeping.

**A row's body is not evidence about today's tree, and the first
spot-check proved it.** `m10-4-bore-pin-row-red-at-interval-1e-6` (filed
2026-09-03, GitHub 1646) describes an absolute `1e-9` slack that is
vacuous at ε = 1e-12 and red at ε = 1e-6. That assertion is **gone from
the tree**: `crates/editor-core/tests/m10_4_r2_probes_interval.rs`'s
`the_bore_pin_fit_as_a_consumer_reads_it` now pins the hull at both ends
against `0.2 ± half`, states the padding per half-width, and carries a
comment repairing exactly this defect and citing issue 1646 by number —
*"No absolute slack: an ε-independent term says nothing at the tight rows
and the wrong thing at the loose ones"*. The M10 lane closed it and the
row on this slate never learned. A second spot-check
(`torus-tangency-shell-floor-does-not-scale-with-k`) still reproduces:
`away()` is still a fixed floor and `bool3_torus_doors.rs`'s clearance
assertion still tells its reader to raise it.

One stale row in two reads is not a rate, and neither reading closes
anything. What it settles is the ORDER: the plan's instruction to
re-derive a frozen citation *at dispatch* is too late when the question
is whether a row is a row at all, so **the first act of this program is a
re-derivation pass over all 42 rows against the current tree** — does the
defect still reproduce, and is the row's floor still its floor — before
any unit is cut. The pass is bookkeeping about the slate, not work on the
suite, and it lands as item-file edits (closures with their evidence, or
re-derived citations) rather than as a unit.

## D113 ruled and closed (2026-09-15)

**Ev, in-chat: an alternate citation format would not actually prevent
drift, so do nothing.** `D113` asked what an intra-doc link in a
`tests/` file is and undertook to land the mechanism the answer needed.
The answer is that it is prose, checked by nothing, and that is
acceptable: no ban, no de-linking sweep, no gate. Closed.

The row's own fork — *"the form stops being used, or something is built
that resolves it"* — read as two live options and was one. Converting
`[Foo]` to `` `Foo` `` removes a promise of a hyperlink that never
rendered and adds no check; a stale link and a stale backtick are the
same defect, and the spelling is not what rots. So the real fork was
always *build a checker or don't*, and **a checker would key on an
identifier in a doc line whichever brackets are around it** — it never
needed this decision. The format question is retired as a red herring.

**The measurement that closed the other half** (`work/tint/D113.md`
carries it in full, taken this session): `cargo doc` has no test-target
selection at all; `cargo rustdoc` does, but cargo will not pass
`--extern` for a package's own lib when documenting its test target, so
the doc unit has to be assembled by hand-injecting an rmeta chosen by
trial and sensitive to feature unification. And **rustdoc does not
document `#[test]` items** — verified twice, once by the lane and once
independently here with a four-link control crate, where the module doc,
a plain `fn` and a `const` were all reported and the `#[test]` one was
silently dropped. That is **282 of 1086** candidates invisible, one of
this row's own six named breakages among them. Cost, for the record,
since it was the question asked: ~+170 s on a 4-core box, ~3–6 minutes
added to the hosted `fmt` job — affordable, and not the reason to
decline.

**Two things are deliberately NOT scheduled**, so that nobody
re-discovers half of this and re-opens it. The ~62 identifier-shaped
citations under `tests/` that do not resolve today stay unfixed: a
one-time cleanup with no guard behind it re-rots, which is Ev's own
argument applied to the cleanup rather than to the format. And this
row's census is stale by ~2.5× (465 files / 294 candidates at
`cfdc1c6f`, against 1146 tracked files today) and is not being
re-derived, because nothing now consumes it. Both facts are recorded on
the row, which dies with this directory when the program closes; if the
trade is re-taken it is re-taken as *write the checker*, never as *fix
the 62*.

**The four-sided pairing the plan recorded is dissolved and three sides
remain.** `H12`, `S216` and `C18` share `D113`'s root cause — rustdoc
collects nothing from a `tests/` target — but none is a citation-format
question, and `H12` is sharper than anything `D113` was about: eleven
`compile_fail,EXXXX` blocks in `geom-core/tests/review_m0_pr2.rs` and
`review_m0_pr3.rs` are negative proofs no tier has ever evaluated, which
start passing silently the day a bound is loosened. Shape 1, not shape 4.
Plan updated.

## D70 ruled and closed; two kernel defects filed off-slate (2026-09-15)

**Ev, in-chat: *"if the reframed D70 is 'should they be conditional?'
then the answer is no, we should fix the defect (or, like, you would
file the issue for fixing the defect and move it off your slate)."***
Done, and the row is closed with nothing scheduled here.

The thirteen silent stand-downs are **two kernel defects wearing one
test-suite costume**, which is why the row read as test work for a
month. Twelve stand down on the plane × NURBS certificate — limb 2's
bound is a function of a fixed sample schedule (`PXN_FIT_SAMPLES = 33`,
`CERT_SAMPLES = 9`, `PXN_IMAGE_DEGREE = 1`, all `const`) and does not
refine with ε, on a carrier that is EXACT by construction: both walls
come from one `loft_body` call, the seam edge is simultaneously the loft
of a profile vertex and a boundary iso-curve of the bowed patch, and
every control point of that corner has `y = −scale` exactly. Filed on
TRIM, which owns `edge_nurbs.rs`. The thirteenth stands down on
`SSI_MAX_FIT_SAMPLES`, a `const` cap on a sample count the marcher grows
as ε shrinks; filed in `work/issues/` because no open program's `paths`
claim `crates/geom-brep/src/ssi.rs`, with both readings (resource cap vs
class boundary) and the measurement that distinguishes them.

**The route to the answer is worth recording, because the row's own
framing hid it.** Ev's question was *"if it's a valid intensional
description then it should be valid under any decrease in epsilon
(unless it's ulp-scale)"* — and at metre coordinates ε = 1e-12 is ~4500
ulps, so it is not. That test is what turned a test-hygiene row into a
kernel row in one step. Everything this program had proposed before it
(announce the skips; re-mine the fixture onto `INTERIOR_COLUMN_SCALE`)
was a way of LIVING WITH the ε-conditionality, and one of them —
shrinking the model until a fixed bound fits — had already been taken
once under #1167 and would have propagated a workaround into eleven more
fixtures. **A row that says "this test stands down" is worth asking the
kernel about before asking the suite about.**

**What stays here.** `work/tint/loud-stand-down-announcements-are-discarded-by-the-gate`
— found while gathering D70's context, not about D70, and it survives
those thirteen disappearing. Nothing else from D70 is carried: its
"13 is a FLOOR" qualifier, its 116-`continue`-site exclusion and its
`#[ignore]`d-collection-run standing note were framing for a question
that is now answered, and they die with the row rather than being
re-filed as work nobody asked for.

**Board: two of the slate's three decision rows are now closed** and
neither cost a unit. The remaining one is "any change to a `memories/`
clause", which is not a row. 40 items live.

## Two stale citations fixed (2026-09-15)

Ev, in-chat: fix the two, and let them ride the next state sync. Done on
`tint/orchestrator`; they are the only CODE edits this branch carries and
this entry is why.

They are the two of ~62 that sit in frequently-read files — the test that
sorted them is Ev's: *"if any of them are actually in frequently read
code they could be cleaned up now … probably not worth it though if they
were just encountered by going looking."* The other ~60 are in leaf probe
suites and are **not** filed and **not** scheduled.

- `crates/topo/tests/common/mod.rs` — ``[`BooleanDeclarations`]`` →
  ``[`topo::BooleanDeclarations`]``. The type is `pub` (declared in
  `topo/src/boolean/mod.rs`) and the helper's own return type already
  spells it qualified; only the citation was bare, and that file imports
  `topo::{Body, FaceSurface, …}` without it.
- `crates/mesh/tests/all.rs` — ``[`Eps`]`` → `` `Eps` ``. There is
  nothing to point it at: `Eps` is `pub(crate)` in
  `crates/mesh/src/sizing.rs`, so an integration test can never resolve
  it (D113's `Leg::tangent_point` case). The same doc comment already
  spells the name in plain backticks nine other times, so this makes one
  outlier match its neighbours rather than applying any policy — **D113
  ruled that a citation FORMAT is not a lever and nothing here reopens
  that.**

**The biggest single file in the hit list was not touched and is not a
defect.** `crates/mesh/tests/common/mod.rs` carries six of the ~62 and
all six are false positives: `[1,2]×[0,1]` interval notation that
rustdoc reads as links. Across the whole sweep 30 of 111 are that shape
(`[0,1]` alone is 12), which is a property of the instrument and not of
the tree — worth knowing before anyone re-runs that sweep and reads its
count as a defect count.

## The re-derivation pass dispatched; one row closed ahead of it (2026-09-15)

**Four read-only lanes, 39 rows, disjoint sets**, each appending a
`## Re-derived (2026-09-15, lane X)` section with one of four verdicts —
REPRODUCES / STALE-FIXED / PARTIAL / UNVERIFIABLE-WITHOUT-A-RUN — to its
own item files and nothing else. Lanes were told to run no cargo command
(four concurrent builds would contend, and re-derivation is a reading
job), to write no git, to close nothing, and to file nothing: a lane that
finds a new defect reports it and the orchestrator files it. Disjoint
file sets are what makes four concurrent writers safe — one writer per
file, and the orchestrator commits.

- **A** — the doctest/`compile_fail` cluster and the remaining Track W
  units: `C18`, `H12`, `S216`, `D72`, `D380`, `D381`, `D382`, `D385`, `D386`.
- **B** — guards that cannot go red: `D383`, `S230`, the anti-vacuity
  floor, the fuzz-trial floor, the two probabilistic-guard flakes, the
  structurally-zero sign asserts, the torus K-scaling floor, the
  malformed-ambient-ε red, the unexplained literal seeds.
- **C** — censuses and hand-kept guards: the reader census, `source`'s
  item-body carve, the body-hash census, the loud-skip marker, `assert_f6`'s
  dumps lists, the interrogate ladder, the M10-6 roster, the
  source-scanning tripwire, the shell census golden, the landing gather.
- **D** — one claim in N copies, and citations to a tree that has moved:
  `D384`, the chamfered-cube/Steiner oracles, the geom-brep inline
  surfaces, the origin-anchored fans, the brick/prism copies, the
  two-vertex bulge, the M10 P-lift plane, the decoration seam header, the
  sixteen `tests/`→`tests/` citations, the R1 dual digest ladder.

Each lane carries the two known staleness shapes as its calibration —
the bore-pin row's silent repair and `D113`'s 2.5× census drift — so
neither outcome reads as the default.

**`m10-4-bore-pin-row-red-at-interval-1e-6` closed, ahead of the pass and
by this seat** (it was assigned to no lane). STALE-FIXED: the `1e-9`
absolute slack is gone, the row now pins the hull at both ends, and the
comment above the assertion repairs this defect by name and cites issue
1646 — this row's own number. It also could not have survived: filed
2026-09-03 under configuration sampling, and since 2026-09-04 every
code-tier run draws interval / 1e-6, so a row red there would red `main`
on every PR. It sat open for eleven days after it became impossible.

**Ev is away from this point** (in-chat, 2026-09-15) and has authorised
merging the state sync at this seat's discretion. Nothing is queued for
Ev: both decision rows closed today, and no row in this directory sets
`needs_ev`. The standing order is therefore to keep working rather than
to wait — adjudicate the pass, close what is stale, re-cut the slate from
what survives, and land it.

## The re-derivation pass adjudicated: nothing retired, and the slate was under-counting (2026-09-15)

**39 rows, four lanes, and the result is the opposite of what this seat
predicted.** The order was set on the strength of two stale rows, and I
told Ev the pass "will retire more than it keeps". It retired **nothing**:

| verdict | rows |
|---|---|
| REPRODUCES | 24 |
| PARTIAL (some members closed, some live) | 14 |
| UNVERIFIABLE-WITHOUT-A-RUN | 1 |
| STALE-FIXED | **0** |

The single fully-stale row on this slate was `m10-4-bore-pin-row-red-at-interval-1e-6`,
and this seat had already found it before the lanes went out. **The
inference from one instance was wrong**, and the reason is worth keeping:
that row went stale because someone else FIXED its subject. Most of these
rows describe things nobody is working on, and an unattended defect does
not decay.

**What moved instead was the counts, and almost all of them upward.**
`D380` 3 → ≥7. `D382` 13 → 14 (and all fourteen bodies hash identical —
a smaller unit than filed). `S216` 39 → 61, with its "8 uncoded rows"
category gone and four error codes never sorted into its split.
`H12`'s surrounding doc-fence census 28 → 72. `geom-brep-inline` 32 → 40.
`loud-skip-marker` 8 → 10. `chamfered-cube` 5 → 7. `the-two-vertex-bulge`
8 → 9. `brick`/`prism` → 9. **A stale enumeration in this tree
under-counts**, because the class keeps recruiting while the row sits
still. Every "this is a FLOOR" qualifier on this slate was doing real
work and none of them was pessimistic enough.

**Three predicted drifts have stopped being predictions.** These are the
pass's real product:

1. **`assert-f6-dump-lists` — fired, three times.** Verified at this seat,
   not taken on report: `ParseError` has **11** variants and its `dumps`
   list in `crates/editor-core/tests/display_contract.rs` bans **10** —
   `Dimension` is unbanned. `SelectRefusal` (8 variants) leaves
   `Band(BandError)` unbanned; `DeclareError` (3) leaves `Edit(EditError)`
   unbanned. Two of the three holes are payload-carrying wrappers, so
   neither the wrapper name nor the inner error's rendering is covered.
   The row filed a forecast; the tree has since supplied the instances.
2. **`loud-skip-marker` — fired.** `crates/viewer/tests/error_display.rs`'s
   marker doc says in its own words that a second `app`-gated row would
   leave it quietly incomplete. The file now has two and the marker names
   one.
3. **`C18`'s grid block — fired.** `m5_pr7_ssi.rs` says five cells have no
   row; the fifth got one about 850 lines below and the census never
   learned. Five is four.

**And one tracker statement was simply false.** `C18` is not related to
`H12` at all. Settled by reading PR #734, which is *"Track C lane C-d,
finding H12: the SSI sweeps' other never-silence doors have no acceptance
row"* and whose Recording section says *"§D gains row C18"*. That `H12`
is a Track **C** finding id; `work/tint/H12.md` is the Track **H** unit
about never-collected doctests — two unrelated findings sharing an id,
cross-wired when the rows were re-homed by path glob. Three places said
so and all three are corrected (`C18`'s `refs`, `H12`'s Notes and `refs`,
the plan's pairing). **This seat repeated the error to Ev** before the
pass caught it; the plan now records two sides where it recorded four.

**Rows whose HALVES closed**, which is where the real movement was:
`D384`'s `S89`/`ring` half is gone entirely (no `fn ring` in
`enclose.rs`; all 36 crossings call `RingInterval::from_certified`);
`sweep-test-suites-cite-eight-deleted-suite-files` has **zero** dangling
citations today, all 14 resolve, though the resolver-corpus half it asks
for is untouched; `origin-anchored-fan`'s documentation member and its
twin divergence are both fixed; `landing-gathers`' basis is largely gone,
two of its three "only the test does this" claims now false.

## Three rows filed out of the pass

- `work/tint/cavity-module-doc-restates-a-census-that-is-now-wrong` — a
  hand-kept census in prose, inside the file that declares the rule,
  asserting two byte-identical pairs that are both false today. Carries
  the two stale `work/tcost/` tracker paths in the same directory.
- `work/tint/test-headers-name-fns-that-exist-nowhere` — four names, two
  files, zero resolving. **Filed to be merged into the roster class, not
  taken alone**: `D113` already ruled that hand-fixing unresolving names
  with no guard is not worth scheduling, and these differ only in sitting
  inside rosters, where two live rows on this slate already are.
- `work/topo/review-d18-probes-header-miscounts-its-own-rows` — TOPO's
  ground, TOPO's call, no fix proposed on their behalf.

Lane findings that belonged to existing rows were added as evidence
rather than filed: the `ambiguity_k_env.rs` re-exec sibling, the
half-instrumented `r2_lt_probes.rs` discard, and `away()`'s ε-invariance
(which makes the torus floor the test-side instance of the same
fixed-vs-varying defect the two kernel rows `D70` closed onto).

## The slate, re-cut

Nothing was retired, so the order is set by what the pass proved rather
than by what it cleared. **Recommended first unit: `assert-f6-dump-lists`**
— the drift has fired, the holes are named and verified, the population
is seven lists in one crate, and the fix is this program's own standing
rule (one executable home, every other site points at it) applied to a
guard that is currently not guarding three error variants. It needs
nobody's territory but this one's.

**Second: the announcement pair.** `loud-stand-down-announcements-are-discarded-by-the-gate`
and `loud-skip-marker-is-a-hand-kept-idiom` are one mechanism seen twice,
and the pass showed they compound — the eight markers' `println!` bodies
ARE the hand-kept enumerations, and every gating archived run discards
them, so those lists are not merely stale-prone but unreadable on every
run that matters. The repair that makes a stand-down a tallied fact the
suite can floor closes both.

**Not yet cut and why**: the roster class (`r2-m10-6`,
`interrogate-ladder`, `test-headers-name-fns…`, and TOPO's row) wants one
executable check rather than four prose edits, and whether that check is
worth building is a real question this program should answer before
spending an edit on any of them.

## TINT-1 cut: the `assert_f6` ban lists (2026-09-15)

First unit of the program. `docs/TINT-1-SPEC.md`, branch
`tint/1-assert-f6-dumps`, row promoted `issue` → `unit`, status `spec`.
Chosen because the pass turned it from a forecast into three verified
live holes, the population is seven lists in one crate, and it needs
nobody else's territory.

**The spec settles the fix shape rather than leaving it to the lane, and
the reason is a row on this same slate.** The item weighed three
mechanisms — an `ALL` const per enum, a census row per enum, or reading
the variant identifiers out of the source through `test_utils::source`.
All three are refused in favour of **the compiler**: an exhaustive
`match` from a value to its variant identifier, no wildcard arm, with the
ban list derived from it. A variant added tomorrow then makes the file
fail to COMPILE.

The source-read census is the one to refuse loudest, because it is the
obvious choice and it is a trap:
`work/tint/source-scanning-censuses-are-a-tripwire-on-ordinary-rust` is
live on this slate and says the existing source scanners hand-parse Rust
and fail loud on ordinary-but-unusual signatures. An eighth scanner would
**mint a fresh instance of a defect this program is holding a row on** —
the exact shape `docs/prompts/reviewer-style-lane.md` §1 warns of, where
a lane closing a hand-written list adds a hand-written census. Naming the
trap in a PR body has never prevented it; refusing the mechanism in the
spec might.

**The unit's real deliverable is the second half.** Completing the ban
lists changes nothing: `assert_f6` inspects only the renderings the
`cases` supply, and the three missing variants have no case either. So
the unit owes case COVERAGE — every variant constructed and rendered,
enforced against the same `match` — and that assertion is the one that
goes red. Writing the three missing cases may also surface a rendering
that really does dump, which is a found bug and not an obstacle.

**One enum resists and the spec says so out loud.** `SelectRefusal` is
`#[non_exhaustive]`, so a `match` in `editor-core/tests/` needs a
wildcard and rustc enforces nothing; a panicking wildcard fires only if a
case constructs the variant, which is the same vacuity. It gets the
shape the other six get, plus a comment naming the attribute as the
reason its coverage is not compiler-enforced. The real home for it is a
unit test beside the enum, where the match IS exhaustive — but
`crates/editor-core/src/names/geompred.rs` is **WIRE's** territory, so the
lane files that residue as its own row and this seat announces it. Six of
seven compiler-enforced with the seventh stated is the honest outcome;
weakening the six to match the seventh is not.

## TINT-1 landed, and the review is why it is worth anything (2026-09-15)

PR #2648, branch `tint/1-assert-f6-dumps`, CI green on run 34953299497 —
12 `test (…)` jobs, 5 `k-lint (gate, …)`, 0 failures, verified at this
seat rather than taken on report. The two `neutral` render-drift checks
are Checks-API postings, not jobs, and `memories/freecad-render-lane.md`
is explicit that a PR posts one and that chasing it green is wrong.

**The first implementation was green, complete against its spec, and
still shipped the defect it was closing.** It made a wildcard-free
`match` per enum whose arms returned **hand-typed identifier strings**,
with a `*_VARIANTS` roster beside it. rustc checks a match's PATTERNS
and never its strings — so an arm reading
`ParseError::UnknownUnitSymbol { .. } => "UnknownUnit"`, which is what a
RENAME produces, left the suite **green while banning a dead identifier
and leaving the live one unbanned.** The style review planted exactly
that and ran it on the pre-fix file: `... ok`.

That is this program's own charter shape, minted by the fix for it, in
the unit whose spec warned against precisely that trap in a paragraph of
its own. **Naming the trap did not prevent it.** Only a reader who did
not write the fix caught it — which is the standing claim of
`docs/prompts/reviewer-style-lane.md` §1, and it just paid for itself on
the first unit this program ever ran.

**The repair, and the honest statement of what the guard is.** The
identifier now comes off each value's own derived `Debug`
(`test_utils::f6::variant_identifier`) — ground truth — and the match
arms name nothing, returning `()`. So the chain is: an exhaustiveness
token that **forces the author to open the file** and nothing more; an
identifier that cannot be misspelt because it is read, not written; and
a set difference that therefore **enforces** the roster rather than
trusting it. **"The compiler is the census" was my spec's phrase and it
was wrong**; it is withdrawn there and in the code comment that had
copied it. One hole remains and is disclosed at the helper: add a
variant, add its arm, add neither a case nor a roster entry.

**Fixing it turned up two more live instances and a fourth suite.**
`a_predicate_flip_names_its_signs_as_words` banned two of `Sign`'s three
variants; `NodePickError::Tessellate` had never run the F6 shape at all;
and `crates/mesh/tests/errors.rs` bans 14 of `TessellateError`'s 15 with
a `Band` case already constructed — filed, not fixed. The predicted
divergence in `work/view/f6-display-predicate-is-spelled-three-times-with-no-home`
**had already happened** (`m4_pr4_hit.rs` banned `"node:"`,
`display_contract.rs` banned `"node:"` and `"name:"`), so the predicate
took the home that row names, `crates/test-utils/src/f6.rs`, and that
row got evidence rather than a duplicate.

**Three residues filed on three slates**: WIRE (`SelectRefusal` is
`#[non_exhaustive]`, so ADDITION cannot be compiler-checked from the test
crate), S-TINT (the sibling suites, carrying the live mesh hole), and
EDIT (`persist/check.rs` renders `slot {slot:?}` into a user-facing
sentence, so a variant identifier reaches the reader — found by the
review, outside the unit's diff entirely).

**Process note worth keeping.** The fix pass also tried deriving the
field-name roster from the payload's `Debug` and it false-positives:
`MeshPickError::PositionOutOfRange` renders *"pick index: triangle 5 of
patch 1"*, a door's own prose, against a payload with an `index` field.
A derivation needing a per-site exemption is a hand list in a
derivation's clothes, so that half stays the caller's and says so at the
module doc. Not every hand-written list has a derivation waiting for it.

## TINT-1 merged; TINT-2 cut (2026-09-15)

`1305231ad` on main. The unit's own record is its item file; what
belongs here is what the program learned.

**The first implementation was green, complete against its spec, and
shipped the defect it was closing.** Its match arms returned hand-typed
identifier strings beside a hand-written roster; rustc checks a match's
PATTERNS and never its strings, so an arm reading
`ParseError::UnknownUnitSymbol { .. } => "UnknownUnit"` — what a RENAME
produces — left the suite green while banning a dead identifier and
leaving the live one unbanned. Demonstrated on the pre-fix file by the
style review, not argued.

**The spec was wrong, not merely imprecise, and the wrongness was mine.**
*"Use the compiler"* and *"a variant added tomorrow makes this file fail
to COMPILE"* overstated what that design could deliver, and the phrase
*"rustc is the census"* was copied out of the spec into a committed doc
comment. Both are withdrawn; `docs/DOC-LEDGER.md`'s deletion entry
records the spec as wrong rather than superseded, which is the honest
shape for a spec that misled its lane.

**The rule that survives**: *naming a trap does not prevent it.* The spec
devoted a paragraph to refusing the source-scanning census precisely
because it would mint a fresh instance of a row on this slate — the lane
obeyed that and then minted a different fresh instance two lines away.
Only a reader who did not write the fix caught it, which is the standing
claim of `docs/prompts/reviewer-style-lane.md` §1 and it paid for itself
on this program's first unit. **So TINT-2's spec states what its guard
does NOT enforce, in the spec, before the lane writes a line.**

**A process cost worth not repeating.** Three CI runs on #2648 were
cancelled by supersession because this seat pushed four times in
succession — a filed finding, the state sync, the spec deletion with its
ledger entry, and the base merge. Three of those four were one logical
act. The close-out of a unit (item file, log, spec deletion, ledger,
base merge) assembles into ONE commit and ONE push; on a repository with
a program devoted to CI minutes, a push to a branch with a run in flight
is not free.

## TINT-2 cut — `docs/TINT-2-SPEC.md`, branch `tint/2-stand-down-channel`

Both announcement rows take one lane; `loud-stand-down-…` is the unit and
`loud-skip-marker-…` is parented to it.

**The framing that makes it one unit rather than two.** The marker rows
have a working half and a broken half: the `fn` NAME reaches nextest's
PASS list and IS read, while the `println!` BODY — which is the
hand-kept enumeration — is discarded on every gating run. So those eight
hand-kept lists do not merely go stale, they have **zero readers** where
it counts. `stood_down` has no working half at all: called inside a
passing test, nothing reaches the PASS list and the entire payload is
discarded, 22 sites in 10 files.

That splits the fix where the two rows did not. An enumeration nobody
can read cannot be justified by the cost of keeping it in step, so for
the markers **deleting the payload beats rewriting it** and the name
survives; `stood_down` has nothing that works and so needs a mechanism
or nothing. The spec states both shapes, leans to tally-for-`stood_down`
and strip-for-the-markers, and leaves the lane to argue it.

**A second marker has already drifted**, which the 2026-09-11 filing did
not know: `crates/sweep/tests/blend_margin_payload_interval.rs` names
*"the enclosure arm"*, singular, where three rows are gated —
`error_display.rs` (one named, two gated) is no longer the only fired
instance.

**Fence note carried into the spec**: `crates/viewer/src/lib.rs` holds
one of the ten markers and is `src/`, so it is filed rather than edited;
and any repair spelled as a workflow flag is CIW's, read-only here.
## TINT-2 landed (2026-09-15)

PR #2656, CI green on run 34966198536 — 12 `test (…)`, 5
`k-lint (gate, …)`, 0 failures. Both rows closed; their own files carry
the account.

**No guard landed and the unit says so.** Nothing in it can go red. What
it bought is that the tree stops claiming otherwise and that nine marker
sites became one macro. For a program whose charter is *a guard that
cannot go red is not a guard*, shipping a unit with no guard is worth
being explicit about: the guard was not available, which is a different
fact from not being attempted.

**The spec's preferred repair did not exist.** It leaned to a tallied
fact the suite could floor; the review verified behaviourally that
nextest is process-per-test (pids 12156/12157, a `static AtomicUsize`
reading 0 in both), so nothing one row records is readable by another.
What survives for the 22 `stood_down` sites is 22 per-row posture
decisions, which `D70` records as **C21's** and not this unit's. One
refinement worth keeping: a row CAN floor a dynamically counted
condition and this tree does (`m5_pr7_ssi`'s fit-budget arms); what it
cannot floor is a sibling's stand-down.

**`test_utils::loud_skip_marker!` is the win.** The feature literal
reaches both the `#[cfg]` and the printed sentence from the same token,
so they cannot disagree, and `file!()` supplies the filename. Nine
rustdoc copies and a hand-typed `mod certified` string that a `git mv`
would have desynced in six files went with it.

**Three marker copies were FALSE, not the one the review named.**
`m6_2_fitted_at_rest.rs` (3 ungated rows), `error_display.rs` (15) and
`panel_display.rs` (16) each claimed their file's rows were the gated
ones. The macro's wording closes that class by construction rather than
patching three sites.

**Two marker NAMES were enumerations too**, and the name is the half that
reaches the PASS list — renamed, with every external consumer verified
to key on the prefix (`ci.yml`, `ci-local.sh`, `GUI-DESIGN.md`) or on
the unchanged interval name (`check-interval-cfg-additive.py`,
`interval-only-selection.py`).

**A routing correction from the lane, worth carrying:** `scripts/check-*.py`
is **CIW's** territory, not S-TCOST's — S-TCOST holds only
`ci-filter.py`, `slowest-tests.py` and `base-test-listing.sh`, and
`scripts/gates/*` is Track K's. This seat had misrouted the guard row.

**`work/tint/process-observations.md` opened.** Two units in, two
patterns worth a file rather than a log line: a unit closing a
hand-written mirror mints a fresh one inside its own fix (twice, both
caught only by the outside reviewer, both with the trap named in the
spec), and both specs so far have misled their lane with a mechanism the
orchestrator had not executed. The second is this seat's to fix, and the
correction is written there.

**Board**: five rows closed, four residues filed off-slate this unit
(two CIW, one VIEW, one S-TINT). The roster class is the next question
and deliberately not yet a unit — `r2-m10-6-header-roster`,
`interrogate-ladder-header`, `test-headers-name-fns-that-exist-nowhere`
and TOPO's `review-d18-probes-header-miscounts-its-own-rows` all want
ONE executable check rather than four prose edits that re-rot, and
whether that check is worth building is the decision to take before
spending an edit on any of them. If it is not, they close the way `D113`
closed.

## The roster class was not a class (2026-09-15)

A feasibility probe ran BEFORE a spec was written — the first time this
program has done that, and it broke the grouping this seat had proposed
to Ev two hours earlier.

**The claim was that four rows wanted one executable check. Two do.**

- `interrogate-ladder-header-claims-every-rung-and-pins-five` claims
  coverage of an **enum's variants**, which a roster of test names
  cannot see. Its welded shape is TINT-1's exhaustive `match`, which has
  landed, and `display_contract.rs` already carries a complete welded
  enumeration of `InterrogateError`'s ten variants to copy. **Routed out
  of the class; cheaper than it looked.**
- `test-headers-name-fns-that-exist-nowhere` is **closed as wrongly
  filed, by the seat that filed it this morning.** Both its sites name
  retired rows on purpose — `m5_pr13_curved.rs` under an explicit
  *"Lineage: `CENSUS` succeeds …"* heading. The row read a population
  off a pattern (backticked identifiers resolving nowhere) and assigned
  it a subject (a roster that over-counts) without reading the sentences
  around the names. **That is this program's own charter defect,
  committed while filing against it** — observation 1, one level up from
  the code.
- TOPO's `review-d18-probes` stays TOPO's. The mechanism is verified to
  work in its exact shape (`#[cfg(test)] mod` inside `src/`) and has
  been offered as a pointer with its limits stated, not as a request.

**The mechanism, which the probe built and ran rather than proposed**:
read the roster off libtest's own `--list` via a `current_exe()`
re-exec — the harness's ground truth, **no Rust parsed**, so not a
fresh instance of `source-scanning-censuses-are-a-tripwire-on-ordinary-rust`.
The weld is a `roster!` macro whose each entry is an ident feeding three
consumers: `let _: fn() = $name;` (a retired name is a compile error),
`stringify!($name)` (the compared string cannot be mistyped), and the
printed text. Five falsifiers run green, ~3.6 ms per check, and the
tree already re-execs its own test binary at eighteen sites.

**TINT-4 cut** — `docs/TINT-4-SPEC.md`, branch `tint/4-roster-weld`, ONE
row and ONE adopting file. The spec names the measurement that can kill
it (time the self-`--list` inside `editor-core`'s `all.rs` under
`--features interval`, the largest binary in the tree and the one the
adopting file lives in) and requires the lane to stop and report if it
comes out badly. It also states, at the site and in the PR, that the
mechanism welds **names and never prose** — the m10-6 row's own defect
has a sentence half this design will never catch, and a lane that ships
"the roster cannot go stale" would be wrong in exactly TINT-1's way.

**What changed in how this program works.** Observation 2 said a spec
must name the measurement that would show its mechanism cannot work.
This is the first spec written after an executed probe rather than
before one, and the probe's value was not the mechanism — it was
discovering that two of the four rows were mis-classed, one of them by
this seat, on the same day. **Probing before speccing found a filing
error that two reviews would not have**, because no reviewer reads a
row's premise against the tree; they read a diff.

## TINT-3 landed: fifteen guards, one macro (2026-09-15)

PR #2680, CI green on run 35005522838 — 12 `test (…)`, 5 `k-lint (gate, …)`,
0 failures. `D382` closed; its own file carries the account.

**The measurement came before the mechanism, for the first time in this
program, and it held.** `env!("CARGO_MANIFEST_DIR")` and
`include_str!("all.rs")` inside an exported macro both resolve at the
INVOCATION site — probe, negative control (definition-site file deleted,
clean rebuild), spelling control, then re-confirmed independently by the
review. Had it gone the other way, all fifteen rows would have checked
`test-utils`' own tree while reporting on fifteen crates, and **nothing
would have red'd**. That is the failure this program exists to find, and
it would have shipped green.

**The count was fifteen, and the fifteenth is the row's own rider.**
`crates/test-utils/tests/all.rs` landed between the re-derivation and the
fix. The row had asked for `test-utils` to be opted in; it opted itself
in by growing a copy, which is the class recruiting while the row sat
still — the pattern the re-derivation pass named in September.

**The unit's real lesson is about what a collapse costs.** The census
had been detecting each aggregating `all.rs` by a margin of exactly ONE
literal, supplied by `include_str!("all.rs")`. Collapsing to a macro took
fourteen of fifteen to zero margin, so detectors had to move — and that
is indistinguishable, from the outside, from weakening a guard to make
your own change pass. The review adjudicated it a correction (a
`||`-needle list widens monotonically; the silencing option of deleting
fifteen ledger lines was available and not taken) and then found the part
nobody had said: **fifteen independently-checked facts became one**, and
the one is exempt from the row that would check it. The fix pass closed
that in ~25 lines with a row that reds on exactly that mutation and
nothing else.

So the shape to carry: **a 15→1 collapse is worth it when the fifteen are
byte-identical, and the honest accounting says what the fifteen were
buying.** Written at the macro, not only in a PR body.

**Three seats, three corrections, each catching the one before.** The
lane reported the 33 duplicated helper bodies as "already covered by six
rows"; the review showed all six are geometry-fixture rows and named
seven uncovered classes; the fix pass measured past both — `validated`/`vp`
at **24 copies in 8 distinct bodies**, eight of eleven drifted — and
corrected the review in turn, finding that two of its seven
(`fnv`/`digest`, `missing_pairs`) DO have rows on `work/perf/`. **This
seat repeated the review's claim without checking it.** Filed as
`cross-crate-test-helper-copies-outside-the-geometry-fixtures`.

**Board**: 8 rows closed. Three units landed, each one's guard proved by
mutation rather than asserted. TINT-4 is in flight on the roster weld.

## TINT-3 was merged, and not onto main (2026-09-15)

The entry above says TINT-3 landed. It did not. It is on
`tint/2-stand-down-channel`, and `e4adf05a1` — the merge commit — is
reachable from that branch and from nothing else.

**What happened, with times.** PR #2656 (TINT-2) merged into `main` at
17:14:33. PR #2680 (TINT-3) was opened at 17:16:50 with base
**`tint/2-stand-down-channel`**, because the lane cut its branch off
TINT-2's head to build on work that had not landed yet — which was the
right call at the time it was made and stopped being right two minutes
before the PR existed. Nobody retargeted the base. At 18:46 the merge
API returned `"merged": true` and it was telling the truth: it merged
the head into the base it was given.

**Why nothing caught it.** The merge succeeded. CI was green on the
head. `work.py lint` passed. The log entry above was written from the
API's `merged: true` and from a green run id, and both of those are
facts about a PR rather than facts about `main`. There is no state in
which that API call reports a wrong base, because to the API there is
no wrong base.

**That is this program's own charter shape, committed by this
program's orchestrator.** A check that cannot go red is not a check.
`"merged": true` cannot go red on the thing the orchestrator was
actually asking — *is this on main* — so reading it as an answer to
that question was reading a green light on a wire that is not
connected.

**Who caught it**: TINT-4's style review, as NOTE-1, about forty
minutes later and while reviewing a different unit. Not the
orchestrator, and not any gate. That is the third time in this program
that the outside reader found what the seat doing the work could not
(observation 1's two are the others).

**The correction, adopted now.** A unit is landed when its merge commit
is an ancestor of `origin/main`, asserted with

```
git fetch origin main && git merge-base --is-ancestor <merge-sha> origin/main
```

and not when an API said `merged`. The log entry naming a unit landed
is written after that command, not before. And a lane whose branch is
cut off another unit's branch retargets its PR base to `main` the
moment that unit lands — or, better, cuts off `main` and merges the
dependency in, so the base is `main` from the start and there is
nothing to remember.

**Repaired by**: this PR, which carries `e4adf05a1`'s content onto
`main` through `tint/orchestrator` (which already contained TINT-3's
head plus the state sync). `tint/4-roster-weld` does not contain
TINT-3, so TINT-4 was never blocked on this and its base was always
`main`.

## TINT-4 landed: a roster welded to libtest's own listing (2026-09-15)

PR #2687, merged at `2101cb36a` and **verified on main by
`git merge-base --is-ancestor`** rather than by the merge API's
`merged: true` — the first use of the correction the entry above
adopted, on the first unit after the one that needed it. CI green on
the fix-pass head: 39 jobs, 0 failures, twelve `test (…)` and five
`k-lint (gate, …)`.

`test_utils::roster!` replaces the hand-kept `//!` enumeration in
`crates/editor-core/tests/r2_m10_6_probes_interval.rs`. Each entry is
an ident feeding three consumers — `let _: fn() = $row;` so a retired
name is a compile error, `stringify!($row)` so the compared string
cannot be mistyped, and the block a human reads — against libtest's own
`--list` through a `current_exe()` re-exec. **No Rust is parsed**, so
it is not a fresh instance of
`source-scanning-censuses-are-a-tripwire-on-ordinary-rust`.

**The review found no MAJOR and one thing worth more than a MAJOR.**
Entry 4's sense change was correct — the lane corrected a wrong header
rather than reversing a kernel claim. But the new prose it wrote
carried a false citation, in the very entry the unit exists to correct.
That is the empirical answer to the question the unit left open: the
prose column is unchecked, and **the first substantive sentence written
into it was wrong**.

**The fix pass's class check is the finding.** Told to check the other
six sentences, it found **two more wrong and one duplicated**:

- entry 3 said the row asserts `Violated`; it asserts
  `!matches!(verdict, Holds)`, and after M10-6's MAJ-1 that arm refuses
  `Unevaluated { WindowSuperset }` — the row's own doc says it "gets no
  verdict at all";
- entry 7's figures were all correct, against an item that says of them
  *"Nowhere else states them"* — so writing them here made a fifth
  site of a measurement deliberately kept to one. Numbers removed,
  pointer added;
- entry 5 overclaimed, entry 2 understated, entry 1 quoted a PR body
  it cannot read, entry 6 holds.

So of seven sentences in a column nothing computes with, **four were
wrong or misplaced on arrival**. The column survives — nothing reads
it, so it cannot make a row green that should be red — but it is now
constrained to be a string (`const _: &[&str]`, which turns `row: 42`
into `E0308`) and labelled as reading nothing.

**T2 took shape (a): the guard's name was made true rather than
narrowed to fit.** A nested `#[test]` is now a violation naming the row
and telling the author to hoist it, because
`the_header_roster_names_every_row_in_this_file` is what reaches the
PASS list, a `--filter` and every future citation. A third falsifier
now exists for it, red where it was green before the pass.

**T7 is the one this program cares about most.** `roster.rs` carried an
assertion that could not fail — a check for `": "` in rows the parse
had already stripped it from. It is not deleted: the parse is factored
out as `rows_of_listing`, and a new row feeds it a synthetic listing
with a `: benchmark` line. Mutating the parse makes the new row fail
while `the_binary_lists_this_very_row` stays green, which is the
demonstration the old assertion could not have produced.

**Six more errors the lane caught in its own diff before committing**,
listed in the PR body as a receipt rather than an assurance — among
them a claim that `harness = false` is "absent from this tree today"
when `benches/Cargo.toml:88` sets it, and a citation off by one line.
That is the first time in this program a lane's own adversarial re-read
caught what the reviewer otherwise would have.

`docs/TINT-4-SPEC.md` is deleted with this sync and recorded in
`docs/DOC-LEDGER.md`, which says what the spec got right, what it did
not anticipate (the prose column it sanctioned), and that it was the
first spec in this program written after an executed probe rather than
before one.

## The orchestrator relayed a review finding it had not checked (2026-09-15)

TINT-4's review reported that the new prose cites a **D5** that does
not exist. **D5 exists.** It is M10-6's own deviation D5 in PR #1685,
whose table reads *"`report_key` takes that tuple **plus the run
dials**"*, and whose Corrections section says *"D5 said `report_key`
'is the cache seam' while it omitted every dial and had no consumer.
Both halves are closed (MINOR-1, D11)."* The same file's OTHER D5
citation, at `:501`, is accurate and was correctly left alone.

The review's substantive finding held — the sentence's polarity was
backwards and **D11** is what put the dials in — but the "there is no
such D5" half was wrong, and this seat put it into a dispatch as an
established fact. **The lane checked it and corrected the
orchestrator.**

**Second time in this program.** The first was repeating a reviewer's
claim that seven helper-copy classes had no rows, when `fnv`/`digest`
and `missing_pairs` do. Both times the claim arrived from a lane that
had done careful work, and its carefulness elsewhere was taken as
warrant for a sentence nobody had run down. That is observation 2's
mechanism — asserting what has not been executed — with the review in
the spec's seat, so it is recorded there rather than as a fourth
observation.

**The correction**: a finding this seat relays into a dispatch as fact
gets its primary source read first, and the dispatch cites where it
was read. One API call would have settled this one.

**Do not delete `tint/2-stand-down-channel`.** It is the only ref from
which `e4adf05a1` — TINT-3's mis-based merge commit — is reachable.
The repo is merge-only and git is its archive, but a commit reachable
from exactly one branch stops being reachable when that branch goes.
Its CONTENT is on main through #2690; the merge commit itself, which is
the evidence for the entry above, is not.

## TINT-5 landed: the F6 weld got a home, and the token stopped being a promise (2026-09-15)

PR #2694, merged at `645e4d6d1` and **verified on main by
`git merge-base --is-ancestor`**. CI green on the fix-pass head: 39
jobs, 0 failures, twelve `test (…)` and five `k-lint (gate, …)`.

`assert_f6_every_variant` and `set_difference` are out of editor-core's
test binary and in `test-utils` (`f6`, `census`); mesh's inlined
predicate and topo's local `assert_f6` copy are retired; the live
defect is closed — `crates/mesh/tests/errors.rs` banned 14 identifiers
against a 15-variant enum, and the weld **reddened on arrival** naming
`["Band"]` before the entry was added. That is the first guard in this
program to red on a live defect rather than a plant.

**The unit's real subject turned out to be the weld's own token.** The
spec and the first implementation passed the exhaustiveness `match` as
`fn(&E)` beside a separately-written roster — two loose parameters, and
the review proved a **no-op token passes green**. So the doc's claim
that *"the compile error is what stands between that and an accident"*
was resting on an invariant the type could not carry, in the mechanism
this unit existed to give a home. The fix pass closed it with the shape
this program has now proven five times: `test_utils::f6_variants!`
writes the match arm and the roster entry from **the same
`$variant:ident`**, so they cannot disagree; `_` is not an `ident`, so
a wildcard is a macro grammar error; and a no-op is not expressible
because there is no body to empty. It also closed the "one hole,
stated" — an author can no longer add the arm and stop, because the arm
and the roster entry are one token. Eleven of twelve censuses are
macro-built; `SelectRefusal` is `#[non_exhaustive]` so rustc forces a
catch-all, and it goes through a named `hand_written` door that says
what it costs.

**`test-utils` held two set-difference comparators for one day.**
TINT-4's `roster::violations_against` spelled a both-direction set
comparison inline; TINT-5 promoted `set_difference` beside it. Two units
of the program whose subject is *one claim in N copies*, four days
apart, in the crate whose job is to hold one of each thing. The lane
**filed** it on three stated blockers; the review **executed** the
substitution and all three were false — the vacuity floor and the
nested-module direction are not set directions at all, they are pushed
before any comparison. `roster.rs` now calls the shared comparator and
the item is deleted. One definition, four call sites.

**The gate-boundary finding, which generalizes past this unit.**
`scripts/gates/lib.sh`'s `gate_require_crate_sources` is
`find crates/*/src -type f -name '*.rs'` — no `tests/`. **Twelve of the
tree's 22 gates take that file set**, so promoting a helper out of a
test binary into `src/` does not cross one gate boundary, it crosses
twelve. Worse, `gate_production_sources` excludes only `#[cfg(test)]`
mounts, so every file this unit promoted is **production source** to
every narrowing gate — in a crate whose own header reads *"DEV-ONLY, by
convention"*. Filed as
`work/tint/test-utils-is-production-source-to-every-narrowing-gate`.
"Give the shared thing a home" is also "move it under a different set of
gates", and nothing in the tree says so anywhere else.

## Two filed rows misstated their own defects, and one was this unit's charter (2026-09-15)

Both found by TINT-5's review, both corrected in its fix pass.

- **`dump-ban-lists-spelled-guts-…`** claimed three identifiers were
  banned that no rendering could produce. **All three are live**:
  `EvalError::UnknownParam` is case 1 of the row's list,
  `Diagnosis::PredicateFlip` is the `ResolveError::Vanished` payload,
  and `BifurcationKind::AmbiguousBasin` is the `WitnessBifurcation`
  kind. The filing read "either enum" over a list that renders **five**
  types and reported the residue as dead. What survives: the list is
  badly incomplete and unwelded, and `MarginDiag` is genuinely dead.
- **`sibling-display-contract-suites-hand-mirror-their-enums-too`** —
  the row this whole unit was cut from — said the missing `Band` entry
  was *"the one identifier that would catch it going back to `Debug`"*.
  The pre-merge check banned `{` unconditionally and `Band` is a struct
  variant, so a full `Debug` regression reddened on the brace whatever
  the roster held. The real gap is a **brace-free** leak of the word
  `Band`: smaller, and still real.

A row that misstates its own defect is this program's charter shape
committed in this program's own paperwork, and it is now the fourth
distinct way that has happened (see `process-observations.md`).

## The orchestrator shipped three wrong counts into a dispatch (2026-09-15)

The D5 entry above adopted a correction: a finding relayed as fact gets
its primary source read first. **It did not cover numbers, and numbers
are where it failed next.**

TINT-5's review reported *"thirteen of the 23 gates"* take
`gate_require_crate_sources` — and listed **fourteen** names under that
claim, inconsistent on its own line. This seat relayed it into the fix
pass dispatch without counting, and added a *"all 23 gates"* of its own.
Both wrong: **twelve** gates call it (`gate-roster.sh` mentions it only
in a comment about a vacuous green; `kernel-serde-free.sh` says outright
it does not use it, its subject being manifests), and there are **22**
gates (`README.md`, `lib.sh` and `viewer-readme-fence.awk` are not
gates). The dispatch also said ten `roster::tests` rows where libtest
lists nine. The lane re-derived every number from `grep` and `--list`
and corrected all three.

**Fourth time today a number passed downstream came back corrected**,
and the second where this seat introduced the error rather than
forwarding it. The correction, widened: **a count in a dispatch is
derived by the command that produces it, and the dispatch carries the
command** — not the number alone, so the reader can re-run it. A
sentence and a number are the same kind of claim, and this program
exists to say so.

## TINT-6 cut — and the probe refused the obvious shape (2026-09-15)

All five units of the first wave are on main, each verified by
`git merge-base --is-ancestor` rather than by a merge API's
`merged: true`. The slate is open at ~55 rows with nothing in flight.

**TINT-6** — `docs/TINT-6-SPEC.md`, branch `tint/6-interrogate-ladder`,
one row: `interrogate-ladder-header-claims-every-rung-and-pins-five`.

**The probe ran before the spec, and killed the shape this seat reached
for.** The row was routed to TINT-1's exhaustive-match shape this
morning, and `f6_variants!` landing in TINT-5 made that look strictly
better. It is not the shape, because the suite pins a rung by asserting
a DOOR'S RETURNED ERROR — so a roster of covered rungs has to accumulate
across rows, and **nextest runs each row in its own process**. That is
TINT-2's measured wall (pids 12156 and 12157, a shared `static
AtomicUsize` reading 0 in both), one row over. A spec proposing it would
have been this seat's third naming a mechanism it had not executed.

**What survives**: one row driving every reachable rung through its own
door in a single process, welded to the enum by `f6_variants!`, so
isolation is irrelevant. The spec names the measurement that decides its
size — are the four unmeasured rungs reachable from a door one test can
call — and says **both answers are legitimate**, because the honest
outcome may be that the header narrows rather than that the suite grows.
`NoBodies` is excluded by name either way; SHELL's row is still open, so
pinning it would pin a defect.

**Three probes, three corrected groupings.** The roster class was not a
class; TINT-5's weld turned out to have no home before it could have
adopters; and this one's obvious mechanism does not exist. The
discipline observation 2 adopted — a spec names the measurement that
would show its mechanism cannot work, and the lane takes it first — has
now been improved on by taking the measurement BEFORE the spec exists,
three times running.

**Seam announced by BLEND (2026-09-17, at unit 15's fix pass):** BLEND
unit 15 (`docs/BLEND-15-SPEC.md`, PR #2514) adds ONE source reader to
`crates/test-utils/src/source.rs` — the `decide*` call-site roster
reader its two `recourse_roster.rs` suites and `profile`'s
`fillet_recourse_followability.rs` census currently carry as three
hand-rolled copies, which both v6 reviewers defeated by mutation (a
turbofish, a wrapped carrier) — beside `plain_string_literal` and
`balanced_end`, which it uses. No test mechanism changes; the
`reader_census.rs` ledger gains the lines the gate demands. Announced
here and in `work/tcost/log.md` because `crates/test-utils/*` is both
programs' ground by declaration; a row justified by a claim that
cannot fail is S-TINT's — this reader exists so a hand-rolled census
cannot stay green on a name it did not read.

## One row arriving from DOOR, 2026-09-20

Ev ruled in chat on 2026-09-20 that FIX carries no design decisions, and
DOOR was swept on the same rule in the same sitting: it claims no paths,
so it can never be the owning track for a decision, and four rows left.

**`unit-symbol-proptest-generators-under-cover-with-no-file`.**
`crates/editor-core/src/expr.rs`'s "what an added unit symbol costs" walk
discloses it in prose: `tests/u8a_parse.rs`'s two `prop_oneof!`
generators (`:482`, `:725`) enumerate the unit symbols by hand and do NOT
go red — *"they silently under-cover, so they want an edit that nothing
announces."* The file exists because a disclosure is not a schedule.

It lands on S-TINT because `crates/editor-core/tests/u8a_parse.rs` is
S-TCOST's and S-TINT's by territory, and a generator that silently
under-covers its domain is test-suite **integrity** rather than cost —
beside `anti-vacuity-floor-cannot-go-red-on-degradation` and
`census-answers-no-field-read-for-a-walk-that-reads-a-field`, the same
defect in other instruments.

**Why it is a decision and not a written fix**, which is what moved it:
the row is explicit that the answer is *not a list to project* but *"a
way for the generator to draw from the symbol table itself"* — a
direction, not a diff.

**Two things to carry.** It shares those two `prop_oneof!` blocks with
DOOR's `dimension-all-has-readers-outside-the-viewer` and nothing else —
that row is a mirror of a closed four-variant enum a published `ALL`
retires, this is a generator over an OPEN, growing set — so do not merge
them, and read both if you are inside those blocks. And the population is
unmeasured: whether these two generators are the only under-covering
enumerations of the unit symbols is the source comment's claim, not a
measurement, so the sweep is owed.

Signed (DOOR orchestrator).

## Announced seam from FIX (2026-09-21)

**`crates/quantity` now dev-depends on `test-utils`, and one module-doc
clause in `crates/test-utils/src/source.rs` moved with it — PR 2944.**

FIX's `quantity-fmt-error-display-row-is-a-verbatim-copy-of-assert-f6`
folded a hand-spelled F6 display row onto the shared door.
`crates/quantity/Cargo.toml` gains `test-utils` under
`[dev-dependencies]` (where `proptest` already sat); `[dependencies]`
stays empty, so the crate's stated leaf property — which is about what a
DEPENDENT carries — is untouched, and the manifest now says that in a
comment rather than leaving the next reader to work it out.

**The wheel closure does not move, and this was measured rather than
reasoned.** `scripts/ci-filter.py`'s `pncad_py_seeds` is the NORMAL
dependency closure, so a dev edge is not followed: called on the tree
before and after the manifest edit it returns **16 members both times,
`quantity` in, `test-utils` out**, identical to the seed set the run
printed. (CI's own `RUN_PNCAD_PY=true` on this PR is *not* evidence of
that — the diff touches `Cargo.lock`, so the filter falls to `TIER=all`
and sets the flag fail-closed without reaching the seed arithmetic. The
lane distinguished the two, which is why this paragraph can say
"measured".)

**The doc clause.** `source.rs`'s module docs listed the crates that do
NOT dev-depend on `test-utils` — *"`pncad`, `pncad-py` and `quantity` do
not"* — which this change makes false. It now names `pncad` and
`pncad-py`. That is a sentence re-worded because an approved change
moved what it describes, so it lands with the change rather than waiting
on anything; the clause's point (that `pncad/tests/all.rs` holds the
class's largest unconverted reader) is unchanged.

The row it folds onto is the census form, `assert_f6_every_variant` with
a `f6_variants!` roster — not bare `assert_f6`, because that would have
wanted a hand-typed dump list and re-minted what
`assert-f6-dump-lists-are-hand-written-mirrors-of-error-enums` closed.
The compiler now forces the roster; proved by planting a sibling arm and
watching `E0004` fire.

Signed (FIX orchestrator).

## Announced seam from FIX (2026-09-21), and a case ADDED to one of your suites

**PR 2945**, FIX's `remap-name-misses-lose-the-id-they-caught-at-six-refactor-sites`.
Three refusals in `crates/editor-core/src/refactor.rs` (FIX's own) gained
a `missing: RecipeNodeId` field, which makes two of your files not
compile until their patterns bind it:

- `crates/editor-core/tests/asm4_split_inline.rs`
- `crates/editor-core/tests/edit_instance_crossing_names.rs`

The pattern updates are mechanical. **One change is not, and it is the
reason this note is longer than a pattern update deserves.**

`asm4_split_inline.rs`'s `StrandedPartName` case now runs **both** name
shapes over one shared setup — flat (minted AT the deleted node,
asserting `missing == extra == name.node`) and nested (minted at the
surviving body with the deleted node's name embedded in a `FromA`
segment, asserting `missing == extra` and `missing != name.node`).

**The lane's first push REPLACED the flat case with the nested one, and
the orchestrator sent it back.** The lane's argument for replacing was
sound on its own terms: under a flat name the failed node and the name's
own mint coincide, so that row was blind by construction to the defect
the unit is about and could never have gone red on it. But narrowing
which shapes your suite covers is not a call a FIX unit gets to make as
a side effect — **a case removed for convenience is invisible to its
owners once it merges; a case added is not.** So both run now, +17 net
lines, no restructure, nothing else in that file touched.

The pair is also the better pin: the id and the name coincide in the
ordinary shape a user hits and come apart in the nested one, which is
exactly why the name alone cannot answer "which node stranded".

`edit_instance_crossing_names.rs` additionally asserts
`missing == keeper`, with the reason recorded in the assertion message
(`walk_names` does not descend through `InPart`, so the one LOCAL node an
instance-qualified name derives from is the instance).

Signed (FIX orchestrator).

## Announced seam from FIX (2026-09-21) — PR 2948

FIX's `recourse-chain-stops-at-the-second-hop-carriers`, the last row
of its wave-4 slate. An arm whose `Display` renders a carried error
whole contributes no recourse of its own, so *"this message names a
repair"* is a claim about the carrier all the way down. Four carriers
gained repairs and an enforcement row each, every repair grounded in the
module's or the variant's own docs rather than invented, and all of them
**proved red by mutation** (run 35548044980 — twelve `test (…)` jobs
red, failure surface exactly the intended rows).

**Your file, one assertion loosened — and the repo'"'"'s own convention is
why.** `crates/geom/tests/curves/domain_door.rs` pinned
`SplineError::DomainInvalid`'"'"'s whole rendering with a full-string
`assert_eq!`. That is the spelling `COINCIDENCE_RECOURSE`'"'"'s doc rules
out in as many words — *"message-pinning tests pin the fragment with
`contains`, never with full-string pins that rot"* — and it made the
new recourse clause literally unwritable. It is a `contains` pin on the
same sentence now, with the reason recorded at the site.

**Checked at the const'"'"'s home before accepting it**, because this seat
sent another lane back this same wave for narrowing a suite'"'"'s coverage.
The two are different: that one REMOVED a case for convenience; this
corrects a pin that contradicted a documented convention and blocked an
approved change. A clause re-worded because the change moved what it
describes lands with the change.

Nothing else in your trees is touched, and the five new enforcement rows
are the first pins those five types have ever had.

Signed (FIX orchestrator).

## Announced seam from DOOR (2026-09-21) — PR 2986, one case ADDED

**`crates/editor-core/tests/asm2a_instantiate.rs`** gains one test and
loses nothing. DOOR's `part-fault-partproduct-degrades-the-product-refusal`
typed `PartFault::PartProduct` (it carried a `String` where a
`ProductErrorKind` now sits beside it), and **nothing in the tree
constructed or asserted that arm** — not in `editor-core`, `pncad`,
`pncad-py`, `viewer` or the demos. So the unit owed its own pin.

`a_gather_refusal_crosses_as_its_class_beside_its_sentence` instantiates
two part documents that refuse the gather for different reasons — one
with no body-denoting root (`NoBodyRoots`), one whose only root is
poisoned through a failed ancestor (`RootPoisoned`) — and asserts both
arrive as `PartProduct` with **different** classes, that
`means_no_body` answers differently for them, and that the gather's own
sentence still travels beside the class. Mutation-checked: hard-coding
the call site to one class reds the poisoned case, so it is a row a bug
breaks rather than a compile-time restatement.

**Why it is in your suite rather than the source file**: `parts.rs` has
no `#[cfg(test)]` module, and the row needs the stub resolver and two
instantiated part documents that already live here beside the sibling
arms' pins. The lane named this as the call it was least sure of; the
orchestrator agrees with it, on the line this wave already drew —
**adding a case to another program's suite, announced, is ordinary;
narrowing one is not a side effect a unit gets to have.**

Signed (DOOR orchestrator).

## 2026-09-22 — announced seam from VGEOM: `crates/viewer/tests/` moved by the render-grid unit

(VGEOM orchestrator. Announcement, not a request — nothing here asks
this program to schedule anything.)

`vgeom/render-grid` (#3068) replaced `crate::readout`'s render
tolerance with an ε-derived one: `min(DEFAULT_EPS * 0.1, |value| *
REL_TOLERANCE)`, a cap one decade below ε met with the existing
relative arm. `readout::MAX_CHARS` went `10 → 22` and `pane::view`'s
`FIELD_WIDTH` `88 → 176`, because the module's own rule is that a box
meets the number rather than the number meeting the box.

**What that did to `crates/viewer/tests/`, which is this program's
ground:**

- `display_budget.rs` — three expectation moves, and a **hand-rolled
  copy of the read-back predicate deleted** in favour of
  `readout::reads_back`, which widened to `pub` for it. That is one
  fewer undisclosed duplicate of the rule; it is also new public
  surface on the crate, which is the half worth this program's
  attention.
- `panel_display.rs`, `valid_range.rs` — expectation moves only.

**Why the crossing rather than a filed row:** these rows asserted
texts the diff changes, so leaving them would have reddened `main`.
A test whose claim a diff falsifies moves with that diff or the gate
goes red; there was no version of this that files instead.

**What a reader of those files should know**: no assertion in them
names a spelling as a literal any more where the grid could move it.
The property that survives at the widget seam is that a drag's text
parses back to exactly the value the drag commits, asserted over
about 9000 magnitudes in `widgets.rs`'s own module.

## TINT-6 landed, and the lane caught its own minting (2026-09-22)

PR #2707, merged at `ccf32a73d` and **verified on main by
`git merge-base --is-ancestor`**. CI green on the fix-pass head: 39
jobs, twelve `test (…)` points, five `k-lint (gate, …)`, 0 failures.

The header claimed ten rungs and the suite reached five. It now drives
eight through a door and excludes two by name **with a guard that
re-takes the measurement every run** rather than a sentence recording
it. The row's own `## Closed` section carries the per-rung answers and
the structural reason the two negatives hold.

**The unit's own account is the headline: the lane minted six instances
of this program's subject and caught all six itself**, on two cold reads
of its own diff, before pushing. Six units in, that is the first time
the catcher was not an outside reader. `process-observations.md`
observation 1 now reads seven instances across five of six units, and
records what actually worked: not the spec's warning — TINT-6's spec
warned too — but reading the diff twice at different framings, the
second pass off `git diff --cached`, which found as many as the first.

**The five-day gap changed one number and the lane reported it rather
than absorbing it.** `origin/main` moved 5145 commits under this branch.
Re-derived on the merged tree: `InterrogateError` still ten variants,
`BlendError` still 23, the corpus sweep still 13324 / 352 / zero / zero.
Moved: `BodyNotIntact`'s sibling-suite count 2 → 3, and the
square-literal sweep 30/24 → 47/39. Both are in the PR body with the
commands that derive them. A lane that had trusted its six-day-old
numbers would have shipped two wrong ones and never known.

## The orchestrator relayed a review's claim into a dispatch, again (2026-09-22)

TINT-4's D5 entry adopted a correction — a finding relayed as fact gets
its primary source read first — and TINT-5's entry widened it to counts,
because counts were where it failed next. **It failed a third time, on
the same mechanism, in TINT-6's fix-pass dispatch.**

The style review reported that `select`, `select_where`,
`find_flush_candidates`, `declare` and `declare_all` are all public and
*"all surface an `InterrogateError` whole"*. This seat put that list
into the dispatch as established. **Three of the five do not surface one
at all**: `select` returns `Vec<StableName>` and cannot refuse;
`declare` and `declare_all` return `DeclareError`, whose arms are
exactly `NoFindings`, `Edit(EditError)` and `NoMintedId`. Only
`select_where` and `find_flush_candidates` qualify, plus the measure
wire's `MeasureRefUnreadable`. The lane derived that by grepping every
`InterrogateError` under `crates/*/src/` and corrected the dispatch; the
orchestrator then verified both facts at the source.

The review's CONCLUSION survived — the narrow reading is the right one
and the third sentence was false under the broad one — which is exactly
why the list went unchecked: a claim whose conclusion is right reads as
a claim that is right. **The rule does not get narrower each time it
fails. It is: anything this seat puts into a dispatch as established, it
has run down itself, and the dispatch says where.**

## A counting habit of this seat's, corrected at the source (2026-09-22)

Every dispatch this program has written says to expect **twelve
`test (…)` jobs** and to say so if fewer appear. On today's `main` the
interval lane runs through a called workflow, so its six points are
named `interval / test (interval, eps = …)` and **do not start with
`test (`**. Both the fix-pass lane and this seat counted six on a
complete matrix and went looking before reporting a narrowing.

`docs/prompts/implementer-discipline.md` §2 already covers this — *"a
lane that moves into a called workflow has its jobs prefixed with the
caller's key, so a reader matching the start of a name sees a fraction
of a full matrix and reads it as a narrowing"* — and says to establish
narrowed-or-not **from the `change filter` log, not by counting job
names**. The stale text was this program's dispatches, not the repo's
discipline. The lane reported §2 as describing the old naming; it does
not, and that was checked rather than relayed.

## Handoff: the program goes back to `ready` (2026-09-22)

Six units landed, all verified on `main` by
`git merge-base --is-ancestor` rather than by a merge API's word:
TINT-1 (`assert_f6` ban lists), TINT-2 (the stand-down channel), TINT-3
(fifteen aggregation guards onto one macro), TINT-4 (a header roster
welded to its rows), TINT-5 (the F6 weld's home, three adopters), TINT-6
(the interrogate ladder driven through its doors). No unit is in flight.

**What works, and a successor should keep doing it.** Probe BEFORE the
spec, not after: three groupings this seat proposed were corrected by a
probe that ran first — the roster class was not a class, TINT-5's weld
had no home before it could have adopters, and TINT-6's obvious
mechanism did not exist (a cross-row roster, which nextest's
process-per-test rules out; TINT-2 measured that wall and the spec cited
the measurement rather than re-deriving it). Each probe cost under an
hour and each saved a lane from a spec that could not work.

**What keeps going wrong, and is why this program changes hands.** The
orchestrator's characteristic failure here is putting a claim into a
spec or dispatch that it has not run down. It happened five times in one
sitting — a fabricated-looking D5 citation that turned out to exist, two
gate counts, a roster-row count, a list of five public doors of which
three do not surface the error at all, and a set of sweep notes whose
self-descriptions were read as evidence. A correction was adopted after
the first and widened after the second, and then violated twice more.
Three of this program's six specs have misled their lane. **The rule
that survives: anything an orchestrator states as established, it has
derived itself, and the text says with what command.** A downstream seat
caught every one of these, which is the system working — but it is
cheaper to be right.

**Queued, not blocked.** `docs/TINT-6-SPEC.md` is still in the tree. Its
deletion waits on `ledger/pointer-notes` (#3063), which deletes
`docs/DOC-LEDGER.md` and moves entries to `docs/doc-ledger/` as short
pointer notes; once that lands the deletion is a four-line note and one
commit. Nothing else is outstanding.

**The slate** is ~55 open rows. The nearest neighbours of what just
landed: `sibling-display-contract`'s residue,
`test-utils-is-production-source-to-every-narrowing-gate` and
`topo-display-contract-rosters-could-be-derived-beside-the-enum` (both
filed by TINT-5), and
`dump-ban-lists-spelled-guts-are-a-fourth-copy-and-two-are-dead`, whose
numbers were corrected by TINT-6's fix pass and are now right.

`process-observations.md` is the file to read before cutting anything:
seven instances across five of six units of a unit minting its own
subject, and what has actually caught them.
