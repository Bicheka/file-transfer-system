
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
    <div className="max-w-xs md:pr-10 md:border-r-2 sticky top-0">
      <form
        onSubmit={handleConnect}
        className="bg-white rounded mb-4 h-full"
      >
        <div className="mb-4">
          <label
            className="block text-gray-700 text-sm font-bold mb-2"
            htmlFor="ipAddress"
          >
            Server IP Address
          </label>
          <input
            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="ipAddress"
            type="text"
            placeholder="Enter IP address"
            value={ipAddress}
            onChange={(e) => setIpAddress(e.target.value)}
          />
          {error && (
            <p className="text-red-500 text-xs italic mt-2">{error}</p>
          )}
        </div>

        <div className="flex items-center justify-between">
          <button
            className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
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
