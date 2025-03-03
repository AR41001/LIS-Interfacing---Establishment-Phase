STEPS TO RUN THE CODE:

    Go to the file location and open terminal
    Run the command on terminal " cargo build ". This will compile libraries and create executable files
    Run the command on terminal " cargo run "
    I tested the code on Raspberry Pi Zero 2w

LIS Interfacing - Establishment Phase

This code implements the establishment phase for Laboratory Information System (LIS) interfacing according to the data link layer protocol. The data link layer is one of the 4 layers which are part of the LIS Protocol. This layer is at the lowest level. The establishment phase is the initial handshake between a client and a machine to establish a connection.

Features:

    The user can choose to act as a client and receive ENQ (Option 1) or transmit ENQ to initiate the connection (Option 2).
    The handshake is performed using ENQ, ACK, and NAK.
    A timeout and retry mechanism is implemented to handle communication errors.
    Contention detection and avoidance is implemented to prevent multiple devices from attempting to connect simultaneously

How to Use:

    Compile the code using a Rust compiler.
    Run the compiled binary.
    The program will prompt you to choose between receiving or transmitting ENQ.
    Enter "1" to receive ENQ (client mode) or "2" to transmit ENQ (initiator mode).
    Follow the on-screen messages for further instructions.

Note:

    This code is for demonstration purposes only and may require further development for specific LIS implementations.
    The serial port path (/dev/ttyS0) might need to be adjusted based on your system configuration.

Code Breakdown:

    The code defines special character constants for ENQ, ACK, and NAK.
    It opens the serial port for communication.
    The main loop allows the user to choose the operation mode (receive or transmit).
    The receive_enq function waits for an ENQ from the sender and responds with ACK.
    The transmit_enq function repeatedly sends ENQ until an ACK is received or a timeout occurs. It also implements contention detection and handling.

Further Development:

    Integrate this code with higher-level LIS communication protocols.
    Implement error handling for unexpected data or communication failures.

I hope this readme provides a clear explanation of the code and its functionality.
Added this line to practice Git Commits
