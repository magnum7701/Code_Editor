import { Editor } from "@monaco-editor/react";
import { open } from "@tauri-apps/api/dialog";
import { readTextFile } from "@tauri-apps/api/fs";
import { useState } from "react";


const App = () => {
  const [fileContent, setFileContent] = useState("");
  const [extension, setExtension] = useState("");

  const handleFileRead = async () => {
    const fileSelect = await open();
    const arr = fileSelect.split("\\");

    const lastIdx = arr[arr.length - 1];

    const extWord = lastIdx.split(".");
    const ext = extWord[extWord.length - 1];

    setExtension(ext);

    if (fileSelect) {
      const content = await readTextFile(fileSelect);
      setFileContent(content);
    }

    else {
      console.log("No files selected");
    }
  }

  return (
    <div>
      <button onClick={handleFileRead}>Open File</button>
      <Editor height="90vh" language={extension} value={fileContent} />;
    </div>
  );
}

export default App;