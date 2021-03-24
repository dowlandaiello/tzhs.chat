enum CmdType {
    MSG,
}

/// A command issued by a websockets client.
pub struct WSCmd {
    kind: CmdType,
}
