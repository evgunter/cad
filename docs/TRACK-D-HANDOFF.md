# Track D — handoff state (2026-08-20)

Written before a usage-limit stop, per `memories/usage-limit-protocol.md` and
the orchestration model's "commit crucial state before stopping". **The
schedule itself lives in `docs/SMELL-SCAN-2026-08.md` §D; this file is the
operational state that does not belong there** — which branches are live, what
each was last told, and what a successor should not re-derive.

Delete this file once Track D closes or a successor absorbs it.

## What Track D was

Constituted 2026-08-20 to execute the *Accepted, unscheduled* table — ten
findings with a verdict from Evan and no row anywhere — plus **B2** and **B3**,
handed over when Track B closed. It ran **beside the model A/B experiment, not
inside it**: no dispatch was an A/B row and nothing in it touched
`docs/MODEL-AB-LOG.md`. Reviews were **style-only** except on rows carrying
real risk, which took a full adversarial lane as well.

## Landed

| Unit | Finding | PR |
|---|---|---|
| D1 | S6 — ten sweep helpers unified; a third chain copy the finding missed | #710 |
| D3 | S18's negative-zero flush — one home, plus a CI gate | #704 |
| D4 | S8/S9/S10 truthed; the schema-ledger guard | #707 |
| D5 | S15's prose-held invariants; the 37-door surface guard | #713 |
| D6 | S12's residue — the release profile CI never ran | #706 |
| D9 | S17's ray-parity twins; the K roster method fixed | #712 |
| D10, D12 | The ray schedule's second home; a verbatim copy adjudicated dimension-forced | #717 |
| D15 | The K harness of record; K-REPORT's provenance corrected | #718 |

## In flight at handoff

Each had CI green, went through review, and was in a **fix pass** when this was
written. None is merged. A successor should read the lane's last report and the
orchestrator's fix-pass message before touching any of them.

| PR | Unit | State |
|---|---|---|
| #719 | D11 — `bool_join_nearest` split | Fix pass: three MAJORs. The K-REPORT decade-3 attribution is **backwards** (three of the 38 samples are negative; `Margin::of(norm)` cannot be); the inverse-rename check cannot detect a wrong site assignment; the declined guard was **one line, already in charter** at `rim_dim_boolean_twins.rs:246-268`. |
| #720 | D16 — W2c, the D2 addendum executed | Fix pass: `kef` is a **second unproven `prev` path**, inside an operator, which re-scopes D18 — fixing `split.rs` alone does not make `link_half_edges` convertible. Also `DESIGN.md:1224-1230` still says the work is "NOT yet done". |
| #721 | D7 (first of three) — `Mat2`/`Affine2` deleted | Fix pass: the deletion **orphaned `Vec2::unit_x`/`unit_y`**, a fresh instance of the `GENUINELY DEAD?` class the unit was closing. Told explicitly **not** to start `PairSolve`. |
| #722 | D13/D14 | Fix pass: the new guard **cannot see a delegated mint**, and the one historical rot it cites (`merge_coplanar_faces`) is exactly that shape. D14 is a **refutation, not a closure**. |

## Row numbering — assigned centrally, and why

Three lanes minted §D row numbers in parallel early on and collided (two lanes
both used D10 and D11). Numbers are now assigned by the orchestrator:

| Row | Owner | Subject |
|---|---|---|
| D17 | landed with #718 | Nothing in CI builds sweep's **test targets** under `probe` — 16 files, 2 compiled |
| D18 | in #720 | `split.rs:253`'s unproven `prev`, **and** `kef`'s — unblocks W2c's last two sites |
| D19 | in #719 | The K roster obligation reaches types, not names-not-reachable-as-bare-literals (~25 sites) |
| D20 | in #722 | D5's +46% is real and **unattributed**; `choose_op` is excluded by measurement |

**The rule, which a successor should keep:** a lane takes the next number the
orchestrator has assigned, never the next gap it can see. A roster with holes
reads as an editing error otherwise.

## Blocked, and on what

**Nothing is blocked by another track any more.** Both external edges closed
on 2026-08-20: **#702** (`f382c4aa`) discharged D7's `PairSolve` row, and
**#705** — the ≥200-file `geom-curves` + `geom-surfaces` merge that had been
the track's single largest scheduling constraint — discharged **D2**, D7's
fillet-helper row, and **D8**, whose `geom-curves/src/fit.rs` it relocated to
`geom/src/curves/fit.rs`.

What is unstarted is unstarted for schedule reasons only, not technical ones:

- **D2** (B3 / S19, the fillet error catch-alls, ADVERSARIAL) is the widest
  unblocked row and gates D7's fillet-helper row — the only edge left in the
  track.
- **D8**, **D17**, and D7's **`PairSolve`** row are edge-free. `PairSolve`'s
  provenance note goes to **issue #611**, not the PR body, because R2's thread
  is live; the deleting PR must cite the recoverable commit SHA.

## For Evan — S14 now has three witnesses

S14's proposed reframe (*"no panic on any reachable state, yes panic on things
that can only indicate bugs"*) sits in *Open decisions — Evan only*. It began
resting on the `Span` case alone. Track D added two more, neither invented for
it:

1. **#713** — `graft_disjoint_all_keyed` mints empty-shelled solids before the
   transplant and can fail mid-write, so a **public door can leave a body
   tier-1-invalid** and the caller still holds `&mut dst`.
2. **#720's review** — sharper: because these are slotmap keys, an unpatched
   source-internal key may **resolve to an unrelated live entity** rather than
   dangle. That is *live but wrong*, which no plan phase can refuse.

Both are the "reachable by API misuse" row S43 proposed and the ratified D2
addendum's five classes do not contain.

## What the review lane actually caught

Recorded because the calibration question in `docs/REVIEW-STYLE-DISPATCH.md` is
open, and this is evidence. Style-only reviews on prose-heavy units found:
four false claims introduced by a fix pass into the finding it was closing;
a CI gate that rustfmt's own output formatting walked straight through; a
guard that could not catch the rot it named; a corroborating example that was
the one site ratified *not* to follow the idiom it illustrated; and two
measurements credited with discriminations they could not make.

Two of the corrected errors originated in **orchestrator briefs**, not lanes —
a mis-scheduled decision (S12's residue was ratified, not open) and a wrong
premise about which pair S15's ray-schedule row names. Reviewers correcting the
dispatcher is a working lane, not a malfunction.

## Standing operational notes earned this session

- **Confirm CI started by reading the workflow *runs* list, not the PR's checks
  list.** A CONFLICTING PR produces **no run at all**, which is indistinguishable
  from a slow one in the checks UI. One lane lost eight minutes to this.
- **The FreeCAD render lane wedges transiently.** Two units hit an identical
  missing-cell failure; the one that could re-run (via a re-merge it owed
  anyway) came back green on identical content. `rerun-failed-jobs` returns
  **403** for this integration, so a fresh run has to come from a legitimate
  push.
- **Do not treat a reviewer's causal story as established.** One review
  attributed a real misattribution to a shallow clone; `is-shallow-repository`
  is `false` here with one root commit. The facts were right and the
  explanation was not — fix the facts, write no causal story.
