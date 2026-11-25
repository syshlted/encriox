# encriox — open questions

Questions a decision left open. One that is answered is struck through, followed by the record that answers it.
The first four were the study's *Open questions*, moved here when W1 was recorded on 2026-11-13.

- ~~**History.** Does stage 1 keep history at all, or only bookmarks and tabs? A private tab records no history, but a
  bookmark made from one goes into `places.sqlite` like any other, in plaintext: Fenix's menu does not ask which kind
  of tab it was (read in the source, 2026-11-16).~~ In encriox's first build no history is recorded, and bookmarks are
  off (W17); they come back with a sealed store of their own.
- ~~**application-services.** `places.sqlite` and the login key are outside Lockstore. Fix them in the fork, or wait for
  Mozilla? The login key is the easier half (read in the source, 2026-11-16): application-services takes the key for
  logins, and the one for addresses and cards, from a key manager the app supplies, and Fenix's keep theirs in
  `SecureAbove22Preferences`, which a release stores in plaintext (`forceInsecure = !Config.channel.isDebug`). A key
  from the vault can take their place in Kotlin alone. `places.sqlite` has no such hook. The spike seals both keys with
  the vault's key now (W6), so `places.sqlite` is what is left. encriox's first build records no history and has no
  bookmarks (W17), so it waits for after that build.~~ **Fixed in the fork, W21**: places', the logins' and
  autofill's databases are sealed whole, through an obfsvfs of application-services' own, with keys from the vault's,
  and the first build carries them.
- ~~**Logins saved from a private tab.** Gecko offers to save a login from a private tab, because
  `signon.privateBrowsingCapture.enabled` is on by default, and nothing found in Android Components' prompts or its
  login delegate skips one. So with every tab private, a saved login still reaches the logins database (read in the
  source, 2026-11-16; unmeasured). Its username and password are sealed with a key the vault's key seals since W6,
  but its origin, and when it was last used, are plain columns of application-services' schema.~~ Not offered:
  `signon.privateBrowsingCapture.enabled` is off by encriox's own default (W17).
- **Extensions.** A package (`.xpi`) is a plain file in the profile (the study, *Stage 2*), and nothing an extension
  stores was measured (W4). An extension runs outside private browsing, so what it stores is likely on disk in
  plaintext (*unverified*).
- **How the browser's own key is kept** (the study, *a vault-gated key*). There are three ways, and the first two need
  no change to encri:
  - **a document in its folder of the vault**, the study's: read after each unlock and wiped at a lock, but readable by
    any app allowed that folder or the whole vault, as each test vault was allowed to the spike (W4, W5), and every
    store it encrypts is lost if the document is deleted;
  - **that document wrapped by a Keystore key of the browser's own**, so that an app reading it gets nothing. Keystore
    unwraps 32 bytes once an unlock, which encri's invariant on bulk crypto allows;
  - **encri wrapping and unwrapping it for the package**, with a key that never leaves encri: no other app could unwrap
    it, and ending the grant could erase the browser's stores by destroying that key. It is a change to the contract,
    for encri-api and encri to decide (L2, L18).

  A key derived from the vault's own and handed to the browser would break encri's *keys never leave encri*. The spike
  keeps the first (W6, flag 1). ~~encriox's is still to choose.~~ encriox keeps the first too, under a hidden name,
  `.encriox.key`, for now (W10); the other two are weighed with app-level security controls (`ROADMAP.md`, *Later*).
- ~~**Web compatibility.** Is Gecko's web compatibility acceptable for the people encri is for?~~ Taken as
  acceptable by W1, flag 2.
