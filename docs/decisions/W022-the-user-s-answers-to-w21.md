# W22 — W21's six flags stand: nothing in the clear at rest, a key the user may change, and a database that does not open is the user's to delete

*2026-11-20 · accepted on the user's word (below). **W21's six flags stand.** Adds two items to `ROADMAP.md`: a
prompt at start for a sealed database that does not open, in item 6, and changing a database's key, before the first
release. **Flagged for veto on three points** (below), one of them a question back to the user: Sync.*

#### What the user decided

After W21, the user answered its six flags:

1. "Are you referring to the encri-api library? What about PQC?"
2. "I assume that changing the key would require re-encryption? That's probably ok for now but perhaps we should
   provide the option of re-encrypting the database if it's thought that it was compromised?"
3. "No unencrypted data at rest. Yes."
4. "Can we add a notification on startup for the user to have the option of deleting it? If a user is logged into
   encriox with the mozilla.com account, sync should work, correct? This means we don't need to maintain a backup
   ourselves, correct?"
5. "Awesome great work!!"
6. "This is kind of the same functionality as ssh-agent or gpg-agent, correct? So it's sort of a necessary tradeoff,
   I presume?"

#### What each answer means

1. **AES-256-GCM stands.** The bindings W21 meant are application-services' own Rust wrapper over NSS, `rc_crypto`'s
   `nss` crate, not encri-api: it has AES-GCM and AES-CBC and no ChaCha20-Poly1305 (read in Firefox 160's tree).
   **Post-quantum, the chain from the vault to a page is symmetric throughout**: encri's containers are
   XChaCha20-Poly1305 under Argon2id and Keystore slots (encri-core D18, D31), `encriox.key` is 32 random bytes in
   the vault, each database's key is an HMAC-SHA-256 of it, and a page is sealed with AES-256-GCM. The quantum attack
   known on a symmetric cipher, Grover's, halves its key's bits, and leaves AES-256 at 128. Where post-quantum
   matters is the key exchange that could be recorded now and broken later: TLS. **Gecko 160 agrees X25519MLKEM768
   by default** (`security.tls.enable_kyber`), and on the A20 the spike's stage 1 build did, with TLS 1.3, at
   `https://pq.cloudflareresearch.com/cdn-cgi/trace` (measured 2026-11-20).
2. **One key for a database's life stands, for now, and changing it comes later** (`ROADMAP.md`, *Before the first
   release*). A new key means every page written again: SQLite's backup or `VACUUM INTO` a new sealed database under
   it, which then replaces the old, and the key's label carrying a count, so that the next start derives the new one.
   What it cannot do is take back what the old key already opened: whoever holds the old key and a copy of the old
   file keeps what it held, and flash memory may keep the old file's blocks after it is deleted. It protects what is
   written after. If `encriox.key` itself is in doubt, every key derived from it changes with it, Gecko's profile
   secret and the logins' and the cards' keys too, and W6's rule that the browser's key is never written over is
   amended for it: that is a rotation of its own, larger, and not decided here.
3. **No unencrypted data at rest**: a database in the clear is refused, never moved into a sealed one, and the rule
   holds for whatever encriox stores next.
4. **A sealed database that does not open is the user's to delete, through a prompt at start** (`ROADMAP.md`, item
   6; built and measured as W25, with a repair beside deleting and keeping, and a store the user kept not opened at
   all). Where places', the logins' or autofill's database cannot be opened under its key, as damaged or as a page whose
   tag fails, encriox says which, history and bookmarks, the logins or the cards and addresses, and offers to delete
   it and start that store empty, or to keep it and go on without that store. It is never deleted unasked. **Sync is
   off in the first build** (W17, point 3, "Firefox accounts and Sync"), so a deleted store does not come back, and
   the prompt says so. With Sync on, bookmarks, logins, cards and addresses would come back from the account, but
   history only its last 60 days, which is all a Sync server keeps of it (`HISTORY_TTL`, 5,184,000 seconds, read in
   application-services' places), and Sync is not a backup: a deletion syncs as a deletion.
5. **The megazord shares Gecko's NSS**, as Firefox's tree builds it.
6. **The keys stay in memory for the process's life**, as ssh-agent and gpg-agent keep a key decrypted while it is
   unlocked: SQLite reads and writes pages all session, each under the key, and the pages it has read sit decrypted
   in its cache beside it, so a key zeroed early would not take the data out of memory. The process ends at a lock
   (W20), as an agent's cache expires. `AGENTS.md`'s constraint 2 says it already: a key the browser holds sits in its
   memory while it runs.

#### Flagged for veto

1. ~~**The prompt cannot tell a wrong key from a damaged file**: under a wrong key every page fails its tag, as a
   damaged first page does. W6 keeps `encriox.key` from ever changing, and each database's key is derived from it, so
   a wrong key should not arise; the prompt treats both as damage.~~ **A key check tells them apart** (amended the
   same day, on the user's word, "yes, add the key check next session"): the user asked whether checks within the
   file could say it is whole. Each page's GCM tag already does, page by page, and SQLite's `integrity_check` does
   for what joins them; what no tag says is why one fails. Every sealed page gains, in the 4 reserved bytes that are
   zero now, a check derived from the database's key, an HMAC of a label of its own under it, which tells nothing of
   the key. At an open, a check that is not the key's means the wrong key: refused, and never offered for deletion. A
   check that is the key's, with tags that fail, means damage, and the prompt can say how many pages, and offer a
   third choice: SQLite's recovery copying every row it can still read into a new sealed database. **Built and
   measured as W24**: a key is refused as not the database's where no page carries its check and some page another's.
2. **Changing a key waits until after the first usable build**, and a rotation of `encriox.key` is not planned.
3. **Sync stays off, as W17 has it, and the user has asked about it**: turning Mozilla's accounts and Sync back on
   would restore a deleted store, but sends the browser's data, sealed with the account's keys, to Mozilla's servers,
   which W17 chose against. The user's to say. Without it, the other way to keep a copy is one in the vault: a
   snapshot of the bookmarks and the logins, written whole, as AGENTS.md's constraint 1 says such a file can be,
   which encri-backup then carries. Not planned.

#### Rejected

- **Deleting a database that does not open, unasked**, as places and autofill did with a corrupt one: W14's rule.
- **Rekeying at every lock or start**: every page written again, for nothing W21's GCM bound needs.

<!-- amendments: generated by scripts/docs/decision-index.py; do not edit -->

## Amended since, as recorded in `amendments.md`

- **W22's first flag** (2026-11-20, the user's word: "yes, add the key check next session"): a key check in each
  sealed page tells a wrong key from a damaged file, and the prompt offers a repair beside deleting and keeping.
- **W22's first flag, built** (2026-11-21, W24): as stage 3's patch 0008. A key is refused as not the database's where
  no page carries its check and some page carries another's, so that a page 1 damaged through its check is told by
  the next.
- **W22's prompt at start, built** (2026-11-22, W25): as stage 3's patches 0010 to 0017, measured on the G6. A damaged
  database may be repaired, deleted or kept, one under another key only kept, one in the clear deleted or kept; a
  kept one's store is refused its key that session, so that nothing opens it; nothing is deleted unasked.

<!-- /amendments -->
