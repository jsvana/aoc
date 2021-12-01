import sys


def decompressed_length(line):
    length = 0
    while line:
        if line[0] == '(':
            end = line.find(')')
            chars, repeat = [int(v) for v in line[1:end].split('x')]
            line = line[end + chars:]
            print('foo', line)
            print('add paren ', chars * repeat)
            length += chars * repeat
        else:
            end = line.find('(')
            if end == -1:
                length += len(line)
                print('add len ', len(line))
                break
            line = line[end:]
            length += end
            print('add end ', end)
    return length


def main():
    tests = [
        ('ADVENT', 6),
        ('A(1x5)BC', 7),
        ('(3x3)XYZ', 9),
        ('A(2x2)BCD(2x2)EFG', 11),
        ('(6x1)(1x3)A', 6),
        ('X(8x2)(3x3)ABCY', 18),
    ]

    for line, expected in tests:
        result = decompressed_length(line)
        if result == expected:
            print('passed {}'.format(line))
        else:
            print('failed {} (expected {} but got {})'.format(line, expected, result))

    sys.exit(0)
    with open(argv[1], 'r') as f:
        line = f.readlines()[0]




if __name__ == '__main__':
    main()
