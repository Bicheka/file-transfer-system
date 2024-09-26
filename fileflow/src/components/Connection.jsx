import Known from "./Known";
import ServerConnectForm from "./ServerConnectForm";

function Connection() {
  return (
    <div className="mx-auto w-full max-w-screen-lg transform p-3 transition-all duration-300 ease-in-out sm:p-6 md:p-10 lg:flex lg:h-[700px] lg:space-x-10">
      <ServerConnectForm />
      <div className="transform transition-all duration-300 ease-in-out">
        <Known />
      </div>
    </div>
  );
}

export default Connection;
