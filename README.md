# Creating Near Voting Smart Contract Step by Step Tutorial! :star_struck:

Hi! I'm Wagih ElBedeawi, Junior in both Rust and Near but I'm trying to share what I know, I welcome any improvements :hugs:. 

# Creating a Rust Contract

We will be following this [Intro](https://docs.near.org/docs/develop/contracts/rust/intro)
## Prerequisites To complete this tutorial successfully, you'll need:

 1. Rust toolchain 
 	Open your terminal and run the below
	
```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
	 
```console
source $HOME/.cargo/env
```
	 
 ```console
rustup target add wasm32-unknown-unknown
```
 2. A NEAR account
		 As stated [Here](https://docs.near.org/docs/develop/contracts/rust/intro#creating-a-near-account) 
 3. NEAR command-line interface (near-cli)
	 make sure you have node 12 or above

	 ```node -v```

	 Then run 

	 ```npm  install -g near-cli```

## Create a new Rust project 

We will use the `cargo` command to create the project
`cargo new voting-app-tutorial`
`cd voting-app-tutorial`

this will create a src folder with a `main.rs` file and cargo.toml file
```
.  
├── Cargo.toml  
└── src  
  └── main.rs
```

delete the `main.rs` file
`rm src/main.rs`

create `lib.rs` file inside src folder
`cd src`
`touch  lib.rs`

![image](https://user-images.githubusercontent.com/1478503/143580147-d40b8093-6136-417c-bc25-cb7877c3fa90.png)


### Editing Cargo.toml
open Cargo.toml file in your favorite editor then changes the values inside to your preference:
```
[package]
name = "voting-contract"
version = "1.0.0"
authors = ["Wagih ElBedeawi"]
edition = "2021"
...
```

### Editing lib.rs
Now open the `lib.rs` file it should be empty.
We will start by adding the near_sdk imports
```rust
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
```

Then lets' define our Contract Struct, what do we need here actually let's think about it,
we need callers:

 1. **To be list candidates**, *So we need like an array of candidates*
 2. **To be able to vote for a candidate**, *So we need that array of candidates to have like a vote count, so let's change it to a Map or an UnorderedMap or a HashMap*
 3. **To be limited to 1 vote**, *So we need a place to save the voters, so let's have another Map for Voters*
 4. **To get stats of the votes** *We can get that info from our candidates Map.*

At the end it seems we need to have 4 functions in our contract & 2 Maps in our struct

Let's by our Contract Struct
```rust
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub  struct  VotingContract {
// SETUP CONTRACT STATE
	candidates : UnorderedMap<String, i128>,
	voters : UnorderedMap<String, String>
}
```

So here we defined the Struct with the 2 maps we discussed earlier, the next step is to initialize those maps so let's implement the Default as below:
```rust
impl Default for VotingContract {
    fn default() -> Self {
        Self {
            candidates: UnorderedMap::new(b"a"),
            voters: UnorderedMap::new(b"b")
        }
    }
}
```
*each Map needs a unique storage name so init them with any unique strings.*

## The Contract implementation
First, let's define an empty contract
```rust
#[near_bindgen]
impl VotingContract {

}
```

Then let's add our first method add Candidate, it needs to take the name of the candidate as output and return a success message, so the function will need a parameter name of type String and it will return String.
```rust
#[near_bindgen]
impl VotingContract {
	pub fn add_candidate(&mut self, name: String) -> String {
        self.candidates.insert(&name, &0);
        return ["Added", &name,"!"].join(" ");
    }
}
```

### Add Candidate Function
we created the function `add_candidate`, it has a String parameter `name`, It gets the contract candidates map and inserts the candidate name with a default value of zero as no one voted for the new candidate yet.

What if you try to add an existing candidate? Let's add a guard.
```rust
pub fn add_candidate(&mut self, name: String) -> String {
        
        if !self.candidates.get(&name).is_none() {
           return "Candidate already exists".to_string(); 
        }

        self.candidates.insert(&name, &0);
        return ["Added", &name,"!"].join(" ");
    }
```
Here we added a simple check trying to get the candidate by using the `get` function if it returns something else than none then the candidate exists so we return `Candidate already exists`, this is our first method and we can try it, but first we need to build & deploy.

#### Build and Deploy
1- Go to your terminal again make sure you are inside the project folder and run the build command

```console
cargo build --target wasm32-unknown-unknown --release
```

it should show compiling for a while then at the end

```Finished release [optimized] target(s) in 2.79s```

![image](https://user-images.githubusercontent.com/1478503/143576652-31087371-e698-4625-80ff-dc652c6eefff.png)


2- Deploy:

 - You can either deploy on your main account as bedeawi.testnet or create a subaccount like voting.bedeawi.testnet, creating the subaccount is easy just run the below but change `YOUR_ACCOUNT` with your account.

 - ```console
near create-account voting.YOUR_ACCOUNT.testnet --masterAccount YOUR_ACCOUNT.testnet
```
	
![image](https://user-images.githubusercontent.com/1478503/143576911-a3e375c7-1991-42b7-a038-c157c97f7729.png)

 - now you can deploy your contract with

 ```console
near deploy --wasmFile target/wasm32-unknown-unknown/release/voting_contract.wasm --accountId $ID
```

 Change `$ID` with either `YOUR_ACCOUNT.testnet` or `voting.YOUR_ACCOUNT.testnet`
It should run successfully and print something like

```console
Done deploying to voting.YOUR_ACCOUNT.testnet
```

![image](https://user-images.githubusercontent.com/1478503/143576746-3e2ef220-018b-4f4c-95fd-2a4c0414a048.png)

3- let's test our contract by calling the `add_candidate` function:
run the below command in terminal

```console
near call voting.YOUR_ACCOUNT.testnet add_candidate '{"name": "Wagih"}' --accountId YOUR_ACCOUNT.testnet
```

This should result with this:

```console
Added Wagih !
```

![image](https://user-images.githubusercontent.com/1478503/143576985-280aac83-4a41-433f-a6e8-be10739deec3.png)

Yaaay it's working!!! :tada: :tada:
  
4- Let's test adding Wagih again to check our Guard
run the same command again

```console
near call voting.YOUR_ACCOUNT.testnet add_candidate '{"name": "Wagih"}' --accountId YOUR_ACCOUNT.testnet
```

This time it should show this:

```console
Candidate already exists
```

That's great :clap:, Now we can add candidates let's continue.


### List Candidate Function
Now let's try to list the existing candidate so that our callers can choose which to vote to, Go back to your editor inside the contract implementation let's add another function

```rust
	pub fn add_candidate(&mut self, name: String) -> String {
       ...
    }

    pub fn list_candidates(&mut self) -> Vec<String> {    
        return self.candidates.keys_as_vector().to_vec().clone();
    }
```
Here we just need the keys of the map as it's the names of our candidates so let's get only the keys by `keys_as_vector` then return that vector.

#### Build and Deploy again 
Then in your terminal run 

```console
near call voting.YOUR_ACCOUNT.testnet list_candidate '' --accountId YOUR_ACCOUNT.testnet`
```

It should return to you an array of names for the candidates, like below

```console
[ 'Wagih' ]
```

![image](https://user-images.githubusercontent.com/1478503/143577129-825776d3-833e-4411-8db1-4d506c8feef0.png)


:two: done :two: to go :wink:

### Vote Function
Now we can add and list candidates let' try to implement the voting function, back to your contract add the following function:
```rust
	pub fn list_candidates(&mut self) -> Vec<String> {    
		...
	}
	
	pub fn add_vote(&mut self, name: String) -> String {
        
        if self.candidates.get(&name).is_none() {
            return "Candidate doesn't exist".to_string();
        }

        if !self.voters.get(&env::signer_account_id()).is_none() {
            return "You have already voted".to_string();
        }

        self.voters.insert(&env::signer_account_id() ,&name);

        let mut votes_count = self.candidates.get(&name).unwrap();
        votes_count += 1;
        self.candidates.insert(&name, &votes_count);
        return ["Voted For", &name, "Total of", &votes_count.to_string() ,"!"].join(" ");
    }
```

Here I added some Guards first check if the candidate exists or not, the second check if the voter exists or not to prevent multiple votes from the same voter.

Also as you can see if you passed all the guards then we will add you to our voters Map, Then retrieve the Candidate Votes count increment it by 1 then reinsert it back to the candidates Map.

Wow :star_struck: That's amazing let's try it.
#### Build and Deploy again 
Then in your terminal call the vote function like below:
```console
near call voting.YOUR_ACCOUNT.testnet add_vote '{"name": "Wagih"}' --accountId YOUR_ACCOUNT.testnet
```

it should print:

```console
'Voted For Wagih Total of 1!'
```

![image](https://user-images.githubusercontent.com/1478503/143577205-88f640c0-df07-45a3-a129-8b8f895faaf7.png)

Try again the same command and you should get 
```console
'You have already voted'
```

![image](https://user-images.githubusercontent.com/1478503/143577264-bc3ee42b-fb55-4951-9104-248b192d7512.png)


Try again the same command and with an un-existing candidate 
```console
near call voting.YOUR_ACCOUNT.testnet add_vote '{"name": "NEAR"}' --accountId YOUR_ACCOUNT.testnet
```
```
"Candidate doesn't exist"
```console
Great :clap:, Let's go to our last function :sweat_smile:	

### Get Stats Function
Now that we have everything in place and people can vote, we need to see the results, Let's try to implement the Get Stats function, Back to your editor add the below function

```rust
	pub fn add_vote(&mut self, name: String) -> String {
		...
	}
	
	pub  fn  get_stats(&mut  self) -> Vec<(String, i128)> {
		return  self.candidates.iter().map(|x|x).collect();
	}
```

it's pretty easy actually we just need to return our candidate Map.
#### Build and Deploy again final time I Promise :D

Let's try it Call the get_stats function
```console
near call voting.YOUR_ACCOUNT.testnet get_stats '' --accountId YOUR_ACCOUNT.testnet
```
the result will be 
```console
[ [ 'Wagih', 1 ] ]
```
![image](https://user-images.githubusercontent.com/1478503/143577322-26615f68-54e7-4b27-be8f-a0cf7b7b3167.png)


if you added another candidate with `near call voting.YOUR_ACCOUNT.testnet add_candidate '{"name": "Bedeawi"}' --accountId YOUR_ACCOUNT.testnet`

Call the get stats again:
```console
near call voting.YOUR_ACCOUNT.testnet get_stats '' --accountId YOUR_ACCOUNT.testnet
```
the result will be 
```console
[ [ 'Wagih', 1 ], [ 'Bedeawi', 0 ] ]
```

I hope it's clear :sweat_smile:	 and it Helps someone :relaxed:	, the lib.rs file is in the repo and surely I welcome any improvements.

