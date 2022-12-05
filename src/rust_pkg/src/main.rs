rosrust::rosmsg_include!(rust_pkg/joystick);
use rosrust::{ros_log, ros_fatal};

use crate::rust_pkg::joystick;


fn main() {
    // Initialize node
    rosrust::init("talker");

    // Create publisher
    let chatter_pub = rosrust::publish("chatter", 100).unwrap();

    let mut count = 0;

    // Create object that maintains 10Hz between sleep requests
    let rate = rosrust::rate(10.0);

    // Breaks when a shutdown signal is sent
    while rosrust::is_ok() {
        // Create string message
        // let mut msg = rosrust_msg::std_msgs::String::default();
        let mut msg1 = joystick::default();
        msg1.id = 10;
        msg1.size = 1000;

        ros_fatal!("fatal message");
        // let mut msg = message.constants().
        // msg = format!("hello world {}", count);

        // Send string message to topic via publisher
        chatter_pub.send(msg1).unwrap();

        // Sleep to maintain 10Hz rate
        rate.sleep();

        count += 1;
    }
}