# Cache-Server
This is a simple cache server in Rust. 

Steps to run:
1. Clone or download the project.
2. Run "cargo build" to build the project.
3. Run "cargo run" to run it.

Testing:
You can test it with postman or CURL.
Get cache data: GET 127.0.0.1:5000/get/{key}
Set cache data: POST 127.0.0.1:5000/set/{key}/{value}
