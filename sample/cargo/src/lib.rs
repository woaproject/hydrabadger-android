#![cfg_attr(feature = "nightly", feature(alloc_system))]
#![cfg_attr(feature = "nightly", feature(proc_macro))]
#![cfg_attr(feature = "cargo-clippy",
            allow(large_enum_variant, new_without_default_derive, expect_fun_call, or_fun_call,
                  useless_format, cyclomatic_complexity, needless_pass_by_value, module_inception,
                  match_bool))]

// android fix
#[cfg(target_os = "android")]
extern crate android_logger;
#[macro_use]
extern crate log_panics;

#[cfg(target_os = "android")]
mod android_c_headers;
#[cfg(target_os = "android")]
pub mod java_glue;
///


#[cfg(feature = "nightly")]
extern crate alloc_system;
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
extern crate crossbeam;
// #[macro_use] extern crate crossbeam_channel;
extern crate chrono;
extern crate crypto;
extern crate num_bigint;
extern crate num_traits;
#[macro_use]
extern crate futures;
extern crate byteorder;
extern crate bytes;
extern crate rand;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;
extern crate uuid;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate clear_on_drop;
extern crate hbbft;
extern crate parking_lot;
extern crate serde;
extern crate serde_bytes;
extern crate tokio_serde_bincode;


// android fix
use android_logger::Filter;
use log::Level;
// 

#[cfg(feature = "nightly")]
use alloc_system::System;

#[cfg(feature = "nightly")]
#[global_allocator]
static A: System = System;

// pub mod network;
pub mod blockchain;
pub mod hydrabadger;
pub mod peer;

use bytes::{Bytes, BytesMut};
use futures::{sync::mpsc, AsyncSink, StartSend};
use rand::{Rand, Rng};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{self, Debug},
    marker::PhantomData,
    net::SocketAddr,
    ops::Deref,

    // android fix
    sync::{
        Arc,
    },
    thread,
    // mem,
    // Mutex,
    //
};
// android fix
use parking_lot::{Mutex};
//

use tokio::{io, net::TcpStream, prelude::*, codec::{Framed, LengthDelimitedCodec}};
use uuid::Uuid;
use hbbft::{
    crypto::{PublicKey, PublicKeySet, SecretKey, Signature},
    dynamic_honey_badger::{JoinPlan, Message as DhbMessage, DynamicHoneyBadger, Change as DhbChange},
    sync_key_gen::{Ack, Part},
    DaStep as MessagingStep,
    Contribution as HbbftContribution,
};

pub use blockchain::{Blockchain, MiningError};
pub use hydrabadger::{Config, Hydrabadger, HydrabadgerWeak};
// TODO: Create a separate, library-wide error type.
pub use hydrabadger::Error;
pub use hbbft::dynamic_honey_badger::Batch;
pub use hydrabadger::StateDsct;

/// Transmit half of the wire message channel.
// TODO: Use a bounded tx/rx (find a sensible upper bound):
type WireTx<T> = mpsc::UnboundedSender<WireMessage<T>>;

/// Receive half of the wire message channel.
// TODO: Use a bounded tx/rx (find a sensible upper bound):
type WireRx<T> = mpsc::UnboundedReceiver<WireMessage<T>>;

/// Transmit half of the internal message channel.
// TODO: Use a bounded tx/rx (find a sensible upper bound):
type InternalTx<T> = mpsc::UnboundedSender<InternalMessage<T>>;

/// Receive half of the internal message channel.
// TODO: Use a bounded tx/rx (find a sensible upper bound):
type InternalRx<T> = mpsc::UnboundedReceiver<InternalMessage<T>>;

/// Transmit half of the batch output channel.
// TODO: Use a bounded tx/rx (find a sensible upper bound):
type BatchTx<T> = mpsc::UnboundedSender<Batch<T, Uid>>;

/// Receive half of the batch output channel.
// TODO: Use a bounded tx/rx (find a sensible upper bound):
pub type BatchRx<T> = mpsc::UnboundedReceiver<Batch<T, Uid>>;

