const express = require("express");
const app = express();

app.use(express.json());

app.get("/auction", (req, res) => {
  res.json({
    highestBid: 100,
    highestBidder: "wallet_address",
  });
});

app.listen(3001, () => {
  console.log("Backend running on port 3001");
});
