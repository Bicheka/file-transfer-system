import { Routes, Route } from "react-router-dom";
import Downloads from "./Downloads";
import Uploads from "./Uploads";
import Settings from "./Settings";
import ServerConnectForm from "./ServerConnectForm";

const MainContent = () => {
  return (
    <main className="flex-1 px-1 pt-6 sm:p-8 bg-slate-200">
      <Routes>
        <Route path="/" element={<ServerConnectForm/>}/>
        <Route path="/downloads" element={<Downloads />} />
        <Route path="/uploads" element={<Uploads />} />
        <Route path="/settings" element={<Settings />} />
      </Routes>
    </main>
  );
};

export default MainContent;