import sys


def aba(chars):
    return chars[0] == chars[2] and chars[0] != chars[1]


def get_abas(line):
    in_bracket = False
    matches = False

    abas = set()
    babs = set()
    for i in range(len(line) - 2):
        word = line[i:i+3]
        if word[0] == '[':
            in_bracket = True
            continue
        if word[0] == ']':
            in_bracket = False
            continue
        if any([c in word for c in '[]']):
            continue
        if aba(word):
            if in_bracket:
                babs.add(word)
            else:
                abas.add(word)
    return abas, babs


def opposite(aba):
    return aba[1] + aba[0] + aba[1]


def supports_ssl(line):
    abas, babs = get_abas(line)
    opposite_abas = {opposite(aba) for aba in abas}
    return len(babs & opposite_abas) > 0


def main():
    if len(sys.argv) != 2:
        print('Usage: {} <input>'.format(sys.argv[0]), file=sys.stderr)
        sys.exit(1)

    tests = [
        ('aba[bab]xyz', True),
        ('xyx[xyx]xyx', False),
        ('aaa[kek]eke', True),
        ('zazbz[bzb]cdb', True),
    ]

    for test, expected in tests:
        if supports_ssl(test) == expected:
            print('pass {}'.format(test))
        else:
            print('fail {}'.format(test))

    tls = 0
    with open(sys.argv[1], 'r') as f:
        for line in f.readlines():
            if supports_ssl(line):
                tls += 1
    print(tls)


if __name__ == '__main__':
    main()
