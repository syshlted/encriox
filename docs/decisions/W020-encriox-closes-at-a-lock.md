# W20 — encriox closes at a lock: Gecko writes what it holds, and every process of the session ends within 4 seconds

*2026-11-20 · built and measured on the user's word ("We should fix these before continuing, if they are not fixed
yet"), as stage 2's patch 0012, on the A20; W17's first recommendation, in the spike. Ends `ROADMAP.md`'s S3. S1 was
not seen again in three tries, and the state it was seen in no longer occurs. **Flagged for veto on three points**
(below). ~~The G6's vault is locked and the S9 was not attached: both are unmeasured.~~ **`check-kek`'s step 5
measured on the G6, 2026-11-20** (below), whose vault is not locked by any check; the S9 was not attached, and a
real lock on the G6 is unmeasured.*

#### What was wrong

At a lock of the vault the spike locked Gecko's keys and went on running (W14). The logins' and the cards' keys stayed
in memory until the process ended (S3, W6 flag 3), and once, a page's local storage written seconds before a lock was
lost (S1, W15).

#### What was settled

1. **At a lock the browser closes** (patch 0012): when the vault's key is dropped, the app finishes its windows and
   takes them out of the recent apps, then shuts Gecko down with `GeckoRuntime.shutdown()` while Gecko still holds the
   profile's keys, so that what it keeps in memory to write later reaches the disk sealed. Gecko ends the process
   itself, at the fast shutdown's last stage (`toolkit.shutdown.fastShutdownStage`, after QuotaManager's shutdown,
   which flushes local storage: `Connection::Close` in `dom/localstorage`).
2. **If Gecko has not ended it in 10 seconds, the app ends it anyway.** On the A20 it never came to that.
3. **Android Components' engine is told nothing.** It throws at a shutdown it did not ask for, to hear of it as a
   crash, so the runtime's delegate is replaced before the shutdown.
4. **Nothing locks Gecko's keys mid-session any more** but the gate's `--es profile lock`, which measures what a
   locked session does (W14). The next start asks for the vault (W15) and reads the profile back whole.
5. **The checks follow it.** `check-unlock` locks the vault as soon as a new origin's page has written local storage
   and nothing else, so that the origin has no directory, key or database until local storage flushes; it requires
   every process of the session that held its pages or keys gone within 30 seconds, the browser out of the recent
   apps, and that local storage read back; it lists what the close wrote. `check-kek`'s step 5 requires the close
   where it required the keys locked. Their settles keep the browser in front (*Found*, below).

#### Measured (2026-11-20, the A20)

| | Before, patch 0011's build | After, patch 0012's |
|---|---|---|
| At each lock | Gecko's keys locked; 6 or 7 processes still running 30 s later, the app's own among them, in six locks of two runs | the browser closed: the app's, the content and the GPU processes gone in 2.2–3.8 s, in three locks |
| In the recent apps after | — | no |
| Local storage written at most 1.5 s (before) or 1.9 s (after) before the lock, on an origin with all its stores, and on one with local storage alone | read back, both | read back |
| `check-unlock` | failed | passed |

`check-kek`, whose step 5 now drops the vault's key and requires the close, did not get past its first step: twice,
encri was killed in the background during `check-plaintext`'s minute at the home screen, which that script keeps
there by design (*Found*, below), so the vault locked and the browser closed before the read-back. ~~Its step 5 is
*unmeasured*;~~ `check-unlock` measures the same close at a real lock. **On the G6** (2026-11-20, stage 3's build,
which carries patch 0012), `check-kek` passed every step: at step 5 the vault's key was dropped seconds after a third
origin wrote its seven stores, the browser closed, and a start with the secret read all six back, local storage among
them. On the S24 the same (W23). **Unmeasured there, and on any phone of Android 14 or later: a lock that reaches a
browser Android has frozen** in the background, as it froze the S24's bench (W23): its close cannot run until the
process is unfrozen, and its keys stay in its memory until then.

Watched by hand once more: the lock at 0 s, Gecko's thread exiting at 0.6 s, the app's process gone at 0.9 s and the
tab's at 1.1 s. A GPU process Gecko started as it shut down went at about 9 s, and the session's crash helper at about
16 s; in another run it was still there 43 s after. It holds no page and no key.

**What the close wrote**, within 2 seconds: the cookies', the Cache API's and Lockstore's write-ahead logs
checkpointed into their databases and left empty; QuotaManager's `storage.sqlite` and each origin's `.metadata-v2`,
the local storage origin's grown from 83 to 98 bytes; and Gecko's shutdown records, startup cache and shader cache.
Every database among them is sealed (W13) but Lockstore's, which holds only wrapped keys.

#### S1, not seen again

W15 lost a page's local storage once in two runs, and read it as a lock inside the 5 seconds local storage keeps a
page's first writes in memory, whose flush makes the origin's database. Three runs on patch 0011's build, two with the
lock at most 1.5 s after the write and one of W15's own script, kept it every time, on an origin with all its stores
and on one with local storage alone: a flush under locked keys made the database. So the 5 seconds are not what lost
it, and what did is unknown. It was lost in a Gecko running on with its keys locked, which no longer happens: the
browser closes instead, and its local storage, written just before the lock, read back.

#### Found: the A20 kills encri in the background, which locks the vault

In two of the runs on patch 0012's build, the vault locked while the browser sat at the home screen for a minute.
Samsung's process manager, Chimera, had killed encri's process (`am_kill … com.systemhalted.encri … Chimera #2`), four
times in eleven minutes, with the phone's memory reported low (2.7 GB), and twice more during `check-kek`, once as a
cached process 20 seconds after the home screen, with Encri Files and every other background app stopped first. A vault's key lives in encri's process, so the
vault locked, and the browser closed, as it should. In daily use on such a phone, encriox closes whenever encri is
killed in the background. That is encri's to weigh (`open-questions.md`). The checks' settles now keep the browser in
front, which keeps encri bound to a foreground app; none was killed in the runs after.

The A20 also starts apps it expects to be opened (`mllaunch`) and runs their scheduled jobs: 3 seconds after a close
it started the browser again in the background, where it waited at the locked vault before Gecko opened anything
(W15).

#### Flagged for veto

1. **The 10 seconds** before the app ends a shutdown that has not ended.
2. **Gecko's crash helper outlives the close** by 16 to 43 seconds. It holds no page and no key.
3. **S1 is closed unreproduced.** Three tries did not lose local storage, and the state it was lost in is gone.

#### Rejected

- **Locking Gecko's keys and going on**, as W14 did: the two bugs.
- **Ending the process without shutting Gecko down**, as a refused start does (W15): what Gecko holds in memory
  would be lost, and its write-ahead logs left as they are.

<!-- amendments: generated by scripts/docs/decision-index.py; do not edit -->

## Amended since, as recorded in `amendments.md`

- **W20's unmeasured step 5 of `check-kek`** (2026-11-20): measured on the G6, on stage 3's build, which carries patch
  0012. The browser closed at the vault's key dropped, and the third origin read back whole, local storage among it.
- **W19's, W20's and W21's unmeasured S24** (2026-11-20, W23): measured, on its owner's word, all as on the other
  phones; W20's real lock not, which would lock the S24's vault.
- **W20's and W23's unmeasured lock of a browser Android has frozen** (2026-11-25, W27): measured on the S24, at a
  lock by encri's end in the background: the frozen browser kept every key for 3½ minutes, until it was opened, then
  closed in a third of a second and kept what its page wrote. On the user's word, encriox keeps a foreground service
  while it holds the keys, and closes after five minutes in the background.

<!-- /amendments -->
