---
id: num-relative-tolerance-collides-above-a-decimetre
kind: issue
title: num()'s relative tolerance exceeds eps above ~0.1 m, so two lengths the kernel can tell apart render as one number
status: review
opened: 2026-09-11
branch: fix/num-tolerance-cap
---


Filed by the DOOR orchestrator, from reading `path::num` while scoping
`work/door/patherror-display-renders-float-noise`. FIX's file, FIX's
program, and FIX has two rows closed on this helper today
(`path-error-numbers-below-1e-9-render-as-zero`, PR #2366;
`verb-and-dimension-render-through-debug`, PR #2347), so the finding
goes here rather than being carried on another slate.

**This is the OPPOSITE end of the range from the row FIX just closed,
and the fix for that one is not implicated.** #2366 removed a
`.max(1.0)` that pinned the tolerance ABSOLUTE at 1e-9 for `|x| <= 1`,
which rendered every sub-nanometre margin as `0` — exactly the margins
these messages exist to report. The purely relative form that replaced
it is right at the small end and must stay.

## The defect

`crates/profile/src/path.rs`, `fn num`:

```rust
let tol = 1e-9 * x.abs();
```

D4's ε is ~1e-9 m and it is a **length, not a ratio**
(`geom_core::tolerance::DEFAULT_EPS`). A relative 1e-9 therefore
crosses ε at `|x| = 1` m and is coarser above it — at a kilometre the
tolerance is 1e-6 m, a thousand ε. Two lengths the kernel can certify
as different then render as the same number, which is how a refusal
comes to read *"margin 1234.5 m exceeds the 1234.5 m the anchor
pins"*.

Executed over the function's exact body, differences stated in
multiples of ε:

| magnitude | difference | renders as |
| --- | --- | --- |
| 1 m | 10 ε | `1` vs `1.00000001` — distinct |
| 100 m | 10 ε | `100` vs `100` — **collide** |
| 1234.5 m | 100 ε | `1234.5` vs `1234.5` — **collide** |
| 1234.5 m | 1000 ε | `1234.5` vs `1234.5` — **collide** |
| 10 km | 1000 ε | `10000` vs `10000` — **collide** |

The crossover is `|x| = 0.1` m: above it the relative tolerance exceeds
a 1e-10 absolute grid, below it the relative arm is finer.

## The shape of the fix

Cap the tolerance rather than floor it — the finer of the two grids,
which leaves the sub-ε regime #2366 repaired completely untouched:

```rust
let tol = (DEFAULT_EPS * 0.1).min(x.abs() * 1e-9);
```

One decade below the ratified ε at the coarse end; relative, as today,
below a decimetre.

**Use the compile-time `DEFAULT_EPS`, not `Tolerance::eps()`.** ε is a
live process value and a code-tier run gates {default, 1e-6, 1e-12};
reading it here would make every rendered refusal a function of process
configuration and every string assertion in the tree eps-sensitive
across three rows. The grid is a display choice stated once against the
ratified default.

## The assertion it owes

A row that goes red when the tolerance drifts back above ε — the
current text has none, and `num_renders_a_sub_nanometre_payload_at_its_own_magnitude`
pins only the small end:

```rust
assert_ne!(num(&100.0_f64), num(&100.000_000_01_f64));
```

## Blind spot of the sweep that found it

Only `crates/profile/src/path.rs` was read. `num` is private to that
file, so the 38 call sites FIX's closed row counts are all inside it —
but **the same relative-vs-absolute confusion is a class**, and the
sweep for it has not been run: any other `Display` impl, assertion
helper or report formatter that writes a relative tolerance where ε is
a length. `crates/profile/src/validate.rs` and the other crates' error
`Display`s are the obvious first look.
