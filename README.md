The jumping button is a Rust application built using the FLTK library. It is a GUI that displays a button which jumps to a random position when clicked.
To run the jumping button application, you have two options: building and running it one time, or using hot module replacement (HMR) with Cargo Watch for development.

## Building and Running the Application

1. Open your terminal and navigate to the project directory where the `Cargo.toml` file is located.

2. Run the following command to build the application:
  ```
  cargo build
  ```

3. Once the build is complete, run the application with:
  ```
  cargo run
  ```

This will compile and run the application one time. You can interact with the jumping button application and observe its behavior as you click the button.

## Developing with Hot Module Replacement (HMR)

To develop the jumping button application with HMR using Cargo Watch, follow these steps:

1. Make sure you have Cargo Watch installed. If not, you can install it by running `cargo install cargo-watch` in your terminal.

2. Open your terminal and navigate to the project directory where the `Cargo.toml` file is located.

3. Run the following command to start the application with HMR:
  ```
  cargo watch -x run
  ```

Cargo Watch will automatically rebuild and restart the application whenever changes are detected in the source code. This allows you to see the changes in real-time without manually restarting the application.

That's it! You can now interact with the jumping button application and observe its behavior as you click the button. Enjoy!