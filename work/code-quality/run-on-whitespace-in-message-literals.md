---
id: run-on-whitespace-in-message-literals
kind: issue
title: Five user-facing message literals carry a run of spaces from a lost line continuation
status: open
opened: 2026-09-04
---

Found by CHROME's style lane reading `crates/viewer/src/pick.rs` end to
end (the brief's Q8), then swept.

A hand-wrapped string literal that loses its trailing `\` keeps the
source indentation inside the message. The rendering a user sees then
carries a run of spaces mid-sentence:

```
that body draws 4 edges, so                  this address was not one
this index handed out
```

Five instances, all pre-existing and none belonging to one program:

- `crates/viewer/src/pick.rs:1771`
- `crates/geom-brep/src/nurbs_iso.rs:112`
- `crates/topo/src/boolean/reduce.rs:2126`
- `crates/topo/src/chart_region.rs:816`
- `crates/topo/src/props.rs:1673`

**Nothing can see them today.** `crates/viewer/tests/error_display.rs`
asserts that refusals read as prose rather than as debug dumps, but its
`debug_shaped` predicate looks only for `" { "` and a variant name — a
run of spaces is prose by that test.

Sweep that found them, offered as the guard's shape rather than as a
finished pattern: `rg '"[^"]*[a-z] {4,}' crates/*/src`. It matches a
lower-case letter followed by four or more spaces inside a literal. It
would miss a run that begins after punctuation or a digit, and it
cannot see a literal assembled from `concat!` or `format!` fragments.

Signed: (CHROME orchestrator)

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/code-quality/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

**No track letter, deliberately.** The five sites are in `viewer/src`
(CHROME/VIEW), `geom-brep/src` (Track R), `topo/src/boolean` (Q),
`topo/src/chart_region.rs` (Q) and `topo/src/props.rs` (M) — a
cross-cutting prose sweep that collides with every fence, which is the
`L1`/`L2`/`C21` shape `work/code-quality/plan.md` says goes after the
tracks empty. It joins the unlettered pile rather than taking a letter
it would have to break.
