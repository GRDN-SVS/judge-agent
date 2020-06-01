use sodiumoxide::crypto;

/// Struct used to manage all the encryption logic
#[derive(Clone)]
pub struct Encrypter {
    pub box_public_key: crypto::box_::PublicKey,
    box_private_key: crypto::box_::SecretKey,
    pub sign_public_key: crypto::sign::PublicKey,
    sign_private_key: crypto::sign::SecretKey,
}

impl Encrypter {
    /// Creates a new `Encrypter` instance with private and public keys
    pub fn new() -> Encrypter {
        let (box_public_key, box_private_key) = crypto::box_::gen_keypair();
        let (sign_public_key, sign_private_key) = crypto::sign::gen_keypair();

        Encrypter {
            box_public_key,
            box_private_key,
            sign_public_key,
            sign_private_key,
        }
    }

    /// Sign a message using the `Encrypter's` private key in order
    /// to guarantee that the message was read by this agent
    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        crypto::sign::sign(msg, &self.sign_private_key)
    }

    /// Encrypt the message using another agent's publix key,
    /// in order to establish a shared secret between the two agents
    pub fn seal(&self, msg: Vec<u8>, nonce: &[u8], other_pk: &[u8]) -> Vec<u8> {
        let nonce = crypto::box_::Nonce::from_slice(nonce).unwrap();
        let other_pk = crypto::box_::PublicKey::from_slice(other_pk).unwrap();

        crypto::box_::seal(&msg, &nonce, &other_pk, &self.box_private_key)
    }
}
