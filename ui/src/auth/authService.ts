// import { URL, URLSearchParams } from 'url';

const PASSWORD_STORE_KEY = 'password-app-token'

interface TokenResponse {
    access_token: string;
    type_type: string;
}

export interface ClientCredentialsForm {
    grant_type: string;
    scope: string,
    client_id: string,
    client_secret: string
}

interface TokenDetails {
    sub: string;
    claims: any;
    exp: number;
    [vals: string]: any
}

type AuthState = 'AUTHORIZED' | 'UNAUTHORIZED' | 'AUTHORIZING';
type AuthRequestContentType = 'application/x-www-form-urlencoded' | 'application/json';

export interface AuthServiceConfig {
    token_url: string;
    request_content_type?: AuthRequestContentType;
    describe_token_url?: string;
    token_mapper?: (o: any) => string
}

export class AuthService<T = TokenResponse, F = ClientCredentialsForm> {
    // config
    token_url: string;
    request_content_type: AuthRequestContentType = 'application/x-www-form-urlencoded';
    token_mapper: (o: T) => string = this.defaultMapper;
    describe_token_url: string | undefined = undefined;

    // data
    token: string | undefined = undefined;
    token_details: TokenDetails | undefined = undefined;

    // states + internal info
    state: AuthState = 'UNAUTHORIZED';


    constructor(config: AuthServiceConfig) {
        this.token_url = config.token_url;

        if (config.request_content_type) {
            this.request_content_type = config.request_content_type
        }

        if (config.token_mapper) {
            this.token_mapper = config.token_mapper
        }
        this.describe_token_url = config?.describe_token_url;
    }

    private defaultMapper (o: T): string {
        return (o as TokenResponse).access_token
    }

    async logout () {
        localStorage.removeItem(PASSWORD_STORE_KEY);
        this.token = undefined;
        this.token_details = undefined;
        this.state = 'UNAUTHORIZED';
    }

    // specify a token mapper to parse out the jwt from the token url response
    async login (loginForm: F, tokenMapper: (o: T) => string = this.defaultMapper): Promise<boolean> {
        this.state = 'AUTHORIZING';

        const maybeToken = localStorage.getItem(PASSWORD_STORE_KEY);

        if (maybeToken && (await this.isAuthedFromToken(maybeToken))) {
            this.token = maybeToken;
            this.state = 'AUTHORIZED';
        }

        try {
            const loginAttempt = await fetch(
                this.token_url, 
                {
                    method: 'POST',
                    headers: {
                        'Content-Type' : this.request_content_type,
                    },
                    body: (this.request_content_type == 'application/json' ? loginForm : new URLSearchParams(loginForm as any)) as any
                }
            );

            if (!loginAttempt.ok) {
                throw new Error('non 200 from token url. ' + loginAttempt.statusText);
            }

            const tokenRespone = (await loginAttempt.json()) as T;
            console.log(tokenRespone);
            this.token = tokenMapper(tokenRespone);

            if (this.token) {
                localStorage.setItem(PASSWORD_STORE_KEY, this.token);
                this.describeToken(this.token);
            }

            this.state = 'AUTHORIZED';
            return true;

        } catch (e) {
            console.log(`ERROR::login = ${(e as Error).message}`)
            this.state = 'UNAUTHORIZED';
            return false;

        }
        

    }

    isAuthed(): boolean {
        return this.state == 'AUTHORIZED';
    }

    private isTokenExpired(exp: number): boolean {
        return (exp > new Date().getTime());
    }

    private async isAuthedFromToken (token: string) {
        await this.describeToken(token);

        // if we can describe the token
        if (this.token_details) {
            return !this.isTokenExpired(this.token_details.exp);
        }

        return false;
    }

    // GET describe_token_url
    private async describeToken(token: string): Promise<void> {
        try {
            if (!this.describe_token_url) {
                return undefined;
            }
            const describeResult = await fetch(
                new URL(this.describe_token_url), 
                {
                    method: 'GET',
                    'headers': {
                        'Authorization': 'Bearer ' + token,
                        'Accept': 'application/json'
                    },
                });
            

            const status = describeResult.status;
            if (status == 200) {
                this.token_details = (await describeResult.json()) as TokenDetails;
            } else {
                // pass error code to exception block
                throw new Error(status.toString());
            }
        } catch (e) {
            const message = (e as Error).message;

            // silent the describe exception.
            // if we got to this describe step it means we have a token to begin with.
            // let the user handle
            const maybeCode = parseInt(message);
            if (!Number.isNaN(maybeCode) && maybeCode != 200) {
                console.log(`ERROR::describeToken = response status code ${message} from ${this.describe_token_url}`);
            } else {
                console.log(`ERROR::describeToken ${message}`);
            }
        }
    }
}