---
id: kernel-wasm-row-denies-no-warnings
kind: issue
title: the kernel/editor-core wasm32 row denies no warnings, and is green under a deny today
status: open
opened: 2026-09-11
---


Found by the sweep on
`wasm-row-warning-debt-comment-names-a-closed-item-and-a-deleted-symbol`
(PR 2326), which flipped the *other* wasm row to a deny. The pattern was
"every `cargo check`/`cargo clippy` invocation in `.github/workflows/`
and `local-scripts/ci-local.sh`, partitioned by whether it denies
warnings". After that flip exactly one hit is left.

## The row

- `.github/workflows/ci.yml:2142` —
  `cargo check --workspace --exclude pncad --exclude pncad-py --exclude
  viewer --features interval --target wasm32-unknown-unknown`, the
  `wasm32 check (kernel + editor-core, --features interval)` step in the
  `fmt` job.
- `local-scripts/ci-local.sh:1139` (`wasm_check`), its mirror.

It denies nothing. Its long comment block (`:2085-2142`) argues the
siting, the billed cost and `check`-not-`build`; it says **nothing**
about warnings, so unlike the viewer row this one never carried a debt
sentence and nothing records a decision not to deny. It is the last row
in either half that cannot fail on a lint.

## Measured

`RUSTFLAGS='-Dwarnings' cargo check --workspace --exclude pncad
--exclude pncad-py --exclude viewer --features interval --target
wasm32-unknown-unknown` at `aa628e5`, empty `CARGO_TARGET_DIR`, 4 vCPU:
**exit 0 in 23 s**, zero diagnostics. So the flip is free and green
today — there is no debt behind the absence, only an absence.

## Why it is not a one-line edit either

Two things to settle, and they are why this is a row rather than a fix
folded into the PR that found it:

- **Which spelling.** The sibling row took `cargo clippy … -- -D
  warnings` over `RUSTFLAGS='-Dwarnings …' cargo check` because the two
  deny identically across workspace path dependencies (measured there)
  and clippy is strictly wider for the same cost. That argument has to
  be re-taken here at `--workspace` scope: this row covers far more
  crates, and a clippy pass over all of them at a second target is not
  the same cost measurement as one `-p viewer` graph.
- **Ev's 2026-08-21 ruling is in the neighbourhood.** That ruling cut
  this guard to one leg (the interval one) on a coverage argument about
  the purely-additive lint, and `ci-local.sh:1131` records that the row
  "inherits" a lint residual from it. A deny changes what that residual
  costs. Read the ruling before spelling this.

## What it buys

`ci.yml`'s viewer wasm row now leans on this one in its seed-key
argument — the skipped case is "a break in a crate `viewer` depends on,
outside `{viewer, pncad, bvh}`, that this row already compiles". That
argument covers BREAKS and not LINTS, because this row denies nothing;
the viewer row's comment says so at the site. Denying here closes that
gap rather than documenting it.
