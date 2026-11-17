# Design fork: a cone face through its apex with a split side (TESSLANE), 2026-11-17

Protocol: ddd201f411f (`git log -1 --format=%h origin/main -- docs/DESIGN-FORK-PROTOCOL.md`, after a fetch).

Dispatch draws, two /dev/urandom bytes:

- arm byte: 10 (below 86 → three designers, otherwise two) → **three designers**
- label byte: 110 (not 255; 110 mod 3 = 2) → **Fable = C; the two Opus designers are A and B**

Problem statement handed to every designer: one text, held as `tesslane-cone-apex-split-side-problem.md` beside this file. Each designer works in its own git worktree of `tesslane/lattice-clearance` at `ba68f8ed0a` (PR 7043's head, where the faces refuse) with its own build directory, and is told to read no pull request's thread and no branch under `analysis/`.

## First reports

Recorded as delivered, before any reconciliation. Dispatched 2026-11-17 ≈00:50Z.

- **B** (Opus): `tesslane-cone-apex-split-side-report-B.md`. Delivered ≈01:00Z; 159,025 tokens, 44 tool calls, 853 s. Recommends one home for every cone face, the development (the chart lane's plane), with the rectangle lane's strip argument restated as a rule of the cone's chart; rejects routing every cone face to the chart lane as it stands (measured ×14 to ×160 the triangles on bare cones); until then PR 7043's `UnresolvedWinding` refusal stands, with an optional temporary bridge (route only the split-side cone faces).
- **A** (Opus): `tesslane-cone-apex-split-side-report-A.md`. Delivered ≈01:06Z; 165,119 tokens, 28 tool calls, 1,037 s. Recommends meshing a ruled face (cone, cylinder) as strips between neighbouring rulings, in the chart lane, with no interior lattice and a fan-triangle bound at δ; every cone face routes there. Until then, route a cone face holding its apex with a split side to the chart lane now (not main's behaviour; a typed refusal naming the split ruling as fallback). Also measured: a cylinder face with a split straight side refuses `CertificateExceeded` at every δ in the rectangle lane; whole-cone density 26× in the chart lane as it stands.
