# encri-firefox — open questions

Questions a decision left open. One that is answered is struck through, followed by the record that answers it.
The first four were the study's *Open questions*, moved here when W1 was recorded on 2026-11-13.

- **History.** Does stage 1 keep history at all, or only bookmarks and tabs?
- **application-services.** `places.sqlite` and the login key are outside Lockstore. Fix them in the fork, or wait for
  Mozilla?
- ~~**Web compatibility.** Is Gecko's web compatibility acceptable for the people encri is for?~~ Taken as
  acceptable by W1, flag 2.
- **Lock behaviour.** On `STATE_LOCKED`, does the browser close outright, or keep running tabs in memory with
  persistence paused? Closing is the conservative answer.
- **~~The name and~~ the package** (W1). ~~The name cannot contain "Firefox" (`AGENTS.md`, hard constraint 3).~~ The
  name is encriox (W3). The package is permanent from the first build a user installs.
- **The branch for stage 2**, release or ESR (W1). It is picked before stage 2 starts. Lockstore and its prefs are in
  the GeckoView Mozilla published at 160 (W5), so on release its first measurements need no Gecko build.
- **How the fork is carried**: a series of patches applied to a pinned `firefox-main`, or a fork of its history. The
  stage 1 spike carries six patches, written out with `git format-patch` and applied by its `fetch` with `git am`
  (W4); how that fares against a two-week rebase is unmeasured.
- **How a release sets Gecko prefs** (W4), such as `browser.privatebrowsing.forceMediaMemoryCache`: GeckoView has no
  public API for one, and the spike's YAML file is read by a debuggable build alone. W5 found in the source that a
  file the app names through `GeckoRuntimeSettings.Builder.configFilePath` is read whatever the build; that is
  unmeasured in a release build.
- **How stage 1 stops private thumbnails** (W4): Android Components writes a full-screen JPEG of each private tab to
  `no_backup/`, unencrypted, until the next start.
- **What of Fenix's that reaches Mozilla or another app stays in encriox.** Glean's telemetry and Nimbus, whose
  experiments can turn a feature on remotely, both left files in the spike's data on the S9 (W5). *Listen to Page*,
  off by default in every channel at 160, hands a page's text to the system's speech engine, writes its audio to a
  cache on disk, and plays it through Media3's ExoPlayer, which reports to the platform's media metrics from API 31
  unless `setUsePlatformDiagnostics(false)` (read in the source, 2026-11-15; Encri Images turned it off in its own
  viewer, encri-images d8de9cf).
- **For encri: a vault with no folders can only be allowed whole** (W4), and encri cannot make a folder itself.
- **Who asks Mozilla, and where** (W1, item 4). Bug 2055641 on Bugzilla is where the OS-keystore KEK is filed.
