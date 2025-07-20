"use client";

import React, { useState, useEffect } from "react";
import axios from "axios";
import { motion } from "framer-motion";

export default function SectionSelector() {
  const [section, setSection] = useState("");

  return (
    <div className="flex h-[20%] flex-row w-[60%] bg-gradient-to-br from-indigo-900 via-purple-900 to-pink-900  relative overflow-hidden p-4 gap-5 justify-items-left">
      <div className="flex justify-left p-3 gap-6 bg-red-500 rounded-xl hover:bg-blue-600">
        <button
          className="hover:bg-blue-600"
          onClick={() => setSection("friends")}
        >
          Friends
        </button>
      </div>
      <div className="flex justify-left p-3 gap-6 bg-red-500 rounded-xl hover:bg-blue-600">
        <button onClick={() => setSection("allfriends")}>Friends Online</button>
      </div>
    </div>
  );
}
