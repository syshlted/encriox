# W13 — Stage 2 builds its own Gecko: QuotaManager fixed, and SQLite's switch on by default, because the launch guard reads nothing else

*2026-11-18 · built and run on the user's word ("OK let's do the gecko build next"), the call W12 left them, on the G6,
the S9 and the A20. **Flagged for veto on three points** (below). Nothing here is a design for encriox yet.*

#### What was built

`scripts/spike/stage-2/`, new: `fetch`, `build`, `mozconfig` and `patches/`.

- **The checkout** is stage 1's, made by its `fetch`, with stage 2's patches on an `encriox-spike-stage-2` branch, on
  the drive for large builds (W7). Cloning took 14 minutes, for 5.9 GB.
- **Gecko is compiled**: `mach build`, optimized, aarch64, release channel, without debug symbols, and Fenix alone of
  the Android projects. **Fenix is built from the checkout's root Gradle build**, `mach gradle fenix:assembleDebug`
  (Mozilla's `mobile/android/docs/fenix.md`). Only there is GeckoView a project, which Android Components then takes
  in place of the published artifact.
- **mach's state is on the same drive**, beside the checkout. It holds what mach bootstrapped: clang, the NDK, a
  sysroot, and a JDK of stage 1's release, which configure took over `JAVA_HOME`. The SDK and bundletool are stage 1's,
  under `~/.mozbuild`.
- **Rust is Mozilla's CI toolchain for Android**, `linux64-rust-android`, 1.95.0, fetched by
  `mach artifact toolchain`. Configure bootstraps Rust only in automation, so the first configure stopped at "Rust
  compiler not found". The machine had no Rust.

The first build took 76 minutes: 69 for `mach build`, at 86% of 8 cores, and 7 for Gradle. A change to one pref then
took 8½ minutes. The objdir holds 11 GB and mach's state 6.6 GB; `~/.gradle`, on the home drive and shared with the
suite's builds, holds 7.9 GB. The arm64 APK was 170 MB. An incremental package stored more of it uncompressed, at
239 MB, with the same `libxul.so`. Every build has the build ID `20261118052545`, which mach keeps across incremental
builds.

#### The patches

1. **QuotaManager records an origin's encryption when it makes the origin's directory.**
   `GetOrCreateTemporaryOriginDirectory` takes the encrypted state from the origin's `OriginInfo`, with the access
   time and the other flags. This is W12's cause, fixed as W12 read it.
2. **The SQLite switch is no longer locked** in `all.js`, and its lint exemption goes with the lock.
3. **The root Gradle build includes encri-api**, as Fenix's own `settings.gradle` does in stage 1's patch 0001.
4. **The SQLite switch is on by default** in `StaticPrefList.yaml`. Below is why.

#### The launch guard reads the build's default, and nothing else (measured)

`CheckEncryptionCompatibility` runs in `XRE_mainStartup`, and XPCOM, with Preferences, starts after it
(`nsAppRunner.cpp`). Preferences cannot start before XPCOM (`Preferences::InitStaticMembers`), so the guard's read of
the switch returns the value compiled from the YAML. A GeckoView embedder's default prefs, a user's and a policy's are
all read later. `main` has the same order (2026-11-18).

Measured on the S9, with patches 1 to 3 and the switch turned on by stage 2's config:

- The gate's start made the profile and encrypted it. `cookies.sqlite`, `permissions.sqlite` and `storage.sqlite` had
  obfsvfs's header, and `compatibility.ini` had `EncryptedDatabases=1`.
- `check-plaintext` then stopped the app, and the marker page's start was refused, 14 ms after Gecko began loading
  SQLite: "Profile encryption state does not match launching build." The guard saw `false`, and the profile was
  encrypted.

So in a build, SQLite's encryption is the build's to choose, not a pref's: patch 4. A config may still turn the
switch off. The guard then refuses the profile at its next start, as having plaintext databases (read in the source,
unmeasured). That still serves a measurement of one session, such as stage 2's third round.

On Mozilla's builds the lock keeps prefs out. But `all.js` says enterprise policies and Lockstore's key recovery
override it by unlocking it, setting its default and locking it again, which happens after the guard. If an override
turns it on, the next start would be refused in the same way (read in the source; desktop unmeasured).

#### What the switches cover now (measured)

A normal session, `check-lockstore`, with patches 1 to 4 and stage 2's config:

| | G6 | S9 | A20 |
|---|---|---|---|
| Files holding the marker, switches off (W4, W5) | 10 | 9 | 11 |
| Switches on, the published GeckoView (W12) | 4 | 5 | 4 |
| Switches on, this build | 4 | 4 | 4 |

The four left are outside Gecko's databases, the same on all three:

- Fenix's saved tabs (`mozilla_components_session_storage_gecko.json`);
- Fenix's history, application-services' `places.sqlite` and its WAL, outside Gecko's profile;
- `serviceworker.txt`, the service worker's registration with its script's URL, which is a text file.

The cookies' WAL no longer holds the marker. Every database in Gecko's profile has obfsvfs's header but three, which
Lockstore leaves out by design: NSS's `cert9.db` and `key4.db`, and Lockstore's own `lockstore.keys.sqlite`, which
holds its `local` keys in plaintext (W12). Every profile has the `EncryptedDatabases=1` marker.

#### Whether the profile opens a second time (measured)

| | G6 | S9 | A20 |
|---|---|---|---|
| Cookie, local storage, IndexedDB, Cache API, service worker, HTTP cache | read back | read back | read back |
| The test origin's flags in `.metadata-v2` | `0x17` | `0x17` | `0x13` |
| The same, the published GeckoView (W12) | `0x7` | `0x7` | `0x13` |

The G6 and the S9, which lost site storage in W12, now record the origin as encrypted, and read all six stores back.
The HTTP cache answered the read-back page itself, on all three. The `chrome` origin, a persistent one, is recorded
`0x9`, plaintext, on all three. QuotaManager encrypts best-effort origins only (`QuotaManager::IsOriginEncrypted`),
and its database had obfsvfs's header from the SQLite switch.

#### What it means for stage 2

1. **All three of Lockstore's switches work in stage 2's build, on all three phones.** What still holds the marker is
   outside Gecko's databases (above), and Lockstore's own keys are in the clear beside them.
2. **Its second round, a key-encryption key from the vault, can start** against this build. It is what makes the
   switches protect anything: Lockstore's `local` keys are beside the data they open.
3. **Its third round, what `obfsvfs` costs**, can turn the switch off by config, on a profile of one session.
4. **Mozilla should know two things** (`ROADMAP.md`, item 5): patch 1, built and measured; and that the launch guard
   sees only the build's default.

After the round, each phone got a fresh Gecko profile, stage 1's APK and stage 1's config again. Stage 2's build
encrypts every profile unless a config turns the switch off, and stage 1's scripts run with stage 1's config, which
does not name it. Stage 1's control on the S9 found its marker in 11 files, on the published GeckoView.

#### Flagged for veto

1. **SQLite's encryption is on by default in stage 2's build** (patch 4), because the launch guard reads nothing else.
   A profile made by this build is encrypted unless its config says otherwise, from its first start.
2. **The build's toolchain is on the drive for large builds, apart from stage 1's**: a second mach state with its own
   JDK, and Rust from Mozilla's CI rather than from rustup.
3. **The phones go back to stage 1's build after each of stage 2's rounds**, rather than keeping stage 2's.

#### Not measured

- Patch 1 reverted in this build: the failing case is W12's, the published GeckoView without it.
- A profile made with the switch off by config, at its second start; a private session with the switches on; a third
  start; any origin but the test one.
- The guard on desktop, under an enterprise policy.
- Mozilla's own tests of the four patches: the build is Android's, and none was run.
- The S24.

#### Rejected

- **Moving the guard after Preferences start.** It is a larger change to startup code that desktop shares, and the
  guard must still run before any storage code. The build's default is what Gecko reads there now.
- **The switch locked `true`.** No config could then turn it off, for the third round's plain SQLite.
- **rustup**, which `mach bootstrap` would install into `~/.cargo`, on the home drive, with a Rust of its choosing.
  Mozilla's artifact is the Rust their Android builds use at this revision.
- **Fenix's own Gradle build against a local Maven GeckoView.** The root build is Mozilla's documented way, and needs
  one patch to include encri-api.
