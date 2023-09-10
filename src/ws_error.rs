#[derive(Debug)]
pub enum WsError {
    InvalidArgumentsError,
    ConnectError(websocket_lite::Error),
    StreamEnded,
    MessageError(websocket_lite::Error),
    TextDecodeError,
    CloseOpcodeReceived,
}

impl std::fmt::Display for WsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for WsError {}
