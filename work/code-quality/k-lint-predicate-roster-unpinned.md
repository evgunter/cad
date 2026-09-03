---
id: k-lint-predicate-roster-unpinned
kind: issue
title: k-lint's EPS_COUPLED_PREDICATES rosters the kernel's predicate vocabulary with no pin in either direction
status: open
opened: 2026-09-03
refs: [D204]
---

## Was

unrowed. Raised by the `D204` lane's cross-root-constant sweep as
**exactly `CHART_TAGS`' shape**, one tool over.

## Finding

`tools/k-lint/src/lib.rs:257` holds
`EPS_COUPLED_PREDICATES = ["props_quad_converged"]` — a roster of the
*kernel's* predicate vocabulary, held in a workspace-excluded consumer.
The name it rosters is minted at `crates/geom-brep/src/props/quad.rs:560`
and `:2993`.

Nothing pins the two together in either direction. `tools/k-lint`
contains no `include_str!` — unlike `tools/tess-meter`, which reaches
into `tools/tess-lint`'s source precisely to pin a constant across a
cargo-root boundary without taking a dependency — and nothing in
`crates/` mentions the roster. So:

- a predicate the kernel **renames** leaves the roster naming nothing,
  and the ε-coupled arm silently stops applying to it;
- a predicate the kernel **adds** to that class is absent from the
  roster and is judged by the wrong rule.

Both directions are silent, which is the half that matters: this is a
lint whose whole job is to decide which margins are ε-coupled, and its
input vocabulary can drift out from under it without anything reddening.

**Why it is not `D204`'s work.** `D204` pinned `CHART_TAGS` from the
meter's side, where an `include_str!` reader already existed to extend.
Closing this one needs either an edit under `crates/` (Track Q's
`props/quad.rs`) or a new reader on the k-lint side, and `tools/k-lint`
has none to extend. A row landing on it draws that fence first.

**Note on the sweep that found it.** The pattern was `const`
declarations, and it cannot see a shared vocabulary that is not a
`const` — `Chart::tag`, the producing half of the constant `D204`
pinned, is match arms returning string literals and was invisible to
the same sweep. Expect siblings spelled that way.
