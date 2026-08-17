---
name: schema-claim-discipline
description: Schema-version bumps — same-number double-claims MERGE CLEAN (git sees identical content, no conflict); the by-eye check against main's live constant is the ONLY guard; claim prose in the ledger is the tripwire
metadata:
  type: project
---

**The hazard (proven live, 2026-08-16, #552's final re-merge; also
the v7/v8 incident before it):** when two in-flight branches bump
`SCHEMA_VERSION` to the SAME number, the merge is CLEAN — git sees
identical constant content and raises no conflict. The earlier
ratified rule assumed "the constant conflicts in any merge, forcing
a conscious resolve" — that is TRUE only when the two sides hold
DIFFERENT numbers; a same-number race sails through silently and
ships two vocabularies under one version.

**Why:** textual merge, not semantic. Identical bytes ≠ compatible
claims.

**How to apply (the discipline, both halves):**
- **Claim loudly at dispatch**: the number + reasoning goes in the
  SCHEMA_VERSION doc comment AND as claim prose in the shared
  ledger (docs/MODEL-AB-LOG.md) — pushed to MAIN at claim time,
  not parked on the branch. The prose caught the #552 incident;
  BUT (sharper, #575's v12→v13 shift): prose appended in
  different ledger regions does NOT conflict either — prose is a
  tripwire, never a guarantee.
- **Check by eye at final re-merge, as an explicit step — the
  ONLY reliable guard**: read
  main's ACTUAL constant immediately before setting yours
  (`git show origin/main:<persist file> | grep SCHEMA_VERSION`),
  take main's next number, and state in the PR body that the check
  ran and what main held. Never rely on a merge conflict to catch
  a collision.
- Deterministic race rule stands: whoever reaches main first keeps
  the number; the other shifts at final re-merge; goldens/fixtures
  re-bless header-only; a gap in the sequence costs nothing.

See [[git-workflow]], [[agent-lane-operations]] (the CONFLICTING =
silent-outage sibling lesson: both are "the absence of a loud
signal is not evidence of safety").
