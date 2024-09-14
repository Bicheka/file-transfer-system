function Path({ data }) {
    return (
        <div className="bg-gray-100 rounded-lg shadow-md flex flex-wrap items-center space-x-4 w-full max-w-4xl mx-auto">
            <div className="flex items-center space-x-1 px-4">
                <p className="font-semibold text-gray-700 text-xs md:text-sm">Name:</p>
                <p className="text-gray-900 text-xs md:text-sm">{data.name}</p>
            </div>

            <div className="flex items-center space-x-1">
                <p className="font-semibold text-gray-700 text-xs md:text-sm">Size:</p>
                <p className="text-gray-900 text-xs md:text-sm">{data.size}</p>
            </div>

            <div className="flex items-center space-x-1">
                <p className="font-semibold text-gray-700 text-xs md:text-sm">Path:</p>
                <p className="text-gray-900 truncate text-xs md:text-sm">{data.path}</p>
            </div>
            <div className="block w-full">
                <p className="text-gray-700 text-xs md:text-sm">Progress: {data.progress}%</p>
            </div>
            <div className="flex items-center space-x-1">
                <p className="font-semibold text-gray-700 text-xs md:text-sm">Uploading to:</p>
                <p className="text-gray-900 text-xs md:text-sm">{data.uploadDestination}</p>
            </div>

            <div className="flex space-x-2 mt-2 md:mt-0">
                <button className="bg-red-500 text-white px-2 py-1 rounded-lg text-xs hover:bg-red-600">
                    Stop
                </button>
                <button className="bg-red-500 text-white px-2 py-1 rounded-lg text-xs hover:bg-red-600">
                    Delete
                </button>
            </div>
        </div>
    );
}

export default Path;
