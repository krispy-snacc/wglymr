"use client";

import { use, useState, useEffect } from "react";
import { Navbar } from "@/ui/Navbar";
import { LoadingScreen } from "@/ui/LoadingScreen";
import { GoldenLayoutHost } from "@/layout/GoldenLayoutHost";
import { MobileLayoutHost } from "@/layout/MobileLayoutHost";
import { GlymProvider } from "@/context/GlymContext";
import { useGlobalInputCapture } from "@/layout/useGlobalInputCapture";

interface PageProps {
    params: Promise<{ glym_id: string }>;
}

export default function GlymPage({ params }: PageProps) {
    const { glym_id } = use(params);
    const [isDesktop, setIsDesktop] = useState(false);
    const [isMounted, setIsMounted] = useState(false);

    useGlobalInputCapture();

    useEffect(() => {
        const checkScreenSize = () => {
            setIsDesktop(window.innerWidth >= 728);
        };

        checkScreenSize();
        window.addEventListener('resize', checkScreenSize);

        const mountTimer = setTimeout(() => setIsMounted(true), 300);

        return () => {
            window.removeEventListener('resize', checkScreenSize);
            clearTimeout(mountTimer);
        };
    }, []);

    return (
        <GlymProvider glymId={glym_id}>
            <LoadingScreen isLoading={!isMounted} />
            <div className="w-screen flex flex-col text-white overflow-hidden" style={{ height: 'calc(var(--vh, 1vh) * 100)' }}>
                <Navbar />

                <div className="flex-1 overflow-y-auto lg:overflow-hidden" style={{ height: 'calc(var(--vh, 1vh) * 100 - 3rem)' }}>
                    {isMounted && (isDesktop ? (
                        <div className="h-full min-h-0  overflow-hidden px-1 bg-black">
                            <GoldenLayoutHost glymId={glym_id} />
                        </div>
                    ) : (
                        <MobileLayoutHost glymId={glym_id} />
                    ))}
                </div>
            </div>
        </GlymProvider>
    );
}
