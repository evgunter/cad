# PRED — one numeric fact, decided in several places (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3).

Branch prefix: **`pred/`**. Away-channel tag `(PRED orchestrator)`.
A/B ordinal band **PRED = 4400–4499**.

## Charter

Every row here is the same finding about a different quantity: a fact
the kernel decides — is this rim planar, is this cone one nappe or two,
does this period have headroom, is this lever arm folded — is decided in
several places, and the places do not agree on the margin. They are not
duplicated *code*; they are duplicated *judgements*, which is why a
mechanical de-duplication is the wrong fix and why these rows have sat
unclaimed while the crates they live in have all had owners.

The obligation this program takes on and that no row states for itself:
**a predicate that gets a shared home gets an agreement test in the same
PR**, showing that the old sites and the new one answer the same on the
cases each was written for. A shared home with no such test is the same
defect with one address, and the near-cases are exactly where the
margins differed.

## Territory — none, and why

This program claims **no paths**. Its rows reach PROPS', CURVED's,
S-BOOL's, S-MESH's and TRIM's files simultaneously — `props/curved.rs`,
`boolean/reduce.rs`, `mesh/curved.rs`, `pcurve_cache.rs` — which is the
class, not an accident of filing. Every unit announces to the owners it
touches and the fence is drawn in the PR.

## The slate

The sitting of 2026-10-25 and 2026-10-26 worked every dispatchable P1
row. The track holds two rows, both parked:

| item | cost | waits on |
| --- | --- | --- |
| `cone-nappe-is-decided-in-five-places` | H, design | PR 5500 (PCERT), which rewrites `pcurve_cache`'s nappe sites. Re-census the fourteen sites first, then send the per-point disposition question (escalate vs fold) to designers |
| `contact-verify-spells-the-orientation-and-second-order-margins-beside-their-homes` | M | the D10 hold: `contact_verify` is declared-contact ground that INTENT's stage 4 retires. Re-read it against the tree first |

What the sitting landed (each row's `## Closed` section is the record):
- the period-headroom rule `geom_core::period` (PRs 5686, 5710, 5724);
- the directional conic extremum (5685);
- step-export through the shell-role door (5684);
- the two-axis-run revolve (5680 ruling, 5707);
- the mesh sizing rule `[mesh.sizing]` (5708 ruling, 5717);
- D46's first slice (5721), and the lever-arm and D57 rows (5681).

The P3 and P4 filings went to PREDTAIL, and so did D46's remainder.

## Review posture

The repo-wide tiers (`memories/orchestration-model.md`). The tier is
named per unit at dispatch, in the log. A unification that changes
which margin decides a reachable answer draws at least a full single
review, because a wrong margin is reachable and silent. Moving a
predicate to a shared home is held to the charter's agreement test.
