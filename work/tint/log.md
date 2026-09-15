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
