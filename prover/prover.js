var fs = require('fs'),
    path = require('path')

function modPow(base, exponent, modulus){
    if (exponent < 0n) {
        throw new TypeError('Negative exponent: ' + exponent);
    }
    if (base === 0n || modulus === 0n || modulus === 1n) {
        return 0n;
    }
    if (exponent === 0n) {
        return 1n;
    }    
    let result = 1n;
    while (exponent > 0n) {
        if ((exponent & 1n) === 1n) {
            result = (result * base) % modulus;
        }
        exponent >>= 1n;
        base = (base * base) % modulus;
    }
    return result;
}

function prove(data_text,mail){
    let data = JSON.parse(data_text)
    const [q,qq,t,b] = data.sigma.map(BigInt)
    const sk = data.secret

    const p = q*qq+1n
    const g = t ** qq % p
    const h = b ** qq % p
    
    function hash(a,b){
        return modPow(g, a%q,p) * modPow(h, b%q,p) % p;
    }
    function hash_str(s){
        let a = BigInt("0x"+ s.substr(0,s.length/2).split("").map(x => x.charCodeAt(0)).map(x => x.toString(16)).join(""))
        let b = BigInt("0x"+s.substr(s.length/2).split("").map(x => x.charCodeAt(0)).map(x => x.toString(16)).join(""))
        return hash(a,b)
    }   
    let key = hash_str(mail)
    let vertex = sk
    let proof =[]
    while(vertex.left && vertex.right){
        e = vertex.e
        r = vertex.r
        c = vertex.c
        c0 = vertex.left.c
        h0 = vertex.left.h
        c1 = vertex.right.c
        h1 = vertex.right.h
        proof.push({e:e,r:r,c:c,c0:c0,h0:h0,c1:c1,h1:h1})
        if(key % 2n){
            vertex = vertex.right
        }else{
            vertex = vertex.left
        }
        key /= 2n
    }
    proof.push({e:vertex.e,r:vertex.r,c:vertex.c,y:["0xf00d","0xba0bab"]})
    return proof
}

function main(){
    let args = process.argv; //arg0: js interpreter, arg1: this file, arg2: SK, arg3: email addr
    if (args.length < 4) process.exit(1);
    let data_text = fs.readFileSync(path.join(__dirname, args[2])).toString();
    let email = args[3]
    proof = prove(data_text, email)
    console.log(JSON.stringify(proof))
}

try{main()}catch{}