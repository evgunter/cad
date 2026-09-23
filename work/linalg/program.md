---
id: linalg
kind: program
title: LINALG — geom-core's vectors, frames and interval conventions: the answers that are wrong at Interval
status: ready
opened: 2026-09-20
area: kernel
prefix: linalg/
tag: (LINALG orchestrator)
ab_band: 9400-9499
paths: [crates/geom-core/src/linalg/*, crates/geom-core/src/interval.rs]
keep_out: [opened by PROPS's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - PROPS measured 108.5 budget points and was cut into tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are PROPS NURBS LINALG VERDICT and they share their parent's territory by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR rather than drawing a fence, ENCL and STACK hold the certified-enclosure half, QUAD owns props/quad.rs, SSI and PCERT own the geom-brep certification lanes, PORT took the four refusal rows whose class is the crate boundary]
priority: P0
---
The bottom of the kernel, where an answer that is merely loose at `f64`
is WRONG at `Interval`. `Vec3::orthonormal_basis` returns a sign-hulled
frame when `n.z` encloses zero; `chord_join`'s sphere-pole branch pick
hands `shift_branch` a two-integer shift at `Interval` when an entry
azimuth lands on the previous exit; `sector_shape` mints
`Indeterminate`s through a local `invalid()` helper AFTER a definite
sign, discarding the sign it just proved.

And one row says D9's determinism claim is narrower than it reads: a
`Mat3`/`Affine3` product's NaN sign and payload differ between debug
and release because LLVM commutes `fadd` across inline sites, so the
fixed-order guarantee covers non-NaN outputs only.

FRAME holds the who-answers-give-me-a-frame question; this program
holds what those answers are made of.

Charter and order: `work/linalg/plan.md`; narrative in `work/linalg/log.md`.
