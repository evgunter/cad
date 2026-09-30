---
id: dl5-has-no-arm-for-a-selection-whose-result-is-the-payload
kind: issue
title: DL5 has no arm for a selection whose result IS the payload (map_refusal's nearest-fit read)
status: open
opened: 2026-09-13
priority: P1
cost: D
---


## Finding (BLEND-11's delta, R1, 2026-09-13)

`crates/geom-core/src/real.rs` (DL5's home, ~`:1087`) and
`docs/DUAL-DESIGN.md` enumerate two admissible reads of the diagnostic
channel: (a) a payload read, and (b) a selection among constructions
whose selected quantity is locally constant. `Why::Selection`'s own doc
in `crates/geom-core/tests/bounds_census.rs` (~`:70`) calls the
locally-constant condition "load-bearing". BLEND-11 (PR 2495) added a
third shape at `crates/profile/src/path/arc_fillet.rs::map_refusal`: a
selection among classified refusal candidates whose RESULT is the
payload rendered to the author — the alternatives carry different
payloads by construction, so the selected quantity is not locally
constant, and the roster row (`bounds_census.rs` ~`:238`) says so
honestly ("the locally-constant clause is not what carries this
site"). The roster now holds a `Selection` member its own variant
definition does not describe, and DL5 has no arm for it; the next site
of this shape has nothing to cite.

## The ask

Either DL5 gains a third arm — a selection whose selected quantity is
itself a payload, admissible when nothing downstream of the read
re-enters the computation — with `Why::Selection`'s doc widened to
match, or the census gains a variant for it. `geom-core/src/real.rs`
and `docs/DUAL-DESIGN.md` are ratified ground (`work.py territory`
names no live program for `real.rs`); a design revision, discussed with
Ev, not a lane's edit.
