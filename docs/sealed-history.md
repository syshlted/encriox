# History and bookmarks in a sealed store: a plan

*2026-11-20 · a study, drafted at the user's word ("Let's work on this next"), after the spikes' three bugs (W17's
amendment). Nothing in it is decided. Every fact below was read in the source at Firefox 160 and application-services
v160.0 on 2026-11-20, unless it says *unmeasured* or *unverified*.*

## What it is for

encriox's first usable build records no history and has no bookmarks (W17), because Fenix keeps both in
application-services' `places.sqlite`, in the clear beside Gecko's sealed profile. This is how to give them back
sealed with the vault's key, as Gecko's own databases are (W13, W14), so that nothing of what the user visited or
kept is readable on disk without the vault.

## Where it starts from

- **Places opens its database through a SQLite of its own**: application-services' Rust, built into Fenix's
  `full-megazord` library with rusqlite's bundled SQLite, not Gecko's. Gecko's obfsvfs, which seals the profile's
  databases (W13), is not in it.
- **Its API takes a path and nothing else.** `PlacesApi(path)` in Kotlin, `placesApiNew(String)` in its bindings, and
  Android Components opens `places.sqlite` in `filesDir` with it (`Connection.kt`). The published `places-160.0` AAR
  says the same.
- **Places opens with `SQLITE_OPEN_URI`** (`support/sql`, `read_write_flags`), so a URI parameter would reach SQLite.
  But `PlacesApi::new` first normalizes the path, and `unurl_path` drops a URI's query, so a key in the path is lost
  before the open. A change to places is needed whichever way is chosen.
- **Gecko's obfsvfs is a VFS shim keyed by a URI parameter**, `key=` and 64 hex digits (`storage/ObfuscatingVFS.cpp`,
  from SQLite's public-domain demo, with an NSS cipher strategy from `dom/quota`). A database opened without a key
  passes through it in the clear. A connection's WAL and journal arrive without the main file's URI, so Gecko's copy
  gives them the main file's key; a port would keep that.
- **application-services builds NSS for Android already** (`libs/build-nss-android.sh`), which obfsvfs's cipher
  needs. SQLCipher, which it once used, is gone from its build.
- **Fenix's other records** are in Room databases, through Android's own SQLite: pinned top sites and collections.
  They are not places, and stay off (W17).

## The ways

1. **obfsvfs in application-services' SQLite. Recommended.** A small crate of encriox's own compiles a port of
   Gecko's obfsvfs against the megazord's bundled SQLite and NSS, and registers it at the megazord's init. Places takes
   a key beside its path and opens `file:…/places.sqlite?key=…`. Android Components passes a key the app derives from
   the vault's key, as stage 2 derives Gecko's secret (patch 0006), and the app opens places only once the vault has
   given it. It is the encryption the rest of the profile already has, with its cost measured (W16), and **the same
   key parameter seals the logins' and the cards' databases too**: their usernames, passwords and card numbers are
   sealed field by field (W6), but each login's origin and when it was last used are plain columns.
2. **SQLCipher in the megazord.** Gives places a `PRAGMA key` instead. application-services removed it, and bringing
   it back is a second cipher, a second crypto provider and a larger patch than obfsvfs, for the same outcome.
3. **Places in memory, written whole into the vault.** A whole snapshot is the one kind of write the vault takes
   (`AGENTS.md`, hard constraint 1). Places has an in-memory mode (`PlacesApi::new_memory`), but no way to export or
   import a snapshot, so this changes places too; a crash loses everything since the last snapshot; and the vault takes
   a new copy of the whole database at every close.
4. **A store of encriox's own** behind Android Components' `HistoryStorage` and `BookmarksStorage`. Fenix's
   `PlacesHistoryStorage` is also its `HistoryMetadataStorage` and an `AutocompleteProvider`, so this is a rewrite of
   places' features in Kotlin. Rejected.
5. **Waiting for Mozilla.** Nothing in either source tree points to a key for places.

## What way 1 costs

- **A second fork to carry.** application-services tags a release for each Firefox release (`v160.0` for 160, the
  version Fenix pins), so its patches follow the same series as Firefox's (W9). About four, from the outline: the
  obfsvfs crate and its registration, places' key, the logins' and autofill's keys, and the build's own settings.
- **A build of the megazord**, with NSS for Android, NDK 29 and Rust's Android targets, which Fenix then takes in place
  of the published one through `autoPublish.application-services.dir`. Its time and disk are *unmeasured*.
- **A port of obfsvfs out of Gecko.** About 1,000 lines of C++, with Lockstore's own key derivation taken out, and
  NSS's cipher strategy beside it, about 170. Gecko's copy stays as it is.

## Decisions it needs first

1. **The way.** Recommended: **1, obfsvfs in application-services' SQLite.**
2. **What it seals.** Recommended: **places, logins and autofill**, all three of application-services' databases
   that hold the user's own data. Remote tabs, Suggest's and Remote Settings' caches hold none of it while sync and
   Suggest are off (W17), and stay as they are.
3. **Whether the first usable build waits for it** (W17's amendment). Recommended: **it waits for the spike round
   below, and carries history and bookmarks only if that round holds** on the A20 and the S9. Otherwise the first build
   goes without them, as W17 has it, and they come in a later build.

## The work, in order

1. **A spike round, stage 3, before writing any of it**: application-services at `v160.0`, built for arm64 on the
   drive for large builds, with obfsvfs and places' key, and stage 2's Fenix built against it. Measured: the build's
   time and disk; a session's marker absent from `places.sqlite`, its WAL and its journal; history and bookmarks read
   back at the next start; what a start without the key does, which must raze nothing (W14's rule); and what obfsvfs
   costs places, against a plain build. M to L.
2. **The same for logins and autofill**, with `check-keys`. S.
3. **History and bookmarks on in Fenix's menus again**, and Fenix's history delegate set (`Core.kt`). S.
4. **The patches carried into encriox's repository** beside Gecko's, and its release script building the megazord
   too (W9). M.
