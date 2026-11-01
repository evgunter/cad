# NAMECHECK — the plan

N0's checks: the denotation verifier, the coverage fuzzer, the `RoleSeg` form map, the storage-order perturbation, and the clauses that point at N0.

Opened 2026-10-20 by WIRE's cut. Nothing dispatched.

## The slate

**17.5 budget points** against 30.

| pri | item | cost |
|---|---|---|
| P2 | `a-denotation-verifier-checks-every-name-against-the-finished-body` | H |
| P2 | `a-coverage-fuzzer-finds-legal-bodies-that-refuse-naming` | H |
| P2 | `every-role-seg-variant-maps-to-one-n0-form` | M |
| P2 | `topo-has-no-arena-permuting-helper-for-a-storage-order-name-check` | M |
| P3 | `n0-the-duplicated-naming-clauses-point-at-n0` | M |

## Order

1. **The form map first.** It is small, and the verifier reads it: each `Made` variant's doc lists the structural fields the verifier confirms.
2. **The storage-order helper next.** It is independent of the map and can run in parallel.
3. **The fuzzer.** Every `Naming(*)` refusal it finds on a legal body is a WIRE naming-defect row. File it there; do not fix it here.
4. **The pointer rewrite last,** once the checks it points at exist. It is a re-wording of ratified clauses with no change of substance, so it lands with its own change under CLAUDE.md's re-wording rule.

## Review posture

Inherits WIRE's (`work/wire/plan.md`): protocol v7, single reviews; FULL where a check could pass vacuously.

**Class note from WIRE's 2026-10-20 sitting:** a check that cannot go red is the commonest failure here. PR 5460's VACUOUS lists, and its admission lists that red in both directions, are the precedent. Every new check is shown red by a mutant before it merges.
