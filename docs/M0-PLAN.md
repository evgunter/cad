# M0 Work Order — **COMPLETE (2026-07-16)**

All seven PRs merged (#2, #3, #5, #6, #7, #8, #10 on GitHub); Q1 residue
ratified into DESIGN.md; CI green at ε ∈ {1e-6, 1e-9, 1e-12} plus the
`interval` feature lane. Running state and per-PR outcomes:
`docs/M0-LOG.md`. Carried into M1: K's numeric value (multi-ε
experiments), the `Body<Interval>` instantiation test, the validator's
M1 items (arity rules, orphan-vertex vs `mvfs`, D5 provenance check),
and the half-edge design (wants the missing Mäntylä chapters).

---

Read `DESIGN.md` first — it is the ratified contract (D1–D9). This file is
the implementation sequence for M0 and the working agreements for how to
run it. M0's goal: `geom-core` — the scalar/tolerance/predicate substrate
everything else stands on — plus the workspace and validation scaffolding.

## Working agreements

- **Small PRs, each one a design conversation.** The Q1 residue (Real
  trait surface, `Dual<Interval>` semantics, k·ε indeterminacy threshold,
  `Body<T>` genericity boundary) was *deliberately* deferred to be settled
  in code review — present the design in the PR, discuss, then ratify the
  outcome back into `DESIGN.md`. Do not batch-implement M0 silently.
- **D9 charter applies from the first line**: no panics on input, typed
  errors, `libm` for transcendentals, deterministic iteration everywhere,
  essentially no unsafe.
- **Property tests from day one** (`proptest`), and a CI job that runs the
  suite at several ε values (D4 ¶1) to flush out tolerance-sensitive code.
- Placeholder workspace name until Q9's name lands; dual MIT OR
  Apache-2.0 license files from the start.

## PR sequence

1. **Workspace scaffolding.** Cargo workspace (`geom-core` first, empty
   siblings can wait); LICENSE-MIT + LICENSE-APACHE; CI: fmt, clippy
   (deny warnings), test, multi-ε test job placeholder; `rust-toolchain`.
2. **`Real` trait + `f64` + `Tolerance`.** Our own minimal trait
   (arithmetic, sqrt/trig via `libm`, constants); `Tolerance { eps,
   eps_angular }` initialized once per run (D4 ¶1), single definition
   site. *Design discussion: the exact trait surface.*
3. **Trilean predicates.** `Sign`/margin types; predicates return
   `Result<bool, Indeterminate>`; f64 semantics of indeterminate (margin
   within k·ε — pick k here). *Design discussion: predicate API shape,
   how margins are surfaced.*
4. **`Interval` wrapper over `inari`** implementing `Real`; comparison
   ops route through the trilean machinery (indeterminate ⇒
   `Indeterminate`, never a guess). Note: inari's `gmp` feature needs a C
   build step and Haswell+ on x86-64.
5. **Duals.** `Dual<f64>` via `num-dual`; in-house `DualNum` wrapper over
   `inari::Interval` (does not exist off the shelf — comparison/signum
   semantics are the design discussion, consistent with PR 3/4).
6. **Fixed-dim linear algebra.** 2-D/3-D points/vectors/transforms,
   hand-rolled, generic over `Real` (DESIGN.md layering table — we
   control the scalar trait, no nalgebra in the kernel core).
7. **Arenas + `Body<T>` skeleton + validation harness.** `slotmap` typed
   keys per entity kind; scalar-free topology / `T`-valued geometry
   arenas split (Q1); the validator scaffold that M1's invariant
   checklist (Euler–Poincaré, watertightness, residual ≤ ε) plugs into.

Exit criteria: all seven merged, Q1 residue ratified into `DESIGN.md` as
code-backed decisions, CI green at multiple ε values. Then M1 (topology +
Euler operators — wants the missing Mäntylä chapters or Hoffmann's
boundary-rep chapter from `references/hoffmann/`).
