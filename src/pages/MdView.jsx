// pages/MdView.jsx
import MDEditor from "@uiw/react-md-editor";
import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import Cookies from "js-cookie";
import "./css/MdView.css";

function MdView() {
  const [files, setFiles] = useState([]);
  const [selectedFile, setSelectedFile] = useState("");
  const [content, setContent] = useState("");
  const [viewMode, setViewMode] = useState(""); // 'system' or 'local' or empty

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

    if (viewMode === "system") fetchMarkdownFiles();
  }, [viewMode]);

  const loadMarkdown = async (filename) => {
    try {
      const userName = Cookies.get("userName");
      const content = await invoke("load_markdown", { userName, filename });
      setContent(content);
    } catch (error) {
      console.error("Error loading markdown:", error);
      alert("Failed to load markdown: " + error);
    }
  };

  const handleFileUpload = (event) => {
    const file = event.target.files[0];
    if (file && file.name.endsWith(".md")) {
      const reader = new FileReader();
      reader.onload = (e) => setContent(e.target.result);
      reader.onerror = () => alert("Failed to read file.");
      reader.readAsText(file);
    } else {
      alert("Please upload a valid Markdown (.md) file.");
    }
  };

  return (
    <div className="md-view-container">
      {viewMode === "" && (
        <div className="selection-mode">
          <h2>View Markdown Document</h2>
          <button onClick={() => setViewMode("system")}>Load from System</button>
          <button onClick={() => setViewMode("local")}>Upload from Computer</button>
        </div>
      )}

      {viewMode === "system" && (
        <div className="system-load">
          <h3>Load Markdown from System</h3>
          <div className="file-select">
            <label>Select a file:</label>
            <select
              onChange={(e) => {
                setSelectedFile(e.target.value);
                setContent("");
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
          <button onClick={() => loadMarkdown(selectedFile)}>Load Markdown</button>
          <button className="back-button" onClick={() => setViewMode("")}>Back</button>
        </div>
      )}

      {viewMode === "local" && (
        <div className="local-upload">
          <h3>Upload Markdown from Computer</h3>
          <input type="file" accept=".md" onChange={handleFileUpload} />
          <button className="back-button" onClick={() => setViewMode("")}>Back</button>
        </div>
      )}

      {content && (
        <div className="markdown-viewer">
          <h3>Markdown Preview: </h3>
          <MDEditor.Markdown source={content} style={{ whiteSpace: "pre-wrap" }} />
        </div>
      )}
    </div>
  );
}

export default MdView;
