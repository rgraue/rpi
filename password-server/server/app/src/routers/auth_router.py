from fastapi import APIRouter, Depends
from typing import Annotated

from auth import login, oauth2_scheme, OAuth2ClientCredentialsRequestForm

authRouter = APIRouter(
    prefix='/auth',
    tags=['auth'],
)

@authRouter.post('/token')
async def oath2_login(
    form: Annotated[OAuth2ClientCredentialsRequestForm, Depends()]
):
    print(form.__dict__)
    return login(form.client_id, form.client_secret)