extern crate untitled as u;

use u::networking as n;

fn main() {
    println!("Hello, world!");
   // let l = n::ab_grand_central_dispatch::ABGrandCentralDispatch;

}

//extern crate chrono;




mod tick_manager {

    pub struct ABTickManager {
        count:i32,
    }



    pub fn add_one(num:&mut i32) {
        *num +=1;
    }
    /// Use to create new ticks and send to GCD
    impl ABTickManager {
        pub fn new() -> ABTickManager {
            ABTickManager {
                count:0,
            }
        }

        pub fn create_new_tick(&mut self) -> ABTick {
            add_one(& mut self.count);
            ABTick {
                open:(0.),
                close: (0.),
                high: (0.),
                low: (0.),
                ask_volume: (0),
                bid_volume: (0),
                open_interest: (0),
                //time_stamp: (),
                tick_id:(self.count),
                product_id: (0),
            }
        }
    }

    pub struct ABTick {
        open: f32,
        close: f32,
        high: f32,
        low: f32,

        ask_volume: i32,
        bid_volume: i32,
        open_interest: i32,

        product_id: i32,
        tick_id:i32
    }


    pub struct ABTickFrame {
        position:i32,
        //tick_vector:[ABTick],
        count:i32,
    }
}