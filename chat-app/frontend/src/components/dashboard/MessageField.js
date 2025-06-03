"use client";

import React, { useState } from "react";

function MessageField() {
  const [message, setMessage] = useState("");
  const [messages, setMessages] = useState([]);

  const sendMessage = () => {
    if (message.trim() === "") return;
    setMessages((prevMessages) => [...prevMessages, message]);
    setMessage("");
  };

  return (
    <div className="flex flex-col w-full h-[90%] justify-end min-h-screen shadow-lg bg-white">
      {/* Message list */}
      <div className="flex-1 overflow-y-auto p-2 border rounded mb-2 bg-gray-100">
        {messages.length === 0 ? (
          <p className="text-gray-500">No messages yet.</p>
        ) : (
          messages.map((msg, index) => (
            <div key={index} className="p-2 bg-blue-100 rounded my-1">
              {msg}
            </div>
          ))
        )}
      </div>

      {/* Input bar at the bottom */}
      <div className="flex items-center gap-2">
        <input
          type="text"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          className="flex-1 p-2 border rounded focus:outline-none focus:ring-2 focus:ring-blue-400"
          placeholder="Type a message..."
        />
        <button
          onClick={sendMessage}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition"
        >
          Send
        </button>
      </div>
    </div>
  );
}

export default MessageField;
