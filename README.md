# ParaDAO

para dão - to give in Portuguese
ParaDAO provides tools for communities with the same passion / shared interests to come together to organise amongst themselves to acheive a common goal.
The decentralised application that allows communities to create a DAO easily with out of the box options for managing:

- Governance process
- Roles
- Opiinion polling

## Special features

### Support by staking

In addition to direct donation , ParaDAO has a native currency that is minted according to a [bonding curve] where the rewards can be distributed to the staker or the DAO treasury.

[bonding curve]:

### Community types

ParaDAO initially has composable options that are designed to support 2 types of DAOs

#### 1. Individual support

Inspired by Patreon / Substack,
individuals who produces valuable contents may be supported by a community / fan base through a DAO.

In this case,
the DAO roles will only be the admin (content producer) and the supporters,
where the supporters can participate in some polls / suggestion and the admin will have full control over the DAO treasury.

#### 2. Interest Groups

Inspired by meetup.com,
a group of individuals may want to contribute to say a software code base, a meetup events etc.
Some members of the DAO should be rewarded for their contributions,
and some other members may just be users of a software or participants of an event.

In this case,
the DAO will have a selection of roles of which the users in each role may be able to vote / execute subset of actions only.
It is also likely in this case that the treasury is managed by proposals and votes.

### Prediction Market

It is a problem with the lack of participation in voting in DAO governance models.
ParaDAO leverages prediction markets to encourage participation in different forms.

For example,
early voters for a [binary prediction market] will have little certainty of the outcome and therefore if the vote automatically provide a position in a prediction market,
early voters will be rewarded more than the late voters, when the outcome is more predictable.

[binary prediction market]:

## Components

- A parachain node with Pallets specifically to allow for DAO creation
- A relay chain node to simulate staking of relay chain native tokens
- Frontend allow users to create DAOs easily, allow DAO members to do proposals, votes, etc
  - integration with Zeigeist sdk

---

## Hack

Requirements:

- Node
- Docker
- Rust
- jq
- curl

### 1. Build the collator for the parachain

```sh
# root dir
cargo build --release
```

### 2. Run the relay chain

```sh
docker-compose --file docker-compose-xc.yml up
```

### 3. Register and start parachain collator

_Note: Ensure that step 2 nodes are producing blocks_

```sh
./scripts/register-paradao.sh
```

### 4. Tear down

```sh
docker-compose --file docker-compose-xc.yml down -v && ./scripts/clear_all.sh
```
