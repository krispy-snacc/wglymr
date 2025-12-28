"use client";

import { useEffect, useState } from "react";

/**
 * ClientOnly component - Only renders children on the client side
 * This prevents hydration mismatches from browser extensions
 */
export function ClientOnly({ children }: { children: React.ReactNode }) {
    const [hasMounted, setHasMounted] = useState(false);

    useEffect(() => {
        setHasMounted(true);
    }, []);

    if (!hasMounted) {
        return null;
    }

    return <>{children}</>;
}