/// Transmit half of the epoch number output channel.
// TODO: Use a bounded tx/rx (find a sensible upper bound):
type EpochTx = mpsc::UnboundedSender<u64>;

/// Receive half of the epoch number output channel.
// TODO: Use a bounded tx/rx (find a sensible upper bound):
pub type EpochRx = mpsc::UnboundedReceiver<u64>;


// android fix
/// A transaction.
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Ord, PartialOrd, Debug, Clone)]
pub struct Transaction(pub String);

static mut M_TEXT1: Option<String> = None;
static mut M_TEXT2: Option<String> = None;
static mut M_TEXT3: Option<String> = None;

impl Transaction {
    fn random(len: usize) -> Option<Transaction> {
        let consonants = "bcdfghjk lmnpqrstvwxyz ";
        let mut result = String::new();

        for _i in 0..len  {
            result.push(rand::sample(&mut rand::thread_rng(), consonants.chars(), 1)[0]);
        }

        Some(Transaction(result))
    }

    fn get_tr1() -> Option<Transaction> {
        unsafe {
            let mut vec: Option<Transaction> = None;
            match M_TEXT1 {
                Some(ref mut x) => {
                    vec = Some(Transaction(x.to_string()));
                    warn!("!!get_tr1: {:?}", M_TEXT1);
                    M_TEXT1 = None;
                    vec
                }
                None => {
                    vec
                }
            }
        }
    }

    fn get_tr2() -> Option<Transaction> {
        unsafe {
            let mut vec: Option<Transaction> = None;
            match M_TEXT2 {
                Some(ref mut x) => {
                    vec = Some(Transaction(x.to_string()));
                    warn!("!!get_tr2: {:?}", M_TEXT2);
                    M_TEXT2 = None;
                    vec
                }
                None => {
                    vec
                }
            }
        }
    }

    fn get_tr3() -> Option<Transaction> {
        unsafe {
            let mut vec: Option<Transaction> = None;
            match M_TEXT3 {
                Some(ref mut x) => {
                    vec = Some(Transaction(x.to_string()));
                    warn!("!!get_tr3: {:?}", M_TEXT3);
                    M_TEXT3 = None;
                    vec
                }
                None => {
                    vec
                }
            }
        }
    }
}
//




pub trait Contribution:
    HbbftContribution + Clone + Debug + Serialize + DeserializeOwned + 'static
{
}
impl<C> Contribution for C where
    C: HbbftContribution + Clone + Debug + Serialize + DeserializeOwned + 'static
{}

/// A unique identifier.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Uid(pub(crate) Uuid);

impl Uid {
    /// Returns a new, random `Uid`.
    pub fn new() -> Uid {
        Uid(Uuid::new_v4())
    }
}

impl Rand for Uid {
    fn rand<R: Rng>(_rng: &mut R) -> Uid {
        Uid::new()
    }
}

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl fmt::Debug for Uid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

type Message = DhbMessage<Uid>;
type Step<T> = MessagingStep<DynamicHoneyBadger<T, Uid>>;
type Change = DhbChange<Uid>;

/// A peer's incoming (listening) address.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InAddr(pub SocketAddr);

impl Deref for InAddr {
    type Target = SocketAddr;
    fn deref(&self) -> &SocketAddr {
        &self.0
    }
}

impl fmt::Display for InAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InAddr({})", self.0)
    }
}

/// An internal address used to respond to a connected peer.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OutAddr(pub SocketAddr);

impl Deref for OutAddr {
    type Target = SocketAddr;
    fn deref(&self) -> &SocketAddr {
        &self.0
    }
}

impl fmt::Display for OutAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OutAddr({})", self.0)
    }
}

/// Nodes of the network.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkNodeInfo {
    pub(crate) uid: Uid,
    pub(crate) in_addr: InAddr,
    pub(crate) pk: PublicKey,
}

type ActiveNetworkInfo = (Vec<NetworkNodeInfo>, PublicKeySet, BTreeMap<Uid, PublicKey>);

/// The current state of the network.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetworkState {
    None,
    Unknown(Vec<NetworkNodeInfo>),
    AwaitingMorePeersForKeyGeneration(Vec<NetworkNodeInfo>),
    GeneratingKeys(Vec<NetworkNodeInfo>, BTreeMap<Uid, PublicKey>),
    Active(ActiveNetworkInfo),
}

