import { useNavigate } from "react-router-dom";

function NavItem({ path, item, text }){

    const navigate = useNavigate();

    const handleClick = () => {
        navigate(path)
    }
    return (
    <div className="group nav-item" onClick={handleClick}>
        {item}

        <span className="group-hover:scale-100 nav-tooltip">
            {text}
        </span>
    </div>
    );
}

export default NavItem;