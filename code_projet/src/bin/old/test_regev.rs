use nalgebra::{DMatrix, DVector, Dynamic, MatrixMN, U1};
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Normal, Uniform};

const Q: f64 = (2 << 7) as f64;
const N : i32 = 20;

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub a: DMatrix<f64>,
    pub b: DVector<f64>,
}

#[derive(Debug, Clone)]
pub struct SecretKey {
    pub s: DVector<f64>,
}

#[derive(Debug, Clone)]
pub struct CyperText {
    pub c1: DVector<f64>,
    pub c2: f64,
}


pub fn keygen() -> (PublicKey, SecretKey) {
    let mut random_gen = rand::thread_rng();

    let q_i64: i64 = Q as i64;
    let alpha = (2.0 * std::f64::consts::PI).sqrt() * Q.powf(-0.75);
    let sigma = (Q * alpha) / (2.0 * std::f64::consts::PI).sqrt();

    let loi_uniforme_matrix = Uniform::from(0..q_i64);
    let loi_normal_vect_e = Normal::<f64>::new(0.0, sigma).unwrap();
    let loi_normal_vect_s = Uniform::from(-1..2 as i64); // Discrete Gaussian for s

    //Vecteurs s
    let vect_s: Vec<f64> = (0..N)
        .map(|_| loi_normal_vect_s.sample(&mut random_gen) as f64)
        .collect();

    // Vecteurs e
    let vect_e: Vec<f64> = (0..N)
        .map(|_| loi_normal_vect_e.sample(&mut random_gen).round()) // Round to nearest integer
        .collect();

    let s = DVector::from_vec(vect_s);
    let e = DVector::from_vec(vect_e).transpose();

    let mut vecteur_a = Vec::new();
    for _ in 0..N {
        let row: Vec<f64> = (0..N)
            .map(|_| loi_uniforme_matrix.sample(&mut random_gen) as f64)
            .collect();
        vecteur_a.push(row);
    }
    let a = DMatrix::from_vec(N as usize, N as usize, vecteur_a.into_iter().flatten().collect());


    let b = (&s.transpose() * &a + &e).map(|x| x % q_i64 as f64);
    let b_vector = DVector::from_iterator(N as usize, b.iter().cloned());


    let public_key = PublicKey { a : a.clone(), b : b_vector };
    let secret_key = SecretKey { s };
    (public_key, secret_key)
}

pub fn encrypt(pk: &PublicKey, delta_m: i64, random_gen: &mut ThreadRng) -> CyperText {
    let a = &pk.a;
    let b = &pk.b;

    let loi_uniforme_r = Uniform::from(0..Q as i64); // Uniform modulo q
    let loi_normal = Normal::<f64>::new(0.0, 2.0).unwrap(); // Discrete Gaussian for errors

    let r: DVector<f64> = DVector::from_iterator(
        a.ncols(),
        (0..a.ncols()).map(|_| loi_uniforme_r.sample(random_gen) as f64),
    );

    let e1: DVector<f64> = DVector::from_iterator(
        a.ncols(),
        (0..a.ncols()).map(|_| loi_normal.sample(random_gen).round()),
    );
    let e2 = loi_normal.sample(random_gen).round();
    print!("FKDJBHKSDFBVKSDJFSDFFD{}", &b.transpose() * &r);

    let c1 = (a * &r + &e1).map(|x| x % Q as f64);
    let c2 = ((&b.transpose() * &r)[0] + e2 + delta_m as f64) % Q as f64;

    CyperText { c1, c2 }
}

fn nearest_integer(x: f64) -> i64 {
    let ceil_value = x.ceil() as i64;
    let floor_value = x.floor() as i64;
    let distance_ceil = (ceil_value as f64 - x).abs();
    let distance_floor = (floor_value as f64 - x).abs();
    if distance_ceil < distance_floor
    { ceil_value
    }
    else
    { floor_value }
}

pub fn decrypt(sk: &SecretKey, cypher: &CyperText) -> i64 {
    let s = &sk.s;
    let c1 = &cypher.c1;
    let c2 = cypher.c2;

    let prod_sk_c1 = s.dot(c1);

    let difference = c2 - prod_sk_c1;
    let reduced_diff = difference % Q as f64;
    let nearest_multiple = (reduced_diff / (Q as f64 / 2.0)).round() * (Q as f64 / 2.0);
    if nearest_multiple == Q as f64 / 2.0 {
        1
    } else {
        0
    }
}


//multiplication modulo Gal(2,e)
/*
fn multiply_vectors(x: [i32; 2], y: [i32; 2]) -> [i32; 2] {
    match (x, y) {
        ([0, 0], _) | (_, [0, 0]) => [0, 0],
        ([1, 0], _) => y,
        (_, [1, 0]) => x,
        ([1, 1], [1, 1]) => [0, 1],
        ([0, 1], [0, 1]) => [1, 1],
        ([0, 1], [1, 1]) | ([1, 1], [0, 1]) => [1, 0],
        _ => [0, 0],
    }
}

 */

fn main() {
    let mut random_gen = rand::thread_rng();
    let (pk, sk) = keygen();
    println!("Clé publique: {:?}", pk);
    println!("Clé privée: {:?}", sk);


    let m = 0;
    let delta_m = (Q as i64 / 2 )* m;
    let c = encrypt(&pk, delta_m, &mut random_gen);
    let m_decypherilparrait = decrypt(&sk, &c);

    println!("dechiffrer {}", m_decypherilparrait);

}
