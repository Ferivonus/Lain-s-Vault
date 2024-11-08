import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Cookies from "js-cookie";

function FileSecurity() {
  const [key, setKey] = useState('');
  const [iv, setIv] = useState('');
  const [excluded_files, setExcludedFiles] = useState(['markdown_config.json']);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState(''); 
  const encryptFiles = async () => {
    if (!key || !iv) {
      setMessage("Please fill in both the key and IV fields.");
      return;
    }

    setLoading(true);
    try {
      const filePath = Cookies.get("userName");
      const result = await invoke("encrypt_files", { 
        filePath, 
        excludedFiles: excluded_files, 
        key, 
        iv 
      });
      setMessage(result);
    } catch (error) {
      console.error("Error encrypting files:", error);
      setMessage("Failed to encrypt files.");
    } finally {
      setLoading(false);
    }
  };

  const decryptFiles = async () => {
    if (!key || !iv) {
      setMessage("Please fill in both the key and IV fields.");
      return;
    }

    setLoading(true);
    try {
      const filePath = Cookies.get("userName");
      const result = await invoke("decrypt_files", { 
        filePath, 
        excludedFiles: excluded_files, 
        key, 
        iv 
      });
      setMessage(result);
    } catch (error) {
      console.error("Error decrypting files:", error);
      setMessage("Failed to decrypt files.");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <h2>File Encryption/Decryption</h2>
      <input
        type="password"
        placeholder="Encryption Key"
        value={key}
        onChange={(e) => setKey(e.target.value)}
      />
      <input
        type="password"
        placeholder="Initialization Vector (IV)"
        value={iv}
        onChange={(e) => setIv(e.target.value)}
      />
      <button onClick={encryptFiles} disabled={loading}>
        {loading ? "Encrypting..." : "Encrypt Files"}
      </button>
      <button onClick={decryptFiles} disabled={loading}>
        {loading ? "Decrypting..." : "Decrypt Files"}
      </button>
      {message && <p>{message}</p>} 
    </div>
  );
}

export default FileSecurity;
