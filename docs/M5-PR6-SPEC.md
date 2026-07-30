# M5 PR 6 — pcurves as per-half-edge certified caches (binding spec)

Executes M5-PLAN PR 6 (C4; deps [4, 5] both merged). Branch
`ev/m5-pr6-pcurves` from main (post-#141). OQ4 stands resolved
carrier-primary: the 3-D carrier cache remains authoritative
machinery; pcurves are peer *caches*, never peers of the
description.

## 1. Storage — the half-edge IS the key

- A pcurve cache lives at the (edge, face-side) incidence = the
  half-edge. Seam edges are the forcing case and the acceptance
  demonstration: both half-edges of a seam edge lie on the SAME
  surface with DIFFERENT pcurves (the u = 0 vs u = 2π sides) —
  per-edge-per-face under-keys; per half-edge has no special
  case.
- **Planar faces keep M2's derive-on-demand status** — nothing
  stored (no speculative caches); pinned by a row asserting
  planar bodies carry zero stored pcurves.
- Caches are minted where curved faces are minted (today: the C5
  table's Plane+Cylinder splitting lane — the shape (i) bodies),
  certified at mint, immutable with the body. Content-keyed cache
  transfer stays banked; no invalidation machinery.

## 2. Parameter and orientation contract

The pcurve's parameter IS the edge's carrier parameter
(he_plus-forward, increasing start→end); the traversal sense per
face is derived, never stored (D1 verbatim). No re-
parameterization, no per-face parameter flips in storage.

## 3. Certification — in meters, through the map

- The certified statement is |S(P(t)) − C(t)| ≤ ε: 3-D
  displacement between the surface-composed pcurve and the
  carrier cache, on the shared certification schedule,
  hull-bounded per C2.2 (the C9/PR 2 machinery; PR 4 fit
  certificates for fitted forms). ε is the D4 ¶2 certification
  tolerance.
- **No UV-space tolerance appears in any certified statement or
  user-facing text.** UV steps inside algorithms are
  implementation dials; the map's local stretch is the lever arm
  and it varies — a certification that quotes chart units is
  dimensionally dishonest and is a review defect.
- Domain validity is part of the certificate: P(t) stays in the
  face's trim region for t in the edge span; periodic charts
  unwrap along ONE continuous branch pinned at the start point,
  continuity certified (the M2 PR 5 meridian-unwrap finding
  generalized — nearest-previous per-sample unwrapping is a bug).
- Certification is scalar-generic (C6 pinning rule): structure
  f64-selected, certificates replayable at 3ε + interval,
  bit-identical (D9; chart evaluations live on `Real`, libm-only).

## 4. Sources (consume PR 5; build nothing speculative)

- Conic carriers: PR 5's constructors become the stored caches —
  plane chart exact (rational-quadratic chain; residual
  identically zero in ℝ, certified enclosure in f64), cylinder
  chart fitted (2-D NURBS sinusoid graph via the PR 4 loop,
  fixed schedule, deterministic bits).
- M2 closed-form carriers on curved charts where minted bodies
  need them (a rim circle on its cylinder chart is the line
  v = const — closed form; keep exact where exact).
- Fitted 2-D NURBS is the general form (A9.10 shape, C6 rule) —
  but only carriers minted by the current table get caches; no
  cache for pairs that cannot yet be minted.

## 5. Consumers wired here (minimal, honest)

- The tier-gate ladder gains a pcurve certification check for
  bodies carrying stored pcurves (certificate present + replay
  passes + trim containment) — refusal typed.
- A corrupted-cache row: a deliberately perturbed stored pcurve
  FAILS certification (the hull bound catches a between-samples
  excursion — OQ2's argument, transposed).
- Hot-path consumers (tessellation trim loops, SSI-on-trimmed-
  faces, census extension) are PR 7/11/12 — do not wire them;
  their arrival is named in the cache's rustdoc.

## 6. Acceptance

- Seam-edge fixture: one seam edge, two half-edges, same surface,
  two DIFFERENT certified pcurves — the under-keying
  counterexample as a committed row.
- Shape (i) corpus bodies (cut_cylinder) gain stored pcurves on
  their elliptical edges: certified at mint, D6.1 round-trip
  preserves them (or re-derives — state which, honestly, per the
  persistence posture), bit-identical replay rows at 3ε +
  interval.
- One-branch unwrap row on a periodic chart crossing the seam;
  the wrong (per-sample nearest) unwrap must be demonstrably
  refused or unrepresentable.
- Planar-zero-caches pin; corrupted-cache refusal row; trim-
  containment refusal row (a pcurve escaping its trim region
  fails typed).
- Messages compose the shared recourse carrier where the
  situation is a coincidence one; frontier refusals name arriving
  PRs.
- Local checks per the narrowed process: touched crates both
  lanes, fmt, clippy touched; hosted CI gates the matrix.

## 7. Out of scope

SSI's native ℝ⁴ pcurve production (PR 7); tessellation/census
consumers (PR 11+); pcurve-primary architecture (OQ4 resolved
carrier-primary); invalidation/transfer machinery (banked); any
new Real methods.

## 8. Process

Standard: foreground rows, one per Bash call, `pgrep -x cargo`
polling only; push per unit; adversarial e2e review + fix pass;
PR by orchestrator. OUTPUT DISCIPLINE per standing header.
