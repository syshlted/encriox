---
paths:
  - "scripts/spike/**"
  - "LICENSE"
---

# The spikes — component brief

`scripts/spike/stage-1/`: Fenix at a Firefox release, built with Gradle against the GeckoView Mozilla published, with
the patches in `patches/` on top, and the scripts that fetch, build and measure it on the phones (W1, W4). It is a
throwaway, and nothing it builds ships. Its `README.md` says how to build it and run it. `scripts/spike/stage-2/` runs
stage 1's app with Lockstore's switches on, and checks what they cover and whether a profile opens again (W12). It
builds its own Gecko, from stage 1's checkout with `patches/` of its own on top, on the drive for large builds (W13),
in which the profile's keys are kept under a secret from the vault's key, and checks what each store does when that
key is locked (W14), and which does not start without the vault (W15).
`LICENSE` is here too until a build of encriox itself has a brief. This brief is level 1; full records are in
`docs/decisions/`.

## Rules that hold here

- **The package is `com.systemhalted.encri.browser.spike`, on test phones alone, never published** (W1, flag 1). The
  phones are the ones holding encri's test vaults; the S24 only with its owner's word (`AGENTS.md`, *Hazards*).
- **The patches are the source.** Change them by committing on the checkout's `encriox-spike-stage-1` branch, or
  stage 2's on `encriox-spike-stage-2` over it, and writing them out again with `git format-patch` (each stage's
  `README.md`). Never edit a `.patch` by hand. A fix is a patch of its own on top, so that what each measurement ran is
  a prefix of the series.
- **Mozilla's toolchain, not the suite's.** Gradle runs on Mozilla's JDK 17 and its SDK under `~/.mozbuild`
  (`README.md`), so the suite's JDK 21 and SDK, which are encri's toolchain, are left as they are. Stage 2's mach state,
  with the clang, NDK and JDK it bootstraps and Mozilla's CI Rust, is beside its checkout on the drive for large builds
  (W13, flag 2). Both builds include `../encri-api` from source, so it is still one Gradle invocation at a time in the
  whole suite.
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
- **Stage 2's build encrypts every profile unless its config says otherwise** (W13, flag 1), and stage 1's scripts run
  with stage 1's config. After a round of stage 2, each phone gets a fresh profile, stage 1's APK and stage 1's config
  again (W13, flag 3).
- **A key that is there and locked is kept, never replaced** (W14). A store reads its key under whichever KEK wraps
  it; one it cannot unwrap because the KEK is locked fails its open as access denied, which no consumer takes for
  corruption. Lockstore reports a locked key and a missing one alike, so a store tells them apart by asking whether
  the key exists. Deleting a key truncates Lockstore's write-ahead log, or the key is still on disk.
- **Stage 2's Gecko waits at startup for the app's answer**, on Gecko's own thread: the secret, an HMAC of the vault's
  key (W14). The app's main thread is never held, because encri-client's binding answers there. With the vault
  locked, encri's unlock comes up, and if the vault stays shut the app ends before Gecko opens anything of the
  profile (W15). Only the gate's `kek withhold` starts Gecko locked. The gate's `kek` setting is read by the next
  start, and the gate's own start may already have read the old one: set it, then stop the app, before a fresh
  profile or a measured start.
- **A test vault whose passphrase is on file may be locked**, and wiped if it does not open again (W15, the user's
  word); `check-unlock` types the passphrase into encri's screen and never prints it. Not the G6's, which does not
  open and would need its owner's screen lock again, nor the S24's.
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
  prints each origin's flags. Stage 2's patch 0001 fixes it (W13). A persistent origin, such as `chrome`, is always
  recorded plaintext: QuotaManager encrypts best-effort origins only.
- **Gecko's launch guard reads the SQLite switch before any pref is loaded**, in `XRE_mainStartup`, before XPCOM: it
  sees the build's default and nothing else. With the switch unlocked and turned on by the config, the gate's start
  encrypted the S9's profile and the marker page's start was refused (W13). Stage 2's patch 0004 turns it on by
  default.
- Fenix's own Gradle build never takes a local GeckoView: `:geckoview` is a project only of the checkout's root build,
  so stage 2 builds `fenix:assembleDebug` there (W13, patch 0003).
- Configure bootstraps Rust only in Mozilla's automation, and the machine has none: "Rust compiler not found". Stage
  2's `build` fetches Mozilla's `linux64-rust-android` (W13). Configure also bootstraps its own JDK into mach's state
  and uses it, whatever `JAVA_HOME` says.
- mach keeps the build ID across incremental builds, so `compatibility.ini`'s `LastVersion` tells stage 2's Gecko from
  the published one, not one of stage 2's builds from another. An incremental package grew the APK from 170 MB to
  239 MB, with the same `libxul.so`.
- Copying the whole profile off the A20 over Wi-Fi took more than ten minutes, so `check-lockstore` reads each file's
  first 24 bytes on the phone. The G6's toybox `dd` refuses `conv=notrunc`, so a byte is changed by writing the whole
  file through `run-as … sh -c 'cat > …'`.
- **Lockstore's stores asked for their keys under the KEK they made them under, by name** (W14). Moved under the
  profile's password KEK, they could not be found: every SQLite open failed, the cookie store moved `cookies.sqlite`
  aside as corrupt, and the HTTP cache deleted its key and made one again under a LocalKey. Stage 2's patch 0007.
