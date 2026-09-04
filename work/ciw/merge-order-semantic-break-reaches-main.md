---
id: merge-order-semantic-break-reaches-main
kind: issue
title: Two green PRs merged 22 minutes apart left main non-compiling: no run ever gates the union
status: open
opened: 2026-09-04
refs: [f3-recosting-on-a-public-repo, main-latently-red-at-tier-all, 1725, 1769, 1792]
---


Found by the CIW lane for `geom-brep-test-unused-edgedescription-import`,
whose first act was to run its proposed row's invocation against
`origin/main` at `a2bb8438`. The row did not reach its subject: the
build failed first, on a workspace member, at DEFAULT features.

## The break

```
error[E0004]: non-exhaustive patterns: `&editor_core::MateFault::Unleverable { .. }` not covered
   --> crates/viewer/src/tree.rs:317:11
```

Reproduced at `a2bb8438` with `cargo clippy -p viewer --all-targets`
(no feature flags, exit non-zero), so it is not an `--all-features`
artefact: `crates/viewer/src/tree.rs` is not `app`-gated, and the
member does not compile.

## Neither PR could have seen it

- `crates/viewer/src/tree.rs`'s `blamed_mates` — the exhaustive match —
  arrived with CHROME PR 1769, merged `bdb5cea1`, 2026-09-03 22:04:54
  -0700. It does not exist in the file at `77f50472`.
- `MateFault::Unleverable` arrived with M10-7 PR 1725 at `77f50472`,
  merged `50d9ba21`, 2026-09-03 22:26:52 -0700 — 22 minutes later, from
  a branch on which `blamed_mates` did not exist.

Each branch compiles. The union does not, and no run compiled the
union. PR 1725's last COMPILING run is 33838705794, created 04:57:24Z
and green; every later head on that branch is docs-only, and the last
of them (33840515302) started 05:26:50Z, two seconds before the merge.
CHROME's `blamed_mates` reached main at 05:04:54Z — after 1725's last
compiling run and before its merge — so no run on either PR had both
changes in its tree, and `ci.yml`'s compiling jobs all carry
`github.event_name != 'push'`, so the merge that created the
combination re-gated nothing.

The M10-7 lane was not careless about this class; it swept it. Its own
commit at `61e5165b` fixes the identical shape one crate over —
"`MateFault`'s `mate` getter matches every arm exhaustively, and the
new Unleverable arm names a mate" — in `pncad-py`, found by a hosted
run. What it could not sweep is a match that did not exist in its tree
and appeared under it 22 minutes before its merge.

## Why this is not `main-latently-red-at-tier-all` again

That item's mechanism (closed 2026-09-04) is a push run whose drawn
tier SKIPS a job that would have fired — one tree, one broken check,
unexecuted. This one has no broken tree to skip: every tree either
side of the merge is green, and the defect exists only in their union.
A wider push-run tier would catch this one; nothing narrower can,
because there is no earlier tree on which the check fails.

## Disposition

**The instance was fixed independently, not here.** `c825bbd2` (PR
1792) landed the same one-line arm while the CIW lane was measuring —
`MateFault::Unleverable { mate, .. }` joins the single-mate group, the
fault carrying a `mate` field and `blamed_mates`' own header saying
every arm but `Band` names its subject. The CIW branch carried a
byte-identical arm for the same reason and dropped it on merging main,
so nothing about the repair is CIW's and no CHROME/VIEW adjudication is
owed.

What is CIW's is the mechanism above, which is what this file records.
It is evidence for `f3-recosting-on-a-public-repo`, whose subject is
what a main push should re-gate, and it argues for a floor that is a
function of MERGE ORDER rather than of tier: the two trees this defect
lives between are each green, so no wider per-PR tier reaches it.

Not measured here: how often this shape fires. One instance in one
night is one instance, and the 34 min 25 s main stood
non-compiling — 05:26:52Z, #1725's merge, to 06:01:17Z, #1792's — is
one sample of exposure, not a distribution.
