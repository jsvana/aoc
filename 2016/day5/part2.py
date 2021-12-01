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


def sum_ok(md5, set_ids):
    try:
        num = int(md5[5])
    except ValueError:
        return False
    if num in set_ids:
        return False
    return md5.startswith('00000') and num >= 0 and num < 8


def main():
    door_id = 'reyedfim'
    idx = 0

    password = [' '] * 8
    set_ids = set()
    while len(set_ids) != 8:
        md5 = md5sum(door_id + str(idx))
        while not sum_ok(md5, set_ids):
            idx += 1
            md5 = md5sum(door_id + str(idx))
        pw_idx = int(md5[5])
        password[pw_idx] = md5[6]
        set_ids.add(pw_idx)
        print('set idx {}'.format(pw_idx))
        idx += 1

    print('Password is {}'.format(''.join(password)))


if __name__ == "__main__":
    main()
