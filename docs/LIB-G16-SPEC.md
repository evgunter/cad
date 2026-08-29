# LIB-G16 — Node::Chamfer, the fillet's twin (recipe door for chamfer_edges)

**Status: spec under ratified `docs/RECIPE-DOORS-DESIGN.md` D2+D3
(Evan, in-chat, 2026-08-29) and issue #918. Binding at dispatch.
Full model-A/B protocol unit — a schema break is not mechanical.**

## Deliverables

1. **`Node::Chamfer { target, distance: Expr, selection:
   Vec<StableName> }`** — `Node::Fillet`'s twin at every site the
   compiler enumerates (node.rs's seven exhaustive matches, the two
   eval dispatch sites with the next free content-key tag, the
   payload-hash match, `NodeErrorKind`), plus the canonicalizing
   construction door and the `SlotId` for distance (Length). Eval arm
   mirrors `wire_fillet`: N5 selection resolution → `chamfer_edges`
   → refuse-if-no-naming → emit → `stamp_minted`.
2. **`names::emit_chamfer` on `emit_topo`'s `TieRows` deferral shape
   from birth, AND `emit_fillet` re-shaped onto the same deferral in
   this unit** (D2 — the #708 debt paid to zero sites, not doubled;
   #708's own text wants the fix landing with the first tie-capable
   emitter). Role vocabulary UNCHANGED per D3: the chamfer emitter
   reuses the fillet `RoleSeg` variants (the minting node
   discriminates); a `// #917` marker at `OpGroup::Fillet` notes the
   group-name rename as not-this-unit.
3. **Schema bump per the dispatch-time-seam discipline**: read
   main's `SCHEMA_VERSION` by eye at branch time (v15 at spec) and at
   EVERY re-merge; claim the next free number; one ledger prose entry
   in persist/mod.rs's version doc-comment; `deny_unknown_fields`
   means old files refuse typed with the standing regenerate
   recourse — regenerate whatever committed fixtures/corpora the bump
   invalidates using their own documented recipes.
4. **`Node.chamfer` in pncad-py** — `Node.fillet`'s twin, same frozen
   text selection (the audit's own words); stubs, ty fixtures, tags.
5. **Census + audit re-cuts, honest**: G16's three stops (rows 2
   `spacer`, 11 `diechamferblank`, 12 `diechamfer`) flip only as far
   as each row's own claim, measured against the tour scenes as
   oracles (`demos/tour/src/bodies.rs`, the die scenes); the
   `spacer` scene note's friction and `diechamfer`'s finding 2 (names
   vs arena keys at the kernel verb) are what the door discharges —
   say in the row what the recipe path now answers and what the
   kernel-direct path still costs. Tallies re-derived; census
   `gap: G16` entries dispositioned.
6. Python tests with closed-form oracles per the scenes' own
   statements; the emitter contract pinned (a planted upstream tie
   must flow through the deferral without `DuplicateName` — build the
   probe even if today's tree mints no first tie, as a unit test of
   the deferral path itself).

## Fences

No #917 rename (marker only). No `Node::Tube` / `Node::Shell` (their
own units). No changes to `chamfer_edges` or any kernel geometry. No
new `RoleSeg` variants (D3). No whole-body `all_edges`-style chamfer
materializer beyond what fillet already has (corpus-measured, later).
`emit_fillet`'s re-shape must be behavior-preserving for every
existing name (the fillet suites are the oracle — byte-identical
name tables on the tour corpus, stated as a claim in the PR).

## Protocol

Full A/B: implementer arm = block LIB-12 slot 3, read back from the
redacted record via git history; the standing LIB-12 contamination
flag rides the dual. Pre-draw fields at this spec: **M-L /
STRUCTURAL**. v6 dual at review (next LIB ordinal claimed at
dispatch); blinding fences as G18a's (no trailers in lane commits, no
model talk, lane-private paths).
