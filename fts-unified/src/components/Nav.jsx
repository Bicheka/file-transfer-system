import NavItem from "./NavItem";
import { FaDownload, FaUpload } from "react-icons/fa6";
import { IoMdSettings } from "react-icons/io";
function Nav(){

    return(
    <nav className="fixed bottom-0 left-0 w-full min-w-20 sm:w-fit bg-slate-800 text-white flex justify-between sm:relative sm:flex-col sm:h-screen">
      {/* Logo or Title */}
      <div className="hidden sm:flex items-center justify-center p-4">
        <span className="text-xl font-bold">FTS</span>
      </div>

      {/* Navigation Links */}
      <div className="flex sm:flex-col justify-around mx-auto sm:mb-16 w-full sm:max-w-fit sm:justify-start size-fit">
          <NavItem path="/" item={<FaDownload/>}/>
        
          <NavItem path="/uploads" item={<FaUpload/>}/>
        
          <NavItem path="/settings" item={<IoMdSettings/>}/>
        
      </div>
    </nav>
    );
}

export default Nav;