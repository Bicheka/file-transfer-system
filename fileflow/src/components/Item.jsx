function Item({ data, updown }) {
  return (
    <div className="mx-auto mb-4 flex w-[%90] max-w-4xl flex-wrap items-center space-x-4 rounded-lg bg-white p-4 shadow-md sm:mb-10">
      <div className="flex items-center space-x-1 px-4 align-middle">
        <p className="text-xs font-semibold text-gray-700 md:text-sm">Name:</p>
        <p className="align-middle text-xl font-medium text-gray-900">
          {data.name}
        </p>
      </div>

      <div className="flex items-center space-x-1">
        <p className="text-xs font-semibold text-gray-700 md:text-sm">Size:</p>
        <p className="align-middle text-xl text-gray-900 md:font-medium">
          {data.size}
        </p>
      </div>

      <div className="hidden items-center space-x-1 sm:visible sm:flex">
        <p className="text-xs font-semibold text-gray-700 md:text-sm">Path:</p>
        <p className="overflow-hidden truncate text-xs text-gray-900 hover:text-clip md:text-sm">
          {data.path}
        </p>
      </div>
      {data.isTransfering && (
        <div className="w-full flex-wrap items-center sm:flex">
          {updown == "upload" ? (
            <p className="w-fit rounded-lg py-1 text-xs font-medium text-green-500 sm:mr-2">
              Uploading
            </p>
          ) : (
            <p className="w-fit rounded-lg py-1 text-xs font-medium text-green-500 sm:mr-2">
              Downloading
            </p>
          )}

          <p className="w-fit text-xs text-gray-700 md:text-sm">
            Progress: {data.progress}%
          </p>
        </div>
      )}
      {data.isTransfering && (
        <div className="flex items-center space-x-1">
          <p className="text-xs font-semibold text-gray-700 md:text-sm">
            Uploading to:
          </p>
          <p className="text-xs text-gray-900 md:text-sm">
            {data.uploadDestination}
          </p>
        </div>
      )}

      <div className="mt-2 flex space-x-2 md:mt-0">
        {data.isTransfering && (
          <button className="rounded-lg bg-red-400 px-2 py-1 text-xs text-white hover:bg-red-600">
            Stop
          </button>
        )}
        {updown == "upload" && (
          <button className="rounded-lg bg-red-400 px-2 py-1 text-xs text-white hover:bg-red-600">
            Delete
          </button>
        )}
        {updown == "download" && !data.isTransfering && (
          <button className="rounded-lg bg-green-500 px-2 py-1 text-xs text-white hover:bg-red-600">
            Download
          </button>
        )}
      </div>
    </div>
  );
}

export default Item;
