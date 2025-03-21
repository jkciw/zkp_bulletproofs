# Bitshala developer's summit
## What are bulletproofs ?
<p align="justify"> Bulletproofs are a type of non-interactive zero-knowledge proof protocol that provides short proofs without requiring a trusted setup. By employing a non-interactive proof system that requires no trusted setup, Bulletproofs allow users to prove that a committed value lies within a specific range without disclosing the actual value. At their core, Bulletproofs use a zero-knowledge weighted inner product argument (zk-WIP) to construct efficient range proofs and arithmetic circuit proofs. This approach allows for significant reduction in proof size, with range proofs shrinking from over 10kB to less than 1kB. Currently, Bulletproofs are implemented in privacy-focused cryptocurrencies like Monero, where they power confidential transactions and reduce blockchain bloat.</p>
<p align="justify"> The prospects for Bulletproofs in Bitcoin are promising, as they could facilitate more scalable and private transactions, enhance fungibility, and support innovative solutions like proofs of solvency for exchanges, thereby addressing ongoing privacy concerns in the Bitcoin ecosystem. As the demand for privacy-enhancing technologies grows, Bulletproofs could play a pivotal role in shaping the future of Bitcoin and other cryptocurrencies. </p>

## Timeline
| Topic | Duration (min) | Format | Host |
|-------|----------------|--------|------|
|Introduction to zero-knowledge proofs| 30 | Presentation | Bala|
|Demonstration of idea of zero-knwoledge| 15 | Demonstration | Delcin |
|Mathematical Pre-requisites|30 | Presentation |Beulah|
|Creating and verifying zkp-bulletproof | 30 | Workshop | Bala, Beulah & Delcin |
|Discussions | 15 | Discussion | Bala, Beulah & Delcin |

## About the workshop
<p align="justify"> We will understand a typical workflow involved in creating a bulletproof and verifying it. Rust is the prefered language as it has a production grade <a href="https://docs.rs/bulletproofs/5.0.0/bulletproofs/index.html">crate</a> implementing the bulletproof system. We will perform the following steps:</p>

1. Setup the generators (Pedersen & Bulletproof)
2. Create Pedersen commitment to the secret value 
3. Create Merlin transcripts that manages the Fiat-Shamir transformation to make the proof non-interactive
4. Generate random blinding factor 
5. Create succint proof that the secret value lies within range [0, 2^proof_bits]
6. Verify proof agains the commitment 

## Additional reading raterials
1. <a href = "https://eprint.iacr.org/2017/1066.pdf">Bulletproofs: Short Proofs for Confidential Transactions and More</a>
2. <a href = "https://tlu.tarilabs.com/cryptography/bulletproofs-and-mimblewimble">Bulletproofs and Mimblewimble</a>
3. <a href = "https://blog.pantherprotocol.io/bulletproofs-in-crypto-an-introduction-to-a-non-interactive-zk-proof/"> Bulletproofs in crypto </a>
4. <a href = "https://academy.bit2me.com/en/que-son-las-bulletproofs/"> What are bulletproofs </a>


