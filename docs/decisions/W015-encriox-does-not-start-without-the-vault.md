# W15 — encriox does not start without the vault: a locked vault at startup brings up encri's unlock, and a refusal ends the browser

*2026-11-18 · accepted on the user's word (below), built as stage 2's patch 0010 and measured on the A20, with the
refusal on the G6. **W14's five flags stand.** **Amends W14** on its flags 2 and 5. **Flagged for veto on four
points** (below). Answers `open-questions.md`'s *Starting Gecko with the vault locked*.*

#### What the user decided

After W14, the user answered its flags and the questions it left:

- "Nothing needed to veto. Looks good." W14's five flags stand.
- "Yes test a real vault. Feel free to wipe the vaults on any of the phones for testing if you need to." A test
  vault may now be locked to measure a lock, which W14's flag 5 had refused, and wiped if it does not open again.
- "Encriox should bring up the encri unlock if the vault is locked during startup. If the vault unlock fails, don't
  continue starting."

#### What was built

`scripts/spike/stage-2/`, patch 0010, on W14's nine:

- **`EncriUnlock`** shows encri's own screens when the browser needs the vault open and it is not: the one that lets
  the browser in, then the one that unlocks the vault, each once, through a translucent `EncriUnlockActivity`. They
  are started from whichever of the browser's activities is in the foreground, or from the next one to come there:
  Android lets an app start an activity from the background only on its old releases. Gecko's startup and the gate
  share one request, and wait on the same answer.
- **Gecko's startup asks it.** `EncriProfileSecret`, which Gecko asks for the profile's secret on its own thread
  (W14), asks the vault first, as before. Where the vault is locked or has not let the browser in, it asks
  `EncriUnlock` and waits for as long as encri's screens are up. If the vault is still shut once they are done, does
  not answer in 30 seconds, or refuses the browser's key, the browser finishes its activities, leaves the recent
  apps and ends its process, before Gecko has opened anything of the profile. The 10-second wait after which W14's
  Gecko started locked is gone.
- **The gate's `--es kek withhold`** still starts Gecko with the profile's keys locked: it is how the spike measures
  what Gecko does then, on a phone whose vault would open, and how the G6, whose vault does not, runs at all.

`check-unlock` runs it on a phone, in five steps (its header). It locks the vault through the gate's `--ez lock true`,
which locks it for every app and for encri as encri's own *Lock* does, and unlocks it by typing the passphrase on
file into encri's screen. It never prints the passphrase.

#### What it does (measured)

On the A20, `check-unlock` twice, all five steps each time:

| | A20, first run | A20, second run |
|---|---|---|
| The vault open at start: the secret given, asking nothing | after 1.7–1.8 s | after 1.4–1.9 s |
| The vault locked mid-session: the app locked Gecko's keys | yes | yes |
| A new origin while locked: written | cookie, session storage, HTTP cache | the same |
| ... and failed | local storage, IndexedDB, Cache API, service worker | the same |
| encri's unlock from the gate, the passphrase typed: Gecko's keys unlocked | yes | yes |
| Origins written before the lock and after the unlock: all six read back at the next start | but local storage, for the one written seconds before the lock (below) | yes |
| The vault locked at start: encri's unlock in front before Gecko opened anything | yes | yes |
| What changed in the profile while Gecko waited | Gecko's startup caches, and `.startup-incomplete`, empty | the same |
| Unlocked: the secret given, and all six read back | after 20.7 s, the script's typing among it | after 31.2 s |
| The vault locked at start, a wrong passphrase: encri's screen stays | yes | yes |
| Back: the browser stopped, its process gone, out of the recent apps | yes | yes |
| What changed in the profile | `.startup-incomplete` | that, and Gecko's startup caches |

The first run started the locked browser with `monkey`, which picked the debug build's other launcher entry,
LeakCanary's; the unlock came up over it all the same. The second started the browser's own entry, `.App`.

**On the G6**, whose vault is locked and does not open (below), the browser's own entry on a fresh profile brought up
encri's unlock over Fenix's home screen; Back ended the browser 37 s after Gecko asked, out of the recent apps. The
profile held NSS's `cert9.db` and `key4.db`, `compatibility.ini`, `times.json` and the lock files, all made before
Gecko asks for the secret, and no store's database, nor Lockstore's key store. A gate that finished before Gecko asked
left the unlock waiting "for the browser to come to the foreground", as built.

**A real lock reaches Gecko's keys as the gate's `drop` did** (W14): encri's `Locked` event dropped the vault's key,
and the app locked the profile's KEK. A lock mid-session again stopped only what needed a new key.

