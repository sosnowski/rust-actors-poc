// use std::collections::{ HashMap };
// use std::sync::{ mpsc };
// use std::thread;
// use super::actor::{ Actor, ActorMessage };
// use super::context::Context;

// pub struct Manager {
//     senders: HashMap<String, mpsc::Sender<ActorMessage>>,
//     handlers: Vec<Option<thread::JoinHandle<()>>>,
//     my_sender: mpsc::Sender<ManagerMessage>
// }

// impl Manager {

//     pub fn new(sender: mpsc::Sender<ManagerMessage>) -> Manager {
//         return Manager {
//             senders: HashMap::new(),
//             handlers: vec![],
//             my_sender: sender
//         };
//     }

//     pub fn spawn_actor_thread(&mut self, actor_id: String, actor: Box<dyn Actor>) -> () {
//         let (sender, receiver) = mpsc::channel::<ActorMessage>();
//         let thread_id = format!("thread_{}", actor_id);
//         let manager_sender = self.my_sender.clone();
//         let actor_sender = sender.clone();
//         let thread_id_copy = thread_id.clone();
//         let actor_id_copy = actor_id.clone();

//         let handle = thread::spawn(move || {
//             let mut actors: HashMap<String, Box<dyn Actor>> = HashMap::new();
//             println!("Starting an actor thread");
//             loop {
//                 let msg = receiver.recv().unwrap();
//                 match msg {
//                     ActorMessage::NewActor { actor_id, mut actor } => {
//                         println!("Actor thread <{}> has received NewActor Message for {}", thread_id, actor_id);
//                         let ctx = Context::new(
//                             actor_id.clone(),
//                             &sender,
//                             &manager_sender
//                         );
//                         actor.started(ctx);
//                         actors.insert(actor_id.clone(), actor);
//                     },
//                     ActorMessage::Msg { actor_id, msg } => {
//                         println!("Actor thread <{}> has received Msg for {}, value: {}", thread_id, actor_id, msg);
//                         if let Some(actor) = actors.get_mut(&actor_id) {
//                             println!("Found actor {}, calling handler...", actor_id);
//                             let ctx = Context::new(
//                                 actor_id,
//                                 &sender,
//                                 &manager_sender
//                             );
//                             actor.handle(msg, ctx);
//                         }
//                     }
//                 }
//             }
//         });


//         actor_sender.send(ActorMessage::NewActor {
//             actor_id: actor_id_copy,
//             actor: actor
//         }).unwrap();

//         self.handlers.push(Some(handle));
//         self.senders.insert(thread_id_copy, actor_sender);
//     }

//     pub fn route_actor_message(&self, actor_id: String, msg: String) -> () {
//         let thread_id = format!("thread_{}", actor_id);
//         if let Some(actor_sender) = self.senders.get(&thread_id) {
//             actor_sender.send(ActorMessage::Msg {
//                 actor_id,
//                 msg
//             }).unwrap();
//         }
//     }

//     pub fn stop_actor_thread(&self, actor_id: String) -> () {
        
//     }

//     pub fn stop(&mut self) -> () {
//         for handle in self.handlers.iter_mut() {
//             if let Some(handle) = handle.take() {
//                 handle.join().unwrap();
//             }
//         }
//     }
// }

// pub enum ManagerMessage {
//     SpawnActor {
//         actor_id: String,
//         actor: Box<dyn Actor>
//     },
//     SendMessage {
//         actor_id: String,
//         msg: String
//     },
//     StopActor {
//         actor_id: String
//     },
//     StopAll
// }