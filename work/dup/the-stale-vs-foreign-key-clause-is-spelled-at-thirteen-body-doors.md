---
id: the-stale-vs-foreign-key-clause-is-spelled-at-thirteen-body-doors
kind: issue
title: The stale-vs-foreign key clause is spelled at thirteen Body doors, two of them near-identically
status: closed
opened: 2026-09-20
refs: [the-guarded-shell-list-of-a-solid-is-spelled-thirteen-times]
priority: P4
cost: D
closed: 2026-09-26
branch: dup/b6-c
---

## Finding

- **Where**: `crates/topo/src/body.rs`, thirteen door rustdocs.
- **Importance**: low
- **Confidence**: sure. Counted at `cd9fdfd6b`, one file.
- **Raised by**: the `Body::shells_of_solid` fold, 2026-09-20 — as an
  **X4 the fold caught in its own diff**.

`body.rs`'s module docs carry a `# Key validity: stale vs. foreign`
section (~:36). Thirteen door rustdocs below it restate the section's
consequence rather than pointing at it, in two shapes:

- **Seven parentheticals**, near-identical: *"or `None` if the key is
  stale (a foreign key is not caught — see the module docs)"* —
  `get_solid`, `get_shell`, `get_face`, `get_loop`, `get_half_edge`,
  `get_edge`, `get_vertex`.
- **Four long forms** that add a door-specific consequence:
  `solid_of_face` and `face_of_half_edge` (composing two lookups, so a
  foreign key does not stop at the first hop), `faces_of_solid` and
  `shells_of_solid` (a foreign `SolidKey` on a live slot hands back
  another solid's entities).

The seven parentheticals earn their keep: one clause, at the door, is
cheaper for a reader than a jump. The **four long forms are the class**,
and two of them were byte-parallel.

## A SECOND family in the same prose, which this row's first pass had no bucket for

Three door paragraphs in `body.rs` open with the byte-identical
sentence **"`None` is the only refusal this door can make"** —
`solid_of_face` (~:876), `face_of_half_edge` (~:999) and
`shells_of_solid` — and each then restates the consequence in its own
words: *"so three shapes of caller keep a hand-written walk"*,
*"so a caller whose own refusal distinguishes the hops keeps its own
walk"*, and (before the fix) *"that is why it can stand under callers
that refuse in different vocabularies"*. One argument, three
paragraphs.

**The third copy was minted by the unit that filed this row**, in the
same diff, and this row as first written sorted `body.rs` door prose
into the two shapes above and could not see it: the census was of the
stale-vs-foreign clause, and the fence was drawn at that clause rather
than at *arguments the module already makes that a door restates*.
`shells_of_solid` now states its refusal by reference; the other two
are untouched. **A census whose bucket is a SENTENCE cannot find the
next sentence**, which is the same shape as the code census this
program keeps re-learning — the fence is drawn at what the author was
already looking at.

## The X4, and how it was caught

`Body::shells_of_solid`'s first draft wrote
*"a foreign `SolidKey` landing on a live slot passes the resolution and
this door hands back **another solid's shell list** as though it were
the caller's"* — which is `Body::faces_of_solid`'s sentence with
"face list" swapped for "shell list", ten doc lines above it in the
same file, in the diff of a unit whose subject is one thing spelled *n*
times. It was caught by re-reading the diff for the class being closed,
before the branch was pushed, and the door now points at
`faces_of_solid` for the `SolidKey` hazard instead of restating it.

**The pair `solid_of_face` / `face_of_half_edge` is the other half and
is untouched**: both spell out the composes-two-lookups consequence in
full. Left as found — this unit fixed the instance it minted, not the
class, and a prose fold across four doors is its own decision about
where the argument's home is (the module section, or the first door).

## Why this is filed on dup

Its subject is one argument spelled four times, which is this
program's charter; `orient-module-prose-accumulation` on the same
slate is the same class in another module. `crates/topo/src/body.rs`
carries no `territory` owner.


## Closed (2026-09-26, PR: batch 6)

**Re-measured at the merge base `032999ff2`**, `git grep -n -i foreign
-- crates/topo/src/body.rs`, then `git grep -n -i -E 'foreign
(key|handle|`)|not caught|stale-vs-foreign|stale.vs.foreign|stale/foreign'`
over every tracked file with no path argument. The row's title said
thirteen and its body counted eleven; **the class was sixteen**:

- **Eleven short parentheticals**, not seven — *"or `None` if the key is
  stale (a foreign key is not caught — see the module docs)"* at
  `get_solid`, `get_shell`, `get_face`, `get_loop`, `get_half_edge`,
  `get_edge`, `get_vertex`, and four the row did not list: `get_point`,
  `get_curve_geom`, `get_surface` and `provenance`.
- **Four long forms**, as the row said: `solid_of_face` and
  `face_of_half_edge` (two lookups composed), `faces_of_solid` and
  `shells_of_solid` (a solid's member list; `shells_of_solid` already
  pointed at `faces_of_solid`).
- **One outside `Body`**: `readback::ReadbackError::Dangling`, *"foreign
  keys are not caught; see the `Body` docs"*.