/// Messages sent over the network between nodes.
///
/// Only [`Message`](enum.WireMessageKind.html#variant.Message) variants are
/// verified.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WireMessageKind<T> {
    HelloFromValidator(Uid, InAddr, PublicKey, NetworkState),
    HelloRequestChangeAdd(Uid, InAddr, PublicKey),
    WelcomeReceivedChangeAdd(Uid, PublicKey, NetworkState),
    RequestNetworkState,
    NetworkState(NetworkState),
    Goodbye,
    #[serde(with = "serde_bytes")]
    // TODO(c0gent): Remove.
    Bytes(Bytes),
    /// A Honey Badger message.
    ///
    /// All received messages are verified against the senders public key
    /// using an attached signature.
    Message(Uid, Message),
    // TODO(c0gent): Remove.
    Transaction(Uid, T),
    KeyGenPart(Part),
    KeyGenAck(Ack),
    JoinPlan(JoinPlan<Uid>),
}

/// Messages sent over the network between nodes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WireMessage<T> {
    kind: WireMessageKind<T>,
}

impl<T: Contribution> WireMessage<T> {
    pub fn hello_from_validator(
        src_uid: Uid,
        in_addr: InAddr,
        pk: PublicKey,
        net_state: NetworkState,
    ) -> WireMessage<T> {
        WireMessageKind::HelloFromValidator(src_uid, in_addr, pk, net_state).into()
    }

    /// Returns a `HelloRequestChangeAdd` variant.
    pub fn hello_request_change_add(
        src_uid: Uid,
        in_addr: InAddr,
        pk: PublicKey,
    ) -> WireMessage<T> {
        WireMessage {
            kind: WireMessageKind::HelloRequestChangeAdd(src_uid, in_addr, pk),
        }
    }

    /// Returns a `WelcomeReceivedChangeAdd` variant.
    pub fn welcome_received_change_add(
        src_uid: Uid,
        pk: PublicKey,
        net_state: NetworkState,
    ) -> WireMessage<T> {
        WireMessage {
            kind: WireMessageKind::WelcomeReceivedChangeAdd(src_uid, pk, net_state),
        }
    }

    /// Returns an `Input` variant.
    pub fn transaction(src_uid: Uid, txn: T) -> WireMessage<T> {
        WireMessage {
            kind: WireMessageKind::Transaction(src_uid, txn),
        }
    }

    /// Returns a `Message` variant.
    pub fn message(src_uid: Uid, msg: Message) -> WireMessage<T> {
        WireMessage {
            kind: WireMessageKind::Message(src_uid, msg),
        }
    }

    pub fn key_gen_part(part: Part) -> WireMessage<T> {
        WireMessage {
            kind: WireMessageKind::KeyGenPart(part),
        }
    }

    pub fn key_gen_part_ack(outcome: Ack) -> WireMessage<T> {
        WireMessageKind::KeyGenAck(outcome).into()
    }

    pub fn join_plan(jp: JoinPlan<Uid>) -> WireMessage<T> {
        WireMessageKind::JoinPlan(jp).into()
    }

    /// Returns the wire message kind.
    pub fn kind(&self) -> &WireMessageKind<T> {
        &self.kind
    }

    /// Consumes this `WireMessage` into its kind.
    pub fn into_kind(self) -> WireMessageKind<T> {
        self.kind
    }
}

impl<T: Contribution> From<WireMessageKind<T>> for WireMessage<T> {
    fn from(kind: WireMessageKind<T>) -> WireMessage<T> {
        WireMessage { kind }
    }
}

/// A serialized `WireMessage` signed by the sender.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedWireMessage {
    message: Vec<u8>,
    sig: Signature,
}

/// A stream/sink of `WireMessage`s connected to a socket.
pub struct WireMessages<T: Contribution> {
    framed: Framed<TcpStream, LengthDelimitedCodec>,
    our_key: SecretKey,
    hdb: Hydrabadger<T>,
    _t: PhantomData<T>,
}

