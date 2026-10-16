---
id: implementer-discipline-slow-set-sentence-is-stale
kind: issue
title: implementer-discipline §2 says the PR gate runs only the slow tests of the crates a diff touches; a seedless all-tier diff runs the whole slow set
status: closed
opened: 2026-10-15
priority: P4
cost: E
closed: 2026-10-16
---


## Found

`docs/prompts/implementer-discipline.md` §2 ("Verification") lists what the
per-PR gate runs as including "the slow tests of the crates your diff
touches". Since PR 5206 that is true only of a diff that seeds crates. On an
`all`-tier diff with no seed (a workflow, root manifest, toolchain or
nextest-config change), ci.yml's slow-set step in the `test` job runs the
whole slow set, with package expression `all()`.

## Wanted

Reword the sentence to cover both cases. For example: "the slow tests of the
crates your diff touches (all of them on a workspace-level diff that touches
no crate)".

This edit waits on Ev. `docs/prompts/` binds every lane, and CLAUDE.md puts
changes to it under Ev's sign-off, so it ships as an `[ev]` PR and is not
edited directly.

— CIW, from PR 5206's review

## Closed (2026-10-16)

Ev signed off on #5224 ("it looks good to merge now!"). The slow-set half fell away when Ev ruled that a workspace-level file adds no slow tests. §2 gains only the changed-test-file clause from #5211: the 1e-6 and 1e-12 rows also run "every test file it adds or edits".
