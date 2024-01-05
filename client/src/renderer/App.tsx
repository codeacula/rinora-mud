import 'tailwindcss/tailwind.css';
import {
  MemoryRouter as Router,
  Routes,
  Route,
  Outlet,
} from 'react-router-dom';
import './App.css';
import { useState } from 'react';
import Sidebar from './Sidebar';
import NotFound from './NotFound';
import Rooms from './pages/Rooms';
import Database from './pages/Database';

function Main() {
  return (
    <div className="flex">
      <Sidebar />
      <div className="content flex-auto">
        <Outlet />
      </div>
    </div>
  );
}

export default function App() {
  const [authenticated] = useState(false);
  return (
    <Router future={{ v7_startTransition: true }}>
      <Routes>
        <Route path="/" element={authenticated ? <Main /> : <Database />}>
          <Route path="/rooms" element={<Rooms />} />
        </Route>
        <Route path="*" element={<NotFound />} />
      </Routes>
    </Router>
  );
}
