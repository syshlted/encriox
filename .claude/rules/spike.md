---
paths:
  - "scripts/spike/**"
  - "LICENSE"
---

# The stage 1 spike — component brief

`scripts/spike/stage-1/`: Fenix at a Firefox release, built with Gradle against the GeckoView Mozilla published, with
the patches in `patches/` on top, and the scripts that fetch, build and measure it on the phones (W1, W4). It is a
throwaway, and nothing it builds ships. Its `README.md` says how to build it and run it. `LICENSE` is here too until a
build of encriox itself has a brief. This brief is level 1; full records are in `docs/decisions/`.

## Rules that hold here

- **The package is `com.systemhalted.encri.browser.spike`, on test phones alone, never published** (W1, flag 1). The
  phones are the ones holding encri's test vaults; the S24 only with its owner's word (`AGENTS.md`, *Hazards*).
- **The patches are the source.** Change them by committing on the checkout's `encriox-spike-stage-1` branch and
  writing them out again with `git format-patch` (`README.md`). Never edit a `.patch` by hand. A fix is a patch of its
  own on top, so that what each measurement ran is a prefix of the series.
- **Mozilla's toolchain, not the suite's.** Gradle runs on Mozilla's JDK 17 and its SDK under `~/.mozbuild`
  (`README.md`), so the suite's JDK 21 and SDK, which are encri's toolchain, are left as they are. The build includes
  `../encri-api` from source, so it is still one Gradle invocation at a time in the whole suite.
- **A new source file begins with Exhibit A's notice** (W2, flag 1), as Mozilla's own do.
- **A download into the vault is whole or not there.** encri saves what a writer finishes (encri-api L11), and `write`
  replaces a document whole, so the writer abandons any copy that stopped short: one that ended before the length
  the response promised, and one whose download was paused or cancelled, which ends the copy as the end of the
  response does (W4, W5).
- **Measure from the phone's own loopback**, never over Wi-Fi: the A20's link ran at 0.4 MB/s (W4, *Rejected*).
- **A script fails loudly.** A step that did not happen stops the run with a message; it never leaves a line out of
  the results or carries on under the wrong label.

## Traps already paid for

- The debug build died of StrictMode violations on the G6, in LG's framework and in Android Components, before any of
  the spike's code ran. A debug build logs them and is never killed (patches 0004, 0006; W4, flag 2).
- `InetAddress.getLoopbackAddress()` is `::1` on Android, so Gecko was refused at `127.0.0.1` (patch 0005).
- The control run of `check-plaintext` found nothing at first: the page was cacheable, so a run loaded the first run's
  marker, and ten seconds after going home a normal session's stores were not on disk yet (W4).
- `am start -W` waits forever on the gate, which finishes without drawing.
- Under `set -u`, dash stops an `EXIT` trap at an unset variable, and nothing after it runs: `check-plaintext` left
  its temporary directory and its `adb reverse` behind whenever it ended before its server started (2026-11-15).
- A pause or a cancel ends Android Components' copy loop just as the end of the response does, and the writer sees
  only that the block returned. Where the response gave no length, a stopped download was saved into the vault cut
  short; and *Try again* from the notification asked for the rest, which the vault cannot take (W5, patch 0008).
- An exception uncaught on one of the loopback server's threads kills the app. A download paused or cancelled closes
  its end of the socket, which the server's write throws on (W5, patch 0007).
- A setting saved with `apply()` is lost if the app is stopped a moment later. The gate's private setting was, on the
  S9, so the control session ran privately and found nothing (W5, patch 0012).
- A session outlives a lock: a held one is asked before it is trusted (W5, patch 0010).
- **Never lock a vault you cannot open again.** The G6's, locked to measure patch 0010, refused the passphrase on file
  though the keys typed were the file's; it waits for its owner. The A20's is passphrase-only (W5).
- LG's framework logs every key event on the G6, so a passphrase typed there over adb is in its log until `logcat -c`.

## Measured, not assumed

- **A private session left no page's marker on disk** on the G6 (W4), the S9 and the A20 (W5), where a normal one left
  it in 10, 9 and 11 files. A private tab's full-screen JPEG stays in
  `no_backup/mozac_browser_thumbnails/private_thumbnails/` until the next start, on all three.
- **64 MiB into the vault through Gecko**: 1.73–1.83 s on the G6, 0.99–1.13 s on the S9 and 1.84–2.29 s on the A20,
  against 0.56–0.73 s, 0.37–0.43 s and 0.76–1.35 s through Fenix's own writer (W4, W5). The vault's copy is as fast
  through Gecko as from memory on each, so encri's pipe sets the pace.
- **A download stopped part way leaves nothing in the vault, and *Try again* brings it whole**, with a length and
  without, on the G6, the S9 and the A20 (`check-stopped`, W5).
- A clean build took about 8 minutes after the artifact build's 6; a change to Fenix, about 10 seconds (W4). It costs
  about 20 GB of disk: the checkout, the objdir, `~/.gradle` and `~/.mozbuild` (W4's table).
- **GeckoView reads an explicit `configFilePath` whether or not the app is debuggable**: `GeckoRuntime` checks
  debuggability only to pick the default under `/data/local/tmp` (read in `GeckoRuntime.java` at Firefox 160,
  2026-11-15). Not measured in a release build.
- **Lockstore is in the published GeckoView 160**: its three prefs, `LockstoreStorageKeyProvider` and the obfuscating
  VFS are in the arm64 `libxul` (W5). Whether the prefs work there is unmeasured.

## Decisions

| | Why it matters here |
|---|---|
| W1 | Stage 1 first, against a published GeckoView; the throwaway package (flag 1) |
| W2 | MPL-2.0 for every file; Exhibit A's notice on a new one (flag 1) |
| W4 | What the spike measured on the G6, and what stage 1 has to do that it found |
| W5 | A stopped download into the vault, and what the A20 and the S9 measured |
| encri-api L11, L12 | `write` saves only a finished writer; a writer's failure is told encri's reason |
| encri D120, D145 | A grant is keyed on the package for good |

## Open here

- How a release sets Gecko prefs, how stage 1 stops private thumbnails, and how the fork is carried
  (`docs/decisions/open-questions.md`).
- The S24's run waits for its owner's word.
- Patch 0010's locked path, on the G6 once its owner has unlocked it (`ROADMAP.md`).
- A stopped download into the vault shows as failed, a cancelled one too (W5, flag 2).

## Read next

- `scripts/spike/stage-1/README.md`; `docs/feasibility-study.md`, *Scenario 1*.
- Full records: `docs/decisions/INDEX.md`; `../encri-api/docs/decisions/L011-*.md` and `L012-*.md`.
