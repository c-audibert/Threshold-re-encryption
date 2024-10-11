use nalgebra::{DMatrix, DVector};
use nalgebra::storage::Storage;
use rand::distributions::{Distribution, Uniform};
use regex::Regex;
use crate::lwe_functions::*;
use secret_sharing::*;
use lwe_functions::{SecretKey, PublicKey, CyperText};
use crate::reconstruction_secret;


mod lwe_functions;
mod secret_sharing;
mod from_file;

const Q: f64 = (2 << 15) as f64;
//Focntion EoD pour les machines
pub fn EoD(public_key: &PublicKey, cypher_text: CyperText, sk :&SecretKey, alea_r : DVector<f64>, e1_prime : DVector<f64>, e2_prime:f64) -> CyperText {
    let Dm =cypher_text.c2-(sk.s.dot(&cypher_text.c1));
    let cipher_prime= E(&public_key, Dm, alea_r, e1_prime, e2_prime);

    //let cyper_prime=encrypt(&public, Dm as i64);
    return cipher_prime
}

//Test de EoD poir n = 1
fn EoD_n_equal_1() -> Result<(), String> {
    let (public_A, secret_A) = keygen();
    let (public_B, secret_B) = keygen();

    let m_vect = vec![
        0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1,
        1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0,
        0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1,
        1, 1, 0, 1, 0, 1];

    let mut recontruit = Vec::new();
    let mut bit = 0;
    let mut nombre = 0;
    for (_, m) in m_vect.iter().enumerate() {
        let delta_m = (Q as i64 / 2) *m ;
        //premier encrypt de la par d'alice
        let cypher = encrypt(&public_A, delta_m);
        //Application de EoD pour la machine
        let (r, e1prime,e2_prime) = generate_alea_encrypt();
        let EoD = EoD(&public_B, cypher.clone(), &secret_A, r,e1prime,e2_prime);
        //m apres dormule EoD
        let m_prime = decrypt(&secret_B, &EoD);
        //test de construction de message
        bit = (bit << 1) | (m_prime as u8);
        nombre += 1;
        if nombre == 8 {
            recontruit.push(bit);
            bit = 0;
            nombre = 0;
        }
        //assert_eq!(*m, m_prime as i64, "pas bon pour {}", m);
    }
    //Conversion bits recu en String
    let messagereconstruit: String = recontruit.iter().map(|&b| b as char).collect();
    println!("\n msg original {}", messagereconstruit);
    Ok(())
}

//test pour n =2 avec partage de secret et generation paire de cle
pub fn EoD_n_equal_2_with_generation_key( m:i64) -> f64 {
    //Generation des clee et randomness
    let (public_A, secret_A) = keygen();
    let (r, e1prime,_) = generate_alea_encrypt();
    let (public_B, secret_B) = keygen();
    let (_, e1_prime,e2_prime) = generate_alea_encrypt();



    /////PARTIE DE ALICE///////
    let m = m;
    let delta_m = (Q as i64 / 2) * m;
    let cypher = encrypt(&public_A, delta_m);
    println!("Message Alice {:?}", m);
    println!("\nCypher ALICE {:?}", cypher);


    //////////////////////////SYSTEME ////////////////////////
    //Secret de la SK alice
    let secret_A_Vector : Vec<_>  = secret_A.clone().s.iter().map(|&x| x as i64).collect();
    //Shares pour les machines 1 et 2
    let  (shares_machine_1, shares_machine_2) = shared_vector_for_two_machines(secret_A_Vector.clone(), 2, Q as i64, 2);

    //Secretshare de r
    let r_vector : Vec<_> = r.iter().map(|&x| x as i64).collect();
    //Shares pour les machines 1 et 2
    let (r_for_machine_1,r_for_machine_2) = shared_vector_for_two_machines(r_vector.clone(), 2, Q as i64, 2);


    ///////////////////MACHINE 1////////////////////////
    let secret_key_machine_1 = SecretKey { s: shares_machine_1.clone() };
    let EoD_machine_1 = EoD(&public_B.clone(), cypher.clone(), &secret_key_machine_1, r_for_machine_1,e1_prime.clone(),e2_prime.clone());
    println!("\n EoD_machine_1 {:?}", EoD_machine_1);

    ///////////////////MACHINE 2////////////////////////
    let secret_key_machine_2 = SecretKey { s: shares_machine_2.clone() };
    let EoD_machine_2 = EoD(&public_B.clone(), cypher.clone(), &secret_key_machine_2, r_for_machine_2,e1_prime.clone().clone(),e2_prime.clone());
    println!("\n EoD_machine_2 {:?}", EoD_machine_2);



    ///////////////////PARTIE BOB////////////////////////
    //Reconstruction des couples de shares
    let couple_share_EoD = regroup_couples_shares(EoD_machine_1.c1.clone(), EoD_machine_2.c1.clone());
    //reconstruction de c1 via les shares  [s[0], s[1]] et lagrange pour deg=1
    let mut c1_prime = reconstruction_secret(couple_share_EoD);
    let c1_prime_dvector = DVector::from_iterator(c1_prime.len(), c1_prime.iter().map(|&x| x as f64));
    //reconstruction de c2 via les shares  [s[0], s[1]] et lagrange interpolation pour deg=1
    let couple_of_c2 = &vec![EoD_machine_1.c2 as i64, EoD_machine_2.c2 as i64];
    let c2_prime = lagrange_interpolation_degree1(0.0,couple_of_c2);
    //Reconstruction du cypher via c1 et c2 reconstruits
    let recontruction_cipher = CyperText { c1: c1_prime_dvector, c2: c2_prime};
    println!("\nCiphetext Reconstruit BOB{:?} ", recontruction_cipher);
    //Decryption du cypher reconstruit
    let decrypted = decrypt(&secret_B, &recontruction_cipher);
    println!(" \n Message dechiffrer {:?}", decrypted);
    decrypted
}

//test pour n =2 avec systeme deja dans un etat initial

fn main() {
    //EoD_n_equal_1();
    EoD_n_equal_2_with_generation_key(1);

}