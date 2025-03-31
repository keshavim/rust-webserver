# rust-webserver

This is a simple rust webserver and database made to learn rust. 

# how download

Clone the project with
`git clone`
or download the zip

then cd into the main workspace

# how to run server

to run server

`cargo run` or `cargo run -p server`
by default server will use the ip address 127.0.0.1:8080
to uses a different ip address
`cargo run -- [address]` or `cargo run -p server [address]`

then open a brower and load a website you have added to the local database

# how to use database

to run the database

`cargo run -p database [options]`

options include:
add path, remove path, clear, and help

add and remove can include multiple paths at once.

path is a directory that contains html files and a urls.txt.
urls.txt should contain the url on the left side and the path to html file on the right with whitespace to seperate.
look at the example folder

