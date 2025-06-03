"use client";

import React from "react";

const SidePanel = ({ friends }) => {
  return (
    <div className="flex items-center justify-center  h-[100%] w-[20%] bg-gray-100">
      <div className="w-full min-h-screen max-w-md bg-white rounded-lg shadow-lg p-4">
        <h2 className="text-xl font-bold mb-4">Friends</h2>
        <ul className="space-y-3">
          {friends && friends.length > 0 ? (
            friends.map((friend) => (
              <li
                key={friend.id}
                className="flex items-center gap-3 p-2 border rounded hover:bg-gray-50 transition"
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
