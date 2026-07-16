---
name: cad-project-state
description: Greenfield Rust CAD kernel — DESIGN.md is the authoritative contract; M0 is next
metadata: 
  node_type: memory
  type: project
  originSessionId: 11974b46-1641-48d9-9802-fdf44dcb6927
---

Greenfield B-rep CAD kernel in Rust (branch mngr/cad). **docs/DESIGN.md is
the authoritative, ratified design contract** — read it before any design
or implementation work; do not re-litigate settled decisions D1–D9
(arenas+Euler ops; intensional edge geometry with prefer-intrinsic rule;
closed surface enum; single per-run tolerance, fail-loud; provenance from
birth; canonical units; import-is-adoption; recipe-as-data; determinism
charter). Q1 residue (Real trait surface, Dual<Interval> semantics, k·ε
indeterminacy threshold, Body<T> genericity boundary) is deliberately
deferred to the first M0 PRs as design-in-code discussions.

As of 2026-07-15: no code yet; M0 (geom-core: Real trait, intervals,
tolerance, predicates, arenas, validation harness) is next. docs/M0-PLAN.md
has the PR sequence. License: dual MIT OR Apache-2.0. Project name still
undecided — placeholder acceptable. `references/` (git-ignored) holds The
NURBS Book (full scan), Mäntylä ch. 4–6 (Euler-operator chapters MISSING —
fuller copy still sought before M1), and Hoffmann complete (recovered via
Wayback). Scanned PDFs need poppler (installed) and are read visually,
page by page. See [[cad-working-style]].
