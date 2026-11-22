---
id: demos-tours-lockfile-is-stale-against-its-manifest
kind: issue
title: demos/tour/Cargo.lock is stale against its manifest: any build of the tour rewrites it, and cargo --locked refuses
status: open
opened: 2026-11-22
priority: P2
cost: C
---



## Finding

From EMBED's lane `embed/remeter` (2026-11-22), which built the tour's
walk (`cargo run --release --example embed_walk` from `demos/tour`) on
main `289a9a517`, `0cae9969f` and `ae076e050`.

Each build leaves `demos/tour/Cargo.lock` modified: cargo adds one line,
`"quantity"`, to one package's dependency list. With `--locked` it
refuses instead:

```
$ (cd demos/tour && cargo metadata --locked --format-version 1)
error: cannot update the lock file …/demos/tour/Cargo.lock because --locked was passed to prevent this
```

So the committed lock does not match the manifests it locks. Every lane
that runs the walk meets a dirty checkout, and has to restore the file
or carry an unrelated change.

## What is needed

The lock regenerated and committed, and whatever let it drift found: the
PR gate lints and tests `demos/tour` only when a diff touches it
(`docs/prompts/implementer-discipline.md`), so a workspace crate gaining
a dependency does not ask the tour's lock.
