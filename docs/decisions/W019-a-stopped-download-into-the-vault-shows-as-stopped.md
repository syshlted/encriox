# W19 — A stopped download into the vault shows as stopped, and *Resume* brings it whole

*2026-11-20 · built and measured on the user's word ("We should fix these before continuing, if they are not fixed
yet"), as stage 1's patch 0020, on the A20. Ends `ROADMAP.md`'s S4 and W5's flag 2, and **amends W5's first
rejection**. The G6's vault is locked and the S9 was not attached: both are unmeasured.*

#### What was wrong

A download into the vault that was paused or cancelled showed as failed (W5, flag 2). The writer throws once the copy
returns, to abandon a write the vault cannot keep in part (W5), and Android Components' download service marks failed
any download whose writer throws, over the status the user asked for. W5 rejected leaving it paused, because *Resume*
looks for the partial file in `Download/`, finds none, and fails.

#### What was settled

1. **A paused or cancelled download stays so when its writer throws.** `startDownloadJob` leaves the status the user
   gave it, and logs that the writer kept nothing.
2. **A download nothing was kept of resumes from the start**, file or no file. The writer already forgets what was
   copied (W5, patch 0008); *Resume* now reads that, where it looked for a file the vault never had. Fenix's own writer
   is unchanged: one it kept part of still resumes from where it stopped, or fails without the file, as before.
3. **What the writer forgets reaches the download's state**, so a paused one no longer shows part of itself copied.
4. **The gate's retry sends the button the notification offers**: *Resume* for a paused download, *Try again* for a
   failed one, and none for a cancelled one, which has left the service. `check-stopped` fails if a stopped download
   ever shows as failed, settles as anything but what stopped it, or leaves anything in the vault; and if a resumed one
   is not whole.

#### Measured (2026-11-20, the A20, `check-stopped`, 16 MiB at 2 MiB/s stopped after 3 s)

| | Before, patch 0019's build | After, patch 0020's |
|---|---|---|
| Paused, with a length | PAUSED, then FAILED; *Download failed* | PAUSED; *Download paused* |
| Paused, without | — | PAUSED; *Download paused* |
| Paused, then started again | PAUSED, FAILED, then *Try again*: whole | PAUSED, then *Resume*: whole, 16777216 bytes, with a length and without |
| Cancelled | CANCELLED, then FAILED | CANCELLED, with a length and without |
| In the vault, stopped | nothing | nothing |

The check failed on the build before the patch, every time, and passed on the build after it.

#### Rejected

- **Telling the writer through `DownloadFileWriter`**, as W5 rejected it: the service's own status already says why
  the copy stopped.
- **Leaving *Try again* as the way back**: a paused download offers *Resume*, and that is the button a user presses.
