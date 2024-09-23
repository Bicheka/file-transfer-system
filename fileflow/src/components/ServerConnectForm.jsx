import { useState } from "react";
const ServerConnectForm = () => {
  const [ipAddress, setIpAddress] = useState("");
  const [error, setError] = useState("");

  // Function to handle form submission
  const handleConnect = (e) => {
    e.preventDefault();

    // Basic validation for IP address format
    const ipPattern =
      /^(25[0-5]|2[100-4][0-9]|1[0-9]{2}|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])){3}$/;
    if (!ipPattern.test(ipAddress)) {
      setError("Invalid IP address format");
      return;
    }

    setError("");

    // Perform connection logic (e.g., fetch or WebSocket)
    console.log(`Connecting to server at ${ipAddress}...`);
    // Insert connection logic here
  };

  return (
    <div className="sticky top-0 max-w-xs md:border-r-2 md:pr-10">
      <form onSubmit={handleConnect} className="mb-4 h-full rounded bg-white">
        <div className="mb-4">
          <label
            className="mb-2 block text-sm font-bold text-gray-700"
            htmlFor="ipAddress"
          >
            Server IP Address
          </label>
          <input
            className="focus:shadow-outlinepx-3 w-full appearance-none rounded border py-2 leading-tight text-gray-700 shadow focus:outline-none"
            id="ipAddress"
            type="text"
            placeholder="Enter IP address"
            value={ipAddress}
            onChange={(e) => setIpAddress(e.target.value)}
          />
          {error && <p className="mt-2 text-xs italic text-red-500">{error}</p>}
        </div>

        <div className="flex items-center justify-between">
          <button
            className="focus:shadow-outline rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none"
            type="submit"
          >
            Connect
          </button>
        </div>
      </form>
    </div>
  );
};

export default ServerConnectForm;
