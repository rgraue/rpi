from fastapi import APIRouter, Depends
from typing import Annotated

from models import TokenData
from auth import get_token_info
from rsuty_pstore_service import get_names

pass_store_router = APIRouter(
    prefix='/pass',
    tags=['pass']
)

@pass_store_router.get('/names')
async def get_pass_names(authenticated_client: Annotated[TokenData, Depends(get_token_info)]):
    return get_names(authenticated_client)