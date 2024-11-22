
const { invoke } = window.__TAURI__.core;

async function login() {
  try {
    // Invoke the Rust function to handle Google OAuth login
    const response = await invoke("login_with_google");
    console.log("Got Response");
  } catch (error) {
    console.error("Failed to log in:", error);
  }
}

function logout() {
  console.log("Logged Out");
  invoke("logout_user");
}
async function get_user_profile() {
  try {
    // Invoke the Rust function to handle Google OAuth login
    const response = await invoke("get_user");
    console.log(response);

    // Replace the login button with the profile picture and user details
    document.getElementById("login_div").innerHTML = `
      <div style="display: flex; align-items: center; gap: 10px;">
        <img src="${response.picture}" alt="Profile Picture" class="profile-pic" />
        <div>
          <p style="margin: 0;" onclick="logout()">${response.name}</p>
          <p style="margin: 0; font-size: 0.9em; color: gray;">${response.email}</p>
        </div>
      </div>
    `;
  } catch (error) {
    console.error("Failed to log in:", error);
  }
}
// async function login() {
//     try {
//         // Invoke the Tauri command to get clipboard content
//         let content = await invoke("login_with_google");
//         document.getElementById("login_div").innerHTML = content;
//     } catch (error) {
//         console.error("Failed to get clipboard content:", error);
//     }
// }

// Array to store unique clipboard contents
let previousClipboardContent = '';

// Function to fetch clipboard content from Tauri backend
async function fetchClipboardContent() {
    try {
        // Invoke the Tauri command to get clipboard content
        const content = await invoke("monitor_clipboard_changes");

        // Check if the new content is not empty and different from the previous one
        if (content && content !== previousClipboardContent) {
            previousClipboardContent = content; // Update the previous content
            addClipboardEntry(content); // Add a new entry to the clipboard display
        }
    } catch (error) {
        console.error("Failed to get clipboard content:", error);
    }
}

// Function to fetch clipboard content from Tauri backend
function write_clipboard(content) {
    try {
        // Invoke the Tauri command to get clipboard content
        invoke("write_to_clipboard_call", { content: content });
    } catch (error) {
        console.error("Failed to get clipboard content:", error);
    }
}

function sleep(milliseconds, callback) {
  setTimeout(callback, milliseconds);
}

function addClipboardEntry(clipboard) {
    const container = document.getElementById("clipboardEntries");

    // Create a container for each entry
    const entryDiv = document.createElement("div");
    entryDiv.classList.add("entry");

    // Get current time as a timestamp
    const timestamp = new Date(clipboard.created);

    // Create and append time element
    const time = document.createElement("div");
    time.classList.add("entry-time");
    time.textContent = formatTimeAgo(timestamp);  // Format the time as relative time
    entryDiv.appendChild(time);

    // Create and append text element for the content
    const text = document.createElement("div");
    text.classList.add("entry-text");
    text.textContent = clipboard.content;
    entryDiv.appendChild(text);

    const actions = document.createElement("div");
    actions.classList.add("entry-actions");

    const copyIcon = document.createElement("span");
    copyIcon.textContent = "📋";
    copyIcon.style.cursor = "pointer"; // Make it look clickable
    copyIcon.onclick = () => {
      write_clipboard(text.textContent);
      copyIcon.textContent = "Copied!"  
      sleep(1000, () => {
        copyIcon.textContent = "📋";
      });
    } // Pass the entry-text content to clipboard function
    actions.appendChild(copyIcon);

    // Add other dummy icons (favorite, share)
    const favoriteIcon = document.createElement("span");
    favoriteIcon.textContent = "⭐";
    actions.appendChild(favoriteIcon);

    const shareIcon = document.createElement("span");
    shareIcon.textContent = "🔗";
    actions.appendChild(shareIcon);

    // Append the entry to the container
    entryDiv.appendChild(actions);
    container.prepend(entryDiv); // Use prepend to add the latest entry at the top

    // Store the timestamp in the entry for later updates
    entryDiv.timestamp = timestamp;

    // Set an interval to update the time every minute
    setInterval(() => updateEntryTime(entryDiv), 60000);
}



// Format time as "X ago"
function formatTimeAgo(timestamp) {
    const now = new Date();
    const seconds = Math.floor((now - timestamp) / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);

    if (minutes < 1) {
        return "Now";
    } else if (minutes < 60) {
        return `${minutes} minute${minutes > 1 ? "s" : ""} ago`;
    } else if (hours < 24) {
        return `${hours} hour${hours > 1 ? "s" : ""} ago`;
    } else {
        return `${days} day${days > 1 ? "s" : ""} ago`;
    }
}

// Update the time displayed in each entry
function updateEntryTime(entryDiv) {
    const timeElement = entryDiv.querySelector('.entry-time');
    if (timeElement) {
        timeElement.textContent = formatTimeAgo(entryDiv.timestamp);
    }
}

// Set an interval to update clipboard content every half second
setInterval(fetchClipboardContent, 500);
// setInterval(get_user_profile, 5000);
document.getElementById('login_button').addEventListener('click', login);
