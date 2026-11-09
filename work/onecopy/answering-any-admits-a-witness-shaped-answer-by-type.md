---
id: answering-any-admits-a-witness-shaped-answer-by-type
kind: issue
title: Answering::Any admits a witness-shaped answer: make the climb's mode a type-state that bounds Any to Alike answers
status: open
opened: 2026-11-09
priority: P2
---


Follows `an-any-climb-answering-with-a-witness-is-first-shaped`. That row's
sites were fixed one by one (PR for `onecopy/any-witness`). Nothing stops the
next one: `climb(Answering::Any, …)` accepts an answer of any type, so a
witness (which gap, which vertex, which limb) can be answered under `Any`
and silently depend on rung order.

## Ruling

Orchestrator, 2026-11-09: **option A**.
- **A (chosen):** type-state modes `Any` and `First` behind a sealed
  `Answers<A>`.
  - `impl<A: Alike> Answers<A> for Any`, `impl<A> Answers<A> for First`.
  - `climb<M: Answers<A>, …>(mode: M, …)`.
  - `Alike` is a marker on answers every answering rung gives alike: `()`,
    `bool`, and the side, sign, parity and verdict enums, each impl'd beside
    its type.
  - A witness carried under `Any` is wrapped in a `Diagnostic<W>` that
    implements `Alike`, with the inner value reachable only as a diagnostic.
  - A witness-shaped answer under `Any` then fails to compile.
  - `Climb::Answered::at` stays as it is; its doc already says it is a
    diagnostic only under `Any`.
- **B (rejected):** two entry points, `climb_any` and `climb_first`. It
  doubles every entry point (`climb_evidence`, `climb_agreeing`).
- **C (rejected):** a text gate over `Answering::Any` sites. The parent row
  already shows such a grep misses a mode passed through a variable
  (`boolean::rim_wedge::rims`).

## Sequencing

Build it once the ladder's move to `geom_core::ladder` (T3, after PR 6675)
is on main. The orchestrator will say when it has landed.

## Scope

- Every `climb`, `climb_evidence` and `climb_agreeing` caller: about 57
  non-test `Answering::Any` sites across 18 files, plus the `First` sites.
  The call-site spelling changes mechanically.
- `rim_wedge::rims` takes its mode as a parameter and answers
  `Option<(Rim, MarginDiag)>`. Under the type route its `Any` caller
  (`rim_exists`) answers a witness-free `bool` itself, or wraps the rim
  in `Diagnostic`.
- The answers currently known to carry a witness under `Any`, each kept
  with a stated reason:
  - `topo::replace_face::gaps_apart`
  - `topo::census::wedges_overlap`
  - `topo::census::probe_vertices`
  These take `Diagnostic<…>`.
