import 'tailwindcss/tailwind.css';
import {
  MemoryRouter as Router,
  Routes,
  Route,
  Outlet,
  redirect,
} from 'react-router-dom';
import './App.css';
import { useState } from 'react';
import Sidebar from './Sidebar';
import NotFound from './NotFound';
import Rooms from './pages/Rooms';
import Database from './pages/Database';

function Main() {
  const [authenticated] = useState(false);

  if (!authenticated) {
    console.log('Should redirect to database');
    redirect('/database');
    return null;
  }

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
  return (
    <Router future={{ v7_startTransition: true }}>
      <Routes>
        <Route path="/" element={<Main />}>
          <Route path="/dashboard" element={<Main />} />
          <Route path="/rooms" element={<Rooms />} />
        </Route>
        <Route path="/database" element={<Database />} />
        <Route path="*" element={<NotFound />} />
      </Routes>
    </Router>
  );
}
