import { Routes, Route } from "react-router-dom";
import Downloads from "./Downloads";
import Uploads from "./Uploads";
import Settings from "./Settings";

const MainContent = () => {
  return (
    <main className="flex-1 p-8 sm:ml-1/5">
      <Routes>
        <Route path="/" element={<Downloads />} />
        <Route path="/uploads" element={<Uploads />} />
        <Route path="/settings" element={<Settings />} />
      </Routes>
    </main>
  );
};

export default MainContent;