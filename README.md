# File transfer system (FTS)
The goal for this project is to allow anyone to be able to send any file/s of large size directly to any other device securely and blazingly fast.

![alt text](fts.png)

<p>
    Server is going to be like the Client that connects to an API reciever
    if the reciever acepts the request then the client can send the file/s
</p>
This project is in a stage where it is still defining its structure so it might be subject of major changes

If you want to build the app you can cd into /fileflow and then follow the README instructions to run in it in dev mode or to compile the executable

## Project architecture

This project repository is basically a cargo workspace with multiples crates
```
./ fileflow------ // combines functionality for server and client into one app using tauri which allows it to build for all mobiles and desktop
./ fts--------- // core library, where most of the code for the lives
    |-server
        |-api
        |-admin
    |-client
    |-p2p
    |-network
    |-file_transfer
    |-graceful_shutdown
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
## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
