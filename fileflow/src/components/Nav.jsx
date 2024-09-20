import { useEffect, useState } from "react";
import NavItem from "./NavItem";
import { FaDownload, FaUpload } from "react-icons/fa6";
import { TbPlugConnected } from "react-icons/tb";
import { FaStop } from "react-icons/fa";
import { IoMdSettings } from "react-icons/io";
import { VscDebugStart } from "react-icons/vsc";
import { invoke } from "@tauri-apps/api/core";

function Nav(){

    // const invoke = window.__TAURI__.invoke
    
    const [isServerRunning, setIsServerRunning] = useState(false);
    async function startServer() {
      let response = await invoke('start_server');
      alert(response);
      setIsServerRunning(true);
    }
    async function stopServer(){
      await invoke('stop_server');
      setIsServerRunning(false);
    }
    useEffect(() => {
        console.log('isServerRunning:', isServerRunning);
    }, [isServerRunning]);
    return(
    <nav className="fixed bottom-0 left-0 w-full shadow-[rgba(0,0,15,0.1)_0px_-1px_10px_1px] sm:shadow-[rgba(0,0,15,0.1)_1px_-4px_4px_1px] min-w-20 sm:w-fit bg-[#ffffff]  text-white flex justify-between  sm:relative sm:flex-col sm:h-screen">
      {/* Navigation Links */}
      <div className="flex sm:flex-col justify-around mx-auto sm:mb-16 w-full sm:max-w-fit sm:justify-start size-fit">
          { isServerRunning 
            ?<div onClick={stopServer}><NavItem item={<FaStop/>}  text={"stop server"}/></div>
            :<div onClick={startServer}><NavItem item={<VscDebugStart/>}  text={"start server"}/></div>
          }
          
          <NavItem path="/" item={<TbPlugConnected/>} text={"connect"}/>

          <NavItem path="/downloads" item={<FaDownload/>} text={"downloads"}/>
        
          <NavItem path="/uploads" item={<FaUpload/>} text={"uploads"}/>
        
          <NavItem path="/settings" item={<IoMdSettings/>} text={"settings"}/>
        
      </div>
    </nav>
    );
}

export default Nav;