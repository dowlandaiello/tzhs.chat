/*
 * - Name: cmd.rs
 * - Author: Dowland Aiello
 * - License: MIT
 *
 * THE TZHS.CHAT BINARY MESSAGE PROTOCOL
 *
 * All messages transmitted along a WebSocket connection must be sent as UTF-8
 * text (unicode allowed). Messages must be prefixed with their intention, or
 * the COMMAND they are specifying:
 *
 * CMD { ARGUMENT } { ARGUMENT } { ARGUMENT } ...
 */

use num_traits::FromPrimitive;
use std::{error::Error, convert::TryFrom};

/// The kind of command being issued.
#[derive(FromPrimitive)]
pub enum CmdKind {
    /// Sends a message to the chat room
    MSG,

    /// Mutes the targeted user on the local device
    MUTE,

    /// Sends a whisper to the targeted user
    DM,
}

/// A command issued by a websocket client
pub struct Cmd<'a> {
    /// The type of the command
    pub kind: CmdKind,

    /// Anything except the type of the cmd specified
    pub args: Vec<&'a str>,
}

/// Occurs when an error coming from a client WS conn is malformed.
pub enum ParseCmdError {
    InvalidKind(Option<Box<dyn Error>>),
}

impl<'a> TryFrom<&'a str> for Cmd<'a> {
    type Error = ParseCmdError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            // Determine the type of CMD from a decimal code
            kind: s[..1]
                .parse()
                .map_err(|e| ParseCmdError::InvalidKind(Some(Box::new(e))))
                .and_then(|cmd_id| {
                    CmdKind::from_u8(cmd_id).ok_or(ParseCmdError::InvalidKind(None))
                })?,
            args: s.split(" { ").collect(),
        })
    }
}
