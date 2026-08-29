# SMELL-KPW — execution log for Tracks K, P and W (plus X's remainder)

**What this is.** The orchestrator log for one session's execution of the
`docs/SMELL-SCAN-2026-08.md` schedule over the three tracks nothing else was
sitting on. It records decisions made unilaterally and the state of work, per
`memories/orchestration-model.md`; it is not a plan and it is not a second
schedule. **The schedule is `docs/SMELL-SCAN-2026-08.md` §D and stays so** — a
row's live status is that file's table, and a landed row leaves it.

## The ground, and how it was checked

Taken 2026-08-29 against `origin/main` at `9337219`. Track K, P and W were
chosen because no live branch sits on their fences — verified rather than
assumed, by diffing every remote branch pushed in the preceding hour against
`origin/main` and filtering for each track's paths. Two qualifications came out
of that check and are stated because the premise they qualify is the reason
these tracks were picked:

- **`scripts/gates/bounds-allowlist.sh` IS contested.** Two live branches edit
  it (adding allowlist entries). Track K's `D102`, `D103`, `D106` and `D109`
  are its rows, and `D106` is a restructure of that file. **Not taken this
  session**; they wait for those branches to land.
- **PR #1169 has already taken four rows** off these tracks — `D200` (K),
  `D67`/`S123` (W), `D400`/`S129` and `D401` (X). They are excluded here.

## Protocol for this session

No A/B protocol (Evan, this session). Every unit gets a **style review**
against `docs/prompts/reviewer-style-lane.md`, briefed with one standing
emphasis: **does the defect the unit closed reappear in a slightly different
form** — §D rule 5's finding, which held eight units out of eight on Track F
and every unit on Track G. Units where a wrong answer is reachable also get a
normal **adversarial correctness review**.

Lanes run in worktrees under `/home/user/lanes/`, do not push, and do not edit
`docs/SMELL-SCAN-2026-08.md` — that file conflicts by construction when lanes
run concurrently, so its bookkeeping is the orchestrator's and rides the
integration branch.

## Decisions made unilaterally

### 1. Two rows are mis-fenced, and the partition rule decides both

§D's partition rule is *"the fence is the file, not the subject"*. Two rows
were placed by subject:

- **`D90` is Track T's, not Track P's.** `octant_chart` is defined at
  `crates/sweep/src/fillet/build.rs:201` and consumed from
  `crates/sweep/src/fillet/surgery.rs`. Track P's fence is eleven named files
  under `crates/topo/src/`; none of them is a `sweep` path. Track T's fence is
  `crates/sweep/`. **Moved to Track T**, keeping its number, its **ADV** mark
  and its provenance — this is a fence correction, not a re-verdict, and §D's
  *"nothing below is closed, re-scoped or re-argued by being moved"* is the
  same operation this partition already performed once.
- **`D107` is on Track W and its ground is `src/`, which no track owns.**
  `review_d18` is `crates/topo/src/review_d18.rs` and
  `crates/topo/src/review_d18_probes.rs`; Track W's fence is `crates/*/tests/`
  plus `crates/test-utils/`. Neither Track P's eleven files nor Track Q's six
  paths name `review_d18*`, so this is the **`geom-brep` hole again** — the one
  §D already had to state explicitly after `C23` turned out to be executable by
  nobody. The partition rule says a row's work reaching an unowned path is not
  a licence to edit it; it is a fence that has not been drawn. **Drawn here:
  `crates/topo/src/review_d18*.rs` and `crates/topo/src/fixtures.rs` go to
  Track P**, which already owns the euler operators `review_d18` hammers, and
  `D107` moves with them.

Both corrections are recorded in `docs/SMELL-SCAN-2026-08.md` in the same
change as this log, per §D rule 2 (*a scope line that lags its lane's diff
silently mis-fences someone else*).

## Units

| Lane | Rows | State |
|---|---|---|
| `k1` | `C15` / `S73` — `tess-lint`'s ordinal join (#746) | implemented; style + adversarial review running |
| `k2` | `D105` / `S160` + `D64`(a) and (c) — `tess-meter`'s split-scan guard | dispatched |
| `p1` | `D38` + `D88` — `merge_faces.rs`'s two failure regimes, and `absorb`'s discard | dispatched |
| `w1` | `D65` / `S121` — bound-domination rows with no ceiling and no floor | dispatched |

## `k1` — what it found beyond its row

**The committed tessellation baseline is stale, and the gate it belongs to could
not say so.** `docs/tess-budget-data/tess-budget-baseline.csv` describes
`teapot/teapotvessel` as 28 faces; the sweep at this head produces 25, four
coplanar planes having merged into one (25+25+22+22 = 94 triangles). Verified
by the orchestrator independently: the diff against a fresh sweep is that one
scene and nothing else, sweep-wide triangle total unchanged. The old gate reads
this as 0 findings — which is the mis-join `C15` describes, caught in the wild
rather than in a fixture.

