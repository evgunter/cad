# GAUTH A/B state — BRANCH-SIDE ONLY until each block closes

Not merged to main while a block has unstarted slots (a block record
naming unstarted slots leaks the remaining arms by arithmetic). The
CONCLUDED record folds into docs/MODEL-AB-LOG.md at block close.

## Block GAUTH-B1 (drawn 2026-08-31, after the plan's difficulty
## pre-logs reached main in the opening commit 58500af)

Draw: /dev/urandom byte **196** (reject-≥252 rule; 196 mod 4 = 0)
⇒ fable slot 0; opus slots 1–3.

- slot 0 → GAUTH-1 (difficulty L, pre-logged): FABLE
- slot 1 → GAUTH-2 (M, pre-logged): OPUS
- slot 2 → GAUTH-3 (M, pre-logged): OPUS
- slot 3 → GAUTH-4 (M, pre-logged): OPUS

## Block GAUTH-B2 (drawn 2026-08-31, at GAUTH-5's dispatch; B1 fully
## dispatched — its concluded record folds into MODEL-AB-LOG when all
## four rows are recorded at merge)

Draw: /dev/urandom byte **34** (reject-≥252; 34 mod 4 = 2)
⇒ fable slot 2; opus slots 0, 1, 3.

- slot 0 → GAUTH-5 (difficulty M, pre-logged): OPUS
- slots 1–3 → unassigned (follow-up units if any)
