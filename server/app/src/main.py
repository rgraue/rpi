import uvicorn
from auth import get_token_info, create_token, login
from rsuty_pstore_service import encode_base64, get_names, add_pass, get_pass_info
from fastapi.middleware.cors import CORSMiddleware
from fastapi import FastAPI

from routers.auth_router import authRouter
from routers.pass_store_router import pass_store_router


def other_main():
    # 09d25e094faa6ca2556c818166b7a9563b93f7099f6f0f4caa6cf63b88e8d3e7
    # rusty_pstore.init_pass_file(encode_base64('test'), 'test')

    # r = rusty_pstore.login('test', 'test')

    # if r is None:
    #     return
    
    # key = r[0]
    # iv = r[1]

    # print(key)
    # print(iv)

    # #         client_id: &str,
    #     # key: &str,
    #     # iv: &str,
    #     # name: &str,
    #     # username: &str,
    #     # password: &str,
    #     # url: &str,

    # # add = rusty_pstore.add_pass('test', key, iv, 'test', 'test', 'test', 'test')
    # # print(add)

    # names = rusty_pstore.get_names('test', key, iv)
    # print(names)

    # info = rusty_pstore.get_pass_info('test', key, iv, 'test')
    # print(info)

    token = login(encode_base64('test'), encode_base64('test'))
    print(token)

    data = get_token_info(token.access_token)
    print(data)

    # add_pass(data, encode_base64('test2'), encode_base64('test'), encode_base64('test'), encode_base64('test'))

    print(get_names(data))
    print(get_pass_info(data, encode_base64('test3')))

app = FastAPI()

def main():
    origins = ["*"]

    app.add_middleware(
        CORSMiddleware,
        allow_origins=origins,
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )
    app.include_router(authRouter)
    app.include_router(pass_store_router)

    uvicorn.run(app, host='0.0.0.0', port=8080)

if __name__ == '__main__':
    main()