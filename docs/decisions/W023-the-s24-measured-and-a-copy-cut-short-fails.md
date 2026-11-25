# W23 — Stages 1 and 3 hold on the S24, and a check whose copy of the app's data was cut short now fails

*2026-11-20 · measured on the user's word ("The S24 is connected. Let's complete the outstanding items for it"), on the
S24, the user's own phone, with nothing captured from its screen but encri's and the spike's, and its vault never
locked by a check. Ends `ROADMAP.md`'s owed S24 rounds of stage 1 (item 3) and of W21 (item 6). **Flagged for veto on
two points** (below).*

#### What was run

The S24 is a Galaxy S24 Ultra, Android 17, on wireless debugging. Its vault was made again after the one of
2026-11-17 was destroyed, and its owner unlocked it twice: after the first, encri's screen, brought up for the spike,
still said it was locked, with encri's process running throughout; why is not known. The spike was installed new:
stage 1's build, patch 0020's, then stage 3's, patch 0007's, then stage 1's again. `check-unlock`, which locks the
vault, was not run (`AGENTS.md`, *Hazards*).

#### Stage 1 (measured)

| | S24, API 37 |
|---|---|
| `check-plaintext normal`, the control | the marker in 11 files |
| `check-plaintext private` | in none; no private thumbnail on disk |
| `measure 64 4`, sink alone, copy: own / vault | 1,362–1,488 / 139–151 MiB/s |
| Through Gecko, copy: own / vault | 598–634 / 135–144 MiB/s |
| Through Gecko, total: own / vault | 0.15–0.16 / 0.50–0.52 s |
| Into the vault, end to end | about 123–128 MiB/s |
| `check-stopped`, the six cases of W19 | each as it should be; nothing in the vault of a stopped one, all 16,777,216 bytes of one resumed |
| `check-keys` | nothing in the clear; the logins asked first came after the key, 121 ms |

The vault takes a download at a quarter of Fenix's own writer's speed, which writes to MediaStore, and at about twice
the S9's (W5): 64 MiB in half a second. Fenix's own preferences of Keystore-sealed keys, `core_prefs*.xml`, are not
there at all: the spike was new on the S24, and never had Keystore's keys.

#### Stage 3 (measured)

