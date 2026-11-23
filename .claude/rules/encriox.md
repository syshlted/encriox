---
paths:
  - "firefox-release"
  - "patches/**"
  - "scripts/fetch"
  - "scripts/build"
  - "scripts/mozconfig"
  - "LICENSE"
---

# encriox's own build — component brief

encriox itself, as against the spikes: `firefox-release` names the Firefox release it is built on, the tag and its
commit (W9); `patches/` is its series on that tag; `scripts/fetch` makes a shallow checkout of the tag with the series
committed on a branch of its own, `encriox`; and `scripts/build` compiles it with `scripts/mozconfig`, Gecko and
application-services' megazord from the checkout and Fenix's debug APK against them, as stage 3's spike does (W26).
`LICENSE` is here too. This brief is level 1; full records are in `docs/decisions/`.

## What the series holds

Seven patches: stage 3's sealing of application-services' databases, with the spike's fixes folded in, and stage 1's
fix that lets a debug build start on the G6 (W26):

| Patch | What it carries | From the spikes, stage 3's but where said |
|---|---|---|
| 0001 | obfsvfs in sql-support: each page sealed with AES-256-GCM, the key by a handle, a check of the key | 0001, 0005, 0008 (W21, W24) |
| 0002 | a sealed open in places, the logins and autofill, with their Kotlin bindings | 0002 |
| 0003 | a database key for Android Components' places, logins and autofill stores | 0003, on the release |
| 0004 | SQLite's recovery compiled into the megazord, and `sqlite_dbpage` in its SQLite | 0010 (W25) |
| 0005 | obfsvfs's check and repair of a sealed database | 0011, 0016's repair |
| 0006 | places carrying the check and the repair to Kotlin, for all three databases | 0012, 0016's places |
| 0007 | Fenix's StrictMode listener ignoring LG's resource preloading, which killed the debug build | stage 1's 0004 |

**Nothing of Fenix's is changed yet** but that: the app is Fenix's own debug build, `org.mozilla.fenix.debug`, which
gives no store a key, so places', the logins' and autofill's databases are in the clear, and history and bookmarks are
on, as Fenix has them (W26). The keys, from `.encriox.key`, and the prompt at start come with the spikes' patches that
are encriox's (`ROADMAP.md`, item 6). Until then a build of the series is checked, never used: W22's point 3 allows
nothing in the clear at rest.

## Rules that hold here

- **The patches are the source**, as the spikes' are. Commit on the checkout's `encriox` branch and write them out
  again, then commit them here; never edit a `.patch` by hand:

  ```sh
  git -C <checkout> format-patch -o "$PWD/patches" --no-numbered --zero-commit FIREFOX_160_0_RELEASE
  ```

- **One change to a patch** (W9, flag 1): a spike's patch and the fixes measured after it are carried as one, and the
  message names the spike's patches it carries. The spikes keep their own series, which their measurements ran.
- **Every prefix of the series builds**: `scripts/build` at each, the release alone included (W26).
- **`firefox-release` is the one place the release is named** (W9): what says which release encriox is built on reads
  it, and `build` refuses a checkout not built on its commit.
- **A new file begins with Exhibit A's notice** (W2), the scripts and the mozconfig included, in `#` comments.
- **The checkout, its objdir `obj-encriox`, and mach's state** go on the drive for large builds, the state beside the
  checkout, which the spikes' checkout shares (W7).
- **obfsvfs's own tests run on this machine** with the spike's harness, which takes any checkout:
  `scripts/spike/stage-3/check-obfsvfs-host <checkout>`.

## Traps already paid for

- **`fetch` reads `patches/` only once its clone is done**, minutes in: a series written meanwhile is the one it
  applies (W26).
- **Cargo takes a vendored crate as it was built last, by its version alone** (W25): an objdir built before patch 0004
  needs libsqlite3-sys's build removed, or its SQLite has no `sqlite_dbpage` and every repair fails, as
  `scripts/spike/stage-3/README.md` says.
- **Stage 1's patches change the logins' and autofill's stores in Android Components**, and stage 3's patch 0003 was
  made on them: carried onto the release, its hunks took a three-way apply, the same lines added (W26).
- **Fenix's debug build is killed on the G6 at its first start**, by its own StrictMode listener, for a read of the
  disk in LG's framework, before any of Fenix's code runs: patch 0007, stage 1's 0004, ignores LG's two methods (W4,
  W26). Its log says `StrictMode violation suppressed` for each since.
- **mach makes a virtualenv for each new checkout**, and Firefox 160's requirements ask for pyyaml 6.0.3 under a
  6.0.4 constraint: pip's *ResolutionImpossible* heads the first build's log, and mach goes on without pyyaml's native
  loader. It is harmless.

## Measured, not assumed

- **`fetch` took 17 minutes** from GitHub, a 1.0 GB pack, and the checkout is 6.1 GB; **the first build 80 minutes**,
  73 of them mach's, Gecko and the megazord, and 7 Gradle's, for an objdir of 12 GB; a change to Fenix alone, 5 (W26).
- **A build of the series on the G6** (W26, patch 0007's): a page opened from another app loaded in a normal tab, and
  went to History; *Bookmark page* saved it, and Bookmarks listed it; the menu offers History and Bookmarks; places'
  database held its row, one visit and one bookmark, in the clear, with places' own 32 KiB pages and no reserved bytes,
  through obfsvfs as SQLite's default VFS; and a new process listed it in History again. Without patch 0007 it was
  killed at its first start.
- **obfsvfs's 23 tests pass** at the tip on this machine (`check-obfsvfs-host`), the same files as stage 3's tip's.

## Decisions

| | Why it matters here |
|---|---|
| W2 | MPL-2.0 for every file; Exhibit A's notice on a new one |
| W8 | The package, `com.systemhalted.encriox`, which the series does not set yet |
| W9 | A series of patches on a release tag, one change to a patch; one file naming the release |
| W17 | The first usable build: what of the spikes is encriox's, and in what order |
| W21, W24, W25 | What patches 0001 to 0006 carry, and what the spike measured of them |
| W22 | Nothing in the clear at rest (point 3) |
| W26 | The series begins: the layout, and stage 3's sealing as six patches |

## Open here

- The rest of `ROADMAP.md`'s item 6: the newest release, a build type and package of encriox's own, its branding and
  prefs, and the spikes' patches that are encriox's, with the keys and the prompt at start.
- W26's flags.
