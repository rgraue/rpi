import React from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import { Banner } from './components/banner';
import { AuthProvider } from './auth/authProvider';
import { AuthService } from './auth/authService';

import {SERVER_URL} from './config.json';

const root = document.getElementById('root')!;

// up to you to gaurd this for nonprod use only
new EventSource('/esbuild').addEventListener('change', () => location.reload())

const authService = new AuthService({
    token_url: `${SERVER_URL}/auth/token`,
    describe_token_url: `${SERVER_URL}/auth/describe`
})

createRoot(root).render(
    <AuthProvider authService={authService}>
        <BrowserRouter>
            <Banner />
        </BrowserRouter>
    </AuthProvider>
)