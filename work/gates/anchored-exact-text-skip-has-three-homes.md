---
id: anchored-exact-text-skip-has-three-homes
kind: issue
title: The anchored exact-text skip with an abstaining subject check is hand-spelled in three gates and lib.sh has no home for it
status: open
opened: 2026-09-06
refs: [unanchored-definition-skip, D211]
---


## Finding

Filed by the GATES orchestrator from PR 2029's style review (Q1). Three
gates now spell the same mechanism by hand — "an exact-text skip
anchored at its home path, with a subject check that reds when the
home file is present but the text is not, and abstains when the file is
gone":

- `scripts/gates/bounds-allowlist.sh` — `DEFINITION_HOME`,
  `DEFINITION_HOME_RE`, the plain texts and their hand-escaped `_RE`
  twins, `gate_definition_skip_subject` with `[ -f … ] || return 0`,
  `gate_definition_skip` filtering `^$HOME_RE:[0-9]+:$DECL_RE$` (PR
  2029, copied from the twin below and said so).
- `scripts/gates/no-extra-real-bounds.sh` — `SEALED_HOME_RE`,
  `gate_sealed_skip_subject`, the filter at `:133`,
  `plant_sealed_home_clean` and `plant_sealed_decl_elsewhere`.
- `scripts/gates/viewer-module-kinds.sh:485` — the same anchored skip
  with a stronger subject check (a missing exception file is a red at
  `:458`, not an abstention).

The row `unanchored-definition-skip` was the cost of the first copy
drifting from the second; the PR that closed it copied the shape by
hand and the review found the copy missing one of the twin's fixtures
(the at-home passes-case). A `lib.sh` helper — `gate_exact_skip HOME
TEXT…` building the anchored filter, the subject check and both
planted cases from the plain text once (so no hand-escaped `_RE`
twin can drift) — is the fix shape; the three gates call it.
Sequenced after PR 2029 lands.

**What was not measured**: whether `viewer-module-kinds.sh`'s
stronger subject check is the rule all three want (a missing home as a
red rather than an abstention), which is decided when the helper is
written.
