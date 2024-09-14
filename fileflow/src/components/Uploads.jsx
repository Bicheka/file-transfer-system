import Path from "./Path";

const data = {
    name: "elden ring", 
    size: "40 GB", 
    path: "Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit",
    progress: "--------------------------------------"
};

function Uploads(){
    return(
        <div>
            <h1>Hello Uploads</h1>
            <Path data={data}/>
        </div>
    );
}

export default Uploads;