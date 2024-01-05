import { Link } from 'react-router-dom';

export default function Sidebar() {
  return (
    <div className="sidebar flex-none w-14">
      Sidebar
      <nav>
        <ul>
          <li>
            <Link to="/">Home</Link>
          </li>
          <li>
            <Link to="/rooms">Rooms</Link>
          </li>
        </ul>
      </nav>
    </div>
  );
}
