# W5 — Stage 1 holds on the S9 and the A20, and a download stopped part way now leaves nothing in the vault

*2026-11-15 · accepted as measured by the stage 1 spike (`scripts/spike/stage-1/`, W1) on the S9 and the A20, and on
the G6 for the stopped downloads, and **flagged for veto on two points** (below). **Amends W4**'s first item of what
stage 1 has to do. The S24 is unmeasured. Nothing here is a design for encriox yet.*

#### Private tabs leave no page in plaintext, on all three phones (measured)

`check-plaintext` as in W4: a page writes a fresh marker into every store a page can reach, and a minute after the app
goes home its data directories are searched for it.

| Files holding the marker | G6, API 28 (W4) | S9, API 29 | A20, API 30 |
|---|---|---|---|
| **Normal, the control** | 10 | 9 | 11 |
| **Private** | none | none | none |
| A private tab's JPEG left in `no_backup/` | yes | 72,750 bytes | 45,807 bytes |

The S9's control has no `places.sqlite` beside its WAL, and the A20's adds `serviceworker.txt`, whose registration
holds the worker's URL with the marker in it. A private session left Gecko's encrypted private storage on each, as on
the G6, and one full-screen JPEG of the private tab, as W4 found.

**The S9's control found nothing at first, and it should have.** The session had run privately: `storage/private/`
and a private thumbnail were written, and the app's preferences had no `encriox_private` at all. The gate saved
`false` through `apply()`, which writes later, and the script stopped the app as soon as the gate said it had, so the
write was lost. The gate commits it now (patch 0012), and `check-plaintext` reads it back after the stop.

#### A download into the vault runs at a third to a half of Fenix's own writer's speed (measured)

`measure <phone> 64 4` as in W4, from memory and through Gecko from a server on the phone's own loopback. On the S9 and
the A20 Fenix's own writer goes through MediaStore; on the G6 it wrote the legacy file.

| 64 MiB, four rounds | G6, API 28 (W4) | S9, API 29 | A20, API 30 |
|---|---|---|---|
| Sink alone, copy: own / vault (MiB/s) | 172–379 / 52–71 | 181–647 / 103–116 | 110–211 / 45–47 |
| Through Gecko, copy: own / vault (MiB/s) | 103–151 / 51–64 | 172–230 / 76–99 | 71–134 / 42–43 |
| Through Gecko, total: own (s) | 0.56–0.73 | 0.37–0.43 | 0.76–1.35 |
| Through Gecko, total: vault (s) | 1.73–1.83 | 0.99–1.13 | 1.84–2.29 |
| Into the vault, end to end | about 35–37 MiB/s | about 57–65 MiB/s | about 28–35 MiB/s |

On every phone the vault's copy runs as fast through Gecko as from memory, so encri's pipe and encryption set the
pace, not Gecko. The A20 is the slowest of the three into the vault, and the slowest by Fenix's own writer too.

#### A download stopped part way was saved cut short into the vault (found, fixed, measured)

`check-stopped` downloads 16 MiB into the vault from the spike's server at 2 MiB/s and stops it after 3 seconds,
through the broadcast its notification's *Pause* or *Cancel* sends (patch 0007). With `unsized` the server sends no
length, and with `retry` the download is then tried again through the broadcast *Try again* sends.

| | Before, on the G6 | After, on the G6, the S9 and the A20 |
|---|---|---|
| No length, paused | **6,242,304 bytes kept**, under the download's own name, shown paused | nothing kept, shown failed |
| No length, cancelled | **6,242,304 bytes kept**, shown cancelled | nothing kept, shown failed |
| A length, paused | nothing kept, shown failed | nothing kept, shown failed |
| A length, paused, then *Try again* | asked for `Range: bytes=6242304-`, failed | started again, kept whole: 16,777,216 bytes |

A pause or a cancel ends Android Components' copy just as the end of the response does, and the writer sees only that
the block returned. The spike's writer abandoned a copy shorter than the length the response promised (W4), so with
no length it could not tell. And *Try again* from the notification kept the bytes copied before, so it asked the
server for the rest, which a write that replaces a document whole cannot take, every time.

