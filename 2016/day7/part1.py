import sys


def abba(chars):
    return (
        chars[0] == chars[3]
        and chars[1] == chars[2]
        and chars[0] != chars[1]
    )


def supports_tls(line):
    in_bracket = False
    matches = False
    for i in range(len(line) - 3):
        word = line[i:i+4]
        if word[0] == '[':
            in_bracket = True
            continue
        if word[0] == ']':
            in_bracket = False
            continue
        if any([c in word for c in '[]']):
            continue
        if abba(word):
            if in_bracket:
                return False
            else:
                matches = True
    return matches


def main():
    if len(sys.argv) != 2:
        print('Usage: {} <input>'.format(sys.argv[0]), file=sys.stderr)
        sys.exit(1)

    tests = [
        ('abba[mnop]qrst', True),
        ('abcd[bddb]xyyx', False),
        ('aaaa[qwer]tyui', False),
        ('ioxxoj[asdfgh]zxcvbn', True),
        ('ab[asdf]ba', False),
        ('aba[foobar]asdf', False),
        ('ab[bafoobar]asdf', False),
        ('ab[bafoobar]abba[asdf]asdf', True),
        ('ab[ba]abba[a]a', True),
        ('abba[qwwq]', False),
        ('[abba]qwwq', False),
        ('abba[abba]qwwq', False),
        ('as[abba]df', False),
        ('as[aaaa]df', False),
        ('c[d]abba[e]f', True),
        ('c[d]abba[asdf][e]f', True),
        ('abudxncgozbrbnx[fllpjgocynbuyawgs]hiphrvpugpfnnppn[jhmlgjsufflkdgw]ldmdclrkorzjtbjqcrn', True),
        ('lwnhrcbjrjqarzdx[jezkqlffqqbioajjbnl]zssjjsdouwbegdbnxx', True),
        ('urlkduvyyyatpkb[zrolecowduswyfn]bgkveercmmeecop', False),
        ('ekecthrkwdbjhsy[klxbdnucasemwhlpjj]jbvenwrnvynlfyjybm[zgfxxurrduhtlmsbelf]lxuxlahnrqvjssffj[wzcpjiesgsbwbtnlrs]sofzsskbviyfvlo', False),
        ('wpvcqnrvyjvfkfpclz[wogcckufvzviggf]oulptksetgaaholu[dwwcwhkktrhgkahbs]sobrvezzrrzvlihicw', True),
    ]

    for test, expected in tests:
        if supports_tls(test) == expected:
            print('pass {}'.format(test))
        else:
            print('fail {}'.format(test))

    tls = 0
    with open(sys.argv[1], 'r') as f:
        for line in f.readlines():
            if supports_tls(line):
                tls += 1
    print(tls)


if __name__ == '__main__':
    main()
