import Nav from "./components/Nav";
import { BrowserRouter as Router } from "react-router-dom";
import MainContent from "./components/MainContent"
function App() {
  return (
    <Router>
      <div className="flex flex-col sm:flex-row min-h-screen">
        <Nav/>
        <MainContent />
      </div>
    </Router>
  );
}

export default App