#### Found: a lock seconds after a page's write can lose its local storage (measured once, read in the source)

In the first run, the first origin's page wrote all its stores, and the vault was locked a few seconds later. At the
next start its IndexedDB, Cache API and service worker read back; its local storage did not, and the origin had no
`ls/data.sqlite`. In the second run, the origin written seconds before the lock kept it all.

Gecko's local storage keeps a page's writes in memory and flushes them 5 seconds later (`kFlushTimeoutMs`,
`dom/localstorage/ActorsParent.cpp`), and an origin's database is made at its first flush. Under the SQLite switch
a new database needs a new key from Lockstore, which cannot be made while the profile's KEK is locked: the flush
fails, and what it held never reaches the disk, though the page read it back in that session. IndexedDB and the Cache
API made their databases when the page wrote. ~~So a lock in the 5 seconds after an origin's first local storage write
of the session loses that write. In both runs the lock was the script's next step after the page; that the first fell
inside the 5 seconds and the second outside them is inferred, not timed.~~ **Amended by W20 (2026-11-20):** three runs
with the lock at most 1.5 s after the write, one of them this record's own script, kept local storage every time,
on an origin with all its stores and on one with local storage alone. The 5 seconds are not what lost it, and what
did is unknown.

#### The G6's vault

The passphrase on file, typed quoted so that its space and its `#` would reach encri, did not open the G6's vault, and
encri said nothing. One tap reached encri's *Unlock with fingerprint or screen lock*, which brought up the G6's pattern
lock, its owner's, and was backed out of. A vault made again there would ask for the screen lock too, as the S24's
will (encri D35, D74; `encri-core/docs/dev-environment.md`), so the G6's was not wiped: wiping it would not give the
spike a vault it can open. It stays locked, and runs the spike with `kek withhold`.

#### What it means for stage 2

1. **No profile is made, and none opened, without the vault**, in encriox: W14's point 2, that Gecko must not make a
   profile before the vault is open, holds by construction. Only a test control starts Gecko locked.
2. **A start can last as long as the user takes** to unlock: Gecko waits on its own thread, and Fenix's home screen
   drew under encri's unlock. No start was killed by Android for it, the longest 37 s.
3. **A refused start leaves the profile as it was**, but for an empty `.startup-incomplete` and Gecko's startup
   caches. Gecko reads that marker to count a crashed start; what it does with several on Android is unread.
4. **What a lock should do is still open** (`open-questions.md`, *Lock behaviour*), and the lost local storage is a
   reason to stop Gecko, letting it flush, before its keys are locked, rather than lock them under a running Gecko.

#### Flagged for veto

1. **Any failure to open the vault stops the browser**, not only a refused unlock: a vault that does not answer in 30
   seconds, encri absent or refusing the connection, or the browser's key in the vault refused, as one of the wrong
   length is (W6).
2. **The browser simply ends**: its activities are finished and its process killed, with nothing said of why beyond
   encri's own screen the user has just left.
3. **A start from the background waits for the browser to come to the foreground** before encri's screen is shown,
   and Gecko waits with it, for as long as that takes.
4. **The gate's `kek withhold` is kept**, a test control that starts Gecko with the profile's keys locked, which
   encriox never does.

#### Not measured

- The S9, which was not attached, and the S24.
- A start from the background, as a download or a push message would make; an unlock by fingerprint or the device's
  credential.
- What Gecko does after several refused starts, each leaving `.startup-incomplete`.
- ~~How often a lock meets local storage's 5 seconds; whether other stores defer writes the same way.~~ Moot since
  W20: the browser closes at a lock, letting Gecko write what it holds, and the 5 seconds did not lose local storage.

#### Rejected

- **Waiting a while, then starting locked**, W14's behaviour: the user's word.
- **A screen of the browser's own asking to unlock**: encri's own screens unlock the vault, as encri-api's contract has
  every companion do (encri D120).
- **A second unlock for the gate**: the gate and Gecko's startup share one request, so one screen comes up.
- **Wiping the G6's vault**: a new one there would need its owner's screen lock as well.

<!-- amendments: generated by scripts/docs/decision-index.py; do not edit -->

## Amended since, as recorded in `amendments.md`

- **W15's reading of the lost local storage** (2026-11-20, W20): not a lock inside local storage's 5 seconds. Three
  runs with the lock at most 1.5 s after the write kept it every time, on an origin with all its stores and on one
  with local storage alone; what lost it once is unknown, in a Gecko running on with its keys locked, which no longer
  happens.

<!-- /amendments -->
