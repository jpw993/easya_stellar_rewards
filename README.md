# ShinyPoints

## Project Description

ShinyPoints makes customer loyalty point collection easy with automated point transfer to customers for in-store purchases.
There is no longer any need for customers to download an app for every shop they visit, all points can be stored in their Stellar Wallet!
Now small businesses can have a digital loyalty programme (without hiring developers) by using our permission-less Sororban dApp!


## Technical Description

ShinyPoints is a Stellar Soroban dApp written in Rust. 
The ShopReward Soroban smart contract acts as a middle layer between a customer and a shop owner.
Instead of a customer making a direct payment to a shop owner, the customer instead interacts with the Soroban smart contract to make a item purchase.
When this 'make_purchase' function is called, the number of rewards points is calculated based on the purchase amount and is transferred to the customer in the same transaction.
All payments are immediately transferred to the shop owner (without the contract holding any funds).
The shop owner has the ability to update the rewards ratio using the 'set_loyalty_reward_ratio' function.

# Presentation (includes screenshots)

https://www.canva.com/design/DAGTX6cd_b0/KWO1qnVeOJ5LEDFgDa-ETQ/view


# User Demo
https://www.youtube.com/watch?v=ZAXk0aVcPWs


# Code Repo Walkthrough 
https://www.youtube.com/watch?v=Z2bUrpGl_n0


# Shop Payment Wallet Connect UI
https://a5321f7d.rewards-47h.pages.dev


# Smart Contract on TestNet
https://stellar.expert/explorer/testnet/contract/CB5XJWYAJ3VEAYMXOGBA26NKH4IJRZ5QCOJKICSPWEUM4MYNL3PLK5KJ