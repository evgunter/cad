---
id: main-does-not-compile-sweep-after-the-from-f64-sweep-merge
kind: issue
title: main does not compile sweep after PR 4591's merge: lift_loop_walls builds NurbsSurface<f64> where it returns NurbsSurface<T>
status: open
opened: 2026-10-11
priority: P0
cost: E
---


Found by the FLUX lane on `flux/rim-span-decide` while confirming a
CI red in a main-only worktree. At `ad20b8c42` (main after merge
`fbee5ef1a`, PR 4591 `intent/from-f64-sweep`, and `d34a66e09`),
`cargo build -p sweep` fails:

- E0308 at `crates/sweep/src/loft.rs:392`, in `lift_loop_walls<T: Real>`:
  `NurbsSurface::new(..)` yields a `NurbsSurface<f64>` where the function
  returns `T`;
- E0277 at `crates/sweep/src/loft.rs:398`: `.collect()` into
  `Vec<Arc<NurbsSurface<T>>>` from `Arc<NurbsSurface<f64>>`.

CI on main agrees: run 38131307231 (`d34a66e09`) is `failure`, and 4591's
own merge run 38131243270 was `cancelled`. It looks like a semantic merge
between 4591's scalar-lifted knots and weights (`ceb38df95` merged main
into the branch) and a `loft.rs` caller that landed meanwhile. Not
bisected further.
