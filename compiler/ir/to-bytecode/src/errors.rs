// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;
use omv_core::errors::VMError;

#[derive(Clone, Debug, Eq, Error, Ord, PartialEq, PartialOrd)]
pub enum InternalCompilerError {
    #[error("Post-compile bounds check errors: {0:?}")]
    BoundsCheckErrors(VMError),
}
