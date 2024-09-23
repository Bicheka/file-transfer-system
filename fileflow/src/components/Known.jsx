import { useState } from "react";

function Known() {
  // TODO create function to get list of known ips from server
  const knownIps = [
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
    { name: "david", ip: "192.0.2.1", ipType: "ipv4" },
    { name: "mario", ip: "2001:db8:0:1234:0:567:8:1", ipType: "ipv6" },
  ];
  const [clickedRowIndex, setClickedRowIndex] = useState(null);

  const handleClick = (index) => {
    setClickedRowIndex(index === clickedRowIndex ? null : index);
  };
  return (
    <div className="h-full min-w-fit overflow-y-auto">
      <table className="min-w-full table-auto">
        <thead className="sticky top-0 z-10 bg-gray-200 text-left">
          <tr>
            <th className="px-4 py-2">Name</th>
            <th className="px-4 py-2">IP Address</th>
            <th className="px-4 py-2">IP Type</th>
          </tr>
        </thead>
        <tbody>
          {knownIps.map((entry, index) => (
            <>
              <tr
                className="cursor-pointer hover:bg-gray-100"
                key={index}
                onClick={() => handleClick(index)}
              >
                <td className="px-4 py-2">{entry.name}</td>
                <td className="px-4 py-2">{entry.ip}</td>
                <td className="px-4 py-2">{entry.ipType}</td>
              </tr>
              {clickedRowIndex === index && (
                <tr>
                  <td colSpan="3" className="px-4 py-2 text-center">
                    <button className="rounded bg-blue-500 px-1 py-1 font-bold text-white hover:bg-blue-700">
                      Connect to {entry.name}
                    </button>
                  </td>
                </tr>
              )}
            </>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default Known;
