export abstract class TokenError extends Error {
    constructor(msg?: string) {
        super(msg);
    }
}

export class TokenAccountNotFoundError extends TokenError {
    name = "TokenAccountNotFoundError"
}

export class TokenInvalidAccountOwnerError extends TokenError{
    name = "TokenInvalidAccountOwnerError"
}

export class TokenInvalidAccountSizeError extends TokenError{
    name = "TokenInvalidAccountSizeError"
}