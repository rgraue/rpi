import rusty_pstore

# rusty_pstore.init_pass_file('test', 'test')
def main():
    r = rusty_pstore.login('test', 'test')

    if r is None:
        return
    
    key = r[0]
    iv = r[1]

    print(key)
    print(iv)

    #         client_id: &str,
        # key: &str,
        # iv: &str,
        # name: &str,
        # username: &str,
        # password: &str,
        # url: &str,

    # add = rusty_pstore.add_pass('test', key, iv, 'test', 'test', 'test', 'test')
    # print(add)

    names = rusty_pstore.get_names('test', key, iv)
    print(names)

    info = rusty_pstore.get_pass_info('test', key, iv, 'test')
    print(info)

if __name__ == '__main__':
    main()