---
id: the-guarded-shell-list-of-a-solid-is-spelled-thirteen-times
kind: issue
title: Body::shells_of_solid is the missing door: the guarded shell list of a solid is spelled twenty-one times in topo/src, not thirteen
status: closed
opened: 2026-09-20
closed: 2026-09-20
refs: [listing-a-solids-faces-is-spelled-four-times-in-topo-src, two-spellings-of-the-face-to-solid-owner-index, the-guarded-shell-list-of-a-solid-is-spelled-eleven-times-outside-topo-src, seqgen-fusion-remake-shell-is-dark, the-boolean-joins-shell-processing-order-is-unasserted]
---



## Finding

- **Where**: thirteen sites in `crates/topo/src`, listed below.
- **Importance**: medium
- **Confidence**: sure. All thirteen were read.
- **Raised by**: the style review of PR #2898 (the `faces_of` fold),
  2026-09-20.

`Body::solid_of_face` exists; `Body::faces_of_solid` was minted by PR
#2898. **The shell-side door between them does not exist**, and its
shape is written out thirteen times: resolve a solid, refuse if it does
not, take its shell list.

### (a) The guarded shell LIST — 8 sites

`get_solid(solid).ok_or(<error>)?.shells.clone()` (or `.iter()`), each
with its own refusal:

| site | refusal |
| --- | --- |
| `boolean/combine.rs` (~:475) | `ok_or_else(corrupt)` |
| `boolean/finish.rs` (~:225) | `JoinDesync { what: "operand solid no longer resolves" }` |
| `boolean/ops.rs` (~:2062) | `corrupt("re-cut solid lost")` |
| `movefac.rs` (~:259) | `EulerOpError::StaleKey { EntityId::Solid }` |
| `shell.rs` (~:1014) | `ShellError::Corrupt { EntityId::Solid }` |
| `splitting/finish.rs` (~:338) | `SplitFinishError::Corrupt` |
| `splitting/finish.rs` (~:726) | `ok_or_else(corrupt)` |
| `offset_together.rs` (~:824) | `?` on `Option` |

**Two of them are the same four-line sequence modulo the error
constructor and the accumulator's name** — `splitting/finish.rs`
(~:338–346) and `boolean/finish.rs` (~:225–233): take the shell list,
then `for shell in shells { all.extend(body.movefac(shell)?) }`.

### (b) The solid's ONE shell — 5 sites

`let [shell] = solid_data.shells[..] else { … }`, `.shells[0]`,
`.shells.last()`:

| site | on failure |
| --- | --- |
| `euler_kill.rs` (~:422) | `EulerOpError::SolidNotSingleShell` |
| `seqgen.rs` (~:1043) | `false` |
| `seqgen.rs` (~:1223) | a `.then_some` guard on `shells.last()` |
| `seqgen.rs` (~:1361) | `[0]`, unguarded |
| `review_m1_pr4.rs` (~:1611) | `false` |

The split matters for the fix: (a) is one door with a posture
question — the eight refusals are eight different types, which is the
same argument that kept `offset_together::scope_of_moves` out of the
`solid_of_face` fold — and (b) may be a second door
(`Body::the_only_shell_of`) or may be (a) plus a caller-side match.

## How it was missed, which is the part worth keeping

PR #2898's third instrument was **every `Solid::shells`-shaped field
read** (139 at `b29fe8bd1`), classified by whether a `.faces` read
followed within 8 lines (41). Against these thirteen sites, run at the
same base:

- **9 are in the 98 the `.faces` filter discarded** — they take the
  shell list and stop, so the filter that defined "list a solid's
  faces" removed them by construction.
- **4 were inside the 41** — `offset_together.rs` (~:824),
  `seqgen.rs` (~:1043, ~:1361), `review_m1_pr4.rs` (~:1611) — and were
  dispositioned only against THAT row's class ("does this materialise
  the solid's face list? no"). Correct for that row, and a bucket drop
  for this one.

