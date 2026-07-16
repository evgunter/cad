---
name: cad-project-state
description: Greenfield Rust CAD kernel — DESIGN.md is the authoritative contract; M0 complete (2026-07-16), M1 (topology + Euler ops) next
metadata:
  node_type: memory
  type: project
  originSessionId: 11974b46-1641-48d9-9802-fdf44dcb6927
---

Greenfield B-rep CAD kernel in Rust (repo evgunter/cad). **docs/DESIGN.md
is the authoritative, ratified design contract** — read it before any
design or implementation work; do not re-litigate settled decisions
D1–D9 (arenas+Euler ops; intensional edge geometry with prefer-intrinsic
rule; closed surface enum; single per-run tolerance, fail-loud;
provenance from birth; canonical units; import-is-adoption;
recipe-as-data; determinism charter).

**M0 complete (2026-07-16)** — all seven PRs merged with Evan sign-offs
on the design PRs: `geom-core` (comparison-free `Real` at f64 /
feature-gated inari `Interval` / in-house `Dual<T>`; once-per-run
`Tolerance` with a single ε — εₐ was ELIMINATED, angular thresholds
derive as ε/lever-arm; trilean `Decide` predicates, sliver-band
semantics with provisional K = 10; hand-rolled linalg) and `topo`
(`Body<T>` = scalar-free topology arenas + T-valued geometry arenas;
validation harness). All Q1 residue ratified; only K's numeric value
stays open (multi-ε experiments). Key operational facts: the `interval`
cargo feature quarantines LGPL (gmp-mpfr-sys) per issue #4 — default
builds stay MIT/Apache + C-free; x86-64 floored at `x86-64-v3` via
.cargo/config.toml; CI's `discipline` job greps `Real +` bounds (L7).
`docs/M0-LOG.md` holds the L-decision log, per-PR outcomes, and the
carried-into-M1 list (validator items, Body<Interval> test, M2
watchlist).

**M1 next**: topology + Euler operators; build a cube by hand;
watertightness + Euler–Poincaré checks. The **full Mäntylä book is now
on hand** (supplied by Evan 2026-07-16, TOC-verified: ch. 9–11 Euler
ops + half-edge + implementation; ch. 12–15 feed M2/M3) —
`references/mantyla-1988-an-introduction-to-solid-modeling-full.pdf`.
Also in references/: Hoffmann (complete), The NURBS Book (full), GSD06
DDG course notes (for M6 kink/subgradient design). Scanned PDFs read
visually (poppler installed). License dual MIT OR Apache-2.0; name
still pending (Q9). See [[cad-working-style]],
[[orchestration-model]].
