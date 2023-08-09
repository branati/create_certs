// use openssl::pkey::{Private, Public};
use openssl::rsa::{Rsa, Padding};
use std::fs::File;
use std::io::Write;

fn main() {
    // Gerando as chaves
    let rsa = Rsa::generate(2048).unwrap();
    let private_key = rsa.private_key_to_pem().unwrap();
    let public_key = rsa.public_key_to_pem().unwrap();
    
    let mut private_key_file = File::create("private_key.pem").unwrap();
    private_key_file.write_all(&private_key).unwrap();

    let mut public_key_file = File::create("public_key.pem").unwrap();
    public_key_file.write_all(&public_key).unwrap();

    // Encriptando e decriptando usando as chaves
    // let plaintext = b"Hello, world!";
    // let mut ciphertext = vec![0; rsa.size() as usize];
    // let len = rsa.public_encrypt(plaintext, &mut ciphertext, Padding::PKCS1).unwrap();
    // let mut decrypted = vec![0; len];
    // let len = rsa.private_decrypt(&ciphertext[..len], &mut decrypted, Padding::PKCS1).unwrap();
    // assert_eq!(&decrypted[..len], plaintext);

    println!("Certificates created successfully! RSA 2048 bits");
}
