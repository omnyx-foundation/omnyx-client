# Nexon
Nexon is a fully custom proprietary client written by HPC and HFT engineers designed to land transactions as fast as possible. 

## sendTransaction
Nexon supports the `sendTransaction` Solana RPC method. Just insert the URL where you would place a RPC URL. Transactions are sent through 1) Self hosted nodes 2) Staked connections and 3) Jito bundles. 
HTTP POST Body
```bash
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "sendTransaction",
    "params": [ 
        "<base64_encoded_tx>",
        { "encoding": "base64" }
    ] 
}
```
## Priority Fee
Nexon gets your transaction to the scheduler. At this point, priority fees matter. It is recommended to set CU price to at least `1,000,000`, or expect subpar performance.
## Retries
Nexon will retry transactions until confirmation or expiry. Nexon will prioritize retries based on the amount tipped.
## Rate Limits
Each API key has an associated rate limit.
## Optimized Routing
What makes Nexon unique is how it routes transactions through the TPU. It knows exactly when and how to send transactions.

Nexon does not simulate transactions.
## Rust
Use full service rpc for getting blockhash. Nexon only support sendTransaction.
```Rust

const Nexon_TIP: Pubkey = pubkey!("TEMPaMeCRFAS9EKF53Jd6KpHxgL47uWLcpFArU1Fanq");
const MIN_TIP_AMOUNT: u64 = 1_000_000;

fn send_nexon_tx(ixs: &mut Vec<Instruction>, signer: &Keypair, rpc_client: &RpcClient) {
    let tip_ix = system_instruction::transfer(&signer.pubkey(), &NEXON_TIP, MIN_TIP_AMOUNT);
    ixs.push(tip_ix);

    let blockhash = rpc_client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(ixs, Some(&signer.pubkey()), &[signer], blockhash);

    rpc_client.send_transaction(&tx).unwrap();
}
```
## Python
Use full service rpc for getting blockhash. Nexon only support sendTransaction.
```Python
NEXON_TIP = PublicKey("TEMPaMeCRFAS9EKF53Jd6KpHxgL47uWLcpFArU1Fanq") MIN_TIP_AMOUNT = 1_000_000
def send_nexon_tx(ixs, signer, rpc_client): # Create transfer instruction tip_ix = transfer(TransferParams( from_pubkey=signer.public_key, to_pubkey=NEXON_TIP, lamports=MIN_TIP_AMOUNT )) ixs.append(tip_ix)
# Get the latest blockhash
blockhash = rpc_client.get_recent_blockhash()['result']['value']['blockhash']

# Create and sign transaction
tx = Transaction().add(*ixs)
tx.recent_blockhash = blockhash
tx.sign(signer)

# Send transaction
rpc_client.send_transaction(tx, signer)
```
## CURL
Please specify base64 encoding, Solana recognizes base58 as default. If you do not specify, you might get malformed transaction error
```
curl <url> -X POST -H "Content-Type: application/json" -d '                                   
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "sendTransaction",
    "params": [      "<base64_tx_data>",
      {                                                                                
        "encoding": "base64",
      }
    ]
  }
```
# Tip Stream
#### Stream Nozomi Tip Floors by Percentile
## Rest Endpoint
```
curl https://api.nexonlabs.net/tip_floor
```
## Websocket
```
wscat -c wss://api.nexonlabs.net/tip_stream
```
## Schema
```
[
  {
    "time": "string (ISO 8601 timestamp)",
    "landed_tips_25th_percentile": "number",
    "landed_tips_50th_percentile": "number",
    "landed_tips_75th_percentile": "number",
    "landed_tips_95th_percentile": "number",
    "landed_tips_99th_percentile": "number"
  }
]
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
