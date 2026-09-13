---
id: remap-name-misses-lose-the-id-they-caught-at-six-refactor-sites
kind: issue
title: refactor.rs discards remap_name's Err(RecipeNodeId) at six sites, so a miss inside a name's PATH segment is reported as the whole name being stranded
status: open
opened: 2026-09-11
refs: [2378]
---


## Finding

Found by WIRE's `names-refusal-carries-cause` lane (PR 2378) in its
`map_err(|_| ..)` sweep over `crates/editor-core/src/`; outside its
fence, filed here by the WIRE orchestrator because
`crates/editor-core/src/refactor.rs` is FIX's by its `paths`. Accurate
at `af8bbca`.

Six sites — `refactor.rs:831`, `:1504`, `:1544`, `:1807`, `:1840`,
`:1858` — call `remap_name(…, &node_map)` and `map_err(|_| …)` the
result into `RemapMiss::Name`, `SplitError::NameStraddlesCut` or
`InlineError::StrandedPartName`, discarding `remap_name`'s
`Err(RecipeNodeId)`.

**Why the id is not redundant with the name the raised error carries.**
A `StableName` embeds other names in its `path`, so the node that could
not be remapped may be in a **path segment** rather than the name's own
`node` field. The raised error names the outer name; the discarded id
names the node that actually failed. For a nested name those are
different, and the one a reader needs to act on is the one thrown away.

The lane classed this `reported` rather than `not-this-unit` precisely
because of that: it is not the information-free `TryFromIntError` shape
that the rest of the sweep's hits are.

## What a taker owes

The id carried on each of the three error kinds, or a stated argument
that the outer name suffices — which would need to say what a reader
does when the outer name is nested and the miss is two segments down.
PR 2378's argument for carrying is worth reading first rather than
re-deriving.

**Where else to look**, since six sites in one file is a class and not
an accident: every other caller of `remap_name` in `refactor.rs`, and
the same question for any sibling remapper that returns a located error
into a call site that raises a name-shaped one.
