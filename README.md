Keymaker is a Secret Network contract that produces trustless ECDH keypairs by crowdsourcing entropy.

- Users can donate entropy to the contract using gather_entropy(), which takes a String.

- Users can generate a keypair with get_key(), which also takes a String as well, and returns a pair of [u8; 32] as the public and private key.


Why would you want to use this?

Random number generation is difficult to do in any blockchains with public data, and even in the Secret Chain which is private, it can be difficult to ensure no one person, or collaborating collective is able to remember their key or put their inputs together to recreate the key used for RNG. 
Keymaker solves this by crowdsourcing the entropy pool, enabling anyone to add entropy privately, changing the RNG seed with every interaction.

Commands:



- gather_entropy():
./secretcli tx compute execute $CONTRACT '{"entropy": {"entropy": "YOUR_ENTROPY_GOES_HERE"}}' --from YOUR_SECRET_ALIAS -y --gas 1000000 --gas-prices=1.0uscrt




- get_key():
./secretcli query compute query $CONTRACT '{"keypair": {"entropy": "YOUR_ENTROPY_GOES_HERE"}}'




Beyond keys, the numbers in this array pair can be used for RNG seeds.


If you want your contract to demonstrate that you don't know the key it was given, when taking a keypair you may want to call gather_entropy(), to change whatever seed was there before, then take your key with get_key(), and finally call gather_entropy() again to prevent yourself from going back in with the same msg used in get_key() to read the seed yourself, since get_key() does not save your entropy to the state.