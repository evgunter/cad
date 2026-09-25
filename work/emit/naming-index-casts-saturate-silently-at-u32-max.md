---
id: naming-index-casts-saturate-silently-at-u32-max
kind: issue
title: editor-core index and count casts saturate or wrap at u32::MAX instead of refusing
status: closed
opened: 2026-09-24
priority: P4
cost: E
closed: 2026-09-25
pr: 3226
---

`names/emit_sweep.rs::ix` (`:63` at filing) turns a loop, segment or vertex
index into the `u32` the role vocabulary stores with
`u32::try_from(i).unwrap_or(u32::MAX)`. It has 9 callers in that file.
Past `u32::MAX` two different indices would spell one name, so a profile
entity would alias another without anything saying so. This is the shape
`name-counts-saturate-silently-at-u32-max` fixed at its three sites, where
the emitters now refuse with `NamingError::Emission` (`emit_topo.rs::group_count`).
That row listed only the count sites. This index site turned up in its sweep.

It cannot be reached: no profile holds that many loops or segments. It
still breaks the fail-loud rule. The fix is `ix` returning
`Result<u32, NamingError>` and its callers using `?`.

Checked and left alone: `names/emit.rs::name_pattern`'s
`output_body(usize::try_from(j).unwrap_or(usize::MAX))` saturates only to
hand the value to `output_body`, which refuses anything over `u32::MAX`
anyway. So that site is already loud, although it reads oddly.

## The truncating `as u32` casts beside it

A bare `as u32` is worse than `ix`: past `u32::MAX` it WRAPS, so the
stored value is small and plausible rather than pinned at the maximum.
`rg 'as u32\b' crates/editor-core/src` (non-test, at filing) finds 14.
None is reachable at any holdable size. The disposition says what each
would become. The helper they should use is `names::emit::to_u32` (or an
`Option`/typed-refusal equivalent where the site is not an emitter).

| site | what it stores | disposition |
|---|---|---|
| `names/emit_shell.rs:129` `hole: j as u32` | a `HoleRim` name's hole index | **fix**: a name. A wrapped index makes two holes spell one name. Use `to_u32` with `?`, as `ix` will |
| `resolve/mod.rs:885` `ents.len() as u32` | the candidate count in `ResolveError::ambiguous` | **fix**: a refusal would report a wrong count. Use a fallible narrowing, or change the field's width |
| `resolve/mod.rs:1167` `ents.len() as u32` | the same, on the `Tied` arm | **fix**, as `:885` |
| `resolve/mod.rs:1182` `ents.len() as u32` | the same, on the fragment-base arm | **fix**, as `:885` |
| `eval/wire.rs:2606` `ents.len() as u32` | `Landing::Tied` candidate count | **fix**, the same count as resolve's |
| `node.rs:2918` `first as u32` | `InputFault::RepeatedDesignation` position | **fix**: a fault would point at the wrong entry |
| `node.rs:2919` `again as u32` | the same, second position | **fix**, as `:2918` |
| `node.rs:2932` `at as u32` | `InputFault::SelectionNotCanonical` position | **fix**, as `:2918` |
| `eval/anchor.rs:274` `pi as u32` | `LoopAnchor::program_loop` | **fix**: an anchor that wraps anchors to another loop |
| `eval/anchor.rs:275` `offset as u32` | `LoopAnchor::offset` | **fix**, as `:274` |
| `eval/anchor.rs:277` `n as u32` | `LoopAnchor::len` | **fix**, as `:274` |
| `eval/wire.rs:1559` `li as u32` | `NodeErrorKind::ProfileReplay::loop_` | **fix**: an error naming the wrong loop |
| `eval/wire.rs:1634` `li as u32` | `NodeErrorKind::ProfileLaneReplay::loop_` | **fix**, as `:1559` |
| `names/role.rs:297` `(a as u32).cmp(&(b as u32))` | the low half of a walk stamp | **leave**: the truncation is the point. The high 32 bits are the epoch, already compared equal on the line above, and the low 32 are the position |
