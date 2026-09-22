# Sweep 10 — 2026-09-09: METER leaves the tracker

Sweep SHA: `2723839067e80198bec2889d041e490000f02275` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
METER's directory is complete and every row in it closed), so every path
below is recoverable at
`git show 2723839067e80198bec2889d041e490000f02275:work/meter/<FILE>` and
`git show 2723839067e80198bec2889d041e490000f02275:docs/METER-EXIT-WALK.md`.

METER — the budget and K instruments, code-quality Track K's `tools/*`
half (`tools/tess-lint`, `tools/tess-meter`, `tools/k-lint` and the two
documents they feed, `docs/TESS-BUDGET.md` and `docs/K-REPORT.md`, with
their committed data) — opened 2026-09-06 in the tracker-wide cut and
closed 2026-09-08 on Ev's ratification of `docs/METER-EXIT-WALK.md`
(PR #2212, ruled in three comments in that thread). Per the sweep-5 rule
the program's directory leaves whole: `program.md`, `plan.md`, `log.md`
and every item file, all seventeen `status: closed` at the sweep SHA
(`D201`, `D203`, `D206`, `D213`, `D214`,
`cert1-notes-pr-body-tracked-on-main`,
`cut-prefix-three-unpinned-spellings`,
`fold-the-two-baseline-census-files`, `k-lint-predicate-roster-unpinned`,
`k-report-baseline-fold-cert1-roster`,
`k-report-era-witnesses-have-no-guard`,
`report-header-column-phrases-unqualified`,
`sweep-deleting-work-meter-dangles-six-refs-on-instr-rows`,
`tess-budget-doc-finding-block-stale`, `tess-lint-face-ordinal-join`,
`tess-lint-twinned-csv-fixture`,
`tools-readme-is-unratified-and-owes-a-design-row`); the other twenty-two
rows the slate held were re-homed first, below. **Thirteen unit PRs**
landed, numbered 0–12 (2111, 2114, 2115, 2125, 2132, 2140, 2151, 2158,
2167, 2177, 2179, 2180, 2187) — not the twelve the plan and the log both
claimed, which is criterion 1's recorded honesty. Two `[ev]` rulings
ratified (2109, `D201`'s design fork, arm A; 2147, `tools/README.md`'s
`CC1`–`CC5`), three orchestrator state-sync PRs (2110, 2128, 2164), and
three closing PRs (2212 the walk, 2218 the discipline paragraph, 2220 the
successor). Infra-only: no A/B rows were written; the band 3200–3299 was
claimed for bookkeeping and stays allocated.

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `meter` | METER — the budget and K instruments | 2026-09-08 | this row and the exit-walk row below; the thirteen units' PRs above; design at `tools/README.md` (`CC1`–`CC5`, the reading-boundary rule, ratified by Ev 2026-09-08 and listed in `docs/DESIGN.md`'s companion table) |

What the walk records: criterion 1 MET with recorded honesty (thirteen
units, not twelve, and `D201` built rather than deferred); criterion 2
**CARRIED** — Track K's `tools/*` half is not empty and was never close,
so the walk's central ruling opened `instr` for it; criterion 3 MET. Its
census re-derived three published figures that were wrong (the slate at
22 open and not 23, 40 files with 23 open and not 24, thirteen units and
not eleven or twelve), and §2 carries one line per unit of what it left
behind.

### Residue re-homed before the deletion

The moves happened in PR #2220, the successor PR, not in the deleting
commit; the deleting commit finds the directory already emptied of live
work.

| item | to |
| --- | --- |
| twenty rows — `C15`, `baseline-census-partition-assert-cannot-fail`, `baseline-sizing-census-pointers-stale`, `baseline-sizing-census-second-copy`, `cut-line-commit-names-no-baseline-change`, `gate-findings-name-no-columns-and-recoverable-has-two-aggregations`, `k-lint-csv-header-unpinned-against-five-producers`, `k-lint-eps-coupled-criterion-unwritten`, `k-lint-gate-described-as-diffing-the-committed-baselines`, `k-lint-last-round-is-eps-coupled-but-unrostered`, `k-lint-roster-wants-a-kernel-side-vocabulary`, `k-lint-rule-1-prose-assumes-every-in-band-site-refuses`, `report-constraint-activity-line-names-no-columns`, `tess-budget-doc-identity-column-list`, `tess-budget-doc-note-finding-rule`, `tess-lint-growth-margin-unprotected-from-ceil-quantisation`, `tess-lint-recourse-quote-half-pinned`, `tess-lint-ungated-columns-fold-silently`, `tess-lint-zero-certificate-two-meanings`, `tess-meter-sampled-retune-figure-unreproducible` | `work/instr/` — INSTR, the successor Ev ruled open on 2026-09-08 (walk §4), which takes METER's `paths` unchanged and the band 3300–3399. Nineteen are instrument rows; `C15` is the twentieth, ruled here rather than to code-quality Track X (walk §5, reversing the walk's own first ruling on Ev's challenge) |
| `reader-census-shared-disposition-survives-partial-reversion` | `work/tcost/` — S-TCOST's `paths` carries `crates/test-utils/*`, and the `Shared` ledger audit is a ruling on every row of that ledger |
| `cert1-notes-pr-body-tracked-on-main` | nowhere — **closed by disposition**: Ev ruled *"delete"* (2026-09-08), `.cert1-notes/pr-body.md` was removed in PR #2220 and the row closed citing the ruling. It asked for a disposition, not for work |

