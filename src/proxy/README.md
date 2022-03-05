# Proxy Smart Contract

The proxy smart contract forwards any call that does not match a
selector of itself to another, specified contract.

The instantiator of the proxy contract on a blockchain can change
the address to which calls are forwarded.

This allows building upgradable contracts following the proxy pattern.
Note though that the state is still stored in the contract to which
calls are forwarded.

In order to test it out you need to do the following:

1. Build a contract containing some logic, e.g. our flipper example:
   ```
   cargo +nightly contract build --manifest-path=examples/flipper/Cargo.toml
   ```
   You will receive the respective `flipper.contract` bundle in the `examples/flipper/target/ink/` folder.
1. Build the proxy contract:
   ```
   cargo +nightly contract build --manifest-path=examples/proxy/Cargo.toml
   ```
   You will receive the respective `proxy.contract` bundle in the `examples/proxy/target/ink/` folder.
1. Upload the `flipper.contract` to the chain.
1. Upload the `proxy.contract` to the chain. During instantiation specify the just instantiated
   `flipper` contract as the `forward_to` parameter.
1. Switch the metadata of the just instantiated `proxy` contract to the metadata of the `flipper`
   contract. In the `polkadot-js` UI this can be done this way:
   1. Click the icon left of the instantiated `proxy` contract to copy the address
      of it into your clipboard.
   1. Click `Add an existing contract`, insert the just copied address, upload the `flipper.contract`
      for the `Contract ABI`.
1. Now you are able to run the operations provided by the `flipper` smart contract via
   the `proxy` contract.

To change the address of the smart contract where calls are forwarded to you would
switch the metadata (i.e. the `Contract ABI`) back to the `proxy` contract
and then invoke the `change_forward_address` message.
