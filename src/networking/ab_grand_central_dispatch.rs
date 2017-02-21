pub mod ab_grand_central_dispatch {

    pub struct ABGrandCentralDispatch {
        manager_queue:Vec<ABOrderManager>,
    }

    impl ABGrandCentralDispatch {
        pub fn new()->ABGrandCentralDispatch {
            ABGrandCentralDispatch {
                manager_queue:Vec::new(),
            }
        }

        pub fn register_manager(manager:&ABOrderManager) {

        }

        pub fn receive_message(message:ABGCDMessage) {

        }

        fn send_message(message:ABGCDMessage) {

        }

        fn setup_queues() {
            // tick queue,
            // order queue,
            // account queue,

        }


    }

    pub struct ABOrderManager {
        // crea
        orders:Vec<ABOrder>,
        count:i32,
    }

    impl ABOrderManager {

        pub fn new() -> ABOrderManager {
            ABOrderManager {
                count:0,
                orders:Vec::new()
            }
        }

        fn send_message(message:ABGCDMessage) {
            ABGrandCentralDispatch::receive_message(message);
        }
        pub fn receive_message(message:ABGCDMessage) {


        }


        fn process_tick_message(message:ABGCDMessage) {

        }

        fn update_order(&mut self) {

        }
        fn create_new_order(&mut self) {
            // push
        }
        fn close_order(&mut self) {
            // remove specific
        }
        fn close_all_orders(&mut self) {
            //
        }
    }

    struct ABOrder {
        product_id:i32,
        security_type:SecurityType,
        order_id:i32,
        order_type:OrderType,
        order_side:OrderSide,
        order_status:OrderStatus,
        // adjust according to market conditions
        stop_value:Option<f32>,
        limit_value:Option<f32>

    }

    enum SecurityType {
        Future,
        Equity,
        Option,
        Bond,
        Forex,
    }

    enum OrderStatus {
        Created,
        Pending,
        Open,
        Filled,
        Closed,
        Cancelled
    }

    enum OrderType {
        Market,
        Limit
    }

    enum OrderSide {
        Long,
        Short
    }
    struct ABProduct {
        symbol:String,
        full_name:String,
        product_type:String,

    }
    pub struct ABGCDMessage {
        id:String,
        message:String,
        value:i32,
    }





}