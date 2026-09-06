---
id: lib-per-arm-error-tags
kind: issue
title: per-arm Python error tags: one tag per op hides which arm refused
status: open
opened: 2026-09-03
---


Banked at LIB-TUBE (#1628), filed at its fix pass because a banked
finding with no item is a finding nobody can pick up.

`crates/pncad-py/src/tags.rs`'s `node_error_tag` gives every op ONE
tag. `revolve` covers all ten `RevolveError` arms; `tube` covers every
`TubeError` arm, including the three that only `hollow_tube` can
raise. So from Python a wall refusal and a frame refusal are the same
tag and differ only in prose, which is exactly the discrimination the
tag exists to spare a caller from parsing.

Not a LIB-TUBE defect and deliberately not fixed there: the tube
followed the convention every other op already sets, and changing the
convention for the one op whose unit happened to be written last would
leave the map inconsistent in a new way. The work is worth doing for
EVERY op at once — one pass over `node_error_tag`, deciding per family
whether the arms a caller can act on differently deserve their own
tags, with the census rows moving together.

Measured at LIB-TUBE: `NodeErrorKind::Tube` carries `Box<TubeError>`,
whose `Display` names the door on every arm, so the information is on
the wire today — it is only the TAG that collapses it.
