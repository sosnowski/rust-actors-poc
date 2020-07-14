use std::sync::{ mpsc };
use std::thread;
use crossbeam_channel::{ Sender, unbounded };
use super::actor::{ ActorMessage, Actor };
use super::thread::spawn_actor_thread;

pub struct System {
    sys_thread: Option<thread::JoinHandle<()>>,
    sys_sender: Sender<ActorMessage>
}

impl System {
    pub fn start<T: Actor + 'static>(main_actor: T) -> System {
        let (sender, receiver) = unbounded();
        let handle = spawn_actor_thread("main".to_owned(), main_actor, sender.clone(), receiver);

        return System {
            sys_thread: Some(handle),
            sys_sender: sender
        };
    }

    pub fn stop(&mut self) -> () {
        if let Some(handle) = self.sys_thread.take() {
            handle.join().unwrap();
        }
    }
}