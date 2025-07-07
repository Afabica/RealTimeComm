"use client";

import React, { useState, useEffect } from "react";
import GET from "../../tools/protected_api";

const SidePanel = ({ friends, privates }) => {
  const [selectedConversations, setSelectedConversation] = useState(null);
  const [conversations, setConversations] = useState([""]);
  const [error, setError] = useState("");
  const privateChats = conversations.filter((c) => !c.isGroup);
  const groupChats = conversations.filter((c) => c.isGroup);

  useEffect(() => {
    const fetchConversations = async () => {
      const token = GET();
      const response = await axios.get(
        `http://localhost:8080/groups?user_id=${encodeURIComponent(userId)}`,
        {
          headers: {
            Authorization: `Bearer ${token}`,
            "Content-Type": "application/json",
          },
        },
      );

      if (response.status === 200) {
        setConversations(response.data);
      } else {
        setError("Error occude during fetching connversations.");
        console.error("Fetch error:", err.response?.data || err.message);
      }
    };
    fetchConversations();
  }, []);

  return (
    <div className="flex items-center justify-center  h-[100%] w-[20%]  text-white  dark:bg-white dark:text-black">
      <div className="w-full min-h-screen max-w-md dark:bg-white dark:text-black rounded-lg shadow-lg p-4">
        <h2 className="text-xl font-bold mb-4">Friends</h2>
        <ul className="space-y-3">
          {friends && friends.length > 0 ? (
            friends.map((friend) => (
              <li
                key={friend.id}
                className="flex items-center gap-3 p-2 border rounded-full hover:bg-gray-50 transition"
              >
                <img
                  src={friend.avatar}
                  alt={`${friend.name}'s avatar`}
                  className="w-10 h-10 rounded-full object-cover"
                />
                <div className="flex flex-col">
                  <span className="font-medium">{friend.name}</span>
                  <span
                    className={`text-sm ${friend.status === "online" ? "text-green-500" : "text-gray-400"}`}
                  >
                    {friend.status}
                  </span>
                </div>
              </li>
            ))
          ) : (
            <li className="text-gray-500 text-center">No friends</li>
          )}
        </ul>
      </div>
    </div>
  );
};

export default SidePanel;
