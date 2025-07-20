"use client";
import React from "react";

export default function OutlinedInput({
  id,
  type = "text",
  value,
  onChange,
  placeholder,
}) {
  return (
    <input
      id={id}
      type={type}
      value={value}
      onChange={onChange}
      placeholder={placeholder}
      className="w-full p-3 border-2 border-gray-400 rounded-md focus:outline-none focus:border-blue-500 transition"
    />
  );
}
