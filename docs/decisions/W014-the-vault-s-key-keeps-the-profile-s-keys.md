# W14 — The vault's key keeps the profile's keys: every store follows it once Gecko is changed, and a start without it razes nothing

*2026-11-18 · built and run on the user's word ("OK let's move onto Stage 2, round 2 of encriox"): stage 2's second
round (`ROADMAP.md`, item 4), on the S9 and the A20, and the vault's locked path on the G6. **Flagged for veto on five
points** (below). Nothing here is a design for encriox yet.*

#### What Gecko had at 160 (read in the source)

Lockstore's only password KEK is the primary password's, `lockstore::kek::password:profile`. `SyncProfileKekAtStartup`
(`security/lockstore/ProfileKek.cpp`) unlocks it before `profile-do-change` by prompting, and moves under it the keys
under `lockstore::kek::local:profile`, which is the HTTP cache's alone. Nothing else reaches it:

- **No embedder can give it a secret.** Every route into startup from Java is the environment or the arguments, and
  GeckoView copies its extras, environment included, into every child process. Events from Java reach Gecko's JS
  only once it is running, after the profile's databases have opened.
- **Its prompt cannot work on GeckoView.** `PromptDelegate.promptPassword` calls `_promptUsernameAndPassword`, which
  does not exist (`GeckoViewPrompt.sys.mjs`), and no window exists yet. With a password KEK on disk, Gecko quits.
- **Each store asks for its key under the KEK it made it under, by name.** SQLite's are under
  `lockstore::kek::local:sqlite`, the HTTP cache's under `local:profile`. QuotaManager follows whichever KEK wraps its
  anchor key.
- **Lockstore answers a locked key and a missing one alike**, `NS_ERROR_NOT_AVAILABLE`.

So a built-in extension holding the secret, the way `open-questions.md` proposed, comes too late, and could protect
site storage alone.

#### What was built

`scripts/spike/stage-2/`, five patches on W13's four:

- **0005**: GeckoView's `ProfileSecretDelegate`, which Gecko asks over JNI from `SyncProfileKekAtStartup`, on its own
  thread, before `profile-do-change`, and waits for; the app's main thread is never held. The secret unlocks the
  profile's password KEK, or makes it the first time, and every key still under a LocalKey is moved under it and the
  LocalKey deleted. Without a secret, a KEK that exists stays locked and startup carries on. Gecko wipes the Java
  copy once it has its own. `GeckoRuntime.lockProfile` and `unlockProfile` lock and unlock it mid-session, through a
  JS module in the parent process.
- **0006**: the spike's `EncriProfileSecret`. The secret is an HMAC-SHA256 of the browser's key in the vault, under a
  label of its own: the spike's `encriox.key` (W6), where encriox's is to be `.encriox.key` (W10). Asked at startup, it
  opens the vault on Gecko's thread, asking the user nothing, and answers with nothing if the vault is locked, has not
  let the browser in, or does not answer in 10 seconds. Then it locks Gecko's keys whenever the vault's key is dropped,
  and unlocks them when it is held again. The gate's `--es kek give|withhold` decides the next start's answer; `--es
  profile lock|unlock` locks or unlocks the running Gecko.
- **0007**: each store reads its key under whichever KEK wraps it, and a new key goes under the profile's password
  KEK once there is one: SQLite's, the HTTP cache's and QuotaManager's anchor. A key that is there and locked is kept,
  never replaced. A database whose key is locked fails to open with `NS_ERROR_FILE_ACCESS_DENIED`, as obfsvfs fails an
  open it cannot key. A session locked from its start does not use the disk cache.
- **0008**: the gate's `--ez drop true` drops the vault's key from memory, as a lock of the vault does, and leaves the
  vault open.
- **0009**: Lockstore's `secure_delete` checkpoints its write-ahead log and truncates it (below).

`check-kek` runs the round on one phone, in six steps (its header).

#### At 160, the move razes two stores (measured)

On the S9, with patches 1 to 6 and the secret given at both starts, the second start moved 14 key wrappings from
LocalKeys under the vault's secret, in 355 ms. Then:

- **every SQLite open failed**, each asking for its key under `local:sqlite`. **The cookie store moved
  `cookies.sqlite` aside**, as `cookies.sqlite.bak`, its WAL left under the old name, and could not make a new one.
  Site storage could not start: local storage, IndexedDB and the Cache API failed. Permissions ran in memory.
