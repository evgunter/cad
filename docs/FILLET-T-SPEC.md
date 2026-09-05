# FILLET-T — Track T's two riders on the blend surgery (spec)

**Program:** FILLET (`work/fillet/plan.md`), units `D325` and `D326`
(`work/fillet/D325.md`, `work/fillet/D326.md`), taken together as ONE unit
because both open `crates/sweep/src/blend/surgery.rs`'s corner fusion and
both were held while FILLET lanes contended that file; every FILLET lane
has landed, so the contention is gone and the plan's exit shape ("Track T
is empty") is what this unit closes. **Track:** kernel change — the
standard v6 unit (binding spec, drawn implementer arm, cross-model dual
review, union fix pass, record-at-merge; §Review below). **Pre-draw
fields, logged before the draw:** difficulty **S**, task-class
**STRUCTURAL**.

- **S** — two code-quality rows: a seeded return through three `sorted`
  call sites, and the same invariant spelled at six `kef` sites; no new
  behaviour on any body.
- **STRUCTURAL** — no predicate, band or margin moves; every existing
  carve bit-identical to the merge base by the dump.

## The claim

1. **D325 — the corner fusion's `first_arc` `unreachable!` goes.** The
   proof that a corner's incidence list holds at least the link that
   discovered it is a fact of the type one level up (`CornerLinks::first`
   returns a link, not an `Option`); `CornerLinks::sorted` hands back a
   `Vec` and loses it. Carry the non-emptiness: `sorted` returns a seeded
   shape (`(first, rest)` — the shape `CornerLinks` already has in its
   own fields) and the arc loop consumes it, so `first_arc` is a value,
   never an `Option`, and the `unreachable!` has nothing to guard. Three
   `sorted` call sites move; the ~forty-line mutation body holding
   `&mut Body` is hoisted, not duplicated.
2. **D326 — the `kef`-argument invariant carried at the call sites.** The
   blend surgery hands `kef` a half at six sites (the blank phase's
   edge-strip and corner-strut `kef`s, the rim and rim-strut pair, the
   annulus rim and seam-crossing pair — plus whatever H7's ruled phase
   added; count them on the head) and at each the face `kef` kills is one
   this surgery MINTED, established by six unrelated local arguments and
   stated at none. Spell it once the way `topo/src/shell.rs`'s
   `canonicalize_chart` does — a picker that structurally refuses to hand
   a source face to `kef` (`dying`/`anchor`), homed beside `attach_contact`
   or `flank`, called at every site — so `blend::naming::Retired`'s absent
   face channel becomes a consequence of the code and `D323`'s
   five-paragraph argument at the type shrinks to two sentences.

## Phase 1 — measure before touching anything

- Enumerate every `kef` call in `surgery.rs` on the head (the six D326
  names plus H7's); for each, record which `mef` minted the face it kills
  and how the site establishes that today (the `he_plus`-in-the-OLD-loop
  contract, `trim_chords`'s `pick(true)`, prose). If any site kills a face
  that is NOT surgery-minted, STOP and report: the invariant is not what
  D326 says, and this unit does not proceed on a false premise.
- Read the three `sorted` call sites and the arc loop; record what each
  reads of the `Vec`.

## Phase 2 — the change

Items 1 and 2 above; `D96`'s row-0 test (*can the type stop representing
the state?*) answered yes at both. Prose: `D323`'s argument at
`naming::Retired` cut to its consequence; the `unreachable!` census in
`surgery.rs`'s header (Row 4) re-counted.

## Constraints, binding

- Every existing carve bit-identical to the merge base (the dump, all
  corpora incl. H7's ruled rows and the concave corpus).
- No new metered predicate; nothing decides.
- One home for the picker; the three `sorted` sites call one seeded
  shape.

## Acceptance

Phase 1's table; the `unreachable!` gone with `first_arc` a value; the
picker at every `kef` site with the count stated; `D323`'s text reduced;
dump identical; hosted CI green (the full matrix runs on every PR).
`D325` and `D326` closed by this PR.

## Out of scope

Any other `unreachable!` in the file (Row 4's census stays); the
`surgery.rs` split (Ev's, PR 1916) — if that ruling lands first, this unit
rides the split lane instead.

## Review

v6 dual on the frozen head, claims to falsify:

- **C1** Every existing carve is bit-identical to the merge base.
- **C2** `first_arc` cannot be `None` by the TYPE (no `Option`, no
  `unwrap`, no `expect` on that path); the three `sorted` sites read the
  same seeded shape.
- **C3** The picker is called at EVERY `kef` site in the file (count them
  independently) and cannot hand `kef` a source face (plant a mutant that
  passes the other half; it must refuse or fail to compile, never carve).
- **C4** `D323`'s prose at `naming::Retired` states only what the code now
  enforces; nothing else in `crates/` still argues the six-site invariant
  by hand.
