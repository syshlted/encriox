# W6 — A key of its own in the vault seals the spike's logins and cards, and a private tab leaves no thumbnail on disk

*2026-11-16 · accepted on the user's word ("add the findings and the[n] proceed"), after the first of
`open-questions.md`'s three ways to keep the browser's own key was recommended for the spikes, and **flagged for veto
on three points** (below). Built in the stage 1 spike (`scripts/spike/stage-1/`, W1) and measured on the S9, the A20
and the G6. **Answers W4's second item** of what stage 1 has to do. Nothing here is a design for encriox yet.*

#### What was built

Seven patches on top of W5's twelve:

- **0013, 0014**: a private tab's thumbnail is kept in memory, compressed as Android Components' disk cache kept it,
  and goes with the process. What an earlier build left on disk is still cleared at start.
- **0015**: Android Components' logins and autofill stores take any store of key material, through the interface
  behind `SecureAbove22Preferences`, made public. It gains `awaitReady()`, which the two key managers wait on before
  asking for a key.
- **0016**: the browser's own key, the study's *vault-gated key*: 32 random bytes in a document named `encriox.key`,
  in the folder the vault allows the spike. The gate reads it once the vault is open, or makes it there and reads it
  back before using it, and holds it in memory until the vault locks or the session ends. A document of the wrong
  length, two of the name, or one that does not read back are refused, never written over.
- **0017**: the logins' and the cards' keys sealed with it, AES-256-GCM, each bound to its name, in preferences of
  their own and committed rather than applied. Fenix kept them in `SecureAbove22Preferences`, which a release stores
  in plaintext beside the stores. The gate saves, lists and deletes a login and a card of the spike's own.
- **0018, 0019**: two faults the S9 found, below.

#### A private tab leaves no thumbnail on disk (measured)

`check-plaintext <phone> private` as in W4 and W5, which now lists any private thumbnail on disk.

| | G6, API 28 | S9, API 29 | A20, API 30 |
|---|---|---|---|
| A private tab's JPEG in `no_backup/`, before (W4, W5) | yes | 72,750 bytes | 45,807 bytes |
| Now | none | none | none |
| The marker page's thumbnail taken, to memory (log) | yes | yes | yes |
| Files holding the marker | none | none | none |

#### The logins' and the cards' keys are sealed with the vault's key (measured)

`check-keys <phone>` saves a login and a card, each a fresh marker, and searches the app's data for the markers and
for a key in the clear. A new process then asks for the logins before the vault is open, as a page with a login form
would, and must have them only once the vault's key arrives.

| | S9 | A20 |
|---|---|---|
| The browser's key | made, then read in every later run | made, then read |
| Markers or a key in the clear on disk | none | none |
| Logins asked for before the vault was open | arrived after the key, 401 and 411 ms later | after the key, 1,840 ms later |
| The login and the card, read back in a new process | whole | whole |

On the G6, whose vault is still locked (W5), the logins were asked for with the vault locked and encri's unlock
screen left unanswered. After more than a minute they had not answered, and nothing had been written: no key was made,
and the logins' and cards' canaries and databases were as they were on 2026-11-13. So with the vault locked the stores
wait, and their key managers make no new key over the old canaries, which would have wiped what those keys sealed.

#### Android Components' stores waited on themselves after a key loss (found, fixed, measured)

