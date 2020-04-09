// Copyright (C) 2019  Braiins Systems s.r.o.
//
// This file is part of Braiins Open-Source Initiative (BOSI).
//
// BOSI is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// Please, keep in mind that we may also license BOSI or any part thereof
// under a proprietary license. For more information on the terms and conditions
// of such proprietary license or if you have any other questions, please
// contact us at opensource@braiins.com.

//! Module that represents stratum protocol errors

use std::{self, fmt, io};

use ii_async_compat::tokio_util;

#[derive(PartialEq, Debug, thiserror::Error)] //TODO: We lost Clone and Eq, is this important
pub enum Error {
    /// Input/Output error.
    #[error("I/O error: {0}")]
    Io(String),

    /// Errors emitted by serde
    #[error("Serde: {0}")]
    Serde(String),

    /// General error used for more specific .
    #[error("General error: {0}")]
    General(String),

    /// Unexpected version of something.
    #[error("Unexpected {0} version: {1}, expected: {2}")]
    UnexpectedVersion(String, String, String),

    #[error("Noise handshake error: {0}")]
    Noise(String),

    /// Stratum version 1 error
    #[error("V1 error: {0}")]
    V1(#[from] super::v1::error::Error),

    /// Stratum version 2 error
    #[error("V2 error: {0}")]
    V2(#[from] super::v2::error::Error),

    /// Hex Decode error
    #[error("Hex value decoding error: {0}")]
    HexDecode(#[from] hex::FromHexError),

    /// Invalid Bitcoin hash
    #[error("Invalid Bitcoin hash: {0}")]
    BitcoinHash(#[from] bitcoin_hashes::Error),

    /// Timeout error
    #[error("Timeout error: {0}")]
    Timeout(#[from] ii_async_compat::TimeoutError)
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e.to_string())
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Error::General(e.to_string())
        
    }
}

impl From<tokio_util::codec::LinesCodecError> for Error {
    fn from(e: tokio_util::codec::LinesCodecError) -> Self {
        Error::Io(e.to_string())
    }
}

impl From<snow::error::Error> for Error {
    fn from(e: snow::error::Error) -> Self {
        Error::Noise(e.to_string())
    }
}

impl From<ed25519_dalek::SignatureError> for Error {
    fn from(e: ed25519_dalek::SignatureError) -> Self {
        Error::Noise(e.to_string())
    }
}

impl From<bs58::decode::Error> for Error {
    fn from(e: bs58::decode::Error) -> Self {
        Error::Noise(e.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Error::General(e.to_string())
    }
}

impl From<&str> for Error {
    fn from(info: &str) -> Self {
       Error::General(info.to_string())
    }
}

impl From<String> for Error {
    fn from(info: String) -> Self {
        Error::General(info)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Error::Serde(e.to_string())
    }
}

impl From<super::v2::serialization::Error> for Error {
    fn from(e: super::v2::serialization::Error) -> Self {
        Error::Serde(e.to_string())
    }
}

/// A specialized `Result` type bound to [`Error`].
pub type Result<T> = std::result::Result<T, Error>;