**This row published 8/5 first, and it was wrong twice over.**
`euler_kill.rs` reads `solid_data.shells[..]` at ~:425 and
`shell_data.faces[..]` at ~:434 — **nine** lines, outside the 8-line
window — so it is in the 98, not the 41. And the list of five given
for "inside the 41" was **not that partition at all**: it was group
(b) above with one member swapped. Two different splits of thirteen
were presented as one, and **the coincidence that both are 8/5 is
exactly what made it read as verified**. The groups are (a) 8 / (b) 5
by SHAPE; the instrument partition is 9 out / 4 in. They are not the
same cut and neither implies the other.

**Two bucket drops, 139 → 41 → 12 named.** Method item 9 says a bucket
disposition is where a census loses things; here it lost a whole
sibling class rather than a member. The instrument was not blind — the
question it was asked was narrower than what it had in hand, and that
conclusion holds under either partition.

And `Body::faces_of_solid`'s own rustdoc now points a caller straight
at the undoored shape — *"a caller that needs the shells kept apart
walks them"* — without saying that walking them is written thirteen
ways.

## Why this is filed on dup, and off its territory owners' slates

The thirteen sites sit on **boolean/curved** (`combine.rs`,
`finish.rs`, `ops.rs`), **shell** (`shell.rs`, `offset_together.rs`),
**splitting** and unclaimed ground (`movefac.rs`, `euler_kill.rs`,
`seqgen.rs`, `review_m1_pr4.rs`). No one owner holds a majority, the
subject is the duplication class rather than any one file's behaviour,
and the decision it asks for — mint `Body::shells_of_solid`, and
whether (b) is a second door — is a `topo::Body` door question. A lane
opening any one of those files will not see this row; it is
cross-referenced from
`work/dup/listing-a-solids-faces-is-spelled-four-times-in-topo-src.md`.
An owner that would rather hold it should move the file, per
`work/README.md`'s one-file-one-item rule.

## Not measured here

No census was taken outside `crates/topo/src`. The same shape in
`crates/sweep/src`, in `tests/` trees or in the cargo roots outside
`--workspace` is unmeasured, and thirteen is therefore a floor for the
crate and says nothing about the tree.

## Re-taken 2026-09-20 at `cd9fdfd6b`; the class is TWENTY-ONE in `topo/src`, thirty-two tree-wide

**Denominator-first** (method item 7), because the row's own two
instruments had already disagreed with each other once. The terminal
read every member must contain is a `Solid`'s `shells` field, so the
enumeration is **every `get_solid(` occurrence in every tracked file,
no path argument — 41** — classified by whether `.shells` appears
within eight lines of it.

| | count |
| --- | --- |
| `get_solid(` occurrences, all tracked files, no path argument | **41** |
| of those, reaching `.shells` within 8 lines | **34** |
| of the 34: `Body::get_solid`'s own definition (a window over-fire onto the arena read two lines below) | 1 |
| of the 34: this row's own prose, line 26 | 1 |
| of the 34: **`crates/topo/src`** — the fold's subject | **21** |
| of the 34: outside `crates/topo/src`, all test-side | 11 |
| of the 41: not reaching `.shells` — **five `is_some()`/`is_none()` liveness checks, one `let _ = …` no-panic row (`review_m1_pr1.rs` ~:1041) and `faces_of_solid`'s guard-and-discard** | 7 |

**The atom's blind spots, closed by measurement rather than asserted:**

- `Solid` has exactly **one** field (`entity.rs`, `pub shells:
  Vec<ShellKey>`), so no caller resolves a solid and reads something
  else. That is also why the last row of the table is what it is: the
  seven `get_solid` sites that do NOT reach `.shells` read nothing at
  all from the solid — five ask whether it is live, one asserts that
  asking does not panic (`review_m1_pr1.rs` ~:1041, a `let _ = …`), and
  one is `faces_of_solid`'s own guard. **This row first called all six
  of the first group liveness checks**, which is one more than there
  are.
- No `Solid::shells()` accessor exists. `git grep 'fn shells'` over
  every tracked file returns `Body::shells` (the ARENA iterator, a
  different object) and one `sweep/tests` helper whose name merely
  starts with `shells_`.
- No site destructures `Solid { shells, .. }` as a pattern. Every
  `Solid { shells` in the tree is a `vec![]` CONSTRUCTION.
- **No site builds one solid's shell list by scanning the shell arena
  on the `Shell::solid` back-pointer** — the inverse spelling, and the
  one blind spot a field-read enumeration would otherwise have. 344
  `.shells()` calls tree-wide, 44 of them carrying a solid token within
  four lines, and every one of the 44 is an arena COUNT or a dump, not
  a per-solid selection.
