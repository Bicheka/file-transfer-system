import { open } from "@tauri-apps/plugin-dialog";
import { platform } from "@tauri-apps/plugin-os";
import { useState } from "react";
const FilePicker = () => {
  const [selectedPath, setSelectedPath] = useState("");

  const selectFileOrFolder = async () => {
    try {
      // Open the file/folder picker dialog
      const currentPlatform = await platform();
      let path = await open({
          directory: false,
          recursive: true,
          multiple: true,
        });

      // Set the selected path
      if (path) {
        setSelectedPath(path);
      }
    } catch (error) {
      console.error("Error selecting file/folder:", error);
    }
  };

  return (
    <div>
      <button onClick={selectFileOrFolder}>Select File/Folder</button>
      {selectedPath && <p>Selected Path: {selectedPath}</p>}
    </div>
  );
};

export default FilePicker;
