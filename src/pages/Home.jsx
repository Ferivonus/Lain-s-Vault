import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import Cookies from "js-cookie";

function Home() {
  const [user_name, setUserName] = useState('');
  const [user_names, setUserNames] = useState([]);
  const [message, setMessage] = useState('');
  const [connectionMessage, setConnectionMessage] = useState('');
  
  // State for file encryption/decryption
  const [key, setKey] = useState('');
  const [iv, setIv] = useState('');
  const [excluded_files, setexcluded_files] = useState(['markdown_config.json']); // Excluded file

  useEffect(() => {
    const loadUserNames = async () => {
      try {
        const names = await invoke("load_user_names");
        setUserNames(names);
        setUserName(names.length > 0 ? names[0] : ""); 
      } catch (error) {
        console.error("Error loading user names:", error);
        setMessage("Failed to load user names.");
      }
    };

    loadUserNames();
  }, []);

  const saveUserName = async () => {
    if (user_name) {
      console.log(user_name);
      try {
        Cookies.set('userName', user_name, { expires: 7 }); // Expires in 7 days
        await invoke("save_user_name", { userName: user_name }); 
        setMessage(`User name saved as "${user_name}"`);
      } catch (error) {
        console.error("Error saving user name:", error);
        setMessage("Failed to save user name.");
      }
    } else {
      setMessage("Please enter a user name.");
    }
  };

  const checkConnection = async () => {
    try {
      const result = await invoke("check_connection");
      setConnectionMessage(result); // Set the result to connectionMessage
    } catch (error) {
      console.error("Error checking connection:", error);
      setConnectionMessage("Failed to check connection.");
    }
  };

  const encryptFiles = async () => {
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
    }
  };

  const decryptFiles = async () => {
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
    }
  };

  return (
    <div>
      <h1>Welcome to the Markdown App</h1>
      <p>Navigate to create or view Markdown files.</p>

      <div>
        <input
          placeholder="Enter your user name"
          value={user_name}
          onChange={(e) => setUserName(e.target.value)}
        />
        <button onClick={saveUserName}>Save User Name</button>
        <button onClick={checkConnection}>Check Connection</button>
      </div>
      
      <div>
        <p>Current user: {user_name}</p>
        {message && <p>{message}</p>}
        {connectionMessage && <p>{connectionMessage}</p>}
      </div>

      <div>
        <h2>File Encryption/Decryption</h2>
        <input
          type="text"
          placeholder="Encryption Key"
          value={key}
          onChange={(e) => setKey(e.target.value)}
        />
        <input
          type="text"
          placeholder="Initialization Vector (IV)"
          value={iv}
          onChange={(e) => setIv(e.target.value)}
        />
        <button onClick={encryptFiles}>Encrypt Files</button>
        <button onClick={decryptFiles}>Decrypt Files</button>
      </div>
    </div>
  );
}

export default Home;
