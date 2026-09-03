---
id: demo-typed-refusal-exit-convention
kind: ruling
title: Should a demo surface a typed refusal as a clean nonzero exit, or is every refusal a demo bug?
status: closed
opened: 2026-08-20
closed: 2026-09-03
github: 795
refs: [787]
---

## Ruled (Ev, in chat, 2026-09-03)

**The same rule applies as to test code: the demo is allowed to panic if
the kernel does something unexpected, but it is also fine for it to
narrate a preexisting, already-known frontier.**

Refinement accepted: *already-known* means the frontier is **declared up
front at the scene**, not discovered by catching a refusal at runtime. A
declared frontier narrates and the run still exits **0**, so the second
exit convention (b) would have cost us — the one `eps_regression.rs`,
`render.sh`, `render.yml` and `check_render_provenance.py` would all have
had to learn — is not needed. An undeclared refusal still panics: that
half of today's behaviour is unchanged.

**Forward obligation:** if we end up with something with a declared
frontier, make sure it fails loudly when it stops refusing — a declared
frontier whose refusal has gone away is a silent skip, and silent skips
are the class the tour has already been bitten by.

No mechanism is being built now: no scene currently reaches a frontier,
and `step_expected` — cited in (a) as `true` at every construction site —
no longer exists anywhere in the repo (zero hits, checked at close). The
first scene that needs a declaration builds the mechanism and can size it
against a real case.

## From GitHub issue 795

Opened 2026-08-20; 0 comments.

Raised by smell-scan Track G lane G-b closing **S110(j)** in #787, and by that PR's style review, which ruled correctly that the replacement text deferred the question without scheduling it. Filing it so the deferral has a register instead of living in a test-file doc comment.

## The state of the tree

`demos/tour/tests/eps_regression.rs` used to open with:

> *"the only legal outcomes are a working run or a typed refusal (which the tour surfaces as a clean nonzero exit, not an abort)"*

while `run_tour` asserts only `output.status.success()`, so a clean nonzero exit fails exactly as a panic does. Two facts settled it:

1. **The tour has no clean-refusal exit.** Its only `std::process::exit(2)` sites (`demos/tour/src/main.rs:522,543`) are inside `#[cfg(not(feature = …))]` usage-error arms for the `k-probe` and `tess-budget` modes. Every typed refusal on the scene path is a panic.
2. **It never had one.** The reviewer confirmed independently that the arm #787 deleted — the STEP subset-frontier arm — *dropped the body and still exited 0*. So the sentence was not describing an earlier design; it was never true at any commit.

#787 therefore rewrote the header to state the contract the file has. But it did so with a deferral and no schedule:

> *"If a scene ever legitimately reaches a frontier, the tour has to grow a way to SAY so at that scene — and this file has to grow the arm that accepts it — before any exit code can carry the news."*

That sentence quietly settles a real question in a test-file comment, which is what this issue exists to undo.

## The question

**When a demo's scene legitimately reaches a shipped frontier of the kernel, what should the demo do?**

- **(a) Every refusal is a demo bug — panic, always.** The position the tour holds today, and it is coherent: the crate doc says the scenes exist to demonstrate real usage of what the library *can* do, and `step_expected` was `true` at every construction site for exactly this reason. A scene that reaches a frontier is a scene that should not have been written that way, or a frontier that should have moved.
- **(b) A demo may narrate a frontier and exit nonzero cleanly.** The position the deleted sentence assumed. It buys a demo that keeps running as the kernel's subset grows and shrinks, and makes the demo a *reporting* instrument rather than a gating one. It costs a second exit convention that `eps_regression.rs`, `render.sh`, `render.yml` and `check_render_provenance.py` would all have to learn.
- **(c) Neither — the scene says so, not the process.** A `Stop` gains a way to declare "this body is at a named frontier", the manifest carries it, the tour narrates it and still exits 0, and the pin asserts the declared set is exactly the expected one. One exit convention, and the claim moves to where the reader is.

I have no stake in the answer and did not resolve it. My weak preference is **(c)**, because it is the only one where "which frontier" is a checkable value rather than an exit code, and because `memories/demo-purpose.md` puts the demo's job at "show what using the library is like" — which includes showing where it stops.

**This may want Ev**, since it is partly a question about what the demos are *for*, which is his (`memories/demo-purpose.md`, 2026-08-09). Flagged rather than assumed.

## Where it is cited

`demos/tour/tests/eps_regression.rs`'s header points here, so the deferral has a destination.

— Claude (smell-scan Track G, lane G-b)

## Home

Code quality: raised by the scan's Track G lane closing `S110(j)` in #787, and `demos/` is Track G/X ground in this program's partition; S-MATE's `keep_out` explicitly pushes "795-adjacent demo questions" away, and VERBS records only that its demos will follow whatever this ratifies. It is a ruling, so it cannot live under `work/issues/`.
