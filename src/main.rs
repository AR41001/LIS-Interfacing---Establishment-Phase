//Below is the code for the establishment phase which is the first phase in the 
//data link layer for LIS interfacing

//In this phase we basically just build a connection between the client and the machine
//by the basic hand shaking method. Sending and receiving ACK etc etc.

//The establishment phase can be initialized from either the client or the machine
//To implement that functionality we have given the option of choosing whether to transmit (initiate the establishment) or receive


extern crate serialport;
// use serialport::SerialPort;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::io;

fn main() {
    // Special characters
    // In the establishment phase we IGNORE all other special characters
    const ENQ: &[u8] = &[0x05];  // Enquiry
    const ACK: &[u8] = &[0x06];  // Acknowledge
    const NAK: &[u8] = &[0x15];  // Not Acknowledged

    // Open the serial port
    let mut port = serialport::new("/dev/ttyS0", 9600)
        .timeout(Duration::from_secs(5))
        .open()
        .expect("Failed to open port");

    // Flags for flow control
    let mut is_connection_established = false;
    let mut _last_sent_time = Instant::now();

    // Timing constants
    const ENQ_INTERVAL: u64 = 30;  // Interval for sending ENQ (30 seconds)
    const TIMEOUT: u64 = 15;       // Timeout for response (15 seconds)
    const RETRY_INTERVAL: u64 = 10;  // Interval for retrying (10 seconds)

    // Main loop
    loop {
        println!("Welcome to the CLIENT side, do you want to transmit or receieve ENQ");
        println!("Choose an option:");
        println!("1. Receive ENQ");
        println!("2. Transmit ENQ");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter 1 or 2.");
                continue;
            }
        };

        match choice {
            1 => {
                // Wait for the sender to send [ENQ]
                let mut client_buffer = [0u8; 1];
                while port.read(&mut client_buffer).is_err() || client_buffer != ENQ {
                    // Continue reading until [ENQ] is received
                }

                if client_buffer == ENQ {
                    // Sender sent [ENQ], respond with [ACK]
                    port.write_all(ACK).expect("Failed to write [ACK]"); //Here we can choose to send ACK or NAK but for testing purposes we will respond with ACK
                    port.flush().expect("Failed to flush");
                    println!("Sent [ACK].");
                    }
                // Wait for a moment before returning to listening for [ENQ]
                sleep(Duration::from_secs(1));
            }
            2 => {
                // Here we are initiating the establishment phase by sending an ENQ to the machine
                let mut _last_sent_enq_time = Instant::now();
                let mut contention_reached = false;

                while !is_connection_established {
                    let current_time = Instant::now();

                    // Check if it's time to send ENQ again
                    if current_time.duration_since(_last_sent_enq_time).as_secs() >= ENQ_INTERVAL {
                        // Send ENQ
                        port.write_all(ENQ).expect("Failed to write [ENQ]");
                        port.flush().expect("Failed to flush");
                        _last_sent_enq_time = current_time;
                    }

                    // Check if a response is received, below are the conditions for all the responses possible in this phase. 
                    if port.bytes_to_read().expect("Failed to get bytes to read") > 0 {
                        let mut client_response = [0u8; 1];
                        port.read_exact(&mut client_response).expect("Failed to read response");

                        if client_response == ACK {
                            println!("ACK received. Establishment phase complete.");
                            is_connection_established = true;
                        } else if client_response == NAK {
                            println!("NAK received. Retrying in 10 seconds.");
                            sleep(Duration::from_secs(RETRY_INTERVAL));
                        } else if client_response == ENQ {
                            println!("Reached contention.");
                            contention_reached = true;
                            break;
                        }
                    }

                    // If no response received within TIMEOUT, go to termination phase
                    if !is_connection_established
                        && current_time.duration_since(_last_sent_enq_time).as_secs() >= TIMEOUT
                    {
                        println!("No response received. Termination phase.");
                        sleep(Duration::from_secs(30));  // Wait for 30 seconds in termination phase
                    }

                    // Implement further logic here based on the established connection
                    // For example, send data or perform other actions.
                }

                if !contention_reached {
                    break;
                }
            }
            _ => {
                println!("Invalid option, please enter 1 or 2.");
            }
        }
    }
}
