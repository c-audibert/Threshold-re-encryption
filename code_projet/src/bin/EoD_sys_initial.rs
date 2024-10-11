use nalgebra::{DMatrix, DVector};
use nalgebra::storage::Storage;
use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;



use crate::lwe_functions::*;
use secret_sharing::*;
use from_file::*;
use crate::from_file::*;
use lwe_functions::{SecretKey, PublicKey, CyperText};

mod lwe_functions;
mod secret_sharing;
mod from_file;
const Q: f64 = (2 << 15) as f64;
const ALICEKEY: &str = "keys/cleAlice.txt";
const BOBKEY: &str = "keys/cleBob.txt";
const RANDOMNESS: &str = "keys/randomness";
const SK_SECRET_SHARED: &str = "keys/skA_secret_share";
const R_SECRET_SHARED: &str = "keys/r_secret_share";


pub fn EoD(public_key: &PublicKey, cypher_text: CyperText, sk :&SecretKey, alea_r : DVector<f64>, e1_prime : DVector<f64>, e2_prime:f64) -> CyperText {
    let Dm =cypher_text.c2-(sk.s.dot(&cypher_text.c1));
    let cipher_prime= E(&public_key, Dm, alea_r, e1_prime, e2_prime);

    //let cyper_prime=encrypt(&public, Dm as i64);
    return cipher_prime
}

//////////////////PARTIE ALICE////////////////////////
fn alice_encrypt(public_A: &PublicKey, message: &str ) -> Vec<CyperText> {
    println!("Message Aice {:?}", message);
    let mut cyphers = Vec::new();
    for byte in message.as_bytes() {
        for i in (0..8).rev() {
            let m = (byte >> i) & 1;
            let delta_m = (Q as i64 / 2) * m as i64;
            let cypher = encrypt(public_A, delta_m);
            cyphers.push(cypher);
        }
    }
    println!("\nCyphers ALICE {:?}", cyphers.as_slice());
    cyphers

}


//////////////////PATIE DES MACHINES////////////////////////

fn machine_process(cypher: &CyperText, shares: DVector<f64>, r_for_machine: DVector<f64>, e1_prime: DVector<f64>, e2_prime: f64, public_B: &PublicKey) -> CyperText {
    let secret_key_machine = SecretKey { s: shares.clone() };
    let mut res = EoD(public_B, cypher.clone(), &secret_key_machine, r_for_machine, e1_prime, e2_prime);
    println!("\nCypher machine {:?} \n {:?}", res.c1.as_slice(), res.c2 as i64);
    res
}

//////////////////PARTIE BOB////////////////////////

fn bob_decrypt(eod_results: Vec<(CyperText, CyperText)>, secret_B: &SecretKey) -> String {
    let mut recontruit = Vec::new();
    let mut bit = 0;
    let mut nombre = 0;
    println!("\nBOB RECOIS {:?}", eod_results.as_slice());

    for (EoD_machine_1, EoD_machine_2) in eod_results {
        let couple_share_EoD = regroup_couples_shares(EoD_machine_1.c1.clone(), EoD_machine_2.c1.clone());
        let mut c1_prime = reconstruction_secret(couple_share_EoD);
        let c1_prime_dvector = DVector::from_iterator(c1_prime.len(), c1_prime.iter().map(|&x| x as f64));

        let couple_of_c2 = vec![EoD_machine_1.c2 as i64, EoD_machine_2.c2 as i64];
        let c2_prime = lagrange_interpolation_degree1(0.0, &couple_of_c2);

        let reconstructed_cipher = CyperText { c1: c1_prime_dvector, c2: c2_prime };
        println!("\nCiphetext Reconstruit {:?} \n {:?}", reconstructed_cipher.c1.as_slice(), reconstructed_cipher.c2 as i64);
        let decrypted = decrypt(secret_B, &reconstructed_cipher);

        bit = (bit << 1) | (decrypted as u8);
        nombre += 1;
        if nombre == 8 {
            recontruit.push(bit);
            bit = 0;
            nombre = 0;
        }
    }

    recontruit.iter().map(|&b| b as char).collect()
}

fn EoD_n_equal_2_with_fileKey(message: &str) -> Result<(), String> {
    // Récupération des clés et des partages aléatoires
    let (shares_machine_1, shares_machine_2) = shared_from_file(SK_SECRET_SHARED).unwrap();
    let (r_for_machine_1, r_for_machine_2) = shared_from_file(R_SECRET_SHARED).unwrap();
    let (public_A, secret_A) = keys_from_file(ALICEKEY).unwrap();
    let (public_B, secret_B) = keys_from_file(BOBKEY).unwrap();

    // Conversion du message en cyphertexts par Alice
    let cyphers = alice_encrypt(&public_A, message);

    // Génération des valeurs de randomness par le système
    let (e1_prime, e2_prime) = randomness_from_file(RANDOMNESS).unwrap();

    // Processus des machines pour chaque cyphertext
    let mut eod_results = Vec::new();
    for cypher in cyphers {
        let EoD_machine_1 = machine_process(&cypher, shares_machine_1.clone(), r_for_machine_1.clone(), e1_prime.clone(), e2_prime.clone(), &public_B);
        let EoD_machine_2 = machine_process(&cypher, shares_machine_2.clone(), r_for_machine_2.clone(), e1_prime.clone(), e2_prime.clone(), &public_B);

        eod_results.push((EoD_machine_1, EoD_machine_2));
    }

    // Déchiffrement par Bob
    let message_reconstruit = bob_decrypt(eod_results, &secret_B);
    println!("\nMessage dechiffrer {}", message_reconstruit);

    Ok(())
}


fn main() {

    EoD_n_equal_2_with_fileKey("Bonjour !");

}