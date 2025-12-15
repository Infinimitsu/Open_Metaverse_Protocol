use ed25519_dalek::{Signer, SigningKey, VerifyingKey, Signature};
use rand::rngs::OsRng;

pub struct IdentityManager {
    keypair: SigningKey,
}

impl IdentityManager {
    pub fn new() -> Self {
        // In a real app, we would load this from disk (the "Wallet").
        // For now, we generate a fresh identity every time (Ephemeral).
        let mut csprng = OsRng;
        let keypair = SigningKey::generate(&mut csprng);
        
        println!("[Identity] Generated new Identity.");
        Self { keypair }
    }

    pub fn get_public_key_string(&self) -> String {
        let pk: VerifyingKey = self.keypair.verifying_key();
        // Convert bytes to hex string for easy display
        hex::encode(pk.to_bytes())
    }

    // NEW: Get raw bytes for transmission
    pub fn as_bytes(&self) -> [u8; 32] {
        self.keypair.verifying_key().to_bytes()
    }

    pub fn sign_message(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }
}

// Simple hex encoder helper since we didn't pull in a hex crate
mod hex {
    pub fn encode(bytes: [u8; 32]) -> String {
        let mut s = String::with_capacity(64);
        for b in bytes {
            use std::fmt::Write;
            write!(&mut s, "{:02x}", b).unwrap();
        }
        s
    }
}