| | S24 |
|---|---|
| `check-sealed unsealed`, the control | the marker in `places.sqlite`, its log, `logins2.sqlite` and autofill's log |
| `check-sealed` | in no file; the three databases and their logs with 8 KiB pages and 32 reserved bytes; the four read back in a new process; each opened without its key refused, and no file of the three changed |
| `check-keys` | nothing in the clear; the logins asked first came after their database's key, 36 ms |
| A database's key derived once the vault's was held | 0–2 ms |
| `check-kek` | every step held, its step 5 among them: the browser closed at the vault's key dropped, and the third origin read back whole |
| The real page, in a normal tab (`check-kek`'s first step) | the marker in Fenix's saved tabs and `serviceworker.txt` alone |

**What sealing costs places** (`bench-places`, as in W21, medians of six rounds):

| | plain | plain-8k | sealed |
|---|---|---|---|
| 2,000 visits | 5.37 s | 3.76 s | 3.81 s |
| 50 autocomplete queries | 129 ms | 130 ms | 130 ms |
| every visit read back | 10 ms | 11 ms | 10 ms |
| opening a new database | 27 ms | 9 ms | 31 ms |
| on disk | 4.5 MB | 2.0 MB | 2.1 MB |

**The cipher costs 1% of a visit's write on the S24**, and sealed places writes in 0.71 of plain's time: a visit
commits in 1.9 ms, against 2.7 ms plain, where the A20 takes 33 ms (W21). Its first run stopped, frozen by Android
(*Found on the way*, 3).

**Pages over TLS agree X25519MLKEM768**, Gecko 160's hybrid post-quantum key exchange, with TLS 1.3, with the megazord
on Gecko's NSS: `https://pq.cloudflareresearch.com/cdn-cgi/trace`, in stage 3's build. The A20 agreed it too, in stage
1's (W22).

#### Found on the way, and fixed

1. **The checks' copy of the app's data failed without a word.** `check-plaintext`, `check-keys` and `check-sealed`
   copied the app's data through `adb exec-out` and the phone's tar, every error thrown away. On the S24, WorkManager
   started the app during `check-keys`' copy, a file went from under tar, tar's error rode the one stream `exec-out`
   has into the archive, and the copy ended before `shared_prefs/`: the check found no key and no marker, and said
   nothing. **A copy cut short reads as one with no marker in it**, which is the result a private or a sealed run
   looks for. `copy-data` copies through `adb shell -T`, which keeps tar's exit status and errors apart, names a file
   that went as tar ran, and fails on anything else. Made to fail on the G6, on a path it could not read, it did.
2. **`check-keys`' new process was not stopped**, so a process WorkManager had started, which held the key already,
   answered the logins in 4 ms, as though they had not waited. It is stopped just before now, and the logins must
   come after a key read in that process: the vault's, or with stage 3 the logins' database's, which the app's own
   start asks for before the gate does, so that the gate then says its key is held.
3. **Android 17 froze the bench.** The gate finishes at once and `bench-places` runs on in the app's process in the
   background, as on every phone; Android 14 and later freeze a cached app's process, and the S24's first run stood
   still for ten minutes, the script waiting on, for up to an hour. It now keeps the process unfrozen for its life
   with `am unfreeze --sticky`, which changes no setting of the phone's, and fails a run whose process it finds
   frozen: frozen by hand mid-run, the run failed.

**The results fix 1 puts in doubt were run again**, with `copy-data`: `check-sealed` on the G6, the A20 and the S24,
and `check-plaintext private` on the G6, the A20 and the S24, stage 1's build again. Every one held: the marker in
no file. Every control run had found the marker, so the copies held then; what no run before had shown is that a
private or a sealed one's copy was whole.

#### Flagged for veto

1. **The S9's private and sealed results rest on copies nobody checked** (W5, W6, W21), until it is attached again:
   each control run there found its marker, so its copies held then, but nothing says a private or a sealed run's did.
2. **A file that goes while tar runs is left out of the search**, named: an app at the home screen still deletes its
   own files, and a check that failed on each would never pass. One that holds a marker and is deleted mid-copy would
   be missed; it is gone from the app's directory, not from the flash it was on.

#### Unmeasured

**A lock that reaches a browser Android has frozen.** The bench's freeze says a browser in the background on Android
14 or later is frozen too, its keys in its memory: the lock that closes it (W20) can run only once its process is
unfrozen, as it is when it comes to the front, and its keys stay in its memory until then. Measuring it locks the
S24's vault, which waits for its owner's word and its owner there to unlock it (`ROADMAP.md`, item 6).

#### Found for the checks

- **WorkManager starts the spike on the S24 by itself**, as on the A20 (W20): `am_proc_start … SystemJobService`,
  during a copy over Wi-Fi.
- **With stage 3, the app's own start asks for the vault's key before the gate**: places', the logins' and autofill's
  stores ask for their databases' keys as the app starts, and the gate then finds the key held.
- **Fenix's crash prompt came up on the S24**, *Firefox Fenix crashed*, with no report pending under `Crash Reports`,
  as on the A20 the same day: closed with Back, nothing sent.
- **On the A20, stage 3's gate started on stage 1's profile**, and Gecko's launch guard refused it, as W14 found: a
  fresh profile comes first, which `check-sealed` makes and a gate started by hand does not.

<!-- amendments: generated by scripts/docs/decision-index.py; do not edit -->

## Amended since, as recorded in `amendments.md`

- **W20's and W23's unmeasured lock of a browser Android has frozen** (2026-11-25, W27): measured on the S24, at a
  lock by encri's end in the background: the frozen browser kept every key for 3½ minutes, until it was opened, then
  closed in a third of a second and kept what its page wrote. On the user's word, encriox keeps a foreground service
  while it holds the keys, and closes after five minutes in the background.

<!-- /amendments -->
