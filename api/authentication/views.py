from typing import Annotated
from fastapi import APIRouter, Cookie, HTTPException
from jwt.exceptions import InvalidTokenError

from ..user.models import User

router = APIRouter(prefix="/auth", tags=["auth"])


def decode_session_token(__session: str) -> dict:
    return {}


def clerk_get_user(secret_key: str, subject: str) -> str:
    return user_id


@router.get("/login")
async def login(db: DbDep, __session: Annotated[str | None, Cookie()] = None):
    try:
        token = decode_session_token()
        user = db.get(User, token["sub"])
        if not user:
            clerk_user = clerk_get_user(settings.secret_key, token["sub"])
            db.add(User(id=clerk_user.id))
        return {"message": "user logged in"}
    except (InvalidTokenError, SDKError, ClerkErrors):
        raise HTTPException(status_code=401, detail="Invalid Session Token")


@router.get("/logout")
async def logout():
    pass
