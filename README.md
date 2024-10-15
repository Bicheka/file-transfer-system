# File transfer system (FTS)
![alt text](fts.png)
A file transfer system library that provides functionality to send files of any size through TCP

## Provides functionality for:
- File transfering and folder recursive transfering
- NAT traversal for p2p connection and IPV6
- Functionality to get current device private and public IP address
- Uses multithreaded tokio runtime to asynchronously send files and process file requests


If you want to build the app you can cd into /fileflow and then follow the README instructions to run in it in dev mode or to compile the executable

## Project architecture

***This project is in a stage where it is still defining its structure so it might be subject of major changes***
```
./ src--------- // core library, where most of the code for the lives
  |-lib  
    -    client
    -    file_transfer
    -    graceful_shutdown
    -    network
    -    p2p
    -    server
```
### Testing
test all
```
cargo test
```

To run a test that is inside a feature
```
cargo test --feature feature-name module-name::tests::test-function-name
```
## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