- A textual enumeration has **no cfg blind spot**: it reads the source,
  so an `interval`- or `probe`-gated member is in the denominator even
  though no default build type-checks it. (Both lanes were compiled
  anyway; neither holds a member.)
- **The mutable-handle population is excluded, and that exclusion is
  a judgement rather than a blind spot — so it is named.** `git grep
  'get_solid_mut('` is **37 tree-wide at `cd9fdfd6b`** (39 once this
  row names the atom twice), **35 in `crates/topo/src`**, of
  which **33** read a solid's shell list through the mut handle; the
  other two are the accessor's own definition (a window over-fire onto
  the adjacent `get_shell_mut`, the same over-fire this census reports
  at `get_solid`'s own definition) and one `is_none()` assert. A
  `get_solid_mut(`-shaped enumeration would itself miss
  `splitting/finish.rs` (~:764), which takes `body.solids.get_mut(solid)`
  directly. Every one of them is a WRITE — `.push`, `.insert`,
  `.retain`, `.clear` — so a read door has nothing to offer them and
  they are not members. **But this unit's own headline diagnosis is a
  bucket dropped without being named**, and a population thirty-three
  strong, sitting one keystroke from the atom, is exactly the bucket a
  later lane would find and call a miss. It is here so that it cannot
  be. (The two raw writes inside this unit's own new test helper are in
  it.)

  **The figure was first written as 31, and the sentence that corrects
  it is a few lines below — where this row had already written it.**
  The 31 came from a five-line window; `movefac.rs` (~:171, ~:295) each
  put their `.shells` write six and seven lines below the call, pushed
  out of that window by a four-line `unreachable!` string. So *the
  instrument that corrects a count can be wrong the same way the count
  was* — twice in one row, the second time inside the paragraph arguing
  that a bucket is safely excludable. It **undercounted the bucket it
  was excluding**, which is the direction that reads as caution and so
  never prompts a re-take. The cheap check the five-line reading never
  ran: the figure is stable at eight, twelve and sixteen lines and
  moves only below eight.
- What it cannot see: a member assembled by a **macro**, which is
  unfalsifiable by text.

### What the row had wrong

- **Thirteen was an undercount of eight in its own crate**, and the
  eight it missed are all in `#[cfg(test)]` modules under `topo/src` —
  `body.rs` ×3, `euler.rs` ×1, `instance.rs` ×2, `movefac.rs` ×2. Both
  of the row's instruments could see them; the row's *subject* was
  stated as the production refusal shape, and a `#[cfg(test)]`
  assertion on the same read was bucketed out of it without being
  named. That is method item 9 for the third unit running.
- **`seqgen.rs` (~:1223) is not "the solid's ONE shell".** It asks
  whether `shell2` is the solid's **last** shell — a predicate no
  `the_only_shell_of` door could express. The row's group (b) was five
  members of four different questions.

## The one-door-or-two question, answered by the measurement

**One door.** Group (b) does not cohere:

| site | what it actually asks | would a `the_only_shell_of` door serve it? |
| --- | --- | --- |
| `euler_kill.rs` `kvfs` (~:422) | one shell, and the **count** when there is not one — `SolidNotSingleShell { solid, shells: n }` | no: the refusal carries the length, which a door answering `Option<ShellKey>` has thrown away |
| `seqgen.rs` `is_skeletal` (~:1042) | the `[x] = xs[..]` ARITY idiom, applied at the shell level and again at the face level on the very next line | no: its sibling is the FACE list two lines down, not the shell list |
| `review_m1_pr4.rs` (~:1610) | the same predicate as `is_skeletal`, written out again | no — and this pair is a duplication of `is_skeletal` ENTIRE, not of the shell read |
| `seqgen.rs` (~:1223) | is `shell2` the solid's **last** shell | no: not an arity question at all |
| `seqgen.rs` (~:1361) | `[0]` on a list a caller-side invariant says has one element | no: it is an unguarded index, and the invariant is `is_skeletal`'s |

