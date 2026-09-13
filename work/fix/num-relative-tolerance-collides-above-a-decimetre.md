---
id: num-relative-tolerance-collides-above-a-decimetre
kind: issue
title: num()'s relative tolerance exceeds eps above 1 m (the id says decimetre; that is where the CHOSEN eps/10 grid crosses, not where collisions start), so two lengths the kernel can tell apart render as one number
status: closed
opened: 2026-09-11
branch: fix/num-tolerance-cap
pr: 2399
closed: 2026-09-12
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

## Closed by PR 2399 — and the title was off by a decade (FIX orchestrator, 2026-09-12)

`let tol = (DEFAULT_EPS * 0.1).min(x.abs() * 1e-9);` — `min`, never
`max`. Below the crossover the relative arm is strictly smaller, so
`min` returns the identical expression and the sub-ε regime PR 2366
repaired is bit-identical **by construction**, not by sampling.

**The title said "above ~0.1 m" and collisions start above 1 m.** The
lane caught it; the body and the table were right all along and only
the title conflated two different crossings. ε is 1e-9 and the old
tolerance was `1e-9·|x|`, so the tolerance exceeds ε at `|x| = 1`,
which is where two lengths a kernel can certify apart begin to render
alike. `0.1` m is where the relative arm crosses the **chosen** ε/10
grid — where renderings change, not where the defect starts. The id
keeps the old spelling, because ids are stable.

Verified independently of the lane before merging: the row's collision
table re-derives exactly over `num`'s real body — all four pairs
collide under the old tolerance, all four distinct under the new, and
the 1 m control is unchanged under both.

**Two further things the lane established that the row did not know.**

The decade in `ε/10` is load-bearing, not decoration: a cap at exactly
ε still collides at 1234.5 m on two lengths one ε apart.

And the `DEFAULT_EPS`-not-`Tolerance::eps()` argument has a receipt
rather than only a rationale — under a live ε the already-pinned
`1.0/3.0` row spells three different ways across the three gated eps
rows, so reading the live value would red an existing string pin on
two of the six lane/eps points.

**Instruction 3 fired, and the pin really was part of the defect**:
nothing discriminated. The sub-nanometre rows are all ≤ 1e-8 and the
ladder asserted `≤ 1e-9·|x|`, which a *tighter* tolerance satisfies
vacuously. Both ends are now mutation-proven in both directions —
reverting the line reds five rows, and swapping `min` for `max` reds
seven, including the rows PR 2366 landed. **The one-character slip
that would re-mint that defect can no longer land silently**, which is
the reviewer brief's standing warning about a fix minting a fresh
instance of what it closes, answered mechanically rather than in
prose.

One consequence the lane disclosed rather than smoothed: above
~3.3e5 m an absolute 1e-10 grid is finer than `f64`'s own spacing, so
payloads there render at full `Debug` precision. It declined a third
arm that would give up once the grid beats the spacing, because that
reintroduces a floor-shaped term and a second knee. Verified: near
`f64::MAX` the spacing is ~2e292 m against a 1e-10 grid, so the exact
spelling is the only one that round-trips and `num` and `{:?}`
necessarily agree — the flipped assertion there is forced, not chosen.

## Residue, both filed

- `fillet-leg-carrier-renders-raw-float-noise` (this slate) — the
  blind spot this row predicted, and the OPPOSITE defect: no
  shortening at all, reaching `path.rs` sentences through
  `CornerReason`'s `{carrier}`.
- `work/props/quadrature-budget-prints-its-two-lengths-alike` —
  reported out of fence by the lane and placed on PROPS's slate by the
  orchestrator, verified first.
