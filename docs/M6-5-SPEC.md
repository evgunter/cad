# M6-5 spec — edge-selection fillet vocabulary (binding)

Mandate (docs/M6-PLAN.md unit 5): `Node::Fillet` grows a
selection of STABLE NAMES; the fillet naming emitter is built;
the composed die becomes a REGISTERED corpus document, closing
the M6-1 review's dev-1 inexpressibility. Substrate: the
measured inventory at `~/.local/share/cad-work/m6-5-substrate/
inventory.md` — read FIRST (the "banked emitter" is ZERO code;
the die document pre-exists in hold-out shape at
editor-core/tests/corpus/die_composed.rs). All five design
forks are RULED (Evan, #217): the rulings below are binding.
Deviations REPORTED (numbered, executed blockers), never
improvised.

## 0. Shape (the #217 rulings)

- **Two sequenced PRs, one implementer.** PR-1 (this spec's
  §§1-4): surgery-door emitter + vocabulary + node + schema +
  eval + die registration. PR-2 (§5): whole-body-door emitter
  totality. Each PR gets its own adversarial review; PR-2
  dispatches only after PR-1 merges.
- **Selection = `Vec<StableName>` ONLY** (no All variant); an
  `all_edges(..)` helper materializes the full set. FREEZE
  semantics documented at the payload: an upstream edit that
  adds edges does not extend a selection — `DocEdit::Rebind` is
  the growth path (a click-selection is a commitment).
  Canonical sorted-set (dedup + stable order) enforced at
  construction and asserted at deserialization.
- **Schema v2 → v3 CLEAN BREAK** (M5 PR 10 precedent): typed
  `SchemaTooOld`, goldens regenerated, no defaults, no
  migration.
- **Re-eval semantics N5-verbatim**: unresolved/tied names are
  typed refusals (NodeGone / Ambiguous+TieWitness / Vanished
  with the honest fallback diagnosis) exactly as
  resolve_declarations; `DocEdit::Rebind` grows the selection
  as its third rewrite site.
- **F-e measure-first**: before touching the document's shape,
  EXECUTE the authored single-call form (12 open chains + 1
  closed rim in one fillet_edges call — supported by contract,
  never exercised). If it works, one node stands; if it
  refuses, grow the surgery door's chain handling (preferred
  over two chained nodes) and report the growth as a numbered
  deviation with the executed refusal.

## 1. The emitter (PR-1: surgery door)

Fillet birth records in the surgery path (`Plan` surgery.rs):
per-entity semantic birth for blend faces, corner patches,
band/rim edges and their vertices — derived at BIRTH, never
matched after the fact (the emit.rs discipline). A new fillet
`RoleSeg` vocabulary in names/role.rs (closed-enum extension;
segments carry the source edge's/corner's own stable name, so
fillet names compose covariantly under upstream bumps — the
boolean emitter's FromA/Seam precedent). The emitter is TOTAL
over surgery-produced bodies (`check_total`); the whole-body
door keeps its empty table in PR-1 with the banked note updated
to name PR-2 (an honest interim dead end, closing next PR).

## 2. Node + schema + eval (PR-1)

- `Node::Fillet { target, radius, selection: Vec<StableName> }`
  per the F-a ruling; `named_nodes()`, content-key hashing
  (`feed_stable_name`), the diff engine, and `DocEdit::Rebind`
  all thread the selection (each has the Declare precedent —
  cite the sites).
- v3 clean break: version gate + `SchemaTooOld` + regenerated
  goldens (no Fillet node exists in committed goldens — verify,
  then the die's golden lands WITH the registration).
- Eval: resolve the selection against the upstream node's
  NameTable (single-operand — simpler than Declare); refusals
  per the F-c ruling; resolved keys feed `fillet_edges` (the
  library door is already selection-shaped).

## 3. The registered die (PR-1)

The hold-out document registers: selection = the pip rim +
box-edge names EXCLUDING the two cavity meridians (the
substrate's ~13 names — derive the exact set and document each
name's provenance); push into `documents()`; S9-flip the
m6_composed_node refusal pin (margin==0.0 → the registered
document builds; history carried); corpus membership rows
(roundtrip, save symmetry, strict maps, interval lane, latency
baseline refresh via the suite's own mechanism). The #217-ruled
freeze semantics get a pin: bump the document (the corpus bump
slides the pip), assert the selection still resolves and the
result is the bumped die — the covariance claim executed.

## 4. Acceptance (PR-1)

1. Surgery-emitter totality on every surgery-produced corpus
   body (check_total green).
2. Name covariance: the boolean-emitter names the selection
   references stay stable under the corpus bump (executed).
3. The die: registered, full corpus rows green, the flipped pin,
   the freeze/covariance pin, interval lane green.
4. v3: SchemaTooOld typed on a v2 file; goldens byte-stable
   across a save/load cycle.
5. Eval refusals: a NodeGone plant, a tie plant (two entries),
   a Vanished plant — each the N5-verbatim type.
6. Regressions: editor-core + topo suites green; hosted CI is
   the gate.

## 5. PR-2 (after PR-1 merges): whole-body totality

Birth records in `Plan::assemble`'s whole-body path; the
empty-table note retires (S9); totality check extends; a
downstream-of-fillet consumer row (a boolean over a filleted
body resolves names through the fillet's table — the dead end
demonstrably closed). Difficulty S; same lane; its own review.

## 6. Constraints

The ratified margin convention binds any new comparand (doors
only; no bespoke thresholds); fail-loud voice; no
Co-Authored-By in lane commits; the M7-LOG/MODEL-AB-LOG are
out of the reviewer's scope. Local battery: editor-core + topo
targeted rows foreground; hosted CI is the gate.