impl<T: Contribution> WireMessages<T> {
    pub fn new(socket: TcpStream, our_key: SecretKey, hdb: Hydrabadger<T>) -> WireMessages<T> {
        WireMessages {
            framed: Framed::new(socket, LengthDelimitedCodec::new()),
            our_key,
            hdb,
            _t: PhantomData,
        }
    }

    pub fn socket(&self) -> &TcpStream {
        self.framed.get_ref()
    }

    pub fn send_msg(&mut self, msg: WireMessage<T>) -> Result<(), Error> {
        self.start_send(msg)?;
        let _ = self.poll_complete()?;
        Ok(())
    }
}

impl<T: Contribution> Stream for WireMessages<T> {
    type Item = WireMessage<T>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match try_ready!(self.framed.poll()) {
            Some(frame) => {
                let s_msg: SignedWireMessage =
                    bincode::deserialize(&frame.freeze()).map_err(Error::Serde)?;
                let msg: WireMessage<T> =
                    bincode::deserialize(&s_msg.message).map_err(Error::Serde)?;

                // Verify signature for `WireMessageKind::Message` variants.
                if let WireMessageKind::Message(uid, _) = &msg.kind {
                    match self.hdb.peers().get_by_uid(uid).and_then(|peer| peer.public_key()) {
                        Some(pk) => {
                            if !pk.verify(&s_msg.sig, &s_msg.message) {
                                return Err(Error::InvalidSignature);
                            }
                        },
                        None => return Err(Error::MessageReceivedUnknownPeer),
                    }
                }

                Ok(Async::Ready(Some(msg)))
            }
            None => Ok(Async::Ready(None)),
        }
    }
}

impl<T: Contribution> Sink for WireMessages<T> {
    type SinkItem = WireMessage<T>;
    type SinkError = Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        // TODO: Reuse buffer:
        let mut serialized = BytesMut::new();

        let message = bincode::serialize(&item).map_err(Error::Serde)?;
        let sig = self.our_key.sign(&message);

        match bincode::serialize(&SignedWireMessage { message, sig }) {
            Ok(s) => serialized.extend_from_slice(&s),
            Err(err) => return Err(Error::Io(io::Error::new(io::ErrorKind::Other, err))),
        }
        match self.framed.start_send(serialized.freeze()) {
            Ok(async_sink) => match async_sink {
                AsyncSink::Ready => Ok(AsyncSink::Ready),
                AsyncSink::NotReady(_) => Ok(AsyncSink::NotReady(item)),
            },
            Err(err) => Err(Error::Io(err)),
        }
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.framed.poll_complete().map_err(Error::from)
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        self.framed.close().map_err(Error::from)
    }
}

/// A message between internal threads/tasks.
#[derive(Clone, Debug)]
pub enum InternalMessageKind<T: Contribution> {
    Wire(WireMessage<T>),
    HbMessage(Message),
    HbContribution(T),
    HbChange(Change),
    PeerDisconnect,
    NewIncomingConnection(InAddr, PublicKey, bool),
    NewOutgoingConnection,
}

/// A message between internal threads/tasks.
#[derive(Clone, Debug)]
pub struct InternalMessage<T: Contribution> {
    src_uid: Option<Uid>,
    src_addr: OutAddr,
    kind: InternalMessageKind<T>,
}

impl<T: Contribution> InternalMessage<T> {
    pub fn new(
        src_uid: Option<Uid>,
        src_addr: OutAddr,
        kind: InternalMessageKind<T>,
    ) -> InternalMessage<T> {
        InternalMessage {
            src_uid,
            src_addr,
            kind,
        }
    }

    /// Returns a new `InternalMessage` without a uid.
    pub fn new_without_uid(src_addr: OutAddr, kind: InternalMessageKind<T>) -> InternalMessage<T> {
        InternalMessage::new(None, src_addr, kind)
    }

    pub fn wire(
        src_uid: Option<Uid>,
        src_addr: OutAddr,
        wire_message: WireMessage<T>,
    ) -> InternalMessage<T> {
        InternalMessage::new(src_uid, src_addr, InternalMessageKind::Wire(wire_message))
    }

