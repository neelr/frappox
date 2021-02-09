fn main() {
   let num = std::env::args().nth(1).expect("no number given");
   let result = approx(num.parse::<f64>().unwrap());
   println!("{}/{}",result.0,result.1);
}

pub fn approx(num :f64) -> (i64,i64) {
    let decimal = (num%1.0 * 1000000000000000.0).round() / 1000000000000000.0;
    let mut denomin = (0,1);
    let mut denomax = (1,1);
    let mut iter = 0;
    let mut mediant = ((denomin.0 + denomax.0) as f64)/((denomin.1 + denomax.1) as f64);
    while iter < 10000000 &&  mediant != decimal {
        if decimal > mediant {
            denomin = (denomin.0 + denomax.0, denomin.1 + denomax.1);
        } else {
            denomax = (denomin.0 + denomax.0, denomin.1 + denomax.1);
        }
        iter+=1;
        mediant =  ((denomin.0 + denomax.0) as f64)/((denomin.1 + denomax.1) as f64);
    }
    return ((denomin.0 + denomax.0) + (num as i64)*(denomin.1 + denomax.1), denomin.1 + denomax.1);
}
