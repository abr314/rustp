
pub mod ab_grand_central_dispatch {

    pub struct ABGrandCentralDispatch {
        open_dispatch_queues:Vec<ABDispatchQueue>,
    }

    impl ABGrandCentralDispatch {
        pub fn new()->ABGrandCentralDispatch {
            ABGrandCentralDispatch {
                open_dispatch_queues:Vec::new(),
            }
        }
        fn push_queue_to_gcd(mut self, queue:ABDispatchQueue) {
            self.open_dispatch_queues.push(queue)
        }

        fn pop_queue_from_gcd(mut self) -> ABDispatchQueue{
            self.open_dispatch_queues.pop().unwrap() as ABDispatchQueue
        }

        fn receive_message(message:ABGCDMessage) {

        }

        fn send_message_to_owned_queue(message:ABGCDMessage) {

        }

        fn setup_queues() {
            // tick queue,
            // order queue,
            // account queue,

        }


    }

    struct ABDispatchQueue {

    }
    pub struct ABGCDMessage {
        id:String,
        message:String,
        value:i32,
    }



}