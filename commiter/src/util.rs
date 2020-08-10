use num_primes::Generator;
use num_bigint::BigUint;
use num_traits::{One, Zero, Pow};
use std::cell::Cell;

pub fn get_sig (k: usize) -> (BigUint,BigUint,BigUint,BigUint){
    let f1: BigUint = One::one();
    let q  = Generator::new_prime(3*k/4);
    let qq = Generator::new_prime(k/4);
    let p: BigUint = &q * &qq +  &f1;
    let ta = Generator::new_uint(k) % &p;
    let tb = Generator::new_uint(k)% &p;
    return (q,qq,ta,tb);
}

pub fn get_h(p:BigUint, q:BigUint, g: BigUint, h: BigUint) -> impl Fn(&BigUint, &BigUint)-> BigUint + 'static {
    move |a,b| -> BigUint { (g.modpow(&(a % &q),&p) * h.modpow(&(b % &q),&p)) % &p}
}

pub fn str_to_int(s : &str) -> BigUint {
    BigUint::from_bytes_be(s.as_bytes())
}

pub fn hash_str(s: &str, h:impl Fn(&BigUint, &BigUint)-> BigUint) ->BigUint{
    let (a,b) = s.split_at(s.len()/2);
    return h(&str_to_int(a),&str_to_int(b))
}

pub fn int_to_hex(i:&BigUint) -> String{
    let bytes = i.to_bytes_be();
        let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02X}", b))
                               .collect();
        String::from("0x") + &strs.join("")
}


fn depth_bit_is(bit:&BigUint, support:&Vec<BigUint>, depth:usize) -> bool{
    let u:u8 = 2;
    let f2 = BigUint::from(u);
    let mask = bit % f2.pow(depth);
    let test = support.iter().map(|x| ((x % f2.pow(depth)) == mask)).collect::<Vec<bool>>();
    return test.contains(&true);
}

pub fn print_tree(k:usize, support:Vec<BigUint>, p:&BigUint,
    q:&BigUint, g:&BigUint,h:&BigUint, H:impl Fn(&BigUint, &BigUint)->BigUint) -> BigUint
{
    //Computes helper functions
    fn unimpl(_: usize,_:&BigUint) -> (BigUint,BigUint) { unimplemented!() }
    let fac_cell: Cell<& dyn Fn(usize,&BigUint) -> (BigUint,BigUint)> = Cell::new(&unimpl);

    let visitleaf = || {
        println!("{{");
        let a = Generator::new_uint(k) % q;
        let b = Generator::new_uint(k) % q;
        let e = Generator::new_uint(k) % q;
        let magic1:u32 = 0xf00d;
        let magic2:u32 = 0xba0bab;
        let m = H(&BigUint::from(magic1),&BigUint::from(magic2));
        let hv = h.modpow(&e,p);
        let r = &a;
        let c = (g.modpow(&m,p) * hv.modpow(&r,p)) % p;
        println!("\"a\":\"{}\",\"b\":\"{}\",\"e\":\"{}\",\"m\":\"{}\",
        \"c\":\"{}\",\"h\":\"{}\",\"r\":\"{}\"}}",int_to_hex(&a),int_to_hex(&b),int_to_hex(&e),
        int_to_hex(&m),int_to_hex(&c),int_to_hex(&hv),int_to_hex(&r));
        return (c,hv);
    };
    let visit_frontier = || {
        println!("{{");
        let a = Generator::new_uint(k) % q;
        let b = Generator::new_uint(k) % q;
        let e = Generator::new_uint(k) % q;
        let m = Zero::zero();
        let hv = g.modpow(&e,p);
        let r = Zero::zero();
        let c = g.modpow(&b,p);
        println!("\"a\":\"{}\",\"b\":\"{}\",\"e\":\"{}\",\"m\":\"{}\",
        \"c\":\"{}\",\"h\":\"{}\",\"r\":\"{}\"}}",int_to_hex(&a),int_to_hex(&b),int_to_hex(&e),
        int_to_hex(&m),int_to_hex(&c),int_to_hex(&hv),int_to_hex(&r));
        return (c,hv);
    };
    
    let visit_impl = |depth:usize, id:&BigUint| -> (BigUint,BigUint){
        let visit = fac_cell.get();
        if depth == k {return visitleaf()};
        println!("{{");
        println!("\"left\":");
        let u:u8 = 2;
        let f2 = BigUint::from(u);
        let left_code:&BigUint = id;
        let right_code:BigUint = (id)+ f2.pow(depth);
        let (cl,hl) = if depth_bit_is(left_code,&support,depth+1) {
            visit(depth+1,left_code)
        } else {visit_frontier()};
        println!(",\"right\":");
        let (cr,hr) = if depth_bit_is(&right_code,&support,depth+1) {
            visit(depth+1,&right_code)
        } else {visit_frontier()};
        let a = Generator::new_uint(k) % q;
        let b = Generator::new_uint(k) % q;
        let e = if depth == 0 {One::one()} else {Generator::new_uint(k) % q};
        let mut lv = cl.to_bytes_be();
        lv.append(&mut hl.to_bytes_be());
        let mut rv = cr.to_bytes_be();
        rv.append(&mut hr.to_bytes_be());
        let m = H(&BigUint::from_bytes_be(&lv),&BigUint::from_bytes_be(&rv));
        let hv = h.modpow(&e,p);
        let r = &a;
        let c = (g.modpow(&m,p) * hv.modpow(&r,p)) % p;
        println!(",\"depth\":{}",depth);
        println!(",\"a\":\"{}\",\"b\":\"{}\",\"e\":\"{}\",\"m\":\"{}\",
        \"c\":\"{}\",\"h\":\"{}\",\"r\":\"{}\"}}",int_to_hex(&a),int_to_hex(&b),int_to_hex(&e),
        int_to_hex(&m),int_to_hex(&c),int_to_hex(&hv),int_to_hex(&r));
        return (c,hv);
    };

    fac_cell.set(&visit_impl);
    let visit = &visit_impl;
    let (c,_) = visit(0,&Zero::zero());
    return c;
    }