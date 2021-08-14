# Intro
Keymaker is a Secret Network contract that produces trustless ECDH keypairs by crowdsourcing entropy.

- Users can donate entropy to the contract using gather_entropy(), which takes a String.

- Users can generate a keypair with get_key(), which also takes a String as well, and returns a pair of [u8; 32] as the public and private key.


Why would you want to use this?

Random number generation is difficult to do in any blockchains with public data, and even in the Secret Chain which is private, it can be difficult to ensure no one person, or collaborating collective is able to remember their key or put their inputs together to recreate the key used for RNG. 
Keymaker solves this by crowdsourcing the entropy pool, enabling anyone to add entropy privately, changing the RNG seed with every interaction.

**These keypairs are useful as RNG seeds themselves.** Call one into your contract and use it for any randomization purposes you may have

Holodeck-2 Testnet Address:
secret10kr8qrz6gte6c5jfza04yacst48tep25jmtxyl


# Commands:



- `gather_entropy():
./secretcli tx compute execute $CONTRACT '{"entropy": {"entropy": "YOUR_ENTROPY_GOES_HERE"}}' --from YOUR_SECRET_ALIAS -y --gas 1000000 --gas-prices=1.0uscrt`




- `get_key():
./secretcli query compute query $CONTRACT '{"keypair": {"entropy": "YOUR_ENTROPY_GOES_HERE"}}'`



# TO MAKE SURE YOUR KEY IS SECURE:
- Use a long input string 
- Call `gather_entropy()` after you get your keys to alter the entropy pool.
- The most secure, and most trustless method is to call `gather_entropy()` in Handle A, then have Handle A call Handle B, in which the you use `get_key()` and `gather_entropy()`. This will ensure the entropy pool is altered both before, and after you receive your keys.



