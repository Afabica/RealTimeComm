"use client";

import React, { useState, useRef } from "react";

import typingIndicator from "../../animations/Indicators";

function MessageField() {
  const [message, setMessage] = useState("");
  const [messages, setMessages] = useState([]);
  const [istyping, setTyping] = useState(false);
  const typingTimeoutRef = useRef(null);

  const sendMessage = () => {
    if (message.trim() === "") return;
    setMessages((prevMessages) => [...prevMessages, message]);
    setMessage("");
  };

  const handleChange = (e) => {
    setMessage(e.target.value);

    if (!istyping) {
      setTyping(true);
    }

    if (typingTimeoutRef.current) {
      clearTimeout(typingTimeoutRef.current);
    }

    typingTimeoutRef.current = setTimeout(() => {
      setTyping(false);
    }, 1000);
  };

  return (
    <div className="flex flex-col w-full justify-end h-[90%]  shadow-lg bg-white">
      <div className="flex-1 overflow-y-auto p-2 border h-max-screen  rounded mb-2">
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
      <div className="flex items-center gap-2 w-[80%]">
        {istyping && typingIndicator()}
        <input
          type="text"
          value={message}
          onChange={handleChange}
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
