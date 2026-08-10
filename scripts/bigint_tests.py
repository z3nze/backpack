def get_digits(x):
    base = 1 << 64
    res = []
    while x > 0:
        res.append(x % base)
        x //= base
    if len(res) == 0:
        res.append(0)

    return res

def main():
    a = 19823749182374901273490812734908219384719283750912735890
    b = 2347057078432058273045720348
    c = 194632

    print(get_digits(a))
    print(get_digits(b))
    print("a + b: {}", get_digits(a + b))
    print("a - b: {}", get_digits(a - b))
    print("a * b: {}", get_digits(a * b))
    print("a / b: {}", get_digits(a // b))
    print("a % b: {}", get_digits(a % b))

    print("a / c: {}", get_digits(a // c))
    print("a % c: {}", get_digits(a % c))

main()
