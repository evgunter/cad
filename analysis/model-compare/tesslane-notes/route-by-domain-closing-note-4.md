Step 2 (a) and (b) are done on 992f7d8986's code. 1d1a221148 changes no code.

- mesh whole with the slow set: 528/528 at 1e-9, 1e-6 and 1e-12.
- That includes `route_rows` (K stream on a run and on a memo hit, bytes), D9, and PR 7054's `split_rulings` and `trimmed::tests` rows.
- No digest moved at any ε, so there is no re-pin commit. The split-side cone digests are unchanged by PR 7054 under this route.

Still running: (c) mesh clippy, then (d) the viewer's slow-set rows and sweep's meshing modules. Results to follow.
