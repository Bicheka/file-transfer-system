import Known from "./Known";
import ServerConnectForm from "./ServerConnectForm";

function Connection() {
  return (
    <div className="mx-auto h-[900px] w-full bg-white p-3 sm:p-10 md:flex md:w-fit md:space-x-10">
      <ServerConnectForm />
      <Known />
    </div>
  );
}

export default Connection;
