import React from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter } from 'react-router';
import { Banner } from './components/banner';

const root = document.getElementById('root')!;

// up to you to gaurd this for nonprod use only
new EventSource('/esbuild').addEventListener('change', () => location.reload())

createRoot(root).render(
    <BrowserRouter>
        <Banner />
    </BrowserRouter>
)