Filed on other slates during the program and already there:
`cut-regex-unanchored-admits-a-line-the-lint-refuses` and
`cut-script-header-claims-no-cross-language-gate-exists` (CIW). Nothing
went to `work/issues/`.

### The six `refs:` this sweep rewrote

Deleting the directory would have broken `scripts/work.py`'s *references
resolve* rule on six INSTR rows that cite five closed METER ids, so the
deleting commit rewrites them first, in GATES' form (`3a8dd05fe`, sweep 7):
the dying id is replaced in `refs:` by the number of the PR that closed
it — ints are PR numbers and lint does not check them — and a
`## Refs at METER's sweep (2026-09-09)` section at each citing row says
which id is now cited by which PR.

| dying id | now cited as | citing row(s) |
| --- | --- | --- |
| `D201` | 2167 | `C15` |
| `cut-prefix-three-unpinned-spellings` | 2151 | `cut-line-commit-names-no-baseline-change` |
| `k-lint-predicate-roster-unpinned` | 2115 | `k-lint-csv-header-unpinned-against-five-producers`, `k-lint-eps-coupled-criterion-unwritten` |
| `k-report-baseline-fold-cert1-roster` | 2140 | `k-lint-gate-described-as-diffing-the-committed-baselines` (already carried 2140, so the id was dropped rather than duplicated) |
| `tess-lint-twinned-csv-fixture` | 2179 | `tess-lint-recourse-quote-half-pinned` (already carried 2179, same) |

The hazard was found and filed while step 3 executed
(`sweep-deleting-work-meter-dangles-six-refs-on-instr-rows`, closed at the
sweep SHA); prose citations of `work/meter/…` and of the walk survive
across the tree unrewritten, which is what *A note on inbound references* in
`docs/DOC-LEDGER.md` is for and what GATES' sweep did with its own.

### §6's process findings — one paragraph carried, the rest ruled out

The walk's §6 wrote out four process findings the log would otherwise take
with it, and §6.4 ruled they should go into
`docs/prompts/implementer-discipline.md` with Ev seeing the diff first.
**He then ruled that only ONE paragraph goes in** (PR #2218, 2026-09-08):
*"can you keep the 'write assertions a bug could break' paragraph and
revert everything else? most of this is not relevant to most work in the
repo, which deals with normal implementation rather than these greps"* —
and, on the same PR, *"if you do what my prev comment stated then you can
consider this approved"*. The merged diff is exactly those five lines, in
§2 beside *a build is not a test*.

**So §6's other findings are deliberately not carried forward**, and this
sentence exists so a later reader does not read the omission as an
oversight: §6.1's nine assertions that cannot fail with their per-instance
record, §6.2's three fake greens with their mechanisms (the mutation
script that failed before writing, the mutation on a doc comment, and
`grep` exiting 0 on a match inside a `&&` chain), §6.3's
shape-versus-reading finding with its six sub-rules, and the two
operational lessons (one `git worktree` per concurrent lane; `main` in an
ephemeral container is not `origin/main`) survive **only** at the sweep
SHA above, in `docs/METER-EXIT-WALK.md` §6 and in `work/meter/log.md`.
That is Ev's call and it was made on the diff itself.

### The exit walk's row

| walk | program closed | ratified on | done-state now |
| --- | --- | --- | --- |
| `METER-EXIT-WALK.md` | 2026-09-08 | PR #2212, in three PR comments (*"1. open / 2. giving them to the successor sounds good / 3. huh i thought track X had closed / 4. delete / 5. i would like to see the diff / 6. this doesn't look like a question?"*, then *"the new 3, instr, and your plan all sound good!"*) | this row; the residue table above; `tools/README.md` and the three instrument crates' own headers; `work/instr/` for the twenty rows it carried forward |
