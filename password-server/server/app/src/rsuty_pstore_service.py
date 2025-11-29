import rusty_pstore
import base64

from models import TokenData, PassInfo

def decode_base64(base64_s: str):
    return base64.b64decode(base64_s.encode()).decode()

def encode_base64(plain_text: str):
    return base64.b64encode(plain_text.encode()).decode()

def login(base64_client_id: str, base64_password: str):
    plaintext_pwd = decode_base64(base64_password)

    login_attempt: list[str] = rusty_pstore.login(base64_client_id, plaintext_pwd)

    if login_attempt is None or len(login_attempt) != 2:
        raise RuntimeError(f'Login attempt failed for {decode_base64(base64_client_id)}')
    
    return {
        'client_id': base64_client_id,
        'key': login_attempt[0],
        'iv': login_attempt[1]
    }

def get_names(token: TokenData):
    return rusty_pstore.get_names(token.client_id, token.key, token.iv)

def add_pass(token: TokenData, base64_name: str, base64_username: str, base64_password: str, base64_url: str):
    plaintext_name = decode_base64(base64_name)
    plaintext_username = decode_base64(base64_username)
    plaintext_password = decode_base64(base64_password)
    plaintext_url = decode_base64(base64_url)

    success: bool = rusty_pstore.add_pass(
        token.client_id, 
        token.key, 
        token.iv, 
        plaintext_name, 
        plaintext_username, 
        plaintext_password, 
        plaintext_url
    )

    if success is False:
        raise RuntimeError('Error saving pass info')
    
def get_pass_info(token: TokenData, base64_looking_for: str):
    plaintext_looking_for = decode_base64(base64_looking_for)

    details = rusty_pstore.get_pass_info(
        token.client_id, 
        token.key, 
        token.iv, 
        plaintext_looking_for
    )
    
    if details is None or len(details) < 3:
        raise RuntimeError(f'Error retrieving pass info for {token.client_id}: {plaintext_looking_for}')
    
    return PassInfo(
        name=plaintext_looking_for,
        username=details[0],
        password=details[1],
        url=details[2]
    )