- ~~**Lock behaviour.** On `STATE_LOCKED`, does the browser close outright, or keep running tabs in memory with
  persistence paused? Closing is the conservative answer. The logins' and the cards' stores keep the keys they were
  given until the process ends, so a browser that keeps running after a lock keeps those keys in memory too; the
  spike wipes only its own key at a lock (W6). Gecko's do the same: with the profile's KEK locked mid-session, the
  cookie store's database and the HTTP cache's key, held since the start, went on writing, and only what needed a new
  key failed (W14), at a real lock of the vault too (W15). Locking the KEK is not locking what Gecko has open. And it
  can lose what Gecko has not written yet: local storage flushes a page's writes 5 seconds later, and an origin's
  first flush needs a new key, so a lock in between lost them on the A20 (W15). Stopping Gecko, and letting it flush,
  before its keys are locked would not.~~ encriox closes at a lock (W17): it shuts Gecko down, letting it write
  what it holds, then locks the profile's keys and ends, and the next start asks for the unlock. **Built in the
  spike, 2026-11-20 (W20)**: Gecko ends the process itself once it has shut down, within 4 seconds on the A20. The
  local storage loss was not seen again, nor caused by the 5 seconds: what caused it is unknown.
- ~~**Starting Gecko with the vault locked** (W14, flag 2). The spike starts Gecko with the profile's keys locked when
  the vault is locked or does not answer in 10 seconds: cookies and permissions are kept in memory alone, sites'
  storage cannot be used, and nothing is razed. Should encriox start Gecko at all then, or wait for the vault, or ask
  for it first? A profile made before the vault first opens is in the clear until it is given the secret (W14).~~ It
  asks for it first, and does not start without it (W15): encri's unlock comes up while Gecko waits, and a refusal
  ends the browser before Gecko has opened anything of the profile.
- ~~**A secret that does not open the profile's KEK**, as a replaced `encriox.key` would give: every key of the profile
  is out of reach for good. Gecko logs it and starts locked, keeping everything (read in the source; unmeasured).
  Whether encriox then starts a new profile, and what it tells the user, is open.~~ encriox says so and stops, and
  offers behind a confirmation to start a new profile (W17).
- ~~**The name and the package** (W1). The name cannot contain "Firefox" (`AGENTS.md`, hard constraint 3).~~ The name
  is encriox (W3), and the package `com.systemhalted.encriox` (W8), permanent from the first build a user installs.
- ~~**The branch for stage 2**, release or ESR (W1). It is picked before stage 2 starts.~~ Release (W7). Lockstore and
  its prefs are in the GeckoView Mozilla published at 160 (W5), so on release its first measurements need no Gecko
  build.
- ~~**How the fork is carried**: a series of patches applied to a pinned `firefox-main`, or a fork of its history.~~ A
  series of patches on each release tag, followed automatically up to a tested build (W9, flag 1). The stage 1 spike
  carried six patches at W4 and nineteen at W6, written out with `git format-patch` and applied by its `fetch` with
  `git am`; how that fares against a two-week rebase is unmeasured.
- ~~**encriox's version**: its version name and code, beside the Firefox release it is built on (W9). Fenix makes its
  version code from the build's time (`generateFennecVersionCode`), counting hours since 2014-12-28 in 17 bits, a
  little under 15 years by its own comment, and the registry's floor is a version code (encri-api L15).~~ Fenix's
  version code, and a version name of the Firefox release and encriox's own count, as *161.0-1* (W17).
- **For Encri Files: a name that begins with a dot** (W10, flag 2). Nothing in the suite hides one, so `.encriox.key`
  is listed in Encri Files and Encri Images like any other document. Whether to hide such names, and to show them on
  request, is Encri Files' to decide.
- ~~**How a release sets Gecko prefs** (W4), such as `browser.privatebrowsing.forceMediaMemoryCache`: GeckoView has no
  public API for one, and the spike's YAML file is read by a debuggable build alone. W5 found in the source that a
  file the app names through `GeckoRuntimeSettings.Builder.configFilePath` is read whatever the build; that is
  unmeasured in a release build.~~ As the build's defaults, in encriox's own Gecko (W17): no config file is
  read.
- ~~**How stage 1 stops private thumbnails** (W4): Android Components writes a full-screen JPEG of each private tab to
  `no_backup/`, unencrypted, until the next start.~~ Kept in memory instead, by W6 (patch 0013).