The first `check-keys` on the S9 never saved its login. The S9 still held the canaries of the keys Fenix had made on
2026-11-15, so with the keys moved to the new store, each key manager found a canary and no key, took the key as
lost, made a new one, and recovered by wiping the logins, or scrubbing the cards, through `getStorage()`. That awaits
the deferred that opens the store, which was waiting for the key manager. The store waited on itself, and every call
to it after that waited for good. The wipe for a new key over an existing logins database (Mozilla's bug 1970409) goes
through `getStorage()` too, and waits the same way.

This is in Android Components, not the spike, so Fenix itself would reach it whenever a key is lost (read in the
source). The store now wipes or scrubs the connection it has just made, and recovery asked for from elsewhere, such as sync's key providers, does so
only once the store is open (patch 0018). With the sealed keys deleted and the canaries left, as on that first run,
the S9 then saved both. The A20 took the same path on its own, from Fenix's keys of 2026-11-15, and saved both too.

**A value the vault's key cannot open is answered as gone** (patch 0019). The spike's store threw on one, which
happens once `encriox.key` is deleted from the vault and the gate makes a new key. The stores would then have failed
for good with an exception their callers do not catch, and saving a login could have taken the app down.

#### Another fault in the spike, fixed

**`check-plaintext` waited for good** on the A20, whose screen had gone off behind its lock screen: `am start -W` waits
for the page to be drawn. It also let a page that had not reported every store go on, so a private run could have
found no marker for want of one written. The page now has two minutes to open, and every store must report, or the
run stops with a message; on the A20 with its screen locked it stopped after 166 seconds.

#### Found, not measured

- **A lock wipes the browser's key, not the stores'.** The logins store holds its key for as long as the process
  runs, and the autofill store's key manager is static. A browser that keeps running after a lock keeps them in memory
  (`open-questions.md`, *Lock behaviour*).
- **Not everything in the two stores is encrypted.** A login's username and password are, and its origin and when it
  was last used are plain columns; a card's number is, and the name on it is not, nor is any address
  (application-services' schemas).
- **Fenix's own traffic in a private session**: on the G6, Mozilla's sponsored tiles from `ads.mozilla.org` and
  Nimbus's experiments from Mozilla's settings server were fetched outside private browsing, and landed in Gecko's
  normal HTTP cache, the tiles in `moz_ads.sqlite` too; the normal cookie jar's WAL changed. None of it held the page's
  marker (`open-questions.md`).

#### For encri

- **The spike left `encriox.key`, 32 bytes, at the root of the S9's and the A20's test vaults**, which each allowed
  whole (W5, flag 1). The spike deletes the logins and the cards it saves, and never the key.
- **encri's unlock screen was shown on the G6**, and left unanswered; its vault is still locked.

#### Flagged for veto

1. **The key is a document in the vault, the first of the three ways**, readable by any app allowed its folder or the
   whole vault, as each test vault is allowed to the spike. The second and the third stay open for encriox.
2. **A key the vault's key cannot open is taken as lost**, as Fenix takes one Keystore cannot: the key managers make
   new keys, and the stores wipe what the old ones sealed. Deleting `encriox.key` from the vault wipes the spike's
   logins and cards the next time the stores open.
3. ~~**A lock leaves the stores' keys in memory** until the process ends, as above. It is a limit of the spike until
   *Lock behaviour* is answered.~~ **Amended by W20 (2026-11-20):** the browser closes at a lock, and the process
   ends with every key it held.

#### Rejected

- **Answering that nothing is stored while the vault is locked**: the key managers would make new keys over the old
  ones and wipe every login and card, the path the S9 took when its keys moved.
- **Blocking a thread until the vault opens**, inside the store's read: a page with a login form would hold a thread
  of the IO pool on every load until then. `awaitReady()` suspends instead.
- **Deriving the stores' keys from the vault's key**: application-services' keys are JSON Web Keys it makes itself,
  and sealing them leaves its stores as they are.
- **Throwing on a value the vault's key cannot open**, above.
- **Locking the S9's or the A20's vault to measure the locked path**, as in W5. The G6's, already locked, measured it.

<!-- amendments: generated by scripts/docs/decision-index.py; do not edit -->

## Amended since, as recorded in `amendments.md`

- **W6's third flag** (2026-11-20, W20): a lock no longer leaves the stores' keys in memory. The browser closes at a
  lock, and its process ends with every key it held.
- **W6's logins' origins** (2026-11-20, W21): each login's origin and times, which W6 left as plain columns of the
  logins' database, are sealed with the database now, under a key of their own from the vault's.
- **W4's, W5's, W6's and W21's private and sealed results** (2026-11-20, W23): they searched copies of the app's data that
  nothing checked were whole. Run again with `copy-data` on the G6, the A20 and the S24, they held; the S9's rest on
  the old copies until it is attached.

<!-- /amendments -->
