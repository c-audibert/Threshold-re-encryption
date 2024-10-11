use std::str::FromStr;
use rand::distributions::Sample;


fn main() {

    let threshold = 2;
    let modulo = 1024;
    let secret = 55;
    let nb_shares = 2;


    pub fn generator_polynome(modulo_number: i64, zero_value: i64, threshold: usize) -> Vec<i64> {
        //genere les distributions
        let mut range = rand::distributions::range::Range::new(0, modulo_number-1);
        let mut rng = rand::OsRng::new().unwrap();

        //On genere les coefficients du polynome
        let mut value_vector: Vec<i64> = (0..=threshold)
            .map(|_| range.sample(&mut rng))
            .collect();
        println!("{}", range.sample(&mut rng)
        );

        //On met le secret au degre voulu
        value_vector[threshold] = zero_value;
        return value_vector;
    }

    pub fn share_generation(coefficients: &[i64], point: i64, modulo: i64) -> i64 {
        //Prends les coeff du x au plus grand au plus petit
        let mut reversed_coefficients = coefficients.iter().rev();

        let haut_deg = *reversed_coefficients.next().unwrap();
        let tail = reversed_coefficients;

        //Modulo Q ou pas sur la valeur des shares?
        //tail.fold(haut_deg, |partial, coef| (partial * point + coef) % modulo)
        tail.fold(haut_deg, |partial, coef| (partial * point + coef) % modulo)

    }


    //n = 1
    pub fn polynome_evaluation_coeff(nb_shares:i64,  coefficients: &[i64], modulo : i64) -> Vec<i64> {
        //Generation les shares pour chaque evaluation
        (0..nb_shares )
            .map(|point| share_generation(coefficients, point as i64, modulo))
            .collect()

    }

    let polynome = generator_polynome(modulo, secret, threshold-1);
    println!("Coeff polynome (1,x,x^2,...) -> {:?}", polynome);
    let shares = polynome_evaluation_coeff(nb_shares, &polynome, modulo);
    println!("Shares %q -> {:?}", shares);



    /////////////////Seconde partie intermediaire - fonction R ///////////////////
    /**
    fn projection_sur_a2(shares: &[i64]) -> f64 {
        let x = [0.0, 1.0, 2.0]; // Points correspondant aux évaluations du polynôme
        //y[0] = share[0], y[1] = share[1], y[2] = share[2]
        let y = [shares[0] as f64, shares[1] as f64, shares[2] as f64];

        let a2 = y[0] / ((x[0] - x[1]) * (x[0] - x[2])) +
            y[1] / ((x[1] - x[0]) * (x[1] - x[2])) +
            y[2] / ((x[2] - x[0]) * (x[2] - x[1]));
        a2
    }
    **/

    pub fn projection_sur_a2(shares: &[i64]) -> f64 {
        return (shares[1] - shares[0]) as f64;
    }


    let test_gab = projection_sur_a2(&shares);
    println!("Coefficient récupéré de x^2: {}", test_gab);





    /////////////////Troisième partie intermediaire ///////////////////


    let polynome_a = generator_polynome(modulo, secret, threshold-1);
    let polynome_b = generator_polynome(modulo, secret, threshold-1);
    let polynome_c = generator_polynome(modulo, secret, threshold-1);

    let shares_a = polynome_evaluation_coeff(3, &polynome_a, modulo);
    let shares_b = polynome_evaluation_coeff(3, &polynome_b, modulo);
    let shares_c = polynome_evaluation_coeff(3, &polynome_c, modulo);
    println!("Shares A %q -> {:?} {:?} {:?}", shares_a, shares_b, shares_c);

    /**
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
    **/

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


    fn share_l_i(s_i: i64, u_i: i64, v_i: i64, a: i32, b: i32, c: i32, d: i32) -> i64 {
        let result = a as i64 * s_i + b as i64 * u_i + c as i64 * v_i + d as i64;
        result
    }



    let l_result = l(&shares_a, &shares_b, &shares_c, 2, 3, 4, 5);
    let tt = share_l_i(shares_a[2], shares_b[2], shares_c[2], 2, 3, 4, 5);
    println!("L(s,u,v) est :{:?}", l_result);
    println!("i-ème share de L(s,u,v) est :{}", tt);



}