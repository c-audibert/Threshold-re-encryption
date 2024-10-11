use std::str::FromStr;
use nalgebra::DVector;
use rand::distributions::*;
use rand_distr::Normal;

const Q: i64 = 2 << 15;

trait Modulo_qsur2 {
    fn modulo_qsur2(self) -> i64;
}
impl Modulo_qsur2 for i64 {
    fn modulo_qsur2(self) -> i64 {
        let mut x = self;
        while x < -Q / 2 {
            x += Q;
        }
        while x >= Q / 2 {
            x -= Q;
        }
        x
    }
}

pub fn generator_polynome(modulo_number: i64, zero_value: i64, threshold: usize) -> Vec<i64> {
    //genere les distributions
    let mut range = Uniform::new(-modulo_number/2, modulo_number/2);
    let mut rng = rand::thread_rng();

    //On genere les coefficients du polynome
    let mut value_vector: Vec<i64> = (0..threshold+1)
        .map(|_| range.sample(&mut rng))
        .collect();
    //On ajoute le zero value pour le coeff constant (juste a changer si on met de plus haut degré)
    value_vector[0] = zero_value;
    return value_vector;
}

//Methode horner's pour evaluer le prolynome pour deg > 1
pub fn share_generation(coefficients: &[i64], point: i64, modulo: i64) -> i64 {
    //Prends les coeff du x au plus grand au plus petit
    let mut reversed_coefficients = coefficients.iter().rev();
    let haut_deg = *reversed_coefficients.next().unwrap();
    let tail = reversed_coefficients;
    tail.fold(haut_deg, |partial, coef| (partial * point + coef).modulo_qsur2())
}

pub fn share_generation_deg_1(coefficients: &[i64], point: i64, modulo: i64) -> i64 {
    coefficients[0] +point*coefficients[1]
}

pub fn polynome_evaluation_coeff(nb_shares:i64,  coefficients: &[i64], modulo : i64) -> Vec<i64> {
    //Generation les shares pour chaque evaluation
    (1..nb_shares+1 )
        .map(|point| share_generation_deg_1(coefficients, point as i64, modulo))
        .collect()
}


//pour le secret sur le coeff de plus haut degré
pub fn projection_sur_a2(shares: &[i64]) -> i64 {
    return shares[1] - shares[0] ;
}


//Denier travail intermediaire (3)
pub fn l(s_i: &[i64], u_i: &[i64], v_i: &[i64], a: i32, b: i32, c: i32, d: i32) -> Vec<i64> {
    let mut result = Vec::new();
    for i in 0..s_i.len() {
        let s = s_i[i];
        let u = u_i[i];
        let v = v_i[i];
        let valeur  = a as i64 * s + b as i64 * u + c as i64 * v + d as i64;
        result.push(valeur);
    }
    result
}
pub fn share_l_i(s_i: i64, u_i: i64, v_i: i64, a: i32, b: i32, c: i32, d: i32) -> i64 {
    let result = a as i64 * s_i + b as i64 * u_i + c as i64 * v_i + d as i64;
    result
}


//Shared un vecteur
pub fn secret_shared_vector(secret: Vec<i64>, threshold: usize, modulo: i64, nb_shares: i64) -> Vec<Vec<i64>> {
    let mut table_of_shares = Vec::new();
    //Shared chaque coeff i du vecteur avec un nouveau polynome
    for value in secret {
        let polynome = generator_polynome(modulo, value, threshold - 1);
        let shares = polynome_evaluation_coeff(nb_shares, &polynome, modulo);
        table_of_shares.push(shares);
    }
    table_of_shares
}

//Shared un vecteur pour deux machines (machine 1 et machine 2)
pub fn shared_vector_for_two_machines(secret: Vec<i64>, threshold: usize, modulo: i64, nb_shares: i64) -> (DVector<f64>, DVector<f64>) {
    let mut table_of_shares_machine_1 = Vec::new();
    let mut table_of_shares_machine_2 = Vec::new();
    let table_of_shares = secret_shared_vector(secret.clone(), threshold, modulo, nb_shares);
    for value in table_of_shares {
        table_of_shares_machine_1.push(value[0] as f64);
        table_of_shares_machine_2.push(value[1] as f64);
    }
    (DVector::from_vec(table_of_shares_machine_1), DVector::from_vec(table_of_shares_machine_2))
}

//Reconstruction du secret via les couples de shares
pub fn reconstruction_secret(shares: Vec<Vec<i64>>) -> Vec<f64> {
    let mut secret_vector = Vec::new();
    for tuple_share in shares {
        //let coeff_reconstruit = projection_sur_a2(&tuple_share);
        let coeff_reconstruit = lagrange_interpolation_degree1(0.0, &tuple_share);
        secret_vector.push(coeff_reconstruit);
    }
    secret_vector
}

//Regroupe les couples de shares des coeff i de chaques vecteurs en un vecteur de couples
pub fn regroup_couples_shares(shares_machine_1: DVector<f64>, shares_machine_2: DVector<f64>) -> Vec<Vec<i64>> {
    let mut couples_of_shares = Vec::new();
    for i in 0..shares_machine_1.len() {
        couples_of_shares.push(vec![shares_machine_1[i] as i64, shares_machine_2[i] as i64]);
    }
    couples_of_shares
}


