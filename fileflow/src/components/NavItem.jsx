import { useNavigate } from "react-router-dom";

function NavItem({ path, item, text, isActive }){

    const navigate = useNavigate();

    const handleClick = () => {
        navigate(path)
    }
    return (
    <div className={`group nav-item ${isActive ? "rounded-md bg-blue-900" : ""}`} onClick={handleClick}>
        {item}

        <span className="group-hover:scale-100 nav-tooltip">
            {text}
        </span>
    </div>
    );
}

export default NavItem;