So the split the row published as (a) 8 / (b) 5 is real as a split of
SHAPES and is not a split into two doors. All five of (b) read the
same list the same way and differ in what they then ask of it, which is
the caller's question, not the door's. **`Body::shells_of_solid` is the
one door; every one of the twenty-one sites reads it, or is exempt.**

## The shape, and the argument for it

```rust
pub fn shells_of_solid(&self, solid: SolidKey) -> Option<&[ShellKey]>
```

**`Option`, so the eight refusal types survive the fold.** The row
framed the eight as a posture question the fold had to settle. It is
not one: `None` is the only refusal the door can make, and each caller
keeps its own vocabulary with its own `ok_or`/`?`/`expect`. What folds
is the lookup, not the refusal — which is the same reading
`Body::solid_of_face`'s rustdoc already states for its own three
populations.

**A borrowed slice, not an owned `Vec`.** This is the one place the
door diverges from `Body::faces_of_solid`'s house form, and the reason
is that the two do different work: `faces_of_solid` SELECTS on the
faces' back-pointers and must build, while this reads a list the arena
already holds. Returning `Vec` would force an allocation on **ten** of
the fourteen production call sites, which today take none —
`combine.rs` (~:476), `ops.rs` (~:2063), `euler_kill.rs` (~:422),
`instance.rs` (~:364), `offset_together.rs` (~:824), `review_m1_pr4.rs`
(~:1610), `seqgen.rs` (~:1042, ~:1222, ~:1360) and `shell.rs` (~:1014).
**A door must not be worse than the spelling it replaces at any call
site.** The other four need to own and say `.to_vec()` where they said
`.clone()`, which is a wash; `shell.rs` LOSES a clone it never needed.

**This row first published that number as FOUR**, taken by eye rather
than counted, in the load-bearing sentence of the one place this door
deliberately diverges from its sibling's house form. The argument gets
STRONGER when the number is re-taken, which is the worst way for a
count to be wrong: nothing about the conclusion flags it. Seventeen
call sites in all, of which three are this door's own row. (The
instrument that first re-took it was wrong too — a four-line window
read `movefac.rs`'s `.to_vec()`, six lines below its call, as a
borrow.)

**What no mutation measures**, and these are the load-bearing reasons:

- Two **public** typed refusals stand on this guard —
  `EulerOpError::StaleKey { Solid }` (`kvfs`) and, one hop out,
  `BooleanError::JoinDesync`. A caller that cannot see the door still
  sees the refusal.
- `Option<&[K]>` is the house form for a borrowing read in this file:
  `Body::surface_source`, `curve_source`, `point_source` and
  `param_source` all return `Option<&T>` off a key.

The guard-deletion mutation below reds three rows and none of them is
in the integration suite, which is thin — the same thinness the
`faces_of_solid` fold reported for its own guard. It is evidence, not
the argument.

## The hit list, every hit dispositioned

**Folded — fifteen sites read the door:**

| site | disposition |
| --- | --- |
| `boolean/combine.rs` (~:476) | folded; the graft's shell attach |
| `boolean/finish.rs` (~:226) | folded; `.clone()` → `.to_vec()` |
| `boolean/ops.rs` (~:2063) | folded |
| `movefac.rs` (~:260) | folded; `.clone()` → `.to_vec()` |
| `shell.rs` (~:1015) | folded, **and a clone deleted**: the site cloned the list to hand `classify_shells_of` a slice it could have borrowed |
| `splitting/finish.rs` (~:339) | folded — one half of the four-line sequence the row named |
| `splitting/finish.rs` (~:726) | folded; `carve` |
| `offset_together.rs` (~:824) | folded; `Scope::walk`'s outer step — the seam this row shares with `two-spellings-of-the-face-to-solid-owner-index` |
| `euler_kill.rs` (~:422) | folded; the binding it kept for `shells.len()` is now the slice |
| `seqgen.rs` (~:1042) | folded; `is_skeletal`, and its `solid_data` binding went with it |
| `seqgen.rs` (~:1223) | folded; `.shells.last()` → `.last()` |
| `seqgen.rs` (~:1361) | folded; the unguarded `[0]` |
| `review_m1_pr4.rs` (~:1610) | folded. **The PR #17 attribution header in this file was not touched** — the diff is four lines inside one closure |
| `instance.rs` (~:364) | **folded — THE ROW MISSED THIS ONE.** A `#[cfg(test)]` count assertion, in the row's own crate, that both its instruments could see |
| `instance.rs` (~:383) | **folded, and not onto THIS door.** It resolved the grafted solid, walked its shells and flat-mapped their faces — which is `Body::faces_of_solid`, minted by PR #2898, whose own hit list does not name it. Its fifth instrument (every `Solid::shells` read reaching a `.faces`) had it in hand. **A fold leaving a member behind in a file it had open, for the sixth consecutive unit** — this one in the sibling's class, found because this unit had the same file open. **It is the sibling of a site #2898 EXEMPTED, and that owed an answer.** `instance.rs` (~:352–358) is exempt because it asserts every shell's back-pointer against its owner's list, and a door built on those back-pointers would assume what it checks. The site folded here is in the same file and the same suite and is NOT exempt, because its row's subject is key FRESHNESS — that no transplanted face reuses a destination key — and not back-pointer integrity. Its own predecessor walk counted faces through `solid.shells → shell.faces`; the door selects through `solid_of_face`; either way the row's question is about the key SET, which both answer, and the row's other assertion (the arrived count) is pinned by plant 4 |

