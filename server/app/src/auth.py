import os
import jwt
from typing import Annotated, Optional
from datetime import datetime, timedelta, timezone

from pydantic import BaseModel
from fastapi import Depends, FastAPI, HTTPException, status, Form
from fastapi.security import OAuth2
from fastapi.openapi.models import OAuthFlows as OAuthFlowsModel
from fastapi.security.utils import get_authorization_scheme_param
from starlette.requests import Request
from starlette.status import HTTP_401_UNAUTHORIZED

from rsuty_pstore_service import login as service_login
from models import Token, TokenData

# to get a string like this run:
# openssl rand -hex 32
SECRET_KEY = os.getenv('JWT_SECRET')
ALGORITHM = "HS256"
ACCESS_TOKEN_EXPIRE_MINUTES = 30

UNAUTHORIZED_EXCEPTION = HTTPException(
        status_code=status.HTTP_401_UNAUTHORIZED,
        detail="Could not validate credentials",
        headers={"WWW-Authenticate": "Bearer"},
    )

class Oauth2ClientCredentials(OAuth2):
    def __init__(
        self,
        tokenUrl: str,
        scheme_name: str = 'oAuth2ClientCredentials',
        scopes: dict = None,
        auto_error: bool = True,
    ):
        if not scopes:
            scopes = {}
        flows = OAuthFlowsModel(clientCredentials={"tokenUrl": tokenUrl, "scopes": scopes})
        super().__init__(flows=flows, scheme_name=scheme_name, auto_error=auto_error)

    async def __call__(self, request: Request) -> Optional[str]:
        authorization: str = request.headers.get("Authorization")
        scheme, param = get_authorization_scheme_param(authorization)
        if not authorization or scheme.lower() != "bearer":
            if self.auto_error:
                raise HTTPException(
                    status_code=HTTP_401_UNAUTHORIZED,
                    detail="Not authenticated",
                    headers={"WWW-Authenticate": "Bearer"},
                )
            else:
                return None
        return param
    
class OAuth2ClientCredentialsRequestForm:
    def __init__(
        self,
        grant_type: str = Form(None, regex="client_credentials"),
        scope: str = Form(""),
        client_id: Optional[str] = Form(None),
        client_secret: Optional[str] = Form(None),
    ):
        self.grant_type = grant_type
        self.scopes = scope.split()
        self.client_id = client_id
        self.client_secret = client_secret

oauth2_scheme = Oauth2ClientCredentials(tokenUrl="auth/token")

def describe_token(token: Annotated[str, Depends(oauth2_scheme)]):
    try:
        return jwt.decode(token, SECRET_KEY, algorithms=[ALGORITHM])
    except Exception:
        raise UNAUTHORIZED_EXCEPTION

def get_token_info(token: Annotated[str, Depends(oauth2_scheme)]):
    try:
        data = jwt.decode(token, SECRET_KEY, algorithms=[ALGORITHM])
        client_id = data.get('sub')
        claims = data.get('claims')

        key = claims.get('key')
        iv = claims.get('iv')

        return TokenData(client_id=client_id, key=key, iv=iv)
    except Exception:
        raise UNAUTHORIZED_EXCEPTION
    
def create_token(base64_client_id: str, base64_key: str, base64_iv: str):
    data = {
        'sub': base64_client_id,
        'claims': {
            'key' : base64_key,
            'iv': base64_iv 
        }
    }

    expire = datetime.now(timezone.utc) + timedelta(minutes= ACCESS_TOKEN_EXPIRE_MINUTES)
    data.update({'exp': expire})
    access_token = jwt.encode(data, SECRET_KEY, algorithm=ALGORITHM)

    return Token(access_token=access_token)

def login(base64_client_id: str, base64_password: str):
    try:
        print(f"logging in {base64_client_id}")
        login_data = service_login(base64_client_id, base64_password)

        return create_token(login_data['client_id'], login_data['key'], login_data['iv'])
    except Exception:
        raise UNAUTHORIZED_EXCEPTION
    