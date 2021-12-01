import hashlib


def md5sum(string):
    m = hashlib.md5()
    m.update(string.encode())
    return m.hexdigest()


def next_char(string, idx):
    md5 = md5sum(string + str(idx))
    while not md5.startswith('00000'):
        idx += 1
        md5 = md5sum(string + str(idx))

    return md5[5], idx


def main():
    door_id = 'reyedfim'
    idx = 0

    password = ''
    for _ in range(8):
        char, idx = next_char(door_id, idx)
        idx += 1
        password += char

    print('Password is {}'.format(password))


if __name__ == "__main__":
    main()
