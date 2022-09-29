use rsa::{
        PublicKey, 
        RsaPrivateKey, 
        RsaPublicKey, 
        PaddingScheme,
        pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey}
};

use std::fs;

use rand::*;

pub struct Keys{
    pub public : String,
    pub private : String,
    pub public_key : RsaPublicKey,
    pub private_key : RsaPrivateKey
}

impl std::fmt::Display for Keys{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.public, self.private)
    }
}

pub fn generate_public_private_keys(bits : usize, to_file : bool) -> Keys{
    let mut rng = rand::thread_rng();
    let bit_size : usize = bits;

    let private_key = RsaPrivateKey::new(&mut rng, bit_size ).expect("Unable to generate private key");
    let public_key = RsaPublicKey::from(&private_key);
    let str_private_key = private_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::default()).unwrap().to_string();
    let str_public_key = public_key.to_public_key_pem(rsa::pkcs8::LineEnding::default()).unwrap().to_string();
    if to_file {
        fs::write("public.pem", &str_public_key).expect("Unable to create public key file");
        fs::write("private.pem", &str_private_key).expect("Unable to create the private key file");
    }

    Keys { 
        public: str_public_key, 
        private: str_private_key 
    }
}

pub fn encrypt_data(key : RsaPublicKey, data : Vec<u8>) -> Vec<u8>{
    let mut rng = rand::thread_rng();
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    key.encrypt(&mut rng, padding, &data).expect("Unable to encrypt data")
}

pub fn decrypt_data(key : RsaPrivateKey, data : Vec<u8>) -> Vec<u8>{
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    key.decrypt(padding, &data).expect("Unable to decrypt data")
}

pub fn get_private_key() -> RsaPrivateKey{
    let private_key_str = String::from_utf8(fs::read("private.pem").expect("Unable to read private key")).unwrap();
    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_str).expect("Unable to extract the private key");
    private_key
}

pub fn get_public_key() -> RsaPublicKey{
    let public_key_str = String::from_utf8(fs::read("public.pem").expect("Unable to read public key file")).unwrap();
    let public_key = RsaPublicKey::from_public_key_pem(&public_key_str).expect("Unable to extract the public key");
    public_key
}