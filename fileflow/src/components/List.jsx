import Item from "./Item";

function List({ list }) {
  return (
    <div className="m-auto h-full w-[%100] overflow-y-auto rounded-lg border-2 border-gray-200 pb-16">
      {list.map((i) => (
        // eslint-disable-next-line react/jsx-key
        <Item data={i} updown="upload" />
      ))}
    </div>
  );
}

export default List;
