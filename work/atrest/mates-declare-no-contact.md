---
id: mates-declare-no-contact
kind: issue
title: D10 stage 4 PR I: ContactClass on mates, class_admission and MintedDeclaration retire; at rest every contact between copies, mated ones included, is recorded at the door
status: closed
pr: 5637
closed: 2026-10-24
opened: 2026-10-08
priority: P0
cost: M
branch: atrest/mates-declare-no-contact
---

INTENT stage 4, PR I. Spec: `docs/INTENT-STAGE4-SPEC.md` §9.

`Node::Mate.class`, `ContactClass`, `class_admission`, `MintedDeclaration`, `NoAtRestRecord` and `FIT_DEFERRAL` go. The at-rest census records every contact between copies at the door, mated ones included. Pierces and same-side crossings stay interference, as stage 5 B (`interference-at-rest-is-a-finding`) leaves them; stage 5 C reports these rows. The perf12 census goldens are re-blessed as rows.

A5's hard error on an unattributed contact was cut out of this unit into J (`an-unattributed-contact-at-rest-is-a-finding`, orchestrator's ruling 2026-10-09) so stage 3's placement unit can wait on it. I extends J's recording to the contacts a mate used to attribute.

## 2026-10-22 — opened

Its two triggers have closed: `the-door-reads-a-mated-seat-by-its-mate`
(PR 5587) and `the-census-records-a-planar-area-contact-as-one-face-pair`.
At rest an unmated area contact between copies is now one face-pair row
and a flush pair a row of its own, so recording a mated contact at the
door records the same shape.


## Handoff

Built on `atrest/mates-declare-no-contact`. A mate is `{on, to}`: the
class, `ContactClass`, `class_admission`/`ClassAdmission`,
`MateFault::ClassNotAdmitted`, the solve gate, `MintedDeclaration` and
the whole mint and attribution machinery (`Mint`, `MintRefusal`,
`Attribution`, `Relation`, `CarriedDeclaration`, `CarriedRefusal`,
`AssemblyError::Uncertified`/`UnmintedMates`/`CarriedMintRefusal`,
`AtRestBy`), `NO_AT_REST_RECORD_RECOURSE`, `FIT_DEFERRAL` and its steer,
and `kernel_wire::contact_class` are gone. `DeclaredContact.class` is a
`GlueClass`; Door 1 is `topo::rest_pair_verdict`.

The gather calls `topo::record_at_rest` from row 0: every touch between
copies, mated or not, is a `CensusAtRest` row (`AtRestRow { copies, row
}`) and a record citing it; a pierce or same-side crossing stays
interference. The coincidence door proves a mated seat's row by its
first rung (`stated_by_a_mate`). `Separation` quiets a pair only on a
record citing a row that rung proves (deviation, filed:
`separation-quiets-only-first-rung-rows`).

Content-key `VERSION` 20. Mate wire `{on, to}`; ids after a mate move.
Re-blessed: perf12 census ×3 (the die tour's ids and the `(..)`
spelling only), `mint_node_ids`, `slot_tables`, `m10_6_certifying_keys`,
the die corpora; msolve14/m4_pr4/seat4 digests re-pinned.

Surfaces: Python (`Mate(on, to)`, no `ContactClass`, no mint classes,
`AssemblyError` arms `at_rest`/`carried_at_rest`/gather tags), viewer
(no class choice, no tree note, the badge certifies without a count),
tour. Docs: ASSEMBLY A3/A5, topo README C4/C6, the assembly guide.

Closed: `stage4-i-deletes-the-mate-mint-and-must-keep-the-records`,
`the-census-reads-each-mated-patchs-decision-twice`. Filed:
`the-contact-recourse-names-a-class-nobody-can-declare`,
`separation-quiets-only-first-rung-rows`. `the-at-rest-census-is-a-check`
stays stage 5 C.

Fix pass after review 1:
- Six slow-set id pins re-pinned (`lib_g16_corpus_name_digests`,
  `edit_placement_corpus_bits`, `m10_p_fence` f64 and interval,
  `perf2_name_keying_differential`, `name_words_corpus`). Exactly the 17
  mate-bearing corpus documents moved; the ids-masked fence held.
- The first rung has one home (`coincide::first_rung`), shared by
  `prove` and `first_rung_proves`. `asm_r2b_assembly`'s
  `a_mate_proves_its_faces_only_in_the_sense_it_states` pins the sense
  check: it goes red under the reviewer's mutant.
- Filed `two-copies-filling-one-volume-certify-at-rest` (P1) and
  `a-declared-contacts-class-admits-continuation`. The contact recourse
  row records why its rewording waits on a ruling.
- `persist/kernel_wire.rs` deleted. `test_validate`'s record row
  re-stated. Tour comment and `at_rest` doc fixed. Archaeology swept.
