# W12 — Lockstore's switches at 160: the HTTP cache's holds, site storage's breaks at the next start, and SQLite's is locked off

*2026-11-18 · run on the user's word ("OK let's move onto stage 2"): stage 2's first round (`ROADMAP.md`, item 4),
on the G6, the S9 and the A20, against stage 1's build, with prefs alone. **Flagged for veto on one point** (below).
Nothing here is a design for encriox yet.*

#### What was run

`scripts/spike/stage-2/`, new:

- **`geckoview-config.yaml`**: stage 1's pref, with Lockstore's three switches on, all off by default at 160:
  `security.storage.encryption.sqlite.enabled`, `browser.cache.disk.encryption.enabled` and
  `dom.quotaManager.encryption.enabled`. GeckoView hands them to Gecko as default prefs.
- **`fresh-profile`**: deletes the spike's Gecko profile and its cache. Fenix's own data, the vault's grant and
  `encriox.key` are not in it.
- **`check-lockstore`**: on a fresh profile, stage 1's `check-plaintext <phone> normal` with this config; then the
  header of every SQLite file in the profile, and each origin's record in QuotaManager; then a second start, whose
  page reads back what the first session wrote, store by store, and writes nothing (`stage-1/site/read.html`).

`check-plaintext` takes the config to push from `CONFIG`, and refuses one whose SQLite switch disagrees with the
profile on the phone. Gecko refuses such a profile, and on Android it only logs it and stops (`nsAppRunner.cpp`,
`CheckEncryptionCompatibility`). After the round each phone got stage 1's config and a fresh profile again, because
`measure`, `check-stopped` and `check-keys` run with whatever config is on the phone.

Lockstore's keys stayed its own `local` ones, which it keeps in plaintext in the profile's `lockstore.keys.sqlite`
(`keystore.rs`, read in the source; the S9's held `lockstore::kek::local:…` rows). So this round measures what the
switches cover, not what they protect.

#### What the switches cover (measured)

A normal session, as stage 1's control:

| | G6 | S9 | A20 |
|---|---|---|---|
| Files holding the marker, switches off (W4, W5) | 10 | 9 | 11 |
| Switches on | 4 | 5 | 4 |

Where the marker still was, on all three:

- **Fenix's saved tabs** and **its history**, `places.sqlite` or its WAL. They are Fenix's and application-services',
  outside Gecko's profile, which no switch reaches (`open-questions.md`).
- **The cookies' WAL**, because the SQLite switch did nothing (below).
- **`serviceworker.txt`**, the service worker's registration, with its script's URL: a text file, which none of the
  three switches is for.

No longer holding it: the HTTP cache, local storage, IndexedDB, and the Cache API's database and bodies.

**The SQLite switch is locked off.** `all.js` sets it `false, locked` (`modules/libpref/init/all.js:4014`, read in the
source), and a default pref cannot change a locked one. On every phone only QuotaManager's own databases had
obfsvfs's header, 8192-byte pages with 32 reserved bytes, and `compatibility.ini` had no `EncryptedDatabases` marker.
`cookies.sqlite`, `permissions.sqlite`, `formhistory.sqlite`, `storage.sqlite` and the rest were plaintext. `main`
locks it too (2026-11-18).

#### Whether the profile opens a second time (measured)

Each phone was started again on the same profile, and a page read back the first session's marker:

| | G6 | S9 | A20 |
|---|---|---|---|
| Cookie, service worker, HTTP cache | read back | read back | read back |
| Local storage, IndexedDB, Cache API | **failed** | **failed** | read back |
| The origin's flags in `.metadata-v2` | `0x7` | `0x7` | `0x13` |

**Where site storage failed, the origin was recorded as plaintext.** Bit `0x10` of an origin's flags says it is
encrypted (`DirectoryMetadataFlags::Encrypted`). On the S9, twice, and on the G6, it was missing, though the origin's
databases were encrypted and their keys were in Lockstore. `storage.sqlite`'s row for the origin said the same. So the
second start opened the Cache API's database without its key, and QuotaManager's first initialization of temporary
storage failed (`dom/cache/DBAction.cpp:215`). Every site's local storage, IndexedDB and Cache API wait on that
initialization. Only the test origin was measured.

**Nothing was wiped.** The databases kept their size and their first session's time. The open failed with an error
that is not a corruption, so `DBAction.cpp`'s wipe did not run.

**The flag is the whole cause.** On the G6 the flags were set to `0x17` by hand, the same with `0x10` added, and the
next start read back all six stores, with no QuotaManager failure in the log.

**Why the flag goes missing** (read in the source, at 160 and on `main` on 2026-11-18). QuotaManager can set an origin
up without its directory. `GetOrCreateTemporaryOriginDirectory` makes the directory later, when a client asks
(`dom/quota/ActorsParent.cpp:3806`). It writes the origin's record from the asking client's `OriginMetadata`. It takes
the access time and the accessed, persisted and dirty flags from the origin's `OriginInfo`, but not its encrypted
state. Local storage, IndexedDB, the Cache API and the file system API all ask through it. Which of the page's stores
asked first on each phone was not measured. The fix would be one line, taking the encrypted state from `OriginInfo`
too, and only a Gecko build or Mozilla can make it.

#### What it means for stage 2

1. **The HTTP cache's switch works with a pref alone**: no marker on disk, and the cached answer came back after a
   restart, on all three.
2. **Site storage's switch must stay off** until QuotaManager records an origin's encryption. With it on, the next
   start can lose every site's local storage, IndexedDB and Cache API: an S1 for encriox, whatever the odds.
3. **SQLite's switch needs Gecko changed**: a build, or the published GeckoView's prefs edited, which is unmeasured.

So two of the three switches wait on a Gecko build or on Mozilla, and without them stage 2's second round, a key from
the vault, would protect the HTTP cache alone. Whether to build Gecko is the user's call (`ROADMAP.md`, item 4).
Mozilla should know about the missing flag (item 5).

#### Flagged for veto

1. **`fresh-profile` deletes the spike's Gecko profile**, not keeps it aside. On a test phone it holds only earlier
   runs' markers, and a profile cannot change its SQLite switch in place. To return to stage 1, start fresh too.

#### Not measured

- A private session with the switches on; whether a third start recovers on its own; any origin but the test one.
- What `obfsvfs` costs, stage 2's third round, which needs the SQLite switch on.
- The S24.

#### Rejected

- **Turning the SQLite switch on from the app.** A pref from the app is a default, and a locked pref takes neither a
  default nor a user's value.
- **The marker search alone.** It found site storage covered on all three phones, and only the second start found
  that two of them could not read it back. `check-lockstore` always starts a second time.
- **Copying the whole profile off the phone to read its headers.** That took more than ten minutes over the A20's
  Wi-Fi, so each file's first 24 bytes are read on the phone instead.
