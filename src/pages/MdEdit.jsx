// pages/MdEdit.jsx
import MDEditor from "@uiw/react-md-editor";
import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import Cookies from "js-cookie";

function MdEdit() {
  const [files, setFiles] = useState([]);
  const [selectedFile, setSelectedFile] = useState("");
  const [content, setContent] = useState("");
  const [key, setKey] = useState("");
  const [isEditing, setIsEditing] = useState(false);

  useEffect(() => {
    const fetchMarkdownFiles = async () => {
      try {
        const userName = Cookies.get("userName"); // Retrieve the username from cookies
        const files = await invoke("list_user_markdown_files", { userName });
        setFiles(files);
      } catch (error) {
        console.error("Error fetching markdown files:", error);
        alert("Failed to load files: " + error);
      }
    };

    fetchMarkdownFiles();
  }, []);

  const loadMarkdown = async (filename) => {
    if (!key) {
      alert("Please enter a decryption key.");
      return;
    }

    try {
      const userName = Cookies.get("userName");
      const fileContent = await invoke("load_markdown", { userName, filename });

      // Assuming you have a decryption function for the content
      // const decryptedContent = decryptContent(fileContent, key);
      setContent(fileContent); // Replace with decryptedContent if decryption is implemented
      setIsEditing(true);
    } catch (error) {
      console.error("Error loading markdown:", error);
      alert("Failed to load markdown: " + error);
    }
  };

  const saveMarkdown = async () => {
    if (!selectedFile) {
      alert("No file selected to save.");
      return;
    }

    try {
      const userName = Cookies.get("userName");
      
      // Assuming you have an encryption function for the content
      // const encryptedContent = encryptContent(content, key);
      
      await invoke("save_markdown", { userName, filename: selectedFile, content });

      alert("Markdown file saved successfully.");
    } catch (error) {
      console.error("Error saving markdown:", error);
      alert("Failed to save markdown: " + error);
    }
  };

  return (
    <div className="editor-container">
      <h2>Edit Markdown Document</h2>
      <div>
        <label>Select a file to edit: </label>
        <select
          onChange={(e) => {
            setSelectedFile(e.target.value);
            setContent(""); // Clear the content when changing files
            setIsEditing(false);
          }}
          value={selectedFile}
        >
          <option value="">-- Select a file --</option>
          {files.map((file, index) => (
            <option key={index} value={file}>
              {file}
            </option>
          ))}
        </select>
      </div>
      <input
        type="password"
        placeholder="Enter decryption key"
        value={key}
        onChange={(e) => setKey(e.target.value)}
      />
      <button onClick={() => loadMarkdown(selectedFile)}>Load Markdown</button>

      {isEditing && (
        <>
          <MDEditor value={content} onChange={setContent} style={{ whiteSpace: "pre-wrap" }} />
          <button onClick={saveMarkdown}>Save Changes</button>
        </>
      )}
    </div>
  );
}

export default MdEdit;