**Kept hand-written — six sites in four rows, and the reason is one
rule:** a row that asserts an ownership LIST and its back-pointer
against each other must name the arena on both sides, or it asserts
`door == door`. This is the exemption class
`listing-a-solids-faces-is-spelled-four-times-in-topo-src` established,
read at the shell level.

| site | what it is the reference for |
| --- | --- |
| `body.rs` (~:1411) | the arena's solid→shell→face wiring, asserted against the shell's own `solid` back-pointer three lines above |
| `body.rs` (~:1488, ~:1494) | `Body::faces_of_solid`'s ORDER row, whose whole subject is that the shell walk and the arena scan are different sequences. The hand walk **is** the contrast |
| `euler.rs` (~:2825) | `mvfs`'s postcondition on the ownership list |
| `movefac.rs` (~:490, ~:491) | `move_shells_to_new_solid`'s effect on two solids' lists, asserted beside both shells' `solid` back-pointers |

**Not members:**

| site | why |
| --- | --- |
| `separation.rs` (~:409, ~:533), `validate.rs` (~:5435, ~:5921), `iso.rs` (~:131), `instance.rs` (~:354), `review_m0_pr7.rs` (~:282), `seqgen.rs` (~:760) | they iterate `body.solids()` and already **hold** the `&Solid`. There is no key to resolve and no refusal to make, so this door has nothing to offer them. The enumeration never fired on them — they carry no `get_solid(` — and they are listed because the row's own shape-shaped description ("the guarded shell list") reads as though it covered them |
| `crates/topo/src/body.rs` (~:859) | `Body::get_solid`'s own definition; the eight-line window reaches the arena read two lines below it. An instrument over-fire |
| `work/dup/the-guarded-shell-list-…-thirteen-times.md` (~:26) | this row's own prose. The enumeration reads every tracked file |
| eleven sites in `sweep/tests`, `topo/tests`, `editor-core/tests` | not this unit's ground; filed as `work/dup/the-guarded-shell-list-of-a-solid-is-spelled-eleven-times-outside-topo-src.md` |

## The mutations (merge base `cd9fdfd6b`, head `230c46738`)

Baseline **733 lib / 566 integration** (`cargo test -p topo --lib` and
`--test all --no-fail-fast`, own target directory outside the
worktree). 732 + the door's own row = 733. **Every row below sums to
the baseline**, which is how a panicking shard is caught: the lib
target aborted under plant 1 and its `test result:` line precedes the
suite's.

The restore harness rewrites the pre-plant BYTES and then diffs
against `HEAD` — *the restore restores exactly what the plant
changed*, the program's method item 17 as PR #2930 numbers it; every
run below restored clean.

**Directions, argued before any result was read** — *the direction of
a plant is argued before its result is read*, method item 16 as
#2930 numbers it:

- **Plant 1 SHRINKS.** Every answer loses its first shell; a
  one-shell solid answers the empty list. It must red every folded
  site that consumes the whole list. It is **not** uniformly
  tightening: `shell.rs`'s `if shells.len() == 1 { continue }` is
  RELAXED by it — a two-shell solid now reads as one and skips the
  `OperandOuterShells` refusal — so a green at that site would have
  been a green-wrong, not a clean site.