- **The cookie store moves its database aside for any open failure** but `NS_ERROR_FILE_ACCESS_DENIED` and a full
  disk, and Lockstore's `NS_ERROR_NOT_AVAILABLE` reaches it as `NS_ERROR_FAILURE`. The HTTP cache, with no key, would
  truncate an entry it reads and its index rebuild deletes one: a locked session leaves the disk cache alone (W14).
- **A deleted LocalKey stayed in `lockstore.keys.sqlite-wal`**, `kek_bytes` and all, 83 copies on the S9, though the
  database held none: SQLite's log keeps each page as it was written (W14, patch 0009). `check-kek` scans both files.
- **A start of the gate starts Gecko**, which asks for the secret before the gate has set `kek`. On the A20 it also met
  the profile stage 1's build had left, which Gecko's launch guard refused, and Fenix died of the shut-down runtime: a
  fresh profile comes before the first start (W14).
- A GeckoResult answers on the thread that asked, and a thread with no Looper cannot ask: the app follows the vault's
  key on the main thread (patch 0006).
- encri's screens refuse a screenshot, which comes back empty; `uiautomator dump` reads them. On the G6 a tap meant
  for encri's passphrase button reached *Unlock with fingerprint or screen lock*, which brought up the owner's
  pattern: Back out of it (W15).
- `monkey -p <package>` starts any of the app's launcher entries, and the debug build has two: it started
  LeakCanary's. `check-unlock` starts `.App`, the browser's own (W15).
- The A20's keyguard came back between two steps, and a page behind it never opened: `wake` retries until it has gone
  (W15).
- A URL with `&` given to `adb shell am start -d` is cut at the `&` by the phone's shell, which runs the rest as a
  command: quote it again (2026-11-18).
- **Local storage flushes a page's writes 5 seconds later**, and an origin's first flush makes its database, whose key
  cannot be made while the profile's KEK is locked: a lock in between lost them (W15, *Bugs* in `ROADMAP.md`).
- After an install of the 239 MB APK, the A20 answered slowly for some minutes: the vault in 9.7 s, near the app's 10 s
  limit, and a page did not open in two minutes, where the run before had taken 1.8 s and opened every page (W14).

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
- **With stage 2's Gecko all three switches work** on the G6, the S9 and the A20 (W13): every database in Gecko's
  profile encrypted but NSS's `cert9.db` and `key4.db` and Lockstore's `lockstore.keys.sqlite`; the test origin
  recorded `0x17`, `0x17` and `0x13`; all six stores read back at the next start; and the marker in 4 files, Fenix's
  saved tabs, its history and the history's WAL, and `serviceworker.txt`.
- **The vault's key keeps the profile's keys** with stage 2's patches 5 to 9 (W14), on the S9 and the A20: the profile
  opened again with all six stores and no key under a LocalKey; started without the secret, it read only the service
  worker, kept every file and key, and read all six back at the next start with it; locked mid-session, a new origin's
  cookie and HTTP cache entry were written and kept, and its site storage failed; the vault's key dropped and read again
  locked and unlocked Gecko's keys through the app. A profile made without the secret was moved under it at the next
  start, in 326–396 ms on the S9 and 757 ms on the A20, leaving no LocalKey in Lockstore's database or its log. Gecko
  waited for the vault's answer 0.64–0.71 s on the S9 and 1.7–1.8 s on the A20, and the PBKDF2 unlock took 250–270 ms
  and 530–630 ms. On the G6, its vault locked, the app knew so after 1.9 s and the profile was made in the clear.
- **A start with the vault locked brings up encri's unlock, and a refusal ends the browser** (W15): on the A20, the
  unlock came up before Gecko opened anything of the profile, and only its startup caches and an empty
  `.startup-incomplete` changed while it waited; a wrong passphrase left encri's screen up, and Back ended the browser,
  out of the recent apps, with the profile as it was; on the G6 too. A real lock of the A20's vault locked Gecko's
  keys as the gate's `drop` had, and the passphrase typed into encri's screen unlocked them.
- **Stage 2's first Gecko build took 76 minutes**, 69 of them `mach build` at 86% of 8 cores, and a change to one pref
  8½. Its objdir holds 11 GB and its mach state 6.6 GB, on the drive for large builds; the checkout's clone took 14
  minutes, for 5.9 GB (W13).

## Decisions

| | Why it matters here |
|---|---|
| W1 | Stage 1 first, against a published GeckoView; the throwaway package (flag 1) |
| W2 | MPL-2.0 for every file; Exhibit A's notice on a new one (flag 1) |
| W4 | What the spike measured on the G6, and what stage 1 has to do that it found |
| W5 | A stopped download into the vault, and what the A20 and the S9 measured |
| W6 | The browser's own key in the vault, sealing the logins' and the cards' keys; private thumbnails in memory |
| W12 | Stage 2's first round: what Lockstore's switches do at 160, and a fresh profile for each run (flag 1) |
| W13 | Stage 2's own Gecko: QuotaManager fixed, SQLite on by default, its toolchain, and the phones back on stage 1 |
| W14 | Stage 2's second round: the profile's keys under the vault's key, the stores following it, and what a lock does |
| W15 | No start without the vault: encri's unlock at startup, a refusal ends the browser; a real lock measured |
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
- Stage 2's third round, what `obfsvfs` costs, on stage 2's Gecko (W13); W13's three flags and W15's four.
- What a lock should do, which the lost local storage bears on (W15).

## Read next

- `scripts/spike/stage-1/README.md`; `docs/feasibility-study.md`, *Scenario 1*.
- Full records: `docs/decisions/INDEX.md`; `../encri-api/docs/decisions/L011-*.md` and `L012-*.md`.
