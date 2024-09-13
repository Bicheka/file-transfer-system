const Table = ({columns, data}) => {
  return (
    <table className="min-w-full xl:min-w-[60%] 2xl:min-w-[1000px] 2xl: divide-y divide-gray-200">
      <thead className="bg-gray-50">
        <tr>
          {columns.map(col => (
            <th key={col.key} className={`px-2 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider ${col.minWidth}`}>
              {col.label || ''}
            </th>
          ))}
        </tr>
      </thead>
      <tbody className="bg-white divide-y divide-gray-200">
        {data.map((row, index) => (
          <tr key={index}>
            {columns.map(col => (
              <td key={col.key} className={`px-2 py-3 text-sm font-medium text-gray-900 ${col.minWidth}`}>
                {row[col.key] || ""}
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default Table;
