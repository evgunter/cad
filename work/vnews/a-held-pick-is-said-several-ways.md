---
id: a-held-pick-is-said-several-ways
kind: issue
title: A held pick's empty state is three sentences and a face pick is spelled two ways
status: open
opened: 2026-09-25
priority: P3
---


Found by VNEWS #3281's style review (findings 9 and 10), after that PR
put the seated and mate panels' held-picks line through one composer,
`seats::picks_line`. The panels that hold a pick and do NOT go through
it say the same two things in other words.

## A held pick's empty state is three sentences

- `no picks yet` — `seats::picks_line` (`seats.rs`), for every seated
  panel and the mate panel;
- `no edges picked yet` — the blend panel,
  `ViewerBehavior::blend_tool_ui` (`pane/create.rs`, near line 1500),
  a literal that repeats `BlendError::NoEdges`'s `Display`
  (`blend.rs`, near line 215) without reading it;
- `none picked` — the face-frame datum form,
  `ViewerBehavior::datum_face_frame_rows` (`pane/create.rs`, near
  line 859).

The blend one names what is missing (edges), which the others do not;
whether that is a reason to differ or a gap in `picks_line` is the
decision.

## A face pick is spelled two ways

- `face of feature N` — `matetool::face_of` (`matetool.rs`), the mate
  panel item and its drop notice;
- `<node> body B` — `Say for BlendTarget` (`blend.rs`), which the
  blend panel's target line and the face-frame datum form draw from the
  landed document, the form for its held face through
  `BlendTarget::of_face` (`pane/create.rs`, `datum_face_frame_rows`).

The same kind of held value — a face the user clicked — is said as the
face in one panel and as the drawn body it sits on in another. The
datum form's choice is argued at the site (the scope a pick is on, in
the one sentence that names it); the mate's is argued at `face_of`.
Neither mentions the other.

## Territory

`pane/create.rs` is claimed by AUTHOR, CHROME, FORMS, VSEAM and this
program; `blend.rs` by AUTHOR, CHROME and VSEAM. The sentences are
held-pick news, this program's vocabulary since PR 3281, which is why
the row is filed here.
