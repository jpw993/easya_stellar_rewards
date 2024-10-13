# ShinyPoints
The Shiny Points dApp puts loyalty programs all in one place, making it seamless for customers to shop and automatically get loyalty points.

## Project Description

The Problem:
The average person has 16 loyalty program subscriptions (US based stat from the Bond Loyalty Report 2022), and most of them are for large businesses. Small businesses have a hard time setting up loyalty programs because it is expensive and time consuming.

Our Solution:
Shiny Points is a dApp that puts all the loyalty programs in one place. We have it such that customers using our dApp will automatically get rewards when shopping at participating businesses. We also make it easy for small businesses to set up their own loyalty program with us at a fraction of a cost that it will take to set it up individually.

How we solve it using Stellar:
Shiny Points is a dApp on Stellar that acts as a middle layer between a customer and a shop owner. Instead of a customer making a direct payment to a shop owner, the customer instead interacts with the Soroban smart contract to make a item purchase. Our smart contract then automatically calculates the amount of reward points, sends it to the customer's wallet, and transfers the purchase funds to the shop's wallet, all within a single atomic transaction.

Businesses can set up with Shiny Points by deploying our builder contract that interacts with our main contract, so each business with Shiny Points has their own contract to control the rewards they want to distribute. We will make it as easy as filling out a google form for the business to deploy a contract as it just requires them to fill out some short details (like the rewards they want to give out) and can handle the Stellar wallet creation/integration easily if the shop doesn't have an existing one.

Future Roadmap:
With more time, we would want to integrate Stripe with our dApp for a seamless fiat use case so that non-crypto native customers can easily use our services, accessing a greater population.


## Technical Description

Shiny Points is a Stellar Soroban dApp written in Rust. Our dApp acts as a middle layer between customers and businesses. The smart contract execution steps are as follows:
1. When a customer makes a purchase, they call the 'make_purchase' function is called
2. The number of rewards points is calculated based on the purchase amount based on the rewards ratio using the 'set_loyalty_reward_ratio' function. Currently our formula is: Floor(Amount/Spend_Increment) x Rewards_Multiplier. For example, for a purchase of 200XLM, a 30XLM Spend_Increment, and a 5x Rewards_Multiplier, the reward points returned is 30 reward points (200//30 = 6, 6x5 = 30). Reward points are represented by the SEP-41 standard token framework, meaning it will be supported by any Stellar wallet.
3. Within the transaction, the payment is transferred to the shop owner and the reward points are transferred to the customer atomically.

For a customer to get set up with us:
1. Just get a Stellar Wallet and put in some funds, and download Shiny Points. It's as easy as that!

For a business to get set up with us:
1. Initialise new 'ShopReward' contract on Stellar blockchain via our onboarding web page
2. Set the ratio of rewards points per amount of XLM spent in store (using the contract function  'set_loyalty_reward_ratio')
3. Generate QR code for customers to use to purchases (via our onboarding webpage)

For future development, we need Stellar because of it's integration with current Web2 payment services like Stripe. We believe that aspect of Stellar will help us gain seamless fiat integration, making our services accessible to even more customers and businesses.


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