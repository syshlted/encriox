# W27 — A browser Android has frozen keeps its keys through a lock until it is opened, so encriox holds a foreground service while it has them, and closes after five minutes in the background

*2026-11-25 · measured on the user's word ("OK What needs to be done on the s24? I am available for your
assistance", then "The S24 is reconnected and free"), on the S24, its owner there, with nothing captured from its
screen and nothing kept of its log but the lines that name the browser or encri; then decided by the user, asked
whether encriox keeps itself from being frozen or closes in the background ("Service, then close"). Ends `ROADMAP.md`'s
item 6's measure of a lock reaching a frozen browser, W23's unmeasured and W20's. **Flagged for veto on five points**
(below).*

#### What was run

`scripts/spike/stage-2/check-frozen`, new, on stage 3's build at its patch 0017, which carries stage 2's close at a
lock (patch 0012), installed over stage 1's: `before` wrote a page's stores in a normal tab from a fresh start, gave it
a minute in front, went to the home screen and timed each process's freeze; `after` watched the frozen browser with
the vault locked, opened it from its launcher entry and timed what followed by the phone's log; `read` read the page
back once the owner had unlocked. The S24 is Android 17, on wireless debugging.

**The lock was not the owner's.** Asked to lock the vault in encri's own screen, the owner found it locked already:
Android's low-memory killer had ended encri's process, frozen in the background with the browser's, and the vault's key
lives in that process. The browser hears of a lock and of encri's death alike, as a message through binder, which a
frozen process takes only once it is unfrozen; the one measured is the death.

#### Measured (2026-11-25, the S24)

| | |
|---|---|
| From the home screen to frozen | 71 s: the browser's five processes, its own, Gecko's GPU process, its crash helper and two content processes, frozen within 10 ms of each other by Android's freezer (`am_freeze`), and encri's process with them. Gecko ended a third content process 0.8 s after the home screen |
| Before that | Samsung's own freezer logged *skipping freeze* for the browser and for encri every 6 s, from 2 s to 68 s; and once frozen, Android compacted each of the browser's processes' memory (`am_compact`) |
| encri, frozen in the background | 4 min 17 s after `before` ended, ended by `lmkd` (*Reclaim*, `oom_score_adj` 999), unfrozen to be ended; the vault locked with it. Android scheduled encri's companion service to start again, for the browser's binding to it. 0.6 s later the owner's tap on encri's icon started a new encri, its vault locked |
| The frozen browser, the vault locked | for 3 min 28 s, until it was opened, the last 2 min of them watched: every process there and frozen, and the app said nothing. The close had not run, and every key the browser holds was in its memory: the profile's secret, the stores' database keys and `encriox.key` |
| Its task in the recent apps meanwhile | there, with the picture of the page Android keeps for it |
| Opened from its launcher entry, from the tap | +0.07 s its activity resumed by Android, +0.09 s every process unfrozen, +0.26 s its window resumed, in front; +0.33 s *browser closes: the vault locked* and the window finished; +0.37 s encri's screen in front again; +0.66 s the app's process gone, and every process of the session but the crash helper within 1.1 s; out of the recent apps |
| The page's six stores, read back once the owner unlocked | all six: cookie, local storage, IndexedDB, the Cache API, the HTTP cache and the service worker |

**So the close works, late.** Opened, the browser closed as W20 measured it, in a third of a second, and kept what
the page wrote. But between the freeze and the opening, the vault's lock changed nothing: the browser's keys stayed in
its memory for as long as it stayed frozen, which on the S24 is for as long as its user does not open it, or until
Android ends it, which skips the close and loses what Gecko holds in memory to write (W20). Its window was in front for
70 ms before the close finished it: whether the page was drawn in that time is not known, as nothing of the screen
was captured.

#### Decided (the user's)

**encriox keeps a foreground service of its own for as long as it holds the vault's keys, and closes after five
minutes in the background.** The service keeps Android from freezing it, so a lock or encri's death reaches it at once
and it closes, as W20 measured; its notification can carry a Lock button. After five minutes with no window of its
in front, it closes as at a lock, so that no key outlives that in a process in the background. A visit to another app
shorter than that keeps the browser's session as it was. Built in the spike first, measured with `check-frozen`, then
carried with the spikes' patches that are encriox's (`ROADMAP.md`, item 6).

