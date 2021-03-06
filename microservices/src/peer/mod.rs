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

mod peer_connection;
pub use peer_connection::{
    PeerConnection, PeerReceiver, PeerSender, RecvMessage, SendMessage,
};

use internet2::presentation;
use lnp::Messages;

use crate::node::TryService;

/// Trait for types handling specific LNPWP messages.
pub trait Handler {
    type Error: crate::error::Error + From<presentation::Error>;

    /// Function that processes specific peer message
    fn handle(&mut self, message: Messages) -> Result<(), Self::Error>;

    fn handle_err(&mut self, error: Self::Error) -> Result<(), Self::Error>;
}

pub struct Listener<H>
where
    H: Handler,
{
    receiver: PeerReceiver,
    handler: H,
}

impl<H> Listener<H>
where
    H: Handler,
{
    pub fn with(receiver: PeerReceiver, handler: H) -> Self {
        Self { receiver, handler }
    }
}

impl<H> TryService for Listener<H>
where
    H: Handler,
{
    type ErrorType = H::Error;

    fn try_run_loop(mut self) -> Result<(), Self::ErrorType> {
        trace!("Entering event loop of the sender service");
        loop {
            match self.run() {
                Ok(_) => trace!("Peer message processing complete"),
                Err(err) => {
                    trace!("Peer connection generated {}", err);
                    self.handler.handle_err(err)?;
                }
            }
        }
    }
}

impl<H> Listener<H>
where
    H: Handler,
{
    fn run(&mut self) -> Result<(), H::Error> {
        trace!("Awaiting for peer messages...");
        let msg = self.receiver.recv_message()?;
        debug!("Processing message {}", msg);
        trace!("Message details: {:?}", msg);
        self.handler.handle(msg)
    }
}
