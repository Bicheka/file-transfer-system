# File transfer system (FTS)
The goal for this project is to allow anyone to be able to send any file/s of large size directly to any other device securely and blazingly fast.

![alt text](fts.png)

<p>
    Server is going to be like the Client that connects to an API reciever
    if the reciever acepts the request then the client can send the file/s
</p>
** This project is in a stage where it is still defining its structure so it might be subject of major changes**

## Project architecture

### Workspace
```
./ fts-unified------ // combines functionality for server and client into one app
./ client----------- // is going to create the binary app for a client only
./ server----------- // in charge of crating a server app only
./ launcher--------- // allows users to launche server and client apps as 2 separate apps and manage them
./ core_lib--------- // where most of the code for the rest of the crates live the heart of the project
    | server.rs ------ // module with all the code for server binary
    | client.rs ------ // module with all the code for client binary
```
### Testing
test all
```
cargo test
```

To run a test that is inside a feature
```
cargo test -p package-name --feature feature-name module-name::tests::test-function-name
```
