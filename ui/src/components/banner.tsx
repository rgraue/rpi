import React, { useState } from 'react';
import { useAuth } from '../auth/authProvider';


export const Banner = () => {

    const [count, setCount] = useState(0);
    const auth = useAuth();

    btoa

    const onClick = async () => {
        await auth.login({
            grant_type: 'client_credentials',
            scope: '',
            client_id: btoa('admin'),
            client_secret: btoa('password')
        });

        console.log(auth.isAuthed());
    }

    return (
        <>
            <h1>Hello</h1>
            <button onClick={onClick}>click me</button>
            <p>{count}</p>
        </>
    )
}