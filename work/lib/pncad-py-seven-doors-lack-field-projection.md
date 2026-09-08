---
id: pncad-py-seven-doors-lack-field-projection
kind: issue
title: pncad-py: seven refusal doors still cross as tag-plus-message, with no per-variant field projection
status: open
opened: 2026-09-01
github: 1479
refs: [730]
---

## From GitHub issue 1479

Opened 2026-09-01; 0 comments.

`crates/pncad-py/src/tags.rs` carried an unowned deferral in its module header — *"Full per-variant field projection (node ids, slots, operand roles) is deferred to the unit that binds the complete surface"* — naming no unit and no criterion. Re-derived against the tree (2026-09-01): most doors have since grown the projection, so what the sentence describes is now a **seven-door remainder**, and this issue is its owner. The header no longer defers; it names the seven and points here.

## What the surface looks like today

Doors that project every arm's payload as attributes, exhaustively, with every attribute present on every arm and `None` where inapplicable (so `getattr` never raises and a caller need not branch on `variant` first):

`tessellate` (`py/mesh.rs`), `readback` (`py/readback.rs`), `select` (`py/select.rs`), `evaluation` (`py/value.rs`), `literal` (`py/doc.rs`), `mate` (`py/mate.rs`), `assembly`/`product` (`py/assembly.rs`), `split`/`inline`/`update` (`py/refactor.rs`), `workspace` (`py/store.rs`), `checks`/`enforce` (`py/checks.rs`).

Doors that cross with `variant` and the message only:

| door | kernel type | payload a caller cannot reach |
|---|---|---|
| `edit_err` (`py/doc.rs`) | `EditError` | node ids, slot indices, entity kinds, dimensions |
| `declare_err` (`py/doc.rs`) | `DeclareError` | its `Edit` arm's inner payload |
| `persist_err` (`py/doc.rs`, `py/store.rs`) | `PersistError` | schema versions, the mismatching ids, the failing site |
| `path_err` (`py/path.rs`) | `PathError<f64>` | the offending radius/leg/angle scalars |
| `frame_err` (`py/place.rs`) | `FrameError` | the degenerate direction, the tolerance |
| `stl_err` (`py/mesh.rs`) | `StlError`, `SolidNameError`, `BinaryHeaderError` | the offending byte/name |
| `step_import` (`py/value.rs`) | `StepImportError` | entity id and line — **has a stated reason at the site** (all 21 arms are reachable and the id/line live in the prose), so it is the one door here that is already argued rather than merely unprojected |

## Why it is worth closing

The rule the crate states is that a typed exception's payload is attributes and its message is prose for humans. At these six-plus-one doors the payload is one string, so a caller that wants the offending slot, version or scalar has to parse the message — which is exactly what the tag exists to make unnecessary. Two of them are the most-raised doors on the surface (`edit`, `persist`).

## Shape of the fix

Per door, the shape the projected doors already use — a positional tuple from one exhaustive `match`, no wildcard arm, so an arm added kernel-side is a compile error rather than a silently unprojected payload. `py/readback.rs::readback_err` and `py/refactor.rs::split_err` are the two worked examples.

Not one unit: `EditError` alone is ~40 arms, and each door's attribute set is a piece of Python surface that has to land in `pncad.pyi`, the binding census and the stub tests with it. Take them a door at a time.

## Notes

- `path_err`'s row is entangled with the `PathError` discriminant question (SMELL scan `D37`/`D39`): the kernel type carries no fieldless discriminant, so `crates/pncad-py/src/tags.rs::path_error_tag` hand-writes one. Projecting that door is worth doing after the discriminant exists, not before.
- The `variant` tag itself is present and exhaustive at all seven; nothing here is a drift-alarm gap.

## Home

`work/lib/` — the whole issue is the `crates/pncad-py` binding surface (its typed-exception payloads, `pncad.pyi`, the binding census), LIB's territory glob and charter.

## Question for Ev (2026-09-08, LIB orchestrator; `[ev]` PR)

The PAYLOAD half of the projection question (the arm half is asked on
`lib-per-arm-error-tags`). Six doors — `edit`, `declare`, `persist`,
`path`, `frame`, `stl` — cross as `variant` plus prose while every
other door projects each arm's payload as attributes, present on every
arm and `None` where the arm does not carry one (`readback_err` and
`split_err` are the worked examples). `step_import` is the seventh and
is already argued at the site.

- **(A) Ratify the rule and take the six as mechanical units, a door
  at a time.** "Every arm's payload is an attribute, every attribute
  present on every arm" is the crate's own stated rule; making it the
  ratified one means each door's attribute set is a mechanical
  consequence of the kernel enum (a positional tuple from one
  exhaustive match, no wildcard), landed with its stub, census and
  fixture rows — no per-door design conversation. `path` waits for the
  kernel-side `PathError` discriminant (SMELL D37/D39) as the issue
  notes. Recommended.
- **(B) Freeze attribute sets door by door** — each door's Python
  attribute names are surface Ev wants to see before they ship
  (`EditError` alone is ~40 arms). Six small `[ev]` PRs.
- **(C) Leave the six as tag-plus-prose** and say so at each site, as
  `step_import` does.

Recommendation: **(A)**. The attribute NAMES follow the kernel's own
field names, which are already Ev's design; what a unit decides is
only the flattening, and the worked examples fix that.

## Ruled (2026-09-08, Ev on `[ev]` PR 2196)

**(A)** — "A sounds good." The rule "every arm's payload is an
attribute, every attribute present on every arm" is ratified for the
binding; the six doors (`edit`, `declare`, `persist`, `frame`, `stl`,
and `path` once the kernel-side `PathError` discriminant lands) are
mechanical units, a door at a time or batched where the arms are few,
each landing the door's attribute set as one exhaustive match with its
stub, census and fixture rows. No per-door design conversation.
