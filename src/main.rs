fn q_rsqrt(number: f32) -> f32 {
    let mut i: i32;
    let x2: f32;
    let mut y: f32;
    const THREEHALFS: f32 = 1.5;
    x2 = number * 0.5;
    y = number;
    i = unsafe { std::mem::transmute::<f32, i32>(y) };
    i = 0x5f3759df - (i >> 1);
    y = unsafe { std::mem::transmute::<i32, f32>(i) };
    y = y * (THREEHALFS - (x2 * y * y));
    return y;
}

fn main(){
    let ans = q_rsqrt(2.0);
    println!("{}", ans);
}
