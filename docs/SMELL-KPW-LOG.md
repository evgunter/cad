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
