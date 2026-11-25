---
id: not-valids-pcurve-finding-names-no-edge-so-the-klein-elbow-stand-down-reads-any-rim
kind: issue
title: ShellError::NotValid's pcurve finding names no edge of a body it carries, so the klein elbow's stand-down accepts any pcurve_envelope escalation
status: open
opened: 2026-11-25
priority: P3
cost: E
---


## The finding

PR 7497 retired the shell's closing mint, and with it `ShellError::Face`
carrying a `ReplaceFaceError::Pcurve` on the result body. A row refusal
now arrives as `ShellError::NotValid { findings }`, the finish's finding
set, and its `ValidationError::Pcurve` names a `HalfEdgeKey` of the
assembled thin solid, a body the refusal does not carry
(`crates/topo/src/shell.rs`, the `NotValid` arm;
`crates/topo/src/finish.rs`, `NotFinished`).

So `torax_interval::interval_the_klein_elbow_hollows_to_its_measured_volume`
(`crates/sweep/tests/torax_interval.rs:432`, `envelope_escalation` at
`:446`) can no longer check that the escalating row is a *derived spiric
rim* rather than a re-authored seam (the old `rim_envelope_escalation`
read the operand's edge). It stands down on any `pcurve_envelope`
escalation of the finding set at a strict band: a loosened pin.

## What tightens it again

`NotValid`'s findings name the edge in terms the caller holds: the
operand's edge the row's edge was offset from (the shell's record maps
result edges to operand edges), or the finding carries the edge's
carrier kind. Then the stand-down restricts to a rim edge again.
