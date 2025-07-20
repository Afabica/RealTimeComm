import React, { useState, useEfect } from "react";
import { motion } from "framer-motion";

export default function InputBox({ children }) {
  return (
    <motion.input
      type="text"
      placeholder="Type your message..."
      className="chat-input"
      whileFocus={{ scale: 1.05, boxShadow: "0 0 8px rgba(59, 130, 246, 0.6)" }}
      transition={{ type: "spring", stiffness: 300 }}
    >
      {children}
    </motion.input>
  );
}