- Plus two restatements that are not doors: the `// Lookup.` banner
  comment above the doors, and the `Body` struct docs' lineage section
  (a pointer, kept, as below).

**The fold.** The module docs' `# Key validity: stale vs. foreign`
section is the one home. It gains one paragraph that sorts the doors by
what a foreign key costs, as a CLASS of three shapes, each stated
there once:

- a door that reads the key's own slot (an arena, or a side table
  parallel to one) hands back one wrong entity's row;
- a door that chains lookups carries the key onward, to a well-formed
  answer about the wrong entity;
- a door that answers a collection (a member list, or a cycle or orbit
  walked from the key) hands back another entity's whole collection.

Every door keeps its own `None` contract and carries ONE link phrase
to the section by anchor (`self#key-validity-stale-vs-foreign`, and
`crate::body#…` from `readback` and `geometry`). A slot-reading door's
phrase is *"[a foreign key is not caught]"*, so a reader hovering in an
IDE still sees the hazard. A chaining or collection door's phrase names
its own shape. The argument itself stays only in the section.

**Widened to the whole class in the fix pass.** The first cut linked
only the 16 doors that restated the clause. That left the same-shape
doors unlinked, and a banner claiming every door linked. Now linked as
well:
- the chaining doors `mate` and `half_edge_end`;
- the collection doors `loop_cycle` and `vertex_orbit`;
- the side-table reads `edge_/vertex_/face_provenance_of`, whose
  *"`None` iff the key is stale"* was false for a foreign key on a live
  slot and now says "if";
- `surface_/curve_/point_source`, `surface_/curve_/point_origin`,
  `surface_field_source`, `pcurve` and `null_face_pair`;
- the two hidden test doors, `flipped_face_sense_for_tests` and
  `with_entity_removed_for_tests`, whose "iff" was false the same way.

Rendered under the gate's flags, `struct.Body.html` carries 32 links to
the anchor. The two hidden test doors do not render.
**Not linked: the `pub` writers** (`set_*_source`,
`set_surface_field_source`, `attach_pcurve`, `detach_pcurve`). None of
them answers a read, and the section makes no claim about them.

**Divergent members, kept.** The four long forms differ from the short
form in substance, not only in wording. A lookup whose foreign key lands
on a live slot hands back one wrong entity. A door answering *through*
the key hands back a well-formed answer about the wrong owner, or
another solid's whole member list. So the difference is kept: its
consequence moved into the home's taxonomy, and each of the four doors
says which shape it is, in its own words, with the link. The `Body` struct's `# Lineage-scoped keys` section is kept as
it was, with its pointer retargeted to the anchor: it argues the other
face of the coin (lineage-scoped keys are what the interval replay
relies on), which the module section does not state.

**The second family** (*"`None` is the only refusal this door can
make"*, three doors). `solid_of_face` stays the home of its own
face→shell→solid populations, and `shells_of_solid` already pointed
there. `face_of_half_edge` keeps its OWN statement, in its own words:
a caller whose refusal names which key went stale (the half-edge's, or
its loop's) keeps its walk. `solid_of_face`'s populations are
face→shell→solid, not half-edge→loop→face, so borrowing them would
have been wrong.

**Also folded where the unit stood**: `readback.rs`'s `Display` comment
restated `DanglingRef`'s two lanes and now points to that type's docs.
`topo/src/geometry.rs`'s module docs now link the anchor.
`geom-brep/src/keys.rs` cannot link upward, so its prose now names the
`topo::body` module docs rather than `topo::Body`.
`get_curve_geom`'s `since M3 PR 1` archaeology went with its
parenthetical.

**Sweeps and their blind spots.**
- Pass 1, the clause's own words (above). It cannot match a restatement
  that uses none of "foreign", "not caught" or "stale/foreign".
- Pass 2, aimed at that gap over `crates/topo/src`:
  `lineage|arbitrary|another body|unrelated|another solid|whatever
  (face|half-edge|entity)`. The only new hits were `Body`'s lineage
  section (kept, as above) and `a_stale_key_has_no_origin`'s rustdoc,
  which is a test citing the module docs and not a door. What neither
  pass can see is a restatement worded with none of those phrases.
- Pointers retargeted rather than rewritten: `crates/topo/src/geometry.rs`
  and `crates/geom-brep/src/keys.rs` (above).
- Not this class: the repo-wide pass also hit the `Dangling` two-lanes
  argument in `lib`'s crates. It is filed there as
  `work/lib/dangling-two-lanes-argument-is-restated-outside-danglingref.md`.

**Instrument for the fold**: `cargo doc -p topo --all-features
--document-private-items --no-deps` under the gate's
`RUSTDOCFLAGS="-D warnings -A rustdoc::private_intra_doc_links"`,
clean. rustdoc does not check an anchor fragment, so the rendered HTML
was read: `body/index.html` carries `id="key-validity-stale-vs-foreign"`,
`struct.Body.html` has 16 links to it, and `ReadbackError`'s page has 1.
After the fix pass: 32 on `struct.Body.html`, 1 on `ReadbackError`, and
1 on `geometry/index.html`.

