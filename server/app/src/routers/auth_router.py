from fastapi import APIRouter, Depends
from typing import Annotated, Any

from auth import login, OAuth2ClientCredentialsRequestForm, describe_token

authRouter = APIRouter(
    prefix='/auth',
    tags=['auth'],
)

@authRouter.post('/token')
async def oath2_login(
    form: Annotated[OAuth2ClientCredentialsRequestForm, Depends()]
):
    return login(form.client_id, form.client_secret)

@authRouter.get('/describe')
async def oauth2_describe(authenticated_client: Annotated[Any, Depends(describe_token)]):
    return authenticated_client