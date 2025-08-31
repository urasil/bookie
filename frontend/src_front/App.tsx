import { Outlet, Link } from 'react-router-dom';
import './App.css';

function App() {
  return (
    <div className="app-container">
      <nav className="navbar">
        <Link to="/">Home</Link>
        <Link to="/matches">Matches</Link>
      </nav>
      <div className="content-container">
        <Outlet />
      </div>
    </div>
  );
}

export default App;
