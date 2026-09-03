---
id: boolean-error-has-no-fieldless-kind
kind: issue
title: topo::BooleanError has no fieldless kind, so editor-core's checks door degrades it to reason: String — the PathErrorKind shape, one crate down
status: open
opened: 2026-09-01
github: 1491
refs: [1490]
---

## From GitHub issue 1491

opened 2026-09-01, 0 comments.

(SMELL-UV orchestrator) Filed as the durable home for a deviation PR 1490's sweep disclosed and could not carry — the fix lives in `crates/topo`, Track Q's fence (S-BOOL's per the register).

**The defect.** `editor-core/src/checks.rs` (~:275-282) carries a boolean refusal as `reason: String` because `topo::BooleanError` is `#[derive(Debug)]` only — no `PartialEq`, no fieldless kind — so a typed refusal degrades to prose at the checks door, and any consumer wanting the class back must substring-match. This is the exact shape §D row `D39` closed for `profile::PathError` in PR 1490: a fieldless kind enum (`PathErrorKind`, 28 variants, projected by an exhaustive `kind()`), carried beside the rendered prose, letting tests and doors match on type. The site's own comment discloses the degradation.

**The fix shape, established by precedent.** A `BooleanErrorKind` + exhaustive `BooleanError::kind()` in `topo` (the `Attr`/`AttrKind` and now `PathError`/`PathErrorKind` convention), then the checks door carries the kind beside the prose. PR 1490's review notes the one-way residue of a hand-mirrored kind enum (a phantom kind variant reds only in a downstream exhaustive match) and that a `transition_table!`-style single declaration closes that direction — worth deciding once for this and the `NodeErrorKind` sibling (SMELL-UV's §D row for that is being minted).

Not taken by SMELL-UV: `crates/topo` is outside both its fences. Per the partition rule, filing is the handoff.

## Home

`work/bool/` — the issue names the fix as living in `crates/topo`, Track Q's fence, which S-BOOL's charter claims (SMELL track Q's topo rows).
