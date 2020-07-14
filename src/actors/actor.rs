use super::context::Context;
use std::sync::mpsc;
use std::collections::HashMap;
use std::fmt::Debug;
use crossbeam_channel::{ Sender };

pub trait Actor: Send + Debug {
    fn handle(&mut self, msg: String, ctx: Context) -> ();

    fn on_start(&mut self, _ctx: Context) -> () {}
    fn on_stop(&mut self, _ctx: Context) -> () {}
}

pub enum ActorMessage {
    NewActor {
        actor_id: String,
        actor: Box<dyn Actor>
    },
    Msg {
        actor_id: String,
        msg: String
    },
    Stop {
        actor_id: String
    }
}

pub trait Address: Sized {
    fn send(&mut self, msg: String) -> ();

    fn stop(self) -> ();
}

pub struct LocalAddress<'a> {
    actor_id: String,
    sender: &'a Sender<ActorMessage>
}

impl<'a> LocalAddress<'a> {
    pub fn new(actor_id: String, sender: &'a Sender<ActorMessage>) -> LocalAddress<'a> {
        return LocalAddress {
            actor_id,
            sender
        };
    }
}

impl<'a> Address for LocalAddress<'a> {
    fn send(&mut self, msg: String) -> () {
        self.sender.send(ActorMessage::Msg {
            actor_id: self.actor_id.clone(),
            msg: msg
        }).unwrap();
    }

    fn stop(self) -> () {
        self.sender.send(ActorMessage::Stop {
            actor_id: self.actor_id.clone()
        }).unwrap();
    }
}

pub struct RemoteAddress {
    actor_id: String,
    sender: Sender<ActorMessage>
}

impl RemoteAddress {
    pub fn new(actor_id: String, sender: Sender<ActorMessage>) -> RemoteAddress {
        return RemoteAddress {
            actor_id,
            sender
        };
    }
}

impl Address for RemoteAddress {
    fn send(&mut self, msg: String) -> () {
        self.sender.send(ActorMessage::Msg {
            actor_id: self.actor_id.clone(),
            msg: msg
        }).unwrap();
    }

    fn stop(self) -> () {
        self.sender.send(ActorMessage::Stop {
            actor_id: self.actor_id.clone()
        }).unwrap();
    }
}

pub struct GroupAddress {
    senders: Vec<(String, Sender<ActorMessage>>
}

impl GroupAddress {
    pub fn new(senders: Vec<(String, Sender<ActorMessage>)>) -> GroupAddress {
        return GroupAddress {
            senders
        };
    }
}

impl Address for GroupAddress {
    fn send(&mut self, msg: String) -> () {
        self.sender.send(ActorMessage::Msg {
            actor_id: self.actor_id.clone(),
            msg: msg
        }).unwrap();
    }

    fn stop(self) -> () {
        self.sender.send(ActorMessage::Stop {
            actor_id: self.actor_id.clone()
        }).unwrap();
    }
}
