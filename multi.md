## Goal:

To integrate the multiassets pallet into chocolate with:

1. CHOC as spending token for settling fees, etc
2. Any generic asset is available for reserve and the like in other pallets (drop-in replacement.). 

> Ideally, there should already be a way of integrating this pallet with specific ones that have actual value e.g DOT,KSM

## Needs

* Some partial migration of the chocolate-node (specifically, native token name: CHOC)

## Testing: 

(As used in chocolate pallet) Reserve, unreserve balances. Fail TX if not enough native token.

* A simple extrinsic that reserves a generic asset in the template pallet, and test that it is reserved as expected.
* A simple extrinsic that unreserves the same generic asset in the template pallet, and test that it is unreserved as expected.
* Both extrinsics will be paid in the native token, test that it fails if the user doesn't have sufficient balance.

