import json
import sys


def modPow(base, exponent, modulus):
    if exponent < 0:
        raise TypeError('Negative exponent: ' + exponent)
    if base == 0 or modulus == 0 or modulus == 1:
        return 0
    if exponent == 0:
        return 1
    result = 1
    while exponent > 0:
        if exponent % 2:
            result = (result * base) % modulus
        exponent >>= 1
        base = (base * base) % modulus
    return result


def verify(sigma, proof):
    [q, qq, t, tt] = map(lambda x: int(x, 16), json.loads(sigma))
    p = q * qq + 1
    g = t ** qq % p
    gg = modPow(t,qq,p)
    h = tt ** qq % p

    def p_hash(a, b):
        return (modPow(g,a % q,p) * modPow(h, b % q, p)) % p

    def commit(hv, mv, rv):
        return (modPow(g, mv, p) * modPow(hv, rv, p)) % p

    proof = json.loads(proof)

    hp = [hex(h)]
    for statement in proof:
        e = int(statement["e"], 0)
        r = int(statement["r"], 0)
        c = int(statement["c"], 0)
        hv = modPow(h,e,p)
        m = 0
        if hv not in map(lambda x: int(x, 16), hp):
            return False
        if not statement.get("y"):
            c0 = statement.get("c0")
            h0 = statement.get("h0")
            c1 = statement.get("c1")
            h1 = statement.get("h1")
            hp = [h0, h1]
            vl = int(c0 + h0[2:], 0)
            vr = int(c1 + h1[2:], 0)
            m = p_hash(vl, vr)
        else:
            [m1, m2] = statement.get("y")
            if [m1, m2] != ["0xf00d", "0xba0bab"]:
                return False
            m = p_hash(int(m1, 0), int(m2, 0))
        cv = commit(hv, m, r)
        if c != cv:
            return False
    return p_hash(*(map(lambda x: int(x, 16), hp)))


def main():
    if len(sys.argv) < 3:
        exit(1)
    sigma_file_name = sys.argv[1]
    file_name = sys.argv[2]
    sigma_str = ""
    proof_str = ""
    with open(sigma_file_name) as f:
        for line in f:
            sigma_str += line
    with open(file_name) as f:
        for line in f:
            proof_str += line
    result = verify(sigma_str, proof_str)
    print(hex(result))


if __name__ == "__main__":
    main()
