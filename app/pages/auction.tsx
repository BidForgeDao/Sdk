import { useState } from "react";

export default function Auction() {
  const [bid, setBid] = useState("");

  const placeBid = async () => {
    console.log("Placing bid:", bid);
    // integrate with Solana later
  };

  return (
    <div style={{ padding: 40 }}>
      <h2>🔥 Live Auction</h2>

      <input
        type="number"
        placeholder="Enter bid"
        value={bid}
        onChange={(e) => setBid(e.target.value)}
      />

      <button onClick={placeBid}>Place Bid</button>
    </div>
  );
}
