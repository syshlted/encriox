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
- **The branch for stage 2**, release or ESR (W1). It is picked before stage 2 starts.
- **How the fork is carried**: a series of patches applied to a pinned `firefox-main`, or a fork of its history. The
  stage 1 spike carries six patches, written out with `git format-patch` and applied by its `fetch` with `git am`
  (W4); how that fares against a two-week rebase is unmeasured.
- **How a release sets Gecko prefs** (W4), such as `browser.privatebrowsing.forceMediaMemoryCache`: GeckoView has no
  public API for one, and the spike's YAML file is read by a debuggable build alone.
- **How stage 1 stops private thumbnails** (W4): Android Components writes a full-screen JPEG of each private tab to
  `no_backup/`, unencrypted, until the next start.
- **For encri: a vault with no folders can only be allowed whole** (W4), and encri cannot make a folder itself.
- **Who asks Mozilla, and where** (W1, item 4). Bug 2055641 on Bugzilla is where the OS-keystore KEK is filed.
