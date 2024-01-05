import "tailwindcss/tailwind.css";
import { MemoryRouter as Router, Routes, Route } from 'react-router-dom';
import './App.css';
import { Sidebar } from "./Sidebar";

function Main() {
  return (
    <div>
      <Sidebar />
      <h1>Main</h1>
    </div>
  );
}

export default function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Main />} />
      </Routes>
    </Router>
  );
}
