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
