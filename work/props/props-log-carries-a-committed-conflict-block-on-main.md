---
id: props-log-carries-a-committed-conflict-block-on-main
kind: issue
title: work/props/log.md carries an unresolved conflict block on main — a fourth instance of the class, and the gate it asked for still does not exist
status: open
opened: 2026-09-21
priority: P2
cost: E
refs: [committed-conflict-markers-reach-main]
---


Found by `vgeom/render-spelling` while resolving its own `work/` merge
conflict: a tree-wide marker grep, run to check the lane's resolution,
hit a file the lane does not touch.

## The finding

`work/props/log.md` on `origin/main` carries a literal, unresolved
conflict block — `<<<<<<< HEAD` at `:1868`, `=======` at `:1981`,
`>>>>>>> origin/main` at `:2014`, about 146 lines. Executed:

```
git show origin/main:work/props/log.md | grep -n '^<<<<<<<\|^>>>>>>>\|^=======$'
```

The HEAD half ends mid-narrative (*"if a unit looks ready to dispatch,
it waits"*) and the other half is FIX's announced seam for PR 2948,
signed. Both halves are content somebody wrote; neither is quoted
prose. So the file's tail — which `work/README.md` calls *"the
program's story"* and which a successor orchestrator reads to learn
the live state — is two narratives with a marker between them.

## It is a fourth instance of a class already written up and closed

`work/issues/committed-conflict-markers-reach-main` (closed
2026-09-04) named three: `docs/KERNEL-VERBS.md`,
`crates/pncad-py/src/py/mod.rs` and `docs/MODEL-AB-LOG.md`. It asked
for a one-line gate —
`git grep -nE '^(<{7}|={7}|>{7})( |$)'`, anchored to line starts
because the SMELL logs quote markers mid-line — and filed it for track
J / S-QA, *"closed with track J empty"*.

**That gate does not exist**, which this instance measures rather than
assumes: the block is on `main` today, and `main` is green. The row
was closed as a finding with a home, not as a repair.

## Two things, and they are separable

- **The block**, which is PROPS's to resolve: keep both halves in
  merge order, the resolution `work/view/plan.md`'s register
  prescribes for an append-only log, and check nothing was lost from
  either side.
- **The gate**, which is not PROPS's and is not filed here. It is one
  line in a CI step over the whole tree, and the class row already
  wrote it. Whoever owns `.github/workflows/*` today takes it; this
  row's only claim about it is that four instances over three weeks
  is enough evidence, and that the class row's argument has gone
  unanswered since it closed on 2026-09-04.

## Fence

`work/props/log.md` — PROPS's own. Not resolved here: an append-only
log resolved by a lane that cannot tell which half is which is how the
block got there.