#### Found on the way

1. **The vault locked by itself on the S24, twice**: between 2026-11-23 and 2026-11-25, why unknown, and on
   2026-11-25 at `lmkd`'s end of encri's frozen process, above. As the A20's Chimera does (W20), so does the S24 then:
   encri ended in the background locks the vault, and the browser with it once it hears.
2. **Fenix asks to be the default browser**, through Android's role dialog over its page, after 4 cold starts, at most
   3 times, 14 days apart (Nimbus's `default-browser-prompt`): a fresh profile's starts by the checks reached 4, and the
   dialog sent the browser to the background before the home screen did. The owner cancelled it.
3. **An alarm's screen over the browser did the same** on 2026-11-23, and Android froze it in its minute in front.
   `check-frozen` now fails a run whose browser is not in front at the end of its minute.
4. **A gate started with encri's own screen in front, after the browser's process was stopped, never ran**: `am` said
   the intent was delivered to the top-most instance, and the app logged nothing. Started again with the vault open,
   it ran. Why is not known.
5. **The checks' own traps**: this machine's `awk` is mawk, which held back the lines of `adb logcat` it read from a
   pipe; Android 17 names the activity in front `topResumedActivity` and a recent task's `mActivityComponent`; `after`
   printed what happened between the steps before the phone's log from minutes before had arrived, and missed encri's
   death; and its read-back waited five minutes for the owner's passphrase, twice. All fixed in `check-frozen`, which
   gained `read`, the read-back alone.
6. **The G6's encri was reinstalled on 2026-11-23 at 21:09**, outside this session, and asks to let the spike in again.

#### Flagged for veto

1. **Five minutes** in the background before the close.
2. **The notification**, shown for as long as the browser holds the keys, in front or not, as a foreground service
   needs one; and **its Lock button**, which locks the vault for every app, as encri's own Lock does.
3. **encri kept running while the service runs**: an app bound by a foreground service is no longer cached, so encri
   is not frozen or ended in the background either, and the vault no longer locks by itself while the browser holds
   it, as it did on the S24 and the A20. Whether the vault locks after a time of its own is encri's to settle
   (`open-questions.md`, *For encri*).
4. **The service's type**: Android 14 and later make a foreground service name one, and none of Android's fits a
   browser holding keys but `specialUse`, which Play reviews for a reason given in the manifest.
5. **The spike first**, on 160.0, as stage 2's close is the spike's; encriox's series gets it with the rest of the
   spikes' Fenix code.

#### Rejected

- **The service with no time in the background**: a lock reaches the browser at once, but its keys stay in a process
  in the background for as long as the vault is open.
- **Closing as it goes to the background**, after a grace shorter than the freeze, which is 10 s by Android's default
  and was 71 s on the S24: no key in the background ever, but every visit to another app longer than the grace closes
  the browser, its next start is cold and asks for the vault, and private tabs are lost.
- **No change**: the keys stay in a frozen browser until it is opened, and an end by Android skips the close.

#### Unmeasured

- **A lock by encri's own Lock** reaching a frozen browser, which is a oneway call to it, not binder's notice of a
  death. Binder holds both for a frozen process, so the same is expected.
- **Whether the S24 writes swapped memory to its storage.** Android compacted the frozen browser's memory, and `lmkd`
  found 31 MB of encri's in swap. Samsung's RAM Plus can back swap with the phone's storage, under the phone's own
  encryption: whether the S24's does, and so whether keys in memory, encri's or the browser's, frozen or not, reach its
  flash, was not read before the S24 went offline.
- **What a frozen browser does when Android ends it**, as `lmkd` ended encri: the close does not run, by its design.
- **The other phones**: the G6 (Android 9) and the S9 (Android 10) have no freezer, and the A20's Android 11 has it
  only behind a developer setting; Android 14 turned it on.