- **What of Fenix's that reaches Mozilla or another app stays in encriox.** Glean's telemetry and Nimbus, whose
  experiments can turn a feature on remotely, both left files in the spike's data on the S9 (W5). *Listen to Page*,
  off by default in every channel at 160, hands a page's text to the system's speech engine, writes its audio to a
  cache on disk, and plays it through Media3's ExoPlayer, which reports to the platform's media metrics from API 31
  unless `setUsePlatformDiagnostics(false)` (read in the source, 2026-11-15; Encri Images turned it off in its own
  viewer, encri-images d8de9cf). In a private session on the G6, Fenix fetched Mozilla's sponsored tiles from
  `ads.mozilla.org` and Nimbus's experiments from Mozilla's settings server, outside private browsing: both landed in
  Gecko's normal HTTP cache, the tiles in `moz_ads.sqlite` too, and the normal cookie jar's WAL changed, none of it
  holding the page's marker (W6). Mozilla's half is answered: off, but for Remote Settings' lists and Safe Browsing
  (W17). *Listen to Page* and ExoPlayer's diagnostics are still to look at.
- **For encri: a vault with no folders can only be allowed whole** (W4), and encri cannot make a folder itself.
- **For encri: Samsung's process manager kills encri in the background**, which locks the vault (W20). On the A20,
  with its memory reported low, Chimera killed encri four times in eleven minutes while the browser sat at the home
  screen, and encriox, which closes at a lock, closed each time. Whether encri keeps itself alive while it is unlocked,
  with a foreground service, say, is encri's to weigh; how often it happens on the other phones is unmeasured. The
  S24 does it too (W27): its low-memory killer ended encri frozen in the background with the browser, 4 minutes after
  both were frozen. Once encriox keeps a foreground service while it holds the keys, encri, bound by it, is not frozen
  or ended in the background either, and the vault stays open until a lock: whether it should lock after a time of its
  own is then encri's question. And whether the S24's swap reaches its storage, which bears on encri's key in memory,
  is to be read.
- ~~**A key-encryption key from the vault without a Gecko build.**~~ No (W14): an extension runs only once Gecko is up,
  after the profile's databases have opened, and at 160 SQLite and the HTTP cache ask for their keys under their own
  LocalKeys by name, so keys moved under a password KEK cannot be found. Stage 2's Gecko takes the secret at startup
  instead. The question as it was: at 160 GeckoView has no Java API for Lockstore, and
  Lockstore's key-encryption keys are `local`, kept in the profile, `pkcs11`, or `password`. But `nsILockstore` is
  scriptable (`@mozilla.org/security/lockstore;1`), and GeckoView counts a built-in extension as privileged, which
  lets it declare `experiment_apis`, privileged script (`Extension.getIsPrivileged`). So a built-in extension of
  encriox's could move the profile's data keys under a `password` key whose secret comes from `.encriox.key`, unlock
  it after each unlock of the vault, and lock it at a lock, as `ProfileKek.cpp` moves them for a primary password.
  What a store opened at start does while that key is still locked is stage 2's question (read in the source,
  2026-11-17; unmeasured). If it holds, W1's item 4 asks Mozilla for less. At 160 it would protect the HTTP cache
  alone, the one switch that works with a pref (W12); with stage 2's Gecko, all three (W13). On the S9 the profile's
  key store held the cache's key and each origin's, under two `local` keys, one QuotaManager's and one named
  `profile`.
- ~~**Lockstore's two other switches** (W12). SQLite's is locked off in `all.js`, and site storage's records an
  encrypted origin as plaintext, so the next start cannot read it. Build Gecko with both changed, or wait for
  Mozilla? The fix to the second is one line, in QuotaManager's `GetOrCreateTemporaryOriginDirectory`.~~ **Built,
  2026-11-18 (W13)**: stage 2's Gecko carries the fix, a few lines, and SQLite's switch on by default, and all three
  switches work on the G6, the S9 and the A20.
- **Who asks Mozilla, and where** (W1, item 4). Bug 2055641 on Bugzilla is where the OS-keystore KEK is filed. Theirs
  to know too: Android Components' logins and autofill stores wait on themselves for good once their key is lost, or
  is new over an existing logins database, which the spike fixed in patch 0018 (W6); QuotaManager records an
  encrypted origin as plaintext, at 160 and on `main` (W12), which stage 2's patch 0001 fixes (W13); and Gecko's
  launch guard reads the SQLite switch before any pref is loaded, so a profile a pref encrypted is refused at its next
  start (W13).
