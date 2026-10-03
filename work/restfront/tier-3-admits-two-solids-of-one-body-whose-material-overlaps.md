---
id: tier-3-admits-two-solids-of-one-body-whose-material-overlaps
kind: issue
title: Tier 3 admits two solids of one body whose material overlaps; only tier 3' census refuses it
status: open
opened: 2026-10-03
priority: P1
cost: M
---

Filed by FUSE's PR 3891 (the piece rule's sort), from its dual
review's NOTE-2.

Under Ev's ruling (PR 3901, `docs/DESIGN.md` "A solid is one piece of
material") the pieces of one body never overlap. Tier 3 does not check
it: check 10 (`crates/topo/src/validate.rs`, `shell_winding_errors`)
reads windings within ONE solid, and nothing in tier 3 reads across
solids. Measured on PR 3891's branch: `cube(0..6)` with `cube(2..4)`
grafted beside it as a second solid (`graft_disjoint_all_keyed`), and
the same with `brick((4,8),(2,4),(2,4))` overlapping it partly, are
both two solids, and `validate_geometric` returns `Ok(())` on both.
`validate_pseudomanifold` (tier 3′) refuses both: the nested pair with
`InstanceInterference`, the partial pair with `EdgeFacePierce` findings
and `InstanceInterference`.

Why it matters now: booleans, `shell` and `split` take bodies, and a
multi-solid operand enters their pipelines as one multi-shell solid
(`Body::merge_all_solids`), so an overlapping pair there is winding-2
material. The result sort refuses an `Outer` straight inside another
(`PieceSortError::Overlapping`), but a partly overlapping pair is two
crossing shells, which only the reduction's own crossing probes or the
census see.

The fix is the owner's: a cross-solid overlap check in tier 3 (check
10 widened over a body's solids, or the census's interference arm
lifted into tier 3), or a ruling that tier 3 certifies per solid and
tier 3′ owns the whole body.
