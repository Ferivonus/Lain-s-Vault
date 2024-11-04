// pages/MdCreate.jsx
import MDEditor from "@uiw/react-md-editor";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import Cookies from "js-cookie";

function MdCreate() {
  const [value, setValue] = useState("**Hello world!!!**");
  const [filename, setFilename] = useState("");
  const [key, setKey] = useState("");
  const [confirmKey, setConfirmKey] = useState("");

  const saveMarkdown = async () => {
    const userName = Cookies.get('userName'); // Retrieve the username from the cookie

    try {
      // Pass userName along with filename and content to save_markdown
      await invoke("save_markdown", { userName, filename, content: value });
      alert(`Markdown saved as ${filename}.md`);
    } catch (error) {
      console.error("Error saving markdown:", error);
      alert("Failed to save markdown: " + error);
    }
  };

  return (
    <div className="editor-container">
      <h2>Create a Markdown Document</h2>
      <input
        placeholder="Enter filename"
        value={filename}
        onChange={(e) => setFilename(e.target.value)}
      />
      <input
        type="password"
        placeholder="Enter encryption key"
        value={key}
        onChange={(e) => setKey(e.target.value)}
      />
      <input
        type="password"
        placeholder="Confirm encryption key"
        value={confirmKey}
        onChange={(e) => setConfirmKey(e.target.value)}
      />
      <div className="editor">
        <MDEditor value={value} onChange={setValue} />
      </div>
      <button onClick={saveMarkdown}>Save Markdown</button>
    </div>
  );
}

export default MdCreate;

