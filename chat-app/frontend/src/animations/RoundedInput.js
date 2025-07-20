"use client";
import React from "react";

export default function RoundedInput({
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
      className="w-full px-4 py-2 rounded-full border border-gray-300 focus:ring-2 focus:ring-blue-300"
    />
  );
}
