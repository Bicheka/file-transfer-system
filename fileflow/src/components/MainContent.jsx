import { Routes, Route } from "react-router-dom";
import Downloads from "./Downloads";
import Uploads from "./Uploads";
import Settings from "./Settings";
import Connection from "./Connection";

const MainContent = () => {
  return (
    <main className="flex-1 bg-slate-200 px-1 pt-6 sm:p-8">
      <Routes>
        <Route path="/" element={<Connection />} />
        <Route path="/downloads" element={<Downloads />} />
        <Route path="/uploads" element={<Uploads />} />
        <Route path="/settings" element={<Settings />} />
      </Routes>
    </main>
  );
};

export default MainContent;
