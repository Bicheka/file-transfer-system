import Table from "./Table";
const columns = [
  { key: 'Name', label: 'Name', minWidth: 'sm:min-w-[150px]' },
  { key: 'size', label: 'Size', minWidth: 'sm:min-w-[100px]' },
  { key: 'downloadStatus', minWidth: 'sm:min-w-[100px]'}
]
const data = [
  { Name: 'John Doe', size: 30, downloadStatus: 'john.doe@example.com' },
  { Name: 'Jane Smith', size: 25, Phone: '555-1234', downloadStatus: 'downloading' },
];

function Downloads(){
    return(
        <div>
            <h1>Hello Downloads</h1>
            <Table columns={columns} data={data}/>
        </div>
    );
}

export default Downloads;