use rsa_crypt::{self, generate_public_private_keys, get_private_key,get_public_key};

fn main(){
    println!("{}",generate_public_private_keys(256, true));
    println!("{:?}",get_private_key());
    println!("{:?}",get_public_key());
}