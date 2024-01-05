import 'tailwindcss/tailwind.css';
import {
  MemoryRouter as Router,
  Routes,
  Route,
  Outlet,
} from 'react-router-dom';
import './App.css';
import Sidebar from './Sidebar';
import NotFound from './NotFound';
import Rooms from './pages/Rooms';

function Main() {
  return (
    <div>
      <Sidebar />
      <div className="content">
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
          <Route path="/rooms" element={<Rooms />} />
        </Route>
        <Route path="*" element={<NotFound />} />
      </Routes>
    </Router>
  );
}
