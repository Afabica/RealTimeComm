"use client";

import React, { useState, useEffect } from "react";
import axios from "axios";
import MessageField from "./MessageField";
import SidePanel from "./SidePanel";

export default function DashBoardPage() {
  const friends = [
    {
      id: "1",
      name: "Alice Johnson",
      status: "online",
      lastMessage: "Hey, are you joining the meeting?",
    },
    {
      id: "2",
      name: "Bob Smith",
      status: "offline",
      lastMessage: "Taslk later!",
    },
    {
      id: "3",
      name: "Charlie Zhang",
      status: "online",
      lastMessage: "Got the filed, thanks",
    },
    {
      id: "4",
      name: "Diana Perez",
      status: "offline",
      lastMessage: "Good night!",
    },
  ];

  return (
    <div className="flex justify-center h-[70%] overflow-hidden">
      <SidePanel friends={friends} />
      <MessageField />
    </div>
  );
}
