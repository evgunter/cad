---
id: frame-linear-generic-door-has-no-consumers
kind: issue
title: A CLASS - public generic doors with zero production call sites, kept alive by their own tests: Frame::linear<T> (PR 2375) and both profile map_scalar rungs (PR 2409)
status: open
opened: 2026-09-11
refs: [2375, 2409, 2475]
---


## Finding

From the full review of PR 2375 (S4, confidence `sure`). Accurate at
that PR's head.

`crates/editor-core/src/placement.rs:230-232`'s `pub fn linear<T: Real>`
has **no consumers anywhere**: PR 2375 moved `determinant` onto
`linear_f64` and `affine` onto `affine_f64`, and repo-wide the only
`.linear::<` call sites left are the three inside that file's own test
module. The reviewer proved it mechanically — dropping `pub` yields
`warning: method 'linear' is never used` from
`cargo check --workspace --all-targets`.

The reviewer's own sentence is the finding: *"a public generic door kept
alive by its own test is exactly what the item was retiring at the other
end."*

## Why it was not done in PR 2375's fix pass

Deliberate, orchestrator's call. It is a **public API removal**, the
review classed it as a style finding (non-gating), and a fix pass exists
to repair what its review found wrong — not to widen the PR into a
surface decision nobody asked for. It gets its own row so the decision
is made on its merits.

## What the decision actually is

Three answers and none is obviously right:

1. **Delete it.** Nothing uses it, the workspace proves it, and this
   project is pre-release with no external consumers to break. Fail-loud
   says an unused public door is a claim the library does not keep.
2. **Keep it as API.** `Frame::affine<T>` is the kernel's placement
   door; `linear<T>` is its other half, and a library that offers the
   affine map at the backend scalar but not the linear part is oddly
   shaped from outside. If that is the argument, it belongs at the site,
   because right now nothing says it.
3. **Demote it to `fn`** beside `linear_f64` and `affine_f64`, keeping
   it for whatever reads it next. Cheapest, and the one that leaves the
   question open rather than answering it.

Whoever takes it should check first whether `Frame` is re-exported on a
public path at all and whether the Python surface reaches it — the
review measured the *workspace*, which is the right instrument for (1)
but not for a claim about external users.


## Second instance, and that makes it a class (2026-09-12, PR 2409's full review)

`crates/profile/src/lib.rs:453` (`Profile::map_scalar`) and `:767`
(`ProfileLoop::map_scalar`). Proven the same way and mechanically:
making both private and running `cargo check -p profile` emits **two**
`method 'map_scalar' is never used` warnings. `Profile::map_scalar`'s
only callers are `crates/profile/tests/scalar_lift_door.rs`;
`ProfileLoop::map_scalar`'s only non-test caller is `Profile::map_scalar`.

**And PR 2409 removed the last production caller in the act of minting
the rung above it.** Before it, the loop rung had one (`sweep`'s
`end_profile`); after, neither rung has any. `D385`'s scheduled
conversion of the two test copies produces only **test** consumers, so
the schedule does not retire the shape.

### Adjudicated: the two instances are NOT the same defect

Kept in one file because the *shape* is one and a taker should see both,
but the dispositions differ and the difference is the useful part:

- **`Frame::linear<T>` LOST its consumers.** PR 2375 moved
  `determinant` and `affine` onto the private f64 doors and left a
  public generic door nothing calls. Nothing owes it an existence; the
  three answers on this row (delete / keep-as-API-with-the-argument-at-
  the-site / demote to `fn`) stand unchanged.
- **`profile`'s rungs were minted to a written convention.**
  `crates/geom/src/scalar_lift.rs:12-23` says *"one name, `map_scalar`
  on every geometry type and `map` on every leaf"* — so in a **library**
  crate the door is owed to an external caller whether or not an in-tree
  site wants it. That is a real argument and PR 2409 did not make it; it
  called the absence "deliberate" and left it there.

**What the class actually asks**, which neither instance answers alone:
does this project want `scalar_lift.rs`'s convention to mint public
doors ahead of consumers, and if so, is the `Frame::linear<T>` case a
violation of the same convention (a geometry type whose lift door should
therefore STAY) rather than dead weight? Answering that disposes of both
rows; answering either alone leaves the other unprincipled.

**Where else to look**: every type named in `crates/geom/src/scalar_lift.rs`'s
convention, checked for a `map`/`map_scalar` with no non-test caller.
The instrument is the one used twice here — drop `pub`, compile, read
the dead-code warnings — and it is exact, not a grep.
