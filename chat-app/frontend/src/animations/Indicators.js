import React from "react";
import { motion } from "framer-motion";

export default function typingIndicator() {
  <span className="flex space-x-1">
    <span className="w-2 h-2 bg-gray-300 rounded-full animate-bounce [animation-delay:-0.3s]" />
    <span className="w-2 h-2 bg-gray-300 rounded-full animate-bounce [animation-delay:-0.15s]" />
    <span className="w-2 h-2 bg-gray-300 rounded-full animate-bounce" />
  </span>;
}
