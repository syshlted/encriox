# W7 — Stage 2 tracks Firefox's release, not an ESR

*2026-11-17 · accepted on the user's word ("I agree that this should be a release, given the breadth of changes since
the last release"), answering W1's item 2. Nothing of stage 2 is built or measured yet.*

#### What the user decided

- **Stage 2 spikes against a release**, as stage 1 did, and not against an ESR.
- **Its builds go on a drive of their own**, `/home/coder/extra_space/encri-extraspace`, which the user made for them:
  387 GB free on 2026-11-17. The home drive had 40 GB free that day, down from 63 GB on 2026-11-13, and stage 1's
  Firefox checkout holds 15 GB of it.

#### What was settled

1. **Stage 2 starts from stage 1's release, Firefox 160.0, and the GeckoView Mozilla published for it.** Its first
   questions are asked of stage 1's build with Lockstore's prefs alone, which W5 found in that GeckoView's `libxul`.
   Only a key-encryption key from the vault needs a Gecko build, and that build is of whichever release stage 2 is on
   by then.
2. **A release is followed, not held.** W9 makes encriox follow each one, and stage 2 is where that first meets
   Lockstore.
3. **Stage 2's checkout and object directory go on that drive.** Mozilla's toolchain stays under `~/.mozbuild`, and
   stage 1's checkout stays where it is until it is retired.

#### Found, not measured

- **Firefox 160.0.1 is tagged and has no GeckoView on Mozilla's Maven.** On 2026-11-17, `FIREFOX_160_0_1_RELEASE`
  (`58fed86b1b`) is in Mozilla's repository, with Android fixes among its changes (a blit on Mali GPUs, the file
  picker's permission result), and the newest `geckoview-omni` on maven.mozilla.org is still `160.0.20261104181552`.
  Whether it is late or will not come is *unverified*. A build against a published GeckoView can only follow the
  releases Mozilla publishes one for (W9).

#### Rejected

- **ESR 153**, which runs to September 2027. It has Lockstore's SQLite encryption but not
  `LockstoreStorageKeyProvider`, and Mozilla ships no Android ESR, so every Android security fix would be backported by
  hand, as Tor Browser's team does (the study, *What carrying the fork costs*).
- **Waiting for ESR 175**, due June 2027 and only likely to carry all of Lockstore (the study; *unverified*).
