# Introduction

While learning about solana development and it's components, I discovered that there are very few detailed resources particularly when it comes to interecting with Solana programs with a Rust client.  My goal with this repo is to provide a reference for myself, but also hopefully help future developers who are trying to break into the solana ecosystem.

This repo focuses heavily on Rust. The solana program development which is written in Rust is based on the solana day3 program created by the Solana Bootcamp [link](https://www.youtube.com/watch?v=EwaKX5YoIjk&list=PLYvTLhNw9VYsndX_sdz-LdZ4_v5e7QAn4&index=3&pp=iAQB). My reason for creating the test in rust is solely on the purpose of expanding my rust knownledge and a personally preference to rust over typescript.

I mostly post solana security related contest on my twitter account [link](https://twitter.com/SkinneeOmeje) so you can definitely check it out if you also wish to know more about the securities in the blockchain.

# Prerequisites

You need to have install Rust, Solana, Anchor. How to install those won't be available in this repo as there plenty excellent resources out there to cover that. I won't also focus on how Rust works(except in situation I deem fit to expand on). This repo is intended for developers that have a decent understanding on rust and solana development.

# Contents

1. src
    The src folder contains the lib.rs file which basically contain the program logic and all the accounts verification for this program all written in rust and anchor. For detailed explaination on this, I advice you check out the day 3 course on solana foundation toturial.

2. tests
    The tests folder contains the staking_vault.rs file which consist of 
    # Tests in Rust for staking_vault
    `programs/anchor-counter/tests/counter_test.rs`
    Run the tests with `cargo test-sbf`

    This program was pathched from old article, the crate source, MarginFi Github, lighthouse Github and many more links which will be available below.

    The 2 key structs are the 'ProgramTest' and the 'PogramTestContext', which is basically the state of our local test blockchain, for each test. This is super useful and my expectation for the test.

    I wrote a helper stuct called SetUpTest and implemented the new function to return a 'validator: ProgramTest, and the 'vault_account: Pubkey' which is a pda for each test but won't. 

    ## impl SetUpTest
    One of the first thing that begin each test is the process of kicking on the local validator in the test environment. I release that a 'programTest' struct uses the name of your program and the program_id. You can pass 'None' as the built-in function and it will automatically pattern match to find the entrypoint. 

    Line 253-254 create our validator.

    Line 256 finds the PDA we created in our contract using the "vault" seed and our program address.  

    ## test_intialize

    Let's see if we can get our Staking_vault account to initialize in a test environment. Solana ' Transactions' take a list of 'Instructions'. We will also need a 'recent_blockhash' and a 'payer' which is required to sign our transactions. This variables were created in each test.

    We created a new Keypair with 'Keypair::new()' assign it to mint_account which is been used in the create_mint function. The create mint function which interact with the spl_token porgram imported to mint some tokens. In order to do so we need a couple of accounts which include 'mint_account': 'Keypair' which will be use to hold the newly minted object, 'owner': 'Keypair' that has the authority to mint token by signing future mint transactions, 'token_program': 'Public' which is public key of the token program.

    in order to mint a token we first need to create a 'mint_account' that has enough space for the mint and is well funded to store the mint data. Then we initialize the mint

    We now need to create our initialize 'Instruction'. We can use the 'solana-program' crate to create an 'Instruction struct. Within the 'Instruction', we will use our types created in the 'anchor_counter::accounts' create to create the 'Intialize' struct, which is the accounts needed for the 'Initialize' function in our program. Anchor provided us with a super handy 'to_account_metas' function to make sure the types are correct.

    All 'Instruction' types require a data field as well, which is any additional parameters passed into the function. We have none but still have to provide it. Anchor provides us with a 'data()' function on using the 'Initialize' instruction here which 'staking::vault::instruction' crate for each of our instructions. We are using the 'Initialize' instruction here which anchor creates for us based on our program.

    Next, we create the `Transaction` to send to the blockchain via our client. We use the `solana_sdk::Transaction` struct for this. There are a few different methods on `Transaction` that allow you to create a new `Transaction` which you can explore in the source. I made use of `new_signed_with_payer`, which creates a new `Transaction` with the list of instructions,is signed by the specified signing Keypairs, and specifies the `Keypair` that is going to pay the transaction fee. Then we got our recent_blockhash which were created earlier ago from the validator as the same for banks_client which is used to send the transaction to the chain.

    Then this transaction sent to the blockchain get process through the  'process_transaction' method in the 'banks_client' object.

    ## test_stake

    Just like the 'test_initialize' function, we created a 'mint_account' through a 'create_mint' helper function and a 'user_token_account_key' through the 'create_ata' helper function  which will be use to create an associated_token_account' for the user interacting with this program. We minted 10 tokens through the 'mint_to' parameter in the 'create_mint' function to this user_token_account_key which is required inorder for the user to have sufficient token to stake in this function. 

    The 'stake_info_account_pda' was been derived through the 'find_program_address' found in the 'Pubkey' object. This pda is use to hold the state of the user stake_account details. This was derived from a 'seed', 'signer pk' and the 'program id'. This same method is also applicable inorder to find the 'stake_account_pda'.

    Next we proceed to get the assocaiated token address of the stake account pda through the 'get_associated_token_address' available to us through 'associated_token' method in the 'anchor_spl' library imported earlier. 

    Similar to the 'test_initialise' function, this program send the instruction with the necessary account to the node with the 'data' which consist of an amount this time. This instruction is been signed by the payer address and processed through the 'banks_client'.

    ## test_destake

    This test is very similar to the 'test_stake' with the only difference been this time we mint to the vault_account inorder to have sufficient token for the reward

3. Cargo.toml

    The Cargo.toml consist of all the neccessary depency needed to test this program. 

# Feedback

This is a learning repo and this project was donely solely to improve my understanding on rust and Solana at large. I undestand I'm not perfect so if you are an experience Solana developer, please feel free to correct my mistake by reaching out to me on Discord[username](skinneeomeje) or Twitter.

# Future

I might end up jumping on more project of different topics. For now I'm really glad I did this one.

# Resources Used
This is basically just a dump of reseaches I found useful while creating this. Although, some of these are few years old, I won't deny it was a source of help to me. One thing I found useful while creatiing this was the answer to questions on solana stackexchange.Also just reading the source for a lot of stuff will give you a general idea of how to do things.

- [Solana Bootcamp](https://www.youtube.com/playlist?list=PLYvTLhNw9VYsndX_sdz-LdZ4_v5e7QAn4)
- [solana stackexchange](https://solana.stackexchange.com)
- [76 Developers Discord](https://discord.gg/HrqDu9hZsS)
- [Lighthouse Github](https://github.com/Jac0xb/lighthouse)
- [Margin Fi Github](https://github.com/mrgnlabs/marginfi-v2/tree/main)
- [Solana Official Docs](https://docs.solana.com/introduction)
- [Solana Dev Course](https://www.soldev.app/course)
- [Solana Cookbook](https://solanacookbook.com/core-concepts/accounts.html#facts)
- [Mint tokens using Rust SDK](https://0xksure.medium.com/mint-tokens-on-solana-using-the-rust-sdk-3b05b07ca842)
- [testing program in rust](https://medium.com/@jacob_62353/testing-an-anchor-solana-program-in-rust-65144b0cc5ce)
- [Blogpost About Testing an Anchor Solana Program With Rust -- OLD](https://medium.com/@jacob_62353/testing-an-anchor-solana-program-in-rust-65144b0cc5ce)
- [Rust articles](medium.com/@SkinneeOmeje)
  
