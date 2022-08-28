# Sanity

A vanity ED25519 SSH Key Bruteforcer



The lazy 43 characters of an ED25519 ssh key are Base64 encode, meaning they can contain arbitrary text. 

Each Base64 character represents 6 bits. To brute force an arbitrary 6 character string, we would have a 2^36 address space to search through. Or in other terms a 1 in 68,719,476,736 chance of generating a key that matched our desired text.

My M1 MacBook can generate about 380,000 keys per second. Meaning to find one key it would take 181,343 seconds or 50 hours. 

This search space also scales linearly with compute.

Time needed to brute force arbitrary characters at various lengths at 380k/keys/s

| Base64 Chars | Address Space | Time Needed |
| ------------ | ------------- | ----------- |
5 chars | 2^30 | 47 Minutes
6 chars | 2^36 | 2 Days
7 chars | 2^42 | 4.5 Months