- **Plant 2 RELAXES.** A stale solid key answers the empty list
  instead of `None`, so no caller can refuse on one; every `ok_or`,
  `?` and `expect` downstream goes green and the caller proceeds on an
  empty list. It reds only rows asserting a typed refusal that names a
  stale SOLID, and rows asserting successful operation are untouched.
- **Plant 3 PERMUTES**, at one folded site. It neither grows nor
  shrinks what the join distributes, so it reds only if a consumer
  depends on the solid's list order.

| planted | direction | lib | integration |
| --- | --- | --- | --- |
| the door drops the first shell of every answer | **shrink** | 706 / **27** | 376 / **190** |
| a stale solid answers `Some(&[])` instead of `None` | **relax the guard** | 730 / **3** | 566 / 0 |
| `boolean/finish.rs` walks the door's answer in reverse | **permute** | 733 / 0 | 566 / 0 |

**Plant 1 leaves the FIFTEENTH site unmeasured, and this row first
claimed otherwise.** It published *"no folded site is dark"* over a
table whose `instance.rs` line lumped two sites together and credited
them with one row — while the same table correctly says
`Body::faces_of_solid` is unmoved by this plant. `instance.rs` (~:383)
reads `faces_of_solid`, so nothing in plant 1 touches it. Run filtered
to `instance::`, plant 1 gives **4 passed / 1 failed**, and the
failure is `a_graft_adds_a_whole_second_solid_and_shares_nothing` —
site 14 — while site 15's row,
`the_graft_mints_fresh_keys_for_every_transplanted_entity`, stays
green. **The one claim the whole plant section exists to support was
false for one site in fifteen, and the evidence against it was inside
the section.** Plant 4 below measures that site; the corrected claim is
*no folded site is dark across plants 1 and 4 together*.

**Plant 1: the other fourteen.** Every one reds, and the mapping is
one-to-one:

