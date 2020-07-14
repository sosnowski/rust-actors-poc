// use std::collections::{ VecDeque, HashMap };
// use std::sync::{ mpsc, Arc, Mutex };
// use std::thread;
// use std::rc::Rc;
// use rand::{ Rng, thread_rng };
// use rand::distributions::Alphanumeric;

// /**
//  * Tworzony aktor zostaje przypisany do konkretnego workera, 
//  * a adres dostaje tylko channel do wątku workera i id agenta. W ogóle nie widzie instancji agenta. Agenci zostają z workerem.
//  * Thread workera nasłuchuje na nowych wiadomosciach z channela i wtedy odpala odpowiedniego agenta
//  * wiadomosc zawiera id aktora i sam message
//  * 
//  * Jak rozdzielić dodawanie aktora do obecnego workera vs do nowego?
//  * W naszym wypadku to będzie wiele senderów (adresy) i jeden receiver (wątek).
//  * 
//  * W sumie, funkcjonalność "spawnGroup" można osiągnać poprzez wiele senderów (adresy) vs wiele receiverów (agenci na róznych wątkach).
//  * Wtedy z Mutexem (jak w artykule), który pierwszy zgadnie ten wygrywa. A msg idzie do wszystkich potencjalnie.
//  * ( https://doc.rust-lang.org/book/ch20-02-multithreaded.html)
//  * 
//  * Cos w rodzaju
//  * 
//  * let addr = AgentsManager.start();
//  * i w srodku, tworzona jest kolekcja workerow
//  * agent jest przypisywany do workera
//  * addr dostaje id i sendera do wątku
//  * 
//  * System
//  *  - Manager
//  *  - Manager
//  *     - MyActo
//  *     - MyActo
//  *       - Runner
//  */

// fn uuid() -> String {
//     return thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(20)
//         .collect();
// }

// trait Actor: Send {
//     fn receive_message(&mut self, message: String, context: Context) -> ();
// }

// type ActorId = String;

// // struct ActorRunner<'a> {
// //     actor: Box<dyn Actor>,
// //     id: ActorId,
// //     thread_id: String,
// //     mailbox: VecDeque<String>,
// //     dispatcher: Arc<Mutex<&'a Dispatcher>>,
// //     executing: bool
// // }

// // impl<'a> ActorRunner<'a> {
// //     fn new<'b>(actor: Box<dyn Actor>, thread_id: String, dispatcher: Arc<Mutex<&'b Dispatcher>>) -> ActorRunner<'b> {
// //         let id = uuid();
// //         return ActorRunner {
// //             actor: actor,
// //             id: id,
// //             thread_id: thread_id,
// //             mailbox: VecDeque::new(),
// //             dispatcher: dispatcher,
// //             executing: false
// //         };
// //     }

// //     fn context(&'a self) -> Context<'a, 'a> {
// //         return Context {
// //             runner: &self
// //         }
// //     }

// //     fn receive_message(&mut self, message: String) -> () {
// //         self.mailbox.push_back(message);
// //         if !self.executing {
// //             self.start_execution();
// //         }
// //     }

// //     fn start_execution(&mut self) -> () {
// //         self.executing = true;
// //         while let Some(message) = self.mailbox.pop_front() {
// //             self.actor.handle(message, self.context());
// //         }
// //         self.executing = false;
// //     }
// // }

// // struct Context<'a, 'b> {
// //     runner: &'a ActorRunner<'b>
// // }

// // impl<'a> Context<'a> {

// //     fn start_actor<T: Actor + 'static>(&self, actor: T) -> Address {
// //         //start actor, get currrent thread_id, generate actor_id, dispatch new actor action
// //         let actor_id = uuid();
// //         let dispatcher = self.runner.dispatcher.lock().unwrap();
// //         dispatcher.dispatch(self.runner.thread_id.clone(), Message::NewActor {
// //             actor_id,
// //             actor: Box::new(actor)
// //         });

// //         return Address {
// //             actor_id,
// //             thread_id: self.runner.thread_id.clone(),
// //             dispatcher: self.runner.dispatcher
// //         };
// //     }

// //     fn spawn_actor<T: Actor + 'static>(&self, actor: T) -> Address {
// //         let actor_id = uuid();

// //         let dispatcher = self.runner.dispatcher.lock().unwrap();
// //         let thread_id = dispatcher.dispatch_to_next(Message::NewActor {
// //             actor_id,
// //             actor: Box::new(actor)
// //         }).unwrap();

// //         return Address {
// //             actor_id,
// //             thread_id: self.runner.thread_id.clone(),
// //             dispatcher: self.runner.dispatcher
// //         };
// //     }
// // }

// // struct Address {
// //     actor_id: ActorId,
// //     thread_id: String,
// //     dispatcher: Arc<Mutex<Dispatcher>>
// // }

// // impl Address {
// //     fn send(&self, message: String) -> () {
// //         let dispatcher = self.dispatcher.lock().unwrap();
// //         dispatcher.dispatch(self.thread_id.clone(), Message::Task {
// //             actor_id: self.actor_id.clone(),
// //             message
// //         });
// //     }
// // }

