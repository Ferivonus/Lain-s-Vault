import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import Cookies from "js-cookie";

function Home() {
  const [user_name, setUserName] = useState('');
  const [user_names, setUserNames] = useState([]);
  const [message, setMessage] = useState('');

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
        // Updated to use the correct key "userName"
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
      </div>
      <div>
        <p>Current user: {user_name}</p>
        {message && <p>{message}</p>}
      </div>
    </div>
  );
}

export default Home;
