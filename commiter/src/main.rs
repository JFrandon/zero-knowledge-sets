mod util;
use num_bigint::BigUint;
use num_traits::{One};
use util::{get_sig, hash_str, int_to_hex, print_tree};
use std::{env, fs};
///
/// fn main(){
///     // Outputs a Large Prime with 512 bits
///     let p = _primesGenerator::new_prime(512);
///     let q = Generator::new_prime(512);
///
///     // Multiplies p times q and returns the product
///     let n = p * q;
/// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mails:Vec<&str> = contents.split("\n").collect();
    println!("{{");
    //let mails = ["jeremy.frandon@mail.mcgill.ca","tst2"];
    let l = &mails.iter().map(|x| x.len()).max().unwrap();
    let k = if args.len() >= 3 {args[2].parse::<usize>().unwrap()} else {3 * 8 * l / 2};
    println!("\"k\": \"{}\"", k);
    let f1: BigUint = One::one();
    let s = get_sig(k);
    let (q,qq,t,b) = &s;
    let sigma = [q,qq,t,b];
    let sig_str = sigma.iter().map(|x| int_to_hex(x)).collect::<Vec<String>>().join("\",\"");
    println!(",\"sigma\": [\"{}\"]", &sig_str);

    let p = q*qq + &f1;
    let g = t.modpow(&qq,&p);
    let h = b.modpow(&qq,&p);
    let hash = util::get_h(p.clone(),q.clone(),g.clone(),h.clone());
    let support = &mails.iter().map(|x| hash_str(&x,&hash)).collect::<Vec<BigUint>>();
    
    
    println!(",\"secret\":");
    let public = print_tree(k,support.to_vec(),&p,&q,&g,&h,&hash);
    println!(",\"public\":\"{}\"}}",int_to_hex(&public));
}
