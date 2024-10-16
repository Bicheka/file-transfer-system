# File transfer system (FTS)
![alt text](fts.png)
A file transfer system library that provides functionality to send files of any size through TCP

## Provides functionality for:
- File transfering and folder recursive transfering
- NAT traversal for p2p connection and IPV6
- Functionality to get current device private and public IP address
- Uses multithreaded tokio runtime to asynchronously send files and process file requests


Check Fileflow which is a tauri app for sending files that implements this library. [Fileflow.](https://github.com/Bicheka/fileflow)

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
cargo test --all-features
```
## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
