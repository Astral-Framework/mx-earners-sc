<sub>*This file has been auto-generated using the [abi-markdowner](https://github.com/0xk0stas/abi-markdowner).*</sub>

# Smart Contract: Fees

<details>
<summary>Documentation</summary>

A simple Smart Contract that distributes a payment (fee) to multiple addresses.

- The payment can be in EGLD or in any single ESDT token.

- The payment is distributed to multiple addresses (called Earners) based on their percentage.

- The total percentage must be exactly 100%.

- Any leftover amount is sent back to the owner.
</details>

<details>
<summary>Build info</summary>

- **Rustc Version**: 1.80.1
- **Commit Hash**: 3f5fd8dd41153bc5fdca9427e9e05be2c767ba23
- **Commit Date**: 2024-08-06
- **Channel**: Stable

- **Framework**: multiversx-sc
- **Version**: 0.53.0
</details>

## Table of Contents

- [Endpoints](#endpoints)
- [Views](#views)
- [Events](#events)

## Endpoints

### Deploy

<details>
<summary>init</summary>


</details>

### Upgrade

<details>
<summary>upgrade</summary>


</details>

### Owner Only

<details>
<summary>setEarner</summary>

Sets a new earner with a name and a percentage.
#### Inputs:
| Name | Type |
| - | - |
| address | Address |
| name | bytes |
| percentage | u64 |


</details>

<details>
<summary>removeEarner</summary>

Removes an earner using the address.
#### Inputs:
| Name | Type |
| - | - |
| address | Address |


</details>

### Other

<details>
<summary>sendFees</summary>

Sends the fees to the earners based on their percentage.
#### Note: This endpoint is payable by any token.


</details>

## Views

<details>
<summary>getEarnersInfo</summary>

Returns the list of earners with their info (address, name, percentage).
#### Outputs:
| Type | MultiValue |
| - | - |
| multi&lt;Address,bytes,u64 | ✔ |


</details>

<details>
<summary>earners</summary>

#### Outputs:
| Type | MultiValue |
| - | - |
| Address | ✔ |


</details>

<details>
<summary>earnerName</summary>

#### Inputs:
| Name | Type |
| - | - |
| address | Address |

#### Outputs:
| Type |
| - |
| bytes |


</details>

<details>
<summary>earnerPercentage</summary>

#### Inputs:
| Name | Type |
| - | - |
| address | Address |

#### Outputs:
| Type |
| - |
| u64 |


</details>

<details>
<summary>earnersTotalPercentage</summary>

#### Outputs:
| Type |
| - |
| u64 |


</details>

## Events

<details>
<summary>set_earner</summary>

#### Inputs:
| Name | Type |
| - | - |
| address | Address |
| name | bytes |
| percentage | u64 |

</details>

<details>
<summary>remove_earner</summary>

#### Inputs:
| Name | Type |
| - | - |
| address | Address |

</details>

