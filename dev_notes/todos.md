Demo build:

Collab DAO type

-   run docker
-   _script: yarn deploy_
-   Transfer funds to MemberA and MemberB on polkadotjs ui (for paradao - 9944 and zeigeist - 8844 )
-   \*MemberB create a collabDAO
-   _script: MemberB and 3 members join dao_
-   \*MemberA sends DOT from relay to paradao chain
-   \*MemberA joins DAO by paying membership fees
-   \*MemberB creates a prediction market for decision making, 3 OAT - Strategy1, Strategy2, None, see https://paradaochain.github.io/docs/prediction_market.html#voting
    (MemberB is the oracle for the PM)
-   _script: MemberB create 2 proposals with PM hash / link?_
-   _script: 1 person vote no on proposal-strategy1_
-   _script: 1 person vote yes on proposal-strategy2_
-   _script: 1 person bought `Strategy1`, 1 bought `None`_
-   \*Show some data of the current PM
-   \*MemberA votes on proposal-strategy1 `yes` and buys 1 `Strategy1` OST

Smart contracts
Governance

-   x add did field to join (store in members)
-   x promote members proposal type
-   voting power table (membership type to weight)
-   make paradao a dao with fees

-   links to doc on frontend
