/// The error message to be used inside the `Failure` state.
///
/// This is mostly used as an error type when returning results with a list of
/// handle actions.
pub type FailureMsg = String;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SignalingState {
    ServerHandshake,
    PeerHandshake,
    Task,
}

/// The server handshake states.
///
/// The `ClientHello` state is only valid for the responder role, otherwise the
/// state will transition from `ServerHello` to `ClientAuth` directly.
///
/// If any invalid transition happens, the state will change to the terminal
/// `Failure` state.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ServerHandshakeState {
    /// Initial state.
    New,
    /// The client-hello (only responder) and client-auth messages have been sent.
    ClientInfoSent,
    /// The server-auth message has been received and processed.
    Done,
    /// Something went wrong. This is a terminal state.
    Failure(String),
}

/// The initiator handshake states.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InitiatorHandshakeState {
    New,
    TokenSent,
    KeySent,
    KeyReceived,
    AuthSent,
    AuthReceived,
    Failure(String),
}

/// The responder handshake states.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ResponderHandshakeState {
    New,
    TokenReceived,
    KeyReceived,
    KeySent,
    AuthReceived,
    AuthSent,
    Failure(String),
}
