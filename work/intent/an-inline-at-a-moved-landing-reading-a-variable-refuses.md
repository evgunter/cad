---
id: an-inline-at-a-moved-landing-reading-a-variable-refuses
kind: issue
title: Inline at a moved landing refuses a world bundle that reads a variable: the landing's motion has no way into the values
status: open
opened: 2026-10-18
priority: P2
cost: M
parent: transform-retires-into-a-placement
---


`refactor::inline` (`crates/editor-core/src/refactor.rs`, the moved-landing check before the carry) splices a part whose instance sits off the identity. Each part placement's world mate is moved by the instance's motion (`mate::pin::moved_by`). A value on a world mate is charted from the world's own frame, though, which the move leaves where it is, so a bundle with values would land where its numbers say rather than moved with the rest.

D's rule: a world bundle that holds numbers alone (constants, or anonymous untoleranced typed values) is re-pinned at the copy's moved pose by `pin_at` (`InlineError::RepinRefused` if its geometry offers no frame). A bundle that reads a named or toleranced variable refuses, typed (`InlineError::ValueOnAMovedWorld`).

C's world pin carried no number, so an instance whose part stood where its bodies stand could be inlined at any landing by moving the pin. D writes `stand`'s constant zeros, which the re-pin still covers. What it narrows: a part copy whose values read a variable (`place_at`, then a value driven by a named parameter) cannot be inlined under a moved instance.

**Done when** the landing's motion enters the values as expressions over the landing's own numbers or variables, so the variable keeps driving the copy after the splice. Or the lever in `a-value-charted-on-the-target-reads-frame-carries-the-datum-chain` is pulled, which makes the move a read-only composition. Raised by the orchestrator's ruling on PR D.
