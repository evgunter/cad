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