Patch 0008 adds two protected members to Android Components' download service: whether a download is still running,
which the vault's writer asks once its copy returns, and forgetting what was copied, which it calls whenever the vault
kept nothing, so a retry asks for the whole file. Retrying from Fenix's downloads screen already started from nothing,
because that path makes a new job (read in the source, not measured).

#### Other faults in the spike, fixed

- **The gate trusted a held session after a lock** (patch 0010). A session outlives a lock, so on the G6, once the
  vault was locked through it, the gate still said it was open and a listing failed with `LOCKED`. The held session
  is now asked for its own scope first. Every gate after a process's first, on the S9 and the A20, took the answered
  path. **The locked path is unmeasured**: the G6's vault, locked to measure it, did not open again (*For encri*).
- **The loopback server's threads threw what they did not catch.** A download stopped part way closes its end, and the
  server's write throws there; uncaught on that thread, it would kill the app (patch 0007). The measured downloads all
  ran to the end, so it had never happened.
- **The scripts went on where they should have stopped**: `check-plaintext` ran with a mode the gate had not
  confirmed and left its temporary directory behind when it ended early, and `measure` left a round out of its results
  when a download did not finish.

#### Found in the source, not measured

1. **A release can set Gecko prefs from a file the app names.** `GeckoRuntimeSettings.Builder.configFilePath` is public,
   and `GeckoRuntime` reads the path it is given whether or not the app is debuggable; it checks debuggability only
   to pick the default under `/data/local/tmp`, which is all the spike uses (read at Firefox 160). This answers W4's
   third item in principle. It is not measured in a release build.
2. **Lockstore is in the GeckoView Mozilla published.** `security/lockstore` is built into `libxul` on every platform,
   and the published `geckoview-omni:160.0.20261104181552` for arm64 holds its three prefs, `LockstoreStorageKeyProvider`
   and the obfuscating VFS. So stage 2's first questions, whether a profile opens a second time with encryption on and
   what it costs, could be asked of stage 1's build with prefs alone, before any Gecko build. Only a key-encryption key
   that the vault supplies needs a patch to Gecko. Whether the prefs work in that build is unmeasured, and the branch
   is still picked before stage 2 starts (W1, item 2).

#### For encri

- **The S9's and the A20's test vaults have no folders either**, so the spike was allowed each whole, as it was the
  G6's (W4).
- **The G6's vault is locked, and the passphrase on file for it does not open it.** The spike locked it to measure
  patch 0010. Encri refused the passphrase twice, though the keys that reached its window were the file's, character
  for character. The vault has a fingerprint-or-screen-lock slot, and the G6 has a screen lock, which only its owner
  knows.
- **LG's framework logs every key event** on the G6, key and shift state, so a passphrase typed there over adb is in
  its log until it is cleared.

#### Flagged for veto

1. **The S9's and the A20's whole test vaults were allowed to the spike**, against W1's one folder, as the G6's was
   (W4, flag 1), because neither has a folder to give it. The spike deletes only what it wrote.
2. **A download into the vault that stopped shows as failed, a cancelled one included.** The writer throws to abandon
   the write, and Android Components marks failed any download whose writer throws. *Try again* then starts it from
   the start.

#### Rejected

- ~~**Leaving a stopped download paused or cancelled.** *Resume* looks for the partial file in `Download/`, finds none,
  and fails; failed offers *Try again*, which now works.~~ **Amended by W19 (2026-11-20):** it is left so, and *Resume*
  starts a download nothing was kept of from the start.
- **Telling the writer through `DownloadFileWriter`**, which Android Components' own writers implement. Two protected
  members of the service are a smaller patch to carry.
- **Locking the S9's or the A20's vault to measure patch 0010's locked path.** The A20's is passphrase-only: had its
  passphrase on file been wrong too, no one could open it again.

<!-- amendments: generated by scripts/docs/decision-index.py; do not edit -->

## Amended since, as recorded in `amendments.md`

- **W5's first rejection** (2026-11-20, W19): a stopped download is left paused or cancelled, and *Resume* starts one
  nothing was kept of from the start, so it no longer fails for want of a file in `Download/`. W5's flag 2 ends.
- **W4's, W5's, W6's and W21's private and sealed results** (2026-11-20, W23): they searched copies of the app's data that
  nothing checked were whole. Run again with `copy-data` on the G6, the A20 and the S24, they held; the S9's rest on
  the old copies until it is attached.

<!-- /amendments -->
