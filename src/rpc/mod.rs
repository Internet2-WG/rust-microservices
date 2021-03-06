// LNP/BP Core Library implementing LNPBP specifications & standards
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#[cfg(feature = "client")]
pub mod client;
mod connection;
mod error;
#[cfg(feature = "node")]
pub mod server;

pub use connection::{Api, Reply, Request, RpcConnection};
pub use error::{ClientError, Failure, FailureCode, FailureCodeExt, ServerError};

/// Marker traits for endpoint identifiers lists
pub trait EndpointId: Copy + Eq + std::hash::Hash + std::fmt::Display {}
