# Welcome to Create Voting Contract!

Hi! I'm Wagih ElBedeawi, Junior in both Rust and Near but I'm trying to share what I know, I welcome any improvements. 

# Creating a Rust Contract

We will be following this [Intro](https://docs.near.org/docs/develop/contracts/rust/intro)
## Prerequisites to complete this tutorial successfully, you'll need:

 1. Rust toolchain 
 	Open your terminal and Run the below
	
	 ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
	 
	 ```source $HOME/.cargo/env```
	 
	 ```rustup target add wasm32-unknown-unknown```
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
```
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
```

Then lets' define our Contract Struct, what do we need here actually let's think about it,
we need callers:

 1. **To be list candidates**, *So we need like an array of candidates*
 2. **To be able to vote for a candidate**, *So we need that array of candidates to have like a vote count, so lets change it to a Map or an UnorderedMap or a HashMap*
 3. **To be limited to 1 vote**, *So we need a place to save the voters, so lets have another Map for Voters*
 4. **To get stats of the votes** *We can get that info from our candidates Map.*

At the end it seems we need to have 4 functions in our contract & 2 Maps in our struct

Let's by our Contract Struct
```
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub  struct  VotingContract {
// SETUP CONTRACT STATE
	candidates : UnorderedMap<String, i128>,
	voters : UnorderedMap<String, String>
}
```

So here we defined the Struct with the 2 maps we discussed earlier, the next step is to initialize those maps so let's implement the Default as below:
```
impl Default for VotingContract {
    fn default() -> Self {
        Self {
            candidates: UnorderedMap::new(b"a"),
            voters: UnorderedMap::new(b"b")
        }
    }
}
```
*each Map need a unique storage name so init them with any unique strings.*

## The Contract implementation
First let's define an empty contract
```
#[near_bindgen]
impl VotingContract {

}
```

Then let's add our first method add Candidate, it needs to take the name of the candidate as output and return a success message, so the function will need a parameter name of type String and it will return String.
```
#[near_bindgen]
impl VotingContract {
	pub fn add_candidate(&mut self, name: String) -> String {
        self.candidates.insert(&name, &0);
        return ["Added", &name,"!"].join(" ");
    }
}
```

### Add Candidate Function
we created the function `add_candidate`, it has a String parameter `name`, It get the contract candidates map and inserts the candidate name with a default value of zero as no one voted for the new candidate yet.

What if you tries to add an existing candidate ? Let's add a guard.
```
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
```cargo build --target wasm32-unknown-unknown --release```
it should show compiling for a while then at the end
```Finished release [optimized] target(s) in 2.79s```
2- Deploy:

 - You can either deploy on your main account as bedeawi.testnet or create a subaccount like voting.bedeawi.testnet, creating the subaccount is easy just run the below but change `YOUR_ACCOUNT` with your account.

 - ```near create-account voting.YOUR_ACCOUNT.testnet --masterAccount YOUR_ACCOUNT.testnet```

 - now you can deploy your contract with

	 ```near deploy --wasmFile target/wasm32-unknown-unknown/release/voting_contract.wasm --accountId $ID```

	 Change `$ID` with either `YOUR_ACCOUNT.testnet` or `voting.YOUR_ACCOUNT.testnet`
	It should run successfully and print something like

	```Done deploying to voting.YOUR_ACCOUNT.testnet```

3- let's test our contract by calling the `add_candidate` function:
	run the below command in terminal

	```near call voting.YOUR_ACCOUNT.testnet add_candidate '{"name": "Wagih"}' --accountId YOUR_ACCOUNT.testnet```

	This should result with this:

	```Added Wagih !```

	Yaaay it's working!!!
  
4- Let's test adding Wagih again to check our Guard
	run the same command again
	```near call voting.YOUR_ACCOUNT.testnet add_candidate '{"name": "Wagih"}' --accountId YOUR_ACCOUNT.testnet```
	This time it should show this:
	```Candidate already exists```
	That's great now we can add candidates let's continue.


### List Candidate Function
Now let's try to list the existing candidate so that our callers can choose which to vote to, Go back to your editor inside the contract implementation let's add another function

```
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
```
near call voting.YOUR_ACCOUNT.testnet list_candidate '' --accountId YOUR_ACCOUNT.testnet`
```
It should return to you an array of names for the candidates, like below
```[ 'Wagih' ]```

2 done 2 to go ;)

### Vote Function
Now we can add and list candidates let' try to implement the voting function, back to your contract add the following function:
```
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

Here I added some Guards first check if the candidate exists or no, secondly check if the voter exists or no to prevent multiple votes from same voter.

Also as you can see if you passed all the guards then we will add you to our voters Map, Then retrieve the Candidate Votes count increment it by 1 then reinsert it back to the candidates Map.

Wow That's amazing let's try it.
#### Build and Deploy again 
Then in your terminal call the vote function like below:
```
near call voting.YOUR_ACCOUNT.testnet add_vote '{"name": "Wagih"}' --accountId YOUR_ACCOUNT.testnet
```
it should print:
```
'Voted For Wagih Total of 1!'
```
Try again the same command and you should get 
```
'You have already voted'
```

Try again the same command and with un-existing candidate 
```
near call voting.YOUR_ACCOUNT.testnet add_vote '{"name": "NEAR"}' --accountId YOUR_ACCOUNT.testnet
```
```
"Candidate doesn't exist"
```
Great, Let's go to our last function

### Get Stats Function
Now that we have everything in place and people can vote, we need to see the results, Let's try to implement the Get Stats function, Back to your editor add the below function

```
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
```
near call voting.YOUR_ACCOUNT.testnet get_stats '' --accountId YOUR_ACCOUNT.testnet
```
the result will be 
```
[ [ 'Wagih', 1 ] ]
```
if you added another candidate with `near call voting.YOUR_ACCOUNT.testnet add_candidate '{"name": "Bedeawi"}' --accountId YOUR_ACCOUNT.testnet`

Call the get stats again:
```
near call voting.YOUR_ACCOUNT.testnet get_stats '' --accountId YOUR_ACCOUNT.testnet
```
the result will be 
```
[ [ 'Wagih', 1 ], [ 'Bedeawi', 0 ] ]
```

I Hope it's clear and it Helps someone, the full file in in the repo and surely I welcome any improvements.

