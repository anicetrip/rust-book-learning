fn main() {
    let s1 = 1;
    let s2 = 1;
    let mut s_n = s1;
    let mut s_n_plus_1 = s2;
    let mut sum;
     println!("{}", s1);
      println!("{}", s2);
    loop {
        sum = s_n +s_n_plus_1;
         println!("{}", sum);
        s_n_plus_1 = sum;
        s_n = s_n_plus_1;
        if sum >1000 {
            break;
        }
    }
}
