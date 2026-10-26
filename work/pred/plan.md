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

Re-censused against main `2d5677e1` on 2026-10-25. Each row's
`## Re-census` section is its current state, and the cost and design
flags in its header are current.

| item | cost | what is left |
| --- | --- | --- |
| `period-headroom-margin-has-no-shared-home` | M, design | No site admits a reversed span any more. Two partial homes (props' `require_span_within_period`, topo's `ConicArc::of`) sit beside about eight one-offs, so what is open is where one home lives |
| `D292` | M | A directional closed-form extremum door in `geom`, read by the one generic `axial_window` / `boundary_axial` body |
| `S18` | M | The step-export volume re-derivation onto `topo::props::classify_shells` |
| `lever-arm-fold-six-hand-rolled-siblings` | E | `ssi::point_arm` against `folded_lever_arm`; two stale docs in `dihedral.rs` |
| `D57` | E | Re-sync the audit's never-reach-the-funnel paragraph |
| `D46` | H | Dimension verdicts for the uncovered names: 19 listed, about 140 more unlisted |
| `S116p` | M, design | Whether the two-run full revolve is built through the void door, as the holed revolve is. Designers, then Ev |
| `S29` | H, design | The sizing policy write-up, a design PR to Ev |
| `cone-nappe-is-decided-in-five-places` | H, design, parked | Fourteen sites. Waits on PR 5500 (PCERT) and PR 5674 (CHART), which rewrite its sites |
| `contact-verify-spells-the-orientation-and-second-order-margins-beside-their-homes` | M, parked | `contact_verify`'s three spellings. Waits on the D10 hold |

The P3 and P4 rows went to PREDTAIL in the 2026-10-25 priority-seam cut.

## Order

The sitting's frame is to work the P1 rows that INTENT's remaining
stages will not make irrelevant. INTENT builds editor-core's
dependency, placement and coincidence model. These rows are kernel
numeric predicates. The two that sit on D10 stage 4 ground (the
declared-contact verifier, `carrier_eq`) are parked on the hold, or
called out where they share a row.

1. In parallel:
   - `D292`, `S18`'s step-export half, and the E pair (`lever-arm`'s
     remainder, `D57`);
   - the designer pairs on `period-headroom-margin-has-no-shared-home`
     and on `S116p`.
2. The period-headroom unit, on the designers' answer.
3. `D46`, cut by file, skipping the stage 4 names.
4. `cone-nappe-is-decided-in-five-places` once its two PRs merge.
5. `S29`'s policy write-up as an `[ev]` PR.

## Review posture

The repo-wide tiers (`memories/orchestration-model.md`). The tier is
named per unit at dispatch, in the log. A unification that changes
which margin decides a reachable answer draws at least a full single
review, because a wrong margin is reachable and silent. Moving a
predicate to a shared home is held to the charter's agreement test.
