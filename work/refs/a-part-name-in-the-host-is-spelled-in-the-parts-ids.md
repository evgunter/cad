---
id: a-part-name-in-the-host-is-spelled-in-the-parts-ids
kind: issue
title: A part's name enters the host verbatim, so its node id is the part's, read by host doors as the host's
status: open
priority: P2
opened: 2026-10-23
cost: M
---


D′3 retires `RoleSeg::InPart`: an instance's copy carries its part's
product names verbatim (spec §5′.3, "a copy's names are its
source's"). Every such name's `node` is a node of the PART document,
minted by the part's own counter, so in the host it is either no live
node or, by coincidence of ordinals, an unrelated live one. Host doors
that read `name.node` as a host id now meet part ids:

- `edit.rs` `mint_selection` (the `foreign` skip near `:1579`): a
  selection whose body reaches an instance skips the
  `DeclareNamesMissingNode` liveness check, but where the part's id
  collides with a live host node it runs `check_name_steps` against
  the host's node instead — a wrong check either way.
- `InterfaceCrossing::Mate.outer` (`node.rs` `payload_names`, the
  insert door's liveness check, `Rebind`, DM7 strands, split's
  `PartNameReachesRemainder`): an `outer` naming a face of another
  instance is now a part-id name. A split whose crossing reads an
  instance's face records an `outer` the insert door refuses (or
  accepts against a colliding host node). The tests
  (`edit_instance_crossing_names.rs`, `asm_r2b_assembly.rs`
  `remainder_with_a_neighbour`, `edit_one_predicate.rs`
  `saved_crossing`) now spell their `outer`s on host nodes.
- `Rebind`, `SetAppearance`, `Doc::name_carriers` and the stranded
  reports key on `name.node`; a part name's node is not the instance,
  so deleting the instance strands nothing by name, and a host node
  whose id collides with the part's is reported as its carrier.
- `refactor::inline` rewrites a spliced part's names through a remap
  keyed on part ids; it uses lenient maps (host ids plus the splice's
  `node_map`) because a part name and a host name are both `RecipeNodeId`.

Two readings, for the designers: a part name read in the host is
always a SITED read (its site the instance, as `SitedRef` already
carries for mates), and the host doors check the site, never
`name.node`; or the copy re-keys its part's names into the host (a
`From { read }`-style segment, which the spec retired as `InPart`).
The spec does not settle it.

**A coverage cost, measured.** `refactor::split` skips a crossing
whose outer side is a part's name read in a kept instance's copy
(`crates/editor-core/src/refactor.rs`, the interface record's
`outer.name.node != outer.at` arm). Every block in
`crates/editor-core/tests/rev_fix_xsplit_unreachable.rs` is an
instance, so both its exhaustive sweeps now mint no crossing at all
(`shapes`: 9 accepted cuts, 6 straddling, 0 crossings; `foreign`: 6, 2,
0). The rows still check that split and the member walk agree, with
the skip mirrored, but AQ8's positive side (a welding constraint
that crosses is recorded) has no exhaustive row until this is
settled. The foreign-master adversary is also gone as written:
`in_part(c, …)` and `in_part(a, …)` spell one block part's ids, so
the head resolves to `a`'s member.

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to REFS's charter (names scoped by their read). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).

## The viewer reads part levels by site (VGEOM, `vgeom/step-focus`, PR 7766, 2026-11-25)

The collision this row describes needs no coincidence beyond equal
first edits: a mint chain starts empty in every document
(`Mint::empty`), so a part and a host that open alike mint the same ids
throughout their common prefix (the fixture of
`crates/viewer/tests/focus_highlight.rs`
`a_parts_wall_is_no_wall_of_the_hosts_step_whatever_its_ids` asserts
it). `Doc::part_spelled` and `resolve::part_name` ask the mint log,
which such an id passes.

`marks::step_focus` (`crates/viewer/src/marks.rs`) therefore reads each
carried level of a drawn name at its site: `swept_by` descends the
carries (`names::origin`), and `spelled_at` follows a level's site
through the pass-through edges (`names::verbatim_edge`) to the node that
spelled it, or to an instance first, which makes the level the part's.
That is a third answer to "is this level a part's", beside the two
mint-log ones; the readings above (a sited read, or a re-keying copy)
would give the three one home.

A further option the readings do not list, for Ev and not decided here:
seed `Mint::empty` with the `DocumentId`, so two documents' ids cannot
collide at all. Viability is unknown: ids are persisted (D9) and every
saved document's ids were drawn from the unseeded chain.
