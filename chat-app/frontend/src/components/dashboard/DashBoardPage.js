"use client";

import React, { useState, useEffect } from "react";
import axios from "axios";
import MessageField from "./MessageField";
import SidePanel from "./SidePanel";
import GET from "../../tools/protected_api";
import SectionSelector from "./UpperPanel";

export default function DashBoardPage() {
  const [token, setToken] = useState(null);

  useEffect(() => {
    const fetchTokenFromCookies = async () => {
      const res = GET();
      setToken(res);
    };

    if (token == null) {
      fetchTokenFromCookies();
    }
  }, []);
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
    <div className="flex justify-center min-h-screen overflow-hidden">
      <SidePanel />
      <SectionSelector />
      <MessageField />
    </div>
  );
}
