use std::sync::mpsc;
use crossbeam_channel::{ Sender, unbounded };
use super::actor::{ Actor, ActorMessage, LocalAddress, RemoteAddress, GroupAddress };
use super::helpers::{ uuid };
use super::thread::{ spawn_actor_thread };

pub struct Context<'a> {
    actor_id: String,
    local_sender: &'a Sender<ActorMessage>
}

impl <'a> Context<'a> {
    pub fn new(actor_id: String, local_sender: &'a Sender<ActorMessage>) -> Context<'a> {
        return Context {
            actor_id,
            local_sender
        };
    }

    pub fn start<T: Actor + 'static>(&self, actor: T) -> LocalAddress {
        return self.start_with_id(actor, uuid());
    }

    pub fn start_with_id<T: Actor + 'static>(&self, actor: T, actor_id: String) -> LocalAddress {
        self.local_sender.send(ActorMessage::NewActor {
            actor_id: actor_id.clone(),
            actor: Box::new(actor)
        }).unwrap();

        return LocalAddress::new(actor_id, self.local_sender);
    }

    pub fn spawn<T: Actor + 'static>(&self, actor: T) -> RemoteAddress {
        return self.spawn_with_id(actor, uuid());
    }

    pub fn spawn_with_id<T: Actor + 'static>(&self, actor: T, actor_id: String) -> RemoteAddress {
        let (sender, receiver) = unbounded::<ActorMessage>();
        spawn_actor_thread(actor_id.clone(), actor, sender.clone(), receiver);
        return RemoteAddress::new(actor_id, sender);
    }

    pub fn spawn_group<T: Actor + Clone + 'static>(&self, actor: T, number_of_threads: usize) -> GroupAddress {
        return self.spawn_group_with_id(actor, number_of_threads, uuid());
    }

    pub fn spawn_group_with_id<T: Actor + Clone + 'static>(&self, actor: T, number_of_threads: usize, actors_id: String) -> GroupAddress {
        let (sender, receiver) = unbounded();
        for i in 0..number_of_threads {
            spawn_actor_thread(actors_id.clone(), actor.clone(), sender.clone(), receiver.clone());
        }

        return GroupAddress::new(actors_id, sender);

        // gdy nowy thread, wysyła sam sobie aktora, to tak naprawdę wysyła go do wszystkich z grupy :/ Tak nie powinno to działać
    }
}
