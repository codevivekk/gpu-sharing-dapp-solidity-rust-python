"use client";

import { wagmiAdapter, projectId } from "../config";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { createAppKit, useAppKitAccount } from "@reown/appkit/react";
import { mainnet, arbitrum, sepolia } from "@reown/appkit/networks";
import React from "react";
import { cookieToInitialState, WagmiProvider } from "wagmi";
import ConnectButton from "@/components/ConnectBtn";

// Set up queryClient
const queryClient = new QueryClient();

if (!projectId) {
    throw new Error("Project ID is not defined");
}

const u2uTestnet = {
    id: 2484,
    name: "Unicorn Ultra Nebulas Testnet",
    rpcUrl: "https://rpc-nebulas-testnet.uniultra.xyz",
    chainId: 2484,
    nativeCurrency: {
        name: "U2U",
        symbol: "U2U",
        decimals: 18,
    },
    blockExplorerUrls: ["https://testnet.u2uscan.xyz"],
};

// Set up metadata
const metadata = {
    name: "appkit-example",
    description: "AppKit Example",
    url: "https://appkitexampleapp.com", // origin must match your domain & subdomain
    icons: ["https://avatars.githubusercontent.com/u/179229932"],
};

// Create the modal
const modal = createAppKit({
    adapters: [wagmiAdapter],
    projectId,
    networks: [ u2uTestnet],
    defaultNetwork: u2uTestnet,
    metadata: metadata,
    features: {
        analytics: true, // Optional - defaults to your Cloud configuration
    },
});

function ContextProvider({ children, cookies }) {
    const initialState = cookieToInitialState(wagmiAdapter.wagmiConfig, cookies);
    const { isConnected } = useAppKitAccount();




    return (
        <QueryClientProvider client={queryClient}>
            <WagmiProvider
                config={wagmiAdapter.wagmiConfig}
                initialState={initialState}
            >
                {!isConnected ? (
                    <div className="flex flex-col justify-center items-center h-screen ">
                        <p className="mb-2">Please Connect Your Wallet to Continue</p>
                        <ConnectButton />
                    </div>
                ) : (
                    children
                )}
            </WagmiProvider>
        </QueryClientProvider>
    );

}

export default ContextProvider;
