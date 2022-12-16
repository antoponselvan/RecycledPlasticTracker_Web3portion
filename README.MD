# Solana Recycled Plastic Tracker - Mission to eliminate plastic wastage
Recycling plastic can happen at different stages and it costs additional $ to do the same. This project aims to create a working prototype that enables creation of tracking and certification system using block-chain that makes it feasible to target incentives to plastic recycling at scale 

## 💡Introduction & Motivation
For a feasible recycled plastic recycling tracker following features are needed:
- Easily trusted by various parties involved in the manufacturing value chain
- Minimal Manual intervention for certification 
- Easy to spot and persecute falsifiers

Block-chain's following features makes the above happen:
- DeCentralized nature & immutability brings trust
- Enables to put ownership on manufactuers rather than certifiers
- Makes it easy to hold people accountable for any false information provided
- 
[Link to Web2 repo](https://github.com/antoponselvan/RecycledPlasticTracker_Web2portion.git)


## 🧑‍💼User Story / Experience
There are two types of users in this case:
- Purchaser 
- Manufacturer

Purchaser can key in product details (manufacturer's public key & product serial #) to trace the transformation stages and the eventual recycled plastic content at each stage. (S)He can also see the critical information stored in a normal DB and the one on blockchain. All auxillary info will be only stored in a normal DB and block-chain will only store hash of the information that can be used for verification

The manufacturer can enter the details of the product made (recycled plastic % and etc) and also link to the entity before and after him/her in the manufacturing value chain. The relevant information is stored in a normal DB and the blockchain. The manufacturer has features to be able to see all the product entries made by him in the past as well. 


## 📺 User Interface
The purchaser will be able to do following:
- Using unique credtials of  product, trace its origin to source of recycling and see the change in recycling content over different transformation stages
- Pick any individual product to see its details as stored in a Web-2 DB and confirm if the key details of this product can be verified in the Web-3 DB as well
![User Interface](/assets/UserStoryPurchaser.gif)

The purchaser will be able to do following:
- Login and Register himself to a web-2 DB
- Register a new product to web-2 DB and also store the critical information in Web-3 using his wallet
- Look back on the previous products he has registerd
![User Interface](/assets/UserStoryManufacturer.gif)


## 🏗️ Program Architecture
Front End program (files and state variable) design looks as follows:
![Front End Design](/assets/RePlasticTracker_FrontEnd.jpg)


Web2 BackEnd program (files and state variable) design looks as follows:
![Front End Design](/assets/RePlasticTracker_BackEnd_Web2.jpg)


Web3 BackEnd program (files and state variable) design looks as follows:
![Front End Design](/assets/RePlasticTracker_BackEnd_Web3.jpg)



## Key Challenges and achievements
Learning various concepts of block-chain and programming was the most challenging part. This project enabled me to learn a lot of new things:
RUST, Anchor framework, Multiple JS libraries involved in taking to SOLANA chain. Also, got a much better understanding of how block chain works.