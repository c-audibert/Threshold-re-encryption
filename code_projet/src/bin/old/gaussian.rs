use nalgebra::{DMatrix, DVector};
use rand::Rng;
use rand_distr::{Distribution, Normal, Uniform};
const Q: f64 = (2 << 7) as f64;
const N : i32 = 5;
const q_sur_2: i64 = Q as i64/ 2i64;

////////////////Definition des structures ///////////////////
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


////////////////Definition de notre modulo ///////////////////




////////////////generation pairs de cle ///////////////////
pub fn keygen() -> (PublicKey, SecretKey) {
    let mut random_gen = rand::thread_rng();

    let alpha = (2.0 * std::f64::consts::PI).sqrt() * Q.powf(-0.75);
    let sigma = (Q * alpha) / (2.0 * std::f64::consts::PI).sqrt();

    let loi_uniforme_matrix = Uniform::from(-q_sur_2..q_sur_2);
    let loi_normal_vect_e = Normal::new(0.0, sigma).unwrap();
    let loi_normal_vect_s = Uniform::from(-1..1);

    //Vecteurs s
    let vect_s: Vec<f64> = (0..N)
        .map(|_| ((loi_normal_vect_s.sample(&mut random_gen))as f64).round().abs())
        .collect();

    //Vecteurs e
    let vect_e: Vec<f64> = (0..N)
        .map(|_| loi_normal_vect_e.sample(&mut random_gen).round().abs() %Q)
        .collect();

    //Transforme les Vec en DVector
    let s = DVector::from_vec(vect_s);
    let e = DVector::from_vec(vect_e).transpose();


    //Generation de la matrice A
    let mut vecteur_a = Vec::new();
    for _ in 0..N {
        let row: Vec<f64> = (0..N)
            .map(|_| loi_uniforme_matrix.sample(&mut random_gen) as f64)
            .collect();
        vecteur_a.push(row);
    }
    let a = DMatrix::from_vec(N as usize, N as usize, vecteur_a.into_iter().flatten().collect());

    //Generation de B
    let b =  &s.transpose() *&a + &e;
    let b_vector = DVector::from_vec(b.iter().map(|x| x %Q).collect());

    let public_key = PublicKey { a, b: b_vector };
    let secret_key = SecretKey { s };
    //println!("B vector: {}", b_vector);
    //println!("(s): {}", s.clone().transpose());
    //println!("(e): {}", e);
    //println!("A: {}", a);

    (public_key, secret_key)
}



////////////////generation de r, e1, e2 ///////////////////
pub fn generate_alea_encrypt() -> (DVector<f64>, DVector<f64>, f64) {
    let mut random_gen = rand::thread_rng();

    //std_dev donner par le prof
    let loi_normal = Normal::<f64>::new(0.0, 2.0).unwrap();

    let r: DVector<f64> = DVector::from_iterator(
        N as usize,
        (0..N).map(|_| loi_normal.sample(&mut random_gen).round() as f64),
    );

    let e1: DVector<f64> = DVector::from_iterator(
        N as usize,
        (0..N).map(|_| loi_normal.sample(&mut random_gen).round() as f64),
    );
    let e2 = loi_normal.sample(&mut random_gen).round() as f64;
    (r, e1, e2)
}


pub fn encrypt(pk: &PublicKey, delta_m: i64) -> CyperText {
    let a = &pk.a;
    let b = &pk.b;

    let (r, e1, e2) = generate_alea_encrypt();

    //println!("(e1): {}", e1);
    //println!("(e2): {}", e2);
    let c1 = ((a * &r) + &e1).map(|x| x%Q as f64);
    let c2 = ((&b.transpose() * &r)[0] + e2 + delta_m as f64).modulo() as f64;

    CyperText { c1, c2 }
}


fn nearest_integer(x: f64) -> i64 {
    let ceil_value = x.ceil() as i64;
    let floor_value = x.floor() as i64;
    let distance_ceil = (ceil_value as f64 - x).abs();
    let distance_floor = (floor_value as f64 - x).abs();

    let mut rounded_value = if distance_ceil < distance_floor {
        ceil_value
    } else if distance_floor < distance_ceil {
        floor_value
    } else {
        ceil_value
    };

    if rounded_value < 0 {
        rounded_value = 0;
    } else if rounded_value > 1 {
        rounded_value = 1;
    }

    rounded_value
}

pub fn decrypt(sk: &SecretKey, cypher: &CyperText) -> f64 {
    let s = &sk.s;
    let c1 = &cypher.c1;
    let c2 = cypher.c2;

    //println!("(c1): {}", c1);
    //println!("(c2): {}", c2);
    //println!("(s): {}", s);

    let prod_sk_c1 = s.dot(c1);

    let scaling_factor = 2.0 / Q;
    let mut prod = c2 - prod_sk_c1;

    /*
    while prod < 0.0 {
        prod += Q;
    }
*/
    println!("c2 - prod_sk_c1; {}", prod );
    let mut test_res = prod.clone()%Q;
    let r = (test_res * scaling_factor) ;
    println!("prod.clone().modulo(); {}", test_res);
    println!("r: {}", r);
    ////println!("approcher r: {}", r);
    let ci = nearest_integer(r.abs());
    ci as f64
}


//multiplication modulo Gal(2,e)
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

fn test_encryption_decryption() -> Result<(), String>{
    let (pk, sk) = keygen();

    let mut rng = rand::thread_rng();
    let _loi_uniforme_r = Uniform::from(0..2);

    for _ in 0..100 {
        let m = _loi_uniforme_r.sample(&mut rng);
        let delta_m = (Q as i64 / 2) * m as i64;
        let c = encrypt(&pk, delta_m);
        let m_decrypted = decrypt(&sk, &c);
        assert_eq!(m, m_decrypted as i64, "pas bon pour {}", m);
    }

    Ok(())
}


fn main() {
    let (pk, sk) = keygen();
    println!("Clé publique: {:?}", pk);
    println!("Clé privée: {:?}", sk);
    //test_encryption_decryption();

     let m = 0;
    let delta_m = (Q as i64 / 2) * m;
    let c = encrypt(&pk, delta_m);
    let m_decypherilparrait = decrypt(&sk, &c);

    println!("dechiffrer {}", m_decypherilparrait);


}