- **the HTTP cache deleted its moved key and made a new one**, under a `local:profile` it made again, and missed.
- a database new that session, `protections.sqlite`, was keyed under a `local:sqlite` made again: in the clear.

#### With the stores following their key (measured)

`check-kek`, with patch 7:

| Step | S9 | A20 |
|---|---|---|
| 1. The secret at both starts: all six stores read back | yes | yes |
| Keys under the vault's secret, and under a LocalKey | 15 and 0 | 16 and 0 |
| 2. Locked at start: what was read | the service worker alone | the same |
| Files lost, or changed, while locked | none but Lockstore's WAL and Gecko's startup caches | none but Lockstore's WAL |
| Keys changed while locked | none | none |
| 3. The secret given again: all six read back | yes | yes |
| 4. Locked mid-session, a second origin: written and kept | cookie, HTTP cache | the same |
| 5. The vault's key dropped: the app locked Gecko's keys | yes | yes |
| A third origin's site storage, written | no | no |
| The key read again: the app unlocked them | yes | yes |
| A fourth origin, all six read back at the next start | yes | yes |
| 6. A profile made with the secret withheld: keys under LocalKeys | 14 of 14 | 14 of 14 |
| Given the secret: moved, and all six read back | yes, in 326–396 ms | yes, in 757 ms |
| LocalKey records left in Lockstore's database and its log | 0 and 0, with patch 9 | 0 and 0 |

Steps 1 to 5 ran with patches 1 to 8 on the S9, and step 6 with patch 9 too; all six ran with patches 1 to 9 on the
A20. A normal session left its marker in the same three files as W13's: Fenix's saved tabs, its history's WAL, and
`serviceworker.txt`.

**Locked at start, nothing is razed, and the session runs without what needs a key.** The cookie store opened no
database and ran in memory, its file kept; site storage could not start; permissions ran in memory; and the HTTP
cache, with no cipher and no disk cache, answered from the network. The service worker's registration read back,
from `serviceworker.txt`, which none of Lockstore's switches is for (W13). Every store read back whole at the next
start with the secret.

**A lock mid-session stops only what needs a new key.** The cookie store's database and the HTTP cache's key, held
since the start, kept writing a new origin's data, which read back at the next start. A new origin's site storage,
and its service worker, failed: their keys could not be made. Each SQLite connection keeps its key, and so does the
cache. Locking the KEK is not locking what Gecko has open.

**A profile made without the secret is in the clear until given it.** On the G6, whose vault stays locked, the app
learnt so after 1.9 s, Gecko made no profile KEK, and every key of the profile went under LocalKeys, which keep
their keys beside them. On the S9 and the A20 a profile made with the secret withheld held 14 keys under three
LocalKeys; given the secret at the next start, Gecko made the KEK and moved all 14, in 326 to 396 ms on the S9 and
757 ms on the A20, deleted the three, and all six stores read back.

#### Moving the keys left them on disk (measured)

After that move, the S9's `lockstore.keys.sqlite` held no LocalKey, but `lockstore.keys.sqlite-wal` held 83 copies of
their records, `kek_bytes` and all, beside the old records of the keys they wrapped. A LocalKey wraps the same key the
move rewraps, so whoever reads the log reads every database: **moving a profile under the secret left it readable
without one.** SQLite's write-ahead log keeps each page as it was written until it writes over it. Lockstore's
`secure_delete` zeroes a record and deletes it in the database alone, and its own comment allows that the bytes "may
persist on disk until the relevant WAL pages are checkpointed and overwritten". Mozilla's adoption of a primary
password moves and deletes `local:profile` through the same `secure_delete` (read in the source).

Patch 9 has `secure_delete` checkpoint the log into the database and truncate it, and fail if a reader keeps it from
being reset; skv's `secure_delete` pragma already zeroes the database's freed cells. On the S9 and the A20 the log held
40 copies of the LocalKeys' records while they were live, and none after the move; the database held none either way.

#### What it costs (measured)

| | S9 | A20 | G6 |
|---|---|---|---|
| Gecko waits for the vault's answer | 0.64–0.71 s | 1.67–1.83 s | 1.9 s, to learn it is locked |
| The KEK unlocked, PBKDF2 at 600,000 iterations | 247–267 ms | 531–627 ms | — |
| The KEK made, and 14 wrappings moved | 326–396 ms | 757 ms | — |

