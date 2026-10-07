# Audit: consuming-model holdovers in ratified and design-page text

Question (Ev, PR #4220): besides A10's "the tip replaces its operands", which
rules only make sense if a node consumes its operands? Swept on `main` at
`b943821b7`, plus the open stage-2 PRs #4216, #4220, #4221 and #4222.
Confidence: **sure**, **likely**, **unsure**. Nothing here proposes a
replacement.

## Hits

**H1. ASSEMBLY A10, the invariants.** "together the root set is exactly
the DAG's sink set and the list adds only the solid order." This
assumes the product is the set of sinks: something read by a node is no
longer product. Under D10 a read operand stays a first-class value. Code:
`roots::check` (`crates/editor-core/src/roots.rs:169`),
`roots::is_sink` (`:328`), and the save/load backstop
(`persist/check.rs:1765`). Open PRs: #4220 drops the invariants, and
#4216 §1 deletes `roots.rs`. Neither carries this. D10 names it by
name ("A10's sink rule"). **Sure.**

**H2. ASSEMBLY A10, the maintenance.** "a node that replaces roots (an
insert consuming them, or split's instance) goes where the first of them
was; removing it puts what it replaced back at its position (a delete's
orphaned inputs in document order, an inline's spliced roots ...)".
This rests on replacement and on orphaned inputs, both of which hold
only under consumption. Code: `roots::on_insert` (`roots.rs:218`, "tip
transfer ... those roots just stopped being sinks"), `roots::on_delete`
(`:270`), `roots::on_set_members` (`:246`), called at
`edit.rs:5592`, `:5712` and `:6735`. Open PRs:
- #4220's rewrite **still carries it**: "the operation's body enters
  where the earliest listed body it reads sits and the other listed
  bodies it reads leave ... a deleted listed body leaves and its orphaned
  operands take its place".
- #4216 §11 FORK-2 frames the question as "add to, replace in, or leave
  alone". Its test 6 ("Migration preserves the product") and unit C's
  "Product membership is unmoved on every corpus file" make the
  replacement behaviour the migration's acceptance.
- #4216 §10 counts 73 `combine_ops` and 38 `creation_ops` viewer rows
  that rely on it, plus `session/op.rs:783` and `session.rs:2776`.
**Sure.**

**H3. ASSEMBLY A4, split acceptance.** "except that the cut's roots come
together where the first of them was (A10's replacement rule: one
instance sits at one place in the list), so split keeps the root order
exactly when the cut's roots are adjacent in it." It rests on the
instance replacing the cut's roots, and Fork-log row 55 (2026-10-03)
ratified this as "A10's replacement rule". Code: `refactor::split`'s
root-list rewrite (`refactor.rs:3175–3183`, `:3344–3355`) and inline's
splice (`:3902–3917`). Open PRs:
- #4220 **carries it**: "the instance takes the first one's place, and
  inline splices the part's product back there".
- #4216 §3 C keeps the split and inline splices over "the cut's listed
  bodies" (`:203`, `:206`).
**Sure.**

**H4. A10 and A12, mates as roots.** A10: "non-body roots contribute
nothing". A12: "`inputs()` stays empty, because a consuming operand
would take the mated bodies out of A10's root set ... so a mate is an
ordinary non-body root: an isolated sink, listed, ignored by the
gather." This rests on two things: a reader removes its operand from the
product, and every sink is a root. A mate's operands are kept as
non-edges only so the mated bodies aren't consumed. Code:
- `Node::inputs` puts `Mate` in the leaf arm (`node.rs:3388`ff.);
- `mate::reading_edges` (`mate/solve.rs:841`);
- `roots::is_sink`'s doc (`roots.rs:317–327`).
Open PRs: #4216 F deletes A12 and `reading_edges`, and C stops listing
non-bodies. That doesn't carry it. #4216 §11 already records the
inconsistency. **Sure.**

**H5. The Measure's references are consuming edges** (ERROR-DESIGN E3 /
E10 together with A10). E3: "one `Measure { expr }` sink node ... failures
poison descendants only (F2 verbatim; sinks have none)" and "DAG
pollution — dozens of measurement sinks. Accepted". This assumes a
measure is a sink that consumes what it reads. Because `refs[].at` are
`inputs()` (`node.rs`, `Node::Measure` arm, ~`:3449`), measuring a
listed body removes it from the product (`roots::on_insert`). This is the
already-filed bug `work/intent/a-measured-part-is-not-a-product-root.md`.
E3's "sinks have none" is also false today, because `Assertion` reads its
`Measure` (`node.rs`, `Node::Assertion` arm). Open PRs: #4216 D makes
`Measure` an operation that defines a scalar, and C fixes the product
(test `the_cut_plate_is_its_product`). **#4216 does not touch E3's
text**, so the "sink node" wording and its rationale survive. **Sure**
on the mechanism; **likely** on the text being a holdover rather than
loose vocabulary.

**H6. REFERENCES DM4, deleting a union member.** "Deleting a pip is
`SetMembers` without it plus a plain `DeleteNode` of the orphaned
transform, one committed action (`commit_action`)". This assumes that a
member dropped from a union becomes "orphaned": a new sink, so A10 would
make it a product root (`roots::on_set_members`, `roots.rs:246`), and it
has to be deleted to keep the pip out of the product. Under D10 the
transform stays a value either way, and whether it shows depends on the
product rule. Open PRs: none touch DM4 (#4222 rewrites DM3, #4221
rewrites DM6). **Likely.**

**H7. REFERENCES §0 / D-2 cut rule (split closure under consumers).**
ASSEMBLY A4: "`refactor::split` cuts a node set closed under the DAG in
both directions". A12 calls `OperandSeveredFromMate` "the reading
edge's twin of D-2's closure rule". The stated reason for the
consumer-ward half is A10's: "closure under consumers is what makes
every cut sink a document sink, i.e. an A10 root"
(`refactor.rs:21–25`). It rests on a cut being a set of sinks that the
instance replaces. With nothing consumed, a remainder reader of a cut
body would read an output of the instance, and the inline twin says the
same in reverse (H8). Code: `SplitError::SeveredEdge`
(`refactor.rs:2667`), `OperandSeveredFromMate`. Open PRs:
- #4216 B keeps the two-way closure over `Doc::upstream` (`:164`), and F
  turns `OperandSeveredFromMate` into "the ordinary severed-read
  refusal". **It carries the rule and drops its stated reason.**
- #4216 FORK-4 names D-2 as one of DM6's reasons that "may or may not
  survive D10". #4221's DM6 rewrite doesn't cite it.
- #4222's DESIGN.md text ("an instance of a part defines one variable
  per entry of the part's product") gives a remainder reader an output
  to read, which is the case the closure assumes doesn't exist.
**Likely.**

**H8. Inline refuses a consumed instance** (code only, no ratified
sentence). `InlineError::InstanceConsumed`: "Splicing would have to
rewire that consumer onto the part's product, which the recipe cannot
express for a placed or multi-root product" (`refactor.rs:1139`, raised
at `:3465` via `roots::consumer`). It assumes that what reads an
instance consumes the part's whole sink-set product. Open PRs: #4216
doesn't list it. Under #4222 an instance defines one output per product
entry, so a reader reads one output. **Likely.**

**H9. Placed under two roots** (A11 (4); NAMING N4's gather rule). A11
(4): "a product refusal (`PlacedUnderTwoRoots`, one instance placed
under two transform roots, among them) is not a mate fault". N4: "a
(name, candidate) pair reaches the product at most once. A strict name
is its own only candidate, so two roots carrying it refuse".
This rests on product members being sinks that reach shared material
only through pass-through ops (N1's `Transform` keeps names verbatim),
so carrying it twice is an error. D10 says "two placements of a part
are two copies". Under a list (or placement) product, listing a body
beside a transform of it, or two transforms of one instance, is
ordinary. Code:
- `product::placed_under_two_roots` (`product.rs:1251`);
- `carry_names` (`:1487`, `ProductError::Naming`);
- `names::verbatim_edge` (`names/role.rs:1785`).
Ratified by Fork-log row 59. Open PRs: #4216 §1 **carries it**:
"`placed_under_two_roots` stays: two listed bodies can still share an
instance below them". **Likely.** It is part consuming-model and part
pre-D10 placement (transform-as-placement).

**H10. A5, minting lifts through consumers.** "carries it up the
operand's consumers to the product's roots ... Where the operand is a
root, or reaches one through `Part` selections and split targets alone,
the lift is the identity", together with the arms `MovedAbove { by }`
and `Vanished { by }` ("`by` is a consumer that merges or cuts the
face"). This assumes the mated operand reaches the product only through
what consumes it. Under D10 the mated instance may itself be listed (or
placed) while a boolean also reads it. Code: `assembly.rs:1446–1545`
(`resolve_face` `:1478`, `names::lift` `:1541`), and `names::lift`
(`names/role.rs:1856`). Open PRs: #4216 F **carries it**: "read the
select's resolved entity carried to the product"; Q8 maps it "into the
product table through the instance's graft map". **Likely.**

**H11. A2, an instance takes the part's A10 product.** "evaluates the
pinned document at the ambient ε, takes its A10 product". It inherits
H1/H2: a part's contribution is its sinks. Code: `src/part.rs`,
`eval/parts.rs`, `product::product`. Open PRs: #4222 DESIGN.md has an
instance define "one variable per entry of the part's product", and so
inherits whatever FORK-2b rules. **Likely** (dependent).

**H12. A9, "consuming ∪ reading edges".** "lie in different connected
components of the DAG under consuming ∪ reading edges". This is the
two-dependency vocabulary D10 retires, and behaviour is unaffected.
Code: `relative_freedom_components` (`mate/solve.rs:877`). Open PRs:
#4216 F reruns it over `Doc::upstream`; its text isn't listed for
rewording. **Likely**, vocabulary only.

**H13. The guide, normative examples.** These are not ratified, but they
are the user contract:
- `docs/guide/assembly.md:322–341`: "A root is a live node nothing else
  consumes — so the two MATES are roots too";
- `:1227`: "A mate is a product root. Roots are the live nodes nothing
  else consumes";
- `:1069`: `outcome.instance in outcome.remainder.roots`.
Each rests on H1/H4. Code: the Python `Doc.roots`/`set_roots`. Open PRs:
#4216 C rewords `:322–341` and `:1227`; `:1069` isn't listed. **Sure.**

**H14. DELETE refuses a node with readers** (DM6 as merged, REFERENCES
§0). "cascade delete (`cascade_delete_order`, `edit.rs`) is the delete
for a node with consumers". This rests on a consumer that can't exist
without its input, whereas D10 says "deleting a variable leaves its
readers unresolved". Code: `EditError::DeleteWouldDangle`
(`edit.rs:1560`, raised at `:6727` via `roots::consumer`) and the load
door's `DanglingInput`. Open PRs: #4221 rewrites DM6 without it, and
#4216 B retires both errors. **Rewritten clean. Sure.**

## Near-misses checked and cleared

- **A11 (3) "Roots"**: the root of a placement group's spanning tree.
  It is about placement, not consumption. The same goes for IDENTITY's
  "roots it (A11 (3))" (`IDENTITY.md:157`) and the guide's `:151`,
  `:244` and `:752`. A11 (2) retires under D10 for other reasons.
- **AQ8 "Only a mate EDGE can cross"**: reading-edge vocabulary that
  retires with A12. Its predicate doesn't rest on consumption.
- **DESIGN.md §B-rep "A product ... is not a boolean operand"**: a
  multi-body refusal, D10-consistent.
- **DESIGN.md `:335`, `:557–561`**: "an op that replaces a cell" and "a
  contact record ... is consumed" are about topology cells inside one
  op, not nodes.
- **NAMING N3** ("names retire into the merge") and the N2/N4 "consumed
  entities have no row" (`names/README.md:304–324`, `:374`): entities
  inside one op's result.
- **REFERENCES DM4–DM5 contact text**: "the fold consumed the pair's
  faces" is about entities.
- **REFERENCES DM1 `:81`** ("a sketch frame is consumed by evaluation"):
  evaluation-time, not a graph rule.
- **REFERENCES DM3**: "body-consuming payload" is vocabulary. #4222
  rewrites DM3 and drops it.
- **REFERENCES DM7**: reports strands at delete and is D10-aligned
  already.
- **VARIABLES VR7**: lifecycle by "read by something"; it already speaks
  D10.
- **DISCIPLINES**:
  - connectedness is per "(root, output) subject" and separation is
    across "root subjects" (`DISCIPLINES-DESIGN.md:308`, `:343`). Two
    sinks could already share ancestry, so neither rests on consumption.
    The rename follows #4216 C (`checks.rs:1166`, `:1267` read
    `doc.roots()`).
  - "finding sink" (`:210`, `:494`) is an unrelated word.
- **GUI-DESIGN `:194`** ("a document's roots"): descriptive. The viewer
  badge is in #4216 C.
- **ERROR-DESIGN other "consumes"** (`:17`, `:193–209`): predicate
  inputs.
- **D10 itself**: "The product is an explicit list of `Body` variables".
  It isn't a consuming holdover, but FORK-2b is re-weighing it against
  the placement model. The retire list names only "A10's sink rule",
  which #4220 read as the invariants but not the maintenance (H2).
- **#4221 DM6 and #4222 DM3/D10 text**: no sink, root, tip, orphan or
  replacement language, except as noted in H7 and H11.
- **`docs/guide/north-star-audit.md:85`, `examples.md`, `GUIDE.md:1050`,
  `profile/README.md` "tip"**: history, unrelated "sink", the profile
  builder's pen tip.

## Method and blind spots

- **Files read:**
  - all of `docs/DESIGN.md` D10 and the companion table;
  - in full around each hit: ASSEMBLY A2–A5, A9–A12 and AQ*;
    REFERENCES §0 and DM3–DM7; names README N1–N4; ERROR E3/E10;
    VARIABLES VR4/VR7; DISCIPLINES rounds 4+; `docs/guide/assembly.md`;
  - the #4216 spec in full; the #4220/#4221/#4222 diffs; #4220's
    thread.
- **Patterns** (case-insensitive, over every companion page,
  `docs/*.md`, `docs/guide/*.md` and `crates/*/README.md`):
  - `sink|consum|replac|\broots?\b|\btips?\b|orphan|supersed|absorb|swallow|stops? being|subsum`;
  - a second pass for `root(s)` near product/document/list,
    `tip transfer`, `ancestor-free`, `InstanceConsumed`,
    `DeleteWouldDangle` and `cascade`.
- **Code:** callers of `roots::*` and `Node::inputs`, read in place.
- **Blind spots:**
  - A rule that encodes consumption without these words wouldn't
    surface. One example is a disjointness assumption phrased as "the
    product's solids are disjoint" (the `graft_disjoint` obligation is
    checked, not assumed).
  - Python docs and `.pyi` stubs, and the viewer README (an
    implementation record), weren't read beyond grep.
  - `work/` items and `docs/doc-ledger/` weren't swept: the ASM-ROOTS
    D-2/D-3 origin is only in the archive.
  - Open PRs other than the four, e.g. #4247 (GUI variables), weren't
    read.
