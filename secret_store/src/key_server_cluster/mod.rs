// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

#![allow(unused_mut)] // TODO: remove me
#![allow(dead_code)] // TODO: remove me
#![allow(unused_imports)] // TODO: remove me
#![allow(unused_variables)] // TODO: remove me
#![allow(unreachable_code)] // TODO: remove me

use std::fmt;
use std::io::Error as IoError;
use ethkey;
use ethcrypto;
use super::types::all::ServerKeyId;

pub use super::types::all::{NodeId, EncryptedDocumentKeyShadow};
pub use super::acl_storage::AclStorage;
pub use super::key_storage::{KeyStorage, DocumentKeyShare};
pub use super::serialization::{SerializableSignature, SerializableH256, SerializableSecret, SerializablePublic, SerializableMessageHash};
pub use self::cluster::{ClusterCore, ClusterConfiguration, ClusterClient};
pub use self::generation_session::Session as GenerationSession;
pub use self::encryption_session::Session as EncryptionSession;
pub use self::decryption_session::Session as DecryptionSession;

#[cfg(test)]
pub use super::key_storage::tests::DummyKeyStorage;
#[cfg(test)]
pub use super::acl_storage::tests::DummyAclStorage;

pub type SessionId = ServerKeyId;

#[derive(Clone, Debug, PartialEq)]
/// Errors which can occur during encryption/decryption session
pub enum Error {
	/// Invalid node address has been passed.
	InvalidNodeAddress,
	/// Invalid node id has been passed.
	InvalidNodeId,
	/// Session with the given id already exists.
	DuplicateSessionId,
	/// Session with the same id already completed.
	CompletedSessionId,
	/// Session is not ready to start yet (required data is not ready).
	NotStartedSessionId,
	/// Session with the given id is unknown.
	InvalidSessionId,
	/// Invalid number of nodes.
	/// There must be at least two nodes participating in encryption.
	/// There must be at least one node participating in decryption.
	InvalidNodesCount,
	/// Node which is required to start encryption/decryption session is not a part of cluster.
	InvalidNodesConfiguration,
	/// Invalid threshold value has been passed.
	/// Threshold value must be in [0; n - 1], where n is a number of nodes participating in the encryption.
	InvalidThreshold,
	/// Current state of encryption/decryption session does not allow to proceed request.
	/// Reschedule this request for later processing.
	TooEarlyForRequest,
	/// Current state of encryption/decryption session does not allow to proceed request.
	/// This means that either there is some comm-failure or node is misbehaving/cheating.
	InvalidStateForRequest,
	/// TODO
	InvalidNodeForRequest,
	/// Message or some data in the message was recognized as invalid.
	/// This means that node is misbehaving/cheating.
	InvalidMessage,
	/// Connection to node, required for this session is not established.
	NodeDisconnected,
	/// Cryptographic error.
	EthKey(String),
	/// I/O error has occured.
	Io(String),
	/// Deserialization error has occured.
	Serde(String),
	/// Key storage error.
	KeyStorage(String),
	/// Consensus is unreachable.
	ConsensusUnreachable,
	/// Acl storage error.
	AccessDenied,
}

impl From<ethkey::Error> for Error {
	fn from(err: ethkey::Error) -> Self {
		Error::EthKey(err.into())
	}
}

impl From<ethcrypto::Error> for Error {
	fn from(err: ethcrypto::Error) -> Self {
		Error::EthKey(err.into())
	}
}

impl From<IoError> for Error {
	fn from(err: IoError) -> Self {
		Error::Io(err.to_string())
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::InvalidNodeAddress => write!(f, "invalid node address has been passed"),
			Error::InvalidNodeId => write!(f, "invalid node id has been passed"),
			Error::DuplicateSessionId => write!(f, "session with the same id is already registered"),
			Error::CompletedSessionId => write!(f, "session with the same id is already completed"),
			Error::NotStartedSessionId => write!(f, "not enough data to start session with the given id"),
			Error::InvalidSessionId => write!(f, "invalid session id has been passed"),
			Error::InvalidNodesCount => write!(f, "invalid nodes count"),
			Error::InvalidNodesConfiguration => write!(f, "invalid nodes configuration"),
			Error::InvalidThreshold => write!(f, "invalid threshold value has been passed"),
			Error::TooEarlyForRequest => write!(f, "session is not yet ready to process this request"),
			Error::InvalidStateForRequest => write!(f, "session is in invalid state for processing this request"),
			Error::InvalidNodeForRequest => write!(f, "node cannot respond to this request"),
			Error::InvalidMessage => write!(f, "invalid message is received"),
			Error::NodeDisconnected => write!(f, "node required for this operation is currently disconnected"),
			Error::EthKey(ref e) => write!(f, "cryptographic error {}", e),
			Error::Io(ref e) => write!(f, "i/o error {}", e),
			Error::Serde(ref e) => write!(f, "serde error {}", e),
			Error::KeyStorage(ref e) => write!(f, "key storage error {}", e),
			Error::ConsensusUnreachable => write!(f, "Consensus unreachable"),
			Error::AccessDenied => write!(f, "Access denied"),
		}
	}
}

impl Into<String> for Error {
	fn into(self) -> String {
		format!("{}", self)
	}
}

mod cluster;
mod cluster_sessions;
mod consensus;
mod consensus_session;
mod decryption_session;
mod encryption_session;
mod generation_session;
mod io;
pub mod math;
mod message;
mod signing_session;
mod net;
