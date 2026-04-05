import Link from "next/link";

export default function Home() {
  return (
    <div style={{ padding: 40 }}>
      <h1>🚀 BidForge</h1>
      <p>Win the auction. Define the token.</p>

      <Link href="/auction">
        <button>Go to Auction</button>
      </Link>
    </div>
  );
}
