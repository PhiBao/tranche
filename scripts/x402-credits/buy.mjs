// Buys Reppo Evaluation API credits over x402 (USDC on Base).
//
// Usage (from the project root):
//   node scripts/x402-credits/buy.mjs [USD_AMOUNT]
// Default: $1 (1 USDC = 2,000 credits ≈ 4 evaluations). $5 ≈ 11 evaluations.
//
// The wallet private key is read from the environment (WALLET_PRIVATE_KEY or
// PRIVATE_KEY) or from backend/.env if present. It is never printed, never
// written to any file, and never sent anywhere except the x402 signer.
//
// The credit token is returned by Reppo exactly once on first purchase.
// Copy it into backend/.env as REPPO_CREDIT_TOKEN (and as a Fly secret).
// Anyone holding the token can spend the credits — treat it as a secret.

import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import path from "node:path";
import axios from "axios";
import { wrapAxiosWithPayment, x402Client } from "@x402/axios";
import { registerExactEvmScheme } from "@x402/evm/exact/client";
import { privateKeyToAccount } from "viem/accounts";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");

function loadEnvFile() {
  const envPath = path.join(root, "backend", ".env");
  if (!existsSync(envPath)) return;
  for (const line of readFileSync(envPath, "utf8").split("\n")) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#") || !trimmed.includes("=")) continue;
    const idx = trimmed.indexOf("=");
    const key = trimmed.slice(0, idx).trim();
    const value = trimmed.slice(idx + 1).trim().replace(/^["']|["']$/g, "");
    if (key && !(key in process.env)) process.env[key] = value;
  }
}
loadEnvFile();

const pk =
  process.env.WALLET_PRIVATE_KEY || process.env.PRIVATE_KEY || process.env.WALLET_PK || "";
if (!/^0x[0-9a-fA-F]{64}$/.test(pk)) {
  console.error("Set WALLET_PRIVATE_KEY=0x... in backend/.env (do not paste it in chat).");
  process.exit(1);
}

const usdArg = process.argv[2] || "1";
const usd = Number.parseInt(usdArg, 10);
if (!Number.isInteger(usd) || usd < 1 || usd > 100) {
  console.error("Amount must be a whole number of USDC from 1 to 100.");
  process.exit(1);
}

const account = privateKeyToAccount(pk);
console.log(`wallet: ${account.address}`);
console.log(`buying: $${usd} of Reppo credits`);

const client = new x402Client();
registerExactEvmScheme(client, { signer: account });
const api = wrapAxiosWithPayment(axios.create(), client);

const { data } = await api.post(`https://eval.reppo.xyz/api/v1/x402/credits?usd=${usd}`);
console.log("response:", JSON.stringify(data, null, 2));
if (data.creditToken) {
  console.log("\nSAVE THIS TOKEN (shown once) -> backend/.env as REPPO_CREDIT_TOKEN");
  console.log(`REPPO_CREDIT_TOKEN=${data.creditToken}`);
} else {
  console.log("\nTop-up complete. The previously saved credit token still works.");
}
