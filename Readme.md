Code Hashes and Contract Addresses:

# Proxy Contract Hash Code
0x57470072d911a2103bfe6cda78bcf7375cf55dc9e56d4bef2b39b1824ab41d79

# Profile Contract
0x78ea710ed189e29ae3f2ca46672a7ede3fe14692882602191a48425888fe4253
5E2zk4B1p4ohQmwG3uFw16yoZbYWE3ZRnb6XgPLNvi5zhG7U

# ArtZero NFT Contract / Mint (PROXY)
0x1958af8a820b5b6689a84fb4974a8a1bb7457eb7933933e51dc82a45fb198b18
5EuozqMa5FEVt3FGFRRddUhZCpH7N5VDNvipNsTcFFRSN7Lp

# ArtZero NFT Staking Contract (PROXY)
0x0e7ff4fb9b7c1ef631975277c64095571e2437562c6035d803ab4c7862a320c4
5Gh6JkVixyzpU5Ne6THQKyMZwwugoAwnzmwNyDqqJD6SzaoB

# Collection Manager (PROXY)
0x29e9d3d74451d008a7f9ff464c6e8d648d2c498255b8f46c0d5a6a2290d76923
5DeTsjiUsy7nNiZU7tqrm2N5pxivdKGWHPxxkJs58TzpYjqL

# Marketplace Contract (PROXY)
0x9a0b60053b96d9dd31ce241e98e1014586d8e87c7e6cf899a43593011026ad2b
5C8xoEUyK2CcuAU3DZryMdejcTAtBFrBZzx4wtqrkH9os796

# LaunchPad Manager (PROXY)
0x3d78dba0e916069f5ef324a4d296d79a0be029e5218cc4cb546fac3d5975a2ea
5Go6HPGUcgpKHwZXVcDDeoJ1inbNxXSuKdzhhDJauaUHBxVq

# Standard Contract
# LaunchPad Standard Contract
0xdda946b6a49f21397414f00f6be489e606b429fca44ffa280f5fb7953e8cceb4

# ArtZero NFT Standard Contract
0xfe96060f8e8f5281d79f30ffea0bd9ce27f093249f39146d753a79bee55ccafa

Deploy Steps:
1. Deploy Profile Contract
2. Deploy NFT Standard Contract
3. Deploy Collection Contract via Proxy
4. Deploy LaunchPad Standard Contract
5. LaunchPad Manager via Proxy with Code Hash of LaunchPad Standard
Add PMP Project -> create Collection -> Update base URI -> Update admin address
Pinana Smartnet: ipfs://QmXtnr9aEJVywiLs1keZdyiKbQwignZT3FhwKYivF15oZp/
PMP Address : 5FSRWvbsnaNeAyNsU1jz3DaMniEfRbWNCUvjWn2JwAPbR85p
6. Deploy Staking Contract via Proxy
With Address of PMP Project
7. Deploy Marketplace Contract via Proxy

rustup component add rustfmt --toolchain nightly
cargo +nightly fmt
