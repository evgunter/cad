# VERBS-RING — retire `FullRevolveHoles` through the shared void-insertion door

Unit of `docs/VERBS-PLAN.md` Wave 1 (row 5, as redefined at #907's
ratification). Branch `verbs/ring`, PR to main. Difficulty logged
pre-draw: **M**. Ratified basis: `docs/OFFSET-DESIGN.md` O4 (as
merged) and DESIGN.md's revised M2 bullet — **every cavity is born
through the shared void-insertion door**; the holed full revolve is
its third producer, DEFINED as
`revolve(outer) − revolve(hole-as-outer)` and executed as the
degenerate no-crossing arm.

## Scope

1. **Factor the void-insertion door.** The boolean already owns
   cavity insertion (the path that lands a reversed interior shell
   in the result with its census/containment evidence). Extract that
   step into a door callable WITHOUT the SSI/crossing pipeline —
   a move-and-expose, not a rewrite: existing boolean callers must
   be bit-identically unaffected (the cheap proof: existing boolean
   suites unchanged and green; state it in the PR body). The door's
   contract: given a valid solid and a certified-strictly-contained
   closed shell, insert the reversed shell as a cavity; refuse typed
   if the containment evidence is absent — the door never derives
   containment itself, callers supply it (that keeps #750's
   box-coarseness out of this unit entirely).
2. **Retire `FullRevolveHoles`.** A full revolve of a holed profile
   revolves the outer boundary (exists today) and each hole as its
   own solid of revolution, then inserts each hole's reversed
   boundary through the door. **Containment is certified
   by construction, in 2D**: profile validity already places each
   hole strictly inside the outer loop with a decided margin, and
   revolving about the shared axis carries strict 2-D containment to
   strict 3-D containment verbatim — derive the evidence from the
   profile's own validated margins; do not re-derive it with 3-D
   box tests. Per-hole seam surgery per the revolve's existing seam
   conventions (the revolve docs call it mechanical; you are the
   first to exercise it — report what the docs missed).
3. **Prose/probes**: the revolve error text and its pointer at the
   explicit composition retire with the refusal; `klein::wall_probes`
   wall 6 flips from pinning the refusal to building the ring (a
   probe re-baseline — say what moved and why); the KERNEL-VERBS
   defect row and demos/README wall count update. Sweep for the
   refusal's name in prose, not just the symbol (the RIM lesson —
   docs/ was the blind spot).

## Fences

- **No shell verb, no offset machinery** (Wave 3 consumes the door
  you factor; do not build ahead of it).
- **No SSI-path changes** — the factoring must leave every crossing
  boolean bit-identical.
- Partial revolves and unholed full revolves bit-identical (the
  output-stability rule: identity chooses among implementations,
  it is not the justification — the justification is the ratified
  definition).
- The multi-shell CURVED body STEP-export refusal
  (`CurvedShellClassification`) is a KNOWN standing gate
  (OFFSET-DESIGN O6's demo-gates list): the ring demo records it as
  a finding, never works around it, and this unit does not fix it.

## Acceptance

- **The one-call hollow ring**: a holed profile fully revolved →
  a two-shell solid (outer + toroidal cavity), tier-3 valid;
  census/shell-count pinned; mass properties = outer minus hole
  closed forms (derive independently); multi-hole profile → one
  cavity per hole.
- The degenerate-arm claim pinned: the construction runs NO SSI
  (assert structurally — e.g. the door path taken, not the
  crossing pipeline; make it RED-able if someone reroutes it).
- Containment-evidence honesty: a door call with absent/failed
  evidence refuses typed (planted-corruption row).
- Existing boolean suites + revolve suites green and bit-identical;
  klein wall 6 re-baselined; the tour builds.
- Per the sampling-CI convention: note the drawn point in the PR
  body.

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer in lane commits (blinding). Lane-private PR draft
(`~/.local/share/cad-work/verbs-ring-*.md`). Merge origin/main
immediately before opening the PR; confirm CI runs STARTED; watch to
completion. Push after every coherent unit. Do not merge.
