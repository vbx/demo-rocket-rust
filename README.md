# Rocket.rs Application Example with Modular Features

This project is an example of an application using the **Rocket.rs** framework in Rust.  
It showcases how to structure a binary in a modular way by leveraging Rust's **features**.  

Using **features**, you can enable or disable specific routes and functionalities at compile time,  
allowing you to tailor the application behavior to meet specific requirements.

Example: 
```shell
cargo run --no-default-features --features project_b
cargo run --no-default-features --features except_project_b
```