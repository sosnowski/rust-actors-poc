mod actors;

use std::thread::sleep;
use std::time::Duration;
use actors::system::System;
use actors::actor::{ Actor, Address };
use actors::context::Context;

#[derive(Debug)]
struct MainActor {
    received_messages: usize,
}

impl Actor for MainActor {
    fn handle(&mut self, msg: String, _ctx: Context) -> () {
        self.received_messages += 1;
        println!("Main Actor has received his {} message: {}", self.received_messages, msg);
    }

    fn on_start(&mut self, ctx: Context) -> () {
        // let mut local_child = ctx.start_with_id(ChildActor {
        //     received_messages: 0
        // }, "local_child_actor".to_owned());

        // let mut remote_child = ctx.spawn_with_id(ChildActor {
        //     received_messages: 0
        // }, "remote_child_actor".to_owned());

        // local_child.send("Hi Baby! It's your papa".to_owned());
        // remote_child.send("Hello from papa to remote!".to_owned());

        // sleep(Duration::from_secs(3));
        // println!("Stopping actors");

        // local_child.stop();
        // remote_child.stop();

        let mut group = ctx.spawn_group_with_id(ChildActor {
            received_messages: 0
        }, 5, "child_actor_group".to_owned());

        group.send("Message to group 0".to_owned());
        group.send("Message to group 1".to_owned());
        group.send("Message to group 2".to_owned());
        group.send("Message to group 3".to_owned());
        group.send("Message to group 4".to_owned());
    }
}

#[derive(Debug, Clone)]
struct ChildActor {
    received_messages: usize,
}

impl Actor for ChildActor {
    fn handle(&mut self, msg: String, _ctx: Context) -> () {
        self.received_messages += 1;
        println!("Child Actor has received his {} message {}", self.received_messages, msg);
    }
}


fn main() {
    println!("Hello, world!");

    let mut sys = System::start(MainActor {
        received_messages: 0
    });

    sys.stop();
}
