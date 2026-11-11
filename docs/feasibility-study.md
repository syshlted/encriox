# A browser for encri: which of three scenarios is feasible

*2026-11-10 · a study, not a decision. Nothing here is accepted until the user picks a scenario. The pick would be
this repository's first decision record, as the sibling repositories keep them.*

Encri's records are cited as *encri D26*, the contract's as *encri-api L11*. Browser facts carry a source link in
[Sources](#sources); anything marked *unverified* was not checked against a primary source.

## The three scenarios

1. **Firefox + encri-api, open source.** A fork of Firefox for Android keeps its cache and storage (pages, images,
   stylesheets, cookies, history, extensions, downloads) in the encri vault, or has encri encrypt it before it reaches
   persistent storage. It reaches encri through encri-api alone, so it can be open source.
2. **Chromium + encri-api, open source.** The same, on a fork of Chromium for Android.
3. **Encri Core inside Chromium, closed.** Chromium carries encri's own code. The browser has a vault of its own,
   separate from encri's, and opening the browser unlocks it. Encri's code is closed (encri D116), so the browser is
   too, under a proprietary licence.

## Verdict

1. **Scenario 1, Firefox through encri-api, is the most feasible.**
   - Its first stage builds with Gradle against Mozilla's own GeckoView. That stage already puts downloads in the vault
     and keeps Gecko's live stores off disk in plaintext.
   - Its second stage builds on at-rest encryption Mozilla is writing (*Lockstore*). The fork's job there is to make
     Lockstore's key come from the vault.
   - It needs no change to encri or the contract.
2. **Scenario 2, Chromium through encri-api, can be built.** Every step is heavier:
   - a full Chromium build for any change;
   - at-rest encryption the fork writes and then rebases every two weeks;
   - no password store and no extensions on phones.
3. **Scenario 3, Encri Core inside Chromium, is the least feasible.** It is everything scenario 2 costs, plus a JNI
   bridge, a second vault and LGPL obligations. It also undoes the separation encri is built on: keys in a process
   with no network access (encri D70).

In none of the three can the browser's storage literally live *inside* the vault. Downloads, extension packages and
session snapshots can. Cookies, history, site storage and the cache cannot: they need to be memory-only, or encrypted
in the browser with a key that only the unlocked vault can produce.

## What decides it: a browser keeps two kinds of data

| Kind | What it is | How it is written | Fits encri-api today |
|---|---|---|---|
| **Artifacts** | downloads, extension packages, tab and session snapshots, thumbnails and icons, prefs, a bookmarks export | once, whole, front to back | **yes**: `openWrite`'s pipe, `openRead`'s descriptor |
| **Live stores** | cookies, history and bookmarks, site storage (IndexedDB, localStorage, Cache API, service workers), the HTTP cache, permissions | in place, page by page, with `fsync`, many times a second | **no** |

The artifacts are a straightforward fit. A download is the best case: a stream written once, which `openWrite` was
built for, and it lands in the vault rather than in `Download/`.

The live stores do not fit, and the reasons are encri's format, not just its IPC:

1. **A committed container is never changed in place; a write is a new container** (encri D26, the `core-container`
   brief). Changing one row of a 30 MB history database would mean writing 30 MB. This holds inside encri's own
   process too, so it applies to scenario 3 as well.
2. **A write is a pipe that commits on a clean close, and a lock abandons it** (encri-api L11, encri D120). SQLite and
   LevelDB need positioned writes, a journal they can roll back, and an `fsync` that means the bytes are durable.
3. **Every read decrypts a 64 KiB block behind AppFuse**: 1.5 ms a block on the S9 and 5.3 ms on the G6, decrypt
   included (encri-api L14, encri D40). A SQLite page is 4 KiB and read at random, and encri keeps one decrypted block
   per open document (encri D23). Expect most of a block's cost for each page that misses. This is reasoned from those
   figures and has not been measured.
4. **A session allows at most four writes at a time, and a listing page holds at most 500 entries** (encri D120). A
   disk cache holds thousands of entries and writes many at once.
5. **The vault can lock under the browser at any moment**, and any companion can call `lock()`. A store that cannot
   write mid-transaction must survive the abort.

So **in every scenario the live stores need one of two things**:

- **memory only**: they never reach disk, as in private browsing, and are gone when the browser closes or the vault
  locks; or
- **encryption inside the browser**, with a key that is reachable only while the vault is unlocked.

Both mean changing the browser engine, except where the engine already has the mode. That engine work, not
encri-api, is the bulk of the project, and how much of it upstream has already done is what separates the scenarios.

### "Encrypted by encri before it is saved": a vault-gated key instead

Scenarios 1 and 2 allow for encri encrypting the data itself. As a service over Binder that is the wrong shape. Each
page read or written costs a round trip, 46 µs on the S9 to 767 µs on the G6 with the screen off (encri-api L14),
before any crypto. It also needs contract surface that does not exist, and it protects nothing more: the browser sees
every byte of plaintext either way (encri's hard constraint 2).

**What works with today's contract** is a key the vault holds:

1. On first run the browser generates a random 256-bit key and writes it with `openWrite` into its own folder in the
   vault, the folder its grant is rooted at.
2. After every unlock it reads the key with `openRead`, holds it in memory only, and uses it to encrypt its live stores
   in its own process.
3. On `STATE_LOCKED` or `STATE_ENDED` it wipes the key and closes every tab. Without the vault it cannot get the key
   back.

*Keys never leave encri* still holds, because this is the browser's key, not one of encri's: it is the content of a
document, which encri hands to a granted app by design. It also needs no contract change. Its limits are real but
shared with every scenario, scenario 3 included: the key sits in the browser's memory while it runs, and wiping it at
a lock is the browser's own behaviour, not something encri enforces. Its data stays outside the vault, so encri's
backups carry the key but not the stores. A store can still be exported into the vault as an artifact.

## Scenario 1 — Firefox + encri-api, open source

**Feasible, in two stages, and the engine work it needs is largely being written by Mozilla already.**

### What Firefox for Android is made of

All of it is in `firefox-main` under `mobile/android/`. The UI is Fenix, in Kotlin, on Android Components (A-C), also
Kotlin. The engine is GeckoView, in C++ and Rust. History, bookmarks, logins and autofill come from
application-services, in Rust, vendored at `third_party/application-services`. [F1]

Where each store lives on disk today [F2]:

| Store | Owner | Where | At rest |
|---|---|---|---|
| cookies, permissions, site storage (`storage/`), extensions (`extensions/`) | Gecko | `filesDir/mozilla/<profile>/` | plaintext |
| HTTP cache (`cache2`) | Gecko | `cacheDir/<profile>/cache2` | plaintext |
| history and bookmarks (`places.sqlite`) | application-services | `filesDir/` | plaintext |
| logins, autofill | application-services | `databases/` | sensitive fields encrypted, **but release Fenix keeps that key in plaintext `SharedPreferences`** (`forceInsecure = !Config.channel.isDebug`) |
| session and tab state, recently closed tabs | A-C | `filesDir/`, written whole with `AtomicFile` | plaintext |
| thumbnails, icons | A-C | `noBackupFilesDir/` | plaintext |
| downloads | A-C | public `Download/` through MediaStore | plaintext, readable by other apps |

Every row is an artifact or a live store as defined above. The artifacts sit in the Kotlin layers, and so does the
code that writes them.

### Stage 1: no Gecko build

A Kotlin-only change builds against a GeckoView Mozilla has already built. That can be an artifact build, which
downloads CI's native code for a revision CI has built, or a published AAR pinned from maven.mozilla.org, which is
how Iceraven builds. [F1] Within that limit:

- **Downloads go into the vault.** `openWrite` into a downloads folder under the browser's grant, instead of
  MediaStore. This is the best fit in the whole study.
- **Gecko's live stores stay off disk in plaintext**, because every tab runs in private mode
  (`GeckoSessionSettings.usePrivateMode`) with `browser.privatebrowsing.forceMediaMemoryCache` set, as Tor Browser
  sets it. In private mode:
  - the HTTP cache is memory-only;
  - IndexedDB, the Cache API and service-worker scripts are written encrypted, with keys kept in RAM, and deleted when
    the private session ends [F3];
  - cookies stay in memory *(unverified on Android)*.

  This is Tor Browser's *disk avoidance*, and Tor Browser for Android is built this way. [F8]
- **Bookmarks, open tabs and, if wanted, history are artifacts.** They are written whole into the vault when the
  browser goes to the background or the vault locks, and read back at unlock.
- **The browser opens behind `Encri.unlockIntent()`**, and on `STATE_LOCKED` it closes every tab.
- **Things to remove, as Encri Images removed Glide's disk cache (encri D145):** private thumbnails, which A-C writes to
  `private_thumbnails` and clears only at the next start; the plaintext `places.sqlite`, so history is kept as an
  artifact or not at all; and the logins key in `SharedPreferences`.

**What stage 1 gives up:** a site login, which is a cookie, does not survive closing the browser. The cache starts
cold after every unlock. Those two are what stage 2 is for.

### Stage 2: Gecko's own encryption, with its key gated by the vault

Mozilla's *Lockstore* work, "[meta] Encrypt sensitive storage", is assigned and landing piece by piece. [F4] Each
piece is behind a pref that is off by default:

- **SQLite** (`security.storage.encryption.sqlite.enabled`, bug 1996558, fixed). Every database under the profile is
  opened through `ObfuscatingVFS` with a key of its own (`storage/SQLiteEncryption.h`, read 2026-11-10).
- **The HTTP cache** (`browser.cache.disk.encryption.enabled`, bug 2032032, fixed).
- **IndexedDB and the Cache API outside private browsing** (`dom.quotaManager.encryption.enabled`, through
  `LockstoreStorageKeyProvider`).
- **TLS session tickets** (bug 2032036, fixed).

Each per-database key is wrapped by a key-encryption key. The KEK types are `LocalKey`, `Pkcs11Token` and `Password`
(`lockstore_rs/src/lib.rs`, read 2026-11-10). The default, `LocalKey`, stores its bytes in plaintext next to the data,
so today the encryption protects nothing against someone who holds the profile. **That is the hook for encri.** The
fork can supply the KEK from the vault, either:

- as a new KEK type whose bytes are the vault-gated key, or
- *(unverified, cheapest)* by unlocking a `Password` KEK with that key at unlock and locking it again at
  `STATE_LOCKED`. Lockstore already reports a locked KEK as unavailable.

Either way, the encryption code stays Mozilla's to maintain, and the fork carries a small patch plus prefs.

**Gaps stage 2 still has to close itself:**

- application-services keeps its own SQLite, so `places.sqlite` and the login key are outside Lockstore. The fork
  has to bring them in, which is unbuilt and unmeasured.
- extension packages (`.xpi`) are plain files in the profile.

**Risks:**

- **It is unfinished upstream.** The SQLite pref is labelled *"INTERNAL — DO NOT ENABLE … will corrupt profiles"*, and
  bug 2087751 records a profile that would not open a second time.
- **It is untested on Android.** Lockstore's own gtests are disabled there (`security/lockstore/moz.build`). Stage 2
  starts by measuring it on the four phones.
- **The fork is an early adopter**, and Mozilla decides the design.

### What carrying the fork costs

- **The cadence.** Firefox has shipped a major release every two weeks since Fx155 on 2026-09-01, Android included.
  Tracking release means about 26 rebases a year. [F6]
- **The ESR alternative.**
  - ESR 153 runs to September 2027, and its branch contains Fenix, A-C and GeckoView.
  - It has Lockstore's SQLite encryption but not `LockstoreStorageKeyProvider`.
  - Tor Browser for Android has built on ESR since 12.0, but Mozilla ships no Android ESR. Tor's team backports the
    Android security fixes by hand in every release, and that work would become this project's. [F6, F8]
- **The build.**
  - Stage 1 is a Gradle build.
  - Stage 2 builds GeckoView on Linux: at least 30 GB of disk and 8 GB of RAM by Mozilla's guide; IronFox budgets
    50 GB per variant. [F1]
  - IronFox and Fennec F-Droid each build everything from source with one or two maintainers. [F1]
- **The licence.**
  - Files the fork modifies stay MPL-2.0, and their source must be published.
  - encri-client is Apache-2.0, as Glimpse's code is for Encri Images.
  - **The name cannot contain "Firefox"**: Mozilla's trademark policy requires the fork's own branding and a statement
    that it is not affiliated. `encri-firefox` is fine as a repository name and not as an app name. [F7]
- **Extensions** come from AMO with no extra work. Firefox for Android has had the open extension ecosystem since
  December 2023. [F5]

### Encri's side

- **It is an ordinary companion**, as Encri Images is (encri D145), and needs no contract change for either stage.
- **It asks for read, write, create and delete on one folder**, never the whole vault. It has `INTERNET`, and encri
  D3's reasoning about Tier C applies: an app with network access cannot be contained, so its grant is kept small.
- **It needs a registry entry.** A grant lapses with every update the registry has not reviewed (encri D120 flag 1,
  D130). At a two-week cadence the user would otherwise consent again twice a month.

## Scenario 2 — Chromium + encri-api, open source

**Feasible in principle: the same architecture as scenario 1, with more of the work at every step falling on this
project.**

### Where each store lives

Profile files live in the app's data directory, and the HTTP cache under Android's cache directory. [C3]

| Store | Format | At rest on Android |
|---|---|---|
| History, Web Data, Favicons, Shortcuts; tab state, moving to `Tabs/TabDB` | SQLite | plaintext |
| IndexedDB on disk (its SQLite backend is used in memory only), Local and Session Storage, Service Worker, `shared_proto_db` | LevelDB | plaintext |
| HTTP cache, Code Cache, GPUCache, Cache Storage | simple disk cache | plaintext unless an enterprise policy turns on its encryption |
| Bookmarks, Preferences | JSON, written whole by `ImportantFileWriter` | plaintext |
| cookies | SQLite | values encrypted under OSCrypt's Android key: AES-128-CBC with a key derived from the constant `"peanuts"`, salt `"saltysalt"`, one PBKDF2 iteration (`posix_key_provider.cc`, read 2026-11-10). This is obfuscation, not secrecy |
| downloads | public `Download/` via MediaStore | plaintext, readable by other apps |
| passwords | **none** | `Login Data` has not been compiled on Android since September 2025. Passwords live in Google Play services, which only Google's own builds can reach, so **a public fork has no password store** |

### What carrying it costs

- **No Gradle-only stage.**
  - Every change, the Java UI included, is built from a full Chromium checkout. That needs an x86-64 Linux host, at
    least 100 GB free, and more than 16 GB of RAM recommended. Plan on 200–300 GB and hours per clean build
    *(unverified estimate)*. [C1]
  - I found no counterpart to Mozilla's artifact build.
- **A memory-only mode exists but still needs that build.** Incognito already keeps the cache and site storage in
  memory. [C4] Incognito tab state is encrypted, but its own documentation calls the scheme *"not cryptographically
  sound"*.
- **No upstream persistent encryption to ride on.** There are two useful seams:
  - OSCryptAsync's `KeyProvider` can be swapped and has a `kTemporarilyUnavailable` answer. That is the place for the
    vault-gated key, and it would give cookies a real key.
  - The enterprise cache encryption (`services/network/enterprise/encryption/`) appears to draw its key through
    OSCrypt. That is inferred from its file names.

  Everything else is new code the fork writes and then carries:
  - an encrypting SQLite VFS. `sql::` already routes through a wrapper VFS, but there is no SQLCipher in the tree, and
    mmap, which is on by default, has to go;
  - encryption for LevelDB, under `ChromiumEnv` / `FilesystemProxy`;
  - encryption for the JSON files and the Java-side stores. [C4]
- **A lock is dangerous here.** Many Chromium stores treat an I/O error as corruption and *raze* the database. A key
  that goes away under a running browser could wipe data, so the browser has to stop before it drops the key.
- **The cadence.**
  - Since Chrome 153 on 2026-09-08 a milestone ships every two weeks, Android included, and stable gets a respin every
    week.
  - Extended Stable is for managed Windows and Mac only. There is no slower Android branch. [C1]
- **What forks show.**
  - Vanadium ships within hours of Google with the GrapheneOS team behind it.
  - Cromite, essentially one maintainer, has been four milestones behind since September.
  - Bromite, Kiwi and Mulch are gone. [C2]
- **Extensions**: none on phones. `is_desktop_android` compiles them in for Googlebooks, but the phone UI is
  untested. The "downloaded extensions" in the scenarios' list barely exist here. [C6]
- **Kotlin.** Chromium's Android code is overwhelmingly Java. encri-client's `suspend` API needs a thin in-tree Kotlin
  adapter that offers callbacks or `ListenableFuture`. kotlinx-coroutines is already in `third_party/android_deps`.
  This is minor. [C5]
- **The licence and the name.** An open fork is unproblematic under BSD-3. "Chrome" and "Chromium" are both Google
  trademarks, so the product needs its own name. [C7]

**What it has over Firefox:** Blink's web compatibility, and the weight of the platform most sites test against.

## Scenario 3 — Encri Core inside Chromium, closed

**Legally possible, the costliest technically, and it works against encri's own design.**

- **"Closed" is narrower than it sounds.**
  - Blink ships LGPL-2 files, and release builds link ffmpeg, under LGPL-2.1, statically.
  - A closed fork still publishes its changes to LGPL and MPL files, allows relinking (LGPL-2.1 §6), and ships a
    credits page. In practice that is Vivaldi's model: publish every Chromium change, and keep only the proprietary
    code closed, in a library of its own. [C7]
  - MPL-2.0 permits the same split, so the licence is not what keeps this scenario off Firefox. [F7]
- **Encri Core does not bring what a browser needs most.**
  - What it brings: the key hierarchy, the key-slot table, Argon2id, Keystore gating, biometric unlock and the unlock
    screen.
  - What it does not bring is a store for live data. Its container is write-once (encri D26), and its code is
    JVM-only, while Chromium's storage runs as C++ in the browser process.
  - So the storage encryption is scenario 2's new C++ work, unchanged, with encri's code reached over JNI to hand over a
    key at unlock. That is what the vault-gated key already gives scenario 2 through encri-api, with no closed code.
- **It puts the keys where encri refuses to.** On Android, Chromium's network service runs in the browser process by
  default, and the storage service always does. [C4] Encri holds its keys in a process with no `INTERNET` permission,
  "a property worth defending" (encri D70), and lets no third-party code share that address space (encri's
  invariants). Scenario 3 puts an encri key hierarchy in the process that talks to the network.
- **It makes a second vault**, with its own passphrase and its own format migrations. The backup apps cannot see it
  (encri D137). Its downloads land in the browser's vault, not the user's.
- **It asks for more trust.** It is a closed browser holding keys, it cannot go to F-Droid (encri D115 as amended), and
  making a Chromium build reproducible, as encri's releases are (encri D116), is a project of its own.
- **What it does better:** it works without encri installed, and one unlock opens it. The cost of IPC is not a
  difference: in scenarios 1 and 2 at stage 2, the crypto runs in-process as well, and the IPC happens once per
  unlock.

## Side by side

| | 1 · Firefox + encri-api | 2 · Chromium + encri-api | 3 · Encri Core in Chromium |
|---|---|---|---|
| A UI-only change builds with | Gradle, against Mozilla's GeckoView | a full Chromium build | a full Chromium build |
| Persistent encryption of live stores | Mozilla's Lockstore, landing, off by default; the fork supplies the KEK | the fork writes it (SQLite VFS, LevelDB, JSON, Java) | the same as 2 |
| Memory-only mode | private mode, no engine change | incognito, needs the build | incognito, needs the build |
| Password store | logins (application-services), key to be fixed | **none** in public Chromium | **none** |
| Extensions on phones | AMO, working | none | none |
| encri-client fit | Kotlin to the engine boundary | Java plus a Kotlin adapter | JNI to encri's JVM code |
| A slower branch | ESR, backported by hand as Tor does | none on Android | none on Android |
| Releases a year | ~26 majors, plus point releases | ~26 milestones, plus weekly respins | the same as 2 |
| Encri contract change | none | none | not applicable |
| Downloads land in | the user's vault | the user's vault | a second vault |
| Source | open (MPL-2.0 files) | open | closed, with Chromium changes published |
| Fits encri's design | an ordinary companion, like Encri Images | an ordinary companion | contradicts D70 and the invariants |

## Recommended path

1. **Pick the branch before stage 2, not before stage 1.**
   - Stage 1 is Gradle-only and can track release.
   - Stage 2 means choosing between rebasing every two weeks and hand-backporting onto ESR. ESR 175, due June 2027, is
     the first ESR likely to carry all of Lockstore *(unverified)*.
2. **Spike stage 1 on the phones.**
   - Fenix built against a published GeckoView, every tab private, downloads through `openWrite`.
   - Measure download throughput through the pipe against MediaStore on the S9, A20, G6 and S24.
   - Confirm that nothing of Gecko's reaches `filesDir` or `cacheDir` in plaintext. Diff the directories before and
     after a session.
3. **Spike stage 2 before writing any of it.**
   - Build GeckoView with Lockstore's prefs on.
   - On each phone: does the profile open a second time? What does each store do when its KEK locks mid-session?
     Razing is the failure to look for. What does `ObfuscatingVFS` cost against plain SQLite?
4. **Ask Mozilla before carrying a patch.** An embedder-supplied KEK in GeckoView sits next to the OS-keystore KEK
   Mozilla has filed (bug 2055641) and could be upstreamed. That would leave the fork with prefs and no Gecko patch.
5. **Settle what is permanent first:** the product's name, which cannot contain "Firefox"; its package, on which a
   grant is keyed for good (encri D120, D145); and its registry entry.

## Open questions

- **History.** Does stage 1 keep history at all, or only bookmarks and tabs?
- **application-services.** `places.sqlite` and the login key are outside Lockstore. Fix them in the fork, or wait for
  Mozilla?
- **Web compatibility.** Is Gecko's web compatibility acceptable for the people encri is for? This is the one
  argument for Chromium that this study cannot weigh.
- **Lock behaviour.** On `STATE_LOCKED`, does the browser close outright, or keep running tabs in memory with
  persistence paused? Closing is the conservative answer.

## Sources

### Firefox (checked 2026-11-10)

- **[F1] Source tree and build.**
  - https://firefox-source-docs.mozilla.org/mobile/android/index.html
  - https://firefox-source-docs.mozilla.org/contributing/build/artifact_builds.html
  - https://firefox-source-docs.mozilla.org/setup/linux_build.html
  - https://gitlab.com/ironfox-oss/IronFox/-/blob/dev/CONTRIBUTING.md
  - searchfox `mobile/android/android-components/components/browser/engine-gecko/build.gradle`
- **[F2] Where each store lives.**
  - searchfox `toolkit/xre/nsXREDirProvider.cpp`
  - `netwerk/cache2/CacheFileIOManager.cpp`
  - `mobile/android/fenix/app/src/main/java/org/mozilla/fenix/components/Core.kt` (`forceInsecure`)
- **[F3] Private mode.**
  - searchfox `netwerk/cache2/CacheStorage.h`
  - https://www.firefox.com/en-US/firefox/115.0/releasenotes/
  - https://bugzilla.mozilla.org/show_bug.cgi?id=1864684
- **[F4] Lockstore.**
  - https://bugzilla.mozilla.org/show_bug.cgi?id=1996001
  - https://bugzilla.mozilla.org/show_bug.cgi?id=1889150
  - searchfox `storage/SQLiteEncryption.h`
  - `security/lockstore/lockstore_rs/src/lib.rs`
  - `modules/libpref/init/StaticPrefList.yaml`
- **[F5] Extensions.**
  - https://blog.mozilla.org/addons/2023/12/14/a-new-world-of-open-extensions-on-firefox-for-android-has-arrived/
- **[F6] Cadence and ESR.**
  - https://whattrainisitnow.com/calendar/
  - https://blog.mozilla.org/sumo/2026/08/19/firefox-new-release-cadence-and-what-to-expect/
- **[F7] Licence and branding.**
  - https://www.mozilla.org/en-US/MPL/2.0/FAQ/
  - https://www.mozilla.org/en-US/foundation/trademarks/policy/
- **[F8] Tor Browser for Android.**
  - https://blog.torproject.org/new-release-tor-browser-160/
  - https://blog.torproject.org/new-release-tor-browser-120/
  - The Tor Browser design document, which describes disk avoidance, was seen only in search snippets: *unverified*.

### Chromium (checked 2026-11-10)

Paths are under https://chromium.googlesource.com/chromium/src/+/main/.

- **[C1] Build and cadence.**
  - `docs/android_build_instructions.md`
  - `docs/process/release_cycle.md`
  - https://developer.chrome.com/blog/chrome-two-week-start
  - https://support.google.com/chrome/a/answer/16942104
- **[C2] Forks.**
  - https://github.com/GrapheneOS/Vanadium/tags
  - https://chromiumdash.appspot.com/fetch_releases?channel=Stable&platform=Android
  - https://github.com/uazo/cromite/releases
  - https://vivaldi.com/blog/technology/why-isnt-vivaldi-browser-open-source/
- **[C3] Where each store lives.**
  - `chrome/common/chrome_paths_android.cc`
  - `content/browser/indexed_db/instance/bucket_context.cc`
  - `components/services/storage/dom_storage/features.cc`
  - `chrome/browser/tab/tab_state_storage_database.cc`
  - `components/os_crypt/async/browser/posix_key_provider.cc`
  - `chrome/browser/password_manager/android/password_manager_android_util.cc`
- **[C4] Hooks and the process model.**
  - `services/network/network_context.cc`
  - `services/network/enterprise/encryption/`
  - `sql/` (`VfsWrapper`, `SandboxedVfs`)
  - the `NetworkServiceInProcess2` feature
- **[C5] Kotlin and dependencies.**
  - `third_party/android_deps/README.md`
  - https://kotlinlang.org/docs/java-to-kotlin-interop.html
- **[C6] Extensions.**
  - `extensions/buildflags/buildflags.gni`
  - `build/config/android/config.gni`
  - https://9to5google.com/2026/09/21/googlebook-launch/ (secondary)
- **[C7] Licences and branding.**
  - `docs/adding_to_third_party.md`
  - `third_party/blink/LICENSE_FOR_ABOUT_CREDITS`
  - https://about.google/brand-resource-center/trademark-list/
  - Whether any GPL code ships in the Android binary is *unverified*. Run `tools/licenses/licenses.py` on the build,
    and have a lawyer read the result before scenario 3.

`posix_key_provider.cc` was read directly in this session. The enterprise encryption directory was seen only as a
file listing on the GitHub mirror, and that the cache delegate takes its key from OSCrypt is inferred from the file
names.

### How the Firefox sources were checked

Searchfox paths are under https://searchfox.org/firefox-main/source/. Three pages were read directly in this session:
`SQLiteEncryption.h`, `lockstore_rs/src/lib.rs` and bug 1996001. The rest come from one research pass on the same day
and were not each re-read.
