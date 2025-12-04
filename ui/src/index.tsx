import React from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import { Banner } from './components/banner';
import { AuthProvider } from './auth/authProvider';
import { AuthService } from './auth/authService';

import { config } from './utils/config';

const root = document.getElementById('root')!;

// up to you to gaurd this for nonprod use only


(async () => {

    // for local dev with esbuild and watch server
    try {
        new EventSource('/esbuild').addEventListener('change', () => location.reload());
    } catch {}
    

    const activeConfig = await config();

    const authService = new AuthService({
        token_url: `${activeConfig.SERVER_URL}/auth/token`,
        describe_token_url: `${activeConfig.SERVER_URL}/auth/describe`
    })

    createRoot(root).render(
        <AuthProvider authService={authService}>
            <BrowserRouter>
                <Banner />
            </BrowserRouter>
        </AuthProvider>
    )
})()