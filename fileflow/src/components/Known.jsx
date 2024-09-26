import { useState } from "react";
import { RiArrowDropDownLine } from "react-icons/ri";
function Known() {
  // TODO create function to get list of known ips from server
  const knownIps = [
    { name: "favid", ip: "192.0.2.1", ipType: "ipv4" },
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
    <div className="mx-auto h-[400px] md:h-[600px] w-full overflow-y-auto rounded lg:h-full">
      <table className="min-w-full table-auto bg-white">
        <thead className="sticky top-0 z-10 bg-blue-500 text-left text-white">
          <tr>
            <th className="px-4 py-1">Name</th>
            <th className="px-4 py-1">IP Address</th>
            <th className="px-4 py-1">IP Type</th>
          </tr>
        </thead>
        <tbody>
          {knownIps.map((entry, index) => (
            <>
              <tr
                className={`cursor-pointer hover:bg-gray-200 ${
                  clickedRowIndex === index
                    ? "bg-gray-300"
                    : index % 2 === 0
                      ? "bg-white"
                      : "bg-gray-50"
                }`}
                key={index}
                onClick={() => handleClick(index)}
              >
                <td className="px-4 py-5">{entry.name}</td>
                <td className="px-4 py-5">{entry.ip}</td>
                <td className="px-4 py-5">{entry.ipType}</td>
              </tr>
              {clickedRowIndex === index && (
                <tr>
                  <td colSpan="3" className="bg-gray-100 px-4 py-2 text-center">
                    <button className="rounded bg-blue-500 px-2 py-1 font-bold text-white hover:bg-blue-700">
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
