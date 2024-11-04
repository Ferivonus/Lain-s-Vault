// App.js
import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom";
import Home from "./pages/Home";
import MdCreate from "./pages/MdCreate";
import MdView from "./pages/MdView";
import MdEdit from "./pages/MdEdit";

import "./App.css";

function App() {
  return (
    <Router>
      <main className="container">
        <h1>Welcome to Lain’s Vault </h1>

        <nav className="navbar">
          <Link to="/">Home</Link>
          <Link to="/md-create">Create Markdown</Link>
          <Link to="/md-view">View Markdown</Link>
          <Link to="/md-edit">Edit Markdown</Link>
        </nav>

        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/md-create" element={<MdCreate />} />
          <Route path="/md-view" element={<MdView />} />
          <Route path="/md-edit" element={<MdEdit />} />
        </Routes>
      </main>
    </Router>
  );
}

export default App;
