---
id: a-new-public-variant-skips-its-downstream-exhaustive-matches-in-the-change-filter
kind: issue
title: The PR change filter does not follow a new public enum variant into downstream exhaustive matches, so a topo-only diff can break editor-core's build unseen
status: open
opened: 2026-10-26
priority: P2
cost: M
---



Found by RESTREAD while chasing `restread/main-red-coincident-wedges-overlap-refuses-flush-copies` (PR 5804).

## What

PR 5704 (`restread/point-touch-winding`) added two public
`topo::ValidationError` variants, `BoundaryCrossesItself` and
`BoundaryCrossesItselfUndecided`. Its diff touched topo only, so the
PR's change filter ran topo and sweep and skipped `editor-core`, whose
exhaustive `finding_solids` match then failed to compile (E0004). The
PR merged green, and main's push run stayed green for the same reason;
ATREST placed the variants in `863030db6b`.

The filter selects jobs by the paths a diff touches. A new variant of a
public enum, or any change to a public type's shape, breaks every
downstream crate that matches it exhaustively, whatever paths it
touches.

## Owed

Build every workspace crate that depends on a touched crate, at least
with `cargo check`, on every PR whose diff touches a library crate's
public surface. The cheap form is a `cargo check --workspace
--all-targets` job that the filter never skips. Nightly catches the
break today, but only after it has merged.
