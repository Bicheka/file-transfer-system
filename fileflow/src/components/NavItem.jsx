import { useNavigate } from "react-router-dom";

function NavItem({ path, item, text, isActive }) {
  const navigate = useNavigate();

  const handleClick = () => {
    navigate(path);
  };
  return (
    <div
      className={`nav-item group ${isActive ? "rounded-md bg-blue-900" : ""}`}
      onClick={handleClick}
    >
      {item}

      <span className="nav-tooltip group-hover:scale-100">{text}</span>
    </div>
  );
}

export default NavItem;
