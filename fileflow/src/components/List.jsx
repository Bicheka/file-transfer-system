import Item from "./Item";

function List({list}){
    return (
        <div className="h-full overflow-y-auto w-[%100] m-auto pb-16 border-2 border-gray-200 rounded-lg">
            {
                list.map((i) =>
                    // eslint-disable-next-line react/jsx-key
                    <Item data={i} updown="upload"/>
                )
            }
        </div>
    );
}

export default List;