import { useEffect, useState } from "react";
import NavItem from "./NavItem";
import { FaDownload, FaUpload } from "react-icons/fa6";
import { TbPlugConnected } from "react-icons/tb";
import { FaStop } from "react-icons/fa";
import { IoMdSettings } from "react-icons/io";
import { VscDebugStart } from "react-icons/vsc";
import { invoke } from "@tauri-apps/api/core";
import { useLocation } from "react-router-dom";
function Nav() {
  // const invoke = window.__TAURI__.invoke
  const location = useLocation();
  const [isServerRunning, setIsServerRunning] = useState(false);
  async function startServer() {
    let response = await invoke("start_server");
    alert(response);
    setIsServerRunning(true);
  }
  async function stopServer() {
    await invoke("stop_server");
    setIsServerRunning(false);
  }
  useEffect(() => {
    console.log("isServerRunning:", isServerRunning);
  }, [isServerRunning]);
  return (
    <nav className="fixed bottom-0 left-0 flex w-full min-w-20 justify-between bg-[#ffffff] text-white shadow-[rgba(0,0,15,0.1)_0px_-1px_10px_1px] sm:relative sm:h-screen sm:w-fit sm:flex-col sm:shadow-[rgba(0,0,15,0.1)_1px_-4px_4px_1px]">
      {/* Navigation Links */}
      <div className="mx-auto flex size-fit w-full justify-around sm:mb-16 sm:max-w-fit sm:flex-col sm:justify-start">
        {isServerRunning ? (
          <div onClick={stopServer}>
            <NavItem item={<FaStop />} text={"stop server"} />
          </div>
        ) : (
          <div onClick={startServer}>
            <NavItem item={<VscDebugStart />} text={"start server"} />
          </div>
        )}

        <NavItem
          path="/"
          item={<TbPlugConnected />}
          text={"connect"}
          isActive={location.pathname === "/"}
        />

        <NavItem
          path="/downloads"
          item={<FaDownload />}
          text={"downloads"}
          isActive={location.pathname === "/downloads"}
        />

        <NavItem
          path="/uploads"
          item={<FaUpload />}
          text={"uploads"}
          isActive={location.pathname === "/uploads"}
        />

        <NavItem
          path="/settings"
          item={<IoMdSettings />}
          text={"settings"}
          isActive={location.pathname === "/settings"}
        />
      </div>
    </nav>
  );
}

export default Nav;
