# Lane `heldrecords/split-keys` — P1, step 2 of the plan

Row: `work/heldrecords/the-splits-own-pinch-and-cut-copies-take-two-point-keys-and-no-record.md` (P1, M). Read `work/heldrecords/plan.md` (Charter steps 1–3) and Ev's ruling in `work/heldrecords/the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime.md` "## Decided (2026-10-28, Ev on PR 5967)": **crossings a cut mints on two edges recorded as one locus share one point key** (decided; not a fork). The row's "pinch copies" framing is off per the plan: pinch copies already share the point; the failing cases are crossings. Check that against the tree.

**Stacked**: branch `heldrecords/split-keys` off `origin/embed/records` (PR 7124, step 1: `AtRestBody` holds records and the split carries its operand's records; it is in its fix pass under EMBED's lane). Open your PR with base `embed/records`; when that branch moves, merge it in; the orchestrator retargets to main once 7124 lands. Do not push to `embed/records`.

Owed: the cut mints one point key where two crossings land on one recorded locus (doc of `split`: "copies share its point key" becomes true), so the straight cases in the row pass tier 3′ on their sides with the records the side holds. Report every case of the row (straight and curved) before and after. The curved instances are the next step's (the split records its own coincident edges with a `SplitOn` row; `a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs`): do not build that here, but say what of it your change already covers. Sweep for every other door that mints two keys for one point on one locus, and give the hit list.

Review tier: single FULL review.

Standing rules: `briefs/common.md` on this branch (`origin/heldrecords/briefs`).