The re-cut is a regenerated golden, which
`memories/output-stability-as-justification.md` calls *"a chore, not a
contract"*, and it must ride the same change as the gate that notices — landing
the gate alone lands a red row. **`docs/tess-budget-data/` is in no track's
territory**, so the orchestrator takes it; that is a fence being drawn, not a
lane crossing one.

One qualification stated rather than buried: `teapot/teapotvessel` has **zero**
Hessian-sized faces on either side, so this instance of the mis-join voided no
measurement. The committed evidence therefore does not exercise the case the row
is actually about, which is why `k1`'s correctness review is briefed to
construct one.

**Residues owed rows on Track K** (numbers minted at integration, from the
`D200`–`D219` block; #1169 has taken `D200`):

- `tools/tess-meter`'s `face_rows` holds a `topo::FaceKey` and writes only
  `enumerate()`'s ordinal, so the CSV carries no stable face identity at all.
  Minting a durable one is a design question reaching `crates/topo` and
  `demos/`, not a column rename.
- `tools/tess-meter`'s `nurbs: by_face.get(&patch.face).map(…)` turns a
  *missing* measurement into *"this face is not on the sized lane"* — the same
  silent-coverage-loss shape `C15` just closed, one step upstream.

`docs/TESS-BUDGET.md`'s enumeration of the gate's rules as three is falsified by
this change and is corrected with it (Q4's first sub-case: the doc rotted, the
code is right).

## `w1` — what it found beyond its row

All five in-fence `D65` sites got a measured ceiling and an anti-vacuity floor,
each red-proved by a perturbation that degrades the guarantee. Two results are
worth more than the row:

- **`r1_pxn_probes.rs` admits no honest *ratio* ceiling, and the measurement is
  why.** `hull_sup/truth` falls monotonically as the amplitude rises
  (inf → 4.58 → 1.10) because every amplitude the fixture probes sits at or
  below the lane's own enclosure floor — so the ratio measures that floor, not
  the envelope. The row is now guarded additively instead. `S121` licenses
  *"no honest ceiling here"* as a passing answer; whether an additive absolute
  constant is an improvement or a re-pin wearing a threshold's clothes is the
  question its style review carries.
- **The sweep for the shape found ten more sites**, in seven files across three
  crates, after the finding's own sweep had filed five and a reviewer had found
  three of those. That is `S121`'s own history repeating: a single-shaped sweep
  under-reports this class. One row on Track W at integration, since the
  deliverable is identical per site.

The `D65` members are closed; `S121`'s `mesh/src/nurbs_cert.rs` member is Track
R's `D300` and stays.

## `k1`'s reviews, and the rule the adjudication turned on

Both reviews broke the same claim, from opposite directions. The precondition
`k1` built is a check on the **per-ordinal lane sequence**, not on the roster:
the adversarial lane constructed a roster move that preserves it — one wall
removed, a different one added inside the same run — and the gate stays green
over a surviving face that regressed **+10.9%**, double the growth tolerance,
across eight real measurements. The style lane measured why that is general
rather than a corner: of 70 scenes, exactly **12 carry a sized face and every
one of them has 4 or 8 NURBS faces**, so in every scene where the slack rule
can fire, at least four faces are mutually indistinguishable to
`(chart, sized)`. **The disclosed blind spot was the whole gated population,
disclosed as an edge case** — §D rule 5's shape, and neither the author nor the
dispatcher saw it.

**The ruling, and it is the part worth keeping past this row: a join's
precondition goes over the columns the comparison does not read.** `chart`,
`nu`, `nv` and the trim box are surface-and-trim identity and are compared by
no rule, so a precondition over them is not circular — which is the ground on
which `k1` had correctly rejected them as a *key*. The distinction the first
pass missed is that a key needs uniqueness and a pointwise precondition does
not, so the eight collisions that disqualify one are harmless to the other.
The alternative the adversarial lane proposed — a multiset over the *measured*
columns — is rejected for the reason `k1` originally gave: it fires on every
change the gate exists to measure.

Two further decisions taken by the orchestrator rather than the lane:

- **Findings below the first disagreement are reported, not discarded.** They
  are provably still aligned, and suppressing them let a 30% slack regression
  hide behind a roster finding whose printed recourse is *"re-cut the
  baseline"* — the fix re-minting rule 3's own silent-coverage-loss shape.
- **The roster rule reds only where it can cost a measurement**, and reports
  elsewhere. The lane's justifying analogy does not hold: a vanished scene
  loses the triangle rule too, so it always announces lost coverage, whereas a
  roster move in an unsized scene loses nothing because that rule still runs.
  As shipped it would red-line 58 of 70 scenes where it gates nothing, and
  cry-wolf-then-allowlist is a measured outcome in this tree, not a hypothesis.

## `w1`'s reviews — the re-mint, and an operational finding about this session

**The style lane caught the re-mint cold: `crates/test-utils/src/vacuity.rs`
already exists.** Its `Exposure::require` is the unit's hand-rolled
`certified > 0` guard exactly, down to the failure vocabulary; its module doc
calls itself *"the anti-vacuity floor — the tree's spelling"*; `geom-brep`
already carries `test-utils` in dev-deps and the sibling suite **in the same
test binary** imports it. So the Q1 framing this orchestrator gave the reviewer
— *"N copies and no shared thing"* — was itself wrong, and the reviewer
corrected it: it is N copies **beside** a shared thing that half the tree
already imports. That correction is the finding.

**The correctness lane then produced the decisive experiment.** Rather than the
blanket multiplier the implementer red-proved with, it constructed the real
degradation — hull-then-difference in `spline/compose/tensor.rs`, the
span-width-scaled enclosure the comments themselves name — and measured it:

| row | today | cancellation lost | ceiling | outcome |
|---|---|---|---|---|
| aligned (pre-existing) | 1.4912 | 20.843 | 10.0 | RED |
| knots | 1.5199 | 6.730 | 3.0 | RED |
| straddle | 3.2639 | 5.108 | 6.0 | **GREEN** |
| bicubic | 1.4434 | 1.809 | 3.0 | **GREEN** |

Two of the three rows the unit added stay green under exactly the degradation
their own assert messages name. **A ceiling red-proved by a blanket
multiplicative loosening has been shown to catch loosening, not the named
mechanism** — that is the durable rule out of this unit, and it generalises past
`D65` to every threshold in the tree whose red-proof was a scaling.

The two rows want different answers, which is why the rule is not "tighten the
constants": `straddle` takes `4.0` with 22.5% headroom, while on `bicubic`'s
geometry a *total* cancellation loss costs only 1.25×, so **no ratio ceiling
separates the two states at all** and the honest close is `S121`'s licensed
written verdict. The unit's *"no honest ceiling"* verdict elsewhere
(`r1_pxn_probes`) was over-broad in the other direction: extending the amplitude
ladder shows the fixture certifies to `a = 1e-10`, three orders above the
enclosure floor the verdict rests on.

### Operational: two reviewers in one worktree contaminate each other

The style reviewer got a confident three-leg red that was the correctness
reviewer's in-flight uncommitted `src/` perturbation, and later hit its
uncommitted `println!`s. **An unbracketed run in a shared worktree produces a
detailed, plausible and wrong MAJOR.** This is a defect in how this session
dispatched, not in either reviewer: they were put in one worktree to save disk.
The standing rule it earns — the mutating reviewer works in its own checkout,
and any reviewer sharing a worktree brackets every measurement with
`git diff --quiet HEAD` before and after — is relocated into
`memories/agent-lane-operations.md` at integration. Both live pairs were warned
mid-flight.

## A merge that silently under-counted, and it did not conflict

Merging `origin/main` (carrying #1169) into this branch conflicted on exactly
one line — Track X's item count — and **the damage was on a line that did not
conflict**. #1169 closed a Track W row and wrote `14 → 13`; this branch had
moved `D107` off W and written `14 → 13` independently. Identical text, so git
took it once and the merged tree declared **13 against 12 actual rows**.

This is `memories/agent-lane-operations.md`'s stated rule firing in a form the
rule does not quite name — it warns that resolving a conflict where *both sides
deleted* something must take the union derived from `main`, and that the
post-condition is checked **against the merged tree, not against your diff,
because a row you never touched cannot appear in your diff**. Here there was no
conflict to resolve at all: two independent decrements of one counter collided
into one. The check that caught it is the one the rule prescribes — re-derive
every track's count from its own table after the merge, never transcribe. Both
counts and the 100-item total now agree with the tables.

## Rows filed from `k1`'s residues, minted before the unit lands

`k1`'s delta review flagged that the rows the lane wrote were **not in the
tree** — the branch touched six files, none of them this schedule — and named
the precedent: a lane that claimed its record edits and shipped without them.
That is this session's process rather than the lane's omission (lanes are
barred from `docs/SMELL-SCAN-2026-08.md` because five of them would conflict on
it by construction), but the exposure is real and the answer is to mint the
rows when they are established, not at the end. Six are now in the tables:
`D181`, `D182` (Track J — the two copies of *"what the budget gate reads"* that
live on a fence `C15` could not cross), `D201`, `D202`, `D203` (Track K), and
`D402` (Track X). Every track's count re-derived from its own table; total 106.

`D203` is the orchestrator's, not a lane's, and it is a **rule rather than
either instance**: `C15` and `D200` independently discovered that a per-column
admissions table cannot state a cross-column invariant, answered it the same
way in two crates, and neither site says the other exists. `k1` declined the
`u0 < u1` check as *"`D200`'s shape, not this row's"* — correctly — and that
declination is what made the class visible.

## `p1`'s reviews — the finding itself was false

The adversarial lane instrumented the regime decision on both trees and ran the
corpus: **3035 decisions, 24 REFUSE / 3011 SKIP on each, sorted multiset diff
empty.** The partition did not move. Then it broke the row.

**`D88` said `absorb` *"drops every ring of an absorbed face and returns `Ok`"*.
It cannot return `Ok`.** The very next statement, `kef(dying_he)`, recomputes the
identical key — `loops[half_edges[dying_he].parent_loop].face` — and refuses
`StaleKey` at `euler_kill.rs:802`, in its **plan phase, before any mutation**.
The `unwrap_or_default()` was fully shadowed dead code. And the unit's two new
probes do not pin its fix: reverting the hunk leaves **both green**, because they
pin `kef`'s precondition. A finding that stood through a scan, a steelman, a
placement and an implementation was falsified by the first reader who executed
it. The rewrite is still an improvement — it removes a discard that *looked*
live — but that is a smaller claim than the row's, and the smaller claim is the
true one.

**The ruling the two reviews forced together: a kernel-bug refusal escapes the
regime split.** `GroupRegime` governs *inventory* failures — this group cannot
be merged — and a stale key is not an inventory fact but a statement about the
arena. As shipped, `Op{StaleKey}` under `RecordsASkip` becomes a skip record and
the door returns `Ok`, and the unit **widens** the set riding that path. It
changes no reachable behaviour (0 ERR in 3035 decisions); it changes what the
door promises, which is what `S19` is about.

**And `group_regime`'s new doc promotes a false premise into a proof.** *"A
group's members share a surface by construction"* — `planes_declared_equal`'s
rung 2 is kind-agnostic since M5 PR 9 and glues different surface keys on
`surface_source` identity alone, through a `pub` door that validates only key
resolution. A Plane/Cylinder pair passes silently in debug and release, and the
group's regime is then decided by **arena order**. Pre-existing, and worse as a
documented proof than as silence.

### `docs/DESIGN.md:483` is false about the shipped kernel — Evan's, not a lane's

It says `merge_coplanar_faces` *"never elides vertices (collinear vertex chains
survive)"* and that tier 3′'s strict record-drop rule is correct **because** of
that. `sweep/tests/verbs_f7_r2_probes.rs:212` already asserts `v1 == v0 - 1`
after a bare `merge_coplanar_faces` — the elision is real and pinned through the
public door. And the rule's justification is broken, not merely its sentence:
`Descendants::absorb_merge` consumes only `group.absorbed` and never
`killed_vertices`, so a `kev`-killed vertex reaches `remap_contacts` as a dead
key with no lineage row and its records drop as *"genuinely consumed"* — the R5
record-carriage class `DESIGN.md` says is closed because of the false premise.
Whether a declared contact vertex can land on such a junction is unconstructed.
**Ratified design is a fork, so it waits for sign-off**; raised to Evan rather
than fixed, and no lane touches `DESIGN.md`.

## `C15` landed — three review rounds, and what each one cost

Merged into the integration branch: `fmt`, `clippy -D warnings` and 38 + 10
tests clean on the merged tree, and the full gate as `ci.yml` runs it (fresh
release sweep, then the lint against the re-cut baseline) at exit 0, 0 findings,
0 notes, nothing on stderr.

**Every round found something the previous round's author could not see, and
the last one changed a mechanism rather than a message.** Identity had been
compared as **rendered text**, and `format!("{:?}", -0.0)` is `"-0.0"` while
`-0.0 == 0.0` — so a signed-zero extent would have announced a re-key and
stopped a scene's comparison, in the two columns whose live value is exactly
zero on all 64 sized rows. It is now compared as numbers, with rendering kept
only for the message. Reachability was never established, which is why the pin
is a test rather than a claim.

The unit closed **narrowed, not clean**, and its disclosure is now a measurement
rather than a shape: 8 pairs, 16 of 64 sized rows, six named scenes, and the
honest addition that among the sized rows five of the eight identity columns are
constant so the pair actually separating them is `nu`/`nv`. Six residues are
rows (`D181`, `D182`, `D201`, `D202`, `D203`, `D204`, `D402`). One row was
checked and **not** filed: `k-lint`'s `FLOAT_COLUMNS` has the admissions test
`tess-lint`'s third table was missing, so the class had one instance, not two.

**An orchestrator error, recorded because it cost evidence.** Under disk
pressure I deleted the finished reviewers' notes directories — including
`k1-review-corr-notes/`, which held the adversarial lane's mutation CSVs. The
fix pass then needed them and had to rebuild equivalents from recorded column
deltas. It reported the rebuild honestly and one row of the A/B came back
different: on a same-lane swap the **old** gate was not silent at all, it exited
2 saying *"the sizing schedule got wastefuller"* about a face whose schedule
never moved — a mis-join in the voice of a measurement, which is worse than the
silence the row was written about. **A review's artefacts are evidence, not
scratch**: they belong with the lane until the unit lands, not with the
reviewer's lifetime.

## `k2`'s reviews — a certificate that does not cover its own headline case

Two reviewers, two methods, one break. The style lane brute-forced a
counterexample; the correctness lane reproduced it independently and then
characterised the class analytically.

**`split_scan_worst_excess` documents itself as the worst excess *"over every
bound whose optimum the range brackets"*, and the derivation printed beneath it
assumes the interior stationary point — which is the optimum only when the
one-division floor does not bind.** For `muv = 0` the closed form applies **iff
`muu ≥ δ_s/2` and `mvv ≥ δ_s/2`**. At the shipped pair, `(2.9808e-4, 0,
1.9437e-2)` scores **2.09%** against a claimed 0.166% and a ceiling of 0.5%,
with the scan's argmin interior so the range claim passes and nothing reds.
**91,506 of 300,000** random floored-class triples exceed the closed form, and
**two of the six family members are outside its domain — including the ruled
wall, the member the row is named for**, which passes at 0.017% by lattice luck
rather than by the bound.

The consumer-derived ceiling inherits the break. Its *cancellation* half is
sound and was verified — the scan sits in the ratio's denominator on both rows,
so a static excess cancels exactly — but the magnitude half bounds the closed
form rather than the excess: at the coarsest step the ceiling admits, the true
worst is **3.54%, 71% of the gate's whole margin rather than a tenth.**

**And the guard never calls the code it is about.** The objective is a hand
re-spelling of the shipped one, so retuning the *call site* rather than the
constants — `split_scan_aspects(DECADES, 21)` — inflates cell counts by
**12.73%, 2.5× the gate's entire margin, with every test green.**

**Ruling: `D105` narrows.** The range claim is sound and provably so
(quasiconvexity in `log t`, 0 violations in 60,000 triples). What lands is the
guard with its domain stated, the family extended into the floored class, and
the objective called rather than re-spelled; the floored-class certificate
becomes a row. The lane's own summary is the sentence to keep: **the pair is
defensible; the certificate attached to it is not.**

### A rule §D does not currently state, earned by this row

`S160`'s measurements were taken on a six-member family that was **deleted with
the instrument it belonged to**, so the evidence that persuaded an orchestrator
to place `D105` over a standing prohibition was unreproducible from the repo:
the taker had to *fit* the missing member back and cross-check it against a
percentage in a closed track's log. §D rule 3 binds a closing PR to relocate a
standing **rule** *"into text that survives, in full"*; its nouns are *sentence*
and *standing rule*, and they do not reach an artefact. **A closing PR that
deletes a measurement's fixture owes the fixture the same relocation the rule
already gives the sentence.** Stated here rather than only in a report, which is
the rule applied to itself.

Related, and it is why the row cannot cite its own evidence: **`S160`'s
published continuous table is a *seeded* measurement and the shipped guard is
unseeded.** Both reviewers reproduced the published column exactly, and only by
admitting the seed the lane's own test refuses. Unseeded, `S160`'s *"falls
smoothly with resolution"* is **false** of the shipped quantity — 400 samples is
6× worse than 321, because the bound is attained at even sample counts.

## `w2` — the first unit that did not re-mint, and the mechanism it built

`D61`/`D80` landed as one lane. Two things distinguish it from every other unit
this session.

**It found the home instead of re-forking it.** `crates/test-utils/src/source.rs`
already existed — #872 had ported the *weaker* of `topo`'s two blankers into it
— so the lane upgraded it in place and paid `S117`'s stated prerequisite
(`raw_string_len`, `char_literal_len`), which turns collapsing
`topo::source_walk::CodeOnly` onto it from a downgrade into a deletion. Three
lanes before it hand-rolled a helper the tree already had; this one looked.

**The shape answers the row rather than the list.** One lexer plus a public
`keeping(text, &[Region])` and three named selections — so **a fourth
combination is an argument, not a fourth reader**, which is the exact trap
`S117` describes (its own count moved a fourth time when a sibling lane landed
a new reader *during the finding's review*). A test pins that the three views
partition a file byte for byte, so there is nothing outside them to build a
fourth lexer from.

**And it built the thing that makes the next reader visible**, which is the half
`S117` said a taker who works the twelve would miss:
`crates/test-utils/tests/reader_census.rs` walks five roots, asserts **set
equality** against a committed ledger, caps `Unconverted` as a ratchet, forbids
a `NotRust` entry that names no language, and carries a vacuity floor so a
broken walk reds rather than passes. **The population was 34, not 12.** Ground
both prior sweeps missed includes a *seventh* reader in `tools/`, and
`sweep/src/fillet/admit.rs`, which reads its own source with **no reader at
all** — its author spliced string literals to avoid self-matching rather than
lex, the same tell `S117` names on the class's worst member.

The red-proofs are the strongest of the session because each is an **A/B against
the pre-conversion spelling**, showing both directions: the old guard green over
a block-commented real site (the silent miss), and falsely red over a comment
that merely mentions the needle (F3's cry-wolf, already realised once in this
tree).

**Five rows minted from its residues**, grouped per track as the partition
requires rather than one per reader: `D287` (Q, four `topo/src` readers), `D261`
(P, the two remaining plus both blanker collapses), `D321` (T, `admit.rs`),
`D205` (K, the seventh reader), and `D382` — this track's own, for the **13-fold
duplication of `every_suite_file_is_aggregated`** that the lane surfaced by
being the first thing to touch all thirteen copies. `crates/topo/src/source_walk.rs`
is drawn into Track P's fence with them, for the same reason `fixtures.rs` was.

**Bookkeeping error worth recording**: minting those rows, two landed in the
adjacent track — a row is inserted *before* the next track's heading, so an
anchor on `## Track Q` appends to Track **P**. The per-track counts still
agreed, because each track had gained exactly one row; only reading the rows
back caught it. **A count that reconciles is not evidence the rows are in the
right table.**

## `w1`'s fix pass — and a ruling of mine it correctly declined

The pass built **`test_utils::tightness`** beside the existing `vacuity`: the
ceiling half of a discipline whose floor half already had a home, with `Sup` and
`Meter` taking a measured ceiling per site and **owning no constant** — a shared
ceiling being this defect at a higher altitude. It also **moved** `caught` out of
`vacuity`'s test module rather than copying it, which is the same rule applied to
its own new code.

Two outcomes are better than the instruction that produced them.

**The bicubic ceiling was removed, not tightened.** A total cancellation loss
costs 1.25× on that geometry while the box rule admits 1.757×, so **no number is
simultaneously above the healthy state and below the broken one** — the fixture
has a residual of 0.910 m on an object 1.600 m across, so there is almost nothing
for tightness to be tight about. The row now claims completion and soundness
only, and points at the file's actual cancellation witness. An assert-nothing
`no_ceiling(why)` API was declined on the ground that a test asserting nothing is
never a gate; the verdict is prose at the claim site with both measured endpoints.

**And it declined my anchor, correctly.** I ruled the aligned row's ceiling
should be anchored on its comment's `~1e-1` span-width figure. Applying that same
anchor to the sibling row on the *same wall* would have declared a shipped,
sound, measured-tight bound (1.286e-1) already degenerate — so `~1e-1` is a
remark about one carrier, not a scale property of the geometry, and it cannot
serve as the rule. The lane used the **computed whole-object box diagonal**
instead, which is the durable form of what I meant, re-derives itself when a
fixture moves, and is now guarded by its own red-proof (`CEILING IS NOT A GUARD`
fires when a ceiling is raised above it even though the bound is well under it).
It deleted the unsourced sentence rather than leaving it standing.

Two further repairs worth naming: `truth * 0.99` is gone — `dense_true_sup` now
**reports unconverged foot points** (0 of 4097 on every carrier), the row pins
that at zero, and domination is exact. And the speed meter's constant is
re-justified from the **integral** arm's own nineteen-carrier population rather
than from rational rows that run a different arm, two of which sit inside the
band the guard forbids.

### An exposure this session created and has not yet paid

`w1` and `w2` **both edit `crates/test-utils/`** — one adding `tightness` and
`panic_capture`, the other rewriting `source`. Both touch `src/lib.rs`. That is a
collision *inside* Track W, and the partition does not prevent it: its rule is
that no two **tracks** share a file, and sequencing within a track is the
orchestrator's job. It was not sequenced. The conflict is textual (two module
declarations) rather than semantic, but the real exposure is wider: **every crate
in the tree dev-depends on `test-utils`**, and between them the two lanes have
verified it against four crates and fourteen. A workspace-wide `cargo check` is
owed on the integration branch before this reaches hosted CI, and it is the
orchestrator's to run.

## `w2`'s style review — the re-mint moved up one level, and the diff names it itself

The lane did **not** re-mint at the helper: `keeping` is genuinely the only lexer
in `source.rs`, verified rather than asserted. It re-minted one level up, three
times, and the sharpest instance is stated in its own docstring.

**`source.rs` says, in this diff, that sharing the *predicate* and re-forking the
*traversal* "leaves each guard free to miss a subdirectory, silently and in the
green direction" — and all thirteen converted mount guards still walk `tests/`
with a flat `read_dir`.** The drift has already happened in exactly that
direction: hash the thirteen bodies and twelve share a digest while `geom`'s is
**recursive** and carries a **converse assertion** (one `#[path]` per file, no
orphan mounts) the other twelve lack. One of the thirteen is a strictly stronger
guard and nothing records that the rest are the weak variant.

Two more at the same altitude: `without_module_mounts` is **a hand-rolled source
reader inside the file whose stated rule is "do not write one"**, and `repo_root`
is the **third verbatim copy** of the "both ways the suite runs" resolver — in
the crate that exists to end that shape. And five bracket/brace-depth scanners
appear over the shared blanked view, in two incompatible algorithms, because
*"carve out the balanced region"* is what every converted call site actually
wanted and the three views do not serve it.

**The census has the silent direction it was built to close.** Only `NotRust` is
checked: nothing verifies that a line dispositioned `Shared` still reaches
`test_utils::source`, so a site that reverts to a hand-rolled reader keeps its
line and the census stays green. And the ratchet does not ratchet — the ceiling
is `<=` at exactly the current count, so converting one entry while adding one
new reader nets zero, which is what the doc two lines above says cannot happen.

**A live instruction now points into the defect.** `docs/SMELL-I-LOG.md:200-202`
tells lanes *"do not mint a thirteenth hand-rolled reader; reuse the shared
`fixtures::code_only` walk"* — and `fixtures::code_only` is now line 192 of this
lane's own debt list. A lane obeying the tree's written rule would be directed
into the class. That is Q4's dangerous half, and it is worse than the silence it
replaced.

**The pattern across the session, now with a name.** Four units reviewed, four
fixes that reproduced the defect they closed — `k1` a missing admissions test on
the third of three tables, `w1` and `p1` a helper the tree already had, `w2` the
traversal half of the walk it shared the predicate of. **None was caught by its
author, and every one was caught by the first reader who did not write it.** §D
rule 5 held at eight of eight on Track F and every unit on Track G; it is now
holding at four of four here.

## `p1`'s fix pass — a lane that withdrew its own case, and answered row 4 with row 0

Two things here are better than what was asked.

**It withdrew its constructed case and proved the withdrawal two ways.** Asked to
restate `D88`'s disposition honestly, the lane did not soften — it verified by
reading that `kef` re-derives the same face key in its plan phase and refuses
`StaleKey` before any mutation, then reverted its own fix in the worktree and ran
all six probes green, confirming the reviewer's result against itself. And it
went further than the reviewer had: **the two arms are provably indistinguishable
from outside**, since reaching the ring lookup's `None` while `kef`'s succeeds
would need one key to resolve two ways in one arena with no mutation between. So
the announce the lane put there is itself unreachable — exactly as the default
was. What `D88` actually names is a **legibility** defect, and it closes by
announcing, which changes no reachable behaviour.

**And it discharged the `unreachable!` demonstration by removal.** The ratified
standard says every such arm must be demonstrated live by poisoning its key. The
lane instead made `edge_faces` return a `HalfEdgeFacts{loop, face, start}` — the
facts the proving walk had already read — so **all three new arms were deleted
rather than asserted about**. Net `unreachable!` added: zero. That is row 0 of
the D2 addendum (*can the type stop representing the state?*) answered in place
of row 4, and it is strictly stronger than the demonstration it replaces.

The ruling landed as `is_arena_fault`: `Op{StaleKey|StaleGeometry}` refuses the
call under both regimes, so `GroupRegime` governs inventory refusals only. The
false premise a reviewer found is gone — `group_regime` now asks **every**
member and refuses a `GroupKindSplit` when they disagree. And `ResultNotClosed`
split from `GroupNotClosed` puts the scope in the type rather than in prose.

**Three rows minted from it.** `D262` (P) — the four predicate helpers still
answer on an unresolved lookup: eleven discard arms and one **value
substitution**, `edge_chord_len(...).unwrap_or_else(T::one)`, which feeds a
unitless `1` as a *length* lever arm into a `decide` site, so a failed lookup
re-scales the margin rather than merely answering the question. The lane's own
earlier disclosure named three; the file holds twelve. `D263` (P) — the regime
test asks *"is it a plane"*, and a face on the `mvfs` `Nurbs` placeholder
answers *"is it curved"* by default, found only because the new kind-split probe
failed against `ops_cube`. `D288` (Q) — the sentence `D38` corrected at its
origin has a second copy across a fence, and both halves of it are now false.

## `k2`'s fix pass — and the finding that outgrew its own row

The guard now drives the **shipped** scan: `split_scan` is one function with the
counting rule as a parameter, `best_split_steps` is that composition, and a new
**claim 0** asserts the lattice under test is the one the constants describe.
The call-site hole is closed — the substitution that inflated cell counts 12.73%
with every test green is now caught, and the lane re-measured what it costs
(+29.77% on the new floored ruled wall). The closed form is renamed
`unfloored_worst_excess`, carries `δ_s` in its signature, and has a companion
predicate expressing its domain; the family grew 6 → 8 with both counterexamples
as members, each declaring a `Shape` that is **checked** rather than asserted.
Both ceilings are now read out of `tess_lint::GROWTH_TOLERANCE` by the
`include_str!` idiom the file already argued for 280 lines below — the mechanism
the lane had said was unavailable and I had ruled was not.

**And the true relationship is now written down instead of the one that was
wanted**: the worst member sits at 2.088%, which is **42% of the gate's whole
margin — the same order as its tolerance, not an order below it.**

**One declined item, and the argument is right.** Told that `SPLIT_SCAN_DECADES`
could be cut 8.0 → 3.7 with every claim green, the lane declined: that no family
member lives above `t = 1` is a property of *the family*, not of the tour, so
narrowing the range to the members that happen to exist is fitting the constant
to the test — which is the move `F-R14` killed. Correct, and it is the second
time this session a lane has refused an instruction on better grounds than the
instruction had.

### `D206` — the finding is probably about the gate, not the meter

The lane raised this and did not file it, saying it was bigger than its row. It
is. The crude slope envelope for the floored class is **5.93%** at the shipped
pair, and a two-sided kink-slope argument tightens it only to 3.91% — both at or
above `tess-lint`'s **entire 5% growth margin**. If the true worst is anywhere
near the envelope rather than near the 2.088% two searches found, then **a face
whose bound moves relative to the scan lattice under a pure geometry change can
move `span_opt_cells` by more than the budget gate's whole tolerance, from the
instrument alone** — the gate would fire, or fail to fire, on lattice placement
rather than on tessellation. `D105`'s residue is the certificate for the kink
case; `D206` is what that certificate would *mean* for the consumer, and it is
the first thing to do with it. The delta review is briefed to establish how close
the true worst is to the envelope, because that number decides whether this is a
documentation gap or a live defect in a shipping gate.

Two rows minted: `D206` above, and `D303` (Track R) — `mesh::sizing::ceil_count`
answers `1` for a **negative** step in the function about to allocate that many
grid points, so after `D105` the kernel is the laxer of a two-spelling pair. A
negative step is not a smaller grid; it is a reading that did not happen.

## `w1`'s delta review — a ruling of mine, measured and found overstated

I ruled that a tightness ceiling *"must sit below the scale at which the
enclosure degenerates to the whole-object box"* and called that **the thing that
makes a ceiling a guard**. The reviewer measured it: the degraded readings are
**smaller** than their boxes in three of four rows — 20.843× against a box
admitting 45.707×, 6.730× against 16.000×, 5.108× against 7.744×. So the anchor
is a **necessary condition, not a sufficient one**; it stops a ceiling from being
obviously vacuous and does not make one a guard. The demonstration is that the
straddle row's *old* `6.0` ceiling would have passed the anchor check while
failing the degradation it was meant to catch. **The rule stands, with its claim
cut to size**: what makes a ceiling a guard is a measured degraded reading it
sits below, which is what the rows already carry — the box is the sanity check
underneath that, not a substitute for it. `tightness.rs`'s module doc inherited
my overstatement and is being corrected with it.

Two further findings are the same class one level in. **The helper can build the
defect it exists to prevent**: `Sup::new(..).truth_at_least(..);` compiles, runs,
asserts only the floor, and is `S121`'s shape again — because the intermediate
builder states are not `#[must_use]`. And **the pass built the ceiling half's
home and left the soundness half hand-rolled at seven sites** across three files,
`s >= m - 1e-12`, with no sentence anywhere admitting the copies and no
derivation of the constant. That is the undisclosed-duplicate shape the style
brief says only the *data* finds, never the prose sweep; it is now folded into
`D383`.

Also confirmed, so the exposure I flagged earlier is closed: **the `test-utils`
blast radius is compile-only and zero** — `panic_capture` is a `#[cfg(test)] mod`
and is never compiled by a downstream dev-dependent, `vacuity`'s public surface
is byte-for-byte unchanged, and the crate has no dependencies to unify. The
workspace-wide check is no longer owed for *this* reason; it is still owed for
`w2`, which rewrites `source.rs` in the same crate.

## `k2`'s delta review — `D206` confirmed, and the certificate written after all

Two results, and both change what the row is.

**The claim-0 hole moved one function inward rather than closing.** Claim 0
asserts properties of `shipped_split_scan_aspects()`; nothing asserts that
`best_split_steps` **calls** it. Three retunes at the call site are green on the
entire gated job — including **the prior round's own counterexample**, unchanged
— at a measured cost of **mean +14.93%, max +100%** on the gated column over
200,000 bounds. A guard that catches a substitution inside the helper and not the
substitution of the helper has relocated the defect. The fix is an assertion on
the composition, and it is one line.

**The certificate the row filed as unwritten exists.** For `muv = 0` with the
floor binding, put `r = muu/δ_s`; the optimum is the kink, the slopes are `−r`
and `1 − 2r`, and worst-case sampling gives `10^(s·r(1−2r)/(1−r)) − 1`,
maximised at `r = (2−√2)/2` → **1.995%**. The lane's own floored counterexample
has `r = 0.29808` — **it is the analytic argmax**, which is why two independent
searches converged on it. So claim 3 stops being a measurement and `D105`'s
residue closes rather than files. An independent 4.6 M-bound sweep put the class
sup at 2.0918% against the lane's 2.088%.

### `D206` is real, and it is about the gate

The decisive distinction is one nobody had drawn: **claim 3 is stated on the
continuous objective and `tess-lint` reads the `ceil`'d one.** On the shipped
counting function the lane's own family member `anisotropic, live cross term`
scores **5.8824%** — already above the gate's whole 5% margin, a number sitting
in the tree twice and never joined to its consequence — and a **single smooth
geometry change** (`mvv` 1× → 100×, counts 10,580 → 105,760) walks the scan/true
ratio from 1.00000 to **1.05948**. The instrument alone moves `span_opt_cells`
past `GROWTH_TOLERANCE` from lattice placement, not from tessellation.

The lever is cheap and now measured — `SPLIT_SCAN_SAMPLES ≥ 379` at 8 decades
puts the envelope under 5% — and I have ruled it **out of `D105`**: raising it
moves every committed budget number and re-cuts the baseline, which is a unit of
its own rather than a rider on a guard's fix pass. `D206` carries it, with the
number recorded so the taker does not re-derive it.

**And the lane's decline of the range narrowing was right on its own axis and
answered the wrong question** — the reviewer's phrasing, and it is the better
one: *the range question is open and the resolution question has a cheaper
answer.*
