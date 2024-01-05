import { Link } from 'react-router-dom';

export default function Sidebar() {
  return (
    <div className="sidebar">
      Sidebar
      <nav>
        <ul>
          <li>
            <Link to="/">Home</Link>
          </li>
          <li>
            <Link to="/rooms">Rooms</Link>
          </li>
          <li>
            <Link to="/butts">Butts</Link>
          </li>
        </ul>
      </nav>
    </div>
  );
}
