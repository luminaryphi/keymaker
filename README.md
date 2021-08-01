Keymaker is a Secret Network contract that produces trustless ECDH keypairs by crowdsourcing entropy.

- Users can donate entropy to the contract using gather_entropy(), which takes a string between 1 and 10 characters. This can be anything, even a random keyboard mashing

- Users can generate a keypair with generate_key(), which also takes a 10 or less random string as it calls the gather_entropy() function. The output will be two [u8; 32] arrays.


Why would you want to use this?

Random number generation is difficult to do in any blockchains with public data, and even in the Secret Chain which is private, it can be difficult to ensure no one person, or collaborating collective is able to remember their key or put their inputs together to recreate the key used for RNG. 
Keymaker solves this by crowdsourcing the entropy pool, enabling anyone to add entropy privately, changing the RNG seed with every interaction.