Both come before Gecko opens a page, at every start.

#### What it means for stage 2

1. **A key from the vault protects all three of Lockstore's stores, but only with Gecko changed.** At 160 nothing
   lets an embedder give a secret before the profile opens, and moving the keys under one razes the cookie store and
   the HTTP cache. Patches 5, 7 and 9 are what it takes, and Mozilla should know all of it (`ROADMAP.md`, item 5).
2. **Gecko must not make a profile before the vault is open**, or the profile is in the clear until the next start
   with the secret moves it. The spike's Gecko waits for the vault's answer before the profile opens, and a locked
   vault gave its answer within two seconds.
3. **A start without the secret loses nothing but the session's own writes**: cookies and permissions set then are in
   memory alone, and a site's storage cannot be used. Whether encriox should start Gecko at all with the vault locked
   is open.
4. **A lock mid-session is not a lock of what is open.** It only stops what needs a new key, so the conservative
   answer to *Lock behaviour*, stopping Gecko at a lock, stands (`open-questions.md`). **Built by W20
   (2026-11-20):** the browser closes at a lock, and only the gate's `--es profile lock` still locks the keys
   mid-session.

After the round, each phone got a fresh Gecko profile, stage 1's APK and stage 1's config again (W13, flag 3). The
gate is left giving the secret, a setting stage 1's build does not read. No vault was locked, and none was touched but
to read the spike's own key.

#### Flagged for veto

1. **The secret is an HMAC of the vault's key, not the key itself**, so that the key that seals the logins' and the
   cards' keys (W6) is not Gecko's password; and it goes through Lockstore's PBKDF2 at 600,000 iterations, which 32
   random bytes do not need, at every start.
2. **Gecko starts with the profile's keys locked** when the vault is locked, has not let the browser in, or does not
   answer in 10 seconds, rather than waiting for it or refusing to start.
3. **A database whose key is locked fails to open as access denied**, obfsvfs's code for an open it cannot key, so
   that the cookie store keeps its file rather than moving it aside.
4. **A session locked from its start does not use the disk cache at all**, even once its keys are unlocked.
5. **No vault was locked to measure a lock.** The gate drops the vault's key instead, which is what
   `EncriKey` does at the vault's `Locked` event, because the G6's vault did not open again once the spike locked it
   (W5).

#### Not measured

- A lock of the vault itself; the S24.
- A secret that does not open the KEK, as a replaced `encriox.key` would give: Gecko logs it and stays locked, by
  the code; what encriox should then do is open.
- The time to make the KEK on a fresh profile; a private session; a third start; Mozilla's own tests of the patches.
- Whether a WAL truncated on flash is gone from it.
- What `obfsvfs` costs, stage 2's third round.

#### Rejected

- **A built-in extension holding the secret** (`open-questions.md`): it runs only once Gecko is up, after the
  databases have opened, and SQLite and the HTTP cache name their LocalKey.
- **The secret in the environment or GeckoView's extras**, which GeckoView copies into every child process, or in a
  pref, which content processes read.
- **Waiting for the vault on the app's main thread**: encri-client's binding answers there, so it would wait on
  itself. Gecko's own thread waits instead.
- **Locking a test vault to measure the vault's lock**, as above.
- **`NS_ERROR_FAILURE` for a locked key**, which the cookie store answers by moving its database aside.

<!-- amendments: generated by scripts/docs/decision-index.py; do not edit -->

## Amended since, as recorded in `amendments.md`

- **W14's second flag** (2026-11-18, W15): Gecko no longer starts with the profile's keys locked when the vault is
  locked or does not answer. On the user's word, the app brings up encri's unlock while Gecko waits, and ends before
  Gecko has opened anything of the profile if the vault stays shut. Only the gate's `kek withhold` starts it locked.
- **W14's fifth flag** (2026-11-18, W15): a vault was locked, on the user's word. The A20's, locked through the gate as
  encri's own *Lock* does, had the app lock Gecko's keys as the gate's `drop` had, and opened again with the
  passphrase on file, typed into encri's screen.
- **W14's lock mid-session** (2026-11-20, W20): the app no longer locks Gecko's keys at a lock and goes on. It
  closes the browser, letting Gecko write what it holds first, and only the gate's `--es profile lock` still locks
  them mid-session.

<!-- /amendments -->
