---
id: the-seated-tools-label-nouns-are-not-pinned-to-the-nodes-they-mint
kind: issue
title: tool_noun's kind words for the seated tools are held to the nodes they mint by nothing
status: open
opened: 2026-10-01
priority: P4
cost: E
refs: [node-labels-are-document-data]
---

A create form's proposed label counts by a kind noun that has to be `node_kind_noun`'s word for the node the form mints. The datum form's nouns (`DatumKindChoice::noun`) and the profile/extrude consts are held to minted nodes by tests (`drafts::tests::each_datum_choices_noun_is_the_kind_of_the_node_it_commits`, `pane::create::creation_nouns`); `ViewerBehavior::tool_noun` (`crates/viewer/src/pane/create.rs`) — Revolve, Boolean, Split, Transform, Pattern/PlacedUnion, Fillet/Chamfer, Part (projection and duplicate), Mate — is held by nothing. A wrong word only misnumbers the proposal (and leaves a typed draft unspent, since `Drafts::creation_landed` finds the draft by the minted node's noun). Pin it: lift the match to a free function over the tool and its two choices, and assert each against the noun of the node its op mints (the sample ops in `viewer/tests/node_labels.rs`'s every-creation row are the scene).
