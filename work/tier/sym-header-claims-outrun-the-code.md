---
id: sym-header-claims-outrun-the-code
kind: issue
title: three more header sentences the code has outgrown: the id-determinism claim, the two-clause Zero claim, and the constructor census
status: open
opened: 2026-09-14
priority: P4
cost: E
---


**Found by SYM-2's review**, on the head that split `sym.rs`. The third,
fourth and fifth instances of the class the parent item
(`sym-rs-is-one-file-with-a-347-line-header`) was filed for: a contract
sentence in the tier's header that the code below it no longer honours.
Left unedited — SYM-2 edits no sentence's technical content — and filed
here so a unit that touches each mechanism can take it. Line numbers are
at `c9a38f5e4` and will rot; the sentences are the citation.

## 1. "the same under every rayon schedule and every insertion order"

`crates/geom-core/src/sym.rs` (`# Node ids are CONTENT HASHES (D9)`,
~`:337`):

> A node's id is a 128-bit structural hash of `(op, children ids,
> payload bits)` — never a sequence number. An id is therefore the same
> under every rayon schedule and every insertion order.

`Sym::opaque` mints `SymOp::Opaque` with `OPAQUE_SEQ`'s next value **as
the payload**, so an opaque node's id is a function of the ORDER its leaf
minted its opaque values in, not of its expression alone. The claim holds
only under the premise `OPAQUE_SEQ`'s own doc states and argues — a leaf
is replayed by one fixed single-threaded walk of its recipe, and
[`with_session`] resets the counter around every replay — and the header
states neither the premise nor the exception. "Never a sequence number"
is false for exactly one op, and that op exists because the alternative
was a soundness defect (`SymId::UNRECORDED`'s doc).

## 2. "answers `Ok(Sign::Zero)` … exactly when two things hold"

`crates/geom-core/src/sym.rs` (`# What a symbolic `Zero` claims, and why
it is a THEOREM`, ~`:25`):

> [`Decide::sign_within`] on a [`Sym<T>`] answers `Ok(Sign::Zero)`
> without consulting the enclosure exactly when two things hold: 1. the
> value channel certifies …; and 2. the node's POLYNOMIAL NORMAL FORM
> over the parameter symbols is the zero polynomial.

`discharge` answers three ways, and `sign_within` returns
`Ok(Sign::Zero)` for all three: `Theorem` (clause 2 as stated),
`SignGated` — rule C's clause-3 fold, which READS A VALUE over the
leaf's box, so the form is zero box-wise and not identically in the
parameters — and `Registered`, an AXIOM a constructor stated, where no
normal form of the node is the zero polynomial at all. The header says
so itself, hundreds of lines below, in the M10-9 and M10-10 sections and
on `SymCounts`; the two-clause claim at the top is the M10-7 tier's and
was never widened. It is the load-bearing sentence of the whole document,
which is what makes it worth a row: a reader who stops at it believes the
`registered` column is a theorem count.

## 3. The constructor census: "it guarantees two", "the second constructor"

`crates/geom-core/src/sym.rs` (`# The registered-identity door (M10-9)`,
~`:136` and ~`:148`):

> The swept arc carrier's builder registers EVERY same-object identity it
> guarantees whose consumer node it can build identically, and it
> guarantees two … The revolve's latitude carriers … state the RIM
> identity too — the same rule applied to the second constructor.

There is a third constructor: `crates/sweep/src/extrude.rs` (~`:1128`)
states the rim identity for an arc wall, by calling
`swept::register_rim_identity(rim, radius)`. The allowlist gate
(`scripts/gates/register-equal-allowlist.sh`, its RATIFIED SITES header)
knows and exempts it — "`crates/sweep/src/extrude.rs` reaches the rim
identity THROUGH `register_rim_identity` and therefore needs no entry" —
so the census is stale rather than wrong about soundness.

**But the exemption is held by convention, which is the one thing the
header says the door is not.** `sym.rs` ~`:153`: "Where the door may be
called is an ALLOWLIST, **not a convention**". The gate's caller arm
greps `\.register_equal\(` over the code-only view, so it sees the two
bodies in `swept.rs` and nothing else: the wrapper route is invisible to
it, and a fourth caller of `register_rim_identity` — or a sixth
constructor minting the same circle — lands with no gate, no entry and no
census line. What is owed is either a gate that follows the wrapper or a
header (and gate header) that says the unit of scope is the REGISTERING
FUNCTION and lists its callers.

**Territory.** The gate half is `guard`'s ground:
`python3 scripts/work.py territory --files -` answers
`scripts/gates/register-equal-allowlist.sh: owned by guard`. The header
half is `sym`'s. Filed here because the claim is `sym.rs`'s; whoever
takes it needs `guard` for the gate.
