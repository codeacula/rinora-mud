export function Sidebar() {
  return (
    <div className="sidebar">
      Sidebar
      <nav>
        <ul>
          <li>
            <a href={`/`}>Home</a>
          </li>
          <li>
          <a href={`/rooms`}>Rooms</a>
          </li>
        </ul>
      </nav>
    </div>
  );
}
