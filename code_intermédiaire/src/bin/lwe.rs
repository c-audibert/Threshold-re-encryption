use tfhe::core_crypto::prelude::*;


fn main() {

    use tfhe::core_crypto::prelude::*;

    let lwe_dimension = LweDimension(742);

    let lwe_noise_distribution =
        Gaussian::from_dispersion_parameter(StandardDev(0.000007069849454709433), 0.0);
    let ciphertext_modulus = CiphertextModulus::new_native();

    println!("{:?}", lwe_noise_distribution);

// Create the PRNG
    let mut seeder = new_seeder();
    let seeder = seeder.as_mut();
    let mut encryption_generator =
        EncryptionRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed(), seeder);
    let mut secret_generator =
        SecretRandomGenerator::<ActivatedRandomGenerator>::new(seeder.seed());

// Create the LweSecretKey
    let lwe_secret_key =
        allocate_and_generate_new_binary_lwe_secret_key(lwe_dimension, &mut secret_generator);
    println!("{:?}", lwe_secret_key);


// Codage du message avec un décalage de 60 bits
    let msg = 3u64;
    let plaintext = Plaintext(msg << 60);


// Create a new LweCiphertext
    let mut lwe = LweCiphertext::new(0u64, lwe_dimension.to_lwe_size(), ciphertext_modulus);
    println!("{:?}", lwe);

    encrypt_lwe_ciphertext(
        &lwe_secret_key,
        &mut lwe,
        plaintext,
        lwe_noise_distribution,
        &mut encryption_generator,
    );


    let decrypted_plaintext = decrypt_lwe_ciphertext(&lwe_secret_key, &lwe);
    println!("{:?}", decrypted_plaintext);

// First create a decomposer working on the high 4 bits corresponding to our encoding.
    let decomposer = SignedDecomposer::new(DecompositionBaseLog(4), DecompositionLevelCount(1));

    let rounded = decomposer.closest_representable(decrypted_plaintext.0);

// Remove the encoding
    let cleartext = rounded >> 60;

// Check we recovered the original message
    assert_eq!(cleartext, msg);
}
