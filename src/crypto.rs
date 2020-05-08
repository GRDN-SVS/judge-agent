use sodiumoxide::crypto;

/// Struct used to manage all the encryption logic
pub struct Encrypter {
    pub public_key: Vec<u8>,
    private_key: Vec<u8>,
}

impl Encrypter {
    /// Creates a new `Encrypter` instance with private and public keys
    pub fn new() -> Encrypter {
        let (public_key, private_key) = crypto::box_::gen_keypair();

        Encrypter {
            public_key: public_key.0.to_vec(),
            private_key: private_key.0.to_vec(),
        }
    }

    /// Sign a message using the `Encrypter's` private key in order
    /// to guarantee that the message was read by this agent
    pub fn sign(&self, msg: Vec<u8>) -> Vec<u8> {
        //sodiumoxide api convertion
        let private_sign_key = crypto::sign::SecretKey::from_slice(&self.private_key).expect("Could not create the sign key");

        let signed_data = crypto::sign::sign(&msg, &private_sign_key);

        signed_data
    }

    /// Encrypt the message using another agent's publix key, 
    /// in order to establish a shared secret between the two agents
    pub fn seal(&self, msg: Vec<u8>, nonce: &Vec<u8>, other_pk: &Vec<u8>) -> Vec<u8> {
        let nonce = crypto::box_::Nonce::from_slice(nonce).unwrap();
        let other_pk = crypto::box_::PublicKey::from_slice(other_pk).unwrap();
        let my_secret_key = crypto::box_::SecretKey::from_slice(&self.private_key).unwrap();
        
        let sealed_data = crypto::box_::seal(&msg, &nonce, &other_pk, &my_secret_key);

        sealed_data
    }
}
