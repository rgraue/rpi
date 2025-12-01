import React, { createContext, useContext, useState } from "react";
import { AuthService } from "./authService";

const authContext = createContext<AuthService | undefined>(undefined);

export const useAuth = () => {
    const context = useContext(authContext);
    if (!context) {
        throw new Error("useAuth must be with AuthProvider");
    }

    return context;
}

export const AuthProvider = ({ children, authService }: { children: any; authService: AuthService }) => {
    const [ authState, setAuthState ] = useState<AuthService>(authService);

    return (
        <authContext.Provider value={authState}>
            {children}
        </authContext.Provider>
    )
}