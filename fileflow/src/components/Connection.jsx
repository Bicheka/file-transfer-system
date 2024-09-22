import Known from "./Known";
import ServerConnectForm from "./ServerConnectForm";

function Connection(){
    return (
        <div className="md:flex h-[900px] bg-white w-full md:w-fit mx-auto md:space-x-10 p-3 sm:p-10">
            <ServerConnectForm/>
            <Known/>
        </div>
    );
}

export default Connection;