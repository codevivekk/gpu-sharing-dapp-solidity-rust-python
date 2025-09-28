import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";
import ContextProvider from "../../provider/wagmiprovider";
import { headers } from "next/headers";
import Navbar from "../components/Navbar";  

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata = {
  title: "GPU Sharing Platform",
  description: "A decentralized GPU sharing platform built on U2U Network",
};

export default async function RootLayout({ children }) {
  const headersObj = await headers();
  const cookies = headersObj.get('cookie')
  return (
    <html lang="en">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
      >
        <ContextProvider cookies={cookies}>
          <Navbar />
          {children}
        </ContextProvider>
      </body>
    </html>
  );
}
