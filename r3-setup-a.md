## Setup for round 3, designer A
- Run `mkdir -p /root/.local/share/cad-work/fork-svc-a && git clone /home/user/cad /root/.local/share/cad-work/fork-svc-a/cad`.
- After that, run every git command as `git -C /root/.local/share/cad-work/fork-svc-a/cad …`. Never chain a `cd` with `;`: in round 2 two designers' commands ran in `/home/user/cad` because a `cd` failed.
- Fetch `https://github.com/evgunter/cad.git gauge/in-arc-pairing main`.
- Target: `/home/user/fork-svc-a-target`, with `CARGO_INCREMENTAL=0` and `local-scripts/with-build-slot.sh -- <cmd>`. Debug builds only.
- Report to `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/af-svc/A-r3.md`, and return it as your final message.
- Delete the clone and the target when you are done.
