# Ethcon 2023 BUIDLersberg Fund
## Decentralized Governance & Asset Management Service across Multiple Chains

### Introduction
The BUIDLersberg Fund offers a chain-agnostic way to govern and manage assets across diverse chains, ensuring decentralized Liquidity Pool Fund Governance & Execution Service.

This innovative protocol empowers Liquidity Providers (LPs) with the ability to oversee their pools through DAO-style governance, all on private governance blockchains. We’re revolutionizing the LP ecosystem by embedding decentralized voting with consensus and execute the transaction calls to multiple blockchain and contracts at once at its core.

### The Challenge
Decentralized liquidity pools (LPs) are pivotal to decentralized exchanges, facilitating efficient token swaps. However, there exists a dilemma: How can LPs uphold the confidentiality of their internal decisions to prevent external manipulations while upholding the transparency and community-centered spirit of a DAO?

### Architectural Overview
![image](https://user-images.githubusercontent.com/41055141/265234056-9f33f349-561f-4f32-9a4b-bddb956edb8c.png)
The protocol helps LP decision confidentiality with decentralized governance. Key components include:

1. **Simperby Chains**: Every DAO/LP operates its own governance chain powered by [Simperby](https://github.com/postech-dao/simperby/blob/main/docs/ssss.md), our custom blockchain system that governs the DAO. These EVM-compatible chains form the core of our solution, ensuring decision confidentiality but providing transparency when appropriate.

2. **Light Clients & Relayers**: Tailored for Uniswap v2, ERC20, and ERC721 contracts. They bridge decisions from the Simperby chain to primary blockchains such as Ethereum, Polygon, Linear, dYdX and more (chain-agnostic!). Relayers relay LP DAO decisions, triggering light clients into action for executing transactions to its pool contracts.

3. **Governance Voting**: A mechanism that lets LP participants propose and vote on items, fostering collective decision-making.

4. **DEX Treasury Aggregation**: Once a consensus is achieved on a Simperby chain, the results are reflected on primary chains via the DEX treasury.

5. **Flexible Participation**: Open to all. Whether you aim to join, exit, or propose pool changes, the protocol accommodates.

6. **Commitment Proof Relayer**: Guarantees the accurate execution of every Simperby chain governance decision on main chains.

### Workflow
1. A Fund requires at least two participants to form.
2. An individual launches the Simperby network, then invites participants using an invite code.
3. Tokens are deposited by participants into designated pool contracts.
4. Any member can suggest an agenda for governance voting.
5. Actions, based on consensus, are carried out on the primary chains via our light clients and relayers.

### Supported Chains & DEX
1. Linea Testnet - ERC20Mock (Now available), ERC721Mock, Uniswap (Soon)
2. Polygon Testnet - ERC20Mock, ERC721Mock, Uniswap (Soon)
3. dYdX Testnet (further planned)

### Practical Use Case
Visualize a friend group pooling their resources to venture into various crypto projects. Instead of a singular decision-maker, they opt for a voting system, with each member's influence tied to their investment size.

**How it Functions**:
- **Fund Formation**: A minimum of two members is essential. They register, providing their cryptographic details.
- **Liquidity Strategy**: Agendas are proposed for asset deposit or withdrawal decisions from different DEXs on various blockchains.
- **Voting Process**: After setting an agenda, votes are cast based on each member's influence. Majority verdicts are executed.
- **Automated Execution**: Approved actions are automated via smart contracts.
- **Transparent Autonomy**: Every move, from fund distribution to voting, is both transparent and self-governed.

**Sample Scenario**:
Consider a situation where a group feels that the value of 'axlUSDC' will drop but 'APE' will rise. They propose to sell off all 'axlUSDC' and invest more in 'APE'. After voting, the majority agree. The protocol then facilitates this shift in investments across the designated exchanges.

However, participants who don’t commit their agreed-upon assets can face penalties, ensuring everyone stays true to their commitments.

This is a simplified way to understand the protocol we’re delving into. But in our case, the decisions aren’t just about which project to back, but also about which blockchain network and decentralized exchange to engage with.

Additionally, there are often private consortiums or single-entity events that require treasury operations. Our solution is invaluable in these situations:

It provides a lightweight, multi-chain solution for governance.
Ensures privacy and autonomy for each DAO or LP, giving them control over their operations without external interference.
Streamlines decision-making processes, making operations efficient and agile.

To illustrate, consider any events where acquiring opUSDC posed challenges. However, during such events, organizers can store funds in the treasury. Upon receiving cash, individual transactions can be crafted. Once organizers validate these via voting, the assets can be disseminated. Such a mechanism is not only feasible but also efficient.

### How to Run?
#### Local Testnet
1. Run own testnet
2. Deploy sample contract in `simperby-evm/contract/scripts`
* Sample ERC20
* Sample ERC721
* Sample UniswapV2 is supported
3. Configure environment variables like the following
```
GITHUB_TOKEN='';
GIT_LOCAL_PATH=/Users/sigridjineth/Documents/simperby-miscs
TEST_PRIVATE_KEY='';
```
4. Run `main.rs` to wake up the relayer & light client