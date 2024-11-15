# File transfer system (FTS)
A file transfer system library that provides functionality to send files of any size through TCP

## Provides functionality for:
- File transfering and folder recursive transfering
- NAT traversal for p2p connection and IPV6
- Functionality to get current device private and public IP address
- Uses multithreaded tokio runtime to asynchronously send files and process file requests

## Contribution
Anyone that wants to contribute is more than welcomed. 
Also, feel free to leave an issue if you want some feature or encounter an error or if you want to give feedback so I know what needs more work, it is really appreciated. [FTS repo](https://github.com/Bicheka/file-transfer-system)

Check Fileflow which is a tauri app for sending files that implements this library. [Fileflow.](https://github.com/Bicheka/fileflow)

## Project architecture

***This project is in a stage where it is still defining its structure so it might be subject of major changes***
```
./ src--------- // core library, where most of the code for the lives
  |-lib  
    -    client
    -    compression
    -    file_transfer
    -    graceful_shutdown
    -    network
    -    p2p
    -    server
```
## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