    pub fn hb_message(src_uid: Uid, src_addr: OutAddr, msg: Message) -> InternalMessage<T> {
        InternalMessage::new(Some(src_uid), src_addr, InternalMessageKind::HbMessage(msg))
    }

    pub fn hb_contribution(src_uid: Uid, src_addr: OutAddr, contrib: T) -> InternalMessage<T> {
        InternalMessage::new(Some(src_uid), src_addr, InternalMessageKind::HbContribution(contrib))
    }

    pub fn hb_vote(src_uid: Uid, src_addr: OutAddr, change: Change) -> InternalMessage<T> {
        InternalMessage::new(Some(src_uid), src_addr, InternalMessageKind::HbChange(change))
    }

    pub fn peer_disconnect(src_uid: Uid, src_addr: OutAddr) -> InternalMessage<T> {
        InternalMessage::new(Some(src_uid), src_addr, InternalMessageKind::PeerDisconnect)
    }

    pub fn new_incoming_connection(
        src_uid: Uid,
        src_addr: OutAddr,
        src_in_addr: InAddr,
        src_pk: PublicKey,
        request_change_add: bool,
    ) -> InternalMessage<T> {
        InternalMessage::new(
            Some(src_uid),
            src_addr,
            InternalMessageKind::NewIncomingConnection(src_in_addr, src_pk, request_change_add),
        )
    }

    pub fn new_outgoing_connection(src_addr: OutAddr) -> InternalMessage<T> {
        InternalMessage::new_without_uid(src_addr, InternalMessageKind::NewOutgoingConnection)
    }

    /// Returns the source unique identifier this message was received in.
    pub fn src_uid(&self) -> Option<&Uid> {
        self.src_uid.as_ref()
    }

    /// Returns the source socket this message was received on.
    pub fn src_addr(&self) -> &OutAddr {
        &self.src_addr
    }

    /// Returns the internal message kind.
    pub fn kind(&self) -> &InternalMessageKind<T> {
        &self.kind
    }

    /// Consumes this `InternalMessage` into its parts.
    pub fn into_parts(self) -> (Option<Uid>, OutAddr, InternalMessageKind<T>) {
        (self.src_uid, self.src_addr, self.kind)
    }
}







// android fix
use std::collections::HashSet;

trait OnEvent {
    fn changed(&self, its_me: bool, id: String, trans: String);
}

fn callback(num_call_back: i32, its_me: bool, id: String, trans: String) {
    unsafe {
        match M_SESSION_PTR {
            Some(ref mut x) => x.change(num_call_back, its_me, id, trans),
            None => panic!(),
        } 
    }
}

static mut M_SESSION_PTR: Option<&'static mut Session> = None;
static mut M_NUM_OF_CALLBACK: i32 = 0;

struct Session {
    observers: Vec<Box<OnEvent>>,
    handler1: Arc<Mutex<Option<Hydrabadger<Vec<Option<Transaction>>>>>>,
    handler2: Arc<Mutex<Option<Hydrabadger<Vec<Option<Transaction>>>>>>,
    handler3: Arc<Mutex<Option<Hydrabadger<Vec<Option<Transaction>>>>>>,
}

impl Session {
    pub fn new() -> Session {
        android_logger::init_once(
            Filter::default()
                .with_min_level(Level::Trace), // limit log level
            Some("HYDRABADGERTAG") // logs will show under mytag tag. If `None`, the crate name will be used
        ); 
           
        log_panics::init(); // log panics rather than printing them
        info!("init log system - done");

        Session {  observers: Vec::new(),
                    handler1: Arc::new(Mutex::new(None)),
                    handler2: Arc::new(Mutex::new(None)),
                    handler3: Arc::new(Mutex::new(None)),
                }
    }

    fn subscribe(&mut self, cb: Box<OnEvent>) {
        warn!("subscribe");
        self.observers.push(cb);
    }

    pub fn after_subscribe(&'static mut self) {
        warn!("!! after_subscribe");
        unsafe {
            M_SESSION_PTR = Some(self);
        }

        callback(0, true, "test".to_string(), "test".to_string());
    }