//Focntion de test
pub fn test_sharing_vector() -> Result<(), String>{
    let threshold = 2;
    let nb_shares = 2;
    for j in 0..100{
        let mut dimension = Uniform::new(5, 50).sample(&mut rand::thread_rng());
        let mut secret_key_alice = vec![0; dimension as usize];
        //secret key alice
        for i in 0..dimension {
            let  coeff = Normal::new(0.0, 2.0).unwrap().sample(&mut rand::thread_rng());
            secret_key_alice[i as usize] = coeff as i64;
        }
        //println!("\n Secret {j}: {:?}", secret_key_alice);
        let shares = secret_shared_vector(secret_key_alice.clone(), threshold, Q, nb_shares);
        //println!("\n Shares {j}: {:?}", shares);
        let secret_reconstruit : Vec<_>= reconstruction_secret(shares).iter().map(|&x| x as i64).collect();
        //println!("\n Secret reconstruit {j}: {:?}", secret_reconstruit);
        assert_eq!(secret_key_alice, secret_reconstruit);
    }
    Ok(())
}


//Pour secret sur le coeff 0
pub fn lagrange_interpolation_degree1(x: f64, shares: &Vec<i64>) -> f64 {
    let x0 = 1.0;
    let x1 = 2.0;
    // Formule de Lagrange pour un polynôme de degré 1
    let a0 = shares[0] as f64 * (x - x1) / (x0 - x1) +
        shares[1] as f64 * (x - x0) / (x1 - x0);
    return a0
}


fn main() {
    test_sharing_vector();
    let threshold = 2;
    let modulo = 1024;
    let nb_shares = 2;
    let secret = vec![ 4,5,3,2,6,7,44,2,95,3,5,37,55];
    let shares = secret_shared_vector(secret.clone(), threshold, modulo, nb_shares);
    println!("Vec retrouver: {:?}", shares);
    let secret_reconstruit = reconstruction_secret(shares);

    let (m1, m2) = shared_vector_for_two_machines(secret.clone(), threshold, modulo, nb_shares);
    println!("Vec retrouver machine 1: {:?}", m1);
    println!("Vec retrouver machine 2: {:?}", m2);
    let couples_of_shares = regroup_couples_shares(m1, m2);
    println!("Couple de shares: {:?}", couples_of_shares);
    let secret_reconstruit = reconstruction_secret(couples_of_shares);
    println!("Secret recon : {:?}", secret_reconstruit);
    //test_sharing_vector();
    fn tkt(){//n = 1
        /*
        let polynome = generator_polynome(modulo, secret, threshold-1);
        println!("Coeff polynome (1,x,x^2,...) -> {:?}", polynome);
        let shares = polynome_evaluation_coeff(nb_shares, &polynome, modulo);
        println!("Shares %q -> {:?}", shares);
        */
        test_sharing_vector();


        /////////////////Seconde partie intermediaire - fonction R //////////////////



        /*
        let test_gab = projection_sur_a2(&shares);
        println!("Coefficient récupéré de x^2: {}", test_gab);

         */}
    fn partie3() {
        /////////////////Troisième partie intermediaire ///////////////////

        //let secret = vec![ 4,5,3,2,6,7,44,2,95,3,5,37,55];
        //let shares = secret_shared_vector(secret, threshold, modulo, nb_shares);
        //println!("Vec retrouver: {:?}", shares);
        //let secret_reconstruit = reconstruction_secret(shares);
        //println!("Secret recon : {:?}", secret_reconstruit)
    }
    fn truc_apres(){
        /*
let polynome_a = generator_polynome(modulo, secret, threshold-1);
let polynome_b = generator_polynome(modulo, secret, threshold-1);
let polynome_c = generator_polynome(modulo, secret, threshold-1);

let shares_a = polynome_evaluation_coeff(3, &polynome_a, modulo);
let shares_b = polynome_evaluation_coeff(3, &polynome_b, modulo);
let shares_c = polynome_evaluation_coeff(3, &polynome_c, modulo);
println!("Shares A %q -> {:?} {:?} {:?}", shares_a, shares_b, shares_c);
*/
        /*
        fn l(s_i: &[i64], u_i: &[i64], v_i: &[i64], a: i32, b: i32, c: i32, d: i32) -> Vec<i64> {
            let mut result = Vec::new();
            for i in 0..s_i.len() {
                let s = s_i[i];
                let u = u_i[i];
                let v = v_i[i];
                let valeur  = a as i64 * s + b as i64 * u + c as i64 * v + d as i64;
                result.push(valeur);
            }

            result
        }
        */



        /*
        let l_result = l(&shares_a, &shares_b, &shares_c, 2, 3, 4, 5);
        let tt = share_l_i(shares_a[2], shares_b[2], shares_c[2], 2, 3, 4, 5);
        println!("L(s,u,v) est :{:?}", l_result);

        println!("i-ème share de L(s,u,v) est :{}", tt);
         */
    }
}