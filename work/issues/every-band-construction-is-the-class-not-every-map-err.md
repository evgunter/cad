---
id: every-band-construction-is-the-class-not-every-map-err
kind: issue
title: The 'a typed cause is destroyed' sweeps key on map_err, and the class has more spellings: let-else, expect, From impls and unit variants at the rendering sites
status: open
opened: 2026-09-11
refs: [2378]
---


## Finding

Raised by the full review of WIRE's PR 2378 as a **class about how this
project sweeps**, not about any one crate. Filed in `work/issues/`
because it spans every crate and no program's charter is about sweep
methodology.

PR 2378 fixed four sites where a typed refusal was caught and discarded,
swept `crates/editor-core/src/` for siblings, and produced an honest
triage table with stated blind spots. It still **missed a live instance
in a file it swept** (`clearance.rs:1076`, filed on SHELL) and **shipped
a red CI run** for a second (`pncad-py/src/py/select.rs:819`, one crate
outside its grep scope).

Both misses share one cause: **the instrument was a spelling, not the
class.** The sweep grepped `map_err(|_| ..)`. The class is *a typed
cause is destroyed*, and it is also spelled:

- **`let Ok(x) = f() else { … }`** — `clearance.rs:1076`, the live miss.
- **`.expect(..)` / `.unwrap()`** on a typed `Result` — two at
  `verbs/split.rs:199` and `assembly.rs:1706` (both `#[cfg(test)]`, so
  not D9 panic-path violations, but invisible to the same grep).
- **`impl From<E>` that narrows** — a conversion can drop as much as a
  closure can, and it is further from the call site.
- **`.ok()` and `unwrap_or_*`** over a `Result`.
- **A unit variant at the RENDERING site** — the payload reaches the
  type and the arm that formats it throws it away. This is the one the
  CI red found, and the lane's own written lesson from it was *"a
  refusal's rendering sites are part of its class."*

**The generalisation worth keeping**: sweep for **the construction of
the thing whose cause can be lost** — every `Band::linear` /
`Band::angular_at` / `Band::new` call, say — rather than for the syntax
of one way of losing it. That instrument found the miss in seconds.

## Why this is worth a row rather than a lesson in one PR body

`docs/prompts/implementer-discipline.md` §5 already says a sweep must
state what its pattern could not match, and PR 2378 **did** state its
blind spots — the `let-else` shape was simply not among the ones its
author thought of, which is exactly the failure §5 cannot prevent by
asking for honesty. The cheap improvement is a worked example in §5 of
**choosing the instrument**: key on the construction or the type, not on
the idiom, and name the type's constructors as the sweep's spine.

`docs/prompts/` is not in any open program's `paths`. A taker should
check whether amending a standing discipline document is an `[ev]`
question — it is read by every implementer lane by path, which puts it
close to `memories/`'s rule that what is read at the start of every
session is Ev's call.

## Where else to look

`grep -rn "let Ok(.*) = .*else" crates/*/src` and the same for
`if let Err(_)`, over any type whose error carries a payload worth
naming. The `BandError` family alone reaches 15+ enums across `topo`,
`geom-brep`, `profile` and `mesh`.
