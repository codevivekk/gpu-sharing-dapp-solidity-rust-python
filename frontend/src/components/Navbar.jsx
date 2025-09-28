import React from "react";
import ConnectButton from "./ConnectBtn";
import Link from "next/link";

export default function Navbar() {
  return (
    <nav className="bg-blue-600 text-white px-6 py-4 flex items-center justify-between">
      <ConnectButton />

      <div className="space-x-4">
        <Link href="/" className="hover:underline">
          HomePage
        </Link>
        <Link href="/node-register" className="hover:underline">
          Add Node
        </Link>
        <Link href="/all-jobs" className="hover:underline">
          All Jobs
        </Link>
        {/* <Link href="/provider-register" className="hover:underline">
          Provider Register
        </Link> */}
      </div>
    </nav>
  );
}
