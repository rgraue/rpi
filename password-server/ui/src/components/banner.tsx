import React, { useState } from 'react';


export const Banner = () => {

    const [count, setCount] = useState(0);

    const onClick = () => {
        console.log(`incrementing count to ${count + 1}`);
        setCount(count + 1);
    }

    return (
        <>
            <h1>Hello</h1>
            <button onClick={onClick}>click me</button>
            <p>{count}</p>
        </>
    )
}