import List from "./List";
const item = {
  name: "elden ring",
  size: "40 GB",
  path: "Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit",
  progress: "--------------------------------------",
  uploadDestination: "david martinez",
  isTransfering: true,
};

const list = [
  item,
  item,
  item,
  item,
  item,
  item,
  item,
  item,
  item,
  item,
  item,
  item,
];

function Uploads() {
  return (
    <div className="h-[900px]">
      <List list={list} />
    </div>
  );
}

export default Uploads;