// // struct Dispatcher {
// //     senders: HashMap<String, mpsc::Sender<Message>>,
// //     keys: Option<Vec<String>>,
// //     next_sender: usize
// // }

// // impl Dispatcher {
// //     fn new() -> Dispatcher {
// //         return Dispatcher {
// //             senders: HashMap::new(),
// //             keys: None,
// //             next_sender: 0
// //         };
// //     }

// //     fn init(&self) {
// //         self.keys = Some(self.senders.keys().map(|&str| str.clone()).collect());
// //     }

// //     fn add(&mut self, thread_id: String, sender: mpsc::Sender<Message>) -> () {
// //         self.senders.insert(thread_id, sender);
// //     }

// //     fn dispatch(&self, thread_id: String, message: Message) -> () {
// //         if let Some(sender) = self.senders.get(&thread_id) {
// //             sender.send(message);
// //         }
// //     }

// //     fn dispatch_to_next(&mut self, message: Message) -> Result<String, String> {
// //         if let Some(keys) = self.keys {
// //             if let Some(thread_id) = keys.get(self.next_sender) {
// //                 self.dispatch(thread_id.clone(), message);
// //                 self.next_sender = self.next_sender + 1;
// //                 if self.next_sender > keys.len() {
// //                     self.next_sender = 0;
// //                 }
// //                 return Ok(thread_id.clone());
// //             }
// //         }
// //         return Err("Error spawning thread".to_owned());
// //     }
// // }

// struct Context<'a> {
//     actor_id: String,
//     sender: mpsc::Sender<Message>,
//     system: Arc<Mutex<&'a System>>
// }

// impl<'a> Context<'a> {
//     fn new(actor_id: String, sender: mpsc::Sender<Message>, system: &mpsc::Sender<SysMessage>) -> Context {
//         return Context {
//             actor_id: actor_id,
//             sender: sender,
//             system: system
//         };
//     }
// }

// struct Worker<T> {
//     thread_handle: thread::JoinHandle<()>,
//     sender: mpsc::Sender<T>
// }

// impl<T> Worker<T> {
//     fn new(id: String, system_sender: mpsc::Sender<SysMessage>) -> Worker<Message> {
//         let (sender, receiver) = mpsc::channel::<Message>();
//         let worker_sender = sender.clone();

//         let handle = thread::spawn(move || { // przekazany System ma referencję do Workersów które mają sendery....
//             println!("Starting worker thread {}", id);
//             let actors: HashMap<ActorId, Box<dyn Actor>> = HashMap::new();
//             loop {
//                 let job = receiver.recv().unwrap();
//                 match job {
//                     Message::NewActor { actor_id, actor } => {
//                         //create runner
//                         // let runner = ActorRunner::new(actor, id, dispatcher);
//                         // println!("Received NewActor message for actor {} in thread {}", runner.id, thread_id);
//                         // runners.insert(runner.id.clone(), runner);
//                     },
//                     Message::Task { actor_id, message } => {
//                         println!("I've just received task {} for actor {} in thread {}", message, actor_id, id);
//                         if let Some(actor) = actors.get_mut(&actor_id) {
//                             let ctx = Context::new(actor_id, sender, &system_sender);
//                             //create context and pass it as argument
//                             actor.receive_message(message, ctx);
//                         }
//                     }
//                 }
//             }
//         });

//         return Worker {
//             thread_handle: handle,
//             sender: worker_sender
//         };
//     }

//     fn new_sys(id: String) -> Worker<SysMessage> {
//         let (sender, receiver) = mpsc::channel::<SysMessage>();
//         let handle = thread::spawn(move || {
//             println!("Starting system worker thread");
//             let senders: Vec<mpsc::Sender<Message>> = vec![];
//             loop {
//                 let task = receiver.recv().unwrap();
//             }
//         });

//         return Worker {
//             thread_handle: handle,
//             sender: sender
//         };
//     }
// }

// struct System {
//     sys_worker: Worker<SysMessage>,
//     workers: Vec<Worker<Message>>,
// }

// impl System {

//     pub fn start<T: Actor + 'static>(main_actor: T, number_of_workers: u32) -> Result<System, String> {
//         if number_of_workers < 1 {
//             return Err("At leat one worker has to be created".to_owned());
//         }
//         let sys_worker = Worker::new_sys("sys".to_owned());
//         let system = System {
//             sys_worker,
//             workers: vec![]
//         };
//     }
// }

// struct Manager {
//     actors: Vec<Box<dyn Actor>>,
//     senders: Vec<mpsc::Sender<Message>>,
//     my_sender: mpsc::Sender<Message>
// }

// impl Manager {
//     fn new(sender: mpsc::Sender<Message>)
// }

// pub type SysMessage = mpsc::Sender<Message>;

// pub enum Message {
//     NewActor {
//         actor_id: ActorId,
//         actor: Box<dyn Actor>
//     },
//     Task {
//         actor_id: ActorId,
//         message: String
//     }
// }