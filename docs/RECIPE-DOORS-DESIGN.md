# Recipe doors for the shipped surgery verbs — chamfer, tube, shell

**Status: DESIGN CONVERSATION — awaits Evan's sign-off. Nothing here
is ratified; the register items are G16 (#918), the audit's tube row
(24, widened by VERBS-TUBEWALL), and G17.** Mechanics below are
measured (file:line), not assumed; the design content is the five
decisions D1–D5.

## The problem, priced

Three kernel verbs ship with no recipe node, so each is
kernel-direct only — no rebuild, no `StableName`, invisible to
Python: `sweep::chamfer_edges` (G16), `tube_along_arc` /
`tube_along_arc_hollow` (audit row 24), `topo::shell` / `shell_open`
(G17). `Node` carries fifteen variants
(`crates/editor-core/src/node.rs:471`); a new kind touches seven
exhaustive matches in node.rs, two eval dispatch sites (next
content-key tag: 24), `NodeErrorKind`, and the viewer/refactor/edit
sweep the v13 `Node::Mate` precedent enumerates. Schema is v15; no
migration machinery exists by LQ7a ruling — every bump is a clean
break with typed regenerate recourse.

## D1 — three units, three bumps, fixed order: chamfer, tube, shell

One shared bump would need the trio argued as one vocabulary change
(the v12 precedent). Against it: the units are wildly unequal (see
D2–D4), shell needs kernel-side work no LIB unit can fence, and a
train held open across three implementations is exactly the
dispatch-time-seam hazard the v7/v8 double-claim taught. Bumps are
CHEAP pre-release. So: each unit claims its version at dispatch per
the standing discipline (claim past open holders; re-read main's
constant by eye at every re-merge; prose tripwire in MODEL-AB-LOG).
Chamfer first (fully specified by #918), tube second (clean 1:1),
shell last (cross-program dependency).

## D2 — Node::Chamfer is Node::Fillet's twin, and the emitter pays the #708 debt

Payload `{ target, distance: Expr, selection: Vec<StableName> }` —
`chamfer_edges` is signature-identical to `fillet_edges` modulo the
size name, and `Chamfered<T>` IS `Filleted<T>` (same birth records,
`sweep/src/fillet/build.rs:256`). Eval arm mirrors `wire_fillet`
(N5 selection resolution, refuse-on-missing-naming). The emitter is
where the care is (#918): `emit_chamfer` is written against
`emit_topo`'s `TieRows` deferral shape from birth — and **the same
unit re-shapes `emit_fillet` onto the deferral**, because #708's own
text says the fix lands with the first tie-capable emitter, and
writing a correct twin beside a defective original mints a
documented-defect pair where one fix was available. Python binding:
`Node.chamfer`, the audit's own words — "`Node.fillet`'s twin, same
frozen text selection".

## D3 — chamfer naming REUSES the fillet role vocabulary

No new `RoleSeg` variants. The kernel reuses the fillet's birth rows
deliberately (the roles ARE the same shapes: a band face off a source
edge, a corner patch off a source vertex); a `StableName` already
carries the minting node, which distinguishes a chamfer's blend from
a fillet's at every selector. Growing the role vocabulary is itself
schema-visible for no added discrimination. The blend-family segs
stay grouped under `OpGroup::Fillet` for now — the group's NAME is
#917's rename (the shared vocabulary "still speaks as the fillet"),
and that ~255-reference rename is explicitly not this unit's; a
`// #917` note at the group is the honest marker.

## D4 — Node::Tube is ONE node kind carrying `wall: Option<Expr>`

Both tube doors are thin wrappers over one private
`build(..., wall: Option<T>, ...)` ("`wall` present ⇔ hollow",
`sweep/src/revolve/tube.rs:328`), so the node maps 1:1: `{ datum
anchoring for center/axis/u_ref per Revolve's payload precedent,
major_radius: Expr, window, minor_radius: Expr, wall: Option<Expr> }`
— the audit row's "one node kind, not two, and the same bump" holds
by construction. Wall-vs-minor validation stays kernel-side (the
door's own typed arms, nothing pre-checked). Naming: the doors return
`Revolved<T>`, so the revolve emitter path is the template; the unit
measures whether it applies wholesale or needs a tube-specific
translation, and reports rather than forcing it. This unit does NOT
touch `wire_sweep`'s banked frontier or the G2 path-composition tail
(U4/LQ3 — kernel-owned, unchanged).

## D5 — shell WAITS on a kernel birth channel, and LIB files the ask now

`shell`/`shell_open` return a bare `Body<T>` — no birth record
exists (measured: no naming machinery anywhere in `topo/src/shell.rs`),
where fillet/split/boolean each have one. A recipe node without an
emitter mints no names, which reproduces G16's exact defect one verb
over. So the shell unit has a hard prerequisite LIB cannot fence:
a `ShellNaming` birth record (rims minted per opened face, wall
faces off source faces, the FilletNaming shape) written by the
kernel door. LIB's proposal: file the kernel ask as an issue at this
doc's ratification, offer the record shape, and hold the Node::Shell
unit until it lands (kernel program's concurrence per the standing
rule). The node payload, decided now so the ask is concrete:
`{ target, thickness: Expr, open: Vec<StableName> }` — open faces by
frozen stable-name selection resolved through the N5 ladder to the
`FaceKey`s the door takes (the teapot's by-description scan is the
friction this replaces; its scene note is the evidence).

## What this doc does not decide

The #917 rename (its own issue, its own scale); un-banking
`wire_sweep` (G2, kernel); whether `all_edges`-style whole-body
materializers grow chamfer conveniences (corpus-measured, later);
Python bindings beyond the named twins (each rides its unit,
LIB-PYBUNDLE shape).

**Sign-off affordance: 👍 the PR comment.** On ratification: G16's
unit dispatches first (full A/B protocol — a schema break is not
mechanical), the shell kernel ask files, and this doc's decisions
fold into DESIGN.md's companion table at the first unit's merge.
