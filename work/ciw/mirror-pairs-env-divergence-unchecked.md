---
id: mirror-pairs-env-divergence-unchecked
kind: issue
title: no check compares the env a mirrored CI pair runs under, so a deliberate divergence and a dropped variable look the same
status: open
opened: 2026-09-04
refs: [mirror-parity-never-compares-flags, 1759, 1739]
---


## What

`scripts/check-ci-mirror-parity.py`'s claim 10 (PR 1759) compares the
semantics-bearing **cargo flags** on the two sides of a `HOSTED MIRROR`
pair. It reads nothing about the **environment** those commands run
under, and the checker's header says so at the claim.

Nothing else reads it either. `scripts/check-cache-prime-parity.py`
compares `env:` blocks, but only between a build job and the
`cache-prime` job that warms its cache — a hosted-to-hosted pair, never a
hosted-to-local one. So for a mirrored pair, the environment is checked
by no gate in either half.

## Why it is worth an item

**This is `mirror-parity-never-compares-flags`'s own shape, one axis
over.** That issue's argument was that a roster check cannot say whether
two paired rows run a check the same way; the same is true of a flag
check that stops at argv. A variable dropped from one half changes what
the run means and leaves no future red — it merges once, silently.

The live instance is a **correct** divergence, which is what makes the
hole hard to see:

- `.github/workflows/render.yml:563` (`scene-inputs`) and `:998`
  (`montage`) set `CAD_RENDER_LOCAL_OVERRIDE=i-am-the-hosted-renderer`.
- `local-scripts/ci-local.sh:485` (`uv_sheet_drift`) sets
  `CAD_RENDER_LOCAL_OVERRIDE=i-accept-local-render-drift`.

`scene-inputs` is cited by two `HOSTED MIRROR` markers
(`local-scripts/ci-local.sh:481-482`), so this is a pair claim 10 reads
and passes. The divergence is deliberate and ratified (PR 1739): the two
sentences mean different things and each half needs its own. A checker
that had no way to say that would be wrong to fire — but today nothing
can say it either way, so a variable that simply went missing is
indistinguishable from this.

## Shape of a fix, not settled

The vocabulary already exists next door: `check-cache-prime-parity.py`
reads a job's whole `env:` block as text. What a mirrored pair needs on
top of it is a declaration form for a divergence that is meant — the
same "declare your asymmetry in a sentence" shape `MIRROR_EXEMPT`,
`GATE_MODE_EXEMPT` and `FLAG_EXEMPT` use, keyed on `(pair, variable)`.

Two things to settle before writing it, and neither is obvious:

1. **What counts as the env of a local row.** Hosted's is a job-level or
   step-level `env:` block, which the recogniser already parses. The
   local half's is an inline assignment prefixing a command, and reading
   those means the shell reader growing a second job.
2. **Whether the population is worth it.** `CAD_RENDER_LOCAL_OVERRIDE`
   may be the only variable a mirrored pair sets on either side today.
   A claim with one member, whose one member is an exemption, is a claim
   that gates nothing — count the population first and close this issue
   if the answer is one.

## Provenance

Found in review of PR 1759 (`mirror-parity-never-compares-flags`): the
reviewer noted that the checker's scope line implied env parity would
need a vocabulary invented for it, when `check-cache-prime-parity.py`
already does that kind of comparison. The scope line was corrected in
that PR and the gap filed here rather than widened into claim 10.
