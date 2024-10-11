use nalgebra::{DMatrix, DVector};
use rand_distr::{Normal, Distribution, Uniform};

fn main() {
    let mut random_gen = rand::thread_rng();

    let taille_s = 10; // Taille de s
    let taille_e = 10; // Taille de e
    let taille_a = 10; // Nombre de lignes de A

    //modulus
    let q: i32 = 44;
    let q_f64: f64 = q as f64;

    //Standard deviation
    let alpha = (2.0 * std::f64::consts::PI).sqrt()*q_f64.powf(-0.75);
    let sigma = (q_f64 * alpha) / (2.0 * std::f64::consts::PI).sqrt();

    //Loi de distribution pour les vecteurs s, e et a
    let loi_uniforme_matrix = Uniform::from(1..q);
    let loi_normal_vect_e = Normal::new(0.0, sigma).unwrap();
    let loi_normal_vect_s = Normal::new(0.0, 10.0).unwrap();

    //let s: DVector<f64> = DVector::from_iterator(taille_s, (0..taille_s).map(|_| (loi_normal.sample(&mut random_gen) % q as f64).round().max((q - 1) as f64).min(0 as f64)));

    //Vecteurs s
    let vect_s: Vec<f64> = (0..taille_s)
        .map(|_| ((loi_normal_vect_s.sample(&mut random_gen))as f64).round().abs() % q as f64)
        .collect();

    //Vecteurs e
    let vect_e: Vec<f64> = (0..taille_e)
        .map(|_| ((loi_normal_vect_e.sample(&mut random_gen))as f64).round().abs() % q as f64)
        .collect();


    //Creation objet DVector via les vecteurs s et e
    let s = DVector::from_vec(vect_s);
    let e = DVector::from_vec(vect_e);

    //Matrice A
    let mut vecteur_a = Vec::new();

    //
    for _ in 0..taille_a {
        let row: Vec<f64> = (0..taille_s)
            .map(|_| (loi_uniforme_matrix.sample(&mut random_gen) % q) as f64)
            .collect();
        vecteur_a.push(row);
    }
    let a = DMatrix::from_vec(taille_a, taille_s, vecteur_a.into_iter()
        .flatten()
        .collect());


    //Calcul de A.s + e
    let result = &a * &s + &e;

    println!("s : {}", s);
    println!("e : {}", e);
    println!("A : {}", a);
    println!("A.s + e: {}", result);
}
