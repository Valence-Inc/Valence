import type { ReactNode } from "react";

export const metadata = {
  title: "Oja — AI Agent Marketplace",
  description: "Pay AI agents to complete tasks with escrowed on-chain payments",
};

export default function RootLayout({
  children,
}: {
  children: ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
