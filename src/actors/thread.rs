use std::sync::{ mpsc };
use std::thread;
use std::collections::HashMap;
use crossbeam_channel::{ unbounded, Sender, Receiver };
use super::actor::{ ActorMessage, Actor };
use super::context::Context;

pub fn spawn_actor_thread<'a, T: Actor + 'static>(thread_actor_id: String, actor: T, sender: Sender<ActorMessage>, receiver: Receiver<ActorMessage>) -> thread::JoinHandle<()> {
    let local_actor_id = thread_actor_id.clone();

    sender.send(ActorMessage::NewActor {
        actor_id: thread_actor_id,
        actor: Box::new(actor)
    }).unwrap();

    //po co tutaj jest move? A może bez move? Wtedy będzie po referrencji leciał? możę wtedy nie trzeba kopii?
    let handle = thread::spawn(move || {
        println!("Sprawning Actor's thread for actor: {}", local_actor_id);

        let mut actors: HashMap<String, Box<dyn Actor>> = HashMap::new();

        loop {
            let msg = receiver.recv().unwrap();
            match msg {
                ActorMessage::NewActor { actor_id, mut actor } => {
                    println!("Actor thread <{}> has recceived New Actor: {}", local_actor_id, actor_id);
                    let ctx = Context::new(
                        actor_id.clone(),
                        &sender
                    );
                    actor.on_start(ctx);
                    actors.insert(actor_id.clone(), actor);
                },
                ActorMessage::Msg { actor_id, msg } => {
                    println!("Actor thread <{}> has received Msg for {}, value: {}", local_actor_id, actor_id, msg);
                    if let Some(actor) = actors.get_mut(&actor_id) {
                        println!("Found actor {}, calling handler...", actor_id);
                        let ctx = Context::new(
                            actor_id,
                            &sender
                        );
                        actor.handle(msg, ctx);
                    }
                },
                ActorMessage::Stop { actor_id } => {
                    println!("Actor thread <{}> Received stop for {}", local_actor_id, actor_id);
                    if let Some(mut actor) = actors.remove(&actor_id) {
                        println!("Found actor {}, stopping the actor", actor_id);
                        let ctx = Context::new(
                            actor_id.clone(),
                            &sender
                        );
                        actor.on_stop(ctx);
                        if actors.len() == 0 {
                            println!("Actor thread <{}> has no more actors, stopping the thread", local_actor_id);
                            break;
                        }
                    }
                }
            }
        }
        println!("Actor thread <{}> finished", local_actor_id);
    });

    return handle;
}
