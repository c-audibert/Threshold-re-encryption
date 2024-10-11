fn share_l(s_i: i32, u_i: i32, v_i: i32, a: i32, b: i32, c: i32, d: i32) -> i32 {
    a * s_i + b * u_i + c * v_i + d
}

fn main() {
    let s_i = 1;
    let u_i = 2;
    let v_i = 3;
    let a = 2;
    let b = 3;
    let c = 4;
    let d = 5;

    let l_i = share_l(s_i, u_i, v_i, a, b, c, d);
    println!("i-ème share de L(s,u,v) est :{}", l_i);
}