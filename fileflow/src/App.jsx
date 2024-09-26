import Nav from "./components/Nav";
import { BrowserRouter as Router } from "react-router-dom";
import MainContent from "./components/MainContent";
function App() {
  return (
    <Router>
      <div className="flex max-h-full min-h-screen flex-col bg-white sm:flex-row">
        <Nav />
        <MainContent />
      </div>
    </Router>
  );
}

export default App;
