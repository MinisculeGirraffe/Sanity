# Sanity

A vanity ED25519 SSH Key Generator



The last 43 characters of an ED25519 SSH key are Base64 encode, meaning they can contain arbitrary text. 

Each Base64 character represents 6 bits. To brute force an arbitrary 6 character string, we would have a 2^36 address space to search through. Or in other terms a 1 in 68,719,476,736 chance of generating a key that matched our desired text.

My M1 MacBook can generate about 380,000 keys per second. Meaning to find one key it would take 181,343 seconds or 50 hours. 

This search space also scales linearly with compute.
Below are the speeds calculating 

Max Threads

|CPU| Speed|
| --- | -- | 
| Apple M1 (8 threads) | 380,000 keys/s |
| Xeon E5-2683 v4 (32 threads) | 400,000 keys/s |
| c7.16xlarge (64 threads) | 960,000 keys/s |


Single Threaded
|CPU| Speed|
| --- | -- | 
| Xeon E5-2683 | 42,500 keys/s |
|c7.16xlarge | 52,916 keys/s |
| Apple M1 * | 79,200 keys/s |

\* The M1 has 4 Performance cores, and 4 Efficiency cores, which is why it doesn't scale linearly when utilizing all CPU cores. 

Time needed to brute force arbitrary characters at various lengths at 380k/keys/s

| Base64 Chars | Address Space | Time Needed |
| ------------ | ------------- | ----------- |
5 chars | 2^30 | 47 Minutes
6 chars | 2^36 | 2 Days
7 chars | 2^42 | 4.5 Months



