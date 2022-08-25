Code Hashes and Contract Addresses:

# Proxy Contract Hash Code
0xc34dfedd02e4985e08948d42551be3d6758ed5f0167004f0c4b7b344d6d42344

# Profile Contract
0x78ea710ed189e29ae3f2ca46672a7ede3fe14692882602191a48425888fe4253
5E2zk4B1p4ohQmwG3uFw16yoZbYWE3ZRnb6XgPLNvi5zhG7U

# ArtZero NFT Contract / Mint (PROXY)
0x1958af8a820b5b6689a84fb4974a8a1bb7457eb7933933e51dc82a45fb198b18
5EuozqMa5FEVt3FGFRRddUhZCpH7N5VDNvipNsTcFFRSN7Lp

# ArtZero NFT Staking Contract (PROXY)
0x0e7ff4fb9b7c1ef631975277c64095571e2437562c6035d803ab4c7862a320c4
5Gh6JkVixyzpU5Ne6THQKyMZwwugoAwnzmwNyDqqJD6SzaoB

# ArtZero NFT Standard Contract
0x7571bc09dcd9e818a17a1b35f1a9c92b5e83ada6f6d08be93d28307508bc1065

# Collection Manager (PROXY)
0x2b8390edc6a55dd2f03172f112eb4e755b90ea6cc8f6eeb57b67aae78af900c5
5Dy7kjw3XUqpZyXVasWBddL6jpq6sgZCwMHLvfy42Q8qMQXg

# Marketplace Contract (PROXY)
0x9a0b60053b96d9dd31ce241e98e1014586d8e87c7e6cf899a43593011026ad2b
5C8xoEUyK2CcuAU3DZryMdejcTAtBFrBZzx4wtqrkH9os796

# LaunchPad Manager (PROXY)
0xcd70fd768ec25fc7e191016c42aa0b01ee7379846c4f867e1217e7fa30841bb5
5CiMZCT4APnW1w5HVqAa4AbJGHU3afjDaEhhfNwCJBzQQY61

# LaunchPad Standard Contract
0xc4ee5b610222ccdf50b02a189815f6b8af126481f6aec8faca2a1fbd7ffcda3b

Deploy Steps:
1. Deploy Profile Contract
2. Deploy NFT Standard Contract
3. Deploy Collection Contract via Proxy
4. Deploy LaunchPad Standard Contract
5. LaunchPad Manager via Proxy with Code Hash of LaunchPad Standard
Add PMP Project -> create Collection -> Update base URI -> Update admin address
Pinana Smartnet: ipfs://QmXtnr9aEJVywiLs1keZdyiKbQwignZT3FhwKYivF15oZp/
PMP Address : 5HrE1aYXt4k1TvBZYkL6B1BBcFp9tCWyMdCZqp2cXDNub5zW
6. Deploy Staking Contract via Proxy
With Address of PMP Project
7. Deploy Marketplace Contract via Proxy

rustup component add rustfmt --toolchain nightly
cargo +nightly fmt
