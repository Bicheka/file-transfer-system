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
    <form
      className="rounded px-1 py-4 xl:p-4 shadow-xs bg-white"
      onSubmit={handleConnect}
    >
      <label className="mb-2 block text-sm font-bold text-gray-700">
        IP Address
      </label>
      <input
        type="text"
        value={ipAddress}
        onChange={(e) => setIpAddress(e.target.value)}
        className="mb-4 w-full rounded-lg border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
        placeholder="Enter IP address"
      />
      {error && <p className="mb-4 text-red-500">{error}</p>}
      <button
        type="submit"
        className="w-full rounded-lg bg-blue-500 py-2 font-bold text-white hover:bg-blue-700"
      >
        Connect
      </button>
    </form>
  );
};

export default ServerConnectForm;
