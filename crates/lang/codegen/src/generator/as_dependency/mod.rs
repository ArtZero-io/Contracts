// Copyright 2018-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod call_builder;
mod contract_ref;

use self::{
    call_builder::CallBuilder,
    contract_ref::ContractRef,
};
use crate::{
    traits::GenerateCodeUsing,
    GenerateCode,
};
use derive_more::From;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Generates `#[cfg(...)]` code to guard against compilation under `ink-as-dependency`.
#[derive(From)]
pub struct NotAsDependencyCfg<'a> {
    contract: &'a ir::Contract,
}

impl GenerateCode for NotAsDependencyCfg<'_> {
    fn generate_code(&self) -> TokenStream2 {
        if self.contract.config().is_compile_as_dependency_enabled() {
            // We use `__ink_DO_NOT_COMPILE` in order to craft a `cfg` that
            // never evaluates to `true` and therefore is always disabled.
            return quote! { #[cfg(feature = "__ink_DO_NOT_COMPILE")] }
        }
        quote! { #[cfg(not(feature = "ink-as-dependency"))] }
    }
}

/// Generates `#[cfg(...)]` code to only allow compilation when `ink-as-dependency` is enabled.
///
/// The `ink-as-dependency` can be enabled mainly by 2 different ways:
///
/// - Enabling it in the associated `Cargo.toml` as crate feature.
///     - Note: This can be enabled by dependencies of an ink! smart contract.
/// - Enabling it in the configuration header with `#[ink::contract(compile_as_dependency = true)]`.
///     - If set here the contract will always be compiled as it is was a dependency.
#[derive(From)]
pub struct OnlyAsDependencyCfg<'a> {
    contract: &'a ir::Contract,
}

impl GenerateCode for OnlyAsDependencyCfg<'_> {
    fn generate_code(&self) -> TokenStream2 {
        if self.contract.config().is_compile_as_dependency_enabled() {
            // We return no code since no code is required to disable compilation.
            return quote! {}
        }
        quote! {
            #[cfg(feature = "ink-as-dependency")]
        }
    }
}

/// Generates code for generating a contract reference.
///
/// Contract references are used to dynamically depend on a smart contract.
/// The contract reference is just a typed thin-wrapper around an `AccountId`
/// that implements an API mirrored by the smart contract.
#[derive(From)]
pub struct ContractReference<'a> {
    /// The contract to generate code for.
    contract: &'a ir::Contract,
}
impl_as_ref_for_generator!(ContractReference);

impl GenerateCode for ContractReference<'_> {
    /// Generates ink! contract code.
    fn generate_code(&self) -> TokenStream2 {
        let call_builder = self.generate_code_using::<CallBuilder>();
        let call_forwarder = self.generate_code_using::<ContractRef>();
        quote! {
            #call_builder
            #call_forwarder
        }
    }
}
