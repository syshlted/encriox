/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// A toy stand-in for NSS's AES-256-GCM, for the harness alone: a keystream and a tag from a keyed hash. It seals
// nothing; it fails to open whatever another key sealed, or whatever was changed, as AES-GCM does.
pub(crate) const NONCE_BYTES: usize = 12;
pub(crate) const TAG_BYTES: usize = 16;

pub(crate) struct Cipher {
    key: [u8; 32],
}

fn mix(mut h: u64, b: u64) -> u64 {
    h ^= b;
    h = h.wrapping_mul(0x100000001b3);
    h ^ (h >> 29)
}

impl Cipher {
    pub(crate) fn new(key: &[u8; 32]) -> Option<Self> {
        Some(Self { key: *key })
    }

    fn stream(&self, nonce: &[u8; NONCE_BYTES], data: &[u8], out: &mut [u8]) {
        let mut h = 0xcbf29ce484222325u64;
        for b in self.key.iter().chain(nonce.iter()) {
            h = mix(h, *b as u64);
        }
        for (i, (d, o)) in data.iter().zip(out.iter_mut()).enumerate() {
            h = mix(h, i as u64);
            *o = d ^ (h as u8);
        }
    }

    fn tag(&self, nonce: &[u8; NONCE_BYTES], sealed: &[u8]) -> [u8; TAG_BYTES] {
        let (mut a, mut b) = (0x9e3779b97f4a7c15u64, 0xc2b2ae3d27d4eb4fu64);
        for x in self.key.iter().chain(nonce.iter()).chain(sealed.iter()) {
            a = mix(a, *x as u64);
            b = mix(b, a ^ *x as u64);
        }
        let mut t = [0u8; TAG_BYTES];
        t[..8].copy_from_slice(&a.to_le_bytes());
        t[8..].copy_from_slice(&b.to_le_bytes());
        t
    }

    pub(crate) fn seal(&self, nonce: &[u8; NONCE_BYTES], data: &[u8], out: &mut [u8]) -> bool {
        if out.len() != data.len() + TAG_BYTES {
            return false;
        }
        let (body, tag) = out.split_at_mut(data.len());
        self.stream(nonce, data, body);
        tag.copy_from_slice(&self.tag(nonce, body));
        true
    }

    pub(crate) fn open(&self, nonce: &[u8; NONCE_BYTES], sealed: &[u8], out: &mut [u8]) -> bool {
        if sealed.len() < TAG_BYTES || out.len() < sealed.len() - TAG_BYTES {
            return false;
        }
        let (body, tag) = sealed.split_at(sealed.len() - TAG_BYTES);
        if self.tag(nonce, body) != tag {
            return false;
        }
        self.stream(nonce, body, &mut out[..body.len()]);
        true
    }

    pub(crate) fn random(buf: &mut [u8]) -> bool {
        use std::sync::atomic::{AtomicU64, Ordering};
        static N: AtomicU64 = AtomicU64::new(0x1234_5678);
        for b in buf.iter_mut() {
            *b = (mix(N.fetch_add(1, Ordering::Relaxed), 7) >> 13) as u8;
        }
        true
    }
}