    pub fn send_message(&self, num: i32, str1: String) {
        unsafe {
            let new_string = format!("{}!", str1);
            warn!("!!send_message string: {:?}", new_string);
            if num == 0 {
                M_TEXT1 = Some(new_string.clone());
            }
            else if num == 1 {
                M_TEXT2 = Some(new_string.clone());
            }
            else if num == 2 {
                M_TEXT3 = Some(new_string.clone());
            }
        }
    }

    pub fn change(&self, x: i32, its_me: bool, id: String, trans: String) {
        let mut i = 0;
        for cb in &self.observers {
            if i == x {
                warn!("Call callback at number: {:?}", i);
                cb.changed(its_me, id.clone(), trans.clone());
            }
            i += 1;
        }
    }

    pub fn start_node(&self, ipport_string_source: String, ipport_string_myout: String, ipports_string_remote: String) {
        unsafe {
            warn!("enter to startNode: {:?}", M_NUM_OF_CALLBACK.clone());
        }

        warn!("parse 1 address");
        let bind_address: SocketAddr = ipport_string_source.parse().expect("Unable to parse socket address bind_address");
        warn!("parse 2 address");
        let bind_address_out: SocketAddr = ipport_string_myout.parse().expect("Unable to parse socket address bind_address_out");
    
        warn!("before remote_addresses");
        let mut remote_addresses: HashSet<SocketAddr> = HashSet::new();
        if !ipports_string_remote.is_empty() {
            let split = ipports_string_remote.split(";");
            for address in split {
                remote_addresses.insert(address.parse().expect("Unable to parse socket address remote_addresses"));
            }
        }
        warn!("after remote_addresses");

        let cfg = Config::default();
         
        let callback_ = callback;

        unsafe {
            let num = M_NUM_OF_CALLBACK.clone();

            if num == 0 {
                *self.handler1.lock() = Some(Hydrabadger::new(bind_address, bind_address_out, cfg, callback_, num));
                M_NUM_OF_CALLBACK += 1;
                
                match self.handler1.lock().take() {
                    Some(v) => {
                        let gen_txn = || {
                            (0..1)
                                .map(|_| Transaction::get_tr1())
                                // .map(|_| Transaction::random(5))
                                .collect::<Vec<_>>()
                        };
                        thread::spawn(move || {
                            if !ipports_string_remote.is_empty() {
                                v.run_node(Some(remote_addresses), Some(gen_txn));
                            }
                            else {
                                v.run_node(None, Some(gen_txn));
                            }
                            warn!("!!run_node Node: {:?}", num);
                        });
                    },
                    None => {},
                }
                warn!("!!match out started Node: {:?}", num);
            }
            else if num == 1 {
                *self.handler2.lock() = Some(Hydrabadger::new(bind_address, bind_address_out, cfg, callback_, num));
                M_NUM_OF_CALLBACK += 1;
       
                match self.handler2.lock().take() {
                    Some(v) => {
                        let gen_txn = || {
                            (0..1)
                                .map(|_| Transaction::get_tr2())
                                // .map(|_| Transaction::random(5))
                                .collect::<Vec<_>>()
                        };

                        thread::spawn(move || {
                            v.run_node(Some(remote_addresses), Some(gen_txn));
                            warn!("!!run_node Node: {:?}", num);
                        });
                    },
                    None => {},
                }
                warn!("!!match out started Node: {:?}", num);
            }
            else if num == 2 {
                *self.handler3.lock() = Some(Hydrabadger::new(bind_address, bind_address_out, cfg, callback_, num));
                M_NUM_OF_CALLBACK += 1;

                match self.handler3.lock().take() {
                    Some(v) => {
                        let gen_txn = || {
                            (0..1)
                                // .map(|_| Transaction::random(5))
                                .map(|_| Transaction::get_tr3())
                                .collect::<Vec<_>>()
                        };

                        thread::spawn(move || {
                            v.run_node(Some(remote_addresses), Some(gen_txn));
                            warn!("!!run_node Node: {:?}", num);
                        });
                    },
                    None => {},
                }
                warn!("!!match out started Node: {:?}", num);
            }
        }
    }

}
//