# File transport system (FTS)
The goal for this project is to allow anyone to be able to send any file/s of large size directly to any other device securely and blazingly fast.

![alt text](fts.png)

### File Structure
```
./
--src
  |
  --main.rs
    //server is going to contain logic to send files/data 
  --sender.rs        
  --sender/
    |
    --
    //reciever is going to expose an api so other devices can connect to it and send files
  --reciever.rs  
  --reciever/       
    |
    --
```

<p>
    Sender is going to be like the Client that connects to an API reciever
    if the reciever acepts the request then the client can send the file/s
</p>