| folded site | rows it redded under plant 1 |
| --- | --- |
| `euler_kill.rs` (kvfs) | `cube_tears_down_to_the_empty_body`, `kvfs_inverts_mvfs_to_the_empty_body`, `kvfs_precondition_errors_are_atomic`, `kvfs_rejects_extra_shells_and_rings`, `kvfs_roundtrips_with_mvfs_beside_another_solid` |
| `movefac.rs` | `move_shells_to_new_solid_{refuses_typed_at_each_precondition, replay_is_byte_identical, splits_one_solid_in_two}` |
| `offset_together.rs` | `scope_walks::{a_scope_holds_only_the_solids_it_names, an_out_of_scope_faces_unmintable_chart_does_not_refuse_the_door, an_out_of_scope_solids_corruption_does_not_refuse_the_scope_walk, the_two_hops_refuse_differently}`, `shell10_r2_probes::r2_a_re_scope_up_holds_the_solid_it_was_aimed_at` |
| `seqgen.rs` ×3 | 4 `seqgen::` rows plus `validate::random_sequences_then_teardown_validate_vacuously` |
| `review_m1_pr4.rs` | `genus_two_double_hole_body_tears_down_to_nothing`, `kvfs_slot_recycling_is_generation_safe`, `seqgen_generates_every_op_kind_and_every_site_shape` |
| `instance.rs` (~:364) and `boolean/combine.rs`, which the graft calls | `a_graft_adds_a_whole_second_solid_and_shares_nothing`. **`instance.rs` (~:383) is NOT in this row** — see plant 4 |
| `body.rs` (the door's own row) | `shells_of_solid_answers_the_solids_own_list_order_not_the_arenas` |
| `shell.rs` | integration: `shell_roles` ×3, `void_door` ×6 |
| `splitting/finish.rs` ×2 | integration: `m3_pr3_split` ×5, `review_m3_pr3_{bob, consumer, order, rings}` ×9 |
| `boolean/finish.rs`, `boolean/ops.rs`, `boolean/combine.rs` | integration: `bool4r1_probes` ×5, `bool4r2_probes` ×5, `issue93_nested_islands` ×5, `issue86_double_subtract`, `merge_skip`, `bool4r2_base_probe`, and the mate/rest rungs downstream |
| `boolean/solid_contain.rs` (NOT folded here) | `a_group_read_key_shared_across_the_selection_refuses_typed` — it reads `Body::faces_of_solid`, which is unmoved; this red is the door pair agreeing, not a site |

**Plant 2's three reds are the argument's evidence and not its
argument.** Two of them are not the door's own row —
`euler_kill::kvfs_precondition_errors_are_atomic` and
`review_m1_pr4::kvfs_slot_recycling_is_generation_safe` — and both
assert `EulerOpError::StaleKey { EntityId::Solid }`, a **public** typed
refusal, on a stale solid key. That is the same shape that carried the
`faces_of_solid` guard (a public `PointInSolidError::NoSuchSolid`
downstream), and it is what the choice rests on. Zero integration rows
red; a reviewer weighing red counts alone would read this guard as
unjustified.

**Plant 4 measures the fifteenth site, and nothing else could.**

- **Direction, argued first: SHRINK.** `Body::faces_of_solid` drops the
  first face of every answer. Site 15's row asserts two things — that
  the grafted face list has the source's length, and that no grafted
  key is among the destination's original keys. A shrink TIGHTENS the
  first (the length goes short) and **RELAXES the second** (fewer faces
  to collide), so if the row reds it reds on the length, which is the
  assertion the folded read feeds. A green would have meant the site
  is dark.

| planted | direction | lib | integration |
| --- | --- | --- | --- |
| `faces_of_solid` drops the first face | **shrink** | 725 / **8** | 564 / **2** |

Restricted to `instance::`: **4 passed / 1 failed**, the failure being
`the_graft_mints_fresh_keys_for_every_transplanted_entity` — the
complement of plant 1's, so the two `instance.rs` sites are now
separately measured and neither is dark. The plant is live well beyond
this site: `faces_of_solid_restricts_the_face_arena_to_one_solid`,
`faces_of_solid_answers_arena_order_where_the_shell_walk_would_not`,
`props::face_list_door_tests::a_solid_s_faces_enclose_that_solid_s_volume_in_a_shared_arena`,
two `offset_together::scope_walks` rows, `shell10_r2_probes::r2_a_…`,
and two integration rows (`bool4_material_containment::a_part_in_a_cavity_clears`,
`bool4r1_probes::probe_g_…`).

**Plant 3 is a null, and its control is plant 1**, which reds this
exact site broadly. The control argument and the red list are argued
once, on the row that holds the finding:
`work/tint/the-boolean-joins-shell-processing-order-is-unasserted.md`.

**Plant 5 was a null whose control was the WRONG control.**
`seqgen::fusion_remake_shell`'s `.last()` → `.first()` reddened
nothing, and so did a control that replaced the whole body with
`-> None`; this row read the pair as proof the site is unasserted
entirely. It is — but the pair does not show it, and a `panic!` plant
does: **732 / 1**, the red being
`seqgen::random_op_sequences::random_op_sequences_hold_all_properties`,
so the site is reached. **Both runs and the rule they yield are argued
in full on `work/tint/seqgen-fusion-remake-shell-is-dark.md`**, which
is the row that holds the measurement; the rule is the program's
method item 19 and does not want a third copy here.

## X4 the fold minted — THREE, of which the unit self-caught one

The two the unit did **not** catch are both in `body.rs`, the file
whose door prose this unit censused, and both were found by the reader
who did not write the fix. That is this program's standing result, now
at four units running.

**Each of the three is argued in ONE place, and this section is not
that place for two of them.** The sibling row set that rule —
*"**This section is the single home for the argument**; the sibling
rows point here rather than restating it, because a restated argument
with restated numbers is `orient-module-prose-accumulation` minted by
the unit disclosing that it had not minted one"* — and the first
draft of this section broke it on both: it carried the fixture
argument in full beside the row that owns that class, and the clause
argument in full beside the row that now has a bucket for it.

| X4 | what it was | fixed by | argued in full at |
| --- | --- | --- | --- |
| **the fixture** | the door's new reference row hand-built the same-solid two-shell body a **fourth** time, forty lines below a copy an open row already prints statement by statement, and dropped the comment saying why the arena removal must be PAIRED | both `body.rs` copies fold onto one local `adopt_shell_into`; the class stays at three SITES and where the shared fixture lives is still that row's decision | `work/dup/the-same-solid-two-shell-body-is-hand-built-three-times.md` |
| **the clause** | a third `body.rs` door paragraph opening with the byte-identical bolded **"`None` is the only refusal this door can make"**, in a prose class this unit's own residue row had no bucket for | the door keeps the FACT at the door and makes the argument a pointer | `work/dup/the-stale-vs-foreign-key-clause-is-spelled-at-thirteen-body-doors.md`, second family |
| **the foreign-key sentence** — the one the unit self-caught, before the push | `faces_of_solid`'s sentence rewritten with "shell list" for "face list", ten doc lines above it. The order argument was written twice the same way | both trimmed to pointers | the same residue row, first family |

**No `.expect` / `.unwrap()` tax.** The `faces_of_solid` fold paid
eighteen sites of ceremony because it turned a `&Solid`-returning
lookup into an `Option<Vec<FaceKey>>`. This fold pays none: both
spellings already carried the same `Option` and the same refusal, so
every site's `ok_or` / `?` / `expect` is the one it already had, and
the four `.clone()`s became `.to_vec()`s of the same length.
`shell.rs` lost a clone outright.

**No `review_m1_pr5_internal` door-table entry is owed.** That guard's
population is `pub fn … &mut self` under `topo/src`
(`crate::source_walk::mutation_doors`); `shells_of_solid` is `&self`
and outside it. The `faces_of_solid` fold's own lesson — that moving a
`pub fn … &mut …` into `topo/src` costs a table entry — does not
reach a read.

## Closed

Folded 2026-09-20. **Fifteen** `topo/src` members read a door;
fourteen read `Body::shells_of_solid` and one reads
`Body::faces_of_solid` (a member the PREVIOUS fold left behind). Six
sites in four rows stay hand-written under the reference-walk
exemption, stated above with the rule that decides them.

Residue, one file each:

| residue | file |
| --- | --- |
| the eleven sites outside `crates/topo/src`, all test-side | `work/dup/the-guarded-shell-list-of-a-solid-is-spelled-eleven-times-outside-topo-src.md` |
| the stale-vs-foreign clause at thirteen `body.rs` doors, **and the "`None` is the only refusal" clause at three** — two X4 families, one instance of each minted here and fixed | `work/dup/the-stale-vs-foreign-key-clause-is-spelled-at-thirteen-body-doors.md` |
| `seqgen::fusion_remake_shell` is unasserted entirely — its own control is a null | `work/tint/seqgen-fusion-remake-shell-is-dark.md` |
| the boolean join's shell-processing order is unasserted | `work/tint/the-boolean-joins-shell-processing-order-is-unasserted.md` |
| the owner index: two doors that already DISAGREE about a lone vertex, measured — **and the falsification is pinned by nothing in the tree** | `work/dup/two-spellings-of-the-face-to-solid-owner-index.md` |
| the same-solid two-shell body: a FOURTH hand-build, minted by this unit's own reference row and folded back to a local helper | `work/dup/the-same-solid-two-shell-body-is-hand-built-three-times.md` |
| `review_m1_pr4.rs`'s *"the probes are otherwise verbatim"* — a closed exception list this unit's fold added to without amending | `work/dup/review-m1-pr4s-verbatim-sentence-is-one-edit-less-true.md` |

## A note on how two of these corrections nearly did not land

Three times in this fix pass, several edits were applied from one
script that asserted every anchor before writing anything. A later
anchor missed, the script aborted, and **the edits that had already
matched were never written** — while the pass reported all of them as
done and went on. Two of this row's corrections and one of a sibling
row's are here only because the delta read asked about a number and
the number was not in the tree to check.

It is the same failure as every other one this unit recorded, at the
level of the tooling rather than the prose: **an operation that
reports success for work it did not do**, and nothing between the
report and the reader that re-reads the artifact. The repair is the
one this repo already prescribes for measurements — read the tree, not
the transcript — and the cheap mechanical form is to apply and verify
each edit independently, so one failure cannot silently take its
neighbours with it. Both re-applications above were made that way and
each printed its own confirmation.
