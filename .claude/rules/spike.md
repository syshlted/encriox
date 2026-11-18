---
paths:
  - "scripts/spike/**"
  - "LICENSE"
---

# The spikes — component brief

`scripts/spike/stage-1/`: Fenix at a Firefox release, built with Gradle against the GeckoView Mozilla published, with
the patches in `patches/` on top, and the scripts that fetch, build and measure it on the phones (W1, W4). It is a
throwaway, and nothing it builds ships. Its `README.md` says how to build it and run it. `scripts/spike/stage-2/` runs
stage 1's build with Lockstore's switches on, and checks what they cover and whether a profile opens again (W12).
`LICENSE` is here too until a build of encriox itself has a brief. This brief is level 1; full records are in
`docs/decisions/`.

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
- **The browser's key is never written over, and nothing answers "none" without it** (W6). `encriox.key` is read,
  or made where there is none and read back; one of the wrong length, two of the name, or one that does not read back
  are refused. A store of keys that cannot be read yet waits in `awaitReady()` or throws, never answers that nothing
  is stored: a key manager told its key is gone makes a new one, and what the old one sealed is wiped. Only a value
  the vault's key cannot open is answered as gone, as Fenix answers one Keystore cannot (W6, flag 2).
- **A profile's SQLite switch is set when the profile is made**, so stage 2 starts each run from `fresh-profile`, and
  going back to stage 1 does too. `check-plaintext` refuses a config whose SQLite switch disagrees with the profile.
- **A profile is checked by starting it again**, not by the marker search alone: site storage held no marker and still
  could not be read at the next start (W12). `check-lockstore` always reads back in a second process.
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
- Android Components' logins and autofill stores waited on themselves for good once their key was lost, or new over an
  existing logins database: recovery went through `getStorage()`, which awaits the deferred that was recovering. The
  S9's old canaries set it off when the keys moved to the vault's (W6, patch 0018).
- `am start -W` waits for good while the screen is off behind the lock screen: `check-plaintext` did on the A20 (W6).
  Its lock screen is not secure, so `input keyevent KEYCODE_WAKEUP` then `wm dismiss-keyguard` wakes it.
- SharedPreferences' XML writes `"` as `&quot;`, so a search for a key in the clear looks for both (`check-keys`).
- **Never lock a vault you cannot open again.** The G6's, locked to measure patch 0010, refused the passphrase on file
  though the keys typed were the file's; it waits for its owner. The A20's is passphrase-only (W5).
- LG's framework logs every key event on the G6, so a passphrase typed there over adb is in its log until `logcat -c`.
- **Lockstore's SQLite switch is locked off** at 160 (`all.js`), and a default pref from the config cannot change it.
  Nothing says so: the switch is simply off (W12).
- Gecko refuses a profile whose SQLite encryption disagrees with the switch, and on Android only logs "Profile
  encryption state does not match launching build." and stops: the page never draws.
- QuotaManager can record an encrypted origin as plaintext, flags `0x7` in its `.metadata-v2` where `0x10` is the
  encrypted bit, and then cannot read its site storage at the next start, nor any site's (W12). `check-lockstore`
  prints each origin's flags.
- Copying the whole profile off the A20 over Wi-Fi took more than ten minutes, so `check-lockstore` reads each file's
  first 24 bytes on the phone. The G6's toybox `dd` refuses `conv=notrunc`, so a byte is changed by writing the whole
  file through `run-as … sh -c 'cat > …'`.

## Measured, not assumed

- **A private session left no page's marker on disk** on the G6 (W4), the S9 and the A20 (W5), where a normal one left
  it in 10, 9 and 11 files. A private tab's full-screen JPEG stayed in
  `no_backup/mozac_browser_thumbnails/private_thumbnails/` until the next start, on all three, until patch 0013 kept
  it in memory: none on disk since, on all three (W6).
- **The logins' and the cards' keys are sealed with the vault's key** (`check-keys`, W6): no marker and no key in the
  clear on the S9 and the A20, and logins asked for before the vault was open arrived after its key, 0.4 s later on the
  S9 and 1.8 s on the A20. On the G6, with its vault locked, they waited more than a minute and nothing was written.
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
  VFS are in the arm64 `libxul` (W5). With the switches on, a normal session's marker was in 4 or 5 files, not 9 to
  11: Fenix's saved tabs and history, the cookies' WAL and `serviceworker.txt` (W12). The HTTP cache's switch held,
  and its entries came back after a restart, on all three. Site storage's encrypted, and on the S9 and the G6 its
  origin was recorded as plaintext, flags `0x7`, so the next start could not read it; the A20's was recorded `0x13`
  and read back. The G6's flags set to `0x17` by hand read back all six stores. SQLite's switch took no effect.

## Decisions

| | Why it matters here |
|---|---|
| W1 | Stage 1 first, against a published GeckoView; the throwaway package (flag 1) |
| W2 | MPL-2.0 for every file; Exhibit A's notice on a new one (flag 1) |
| W4 | What the spike measured on the G6, and what stage 1 has to do that it found |
| W5 | A stopped download into the vault, and what the A20 and the S9 measured |
| W6 | The browser's own key in the vault, sealing the logins' and the cards' keys; private thumbnails in memory |
| W12 | Stage 2's first round: what Lockstore's switches do at 160, and a fresh profile for each run (flag 1) |
| encri-api L11, L12 | `write` saves only a finished writer; a writer's failure is told encri's reason |
| encri D120, D145 | A grant is keyed on the package for good |

## Open here

- How a release sets Gecko prefs, how the fork is carried, and what a lock does, which decides whether the stores' keys
  may stay in memory after one (`docs/decisions/open-questions.md`; W6, flag 3).
- The S24's run waits for its owner's word: `check-plaintext`, `measure`, `check-stopped` and `check-keys`.
- W6's three flags: the key as a document in the vault, an unopenable key taken as lost, and the stores' keys left in
  memory at a lock.
- Patch 0010's locked path, on the G6 once its owner has unlocked it (`ROADMAP.md`).
- A stopped download into the vault shows as failed, a cancelled one too (W5, flag 2).
- Stage 2's next two rounds, a key from the vault and what `obfsvfs` costs, wait on a Gecko build or on Mozilla (W12):
  the user's call.

## Read next

- `scripts/spike/stage-1/README.md`; `docs/feasibility-study.md`, *Scenario 1*.
- Full records: `docs/decisions/INDEX.md`; `../encri-api/docs/decisions/L011-*.md` and `L012-*.md`.
