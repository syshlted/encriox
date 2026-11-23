# W4 — Stage 1 holds on the G6: private tabs leave no page in plaintext, and the vault takes downloads at a third of Fenix's speed

*2026-11-14 · accepted as measured on the LG G6 alone, by the stage 1 spike (`scripts/spike/stage-1/`, W1), and
**flagged for veto on two points** (below). The S9, the A20 and the S24 are unmeasured. Nothing here is a design for
encriox yet: it is what the study's stage 1 does on a phone.*

#### What was built

Fenix at Firefox 160.0, built with Gradle against `geckoview-omni:160.0.20261104181552` from maven.mozilla.org, with
six patches (`scripts/spike/stage-1/patches/`):

- every tab private, a link from another app's included;
- a download written into the vault through encri-client's `write`, in the folder the grant gives;
- a throwaway package, `com.systemhalted.encri.browser.spike`, at API 27, encri's floor (encri D14), which
  encri-client declares and Fenix's 26 is below;
- StrictMode that logs and never kills, because on the G6 the debug build died of violations in LG's framework, in
  Android Components' notification for a finished download, and in another process's query of one of its providers.

**It builds with Gradle alone**, once `mach` has configured and run an artifact build, which downloads Mozilla's
prebuilt Gecko for the revision and generates sources Android Components needs (`CrashReport.kt`). Firefox 160 uses
AGP 9.4.1 and Kotlin 2.4.21, encri's own versions, so encri-api is included from source, as the companions include it
(encri D126). A clean build took about 8 minutes, after the artifact build's 6; a change to Fenix, about 10 seconds.

| What it costs on disk | |
|---|---|
| The Firefox checkout, shallow | 5.9 GB |
| The objdir, after the artifact build | 6.4 GB |
| `~/.gradle`, the first build's | 5.1 GB |
| `~/.mozbuild`: Mozilla's JDK 17, its SDK with the emulator configure insists on, Node, nimbus-fml | 2.1 GB |

#### Private tabs leave no page in plaintext (measured)

`check-plaintext` opens a page that writes a fresh marker into the cookie jar, local and session storage, IndexedDB,
the Cache API, a service worker and the HTTP cache, then reads it back. A minute after the app goes home, its whole data
directory is copied off the phone and searched for the marker, as ASCII and as UTF-16.

| Session | Files holding the marker |
|---|---|
| **Normal, the control** | 10: the HTTP cache, history (`places.sqlite` and its WAL), the saved tabs, the cookie WAL, local storage, IndexedDB, and the Cache API's database and two of its bodies |
| **Private** | none |

The control failed at first, and that is how its faults were found: the page was cacheable, so later runs loaded the
first run's page and marker; and ten seconds after going home, a normal session's cookies and storage were not on disk
yet.

What a private session does leave:

- **Gecko's private storage**, `storage/private/uuid+++…/`: local storage, IndexedDB and the Cache API, under a UUID for
  the origin, encrypted with keys held in memory (the study, [F3]). No marker was in it. It was gone after a restart.
- **A full-screen JPEG of each private tab**, in `no_backup/mozac_browser_thumbnails/private_thumbnails/`, unencrypted.
  One taken of the marker page stayed until the next start, which deleted it and wrote one of the new private home
  page. The study said stage 1 must remove these; this confirms it.

Not measured: media, so `browser.privatebrowsing.forceMediaMemoryCache` was set and not tested; extensions' own storage;
and what Android's task switcher keeps of the app's screen.

#### A download into the vault runs at about a third of Fenix's own writer (measured)

`measure` writes 64 MiB, alternating which goes first: from memory through each sink, which is the sink alone, and
through Fenix's download service and Gecko from a server on the phone's own loopback, so that no Wi-Fi is measured.
Fenix's own writer is the legacy file in `Download/` at API 28, and its times include no sync.

| G6, API 28, 64 MiB, four rounds | Fenix's own writer | Into the vault |
|---|---|---|
| Sink alone: copy | 172–379 MiB/s | 52–71 MiB/s |
| Sink alone: total, with create and commit | 0.23–0.59 s | 1.74–2.34 s |
| Through Gecko: copy | 103–151 MiB/s | 51–64 MiB/s |
| Through Gecko: total | 0.56–0.73 s | 1.73–1.83 s, about 35–37 MiB/s |

The vault's copy is as fast through Gecko as from memory, so encri's pipe and encryption set the pace, not Gecko. About
35 MiB/s end to end, on the oldest of the four phones, is likely faster than the connection a download comes over
(*unverified*: no network was measured).

#### What stage 1 has to do that the spike found

1. **Abandon a write that did not finish.** A paused or cancelled download leaves Android Components' copy loop and
   returns normally, and encri saves whatever a writer finishes (encri-api L11). ~~The spike's writer throws when fewer
   bytes arrived than the response promised, which abandons the write;~~ **Amended by W5**: a response need not give a
   length, and one that gave none was saved cut short; the writer also asks whether the download is still running,
   and a retry starts from the start. Fenix's own keeps a partial file to resume. A download into the vault cannot
   resume: `write` replaces a document whole.
2. **Turn off private thumbnails**, above.
3. **Set Gecko prefs some other way.** GeckoView has no public API for an arbitrary pref. The spike uses the YAML file a
   debuggable GeckoView reads from `/data/local/tmp`, which a release reads nothing from.
4. **Name the app.** encri's consent screen names an app by its label, which in the spike is still Fenix's own.

#### For encri

**A vault with no folders can only be allowed whole.** The G6's test vault had none, so encri's *Allow one folder
only…* offered nothing to choose, and encri itself cannot make a folder. The picker (DocumentsUI) said *"Can't load
content at the moment"* for encri's root a moment later, which was not followed up. The spike was allowed the whole
test vault, which held nothing.

#### Flagged for veto

1. **The whole of the G6's test vault was allowed to the spike**, against W1's one folder, because there was no folder
   to give it. The measurements do not depend on the scope.
2. **The spike's StrictMode never kills**, so a debug build of encriox would not die of the violations above. Release
   builds never kill on StrictMode. *(Amended by W26: encriox's own debug build ignores LG's framework alone, stage
   1's patch 0004, and otherwise kills as Fenix's does.)*

#### Rejected

- **`mach` bootstrap of the Android SDK**, which also builds an emulator image. The SDK packages were installed by hand
  into mach's own SDK, so the suite's SDK, encri's toolchain, is left as it is.
- **Measuring over Wi-Fi.** The A20's link ran at 0.4 MB/s; a download's speed would have been the network's.

<!-- amendments: generated by scripts/docs/decision-index.py; do not edit -->

## Amended since, as recorded in `amendments.md`

- **W4's first item of what stage 1 has to do** (2026-11-15, W5): the length a response promises is not enough to tell
  a stopped copy from a finished one. A download with no length, paused or cancelled, was saved cut short into the
  vault. The writer also asks whether the download is still running, and a retry starts from the start.
- **W4's, W5's, W6's and W21's private and sealed results** (2026-11-20, W23): they searched copies of the app's data that
  nothing checked were whole. Run again with `copy-data` on the G6, the A20 and the S24, they held; the S9's rest on
  the old copies until it is attached.
- **W4's second flag** (2026-11-23, W26): encriox's own debug build keeps Fenix's StrictMode, which kills, with LG's
  framework ignored, stage 1's patch 0004 carried as encriox's 0007; the spike's StrictMode that never kills stays in
  the spike.

<!-- /amendments -->
