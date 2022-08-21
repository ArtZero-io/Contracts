Code Hashes and Contract Addresses:

# Proxy Contract Hash Code
0x949a226723dfd260e7be6f192d0223de70a92667c8e200abe08773ff7982942b

# Profile Contract
0xc2b0257c8470112abcdafc29e5fa9dafaff4281e5df9667692b1dcd4710418ad
5FcxApkPqTq3NKjcN9FuNqZvVuKJqrpahNHvTWJwsusJEL3m

# PMP Contract
0xc32e1c66bb726d48673a00922bef71b0be607e9c4270fe8d654c14b7bfdf9575
5FDE1rdmDpG45pw94RX8Tc4YvGctdmhXaWhuftG3uiTPkLkb

# ArtZero NFT Staking Contract (PROXY)
0xc020dd75fc0467e9aa946931ec630bcb0c4fe521244189d6bba8767141616ad0
5FmTW2PbLH6ffRtozDd27XDkh6qdHJBWuHk1Ez8NPKEQgPpw

# ArtZero NFT Standard Contract
0x640f6303817175002ec922365128001b92892883c69487927505094741807275
5EY3paqxYZxfCEApdppC8iBgtA4UNCNqTJi56YX4KVLpqeXi

# Collection Manager (PROXY)
0x341efe8208643c86d2d9cd844f05d49961486fe9a7e7905c37d79d5edcb1bd96
5DABaBNe2j5ZViADmQHXwgu5HvDGFApRGyvb1UHPXBpJvXWq

# Marketplace Contract (PROXY)
0x2f2db229e5c7897a14be8f5ad7b8cdf132c2afac52ced31ed1d23b197a0c6a7b
5E8HsjBfoNfLHKTCv3mbZ5XrkRoSVTTrLQxBFhzEjxC4Gtsn

# LaunchPad Manager (PROXY)
0x8e6be975ea7b768336ed10a616bb68112877541a43a0eec1eb59475c5fd7ece9
5EGDJc7yApKutPXhRnjR7FcE2k1tevPxuEqXth5uoh77zyrY

# LaunchPad Standard Contract
0x861a860f2399a78f3e9f6136049b274474f4c344e8ce7496f73c93545a6beaed
5HQAuCF233M5HnoDKiDoggFsn9GL44kcGvQ9pHKwT9XHeSgw

Deploy Steps:
1. Deploy Profile Contract
2. Deploy NFT Standard Contract
3. Deploy Collection Contract via Proxy
4. Deploy LaunchPad Standard Contract
5. LaunchPad Manager via Proxy with Code Hash of LaunchPad Standard
Add PMP Project -> create Collection -> Update base URI -> Update admin address
Pinana Smartnet: ipfs://QmXtnr9aEJVywiLs1keZdyiKbQwignZT3FhwKYivF15oZp/
PMP Address : 5Co9CQPprxfUKuJeDQZfWy5SubCVHnBSF6psjMUH35PYLrYx
6. Deploy Staking Contract via Proxy
With Address of PMP Project
7. Deploy Marketplace Contract via Proxy

Deploy Cli:
cargo +nightly contract build

Optimize Cli:
rustup component add rustfmt --toolchain nightly
cargo +nightly fmt
