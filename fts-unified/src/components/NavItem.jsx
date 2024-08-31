import { useNavigate } from "react-router-dom";

function NavItem({ path, item }){

    const navigate = useNavigate();

    const handleClick = () => {
        navigate(path)
    }
    return (<div className="nav-item" onClick={handleClick}>
        {item}
    </div>
    );
}

export default NavItem;